//! Appearance attributes (M4 PR 7; DESIGN.md's ratified contract:
//! attributes attach **in the document layer, keyed by stable names,
//! from M4 — and nowhere, in any form, before that**).
//!
//! Per-face and per-body PRESENTATION attributes keyed by a body read
//! and a [`StableName`] ([`AppearanceKey`]) — never by arena key (G1: keys are per-evaluation
//! and die on recompute; an arena-keyed container would be fake
//! durability). Appearance is document metadata: it does **not**
//! enter evaluation content keys, so an appearance edit recomputes no
//! geometry, and a parameter edit that flips no verdict on a name's
//! derivation leaves the attribute attached — the attribute rides the
//! name, not the topology.
//!
//! # Attribute vocabulary
//!
//! [`Attr`] is a closed enum with one variant per attribute kind
//! (DESIGN.md's listed trio: color, name/label, visibility); each
//! entity holds at most one attribute per [`AttrKind`]. Adding a kind
//! is ADDITIVE — a new variant pair, no schema churn (F3's enum-tagged
//! serialization). Values are exact (integers, bools, strings): no
//! floats, so bit-exact persistence and [`crate::doc::Doc::bit_eq`]
//! are structural.
//!
//! # Loss semantics (N3/N5 — loud, typed, the PR 4 hook)
//!
//! Resolution failures are never silent: an attribute whose name does
//! not resolve in the evaluation is reported as an [`AppearanceLoss`]
//! carrying the full attribute payload and a typed
//! [`AppearanceLossCause`]. Nothing is dropped and nothing is
//! auto-rebound (the N5 rebinding menu is EMPTY by ratified decision;
//! the repairs are explicit edits — PR 4's `Rebind`, or
//! [`crate::edit::DocEdit::ClearAppearance`]). PR 4's
//! `Diagnosis`/verdict-diff engine attaches to this hook: this module
//! states WHICH names lost their targets and the coarse cause visible
//! to the evaluation itself; the flipping predicate is PR 4's job.
//!
//! An entry is keyed by its body read and its name ([`AppearanceKey`];
//! NAMES N1, the scope), and counts as resolved when that read's table
//! carries the name ([`AppearanceResolution`] docs spell out the ruled
//! consequence: operand paint does not follow the face through a
//! boolean, and that gap is silent-by-design — N1 identity plus the
//! empty N5 auto-rebind menu — with explicit re-attachment or PR 4's
//! `Rebind` as the repair path).
//!
//! # Wrapper readiness (B11, NAMING-DESIGN Q-i)
//!
//! Appearance attachment is one of the four sanctioned seams where
//! assemblies later compose the (document identity × local ref)
//! wrapper AROUND the local name type. The store is document-local
//! (keys are a body read and a [`StableName`], GQ4 locality) and resolution is
//! per-document, per-evaluation; nothing here assumes global name
//! uniqueness, so the wrapper composes without modifying this module.

use std::collections::BTreeMap;

use crate::eval::NodeStanding;
use crate::meta::MetaValue;
use crate::names::{Entry, NameTable, StableName};
use crate::node::RecipeNodeId;

pub use crate::names::EntityRef;

/// An exact 8-bit-per-channel sRGB color with alpha. Integer channels
/// by design: appearance is presentation data and must persist
/// bit-exactly without entering any float policy (F3's NaN/inf
/// refusal has nothing to inspect here).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct Rgba8 {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
    /// Alpha channel (255 = opaque).
    pub a: u8,
}

impl Rgba8 {
    /// An opaque color.
    ///
    /// `const` so a palette can be stated as one: `viewer`'s themes
    /// (`crates/viewer/src/theme.rs`) are `const` values, and a
    /// constructor that could not be called in one would have forced
    /// them to spell the struct literal and bypass this door.
    pub const fn opaque(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

/// The attribute kinds (the per-entity map's key). One entry per kind
/// per entity; a new kind is an additive variant here plus its
/// [`Attr`] twin.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum AttrKind {
    /// Display color.
    Color,
    /// User-facing display label (DESIGN.md's "name" attribute —
    /// called Label here to avoid colliding with [`StableName`]).
    Label,
    /// Display visibility.
    Visibility,
}

impl AttrKind {
    /// The kind as a prose noun — the one spelling every user-facing
    /// message uses, so a rendered kind never leans on `Debug`.
    pub(crate) fn noun(self) -> &'static str {
        match self {
            Self::Color => "color",
            Self::Label => "label",
            Self::Visibility => "visibility",
        }
    }
}

