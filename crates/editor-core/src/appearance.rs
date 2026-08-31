//! Appearance attributes (M4 PR 7; DESIGN.md's ratified contract:
//! attributes attach **in the document layer, keyed by stable names,
//! from M4 — and nowhere, in any form, before that**).
//!
//! Per-face and per-body PRESENTATION attributes keyed by
//! [`StableName`] — never by arena key (G1: keys are per-evaluation
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
//! An entry counts as resolved when its name is found in ANY Ok
//! node's table ([`AppearanceResolution`] docs spell out the ruled
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
//! (keys are plain [`StableName`]s, GQ4 locality) and resolution is
//! per-document, per-evaluation; nothing here assumes global name
//! uniqueness, so the wrapper composes without modifying this module.

use std::collections::BTreeMap;

use crate::meta::MetaValue;
use crate::names::{Entry, NameTable, RoleSeg, StableName};
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
#[serde(deny_unknown_fields)]
pub enum AttrKind {
    /// Display color.
    Color,
    /// User-facing display label (DESIGN.md's "name" attribute —
    /// called Label here to avoid colliding with [`StableName`]).
    Label,
    /// Display visibility.
    Visibility,
}

/// One appearance attribute value (closed enum, one variant per
/// [`AttrKind`]; module docs on extension).
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum Attr {
    /// Display color.
    Color(Rgba8),
    /// User-facing display label.
    Label(String),
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

/// The document's appearance store: records keyed by stable name
/// (the B11 wrapper-ready seam — document-local keys, module docs).
pub type AppearanceMap = BTreeMap<StableName, AppearanceRecord>;

/// Why an appearance entry failed to resolve in an evaluation — the
/// typed "appearance lost its target" state (N3/N5). PR 4's
/// [`crate::resolve::enrich_appearance_loss`] maps each cause onto
/// the full N5 ladder (the verdict-vector diff engine supplies the
/// flipping predicate); the causes here are what the evaluation
/// itself can see without that engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppearanceLossCause {
    /// The name's minting node is no longer in the document (a later
    /// `DeleteNode` stranded it — N5's `NodeGone` semantics; the
    /// repair is `Rebind` (PR 4) or `ClearAppearance`).
    NodeGone,
    /// The name's minting node failed this evaluation; the attachment
    /// is indeterminate, not retired — it resolves again when the
    /// node evaluates.
    TargetFailed {
        /// The failed node (= the name's `node`).
        node: RecipeNodeId,
    },
    /// The name's minting node was poisoned by an upstream failure.
    TargetPoisoned {
        /// The nearest failed ancestor (walkable in the result DAG).
        through: RecipeNodeId,
    },
    /// The name's minting node has no result in this evaluation (a
    /// canceled run's incomplete suffix).
    TargetNotEvaluated,
    /// The name's node evaluated, but no table in the evaluation
    /// carries the name — the entity's derivation no longer produces
    /// it (N5 `Vanished`: a verdict flip, structural-parameter
    /// change, or recipe edit; PR 4 diagnoses which).
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
    /// tie several times (review A2); `at` is the first carrying node
    /// in id order, and the full candidate set is recoverable by
    /// table lookup at `at` (PR 4's `Ambiguous{candidates}` builds on
    /// exactly that).
    Ambiguous {
        /// The first (defining) node whose table holds the tie.
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
/// # Success criterion: resolves ANYWHERE (review A1, ruled)
///
/// An entry is "resolved" when at least one Ok node's table carries
/// its name — losses fire only when NO table does. Consequence,
/// upheld as correct v1 semantics: painting an OPERAND's face and
/// then consuming the operand in a boolean paints the operand node's
/// output only. The final node's corresponding face is a DIFFERENT
/// derivation (`FromA(name)` ≠ `name` — N1 identity), so it shows
/// neither the paint nor a loss, and [`Self::is_lossless`] is true.
/// This is deliberate: auto-following a face through a boolean would
/// be a silent rebinding policy, and the N5 policy menu is EMPTY by
/// ratified decision. The repairs are explicit — attribute the name
/// the displayed node's table actually mints (what a GUI does), or
/// [`crate::edit::DocEdit::Rebind`] (which moves the appearance key;
/// [`crate::resolve::appearance_rebind_suggestions`] feeds exactly
/// this gap with the wrapping final-node derivations).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AppearanceResolution {
    /// Per-node resolved rows: for each node whose table carried an
    /// attributed name, the entity and its attributes. A name minted
    /// at node `n` appears under every node whose table carries it
    /// (pass-through ops keep names, so a transform's output carries
    /// its input's attributes — N1 derivation-path semantics).
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

/// One node's result as appearance resolution sees it (a thin view of
/// the result DAG, T-erased: names and tables are scalar-independent,
/// so f64/Interval agree here whenever the tables agree — pinned by
/// the interval lane's tests).
#[derive(Debug, Clone, Copy)]
pub(crate) enum NodeState<'a> {
    /// The node evaluated; its name table.
    Ok(&'a NameTable),
    /// The node failed.
    Failed,
    /// The node was poisoned through the given failed ancestor.
    Poisoned {
        /// The nearest failed ancestor.
        through: RecipeNodeId,
    },
}

/// Resolves the document's appearance store against one evaluation's
/// tables (total: every entry lands in `resolved` or in `losses` —
/// never both silently dropped, never panicking).
///
/// `is_live` answers "is this node in the document" (NodeGone
/// detection); `states` holds the evaluation's per-node outcomes —
/// nodes absent from `states` but live are a canceled run's
/// unevaluated suffix.
pub(crate) fn resolve(
    appearance: &AppearanceMap,
    is_live: impl Fn(RecipeNodeId) -> bool,
    states: &BTreeMap<RecipeNodeId, NodeState<'_>>,
) -> AppearanceResolution {
    let mut resolution = AppearanceResolution::default();
    for (name, rec) in appearance {
        if !is_live(name.node) {
            resolution.losses.push(AppearanceLoss {
                name: name.clone(),
                attrs: rec.attrs.clone(),
                metadata: rec.metadata.clone(),
                cause: AppearanceLossCause::NodeGone,
            });
            continue;
        }
        // A name resolves in EVERY table that carries it: pass-through
        // ops (Transform) keep upstream names, so downstream outputs
        // inherit the attachment through the SAME name — no policy,
        // just N1 lookup.
        let mut hit = false;
        // Losses are per NAME, one row per cause (review A2): a tied
        // name carried by several tables (pass-through) reports ONE
        // Ambiguous loss, at the first table in node-id order — the
        // defining site; the others are derivable by lookup.
        let mut tie: Option<(RecipeNodeId, usize)> = None;
        for (&id, state) in states {
            let NodeState::Ok(table) = state else {
                continue;
            };
            match table.lookup(name) {
                Some(Entry::Unique(ent)) => {
                    hit = true;
                    resolution
                        .resolved
                        .entry(id)
                        .or_default()
                        .entry(*ent)
                        .or_default()
                        .extend(rec.attrs.iter().map(|(k, v)| (*k, v.clone())));
                }
                Some(Entry::Tied(cands)) => {
                    // N2: never auto-pick among equally admissible
                    // candidates — painting all of them would be one.
                    hit = true;
                    if tie.is_none() {
                        tie = Some((id, cands.len()));
                    }
                }
                None => {}
            }
        }
        if let Some((at, width)) = tie {
            resolution.losses.push(AppearanceLoss {
                name: name.clone(),
                attrs: rec.attrs.clone(),
                metadata: rec.metadata.clone(),
                cause: AppearanceLossCause::Ambiguous { at, width },
            });
        }
        if hit {
            continue;
        }
        let cause = match states.get(&name.node) {
            None => AppearanceLossCause::TargetNotEvaluated,
            Some(NodeState::Failed) => AppearanceLossCause::TargetFailed { node: name.node },
            Some(NodeState::Poisoned { through }) => {
                AppearanceLossCause::TargetPoisoned { through: *through }
            }
            Some(NodeState::Ok(_)) => AppearanceLossCause::Vanished {
                candidates: vanished_candidates(name, states),
            },
        };
        resolution.losses.push(AppearanceLoss {
            name: name.clone(),
            attrs: rec.attrs.clone(),
            metadata: rec.metadata.clone(),
            cause,
        });
    }
    resolution
}

/// N3 candidate offers for a vanished name (structural, no
/// heuristics): if the name IS a merged name, its constituents (the
/// symmetric unmerge case — `Merged{a,b}` vanishes with candidates
/// {a, b}); otherwise, any live merged name whose constituent set
/// contains it (the retire-into-merge case — the reference fails
/// "with the merged face as the offered candidate").
///
/// Scope of "structurally detectable" (review A5): the unmerge
/// direction fires only when `Merged` is the path's LAST segment —
/// names wrapping a merge deeper in (`Instance{of: Merged}`,
/// `FromA(Merged)`) get empty offers; PR 4's resolution ladder owns
/// anything beyond this.
fn vanished_candidates(
    name: &StableName,
    states: &BTreeMap<RecipeNodeId, NodeState<'_>>,
) -> Vec<StableName> {
    if let Some(RoleSeg::Merged(constituents)) = name.path.last() {
        return constituents.clone();
    }
    let mut out = Vec::new();
    for state in states.values() {
        let NodeState::Ok(table) = state else {
            continue;
        };
        for (candidate, _) in table.iter() {
            if let Some(RoleSeg::Merged(constituents)) = candidate.path.last()
                && constituents.contains(name)
                && !out.contains(candidate)
            {
                out.push(candidate.clone());
            }
        }
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! Unit tests for the resolve mechanics that the eval-level
    //! integration tests cannot reach: the N3 merged-name candidate
    //! offers (no eval-level path mints merges in v1 — the PR 3 R8
    //! record) and the tie refusal, over hand-built tables.

    use super::*;
    use crate::names::{EntityKey, EntityKind};

    fn body_name(node: u64, path: Vec<RoleSeg>) -> StableName {
        StableName {
            kind: EntityKind::Body,
            node: RecipeNodeId(node),
            path,
        }
    }

    fn ent(body: u32) -> EntityRef {
        EntityRef {
            body,
            key: EntityKey::Body,
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
        states.insert(RecipeNodeId(9), NodeState::Ok(&t));
        // Appearance rides constituent `a`, whose node (7) is live but
        // whose name is in no table.
        let mut appearance = AppearanceMap::new();
        appearance.insert(a.clone(), color());
        // Node 7 evaluated Ok with an empty table (its face was
        // absorbed downstream).
        let empty = NameTable::new();
        states.insert(RecipeNodeId(7), NodeState::Ok(&empty));
        let r = resolve(&appearance, |_| true, &states);
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
        states.insert(RecipeNodeId(9), NodeState::Ok(&empty));
        let mut appearance = AppearanceMap::new();
        appearance.insert(merged.clone(), color());
        let r = resolve(&appearance, |_| true, &states);
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
        states.insert(RecipeNodeId(3), NodeState::Ok(&t));
        let mut appearance = AppearanceMap::new();
        appearance.insert(tied.clone(), color());
        let r = resolve(&appearance, |_| true, &states);
        assert!(r.resolved.is_empty(), "a tie must never be painted");
        assert_eq!(r.losses.len(), 1);
        assert_eq!(
            r.losses[0].cause,
            AppearanceLossCause::Ambiguous {
                at: RecipeNodeId(3),
                width: 2
            }
        );
    }
}
