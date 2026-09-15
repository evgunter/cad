//! Eager per-op name-table emission (N4; spec D4): mechanical, linear
//! passes over each op's output, driven ONLY by kernel birth data and
//! mint-time wiring facts (`Extruded`/`Revolved` maps, `SplitNaming`,
//! `BooleanNaming`, D5 provenance) plus combinatorial adjacency
//! between EMITTED anchors — never geometric matching. Where geometry
//! must discriminate (N2 fragment qualifiers), it enters as margined
//! predicate VERDICTS through `k_stats` ([`super::discriminate`]).
//!
//! Every emitter finishes with the TOTALITY check: every live
//! boundary entity of every output body must be named (a gap is a
//! typed [`NamingError`], surfacing kernel-emission holes loudly),
//! and injectivity is enforced by [`NameTable`] at insertion.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_core::{BandError, Indeterminate};
use topo::{Body, EdgeKey, FaceKey, HalfEdgeKey, SplitLineageCycle, VertexKey};

use super::role::{EntityKind, StableName};
use super::table::{DuplicateName, EntityKey, EntityRef, NameTable};
use crate::node::RecipeNodeId;

/// Typed failure of name emission (spec D4's loud assertions, as
/// in-band errors — this crate has no panic paths).
///
/// **Three kinds of thing live here, and the reader is told which.**
/// Most variants are an emission BUG — a mint-time fact inconsistent
/// with the result body — and [`Self::Escalated`] is an honest in-band
/// escalation. [`Self::SeamVertexParentage`] and [`Self::SharedRim`]
/// are neither: they are reached from recipes nothing is wrong with,
/// where the emitter has no rule for a construction the recipe
/// produced. They read as a MISSING RULE, not as a bug report,
/// because telling an author to file a kernel bug over their own legal
/// document fails in the expensive direction. The dividing line is the
/// one [`Self::Band`] already draws: nothing about the result body is
/// wrong at those two sites.
#[derive(Debug)]
pub enum NamingError {
    /// A would-be duplicate name outside the tie path (the
    /// no-silent-aliasing guarantee N1 rests on), or a kind/entity
    /// collision at insertion.
    Duplicate {
        /// The colliding name.
        name: Box<StableName>,
    },
    /// A live boundary entity the emission failed to cover — a
    /// kernel-emission gap surfaced (spec D4: never guessed around).
    Unnamed {
        /// The uncovered entity's kind.
        kind: EntityKind,
        /// The output-body index it lives in.
        body: u32,
    },
    /// An upstream input's name table lacks an entity the emission
    /// needed (wiring bug: upstream tables are total by this same
    /// machinery).
    MissingUpstream {
        /// The upstream node whose table missed.
        node: RecipeNodeId,
    },
    /// A mint-time emission fact was inconsistent with the result
    /// body (kernel bug, loudly).
    Emission {
        /// What was inconsistent.
        what: &'static str,
    },
    /// An edge's split lineage cycles, caught where an emitter chased
    /// it to its root — the same category of kernel bug as
    /// [`Self::Emission`], carrying the one thing the repair needs
    /// that a sentence cannot supply: WHICH edge.
    ///
    /// **Raised from two chases, one guarded and one not, and the
    /// dividing line is WRITER ACCESS.** `emit_topo`'s `chase_b` is
    /// guarded (`a_cycling_graft_map_refuses_in_the_b_lane`): it hops
    /// through a graft map the CALLER supplies between provenance
    /// reads, so a loop closes from outside `topo`.
    /// `chase_edge_to_table` is not, because it advances only on
    /// `Body::edge_provenance`, which is `pub(crate)` to `topo` and is
    /// written by one door — `Body::split_edge`, recording the parent
    /// on a child it has just minted, so a chain is strictly
    /// decreasing in age and no caller can close it. That a cycling
    /// lineage exists at all is real: `topo::props`' carrier-identity
    /// fold documents it as what a graft aliases, and
    /// `work/bool/graft-copies-provenance-keys-verbatim.md` records
    /// `Body::split_root`'s cycle arm firing on real assembly
    /// products — `topo`-internally, where this crate has no door.
    SplitLineage(SplitLineageCycle),
    /// A face's FRAGMENT lineage cycles, caught where an emitter
    /// chased it to its root through a split's or a boolean's
    /// `face_fragments` rows — the same category of kernel bug as
    /// [`Self::Emission`] and the same category as
    /// [`Self::SplitLineage`], carrying the one thing the repair needs
    /// that a sentence cannot supply: WHICH face.
    ///
    /// A sibling word rather than one generalised over
    /// [`super::table::EntityKey`]: the two cycles are corrupt records
    /// of DIFFERENT families — a mint-time `face_fragments` row here,
    /// a `SplitEdge` birth record for [`Self::SplitLineage`] — so one
    /// word for the class would name the key's kind while hiding which
    /// map to go and read.
    ///
    /// Guarded by `emit_topo`'s `a_cycling_fragment_map_refuses`.
    FragmentLineage {
        /// The face whose fragment chain cycles — the key the chase
        /// was ASKED about, which is the one a repair starts from.
        face: FaceKey,
    },
    /// A boolean's seam VERTEX whose parentage no rule determines: it
    /// has neither one operand-descended edge on each side nor a pair
    /// of seam lines, so the case analysis over its incident edge
    /// names has no arm for it.
    ///
    /// **Not an emission inconsistency**, and the test is the same one
    /// [`Self::Band`] applies: nothing about the result body is wrong
    /// here. Ordinary declared unions reach it — a member stacked on a
    /// face that a second declared contact has merged puts a seam
    /// vertex on a merged wall with mixed parentage — and what such an
    /// author needs is that the emitter has no rule for this shape, not
    /// an instruction to file a kernel bug.
    ///
    /// Carries the vertex, the one thing a sentence cannot supply and
    /// the key a rule for this shape starts from.
    SeamVertexParentage {
        /// The result-body vertex whose parentage is not determined.
        vertex: VertexKey,
    },
    /// Two faces the derivation believes meet along ONE edge do not:
    /// the rim a combinatorial derivation asks for is not unique.
    ///
    /// **Not an emission inconsistency.** Zero shared edges is an
    /// ordinary property of a sound body (most face pairs of any body
    /// share none) and two shared edges is an ordinary property of a
    /// fragmented one, so neither answer is a claim that the body is
    /// corrupt. What is wrong is the DERIVATION's belief that the pair
    /// has one rim — a belief a declared union invalidates when a
    /// later member splits a merged face.
    ///
    /// A sibling word of [`Self::SeamVertexParentage`] rather than one
    /// generalised over [`super::table::EntityKey`], by
    /// [`Self::FragmentLineage`]'s test: the two name DIFFERENT
    /// structures to go and read — face adjacency in an operand body
    /// here, a vertex's incident edge ROLES there — and their subjects
    /// differ in arity as well as kind, so one word for the class would
    /// hide which structure the missing rule is about.
    ///
    /// The two ways the rim fails to be unique are ONE fact with a
    /// typed discriminant ([`RimShare`]) rather than two words: the
    /// question asked is "is the shared rim unique", the subject is the
    /// same face pair, and the author's move is the same either way.
    /// A `&'static str` here would be [`Self::Emission`] one level
    /// down.
    SharedRim {
        /// One face of the pair whose rim was asked for, in the body
        /// that was walked.
        face: FaceKey,
        /// The other — `ShellError::WallClearance`'s spelling for the
        /// same shape, a refusal about a PAIR of faces of one body.
        other: FaceKey,
        /// What the walk found instead of one edge.
        found: RimShare,
    },
    /// The N2 classification band could not be built from the ambient
    /// tolerance, so no discriminator below it can be decided.
    ///
    /// The cause is NOT unique — a validated
    /// [`Tolerance`](geom_core::tolerance::Tolerance) reaches
    /// [`BandError::InvalidValue`] when K·ε overflows to infinity, and
    /// [`BandError::Empty`] when K·ε rounds back down onto ε, which for
    /// ε = n·2⁻¹⁰⁷⁴ happens exactly when K·n rounds back to n (every K
    /// below 1.5 at the smallest ε; no admitted K above ε = 2⁻¹⁰²³).
    /// So the constructor's own diagnostic rides along rather than being
    /// relabelled as an emission inconsistency, which this is not:
    /// nothing about the result body is wrong here.
    Band(BandError),
    /// An N2 discriminator margin escalated in-band (typed, never a
    /// silent pick — spec D3).
    Escalated {
        /// The named predicate.
        predicate: &'static str,
        /// The escalation, unaltered.
        source: Indeterminate,
    },
}