/// One appearance attribute value (closed enum, one variant per
/// [`AttrKind`]; module docs on extension).
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Attr {
    /// Display color.
    Color(Rgba8),
    /// User-facing display label: the same [`crate::Label`] text a
    /// node's label is.
    Label(crate::Label),
    /// Display visibility (`false` = hidden).
    Visibility(bool),
}

impl Attr {
    /// The kind slot this attribute occupies.
    pub fn kind(&self) -> AttrKind {
        match self {
            Self::Color(_) => AttrKind::Color,
            Self::Label(_) => AttrKind::Label,
            Self::Visibility(_) => AttrKind::Visibility,
        }
    }
}

/// An entity's attributes, at most one per kind.
pub type AttrSet = BTreeMap<AttrKind, Attr>;

/// One name's full appearance record (M4 PR 6 spec D7): the typed
/// display attributes plus BLACK-BOX metadata — a string-keyed map of
/// [`MetaValue`] trees the kernel stores, round-trips, and never
/// interprets (GUI/tooling vocabulary; each value carries a `"v"`
/// version field by enforced producer convention — see
/// [`crate::meta`]). N3/N5 retire/vanish semantics apply to the WHOLE
/// record, metadata included ([`AppearanceLoss`] carries both).
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppearanceRecord {
    /// The typed display attributes, at most one per kind.
    #[serde(with = "crate::persist::strict::attrs")]
    pub attrs: AttrSet,
    /// The black-box metadata (spec D7; floats inside obey D2 —
    /// bit-exact via [`MetaValue`]'s bits-equality, NaN/inf refused at
    /// the edit and persist doors).
    #[serde(with = "crate::persist::strict::record_metadata")]
    pub metadata: BTreeMap<String, MetaValue>,
}

impl AppearanceRecord {
    /// True when the record holds nothing (the store drops empty
    /// records rather than keeping tombstones).
    pub fn is_empty(&self) -> bool {
        self.attrs.is_empty() && self.metadata.is_empty()
    }
}

/// **An appearance entry's key**: a name and the body read it is read
/// in (NAMES N1, the scope — a name tells apart only what its variable
/// holds, so a split's two halves carry one section face name and a
/// paint names the half; D10, a repair is addressed by body and name).
pub type AppearanceKey = crate::node::SitedRef<crate::VarId>;

/// The document's appearance store: records keyed by body read and
/// stable name (the B11 wrapper-ready seam — document-local keys,
/// module docs).
pub type AppearanceMap = BTreeMap<AppearanceKey, AppearanceRecord>;

/// Why an appearance entry failed to resolve in an evaluation — the
/// typed "appearance lost its target" state (N3/N5). PR 4's
/// [`crate::resolve::enrich_appearance_loss`] maps each cause onto
/// the full N5 ladder (the verdict-vector diff engine supplies the
/// flipping predicate); the causes here are what the evaluation
/// itself can see without that engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppearanceLossCause {
    /// The entry's body read is no longer the document's: the
    /// operation defining it was deleted (N5's `ReadGone`, ahead of
    /// `NodeGone`). The repair re-attaches the record in a live read
    /// (`SetAppearance` there, `ClearAppearance` here), never `Rebind`,
    /// which repairs a name and keeps the read.
    ReadGone,
    /// The name's minting node is no longer in the document (a later
    /// `DeleteNode` stranded it — N5's `NodeGone` semantics; the
    /// repair is `Rebind` (PR 4) or `ClearAppearance`).
    NodeGone,
    /// The name's minting node, or the operation defining the entry's
    /// body read, has no value in this evaluation — its standing says
    /// why. The attachment is indeterminate, not retired: it resolves
    /// again when the node evaluates.
    Indeterminate(NodeStanding),
    /// The name's node evaluated, but neither the entry's body read
    /// nor any read downstream of it carries the name — the entity's
    /// derivation no longer produces it there (N5 `Vanished`: a verdict
    /// flip, structural-parameter change, or recipe edit; PR 4
    /// diagnoses which).
    Vanished {
        /// N3 candidate offers, when structurally detectable: the
        /// merged name a constituent retired into, or a vanished
        /// merged name's constituents (unmerge). Empty otherwise;
        /// PR 4's resolution ladder owns the full candidate story.
        candidates: Vec<StableName>,
    },
    /// The name is tie-marked (N2): referencing a tied name is
    /// ambiguous, so painting any candidate would be an auto-pick —
    /// refused loudly until the recipe records a disambiguation.
    /// Reported ONCE per name even when pass-through tables carry the
    /// tie several times (review A2); `at` is the operation of the
    /// first read holding the tie — the entry's body read, else the
    /// first read downstream of it — and the full candidate set is
    /// recoverable by table lookup there (PR 4's
    /// `Ambiguous{candidates}` builds on exactly that).
    Ambiguous {
        /// The operation of the first read whose table holds the tie.
        at: RecipeNodeId,
        /// How many candidates tie there.
        width: usize,
    },
}

