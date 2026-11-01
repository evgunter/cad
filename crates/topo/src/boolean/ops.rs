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
//!   (∪ of separated bodies), including assemblies touching at vertex,
//!   edge and face contacts (the carried
//!   [`ContactRecords`] say where; genuinely 3′, certified by PR 6's
//!   validator), a tangent line through two faces' interiors among them.
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
//!   ([`Exempt::Rest`]), and a settled one-carrier pair between two
//!   shells that lie on each other by their `On` verdict, or that read
//!   sides where the op keeps one operand at most ([`OnDeferred`]).

use geom_core::{Band, Bounds, Decide, Indeterminate, Margin, Point3, Real, Sign, Tol, Vec3};

use std::collections::{BTreeMap, BTreeSet};

use super::SphereQuestion;
use super::boxes;
use super::combine::{GraftMap, graft_solids_disjoint};
use super::contain::{ContainError, FaceContainment, contfp};
use super::edge_join::join_stage;
use super::finish::setopfinish;
use super::join::bool_connect;
use super::reduce::PendingRow;
use super::section_cert::Refusal as SectionRefusal;
use super::section_cert::Sheet;
use super::shell_witness::{
    ShellVerdict, check_mutual, debug_assert_contacts_undecisive, kept_shells, shell_verdict,
};
use super::solid_contain::{SolidContainment, closed_sphere_group};
use super::voids;
use super::zip::{Fusions, Seam, SeamCorrespondence, zip_seam, zip_split};
use super::{BooleanDecision, LineWedgeFault};
use super::{
    BooleanError, BooleanInputs, BooleanOp, BooleanReduction, Cell, Contact, ContactRecords,
    EeContact, EfContact, FfContact, Glue, GlueVerdict, Operand, OperandRecords, ReductionContacts,
    SelfCheck, SweepStrategy, VeContact, VfContact, VvContact,
};
use crate::body::Body;
use crate::coincidence::{Backing, Cited, Cites};
use crate::entity::{EdgeKey, EntityId, FaceKey, LoopBoundary, ShellKey, VertexKey};
use crate::geometry::SurfaceKey;
use crate::live::{BoundaryMember, linked, proven};
use crate::merge_faces::{DescribeRefusal, DihedralReading, EdgeDescribeFailure};
use crate::props::AtRestPolicy;
use crate::splitting::finish::{carve, single_solid};
use crate::validate::{
    AtRestBody, OrientationUnread, ValidationError, decide, scaffolds_at_rest, validate_closed,
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
    /// without a seam (disjoint, or touching, each touch a contact
    /// record).
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
    /// The face pairs those coincidences glue, in their order
    /// ([`GlueVerdict`]).
    pub verdicts: Vec<GlueVerdict<T>>,
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
#[derive(Debug)]
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
    /// Seam edges surviving the zips, each once, in the zip/cycle order
    /// that first reports it, result keys.
    pub seam_edges: Vec<EdgeKey>,
    /// Vertex fusions `(dead, kept)` in mint order, result keys: the
    /// zips', then the partings' joined ends
    /// (`boolean::slit`, an end joined to the vertex of the face's loop
    /// on its point).
    pub vertex_merges: Fusions,
    /// Face absorption groups `(kept, absorbed…)`, result keys:
    /// `merge_coplanar_faces`'.
    pub merge_groups: Vec<(FaceKey, Vec<FaceKey>)>,
    /// The output stage's substitutions, `(dead, by)` in stage order,
    /// result keys: first the merge's lineage, each cell a group killed
    /// (an absorbed face, a killed shared edge, a pruned vertex) with the
    /// kept face holding its interior
    /// ([`MergedGroup::substitutions`](crate::merge_faces::MergedGroup::substitutions));
    /// then the join's, each joined-away vertex and edge with the kept
    /// edge holding its interior ([`Self::edge_joins`]). Checked where
    /// each row is added, so a chase through it ends
    /// ([`Self::settler`]).
    pub substitutions: crate::lineage::Substitutions,
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
    pub face_fragments_a: crate::lineage::FaceDivisions,
    /// B-side chord-mef fragment rows, in B-CLONE keys (translate the
    /// new-face column through `graft_faces` for result keys). On an
    /// `Absent` side they divide only the dropped coincident faces the
    /// `substituted` and `discards` rows name.
    pub face_fragments_b: crate::lineage::FaceDivisions,
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
    /// renumbers onto its rows ([`carry`]), so its citations index no
    /// list a reader holds.
    pub reduction_contacts: ReductionContacts,
    /// Every face an operand lost to the result, with the kept faces it
    /// bordered (`boolean::discard`, which says how each path records
    /// its rows), except the dropped coincident faces in [`Self::uncut`];
    /// and every face the material divided along an edge lying in it,
    /// which lost the line between its pieces (`boolean::slit`).
    pub discards: Vec<super::DiscardRow>,
    /// Every fragment of a coincident face the result holds through the
    /// other operand's copy ([`super::SubstitutedFace`]), sorted and
    /// deduplicated. Each is a fragment as cut where the held region
    /// ends, on the section path and in the containment fallback alike.
    ///
    /// The rows are in clone keys, which
    /// [`substitutions`](Self::substitutions) cannot hold: those are result keys,
    /// and the dropped fragment is never in the result. In a
    /// containment fallback its whole operand is absent
    /// ([`OperandKeys::Absent`]), so no key of the result names it. A
    /// path that never classifies (disjoint boxes) has none.
    pub substituted: Vec<super::SubstitutedFace>,
    /// Every dropped coincident face a containment fallback could not
    /// divide where the region its kept face holds ends, one row per face
    /// and kept face, with the refusal of that face's own cut
    /// ([`super::UncutFace`]); sorted by operand, face and kept face. Such
    /// a face has neither a [`Self::substituted`] nor a [`Self::discards`]
    /// row. The section path records none: there the same cut refuses
    /// the boolean, so the other operand order of a pose recorded here
    /// can refuse where this order builds.
    pub uncut: Vec<super::UncutFace>,
    /// Every operand edge piece the classification read beside a vertex,
    /// with its side of the other operand
    /// ([`EdgePieceClass`](super::EdgePieceClass)), sorted: the vertex in
    /// clone keys, as `reduction_contacts`. Read off the classification,
    /// so a path that never classifies has none.
    pub edge_classes: Vec<super::EdgePieceClass>,
    /// Every vertex in one pair whose partner the classification read
    /// nothing of, so it holds no [`Self::edge_classes`] row, with why
    /// ([`UnreadPartner`](super::UnreadPartner)), sorted: the vertex in
    /// clone keys as `edge_classes`. A reader that needs a class there
    /// refuses naming the partner, not a missing row.
    pub unread_partners: Vec<super::UnreadPartner>,
    /// The meridian edges a sphere cut-in minted in each operand before
    /// the op re-entered on the cut operands, A's then B's, in clone keys
    /// as `edge_classes`. Each divides one operand face into two of its
    /// pieces (`face_fragments_a`/`face_fragments_b`): a cut of the
    /// face's chart, not an edge of the operand.
    pub chart_cuts: [Vec<EdgeKey>; 2],
    /// The cells a sphere re-chart turned in each operand before the op
    /// re-entered on the re-cut operands, A's then B's, in clone keys as
    /// `chart_cuts`, each with the faces of the input shell it was turned
    /// over ([`Recharts`]). Such a cell's key, an input key or one the
    /// re-chart's graft minted, names no operand cell at its place.
    pub recharted: Box<[Recharts; 2]>,
    /// The two vertices each of the classification's null edges joins,
    /// `(operand, below end, above end)` in clone keys as
    /// `edge_classes`: one point by construction, so a vertex and its
    /// copies are one place on the operand, and an operand edge ending at
    /// one ends at all of them. A path that never classifies has none.
    pub null_copies: Vec<(super::Operand, VertexKey, VertexKey)>,
    /// The zip's splits of a pinch per cone (`zip::split_cones`),
    /// `(new, split)` in the order made, in result keys before the zip's
    /// fusions: `new` is a copy of `split` on its point, holding one cone
    /// of the result's boundary there, so it is the operand vertex
    /// `split` is ([`Settler::settle_cones`]). Each copy was checked new
    /// to the list as the split was made, so the copies of a vertex form
    /// a tree. A path that never zips has none.
    pub cone_splits: crate::lineage::ConeCopies,
    /// The output stage's joins in the order made, result keys: each
    /// row's vertex and `gone` edge are dead, and its `kept` edge holds
    /// their interiors (maximal edges, `docs/DESIGN.md`'s merge stage).
    pub edge_joins: Vec<super::EdgeJoin>,
    /// Killed edge → the edge it was split from, for each edge the
    /// output stage killed that held a `SplitEdge` record: an edge the
    /// merge of coplanar faces killed, and each `gone` edge of
    /// [`Self::edge_joins`]. A kill takes its edge's birth record with
    /// it, so an edge this op split and then merged or joined away leads
    /// to its parent only through this copy, taken before the kills. The
    /// Split, which runs no merge, keeps the same copy of its join kills
    /// alone (`SplitNaming::joined_lineage`).
    pub killed_lineage: BTreeMap<EdgeKey, EdgeKey>,
    /// Killed edge → the edge it was split from, for each edge an
    /// operand's carve of its discarded shells killed that held a
    /// `SplitEdge` record, A's then B's, in each operand's clone keys as
    /// [`Self::discards`] (`boolean::discard`'s `carved_lineage`). Only
    /// the section path carves.
    pub carved_lineage: [BTreeMap<EdgeKey, EdgeKey>; 2],
}

impl Default for BooleanNaming {
    // Each lineage list names itself where it is made (`LineageRows`).
    fn default() -> Self {
        Self {
            a_keys: Default::default(),
            b_keys: Default::default(),
            graft_vertices: Default::default(),
            graft_edges: Default::default(),
            graft_dead_edges: Default::default(),
            graft_faces: Default::default(),
            seam_edges: Default::default(),
            vertex_merges: Fusions::new(crate::lineage::LineageRows::Fusions),
            merge_groups: Default::default(),
            substitutions: crate::lineage::Substitutions::new(
                crate::lineage::LineageRows::Substitutions,
            ),
            merge_skipped: Default::default(),
            face_fragments_a: crate::lineage::FaceDivisions::new(
                crate::lineage::LineageRows::FaceDivisions,
            ),
            face_fragments_b: crate::lineage::FaceDivisions::new(
                crate::lineage::LineageRows::FaceDivisions,
            ),
            reduction_contacts: Default::default(),
            discards: Default::default(),
            substituted: Default::default(),
            uncut: Default::default(),
            edge_classes: Default::default(),
            unread_partners: Default::default(),
            chart_cuts: Default::default(),
            recharted: Default::default(),
            null_copies: Default::default(),
            cone_splits: crate::lineage::ConeCopies::new(crate::lineage::LineageRows::ConeCopies),
            edge_joins: Default::default(),
            killed_lineage: Default::default(),
            carved_lineage: Default::default(),
        }
    }
}

impl BooleanNaming {
    /// **The one read of a result key back to its operand**: which
    /// operand the result key `k` belongs to, its key in that operand's
    /// space, and whether it was read back through the graft rows (`inv`,
    /// the graft's destination → source keys of `k`'s kind). A result key
    /// is an A key only where A's clone is the arena, and a B key either
    /// through the graft rows or because B's clone is the arena itself.
    /// Reading the other way, an operand's key into the result, matches
    /// on [`Self::a_keys`] and [`Self::b_keys`] where it is read.
    ///
    /// # Panics
    ///
    /// On a layout the boolean never records: it builds its result as
    /// A's clone with B grafted in, or as one operand's clone with the
    /// other absent ([`Self::a_keys`], [`Self::b_keys`]).
    #[track_caller]
    pub fn operand_of<K: Copy + Ord>(&self, inv: &BTreeMap<K, K>, k: K) -> (Operand, K, bool) {
        match (self.a_keys, self.b_keys) {
            (OperandKeys::Direct, OperandKeys::Grafted) => match inv.get(&k) {
                Some(&kb) => (Operand::B, kb, true),
                None => (Operand::A, k, false),
            },
            (OperandKeys::Direct, OperandKeys::Absent) => (Operand::A, k, false),
            (OperandKeys::Absent, OperandKeys::Direct) => (Operand::B, k, false),
            (a_keys, b_keys) => unreachable!(
                "a boolean's naming records operand keys {a_keys:?} and {b_keys:?}: the boolean \
                 builds its result as A's clone with B grafted in, or as one operand's clone with \
                 the other absent"
            ),
        }
    }

    /// Each result vertex a zip or a parting fused away → the live cell
    /// of `body` its point lies in ([`Self::settler`]).
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
            splits: &self.cone_splits,
            rows: &self.substitutions,
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
    /// The zip's splits per cone ([`BooleanNaming::cone_splits`]).
    splits: &'a crate::lineage::ConeCopies,
    /// The output stage's substitutions: the merge's, then the join's.
    rows: &'a crate::lineage::Substitutions,
}

impl<T: Real> Settler<'_, T> {
    /// The live cell `vertex`'s point lies in: the vertex it finally
    /// fused into, following the zips' and the partings' fusions
    /// ([`BooleanNaming::vertex_merges`]) through every hop, while
    /// that lives. A later stage can delete it without moving its point,
    /// and both such stages record the kill: the merge's pruning leaves
    /// the point inside its group's kept face ([`Cell::Face`]), and the
    /// output stage's join inside the joined edge ([`Cell::Edge`]), each
    /// a row of [`BooleanNaming::substitutions`]. The rows were checked
    /// where they were added, so the chase ends.
    ///
    /// # Errors
    ///
    /// [`SettleError::Dead`] where the chase ends on a cell `body` does
    /// not hold: a stage deleted it without its row.
    pub fn settle(&self, vertex: VertexKey) -> Result<Cell, SettleError> {
        crate::lineage::settle_through(
            self.fusions,
            Cell::Vertex(vertex),
            |c| self.rows.get(c),
            |c| holds(self.body, c),
        )
        .map_err(|reached| SettleError::Dead { vertex, reached })
    }

    /// The live cells `vertex`'s point lies in, one per cone of the
    /// result's boundary there: `vertex` and each copy the zip's splits
    /// per cone made of it, through splits of those copies (its
    /// [`crate::lineage::Descent`] over the splits, finite since the
    /// copies form a tree), each settled ([`Self::settle`]), in key
    /// order. A vertex no split
    /// touched settles on one cell. An edge that ended at `vertex`
    /// before the split ends at one of these after it.
    ///
    /// # Errors
    ///
    /// [`SettleError`], as [`Self::settle`].
    pub fn settle_cones(&self, vertex: VertexKey) -> Result<Vec<Cell>, SettleError> {
        crate::lineage::Descent::of(vertex, self.splits.rows())
            .cells()
            .map(|v| self.settle(v))
            .collect()
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
}

impl core::fmt::Display for SettleError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let what = match self {
            Self::Dead { .. } => "settles on a cell the result does not hold",
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
/// # Panics
///
/// On a kernel bug: an edge the body stores, re-minted or restated on its
/// own carrier and interval, whose span or winding reads certainly
/// refused at the run's band.
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
/// # Panics
///
/// On a kernel bug: an edge the body stores, re-minted or restated on its
/// own carrier and interval, whose span or winding reads certainly
/// refused at the run's band.
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
/// # Panics
///
/// On a kernel bug: an edge the body stores, re-minted or restated on its
/// own carrier and interval, whose span or winding reads certainly
/// refused at the run's band.
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
/// # Panics
///
/// On a kernel bug: an edge the body stores, re-minted or restated on its
/// own carrier and interval, whose span or winding reads certainly
/// refused at the run's band.
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
    boolean_op_given(op, a, b, inputs, None, strategy, tol)
}

/// [`boolean_op_with`], its coincidences `given` where they are a fold
/// step's of the n-ary door ([`super::nary`]): `Some` makes those pairs
/// the operation's only coincidences, and it decides none of its own.
pub(crate) fn boolean_op_given<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
    inputs: &BooleanInputs,
    given: Option<&[super::GluedPair<T>]>,
    strategy: SweepStrategy,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    #[cfg(feature = "door-tier3-meter")]
    let door_from = std::time::Instant::now();
    let result = boolean_door(op, a, b, &Glue::of(inputs, given), strategy, tol);
    #[cfg(feature = "door-tier3-meter")]
    super::door_meter::record(op, &result, door_from.elapsed(), [a.outcome(), b.outcome()]);
    if let Ok(BooleanResult::Body(out)) = &result {
        rows_name_input_cells(&out.coincidences, [a, b]);
    }
    result
}

/// **Every input cell a row names is live in its input**, a meeting's
/// further cells included: a row is keyed in the op's inputs
/// (`crate::coincidence`), never in the clones the op minted cells in.
/// It checks liveness only: a key the input holds but that the op read
/// at another place (a re-charted shell's input-keyed cell,
/// [`Recharts`]) passes it.
///
/// # Panics
///
/// For a row naming a cell its input does not hold: every cell the op
/// minted is read back to an input cell before its row leaves the op
/// ([`input_cell`], [`respell_through_cuts`]), so a correct kernel
/// names none.
fn rows_name_input_cells<T: Real>(rows: &[crate::Coincidence], inputs: [&Body<T>; 2]) {
    for row in rows {
        for cell in row.every_cell() {
            if let crate::RowCell::Input { input, cell } = cell
                && !super::records::holds(inputs[side(input)], cell)
            {
                unreachable!(
                    "row {row:?} names {input:?}'s {cell:?}, which that input does not hold, \
                     though every cell the op minted is read back to an input cell before its \
                     row leaves the op"
                );
            }
        }
    }
}

/// An operand's index in an `[A, B]` pair.
fn side(input: Operand) -> usize {
    usize::from(input == Operand::B)
}

/// [`boolean_op_with`]'s body.
fn boolean_door<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
    decls: &Glue<T>,
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

/// **∖'s check on an operand part it reverts and nothing joins
/// afterwards**: the single-operand fallback's B whole, and the
/// containment fallback's cavity. Each line where two of its edges lie
/// ([`crate::census::collinear_overlaps`]) reaches the result as the
/// reversal leaves it, so each is refused: where four or more sheets
/// meet on a line, reversing each face's side always mis-pairs them.
///
/// # Errors
///
/// [`BooleanError::Revert`]: such a line. [`BooleanError::Escalated`]
/// ([`BooleanDecision::LineEdges`]): a pair the band cannot read, an
/// in-band reading of the input.
fn no_unjoined_line<T: Decide + Bounds>(
    body: &crate::body::Body<T>,
    band: Band,
) -> Result<(), BooleanError> {
    let lines = crate::census::collinear_overlaps(body, band).map_err(collinear_escalated)?;
    match lines.first() {
        Some(line) => Err(BooleanError::Revert {
            operand: Operand::B,
            source: line.refusal(),
        }),
        None => Ok(()),
    }
}

/// An in-band reading of whether two straight edges lie on one line
/// ([`BooleanDecision::LineEdges`]), of the subtracted solid's or of the
/// result's: a reading of the input, which moving its geometry reaches.
/// Read past the rounding of edges too long for the tolerance
/// ([`crate::census::is_span_precision`]), it is that precision limit
/// instead ([`BooleanDecision::LineSpanPrecision`]), which no move apart
/// reaches.
pub(super) fn collinear_escalated(
    (_, diag): ([crate::entity::EdgeKey; 2], Indeterminate),
) -> BooleanError {
    BooleanError::Escalated {
        decision: if crate::census::is_span_precision(diag.predicate) {
            BooleanDecision::LineSpanPrecision
        } else {
            BooleanDecision::LineEdges
        },
        diag,
    }
}

/// **The result's coincident edges bound disjoint material**: on each
/// line where two or more of the result's edges lie along a length
/// ([`crate::census::collinear_overlaps`]), their material wedges
/// ([`crate::census::line_wedges`]) do not overlap. D1 admits coincident
/// distinct edges only as each classifying its own wedge; where four or
/// more sheets meet on a line the material decides which pair into
/// edges, and an op that pairs them otherwise (a join that pairs each
/// solid's faces with the other's, ∖'s reversal that keeps its
/// operand's pairing) counts the line's material twice, with the
/// volume still right and every tier passing.
///
/// # Errors
///
/// [`BooleanError::LineWedges`]: an overlap, whose re-pairing is not
/// built; or a wedge the reading cannot take, a face with no sheet on
/// the line, a cusp or slit edge (D1: a consumer with no wedge-end
/// answer refuses typed), or an edge too long or short to measure.
/// [`BooleanError::Escalated`]: a reading in band, of whether two of the
/// result's straight edges lie on one line ([`BooleanDecision::LineEdges`])
/// or of their wedges ([`BooleanDecision::LineWedges`]). Both read where
/// the operands' faces meet: the result carries their surfaces
/// unchanged, and a vertex minted where they meet carries the rounding
/// of its computation, far inside the band (where a minted pair's gap is
/// in band the reduction reads it first, at the operands' own gap:
/// `a_doubled_line_does_not_revert::a_minted_edges_in_band_gap_is_the_operands_own`).
/// So a reading in band is a near-coincidence of the operands' geometry,
/// and moving it reaches the reading.
fn coincident_wedges_disjoint<T: Decide + Bounds>(
    result: &Body<T>,
    band: Band,
) -> Result<(), BooleanError> {
    let pairs = crate::census::collinear_overlaps(result, band).map_err(collinear_escalated)?;
    let (i, fault) = match crate::census::line_wedges(result, &pairs, band) {
        Ok(None) => return Ok(()),
        Ok(Some(i)) => (
            i,
            LineWedgeFault::Overlap {
                edges: pairs[i].edges,
            },
        ),
        Err((_, crate::census::WedgeRefusal::Undecided(diag))) => {
            return Err(BooleanError::Escalated {
                decision: BooleanDecision::LineWedges,
                diag,
            });
        }
        Err((i, crate::census::WedgeRefusal::Unread(edge, why))) => {
            (i, LineWedgeFault::Unread { edge, why })
        }
    };
    Err(BooleanError::LineWedges {
        fault,
        from: crate::census::witness(pairs[i].ends[0]),
        to: crate::census::witness(pairs[i].ends[1]),
    })
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

/// The seam edges surviving a boolean's zips, each once, in the order
/// first reported: an edge two zips each glue one of its half-edges
/// along is one edge of the result.
#[derive(Default)]
pub(super) struct SeamEdges {
    listed: Vec<EdgeKey>,
    held: BTreeSet<EdgeKey>,
}

impl SeamEdges {
    fn extend(&mut self, edges: impl IntoIterator<Item = EdgeKey>) {
        for e in edges {
            if self.held.insert(e) {
                self.listed.push(e);
            }
        }
    }

    fn retain(&mut self, keep: impl Fn(&EdgeKey) -> bool) {
        self.listed.retain(&keep);
        self.held.retain(keep);
    }
}

/// The pipeline behind the front door, parameterized on whether the
/// no-crossings sphere RE-CUT (M5 S13) may still run: the re-entry
/// pass sets `recut = false`, so a re-cut that surfaces no crossings
/// is a loud invariant failure rather than a loop.
pub(super) fn boolean_op_recut<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    decls: &Glue<T>,
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
    let edge_classes = red.edge_classes.clone();
    let unread_partners = red.unread_partners.clone();
    let null_copies = super::null_copy_rows(&red.null_edges);
    let copies = Descendants::null_copies(&red.null_edges);
    let along = connected.along.clone();
    let scaffold = [
        connected.completed.iter().map(|p| p.a_face).collect(),
        connected.completed.iter().map(|p| p.b_face).collect(),
    ];
    let ledger = ledger(&red, decls, [&scaffold[0], &scaffold[1]])?;
    let substituted = connected.fates.substituted.clone();
    let tangent_lines = red.tangent_lines.clone();
    let fin = setopfinish(op, red, &connected, a, b, band, tol)?;
    // The zip, the merge, the re-description and the closing mint are
    // one door's surgery (`crate::surgery`): the operators inside them
    // do not each re-derive the whole body, and `gate` below — tier 3
    // over the result, on every build — is what this door pays instead.
    // The guard owns the borrow, so a refusal on the way closes the
    // scope by dropping it.
    let mut finished = fin.body;
    let mut body = finished.begin_surgery();
    let mut seam_edges = SeamEdges::default();
    let mut struts = Vec::new();
    // Each operand's face divisions: the join's chord-mef rows.
    let fragments = [connected.a_fragments.clone(), connected.b_fragments.clone()];
    let mut desc = Descendants::default()
        .with_copies(copies)
        .with_along(along)
        .with_fragments(fragments.clone())
        .with_absorbed(connected.absorbed.clone());
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
        struts.extend(rep.struts);
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
    let mut vertex_merges = desc.vertex_merges();
    let mut cone_splits = cones.splits().clone();
    let glued_pairs = GluedSurfaces::of_result(op, &body, a, b, decls, &fin.graft);
    let views = [&KeyView::Direct, &KeyView::Graft(&fin.graft)];
    let mut discarded = [BTreeSet::new(), BTreeSet::new()];
    // A substituted fragment leaves the result as a discard does.
    for (operand, face) in fin
        .discards
        .iter()
        .map(|r| (r.operand, r.face))
        .chain(substituted.iter().map(|s| (s.operand, s.face)))
    {
        discarded[usize::from(operand == Operand::B)].insert(face);
    }
    let recorded = carried_vertices(&body, &ledger, views, &desc, [&discarded[0], &discarded[1]])?;
    let OutputStage {
        closed,
        merged,
        slivers,
        edge_joins,
        killed_lineage,
    } = finish_output(
        &mut body,
        &mut desc,
        &glued_pairs,
        (&mut seam_edges, &struts),
        &recorded,
        band,
        tol,
    )?;
    let output = output_substitutions(&closed, &merged, &slivers, &edge_joins)?;
    // Curved results carry certified per-half-edge pcurves at rest
    // (M5 PR 9, the PR 6 contract): re-derive the whole cache set on
    // the finished body — the same pass the split lane runs. A planar
    // body mints nothing (no curved faces), so the M3 lane is
    // untouched bit-identically.
    crate::pcurves::mint_pcurves(&mut body, tol)
        .map_err(|source| BooleanError::Pcurves { source })?;
    let (contacts, coincidences) = carry(
        &body,
        &ledger,
        views,
        &desc,
        [&discarded[0], &discarded[1]],
        band,
    )?;
    share_recorded_points(&mut body, &contacts, &coincidences, band)?;
    let (parted, contacts) = super::slit::part_riders(&mut body, &contacts, band, tol)?;
    if !parted.is_empty() {
        crate::pcurves::mint_pcurves(&mut body, tol)
            .map_err(|source| BooleanError::Pcurves { source })?;
    }
    // Each parted edge now bounds a face of each operand, as a seam does.
    seam_edges.extend(parted.iter().flat_map(|p| [p.kept, p.minted]));
    body.sweep_and_close();
    let body = gate(finished, band, tol)?;
    interior_loops?;
    let (graft_vertices, graft_edges, graft_dead_edges, mut graft_faces) = graft_rows(&fin.graft);
    let [mut face_fragments_a, face_fragments_b] = fragments;
    let mut discards = fin.discards;
    name_parted(
        &parted,
        &mut graft_faces,
        &mut face_fragments_a,
        &mut cone_splits,
        &mut vertex_merges,
        &mut discards,
    )?;
    let naming = BooleanNaming {
        a_keys: OperandKeys::Direct,
        b_keys: OperandKeys::Grafted,
        graft_vertices,
        graft_edges,
        graft_dead_edges,
        graft_faces,
        seam_edges: seam_edges.listed,
        vertex_merges,
        merge_groups: merge_rows(&merged, &output)?,
        substitutions: output.clone(),
        merge_skipped: merged.skipped.clone(),
        face_fragments_a,
        face_fragments_b,
        reduction_contacts,
        discards,
        carved_lineage: fin.carved_lineage,
        substituted,
        uncut: Vec::new(),
        edge_classes,
        unread_partners,
        chart_cuts: [Vec::new(), Vec::new()],
        recharted: Default::default(),
        null_copies,
        cone_splits,
        edge_joins,
        killed_lineage,
    };
    Ok(BooleanResult::Body(BooleanBody {
        body,
        kind: BooleanResultKind::Seamed,
        contacts,
        naming,
        verdicts: GlueVerdict::of_rows(&coincidences, &tangent_lines),
        coincidences,
    }))
}