/// **The one sentence every emission-inconsistency refusal opens
/// with**, written once. Two variants speak it — [`NamingError::Emission`]
/// with a fact, [`NamingError::SplitLineage`] with the record it caught
/// — and a reworded copy would let two refusals of one category read as
/// two categories.
const EMISSION_FRAMING: &str = "a mint-time emission fact was inconsistent with the result body";

/// **The one sentence every missing-rule refusal opens with**, written
/// once, and deliberately NOT a reworded copy of [`EMISSION_FRAMING`]:
/// the two say opposite things about whose fault the failure is, and an
/// author who reads the wrong one goes and files a kernel bug against a
/// recipe that is fine.
const UNRULED_FRAMING: &str = "the emitter has no naming rule for a construction this recipe                                reached — the recipe is well formed and the result body is sound,                                so what is missing is a rule, not a repair";

/// What a shared-rim derivation found instead of the one edge it asked
/// for — the fact a free-text sentence would have hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RimShare {
    /// The two faces are not adjacent at all.
    NotAdjacent,
    /// They meet along more than one edge, so no single edge is the rim.
    Several,
}

impl core::fmt::Display for RimShare {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotAdjacent => write!(f, "no edge"),
            Self::Several => write!(f, "more than one edge"),
        }
    }
}

// #380: the refusal must NAME its subject. Every variant above is
// diagnosed precisely at the emitter — which name collided, which
// entity went uncovered, which upstream table missed, which
// consistency fact broke — and until this impl existed all of that
// died at the `EvalError::Naming` boundary, whose `Display` said only
// "name emission failed". Callers (Python's typed exception text
// included) had to BISECT scenes to relocate a wall the emitter had
// already located. Unlike the kernel-refusal payloads that ride the
// op variants unaltered (`NodeErrorKind`'s Display note, D2), this is
// editor-core's OWN error: rendering it IS the op's vocabulary, and
// there is no other path by which it reaches a human.
impl core::fmt::Display for NamingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            // The role path rides along here, alone among the crate's
            // name renderings: a duplicate mint is a kernel bug report,
            // and the PATH is what distinguishes the colliding name
            // from every other name the node minted.
            Self::Duplicate { name } => write!(
                f,
                "the {name} (role path {:?}) was minted twice — names alias silently only \
                 over the kernel's dead body",
                name.path
            ),
            Self::Unnamed { kind, body } => write!(
                f,
                "a live {} of output body {body} was left unnamed — a kernel-emission \
                 gap, not a naming choice",
                kind.noun()
            ),
            Self::MissingUpstream { node } => write!(
                f,
                "the name table of upstream node {} lacks an entity the emission needed",
                node.0
            ),
            Self::Emission { what } => write!(f, "{EMISSION_FRAMING}: {what}"),
            // The category IS an emission inconsistency, so the framing
            // is the same one; what the caught record adds is the locator.
            Self::SplitLineage(cycle) => write!(f, "{EMISSION_FRAMING}: {cycle}"),
            // The record family is in the sentence, not only the key:
            // `fragment lineage` and `split lineage` are two different
            // things to go and read, and a reader who gets the wrong
            // one searches the wrong map.
            Self::FragmentLineage { face } => write!(
                f,
                "{EMISSION_FRAMING}: fragment lineage of face {face:?} cycles: its \
                 face-fragment rows never reach a root"
            ),
            // The framing is the missing-rule one, not the emission
            // one: what this refusal reports is the absence of a rule,
            // and the vertex is where a rule would be applied.
            Self::SeamVertexParentage { vertex } => write!(
                f,
                "{UNRULED_FRAMING}: seam vertex {vertex:?} has neither one operand-descended \
                 edge on each side nor a pair of seam lines, so its parentage is not determined \
                 by the edges around it"
            ),
            Self::SharedRim { face, other, found } => write!(
                f,
                "{UNRULED_FRAMING}: faces {face:?} and {other:?} share {found} where a rim \
                 derived from adjacency alone needs exactly one"
            ),
            Self::Band(error) => write!(
                f,
                "the N2 classification band could not be built from the ambient tolerance, so \
                 no discriminator below it can be decided: {error}"
            ),
            Self::Escalated { predicate, source } => write!(
                f,
                "the discriminator {predicate} escalated (in-band indeterminacy): {source}"
            ),
        }
    }
}

// `Band::linear(tol)?` rather than a closure at the band door: one
// total conversion, so there is no site at which the caught
// `BandError` could be dropped again.
impl From<BandError> for NamingError {
    fn from(e: BandError) -> Self {
        Self::Band(e)
    }
}

// `body.split_root(..)?` rather than a closure at the chase site: a
// `map_err` closure is one keystroke from `map_err(|_| ..)`, and the
// `EdgeKey` this carries is the only locator a cycling lineage has.
impl From<SplitLineageCycle> for NamingError {
    fn from(e: SplitLineageCycle) -> Self {
        Self::SplitLineage(e)
    }
}

impl From<DuplicateName> for NamingError {
    fn from(e: DuplicateName) -> Self {
        Self::Duplicate { name: e.name }
    }
}

/// A name at `node` with a single role segment.
pub(crate) fn name1(kind: EntityKind, node: RecipeNodeId, seg: super::role::RoleSeg) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

/// An entity in output body `body`.
pub(crate) fn ent(body: u32, key: EntityKey) -> EntityRef {
    EntityRef { body, key }
}

/// The empty table (nodes with no output bodies: datum, profile,
/// declare, empty boolean).
pub(crate) fn empty() -> Arc<NameTable> {
    Arc::new(NameTable::new())
}

