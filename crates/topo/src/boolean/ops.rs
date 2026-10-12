//! The public boolean set operations (M3 PR 5): [`union`],
//! [`intersect`], [`subtract`] — functional (operands untouched),
//! composing reduce → classify (PR 4) → join → `setopfinish` → the
//! combine door → seam zip → the `merge_coplanar_faces` output stage
//! (F7) → the edge join (maximal edges) → the records' carriage → tier
//! gates. Every stage's refusal passes through typed as a
//! [`BooleanError`] variant.
//!
//! # Results (F8)
//!
//! ∅ is a typed SUCCESS ([`BooleanResult::Empty`]) — GQ2's per-node
//! result DAG wants a value, not an error. Real results carry a
//! [`BooleanResultKind`]:
//!
//! - [`Seamed`](BooleanResultKind::Seamed): boundaries intersected;
//!   the seam was zipped.
//! - [`OperandA`](BooleanResultKind::OperandA) /
//!   [`OperandB`](BooleanResultKind::OperandB): one operand's material
//!   is the whole answer (disjoint ∖, nested ∩, …).
//! - [`Assembly`](BooleanResultKind::Assembly): components of both
//!   operands without a seam, each piece a solid — the disjoint union
//!   (∪ of separated bodies), including assemblies touching at declared
//!   vertex and edge contacts (the carried
//!   [`ContactRecords`] say where; genuinely 3′, certified by PR 6's
//!   validator). A tangent line contact through a face's interior is
//!   a face–face touch the reduction does not yet record, so the union
//!   refuses it ([`BooleanError::TangentSlitArmUnbuilt`]).
//! - [`Voided`](BooleanResultKind::Voided): **legitimate voids** —
//!   A∖B with B strictly inside A yields the outer shell plus the
//!   reverted inner shell, a tier-2-legal multi-shell body. The
//!   insertion itself is [`super::voids::insert_void`] — the shared
//!   void-insertion door every cavity is born through (this fallback,
//!   the holed full revolve, and `shell`'s sealed hollow), with this
//!   fallback's probe verdicts as the door's containment evidence. A
//!   hollow B's cavities land as pieces inside the void B leaves.
//!
//! **Bodies in, bodies out** (`docs/DESIGN.md`, "A solid is one piece
//! of material"). An operand may hold any number of solids: the
//! pipeline reads each operand as one multi-shell solid
//! ([`Body::merge_all_solids`] on a clone), since it classifies, keeps and
//! grafts shells, and every result leaves [`boolean_op_with`] sorted
//! into pieces ([`crate::pieces`]) — one `Outer` per solid, each `Void`
//! under the piece whose material surrounds it.
//!
//! When operand boundaries do not intersect, classification falls back
//! to per-shell containment against the pristine other operand: the
//! uncut-shell witness ([`super::shell_witness`]) probes the shell's
//! points with [`super::solid_contain::point_in_solid`] (F8's ray design
//! promoted to 3-D) until one lies off the other boundary.
//!
//! # The merge output stage (F7)
//!
//! The seam zip manufactures coplanar same-surface-key face pairs by
//! construction (a cut face's fragments), so each op runs the merge
//! (`Body::merge_coplanar_faces_unjoined`, through [`finish_output`]) as
//! a documented final stage — part of the
//! op's contract, not hidden healing (the recipe records ONE boolean
//! node). The mergeable pairs are structural/declared by construction;
//! cross-operand *numeric* coplanarity is honestly left unmerged (the
//! coincidence ladder has no numeric rung).
//!
//! After the merge and its re-description, every output stage (the
//! seamed path, the graft and single-operand fallbacks, all through
//! [`finish_output`]) runs the edge join
//! ([`super::edge_join::join_stage`]): every joinable vertex is joined
//! away, so every result has maximal edges (`docs/DESIGN.md`, the
//! merge stage). Each join writes its substitution rows into the
//! op's descendant map before the records are carried.
//!
//! # Carried contacts
//!
//! Result bodies carry the contact records, discovered and carried, in
//! result keys, through one substitution door ([`carry`]): B-side keys
//! cross by the combine door's graft map, and a record whose cell an op
//! replaced names the replacement: a zip's survivor, the merge's kept
//! face, the edge a join made, or the chord the join minted along an
//! edge it then dropped. A record leaves only where its cell left the
//! result with no such row (a contact between A and B is only
//! meaningful in a result holding both sides), or where its two cells
//! became one or incident, which is structure ([`record`]).
//!
//! # Known limitations (PR 5.5 — the honest envelope)
//!
//! The seam lane's WORKING envelope (all exact-oracle-verified):
//! transversal boundary crossings; single-ring pockets/bosses on ALL
//! six face orientations (PR 5's R1 closed); double-ring single-face
//! seams (through-pillar tunnel, inset-leg union); multi-collinear-
//! site seams (R2) and crossing-polygon disconnections (R3),
//! including mixed collinear+transversal channel cuts (the PR 5.5
//! review's E-2, closed by the degenerate-segment fix in
//! `point_in_vertex_polygon`); interior-rest flush contacts (pillar standing on
//! a face); and the Fig 15.1 coplanar-overlap ∩ (seam partly on
//! shared cap planes) — the `join` module's derived sense/role
//! discipline is the consistency theorem behind all of them.
//!
//! Still refusing — typed, deterministic, operands untouched; never a
//! silent wrong body:
//!
//! - **Boundary-on-boundary seams**: declared unions of pure REST
//!   contacts (the full-overlap stacked union, corner-flush rests, the
//!   mated cross-lap, a shaft in a bore, a plate on a rounded plate)
//!   build through the join, which discards each contact side whole
//!   and fuses along the seam. What still refuses, typed: undeclared
//!   mates (the coincidence door, ladder rung (b)), and the join's own
//!   refusals. The ∩ and ∖ of a
//!   pure REST contact leave no null pair, so they take the
//!   no-crossings fallback, which keeps or drops whole shells; its
//!   certificates answer a verified `Rest` pair as a touch
//!   ([`Exempt::Rest`]).

use geom_core::interval::Interval;
use geom_core::{Band, Bounds, Decide, Indeterminate, Margin, Point3, Real, Sign, Tol, Vec3};

use std::collections::{BTreeMap, BTreeSet};

use super::BooleanDecision;
use super::SphereQuestion;
use super::boxes;
use super::combine::{GraftMap, graft_solids_disjoint};
use super::contain::{ContainError, FaceContainment, contfp};
use super::edge_join::join_stage;
use super::finish::setopfinish;
use super::join::bool_connect;
use super::reduce::PendingRow;
use super::section_cert::Refusal as SectionRefusal;
use super::shell_witness::{
    ShellVerdict, check_mutual, debug_assert_contacts_undecisive, kept_shells, shell_verdict,
};
use super::solid_contain::{SolidContainment, closed_sphere_group};
use super::voids;
use super::zip::{Fusions, Seam, SeamCorrespondence, zip_seam, zip_split};
use super::{
    BooleanError, BooleanInputs, BooleanOp, BooleanReduction, Cell, Contact, ContactRecords,
    EeContact, Glue, GluedPair, Operand, OperandRecords, ReductionContacts, SelfCheck,
    SweepStrategy, VeContact, VfContact, VvContact,
};
use crate::body::Body;
use crate::coincidence::{Backing, Cited, Cites};
use crate::entity::{EdgeKey, EntityId, FaceKey, LoopBoundary, ShellKey, VertexKey};
use crate::geometry::SurfaceKey;
use crate::live::{BoundaryMember, linked, proven};
use crate::merge_faces::{DescribeRefusal, DihedralReading, EdgeDescribeFailure};
use crate::props::AtRestPolicy;
use crate::props::QuadLane;
use crate::splitting::finish::{carve, single_solid};
use crate::validate::{
    AtRestBody, InBandShell, ValidationError, decide, scaffolds_at_rest, validate, validate_closed,
};
use geom_brep::recourse::Refused;
use geom_core::k_stats::NonzeroSign;

/// How a boolean result body came to be (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BooleanResultKind {
    /// Boundaries intersected; the seam was joined and zipped.
    Seamed,
    /// The result is operand A's material.
    OperandA,
    /// The result is operand B's material.
    OperandB,
    /// A multi-shell combination of components from both operands
    /// without a seam (disjoint, or touching at vertices and edges).
    Assembly,
    /// A∖B with B inside A: outer shell + reverted inner void shell.
    Voided,
}

/// A real (non-empty) boolean result.
///
/// # Validity-class carriage (D2 — the F1 contract)
///
/// The validity class rides THIS wrapper, never a mutable field on
/// [`Body`]: `body` is a finished body ([`AtRestBody`]), whose tier-3
/// verdict the door took on these bits ([`AtRestPolicy::gate_at_rest_kept`]:
/// [`AtRestOutcome::Validated`](crate::AtRestOutcome::Validated) at a
/// certifying scalar, [`AtRestOutcome::NotRunAtThisScalar`](crate::AtRestOutcome::NotRunAtThisScalar)
/// at a dual, where the door runs tiers 1 and 2 and the scaffold
/// fence in its place). `contacts` are the machine-checkable record of every
/// intentional touching the pipeline propagated (F2's explicit-intent
/// condition), and `body.validate_pseudomanifold(&contacts, tol)` is
/// the tier-3′ pass over them, empty or not. The door does not run
/// that census: a curved solid within reach of another solid is
/// beyond its cross-solid lane, which would refuse valid disjoint
/// unions (`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
#[derive(Debug)]
pub struct BooleanBody<T: Real> {
    /// The result body: one solid per piece of material
    /// ([`crate::pieces`]), gated at tier 3 by the door that built it —
    /// tier 3, not tier 3′: the census over [`Self::contacts`] is not run
    /// here (type-level docs).
    pub body: AtRestBody<T>,
    /// How it was produced.
    pub kind: BooleanResultKind,
    /// Contacts, declared or glued on a decided Zero, surviving into the
    /// result, in result keys
    /// (module docs) — what a tier-3′ pass over the result reads
    /// (type-level docs); the door has not checked them.
    pub contacts: ContactRecords,
    /// Naming emission (M4 PR 3, NAMING-DESIGN N4): the mint-time
    /// wiring facts the naming layer consumes — recorded as the
    /// pipeline runs, never reconstructed by post-hoc inspection.
    pub naming: BooleanNaming,
    /// The coincidences the op decided from values, in decision order
    /// and operand keys ([`crate::coincidence`]).
    pub coincidences: Vec<crate::Coincidence>,
}

/// How one operand's keys relate to the result body's keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OperandKeys {
    /// The result arena IS this operand's clone: operand keys resolve
    /// directly (surviving entities keep their keys).
    #[default]
    Direct,
    /// This operand was grafted in: its surviving keys appear as the
    /// source column of the corresponding `graft_*` rows.
    Grafted,
    /// No material of this operand is in the result.
    Absent,
}

/// Mint-time naming facts of one boolean result (M4 PR 3). Rows are
/// historical: a listed key may have been consumed by a later stage
/// (zip scaffolding, merge absorption, discarded material) — consumers
/// filter against the entities alive in `body` / chase the rows.
#[derive(Debug, Default)]
pub struct BooleanNaming {
    /// How operand A's keys map into the result.
    pub a_keys: OperandKeys,
    /// How operand B's keys map into the result.
    pub b_keys: OperandKeys,
    /// B-side graft lineage, `(B key, result key)` in arena slot
    /// order (empty unless `b_keys` is `Grafted`). Source keys are
    /// B-CLONE keys: operand keys for surviving operand entities plus
    /// reduction-minted keys.
    pub graft_vertices: Vec<(VertexKey, VertexKey)>,
    /// B-side edge graft lineage (see `graft_vertices`).
    pub graft_edges: Vec<(EdgeKey, EdgeKey)>,
    /// B-side edges the grafted body's records name but that died in
    /// B before the graft, `(B key, result key)` in B-key order: the
    /// result key is dead on arrival and stands for the B key in every
    /// forwarded record (a split's parent), so a lineage chased in the
    /// result reads back to the B ancestor through this row.
    pub graft_dead_edges: Vec<(EdgeKey, EdgeKey)>,
    /// B-side face graft lineage (see `graft_vertices`).
    pub graft_faces: Vec<(FaceKey, FaceKey)>,
    /// Seam edges surviving the zips, in zip/cycle order, result keys.
    pub seam_edges: Vec<EdgeKey>,
    /// Vertex fusions `(dead, kept)` in mint order, result keys: the
    /// zips'.
    pub vertex_merges: Fusions,
    /// Face absorption groups `(kept, absorbed…)`, result keys:
    /// `merge_coplanar_faces`'.
    pub merge_groups: Vec<(FaceKey, Vec<FaceKey>)>,
    /// The merge's lineage, `(dead, kept)` in group order, result keys:
    /// each cell a group killed (an absorbed face, a killed shared edge,
    /// a pruned vertex) with the kept face holding its interior
    /// ([`MergedGroup::substitutions`](crate::merge_faces::MergedGroup::substitutions)).
    pub merge_substitutions: Vec<(Cell, Cell)>,
    /// Curved merge groups the output stage did NOT glue, as outside
    /// the merge's Euler inventory (M4 PR 5), and declared surface pairs
    /// the door has no rung for (a non-planar carrier) — the record's
    /// faces plus the typed
    /// [`MergeCoplanarError`](crate::merge_faces::MergeCoplanarError)
    /// that stopped each, carried whole. WHICH groups are recorded
    /// here rather than refusing the whole call is the regime's own
    /// statement, at
    /// [`MergeCoplanarOutcome::skipped`](crate::merge_faces::MergeCoplanarOutcome::skipped),
    /// and is not restated here. The skip is visible HERE — a
    /// consumer can see what was not glued and why; the skipped
    /// faces' in-plane descriptions are re-checked against the
    /// actual adjacency before the result ships (review F1/F2).
    pub merge_skipped: Vec<crate::merge_faces::SkippedMerge>,
    /// A-side chord-mef fragment rows `(new face, divided-from face)`
    /// in mint order — A-clone keys, which ARE result keys when
    /// `a_keys` is `Direct`.
    pub face_fragments_a: Vec<(FaceKey, FaceKey)>,
    /// B-side chord-mef fragment rows, in B-CLONE keys (translate the
    /// new-face column through `graft_faces` for result keys).
    pub face_fragments_b: Vec<(FaceKey, FaceKey)>,
    /// The reduction's working contact set BEFORE result
    /// remapping ([`ReductionContacts`]): each row's A column in A-CLONE keys and its B column
    /// in B-CLONE keys — the result's keys on whichever side is
    /// `Direct`, the graft's source keys on a `Grafted` side, and keys
    /// of no body in the result on an `Absent` side. The mint-time
    /// crossing correspondences the
    /// naming layer reads even when one side's key was consumed
    /// (`BooleanBody::contacts` drops such rows by design). Its cells
    /// are the reading: each record's [`crate::Backing::Decided`]
    /// names a decision of the reduction's own, which the result
    /// records as a row where a record citing it survives or where it
    /// put a vertex the op left as copies on one point key (a pinch the
    /// op made, [`carry`]), so its citations index no list a reader
    /// holds.
    pub reduction_contacts: ReductionContacts,
    /// Every face an operand lost to the result, with the kept faces it
    /// bordered (`boolean::discard`, which says which paths record
    /// rows and why the others have none to record).
    pub discards: Vec<super::DiscardRow>,
    /// Each pair `(A face, B face)` of coincident faces where the result
    /// holds either face's region through the other: their materials lie
    /// on one side, so the classification keeps one operand's copy of the
    /// region they share and drops the other's (Eq. 15.3). The pair is
    /// one fact whichever copy is kept. Clone keys, as
    /// [`super::DiscardRow::face`]: chase `face_fragments_a`/`face_fragments_b`
    /// for the operand faces. Read off the classification, so every path
    /// that classifies records it — the section path and the
    /// containment fallback — sorted and deduplicated;
    /// a path that never classifies (disjoint boxes) has none.
    pub covered: Vec<(FaceKey, FaceKey)>,
    /// Every operand edge piece the classification read beside a vertex,
    /// with its side of the other operand
    /// ([`EdgePieceClass`](super::EdgePieceClass)), sorted: the vertex in
    /// clone keys, as `reduction_contacts`. Read off the classification,
    /// so a path that never classifies has none.
    pub edge_classes: Vec<super::EdgePieceClass>,
    /// The two vertices each of the classification's null edges joins,
    /// `(operand, below end, above end)` in clone keys as
    /// `edge_classes`: one point by construction, so a vertex and its
    /// copies are one place on the operand, and an operand edge ending at
    /// one ends at all of them. A path that never classifies has none.
    pub null_copies: Vec<(super::Operand, VertexKey, VertexKey)>,
    /// The output stage's joins in the order made, result keys: each
    /// row's vertex and `gone` edge are dead, and its `kept` edge holds
    /// their interiors (maximal edges, `docs/DESIGN.md`'s merge stage).
    pub edge_joins: Vec<super::EdgeJoin>,
}

impl BooleanNaming {
    /// Each result vertex a zip fused away → the live cell of `body`
    /// its point lies in ([`Self::settler`]).
    ///
    /// # Errors
    ///
    /// [`SettleError`], as [`Settler::settle`].
    pub fn fused_into<T: Real>(
        &self,
        body: &Body<T>,
    ) -> Result<BTreeMap<VertexKey, Cell>, SettleError> {
        let settler = self.settler(body);
        self.vertex_merges
            .rows()
            .iter()
            .map(|&(dead, _)| Ok((dead, settler.settle(dead)?)))
            .collect()
    }

    /// Settles result vertices on the live cells of `body` their points
    /// lie in, read off this naming's rows ([`Settler::settle`]).
    #[must_use]
    pub fn settler<'a, T: Real>(&'a self, body: &'a Body<T>) -> Settler<'a, T> {
        Settler {
            body,
            fusions: &self.vertex_merges,
            joins: &self.edge_joins,
            merged: self.merge_substitutions.iter().fold(
                BTreeMap::new(),
                |mut rows, &(dead, by)| {
                    rows.entry(dead)
                        .and_modify(|row| *row = None)
                        .or_insert(Some(by));
                    rows
                },
            ),
        }
    }

    /// The live edge `edge` is part of after the output stage's joins:
    /// itself, or the edge a join killed it into, followed through
    /// every later join.
    #[must_use]
    pub fn joined_edge(&self, edge: EdgeKey) -> EdgeKey {
        super::edge_join::joined_edge(&self.edge_joins, edge)
    }

    /// The live edge a stretch lies along when a join removed either
    /// end, its ends settled ([`Settler::settle`]): the edge holding the
    /// joined-away end ([`Cell::Edge`]), where the other end is one of its
    /// ends or is held by it too. An end inside a face lies on no edge.
    /// `None` where neither end was joined away, or the two do not lie
    /// along one edge.
    #[must_use]
    pub fn stretch_through_joins<T: Real>(
        &self,
        body: &Body<T>,
        (u, w): (Cell, Cell),
    ) -> Option<EdgeKey> {
        let on = |c: Cell, e: EdgeKey| match c {
            Cell::Vertex(v) => body.get_edge(e).is_some_and(|d| {
                [d.he_plus, d.he_minus]
                    .into_iter()
                    .any(|h| body.get_half_edge(h).is_some_and(|h| h.start == v))
            }),
            Cell::Edge(x) => x == e,
            Cell::Face(_) => false,
        };
        [u, w]
            .into_iter()
            .filter_map(|c| match c {
                Cell::Edge(e) => Some(e),
                Cell::Vertex(_) | Cell::Face(_) => None,
            })
            .find(|&e| on(u, e) && on(w, e))
    }
}

/// [`BooleanNaming::settler`]: result vertices settled on live cells.
#[derive(Debug)]
pub struct Settler<'a, T: Real> {
    body: &'a Body<T>,
    fusions: &'a Fusions,
    /// The output stage's joins, read by [`super::edge_join::joined_cell`].
    joins: &'a [super::EdgeJoin],
    /// The merge's lineage, each dead cell → its group's kept face;
    /// `None` where two rows name the cell, which the chase refuses
    /// ([`SettleError::DuplicateRow`]).
    merged: BTreeMap<Cell, Option<Cell>>,
}

impl<T: Real> Settler<'_, T> {
    /// The live cell `vertex`'s point lies in: the vertex it finally
    /// fused into, following the zips' fusions through every hop, while
    /// that lives. A later stage can delete it without moving its point,
    /// and both such stages record the kill: the merge's pruning leaves
    /// the point inside its group's kept face ([`Cell::Face`], the
    /// merge's own lineage), and the output stage's join inside the
    /// joined edge ([`Cell::Edge`], [`super::edge_join::joined_cell`]).
    ///
    /// # Errors
    ///
    /// [`SettleError::Dead`] where the chase ends on a cell `body` does
    /// not hold (a stage deleted it without its row), and
    /// [`SettleError::Cyclic`] where the rows cycle.
    pub fn settle(&self, vertex: VertexKey) -> Result<Cell, SettleError> {
        let start = Cell::Vertex(self.fusions.survivor(vertex));
        let twice = core::cell::Cell::new(None);
        let step = |c: Cell| match self.merged.get(&c) {
            Some(&Some(by)) => Some(by),
            Some(None) => {
                twice.set(Some(c));
                None
            }
            None => {
                let joined = super::edge_join::joined_cell(self.joins, c);
                (joined != c).then_some(joined)
            }
        };
        let chased = chase_live(self.body, start, self.merged.len() + self.joins.len(), step);
        if let Some(dead) = twice.get() {
            return Err(SettleError::DuplicateRow { vertex, dead });
        }
        match chased {
            Ok(Ok(cell)) => Ok(cell),
            Ok(Err(reached)) => Err(SettleError::Dead { vertex, reached }),
            Err(RowsCycle) => Err(SettleError::Cyclic { vertex }),
        }
    }
}

/// A result vertex the naming's rows cannot settle on a live cell
/// ([`Settler::settle`]): a corrupt record, which a reader must not
/// take for a stale key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettleError {
    /// The chase ended on a cell the result does not hold: a stage
    /// deleted it without appending its row.
    Dead {
        /// The vertex asked about.
        vertex: VertexKey,
        /// The dead cell the chase reached.
        reached: Cell,
    },
    /// The rows cycle.
    Cyclic {
        /// The vertex asked about.
        vertex: VertexKey,
    },
    /// The chase reached a cell two of the merge's rows name: which
    /// holds it is not recorded.
    DuplicateRow {
        /// The vertex asked about.
        vertex: VertexKey,
        /// The cell named twice.
        dead: Cell,
    },
}

impl core::fmt::Display for SettleError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let what = match self {
            Self::Dead { .. } => "settles on a cell the result does not hold",
            Self::Cyclic { .. } => "is settled through rows that cycle",
            Self::DuplicateRow { .. } => "is settled through a cell two rows name",
        };
        write!(f, "a vertex of a Boolean's result {what}")
    }
}

impl std::error::Error for SettleError {}

/// Whether `body` holds `cell`.
pub(super) fn holds<T: Real>(body: &Body<T>, cell: Cell) -> bool {
    match cell {
        Cell::Vertex(v) => body.get_vertex(v).is_some(),
        Cell::Edge(e) => body.get_edge(e).is_some(),
        Cell::Face(f) => body.get_face(f).is_some(),
    }
}

/// The rows a chase walked revisit a cell.
pub(super) struct RowsCycle;

/// The first cell `body` holds along `step` from `at`: `Ok(live)`, or
/// `Err(dead)` where `step` stops at a dead cell. A walk longer than
/// `rows` steps has revisited a cell, and refuses.
pub(super) fn chase_live<T: Real>(
    body: &Body<T>,
    mut at: Cell,
    rows: usize,
    step: impl Fn(Cell) -> Option<Cell>,
) -> Result<Result<Cell, Cell>, RowsCycle> {
    for _ in 0..=rows {
        if holds(body, at) {
            return Ok(Ok(at));
        }
        match step(at) {
            Some(next) => at = next,
            None => return Ok(Err(at)),
        }
    }
    Err(RowsCycle)
}

/// The typed result of a boolean op: a body, or the typed empty
/// success (F8: ∅ is a value, not an error).
// Size skew vs `Empty` is inherent (same posture as `SplitPart`).
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum BooleanResult<T: Real> {
    /// The regularized result is empty.
    Empty,
    /// A real result body.
    Body(BooleanBody<T>),
}

impl<T: Real> BooleanResult<T> {
    /// The result body, if non-empty.
    pub fn body(&self) -> Option<&BooleanBody<T>> {
        match self {
            Self::Body(b) => Some(b),
            Self::Empty => None,
        }
    }
}

/// A ∪* B (module docs; functional). Surface kinds are gated per arm,
/// not wholesale — see `reduce::gate_operand_pairs`.
///
/// # Errors
///
/// [`BooleanError`] — every stage's typed refusals pass through.
pub fn union<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(
        BooleanOp::Union,
        a,
        b,
        &BooleanInputs::default(),
        SweepStrategy::Realized,
        tol,
    )
}

/// A ∩* B (module docs).
///
/// # Errors
///
/// [`BooleanError`].
pub fn intersect<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(
        BooleanOp::Intersect,
        a,
        b,
        &BooleanInputs::default(),
        SweepStrategy::Realized,
        tol,
    )
}

/// A ∖* B (module docs).
///
/// # Errors
///
/// [`BooleanError`].
pub fn subtract<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        &BooleanInputs::default(),
        SweepStrategy::Realized,
        tol,
    )
}

/// The shared pipeline (module docs), with an explicit
/// [`SweepStrategy`] — the idealized/realized door (PERF-PLAN §4.4):
/// the tree only changes candidate GENERATION, so both strategies
/// produce bit-identical results; the differential suite runs full
/// ops through both and pins exactly that. Production wrappers pass
/// [`SweepStrategy::Realized`].
///
/// # Errors
///
/// [`BooleanError`] — identical to [`union`] and friends.
pub fn boolean_op_with<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
    inputs: &BooleanInputs,
    strategy: SweepStrategy,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    #[cfg(feature = "door-tier3-meter")]
    let door_from = std::time::Instant::now();
    let result = Glue::of(inputs).and_then(|glue| boolean_door(op, a, b, &glue, strategy, tol));
    #[cfg(feature = "door-tier3-meter")]
    super::door_meter::record(op, &result, door_from.elapsed(), [a.outcome(), b.outcome()]);
    result
}

/// [`boolean_op_with`]'s body.
fn boolean_door<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
    decls: &Glue,
    strategy: SweepStrategy,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    // The curved ∖/∩ front door, per class (C12.1 — retire per class,
    // never wholesale). Both ops route regions through `revert`
    // (A∖B ≡ A∩revert(B), the §15.9 posture), which is kind-generic:
    // `Face::sense` carries the flip on every carrier. The roster
    // (`reduce::revert_arm_exists`) is ∪'s (`boolean_arm_exists`) minus
    // `Nurbs`. A kind on it has a crossing layer whose every door is
    // certified or typed, and a no-crossings path that certifies its
    // reach or refuses — the sphere's extent by `sphere_extent_scan`
    // and its re-cut, then every pair with a curved face by
    // `section_extent_pass` — with `interior_loop_verdict` guarding the crossings path, so no
    // face pair of the kind is answered by a vertex probe that could
    // not see into it. `Nurbs` is on ∪'s roster for its plane×NURBS
    // germ arm, but has no edge×NURBS-face crossing layer (deviation 5),
    // so ∖ and ∩ have no seam lane for it.
    //
    // Up front and PAIR-SCOPED: the kinds are read exactly, and the
    // question of whether a kind can matter to this operation is
    // decided by boxes (`reduce::first_unsupported_pair` — non-overlap
    // is a certificate, overlap is a may). Operands untouched, no
    // reduction work before it.
    //
    // **No covered-pair rung here, and the asymmetry is the point.**
    // The operand gate admits a declared pair because a declaration
    // supplies the VERDICT a germ arm would have supplied. This roster
    // is not about verdicts: it names the kinds that have a seam lane
    // to revert through, and no declaration can supply one. A declared
    // pair of a kind off this roster is exactly as refused as an
    // undeclared one, and says so at the same site.
    if !matches!(op, BooleanOp::Union) {
        let band = Band::linear(tol)?;
        if let Some(p) = super::reduce::first_unsupported_pair(
            a,
            b,
            band,
            super::reduce::revert_arm_exists,
            |_, _, _| false,
        )? {
            return Err(BooleanError::CurvedPairUnsupported {
                op: Some(op),
                site: super::PairRefusalSite::RevertRoster,
                operand: p.operand,
                face: p.face,
                kind: p.kind,
                other_face: p.other_face,
                other_kind: p.other_kind,
            });
        }
    }
    let band = Band::linear(tol)?;
    for (operand, body) in [(Operand::A, a), (Operand::B, b)] {
        super::reduce::gate_unverdicted_operand(body, operand, band, tol)?;
    }
    let (a, b) = (one_solid(a)?, one_solid(b)?);
    boolean_op_recut(op, &a, &b, decls, strategy, true, tol)
}

/// `body` as the pipeline reads an operand: as is when it holds at most
/// one solid, else a clone with every shell under one solid (module
/// docs, "Bodies in, bodies out").
fn one_solid<T: Decide>(body: &Body<T>) -> Result<std::borrow::Cow<'_, Body<T>>, BooleanError> {
    if body.solids().nth(1).is_none() {
        return Ok(std::borrow::Cow::Borrowed(body));
    }
    let mut flat = body.clone();
    flat.merge_all_solids()?;
    Ok(std::borrow::Cow::Owned(flat))
}

/// The pipeline behind the front door, parameterized on whether the
/// no-crossings sphere RE-CUT (M5 S13) may still run: the re-entry
/// pass sets `recut = false`, so a re-cut that surfaces no crossings
/// is a loud invariant failure rather than a loop.
pub(super) fn boolean_op_recut<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    decls: &Glue,
    strategy: SweepStrategy,
    recut: bool,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    let band = Band::linear(tol)?;
    let (red, connected, interior_loops, glued) =
        match through_the_join(op, a, b, decls, strategy, recut, tol)? {
            Joined::Answered(result) => return Ok(*result),
            Joined::Connected {
                red,
                connected,
                interior_loops,
                decls,
            } => (*red, *connected, interior_loops, decls),
        };
    let decls = &*glued;
    let reduction_contacts = red.contacts.clone();
    let covered = red.covered.clone();
    let edge_classes = red.edge_classes.clone();
    let null_copies = super::null_copy_rows(&red.null_edges);
    let copies = Descendants::null_copies(&red.null_edges);
    let along = connected.along.clone();
    let ledger = ledger(&red, decls, band)?;
    let fin = setopfinish(op, red, &connected, a, b, band, tol)?;
    // The zip, the merge, the re-description and the closing mint are
    // one door's surgery (`crate::surgery`): the operators inside them
    // do not each re-derive the whole body, and `gate` below — tier 3
    // over the result, on every build — is what this door pays instead.
    // The guard owns the borrow, so a refusal on the way closes the
    // scope by dropping it.
    let mut finished = fin.body;
    let mut body = finished.begin_surgery();
    let mut seam_edges = Vec::new();
    // Each operand's face divisions: the join's chord-mef rows.
    let fragments = [connected.a_fragments.clone(), connected.b_fragments.clone()];
    let mut desc = Descendants::default()
        .with_copies(copies)
        .with_along(along)
        .with_fragments(fragments.clone());
    // A vertex several seams meet is fused by the first: each later zip
    // reads the correspondence through the fusions made.
    let mut vertex_map = fin.vertex_map.clone();
    let points = super::zip::point_classes(&body, &vertex_map)?;
    let (seams, cones) = super::zip::split_cones(&mut body, &fin.seams, &mut vertex_map, tol)?;
    for seam in &seams {
        let rep = match seam {
            Seam::Faces(a_face, b_face) => zip_seam(&mut body, *a_face, *b_face, &vertex_map, tol)?,
            Seam::Split {
                a_faces,
                b_faces,
                edges,
            } => zip_split(&mut body, (a_faces, b_faces), edges, tol)?,
        };
        desc.absorb_zip(&rep)?;
        seam_edges.extend(rep.seam_edges);
        vertex_map = fused_through(&vertex_map, &rep.vertex_merges);
    }
    super::zip::share_points(&mut body, &points)?;
    // A pinch keeps one vertex per cone, so lumps that met only there
    // share no edge: each connected piece of a shell becomes a shell.
    let shells: Vec<crate::entity::ShellKey> = body
        .solids()
        .flat_map(|(_, solid)| solid.shells.clone())
        .collect();
    for shell in shells {
        body.movefac(shell)?;
    }
    let vertex_merges = desc.vertex_merges();
    let declared_pairs = declared_surface_pairs(&body, a, b, decls, &fin.graft);
    let views = [&KeyView::Direct, &KeyView::Graft(&fin.graft)];
    let recorded = carried_vertices(&body, &ledger, views, &desc)?;
    let (merged, edge_joins) = finish_output(
        &mut body,
        &mut desc,
        &declared_pairs,
        &seam_edges,
        &recorded,
        band,
        tol,
    )?;
    let zipped = Zipped {
        operands: [a, b],
        _cones: cones,
    };
    let (contacts, coincidences) = carry(&body, &ledger, views, &desc, Some(zipped), band)?;
    share_recorded_points(&mut body, &contacts, &coincidences)?;
    // Curved results carry certified per-half-edge pcurves at rest
    // (M5 PR 9, the PR 6 contract): re-derive the whole cache set on
    // the finished body — the same pass the split lane runs. A planar
    // body mints nothing (no curved faces), so the M3 lane is
    // untouched bit-identically.
    crate::pcurves::mint_pcurves(&mut body, tol)
        .map_err(|source| BooleanError::Pcurves { source })?;
    body.sweep_and_close();
    let body = gate(finished, band, tol)?;
    interior_loops?;
    T::gate_volume_backstop(op, a, b, &body, band, tol)?;
    let (graft_vertices, graft_edges, graft_dead_edges, graft_faces) = graft_rows(&fin.graft);
    let [face_fragments_a, face_fragments_b] = fragments;
    let naming = BooleanNaming {
        a_keys: OperandKeys::Direct,
        b_keys: OperandKeys::Grafted,
        graft_vertices,
        graft_edges,
        graft_dead_edges,
        graft_faces,
        seam_edges,
        vertex_merges,
        merge_groups: merge_rows(&merged),
        merge_substitutions: merge_substitution_rows(&merged),
        merge_skipped: merged.skipped.clone(),
        face_fragments_a,
        face_fragments_b,
        reduction_contacts,
        discards: fin.discards,
        covered,
        edge_classes,
        null_copies,
        edge_joins,
    };
    Ok(BooleanResult::Body(BooleanBody {
        body,
        kind: BooleanResultKind::Seamed,
        contacts,
        naming,
        coincidences,
    }))
}

/// `map` with each vertex on either side read as the vertex a zip's
/// fusions `(dead, kept)` left in its place.
fn fused_through(map: &SeamCorrespondence, merges: &Fusions) -> SeamCorrespondence {
    let mut out = SeamCorrespondence::new();
    for (&a, bs) in map {
        out.entry(merges.survivor(a))
            .or_default()
            .extend(bs.iter().map(|&b| merges.survivor(b)));
    }
    out
}

/// What the pipeline reaches through its join ([`through_the_join`]).
pub(super) enum Joined<T: Real> {
    /// The pipeline's answer, reached without a join to finish: the
    /// no-crossings path (the re-cut or the containment fallback).
    Answered(Box<BooleanResult<T>>),
    /// The join, done: the reduction with both operands as it leaves
    /// them, every null edge killed, what it completed (never empty),
    /// and the interior-loop verdict, which the pipeline raises on the
    /// built body, before the volume backstop.
    Connected {
        /// The reduction, its operands joined.
        red: Box<BooleanReduction<T>>,
        /// The completed polygons and the fragments the join made.
        connected: Box<super::join::Connected>,
        /// [`interior_loop_verdict`]'s answer, not yet raised.
        interior_loops: Result<(), BooleanError>,
        /// The declarations the pipeline ran on: the caller's, and the
        /// pairs the glue door decided ([`super::glue`]).
        decls: Box<Glue>,
    },
}

/// **The pipeline through its join**: the glue door over the caller's
/// declarations `decls`, the reduction, then the no-crossings path where
/// there is no null pair, and otherwise the join.
/// [`boolean_op_recut`] finishes what it returns, and the test hook
/// that stops at the join (`boolean::through_the_join`) reads it, so
/// the two run one sequence. A re-cut re-enters with the caller's
/// declarations, and the glue door reads the re-cut operands afresh.
///
/// # Errors
///
/// The reduction's, the no-crossings path's and the join's refusals.
///
/// The reduction sweeps by `strategy`.
pub(super) fn through_the_join<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    decls: &Glue,
    strategy: SweepStrategy,
    recut: bool,
    tol: Tol,
) -> Result<Joined<T>, BooleanError> {
    let band = Band::linear(tol)?;
    let glued = super::glue::decided_declarations(a, b, decls, band)?;
    let mut red = super::boolean_reduce_declared_strategy(op, a, b, &glued, strategy, tol)?;

    if red.null_pairs.is_empty() {
        if !red.null_edges.is_empty() {
            return Err(BooleanError::ClassificationInvariant {
                what: "null edges without pairs reached the op",
            });
        }
        // M5 S13, the containment-fallback re-cut. Before any vertex
        // is probed, the curved-EXTENT scan certifies the sphere class
        // structurally: every closed sphere group's true extent
        // (center ± r) is consulted against every face of the other
        // operand — exact structure and certified enclosures, never a
        // sampled normal. Three outcomes:
        //
        // - **no escape**: the boundaries are certified disjoint
        //   (sphere-involved pairs), or meet only across a verified
        //   `Rest` pair, which touches without overlapping
        //   ([`Exempt::Rest`]), so the vertex-probe fallback's
        //   whole-shell answer is sound — proceed.
        // - **escape** (a sphere definitely leaves the other solid
        //   through a plane face — the S12 finding's
        //   poking-but-not-crossing shape — or two sphere faces cross
        //   in a circle no edge reaches): the operand is RE-CUT —
        //   a closed group whose escapes share one axis is rigidly
        //   re-charted onto it (a rotation about its own center: the
        //   same point set, seams now transverse to the section
        //   circles), any other escaping face is cut along its chart's
        //   meridian through each circle ([`SphereCutIn`]), and the
        //   pipeline re-enters once; the ordinary crossing layer then
        //   finds the section circles and the (Plane, Sphere) or
        //   (Sphere, Sphere) germ arm joins them exactly. Both spheres
        //   of a crossing pair are re-cut, so the circle lands as
        //   chords on both and neither keeps it as a ring.
        // - **uncertifiable** (NURBS re-gate, a trimmed group's circle
        //   the section certificate does not place inside one face,
        //   sphere faces meeting other than across a verified `Rest`,
        //   tangency, boundary-grazing circles,
        //   one group escaping through NON-PARALLEL faces): typed
        //   refusal — the S12 silence never re-opens.
        let recuts = sphere_extent_scan(a, b, &red.rest_contacts, band)?;
        if !recuts.rechart.is_empty() || !recuts.cut_in.is_empty() {
            if !recut {
                return Err(BooleanError::ClassificationInvariant {
                    what: "re-cut sphere operands still produced no crossings",
                });
            }
            let (mut a2, mut b2) = apply_recuts(a, b, &recuts.rechart, tol)?;
            // The cut-ins name faces of `a` and `b`; the re-charts carve
            // and graft other shells, and `carve` keeps every kept
            // entity's key (`splitting::finish`'s `carve`), so those
            // names still hold on `a2` and `b2`.
            apply_cut_ins(&mut a2, &mut b2, &recuts.cut_in, band, tol)?;
            return boolean_op_recut(op, &a2, &b2, decls, strategy, false, tol)
                .map(|result| Joined::Answered(Box::new(result)));
        }
        // The curved kinds the extent scan leaves: every torus,
        // cylinder and cone face's pairs, certified per pair by the
        // section certificate or refused typed.
        section_extent_pass(a, b, &red.rest_contacts, band)?;
        return fallback(op, &red, a, b, &glued, band, tol)
            .map(|result| Joined::Answered(Box::new(result)));
    }

    let interior_loops = interior_loop_verdict(op, a, b, &red, &glued, band);
    // The join carves both reduction operands through the Euler
    // operators; one scope per operand body, and what certifies the
    // result is `gate` below, over the body they are finished into.
    // The pair is guardless because the join takes the whole
    // reduction — `BooleanReduction::enter_join_surgery` carries the
    // argument — and `red` is a local of this pipeline, so a refusal
    // on the way drops it.
    red.enter_join_surgery();
    let connected = bool_connect(&mut red, a, b, band, tol);
    red.leave_join_surgery(connected.is_ok());
    let connected = connected?;
    if connected.completed.is_empty() {
        return Err(BooleanError::JoinDesync {
            what: "null pairs joined into no completed polygon",
        });
    }
    Ok(Joined::Connected {
        red: Box::new(red),
        connected: Box::new(connected),
        interior_loops,
        decls: Box::new(glued.into_owned()),
    })
}

/// **The crossings path's guard for the interior-loop class: the
/// section certificate, per pair** ([`super::section_cert`], whose
/// module docs carry the argument).
///
/// Every undeclared cross-operand pair whose certified boxes overlap,
/// where either face is not a plane, is classified and certified. A DECLARED pair is exempt: its contact is
/// the verified carrier the declared rungs walk along its edges. The
/// first refusing pair, in arena order (A's faces, then B's), refuses
/// the operation as the operand gate refuses a pair with no arm —
/// [`BooleanError::CurvedPairUnsupported`] with the op named — and its
/// `face` is the pair's A face when that face is curved, else its B
/// face.
///
/// Decided on the unmutated reduction and raised on the built body,
/// so every refusal the join and the build meet first stands verbatim.
/// It is raised after `gate`, whose tiers hold for a correct surgery
/// whatever the classification, and before the volume backstop, which
/// checks the classification this verdict has already declined to
/// certify: on a refused pair the verdict is the cause, and the
/// backstop's refusal on the same body would name a symptom.
fn interior_loop_verdict<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    red: &BooleanReduction<T>,
    decls: &Glue,
    band: Band,
) -> Result<(), BooleanError> {
    let events = event_pairs(red);
    let pairs = section_pairs(
        a,
        b,
        band,
        SectionPath::Crossings,
        Exempt::Glued(decls),
        |fa, fb| events.contains(&(fa, fb)),
        true,
    )?;
    match pairs.iter().find(|p| p.verdict.is_err()) {
        None => Ok(()),
        Some(p) => {
            let (operand, face, kind, other_face, other_kind) = p.named();
            Err(BooleanError::CurvedPairUnsupported {
                op: Some(op),
                site: super::PairRefusalSite::InteriorLoopGuard,
                operand,
                face,
                kind,
                other_face,
                other_kind,
            })
        }
    }
}

/// Does the glue hold EXACTLY the pair `(A face, B face)`? A glued
/// pair covers itself and no other: a face glued to one partner meets
/// every other partner as any face does.
pub(crate) fn glues_pair(decls: &Glue, fa: FaceKey, fb: FaceKey) -> bool {
    decls.pairs.iter().any(|d| d.a == fa && d.b == fb)
}

/// **`chart_boundary`'s verdict per face, asked once.** Keyed by the
/// operand as well as the face: the two operands' arenas mint their
/// keys independently, so A's face and B's face can share a key. (The
/// map keys the operand as "is B", `Operand` carrying no order.)
#[derive(Default)]
pub(crate) struct ChartCache(BTreeMap<(bool, FaceKey), bool>);

impl ChartCache {
    /// Does `operand`'s `face` of `body` describe — does every closed
    /// curve in its interior lift to a closed curve with zero winding
    /// (W2's premise)? Yes when [`crate::pcurves::chart_boundary`]
    /// answers `Ok`, and on a cone face also when the apex closure
    /// closes ([`crate::chord_join::cone_apex_closure`]: one apex
    /// visit, no ring) on a window at most a period wide.
    ///
    /// The cone clause is sound although `chart_boundary` refuses the
    /// apex as a singular joint. The apex lies on `∂F`, so `int F`
    /// excludes it, and `int F` maps homeomorphically onto the interior
    /// of the lifted region: a bounded region of one sheet of the
    /// punctured nappe's universal cover, at most a period wide. Every
    /// closed curve in `int F` therefore lifts to a closed curve with
    /// zero winding, and no essential component lies in `int F`.
    pub(crate) fn describes<T: crate::props::AtRestPolicy>(
        &mut self,
        operand: Operand,
        body: &Body<T>,
        face: FaceKey,
        surface: &geom::Surface<T>,
        band: Band,
    ) -> bool {
        *self
            .0
            .entry((operand == Operand::B, face))
            .or_insert_with(|| {
                crate::pcurves::chart_boundary(body, face, surface, band).is_ok()
                    || apex_closure_describes(body, face, surface, band)
            })
    }
}

/// The cone clause of [`ChartCache::describes`]: the apex closure
/// closes, on a window not definitely wider than a period.
fn apex_closure_describes<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    surface: &geom::Surface<T>,
    band: Band,
) -> bool {
    use crate::chord_join::ApexClosure;
    if !matches!(surface, geom::Surface::Cone { .. }) {
        return false;
    }
    let Ok(ApexClosure::Closed { window, reach, .. }) =
        crate::chord_join::cone_apex_closure(body, surface, face, band)
    else {
        return false;
    };
    matches!(
        crate::validate::decide(
            "bool_cone_closure_period",
            Margin::levered(T::tau() - (window.1 - window.0), reach),
            band,
        ),
        Ok(Sign::Positive | Sign::Zero)
    )
}

/// Which path a section scan serves; the two differ in which pairs
/// they examine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SectionPath {
    /// The crossings path: every pair with a face that is not a plane.
    Crossings,
    /// The no-crossings fallback: every pair with a face that is not a
    /// plane, except a sphere against a plane, a sphere or a spline —
    /// those are the extent scan's ([`sphere_extent_scan`]), which runs
    /// first and keeps its re-cut. A sphere against a cylinder, a torus
    /// or a cone is certified here: none is ever an escape face, so the
    /// scan's only question of the pair is disjointness, and with no
    /// event anywhere "every component cleared" is disjointness.
    Fallback,
}

impl SectionPath {
    fn scope<T: Real>(self, x: &geom::Surface<T>, y: &geom::Surface<T>) -> bool {
        use geom::Surface as S;
        let curved = |s: &geom::Surface<T>| !matches!(s, S::Plane { .. });
        let sphere = |s: &geom::Surface<T>| matches!(s, S::Sphere { .. });
        let scanned =
            (sphere(x) && !section_pass_takes(y)) || (sphere(y) && !section_pass_takes(x));
        (curved(x) || curved(y)) && !(self == Self::Fallback && scanned)
    }

    /// Is `s` a face this path's refusal names?
    fn names<T: Real>(self, s: &geom::Surface<T>) -> bool {
        self.scope(s, s)
    }
}

/// **The kinds whose pairs with a sphere the section pass certifies on
/// the no-crossings path**, rather than the extent scan
/// ([`sphere_extent_scan`]): none is ever an escape face, so the scan's
/// only question of the pair is disjointness, which the pass answers per
/// face pair.
fn section_pass_takes<T: Real>(s: &geom::Surface<T>) -> bool {
    use geom::Surface as S;
    match s {
        S::Cylinder { .. } | S::Torus { .. } | S::Cone { .. } => true,
        S::Plane { .. } | S::Sphere { .. } | S::Nurbs(_) | S::Approx(_) => false,
    }
}

/// One pair's certificate outcome.
#[derive(Clone, Debug)]
pub(crate) struct PairVerdict {
    /// The pair's face of A.
    pub a_face: FaceKey,
    /// Its kind.
    pub a_kind: geom::SurfaceKind,
    /// The pair's face of B.
    pub b_face: FaceKey,
    /// Its kind.
    pub b_kind: geom::SurfaceKind,
    /// A's face is one the path names in its refusal.
    pub a_named: bool,
    /// Each component's witness, or the pair's refusal.
    pub verdict: Result<Vec<super::section_cert::Cleared>, super::section_cert::Refusal>,
}

impl PairVerdict {
    /// `(operand, face, kind, other_face, other_kind)`, the named face
    /// first.
    fn named(
        &self,
    ) -> (
        Operand,
        FaceKey,
        geom::SurfaceKind,
        FaceKey,
        geom::SurfaceKind,
    ) {
        if self.a_named {
            (
                Operand::A,
                self.a_face,
                self.a_kind,
                self.b_face,
                self.b_kind,
            )
        } else {
            (
                Operand::B,
                self.b_face,
                self.b_kind,
                self.a_face,
                self.a_kind,
            )
        }
    }
}

/// Does the face, which the caller read out of `body`, carry a
/// lone-vertex loop?
#[track_caller]
fn has_lone_vertex<T: Real>(body: &Body<T>, face: FaceKey) -> bool {
    let fd = proven(&body.faces, face, EntityId::Face);
    body.face_loops_linked(face, fd)
        .any(|(_, l)| matches!(l.boundary, LoopBoundary::Empty { .. }))
}

/// Places a witness point in one face ([`super::contain::face_placement`]).
/// A point definitely OFF the carrier is no verdict: the witness was
/// built on the carrier, so that answer contradicts its construction
/// rather than placing it, and so does a margin from a plane in band.
pub(crate) fn place_witness<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    p: Point3<T>,
    band: Band,
) -> Option<FaceContainment> {
    use super::contain::{CurvedPlacement, PlacementError};
    match super::contain::face_placement(body, face, p, band, "bool_section_witness_on_plane") {
        Ok(CurvedPlacement::Trim(at)) => at,
        Ok(CurvedPlacement::OffCarrier) | Err(PlacementError::OnPlane(_)) => None,
        Err(PlacementError::Door(ContainError::StaleFace(face))) => {
            super::contain::driver_face_stale(face)
        }
        Err(PlacementError::Door(_)) => None,
    }
}

/// One face as the section certificate reads it: its key, its surface's
/// key and description, and its certified box.
pub(crate) struct FaceRow<T: Real> {
    face: FaceKey,
    key: SurfaceKey,
    surface: geom::Surface<T>,
    bbox: bvh::Aabb,
}

/// Every face of `body` as a [`FaceRow`], in face-arena order.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for a face whose surface
/// does not resolve, and the box builder's own errors.
pub(crate) fn face_rows<T: Decide + Bounds>(
    body: &Body<T>,
    band: Band,
) -> Result<Vec<FaceRow<T>>, BooleanError> {
    let pad = boxes::sweep_pad(band);
    body.faces()
        .map(|(face, fd)| {
            let surface = body
                .get_surface(fd.surface)
                .ok_or(BooleanError::ClassificationInvariant {
                    what: "section certificate: a face surface is lost",
                })?
                .clone();
            Ok(FaceRow {
                face,
                key: fd.surface,
                surface,
                bbox: boxes::face_box(body, face, pad, band)?,
            })
        })
        .collect()
}

/// The section certificate's [`Reach`](super::section_cert::Reach) of
/// two overlapping face boxes, which are `f64` hulls the face walk
/// computed: the ball enters as computed values.
fn overlap_reach<T: Real>(box_a: &bvh::Aabb, box_b: &bvh::Aabb) -> super::section_cert::Reach<T> {
    let (lo, hi) = (
        Vec3::new(
            box_a.min_x.max(box_b.min_x),
            box_a.min_y.max(box_b.min_y),
            box_a.min_z.max(box_b.min_z),
        ),
        Vec3::new(
            box_a.max_x.min(box_b.max_x),
            box_a.max_y.min(box_b.max_y),
            box_a.max_z.min(box_b.max_z),
        ),
    );
    super::section_cert::Reach {
        centre: Point3::origin() + ((lo + hi) * 0.5).map(T::from_computed),
        radius: T::from_computed((hi - lo).norm() * 0.5),
    }
}

/// **The section certificate's per-pair rule** for A's face `fa` (the
/// section's `F`) against B's face `fb` (its `G`), whose boxes overlap:
/// the pair's classified section and every component's witness, or the
/// pair's refusal. The reach — the ball about the two boxes' overlap,
/// which pivots and levers the angular margins — is built here
/// ([`super::section_cert`]'s module docs).
fn pair_verdict<T: Decide + Bounds + crate::props::AtRestPolicy>(
    (a, fa): (&Body<T>, &FaceRow<T>),
    (b, fb): (&Body<T>, &FaceRow<T>),
    band: Band,
    evented: bool,
    charts: &mut ChartCache,
) -> Result<Vec<super::section_cert::Cleared>, super::section_cert::Refusal> {
    use super::section_cert::{Refusal, Side, certify, classify};
    if has_lone_vertex(a, fa.face) || has_lone_vertex(b, fb.face) {
        return Err(Refusal::LoneVertex);
    }
    let reach = overlap_reach(&fa.bbox, &fb.bbox);
    let section = classify(&fa.surface, &fb.surface, reach, band);
    certify(
        &section,
        evented,
        |side| {
            let (operand, body, row) = match side {
                Side::F => (Operand::A, a, fa),
                Side::G => (Operand::B, b, fb),
            };
            charts.describes(operand, body, row.face, &row.surface, band)
        },
        |p| {
            [
                place_witness(a, fa.face, p, band),
                place_witness(b, fb.face, p, band),
            ]
        },
    )
}

/// **Whether a boundary edge of `face` may meet `circle`**: some edge of
/// one of its loops has a certified box, padded by `pad`, overlapping
/// `region` (the circle's box), and the two are not apart along a
/// direction that turns with them ([`super::separating::apart`]).
/// `face` is one the caller read out of `body`.
///
/// # Panics
///
/// Where a boundary hop past `face` (a loop, a member's edge, a lone vertex's
/// point) does not resolve, or a loop walk does not close
/// ([`crate::live::NAMES_ONLY_LIVE`] / [`crate::body::WALKS_CLOSE`];
/// [`crate::live::OPERATORS_KEEP_LINKS`]).
#[track_caller]
fn face_boundary_meets<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    (circle, region): (super::separating::Item<T>, &bvh::Aabb),
    axes: &[geom_core::UnitVec3<T>],
    pad: f64,
    band: Band,
) -> bool {
    let fd = proven(&body.faces, face, EntityId::Face);
    for member in body.face_boundary_linked(face, fd) {
        let BoundaryMember::Edge { ek, .. } = member else {
            continue;
        };
        if boxes::edge_box(body, ek, pad).overlaps(region)
            && !super::separating::apart(
                (body, super::separating::Item::Edge(ek)),
                (body, circle),
                axes,
                pad,
                band,
            )
        {
            return true;
        }
    }
    false
}

/// **The section certificate over pairs of rows**: every `(A row, B
/// row)` pair whose certified boxes overlap, which the narrow phase
/// ([`super::separating::apart`]) does not part and which `admit`
/// takes, in the rows' order, each [`pair_verdict`]'s. `evented` says whether
/// the reduction recorded an event on the pair `(A face, B face)`;
/// `named` says whether A's face is the one a refusal names. With
/// `stop` the walk returns at the first refusing pair.
#[allow(clippy::too_many_arguments)]
fn walk_pairs<'r, T: Decide + Bounds + crate::props::AtRestPolicy + 'r>(
    (a, a_rows): (&Body<T>, impl IntoIterator<Item = &'r FaceRow<T>>),
    (b, b_rows): (&Body<T>, impl IntoIterator<Item = &'r FaceRow<T>>),
    band: Band,
    charts: &mut ChartCache,
    admit: impl Fn(&FaceRow<T>, &FaceRow<T>) -> bool,
    evented: impl Fn(FaceKey, FaceKey) -> bool,
    named: impl Fn(&geom::Surface<T>) -> bool,
    stop: bool,
    axes: &super::separating::OperandAxes<T>,
) -> Vec<PairVerdict> {
    let b_rows: Vec<&FaceRow<T>> = b_rows.into_iter().collect();
    let pad = boxes::sweep_pad(band);
    let mut out = Vec::new();
    for fa in a_rows {
        for &fb in &b_rows {
            if !fa.bbox.overlaps(&fb.bbox) || !admit(fa, fb) {
                continue;
            }
            // The overlap is a world-axis one; a pair apart along a
            // direction that turns with the operands has no section.
            if super::separating::apart(
                (a, super::separating::Item::Face(fa.face)),
                (b, super::separating::Item::Face(fb.face)),
                axes.of(a, b, band),
                pad,
                band,
            ) {
                continue;
            }
            let verdict = pair_verdict((a, fa), (b, fb), band, evented(fa.face, fb.face), charts);
            let refused = verdict.is_err();
            out.push(PairVerdict {
                a_face: fa.face,
                a_kind: fa.surface.kind(),
                b_face: fb.face,
                b_kind: fb.surface.kind(),
                a_named: named(&fa.surface),
                verdict,
            });
            if stop && refused {
                return out;
            }
        }
    }
    out
}

/// **The section certificate over every in-scope pair** of `a` × `b`
/// whose certified boxes overlap, which the narrow phase does not part
/// and which `exempt` does not answer, in arena order ([`walk_pairs`]). `evented` says whether the reduction
/// recorded an event on the pair `(A face, B face)`. With `stop` the
/// scan returns at the first refusing pair. `chart_boundary` is asked
/// once per face and cached.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for a face whose surface
/// or loops do not resolve, and the box builder's own errors.
pub(crate) fn section_pairs<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    band: Band,
    path: SectionPath,
    exempt: Exempt<'_>,
    evented: impl Fn(FaceKey, FaceKey) -> bool,
    stop: bool,
) -> Result<Vec<PairVerdict>, BooleanError> {
    let (a_rows, b_rows) = (face_rows(a, band)?, face_rows(b, band)?);
    Ok(walk_pairs(
        (a, &a_rows),
        (b, &b_rows),
        band,
        &mut ChartCache::default(),
        |fa, fb| path.scope(&fa.surface, &fb.surface) && !exempt.answers(fa.face, fb.face),
        evented,
        |s| path.names(s),
        stop,
        &super::separating::OperandAxes::new(boxes::axis_key),
    ))
}

/// **The pairs a section walk answers without their section**, one per
/// path, each keyed to its exact `(A face, B face)` pair: a face
/// exempted against one partner meets every other partner as any face
/// does.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Exempt<'r> {
    /// Every in-scope pair is classified: the test-support and
    /// sweep-testing doors' spelling, which no production path takes.
    #[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
    Nothing,
    /// The crossings path: a GLUED pair, whose contact is the carrier
    /// the glued rungs walk along its edges.
    Glued(&'r Glue),
    /// The no-crossings path: a `Rest` declaration the declaration door
    /// verified one carrier with opposed senses (the reduction's
    /// `rest_contacts`). Its section lies on the one carrier, where
    /// each operand's material stands on its own side, so every point
    /// the two faces share is a touch of the two boundaries and none is
    /// a point of both interiors: the pair hides no overlap from the
    /// vertex probe, whatever its section's component count. A
    /// continuation puts both materials on one side and is not such a
    /// pair.
    Rest(&'r [(FaceKey, FaceKey)]),
}

impl Exempt<'_> {
    /// Does this exemption answer the pair `(A face, B face)`?
    pub(crate) fn answers(self, fa: FaceKey, fb: FaceKey) -> bool {
        match self {
            #[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
            Self::Nothing => false,
            Self::Glued(decls) => glues_pair(decls, fa, fb),
            Self::Rest(pairs) => pairs.contains(&(fa, fb)),
        }
    }
}

/// **The no-crossings path's section pass.** With no event anywhere,
/// W4 never fires and the no-event decision decides every lone
/// component, so each pair the vertex probe could not see into is
/// either certified or refused here, typed as the fallback's extent
/// refusal ([`BooleanError::FallbackExtentUnsupported`]) naming the
/// pair's curved face. A verified `Rest` pair (`rest`, [`Exempt::Rest`])
/// is answered without its section.
fn section_extent_pass<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    rest: &[(FaceKey, FaceKey)],
    band: Band,
) -> Result<(), BooleanError> {
    let pairs = section_pairs(
        a,
        b,
        band,
        SectionPath::Fallback,
        Exempt::Rest(rest),
        |_, _| false,
        true,
    )?;
    match pairs
        .iter()
        .find_map(|p| p.verdict.as_ref().err().map(|r| (p, *r)))
    {
        None => Ok(()),
        Some((p, refusal)) => {
            let (operand, face, ..) = p.named();
            Err(BooleanError::FallbackExtentUnsupported {
                operand,
                face,
                what: refusal.what(),
            })
        }
    }
}

/// **The section certificate's per-pair report, for the verdict rows:**
/// the reduction `op` would run, and every in-scope pair's outcome on
/// the crossings path with that reduction's events.
///
/// # Errors
///
/// The reduction's own refusals, and [`section_pairs`]'.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn section_report<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    tol: Tol,
) -> Result<Vec<PairVerdict>, BooleanError> {
    let band = Band::linear(tol)?;
    let decls = Glue::default();
    let red =
        super::boolean_reduce_declared_strategy(op, a, b, &decls, SweepStrategy::Realized, tol)?;
    let events = event_pairs(&red);
    section_pairs(
        a,
        b,
        band,
        SectionPath::Crossings,
        Exempt::Nothing,
        |fa, fb| events.contains(&(fa, fb)),
        false,
    )
}

/// **The section pass's per-pair report on the no-crossings path**:
/// every pair the pass examines, with no event anywhere, whatever the
/// crossing layer would find. The pass itself stops at the first
/// refusal; this reads every pair.
///
/// # Errors
///
/// [`section_pairs`]'.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn no_crossings_section_report(
    a: &Body<f64>,
    b: &Body<f64>,
    tol: Tol,
) -> Result<Vec<PairVerdict>, BooleanError> {
    section_pairs(
        a,
        b,
        Band::linear(tol)?,
        SectionPath::Fallback,
        Exempt::Nothing,
        |_, _| false,
        false,
    )
}

/// **The no-crossings path's two certificates**, run as the path runs
/// them before the vertex probe on undeclared operands: the sphere
/// extent scan, then — when it asks for no re-cut — the section pass.
/// With no declaration there is no verified `Rest` pair, so both run
/// with the empty list the op hands them. `Ok` with the number of
/// re-cuts the scan asked for.
///
/// # Errors
///
/// Either certificate's refusal.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn no_crossings_certificates(
    a: &Body<f64>,
    b: &Body<f64>,
    tol: Tol,
) -> Result<usize, BooleanError> {
    let band = Band::linear(tol)?;
    let no_rest: &[(FaceKey, FaceKey)] = &[];
    let recuts = sphere_extent_scan(a, b, no_rest, band)?;
    let asked = recuts.rechart.len() + recuts.cut_in.len();
    if asked == 0 {
        section_extent_pass(a, b, no_rest, band)?;
    }
    Ok(asked)
}

/// **A ball against a plane's CARRIER: the one home of that gap.**
/// Decides `r − |s|` under `bool_sphere_extent_gap`, where `s` is the
/// centre's signed distance to the plane along its stored normal, which
/// the plane convention makes unit (and which is read as stored rather
/// than re-normalised, because at the interval scalar a division by the
/// normal's own norm widens `s` and moves the scan's escalations):
/// `Negative` is a ball definitely clear of the carrier, `Zero` a
/// tangency, `Positive` a ball the carrier cuts in a circle of radius
/// `√((r − |s|)(r + |s|))` about `centre − n̂·s`. Returns the sign and
/// `s`. Read by the no-crossings sphere scan ([`sphere_extent_scan`]).
fn ball_against_plane<T: Decide>(
    center: Point3<T>,
    radius: T,
    origin: Point3<T>,
    normal: Vec3<T>,
    band: Band,
) -> Result<(NonzeroSign, T), geom_core::Indeterminate> {
    let s = (center - origin).dot(normal);
    let sign = crate::validate::decide_nonzero(
        "bool_sphere_extent_gap",
        Margin::of(radius - s.abs()),
        band,
    )?;
    Ok((sign, s))
}

/// The (A face, B face) pairs the reduction recorded an event on: a
/// vertex-on-face contact pairs every face the vertex bounds with the
/// face it lies on, and a vertex-on-vertex contact pairs every face
/// each vertex bounds.
///
/// "The faces a vertex bounds" are read at the contact point, not at
/// one key: the classification's null edges split a contact vertex into
/// copies at that point, and which copy keeps which face's corner is
/// the scaffolding's choice (the side each null edge's new vertex
/// took), so every copy the vertex's null edges reach, transitively (a
/// strut nested in another's segment hangs at its tip), is read with it.
fn event_pairs<T: Real>(red: &BooleanReduction<T>) -> BTreeSet<(FaceKey, FaceKey)> {
    contact_face_pairs(&red.a, &red.b, &red.contacts, &red.null_edges)
}

/// [`event_pairs`] over its parts: the split operands, the sweep's
/// contacts, and the classification's null edges (none before it runs).
pub(crate) fn contact_face_pairs<T: Real>(
    a: &Body<T>,
    b: &Body<T>,
    contacts: &ReductionContacts,
    null_edges: &[super::BoolNullEdgeRecord<T>],
) -> BTreeSet<(FaceKey, FaceKey)> {
    let a_faces = faces_by_vertex(a);
    let b_faces = faces_by_vertex(b);
    let desc = Descendants::default().with_copies(Descendants::null_copies(null_edges));
    let around = |operand: Operand, v: VertexKey| {
        let m = match operand {
            Operand::A => &a_faces,
            Operand::B => &b_faces,
        };
        let mut faces: Vec<FaceKey> = desc
            .copies_of(operand, v)
            .into_iter()
            .flat_map(|u| m.get(&u).cloned().unwrap_or_default())
            .collect();
        faces.sort();
        faces.dedup();
        faces
    };
    let mut out = BTreeSet::new();
    for c in &contacts.a_on_b {
        for fa in around(Operand::A, c.vertex) {
            out.insert((fa, c.face));
        }
    }
    for c in &contacts.b_on_a {
        for fb in around(Operand::B, c.vertex) {
            out.insert((c.face, fb));
        }
    }
    for c in &contacts.vv {
        for fa in around(Operand::A, c.a) {
            for fb in around(Operand::B, c.b) {
                out.insert((fa, fb));
            }
        }
    }
    out
}

/// Every face each vertex bounds, from the faces' own loops.
///
/// # Panics
///
/// Where a face's boundary hop (a loop, a member's edge, a lone vertex's
/// point) does not resolve, or a loop walk does not close
/// ([`crate::live::NAMES_ONLY_LIVE`] / [`crate::body::WALKS_CLOSE`];
/// [`crate::live::OPERATORS_KEEP_LINKS`]).
fn faces_by_vertex<T: Real>(body: &Body<T>) -> BTreeMap<VertexKey, Vec<FaceKey>> {
    let mut out: BTreeMap<VertexKey, Vec<FaceKey>> = BTreeMap::new();
    for (face, fd) in body.faces() {
        for member in body.face_boundary_linked(face, fd) {
            // A lone-vertex loop's vertex bounds the face as much as a
            // cycle's do.
            let v = match member {
                BoundaryMember::Isolated { vertex, .. } => vertex,
                BoundaryMember::Edge { half, .. } => half.start,
            };
            let faces = out.entry(v).or_default();
            if !faces.contains(&face) {
                faces.push(face);
            }
        }
    }
    out
}

/// The graft map as sorted-order row vectors (naming emission).
type GraftRows = (
    Vec<(VertexKey, VertexKey)>,
    Vec<(EdgeKey, EdgeKey)>,
    Vec<(EdgeKey, EdgeKey)>,
    Vec<(FaceKey, FaceKey)>,
);

pub(super) fn graft_rows(g: &GraftMap) -> GraftRows {
    (
        g.vertices.iter().map(|(k, &v)| (k, v)).collect(),
        g.edges.iter().map(|(k, &v)| (k, v)).collect(),
        g.dead_edges.iter().map(|(&k, &v)| (k, v)).collect(),
        g.faces.iter().map(|(k, &v)| (k, v)).collect(),
    )
}

/// The merge outcome as naming rows.
pub(super) fn merge_rows(
    m: &crate::merge_faces::MergeCoplanarOutcome,
) -> Vec<(FaceKey, Vec<FaceKey>)> {
    m.groups
        .iter()
        .map(|g| (g.kept, g.absorbed.clone()))
        .collect()
}

/// The merge's lineage as naming rows ([`MergedGroup::substitutions`](crate::merge_faces::MergedGroup::substitutions)).
pub(super) fn merge_substitution_rows(
    m: &crate::merge_faces::MergeCoplanarOutcome,
) -> Vec<(Cell, Cell)> {
    m.groups
        .iter()
        .flat_map(crate::merge_faces::MergedGroup::substitutions)
        .collect()
}

/// The volume-inequality backstop at the op gate (PR 5 review): every
/// `Seamed` result must satisfy the set-theoretic bounds
/// vol(∩) ≤ min(vol A, vol B), max(vol A, vol B) ≤ vol(∪) ≤ vol A + vol B,
/// vol A − vol B ≤ vol(∖) ≤ vol A; and a bounded result must not
/// enclose negative volume, tier 3's +V invariant read by its own rule
/// (`crate::validate::plus_v_read`, in-band exempt). The
/// min/max are decomposed into per-operand inequalities. No inequality over these three volumes bounds vol(∩)
/// from below — that needs vol(∪), which the op does not compute — so a
/// short positive intersect is invisible here.
///
/// # What it measures with
///
/// The three bodies are measured through `lane`, the certified
/// quadrature, by the sign-level walk ([`crate::props::sign_walk`]) run
/// to its reporting target: a face trimmed by an ellipse or a spline is
/// enclosed by the quadrature, every other face takes its closed form,
/// and the enclosures are the ones [`crate::mass_properties`] reports.
/// The scalar's policy decides whether the gate runs at all
/// ([`crate::AtRestPolicy::gate_volume_backstop`]): a dual runs nothing
/// (DL3). A body the property layer cannot measure refuses
/// [`BooleanError::VolumeUnmeasured`], and one whose structure does not
/// resolve [`BooleanError::VolumeCorrupt`] — tier 3's reading of the
/// same refusals (`validate::classify_mass_props`). A bound that cannot
/// be evaluated is not a pass.
///
/// # Enclosures, and the resolution they leave
///
/// A quadrature volume is an enclosure `[volume_lo, volume_hi]`
/// ([`crate::MassProperties::enclosure`]), so each decision reads the
/// end that makes it conservative: an operand is bounded only when its
/// lower end is certified positive, and a bound is violated only when
/// the margin's UPPER end — the two enclosures at their least
/// favourable — is certified negative. Where the upper end is not
/// negative but the lower one is, the sign is open because of the
/// enclosures and not the bodies, and the gate REFINES before it
/// decides: both compared bodies' quadrature faces are taken one round
/// further at a time from the round each met its target at
/// ([`crate::props::PastTarget::refine`]), until the
/// sign is decided or every face reaches the last round every lane runs
/// (`geom_brep::props::quad::LAST_ROUND_EVERY_LANE_RUNS`).
///
/// There the rounds run out at a resolution: the last round's
/// half-width, measured on the oblique-capped rod (`r = 0.5`, cut 20° at
/// `z = 3.5`, `sweep/tests/reach_volume_backstop.rs`) as ≈ 2.2e-10 m³
/// of rule remainder whatever ε plus ≈ 0.55·ε m³ (7.7e-10 m³ at the
/// default ε, against a reporting half-width of 9.2e-7 m³), the
/// remainder growing with the body's size. What is still open is then
/// metered as a boundary displacement — the margin's lower end over
/// the two bodies' summed area — and decided against the model's own
/// band: inside it, the open range is
/// below the model's resolution and the bound is accepted as an
/// in-band margin is (below); certified beyond it, the measurement
/// cannot decide a question the model can tell apart, and the gate
/// refuses [`BooleanError::VolumeUndecided`]. On a body whose faces are
/// all closed-form both half-widths are exactly zero and every
/// decision is the closed form's, re-derived in interval arithmetic
/// before it refuses (below).
///
/// Comparison posture: each bound margin is classified through the
/// k_stats funnel (`decide` — the certified trilean against the op's
/// linear band, under this gate's own predicate name), the codebase's
/// only legal comparison (Q1). Only a CERTIFIED violating sign
/// refuses ([`BooleanError::ResultVolumeImplausible`]);
/// `Zero` and in-band indeterminate margins PASS. A POISONED margin
/// (NaN) still refuses loudly ([`BooleanError::Escalated`]) — poison
/// never passes a gate.
///
/// # Rounding
///
/// A closed-form face's flux and the walk's sum round at `f64` with no
/// pad, so a result congruent to an operand through another face order
/// can land ulps past it. A violation the walk's sums call is therefore
/// re-derived in interval arithmetic before it refuses
/// ([`crate::props::PastTarget::interval_volume`]); one the interval
/// margin does not certify stays open.
///
/// A declared coincidence the door glues inside the band also moves a
/// correct result's volume, by up to the band over the glued face, and
/// at a tight bound that refuses: a correct body refused, the safe
/// direction
/// (`work/reachhold/a-settled-declared-coincidence-crosses-a-tight-volume-bound.md`).
///
/// Each bound applies only when its reference operand's volume is
/// certified POSITIVE (a bounded solid). An operand is a finished body,
/// so check 7 passed on each of its solids, and what reaches here
/// uncertified is an operand whose sign stayed open in the backstop's
/// own lane; against it the bound is skipped, never misread as a
/// violation.
///
/// # Dimension (audit F3, `docs/predicate-dimension-audit.md`)
///
/// The bounds are statements about VOLUMES (m³) but ε is a point
/// deviation (D4), so each margin is metered to a LENGTH before it is
/// decided. Displacing a boundary by δ changes the volume it encloses
/// by ≈ δ·A, so a volume defect `ΔV` between two compared bodies is
/// explained by a boundary deviation of `ΔV / (A_got + A_bound)` — the
/// sum of the two bodies' surface areas is the whole boundary that
/// could have moved, and the quotient is the MEAN BOUNDARY DISPLACEMENT
/// the bound violation corresponds to. (Summed, not one body's area:
/// both boundaries are free to move, so the sum is the honest total
/// lever. It is also the larger denominator, hence the smaller margin —
/// the anti-refusal direction, which is why arm 1 below exists.) The
/// operand-boundedness question is about one body, so
/// `volume_backstop_operand` uses that body's own area (`V/A`) —
/// verbatim the `positive_volume` precedent in `crate::validate`'s
/// tier-3 check 7. Both quotients are exact zero when the volumes
/// agree exactly, so the gate's non-strict pass direction is unmoved.
///
/// ## Two questions, two bands (the #200 review's MAJ-1)
///
/// Metering ALONE would have weakened this gate, and the direction is
/// worth naming: `ΔV/(A_got + A_bound)` shrinks with the bodies' area,
/// so a localized wrong-component defect on a large body can meter
/// below ε even though the defect is macroscopic. Executed
/// (`tests/probe_f34_review.rs`): a wrongly-kept 3 mm cube on a
/// 2 m × 2 m × 0.1 m plate is ΔV = 2.7e-8 m³ against ~17.6 m² of
/// boundary — 1.53e-9 m metered, inside the default band, where the
/// raw-m³ comparand had refused decisively.
///
/// The resolution is that the backstop asks TWO different questions and
/// only one of them is about a magnitude:
///
/// 1. **Is the inequality violated?** `vol(A ∖ B) ≤ vol(A)` is an
///    inequality, so a SIGN-CERTAIN negative margin is a violated bound
///    — a dimension-free fact, true regardless of how many metres of
///    boundary the defect is smeared over, down to the enclosures'
///    resolution on a quadrature-measured pair (above). This arm
///    decides against the **exact (bit-hairline) band**, where
///    "certain" means "proven beyond the enclosure's own width": a
///    negative the walk's sums call, which the interval re-derivation
///    then certifies (a straddling enclosure escalates and falls
///    through to the open arm). No ε enters — this is a sign, not a
///    comparison against a length. Predicate:
///    `volume_backstop_violation`.
/// 2. **Is the violation above the model's own resolution?** Only for
///    the near-zero region arm 1 leaves open, and there ε *is* the
///    right scale — which is exactly where the metered mean
///    displacement belongs. Predicate: `volume_backstop`.
///
/// Both arms consume the SAME metered comparand. Dividing by a
/// certainly-positive lever cannot change a sign, so arm 1 certifies
/// precisely the fact it would have certified on the raw volume, while
/// the recorded margin stays a length and the K telemetry stays
/// dimensionally honest (the whole point of F3). The one gap is
/// arithmetic rather than semantic: a defect small enough that the
/// quotient underflows to exactly zero (`|ΔV| ≲ 1e-322 m³` at `f64`)
/// would lose its sign — far below any representable model.
///
/// Both gates decide through the k_stats funnel under those names.
pub(crate) fn volume_backstop<T: Decide>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    result: &Body<T>,
    band: Band,
    tol: Tol,
    lane: QuadLane<T>,
) -> Result<(), BooleanError> {
    // Each body to its reporting target, keeping the walk so a bound
    // the reporting enclosures leave open can be refined past it.
    let measure = |body, operand| {
        let faces = crate::query::all_faces(body);
        let (refused, certificate) = crate::props::sign_walk(
            body,
            &faces,
            band,
            tol,
            Some(lane),
            Some(lane),
            |_| None,
            |refused| refused,
        )
        .map_err(|source| props_refusal(operand, source))?;
        match refused {
            Some(source) => Err(props_refusal(operand, source)),
            None => Ok(certificate.past_target()),
        }
    };
    // The exact (bit-hairline) band for the sign arm below — the same
    // device the splitter's total order uses (`splitting::order`, audit
    // note N6): its open interior holds no representable `f64`, so at
    // `f64` a sign is certain iff the margin is not exactly zero, and at
    // the interval scalar an enclosure straddling the hairline escalates
    // honestly. That IS "proven beyond the enclosure's own width".
    let exact = crate::splitting::order::exact_band()?;
    let mut ca = measure(a, Some(Operand::A))?;
    let mut cb = measure(b, Some(Operand::B))?;
    let mut cr = measure(result, None)?;
    // A bound applies only against a certified-bounded operand
    // (positive flux volume, at its enclosure's lower end); complement
    // operands (certified negative) make it vacuous. Poison refuses.
    // Metered `V/A` — the operand's MEAN THICKNESS (fn docs). Surface
    // area is unsigned (props.rs), so the quotient keeps V's sign and
    // the complement arm is unchanged. An in-band quotient is an
    // operand thinner than the model's own resolution — no bound is
    // certifiable from it, skip.
    // The backstops live on the INVARIANT LANE (Ev's #213 layering
    // ruling): consistency inequalities between integral results are
    // outside the length seam by design — no door, bare T — and a
    // certified violation is a kernel invariant failure, not a
    // validity refusal.
    let bounded = |p: crate::MassProperties<T>| -> Result<bool, BooleanError> {
        let e = p.enclosure();
        match geom_core::k_stats::decide_invariant(
            "volume_backstop_operand",
            e.volume_lo / e.surface_area,
            band,
        ) {
            Ok(Sign::Positive) => Ok(true),
            Ok(Sign::Zero | Sign::Negative) => Ok(false),
            Err(diag) if diag.margin.is_invalid() => Err(BooleanError::Escalated {
                decision: BooleanDecision::VolumeBackstop,
                diag,
            }),
            Err(_) => Ok(false),
        }
    };
    let (ba, bb) = (bounded(ca.props())?, bounded(cb.props())?);
    let (a_, b_, r_) = (Some(Operand::A), Some(Operand::B), None);
    let bound = Posture::Bound { band, exact };
    let plus_v = Posture::PlusV { band };
    match op {
        BooleanOp::Intersect => {
            if ba {
                let (small, large) = (&mut [(r_, &mut cr)], &mut [(a_, &mut ca)]);
                bound_holds("vol(A ∩ B) ≤ vol(A)", small, large, bound)?;
            }
            if bb {
                let (small, large) = (&mut [(r_, &mut cr)], &mut [(b_, &mut cb)]);
                bound_holds("vol(A ∩ B) ≤ vol(B)", small, large, bound)?;
            }
            if ba || bb {
                bound_holds("vol(A ∩ B) ≥ 0", &mut [], &mut [(r_, &mut cr)], plus_v)?;
            }
        }
        BooleanOp::Union => {
            if ba {
                let (small, large) = (&mut [(a_, &mut ca)], &mut [(r_, &mut cr)]);
                bound_holds("vol(A ∪ B) ≥ vol(A)", small, large, bound)?;
            }
            if bb {
                let (small, large) = (&mut [(b_, &mut cb)], &mut [(r_, &mut cr)]);
                bound_holds("vol(A ∪ B) ≥ vol(B)", small, large, bound)?;
            }
            if ba && bb {
                let (small, large) = (&mut [(r_, &mut cr)], &mut [(a_, &mut ca), (b_, &mut cb)]);
                bound_holds("vol(A ∪ B) ≤ vol(A) + vol(B)", small, large, bound)?;
            }
        }
        BooleanOp::Subtract => {
            if ba {
                let (small, large) = (&mut [(r_, &mut cr)], &mut [(a_, &mut ca)]);
                bound_holds("vol(A ∖ B) ≤ vol(A)", small, large, bound)?;
                bound_holds("vol(A ∖ B) ≥ 0", &mut [], &mut [(r_, &mut cr)], plus_v)?;
            }
            if ba && bb {
                let (small, large) = (&mut [(a_, &mut ca)], &mut [(r_, &mut cr), (b_, &mut cb)]);
                bound_holds("vol(A ∖ B) ≥ vol(A) − vol(B)", small, large, bound)?;
            }
        }
    }
    Ok(())
}

/// The property layer's refusal, as the backstop reports it — in tier
/// 3's reading (`validate::classify_mass_props`): structure that does
/// not resolve is [`BooleanError::VolumeCorrupt`], anything else a body
/// the kernel has no measurement for, [`BooleanError::VolumeUnmeasured`].
fn props_refusal(operand: Option<Operand>, source: crate::MassPropsError) -> BooleanError {
    if crate::validate::classify_mass_props(&source).defect {
        BooleanError::VolumeCorrupt { operand, source }
    } else {
        BooleanError::VolumeUnmeasured { operand, source }
    }
}

/// One body of a backstop inequality: the operand a refusal of its
/// measurement names (`None` is the result), and the measurement.
type Term<'t, 'b, T> = (Option<Operand>, &'t mut crate::props::PastTarget<'b, T>);

/// Which side of `Σ small ≤ Σ large` a body's volume is summed on.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Small,
    Large,
}

/// How a backstop inequality's margin is decided.
#[derive(Clone, Copy)]
enum Posture {
    /// A set bound ([`volume_backstop`]'s fn docs, "Two questions, two
    /// bands"): a violation certified at the `exact` band refuses
    /// whatever its size, an open sign is refined and then decided
    /// against `band`, and the magnitude arm reads `band`.
    Bound { band: Band, exact: Band },
    /// Tier 3's positive-volume invariant (`docs/DESIGN.md`, tier 3),
    /// read at the door by tier 3's own reading
    /// ([`crate::validate::plus_v_read`]): only a negative certified
    /// past the band refuses, and zero, in-band and open are exempt.
    PlusV { band: Band },
}

/// What a margin's two ends say, under a [`Posture`].
#[derive(Clone, Copy, PartialEq, Eq)]
enum Reading {
    /// Certified violated.
    Violated,
    /// Certified held.
    Held,
    /// Neither, at these ends.
    Open,
}

impl Posture {
    /// The margin's reading — exact, or a bracket over the bodies'
    /// quadrature pads ([`crate::props::MassProperties::reading`]) — read
    /// at whatever scalar it is carried in.
    fn read<U: Decide>(self, margin: crate::props::SignReading<U>) -> Reading {
        match self {
            Posture::Bound { exact, .. } => {
                let (lo, hi, lever) = margin.ends();
                let sign = |end: U| {
                    geom_core::k_stats::decide_invariant(
                        "volume_backstop_violation",
                        end / lever,
                        exact,
                    )
                };
                let upper = sign(hi);
                let held = if margin.is_exact() {
                    upper.is_ok()
                } else {
                    matches!(sign(lo), Ok(Sign::Zero | Sign::Positive))
                };
                if upper == Ok(Sign::Negative) {
                    Reading::Violated
                } else if held {
                    Reading::Held
                } else {
                    Reading::Open
                }
            }
            Posture::PlusV { band } => match crate::validate::plus_v_read(margin, band) {
                Some(crate::props::ShellRole::Void) => Reading::Violated,
                Some(crate::props::ShellRole::Outer) => Reading::Held,
                None => Reading::Open,
            },
        }
    }
}

/// A body's volume and area in interval arithmetic
/// ([`crate::props::PastTarget::interval_volume`]), a refusal naming
/// the body by `operand`.
fn interval_measure<T: Decide>(
    which: &'static str,
    operand: Option<Operand>,
    target: &crate::props::PastTarget<'_, T>,
) -> Result<(Interval, Interval), BooleanError> {
    match target.interval_volume() {
        Some(Ok(measured)) => Ok(measured),
        Some(Err(source)) => Err(props_refusal(operand, source)),
        None => Err(BooleanError::VolumeUndecided { which }),
    }
}

/// One inequality of [`volume_backstop`], `Σ small ≤ Σ large` over the
/// bodies' volumes, named by `which` and decided as `posture` says:
/// refuse a certified violation once the interval re-derivation
/// confirms it, refine while the enclosures alone keep the sign open,
/// and (a set bound) decide what the last round leaves against the
/// band, then the magnitude arm.
fn bound_holds<'t, 'b, T: Decide>(
    which: &'static str,
    small: &mut [Term<'t, 'b, T>],
    large: &mut [Term<'t, 'b, T>],
    posture: Posture,
) -> Result<(), BooleanError> {
    let escalated = |diag| BooleanError::Escalated {
        decision: BooleanDecision::VolumeBackstop,
        diag,
    };
    loop {
        // The margin `Σ large − Σ small`, and the summed surface area it
        // is metered over (fn docs, audit F3).
        let mut margin = crate::props::SignReading::exact(T::zero(), T::zero());
        // The result's volume, and what the other bodies bound it by,
        // for the refusal's text.
        let (mut got, mut others) = (T::zero(), T::zero());
        let mut result_side = Side::Small;
        for (side, terms) in [(Side::Small, &*small), (Side::Large, &*large)] {
            for (operand, target) in terms {
                let p = target.props();
                margin = margin.plus(p.reading(), side == Side::Small);
                match (operand, side) {
                    (None, _) => (got, result_side) = (p.volume, side),
                    (Some(_), Side::Small) => others = others - p.volume,
                    (Some(_), Side::Large) => others = others + p.volume,
                }
            }
        }
        let bound = match result_side {
            Side::Small => others,
            Side::Large => -others,
        };
        let implausible = || BooleanError::ResultVolumeImplausible {
            which,
            got: format!("{got:?}"),
            bound: format!("{bound:?}"),
        };
        let (mut lo, hi, lever) = margin.ends();
        let metered = hi / lever;
        let reading = posture.read(margin);
        // Open: the enclosures keep the sign open.
        let mut open = !margin.is_exact() && reading == Reading::Open;
        // Arm 1 — the inequality itself, at the margin's upper end. The
        // walk's own sums round (`PastTarget::interval_volume`), so a
        // violation they call is re-derived in interval arithmetic
        // before it refuses: one the interval margin does not certify
        // is the rounding's, and stays open.
        if reading == Reading::Violated {
            let (mut margin, mut area) = (Interval::zero(), Interval::zero());
            for (side, terms) in [(Side::Small, &*small), (Side::Large, &*large)] {
                for (operand, target) in terms {
                    let (v, a) = interval_measure(which, *operand, target)?;
                    margin = match side {
                        Side::Small => margin - v,
                        Side::Large => margin + v,
                    };
                    area = area + a;
                }
            }
            if posture.read(crate::props::SignReading::exact(margin, area)) == Reading::Violated {
                return Err(implausible());
            }
            open = true;
            lo = T::from_computed(margin.lo());
        }
        if open {
            // Every body, not the first that moves.
            let mut moved = false;
            for (_, target) in small.iter_mut().chain(large.iter_mut()) {
                moved |= target.refine();
            }
            if moved {
                continue;
            }
        }
        let Posture::Bound { band, .. } = posture else {
            // The +V invariant exempts what is left open, as at rest.
            return Ok(());
        };
        // The rounds ran out with the sign open: what is left open,
        // metered, is either below the model's resolution or not.
        if open {
            match geom_core::k_stats::decide_invariant("volume_backstop", lo / lever, band) {
                Ok(Sign::Negative) => return Err(BooleanError::VolumeUndecided { which }),
                Err(diag) if diag.margin.is_invalid() => return Err(escalated(diag)),
                Ok(Sign::Zero | Sign::Positive) | Err(_) => {}
            }
        }
        // Arm 2 — the magnitude, for the near-zero region arm 1 leaves
        // open: only a certified negative refuses, Zero and in-band
        // PASS, poison refuses.
        return match geom_core::k_stats::decide_invariant("volume_backstop", metered, band) {
            Ok(Sign::Negative) => Err(implausible()),
            Ok(Sign::Zero | Sign::Positive) => Ok(()),
            Err(diag) if diag.margin.is_invalid() => Err(escalated(diag)),
            Err(_) => Ok(()),
        };
    }
}

/// **The boolean's output stage**, the one every result path takes
/// (`docs/DESIGN.md`, the merge stage): the merge of coplanar faces
/// over the `declared` surface pairs (none where one operand is absent
/// from the result), the re-description of the edges the op minted
/// (`seam_edges`, plus the boundaries of groups the merge skipped), and
/// the join ([`super::edge_join::join_stage`]), each writing its rows
/// into the op's one `desc`. The result then has maximal faces and
/// maximal edges, and its records are carried over `desc` after.
///
/// # Errors
///
/// The merge's ([`BooleanError::Merge`]), the description's, or the
/// join's.
pub(super) fn finish_output<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    desc: &mut Descendants,
    declared: &[(SurfaceKey, SurfaceKey)],
    seam_edges: &[EdgeKey],
    recorded: &BTreeSet<VertexKey>,
    band: Band,
    tol: Tol,
) -> Result<
    (
        crate::merge_faces::MergeCoplanarOutcome,
        Vec<super::EdgeJoin>,
    ),
    BooleanError,
> {
    #[cfg(feature = "sweep-testing")]
    OUTPUT_STAGE_POINTS.with(|s| {
        if let Some(stages) = s.borrow_mut().as_mut() {
            stages.push(
                body.vertices()
                    .map(|(v, data)| {
                        let p = body.point_of(v, data);
                        (v, [p.x, p.y, p.z].map(|c| format!("{c:?}")))
                    })
                    .collect(),
            );
        }
    });
    let merged = body
        .merge_coplanar_faces_unjoined(declared, tol)
        .map_err(of_merge)?;
    desc.absorb_merge(&merged);
    describe_minted_edges(body, seam_edges, &merged, band, tol)?;
    let edge_joins = join_stage(body, desc, recorded, band, tol)?;
    Ok((merged, edge_joins))
}

/// One output stage's vertices, each with its point rendered
/// ([`take_output_stage_points`]).
#[cfg(feature = "sweep-testing")]
pub type StagePoints = Vec<(VertexKey, [String; 3])>;

#[cfg(feature = "sweep-testing")]
thread_local! {
    /// Each vertex's point as the output stage found it, one list per
    /// [`finish_output`] call, while this thread is armed
    /// ([`arm_output_stage_points`]); `None` unarmed, and nothing is
    /// recorded.
    static OUTPUT_STAGE_POINTS: core::cell::RefCell<Option<Vec<StagePoints>>> =
        const { core::cell::RefCell::new(None) };
}

/// Arms this thread to record, at the start of every output stage, each
/// vertex's key and point before the stage's merge and join delete any
/// ([`take_output_stage_points`]), dropping anything recorded before.
/// Unarmed, the stage records nothing: only the row that asks pays for
/// it. `sweep-testing` only.
#[cfg(feature = "sweep-testing")]
pub fn arm_output_stage_points() {
    OUTPUT_STAGE_POINTS.with(|s| *s.borrow_mut() = Some(Vec::new()));
}

/// Drains what the output stages recorded on this armed thread since
/// the last drain or arming, one list per stage, and stays armed: what
/// a row needs to check that the cell a deleted vertex settles on
/// ([`Settler::settle`]) holds its point, which the result no longer
/// stores. Each coordinate is its `{:?}` rendering, as
/// [`super::zip::take_shared_points`] gives it and for its reason.
/// `None` on an unarmed thread, where nothing could have been recorded.
#[cfg(feature = "sweep-testing")]
pub fn take_output_stage_points() -> Option<Vec<StagePoints>> {
    OUTPUT_STAGE_POINTS.with(|s| s.borrow_mut().as_mut().map(core::mem::take))
}

/// D6 (M3 PR 6a): honest descriptions on boolean-minted edges AT MINT
/// TIME — the worklist is tracked lineage (the zips' surviving seam
/// edges plus every boundary edge of a face whose merge group the
/// merge door SKIPPED), never a post-hoc scan of the body. The
/// merge-kept faces' boundaries are the door's own: it re-describes
/// them before it returns ([`Body::merge_coplanar_faces_declared`]).
/// Each worklist edge is described by [`describe_edges`].
pub(super) fn describe_minted_edges<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    seam_edges: &[crate::entity::EdgeKey],
    merged: &crate::merge_faces::MergeCoplanarOutcome,
    band: Band,
    tol: Tol,
) -> Result<(), BooleanError> {
    let worklist = describe_worklist(body, seam_edges, merged)?;
    describe_edges(body, worklist, &merged.skipped, band, tol).map_err(of_describe)
}

/// [`describe_minted_edges`]' worklist: the seam edges that survived
/// the merge, then the skipped groups' faces' boundaries.
fn describe_worklist<T: Real>(
    body: &Body<T>,
    seam_edges: &[crate::entity::EdgeKey],
    merged: &crate::merge_faces::MergeCoplanarOutcome,
) -> Result<Vec<crate::entity::EdgeKey>, BooleanError> {
    let mut worklist: Vec<crate::entity::EdgeKey> = Vec::new();
    for &e in seam_edges {
        if body.get_edge(e).is_some() {
            worklist.push(e); // merge may have consumed flush seam edges
        }
    }
    // SKIPPED groups' faces' boundaries (M4 PR 5 F1: the glue those
    // groups' classification anticipated did NOT happen, so their
    // in-plane cut edges may carry descriptions citing no-longer-
    // adjacent surfaces — they must be re-checked against the ACTUAL
    // adjacency). A declared pair the door declined (a non-planar
    // carrier) enters this worklist through the same field: its faces
    // were left as the zip shipped them, and their boundaries are
    // re-checked here for the same reason.
    let skipped = boundary_edges(
        body,
        merged.skipped.iter().flat_map(|s| s.faces.iter().copied()),
    )
    .ok_or(BooleanError::JoinDesync {
        what: "description worklist edge not walkable",
    })?;
    worklist.extend(skipped.into_iter().map(|(_, edge)| edge));
    Ok(worklist)
}

/// Every boundary edge of each live face in `faces`, once, with the
/// first listed face that holds it: outer loop then rings, each in
/// cycle order. A face that no longer resolves contributes nothing;
/// `None` when a loop of a live face does not walk.
pub(crate) fn boundary_edges<T: Real>(
    body: &Body<T>,
    faces: impl IntoIterator<Item = FaceKey>,
) -> Option<Vec<(FaceKey, crate::entity::EdgeKey)>> {
    let mut seen: BTreeSet<crate::entity::EdgeKey> = BTreeSet::new();
    let mut edges = Vec::new();
    for f in faces {
        let Some(face) = body.get_face(f) else {
            continue;
        };
        for &lk in core::iter::once(&face.outer).chain(&face.rings) {
            let LoopBoundary::Cycle { first } = body.get_loop(lk)?.boundary else {
                continue;
            };
            for he in body.loop_cycle(first)? {
                let edge = body.get_half_edge(he)?.edge;
                if seen.insert(edge) {
                    edges.push((f, edge));
                }
            }
        }
    }
    Some(edges)
}

/// The boolean's refusal for an edge its describer could not describe.
/// Its line chord read in band of one of its faces is the input's (D9
/// row 1, [`crate::merge_faces::edge_on_faces_reading`]); every other
/// failure is the kernel's.
pub(super) fn of_describe(refusal: DescribeRefusal) -> BooleanError {
    match refusal {
        DescribeRefusal::Failed {
            edge,
            failure,
            cause,
            fitted,
        } => BooleanError::EdgeUndescribed {
            edge,
            failure,
            cause,
            fitted,
        },
        DescribeRefusal::Undecided { reading, diag, .. } => seam_refusal(reading, diag),
        DescribeRefusal::OffFace { diag, .. } => BooleanError::Escalated {
            decision: BooleanDecision::EdgeOnFaces,
            diag,
        },
    }
}

/// The describer's refusal for `edge` when the attachment gate refuses
/// its description on a carrier of kind `carrier`: the edge's own
/// reading on its faces where
/// [`crate::merge_faces::edge_on_faces_reading`] finds one, and
/// otherwise a failed edge carrying the gate's refusal, ending as a
/// fitted carrier's or the kernel's.
fn of_attachment(
    edge: crate::entity::EdgeKey,
    failure: EdgeDescribeFailure,
    carrier: geom::CurveKind,
    cause: crate::euler::EulerOpError,
) -> DescribeRefusal {
    match crate::merge_faces::edge_on_faces_reading(carrier, &cause) {
        Some(diag) => DescribeRefusal::off_face(edge, diag),
        None => DescribeRefusal::refused(edge, failure, cause, carrier == geom::CurveKind::Nurbs),
    }
}

/// The boolean's refusal for a merge-door refusal: the door's
/// re-description of its kept faces' boundaries refuses as the
/// boolean's own description pass does ([`of_describe`]), and every
/// other refusal is the output stage's.
pub(super) fn of_merge(refusal: crate::merge_faces::MergeCoplanarError) -> BooleanError {
    match refusal.kept_boundary() {
        Some(described) => of_describe(described),
        None => BooleanError::Merge(refusal),
    }
}

/// Describes each worklist edge from its two faces' surfaces
/// (structural adjacency): definitely transverse ⇒ `Intersection` with
/// the chord-midpoint witness; definitely smooth ⇒ the must-carry rule
/// over the edge ([`must_carry_reading`]) — the intrinsic
/// `TangentIntersection` where the surfaces determine the locus, else
/// the conventional description (D2's split); an undecided reading
/// refuses typed. `skipped` is the merge stage's record of the groups
/// it left unglued. The boolean's minted edges and the merge door's
/// kept boundaries are both described here, and each door words the
/// refusal as its own.
///
/// A worklist edge is a key the caller carries: its miss refuses typed
/// ([`EdgeDescribeFailure::NotWalkable`]).
///
/// # Panics
///
/// Where a record past a resolved worklist edge does not resolve (D2
/// row 4): its halves, their loops and faces, the faces' surfaces, its
/// ends' points and its curve. A torn curve is not an uncertified one,
/// and a torn half's face is not one outside a recorded skip. Both
/// callers run it mid-operation (the boolean's output stage, the merge
/// door's re-description), where links hold by
/// [`crate::live::OPERATORS_KEEP_LINKS`].
pub(crate) fn describe_edges<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    worklist: impl IntoIterator<Item = crate::entity::EdgeKey>,
    skipped: &[crate::merge_faces::SkippedMerge],
    band: Band,
    tol: Tol,
) -> Result<(), DescribeRefusal> {
    // Whether two faces both belong to one recorded merge skip: the
    // licensed cosurface pairs the merge stage ships unglued.
    let recorded_skip = |(f1, f2): (FaceKey, FaceKey)| {
        skipped
            .iter()
            .any(|s| s.faces.contains(&f1) && s.faces.contains(&f2))
    };
    for edge in worklist {
        let corrupt = || DescribeRefusal::failed(edge, EdgeDescribeFailure::NotWalkable);
        let edge_data = body.get_edge(edge).ok_or_else(corrupt)?.clone();
        let sides = crate::readback::edge_sides_of(body, edge, &edge_data);
        let (s1, s2) = sides.surfaces();
        let he_plus = sides.plus.half_edge;
        let start = proven(&body.half_edges, he_plus, EntityId::HalfEdge).start;
        let p0 = body.linked_vertex_point(start, EntityId::HalfEdge(he_plus), "start");
        let p1 = body.linked_vertex_point(
            body.proven_half_edge_end(he_plus),
            EntityId::HalfEdge(he_plus),
            "end",
        );
        let surface_of = |side: crate::readback::EdgeSide| {
            body.face_surface_linked(side.face, proven(&body.faces, side.face, EntityId::Face))
        };
        let (surf1, surf2) = (surface_of(sides.plus), surface_of(sides.minus));
        let existing = body
            .edge_curve_linked(edge, &edge_data)
            .certified()
            .cloned();
        let curved = existing.as_ref().is_some_and(|c| c.carrier().is_curved());
        let draft = geom_brep::IntersectionDraft::of(existing.as_ref(), p0, p1);
        let (witness, extent) = (draft.witness, draft.extent);
        match seam_reading(surf1, surf2, witness, extent, band)
            .map_err(|(reading, diag)| DescribeRefusal::undecided(edge, reading, diag))?
        {
            geom_brep::DihedralClass::Transverse => {
                let spec = draft.into_spec(geom_brep::SurfacePair::new(s1, s2));
                let carrier = spec.carrier.kind();
                body.set_edge_curve(edge, spec, tol).map_err(|e| {
                    of_attachment(edge, EdgeDescribeFailure::Intersection, carrier, e)
                })?;
            }
            geom_brep::DihedralClass::Smooth => {
                // F1 (the declared-merge SKIP lane): a SURVIVING
                // smooth-adjacency edge whose existing
                // intrinsic or chart description no longer cites
                // its two adjacent faces' surfaces would violate D2
                // adjacency coherence at tier 3 — the glue its
                // description anticipated was skipped (or the merge
                // re-homed its neighbors, or the zip fused it between
                // new faces). Re-describe conventionally where the
                // surfaces under-determine the locus (D2's split).
                let stale = match existing.as_ref().ok_or_else(corrupt)?.description() {
                    geom_brep::EdgeDescription::Intersection { pair, .. }
                    | geom_brep::EdgeDescription::TangentIntersection { pair, .. } => {
                        *pair != geom_brep::SurfacePair::new(s1, s2)
                    }
                    // A chart image cites ONE adjacent surface (its
                    // residual chart); stale iff neither side is it
                    // (the attach-door adjacency rule, M6-3) — except
                    // a wrap edge, whose two halves bound one face by
                    // what a wrap edge is (D1).
                    geom_brep::EdgeDescription::Chart(c) if c.wrap => {
                        !(c.surface == s1 && sides.plus.face == sides.minus.face)
                    }
                    geom_brep::EdgeDescription::Chart(c) => !(c.surface == s1 || c.surface == s2),
                    // A scaffold comes to rest here only between the two
                    // faces of a declared pair the merge stage could not
                    // glue and RECORDED (a curved group: DESIGN's
                    // "only a curved group's skip is recorded and
                    // shipped"); it is described where it rests. Any
                    // other scaffold stays one, and tier 3 refuses it.
                    geom_brep::EdgeDescription::Scaffold(_) => recorded_skip(sides.faces()),
                };
                // The D6 smooth ladder (M9-3): a definitely-smooth
                // seam descends one order through the must-carry rule
                // over the edge, at the stations tier 3's must-carry arm
                // re-reads (`must_carry_reading`).
                let mint_intrinsic = {
                    let c = existing.as_ref().ok_or_else(corrupt)?;
                    let (t0, t1) = c.params();
                    let (_, [sa, sb]) = geom_brep::SurfacePair::sorted(s1, surf1, s2, surf2);
                    must_carry_reading(sa, sb, c.carrier(), t0, t1, extent, band).map_err(
                        |(reading, diag)| DescribeRefusal::undecided(edge, reading, diag),
                    )?
                };
                if mint_intrinsic {
                    // Mint the intrinsic tangency on the existing
                    // carrier (U2: today's taxonomy, 1:1 onto
                    // (surface, exact-lane pcurve)) — this also
                    // refreshes a stale citation, since the minted
                    // surfaces are the CURRENT adjacency.
                    let c = existing.as_ref().ok_or_else(corrupt)?;
                    let (t0, t1) = c.params();
                    let spec = geom_brep::EdgeCurveSpec {
                        description: geom_brep::EdgeDescriptionSpec::TangentIntersection {
                            pair: geom_brep::SurfacePair::new(s1, s2),
                            witness,
                        },
                        carrier: c.carrier().clone(),
                        param_start: t0,
                        param_end: t1,
                    };
                    body.set_edge_curve(edge, spec, tol).map_err(|e| {
                        of_attachment(edge, EdgeDescribeFailure::Tangency, c.carrier().kind(), e)
                    })?;
                } else if stale {
                    if curved {
                        // The conventional re-description for an arc
                        // the adjacent surfaces under-determine, on the
                        // UNCHANGED carrier (no silent geometric
                        // rewrite — only the description moves). The
                        // edge comes to REST here, so it is described
                        // where it rests — as an image in `s1`'s own
                        // chart (D3's transience fence) — and the arc
                        // pushforward that used to BE the description
                        // is recorded as the authority beside it.
                        // Carrier kinds with no conventional
                        // pushforward keep the typed refusal.
                        let c = existing.as_ref().ok_or_else(corrupt)?;
                        let (t0, t1) = c.params();
                        let Some(spec) =
                            geom_brep::EdgeCurveSpec::arc_of_circle(c.carrier().clone(), t0, t1)
                        else {
                            return Err(DescribeRefusal::failed(
                                edge,
                                EdgeDescribeFailure::NoConventionalLane,
                            ));
                        };
                        body.set_edge_curve(edge, spec.at_rest_in_chart(s1, false), tol)
                            .map_err(|e| {
                                of_attachment(edge, EdgeDescribeFailure::Arc, c.carrier().kind(), e)
                            })?;
                    } else {
                        let spec = geom_brep::EdgeCurveSpec::line_between(p0, p1);
                        let carrier = spec.carrier.kind();
                        body.set_edge_curve(edge, spec.at_rest_in_chart(s1, false), tol)
                            .map_err(|e| {
                                of_attachment(edge, EdgeDescribeFailure::Line, carrier, e)
                            })?;
                    }
                }
            }
        }
    }
    Ok(())
}

/// **A seam edge of the result, as its re-description reads it**: the
/// dihedral of its two surfaces at `witness` over `extent`, whose arm
/// rung is the seam's own lever and whose reading is the seam's wedge.
#[cfg(test)]
pub(super) fn seam_class<T: Decide>(
    surf1: &geom::Surface<T>,
    surf2: &geom::Surface<T>,
    witness: Point3<T>,
    extent: T,
    band: Band,
) -> Result<geom_brep::DihedralClass, BooleanError> {
    seam_reading(surf1, surf2, witness, extent, band)
        .map_err(|(reading, diag)| seam_refusal(reading, diag))
}

/// [`seam_class`]'s reading, unworded: the rung that could not decide
/// and its diagnostics.
fn seam_reading<T: Decide>(
    surf1: &geom::Surface<T>,
    surf2: &geom::Surface<T>,
    witness: Point3<T>,
    extent: T,
    band: Band,
) -> Result<geom_brep::DihedralClass, (DihedralReading, Indeterminate)> {
    geom_brep::classify_dihedral(surf1, surf2, witness, extent, band)
        .map_err(DihedralReading::of_lever)
}

/// The boolean's refusal for an undecided seam reading: the seam's
/// first-order arm or wedge, by rung, or its second-order bend. The
/// boolean's decision reads the rung and the margin alone, so no
/// [`geom_brep::LeverEscalation`] is rebuilt from the two: the arm's
/// verdict is the gate's to mint.
pub(super) fn seam_refusal(reading: DihedralReading, diag: Indeterminate) -> BooleanError {
    match reading {
        DihedralReading::Lever(rung) => BooleanError::of_lever_rung(
            super::LeverArm::Seam,
            super::DeclarationRead::Moot,
            rung,
            diag,
        ),
        DihedralReading::Bend => BooleanError::Escalated {
            decision: BooleanDecision::SeamJet,
            diag,
        },
    }
}

/// **A smooth seam edge of the result, one order down**: whether its
/// two surfaces determine the locus along it, by the must-carry rule
/// over the edge ([`geom_brep::must_carry_over_edge`]), at the
/// stations tier 3's must-carry arm reads.
///
/// - jet-determinate ⇒ `true`: the intrinsic `TangentIntersection` is
///   demanded;
/// - under-determined ⇒ `false`: the conventional description is the
///   honest one (coplanar planes' exact-zero jet lands here, so a planar
///   split keeps its chord description);
/// - a station that reads the seam a corner ⇒ `false`: the seam is
///   smooth at its witness and transverse elsewhere, an edge tier 3
///   holds to neither description, so it keeps the conventional one;
/// - in band at a station ⇒ the typed escalation of the reading that
///   raised it: the seam's first-order arm or wedge, by rung, as
///   [`seam_class`] ends it, or its second-order bend
///   ([`BooleanDecision::SeamJet`]). Certifiable as neither, so never
///   folded into either description (D4 ¶3).
#[cfg(test)]
pub(super) fn seam_must_carry<T: Decide>(
    surf1: &geom::Surface<T>,
    surf2: &geom::Surface<T>,
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    extent: T,
    band: Band,
) -> Result<bool, BooleanError> {
    must_carry_reading(surf1, surf2, carrier, t0, t1, extent, band)
        .map_err(|(reading, diag)| seam_refusal(reading, diag))
}

/// [`seam_must_carry`]'s verdict, unworded: an undecided station
/// returns the reading that raised it and its diagnostics.
fn must_carry_reading<T: Decide>(
    surf1: &geom::Surface<T>,
    surf2: &geom::Surface<T>,
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    extent: T,
    band: Band,
) -> Result<bool, (DihedralReading, Indeterminate)> {
    use geom_brep::MustCarryVerdict;
    match geom_brep::must_carry_over_edge(surf1, surf2, carrier, t0, t1, extent, band) {
        MustCarryVerdict::JetDeterminate => Ok(true),
        MustCarryVerdict::UnderDetermined | MustCarryVerdict::Transverse => Ok(false),
        MustCarryVerdict::InBand(escalation) => Err(DihedralReading::of_must_carry(escalation)),
    }
}

/// How one operand's keys map into the result body.
#[derive(Clone, Copy)]
pub(super) enum KeyView<'a> {
    /// Keys carried through unchanged (carve preserves keys).
    Direct,
    /// Keys bridged by the combine door's graft.
    Graft(&'a GraftMap),
    /// The operand is not part of the result.
    Absent,
}

impl KeyView<'_> {
    fn vertex(&self, v: VertexKey) -> Option<VertexKey> {
        match self {
            Self::Direct => Some(v),
            Self::Graft(g) => g.vertices.get(v).copied(),
            Self::Absent => None,
        }
    }

    fn face(&self, f: FaceKey) -> Option<FaceKey> {
        match self {
            Self::Direct => Some(f),
            Self::Graft(g) => g.faces.get(f).copied(),
            Self::Absent => None,
        }
    }

    fn edge(&self, e: EdgeKey) -> Option<EdgeKey> {
        match self {
            Self::Direct => Some(e),
            Self::Graft(g) => g.edges.get(e).copied(),
            Self::Absent => None,
        }
    }
}

/// The D5 descendant map (M3 PR 6a, PR 5 review R5): result-stage
/// entity replacement — seam-zip vertex fusions and
/// `merge_coplanar_faces` face absorption — as old key → surviving
/// key rows, extending the graft's key lineage so a contact record
/// drops ONLY when its coincidence is consumed (entity gone, not
/// renamed). Re-derivation at the 3′ gate is rejected as
/// scan-to-bless (F1); the descendants ARE the mint-time knowledge.
#[derive(Default)]
pub(super) struct Descendants {
    /// The vertex fusions, in mint order, result keys: the zips'
    /// ([`Self::absorb_fusions`]).
    vertices: Fusions,
    /// Merge absorption, absorbed face → the group's kept face: an
    /// acyclic relation, since a kept face is never absorbed. A cycle
    /// is a corrupt record, and [`Self::live`] refuses it typed.
    faces: std::collections::BTreeMap<FaceKey, FaceKey>,
    /// Every vertex a row of `vertices` names (dead OR kept): its point
    /// rests were consumed into seam structure.
    fused: std::collections::BTreeSet<VertexKey>,
    /// Each operand's null-edge copies, in its clone keys: the vertices
    /// one null edge joins, on one point by construction.
    copies: [Vec<(VertexKey, VertexKey)>; 2],
    /// Each operand's chords along its own edges, `(edge, chord)` in
    /// its clone keys, read before its key view: the join minted
    /// `chord` on a segment that IS `edge` (`join::Connected::along`),
    /// so where the op drops `edge`, the chord holds its interior.
    /// Read only where `edge` left the result.
    along: [Vec<(EdgeKey, EdgeKey)>; 2],
    /// Each operand's face divisions, `(new face, divided-from face)` in
    /// its clone keys, read before its key view: the join's chord-mef
    /// fragments. A division keeps the divided
    /// face's key on one piece, whichever piece that is, so a record
    /// naming the face is read through all of its pieces
    /// ([`Self::holder`]).
    fragments: [Vec<(FaceKey, FaceKey)>; 2],
    /// Every other substitution, dead cell → the cell that holds its
    /// interior now, result keys: a zipped ring edge → its seam edge, a
    /// cell the merge swallowed → the kept face, a joined vertex or
    /// edge → the joined edge.
    cells: BTreeMap<Cell, Cell>,
}

impl Descendants {
    /// The two ends of each of the reduction's null edges, per operand.
    pub(super) fn null_copies<T: Real>(
        null_edges: &[super::BoolNullEdgeRecord<T>],
    ) -> [Vec<(VertexKey, VertexKey)>; 2] {
        let mut copies = [Vec::new(), Vec::new()];
        for r in null_edges {
            copies[usize::from(r.operand == Operand::B)].push((r.attr.below_end, r.attr.above_end));
        }
        copies
    }

    /// The map that also reaches each v-v group's null-edge copies
    /// ([`carry`]).
    pub(super) fn with_copies(self, copies: [Vec<(VertexKey, VertexKey)>; 2]) -> Self {
        Self { copies, ..self }
    }

    /// The map that also reaches the chords the join minted along each
    /// operand's own edges ([`Self::along`]).
    pub(super) fn with_along(self, along: [Vec<(EdgeKey, EdgeKey)>; 2]) -> Self {
        Self { along, ..self }
    }

    /// The map that also reads each operand's face divisions
    /// ([`Self::fragments`]).
    pub(super) fn with_fragments(self, fragments: [Vec<(FaceKey, FaceKey)>; 2]) -> Self {
        Self { fragments, ..self }
    }

    /// `v` and every vertex null edges join it to, transitively, in
    /// `side`'s clone keys.
    fn copies_of(&self, side: Operand, v: VertexKey) -> Vec<VertexKey> {
        let rows = &self.copies[usize::from(side == Operand::B)];
        let mut out = vec![v];
        let mut i = 0;
        while let Some(&at) = out.get(i) {
            for &(x, y) in rows {
                let other = if x == at {
                    y
                } else if y == at {
                    x
                } else {
                    continue;
                };
                if !out.contains(&other) {
                    out.push(other);
                }
            }
            i += 1;
        }
        out
    }

    /// Every vertex fusion in result keys, in mint order.
    pub(super) fn vertex_merges(&self) -> Fusions {
        self.vertices.clone()
    }

    /// A zip's fusions ([`Self::absorb_fusions`]) and edge merges.
    ///
    /// # Errors
    ///
    /// As [`Self::absorb_fusions`].
    pub(super) fn absorb_zip(&mut self, rep: &super::zip::ZipReport) -> Result<(), BooleanError> {
        self.absorb_fusions(&rep.vertex_merges)?;
        for &(dead, kept) in &rep.edge_merges {
            if dead != kept {
                self.substitute(Cell::Edge(dead), Cell::Edge(kept));
            }
        }
        Ok(())
    }

    /// Vertex fusions `(dead, kept)`, result keys: a zip's.
    ///
    /// # Errors
    ///
    /// [`BooleanError::JoinDesync`] where `merges` names a key the
    /// fusions before it killed ([`Fusions::extend`]).
    fn absorb_fusions(&mut self, merges: &Fusions) -> Result<(), BooleanError> {
        self.vertices.extend(merges)?;
        for &(dead, kept) in merges {
            self.fused.insert(dead);
            self.fused.insert(kept);
        }
        Ok(())
    }

    pub(super) fn absorb_merge(&mut self, merged: &crate::merge_faces::MergeCoplanarOutcome) {
        // The boolean merges unjoined (`finish_output`): its joins are
        // written by its own join stage, after the re-description.
        debug_assert!(
            merged.joins.is_empty(),
            "absorb_merge: the boolean's merge is the unjoined one, so it reports no join"
        );
        let mut seen = BTreeSet::new();
        for group in &merged.groups {
            for (dead, by) in group.substitutions() {
                match dead {
                    Cell::Face(f) => {
                        self.faces.insert(f, group.kept);
                    }
                    _ => self.substitute(dead, by),
                }
                debug_assert!(
                    seen.insert(dead),
                    "absorb_merge: two merge rows name {dead:?}"
                );
            }
        }
    }

    /// Records that `by` holds `dead`'s interior now.
    pub(super) fn substitute(&mut self, dead: Cell, by: Cell) {
        self.cells.insert(dead, by);
    }

    /// Operand `side`'s cell, read through its key `view`, the zips'
    /// fusions (a vertex), then the face absorptions and the
    /// substitution rows until live: `None` where
    /// the chain ends at a dead cell with no row (the cell left the
    /// result). An edge whose chain ends so is read on through the
    /// chords the join minted along it ([`Self::along`]), each by the
    /// same chain: the one that is live holds it.
    ///
    /// # Errors
    ///
    /// [`BooleanError::JoinDesync`] on rows that cycle: a walk that
    /// outlasts the row count has revisited a cell, a corrupt record,
    /// and reading it as gone would drop a recorded contact. So too where
    /// two chords along one edge both reach a live cell, and they are
    /// not one: two edges cannot both hold one edge's interior.
    pub(super) fn live<T: Real>(
        &self,
        body: &Body<T>,
        side: Operand,
        view: &KeyView<'_>,
        cell: Cell,
    ) -> Result<Option<Cell>, BooleanError> {
        if let Some(c) = self.chained(body, view, cell)? {
            return Ok(Some(c));
        }
        let Cell::Edge(edge) = cell else {
            return Ok(None);
        };
        let mut held = None;
        let rows = &self.along[usize::from(side == Operand::B)];
        for &(_, chord) in rows.iter().filter(|&&(e, _)| e == edge) {
            // A chord is minted, never an `along` row's edge, so one
            // hop ends the read.
            match (held, self.chained(body, view, Cell::Edge(chord))?) {
                (_, None) => {}
                (None, Some(c)) => held = Some(c),
                (Some(h), Some(c)) if h == c => {}
                (Some(_), Some(_)) => {
                    return Err(BooleanError::JoinDesync {
                        what: "two live edges hold one edge's interior through the join's chords",
                    });
                }
            }
        }
        Ok(held)
    }

    /// Operand `side`'s face `face` as a vertex-on-face record reads it,
    /// `vertex` the record's vertex in result keys: the live piece of
    /// the face that holds the vertex. A face no division touched is
    /// [`Self::live`]'s read. A division keeps the key on one piece,
    /// which need not be the one the vertex rests on, so each live piece
    /// of a divided face (its lineage through [`Self::fragments`], each
    /// through [`Self::live`]) is placed ([`piece_holds`]): the piece the
    /// vertex bounds (the record is then structure), else the one whose
    /// interior holds it. `None` where every piece is read and none holds
    /// it: the part of the face it rested on left the result, or the
    /// vertex is on a piece's boundary.
    ///
    /// # Errors
    ///
    /// As [`Self::live`]; [`BooleanError::Escalated`] where a piece's
    /// margin is in band; [`BooleanError::RestPieceUnread`] where no
    /// piece holds the vertex and a piece could not be read, so the rest
    /// may be on it; [`BooleanError::JoinDesync`] where two pieces of one
    /// face both hold the vertex.
    fn holder<T: Decide>(
        &self,
        body: &Body<T>,
        side: Operand,
        view: &KeyView<'_>,
        face: FaceKey,
        vertex: VertexKey,
        band: Band,
    ) -> Result<Option<Cell>, BooleanError> {
        let desync = |what| BooleanError::JoinDesync { what };
        let lineage =
            crate::lineage::Lineage::of(face, &self.fragments[usize::from(side == Operand::B)]);
        if lineage.cells().nth(1).is_none() {
            return self.live(body, side, view, Cell::Face(face));
        }
        let mut pieces = Vec::new();
        for f in lineage.cells() {
            if let Some(Cell::Face(p)) = self.live(body, side, view, Cell::Face(f))?
                && !pieces.contains(&p)
            {
                pieces.push(p);
            }
        }
        if let Some(&p) = pieces
            .iter()
            .find(|&&p| super::records::incident(body, [Cell::Vertex(vertex), Cell::Face(p)]))
        {
            return Ok(Some(Cell::Face(p)));
        }
        let q = body
            .get_vertex(vertex)
            .and_then(|d| body.get_point(d.point))
            .copied()
            .ok_or_else(|| desync("a recorded vertex has no point"))?;
        let (mut held, mut unread) = (None, None);
        for p in pieces {
            match piece_holds(body, p, q, band)? {
                PieceRead::In => {
                    if held.replace(p).is_some() {
                        return Err(desync(
                            "two pieces of one divided face hold a recorded vertex",
                        ));
                    }
                }
                PieceRead::NotIn => {}
                PieceRead::Unread(refusal) => {
                    unread.get_or_insert(BooleanError::RestPieceUnread { face: p, refusal });
                }
            }
        }
        match (held, unread) {
            (Some(p), _) => Ok(Some(Cell::Face(p))),
            (None, Some(refused)) => Err(refused),
            (None, None) => Ok(None),
        }
    }

    /// [`Self::live`] without the chord rows.
    fn chained<T: Real>(
        &self,
        body: &Body<T>,
        view: &KeyView<'_>,
        cell: Cell,
    ) -> Result<Option<Cell>, BooleanError> {
        let start = match cell {
            Cell::Vertex(v) => view
                .vertex(v)
                .map(|k| Cell::Vertex(self.vertices.survivor(k))),
            Cell::Edge(e) => view.edge(e).map(Cell::Edge),
            Cell::Face(f) => view.face(f).map(Cell::Face),
        };
        let Some(at) = start else {
            return Ok(None);
        };
        let step = |at: Cell| {
            match at {
                Cell::Face(f) => self.faces.get(&f).map(|&k| Cell::Face(k)),
                _ => None,
            }
            .or_else(|| self.cells.get(&at).copied())
        };
        match chase_live(body, at, self.faces.len() + self.cells.len(), step) {
            Ok(reached) => Ok(reached.ok()),
            Err(RowsCycle) => Err(BooleanError::JoinDesync {
                what: "a cell's substitution rows are cyclic",
            }),
        }
    }
}

/// What [`piece_holds`] read of one piece.
#[derive(Debug)]
enum PieceRead {
    /// The point is strictly inside the piece.
    In,
    /// The point is off the piece's carrier, outside its region, or on
    /// its boundary, where it meets an edge or vertex that a record of
    /// its own names.
    NotIn,
    /// The read failed: the containment door's refusal, or `None` where
    /// the curved chart trim gives no verdict.
    Unread(Option<ContainError>),
}

/// Whether `face` holds `q` strictly inside it, placed as the census
/// confirms a vertex-on-face record ([`super::contain::face_placement`]).
///
/// # Errors
///
/// [`BooleanError::Escalated`] (`SelfCheck(CarriedLineage)`) on a margin
/// in band, the on-plane one or the door's.
fn piece_holds<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    q: Point3<T>,
    band: Band,
) -> Result<PieceRead, BooleanError> {
    use super::contain::{CurvedPlacement, PlacementError};
    let in_band = |diag| BooleanError::Escalated {
        decision: BooleanDecision::SelfCheck(SelfCheck::CarriedLineage),
        diag,
    };
    match super::contain::face_placement(body, face, q, band, "bool_carry_vf_on_plane") {
        Ok(CurvedPlacement::Trim(Some(FaceContainment::In))) => Ok(PieceRead::In),
        Ok(
            CurvedPlacement::OffCarrier
            | CurvedPlacement::Trim(Some(
                FaceContainment::Out | FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_),
            )),
        ) => Ok(PieceRead::NotIn),
        Ok(CurvedPlacement::Trim(None)) => Ok(PieceRead::Unread(None)),
        Err(
            PlacementError::OnPlane(diag)
            | PlacementError::Door(ContainError::Escalated { diag, .. }),
        ) => Err(in_band(diag)),
        Err(PlacementError::Door(ContainError::StaleFace(face))) => {
            super::contain::driver_face_stale(face)
        }
        // Listed rather than wildcarded: a new door refusal is routed
        // here deliberately.
        Err(PlacementError::Door(
            refusal @ (ContainError::RayExhausted
            | ContainError::EmptyLoop(_)
            | ContainError::LoopUnreadable(_)
            | ContainError::Uncrossable(_)
            | ContainError::Curved(_)
            | ContainError::ArcEndClearOfCorners(_)),
        )) => Ok(PieceRead::Unread(Some(refusal))),
    }
}

/// **Edge-split lineage**: each operand's carried records, with every
/// `(vertex, edge)` row moved onto the piece of its edge the vertex
/// rests on. The reduction's split log ([`super::EdgeSplit`]) says
/// which pieces an edge became, in split order; at each split of the
/// piece the row names, the split's own vertex decides the side: the
/// parent keeps the span from its start to that vertex, so the row is a
/// vertex-vertex row with the split vertex when its vertex is at it,
/// stays on the parent when its vertex lies on the parent's span, and
/// moves to the child otherwise. Each is asked as the census asks it
/// ([`crate::census::on_segment_interior`]), so what lands is what the
/// census confirms. Every edge-edge
/// row is placed by [`ee_lineage`]. Nothing searches the result for an
/// edge the vertex lies on.
///
/// # Errors
///
/// [`BooleanError::Escalated`]: `VertexOnVertex` where a vertex is in
/// band of a split vertex, `SelfCheck(CarriedLineage)` where a span
/// question is; [`BooleanError::JoinDesync`] where a logged key does
/// not resolve in its clone.
pub(super) fn ledger<T: Decide>(
    red: &BooleanReduction<T>,
    decls: &Glue,
    band: Band,
) -> Result<Ledger, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut out = [
        Rows::of(&decls.records_a, Operand::A),
        Rows::of(&decls.records_b, Operand::B),
    ];
    let mut pending = red.pending.clone();
    for ((side, clone), carried) in [(Operand::A, &red.a), (Operand::B, &red.b)]
        .into_iter()
        .zip(&mut out)
    {
        let point = |v: VertexKey| {
            clone
                .get_vertex(v)
                .and_then(|d| clone.get_point(d.point))
                .copied()
                .ok_or_else(|| desync("a split row's vertex does not resolve in its clone"))
        };
        for row in core::mem::take(&mut carried.ve) {
            let (vertex, mut edge) = (row.vertex, row.edge);
            let mut at = None;
            for split in red.edge_splits.iter().filter(|s| s.operand == side) {
                // A row moved to a child reads only that child's later
                // splits.
                if split.parent != edge {
                    continue;
                }
                let start = clone
                    .get_edge(split.parent)
                    .and_then(|e| clone.get_half_edge(e.he_plus))
                    .map(|h| h.start)
                    .ok_or_else(|| desync("a split parent does not resolve in its clone"))?;
                let (p_start, p_split, p_vertex) =
                    (point(start)?, point(split.vertex)?, point(vertex)?);
                // The census's own questions ([`ee_lineage`]'s): at the
                // split vertex, else on the parent's span from its start
                // to the split, else past it, on the child.
                let gap = Margin::norm3(p_vertex - p_split);
                let (apart, margin) = geom_core::k_stats::decide_magnitude_reported(
                    "bool_carried_ve_split_vertex",
                    gap,
                    band,
                )
                .map_err(|diag| BooleanError::Escalated {
                    decision: BooleanDecision::VertexOnVertex,
                    diag,
                })?;
                if apart == geom_core::k_stats::Magnitude::Zero {
                    at = Some((split.vertex, margin));
                    break;
                }
                let stays = crate::census::on_segment_interior(p_vertex, (p_start, p_split), band)
                    .map_err(|diag| BooleanError::Escalated {
                        decision: BooleanDecision::SelfCheck(SelfCheck::CarriedLineage),
                        diag,
                    })?;
                if !stays {
                    edge = split.child;
                }
            }
            match at {
                // The carried vertex decided one with the vertex the
                // split minted: a decision of this op's, cited beside the
                // carried record.
                Some((w, margin)) => {
                    let fresh = decide_one(
                        &mut pending,
                        (side, Cell::Vertex(vertex)),
                        (side, Cell::Vertex(w)),
                        margin,
                    );
                    carried.vv.push(Cited::new(
                        VvContact { a: vertex, b: w },
                        row.cites.union(&fresh),
                    ));
                }
                None => carried
                    .ve
                    .push(Cited::new(VeContact { vertex, edge }, row.cites)),
            }
        }
        for row in core::mem::take(&mut carried.ee) {
            ee_lineage(
                clone,
                side,
                &red.edge_splits,
                &red.null_edges,
                row,
                carried,
                &mut pending,
                band,
            )?;
        }
    }
    // Two pierces one ring serves, on two point keys: the ring joined
    // them by a value decision. It is this op's, cited by a vertex-vertex
    // record of the piercing operand's, so the carry records every pair
    // of result cells the two reach. A pair on one point key is
    // structure, and nothing is recorded.
    for tie in &red.ring_ties {
        let clone = match tie.operand {
            Operand::A => &red.a,
            Operand::B => &red.b,
        };
        let key = |v: VertexKey| {
            clone
                .get_vertex(v)
                .map(|d| d.point)
                .ok_or_else(|| desync("a ring tie's vertex does not resolve in its clone"))
        };
        if key(tie.a)? == key(tie.b)? {
            continue;
        }
        let fresh = decide_one(
            &mut pending,
            (tie.operand, Cell::Vertex(tie.a)),
            (tie.operand, Cell::Vertex(tie.b)),
            tie.margin,
        );
        out[usize::from(tie.operand == Operand::B)]
            .vv
            .push(Cited::new(VvContact { a: tie.a, b: tie.b }, fresh));
    }
    Ok(Ledger {
        discovered: red.contacts.clone(),
        carried: out,
        pending,
        splits: red.edge_splits.clone(),
        recorded: red.coincidences.clone(),
    })
}

/// Records a decision `first` and `second` are one, by `margin`, as a
/// pending row ([`super::reduce::pend`]), and the citation of it.
fn decide_one(
    pending: &mut Vec<PendingRow>,
    first: End,
    second: End,
    margin: geom_core::MarginDiag,
) -> Cites {
    let k = super::reduce::pend(pending, PendingRow::on(first, second, Some(margin)));
    Cites::decided(super::reduce::index(k))
}

/// Pushes `record` onto `list`, or where a record of the same cells is
/// already there (`same`), adds `record`'s citations to it.
fn push_cited<R>(list: &mut Vec<Cited<R>>, record: Cited<R>, same: impl Fn(&R, &R) -> bool) {
    match list.iter_mut().find(|r| same(&r.record, &record.record)) {
        Some(r) => r.cites = r.cites.union(&record.cites),
        None => list.push(record),
    }
}

/// **Everything a result's records are carried from**: the reduction's
/// own records, each operand's carried records through the reduction's
/// splits ([`ledger`]), and every decision a record may cite, in
/// decision order.
pub(super) struct Ledger {
    /// The reduction's working set, citing rows of `pending`.
    discovered: ReductionContacts,
    /// Each operand's carried records, through the reduction's splits.
    carried: [Rows; 2],
    /// The reduction's decisions, then the lineage's.
    pending: Vec<PendingRow>,
    /// The reduction's edge splits, which read a vertex it minted back
    /// to the input edge it was minted on.
    splits: Vec<super::EdgeSplit>,
    /// The coincidences the op recorded before its records were carried.
    recorded: Vec<crate::Coincidence>,
}

/// One piece of a split edge: its key and its two end vertices.
type Piece = (EdgeKey, VertexKey, VertexKey);

/// `edge`'s pieces after the reduction's splits ([`super::EdgeSplit`]),
/// with every vertex a split minted on it: a split keeps the parent's
/// leading span, so its child is the piece right after the parent.
fn pieces<T: Real>(
    clone: &Body<T>,
    side: Operand,
    splits: &[super::EdgeSplit],
    edge: EdgeKey,
) -> Result<(Vec<Piece>, Vec<VertexKey>), BooleanError> {
    let mut keys = vec![edge];
    let mut minted = Vec::new();
    for split in splits.iter().filter(|s| s.operand == side) {
        if let Some(i) = keys.iter().position(|&k| k == split.parent) {
            keys.insert(i + 1, split.child);
            minted.push(split.vertex);
        }
    }
    let ends = |e: EdgeKey| {
        let edge = clone.get_edge(e)?;
        Some((
            e,
            clone.get_half_edge(edge.he_plus)?.start,
            clone.get_half_edge(edge.he_minus)?.start,
        ))
    };
    let pieces = keys
        .into_iter()
        .map(|e| {
            ends(e).ok_or(BooleanError::JoinDesync {
                what: "a split piece does not resolve in its clone",
            })
        })
        .collect::<Result<_, _>>()?;
    Ok((pieces, minted))
}

/// **Edge-split lineage of an edge-edge record**: where the reduction
/// split either of its edges, the record lands on every pair of pieces,
/// one from each edge, whose interiors still meet, and each vertex a
/// split minted on the contact is recorded against the other edge's
/// piece it rests on (or the vertex it meets). An edge-edge record
/// names no vertex, so where the contact lies is decided against the
/// pieces themselves, on the census's own span and segment predicates
/// ([`crate::census`]): the span first, since a short piece's line
/// extrapolated along a long one reads a gap the piece does not have,
/// then the segment question on either piece's line, the first decided
/// answer. Only the record's own lineage is read: its two edges'
/// pieces, never another cell of the body. A bound the record already
/// had is a vertex record of its own, carried by its own lineage, and is
/// not restated.
///
/// # Errors
///
/// [`BooleanError::Escalated`] (`VertexOnVertex`) where a segment
/// question is in band; [`BooleanError::JoinDesync`] where a logged key
/// does not resolve in its clone.
#[allow(clippy::too_many_arguments)]
fn ee_lineage<T: Decide>(
    clone: &Body<T>,
    side: Operand,
    splits: &[super::EdgeSplit],
    null_edges: &[super::BoolNullEdgeRecord<T>],
    row: Cited<EeContact>,
    carried: &mut Rows,
    pending: &mut Vec<PendingRow>,
    band: Band,
) -> Result<(), BooleanError> {
    let (a, a_minted) = pieces(clone, side, splits, row.a)?;
    let (b, b_minted) = pieces(clone, side, splits, row.b)?;
    if a.len() == 1 && b.len() == 1 {
        carried.ee.push(row);
        return Ok(());
    }
    let lineage = |diag| BooleanError::Escalated {
        decision: BooleanDecision::SelfCheck(SelfCheck::CarriedLineage),
        diag,
    };
    let coincide = |diag| BooleanError::Escalated {
        decision: BooleanDecision::VertexOnVertex,
        diag,
    };
    let point = |v: VertexKey| {
        clone
            .get_vertex(v)
            .and_then(|d| clone.get_point(d.point))
            .copied()
            .ok_or(BooleanError::JoinDesync {
                what: "a split piece's vertex does not resolve in its clone",
            })
    };
    let segment = |&(_, v0, v1): &Piece| Ok::<_, BooleanError>((point(v0)?, point(v1)?));
    for pa in &a {
        for pb in &b {
            let (sa, sb) = (segment(pa)?, segment(pb)?);
            let meet = || {
                crate::census::segment_interiors_meet(sa, sb, band)
                    .or_else(|_| crate::census::segment_interiors_meet(sb, sa, band))
                    .map_err(lineage)
            };
            if !crate::census::segments_apart(sa, sb, band)
                && !crate::census::segments_apart(sb, sa, band)
                && meet()?
            {
                push_cited(
                    &mut carried.ee,
                    Cited::new(EeContact { a: pa.0, b: pb.0 }, row.cites.clone()),
                    |r, c| (r.a, r.b) == (c.a, c.b),
                );
            }
            // A minted vertex of one piece, against the other piece.
            for (mine, minted, theirs, their_seg) in
                [(pa, &a_minted, pb, sb), (pb, &b_minted, pa, sa)]
            {
                for v in [mine.1, mine.2].into_iter().filter(|v| minted.contains(v)) {
                    let q = point(v)?;
                    // Past the other piece's span the vertex is on no
                    // part of it, and its line's reading there is the
                    // carrier's, not the piece's.
                    if !crate::census::past_segment(q, their_seg, band)
                        && crate::census::on_segment_interior(q, their_seg, band)
                            .map_err(lineage)?
                    {
                        push_cited(
                            &mut carried.ve,
                            Cited::new(
                                VeContact {
                                    vertex: v,
                                    edge: theirs.0,
                                },
                                row.cites.clone(),
                            ),
                            |r, c| r == c,
                        );
                        continue;
                    }
                    for w in [theirs.1, theirs.2] {
                        if w == v {
                            continue;
                        }
                        let gap = Margin::norm3(q - point(w)?);
                        let (apart, margin) = geom_core::k_stats::decide_magnitude_reported(
                            "bool_carried_ee_split_vertex",
                            gap,
                            band,
                        )
                        .map_err(coincide)?;
                        if apart == geom_core::k_stats::Magnitude::Zero {
                            // The minted vertex decided one with the other
                            // piece's end: this op's decision, cited
                            // beside the carried record, naming the
                            // vertex the end is a null-edge copy of.
                            let fresh = decide_one(
                                pending,
                                (side, Cell::Vertex(v)),
                                (side, Cell::Vertex(uncopied(null_edges, side, w)?)),
                                margin,
                            );
                            push_cited(
                                &mut carried.vv,
                                Cited::new(VvContact { a: v, b: w }, row.cites.union(&fresh)),
                                |r, c| (r.a, r.b) == (c.a, c.b) || (r.a, r.b) == (c.b, c.a),
                            );
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// The vertex `v` of operand `side`'s clone is a copy of: where a null
/// edge minted `v` at another vertex, that vertex, read back through
/// every null edge that hangs at a copy; `v` itself where none did. A
/// decision's cells name the op's input, which holds no copy.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] where the copies' null edges close a
/// cycle, so no vertex is the original.
fn uncopied<T: Real>(
    null_edges: &[super::BoolNullEdgeRecord<T>],
    side: Operand,
    mut v: VertexKey,
) -> Result<VertexKey, BooleanError> {
    for _ in 0..=null_edges.len() {
        let minted = null_edges.iter().find(|r| {
            r.operand == side
                && r.at_vertex != v
                && (r.attr.below_end == v || r.attr.above_end == v)
        });
        match minted {
            Some(r) => v = r.at_vertex,
            None => return Ok(v),
        }
    }
    Err(BooleanError::JoinDesync {
        what: "a vertex's null-edge copies close a cycle with no original",
    })
}

/// One operand's own records re-entering an op, in its keys: the cell
/// pairs alone, each citing the operand's record it is
/// ([`Backing::Carried`]). A carried row's class is the recipe's and
/// stays there; what an op carries through its cells is the record.
#[derive(Clone, Debug, Default)]
pub(super) struct Rows {
    vv: Vec<Cited<VvContact>>,
    vf: Vec<Cited<VfContact>>,
    ve: Vec<Cited<VeContact>>,
    ee: Vec<Cited<EeContact>>,
}

impl Rows {
    pub(super) fn of(carried: &OperandRecords, side: Operand) -> Self {
        let OperandRecords { vv, vf, ve, ee } = carried;
        let input = u32::from(side == Operand::B);
        let cite = |record| Cites::one(Backing::Carried { input, record });
        Self {
            vv: vv
                .iter()
                .map(|c| Cited::new(c.contact, cite(c.record)))
                .collect(),
            vf: vf
                .iter()
                .map(|c| Cited::new(c.contact, cite(c.record)))
                .collect(),
            ve: ve
                .iter()
                .map(|c| Cited::new(c.contact, cite(c.record)))
                .collect(),
            ee: ee
                .iter()
                .map(|c| Cited::new(c.contact, cite(c.record)))
                .collect(),
        }
    }
}

/// One end of a record entering [`carry`]: the arena it is keyed in
/// and the cell.
type End = (Operand, Cell);

/// A record entering [`carry`], operand-tagged: its two ends and what
/// it cites.
type Tagged<'a> = ([End; 2], &'a Cites);

/// The reduction's working set as tagged rows
/// ([`ReductionContacts::rows`]).
fn tagged(d: &ReductionContacts) -> Vec<Tagged<'_>> {
    d.rows().collect()
}

/// **The substitution door**: every contact record an op holds,
/// discovered or carried, in operand keys, rewritten into result keys
/// through the operand views and the descendant map. A record whose
/// cell an op replaced now names the replacement: a fused vertex its
/// survivor, an absorbed face its kept face, a zipped ring edge its
/// seam edge, a cell the merge swallowed the face it lies in, a joined
/// vertex or edge the edge the join made, an edge the op dropped the
/// chord the join minted along it ([`Descendants::live`]); a face the
/// op divided, the piece of it that holds the record's vertex
/// ([`Descendants::holder`]). A record is never re-derived from
/// geometry: a division only chooses among the recorded face's own
/// pieces. It leaves only where its cell left the result with no such
/// row (a discarded fragment), where its vertex rests on no live piece
/// of the face it named or on a piece's boundary, or where its two cells
/// became one or incident, which is structure ([`record`]).
///
/// **Vertex records are carried as groups.** Records that share a
/// vertex end, discovered and carried alike, are one group, closed
/// transitively: every cell in it lies at one point, the shared
/// key's. A vertex end stands for itself and its null-edge copies, and
/// an edge end a `(vertex, edge)` record attaches to its vertex's group.
/// Every two distinct, non-incident cells the group's ends map to are
/// recorded, though no single record named that pair: what the ops'
/// own coincidences imply, read from keys and never from positions.
/// A pair landing on two edges is an edge-edge record (two joined
/// vertices that coincided: the edges cross or overlap there), and one
/// landing on an edge and a face an edge-face record.
///
/// **A vertex a zip fused** sits on the seam, on the boundary of the
/// face a vertex-on-face record named: the substituted pair is an
/// incidence, and it collapses as one, read off the fusion: the zip's
/// own divisions are in no row of [`Descendants::fragments`].
///
/// **Every record cites the decisions that back it** (D1 (ii)). A
/// record one input record became cites what that record cited; a pair
/// a group implies cites every record on every shortest chain of
/// records joining its two cells in the group, so no chain is picked
/// over another; a pair of two copies of one vertex cites the records
/// that put the vertex in its group. Each decision a surviving record
/// cites is emitted, in decision order, after the op's own
/// coincidences, and the records are renumbered onto those rows. So is
/// each decision at a pinch the op made ([`pinch_rows`]), read only
/// for a result the seams were zipped into (`zipped`): the op's copies
/// there touch on one point key, which no record states. Any other
/// decision placed topology and is not a coincidence (D1). A surviving
/// record whose decisions all decided no margin (a transverse pierce)
/// is a touch without a decision, which the op refuses rather than
/// ship.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] on a corrupt fusion list or cycling
/// substitution rows ([`Descendants::live`]);
/// [`BooleanError::ClassificationInvariant`] on a surviving record no
/// decision backs.
pub(super) fn carry<T: Decide>(
    body: &Body<T>,
    ledger: &Ledger,
    views: [&KeyView<'_>; 2],
    desc: &Descendants,
    zipped: Option<Zipped<'_, T>>,
    band: Band,
) -> Result<(ContactRecords, Vec<crate::Coincidence>), BooleanError> {
    let [a, b] = &ledger.carried;
    let mut records = carry_rows(
        body,
        &tagged(&ledger.discovered),
        [a, b],
        views,
        desc,
        false,
        band,
    )?;
    record_spans(body, &mut records, band)?;
    let pinched = match zipped {
        Some(z) => pinch_rows(body, z.operands, ledger, views, desc)?,
        None => Vec::new(),
    };
    cite_rows(records, ledger, &pinched)
}

/// The result's vertices an operand's carried vertex-vertex records
/// name, read as [`carry`] reads them (each vertex's copies, chased
/// live), before the output stage's join: the vertices whose touch a
/// record of the result will name though the join runs an edge
/// through it (`edge_join::free_vertices`).
///
/// # Errors
///
/// As [`Descendants::live`].
fn carried_vertices<T: Real>(
    body: &Body<T>,
    ledger: &Ledger,
    views: [&KeyView<'_>; 2],
    desc: &Descendants,
) -> Result<BTreeSet<VertexKey>, BooleanError> {
    let mut out = BTreeSet::new();
    for ((side, rows), view) in [Operand::A, Operand::B]
        .into_iter()
        .zip(&ledger.carried)
        .zip(views)
    {
        for c in &rows.vv {
            for v in [c.a, c.b] {
                for k in desc.copies_of(side, v) {
                    if let Some(Cell::Vertex(x)) = desc.live(body, side, view, Cell::Vertex(k))? {
                        out.insert(x);
                    }
                }
            }
        }
    }
    Ok(out)
}

/// **A result the seams were zipped into**, with its two operands: the
/// only path that leaves an op's copies of one vertex on one point key
/// (the join's null-edge copies, the zip's split per cone), so the only
/// one [`carry`] reads pinches for. It holds the zip's
/// [`super::zip::ConesSplit`], which only `zip::split_cones` builds.
pub(super) struct Zipped<'a, T: Real> {
    operands: [&'a Body<T>; 2],
    _cones: super::zip::ConesSplit,
}

/// **The decisions at the pinches the op made**, as indices into the
/// ledger's pending rows, ascending: each decided row with a vertex end
/// whose live descendant shares its point key with another live vertex,
/// where not every vertex on that key descends from an operand vertex
/// that already shared its key. Two vertices on one point key are an
/// op's copies of one vertex: the join's null-edge copies
/// ([`Descendants::copies_of`]) or the zip's split per cone
/// (`zip::split_cones`), so pieces of the result touch there with no
/// record between them (D1's structural sharing), and the row that put
/// the vertex on the other operand's cell is the coincidence that made
/// them touch (DESIGN, the reduction's coincidence discipline). Copies
/// an operand already held are its own structure, which no decision of
/// this op made.
fn pinch_rows<T: Real>(
    body: &Body<T>,
    operands: [&Body<T>; 2],
    ledger: &Ledger,
    views: [&KeyView<'_>; 2],
    desc: &Descendants,
) -> Result<Vec<u32>, BooleanError> {
    let mut inherited: BTreeSet<VertexKey> = BTreeSet::new();
    for (side, operand) in [Operand::A, Operand::B].into_iter().zip(operands) {
        let mut shared: BTreeMap<crate::geometry::PointKey, usize> = BTreeMap::new();
        for (_, v) in operand.vertices() {
            *shared.entry(v.point).or_default() += 1;
        }
        let view = views[usize::from(side == Operand::B)];
        for (u, v) in operand.vertices() {
            if shared[&v.point] >= 2
                && let Some(Cell::Vertex(w)) = desc.live(body, side, view, Cell::Vertex(u))?
            {
                inherited.insert(w);
            }
        }
    }
    let mut held: BTreeMap<crate::geometry::PointKey, (usize, bool)> = BTreeMap::new();
    for (w, v) in body.vertices() {
        let at = held.entry(v.point).or_default();
        at.0 += 1;
        at.1 |= !inherited.contains(&w);
    }
    let pinch = |w: VertexKey| {
        body.get_vertex(w)
            .is_some_and(|v| held.get(&v.point).is_some_and(|&(n, made)| n >= 2 && made))
    };
    let mut out = Vec::new();
    for (k, row) in ledger.pending.iter().enumerate() {
        if row.margin.is_none() {
            continue;
        }
        let mut at_pinch = false;
        for &(side, cell) in &row.cells {
            let Cell::Vertex(v) = cell else { continue };
            let view = views[usize::from(side == Operand::B)];
            for copy in desc.copies_of(side, v) {
                if let Some(Cell::Vertex(w)) = desc.live(body, side, view, Cell::Vertex(copy))? {
                    at_pinch |= pinch(w);
                }
            }
        }
        if at_pinch {
            out.push(super::reduce::index(k));
        }
    }
    Ok(out)
}

/// **The output stage's one key rule**: the vertices of every
/// vertex-vertex record of the result move onto one point key
/// (`zip::record_classes`), on every path a boolean builds by. Run
/// after [`carry`], on the records the result ships.
///
/// # Errors
///
/// As `zip::record_classes` and `zip::share_points`.
fn share_recorded_points<T: Decide>(
    body: &mut Body<T>,
    contacts: &ContactRecords,
    coincidences: &[crate::Coincidence],
) -> Result<(), BooleanError> {
    let classes = super::zip::record_classes(body, contacts, coincidences)?;
    super::zip::share_points(body, &classes)
}

/// The records `carry_rows` left, their decisions and `pinched`'s
/// emitted after the op's own coincidences and the records' citations
/// renumbered onto them ([`carry`]).
fn cite_rows(
    records: ContactRecords,
    ledger: &Ledger,
    pinched: &[u32],
) -> Result<(ContactRecords, Vec<crate::Coincidence>), BooleanError> {
    let decided = records.decided();
    if decided
        .last()
        .is_some_and(|&k| k as usize >= ledger.pending.len())
    {
        return Err(BooleanError::JoinDesync {
            what: "a contact record cites a decision the reduction did not make",
        });
    }
    // A decision with no margin (a transverse pierce's landing) is not
    // a coincidence, and a record citing only such is refused below.
    let mut used: Vec<u32> = decided
        .into_iter()
        .filter(|&k| ledger.pending[k as usize].margin.is_some())
        .chain(pinched.iter().copied())
        .collect();
    used.sort_unstable();
    used.dedup();
    let mut coincidences = ledger.recorded.clone();
    let base = coincidences.len();
    for &k in &used {
        let row = ledger.pending[k as usize];
        let Some(margin) = row.margin else {
            unreachable!("only a decided row is emitted")
        };
        coincidences.push(crate::Coincidence {
            cells: row.cells.map(|end| input_cell(end, &ledger.splits)),
            relation: row.relation,
            site: crate::DecisionSite::VertexFusion,
            margin,
            discharge: crate::Discharge::of(margin),
        });
    }
    let records = records
        .renumbered(|k| {
            used.binary_search(&k)
                .ok()
                .map(|rank| super::reduce::index(base + rank))
        })
        .map_err(|_| BooleanError::ClassificationInvariant {
            what: "a contact record surviving into the result cites no decision",
        })?;
    Ok((records, coincidences))
}

/// A pending row's cell as the deciding op's input holds it: a vertex
/// the reduction minted by splitting an edge is read as the input edge
/// it was minted on, through every split of that edge's pieces.
pub(super) fn input_cell((input, cell): End, splits: &[super::EdgeSplit]) -> crate::RowCell {
    let root = |mut edge: EdgeKey| {
        while let Some(s) = splits
            .iter()
            .find(|s| s.operand == input && s.child == edge)
        {
            edge = s.parent;
        }
        edge
    };
    let cell = match cell {
        Cell::Vertex(v) => splits
            .iter()
            .find(|s| s.operand == input && s.vertex == v)
            .map_or(cell, |s| Cell::Edge(root(s.parent))),
        Cell::Edge(e) => Cell::Edge(root(e)),
        // The sweep splits edges and never a face: every pending row is
        // written before the insertion mints a face, so a face key in the
        // clone is the input's.
        Cell::Face(_) => cell,
    };
    crate::RowCell::Input { input, cell }
}

/// [`carry`] for records already keyed in the body's own arena, through
/// an op that replaced cells in place (the join): one arena, so two
/// records naming one vertex on either side are one group. Citations
/// are carried as given.
///
/// # Errors
///
/// As [`carry`].
#[cfg(test)]
pub(super) fn carry_in_place<T: Decide>(
    body: &Body<T>,
    records: &ContactRecords,
    desc: &Descendants,
    band: Band,
) -> Result<ContactRecords, BooleanError> {
    let none = Rows::default();
    let rows: Vec<Tagged<'_>> = records
        .rows()
        .iter()
        .map(|c| {
            let [x, y] = c.cells();
            ([(Operand::A, x), (Operand::A, y)], &c.cites)
        })
        .collect();
    carry_rows(
        body,
        &rows,
        [&none, &none],
        [&KeyView::Direct, &KeyView::Direct],
        desc,
        true,
        band,
    )
}

/// A node of a vertex group's graph ([`carry_rows`]): a vertex end, by
/// its index among the group's ends, or the edge end of a
/// `(vertex, edge)` row, by the row's index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Node {
    End(usize),
    VeEdge(usize),
}

/// The vertex groups' graph: one link per vertex record (between its two
/// ends) and per `(vertex, edge)` record (between its vertex end and
/// its edge end), each with the record's citations.
struct Chains {
    links: Vec<(Node, Node, Cites)>,
}

impl Chains {
    /// Hops from `from` to every node it reaches.
    fn hops(&self, from: &[Node]) -> std::collections::BTreeMap<Node, usize> {
        let mut dist: std::collections::BTreeMap<Node, usize> =
            from.iter().map(|&n| (n, 0)).collect();
        let mut frontier: Vec<Node> = from.to_vec();
        while !frontier.is_empty() {
            let mut next = Vec::new();
            for n in frontier {
                let d = dist[&n] + 1;
                for &(x, y, _) in &self.links {
                    let other = if x == n {
                        y
                    } else if y == n {
                        x
                    } else {
                        continue;
                    };
                    if let std::collections::btree_map::Entry::Vacant(slot) = dist.entry(other) {
                        slot.insert(d);
                        next.push(other);
                    }
                }
            }
            frontier = next;
        }
        dist
    }

    /// What a pair joined from `xs` and `ys` cites: every link on every
    /// shortest chain between them, or, where they share a node (two
    /// copies of one vertex), every link at a shared node.
    fn cites(&self, xs: &[Node], ys: &[Node]) -> Option<Cites> {
        let shared: Vec<Node> = xs.iter().filter(|n| ys.contains(n)).copied().collect();
        if !shared.is_empty() {
            return Cites::of(
                self.links
                    .iter()
                    .filter(|(x, y, _)| shared.contains(x) || shared.contains(y))
                    .flat_map(|(_, _, c)| c.iter()),
            );
        }
        let (dx, dy) = (self.hops(xs), self.hops(ys));
        let length = ys.iter().filter_map(|n| dx.get(n)).min()?;
        let on_chain = |u: &Node, v: &Node| matches!((dx.get(u), dy.get(v)), (Some(a), Some(b)) if a + 1 + b == *length);
        Cites::of(
            self.links
                .iter()
                .filter(|(x, y, _)| on_chain(x, y) || on_chain(y, x))
                .flat_map(|(_, _, c)| c.iter()),
        )
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod chain_rows {
    use super::{Chains, Node};
    use crate::{Backing, Cites};

    fn link(x: usize, y: usize, k: u32) -> (Node, Node, Cites) {
        (Node::End(x), Node::End(y), Cites::decided(k))
    }

    fn cited(c: Option<Cites>) -> Vec<u32> {
        c.expect("a chain")
            .iter()
            .map(|b| match b {
                Backing::Decided(k) => k,
                Backing::Carried { .. } => unreachable!("decided links only"),
            })
            .collect()
    }

    /// Two ends each decided one with a third: the pair the group
    /// implies cites both decisions, and a longer detour cites nothing.
    /// Red if one link of the chain is dropped, or the detour is taken.
    #[test]
    fn a_pair_two_decisions_imply_cites_both() {
        let chains = Chains {
            links: vec![
                link(0, 2, 0),
                link(1, 2, 1),
                link(0, 3, 2),
                link(3, 4, 3),
                link(4, 1, 4),
            ],
        };
        assert_eq!(
            cited(chains.cites(&[Node::End(0)], &[Node::End(1)])),
            [0, 1]
        );
    }

    /// Two shortest chains of one length between the pair: the record
    /// cites both, whichever was listed first. Red if a tie is broken by
    /// list order (one chain's links are missing).
    #[test]
    fn tied_chains_are_cited_together() {
        let one = Chains {
            links: vec![link(0, 2, 0), link(2, 1, 1), link(0, 3, 2), link(3, 1, 3)],
        };
        let other = Chains {
            links: vec![link(0, 3, 2), link(3, 1, 3), link(0, 2, 0), link(2, 1, 1)],
        };
        let ends = |c: &Chains| cited(c.cites(&[Node::End(0)], &[Node::End(1)]));
        assert_eq!(ends(&one), [0, 1, 2, 3]);
        assert_eq!(
            ends(&other),
            ends(&one),
            "the order of the links is not read"
        );
    }

    /// Two copies of one vertex share their node: the pair cites the
    /// records that put the vertex in its group.
    #[test]
    fn copies_of_one_vertex_cite_the_vertexs_records() {
        let chains = Chains {
            links: vec![link(0, 1, 5), link(2, 3, 6)],
        };
        assert_eq!(cited(chains.cites(&[Node::End(0)], &[Node::End(0)])), [5]);
    }
}

/// The door's one body, over records in two arenas (`one_arena`
/// false, the operands') or one.
fn carry_rows<T: Decide>(
    body: &Body<T>,
    discovered: &[Tagged<'_>],
    carried: [&Rows; 2],
    views: [&KeyView<'_>; 2],
    desc: &Descendants,
    one_arena: bool,
    band: Band,
) -> Result<ContactRecords, BooleanError> {
    let sides = [Operand::A, Operand::B];
    let view = |side: Operand| views[usize::from(side == Operand::B)];
    let mut out = ContactRecords::default();
    // The rows by the kinds of their two cells, each vertex end first.
    let mut vv_rows: Vec<Tagged<'_>> = Vec::new();
    let mut ve_rows: Vec<Tagged<'_>> = Vec::new();
    let mut vf_rows: Vec<Tagged<'_>> = Vec::new();
    let mut cell_rows: Vec<Tagged<'_>> = Vec::new();
    for &([x, y], cites) in discovered {
        match (x.1, y.1) {
            (Cell::Vertex(_), Cell::Vertex(_)) => vv_rows.push(([x, y], cites)),
            (Cell::Vertex(_), Cell::Edge(_)) => ve_rows.push(([x, y], cites)),
            (Cell::Edge(_), Cell::Vertex(_)) => ve_rows.push(([y, x], cites)),
            (Cell::Vertex(_), Cell::Face(_)) => vf_rows.push(([x, y], cites)),
            (Cell::Face(_), Cell::Vertex(_)) => vf_rows.push(([y, x], cites)),
            (Cell::Edge(_) | Cell::Face(_), Cell::Edge(_) | Cell::Face(_)) => {
                cell_rows.push(([x, y], cites));
            }
        }
    }
    for (side, rows) in sides.into_iter().zip(carried) {
        vv_rows.extend(rows.vv.iter().map(|c| {
            (
                [(side, Cell::Vertex(c.a)), (side, Cell::Vertex(c.b))],
                &c.cites,
            )
        }));
        ve_rows.extend(rows.ve.iter().map(|c| {
            (
                [(side, Cell::Vertex(c.vertex)), (side, Cell::Edge(c.edge))],
                &c.cites,
            )
        }));
        vf_rows.extend(rows.vf.iter().map(|c| {
            (
                [(side, Cell::Vertex(c.vertex)), (side, Cell::Face(c.face))],
                &c.cites,
            )
        }));
        cell_rows.extend(
            rows.ee
                .iter()
                .map(|c| ([(side, Cell::Edge(c.a)), (side, Cell::Edge(c.b))], &c.cites)),
        );
    }
    // Groups over vertex ends, closed transitively; `ends` holds each
    // end once, in the order the rows first name it.
    let mut ends: Vec<End> = Vec::new();
    let mut group: Vec<usize> = Vec::new();
    let arena = |(side, cell): End| (if one_arena { Operand::A } else { side }, cell);
    let slot = |end: End, ends: &mut Vec<End>, group: &mut Vec<usize>| {
        let end = arena(end);
        ends.iter().position(|&e| e == end).unwrap_or_else(|| {
            ends.push(end);
            group.push(group.len());
            ends.len() - 1
        })
    };
    let mut chains = Chains { links: Vec::new() };
    for ([x, y], cites) in &vv_rows {
        let (i, j) = (
            slot(*x, &mut ends, &mut group),
            slot(*y, &mut ends, &mut group),
        );
        chains
            .links
            .push((Node::End(i), Node::End(j), (*cites).clone()));
        let (gi, gj) = (group[i], group[j]);
        if gi != gj {
            group.iter_mut().filter(|g| **g == gi).for_each(|g| *g = gj);
        }
    }
    let mut attached: Vec<usize> = Vec::new();
    for (r, ([v, _], cites)) in ve_rows.iter().enumerate() {
        let i = slot(*v, &mut ends, &mut group);
        attached.push(i);
        chains
            .links
            .push((Node::End(i), Node::VeEdge(r), (*cites).clone()));
    }
    // Every group's live cells: its vertex ends with their copies, in
    // end order, then the edges attached to it; each with the nodes it
    // stands for. A result cell is one entry whichever side reaches it
    // (named by the first), so a pair it joins cites the shortest chain
    // from any of its nodes rather than each side's own.
    let mut live: Vec<(usize, End, Vec<Node>)> = Vec::new();
    let push = |live: &mut Vec<(usize, End, Vec<Node>)>,
                g: usize,
                side: Operand,
                c: Option<Cell>,
                node: Node| {
        if let Some(c) = c {
            match live.iter_mut().find(|(h, e, _)| (*h, e.1) == (g, c)) {
                Some((_, _, nodes)) => {
                    if !nodes.contains(&node) {
                        nodes.push(node);
                    }
                }
                None => live.push((g, (side, c), vec![node])),
            }
        }
    };
    for (i, &(side, cell)) in ends.iter().enumerate() {
        let Cell::Vertex(v) = cell else {
            continue;
        };
        for k in desc.copies_of(side, v) {
            push(
                &mut live,
                group[i],
                side,
                desc.live(body, side, view(side), Cell::Vertex(k))?,
                Node::End(i),
            );
        }
    }
    for (r, (&i, ([_, (side, e)], _))) in attached.iter().zip(&ve_rows).enumerate() {
        let c = desc.live(body, *side, view(*side), *e)?;
        push(&mut live, group[i], *side, c, Node::VeEdge(r));
    }
    for (i, (g, x, xs)) in live.iter().enumerate() {
        for (_, y, ys) in live[i + 1..].iter().filter(|(h, _, _)| h == g) {
            let Some(cites) = chains.cites(xs, ys) else {
                return Err(BooleanError::JoinDesync {
                    what: "two cells of one vertex group have no chain of records between them",
                });
            };
            record(body, &mut out, *x, *y, cites);
        }
    }
    // Vertex-on-face records: the vertex's own key, the face chased.
    for ([(vs, vc), (fs, fc)], cites) in vf_rows {
        // A vertex a zip fused sits on the seam, on the boundary of the
        // face it rested on: the substituted pair is an incidence.
        let Cell::Vertex(v) = vc else {
            unreachable!("a vertex-on-face row's first end is its vertex")
        };
        if view(vs).vertex(v).is_some_and(|k| desc.fused.contains(&k)) {
            continue;
        }
        let Some(x) = desc.live(body, vs, view(vs), vc)? else {
            continue;
        };
        let y = match (x, fc) {
            (Cell::Vertex(k), Cell::Face(f)) => desc.holder(body, fs, view(fs), f, k, band)?,
            _ => desc.live(body, fs, view(fs), fc)?,
        };
        if let Some(y) = y {
            record(body, &mut out, (vs, x), (fs, y), cites.clone());
        }
    }
    // Records of two edges or faces: the reduction's touches along a
    // line, the joins' and each operand's carried ones, both cells
    // chased.
    for ([(xs, xc), (ys, yc)], cites) in cell_rows {
        if let (Some(x), Some(y)) = (
            desc.live(body, xs, view(xs), xc)?,
            desc.live(body, ys, view(ys), yc)?,
        ) {
            record(body, &mut out, (xs, x), (ys, y), cites.clone());
        }
    }
    Ok(out)
}

/// **The spans a line edge's records bound** (D1: every pair whose
/// interiors meet is recorded, an edge lying in a face or along an edge
/// included). A record bounds a line edge where it names a point of it
/// ([`bound_reading`]); an edge some record bounds is also bounded by
/// its own ends, by structure ([`span_bounds`]). Between two
/// consecutive bounds the edge's span is recorded against each cell
/// both bounds touch that holds it ([`span_held`]), citing the records
/// that put each bound on that cell. A record naming a cell the body
/// does not hold bounds nothing; the census reads it stale.
///
/// # Errors
///
/// [`BooleanError::Escalated`] (`SelfCheck(CarriedLineage)`) where a
/// question is in band; [`BooleanError::RestPieceUnread`] where the
/// containment door cannot place a midpoint in a face.
fn record_spans<T: Decide>(
    body: &Body<T>,
    out: &mut ContactRecords,
    band: Band,
) -> Result<(), BooleanError> {
    let mut spans: Vec<Cited<Contact>> = Vec::new();
    for e in span_lines(body, out) {
        let bounds = span_bounds(body, out, e, band)?;
        for pair in bounds.windows(2) {
            let [lo, hi] = pair else { continue };
            for &c in lo.cells().iter().filter(|c| hi.cells().contains(c)) {
                let (Some(cites), Ok(contact)) = (
                    either_cites(lo.cites_on(c), hi.cites_on(c)),
                    Contact::new(body, Cell::Edge(e), c),
                ) else {
                    continue;
                };
                if span_held(body, (lo.point, hi.point), c, band)? {
                    spans.push(Cited::new(contact, cites));
                }
            }
        }
    }
    for span in spans {
        out.push(span);
    }
    Ok(())
}

/// The escalation of a question [`record_spans`] asks.
fn span_in_band(diag: Indeterminate) -> BooleanError {
    BooleanError::Escalated {
        decision: BooleanDecision::SelfCheck(SelfCheck::CarriedLineage),
        diag,
    }
}

/// Two optional citations, together.
fn either_cites(a: Option<Cites>, b: Option<Cites>) -> Option<Cites> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.union(&b)),
        (a, b) => a.or(b),
    }
}

/// The line edges a record names, or meets at one of their ends.
fn span_lines<T: Real>(body: &Body<T>, out: &ContactRecords) -> BTreeSet<EdgeKey> {
    let mut lines: BTreeSet<EdgeKey> = BTreeSet::new();
    for (x, y) in out.cell_pairs() {
        for c in [x, y] {
            match c {
                Cell::Edge(e) => {
                    lines.insert(e);
                }
                Cell::Vertex(v) => lines.extend(body.edges_of_vertex(v).unwrap_or_default()),
                Cell::Face(_) => {}
            }
        }
    }
    lines.retain(|&e| crate::census::edge_is_line(body, e));
    lines
}

/// A point where a line edge's span may begin or end: its parameter
/// along the edge, the point, and each reading that put it there: the
/// cells it touches there, with the citations of the record that read
/// it (none for the edge's own end, which bounds by structure).
struct Bound<T: Real> {
    at: T,
    point: Point3<T>,
    reads: Vec<(Vec<Cell>, Option<Cites>)>,
}

impl<T: Real> Bound<T> {
    /// Every cell some reading touches.
    fn cells(&self) -> Vec<Cell> {
        self.reads
            .iter()
            .flat_map(|(t, _)| t.iter().copied())
            .collect()
    }

    /// The citations of the readings that touch `c`.
    fn cites_on(&self, c: Cell) -> Option<Cites> {
        self.reads
            .iter()
            .filter(|(t, _)| t.contains(&c))
            .fold(None, |acc, (_, cites)| either_cites(acc, cites.clone()))
    }
}

/// Edge `e`'s bounds, in order along it, those at one point one bound:
/// a bound per record that names a point of it, and its two ends. None
/// where no record bounds it.
///
/// # Errors
///
/// [`BooleanError::Escalated`] where a reading or the order of two
/// bounds is in band.
fn span_bounds<T: Decide>(
    body: &Body<T>,
    out: &ContactRecords,
    e: EdgeKey,
    band: Band,
) -> Result<Vec<Bound<T>>, BooleanError> {
    let Some(((v0, v1), (p0, p1))) = line_ends(body, e)
        .and_then(|(v0, v1)| Some(((v0, v1), (vertex_point(body, v0)?, vertex_point(body, v1)?))))
    else {
        return Ok(Vec::new());
    };
    let dir = (p1 - p0).normalize();
    let mut readings = Vec::new();
    for c in out.rows() {
        if let Some((q, touches)) =
            bound_reading(body, (e, v0, v1, p0, p1), c.record.cells(), band)?
        {
            readings.push((q, touches, Some(c.cites.clone())));
        }
    }
    if readings.is_empty() {
        return Ok(Vec::new());
    }
    for (v, p) in [(v0, p0), (v1, p1)] {
        if let Some(touches) = vertex_star(body, v) {
            readings.push((p, touches, None));
        }
    }
    let mut bounds: Vec<Bound<T>> = Vec::new();
    for (q, touches, cites) in readings {
        let at = (q - p0).dot(dir);
        let mut place = bounds.len();
        let mut same = false;
        for (k, b) in bounds.iter().enumerate() {
            match decide("bool_span_bound_order", Margin::of(at - b.at), band)
                .map_err(span_in_band)?
            {
                Sign::Positive => {}
                Sign::Negative => {
                    place = k;
                    break;
                }
                Sign::Zero => {
                    (place, same) = (k, true);
                    break;
                }
            }
        }
        if same {
            bounds[place].reads.push((touches, cites));
        } else {
            bounds.insert(
                place,
                Bound {
                    at,
                    point: q,
                    reads: vec![(touches, cites)],
                },
            );
        }
    }
    Ok(bounds)
}

/// A point a record puts on a line edge, and the cells it touches
/// there.
type BoundReading<T> = (Point3<T>, Vec<Cell>);

/// Where a record of `cells` bounds the line edge `edge` (its key, end
/// vertices and their points), and the cells of the other side it
/// touches there: a vertex on the edge's interior, at its point; a
/// line edge crossing it, at the crossing ([`crate::census::segment_cross_point`]);
/// one of the edge's own ends touching a cell. A line edge along it is
/// the record of its own overlap, and bounds nothing.
///
/// # Errors
///
/// [`BooleanError::Escalated`] where the parallel test is in band.
fn bound_reading<T: Decide>(
    body: &Body<T>,
    (e, v0, v1, p0, p1): (EdgeKey, VertexKey, VertexKey, Point3<T>, Point3<T>),
    cells: [Cell; 2],
    band: Band,
) -> Result<Option<BoundReading<T>>, BooleanError> {
    Ok(match cells {
        [Cell::Edge(a), other] | [other, Cell::Edge(a)] if a == e => match other {
            Cell::Vertex(w) => vertex_point(body, w).zip(vertex_star(body, w)),
            Cell::Edge(g) if crate::census::edge_is_line(body, g) => {
                let Some(s) = line_ends(body, g)
                    .and_then(|(w0, w1)| Some((vertex_point(body, w0)?, vertex_point(body, w1)?)))
                else {
                    return Ok(None);
                };
                if crate::census::segments_parallel((p0, p1), s, band).map_err(span_in_band)? {
                    None
                } else {
                    edge_cells(body, g)
                        .map(|t| (crate::census::segment_cross_point((p0, p1), s), t))
                }
            }
            _ => None,
        },
        [Cell::Vertex(v), other] | [other, Cell::Vertex(v)] if v == v0 || v == v1 => {
            let touches = match other {
                Cell::Vertex(w) => vertex_star(body, w),
                Cell::Edge(g) => edge_cells(body, g),
                Cell::Face(f) => Some(vec![Cell::Face(f)]),
            };
            vertex_point(body, v).zip(touches)
        }
        _ => None,
    })
}

/// Whether cell `c` holds a line edge's span `(lo, hi)`: a plane
/// face whose interior holds its midpoint (two points of the plane hold
/// the line between them), placed by [`piece_holds`]. A curved face is
/// not read: a line between two of its points need not lie in it. An
/// edge along `e` is the reduction's own touch record
/// ([`super::touch_rows`]), not a span's.
///
/// # Errors
///
/// [`BooleanError::Escalated`] where a placement is in band;
/// [`BooleanError::RestPieceUnread`] where the containment door cannot
/// place the midpoint.
fn span_held<T: Decide>(
    body: &Body<T>,
    (lo, hi): (Point3<T>, Point3<T>),
    c: Cell,
    band: Band,
) -> Result<bool, BooleanError> {
    match c {
        Cell::Face(f)
            if matches!(
                body.get_face(f).and_then(|d| body.get_surface(d.surface)),
                Some(geom::Surface::Plane { .. })
            ) =>
        {
            match piece_holds(body, f, lo + (hi - lo) * T::from_f64(0.5), band)? {
                PieceRead::In => Ok(true),
                PieceRead::NotIn => Ok(false),
                PieceRead::Unread(refusal) => {
                    Err(BooleanError::RestPieceUnread { face: f, refusal })
                }
            }
        }
        _ => Ok(false),
    }
}

/// An edge and its two faces.
fn edge_cells<T: Real>(body: &Body<T>, g: EdgeKey) -> Option<Vec<Cell>> {
    let (f0, f1) = crate::readback::edge_sides(body, g).ok()?.faces();
    Some(vec![Cell::Edge(g), Cell::Face(f0), Cell::Face(f1)])
}

/// A vertex, its edges and its faces.
fn vertex_star<T: Real>(body: &Body<T>, w: VertexKey) -> Option<Vec<Cell>> {
    let edges = body.edges_of_vertex(w)?;
    let faces = body.faces_of_vertex(w)?;
    Some(
        core::iter::once(Cell::Vertex(w))
            .chain(edges.into_iter().map(Cell::Edge))
            .chain(faces.into_iter().map(Cell::Face))
            .collect(),
    )
}

/// A vertex's point.
fn vertex_point<T: Real>(body: &Body<T>, v: VertexKey) -> Option<Point3<T>> {
    body.get_vertex(v)
        .and_then(|d| body.get_point(d.point))
        .copied()
}

/// An edge's start and end vertices.
fn line_ends<T: Real>(body: &Body<T>, e: EdgeKey) -> Option<(VertexKey, VertexKey)> {
    let he = body.get_edge(e)?.he_plus;
    Some((body.get_half_edge(he)?.start, body.half_edge_end(he)?))
}

/// Records the cell pair `x`, `y` (result keys), once, backed by
/// `cites` (a second record of the pair adds its citations). Two cells
/// that are one, or one of which bounds the other, are structure and
/// record nothing ([`Contact::new`]); every other pair is recorded,
/// whatever its kind: how the census confirms it is read from its two
/// cells' carriers, not decided here.
fn record<T: Real>(body: &Body<T>, out: &mut ContactRecords, x: End, y: End, cites: Cites) {
    let cell = |c: Cell| super::records::conventional_edge(body, c);
    if let Ok(contact) = Contact::new(body, cell(x.1), cell(y.1)) {
        out.push(Cited::new(contact, cites));
    }
}

/// The declared face pairs lowered to SURVIVING result SURFACE pairs
/// (M4 PR 5): the equivalence rides surfaces because fragments
/// inherit their parent's surface key — face-key churn (a declared
/// face whose original key died with a discarded fragment) cannot
/// strand the intent as long as any fragment keeps the surface
/// alive. A pair with a consumed side (surface gone from the result)
/// licenses nothing and drops — its contact material did not survive
/// the op (the same consumed-record rule as contact rows);
/// resolution-level dangling was already refused at the door
/// (`validate_declarations`).
pub(super) fn declared_surface_pairs<T: Real>(
    result: &Body<T>,
    a: &Body<T>,
    b: &Body<T>,
    decls: &Glue,
    graft: &GraftMap,
) -> Vec<(SurfaceKey, SurfaceKey)> {
    decls
        .pairs
        .iter()
        .filter_map(
            |&GluedPair {
                 a: fa,
                 b: fb,
                 class,
             }| {
                // A one-carrier declaration (`Rest` or a continuation)
                // licenses a merge-stage coincidence; a `Tangent` or
                // seam pair's carriers are DISTINCT by its own
                // verification and never merge.
                if !class.is_one_carrier() {
                    return None;
                }
                // A-clone surface keys ARE result keys (carve/clone
                // preserve them); B bridges through the graft.
                let ka = a.get_face(fa)?.surface;
                let kb = graft.surfaces.get(b.get_face(fb)?.surface).copied()?;
                (result.get_surface(ka).is_some() && result.get_surface(kb).is_some() && ka != kb)
                    .then_some((ka, kb))
            },
        )
        .collect()
}

/// **The result gate every [`BooleanBody`] passes**, at the site that
/// built it and before the volume backstop reads it: the body is
/// sorted one solid per piece of material ([`crate::pieces`]), then
/// finished ([`AtRestPolicy::gate_at_rest_kept`]: tier 3, whose first
/// act is tiers 1 and 2), so the verdict rides the result and is taken
/// on the bits the caller receives. Where the scalar runs no at-rest
/// gate, [`structural_gate`] runs in its place. The at-rest gate's
/// findings are typed by [`finished_body_refusal`], and a solid it
/// passed whose only shell is certified in band
/// ([`crate::AtRestBody::slivers`]) is refused as the in-band shell it
/// is: the at-rest invariant exempts it, and a result may not hold
/// it (D10, Booleans). A scalar that runs no at-rest gate
/// ([`crate::AtRestOutcome::NotRunAtThisScalar`], a dual) reads no
/// slivers. The structural
/// gate's are structure alone, the kernel's.
///
/// # Errors
///
/// [`BooleanError::Pieces`] where the body's pieces cannot be read;
/// [`BooleanError::Escalated`] where every finding is the operands'
/// ill-conditioning, and [`BooleanError::ResultInvalid`] carrying the
/// validator's findings otherwise.
pub(super) fn gate<T: Decide + Bounds + AtRestPolicy>(
    body: Body<T>,
    band: Band,
    tol: Tol,
) -> Result<AtRestBody<T>, BooleanError> {
    let mut body = body;
    let pad = super::boxes::sweep_pad(band);
    let face_box = |body: &Body<T>, f| super::boxes::face_box(body, f, pad, band).ok();
    crate::pieces::sort_into_pieces(&mut body, band, tol, T::quad_lane(), Some(&face_box))
        .map_err(BooleanError::Pieces)?;
    #[cfg(feature = "door-tier3-meter")]
    let from = std::time::Instant::now();
    let kept = T::gate_at_rest_kept(body, tol);
    #[cfg(feature = "door-tier3-meter")]
    super::door_meter::note_gate(super::door_meter::GateReading::of(
        from.elapsed(),
        kept.as_ref().map(|_| ()).map_err(Vec::as_slice),
    ));
    let kept = kept.map_err(finished_body_refusal)?;
    if let Some(refusal) = escalated(kept.slivers()) {
        return Err(refusal);
    }
    if kept.outcome() == crate::AtRestOutcome::NotRunAtThisScalar {
        structural_gate(&kept, band)?;
    }
    Ok(kept)
}

/// **How the finished-body gate types what it found** (D10, Booleans):
/// a result holds no in-band pair or shell, and one that does is the
/// operands' ill-conditioning, which definite cuts can compose. So where
/// every finding is born of a margin certified in band, the refusal is
/// [`BooleanError::Escalated`] on the shell whose certified enclosure
/// binds the offer (nearest zero: a tolerance deciding it decides every
/// other), naming how many more lie in band; its recourse is a smaller
/// tolerance or moved parts. Any other finding is the kernel's own
/// defect, and the refusal is [`BooleanError::ResultInvalid`] carrying
/// every finding. Each finding's arm is [`finding_arm`]'s.
fn finished_body_refusal(errors: Vec<ValidationError>) -> BooleanError {
    let in_band: Option<Vec<InBandShell>> = errors.iter().map(finding_arm).collect();
    in_band
        .and_then(|shells| escalated(&shells))
        .unwrap_or(BooleanError::ResultInvalid { errors })
}

/// The refusal for `shells`, the result's shells certified in band: on
/// the one whose enclosure binds the offer, naming how many more there
/// are. `None` where there are none.
fn escalated(shells: &[InBandShell]) -> Option<BooleanError> {
    let binding = shells.iter().copied().reduce(|a, b| {
        if a.sliver
            .reading()
            .margin
            .binds_before(b.sliver.reading().margin)
        {
            a
        } else {
            b
        }
    })?;
    Some(BooleanError::Escalated {
        decision: BooleanDecision::ShellRole {
            solid: binding.solid,
            shell: binding.shell,
            others: shells.len() - 1,
        },
        diag: *binding.sliver.reading(),
    })
}

/// **Which of Q1's arms one finding of the finished-body gate is**: the
/// shell and certified margin of a finding certified in band, or `None`
/// for the kernel's. Every finding states its arm here.
fn finding_arm(finding: &ValidationError) -> Option<InBandShell> {
    use crate::props::ShellClassifyError;
    use ValidationError as V;
    match finding {
        // In band: a shell whose volume over its area, re-derived in
        // interval arithmetic, lies wholly inside one sliver band.
        // The certificate decides, whatever words the walk's ends gave the
        // refusal: a walk that read zero is refuted by it.
        V::ShellRoleUndecided {
            solid,
            error:
                ShellClassifyError::Escalated { shell, .. }
                | ShellClassifyError::ZeroVolume { shell, .. }
                | ShellClassifyError::Straddles { shell }
                | ShellClassifyError::Props { shell, .. },
            sliver: Some(certified),
        } => Some(InBandShell {
            solid: *solid,
            shell: *shell,
            sliver: **certified,
        }),
        // Undecided, and not certified in band: a role read whose
        // enclosure the arithmetic left wider than the band or straddling
        // its edge (`work/tally/a-threshold-straddling-in-band-shell-is-typed-the-kernels.md`),
        // or poisoned; or a band that does not form.
        V::ShellRoleUndecided { .. } => None,
        // Undecided at a point margin the gate has not shown conditioned
        // (`work/tally/the-door-gates-other-in-band-findings-are-typed-the-kernels.md`).
        V::DegenerateTorusEscalated { .. }
        | V::PlanarFaceEscalated { .. }
        | V::PlanarBoundaryEscalated { .. }
        | V::SliverDihedral { .. }
        | V::JoinUndecidedAtRest { .. }
        | V::VolumeSignUnresolved { .. }
        | V::RingContactEscalated { .. }
        | V::RingNestingUndecided { .. }
        | V::RingPairContactEscalated { .. }
        | V::PinchCornerEscalated { .. } => None,
        // The census's findings: the door runs no census
        // (`work/reachhold/boolean-door-runs-the-census-over-its-result.md`).
        V::UnrecordedContact { .. }
        | V::StaleContactDeclaration { .. }
        | V::ContactContradicted { .. }
        | V::CensusEscalated { .. }
        | V::CensusUnsupported { .. }
        | V::CensusLaneUnsupported { .. }
        | V::CensusUndecidable { .. }
        | V::ArcEndOffCorner { .. }
        | V::InstanceInterference { .. } => None,
        // A certificate that failed or a lane that cannot run: the
        // kernel's.
        V::Band { .. }
        | V::UncertifiableSurface { .. }
        | V::PoisonedSurfaceDescription { .. }
        | V::ApproxCertification { .. }
        | V::ApproxLaneUnsupported { .. }
        | V::PoisonedSurfaceDatum { .. }
        | V::UnrepresentableSurfaceDatum { .. }
        | V::PoisonedCurveDatum { .. }
        | V::UnrepresentableCurveDatum { .. }
        | V::EdgeCertification { .. }
        | V::VolumeUncomputable { .. }
        | V::Pcurve { .. } => None,
        // Definite findings: a kernel defect.
        V::DanglingDescription { .. }
        | V::DegenerateTorus { .. }
        | V::DescriptionNotAdjacent { .. }
        | V::PlanarFaceResidual { .. }
        | V::PlanarBoundaryResidual { .. }
        | V::TransverseNotIntrinsic { .. }
        | V::ScaffoldAtRest { .. }
        | V::JoinableVertexAtRest { .. }
        | V::TangentNotIntrinsic { .. }
        | V::LaminaWedge { .. }
        | V::NoDihedralArm { .. }
        | V::LoopRoleInverted { .. }
        | V::CurvedSenseInverted { .. }
        | V::NegativeVolume { .. }
        | V::SliverInsideOut { .. }
        | V::RingMeetsOuter { .. }
        | V::RingOutsideOuter { .. }
        | V::RingMeetsRing { .. }
        | V::PinchCornerCrossed { .. }
        | V::ShellWinding { .. }
        | V::SolidOuterShells { .. }
        | V::DanglingTopology { .. }
        | V::DanglingGeometry { .. }
        | V::NextPrevMismatch { .. }
        | V::LoopCycleOverrun { .. }
        | V::ParentLoopMismatch { .. }
        | V::UnreachableHalfEdge { .. }
        | V::EdgeHalvesIdentical { .. }
        | V::EdgeSlotBackpointerMismatch { .. }
        | V::HalfEdgeUnclaimed { .. }
        | V::HalfEdgeMultiplyClaimed { .. }
        | V::EdgeNotAntiparallel { .. }
        | V::EmanatingStartMismatch { .. }
        | V::EmptyLoopVertexWithEmanating { .. }
        | V::LoneVertexWithIncidence { .. }
        | V::VertexOrbitOverrun { .. }
        | V::OrbitForeignMember { .. }
        | V::SplitVertexOrbit { .. }
        | V::OuterListedAsRing { .. }
        | V::BackPointerMismatch { .. }
        | V::OrphanEntity { .. }
        | V::MultiplyOwned { .. }
        | V::OrphanGeometry { .. }
        | V::SolidWithoutShells { .. }
        | V::ShellWithoutFaces { .. }
        | V::EdgeAcrossShells { .. }
        | V::ComponentEulerViolation { .. }
        | V::MissingProvenance { .. }
        | V::LeakedProvenance { .. }
        | V::ScaffoldingEmptyLoop { .. }
        | V::ScaffoldingStrutVertex { .. }
        | V::ShellDisconnected { .. }
        | V::NullScaffoldShared { .. }
        | V::LeakedNullFaceRecord { .. }
        | V::StaleNullFaceLoop { .. }
        | V::StaleNullFaceOwnership { .. }
        | V::NullEdgeAtRest { .. }
        | V::NullFaceAtRest { .. } => None,
    }
}

/// The result gate where no at-rest gate ran (a dual's policy answers
/// [`crate::AtRestOutcome::NotRunAtThisScalar`]): tiers 1 and 2, then
/// tier 3's transience fence
/// ([`ValidationError::ScaffoldAtRest`](crate::ValidationError::ScaffoldAtRest))
/// and its check 11 in `band`
/// ([`ValidationError::JoinableVertexAtRest`](crate::ValidationError::JoinableVertexAtRest)),
/// which read no certification arithmetic and so answer at every
/// scalar. An edge of the result still described as a scaffold is a
/// construction that stopped half-way, and a joinable vertex one the
/// join did not finish.
pub(super) fn structural_gate<T: Decide>(body: &Body<T>, band: Band) -> Result<(), BooleanError> {
    validate(body).map_err(|errors| BooleanError::ResultInvalid { errors })?;
    validate_closed(body).map_err(|errors| BooleanError::ResultInvalid { errors })?;
    let mut errors = scaffolds_at_rest(body);
    errors.extend(crate::validate::joinable_at_rest_errors(body, band));
    if errors.is_empty() {
        Ok(())
    } else {
        Err(BooleanError::ResultInvalid { errors })
    }
}

/// What the extent scan asks of the operands before the pipeline
/// re-enters: closed groups to re-chart, trimmed faces to cut.
struct Recuts<T: Real> {
    /// Closed groups, rotated about their centres ([`apply_recuts`]).
    rechart: Vec<SphereRecut<T>>,
    /// Trimmed faces, cut across their section circles
    /// ([`apply_cut_ins`]).
    cut_in: Vec<SphereCutIn<T>>,
}

/// A sphere face that a plane face's carrier (the face's group
/// TRIMMED) or a sphere face's carrier (the group trimmed, or crossed
/// along non-parallel axes) cuts in a circle
/// certified inside both faces with no event (the section
/// certificate's R-loop). The face is cut along the meridian of its own
/// sphere's chart through the circle's centre (through a point of the
/// circle, `u_ref`'s, where the centre's direction is a pole of the
/// chart), from its boundary below the circle to its boundary above: a
/// new seam of the face, which the circle crosses twice inside the plane
/// face. Both pieces keep the
/// surface key (the maximal form a curved operand takes), a meridian
/// and the face's own edges bound each, and the re-entered crossing
/// layer meets the circle at the two crossings, as it meets a closed
/// group's re-charted seams.
struct SphereCutIn<T: Real> {
    /// The operand holding the face.
    operand: Operand,
    /// The sphere face the section circle lies inside.
    face: FaceKey,
    /// The sphere's center.
    center: Point3<T>,
    /// Its radius.
    radius: T,
    /// The section circle's center.
    foot: Point3<T>,
    /// The section circle's radius.
    rho: T,
    /// The section circle's axis.
    normal: Vec3<T>,
    /// A unit direction across the circle, which places a point of it
    /// where `foot` lies on the chart's polar axis.
    u_ref: Vec3<T>,
}

/// One sphere group the extent scan wants re-cut: rigidly re-charted
/// about `align` (the first escape plane's normal) so its seam
/// meridians run pole-to-pole TRANSVERSE to the escape planes and the
/// ordinary crossing layer sees the section circles.
struct SphereRecut<T: Real> {
    /// The operand whose group escapes.
    operand: Operand,
    /// The group's representative face (its shell is what rotates).
    representative: FaceKey,
    /// The sphere's center — the rotation's fixed point.
    center: Point3<T>,
    /// Its radius (the alignment trilean's lever arm).
    radius: T,
    /// The stored polar axis (rotation source direction).
    axis: Vec3<T>,
    /// The escape plane's normal, or the escape circle's axis (rotation
    /// target direction).
    align: Vec3<T>,
    /// The stored seam direction (`u_ref`), which `seam` turns to.
    u_ref: Vec3<T>,
    /// Where the group escapes through a sphere: the seam direction the
    /// re-chart turns `u_ref` to about `align`. A's group takes the first
    /// of `align`'s orthonormal pair and B's the second, so two balls
    /// re-charted along one centre line keep their seams in planes a
    /// quarter turn apart, and neither seam meets the circle where the
    /// other does.
    seam: Option<Vec3<T>>,
}

/// The sphere extent scan's reading of a refused [`contfp`] on `face`, a
/// plane face of the operand facing the sphere's (`sphere_is`). The
/// operands are AT REST, past the operand gate, whose tier 2 refuses a
/// lone-vertex loop: meeting one here is a kernel bug. Every other
/// refusal reads as the reduction reads it ([`super::reduce::esc`]).
/// Two test-support doors run the scan on ungated bodies
/// (`test_support::boolean_through_the_join` and
/// [`no_crossings_certificates`]); there a lone-vertex loop is the
/// fixture's, and still answers this typed invariant.
fn extent_scan_refusal(e: ContainError, sphere_is: Operand, face: FaceKey) -> BooleanError {
    match e {
        ContainError::EmptyLoop(_) => BooleanError::ClassificationInvariant {
            what: "extent scan: a gated operand face has a lone-vertex loop",
        },
        e => super::reduce::esc(e, sphere_is.other(), face),
    }
}

/// The curved-EXTENT scan (fn-level story on the call site in
/// [`boolean_op_recut`]). Sound because every CURVED-involved boundary
/// pair is either **certified disjoint** (so a connected shell shares
/// its witness vertex's side — the vertex probe's missing
/// certificate), **an escape** (re-cut and re-entered), or **refused
/// typed**. Plane-only configurations run zero new predicates.
///
/// Two classes, and they get different answers because different
/// evidence exists for them:
///
/// - **Sphere**: a closed group's true extent is `center ± r`, so the
///   pairs it can meet are enumerable exactly, and an escape through a
///   plane face is repairable by a re-chart. Two spheres whose carriers
///   cross or touch, and a sphere touching a plane's carrier, are asked
///   whether their FACES meet ([`sphere_faces_apart`]); a crossing
///   circle inside both faces is an escape of each sphere through the
///   other, and both are re-cut.
/// - **Torus, cylinder and cone**: no closed-group extent exists, so
///   their pairs are certified per pair by the section certificate
///   ([`section_extent_pass`]), which runs after this scan. A sphere's
///   pairs with a cylinder, torus or cone face are the pass's too: none
///   is an escape face, so disjointness is all the scan would ask of
///   them.
///
/// Determinism (D9): face-arena order throughout; the first escape's
/// normal is the alignment target.
///
/// # Panics
///
/// Where an arena face's surface does not resolve (D2 row 4): a torn
/// surface is neither NURBS nor a sphere. The operands are at rest
/// (gated, or a rigid re-chart of gated operands).
fn sphere_extent_scan<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    rest: &[(FaceKey, FaceKey)],
    band: Band,
) -> Result<Recuts<T>, BooleanError> {
    let esc = |question| {
        move |diag| BooleanError::Escalated {
            decision: BooleanDecision::Sphere(question),
            diag,
        }
    };
    // The NURBS re-gate (M5 S13, pinned): ANY fallback entry with a
    // NURBS face refuses before a vertex is probed — the extent test
    // is unwritable for the kind (variant docs).
    for (operand, body) in [(Operand::A, a), (Operand::B, b)] {
        for (face, fd) in body.faces() {
            if matches!(body.face_surface_linked(face, fd), geom::Surface::Nurbs(_)) {
                return Err(BooleanError::NurbsExtentUnsupported { operand, face });
            }
        }
    }
    let pad = boxes::sweep_pad(band);
    let axes = super::separating::OperandAxes::new(boxes::axis_key);
    let mut section_charts = ChartCache::default();
    let rows = [face_rows(a, band)?, face_rows(b, band)?];
    let mut out: Vec<SphereRecut<T>> = Vec::new();
    let mut cut_ins: Vec<SphereCutIn<T>> = Vec::new();
    for (x_is, x, x_rows, y, y_rows) in [
        (Operand::A, a, &rows[0], b, &rows[1]),
        (Operand::B, b, &rows[1], a, &rows[0]),
    ] {
        // The scope is the whole operand: what the escape arm re-charts
        // is the sphere itself, which every wearer shares.
        let charts = crate::chart_groups::ChartGroups::of_body(x);
        let mut seen: Vec<SurfaceKey> = Vec::new();
        for (face, fd) in x.faces() {
            let x_surface = x.face_surface_linked(face, fd);
            let &geom::Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } = x_surface
            else {
                continue;
            };
            if seen.contains(&fd.surface) {
                continue;
            }
            seen.push(fd.surface);
            // **Closedness is asked where it is USED, not on arrival.**
            // The certificate this scan spends is `center ± r`, and for
            // a TRIMMED group that box over-claims — which is the sound
            // direction for every SEPARATION test below, because a
            // trimmed face is a subset of the sphere and a box that
            // proves the whole sphere clear proves the subset clear.
            // What genuinely needs the closed group is the plane arm's
            // ESCAPE conclusion: it reasons about the whole section
            // circle of the sphere CARRIER, and it hands the result to
            // a re-chart that rotates the group about its own centre —
            // neither statement survives trimming. A trimmed group's
            // escape is read off its faces instead, and cut in.
            let group = closed_sphere_group(x, face, &charts);
            let ball_box = boxes::centred_box(center, radius, pad);
            let mut escape_normals: Vec<Vec3<T>> = Vec::new();
            // A closed group's sphere escapes, one cut each, for where
            // one re-chart cannot serve them all.
            let mut sphere_escapes: Vec<SphereCutIn<T>> = Vec::new();
            for (y_row, (yf, _)) in y_rows.iter().zip(y.faces()) {
                if y_row.face != yf {
                    return Err(BooleanError::ClassificationInvariant {
                        what: "extent scan: face rows out of arena order",
                    });
                }
                // Whether the section certificate clears every face on
                // this sphere against `yf` (`None`), else the first
                // refusing pair's face on this sphere and its refusal. A carrier margin decided zero is a
                // touch, no event where this returns `None`; one in the
                // band asks too, and the certificate, reading it
                // undecided, refuses.
                let mut faces = || {
                    sphere_faces_apart(
                        (x_is, x, x_rows),
                        fd.surface,
                        (y, y_row),
                        band,
                        &mut section_charts,
                        &axes,
                    )
                };
                match &y_row.surface {
                    &geom::Surface::Plane {
                        origin,
                        normal,
                        u_ref,
                    } => {
                        let (side, s) =
                            match ball_against_plane(center, radius, origin, normal, band) {
                                Ok(read) => read,
                                Err(diag) => {
                                    if faces().is_some() {
                                        return Err(esc(SphereQuestion::AgainstPlane)(diag));
                                    }
                                    continue;
                                }
                            };
                        match side {
                            // Clear of the whole carrier plane.
                            NonzeroSign::Negative => {}
                            NonzeroSign::Positive => {
                                // A TRIMMED group's faces may never reach
                                // the circle the carrier cuts: certified
                                // apart from this face, they pose no
                                // escape through it. Otherwise `held` is
                                // the face of the group the certificate
                                // refused, with its reason.
                                let held = match group {
                                    Some(_) => None,
                                    None => match faces() {
                                        None => continue,
                                        held => held,
                                    },
                                };
                                // The sphere definitely crosses the
                                // CARRIER in a circle; classify the
                                // circle against the FACE. Certified
                                // enclosure first: a circle box clear
                                // of every boundary-edge box cannot
                                // cross the boundary, so one exact
                                // witness extends to the whole circle.
                                let foot = center - normal * s;
                                let rho = ((radius - s.abs()) * (radius + s.abs())).sqrt();
                                let circle = super::separating::Circle {
                                    center: foot,
                                    u_ref,
                                    v_ref: normal.cross(u_ref),
                                    radius: rho,
                                };
                                if face_boundary_meets(
                                    y,
                                    yf,
                                    (
                                        super::separating::Item::Circle(circle),
                                        &boxes::circle_box(&circle, pad),
                                    ),
                                    axes.of(a, b, band),
                                    pad,
                                    band,
                                ) {
                                    return Err(BooleanError::FallbackExtentUnsupported {
                                        operand: x_is,
                                        face,
                                        what: "the sphere's section circle runs near the \
                                               plane face's boundary — whole-circle \
                                               membership cannot be certified from the \
                                               enclosures, and no crossing layer saw an \
                                               event",
                                    });
                                }
                                let witness = foot + u_ref * rho;
                                match contfp(y, yf, normal, witness, band)
                                    .map_err(|e| extent_scan_refusal(e, x_is, yf))?
                                {
                                    // The circle misses this face
                                    // (it crosses the carrier plane
                                    // elsewhere).
                                    FaceContainment::Out => {}
                                    // The whole-circle membership just
                                    // decided is the CARRIER's. A closed
                                    // group wears all of the carrier, so
                                    // the circle is its escape, and the
                                    // re-chart rotates it about its
                                    // centre. A trimmed group escapes
                                    // only where the certificate placed
                                    // the circle inside one of its faces
                                    // (R-loop), and that face takes the
                                    // cut instead ([`SphereCutIn`]).
                                    FaceContainment::In => match cut_holder(held) {
                                        Ok(None) => escape_normals.push(normal),
                                        Ok(Some(holder)) => {
                                            cut_ins.push(SphereCutIn {
                                                operand: x_is,
                                                face: holder,
                                                center,
                                                radius,
                                                foot,
                                                rho,
                                                normal,
                                                u_ref,
                                            });
                                        }
                                        Err(what) => {
                                            return Err(BooleanError::FallbackExtentUnsupported {
                                                operand: x_is,
                                                face,
                                                what,
                                            });
                                        }
                                    },
                                    // Boxes cleared yet the witness is
                                    // ON the boundary: contradictory
                                    // enclosures, loudly.
                                    FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_) => {
                                        return Err(BooleanError::ClassificationInvariant {
                                            what: "extent scan: witness on a boundary the \
                                                   certified boxes cleared",
                                        });
                                    }
                                }
                            }
                        }
                    }
                    &geom::Surface::Sphere {
                        center: c2,
                        radius: r2,
                        ..
                    } => {
                        // Every face this sphere carries is a verified
                        // `Rest` against `yf` ([`Exempt::Rest`]): the
                        // carriers are one and every face of the pair
                        // only touches, so nothing of it is the scan's.
                        // The carrier's faces include `face`, so the
                        // set is never empty.
                        let rest_pair = |r: &FaceRow<T>| match x_is {
                            Operand::A => Exempt::Rest(rest).answers(r.face, yf),
                            Operand::B => Exempt::Rest(rest).answers(yf, r.face),
                        };
                        if x_rows.iter().filter(|r| r.key == fd.surface).all(rest_pair) {
                            continue;
                        }
                        let d = (c2 - center).norm();
                        // A decided zero is band-decided: the spheres
                        // touch within the tolerance, and a positive gap
                        // there is one a smaller tolerance decides apart.
                        // Where the faces may meet at the touch, it refuses
                        // as the question's in-band arm does, with its
                        // decided margin.
                        let gap = match crate::validate::decide_nonzero(
                            "bool_sphere_sphere_gap",
                            Margin::of(d - (radius + r2)),
                            band,
                        ) {
                            Ok(gap) => gap,
                            Err(diag) => {
                                if faces().is_some() {
                                    return Err(esc(SphereQuestion::Apart)(diag));
                                }
                                continue;
                            }
                        };
                        match gap {
                            // Definitely separated.
                            NonzeroSign::Positive => {}
                            NonzeroSign::Negative => {
                                // Nested (one strictly inside the
                                // other) is boundary-disjoint too;
                                // anything else is the sphere×sphere
                                // seam frontier.
                                let big = radius.max(r2);
                                let small = radius.min(r2);
                                // Neither separated nor strictly nested:
                                // the CARRIERS cross in a circle while the
                                // crossing layer found no edge crossing a
                                // face. Whether the FACES meet is the
                                // section certificate's question, asked
                                // of every face on this sphere against
                                // `yf`: with no event anywhere, a circle
                                // certified out of one face of each pair
                                // leaves the two boundaries disjoint. A
                                // circle certified inside both faces is
                                // an escape off every edge, of each
                                // sphere through the other: a closed
                                // group's is re-charted along the centre
                                // line, a trimmed group's face is cut in
                                // ([`sphere_pair_cut`]), and the other
                                // pass serves the other sphere; any
                                // other refusal of the certificate is
                                // its own reason, raised as the pass
                                // raises it. A touch (a decided zero)
                                // refuses as spheres that meet where the
                                // faces are not certified apart.
                                let nested = crate::validate::decide_reported(
                                    "bool_sphere_sphere_nested",
                                    Margin::of(big - (d + small)),
                                    band,
                                )
                                .map_err(esc(SphereQuestion::Nested))?;
                                match Refused::of(nested, band) {
                                    None => {}
                                    Some(verdict @ Refused::Zero(_)) => {
                                        if faces().is_some() {
                                            return Err(BooleanError::SpheresMeet {
                                                operand: x_is,
                                                face,
                                                verdict,
                                            });
                                        }
                                        continue;
                                    }
                                    Some(Refused::Negative { .. }) => match faces() {
                                        None => {}
                                        Some((holder, SectionRefusal::Loop)) => {
                                            let cut = sphere_pair_cut(
                                                (x_is, holder),
                                                x_surface,
                                                &y_row.surface,
                                                band,
                                            )?;
                                            if group.is_some() {
                                                escape_normals.push(cut.normal);
                                                sphere_escapes.push(cut);
                                            } else {
                                                cut_ins.push(cut);
                                            }
                                        }
                                        Some((_, refusal)) => {
                                            return Err(BooleanError::FallbackExtentUnsupported {
                                                operand: x_is,
                                                face,
                                                what: refusal.what(),
                                            });
                                        }
                                    },
                                }
                            }
                        }
                    }
                    geom::Surface::Nurbs(_) => {
                        // Unreachable: the re-gate above runs first.
                        return Err(BooleanError::NurbsExtentUnsupported {
                            operand: x_is.other(),
                            face: yf,
                        });
                    }
                    // `Approx` joins the no-wired-arm refusal, not the
                    // NURBS lane: the pair-scoped operand gate refuses
                    // it by kind before this scan runs.
                    geom::Surface::Approx(_) => {
                        // REACH FIRST, kind second. This arm asks
                        // whether the ball can escape past THIS face;
                        // a face whose box cannot meet the ball's
                        // certified extent bounds no escape route
                        // through it, and its kind is then no more
                        // relevant here than it is at the operand
                        // gate. Only a face the ball may actually
                        // reach costs the operation its answer.
                        if !ball_may_reach(
                            (y, yf, &y_row.bbox),
                            (center, radius, &ball_box),
                            axes.of(a, b, band),
                            pad,
                            band,
                        ) {
                            continue;
                        }
                        return Err(BooleanError::CurvedBooleanUnsupported {
                            operand: x_is.other(),
                            face: yf,
                            kind: y_row.surface.kind(),
                        });
                    }
                    // Every other kind's pairs with a sphere are the
                    // section pass's ([`section_pass_takes`]): never an
                    // escape face, so the one question is disjointness.
                    other => {
                        if !section_pass_takes(other) {
                            return Err(BooleanError::ClassificationInvariant {
                                what: "extent scan: a surface kind neither the scan nor the \
                                       section pass takes",
                            });
                        }
                    }
                }
            }
            // An escape was recorded only on a closed group (a trimmed
            // one records cut-ins), which has a representative.
            if let (Some((&align, rest)), Some(representative)) =
                (escape_normals.split_first(), group)
            {
                // ONE alignment per group: the re-chart makes every
                // section polar only when ALL of this group's escapes
                // share an axis (antiparallel is the same axis). A group
                // re-charted for one axis would leave another cap's join
                // to a tilted-section refusal the re-entered crossing
                // layer may never reach, and the result could come out
                // valid and short a cap.
                let mut parallel = true;
                for &n in rest {
                    match decide(
                        "bool_sphere_escape_parallel",
                        Margin::levered(align.cross(n).norm(), radius),
                        band,
                    )
                    .map_err(esc(SphereQuestion::EscapeParallel))?
                    {
                        Sign::Zero => {}
                        Sign::Positive | Sign::Negative => parallel = false,
                    }
                }
                if !parallel {
                    // Escapes through spheres alone are cut in, one cut
                    // per circle, on the group's own chart. A plane's
                    // escape is served only by the re-chart, whose graft
                    // renames the faces a cut would name.
                    if sphere_escapes.len() == escape_normals.len() {
                        cut_ins.append(&mut sphere_escapes);
                        continue;
                    }
                    return Err(BooleanError::FallbackExtentUnsupported {
                        operand: x_is,
                        face,
                        what: if sphere_escapes.is_empty() {
                            "one sphere group escapes through NON-PARALLEL \
                             plane faces — a single re-chart cannot make \
                             every section polar, and multi-chart \
                             re-cutting is not built; refused whole rather \
                             than metering one cap and dropping the other"
                        } else {
                            "one sphere group escapes through a plane face and crosses \
                             a sphere face off every edge along NON-PARALLEL axes — the \
                             re-chart the plane asks for would rename the face the \
                             sphere's cut names"
                        },
                    });
                }
                let seam = (!sphere_escapes.is_empty()).then(|| {
                    let (first, second) = align.orthonormal_basis();
                    match x_is {
                        Operand::A => first,
                        Operand::B => second,
                    }
                });
                out.push(SphereRecut {
                    operand: x_is,
                    representative,
                    center,
                    radius,
                    axis,
                    align,
                    u_ref,
                    seam,
                });
            }
        }
    }
    Ok(Recuts {
        rechart: out,
        cut_in: cut_ins,
    })
}

/// **The cut a sphere pair's crossing asks of `holder`**, the face of
/// `operand`'s sphere `x` that holds the circle `x` crosses the sphere
/// `y` in. The circle is the pair's radical-plane section, read from
/// its owner ([`geom_brep::sphere_sphere_section`]): its centre on the
/// centre line, its axis the unit centre line from `x` toward `y`, and
/// a unit `u_ref` across it. A closed group's escape aligns its
/// re-chart on that axis; a trimmed group's face takes the cut.
fn sphere_pair_cut<T: Decide>(
    (operand, holder): (Operand, FaceKey),
    x: &geom::Surface<T>,
    y: &geom::Surface<T>,
    band: Band,
) -> Result<SphereCutIn<T>, BooleanError> {
    let corrupt = |what| BooleanError::ClassificationInvariant { what };
    let &geom::Surface::Sphere { center, radius, .. } = x else {
        return Err(corrupt(
            "extent scan: a sphere pair's cut on a face not on a sphere",
        ));
    };
    match geom_brep::sphere_sphere_section(x, y, band) {
        Ok(geom_brep::SphereSphereSection::Circle(geom::Curve3::Circle {
            center: foot,
            axis: normal,
            radius: rho,
            u_ref,
        })) => Ok(SphereCutIn {
            operand,
            face: holder,
            center,
            radius,
            foot,
            rho,
            normal,
            u_ref,
        }),
        Err(geom_brep::SectionError::Escalated(diag)) => Err(BooleanError::Escalated {
            decision: BooleanDecision::Sphere(SphereQuestion::CutIn),
            diag,
        }),
        Ok(_) | Err(_) => Err(corrupt(
            "extent scan: spheres the scan decided cross have no section circle",
        )),
    }
}

/// **What a section circle wholly inside a plane face asks of the sphere
/// group it lies on**, from the section certificate's reading of the
/// group's faces against that face (`None` for a closed group, which the
/// certificate is not asked of): `Ok(None)` re-charts the closed group,
/// `Ok(Some(face))` cuts the trimmed face the certificate placed the
/// circle inside (R-loop), and any other verdict refuses with its own
/// reason.
fn cut_holder(held: Option<(FaceKey, SectionRefusal)>) -> Result<Option<FaceKey>, &'static str> {
    match held {
        None => Ok(None),
        Some((holder, SectionRefusal::Loop)) => Ok(Some(holder)),
        Some((_, refusal)) => Err(refusal.what()),
    }
}

/// **Whether the ball about `center` of radius `radius` may reach
/// `y`'s face `yf`**: their boxes (`face_box`, the ball's `ball_box`)
/// overlap, and the narrow phase does not part the face from the WHOLE
/// ball ([`super::separating::Item::Ball`]). The question is the ball's
/// escape, so it is asked of the ball whatever part of the sphere the
/// faces on it keep: a reach of those faces is a subset of the ball, and
/// a face clear of that subset may still lie inside the ball.
fn ball_may_reach<T: Decide>(
    (y, yf, face_box): (&Body<T>, FaceKey, &bvh::Aabb),
    (center, radius, ball_box): (Point3<T>, T, &bvh::Aabb),
    axes: &[geom_core::UnitVec3<T>],
    pad: f64,
    band: Band,
) -> bool {
    face_box.overlaps(ball_box)
        && !super::separating::apart(
            (y, super::separating::Item::Face(yf)),
            (y, super::separating::Item::Ball { center, radius }),
            axes,
            pad,
            band,
        )
}

/// **Whether every face on `x`'s sphere `surface` is certified apart
/// from `y`'s face `y_row`**, whose carrier the sphere crosses or
/// touches: the section certificate's walk ([`walk_pairs`]) over those
/// pairs on the no-event path. `None` when every pair clears, else the
/// first refusing pair's face of `x` and its refusal, which is the cause.
fn sphere_faces_apart<T: Decide + Bounds + crate::props::AtRestPolicy>(
    (x_is, x, x_rows): (Operand, &Body<T>, &[FaceRow<T>]),
    surface: SurfaceKey,
    (y, y_row): (&Body<T>, &FaceRow<T>),
    band: Band,
    charts: &mut ChartCache,
    axes: &super::separating::OperandAxes<T>,
) -> Option<(FaceKey, SectionRefusal)> {
    let on_sphere = x_rows.iter().filter(|r| r.key == surface);
    let pairs = match x_is {
        Operand::A => walk_pairs(
            (x, on_sphere),
            (y, [y_row]),
            band,
            charts,
            |_, _| true,
            |_, _| false,
            |_| true,
            true,
            axes,
        ),
        Operand::B => walk_pairs(
            (y, [y_row]),
            (x, on_sphere),
            band,
            charts,
            |_, _| true,
            |_, _| false,
            |_| true,
            true,
            axes,
        ),
    };
    pairs.into_iter().find_map(|p| {
        let held = match x_is {
            Operand::A => p.a_face,
            Operand::B => p.b_face,
        };
        p.verdict.err().map(|refusal| (held, refusal))
    })
}

/// **Whether a re-cut sphere's polar axis leans off the escape normal**
/// ([`SphereQuestion::RecutAlign`]): the axes' cross levered at the
/// radius. Definite by construction on a crossing-free escape (an
/// aligned axis's seam crosses the escape plane, which the crossing
/// layer sees first), so an aligned or in-band axis refuses, with its
/// decided margin where it decided zero.
pub(super) fn recut_lean<T: Decide>(
    axis: Vec3<T>,
    align: Vec3<T>,
    radius: T,
    band: Band,
) -> Result<(), BooleanError> {
    crate::validate::decide_nonzero(
        "bool_sphere_recut_align",
        Margin::levered(axis.cross(align).norm(), radius),
        band,
    )
    .map(|_| ())
    .map_err(|diag| BooleanError::Escalated {
        decision: BooleanDecision::Sphere(SphereQuestion::RecutAlign),
        diag,
    })
}

/// Applies the scan's re-cuts: each escaping group's shell is carved
/// out, rigidly rotated about the sphere's own center so the stored
/// polar axis lands on the escape normal (the same point set — a
/// sphere is rotation-invariant about its center — with the seam
/// meridians now transverse to the escape planes), and grafted back.
fn apply_recuts<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    recuts: &[SphereRecut<T>],
    tol: Tol,
) -> Result<(Body<T>, Body<T>), BooleanError> {
    let band = Band::linear(tol)?;
    let mut out_a = a.clone();
    let mut out_b = b.clone();
    for (operand, out) in [(Operand::A, &mut out_a), (Operand::B, &mut out_b)] {
        let mine: Vec<&SphereRecut<T>> = recuts.iter().filter(|r| r.operand == operand).collect();
        if mine.is_empty() {
            continue;
        }
        let src: &Body<T> = if operand == Operand::A { a } else { b };
        let corrupt = |what| BooleanError::ClassificationInvariant { what };
        let solid = single_solid(src).map_err(|_| corrupt("re-cut operand is not one solid"))?;
        // Shell of each group's representative face, arena order.
        let shell_of = |face: FaceKey| -> Result<ShellKey, BooleanError> {
            src.shells()
                .find(|(_, sd)| sd.faces.contains(&face))
                .map(|(k, _)| k)
                .ok_or(corrupt("re-cut representative face has no shell"))
        };
        let mut rotated: Vec<Body<T>> = Vec::new();
        let mut cut_shells: Vec<ShellKey> = Vec::new();
        for r in &mine {
            let shell = shell_of(r.representative)?;
            cut_shells.push(shell);
            let ball = carve(src, solid, &[shell])
                .map_err(|_| corrupt("re-cut carve of the sphere shell failed"))?;
            recut_lean(r.axis, r.align, r.radius, band)?;
            let cross = r.axis.cross(r.align);
            // The alignment rotation, built ALGEBRAICALLY (Rodrigues
            // with the angle eliminated: R = I + K + K²/(1+c) for
            // K = [â×n̂]ₓ, c = â·n̂ — division guarded by the
            // alignment trilean above, which already refused the
            // antiparallel c ≈ −1 arm together with the parallel one).
            // No atan2/sin/cos anywhere: under the Interval scalar a
            // trig-built rotation carries straddling-zero matrix
            // entries whose downstream crossing-root phases explode at
            // the atan2 branch cut; the algebraic form keeps exact
            // zeros exact, and the f64 lane is bit-for-bit the same
            // standard construction.
            let k = cross;
            let c = r.axis.dot(r.align);
            let kx = |v: Vec3<T>| k.cross(v);
            let one = T::one();
            let col = |e: Vec3<T>| {
                let kv = kx(e);
                e + kv + kx(kv) / (one + c)
            };
            let linear = geom_core::Mat3::from_cols(
                col(Vec3::new(one, T::zero(), T::zero())),
                col(Vec3::new(T::zero(), one, T::zero())),
                col(Vec3::new(T::zero(), T::zero(), one)),
            );
            // The spin about `align` that turns the rotated seam to
            // `seam`, the same algebraic form on the seam directions; the
            // sign that keeps the turn at most a quarter is either of the
            // seam's two half-planes.
            let linear = match r.seam {
                None => linear,
                Some(seam) => {
                    let a = linear * r.u_ref;
                    let flip = matches!(
                        decide("bool_sphere_recut_seam", Margin::of(a.dot(seam)), band),
                        Ok(Sign::Negative)
                    );
                    let b = if flip { -seam } else { seam };
                    let k = a.cross(b);
                    let c = a.dot(b);
                    let kx = |v: Vec3<T>| k.cross(v);
                    let col = |e: Vec3<T>| {
                        let kv = kx(e);
                        e + kv + kx(kv) / (one + c)
                    };
                    let spin = geom_core::Mat3::from_cols(
                        col(Vec3::new(one, T::zero(), T::zero())),
                        col(Vec3::new(T::zero(), one, T::zero())),
                        col(Vec3::new(T::zero(), T::zero(), one)),
                    );
                    spin * linear
                }
            };
            let q = r.center - Point3::origin();
            let map = geom_core::Affine3::from_parts(linear, q - linear * q);
            let turned = crate::transform::transform_rigid(&ball, &map, tol)
                .map_err(|_| corrupt("re-cut rotation failed to re-certify"))?;
            rotated.push(turned);
        }
        let keep: Vec<ShellKey> = src
            .shells_of_solid(solid)
            .ok_or(corrupt("re-cut solid lost"))?
            .iter()
            .copied()
            .filter(|s| !cut_shells.contains(s))
            .collect();
        let mut rebuilt: Option<Body<T>> = if keep.is_empty() {
            None
        } else {
            Some(
                carve(src, solid, &keep)
                    .map_err(|_| corrupt("re-cut carve of the kept shells failed"))?,
            )
        };
        for turned in rotated {
            match rebuilt.as_mut() {
                None => rebuilt = Some(turned),
                Some(base) => {
                    let base_solid =
                        single_solid(base).map_err(|_| corrupt("re-cut base is not one solid"))?;
                    graft_solids_disjoint(base, &[base_solid], &turned)?;
                }
            }
        }
        out.adopt(rebuilt.ok_or(corrupt("re-cut produced no body"))?);
    }
    Ok((out_a, out_b))
}

/// Applies the scan's cut-ins ([`SphereCutIn`]): each face is cut along
/// the meridian half-plane of its sphere's chart that holds the section
/// circle's centre direction.
///
/// The circle meets that half-plane in two points (latitudes `s₁ < s₂`,
/// as sines). Every boundary arc of the face meets the meridian's plane
/// at the first-harmonic door's certified roots; those inside the arc's
/// span and on the half-plane (or at a pole) are the face's boundary
/// hits. The cut runs from the highest hit below `s₁` to the lowest
/// above `s₂`, so nothing of the boundary lies between and the arc is
/// inside the face. A hit at an arc's end is that vertex; one inside an
/// arc splits it there. A hit between `s₁` and `s₂` (a hole inside the
/// circle), no hit on a side, a carrier the door cannot read, or ends on
/// different loops of the face refuses typed.
///
/// A face an earlier cut split is read piece by piece: the later cut
/// runs on the piece holding both its circle's crossings
/// ([`super::sphere_region`]), and a circle whose crossings lie on an
/// earlier cut's meridian takes no cut of its own, the crossing layer
/// meeting it there as it meets a seam.
fn apply_cut_ins<T: Decide + crate::props::AtRestPolicy>(
    a: &mut Body<T>,
    b: &mut Body<T>,
    cut_ins: &[SphereCutIn<T>],
    band: Band,
    tol: Tol,
) -> Result<(), BooleanError> {
    const ROWS: super::circle_roots::FirstHarmonicRows = super::circle_roots::FirstHarmonicRows {
        noise: "bool_sphere_cut_roots_noise",
        coaxial: "bool_sphere_cut_roots_coaxial",
        extreme: "bool_sphere_cut_roots_extreme",
        root_slack: "bool_sphere_cut_roots_slack",
        decision: BooleanDecision::Sphere(SphereQuestion::CutIn),
    };
    let corrupt = |what| BooleanError::ClassificationInvariant { what };
    // The faces each scanned face has been cut into so far, beside its
    // own key: `(operand, scanned face, piece)`.
    let mut pieces: Vec<(Operand, FaceKey, FaceKey)> = Vec::new();
    for cut in cut_ins {
        let body = match cut.operand {
            Operand::A => &mut *a,
            Operand::B => &mut *b,
        };
        let refuse_at = |face, what| BooleanError::FallbackExtentUnsupported {
            operand: cut.operand,
            face,
            what,
        };
        let esc = |diag| BooleanError::Escalated {
            decision: BooleanDecision::Sphere(SphereQuestion::CutIn),
            diag,
        };
        let r = cut.radius;
        let sign = |name, m: Margin<T>| decide(name, m, band).map_err(esc);
        let sphere = proven(&body.faces, cut.face, EntityId::Face).surface;
        let Some(&geom::Surface::Sphere { axis, .. }) = body.get_surface(sphere) else {
            return Err(corrupt("cut-in: the face is not on a sphere"));
        };
        let axis = axis / axis.norm();
        // The meridian plane through the circle's centre direction, or
        // through a point of it where the circle is a rim of the chart.
        let toward = |d: Vec3<T>| axis.cross(d);
        let mut k = toward(cut.foot - cut.center);
        if sign("bool_sphere_cut_meridian", Margin::of(k.norm()))? == Sign::Zero {
            k = toward(cut.foot + cut.u_ref * cut.rho - cut.center);
            if sign("bool_sphere_cut_meridian", Margin::of(k.norm()))? == Sign::Zero {
                return Err(refuse_at(
                    cut.face,
                    "the section circle has no meridian of the face's chart",
                ));
            }
        }
        let k = k / k.norm();
        let h = k.cross(axis);
        // A point of the half-meridian by its coordinates across and
        // along the axis, `r·(cos φ, sin φ)`.
        let latitude = |p: Point3<T>| (h.dot(p - cut.center), axis.dot(p - cut.center));
        // The circle's two crossings, both on the half-plane `h`.
        let across = cut.normal.cross(k);
        if sign(
            "bool_sphere_cut_section_meridian",
            Margin::of(across.norm()),
        )? == Sign::Zero
        {
            return Err(refuse_at(
                cut.face,
                "the section circle lies in its own meridian plane",
            ));
        }
        let e = across / across.norm();
        let crossings = [cut.foot - e * cut.rho, cut.foot + e * cut.rho];
        for p in crossings {
            if sign("bool_sphere_cut_half", Margin::of(h.dot(p - cut.center)))? != Sign::Positive {
                return Err(refuse_at(
                    cut.face,
                    "the section circle holds a pole of the face's chart",
                ));
            }
        }
        // The piece of the scanned face that holds the circle: the face
        // itself until a cut splits it, then the piece both crossings
        // lie inside. Both on one piece's boundary is the circle across
        // an earlier cut's meridian, which the re-entered crossing layer
        // meets as it meets a seam: no cut of its own.
        let face = {
            let candidates: Vec<FaceKey> = core::iter::once(cut.face)
                .chain(
                    pieces
                        .iter()
                        .filter(|&&(o, f, _)| o == cut.operand && f == cut.face)
                        .map(|&(_, _, piece)| piece),
                )
                .collect();
            if let [only] = candidates[..] {
                only
            } else {
                let mut holder = None;
                let mut across_a_cut = false;
                for &piece in &candidates {
                    let unread = || {
                        refuse_at(
                            piece,
                            "a piece of the cut sphere face cannot be read against the section \
                             circle's crossings",
                        )
                    };
                    let read = |e: super::solid_contain::PointInSolidError| match e {
                        super::solid_contain::PointInSolidError::Escalated { diag, .. } => {
                            esc(diag)
                        }
                        _ => unread(),
                    };
                    let region =
                        super::sphere_region::sphere_face_region(body, piece, cut.center, r)
                            .map_err(read)?
                            .ok_or_else(unread)?;
                    // The region's own refusal: in band of an arc, or no
                    // ray of its walk settled, or an arc that bounds
                    // nothing — each a piece not read against the circle.
                    let read_region = |e: super::sphere_region::RegionRefusal| match e {
                        super::sphere_region::RegionRefusal::Escalated(diag) => esc(diag),
                        super::sphere_region::RegionRefusal::RayExhausted { .. }
                        | super::sphere_region::RegionRefusal::WoundPastPeriod => unread(),
                    };
                    let [lo, hi] = crossings;
                    match (
                        region.contains(lo, band).map_err(read_region)?,
                        region.contains(hi, band).map_err(read_region)?,
                    ) {
                        (Some(true), Some(true)) if holder.is_none() => holder = Some(piece),
                        (Some(false), Some(false)) => {}
                        (None, None) => across_a_cut = true,
                        _ => return Err(unread()),
                    }
                }
                match holder {
                    Some(piece) if !across_a_cut => piece,
                    None if across_a_cut => continue,
                    _ => {
                        return Err(corrupt(
                            "cut-in: the section circle lies in no one piece of its face",
                        ));
                    }
                }
            }
        };
        let refuse = |what| refuse_at(face, what);
        let fd = proven(&body.faces, face, EntityId::Face);
        let mut circle = crossings.map(latitude);
        // `x` lies above `y` by `r·sin(φx − φy)`, a length linear in the
        // arc between them (both lie on the closed half-meridian, so the
        // angle between them is short of π).
        let above = |x: (T, T), y: (T, T)| {
            sign(
                "bool_sphere_cut_order",
                Margin::of((x.1 * y.0 - x.0 * y.1) / r),
            )
        };
        if above(circle[0], circle[1])? == Sign::Positive {
            circle.swap(0, 1);
        }
        let [s1, s2] = circle;
        // The face's boundary hits on the half-meridian.
        struct Hit<T> {
            s: (T, T),
            vertex: Option<VertexKey>,
            /// The arc, the root's parameter, whether the parameter
            /// rises along the arc's edge, and the arc's radius.
            split: Option<(EdgeKey, T, bool, T)>,
        }
        let mut hits: Vec<Hit<T>> = Vec::new();
        for lk in core::iter::once(fd.outer).chain(fd.rings.iter().copied()) {
            let l = linked(
                &body.loops,
                lk,
                EntityId::Loop,
                EntityId::Face(face),
                "loop",
            );
            let LoopBoundary::Cycle { first } = l.boundary else {
                return Err(refuse("the sphere face carries a lone-vertex loop"));
            };
            for he in body.loop_walk(first).closed("loop", first) {
                let hd = proven(&body.half_edges, he, EntityId::HalfEdge);
                let edge = hd.edge;
                let ed = proven(&body.edges, edge, EntityId::Edge);
                let Some(curve) = body.edge_curve_linked(edge, ed).certified() else {
                    return Err(corrupt("cut-in: a boundary edge has no carrier"));
                };
                let (t0, t1) = curve.params();
                let geom::Curve3::Circle {
                    center: cc,
                    axis: ca,
                    radius: cr,
                    u_ref: cu,
                } = *curve.carrier()
                else {
                    return Err(refuse("a boundary edge of the sphere face is not a circle"));
                };
                let offset = cc - cut.center;
                let d = k.dot(offset);
                let (cos_part, sin_part) = (cr * k.dot(cu), cr * k.dot(ca.cross(cu)));
                let amplitude = (cos_part.powi(2) + sin_part.powi(2)).sqrt();
                let noise = super::circle_roots::rounding_charge(offset.norm() + cr);
                let roots = super::circle_roots::first_harmonic_roots(
                    &super::circle_roots::FirstHarmonic {
                        lo: d - amplitude,
                        hi: d + amplitude,
                        cos_part,
                        sin_part,
                        lo_noise: noise,
                        hi_noise: noise,
                        phase_noise: T::zero(),
                    },
                    cr,
                    t0,
                    t1,
                    &ROWS,
                    band,
                )?;
                let thetas = match roots {
                    super::circle_roots::FirstHarmonicRoots::Miss => continue,
                    super::circle_roots::FirstHarmonicRoots::Certified(thetas) => thetas.to_vec(),
                    super::circle_roots::FirstHarmonicRoots::OnSurface
                    | super::circle_roots::FirstHarmonicRoots::Uncertain => {
                        return Err(refuse(
                            "a boundary arc of the sphere face meets the cut's meridian plane \
                             where its roots are not certified",
                        ));
                    }
                };
                let rising =
                    sign("bool_sphere_cut_span", Margin::levered(t1 - t0, cr))? == Sign::Positive;
                let (lo_t, hi_t) = if rising { (t0, t1) } else { (t1, t0) };
                for theta in thetas {
                    let into = |end: T| Margin::levered(theta - end, cr);
                    let (from_lo, to_hi) = (
                        sign("bool_sphere_cut_span", into(lo_t))?,
                        sign("bool_sphere_cut_span", Margin::levered(hi_t - theta, cr))?,
                    );
                    if from_lo == Sign::Negative || to_hi == Sign::Negative {
                        continue;
                    }
                    let p = curve.carrier().eval(theta);
                    let at_pole = sign(
                        "bool_sphere_cut_pole",
                        Margin::of(axis.cross(p - cut.center).norm()),
                    )? == Sign::Zero;
                    if !at_pole
                        && sign("bool_sphere_cut_half", Margin::of(h.dot(p - cut.center)))?
                            != Sign::Positive
                    {
                        continue;
                    }
                    let vertex = if from_lo == Sign::Zero || to_hi == Sign::Zero {
                        let start = hd.start;
                        let end = proven(&body.half_edges, hd.next, EntityId::HalfEdge).start;
                        let at = |v| {
                            let q = body.resolve_vertex_point(v, crate::live::Proven);
                            sign("bool_sphere_cut_vertex", Margin::of((q - p).norm()))
                        };
                        match (at(start)?, at(end)?) {
                            (Sign::Zero, _) => Some(start),
                            (_, Sign::Zero) => Some(end),
                            _ => {
                                return Err(refuse(
                                    "the cut's meridian meets a boundary arc at its end, off \
                                     its vertices",
                                ));
                            }
                        }
                    } else {
                        None
                    };
                    hits.push(Hit {
                        s: latitude(p),
                        vertex,
                        split: vertex.is_none().then_some((edge, theta, rising, cr)),
                    });
                }
            }
        }
        let mut below: Option<&Hit<T>> = None;
        let mut over: Option<&Hit<T>> = None;
        for hit in &hits {
            match (above(hit.s, s1)?, above(hit.s, s2)?) {
                (Sign::Negative, _) => {
                    let nearer = match below {
                        None => true,
                        Some(b) => above(hit.s, b.s)? == Sign::Positive,
                    };
                    if nearer {
                        below = Some(hit);
                    }
                }
                (_, Sign::Positive) => {
                    let nearer = match over {
                        None => true,
                        Some(o) => above(o.s, hit.s)? == Sign::Positive,
                    };
                    if nearer {
                        over = Some(hit);
                    }
                }
                _ => {
                    return Err(refuse(
                        "the sphere face's boundary meets the cut's meridian inside the section \
                         circle",
                    ));
                }
            }
        }
        let (Some(below), Some(over)) = (below, over) else {
            return Err(refuse(
                "the cut's meridian leaves the sphere face through no boundary",
            ));
        };
        let (below_split, below_vertex) = (below.split, below.vertex);
        let (over_split, over_vertex) = (over.split, over.vertex);
        let mut end =
            |split: Option<(EdgeKey, T, bool, T)>, vertex: Option<VertexKey>| match (split, vertex)
            {
                (_, Some(v)) => Ok(v),
                (Some((edge, theta, _, _)), None) => Ok(body.split_edge(edge, theta, tol)?.vertex),
                (None, None) => Err(corrupt(
                    "cut-in: a boundary hit with neither vertex nor arc",
                )),
            };
        // Two ends on one arc: the split keeps the arc's key on its
        // first child, so the end farther along the arc's parameter is
        // split first and the nearer one still lies on the key.
        let below_first = match (below_split, over_split) {
            (Some((e1, lo, rising, arm)), Some((e2, hi, _, _))) if e1 == e2 => {
                let along = if rising { lo - hi } else { hi - lo };
                sign("bool_sphere_cut_span", Margin::levered(along, arm))? == Sign::Positive
            }
            _ => true,
        };
        let (v_lo, v_hi) = if below_first {
            let v_lo = end(below_split, below_vertex)?;
            (v_lo, end(over_split, over_vertex)?)
        } else {
            let v_hi = end(over_split, over_vertex)?;
            (end(below_split, below_vertex)?, v_hi)
        };
        // The two ends' half-edges in the face's one loop.
        let fd = proven(&body.faces, face, EntityId::Face);
        let mut ends = [None, None];
        for lk in core::iter::once(fd.outer).chain(fd.rings.iter().copied()) {
            let l = linked(
                &body.loops,
                lk,
                EntityId::Loop,
                EntityId::Face(face),
                "loop",
            );
            let LoopBoundary::Cycle { first } = l.boundary else {
                continue;
            };
            for he in body.loop_walk(first).closed("loop", first) {
                let start = proven(&body.half_edges, he, EntityId::HalfEdge).start;
                for (slot, v) in ends.iter_mut().zip([v_lo, v_hi]) {
                    if start == v {
                        if slot.is_some() {
                            return Err(refuse(
                                "the cut's end vertex recurs on the sphere face's loops",
                            ));
                        }
                        *slot = Some((lk, he));
                    }
                }
            }
        }
        let [Some((l_lo, he_lo)), Some((l_hi, he_hi))] = ends else {
            return Err(corrupt("cut-in: a cut end is not on the face's loops"));
        };
        if l_lo != l_hi {
            return Err(refuse(
                "the cut's meridian joins two loops of the sphere face",
            ));
        }
        let p_lo = body.resolve_vertex_point(v_lo, crate::live::Proven);
        let p_hi = body.resolve_vertex_point(v_hi, crate::live::Proven);
        let u_arc = (p_lo - cut.center) / r;
        let w = -k;
        let to_hi = (p_hi - cut.center) / r;
        // Both ends lie on the closed half-meridian `h`, the low one
        // below, so the turn about `w` from one to the other is in
        // `(0, π]`: the cross term's sign is known, and where both ends
        // are poles it is a rounded zero of either sign.
        let span = w.dot(u_arc.cross(to_hi)).abs().atan2(u_arc.dot(to_hi));
        let carrier = geom::Curve3::Circle {
            center: cut.center,
            axis: w,
            radius: r,
            u_ref: u_arc,
        };
        let plane = body.add_surface(geom::Surface::Plane {
            origin: cut.center,
            normal: k,
            u_ref: h,
        });
        let created = body.mef(
            crate::euler::MefSite::Chords {
                he1: he_lo,
                he2: he_hi,
            },
            geom_brep::EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::Intersection {
                    pair: geom_brep::SurfacePair::new(sphere, plane),
                    witness: carrier.mid_point(T::zero(), span),
                },
                carrier,
                param_start: T::zero(),
                param_end: span,
            },
            crate::euler::FaceSurface::Inherit,
            tol,
        )?;
        pieces.push((cut.operand, cut.face, created.face));
    }
    for body in [a, b] {
        crate::pcurves::mint_pcurves(body, tol)
            .map_err(|source| BooleanError::Pcurves { source })?;
    }
    Ok(())
}

/// Per-shell classification of one operand's clone against the other
/// pristine operand (containment fallback), by the uncut-shell witness
/// ([`super::shell_witness`]).
///
/// **The witness is not the certificate** (M5 S13). A curved boundary
/// can leave the other solid strictly between a shell's witnesses (the
/// S12 finding), so for the sphere class the answer is only sound
/// because [`sphere_extent_scan`] ran first and certified every
/// sphere-involved boundary pair disjoint (or re-cut / refused): a
/// connected shell whose surface avoids the other boundary lies in one
/// component, and the witness names it.
fn classify_shells<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    other: &Body<T>,
    operand: Operand,
    coincident: &[super::SettledPair],
    band: Band,
    tol: Tol,
) -> Result<Vec<(ShellKey, ShellVerdict)>, BooleanError> {
    body.shells()
        .map(|(shell, _)| {
            let verdict = shell_verdict((body, shell, operand), other, coincident, band, tol)?;
            Ok((shell, verdict))
        })
        .collect()
}

/// The containment fallback (F8): no crossings — classify whole
/// shells, keep per Eq. 15.1's sides, and assemble the typed result.
fn fallback<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    red: &BooleanReduction<T>,
    a_pristine: &Body<T>,
    b_pristine: &Body<T>,
    decls: &Glue,
    band: Band,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    debug_assert_contacts_undecisive(
        &red.contacts,
        (&red.a, b_pristine),
        (&red.b, a_pristine),
        band,
        tol,
    );
    let a_sides = classify_shells(&red.a, b_pristine, Operand::A, &red.coincident, band, tol)?;
    let b_sides = classify_shells(&red.b, a_pristine, Operand::B, &red.coincident, band, tol)?;
    check_mutual(
        [(&red.a, &a_sides), (&red.b, &b_sides)],
        [a_pristine, b_pristine],
    )?;
    let a_keep = kept_shells(op, Operand::A, &a_sides);
    let b_keep = kept_shells(op, Operand::B, &b_sides);

    let carve_kept = |body: &Body<T>, keep: &[ShellKey]| -> Result<Body<T>, BooleanError> {
        let solid = single_solid(body).map_err(|_| desync("fallback operand not one solid"))?;
        carve(body, solid, keep).map_err(|_| desync("fallback carve failed"))
    };

    match (a_keep.is_empty(), b_keep.is_empty()) {
        (true, true) => {
            if op == BooleanOp::Union {
                // ∪ can never be empty; only complement operands (both
                // boundaries inside the other's material) reach here.
                Err(BooleanError::UnrepresentableResult)
            } else {
                Ok(BooleanResult::Empty)
            }
        }
        (false, true) => {
            let body = carve_kept(&red.a, &a_keep)?;
            finish_fallback(op, body, red, decls, BooleanResultKind::OperandA, band, tol)
        }
        (true, false) => {
            let body = carve_kept(&red.b, &b_keep)?;
            finish_fallback(op, body, red, decls, BooleanResultKind::OperandB, band, tol)
        }
        (false, false) => {
            let mut body = carve_kept(&red.a, &a_keep)?;
            let b_body = carve_kept(&red.b, &b_keep)?;
            let solid =
                single_solid(&body).map_err(|_| desync("fallback A carve not one solid"))?;
            let graft = if op == BooleanOp::Subtract {
                // ∖ with disjoint boundaries and kept B shells: every
                // kept shell classified strictly `In` A, so this is
                // the cavity case, routed through THE void-insertion
                // door (`voids` module — the one birthplace of
                // cavities), with this fallback's own probe verdicts
                // supplied as the door's containment evidence. The
                // evidence-shape refusals are unreachable from here
                // (the kept set IS the evidence set), mapped to the
                // desync they would represent.
                let evidence = voids::VoidEvidence {
                    shells: b_keep
                        .iter()
                        .map(|&s| (s, voids::VoidContainment::Probed(SolidContainment::In)))
                        .collect(),
                };
                voids::insert_hollow_voids(&mut body, &[solid], b_body, &evidence)
                    .map_err(|e| match e {
                        voids::VoidInsertError::StaleSolid { .. }
                        | voids::VoidInsertError::SolidCount { .. } => unreachable!(
                            "the void fallback grafts a one-solid B carve into the one solid \
                             of the A carve it holds: {e:?}"
                        ),
                        voids::VoidInsertError::MissingEvidence { .. }
                        | voids::VoidInsertError::NotStrictlyContained { .. }
                        | voids::VoidInsertError::ForeignShell { .. }
                        | voids::VoidInsertError::DuplicateEvidence { .. }
                        | voids::VoidInsertError::HollowCavity { .. } => {
                            desync("void evidence desynced from the kept B shells")
                        }
                    })?
                    .graft
            } else {
                // The kept B shells cross whole, as the reduction left
                // them: any edge it split at a contact was certified
                // there through the policy's lane, and nothing touches
                // them since, so they keep their certificates, as at the
                // void door above.
                graft_solids_disjoint(&mut body, &[solid], &b_body)?
            };
            let kind = match op {
                BooleanOp::Subtract => BooleanResultKind::Voided,
                _ => BooleanResultKind::Assembly,
            };
            let declared_pairs =
                declared_surface_pairs(&body, a_pristine, b_pristine, decls, &graft);
            let mut desc = Descendants::default();
            let ledger = ledger(red, decls, band)?;
            let views = [&KeyView::Direct, &KeyView::Graft(&graft)];
            let recorded = carried_vertices(&body, &ledger, views, &desc)?;
            let (merged, edge_joins) = finish_output(
                &mut body,
                &mut desc,
                &declared_pairs,
                &[],
                &recorded,
                band,
                tol,
            )?;
            crate::pcurves::mint_pcurves(&mut body, tol)
                .map_err(|source| BooleanError::Pcurves { source })?;
            let (contacts, coincidences) = carry(&body, &ledger, views, &desc, None, band)?;
            share_recorded_points(&mut body, &contacts, &coincidences)?;
            let body = gate(body, band, tol)?;
            let (graft_vertices, graft_edges, graft_dead_edges, graft_faces) = graft_rows(&graft);
            let naming = BooleanNaming {
                a_keys: OperandKeys::Direct,
                b_keys: OperandKeys::Grafted,
                graft_vertices,
                graft_edges,
                graft_dead_edges,
                graft_faces,
                merge_groups: merge_rows(&merged),
                merge_substitutions: merge_substitution_rows(&merged),
                merge_skipped: merged.skipped.clone(),
                reduction_contacts: red.contacts.clone(),
                covered: red.covered.clone(),
                edge_classes: red.edge_classes.clone(),
                null_copies: super::null_copy_rows(&red.null_edges),
                edge_joins,
                ..BooleanNaming::default()
            };
            Ok(BooleanResult::Body(BooleanBody {
                body,
                kind,
                contacts,
                naming,
                coincidences,
            }))
        }
    }
}

/// Finishes a single-operand fallback result (the merge output stage
/// is a documented no-op on a maximal-faced operand but runs anyway —
/// the contract is uniform), applying ∖'s B-side revert when needed.
fn finish_fallback<T: Decide + Bounds + AtRestPolicy>(
    op: BooleanOp,
    body: Body<T>,
    red: &BooleanReduction<T>,
    decls: &Glue,
    kind: BooleanResultKind,
    band: Band,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    let (contacts, covered) = (&red.contacts, &red.covered);
    let reduction_contacts = contacts.clone();
    let mut body = body;
    if kind == BooleanResultKind::OperandB && op == BooleanOp::Subtract {
        body = body.revert();
    }
    // No cross-operand pair merges here: one operand is absent from the
    // result. A declared pair that held the absent operand's region
    // through the kept one is `covered`; the surviving operand's
    // CARRIED records still apply.
    let mut desc = Descendants::default();
    let (a_view, b_view) = match kind {
        BooleanResultKind::OperandA => (KeyView::Direct, KeyView::Absent),
        _ => (KeyView::Absent, KeyView::Direct),
    };
    let ledger = ledger(red, decls, band)?;
    let recorded = carried_vertices(&body, &ledger, [&a_view, &b_view], &desc)?;
    let (merged, edge_joins) = finish_output(&mut body, &mut desc, &[], &[], &recorded, band, tol)?;
    crate::pcurves::mint_pcurves(&mut body, tol)
        .map_err(|source| BooleanError::Pcurves { source })?;
    let (contacts, coincidences) = carry(&body, &ledger, [&a_view, &b_view], &desc, None, band)?;
    share_recorded_points(&mut body, &contacts, &coincidences)?;
    let body = gate(body, band, tol)?;
    let naming = match kind {
        BooleanResultKind::OperandA => BooleanNaming {
            a_keys: OperandKeys::Direct,
            b_keys: OperandKeys::Absent,
            merge_groups: merge_rows(&merged),
            merge_substitutions: merge_substitution_rows(&merged),
            merge_skipped: merged.skipped.clone(),
            reduction_contacts: reduction_contacts.clone(),
            covered: covered.to_vec(),
            edge_classes: red.edge_classes.clone(),
            null_copies: super::null_copy_rows(&red.null_edges),
            edge_joins: edge_joins.clone(),
            ..BooleanNaming::default()
        },
        // The result arena IS the B clone: B keys direct, A absent.
        _ => BooleanNaming {
            a_keys: OperandKeys::Absent,
            b_keys: OperandKeys::Direct,
            merge_groups: merge_rows(&merged),
            merge_substitutions: merge_substitution_rows(&merged),
            merge_skipped: merged.skipped.clone(),
            reduction_contacts: reduction_contacts.clone(),
            covered: covered.to_vec(),
            edge_classes: red.edge_classes.clone(),
            null_copies: super::null_copy_rows(&red.null_edges),
            edge_joins: edge_joins.clone(),
            ..BooleanNaming::default()
        },
    };
    Ok(BooleanResult::Body(BooleanBody {
        body,
        kind,
        contacts,
        naming,
        coincidences,
    }))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    /// **Every pair of distinct, non-incident cells is recorded**
    /// ([`super::record`]): a brick's edge against each face it does not
    /// bound, and a cylinder sheet's edges against another sheet's
    /// curved wall; a sheet's edges against its own wall are structure.
    #[test]
    fn every_non_incident_edge_face_pair_is_recorded_whatever_its_carriers() {
        use crate::boolean::{Cell, ContactRecords, Operand};
        use crate::test_support_fixtures::{CylFrame, brick, cyl_wall_sheet};
        let tol = geom_core::Tol::witness();
        let block = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let mut out = ContactRecords::default();
        for (e, _) in block.edges() {
            for (f, _) in block.faces() {
                super::record(
                    &block,
                    &mut out,
                    (Operand::A, Cell::Edge(e)),
                    (Operand::B, Cell::Face(f)),
                    crate::Cites::decided(0),
                );
            }
        }
        assert_eq!(
            out.edge_face().count(),
            12 * 4,
            "each edge against the four faces it does not bound"
        );
        let mut sheet = crate::Body::<f64>::new();
        let wall = cyl_wall_sheet(
            &mut sheet,
            CylFrame::canonical(1.0),
            (0.2, 1.4),
            (0.0, 1.0),
            tol,
        );
        let own: Vec<crate::EdgeKey> = sheet.edges().map(|(e, _)| e).collect();
        let mut out = ContactRecords::default();
        let mut put = |sheet: &crate::Body<f64>, edge| {
            super::record(
                sheet,
                &mut out,
                (Operand::A, Cell::Edge(edge)),
                (Operand::B, Cell::Face(wall)),
                crate::Cites::decided(0),
            );
        };
        for &edge in &own {
            put(&sheet, edge);
        }
        cyl_wall_sheet(
            &mut sheet,
            CylFrame::canonical(1.0),
            (2.0, 3.0),
            (0.0, 1.0),
            tol,
        );
        let other: Vec<crate::EdgeKey> = sheet
            .edges()
            .map(|(e, _)| e)
            .filter(|e| !own.contains(e))
            .collect();
        for &edge in &other {
            put(&sheet, edge);
        }
        assert_eq!(
            out.edge_face().map(|(e, _)| e).collect::<Vec<_>>(),
            other,
            "the other sheet's edges are recorded and the wall's own are not"
        );
    }
    /// The door over discovered records alone.
    fn found<T: geom_core::Decide>(
        body: &crate::Body<T>,
        contacts: &crate::boolean::ReductionContacts,
        views: [&super::KeyView<'_>; 2],
        desc: &super::Descendants,
    ) -> Result<crate::boolean::ContactRecords, crate::boolean::BooleanError> {
        let none = super::Rows::default();
        let rows = super::tagged(contacts);
        super::carry_rows(body, &rows, [&none, &none], views, desc, false, band())
    }

    /// The door over carried records alone.
    fn carried_in<T: geom_core::Decide>(
        body: &crate::Body<T>,
        decls: &crate::boolean::Glue,
        views: [&super::KeyView<'_>; 2],
        desc: &super::Descendants,
    ) -> Result<crate::boolean::ContactRecords, crate::boolean::BooleanError> {
        super::carry_rows(
            body,
            &[],
            [
                &super::Rows::of(&decls.records_a, crate::boolean::Operand::A),
                &super::Rows::of(&decls.records_b, crate::boolean::Operand::B),
            ],
            views,
            desc,
            false,
            band(),
        )
    }

    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{gate, seam_class, seam_must_carry, volume_backstop};

    /// **A cycle in the rows refuses, and a cell with no row ends
    /// dead**, for both readers of the shared chase (`chase_live`): the
    /// naming's settler and the descendant map. Against an empty body
    /// nothing is live, so a two-row cycle between faces `a` and `b` can
    /// only be walked round, and a vertex no row names stops where it
    /// starts. The settler also refuses a cell two merge rows name.
    #[test]
    fn the_chase_refuses_a_cycle_and_a_twice_named_cell_and_ends_dead_at_a_cell_with_no_row() {
        use super::{BooleanNaming, Descendants, KeyView, SettleError};
        use crate::boolean::Cell;
        use crate::entity::{FaceKey, VertexKey};
        let key = |n: u64| slotmap::KeyData::from_ffi((1u64 << 32) | n);
        let (v, w): (VertexKey, VertexKey) = (key(1).into(), key(2).into());
        let (a, b): (FaceKey, FaceKey) = (key(3).into(), key(4).into());
        let body = crate::Body::<f64>::new();

        let naming = BooleanNaming {
            merge_substitutions: vec![
                (Cell::Vertex(v), Cell::Face(a)),
                (Cell::Face(a), Cell::Face(b)),
                (Cell::Face(b), Cell::Face(a)),
            ],
            ..BooleanNaming::default()
        };
        let settler = naming.settler(&body);
        assert_eq!(settler.settle(v), Err(SettleError::Cyclic { vertex: v }));

        // Two rows naming one cell refuse, whichever kept face is last.
        let twice = BooleanNaming {
            merge_substitutions: vec![
                (Cell::Vertex(w), Cell::Face(a)),
                (Cell::Vertex(w), Cell::Face(b)),
            ],
            ..BooleanNaming::default()
        };
        assert_eq!(
            twice.settler(&body).settle(w),
            Err(SettleError::DuplicateRow {
                vertex: w,
                dead: Cell::Vertex(w),
            })
        );
        assert_eq!(
            settler.settle(w),
            Err(SettleError::Dead {
                vertex: w,
                reached: Cell::Vertex(w),
            })
        );

        let mut desc = Descendants::default();
        desc.substitute(Cell::Face(a), Cell::Face(b));
        desc.substitute(Cell::Face(b), Cell::Face(a));
        assert!(matches!(
            desc.chained(&body, &KeyView::Direct, Cell::Face(a)),
            Err(BooleanError::JoinDesync { .. })
        ));
        assert!(matches!(
            desc.chained(&body, &KeyView::Direct, Cell::Vertex(w)),
            Ok(None)
        ));
    }
    use crate::boolean::{BooleanDecision, BooleanError, BooleanOp, LeverArm};
    use crate::props::QuadLane;
    use crate::splitting::reassembly::quad_prism;
    use crate::test_support::finished;

    /// The result gate refuses a scaffold at rest: a box whose chords
    /// were never described is a closed solid (tiers 1 and 2 pass), and
    /// tier 3 refuses it with one `ScaffoldAtRest` per edge among its
    /// findings; the same box described passes.
    #[test]
    fn the_result_gate_refuses_a_scaffold_at_rest() {
        let tol = Tol::witness();
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let raw = quad_prism(&square, 1.0, tol);
        assert_eq!(
            crate::validate_closed(&raw),
            Ok(()),
            "the raw box is closed"
        );
        let band = Band::linear(tol).unwrap();
        let Err(BooleanError::ResultInvalid { errors }) = gate(raw.clone(), band, tol) else {
            panic!("the raw box's chords are scaffolds at rest");
        };
        let mut named: Vec<_> = errors
            .iter()
            .filter_map(|e| match e {
                crate::ValidationError::ScaffoldAtRest { edge } => Some(*edge),
                _ => None,
            })
            .collect();
        named.sort();
        let mut edges: Vec<_> = raw.edges().map(|(k, _)| k).collect();
        edges.sort();
        assert_eq!(named, edges, "one finding per edge, each edge once");
        let described =
            crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        assert_eq!(
            gate(described, band, tol)
                .map(|_| ())
                .map_err(|e| e.to_string()),
            Ok(()),
            "the described box passes"
        );
    }

    /// The findings `gate` refuses `body` with, each matching `want`.
    fn refused_by_gate(
        what: &str,
        body: crate::Body<f64>,
        want: fn(&crate::ValidationError) -> bool,
    ) -> usize {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let Err(BooleanError::ResultInvalid { errors }) = gate(body, band, tol) else {
            panic!("{what}: the result gate refuses it");
        };
        assert!(errors.iter().all(want), "{what}: {errors:?}");
        errors.len()
    }

    /// **The result gate is tier 3, not tiers 1 and 2 with the scaffold
    /// fence.** Three bodies that pass tiers 1 and 2 and carry no
    /// scaffold, each refused for a tier-3 reason of its own: a lamina
    /// (the coplanar digon pillow with one face's material side flipped,
    /// one `LaminaWedge` per edge), an inside-out brick (its revert, one
    /// `NegativeVolume`), and a stranded split top (one half lifted
    /// `1000 ε` off its neighbour's plane, planar residuals only).
    #[test]
    fn the_result_gate_refuses_what_only_tier_3_sees() {
        use crate::ValidationError as V;
        let tol = Tol::witness();
        let (pillow, split) = crate::tier3_tests::coplanar_pillow(tol);
        let lamina = pillow.flipped_face_sense_for_tests(split.face).unwrap();
        let inside_out =
            crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol)
                .revert();
        let up = Vec3::new(0.0, 0.0, 1.0);
        let offset = 1e3 * tol.get().eps;
        let stranded =
            crate::boolean::refusal_routes::tests::top_split_redescribed(|p0, along, _| {
                crate::test_support_fixtures::plane(
                    &[
                        p0 + up * offset,
                        p0 + up * offset + along,
                        p0 + up * offset + up.cross(along),
                    ],
                    tol,
                )
            });
        for (what, body) in [
            ("lamina", &lamina),
            ("inside-out", &inside_out),
            ("stranded", &stranded),
        ] {
            assert_eq!(
                crate::validate_closed(body),
                Ok(()),
                "{what}: tiers 1 and 2 pass"
            );
            assert!(
                crate::validate::scaffolds_at_rest(body).is_empty(),
                "{what}: no scaffold"
            );
        }
        assert_eq!(
            refused_by_gate("lamina", lamina, |e| matches!(e, V::LaminaWedge { .. })),
            2,
            "one per edge of the digon"
        );
        assert_eq!(
            refused_by_gate("inside-out", inside_out, |e| matches!(
                e,
                V::NegativeVolume { .. }
            )),
            1,
            "its one solid"
        );
        assert!(
            refused_by_gate("stranded", stranded, |e| matches!(
                e,
                V::PlanarFaceResidual { .. }
                    | V::PlanarBoundaryResidual { .. }
                    | V::DescriptionNotAdjacent { .. }
            )) > 0
        );
    }

    /// **The gate's findings take their arm** (D10, Booleans): shells
    /// certified slivers alone are the operands' ill-conditioning,
    /// refused `Escalated` on the one whose enclosure binds the offer
    /// (nearest zero, on either side, in either order) and naming how many
    /// more lie in band; the same shell beside a definite finding, or an
    /// escalation the arithmetic may have made, is the kernel's.
    #[test]
    fn the_gate_types_only_certified_in_band_findings_escalated() {
        use crate::ShellClassifyError as S;
        use crate::ValidationError as V;
        use crate::entity::{ShellKey, SolidKey};
        use crate::props::CertifiedSliver;
        use geom_core::{Band, Indeterminate, MarginDiag};
        let band = Band::new(1e-9, 1e-8).unwrap();
        let read = |margin, terminal_sliver| Indeterminate {
            margin,
            band,
            predicate: Some("positive_volume"),
            terminal_sliver,
        };
        let walk = read(MarginDiag::value(3.05e-9), false);
        let sliver = |lo, hi| {
            CertifiedSliver::of(read(MarginDiag::enclosure(lo, hi), true)).expect("in band")
        };
        let (thin, thick) = (sliver(-3.29e-9, -3.28e-9), sliver(6.57e-9, 6.58e-9));
        let shells: Vec<ShellKey> = (1..=3)
            .map(|n: u64| slotmap::KeyData::from_ffi((1 << 32) | n).into())
            .collect();
        let role = |shell: ShellKey, sliver: Option<CertifiedSliver>| V::ShellRoleUndecided {
            solid: SolidKey::default(),
            error: S::Escalated {
                shell,
                source: walk,
            },
            sliver: sliver.map(Box::new),
        };
        // A walk that read zero, refuted by the certificate.
        let zero_read = |shell: ShellKey, sliver: Option<CertifiedSliver>| V::ShellRoleUndecided {
            solid: SolidKey::default(),
            error: S::ZeroVolume {
                shell,
                verdict: geom_brep::recourse::Classified {
                    margin: MarginDiag::value(8.7e-10),
                    band,
                },
            },
            sliver: sliver.map(Box::new),
        };
        let definite = V::NegativeVolume {
            solid: SolidKey::default(),
        };
        for (what, errors) in [
            (
                "thin first",
                vec![role(shells[0], Some(thin)), role(shells[1], Some(thick))],
            ),
            (
                "thick first",
                vec![role(shells[1], Some(thick)), role(shells[0], Some(thin))],
            ),
            (
                "the thin one's walk read zero",
                vec![
                    role(shells[1], Some(thick)),
                    zero_read(shells[0], Some(thin)),
                ],
            ),
        ] {
            let refusal = super::finished_body_refusal(errors);
            assert!(
                matches!(
                    &refusal,
                    BooleanError::Escalated {
                        decision: BooleanDecision::ShellRole { shell, others: 1, .. },
                        diag,
                    } if *shell == shells[0] && diag == thin.reading()
                ),
                "{what}: the thinner shell binds, the other counted: {refusal:?}"
            );
        }
        for (what, errors) in [
            ("an uncertified escalation", vec![role(shells[0], None)]),
            ("an uncertified zero", vec![zero_read(shells[0], None)]),
            (
                "beside one",
                vec![role(shells[0], Some(thin)), role(shells[2], None)],
            ),
            (
                "beside a definite finding",
                vec![role(shells[0], Some(thin)), definite],
            ),
            ("nothing", Vec::new()),
        ] {
            let n = errors.len();
            assert!(
                matches!(
                    super::finished_body_refusal(errors),
                    BooleanError::ResultInvalid { errors } if errors.len() == n
                ),
                "{what}: the kernel's, every finding kept"
            );
        }
    }

    /// **At a dual the result gate is main's structural gate.** A dual's
    /// policy runs no at-rest gate, so tiers 1 and 2 and the transience
    /// fence run in its place: the undescribed box refuses one
    /// `ScaffoldAtRest` per chord, and the described box passes, with no
    /// verdict.
    #[test]
    fn at_a_dual_the_result_gate_still_refuses_a_scaffold_at_rest() {
        use geom_core::Dual64;
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let mut raw = crate::Body::<Dual64>::new();
        crate::test_support_fixtures::prism_ops(
            &mut raw,
            &square,
            (0.0, 1.0),
            crate::test_support_fixtures::identity_map,
            crate::test_support_fixtures::FaceGeometry::Certified,
            tol,
        );
        let Err(BooleanError::ResultInvalid { errors }) = gate(raw.clone(), band, tol) else {
            panic!("the raw box's chords are scaffolds at rest, at a dual too");
        };
        assert_eq!(
            errors.len(),
            raw.edges().count(),
            "one per chord: {errors:?}"
        );
        assert!(
            errors
                .iter()
                .all(|e| matches!(e, crate::ValidationError::ScaffoldAtRest { .. })),
            "{errors:?}"
        );
        let described =
            crate::test_support_fixtures::brick::<Dual64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let kept = gate(described, band, tol).expect("the described box passes");
        assert_eq!(kept.outcome(), crate::AtRestOutcome::NotRunAtThisScalar);
    }

    /// **At a dual the result gate asks check 11 too.** No boolean
    /// output reaches this arm: the output stage ends with the join,
    /// which takes every vertex the same predicate reads, and nothing
    /// between the join and the gate makes a vertex (`sort_into_pieces`
    /// only sorts faces into solids). So the arm is the output's
    /// postcondition, witnessed here on the described box with one edge
    /// split by hand: the gate refuses it with exactly the split vertex,
    /// and the box passes once joined.
    #[test]
    fn at_a_dual_the_result_gate_refuses_a_joinable_vertex() {
        use geom_core::{Dual64, Real};
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body =
            crate::test_support_fixtures::brick::<Dual64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let (edge, curve) = body.edges().next().map(|(k, d)| (k, d.curve)).unwrap();
        let (t0, t1) = body
            .get_curve_geom(curve)
            .and_then(crate::CurveGeom::certified)
            .unwrap()
            .params();
        let split = body
            .split_edge(edge, (t0 + t1) * Dual64::from_f64(0.5), tol)
            .unwrap()
            .vertex;
        let Err(BooleanError::ResultInvalid { errors }) = gate(body.clone(), band, tol) else {
            panic!("a split vertex is no output's");
        };
        assert_eq!(
            errors,
            vec![crate::ValidationError::JoinableVertexAtRest { vertex: split }]
        );
        let joins = body
            .join_edges(band, tol)
            .expect("the split edge joins back");
        assert_eq!(joins.len(), 1, "the split vertex, joined");
        let kept = gate(body, band, tol).expect("the joined box passes");
        assert_eq!(kept.outcome(), crate::AtRestOutcome::NotRunAtThisScalar);
    }

    /// The gate's fence is check 2's: on tier-2-valid bodies,
    /// `scaffolds_at_rest` is exactly the `ScaffoldAtRest` findings of
    /// `validate_geometric`, in the same order. An undescribed box has
    /// a scaffold on every edge; a described prism with a collinear
    /// profile run keeps one, on the smooth edge between its two
    /// coplanar walls.
    #[test]
    fn the_fence_is_check_twos_scaffold_findings() {
        let tol = Tol::witness();
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let run = [(0.0, 0.0), (0.5, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        for (what, body, count) in [
            ("undescribed box", quad_prism(&square, 1.0, tol), 12),
            (
                "collinear prism",
                crate::test_support_fixtures::prism_z::<f64>(&run, 0.0, 1.0, tol).body,
                1,
            ),
        ] {
            assert_eq!(crate::validate_closed(&body), Ok(()), "{what}: tier 2");
            let fence = crate::validate::scaffolds_at_rest(&body);
            let check_two: Vec<_> = crate::validate_geometric(&body, tol)
                .err()
                .unwrap_or_default()
                .into_iter()
                .filter(|e| matches!(e, crate::ValidationError::ScaffoldAtRest { .. }))
                .collect();
            assert_eq!(fence, check_two, "{what}: the fence is check 2's");
            assert_eq!(fence.len(), count, "{what}: scaffolds at rest");
        }
    }

    /// An operand's own scaffold never reaches the door: the undescribed
    /// box is refused at the at-rest gate that finishes it, naming its
    /// twelve chords, and the described box answers through the door.
    #[test]
    fn an_operand_carried_scaffold_is_refused_at_the_at_rest_gate() {
        let tol = Tol::witness();
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let raw = quad_prism(&square, 1.0, tol);
        let errors = crate::AtRestBody::validate(raw.clone(), tol)
            .expect_err("the box's chords are scaffolds at rest");
        let mut named: Vec<_> = errors
            .iter()
            .filter_map(|e| match e {
                crate::ValidationError::ScaffoldAtRest { edge } => Some(*edge),
                _ => None,
            })
            .collect();
        named.sort();
        let mut edges: Vec<_> = raw.edges().map(|(k, _)| k).collect();
        edges.sort();
        assert_eq!(named, edges, "one finding per chord: {errors:?}");
        let far = finished(
            "far brick",
            crate::test_support_fixtures::brick::<f64>((5.0, 6.0), (0.0, 1.0), (0.0, 1.0), tol),
            tol,
        );
        let described = finished(
            "described box",
            crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
            tol,
        );
        assert!(
            crate::boolean::subtract(&described, &far, tol).is_ok(),
            "the described box answers"
        );
    }

    /// The backstop's refusal wiring, by construction: feed it a
    /// "result" whose exact volume violates the op's bound (a half-height
    /// prism vs the unit cube, both surface-certified geometric bodies)
    /// and require the typed `ResultVolumeImplausible`; the non-strict
    /// pass direction (result ≡ operand, margin exactly zero) must pass
    /// all three ops.
    #[test]
    fn volume_backstop_wiring() {
        let band = Band::linear(Tol::witness()).unwrap();
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let cube = quad_prism(&square, 1.0, Tol::witness());
        let small = quad_prism(&square, 0.5, Tol::witness());
        let implausible = |e: BooleanError| {
            assert!(
                matches!(e, BooleanError::ResultVolumeImplausible { .. }),
                "expected ResultVolumeImplausible, got {e:?}"
            );
        };
        // vol(∪) ≥ max: a union "result" smaller than an operand.
        implausible(
            volume_backstop(
                BooleanOp::Union,
                &cube,
                &cube,
                &small,
                band,
                Tol::witness(),
                QuadLane::certified(),
            )
            .unwrap_err(),
        );
        // vol(∖) ≤ vol(A): a subtract "result" larger than A.
        implausible(
            volume_backstop(
                BooleanOp::Subtract,
                &small,
                &cube,
                &cube,
                band,
                Tol::witness(),
                QuadLane::certified(),
            )
            .unwrap_err(),
        );
        // vol(∩) ≤ min: an intersect "result" larger than an operand.
        implausible(
            volume_backstop(
                BooleanOp::Intersect,
                &small,
                &cube,
                &cube,
                band,
                Tol::witness(),
                QuadLane::certified(),
            )
            .unwrap_err(),
        );
        // Equal volumes: every bound is non-strict — all pass.
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            volume_backstop(
                op,
                &cube,
                &cube,
                &cube,
                band,
                Tol::witness(),
                QuadLane::certified(),
            )
            .unwrap();
        }
        // Complement operand (negative flux volume): its bound is
        // vacuous and must be SKIPPED — A ∩ revert(B) legitimately
        // exceeds vol(revert B); the A-side bound still applies.
        let rev = quad_prism(&square, 0.5, Tol::witness()).revert();
        volume_backstop(
            BooleanOp::Intersect,
            &cube,
            &rev,
            &cube,
            band,
            Tol::witness(),
            QuadLane::certified(),
        )
        .unwrap();
        implausible(
            volume_backstop(
                BooleanOp::Intersect,
                &small,
                &rev,
                &cube,
                band,
                Tol::witness(),
                QuadLane::certified(),
            )
            .unwrap_err(),
        );
    }

    /// The arms the operands bound together, and the result's own sign:
    /// `vol(A ∪ B) ≤ vol(A) + vol(B)`, `vol(A ∖ B) ≥ vol(A) − vol(B)`,
    /// and a bounded result enclosing material. Each refuses a planted
    /// result that breaks it and only it, named by `which`.
    #[test]
    fn volume_backstop_joint_and_sign_arms() {
        let band = Band::linear(Tol::witness()).unwrap();
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let prism = |h: f64| quad_prism(&square, h, Tol::witness());
        let (cube, small, quarter, tall) = (prism(1.0), prism(0.5), prism(0.25), prism(1.5));
        let rev = cube.revert();
        let refuses = |op, a, b, result, which: &str| {
            let err = volume_backstop(
                op,
                a,
                b,
                result,
                band,
                Tol::witness(),
                QuadLane::certified(),
            )
            .unwrap_err();
            assert!(
                matches!(err, BooleanError::ResultVolumeImplausible { which: w, .. } if w == which),
                "expected {which}, got {err:?}"
            );
        };
        refuses(
            BooleanOp::Union,
            &small,
            &small,
            &tall,
            "vol(A ∪ B) ≤ vol(A) + vol(B)",
        );
        refuses(
            BooleanOp::Subtract,
            &cube,
            &small,
            &quarter,
            "vol(A ∖ B) ≥ vol(A) − vol(B)",
        );
        refuses(BooleanOp::Intersect, &cube, &cube, &rev, "vol(A ∩ B) ≥ 0");
        refuses(BooleanOp::Subtract, &cube, &small, &rev, "vol(A ∖ B) ≥ 0");
    }

    /// **A tie between congruent closed-form bodies passes.** One prism
    /// over a non-dyadic quadrilateral, its profile started at two
    /// different corners: the same solid, whose `f64` flux sums round
    /// apart (0.88374999999999970 against 0.88375000000000004 m³). Each
    /// bound reads the tie through the interval re-derivation and
    /// passes, in both orders; a result the walk alone would call past
    /// its bound.
    #[test]
    fn volume_backstop_passes_a_closed_form_rounding_tie() {
        let band = Band::linear(Tol::witness()).unwrap();
        let profile = [(0.1, 0.2), (1.3, 0.25), (1.1, 1.7), (0.05, 0.9)];
        let mut turned = profile;
        turned.rotate_left(3);
        let (low, high) = (
            quad_prism(&profile, 0.7, Tol::witness()),
            quad_prism(&turned, 0.7, Tol::witness()),
        );
        let volume = |body| crate::mass_properties(body, Tol::witness()).unwrap().volume;
        assert!(
            volume(&low) < volume(&high),
            "the two corners' sums round apart: {} vs {}",
            volume(&low),
            volume(&high)
        );
        for (op, operand, result) in [
            (BooleanOp::Intersect, &low, &high),
            (BooleanOp::Subtract, &low, &high),
            (BooleanOp::Union, &high, &low),
        ] {
            let got = volume_backstop(
                op,
                operand,
                operand,
                result,
                band,
                Tol::witness(),
                QuadLane::certified(),
            );
            assert!(got.is_ok(), "{op:?}: {got:?}");
        }
    }

    /// **The +V arm reads tier 3's rule**: `V/A` against the band, a
    /// negative certified past it refuses and an in-band one is exempt.
    /// An intersect of two unit cubes whose "result" is one body of two
    /// shells, a reverted unit cube and a cube of side `s` beside it, so
    /// `V = s³ − 1` over `A ≈ 12 m²`: at `V/A = −ε/2` it is in band and
    /// passes, and one step past the band's edge (`−1.01·escalate`) and
    /// well past it (`−4·escalate`) it refuses.
    #[test]
    fn volume_backstop_reads_the_sign_against_the_band() {
        use crate::test_support_fixtures::{cube_into, mapped_cube};
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let unit = |x0: f64, side: f64| {
            move |u: f64, v: f64, w: f64| geom_core::Point3::new(x0 + side * u, side * v, side * w)
        };
        let cube = mapped_cube::<f64>(unit(0.0, 1.0), tol);
        let run = |v_over_a: f64| {
            // `V = s³ − 1` over `A = 6 + 6s²`.
            let s = (1.0 + 12.0 * v_over_a).cbrt();
            let mut result = mapped_cube::<f64>(unit(0.0, 1.0), tol).revert();
            cube_into(&mut result, unit(5.0, s), tol);
            volume_backstop(
                BooleanOp::Intersect,
                &cube,
                &cube,
                &result,
                band,
                tol,
                QuadLane::certified(),
            )
        };
        let in_band = run(-0.5 * band.zero());
        assert!(in_band.is_ok(), "in band: {in_band:?}");
        for past in [1.01, 4.0] {
            assert!(
                matches!(
                    run(-past * band.escalate()),
                    Err(BooleanError::ResultVolumeImplausible {
                        which: "vol(A ∩ B) ≥ 0",
                        ..
                    })
                ),
                "{past}·escalate past the band"
            );
        }
    }

    /// **The #200 review's MAJ-1, end to end through the real gate.**
    /// A wrongly-kept 3 mm cube on a 2 m × 2 m × 0.1 m plate: the
    /// violation is ΔV = 2.7e-8 m³ spread over ~17.6 m² of boundary, so
    /// the METERED mean displacement is 1.53e-9 m — inside the default
    /// `Band{1e-9, 1e-8}`, i.e. exactly the region where a
    /// magnitude-only gate passes a macroscopic wrong component. The
    /// sign arm (`volume_backstop_violation`, exact band) certifies the
    /// inequality is violated regardless, so the gate REFUSES.
    ///
    /// This is the pin on the dual-arm structure: delete arm 1 and this
    /// test goes red, the one row whose violation hides inside the band
    /// by its area (the other planted-violation rows red with it).
    #[test]
    fn volume_backstop_refuses_a_wrong_component_hidden_by_a_large_area() {
        let band = Band::linear(Tol::witness()).unwrap();
        let plate_profile = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
        let plate = quad_prism(&plate_profile, 0.1, Tol::witness());
        // The "result" of `plate ∖ tool` that wrongly KEPT a 3 mm cube:
        // same footprint, 2.7e-8 m³ of extra material (a 6.75e-9 m lift
        // over the 4 m² footprint — the volume of a 3 mm cube).
        let kept = 0.003_f64.powi(3);
        let wrong = quad_prism(&plate_profile, 0.1 + kept / 4.0, Tol::witness());
        let err = volume_backstop(
            BooleanOp::Subtract,
            &plate,
            &plate,
            &wrong,
            band,
            Tol::witness(),
            QuadLane::certified(),
        )
        .unwrap_err();
        assert!(
            matches!(err, BooleanError::ResultVolumeImplausible { .. }),
            "a macroscopic wrong component must refuse however much \
             boundary area it is smeared over, got {err:?}"
        );
        // The exact-zero pass direction is untouched by the sign arm.
        volume_backstop(
            BooleanOp::Subtract,
            &plate,
            &plate,
            &plate,
            band,
            Tol::witness(),
            QuadLane::certified(),
        )
        .unwrap();
    }

    /// **The NURBS re-gate, pinned (M5 S13 §1), and the placeholder's
    /// own door.** The fallback's curved-extent test is UNWRITABLE for
    /// NURBS today (`implicit_residual` is poison there, and no
    /// projection-based extent argument has been written — the
    /// `NurbsSurface::project` half of the old blocker retired at
    /// M6-2's lift), so the class is re-gated AT THE FALLBACK with a
    /// typed refusal naming its TRUE blocker — a future NURBS body
    /// constructor inherits this door, never the vertex-probe silence
    /// the S12 finding executed. That re-gate is pinned here at the
    /// mechanism, because a body on the `mvfs` `Nurbs` PLACEHOLDER
    /// surface cannot reach the boolean at all: it is not a finished
    /// body, and the at-rest gate refuses each of its faces as
    /// uncertifiable. Both are pinned below; what neither may become is
    /// a silent assembly.
    #[test]
    fn nurbs_faces_refuse_typed_at_both_doors() {
        use crate::test_support_fixtures::{FaceGeometry, UNIT_SQUARE, declined_cube, prism_ops};
        use geom_core::Point3;

        // `declined_cube`, x-shifted: the same builder under one
        // translating map. The fixture is anchored at the origin, and
        // this operand needs a disjoint certified box so the realized
        // sweep examines no pair at all.
        let far_cube = |dx: f64| {
            let mut body = crate::Body::<f64>::new();
            prism_ops(
                &mut body,
                &UNIT_SQUARE,
                (0.0, 1.0),
                |x, y, z| Point3::new(x + dx, y, z),
                FaceGeometry::Declined,
                Tol::witness(),
            );
            body
        };

        let a = declined_cube::<f64>(Tol::witness()).body;
        let b = far_cube(10.0);
        // Door 1 — the placeholder is not a finished body: the at-rest
        // gate that would hand it to the boolean refuses each of its
        // faces as uncertifiable, so no NURBS placeholder reaches a pair.
        for (what, body) in [("the declined cube", &a), ("the far declined cube", &b)] {
            let errors = crate::AtRestBody::validate(body.clone(), Tol::witness())
                .expect_err("a NURBS placeholder is not a finished body");
            let mut uncertifiable: Vec<_> = errors
                .iter()
                .filter_map(|e| match e {
                    crate::ValidationError::UncertifiableSurface { face } => Some(*face),
                    _ => None,
                })
                .collect();
            uncertifiable.sort();
            let mut faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
            faces.sort();
            assert_eq!(uncertifiable, faces, "{what}: every face named: {errors:?}");
        }

        // Door 2 — the fallback's own re-gate, at the mechanism: any
        // fallback entry carrying a NURBS face refuses BEFORE a vertex
        // is probed. NO end-to-end path reaches it today (a lofted
        // operand is refused at its NURBS EDGES first, a placeholder at
        // the at-rest gate) — `sweep`'s `s16_box_soundness` pins the
        // first and Door 1 above the second, so the day one lifts is
        // loud.
        let band = Band::linear(Tol::witness()).unwrap();
        let Err(err) = super::sphere_extent_scan(&a, &b, &[], band) else {
            panic!("the NURBS fallback must be re-gated, never vertex-probed");
        };
        let BooleanError::NurbsExtentUnsupported { .. } = err else {
            panic!("expected the NURBS re-gate, got {err:?}");
        };
        // The refusal names the face kind that stopped it and ends on
        // what the person can do; the lift blocker is the variant's
        // rustdoc, not the sentence.
        let msg = err.to_string();
        assert!(msg.contains("spline (NURBS) face"), "{msg}");
        assert!(msg.contains("Recourse: "), "{msg}");
    }

    /// The D5 descendant chase, pinned at the mechanism level (M3
    /// PR 6a): a v-on-f record whose FACE key is dead (an absorbed
    /// merge fragment — realized here with a foreign-arena key, the
    /// same dead-key shape) survives [`super::carry`] when the
    /// descendant map names its surviving fragment, and drops without
    /// the row — record loss over a live coincidence is exactly what
    /// the map exists to prevent (PR 5 review R5). The v-v lane's
    /// consumed-pair rule is pinned too: a pair fused into ONE vertex
    /// drops.
    #[test]
    fn descendant_chase_wiring() {
        use super::{Descendants, KeyView};
        use crate::boolean::{ContactKind, ReductionContacts, VfContact, VvContact};

        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let mut body = quad_prism(&square, 1.0, Tol::witness());
        let live_vertex = body.vertices().next().map(|(k, _)| k).unwrap();
        let live_face = body.faces().next().map(|(k, _)| k).unwrap();
        // Dead keys: arena entries removed in place (the test only
        // exercises key lookups; tier validity is irrelevant here).
        let dead_face = body.faces().map(|(k, _)| k).nth(5).unwrap();
        body.faces.remove(dead_face);
        assert!(body.get_face(dead_face).is_none(), "key is dead");

        let contacts = ReductionContacts {
            vv: vec![],
            a_on_b: vec![crate::Cited::new(
                VfContact {
                    vertex: live_vertex,
                    face: dead_face,
                },
                crate::Cites::decided(0),
            )],
            b_on_a: vec![],
            ee: vec![],
        };
        // Without the descendant row: the record drops (pre-D5 loss).
        let out = found(
            &body,
            &contacts,
            [&KeyView::Direct; 2],
            &Descendants::default(),
        )
        .unwrap();
        assert!(out.is_empty());
        // With the row: the record survives, renamed to the survivor.
        let mut desc = Descendants::default();
        desc.faces.insert(dead_face, live_face);
        let out = found(&body, &contacts, [&KeyView::Direct; 2], &desc).unwrap();
        assert_eq!(
            out.kinds().collect::<Vec<_>>(),
            [ContactKind::VertexFace(live_vertex, live_face)]
        );

        // v-v consumed-pair rule: both sides chased into one vertex ⇒
        // the coincidence is structural now, the record drops.
        let dead_vertex = body.vertices().map(|(k, _)| k).nth(3).unwrap();
        assert_ne!(dead_vertex, live_vertex);
        body.vertices.remove(dead_vertex);
        let contacts = ReductionContacts {
            vv: vec![crate::Cited::new(
                VvContact {
                    a: dead_vertex,
                    b: live_vertex,
                },
                crate::Cites::decided(0),
            )],
            a_on_b: vec![],
            b_on_a: vec![],
            ee: vec![],
        };
        let mut desc = Descendants::default();
        desc.vertices.push((dead_vertex, live_vertex)).unwrap();
        let out = found(&body, &contacts, [&KeyView::Direct; 2], &desc).unwrap();
        assert!(out.is_empty(), "fused-into-one pair is consumed");
    }

    /// **A fusion chase that ends on a key dead in the body drops the
    /// record, and a fusion the map refuses fails its absorb.** The row
    /// `(c, a)` is sound as a list, but `a` is gone from the body: both
    /// remaps drop the record. Red if a dead end refuses or keeps it, or
    /// if [`Descendants::absorb_fusions`] swallows its list's refusal of
    /// a row keeping the killed `a`.
    #[test]
    fn a_dead_end_drops_and_a_refused_fusion_fails_its_absorb() {
        use super::{Descendants, KeyView};
        use crate::boolean::{CarriedRecord, Glue, OperandRecords, ReductionContacts, VvContact};
        use crate::entity::VertexKey;

        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let mut body = quad_prism(&square, 1.0, Tol::witness());
        let keys: Vec<VertexKey> = body.vertices().map(|(k, _)| k).collect();
        let (a, b, c, partner) = (keys[0], keys[1], keys[2], keys[3]);
        body.vertices.remove(a);
        body.vertices.remove(c);
        let pair = VvContact { a: c, b: partner };
        let contacts = ReductionContacts {
            vv: vec![crate::Cited::new(pair, crate::Cites::decided(0))],
            ..ReductionContacts::default()
        };
        let decls = Glue {
            records_a: OperandRecords {
                vv: vec![CarriedRecord {
                    contact: pair,
                    record: 0,
                }],
                ..OperandRecords::default()
            },
            ..Glue::default()
        };

        let mut dead_end = Descendants::default();
        dead_end.vertices.push((c, a)).unwrap();
        let dropped = found(&body, &contacts, [&KeyView::Direct; 2], &dead_end)
            .expect("a well-ordered list remaps");
        assert!(dropped.is_empty(), "dead end: {dropped:?}");
        let carried = carried_in(&body, &decls, [&KeyView::Direct; 2], &dead_end)
            .expect("a well-ordered list remaps");
        assert!(carried.is_empty(), "carried dead end: {carried:?}");

        let mut corrupt = Descendants::default();
        corrupt.vertices.push((a, b)).unwrap();
        let mut later = super::Fusions::default();
        later.push((c, a)).unwrap();
        assert!(
            matches!(
                corrupt.absorb_fusions(&later),
                Err(BooleanError::JoinDesync {
                    what: "a fusion row names a key an earlier row killed"
                })
            ),
            "absorbing a row that keeps the killed a refuses"
        );
    }

    /// **A cycle in the face absorption rows refuses; a chain that ends
    /// at a dead key drops the record.** The two dead faces name each
    /// other, so the chase never reaches a live face nor a key without
    /// a row: it revisits, and a revisit is a corrupt record. Read as
    /// "consumed", it would drop a declared contact without a word —
    /// the dead end's answer, which this pins as different.
    #[test]
    fn a_cycling_absorption_row_refuses_where_a_dead_end_drops() {
        use super::{Descendants, KeyView};
        use crate::boolean::{CarriedRecord, Glue, OperandRecords, ReductionContacts, VfContact};
        use crate::entity::FaceKey;

        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let mut body = quad_prism(&square, 1.0, Tol::witness());
        let vertex = body.vertices().next().map(|(k, _)| k).unwrap();
        let keys: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        let (d0, d1) = (keys[4], keys[5]);
        body.faces.remove(d0);
        body.faces.remove(d1);
        let contacts = ReductionContacts {
            a_on_b: vec![crate::Cited::new(
                VfContact { vertex, face: d0 },
                crate::Cites::decided(0),
            )],
            ..ReductionContacts::default()
        };
        let remap = |desc: &Descendants| found(&body, &contacts, [&KeyView::Direct; 2], desc);

        let mut dead_end = Descendants::default();
        dead_end.faces.insert(d0, d1);
        let dropped = remap(&dead_end).expect("a chain that ends is not corrupt");
        assert!(dropped.is_empty(), "dead end: {dropped:?}");

        let mut cycle = Descendants::default();
        cycle.faces.insert(d0, d1);
        cycle.faces.insert(d1, d0);
        let joined = |r: Result<_, BooleanError>| match r {
            Err(BooleanError::JoinDesync { what }) => what,
            other => panic!("a cycle must refuse JoinDesync, got {other:?}"),
        };
        assert_eq!(
            joined(remap(&cycle).map(|_| ())),
            "a cell's substitution rows are cyclic",
            "discovered records on a cycle"
        );

        // The carried lane reads the same chase.
        let decls = Glue {
            records_a: OperandRecords {
                vf: vec![CarriedRecord {
                    contact: VfContact { vertex, face: d0 },
                    record: 0,
                }],
                ..OperandRecords::default()
            },
            ..Glue::default()
        };
        let carry = |desc: &Descendants| carried_in(&body, &decls, [&KeyView::Direct; 2], desc);
        let carried = carry(&dead_end).expect("a chain that ends is not corrupt");
        assert!(carried.is_empty(), "carried dead end: {carried:?}");
        assert_eq!(
            joined(carry(&cycle).map(|_| ())),
            "a cell's substitution rows are cyclic",
            "carried records on a cycle"
        );
    }

    /// **A record citing a vertex the merge pruned lands on the face
    /// that swallowed it, through the merge's REAL outcome.** The records cannot reach this point in a
    /// boolean by construction, so the row drives the rule itself with
    /// a real pruning merge instead:
    ///
    /// - A vertex the pruning deletes is a junction of the seam the
    ///   zip laid in the merged plane — a vertex interior to the other
    ///   operand's coplanar face, where its cut turns. A vertex-on-face
    ///   rest there is one the zip fused into seam structure, so it
    ///   has already dropped as fused before the merge runs.
    /// - A vertex-vertex record there names the other operand's vertex
    ///   at the same point, which the zip fuses into it too, so the
    ///   pair is one vertex and consumed.
    ///
    /// What is left for the merge to get wrong is the descendant
    /// chase: a fusion row whose survivor the pruning then deletes. So
    /// the row takes a prism whose top is split by a seam bent at an
    /// interior vertex, runs the real `merge_coplanar_faces`, absorbs
    /// its outcome into [`Descendants`] exactly as the boolean does,
    /// adds the fusion row a zip would have written into the deleted
    /// vertex, and requires a v-v record citing either key to become
    /// the other vertex resting on the kept face. Red when the merge's
    /// pruned vertices are left out of the substitution rows (the
    /// record drops) or mapped to a vertex.
    /// **Two live chords for one edge refuse.** The join's chords along
    /// an edge each hold its interior where the op drops it
    /// ([`Descendants::live`]); two live ones that are not one edge
    /// would mean two edges hold one interior. Red when the read takes
    /// the first live chord, or panics.
    #[test]
    fn two_live_chords_along_one_dropped_edge_refuse_typed() {
        use super::{Cell, Descendants, KeyView, Operand};
        use crate::entity::EdgeKey;

        let tol = Tol::witness();
        let body = crate::test_support_fixtures::prism_z::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            0.0,
            1.0,
            tol,
        )
        .body;
        let mut edges = body.edges().map(|(k, _)| k);
        let (c1, c2) = (edges.next().unwrap(), edges.next().unwrap());
        // A key no live edge has: the dropped edge.
        let gone = EdgeKey::default();
        assert!(body.get_edge(gone).is_none());
        let one = Descendants::default().with_along([vec![(gone, c1)], Vec::new()]);
        assert_eq!(
            one.live(&body, Operand::A, &KeyView::Direct, Cell::Edge(gone))
                .unwrap(),
            Some(Cell::Edge(c1)),
            "one live chord holds the dropped edge"
        );
        let two = Descendants::default().with_along([vec![(gone, c1), (gone, c2)], Vec::new()]);
        assert!(
            matches!(
                two.live(&body, Operand::A, &KeyView::Direct, Cell::Edge(gone)),
                Err(BooleanError::JoinDesync { .. })
            ),
            "two live chords refuse typed"
        );
    }

    #[test]
    fn a_record_citing_a_pruned_free_end_lands_on_the_face_that_swallowed_it() {
        use super::{Cell, Descendants, KeyView, Operand};
        use crate::boolean::{
            Contact, ContactKind, ContactRecords, ReductionContacts, VfContact, VvContact,
        };
        use crate::entity::VertexKey;
        use crate::{MefSite, MevSite};
        use geom_core::Point3;

        let tol = Tol::witness();
        let p = crate::test_support_fixtures::prism_z::<f64>(
            &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
            0.0,
            1.0,
            tol,
        );
        let mut body = p.body;
        let corner = |body: &crate::Body<f64>, x: f64, y: f64| {
            let outer = body.get_face(p.top_face).unwrap().outer;
            let crate::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary
            else {
                panic!("the top is a cycle")
            };
            body.loop_cycle(first)
                .unwrap()
                .into_iter()
                .find(|&he| {
                    let v = body.get_half_edge(he).unwrap().start;
                    let q = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                    (q.x, q.y) == (x, y)
                })
                .unwrap()
        };
        let he1 = corner(&body, 0.0, 0.0);
        let strut = body
            .mev_line(
                MevSite::Fan { he1, he2: he1 },
                Point3::new(0.9, 0.6, 1.0),
                tol,
            )
            .unwrap();
        let he2 = corner(&body, 2.0, 2.0);
        body.mef_chord(
            MefSite::Chords {
                he1: strut.he_minus,
                he2,
            },
            tol,
        )
        .unwrap();
        let bend = strut.vertex;

        let merged = body
            .merge_coplanar_faces(tol)
            .expect("the bent seam repairs");
        let killed: Vec<VertexKey> = merged
            .groups
            .iter()
            .flat_map(|g| g.killed_vertices.iter().copied())
            .collect();
        assert_eq!(killed, vec![bend], "the bend is the pruned free end");
        assert!(body.get_vertex(bend).is_none());

        let mut desc = Descendants::default();
        desc.absorb_merge(&merged);
        // The row the zip writes when it fuses a coincident vertex
        // into the one the merge later deletes; the fused-in key is
        // dead (the null key, which no arena holds).
        let fused_in = VertexKey::default();
        // A floor vertex: on no boundary of the kept top.
        let survivor = body
            .vertices()
            .find(|(_, v)| body.get_point(v.point).unwrap().z == 0.0)
            .map(|(k, _)| k)
            .unwrap();
        let kept = merged.groups[0].kept;
        let face = body.faces().map(|(k, _)| k).find(|&f| f != kept).unwrap();
        // The seam edges the glue killed land on the kept face too: a
        // `(vertex, edge)` record naming one becomes the vertex resting
        // on that face. Red when the merge's killed edges get no
        // substitution row (the record drops).
        let killed_edges = &merged.groups[0].killed_edges;
        assert!(!killed_edges.is_empty(), "the glue killed the seam's edges");
        for &edge in killed_edges {
            let on_edge: ContactRecords = [crate::Cited::new(
                Contact::of(Cell::Vertex(survivor), Cell::Edge(edge)).unwrap(),
                crate::Cites::decided(0),
            )]
            .into_iter()
            .collect();
            assert_eq!(
                super::carry_in_place(&body, &on_edge, &desc, band())
                    .unwrap()
                    .kinds()
                    .collect::<Vec<_>>(),
                [ContactKind::VertexFace(survivor, kept)],
                "{edge:?}: the record lands on the kept face"
            );
        }
        desc.vertices.push((fused_in, bend)).unwrap();
        desc.fused.insert(bend);
        let lands = |v| desc.live(&body, Operand::A, &KeyView::Direct, Cell::Vertex(v));
        assert_eq!(
            lands(bend).unwrap(),
            Some(Cell::Face(kept)),
            "{bend:?} lands on the kept face"
        );

        let records = |v: VertexKey| ReductionContacts {
            vv: vec![crate::Cited::new(
                VvContact { a: v, b: survivor },
                crate::Cites::decided(0),
            )],
            a_on_b: vec![crate::Cited::new(
                VfContact { vertex: v, face },
                crate::Cites::decided(0),
            )],
            b_on_a: vec![crate::Cited::new(
                VfContact { vertex: v, face },
                crate::Cites::decided(0),
            )],
            ee: vec![],
        };
        let remap = |c: &ReductionContacts| {
            found(&body, c, [&KeyView::Direct; 2], &desc)
                .unwrap()
                .kinds()
                .collect::<Vec<_>>()
        };
        assert_eq!(
            lands(fused_in).unwrap(),
            Some(Cell::Face(kept)),
            "the chase through the fusion lands too"
        );
        // The fused vertex's own rest is the zip's incidence and drops;
        // the key fused into it, chased onto the kept face, leaves the
        // two faces meeting at its point.
        assert_eq!(
            remap(&records(bend)),
            [ContactKind::VertexFace(survivor, kept)],
            "{bend:?}: the partner rests on the face that swallowed it"
        );
        assert_eq!(
            remap(&records(fused_in)),
            [
                ContactKind::VertexFace(survivor, kept),
                ContactKind::FaceFace(kept.min(face), kept.max(face)),
            ],
            "{fused_in:?}: the partner rests on the face that swallowed it, and the \
             chased rest is the two faces meeting there"
        );
        // The control: the same records on a live vertex carry.
        let other = body
            .vertices()
            .map(|(k, _)| k)
            .find(|&k| {
                k != survivor && Contact::new(&body, Cell::Vertex(k), Cell::Face(face)).is_ok()
            })
            .unwrap();
        assert_eq!(
            remap(&records(other)),
            [
                ContactKind::VertexVertex(other.min(survivor), other.max(survivor)),
                ContactKind::VertexFace(other, face),
            ]
        );
    }

    /// **A face pair's zip is the reassembly oracle's sequence**: the
    /// seams of brick pairs (a pierce, a corner overlap, a slot through)
    /// under every op, each zipped by `zip_seam` and, on a copy, by the
    /// splitting oracle's `reglue_pair`, which aligns the cycles by
    /// bit-equal points and runs the loopglue sequence itself. The two
    /// bodies' topology (faces, loops, half-edges, vertices) is the
    /// same, key for key. Red if `glue`'s joints or kills drift from that
    /// sequence.
    #[test]
    fn a_face_pair_zip_is_the_reassembly_oracles_sequence() {
        use super::{Joined, SweepStrategy, fused_through, through_the_join, zip_seam};
        use crate::boolean::BooleanOp;
        use crate::test_support_fixtures::brick;
        let tol = Tol::witness();
        let topology = |body: &crate::Body<f64>| {
            format!(
                "{:?}{:?}{:?}{:?}",
                body.faces, body.loops, body.half_edges, body.vertices
            )
        };
        let mut seams = 0;
        for (label, a, b) in [
            (
                "pierce",
                brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
                brick::<f64>((0.5, 1.5), (0.2, 0.8), (0.2, 0.8), tol),
            ),
            (
                "corner",
                brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
                brick::<f64>((0.5, 1.5), (0.5, 1.5), (0.5, 1.5), tol),
            ),
            (
                "slot",
                brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
                brick::<f64>((-0.5, 1.5), (0.3, 0.6), (0.4, 1.5), tol),
            ),
        ] {
            for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
                let decls = super::Glue::none();
                let Joined::Connected { red, connected, .. } =
                    through_the_join(op, &a, &b, &decls, SweepStrategy::Realized, true, tol)
                        .unwrap()
                else {
                    panic!("{label} {op:?}: the op joins");
                };
                let fin = super::setopfinish(op, *red, &connected, &a, &b, band(), tol).unwrap();
                let mut body = fin.body;
                let mut vertex_map = fin.vertex_map.clone();
                for &(a_face, b_face) in &fin.seams {
                    let mut oracle = body.clone();
                    crate::splitting::reassembly::reglue_pair(&mut oracle, a_face, b_face, tol);
                    let rep = zip_seam(&mut body, a_face, b_face, &vertex_map, tol).unwrap();
                    assert_eq!(
                        topology(&body),
                        topology(&oracle),
                        "{label} {op:?}: seam ({a_face:?}, {b_face:?}) zips as the oracle does"
                    );
                    vertex_map = fused_through(&vertex_map, &rep.vertex_merges);
                    seams += 1;
                }
            }
        }
        assert!(seams >= 9, "a seam per op at least: {seams}");
    }

    /// **Each zip edge row pairs a ring edge with the seam edge it lies
    /// on.** The real pipeline up to the zips, on a pierce and on a
    /// flush overlap: before each zip, every row's two edges span one
    /// segment (the same two end points). Red when a ring edge is
    /// paired with the seam edge of its own index: `align` runs `rs[j]`
    /// between the correspondents of `ob[j]`'s start and `ob[j − 1]`'s,
    /// so its segment is `ob[j − 1]`'s, and a record naming the ring
    /// edge would land on a neighbouring edge.
    #[test]
    fn every_zip_edge_row_pairs_a_ring_edge_with_the_seam_edge_it_lies_on() {
        use super::{Joined, SweepStrategy, fused_through, through_the_join, zip_seam};
        use crate::boolean::BooleanOp;
        use crate::test_support_fixtures::brick;
        let tol = Tol::witness();
        let ends = |body: &crate::Body<f64>, e: crate::entity::EdgeKey| {
            let d = body.get_edge(e).unwrap();
            let mut ps = [d.he_plus, d.he_minus].map(|h| {
                let v = body.get_half_edge(h).unwrap().start;
                let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                [p.x, p.y, p.z]
            });
            ps.sort_by(|p, q| p.partial_cmp(q).unwrap());
            ps
        };
        let near = |p: [[f64; 3]; 2], q: [[f64; 3]; 2]| {
            p.iter()
                .flatten()
                .zip(q.iter().flatten())
                .all(|(x, y)| (x - y).abs() < 1e-9)
        };
        let mut rows = 0;
        for (label, a, b) in [
            (
                "pierce",
                brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
                brick::<f64>((0.5, 1.5), (0.2, 0.8), (0.2, 0.8), tol),
            ),
            (
                "corner",
                brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
                brick::<f64>((0.5, 1.5), (0.5, 1.5), (0.5, 1.5), tol),
            ),
        ] {
            let decls = super::Glue::none();
            let Joined::Connected { red, connected, .. } = through_the_join(
                BooleanOp::Union,
                &a,
                &b,
                &decls,
                SweepStrategy::Realized,
                true,
                tol,
            )
            .unwrap() else {
                panic!("{label}: the union joins");
            };
            let fin = super::setopfinish(BooleanOp::Union, *red, &connected, &a, &b, band(), tol)
                .unwrap();
            let mut body = fin.body;
            let mut vertex_map = fin.vertex_map.clone();
            for &(a_face, b_face) in &fin.seams {
                let before = body.clone();
                let rep = zip_seam(&mut body, a_face, b_face, &vertex_map, tol).unwrap();
                assert!(
                    !rep.edge_merges.is_empty(),
                    "{label}: the zip kills ring edges"
                );
                for &(dead, kept) in &rep.edge_merges {
                    assert!(
                        near(ends(&before, dead), ends(&before, kept)),
                        "{label}: ring edge {dead:?} {:?} paired with seam edge {kept:?} {:?}",
                        ends(&before, dead),
                        ends(&before, kept)
                    );
                    rows += 1;
                }
                vertex_map = fused_through(&vertex_map, &rep.vertex_merges);
            }
        }
        assert!(
            rows >= 8,
            "two section polygons of four edges each at least: {rows}"
        );
    }

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A margin inside the band at every ε row: its two edges'
    /// geometric mean.
    fn in_band() -> f64 {
        (band().zero() * band().escalate()).sqrt()
    }

    /// The floor `z = 0` and a unit cylinder resting on it along the
    /// `y` axis, which is their tangency ruling: `κ_rel = 1` across it.
    fn resting() -> (geom::Surface<f64>, geom::Surface<f64>, geom::Curve3<f64>) {
        let floor = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let cylinder = geom::Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0),
            axis: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let ruling = geom::Curve3::Line {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 1.0, 0.0),
        };
        (floor, cylinder, ruling)
    }

    /// The seam's answer over `[0, extent]` of `carrier`, in both
    /// argument orders, with its witness read smooth first (the arm the
    /// rule is asked from).
    fn both_orders(
        s1: &geom::Surface<f64>,
        s2: &geom::Surface<f64>,
        carrier: &geom::Curve3<f64>,
        (t0, t1): (f64, f64),
        extent: f64,
    ) -> [Result<bool, BooleanError>; 2] {
        let witness = carrier.eval((t0 + t1) / 2.0);
        [(s1, s2), (s2, s1)].map(|(a, b)| {
            assert!(
                matches!(
                    seam_class(a, b, witness, extent, band()),
                    Ok(geom_brep::DihedralClass::Smooth)
                ),
                "the row's seam reads smooth at its witness"
            );
            seam_must_carry(a, b, carrier, t0, t1, extent, band())
        })
    }

    /// **A smooth seam whose bend is in band refuses as `SeamJet`, in
    /// both orders**: its stations' second-order sagitta is certifiable
    /// as neither the intrinsic tangency nor the conventional posture,
    /// so it is never stored as either (tier 3 refuses such an edge
    /// `SliverDihedral`).
    #[test]
    fn a_seam_bending_apart_in_band_refuses_as_its_second_order_decision() {
        let (floor, cylinder, ruling) = resting();
        // `κ_rel · extent² / 2` with `κ_rel = 1` and the arm the extent.
        let extent = (2.0 * in_band()).sqrt();
        for (order, got) in both_orders(&floor, &cylinder, &ruling, (0.0, extent), extent)
            .into_iter()
            .enumerate()
        {
            match got {
                Err(BooleanError::Escalated {
                    decision: BooleanDecision::SeamJet,
                    diag,
                }) => assert_eq!(
                    diag.predicate,
                    Some("tangent_second_order"),
                    "order {order}: the escalation is the station's sagitta"
                ),
                other => panic!("order {order}: an in-band bend must refuse typed, got {other:?}"),
            }
        }
    }

    /// **A station whose first-order dihedral is in band refuses as the
    /// seam's own lever**, the rung `seam_class` gives the same reading
    /// at the witness: over an extent in band the arm cannot measure the
    /// angle.
    #[test]
    fn a_seam_whose_station_arm_is_in_band_refuses_as_the_seam_lever() {
        let (floor, cylinder, ruling) = resting();
        let extent = in_band();
        for (a, b) in [(&floor, &cylinder), (&cylinder, &floor)] {
            let got = seam_must_carry(a, b, &ruling, 0.0, 1.0, extent, band());
            assert!(
                matches!(
                    got,
                    Err(BooleanError::Escalated {
                        decision: BooleanDecision::LeverArm(LeverArm::Seam),
                        ..
                    })
                ),
                "an in-band arm is the seam lever's refusal, got {got:?}"
            );
        }
    }

    /// **A determinate seam is intrinsic, a flush one conventional**:
    /// the resting cylinder over a unit extent bends apart definitely,
    /// and two coplanar planes split along a line bend apart not at
    /// all — exactly zero, so the conventional chord stays.
    #[test]
    fn a_determinate_seam_is_intrinsic_and_a_flush_one_conventional() {
        let (floor, cylinder, ruling) = resting();
        for (order, got) in both_orders(&floor, &cylinder, &ruling, (0.0, 1.0), 1.0)
            .into_iter()
            .enumerate()
        {
            assert!(
                matches!(got, Ok(true)),
                "order {order}: a definite bend demands the intrinsic tangency, got {got:?}"
            );
        }
        let split = geom::Surface::Plane {
            origin: Point3::new(0.0, 5.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        };
        for (order, got) in both_orders(&floor, &split, &ruling, (0.0, 1.0), 1.0)
            .into_iter()
            .enumerate()
        {
            assert!(
                matches!(got, Ok(false)),
                "order {order}: coplanar planes keep the conventional chord, got {got:?}"
            );
        }
    }

    /// A torus (`R = 2`, `r = 1`) about `z`, its bitangent plane, and the
    /// Villarceau circle they share through the plane's point of
    /// tangency, which is the circle's `θ = 0`: the plane touches the
    /// torus there and crosses it everywhere else on the circle.
    fn villarceau() -> (geom::Surface<f64>, geom::Surface<f64>, geom::Curve3<f64>) {
        let (big, r) = (2.0_f64, 1.0_f64);
        let (sin, cos) = (r / big, (1.0 - (r / big).powi(2)).sqrt());
        let torus = geom::Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: big,
            minor_radius: r,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let normal = Vec3::new(sin, 0.0, -cos);
        let bitangent = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal,
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        };
        let center = Point3::new(0.0, r, 0.0);
        let touch = Point3::new(big - r * sin, 0.0, r * cos);
        let circle = geom::Curve3::Circle {
            center,
            axis: normal,
            radius: big,
            u_ref: (touch - center) / big,
        };
        for i in 0..geom_brep::CERT_SAMPLES {
            let p = circle.eval(geom_brep::sample_param(-1.0, 1.0, i));
            let on = |s: &geom::Surface<f64>| geom_brep::implicit_residual(s, p).abs() < 1e-12;
            assert!(
                on(&torus) && on(&bitangent),
                "the circle lies on both surfaces at {p:?}"
            );
        }
        (torus, bitangent, circle)
    }

    /// **A seam smooth at its witness and a corner at a station keeps
    /// the conventional posture, in both orders**, on [`villarceau`]'s
    /// arc a radian either side of the tangency. Each station's jet read
    /// off a smooth join measures a transverse direction tangent to the
    /// first surface alone, so the per-station sagitta reads definitely
    /// positive in both orders here, at values that differ with the
    /// order; the rule reads the corner first-order instead, and tier 3
    /// holds an edge that is not smooth throughout to neither
    /// description, so no intrinsic tangency is demanded of it.
    #[test]
    fn a_seam_smooth_at_its_witness_and_a_corner_at_a_station_stays_conventional() {
        let (torus, bitangent, arc) = villarceau();
        let span = (-1.0, 1.0);
        assert!(
            matches!(
                seam_class(&torus, &bitangent, arc.eval(span.0), 1.0, band()),
                Ok(geom_brep::DihedralClass::Transverse)
            ),
            "the arc's end is a corner"
        );
        for (a, b) in [(&torus, &bitangent), (&bitangent, &torus)] {
            assert_eq!(
                geom_brep::must_carry_over_edge(a, b, &arc, span.0, span.1, 1.0, band()),
                geom_brep::MustCarryVerdict::Transverse,
                "a station reads the seam a corner"
            );
        }
        for (order, got) in both_orders(&torus, &bitangent, &arc, span, 1.0)
            .into_iter()
            .enumerate()
        {
            assert!(
                matches!(got, Ok(false)),
                "order {order}: a seam that is a corner somewhere keeps the conventional \
                 description, got {got:?}"
            );
        }
    }

    /// **A station whose wedge is in band refuses as `SeamWedge`, even
    /// behind a station that reads a corner.** [`villarceau`]'s arc over
    /// a span short enough that the stations beside the tangency open a
    /// wedge in band while those further out read definitely
    /// transverse: the stations read corner, corner, in band, smooth,
    /// in band, corner, corner. Tier 3 classifies every station and
    /// refuses an in-band one `SliverDihedral` wherever it sits, so the
    /// seam is never stored as either description; a walk that answered
    /// from the first station would keep it conventional.
    #[test]
    fn an_in_band_wedge_behind_a_corner_refuses_as_the_seam_wedge() {
        let (torus, bitangent, arc) = villarceau();
        let extent = 1.0;
        // The wedge's margin, `sin θ` levered over the folded arm, at
        // the arc's `t`: linear in `t` beside the tangency.
        let wedge = |t: f64| {
            let p = arc.eval(t);
            let n = |s: &geom::Surface<f64>| geom_brep::implicit_gradient(s, p).normalize();
            n(&torus).cross(n(&bitangent)).norm()
                * geom_brep::folded_lever_arm(&torus, &bitangent, p, extent)
        };
        let slope = wedge(1e-4) / 1e-4;
        // The stations sit a quarter of the half-span apart: the inner
        // two read 0.7 of the band's escalation edge, the next 1.4.
        let half = 4.0 * 0.7 * band().escalate() / slope;
        let span = (-half, half);
        let classes: Vec<_> = (1..geom_brep::CERT_SAMPLES - 1)
            .map(|i| {
                let p = arc.eval(geom_brep::sample_param(span.0, span.1, i));
                match geom_brep::classify_dihedral(&torus, &bitangent, p, extent, band()) {
                    Ok(geom_brep::DihedralClass::Transverse) => 'T',
                    Ok(geom_brep::DihedralClass::Smooth) => 'S',
                    Err(_) => 'E',
                }
            })
            .collect();
        assert_eq!(
            classes.iter().collect::<String>(),
            "TTESETT",
            "the fixture's stations (half-span {half:e})"
        );
        for (a, b) in [(&torus, &bitangent), (&bitangent, &torus)] {
            let verdict =
                geom_brep::must_carry_over_edge(a, b, &arc, span.0, span.1, extent, band());
            assert!(
                matches!(
                    verdict,
                    geom_brep::MustCarryVerdict::InBand(
                        geom_brep::MustCarryEscalation::FirstOrder(escalation)
                    ) if escalation.rung() == geom_brep::LeverRung::Reading
                ),
                "an in-band wedge anywhere escalates, got {verdict:?}"
            );
        }
        for (order, got) in both_orders(&torus, &bitangent, &arc, span, extent)
            .into_iter()
            .enumerate()
        {
            assert!(
                matches!(
                    got,
                    Err(BooleanError::Escalated {
                        decision: BooleanDecision::SeamWedge,
                        ..
                    })
                ),
                "order {order}: the in-band wedge refuses as the seam's wedge, got {got:?}"
            );
        }
    }

    /// **The rebuild's smooth arm decides through [`must_carry_reading`]
    /// — the verdict [`seam_must_carry`] words — and spells no
    /// second-order reading of its own**, so the rows above, which call
    /// the helper, speak for the boolean's seams: a loop inlined back
    /// into `describe_edges` reds here.
    #[test]
    fn the_smooth_seam_arm_routes_through_the_must_carry_rule() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/boolean/ops.rs");
        let source = test_utils::source::code_only(&std::fs::read_to_string(path).unwrap());
        let start = source
            .find("fn describe_edges")
            .expect("the rebuild's description pass");
        let body = &source[start..start + source[start..].find("\n}\n").expect("its end")];
        assert_eq!(
            body.matches("must_carry_reading(").count(),
            1,
            "the smooth arm asks the rule once"
        );
        for spelling in [
            "tangent_jet",
            "tangent_second_order",
            "must_carry_over_edge",
            "Margin::sagitta",
            "lever_arm(",
            "decide(",
        ] {
            assert!(
                !body.contains(spelling),
                "describe_edges spells `{spelling}` beside the rule"
            );
        }
    }

    /// **The kept faces' boundaries are the merge door's, not this
    /// worklist's.** For a merge outcome that kept one face and skipped
    /// nothing, the boolean's description worklist over no seam edges
    /// is empty: the door re-describes a kept face's boundary before it
    /// returns, and the boolean does not describe it a second time.
    #[test]
    fn the_description_worklist_carries_no_kept_boundary() {
        let prism = crate::test_support_fixtures::prism_z::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            0.0,
            1.0,
            Tol::witness(),
        );
        let kept = prism.side_faces[0];
        let mut merged = crate::merge_faces::MergeCoplanarOutcome::default();
        merged.groups.push(crate::merge_faces::MergedGroup {
            kept,
            absorbed: Vec::new(),
            killed_edges: Vec::new(),
            rings_made: Vec::new(),
            killed_vertices: Vec::new(),
        });
        assert_eq!(
            super::boundary_edges(&prism.body, [kept]).map(|edges| edges.len()),
            Some(4),
            "the kept wall has four boundary edges for the worklist to leave out"
        );
        assert!(
            matches!(super::describe_worklist(&prism.body, &[], &merged), Ok(w) if w.is_empty()),
            "the boolean's worklist holds none of the kept face's boundary"
        );
    }

    /// **A boundary edge both of whose sides are listed is walked once**,
    /// so the describer is handed it once.
    #[test]
    fn a_boundary_edge_between_two_listed_faces_is_listed_once() {
        let prism = crate::test_support_fixtures::prism_z::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            0.0,
            1.0,
            Tol::witness(),
        );
        let faces = [prism.side_faces[0], prism.side_faces[1]];
        let edges = super::boundary_edges(&prism.body, faces).expect("the walls' loops walk");
        assert_eq!(
            edges.len(),
            7,
            "two quads sharing one edge: 4 + 4 - 1 = {edges:?}"
        );
        assert_eq!(
            edges.iter().filter(|&&(face, _)| face == faces[1]).count(),
            3,
            "the shared edge is listed with the first face that holds it"
        );
    }

    /// **The door's refusal comes back to the boolean as the boolean's
    /// own**: every refusal the describer can raise, worded by the merge
    /// door and handed through `of_merge`, is the very `BooleanError`
    /// the boolean's own description pass raises for it.
    #[test]
    fn a_kept_boundary_refusal_comes_back_as_the_booleans_own() {
        use crate::merge_faces::{DescribeRefusal, DihedralReading, EdgeDescribeFailure};
        let (kept, strut) = (
            crate::entity::FaceKey::default(),
            crate::entity::EdgeKey::default(),
        );
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let diag = geom_core::Indeterminate {
            margin: geom_core::MarginDiag::value((band.zero() * band.escalate()).sqrt()),
            band,
            predicate: Some("dihedral_arm"),
            terminal_sliver: false,
        };
        let text = super::of_merge(crate::merge_faces::MergeCoplanarError::of_kept_boundary(
            &[(kept, strut)],
            DescribeRefusal::undecided(strut, DihedralReading::Bend, diag),
        ))
        .to_string();
        assert!(
            text.contains("seam") && !text.contains("kept face"),
            "the boolean words it as its own seam's: {text}"
        );
        let mut every = vec![
            DescribeRefusal::undecided(
                strut,
                DihedralReading::Lever(geom_brep::LeverRung::Arm),
                diag,
            ),
            DescribeRefusal::undecided(
                strut,
                DihedralReading::Lever(geom_brep::LeverRung::Reading),
                diag,
            ),
            DescribeRefusal::undecided(strut, DihedralReading::Bend, diag),
        ];
        let residual = crate::euler::EulerOpError::Certification {
            error: geom_brep::CertifyError::ResidualExceeded {
                check: geom_brep::CertCheck::SurfaceResidual {
                    surface: crate::geometry::SurfaceKey::default(),
                },
                sample: 8,
                margin: geom_core::MarginDiag::value(3.0 * band.escalate()),
            },
        };
        for failure in [
            EdgeDescribeFailure::NotWalkable,
            EdgeDescribeFailure::Intersection,
            EdgeDescribeFailure::Tangency,
            EdgeDescribeFailure::NoConventionalLane,
            EdgeDescribeFailure::Arc,
            EdgeDescribeFailure::Line,
        ] {
            every.push(DescribeRefusal::failed(strut, failure));
            for fitted in [false, true] {
                every.push(DescribeRefusal::refused(
                    strut,
                    failure,
                    residual.clone(),
                    fitted,
                ));
            }
        }
        every.push(DescribeRefusal::off_face(strut, diag));
        for described in every {
            let worded = crate::merge_faces::MergeCoplanarError::of_kept_boundary(
                &[(kept, strut)],
                described.clone(),
            );
            assert_eq!(
                format!("{:?}", super::of_merge(worded)),
                format!("{:?}", super::of_describe(described.clone())),
                "{described:?} passes through as the boolean's own"
            );
        }
    }

    /// **An attachment refusal at the describe pass tells its own
    /// story, the same at both doors.** A line chord's residual against
    /// one of its faces read in band (the near-tangent census's 12
    /// witnesses, `sweep`'s `near_tangent_pierce_poses`) is the input's,
    /// ending in the lever. The witness's residual, a definite miss, a
    /// circle's residual in band and any other refusal are the kernel's,
    /// ending in the defect ending with the check, the sample and the miss
    /// and no arena key; a fitted carrier's ends as the certification's
    /// table gives it.
    #[test]
    fn a_describe_refusal_carries_its_certification() {
        use crate::euler::EulerOpError;
        use crate::merge_faces::{EdgeDescribeFailure, MergeCoplanarError};
        use geom_brep::{CertCheck, CertifyError};
        use geom_core::{KERNEL_DEFECT_ENDING, KERNEL_LIMIT_RECOURSE, MarginDiag};
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let (edge, face, surface) = (
            crate::entity::EdgeKey::default(),
            crate::entity::FaceKey::default(),
            crate::geometry::SurfaceKey::default(),
        );
        let line = geom::Curve3::Line {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 0.0, 0.0),
        };
        let circle = geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let fitted = geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
                vec![1.0, 1.0],
            )
            .unwrap(),
        ));
        let in_band = geom_core::Indeterminate {
            margin: MarginDiag::value(band.zero() * (1.0 + 5e-8)),
            band,
            predicate: Some("carrier_on_surface_2"),
            terminal_sliver: false,
        };
        let certification = |error| EulerOpError::Certification { error };
        let escalated = |check| {
            certification(CertifyError::Escalated {
                check,
                sample: 8,
                cause: in_band,
            })
        };
        let definite = certification(CertifyError::ResidualExceeded {
            check: CertCheck::SurfaceResidual { surface },
            sample: 8,
            margin: MarginDiag::value(3e-8),
        });
        // Each door's text, the merge door's first.
        let said = |carrier: &geom::Curve3<f64>, cause| {
            let refusal = super::of_attachment(
                edge,
                EdgeDescribeFailure::Intersection,
                carrier.kind(),
                cause,
            );
            let merged = MergeCoplanarError::of_kept_boundary(&[(face, edge)], refusal.clone());
            let own = super::of_describe(refusal);
            assert_eq!(
                format!("{:?}", super::of_merge(merged.clone())),
                format!("{own:?}"),
                "the merge door's refusal comes back as the boolean's own"
            );
            (merged.to_string(), own)
        };
        let lever = "Recourse: move the geometry so each edge and vertex that meets a face lies \
                     clearly on it or clearly off it, and an edge that ends on it leaves it at a \
                     clear angle";

        let (merged, err) = said(&line, escalated(CertCheck::SurfaceResidual { surface }));
        assert!(
            matches!(
                err,
                BooleanError::Escalated {
                    decision: BooleanDecision::EdgeOnFaces,
                    diag,
                } if diag == in_band
            ),
            "a line chord read in band of its face is the edge's reading: {err:?}"
        );
        assert_eq!(
            err.to_string(),
            format!(
                "whether an edge of the result lies on both faces it bounds is undecided: {}. \
                 {lever}",
                in_band.payload()
            ),
            "the Boolean: the decision's one sentence, ending in the lever alone"
        );
        assert_eq!(
            merged,
            format!(
                "merge_coplanar_faces: re-describing kept face {face:?}'s boundary edge \
                 {edge:?}, whether the edge lies on both faces it bounds is undecided: {}. \
                 {lever}",
                in_band.payload()
            ),
            "the merge door: the same reading and lever"
        );

        let undescribed = |payload: &str, ending: &str| {
            (
                format!(
                    "merge_coplanar_faces: kept face {face:?}'s boundary edge {edge:?} cannot \
                     be re-described against the two faces it now lies between: its two faces' \
                     intersection does not certify on its curve ({payload}). The body is \
                     untouched. {ending}"
                ),
                format!(
                    "an edge of the result cannot be described against the two faces it lies \
                     between: its two faces' intersection does not certify on its curve \
                     ({payload}). {ending}"
                ),
            )
        };
        let witness_payload = format!(
            "the witness point's residual against one of the edge's surfaces at sample 8 \
             escalated: {}",
            in_band.payload()
        );
        let miss_payload = "the residual against one of the edge's surfaces at sample 8 is \
                            3e-8, definitely past the tolerance";
        let surface_payload = format!(
            "the residual against one of the edge's surfaces at sample 8 escalated: {}",
            in_band.payload()
        );
        for (label, carrier, cause, payload, ending) in [
            (
                "the witness's residual, an exact construction",
                &line,
                escalated(CertCheck::WitnessSurfaceResidual { surface }),
                witness_payload.as_str(),
                KERNEL_DEFECT_ENDING,
            ),
            (
                "a definite miss",
                &line,
                definite,
                miss_payload,
                KERNEL_DEFECT_ENDING,
            ),
            (
                "a circle read in band, a construction whose own error the band may hold",
                &circle,
                escalated(CertCheck::SurfaceResidual { surface }),
                surface_payload.as_str(),
                KERNEL_DEFECT_ENDING,
            ),
            (
                "a fitted carrier read in band, the fit's limit",
                &fitted,
                escalated(CertCheck::SurfaceResidual { surface }),
                surface_payload.as_str(),
                KERNEL_LIMIT_RECOURSE,
            ),
        ] {
            let (merged, err) = said(carrier, cause);
            let text = err.to_string();
            assert!(
                matches!(err, BooleanError::EdgeUndescribed { .. }),
                "{label}: {err:?}"
            );
            assert_eq!(
                (merged, text.clone()),
                undescribed(payload, ending),
                "{label}"
            );
            assert!(!text.contains("Key("), "{label}: no arena key: {text}");
        }
    }

    /// **The `Approx` arm asks after the BALL, not the sphere face**
    /// (`ball_may_reach`). The face here keeps a small cap of its
    /// sphere near the north pole; the brick inside the ball near the
    /// south pole is clear of any honest reach of that cap and still
    /// in the ball, which is where the escape it guards runs. So it is
    /// reached, face by face, whatever the cap's own reach. A brick past
    /// the ball on the diagonal, inside the ball's world box, is parted
    /// by the narrow phase. Asking after the face instead, the inside
    /// brick reads apart the day a sphere face's reach is tighter than
    /// its ball.
    #[test]
    fn the_approx_arm_asks_whether_the_ball_reaches_the_face() {
        use super::ball_may_reach;
        use crate::boolean::boxes;
        use crate::test_support::brick;
        let tol = geom_core::Tol::witness();
        let band = Band::linear(tol).unwrap();
        let pad = boxes::sweep_pad(band);
        // The operators' chords, not described as intersections, so the
        // cap's carrier can be swapped for the sphere's.
        let mut capped = crate::Body::<f64>::new();
        crate::test_support::prism_ops(
            &mut capped,
            &[(-0.2, -0.2), (0.2, -0.2), (0.2, 0.2), (-0.2, 0.2)],
            (0.9, 0.98),
            Point3::new,
            crate::test_support::FaceGeometry::Certified,
            tol,
        );
        let top = capped
            .faces()
            .map(|(f, _)| f)
            .find(|&f| {
                crate::face_normal::face_outward_normal(&capped, f).is_some_and(|n| n.vec().z > 0.5)
            })
            .unwrap();
        capped
            .set_face_surface(
                top,
                crate::FaceSurface::New {
                    surface: geom::Surface::Sphere {
                        center: Point3::new(0.0, 0.0, 0.0),
                        radius: 1.0,
                        axis: geom_core::Vec3::new(0.0, 0.0, 1.0),
                        u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
                    },
                    sense: true,
                },
            )
            .unwrap();
        let &geom::Surface::Sphere { center, radius, .. } =
            capped.face_surface_linked(top, capped.get_face(top).unwrap())
        else {
            panic!("the cap's carrier is the sphere");
        };
        let ball_box = boxes::centred_box(center, radius, pad);
        for (what, (x, y, z), reached) in [
            (
                "inside the ball, off the cap",
                ((-0.1, 0.1), (-0.1, 0.1), (-0.9, -0.7)),
                true,
            ),
            (
                "past the ball on the diagonal",
                ((0.8, 0.9), (0.8, 0.9), (0.8, 0.9)),
                false,
            ),
        ] {
            let other = brick::<f64>(x, y, z, tol);
            // Each brick its own cell: the axes are read off this pair.
            let axes = crate::boolean::separating::OperandAxes::new(boxes::axis_key);
            for (yf, _) in other.faces() {
                let face_box = boxes::face_box(&other, yf, pad, band).unwrap();
                assert!(face_box.overlaps(&ball_box), "{what}: the boxes overlap");
                assert_eq!(
                    ball_may_reach(
                        (&other, yf, &face_box),
                        (center, radius, &ball_box),
                        axes.of(&capped, &other, band),
                        pad,
                        band,
                    ),
                    reached,
                    "{what}: face {yf:?}"
                );
            }
        }
    }

    /// **The sphere extent scan answers `ContainError`'s reachable
    /// refusals typed, on the face's own operand.** It runs on operands
    /// AT REST: a refusal the face door owns is carried whole with the
    /// face and the operand facing the sphere's — decided, not
    /// forwarded — and only a lone-vertex loop, which the gate's tier 2
    /// rules out, stays a kernel-bug claim.
    #[test]
    fn extent_scan_refusals_name_the_faces_operand() {
        use crate::boolean::Operand;
        use crate::boolean::contain::ContainError;
        use crate::boolean::solid_contain::PointInSolidError;
        use crate::entity::{FaceKey, LoopKey};
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let (face, lk) = (FaceKey::from(key(7)), LoopKey::from(key(9)));
        for (sphere_is, face_is) in [(Operand::A, Operand::B), (Operand::B, Operand::A)] {
            for e in [
                ContainError::LoopUnreadable(lk),
                ContainError::RayExhausted,
                ContainError::Curved(PointInSolidError::CorruptFace { face }),
            ] {
                let got = super::extent_scan_refusal(e.clone(), sphere_is, face);
                assert!(
                    matches!(
                        &got,
                        BooleanError::PointInFaceRefused { operand, face: f, refusal }
                            if *operand == face_is && *f == face && *refusal == e
                    ),
                    "sphere on {sphere_is:?}: {e:?} answered {got:?}"
                );
            }
        }
        assert!(
            matches!(
                super::extent_scan_refusal(ContainError::EmptyLoop(lk), Operand::A, face),
                BooleanError::ClassificationInvariant { .. }
            ),
            "a lone-vertex loop past the gate is a kernel bug"
        );
    }
}

/// **`sphere_extent_scan`: a torn surface panics**, where it read as a
/// face that is neither NURBS nor a sphere; **the face boundary walks
/// panic on a ring link that does not resolve**, where they stepped
/// over it.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_hop_rows {
    use super::*;
    use crate::entity::GeomRef;
    use crate::live::OPERATORS_KEEP_LINKS;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};

    #[test]
    fn the_extent_scan_panics_on_a_torn_surface() {
        let band = Band::linear(Tol::witness()).unwrap();
        let cube = || crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let (mut a, b) = (cube(), cube());
        assert!(
            {
                let r = sphere_extent_scan(&a, &b, &[], band).unwrap();
                r.rechart.is_empty() && r.cut_in.is_empty()
            },
            "two cubes carry no sphere"
        );
        let (face, fd) = a.faces().next().map(|(k, d)| (k, d.clone())).unwrap();
        a.surfaces.remove(fd.surface);
        let named = format!(
            "{}'s surface names {}",
            EntityId::Face(face),
            GeomRef::Surface(fd.surface)
        );
        assert_torn_op_panics(
            "sphere_extent_scan",
            &mut a,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |a| sphere_extent_scan(a, &b, &[], band).map(|r| r.rechart.len() + r.cut_in.len()),
        );
    }

    #[test]
    fn the_face_boundary_walks_panic_on_a_torn_ring_link() {
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let face = body.faces().next().map(|(k, _)| k).unwrap();
        let far = bvh::Aabb {
            min_x: 10.0,
            min_y: 10.0,
            min_z: 10.0,
            max_x: 11.0,
            max_y: 11.0,
            max_z: 11.0,
        };
        let circle = super::super::separating::Item::Circle(super::super::separating::Circle {
            center: Point3::new(10.5, 10.5, 10.5),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            v_ref: Vec3::new(0.0, 1.0, 0.0),
            radius: 0.5,
        });
        let band = Band::linear(Tol::witness()).unwrap();
        assert!(
            !face_boundary_meets(&body, face, (circle, &far), &[], 0.0, band),
            "a far box"
        );
        assert!(!has_lone_vertex(&body, face), "a cube face has none");
        assert_eq!(faces_by_vertex(&body).len(), 8, "eight corners");
        let named = crate::review_d18::tear_ring(&mut body, face);
        let premise = [named.as_str(), ROW_FOUR, OPERATORS_KEEP_LINKS];
        assert_torn_op_panics("face_boundary_meets", &mut body, &premise, |b| {
            face_boundary_meets(b, face, (circle, &far), &[], 0.0, band)
        });
        assert_torn_op_panics("has_lone_vertex", &mut body, &premise, |b| {
            has_lone_vertex(b, face)
        });
        assert_torn_op_panics("faces_by_vertex", &mut body, &premise, |b| {
            faces_by_vertex(b).len()
        });
    }
}

/// **Only an R-loop verdict cuts a trimmed sphere face** ([`cut_holder`]):
/// a closed group re-charts, the face the certificate placed the circle
/// inside takes the cut, and every other verdict refuses with its own
/// reason rather than cutting the face it names.
#[cfg(test)]
mod cut_holder_rows {
    use super::*;

    #[test]
    fn only_the_loop_verdict_cuts_its_face() {
        let face = FaceKey::default();
        assert_eq!(cut_holder(None), Ok(None), "a closed group re-charts");
        assert_eq!(
            cut_holder(Some((face, SectionRefusal::Loop))),
            Ok(Some(face)),
            "R-loop cuts the face it names"
        );
        for refusal in [
            SectionRefusal::Reach,
            SectionRefusal::Tangent("section_sphere_plane_reach"),
            SectionRefusal::Unordered("section_torus_square_wall_fold_half"),
            SectionRefusal::Undecided,
            SectionRefusal::LoneVertex,
        ] {
            assert_eq!(
                cut_holder(Some((face, refusal))),
                Err(refusal.what()),
                "{refusal:?} refuses with its own reason"
            );
        }
    }
}

#[cfg(test)]
mod from_computed_rows {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use geom_core::{Band, Point3, Real, SymBudget, Tol};

    use super::{overlap_reach, volume_backstop};
    use crate::Body;
    use crate::boolean::BooleanOp;
    use crate::props::QuadLane;
    use crate::revalue_pin::{Revalued, S, revalued_at};
    use crate::test_support_fixtures::{FaceGeometry, prism_ops};

    /// The backstop's lower end takes the interval re-derivation as a
    /// computed value: `volume_backstop_passes_a_closed_form_rounding_tie`
    /// at `Sym<f64>`, the tier off so the walk's rounded sums read the
    /// tie as violated and the interval margin is what leaves it open,
    /// with every coordinate scaled by the parameter. The scale is a
    /// power of two, so both builds round alike and take the same
    /// decisions, and none re-values to the margin `p0` re-derived.
    #[test]
    fn the_backstop_takes_its_rederived_margin_as_computed() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let profile = [(0.1, 0.2), (1.3, 0.25), (1.1, 1.7), (0.05, 0.9)];
        let mut turned = profile;
        turned.rotate_left(3);
        let off = SymBudget {
            max_terms: 0,
            max_degree: 0,
        };
        let found = revalued_at(off, [1.0, 2.0], |_, s| {
            let prism = |corners: &[(f64, f64)]| {
                let mut body = Body::<S>::new();
                let at = |x: f64, y: f64, z: f64| Point3::new(x, y, z).map(|c| S::from_f64(c) * s);
                prism_ops(
                    &mut body,
                    corners,
                    (0.0, 0.7),
                    at,
                    FaceGeometry::Certified,
                    tol,
                );
                body
            };
            let (low, high) = (prism(&profile), prism(&turned));
            let got = volume_backstop(
                BooleanOp::Intersect,
                &low,
                &low,
                &high,
                band,
                tol,
                QuadLane::certified(),
            );
            assert!(got.is_ok(), "the tie passes: {got:?}");
            Vec::new()
        });
        assert_eq!(
            found,
            Revalued {
                kept: 343,
                opaque: 2,
                laundered: 0,
            },
            "the two decisions the re-derived lower end feeds, and nothing else, are opaque"
        );
    }

    /// The reach enters as computed values: built at `Sym<f64>` from two
    /// boxes the face walk would measure from geometry the parameter
    /// moves, neither its centre nor its radius re-values to the ball
    /// `p0` measured.
    #[test]
    fn the_reach_takes_its_ball_as_computed() {
        let found = crate::revalue_pin::revalued([1.0, 1.0 + 1.0e-3], |p, _| {
            let cube = |lo: f64, hi: f64| bvh::Aabb {
                min_x: lo,
                min_y: lo,
                min_z: lo,
                max_x: hi,
                max_y: hi,
                max_z: hi,
            };
            let reach = overlap_reach::<S>(&cube(0.0, p), &cube(0.5 * p, 2.0 * p));
            vec![reach.centre.x, reach.radius]
        });
        assert_eq!(found.laundered, 0, "{found:?}");
        assert_eq!(found.opaque, 2, "{found:?}");
    }
}

/// **Rows for [`Descendants::holder`]'s arms**, each through
/// [`carry_in_place`] on a 2×2×1 prism whose top face a diagonal chord
/// divides, with the division logged as a fragment row. The record's
/// vertex is a lone vertex ([`Body::mvfs`]) set where the row needs it,
/// so it bounds no piece and only the placement decides.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod holder_rows {
    use super::{Cell, Descendants, carry_in_place};
    use crate::boolean::{
        BooleanDecision, BooleanError, Contact, ContactKind, ContactRecords, SelfCheck,
    };
    use crate::entity::{FaceKey, VertexKey};
    use crate::{Body, MefSite, MevSite};
    use geom_core::{Band, Point3, Tol};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The prism, its top face (the key piece, holding the corner
    /// `(0, 2)`) and the piece the chord walls off (holding `(2, 0)`).
    fn divided() -> (Body<f64>, FaceKey, FaceKey) {
        let tol = Tol::witness();
        let p = crate::test_support_fixtures::prism_z::<f64>(
            &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
            0.0,
            1.0,
            tol,
        );
        let mut body = p.body;
        let corner = |body: &Body<f64>, x: f64, y: f64| {
            let outer = body.get_face(p.top_face).unwrap().outer;
            let crate::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary
            else {
                panic!("the top is a cycle")
            };
            body.loop_cycle(first)
                .unwrap()
                .into_iter()
                .find(|&he| {
                    let v = body.get_half_edge(he).unwrap().start;
                    let q = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                    (q.x, q.y) == (x, y)
                })
                .unwrap()
        };
        let (he1, he2) = (corner(&body, 0.0, 0.0), corner(&body, 2.0, 2.0));
        let made = body.mef_chord(MefSite::Chords { he1, he2 }, tol).unwrap();
        (body, p.top_face, made.face)
    }

    /// Whether `face` holds the top's point `(x, y)`.
    fn holds(body: &Body<f64>, face: FaceKey, x: f64, y: f64) -> bool {
        super::piece_holds(body, face, Point3::new(x, y, 1.0), band())
            .is_ok_and(|r| matches!(r, super::PieceRead::In))
    }

    /// A lone vertex at `(x, y, z)`.
    fn lone(body: &mut Body<f64>, x: f64, y: f64, z: f64) -> VertexKey {
        body.mvfs(Point3::new(x, y, z), true).unwrap().vertex
    }

    /// `vertex` recorded resting on `top`, carried with `top`'s division
    /// logged as `rows`.
    fn carried(
        body: &Body<f64>,
        vertex: VertexKey,
        top: FaceKey,
        rows: Vec<(FaceKey, FaceKey)>,
    ) -> Result<Vec<ContactKind>, BooleanError> {
        let records: ContactRecords = [crate::Cited::new(
            Contact::of(Cell::Vertex(vertex), Cell::Face(top)).unwrap(),
            crate::Cites::decided(0),
        )]
        .into_iter()
        .collect();
        let desc = Descendants::default().with_fragments([rows.clone(), rows]);
        carry_in_place(body, &records, &desc, band()).map(|r| r.kinds().collect())
    }

    /// A vertex inside the walled-off piece lands on it, not on the
    /// key piece. Red when the record follows the key.
    #[test]
    fn an_interior_rest_lands_on_the_piece_that_holds_it() {
        let (mut body, top, other) = divided();
        let (x, y) = if holds(&body, other, 1.5, 0.5) {
            (1.5, 0.5)
        } else {
            (0.5, 1.5)
        };
        assert!(
            holds(&body, other, x, y),
            "the fixture's walled-off piece holds ({x}, {y})"
        );
        let v = lone(&mut body, x, y, 1.0);
        assert_eq!(
            carried(&body, v, top, vec![(other, top)]).unwrap(),
            [ContactKind::VertexFace(v, other)]
        );
    }

    /// A vertex on the chord meets the edge both pieces share, and no
    /// piece's interior: the record drops. Red when a piece's boundary
    /// counts as holding (two pieces would hold it).
    #[test]
    fn a_rest_on_the_dividing_chord_drops() {
        let (mut body, top, other) = divided();
        let v = lone(&mut body, 1.0, 1.0, 1.0);
        assert_eq!(carried(&body, v, top, vec![(other, top)]).unwrap(), []);
    }

    /// A vertex whose margin from the top's plane is in band escalates
    /// as the carry's lineage check. Red when an in-band margin is read
    /// as off the plane, or as on it.
    #[test]
    fn a_rest_in_band_of_the_plane_escalates() {
        let (mut body, top, other) = divided();
        let v = lone(&mut body, 1.5, 0.5, 1.0 + 3.0 * band().zero());
        assert!(
            matches!(
                carried(&body, v, top, vec![(other, top)]),
                Err(BooleanError::Escalated {
                    decision: BooleanDecision::SelfCheck(SelfCheck::CarriedLineage),
                    ..
                })
            ),
            "{:?}",
            carried(&body, v, top, vec![(other, top)])
        );
    }

    /// Two lineage pieces that both hold the vertex are a corrupt
    /// division: the top's own copy, grafted in as a second shell, is
    /// logged as divided from it. Red when the second holder replaces
    /// the first.
    #[test]
    fn two_pieces_holding_one_rest_refuse() {
        let (mut body, top, _) = divided();
        let copy = crate::test_support_fixtures::prism_z::<f64>(
            &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
            0.0,
            1.0,
            Tol::witness(),
        );
        let solid = body.solids().next().unwrap().0;
        let graft = crate::boolean::combine::graft_solids_disjoint(&mut body, &[solid], &copy.body)
            .unwrap();
        let twin = *graft.faces.get(copy.top_face).unwrap();
        let (x, y) = if holds(&body, top, 0.5, 1.5) {
            (0.5, 1.5)
        } else {
            (1.5, 0.5)
        };
        assert!(holds(&body, twin, x, y), "the twin holds ({x}, {y})");
        let v = lone(&mut body, x, y, 1.0);
        assert!(
            matches!(
                carried(&body, v, top, vec![(twin, top)]),
                Err(BooleanError::JoinDesync { .. })
            ),
            "{:?}",
            carried(&body, v, top, vec![(twin, top)])
        );
    }

    /// A piece the containment door cannot read (a lone-vertex ring,
    /// which bounds no region) may hold the rest, and none other does:
    /// the op refuses, naming the piece. Red when an unread piece is
    /// read as not holding (the rest drops) or falls back to the key.
    #[test]
    fn an_unreadable_piece_refuses() {
        let tol = Tol::witness();
        let (mut body, top, other) = divided();
        let (x, y) = if holds(&body, other, 1.5, 0.5) {
            (1.5, 0.5)
        } else {
            (0.5, 1.5)
        };
        // A strut from a corner of `other` into it, killed: its far end
        // is stranded as a lone-vertex ring of `other`.
        let outer = body.get_face(other).unwrap().outer;
        let crate::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("the piece is a cycle")
        };
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: first,
                    he2: first,
                },
                Point3::new((x + 1.0) / 2.0, (y + 1.0) / 2.0, 1.0),
                tol,
            )
            .unwrap();
        body.kemr(strut.he_plus, strut.he_minus).unwrap();
        let v = lone(&mut body, x, y, 1.0);
        match carried(&body, v, top, vec![(other, top)]) {
            Err(BooleanError::RestPieceUnread { face, refusal }) => {
                assert_eq!(face, other, "the refusal names the unread piece");
                assert!(refusal.is_some(), "the planar door's refusal is carried");
            }
            got => panic!("{got:?}"),
        }
    }
}