/// **The naming rows of the partings [`super::slit::part_riders`]
/// made.** An end the material parted is held as one copy per wedge, as
/// the zip's per-cone splits are (`cone_splits`); an end joined to the
/// vertex of the face's loop on its point is fused into it, as the zip
/// fuses a seam pair (`vertex_merges`). A face it divided along
/// an edge lying in it descends from the face it was divided off, on that
/// face's side (`graft_faces` for B's, `face_fragments_a` for A's), and
/// lost the line between its pieces: a discard row whose stretch is the
/// line, along which each piece borders the region the other holds.
///
/// # Errors
///
/// [`BooleanError::LineageRow`] where a row would make a list cycle.
fn name_parted(
    parted: &[super::slit::Parted],
    graft_faces: &mut Vec<(FaceKey, FaceKey)>,
    face_fragments_a: &mut crate::lineage::FaceDivisions,
    cone_splits: &mut crate::lineage::ConeCopies,
    vertex_merges: &mut Fusions,
    discards: &mut Vec<super::DiscardRow>,
) -> Result<(), BooleanError> {
    for p in parted {
        for &row in &p.copies {
            cone_splits.push(row)?;
        }
        vertex_merges.extend(&p.fused)?;
        let Some(piece) = p.divided else { continue };
        let (operand, face) = match graft_faces.iter().find(|&&(_, r)| r == p.face) {
            Some(&(from, _)) => {
                graft_faces.push((from, piece));
                (Operand::B, from)
            }
            None => {
                face_fragments_a.push((piece, p.face))?;
                (Operand::A, p.face)
            }
        };
        discards.push(super::DiscardRow {
            operand,
            face,
            bordered: vec![p.ends],
            bordered_through: Vec::new(),
            boundary_chains: Vec::new(),
        });
    }
    Ok(())
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
    /// built body.
    Connected {
        /// The reduction, its operands joined.
        red: Box<BooleanReduction<T>>,
        /// The completed polygons and the fragments the join made.
        connected: Box<super::join::Connected>,
        /// [`interior_loop_verdict`]'s answer, not yet raised.
        interior_loops: Result<(), BooleanError>,
        /// The declarations the pipeline ran on: the caller's, and the
        /// pairs the glue door decided ([`super::glue`]).
        decls: Box<Glue<T>>,
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
    decls: &Glue<T>,
    strategy: SweepStrategy,
    recut: bool,
    tol: Tol,
) -> Result<Joined<T>, BooleanError> {
    let band = Band::linear(tol)?;
    let glued = super::glue::decided_glue(a, b, decls, band)?;
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
        let recuts = match no_crossings_pass(a, b, &red, band)? {
            NoCrossings::Certified(deferred) => {
                return fallback(op, &mut red, a, b, &glued, deferred, band, tol)
                    .map(|result| Joined::Answered(Box::new(result)));
            }
            NoCrossings::Recut(recuts) => recuts,
        };
        if !recut {
            return Err(BooleanError::ClassificationInvariant {
                what: "re-cut sphere operands still produced no crossings",
            });
        }
        let (mut a2, mut b2, recharted) = apply_recuts(a, b, &recuts.rechart, tol)?;
        // The cut-ins name faces of `a` and `b`; the re-charts carve
        // and graft other shells, and `carve` keeps every kept
        // entity's key (`splitting::finish`'s `carve`), so those
        // names still hold on `a2` and `b2`.
        let cuts = apply_cut_ins(&mut a2, &mut b2, &recuts.cut_in, band, tol)?;
        let mut result = boolean_op_recut(op, &a2, &b2, decls, strategy, false, tol)?;
        if let BooleanResult::Body(out) = &mut result {
            // A row names the cells of the op's inputs, not of the
            // cut operands ([`crate::coincidence`]), through the one
            // door ([`spell`]).
            respell_through_cuts(&mut out.coincidences, &cuts, &recharted)?;
            // The cut-ins divided faces before the join did, in the
            // operand keys the join's rows are in.
            let n = &mut out.naming;
            for ((made, joined), edges) in cuts
                .into_iter()
                .zip([&mut n.face_fragments_a, &mut n.face_fragments_b])
                .zip(&mut n.chart_cuts)
            {
                *joined = crate::lineage::FaceDivisions::from_rows(
                    made.divisions
                        .into_iter()
                        .chain(joined.rows().iter().copied()),
                )?;
                edges.extend(made.edges);
            }
            *n.recharted = recharted;
        }
        return Ok(Joined::Answered(Box::new(result)));
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
    let connected = bool_connect(&mut red, a, b, band, tol).and_then(|mut c| {
        c.fates = super::fate::cut(
            &mut red,
            [&mut c.a_fragments, &mut c.b_fragments],
            band,
            tol,
        )?;
        Ok(c)
    });
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
/// where either face is not a plane, is classified and certified. A GLUED pair is exempt: its contact is
/// the carrier the glue door verified, which the covered rungs walk along its edges. The
/// first refusing pair, in arena order (A's faces, then B's), refuses
/// the operation as the operand gate refuses a pair with no arm —
/// [`BooleanError::CurvedPairUnsupported`] with the op named — and its
/// `face` is the pair's A face when that face is curved, else its B
/// face.
///
/// Decided on the unmutated reduction and raised on the built body,
/// so every refusal the join and the build meet first stands verbatim.
/// It is raised after `gate`, whose tiers hold for a correct surgery
/// whatever the classification.
fn interior_loop_verdict<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    red: &BooleanReduction<T>,
    decls: &Glue<T>,
    band: Band,
) -> Result<(), BooleanError> {
    let pairs = crossings_section_pairs(a, b, red, decls, band, true)?;
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

/// **The crossings path's section pairs**, as the interior-loop guard
/// reads them: the reduction's sections and events, the glued pairs
/// exempt. With `stop` the walk returns at the first refusing pair.
fn crossings_section_pairs<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    red: &BooleanReduction<T>,
    decls: &Glue<T>,
    band: Band,
    stop: bool,
) -> Result<Vec<PairVerdict>, BooleanError> {
    let events = event_pairs(red);
    let glued: Vec<(FaceKey, FaceKey)> = decls.pairs.iter().map(|d| (d.a, d.b)).collect();
    section_pairs(
        a,
        b,
        &red.sections,
        band,
        SectionPath::Crossings,
        Exempt::Glued(&glued),
        |fa, fb| events.contains(&(fa, fb)),
        stop,
    )
}

/// **W2's verdict per face, asked once.** Keyed by the operand as well
/// as the face: the two operands' arenas mint their keys independently,
/// so A's face and B's face can share a key. (The map keys the operand
/// as "is B", `Operand` carrying no order.)
#[derive(Default)]
pub(crate) struct ChartCache(BTreeMap<(bool, FaceKey), Sheet>);

impl ChartCache {
    /// Does `operand`'s `face` of `body` lift to one sheet — does every
    /// closed curve in its interior lift to a closed curve with zero
    /// winding (W2's premise)? Asked only of a cylinder, cone or torus
    /// face: no section arm marks a component essential on a plane, a
    /// sphere or a spline, and a plane and a sphere carry no essential
    /// curve at all. Read off the loop walk
    /// ([`crate::pcurves::loop_windings`]), [`Sheet::One`] when
    ///
    /// 1. every loop off the chart's singular set (a cone's apex) winds
    ///    the identity,
    /// 2. at most one loop meets that set, and
    /// 3. a cone face's corners lie on one nappe
    ///    ([`crate::offset_nappe::face_nappe`]).
    ///
    /// **Why that is enough.** Its premises: `int F` is the face less
    /// its edges, seams included, because the sweep examines every edge
    /// of `F` against `G`, a seam's two halves among them, so a section
    /// crossing any edge carries an event (L1, premise S); and `F`
    /// carries no lone-vertex loop, which [`pair_verdict`] refuses
    /// (`Refusal::LoneVertex`) before W2 is asked, so every loop the walk
    /// reads is a cycle. Then, on
    /// a cylinder or a cone a closed curve `γ` off the apex winds by its
    /// class in the first homology of the carrier less the apex, and `γ`
    /// in `int F` is homologous there to a sum over every boundary
    /// circle of `F` but one, and over a small circle about the apex
    /// where `int F` holds it: `F` is planar, since it lies on a planar
    /// carrier. Each loop off the apex lies off it, so its term is its
    /// winding, zero by (1); the loop through the apex, if any, is the
    /// one left out (2); and an apex in `int F` sits on no loop, so the
    /// sum over all of them would be ±1, which (1) excludes. The nappe
    /// clause excludes a loop that crosses the apex inside an edge,
    /// whose term is not its walk's winding. A pinched face (two sectors
    /// meeting at the apex) reads the same of each sector. A torus has
    /// no singular set, and (1) is enough while its face is planar, as a
    /// face its seams cut open is.
    ///
    /// A walk that refuses for a numerical or unbuilt reason is
    /// [`Sheet::Unread`], never `Not`.
    ///
    /// # Errors
    ///
    /// [`BooleanError::ClassificationInvariant`] for a torn face
    /// ([`WalkRefusal::Torn`]: a state the at-rest pass refuses) or one
    /// the walk cannot be asked of, and for a face on a plane, a sphere
    /// or a spline, which no section arm asks about.
    pub(crate) fn describes<T: crate::props::AtRestPolicy>(
        &mut self,
        operand: Operand,
        body: &Body<T>,
        face: FaceKey,
        surface: &geom::Surface<T>,
        band: Band,
    ) -> Result<Sheet, BooleanError> {
        let key = (operand == Operand::B, face);
        if let Some(&sheet) = self.0.get(&key) {
            return Ok(sheet);
        }
        let sheet = one_sheet(body, face, surface, band)?;
        self.0.insert(key, sheet);
        Ok(sheet)
    }
}

/// [`ChartCache::describes`]'s reading, uncached.
fn one_sheet<T: crate::props::AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Sheet, BooleanError> {
    use geom::Surface as S;
    match surface {
        S::Cylinder { .. } | S::Cone { .. } | S::Torus { .. } => {}
        S::Plane { .. } | S::Sphere { .. } | S::Nurbs(_) | S::Approx(_) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "the section certificate's W2 was asked of a face on a plane, a sphere or \
                       a spline, on which no section arm marks a component essential",
            });
        }
    }
    let torn = || BooleanError::ClassificationInvariant {
        what: "an operand face's loop walk reads it torn (a stale key, an edge with no carrier, \
               a missing or misstated row, a row that is not its edge's on the face, or a walk \
               that does not close), which the at-rest pass refuses",
    };
    let loops = match crate::pcurves::loop_windings(body, face, surface, band) {
        Ok(loops) => loops,
        Err(e) => {
            return match walk_refusal(&e) {
                WalkRefusal::Torn => Err(torn()),
                WalkRefusal::Unread => Ok(Sheet::Unread),
                WalkRefusal::NotRaised => Err(BooleanError::ClassificationInvariant {
                    what: "W2's loop walk answered a refusal only the chart description or the \
                           at-rest pass raises",
                }),
            };
        }
    };
    if !loops_lift(&loops) {
        return Ok(Sheet::Not);
    }
    if matches!(surface, S::Cone { .. }) {
        use crate::offset_nappe::NappeRefusal;
        match crate::offset_nappe::face_nappe_reading(body, face, band) {
            Ok(_) => {}
            Err(NappeRefusal::Straddles { .. }) => return Ok(Sheet::Not),
            Err(NappeRefusal::Escalated(_)) => return Ok(Sheet::Unread),
            Err(NappeRefusal::Stale) => return Err(torn()),
        }
    }
    Ok(Sheet::One)
}

/// How [`ChartCache::describes`] reads a refusal of the loop walk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WalkRefusal {
    /// The face is not what its rest certificate says: a kernel defect.
    Torn,
    /// A numerical or unbuilt refusal: W2 reads nothing of the face.
    Unread,
    /// A refusal [`crate::pcurves::loop_windings`] never returns: only
    /// the at-rest pass raises it.
    NotRaised,
}

/// [`WalkRefusal`] of `e`, matched whole so a new refusal is read here
/// before it is folded anywhere. A certification refusal is torn where
/// it says the stored rows are not the face's (a malformed or misstated
/// row, a span or a winding past its gate, a carrier decided off its
/// chart: a definite contradiction the kernel holds), and unread where
/// it is a precision limit, a lane not built or a read in band (a
/// carrier at a pose where its chart holds no curve of its kind, D3): a
/// face whose rows are not owed (`pcurves::not_owed`) is legal at rest,
/// and its fresh derivation can meet one. An incidence refusal is read
/// by the one classifier ([`geom_brep::IncidenceRead::of`]) and its one
/// policy ([`geom_brep::IncidenceRead::is_definite`]), as its ending is.
fn walk_refusal(e: &crate::pcurves::PcurveMintError) -> WalkRefusal {
    use crate::pcurves::PcurveMintError as M;
    match e {
        M::Stale { .. }
        | M::PlaceholderChart { .. }
        | M::NoCarrier { .. }
        | M::LoopDiscontinuity { .. }
        | M::LoopNotClosed { .. }
        | M::MissingCache { .. }
        | M::RowInterval { .. } => WalkRefusal::Torn,
        M::Escalated { .. }
        | M::NetStretchUndecided { .. }
        | M::JointWithoutRoom { .. }
        | M::Band(_) => WalkRefusal::Unread,
        M::Unminted { .. } => WalkRefusal::NotRaised,
        // An incidence refusal is read by the one classifier and its one
        // policy: a definite read (decided off, or a margin that could
        // not be read) is torn, a read in band unread.
        M::Certify { error, .. } => match geom_brep::IncidenceRead::of(error) {
            Some(read) if read.is_definite() => WalkRefusal::Torn,
            Some(_) => WalkRefusal::Unread,
            None => walk_certify_refusal(error),
        },
    }
}

/// [`walk_refusal`] of a certification refusal that carries no incidence
/// read (the incidence variants are listed for the match, and read by
/// the classifier before they reach it).
fn walk_certify_refusal(error: &geom_brep::PcurveCertifyError) -> WalkRefusal {
    use geom_brep::PcurveCertifyError as C;
    match error {
        C::PlaceholderChart
        | C::CarrierOffChart { .. }
        | C::ChartRow { .. }
        | C::ImageMismatch { .. }
        | C::ChannelKindMismatch { .. }
        | C::WindingMismatch { .. }
        | C::FittedCertificate { .. }
        | C::IntervalNotForward
        | C::AzimuthPeriodExceeded
        | C::TubePeriodExceeded => WalkRefusal::Torn,
        C::UnsupportedChart { .. }
        | C::IsoUnsupported { .. }
        | C::ChartWindingUnsupported
        | C::NoGeneralLane { .. }
        | C::SectorRefused { .. }
        | C::FittedLaneUnsupported { .. }
        | C::CarrierDomain(_)
        | C::PlacementTurnsExceeded
        | C::CarrierEnclosure(_)
        | C::ResidualExceeded { .. }
        | C::FittedLimb { .. }
        | C::FittedEscalated { .. }
        | C::Escalated { .. }
        | C::Band(_) => WalkRefusal::Unread,
    }
}

/// Clauses (1) and (2) of [`ChartCache::describes`]: every loop off the
/// singular set winds the identity, and at most one loop meets it.
pub(crate) fn loops_lift(loops: &[crate::pcurves::LoopWinding]) -> bool {
    loops.iter().filter(|l| l.singular).count() <= 1 && loops.iter().all(|l| l.singular || l.zero)
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
///
/// # Errors
///
/// [`ChartCache::describes`]'s, for a torn face.
fn pair_verdict<T: Decide + Bounds + crate::props::AtRestPolicy>(
    (a, fa): (&Body<T>, &FaceRow<T>),
    (b, fb): (&Body<T>, &FaceRow<T>),
    sections: &super::section_table::SectionTable<T>,
    band: Band,
    evented: bool,
    charts: &mut ChartCache,
) -> Result<Result<Vec<super::section_cert::Cleared>, SectionRefusal>, BooleanError> {
    use super::section_cert::{Side, certify, classify, of_carried, of_pair_section};
    use super::section_table::Uncarried;
    if has_lone_vertex(a, fa.face) || has_lone_vertex(b, fb.face) {
        return Ok(Err(SectionRefusal::LoneVertex));
    }
    let section = match sections.held((a, fa.face), (b, fb.face)) {
        Some(held) => of_pair_section(held, band, |_| {
            match sections.carried((a, fa.face), (b, fb.face)) {
                Some(Ok(carried)) => of_carried(Ok(carried)),
                Some(Err(Uncarried::Carry(e))) => of_carried(Err(e)),
                Some(Err(Uncarried::Section(_) | Uncarried::Component)) | None => {
                    super::section_cert::Section::Unmarched
                }
            }
        }),
        None => classify(
            &fa.surface,
            &fb.surface,
            overlap_reach(&fa.bbox, &fb.bbox),
            T::ssi_lane(),
            band,
        ),
    };
    let mut torn = None;
    let verdict = certify(
        &section,
        evented,
        band,
        |side| {
            let (operand, body, row) = match side {
                Side::F => (Operand::A, a, fa),
                Side::G => (Operand::B, b, fb),
            };
            charts
                .describes(operand, body, row.face, &row.surface, band)
                .unwrap_or_else(|e| {
                    torn = Some(e);
                    Sheet::Unread
                })
        },
        |p, band| {
            [
                place_witness(a, fa.face, p, band),
                place_witness(b, fb.face, p, band),
            ]
        },
    );
    match torn {
        Some(e) => Err(e),
        None => Ok(verdict),
    }
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
///
/// # Errors
///
/// [`pair_verdict`]'s.
#[allow(clippy::too_many_arguments)]
fn walk_pairs<'r, T: Decide + Bounds + crate::props::AtRestPolicy + 'r>(
    (a, a_rows): (&Body<T>, impl IntoIterator<Item = &'r FaceRow<T>>),
    (b, b_rows): (&Body<T>, impl IntoIterator<Item = &'r FaceRow<T>>),
    sections: &super::section_table::SectionTable<T>,
    band: Band,
    charts: &mut ChartCache,
    admit: impl Fn(&FaceRow<T>, &FaceRow<T>) -> bool,
    evented: impl Fn(FaceKey, FaceKey) -> bool,
    named: impl Fn(&geom::Surface<T>) -> bool,
    stop: bool,
    axes: &super::separating::OperandAxes<T>,
) -> Result<Vec<PairVerdict>, BooleanError> {
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
            let verdict = pair_verdict(
                (a, fa),
                (b, fb),
                sections,
                band,
                evented(fa.face, fb.face),
                charts,
            )?;
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
                return Ok(out);
            }
        }
    }
    Ok(out)
}

/// **The section certificate over every in-scope pair** of `a` × `b`
/// whose certified boxes overlap, which the narrow phase does not part
/// and which `exempt` does not answer, in arena order ([`walk_pairs`]). `evented` says whether the reduction
/// recorded an event on the pair `(A face, B face)`. With `stop` the
/// scan returns at the first refusing pair. W2's question is asked once
/// per face and cached ([`ChartCache`]).
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for a face whose surface
/// or loops do not resolve, or whose loop walk reads it torn
/// ([`ChartCache::describes`]), and the box builder's own errors.
#[allow(clippy::too_many_arguments)]
pub(crate) fn section_pairs<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    sections: &super::section_table::SectionTable<T>,
    band: Band,
    path: SectionPath,
    exempt: Exempt<'_>,
    evented: impl Fn(FaceKey, FaceKey) -> bool,
    stop: bool,
) -> Result<Vec<PairVerdict>, BooleanError> {
    let (a_rows, b_rows) = (face_rows(a, band)?, face_rows(b, band)?);
    walk_pairs(
        (a, &a_rows),
        (b, &b_rows),
        sections,
        band,
        &mut ChartCache::default(),
        |fa, fb| path.scope(&fa.surface, &fb.surface) && !exempt.answers(fa.face, fb.face),
        evented,
        |s| path.names(s),
        stop,
        &super::separating::OperandAxes::new(boxes::axis_key),
    )
}

/// **The pairs a section walk answers without their section**, one per
/// path, each keyed to its exact `(A face, B face)` pair: a face
/// exempted against one partner meets every other partner as any face
/// does.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Exempt<'r> {
    /// Every in-scope pair is classified: the certificate's own rows'
    /// spelling, which no production path takes.
    #[cfg(any(test, feature = "test-support"))]
    Nothing,
    /// The crossings path: the GLUED pairs, `(A face, B face)`, whose
    /// contact is the carrier the glued rungs walk along their edges. A
    /// glued pair covers itself and no other: a face glued to one
    /// partner meets every other partner as any face does.
    Glued(&'r [(FaceKey, FaceKey)]),
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
            #[cfg(any(test, feature = "test-support"))]
            Self::Nothing => false,
            Self::Glued(pairs) => pairs.contains(&(fa, fb)),
            Self::Rest(pairs) => pairs.contains(&(fa, fb)),
        }
    }
}

/// **What the reduction hands the no-crossings certificates**
/// ([`sphere_extent_scan`], [`section_extent_pass`]).
struct Evidence<'r> {
    /// The verified `Rest` pairs ([`Exempt::Rest`]), answered without
    /// their section.
    rest: &'r [(FaceKey, FaceKey)],
    /// The pairs the reduction settled one carrier, whose refusals are
    /// held for the fallback's shell verdicts ([`OnDeferred`]).
    settled: &'r [super::SettledPair],
    /// The pairs the sweep recorded a contact on ([`event_pairs`]):
    /// touches, since no crossing was found. W4 reads them.
    events: &'r BTreeSet<(FaceKey, FaceKey)>,
}

impl<'r> Evidence<'r> {
    /// What `red` holds for the certificates, its touches read once
    /// into `events` ([`event_pairs`]).
    fn of<T: Real>(red: &'r BooleanReduction<T>, events: &'r BTreeSet<(FaceKey, FaceKey)>) -> Self {
        Self {
            rest: &red.rest_contacts,
            settled: &red.coincident,
            events,
        }
    }
}

/// **What the no-crossings certificates answer** ([`no_crossings_pass`]).
enum NoCrossings<T: Real> {
    /// The extent scan asks for re-cuts; the pipeline re-enters on the
    /// re-cut operands.
    Recut(Recuts<T>),
    /// Every pair is certified, or held for the `On` verdict.
    Certified(OnDeferred),
}

/// **The no-crossings path's certificates, in the order the path runs
/// them**: the sphere extent scan, then, where it asks for no re-cut,
/// the section pass, both over what the reduction holds
/// ([`Evidence::of`]).
fn no_crossings_pass<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    red: &BooleanReduction<T>,
    band: Band,
) -> Result<NoCrossings<T>, BooleanError> {
    let events = event_pairs(red);
    let evidence = Evidence::of(red, &events);
    let (recuts, deferred) = sphere_extent_scan(a, b, &evidence, band)?;
    if !recuts.rechart.is_empty() || !recuts.cut_in.is_empty() {
        return Ok(NoCrossings::Recut(recuts));
    }
    section_extent_pass(a, b, &red.sections, &evidence, deferred, band).map(NoCrossings::Certified)
}

impl Evidence<'static> {
    /// No reduction behind the certificates: no `Rest` pair, no settled
    /// pair, no event.
    #[cfg(test)]
    const NONE: Self = Self {
        rest: &[],
        settled: &[],
        events: &BTreeSet::new(),
    };
}

/// **The no-crossings path's section pass.** Each pair the vertex probe
/// could not see into is certified here, with the sweep's touches as
/// its events, or refused. W4 clears a lone component a touch lies on,
/// and with no crossing anywhere such a component meets neither face's
/// interior ([`super::section_cert`]'s W4 says why). A refusal is
/// typed as the fallback's extent refusal
/// ([`BooleanError::FallbackExtentUnsupported`]) naming the pair's
/// curved face. A verified `Rest` pair is answered without its section,
/// and a refusing pair the reduction settled one carrier is held
/// ([`OnDeferred::hold`]).
fn section_extent_pass<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    sections: &super::section_table::SectionTable<T>,
    evidence: &Evidence<'_>,
    mut deferred: OnDeferred,
    band: Band,
) -> Result<OnDeferred, BooleanError> {
    for p in &fallback_section_pairs(a, b, sections, evidence, band)? {
        let Err(refusal) = p.verdict else {
            continue;
        };
        let (operand, face, ..) = p.named();
        deferred.hold(
            evidence.settled,
            (p.a_face, p.b_face),
            BooleanError::FallbackExtentUnsupported {
                operand,
                face,
                what: refusal.what(),
            },
        )?;
    }
    Ok(deferred)
}

/// **The no-crossings path's section pairs, every one**, as
/// [`section_extent_pass`] reads them: a verified `Rest` pair exempt,
/// the sweep's touches as events.
fn fallback_section_pairs<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    sections: &super::section_table::SectionTable<T>,
    evidence: &Evidence<'_>,
    band: Band,
) -> Result<Vec<PairVerdict>, BooleanError> {
    section_pairs(
        a,
        b,
        sections,
        band,
        SectionPath::Fallback,
        Exempt::Rest(evidence.rest),
        |fa, fb| evidence.events.contains(&(fa, fb)),
        false,
    )
}

/// **The no-crossings certificates' refusals on settled one-carrier
/// pairs**, each `(A face, B face, refusal)` in the order they were
/// raised, held for the fallback's shell verdicts.
#[derive(Debug, Default)]
struct OnDeferred(Vec<(FaceKey, FaceKey, BooleanError)>);

impl OnDeferred {
    /// Holds `refusal` on the pair `(fa, fb)` where the reduction
    /// settled the pair one carrier, and returns it otherwise. A settled
    /// pair's section is not a curve but the part of the one carrier
    /// the two faces share, so no certificate reads it; the fallback
    /// answers it from both faces' shells' verdicts ([`Self::answered`]).
    fn hold(
        &mut self,
        settled: &[super::SettledPair],
        (fa, fb): (FaceKey, FaceKey),
        refusal: BooleanError,
    ) -> Result<(), BooleanError> {
        if settled.iter().any(|q| q.a == fa && q.b == fb) {
            self.0.push((fa, fb, refusal));
            Ok(())
        } else {
            Err(refusal)
        }
    }