/// **An output-body index, as the table carries it.** [`EntityRef::body`]
/// is a `u32`, so a body count past that has no row to land in — the
/// one bound every multi-body value shares, which is why an index past
/// it is this layer's refusal wherever it is met (the evaluator's
/// placers and `Node::Part`, the emitters, the mate walk's `Part`
/// agreement) and not a number silently narrowed.
pub(crate) fn output_body(index: usize) -> Result<u32, NamingError> {
    u32::try_from(index).map_err(|_| NamingError::Emission {
        what: "an output-body index exceeds the table's u32 row width",
    })
}

/// **The placement-major layout**, the one home of its arithmetic:
/// placement `placement` of a master's body `body`, the master holding
/// `per` bodies, is output body `placement·per + body`. `wire_pattern`
/// builds that body there, [`name_pattern`] keys its rows by it, and
/// the mate walk reads a `Part`'s index through it. A `body` at or past
/// `per` is a row the master does not have; a product past `u32` is
/// [`output_body`]'s refusal.
pub(crate) fn flat_body_index(placement: u32, per: u32, body: u32) -> Result<u32, NamingError> {
    if body >= per {
        return Err(NamingError::Emission {
            what: "a pattern master's table names a body the master does not have",
        });
    }
    placement
        .checked_mul(per)
        .and_then(|b| b.checked_add(body))
        .ok_or(NamingError::Emission {
            what: "an output-body index exceeds the table's u32 row width",
        })
}

/// Wraps a pattern master's table per structural placement index
/// (A8/N1 `Instance(j)`): placement `j` holds the master's keys
/// verbatim (`transform_rigid` key-stability).
///
/// **The wrapping is uniform** (ASM-2K D-2): `Instance(j)` wraps EVERY
/// name of the master, whatever the master's body holds. A master with
/// several SOLIDS is one such master and is admitted — its names are
/// already distinct within it (derivation paths tell its solids apart),
/// and one qualifier per placement carries that distinctness across the
/// placements; no per-solid sub-index exists, because which solid a
/// name denotes is read off the name's own derivation, never off the
/// instance qualifier.
///
/// **The instance×body layout.** The master has `per` output bodies —
/// one for a body-valued input, `M` for an `Instances` value placed
/// whole (a nested pattern) — and the wrapped table is placement-major
/// over them: the master's row at body `i` lands, under placement
/// `j`, at output body `j·per + i`, the index `wire_pattern` builds
/// that body at and `Node::Part` selects it by. For a one-body master
/// the body index IS the placement index. A nested pattern's name is
/// therefore `Instance(j)` over the inner `Instance(i)` over the
/// master's name — the chain the mate walk consumes outermost first —
/// and no row of the master is re-keyed past `per`: one at a body the
/// master does not have is the input's emission bug, refused typed
/// ([`flat_body_index`]). Totality is checked against every output body.
pub(crate) fn name_pattern<T: geom_core::Real>(
    node: RecipeNodeId,
    master: &NameTable,
    n: i64,
    per: usize,
    instances: &[Arc<Body<T>>],
) -> Result<Arc<NameTable>, NamingError> {
    let per = output_body(per)?;
    // An operand table read WHOLE seals here, exactly as one read an
    // entity at a time seals in `upstream_name`, and every row below
    // embeds the master's own handle rather than a copy of it.
    master.seal_order();
    let mut t = NameTable::new();
    for j in 0..n {
        let ju = output_body(usize::try_from(j).unwrap_or(usize::MAX))?;
        // Output body of the master's body `i` under placement `j`.
        let at = |e: &EntityRef| -> Result<EntityRef, NamingError> {
            Ok(ent(flat_body_index(ju, per, e.body)?, e.key))
        };
        for (name, entry) in master.iter_refs() {
            let wrapped = StableName {
                kind: name.kind,
                node,
                path: vec![super::role::RoleSeg::Instance {
                    i: ju,
                    of: name.clone(),
                }],
            };
            match entry {
                super::table::Entry::Unique(e) => t.insert(wrapped, at(e)?)?,
                super::table::Entry::Tied(es) => {
                    let rows = es.iter().map(at).collect::<Result<Vec<_>, _>>()?;
                    t.insert_tied(wrapped, rows)?;
                }
            }
        }
    }
    for (i, body) in instances.iter().enumerate() {
        check_total(&t, body, output_body(i)?)?;
    }
    Ok(Arc::new(t))
}