/// One appearance entry that failed to resolve: the name, its FULL
/// attribute payload (nothing is silently dropped — the store still
/// holds it), and the typed cause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppearanceLoss {
    /// The body read the entry is read in.
    pub body: crate::VarId,
    /// The name whose target was lost.
    pub name: StableName,
    /// The attributes that were attached to it.
    pub attrs: AttrSet,
    /// The black-box metadata attached to it (spec D7: the loss
    /// carries the WHOLE record — nothing is silently dropped).
    pub metadata: BTreeMap<String, MetaValue>,
    /// Why it did not resolve.
    pub cause: AppearanceLossCause,
}

/// Resolved appearance for one evaluation: per node, entity → its
/// attributes — the renderer/exporter-facing shape (no table reads
/// needed downstream). Resolved rows carry the typed ATTRS only:
/// D7 metadata stays name-keyed in the document store (black-box —
/// merging several names' metadata trees under one entity would
/// need an interpretation policy the kernel refuses to have; a GUI
/// reads `Doc::appearance_of(name).metadata` directly). Losses, by
/// contrast, carry the WHOLE record — nothing is silently dropped. Entity refs are per-THIS-evaluation data (they
/// die with it, G1); the durable attachment remains the document's
/// name-keyed store.
///
/// # Success criterion: resolves in its body read (review A1, ruled; N1)
///
/// An entry is "resolved" when its body read's table carries its name,
/// and paints every read downstream of it that carries the name too —
/// losses fire when the body read's table does not. Consequence,
/// upheld as correct v1 semantics: painting an OPERAND's face and
/// then consuming the operand in a boolean paints the operand node's
/// output only. The final node's corresponding face is a DIFFERENT
/// derivation (`From { read, of: name }` ≠ `name` — N1 identity), so it shows
/// neither the paint nor a loss, and [`Self::is_lossless`] is true.
/// This is deliberate: auto-following a face through a boolean would
/// be a silent rebinding policy, and the N5 policy menu is EMPTY by
/// ratified decision. The repair is explicit: attribute the name the
/// displayed node's table actually mints, read in that node's output
/// (what a GUI does) — a re-attachment, `SetAppearance` there and
/// `ClearAppearance` here, since a [`crate::edit::DocEdit::Rebind`]
/// repairs a key's name and never moves its read;
/// [`crate::resolve::appearance_rebind_suggestions`] feeds exactly this
/// gap with the wrapping final-node derivations and their reads.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AppearanceResolution {
    /// Per-node resolved rows: for each node whose table carried an
    /// attributed name, the entity and its attributes. An entry appears
    /// under its body read's node and every node downstream of that
    /// read whose table carries the name (pass-through ops keep names,
    /// so a transform's output carries its input's attributes — N1
    /// derivation-path semantics).
    pub resolved: BTreeMap<RecipeNodeId, BTreeMap<EntityRef, AttrSet>>,
    /// Every appearance entry that failed to resolve, loudly typed
    /// (empty = every attribute landed). Per-name granularity: an
    /// entry lands here AT MOST ONCE PER CAUSE (review A2) — in v1
    /// each name gets at most one loss row total, since the causes
    /// are mutually exclusive per evaluation.
    pub losses: Vec<AppearanceLoss>,
}

impl AppearanceResolution {
    /// The resolved entity → attributes map for one node's output.
    pub fn for_node(&self, id: RecipeNodeId) -> Option<&BTreeMap<EntityRef, AttrSet>> {
        self.resolved.get(&id)
    }