    /// Each deferred pair answered, or the first one's refusal. A pair
    /// `(fa, fb)` is answered where `fa`'s shell reads `On` `fb`'s shell
    /// and `fb`'s reads `On` `fa`'s (`working` the shells' verdicts,
    /// `pristine` the operands their partners are named in). Nothing
    /// here reads the pair's patch: what carries the answer is the `On`
    /// verdict ([`super::shell_witness`]), that every witness the ladder
    /// takes of each shell, its vertices and its edges' midpoints, lies
    /// on the other solid's boundary, and that every face of each is
    /// settled one carrier with a face of the other at one orientation.
    /// The ladder takes no interior witness on a curved face, so two
    /// faces on one carrier whose patches differ read `On` only where
    /// their edges do too; where an edge of one runs off the other
    /// solid's boundary (a torus lens inside the union of its tori), its
    /// midpoint sides the shell and the pair keeps its refusal.
    ///
    /// A pair is answered by its shells' sides instead where both shells
    /// read a side and the op keeps the shells of one operand at most
    /// (`keeps`, A then B). What certifies the sides is that the fallback
    /// runs with no null pair ([`fallback`]). Where one operand's boundary
    /// leaves the shared carrier inside a face of the other, the reduction
    /// classifies the vertex there against that face, its sectors on the
    /// carrier lumped by Eq. 15.3, and mints a null pair (or refuses)
    /// unless every sector reads one class: so the face that leaves lies
    /// on the side of its operand's lump. Where the other operand's
    /// boundary leaves instead, the first operand's face runs on across
    /// the edge, opposite the leaving face for aligned senses and with it
    /// for opposed ones; Eq. 15.3 lumps the two operands oppositely for
    /// aligned senses and alike for opposed ones, so that is the side of
    /// the first operand's lump too. Every way a shell leaves a shared
    /// patch is to its lump's side, so no shell lies on both sides of the
    /// other's boundary, and keeping a shell keeps its patch where
    /// Eq. 15.3 does. Two rods overlapping over part of each one's length
    /// never reach this: each rim lies in the other's wall, the two lumps
    /// differ, so one rim reads a transition whatever the op and the join
    /// cuts both walls.
    fn answered<T: Decide>(
        self,
        working: [(&Body<T>, &[super::shell_witness::Verdict]); 2],
        pristine: [&Body<T>; 2],
        keeps: [bool; 2],
    ) -> Result<(), BooleanError> {
        // `f`'s shell, in `body`'s keys.
        let shell = |body: &Body<T>, f: FaceKey| body.get_face(f).map(|face| face.shell);
        let verdict = |i: usize, f: FaceKey| {
            let (body, sides) = working[i];
            let mine = shell(body, f)?;
            sides.iter().find(|&&(s, _)| s == mine).map(|&(_, v)| v)
        };
        // `f`'s shell of `working[i]` reads `On` the shell of `g` in
        // `pristine[1 − i]`.
        let on = |i: usize, f: FaceKey, g: FaceKey| {
            let Some(theirs) = shell(pristine[1 - i], g) else {
                return false;
            };
            matches!(verdict(i, f), Some(ShellVerdict::On { partner, .. }) if partner == theirs)
        };
        let both_sided = |fa: FaceKey, fb: FaceKey| {
            matches!(verdict(0, fa), Some(ShellVerdict::Side(_)))
                && matches!(verdict(1, fb), Some(ShellVerdict::Side(_)))
        };
        let by_sides =
            !(keeps[0] && keeps[1]) && self.0.iter().all(|&(fa, fb, _)| both_sided(fa, fb));
        match self
            .0
            .into_iter()
            .find(|(fa, fb, _)| !by_sides && !(on(0, *fa, *fb) && on(1, *fb, *fa)))
        {
            None => Ok(()),
            Some((.., refusal)) => Err(refusal),
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
pub(crate) fn section_report(
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
    tol: Tol,
) -> Result<Vec<PairVerdict>, BooleanError> {
    let (red, glued, band) = reduced_as_the_op_does(op, a, b, tol)?;
    crossings_section_pairs(a, b, &red, &glued, band, false)
}

/// **The glue door and the reduction, run as the op runs them** on
/// undeclared operands: the test doors' road to the pipeline's own
/// readings.
#[cfg(any(test, feature = "test-support"))]
fn reduced_as_the_op_does(
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
    tol: Tol,
) -> Result<(BooleanReduction<f64>, Glue<f64>, Band), BooleanError> {
    let band = Band::linear(tol)?;
    let none = Glue::none();
    let glued = super::glue::decided_glue(a, b, &none, band)?.into_owned();
    let red =
        super::boolean_reduce_declared_strategy(op, a, b, &glued, SweepStrategy::Realized, tol)?;
    Ok((red, glued, band))
}

/// **The section pass's per-pair report on the no-crossings path**:
/// every pair the pass examines, read as the pass reads it
/// ([`fallback_section_pairs`]) over `op`'s glue door and reduction,
/// whatever the crossing layer found. A pair the reduction
/// settled one carrier reports its refusal, which the pass holds.
///
/// # Errors
///
/// The glue door's and the reduction's refusals, and [`section_pairs`]'.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn no_crossings_section_report(
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
    tol: Tol,
) -> Result<Vec<PairVerdict>, BooleanError> {
    let (red, _, band) = reduced_as_the_op_does(op, a, b, tol)?;
    let events = event_pairs(&red);
    fallback_section_pairs(a, b, &red.sections, &Evidence::of(&red, &events), band)
}

/// **The section certificate as if the sweep had found nothing**: every
/// pair the no-crossings pass would examine, no pair exempt, no event,
/// no section table. No production path reads this; it is the reading
/// a certificate arm's own rows take on a pose the crossing layer would
/// see, to reach the no-event decision.
///
/// # Errors
///
/// [`section_pairs`]'.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn no_event_section_report(
    a: &Body<f64>,
    b: &Body<f64>,
    tol: Tol,
) -> Result<Vec<PairVerdict>, BooleanError> {
    section_pairs(
        a,
        b,
        &super::section_table::SectionTable::default(),
        Band::linear(tol)?,
        SectionPath::Fallback,
        Exempt::Nothing,
        |_, _| false,
        false,
    )
}

/// **The no-crossings path's two certificates**, the path's own
/// [`no_crossings_pass`] over `op`'s glue door and reduction, whatever
/// the crossing layer found. `Ok` with the number of re-cuts the scan
/// asked for and the number of refusals held for the `On` verdict,
/// which this door does not run.
///
/// # Errors
///
/// The glue door's and the reduction's refusals, and either
/// certificate's.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn no_crossings_certificates(
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
    tol: Tol,
) -> Result<(usize, usize), BooleanError> {
    let (red, _, band) = reduced_as_the_op_does(op, a, b, tol)?;
    Ok(match no_crossings_pass(a, b, &red, band)? {
        NoCrossings::Recut(recuts) => (recuts.rechart.len() + recuts.cut_in.len(), 0),
        NoCrossings::Certified(held) => (0, held.0.len()),
    })
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

/// The merge's groups as naming rows, `(kept, absorbed…)`.
///
/// # Errors
///
/// [`BooleanError::LineageRow`] where a group's kept face is one the
/// output stage's substitutions retire (`output`): the kept face would
/// be absorbed, by its own group or a later one, whatever order the
/// groups come in. A face absorbed twice is refused where `output`
/// takes its second row.
pub(super) fn merge_rows(
    m: &crate::merge_faces::MergeCoplanarOutcome,
    output: &crate::lineage::Substitutions,
) -> Result<Vec<(FaceKey, Vec<FaceKey>)>, BooleanError> {
    m.groups
        .iter()
        .map(|g| match output.get(Cell::Face(g.kept)) {
            None => Ok((g.kept, g.absorbed.clone())),
            Some(_) => Err(crate::lineage::RowRefusal::of(
                output.tag(),
                crate::lineage::RowDefect::NamesRetired,
                Cell::Face(g.kept),
            )
            .into()),
        })
        .collect()
}

/// The output stage's substitutions as naming rows
/// ([`BooleanNaming::substitutions`]): each closed strut's edge and
/// vertices onto its face, the merge's lineage
/// ([`MergedGroup::substitutions`](crate::merge_faces::MergedGroup::substitutions)),
/// then each join's joined-away vertex and edge onto its kept edge.
///
/// # Errors
///
/// [`BooleanError::LineageRow`] at the first row the list refuses.
pub(super) fn output_substitutions(
    closed: &[super::slit::ClosedStrut],
    m: &crate::merge_faces::MergeCoplanarOutcome,
    slivers: &[super::slit::Sliver],
    joins: &[super::EdgeJoin],
) -> Result<crate::lineage::Substitutions, BooleanError> {
    let mut rows = crate::lineage::Substitutions::new(crate::lineage::LineageRows::Substitutions);
    let merged = m
        .groups
        .iter()
        .flat_map(crate::merge_faces::MergedGroup::substitutions);
    let joined = joins.iter().flat_map(|j| {
        let kept = Cell::Edge(j.kept);
        [
            Some(Cell::Vertex(j.vertex)),
            Some(Cell::Edge(j.gone)),
            j.conventional.map(Cell::Vertex),
        ]
        .into_iter()
        .flatten()
        .map(move |dead| (dead, kept))
    });
    closed
        .iter()
        .flat_map(super::slit::ClosedStrut::substitutions)
        .chain(merged)
        .chain(slivers.iter().flat_map(super::slit::Sliver::substitutions))
        .chain(joined)
        .try_for_each(|row| rows.push(row))?;
    Ok(rows)
}

/// What [`finish_output`] made.
pub(super) struct OutputStage {
    /// The struts closed away ([`super::slit::close_struts`]).
    pub(super) closed: Vec<super::slit::ClosedStrut>,
    /// The merge of coplanar faces.
    pub(super) merged: crate::merge_faces::MergeCoplanarOutcome,
    /// The slivers merged away
    /// ([`super::slit::merge_slivers`]).
    pub(super) slivers: Vec<super::slit::Sliver>,
    /// The joins, in the order made ([`BooleanNaming::edge_joins`]).
    pub(super) edge_joins: Vec<super::EdgeJoin>,
    /// The parents of the edges the closing, the merge, the slivers'
    /// merge and the join killed ([`BooleanNaming::killed_lineage`]).
    pub(super) killed_lineage: BTreeMap<EdgeKey, EdgeKey>,
}

/// **The boolean's output stage**, the one every result path takes
/// (`docs/DESIGN.md`, the merge stage): the closing of a face across
/// each of the zip's `struts` and `seam_edges` that it lies on both
/// sides of ([`super::slit::close_struts`]), the merge of coplanar faces over the
/// `declared` surface pairs (none where one operand is absent from the
/// result), the re-description of the edges the op minted (`seam_edges`,
/// plus the boundaries of groups the merge skipped), the merge of each
/// sliver's two edges where its line holds no other edge
/// ([`super::slit::merge_slivers`]), and the join
/// ([`super::edge_join::join_stage`]), each writing its rows into the
/// op's one `desc`. The result then has maximal faces and maximal edges,
/// and its records are carried over `desc` after. `seam_edges` loses the
/// edges the closing and the slivers' merge kill.
///
/// # Errors
///
/// The closing's ([`super::slit::close_struts`]), the merge's
/// ([`BooleanError::Merge`]), the description's, the slivers' merge's,
/// or the join's.
pub(super) fn finish_output<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    desc: &mut Descendants,
    glued: &GluedSurfaces,
    (seam_edges, struts): (&mut SeamEdges, &[EdgeKey]),
    recorded: &BTreeSet<VertexKey>,
    band: Band,
    tol: Tol,
) -> Result<OutputStage, BooleanError> {
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
    // One snapshot of the records serves every stage: nothing between
    // the closing and the join mints an edge that survives it.
    let split_from = super::edge_join::split_records(body);
    // The zip names the section struts it keeps (`ZipReport::struts`),
    // not a seam edge its gluing leaves with one face on both sides,
    // which the closing deletes as well: each candidate once, the named
    // struts first.
    let mut seen = BTreeSet::new();
    let candidates: Vec<EdgeKey> = struts
        .iter()
        .chain(&seam_edges.listed)
        .copied()
        .filter(|&e| seen.insert(e))
        .collect();
    let closed = super::slit::close_struts(body, &candidates, tol)?;
    let mut killed_lineage = BTreeMap::new();
    super::edge_join::record_killed_lineage(
        &mut killed_lineage,
        &split_from,
        closed.iter().map(|c| c.edge),
    );
    for row in closed
        .iter()
        .flat_map(super::slit::ClosedStrut::substitutions)
    {
        desc.substitute(row)?;
    }
    seam_edges.retain(|e| !closed.iter().any(|c| c.edge == *e));
    // The closing sweeps the arena, so a glued surface no face held (a
    // dropped copy's) is gone now: its pair licenses nothing, as a
    // consumed side does at `GluedSurfaces::of_result`.
    let merged = body.merge_coplanar_faces_unjoined(&glued.surviving(body), tol)?;
    super::edge_join::record_killed_lineage(
        &mut killed_lineage,
        &split_from,
        merged
            .groups
            .iter()
            .flat_map(|g| g.killed_edges.iter().copied()),
    );
    desc.absorb_merge(&merged)?;
    describe_minted_edges(body, &seam_edges.listed, &merged, band, tol)?;
    let slivers = super::slit::merge_slivers(body, band, tol)?;
    super::edge_join::record_killed_lineage(
        &mut killed_lineage,
        &split_from,
        slivers.iter().map(|s| s.killed),
    );
    for row in slivers.iter().flat_map(super::slit::Sliver::substitutions) {
        desc.substitute(row)?;
    }
    seam_edges.retain(|e| !slivers.iter().any(|s| s.killed == *e));
    let edge_joins = join_stage(
        body,
        desc,
        recorded,
        &split_from,
        &mut killed_lineage,
        band,
        tol,
    )?;
    Ok(OutputStage {
        closed,
        merged,
        slivers,
        edge_joins,
        killed_lineage,
    })
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
/// them before it returns ([`Body::merge_coplanar_faces`]).
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
        DescribeRefusal::Mixed { edge, stations } => BooleanError::MixedSeam { edge, stations },
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

/// Describes each worklist edge from its two faces' surfaces
/// (structural adjacency): definitely transverse ⇒ `Intersection` with
/// the chord-midpoint witness; definitely smooth ⇒ the must-carry rule
/// over the edge ([`must_carry_reading`]) — the intrinsic
/// `TangentIntersection` where the surfaces determine the locus, else
/// the conventional description (D2's split); an undecided reading,
/// or a seam a corner at one station and smooth at another, refuses
/// typed. `skipped` is the merge stage's record of the groups
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
        let draft = geom_brep::IntersectionDraft::of(existing.as_ref(), p0, p1);
        let (carrier, interval) = draft.span();
        let reach =
            crate::wedge_reach(body, [sides.plus.face, sides.minus.face], carrier, interval);
        let witness = draft.witness;
        let face = |side, surface| seam_face(body, edge, side, surface, existing.as_ref(), band);
        let faces = [face(sides.plus, surf1)?, face(sides.minus, surf2)?];
        match seam_reading(faces, witness, reach, band).map_err(|refusal| match refusal {
            geom_brep::SeamEscalation::Lever(e) => {
                let (reading, diag) = DihedralReading::of_lever(e);
                DescribeRefusal::undecided(edge, reading, diag)
            }
            geom_brep::SeamEscalation::Crease(_) => {
                DescribeRefusal::failed(edge, EdgeDescribeFailure::Crease)
            }
            geom_brep::SeamEscalation::SplineReadAsAnalytic => {
                unreachable!("seam_face reads every spline face by its chart")
            }
        })? {
            geom_brep::DihedralClass::Transverse => {
                let carrier = draft.span().0.kind();
                let drafted = crate::pcurves::seam_intersection(
                    body,
                    draft,
                    [
                        (sides.plus.half_edge, s1, surf1),
                        (sides.minus.half_edge, s2, surf2),
                    ],
                    band,
                );
                body.describe_intersection(
                    crate::policy_lane::KernelSite::SeamIntersection,
                    edge,
                    drafted,
                    tol,
                )
                .map_err(|e| of_attachment(edge, EdgeDescribeFailure::Intersection, carrier, e))?;
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
                    | geom_brep::EdgeDescription::TangentIntersection { pair, .. }
                    | geom_brep::EdgeDescription::OnPair { pair } => {
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
                // re-reads (`must_carry_reading`). A seam that reads a
                // corner at some station and smooth at another is
                // certifiable as neither, and refuses (D2).
                let reading = {
                    let c = existing.as_ref().ok_or_else(corrupt)?;
                    let (t0, t1) = c.params();
                    let (_, [sa, sb]) = geom_brep::SurfacePair::sorted(s1, surf1, s2, surf2);
                    must_carry_reading(sa, sb, c.carrier(), t0, t1, reach, band).map_err(
                        |refusal| match refusal {
                            SeamRefusal::Undecided(reading, diag) => {
                                DescribeRefusal::undecided(edge, reading, diag)
                            }
                            SeamRefusal::Mixed(stations) => {
                                DescribeRefusal::Mixed { edge, stations }
                            }
                        },
                    )?
                };
                if reading == SeamReading::Intrinsic {
                    // Mint the intrinsic tangency on the existing
                    // carrier (U2: today's taxonomy, 1:1 onto
                    // (surface, exact-lane pcurve)) — this also
                    // refreshes a stale citation, since the minted
                    // surfaces are the CURRENT adjacency.
                    let c = existing.as_ref().ok_or_else(corrupt)?;
                    let description = geom_brep::EdgeDescriptionSpec::TangentIntersection {
                        pair: geom_brep::SurfacePair::new(s1, s2),
                        witness,
                    };
                    body.restate_edge(
                        crate::policy_lane::KernelSite::SeamTangency,
                        edge,
                        description,
                        tol,
                    )
                    .map_err(|e| {
                        of_attachment(edge, EdgeDescribeFailure::Tangency, c.carrier().kind(), e)
                    })?;
                } else if stale {
                    // A locus the adjacent surfaces under-determine, of
                    // any carrier kind, rests where it lies
                    // (`crate::rest_locus`), on its UNCHANGED carrier
                    // (only the description moves): the pair of two
                    // distinct surfaces, or an image in the chart of the
                    // one surface both faces lie on.
                    let c = existing.as_ref().ok_or_else(corrupt)?;
                    let rest = crate::rest_locus(
                        body,
                        [(s1, Some(sides.plus.face)), (s2, Some(sides.minus.face))],
                        c.carrier(),
                        c.params(),
                        band,
                    );
                    let failure = match rest {
                        geom_brep::RestLocus::Chart(_) => EdgeDescribeFailure::Chart,
                        geom_brep::RestLocus::Pair(_) => EdgeDescribeFailure::Pair,
                    };
                    let description = c.restated_at(rest).description;
                    body.restate_edge(
                        crate::policy_lane::KernelSite::SeamAtRest,
                        edge,
                        description,
                        tol,
                    )
                    .map_err(|e| of_attachment(edge, failure, c.carrier().kind(), e))?;
                }
            }
        }
    }
    Ok(())
}

/// **A seam edge of the result, as its re-description reads it**: the
/// dihedral of its two surfaces at `witness` over `reach`, its faces' extent, whose arm
/// rung is the seam's own lever and whose reading is the seam's wedge.
#[cfg(test)]
pub(super) fn seam_class<T: Decide + crate::props::AtRestPolicy>(
    surf1: &geom::Surface<T>,
    surf2: &geom::Surface<T>,
    witness: Point3<T>,
    reach: T,
    band: Band,
) -> Result<geom_brep::DihedralClass, BooleanError> {
    let faces = [surf1, surf2].map(geom_brep::SeamFace::Analytic);
    seam_reading(faces, witness, reach, band).map_err(|refusal| match refusal {
        geom_brep::SeamEscalation::Lever(e) => {
            let (reading, diag) = DihedralReading::of_lever(e);
            seam_refusal(reading, diag)
        }
        geom_brep::SeamEscalation::Crease(_) | geom_brep::SeamEscalation::SplineReadAsAnalytic => {
            unreachable!("seam_class reads analytic pairs: {surf1:?} {surf2:?}")
        }
    })
}

/// `seam_class`'s reading, unworded: the dihedral of the seam's two
/// faces, each read by its kind ([`geom_brep::classify_seam`]).
fn seam_reading<T: Decide + crate::props::AtRestPolicy>(
    faces: [geom_brep::SeamFace<'_, T>; 2],
    witness: Point3<T>,
    reach: T,
    band: Band,
) -> Result<geom_brep::DihedralClass, geom_brep::SeamEscalation> {
    geom_brep::classify_seam(
        faces,
        witness,
        reach,
        |chart, p, reach| T::spline_chart_arm(chart, p, reach).ok(),
        band,
    )
}

/// **A face of a seam edge, as its dihedral reads it** at the edge's
/// witness ([`geom_brep::SeamFace`]): an analytic face at the witness;
/// a spline face at the witness's parameters on its own chart, read off
/// an image of the edge in that chart ([`crate::pcurves::pcurve_of`]) at
/// the edge's own parameter of the witness, the middle of its interval.
/// The image is the stored one where the half-edge has it, and otherwise
/// the one the chart's description derives, an iso row or arc rim of the
/// chart. Either is the edge on the face's own sheet, where a projection
/// foot of the witness could land on another sheet of the chart; a
/// stored `Pcurve::Fitted` image was itself fitted through the chart's
/// feet of a trace, placed on the face's sheet when it was minted.
///
/// A null edge has no carrier and no image to read a spline face at, and
/// is refused unplaced. Its witness could be placed off its neighbour in
/// the face's loop, but no class would follow: its zero-length chord gives
/// the wedge no reach, so its dihedral escalates at the arm whatever the
/// faces read.
///
/// # Errors
///
/// [`DescribeRefusal::Failed`] with [`EdgeDescribeFailure::Unplaced`]:
/// a spline face with no image of the edge in its chart to read the
/// witness's parameters off, or a null edge.
fn seam_face<'s, T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    edge: crate::entity::EdgeKey,
    side: crate::readback::EdgeSide,
    surface: &'s geom::Surface<T>,
    curve: Option<&geom_brep::EdgeCurve<T>>,
    band: Band,
) -> Result<geom_brep::SeamFace<'s, T>, DescribeRefusal> {
    let Some(chart) = surface.spline_chart() else {
        return Ok(geom_brep::SeamFace::Analytic(surface));
    };
    let unplaced = || DescribeRefusal::failed(edge, EdgeDescribeFailure::Unplaced);
    let (t0, t1) = curve.ok_or_else(unplaced)?.params();
    let image = crate::pcurves::pcurve_of(body, side.half_edge, band).map_err(|_| unplaced())?;
    let uv = image.eval(geom::mid_param(t0, t1));
    Ok(geom_brep::SeamFace::Spline {
        chart,
        uv: (uv.x, uv.y),
    })
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
/// - a station that reads the seam a corner ⇒ [`SeamRefusal::Mixed`]
///   with the two stations that disagree, which the describe pass words
///   as [`BooleanError::MixedSeam`] naming its edge: the seam is smooth
///   at its witness and a corner elsewhere, so the dihedral passes
///   through the band at a point no station read, and the seam is
///   certifiable as neither (D2);
/// - every station a corner ⇒ [`SeamRefusal::Mixed`] with no stations:
///   the corner refutes the smooth reading at the witness, which is not
///   a station on every carrier (a line's is its vertices' midpoint), as
///   the smooth-premise doors refuse it;
/// - in band at a station ⇒ [`SeamRefusal::Undecided`], worded as the
///   typed escalation of the reading that raised it
///   ([`seam_refusal`]): the seam's first-order arm or wedge, by rung,
///   as [`seam_class`] ends it, or its second-order bend
///   ([`BooleanDecision::SeamJet`]). Certifiable as neither, so never
///   folded into either description (D4 ¶3).
///
/// Edge-free: the describe pass names the edge where it words a refusal.
#[cfg(test)]
pub(super) fn seam_must_carry<T: Decide>(
    surf1: &geom::Surface<T>,
    surf2: &geom::Surface<T>,
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    reach: T,
    band: Band,
) -> Result<SeamReading, SeamRefusal> {
    must_carry_reading(surf1, surf2, carrier, t0, t1, reach, band)
}

/// Why a seam smooth at its witness has no description: a station
/// undecided, with the reading that raised it and its diagnostics, or
/// one that reads a corner (`seam_must_carry`).
#[derive(Clone, Copy, Debug)]
pub(super) enum SeamRefusal {
    Undecided(DihedralReading, Indeterminate),
    Mixed(Option<geom_brep::MixedStations>),
}

/// What a seam smooth at its witness stores (`seam_must_carry`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SeamReading {
    /// The intrinsic tangency: the surfaces determine the locus.
    Intrinsic,
    /// The conventional pair: the surfaces under-determine it.
    Conventional,
}

/// `seam_must_carry`'s verdict, unworded.
fn must_carry_reading<T: Decide>(
    surf1: &geom::Surface<T>,
    surf2: &geom::Surface<T>,
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    reach: T,
    band: Band,
) -> Result<SeamReading, SeamRefusal> {
    use geom_brep::MustCarryVerdict;
    match geom_brep::must_carry_over_edge(surf1, surf2, carrier, t0, t1, reach, band) {
        MustCarryVerdict::JetDeterminate => Ok(SeamReading::Intrinsic),
        MustCarryVerdict::UnderDetermined => Ok(SeamReading::Conventional),
        MustCarryVerdict::Transverse => Err(SeamRefusal::Mixed(None)),
        MustCarryVerdict::Mixed(stations) => Err(SeamRefusal::Mixed(Some(stations))),
        MustCarryVerdict::InBand(escalation) => {
            let (reading, diag) = DihedralReading::of_must_carry(escalation);
            Err(SeamRefusal::Undecided(reading, diag))
        }
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
pub(super) struct Descendants {
    /// The vertex fusions, in mint order, result keys: the zips'
    /// ([`Self::absorb_fusions`]).
    vertices: Fusions,
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
    /// ([`operand_fate`], [`place_found`]).
    fragments: [crate::lineage::FaceDivisions; 2],
    /// Each operand's faces the join killed, `(killed face, face
    /// holding its region)` in its clone keys: a division's fragment the
    /// join merged back (`join::Connected::absorbed`). Checked where each
    /// row is added, so it cannot cycle.
    absorbed: [crate::lineage::Substitutions<FaceKey>; 2],
    /// Every other substitution, dead cell → the cell that holds its
    /// interior now, result keys: a zipped ring edge → its seam edge, a
    /// cell the merge swallowed → the kept face, a joined vertex or
    /// edge → the joined edge. Checked where each row is added, so it
    /// cannot cycle.
    cells: crate::lineage::Substitutions,
}

impl Default for Descendants {
    // Each lineage list names itself where it is made (`LineageRows`).
    fn default() -> Self {
        Self {
            vertices: Fusions::new(crate::lineage::LineageRows::Fusions),
            fused: Default::default(),
            copies: Default::default(),
            along: Default::default(),
            fragments: [(); 2].map(|()| {
                crate::lineage::FaceDivisions::new(crate::lineage::LineageRows::FaceDivisions)
            }),
            absorbed: [(); 2].map(|()| {
                crate::lineage::Substitutions::new(crate::lineage::LineageRows::Substitutions)
            }),
            cells: crate::lineage::Substitutions::new(crate::lineage::LineageRows::Substitutions),
        }
    }
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
    pub(super) fn with_fragments(self, fragments: [crate::lineage::FaceDivisions; 2]) -> Self {
        Self { fragments, ..self }
    }

    /// The map that also reads the faces the join killed
    /// ([`Self::absorbed`]).
    pub(super) fn with_absorbed(
        self,
        absorbed: [crate::lineage::Substitutions<FaceKey>; 2],
    ) -> Self {
        Self { absorbed, ..self }
    }

    /// The clone face holding `f`'s region: `f`, or the face the join
    /// merged it into, followed while that one was merged too.
    fn holding(&self, side: Operand, f: FaceKey) -> FaceKey {
        self.absorbed[usize::from(side == Operand::B)].survivor(f)
    }

    /// `v` and every vertex null edges join it to, transitively, in
    /// `side`'s clone keys. Not a [`crate::lineage::Descent`]: a null
    /// edge's two ends are a pair, not a row from one to the other, so
    /// the walk follows each row both ways and stops at a vertex it has
    /// met (the null-copy closure's other spellings are
    /// `emit/crossing-sense-carries-five-copies-of-the-null-copy-closure`).
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
                self.substitute((Cell::Edge(dead), Cell::Edge(kept)))?;
            }
        }
        Ok(())
    }

    /// Vertex fusions `(dead, kept)`, result keys: a zip's.
    ///
    /// # Errors
    ///
    /// [`BooleanError::LineageRow`] where `merges` names a key the
    /// fusions before it killed ([`Fusions::extend`]).
    fn absorb_fusions(&mut self, merges: &Fusions) -> Result<(), BooleanError> {
        self.vertices.extend(merges)?;
        for &(dead, kept) in merges {
            self.fused.insert(dead);
            self.fused.insert(kept);
        }
        Ok(())
    }

    /// The merge's lineage, each cell a group killed onto its kept face
    /// ([`MergedGroup::substitutions`](crate::merge_faces::MergedGroup::substitutions)).
    ///
    /// # Errors
    ///
    /// As [`Self::substitute`], at the first row that refuses.
    pub(super) fn absorb_merge(
        &mut self,
        merged: &crate::merge_faces::MergeCoplanarOutcome,
    ) -> Result<(), BooleanError> {
        // The boolean merges unjoined (`finish_output`): its joins are
        // written by its own join stage, after the re-description.
        debug_assert!(
            merged.joins.is_empty(),
            "absorb_merge: the boolean's merge is the unjoined one, so it reports no join"
        );
        merged
            .groups
            .iter()
            .flat_map(crate::merge_faces::MergedGroup::substitutions)
            .try_for_each(|row| self.substitute(row))
    }

    /// Records that `by` holds `dead`'s interior now, as `(dead, by)`.
    ///
    /// # Errors
    ///
    /// [`BooleanError::LineageRow`] of the list's refusal: a row that maps a cell to
    /// itself, or names a cell an earlier row retired.
    pub(super) fn substitute(&mut self, row: (Cell, Cell)) -> Result<(), BooleanError> {
        self.cells.push(row).map_err(Into::into)
    }

    /// Operand `side`'s cell, read through its key `view`, the zips'
    /// fusions (a vertex), then the substitution rows (an absorbed face
    /// among them) until live: `None` where
    /// the chain ends at a dead cell with no row (the cell left the
    /// result). An edge whose chain ends so is read on through the
    /// chords the join minted along it ([`Self::along`]), each by the
    /// same chain: the one that is live holds it.
    ///
    /// # Errors
    ///
    /// [`BooleanError::JoinDesync`] where two chords along one edge both
    /// reach a live cell, and they are not one: two edges cannot both
    /// hold one edge's interior. The rows themselves cannot cycle: each
    /// was checked where it was added ([`crate::lineage::Substitutions`]).
    pub(super) fn live<T: Real>(
        &self,
        body: &Body<T>,
        side: Operand,
        view: &KeyView<'_>,
        cell: Cell,
    ) -> Result<Option<Cell>, BooleanError> {
        if let Some(c) = self.chained(body, view, cell) {
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
            match (held, self.chained(body, view, Cell::Edge(chord))) {
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

    /// [`Self::live`] without the chord rows.
    fn chained<T: Real>(&self, body: &Body<T>, view: &KeyView<'_>, cell: Cell) -> Option<Cell> {
        let start = match cell {
            Cell::Vertex(v) => view
                .vertex(v)
                .map(|k| Cell::Vertex(self.vertices.survivor(k))),
            Cell::Edge(e) => view.edge(e).map(Cell::Edge),
            Cell::Face(f) => view.face(f).map(Cell::Face),
        };
        self.cells.chase(start?, |c| holds(body, c)).ok()
    }
}

/// What [`piece_holds`] read of one piece.
#[derive(Debug)]
pub(super) enum PieceRead {
    /// The point is strictly inside the piece.
    In,
    /// The point is off the piece's carrier, outside its region, or on
    /// its boundary, where it meets an edge or vertex that a record of
    /// its own names.
    NotIn,
    /// The read failed: the containment door's refusal, or `None` where
    /// a curved face's region gives no verdict.
    Unread(Option<ContainError>),
}

/// Whether `face` holds `q` strictly inside it, placed as the census
/// confirms a vertex-on-face record ([`super::contain::face_placement`]).
///
/// # Errors
///
/// [`BooleanError::Escalated`] (`SelfCheck(CarriedLineage)`) on a margin
/// in band, the on-plane one or the door's.
pub(super) fn piece_holds<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    q: Point3<T>,
    band: Band,
) -> Result<PieceRead, BooleanError> {
    piece_reading(body, face, q, band, DoorInBand::SelfCheck)
}

/// How [`piece_reading`] types an in-band margin of the containment door.
#[derive(Clone, Copy)]
pub(super) enum DoorInBand {
    /// The carry's check: the door re-reads a cell a record already
    /// placed, so a margin in band is the kernel disagreeing with itself.
    SelfCheck,
    /// A first reading of a point against the face: the door's own
    /// decision, whose margin is the operands' geometry.
    Containment,
}

/// [`piece_holds`], typing an in-band margin of the containment door as
/// `door` says. The on-plane margin re-reads a plane the census already
/// put the point on, so it escalates as the carry's check either way.
///
/// # Errors
///
/// [`BooleanError::Escalated`] on a margin in band: `SelfCheck(CarriedLineage)`
/// for the on-plane one, `door`'s for the door's.
pub(super) fn piece_reading<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    q: Point3<T>,
    band: Band,
    door: DoorInBand,
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
        Err(PlacementError::OnPlane(diag)) => Err(in_band(diag)),
        Err(PlacementError::Door(ContainError::Escalated {
            decision,
            escalation,
            diag,
        })) => Err(match door {
            DoorInBand::SelfCheck => in_band(diag),
            DoorInBand::Containment => BooleanError::Escalated {
                decision: BooleanDecision::Containment {
                    decision,
                    escalation,
                },
                diag,
            },
        }),
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

/// **What a result's records are carried from**, read off the reduction
/// before its finish: each operand's carried records as the cell pairs
/// of its input, citing the operand's record each is
/// ([`Backing::Carried`]); and each tie two pierces of one ring make, a
/// vertex pair the ring joined by a value decision of this op's, on two
/// point keys (a pair on one point key is structure and is not listed).
/// Neither is placed here: the one carry places both on the result
/// ([`carry`]).
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] where a ring tie's vertex does not
/// resolve in its clone.
pub(super) fn ledger<T: Decide>(
    red: &BooleanReduction<T>,
    decls: &Glue<T>,
    scaffold: [&BTreeSet<FaceKey>; 2],
) -> Result<Ledger, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut carried = [
        carried_rows(&decls.records_a, Operand::A),
        carried_rows(&decls.records_b, Operand::B),
    ];
    let mut pending = red.pending.clone();
    let mut pierced = Vec::new();
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
        let ring = red
            .ring_of(tie.operand.other(), tie.a)
            .ok_or_else(|| desync("a ring tie's pierce is served by no ring"))?;
        let k = decide_one(
            &mut pending,
            (tie.operand, Cell::Vertex(tie.a)),
            (tie.operand, Cell::Vertex(tie.b)),
            tie.margin,
        );
        pierced.push((
            k,
            (tie.operand.other(), Cell::Face(red.pierce_rings[ring].face)),
        ));
        carried[usize::from(tie.operand == Operand::B)].push((
            [Cell::Vertex(tie.a), Cell::Vertex(tie.b)],
            Cites::decided(k),
        ));
    }
    Ok(Ledger {
        discovered: red.contacts.clone(),
        carried,
        pending,
        pierced,
        mints: red.mints.clone(),
        recorded: red.coincidences.clone(),
        glued: red.glued.clone(),
        incidence: [
            Incidence::of(&red.a, scaffold[0]),
            Incidence::of(&red.b, scaffold[1]),
        ],
        copies: red.copies.clone(),
    })
}