/// Names a [`crate::node::Node::PlacedUnion`]'s ONE output body
/// (GROUP-BOOLEAN-DESIGN, ratified A′).
///
/// The vocabulary does NOT grow: per-instance discrimination is
/// [`name_pattern`]'s own `Instance(i)` wrapper, segment for segment,
/// so "instance 7's cavity face" is the same one-row selector on a
/// fused group as on an unfused pattern. What differs is the TARGET —
/// a PlacedUnion emits one body, so every instance's rows land at body
/// index 0, re-keyed through that instance's graft bridge (`bridges[i]`
/// is the correspondence the graft of instance `i` established).
///
/// The BODY name is minted here rather than wrapped, for
/// [`name_in_part`]'s reason: the fused body is not the body any
/// instance-local name denotes, so it is this node's own output body,
/// named like every other body-producing op's. The prototype's own
/// body-kind rows therefore do not carry — they have nothing left to
/// point at, exactly as the product gather rules.
pub(crate) fn name_placed_union<T: geom_core::Real>(
    node: RecipeNodeId,
    master: &NameTable,
    bridges: &[topo::GraftKeys],
    fused: &Body<T>,
) -> Result<Arc<NameTable>, NamingError> {
    // The prototype's table is read whole; sealing it here is what
    // `upstream_name` does for a table read an entity at a time.
    master.seal_order();
    let mut t = NameTable::new();
    t.insert(
        name1(EntityKind::Body, node, super::role::RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;
    for (i, keys) in bridges.iter().enumerate() {
        let iu = output_body(i)?;
        let mapped = |key: EntityKey| -> Option<EntityKey> {
            match key {
                EntityKey::Body => None,
                EntityKey::Face(f) => keys.face(f).map(EntityKey::Face),
                EntityKey::Edge(e) => keys.edge(e).map(EntityKey::Edge),
                EntityKey::Vertex(v) => keys.vertex(v).map(EntityKey::Vertex),
            }
        };
        for (name, entry) in master.iter_refs() {
            let wrapped = StableName {
                kind: name.kind,
                node,
                path: vec![super::role::RoleSeg::Instance {
                    i: iu,
                    of: name.clone(),
                }],
            };
            // The prototype is ONE body — a placed union fuses what
            // `body_operand` admits, and a value of several bodies is
            // `Pattern`'s to place, never this node's to fuse — so its
            // table is a single-output-body table, and a row at any
            // other index is a bug: surfaced, not dropped.
            let rows: Vec<EntityRef> = match entry {
                super::table::Entry::Unique(e) => vec![*e],
                super::table::Entry::Tied(es) => es.clone(),
            };
            if rows.iter().any(|e| e.body != 0) {
                return Err(NamingError::Emission {
                    what: "a placed union's prototype table names a body the prototype does not have",
                });
            }
            let moved: Vec<EntityRef> = rows
                .into_iter()
                .filter_map(|e| mapped(e.key).map(|key| ent(0, key)))
                .collect();
            match moved.len() {
                0 => {}
                1 => t.insert(wrapped, moved[0])?,
                _ => t.insert_tied(wrapped, moved)?,
            }
        }
    }
    check_total(&t, fused, 0)?;
    Ok(Arc::new(t))
}

/// Wraps an instantiated part's product table under the instance
/// (ASM-2A D-4: the GQ4 wrapper composed with N1–N7).
///
/// Every stable name of the referenced part's product body is re-minted
/// as `InPart(part-local name)` at the INSTANTIATE node, so two
/// instances of one part have wholly distinct names (their nodes
/// differ) while a part-document edit that breaks a local name surfaces
/// through the unchanged N1–N7 diagnosis ladder — the wrapped name is
/// still in there, structurally.
///
/// Keys hold verbatim: `product_named` keyed the part table onto the
/// product body, and `transform_rigid` is key-stable, so the placed
/// copy's arena answers to exactly those keys.
///
/// The BODY name is minted here rather than wrapped: the part's product
/// is not a body any part-local name denotes (see
/// [`crate::product::product_named`]), and an instance's placed body is
/// this node's own output, named like every other body-producing op's.
pub(crate) fn name_in_part<T: geom_core::Real>(
    node: RecipeNodeId,
    part: &NameTable,
    placed: &Body<T>,
) -> Result<Arc<NameTable>, NamingError> {
    // The part's table is read whole; sealing it here is what
    // `upstream_name` does for a table read an entity at a time.
    part.seal_order();
    let mut t = NameTable::new();
    t.insert(
        name1(EntityKind::Body, node, super::role::RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;
    for (name, entry) in part.iter_refs() {
        let wrapped = StableName {
            kind: name.kind,
            node,
            path: vec![super::role::RoleSeg::InPart { of: name.clone() }],
        };
        // The part's table is the PRODUCT's: one body, index 0. A row
        // anywhere else is a gather bug, surfaced rather than dropped.
        let misplaced = || NamingError::Emission {
            what: "an instantiated part's product table names a body other than the product",
        };
        match entry {
            super::table::Entry::Unique(e) => {
                if e.body != 0 {
                    return Err(misplaced());
                }
                t.insert(wrapped, ent(0, e.key))?;
            }
            super::table::Entry::Tied(es) => {
                if es.iter().any(|e| e.body != 0) {
                    return Err(misplaced());
                }
                t.insert_tied(wrapped, es.iter().map(|e| ent(0, e.key)).collect())?;
            }
        }
    }
    check_total(&t, placed, 0)?;
    Ok(Arc::new(t))
}

/// Per-body topology walk products the emitters share: half-edge
/// incidence (vertex → edges, edge → faces) built in ONE deterministic
/// arena-order pass — combinatorial wiring facts, not inspection.
pub(crate) struct Incidence {
    /// Vertex → incident edges (sorted, deduplicated).
    pub vertex_edges: BTreeMap<VertexKey, Vec<EdgeKey>>,
    /// Edge → the (≤ 2) faces of its halves' loops.
    pub edge_faces: BTreeMap<EdgeKey, Vec<FaceKey>>,
}

impl Incidence {
    /// Builds the incidence maps for one body.
    pub(crate) fn of<T: geom_core::Real>(body: &Body<T>) -> Result<Self, NamingError> {
        let bug = || NamingError::Emission {
            what: "incidence walk: dangling half-edge reference",
        };
        let mut vertex_edges: BTreeMap<VertexKey, Vec<EdgeKey>> = BTreeMap::new();
        let mut edge_faces: BTreeMap<EdgeKey, Vec<FaceKey>> = BTreeMap::new();
        for (_, he) in body.half_edges() {
            vertex_edges.entry(he.start).or_default().push(he.edge);
            let face = body.get_loop(he.parent_loop).ok_or_else(bug)?.face;
            edge_faces.entry(he.edge).or_default().push(face);
        }
        for v in vertex_edges.values_mut() {
            v.sort_unstable();
            v.dedup();
        }
        for f in edge_faces.values_mut() {
            f.sort_unstable();
            f.dedup();
        }
        Ok(Self {
            vertex_edges,
            edge_faces,
        })
    }
}

/// The unique edge shared by face `f` and face `g` (cap–wall rims:
/// derived combinatorially from emitted anchors).
///
/// **Two different failures, and they are not the same category.** The
/// walk's three dangling-key arms are structural corruption of `body` —
/// `topo`'s derived accessors return `None` exactly for a stale key or
/// a broken edge/half-edge bijection — so they stay
/// [`NamingError::Emission`]. Finding no shared edge, or two, is not:
/// both are ordinary properties of a sound body, and what they refute
/// is the CALLER's belief that this pair has one rim. That refusal is
/// [`NamingError::SharedRim`], which names the pair.
pub(crate) fn unique_shared_edge<T: geom_core::Real>(
    body: &Body<T>,
    f: FaceKey,
    g: FaceKey,
) -> Result<EdgeKey, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let rim = |found| NamingError::SharedRim {
        face: f,
        other: g,
        found,
    };
    let mut found: Option<EdgeKey> = None;
    for he in face_half_edges(body, f)? {
        let mate = body
            .mate(he)
            .ok_or_else(|| bug("shared-edge walk: unmated half-edge"))?;
        let mate_he = body
            .get_half_edge(mate)
            .ok_or_else(|| bug("shared-edge walk: dangling mate"))?;
        let other = body
            .get_loop(mate_he.parent_loop)
            .ok_or_else(|| bug("shared-edge walk: dangling loop"))?
            .face;
        if other == g {
            let edge = mate_he.edge;
            if found.is_some_and(|e| e != edge) {
                return Err(rim(RimShare::Several));
            }
            found = Some(edge);
        }
    }
    found.ok_or_else(|| rim(RimShare::NotAdjacent))
}

/// All half-edges of a face (outer loop + rings), deterministic
/// order.
pub(crate) fn face_half_edges<T: geom_core::Real>(
    body: &Body<T>,
    f: FaceKey,
) -> Result<Vec<HalfEdgeKey>, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let face = body
        .get_face(f)
        .ok_or_else(|| bug("face walk: dangling face"))?;
    let mut out = Vec::new();
    for l in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        match body
            .get_loop(l)
            .ok_or_else(|| bug("face walk: dangling loop"))?
            .boundary
        {
            topo::LoopBoundary::Empty { .. } => {}
            topo::LoopBoundary::Cycle { first } => {
                out.extend(
                    body.loop_cycle(first)
                        .ok_or_else(|| bug("face walk: unwalkable loop"))?,
                );
            }
        }
    }
    Ok(out)
}

/// The two endpoint vertices of an edge.
pub(crate) fn edge_ends<T: geom_core::Real>(
    body: &Body<T>,
    e: EdgeKey,
) -> Result<(VertexKey, VertexKey), NamingError> {
    let bug = |what| NamingError::Emission { what };
    let edge = body
        .get_edge(e)
        .ok_or_else(|| bug("edge ends: dangling edge"))?;
    let start = body
        .get_half_edge(edge.he_plus)
        .ok_or_else(|| bug("edge ends: dangling he_plus"))?
        .start;
    let end = body
        .half_edge_end(edge.he_plus)
        .ok_or_else(|| bug("edge ends: he_plus has no end"))?;
    Ok((start, end))
}

/// TOTALITY (spec D4): every live face/edge/vertex of `body` (output
/// body index `ix`) is covered by the table, plus the body row
/// itself.
pub(crate) fn check_total<T: geom_core::Real>(
    table: &NameTable,
    body: &Body<T>,
    ix: u32,
) -> Result<(), NamingError> {
    let miss = |kind| NamingError::Unnamed { kind, body: ix };
    if table.name_of(&ent(ix, EntityKey::Body)).is_none() {
        return Err(miss(EntityKind::Body));
    }
    for (f, _) in body.faces() {
        if table.name_of(&ent(ix, EntityKey::Face(f))).is_none() {
            return Err(miss(EntityKind::Face));
        }
    }
    for (e, _) in body.edges() {
        if table.name_of(&ent(ix, EntityKey::Edge(e))).is_none() {
            return Err(miss(EntityKind::Edge));
        }
    }
    for (v, _) in body.vertices() {
        if table.name_of(&ent(ix, EntityKey::Vertex(v))).is_none() {
            return Err(miss(EntityKind::Vertex));
        }
    }
    Ok(())
}

/// **ASM-2K D-2 rows**: the uniform `Instance(i)` wrapping over a
/// master whose body holds SEVERAL SOLIDS — the shape an assembly's
/// instantiated part arrives in (`topo::graft_disjoint_all`). Kernel
/// level on purpose: no document node produces a multi-solid body yet
/// (solids are minted inside `topo` alone), so the master here is
/// built at the door that will feed one.
#[cfg(test)]
mod pattern_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::Tol;
    use std::collections::BTreeSet;
    use std::sync::Arc;

    use geom_core::{Affine3, Vec3};
    use topo::Body;

    use super::*;
    use crate::names::emit_sweep::name_extrude;
    use crate::names::role::RoleSeg;
    use crate::names::table::Entry;
    use crate::node::RecipeNodeId;
    use profile::RawLoop;

    /// A unit cube at `dx`, with its extrude's own name table.
    fn cube(node: RecipeNodeId, dx: f64) -> (Body<f64>, Arc<NameTable>) {
        let plane = profile::SketchPlane::from_frame(
            geom_core::Point3::new(dx, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let square = profile::ProfileLoop::polygon(
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
                .into_iter()
                .map(|(x, y)| geom_core::Point2::new(x, y)),
        );
        let prof = profile::Profile::new(plane, vec![square])
            .validate(geom_core::Tol::witness())
            .unwrap();
        let built =
            sweep::extrude(&prof, sweep::Extrusion::Distance(1.0_f64), Tol::witness()).unwrap();
        let table = name_extrude(node, &built).unwrap();
        (built.body, table)
    }

    /// A master body holding TWO solids, and a table that covers it:
    /// solid 0 wears its own extrude's names (node 1), solid 1 wears
    /// node 2's, re-keyed kind-wise in arena order onto the grafted
    /// copies. Two derivations, so the master's names are distinct
    /// within the master — which is the premise the wrapping rests on.
    /// WHICH grafted face wears which of node 2's names is immaterial
    /// to these rows (they pin the wrapping); minting a faithful key
    /// bridge across a graft is the consumer's job, not this door's.
    fn two_solid_master() -> (Body<f64>, NameTable) {
        let (mut body, a) = cube(RecipeNodeId(1), 0.0);
        let (second, b) = cube(RecipeNodeId(2), 10.0);
        let was: (BTreeSet<_>, BTreeSet<_>, BTreeSet<_>) = (
            body.faces().map(|(k, _)| k).collect(),
            body.edges().map(|(k, _)| k).collect(),
            body.vertices().map(|(k, _)| k).collect(),
        );
        topo::graft_disjoint(&mut body, &second, Tol::witness()).expect("a two-solid master");
        let fresh_f: Vec<_> = body
            .faces()
            .map(|(k, _)| k)
            .filter(|k| !was.0.contains(k))
            .collect();
        let fresh_e: Vec<_> = body
            .edges()
            .map(|(k, _)| k)
            .filter(|k| !was.1.contains(k))
            .collect();
        let fresh_v: Vec<_> = body
            .vertices()
            .map(|(k, _)| k)
            .filter(|k| !was.2.contains(k))
            .collect();

        let mut t = NameTable::new();
        for (name, entry) in a.iter() {
            match entry {
                Entry::Unique(e) => t.insert(name.clone(), *e).unwrap(),
                Entry::Tied(es) => t.insert_tied(name.clone(), es.clone()).unwrap(),
            }
        }
        let (mut fi, mut ei, mut vi) = (0, 0, 0);
        for (name, entry) in b.iter() {
            let Entry::Unique(e) = entry else {
                panic!("the fixture's extrude ties nothing");
            };
            // The aggregate is ONE body: it keeps solid 0's body name.
            let key = match e.key {
                EntityKey::Body => continue,
                EntityKey::Face(_) => {
                    fi += 1;
                    EntityKey::Face(fresh_f[fi - 1])
                }
                EntityKey::Edge(_) => {
                    ei += 1;
                    EntityKey::Edge(fresh_e[ei - 1])
                }
                EntityKey::Vertex(_) => {
                    vi += 1;
                    EntityKey::Vertex(fresh_v[vi - 1])
                }
            };
            t.insert(name.clone(), ent(0, key)).unwrap();
        }
        assert_eq!(
            (fi, ei, vi),
            (fresh_f.len(), fresh_e.len(), fresh_v.len()),
            "the second solid is covered exactly"
        );
        (body, t)
    }

    /// The pattern's instance bodies for a master and a z-step —
    /// `wire_pattern`'s own shape: instance 0 verbatim, the rest
    /// through the key-stable placement door.
    fn instances(master: &Body<f64>, n: i64, step: f64) -> Vec<Arc<Body<f64>>> {
        (0..n)
            .map(|i| {
                Arc::new(if i == 0 {
                    master.clone()
                } else {
                    topo::transform_rigid(
                        master,
                        &Affine3::translation(Vec3::new(0.0, 0.0, step * i as f64)),
                        Tol::witness(),
                    )
                    .unwrap()
                })
            })
            .collect()
    }

    /// **Row 4.** A pattern of a two-solid master is admitted: N×2
    /// solids, N × the master's census, and every stable name distinct
    /// — one `Instance(i)` over the master's own name, no per-solid
    /// sub-index anywhere in the path.
    #[test]
    fn a_multi_solid_master_patterns_with_uniform_instance_wrapping() {
        let (master_body, master) = two_solid_master();
        assert_eq!(master_body.solids().count(), 2, "a two-solid master");
        let n = 3_i64;
        let bodies = instances(&master_body, n, 5.0);
        let node = RecipeNodeId(9);
        let t =
            name_pattern(node, &master, n, 1, &bodies).expect("a multi-solid master is admitted");

        let times = usize::try_from(n).unwrap();
        assert_eq!(t.len(), master.len() * times, "census: N × the master's");
        assert_eq!(
            bodies.iter().map(|b| b.solids().count()).sum::<usize>(),
            2 * times,
            "N × 2 solids"
        );
        assert_eq!(
            bodies.iter().map(|b| b.faces().count()).sum::<usize>(),
            master_body.faces().count() * times
        );

        let mut seen = BTreeSet::new();
        for (name, entry) in t.iter() {
            assert!(seen.insert(name.clone()), "a name repeated: {name:?}");
            assert_eq!(name.node, node);
            assert_eq!(name.path.len(), 1, "one qualifier, not one per solid");
            let RoleSeg::Instance { i, of } = &name.path[0] else {
                panic!("not wrapped: {name:?}");
            };
            assert!(master.lookup(of).is_some(), "wraps a master name");
            if let Entry::Unique(e) = entry {
                assert_eq!(e.body, *i, "the instance index IS the body index");
            }
        }
        assert_eq!(seen.len(), master.len() * times, "all distinct");
    }

    /// **Row 5.** Names of instance `i` resolve to instance `i`'s
    /// geometry: the wrapped name looks up to `(body i, the master's
    /// key)`, the key is live in that instance (placement is
    /// key-stable), and the entity it names sits where instance `i`
    /// sits — for BOTH of the master's solids.
    #[test]
    fn instance_i_names_resolve_to_instance_i_geometry() {
        let (master_body, master) = two_solid_master();
        let (n, step) = (3_i64, 5.0);
        let bodies = instances(&master_body, n, step);
        let node = RecipeNodeId(9);
        let t = name_pattern(node, &master, n, 1, &bodies).expect("admitted");

        let mut checked = 0;
        for i in 0..n {
            let iu = u32::try_from(i).unwrap();
            for (name, entry) in master.iter() {
                let Entry::Unique(e) = entry else { continue };
                let wrapped = StableName {
                    kind: name.kind,
                    node,
                    path: vec![RoleSeg::Instance {
                        i: iu,
                        of: name.clone().into(),
                    }],
                };
                assert_eq!(t.lookup(&wrapped), Some(&Entry::Unique(ent(iu, e.key))));
                assert_eq!(t.name_of(&ent(iu, e.key)), Some(&wrapped));
                let EntityKey::Vertex(v) = e.key else {
                    continue;
                };
                let at = |b: &Body<f64>| {
                    let p = b.get_vertex(v).expect("key-stable placement").point;
                    *b.get_point(p).unwrap()
                };
                let (m, inst) = (at(&master_body), at(&bodies[usize::try_from(i).unwrap()]));
                assert_eq!(inst.x, m.x, "unmoved across the pattern direction");
                assert_eq!(inst.z, m.z + step * i as f64, "instance {i}'s geometry");
                checked += 1;
            }
        }
        assert_eq!(checked, 16 * 3, "both solids' 8 vertices, every instance");
    }

    /// A master's table re-keyed onto body `body`, names verbatim.
    fn at_body(table: &NameTable, body: u32) -> NameTable {
        let mut t = NameTable::new();
        for (name, entry) in table.iter() {
            let Entry::Unique(e) = entry else {
                panic!("the fixture's extrude ties nothing");
            };
            t.insert(name.clone(), ent(body, e.key)).unwrap();
        }
        t
    }

    /// A master row at a body the master does not have refuses typed:
    /// the layout re-keys body `i < per` to `j·per + i`, and a row past
    /// `per` is the input's own emission bug, never re-keyed into
    /// another placement's range.
    #[test]
    fn a_master_row_past_the_masters_body_count_refuses_typed() {
        let (body, a) = cube(RecipeNodeId(1), 0.0);
        let master = at_body(&a, 1);
        let err = name_pattern(RecipeNodeId(9), &master, 2, 1, &[Arc::new(body)])
            .expect_err("a row past the master's body count must refuse");
        assert!(
            format!("{err:?}").contains("does not have"),
            "typed, and about the body: {err:?}"
        );
    }

    /// **The instance×body layout**, at the emitter: a two-body master
    /// (`per = 2`) under `n = 3` placements names `3 × 2` bodies,
    /// body `i`'s row under placement `j` at output body `j·2 + i`,
    /// wrapped `Instance(j)` over the master's own name; totality
    /// holds over all six.
    #[test]
    fn a_multi_output_body_master_lays_out_placement_major() {
        let (b0, a) = cube(RecipeNodeId(1), 0.0);
        let (b1, b) = cube(RecipeNodeId(2), 10.0);
        let mut master = at_body(&a, 0);
        for (name, entry) in at_body(&b, 1).iter() {
            let Entry::Unique(e) = entry else {
                panic!("the fixture's extrude ties nothing");
            };
            master.insert(name.clone(), *e).unwrap();
        }
        let (n, per, step) = (3_i64, 2_usize, 5.0);
        let mut bodies: Vec<Arc<Body<f64>>> = Vec::new();
        for j in 0..n {
            for body in [&b0, &b1] {
                bodies.push(Arc::new(if j == 0 {
                    body.clone()
                } else {
                    topo::transform_rigid(
                        body,
                        &Affine3::translation(Vec3::new(0.0, 0.0, step * j as f64)),
                        Tol::witness(),
                    )
                    .unwrap()
                }));
            }
        }
        let node = RecipeNodeId(9);
        let t = name_pattern(node, &master, n, per, &bodies).expect("admitted");
        assert_eq!(t.len(), master.len() * 3, "census: N × the master's");
        for j in 0..n {
            let ju = u32::try_from(j).unwrap();
            for (name, entry) in master.iter() {
                let Entry::Unique(e) = entry else { continue };
                let wrapped = StableName {
                    kind: name.kind,
                    node,
                    path: vec![RoleSeg::Instance {
                        i: ju,
                        of: name.clone().into(),
                    }],
                };
                let flat = ju * u32::try_from(per).unwrap() + e.body;
                assert_eq!(
                    t.lookup(&wrapped),
                    Some(&Entry::Unique(ent(flat, e.key))),
                    "body {} under placement {j} is output body {flat}",
                    e.body
                );
            }
        }
    }
}

/// **#380**: the diagnostic must SURVIVE the `EvalError` boundary. The
/// bug these rows close was not a missing diagnosis — every emitter
/// already refused precisely — but a rendering that threw the
/// diagnosis away, leaving `Display` consumers (Python's typed
/// exception text among them) with "name emission failed" and a
/// bisect. Substring pins, per the [`geom_core::COINCIDENCE_RECOURSE`]
/// convention: prose may be reworded, the SUBJECT may not vanish.
#[cfg(test)]
mod display_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::{Band, MarginDiag};

    use super::*;
    use crate::eval::NodeErrorKind;
    use crate::names::role::RoleSeg;

    fn escalation() -> Indeterminate {
        Indeterminate {
            margin: MarginDiag::Invalid,
            band: Band::new(1e-9, 1e-6).unwrap(),
            predicate: Some("side_of_plane"),
        }
    }

    /// **Two distinct edge keys, out of a real arena.** `EdgeKey` is a
    /// slotmap key and nothing in this crate mints one by hand, so the
    /// arena is the only source; nothing below depends on their VALUES,
    /// only on their being distinct and rendering distinctly.
    fn two_edges() -> (EdgeKey, EdgeKey) {
        // Two solids in ONE arena: keys are per-body, so two bodies
        // would hand out the same index twice.
        let mut body = topo::Body::<f64>::new();
        let mut mint = |x: f64| {
            let born = body
                .mvfs(geom_core::Point3::new(x, 0.0, 0.0))
                .expect("mvfs births a solid, shell, face and lone vertex");
            body.mev_line(
                topo::MevSite::Lone {
                    r#loop: born.r#loop,
                },
                geom_core::Point3::new(x + 1.0, 0.0, 0.0),
                geom_core::Tol::witness(),
            )
            .expect("mev on an empty loop grows it by one edge")
            .edge
        };
        let (a, b) = (mint(0.0), mint(10.0));
        assert_ne!(a, b, "two DISTINCT keys, or the rows below prove nothing");
        (a, b)
    }

    /// **Two distinct face keys, out of a real arena.** `FaceKey` is a
    /// slotmap key and nothing in this crate mints one by hand, so the
    /// arena is the only source; two solids in ONE arena is what makes
    /// the keys different, and nothing below depends on their VALUES.
    fn two_faces() -> (FaceKey, FaceKey) {
        let mut body = topo::Body::<f64>::new();
        let mut mint = |x: f64| {
            body.mvfs(geom_core::Point3::new(x, 0.0, 0.0))
                .expect("mvfs births a solid, shell, face and lone vertex")
                .face
        };
        let (a, b) = (mint(0.0), mint(10.0));
        assert_ne!(a, b, "two DISTINCT keys, or the rows below prove nothing");
        (a, b)
    }

    /// Two DISTINCT vertex keys, minted the same way.
    fn two_vertices() -> (VertexKey, VertexKey) {
        let mut body = topo::Body::<f64>::new();
        let mut mint = |x: f64| {
            body.mvfs(geom_core::Point3::new(x, 0.0, 0.0))
                .expect("mvfs births a solid, shell, face and lone vertex")
                .vertex
        };
        let (a, b) = (mint(0.0), mint(10.0));
        assert_ne!(a, b, "two DISTINCT keys, or the rows below prove nothing");
        (a, b)
    }

    /// **The cycling edge survives the conversion and the node
    /// boundary — and it is THIS edge, not a constant.** The node-level
    /// prose is the only route by which an emitter refusal reaches a
    /// human (Python's typed exception message is exactly this string),
    /// so the locator is pinned there rather than at `NamingError`'s own
    /// `Display`. Two different keys must give two different sentences:
    /// a refusal that reads the same for both has no locator, which is
    /// the defect `map_err(|_| ..)` used to have here.
    #[test]
    fn the_cycling_edge_reaches_the_node_level_prose() {
        let (a, b) = two_edges();
        let carried =
            |edge| NodeErrorKind::Naming(NamingError::from(SplitLineageCycle { edge })).to_string();

        let sa = carried(a);
        assert!(
            sa.contains(&format!("{a:?}")),
            "the cycling edge is the only locator this failure has: {sa}"
        );
        let sb = carried(b);
        assert!(sb.contains(&format!("{b:?}")), "{sb}");
        assert_ne!(
            sa, sb,
            "a refusal that reads the same for two edges has no locator"
        );
        assert!(
            !sa.contains(&format!("{b:?}")),
            "the refusal names the edge it caught, not another: {sa}"
        );

        // The category stays honest: a corrupt birth record IS an
        // emission inconsistency, so the framing sentence is the one
        // `Emission` speaks and the node boundary's own word survives it.
        assert!(sa.contains(EMISSION_FRAMING), "category kept: {sa}");
        assert!(
            sa.contains("name emission failed"),
            "node category kept: {sa}"
        );

        // `crate::py::typed_err` (pncad-py) asserts `reads_as_prose` on
        // every raise, live under release, and its fingerprint is the
        // field brace. A payload rendered through a derived `Debug` is
        // how that assertion gets broken, and this refusal carries one.
        for s in [&sa, &sb] {
            assert!(
                !s.contains(" { "),
                "a braced payload panics the Python binding at the arm \
                 meant to refuse gracefully: {s}"
            );
        }
    }

    /// **The cycling FACE survives to the node-level prose — and it is
    /// THIS face.** The sibling of the row above, for the third
    /// bounded lineage walk: `emit_topo`'s `chase` used to fall out of
    /// its budget and return the cursor it was holding, which became a
    /// group key and named faces after a stranger. It refuses now, and
    /// what makes the refusal worth having rather than a sentence is
    /// that the key reaches the human: two different faces must give
    /// two different sentences.
    #[test]
    fn the_cycling_face_reaches_the_node_level_prose() {
        let (a, b) = two_faces();
        let carried =
            |face| NodeErrorKind::Naming(NamingError::FragmentLineage { face }).to_string();

        let sa = carried(a);
        assert!(
            sa.contains(&format!("{a:?}")),
            "the cycling face is the only locator this failure has: {sa}"
        );
        let sb = carried(b);
        assert!(sb.contains(&format!("{b:?}")), "{sb}");
        assert_ne!(
            sa, sb,
            "a refusal that reads the same for two faces has no locator"
        );
        assert!(
            !sa.contains(&format!("{b:?}")),
            "the refusal names the face it caught, not another: {sa}"
        );

        // The category is the same as the edge cycle's — a corrupt
        // mint-time record — and the record FAMILY is what separates
        // the two sentences, which is the whole argument for a second
        // word rather than one generalised over the key's kind.
        assert!(sa.contains(EMISSION_FRAMING), "category kept: {sa}");
        assert!(
            sa.contains("name emission failed"),
            "node category kept: {sa}"
        );
        assert!(
            sa.contains("fragment lineage") && !sa.contains("split lineage"),
            "the face cycle names the record family it caught, not the edge \
             walk's: {sa}"
        );

        // `crate::py::typed_err` (pncad-py) asserts `reads_as_prose` on
        // every raise, live under release, and its fingerprint is the
        // field brace.
        for s in [&sa, &sb] {
            assert!(
                !s.contains(" { "),
                "a braced payload panics the Python binding at the arm \
                 meant to refuse gracefully: {s}"
            );
        }
    }

    /// Every variant names WHICH entity/node/fact it refused on.
    ///
    /// **The enumeration is the compiler's claim, not prose**: the
    /// `sampled` match below is exhaustive, so a new variant has no arm
    /// until someone writes one, and the arm names the row that samples
    /// it, so a row that goes missing reds the set comparison.
    #[test]
    fn every_variant_names_its_subject() {
        let name = StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(7),
            path: vec![RoleSeg::Cap(super::super::role::CapEnd::End)],
        };
        let rows: Vec<(NamingError, Vec<&str>)> = vec![
            (
                NamingError::Duplicate {
                    name: Box::new(name),
                },
                vec!["face", "7", "Cap"],
            ),
            (
                NamingError::Unnamed {
                    kind: EntityKind::Edge,
                    body: 3,
                },
                vec!["edge", "3"],
            ),
            (
                NamingError::MissingUpstream {
                    node: RecipeNodeId(11),
                },
                vec!["11"],
            ),
            (
                // A refusal that still EXISTS (ties propagate, so there
                // is no tied-upstream refusal to sample): a sample
                // payload that greps to nothing would outlive its own
                // subject.
                NamingError::Emission {
                    what: "section face classified On",
                },
                vec!["section face classified On"],
            ),
            (
                NamingError::Escalated {
                    predicate: "side_of_plane",
                    source: escalation(),
                },
                vec!["side_of_plane"],
            ),
            (
                NamingError::SplitLineage(SplitLineageCycle {
                    edge: two_edges().0,
                }),
                vec!["split lineage of edge"],
            ),
            (
                NamingError::FragmentLineage {
                    face: two_faces().0,
                },
                vec!["fragment lineage of face"],
            ),
            (
                NamingError::SeamVertexParentage {
                    vertex: two_vertices().0,
                },
                vec!["seam vertex"],
            ),
            (
                NamingError::SharedRim {
                    face: two_faces().0,
                    other: two_faces().1,
                    found: RimShare::Several,
                },
                vec!["more than one edge"],
            ),
            (
                // The band's subject is the pair of thresholds that
                // could not separate: which end of the axis the ambient
                // tolerance landed on is what tells the reader whether
                // to raise eps or lower K.
                NamingError::Band(BandError::Empty {
                    zero: 5e-324,
                    escalate: 5e-324,
                }),
                vec!["5e-324"],
            ),
        ];
        let sampled = |err: &NamingError| -> usize {
            match err {
                NamingError::Duplicate { .. } => 0,
                NamingError::Unnamed { .. } => 1,
                NamingError::MissingUpstream { .. } => 2,
                NamingError::Emission { .. } => 3,
                NamingError::Escalated { .. } => 4,
                NamingError::SplitLineage(_) => 5,
                NamingError::FragmentLineage { .. } => 6,
                NamingError::SeamVertexParentage { .. } => 7,
                NamingError::SharedRim { .. } => 8,
                NamingError::Band(_) => 9,
            }
        };
        let covered: std::collections::BTreeSet<usize> =
            rows.iter().map(|(e, _)| sampled(e)).collect();
        assert_eq!(
            covered,
            (0..rows.len()).collect::<std::collections::BTreeSet<_>>(),
            "a variant lost its row, so \"every variant\" is prose again"
        );
        for (err, wanted) in rows {
            let shown = err.to_string();
            for w in wanted {
                assert!(shown.contains(w), "{err:?} rendered without {w:?}: {shown}");
            }
        }
    }

    /// **A body that VALIDATES answers "no shared edge", so that
    /// answer cannot be a claim that the body is corrupt.**
    ///
    /// This is the whole argument for [`NamingError::SharedRim`]. A
    /// unit cube's two caps are not adjacent — as most face pairs of
    /// most bodies are not — and asking a sound body for a rim between
    /// them is a question about the CALLER's belief, not about the
    /// body. The control is the pair that does share exactly one edge:
    /// if the walk stopped finding rims at all, the refusal below would
    /// pass for the wrong reason.
    #[test]
    fn a_sound_body_has_face_pairs_with_no_rim_and_face_pairs_with_one() {
        use geom_core::{Point2, Vec3};
        use profile::RawLoop;
        let plane = profile::SketchPlane::from_frame(
            geom_core::Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let square = profile::ProfileLoop::polygon(
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
                .into_iter()
                .map(|(x, y)| Point2::new(x, y)),
        );
        let prof = profile::Profile::new(plane, vec![square])
            .validate(geom_core::Tol::witness())
            .expect("a unit square validates");
        let cube = sweep::extrude(
            &prof,
            sweep::Extrusion::Distance(1.0_f64),
            geom_core::Tol::witness(),
        )
        .expect("a unit cube extrudes");
        topo::validate_closed(&cube.body).expect("the cube is a sound closed body");

        let wall = cube.side_faces[0][0];
        let rim = unique_shared_edge(&cube.body, cube.top, wall)
            .expect("a cap and a wall of a box share exactly one rim");
        assert!(
            cube.body.get_edge(rim).is_some(),
            "the control must return a LIVE edge, or the refusal below \
             passes because the walk finds nothing at all"
        );

        match unique_shared_edge(&cube.body, cube.top, cube.bottom) {
            Err(NamingError::SharedRim {
                face,
                other,
                found: RimShare::NotAdjacent,
            }) => {
                assert_eq!(
                    (face, other),
                    (cube.top, cube.bottom),
                    "the refusal names the pair it was asked about"
                );
            }
            other => panic!("opposite caps of a box share no rim, got {other:?}"),
        }

        // The sentence tells the author their document is fine. A
        // reader handed the emission framing here goes and files a
        // kernel bug against a body that just validated.
        let shown = NodeErrorKind::Naming(NamingError::SharedRim {
            face: cube.top,
            other: cube.bottom,
            found: RimShare::NotAdjacent,
        })
        .to_string();
        assert!(
            shown.contains(UNRULED_FRAMING),
            "missing-rule framing: {shown}"
        );
        assert!(
            !shown.contains(EMISSION_FRAMING),
            "a missing rule must not read as a kernel bug: {shown}"
        );
        assert!(
            !shown.contains(" { "),
            "a braced payload panics the Python binding at the arm meant \
             to refuse gracefully: {shown}"
        );
    }

    /// The seam-vertex refusal carries THIS vertex, not a constant, and
    /// opens with the missing-rule framing rather than the bug one.
    #[test]
    fn the_underdetermined_seam_vertex_reaches_the_node_level_prose() {
        let (a, b) = two_vertices();
        let carried =
            |vertex| NodeErrorKind::Naming(NamingError::SeamVertexParentage { vertex }).to_string();
        let (sa, sb) = (carried(a), carried(b));
        assert!(sa.contains(&format!("{a:?}")), "no locator: {sa}");
        assert_ne!(
            sa, sb,
            "a refusal that reads the same for two vertices has no locator"
        );
        assert!(sa.contains(UNRULED_FRAMING), "missing-rule framing: {sa}");
        assert!(
            !sa.contains(EMISSION_FRAMING),
            "a missing rule must not read as a kernel bug: {sa}"
        );
        for s in [&sa, &sb] {
            assert!(
                !s.contains(" { "),
                "a braced payload panics the Python binding at the arm \
                 meant to refuse gracefully: {s}"
            );
        }
    }

    /// The boundary the issue was actually about: wrapping the refusal
    /// in `NodeErrorKind` must not swallow it.
    #[test]
    fn the_eval_boundary_carries_the_diagnostic_through() {
        let shown = NodeErrorKind::Naming(NamingError::Emission {
            what: "split naming across boolean-minted faces",
        })
        .to_string();
        assert!(
            shown.contains("split naming across boolean-minted faces"),
            "the emitter's diagnostic must reach Display consumers: {shown}"
        );
    }
}