    /// True when every appearance entry resolved.
    pub fn is_lossless(&self) -> bool {
        self.losses.is_empty()
    }
}

/// One node's result as appearance resolution sees it: its name table,
/// or its standing (T-erased: names and tables are scalar-independent,
/// so f64/Interval agree here whenever the tables agree — pinned by
/// the interval lane's tests).
///
/// A node's tables are its outputs' (NAMES N1, the scope), each with the
/// output body its entities' body 0 is in the node's value (a split's
/// halves at their ports, every other node's one output at 0).
pub(crate) type NodeState<'a> = Result<Vec<(u32, &'a NameTable)>, NodeStanding>;

/// **The reads appearance resolution walks**: each output read's node
/// and port, and the output reads each read feeds — those of every live
/// operation taking it as a body, directly or through a selection of it
/// ([`crate::Doc::feeds`]).
pub(crate) struct Reads {
    /// Each output read and the port that defines it.
    pub(crate) ports: BTreeMap<crate::VarId, (RecipeNodeId, u8)>,
    /// Each read and the output reads of the operations that read it.
    pub(crate) feeds: BTreeMap<crate::VarId, Vec<crate::VarId>>,
    /// The body reads that reach an instance, whose tables may carry
    /// its part's names, spelled in the part's ids
    /// ([`crate::Doc::reaches_instance`]).
    pub(crate) parted: std::collections::BTreeSet<crate::VarId>,
}

impl Reads {
    /// `body` and every read downstream of it, each once, in the
    /// evaluation's order (a read's node's place in `order`, then its
    /// port).
    fn downstream(&self, body: crate::VarId, order: &[RecipeNodeId]) -> Vec<crate::VarId> {
        let mut seen = std::collections::BTreeSet::from([body]);
        let mut stack = vec![body];
        while let Some(read) = stack.pop() {
            for &next in self.feeds.get(&read).into_iter().flatten() {
                if seen.insert(next) {
                    stack.push(next);
                }
            }
        }
        let place = |read: &crate::VarId| {
            let (node, port) = self.ports.get(read).copied()?;
            Some((order.iter().position(|&id| id == node)?, port))
        };
        let mut out: Vec<(usize, u8, crate::VarId)> = seen
            .into_iter()
            .filter_map(|read| place(&read).map(|(at, port)| (at, port, read)))
            .collect();
        out.sort_unstable();
        out.into_iter().map(|(_, _, read)| read).collect()
    }
}