/// One operand's carried records as input cell pairs, each citing the
/// operand's record it is.
fn carried_rows(records: &OperandRecords, side: Operand) -> Vec<([Cell; 2], Cites)> {
    let OperandRecords {
        vv,
        vf,
        ve,
        ee,
        ef,
        ff,
    } = records;
    let input = u32::from(side == Operand::B);
    let cite = |record| Cites::one(Backing::Carried { input, record });
    vv.iter()
        .map(|c| {
            (
                [Cell::Vertex(c.contact.a), Cell::Vertex(c.contact.b)],
                cite(c.record),
            )
        })
        .chain(vf.iter().map(|c| {
            (
                [Cell::Vertex(c.contact.vertex), Cell::Face(c.contact.face)],
                cite(c.record),
            )
        }))
        .chain(ve.iter().map(|c| {
            (
                [Cell::Vertex(c.contact.vertex), Cell::Edge(c.contact.edge)],
                cite(c.record),
            )
        }))
        .chain(ee.iter().map(|c| {
            (
                [Cell::Edge(c.contact.a), Cell::Edge(c.contact.b)],
                cite(c.record),
            )
        }))
        .chain(ef.iter().map(|c| {
            (
                [Cell::Edge(c.contact.edge), Cell::Face(c.contact.face)],
                cite(c.record),
            )
        }))
        .chain(ff.iter().map(|c| {
            (
                [Cell::Face(c.contact.a), Cell::Face(c.contact.b)],
                cite(c.record),
            )
        }))
        .collect()
}

/// Records a decision `first` and `second` are one, by `margin`, as a
/// pending row ([`super::reduce::pend`]), and returns its index.
fn decide_one(
    pending: &mut Vec<PendingRow>,
    first: End,
    second: End,
    margin: geom_core::MarginDiag,
) -> u32 {
    super::reduce::index(super::reduce::pend(
        pending,
        PendingRow::on(first, second, Some(margin)),
    ))
}

/// **Everything a result's records are carried from**: the reduction's
/// own records, each operand's carried records and ring ties
/// ([`ledger`]), every decision a record may cite, in decision order,
/// and what the reduction made of each operand's cells.
pub(super) struct Ledger {
    /// The reduction's working set, citing rows of `pending`.
    discovered: ReductionContacts,
    /// Each operand's carried records and ring ties, input cell pairs.
    carried: [Vec<([Cell; 2], Cites)>; 2],
    /// The reduction's decisions, then the ring ties'.
    pending: Vec<PendingRow>,
    /// Each ring tie's decision in `pending`, with the face its two
    /// pierces pierced ([`super::PierceRingRecord::face`]).
    pierced: Vec<(u32, End)>,
    /// Where the reduction minted each vertex, which reads it back to the
    /// input cell it lies in ([`super::Mints`]).
    mints: super::Mints,
    /// The coincidences the op recorded before its records were carried.
    recorded: Vec<crate::Coincidence>,
    /// Each pending row the op recorded already, with its index in
    /// `recorded` ([`super::BooleanReduction::glued`]).
    glued: Vec<(u32, u32)>,
    /// Each operand clone's faces around each vertex and edge, before
    /// the finish discards any.
    incidence: [Incidence; 2],
    /// Each operand's null-edge copies.
    copies: [crate::lineage::NullEdgeCopies; 2],
}

/// **The faces around each cell of an operand's clone**, read before
/// the finish, the join's null faces (`scaffold`, which the zip
/// consumes) left out: a vertex or edge the finish deleted is one it
/// removed where every face around it was discarded.
#[derive(Default)]
pub(super) struct Incidence {
    vertex: BTreeMap<VertexKey, Vec<FaceKey>>,
    edge: BTreeMap<EdgeKey, Vec<FaceKey>>,
    /// The join's null faces: a division's fragment that became one
    /// holds none of the divided face's region.
    scaffold: BTreeSet<FaceKey>,
}

impl Incidence {
    /// The cells the division of one face into `fragments` minted in
    /// its interior: each edge between two of them (a chord), and each
    /// vertex no face but them bounds (where chords meet inside it).
    fn chords<'a>(&'a self, fragments: &'a [FaceKey]) -> impl Iterator<Item = Cell> + 'a {
        let inside = move |fs: &[FaceKey]| {
            fs.len() >= 2
                && fs.iter().all(|f| fragments.contains(f))
                && fs.iter().any(|f| f != &fs[0])
        };
        self.edge
            .iter()
            .filter(move |(_, fs)| inside(fs))
            .map(|(&e, _)| Cell::Edge(e))
            .chain(
                self.vertex
                    .iter()
                    .filter(move |(_, fs)| inside(fs))
                    .map(|(&v, _)| Cell::Vertex(v)),
            )
    }

    fn of<T: Real>(clone: &Body<T>, scaffold: &BTreeSet<FaceKey>) -> Self {
        let real = |fs: Vec<FaceKey>| -> Vec<FaceKey> {
            fs.into_iter().filter(|f| !scaffold.contains(f)).collect()
        };
        let edge = clone
            .edges()
            .filter_map(|(e, _)| {
                let (f0, f1) = crate::readback::edge_sides(clone, e).ok()?.faces();
                Some((e, real(vec![f0, f1])))
            })
            .collect();
        Self {
            vertex: faces_by_vertex(clone)
                .into_iter()
                .map(|(v, fs)| (v, real(fs)))
                .collect(),
            edge,
            scaffold: scaffold.clone(),
        }
    }
}

/// The vertex `v` of operand `side`'s clone is a copy of: where a null
/// edge minted `v` at another vertex, that vertex, read back through
/// every null edge that hangs at a copy; `v` itself where none did. A
/// decision's cells name the op's input, which holds no copy. Total:
/// the copies were checked as each was recorded, so they cannot cycle
/// ([`crate::lineage::NullEdgeCopies`]).
fn uncopied(
    copies: &[crate::lineage::NullEdgeCopies; 2],
    side: Operand,
    v: VertexKey,
) -> VertexKey {
    copies[usize::from(side == Operand::B)].root(v)
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
    ef: Vec<Cited<EfContact>>,
    ff: Vec<Cited<FfContact>>,
}

impl Rows {
    /// Adds a landing in clone keys, filed by its two cells' kinds, a
    /// pair of one kind in key order, or where its pair is already here
    /// adds its citations.
    fn push(&mut self, x: Cell, y: Cell, cites: Cites) {
        fn add<R: PartialEq>(list: &mut Vec<Cited<R>>, record: R, cites: Cites) {
            match list.iter_mut().find(|r| r.record == record) {
                Some(r) => r.cites = r.cites.union(&cites),
                None => list.push(Cited::new(record, cites)),
            }
        }
        fn ordered<K: Ord>(a: K, b: K) -> (K, K) {
            if a <= b { (a, b) } else { (b, a) }
        }
        match (x, y) {
            (Cell::Vertex(a), Cell::Vertex(b)) => {
                let (a, b) = ordered(a, b);
                add(&mut self.vv, VvContact { a, b }, cites);
            }
            (Cell::Vertex(vertex), Cell::Edge(edge)) | (Cell::Edge(edge), Cell::Vertex(vertex)) => {
                add(&mut self.ve, VeContact { vertex, edge }, cites);
            }
            (Cell::Vertex(vertex), Cell::Face(face)) | (Cell::Face(face), Cell::Vertex(vertex)) => {
                add(&mut self.vf, VfContact { vertex, face }, cites);
            }
            (Cell::Edge(a), Cell::Edge(b)) => {
                let (a, b) = ordered(a, b);
                add(&mut self.ee, EeContact { a, b }, cites);
            }
            (Cell::Edge(edge), Cell::Face(face)) | (Cell::Face(face), Cell::Edge(edge)) => {
                add(&mut self.ef, EfContact { edge, face }, cites);
            }
            (Cell::Face(a), Cell::Face(b)) => {
                let (a, b) = ordered(a, b);
                add(&mut self.ff, FfContact { a, b }, cites);
            }
        }
    }

    #[cfg(test)]
    pub(super) fn of(carried: &OperandRecords, side: Operand) -> Self {
        let OperandRecords {
            vv,
            vf,
            ve,
            ee,
            ef,
            ff,
        } = carried;
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
            ef: ef
                .iter()
                .map(|c| Cited::new(c.contact, cite(c.record)))
                .collect(),
            ff: ff
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

/// **The substitution door**: every contact record an op holds, in
/// result keys. Each operand's carried records go through the one carry
/// ([`carried`]): placed on the result by what the op made of each input
/// cell, by lineage where the op kept both cells and by the census's own
/// record confirm where it divided one (D1). The reduction's own
/// records are rewritten through the operand views and the descendant
/// map: a record whose cell an op replaced now names the replacement (a
/// fused vertex its survivor, an absorbed face its kept face, a zipped
/// ring edge its seam edge, a cell the merge swallowed the face it lies
/// in, a joined vertex or edge the edge the join made, an edge the op
/// dropped the chord the join minted along it, [`Descendants::live`]);
/// a face the join divided after the record was written, the pairs of
/// its pieces and chords whose interiors meet ([`place_found`]). Such a
/// record leaves only where its cell left the result with no such row,
/// or where its two cells became one or incident, which is structure
/// ([`record`]).
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
/// that put the vertex in its group.
///
/// **Every decision the op took with a margin is a row**, cited or
/// not (D10 `[coincidence.door]`): emitted in decision order after the
/// op's own coincidences, the records renumbered onto them, each
/// spelled in its inputs' cells ([`spell`]). A decision
/// with no margin (a transverse pierce's landing) placed topology and
/// is not a coincidence (D1); a surviving record whose decisions all
/// decided no margin is a touch without a decision, which the op
/// refuses rather than ship.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] where two chords along one edge both
/// hold its interior ([`Descendants::live`]);
/// [`BooleanError::ClassificationInvariant`] on a surviving record no
/// decision backs.
pub(super) fn carry<T: Decide + AtRestPolicy>(
    body: &Body<T>,
    ledger: &Ledger,
    views: [&KeyView<'_>; 2],
    desc: &Descendants,
    discarded: [&BTreeSet<FaceKey>; 2],
    band: Band,
) -> Result<Carried, BooleanError> {
    let mut pending = ledger.pending.clone();
    let [a, b] = carried(body, ledger, views, desc, discarded, &mut pending, band)?;
    let mut records = carry_rows(
        body,
        &tagged(&ledger.discovered),
        [&a, &b],
        views,
        (desc, &ledger.incidence, discarded),
        false,
        band,
    )?;
    record_spans(body, &mut records, band)?;
    cite_rows(records, ledger, &pending)
}

/// **The one carry, for the boolean** (D1): each operand's carried
/// records ([`Ledger::carried`]) placed on the result through what the
/// op made of each input cell ([`operand_fate`]), by the one carry's
/// placement ([`super::records::landings`]), each landing restated
/// in its operand's clone keys so the vertex groups read it as they
/// read the reduction's records ([`carry_rows`]). Two cells the op
/// minted on a record's two cells that meet at a point are decided at
/// this door: the tool has many faces, so a meeting one face did not
/// make is a value coincidence (D10), and the landing cites the
/// decision beside the record.
///
/// # Errors
///
/// [`BooleanError::CarryRefused`] for a record the carry refuses;
/// [`BooleanError::JoinDesync`] for a cell that left the result with no
/// row stating its removal, or a minted meeting this door has no
/// decision for; [`BooleanError::Escalated`] (`VertexOnVertex`) where a
/// minted meeting's margin is in band.
fn carried<T: Decide + AtRestPolicy>(
    body: &Body<T>,
    ledger: &Ledger,
    views: [&KeyView<'_>; 2],
    desc: &Descendants,
    discarded: [&BTreeSet<FaceKey>; 2],
    pending: &mut Vec<PendingRow>,
    band: Band,
) -> Result<[Rows; 2], BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut out = [Rows::default(), Rows::default()];
    let mut confirms = super::records::Confirms::new(1);
    for (s, side) in [Operand::A, Operand::B].into_iter().enumerate() {
        let rows = &ledger.carried[s];
        if rows.is_empty() {
            continue;
        }
        let mut lineage = crate::Lineage::new().glued();
        let mut back: Vec<Back> = Vec::new();
        let cells: BTreeSet<Cell> = rows.iter().flat_map(|(c, _)| *c).collect();
        for x in cells {
            let fate = operand_fate(
                body,
                ledger,
                (side, views[s]),
                desc,
                discarded[s],
                x,
                &mut back,
            )?;
            lineage = lineage
                .with(x, fate)
                .unwrap_or_else(|e| unreachable!("each cell is stated once: {e:?}"));
        }
        let pairs: Vec<[Cell; 2]> = rows.iter().map(|&(c, _)| c).collect();
        let landed = super::records::landings(
            &pairs,
            &lineage,
            &[body],
            &mut confirms,
            band,
            T::region_lane(),
        )
        .map_err(|refusal| {
            // A carried record cites the operand's record it is; a ring
            // tie, listed after them, cites the reduction's decision.
            let k = refusal.record() as usize;
            let ties = rows
                .iter()
                .take(k)
                .filter(|(_, cites)| operand_record(cites).is_none())
                .count();
            match rows.get(k).and_then(|(_, cites)| operand_record(cites)) {
                Some(record) => BooleanError::CarryRefused {
                    row: super::RefusedRow::Carried(side),
                    refusal: refusal.renumbered(record),
                },
                None => BooleanError::CarryRefused {
                    row: super::RefusedRow::RingTie(side),
                    refusal: refusal.renumbered(super::reduce::index(ties)),
                },
            }
        })?;
        for l in landed {
            let (cells, cites) = &rows[l.row];
            let clone = |i: usize| {
                back.iter()
                    .find(|b| {
                        b.input == cells[i]
                            && super::records::conventional_edge(body, b.result) == l.at[i]
                    })
                    .map(|b| b.clone)
                    .ok_or_else(|| desync("a carried record landed on a cell no image names"))
            };
            let (x, y) = (clone(0)?, clone(1)?);
            let mut cites = cites.clone();
            if l.minted {
                let (Cell::Vertex(u), Cell::Vertex(w), Cell::Vertex(ru), Cell::Vertex(rw)) =
                    (x, y, l.at[0], l.at[1])
                else {
                    return Err(desync(
                        "two cells the boolean minted on a carried record meet at a point \
                         that is no vertex pair",
                    ));
                };
                let (meet, margin) =
                    vertices_meet(body, [ru, rw], "bool_carried_minted_meeting", band)?;
                if !meet {
                    return Err(desync(
                        "the census confirms two minted vertices meeting that the door reads apart",
                    ));
                }
                let fresh = decide_one(
                    pending,
                    (side, Cell::Vertex(uncopied(&ledger.copies, side, u))),
                    (side, Cell::Vertex(uncopied(&ledger.copies, side, w))),
                    margin,
                );
                cites = cites.union(&Cites::decided(fresh));
            }
            out[s].push(x, y, cites);
        }
    }
    Ok(out)
}

/// The operand's record a carried row is: the one it cites.
fn operand_record(cites: &Cites) -> Option<u32> {
    match cites.iter().collect::<Vec<_>>()[..] {
        [Backing::Carried { record, .. }] => Some(record),
        _ => None,
    }
}

/// One image of a carried record's input cell: the result cell, and the
/// operand clone's cell it is the live image of.
struct Back {
    input: Cell,
    result: Cell,
    clone: Cell,
}

/// **What the boolean made of operand `side`'s input cell `x`** (D1), in
/// result keys, each image's clone cell logged in `back`. The cell's
/// clone cells are the vertex and its null-edge copies
/// ([`Descendants::copies_of`]); an edge's pieces after the reduction's
/// splits, with the vertices the splits minted between them; a face's
/// fragments after the join's divisions ([`Descendants::fragments`]),
/// with the chords between them. Each is read through the key view and
/// the descendant map ([`Descendants::live`]). One the result does not
/// hold is removed only where the op discarded it: a face the finish
/// discarded, or an edge or vertex every face around which it
/// discarded (`discarded`, [`Incidence`]); the removal makes a division's
/// remainder removed.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] for a clone cell the result does not hold
/// and the op did not discard; as [`Descendants::live`].
fn operand_fate<T: Real>(
    body: &Body<T>,
    ledger: &Ledger,
    (side, view): (Operand, &KeyView<'_>),
    desc: &Descendants,
    discarded: &BTreeSet<FaceKey>,
    x: Cell,
    back: &mut Vec<Back>,
) -> Result<crate::Fate, BooleanError> {
    let incidence = &ledger.incidence[usize::from(side == Operand::B)];
    let copies = |v: VertexKey| desc.copies_of(side, v).into_iter().map(Cell::Vertex);
    let (pieces, minted, divided): (Vec<Cell>, Vec<Cell>, bool) = match x {
        Cell::Vertex(v) => (copies(v).collect(), Vec::new(), false),
        Cell::Edge(e) => {
            let (keys, ms) = ledger.mints.pieces(side, e);
            let divided = keys.len() > 1;
            (
                keys.into_iter().map(Cell::Edge).collect(),
                ms.into_iter().flat_map(copies).collect(),
                divided,
            )
        }
        Cell::Face(f) => face_pieces(desc, incidence, side, f),
    };
    let removed = |c: Cell| stated_removed(incidence, discarded, c);
    let mut gone = false;
    let pieces = imaged(
        body,
        desc,
        (side, view),
        &removed,
        x,
        pieces,
        &mut gone,
        back,
    )?;
    let minted = imaged(
        body,
        desc,
        (side, view),
        &removed,
        x,
        minted,
        &mut gone,
        back,
    )?;
    Ok(fate_of(pieces, minted, divided, gone))
}

/// A cell's fate from its live pieces and minted cells: removed where
/// none is left, kept where it is neither divided nor lost any part, and
/// otherwise divided, its remainder removed where it lost part.
fn fate_of(pieces: Vec<Cell>, minted: Vec<Cell>, divided: bool, gone: bool) -> crate::Fate {
    use crate::lineage::{Division, Remainder};
    match pieces.split_first() {
        None if minted.is_empty() => crate::Fate::Removed,
        Some((&at, copies)) if !divided && !gone => crate::Fate::Kept {
            at,
            copies: copies.to_vec(),
        },
        _ => crate::Fate::Divided(Division {
            pieces,
            minted,
            remainder: if gone {
                Remainder::Removed
            } else {
                Remainder::Exhaustive
            },
        }),
    }
}

/// Face `f`'s pieces in `side`'s clone keys: the fragments the join's
/// divisions left of it, each read through the faces the join merged
/// back ([`Descendants::holding`]), the join's null faces left out, with
/// the chords between them ([`Incidence::chords`]), and whether it is
/// divided.
fn face_pieces(
    desc: &Descendants,
    incidence: &Incidence,
    side: Operand,
    f: FaceKey,
) -> (Vec<Cell>, Vec<Cell>, bool) {
    let mut fragments: Vec<FaceKey> = Vec::new();
    for g in crate::lineage::Descent::of(f, desc.fragments[usize::from(side == Operand::B)].rows())
        .cells()
    {
        let g = desc.holding(side, g);
        if !incidence.scaffold.contains(&g) && !fragments.contains(&g) {
            fragments.push(g);
        }
    }
    let chords = incidence.chords(&fragments).collect();
    let divided = fragments.len() > 1;
    (
        fragments.into_iter().map(Cell::Face).collect(),
        chords,
        divided,
    )
}

/// Whether the op stated the removal of `c`, a clone cell: a face it
/// discarded, or an edge or vertex every real face around which it
/// discarded ([`Incidence`]).
fn stated_removed(incidence: &Incidence, discarded: &BTreeSet<FaceKey>, c: Cell) -> bool {
    let all_discarded = |fs: Option<&Vec<FaceKey>>| {
        fs.is_some_and(|fs| !fs.is_empty() && fs.iter().all(|f| discarded.contains(f)))
    };
    match c {
        Cell::Face(f) => discarded.contains(&f),
        Cell::Edge(e) => all_discarded(incidence.edge.get(&e)),
        Cell::Vertex(v) => all_discarded(incidence.vertex.get(&v)),
    }
}

/// The live images of `cells`, clone cells of input cell `x`, each
/// logged in `back`; one the result does not hold sets `gone` where its
/// removal is stated.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] for a cell the result does not hold
/// whose removal is not stated; as [`Descendants::live`].
#[allow(clippy::too_many_arguments)]
fn imaged<T: Real>(
    body: &Body<T>,
    desc: &Descendants,
    (side, view): (Operand, &KeyView<'_>),
    removed: &dyn Fn(Cell) -> bool,
    x: Cell,
    cells: Vec<Cell>,
    gone: &mut bool,
    back: &mut Vec<Back>,
) -> Result<Vec<Cell>, BooleanError> {
    let mut out: Vec<Cell> = Vec::new();
    for c in cells {
        match desc.live(body, side, view, c)? {
            Some(r) => {
                back.push(Back {
                    input: x,
                    result: r,
                    clone: c,
                });
                if !out.contains(&r) {
                    out.push(r);
                }
            }
            None if removed(c) => *gone = true,
            None => {
                return Err(BooleanError::JoinDesync {
                    what: "a carried record's cell left the result with no row stating its \
                           removal",
                });
            }
        }
    }
    Ok(out)
}

/// The result's vertices an operand's carried vertex-vertex records
/// name, read through [`operand_fate`] as [`carried`] reads them, before
/// the output stage's join: the vertices whose touch a record of the
/// result will name though the join runs an edge through it
/// (`edge_join::free_vertices`).
///
/// # Errors
///
/// As [`operand_fate`].
fn carried_vertices<T: Real>(
    body: &Body<T>,
    ledger: &Ledger,
    views: [&KeyView<'_>; 2],
    desc: &Descendants,
    discarded: [&BTreeSet<FaceKey>; 2],
) -> Result<BTreeSet<VertexKey>, BooleanError> {
    let mut back: Vec<Back> = Vec::new();
    for (s, side) in [Operand::A, Operand::B].into_iter().enumerate() {
        for &(pair, _) in &ledger.carried[s] {
            if let [x @ Cell::Vertex(_), y @ Cell::Vertex(_)] = pair {
                for c in [x, y] {
                    operand_fate(
                        body,
                        ledger,
                        (side, views[s]),
                        desc,
                        discarded[s],
                        c,
                        &mut back,
                    )?;
                }
            }
        }
    }
    Ok(back
        .iter()
        .filter_map(|b| match b.result {
            Cell::Vertex(v) => Some(v),
            _ => None,
        })
        .collect())
}

/// **The output stage's one key rule**: the vertices of every
/// vertex-vertex record of the result move onto one point key
/// ([`record_classes`]), on every path a boolean builds by. Run
/// after [`carry`], on the records the result ships.
///
/// # Errors
///
/// As [`record_classes`] and `zip::share_points`.
fn share_recorded_points<T: Decide>(
    body: &mut Body<T>,
    contacts: &ContactRecords,
    coincidences: &[crate::Coincidence],
    band: Band,
) -> Result<(), BooleanError> {
    let classes = record_classes(body, contacts, coincidences, band)?;
    super::zip::share_points(body, &classes)
}

/// **Two vertices a record says touch sit on one point key.** A
/// boolean that decides from values that two vertices of its operands
/// coincide puts them on one point key in its result, whether they are
/// cones of one solid's pinch or touching corners of two solids in an
/// assembly. A vertex-vertex record of the result states that the two
/// are one point: the op decided it (the carry refuses a record no
/// decision backs, and every decision it emits decided its margin
/// Zero), or an operand carried it in, landed by lineage on vertices
/// the op kept or confirmed by the census where it divided one
/// (`records::landings`). Moving a vertex onto another's key moves
/// geometry on the record's word, so each pair on two keys has its gap
/// decided Zero at `band` first (D10): a carried pair apart is the
/// caller's stale record, refused, and never moves a vertex. Every key
/// the records reach, through their pairs and through the keys their
/// vertices share, then names one point, and the classes of two or more
/// keys are returned for `zip::share_points`.
/// The records themselves stay, citing their decisions, which is what
/// the `unproven-coincidence` lint reads.
///
/// # Errors
///
/// [`BooleanError::CarryRefused`] ([`super::CarryRefusal::Stale`]) for a
/// carried pair whose two points are apart, and
/// [`BooleanError::JoinDesync`] for a decided one;
/// [`BooleanError::Escalated`] (`VertexOnVertex`) where a gap is in
/// band; [`BooleanError::JoinDesync`] where a record's vertex does not
/// resolve in `body`.
fn record_classes<T: Decide>(
    body: &Body<T>,
    contacts: &crate::ContactRecords,
    coincidences: &[crate::Coincidence],
    band: Band,
) -> Result<Vec<super::zip::PointClass>, BooleanError> {
    let key = |v: VertexKey| {
        body.get_vertex(v)
            .map(|d| d.point)
            .ok_or(BooleanError::JoinDesync {
                what: "a vertex-vertex record names a vertex the result does not hold",
            })
    };
    let mut roots = super::zip::Roots::new();
    for row in contacts.rows() {
        let crate::ContactKind::VertexVertex(a, b) = row.kind() else {
            continue;
        };
        let (ka, kb) = (key(a)?, key(b)?);
        if ka != kb && !vertices_meet(body, [a, b], "bool_shared_record_points", band)?.0 {
            let carried = row.cites.iter().find_map(|b| match b {
                Backing::Carried { input, record } => Some((input, record)),
                Backing::Decided(_) => None,
            });
            return Err(match carried {
                Some((input, record)) => BooleanError::CarryRefused {
                    row: super::RefusedRow::Carried(if input == 0 {
                        Operand::A
                    } else {
                        Operand::B
                    }),
                    refusal: super::CarryRefusal::Stale { record },
                },
                None => BooleanError::JoinDesync {
                    what: "a decided vertex-vertex record names two points apart",
                },
            });
        }
        roots.union(ka, kb);
    }
    #[cfg_attr(not(feature = "sweep-testing"), allow(unused_mut))]
    let mut classes = roots.classes();
    // The margins each class's records cite, for the test witness
    // (`take_shared_points`).
    #[cfg(feature = "sweep-testing")]
    for row in contacts.rows() {
        let crate::ContactKind::VertexVertex(a, _) = row.kind() else {
            continue;
        };
        let Some(class) = classes.get_mut(&roots.find(key(a)?)) else {
            continue;
        };
        for cite in row.cites.iter() {
            if let crate::Backing::Decided(k) = cite
                && let Some(c) = coincidences.get(k as usize)
            {
                class.margins.push(c.margin);
            }
        }
    }
    #[cfg(not(feature = "sweep-testing"))]
    let _ = coincidences;
    Ok(classes.into_values().collect())
}

/// The records `carry_rows` left and every decision the op took with a
/// margin, each spelled in its inputs' cells ([`spell`]) and emitted
/// after the op's own coincidences in decision order, the records'
/// citations renumbered onto them ([`carry`]).
fn cite_rows(
    records: ContactRecords,
    ledger: &Ledger,
    pending: &[PendingRow],
) -> Result<Carried, BooleanError> {
    let decided = records.decided();
    if decided.last().is_some_and(|&k| k as usize >= pending.len()) {
        return Err(BooleanError::JoinDesync {
            what: "a contact record cites a decision the reduction did not make",
        });
    }
    // A row the op recorded already (a tangency the glue door decided)
    // is cited where it stands: one decision, one row.
    let mut coincidences = ledger.recorded.clone();
    let mut used = Vec::new();
    let mut at = Vec::new();
    for (k, row) in pending.iter().enumerate() {
        // A decision with no margin (a transverse pierce's landing) is
        // not a coincidence, and a record citing only such is refused
        // below.
        let Some(margin) = row.margin else { continue };
        let k = super::reduce::index(k);
        let mut faces = ledger.pierced.iter().filter(|&&(p, _)| p == k);
        let through = faces.next().map(|&(_, face)| face);
        debug_assert!(
            faces.all(|&(_, face)| Some(face) == through),
            "decision {k} is tied against two pierced faces: {:?}",
            ledger.pierced
        );
        let (cells, more, relation) = spell(row.cells, through.as_slice(), row.relation, |end| {
            input_read(end, &ledger.mints)
        })?;
        used.push(k);
        if let Some(&(_, c)) = ledger.glued.iter().find(|&&(p, _)| p == k) {
            let recorded = &coincidences[c as usize];
            if recorded.cells != cells || recorded.more != more || recorded.relation != relation {
                return Err(BooleanError::JoinDesync {
                    what: "a decision the op recorded already names other cells than its row",
                });
            }
            at.push(c);
            continue;
        }
        at.push(super::reduce::index(coincidences.len()));
        coincidences.push(crate::Coincidence {
            cells,
            more,
            relation,
            site: crate::DecisionSite::VertexFusion,
            margin,
            discharge: crate::Discharge::of(margin),
        });
    }
    let records = records
        .renumbered(|k| used.binary_search(&k).ok().map(|rank| at[rank]))
        .map_err(|_| BooleanError::ClassificationInvariant {
            what: "a contact record surviving into the result cites no decision",
        })?;
    Ok((records, coincidences))
}

/// What [`carry`] hands the result: its records and its rows.
type Carried = (ContactRecords, Vec<crate::Coincidence>);

/// **The one door a decision's cells pass to become a row's**: `read`
/// takes each cell to the cell of the op's input it lies in, and, for a
/// vertex the op minted on an edge, to the cell of the other input whose
/// crossing minted it ([`input_read`]). A cell read as one of higher
/// dimension (a vertex minted on an edge read as that edge, a vertex
/// minted inside a face read as that face, a chart cut read as the face
/// it runs in) is widened. The row keeps the relation decided where
/// widening cannot change what it says: no cell widened; or no minting
/// cell beyond the two read and, for a cell decided on another's
/// carrier, the second widened under a first that is a point, which
/// lies on whatever the second lies in, or two points decided one with
/// the first widened, spelled the other way round. Any other spelling
/// of a point decided on a cell states the point as the
/// [`crate::Relation::Meet`] of the cells read, of the cells whose
/// crossings minted them, and of `through` (the face a ring tie's two
/// pierces pierced, which it was decided against, or a meeting's cells
/// read again), each once and sorted.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] for a widened cell of any other
/// relation: only a point is decided on a cell; and for a meeting of
/// more cells than a row holds, which no reading makes: two cells read
/// with a minting cell each and one `through`, or two read with none
/// and a meeting's three.
fn spell(
    cells: [End; 2],
    through: &[End],
    relation: crate::Relation,
    read: impl Fn(End) -> Read,
) -> Result<Spelled, BooleanError> {
    let dim = |c: Cell| match c {
        Cell::Vertex(_) => 0,
        Cell::Edge(_) => 1,
        Cell::Face(_) => 2,
    };
    let row_cell = |(input, cell): End| crate::RowCell::Input { input, cell };
    let reads = cells.map(&read);
    let read_cells = reads.map(|(at, _)| at);
    let widened = [0, 1].map(|i| dim(read_cells[i].1) > dim(cells[i].1));
    let row = |[x, y]: [End; 2]| Ok(([x, y].map(row_cell), [None; 3], relation));
    let point = |c: Cell| matches!(c, Cell::Vertex(_));
    let two = read_cells.map(row_cell);
    let bys = reads.iter().filter_map(|&(_, by)| by);
    let minted = bys.clone().any(|by| !two.contains(&row_cell(by)));
    let throughs = through.iter().flat_map(|&end| {
        let (at, by) = read(end);
        std::iter::once(at).chain(by)
    });
    let mut further: Vec<crate::RowCell> = Vec::new();
    for c in bys.chain(throughs).map(row_cell) {
        if !two.contains(&c) && !further.contains(&c) {
            further.push(c);
        }
    }
    // A meeting is unordered: its cells sorted, each beyond the two read
    // once.
    let meet = || {
        let mut all: Vec<crate::RowCell> = two.into_iter().chain(further.iter().copied()).collect();
        all.sort();
        let (x, y, rest) = match all.as_slice() {
            [x, y, rest @ ..] if rest.len() <= 3 => (*x, *y, rest),
            _ => {
                return Err(BooleanError::JoinDesync {
                    what: "a meeting holds more cells than a row does",
                });
            }
        };
        let mut more = [None; 3];
        for (slot, &c) in more.iter_mut().zip(rest) {
            *slot = Some(c);
        }
        Ok(([x, y], more, crate::Relation::Meet))
    };
    let on = relation == crate::Relation::OnCarrier;
    match widened {
        _ if relation == crate::Relation::Meet => meet(),
        [false, false] => row(read_cells),
        _ if on && minted => meet(),
        [false, true] if on && point(cells[0].1) => row(read_cells),
        [true, false] if on && point(cells[0].1) && point(cells[1].1) => {
            row([read_cells[1], read_cells[0]])
        }
        _ if on => meet(),
        _ => Err(BooleanError::JoinDesync {
            what: "a decision other than a point's on a cell reads a cell it lies in",
        }),
    }
}

/// What [`spell`]'s `read` makes of a cell: the cell of the op's input
/// it lies in, and the cell of the other input whose crossing minted it,
/// where the op minted it on an edge.
type Read = (End, Option<End>);

/// What [`spell`] makes of a decision: its row's two cells, its further
/// ones ([`crate::Coincidence::more`]) and its relation.
type Spelled = (
    [crate::RowCell; 2],
    [Option<crate::RowCell>; 3],
    crate::Relation,
);

/// A pending row's cell as the deciding op's input holds it
/// ([`super::Mints`]): a vertex the reduction minted by splitting an
/// edge is read as the input edge it was minted on, through every split
/// of that edge's pieces ([`super::Mints::root`], total), and one it
/// minted inside a face as that face.
pub(super) fn input_cell(end: End, mints: &super::Mints) -> crate::RowCell {
    let (input, cell) = input_end(end, mints);
    crate::RowCell::Input { input, cell }
}

/// [`input_end`], and for a vertex the reduction minted on an edge the
/// cell of the other operand whose crossing minted it
/// ([`super::Mints::minted_by`]), read the same way.
fn input_read((input, cell): End, mints: &super::Mints) -> Read {
    let by = match cell {
        Cell::Vertex(v) => mints
            .minted_by(input, v)
            .map(|by| input_end((input.other(), by), mints)),
        Cell::Edge(_) | Cell::Face(_) => None,
    };
    (input_end((input, cell), mints), by)
}

/// [`input_cell`], as an operand and its cell.
fn input_end((input, cell): End, mints: &super::Mints) -> End {
    let root = |edge: EdgeKey| mints.root(input, edge);
    let cell = match cell {
        Cell::Vertex(v) => match (mints.minted_on(input, v), mints.minted_in(input, v)) {
            (Some(parent), _) => Cell::Edge(root(parent)),
            (None, Some(face)) => Cell::Face(face),
            (None, None) => cell,
        },
        Cell::Edge(e) => Cell::Edge(root(e)),
        // The sweep splits edges and never a face: every pending row is
        // written before the insertion mints a face, so a face key in the
        // clone is the input's.
        Cell::Face(_) => cell,
    };
    (input, cell)
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
pub(super) fn carry_in_place<T: Decide + AtRestPolicy>(
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
    let incidence = [
        Incidence::of(body, &BTreeSet::new()),
        Incidence::of(body, &BTreeSet::new()),
    ];
    let none_discarded = BTreeSet::new();
    carry_rows(
        body,
        &rows,
        [&none, &none],
        [&KeyView::Direct, &KeyView::Direct],
        (desc, &incidence, [&none_discarded, &none_discarded]),
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
fn carry_rows<T: Decide + AtRestPolicy>(
    body: &Body<T>,
    discovered: &[Tagged<'_>],
    carried: [&Rows; 2],
    views: [&KeyView<'_>; 2],
    (desc, incidence, discarded): (&Descendants, &[Incidence; 2], [&BTreeSet<FaceKey>; 2]),
    one_arena: bool,
    band: Band,
) -> Result<ContactRecords, BooleanError> {
    let sides = [Operand::A, Operand::B];
    let view = |side: Operand| views[usize::from(side == Operand::B)];
    let mut out = ContactRecords::default();
    // The rows by the kinds of their two cells, each vertex end first.
    let mut vv_rows: Vec<Tagged<'_>> = Vec::new();
    let mut ve_rows: Vec<Tagged<'_>> = Vec::new();
    // A vertex-on-face or two-cell row of the reduction's, with its
    // index in the working set, is placed where the join divided its
    // face ([`place_found`]); a carried one was placed by the carry.
    let mut vf_rows: Vec<(Tagged<'_>, Option<usize>)> = Vec::new();
    let mut cell_rows: Vec<(Tagged<'_>, Option<usize>)> = Vec::new();
    for (k, &([x, y], cites)) in discovered.iter().enumerate() {
        match (x.1, y.1) {
            (Cell::Vertex(_), Cell::Vertex(_)) => vv_rows.push(([x, y], cites)),
            (Cell::Vertex(_), Cell::Edge(_)) => ve_rows.push(([x, y], cites)),
            (Cell::Edge(_), Cell::Vertex(_)) => ve_rows.push(([y, x], cites)),
            (Cell::Vertex(_), Cell::Face(_)) => vf_rows.push((([x, y], cites), Some(k))),
            (Cell::Face(_), Cell::Vertex(_)) => vf_rows.push((([y, x], cites), Some(k))),
            (Cell::Edge(_) | Cell::Face(_), Cell::Edge(_) | Cell::Face(_)) => {
                cell_rows.push((([x, y], cites), Some(k)));
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
                (
                    [(side, Cell::Vertex(c.vertex)), (side, Cell::Face(c.face))],
                    &c.cites,
                ),
                None,
            )
        }));
        cell_rows.extend(rows.ee.iter().map(|c| {
            (
                ([(side, Cell::Edge(c.a)), (side, Cell::Edge(c.b))], &c.cites),
                None,
            )
        }));
        cell_rows.extend(rows.ef.iter().map(|c| {
            (
                (
                    [(side, Cell::Edge(c.edge)), (side, Cell::Face(c.face))],
                    &c.cites,
                ),
                None,
            )
        }));
        cell_rows.extend(rows.ff.iter().map(|c| {
            (
                ([(side, Cell::Face(c.a)), (side, Cell::Face(c.b))], &c.cites),
                None,
            )
        }));
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
    let mut confirms = super::records::Confirms::new(1);
    for (([(vs, vc), (fs, fc)], cites), found) in vf_rows {
        // A vertex a zip fused sits on the seam, on the boundary of the
        // face it rested on: the substituted pair is an incidence.
        let Cell::Vertex(v) = vc else {
            unreachable!("a vertex-on-face row's first end is its vertex")
        };
        if view(vs).vertex(v).is_some_and(|k| desc.fused.contains(&k)) {
            continue;
        }
        for (x, y) in place_found(
            body,
            (desc, incidence, discarded),
            views,
            [(vs, vc), (fs, fc)],
            found,
            &mut confirms,
            band,
        )? {
            record(body, &mut out, (vs, x), (fs, y), cites.clone());
        }
    }
    // Records of two edges or faces: the reduction's touches along a
    // line, the joins' and each operand's carried ones, both cells
    // chased.
    for (([(xs, xc), (ys, yc)], cites), found) in cell_rows {
        for (x, y) in place_found(
            body,
            (desc, incidence, discarded),
            views,
            [(xs, xc), (ys, yc)],
            found,
            &mut confirms,
            band,
        )? {
            record(body, &mut out, (xs, x), (ys, y), cites.clone());
        }
    }
    Ok(out)
}

/// **A record's cells in result keys**, placed by the one carry
/// ([`super::records::landings`]): each cell chased through
/// [`Descendants::live`] lands there by lineage, unread. A row of the
/// reduction's own (`found`, its index in the working set) on a face the
/// join divided after the row was written has that face divided into
/// its live pieces and the chords between them ([`face_pieces`]), its
/// remainder removed where the op stated a piece's removal
/// ([`stated_removed`]), and lands on every pair of images the census's
/// record confirm reads as meeting. A row whose every pair is structure
/// is consumed, as structural sharing needs no record (D1,
/// [`crate::Lineage::glued`]). None where the other cell left the
/// result: a row of the reduction's own leaves with its cell.
///
/// # Errors
///
/// [`BooleanError::CarryRefused`] ([`super::RefusedRow::Found`]) for a
/// row the carry refuses; [`BooleanError::JoinDesync`] for a piece the
/// result does not hold whose removal is not stated; as
/// [`Descendants::live`].
fn place_found<'b, T: Decide + AtRestPolicy>(
    body: &'b Body<T>,
    (desc, incidence, discarded): (&Descendants, &[Incidence; 2], [&BTreeSet<FaceKey>; 2]),
    views: [&KeyView<'_>; 2],
    ends: [End; 2],
    found: Option<usize>,
    confirms: &mut super::records::Confirms<'b, T>,
    band: Band,
) -> Result<Vec<(Cell, Cell)>, BooleanError> {
    let at = |side: Operand| usize::from(side == Operand::B);
    let mut divided: Option<(usize, crate::Fate)> = None;
    if found.is_some() {
        for (i, &(side, cell)) in ends.iter().enumerate() {
            let Cell::Face(f) = cell else { continue };
            let (pieces, chords, true) = face_pieces(desc, &incidence[at(side)], side, f) else {
                continue;
            };
            if divided.is_some() {
                return Err(BooleanError::JoinDesync {
                    what: "a row the reduction found names two faces the join divided",
                });
            }
            let removed = |c: Cell| stated_removed(&incidence[at(side)], discarded[at(side)], c);
            let (view, mut gone, mut back) = (views[at(side)], false, Vec::new());
            let pieces = imaged(
                body,
                desc,
                (side, view),
                &removed,
                cell,
                pieces,
                &mut gone,
                &mut back,
            )?;
            let chords = imaged(
                body,
                desc,
                (side, view),
                &removed,
                cell,
                chords,
                &mut gone,
                &mut back,
            )?;
            divided = Some((i, fate_of(pieces, chords, true, gone)));
        }
    }
    let live = |i: usize| {
        let (side, cell) = ends[i];
        desc.live(body, side, views[at(side)], cell)
    };
    // The row in the keys the lineage reads: a divided cell in its
    // operand's, a kept one as the result holds it.
    let (row, lineage) = match divided {
        None => {
            let (Some(x), Some(y)) = (live(0)?, live(1)?) else {
                return Ok(Vec::new());
            };
            ([x, y], crate::Lineage::new().glued())
        }
        Some((i, fate)) => {
            let Some(other) = live(1 - i)? else {
                return Ok(Vec::new());
            };
            let cells = ends.map(|(_, c)| c);
            let lineage = crate::Lineage::new()
                .glued()
                .with(cells[i], fate)
                .and_then(|l| {
                    l.with(
                        cells[1 - i],
                        crate::Fate::Kept {
                            at: other,
                            copies: Vec::new(),
                        },
                    )
                })
                .map_err(|_| BooleanError::JoinDesync {
                    what: "a row the reduction found names one cell twice",
                })?;
            (cells, lineage)
        }
    };
    let refused = |refusal: super::CarryRefusal| match found {
        Some(row) => BooleanError::CarryRefused {
            row: super::RefusedRow::Found,
            refusal: refusal.renumbered(super::reduce::index(row)),
        },
        None => BooleanError::JoinDesync {
            what: "the carry refused a row it places by lineage",
        },
    };
    let landed =
        super::records::landings(&[row], &lineage, &[body], confirms, band, T::region_lane())
            .map_err(refused)?;
    landed
        .into_iter()
        .map(|l| {
            if l.minted {
                return Err(BooleanError::JoinDesync {
                    what: "a row the reduction found lands on two cells the join minted",
                });
            }
            Ok((l.at[0], l.at[1]))
        })
        .collect()
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
/// Whether vertices `a` and `b` of `body` are one point: their gap
/// decided Zero at `band` by `predicate`, with the margin read.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] where a vertex has no point, and
/// [`BooleanError::Escalated`] (`VertexOnVertex`) where the gap is in
/// band.
fn vertices_meet<T: Decide>(
    body: &Body<T>,
    [a, b]: [VertexKey; 2],
    predicate: &'static str,
    band: Band,
) -> Result<(bool, geom_core::MarginDiag), BooleanError> {
    let gap = match (vertex_point(body, a), vertex_point(body, b)) {
        (Some(p), Some(q)) => Margin::norm3(p - q),
        _ => {
            return Err(BooleanError::JoinDesync {
                what: "a vertex the carry or the output stage reads has no point",
            });
        }
    };
    let (magnitude, margin) = geom_core::k_stats::decide_magnitude_reported(predicate, gap, band)
        .map_err(|diag| BooleanError::Escalated {
        decision: BooleanDecision::VertexOnVertex,
        diag,
    })?;
    Ok((magnitude == geom_core::k_stats::Magnitude::Zero, margin))
}

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

/// **The surface pairs a Boolean glued one carrier**, in its result's
/// keys: what the result's merge reads as one carrier, through the
/// glue's record and no ladder ([`Body::merge_coplanar_faces_unjoined`]).
/// Only the Boolean's result lowering builds one ([`Self::of_result`]),
/// so no caller states a coincidence to the merge.
#[derive(Clone, Debug, Default)]
pub(crate) struct GluedSurfaces {
    pairs: Vec<GluedSurfacePair>,
}

/// A surface and the sense its faces carry.
pub(crate) type SensedSurface = (SurfaceKey, bool);

/// **Two surfaces a Boolean glued one carrier**, the kept side's first
/// ([`super::glue::kept_first`]: the description the merged face
/// keeps), and whether their charts face the same way in the result. A
/// face's material side is its sense applied to its surface's chart,
/// and fragments inherit both from their parent, so which way the two
/// charts face holds for every face either surface carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct GluedSurfacePair {
    /// The kept side's surface.
    pub(crate) kept: SurfaceKey,
    /// The other side's surface.
    pub(crate) other: SurfaceKey,
    /// Whether the two charts face the same way.
    pub(crate) charts_agree: bool,
}

impl GluedSurfacePair {
    /// The pair of `kept` and `other` whose faces of senses `(sk, so)`
    /// have material sides that agree or not (`materials_agree`).
    fn of((kept, sk): SensedSurface, (other, so): SensedSurface, materials_agree: bool) -> Self {
        Self {
            kept,
            other,
            charts_agree: materials_agree == (sk == so),
        }
    }
}

impl GluedSurfaces {
    /// No glued pair: the merge reads every pair by its margins.
    pub(crate) fn none() -> Self {
        Self::default()
    }

    /// The pairs.
    pub(crate) fn pairs(&self) -> &[GluedSurfacePair] {
        &self.pairs
    }

    /// `pairs` as a Boolean would glue them, each two surfaces whose
    /// faces of senses `(sk, so)` face the same way, kept side first,
    /// for the merge rows that drive its glued path on one body.
    #[cfg(test)]
    pub(crate) fn of_pairs(pairs: &[(SensedSurface, SensedSurface)]) -> Self {
        Self {
            pairs: pairs
                .iter()
                .map(|&(a, b)| GluedSurfacePair::of(a, b, true))
                .collect(),
        }
    }

    /// **The glued face pairs lowered to SURVIVING result SURFACE
    /// pairs**: the equivalence rides surfaces because fragments
    /// inherit their parent's surface key — face-key churn (a glued face
    /// whose original key died with a discarded fragment) cannot strand
    /// the glue as long as any fragment keeps the surface alive. A pair
    /// with a consumed side (surface gone from the result) licenses
    /// nothing and drops — its contact material did not survive the op
    /// (the same consumed-record rule as contact rows). A `Tangent`
    /// pair's carriers are distinct, and never merge.
    ///
    /// The record's relation says whether the two operand faces'
    /// material sides agree; `op` turns B's inside out where it
    /// subtracts (`A ∖ B ≡ A ∩ revert(B)`), and the result's faces on
    /// the two surfaces carry the senses the charts are read under.
    pub(super) fn of_result<T: Real>(
        op: BooleanOp,
        result: &Body<T>,
        a: &Body<T>,
        b: &Body<T>,
        decls: &Glue<T>,
        graft: &GraftMap,
    ) -> Self {
        let sense_on = |k: SurfaceKey| {
            result
                .faces()
                .find(|(_, face)| face.surface == k)
                .map(|(_, face)| face.sense)
        };
        Self {
            pairs: decls
                .pairs
                .iter()
                .filter_map(|pair| {
                    let relation = pair.class.carrier_relation()?;
                    // A-clone surface keys ARE result keys (carve/clone
                    // preserve them); B bridges through the graft.
                    let ka = a.get_face(pair.a)?.surface;
                    let kb = graft.surfaces.get(b.get_face(pair.b)?.surface).copied()?;
                    let materials_agree = (relation == super::CarrierRelation::SameOriented)
                        != (op == BooleanOp::Subtract);
                    (ka != kb).then_some(())?;
                    let (kept, other) = super::glue::kept_first(
                        (Operand::A, (ka, sense_on(ka)?)),
                        (kb, sense_on(kb)?),
                    );
                    Some(GluedSurfacePair::of(kept, other, materials_agree))
                })
                .collect(),
        }
    }

    /// The pairs both of whose surfaces `body` still holds.
    pub(super) fn surviving<T: Real>(&self, body: &Body<T>) -> Self {
        Self {
            pairs: self
                .pairs
                .iter()
                .copied()
                .filter(|p| {
                    body.get_surface(p.kept).is_some() && body.get_surface(p.other).is_some()
                })
                .collect(),
        }
    }
}

/// **The result gate every [`BooleanBody`] passes**, at the site that
/// built it: the body is sorted one solid per piece of material
/// ([`crate::pieces`]), then
/// finished ([`AtRestPolicy::gate_at_rest_kept`]: tier 3, whose first
/// act is tiers 1 and 2), so the verdict rides the result and is taken
/// on the bits the caller receives. Where the scalar runs no at-rest
/// gate, [`structural_gate`] runs in its place. The at-rest gate's
/// findings are typed by [`finished_body_refusal`]; the structural
/// gate's are structure alone, the kernel's. A solid whose only shell
/// check 7 certified in band passes: how thin a result is gets judged
/// by its pairs alone (D10, Booleans). Last, the edges on every line
/// where two or more lie are read for disjoint material
/// ([`coincident_wedges_disjoint`]), at every scalar.
///
/// # Errors
///
/// [`BooleanError::Pieces`] where the body's pieces cannot be read;
/// [`BooleanError::Escalated`] where every finding is the operands'
/// ill-conditioning, and [`BooleanError::ResultInvalid`] carrying the
/// validator's findings otherwise; [`coincident_wedges_disjoint`]'s.
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
    if kept.outcome() == crate::AtRestOutcome::NotRunAtThisScalar {
        structural_gate(&kept, band)?;
    }
    coincident_wedges_disjoint(&kept, band)?;
    Ok(kept)
}

/// **How the at-rest gate's refusal of a door's result is typed** (D10,
/// Booleans), for the two doors that share it: the Boolean's [`gate`]
/// and [`crate::splitting::split`]. The other doors that gate at rest
/// type on their own
/// (`work/cleave/the-at-rest-result-gate-and-its-typing-live-in-five-homes.md`).
/// A result read in band is the operands' ill-conditioning, which
/// definite cuts can compose: where every finding is a reading in band of
/// a shell's role (check 10) or a solid's orientation (check 7) — an
/// aimed ray's pair in band ([`BooleanDecision::ShellRole`]) — the
/// refusal is in band, on the reading whose margin binds the offer
/// (nearest zero: a tolerance deciding it decides every other), naming
/// how many more lie in band. The volume, read where no ray reads, is
/// never in band: its sign is certified by its enclosure alone (D10), so
/// an enclosure holding zero is the kernel's.
#[derive(Debug)]
pub(crate) enum AtRestRefusal {
    /// Every finding is a reading in band: the binding one's decision,
    /// counting the others, with its margin. Its recourse is a smaller
    /// tolerance or moved geometry.
    InBand {
        /// [`BooleanDecision::ShellRole`] on the binding reading.
        decision: BooleanDecision,
        /// The binding reading's margin.
        diag: geom_core::Indeterminate,
    },
    /// Any other finding: the kernel's own defect or limit, every
    /// finding carried, each with its own ending.
    Kernel(Vec<ValidationError>),
}

/// The at-rest gate's findings typed ([`AtRestRefusal`]); each finding's
/// arm is [`finding_arm`]'s.
pub(crate) fn at_rest_refusal(errors: Vec<ValidationError>) -> AtRestRefusal {
    let in_band: Option<Vec<InBand>> = errors.iter().map(finding_arm).collect();
    match in_band.as_deref().and_then(binding) {
        Some(InBand { decision, diag }) => AtRestRefusal::InBand { decision, diag },
        None => AtRestRefusal::Kernel(errors),
    }
}

/// **How the finished-body gate's refusal reads at the Boolean**: the
/// shared typing ([`at_rest_refusal`]), a reading in band as
/// [`BooleanError::Escalated`] and the kernel's findings as
/// [`BooleanError::ResultInvalid`].
pub(super) fn finished_body_refusal(errors: Vec<ValidationError>) -> BooleanError {
    match at_rest_refusal(errors) {
        AtRestRefusal::InBand { decision, diag } => BooleanError::Escalated { decision, diag },
        AtRestRefusal::Kernel(errors) => BooleanError::ResultInvalid { errors },
    }
}

/// **A reading in band** of the result's side at infinity: the decision
/// it escalates as, with no count of the others yet, and its margin.
#[derive(Clone, Copy, Debug)]
struct InBand {
    /// The decision, `others` zero.
    decision: BooleanDecision,
    /// Its margin.
    diag: Indeterminate,
}

/// The binding one of `readings`, the result's readings in band: the one
/// whose margin binds the offer, its decision counting the others.
/// `None` where there are none.
fn binding(readings: &[InBand]) -> Option<InBand> {
    let binding = readings.iter().copied().reduce(|a, b| {
        if a.diag.margin.binds_before(b.diag.margin) {
            a
        } else {
            b
        }
    })?;
    let more = u32::try_from(readings.len() - 1).unwrap_or(u32::MAX);
    let decision = match binding.decision {
        BooleanDecision::ShellRole { near, far, .. } => BooleanDecision::ShellRole {
            near,
            far,
            others: more,
        },
        other => other,
    };
    Some(InBand {
        decision,
        diag: binding.diag,
    })
}

/// **Which of Q1's arms one finding of the finished-body gate is**: the
/// decision and margin of a reading in band, or `None` for the kernel's.
/// Every finding states its arm here.
fn finding_arm(finding: &ValidationError) -> Option<InBand> {
    use crate::props::ShellClassifyError;
    use ValidationError as V;
    match finding {
        // In band: an aimed ray met two faces that do not meet in band
        // of each other. A ray set aside at a sharp corner (two faces
        // that meet) is no pair, and never reaches here as one.
        V::ShellRoleUndecided {
            error: ShellClassifyError::RayInBand { pair, .. },
            ..
        }
        | V::OrientationUndecided {
            reading: OrientationUnread::Pair(pair),
            ..
        } => Some(InBand {
            decision: BooleanDecision::ShellRole {
                near: pair.near,
                far: pair.far,
                others: 0,
            },
            diag: pair.gap,
        }),
        // The kernel's otherwise: a volume whose certified enclosure holds
        // zero (its sign is read off that enclosure alone, D10), an aimed
        // ray's walk refusal on a broken face, a limit, or a band that
        // does not form.
        V::ShellRoleUndecided { .. } | V::OrientationUndecided { .. } => None,
        // Undecided at a point margin the gate has not shown conditioned
        // (`work/tally/the-door-gates-other-in-band-findings-are-typed-the-kernels.md`).
        V::DegenerateTorusEscalated { .. }
        | V::PlanarFaceEscalated { .. }
        | V::PlanarBoundaryEscalated { .. }
        | V::SliverDihedral { .. }
        | V::MixedDihedral { .. }
        | V::JoinUndecidedAtRest { .. }
        | V::VolumeSignUnresolved { .. }
        | V::SeamRegionCoverUnresolved { .. }
        | V::ChartSingularityUndecided { .. }
        | V::WrapTipUndecided { .. }
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
        | V::FaceThroughEdgeWedge { .. }
        | V::FaceThroughEdgeWedgeUndecided { .. }
        | V::CoincidentWedgesOverlap { .. }
        | V::CoincidentWedgesOverlapUndecided { .. }
        | V::BoundaryCrossesItself { .. }
        | V::BoundaryCrossesItselfUndecided { .. }
        | V::ArcEndOffCorner { .. }
        | V::InstanceInterference { .. }
        | V::SameSideInterference { .. }
        | V::TouchInterference { .. }
        | V::CuspEdgesMissing { .. } => None,
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
        | V::SeamRegionOvercovered { .. }
        | V::ChartSingularityNotAVertex { .. }
        | V::WrapTipOffChartSingularity { .. }
        | V::WrapTipWithoutChartSingularity { .. }
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
/// ([`ValidationError::ScaffoldAtRest`])
/// and its check 11 in `band`
/// ([`ValidationError::JoinableVertexAtRest`]),
/// which read no certification arithmetic and so answer at every
/// scalar. An edge of the result still described as a scaffold is a
/// construction that stopped half-way, and a joinable vertex one the
/// join did not finish.
pub(super) fn structural_gate<T: Decide>(body: &Body<T>, band: Band) -> Result<(), BooleanError> {
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
    /// re-chart turns `u_ref` to about `align`. A's group takes the
    /// first escape's [`pair_seam`] in the plane across `align`, and B's
    /// that direction turned a quarter about `align`, so two balls
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
    evidence: &Evidence<'_>,
    band: Band,
) -> Result<(Recuts<T>, OnDeferred), BooleanError> {
    let rest = evidence.rest;
    let mut deferred = OnDeferred::default();
    let esc = |question| {
        move |diag| BooleanError::Escalated {
            decision: BooleanDecision::Sphere(question),
            diag,
        }
    };
    // Every surface is read through its link first: a torn surface is a
    // broken at-rest operand, which panics (D2 row 4), whereas
    // `face_rows` reads surfaces by lookup and would answer it as a lost
    // surface, a refusal.
    for body in [a, b] {
        for (face, fd) in body.faces() {
            let _ = body.face_surface_linked(face, fd);
        }
    }
    let pad = boxes::sweep_pad(band);
    let axes = super::separating::OperandAxes::new(boxes::axis_key);
    let mut section_charts = ChartCache::default();
    let rows = [face_rows(a, band)?, face_rows(b, band)?];
    // The NURBS re-gate (M5 S13, pinned), REACH FIRST: a NURBS face
    // whose certified box (its control net's hull) meets no face of the
    // other operand is certified disjoint from it, and every other
    // fallback entry with a NURBS face refuses before a vertex is
    // probed — the extent test is unwritable for the kind (variant docs).
    for (operand, x, x_rows, y, y_rows) in [
        (Operand::A, a, &rows[0], b, &rows[1]),
        (Operand::B, b, &rows[1], a, &rows[0]),
    ] {
        for row in x_rows {
            if !matches!(row.surface, geom::Surface::Nurbs(_)) {
                continue;
            }
            let reaches = y_rows.iter().any(|y_row| {
                row.bbox.overlaps(&y_row.bbox)
                    && !super::separating::apart(
                        (x, super::separating::Item::Face(row.face)),
                        (y, super::separating::Item::Face(y_row.face)),
                        axes.of(a, b, band),
                        pad,
                        band,
                    )
            });
            if reaches {
                return Err(BooleanError::NurbsExtentUnsupported {
                    operand,
                    face: row.face,
                });
            }
        }
    }
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
            // The first sphere escape's pair seam ([`pair_seam`]).
            let mut first_pair_seam: Option<Vec3<T>> = None;
            for (y_row, (yf, _)) in y_rows.iter().zip(y.faces()) {
                if y_row.face != yf {
                    return Err(BooleanError::ClassificationInvariant {
                        what: "extent scan: face rows out of arena order",
                    });
                }
                // Each pair of a face on this sphere with `yf` the
                // section certificate refuses, with its refusal; empty
                // when it clears every one ([`sphere_faces_apart`]). A
                // carrier margin decided zero is a touch, no event where
                // this comes back empty; one in the band asks too, and
                // the certificate, reading it undecided, refuses.
                let mut faces = || {
                    sphere_faces_apart(
                        (x_is, x, x_rows),
                        fd.surface,
                        (y, y_row),
                        evidence.events,
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
                                    if !faces()?.is_empty() {
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
                                    None => match faces()?.first() {
                                        None => continue,
                                        held => held.copied(),
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
                                if !faces()?.is_empty() {
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
                                // `yf`: with no crossing anywhere, a circle
                                // certified out of one face of each pair,
                                // or evidenced by a touch on it (W4),
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
                                // faces are not certified apart, unless the
                                // reduction settled the pair one carrier
                                // ([`OnDeferred::hold`]).
                                let nested = crate::validate::decide_reported(
                                    "bool_sphere_sphere_nested",
                                    Margin::of(big - (d + small)),
                                    band,
                                )
                                .map_err(esc(SphereQuestion::Nested))?;
                                match Refused::of(nested, band) {
                                    None => {}
                                    Some(verdict @ Refused::Zero(_)) => {
                                        for (held, _) in faces()? {
                                            let pair = match x_is {
                                                Operand::A => (held, yf),
                                                Operand::B => (yf, held),
                                            };
                                            deferred.hold(
                                                evidence.settled,
                                                pair,
                                                BooleanError::SpheresMeet {
                                                    operand: x_is,
                                                    face,
                                                    verdict,
                                                },
                                            )?;
                                        }
                                        continue;
                                    }
                                    Some(Refused::Negative { .. }) => match faces()?
                                        .first()
                                        .copied()
                                    {
                                        None => {}
                                        Some((holder, SectionRefusal::Loop)) => {
                                            let cut = sphere_pair_cut(
                                                (x_is, holder),
                                                x_surface,
                                                &y_row.surface,
                                                band,
                                            )?;
                                            if group.is_some() {
                                                if first_pair_seam.is_none() {
                                                    first_pair_seam = Some(pair_seam(
                                                        x_is,
                                                        x_surface,
                                                        &y_row.surface,
                                                        band,
                                                    )?);
                                                }
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
                    // The re-gate above certified the face clear of
                    // every face of this operand, the sphere's among
                    // them, or refused.
                    geom::Surface::Nurbs(_) => {}
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
                let seam = first_pair_seam.map(|pair| {
                    let first = (pair - align * align.dot(pair)).normalize();
                    match x_is {
                        Operand::A => first,
                        Operand::B => align.cross(first),
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
    Ok((
        Recuts {
            rechart: out,
            cut_in: cut_ins,
        },
        deferred,
    ))
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

/// **The seam a sphere pair's re-charts share**: the `u_ref` of the
/// pair's section circle as read from operand A's sphere
/// ([`geom_brep::sphere_sphere_section`] with A's sphere first), whichever
/// operand `x` (on `x_is`) is. Both groups of a pair read the same
/// direction, and it is a function of the operands, never of the
/// computing axes (D10 `[placement.frame]`).
fn pair_seam<T: Decide>(
    x_is: Operand,
    x: &geom::Surface<T>,
    y: &geom::Surface<T>,
    band: Band,
) -> Result<Vec3<T>, BooleanError> {
    let (a, b) = match x_is {
        Operand::A => (x, y),
        Operand::B => (y, x),
    };
    match geom_brep::sphere_sphere_section(a, b, band) {
        Ok(geom_brep::SphereSphereSection::Circle(geom::Curve3::Circle { u_ref, .. })) => Ok(u_ref),
        Err(geom_brep::SectionError::Escalated(diag)) => Err(BooleanError::Escalated {
            decision: BooleanDecision::Sphere(SphereQuestion::CutIn),
            diag,
        }),
        Ok(_) | Err(_) => Err(BooleanError::ClassificationInvariant {
            what: "extent scan: spheres the scan decided cross have no section circle",
        }),
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
/// pairs, with the sweep's touches as its events. Each refusing pair's
/// face of `x` and its refusal, in arena order: empty when every pair
/// clears, and the first is the cause.
///
/// # Errors
///
/// [`walk_pairs`]'.
fn sphere_faces_apart<T: Decide + Bounds + crate::props::AtRestPolicy>(
    (x_is, x, x_rows): (Operand, &Body<T>, &[FaceRow<T>]),
    surface: SurfaceKey,
    (y, y_row): (&Body<T>, &FaceRow<T>),
    events: &BTreeSet<(FaceKey, FaceKey)>,
    band: Band,
    charts: &mut ChartCache,
    axes: &super::separating::OperandAxes<T>,
) -> Result<Vec<(FaceKey, SectionRefusal)>, BooleanError> {
    let on_sphere = x_rows.iter().filter(|r| r.key == surface);
    let pairs = match x_is {
        Operand::A => walk_pairs(
            (x, on_sphere),
            (y, [y_row]),
            &super::section_table::SectionTable::default(),
            band,
            charts,
            |_, _| true,
            |fa, fb| events.contains(&(fa, fb)),
            |_| true,
            false,
            axes,
        )?,
        Operand::B => walk_pairs(
            (y, [y_row]),
            (x, on_sphere),
            &super::section_table::SectionTable::default(),
            band,
            charts,
            |_, _| true,
            |fa, fb| events.contains(&(fa, fb)),
            |_| true,
            false,
            axes,
        )?,
    };
    Ok(pairs
        .into_iter()
        .filter_map(|p| {
            let held = match x_is {
                Operand::A => p.a_face,
                Operand::B => p.b_face,
            };
            p.verdict.err().map(|refusal| (held, refusal))
        })
        .collect())
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
///
/// Returns the re-cut operands, A's then B's, and what the re-charts
/// made in each.
fn apply_recuts<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    recuts: &[SphereRecut<T>],
    tol: Tol,
) -> Result<(Body<T>, Body<T>, RechartsPerOperand), BooleanError> {
    let band = Band::linear(tol)?;
    let mut out_a = a.clone();
    let mut out_b = b.clone();
    let mut made: RechartsPerOperand = Default::default();
    for ((operand, out), made) in [(Operand::A, &mut out_a), (Operand::B, &mut out_b)]
        .into_iter()
        .zip(&mut made)
    {
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
        let mut rotated: Vec<(Body<T>, Vec<FaceKey>)> = Vec::new();
        let mut cut_shells: Vec<ShellKey> = Vec::new();
        for r in &mine {
            let shell = shell_of(r.representative)?;
            cut_shells.push(shell);
            let faces = src
                .get_shell(shell)
                .ok_or(corrupt("re-cut shell lost"))?
                .faces
                .clone();
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
            // Built from bare unit-by-convention directions, so its
            // rigidity is decided at the door rather than carried
            // (`work/cleave/the-sphere-re-cut-rotation-is-decided-at-the-door-not-minted`).
            let map = geom_core::RigidMotion::check_rigid(&map, band)
                .map_err(|_| corrupt("re-cut rotation is not decided rigid"))?;
            let turned = crate::transform::transform_rigid(&ball, &map, tol)
                .map_err(|_| corrupt("re-cut rotation failed to re-certify"))?;
            rotated.push((turned, faces));
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
        // A turned shell that becomes the base keeps its input keys at
        // turned places, and a grafted one takes new keys: either way its
        // cells are the ones it adds to the rebuilt body.
        for (turned, faces) in rotated {
            let cells = match rebuilt.as_mut() {
                None => {
                    let cells = cells_of(&turned);
                    rebuilt = Some(turned);
                    cells
                }
                Some(base) => {
                    let base_solid =
                        single_solid(base).map_err(|_| corrupt("re-cut base is not one solid"))?;
                    let before = cells_of(base);
                    graft_solids_disjoint(base, &[base_solid], &turned)?;
                    cells_of(base).difference(&before).copied().collect()
                }
            };
            made.add(cells, faces);
        }
        out.adopt(rebuilt.ok_or(corrupt("re-cut produced no body"))?);
    }
    Ok((out_a, out_b, made))
}

/// Every cell of `body`.
fn cells_of<T: Real>(body: &Body<T>) -> BTreeSet<Cell> {
    body.faces()
        .map(|(f, _)| Cell::Face(f))
        .chain(body.edges().map(|(e, _)| Cell::Edge(e)))
        .chain(body.vertices().map(|(v, _)| Cell::Vertex(v)))
        .collect()
}

/// What [`apply_recuts`] made in each operand, A's then B's.
type RechartsPerOperand = [Recharts; 2];

/// What [`apply_recuts`] made in one operand: the cells of each sphere
/// shell it turned about its centre, each with the faces of the input
/// shell it was turned over.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Recharts {
    /// Each re-charted shell's cells in the re-cut operand, to that
    /// shell's place in `shells`.
    cells: BTreeMap<Cell, usize>,
    /// The faces of the input shell each re-charted shell was turned
    /// over, sorted.
    shells: Vec<Vec<FaceKey>>,
}

impl Recharts {
    /// Whether `cell` of the re-cut operand is a re-charted shell's.
    #[must_use]
    pub fn contains(&self, cell: Cell) -> bool {
        self.cells.contains_key(&cell)
    }

    /// Whether no shell was re-charted.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.shells.is_empty()
    }

    /// Records one re-charted shell: its cells in the re-cut operand and
    /// its input shell's faces.
    fn add(&mut self, cells: BTreeSet<Cell>, mut faces: Vec<FaceKey>) {
        faces.sort_unstable();
        let at = self.shells.len();
        self.shells.push(faces);
        self.cells.extend(cells.into_iter().map(|cell| (cell, at)));
    }

    /// The faces of the input shell whose sphere `cell` of the re-cut
    /// operand was turned over, where `cell` is a re-charted shell's.
    ///
    /// The turn moves the shell's cells over its sphere, so no input
    /// cell finer than that sphere holds one, and every face of the
    /// input shell wears that sphere: a closed sphere group
    /// ([`super::solid_contain::closed_sphere_group`]) is the whole
    /// shell.
    fn input_faces(&self, cell: Cell) -> Option<&[FaceKey]> {
        self.cells.get(&cell).map(|&at| self.shells[at].as_slice())
    }
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
///
/// Returns what the cuts made in each operand, A's then B's.
fn apply_cut_ins<T: Decide + crate::props::AtRestPolicy>(
    a: &mut Body<T>,
    b: &mut Body<T>,
    cut_ins: &[SphereCutIn<T>],
    band: Band,
    tol: Tol,
) -> Result<CutsPerOperand, BooleanError> {
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
    let mut made: CutsPerOperand = Default::default();
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
                    let read_region = |e: super::region::RegionRefusal| match e {
                        super::region::RegionRefusal::Escalated(diag) => esc(diag),
                        super::region::RegionRefusal::RayExhausted { .. }
                        | super::region::RegionRefusal::WoundPastPeriod
                        | super::region::RegionRefusal::Unreadable(_) => unread(),
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
                let roots = super::circle_roots::first_harmonic_roots(
                    &super::circle_roots::FirstHarmonic::circle_off_plane(
                        (cc, ca, cr, cu),
                        (cut.center, k),
                    ),
                    cr,
                    t0,
                    t1,
                    &ROWS,
                    band,
                )?;
                let thetas = match roots {
                    Ok(geom_brep::roots::Roots::Miss) => continue,
                    Ok(geom_brep::roots::Roots::Certified(found)) => found.roots().to_vec(),
                    Ok(geom_brep::roots::Roots::On(_)) | Err(_) => {
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
        let mut arc_splits = Vec::new();
        let mut end =
            |split: Option<(EdgeKey, T, bool, T)>, vertex: Option<VertexKey>| match (split, vertex)
            {
                (_, Some(v)) => Ok(v),
                (Some((edge, theta, _, _)), None) => {
                    let created = body.split_edge(edge, theta, tol)?;
                    arc_splits.push((created.vertex, created.new_edge, edge));
                    Ok(created.vertex)
                }
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
                    selector: geom_brep::CurveSelector::Witness(carrier.mid_point(T::zero(), span)),
                },
                carrier,
                param_start: T::zero(),
                param_end: span,
            },
            crate::euler::FaceSurface::Inherit,
            tol,
        )?;
        pieces.push((cut.operand, cut.face, created.face));
        let side = &mut made[usize::from(cut.operand == Operand::B)];
        side.divisions.push((created.face, face));
        side.edges.push(created.edge);
        side.arc_splits.extend(arc_splits);
    }
    for body in [a, b] {
        crate::pcurves::mint_pcurves(body, tol)
            .map_err(|source| BooleanError::Pcurves { source })?;
    }
    Ok(made)
}

/// **The recut's rows read through the cut-ins and the re-charts**
/// ([`spell`]): each row whose cells the cuts or the re-charts made is
/// spelled in the op's inputs' cells, one row for one, so the records
/// cite the rows they cited.
///
/// A re-charted shell's cell is read back as the set of its input
/// shell's faces ([`Recharts::input_faces`]), which wear the sphere the
/// turn moved it over, as a vertex minted on an edge is read as that
/// edge. A shell of one face holds a turned face whole, so a row on one
/// keeps its relation, and a turned edge or vertex reads as that face
/// through [`spell`]. A set of several faces states only a point decided
/// on a cell ([`states_a_point`]), as the [`crate::Relation::Meet`] of
/// every cell the row's cells read as, sorted.
///
/// # Errors
///
/// [`BooleanError::RechartedRowUnstated`] for a row of any other
/// decision on a re-charted cell, and for a meeting of more cells than a
/// row holds; otherwise as [`spell`].
fn respell_through_cuts(
    rows: &mut [crate::Coincidence],
    cuts: &CutsPerOperand,
    recharted: &RechartsPerOperand,
) -> Result<(), BooleanError> {
    // A cell of a re-cut operand read through the cut-ins, beside the
    // input shell's faces where a re-chart turned it: a cut-in cuts a
    // face no re-chart turned.
    let recut = |(input, cell): End| {
        let cell = cuts[side(input)].input_cell(cell);
        (input, cell, recharted[side(input)].input_faces(cell))
    };
    let read = |end: End| match recut(end) {
        (input, _, Some(&[face])) => ((input, Cell::Face(face)), None),
        (input, cell, _) => ((input, cell), None),
    };
    let end = |cell| match cell {
        crate::RowCell::Input { input, cell } => Some((input, cell)),
        crate::RowCell::Tool | crate::RowCell::Result { .. } => None,
    };
    for row in rows {
        let [Some(x), Some(y)] = row.cells.map(end) else {
            continue;
        };
        let through = row
            .more
            .into_iter()
            .flatten()
            .map(|cell| {
                end(cell).ok_or(BooleanError::JoinDesync {
                    what: "a meeting's further cell is no cell of the op's input",
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let ends: Vec<End> = [x, y].into_iter().chain(through.iter().copied()).collect();
        let point = states_a_point(row.relation, recut(x).1);
        // The first cell read as a set of several faces.
        let mut set = None;
        for (operand, cell, faces) in ends.iter().copied().map(recut) {
            let Some(faces) = faces else {
                continue;
            };
            let whole = faces.len() == 1 && matches!(cell, Cell::Face(_));
            if !whole && !point {
                return Err(BooleanError::RechartedRowUnstated {
                    operand,
                    relation: row.relation,
                    faces: faces.to_vec(),
                    what: "only a point decided there is stated by the faces that hold it",
                });
            }
            if faces.len() > 1 && set.is_none() {
                set = Some((operand, faces));
            }
        }
        let Some((operand, faces)) = set else {
            (row.cells, row.more, row.relation) = spell([x, y], &through, row.relation, read)?;
            continue;
        };
        let mut all: Vec<crate::RowCell> = Vec::new();
        for (input, cell, faces) in ends.iter().copied().map(recut) {
            let read: Vec<Cell> = match faces {
                Some(faces) => faces.iter().map(|&f| Cell::Face(f)).collect(),
                None => vec![cell],
            };
            all.extend(
                read.into_iter()
                    .map(|cell| crate::RowCell::Input { input, cell }),
            );
        }
        all.sort();
        all.dedup();
        let (x, y, rest) = match all.as_slice() {
            [x, y, rest @ ..] if rest.len() <= row.more.len() => (*x, *y, rest),
            _ => {
                return Err(BooleanError::RechartedRowUnstated {
                    operand,
                    relation: row.relation,
                    faces: faces.to_vec(),
                    what: "the faces that hold it and the row's other cells are more cells \
                           than a row holds",
                });
            }
        };
        let mut more = [None; 3];
        for (slot, &c) in more.iter_mut().zip(rest) {
            *slot = Some(c);
        }
        (row.cells, row.more, row.relation) = ([x, y], more, crate::Relation::Meet);
    }
    Ok(())
}

/// Whether a `relation` decided with `first` its first cell is a point
/// decided on a cell, which holds of every cell that holds that cell: a
/// meeting, or a vertex on a carrier. Every other relation is decided of
/// the cells' carriers or of their shapes, which a cell widened to the
/// cells holding it does not keep.
fn states_a_point(relation: crate::Relation, first: Cell) -> bool {
    use crate::Relation;
    match relation {
        Relation::Meet => true,
        Relation::OnCarrier => matches!(first, Cell::Vertex(_)),
        Relation::SameOriented
        | Relation::SameOpposite
        | Relation::CommonFoot
        | Relation::Tangent { .. }
        | Relation::SurfaceTangent
        | Relation::Coaxial
        | Relation::SameRadius
        | Relation::AxesMeet
        | Relation::CoRuled => false,
    }
}

/// What [`apply_cut_ins`] made in each operand, A's then B's.
type CutsPerOperand = [ChartCuts; 2];

/// What [`apply_cut_ins`] made in one operand.
#[derive(Default)]
struct ChartCuts {
    /// `(new face, divided-from face)`, in mint order.
    divisions: Vec<(FaceKey, FaceKey)>,
    /// The meridian edge each cut minted, beside its division.
    edges: Vec<EdgeKey>,
    /// `(new vertex, new edge, split edge)` for each arc a cut's end
    /// split, in mint order.
    arc_splits: Vec<(VertexKey, EdgeKey, EdgeKey)>,
}

impl ChartCuts {
    /// The input cell `cell` of the cut operand lies in: a division's
    /// face is the face it was cut from, a meridian the face it runs
    /// in, and an arc split's vertex or second piece the arc it split.
    fn input_cell(&self, mut cell: Cell) -> Cell {
        // One step at a time to a fixed point, so a cell a later cut
        // made of an earlier cut's (an arc split of a meridian) reads
        // through both.
        loop {
            let next = match cell {
                Cell::Face(f) => self
                    .divisions
                    .iter()
                    .find(|&&(new, _)| new == f)
                    .map(|&(_, from)| Cell::Face(from)),
                Cell::Edge(e) => self
                    .edges
                    .iter()
                    .position(|&m| m == e)
                    .map(|i| Cell::Face(self.divisions[i].1))
                    .or_else(|| {
                        self.arc_splits
                            .iter()
                            .find(|&&(_, new, _)| new == e)
                            .map(|&(_, _, split)| Cell::Edge(split))
                    }),
                Cell::Vertex(v) => self
                    .arc_splits
                    .iter()
                    .find(|&&(w, _, _)| w == v)
                    .map(|&(_, _, split)| Cell::Edge(split)),
            };
            match next {
                Some(next) => cell = next,
                None => return cell,
            }
        }
    }
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
/// It runs only where the reduction minted no null pair, which is what
/// certifies a deferred pair's sides ([`OnDeferred::answered`]).
#[allow(clippy::too_many_arguments)]
fn fallback<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    red: &mut BooleanReduction<T>,
    a_pristine: &Body<T>,
    b_pristine: &Body<T>,
    decls: &Glue<T>,
    deferred: OnDeferred,
    band: Band,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    if !red.null_pairs.is_empty() {
        return Err(BooleanError::ClassificationInvariant {
            what: "the containment fallback reached with null pairs",
        });
    }
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
    deferred.answered(
        [(&red.a, &a_sides), (&red.b, &b_sides)],
        [a_pristine, b_pristine],
        [!a_keep.is_empty(), !b_keep.is_empty()],
    )?;

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
            finish_fallback(
                op,
                (body, &a_keep),
                red,
                decls,
                BooleanResultKind::OperandA,
                band,
                tol,
            )
        }
        (true, false) => {
            let body = carve_kept(&red.b, &b_keep)?;
            finish_fallback(
                op,
                (body, &b_keep),
                red,
                decls,
                BooleanResultKind::OperandB,
                band,
                tol,
            )
        }
        (false, false) => {
            let mut body = carve_kept(&red.a, &a_keep)?;
            let a_kept: BTreeSet<VertexKey> = body.vertices().map(|(v, _)| v).collect();
            let b_body = carve_kept(&red.b, &b_keep)?;
            if op == BooleanOp::Subtract {
                // Read here: the void door decides nothing, so its own
                // reading is structural.
                no_unjoined_line(&b_body, band)?;
            }
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
                        voids::VoidInsertError::Revert(source) => BooleanError::Revert {
                            operand: Operand::B,
                            source,
                        },
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
            let glued_pairs =
                GluedSurfaces::of_result(op, &body, a_pristine, b_pristine, decls, &graft);
            let mut desc = Descendants::default();
            let none = BTreeSet::new();
            let ledger = ledger(red, decls, [&none, &none])?;
            let views = [&KeyView::Direct, &KeyView::Graft(&graft)];
            let discarded = [
                faces_outside(&red.a, &a_keep),
                faces_outside(&red.b, &b_keep),
            ];
            let recorded =
                carried_vertices(&body, &ledger, views, &desc, [&discarded[0], &discarded[1]])?;
            let OutputStage {
                closed,
                merged,
                slivers,
                edge_joins,
                killed_lineage,
            } = finish_output(
                &mut body,
                &mut desc,
                &glued_pairs,
                (&mut SeamEdges::default(), &[]),
                &recorded,
                band,
                tol,
            )?;
            let output = output_substitutions(&closed, &merged, &slivers, &edge_joins)?;
            crate::pcurves::mint_pcurves(&mut body, tol)
                .map_err(|source| BooleanError::Pcurves { source })?;
            let (contacts, coincidences) = carry(
                &body,
                &ledger,
                views,
                &desc,
                [&discarded[0], &discarded[1]],
                band,
            )?;
            share_recorded_points(&mut body, &contacts, &coincidences, band)?;
            let (parted, contacts) = super::slit::part_riders(&mut body, &contacts, band, tol)?;
            if !parted.is_empty() {
                crate::pcurves::mint_pcurves(&mut body, tol)
                    .map_err(|source| BooleanError::Pcurves { source })?;
            }
            let body = gate(body, band, tol)?;
            let FallbackFates {
                substituted,
                uncut,
                mut discards,
                divisions: [_, face_fragments_b],
            } = fallback_fates(
                red,
                [&discarded[0], &discarded[1]],
                [&|v| a_kept.contains(&v).then_some(v), &|v| {
                    graft.vertices.get(v).copied()
                }],
                band,
                tol,
            )?;
            let (graft_vertices, graft_edges, graft_dead_edges, mut graft_faces) =
                graft_rows(&graft);
            let mut face_fragments_a =
                crate::lineage::FaceDivisions::new(crate::lineage::LineageRows::FaceDivisions);
            let mut cone_splits =
                crate::lineage::ConeCopies::new(crate::lineage::LineageRows::ConeCopies);
            let mut vertex_merges = Fusions::new(crate::lineage::LineageRows::Fusions);
            name_parted(
                &parted,
                &mut graft_faces,
                &mut face_fragments_a,
                &mut cone_splits,
                &mut vertex_merges,
                &mut discards,
            )?;
            let naming = BooleanNaming {
                a_keys: OperandKeys::Direct,
                b_keys: OperandKeys::Grafted,
                graft_vertices,
                graft_edges,
                graft_dead_edges,
                graft_faces,
                merge_groups: merge_rows(&merged, &output)?,
                substitutions: output.clone(),
                merge_skipped: merged.skipped.clone(),
                reduction_contacts: red.contacts.clone(),
                substituted,
                uncut,
                edge_classes: red.edge_classes.clone(),
                unread_partners: red.unread_partners.clone(),
                null_copies: super::null_copy_rows(&red.null_edges),
                edge_joins,
                killed_lineage,
                seam_edges: parted.iter().flat_map(|p| [p.kept, p.minted]).collect(),
                face_fragments_a,
                face_fragments_b,
                cone_splits,
                vertex_merges,
                discards,
                ..BooleanNaming::default()
            };
            Ok(BooleanResult::Body(BooleanBody {
                body,
                kind,
                contacts,
                naming,
                verdicts: GlueVerdict::of_rows(&coincidences, &red.tangent_lines),
                coincidences,
            }))
        }
    }
}

/// Every face of `clone` outside the shells `keep`: what a containment
/// fallback discards of that operand.
fn faces_outside<T: Real>(clone: &Body<T>, keep: &[ShellKey]) -> BTreeSet<FaceKey> {
    clone
        .shells()
        .filter(|(shell, _)| !keep.contains(shell))
        .flat_map(|(_, shell)| shell.faces.iter().copied())
        .collect()
}

/// What a containment fallback records of its coincident faces: each
/// substituted fragment, each discard, and each operand's face
/// divisions, A's then B's.
struct FallbackFates {
    substituted: Vec<super::SubstitutedFace>,
    uncut: Vec<super::UncutFace>,
    discards: Vec<super::DiscardRow>,
    divisions: [crate::lineage::FaceDivisions; 2],
}

/// **A containment fallback divides its dropped coincident faces as the
/// section path does** (`fate::cut`), once its result is built: it keeps
/// or drops whole shells, so a dropped face is a discard and its clone
/// is free to divide, and the cut changes only how the rows describe it.
/// `discarded` is each operand's faces outside the kept shells, A's then
/// B's; `kept` maps each operand's vertex to the result's, before the
/// output stage. A pair whose faces the result holds both or neither of
/// owes nothing and is not cut. Every other face of a discarded shell is
/// a discard (`discard::discard_row`): no face of its own operand across
/// it is kept, so it borders a kept face only through a substituted
/// fragment.
///
/// Each dropped face is cut on its own. Where its cut meets one of the
/// cut's placement limits (`fate::Limit`), that face alone
/// is restored as it stood and recorded as uncut
/// ([`super::UncutFace`]), with neither a substituted nor a discard row;
/// the result holds nothing of the dropped clone, so no body moves.
///
/// # Errors
///
/// Every other refusal of the cut, as it is: a key that no longer
/// resolves, a reading in band, an Euler operator's refusal. A pair
/// whose kept copy the result drops while it keeps the copy the lump
/// dropped, or a cut in operand A's clone (the lump keeps A's copy,
/// `tables::kept_copy`), refuses
/// [`BooleanError::ClassificationInvariant`]; a discard row's reading
/// (`discard::discard_row`) refuses as it does.
fn fallback_fates<T: Decide + AtRestPolicy>(
    red: &mut BooleanReduction<T>,
    discarded: [&BTreeSet<FaceKey>; 2],
    kept: [super::discard::InResult<'_>; 2],
    band: Band,
    tol: Tol,
) -> Result<FallbackFates, BooleanError> {
    let gone = |o: Operand, f: FaceKey| match o {
        Operand::A => discarded[0].contains(&f),
        Operand::B => discarded[1].contains(&f),
    };
    // Whether the pair is cut: its kept copy held, its dropped one not.
    let owed = |holder: Operand, kept: FaceKey, dropped: FaceKey| match (
        gone(holder, kept),
        gone(holder.other(), dropped),
    ) {
        (false, true) => Ok(true),
        (true, false) => Err(BooleanError::ClassificationInvariant {
            what: "a containment fallback drops the coincident copy the lump keeps",
        }),
        _ => Ok(false),
    };
    let mut edges = Vec::new();
    for fe in std::mem::take(&mut red.fates.edges) {
        if owed(fe.holder, fe.kept, fe.face)? {
            edges.push(fe);
        }
    }
    let across = std::mem::take(&mut red.fates.across);
    let mut corners = Vec::new();
    for fc in std::mem::take(&mut red.fates.corners) {
        if owed(fc.holder, fc.kept.face(), fc.dropped.face)? {
            corners.push(fc);
        }
    }
    if edges.iter().any(|fe| fe.holder == Operand::B)
        || corners.iter().any(|fc| fc.holder == Operand::B)
    {
        return Err(BooleanError::ClassificationInvariant {
            what: "a containment fallback cuts operand A's copy of a coincident pair",
        });
    }
    let fresh = || crate::lineage::FaceDivisions::new(crate::lineage::LineageRows::FaceDivisions);
    let mut divisions = [fresh(), fresh()];
    let mut cuts = super::fate::FateCuts::default();
    let mut uncut = Vec::new();
    let faces: BTreeSet<FaceKey> = edges
        .iter()
        .map(|fe| fe.face)
        .chain(corners.iter().map(|fc| fc.dropped.face))
        .collect();
    for &face in &faces {
        red.fates = super::fate::Fates {
            edges: edges.iter().filter(|fe| fe.face == face).copied().collect(),
            corners: corners
                .iter()
                .filter(|fc| fc.dropped.face == face)
                .copied()
                .collect(),
            across: across.iter().filter(|a| a.1 == face).copied().collect(),
        };
        // A face no fate edge lies in is held whole, and its cut divides
        // nothing; any other is kept as it stands, for a cut that meets a
        // placement limit part way.
        let before = (!red.fates.edges.is_empty()).then(|| (red.b.clone(), divisions[1].clone()));
        red.b.enter_surgery();
        let [div_a, div_b] = &mut divisions;
        let got = super::fate::cut(red, [div_a, div_b], band, tol);
        match (got, before) {
            (Ok(c), _) => {
                red.b.leave_surgery_and_sweep();
                cuts.substituted.extend(c.substituted);
                cuts.chords.extend(c.chords);
            }
            (Err(super::fate::CutError::Limit(limit)), Some((b, div_b))) => {
                red.b.leave_surgery();
                red.b = b;
                divisions[1] = div_b;
                let e = BooleanError::from(super::fate::CutError::Limit(limit));
                uncut.extend(red.fates.edges.iter().map(|fe| super::UncutFace {
                    operand: Operand::B,
                    face,
                    by: fe.kept,
                    kind: e.kind(),
                    what: e.to_string(),
                }));
            }
            (Err(e), _) => {
                red.b.leave_surgery();
                return Err(e.into());
            }
        }
    }
    red.fates = super::fate::Fates {
        edges,
        corners,
        across,
    };
    cuts.substituted.sort();
    cuts.substituted.dedup();
    uncut.sort_by_key(|u: &super::UncutFace| (u.operand, u.face, u.by));
    uncut.dedup();
    let refuse = |_: crate::entity::HalfEdgeKey,
                  _: VertexKey,
                  _: VertexKey|
     -> Result<(VertexKey, VertexKey), BooleanError> {
        Err(BooleanError::JoinDesync {
            what: "a containment fallback's discard borders a kept face of its own operand",
        })
    };
    let mut discards = Vec::new();
    for (operand, [own, theirs], div) in [
        (Operand::A, [kept[0], kept[1]], &divisions[0]),
        (Operand::B, [kept[1], kept[0]], &divisions[1]),
    ] {
        let own_to_result =
            |v: VertexKey| super::discard::own_in_result(red, operand, (own, theirs), v);
        let chords = super::discard::ChordsAcross {
            cuts: &cuts,
            to_result: theirs,
            own_to_result: &own_to_result,
        };
        let body = match operand {
            Operand::A => &red.a,
            Operand::B => &red.b,
        };
        for (face, _) in body.faces() {
            let root = div.root(face);
            let whole = uncut.iter().any(|u| (u.operand, u.face) == (operand, root));
            if gone(operand, root) && !whole && !cuts.is_substituted(operand, face) {
                discards.push(super::discard::discard_row(
                    body,
                    face,
                    operand,
                    &|_| false,
                    &refuse,
                    Some(&chords),
                )?);
            }
        }
    }
    Ok(FallbackFates {
        substituted: cuts.substituted,
        uncut,
        discards,
        divisions,
    })
}

/// Finishes a single-operand fallback result (the merge output stage
/// is a documented no-op on a maximal-faced operand but runs anyway —
/// the contract is uniform), applying ∖'s B-side revert when needed.
fn finish_fallback<T: Decide + Bounds + AtRestPolicy>(
    op: BooleanOp,
    (body, keep): (Body<T>, &[ShellKey]),
    red: &mut BooleanReduction<T>,
    decls: &Glue<T>,
    kind: BooleanResultKind,
    band: Band,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    let contacts = &red.contacts;
    let reduction_contacts = contacts.clone();
    let mut body = body;
    if kind == BooleanResultKind::OperandB && op == BooleanOp::Subtract {
        no_unjoined_line(&body, band)?;
        body = body.revert().map_err(|source| BooleanError::Revert {
            operand: Operand::B,
            source,
        })?;
    }
    // No cross-operand pair merges here: one operand is absent from the
    // result. A coincident pair that held the absent operand's region
    // through the kept one is a `substituted` row (`fallback_fates`); the
    // surviving operand's CARRIED records still apply.
    let mut desc = Descendants::default();
    let (a_view, b_view) = match kind {
        BooleanResultKind::OperandA => (KeyView::Direct, KeyView::Absent),
        _ => (KeyView::Absent, KeyView::Direct),
    };
    let none = BTreeSet::new();
    let ledger = ledger(red, decls, [&none, &none])?;
    let discarded = match kind {
        BooleanResultKind::OperandA => [faces_outside(&red.a, keep), faces_outside(&red.b, &[])],
        _ => [faces_outside(&red.a, &[]), faces_outside(&red.b, keep)],
    };
    let carved: BTreeSet<VertexKey> = body.vertices().map(|(v, _)| v).collect();
    let recorded = carried_vertices(
        &body,
        &ledger,
        [&a_view, &b_view],
        &desc,
        [&discarded[0], &discarded[1]],
    )?;
    let OutputStage {
        closed,
        merged,
        slivers,
        edge_joins,
        killed_lineage,
    } = finish_output(
        &mut body,
        &mut desc,
        &GluedSurfaces::none(),
        (&mut SeamEdges::default(), &[]),
        &recorded,
        band,
        tol,
    )?;
    let output = output_substitutions(&closed, &merged, &slivers, &edge_joins)?;
    crate::pcurves::mint_pcurves(&mut body, tol)
        .map_err(|source| BooleanError::Pcurves { source })?;
    let (contacts, coincidences) = carry(
        &body,
        &ledger,
        [&a_view, &b_view],
        &desc,
        [&discarded[0], &discarded[1]],
        band,
    )?;
    share_recorded_points(&mut body, &contacts, &coincidences, band)?;
    let body = gate(body, band, tol)?;
    let own = |v: VertexKey| carved.contains(&v).then_some(v);
    let absent = |_: VertexKey| None;
    let kept: [super::discard::InResult<'_>; 2] = match kind {
        BooleanResultKind::OperandA => [&own, &absent],
        _ => [&absent, &own],
    };
    let FallbackFates {
        substituted,
        uncut,
        discards,
        divisions: [face_fragments_a, face_fragments_b],
    } = fallback_fates(red, [&discarded[0], &discarded[1]], kept, band, tol)?;
    let naming = match kind {
        BooleanResultKind::OperandA => BooleanNaming {
            a_keys: OperandKeys::Direct,
            b_keys: OperandKeys::Absent,
            merge_groups: merge_rows(&merged, &output)?,
            substitutions: output.clone(),
            merge_skipped: merged.skipped.clone(),
            reduction_contacts: reduction_contacts.clone(),
            substituted,
            uncut,
            edge_classes: red.edge_classes.clone(),
            unread_partners: red.unread_partners.clone(),
            null_copies: super::null_copy_rows(&red.null_edges),
            edge_joins,
            killed_lineage,
            face_fragments_a,
            face_fragments_b,
            discards,
            ..BooleanNaming::default()
        },
        // The result arena IS the B clone: B keys direct, A absent.
        _ => BooleanNaming {
            a_keys: OperandKeys::Absent,
            b_keys: OperandKeys::Direct,
            merge_groups: merge_rows(&merged, &output)?,
            substitutions: output.clone(),
            merge_skipped: merged.skipped.clone(),
            reduction_contacts: reduction_contacts.clone(),
            substituted,
            uncut,
            edge_classes: red.edge_classes.clone(),
            unread_partners: red.unread_partners.clone(),
            null_copies: super::null_copy_rows(&red.null_edges),
            edge_joins,
            killed_lineage,
            face_fragments_a,
            face_fragments_b,
            discards,
            ..BooleanNaming::default()
        },
    };
    Ok(BooleanResult::Body(BooleanBody {
        body,
        kind,
        contacts,
        naming,
        verdicts: GlueVerdict::of_rows(&coincidences, &red.tangent_lines),
        coincidences,
    }))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    /// **The loop walk reads a carrier decided off its chart as torn, and
    /// one read in band at a pose as unread** (D3): only a definite miss
    /// is a contradiction the kernel holds; a read in band is never a
    /// defect, so W2 reads nothing of the face rather than claiming one.
    /// It reads through the one classifier and its one policy, so it
    /// agrees with the ending on every read: the test's residual exceeded
    /// and a poisoned margin are definite, an undecided read in band is
    /// not (review on PR 5844, H7).
    #[test]
    fn the_walk_tears_only_on_a_definite_incidence_miss() {
        use super::{WalkRefusal, walk_refusal};
        use geom_brep::{OffMargin, PcurveCertifyError};
        let off = |margin| crate::pcurves::PcurveMintError::Certify {
            half_edge: crate::HalfEdgeKey::default(),
            error: PcurveCertifyError::CarrierOffChart {
                chart: geom::SurfaceKind::Cone,
                carrier: geom::CurveKind::Circle,
                why: "a circle no cone holds here",
                margin,
            },
        };
        let m = geom_core::MarginDiag::value(4e-10);
        let poisoned = geom_core::MarginDiag::INVALID;
        for (margin, want) in [
            (None, WalkRefusal::Torn),
            (Some(OffMargin::Past(m)), WalkRefusal::Torn),
            (Some(OffMargin::InBand(m)), WalkRefusal::Unread),
            // A margin that could not be read is definite, as its ending.
            (Some(OffMargin::InBand(poisoned)), WalkRefusal::Torn),
        ] {
            assert_eq!(walk_refusal(&off(margin)), want, "{margin:?}");
        }
        // The rest of the classifier's reads, each by the same policy.
        let certify = |error| crate::pcurves::PcurveMintError::Certify {
            half_edge: crate::HalfEdgeKey::default(),
            error,
        };
        let undecided = |margin| PcurveCertifyError::Escalated {
            check: geom_brep::PcurveCheck::ChartIncidence,
            sample: 0,
            cause: geom_core::Indeterminate {
                margin,
                band: geom_core::Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("pcurve_cone_chart_incident"),
                terminal_sliver: false,
            },
        };
        for (error, want) in [
            (
                PcurveCertifyError::ResidualExceeded {
                    check: geom_brep::PcurveCheck::ChartIncidence,
                    sample: 0,
                },
                WalkRefusal::Torn,
            ),
            (
                undecided(geom_core::MarginDiag::value(5e-9)),
                WalkRefusal::Unread,
            ),
            (undecided(poisoned), WalkRefusal::Torn),
        ] {
            let read = geom_brep::IncidenceRead::of(&error).expect("an incidence refusal");
            assert_eq!(read.is_definite(), want == WalkRefusal::Torn, "{error:?}");
            assert_eq!(walk_refusal(&certify(error.clone())), want, "{error:?}");
        }
    }

    /// **A re-charted shell's cell reads as its input shell's faces**
    /// ([`super::respell_through_cuts`]): a meeting on a turned edge, and
    /// a vertex on a turned face, of a two-face shell are the meeting of
    /// both faces; a carrier pair on a turned face of a one-face shell
    /// keeps its relation on that face; a carrier pair on a two-face
    /// shell, and a meeting of more cells than a row holds, refuse typed.
    #[test]
    fn a_re_charted_cell_reads_as_its_input_shells_faces() {
        use super::{ChartCuts, Recharts, respell_through_cuts};
        use crate::boolean::{BooleanError, Cell, Operand};
        use crate::{Coincidence, Relation, RowCell};
        let key = |n: u64| slotmap::KeyData::from_ffi((1u64 << 32) | n);
        let face = |n| Cell::Face(key(n).into());
        let (plane, one, two, three, turned_edge, turned_face) = (1, 2, 3, 4, 5, 6);
        let a = |cell| RowCell::Input {
            input: Operand::A,
            cell,
        };
        let b = |cell| RowCell::Input {
            input: Operand::B,
            cell,
        };
        let row = |first: RowCell, second: RowCell, relation| Coincidence {
            cells: [first, second],
            more: [None; 3],
            relation,
            site: crate::DecisionSite::VertexFusion,
            margin: geom_core::MarginDiag::value(0.0),
            discharge: crate::Discharge::Numeric,
        };
        let turned = |faces: &[u64]| {
            let mut made = Recharts::default();
            made.add(
                [Cell::Edge(key(turned_edge).into()), face(turned_face)].into(),
                faces.iter().rev().map(|&n| key(n).into()).collect(),
            );
            made
        };
        let b_turned = |faces: &[u64]| [Recharts::default(), turned(faces)];
        let cuts = [ChartCuts::default(), ChartCuts::default()];
        let edge = b(Cell::Edge(key(turned_edge).into()));
        let vertex = a(Cell::Vertex(key(plane).into()));
        let mut rows = [
            row(a(face(plane)), edge, Relation::Meet),
            row(vertex, b(face(turned_face)), Relation::OnCarrier),
            row(a(face(plane)), b(face(two)), Relation::SameOpposite),
        ];
        respell_through_cuts(&mut rows, &cuts, &b_turned(&[one, two])).unwrap();
        assert_eq!(
            rows.map(|r| (r.cells, r.more, r.relation)),
            [
                (
                    [a(face(plane)), b(face(one))],
                    [Some(b(face(two))), None, None],
                    Relation::Meet
                ),
                (
                    [vertex, b(face(one))],
                    [Some(b(face(two))), None, None],
                    Relation::Meet
                ),
                (
                    [a(face(plane)), b(face(two))],
                    [None; 3],
                    Relation::SameOpposite
                ),
            ],
            "a point on a two-face shell's turned cell meets both faces; a cell no shell \
             turned is kept"
        );
        let mut whole = [row(
            a(face(plane)),
            b(face(turned_face)),
            Relation::SameOpposite,
        )];
        respell_through_cuts(&mut whole, &cuts, &b_turned(&[one])).unwrap();
        assert_eq!(
            (whole[0].cells, whole[0].relation),
            ([a(face(plane)), b(face(one))], Relation::SameOpposite),
            "a one-face shell holds its turned face whole, so the pair keeps its relation"
        );
        for relation in [Relation::SameOpposite, Relation::OnCarrier] {
            let mut split = [row(a(face(plane)), b(face(turned_face)), relation)];
            let refused = respell_through_cuts(&mut split, &cuts, &b_turned(&[one, two]));
            assert!(
                matches!(
                    &refused,
                    Err(BooleanError::RechartedRowUnstated { operand: Operand::B, relation: r, faces, .. })
                        if *r == relation && *faces == [key(one).into(), key(two).into()]
                ),
                "a {relation:?} face pair on a two-face shell's turned face refuses typed: \
                 {refused:?}"
            );
        }
        let mut crowded = [row(
            a(Cell::Edge(key(turned_edge).into())),
            edge,
            Relation::Meet,
        )];
        let both = [turned(&[one, two, three]), turned(&[one, two, three])];
        let refused = respell_through_cuts(&mut crowded, &cuts, &both);
        assert!(
            matches!(
                &refused,
                Err(BooleanError::RechartedRowUnstated { operand: Operand::A, faces, .. })
                    if faces.len() == 3
            ),
            "six faces are more than a row holds: {refused:?}"
        );
    }

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
    fn found<T: geom_core::Decide + crate::props::AtRestPolicy>(
        body: &crate::Body<T>,
        contacts: &crate::boolean::ReductionContacts,
        views: [&super::KeyView<'_>; 2],
        desc: &super::Descendants,
    ) -> Result<crate::boolean::ContactRecords, crate::boolean::BooleanError> {
        let none = super::Rows::default();
        let rows = super::tagged(contacts);
        super::carry_rows(
            body,
            &rows,
            [&none, &none],
            views,
            (
                desc,
                &Default::default(),
                [&Default::default(), &Default::default()],
            ),
            false,
            band(),
        )
    }

    /// The door over carried records alone.
    fn carried_in<T: geom_core::Decide + crate::props::AtRestPolicy>(
        body: &crate::Body<T>,
        decls: &crate::boolean::Glue<T>,
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
            (
                desc,
                &Default::default(),
                [&Default::default(), &Default::default()],
            ),
            false,
            band(),
        )
    }

    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{gate, seam_class, seam_must_carry};

    /// **A merge group's kept face is never one the output rows retire,
    /// in either order the groups come in.** `(K1, [F])` then
    /// `(K2, [K1])` is a chain the substitution list takes (`F` onto
    /// `K1`, `K1` onto `K2`), so the kept-face check is what refuses it.
    /// The other order is refused by the list itself: `F` onto `K1`
    /// names the face the first row retired. Red if the kept-face check
    /// is dropped.
    #[test]
    fn a_kept_face_another_group_absorbs_refuses_in_either_order() {
        use super::{merge_rows, output_substitutions};
        use crate::boolean::Cell;
        use crate::entity::FaceKey;
        use crate::lineage::{LineageRows, RowDefect};
        use crate::merge_faces::{MergeCoplanarOutcome, MergedGroup};
        let key = |n: u64| slotmap::KeyData::from_ffi((1u64 << 32) | n);
        let (k1, k2, f): (FaceKey, FaceKey, FaceKey) =
            (key(1).into(), key(2).into(), key(3).into());
        let group = |kept, absorbed: Vec<FaceKey>| MergedGroup {
            kept,
            absorbed,
            killed_edges: Vec::new(),
            rings_made: Vec::new(),
            killed_vertices: Vec::new(),
        };
        let refused_at = |r: Result<(), BooleanError>| match r {
            Err(BooleanError::LineageRow {
                rows: LineageRows::Substitutions,
                defect: RowDefect::NamesRetired,
                key: Cell::Face(k),
            }) => k,
            other => panic!("a kept face absorbed must refuse, got {other:?}"),
        };

        let chained = MergeCoplanarOutcome {
            groups: vec![group(k1, vec![f]), group(k2, vec![k1])],
            ..MergeCoplanarOutcome::default()
        };
        let rows = output_substitutions(&[], &chained, &[], &[]).expect("a chain is no cycle");
        assert_eq!(
            refused_at(merge_rows(&chained, &rows).map(|_| ())),
            k1,
            "K1 kept, then absorbed"
        );

        let reversed = MergeCoplanarOutcome {
            groups: vec![group(k2, vec![k1]), group(k1, vec![f])],
            ..MergeCoplanarOutcome::default()
        };
        assert_eq!(
            refused_at(output_substitutions(&[], &reversed, &[], &[]).map(|_| ())),
            k1,
            "F onto K1, which the first row retired"
        );

        let sound = MergeCoplanarOutcome {
            groups: vec![group(k1, vec![f])],
            ..MergeCoplanarOutcome::default()
        };
        let rows = output_substitutions(&[], &sound, &[], &[]).unwrap();
        assert_eq!(merge_rows(&sound, &rows).unwrap(), [(k1, vec![f])]);
    }

    /// **A cycle in either chase's rows refuses where the row is added,
    /// and both chases end.** The descendant map's substitution row back
    /// onto a retired face refuses at append. The naming's output rows
    /// take the joins too, so a join-only cycle refuses there:
    /// `e0` joined into `e1`, `e2` into `e0`, then `e1` into `e2` refuses
    /// at the second join, whose kept edge `e0` the first retired. What
    /// is left of the settler's chase is a dead end, here, since nothing
    /// is live in an empty body.
    #[test]
    fn a_cycle_refuses_at_append_in_both_chases_and_each_ends() {
        use super::{BooleanNaming, Descendants, KeyView, SettleError, output_substitutions};
        use crate::boolean::{Cell, EdgeJoin};
        use crate::entity::{EdgeKey, FaceKey, VertexKey};
        use crate::lineage::{LineageRows, RowDefect};
        let key = |n: u64| slotmap::KeyData::from_ffi((1u64 << 32) | n);
        let (u, v, w): (VertexKey, VertexKey, VertexKey) =
            (key(1).into(), key(2).into(), key(7).into());
        let (a, b): (FaceKey, FaceKey) = (key(3).into(), key(4).into());
        let (e0, e1, e2): (EdgeKey, EdgeKey, EdgeKey) =
            (key(5).into(), key(6).into(), key(8).into());
        let body = crate::Body::<f64>::new();

        let mut desc = Descendants::default();
        desc.substitute((Cell::Face(a), Cell::Face(b))).unwrap();
        assert!(matches!(
            desc.substitute((Cell::Face(b), Cell::Face(a))),
            Err(BooleanError::LineageRow { .. })
        ));
        assert_eq!(desc.chained(&body, &KeyView::Direct, Cell::Face(a)), None);

        let join = |vertex, gone, kept| EdgeJoin {
            vertex,
            gone,
            kept,
            conventional: None,
        };
        let merged = crate::merge_faces::MergeCoplanarOutcome::default();
        let cycle = [join(u, e0, e1), join(v, e2, e0), join(w, e1, e2)];
        assert!(
            matches!(
                output_substitutions(&[], &merged, &[], &cycle),
                Err(BooleanError::LineageRow {
                    rows: LineageRows::Substitutions,
                    defect: RowDefect::NamesRetired,
                    key: Cell::Edge(k),
                }) if k == e0
            ),
            "the second join keeps the edge the first retired"
        );

        let joins = [join(u, e0, e1)];
        let naming = BooleanNaming {
            substitutions: output_substitutions(&[], &merged, &[], &joins).unwrap(),
            edge_joins: joins.to_vec(),
            ..BooleanNaming::default()
        };
        let settler = naming.settler(&body);
        assert_eq!(
            settler.settle(u),
            Err(SettleError::Dead {
                vertex: u,
                reached: Cell::Edge(e1),
            }),
            "a joined vertex is chased to its kept edge, dead in an empty body"
        );
        assert_eq!(
            settler.settle(w),
            Err(SettleError::Dead {
                vertex: w,
                reached: Cell::Vertex(w),
            })
        );
    }
    use crate::boolean::{BooleanDecision, BooleanError, LeverArm};
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
                .revert()
                .unwrap();
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

    /// **The gate's findings take their arm** (D10, Booleans): readings
    /// in band alone — an aimed ray's pair, read for a shell's role or a
    /// solid's orientation — are the operands' ill-conditioning, refused
    /// `Escalated` on the reading whose margin binds the offer (nearest
    /// zero, in either order) and naming how many more lie in band. A ray
    /// set aside at a sharp corner is never escalated as a pair. A pair
    /// beside a definite finding, and a volume whose certified enclosure
    /// holds zero (a solid's or a shell's, after a corner or not), are the
    /// kernel's.
    #[test]
    fn the_gate_types_only_readings_in_band_escalated() {
        use crate::ShellClassifyError as S;
        use crate::ValidationError as V;
        use crate::entity::{FaceKey, ShellKey, SolidKey};
        use geom_core::{Band, Indeterminate, MarginDiag};
        let band = Band::new(1e-9, 1e-8).unwrap();
        let gap = |m| Indeterminate {
            margin: MarginDiag::value(m),
            band,
            predicate: Some("side_at_infinity_order"),
            terminal_sliver: false,
        };
        let shells: Vec<ShellKey> = (1..=3)
            .map(|n: u64| slotmap::KeyData::from_ffi((1 << 32) | n).into())
            .collect();
        // Each shell's pair is told apart by its near face.
        let near = |shell: ShellKey| -> FaceKey {
            slotmap::KeyData::from_ffi(slotmap::Key::data(&shell).as_ffi()).into()
        };
        let pair = |shell: ShellKey, m| crate::AimedPair {
            shell,
            aimed: FaceKey::default(),
            aim: "(0.5, 0.5, 1.0)".to_owned(),
            near: near(shell),
            far: FaceKey::default(),
            gap: gap(m),
        };
        let (thin, thick) = (pair(shells[0], 3.3e-9), pair(shells[1], 6.6e-9));
        let role = |pair: &crate::AimedPair| V::ShellRoleUndecided {
            solid: SolidKey::default(),
            error: S::RayInBand {
                shell: pair.shell,
                pair: Box::new(pair.clone()),
            },
        };
        let orientation = |pair: &crate::AimedPair| V::OrientationUndecided {
            solid: SolidKey::default(),
            reading: crate::OrientationUnread::Pair(Box::new(pair.clone())),
        };
        // A solid no aimed ray read, one ray set aside at a sharp corner,
        // and its volume's enclosure holding zero (`margin`, an end
        // escalated) or straddling it (`None`).
        let corner = |margin: Option<f64>| V::OrientationUndecided {
            solid: SolidKey::default(),
            reading: crate::OrientationUnread::Volume {
                margin: margin.map(gap),
                aimed: 1,
                set_aside: vec![crate::side_reader::SetAside {
                    aimed: FaceKey::default(),
                    aim: Some("(0.5, 0.5, 1.0)".to_owned()),
                    why: crate::side_reader::SetAsideWhy::Corner {
                        faces: [FaceKey::default(); 2],
                        gap: gap(1e-9),
                    },
                }],
            },
        };
        // A shell no aimed ray read, its volume's enclosure holding zero.
        let thin_volume = V::ShellRoleUndecided {
            solid: SolidKey::default(),
            error: S::Escalated {
                shell: shells[2],
                source: gap(5e-9),
            },
        };
        // A shell whose volume's bracket straddles zero: no reading in
        // band, the needle class's fork.
        let straddling_volume = V::ShellRoleUndecided {
            solid: SolidKey::default(),
            error: S::Straddles {
                shell: shells[2],
                low: geom_core::MarginDiag::value(-1e-18),
                high: geom_core::MarginDiag::value(1e-18),
            },
        };
        let definite = V::NegativeVolume {
            solid: SolidKey::default(),
        };
        for (what, errors) in [
            ("thin first", vec![role(&thin), role(&thick)]),
            ("thick first", vec![role(&thick), role(&thin)]),
            (
                "an orientation pair",
                vec![role(&thick), orientation(&thin)],
            ),
        ] {
            let refusal = super::finished_body_refusal(errors);
            assert!(
                matches!(
                    &refusal,
                    BooleanError::Escalated {
                        decision: BooleanDecision::ShellRole { near: n, others: 1, .. },
                        diag,
                    } if *n == near(shells[0]) && *diag == thin.gap
                ),
                "{what}: the thinner pair binds, the other counted: {refusal:?}"
            );
        }
        // A volume where no ray read is never in band: its sign is
        // certified by its enclosure alone (D10), so an enclosure holding
        // zero — a solid's after a corner, a shell's — is the kernel's,
        // beside a pair too.
        for (what, errors) in [
            ("a solid's enclosure holding zero", vec![corner(Some(2e-9))]),
            ("beside a pair", vec![role(&thick), corner(Some(2e-9))]),
            (
                "a shell's enclosure holding zero",
                vec![thin_volume.clone()],
            ),
            ("beside a thinner pair", vec![thin_volume, role(&thin)]),
            ("a volume straddling zero", vec![straddling_volume.clone()]),
            ("beside one", vec![role(&thin), straddling_volume]),
            ("beside a definite finding", vec![role(&thin), definite]),
            ("a corner and a volume straddling zero", vec![corner(None)]),
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

        // Door 2 — the fallback's own re-gate, at the mechanism: a
        // fallback entry carrying a NURBS face whose box may meet a face
        // of the other operand refuses BEFORE a vertex is probed. A
        // placeholder's net is poison, so its box meets every face, far
        // or not. `sweep`'s `s16_box_soundness` pins the re-gate end to
        // end on a lofted operand.
        let band = Band::linear(Tol::witness()).unwrap();
        let Err(err) = super::sphere_extent_scan(&a, &b, &super::Evidence::NONE, band) else {
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
            ff: vec![],
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
        desc.substitute((
            crate::boolean::Cell::Face(dead_face),
            crate::boolean::Cell::Face(live_face),
        ))
        .unwrap();
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
            ff: vec![],
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
            ..Glue::none()
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
        let mut later = super::Fusions::new(crate::lineage::LineageRows::Fusions);
        later.push((c, a)).unwrap();
        assert!(
            matches!(
                corrupt.absorb_fusions(&later),
                Err(BooleanError::LineageRow {
                    rows: crate::lineage::LineageRows::Fusions,
                    defect: crate::lineage::RowDefect::NamesRetired,
                    ..
                })
            ),
            "absorbing a row that keeps the killed a refuses"
        );
    }

    /// **An absorption row that would close a cycle refuses where it is
    /// added; a chain that ends at a dead key drops the record.** The
    /// second row names `d0`, which the first retired, so the list never
    /// holds a cycle for a chase to meet. A dead end is no corrupt
    /// record: the record left the result with its face.
    #[test]
    fn an_absorption_row_closing_a_cycle_refuses_at_append_where_a_dead_end_drops() {
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
        dead_end
            .substitute((
                crate::boolean::Cell::Face(d0),
                crate::boolean::Cell::Face(d1),
            ))
            .unwrap();
        let dropped = remap(&dead_end).expect("a chain that ends is not corrupt");
        assert!(dropped.is_empty(), "dead end: {dropped:?}");

        let mut cycle = Descendants::default();
        cycle
            .substitute((
                crate::boolean::Cell::Face(d0),
                crate::boolean::Cell::Face(d1),
            ))
            .unwrap();
        assert!(
            matches!(
                cycle.substitute((
                    crate::boolean::Cell::Face(d1),
                    crate::boolean::Cell::Face(d0)
                )),
                Err(BooleanError::LineageRow {
                    rows: crate::lineage::LineageRows::Substitutions,
                    defect: crate::lineage::RowDefect::NamesRetired,
                    key: crate::boolean::Cell::Face(k),
                }) if k == d0
            ),
            "the row back onto a retired face refuses"
        );
        assert_eq!(cycle.cells, dead_end.cells, "the list is left as it was");

        // The carried lane reads the same chase.
        let decls = Glue {
            records_a: OperandRecords {
                vf: vec![CarriedRecord {
                    contact: VfContact { vertex, face: d0 },
                    record: 0,
                }],
                ..OperandRecords::default()
            },
            ..Glue::none()
        };
        let carry = |desc: &Descendants| carried_in(&body, &decls, [&KeyView::Direct; 2], desc);
        let carried = carry(&dead_end).expect("a chain that ends is not corrupt");
        assert!(carried.is_empty(), "carried dead end: {carried:?}");
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
        use crate::construction::{MefSite, MevSite};
        use crate::entity::VertexKey;
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
        desc.absorb_merge(&merged)
            .expect("the merge's rows are a lineage");
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
            ff: vec![],
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
    ) -> [Result<super::SeamReading, super::SeamRefusal>; 2] {
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

    /// A seam refusal undecided at a station, worded as the describe pass
    /// words it ([`super::seam_refusal`]); any other answer, as read.
    fn worded(
        got: Result<super::SeamReading, super::SeamRefusal>,
    ) -> Result<super::SeamReading, Result<BooleanError, super::SeamRefusal>> {
        got.map_err(|refusal| match refusal {
            super::SeamRefusal::Undecided(reading, diag) => Ok(super::seam_refusal(reading, diag)),
            mixed @ super::SeamRefusal::Mixed(_) => Err(mixed),
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
            match worded(got) {
                Err(Ok(BooleanError::Escalated {
                    decision: BooleanDecision::SeamJet,
                    diag,
                })) => assert_eq!(
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
            let got = worded(seam_must_carry(a, b, &ruling, 0.0, 1.0, extent, band()));
            assert!(
                matches!(
                    got,
                    Err(Ok(BooleanError::Escalated {
                        decision: BooleanDecision::LeverArm(LeverArm::Seam),
                        ..
                    }))
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
                matches!(got, Ok(super::SeamReading::Intrinsic)),
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
                matches!(got, Ok(super::SeamReading::Conventional)),
                "order {order}: coplanar planes keep the conventional chord, got {got:?}"
            );
        }
    }

    /// **A corner at every station maps to the seam's typed refusal, with
    /// no stations.** What this row shows is the mapping: the floor and a
    /// plane through its ruling at 45° read a corner at every station
    /// (and at the witness too, so the describe pass's smooth arm would
    /// not ask these surfaces). The arm asks the rule of a seam it read
    /// smooth at the witness, which is not a station on every carrier (a
    /// line's is its vertices' midpoint); where such a seam reads a corner
    /// at every station, this is the refusal it gets, as the
    /// smooth-premise doors refuse it, rather than a panic.
    #[test]
    fn a_corner_at_every_station_maps_to_the_seams_typed_refusal() {
        let (floor, _, ruling) = resting();
        let s = std::f64::consts::FRAC_1_SQRT_2;
        let tilted = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(s, 0.0, s),
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        };
        for (a, b) in [(&floor, &tilted), (&tilted, &floor)] {
            let got = seam_must_carry(a, b, &ruling, 0.0, 1.0, 1.0, band());
            assert!(
                matches!(got, Err(super::SeamRefusal::Mixed(None))),
                "a corner at every station refuses typed, got {got:?}"
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

    /// **A seam smooth at its witness and a corner at a station refuses
    /// as a mixed seam, in both orders**, on [`villarceau`]'s arc a
    /// radian either side of the tangency. Each station's jet read off a
    /// smooth join measures a transverse direction tangent to the first
    /// surface alone, so the per-station sagitta reads definitely
    /// positive in both orders here, at values that differ with the
    /// order; the rule reads the corner first-order instead, and the
    /// stations disagreeing bracket a point no station read in band, so
    /// the seam is certifiable as neither (D2).
    #[test]
    fn a_seam_smooth_at_its_witness_and_a_corner_at_a_station_refuses_as_mixed() {
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
                geom_brep::MustCarryVerdict::Mixed(geom_brep::MixedStations {
                    transverse: 3,
                    smooth: 4
                }),
                "the stations beside the tangency read a corner, the one at it smooth"
            );
        }
        for (order, got) in both_orders(&torus, &bitangent, &arc, span, 1.0)
            .into_iter()
            .enumerate()
        {
            assert!(
                matches!(
                    got,
                    Err(super::SeamRefusal::Mixed(Some(geom_brep::MixedStations {
                        transverse: 3,
                        smooth: 4
                    })))
                ),
                "order {order}: a seam that is a corner somewhere refuses as mixed, got {got:?}"
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
                    worded(got),
                    Err(Ok(BooleanError::Escalated {
                        decision: BooleanDecision::SeamWedge,
                        ..
                    }))
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
    /// door and handed through `BooleanError`'s
    /// `From<MergeCoplanarError>`, is the very `BooleanError` the
    /// boolean's own description pass raises for it.
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
        let text = BooleanError::from(crate::merge_faces::MergeCoplanarError::of_kept_boundary(
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
            EdgeDescribeFailure::Chart,
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
                format!("{:?}", BooleanError::from(worded)),
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
                format!("{:?}", BooleanError::from(merged.clone())),
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
                crate::construction::FaceSurface::New {
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
                let (r, _) = sphere_extent_scan(&a, &b, &Evidence::NONE, band).unwrap();
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
            |a| {
                sphere_extent_scan(a, &b, &Evidence::NONE, band)
                    .map(|(r, _)| r.rechart.len() + r.cut_in.len())
            },
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
        use strum::IntoEnumIterator;
        for refusal in SectionRefusal::iter().filter(|r| *r != SectionRefusal::Loop) {
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
    use super::overlap_reach;
    use crate::revalue_pin::S;

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

/// **Rows for [`place_found`] on a divided face**, each through
/// [`carry_in_place`] on a 2×2×1 prism whose top face a diagonal chord
/// divides, with the division logged as a fragment row. The record's
/// vertex is a lone vertex ([`Body::mvfs`](crate::construction::Construct::mvfs)) set where the row needs it,
/// so it bounds no piece and only the placement decides.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod found_on_a_divided_face {
    use super::{Cell, Descendants, carry_in_place};
    use crate::Body;
    use crate::boolean::{BooleanError, Contact, ContactKind, ContactRecords};
    use crate::construction::{MefSite, MevSite};
    use crate::entity::{FaceKey, VertexKey};
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
        let mut divisions =
            crate::lineage::FaceDivisions::new(crate::lineage::LineageRows::FaceDivisions);
        for row in rows {
            divisions.push(row).unwrap();
        }
        let desc = Descendants::default().with_fragments([divisions.clone(), divisions]);
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
    /// piece's interior: the record lands on the chord, which the
    /// division minted between the pieces. Red when the placement reads
    /// the pieces alone (the record drops) or counts a piece's boundary
    /// as holding (two pieces would hold it).
    #[test]
    fn a_rest_on_the_dividing_chord_lands_on_the_chord() {
        let (mut body, top, other) = divided();
        let v = lone(&mut body, 1.0, 1.0, 1.0);
        let chord = body
            .edges()
            .map(|(e, _)| e)
            .find(|&e| {
                crate::readback::edge_sides(&body, e).is_ok_and(|s| {
                    let (f0, f1) = s.faces();
                    [f0, f1] == [top, other] || [f1, f0] == [top, other]
                })
            })
            .expect("the chord between the pieces");
        assert_eq!(
            carried(&body, v, top, vec![(other, top)]).unwrap(),
            [ContactKind::VertexEdge(v, chord)]
        );
    }

    /// A vertex whose margin from the top's plane is in band cannot be
    /// read by the census's confirm, and the placement refuses it,
    /// naming the pair. Red when an in-band margin is read as off the
    /// plane, or as on it.
    #[test]
    fn a_rest_in_band_of_the_plane_refuses_unread() {
        let (mut body, top, other) = divided();
        let v = lone(&mut body, 1.5, 0.5, 1.0 + 3.0 * band().zero());
        let got = carried(&body, v, top, vec![(other, top)]);
        assert!(
            matches!(
                got,
                Err(BooleanError::CarryRefused {
                    refusal: crate::CarryRefusal::Unread {
                        cells: [Cell::Vertex(u), Cell::Face(_)],
                        ..
                    },
                    ..
                }) if u == v
            ),
            "{got:?}"
        );
    }

    /// A piece the census cannot read (a lone-vertex ring, which bounds
    /// no region) may hold the rest: the placement refuses, naming the
    /// pair. Red when an unread piece is read as not holding.
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
            Err(BooleanError::CarryRefused {
                refusal: crate::CarryRefusal::Unread { cells, .. },
                ..
            }) => {
                assert_eq!(
                    cells,
                    [Cell::Vertex(v), Cell::Face(other)],
                    "the refusal names the unread piece"
                );
            }
            got => panic!("{got:?}"),
        }
    }

    /// A vertex a later stage took, as the join takes one: the record's
    /// vertex is killed, and its interior is held now by an edge that
    /// lies in the walled-off piece (the bottom edge of a small prism
    /// standing on that piece, grafted in as a second shell). The record
    /// lands on the edge and the piece it lies in, not on the key piece.
    /// Red when the face of a taken vertex's row is read by its key.
    #[test]
    fn a_taken_vertexs_row_lands_on_the_piece_its_edge_lies_in() {
        let tol = Tol::witness();
        let (mut body, top, other) = divided();
        assert!(
            holds(&body, other, 1.5, 0.5),
            "the walled-off piece holds the x > y half"
        );
        let stand = crate::test_support_fixtures::prism_z::<f64>(
            &[(1.2, 0.2), (1.8, 0.2), (1.8, 0.8)],
            1.0,
            1.5,
            tol,
        );
        let solid = body.solids().next().unwrap().0;
        crate::boolean::combine::graft_solids_disjoint(&mut body, &[solid], &stand.body).unwrap();
        let at = |p: Point3<f64>, x: f64, y: f64| (p.x, p.y, p.z) == (x, y, 1.0);
        let edge = body
            .edges()
            .map(|(e, _)| e)
            .find(|&e| {
                let d = body.get_edge(e).unwrap();
                let ends = [d.he_plus, d.he_minus].map(|h| {
                    let v = body.get_half_edge(h).unwrap().start;
                    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
                });
                (at(ends[0], 1.2, 0.2) && at(ends[1], 1.8, 0.2))
                    || (at(ends[1], 1.2, 0.2) && at(ends[0], 1.8, 0.2))
            })
            .expect("the stand's edge along y = 0.2 on the top's plane");
        let taken = body.mvfs(Point3::new(1.5, 0.2, 1.0), true).unwrap();
        body.kvfs(taken.solid).unwrap();
        let records: ContactRecords = [crate::Cited::new(
            Contact::of(Cell::Vertex(taken.vertex), Cell::Face(top)).unwrap(),
            crate::Cites::decided(0),
        )]
        .into_iter()
        .collect();
        let mut divisions =
            crate::lineage::FaceDivisions::new(crate::lineage::LineageRows::FaceDivisions);
        divisions.push((other, top)).unwrap();
        let mut desc = Descendants::default();
        desc.substitute((Cell::Vertex(taken.vertex), Cell::Edge(edge)))
            .unwrap();
        let desc = desc.with_fragments([divisions.clone(), divisions]);
        assert_eq!(
            carry_in_place(&body, &records, &desc, band())
                .unwrap()
                .kinds()
                .collect::<Vec<_>>(),
            [ContactKind::EdgeFace(edge, other)]
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod spline_seam_rows {
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Point3, Tol};

    use super::{DescribeRefusal, seam_face, seam_reading};
    use crate::{Body, EdgeKey, FaceKey};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn spline((ku, kv): (KnotVector, KnotVector), control: Vec<Point3<f64>>) -> geom::Surface<f64> {
        let n = control.len();
        geom::Surface::Nurbs(std::sync::Arc::new(
            geom::NurbsSurface::new(ku, kv, control, vec![1.0; n]).unwrap(),
        ))
    }

    fn linear() -> KnotVector {
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
    }

    fn quadratic() -> KnotVector {
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
    }

    /// The bilinear chart through the four corners, `u` from `o` along
    /// `du`, `v` along `dv`.
    fn flat(o: [f64; 3], du: [f64; 3], dv: [f64; 3]) -> geom::Surface<f64> {
        let at = |a: f64, b: f64| {
            Point3::new(
                o[0] + a * du[0] + b * dv[0],
                o[1] + a * du[1] + b * dv[1],
                o[2] + a * du[2] + b * dv[2],
            )
        };
        let control = vec![at(0.0, 0.0), at(0.0, 1.0), at(1.0, 0.0), at(1.0, 1.0)];
        spline((linear(), linear()), control)
    }

    /// The flat top `z = 1`, `u` along `x` and `v` along `y`.
    fn flat_top() -> geom::Surface<f64> {
        flat([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
    }

    /// A biquadratic top on the same unit square, its boundary rows the
    /// square's sides, whose middle control point drops under the `y = 0`
    /// side: along that side it leaves flat at the two corners and turns
    /// to stand vertical at the middle.
    fn turning_top() -> geom::Surface<f64> {
        let mut control = Vec::new();
        for (i, x) in [0.0, 0.5, 1.0].into_iter().enumerate() {
            for (j, y) in [0.0, 0.5, 1.0].into_iter().enumerate() {
                control.push(if (i, j) == (1, 1) {
                    Point3::new(x, -0.5, 1.4)
                } else {
                    Point3::new(x, y, 1.0)
                });
            }
        }
        spline((quadratic(), quadratic()), control)
    }

    /// The unit cube with its top on `top` and its `y = 0` side on a flat
    /// spline chart, and the two faces and the edge they share.
    fn two_spline_faces(top: geom::Surface<f64>) -> (Body<f64>, [FaceKey; 2], EdgeKey) {
        let cube = crate::test_support_fixtures::prism::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            1.0,
            Tol::witness(),
        );
        let mut body = cube.body;
        let side = cube.side_faces[0];
        for (face, surface) in [
            (cube.top_face, top),
            (
                side,
                flat([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            ),
        ] {
            body.set_face_surface_unvouched_for_tests(
                face,
                crate::euler::FaceSurface::New {
                    surface,
                    sense: true,
                },
            )
            .unwrap();
        }
        let faces = [cube.top_face, side];
        let edge = body
            .edges()
            .map(|(e, _)| e)
            .find(|&e| {
                let sides = crate::readback::edge_sides_of(&body, e, body.get_edge(e).unwrap());
                let (a, b) = sides.faces();
                faces.contains(&a) && faces.contains(&b) && a != b
            })
            .expect("the top's edge on the side");
        (body, faces, edge)
    }

    /// The seam reading of `edge`, each face read as `describe_edges`
    /// reads it, its curve read through `curve`.
    fn reading(
        body: &Body<f64>,
        edge: EdgeKey,
    ) -> Result<geom_brep::DihedralClass, geom_brep::SeamEscalation> {
        let sides = crate::readback::edge_sides_of(body, edge, body.get_edge(edge).unwrap());
        let surface = |side: crate::readback::EdgeSide| body.get_surface(side.surface).unwrap();
        let curve = body.stored_curve(edge).unwrap();
        let face = |side| seam_face(body, edge, side, surface(side), Some(curve), band()).unwrap();
        seam_reading(
            [face(sides.plus), face(sides.minus)],
            curve.mid_point(),
            1.0,
            band(),
        )
    }

    /// **A seam between two spline faces reads its dihedral**: each
    /// face's normal is its chart's at the edge's pcurve on it, so the
    /// cube's top edge between two flat spline faces reads transverse,
    /// where the two carriers' implicit forms read no arm.
    #[test]
    fn a_seam_between_two_spline_faces_reads_transverse() {
        let (body, _, edge) = two_spline_faces(flat_top());
        assert_eq!(
            reading(&body, edge),
            Ok(geom_brep::DihedralClass::Transverse),
            "the spline seam"
        );
        let sides = crate::readback::edge_sides_of(&body, edge, body.get_edge(edge).unwrap());
        let surface = |side: crate::readback::EdgeSide| body.get_surface(side.surface).unwrap();
        let analytic = geom_brep::classify_dihedral(
            surface(sides.plus),
            surface(sides.minus),
            body.stored_curve(edge).unwrap().mid_point(),
            1.0,
            band(),
        );
        assert!(
            matches!(analytic, Err(e) if e.diag().predicate == Some("dihedral_arm")),
            "the carriers' implicit forms: {analytic:?}"
        );
    }

    /// **The normal is read at the witness's own parameter**: along the
    /// `y = 0` side the turning top leaves flat at the corners, a corner
    /// against the side, and stands vertical at the middle, tangent to it.
    /// The seam reads smooth at the witness, the middle of the edge's
    /// interval; read at either end of the interval, it would read
    /// transverse.
    #[test]
    fn the_seam_is_read_at_its_witness_on_a_turning_chart() {
        let (body, _, edge) = two_spline_faces(turning_top());
        assert_eq!(
            reading(&body, edge),
            Ok(geom_brep::DihedralClass::Smooth),
            "tangent at the witness"
        );
        let top = turning_top();
        let chart = top.spline_chart().unwrap();
        let normal = |u: f64| {
            let j = chart.ders(u, 0.0);
            j.du.cross(j.dv).normalize()
        };
        assert!(
            normal(0.0).z > 0.9 && normal(1.0).z > 0.9,
            "flat at the corners: {:?} {:?}",
            normal(0.0),
            normal(1.0)
        );
    }

    /// **A null edge is refused unplaced**: a null edge minted at the
    /// cube's corner between the two spline faces has no carrier and no
    /// image to read either face at, and the describer says so rather than
    /// placing it. Placed, it would stop at its arm, its zero-length chord
    /// giving the wedge no reach.
    #[test]
    fn a_null_edge_between_two_spline_faces_is_refused_unplaced() {
        let (mut body, [top, side], edge) = two_spline_faces(flat_top());
        let he_of = |body: &Body<f64>, face: FaceKey| {
            let e = body.get_edge(edge).unwrap();
            [e.he_plus, e.he_minus]
                .into_iter()
                .find(|&h| {
                    let l = body.get_half_edge(h).unwrap().parent_loop;
                    body.get_loop(l).unwrap().face == face
                })
                .unwrap()
        };
        let (on_top, on_side) = (he_of(&body, top), he_of(&body, side));
        let corner = body.get_half_edge(on_top).unwrap().start;
        let leaving = |body: &Body<f64>, he: crate::entity::HalfEdgeKey| {
            let h = body.get_half_edge(he).unwrap();
            if h.start == corner { he } else { h.next }
        };
        let (he1, he2) = (leaving(&body, on_top), leaving(&body, on_side));
        let null = body
            .mev_null(
                crate::euler::MevSite::Fan { he1, he2 },
                crate::null::NewVertexSide::Above,
            )
            .unwrap();
        let got =
            crate::boolean::describe_edges(&mut body, [null.edge], &[], band(), Tol::witness());
        assert_eq!(
            got,
            Err(DescribeRefusal::failed(
                null.edge,
                crate::merge_faces::EdgeDescribeFailure::Unplaced
            )),
            "the null edge"
        );
    }
}