/// Resolves the document's appearance store against one evaluation's
/// tables (total: every entry lands in `resolved` or in `losses` —
/// never both silently dropped, never panicking).
///
/// An entry resolves in its body read's table (NAMES N1, the scope),
/// and rides on downstream through every read that read feeds whose
/// table carries the name verbatim (pass-through ops keep names, so a
/// transform of the painted body shows the paint; the other half of a
/// split, which carries the same section face name, does not).
///
/// `states` holds every live node of the document — a canceled run's
/// unevaluated suffix with its standing like any other — so a name
/// whose node it does not hold names a node the document no longer
/// has. `order` is the evaluation's order over those nodes.
pub(crate) fn resolve(
    appearance: &AppearanceMap,
    order: &[RecipeNodeId],
    states: &BTreeMap<RecipeNodeId, NodeState<'_>>,
    reads: &Reads,
) -> AppearanceResolution {
    let in_order = || order.iter().filter_map(|id| Some((*id, states.get(id)?)));
    let table_of = |read: &crate::VarId| {
        let &(node, port) = reads.ports.get(read)?;
        let tables = states.get(&node)?.as_ref().ok()?;
        let &(base, table) = tables.iter().find(|(p, _)| *p == u32::from(port))?;
        Some((node, base, table))
    };
    let mut resolution = AppearanceResolution::default();
    for (key, rec) in appearance {
        let (body, name) = (key.at, &key.name);
        let loss = |cause| AppearanceLoss {
            body,
            name: name.clone(),
            attrs: rec.attrs.clone(),
            metadata: rec.metadata.clone(),
            cause,
        };
        // The body read must be the document's (rung 0), then the
        // name's node (rung 1).
        let Some(at) = reads
            .ports
            .get(&body)
            .and_then(|&(node, _)| states.get(&node))
        else {
            resolution.losses.push(loss(AppearanceLossCause::ReadGone));
            continue;
        };
        // A part's name is no node of this document: its node is the
        // part's, and the body read is the one rung it has.
        let target = match states.get(&name.node) {
            Some(target) => target,
            None if reads.parted.contains(&body) => at,
            None => {
                resolution.losses.push(loss(AppearanceLossCause::NodeGone));
                continue;
            }
        };
        if let Err(standing) = target.as_ref().and(at.as_ref()) {
            resolution
                .losses
                .push(loss(AppearanceLossCause::Indeterminate(*standing)));
            continue;
        }
        // The body read's own table first, then each read downstream of
        // it: a name resolves in EVERY one that carries it, so
        // pass-through outputs inherit the attachment through the SAME
        // name — no policy, just N1 lookup along the reads.
        let mut hit = false;
        // Losses are per entry, one row per cause (review A2): a tied
        // name carried by several tables reports ONE Ambiguous loss, at
        // the first table in evaluation order.
        let mut tie: Option<(RecipeNodeId, usize)> = None;
        for read in reads.downstream(body, order) {
            let Some((node, base, table)) = table_of(&read) else {
                if read == body {
                    break;
                }
                continue;
            };
            match table.lookup(name) {
                Some(Entry::Unique(ent)) => {
                    hit = true;
                    let ent = EntityRef {
                        body: base + ent.body,
                        key: ent.key,
                    };
                    resolution
                        .resolved
                        .entry(node)
                        .or_default()
                        .entry(ent)
                        .or_default()
                        .extend(rec.attrs.iter().map(|(k, v)| (*k, v.clone())));
                }
                Some(Entry::Tied(cands)) => {
                    // N2: never auto-pick among equally admissible
                    // candidates — painting all of them would be one.
                    hit = true;
                    if tie.is_none() {
                        tie = Some((node, cands.len()));
                    }
                }
                // The body read's own table is where the entry is: a
                // name it does not hold is lost, whatever carries it
                // downstream.
                None if read == body => break,
                None => {}
            }
        }
        if let Some((at, width)) = tie {
            resolution
                .losses
                .push(loss(AppearanceLossCause::Ambiguous { at, width }));
        }
        if hit {
            continue;
        }
        resolution.losses.push(loss(AppearanceLossCause::Vanished {
            candidates: vanished_candidates(name, in_order()),
        }));
    }
    resolution
}

/// N3 candidate offers for a vanished name (structural, no
/// heuristics; `names::merged::offers`, the one reading of a
/// set-holding row): a merged name's constituents (the symmetric
/// unmerge case — `Merged{a,b}` vanishes with candidates {a, b}), a
/// broken run wall's pieces' live walls, and any live row whose set
/// COVERS the name (`names::merged::row_covers`: the retire-into-merge
/// case — the reference fails "with the merged face as the offered
/// candidate" — a merged face consumed by a wider merge, covered by the
/// row that lists its faces, and a wall a station joined into a run).
fn vanished_candidates<'s, 'a: 's>(
    name: &StableName,
    states: impl Iterator<Item = (RecipeNodeId, &'s NodeState<'a>)>,
) -> Vec<StableName> {
    let rows: Vec<&StableName> = states
        .filter_map(|(_, state)| state.as_ref().ok())
        .flatten()
        .flat_map(|(_, table)| table.iter().map(|(candidate, _)| candidate))
        .collect();
    crate::names::merged::offers(name, rows.iter().copied())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! Unit tests for the resolve mechanics that the eval-level
    //! integration tests cannot reach: the N3 merged-name candidate
    //! offers (no eval-level path mints merges in v1 — the PR 3 R8
    //! record) and the tie refusal, over hand-built tables.

    use super::*;
    use crate::names::{EntityKey, EntityKind, RoleSeg};

    fn body_name(node: u64, path: Vec<RoleSeg>) -> StableName {
        StableName {
            kind: EntityKind::Body,
            node: RecipeNodeId::new(0, node),
            path,
        }
    }

    fn ent(body: u32) -> EntityRef {
        EntityRef {
            body,
            key: EntityKey::Body,
        }
    }

    /// An entry on `name`, read in its minting node's one output.
    fn key(name: &StableName) -> AppearanceKey {
        AppearanceKey::new(crate::VarId(name.node.0), name.clone())
    }

    /// Each node's one output read, at port 0, feeding nothing.
    fn reads(states: &BTreeMap<RecipeNodeId, NodeState<'_>>) -> Reads {
        Reads {
            ports: states
                .keys()
                .map(|&node| (crate::VarId(node.0), (node, 0)))
                .collect(),
            feeds: BTreeMap::new(),
            parted: std::collections::BTreeSet::new(),
        }
    }

    fn color() -> AppearanceRecord {
        let mut s = AttrSet::new();
        s.insert(AttrKind::Color, Attr::Color(Rgba8::opaque(1, 2, 3)));
        AppearanceRecord {
            attrs: s,
            metadata: BTreeMap::new(),
        }
    }

    #[test]
    fn constituent_vanishing_into_a_merge_offers_the_merged_name() {
        // Table: one merged name whose constituents are a and b.
        let a = body_name(7, vec![RoleSeg::OutputBody]);
        let b = body_name(8, vec![RoleSeg::OutputBody]);
        let merged = body_name(9, vec![RoleSeg::Merged(vec![a.clone(), b.clone()])]);
        let mut t = NameTable::new();
        t.insert(merged.clone(), ent(0)).unwrap();
        let mut states = BTreeMap::new();
        states.insert(RecipeNodeId::new(0, 9), Ok(vec![(0, &t)]));
        // Appearance rides constituent `a`, whose node (7) is live but
        // whose name is in no table.
        let mut appearance = AppearanceMap::new();
        appearance.insert(key(&a), color());
        // Node 7 evaluated Ok with an empty table (its face was
        // absorbed downstream).
        let empty = NameTable::new();
        states.insert(RecipeNodeId::new(0, 7), Ok(vec![(0, &empty)]));
        let r = resolve(
            &appearance,
            &states.keys().copied().collect::<Vec<_>>(),
            &states,
            &reads(&states),
        );
        assert!(r.resolved.is_empty());
        assert_eq!(r.losses.len(), 1);
        let loss = &r.losses[0];
        assert_eq!(loss.name, a);
        assert_eq!(loss.attrs, color().attrs);
        assert_eq!(
            loss.cause,
            AppearanceLossCause::Vanished {
                candidates: vec![merged]
            }
        );
    }

    #[test]
    fn vanished_merged_name_offers_its_constituents() {
        // The merged name itself is attributed but has unmerged: the
        // constituents are the offered candidates (N3 symmetric case).
        let a = body_name(7, vec![RoleSeg::OutputBody]);
        let b = body_name(8, vec![RoleSeg::OutputBody]);
        let merged = body_name(9, vec![RoleSeg::Merged(vec![a.clone(), b.clone()])]);
        let empty = NameTable::new();
        let mut states = BTreeMap::new();
        states.insert(RecipeNodeId::new(0, 9), Ok(vec![(0, &empty)]));
        let mut appearance = AppearanceMap::new();
        appearance.insert(key(&merged), color());
        let r = resolve(
            &appearance,
            &states.keys().copied().collect::<Vec<_>>(),
            &states,
            &reads(&states),
        );
        assert_eq!(r.losses.len(), 1);
        assert_eq!(
            r.losses[0].cause,
            AppearanceLossCause::Vanished {
                candidates: vec![a, b]
            }
        );
    }

    #[test]
    fn tied_name_is_refused_ambiguous_not_painted() {
        let tied = body_name(3, vec![RoleSeg::OutputBody]);
        let mut t = NameTable::new();
        t.insert_tied(tied.clone(), vec![ent(0), ent(1)]).unwrap();
        let mut states = BTreeMap::new();
        states.insert(RecipeNodeId::new(0, 3), Ok(vec![(0, &t)]));
        let mut appearance = AppearanceMap::new();
        appearance.insert(key(&tied), color());
        let r = resolve(
            &appearance,
            &states.keys().copied().collect::<Vec<_>>(),
            &states,
            &reads(&states),
        );
        assert!(r.resolved.is_empty(), "a tie must never be painted");
        assert_eq!(r.losses.len(), 1);
        assert_eq!(
            r.losses[0].cause,
            AppearanceLossCause::Ambiguous {
                at: RecipeNodeId::new(0, 3),
                width: 2
            }
        );
    }
}
