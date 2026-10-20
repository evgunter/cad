//! The tier-3′ **global coincidence census** + contact-record
//! certification (F1/F2) — the injectivity pass tier 3 defers, run
//! at rest against a body's contact records.
//!
//! **Sweep shape**: seven vertex-granular sweeps in arena order
//! (vertex×vertex, vertex×edge, vertex×curved edge, vertex×face,
//! edge×face, edge×edge, curved edge×edge),
//! each examining the candidate pairs the BVH pre-filter hands it
//! ([`Candidates`] — the boolean edge×face sweep's convention, one
//! door over: a tree per entity class over the snapshot's boxes, the
//! candidates a subsequence of the arena order, so every sweep's
//! decision order, verdict order and error vector are the exact
//! all-pairs sweep's restricted to the survivors; the pad and the
//! conservative-superset argument are stated at [`Trees`]), plus the
//! conformal-patch arm and the cross-solid backstop named below. The
//! conformal arm groups by carrier KEY — not a coordinate sweep, and
//! it takes no filter; the backstop pairs every face of one solid
//! with every face of another and clears a pair on its reach boxes,
//! so it takes the same pre-filter over those very boxes
//! ([`sweep_cross_solid_backstop`]). Exact on the F5 planar subset (`Line`
//! carriers, `Plane` surfaces). **Since M9-2 the census ADMITS every
//! carrier kind**, and its reach is exactly this, stated class by
//! class (the union fix's truth pass):
//!
//! - **Examined and decided**: everything with vertex/line/planar
//!   evidence (the exact sweeps); a curved edge's interior meeting a
//!   vertex or another edge (passes 2′ and 5′, through
//!   `geom_brep::curve_pair`); same-key opposed-sense curved
//!   pairs (the conformal arm, [`sweep_conformal_patches`], through
//!   the chart-region predicate); face–face records (the patch
//!   certifier on one carrier or two planes, the `Tangent` table along
//!   the closed-form tangent line on two other carriers, the line
//!   placed in both faces). A proper pierce
//!   (`EdgeFacePierce`) is CATEGORICALLY undeclarable at rest — a
//!   transverse dive is interpenetration, and no record backs it.
//!   Between two copies at rest the assembly layer reports it as an
//!   interference finding; inside one body the recourse is separating
//!   the solids or making the crossing a boolean's working state. An
//!   in-plane `EdgeEdgeCross`
//!   at a declared seat is different: it is what an overhanging seat
//!   looks like from the census's side, and it is backable at the
//!   unified strength ([`ee_cross_backed`] — the crossing point in
//!   the declared pair's verified overlap region, material on
//!   opposite sides of the shared carrier).
//! - **Refused as UNDECIDABLE** (the conservative loudness backstop,
//!   [`sweep_cross_solid_backstop`]): cross-solid face pairs with a
//!   curved side within reach and no gap in depth between them
//!   (`depth_gap`) — curved × curved (the conformal cradle, a boss
//!   touching its hole) and curved × planar (the embedded ball cap,
//!   F5). Cone, torus and NURBS sides are reach-tested like any
//!   other — every surface kind has a sound cheap reach bound
//!   ([`face_reach`]); what refuses without a distance test is a
//!   DESCRIPTION with no claim in it (a placeholder patch), not a
//!   kind. A cross-key face–face record on such a pair is read by the
//!   tangent lane where the carriers derive a closed-form line (plane ×
//!   cylinder, parallel cylinders) and refused typed where they do not,
//!   so the class is never silently passed. One
//!   instance's extent box inside another's (the nested-instance
//!   class) is no longer refused on the box: the box is the GATE, and
//!   the MATERIAL test decides — every vertex of each instance probed
//!   against the other's material through the per-solid point-in-solid
//!   door, a vertex strictly inside refusing typed as
//!   [`ValidationError::InstanceInterference`] and a pair whose
//!   vertices all lie outside-or-on clearing only under the arm's
//!   stated conditions (its header); what the backstop still refuses
//!   there is a witness the door cannot answer, or a touch the local
//!   touch analysis cannot certify. The extent gate clears only a pair
//!   with nothing on record between it and no outer shell of either
//!   inside the other's reach; a pair that meets is probed and its
//!   findings read, so half-overlapping cubes whose boundaries meet
//!   only in touches refuse on the touch analysis. Why that decides a
//!   pair, and on what premises, is stated once, at arm 2's loop in
//!   [`sweep_cross_solid_backstop`].
//! - **Discovered within one solid**: two faces on distinct carriers
//!   touching along their closed-form tangent line through both
//!   interiors ([`sweep_same_solid_tangent_lines`]).
//! - **Genuinely undetected until C9/C6**: the other SAME-solid
//!   distinct-key curved pairs (the backstop is cross-solid), and one
//!   cross-solid residue named at that loop: a pair whose ONLY
//!   meetings are declared has some of its records taken on their word
//!   (`work/contacthold/declared-only-meetings-clear-at-the-census-gate-unread.md`).
//!   Cross-solid
//!   pairs the reach filter CLEARS are cleared soundly (the pads are
//!   sound bounds for the kinds that take the test), so clearance is a
//!   genuine no-touch certificate, not a skip. Named, not sampled.
//!
//! A record or candidate a certifier could not certify refuses
//! [`ValidationError::CensusUnsupported`], never samples. The refusal
//! carries [`ValidationError::CensusUnsupported::cause`]: usually a
//! lane's inventory limit, but the same arm reports a chart-region
//! search that ran out of budget, a body whose pcurve caches are
//! absent, a face the bounding sweep could not read at all (through
//! [`CensusUnsupportedCause::FaceUnboundable`] — its outer loop is
//! empty or its boundary does not resolve), and a face whose region
//! the point-in-face door cannot express or walk (through
//! [`CensusUnsupportedCause::Containment`]). Five recourses, one
//! variant, and the cause is which.
//!
//! **The chart-region door is a parameter of the pass, not a bound on
//! its scalar.** Every pass here takes
//! `region: Option<`[`RegionLane<T>`]`>` — the two chart-region doors
//! as one `Copy` value, constructible only at a scalar holding
//! `CertifiedBounds` — and the `pub` doors carry it to their callers:
//! [`census_and_certify`] and, under `sweep-testing`, [`census_traces`]
//! / [`census_traces_planted`]. It is read at three sites, and `None`
//! is the same typed non-answer at each, at whatever scalar the pass
//! runs: the conformal face-pair arm ([`sweep_conformal_patches`]) and
//! the face–face record confirm arm ([`confirm_patch`], its Door 2) each push [`ValidationError::CensusLaneUnsupported`]
//! naming the pair instead of examining it, and the region-confined
//! rungs' backing consult ([`pair_region_verified`], for a crossing and
//! for an edge-on-face bound) answers `false`, so an event the pair
//! would have backed stays an [`ValidationError::UnrecordedContact`].
//! `validate_pseudomanifold` hands `Some(RegionLane::certified())`; its
//! `_structural` twin hands `None` at every scalar it is called at — a
//! dual's only route, and an `f64` caller's when it chooses that door.
//!
//! **Sense-invariant** (M5 S10 audit), with ONE named exception.
//! Every use of a face's plane `normal` in the COINCIDENCE sweeps is
//! either an on-plane residual compared against `Sign::Zero` (does
//! this point/segment LIE on the face's carrier?) or the in-plane
//! frame of a ray-crossing PARITY count (`contfp`-style containment).
//! Neither reads a side: negating the normal leaves a zero residual
//! zero and leaves a crossing count unchanged — coincidence is a
//! question about position, never about which way the material lies.
//! The exception is the crossing rung's SIDE TEST
//! ([`ee_cross_backed`]): whether a declared pair may back an
//! `EdgeEdgeCross` is a question about which way the material lies —
//! opposite sides of the shared carrier is a legal overhang, one side
//! is interpenetration — and it hands each face's `Face::sense` bit to
//! the one sense algebra the kernel already carries
//! (`geom_brep::classify_material_pairing`, the tier-3 wedge pass's
//! family), never a hand-rolled sign.
//!
//! **Certification (F1/F2(iii))**: the census never blesses — every
//! finding must be *backed* by a declaration and every declaration
//! must be *confirmed* by geometry; both directions are typed errors.
//! Structural sharing (the same key) is intent by construction and
//! needs no record (the round-8 ladder's first rung).
//!
//! # The D3 segment-reconstruction rule (derived, pinned here)
//!
//! The records this rule reconstructs FROM are vertex–vertex and
//! vertex–face pairs. An edge–face record of a straight edge in a
//! plane is confirmed by this same reconstruction and backs no event of
//! its own, so what an overlap needs is unchanged by its presence. A
//! body's records also hold face–face pairs, and each bullet
//! below has a corresponding face rung — `Declared::vv_face_backed` /
//! `vf_face_backed` / `ve_face_backed`, which back a subordinate
//! vertex event from a declared FACE pair holding the entities it
//! relates on the pair's two sides, and `ee_face_backed`, which stands
//! in for a collinear overlap's edge–edge record where a declared face
//! pair holds the two edges. Those rungs are named in the bullets
//! rather than left out of the derivation that licenses them. There
//! are four rungs and not one per bullet: a bound whose event is a
//! vertex on an edge's interior reads `ve_face_backed` whichever
//! overlap it bounds, edge-edge or edge-on-face.
//!
//! **The ruled strength is UNIFIED** (CONTACT-DESIGN C3/C4's
//! post-ratification annotation; `crates/topo/README.md`):
//! *a declared pair answers exactly for its verified interface — the
//! overlap region, with material opposition being what "interface"
//! means for a crossing.* Two consult sites hold that strength: the
//! crossing rung ([`ee_cross_backed`], born at it) and
//! [`ef_bound_backed`]'s face-pair arms
//! ([`Declared::vf_face_backed_confined`]). Every other rung is
//! structural-incidence and region-unconfined — GRANDFATHERED BY NAME,
//! each to be migrated one at a time with its own measurement:
//! [`Declared::vv_face_backed`] at [`sweep_vertex_vertex`],
//! [`Declared::vf_face_backed`] at [`sweep_vertex_face`],
//! [`Declared::ve_face_backed`] at [`sweep_vertex_edge`] and
//! [`sweep_vertex_curved_edge`],
//! [`Declared::ee_face_backed`] at [`ee_collinear_lane`] and
//! [`sweep_curved_edge_edge`], and
//! [`ee_bound_backed`]'s face-pair arms. The face pairs every rung reads
//! are the ones the body's face–face records name, not user
//! declarations.
//! A grandfathered rung asks whether a declared face pair HOLDS the
//! entities of the event — one on each side, through boundary
//! membership and an edge's incidence to the faces it bounds — and
//! never where on those faces the event lies, so it backs an event on
//! the entities it holds even where that event lies outside the
//! pair's own overlap region.
//! Continuous overlaps — two collinear edges sharing a positive-length
//! segment, an edge resting in a face's region — are confirmed by
//! **reconstruction from their bounding vertex events**:
//!
//! - An **edge-edge collinear overlap** is a pair whose interiors meet,
//!   so it is owed its edge-edge record (D1), or a declared face pair
//!   whose interface it lies on, or both its bounds structural (each
//!   one point key); it is certified iff that holds and each of its
//!   two bounds is backed. Where both edges hold a vertex at the bound,
//!   that means the pair is v-v-declared, or backed by a declared face
//!   pair holding the two vertices on its two boundaries
//!   (`vv_face_backed`), or is one vertex or two on one point —
//!   structural. Where only one edge holds a vertex there — the
//!   endpoint rests on the other edge's INTERIOR — the bound is a
//!   vertex-on-edge event and is backed as that lane's events are: its
//!   `(vertex, edge)` record, or a declared face pair holding the vertex
//!   on one boundary and naming a face the other edge bounds
//!   (`ve_face_backed`). Derivation: the
//!   overlap of two collinear spans is an interval whose each bound is
//!   an endpoint of at least one span, so one of the two arms applies
//!   at every bound. Between two backed bounds the carriers coincide
//!   identically (two lines sharing two points are one line), so the
//!   bounds confirm the record: the interior overlap is exactly the
//!   convex closure of the bounded events on both carriers.
//! - An **edge-on-face overlap** is certified iff each of its two
//!   bounds is backed. A bound is a point where the edge ends or where
//!   the face's region ends along it, and the region ends only on the
//!   face's boundary, which meets the edge at a boundary vertex or
//!   where a boundary edge crosses it. So a bound is one of three
//!   events, each backed as its own lane's events are:
//!   - **The edge's own vertex.** It must be v-on-f-declared on this
//!     face, v-v-declared with a coincident vertex of the face's
//!     boundary, backed by a declared face pair naming this face and
//!     one holding the vertex, inside the pair's verified interface,
//!     `(vertex, edge)`-recorded onto an edge of the face's boundary,
//!     or itself a vertex of the face's boundary or on one point with
//!     one (structural).
//!   - **A boundary vertex of the face resting on the edge's
//!     interior.** A vertex-on-edge event: its `(vertex, edge)` record,
//!     or a declared face pair holding that vertex on one boundary and
//!     naming a face the edge bounds, inside its verified interface.
//!   - **A boundary edge crossing the edge's interior.** The edge-edge
//!     lane's `EdgeEdgeCross` event ([`boundary_crossings`] cuts there):
//!     an op's edge-edge record, or [`ee_cross_backed`]. A curved
//!     boundary edge's crossing is pass 5′'s event
//!     ([`sweep_curved_edge_edge`]), and backs the bound by its
//!     edge-edge record alone: the crossing rung is planar-first.
//!
//!   Same argument as the edge-edge bullet's, one dimension up: which
//!   entity ends at a bound is a fact about the configuration, not
//!   about what a declaration can hold. With a cut at every one of
//!   these, no boundary crosses a cell's open span on a decided
//!   reading, so each cell lies inside or outside the face whole and its
//!   midpoint probe answers for all of it.
//!
//! Failure mode: a segment overlap with a missing bounding record is
//! [`ValidationError::UnrecordedContact`] — never inferred. (A
//! configuration needing MORE than bounding records would require a
//! curved carrier — out of the planar inventory, refused as
//! `CensusUnsupported` with an inventory cause; no counterexample
//! exists on the F5 corpus.)
//!
//! # The D4 vertex-on-edge event
//!
//! A vertex resting on an edge's interior is backed two ways:
//!
//! - **Its own record**, the cell pair `(vertex, edge)`
//!   ([`crate::ContactKind::VertexEdge`]): a coincidence an op decided Zero
//!   and recorded (D10). The reduction never mints one: it
//!   splits the *other* edge at every on-edge event
//!   (`split_other_at_point`) and records a v-v pair instead. Every
//!   boolean output stage's join mints one, from the v-v record whose
//!   vertex it joined away. An edge split moves the record onto the piece
//!   the vertex rests on; carried into a later op, it backs the event
//!   there.
//! - **The face rung**: a declared face pair holding the vertex on one
//!   boundary and naming a face the edge bounds (`ve_face_backed`) holds
//!   the whole event — the vertex on one side of the interface, the
//!   edge on the other — exactly as `vv_face_backed` holds a coincident
//!   vertex pair. At rest nothing refines, so a seat whose two faces
//!   share a boundary induces this event raw, and the declaration that
//!   says the faces rest says it once for everything the seat induces
//!   — including where the event is a BOUND of a continuous overlap
//!   rather than a finding of its own: the D3 bullets read this event's
//!   backing at such a bound, in both the edge-edge and the
//!   edge-on-face lane.
//!
//! With neither, the census reports [`CensusContact::VertexOnEdge`] as
//! `UnrecordedContact`. Both are written before the census runs — the
//! record by the op that decided the coincidence, the face pair by the
//! recipe — and neither is read off the geometry's own agreement with
//! itself: a configuration nothing records stays the F1 hard error
//! however exactly it coincides.
//!
//! Cross-reference: the face rung is CONTACT-DESIGN C3's declared
//! rung read at the granularity the records already carry, not a new
//! identity claim about the carriers — what the declared pair itself
//! must satisfy is the confirm pass's, on C3's ladder.

use std::collections::BTreeSet;

use bvh::{Aabb, Bvh};
use geom_brep::curve_pair::{Hit, MeetFault, Span as CarrierSpan, meet};
use geom_core::k_stats::Magnitude;
use geom_core::{
    Band, Bounds, Decide, Indeterminate, Margin, MarginDiag, Point3, Real, Sign, Tol, Vec3,
};

use crate::body::Body;
use crate::boolean::boxes::{edge_box, face_box, sweep_pad};
use crate::boolean::{
    ContactKind, ContactRecords, ContainError, FaceContainment, SolidContainment, contfp,
};
use crate::chart_region::{ChartRegionError, RegionLane};
use crate::entity::{
    EdgeKey, EntityId, Face, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey,
};
use crate::geometry::PointKey;
use crate::live::{BoundaryMember, proven};
use crate::null::CurveGeom;
use crate::validate::{
    CensusContact, CensusSubject, CensusUnsupportedCause, CrossingSideVerdict, StaleDeclaration,
    ValidationError, decide,
};

/// One edge's exact census geometry (post-gate: a `Line` carrier).
struct EdgeGeo<T: Real> {
    key: EdgeKey,
    v0: VertexKey,
    v1: VertexKey,
    p0: Point3<T>,
    /// `v1`'s own point.
    p1: Point3<T>,
    /// The point keys `v0` and `v1` sit on; `None` for a segment no
    /// body holds.
    points: [Option<PointKey>; 2],
    /// Unit direction v0 → v1.
    dir: Vec3<T>,
    /// Chord length (positive on tier-2 bodies).
    len: T,
    /// The two adjacent faces (structural-adjacency exclusions).
    f_plus: FaceKey,
    f_minus: FaceKey,
}

/// A straight span: its start, unit direction and length — what
/// [`collinear_overlap`] reads of an edge.
#[derive(Clone, Copy)]
struct Span<T: Real> {
    p0: Point3<T>,
    dir: Vec3<T>,
    len: T,
}

/// What has a straight span.
trait Spans<T: Real> {
    fn span(&self) -> Span<T>;
}

impl<T: Real> Spans<T> for Span<T> {
    fn span(&self) -> Span<T> {
        *self
    }
}

impl<T: Real> Spans<T> for EdgeGeo<T> {
    fn span(&self) -> Span<T> {
        Span {
            p0: self.p0,
            dir: self.dir,
            len: self.len,
        }
    }
}

/// One face's exact census geometry (post-gate: a `Plane`).
struct FaceGeo<T: Real> {
    key: FaceKey,
    origin: Point3<T>,
    normal: Vec3<T>,
    /// Every vertex on the face's outer loop and rings.
    boundary: BTreeSet<VertexKey>,
    /// The face's loops, each its vertices in walk order (the outer
    /// loop first) — the order a corner is read in.
    loops: Vec<Vec<VertexKey>>,
    /// Every loop walked and every edge of every loop is a `Line`: the
    /// face is the planar polygon its vertices bound, so it lies in
    /// their convex hull.
    polygon: bool,
}

/// The census snapshot: the exact planar geometry the vertex-granular
/// sweeps read, plus the curved inventory the face-granular arms own
/// (M9-2: the census admits every carrier kind; what changes per kind
/// is WHICH arm can certify it — module docs).
struct Geo<T: Real> {
    verts: Vec<(VertexKey, Point3<T>)>,
    edges: Vec<EdgeGeo<T>>,
    faces: Vec<FaceGeo<T>>,
    /// Key → position (the sweeps' random-access view of `verts`).
    vmap: std::collections::BTreeMap<VertexKey, Point3<T>>,
    /// Key → the point the vertex sits on: two vertices on one point
    /// are structural sharing ([`Geo::same_point`]).
    vpoint: std::collections::BTreeMap<VertexKey, PointKey>,
    /// Conventional vertex → its closed edge: a touch at the vertex is
    /// a touch on that edge's interior ([`Declared::at_conventional`]).
    conventional: std::collections::BTreeMap<VertexKey, EdgeKey>,
    /// Faces on non-`Plane` carriers — outside the exact planar
    /// sweeps, inside the conformal face-pair arm.
    curved_faces: Vec<FaceKey>,
    /// Vertex → the faces whose boundary holds it (EVERY face, curved
    /// included) — the face-granularity backing rung's incidence.
    vertex_faces: std::collections::BTreeMap<VertexKey, BTreeSet<FaceKey>>,
}

/// Which candidate-generation path the six vertex-granular sweeps
/// and the backstop run — the idealized/realized pair of PERF-PLAN
/// §4.4, the boolean sweep's [`crate::boolean::SweepStrategy`] one
/// door over. Production entries always run
/// [`CensusStrategy::Realized`]; the idealized path is the executable
/// definition of the candidate set, alive for the differential suite's
/// pins alone. Outside `sweep-testing` this enum has ONE variant, and
/// the trace plumbing ([`CensusTrace`]) rides every production sweep
/// as `None` — the [`crate::boolean::SweepTrace`] precedent, kept
/// rather than a test-only twin of the sweeps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CensusStrategy {
    /// BVH-pruned candidates (the production path).
    Realized,
    /// Every pair in arena order (the reference definition).
    ///
    /// `sweep-testing` feature only, for the reason the boolean's
    /// `Idealized` variant is gated: the variant exists so the
    /// differential suite can execute the definition, not so a
    /// production caller can choose the quadratic sweep — the gate
    /// makes "production runs `Realized`" a fact the compiler holds.
    #[cfg(feature = "sweep-testing")]
    Idealized,
}

/// The pairs one sweep examined, and the ones it decided AGAINST —
/// pushed at least one finding for — each in the sweep's own order.
/// The differential suite's currency: realized `examined` must hold
/// every idealized `accepted` pair (the conservative-superset pin).
#[derive(Clone, Debug, Default)]
pub struct SweepPairs {
    /// Every pair the sweep examined, in decision order.
    pub examined: Vec<(EntityId, EntityId)>,
    /// The examined pairs whose examination pushed a finding.
    pub accepted: Vec<(EntityId, EntityId)>,
}

impl SweepPairs {
    fn note(&mut self, pair: (EntityId, EntityId), accepted: bool) {
        self.examined.push(pair);
        if accepted {
            self.accepted.push(pair);
        }
    }

    /// Whether this sweep's examined sequence is `exact`'s with pairs
    /// removed and nothing reordered — the order pin of the
    /// differential suites (`sweep-testing`; both suites read it here).
    #[cfg(feature = "sweep-testing")]
    pub fn is_restriction_of(&self, exact: &Self) -> bool {
        let mut it = exact.examined.iter();
        self.examined.iter().all(|p| it.any(|q| q == p))
    }

    /// `exact`'s accepted pairs this sweep never examined — any entry
    /// is a finding the filter lost (the superset pin).
    #[cfg(feature = "sweep-testing")]
    pub fn lost_accepted(&self, exact: &Self) -> Vec<(EntityId, EntityId)> {
        exact
            .accepted
            .iter()
            .filter(|p| !self.examined.contains(p))
            .copied()
            .collect()
    }
}

/// One census run's examined/accepted pairs, per sweep.
#[derive(Clone, Debug, Default)]
pub struct CensusTrace {
    /// Vertex×vertex (pass 1).
    pub vv: SweepPairs,
    /// Vertex×edge (pass 2).
    pub ve: SweepPairs,
    /// Vertex×curved edge (pass 2′).
    pub vce: SweepPairs,
    /// Vertex×face (pass 3).
    pub vf: SweepPairs,
    /// Edge×face (pass 4).
    pub ef: SweepPairs,
    /// Edge×edge (pass 5).
    pub ee: SweepPairs,
    /// Curved edge×edge (pass 5′).
    pub cee: SweepPairs,
    /// The cross-solid backstop's pairs: the face pairs that reached
    /// its reach test (after its structural and deferral skips) and
    /// the instance pairs that reached its extent-containment test.
    pub backstop: SweepPairs,
}

#[cfg(feature = "sweep-testing")]
impl CensusTrace {
    /// The eight sweeps by name, in census order — what a differential
    /// suite walks.
    pub fn sweeps(&self) -> [(&'static str, &SweepPairs); 8] {
        [
            ("vv", &self.vv),
            ("ve", &self.ve),
            ("vce", &self.vce),
            ("vf", &self.vf),
            ("ef", &self.ef),
            ("ee", &self.ee),
            ("cee", &self.cee),
            ("backstop", &self.backstop),
        ]
    }
}

/// The BVH pre-filter: one tree per entity class over the snapshot's
/// boxes and the curved edges' (pass 2′'s answering side), built in arena order (the crate's determinism contract), the
/// item index being the snapshot index.
///
/// # The boxes, and why pruning is conservative
///
/// Every box is a certified superset of its entity's locus, padded by
/// [`sweep_pad`] — the boolean sweep's own pad, `escalate + 2·zero`:
///
/// - a **face** box is [`face_box`], sound for every carrier kind
///   (planar faces and `Line` edges take the boundary hull; a boundary
///   carrying a curve with no sound box, or a null carrier, poisons —
///   the poison box overlaps everything and is never pruned);
/// - an **edge** box is [`edge_box`] — the chord box, since only `Line`
///   carriers enter the snapshot; a **curved edge**'s, pass 2′'s
///   answering side, is the same door's sound box for its kind (a
///   carrier with no sound box poisons, and is never pruned);
/// - a **vertex** box is the point widened by the pad;
/// - the **backstop**'s box is the face's own reach ([`face_reach`],
///   sound for every carrier kind), padded the same way — and a face
///   with no sound reach takes the poison box, so it is examined
///   against everything and refuses exactly as it does today.
///
/// A pair is pruned only when its two padded boxes are strictly
/// disjoint on some axis, so the loci are more than `2·pad` apart.
/// Every finding the exact sweeps can push needs the two entities
/// within `zero` of each other — the coincidence itself: a vertex on
/// a vertex, on an edge's interior, in a face's region; an edge
/// piercing or resting in a face; edges crossing or overlapping —
/// and every escalation or refusal about the pair's OWN interface (an
/// in-band span at a coincidence, a containment walk near the
/// boundary, a region the walk cannot express at a point in it)
/// needs them within `√(zero² + escalate²) < pad`. None is reachable
/// on a pruned pair, so the filtered sweeps push every finding the
/// exact sweeps push and their result stays a function of exact
/// tests only (D9; PERF-PLAN §2.1's contract).
///
/// What a pruned pair CAN carry in the exact sweep is an outcome
/// decided on the infinite CARRIERS rather than on the entities: a
/// vertex coplanar with a face
/// whose region walk refuses or escalates though the vertex is
/// nowhere near the face (`contfp`'s `RayExhausted` refusal, its
/// ray-walk escalations, and its `Uncrossable` refusal anywhere
/// inside the reach of a spline edge, which can be far wider than the
/// face). The exact sweep
/// escalates or refuses those — it asked a carrier question and could
/// not answer it — and the box separation answers the entity question
/// the census is asking with a DEFINITE verdict: the entities are
/// apart by more than the pad. No indeterminate is swallowed — a
/// cleared pair is decided, not skipped, and a poisoned box is never
/// cleared. That class is the filter's one behavioural difference
/// from the exact sweep, the same one the boolean sweep names for its
/// tree (`boolean/reduce.rs`'s module docs: a pair whose boxes are
/// disjoint can still escalate the brute path on a face's infinite
/// plane; pruning drops only such spurious escalations), admitted as
/// such by the differential suites, never absorbed. No row exhibits a
/// member of the class
/// (`work/restread/the-census-filters-named-difference-has-no-witness-row.md`).
///
/// A box door that reports corruption yields the poison box: the
/// filter has no opinion, and the exact examination that follows is
/// the authority on a face the tier-1 gate let through.
struct Trees {
    /// The vertices — the query side of three sweeps and the
    /// self-joined side of the fourth.
    verts: Class,
    /// The `Line` edges — the query side of edge×face, self-joined for
    /// edge×edge, and the curved edge×edge pass's answering side.
    edges: Class,
    /// The planar faces — always the answering side.
    faces: Class,
    /// The curved edges, in arena order — pass 2′'s answering side, and
    /// pass 5′'s query side, self-joined for its curved pairs.
    curved: Class,
    /// Every face, curved or planar, in arena order — self-joined by the
    /// same-solid tangent-line pass.
    every_face: Class,
}

/// One entity class under the strategy: its padded boxes in snapshot
/// order and, where the strategy prunes, the C10 tree over them.
///
/// `boxes` is kept beside the tree, which holds its own copy: the
/// crate hands back no item box, and the query side of a cross-class
/// sweep (a vertex against the edge tree) needs this class's box for
/// item `i`.
struct Class {
    boxes: Vec<Aabb>,
    /// `None` where every pair is a candidate (the idealized strategy).
    tree: Option<Bvh>,
}

impl Class {
    fn build(strategy: CensusStrategy, boxes: Vec<Aabb>) -> Self {
        let tree = match strategy {
            CensusStrategy::Realized => Some(Bvh::build(&boxes)),
            #[cfg(feature = "sweep-testing")]
            CensusStrategy::Idealized => None,
        };
        Self { boxes, tree }
    }

    /// This class's box for item `i` — the query another class answers.
    fn query(&self, i: usize) -> &Aabb {
        candidate(&self.boxes, i)
    }

    /// The items whose box overlaps `query`, ascending — a subsequence
    /// of the snapshot order, independent of tree shape
    /// ([`Bvh::overlapping`]'s contract); every item where nothing
    /// prunes.
    fn overlapping(&self, query: &Aabb) -> Vec<usize> {
        match &self.tree {
            Some(tree) => tree.overlapping(query),
            None => (0..self.boxes.len()).collect(),
        }
    }

    /// The items after `i` whose box overlaps item `i`'s — the
    /// self-join of a class, ascending.
    fn later(&self, i: usize) -> Vec<usize> {
        let mut out = self.overlapping(self.query(i));
        out.retain(|&j| j > i);
        out
    }
}

/// The empty box: overlaps nothing, so the face it stands for is
/// examined against nothing — the planted degradation the
/// differential suites must catch (`boolean::reduce`'s pin (iii), the
/// same shape).
fn empty_box() -> Aabb {
    Aabb {
        min_x: f64::INFINITY,
        min_y: f64::INFINITY,
        min_z: f64::INFINITY,
        max_x: f64::NEG_INFINITY,
        max_y: f64::NEG_INFINITY,
        max_z: f64::NEG_INFINITY,
    }
}

/// The candidate pairs every sweep examines, under one strategy: the
/// three snapshot classes, the curved edges and every face, and a door for the
/// classes the backstop
/// builds over its own boxes.
struct Candidates {
    strategy: CensusStrategy,
    trees: Trees,
    /// The curved edges [`Trees::curved`] indexes, in its order.
    curved: Vec<EdgeKey>,
    /// The faces [`Trees::every_face`] indexes, in its order.
    every_face: Vec<FaceKey>,
}

/// The item of a snapshot slice at a candidate index. The index is
/// minted by a tree built over this very slice (or by a range over
/// its length), so a miss is a kernel bug, not an input.
fn candidate<X>(items: &[X], i: usize) -> &X {
    items
        .get(i)
        .unwrap_or_else(|| unreachable!("a candidate index is minted over this slice"))
}

impl Candidates {
    /// `plant` names a face whose box is the empty box instead of its
    /// own (the planted degradation; `None` in production).
    fn build<T: Decide + Bounds>(
        body: &Body<T>,
        geo: &Geo<T>,
        band: Band,
        strategy: CensusStrategy,
        plant: Option<FaceKey>,
    ) -> Self {
        let pad = sweep_pad(band);
        let vert_boxes: Vec<Aabb> = geo
            .verts
            .iter()
            .map(|&(_, p)| Aabb::from_points([p]).map_or_else(Aabb::poison, |b| b.padded(pad)))
            .collect();
        let edge_boxes: Vec<Aabb> = geo
            .edges
            .iter()
            .map(|e| edge_box(body, e.key, pad))
            .collect();
        let face_boxes: Vec<Aabb> = geo
            .faces
            .iter()
            .map(|f| {
                if plant == Some(f.key) {
                    empty_box()
                } else {
                    face_box(body, f.key, pad, band).unwrap_or_else(|_| Aabb::poison())
                }
            })
            .collect();
        let curved: Vec<EdgeKey> = body
            .edges
            .keys()
            .filter(|&k| !edge_is_line(body, k))
            .collect();
        // A curved edge's box: a spline's control hull
        // (`nurbs_curve_aabb`, which holds the curve for its positive
        // weights), where `edge_box` has no sound box for the kind and
        // answers the poison box every vertex would meet; every other
        // kind's from `edge_box`.
        let curved_box = |k: EdgeKey| {
            let spline = body
                .edges
                .get(k)
                .and_then(|d| body.curves.get(d.curve))
                .and_then(crate::null::CurveGeom::certified)
                .and_then(|c| match c.carrier() {
                    geom::Curve3::Nurbs(n) => Some(geom::curves::boxes::nurbs_curve_aabb(n)),
                    _ => None,
                });
            spline.map_or_else(|| edge_box(body, k, pad), |hull| hull.padded(pad))
        };
        let curved_boxes = curved.iter().map(|&k| curved_box(k)).collect();
        let every_face: Vec<FaceKey> = body.faces.keys().collect();
        let every_face_boxes = every_face
            .iter()
            .map(|&k| {
                if plant == Some(k) {
                    empty_box()
                } else {
                    face_box(body, k, pad, band).unwrap_or_else(|_| Aabb::poison())
                }
            })
            .collect();
        Self {
            strategy,
            trees: Trees {
                verts: Class::build(strategy, vert_boxes),
                edges: Class::build(strategy, edge_boxes),
                faces: Class::build(strategy, face_boxes),
                curved: Class::build(strategy, curved_boxes),
                every_face: Class::build(strategy, every_face_boxes),
            },
            curved,
            every_face,
        }
    }

    /// A class over `boxes` under this run's strategy — the door the
    /// backstop's two arms build their own classes through.
    fn class(&self, boxes: Vec<Aabb>) -> Class {
        Class::build(self.strategy, boxes)
    }
}

/// Runs the census and the two-direction certification diff (module
/// docs); returns every failure in deterministic sweep order. Assumes
/// tiers 1–3-local already passed (the caller gates). Production
/// entry: the realized strategy, no trace.
pub(crate) fn census_and_certify<T: Decide + crate::props::AtRestPolicy + Bounds>(
    body: &Body<T>,
    contacts: &ContactRecords,
    band: Band,
    tol: Tol,
    region: Option<RegionLane<T>>,
) -> Vec<ValidationError> {
    census_with(
        body,
        contacts,
        band,
        tol,
        region,
        CensusStrategy::Realized,
        None,
        None,
    )
}

/// The differential door: the census under `strategy`, with every
/// sweep's examined and accepted pairs recorded. Reference surface
/// (`sweep-testing`), exactly like [`crate::boolean::sweep_traces`].
///
/// Unlike the validator's door, this one runs the census WITHOUT the
/// tier-1 gate: on a body tier 1 would refuse (a split vertex orbit, a
/// dangling key), the touch analysis reports
/// [`Undecided::CorruptInstance`] — a kernel defect — about a body the
/// caller never validated. The sentence is only as true as the caller.
#[cfg(feature = "sweep-testing")]
pub fn census_traces<T: Decide + crate::props::AtRestPolicy + Bounds>(
    body: &Body<T>,
    contacts: &ContactRecords,
    band: Band,
    tol: Tol,
    region: Option<RegionLane<T>>,
    strategy: CensusStrategy,
) -> (Vec<ValidationError>, CensusTrace) {
    let mut trace = CensusTrace::default();
    let errors = census_with(
        body,
        contacts,
        band,
        tol,
        region,
        strategy,
        None,
        Some(&mut trace),
    );
    (errors, trace)
}

/// [`census_traces`] with one face's box planted EMPTY, so that face is
/// examined against nothing: the degradation the differential suites'
/// superset comparator must catch (`boolean::reduce`'s pin (iii)).
/// Runs without the tier-1 gate, as [`census_traces`] does.
#[cfg(feature = "sweep-testing")]
pub fn census_traces_planted<T: Decide + crate::props::AtRestPolicy + Bounds>(
    body: &Body<T>,
    contacts: &ContactRecords,
    band: Band,
    tol: Tol,
    region: Option<RegionLane<T>>,
    strategy: CensusStrategy,
    plant: crate::boolean::PlantedDegradation,
) -> (Vec<ValidationError>, CensusTrace) {
    let mut trace = CensusTrace::default();
    let errors = census_with(
        body,
        contacts,
        band,
        tol,
        region,
        strategy,
        Some(plant.face),
        Some(&mut trace),
    );
    (errors, trace)
}

/// `tol` rides beside `band` for one consumer: the instance-containment
/// arm's material test, whose at-infinity fold reads the sign of a
/// volume through the props lane (`Tol` is never witnessed here).
#[allow(clippy::too_many_arguments)] // the census's whole state: the doors' four, the region door, and the trace's three
fn census_with<T: Decide + crate::props::AtRestPolicy + Bounds>(
    body: &Body<T>,
    contacts: &ContactRecords,
    band: Band,
    tol: Tol,
    region: Option<RegionLane<T>>,
    strategy: CensusStrategy,
    plant: Option<FaceKey>,
    mut trace: Option<&mut CensusTrace>,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let geo = snapshot(body);
    let declared = Declared::index(&geo, contacts);
    let cands = Candidates::build(body, &geo, band, strategy, plant);
    sweep_vertex_vertex(
        &geo,
        &declared,
        band,
        &cands,
        trace.as_deref_mut(),
        &mut errors,
    );
    sweep_vertex_edge(
        &geo,
        &declared,
        band,
        &cands,
        trace.as_deref_mut(),
        &mut errors,
    );
    sweep_vertex_curved_edge(
        body,
        &geo,
        &declared,
        band,
        &cands,
        trace.as_deref_mut(),
        &mut errors,
    );
    sweep_vertex_face(
        body,
        &geo,
        &declared,
        band,
        &cands,
        trace.as_deref_mut(),
        &mut errors,
    );
    sweep_edge_face(
        body,
        &geo,
        &declared,
        band,
        region,
        &cands,
        trace.as_deref_mut(),
        &mut errors,
    );
    sweep_edge_edge(
        body,
        &geo,
        &declared,
        band,
        region,
        &cands,
        trace.as_deref_mut(),
        &mut errors,
    );
    sweep_curved_edge_edge(
        body,
        &geo,
        &declared,
        band,
        &cands,
        trace.as_deref_mut(),
        &mut errors,
    );
    sweep_conformal_patches(body, &geo, &declared, band, region, &mut errors);
    sweep_same_solid_tangent_lines(body, &geo, &declared, band, &cands, &mut errors);
    sweep_cross_solid_backstop(body, &geo, &declared, band, tol, &cands, trace, &mut errors);
    confirm_records(body, &geo, contacts, band, region, &mut errors);
    errors
}

/// The declaration index: v-v pairs normalized to arena order is NOT
/// meaningful across operands, so pairs are stored unordered (both
/// orientations); v-on-f pairs merge both operand directions (which
/// operand pierced is reduction bookkeeping — in result keys the pair
/// is the declaration).
struct Declared {
    vv: BTreeSet<(VertexKey, VertexKey)>,
    vf: BTreeSet<(VertexKey, FaceKey)>,
    ve: BTreeSet<(VertexKey, EdgeKey)>,
    /// Recorded edge-edge pairs, both orientations.
    ee: BTreeSet<(EdgeKey, EdgeKey)>,
    /// Face-granularity keys (M9-2): the face pairs the body's
    /// face–face records name, both orientations. A face-pair
    /// record backs the events SUBORDINATE to it — a boundary vertex
    /// of one declared face resting on the other, the coincident
    /// vertex pairs and segment bounds along their interface, and a
    /// collinear overlap of two edges it holds — exactly as a v-v/v-f
    /// declaration backs its own event; the record's own geometric
    /// confirmation is the confirm pass's (two-directional, as ever).
    faces: BTreeSet<(FaceKey, FaceKey)>,
}

impl Declared {
    fn index<T: Real>(geo: &Geo<T>, contacts: &ContactRecords) -> Self {
        let mut out = Self {
            vv: BTreeSet::new(),
            vf: BTreeSet::new(),
            ve: BTreeSet::new(),
            ee: BTreeSet::new(),
            faces: BTreeSet::new(),
        };
        // Every kind, by name: a new kind fails to compile here until
        // the census says what it backs.
        for kind in contacts.kinds() {
            match kind {
                ContactKind::VertexVertex(a, b) => {
                    out.vv.insert((a, b));
                    out.vv.insert((b, a));
                }
                ContactKind::VertexFace(v, f) => {
                    out.vf.insert((v, f));
                }
                ContactKind::VertexEdge(v, e) => {
                    out.ve.insert((v, e));
                }
                ContactKind::EdgeEdge(a, b) => {
                    out.ee.insert((a, b));
                    out.ee.insert((b, a));
                }
                // An edge–face record backs the vertex-on-face event at
                // each conventional vertex of its edge, a point of the
                // edge's interior: what a vertex-on-face record of that
                // vertex backed before the carry wrote it on the edge.
                // Its overlap is certified from its bounds, as an
                // unrecorded edge resting in a face is, so a straight
                // edge in a plane backs no event of its own.
                ContactKind::EdgeFace(e, f) => {
                    for v in conventional_vertices(geo, e) {
                        out.vf.insert((v, f));
                    }
                }
                ContactKind::FaceFace(a, b) => {
                    out.faces.insert((a, b));
                    out.faces.insert((b, a));
                }
            }
        }
        out
    }

    /// The face rung for a v-v event: some declared face pair holds
    /// `a` on one boundary and `b` on the other.
    fn vv_face_backed(&self, geo: &Geo<impl Real>, a: VertexKey, b: VertexKey) -> bool {
        let (Some(fa), Some(fb)) = (geo.vertex_faces.get(&a), geo.vertex_faces.get(&b)) else {
            return false;
        };
        fa.iter()
            .any(|&ga| fb.iter().any(|&gb| self.faces.contains(&(ga, gb))))
    }

    /// The face rung for a v-on-f event: some declared face pair
    /// holds `v` on its boundary and names `f` as the other side.
    fn vf_face_backed(&self, geo: &Geo<impl Real>, v: VertexKey, f: FaceKey) -> bool {
        geo.vertex_faces
            .get(&v)
            .is_some_and(|gs| gs.iter().any(|&g| self.faces.contains(&(g, f))))
    }

    /// The face rung for a vertex-on-edge event: the v-on-f rung
    /// against either face the edge bounds. An edge is structurally
    /// incident to `f_plus` and `f_minus`, so a declared pair holding
    /// `v` on one boundary and naming a face of `e` on the other holds
    /// the whole event — the vertex on one side of the interface, the
    /// edge on the other.
    ///
    /// Strength, stated rather than left to be read off the code: this
    /// rung is STRUCTURAL-INCIDENCE, exactly as strong as the two
    /// rungs it is built from and no stronger. It asks whether the
    /// declared pair holds the two entities, never WHERE on the pair
    /// the event lies, so it backs an event anywhere on the incident
    /// entities — including outside the declared faces' own overlap
    /// region. That reach is no longer the doctrine, it is the
    /// GRANDFATHER (module docs: the unified strength is the ruled
    /// sentence, the crossing rung its first instance, and the census
    /// deliberately holds its rungs to two standards while the named
    /// migrations land one measured step at a time).
    fn ve_face_backed<T: Real>(
        &self,
        geo: &Geo<T>,
        v: VertexKey,
        (f_plus, f_minus): (FaceKey, FaceKey),
    ) -> bool {
        self.vf_face_backed(geo, v, f_plus) || self.vf_face_backed(geo, v, f_minus)
    }

    /// [`Declared::vf_face_backed`] at the UNIFIED strength: a declared
    /// pair `(g, f)` holding `v` on `g`'s boundary backs the event only
    /// where the pair verifies ([`pair_verified`]). That is the whole
    /// confinement, read as the closed intersection of the two regions,
    /// the licence [`ee_cross_backed`] documents: at both call sites the
    /// event's point lies on `v` and so in `g`'s closed region, and on
    /// the edge resting in `f` (or on an edge `f` bounds) and so in
    /// `f`'s, and Door 1 has made the two carriers one. A touching event
    /// has no crossing to take a side on, so there is no side test.
    /// Planar-first, as the crossing rung: a curved pair answers nothing
    /// here.
    fn vf_face_backed_confined<T: Decide>(
        &self,
        body: &Body<T>,
        geo: &Geo<T>,
        v: VertexKey,
        f: FaceKey,
        band: Band,
        region: Option<RegionLane<T>>,
    ) -> bool {
        geo.vertex_faces.get(&v).is_some_and(|gs| {
            gs.iter().any(|&g| {
                self.faces.contains(&(g, f))
                    && planar_face(geo, g).is_some()
                    && planar_face(geo, f).is_some()
                    && pair_verified(body, g, f, band, region)
            })
        })
    }

    /// The rung for a touch at a **conventional vertex**: it has no
    /// identity of its own (`docs/DESIGN.md`, maximal edges), so every
    /// op writes a record there as its closed edge's (`boolean::ops::
    /// record`), and the touch is backed by that record — `b` at `a`
    /// as `b` on `a`'s edge, two of them as their edges meeting, and an
    /// edge `b` through one as the two edges meeting.
    fn at_conventional<T: Real>(&self, geo: &Geo<T>, a: VertexKey, b: EntityId) -> bool {
        let Some(&ea) = geo.conventional.get(&a) else {
            return false;
        };
        match b {
            EntityId::Vertex(b) => match geo.conventional.get(&b) {
                Some(&eb) => self.ee_recorded(ea, eb),
                None => self.ve_recorded(b, ea),
            },
            EntityId::Edge(e) => self.ee_recorded(ea, e),
            _ => false,
        }
    }

    /// Whether an op recorded `v` resting on `e`'s interior: the
    /// coincidence it decided Zero (D10), as a `(vertex, edge)` record.
    fn ve_recorded(&self, v: VertexKey, e: EdgeKey) -> bool {
        self.ve.contains(&(v, e))
    }

    /// Whether an op recorded the interiors of `a` and `b` meeting: the
    /// coincidence it decided Zero (D10), as an edge-edge record.
    fn ee_recorded(&self, a: EdgeKey, b: EdgeKey) -> bool {
        self.ee.contains(&(a, b))
    }

    /// The face rung for a collinear overlap: some declared face pair
    /// names a face `a` bounds on one side and a face `b` bounds on the
    /// other. GRANDFATHERED, at [`Declared::vv_face_backed`]'s
    /// structural-incidence strength: it asks whether the pair holds
    /// the two edges, never where the overlap lies, so it backs an
    /// overlap outside the pair's own overlap region (two coplanar
    /// faces touching along one line share no area, and their edges'
    /// overlap is backed all the same).
    fn ee_face_backed<T: Real>(&self, a: &EdgeGeo<T>, b: &EdgeGeo<T>) -> bool {
        self.sides_face_backed((a.f_plus, a.f_minus), (b.f_plus, b.f_minus))
    }

    /// [`Declared::ee_face_backed`] on the two edges' sides, for an edge
    /// the planar snapshot does not hold (pass 5′).
    fn sides_face_backed(&self, a: (FaceKey, FaceKey), b: (FaceKey, FaceKey)) -> bool {
        [a.0, a.1]
            .iter()
            .any(|&fa| [b.0, b.1].iter().any(|&fb| self.faces.contains(&(fa, fb))))
    }
}

/// The planar snapshot entry for a face, or `None` for a curved one.
fn planar_face<T: Real>(geo: &Geo<T>, key: FaceKey) -> Option<&FaceGeo<T>> {
    geo.faces.iter().find(|f| f.key == key)
}

/// The crossing rung's region confinement: the event
/// point lies ON the declared pair's shared carrier (a signed
/// point-to-plane residual per face — metres, [`Margin::of`]'s door)
/// and within both faces' CLOSED regions. Closed, not strict: the
/// event this confines (a crossing of two boundary edges, for the
/// crossing rung) lies on the trims' boundaries by construction,
/// so `In`, `OnEdge` and `OnVertex` all hold the point; only a
/// definite `Out` (or a containment the census cannot decide, already
/// pushed by [`contain`]) exiles it. A definitely-off-carrier point
/// is a silent non-answer — the pair simply does not hold the event —
/// while an in-band residual escalates through the shared row.
///
/// `pm_census_confined_carrier` is the THIRD spelling of a signed
/// point-to-plane residual in this module, beside the v-on-f sweep's
/// `pm_census_vf_residual` and the confirm pass's
/// `pm_census_confirm_vf` (and [`pair_holds_edges`] reads this same
/// row at edge endpoints). One quantity, one `Margin::of` door, three
/// sites: they are separate rows because each site meters its own
/// question and a shared name would let one site's distribution mask
/// another's in the k-report; if any spelling changes, the other two
/// do not follow it — each row is its own contract.
fn pair_holds_point<T: Decide>(
    body: &Body<T>,
    fa: &FaceGeo<T>,
    fb: &FaceGeo<T>,
    q: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> bool {
    for f in [fa, fb] {
        let residual = (q - f.origin).dot(f.normal);
        if signed_is_zero(
            "pm_census_confined_carrier",
            Margin::of(residual),
            band,
            errors,
        ) != Some(true)
        {
            return false;
        }
        if !matches!(
            contain(body, f, q, band, errors),
            Some(FaceContainment::In | FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_))
        ) {
            return false;
        }
    }
    true
}

/// The EDGE half of the confinement screen ([`ee_cross_backed`]
/// condition 1): the crossing is an event OF the pair's carrier only
/// if BOTH crossing edges LIE IN both faces' carriers — the design's
/// "two coplanar boundary edges" as a decided screen rather than an
/// assumption. A line edge lies in a plane iff its two endpoints do;
/// both endpoints of both edges are decided against both planes
/// (eight residuals through [`pair_holds_point`]'s row — the same
/// quantity at the same door, see that row's three-spellings note),
/// so the screen does not lean on the crossing point's own residual.
/// What it refuses is exactly the transverse shapes the side test is
/// blind to: an edge DIVING through the carrier (its far endpoints
/// definitely off-plane — material continuing through the seat), and
/// a SKEW candidate pair whose two carriers cannot both contain two
/// crossing lines (two crossing lines span one plane, so both screens
/// passing forces the carriers to agree at the crossing within ε —
/// which is what makes the Smooth precondition establishable at all).
/// Definitely-off is a silent non-answer; in-band escalates through
/// the shared row.
fn pair_holds_edges<T: Decide>(
    fa: &FaceGeo<T>,
    fb: &FaceGeo<T>,
    ea: &EdgeGeo<T>,
    eb: &EdgeGeo<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> bool {
    for f in [fa, fb] {
        for e in [ea, eb] {
            for p in [e.p0, e.p1] {
                let residual = (p - f.origin).dot(f.normal);
                if signed_is_zero(
                    "pm_census_confined_carrier",
                    Margin::of(residual),
                    band,
                    errors,
                ) != Some(true)
                {
                    return false;
                }
            }
        }
    }
    true
}

/// The VERIFIED-INTERFACE half of the confinement: the declared pair
/// itself answers through the SAME two doors the confirm pass runs —
/// Door 1 (`contact_pair_verdict`, class `Rest`: carrier identity
/// through the kind ladder, senses opposed) and Door 2 (the
/// chart-region overlap through the [`RegionLane`] the pass holds, the
/// verdict carried between them, `interior_witness`'s rescue included) — and
/// the overlap region is definitely positive. Anything else is a
/// non-answer HERE, deliberately unreported — a door ERROR included,
/// and that asymmetry (loud doors at the confirm pass, a silent
/// `false` here) is argued rather than accidental: every pair this
/// rung consults comes out of `Declared::faces`, which is built from
/// exactly the records [`confirm_records`] walks, so the same
/// doors run on the same pair once per validation with full typed
/// reporting there; a second report from inside a backing consult
/// would duplicate every refusal once per crossing the pair is asked
/// about. What the swallow costs is the association between THIS
/// crossing and THAT pair's refusal, which the finding's own hardness
/// preserves (an unverified pair backs nothing, so the crossing stays
/// loud). The doors are also RE-RUN per consult, unmemoised: a
/// crossing-heavy seat pays the two doors once per (crossing ×
/// opposite-sided candidate pair) — correctness first, the cache is
/// PERF-PLAN's if it ever shows up in a profile. A pass holding no
/// region door (`None` — the `_structural` twin's value at every
/// scalar, a dual's only one) has no Door 2 to ask and lands in the
/// same non-answer — and that half of the swallow is the one with a
/// loud twin of its own: the confirm pass reports it as
/// [`ValidationError::CensusLaneUnsupported`], distinct from the
/// geometry refusals, so a reader of the vector can tell *this pass
/// held no region door; replay through the certified door* from *this
/// pair's geometry is outside the lane* without knowing which arm
/// swallowed what.
fn pair_region_verified<T: Decide>(
    body: &Body<T>,
    fa: FaceKey,
    fb: FaceKey,
    band: Band,
    region: Option<RegionLane<T>>,
) -> bool {
    let Ok(verdict) = crate::boolean::contact_pair_verdict(
        body,
        fa,
        body,
        fb,
        crate::contact::ContactClass::Rest,
        None,
        band,
    ) else {
        return false;
    };
    matches!(
        region.map(|lane| lane.declared_overlap(body, fa, body, fb, verdict, band)),
        Some(Ok(crate::chart_region::ChartOverlap::PositiveArea))
    )
}

/// [`pair_region_verified`] in either frame order: the certified
/// answers are frame-invariant (`world_carrier`'s lemma), and trying
/// both keeps a rung total over how the record spells the pair.
fn pair_verified<T: Decide>(
    body: &Body<T>,
    fa: FaceKey,
    fb: FaceKey,
    band: Band,
    region: Option<RegionLane<T>>,
) -> bool {
    pair_region_verified(body, fa, fb, band, region)
        || pair_region_verified(body, fb, fa, band, region)
}

/// The exact sweeps' admission test for one edge, in ONE place.
/// [`snapshot`] keeps an edge iff its carrier is a certified `Line`,
/// and `line_bounded` — the planar × planar skip's premise — asks the
/// same question of a whole face boundary. The two must not drift:
/// if snapshot's rule widens and the skip's does not, the skip keeps
/// firing on faces whose boundary is no longer in front of the
/// sweeps, which is the UNSOUND direction. Sharing the predicate is
/// what makes the coupling structural rather than remembered.
///
/// Snapshot drops one thing more than this models: a certified line
/// edge whose endpoints do not resolve (tier 1 guarantees they do, so
/// the skip's premise is not weakened by the difference in practice —
/// but the difference is real, and it is here, not implied). The
/// boolean's edge-edge rows (`boolean::touch_rows`) ask this one;
/// `validate`'s segment readers match the carrier themselves, each
/// answering its own question, not the snapshot's.
pub(crate) fn edge_is_line<T: Real>(body: &Body<T>, ek: crate::entity::EdgeKey) -> bool {
    body.edges
        .get(ek)
        .and_then(|e| body.curves.get(e.curve))
        .and_then(CurveGeom::certified)
        .map(geom_brep::EdgeCurve::carrier)
        .is_some_and(|c| matches!(c, geom::Curve3::Line { .. }))
}

/// Every loop of `face` — the outer, then the rings.
fn face_loops(face: &Face) -> impl Iterator<Item = LoopKey> + '_ {
    core::iter::once(face.outer).chain(face.rings.iter().copied())
}

/// Every loop of `face` with its walk: `Ok(cycle)` for a cycle loop
/// `body` can walk, `Err(loop)` for one it cannot — a lost loop, an
/// empty (lone-vertex) loop, an unwalkable cycle. What an unwalkable
/// loop MEANS is the caller's to say, at the call site; this walk
/// never decides it. The typed walk that `snapshot` and the backstop
/// read; the boundary-box sites read
/// [`Body::face_boundary_linked`], whose every hop is a link.
fn face_cycles<'a, T: Real>(
    body: &'a Body<T>,
    face: &'a Face,
) -> impl Iterator<Item = Result<Vec<HalfEdgeKey>, LoopKey>> + 'a {
    face_loops(face).map(move |lk| {
        let Some(loop_) = body.loops.get(lk) else {
            return Err(lk);
        };
        let LoopBoundary::Cycle { first } = loop_.boundary else {
            return Err(lk);
        };
        body.loop_cycle(first).ok_or(lk)
    })
}

/// Builds the geometry snapshot: exact planar entities for the
/// vertex-granular sweeps, curved entities routed to the
/// face-granular arms (M9-2 — the census ADMITS every carrier kind;
/// the blanket exact-on-planar refusal retired with the census arms
/// that replaced it, and what each arm can and cannot certify is the
/// module-docs envelope, stated rather than sampled). Total: every
/// entity lands in exactly one bucket, so there is no refusal path.
/// Every line edge's census geometry, in arena order. A curved-carrier
/// edge is outside the exact sweeps; its contact obligations ride the
/// curved arms and the records naming it — no blanket refusal. Tier 1
/// guarantees every link resolves; an edge one does not is skipped.
fn line_edges<T: Real>(body: &Body<T>) -> Vec<EdgeGeo<T>> {
    body.edges
        .iter()
        .filter(|&(key, _)| edge_is_line(body, key))
        .filter_map(|(key, _)| edge_geo(body, key))
        .collect()
}

/// One edge's census geometry ([`line_edges`]); `None` where a link
/// does not resolve.
fn edge_geo<T: Real>(body: &Body<T>, key: EdgeKey) -> Option<EdgeGeo<T>> {
    let edge = body.edges.get(key)?;
    let plus = body.half_edges.get(edge.he_plus)?;
    let v0 = plus.start;
    let v1 = body.half_edge_end(edge.he_plus)?;
    let (k0, k1) = (body.vertices.get(v0)?.point, body.vertices.get(v1)?.point);
    let p0 = *body.points.get(k0)?;
    let p1 = *body.points.get(k1)?;
    let sides = crate::readback::edge_sides(body, key).ok()?;
    let (f_plus, f_minus) = sides.faces();
    let chord = p1 - p0;
    Some(EdgeGeo {
        key,
        v0,
        v1,
        p0,
        p1,
        points: [Some(k0), Some(k1)],
        dir: chord.normalize(),
        len: chord.norm(),
        f_plus,
        f_minus,
    })
}

/// **Two distinct line edges whose interiors overlap along a positive
/// length**, as the census's edge-edge pass reads them: the pair, and
/// the overlap's two ends.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CollinearOverlap<T: Real> {
    /// The two edges, in arena order.
    pub(crate) edges: [EdgeKey; 2],
    /// Where the overlap begins and ends.
    pub(crate) ends: [Point3<T>; 2],
}

/// Every [`CollinearOverlap`] among `body`'s line edges: the edge-edge
/// pass's candidates ([`Trees`]' edge class), its
/// parallel screen ([`parallel_reading`]) and its overlap
/// ([`collinear_overlap`]), with no backing asked of any. Two edges on
/// one line overlap whatever backs them, so a structural pair (one point
/// key at each bound), a recorded pair and a partial overlap read alike.
///
/// The pairs are read as pass 5 reads them ([`parallel_overlap`]): an
/// in-band parallel reading is settled where the overlap reading
/// definitely refuses an overlap, and is the pair's escalation
/// otherwise.
///
/// # Errors
///
/// The first pair, in sweep order, a decision could not read, with the
/// escalation.
pub(crate) fn collinear_overlaps<T: Decide + Bounds>(
    body: &Body<T>,
    band: Band,
) -> Result<Vec<CollinearOverlap<T>>, ([EdgeKey; 2], Indeterminate)> {
    let edges = line_edges(body);
    let pad = sweep_pad(band);
    let class = Class::build(
        CensusStrategy::Realized,
        edges.iter().map(|e| edge_box(body, e.key, pad)).collect(),
    );
    let mut found = Vec::new();
    for (i, ea) in edges.iter().enumerate() {
        for j in class.later(i) {
            let eb = candidate(&edges, j);
            let mut errors = Vec::new();
            let Some(overlap) = parallel_overlap(ea, eb, band, &mut errors) else {
                return Err(([ea.key, eb.key], escalation_pushed(errors)));
            };
            if let Some(((lo, hi), _)) = overlap {
                found.push(CollinearOverlap {
                    edges: [ea.key, eb.key],
                    ends: [ea.p0 + ea.dir * lo, ea.p0 + ea.dir * hi],
                });
            }
        }
    }
    Ok(found)
}

/// The escalation a reading pushed where it returned escalated: the
/// parallel screen and the overlap push only their escalations.
fn escalation_pushed(errors: Vec<ValidationError>) -> Indeterminate {
    match errors.into_iter().next() {
        Some(ValidationError::CensusEscalated { cause }) => cause,
        other => unreachable!("an escalated reading pushes its escalation, got {other:?}"),
    }
}

/// **Every pair of line edges `body` holds on one segment by structure**:
/// two distinct line edges whose ends lie on the same two point keys
/// ([`Bound::Shared`] at both bounds of [`ee_collinear_lane`]), which
/// two lines through two points make one segment, no decision taken.
/// The collinear overlaps a decision reads beyond these (a bound on a
/// record, or one edge's end on the other's interior) are
/// [`collinear_overlaps`]'.
pub(crate) fn shared_segments<T: Real>(body: &Body<T>) -> Vec<CollinearOverlap<T>> {
    let mut by_ends: std::collections::BTreeMap<[PointKey; 2], Vec<EdgeKey>> =
        std::collections::BTreeMap::new();
    let mut ends_of = std::collections::BTreeMap::new();
    for e in line_edges(body) {
        let [Some(p), Some(q)] = e.points else {
            unreachable!("a body's line edge sits on its vertices' point keys")
        };
        let key = if p <= q { [p, q] } else { [q, p] };
        ends_of.insert(key, [e.p0, e.p1]);
        by_ends.entry(key).or_default().push(e.key);
    }
    let mut found = Vec::new();
    for (key, edges) in by_ends {
        let Some(&ends) = ends_of.get(&key) else {
            continue;
        };
        for (i, &a) in edges.iter().enumerate() {
            for &b in &edges[i + 1..] {
                found.push(CollinearOverlap {
                    edges: [a, b],
                    ends,
                });
            }
        }
    }
    found
}

/// Why [`line_wedges`] could not answer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum WedgeRefusal {
    /// A decision landed in band, or a wedge has no reading at this
    /// tolerance (a spike, an arm too short to meter). Every reading is
    /// of the surfaces of the faces meeting on the line, which a
    /// Boolean's result carries unchanged from its operands, so moving
    /// the parts reaches it: the Boolean types it as an escalation of
    /// its input ([`crate::boolean::BooleanDecision::LineWedges`]).
    Undecided(Indeterminate),
    /// A face of this edge has no tangent plane where it meets the
    /// line, so it has no sheet to read there.
    NoSheet(EdgeKey),
    /// This edge's two sheets definitely meet at a zero angle or a full
    /// turn: a cusp or a knife slit, D1's wedge ends, which this reading
    /// has no interior direction for.
    WedgeEnd(EdgeKey),
}

/// One edge's material wedge on the line: its two face sheets (the
/// unit directions square to the line into each face), the edge's
/// direction it turns about, an interior direction, and the arm its
/// readings are levered at.
struct Wedge<T: Real> {
    axis: Vec3<T>,
    own: Vec3<T>,
    next: Vec3<T>,
    inside: Vec3<T>,
    arm: T,
}

/// **The first coincident pair whose material wedges overlap**: for
/// each pair `pairs` lists ([`collinear_overlaps`]), the two edges'
/// material wedges, read at the middle of the span they share, and
/// whether they overlap. `Ok(Some(i))` names the first such pair,
/// `Ok(None)` says none does. Each edge's census geometry is read once;
/// its sheets are read per pair at the shared span, so a partial overlap
/// on a face whose tangent plane turns along a ruling compares the two
/// wedges at one point.
///
/// A wedge is the material between an edge's two face sheets, CCW
/// about the edge's own direction from its minus half's sheet to its
/// plus half's: an interior direction comes from the sector rungs
/// ([`crate::sector_shape::sector_shape`]), which refuse a spike, and
/// whether a direction lies strictly inside it is tier 3's pinch-corner
/// reading ([`crate::validate::wedge_holds_named`], under names of its
/// own, [`LINE_WEDGE`]), the same question that
/// check asks of two corners at a pinch vertex. Two wedges overlap
/// where a sheet or the interior direction of one lies strictly inside
/// the other: two arcs of a circle share an interior point exactly
/// where one holds an end or a middle of the other.
///
/// The Boolean's result gate runs it over every line where coincident
/// edges lie; it is the reading tier 3′'s side check owes there too
/// (restread's `census-passes-a-face-inside-an-edges-material-wedge`):
/// coincident edges' material must lie on opposite sides. A face that
/// crosses the line with no edge of its own there is not read: it holds
/// no sheet at the line, and finding it is that check's point-in-face
/// question.
///
/// # Errors
///
/// The pair, by its index, whose reading refused ([`WedgeRefusal`]): an
/// in-band reading, a face with no sheet, or a cusp or slit edge.
pub(crate) fn line_wedges<T: Decide>(
    body: &Body<T>,
    pairs: &[CollinearOverlap<T>],
    band: Band,
) -> Result<Option<usize>, (usize, WedgeRefusal)> {
    let mut geo = std::collections::BTreeMap::new();
    for pair in pairs {
        for key in pair.edges {
            if let std::collections::btree_map::Entry::Vacant(slot) = geo.entry(key) {
                // Tier 1 resolves every link of an edge `collinear_overlaps` read.
                let Some(e) = edge_geo(body, key) else {
                    unreachable!("a line edge the overlap read resolves: {key:?}")
                };
                slot.insert(e);
            }
        }
    }
    for (i, pair) in pairs.iter().enumerate() {
        let at = pair.ends[0] + (pair.ends[1] - pair.ends[0]) * T::from_f64(0.5);
        let read = |key: EdgeKey| wedge(body, &geo[&key], at, band).map_err(|r| (i, r));
        let (a, b) = (read(pair.edges[0])?, read(pair.edges[1])?);
        for (w, probes) in [(&a, &b), (&b, &a)] {
            for v in [probes.own, probes.next, probes.inside] {
                if strictly_inside(w, v, band).map_err(|r| (i, r))? {
                    return Ok(Some(i));
                }
            }
        }
    }
    Ok(None)
}

/// `e`'s material wedge ([`line_wedges`]), its sheets read at `at`, a
/// point of the edge.
fn wedge<T: Decide>(
    body: &Body<T>,
    e: &EdgeGeo<T>,
    at: Point3<T>,
    band: Band,
) -> Result<Wedge<T>, WedgeRefusal> {
    let normal =
        |face: FaceKey| match crate::face_normal::face_outward_normal_at(body, face, at, band) {
            Ok(Some(n)) => Ok(n),
            Err(crate::face_normal::NormalAtError::Escalated { diag, .. }) => {
                Err(WedgeRefusal::Undecided(diag))
            }
            Ok(None) | Err(_) => Err(WedgeRefusal::NoSheet(e.key)),
        };
    // The plus half runs along `e.dir` on `f_plus`, the minus half back
    // on `f_minus`; each sheet is the face's normal crossed with its
    // half's direction, which points into the face.
    let (n_plus, n_minus) = (normal(e.f_plus)?, normal(e.f_minus)?);
    let own = n_plus.vec().cross(e.dir);
    let next = n_minus.vec().cross(-e.dir);
    let arm = e.len;
    let shape = crate::sector_shape::edge_wedge_shape(own * arm, next * arm, e.dir, band).map_err(
        |fault| match fault {
            crate::sector_shape::SectorFault::Rung {
                rung: crate::sector_shape::SectorRung::Straight { .. },
                diag,
            } if diag.margin.rejected_sign().is_some() => WedgeRefusal::WedgeEnd(e.key),
            crate::sector_shape::SectorFault::Rung { diag, .. } => WedgeRefusal::Undecided(diag),
            crate::sector_shape::SectorFault::NonFiniteChord
            | crate::sector_shape::SectorFault::UnderflowedChord => WedgeRefusal::NoSheet(e.key),
        },
    )?;
    Ok(Wedge {
        axis: e.dir,
        own: shape.unit_own,
        next: shape.unit_next,
        inside: shape
            .bisector
            .unwrap_or_else(|| (shape.unit_own + shape.unit_next).normalize()),
        arm: shape.arm,
    })
}

/// Whether the unit direction `v` lies strictly inside `w`'s material
/// ([`line_wedges`]): the wedge CCW about the edge from its minus
/// half's sheet to its plus half's, every direction scaled to the
/// wedge's arm so each sign is a length.
fn strictly_inside<T: Decide>(w: &Wedge<T>, v: Vec3<T>, band: Band) -> Result<bool, WedgeRefusal> {
    crate::validate::wedge_holds_named(
        &LINE_WEDGE,
        w.axis,
        w.next * w.arm,
        w.own * w.arm,
        v * w.arm,
        band,
    )
    .map_err(WedgeRefusal::Undecided)
}

/// [`strictly_inside`]'s predicate names: tier 3's pinch-corner
/// reading, asked of a line's wedges under names of its own, so the K
/// stream keeps the two questions apart.
const LINE_WEDGE: crate::validate::WedgeNames = crate::validate::WedgeNames {
    after_leaving: LINE_WEDGE_AFTER_LEAVING,
    before_arriving: LINE_WEDGE_BEFORE_ARRIVING,
    turn: LINE_WEDGE_TURN,
    straight: LINE_WEDGE_STRAIGHT,
};

/// [`LINE_WEDGE`]'s first-bound sign.
pub(crate) const LINE_WEDGE_AFTER_LEAVING: &str = "pm_census_wedge_after_leaving";
/// [`LINE_WEDGE`]'s second-bound sign.
pub(crate) const LINE_WEDGE_BEFORE_ARRIVING: &str = "pm_census_wedge_before_arriving";
/// [`LINE_WEDGE`]'s turn.
pub(crate) const LINE_WEDGE_TURN: &str = "pm_census_wedge_turn";
/// [`LINE_WEDGE`]'s straight-or-cusp sign.
pub(crate) const LINE_WEDGE_STRAIGHT: &str = "pm_census_wedge_straight";

fn snapshot<T: Decide>(body: &Body<T>) -> Geo<T> {
    let resolved: Vec<(VertexKey, PointKey, Point3<T>)> = body
        .vertices
        .iter()
        .filter_map(|(k, v)| body.points.get(v.point).map(|p| (k, v.point, *p)))
        .collect();
    let verts: Vec<(VertexKey, Point3<T>)> = resolved.iter().map(|&(k, _, p)| (k, p)).collect();
    // A conventional vertex has no identity of its own: the sweeps
    // still read its point, and a touch there is its edge's.
    let conventional = resolved
        .iter()
        .filter(|&&(k, _, _)| crate::boolean::is_conventional_vertex(body, k))
        .filter_map(|&(k, _, _)| {
            let he = body.vertices.get(k)?.emanating?;
            Some((k, body.half_edges.get(he)?.edge))
        })
        .collect();
    let vmap = resolved.iter().map(|&(k, _, p)| (k, p)).collect();
    let vpoint = resolved.iter().map(|&(k, point, _)| (k, point)).collect();
    let edges = line_edges(body);
    let mut faces = Vec::new();
    let mut curved_faces = Vec::new();
    let mut vertex_faces: std::collections::BTreeMap<VertexKey, BTreeSet<FaceKey>> =
        std::collections::BTreeMap::new();
    for (key, face) in body.faces.iter() {
        let plane = match body.surfaces.get(face.surface) {
            Some(&geom::Surface::Plane { origin, normal, .. }) => Some((origin, normal)),
            _ => {
                curved_faces.push(key);
                None
            }
        };
        let mut boundary = BTreeSet::new();
        let mut loops = Vec::new();
        let mut polygon = true;
        // A loop this cannot walk contributes no boundary vertex, and
        // the face is no polygon the touch analysis reads.
        for cycle in face_cycles(body, face) {
            let Ok(cycle) = cycle else {
                polygon = false;
                continue;
            };
            let mut ring = Vec::with_capacity(cycle.len());
            for he in cycle {
                if let Some(he_data) = body.half_edges.get(he) {
                    boundary.insert(he_data.start);
                    ring.push(he_data.start);
                    polygon &= edge_is_line(body, he_data.edge);
                } else {
                    polygon = false;
                }
            }
            loops.push(ring);
        }
        // Incidence for the face-granularity backing rung — EVERY
        // face, curved included (a curved declared face's boundary
        // vertices are exactly the ones its record must back).
        for &v in &boundary {
            vertex_faces.entry(v).or_default().insert(key);
        }
        if let Some((origin, normal)) = plane {
            faces.push(FaceGeo {
                key,
                origin,
                normal,
                boundary,
                loops,
                polygon,
            });
        }
    }
    Geo {
        verts,
        edges,
        faces,
        vmap,
        vpoint,
        conventional,
        curved_faces,
        vertex_faces,
    }
}

impl<T: Real> Geo<T> {
    /// The line edge `e` of the snapshot; `None` for an edge that is not
    /// a line (it has no [`EdgeGeo`]).
    fn line_edge(&self, e: EdgeKey) -> Option<&EdgeGeo<T>> {
        self.edges.iter().find(|d| d.key == e)
    }

    /// Whether `a` and `b` are one vertex or sit on one point — the
    /// structural rung of D1 tier 3′ ("same surface or point key"): an
    /// op's copies of one vertex share its point, and so do the cones of
    /// a pinch (Ev, PR 4057), whose vertices may be copies of several
    /// operand vertices the op glued there by a value decision (one
    /// pierce ring serving several pierces, `boolean::vtxfac::mint_ring`).
    /// So do
    /// two vertices a boolean decided one point from values, across
    /// solids too: the touching corners of an assembly's parts share
    /// the key their vertex-vertex record ties
    /// (`boolean::ops::record_classes`). The op records each such
    /// decision (`DecisionSite::VertexFusion`) for the
    /// `unproven-coincidence` lint, whatever the result made of the
    /// vertex (`boolean::reduce::PendingRow`). The census reads keys,
    /// not lineage, so it cannot tell a pinch from a value decision and
    /// does not ask for the record.
    fn same_point(&self, a: VertexKey, b: VertexKey) -> bool {
        a == b
            || matches!(
                (self.vpoint.get(&a), self.vpoint.get(&b)),
                (Some(pa), Some(pb)) if pa == pb
            )
    }
}

/// A witness position rendered as COORDINATES (the payload posture of
/// `ResultVolumeImplausible`: display data, never load-bearing).
///
/// Coordinates rather than the point's own `Debug`, because this string
/// is pasted verbatim into a user-facing refusal: `Point3`'s derived
/// rendering carries the field braces that mark a message as `Debug`
/// guts, and the coordinate-triple SHAPE is what the rest of the tree
/// uses for a point in prose (`mate/coset.rs`, `py/readback.rs` — both
/// concrete `Point3<f64>`, so both spell it `{}`).
///
/// Each scalar keeps `{:?}` because `Real` bounds `Debug` and not
/// `Display`: there is no route to `{}` on a generic `T` here. The
/// alternative — projecting `T` to `f64` through `Bounds` — is what
/// #990 refuses, naming `fn margin_of<T: Bounds>(…) -> f64` as the
/// helper asked for and declined. (Not L7, which is the scope rule and
/// admits rendering as legitimate `Bounds` context; the refusal is
/// #990's.) `{:?}` prints `1.0` where `{}` prints `1`, so this matches
/// the tree's shape and not its spelling.
pub(crate) fn witness<T: Real>(p: Point3<T>) -> String {
    format!("({:?}, {:?}, {:?})", p.x, p.y, p.z)
}

/// The witness of a conformal-patch finding: the chart-region verdict
/// (`ChartOverlap::PositiveArea`) says the two faces share a region of
/// positive area and hands back no point in it, so the slot names that
/// region. It reads after "at" in [`ValidationError::UnrecordedContact`]'s
/// `Display`, as a coordinate does.
pub(crate) const CONFORMAL_REGION_WITNESS: &str = "their shared region";

/// The witness of a patch record Door 1 contradicts: the verdict
/// compares the two faces' carriers whole — the same surface, opposed
/// senses — and hands back no point, so the slot names where the
/// contradiction holds: everywhere. It reads after "contradicted" in
/// [`ValidationError::ContactContradicted`]'s `Display`.
pub(crate) const CARRIER_COMPARISON_WITNESS: &str = "across the whole of both faces";

/// The witness of a face–face record on two carriers the confirm pass
/// contradicts: the closed-form line their tangency would run along.
/// It reads after "contradicted" in
/// [`ValidationError::ContactContradicted`]'s `Display`.
pub(crate) const TANGENT_LOCUS_WITNESS: &str = "along their tangent line";

/// A gap margin as a trilean coincidence verdict: `Some(true)`
/// coincident, `Some(false)` apart, `None` escalated (already pushed).
/// Every caller's gap is nonnegative by construction (a norm, an `abs`
/// over a norm, a norm levered by a chord length), the magnitude door's
/// precondition.
fn gap_is_zero<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    gap_zero_reading(name, margin, band, errors).map(|zero| zero.is_some())
}

/// [`gap_is_zero`], keeping the Zero's reading: `Some(Some(margin))`
/// zero, `Some(None)` definitely positive, `None` escalated (pushed).
fn gap_zero_reading<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<MarginDiag>> {
    match geom_core::k_stats::decide_magnitude_reported(name, margin, band) {
        Ok((Magnitude::Zero, margin)) => Some(Some(margin)),
        Ok((Magnitude::Positive, _)) => Some(None),
        Err(cause) => {
            errors.push(ValidationError::CensusEscalated { cause });
            None
        }
    }
}

/// Census pass 1: vertex–vertex coincidence (candidate pairs, arena
/// order).
fn sweep_vertex_vertex<T: Decide>(
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    cands: &Candidates,
    mut trace: Option<&mut CensusTrace>,
    errors: &mut Vec<ValidationError>,
) {
    for (i, &(ka, pa)) in geo.verts.iter().enumerate() {
        for j in cands.trees.verts.later(i) {
            let &(kb, pb) = candidate(&geo.verts, j);
            let before = errors.len();
            pair_vertex_vertex(geo, declared, band, (ka, pa), (kb, pb), errors);
            if let Some(t) = trace.as_deref_mut() {
                t.vv.note(
                    (EntityId::Vertex(ka), EntityId::Vertex(kb)),
                    errors.len() > before,
                );
            }
        }
    }
}

/// One vertex pair of pass 1.
fn pair_vertex_vertex<T: Decide>(
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    (ka, pa): (VertexKey, Point3<T>),
    (kb, pb): (VertexKey, Point3<T>),
    errors: &mut Vec<ValidationError>,
) {
    let Some(zero) = gap_zero_reading("pm_census_vv_gap", Margin::norm3(pa - pb), band, errors)
    else {
        return;
    };
    if let Some(margin) = zero
        && !geo.same_point(ka, kb)
        && !declared.vv.contains(&(ka, kb))
        && !declared.vv_face_backed(geo, ka, kb)
        && !declared.at_conventional(geo, ka, EntityId::Vertex(kb))
        && !declared.at_conventional(geo, kb, EntityId::Vertex(ka))
    {
        errors.push(ValidationError::UnrecordedContact {
            contact: CensusContact::VertexVertex { a: ka, b: kb },
            witness: witness(pa),
            margin: Some(margin),
        });
    }
}

/// Census pass 2: vertex on an edge's **interior** — backed by its
/// `(vertex, edge)` record or the face rung (module docs, D4), and
/// otherwise a hard finding.
fn sweep_vertex_edge<T: Decide>(
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    cands: &Candidates,
    mut trace: Option<&mut CensusTrace>,
    errors: &mut Vec<ValidationError>,
) {
    for (i, &(vk, q)) in geo.verts.iter().enumerate() {
        for j in cands.trees.edges.overlapping(cands.trees.verts.query(i)) {
            let e = candidate(&geo.edges, j);
            if vk == e.v0 || vk == e.v1 {
                continue; // structural adjacency
            }
            let before = errors.len();
            pair_vertex_edge(geo, declared, band, (vk, q), e, errors);
            if let Some(t) = trace.as_deref_mut() {
                t.ve.note(
                    (EntityId::Vertex(vk), EntityId::Edge(e.key)),
                    errors.len() > before,
                );
            }
        }
    }
}

/// One non-adjacent vertex–edge pair of pass 2.
fn pair_vertex_edge<T: Decide>(
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    (vk, q): (VertexKey, Point3<T>),
    e: &EdgeGeo<T>,
    errors: &mut Vec<ValidationError>,
) {
    if let Some(Some((_, margin))) = on_edge_interior_reading(q, e, band, errors)
        && !declared.ve_recorded(vk, e.key)
        && !declared.ve_face_backed(geo, vk, (e.f_plus, e.f_minus))
        && !declared.at_conventional(geo, vk, EntityId::Edge(e.key))
    {
        errors.push(ValidationError::UnrecordedContact {
            contact: CensusContact::VertexOnEdge {
                vertex: vk,
                edge: e.key,
            },
            witness: witness(q),
            margin: Some(margin),
        });
    }
}

/// Census pass 2′: a vertex on a **curved** edge's interior, the
/// question pass 2 asks of a line, read through the curved lane
/// ([`curved::on_curved_interior`]) and backed alike: by its
/// `(vertex, edge)` record, the face rung, or a conventional vertex's
/// edge-edge record. A vertex at one of the edge's ends is pass 1's.
/// Candidates are the pairs whose padded boxes meet; of those, a vertex
/// the span's pieces all clear ([`curved::may_rest_on`]) is passed by
/// with no decision pushed, so an edge the reading would refuse is
/// refused only for a vertex that reaches it.
fn sweep_vertex_curved_edge<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    cands: &Candidates,
    mut trace: Option<&mut CensusTrace>,
    errors: &mut Vec<ValidationError>,
) {
    // Each curved edge's ends and sides. `Candidates::build` boxed every
    // one through `edge_box`, which panics on a link that does not
    // resolve; a side read that refuses is passed by, as the snapshot's.
    let edges: Vec<_> = cands
        .curved
        .iter()
        .map(|&ek| {
            let edge = body.edges.get(ek)?;
            let v0 = body.half_edges.get(edge.he_plus)?.start;
            let v1 = body.half_edge_end(edge.he_plus)?;
            let point = |v| {
                body.vertices
                    .get(v)
                    .and_then(|d| body.points.get(d.point))
                    .copied()
            };
            let sides = crate::readback::edge_sides(body, ek).ok()?.faces();
            Some((ek, [(v0, point(v0)?), (v1, point(v1)?)], sides))
        })
        .collect();
    for (i, &(vk, q)) in geo.verts.iter().enumerate() {
        for j in cands.trees.curved.overlapping(cands.trees.verts.query(i)) {
            let Some((ek, ends, sides)) = *candidate(&edges, j) else {
                continue;
            };
            if ends.iter().any(|&(v, _)| v == vk) {
                continue; // structural adjacency
            }
            let before = errors.len();
            pair_vertex_curved_edge(
                body,
                declared,
                geo,
                band,
                (vk, q),
                (ek, ends, sides),
                errors,
            );
            if let Some(t) = trace.as_deref_mut() {
                t.vce.note(
                    (EntityId::Vertex(vk), EntityId::Edge(ek)),
                    errors.len() > before,
                );
            }
        }
    }
}

/// One non-adjacent vertex–curved-edge pair of pass 2′.
#[allow(clippy::type_complexity)] // the edge's key, ends and sides, read once per edge
fn pair_vertex_curved_edge<T: Decide>(
    body: &Body<T>,
    declared: &Declared,
    geo: &Geo<T>,
    band: Band,
    (vk, q): (VertexKey, Point3<T>),
    (ek, ends, sides): (EdgeKey, [(VertexKey, Point3<T>); 2], (FaceKey, FaceKey)),
    errors: &mut Vec<ValidationError>,
) {
    // A vertex at an end, or in the band of one, is pass 1's: it
    // decides the pair, and escalates the band itself.
    let clear_of_end = |p: Point3<T>| {
        matches!(
            geom_core::k_stats::decide("pm_census_ve_curved_sweep_end", Margin::norm3(q - p), band),
            Ok(Sign::Positive | Sign::Negative)
        )
    };
    if !ends.iter().all(|&(_, p)| clear_of_end(p)) || !curved::may_rest_on(body, ek, q, band) {
        return;
    }
    if let Some(Some(margin)) = curved::on_curved_interior_reading(body, ek, q, band, errors)
        && !declared.ve_recorded(vk, ek)
        && !declared.ve_face_backed(geo, vk, sides)
        && !declared.at_conventional(geo, vk, EntityId::Edge(ek))
    {
        errors.push(ValidationError::UnrecordedContact {
            contact: CensusContact::VertexOnEdge {
                vertex: vk,
                edge: ek,
            },
            witness: witness(q),
            margin: Some(margin),
        });
    }
}

/// Whether `q` lies on `e`'s interior: [`on_edge_interior_reading`]
/// placing it on the edge. `None` where a decision escalated (pushed).
/// Pass 2 and the confirm pass of a `(vertex, edge)` record ask this one
/// question.
fn on_edge_interior<T: Decide>(
    q: Point3<T>,
    e: &EdgeGeo<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    on_edge_interior_reading(q, e, band, errors).map(|on| on.is_some())
}

/// `q` against `e`'s segment by [`Span::contact`], the one point-on-span
/// reading, at its foot on the line ([`geom::Curve3::foot`]):
/// `Some(Some((s, margin)))` on the interior at arc length `s`, by the
/// margin that put it on the line; `Some(None)` off the interior, an end
/// included; `None` escalated (pushed).
fn on_edge_interior_reading<T: Decide>(
    q: Point3<T>,
    e: &EdgeGeo<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<(T, MarginDiag)>> {
    let line = geom::Curve3::Line {
        origin: e.p0,
        dir: e.dir,
    };
    match CarrierSpan::new(&line, (T::zero(), e.len)).contact(q, T::zero(), band) {
        Ok(Some(Hit::On { t, margin })) => Some(Some((t, margin))),
        Ok(Some(Hit::End(_)) | None) => Some(None),
        Err(geom::CarrierRefusal::Escalated(cause) | geom::CarrierRefusal::Straddle(cause)) => {
            errors.push(ValidationError::CensusEscalated { cause });
            None
        }
        Err(geom::CarrierRefusal::Exhausted | geom::CarrierRefusal::Unbounded) => {
            unreachable!("a line's carrier reading walks no pieces")
        }
    }
}

/// A signed residual as a trilean on-verdict: `Some(true)` on,
/// `Some(false)` definitely off, `None` escalated (pushed).
fn signed_is_zero<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    signed_zero_reading(name, margin, band, errors).map(|zero| zero.is_some())
}

/// [`signed_is_zero`], keeping the Zero's reading, as
/// [`gap_zero_reading`] does.
fn signed_zero_reading<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<MarginDiag>> {
    match geom_core::k_stats::decide_reported(name, margin, band) {
        Ok(geom_core::Decided {
            sign: Sign::Zero,
            margin,
        }) => Some(Some(margin)),
        Ok(_) => Some(None),
        Err(cause) => {
            errors.push(ValidationError::CensusEscalated { cause });
            None
        }
    }
}

/// `contfp` with census escalation plumbing (`None` = escalated,
/// already pushed).
fn contain<T: Decide>(
    body: &Body<T>,
    f: &FaceGeo<T>,
    q: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<FaceContainment> {
    read_containment(f.key, contfp(body, f.key, f.normal, q, band), errors)
}

/// A containment door's answer about `face`, its refusal pushed
/// (`None`): the census's one routing of [`ContainError`], for the
/// planar door and the curved one alike.
fn read_containment<V>(
    face: FaceKey,
    read: Result<V, ContainError>,
    errors: &mut Vec<ValidationError>,
) -> Option<V> {
    match read {
        Ok(v) => Some(v),
        Err(ContainError::Escalated { diag: cause, .. }) => {
            errors.push(ValidationError::CensusEscalated { cause });
            None
        }
        // A loop with an edge no walk crosses, an exhausted ray
        // schedule, a loop with no region or no walk, a curved chart
        // read, an arc's end its corners disagree with: refusals that
        // left no margin undecided, CARRIED rather than replaced. An
        // escalation is what a predicate says when it measured and
        // could not decide, so minting one for a door that did not
        // put a fabricated quantity in the field a reader judges the
        // call by.
        //
        // The subject is the FACE, which is the one entity every
        // caller of this helper shares: the second entity is a vertex
        // at one call site, an edge midpoint at another and a
        // plane-crossing point at a third, while the refusal is always
        // about whether THIS face's region can be read at all.
        //
        // Listed rather than wildcarded, because this arm is one half
        // of the `Escalated`/`Unsupported` discrimination
        // `editor_core::attribute` classifies: a new `ContainError`
        // arm must be routed here deliberately rather than default
        // into the wrong half.
        Err(ContainError::StaleFace(face)) => crate::boolean::driver_face_stale(face),
        // The census reads at the run's band a body tier 3 passed, and
        // tier 3 pins every carrier end within that band's zero of its
        // corner, so an end read off both corners is the kernel's two
        // readings of one end parting (rounding): a defect, not a reading.
        Err(ContainError::ArcEndClearOfCorners(gaps)) if gaps.off_corner() => {
            errors.push(ValidationError::ArcEndOffCorner { face, gaps });
            None
        }
        Err(
            e @ (ContainError::Uncrossable(_)
            | ContainError::RayExhausted
            | ContainError::EmptyLoop(_)
            | ContainError::LoopUnreadable(_)
            | ContainError::Curved(_)
            | ContainError::ArcEndClearOfCorners(_)),
        ) => {
            errors.push(ValidationError::CensusUnsupported {
                subject: CensusSubject::Entity(EntityId::Face(face)),
                cause: CensusUnsupportedCause::Containment(e),
            });
            None
        }
    }
}

/// Census pass 3: vertex on a face's **interior** (strictly inside the
/// region — boundary coincidences are pass-1/2 findings).
fn sweep_vertex_face<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    cands: &Candidates,
    mut trace: Option<&mut CensusTrace>,
    errors: &mut Vec<ValidationError>,
) {
    for (i, &(vk, q)) in geo.verts.iter().enumerate() {
        for j in cands.trees.faces.overlapping(cands.trees.verts.query(i)) {
            let f = candidate(&geo.faces, j);
            if f.boundary.contains(&vk) {
                continue; // structural adjacency
            }
            let before = errors.len();
            pair_vertex_face(body, geo, declared, band, (vk, q), f, errors);
            if let Some(t) = trace.as_deref_mut() {
                t.vf.note(
                    (EntityId::Vertex(vk), EntityId::Face(f.key)),
                    errors.len() > before,
                );
            }
        }
    }
}

/// One vertex–face pair of pass 3, the vertex off the face's boundary.
/// A residual in the band's edge is read by [`read_undecided`]: off the
/// plane the pair finds nothing, so it escalates only where the vertex
/// on the plane would find something.
fn pair_vertex_face<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    (vk, q): (VertexKey, Point3<T>),
    f: &FaceGeo<T>,
    errors: &mut Vec<ValidationError>,
) {
    // Boundary coincidences are pass-1/2 findings; Out and
    // escalations (pushed) need nothing more here.
    let on_plane = |margin, errors: &mut Vec<ValidationError>| {
        if contain(body, f, q, band, errors) == Some(FaceContainment::In)
            && !declared.vf.contains(&(vk, f.key))
            && !declared.vf_face_backed(geo, vk, f.key)
        {
            errors.push(ValidationError::UnrecordedContact {
                contact: CensusContact::VertexOnFace {
                    vertex: vk,
                    face: f.key,
                },
                witness: witness(q),
                margin,
            });
        }
    };
    let residual = (q - f.origin).dot(f.normal);
    let mut refused = Vec::new();
    match signed_zero_reading(
        "pm_census_vf_residual",
        Margin::of(residual),
        band,
        &mut refused,
    ) {
        Some(Some(margin)) => on_plane(Some(margin), errors),
        Some(None) => {}
        None => {
            let on = |()| {
                let mut found = Vec::new();
                on_plane(None, &mut found);
                found
            };
            if let Some(carried) = read_undecided([()], on) {
                errors.extend(refused);
                errors.extend(carried);
            }
        }
    }
}

/// Trilean comparison for span bookkeeping (`None` = escalated,
/// pushed).
fn tri_cmp<T: Decide>(
    a: T,
    b: T,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<core::cmp::Ordering> {
    match decide("pm_census_span_order", Margin::of(a - b), band) {
        Ok(Sign::Positive) => Some(core::cmp::Ordering::Greater),
        Ok(Sign::Negative) => Some(core::cmp::Ordering::Less),
        Ok(Sign::Zero) => Some(core::cmp::Ordering::Equal),
        Err(cause) => {
            errors.push(ValidationError::CensusEscalated { cause });
            None
        }
    }
}

/// The edge's vertex at span position `s` along it, if `s` is one of
/// its ends within ε (`None`: the position is interior — the bound
/// then has no carrying vertex on this edge).
fn edge_vertex_at<T: Decide>(
    e: &EdgeGeo<T>,
    s: T,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<VertexKey> {
    match signed_is_zero("pm_census_bound_end", Margin::of(s), band, errors) {
        Some(true) => return Some(e.v0),
        Some(false) => {}
        None => return None,
    }
    match signed_is_zero("pm_census_bound_end", Margin::of(e.len - s), band, errors) {
        Some(true) => Some(e.v1),
        _ => None,
    }
}

/// Some boundary vertex of `f` sitting AT the point `q` satisfies
/// `backs` — the census's own coincidence test, short-circuiting on the
/// first vertex that backs (escalations pushed as they are decided).
///
/// At a vertex cut, `q` is where [`on_edge_interior_reading`] put the
/// vertex's foot, so the distance read here is the gap that reading
/// decided `Zero` (`curve_carrier_gap`), recomputed from `q`: one
/// quantity in two spellings, which can part by a rounding at the band's
/// edge. A parting is loud (the bound goes unbacked, or an escalation is
/// pushed), never a silent pass. The helper also serves a bound at the
/// edge's own end, where no point-on-span reading was made.
fn any_boundary_vertex_at<T: Decide>(
    f: &FaceGeo<T>,
    geo: &Geo<T>,
    q: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
    mut backs: impl FnMut(VertexKey) -> bool,
) -> bool {
    for &w in &f.boundary {
        let Some(&pw) = geo.vmap.get(&w) else {
            continue;
        };
        if gap_is_zero(
            "pm_census_bound_vertex",
            Margin::norm3(pw - q),
            band,
            errors,
        ) == Some(true)
            && backs(w)
        {
            return true;
        }
    }
    false
}

/// D3 backing for one bound of an edge-on-face overlap (module docs),
/// at the two granularities a bound can have.
///
/// Its face-pair arms hold the unified strength: a declared pair backs
/// the bound only where the pair verifies, the bound lying in the
/// closed intersection of its two regions by incidence
/// ([`Declared::vf_face_backed_confined`]). Each is read after the
/// record and structural arms of its bound, because the region doors it
/// runs are the dearest reads here; it pushes nothing.
///
/// Where a boundary edge of `f` crosses the edge at the bound, the
/// event is that `EdgeEdgeCross`, and it takes that lane's backing: an
/// op's edge-edge record, or the crossing rung ([`ee_cross_backed`]). A
/// curved boundary edge's crossing is pass 5′'s event
/// ([`sweep_curved_edge_edge`]), and backs this bound by its record
/// alone: the face rung pass 5′ grandfathers holds no region, and this
/// rung's face-pair arms are confined.
///
/// Where the EDGE holds a vertex at the bound, the event is that vertex
/// against `f`: v-on-f-declared on `f`, v-v-declared with a coincident
/// boundary vertex of `f`, face-backed onto `f` inside the pair's
/// interface, `(vertex, edge)`-recorded
/// onto an edge bounding `f` (it rests on `f`'s boundary), or the
/// vertex is itself on `f`'s boundary or shares its point with a
/// boundary vertex of `f` at the bound (structural).
///
/// Where it does not, a boundary vertex of `f` rests at the bound: the
/// event is a vertex-on-edge, and it takes that lane's backing
/// (its `(vertex, edge)` record, or the same declared face pair one
/// incidence step further out, inside its interface), as
/// [`ee_bound_backed`]'s asymmetric arm reads it (grandfathered there)
/// for a collinear overlap. A bound is a bound
/// of the overlap because some entity ends there; which side's entity
/// that is, is a fact about the configuration, not about what a
/// declaration can hold.
///
/// [`edge_vertex_at`] answers `None` for TWO reasons — an interior
/// position, and an escalated span decide — and this arm is selected by
/// the `None`, so an escalated bound can reach the rung and be backed
/// where it was refused before. That is louder, not weaker: the
/// escalation is already pushed as [`ValidationError::CensusEscalated`],
/// which refuses the body on its own, and it is the arm's caveat rather
/// than the module docs' because it is a property of this call site.
#[allow(clippy::too_many_arguments)] // the rung's whole state, no less
fn ef_bound_backed<T: Decide>(
    body: &Body<T>,
    e: &EdgeGeo<T>,
    f: &FaceGeo<T>,
    cut: Cut<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    region: Option<RegionLane<T>>,
    errors: &mut Vec<ValidationError>,
) -> bool {
    let q = e.p0 + e.dir * cut.s;
    match cut.at {
        CutAt::Crossing(g) => {
            return declared.ee_recorded(e.key, g)
                || geo.line_edge(g).is_some_and(|g| {
                    matches!(
                        ee_cross_backed(body, geo, declared, e, g, q, band, region, errors),
                        CrossingBacking::Backed
                    )
                });
        }
        CutAt::CurvedCrossing(g) => return declared.ee_recorded(e.key, g),
        CutAt::Vertex => {}
    }
    let confined = |v, g| declared.vf_face_backed_confined(body, geo, v, g, band, region);
    let Some(ve) = edge_vertex_at(e, cut.s, band, errors) else {
        return any_boundary_vertex_at(f, geo, q, band, errors, |w| {
            declared.ve_recorded(w, e.key) || confined(w, e.f_plus) || confined(w, e.f_minus)
        });
    };
    if declared.vf.contains(&(ve, f.key))
        || f.boundary.contains(&ve)
        || geo
            .edges
            .iter()
            .any(|g| (g.f_plus == f.key || g.f_minus == f.key) && declared.ve_recorded(ve, g.key))
    {
        return true;
    }
    any_boundary_vertex_at(f, geo, q, band, errors, |w| {
        geo.same_point(ve, w) || declared.vv.contains(&(ve, w))
    }) || confined(ve, f.key)
}

/// Census pass 4: edge × face — transversal pierces (undeclarable) and
/// in-plane overlap segments (D3-certified).
#[allow(clippy::too_many_arguments)] // the census's fixed sweep signature plus the region door the rung consults
fn sweep_edge_face<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    region: Option<RegionLane<T>>,
    cands: &Candidates,
    mut trace: Option<&mut CensusTrace>,
    errors: &mut Vec<ValidationError>,
) {
    for (i, e) in geo.edges.iter().enumerate() {
        for j in cands.trees.faces.overlapping(cands.trees.edges.query(i)) {
            let f = candidate(&geo.faces, j);
            if f.key == e.f_plus || f.key == e.f_minus {
                continue; // structural adjacency
            }
            let before = errors.len();
            pair_edge_face(body, geo, declared, band, region, e, f, errors);
            if let Some(t) = trace.as_deref_mut() {
                t.ef.note(
                    (EntityId::Edge(e.key), EntityId::Face(f.key)),
                    errors.len() > before,
                );
            }
        }
    }
}

/// One edge–face pair of pass 4, the edge not bounding the face.
///
/// An end whose residual is in the band's edge is read by
/// [`read_undecided`], under every sign it could take
/// ([`EfPair::hypothetical`]).
#[allow(clippy::too_many_arguments)] // one pair of pass 4, the region door riding to the rung
fn pair_edge_face<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    region: Option<RegionLane<T>>,
    e: &EdgeGeo<T>,
    f: &FaceGeo<T>,
    errors: &mut Vec<ValidationError>,
) {
    let p1 = e.p0 + e.dir * e.len;
    let r = [
        (e.p0 - f.origin).dot(f.normal),
        (p1 - f.origin).dot(f.normal),
    ];
    let read = r
        .map(|r| geom_core::k_stats::decide_reported("pm_census_ef_residual", Margin::of(r), band));
    let pair = EfPair {
        body,
        geo,
        declared,
        band,
        region,
        e,
        f,
        r,
    };
    if let [Ok(d0), Ok(d1)] = &read {
        pair.on_signs(
            [d0.sign, d1.sign],
            Some(d0.margin.jointly(d1.margin)),
            errors,
        );
        return;
    }
    let readings = [0, 1].map(|i| match &read[i] {
        Ok(d) => vec![d.sign],
        Err(_) => undecided_signs(r[i]),
    });
    // Both ends able to read either side give `[P, N]` and `[N, P]`, one
    // crossing placed by one lever: read it once.
    let both_ways = readings[0].contains(&Sign::Positive) && readings[1].contains(&Sign::Negative);
    let signs = readings[0]
        .iter()
        .flat_map(|&s0| readings[1].iter().map(move |&s1| [s0, s1]))
        .filter(|&s| !(both_ways && s == [Sign::Negative, Sign::Positive]));
    let under = |signs| {
        let mut found = Vec::new();
        pair.hypothetical(signs, &mut found);
        found
    };
    if let Some(carried) = read_undecided(signs, under) {
        errors.extend(read.into_iter().filter_map(|d| {
            d.err()
                .map(|cause| ValidationError::CensusEscalated { cause })
        }));
        errors.extend(carried);
    }
}

/// The signs a residual `r` its band left undecided can read: `Zero`,
/// and the side `r` itself lies on, read at the least band. Where that
/// side cannot be read (an enclosure across zero), all three.
fn undecided_signs<T: Decide>(r: T) -> Vec<Sign> {
    match decide("pm_census_ef_residual_side", Margin::of(r), Band::least()) {
        Ok(side @ (Sign::Positive | Sign::Negative)) => vec![Sign::Zero, side],
        Ok(Sign::Zero) | Err(_) => vec![Sign::Negative, Sign::Zero, Sign::Positive],
    }
}

/// **A pair read where one of its margins is undecided**: `branch` run
/// under every reading the margin can take. `None` where no branch
/// [`Branch::finds`] anything, so the pair holds nothing whichever
/// reading is right and the margin's escalation is moot. Otherwise what
/// every branch [`Branch::carries`], for the caller to refuse with beside
/// the margin's own escalation (each branch's, not the first finding
/// one's alone, so the refusal holds every cause a reading met,
/// `terminal_sliver` included).
///
/// It escalates where ANY reading finds something. That is "the readings
/// differ" exactly where the margin's off reading finds nothing, which
/// holds at each caller here: a residual or a chord read off finds no
/// contact and no cut.
fn read_undecided<R, B: Branch>(
    readings: impl IntoIterator<Item = R>,
    branch: impl FnMut(R) -> B,
) -> Option<Vec<ValidationError>> {
    let answers: Vec<B> = readings.into_iter().map(branch).collect();
    answers
        .iter()
        .any(Branch::finds)
        .then(|| answers.into_iter().flat_map(Branch::carries).collect())
}

/// What one branch of [`read_undecided`] answers.
trait Branch {
    /// Whether the branch found anything: a finding, a refusal, a cut.
    fn finds(&self) -> bool;
    /// What a refusal made beside the undecided margin carries of it:
    /// every refusal and defect the branch met, each of which reads the
    /// body's own cells or can only add to that escalation. An unrecorded
    /// contact, or a cut, holds only under the branch's reading, and is
    /// dropped.
    fn carries(self) -> Vec<ValidationError>;
}

/// A branch that pushes what it found.
impl Branch for Vec<ValidationError> {
    fn finds(&self) -> bool {
        !self.is_empty()
    }
    fn carries(self) -> Vec<ValidationError> {
        self.into_iter()
            .filter(|e| !matches!(e, ValidationError::UnrecordedContact { .. }))
            .collect()
    }
}

/// A branch that answers cuts (`None` where it refused) beside what it
/// pushed.
impl<C> Branch for (Option<Vec<C>>, Vec<ValidationError>) {
    fn finds(&self) -> bool {
        !matches!(&self.0, Some(cuts) if cuts.is_empty()) || self.1.finds()
    }
    fn carries(self) -> Vec<ValidationError> {
        self.1.carries()
    }
}

/// One edge–face pair of pass 4 and its ends' two plane residuals.
struct EfPair<'a, T: Decide> {
    body: &'a Body<T>,
    geo: &'a Geo<T>,
    declared: &'a Declared,
    band: Band,
    region: Option<RegionLane<T>>,
    e: &'a EdgeGeo<T>,
    f: &'a FaceGeo<T>,
    r: [T; 2],
}

impl<T: Decide> EfPair<'_, T> {
    /// [`Self::on_signs`] under signs an undecided residual can take
    /// ([`undecided_signs`]). An end read strictly on one side lies past
    /// the band's zero `z` off the plane: past its escalation if decided,
    /// and past `z` if undecided, whose residual lies between the two. So
    /// two ends on opposite strict sides differ by more than `2z` (more
    /// than `esc + z` with one decided). A difference read `Zero`, within
    /// `z`, rules that pair of signs out, and one read definite is a
    /// divisor the crossing can be placed by.
    fn hypothetical(&self, signs: [Sign; 2], errors: &mut Vec<ValidationError>) {
        if let [Sign::Positive, Sign::Negative] | [Sign::Negative, Sign::Positive] = signs {
            let [r0, r1] = self.r;
            match decide("pm_census_ef_pierce_lever", Margin::of(r0 - r1), self.band) {
                Ok(Sign::Zero) => return,
                Ok(Sign::Positive | Sign::Negative) => {}
                Err(cause) => {
                    errors.push(ValidationError::CensusEscalated { cause });
                    return;
                }
            }
        }
        self.on_signs(signs, None, errors);
    }

    /// What the pair finds with its ends read `signs` off the plane,
    /// `margin` the two readings' joint margin where both were decided.
    fn on_signs(
        &self,
        signs: [Sign; 2],
        margin: Option<MarginDiag>,
        errors: &mut Vec<ValidationError>,
    ) {
        let (body, e, f, [r0, r1]) = (self.body, self.e, self.f, self.r);
        let p1 = e.p0 + e.dir * e.len;
        match signs {
            [Sign::Positive, Sign::Negative] | [Sign::Negative, Sign::Positive] => {
                // Where the plane meets the edge's line: inside the
                // segment where the signs are the ends' own, possibly off
                // it under [`Self::hypothetical`], which reads it all the
                // same and so can only keep an escalation.
                let q = e.p0 + (p1 - e.p0) * (r0 / (r0 - r1));
                // OnEdge → the edge-edge pass's crossing finding;
                // OnVertex → the v-on-e finding; Out/escalated →
                // nothing more here.
                //
                // NO backing rung is consulted, deliberately: a
                // transverse dive through a face's interior is
                // interpenetration however the seat is declared
                // (between two copies at rest, the assembly
                // layer's interference evidence); the crossing
                // rung [`ee_cross_backed`] is the in-contact-plane
                // case and does not reach here.
                if contain(body, f, q, self.band, errors) == Some(FaceContainment::In) {
                    errors.push(ValidationError::UnrecordedContact {
                        contact: CensusContact::EdgeFacePierce {
                            edge: e.key,
                            face: f.key,
                        },
                        witness: witness(q),
                        margin: None,
                    });
                }
            }
            [Sign::Zero, Sign::Zero] => ef_overlap_lane(
                body,
                (e, f, margin),
                self.geo,
                self.declared,
                self.band,
                self.region,
                errors,
            ),
            // One endpoint on the plane: pass-1/2/3 territory.
            _ => {}
        }
    }
}

/// The in-plane overlap lane of pass 4: cut the edge's span at the
/// face's coincident boundary vertices and where its boundary crosses
/// the edge, probe each cell midpoint, and D3-certify every `In` cell.
#[allow(clippy::too_many_arguments)] // the lane's whole state, the region door riding to the rung
fn ef_overlap_lane<T: Decide>(
    body: &Body<T>,
    (e, f, margin): (&EdgeGeo<T>, &FaceGeo<T>, Option<MarginDiag>),
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    region: Option<RegionLane<T>>,
    errors: &mut Vec<ValidationError>,
) {
    for cell in ef_overlap_cells(body, e, f, geo, band, errors) {
        let bound = |cut, errors: &mut Vec<ValidationError>| {
            ef_bound_backed(body, e, f, cut, geo, declared, band, region, errors)
        };
        if !(bound(cell.lo, errors) && bound(cell.hi, errors)) {
            errors.push(ValidationError::UnrecordedContact {
                contact: CensusContact::EdgeFaceOverlap {
                    edge: e.key,
                    face: f.key,
                },
                witness: witness(cell.mid),
                margin,
            });
        }
    }
}

/// Where a cut of an edge's span in a face's plane sits.
#[derive(Clone, Copy, Debug)]
enum CutAt {
    /// At one of the edge's ends or a boundary vertex of the face on
    /// its line: the entities there are read by position
    /// ([`ef_bound_backed`]).
    Vertex,
    /// Where a straight boundary edge of the face crosses the edge,
    /// strictly inside both: the pass-5 `EdgeEdgeCross` event.
    Crossing(EdgeKey),
    /// Where a curved boundary edge of the face crosses the edge,
    /// strictly inside both: the pass-5′ event of that pair's interiors
    /// meeting.
    CurvedCrossing(EdgeKey),
}

impl CutAt {
    /// A fixed order over what can sit at one point: a vertex, then
    /// straight crossings by edge key, then curved crossings by edge key.
    fn rank(self) -> (u8, Option<EdgeKey>) {
        match self {
            Self::Vertex => (0, None),
            Self::Crossing(g) => (1, Some(g)),
            Self::CurvedCrossing(g) => (2, Some(g)),
        }
    }
}

/// One cut of an edge's span: its arc length from the edge's start,
/// and what sits there.
#[derive(Clone, Copy, Debug)]
struct Cut<T> {
    s: T,
    at: CutAt,
}

/// One cell of an edge-on-face overlap: two consecutive cuts, and the
/// midpoint the face holds `In`.
struct Cell<T: Real> {
    lo: Cut<T>,
    hi: Cut<T>,
    mid: Point3<T>,
}

/// The rows the face's loop is built under for the crossing cuts: a
/// conic arc's span by [`crate::splitting::containment::carrier_loop`].
/// The crossings themselves are the meeting reading's
/// ([`edge_crossings`]).
const EF_CROSS_CONIC: crate::splitting::containment::ConicRows =
    crate::splitting::containment::ConicRows {
        span: "pm_census_ef_cross_arc_span",
        straddle: "pm_census_ef_cross_arc_straddle",
    };

/// The cells of edge `e` lying in face `f`'s interior: the edge cut at
/// every face vertex on its interior ([`on_edge_interior_reading`]) and
/// at every crossing of the face's
/// boundary ([`boundary_crossings`]), each cell whose midpoint the face
/// holds `In` — the overlap [`ef_overlap_lane`] reports, and the points
/// the touch analysis reads it at.
///
/// No boundary crosses a cell's open span on a decided reading, so a
/// cell lies inside or outside the face whole and its midpoint answers
/// for all of it. A crossing that could not be decided refuses the
/// pair's lane (pushed) rather than leave a cell straddling it.
fn ef_overlap_cells<T: Decide>(
    body: &Body<T>,
    e: &EdgeGeo<T>,
    f: &FaceGeo<T>,
    geo: &Geo<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Vec<Cell<T>> {
    let mut out = Vec::new();
    let end = |s| Cut {
        s,
        at: CutAt::Vertex,
    };
    let mut cuts: Vec<Cut<T>> = vec![end(T::zero()), end(e.len)];
    for &w in &f.boundary {
        let Some(&pw) = geo.vmap.get(&w) else {
            continue;
        };
        // A vertex on the interior cuts it there; at an end it is that
        // end's own cut, and off the segment it cuts nothing.
        if let Some(Some((s, _))) = on_edge_interior_reading(pw, e, band, errors) {
            cuts.push(end(s));
        }
    }
    let Some(crossings) = boundary_crossings(body, e, f, band, errors) else {
        return out;
    };
    cuts.extend(crossings);
    // Insertion sort through the trilean comparator (tiny lists); an
    // escalated comparison aborts this pair's lane (already reported).
    // Coincident cuts are ordered by what sits there
    // ([`CutAt::rank`]), so which one bounds the cell on either side is
    // a function of the cuts, never of the order they were found in.
    for i in 1..cuts.len() {
        let mut j = i;
        while j > 0 {
            let swap = match tri_cmp(cuts[j - 1].s, cuts[j].s, band, errors) {
                Some(core::cmp::Ordering::Greater) => true,
                Some(core::cmp::Ordering::Equal) => cuts[j - 1].at.rank() > cuts[j].at.rank(),
                Some(core::cmp::Ordering::Less) => false,
                None => return out,
            };
            if !swap {
                break;
            }
            cuts.swap(j - 1, j);
            j -= 1;
        }
    }
    for pair in cuts.windows(2) {
        let (lo, hi) = (pair[0], pair[1]);
        // The sort decided each adjacent pair's order on this same
        // difference, so this reads `Zero` (an empty cell) or `Positive`;
        // an escalation here is pushed all the same, never passed by.
        match decide("pm_census_span_gap", Margin::of(hi.s - lo.s), band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) => continue,
            Err(cause) => {
                errors.push(ValidationError::CensusEscalated { cause });
                continue;
            }
        }
        let mid = e.p0 + e.dir * geom::mid_param(lo.s, hi.s);
        if contain(body, f, mid, band, errors) != Some(FaceContainment::In) {
            // Out: no overlap here. OnEdge: a collinear boundary rest —
            // the edge-edge overlap pass's finding. OnVertex: degenerate
            // cell, vertex passes cover it.
            continue;
        }
        out.push(Cell { lo, hi, mid });
    }
    out
}

/// Every point strictly inside `e`'s span where `f`'s boundary crosses
/// it away from its vertices, as cuts; `None` where the boundary could
/// not be read or a crossing could not be decided (refusal pushed). Each
/// loop is read by [`crate::splitting::containment::carrier_loop`], and
/// each of its edges against `e` by [`edge_crossings`].
fn boundary_crossings<T: Decide>(
    body: &Body<T>,
    e: &EdgeGeo<T>,
    f: &FaceGeo<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Vec<Cut<T>>> {
    use crate::splitting::containment::carrier_loop;
    let refused =
        |r: CrossRefusal, errors: &mut Vec<ValidationError>| refuse_crossing(f.key, r, errors);
    let Some(face) = body.get_face(f.key) else {
        refused(
            CrossRefusal::Unsupported(ContainError::StaleFace(f.key)),
            errors,
        );
        return None;
    };
    let mut cuts = Vec::new();
    for lk in face_loops(face) {
        let lp = match carrier_loop(body, lk, EF_CROSS_CONIC, band) {
            Ok(lp) => lp,
            Err(err) => {
                let r = match ContainError::from(err) {
                    ContainError::Escalated { diag, .. } => CrossRefusal::Escalated(vec![diag]),
                    other => CrossRefusal::Unsupported(other),
                };
                refused(r, errors);
                return None;
            }
        };
        let n = lp.verts.len();
        for (i, edge) in lp.edges.iter().enumerate() {
            let side = BoundarySide {
                r#loop: lk,
                key: lp.keys[i],
                ends: (lp.verts[i], lp.verts[(i + 1) % n]),
            };
            let points = edge_crossings(body, e, f.key, edge, side, band, errors)?;
            cuts.extend(points.into_iter().map(|(s, at)| Cut { s, at }));
        }
    }
    Some(cuts)
}

/// `r`, a refusal reading `face`'s boundary for crossings, pushed as
/// the census reports it.
fn refuse_crossing(face: FaceKey, r: CrossRefusal, errors: &mut Vec<ValidationError>) {
    match r {
        CrossRefusal::Escalated(causes) => errors.extend(
            causes
                .into_iter()
                .map(|cause| ValidationError::CensusEscalated { cause }),
        ),
        CrossRefusal::Unsupported(cause) => errors.push(ValidationError::CensusUnsupported {
            subject: CensusSubject::Entity(EntityId::Face(face)),
            cause: CensusUnsupportedCause::Containment(cause),
        }),
        CrossRefusal::Unread(what) => errors.push(ValidationError::CensusUnsupported {
            subject: CensusSubject::Entity(EntityId::Face(face)),
            cause: CensusUnsupportedCause::ContactLane(
                crate::contact::ContactRefusal::NotCertifiable { what },
            ),
        }),
    }
}

/// What reading one boundary edge for crossings refused with.
enum CrossRefusal {
    /// A crossing too close to call: the rows' own escalations.
    Escalated(Vec<geom_core::Indeterminate>),
    /// The boundary could not be read, typed.
    Unsupported(ContainError),
    /// The meeting reading refused past the band's own escalation.
    Unread(&'static str),
}

/// One boundary edge of a face: its loop, its key, and its two ends in
/// the loop's walk order.
#[derive(Clone, Copy)]
struct BoundarySide<T: Real> {
    r#loop: LoopKey,
    key: EdgeKey,
    ends: (Point3<T>, Point3<T>),
}

/// Where one boundary edge of `f` crosses `e`'s span strictly inside
/// both: [`geom_brep::curve_pair::meet`] of `e`'s segment and the
/// boundary edge's span — a straight edge's segment between its two ends,
/// any other its certified carrier's — each meeting on both interiors a
/// cut there. A meeting at a boundary edge's end is a boundary vertex,
/// which the vertex cuts take; one at an end of `e` is that end's own
/// cut; a shared run is a collinear boundary rest, the edge-edge overlap
/// pass's finding. `None` where the reading refused (pushed, about
/// `face`).
fn edge_crossings<T: Decide>(
    body: &Body<T>,
    e: &EdgeGeo<T>,
    face: FaceKey,
    edge: &crate::splitting::containment::LoopEdge<T>,
    side: BoundarySide<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Vec<(T, CutAt)>> {
    use crate::splitting::containment::LoopEdge;
    let stored = body
        .get_edge(side.key)
        .and_then(|edge| body.get_curve_geom(edge.curve))
        .and_then(CurveGeom::certified);
    // Two straight segments from one point meet nowhere else unless they
    // run together, and a shared run is no cut either.
    let straight = stored.is_none_or(|c| matches!(c.carrier(), geom::Curve3::Line { .. }));
    if matches!(edge, LoopEdge::Chord) && straight && shares_an_end(body, e, side.key) {
        return Some(Vec::new());
    }
    let meets = |errors: &mut Vec<ValidationError>| {
        meetings(e, edge, side, stored, band)
            .map_err(|r| refuse_crossing(face, r, errors))
            .ok()
    };
    if !matches!(edge, LoopEdge::Chord) {
        return meets(errors);
    }
    // A chord of no length is null scaffolding, a coincident copy of its
    // vertex: no crossing of its own, and the vertex cuts read its point.
    // One whose length is undecided is read as a chord by
    // [`read_undecided`].
    let (a, b) = side.ends;
    match decide("pm_census_ef_cross_chord", Margin::norm3(b - a), band) {
        Ok(Sign::Zero) => Some(Vec::new()),
        Ok(Sign::Positive | Sign::Negative) => meets(errors),
        Err(cause) => {
            let as_chord = |()| {
                let mut found = Vec::new();
                (meets(&mut found), found)
            };
            let carried = read_undecided([()], as_chord);
            let Some(carried) = carried else {
                return Some(Vec::new());
            };
            errors.push(ValidationError::CensusEscalated { cause });
            errors.extend(carried);
            None
        }
    }
}

/// [`edge_crossings`]' meeting reading of `e`'s segment and the boundary
/// edge's span, past its gates.
fn meetings<T: Decide>(
    e: &EdgeGeo<T>,
    edge: &crate::splitting::containment::LoopEdge<T>,
    side: BoundarySide<T>,
    stored: Option<&geom_brep::EdgeCurve<T>>,
    band: Band,
) -> Result<Vec<(T, CutAt)>, CrossRefusal> {
    use crate::splitting::containment::LoopEdge;
    let along = geom::Curve3::Line {
        origin: e.p0,
        dir: e.dir,
    };
    let (a, b) = side.ends;
    // A straight edge with no certified curve of its own is the segment
    // between its loop ends.
    let chord = geom::Curve3::Line {
        origin: a,
        dir: (b - a).normalize(),
    };
    let (boundary, at) = match (edge, stored) {
        (LoopEdge::Chord, Some(curve)) => (
            CarrierSpan::new(curve.carrier(), curve.params()),
            CutAt::Crossing(side.key),
        ),
        (LoopEdge::Chord, None) => (
            CarrierSpan::new(&chord, (T::zero(), (b - a).norm())),
            CutAt::Crossing(side.key),
        ),
        (LoopEdge::Conic(_) | LoopEdge::Spiric(_) | LoopEdge::Unrowed { .. }, Some(curve)) => (
            CarrierSpan::new(curve.carrier(), curve.params()),
            CutAt::CurvedCrossing(side.key),
        ),
        (LoopEdge::Conic(_) | LoopEdge::Spiric(_) | LoopEdge::Unrowed { .. }, None) => {
            return Err(CrossRefusal::Unsupported(ContainError::LoopUnreadable(
                side.r#loop,
            )));
        }
    };
    let met = meet(&CarrierSpan::new(&along, (T::zero(), e.len)), &boundary, band).map_err(|r| {
        match r.fault {
            MeetFault::Escalated(cause) => CrossRefusal::Escalated(vec![cause]),
            MeetFault::Unsettled(_) => {
                CrossRefusal::Unread("the meeting reading of a census edge and a face boundary edge could not settle a count")
            }
            MeetFault::Unsupported { pose } => CrossRefusal::Unread(pose),
            MeetFault::Desync => CrossRefusal::Unread(
                "the meeting reading of a census edge and a face boundary edge misread a carrier's kind",
            ),
            MeetFault::Contradiction { readings } => CrossRefusal::Unread(readings),
        }
    })?;
    if met.shared.is_some() {
        return Ok(Vec::new());
    }
    Ok(met
        .points
        .iter()
        .filter_map(|m| match (m.on_a, m.on_b) {
            (Hit::On { t, .. }, Hit::On { .. }) => Some((t, at)),
            (Hit::On { .. } | Hit::End(_), Hit::End(_)) | (Hit::End(_), Hit::On { .. }) => None,
        })
        .collect())
}

/// Whether edge `g` has an end on one of `e`'s ends' points.
fn shares_an_end<T: Real>(body: &Body<T>, e: &EdgeGeo<T>, g: EdgeKey) -> bool {
    let point = |v: VertexKey| body.vertices.get(v).map(|d| d.point);
    let ends = body.get_edge(g).and_then(|g| {
        Some([
            body.half_edges.get(g.he_plus)?.start,
            body.half_edge_end(g.he_plus)?,
        ])
    });
    ends.is_some_and(|ends| {
        ends.iter().any(|&w| {
            [e.v0, e.v1]
                .iter()
                .any(|&v| v == w || point(v).is_some_and(|p| point(w) == Some(p)))
        })
    })
}

/// Census pass 5: edge × edge — proper interior crossings (backable
/// at the unified strength through [`ee_cross_backed`]) and collinear
/// positive-length overlaps ([`ee_collinear_lane`]).
#[allow(clippy::too_many_arguments)] // the census's fixed sweep signature plus the region door the rung consults
fn sweep_edge_edge<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    region: Option<RegionLane<T>>,
    cands: &Candidates,
    mut trace: Option<&mut CensusTrace>,
    errors: &mut Vec<ValidationError>,
) {
    for (i, ea) in geo.edges.iter().enumerate() {
        for j in cands.trees.edges.later(i) {
            let eb = candidate(&geo.edges, j);
            let before = errors.len();
            pair_edge_edge(body, geo, declared, band, region, ea, eb, errors);
            if let Some(t) = trace.as_deref_mut() {
                t.ee.note(
                    (EntityId::Edge(ea.key), EntityId::Edge(eb.key)),
                    errors.len() > before,
                );
            }
        }
    }
}

/// One edge pair of pass 5.
#[allow(clippy::too_many_arguments)] // one pair of pass 5, the region door riding to the rung
fn pair_edge_edge<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    region: Option<RegionLane<T>>,
    ea: &EdgeGeo<T>,
    eb: &EdgeGeo<T>,
    errors: &mut Vec<ValidationError>,
) {
    match ee_meeting(ea, eb, band, errors) {
        Some(Some(Meeting::Cross(margin))) => {
            ee_crossing_lane(body, geo, declared, ea, eb, margin, band, region, errors);
        }
        Some(Some(Meeting::Overlap(span, margin))) => {
            ee_collinear_lane(ea, eb, span, margin, geo, declared, band, errors);
        }
        Some(None) | None => {}
    }
}

/// Where two line edges' interiors meet.
enum Meeting<T> {
    /// At one point strictly inside both spans, with the reading of
    /// the Zero that made the lines meet.
    Cross(MarginDiag),
    /// Along `[lo, hi]` of the first edge's span, with the reading of
    /// the Zero that put the two on one line.
    Overlap((T, T), MarginDiag),
}

/// How two line edges' interiors meet: through the parallel screen
/// ([`parallel_reading`]) to the crossing ([`crossing_reading`]) or the
/// collinear overlap ([`collinear_overlap`]). `Some(Some(_))` where
/// they meet, `Some(None)` where they do not, `None` where a decision
/// escalated (pushed). An in-band screen is [`settle_in_band`]'s.
fn ee_meeting<T: Decide>(
    ea: &EdgeGeo<T>,
    eb: &EdgeGeo<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<Meeting<T>>> {
    let mut screen = Vec::new();
    match parallel_reading(ea, eb, band, &mut screen) {
        Some(false) => Some(crossing_reading(ea, eb, band, errors)?.map(Meeting::Cross)),
        Some(true) => Some(
            collinear_overlap(ea, eb, band, errors)?
                .map(|(span, margin)| Meeting::Overlap(span, margin)),
        ),
        None => settle_in_band(ea, eb, band, screen, errors).map(|()| None),
    }
}

/// Census pass 5′: a **curved** edge's interior meeting another edge's,
/// the question pass 5 asks of two lines, read through the curved lane
/// ([`curved::interiors_meeting`], the one reading of two carrier spans'
/// meetings): each curved edge against the line edges and the later
/// curved edges whose padded boxes meet its own. A meeting on both
/// interiors is owed the pair's edge-edge record (D1), or a declared face
/// pair holding the two edges ([`Declared::sides_face_backed`], the
/// grandfathered rung of pass 5's overlap lane: the crossing rung,
/// [`ee_cross_backed`], is planar-first and answers no curved pair, so
/// a crossing takes the rung pass 2′ takes for its vertex). Unbacked,
/// a crossing or touch at a point is an `EdgeEdgeCross` and an overlap
/// along one curve an `EdgeEdgeOverlap`, each an `UnrecordedContact`. An overlap's bounds are
/// vertex events, ends of one edge resting on the other or on each
/// other, which passes 1, 2 and 2′ answer at their own points. A meeting
/// at an end of either edge is theirs too, and it takes its whole
/// stretch. Edges sharing a vertex are read like any pair: their meeting
/// there is at an end of both.
fn sweep_curved_edge_edge<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    cands: &Candidates,
    mut trace: Option<&mut CensusTrace>,
    errors: &mut Vec<ValidationError>,
) {
    // Each edge's sides. `edge_sides` refuses only a dangling key, and
    // the curved candidates are the body's own live keys.
    let sides = |ek: EdgeKey| {
        crate::readback::edge_sides(body, ek)
            .unwrap_or_else(|_| unreachable!("a curved candidate is a live edge"))
            .faces()
    };
    let curved: Vec<_> = cands.curved.iter().map(|&ek| (ek, sides(ek))).collect();
    let lines: Vec<_> = geo
        .edges
        .iter()
        .map(|e| (e.key, (e.f_plus, e.f_minus)))
        .collect();
    for (i, &(a, sides_a)) in curved.iter().enumerate() {
        let query = cands.trees.curved.query(i);
        let others = cands
            .trees
            .edges
            .overlapping(query)
            .into_iter()
            .map(|j| *candidate(&lines, j))
            .chain(
                cands
                    .trees
                    .curved
                    .later(i)
                    .into_iter()
                    .map(|j| *candidate(&curved, j)),
            );
        for (b, sides_b) in others {
            let before = errors.len();
            pair_curved_edge_edge(body, declared, band, (a, sides_a), (b, sides_b), errors);
            if let Some(t) = trace.as_deref_mut() {
                t.cee.note(
                    (EntityId::Edge(a), EntityId::Edge(b)),
                    errors.len() > before,
                );
            }
        }
    }
}

/// One curved edge–edge pair of pass 5′.
fn pair_curved_edge_edge<T: Decide>(
    body: &Body<T>,
    declared: &Declared,
    band: Band,
    (a, sides_a): (EdgeKey, (FaceKey, FaceKey)),
    (b, sides_b): (EdgeKey, (FaceKey, FaceKey)),
    errors: &mut Vec<ValidationError>,
) {
    let Some(Some(met)) = curved::interiors_meeting(body, a, b, band, errors) else {
        return;
    };
    if declared.ee_recorded(a, b) || declared.sides_face_backed(sides_a, sides_b) {
        return;
    }
    let contact = if met.overlap {
        CensusContact::EdgeEdgeOverlap { a, b }
    } else {
        CensusContact::EdgeEdgeCross { a, b, side: None }
    };
    errors.push(ValidationError::UnrecordedContact {
        contact,
        witness: witness(met.point),
        margin: Some(met.margin),
    });
}

/// Two straight spans' collinear overlap behind the parallel screen:
/// [`collinear_overlap`] where [`parallel_reading`] reads them
/// parallel; `Some(None)` where it reads them not (a crossing's
/// question, never an overlap); `None` where a decision escalated
/// (pushed). An in-band screen is [`settle_in_band`]'s.
fn parallel_overlap<T: Decide>(
    a: &impl Spans<T>,
    b: &impl Spans<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<((T, T), MarginDiag)>> {
    let mut screen = Vec::new();
    match parallel_reading(a, b, band, &mut screen) {
        Some(true) => collinear_overlap(a, b, band, errors),
        Some(false) => Some(None),
        None => settle_in_band(a, b, band, screen, errors).map(|()| None),
    }
}

/// An in-band parallel screen (`screen`, its escalation):
/// `Some(())`, settled with nothing pushed, where the overlap reading
/// definitely refuses an overlap; `None`, the screen's escalation
/// pushed, otherwise.
///
/// Neither of the screen's verdicts then finds a meeting. Its parallel
/// verdict reads this overlap. Its other verdict asks for a crossing
/// strictly inside both spans, each span margin at least `Kε`: then
/// the shorter span exceeds `2Kε`, so an in-band screen
/// (`sin φ · len < Kε`) has `sin φ < 1/2`, `cos φ > √3/2`, and the
/// `2Kε` of `b` about the crossing projects perpendicularly onto `a`'s
/// axis as more than `√3·Kε` of `a`'s span: a positive overlap,
/// whose run holds the crossing within `ε` of `b`'s line, so a Zero
/// gap. The overlap reading would not have refused.
fn settle_in_band<T: Decide>(
    a: &impl Spans<T>,
    b: &impl Spans<T>,
    band: Band,
    screen: Vec<ValidationError>,
    errors: &mut Vec<ValidationError>,
) -> Option<()> {
    if let Some(None) = collinear_overlap(a, b, band, &mut Vec::new()) {
        return Some(());
    }
    errors.extend(screen);
    None
}

/// Whether two straight spans are parallel: `Some(true)` parallel,
/// `Some(false)` not, `None` escalated (pushed). The margin is
/// sin(angle of the unit dirs) × the shorter span's own length: the
/// displacement the angular deviation induces over it.
fn parallel_reading<T: Decide>(
    a: &impl Spans<T>,
    b: &impl Spans<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    let (a, b) = (a.span(), b.span());
    gap_is_zero(
        "pm_census_ee_parallel",
        Margin::levered(a.dir.cross(b.dir).norm(), a.len.min(b.len)),
        band,
        errors,
    )
}

/// What the crossing rung answered for one `EdgeEdgeCross`.
enum CrossingBacking {
    /// A declared pair answers for the crossing: point in its
    /// verified overlap region, side verdict
    /// [`CrossingSideVerdict::OppositeSides`].
    Backed,
    /// A declared, region-holding pair answered — and its verdict
    /// refuses. The finding must NAME it.
    Refused(CrossingSideVerdict),
    /// No declared pair answers for the crossing point at all: the
    /// plain hard finding, exactly as an unrecorded seat has always
    /// read.
    Unanswered,
}

/// **The `EdgeEdgeCross` backing rung** — the unified strength's
/// first instance (module docs; `crates/topo/README.md`,
/// option A, planar-first): a declared face pair backs a crossing of
/// two coplanar boundary edges iff
///
/// 1. the crossing is an event OF the pair's carrier — the crossing
///    point on both carriers and within both CLOSED trim regions
///    ([`pair_holds_point`]), BOTH crossing edges lying in both
///    carriers ([`pair_holds_edges`], the spec's "two coplanar
///    boundary edges" read as a decided screen: an edge DIVING
///    through the carrier crosses in-plane edges at plane level, and
///    such a transverse crossing is interpenetration however the seat
///    is declared), and the carriers not definitely transverse at the
///    crossing ([`geom_brep::classify_dihedral`], the same all-smooth
///    gate the tier-3 wedge pass and the rim-wedge screen run);
/// 2. the side question is validly posed and answers
///    [`CrossingSideVerdict::OppositeSides`] — the Smooth-site
///    precondition [`geom_brep::classify_material_pairing`]'s
///    contract states is ESTABLISHED first (the dihedral of 1
///    decided Smooth), then the material pairing decided by that one sense
///    algebra (which mints both outward normals itself from the
///    faces' `Face::sense` bits, levered by
///    [`geom_brep::folded_lever_arm`] over the shorter edge — the
///    same arm the parallel gate meters). No new numerics; and
/// 3. the pair itself is VERIFIED — [`pair_region_verified`], the
///    confirm pass's two doors, tried in both frame orders (the
///    certified answers are frame-invariant, `world_carrier`'s lemma;
///    trying both keeps the rung total over the record's spelling).
///
/// The side test is read BEFORE the verified-overlap doors, and the
/// order is load-bearing rather than convenient: Door 1 CONTRADICTS a
/// declared `Rest` pair whose senses are aligned, so a transverse
/// (same-side) crossing can never emerge from behind those doors with
/// its verdict named — it would drown as one more unverified pair.
/// The carrier screens run first either way, so only a pair whose
/// carrier holds the whole crossing ever speaks; a remote, diving, or
/// skew pair's senses name nothing.
///
/// **Two-pass shape, so the posture is a total function of the
/// geometry and never of `FaceKey` order**: pass 1 walks each
/// UNORDERED declared pair once (`Declared::index` stores both
/// orientations; every screen and the side algebra are symmetric in
/// the pair) and collects its outcome; pass 2 emits — every
/// undecided candidate's escalation exactly once, whether or not some
/// other pair backs (an undecidable question that was validly
/// reached stays loud), then the verdict: backed if any pair backs,
/// else same-side over undecided (definite evidence over an
/// escalation), else unanswered.
///
/// **What region containment replaced, named**: the grandfathered
/// rungs confine by STRUCTURAL INCIDENCE (the pair must hold the
/// event's entities); this rung confines by the carrier instead —
/// nothing checks that `ea`/`eb` are boundary edges of `fa`/`fb`, and
/// the CLOSED containment admits a pair whose region only
/// corner-touches the crossing point. That is the unified sentence's
/// licence read literally — the pair answers for every event at its
/// verified interface, whoever's entities carry it — and after the
/// edge screen the interface really does carry the whole crossing;
/// the trade (incidence out, carrier-confinement in) is this
/// paragraph, not an accident.
///
/// Curved pairs are outside the planar-first rung and never answer;
/// `EdgeFacePierce` takes NO rung at all: a transverse dive is
/// interpenetration.
#[allow(clippy::too_many_arguments)] // the rung's whole state, no less
fn ee_cross_backed<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    ea: &EdgeGeo<T>,
    eb: &EdgeGeo<T>,
    q: Point3<T>,
    band: Band,
    region: Option<RegionLane<T>>,
    errors: &mut Vec<ValidationError>,
) -> CrossingBacking {
    // Pass 1: one collected outcome per unordered candidate pair.
    let mut backed = false;
    let mut same_side = false;
    let mut undecided: Vec<Indeterminate> = Vec::new();
    for &(fa, fb) in &declared.faces {
        if fa >= fb {
            continue; // unordered walk (both orientations are stored)
        }
        let (Some(ga), Some(gb)) = (planar_face(geo, fa), planar_face(geo, fb)) else {
            continue; // planar-first: a curved pair never answers here
        };
        if !pair_holds_point(body, ga, gb, q, band, errors)
            || !pair_holds_edges(ga, gb, ea, eb, band, errors)
        {
            continue; // the pair's carrier does not hold the crossing
        }
        // Both faces are in the snapshot, read off the live arena.
        let (da, db) = (
            proven(&body.faces, fa, EntityId::Face),
            proven(&body.faces, fb, EntityId::Face),
        );
        let (sa, sb) = (
            body.face_surface_linked(fa, da),
            body.face_surface_linked(fb, db),
        );
        let arm_extent = ea.len.min(eb.len);
        // The last carrier screen, and the Smooth-site precondition the
        // pairing below states. The edge screen reads points against
        // each plane; this reads the planes against each other, at the
        // crossing. Two edges can run within the zero band of two
        // transverse planes where both hug the planes' common line, so a
        // definitely transverse pair passes the edge screen and still
        // does not hold the crossing: it answers for nothing.
        match geom_brep::classify_dihedral(sa, sb, q, arm_extent, band) {
            Ok(geom_brep::DihedralClass::Smooth) => {}
            Ok(geom_brep::DihedralClass::Transverse) => continue,
            Err(escalation) => {
                undecided.push(escalation.diag());
                continue;
            }
        }
        let arm = geom_brep::folded_lever_arm(sa, sb, q, arm_extent);
        let side =
            match geom_brep::classify_material_pairing(sa, da.sense, sb, db.sense, q, arm, band) {
                Ok(geom_brep::MaterialPairing::Opposed) => CrossingSideVerdict::OppositeSides,
                Ok(geom_brep::MaterialPairing::Aligned) => CrossingSideVerdict::SameSide,
                Err(cause) => {
                    undecided.push(cause);
                    continue;
                }
            };
        if side == CrossingSideVerdict::OppositeSides {
            if pair_verified(body, fa, fb, band, region) {
                backed = true;
            }
            // An opposite-sides pair that does not verify answers for
            // nothing; its own state is the confirm pass's report.
        } else {
            same_side = true;
        }
    }
    // Pass 2: emit once, as a function of the collected outcomes —
    // every undecided candidate's escalation stays loud whether or
    // not another pair backs.
    let any_undecided = !undecided.is_empty();
    for cause in undecided {
        errors.push(ValidationError::CensusEscalated { cause });
    }
    if backed {
        CrossingBacking::Backed
    } else if same_side {
        CrossingBacking::Refused(CrossingSideVerdict::SameSide)
    } else if any_undecided {
        CrossingBacking::Refused(CrossingSideVerdict::Undecided)
    } else {
        CrossingBacking::Unanswered
    }
}

/// Where two non-parallel edge lines come closest, each quantity as
/// `[lower, upper]`: an enclosure of the real value under the closed
/// form's rounding.
struct LinesClosest<T: Real> {
    /// The lines' distance there.
    gap: [T; 2],
    /// Per edge, `[from its start, to its end]` along it.
    reach: [[[T; 2]; 2]; 2],
    /// The closest point on `ea`'s line.
    point: Point3<T>,
}

/// Where two non-parallel edge lines come closest.
///
/// Two edges ending on one point key meet there: each reach is exactly
/// `0` or its edge's length, the gap exactly `0`.
///
/// Otherwise every quantity is read from the nearest ends the closed
/// form allows, and charged its rounding. With `n = da × db` and `s =
/// |n| = sin φ`, an end `p` of one edge stands `(q − p)·(d' × n)/s²`
/// from the closest point, for any point `q` of the other line (`d'`
/// its direction); the lines' distance is `|(q − p)·n|/s` for any `p`,
/// `q` on them. Read from ends `e = q − p` apart, the rounding of `e`,
/// of the unit directions and of `n` (whose cancellation leaves it
/// `u` off in any direction, `u/s` relative) is about `|e|·u/s` in the
/// gap, and `(|e| + |reach| + gap/s)·u/s` in a reach: `n`'s tilt leaks
/// the off-plane part of every `e`, the lines' real distance, into it
/// over `s` once more, so a reach is charged the gap's upper bound.
/// Counted to first order, the error is about
/// `8.5u·|e|/s + 5.5u·|reach| + 5.5u·gap/s` in a reach and
/// `(5.5/s + 4)u·|e|` in the gap, under the
/// `16u` of [`geom_brep::rounding_charge`] by about 2×, and every single
/// reading's error is held under half its charge at coordinates to
/// `1e4` and ends to a thousand lengths off the crossing
/// (`segment_fuzz::each_readings_error_stays_inside_its_charge_far_from_the_unit_box`).
/// Each reading is taken from every pair of ends it allows and the
/// enclosures intersected, so the nearest ends bound it.
fn ee_lines_closest<T: Real>(ea: &EdgeGeo<T>, eb: &EdgeGeo<T>) -> LinesClosest<T> {
    let end = |e: &EdgeGeo<T>, k: usize| if k == 0 { e.p0 } else { e.p1 };
    let shared = [(0, 0), (0, 1), (1, 0), (1, 1)]
        .into_iter()
        .find(|&(i, j)| ea.points[i].is_some() && ea.points[i] == eb.points[j]);
    if let Some((i, j)) = shared {
        let exact = |x: T| [x, x];
        let reach = |k: usize, len: T| {
            if k == 0 {
                [exact(T::zero()), exact(len)]
            } else {
                [exact(len), exact(T::zero())]
            }
        };
        return LinesClosest {
            gap: exact(T::zero()),
            reach: [reach(i, ea.len), reach(j, eb.len)],
            point: end(ea, i),
        };
    }
    let (n, nn, s) = lines_frame(ea, eb);
    let ends = [
        (ea.p0, eb.p0),
        (ea.p0, eb.p1),
        (ea.p1, eb.p0),
        (ea.p1, eb.p1),
    ];
    let [lo, hi] = ends
        .into_iter()
        .map(|(p, q)| gap_from_ends(q - p, n, s))
        .reduce(both_enclose)
        .unwrap_or_else(|| unreachable!("four ends"));
    // `x`'s ends against the line of `y`: from its start, to its end.
    let reaches = |x: &EdgeGeo<T>, y: &EdgeGeo<T>| {
        let w = reach_axis(x, y, nn);
        let from = |p: Point3<T>, flip: bool| {
            let read = |q: Point3<T>| reach_from_end(q - p, w, flip, hi, s);
            both_enclose(read(y.p0), read(y.p1))
        };
        [from(x.p0, false), from(x.p1, true)]
    };
    let reach = [reaches(ea, eb), reaches(eb, ea)];
    let [from_lo, from_hi] = reach[0][0];
    LinesClosest {
        gap: [lo.max(T::zero()), hi],
        point: ea.p0 + ea.dir * geom::mid_param(from_lo, from_hi),
        reach,
    }
}

/// Two edge lines' frame: `n = da × db`, `|n|²` and `s = |n|`.
fn lines_frame<T: Real>(ea: &EdgeGeo<T>, eb: &EdgeGeo<T>) -> (Vec3<T>, T, T) {
    let n = ea.dir.cross(eb.dir);
    let nn = n.norm_squared();
    (n, nn, nn.sqrt())
}

/// The lines' gap read from ends `e` apart, `|e·n|/s`, enclosed by its
/// rounding charge ([`ee_lines_closest`]).
fn gap_from_ends<T: Real>(e: Vec3<T>, n: Vec3<T>, s: T) -> [T; 2] {
    let g = e.dot(n).abs() / s;
    let charge = geom_brep::rounding_charge(e.norm() * (T::one() + s)) / s;
    [g - charge, g + charge]
}

/// The axis `w = d_y × (d_x × d_y)/|n|²` whose dot with `q − p` is how
/// far `x`'s end `p` stands from the closest point, `q` on `y`'s line.
fn reach_axis<T: Real>(x: &EdgeGeo<T>, y: &EdgeGeo<T>, nn: T) -> Vec3<T> {
    y.dir.cross(x.dir.cross(y.dir)) / nn
}

/// A reach read from an end `e` short of a point of the other line,
/// `e·w` (negated from an edge's far end, `flip`), enclosed by its
/// rounding charge, `gap_hi` the gap's upper bound ([`ee_lines_closest`]).
fn reach_from_end<T: Real>(e: Vec3<T>, w: Vec3<T>, flip: bool, gap_hi: T, s: T) -> [T; 2] {
    let r = if flip { -e.dot(w) } else { e.dot(w) };
    let charge = geom_brep::rounding_charge(e.norm() + r.abs() + gap_hi / s) / s;
    [r - charge, r + charge]
}

/// The intersection of two enclosures of one value.
fn both_enclose<T: Real>([lo_a, hi_a]: [T; 2], [lo_b, hi_b]: [T; 2]) -> [T; 2] {
    [lo_a.max(lo_b), hi_a.min(hi_b)]
}

/// The crossing point of two edges whose lines meet: the one point the
/// crossing lane reports and the touch analysis reads.
fn ee_cross_point<T: Real>(ea: &EdgeGeo<T>, eb: &EdgeGeo<T>) -> Point3<T> {
    ee_lines_closest(ea, eb).point
}

/// The midpoint of two collinear edges' overlap: a point on both
/// edges, where a collinear overlap's touch is read.
fn ee_overlap_midpoint<T: Real>(ea: &EdgeGeo<T>, eb: &EdgeGeo<T>) -> Point3<T> {
    let s0 = (eb.p0 - ea.p0).dot(ea.dir);
    let s1 = s0 + eb.len * eb.dir.dot(ea.dir);
    let lo = s0.min(s1).max(T::zero());
    let hi = s0.max(s1).min(ea.len);
    ea.p0 + ea.dir * geom::mid_param(lo, hi)
}

/// Whether two non-parallel edges cross at a point inside both their
/// spans: the lines' closest point is strictly interior to each
/// (`pm_census_ee_span`) and the lines meet there (`pm_census_ee_gap`),
/// each read off its enclosure ([`ee_lines_closest`]).
/// `Some(Some(margin))` a crossing, with the reading of the Zero that
/// made the lines meet; `Some(None)` none; `None` where a decision
/// escalated (pushed). Either reading's definite no settles the pair
/// whatever the other reads, and an escalation is pushed only where
/// none did. The crossing lane and the confirm pass of an edge-edge
/// record ask this one question.
fn crossing_reading<T: Decide>(
    ea: &EdgeGeo<T>,
    eb: &EdgeGeo<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<MarginDiag>> {
    let closest = ee_lines_closest(ea, eb);
    let mut held = Vec::new();
    for reach in closest.reach.into_iter().flatten() {
        match span_reading(reach, band) {
            Ok(true) => {}
            Ok(false) => return Some(None),
            Err(cause) => held.push(ValidationError::CensusEscalated { cause }),
        }
    }
    match gap_reading(closest.gap, band) {
        Ok(None) => Some(None),
        Ok(Some(margin)) if held.is_empty() => Some(Some(margin)),
        Ok(Some(_)) => {
            errors.extend(held);
            None
        }
        Err(cause) => {
            errors.extend(held);
            errors.push(ValidationError::CensusEscalated { cause });
            None
        }
    }
}

/// What an enclosure `[lo, hi]` of a quantity decides.
enum Enclosed {
    /// `lo` reads Positive: the quantity exceeds `Kε`.
    Above,
    /// `hi` reads Zero or Negative: the quantity is at most `ε`, with
    /// `hi`'s reading.
    AtMost(MarginDiag),
}

/// The one reading of a quantity known only to lie in `[lo, hi]`, each
/// bound decided by `decide`: [`Enclosed`], else the escalation of a
/// bound that did not decide, or the bounds' straddle of the band under
/// `straddle`, the site's own row.
fn enclosure_reading<T: Decide>(
    [lo, hi]: [T; 2],
    straddle: &'static str,
    band: Band,
    decide: impl Fn(T) -> Result<geom_core::Decided, Indeterminate>,
) -> Result<Enclosed, Indeterminate> {
    match (decide(lo), decide(hi)) {
        (Ok(b), _) if b.sign == Sign::Positive => Ok(Enclosed::Above),
        (_, Ok(a)) if a.sign != Sign::Positive => Ok(Enclosed::AtMost(a.margin)),
        (Ok(b), Ok(a)) => Err(geom_core::k_stats::straddle(straddle, b, a, band)),
        (Err(cause), _) | (_, Err(cause)) => Err(cause),
    }
}

/// Whether a reach enclosed by `[lo, hi]` stands strictly inside its
/// span: `Ok(true)` above `Kε`; `Ok(false)` at most `ε`, so the closest
/// point lies at an end or past it ([`enclosure_reading`]).
fn span_reading<T: Decide>(reach: [T; 2], band: Band) -> Result<bool, Indeterminate> {
    let decide = |m| geom_core::k_stats::decide_reported("pm_census_ee_span", Margin::of(m), band);
    enclosure_reading(reach, "pm_census_ee_span_straddle", band, decide)
        .map(|read| matches!(read, Enclosed::Above))
}

/// Whether a gap enclosed by `[lo, hi]` is zero: `Ok(Some(margin))` at
/// most `ε`, with `hi`'s reading; `Ok(None)` above `Kε`
/// ([`enclosure_reading`]). Both bounds are nonnegative by construction
/// (`lo` a `max` from zero, `hi` the least `|e·n|/s` plus its charge), the magnitude
/// door's precondition.
fn gap_reading<T: Decide>(gap: [T; 2], band: Band) -> Result<Option<MarginDiag>, Indeterminate> {
    let decide = |m| {
        geom_core::k_stats::decide_magnitude_reported("pm_census_ee_gap", Margin::of(m), band).map(
            |(magnitude, margin)| geom_core::Decided {
                sign: match magnitude {
                    Magnitude::Zero => Sign::Zero,
                    Magnitude::Positive => Sign::Positive,
                },
                margin,
            },
        )
    };
    enclosure_reading(gap, "pm_census_ee_gap_straddle", band, decide).map(|read| match read {
        Enclosed::Above => None,
        Enclosed::AtMost(margin) => Some(margin),
    })
}

/// A crossing strictly inside both spans ([`crossing_reading`];
/// endpoint events are pass-1/2 findings), with the margin that made
/// the lines meet: backed by its edge-edge record, or by a declared
/// pair at the unified strength ([`ee_cross_backed`]), or a hard
/// finding, with the side verdict NAMED in the witness when a
/// region-holding pair refused it.
#[allow(clippy::too_many_arguments)] // the lane's whole state, no less
fn ee_crossing_lane<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    ea: &EdgeGeo<T>,
    eb: &EdgeGeo<T>,
    margin: MarginDiag,
    band: Band,
    region: Option<RegionLane<T>>,
    errors: &mut Vec<ValidationError>,
) {
    if declared.ee_recorded(ea.key, eb.key) {
        return;
    }
    let q = ee_cross_point(ea, eb);
    match ee_cross_backed(body, geo, declared, ea, eb, q, band, region, errors) {
        CrossingBacking::Backed => {}
        CrossingBacking::Unanswered => {
            errors.push(ValidationError::UnrecordedContact {
                contact: CensusContact::EdgeEdgeCross {
                    a: ea.key,
                    b: eb.key,
                    side: None,
                },
                witness: witness(q),
                margin: Some(margin),
            });
        }
        CrossingBacking::Refused(verdict) => {
            // Same-side is interpenetration evidence, which the at-rest
            // door between two copies reads off this field; undecided
            // is already escalated typed alongside this finding.
            if verdict == CrossingSideVerdict::OppositeSides {
                unreachable!("Backed above");
            }
            errors.push(ValidationError::UnrecordedContact {
                contact: CensusContact::EdgeEdgeCross {
                    a: ea.key,
                    b: eb.key,
                    side: Some(verdict),
                },
                witness: witness(q),
                margin: Some(margin),
            });
        }
    }
}

/// The positive-length overlap `[lo, hi]` of two parallel edges on
/// `ea`'s axis, where they are collinear: `Some(Some(_))` an overlap;
/// `Some(None)` where they are not collinear, or the overlap is a point
/// or empty (pass 1's territory); `None` where a decision escalated
/// (pushed). The span is read before the gap, and the gap
/// where the two run side by side ([`overlap_gap`]); either one's
/// definite no settles the pair whatever the other reads, and an
/// escalation is pushed only where neither did. The collinear lane and
/// the confirm pass of an edge-edge record ask this one question.
fn collinear_overlap<T: Decide>(
    ea: &impl Spans<T>,
    eb: &impl Spans<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<((T, T), MarginDiag)>> {
    let (ea, eb) = (ea.span(), eb.span());
    let t0 = (eb.p0 - ea.p0).dot(ea.dir);
    let t1 = t0 + eb.len * eb.dir.dot(ea.dir);
    let mut held = Vec::new();
    let span = projected_overlap(&ea, (t0, t1), band, &mut held);
    if matches!(span, Some(None)) {
        return Some(None);
    }
    let gap = overlap_gap(&ea, &eb, (t0, t1));
    match (
        span,
        gap_zero_reading("pm_census_ee_line_gap", gap, band, &mut held),
    ) {
        (Some(Some(span)), Some(Some(margin))) => Some(Some((span, margin))),
        (_, Some(None)) => Some(None),
        _ => {
            errors.extend(held);
            None
        }
    }
}

/// The least distance from `ea`'s points over `eb`'s projection on its
/// axis `[t0, t1]`, clipped to its span by value, to `eb`'s line: where
/// the two spans run side by side, so neither one's tilt is levered
/// along the other's extended line. Zero puts a point of `ea` on the
/// run within `ε` of `eb`'s line, its foot inside `eb`'s span to about
/// `ε·tan φ` at the run's ends; a definite distance holds every point
/// of the run clear of `eb`'s line, so of `eb`, and off the run `ea`
/// stands past `eb`'s ends.
fn overlap_gap<T: Real>(ea: &Span<T>, eb: &Span<T>, (t0, t1): (T, T)) -> Margin<T> {
    let lo = t0.min(t1).max(T::zero());
    let run = (t0.max(t1).min(ea.len) - lo).max(T::zero());
    let off = |w: Vec3<T>| eb.dir.cross(w.cross(eb.dir));
    let start = off(ea.p0 + ea.dir * lo - eb.p0);
    let turn = off(ea.dir);
    // An exactly parallel pair has no turn, its offset the same all
    // along the run; the floor only keeps its 0/0 off the division.
    let rate = turn.norm_squared().max(T::from_f64(f64::MIN_POSITIVE));
    let along = (-start.dot(turn) / rate).max(T::zero()).min(run);
    Margin::norm3(start + turn * along)
}

/// `[t0, t1]` on `ea`'s axis clipped to its span: `Some(Some((lo, hi)))`
/// a positive length (`pm_census_ee_overlap`), `Some(None)` a point or
/// empty, `None` escalated (pushed).
fn projected_overlap<T: Decide>(
    ea: &Span<T>,
    (t0, t1): (T, T),
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<(T, T)>> {
    let (blo, bhi) = match tri_cmp(t0, t1, band, errors)? {
        core::cmp::Ordering::Greater => (t1, t0),
        _ => (t0, t1),
    };
    let lo = match tri_cmp(blo, T::zero(), band, errors)? {
        core::cmp::Ordering::Greater => blo,
        _ => T::zero(),
    };
    let hi = match tri_cmp(bhi, ea.len, band, errors)? {
        core::cmp::Ordering::Less => bhi,
        _ => ea.len,
    };
    match decide("pm_census_ee_overlap", Margin::of(hi - lo), band) {
        Ok(Sign::Positive) => Some(Some((lo, hi))),
        Ok(_) => Some(None),
        Err(cause) => {
            errors.push(ValidationError::CensusEscalated { cause });
            None
        }
    }
}

/// **A collinear overlap of two line edges**: a pair whose interiors
/// meet, so it is owed its edge-edge record (D1), or a declared face
/// pair whose interface it lies on ([`Declared::ee_face_backed`]), and
/// the record is confirmed by reconstruction from its bounds: between
/// two backed bounds, two lines sharing two points are one line. Two
/// straight edges ending at one point key at both bounds
/// ([`Bound::Shared`] at each) are one segment by structure, which owes
/// no record (D1). That reaches `split`'s tip copies, and a boolean
/// pinch that collapses each end of its touch to one vertex (a
/// subtraction's or an intersection's). One shared bound is not
/// enough: the other bound's record backs only that bound, never the
/// segment. An overlap with no record, or with a bound
/// [`ee_bound_backed`] does not back, is an `UnrecordedContact`; a
/// bound is never inferred.
#[allow(clippy::too_many_arguments)] // the overlap and its reading, plus the lane's state
fn ee_collinear_lane<T: Decide>(
    ea: &EdgeGeo<T>,
    eb: &EdgeGeo<T>,
    (lo, hi): (T, T),
    margin: MarginDiag,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    let low = ee_bound_backed(ea, eb, lo, geo, declared, band, errors);
    let high = match low {
        Bound::Unbacked => Bound::Unbacked,
        _ => ee_bound_backed(ea, eb, hi, geo, declared, band, errors),
    };
    let backed = low != Bound::Unbacked && high != Bound::Unbacked;
    let recorded = (low, high) == (Bound::Shared, Bound::Shared)
        || declared.ee_recorded(ea.key, eb.key)
        || declared.ee_face_backed(ea, eb);
    if !(backed && recorded) {
        errors.push(ValidationError::UnrecordedContact {
            contact: CensusContact::EdgeEdgeOverlap {
                a: ea.key,
                b: eb.key,
            },
            witness: witness(ea.p0 + ea.dir * geom::mid_param(lo, hi)),
            margin: Some(margin),
        });
    }
}

/// D3 backing for one bound of a collinear overlap: both edges hold a
/// vertex at the bound and the pair is declared (or is one vertex, or
/// two on one point — structural), or — where only ONE edge has a vertex there,
/// so the bound rests on the other edge's interior — that vertex is
/// `(vertex, edge)`-recorded or face-backed onto the other edge.
///
/// The two arms are one rule at two granularities: a bound of a
/// collinear overlap is an endpoint of at least one span, and whether
/// the other span happens to end there too is a fact about the
/// configuration, not about what a declaration can hold. Both edges
/// with a vertex is a v-v event and takes the v-v rungs; one edge with
/// a vertex is a vertex-on-edge event and takes that lane's backing
/// (its `(vertex, edge)` record, or the same declared face pair one
/// incidence step further out).
fn ee_bound_backed<T: Decide>(
    ea: &EdgeGeo<T>,
    eb: &EdgeGeo<T>,
    s: T,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Bound {
    let va = edge_vertex_at(ea, s, band, errors);
    let q = ea.p0 + ea.dir * s;
    let sb = (q - eb.p0).dot(eb.dir);
    let vb = edge_vertex_at(eb, sb, band, errors);
    let backed = match (va, vb) {
        (Some(va), Some(vb)) if geo.same_point(va, vb) => return Bound::Shared,
        (Some(va), Some(vb)) => {
            declared.vv.contains(&(va, vb)) || declared.vv_face_backed(geo, va, vb)
        }
        (Some(va), None) => {
            declared.ve_recorded(va, eb.key)
                || declared.ve_face_backed(geo, va, (eb.f_plus, eb.f_minus))
        }
        (None, Some(vb)) => {
            declared.ve_recorded(vb, ea.key)
                || declared.ve_face_backed(geo, vb, (ea.f_plus, ea.f_minus))
        }
        // Neither edge resolves a vertex at the bound: an escalated
        // span (already pushed), never a backing.
        (None, None) => false,
    };
    if backed {
        Bound::Backed
    } else {
        Bound::Unbacked
    }
}

/// How [`ee_bound_backed`] backs one bound of a collinear overlap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Bound {
    /// Both edges end there, at one point key: structural sharing.
    Shared,
    /// A record or a declared face pair backs it.
    Backed,
    /// Nothing backs it.
    Unbacked,
}

/// **The conformal face-pair arm** (M9-2, C2's structural rung run as
/// a census sweep): for every pair of CURVED faces sharing one
/// `SurfaceKey` with OPPOSED senses — the only configuration that is
/// conformal contact (C1: aligned coincidence is containment/flush,
/// `SameOriented`, and is not contact) — the trim regions' overlap is
/// decided in the shared chart through the PR-1 predicate.
///
/// Scope, stated exactly:
///
/// - **Curved faces only.** Planar conformal interfaces are already
///   fully evidenced at vertex granularity by the exact sweeps (their
///   every boundary event is a v-v/v-f/segment finding those passes
///   report and records back), so the face-pair arm exists for the
///   inventory the exact sweeps cannot read.
/// - **Shared key only.** C2's identity lemma makes every true
///   conformal contact same-carrier for analytic kinds, and within
///   ONE body the structural rung IS key identity; value-equal
///   independent descriptions do not glue (the ladder), and the
///   declared rung's face pairs are the confirm pass's business.
/// - **What stays outside this arm** (module docs carry the full
///   envelope): distinct-key pairs. Cross-solid ones are caught by
///   the conservative loudness backstop
///   ([`sweep_cross_solid_backstop`] — refused undecidable, never
///   silently passed); a same-solid pair on distinct carriers touching
///   along a closed-form tangent line is
///   [`sweep_same_solid_tangent_lines`]'s, and the other same-solid
///   distinct-key pairs are the named residue until the rest of the C9
///   exclusion ring lands.
///
/// A definitely-positive overlap is a FINDING — the kernel
/// vocabulary's [`crate::contact::ContactFinding`], carried on the
/// refusal so the recourse can quote exactly what would verify —
/// and, unbacked by a face-granularity record, refuses as
/// `UnrecordedContact` (discovery is never declaration, F1).
fn sweep_conformal_patches<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    region: Option<RegionLane<T>>,
    errors: &mut Vec<ValidationError>,
) {
    // The scope is the WHOLE body's curved faces, deliberately: a
    // shared key across solids is the structural contact rung this arm
    // reads. `geo.curved_faces` is read off the live face arena, so
    // every face resolves.
    let by_key = crate::chart_groups::ChartGroups::within(body, geo.curved_faces.iter().copied())
        .unwrap_or_else(|face| {
            unreachable!("census: curved face {face:?} was read off the live arena")
        });
    for (_, group) in by_key.iter() {
        for (i, &fa) in group.iter().enumerate() {
            for &fb in &group[i + 1..] {
                let (da, db) = (
                    proven(&body.faces, fa, EntityId::Face),
                    proven(&body.faces, fb, EntityId::Face),
                );
                if da.sense == db.sense {
                    continue; // SameOriented: flush material, not contact (C1)
                }
                match region.map(|lane| lane.chart_overlap(body, fa, body, fb, band)) {
                    None => {
                        // The pass holds no CERTIFIED region door (the
                        // `_structural` twin's value at every scalar,
                        // a dual's only route), so the pair cannot be
                        // decided here — typed, never silent. (A dual
                        // DOES carry a bracket; the refusal is the
                        // door's absence, not a missing `Bounds` impl.)
                        //
                        // Its own variant, and not the one the typed
                        // predicate refusals below raise: this absence
                        // is a fact about the RUN — which door it came
                        // through — and the same candidate is examined
                        // through the certified door at any certifying
                        // scalar, while those are facts about THIS
                        // pair's geometry that no replay changes. One
                        // variant for both made a run-wide condition
                        // read as a per-pair geometric refusal, and the
                        // two recourses are opposite.
                        errors.push(ValidationError::CensusLaneUnsupported {
                            subject: CensusSubject::FacePair(fa, fb),
                        });
                    }
                    Some(Ok(crate::chart_region::ChartOverlap::Empty)) => {}
                    Some(Ok(crate::chart_region::ChartOverlap::PositiveArea)) => {
                        if !declared.faces.contains(&(fa, fb)) {
                            let finding = crate::contact::ContactFinding {
                                pair: crate::contact::DeclaredContact {
                                    a: fa,
                                    b: fb,
                                    class: crate::contact::ContactClass::Rest,
                                },
                                verdict: crate::contact::ContactVerdict::Definite,
                            };
                            // The witness names the LOCUS, not the
                            // pair: the finding beside it carries the
                            // pair typed, and `PositiveArea` is a
                            // region verdict that hands back no point,
                            // so the locus is the region itself.
                            errors.push(ValidationError::UnrecordedContact {
                                contact: CensusContact::ConformalPatch { finding },
                                witness: CONFORMAL_REGION_WITNESS.to_owned(),
                                margin: None,
                            });
                        }
                    }
                    Some(Err(ChartRegionError::Escalated(cause))) => {
                        errors.push(ValidationError::CensusEscalated { cause });
                    }
                    // Every other typed predicate refusal: the pair
                    // was not certified, and WHICH refusal said so is
                    // carried rather than replaced. They do not share
                    // a cause — a stopped interior-witness
                    // search, an absent pcurve cache and a non-planar
                    // trim want three different repairs — so the one
                    // thing this arm may not do is restate them as
                    // the inventory refusal. Refused loudly, never
                    // skipped silently.
                    //
                    // Spelled out rather than matched by wildcard,
                    // because this arm is one half of the
                    // `Escalated`/`Unsupported` discrimination
                    // `editor_core::attribute` turns into `AtRest`
                    // against `Uncertified`: a new `ChartRegionError`
                    // arm must be classified here deliberately rather
                    // than default into an unrefuted frontier.
                    Some(Err(
                        cause @ (ChartRegionError::ChartDivergence { .. }
                        | ChartRegionError::NonPlanarTrim { .. }
                        | ChartRegionError::MissingCache { .. }
                        | ChartRegionError::ArmUnbounded { .. }
                        | ChartRegionError::SeamBranch
                        | ChartRegionError::PeriodFold
                        | ChartRegionError::CarrierTilt
                        | ChartRegionError::TouchingBoundary
                        | ChartRegionError::DegenerateLoop { .. }
                        | ChartRegionError::RayExhausted
                        | ChartRegionError::WitnessSegmentCapExceeded { .. }
                        | ChartRegionError::WitnessCellCapExceeded { .. }
                        | ChartRegionError::Corrupt),
                    )) => {
                        // The refusal is CARRIED, not replaced. The
                        // refusals say different things with different
                        // recourses — a stopped witness search is not
                        // a thin overlap, and neither is an absent
                        // pcurve cache — and flattening them here made
                        // every one of them read as the inventory
                        // statement at the census door.
                        errors.push(ValidationError::CensusUnsupported {
                            subject: CensusSubject::FacePair(fa, fb),
                            cause: CensusUnsupportedCause::ChartRegion(cause),
                        });
                    }
                }
            }
        }
    }
}

/// **Same-solid discovery along a tangent line** (C2): two faces of one
/// solid on distinct carriers that touch along their closed-form
/// tangent line through both interiors, read on the lane that confirms
/// a face–face record on two carriers
/// ([`crate::boolean::tangent_pair_at_rest`], then
/// [`crate::boolean::rest_line_in_both_faces`], then
/// [`crate::boolean::tangent_table_at_rest`] in the detector posture).
///
/// - A void crescent (a rod resting on a plate, bridged into one
///   solid) is a touch, and one no face–face record names is
///   `UnrecordedContact`.
/// - A material crescent (a bore tangent to a face from inside) owes
///   two coincident cusp edges on the line (D1 tier 3), so a pair
///   running through it is [`ValidationError::CuspEdgesMissing`].
/// - Any other contradiction of the table is the lane refusing typed.
///
/// The candidates: faces of one solid, at least one curved, on distinct
/// carriers, sharing no edge, whose padded boxes overlap. A pair the
/// carriers give no closed-form line, or decide not tangent, has no
/// line touch to read; a pair the records name is the confirm pass's.
/// A touch at an isolated point, where a crescent one solid shares with
/// itself is legal, is not this arm's.
fn sweep_same_solid_tangent_lines<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    cands: &Candidates,
    errors: &mut Vec<ValidationError>,
) {
    if geo.curved_faces.is_empty() {
        return;
    }
    let curved: BTreeSet<FaceKey> = geo.curved_faces.iter().copied().collect();
    let face_edges = |f: FaceKey| -> BTreeSet<EdgeKey> {
        body.get_face(f).map_or_else(BTreeSet::new, |face| {
            face_cycles(body, face)
                .filter_map(Result::ok)
                .flatten()
                .filter_map(|he| body.half_edges.get(he).map(|h| h.edge))
                .collect()
        })
    };
    for (i, &a) in cands.every_face.iter().enumerate() {
        let edges_a = face_edges(a);
        for j in cands.trees.every_face.later(i) {
            let b = *candidate(&cands.every_face, j);
            let (Some(fa), Some(fb)) = (body.get_face(a), body.get_face(b)) else {
                continue;
            };
            let solid = body.solid_of_face(a);
            if solid.is_none()
                || solid != body.solid_of_face(b)
                || !(curved.contains(&a) || curved.contains(&b))
                || fa.surface == fb.surface
                || declared.faces.contains(&(a, b))
                || !edges_a.is_disjoint(&face_edges(b))
            {
                continue;
            }
            same_solid_tangent_pair(body, a, b, band, errors);
        }
    }
}

/// One candidate of [`sweep_same_solid_tangent_lines`].
fn same_solid_tangent_pair<T: Decide>(
    body: &Body<T>,
    a: FaceKey,
    b: FaceKey,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    let unsupported = |refusal| ValidationError::CensusUnsupported {
        subject: CensusSubject::FacePair(a, b),
        cause: CensusUnsupportedCause::ContactLane(refusal),
    };
    let (carrier, overlap) = match crate::boolean::tangent_pair_at_rest(body, a, b, band) {
        Ok(crate::boolean::RestTangent::Line { carrier, overlap }) => (carrier, overlap),
        Ok(crate::boolean::RestTangent::Apart | crate::boolean::RestTangent::NoLine)
        | Err(crate::contact::ContactRefusal::Contradicted { .. }) => return,
        Err(
            crate::contact::ContactRefusal::Escalated { diag }
            | crate::contact::ContactRefusal::Undeclared { diag },
        ) => {
            errors.push(ValidationError::CensusEscalated { cause: diag });
            return;
        }
        Err(refusal @ crate::contact::ContactRefusal::NotCertifiable { .. }) => {
            errors.push(unsupported(refusal));
            return;
        }
    };
    let at = witness(carrier.eval((overlap.0 + overlap.1) * T::from_f64(0.5)));
    let through = crate::boolean::rest_line_in_both_faces(
        body,
        [a, b],
        carrier,
        overlap,
        band,
        |i, why, held| match why {
            crate::boolean::FaceUnread::Trim => held.push(unsupported(
                crate::contact::ContactRefusal::NotCertifiable {
                    what: "a face of a two-carrier tangent pair whose trim the containment \
                           door does not read",
                },
            )),
            crate::boolean::FaceUnread::Placement(crate::boolean::PlacementError::OnPlane(
                cause,
            )) => held.push(ValidationError::CensusEscalated { cause }),
            crate::boolean::FaceUnread::Placement(crate::boolean::PlacementError::Door(e)) => {
                read_containment([a, b][i], Err::<(), _>(e), held);
            }
        },
    );
    let line = match through {
        Ok(Some(line)) => line,
        Ok(None) => return,
        Err(held) => {
            errors.extend(held);
            return;
        }
    };
    match crate::boolean::tangent_table_at_rest(&line, false, band) {
        Ok(verdict) => errors.push(ValidationError::UnrecordedContact {
            contact: CensusContact::FaceFaceTangent {
                finding: crate::contact::ContactFinding {
                    pair: crate::contact::DeclaredContact {
                        a,
                        b,
                        class: crate::contact::ContactClass::Tangent,
                    },
                    verdict,
                },
            },
            witness: at,
            margin: None,
        }),
        Err(crate::contact::ContactRefusal::Contradicted { diag, .. })
            if diag.predicate == Some("contact_tangent_overlap") =>
        {
            errors.push(ValidationError::CuspEdgesMissing { a, b, witness: at });
        }
        Err(
            crate::contact::ContactRefusal::Escalated { diag }
            | crate::contact::ContactRefusal::Undeclared { diag },
        ) => errors.push(ValidationError::CensusEscalated { cause: diag }),
        Err(refusal) => errors.push(unsupported(refusal)),
    }
}

/// **`boolean::boxes`'s `FaceBoxRule`/`EdgeBoxRule`, instantiated at
/// this lane's scalar.** Both the rules — which kinds have a cheap
/// sound superset, and by what construction — AND the extent
/// arithmetic that realizes them come from there: the per-kind
/// extents are written once, against `boxes::Span`, and this lane
/// enters them with degenerate spans (`lo == hi`) at its own `T`
/// while the boolean lane enters them with `[lo(), hi()]` brackets at
/// `f64`. Neither takes a bound the other cannot, and no bound is
/// derived twice.
///
/// What this lane still owns is its answer for a description with no
/// claim in it: `None`, versus the poison box there. That is not
/// arithmetic, and `the_two_box_lanes_agree_face_for_face` in
/// `boolean::boxes` pins that what is left cannot drift.
///
/// A NURBS net with poison in any channel — the placeholder, or a
/// described net poisoned in one — gets the poison box from `face_box`,
/// which never prunes. Here it answers `None`: this door keeps the
/// postcondition [`geom_core::CertifiedEnclosure`] states for a
/// certified bracket — a `Some` never carries a NaN end.
///
/// # Panics
///
/// As [`face_reach_in`], to which it forwards.
pub(crate) fn face_reach<T: Decide>(
    body: &Body<T>,
    f: crate::entity::FaceKey,
    band: Band,
) -> Option<(Point3<T>, Point3<T>)> {
    face_reach_in(body, f, band, &crate::boolean::boxes::BoxFrame::World)
}

/// **The axis-aligned hull of `points`**: its low and high corners, each
/// coordinate the least and the greatest. `None` for no points.
pub(crate) fn point_hull<T: Decide>(
    points: impl IntoIterator<Item = Point3<T>>,
) -> Option<(Point3<T>, Point3<T>)> {
    let mut it = points.into_iter();
    let first = it.next()?;
    Some(it.fold((first, first), |(lo, hi), p| {
        (
            Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z)),
            Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z)),
        )
    }))
}

/// [`face_reach`] read in `frame` ([`crate::boolean::boxes::BoxFrame`]):
/// every point and direction the rule reads enters through the frame,
/// and every extent is the same one.
///
/// # Panics
///
/// Where `f`, which every caller read out of `body`, or a record on the
/// walk from it does not resolve, or a loop walk does not close (D2
/// row 4): a torn boundary is not one with no claim. The bodies are at
/// rest (the census, the split's gate), or mid-operation (the split's
/// crossing insertion), where the links hold by
/// [`crate::live::OPERATORS_KEEP_LINKS`].
pub(crate) fn face_reach_in<T: Decide>(
    body: &Body<T>,
    f: crate::entity::FaceKey,
    band: Band,
    frame: &crate::boolean::boxes::BoxFrame<T>,
) -> Option<(Point3<T>, Point3<T>)> {
    let face = proven(&body.faces, f, EntityId::Face);
    let surface = body.face_surface_linked(f, face);
    // A cylinder whose axis is no direction is a broken carrier,
    // and a description with no claim in it answers `None`.
    match crate::boolean::boxes::face_box_rule(surface, band).ok()? {
        crate::boolean::boxes::FaceBoxRule::BoundaryHull => boundary_reach(body, f, face, frame),
        crate::boolean::boxes::FaceBoxRule::ControlNet(patch) => match patch.net_state() {
            // Poison in any channel bounds the locus on no axis, so
            // neither state has a claim to make. `None`, not an
            // exclusion: the face's pairs refuse and its solid is never
            // the container, where dropping it would under-claim.
            geom::NetState::Placeholder | geom::NetState::Poisoned => None,
            geom::NetState::Described => {
                point_hull(patch.control().iter().map(|p| frame.point(*p)))
            }
        },
        crate::boolean::boxes::FaceBoxRule::SphereWindow { .. } => {
            Some(crate::boolean::boxes::sphere_reach(body, f, band, frame))
        }
        crate::boolean::boxes::FaceBoxRule::TorusWindow {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            use crate::boolean::boxes::{Span, SpanBox, meet, torus_extent, torus_window_extent};
            let (c, ax) = (
                SpanBox::point(frame.point(center)),
                SpanBox::vector(frame.vector(axis)),
            );
            let whole = torus_extent(&c, &ax, major_radius, minor_radius);
            // The chart window from the boundary's own stored
            // certified pcurves: the same walk, the same guards and
            // the same extent as the boolean lane, at this lane's
            // scalar ([`torus_chart_window`]).
            Some(span_pts(
                match torus_chart_window(body, f, major_radius, minor_radius) {
                    None => whole,
                    Some((u, v)) => meet(
                        torus_window_extent(
                            &c,
                            &ax,
                            &SpanBox::vector(frame.vector(u_ref)),
                            &SpanBox::vector(frame.vector(axis.cross(u_ref))),
                            Span::exact(major_radius),
                            Span::exact(minor_radius),
                            (u, v),
                        ),
                        whole,
                    ),
                },
            ))
        }
        crate::boolean::boxes::FaceBoxRule::CylinderSlab {
            origin,
            axis,
            radius,
        } => {
            // The axial extent comes from the boundary's own LOCUS
            // — not its vertices, and not a box around it: the axial
            // coordinate is linear along the surface, so the face's
            // axial extremes lie ON the boundary, but not
            // necessarily at a boundary VERTEX.
            let h = boundary_axial(body, f, face, origin, axis.get())?;
            let slab = span_pts(crate::boolean::boxes::slab_extent(
                &crate::boolean::boxes::SpanBox::point(frame.point(origin)),
                &frame.unit(axis),
                h,
                radius,
            ));
            // The clip to the boundary's reach, mirroring the boolean
            // lane's `clip_to_boundary`: a cylinder face lies in its
            // boundary's hull because the cylinder is ruled
            // (`FaceBoxRule`'s `BoundaryHull` arm), so clipping every
            // coordinate is sound.
            Some(match boundary_reach(body, f, face, frame) {
                Some((blo, bhi)) => (
                    Point3::new(
                        slab.0.x.max(blo.x),
                        slab.0.y.max(blo.y),
                        slab.0.z.max(blo.z),
                    ),
                    Point3::new(
                        slab.1.x.min(bhi.x),
                        slab.1.y.min(bhi.y),
                        slab.1.z.min(bhi.z),
                    ),
                ),
                None => slab,
            })
        }
        crate::boolean::boxes::FaceBoxRule::ConeSlab {
            apex,
            axis,
            half_angle,
        } => {
            let h = boundary_axial(body, f, face, apex, axis)?;
            let apex = crate::boolean::boxes::SpanBox::point(frame.point(apex));
            let axis = crate::boolean::boxes::SpanBox::vector(frame.vector(axis));
            Some(span_pts(crate::boolean::boxes::cone_frustum_extent(
                &apex,
                &axis,
                h,
                half_angle.tan(),
            )))
        }
    }
}

/// A torus face's CHART WINDOW at this lane's scalar — the SAME walk
/// the boolean lane runs, not a mirror of it: both enter
/// [`crate::boolean::boxes::face_window_steps`] and
/// [`crate::boolean::boxes::torus_chart_window`], which is where the
/// two guards and every fail mode live.
///
/// # Panics
///
/// As [`crate::boolean::boxes::face_window_steps`]: `f` is a face
/// [`face_reach_in`] resolved, and a torn hop past it is a kernel bug.
pub(crate) fn torus_chart_window<T: Decide>(
    body: &Body<T>,
    f: crate::entity::FaceKey,
    major: T,
    minor: T,
) -> Option<crate::boolean::boxes::TorusWindowPair<T>> {
    crate::boolean::boxes::torus_chart_window(
        &crate::boolean::boxes::face_window_steps(body, f),
        major,
        minor,
    )
}

/// One boundary edge's carrier as an axial span reads it
/// ([`crate::boolean::boxes::axial_carrier`], at this lane's scalar).
fn axial_carrier<T: Decide>(
    body: &Body<T>,
    ek: EdgeKey,
    e: &crate::entity::Edge,
) -> crate::boolean::boxes::AxialCarrier<T> {
    use crate::boolean::boxes::{SpanBox, edge_box_rule};
    crate::boolean::boxes::axial_carrier(
        edge_box_rule(body.edge_curve_linked(ek, e).certified()),
        SpanBox::point,
        SpanBox::vector,
        |x| x,
        |span| span,
    )
}

/// The face boundary's AXIAL range about `(origin, axis)` — the
/// boundary's own locus projected on the axis, edge by edge, rather
/// than the corners of a box around it
/// ([`crate::boolean::boxes::edge_axial_span`], which carries why the
/// difference is not cosmetic at a tilted axis). `None` for a face
/// with no boundary, or with a boundary edge whose axial span has no
/// claim ([`axial_carrier`]). `face` is `f`'s record, resolved by
/// [`face_reach_in`]; every hop past it is a link and panics as there.
fn boundary_axial<T: Decide>(
    body: &Body<T>,
    f: crate::entity::FaceKey,
    face: &Face,
    origin: Point3<T>,
    axis: Vec3<T>,
) -> Option<crate::boolean::boxes::Span<T>> {
    use crate::boolean::boxes::{AxialCarrier, SpanBox, edge_axial_span, edge_end_point};
    let (o, ax) = (SpanBox::point(origin), SpanBox::vector(axis));
    let mut acc: Option<crate::boolean::boxes::Span<T>> = None;
    for member in body.face_boundary_linked(f, face) {
        let sp = match member {
            BoundaryMember::Isolated { point: p, .. } => {
                let p = SpanBox::point(p);
                edge_axial_span(&o, &ax, &AxialCarrier::Chord, (&p, &p))
            }
            BoundaryMember::Edge { ek, edge: e, .. } => {
                let end = |h, field| SpanBox::point(edge_end_point(body, ek, h, field));
                let axial = axial_carrier(body, ek, e);
                if let AxialCarrier::Unclaimable = axial {
                    return None;
                }
                let (a, b) = (end(e.he_plus, "he_plus"), end(e.he_minus, "he_minus"));
                edge_axial_span(&o, &ax, &axial, (&a, &b))
            }
        };
        acc = Some(acc.map_or(sp, |a| a.hull(sp)));
    }
    acc
}

/// Every boundary edge's reach, hulled with the isolated-vertex loops
/// (which have no edge to speak for them). `None` as soon as one
/// boundary curve has no sound box — [`face_reach`]'s boundary walk.
/// `face` is `f`'s record, resolved by [`face_reach_in`]; every hop
/// past it is a link and panics as there.
fn boundary_reach<T: Decide>(
    body: &Body<T>,
    f: crate::entity::FaceKey,
    face: &Face,
    frame: &crate::boolean::boxes::BoxFrame<T>,
) -> Option<(Point3<T>, Point3<T>)> {
    let mut acc: Option<(Point3<T>, Point3<T>)> = None;
    let mut grow = |(lo, hi): (Point3<T>, Point3<T>)| {
        acc = Some(match acc {
            None => (lo, hi),
            Some((l, h)) => (
                Point3::new(l.x.min(lo.x), l.y.min(lo.y), l.z.min(lo.z)),
                Point3::new(h.x.max(hi.x), h.y.max(hi.y), h.z.max(hi.z)),
            ),
        });
    };
    for member in body.face_boundary_linked(f, face) {
        match member {
            BoundaryMember::Isolated { point: p, .. } => {
                let p = frame.point(p);
                grow((p, p));
            }
            BoundaryMember::Edge { ek, edge, .. } => grow(edge_reach_of(body, ek, edge, frame)?),
        }
    }
    acc
}

/// One edge's reach — [`crate::boolean::boxes::EdgeBoxRule`] at this
/// lane's scalar, read in `frame` as [`face_reach_in`] reads a face.
/// `None` where the edge's carrier has no sound box, null scaffolding
/// included.
///
/// # Panics
///
/// Where `ek`, which every caller read out of `body`, or a record past
/// it does not resolve, as [`face_reach_in`]: a torn curve is not null
/// scaffolding.
pub(crate) fn edge_reach_in<T: Decide>(
    body: &Body<T>,
    ek: crate::entity::EdgeKey,
    frame: &crate::boolean::boxes::BoxFrame<T>,
) -> Option<(Point3<T>, Point3<T>)> {
    edge_reach_of(body, ek, proven(&body.edges, ek, EntityId::Edge), frame)
}

/// [`edge_reach_in`] past its edge's lookup: `e` is `ek`'s record.
///
/// No public door builds a spiric-bearing operand that reaches the
/// contact census at this head (the boolean's operand gate refuses the
/// kind; a hollowed partial revolve stops at tier 3's check 7 before
/// the census), so the spiric arm is exercised by the box module's
/// hand-built sector rows.
fn edge_reach_of<T: Decide>(
    body: &Body<T>,
    ek: crate::entity::EdgeKey,
    e: &crate::entity::Edge,
    frame: &crate::boolean::boxes::BoxFrame<T>,
) -> Option<(Point3<T>, Point3<T>)> {
    use crate::boolean::boxes::{ConicArc, Span, SpanBox, arc_extent, edge_end_point};
    let end = |he, field| SpanBox::point(frame.point(edge_end_point(body, ek, he, field)));
    // The ARC's own extent, by `arc_extent`'s subdivision plus sagitta
    // charge — not the exact arc box `edge_box` reads, because that one
    // asks whether an extremal angle lies in the span and this scalar
    // carries no ordering to answer. One rule, two arithmetics, the
    // census box wider by at most the charge: `EdgeBoxRule`'s conic
    // bullet states it, `the_two_box_lanes_agree_face_for_face` pins it.
    let conic = |arc: &ConicArc<'_, T>| {
        arc_extent(
            &SpanBox::point(frame.point(arc.center)),
            &SpanBox::vector(frame.vector(arc.u_ref)),
            &SpanBox::vector(frame.vector(arc.axis.cross(arc.u_ref))),
            Span::exact(arc.semi_u),
            Span::exact(arc.semi_v),
            arc.params.0,
            arc.params.1,
        )
    };
    crate::boolean::boxes::edge_rule_box(
        crate::boolean::boxes::edge_box_rule(body.edge_curve_linked(ek, e).certified()),
        (end(e.he_plus, "he_plus"), end(e.he_minus, "he_minus")),
        frame,
        SpanBox::point,
        conic,
    )
    .map(span_pts)
}

/// A [`crate::boolean::boxes::SpanBox`] as this lane's `(lo, hi)`
/// corner pair, and back.
pub(crate) fn span_pts<T: Decide>(s: crate::boolean::boxes::SpanBox<T>) -> (Point3<T>, Point3<T>) {
    (
        Point3::new(s.x.lo, s.y.lo, s.z.lo),
        Point3::new(s.x.hi, s.y.hi, s.z.hi),
    )
}

/// **Why the cross-solid backstop left a pair undecided** — the one
/// source of every `what` a [`ValidationError::CensusUndecidable`]
/// carries. Each reason's sentence ends in its own recourse, or says
/// plainly that there is none yet, because the reasons want different
/// repairs: a standing crossing wants that crossing fixed, a damaged
/// instance is a kernel defect, and a designed resting contact the
/// backstop cannot place has no way through yet.
///
/// A raise site names a variant and never writes a sentence:
/// `the_backstop_writes_no_sentence_of_its_own` holds that, so the
/// enum's variants are every `what` a run can render.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// Every reason, for the samples: the roster is the enum itself.
#[cfg_attr(any(test, feature = "test-support"), derive(strum::EnumIter))]
pub(crate) enum Undecided {
    /// Arm 1: faces of two solids within reach, one of them curved.
    CurvedWithinReach,
    /// Arm 1: a face with no sound reach bound.
    NoSoundReach,
    /// Arm 2: the containing instance's extent is unclaimable.
    Unclaimable,
    /// Arm 2: a crossing already stands against the pair.
    Crossing,
    /// Arm 2: an unexamined or escalated finding already stands.
    Unexamined,
    /// Arm 2: a touch where one material passes into the other — a
    /// crossing at a lower-dimensional feature.
    MixedTouch,
    /// Arm 2: a touch whose analysis needed a sign in band.
    TouchInBand,
    /// Arm 2: a touch where one face, seen from the touch point, could
    /// not be traced on decided readings.
    TouchPieceInBand,
    /// Arm 2: a touch at a curved face or edge.
    TouchUnreadable,
    /// Arm 2: a touch where a face meets the touch point both at a
    /// corner and along an edge, or a solid's faces there do not close.
    TouchUnanalysed,
    /// Arm 2: a touch whose readings of one solid's faces contradict
    /// each other.
    TouchDegenerate,
    /// Arm 2: a touch at a corner whose edge length is not a number.
    TouchChordScale,
    /// Arm 2: a face-pair contact record backing sideless events.
    RecordedFacePair,
    /// Arm 2: every vertex of the contained instance on the boundary.
    AllOn,
    /// Arm 2: the contained instance has no vertex.
    NoVertex,
    /// Arm 2: the point-in-solid door could not place a vertex near
    /// the boundary: its planar loop walk escalated on the decision
    /// carried, or (`None`) the door escalated on a row of its own, which
    /// it does not name.
    WitnessTooClose(Option<crate::splitting::LoopDecision>),
    /// Arm 2: no ray the point-in-solid door cast settled — each grazed
    /// or gave nothing to read — for a vertex its pre-pass placed off the
    /// boundary.
    WitnessGrazed,
    /// Arm 2: a vertex near the end of a narrow cylinder wall's window,
    /// which the point-in-solid door's window reading cannot place.
    WitnessNarrowWindow,
    /// Arm 2: a ray the point-in-solid door cast met a torus face where
    /// the kernel's count of its crossings came apart.
    WitnessRootCount,
    /// Arm 2: an instance of (near-)zero signed volume. Tier 3's +V
    /// check passes a volume in band of zero, so the part is the
    /// model's to fix, as tier 3 reads it too.
    ZeroVolume,
    /// Arm 2: an instance whose volume's sign is uncertified.
    VolumeUncertified,
    /// Arm 2: an instance with a face kind the door does not serve.
    FaceKindUnsupported,
    /// Arm 2: an instance whose topology the door could not walk.
    CorruptInstance,
    /// Arm 2: a flat face of an instance the door could not read in
    /// its own plane.
    OffPlane,
}

/// [`Undecided::WitnessTooClose`]'s sentence, for every decision the
/// loop walk carries and for none: the walk's question about the corner
/// tested (the point its subject names), and its lever, both read from
/// [`crate::boolean::placement_subject`] and
/// [`crate::boolean::placement_lever`]. The margin that would value a
/// tighter tolerance stops here: `what` is a `&'static str`, so each
/// sentence is built once and kept.
fn witness_too_close(decision: Option<crate::splitting::LoopDecision>) -> &'static str {
    type Sentences = Vec<(Option<crate::splitting::LoopDecision>, String)>;
    static SENTENCES: std::sync::LazyLock<Sentences> = std::sync::LazyLock::new(|| {
        core::iter::once(None)
            .chain(crate::splitting::LoopDecision::ALL.map(Some))
            .map(|decision| {
                let carried = decision.map(Into::into);
                let sentence = format!(
                    "testing a corner of one against the other, {} is undecided at this \
                     tolerance. Recourse: {}",
                    crate::boolean::placement_subject(carried),
                    crate::boolean::placement_lever(carried)
                );
                (decision, sentence)
            })
            .collect()
    });
    SENTENCES
        .iter()
        .find(|(d, _)| *d == decision)
        .map_or("", |(_, sentence)| sentence.as_str())
}

impl Undecided {
    /// Whether arm 1 raises it, on a face pair; arm 2 raises the rest,
    /// on a solid pair.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn on_faces(self) -> bool {
        matches!(self, Self::CurvedWithinReach | Self::NoSoundReach)
    }

    /// Whether this is a touch the local analysis could not decide —
    /// a reason that waits in `blocks` for a decided one.
    fn is_undecided_touch(self) -> bool {
        matches!(
            self,
            Self::TouchInBand
                | Self::TouchPieceInBand
                | Self::TouchUnreadable
                | Self::TouchUnanalysed
                | Self::TouchDegenerate
                | Self::TouchChordScale
        )
    }

    /// The reason and its recourse, as the finding renders them.
    ///
    /// "Until their bounding boxes no longer overlap" is the distance
    /// that clears a pair: arm 1 examines only faces within each
    /// other's reach box and arm 2 only instances whose padded extents
    /// overlap, so a recourse to move the parts is true exactly past
    /// that — and is not offered by a reason no distance clears.
    pub(crate) fn what(self) -> &'static str {
        match self {
            // Arm 1 is gated by the faces' reach boxes, so moving the
            // parts until those boxes no longer overlap clears it; a
            // designed contact (a boss in a hole, a cradle, an embedded
            // cap) lands here too and has no way through yet.
            Self::CurvedWithinReach => {
                "a curved face of one is within reach of the other, and the kernel cannot \
                 yet tell whether curved faces of two parts touch. There is no way through \
                 yet for a designed contact; otherwise move the parts until their bounding \
                 boxes no longer overlap"
            }
            // Neither this nor `Unclaimable` is gated by distance: a
            // face with no sound reach bound is refused at ANY
            // separation, so moving the parts is no way through. A
            // placeholder surface is also reported on its own, as the
            // kernel defect it is (`UncertifiableSurface`).
            Self::NoSoundReach => {
                "one face has no extent the check can bound (a placeholder surface, or an \
                 edge whose curve has no bounds), so no distance clears it. There is no way \
                 through yet for this shape"
            }
            Self::Unclaimable => {
                "a face of one has no extent the check can bound, so where that part ends \
                 is unknown at any distance. There is no way through yet for this shape"
            }
            Self::Crossing => {
                "another finding reports their boundaries crossing, and this one cannot be \
                 decided until that is fixed. Recourse: fix that crossing first"
            }
            Self::Unexamined => {
                "another finding left their boundaries unchecked, and this one cannot be \
                 decided until that is resolved. Recourse: resolve that finding first"
            }
            Self::MixedTouch => {
                "one passes into the other where they touch. Recourse: move them so they \
                 meet without overlapping, or not at all"
            }
            Self::TouchInBand => {
                concat!(
                    "they touch, and ",
                    geom_core::undecided!("whether they overlap there"),
                    ". There is no way through yet for a designed resting contact; otherwise \
                     move them until their bounding boxes no longer overlap"
                )
            }
            Self::TouchPieceInBand => {
                "they touch where a face of one, seen from the touch, has corners or edges \
                 too nearly in line to trace at this tolerance, so the check cannot tell \
                 whether they overlap there. There is no way through yet for a designed \
                 resting contact; otherwise move them until their bounding boxes no longer \
                 overlap"
            }
            Self::TouchUnreadable => {
                "they touch at a curved face or edge, and the check cannot yet tell which \
                 side each is on. There is no way through yet for a designed resting \
                 contact; otherwise move them until their \
                 bounding boxes no longer overlap"
            }
            Self::TouchUnanalysed => {
                "they touch where a face of one meets it along an edge and again, or where \
                 one's faces there do not close around it, so the check cannot tell whether \
                 they overlap. There is no way through yet for \
                 a designed resting contact; otherwise move them until their bounding boxes \
                 no longer overlap"
            }
            Self::TouchDegenerate => {
                "they touch where the check's readings of one's faces contradict each \
                 other (as two faces folded onto each other do), so it cannot tell whether \
                 they overlap there. There is no way through yet for this shape; otherwise \
                 move them until their bounding boxes no longer overlap"
            }
            Self::TouchChordScale => {
                "they touch at a corner with an edge too long or too short to measure. \
                 Recourse: rescale the model so every edge length is an ordinary number, \
                 or move them until their bounding boxes no longer overlap"
            }
            Self::RecordedFacePair => {
                "a contact record between their faces touches only at corners the check \
                 cannot yet place. There is no way through yet for this record"
            }
            Self::AllOn => {
                "every corner of one lies on the other's boundary, so the corners cannot \
                 place it. There is no way through yet for a designed resting fit; \
                 otherwise move them until their \
                 bounding boxes no longer overlap"
            }
            Self::NoVertex => {
                "one has no corner to test, so nothing places it. There is no way through \
                 yet for this shape; if they are not meant to meet, move them until their \
                 bounding boxes no longer overlap"
            }
            Self::WitnessTooClose(decision) => witness_too_close(decision),
            Self::WitnessGrazed => {
                "a corner of one is off the other's boundary, but no test ray from it \
                 settled where it lies: each grazed that boundary or could not be read. \
                 Recourse: move the parts until their bounding boxes no longer overlap"
            }
            Self::WitnessNarrowWindow => {
                "a corner of one lies near the end of a narrow cylinder wall of the other, \
                 where the check's reading of the wall's window cannot place it. Recourse: \
                 move the parts until their bounding boxes no longer overlap; a limit of the \
                 kernel's narrow-window reading, worth reporting"
            }
            Self::WitnessRootCount => {
                "a test ray from a corner of one met a torus face of the other where the \
                 kernel's count of its crossings came apart. Recourse: move the parts until \
                 their bounding boxes no longer overlap; this may indicate a kernel bug worth \
                 reporting"
            }
            Self::ZeroVolume => {
                "one has no volume, so nothing can be inside it. Recourse: fix that part \
                 so it encloses a volume"
            }
            Self::VolumeUncertified => {
                "the check could not certify one's volume, so it cannot tell that part's \
                 inside from its outside. Recourse: move the parts until \
                 their bounding boxes no longer overlap"
            }
            Self::FaceKindUnsupported => {
                "one has a face (a spline, part of a sphere, cone or torus, a flat \
                 face edged by one, or a cylinder wall with a hole or an edge that \
                 is not a flat cut) the check cannot yet test a point against. Recourse: move the parts until \
                 their bounding boxes no longer overlap"
            }
            Self::CorruptInstance => concat!(
                "one part's topology could not be walked. ",
                geom_core::kernel_or_file_defect_ending!()
            ),
            Self::OffPlane => concat!(
                "the check could not read a flat face of one in that face's own plane. ",
                geom_core::kernel_or_file_defect_ending!()
            ),
        }
    }

    /// The reason a point-in-solid refusal leaves a placement
    /// undecided. Exhaustive, so a new refusal is placed here by hand.
    pub(crate) fn of_point_in_solid(e: &crate::boolean::PointInSolidError) -> Self {
        use crate::boolean::PointInSolidError as E;
        use crate::splitting::PointInLoopError as L;
        match e {
            E::Escalated { .. } | E::JunctionSidesDisagree { .. } => Self::WitnessTooClose(None),
            E::NarrowWindowUnread { .. } => Self::WitnessNarrowWindow,
            E::RootCountDisagrees { .. } => Self::WitnessRootCount,
            E::Loop(L::Escalated { decision, .. }) => Self::WitnessTooClose(Some(*decision)),
            E::RayExhausted | E::Loop(L::RayExhausted { .. }) => Self::WitnessGrazed,
            E::ZeroVolumeBody => Self::ZeroVolume,
            E::VolumeUncertified => Self::VolumeUncertified,
            E::KindUnsupported { .. }
            | E::PartialSphereFace { .. }
            | E::PartialConeFace { .. }
            | E::PartialTorusFace { .. }
            | E::EdgeCarrierUnsupported { .. }
            | E::WallOutlineUnsupported { .. }
            | E::Loop(L::Uncrossable(_)) => Self::FaceKindUnsupported,
            E::CorruptFace { .. } | E::NoSuchSolid { .. } | E::Loop(L::CorruptLoop { .. }) => {
                Self::CorruptInstance
            }
            E::Loop(L::OffPlane(_)) => Self::OffPlane,
            E::InsideOut => unreachable!(
                "the census reads its instances' own selections, never one read as given, \
                 and only that refuses InsideOut"
            ),
        }
    }
}

/// What a standing finding says, read by arm 2 against one solid pair.
#[derive(Clone, Copy, Debug)]
enum Said {
    /// A planar touch, read through [`TouchSite::verdict`].
    Touch(TouchSite),
    /// An edge through a face's interior: a crossing.
    Pierce,
    /// A curved touch: a conformal patch (the conformal arm finds
    /// same-carrier CURVED pairs only), or a tangent line.
    CurvedTouch,
    /// A candidate an arm left unexamined (arm 1's refusal, an
    /// unsupported face pair).
    Unexamined,
}

/// What a standing finding names — the ONE mapping from a finding to
/// the solid pairs it is about, shared by arm 2's `meets` and `blocks`.
#[derive(Clone, Copy, Debug)]
enum Named {
    /// Two entities, and what the finding says of them.
    Two(EntityId, EntityId, Said),
    /// One entity left unexamined against everything.
    One(EntityId),
    /// No entity: an escalation, or a finding this arm does not know.
    Nothing,
}

impl Named {
    fn of(e: &ValidationError) -> Self {
        match e {
            ValidationError::UnrecordedContact { contact, .. } => match TouchSite::of(contact) {
                Some(site) => {
                    let (x, y) = site.entities();
                    Self::Two(x, y, Said::Touch(site))
                }
                None => match *contact {
                    CensusContact::EdgeFacePierce { edge, face } => {
                        Self::Two(EntityId::Edge(edge), EntityId::Face(face), Said::Pierce)
                    }
                    CensusContact::ConformalPatch { ref finding }
                    | CensusContact::FaceFaceTangent { ref finding } => Self::Two(
                        EntityId::Face(finding.pair.a),
                        EntityId::Face(finding.pair.b),
                        Said::CurvedTouch,
                    ),
                    // `TouchSite::of` maps every other kind.
                    _ => Self::Nothing,
                },
            },
            ValidationError::CensusUnsupported { subject, .. }
            | ValidationError::CensusLaneUnsupported { subject } => match *subject {
                CensusSubject::Entity(id) => Self::One(id),
                CensusSubject::FacePair(f, g) => {
                    Self::Two(EntityId::Face(f), EntityId::Face(g), Said::Unexamined)
                }
            },
            ValidationError::CensusUndecidable { a, b, .. } => Self::Two(*a, *b, Said::Unexamined),
            _ => Self::Nothing,
        }
    }
}

/// A contact record naming one entity of each of a solid pair, as
/// arm 2 reads it.
#[derive(Clone, Copy, Debug)]
enum Recorded {
    /// A v-on-f or v-v record: a touch site the analysis reads.
    Site(TouchSite),
    /// A curve or patch face-pair record: it backs events without a side.
    FacePair,
}

// ---- The touch analysis: arm 2's clear, condition 3.
//
// A touch is read once per solid pair and touch point, and each side is
// its solid's whole LINK there ([`Link`]): every face of the solid
// through the point, as its sector. One test of two polygonal cones
// decides the pair ([`rest::judge`]).
//
// Every sign it decides as a verdict is a point's signed distance from
// a plane, in metres, and it comes through ONE door: [`metric`], whose
// [`metric::Distance`] is built only from a point and a plane (a point
// on it and its unit normal). Directions — face normals, ray
// directions, their bisectors — only PROPOSE planes. One other decision
// stays, named where it is made: [`pairing`], a sign of unit normals of
// magnitude about 1. `census::tests::
// the_touch_analysis_decides_only_through_its_doors` holds that by
// reading this section's source.
//
// Each face is read through its PIECE at the touch point `p`: the part
// of the face visible from `p` ([`face_piece`]). The piece is
// star-shaped from `p`, so every point of it lies on a segment from `p`
// to its boundary, and it spans the face's whole sector at `p`. Nothing
// is extrapolated, and a face that is not convex is read where it is
// near `p`, not where it reaches back far from it.

/// K name: a point's signed distance from the plane of a link's face
/// through the touch point `p`, in metres — which side of the face's
/// plane a piece's end, a probe or a parity arc's end lies on.
const TOUCH_PLANE: &str = "census_touch_plane";
/// K name: a point's signed distance from the plane through a ray from
/// `p` (a sector's ray, or a face vertex's while its piece is built),
/// normal to its face, in metres — which side of the ray the point lies
/// on, in the face's plane, read no further out than the ray's point.
const TOUCH_WEDGE: &str = "census_touch_wedge";
/// K name: a point on a ray's line through `p` read along the ray, in
/// metres — ahead of `p` or behind it, read no further out than the
/// ray's point.
const TOUCH_AHEAD: &str = "census_touch_ahead";
/// K name: a parity arc's two ends read against the plane through `p`
/// normal to their bisector, in metres — the arc under half a turn.
const TOUCH_ARC: &str = "census_touch_arc";
/// K name, building a face's piece: one edge's crossing of a probe ray
/// read against another's along the ray, in metres — which edge the ray
/// meets first.
const TOUCH_PIECE_REACH: &str = "census_touch_piece_reach";
/// K name, building a face's piece: where a ray through a gap's bound
/// meets the gap's edge, read against that edge's ends along it, in
/// metres — on the edge, or past an end.
const TOUCH_PIECE_MEET: &str = "census_touch_piece_meet";
/// K name: two coplanar faces' outward normals, one of each link,
/// aligned or opposed ([`geom_brep::classify_material_pairing_as`]).
const TOUCH_NORMAL: &str = "census_touch_normal";

mod curved;
mod depth_gap;
#[cfg(test)]
mod depth_gap_fuzz;
#[cfg(test)]
mod link_fuzz;
mod point;
#[cfg(test)]
mod segment_fuzz;

/// **The one door every verdict sign of the touch analysis comes
/// through.** A [`Distance`] has a private field and one constructor,
/// from a point and a plane, so a lever, a product or any other
/// quantity cannot be decided here as a verdict: it has no way to
/// become a `Distance`.
///
/// What the type cannot hold: that `n` is a unit normal. Every caller
/// passes a normalized direction or a face's outward normal, and a
/// normal scaled by a length would be a lever spelled as a plane; the
/// source row states that blind spot.
mod metric {
    use super::{Band, Decide, Margin, Point3, Real, Sign, Vec3, decide};

    /// A point's signed distance from a plane, in metres.
    pub(super) struct Distance<T>(T);

    impl<T: Real> Distance<T> {
        /// `n·(q − o)`: `q`'s signed distance from the plane through
        /// `o` with unit normal `n`.
        pub(super) fn of(q: Point3<T>, o: Point3<T>, n: Vec3<T>) -> Self {
            Self(n.dot(q - o))
        }
    }

    /// How a link reading decides: the run band, and a slack — the
    /// farthest a link's sector stands from the point it is read about
    /// (a corner on another key of the point, decided one point with
    /// it). A definite sign is taken only where it holds over every
    /// point within the slack of the one read, so no sector's offset
    /// can flip it; a Zero stays the band's coincidence, the slack being
    /// within it. `None` where the reading is about the sector's own
    /// apex ([`face_piece`]), so nothing is moved and the one read is
    /// the reading.
    #[derive(Clone, Copy)]
    pub(super) struct Reading<T> {
        pub(super) band: Band,
        pub(super) slack: Option<T>,
    }

    impl<T: Real> Reading<T> {
        /// The slack as a distance: zero where there is none.
        pub(super) fn slack(&self) -> T {
            self.slack.unwrap_or(T::zero())
        }
    }

    /// The distance's sign under the run band, as the K row `name`;
    /// `None` in band.
    pub(super) fn sign<T: Decide>(name: &'static str, d: Distance<T>, band: Band) -> Option<Sign> {
        read(name, d.0, band)
    }

    /// The distance's sign under `r` ([`Reading`]): Zero as the band
    /// reads it, a definite sign only where the distance moved by the
    /// slack toward zero reads the same; `None` otherwise.
    pub(super) fn sign_over<T: Decide>(
        name: &'static str,
        d: Distance<T>,
        r: Reading<T>,
    ) -> Option<Sign> {
        let s = read(name, d.0, r.band)?;
        let Some(slack) = r.slack else {
            return Some(s);
        };
        match s {
            Sign::Zero => Some(Sign::Zero),
            Sign::Positive => {
                (read(name, d.0 - slack, r.band)? == Sign::Positive).then_some(Sign::Positive)
            }
            Sign::Negative => {
                (read(name, d.0 + slack, r.band)? == Sign::Negative).then_some(Sign::Negative)
            }
        }
    }

    fn read<T: Decide>(name: &'static str, v: T, band: Band) -> Option<Sign> {
        decide(name, Margin::of(v), band).ok()
    }
}

use metric::{Distance, Reading};

/// What the local analysis of one touch between two solids decided.
#[derive(Debug, PartialEq, Eq)]
enum TouchVerdict {
    /// Every piece of both links read, on decided readings, as no
    /// crossing: the materials' interiors are disjoint near the touch.
    /// The certificate is minted only by [`rest::judge`], which runs
    /// the readings.
    Rest(rest::Rest),
    /// The two materials' interiors meet at the touch, on decided
    /// readings: the touch is the degenerate shape of a crossing.
    Crossing,
    /// A sign the analysis needed is in band.
    InBand,
    /// A face's piece at the touch did not build: a choice in building
    /// it was in band, or its readings contradicted each other within
    /// ε. The smallest piece of all, which refuses.
    PieceInBand,
    /// A curved face or edge takes part.
    Unreadable,
    /// A face meets the touch point both at a corner and along an edge,
    /// or along two of its edges; its piece there does not close on
    /// decided readings (a gap between neighbouring turns of half a turn
    /// or more, or no edge met across one, [`face_piece`]); or a link's
    /// sectors do not pair up at their bounds ([`Link::closes`]).
    Unanalysed,
    /// Two faces of one solid lie on each other at the touch (a slit,
    /// or a face doubled).
    Degenerate,
    /// A link's extent is too long or too short to be a number.
    ChordScale,
    /// The topology around the touch did not read. The census runs only
    /// on bodies tier 1 admits (loops closed, every key resolving), so
    /// this is a kernel defect.
    Corrupt,
}

impl TouchVerdict {
    /// The refusal a verdict pushes, or `None` for a rest.
    fn refusal(&self) -> Option<Undecided> {
        match self {
            Self::Rest(_) => None,
            Self::Crossing => Some(Undecided::MixedTouch),
            Self::InBand => Some(Undecided::TouchInBand),
            Self::PieceInBand => Some(Undecided::TouchPieceInBand),
            Self::Unreadable => Some(Undecided::TouchUnreadable),
            Self::Unanalysed => Some(Undecided::TouchUnanalysed),
            Self::Degenerate => Some(Undecided::TouchDegenerate),
            Self::ChordScale => Some(Undecided::TouchChordScale),
            Self::Corrupt => Some(Undecided::CorruptInstance),
        }
    }

    #[cfg(test)]
    fn is_rest(&self) -> bool {
        matches!(self, Self::Rest(_))
    }
}

/// Where a touch between two solids is, as a finding or a record names
/// it: the one mapping from a touch kind to what the analysis reads,
/// shared by unrecorded findings and contact records.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum TouchSite {
    VertexVertex(VertexKey, VertexKey),
    VertexOnEdge(VertexKey, EdgeKey),
    EdgeEdge(EdgeKey, EdgeKey),
    VertexOnFace(VertexKey, FaceKey),
    EdgeInFace(EdgeKey, FaceKey),
    /// Two edges crossing at one point interior to both
    /// ([`CensusContact::EdgeEdgeCross`]). The finding exists only
    /// where the pass-5 sweep decided the two lines meet
    /// (`pm_census_ee_gap`, in band escalated there), so the two edges
    /// always span one plane: a beam's bottom edge over a support's top
    /// edge, or two ridges crossed edge on edge. The touch is the
    /// crossing point, and the two links are the edges' two faces
    /// each. A cross whose materials pass into each other is not a
    /// separate class the site has to screen out: the analysis reads
    /// it as a crossing.
    EdgeCross(EdgeKey, EdgeKey),
}

/// The unit a touch is read in: one verdict per unit, shared by every
/// finding and record in it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum TouchUnit {
    /// Two solids' links at one touch point ([`touch_keys`]), named by
    /// the solids in order and the point's least key.
    Point(crate::SolidKey, crate::SolidKey, PointKey),
    /// A touch at no vertex (an edge's or a face's interior), read at
    /// the site alone.
    Site(TouchSite),
}

/// Every pair a finding or a contact record ties to a point: a vertex
/// on a vertex, an edge or a face. A link gathers what they tie to its
/// touch point.
#[derive(Default)]
struct Ties {
    vv: Vec<(VertexKey, VertexKey)>,
    ve: Vec<(VertexKey, EdgeKey)>,
    vf: Vec<(VertexKey, FaceKey)>,
}

impl Ties {
    fn of(declared: &Declared, findings: &[ValidationError]) -> Self {
        let mut ties = Self {
            vv: declared.vv.iter().copied().collect(),
            ve: declared.ve.iter().copied().collect(),
            vf: declared.vf.iter().copied().collect(),
        };
        for e in findings {
            let ValidationError::UnrecordedContact { contact, .. } = e else {
                continue;
            };
            match *contact {
                CensusContact::VertexVertex { a, b } => ties.vv.push((a, b)),
                CensusContact::VertexOnEdge { vertex, edge } => ties.ve.push((vertex, edge)),
                CensusContact::VertexOnFace { vertex, face } => ties.vf.push((vertex, face)),
                _ => {}
            }
        }
        ties
    }
}

/// **A touch point's keys**: those of `seed`, closed under the
/// vertex–vertex ties between their vertices — one point, however many
/// keys a value decision left it on. `None` where a vertex has no key.
fn touch_keys<T: Real>(
    geo: &Geo<T>,
    ties: &Ties,
    seed: &[VertexKey],
) -> Option<BTreeSet<PointKey>> {
    let mut keys = seed
        .iter()
        .map(|v| geo.vpoint.get(v).copied())
        .collect::<Option<BTreeSet<_>>>()?;
    loop {
        let before = keys.len();
        for (a, b) in &ties.vv {
            if let (Some(&ka), Some(&kb)) = (geo.vpoint.get(a), geo.vpoint.get(b))
                && keys.contains(&ka) != keys.contains(&kb)
            {
                keys.insert(ka);
                keys.insert(kb);
            }
        }
        if keys.len() == before {
            return Some(keys);
        }
    }
}

impl TouchSite {
    /// The site a touch finding names; `None` for a finding that is not
    /// a planar touch (a pierce, a conformal patch).
    fn of(contact: &CensusContact) -> Option<Self> {
        Some(match *contact {
            CensusContact::VertexVertex { a, b } => Self::VertexVertex(a, b),
            CensusContact::VertexOnEdge { vertex, edge } => Self::VertexOnEdge(vertex, edge),
            CensusContact::EdgeEdgeOverlap { a, b } => Self::EdgeEdge(a, b),
            CensusContact::VertexOnFace { vertex, face } => Self::VertexOnFace(vertex, face),
            CensusContact::EdgeFaceOverlap { edge, face } => Self::EdgeInFace(edge, face),
            CensusContact::EdgeEdgeCross { a, b, .. } => Self::EdgeCross(a, b),
            CensusContact::EdgeFacePierce { .. }
            | CensusContact::ConformalPatch { .. }
            | CensusContact::FaceFaceTangent { .. } => {
                return None;
            }
        })
    }

    /// The two entities the site pairs, one per solid.
    fn entities(self) -> (EntityId, EntityId) {
        match self {
            Self::VertexVertex(a, b) => (EntityId::Vertex(a), EntityId::Vertex(b)),
            Self::VertexOnEdge(v, e) => (EntityId::Vertex(v), EntityId::Edge(e)),
            Self::EdgeEdge(a, b) => (EntityId::Edge(a), EntityId::Edge(b)),
            Self::VertexOnFace(v, f) => (EntityId::Vertex(v), EntityId::Face(f)),
            Self::EdgeInFace(e, f) => (EntityId::Edge(e), EntityId::Face(f)),
            Self::EdgeCross(a, b) => (EntityId::Edge(a), EntityId::Edge(b)),
        }
    }

    /// The site's touch point, where it names a vertex: its keys
    /// ([`touch_keys`]), the vertex's position, and the two solids in
    /// the site's order. `None` for a site at no vertex; `Err` where the
    /// topology does not read.
    #[allow(clippy::type_complexity)] // the point's keys, where it is, and whose links
    fn point<T: Real>(
        self,
        body: &Body<T>,
        geo: &Geo<T>,
        ties: &Ties,
    ) -> Result<
        Option<(
            BTreeSet<PointKey>,
            Point3<T>,
            (crate::SolidKey, crate::SolidKey),
        )>,
        TouchVerdict,
    > {
        let seed = match self {
            Self::VertexVertex(a, b) => vec![a, b],
            Self::VertexOnEdge(v, _) | Self::VertexOnFace(v, _) => vec![v],
            Self::EdgeEdge(..) | Self::EdgeInFace(..) | Self::EdgeCross(..) => return Ok(None),
        };
        let keys = touch_keys(geo, ties, &seed).ok_or(TouchVerdict::Corrupt)?;
        let at = geo
            .vmap
            .get(&seed[0])
            .copied()
            .ok_or(TouchVerdict::Corrupt)?;
        let (x, y) = self.entities();
        let solid = |id| census_entity_solid(body, id).ok_or(TouchVerdict::Corrupt);
        Ok(Some((keys, at, (solid(x)?, solid(y)?))))
    }

    /// The unit the site is read in ([`TouchUnit`]).
    fn unit<T: Real>(self, body: &Body<T>, geo: &Geo<T>, ties: &Ties) -> TouchUnit {
        match self.point(body, geo, ties) {
            Ok(Some((keys, _, (sa, sb)))) => match keys.first() {
                Some(&k) => TouchUnit::Point(sa.min(sb), sa.max(sb), k),
                None => TouchUnit::Site(self),
            },
            Ok(None) | Err(_) => TouchUnit::Site(self),
        }
    }

    /// **The analysis's verdict on the site's unit.** At a vertex, the
    /// two solids' whole links at the touch point ([`Link::at_point`]),
    /// so every site at the point reads the same verdict. Elsewhere the
    /// site's own: an edge's two faces or a face at the point it meets
    /// at — a rest only where every point it meets at reads one, and
    /// otherwise a decided [`TouchVerdict::Crossing`] at any point over
    /// a refusal at another, and failing that the first point's
    /// refusal. An edge in a face meets at every cell of its overlap:
    /// the cells are separate pieces of the face, so no one of them
    /// answers for another.
    fn verdict<T: Decide>(
        self,
        body: &Body<T>,
        geo: &Geo<T>,
        ties: &Ties,
        band: Band,
    ) -> TouchVerdict {
        let pairs = match self.links(body, geo, ties, band) {
            Ok(pairs) => pairs,
            Err(v) => return v,
        };
        let mut all: Vec<TouchVerdict> = pairs
            .into_iter()
            .map(|pair| match pair {
                (Ok(a), Ok(b)) => rest::judge(&a, &b, band),
                (Err(v), _) | (_, Err(v)) => v,
            })
            .collect();
        if all.is_empty() {
            return TouchVerdict::InBand;
        }
        let pick = all
            .iter()
            .position(|v| matches!(v, TouchVerdict::Crossing))
            .or_else(|| all.iter().position(|v| !matches!(v, TouchVerdict::Rest(_))))
            .unwrap_or(0);
        all.swap_remove(pick)
    }

    /// The two links the site compares at each point it reads, each as
    /// built or as the verdict its build refused with; `Err` where no
    /// touch point reads.
    fn links<T: Decide>(
        self,
        body: &Body<T>,
        geo: &Geo<T>,
        ties: &Ties,
        band: Band,
    ) -> Result<Vec<LinkPair<T>>, TouchVerdict> {
        if let Some((keys, at, (sa, sb))) = self.point(body, geo, ties)? {
            let side = |s| Link::at_point(body, geo, ties, s, &keys, at, band);
            return Ok(vec![(side(sa), side(sb))]);
        }
        let edge = |e: EdgeKey| geo.line_edge(e).ok_or(TouchVerdict::Unreadable);
        let on_edge = |e: EdgeKey, p: Point3<T>| Link::on_edge(body, geo, e, p, band);
        let in_face = |f: FaceKey, p: Point3<T>| Link::in_face(body, geo, f, p, band);
        Ok(match self {
            Self::EdgeEdge(a, b) => {
                let m = ee_overlap_midpoint(edge(a)?, edge(b)?);
                vec![(on_edge(a, m), on_edge(b, m))]
            }
            Self::EdgeInFace(e, f) => {
                // Every cell of the overlap the finding reports,
                // re-derived by the same walk. A refusal on the way is
                // this touch's own.
                let fg = planar_face(geo, f).ok_or(TouchVerdict::Unreadable)?;
                let mut refused = Vec::new();
                let cells = ef_overlap_cells(body, edge(e)?, fg, geo, band, &mut refused);
                if !refused.is_empty() {
                    return Err(TouchVerdict::InBand);
                }
                cells
                    .iter()
                    .map(|c| (on_edge(e, c.mid), in_face(f, c.mid)))
                    .collect()
            }
            Self::EdgeCross(a, b) => {
                let q = ee_cross_point(edge(a)?, edge(b)?);
                vec![(on_edge(a, q), on_edge(b, q))]
            }
            Self::VertexVertex(..) | Self::VertexOnEdge(..) | Self::VertexOnFace(..) => {
                return Err(TouchVerdict::Corrupt);
            }
        })
    }
}

/// A touch's refusal, read once per unit ([`TouchSite::unit`]) and kept
/// in `read`: every site of a unit shares its verdict.
fn read_touch<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    ties: &Ties,
    read: &core::cell::RefCell<std::collections::BTreeMap<TouchUnit, Option<Undecided>>>,
    site: TouchSite,
    band: Band,
) -> Option<Undecided> {
    let unit = site.unit(body, geo, ties);
    if let Some(&known) = read.borrow().get(&unit) {
        return known;
    }
    let got = site.verdict(body, geo, ties, band).refusal();
    read.borrow_mut().insert(unit, got);
    got
}

/// One touch point's two links, each as built or as the verdict its
/// build refused with.
type LinkPair<T> = (Result<Link<T>, TouchVerdict>, Result<Link<T>, TouchVerdict>);

/// Where a face holds the touch point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Hold {
    /// At its corner, this vertex.
    Corner(VertexKey),
    /// Inside its edge, the edge between these two vertices.
    Edge(VertexKey, VertexKey),
    /// Inside the face.
    Inside,
}

/// **One face's sector at the touch point `p`**: the face's piece there
/// ([`face_piece`]) as a fan of wedges about `p`, each a triangle
/// `(p, a, b)` of real points of the face under half a turn, in turn
/// counterclockwise about the outward normal, consecutive wedges on one
/// ray. The fan spans the face's whole sector at `p`.
struct Sector<T: Real> {
    surface: geom::Surface<T>,
    /// `Face::sense`, the bit [`geom_brep::classify_material_pairing_as`]
    /// takes.
    sense: bool,
    /// The outward unit normal.
    outward: Vec3<T>,
    /// The point the fan is built about: the link's point, or the
    /// corner of this face on one of its keys.
    apex: Point3<T>,
    wedges: Vec<Segment<T>>,
    /// The fan closes all the way round: `p` is inside the face.
    /// Otherwise its first ray and its last are the face's edges through
    /// `p`, the sector's bounds.
    closed: bool,
}

/// Where a point on a sector's plane lies against the sector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Place {
    /// Strictly inside a wedge, or on a ray two wedges share.
    Inside,
    Outside,
    /// On one of the sector's bounds.
    Bound,
}

/// **One solid's link at a touch point `p`**: every face of the solid
/// through `p`, as its sector there. Its material near `p` is the set of
/// directions an odd number of its sectors separate from outside
/// ([`Link::reads`]).
struct Link<T: Real> {
    p: Point3<T>,
    sectors: Vec<Sector<T>>,
    /// The farthest wedge point from `p`.
    reach: T,
    /// The radius of the ball about `p` the link's wedges fill: the
    /// least distance from `p` to a wedge's far side. Inside it each of
    /// the link's faces covers its whole sector; a face of the solid
    /// that reaches into the ball without being tied to the point is
    /// not in the link, and is the backstop's to answer for (its
    /// material probe and findings).
    ball: T,
    /// The farthest a sector's apex stands from `p`: zero but where a
    /// corner on another key of the point was decided one point with it.
    tie: T,
}

/// Where a direction from `p` lies against a link's material.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Material {
    In,
    Out,
    /// On the link's sector with this index.
    On(usize),
}

/// Two outward normals against each other, aligned or opposed, through
/// the one classifier of that question under the K row `name`: two
/// coplanar faces', one of each link ([`TOUCH_NORMAL`]). The magnitude
/// is about 1 wherever it is asked (the faces lie on one plane), and
/// only its sign is read.
/// `None` is every refusal — in band, decided zero, or poisoned — and
/// each reads as in band, ending as the touch's one in-band refusal
/// (`TouchInBand`), which carries no margin.
fn pairing<T: Decide>(
    name: &'static str,
    s: (&geom::Surface<T>, bool),
    t: (&geom::Surface<T>, bool),
    p: Point3<T>,
    reach: T,
    band: Band,
) -> Option<geom_brep::MaterialPairing> {
    let arm = geom_brep::folded_lever_arm(s.0, t.0, p, reach);
    geom_brep::classify_material_pairing_as(name, s.0, s.1, t.0, t.1, p, arm, band).ok()
}

/// A chord from the touch point to `q`, as its length: a number, or a
/// [`TouchVerdict::ChordScale`] refusal for an edge too long or too
/// short to measure. The one home of that question in the analysis.
fn chord<T: Real>(p: Point3<T>, q: Point3<T>) -> Result<T, TouchVerdict> {
    let v = q - p;
    let len = v.norm();
    if !geom_core::is_finite_length(len) || geom_core::is_underflowed_length(len, v.norm_witness())
    {
        return Err(TouchVerdict::ChordScale);
    }
    Ok(len)
}

/// A boundary edge of a face, as its two end points.
type Segment<T> = (Point3<T>, Point3<T>);

/// Where `p` sits on the face whose piece is built.
#[derive(Clone, Copy, Debug)]
enum PieceAt {
    /// At the face's corner: loop positions `(ring, k)` of `p`.
    Corner(usize, usize),
    /// Inside the face's edge from `ring[k]` to `ring[k + 1]`.
    Edge(usize, usize),
    /// Inside the face.
    Inside,
}

/// Where a direction from `p` lies, turning counterclockwise about the
/// face's outward normal from a reference ray: on it, strictly left of
/// it, straight back, or strictly right.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Turn {
    Ahead,
    Left,
    Back,
    Right,
}

/// **A face's piece at `p`: the part of the face visible from `p`**
/// (the design's S2′), as its vertices — face vertices and points where
/// a ray from `p` through a face vertex meets the face's boundary
/// beyond it, all real points of the face.
///
/// `rings` are the face's loops as the snapshot walks them: each with
/// the face's interior on its LEFT about the outward normal `m` (the
/// outer loop counterclockwise, each hole clockwise). The sector the
/// piece spans at a corner or an edge is read off that orientation.
///
/// `also` names the face's other corners at `p`, where the face meets
/// `p` again: their edges bound other sectors of the face and are not
/// read as blocking here, and the sector ends at the first of their
/// incoming rays it reaches.
///
/// Holes' edges block and holes' vertices cast windows. At a corner the
/// piece is bounded by `p`'s two edges and holds the corner's whole
/// sector; inside an edge it is bounded by that edge; inside the face
/// it holds a disc about `p`. It is built as a fan about `p`:
/// 1. the face's vertices are sorted by their turn about `p`;
/// 2. each gap between two neighbouring turns is checked EMPTY — no
///    vertex strictly inside it, on decided readings — so no edge ends
///    inside a gap. That is what the sort's comparisons cannot promise
///    alone: two readings within ε are not transitive, and a sort over
///    them can misplace a vertex; the check refuses where it did;
/// 3. in each gap, the edge a ray from `p` meets first is read on the
///    bisecting ray — no edge ends inside the gap, so the order of the
///    edges along every ray through it is the same — and cut where the
///    rays through the gap's two bounds meet it, each meeting checked
///    to lie on that edge.
///
/// Every choice is decided in metres, through [`metric`]. A choice in
/// band, or readings that contradict each other within ε, build no
/// piece ([`TouchVerdict::PieceInBand`]): the smallest piece of all,
/// which costs a refusal and never a rest. A turn read Zero is a
/// direction shared within ε, and an edge read as blocking within ε
/// blocks: both only make the piece smaller. An edge whose both ends lie
/// decidedly behind `p` along a probe ray never meets it, whatever else
/// is undecided about it.
fn face_piece<T: Decide>(
    p: Point3<T>,
    m: Vec3<T>,
    rings: &[Vec<Point3<T>>],
    at: PieceAt,
    also: &[(usize, usize)],
    band: Band,
) -> Result<Vec<Point3<T>>, TouchVerdict> {
    use core::cmp::Ordering::{Equal, Greater, Less};
    use metric::sign;
    // The fan is built about its own apex: no slack.
    let rd = Reading { band, slack: None };
    // Which side of the ray from `p` through the point `r` a point is
    // on ([`wedge_side`], read no further out than `r`).
    let side = |r: Point3<T>, q: Point3<T>| wedge_side(p, m, r, q, rd);
    let unit = |q: Point3<T>| (q - p).normalize();
    // Every face vertex but `p`, and every boundary edge that does not
    // hold `p`.
    let mut points = Vec::new();
    let mut edges: Vec<Segment<T>> = Vec::new();
    for (r, ring) in rings.iter().enumerate() {
        let n = ring.len();
        for k in 0..n {
            let corner = |(rr, kk): (usize, usize)| rr == r && (k == kk || (k + 1) % n == kk);
            let holds_p = match at {
                PieceAt::Corner(rr, kk) => corner((rr, kk)),
                PieceAt::Edge(rr, kk) => rr == r && k == kk,
                PieceAt::Inside => false,
            } || also.iter().any(|&c| corner(c));
            if !holds_p {
                edges.push((ring[k], ring[(k + 1) % n]));
            }
            let at_p = matches!(at, PieceAt::Corner(rr, kk) if rr == r && kk == k)
                || also.contains(&(r, k));
            if !at_p {
                points.push(ring[k]);
            }
        }
    }
    // The reference ray the turns count from, and the last turn the
    // piece reaches (`None`: all the way round).
    let (start, end) = match at {
        PieceAt::Corner(r, k) => {
            let n = rings[r].len();
            (rings[r][(k + 1) % n], Some(rings[r][(k + n - 1) % n]))
        }
        PieceAt::Edge(r, k) => {
            let n = rings[r].len();
            (rings[r][(k + 1) % n], Some(rings[r][k]))
        }
        PieceAt::Inside => (*points.first().ok_or(TouchVerdict::Corrupt)?, None),
    };
    let turn = |q: Point3<T>| -> Result<Turn, TouchVerdict> {
        Ok(match side(start, q) {
            Some(Sign::Positive) => Turn::Left,
            Some(Sign::Negative) => Turn::Right,
            Some(Sign::Zero) => match ahead(p, start, q, rd) {
                Some(Sign::Positive) => Turn::Ahead,
                Some(Sign::Negative) => Turn::Back,
                _ => return Err(TouchVerdict::PieceInBand),
            },
            None => return Err(TouchVerdict::PieceInBand),
        })
    };
    // The order of two points' turns about `p`.
    let order =
        |a: (Turn, Point3<T>), b: (Turn, Point3<T>)| -> Result<core::cmp::Ordering, TouchVerdict> {
            if a.0 != b.0 {
                return Ok(a.0.cmp(&b.0));
            }
            if matches!(a.0, Turn::Ahead | Turn::Back) {
                return Ok(Equal);
            }
            Ok(match side(a.1, b.1) {
                Some(Sign::Positive) => Less,
                Some(Sign::Negative) => Greater,
                Some(Sign::Zero) => Equal,
                None => return Err(TouchVerdict::PieceInBand),
            })
        };
    let mut sorted: Vec<(Turn, Point3<T>)> = Vec::with_capacity(points.len());
    for &q in &points {
        let item = (turn(q)?, q);
        let mut slot = sorted.len();
        while slot > 0 && order(sorted[slot - 1], item)? == Greater {
            slot -= 1;
        }
        sorted.insert(slot, item);
    }
    // Where the face meets `p` more than once, a corner's sector runs
    // from its outgoing ray to the first incoming ray of ANY pass
    // counterclockwise from it: the face's interior lies left of the
    // outgoing edge until the loop next reaches `p`. (A loop through two
    // vertices on one point, a pinch, returns to `p` on another lobe's
    // edge, so its own pass's incoming edge is not where it ends.)
    let end = match (at, end) {
        (PieceAt::Corner(..), Some(own)) if !also.is_empty() => {
            let mut best = (turn(own)?, own);
            for &(rr, kk) in also {
                let n = rings[rr].len();
                let q = rings[rr][(kk + n - 1) % n];
                let item = (turn(q)?, q);
                if order(item, best)? == Less {
                    best = item;
                }
            }
            Some(best.1)
        }
        _ => end,
    };
    // Only the turns inside the piece's sector.
    if let Some(end) = end {
        let last = (turn(end)?, end);
        let mut kept = Vec::with_capacity(sorted.len());
        for item in sorted {
            if order(item, last)? != Greater {
                kept.push(item);
            }
        }
        sorted = kept;
    }
    // One direction per distinct turn.
    let mut rays: Vec<(Turn, Point3<T>)> = Vec::new();
    for item in sorted {
        match rays.last() {
            Some(&last) if order(last, item)? == Equal => {}
            _ => rays.push(item),
        }
    }
    let mut gaps: Vec<(Point3<T>, Point3<T>)> = rays.windows(2).map(|w| (w[0].1, w[1].1)).collect();
    if end.is_none() {
        match (rays.first(), rays.last()) {
            (Some(&first), Some(&last)) if rays.len() > 1 => gaps.push((last.1, first.1)),
            _ => return Err(TouchVerdict::Unanalysed),
        }
    }
    if gaps.is_empty() {
        return Err(TouchVerdict::Unanalysed);
    }
    let mut piece = Vec::with_capacity(2 * gaps.len());
    for (qa, qb) in gaps {
        // Under half a turn apart, so the bisector lies between them.
        match side(qa, qb) {
            Some(Sign::Positive) => {}
            Some(_) => return Err(TouchVerdict::Unanalysed),
            None => return Err(TouchVerdict::PieceInBand),
        }
        // Empty: no vertex strictly left of `ua` and right of `ub`.
        for &q in &points {
            match (side(qa, q), side(qb, q)) {
                (Some(Sign::Negative | Sign::Zero), _) | (_, Some(Sign::Positive | Sign::Zero)) => {
                }
                _ => return Err(TouchVerdict::PieceInBand),
            }
        }
        // The bisecting ray, as a point at the nearer bound's reach along
        // it: the scale its two bounds fix it at.
        let probe = (unit(qa) + unit(qb)).normalize();
        let pr = p + probe * ((qa - p).norm().min((qb - p).norm()) * unit(qa).dot(probe));
        let front = |q: Point3<T>| ahead(p, pr, q, rd);
        // The edge the probe meets first, with the point it meets it at.
        let mut first: Option<(Segment<T>, Point3<T>)> = None;
        for &(x, y) in &edges {
            let (sx, sy) = (side(pr, x), side(pr, y));
            if matches!(
                (sx, sy),
                (Some(Sign::Positive), Some(Sign::Positive))
                    | (Some(Sign::Negative), Some(Sign::Negative))
            ) {
                continue;
            }
            // Wholly behind `p` along the probe: it never meets the ray.
            if front(x) == Some(Sign::Negative) && front(y) == Some(Sign::Negative) {
                continue;
            }
            let cross = match (sx, sy) {
                (None, _) | (_, None) | (Some(Sign::Zero), Some(Sign::Zero)) => {
                    return Err(TouchVerdict::PieceInBand);
                }
                (Some(Sign::Zero), _) => x,
                (_, Some(Sign::Zero)) => y,
                _ => ray_meets(p, m, pr, (x, y)),
            };
            match front(cross) {
                Some(Sign::Positive) => {}
                Some(Sign::Negative) => continue,
                _ => return Err(TouchVerdict::PieceInBand),
            }
            first = match first {
                None => Some(((x, y), cross)),
                Some((e, c)) => {
                    match sign(TOUCH_PIECE_REACH, Distance::of(cross, c, probe), band) {
                        Some(Sign::Negative) => Some(((x, y), cross)),
                        Some(_) => Some((e, c)),
                        None => return Err(TouchVerdict::PieceInBand),
                    }
                }
            };
        }
        let ((x, y), _) = first.ok_or(TouchVerdict::Unanalysed)?;
        // Where the ray through `u` meets that edge: on it, each end
        // read along the edge, or the piece does not build.
        let along = (y - x).normalize();
        let meet = |r: Point3<T>| -> Result<Point3<T>, TouchVerdict> {
            let q = ray_meets(p, m, r, (x, y));
            let past_x = sign(TOUCH_PIECE_MEET, Distance::of(q, x, along), band);
            let past_y = sign(TOUCH_PIECE_MEET, Distance::of(q, y, -along), band);
            match (past_x, past_y) {
                (Some(Sign::Positive | Sign::Zero), Some(Sign::Positive | Sign::Zero)) => Ok(q),
                _ => Err(TouchVerdict::PieceInBand),
            }
        };
        piece.push(meet(qa)?);
        piece.push(meet(qb)?);
    }
    Ok(piece)
}

/// Which side of the ray from `p` through `r` the point `x` lies on, in
/// the plane of the face whose outward normal is `m`: Positive to the
/// ray's left about `m`. A ray is known by its real point `r`, so `x` is
/// read no further out than `r` ([`within`]): past it the ray would be
/// carried beyond what fixes it.
fn wedge_side<T: Decide>(
    p: Point3<T>,
    m: Vec3<T>,
    r: Point3<T>,
    x: Point3<T>,
    rd: Reading<T>,
) -> Option<Sign> {
    let x = within(p, x, (r - p).norm());
    metric::sign_over(
        TOUCH_WEDGE,
        Distance::of(x, p, m.cross((r - p).normalize())),
        rd,
    )
}

/// Whether `x` lies ahead of `p` along the ray through `r` or behind it,
/// read no further out than `r` (as [`wedge_side`]).
fn ahead<T: Decide>(p: Point3<T>, r: Point3<T>, x: Point3<T>, rd: Reading<T>) -> Option<Sign> {
    let x = within(p, x, (r - p).norm());
    metric::sign_over(TOUCH_AHEAD, Distance::of(x, p, (r - p).normalize()), rd)
}

/// `q` drawn in toward `p` along its ray to lie within `r` of it — on a
/// wedge's ray or inside it, a point of the same face.
fn within<T: Real>(p: Point3<T>, q: Point3<T>, r: T) -> Point3<T> {
    p + (q - p) * (r / (q - p).norm()).min(T::from_f64(1.0))
}

/// The point of the segment `x`–`y` where a quantity affine along it,
/// `ox` at `x` and `oy` at `y`, vanishes — a construction, read only
/// where the quantity is decided to change sign between them, or to
/// bound where it could.
fn on_plane<T: Real>(x: Point3<T>, y: Point3<T>, ox: T, oy: T) -> Point3<T> {
    x + (y - x) * (ox / (ox - oy))
}

/// Where the line from `p` along the ray through `r`, in the face of
/// outward normal `m`, meets the segment `a`–`b` of that face.
fn ray_meets<T: Real>(p: Point3<T>, m: Vec3<T>, r: Point3<T>, (a, b): Segment<T>) -> Point3<T> {
    let n = m.cross((r - p).normalize());
    on_plane(a, b, n.dot(a - p), n.dot(b - p))
}

impl<T: Decide> Sector<T> {
    /// One planar, line-bounded face of the snapshot as its sectors at
    /// the touch point, held there as `holds` say: inside it or inside
    /// one edge of it (one sector, about `p`), or at its corners — every
    /// place its loops pass through a vertex on the point, each its own
    /// sector about that vertex, whether the loops pass one vertex twice
    /// or two vertices on one point (a pinch, one vertex per cone). A
    /// face held inside two of its edges, or both at a corner and inside
    /// an edge, touches itself there in a way no sector reads, and is not
    /// read. A corner tie that is a face tie too reads the corner, and an
    /// edge tie that is a face tie too reads the edge: the face's interior
    /// holds no point of its boundary.
    fn of(
        body: &Body<T>,
        geo: &Geo<T>,
        f: FaceKey,
        p: Point3<T>,
        holds: &[Hold],
        band: Band,
    ) -> Result<Vec<Self>, TouchVerdict> {
        let fg = planar_face(geo, f).ok_or(TouchVerdict::Unreadable)?;
        if !fg.polygon {
            return Err(TouchVerdict::Unreadable);
        }
        let face = body.get_face(f).ok_or(TouchVerdict::Corrupt)?;
        let surface = body
            .get_surface(face.surface)
            .cloned()
            .ok_or(TouchVerdict::Corrupt)?;
        let outward = crate::face_normal::face_outward_normal(body, f)
            .ok_or(TouchVerdict::Unreadable)?
            .vec();
        let rings = fg
            .loops
            .iter()
            .map(|ring| {
                ring.iter()
                    .map(|v| geo.vmap.get(v).copied().ok_or(TouchVerdict::Corrupt))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        let corner = |v: VertexKey| holds.contains(&Hold::Corner(v));
        // Every chord from `p` a number (the point's own corners aside).
        for (&v, &q) in fg.loops.iter().flatten().zip(rings.iter().flatten()) {
            if !corner(v) {
                chord(p, q)?;
            }
        }
        // Where the loops hold the point: each a loop position, with the
        // point it is read about.
        let mut corners = Vec::new();
        let mut edges = Vec::new();
        for (r, ring) in fg.loops.iter().enumerate() {
            let n = ring.len();
            for k in 0..n {
                if corner(ring[k]) {
                    corners.push(((r, k), rings[r][k]));
                }
                for &h in holds {
                    if let Hold::Edge(a, b) = h
                        && ((ring[k] == a && ring[(k + 1) % n] == b)
                            || (ring[k] == b && ring[(k + 1) % n] == a))
                    {
                        edges.push((r, k));
                    }
                }
            }
        }
        let inside = holds.contains(&Hold::Inside);
        let sector = |at: Point3<T>, place: PieceAt, also: &[(usize, usize)]| {
            let piece = face_piece(at, outward, &rings, place, also, band)?;
            Ok(Self {
                surface: surface.clone(),
                sense: face.sense,
                outward,
                apex: at,
                wedges: piece.chunks_exact(2).map(|w| (w[0], w[1])).collect(),
                closed: matches!(place, PieceAt::Inside),
            })
        };
        let named_corners = holds
            .iter()
            .filter(|h| matches!(h, Hold::Corner(_)))
            .count();
        match (&corners[..], &edges[..], inside) {
            // A corner tie that is a face tie too names the touch at the
            // corner: the face's interior holds no point of its boundary.
            ([_, ..], [], _) => corners
                .iter()
                .map(|&(at, q)| {
                    let also: Vec<_> = corners.iter().map(|c| c.0).filter(|&c| c != at).collect();
                    sector(q, PieceAt::Corner(at.0, at.1), &also)
                })
                .collect(),
            ([], &[(r, k)], _) => Ok(vec![sector(p, PieceAt::Edge(r, k), &[])?]),
            ([], [], true) if named_corners == 0 => Ok(vec![sector(p, PieceAt::Inside, &[])?]),
            ([], [], _) => Err(TouchVerdict::Corrupt),
            _ => Err(TouchVerdict::Unanalysed),
        }
    }

    /// The sector's bounds, where it has them: real points on its first
    /// ray and its last.
    fn bounds(&self) -> Option<[Point3<T>; 2]> {
        match (self.closed, self.wedges.first(), self.wedges.last()) {
            (false, Some(first), Some(last)) => Some([first.0, last.1]),
            _ => None,
        }
    }

    /// **Where `x`, on the sector's plane, lies against the sector**, in
    /// its plane: each wedge `(p, a, b)` read by `x`'s side of its two
    /// rays (a wedge is under half a turn, so strictly left of `a` and
    /// right of `b` is strictly inside). `None` where a reading the
    /// answer needs is in band.
    fn place(&self, p: Point3<T>, x: Point3<T>, rd: Reading<T>) -> Option<Place> {
        let m = self.outward;
        let last = self.wedges.len().saturating_sub(1);
        let (mut in_band, mut bound) = (false, false);
        for (k, &(a, b)) in self.wedges.iter().enumerate() {
            // On a ray: a bound of the sector, or a ray two wedges share.
            let on = |outer: bool, ahead: Option<Sign>| match ahead {
                Some(Sign::Positive) if outer => Some(Place::Bound),
                Some(Sign::Positive) => Some(Place::Inside),
                _ => None,
            };
            let got = match (wedge_side(p, m, a, x, rd), wedge_side(p, m, b, x, rd)) {
                (Some(Sign::Negative), _) | (_, Some(Sign::Positive)) => continue,
                (Some(Sign::Positive), Some(Sign::Negative)) => Some(Place::Inside),
                (Some(Sign::Zero), Some(Sign::Negative)) => {
                    on(k == 0 && !self.closed, ahead(p, a, x, rd))
                }
                (Some(Sign::Positive), Some(Sign::Zero)) => {
                    on(k == last && !self.closed, ahead(p, b, x, rd))
                }
                _ => None,
            };
            match got {
                Some(Place::Inside) => return Some(Place::Inside),
                Some(Place::Bound) => bound = true,
                Some(Place::Outside) => {}
                None => in_band = true,
            }
        }
        match (in_band, bound) {
            (true, _) => None,
            (false, true) => Some(Place::Bound),
            (false, false) => Some(Place::Outside),
        }
    }
}

impl<T: Decide> Sector<T> {
    /// Whether the segment `x0`–`x1` (seen from `p`, an arc) decidedly
    /// misses the sector: for every wedge both ends lie strictly beyond
    /// one of its rays, a half-space through `p` that then holds the
    /// whole arc.
    fn misses(&self, p: Point3<T>, (x0, x1): Segment<T>, rd: Reading<T>) -> bool {
        let m = self.outward;
        let beyond = |r: Point3<T>, sign: Sign| {
            [x0, x1]
                .iter()
                .all(|&x| wedge_side(p, m, r, x, rd) == Some(sign))
        };
        self.wedges
            .iter()
            .all(|&(a, b)| beyond(a, Sign::Negative) || beyond(b, Sign::Positive))
    }
}

impl<T: Decide> Link<T> {
    /// A link from its sectors.
    fn of(p: Point3<T>, sectors: Vec<Sector<T>>) -> Result<Self, TouchVerdict> {
        if sectors.is_empty() {
            return Err(TouchVerdict::Corrupt);
        }
        let wedges = || sectors.iter().flat_map(|s| &s.wedges);
        let reach = wedges()
            .flat_map(|&(a, b)| [a, b])
            .map(|q| (q - p).norm())
            .fold(T::zero(), |r, l| r.max(l));
        // The nearest point of each far side: its foot from `p`, held to
        // the segment.
        let ball = wedges()
            .map(|&(a, b)| {
                let ab = b - a;
                let t = ((p - a).dot(ab) / ab.dot(ab))
                    .max(T::zero())
                    .min(T::from_f64(1.0));
                (a + ab * t - p).norm()
            })
            .fold(reach, |r, l| r.min(l));
        let tie = sectors
            .iter()
            .map(|s| (s.apex - p).norm())
            .fold(T::zero(), |r, l| r.max(l));
        Ok(Self {
            p,
            sectors,
            reach,
            ball,
            tie,
        })
    }

    /// **Solid `solid`'s link at the touch point of `keys`**, gathered
    /// structurally: every face of the solid holding one of its vertices
    /// on those keys, at that corner; each edge of the solid a tie names
    /// at the point, its two faces inside it; each face of the solid a
    /// tie names at the point, inside it. How a face held more than one
    /// way is read is [`Sector::of`]'s.
    #[allow(clippy::too_many_arguments)] // the snapshot, the ties, whose link, and where
    fn at_point(
        body: &Body<T>,
        geo: &Geo<T>,
        ties: &Ties,
        solid: crate::SolidKey,
        keys: &BTreeSet<PointKey>,
        p: Point3<T>,
        band: Band,
    ) -> Result<Self, TouchVerdict> {
        let on = |v: &VertexKey| geo.vpoint.get(v).is_some_and(|k| keys.contains(k));
        let mine = |id: EntityId| census_entity_solid(body, id) == Some(solid);
        let verts: BTreeSet<VertexKey> = geo
            .verts
            .iter()
            .map(|&(v, _)| v)
            .filter(|v| on(v) && mine(EntityId::Vertex(*v)))
            .collect();
        let mut holds: std::collections::BTreeMap<FaceKey, Vec<Hold>> =
            std::collections::BTreeMap::new();
        for &v in &verts {
            for &f in geo.vertex_faces.get(&v).ok_or(TouchVerdict::Corrupt)? {
                holds.entry(f).or_default().push(Hold::Corner(v));
            }
        }
        for (v, e) in &ties.ve {
            if !on(v) || !mine(EntityId::Edge(*e)) {
                continue;
            }
            let edge = geo.line_edge(*e).ok_or(TouchVerdict::Unreadable)?;
            if verts.contains(&edge.v0) || verts.contains(&edge.v1) {
                continue;
            }
            for f in [edge.f_plus, edge.f_minus] {
                holds
                    .entry(f)
                    .or_default()
                    .push(Hold::Edge(edge.v0, edge.v1));
            }
        }
        for (v, f) in &ties.vf {
            if on(v) && mine(EntityId::Face(*f)) {
                holds.entry(*f).or_default().push(Hold::Inside);
            }
        }
        let mut sectors = Vec::with_capacity(holds.len());
        for (f, all) in holds {
            let mut hs: Vec<Hold> = Vec::with_capacity(all.len());
            for h in all {
                if !hs.contains(&h) {
                    hs.push(h);
                }
            }
            sectors.extend(Sector::of(body, geo, f, p, &hs, band)?);
        }
        Self::of(p, sectors)
    }

    /// The link at a point `p` inside a line edge: the edge's two faces.
    fn on_edge(
        body: &Body<T>,
        geo: &Geo<T>,
        e: EdgeKey,
        p: Point3<T>,
        band: Band,
    ) -> Result<Self, TouchVerdict> {
        let edge = geo.line_edge(e).ok_or(TouchVerdict::Unreadable)?;
        let mut keys = vec![edge.f_plus];
        if edge.f_minus != edge.f_plus {
            keys.push(edge.f_minus);
        }
        let mut sectors = Vec::new();
        for &f in &keys {
            sectors.extend(Sector::of(
                body,
                geo,
                f,
                p,
                &[Hold::Edge(edge.v0, edge.v1)],
                band,
            )?);
        }
        Self::of(p, sectors)
    }

    /// The link at a point `p` inside a planar face: that face.
    fn in_face(
        body: &Body<T>,
        geo: &Geo<T>,
        f: FaceKey,
        p: Point3<T>,
        band: Band,
    ) -> Result<Self, TouchVerdict> {
        Self::of(p, Sector::of(body, geo, f, p, &[Hold::Inside], band)?)
    }

    /// `q`'s side of sector `s`'s plane through `p`.
    fn side(&self, s: &Sector<T>, q: Point3<T>, rd: Reading<T>) -> Option<Sign> {
        metric::sign_over(TOUCH_PLANE, Distance::of(q, self.p, s.outward), rd)
    }

    /// **Whether the link closes**: every sector bound is the bound of an
    /// even number of the link's sectors, counted with its own, so the
    /// sectors bound a region of directions and crossing-parity reads
    /// it ([`Link::reads`]). Gathering makes it so — every edge at a
    /// corner is two faces' at that corner, an edge tie brings both its
    /// faces, a face tie is a closed disc — and this holds that a body
    /// whose faces at the point do not pair up is refused, not read.
    /// `None` where a bound's reading is in band.
    fn closes(&self, rd: Reading<T>) -> Option<bool> {
        let p = self.p;
        let bounds: Vec<(Vec3<T>, Point3<T>)> = self
            .sectors
            .iter()
            .filter_map(|s| Some(s.bounds()?.map(|q| (s.outward, q))))
            .flatten()
            .collect();
        for &(m, q) in &bounds {
            let mut count = 0usize;
            for &(_, x) in &bounds {
                // On the ray: within zero of its plane and of its line in
                // that plane, and ahead. Any one reading decided otherwise
                // puts `x` off it, whatever the others read. All three
                // ask one thing about the ray, which its real point `q`
                // fixes, so `x` is drawn in to `q`'s reach for each, the
                // plane's too ([`within`]). [`Link::side`] reads a point
                // against a face's plane, a surface, at its own radius.
                let readings = [
                    metric::sign_over(
                        TOUCH_PLANE,
                        Distance::of(within(p, x, (q - p).norm()), p, m),
                        rd,
                    )
                    .map(|s| s == Sign::Zero),
                    wedge_side(p, m, q, x, rd).map(|s| s == Sign::Zero),
                    ahead(p, q, x, rd).map(|s| s == Sign::Positive),
                ];
                if readings.contains(&Some(false)) {
                    continue;
                }
                if readings.contains(&None) {
                    return None;
                }
                count += 1;
            }
            if count % 2 == 1 {
                return Some(false);
            }
        }
        Some(true)
    }

    /// The link's rays: every sector's bounds.
    fn rays(&self) -> Vec<Point3<T>> {
        self.sectors
            .iter()
            .filter_map(Sector::bounds)
            .flatten()
            .collect()
    }

    /// **Sector `s`'s pieces**: each wedge cut at every ray in `rays`
    /// lying strictly inside it, each piece as the two real points of the
    /// face its bounding rays meet the wedge's far side at. A ray on the
    /// plane is decided inside the wedge, on one of its rays, or outside;
    /// one in band of the plane, outside the wedge or the piece refuses.
    fn pieces(
        &self,
        s: &Sector<T>,
        rays: &[Point3<T>],
        rd: Reading<T>,
    ) -> Result<Vec<Segment<T>>, TouchVerdict> {
        use Sign::{Negative, Positive, Zero};
        let (p, m) = (self.p, s.outward);
        let mut out = Vec::new();
        for &(a, b) in &s.wedges {
            let mut cuts: Vec<Point3<T>> = Vec::new();
            for &r in rays {
                let sides = (wedge_side(p, m, a, r, rd), wedge_side(p, m, b, r, rd));
                let outside = matches!(sides, (Some(Negative), _) | (_, Some(Positive)));
                let inside = match self.side(s, r, rd) {
                    Some(Zero) => match sides {
                        (Some(Positive), Some(Negative)) => true,
                        (Some(Zero), Some(Negative)) | (Some(Positive), Some(Zero)) => false,
                        _ if outside => false,
                        _ => return Err(TouchVerdict::InBand),
                    },
                    Some(_) => false,
                    None if outside => false,
                    None => return Err(TouchVerdict::InBand),
                };
                if !inside {
                    continue;
                }
                let c = ray_meets(p, m, r, (a, b));
                // In turn from `a`: after every cut it is left of.
                let mut slot = cuts.len();
                while slot > 0 {
                    match wedge_side(p, m, cuts[slot - 1], c, rd) {
                        Some(Positive) => break,
                        Some(Negative) => slot -= 1,
                        Some(Zero) => {
                            slot = usize::MAX;
                            break;
                        }
                        None => return Err(TouchVerdict::InBand),
                    }
                }
                if slot != usize::MAX {
                    cuts.insert(slot, c);
                }
            }
            let mut ends = vec![a];
            ends.extend(cuts);
            ends.push(b);
            out.extend(ends.windows(2).map(|w| (w[0], w[1])));
        }
        Ok(out)
    }

    /// **Where the direction to `q` lies against the link's material**,
    /// by arc crossing-parity, no convexity assumed. `q` on a sector's
    /// plane inside it is [`Material::On`] that sector. Otherwise `q` is
    /// read along the minor arc to a reference `w`, a wedge's centroid on
    /// one sector `T` carried out to `q`'s distance (planes are read at
    /// one scale; rays at their own, [`wedge_side`]), where the link's
    /// material is the side behind `T`:
    /// the material holds `q` iff `q` lies behind `T` and the arc crosses
    /// the other sectors an even number of times, or in front and odd.
    /// The arc is the segment `q`–`w` seen from `p`, under half a turn
    /// (both ends ahead of the plane normal to their bisector), so it
    /// meets each plane through `p` at most once and `T`'s only at `w`.
    /// Each other sector is read from its plane's sides at `q` and `w`:
    /// on one side, not crossed; on opposite sides, crossed iff the
    /// segment's point on the plane lies inside the sector; one end on
    /// the plane (within `zero`), not crossed iff the stretch of the arc
    /// where it could then meet the plane misses the sector
    /// ([`Sector::misses`]). A reference
    /// any reading of which is in band, on a sector's bound, or on a
    /// second sector at `w` is passed over for the next; `None` where
    /// none reads, or where `q` itself is in band of a plane or on a
    /// bound.
    fn reads(&self, q: Point3<T>, rd: Reading<T>) -> Option<Material> {
        let p = self.p;
        // `q`'s strict side of each plane; `None` for a plane `q` lies
        // on outside its sector, which no arc from `q` crosses inside it.
        let mut sides = Vec::with_capacity(self.sectors.len());
        for (k, s) in self.sectors.iter().enumerate() {
            sides.push(match self.side(s, q, rd)? {
                Sign::Zero => match s.place(p, q, rd)? {
                    Place::Inside => return Some(Material::On(k)),
                    Place::Bound => return None,
                    Place::Outside => None,
                },
                sign => Some(sign),
            });
        }
        for (t, s0) in self.sectors.iter().enumerate() {
            let Some(base) = sides[t] else { continue };
            'reference: for &(a, b) in &s0.wedges {
                // The wedge's centroid, carried along its ray to `q`'s
                // distance: a plane is a surface, read at one scale for
                // both ends of the arc.
                let c = (a - p) + (b - p);
                let w = p + c * ((q - p).norm() / c.norm());
                let half = ((q - p).normalize() + (w - p).normalize()).normalize();
                for x in [q, w] {
                    if metric::sign_over(TOUCH_ARC, Distance::of(x, p, half), rd)
                        != Some(Sign::Positive)
                    {
                        continue 'reference;
                    }
                }
                let mut held = base == Sign::Negative;
                for (k, s) in self.sectors.iter().enumerate() {
                    if k == t {
                        continue;
                    }
                    let Some(at_w) = self.side(s, w, rd) else {
                        continue 'reference;
                    };
                    let (oq, ow) = (s.outward.dot(q - p), s.outward.dot(w - p));
                    // A Zero reads within the band's zero, the slack added.
                    let zero = T::from_computed(rd.band.zero()) + rd.slack();
                    let at = |t: T| q + (w - q) * t;
                    // An end read Zero lies within `zero` of the plane on
                    // either side, so a crossing, if any, lies on the
                    // stretch of the arc where the plane can then be met:
                    // read as not crossed only where that whole stretch
                    // misses the sector.
                    let crossed = match (sides[k], at_w) {
                        (None, Sign::Zero) => continue 'reference,
                        (None, sw) => {
                            let ow = if sw == Sign::Positive { ow } else { -ow };
                            if !s.misses(p, (q, at(zero / (zero + ow))), rd) {
                                continue 'reference;
                            }
                            false
                        }
                        (Some(sq), Sign::Zero) => {
                            let z = if sq == Sign::Positive { zero } else { -zero };
                            if !s.misses(p, (at(oq / (oq + z)), w), rd) {
                                continue 'reference;
                            }
                            false
                        }
                        (Some(at_q), at_w) if at_q == at_w => false,
                        (Some(_), _) => match s.place(p, on_plane(q, w, oq, ow), rd) {
                            Some(Place::Inside) => true,
                            Some(Place::Outside) => false,
                            _ => continue 'reference,
                        },
                    };
                    held ^= crossed;
                }
                return Some(if held { Material::In } else { Material::Out });
            }
        }
        None
    }
}

/// [`Link::reads`] over a bare polygon cone at `p`, each face its
/// outward normal and one wedge's two real points, read on `band`
/// alone: how the boolean's sector fuzz runs the census's reader on its
/// own cones beside `cone_side` (`boolean::sectors::cone_fuzz`).
#[cfg(test)]
pub(crate) fn bare_cone_reads(
    p: Point3<f64>,
    faces: &[(Vec3<f64>, Point3<f64>, Point3<f64>)],
    q: Point3<f64>,
    band: Band,
) -> Option<Material> {
    let sectors = faces
        .iter()
        .map(|&(m, a, b)| Sector {
            surface: geom::Surface::Plane {
                origin: p,
                normal: m,
                u_ref: (a - p).normalize(),
            },
            sense: true,
            outward: m,
            apex: p,
            wedges: vec![(a, b)],
            closed: false,
        })
        .collect();
    let rd = Reading { band, slack: None };
    Link::of(p, sectors).ok()?.reads(q, rd)
}

/// **The one door a rest comes through.** [`Rest`](rest::Rest) has a
/// private field and is neither `Copy` nor `Clone`, so nothing outside
/// this module can build one or replay one: a rest exists only where
/// [`judge`](rest::judge) ran the readings over two links and they
/// held, once.
mod rest {
    use super::{
        Band, Decide, Link, Material, Place, Point3, Reading, Segment, Sign, TouchVerdict,
    };
    use super::{TOUCH_NORMAL, ahead, on_plane, pairing, wedge_side, within};

    /// The certificate of a rest: every piece of both links read, on
    /// decided readings, as no crossing.
    #[derive(Debug, PartialEq, Eq)]
    pub(super) struct Rest {
        _sealed: (),
    }

    /// What the readings noted on the way: the reasons the test could
    /// not certify other than a decided crossing.
    #[derive(Default)]
    struct Notes {
        in_band: bool,
        degenerate: bool,
    }

    /// **The one test of two polygonal cones** at a touch point `p`,
    /// each a solid's link ([`Link`]). Every face sector of each link is
    /// cut into PIECES at every ray of either link lying on it
    /// ([`Link::pieces`]). Then:
    ///
    /// 1. **Crossing** if a piece of one link crosses a face of the other
    ///    along a ray inside both — its two ends decidedly on opposite
    ///    sides of the face's plane, the point between them on that plane
    ///    inside the face's sector and on no ray of either link lying on
    ///    the face ([`crosses`]) — or if a piece's
    ///    probe (its middle) lies on a face of the other link inside it
    ///    with the two outward normals aligned: coplanar sectors facing
    ///    the same way, overlapping in their plane.
    /// 2. Otherwise **Crossing** if any piece's probe reads inside the
    ///    other link's material ([`Link::reads`]).
    /// 3. Otherwise **Rest**.
    ///
    /// Both links are first held to close ([`Link::closes`]), and every
    /// reading is taken over the two links' tie ([`Reading`]).
    ///
    /// **Sound.** (1): along a ray inside a piece of `S` and inside `T`'s
    /// sector off every ray lying on `T`, each material is the half-space
    /// behind its face (no other face of either link reaches the ray: on
    /// `S`'s side the piece is cut at every ray, on `T`'s the point is off
    /// them), and two half-spaces through one line overlap in a wedge; aligned coplanar faces put both
    /// materials behind one plane. (2): the probe lies inside a piece,
    /// off every face of the other link, so the direction just off it
    /// on its own material side reads as the probe does, and lies in
    /// both materials.
    ///
    /// **Complete.** Let `U` be the directions inside both materials,
    /// open on the sphere about `p`. Were it not empty, its frontier
    /// holds an arc of one link's face, `S`, say, inside some piece `P`
    /// of `S` and off the other link's faces: a frontier point on both
    /// links' faces at once lies on two faces crossing (1) or two
    /// coplanar ones, aligned (1) or opposed (then the materials lie on
    /// opposite sides there, and the point is not on the frontier). `U`
    /// lies on `S`'s material side of that arc, inside the other
    /// material. Nothing of either link meets `P`'s interior — the
    /// pieces are cut at every ray lying on them, and (1) found no face
    /// crossing them — so the directions just off all of `P` on its
    /// material side lie on one side of one face, and in one cell of the
    /// two links' arrangement: `U`'s. `P`'s probe then reads inside the
    /// other material (2).
    ///
    /// **Each verdict at its own scale.** The readings run at the
    /// pieces' real points, each link's as far out as its faces reach.
    /// A rest needs every one decided there: a Zero read at a far point
    /// is a Zero at every nearer one, so that is where the fewest
    /// coincidences are taken. A crossing is confirmed inside the BALL
    /// where both links are their solids — out to the nearest far side
    /// of any wedge of either ([`Link::ball`]) — with the piece's points
    /// drawn in along their rays ([`within`]), still real points of its
    /// face: an overlap read there is one of the two bodies, not of one
    /// link's planes carried past the other's faces. A crossing read
    /// only further out refuses in band.
    ///
    /// A crossing is claimed on decided readings only; any other
    /// reading the test needed in band, or a sector whose pieces did not
    /// cut, leaves the touch [`TouchVerdict::InBand`] (or the cut's own
    /// refusal), and a probe lying on another face of its own link (a
    /// slit, or a face doubled) leaves it [`TouchVerdict::Degenerate`] —
    /// never a crossing and never a rest.
    pub(super) fn judge<T: Decide>(a: &Link<T>, b: &Link<T>, band: Band) -> TouchVerdict {
        // Every reading over the pair is taken about `p`, while each
        // sector stands on its own apex, off `p` by at most its link's
        // tie. A plane through the apex lies within the tie of the same
        // plane through `p`. A ray's line through the apex and the same
        // ray's line through `p` meet at the ray's real point `r`, and a
        // point read against them is drawn in within `|r − p|` of `p`
        // ([`within`]), so it lies at most twice that from `r`: the two
        // lines part there by at most twice the tie, and an ahead
        // reading, whose line turns with the ray, by as much. One link's
        // point read against the other's planes and rays is moved by
        // both, so twice their sum covers every reading.
        let rd = Reading {
            band,
            slack: Some((a.tie + b.tie) * T::from_f64(2.0)),
        };
        for link in [a, b] {
            match link.closes(rd) {
                Some(true) => {}
                Some(false) => return TouchVerdict::Unanalysed,
                None => return TouchVerdict::InBand,
            }
        }
        let mut notes = Notes::default();
        let mut refused = None;
        let ball = a.ball.min(b.ball);
        let rays: Vec<_> = a.rays().into_iter().chain(b.rays()).collect();
        let mut all: [Vec<(usize, Vec<Segment<T>>)>; 2] = [Vec::new(), Vec::new()];
        for (side, link) in [a, b].into_iter().enumerate() {
            for (k, s) in link.sectors.iter().enumerate() {
                match link.pieces(s, &rays, rd) {
                    Ok(pieces) => all[side].push((k, pieces)),
                    Err(v) => {
                        refused.get_or_insert(v);
                    }
                }
            }
        }
        for (side, (own, other)) in [(a, b), (b, a)].into_iter().enumerate() {
            let near = |q| within(own.p, q, ball);
            let cut: Vec<usize> = all[1 - side].iter().map(|(k, _)| *k).collect();
            for (k, pieces) in &all[side] {
                let s = &own.sectors[*k];
                for &(c0, c1) in pieces {
                    if crosses(other, &cut, &rays, (c0, c1), rd, &mut notes) {
                        let mut scratch = Notes::default();
                        if crosses(other, &cut, &rays, (near(c0), near(c1)), rd, &mut scratch) {
                            return TouchVerdict::Crossing;
                        }
                        notes.in_band = true;
                    }
                    let probe = c0 + (c1 - c0) * T::from_f64(0.5);
                    // On another face of its own link: a slit or a face
                    // doubled, no boundary the probe can stand on.
                    for (j, t) in own.sectors.iter().enumerate() {
                        if j == *k || own.side(t, probe, rd) != Some(Sign::Zero) {
                            continue;
                        }
                        match t.place(own.p, probe, rd) {
                            Some(Place::Outside) => {}
                            Some(Place::Inside | Place::Bound) => notes.degenerate = true,
                            None => notes.in_band = true,
                        }
                    }
                    // Every point strictly inside the piece reads its one
                    // cell, so any one that decides is the piece's
                    // reading. The far side is tried at its middle, then
                    // its quarter and sixteenth points from each end: a
                    // point stands in band of a plane only on a stretch
                    // of the side (the distance is affine along it), and
                    // these sit at five distinct places, nearer each end
                    // in turn, where a far end lifted off a plane reads
                    // most of its lift. The ladder chooses only how often
                    // a piece refuses, never what one that reads says.
                    let fractions = [
                        T::from_f64(0.5),
                        T::from_f64(0.25),
                        T::from_f64(0.75),
                        T::from_f64(0.0625),
                        T::from_f64(0.9375),
                    ];
                    let read = fractions.into_iter().find_map(|t| {
                        let q = c0 + (c1 - c0) * t;
                        other.reads(q, rd).map(|m| (q, m))
                    });
                    let full = read.map(|(_, m)| m);
                    let overlaps = match full {
                        Some(Material::In) => true,
                        Some(Material::Out) => false,
                        Some(Material::On(j)) => {
                            let t = &other.sectors[j];
                            match pairing(
                                TOUCH_NORMAL,
                                (&s.surface, s.sense),
                                (&t.surface, t.sense),
                                own.p,
                                own.reach.max(other.reach),
                                rd.band,
                            ) {
                                Some(geom_brep::MaterialPairing::Aligned) => true,
                                Some(geom_brep::MaterialPairing::Opposed) => false,
                                None => {
                                    notes.in_band = true;
                                    false
                                }
                            }
                        }
                        None => {
                            notes.in_band = true;
                            false
                        }
                    };
                    // Confirmed inside the ball: the same reading there.
                    if let (true, Some((q, _))) = (overlaps, read) {
                        if other.reads(near(q), rd) == full {
                            return TouchVerdict::Crossing;
                        }
                        notes.in_band = true;
                    }
                }
            }
        }
        if let Some(v) = refused {
            v
        } else if notes.degenerate {
            TouchVerdict::Degenerate
        } else if notes.in_band {
            TouchVerdict::InBand
        } else {
            TouchVerdict::Rest(Rest { _sealed: () })
        }
    }

    /// Whether the piece `c0`–`c1` crosses a face of `other` along a ray
    /// inside both: its ends decidedly on opposite sides of the face's
    /// plane, and the point between them on the plane inside the face's
    /// sector ([`Sector::place`]: strictly inside a wedge, or on a ray two
    /// of its wedges share) and on none of the `rays` of either link that
    /// lie on the face — a cut, where another face of one link reaches
    /// the ray and neither material is a half-space there. Only the
    /// sectors in `cut` are read (their pieces built). A reading in band
    /// notes it, unless the point is decidedly outside the face.
    fn crosses<T: Decide>(
        other: &Link<T>,
        cut: &[usize],
        rays: &[Point3<T>],
        (c0, c1): Segment<T>,
        rd: Reading<T>,
        notes: &mut Notes,
    ) -> bool {
        use Sign::{Negative, Positive, Zero};
        let p = other.p;
        for &k in cut {
            let t = &other.sectors[k];
            let sides = (other.side(t, c0, rd), other.side(t, c1, rd));
            let decided = match sides {
                (Some(Positive), Some(Negative)) | (Some(Negative), Some(Positive)) => true,
                (Some(_), Some(_)) => continue,
                _ => false,
            };
            let x = on_plane(c0, c1, t.outward.dot(c0 - p), t.outward.dot(c1 - p));
            let place = t.place(p, x, rd);
            if decided && place == Some(Place::Inside) {
                // On a cut: a ray lying on the face, the point on it.
                let mut on_cut = Some(false);
                for &r in rays {
                    let on = match other.side(t, r, rd) {
                        Some(Zero) => match wedge_side(p, t.outward, r, x, rd) {
                            Some(Zero) => ahead(p, r, x, rd).map(|a| a == Positive),
                            Some(_) => Some(false),
                            None => None,
                        },
                        Some(_) => Some(false),
                        None => None,
                    };
                    on_cut = match (on_cut, on) {
                        (Some(false), Some(false)) => Some(false),
                        (_, Some(true)) | (Some(true), _) => Some(true),
                        _ => None,
                    };
                }
                if on_cut == Some(false) {
                    return true;
                }
            }
            if place != Some(Place::Outside) {
                notes.in_band = true;
            }
        }
        false
    }
}

/// The role of `shell` the cross-solid gate reads: the one shell-role
/// reader ([`crate::props::shell_role`], a solid's only shell `Outer` by
/// arity) through the scalar's own lane, so the gate drops exactly the
/// shells check 10, the result sort and the shell classification read
/// as `Void`. `None` where it does not read.
pub(crate) fn gate_role<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    shell: crate::entity::ShellKey,
    band: Band,
    tol: Tol,
) -> Option<crate::props::ShellRole> {
    crate::props::shell_role(body, shell, band, tol, T::quad_lane()).ok()
}

/// A face's world reach ([`face_reach`]): its low and high corners.
type WorldReach<T> = (Point3<T>, Point3<T>);

/// Whether arm 1 of [`sweep_cross_solid_backstop`] may clear the face
/// pair `a`, `b` along a direction that turns with the pair: the axis
/// between the two faces' anchors, or either face's own normal where it
/// is planar. Each face comes with its world reach, the lever its
/// normal is read over.
///
/// The test is the boolean's narrow phase ([`crate::boolean::separating::apart`]):
/// both reaches are read in the frame the direction aims
/// ([`face_reach_in`]), and the pair clears only on a gap, less both
/// reaches' [`sweep_pad`], decided definitely positive at this lane's
/// scalar. Each reach encloses its face's locus along that direction,
/// so the gap proves the loci disjoint, as a world-axis gap does. The
/// pad covers the aimed frame's rounding ([`crate::boolean::boxes::BoxFrame`]),
/// so the clear does not rest on the user-set escalate threshold.
fn backstop_apart<T: Decide>(
    body: &Body<T>,
    (fa, ra): (FaceKey, WorldReach<T>),
    (fb, rb): (FaceKey, WorldReach<T>),
    band: Band,
) -> bool {
    use crate::boolean::separating::{Item, apart, planar_axis};
    let axes: Vec<_> = [(fa, ra), (fb, rb)]
        .into_iter()
        .filter_map(|(f, r)| planar_axis(body, f, || Some(r), band))
        .collect();
    apart(
        (body, Item::Face(fa)),
        (body, Item::Face(fb)),
        &axes,
        sweep_pad(band),
        band,
    )
}

/// **The conservative loudness backstop** (M9-2 union fix F1): the
/// census must DECIDE or REFUSE — it must never silently not-examine
/// (A5's letter). Two cross-solid candidate classes have no examining
/// arm yet, and both fire a typed [`ValidationError::CensusUndecidable`]
/// here instead of passing silently:
///
/// 1. **Proximity** (the C9-ring conformal-rest / partial-embedding
///    class): a pair of faces from DIFFERENT solids, not
///    vertex-adjacent, whose reach boxes cannot be definitely
///    separated, and which the exact sweeps cannot see — every pair
///    with a curved side (F5: curved × planar included, since a
///    revolved cap embedded in a plate's slab leaves no
///    vertex/line/planar evidence), PLUS planar × planar where either
///    side has a curved boundary (an arc rim is not in the snapshot,
///    so a cylinder's cap leaves nothing either). Distinct-key value-equal carriers in conformal rest
///    (the cradle witness), value-equal walls at gap zero
///    (boss-in-hole), and the embedded ball cap (the delta witness)
///    land here; the certified excluder this stands in for is the C9
///    exclusion ring. The reach boxes are SOUND per-kind supersets:
///    [`face_reach`] is [`crate::boolean::boxes::FaceBoxRule`] — the
///    ONE face-box rule — with this module's own arena walk over the
///    module's own per-kind extents, so no bound is derived twice. A
///    description with no claim in it (a placeholder patch) refuses
///    WITHOUT a distance test rather than under-claiming its reach. A
///    planar face vf-NAMED by a record whose vertex is on the OTHER
///    FACE OF THIS PAIR defers to the confirm pass (the declared
///    boss-on-plate class) — the record has to name both sides of the
///    pair it defers.
/// 2. **Instance containment**: one solid's vertex-extent box against
///    another's REACH box (the containing side must be a superset; the
///    contained side is a subset of its own locus, which is what makes
///    the box clear sound) — a nested placement makes no boundary
///    event at all (the reviewed nested-cube witness), so this arm is
///    the only one that sees it. **The box is the GATE and the MATERIAL
///    test is the verdict.** The gate answers containment, per OUTER
///    shell: the pair clears there only when every outer shell of each
///    separates from the other's reach and nothing on record says the
///    two boundaries meet. Otherwise the material test runs in BOTH
///    orderings
///    with no gate on either: every vertex of each instance is probed
///    against the other's material through the per-solid point-in-solid
///    door ([`crate::boolean::point_in_solid_faces`], at the run band, through that
///    door's own predicates — no comparand of this arm's), and:
///    - any vertex strictly `In` the other's material ⇒
///      [`ValidationError::InstanceInterference`] (`outer` holds the
///      witness, `inner` owns it), whatever else stands;
///    - a vertex the door refuses (escalation, ray exhaustion, a kind it
///      does not serve, …) and no `In` ⇒ [`ValidationError::CensusUndecidable`]
///      naming the cause; every vertex `OnBoundary` ⇒ the typed refusal;
///    - both orderings "every vertex `Out` or `OnBoundary`, at least one
///      `Out`" ⇒ the CLEAR, and only when every finding about the pair is a rest.
///
///    **Why the clear is sound, on what premises, and what it does not
///    cover** is stated once, at the loop below. In short: the probe
///    finds any outer shell lying inside the other's material, and
///    every finding about the pair must be a REST — the two solids'
///    whole links at the touch point read apart ([`rest::judge`], the
///    one analysis for every planar touch kind, [`TouchSite`]). What
///    the analysis cannot certify refuses typed: a sign in band, a face
///    meeting the point twice over, an escalation, and anything a curved
///    face takes part in — arm 1 refuses every cross-solid pair with a
///    curved side it cannot prove apart BEFORE this arm runs, and what
///    it clears (a box gap, a turned direction, a gap in depth) is a
///    proof that the two faces share no point. That is the PRECONDITION
///    of the clear for curved instances: with every curved pair proved
///    disjoint or refused, a clear's boundaries meet only where the
///    exact sweeps and the touch analysis read them, so each shell lies
///    wholly in or wholly out of the other's material, and one probed
///    vertex of it decides which.
///
///    **Over-width in the containing reach box costs a probe per
///    vertex** where it costs anything: a pair whose margins are not
///    definitely negative is sent to the material test instead of being
///    cleared for free. On planar-only pairs the reach box is the
///    vertex hull, so the case is not reached; a reach box a curved
///    face inflates reaches it once arm 1 has proved that face's pairs
///    apart (`bool4r1_probes::probe_d`). Each box rule claims exactly
///    its construction (`boxes`' ceiling rows pin that).
///
/// Arm 1 clears a pair ONLY on a definitely-positive separation margin
/// (`census_backstop_gap` — metre coordinate differences — then the
/// turned directions of [`backstop_apart`], then a gap in depth about
/// an axis, a centre or a circle, `census_backstop_depth_gap`, for the
/// nested walls no direction separates: [`depth_gap`]) and refuses
/// anything weaker; arm 2's box clears only on a definitely-negative
/// margin (`census_backstop_containment`) and hands anything weaker to
/// the material test.
///
/// A planar × planar pair is skipped only when BOTH faces are bounded
/// entirely by line edges. That — not the kind of solid the
/// faces sit on — is what the skip's premise is about: [`snapshot`]
/// admits line edges and drops every curved one, so a wholly
/// line-bounded planar face has its whole boundary, every vertex AND
/// every edge, in front of the exact sweeps. The premise is that
/// nothing such a pair can do to the other is invisible — NOT that
/// contact takes one of two shapes. It takes at least five, and each
/// has a lane: coincident boundary vertices in [`sweep_vertex_vertex`],
/// a vertex on the other boundary in [`sweep_vertex_edge`], collinear
/// boundary overlap in [`ee_collinear_lane`], a boundary lying in the
/// other face in [`sweep_vertex_face`] / [`ef_overlap_lane`], and a
/// pierce in [`sweep_edge_face`]. A future reader deciding whether
/// some OTHER face class may be skipped owes the same enumeration,
/// not an appeal to the two obvious cases.
///
/// A planar face with a curved boundary — a cylinder's cap — is
/// outside all of it: its rim is not in the snapshot, so no lane
/// holds it, and a cap can rest on another face with nothing to see
/// at vertex/line granularity. Such a pair is THIS arm's: the conformal
/// arm takes same-carrier CURVED faces only, and the confirm pass
/// takes declared pairs only, so the unrecorded cap pair has no other
/// home. Its plane is boxable ([`crate::boolean::boxes::FaceBoxRule`]
/// hulls the boundary's certified edge reaches, arcs included), so
/// this arm can clear the separated majority instead of refusing
/// every one of them.
///
/// **Jurisdiction, exactly** (a backstop refuses only what NO arm
/// examines): same-`SurfaceKey` CURVED face pairs are the conformal
/// arm's candidates and are skipped here — curved, because that arm
/// groups [`Geo::curved_faces`], so a same-key PLANAR pair is not on
/// its list and stays here; record-NAMED face pairs are the
/// patch/curve certifier's (a cross-key record escalates loudly
/// there — skipping it here loses no loudness). Every deferral above
/// names an arm that asks the SAME question about the SAME pair; that
/// is the bar a deferral has to clear, and it is what makes a skip
/// something other than a silent not-examine (A5's letter).
///
/// **Contact records do not license a deferral in arm 2, and this is
/// the one place the rule had to be derived rather than inherited.**
/// Each record states one coincidence of its two cells, whatever its
/// kind — two vertices at one point, a vertex on an edge's interior or
/// in a face's region, two edges crossing or overlapping, an edge
/// meeting a face, two faces meeting — and the confirm pass asks exactly
/// that of each: is THIS
/// coincidence geometrically real. Arm 2's question is different in
/// kind: where does one instance sit relative to another. No record
/// type in [`crate::boolean::ContactRecords`] states a placement
/// relation, so no set of them can answer it, and the confirm pass
/// never asks. A solid pair carrying records is therefore examined
/// here exactly like a pair carrying none.
///
/// The unsound direction is a deferral keyed on *whether* records
/// exist: a truthful declaration then switches the containment
/// examination off, and an instance embedded in another's material
/// validates clean — which it did until the deferral was removed. No
/// record admits an interference: between two copies at rest the
/// assembly layer reports it as a finding (ASSEMBLY A5
/// *Interference.*).
#[allow(clippy::too_many_arguments)] // the census's fixed sweep signature plus `tol` for one consumer
fn sweep_cross_solid_backstop<T: Decide + crate::props::AtRestPolicy + Bounds>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    tol: Tol,
    cands: &Candidates,
    mut trace: Option<&mut CensusTrace>,
    errors: &mut Vec<ValidationError>,
) {
    use crate::entity::{FaceKey as FK, SolidKey};
    if body.solids().count() < 2 {
        return; // single-solid bodies have no cross-solid pairs
    }
    let solid_of = |f: FK| -> Option<SolidKey> { body.solid_of_face(f) };
    // Every boundary vertex of `f` — outer loop and rings — exactly as
    // `snapshot` records them into `vertex_faces`; a loop this cannot
    // walk contributes none, and an empty set is disjoint from every
    // other, so the pair it belongs to is examined rather than skipped.
    let face_vertices = |f: FK| -> BTreeSet<VertexKey> {
        let mut out = BTreeSet::new();
        let Some(face) = body.get_face(f) else {
            return out;
        };
        for cycle in face_cycles(body, face).filter_map(Result::ok) {
            for he in cycle {
                if let Some(hd) = body.half_edges.get(he) {
                    out.insert(hd.start);
                }
            }
        }
        out
    };
    // Boundary-vertex hull of a face (every loop), as raw points.
    //
    // A loop this cannot walk contributes nothing, and an EMPTY loop is
    // one of them: a lone-vertex loop has no cycle, and a face whose
    // OUTER loop is empty is unbounded — there is no hull of it. This
    // closure does not decide what that means, because its two callers
    // want opposite things from it; each answers emptiness itself, and
    // both say so at the call site. What is never allowed is the third
    // reading, "empty means nothing to look at" — a `continue` on an
    // empty set, under exactly that reading, is the same defect one
    // deferral over, and `splitting/rules.rs` carried one.
    let face_points = |f: FK| -> Vec<Point3<T>> {
        let mut out = Vec::new();
        let Some(face) = body.get_face(f) else {
            return out;
        };
        for cycle in face_cycles(body, face).filter_map(Result::ok) {
            for he in cycle {
                if let Some(hd) = body.half_edges.get(he)
                    && let Some(v) = body.vertices.get(hd.start)
                    && let Some(p) = body.points.get(v.point)
                {
                    out.push(*p);
                }
            }
        }
        out
    };
    // Every boundary edge of `f` — outer loop and rings — passes
    // [`edge_is_line`], the exact sweeps' own admission test. That is
    // the property the planar × planar skip rests on: only a face
    // whose whole boundary is admitted is in front of those sweeps.
    // Anything unresolvable — a missing loop, an isolated-vertex
    // boundary, an uncertified carrier — is not a line, so the face
    // stays with this arm. `edge_is_line` carries the one way this
    // is weaker than snapshot's admission (unresolvable endpoints).
    let line_bounded = |f: FK| -> bool {
        let Some(face) = body.get_face(f) else {
            return false;
        };
        for cycle in face_cycles(body, face) {
            let Ok(cycle) = cycle else {
                return false;
            };
            for he in cycle {
                let Some(hd) = body.half_edges.get(he) else {
                    return false;
                };
                if !edge_is_line(body, hd.edge) {
                    return false;
                }
            }
        }
        true
    };
    let hull = |pts: &[Point3<T>]| point_hull(pts.iter().copied());
    let reach_box = |f: FK| face_reach(body, f, band);

    // Arm 1: cross-solid proximity — curved × curved, (F5) curved ×
    // planar, and the planar × planar pairs the exact sweeps cannot
    // see (a pair is theirs only when BOTH faces are wholly
    // line-bounded, which is what puts a whole boundary in the
    // snapshot — the module docs carry the argument).
    struct Reach<T: Real> {
        face: crate::entity::FaceKey,
        solid: crate::entity::SolidKey,
        /// The sound reach box (`reach_box` docs), or `None` for a
        /// kind with no cheap sound box — its pairs refuse without a
        /// distance test.
        boxed: Option<(Point3<T>, Point3<T>)>,
        verts: BTreeSet<VertexKey>,
        planar: bool,
        /// Planar AND bounded entirely by line edges, so the exact
        /// sweeps hold this face's whole boundary.
        line_bounded: bool,
    }
    let mut reaches: Vec<Reach<T>> = Vec::new();
    let planar_keys: Vec<FaceKey> = geo.faces.iter().map(|f| f.key).collect();
    for (&f, planar) in geo
        .curved_faces
        .iter()
        .map(|f| (f, false))
        .chain(planar_keys.iter().map(|f| (f, true)))
    {
        // A placeholder surface is "no description yet" (mid-surgery
        // scaffolding): there is no geometry to be within reach OF —
        // excluded from the backstop. What licenses the exclusion is a
        // named check, not an accident: the census's only production
        // caller is [`crate::validate::validate_pseudomanifold`], and
        // it runs `census_and_certify` ONLY when `tier3_local_checks`
        // came back empty, whose check 1 pushes `UncertifiableSurface`
        // for every placeholder face. So no body reaching this arm
        // through the public door carries one. The unsound direction is
        // a SECOND caller of `census_and_certify` that does not gate on
        // tier 3 — this skip would go quiet on faces the arm can say
        // nothing about. (The in-src rows below are exactly that
        // caller, deliberately: they run open euler scaffolds below
        // tier 3's bar, and their seed faces still carry the mvfs
        // placeholder. They are why this line is reachable at all.)
        if matches!(
            body.get_face(f).and_then(|d| body.surfaces.get(d.surface)),
            Some(geom::Surface::Nurbs(p)) if p.is_placeholder()
        ) {
            continue;
        }
        let Some(solid) = solid_of(f) else { continue };
        let pts = face_points(f);
        if pts.is_empty() {
            // No boundary vertex at all: an unbounded face (empty outer
            // loop) or one whose boundary does not resolve. Either way
            // this arm cannot bound it, and A5's letter is that it must
            // then REFUSE rather than drop the face out of the sweep —
            // which is what this early-out did, silently, in the
            // function whose header forbids exactly that.
            //
            // Two states reach it and only one is closed off ahead of
            // here: `validate_closed`'s tier-2 check 1 refuses every
            // empty loop, and `validate_pseudomanifold` runs it before
            // the census, so the unbounded-face half cannot arrive
            // through the public door. The other half — a boundary
            // that does not RESOLVE, any dangling loop, half-edge,
            // vertex or point reference — is not argued unreachable
            // here; it is refused on its merits. Both cost nothing and
            // stay loud if a second, ungated caller ever appears.
            errors.push(ValidationError::CensusUnsupported {
                subject: CensusSubject::Entity(EntityId::Face(f)),
                cause: CensusUnsupportedCause::FaceUnboundable,
            });
            continue;
        }
        // The face's own boundary vertices — the same walk that built
        // `geo.vertex_faces`, read forward from the face rather than by
        // scanning every vertex's face set for this one (that scan was
        // faces × vertices, the arena-scan shape of PERF-PLAN §2.1).
        let verts = face_vertices(f);
        let boxed = reach_box(f);
        reaches.push(Reach {
            face: f,
            solid,
            boxed,
            verts,
            planar,
            line_bounded: planar && line_bounded(f),
        });
    }
    // The vf-record deferral for the curved × planar arm (F5): a
    // planar face NAMED by a v-on-f record is the declared interface
    // the confirm pass and the exact sweeps examine (the boss-on-plate
    // acceptance class), so the face on the other side of THAT
    // interface defers to that verdict; a bogus record still errors
    // there as stale, so nothing blesses silently.
    //
    // The record's vertex must be a boundary vertex of the other face
    // of the pair, not merely a vertex of the other SOLID. The finding
    // being deferred is about the face pair, so the record that defers
    // it has to name both of its sides. A solid-granular test lets one
    // declared interface silence every other face of the same solid
    // against `f_planar` — including a face resting on it somewhere
    // else with no vertex evidence of its own, which is the class this
    // arm exists for. Widening this back to the solid is the UNSOUND
    // direction.
    //
    // The narrowing is a narrowing on every body that can reach here,
    // and that rests on a tier-1 fact rather than on the shape of the
    // test: `Reach::verts` is `{ v : vertex_faces[v] ∋ f }`, and the
    // old test read the solid of `vertex_faces[v]`'s smallest member,
    // so the new test could be true where the old was false ONLY for a
    // vertex incident to faces of two different solids — a vertex
    // orbit spanning two shells, which tier 1 pass 6 forbids and
    // `validate_pseudomanifold` checks (`validate_closed(body)?`)
    // before the census runs. So new ⊆ old here, strictly.
    let planar_face_bridged = |f_planar: FK, other: &BTreeSet<VertexKey>| -> bool {
        declared
            .vf
            .iter()
            .any(|&(v, vf)| vf == f_planar && other.contains(&v))
    };
    // The pre-filter over the reach boxes themselves (`Trees` docs): a
    // pair whose padded reach boxes are disjoint on an axis is one the
    // gap test below clears, so pruning it loses nothing; a face with
    // no sound reach takes the poison box and is never pruned.
    let pad = sweep_pad(band);
    let reach_boxes: Vec<Aabb> = reaches
        .iter()
        .map(|r| {
            r.boxed.map_or_else(Aabb::poison, |(lo, hi)| {
                Aabb::from_points([lo, hi]).map_or_else(Aabb::poison, |b| b.padded(pad))
            })
        })
        .collect();
    let reach = cands.class(reach_boxes);
    for (i, a) in reaches.iter().enumerate() {
        for j in reach.later(i) {
            let b = candidate(&reaches, j);
            if a.solid == b.solid || !a.verts.is_disjoint(&b.verts) {
                continue; // same instance / structural adjacency
            }
            if a.planar && b.planar && a.line_bounded && b.line_bounded {
                continue; // both boundaries in the snapshot — the exact sweeps' pair
            }
            let same_key = body
                .get_face(a.face)
                .zip(body.get_face(b.face))
                .is_some_and(|(da, db)| da.surface == db.surface);
            // The conformal arm groups CURVED faces by carrier, so a
            // same-key planar pair would be on no list at all; the
            // deferral may only name an arm that takes the pair.
            // The `!planar` conjunct is a TAUTOLOGY, deliberately: one
            // surface key is one surface kind, and `planar` is
            // derived from the kind, so `same_key` already implies
            // `a.planar == b.planar`. It is written so the deferral
            // cannot quietly start covering pairs the conformal arm
            // never walks — which is what it did until the skip above
            // was narrowed and a planar pair began reaching this line.
            //
            // **Opposed senses is not a tautology, and it is the half
            // that makes the deferral true.** The bar this arm's docs
            // set is that a deferral names an arm which asks the SAME
            // question about the SAME pair; "the conformal arm walks
            // it" is a weaker test, and the gap between them is real.
            // [`sweep_conformal_patches`] reaches a same-sense pair and
            // `continue`s on it — aligned coincidence is flush material
            // within one solid, not contact (C1) — so it WALKS such a
            // pair and DECIDES nothing about it. Deferring on carrier
            // identity alone therefore hands a cross-solid same-key
            // same-sense pair to an arm that will drop it, and neither
            // arm ever answers this one's question (are these two
            // within reach and unexamined). Requiring opposed senses
            // here keeps exactly the pairs that arm returns a verdict
            // on; the rest fall through to the box test below, where a
            // separated pair still clears.
            let same_key_conformal = same_key
                && !a.planar
                && !b.planar
                && body
                    .get_face(a.face)
                    .zip(body.get_face(b.face))
                    .is_some_and(|(da, db)| da.sense != db.sense);
            if same_key_conformal || declared.faces.contains(&(a.face, b.face)) {
                continue; // the conformal arm's pair, or a recorded face pair's
            }
            let vf_deferred = (a.planar && planar_face_bridged(a.face, &b.verts))
                || (b.planar && planar_face_bridged(b.face, &a.verts));
            if vf_deferred {
                continue; // the declared interface — confirm pass's pair
            }
            // A DESCRIPTION with no claim in it refuses without a
            // distance test (`face_reach` docs — the loud direction).
            // Every surface KIND has a reach bound; what has none is a
            // placeholder patch, or a face whose boundary carries an
            // unboxable curve.
            let before = errors.len();
            if let (Some((alo, ahi)), Some((blo, bhi))) = (a.boxed, b.boxed) {
                // Definite separation along ANY direction clears the
                // pair: the margin is the gap between the sound reaches
                // along it — a metre coordinate difference (audit row).
                // The world axes are read first; then the directions
                // that turn with the pair ([`backstop_apart`]).
                let mut cleared = false;
                for (alo, ahi, blo, bhi) in [
                    (alo.x, ahi.x, blo.x, bhi.x),
                    (alo.y, ahi.y, blo.y, bhi.y),
                    (alo.z, ahi.z, blo.z, bhi.z),
                ] {
                    let gap = (blo - ahi).max(alo - bhi);
                    if matches!(
                        decide("census_backstop_gap", Margin::of(gap), band),
                        Ok(Sign::Positive)
                    ) {
                        cleared = true;
                        break;
                    }
                }
                if !cleared
                    && !backstop_apart(body, (a.face, (alo, ahi)), (b.face, (blo, bhi)), band)
                    && !depth_gap::apart_in_depth(body, a.face, b.face, band)
                {
                    errors.push(ValidationError::CensusUndecidable {
                        a: EntityId::Face(a.face),
                        b: EntityId::Face(b.face),
                        what: Undecided::CurvedWithinReach.what(),
                    });
                }
            } else {
                errors.push(ValidationError::CensusUndecidable {
                    a: EntityId::Face(a.face),
                    b: EntityId::Face(b.face),
                    what: Undecided::NoSoundReach.what(),
                });
            }
            if let Some(t) = trace.as_deref_mut() {
                t.backstop.note(
                    (EntityId::Face(a.face), EntityId::Face(b.face)),
                    errors.len() > before,
                );
            }
        }
    }

    // Arm 2: instance containment (the contained side's vertex hull
    // against the containing side's reach box).
    /// A vertex hull: its low and high corners.
    type Hull<U> = (Point3<U>, Point3<U>);
    let join = |(l, h): Hull<T>, (lo, hi): Hull<T>| -> Hull<T> {
        (
            Point3::new(l.x.min(lo.x), l.y.min(lo.y), l.z.min(lo.z)),
            Point3::new(h.x.max(hi.x), h.y.max(hi.y), h.z.max(hi.z)),
        )
    };
    // Each SHELL's vertex hull, with its solid: a solid may have several
    // outer shells (a union of two disjoint lumps is one solid), and one
    // lump can sit inside another instance while its sibling lies far
    // away, so the gate reads shells, never only the solid.
    let mut shell_boxes: std::collections::BTreeMap<crate::entity::ShellKey, (SolidKey, Hull<T>)> =
        std::collections::BTreeMap::new();
    for (f, _) in body.faces.iter() {
        let Some(solid) = solid_of(f) else { continue };
        let Some(shell) = body.get_face(f).map(|d| d.shell) else {
            continue;
        };
        let pts = face_points(f);
        // The CONTAINED side's hull, and it may be any subset of the
        // solid's locus (this arm's own comment below). A face that
        // contributes no vertices shrinks the hull, which makes a
        // definite separation harder to claim — so the pair is sent to
        // the material test rather than cleared at the gate, the
        // conservative direction — and the vertices it would have
        // contributed are probed by the material test whether or not
        // the hull saw them. Skipping it here is therefore sound where
        // the same skip in the `reaches` build above is not. Arm 1 has
        // already refused the face itself.
        let Some(h) = hull(&pts) else { continue };
        shell_boxes
            .entry(shell)
            .and_modify(|(_, b)| *b = join(*b, h))
            .or_insert((solid, h));
    }
    // A solid's hull is its shells' hulls joined.
    let mut solid_boxes: std::collections::BTreeMap<SolidKey, Hull<T>> =
        std::collections::BTreeMap::new();
    for &(solid, h) in shell_boxes.values() {
        solid_boxes
            .entry(solid)
            .and_modify(|b| *b = join(*b, h))
            .or_insert(h);
    }
    // The shells the gate reads: every shell but a VOID ([`gate_role`]),
    // and a shell whose role does not read is kept — the conservative
    // direction, since keeping a shell only sends more pairs to the
    // probe. Why voids may be dropped is the loop's argument below.
    let gate_shells: Vec<(SolidKey, Hull<T>)> = shell_boxes
        .iter()
        .filter(|&(&shell, _)| {
            gate_role(body, shell, band, tol) != Some(crate::props::ShellRole::Void)
        })
        .map(|(_, &b)| b)
        .collect();
    // The CONTAINING side must be a superset, so it is built from the
    // one face-box rule, not from vertices: a cylinder solid's vertex
    // hull is the segment joining its two seam vertices, and using
    // that as a container would clear every body nested inside it. The
    // contained side stays the vertex hull, which is a SUBSET of the
    // solid's locus — that is what makes the clear sound, since a
    // witness point outside the container is genuinely outside. A
    // solid carrying a face with no cheap sound box has no claimable
    // extent at all and can never be the container.
    /// A solid's claimable extent, or `None` when one of its faces has
    /// no cheap sound box (so the solid can never be the container).
    type SolidReach<T> = Option<(Point3<T>, Point3<T>)>;
    let mut solid_reach: std::collections::BTreeMap<SolidKey, SolidReach<T>> =
        std::collections::BTreeMap::new();
    for (f, _) in body.faces.iter() {
        let Some(solid) = solid_of(f) else { continue };
        let this = reach_box(f);
        let slot = solid_reach.entry(solid).or_insert(this);
        *slot = match (*slot, this) {
            (Some((l, h)), Some((lo, hi))) => Some((
                Point3::new(l.x.min(lo.x), l.y.min(lo.y), l.z.min(lo.z)),
                Point3::new(h.x.max(hi.x), h.y.max(hi.y), h.z.max(hi.z)),
            )),
            _ => None,
        };
    }
    let solids: Vec<_> = solid_boxes.iter().collect();
    // The findings standing when arm 2 begins: every exact sweep's, the
    // conformal arm's and arm 1's. The clear's precondition reads this
    // prefix and nothing arm 2 itself pushes — a placement verdict on
    // one pair says nothing about another pair's boundaries.
    let standing = errors.len();
    let solid_of_entity = |id: EntityId| -> Option<SolidKey> {
        match id {
            EntityId::Solid(s) => Some(s),
            EntityId::Shell(s) => body.get_shell(s).map(|d| d.solid),
            EntityId::Face(f) => body.solid_of_face(f),
            EntityId::Loop(l) => body.get_loop(l).and_then(|d| body.solid_of_face(d.face)),
            EntityId::HalfEdge(h) => body
                .face_of_half_edge(h)
                .and_then(|f| body.solid_of_face(f)),
            EntityId::Edge(e) => body
                .edges
                .get(e)
                .and_then(|d| body.face_of_half_edge(d.he_plus))
                .and_then(|f| body.solid_of_face(f)),
            EntityId::Vertex(v) => geo
                .vertex_faces
                .get(&v)
                .and_then(|fs| fs.iter().next().copied())
                .and_then(|f| body.solid_of_face(f)),
        }
    };
    let vertex_solid = |v: VertexKey| solid_of_entity(EntityId::Vertex(v));
    // ---- The clear's condition 3: every touch between the pair is a
    // rest by the one local analysis ([`rest::judge`]).
    // One verdict per unit ([`TouchUnit`]): every finding and record at
    // a touch point reads the two solids' whole links there.
    let ties = Ties::of(declared, &errors[..standing]);
    let read = core::cell::RefCell::new(std::collections::BTreeMap::new());
    let touch = |site: TouchSite| read_touch(body, geo, &ties, &read, site, band);
    // The vertices of each point key more than one vertex sits on.
    let mut on_key: std::collections::BTreeMap<PointKey, Vec<VertexKey>> =
        std::collections::BTreeMap::new();
    for (&v, &k) in &geo.vpoint {
        on_key.entry(k).or_default().push(v);
    }
    on_key.retain(|_, vs| vs.len() > 1);
    // A vertex pair of the two solids on one key at each point they
    // share structurally, whether or not a finding or record names it.
    let shared = |sa: SolidKey, sb: SolidKey| -> Vec<TouchSite> {
        on_key
            .values()
            .filter_map(|vs| {
                let of = |s: SolidKey| vs.iter().copied().find(|&v| vertex_solid(v) == Some(s));
                Some(TouchSite::VertexVertex(of(sa)?, of(sb)?))
            })
            .collect()
    };
    // One entity on each side of the pair `(sa, sb)`.
    let straddles = |sa: SolidKey, sb: SolidKey, x: EntityId, y: EntityId| {
        matches!(
            (solid_of_entity(x), solid_of_entity(y)),
            (Some(p), Some(q)) if (p == sa && q == sb) || (p == sb && q == sa)
        )
    };
    // ---- Whether the two boundaries MEET on record: a standing
    // finding naming one entity of each solid, or a contact record
    // naming one of each (a recorded event leaves no finding, so its
    // record is the only trace of it).
    let meets_found = |standing_errors: &[ValidationError], sa: SolidKey, sb: SolidKey| -> bool {
        standing_errors.iter().any(|e| match Named::of(e) {
            Named::Two(x, y, _) => straddles(sa, sb, x, y),
            Named::One(_) | Named::Nothing => false,
        })
    };
    // The contact records naming one entity of each solid, as the one
    // mapping every reader of them shares: a v-on-f or v-v record is a
    // touch site the analysis reads; a curve or patch face pair backs
    // events without a side.
    let recorded = |sa: SolidKey, sb: SolidKey| -> Vec<Recorded> {
        let sites = declared
            .vf
            .iter()
            .map(|&(v, f)| TouchSite::VertexOnFace(v, f))
            .chain(
                declared
                    .vv
                    .iter()
                    .filter(|(a, b)| a < b)
                    .map(|&(a, b)| TouchSite::VertexVertex(a, b)),
            )
            .filter(|site| {
                let (x, y) = site.entities();
                straddles(sa, sb, x, y)
            })
            .map(Recorded::Site);
        let pairs = declared
            .faces
            .iter()
            .filter(|&&(a, b)| a < b && straddles(sa, sb, EntityId::Face(a), EntityId::Face(b)))
            .map(|_| Recorded::FacePair);
        sites.chain(pairs).collect()
    };
    // ---- The clear's condition: every finding about the pair is a
    // rest. A finding is ABOUT the pair when it names one entity of
    // each solid; one naming a third solid is that other pair's, and
    // is read there. Three kinds block beyond that, each because the
    // clear's argument (at the loop) needs every meeting of the pair
    // on record: an escalation names no entity, so the event that
    // escalated may be this pair's meeting; an entity left unexamined
    // against everything may meet either solid unseen; and a pierce or
    // edge cross between two entities of ONE of the pair's solids is
    // that solid's boundary crossing itself, whose material no
    // placement can be read against.
    //
    // `records_on_their_word` is the declared-only pair's reading (the
    // loop's argument says why): its records are read only for a
    // decided crossing, and a record the analysis cannot read or a face
    // pair is taken on its word.
    let blocks = |standing_errors: &[ValidationError],
                  sa: SolidKey,
                  sb: SolidKey,
                  records_on_their_word: bool|
     -> Option<Undecided> {
        let owns = |id: EntityId| solid_of_entity(id).is_some_and(|s| s == sa || s == sb);
        let between = |x: EntityId, y: EntityId| straddles(sa, sb, x, y);
        let mut undecided_touch = None;
        for e in standing_errors {
            let what = match Named::of(e) {
                Named::Nothing => Some(Undecided::Unexamined),
                Named::One(id) => owns(id).then_some(Undecided::Unexamined),
                Named::Two(x, y, said) if between(x, y) => match said {
                    Said::Touch(site) => touch(site),
                    Said::Pierce => Some(Undecided::Crossing),
                    Said::CurvedTouch => Some(Undecided::TouchUnreadable),
                    Said::Unexamined => Some(Undecided::Unexamined),
                },
                Named::Two(x, y, said) if owns(x) && solid_of_entity(x) == solid_of_entity(y) => {
                    match said {
                        Said::Touch(TouchSite::EdgeCross(..)) | Said::Pierce => {
                            Some(Undecided::Crossing)
                        }
                        Said::Unexamined => Some(Undecided::Unexamined),
                        Said::Touch(_) | Said::CurvedTouch => None,
                    }
                }
                Named::Two(..) => None,
            };
            match what {
                // A touch the analysis could not decide waits: a
                // decided crossing, or any other reason, standing
                // later in the findings is the stronger refusal.
                Some(why) if why.is_undecided_touch() => {
                    undecided_touch = undecided_touch.or(what);
                }
                Some(_) => return what,
                None => {}
            }
        }
        // Declared touches leave no finding and are confirmed for their
        // coincidence only, never for their side: the same analysis runs
        // over the records that name this pair, and over every point the
        // two share a key at, named or not. A shared key is structure
        // the body holds, not a declaration: its reading is never taken
        // on its word, so an undecided one refuses.
        let records = recorded(sa, sb);
        let sites = records
            .iter()
            .filter_map(|r| match *r {
                Recorded::Site(site) => Some((site, !records_on_their_word)),
                Recorded::FacePair => None,
            })
            .chain(shared(sa, sb).into_iter().map(|site| (site, true)));
        for (site, strict) in sites {
            match touch(site) {
                Some(Undecided::MixedTouch) => return Some(Undecided::MixedTouch),
                what if strict => undecided_touch = undecided_touch.or(what),
                _ => {}
            }
        }
        if !records_on_their_word && records.iter().any(|r| matches!(r, Recorded::FacePair)) {
            return Some(Undecided::RecordedFacePair);
        }
        undecided_touch
    };
    // ---- The material test of one ordering: every vertex of `inner`
    // against `outer`'s material.
    enum Probe {
        /// A vertex of `inner` strictly inside `outer`'s material.
        In(VertexKey),
        /// The door refused a vertex (and none was `In`).
        Refused(&'static str),
        /// Every vertex on `outer`'s boundary.
        AllOn,
        /// Every vertex outside or on the boundary, at least one outside.
        Clear,
        /// `inner` has no vertex to probe.
        NoVertex,
    }
    // One selection per container, shared by every pair it outers, so
    // its side at infinity is read once.
    let selections: std::collections::BTreeMap<
        SolidKey,
        std::sync::OnceLock<Result<crate::boolean::SolidFaces, crate::boolean::PointInSolidError>>,
    > = body
        .solids()
        .map(|(s, _)| (s, std::sync::OnceLock::new()))
        .collect();
    let probe = |outer: SolidKey, inner: SolidKey| -> Probe {
        let selected = match selections.get(&outer) {
            Some(cell) => cell.get_or_init(|| crate::boolean::SolidFaces::of(body, outer)),
            None => &Err(crate::boolean::PointInSolidError::NoSuchSolid { solid: outer }),
        };
        let sel = match selected {
            Ok(sel) => sel,
            Err(e) => return Probe::Refused(Undecided::of_point_in_solid(e).what()),
        };
        let (mut refused, mut out_seen, mut any) = (None, false, false);
        for &(v, p) in geo
            .verts
            .iter()
            .filter(|(v, _)| vertex_solid(*v) == Some(inner))
        {
            any = true;
            match crate::boolean::point_in_solid_faces(body, sel, p, band, tol) {
                Ok(SolidContainment::In) => return Probe::In(v),
                Ok(SolidContainment::Out) => out_seen = true,
                Ok(SolidContainment::OnBoundary) => {}
                Err(e) => {
                    refused.get_or_insert(Undecided::of_point_in_solid(&e).what());
                }
            }
        }
        match (any, refused, out_seen) {
            (false, _, _) => Probe::NoVertex,
            (_, Some(what), _) => Probe::Refused(what),
            (_, None, true) => Probe::Clear,
            (_, None, false) => Probe::AllOn,
        }
    };
    // The pre-filter over the instances' extents (`Trees` docs): a
    // solid's extent is its vertex hull joined with its reach box,
    // padded — the box each ordering below reads on one side or the
    // other — and an instance with no sound reach takes the poison
    // box, so the refusal it owes every partner is still pushed. Two
    // extents disjoint on an axis put a definite Negative among that
    // axis's margins in BOTH orderings, which is exactly the clearance
    // the gate finds; pruning the pair loses no finding.
    let extent_boxes: Vec<Aabb> = solids
        .iter()
        .map(
            |&(&solid, &(lo, hi))| match solid_reach.get(&solid).copied().flatten() {
                Some((rlo, rhi)) => Aabb::from_points([lo, hi, rlo, rhi])
                    .map_or_else(Aabb::poison, |b| b.padded(pad)),
                None => Aabb::poison(),
            },
        )
        .collect();
    let extents = cands.class(extent_boxes);
    // The gate shells' hulls of a solid, the contained side of the gate.
    let hulls_of = |solid: SolidKey| -> Vec<Hull<T>> {
        gate_shells
            .iter()
            .filter(|(s, _)| *s == solid)
            .map(|&(_, b)| b)
            .collect()
    };
    for (i, &(&sa, _)) in solids.iter().enumerate() {
        for j in extents.later(i) {
            let &(&sb, _) = candidate(&solids, j);
            let before = errors.len();
            // No deferral on records here, deliberately: every record in
            // `ContactRecords` states one coincidence, and this arm's
            // question is where one instance sits relative to another.
            // A record is a meeting, so a pair carrying records is
            // probed; what differs is only how its records are read
            // (`blocks`' `records_on_their_word`).
            //
            // The GATE, per ordering and per non-void SHELL of `inner`:
            // the shell's vertex hull against `outer`'s reach box; a
            // definitely-negative margin on any axis says that shell is
            // not inside `outer`'s reach. A container whose extent no
            // sound box claims refuses the ordering.
            let mut unclaimable = false;
            let mut reaches = false;
            for (outer, inner) in [(sa, sb), (sb, sa)] {
                let Some((olo, ohi)) = solid_reach.get(&outer).copied().flatten() else {
                    errors.push(ValidationError::CensusUndecidable {
                        a: EntityId::Solid(outer),
                        b: EntityId::Solid(inner),
                        what: Undecided::Unclaimable.what(),
                    });
                    unclaimable = true;
                    continue;
                };
                for (ilo, ihi) in hulls_of(inner) {
                    let margins = [
                        ilo.x - olo.x,
                        ohi.x - ihi.x,
                        ilo.y - olo.y,
                        ohi.y - ihi.y,
                        ilo.z - olo.z,
                        ohi.z - ihi.z,
                    ];
                    let separated = margins.into_iter().any(|m| {
                        matches!(
                            decide("census_backstop_containment", Margin::of(m), band),
                            Ok(Sign::Negative)
                        )
                    });
                    reaches |= !separated;
                }
            }
            let found = meets_found(&errors[..standing], sa, sb);
            let declared_only =
                !found && (!recorded(sa, sb).is_empty() || !shared(sa, sb).is_empty());
            // **Why arm 2 decides the pair** — the one statement of the
            // argument; the module and arm docs point here.
            //
            // Let `U` be the overlap of the two interiors. Where the
            // boundaries meet in a rest no point of `U` is near (up to
            // the tolerance below), so if every
            // meeting is a rest, `U`'s boundary splits into points of ∂A
            // inside B and points of ∂B inside A. Each part is open and
            // closed in its own boundary, so it is a union of whole
            // SHELLS, each lying wholly inside the other solid's
            // material. A component of `U` is bounded, and the outermost
            // of the shells bounding it encloses it: `U` lies on that
            // shell's bounded side, and `U` is inside its own solid's
            // material there, so the shell bounds material from outside
            // — an OUTER shell. So `U` is non-empty only if some outer
            // shell of one solid lies wholly inside the other's material:
            // its vertices are strictly inside, and its hull is inside
            // the other's reach box. Hence:
            // - nothing on record and every outer shell separated:
            //   nothing can overlap, and the pair clears at the gate;
            // - otherwise every vertex is probed both ways, and the
            //   findings about the pair are read (`blocks`); a pair
            //   whose meetings are all rests and whose vertices are all
            //   outside or on clears.
            //
            // The premises, and what checks them. (1) A shell is one
            // CONNECTED closed surface, so it has a bounded side: tier 2
            // (`validate_closed`, c = 1 per shell) runs before the
            // census, and the census refuses a solid whose boundary
            // crosses itself (`blocks`). (2) A void shell's role is the
            // sign of its own volume, and tier 3's check 10 has held
            // the shells' windings to 0 or 1; a shell whose role does
            // not read is gated as outer. (3) The census is COMPLETE for
            // planar boundaries — every point where they meet stands as
            // a finding or a record — which is why `blocks` refuses on
            // an escalation or an unexamined entity. A curved pair is
            // arm 1's: it refuses every one it cannot prove disjoint,
            // and its clears (box, direction and depth gaps) are such
            // proofs, so no curved meeting goes unseen. (4) Every shell
            // has a vertex the probe reads: tier 1 refuses a shell with
            // no face (`ShellWithoutFaces`), every face has an outer
            // loop, and tier 2 refuses a lone-vertex loop at rest
            // (`ScaffoldingEmptyLoop`), so each face's outer loop is a
            // cycle of half-edges whose vertices `face_points` and the
            // probe both see — no seamless sphere or torus shell is
            // vertex-less. A pair with nothing on
            // record clears at the gate even beside an escalation, which
            // stands as the body's refusal.
            //
            // **What a rest certifies, and so the tolerance of this
            // argument** ([`rest::judge`]). A rest reads each face of
            // both links through its piece at the touch point `p` — the
            // part of the face visible from `p` — and certifies, on
            // readings at the pieces' own points, that no piece of either
            // link crosses a face of the other or lies, past `zero`, in
            // the other's material. So:
            // - within the ball about `p` that reaches the nearest
            //   boundary of the pieces read, the materials are the two
            //   links' cones, and every point where one enters the other
            //   lies within `zero` of a face of each: coincidence under
            //   D4, not interference;
            // - beyond that ball, an overlap deeper than the band is a
            //   dip of some face below the other solid's boundary.
            //   Signed distance is affine on a planar face, so the dip
            //   shows at a vertex of that face, which the material test
            //   probes, or where the dip's boundary crosses the other
            //   solid's edges, which stands as a finding or an
            //   escalation that `blocks` reads.
            // "No point of `U` is near a rest" above holds in exactly
            // that sense: `U` near a rest is confined to that slab.
            //
            // What it cannot see: a pair whose ONLY meetings are
            // declared. It is probed, and a declared v-on-f or v-v touch
            // that decidedly crosses refuses, but a declared touch the
            // analysis cannot read and the events a declared face pair
            // backs are taken on the records' word — reading them
            // refuses ratified declared seats
            // (`work/contacthold/declared-only-meetings-clear-at-the-census-gate-unread.md`).
            // A point the two share a key at is no declaration: such a
            // pair is probed too, and its shared points are read strictly,
            // an undecided reading refusing (`blocks`).
            if !unclaimable && (reaches || found || declared_only) {
                // The material test, both orderings, every vertex. An
                // `In` is the decided interference whatever else stands.
                let mut interfered = false;
                let mut undecided = Vec::new();
                for (outer, inner) in [(sa, sb), (sb, sa)] {
                    let what = match probe(outer, inner) {
                        Probe::In(witness) => {
                            interfered = true;
                            errors.push(ValidationError::InstanceInterference {
                                outer,
                                inner,
                                witness,
                            });
                            continue;
                        }
                        Probe::Clear => continue,
                        Probe::Refused(what) => what,
                        Probe::AllOn => Undecided::AllOn.what(),
                        Probe::NoVertex => Undecided::NoVertex.what(),
                    };
                    undecided.push(ValidationError::CensusUndecidable {
                        a: EntityId::Solid(outer),
                        b: EntityId::Solid(inner),
                        what,
                    });
                }
                // With no interference, the findings are read whether or
                // not the probe decided: a reason they give is reported
                // in place of the probe's own, and the probe's stands
                // when they give none.
                let why = if interfered {
                    None
                } else {
                    blocks(&errors[..standing], sa, sb, declared_only)
                };
                match why {
                    Some(why) => errors.push(ValidationError::CensusUndecidable {
                        a: EntityId::Solid(sa),
                        b: EntityId::Solid(sb),
                        what: why.what(),
                    }),
                    None => errors.extend(undecided),
                }
            }
            if let Some(t) = trace.as_deref_mut() {
                t.backstop.note(
                    (EntityId::Solid(sa), EntityId::Solid(sb)),
                    errors.len() > before,
                );
            }
        }
    }
}

/// The confirmation direction of the certification diff: every
/// declaration must have a geometric witness — dead keys, equal keys,
/// and coincidence-free records are stale, typed.
fn confirm_records<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    contacts: &ContactRecords,
    band: Band,
    region: Option<RegionLane<T>>,
    errors: &mut Vec<ValidationError>,
) {
    for kind in contacts.kinds() {
        match kind {
            ContactKind::VertexVertex(a, b) => confirm_vertex_vertex(geo, a, b, band, errors),
            ContactKind::VertexEdge(v, e) => confirm_vertex_on_edge(body, geo, v, e, band, errors),
            ContactKind::VertexFace(v, f) => confirm_vertex_on_face(body, geo, v, f, band, errors),
            ContactKind::EdgeEdge(a, b) => confirm_edge_edge(body, geo, a, b, band, errors),
            ContactKind::EdgeFace(e, f) => confirm_edge_face(body, geo, e, f, band, errors),
            ContactKind::FaceFace(a, b) => {
                confirm_face_pair(body, geo, a, b, band, region, errors);
            }
        }
    }
}

/// One vertex–vertex record's witness: both vertices live, and at one
/// point.
fn confirm_vertex_vertex<T: Decide>(
    geo: &Geo<T>,
    a: VertexKey,
    b: VertexKey,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    let stale = ValidationError::StaleContactDeclaration {
        declaration: StaleDeclaration::VertexVertex { a, b },
    };
    let (Some(&pa), Some(&pb)) = (geo.vmap.get(&a), geo.vmap.get(&b)) else {
        errors.push(stale);
        return;
    };
    match gap_is_zero("pm_census_confirm_vv", Margin::norm3(pa - pb), band, errors) {
        Some(true) | None => {}
        Some(false) => errors.push(stale),
    }
}

/// The conventional vertices of `edge`: points of its interior that
/// the topology keeps as vertices ([`Geo::conventional`]).
fn conventional_vertices<T: Real>(geo: &Geo<T>, edge: EdgeKey) -> Vec<VertexKey> {
    geo.conventional
        .iter()
        .filter(|&(_, &e)| e == edge)
        .map(|(&v, _)| v)
        .collect()
}

/// One edge–face record's witness, read from the two cells' carriers:
/// both cells live and their interiors meeting.
///
/// - **A straight edge in a plane** is confirmed by reconstruction: both
///   its ends on the face's plane, and a cell of its span inside the
///   face's region ([`ef_overlap_cells`], the overlap pass 4 reports).
///   Off the plane or outside the region, the record is stale.
/// - **Otherwise** the touch is read where the record holds a point of
///   the edge's interior: a conventional vertex, inside the face
///   ([`confirm_vertex_on_face`]'s two readings), or else the point
///   touch of the two carriers ([`point::edge_face`]). A circle lying in
///   a plane away from such a vertex is a curve touch, refused typed:
///   the census builds no lane for it.
fn confirm_edge_face<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    edge: EdgeKey,
    face: FaceKey,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    let stale = ValidationError::StaleContactDeclaration {
        declaration: StaleDeclaration::EdgeFace { edge, face },
    };
    if body.get_edge(edge).is_none() || body.get_face(face).is_none() {
        errors.push(stale);
        return;
    }
    let line = geo.line_edge(edge);
    let plane = planar_face(geo, face);
    if let (Some(e), Some(f)) = (line, plane) {
        for p in [e.p0, e.p1] {
            match signed_is_zero(
                "pm_census_confirm_ef",
                Margin::of((p - f.origin).dot(f.normal)),
                band,
                errors,
            ) {
                Some(true) => {}
                Some(false) => {
                    errors.push(stale);
                    return;
                }
                None => return,
            }
        }
        let before = errors.len();
        let cells = ef_overlap_cells(body, e, f, geo, band, errors);
        if cells.is_empty() && errors.len() == before {
            errors.push(stale);
        }
        return;
    }
    for v in conventional_vertices(geo, edge) {
        let mut read = Vec::new();
        confirm_vertex_on_face(body, geo, v, face, band, &mut read);
        if read.is_empty() {
            return;
        }
        if read.iter().any(|e| {
            !matches!(
                e,
                ValidationError::StaleContactDeclaration {
                    declaration: StaleDeclaration::VertexOnFace { .. }
                }
            )
        }) {
            errors.extend(read);
            return;
        }
    }
    if point::edge_face(body, edge, face, band, errors) == point::Read::Stale {
        errors.push(stale);
    }
}

/// One face–face record's witness, read from the two faces' carriers.
///
/// - **One carrier, or two planes**: the patch doors ([`confirm_patch`]),
///   whose carrier door contradicts two distinct planes.
/// - **A sphere and a plane, or two spheres**: their point touch
///   ([`point::face_face`]).
/// - **Two other carriers**: the closed-form tangent line they derive
///   and both faces' extents along it
///   ([`crate::boolean::tangent_pair_at_rest`]). Extents definitely
///   apart are stale. Where they meet, the line is read through both
///   faces the way the op reads it
///   ([`crate::boolean::line_through_both_faces`]): every station out
///   of either is stale, whatever the two faces' sides, since a pair the
///   line misses does not meet. Some station inside both reads the
///   table along it, sides included
///   ([`crate::boolean::tangent_table_at_rest`]), which confirms or
///   contradicts the record. Refused typed where no line derives.
fn confirm_face_pair<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    a: FaceKey,
    b: FaceKey,
    band: Band,
    region: Option<RegionLane<T>>,
    errors: &mut Vec<ValidationError>,
) {
    let stale = ValidationError::StaleContactDeclaration {
        declaration: StaleDeclaration::Patch {
            face_a: a,
            face_b: b,
        },
    };
    if body.get_face(a).is_none() || body.get_face(b).is_none() {
        errors.push(stale);
        return;
    }
    if planar_face(geo, a).is_some() && planar_face(geo, b).is_some() {
        confirm_patch(body, a, b, band, region, errors);
        return;
    }
    // The detector posture reads which: a carrier the ladder calls one,
    // or one within the band (the record then stands as the declaration
    // the patch door's rung 3 reads), is a patch.
    match crate::boolean::carrier_pair_relation(body, a, body, b, false, band) {
        Ok(Ok(crate::boolean::CarrierRelation::Distinct))
        | Err(crate::boolean::PairUnread::OutsideInventory) => {}
        Ok(Ok(_) | Err(crate::boolean::CarrierEqError::Undecided { .. })) => {
            confirm_patch(body, a, b, band, region, errors);
            return;
        }
        Ok(Err(
            crate::boolean::CarrierEqError::Escalated { diag, .. }
            | crate::boolean::CarrierEqError::Contradicted { diag, .. }
            | crate::boolean::CarrierEqError::Unsettled { diag },
        )) => {
            errors.push(ValidationError::CensusEscalated { cause: diag });
            return;
        }
        Err(crate::boolean::PairUnread::Extent(_)) => {
            errors.push(ValidationError::CensusUnsupported {
                subject: CensusSubject::FacePair(a, b),
                cause: CensusUnsupportedCause::ContactLane(
                    crate::contact::ContactRefusal::NotCertifiable {
                        what: "a face of the face-face record has no extent to read",
                    },
                ),
            });
            return;
        }
    }
    if let Some(read) = point::face_face(body, a, b, band, errors) {
        if read == point::Read::Stale {
            errors.push(stale);
        }
        return;
    }
    let refused = |refusal| match refusal {
        crate::contact::ContactRefusal::Contradicted { diag, steer } => {
            ValidationError::ContactContradicted {
                declaration: crate::contact::DeclaredContact {
                    a,
                    b,
                    class: crate::contact::ContactClass::Tangent,
                },
                witness: TANGENT_LOCUS_WITNESS.to_owned(),
                margin: diag,
                steer,
            }
        }
        crate::contact::ContactRefusal::Escalated { diag }
        | crate::contact::ContactRefusal::Undeclared { diag } => {
            ValidationError::CensusEscalated { cause: diag }
        }
        refusal @ crate::contact::ContactRefusal::NotCertifiable { .. } => {
            ValidationError::CensusUnsupported {
                subject: CensusSubject::FacePair(a, b),
                cause: CensusUnsupportedCause::ContactLane(refusal),
            }
        }
    };
    let (carrier, overlap) = match crate::boolean::tangent_pair_at_rest(body, a, b, band) {
        Ok(crate::boolean::RestTangent::Apart) => {
            errors.push(stale);
            return;
        }
        Ok(crate::boolean::RestTangent::Line { carrier, overlap }) => (carrier, overlap),
        Ok(crate::boolean::RestTangent::NoLine) => {
            errors.push(refused(crate::contact::ContactRefusal::NotCertifiable {
                what: "a face-face record on two carriers with no closed-form tangent locus \
                       (plane x cylinder along a ruling, parallel cylinders)",
            }));
            return;
        }
        Err(refusal) => {
            errors.push(refused(refusal));
            return;
        }
    };
    let through = crate::boolean::rest_line_in_both_faces(
        body,
        [a, b],
        carrier,
        overlap,
        band,
        |i, why, held| match why {
            crate::boolean::FaceUnread::Trim => held.push(ValidationError::CensusUnsupported {
                subject: CensusSubject::FacePair(a, b),
                cause: CensusUnsupportedCause::ContactLane(
                    crate::contact::ContactRefusal::NotCertifiable {
                        what: "a face of a two-carrier face-face record whose trim the \
                               containment door does not read",
                    },
                ),
            }),
            crate::boolean::FaceUnread::Placement(crate::boolean::PlacementError::OnPlane(
                cause,
            )) => held.push(ValidationError::CensusEscalated { cause }),
            crate::boolean::FaceUnread::Placement(crate::boolean::PlacementError::Door(e)) => {
                read_containment([a, b][i], Err::<(), _>(e), held);
            }
        },
    );
    let line = match through {
        Ok(Some(line)) => line,
        Ok(None) => {
            errors.push(stale);
            return;
        }
        Err(held) => {
            errors.extend(held);
            return;
        }
    };
    if let Err(refusal) = crate::boolean::tangent_table_at_rest(&line, true, band) {
        errors.push(refused(refusal));
    }
}

/// Whether a point is in two cells, each reading pushing what stopped
/// it: `Some(false)` where either reads it out, whatever the other
/// pushed (a decided answer drops the other's escalations); `Some(true)`
/// where both read it in; `None` otherwise, with what both pushed.
pub(crate) fn in_both<E>(
    a: impl FnOnce(&mut Vec<E>) -> Option<bool>,
    b: impl FnOnce(&mut Vec<E>) -> Option<bool>,
    errors: &mut Vec<E>,
) -> Option<bool> {
    let mut held = Vec::new();
    let on_a = a(&mut held);
    if on_a == Some(false) {
        return Some(false);
    }
    match (on_a, b(&mut held)) {
        (_, Some(false)) => Some(false),
        (Some(true), Some(true)) => Some(true),
        _ => {
            errors.extend(held);
            None
        }
    }
}

/// Where `face` holds `q`, a point on its carrier: the boolean's own
/// placement ([`crate::boolean::face_placement`]), a point decided off a
/// planar face's plane read `Out`. `Some(None)` where the curved door
/// leaves the point unread; `None` where a decision escalated or the door
/// refused (pushed).
pub(crate) fn face_holds<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    q: Point3<T>,
    band: Band,
    on_plane: &'static str,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<FaceContainment>> {
    use crate::boolean::{CurvedPlacement, PlacementError};
    match crate::boolean::face_placement(body, face, q, band, on_plane) {
        Ok(CurvedPlacement::OffCarrier) => Some(Some(FaceContainment::Out)),
        Ok(CurvedPlacement::Trim(at)) => Some(at),
        Err(PlacementError::OnPlane(cause)) => {
            errors.push(ValidationError::CensusEscalated { cause });
            None
        }
        Err(PlacementError::Door(e)) => {
            read_containment(face, Err::<(), _>(e), errors).map(|()| None)
        }
    }
}

/// One `(vertex, face)` record's witness: both cells live, and the
/// vertex strictly inside the face — on its plane and inside its region
/// for a planar face, inside its trim by the curved containment door
/// ([`crate::boolean::curved_face_containment`], which puts a point off
/// the carrier `Out`) for a curved one.
fn confirm_vertex_on_face<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    vertex: VertexKey,
    face: FaceKey,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    let stale = ValidationError::StaleContactDeclaration {
        declaration: StaleDeclaration::VertexOnFace { vertex, face },
    };
    let Some(&q) = geo.vmap.get(&vertex) else {
        errors.push(stale);
        return;
    };
    // A body edited after its records were made can leave a record's
    // vertex on its face's boundary: incident, so not a contact.
    if crate::boolean::incident(
        body,
        [
            crate::boolean::Cell::Vertex(vertex),
            crate::boolean::Cell::Face(face),
        ],
    ) {
        errors.push(stale);
        return;
    }
    if planar_face(geo, face).is_none() && !geo.curved_faces.contains(&face) {
        errors.push(stale);
        return;
    }
    match face_holds(body, face, q, band, "pm_census_confirm_vf", errors) {
        Some(Some(FaceContainment::In)) => {}
        Some(Some(_)) => errors.push(stale),
        // `None` is the door's remainder, not a refusal: no
        // `ContainError` stands behind it to carry, so it is named the
        // way the census names a configuration outside its lanes.
        Some(None) => errors.push(ValidationError::CensusUnsupported {
            subject: CensusSubject::Entity(EntityId::Face(face)),
            cause: CensusUnsupportedCause::ContactLane(
                crate::contact::ContactRefusal::NotCertifiable {
                    what: "a vertex-on-face record on a curved face whose trim \
                           the containment door does not read",
                },
            ),
        }),
        None => {}
    }
}

/// One `(vertex, edge)` record's witness: both cells live, the vertex
/// not an end of the edge, on the edge's carrier and strictly inside its
/// span: pass 2's two decisions on a line, so the two directions agree,
/// and the curved lane's ([`curved::on_curved_interior`]) on any other
/// carrier.
fn confirm_vertex_on_edge<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    vertex: VertexKey,
    edge: EdgeKey,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    let stale = ValidationError::StaleContactDeclaration {
        declaration: StaleDeclaration::VertexOnEdge { vertex, edge },
    };
    let Some(&q) = geo.vmap.get(&vertex) else {
        errors.push(stale);
        return;
    };
    // A body edited after its records were made can leave a record's
    // vertex at its edge's end: incident, so not a contact.
    if body.get_edge(edge).is_none()
        || crate::boolean::incident(
            body,
            [
                crate::boolean::Cell::Vertex(vertex),
                crate::boolean::Cell::Edge(edge),
            ],
        )
    {
        errors.push(stale);
        return;
    }
    let meets = match geo.line_edge(edge) {
        Some(e) => on_edge_interior(q, e, band, errors),
        None => curved::on_curved_interior(body, edge, q, band, errors),
    };
    if meets == Some(false) {
        errors.push(stale);
    }
}

/// One edge-edge record's witness: both edges live, and
/// their interiors meet — crossing inside both spans, or overlapping
/// along a positive length — by the edge-edge lanes' own decisions on
/// two lines, and the curved lane's
/// ([`curved::curved_interiors_meet`]) where either is not one.
fn confirm_edge_edge<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    a: EdgeKey,
    b: EdgeKey,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    let stale = ValidationError::StaleContactDeclaration {
        declaration: StaleDeclaration::EdgeEdge { a, b },
    };
    let lookup = |k: EdgeKey| geo.line_edge(k);
    let (Some(ea), Some(eb)) = (lookup(a), lookup(b)) else {
        let live = |k| body.get_edge(k).is_some();
        if !live(a)
            || !live(b)
            || curved::curved_interiors_meet(body, a, b, band, errors) == Some(false)
        {
            errors.push(stale);
        }
        return;
    };
    if interiors_meet(ea, eb, band, errors) == Some(false) {
        errors.push(stale);
    }
}

/// Whether two line edges' interiors meet, crossing or overlapping
/// ([`ee_meeting`]): the question an edge-edge record's confirm pass
/// asks. `None` where a decision escalated (pushed).
fn interiors_meet<T: Decide>(
    ea: &EdgeGeo<T>,
    eb: &EdgeGeo<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    ee_meeting(ea, eb, band, errors).map(|met| met.is_some())
}

/// A segment's census geometry, for the questions an op asks of two
/// segments before any body holds them as edges.
fn segment<T: Real>((p0, p1): (Point3<T>, Point3<T>)) -> EdgeGeo<T> {
    let chord = p1 - p0;
    EdgeGeo {
        key: EdgeKey::default(),
        v0: VertexKey::default(),
        v1: VertexKey::default(),
        p0,
        p1,
        points: [None; 2],
        dir: chord.normalize(),
        len: chord.norm(),
        f_plus: FaceKey::default(),
        f_minus: FaceKey::default(),
    }
}

/// **A record's confirm pass, one pair at a time**: whether two cells'
/// interiors meet in `body`, decided as [`confirm_records`]
/// decides a record of them, so a record placed by this answer is one
/// the census confirms. The body is read once, at [`Self::of`]. The
/// chart-region door is the caller's, as at every census door: without
/// it a face–face pair is outside the confirm's lanes.
pub(crate) struct RecordConfirm<'b, T: Decide> {
    body: &'b Body<T>,
    geo: Geo<T>,
    band: Band,
    region: Option<RegionLane<T>>,
}

impl<'b, T: Decide> RecordConfirm<'b, T> {
    pub(crate) fn of(body: &'b Body<T>, band: Band, region: Option<RegionLane<T>>) -> Self {
        Self {
            body,
            geo: snapshot(body),
            band,
            region,
        }
    }

    /// Whether a record of `pair` is confirmed: `Ok(false)` where the
    /// census reads it stale.
    ///
    /// # Errors
    ///
    /// The first finding that is not staleness: an escalated decision,
    /// or a pair outside the census's lanes.
    pub(crate) fn meets(&self, pair: crate::Contact) -> Result<bool, ValidationError> {
        let one: ContactRecords = core::iter::once(crate::coincidence::Cited::new(
            pair,
            crate::coincidence::Cites::decided(0),
        ))
        .collect();
        let mut errors = Vec::new();
        confirm_records(
            self.body,
            &self.geo,
            &one,
            self.band,
            self.region,
            &mut errors,
        );
        match errors
            .iter()
            .find(|e| !matches!(e, ValidationError::StaleContactDeclaration { .. }))
        {
            Some(finding) => Err(finding.clone()),
            None => Ok(errors.is_empty()),
        }
    }
}

/// [`parallel_reading`] over two segments: whether their lines are parallel,
/// decided as an edge-edge record's confirm pass decides it.
///
/// # Errors
///
/// The escalated decision.
pub(crate) fn segments_parallel<T: Decide>(
    a: (Point3<T>, Point3<T>),
    b: (Point3<T>, Point3<T>),
    band: Band,
) -> Result<bool, Indeterminate> {
    let mut errors = Vec::new();
    let parallel = parallel_reading(&segment(a), &segment(b), band, &mut errors);
    escalation(parallel, errors)
}

/// The point of `a`'s line nearest `b`'s, for two segments whose lines
/// are not parallel: where they cross, once they are decided to meet
/// ([`ee_cross_point`]).
pub(crate) fn segment_cross_point<T: Real>(
    a: (Point3<T>, Point3<T>),
    b: (Point3<T>, Point3<T>),
) -> Point3<T> {
    ee_cross_point(&segment(a), &segment(b))
}

/// A census answer, or the escalation it pushed.
///
/// # Panics
///
/// Where the question answered `None` and pushed no escalation: its
/// contract is `None` exactly where a decision escalated, so a `None`
/// with nothing pushed is a broken invariant, never a "no".
fn escalation(answer: Option<bool>, errors: Vec<ValidationError>) -> Result<bool, Indeterminate> {
    for e in errors {
        if let ValidationError::CensusEscalated { cause } = e {
            return Err(cause);
        }
    }
    match answer {
        Some(answer) => Ok(answer),
        None => unreachable!("a census question answers `None` only where it pushed an escalation"),
    }
}

/// **Door 1 of a face pair at rest**: carrier identity and opposed
/// senses through the `Rest` door, the pair standing as its own
/// declaration (C3: rung 2/3, never value-equal; aligned coincidence
/// contradicts), with the margin that decided the one carrier. A
/// refusal is the census finding it raises: the declaration is carried
/// in the pair's own order, and the witness names the LOCUS, not the
/// pair (`declaration` carries that) — Door 1 compares the two faces'
/// carriers whole and hands back no point, so the locus is the
/// surfaces.
fn rest_door_one<T: Decide>(
    body: &Body<T>,
    face_a: FaceKey,
    face_b: FaceKey,
    band: Band,
) -> Result<(crate::contact::ContactVerdict, geom_core::MarginDiag), ValidationError> {
    crate::boolean::rest_pair_reading(body, face_a, body, face_b, band).map_err(|refusal| {
        match refusal {
            crate::contact::ContactRefusal::Contradicted { diag, steer } => {
                ValidationError::ContactContradicted {
                    declaration: crate::contact::DeclaredContact {
                        a: face_a,
                        b: face_b,
                        class: crate::contact::ContactClass::Rest,
                    },
                    witness: CARRIER_COMPARISON_WITNESS.to_owned(),
                    margin: diag,
                    steer,
                }
            }
            crate::contact::ContactRefusal::Escalated { diag }
            | crate::contact::ContactRefusal::Undeclared { diag } => {
                ValidationError::CensusEscalated { cause: diag }
            }
            refusal @ crate::contact::ContactRefusal::NotCertifiable { .. } => {
                ValidationError::CensusUnsupported {
                    subject: CensusSubject::FacePair(face_a, face_b),
                    cause: CensusUnsupportedCause::ContactLane(refusal),
                }
            }
        }
    })
}

/// **The at-rest census's decision that two faces of a body rest on one
/// carrier, opposed**: the row a contact record between them cites
/// ([`crate::DecisionSite::CensusAtRest`]), decided by the same Door 1
/// the census confirms a patch record through, so the row and the
/// census read one door.
///
/// # Errors
///
/// The finding the census raises for the pair where Door 1 refuses it.
pub fn census_rest_decision<T: Decide>(
    body: &Body<T>,
    face_a: FaceKey,
    face_b: FaceKey,
    band: Band,
) -> Result<crate::Coincidence, ValidationError> {
    let (_, margin) = rest_door_one(body, face_a, face_b, band)?;
    let cell = |face| crate::RowCell::Result {
        cell: crate::Cell::Face(face),
    };
    Ok(crate::Coincidence {
        cells: [cell(face_a), cell(face_b)],
        relation: crate::Relation::SameOpposite,
        site: crate::DecisionSite::CensusAtRest,
        margin,
        discharge: crate::Discharge::of(margin),
    })
}

/// **What the census recorded over one body at rest**
/// ([`record_at_rest`]).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RecordedAtRest {
    /// A record for each touch between two groups that no record
    /// backed, each citing the rows of [`Self::rows`] that decided its
    /// pair, numbered from the caller's `first_row`. They certify the
    /// body only beside empty [`Self::findings`]: a refusal leaves them
    /// as far as the census got.
    pub records: ContactRecords,
    /// The census's decisions those records cite, in the order it made
    /// them ([`crate::DecisionSite::CensusAtRest`]).
    pub rows: Vec<crate::Coincidence>,
    /// Every other finding, in the census's order: a touch inside one
    /// group (that group's defect, D1 (iii)), every touch between two
    /// groups the census found interpenetrating, and whatever else the
    /// census refuses.
    pub findings: Vec<ValidationError>,
}

/// How many times [`record_at_rest`] runs the census before it reports
/// what it found: once to find the touches, once to confirm their
/// records, and twice more for what a recorded touch exposes — a pair
/// found interpenetrating whose touches were recorded on an earlier
/// pass.
const RECORDING_PASSES: usize = 4;

/// Two groups, the lesser first.
type GroupPair = (u32, u32);

/// **The tier-3′ census over a body at rest, recording what it
/// decides** (D1 (ii), (iii); D10): every touch it finds between cells
/// of two different groups that no record backs is a row of its own
/// Zero, and a record of that pair citing it, and the census runs again
/// over the records so each is confirmed like any other. `group` is the caller's partition of the body's solids (a
/// placement's copies, an import's solids); a solid it answers `None`
/// for is in no group, and no touch of it is recorded. Where everything
/// is one group nothing is recorded, and the census is an op's
/// finished-body gate.
///
/// **Two groups that interpenetrate have nothing recorded.** Where the
/// census finds the pair's material overlapping, or cannot clear it —
/// an instance's vertex inside the other's material, an edge piercing
/// a face, a crossing whose side test reads `SameSide` or does not
/// decide, a cross-solid pair it cannot look at — every touch between
/// the two stays a finding: the census reads an overlap off those
/// touches, and a record would erase the evidence. A touch inside one
/// group stays a finding too.
///
/// `census` is the caller's door to the tier-3′ pass over `body` with
/// the records it is handed (its at-rest policy, or the certificate
/// form an import keeps): this door decides nothing of its own, and
/// what it records is only what that pass found.
pub fn record_at_rest<T: Real>(
    body: &Body<T>,
    contacts: &ContactRecords,
    mut census: impl FnMut(&ContactRecords) -> Vec<ValidationError>,
    group: &dyn Fn(crate::SolidKey) -> Option<u32>,
    first_row: u32,
) -> RecordedAtRest {
    let groups = |x: crate::SolidKey, y: crate::SolidKey| -> Option<GroupPair> {
        let (gx, gy) = (group(x)?, group(y)?);
        (gx != gy).then_some((gx.min(gy), gx.max(gy)))
    };
    let mut minted: Vec<(GroupPair, crate::Coincidence, crate::Contact)> = Vec::new();
    let mut overlapping: std::collections::BTreeSet<GroupPair> = std::collections::BTreeSet::new();
    let mut findings = Vec::new();
    let mut read = 0;
    for pass in 1..=RECORDING_PASSES {
        let records = cite_minted(contacts, &minted, first_row);
        read = minted.len();
        findings = census(&records);
        let found = findings
            .iter()
            .filter_map(|f| census_uncleared_solids(body, f))
            .filter_map(|(x, y)| groups(x, y))
            .filter(|pair| !overlapping.contains(pair))
            .collect::<Vec<_>>();
        let mut changed = !found.is_empty();
        overlapping.extend(found);
        let before = minted.len();
        minted.retain(|(pair, ..)| !overlapping.contains(pair));
        changed |= minted.len() != before;
        if pass < RECORDING_PASSES {
            let mut kept = Vec::new();
            for finding in findings {
                match recordable(body, &finding, &groups)
                    .filter(|(pair, ..)| !overlapping.contains(pair))
                {
                    Some(touch) => {
                        minted.push(touch);
                        changed = true;
                    }
                    None => kept.push(finding),
                }
            }
            findings = kept;
        }
        if !changed {
            break;
        }
    }
    // A last pass that found a pair interpenetrating dropped its records
    // after the census read them, so its touches are in no finding: read
    // the census once more over what is kept, and report what it finds.
    if minted.len() != read {
        findings = census(&cite_minted(contacts, &minted, first_row));
    }
    RecordedAtRest {
        records: cite_minted(&ContactRecords::default(), &minted, first_row),
        rows: minted.into_iter().map(|(_, row, _)| row).collect(),
        findings,
    }
}

/// `contacts`, then each of `minted` citing its row, numbered from
/// `first_row`.
fn cite_minted(
    contacts: &ContactRecords,
    minted: &[(GroupPair, crate::Coincidence, crate::Contact)],
    first_row: u32,
) -> ContactRecords {
    let mut records = contacts.clone();
    records.extend(minted.iter().zip(0u32..).map(|((_, _, contact), k)| {
        let row = first_row
            .checked_add(k)
            .unwrap_or_else(|| unreachable!("a body's rows number fewer than u32"));
        crate::Cited::new(*contact, crate::Cites::decided(row))
    }));
    records
}

/// A finding's touch between two groups, as the row recording it and
/// the record of its pair: `None` for any finding but a coincidence
/// [`census_contact_row`] reads, and for a touch not between two groups.
fn recordable(
    body: &Body<impl Real>,
    finding: &ValidationError,
    groups: &dyn Fn(crate::SolidKey, crate::SolidKey) -> Option<GroupPair>,
) -> Option<(GroupPair, crate::Coincidence, crate::Contact)> {
    let row = census_contact_row(finding)?;
    let [x, y] = row.cells.map(|cell| match cell {
        crate::RowCell::Result { cell } => Some(cell),
        crate::RowCell::Input { .. } | crate::RowCell::Tool => None,
    });
    let (x, y) = (x?, y?);
    let pair = groups(census_cell_solid(body, x)?, census_cell_solid(body, y)?)?;
    let contact = crate::Contact::new(body, x, y).ok()?;
    Some((pair, row, contact))
}

/// **The two solids a finding decides interfere**: an instance's vertex
/// inside the other's material, an edge piercing a face, or a crossing
/// whose side test read `SameSide`. `None` for every other finding,
/// including the ones that only fail to clear a pair
/// ([`census_uncleared_solids`]). The gather localizes these pairs as
/// interference; this is that reading's one home.
#[must_use]
pub fn census_interference_solids<T: Real>(
    body: &Body<T>,
    finding: &ValidationError,
) -> Option<(crate::SolidKey, crate::SolidKey)> {
    let edge = |e| census_cell_solid(body, crate::Cell::Edge(e));
    match finding {
        ValidationError::InstanceInterference { outer, inner, .. } => Some((*outer, *inner)),
        ValidationError::UnrecordedContact {
            contact: CensusContact::EdgeFacePierce { edge: e, face },
            ..
        } => Some((edge(*e)?, body.solid_of_face(*face)?)),
        ValidationError::UnrecordedContact {
            contact:
                CensusContact::EdgeEdgeCross {
                    a,
                    b,
                    side: Some(CrossingSideVerdict::SameSide),
                },
            ..
        } => Some((edge(*a)?, edge(*b)?)),
        _ => None,
    }
}

/// **The two solids a finding does not clear of interpenetrating**:
/// every pair [`census_interference_solids`] decides, and the two the
/// census could not decide, a crossing whose side test did not decide
/// and a pair it cannot look at (`CensusUndecidable`). `None` for every
/// other finding. [`record_at_rest`] records no touch between such a
/// pair, since the census reads the pair's overlap off those touches.
#[must_use]
pub fn census_uncleared_solids<T: Real>(
    body: &Body<T>,
    finding: &ValidationError,
) -> Option<(crate::SolidKey, crate::SolidKey)> {
    let edge = |e| census_cell_solid(body, crate::Cell::Edge(e));
    match finding {
        ValidationError::CensusUndecidable { a, b, .. } => Some((
            census_entity_solid(body, *a)?,
            census_entity_solid(body, *b)?,
        )),
        ValidationError::UnrecordedContact {
            contact:
                CensusContact::EdgeEdgeCross {
                    a,
                    b,
                    side: Some(CrossingSideVerdict::Undecided),
                },
            ..
        } => Some((edge(*a)?, edge(*b)?)),
        decided => census_interference_solids(body, decided),
    }
}

/// **The solid a cell of `body` belongs to**: a face's, or an edge's or
/// a vertex's through a half-edge of it; `None` for a lone vertex.
#[must_use]
pub fn census_cell_solid<T: Real>(body: &Body<T>, cell: crate::Cell) -> Option<crate::SolidKey> {
    let he = match cell {
        crate::Cell::Face(f) => return body.solid_of_face(f),
        crate::Cell::Edge(e) => body.get_edge(e)?.he_plus,
        crate::Cell::Vertex(v) => body.get_vertex(v)?.emanating?,
    };
    body.solid_of_face(body.face_of_half_edge(he)?)
}

/// **The solid a census finding's entity belongs to**: itself, or the
/// solid of its face, edge or vertex ([`census_cell_solid`]); `None`
/// for a shell, a loop or a half-edge, which no finding names.
#[must_use]
pub fn census_entity_solid<T: Real>(body: &Body<T>, entity: EntityId) -> Option<crate::SolidKey> {
    match entity {
        EntityId::Solid(s) => Some(s),
        EntityId::Face(f) => body.solid_of_face(f),
        EntityId::Edge(e) => census_cell_solid(body, crate::Cell::Edge(e)),
        EntityId::Vertex(v) => census_cell_solid(body, crate::Cell::Vertex(v)),
        EntityId::Shell(_) | EntityId::Loop(_) | EntityId::HalfEdge(_) => None,
    }
}

/// **The census's coincidence a finding reports, as the row recording
/// it** ([`crate::DecisionSite::CensusAtRest`]): a contact no record
/// backs whose Zero the census decided, its cells in the body's own keys
/// and its margin that Zero's. `None` for every other finding, and for a
/// contact no Zero decided or that may be interpenetration rather than a
/// coincidence: a pierce, and a crossing whose side test read `SameSide`
/// or did not decide.
#[must_use]
pub fn census_contact_row(finding: &ValidationError) -> Option<crate::Coincidence> {
    let ValidationError::UnrecordedContact {
        contact,
        margin: Some(margin),
        ..
    } = finding
    else {
        return None;
    };
    use crate::Cell::{Edge, Face, Vertex};
    let cells = match *contact {
        CensusContact::VertexVertex { a, b } => [Vertex(a), Vertex(b)],
        CensusContact::VertexOnFace { vertex, face } => [Vertex(vertex), Face(face)],
        CensusContact::VertexOnEdge { vertex, edge } => [Vertex(vertex), Edge(edge)],
        CensusContact::EdgeFaceOverlap { edge, face } => [Edge(edge), Face(face)],
        CensusContact::EdgeEdgeOverlap { a, b }
        | CensusContact::EdgeEdgeCross {
            a,
            b,
            side: None | Some(CrossingSideVerdict::OppositeSides),
        } => [Edge(a), Edge(b)],
        CensusContact::EdgeEdgeCross {
            side: Some(CrossingSideVerdict::SameSide | CrossingSideVerdict::Undecided),
            ..
        }
        | CensusContact::EdgeFacePierce { .. }
        | CensusContact::ConformalPatch { .. }
        | CensusContact::FaceFaceTangent { .. } => return None,
    };
    Some(crate::Coincidence {
        cells: cells.map(|cell| crate::RowCell::Result { cell }),
        relation: crate::Relation::OnCarrier,
        site: crate::DecisionSite::CensusAtRest,
        margin: *margin,
        discharge: crate::Discharge::of(*margin),
    })
}

/// The at-rest confirmation of a face–face record, as a patch (C3):
/// through the two doors its certification obligation names — the
/// `Rest` carrier/sense door (`contact_pair_verdict`: carrier identity
/// through the kind ladder with the record standing as the declaration,
/// senses opposed, aligned coincidence contradicted) and the
/// chart-region overlap predicate (region overlap in the shared chart
/// with definitely-positive area). Overlap `Empty` ⇒ the record is
/// STALE (C3's letter); an in-band overlap escalates; a pair the
/// predicate refuses typed (no exact-constant-arm chart, seam-branch
/// divergence, non-planar trims) is unsupported inventory — refused,
/// never sampled, never blessed.
///
/// A mate's record lands in the product body's `ContactRecords` — the
/// boolean 3′ currency, same type, no adapter — and THIS is the at-rest
/// evidence door those records certify through.
fn confirm_patch<T: Decide>(
    body: &Body<T>,
    face_a: FaceKey,
    face_b: FaceKey,
    band: Band,
    region: Option<RegionLane<T>>,
    errors: &mut Vec<ValidationError>,
) {
    let stale = ValidationError::StaleContactDeclaration {
        declaration: StaleDeclaration::Patch { face_a, face_b },
    };
    if body.get_face(face_a).is_none() || body.get_face(face_b).is_none() {
        errors.push(stale);
        return;
    }
    // Door 1 — carrier identity + opposed senses, the record
    // standing as its own declaration (C3: rung 2/3, never
    // value-equal; aligned coincidence contradicts).
    //
    // INVARIANT: the verdict is CARRIED to Door 2, not discarded.
    // Door 2's world-carrier arm exists only because this
    // declaration verified, so it must know which of C4's two
    // passing lists the pair landed in — `Definite` (the geometry's
    // own evidence) or `Bridged` (an in-band residue the
    // declaration covered) — rather than re-deriving or assuming
    // it.
    let door_one = match rest_door_one(body, face_a, face_b, band) {
        Ok((verdict, _)) => verdict,
        Err(error) => {
            errors.push(error);
            return;
        }
    };
    // Door 2 — region overlap in the pair's chart, definitely
    // positive (the PR-1 predicate through the region door the
    // pass holds), the chart being either the structural one or
    // the declared pair's shared world carrier.
    match region.map(|lane| lane.declared_overlap(body, face_a, body, face_b, door_one, band)) {
        // The pass holds no certified region door, exactly as at
        // the sweep arm and split from the geometry refusals below
        // for the same reason: this absence is a fact about the
        // RUN — the door it came through, at whatever scalar — the
        // same record is confirmed through the certified door at
        // any certifying scalar, and the recourse is a replay
        // through that door rather than a change to the geometry.
        None => {
            errors.push(ValidationError::CensusLaneUnsupported {
                subject: CensusSubject::FacePair(face_a, face_b),
            });
        }
        Some(Ok(crate::chart_region::ChartOverlap::PositiveArea)) => {}
        Some(Ok(crate::chart_region::ChartOverlap::Empty)) => errors.push(stale),
        Some(Err(ChartRegionError::Escalated(cause))) => {
            errors.push(ValidationError::CensusEscalated { cause });
        }
        // As the sweep arm: every other typed refusal is carried
        // whole rather than restated as the inventory refusal,
        // and the list is exhaustive so a new `ChartRegionError`
        // arm is a compile error here rather than a silent
        // promotion to an unrefuted frontier.
        Some(Err(
            cause @ (ChartRegionError::ChartDivergence { .. }
            | ChartRegionError::NonPlanarTrim { .. }
            | ChartRegionError::MissingCache { .. }
            | ChartRegionError::ArmUnbounded { .. }
            | ChartRegionError::SeamBranch
            | ChartRegionError::PeriodFold
            | ChartRegionError::CarrierTilt
            | ChartRegionError::TouchingBoundary
            | ChartRegionError::DegenerateLoop { .. }
            | ChartRegionError::RayExhausted
            | ChartRegionError::WitnessSegmentCapExceeded { .. }
            | ChartRegionError::WitnessCellCapExceeded { .. }
            | ChartRegionError::Corrupt),
        )) => {
            // Carried, as at the sweep arm and for the same
            // reason: which refusal fired is the whole of
            // what tells a reader which repair to make.
            errors.push(ValidationError::CensusUnsupported {
                subject: CensusSubject::FacePair(face_a, face_b),
                cause: CensusUnsupportedCause::ChartRegion(cause),
            });
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! The conformal face-pair arm and the patch-record certifier,
    //! pinned at the census door itself: the fixtures are open
    //! euler-built sheet scaffolds (the PR-1 chart fixtures'
    //! pattern), below tier 3's closed-body bar — which is exactly
    //! why these rows call [`census_and_certify`] directly. The
    //! closed-body end-to-end rows live in the acceptance suites.
    use super::*;
    use crate::entity::FaceKey;
    use crate::euler::{FaceSurface, MefSite, MevSite};
    use crate::test_support_fixtures::{CylFrame, CylKey, cyl_wall_sheet_keyed, unit_cyl_sheet};
    use geom::Surface;
    use geom_core::Tol;
    use geom_core::Vec3;

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// A refusal the solid door carries reads by its own kind: only an
    /// in-band walk is a witness too close to call, and one every ray
    /// grazed is not.
    #[test]
    fn a_carried_loop_refusal_is_placed_by_its_kind() {
        use crate::boolean::PointInSolidError as E;
        use crate::splitting::{
            OffPlane, OffPlaneCause, PointInLoopError as L, Uncrossable, UncrossableCarrier,
        };
        let r#loop = crate::entity::LoopKey::default();
        let read = |e: L| Undecided::of_point_in_solid(&E::Loop(e)).what();
        for grazed in [
            Undecided::of_point_in_solid(&E::RayExhausted).what(),
            read(L::RayExhausted { r#loop }),
        ] {
            assert_eq!(grazed, Undecided::WitnessGrazed.what());
        }
        assert_eq!(
            read(L::Escalated {
                r#loop,
                decision: crate::splitting::LoopDecision::Boundary,
                escalation: crate::splitting::Escalation::Margin,
                diag: geom_core::Indeterminate {
                    margin: geom_core::MarginDiag::value(5e-9),
                    band: Band::new(1e-9, 1e-8).expect("a well-formed band"),
                    predicate: None,
                    terminal_sliver: false,
                },
            }),
            Undecided::WitnessTooClose(Some(crate::splitting::LoopDecision::Boundary)).what()
        );
        assert_eq!(
            read(L::CorruptLoop { r#loop }),
            Undecided::CorruptInstance.what()
        );
        for cause in [
            OffPlaneCause::Query,
            OffPlaneCause::NormalNotUnit,
            OffPlaneCause::Loop {
                edge: crate::entity::EdgeKey::default(),
            },
        ] {
            assert_eq!(
                read(L::OffPlane(OffPlane { r#loop, cause })),
                Undecided::OffPlane.what(),
                "{cause:?}"
            );
        }
        assert_eq!(
            read(L::Uncrossable(Uncrossable {
                r#loop,
                edge: crate::entity::EdgeKey::default(),
                carrier: UncrossableCarrier::Spline,
            })),
            Undecided::FaceKindUnsupported.what()
        );
    }

    /// **The backstop writes no sentence of its own**: every `what` it
    /// raises comes from [`Undecided`], so its variants — what the
    /// refusal-budget row renders — are every `what` a run can show. A
    /// raise site that wrote a sentence inline would be a `what` no row
    /// measures; this reads the function's source for a multi-word
    /// string literal (a predicate name is one word) and reds on one.
    ///
    /// It reads LITERALS, not provenance: a `what` built some other way
    /// (a `format!`, a string from another module) passes it. A type
    /// would hold provenance, but `CensusUndecidable.what` is a
    /// `&'static str` payload, and the refusal standard keeps payloads
    /// as they are — accepted, and stated here.
    #[test]
    fn the_backstop_writes_no_sentence_of_its_own() {
        use test_utils::source::{Region, balanced_end, code_only, keeping};
        let src = include_str!("census.rs");
        let code = code_only(src);
        let at = code
            .find("\nfn sweep_cross_solid_backstop")
            .expect("the backstop is in this file");
        let open = at + code[at..].find('{').expect("the backstop's body");
        let close = balanced_end(&code, open).expect("the backstop's closing brace");
        // Every space marked before lexing, then only the literals
        // kept: a literal still holding a mark had a space in it — a
        // sentence, where a predicate name is one word.
        let marked = src[open..=close].replace(' ', "\u{1}");
        let literals = keeping(&marked, &[Region::Literal]);
        let sentences: Vec<String> = literals
            .split([' ', '\n'])
            .filter(|literal| literal.contains('\u{1}'))
            .map(|literal| literal.replace('\u{1}', " "))
            .collect();
        assert!(
            sentences.is_empty(),
            "the backstop writes a `what` inline; name an `Undecided` reason instead: \
             {sentences:#?}"
        );
    }

    /// **The touch analysis decides only through its doors** (S4 of
    /// `CONTACT-7`'s design). A levered reading of directions decides a
    /// verdict right wherever it is definite and wrong where its Zero is
    /// taken as one: the class five review rounds found one site at a
    /// time. So every verdict sign of the analysis comes through
    /// [`metric::sign`], whose [`metric::Distance`] is built only from a
    /// point and a plane. This reads the analysis's section of this file
    /// (from its banner to the backstop's header) and holds:
    /// - every `Margin` door and every `decide` call lies inside `mod
    ///   metric` (one `Margin::of`);
    /// - the classifier of unit normals is called only from `pairing`;
    /// - nothing reads a bare sign: no `sign_within`, no comparison of a
    ///   real against zero (`< T::zero()`, `> T::zero()`), no
    ///   `is_sign_negative`, `is_sign_positive`, `signum`, `copysign`,
    ///   `abs(`, `partial_cmp` or `total_cmp`, and no `classify_dihedral` (a levered
    ///   classifier of its own). No construction code in the section
    ///   needs one today, so the list has no exemption; one added for
    ///   code whose outcome only shrinks a piece must be named here;
    /// - `Distance` is built in exactly one place: its field is private,
    ///   `mod metric` holds exactly one `Self(` — inside `Distance::of` —
    ///   and no `Distance(`, and `of` is the module's only function
    ///   returning a `Distance`. So a product or a lever has no way to
    ///   become one, not even through a second constructor beside `of`.
    ///
    /// **What it cannot see.** `Distance::of` takes the plane's normal
    /// on trust: a normal scaled by a length is a lever spelled as a
    /// plane, and a point built as `p + (q − p)·k` is a product spelled
    /// as a point. Every caller in the section passes a face's outward
    /// normal or a normalized direction, and a real face point or a
    /// point interpolated on a face's edge; that is checked by reading,
    /// not by this row.
    #[test]
    fn the_touch_analysis_decides_only_through_its_doors() {
        use test_utils::source::{balanced_end, code_only};
        let src = include_str!("census.rs");
        let code = code_only(src);
        let start = src
            .find("// ---- The touch analysis: arm 2's clear, condition 3.")
            .expect("the touch analysis's banner");
        let end = start
            + src[start..]
                .find("\n/// **The conservative loudness backstop**")
                .expect("the backstop's header after it");
        let section = &code[start..end];
        let body = |anchor: &str| -> core::ops::Range<usize> {
            let at = section
                .find(anchor)
                .unwrap_or_else(|| panic!("the section holds `{anchor}`"));
            let open = at + section[at..].find('{').expect("its body");
            open..balanced_end(section, open).expect("its closing brace")
        };
        let metric = body("\nmod metric");
        let pairing = body("\nfn pairing");
        let outside = |needle: &str, allowed: &[&core::ops::Range<usize>]| -> Vec<String> {
            section
                .match_indices(needle)
                .map(|(i, _)| i)
                .filter(|i| !allowed.iter().any(|r| r.contains(i)))
                .map(|i| {
                    section[i.saturating_sub(60)..(i + 40).min(section.len())]
                        .trim()
                        .to_owned()
                })
                .collect()
        };
        let mut stray = Vec::new();
        for needle in ["Margin::", "decide("] {
            stray.extend(outside(needle, &[&metric]));
        }
        stray.extend(outside("classify_material_pairing", &[&pairing]));
        for needle in [
            "sign_within",
            "classify_dihedral",
            "< T::zero()",
            "> T::zero()",
            "is_sign_negative",
            "is_sign_positive",
            "signum",
            "partial_cmp",
            "total_cmp",
            "copysign",
            "abs(",
        ] {
            stray.extend(outside(needle, &[]));
        }
        assert!(
            stray.is_empty(),
            "a decision in the touch analysis outside its doors: {stray:#?}"
        );
        let count =
            |needle: &str, r: &core::ops::Range<usize>| section[r.clone()].matches(needle).count();
        assert_eq!(
            count("Margin::", &metric),
            1,
            "the metric door's one `Margin::of`"
        );
        assert_eq!(
            count("Margin::of(", &metric),
            1,
            "the metric door's one `Margin::of`"
        );
        let door = &section[metric.clone()];
        assert!(
            door.contains("struct Distance<T>(T);"),
            "`Distance`'s field is private"
        );
        let of = body("\n        pub(super) fn of(");
        assert!(
            metric.contains(&of.start),
            "`Distance::of` is in `mod metric`"
        );
        let builds: Vec<usize> = section
            .match_indices("Self(")
            .chain(section.match_indices("Distance("))
            .map(|(i, _)| i)
            .filter(|i| metric.contains(i))
            .collect();
        assert!(
            builds.len() == 1 && of.contains(&builds[0]),
            "`Distance` is built once, inside `Distance::of`: {} builds",
            builds.len()
        );
        assert_eq!(
            door.matches("-> Self").count() + door.matches("-> Distance").count(),
            1,
            "`Distance::of` is `mod metric`'s only function returning a `Distance`"
        );
    }

    /// Two overlapping opposed-sense wall sheets on one cylinder key.
    fn conformal_pair() -> (Body<f64>, FaceKey, FaceKey) {
        let mut body = Body::<f64>::new();
        let (w1, cyl) = unit_cyl_sheet(
            &mut body,
            None,
            (0.2, 1.6),
            (0.0, 1.0),
            true,
            Tol::witness(),
        );
        let (w2, _) = unit_cyl_sheet(
            &mut body,
            Some(cyl),
            (1.0, 2.4),
            (0.3, 0.7),
            false,
            Tol::witness(),
        );
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        (body, w1, w2)
    }

    /// **A `(vertex, edge)` record on a circle edge is witnessed by the
    /// curved lane**: a vertex off the rim's carrier reads stale, as it
    /// would against a line. Red when the arm skips the record or still
    /// refuses the circle as unsupported.
    #[test]
    fn a_vertex_on_edge_record_off_a_circle_edge_reads_stale() {
        let mut body = Body::<f64>::new();
        unit_cyl_sheet(
            &mut body,
            None,
            (0.2, 1.6),
            (0.0, 1.0),
            true,
            Tol::witness(),
        );
        let rim = body
            .edges()
            .map(|(e, _)| e)
            .find(|&e| {
                body.get_edge(e)
                    .and_then(|d| body.get_curve_geom(d.curve))
                    .and_then(|g| g.certified())
                    .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Circle { .. }))
            })
            .expect("the sheet has a rim arc");
        let far = body
            .vertices()
            .map(|(k, _)| k)
            .find(|&v| {
                let d = body.get_edge(rim).unwrap();
                v != body.get_half_edge(d.he_plus).unwrap().start
                    && v != body.get_half_edge(d.he_minus).unwrap().start
            })
            .expect("a vertex off the rim");
        let records: ContactRecords = [crate::Cited::new(
            crate::Contact::of(crate::Cell::Vertex(far), crate::Cell::Edge(rim)).unwrap(),
            crate::Cites::decided(0),
        )]
        .into_iter()
        .collect();
        let errors = census_and_certify(&body, &records, band(), Tol::witness(), None);
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::StaleContactDeclaration {
                    declaration: StaleDeclaration::VertexOnEdge { vertex, edge },
                } if (*vertex, *edge) == (far, rim)
            )),
            "{errors:?}"
        );
        assert!(
            !errors
                .iter()
                .any(|e| matches!(e, ValidationError::CensusUnsupported { .. })),
            "{errors:?}"
        );
    }

    #[test]
    fn the_conformal_arm_finds_an_unrecorded_pair_and_carries_the_finding() {
        let (body, w1, w2) = conformal_pair();
        let errors = census_and_certify(
            &body,
            &ContactRecords::default(),
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
        );
        let hit = errors
            .iter()
            .find_map(|e| match e {
                ValidationError::UnrecordedContact {
                    contact: CensusContact::ConformalPatch { finding },
                    ..
                } => Some(*finding),
                _ => None,
            })
            .expect("the finding-carrying refusal: {errors:?}");
        // The kernel finding names the pair and the class that would
        // verify — the recourse is a quotable declaration.
        let pair = (hit.pair.a.min(hit.pair.b), hit.pair.a.max(hit.pair.b));
        assert_eq!(pair, (w1.min(w2), w1.max(w2)));
        assert_eq!(hit.pair.class, crate::contact::ContactClass::Rest);
        assert_eq!(hit.verdict, crate::contact::ContactVerdict::Definite);
    }

    /// Every error these scaffold rows may legitimately still carry:
    /// the containment arm's solid-granular refusal.
    ///
    /// **Why it fires, exactly — and it is not about where the sheets
    /// are.** Each `cyl_sheet` is its own `mvfs` seed, so a two-sheet
    /// fixture is a two-INSTANCE body; and each seed FACE keeps the
    /// mvfs NURBS placeholder, whose control net is poison. A solid
    /// carrying a face with no claimable box can never be the container
    /// ([`sweep_cross_solid_backstop`] arm 2), so both directions refuse
    /// with *"a surface kind with no cheap sound box leaves the
    /// containing instance's extent unclaimable"*. The refusal is
    /// therefore position-independent: moving one sheet a kilometre
    /// away produces the identical pair of errors, because no extent
    /// comparison happens at all. Do not read these rows as the arm
    /// measuring the sheets.
    ///
    /// These fixtures sit below tier 3's closed-body bar and call
    /// `census_and_certify` directly, which is the only door that
    /// reaches a placeholder face; through `validate_pseudomanifold` the
    /// state cannot be minted.
    ///
    /// The filter is by ENTITY GRANULARITY rather than by count, so a
    /// face-granular finding — which is what these rows are about —
    /// can never hide inside the allowance. What it does admit,
    /// deliberately and without bound, is any number of `Solid`×`Solid`
    /// refusals of EITHER of arm 2's two `what` strings. That width is
    /// the price of not pinning a `what` string in a row about the
    /// conformal doors; the paragraph above is what keeps it honest,
    /// and it is a paragraph, not a check.
    fn face_findings(errors: &[ValidationError]) -> Vec<&ValidationError> {
        errors
            .iter()
            .filter(|e| {
                !matches!(
                    e,
                    ValidationError::CensusUndecidable {
                        a: EntityId::Solid(_),
                        b: EntityId::Solid(_),
                        ..
                    }
                )
            })
            .collect()
    }

    /// **What the containment arm actually says about these scaffolds,
    /// and that it is not about where they are.**
    ///
    /// Each `cyl_sheet` keeps its `mvfs` NURBS placeholder on the seed
    /// face. A placeholder net is poison, so the solid has no claimable
    /// extent and can never be the container: arm 2 must reach its
    /// TYPED refusal for that case. Folding the poison net instead
    /// yields `Some((NaN, NaN))`, every margin decides neither sign,
    /// and the arm falls out at its in-band refusal having compared no
    /// geometry — which reads, in a suite, exactly like the arm
    /// measuring two overlapping sheets.
    ///
    /// The second half is what makes that unmistakable: a sheet a
    /// KILOMETRE away produces the byte-identical refusal. If this row
    /// ever fails there, an extent comparison has started happening and
    /// the `face_findings` allowance above needs re-reading.
    #[test]
    fn a_placeholder_seed_leaves_the_containing_extent_unclaimable() {
        let refusals = |z0: f64, z1: f64| -> Vec<String> {
            let mut body = Body::<f64>::new();
            let (_w1, cyl) = unit_cyl_sheet(
                &mut body,
                None,
                (0.2, 1.6),
                (0.0, 1.0),
                true,
                Tol::witness(),
            );
            let (_w2, _) = unit_cyl_sheet(
                &mut body,
                Some(cyl),
                (1.0, 2.4),
                (z0, z1),
                false,
                Tol::witness(),
            );
            crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
            census_and_certify(
                &body,
                &ContactRecords::default(),
                band(),
                Tol::witness(),
                Some(RegionLane::certified()),
            )
            .into_iter()
            .filter_map(|e| match e {
                ValidationError::CensusUndecidable {
                    a: EntityId::Solid(a),
                    b: EntityId::Solid(b),
                    what,
                } => Some(format!("{a:?}~{b:?}: {what}")),
                _ => None,
            })
            .collect()
        };
        let near = refusals(0.3, 0.7);
        assert!(
            !near.is_empty()
                && near
                    .iter()
                    .all(|r| r.ends_with(Undecided::Unclaimable.what())),
            "the placeholder seed must reach the TYPED refusal, not the in-band \
             fallout of a NaN box: {near:?}"
        );
        assert_eq!(
            near,
            refusals(1000.3, 1000.7),
            "the refusal is about an unclaimable extent, so moving a sheet a \
             kilometre away must not change it"
        );
    }

    /// **A container carrying a face kind the material test does not
    /// serve.** A 3 m box whose far wall (`x = 0`) is a described
    /// NURBS net, with a 0.6 m box floating inside: arm 1 clears every
    /// pair the spline face is in (its reach box is two metres from
    /// the part), the extent gate cannot separate the pair, and the
    /// material test's pre-pass meets the spline face and refuses
    /// `KindUnsupported` — which arm 2 reports as `CensusUndecidable`
    /// naming the cause, on BOTH orderings (the part's own material
    /// test of the box is the reverse ordering, and it clears on its
    /// gate: the box's hull is not inside the part's reach). Through
    /// `census_and_certify` directly, because the public door refuses
    /// the re-described wall at tier 3 first (`DescriptionNotAdjacent`
    /// on its edges, or the M7-8 edge lane's own certification); this
    /// is the door below that bar.
    #[test]
    fn an_unserved_face_kind_on_the_container_refuses_naming_the_cause() {
        use crate::euler::FaceSurface;
        use crate::splitting::reassembly::quad_prism;
        let tol = Tol::witness();
        let mut body = quad_prism(&[(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)], 3.0, tol);
        let far_wall = body
            .faces()
            .find(|&(f, _)| {
                let face = body.get_face(f).unwrap();
                face_cycles(&body, face)
                    .filter_map(Result::ok)
                    .flatten()
                    .all(|he| {
                        let v = body.half_edges.get(he).unwrap().start;
                        body.points[body.vertices[v].point].x.abs() < 1e-12
                    })
            })
            .map(|(f, _)| f)
            .unwrap();
        let k =
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let ticks = [-1.0, 1.5, 4.0];
        let (mut control, mut weights) = (Vec::new(), Vec::new());
        for &y in &ticks {
            for &z in &ticks {
                control.push(Point3::new(0.0, y, z));
                weights.push(1.0);
            }
        }
        let net = geom::NurbsSurface::new(k.clone(), k, control, weights).unwrap();
        assert!(!net.is_placeholder());
        body.set_face_surface(
            far_wall,
            FaceSurface::New {
                surface: geom::Surface::Nurbs(std::sync::Arc::new(net)),
                sense: true,
            },
        )
        .unwrap();
        // The part lifted off the floor: z ∈ [1.2, 1.8].
        let part = crate::transform_rigid(
            &quad_prism(&[(1.2, 1.2), (1.8, 1.2), (1.8, 1.8), (1.2, 1.8)], 0.6, tol),
            &geom_core::RigidMotion::translation(Vec3::new(0.0, 0.0, 1.2)).unwrap(),
            tol,
        )
        .unwrap();
        crate::instance::graft_disjoint(&mut body, &part).unwrap();
        let errors = census_and_certify(
            &body,
            &ContactRecords::default(),
            band(),
            tol,
            Some(RegionLane::certified()),
        );
        let whats: Vec<&str> = errors
            .iter()
            .filter_map(|e| match e {
                ValidationError::CensusUndecidable {
                    a: EntityId::Solid(_),
                    b: EntityId::Solid(_),
                    what,
                } => Some(*what),
                _ => None,
            })
            .collect();
        assert_eq!(whats.len(), 1, "{errors:?}");
        assert!(
            whats[0] == Undecided::FaceKindUnsupported.what(),
            "{whats:?}"
        );
        assert!(
            !errors
                .iter()
                .any(|e| matches!(e, ValidationError::InstanceInterference { .. })),
            "{errors:?}"
        );
    }

    #[test]
    fn a_patch_record_backs_the_pair_and_confirms_through_both_doors() {
        let (body, w1, w2) = conformal_pair();
        let mut records = ContactRecords::default();
        records.push(crate::Cited::new(
            crate::Contact::of(crate::Cell::Face(w1), crate::Cell::Face(w2)).unwrap(),
            crate::Cites::decided(0),
        ));
        let errors = census_and_certify(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
        );
        let findings = face_findings(&errors);
        assert!(
            findings.is_empty(),
            "the declared conformal patch certifies: {findings:?} (all: {errors:?})"
        );
    }

    #[test]
    fn a_disjoint_patch_record_is_stale_typed() {
        let mut body = Body::<f64>::new();
        let (w1, cyl) = unit_cyl_sheet(
            &mut body,
            None,
            (0.2, 1.6),
            (0.0, 1.0),
            true,
            Tol::witness(),
        );
        let (w3, _) = unit_cyl_sheet(
            &mut body,
            Some(cyl),
            (3.0, 4.0),
            (0.0, 1.0),
            false,
            Tol::witness(),
        );
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        let mut records = ContactRecords::default();
        records.push(crate::Cited::new(
            crate::Contact::of(crate::Cell::Face(w1), crate::Cell::Face(w3)).unwrap(),
            crate::Cites::decided(0),
        ));
        let errors = census_and_certify(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
        );
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::StaleContactDeclaration { .. })),
            "overlap Empty ⇒ stale (C3's letter): {errors:?}"
        );
    }

    // ================= The `None` path, observed =================

    /// The sentence [`ValidationError::CensusLaneUnsupported`] renders
    /// — for any pair, since it names none — so the `None` rows below
    /// pin the `Display` and not only the variant.
    const LANE_UNSUPPORTED_SENTENCE: &str = "a pair of faces was not examined, \
         because this structural check holds no certified chart-overlap lane at any scalar; \
         nothing was decided about the geometry. Recourse: run the certified check \
         (validate_pseudomanifold, or validate_pseudomanifold_certificate for a certificate) \
         at a certifying scalar";

    /// The one [`ValidationError::CensusLaneUnsupported`] in `errors`,
    /// with its subject and `Display` pinned to `pair`.
    fn the_one_lane_refusal(errors: &[ValidationError], pair: (FaceKey, FaceKey)) {
        let lane: Vec<&ValidationError> = errors
            .iter()
            .filter(|e| matches!(e, ValidationError::CensusLaneUnsupported { .. }))
            .collect();
        assert_eq!(
            lane.len(),
            1,
            "exactly one lane refusal, for the one pair the arm could not examine: {errors:?}"
        );
        assert_eq!(
            *lane[0],
            ValidationError::CensusLaneUnsupported {
                subject: CensusSubject::FacePair(pair.0, pair.1),
            }
        );
        assert_eq!(lane[0].to_string(), LANE_UNSUPPORTED_SENTENCE);
    }

    /// **The `None` path at the conformal face-pair arm, observed.** The
    /// same conformal pair the arm finds UNDECLARED with the region door
    /// in hand is, with the pass holding `None`, exactly one
    /// [`ValidationError::CensusLaneUnsupported`] naming the pair — the
    /// typed refusal, never a silent skip, and never the arm's unrecorded
    /// finding (it examined nothing). What the pair's edges say is pass
    /// 5′'s, which needs no region door: the second sheet's two rim arcs
    /// cross the first's seam line, and with no record either crossing is
    /// unrecorded.
    #[test]
    fn a_pass_holding_no_region_door_refuses_the_conformal_pair_typed() {
        let (body, w1, w2) = conformal_pair();
        let errors = census_and_certify(
            &body,
            &ContactRecords::default(),
            band(),
            Tol::witness(),
            None,
        );
        let (crossings, findings): (Vec<_>, Vec<_>) =
            face_findings(&errors).into_iter().partition(|e| {
                matches!(
                    e,
                    ValidationError::UnrecordedContact {
                        contact: CensusContact::EdgeEdgeCross { side: None, .. },
                        ..
                    }
                )
            });
        assert_eq!(
            findings.len(),
            1,
            "the arm's whole verdict on the pair is the one refusal: {errors:?}"
        );
        the_one_lane_refusal(&errors, (w1, w2));
        assert_eq!(
            crossings.len(),
            2,
            "pass 5′ finds the rims crossing the seam: {errors:?}"
        );
    }

    /// **The `None` path at the confirm pass, observed, and the crossing
    /// rung's `false` with it.** The straddle seat's declared pair (its
    /// one patch record) certifies through both doors with the region
    /// door in hand and backs the seat's two edge crossings; with the
    /// pass holding `None` the record is exactly one
    /// [`ValidationError::CensusLaneUnsupported`] naming the pair, and
    /// the two crossings are the hard findings the unrecorded seat has
    /// — the backing consult folded to `false`, so the declaration
    /// backs nothing, exactly as a scalar with no door would see it.
    #[test]
    fn a_pass_holding_no_region_door_refuses_the_declared_pair_typed_and_backs_no_crossing() {
        let seat = crate::test_support_fixtures::straddle_seat(Tol::witness());
        let pair = (seat.post_top, seat.shelf_bottom);
        let mut records = ContactRecords::default();
        records.push(crate::Cited::new(
            crate::Contact::of(crate::Cell::Face(pair.0), crate::Cell::Face(pair.1)).unwrap(),
            crate::Cites::decided(0),
        ));
        let crossings = |errors: &[ValidationError]| -> Vec<String> {
            errors
                .iter()
                .filter_map(|e| match e {
                    ValidationError::UnrecordedContact {
                        contact: contact @ CensusContact::EdgeEdgeCross { .. },
                        witness,
                        ..
                    } => Some(format!("{contact:?} at {witness}")),
                    _ => None,
                })
                .collect()
        };

        let with = census_and_certify(
            &seat.body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
        );
        assert!(
            with.is_empty(),
            "the declared seat certifies and backs both crossings: {with:?}"
        );

        let bare = census_and_certify(
            &seat.body,
            &ContactRecords::default(),
            band(),
            Tol::witness(),
            None,
        );
        assert!(
            !crossings(&bare).is_empty(),
            "the unrecorded seat's crossings are hard findings: {bare:?}"
        );
        assert!(
            !bare
                .iter()
                .any(|e| matches!(e, ValidationError::CensusLaneUnsupported { .. })),
            "no declaration, no consult, no lane refusal: {bare:?}"
        );

        let without = census_and_certify(&seat.body, &records, band(), Tol::witness(), None);
        the_one_lane_refusal(&without, pair);
        assert_eq!(
            crossings(&without),
            crossings(&bare),
            "the declared pair backs nothing when the consult holds no door: {without:?}"
        );
    }

    /// **The backing consult's fold, at the consult itself**: the
    /// straddle seat's declared pair verifies with the region door in
    /// hand (Door 1's rest verdict, Door 2's positive area) and answers
    /// `false` without one — the silent half of the `None` path, whose
    /// loud twin is the confirm pass's refusal in the row above.
    #[test]
    fn the_backing_consult_holding_no_region_door_answers_false() {
        let seat = crate::test_support_fixtures::straddle_seat(Tol::witness());
        assert!(
            pair_region_verified(
                &seat.body,
                seat.post_top,
                seat.shelf_bottom,
                band(),
                Some(RegionLane::certified()),
            ),
            "the declared seat's pair verifies through both doors"
        );
        assert!(
            !pair_region_verified(&seat.body, seat.post_top, seat.shelf_bottom, band(), None),
            "no door, no verification"
        );
    }

    /// **The crossing rung reads a declared face's surface as a link.**
    /// On the declared straddle seat the rung backs the cap side edge's
    /// crossing with the shelf edge; with the cap's surface record torn
    /// from the arena after the snapshot, the rung panics naming the
    /// face's `surface` link rather than reading the pair as absent.
    #[test]
    #[should_panic(expected = "'s surface names")]
    fn the_crossing_rung_panics_on_a_torn_declared_surface() {
        let mut seat = crate::test_support_fixtures::straddle_seat(Tol::witness());
        let mut records = ContactRecords::default();
        records.push(crate::Cited::new(
            crate::Contact::of(
                crate::Cell::Face(seat.post_top),
                crate::Cell::Face(seat.shelf_bottom),
            )
            .unwrap(),
            crate::Cites::decided(0),
        ));
        let geo = snapshot(&seat.body);
        let declared = Declared::index(&geo, &records);
        let on = |e: &&EdgeGeo<f64>, pick: fn(Point3<f64>) -> bool| {
            [e.p0, e.p1].into_iter().all(|p| p.z == 0.5 && pick(p))
        };
        let ea = geo
            .edges
            .iter()
            .find(|e| on(e, |p| p.x == 0.30))
            .expect("the cap's x = 0.30 top edge");
        let eb = geo
            .edges
            .iter()
            .find(|e| on(e, |p| p.y == 0.30))
            .expect("the shelf's y = 0.30 underside edge");
        let q = Point3::new(0.30, 0.30, 0.5);
        let mut errors = Vec::new();
        let region = Some(RegionLane::certified());
        let intact = ee_cross_backed(
            &seat.body,
            &geo,
            &declared,
            ea,
            eb,
            q,
            band(),
            region,
            &mut errors,
        );
        assert!(
            matches!(intact, CrossingBacking::Backed) && errors.is_empty(),
            "the intact seat backs the crossing: {errors:?}"
        );
        let surface = seat.body.get_face(seat.post_top).expect("the cap").surface;
        seat.body.surfaces.remove(surface).expect("its surface");
        ee_cross_backed(
            &seat.body,
            &geo,
            &declared,
            ea,
            eb,
            q,
            band(),
            region,
            &mut errors,
        );
    }

    /// **The consult on the mate9 Door-2 isolator pair** (kitty-corner,
    /// zero-area overlap) answers `false` with the door in hand too:
    /// Door 2 never answers `PositiveArea` on it, so that body cannot
    /// carry the `true`/`false` pin the row above takes on the straddle
    /// seat's declared pair, and the `None` fold is invisible on it.
    #[test]
    fn the_isolator_pair_answers_false_with_the_door_in_hand_too() {
        let tol = Tol::witness();
        let post: crate::test_support_fixtures::Prism<f64> = crate::test_support_fixtures::prism_z(
            &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
            0.0,
            0.5,
            tol,
        );
        let shelf: crate::test_support_fixtures::Prism<f64> = crate::test_support_fixtures::prism_z(
            &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
            0.5,
            0.54,
            tol,
        );
        let block: crate::test_support_fixtures::Prism<f64> = crate::test_support_fixtures::prism_z(
            &[(0.10, 0.30), (0.30, 0.30), (0.30, 0.50), (0.10, 0.50)],
            0.5,
            0.54,
            tol,
        );
        let post_top = post.top_face;
        let mut body = post.body;
        let _ = crate::graft_disjoint_all_keyed(&mut body, &shelf.body).unwrap();
        let bkeys = crate::graft_disjoint_all_keyed(&mut body, &block.body).unwrap();
        let block_bottom = bkeys.face(block.bottom_face).unwrap();
        for region in [Some(RegionLane::certified()), None] {
            assert!(
                !pair_region_verified(&body, post_top, block_bottom, band(), region),
                "the isolator pair answers false whether or not the door is in hand"
            );
        }
    }

    /// **A declared pair whose planes cross definitely transverse at a
    /// crossing does not hold it, however near both edges run to both
    /// planes.** Two blocks in one body: one's top face in `z = 0`, the
    /// other's bottom face that plane turned by `α` about the x axis, so
    /// the two planes meet along the x axis. Each face's crossing edge
    /// runs through the origin at an angle `φ` to that axis, so its ends
    /// stand `φ·sin α = ε/2` off the other plane: both edges lie in both
    /// planes at the band, the two edges cross past the band, and the
    /// dihedral is past it by orders. The pair is declared. Its carrier
    /// screens (the dihedral last) refuse it silently, so the crossing is
    /// the plain hard finding, nothing escalates for it, and the confirm
    /// pass reports the declaration contradicted. Three poses per row,
    /// `sin α` scaled by `2/K` so every pose holds at any `K`.
    #[test]
    fn a_transverse_declared_pair_holds_no_crossing_its_edges_hug() {
        use crate::test_support_fixtures::{
            FaceGeometry, describe_as_intersections, identity_map, prism_ops,
        };
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let k = band.escalate() / band.zero();
        for f in [0.25, 0.5, 0.75] {
            let sin = f * 2.0 / k;
            let cos = (1.0 - sin * sin).sqrt();
            let phi = band.zero() / (2.0 * sin);
            let turn =
                move |x: f64, y: f64, z: f64| Point3::new(x, y * cos - z * sin, y * sin + z * cos);
            let mut body = Body::<f64>::new();
            let low = prism_ops(
                &mut body,
                &[(-1.0, -phi), (1.0, phi), (1.0, 1.0), (-1.0, 1.0)],
                (-1.0, 0.0),
                identity_map,
                FaceGeometry::Certified,
                tol,
            );
            let high = prism_ops(
                &mut body,
                &[(-1.0, phi), (-1.0, -1.0), (1.0, -1.0), (1.0, -phi)],
                (0.0, 1.0),
                turn,
                FaceGeometry::Certified,
                tol,
            );
            describe_as_intersections(&mut body, tol);
            let (fa, fb) = (low.seed.face, high.bottom.face);
            let mut records = ContactRecords::default();
            records.push(crate::Cited::new(
                crate::Contact::of(crate::Cell::Face(fa), crate::Cell::Face(fb)).unwrap(),
                crate::Cites::decided(0),
            ));
            let what = format!("sin α {sin}, φ {phi:e}");
            let geo = snapshot(&body);
            let edge = |a: Point3<f64>, b: Point3<f64>| {
                geo.edges
                    .iter()
                    .find(|e| {
                        let far = e.p1;
                        [(a, b), (b, a)].iter().any(|&(p, q)| {
                            (e.p0 - p).norm() == 0.0 && (far - q).norm() <= 1e-3 * band.zero()
                        })
                    })
                    .unwrap_or_else(|| panic!("{what}: the edge {a:?} → {b:?}"))
            };
            let ea = edge(Point3::new(-1.0, -phi, 0.0), Point3::new(1.0, phi, 0.0));
            let eb = edge(turn(-1.0, phi, 0.0), turn(1.0, -phi, 0.0));
            let mut errors = Vec::new();
            let backing = ee_cross_backed(
                &body,
                &geo,
                &Declared::index(&geo, &records),
                ea,
                eb,
                Point3::origin(),
                band,
                Some(RegionLane::certified()),
                &mut errors,
            );
            assert!(
                matches!(backing, CrossingBacking::Unanswered) && errors.is_empty(),
                "{what}: the transverse pair answers for nothing and raises nothing: {errors:?}"
            );
            let errors = crate::validate_pseudomanifold(&body, &records, tol).unwrap_err();
            let pair = [ea.key, eb.key];
            assert!(
                errors.iter().any(|e| matches!(e,
                    ValidationError::UnrecordedContact {
                        contact: CensusContact::EdgeEdgeCross { a, b, side: None },
                        ..
                    } if [*a, *b] == pair || [*b, *a] == pair)),
                "{what}: the crossing is the plain hard finding: {errors:?}"
            );
            assert!(
                !errors.iter().any(|e| matches!(e,
                    ValidationError::CensusEscalated { cause }
                        if cause.predicate == Some("material_wedge_side"))),
                "{what}: nothing escalates for the crossing: {errors:?}"
            );
            assert!(
                errors.iter().any(|e| matches!(e,
                    ValidationError::ContactContradicted { declaration, .. }
                        if [declaration.a, declaration.b] == [fa, fb]
                            || [declaration.a, declaration.b] == [fb, fa])),
                "{what}: the confirm pass reports the declaration: {errors:?}"
            );
        }
    }

    /// **The band's own parting no longer parts an arc's end from its
    /// corner.** At rest every carrier end lies at its corner, and the
    /// boundary reading puts a point at an arc's end only on its raw chord
    /// to the end's point ([`geom_brep::curve_pair::Span::place`]), the
    /// same distance the vertex pass reads to the corner. So the band
    /// `(ε, 1.25ε)`, on a half-disc cap the public operators built, reads
    /// a point `≈ 1.34·zero` from the corner clear of the corner and of the
    /// end alike. Its foot lies within the zero band of the end along the
    /// arc, which the side of the end cannot place beside a chord past the
    /// band, so it escalates on that straddle and the census carries the
    /// escalation.
    #[test]
    fn the_band_parting_an_arc_end_from_its_corner_escalates() {
        use crate::test_support_fixtures::half_disc_cap;
        let tol = Tol::witness();
        let zero = Band::linear(tol).unwrap().zero();
        let (body, cap, end) = half_disc_cap(0.0, tol);
        let q = end + Vec3::new(-0.95 * zero, 0.95 * zero, 0.0);
        let narrow = Band::new(zero, 1.25 * zero).unwrap();
        let read = crate::boolean::contfp(&body, cap, Vec3::unit_z(), q, narrow);
        let Err(ContainError::Escalated { diag, .. }) = read.clone() else {
            panic!("the band's own parting: {read:?}");
        };
        assert_eq!(diag.predicate, Some("curve_pair_span_end_straddle"));
        let mut errors = Vec::new();
        assert_eq!(read_containment(cap, read, &mut errors), None);
        assert_eq!(
            errors,
            vec![ValidationError::CensusEscalated { cause: diag }]
        );
    }

    /// **An arc's end read off both corners at the census is a defect**:
    /// tier 3, which the census runs behind, pins every carrier end within
    /// the run's zero band of its corner, so an end the census reads off
    /// both is the kernel's two readings of one end parting. A certified
    /// cap read at a band finer than the run's stands in for that parting
    /// here; the census reports it as its own finding, with the defect
    /// ending, and never as a face it "could not check".
    #[test]
    fn an_arc_end_off_both_corners_is_a_census_defect() {
        use crate::test_support_fixtures::half_disc_cap;
        let tol = Tol::witness();
        let off = Band::linear(tol).unwrap().zero() / 2.0;
        let (body, cap, end) = half_disc_cap(off, tol);
        let fine = Band::new(off / 50.0, off / 5.0).unwrap();
        let read = crate::boolean::contfp(&body, cap, Vec3::unit_z(), end, fine);
        let mut errors = Vec::new();
        assert_eq!(read_containment(cap, read, &mut errors), None);
        let [ValidationError::ArcEndOffCorner { face, gaps }] = errors.as_slice() else {
            panic!("the census's own finding: {errors:?}");
        };
        assert!(*face == cap && gaps.off_corner(), "{errors:?}");
        let text = errors[0].to_string();
        assert!(
            text.ends_with(geom_core::KERNEL_OR_FILE_DEFECT_ENDING)
                && !text.contains("could not be checked"),
            "{text}"
        );
    }

    /// **A far arc at `K` near one reads its end where certification
    /// pinned it** (the review's witness). A cap on a circle of radius
    /// `98765.4` at ε = 1e-9 (scaled with ε), centre and axis generic,
    /// over `[0, 2.5]`, its end corner certified `0.9905·zero` off the
    /// carrier's end, along the direction from the end the conic's own
    /// frame computes to the one `Curve3::eval` computes; the point
    /// `0.95·zero` back along the arc from the end and `0.3·zero` off it.
    /// The two ends part by rounding past `(K − 1)·zero` at `K = 1.001`,
    /// so a boundary pass reading the frame's end read the end off its
    /// corner. It reads `eval`'s, the point certification pinned: at
    /// `K = 1.001` and `K = 1.2` every end lies at a corner, read as the
    /// band's own parting or, where the point's chord to the end falls in
    /// the band's hair-wide gap, as the end reading's escalation, and the
    /// census reports no defect.
    #[test]
    fn a_far_arc_at_k_near_one_reads_its_end_where_certification_pinned_it() {
        use crate::test_support_fixtures::arc_cap;
        use geom::Curve3;
        let tol = Tol::witness();
        let zero = Band::linear(tol).unwrap().zero();
        let axis = Vec3::new(0.3, -0.2, 1.0).normalize();
        let u_ref = axis.cross(Vec3::new(0.6, 0.7, 0.1)).normalize();
        let v_ref = axis.cross(u_ref);
        let t1 = 2.5_f64;
        let (s, c) = t1.sin_cos();
        // The review's radius at ε = 1e-9, scaled with ε, and the first of
        // a few nearby where the frame and `eval` part by rounding past
        // `(K − 1)·zero` at `K = 1.001`: the witness is sharp at every row.
        let circle = |r: f64| {
            let center = Point3::new(0.31 * r, -0.17 * r, 0.05 * r);
            let carrier = Curve3::Circle {
                center,
                axis,
                radius: r,
                u_ref,
            };
            let end = carrier.eval(t1);
            let frame_end = center + u_ref * (r * c) + v_ref * (r * s);
            (carrier, center, end, end - frame_end)
        };
        let (carrier, center, end, parting) = [1.0, 1.07, 1.13, 0.91, 0.83, 1.29]
            .into_iter()
            .map(|m| circle(98765.4 * (zero / 1e-9) * m))
            .find(|(.., parting)| parting.norm() > 0.001 * zero)
            .expect("a radius where the frame's end and eval's part past (K - 1)·zero");
        let r = (end - center).norm();
        let away = parting / parting.norm();
        let parting = parting.norm();
        let (body, cap) = arc_cap(carrier, (0.0, t1), away * (0.9905 * zero), tol);
        let tangent = u_ref * (-s) + v_ref * c;
        let radial = (end - center) / r;
        let q = end - tangent * (0.95 * zero) + radial * (0.3 * zero);
        for k in [1.001, 1.2] {
            let band = Band::new(zero, k * zero).unwrap();
            let read = crate::boolean::contfp(&body, cap, axis, q, band);
            // The point's chord to the end sits within a hair of the zero
            // band, which a band this narrow reads either way as the
            // rounding falls: the finding with every end at a corner, or
            // the end reading's escalation. Never an end off its corner.
            assert!(
                matches!(&read, Err(ContainError::ArcEndClearOfCorners(g)) if !g.off_corner())
                    || matches!(&read, Err(ContainError::Escalated { .. })),
                "K {k}: the end lies at its corner, and only the band parts the passes (frame \
                 and eval part by {parting:e}): {read:?}"
            );
            let mut errors = Vec::new();
            read_containment(cap, read, &mut errors);
            assert!(
                !errors
                    .iter()
                    .any(|e| matches!(e, ValidationError::ArcEndOffCorner { .. })),
                "K {k}: the census reports no defect: {errors:?}"
            );
        }
    }

    // ================= R1 review probes (m9-2b-r1) =================

    /// R1 probe (claim 2): an IN-BAND sliver overlap must escalate at
    /// BOTH doors — the conformal arm (unrecorded direction) and the
    /// patch-record certifier (declared direction) — never decide.
    #[test]
    fn r1_probe_in_band_sliver_overlap_escalates_both_directions() {
        let mut body = Body::<f64>::new();
        // Overlap region u ∈ [0.4, 1.4] × z ∈ [0.5, 0.5 + 5e-9]:
        // mean width ≈ 5e-9 m, inside Band{1e-9, 1e-8}.
        let (w1, cyl) = unit_cyl_sheet(
            &mut body,
            None,
            (0.2, 1.6),
            (0.0, 0.5 + 5e-9),
            true,
            Tol::witness(),
        );
        let (w2, _) = unit_cyl_sheet(
            &mut body,
            Some(cyl),
            (0.4, 1.4),
            (0.5, 1.0),
            false,
            Tol::witness(),
        );
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        let arm = census_and_certify(
            &body,
            &ContactRecords::default(),
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
        );
        assert!(
            arm.iter()
                .any(|e| matches!(e, ValidationError::CensusEscalated { .. })),
            "an in-band sliver must escalate at the arm: {arm:?}"
        );
        assert!(
            !arm.iter().any(|e| matches!(
                e,
                ValidationError::UnrecordedContact {
                    contact: CensusContact::ConformalPatch { .. },
                    ..
                }
            )),
            "an in-band sliver must never DECIDE unrecorded: {arm:?}"
        );
        let mut records = ContactRecords::default();
        records.push(crate::Cited::new(
            crate::Contact::of(crate::Cell::Face(w1), crate::Cell::Face(w2)).unwrap(),
            crate::Cites::decided(0),
        ));
        let cert = census_and_certify(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
        );
        assert!(
            cert.iter()
                .any(|e| matches!(e, ValidationError::CensusEscalated { .. })),
            "the record certifier must escalate in band: {cert:?}"
        );
        assert!(
            !cert
                .iter()
                .any(|e| matches!(e, ValidationError::StaleContactDeclaration { .. })),
            "in-band is escalation, never stale: {cert:?}"
        );
    }

    /// R1 probe (claim 1): a same-key pair AUTHORED one period apart
    /// (u-windows on the "next branch") cannot slip the sweep — the
    /// pcurve mint normalizes the branch, the windows overlap, and
    /// the arm reports the unrecorded conformal contact; the same
    /// pair backed by its patch record certifies clean. (The probe
    /// originally expected the seam-branch typed refusal here; the
    /// mint normalizes first, so the refusal is unreachable from
    /// euler-authored geometry — a stronger answer, pinned.)
    #[test]
    fn r1_probe_next_branch_windows_still_find_the_overlap() {
        let tau = core::f64::consts::TAU;
        let mut body = Body::<f64>::new();
        let (w1, cyl) = unit_cyl_sheet(
            &mut body,
            None,
            (0.2, 1.6),
            (0.0, 1.0),
            true,
            Tol::witness(),
        );
        // Same locus, next periodic branch; u-nested and z-nested so
        // no strut/vertex coincidences muddy the face-pair question.
        let (w2, _) = unit_cyl_sheet(
            &mut body,
            Some(cyl),
            (0.5 + tau, 1.2 + tau),
            (0.3, 0.7),
            false,
            Tol::witness(),
        );
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        let arm = census_and_certify(
            &body,
            &ContactRecords::default(),
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
        );
        assert!(
            arm.iter().any(|e| matches!(
                e,
                ValidationError::UnrecordedContact {
                    contact: CensusContact::ConformalPatch { .. },
                    ..
                }
            )),
            "the next-branch authoring must not evade the arm: {arm:?}"
        );
        let mut records = ContactRecords::default();
        records.push(crate::Cited::new(
            crate::Contact::of(crate::Cell::Face(w1), crate::Cell::Face(w2)).unwrap(),
            crate::Cites::decided(0),
        ));
        let cert = census_and_certify(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
        );
        let findings = face_findings(&cert);
        assert!(
            findings.is_empty(),
            "the backed next-branch pair certifies: {findings:?} (all: {cert:?})"
        );
    }

    #[test]
    fn an_aligned_same_sense_patch_record_is_contradicted() {
        // Same key, SAME sense: aligned coincidence is containment or
        // flush material, never contact (C1) — the record lies.
        let mut body = Body::<f64>::new();
        let (w1, cyl) = unit_cyl_sheet(
            &mut body,
            None,
            (0.2, 1.6),
            (0.0, 1.0),
            true,
            Tol::witness(),
        );
        let (w2, _) = unit_cyl_sheet(
            &mut body,
            Some(cyl),
            (1.0, 2.4),
            (0.3, 0.7),
            true,
            Tol::witness(),
        );
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        let mut records = ContactRecords::default();
        records.push(crate::Cited::new(
            crate::Contact::of(crate::Cell::Face(w1), crate::Cell::Face(w2)).unwrap(),
            crate::Cites::decided(0),
        ));
        let errors = census_and_certify(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
        );
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::ContactContradicted { .. })),
            "{errors:?}"
        );
        // And the ARM stays quiet on the aligned pair: SameOriented
        // is flush, not a conformal candidate.
        let arm_only = census_and_certify(
            &body,
            &ContactRecords::default(),
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
        );
        assert!(
            !arm_only.iter().any(|e| matches!(
                e,
                ValidationError::UnrecordedContact {
                    contact: CensusContact::ConformalPatch { .. },
                    ..
                }
            )),
            "{arm_only:?}"
        );
    }

    // ============ MATE-5: the cross-description cylinder rows ==========
    //
    // The same wall-sheet fixtures at the CENSUS door, but the second
    // sheet authored in a DIVERGENT description of the same cylinder
    // locus (origin a quarter up the axis, axis direction opposed,
    // seam rotated 0.7 rad, its own surface key): the pair flows
    // through the certified-ε enclosure arm.

    /// A wall sheet over the DIVERGENT description of the unit
    /// cylinder: `θ_world = 0.7 − u_B`, `z_world = 0.25 − v_B`. Takes
    /// the WORLD window and converts.
    ///
    /// The frame is [`CylFrame::opposed`] at seam `0.7` — the same
    /// locus as [`unit_cyl_sheet`]'s and not one field in common with
    /// it — so the pair this seeds disagrees on every field of the
    /// description while naming one cylinder.
    fn cyl_sheet_b(
        body: &mut Body<f64>,
        th0: f64,
        th1: f64,
        z0: f64,
        z1: f64,
        sense: bool,
    ) -> FaceKey {
        let d = 0.7_f64;
        let (u0, u1, v0, v1) = (d - th1, d - th0, 0.25 - z1, 0.25 - z0);
        let (face, _) = cyl_wall_sheet_keyed(
            body,
            CylFrame::opposed(d),
            CylKey::Bare,
            (u0, u1),
            (v0, v1),
            Tol::witness(),
        );
        body.set_face_sense(face, sense).unwrap();
        face
    }

    /// One arena, two wall sheets on DIVERGENT descriptions of one
    /// cylinder, opposed senses — the seat, at the
    /// census's own door.
    fn cross_description_pair(
        th0: f64,
        th1: f64,
        z0: f64,
        z1: f64,
    ) -> (Body<f64>, FaceKey, FaceKey) {
        let mut body = Body::<f64>::new();
        let (w1, _) = unit_cyl_sheet(
            &mut body,
            None,
            (0.2, 1.6),
            (0.0, 1.0),
            true,
            Tol::witness(),
        );
        let w2 = cyl_sheet_b(&mut body, th0, th1, z0, z1, false);
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        (body, w1, w2)
    }

    /// MATE-5's census half of the red-first row: the declared
    /// cross-description cylinder seat CERTIFIES through the patch
    /// certifier — Door 1 verifies the carrier through the record's
    /// own declaration, Door 2 answers through the certified-ε
    /// enclosure. Before this arm, the same records dead-ended
    /// `CensusUnsupported{FacePair}` (the chain the predicate-level
    /// red-first suite quotes).
    #[test]
    fn a_cross_description_cylinder_patch_record_certifies() {
        let (body, w1, w2) = cross_description_pair(0.5, 1.3, 0.3, 0.7);
        let mut records = ContactRecords::default();
        records.push(crate::Cited::new(
            crate::Contact::of(crate::Cell::Face(w1), crate::Cell::Face(w2)).unwrap(),
            crate::Cites::decided(0),
        ));
        let errors = census_and_certify(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
        );
        let findings = face_findings(&errors);
        assert!(
            findings.is_empty(),
            "the declared cross-description cylinder seat certifies: \
             {findings:?} (all: {errors:?})"
        );
    }

    /// MATE-5's Refuted-arm acceptance row (the #1063 consequence
    /// wiring, now live for cylinders): a cylinder declaration the
    /// geometry refutes — the trims' axial bands definitely disjoint
    /// on the shared carrier — is `StaleContactDeclaration`, which the
    /// assembly attribution maps to `Refuted` naming its mate
    /// (`editor-core`'s already-landed arm consumes this variant).
    #[test]
    fn a_refuted_cross_description_cylinder_record_is_stale_typed() {
        let (body, w1, w2) = cross_description_pair(0.5, 1.3, 2.0, 2.5);
        let mut records = ContactRecords::default();
        records.push(crate::Cited::new(
            crate::Contact::of(crate::Cell::Face(w1), crate::Cell::Face(w2)).unwrap(),
            crate::Cites::decided(0),
        ));
        let errors = census_and_certify(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
        );
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::StaleContactDeclaration { .. })),
            "a refuted cylinder declaration is stale typed: {errors:?}"
        );
    }
    // ---- A described net carrying poison: the reach and box lanes ----

    /// Every mvfs placeholder seed face of `body`, its surface swapped
    /// for `surface`.
    fn swap_placeholders(body: &mut Body<f64>, surface: &Surface<f64>) -> Vec<FaceKey> {
        let seeds: Vec<FaceKey> = body
            .faces()
            .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Nurbs(p)) if p.is_placeholder()))
            .map(|(k, _)| k)
            .collect();
        for &f in &seeds {
            // Lifts RechartUnvouched: the census reads the masquerade patch on each seed, wherever its boundary lies.
            body.set_face_surface_unvouched_for_tests(
                f,
                FaceSurface::New {
                    surface: surface.clone(),
                    sense: true,
                },
            )
            .unwrap();
        }
        seeds
    }

    /// A described net carrying poison has no reach: the backstop
    /// refuses every cross-solid pair naming it, and arm 2 takes
    /// neither solid as the container, rather than either clearing on
    /// the finite channels. The net's `z` lane sits at `2`, a kilometre
    /// below the other sheet, and would decide that gap on its own. Two
    /// nets: `x` poisoned at every point, and at one point only.
    #[test]
    fn a_net_poisoned_in_one_channel_has_no_reach_and_clears_no_pair() {
        for (what, poisoned) in [
            ("every point", &(|_| true) as &dyn Fn(usize) -> bool),
            ("point 0", &|i| i == 0),
        ] {
            let mut body = Body::<f64>::new();
            let (wall_a, cyl) = unit_cyl_sheet(
                &mut body,
                None,
                (0.2, 1.6),
                (0.0, 1.0),
                true,
                Tol::witness(),
            );
            let (wall_b, _) = unit_cyl_sheet(
                &mut body,
                Some(cyl),
                (1.0, 2.4),
                (1000.3, 1000.7),
                false,
                Tol::witness(),
            );
            crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
            let seeds = swap_placeholders(&mut body, &crate::fixtures::poisoned_net(poisoned));
            assert_eq!(seeds.len(), 2, "{what}: one seed face per sheet");
            for &f in &seeds {
                let reach = face_reach(&body, f, Band::linear(Tol::witness()).unwrap());
                assert!(
                    reach.is_none(),
                    "{what}, {f:?}: a net poisoned in x bounds its locus on no axis, got {reach:?}"
                );
                // The boolean lane's answer for the same net: the poison box.
                let boxed = crate::boolean::boxes::face_box(
                    &body,
                    f,
                    0.0,
                    Band::linear(Tol::witness()).unwrap(),
                )
                .unwrap();
                assert!(
                    [
                        boxed.min_x,
                        boxed.min_y,
                        boxed.min_z,
                        boxed.max_x,
                        boxed.max_y,
                        boxed.max_z
                    ]
                    .iter()
                    .all(|c| c.is_nan()),
                    "{what}, {f:?}: the boolean lane's box is poison on every axis, got {boxed:?}"
                );
            }
            let errs = census_and_certify(
                &body,
                &ContactRecords::default(),
                band(),
                Tol::witness(),
                Some(RegionLane::certified()),
            );
            let refused = |x: EntityId, y: EntityId, why: Undecided, ordered: bool| {
                errs.iter().any(|e| {
                    matches!(
                        e,
                        ValidationError::CensusUndecidable { a, b, what }
                            if ((*a, *b) == (x, y) || (!ordered && (*a, *b) == (y, x)))
                                && *what == why.what()
                    )
                })
            };
            let solid = |f: FaceKey| body.solid_of_face(f).unwrap();
            for &seed in &seeds {
                for other in [wall_a, wall_b].into_iter().chain(seeds.iter().copied()) {
                    if solid(other) == solid(seed) {
                        continue;
                    }
                    assert!(
                        refused(
                            EntityId::Face(seed),
                            EntityId::Face(other),
                            Undecided::NoSoundReach,
                            false
                        ),
                        "{what}, {seed:?} x {other:?}: a poisoned net refuses as unclaimable, \
                         never clears; census said {errs:#?}"
                    );
                }
            }
            // Arm 2: each solid carries a poisoned face, so neither has a
            // claimable extent and both orderings refuse.
            let (sa, sb) = (
                EntityId::Solid(solid(wall_a)),
                EntityId::Solid(solid(wall_b)),
            );
            for (outer, inner) in [(sa, sb), (sb, sa)] {
                assert!(
                    refused(outer, inner, Undecided::Unclaimable, true),
                    "{what}, {outer:?} around {inner:?}: a solid with a poisoned face is \
                     never the container; census said {errs:#?}"
                );
            }
        }
    }

    /// The sibling discriminator-free fold: `face_box` (boolean lane)
    /// on the masquerade, and whether its partially poisoned box PRUNES
    /// against a box that is disjoint on a finite axis.
    #[test]
    fn n2r2_face_box_partial_poison_prunes_on_finite_axes() {
        let mut body = Body::<f64>::new();
        let (_w1, _cyl) = unit_cyl_sheet(
            &mut body,
            None,
            (0.2, 1.6),
            (0.0, 1.0),
            true,
            Tol::witness(),
        );
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        let seeds = swap_placeholders(&mut body, &crate::fixtures::poisoned_net(|_| true));
        let b = crate::boolean::boxes::face_box(
            &body,
            seeds[0],
            1e-9,
            Band::linear(Tol::witness()).unwrap(),
        )
        .unwrap();
        eprintln!("[face_box] masquerade box = {b:?}");
        let far = bvh::Aabb::from_points([
            Point3::new(0.0, 100.0, 100.0),
            Point3::new(1.0, 101.0, 101.0),
        ])
        .unwrap();
        let near = bvh::Aabb::from_points([Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)])
            .unwrap();
        eprintln!(
            "[face_box] overlaps(far y/z-disjoint box) = {}  overlaps(near box) = {}",
            b.overlaps(&far),
            b.overlaps(&near)
        );
        // EXPECTATION UPDATED BY THE FIX PASS, and this is the row that
        // records the repair: the probe was written when this fold
        // answered a box poisoned in x and finite in y/z, which pruned
        // against `far`. The door it calls is `geom`'s
        // `nurbs_surface_aabb`, which now screens on poison ANYWHERE, so
        // the box is poison on every axis and prunes against neither.
        assert!(
            b.overlaps(&far) && b.overlaps(&near),
            "the boolean lane's face box must not prune on a net it cannot bound"
        );
    }

    // ------------------------------------------------------------------
    // The BVH pre-filter (`Candidates`): the idealized/realized pair at
    // the census door itself, on operator-built unit cubes placed by
    // rigid translation and grafted into one body. The corpus-scale and
    // curved rows live in editor-core's `perf12_census_bvh_diff`.

    /// The unit cube with its minimum corner at `at`: the operator
    /// prism skeleton with real geometry — every face a `Plane` with
    /// its outward normal, every edge a `Line` between its vertices.
    fn cube_at(at: Vec3<f64>, tol: Tol) -> Body<f64> {
        cube_at_turned(at, 0.0, tol)
    }

    /// [`cube_at`], turned by `theta` about the vertical through its
    /// minimum corner — a marginal `theta` puts its edges at a
    /// marginal angle to an axis-aligned cube's.
    fn cube_at_turned(at: Vec3<f64>, theta: f64, tol: Tol) -> Body<f64> {
        use geom_brep::EdgeCurveSpec;
        let mut p = crate::fixtures::raw_prism(4, tol);
        let corners = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let (c, sn) = (theta.cos(), theta.sin());
        for (i, (x, y)) in corners.into_iter().enumerate() {
            for (v, z) in [(p.t[i], 1.0), (p.u[i], 0.0)] {
                let point = p.body.get_vertex(v).expect("a rim vertex").point;
                *p.body.points.get_mut(point).expect("its point") =
                    Point3::new(at.x + x * c - y * sn, at.y + x * sn + y * c, at.z + z);
            }
        }
        let point_of = |body: &Body<f64>, v: VertexKey| -> Point3<f64> {
            *body
                .points
                .get(body.vertices.get(v).expect("a vertex").point)
                .expect("a point")
        };
        let centre = Point3::new(at.x + 0.5 * (c - sn), at.y + 0.5 * (sn + c), at.z + 0.5);
        let faces: Vec<FaceKey> = p.body.faces().map(|(k, _)| k).collect();
        for f in faces {
            let outer = p.body.get_face(f).expect("a face").outer;
            let LoopBoundary::Cycle { first } = p.body.loops.get(outer).expect("a loop").boundary
            else {
                panic!("a prism face's outer loop is a cycle");
            };
            let pts: Vec<Point3<f64>> = p
                .body
                .loop_cycle(first)
                .expect("a cycle")
                .iter()
                .map(|&he| {
                    point_of(
                        &p.body,
                        p.body.half_edges.get(he).expect("a half-edge").start,
                    )
                })
                .collect();
            let n = pts.len() as f64;
            let centroid = Point3::new(
                pts.iter().map(|q| q.x).sum::<f64>() / n,
                pts.iter().map(|q| q.y).sum::<f64>() / n,
                pts.iter().map(|q| q.z).sum::<f64>() / n,
            );
            // A cube: the face centroid sits exactly half a side from the
            // centre along the face's own (turned) axis.
            let d = centroid - centre;
            let normal = Vec3::new(2.0 * d.x, 2.0 * d.y, 2.0 * d.z);
            // A wall's normal is horizontal, so the vertical is in its
            // plane; a cap's is vertical, so any horizontal axis is.
            let u_ref = if normal.z.abs() < 0.5 {
                Vec3::unit_z()
            } else {
                Vec3::unit_x()
            };
            p.body
                .set_face_surface(
                    f,
                    FaceSurface::New {
                        surface: Surface::Plane {
                            origin: centroid,
                            normal,
                            u_ref,
                        },
                        sense: true,
                    },
                )
                .expect("a plane on a planar loop");
        }
        let edges: Vec<EdgeKey> = p.body.edges().map(|(k, _)| k).collect();
        for e in edges {
            let he = p.body.get_edge(e).expect("an edge").he_plus;
            let p0 = point_of(
                &p.body,
                p.body.get_half_edge(he).expect("a half-edge").start,
            );
            let p1 = point_of(&p.body, p.body.half_edge_end(he).expect("an end"));
            p.body
                .set_edge_curve(e, EdgeCurveSpec::line_between(p0, p1), tol)
                .expect("a line between the edge's vertices");
        }
        p.body
    }

    /// **The two readings of an at-rest finding, side by side.** The gather
    /// localizes interference only where the census decided it:
    /// [`census_interference_solids`] reads an instance's vertex in the
    /// other's material, a pierce and a `SameSide` crossing. The door
    /// records no touch wherever the census did not clear the pair:
    /// [`census_uncleared_solids`] reads those, plus an `Undecided`
    /// crossing and a pair it cannot look at, whose entities may be
    /// edges or vertices. A touch, and a crossing read `OppositeSides`,
    /// is neither. Red if either reading drops a finding it owes, or if
    /// the decided reading takes in something the census left
    /// undecided.
    #[test]
    fn the_decided_reading_is_the_uncleared_reading_less_what_the_census_left_open() {
        let body = two_cubes(Vec3::new(2.0, 0.0, 0.0));
        let solids: Vec<crate::SolidKey> = body.solids().map(|(k, _)| k).collect();
        let [s0, s1] = solids[..] else {
            panic!("two solids: {solids:?}");
        };
        let edge_of = |s| {
            body.edges()
                .map(|(k, _)| k)
                .find(|&e| census_cell_solid(&body, crate::Cell::Edge(e)) == Some(s))
                .expect("an edge of the solid")
        };
        let vertex_of = |s| {
            body.vertices()
                .map(|(k, _)| k)
                .find(|&v| census_cell_solid(&body, crate::Cell::Vertex(v)) == Some(s))
                .expect("a vertex of the solid")
        };
        let face_of = |s| {
            body.faces()
                .map(|(k, _)| k)
                .find(|&f| body.solid_of_face(f) == Some(s))
                .expect("a face of the solid")
        };
        let (ea, eb) = (edge_of(s0), edge_of(s1));
        let touch = |contact| ValidationError::UnrecordedContact {
            contact,
            witness: String::new(),
            margin: None,
        };
        let cross = |side| touch(CensusContact::EdgeEdgeCross { a: ea, b: eb, side });
        let pair = Some((s0, s1));
        // (finding, the decided reading, the uncleared reading)
        let rows = [
            (
                ValidationError::InstanceInterference {
                    outer: s0,
                    inner: s1,
                    witness: vertex_of(s1),
                },
                pair,
                pair,
            ),
            (
                touch(CensusContact::EdgeFacePierce {
                    edge: ea,
                    face: face_of(s1),
                }),
                pair,
                pair,
            ),
            (cross(Some(CrossingSideVerdict::SameSide)), pair, pair),
            (cross(Some(CrossingSideVerdict::Undecided)), None, pair),
            (cross(Some(CrossingSideVerdict::OppositeSides)), None, None),
            (cross(None), None, None),
            (
                ValidationError::CensusUndecidable {
                    a: EntityId::Edge(ea),
                    b: EntityId::Vertex(vertex_of(s1)),
                    what: "",
                },
                None,
                pair,
            ),
            (
                touch(CensusContact::VertexOnFace {
                    vertex: vertex_of(s0),
                    face: face_of(s1),
                }),
                None,
                None,
            ),
        ];
        for (finding, decided, uncleared) in rows {
            assert_eq!(
                census_interference_solids(&body, &finding),
                decided,
                "decided: {finding:?}"
            );
            assert_eq!(
                census_uncleared_solids(&body, &finding),
                uncleared,
                "uncleared: {finding:?}"
            );
        }
    }

    /// One body: the unit cube at the origin and a second cube whose
    /// minimum corner is at `at`.
    fn two_cubes(at: Vec3<f64>) -> Body<f64> {
        two_cubes_turned(at, 0.0)
    }

    /// [`two_cubes`] with the second cube turned by `theta`.
    fn two_cubes_turned(at: Vec3<f64>, theta: f64) -> Body<f64> {
        let tol = Tol::witness();
        let mut body = cube_at(Vec3::new(0.0, 0.0, 0.0), tol);
        let other = cube_at_turned(at, theta, tol);
        crate::instance::graft_disjoint(&mut body, &other).expect("a disjoint graft");
        body
    }

    /// A planar half-disc cap — one arc edge of the unit circle and its
    /// chord, a two-vertex arc-bearing loop whose vertex polygon has no
    /// area — with a cube grafted in beside it whose bottom vertices lie
    /// in the cap's plane, far away.
    fn half_disc_cap_and_far_cube() -> Body<f64> {
        use geom::Curve3;
        use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let (p0, p1) = (Point3::new(1.0, 0.0, 0.0), Point3::new(-1.0, 0.0, 0.0));
        let seed = body.mvfs(p0, true).expect("a seed");
        // After the seed: a surface on an empty body is an orphan.
        let plane = body.add_surface(Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        });
        let cyl = body.add_surface(CylFrame::canonical(1.0).surface());
        let arc = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p1,
                EdgeCurveSpec {
                    description: EdgeDescriptionSpec::Intersection {
                        pair: SurfacePair::new(cyl, plane),
                        witness: Point3::new(0.0, 1.0, 0.0),
                    },
                    carrier: Curve3::Circle {
                        center: Point3::origin(),
                        axis: Vec3::unit_z(),
                        radius: 1.0,
                        u_ref: Vec3::unit_x(),
                    },
                    param_start: 0.0,
                    param_end: std::f64::consts::PI,
                },
                tol,
            )
            .expect("the arc");
        body.mef(
            MefSite::Chords {
                he1: arc.he_minus,
                he2: arc.he_plus,
            },
            EdgeCurveSpec::line_between(p1, p0),
            FaceSurface::Shared {
                key: plane,
                sense: true,
            },
            tol,
        )
        .expect("the chord closes the cap");
        let cube = cube_at(Vec3::new(10.0, 3.0, 0.0), tol);
        crate::instance::graft_disjoint(&mut body, &cube).expect("a disjoint graft");
        body
    }

    /// A planar cap in the plane `x = offset`, bounded by one SPIRIC arc
    /// — the section of the torus `(R, r)` about the `z` axis, over its
    /// minor angle `v ∈ [v0, v1]` — and the chord closing it.
    fn spiric_cap(big_r: f64, r: f64, offset: f64, (v0, v1): (f64, f64)) -> Body<f64> {
        use geom::Curve3;
        use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
        let tol = Tol::witness();
        let spiric = Curve3::Spiric {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
            major_radius: big_r,
            minor_radius: r,
            offset,
        };
        let (p0, p1) = (spiric.eval(v0), spiric.eval(v1));
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p0, true).expect("a seed");
        let plane = body.add_surface(Surface::Plane {
            origin: Point3::new(offset, 0.0, 0.0),
            normal: Vec3::unit_x(),
            u_ref: Vec3::unit_y(),
        });
        let torus = body.add_surface(Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: big_r,
            minor_radius: r,
            u_ref: Vec3::unit_x(),
        });
        let arc = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p1,
                EdgeCurveSpec {
                    description: EdgeDescriptionSpec::Intersection {
                        pair: SurfacePair::new(torus, plane),
                        witness: spiric.eval(0.5 * (v0 + v1)),
                    },
                    carrier: spiric,
                    param_start: v0,
                    param_end: v1,
                },
                tol,
            )
            .expect("the spiric arc");
        body.mef(
            MefSite::Chords {
                he1: arc.he_minus,
                he2: arc.he_plus,
            },
            EdgeCurveSpec::line_between(p1, p0),
            FaceSurface::Shared {
                key: plane,
                sense: true,
            },
            tol,
        )
        .expect("the chord closes the cap");
        body
    }

    /// [`spiric_cap`] of the torus `R = 2, r = 1` in `x = 1/2`, over
    /// `[−π/2, π/2]`, with a cube grafted in beside it whose `x = 1/2`
    /// face lies in the cap's plane, outside the cap and inside the ball
    /// the spiric arc is held in (its midpoint `(1/2, √8.75, 0)`, reach
    /// `(π/2)·r(R − r)/√((R − r)² − 1/4) ≈ 1.81`).
    fn spiric_cap_and_near_cube() -> Body<f64> {
        use std::f64::consts::FRAC_PI_2;
        let tol = Tol::witness();
        let mut body = spiric_cap(2.0, 1.0, 0.5, (-FRAC_PI_2, FRAC_PI_2));
        let cube = cube_at(Vec3::new(0.5, 3.9, -0.5), tol);
        crate::instance::graft_disjoint(&mut body, &cube).expect("a disjoint graft");
        body
    }

    fn rendered(errors: &[ValidationError]) -> Vec<String> {
        errors.iter().map(|e| format!("{e:?}")).collect()
    }

    /// The realized run against the idealized one: identical errors,
    /// the realized examined sequence a restriction of the exact one
    /// holding every accepted pair. Returns the pairs pruned. The
    /// comparators are [`SweepPairs`]'s own, shared with editor-core's
    /// `perf12_census_bvh_diff`, whose `pin` is this loop over a
    /// product body.
    fn pin(body: &Body<f64>) -> usize {
        let records = ContactRecords::default();
        let (real_errors, real) = census_traces(
            body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Realized,
        );
        let (ideal_errors, ideal) = census_traces(
            body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Idealized,
        );
        assert_eq!(rendered(&real_errors), rendered(&ideal_errors));
        let mut pruned = 0;
        for ((name, r), (_, i)) in real.sweeps().iter().zip(ideal.sweeps().iter()) {
            assert!(r.is_restriction_of(i), "{name}: order");
            let lost = r.lost_accepted(i);
            assert!(
                lost.is_empty(),
                "{name}: the filter pruned accepted pairs {lost:?}"
            );
            assert_eq!(r.accepted, i.accepted, "{name}: accepted");
            pruned += i.examined.len() - r.examined.len();
        }
        pruned
    }

    /// The comparator can go red: plant one face box empty and the
    /// superset comparator MUST report the loss (`m5_pr8_bvh_diff`'s
    /// pin 3, at this door).
    #[test]
    fn planted_degradation_is_caught() {
        // The second cube's corner rests in the interior of the first's
        // x = 1 face: a vertex-on-face event, unrecorded.
        let flush = two_cubes(Vec3::new(1.0, 0.25, 0.25));
        let records = ContactRecords::default();
        let (_, ideal) = census_traces(
            &flush,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Idealized,
        );
        let &(_, EntityId::Face(face)) = ideal
            .vf
            .accepted
            .first()
            .expect("a corner in a face's interior is a vertex-on-face event")
        else {
            panic!("a vf pair names a face");
        };
        let plant = crate::boolean::PlantedDegradation { face };
        let (_, real) = census_traces_planted(
            &flush,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Realized,
            plant,
        );
        let lost = real.vf.lost_accepted(&ideal.vf);
        assert!(
            lost.iter().any(|&(_, f)| f == EntityId::Face(face)),
            "a planted-empty face box must lose its accepted pairs (got {lost:?})"
        );
    }

    /// **A plane residual in the band's edge escalates only where the
    /// face can be touched.** A unit cube turned a quarter-right-angle
    /// stands `5e-9` above another's top, in the band's edge of it
    /// (`band()`), its bottom a diamond whose box overlaps the top's.
    /// Where the diamond lies 0.32 clear of the top, the top's corner
    /// (vertex–face) and the diamond's edges (edge–face) read nothing
    /// whichever the residual is; where it covers the top's corner,
    /// both escalate.
    #[test]
    fn a_plane_residual_in_band_escalates_only_where_the_face_can_be_touched() {
        let residuals = |at: Vec3<f64>| -> Vec<&'static str> {
            let body = two_cubes_turned(at, core::f64::consts::FRAC_PI_4);
            let (errors, _) = census_traces(
                &body,
                &ContactRecords::default(),
                band(),
                Tol::witness(),
                Some(RegionLane::certified()),
                CensusStrategy::Realized,
            );
            errors
                .iter()
                .filter_map(|e| match e {
                    ValidationError::CensusEscalated { cause } => cause.predicate,
                    _ => None,
                })
                .filter(|p| ["pm_census_ef_residual", "pm_census_vf_residual"].contains(p))
                .collect()
        };
        let lift = 1.0 + 5e-9;
        let apart = residuals(Vec3::new(1.657, 0.793, lift));
        assert!(apart.is_empty(), "the diamond clear of the top: {apart:?}");
        let over = residuals(Vec3::new(1.157, 0.293, lift));
        for p in ["pm_census_ef_residual", "pm_census_vf_residual"] {
            assert!(
                over.contains(&p),
                "the diamond over the top's corner: {p} in {over:?}"
            );
        }
    }

    /// The predicates `errors` escalated.
    fn escalated(errors: &[ValidationError]) -> Vec<&'static str> {
        errors
            .iter()
            .filter_map(|e| match e {
                ValidationError::CensusEscalated { cause } => cause.predicate,
                _ => None,
            })
            .collect()
    }

    /// **A face's corner in the band's edge of an edge's line cuts the
    /// edge where it is on the interior, and nothing past an end.** A
    /// cube turned a quarter-right-angle stands on the unit cube's top,
    /// its bottom a diamond whose lowest corner sits `5e-9` off the top's
    /// edge `y = 0`. Over the edge's middle the corner's reading on the
    /// segment escalates, beside the crossing reading of its two sides;
    /// half a unit past its end it cuts nothing and reads nothing.
    #[test]
    fn a_corner_in_band_of_an_edges_line_escalates_only_over_its_interior() {
        let corner_reading = |x: f64| -> Vec<&'static str> {
            let body = two_cubes_turned(Vec3::new(x, 5e-9, 1.0), core::f64::consts::FRAC_PI_4);
            let geo = snapshot(&body);
            let at = |p: Point3<f64>| {
                geo.verts
                    .iter()
                    .find(|&&(_, q)| (q - p).norm() < 1e-12)
                    .map(|&(k, _)| k)
                    .expect("a vertex there")
            };
            let (a, b) = (
                at(Point3::new(0.0, 0.0, 1.0)),
                at(Point3::new(1.0, 0.0, 1.0)),
            );
            let e = geo
                .edges
                .iter()
                .find(|e| [e.v0, e.v1] == [a, b] || [e.v0, e.v1] == [b, a])
                .expect("the top's edge y = 0");
            let corner = at(Point3::new(x, 5e-9, 1.0));
            let f = geo
                .faces
                .iter()
                .find(|f| f.boundary.contains(&corner) && f.normal.z < -0.5)
                .expect("the diamond");
            let mut errors = Vec::new();
            let _ = ef_overlap_cells(&body, e, f, &geo, band(), &mut errors);
            escalated(&errors)
                .into_iter()
                .filter(|p| p.starts_with("curve_carrier_"))
                .collect()
        };
        // Two readings in the band's edge: the corner's own on the
        // segment, and the crossing reading of the corner's sides.
        let over = corner_reading(0.5);
        assert_eq!(over.len(), 2, "the corner over the edge's middle: {over:?}");
        let past = corner_reading(1.5);
        assert!(past.is_empty(), "the corner past the edge's end: {past:?}");
    }

    /// One edge–face pair read alone against the unit cube's top
    /// (`band()`: zero `1e-9`, escalation `1e-8`): what it pushed.
    /// `backed` records both ends of the segment on the top.
    fn top_pair(p0: [f64; 3], p1: [f64; 3], backed: bool) -> Vec<ValidationError> {
        let body = cube_at(Vec3::new(0.0, 0.0, 0.0), Tol::witness());
        let geo = snapshot(&body);
        let top = geo
            .faces
            .iter()
            .find(|f| f.normal.z > 0.5)
            .expect("the top");
        let e = segment((
            Point3::new(p0[0], p0[1], p0[2]),
            Point3::new(p1[0], p1[1], p1[2]),
        ));
        let mut declared = Declared::index(&geo, &ContactRecords::default());
        if backed {
            declared.vf.insert((e.v0, top.key));
        }
        let mut errors = Vec::new();
        pair_edge_face(
            &body,
            &geo,
            &declared,
            band(),
            Some(RegionLane::certified()),
            &e,
            top,
            &mut errors,
        );
        errors
    }

    /// **A residual in the band's edge escalates by every arm a sign it
    /// can take reaches**, each pair read alone against the unit cube's
    /// top ([`top_pair`]). An undecided end reads `Zero` or the side its
    /// residual lies on:
    ///
    /// - lying `5e-9` above the top's middle: only the overlap arm finds
    ///   the edge on the face, and the contact it finds there holds only
    ///   under that reading, so it does not ship;
    /// - standing up from `5e-9` below it: only the pierce finds the face;
    /// - standing up from `5e-9` above it: no reading crosses the plane
    ///   (its foot on the face is the vertex–face pass's);
    /// - tilted through it from `+8e-9` to `-8e-9`, the overlap backed by
    ///   a record of both ends on the face: only the opposite signs'
    ///   pierce finds it;
    /// - lying `5e-9` above the plane a unit off the face: nothing.
    #[test]
    fn an_undecided_residual_escalates_by_each_arm_its_signs_reach() {
        let residual = |what: &str, found: Vec<ValidationError>| {
            assert!(
                escalated(&found).contains(&"pm_census_ef_residual"),
                "{what}: {found:?}"
            );
            assert!(
                !found
                    .iter()
                    .any(|e| matches!(e, ValidationError::UnrecordedContact { .. })),
                "{what}: a contact found under a hypothetical reading ships: {found:?}"
            );
        };
        let nothing = |what: &str, found: Vec<ValidationError>| {
            assert!(found.is_empty(), "{what}: {found:?}");
        };
        let (up, down) = (1.0 + 5e-9, 1.0 - 5e-9);
        residual(
            "lying over the top",
            top_pair([0.25, 0.5, up], [0.75, 0.5, up], false),
        );
        residual(
            "standing up from below the top",
            top_pair([0.5, 0.5, down], [0.5, 0.5, 2.0], false),
        );
        nothing(
            "standing up from above the top",
            top_pair([0.5, 0.5, up], [0.5, 0.5, 2.0], false),
        );
        residual(
            "tilted through the top, its overlap backed",
            top_pair([0.3, 0.5, 1.0 + 8e-9], [0.7, 0.5, 1.0 - 8e-9], true),
        );
        nothing(
            "lying off the face",
            top_pair([2.25, 0.5, up], [2.75, 0.5, up], false),
        );
    }

    /// **A refusal beside an undecided residual carries every branch's
    /// refusals, and no branch's contact.** Tilted through the top from
    /// `+4e-9` to `-4e-9`, unbacked: the first reading to find anything
    /// (both ends on the plane) finds an overlap that holds only under it,
    /// and the last (the two opposite sides) escalates the crossing's
    /// lever, whose difference `8e-9` is in the band's edge. The refusal
    /// holds the residuals and the lever, the lever once, and no contact.
    #[test]
    fn a_refusal_beside_an_undecided_residual_carries_every_branch() {
        let found = top_pair([0.3, 0.5, 1.0 + 4e-9], [0.7, 0.5, 1.0 - 4e-9], false);
        let rows = escalated(&found);
        assert_eq!(
            rows.iter()
                .filter(|p| **p == "pm_census_ef_residual")
                .count(),
            2,
            "{found:?}"
        );
        assert_eq!(
            rows.iter()
                .filter(|p| **p == "pm_census_ef_pierce_lever")
                .count(),
            1,
            "{found:?}"
        );
        assert_eq!(rows.len(), found.len(), "only escalations: {found:?}");
    }

    /// **A vertex–face refusal carries what was read at the vertex.** A
    /// vertex `5e-9` above the top's plane and `5e-9` past its edge
    /// `x = 1`: on the plane it would lie in the band's edge of the face's
    /// boundary, so its containment escalates there, read at the vertex
    /// itself, and the refusal carries it beside the residual.
    #[test]
    fn a_vertex_face_refusal_carries_the_containment_read_at_the_vertex() {
        let body = cube_at(Vec3::new(0.0, 0.0, 0.0), Tol::witness());
        let geo = snapshot(&body);
        let top = geo
            .faces
            .iter()
            .find(|f| f.normal.z > 0.5)
            .expect("the top");
        let declared = Declared::index(&geo, &ContactRecords::default());
        let q = Point3::new(1.0 + 5e-9, 0.5, 1.0 + 5e-9);
        let mut errors = Vec::new();
        pair_vertex_face(
            &body,
            &geo,
            &declared,
            band(),
            (VertexKey::default(), q),
            top,
            &mut errors,
        );
        let rows = escalated(&errors);
        assert!(rows.contains(&"pm_census_vf_residual"), "{errors:?}");
        assert!(
            rows.iter().any(|p| *p != "pm_census_vf_residual"),
            "the containment read at the vertex: {errors:?}"
        );
    }

    /// **A chord whose length is undecided cuts where a chord would**:
    /// read at unit level against the top's edge-free middle line. A
    /// chord `9e-9` long across the line meets it, so its length
    /// escalates, carrying what the meeting read; the same chord a unit
    /// off the line meets nothing, and pushes nothing.
    #[test]
    fn an_undecided_chord_cuts_where_a_chord_would() {
        use crate::splitting::containment::LoopEdge;
        let body = cube_at(Vec3::new(0.0, 0.0, 0.0), Tol::witness());
        let geo = snapshot(&body);
        let top = geo
            .faces
            .iter()
            .find(|f| f.normal.z > 0.5)
            .expect("the top");
        let e = segment((Point3::new(0.0, 0.5, 1.0), Point3::new(1.0, 0.5, 1.0)));
        let crossings = |x: f64, y: f64| {
            let side = BoundarySide {
                r#loop: LoopKey::default(),
                key: EdgeKey::default(),
                ends: (
                    Point3::new(x, y - 4.5e-9, 1.0),
                    Point3::new(x, y + 4.5e-9, 1.0),
                ),
            };
            let mut errors = Vec::new();
            let cuts = edge_crossings(
                &body,
                &e,
                top.key,
                &LoopEdge::Chord,
                side,
                band(),
                &mut errors,
            );
            (cuts, errors)
        };
        let (cuts, errors) = crossings(0.5, 0.5);
        assert!(cuts.is_none(), "across the line: {cuts:?}");
        assert!(
            escalated(&errors).contains(&"pm_census_ef_cross_chord"),
            "across the line: {errors:?}"
        );
        let (cuts, errors) = crossings(0.5, 1.5);
        assert_eq!(cuts.map(|c| c.len()), Some(0), "off the line");
        assert!(errors.is_empty(), "off the line: {errors:?}");
    }

    /// Review 2 (PR 5261): one edge-face pair read alone against the unit
    /// cube's top, its escalated predicates.
    fn review2_pair_reads(p0: [f64; 3], p1: [f64; 3], backed: bool) -> Vec<&'static str> {
        escalated(&top_pair(p0, p1, backed))
    }

    /// Review 2: tilted through the top from `+4e-9` to `-4e-9`, the
    /// residuals' difference `8e-9` is in the band's edge, so the lever
    /// escalates. Pins the lever's `Err` arm (green at the head; red
    /// under a mutant that returns silently there).
    #[test]
    fn review2_an_undecided_lever_escalates() {
        let found = review2_pair_reads([0.3, 0.5, 1.0 + 4e-9], [0.7, 0.5, 1.0 - 4e-9], true);
        assert!(found.contains(&"pm_census_ef_pierce_lever"), "{found:?}");
    }

    /// Review 2: the same pair, counting. `[P, N]` and `[N, P]` decide
    /// the one lever twice and push its escalation twice (red at the
    /// head: the refusal carries one cause two times).
    #[test]
    fn review2_an_undecided_lever_escalates_once() {
        let found = review2_pair_reads([0.3, 0.5, 1.0 + 4e-9], [0.7, 0.5, 1.0 - 4e-9], true);
        let lever = found
            .iter()
            .filter(|p| **p == "pm_census_ef_pierce_lever")
            .count();
        assert_eq!(lever, 1, "{found:?}");
    }

    /// Review 2: an edge descending toward the top's plane from `3e-8`
    /// above at `x = -1` to `5e-9` above at `x = -0.1`, wholly outside the
    /// face. No reading of the undecided end touches the face: read
    /// `Zero` it is a vertex outside the face (pass 3), read on its
    /// computed side nothing crosses. Only the hypothetical `[P, N]`,
    /// whose pierce point is placed from the computed residuals at
    /// `r0 / (r0 - r1) = 1.2`, lands at `x = 0.08`, inside the face, and
    /// escalates the residual (red at the head).
    #[test]
    fn review2_an_extrapolated_pierce_escalates_no_residual() {
        let found = review2_pair_reads([-1.0, 0.5, 1.0 + 3e-8], [-0.1, 0.5, 1.0 + 5e-9], false);
        assert!(found.is_empty(), "{found:?}");
    }

    #[test]
    fn flush_and_far_cubes_decide_identically_through_either_strategy() {
        // Flush: the second cube's x = 1 face on the first's, unrecorded
        // — findings on every shared-boundary event; the far faces and
        // vertices of each cube are pruned against the other's.
        let flush = two_cubes(Vec3::new(1.0, 0.0, 0.0));
        let (errors, _) = census_traces(
            &flush,
            &ContactRecords::default(),
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Realized,
        );
        assert!(!errors.is_empty(), "flush cubes have unrecorded contacts");
        assert!(pin(&flush) > 0, "the flush pair prunes the far faces");
        // Far: nothing cross-solid survives the filter, and the census
        // is clean either way.
        let far = two_cubes(Vec3::new(10.0, 0.0, 0.0));
        let (errors, real) = census_traces(
            &far,
            &ContactRecords::default(),
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Realized,
        );
        assert!(errors.is_empty(), "{errors:?}");
        assert!(pin(&far) > 0);
        let x_of = |v: VertexKey| {
            far.points
                .get(far.vertices.get(v).expect("a vertex").point)
                .expect("a point")
                .x
        };
        for (v, f) in &real.vf.examined {
            let (EntityId::Vertex(v), EntityId::Face(f)) = (v, f) else {
                panic!("a vf pair");
            };
            let LoopBoundary::Cycle { first } = far
                .loops
                .get(far.get_face(*f).expect("a face").outer)
                .expect("a loop")
                .boundary
            else {
                panic!("a cube face's outer loop is a cycle");
            };
            let face_x: Vec<f64> = far
                .loop_cycle(first)
                .expect("a cycle")
                .iter()
                .map(|&he| x_of(far.half_edges.get(he).expect("a half-edge").start))
                .collect();
            let same_solid = face_x.iter().all(|&x| (x >= 9.0) == (x_of(*v) >= 9.0));
            assert!(
                same_solid,
                "a cross-solid pair survived the filter on cubes 9 m apart"
            );
        }
    }

    #[test]
    fn a_poisoned_face_box_is_examined_against_everything() {
        // Null the carrier of one edge of the near cube: the two faces
        // it bounds lose their sound box (the boundary hull poisons),
        // so every far vertex is examined against them — while the near
        // cube's other faces are pruned against the same vertices.
        let mut body = two_cubes(Vec3::new(10.0, 0.0, 0.0));
        let x_of = |body: &Body<f64>, v: VertexKey| {
            body.points
                .get(body.vertices.get(v).expect("a vertex").point)
                .expect("a point")
                .x
        };
        let (edge, v0, v1) = body
            .edges()
            .find_map(|(k, e)| {
                let v0 = body.get_half_edge(e.he_plus)?.start;
                let v1 = body.half_edge_end(e.he_plus)?;
                (x_of(&body, v0) <= 1.0 && x_of(&body, v1) <= 1.0).then_some((k, v0, v1))
            })
            .expect("an edge of the near cube");
        let (f_plus, f_minus) = {
            let e = body.get_edge(edge).expect("the edge");
            let face_of = |he| {
                body.face_of_half_edge(he)
                    .expect("a half-edge and its loop")
            };
            (face_of(e.he_plus), face_of(e.he_minus))
        };
        let null = body.add_null_curve(crate::null::NullEdge {
            below_end: v0,
            above_end: v1,
        });
        body.get_edge_mut(edge).expect("the edge").curve = null;
        let far_vertices: Vec<VertexKey> = body
            .vertices()
            .map(|(k, _)| k)
            .filter(|&v| x_of(&body, v) >= 9.0)
            .collect();
        assert_eq!(far_vertices.len(), 8);
        let other_near_face = body
            .faces()
            .map(|(k, _)| k)
            .find(|&f| {
                f != f_plus && f != f_minus && {
                    let outer = body.get_face(f).expect("a face").outer;
                    let LoopBoundary::Cycle { first } =
                        body.loops.get(outer).expect("a loop").boundary
                    else {
                        return false;
                    };
                    body.loop_cycle(first).expect("a cycle").iter().all(|&he| {
                        x_of(&body, body.half_edges.get(he).expect("a half-edge").start) <= 1.0
                    })
                }
            })
            .expect("a near face off the nulled edge");
        let records = ContactRecords::default();
        let (real_errors, real) = census_traces(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Realized,
        );
        let (ideal_errors, _) = census_traces(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Idealized,
        );
        assert_eq!(rendered(&real_errors), rendered(&ideal_errors));
        for &v in &far_vertices {
            for f in [f_plus, f_minus] {
                assert!(
                    real.vf
                        .examined
                        .contains(&(EntityId::Vertex(v), EntityId::Face(f))),
                    "a poisoned face is examined against every vertex"
                );
            }
            assert!(
                !real
                    .vf
                    .examined
                    .contains(&(EntityId::Vertex(v), EntityId::Face(other_near_face))),
                "a sound far face is pruned"
            );
        }
    }

    #[test]
    fn separated_cubes_on_in_band_edge_lines_read_apart_in_both_sweeps() {
        // The second cube 10 m along x and 5e-9 along y — inside the
        // band's escalation zone (`zero` 1e-9, `escalate` 1e-8) of the
        // first cube's x-parallel edge LINES, and metres from its
        // edges. The exact sweep reads every pair by where the entities
        // lie, not by their carriers: the edge pairs as segments, a
        // vertex metres past an edge's span as past it, and a straight
        // edge against a face's boundary edge on one line by the gap
        // along that line. So it decides what the filter answers by
        // separation, and neither sweep raises anything.
        let body = two_cubes(Vec3::new(10.0, 5e-9, 0.0));
        let records = ContactRecords::default();
        let (real_errors, real) = census_traces(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Realized,
        );
        let (ideal_errors, ideal) = census_traces(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Idealized,
        );
        assert!(real_errors.is_empty(), "realized: {real_errors:?}");
        assert!(ideal_errors.is_empty(), "exact: {ideal_errors:?}");
        restricted_and_silent(&real, &ideal);
    }

    /// The class's shape, shared by its three rows: the realized
    /// examination is the exact one restricted, and every pair the
    /// exact sweep pushed for is one the filter pruned.
    fn restricted_and_silent(real: &CensusTrace, ideal: &CensusTrace) {
        for ((name, r), (_, i)) in real.sweeps().iter().zip(ideal.sweeps().iter()) {
            assert!(r.is_restriction_of(i), "{name}: order");
            assert!(
                i.accepted.iter().all(|p| !r.examined.contains(p)),
                "{name}: every accepted pair is a pruned one"
            );
        }
    }

    #[test]
    fn separated_edges_turned_in_band_read_apart_in_both_sweeps() {
        // The second cube 10 m away and turned by 5e-9 rad: its edges
        // meet the first cube's at sin θ · min_len = 5e-9, inside the
        // escalation zone of `pm_census_ee_parallel`, metres from any
        // edge. The exact sweep settles the in-band angle by the pair's
        // overlap reading, which finds the segments apart, so no edge
        // pair escalates in either sweep.
        let body = two_cubes_turned(Vec3::new(10.0, 0.0, 0.0), 5e-9);
        let records = ContactRecords::default();
        let (real_errors, real) = census_traces(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Realized,
        );
        let (ideal_errors, ideal) = census_traces(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Idealized,
        );
        assert!(real_errors.is_empty(), "{real_errors:?}");
        let mut predicates = BTreeSet::new();
        for e in &ideal_errors {
            let ValidationError::CensusEscalated { cause } = e else {
                panic!("a non-escalation on separated cubes: {e:?}");
            };
            predicates.insert(cause.predicate.expect("a named predicate"));
        }
        assert!(
            !predicates.iter().any(|p| p.starts_with("pm_census_ee_")),
            "an edge pair escalated: {predicates:?}"
        );
        assert!(ideal.ee.accepted.is_empty(), "{:?}", ideal.ee.accepted);
        restricted_and_silent(&real, &ideal);
    }

    #[test]
    fn a_separated_vertex_in_a_half_disc_caps_plane_is_decided_by_both_sweeps() {
        // A far cube's bottom vertices lie in a half-disc cap's plane.
        // The exact sweep reaches the cap's region walk for each, and the
        // walk reads the cap's two-vertex loop on its carriers: each
        // vertex is decided outside the cap, and nothing is refused
        // about a face it is nowhere near, under either strategy.
        // (The sheet's seed face has a placeholder surface, so the
        // backstop refuses the sheet×cube instance pair `Undecidable`
        // under BOTH strategies — a poisoned extent is never pruned;
        // that shared refusal is not this row's subject.)
        let body = half_disc_cap_and_far_cube();
        let records = ContactRecords::default();
        let (real_errors, real) = census_traces(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Realized,
        );
        let (ideal_errors, ideal) = census_traces(
            &body,
            &records,
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Idealized,
        );
        for e in real_errors.iter().chain(&ideal_errors) {
            assert!(
                !matches!(
                    e,
                    ValidationError::CensusUnsupported {
                        cause: CensusUnsupportedCause::Containment(_),
                        ..
                    }
                ),
                "the cap's region walk refused a separated vertex: {e:?}"
            );
        }
        let ideal_rendered = rendered(&ideal_errors);
        assert!(
            rendered(&real_errors)
                .iter()
                .all(|e| ideal_rendered.contains(e)),
            "the realized errors are among the exact errors"
        );
        for ((name, r), (_, i)) in real.sweeps().iter().zip(ideal.sweeps().iter()) {
            assert!(r.is_restriction_of(i), "{name}: order");
        }
        // The decided answer itself: each far vertex lies OUTSIDE the
        // cap, which a wrong `In` would turn into a contact finding.
        let cap = body
            .faces()
            .find(|(_, f)| {
                matches!(body.get_surface(f.surface), Some(Surface::Plane { .. }))
                    && match body.loops[f.outer].boundary {
                        LoopBoundary::Cycle { first } => {
                            body.loop_cycle(first).is_some_and(|c| c.len() == 2)
                        }
                        _ => false,
                    }
            })
            .map(|(k, _)| k)
            .expect("the cap: a plane face of two edges");
        let mut asked = 0;
        for (_, v) in body.vertices() {
            let p = body.points[v.point];
            if p.z != 0.0 || p.x < 5.0 {
                continue;
            }
            asked += 1;
            assert_eq!(
                crate::boolean::contfp(&body, cap, Vec3::unit_z(), p, band()),
                Ok(FaceContainment::Out),
                "a far vertex in the cap's plane at {p:?}"
            );
        }
        assert_eq!(
            asked, 4,
            "the cube's four bottom vertices lie in the cap's plane"
        );
        assert!(
            ideal_errors.iter().all(|e| !matches!(
                e,
                ValidationError::UnrecordedContact {
                    contact: CensusContact::VertexOnFace { face, .. },
                    ..
                } if *face == cap
            )),
            "no far vertex is a contact with the cap: {ideal_errors:?}"
        );
    }

    /// **Just past a spiric arc's end, the cap answers.** On the torus
    /// `R = 10, r = 1` cut at `offset = 5`, a short arc (`w = 0.2`) near
    /// `v = π/2`, where the oval's speed is within 4% of its bound, and a
    /// point just past the arc's end along its tangent — inside the ball
    /// the whole arc lies in, so no ray from it misses that ball. The
    /// walk crosses the arc on the oval itself, and the point lies past
    /// the chord's end, outside the lens between chord and arc.
    #[test]
    fn just_past_a_spiric_arcs_end_the_cap_answers() {
        use std::f64::consts::FRAC_PI_2;
        let (v0, v1) = (FRAC_PI_2 - 0.1, FRAC_PI_2 + 0.1);
        let body = spiric_cap(10.0, 1.0, 5.0, (v0, v1));
        let spiric = geom::Curve3::Spiric {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
            major_radius: 10.0,
            minor_radius: 1.0,
            offset: 5.0,
        };
        let (start, end) = (spiric.eval(v0), spiric.eval(v1));
        let tangent = (spiric.eval(v1 + 1e-6) - spiric.eval(v1 - 1e-6)).normalize();
        let mid = spiric.eval(0.5 * (v0 + v1));
        let reach = geom::spiric_period_rate_bounds(10.0, 1.0, 5.0).0 * 0.1;
        let q = end + tangent * 0.003;
        let chord = end - start;
        assert!(
            (q - mid).norm() < reach && (q - start).dot(chord) > chord.dot(chord),
            "the fixture's premise: inside the arc's ball, past the chord's end"
        );
        let cap = body
            .faces()
            .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Plane { .. })))
            .map(|(k, _)| k)
            .expect("the cap");
        assert_eq!(
            crate::boolean::contfp(&body, cap, Vec3::unit_x(), q, band()),
            Ok(FaceContainment::Out),
            "just past the arc's end, the cap answers"
        );
    }

    /// The census's point-in-face reading, executed: the cube's
    /// vertices in the spiric cap's plane lie inside the ball the cap's
    /// spiric arc lies in, and outside the cap. The walk crosses the
    /// arc, so the census reads every one of them off the cap — no
    /// `CensusUnsupported` about the face, no contact with it, and no
    /// `CensusEscalated` over a margin nothing metred.
    #[test]
    fn a_spiric_cap_reads_the_near_vertices_off_it() {
        let body = spiric_cap_and_near_cube();
        let (errors, _) = census_traces(
            &body,
            &ContactRecords::default(),
            band(),
            Tol::witness(),
            Some(RegionLane::certified()),
            CensusStrategy::Idealized,
        );
        assert!(
            errors.iter().all(|e| !matches!(
                e,
                ValidationError::CensusUnsupported { .. }
                    | ValidationError::UnrecordedContact {
                        contact: CensusContact::VertexOnFace { .. },
                        ..
                    }
            )),
            "the spiric cap reads the near vertices off it: {errors:?}"
        );
        for e in &errors {
            if let ValidationError::CensusEscalated { cause } = e {
                assert!(
                    !cause.margin.is_invalid()
                        && !matches!(
                            cause.predicate,
                            Some("pm_census_containment" | "bool_contfp_boundary")
                        ),
                    "a census escalation over a margin nothing metred: {e:?}"
                );
            }
        }
    }

    /// The touch analysis's verdict on every touch finding of `body`,
    /// by kind — through [`TouchSite`], the census's own mapping.
    fn touch_verdicts(body: &Body<f64>) -> Vec<(&'static str, TouchVerdict)> {
        sites(body)
            .into_iter()
            .map(|(site, verdict)| {
                let kind = match site {
                    TouchSite::VertexVertex(..) => "VertexVertex",
                    TouchSite::VertexOnEdge(..) => "VertexOnEdge",
                    TouchSite::EdgeEdge(..) => "EdgeEdgeOverlap",
                    TouchSite::VertexOnFace(..) => "VertexOnFace",
                    TouchSite::EdgeInFace(..) => "EdgeFaceOverlap",
                    TouchSite::EdgeCross(..) => "EdgeEdgeCross",
                };
                (kind, verdict)
            })
            .collect()
    }

    /// The L-bracket over `z ∈ [0, 1]` with an axis-aligned block
    /// grafted beside it.
    fn bracket_and_block(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
        const L: [(f64, f64); 6] = [
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ];
        let tol = Tol::witness();
        let mut body = crate::test_support_fixtures::prism_z::<f64>(&L, 0.0, 1.0, tol).body;
        let part = crate::test_support_fixtures::brick::<f64>(x, y, z, tol);
        crate::instance::graft_disjoint(&mut body, &part).unwrap();
        body
    }

    /// Two unit cubes united corner to corner at the origin, `[0, 1]³`
    /// and `[−1, 0]³`, made one solid with two cones there on one key —
    /// the shape an op leaves where it decided the corners one point — and a
    /// parallelepiped `u·a + v·b + w·c` grafted beside them, its corner
    /// moved onto that key too. Returns the body, the corner, and the
    /// pinch's vertex of the first cube's cone.
    fn pinch_and_corner(
        a: [f64; 3],
        b: [f64; 3],
        c: [f64; 3],
    ) -> (Body<f64>, VertexKey, VertexKey) {
        let tol = Tol::witness();
        let cube = |lo: f64| {
            crate::test_support_fixtures::brick::<f64>(
                (lo, lo + 1.0),
                (lo, lo + 1.0),
                (lo, lo + 1.0),
                tol,
            )
        };
        let mut body = cube(0.0);
        crate::instance::graft_disjoint(&mut body, &cube(-1.0)).unwrap();
        let part = crate::test_support_fixtures::mapped_cube::<f64>(
            move |u, v, w| {
                Point3::new(
                    u * a[0] + v * b[0] + w * c[0],
                    u * a[1] + v * b[1] + w * c[1],
                    u * a[2] + v * b[2] + w * c[2],
                )
            },
            tol,
        );
        let before: BTreeSet<VertexKey> = body.vertices.keys().collect();
        crate::instance::graft_disjoint(&mut body, &part).unwrap();
        let at_origin = |body: &Body<f64>, v: VertexKey| {
            let q = body.points[body.vertices[v].point];
            q.x.abs() + q.y.abs() + q.z.abs() < 1e-12
        };
        let corner = body
            .vertices
            .keys()
            .find(|v| !before.contains(v) && at_origin(&body, *v))
            .expect("the part's corner");
        let mut cones: Vec<VertexKey> = before
            .into_iter()
            .filter(|&v| at_origin(&body, v))
            .collect();
        assert_eq!(cones.len(), 2, "one vertex per cone at the pinch");
        // The second cube's shell joins the first's solid: one solid with
        // two cones.
        let solid = |body: &Body<f64>, v| census_entity_solid(body, EntityId::Vertex(v)).unwrap();
        let (keep, gone) = (solid(&body, cones[0]), solid(&body, cones[1]));
        for sh in body
            .solids
            .remove(gone)
            .expect("the second cube's solid")
            .shells
        {
            body.shells[sh].solid = keep;
            body.solids[keep].shells.push(sh);
        }
        body.solid_provenance.remove(gone);
        let geo = snapshot(&body);
        // The first cube's cone: its faces' normals point to `+` octant's outside.
        cones.sort_by_key(|&v| {
            !geo.vertex_faces[&v].iter().all(|&f| {
                crate::face_normal::face_outward_normal(&body, f)
                    .is_some_and(|m| m.vec().x + m.vec().y + m.vec().z < 0.0)
            })
        });
        let key = body.vertices[cones[0]].point;
        for v in [cones[1], corner] {
            let old = body.vertices[v].point;
            body.vertices[v].point = key;
            if old != key {
                body.points.remove(old);
            }
        }
        (body, corner, cones[0])
    }

    /// **A rest recorded at one cone of a pinch is read with the other**
    /// (`census-reads-only-the-recorded-vertex-pair-at-a-shared-point`).
    /// A parallelepiped's corner on the pinch's key, recorded against
    /// the first cube's cone, reaches into the second cube's octant
    /// (`a + b + c` points into it) and stays out of the first's. Read
    /// against the recorded vertex's cone alone that is a rest; the
    /// unit reads the solid's whole link at the point, both cones, and
    /// the touch is a crossing. The same corner turned into the empty
    /// octants only, `(−, +, ±)`, rests. Read at the touch: the two cones
    /// here are two shells of one solid, which tier 3 refuses before the
    /// census (a solid has one outer shell); a boolean's pinch makes the
    /// shape with one shell touching itself, and
    /// `join_pierce_runs_sweep::four_pairs_battery` reads those whole.
    #[test]
    fn a_rest_recorded_at_one_cone_of_a_pinch_reads_the_other() {
        let band = Band::linear(Tol::witness()).unwrap();
        for (a, b, c, crossing) in [
            ([0.6, -3.0, 0.3], [-3.0, 0.6, 0.3], [0.3, 0.3, -3.0], true),
            ([-0.1, 0.7, 0.1], [-0.6, 0.2, 0.1], [-0.2, 0.3, 0.6], false),
        ] {
            let (body, corner, cone) = pinch_and_corner(a, b, c);
            let geo = snapshot(&body);
            let ties = Ties {
                vv: vec![(corner, cone), (cone, corner)],
                ..Ties::default()
            };
            let verdict = TouchSite::VertexVertex(corner, cone).verdict(&body, &geo, &ties, band);
            if crossing {
                assert_eq!(verdict, TouchVerdict::Crossing);
            } else {
                assert!(verdict.is_rest(), "{verdict:?}");
            }
        }
    }

    /// **A point two solids share a key at is read, strictly.** A cube
    /// `[0, 1]³` and a parallelepiped whose corner shares the cube's
    /// origin KEY, no record and no finding naming it. In band, the
    /// parallelepiped lies at `y ≤ 0`, one face on the cube's extended
    /// plane `z = 0` and one standing off its extended plane `x = 0` by
    /// between `zero` and `escalate` all along its far side: the shared
    /// point reads in band, and tier 3′ refuses the pair — an undecided
    /// reading at a shared key is never taken on its word. Decided
    /// apart, the point rests and the pair clears. Red, the in-band pose
    /// clearing, where `blocks` reads no shared key or reads one
    /// leniently.
    #[test]
    fn a_shared_key_only_pair_reads_strictly() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let e = (band.zero() * band.escalate()).sqrt();
        for (label, a, b, c) in [
            (
                "in band",
                [-0.3, 0.0, 0.0],
                [e, -0.8, 0.0],
                [0.375 * e, -0.3, 0.3],
            ),
            (
                "decided apart",
                [-0.3, 0.0, 0.0],
                [-0.1, -0.8, 0.0],
                [-0.1, -0.3, 0.3],
            ),
        ] {
            let mut body =
                crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
            let before: BTreeSet<VertexKey> = body.vertices.keys().collect();
            let part = crate::test_support_fixtures::mapped_cube::<f64>(
                move |u, v, w| {
                    Point3::new(
                        u * a[0] + v * b[0] + w * c[0],
                        u * a[1] + v * b[1] + w * c[1],
                        u * a[2] + v * b[2] + w * c[2],
                    )
                },
                tol,
            );
            crate::instance::graft_disjoint(&mut body, &part).unwrap();
            let at_origin = |body: &Body<f64>, v: VertexKey| {
                let q = body.points[body.vertices[v].point];
                q.x == 0.0 && q.y == 0.0 && q.z == 0.0
            };
            let mine = body
                .vertices
                .keys()
                .find(|&v| before.contains(&v) && at_origin(&body, v))
                .unwrap();
            let theirs = body
                .vertices
                .keys()
                .find(|&v| !before.contains(&v) && at_origin(&body, v))
                .unwrap();
            let key = body.vertices[mine].point;
            let orphan = body.vertices[theirs].point;
            body.vertices[theirs].point = key;
            body.points.remove(orphan);
            let geo = snapshot(&body);
            let verdict =
                TouchSite::VertexVertex(mine, theirs).verdict(&body, &geo, &Ties::default(), band);
            let got = crate::validate_pseudomanifold(&body, &ContactRecords::default(), tol);
            if label == "in band" {
                assert_eq!(verdict, TouchVerdict::InBand);
                assert!(got.is_err(), "{label}: an in-band shared point clears");
            } else {
                assert!(
                    verdict.is_rest() && got.is_ok(),
                    "{label}: {verdict:?} {got:?}"
                );
            }
        }
    }

    /// **A block sunk into the inner corner of an L crosses there.** The
    /// block `[0.5, 1]² × [0, 0.5]` lies in the bracket's material, its
    /// corner on the saddle corner `(1, 1, 0)`: the touch there is a
    /// crossing, read as one, not refused as a saddle.
    #[test]
    fn a_block_sunk_into_the_inner_corner_crosses_there() {
        let body = bracket_and_block((0.5, 1.0), (0.5, 1.0), (0.0, 0.5));
        let geo = snapshot(&body);
        let at = |v: &VertexKey| {
            let q = geo.vmap[v];
            (q - Point3::new(1.0, 1.0, 0.0)).norm() < 1e-12
        };
        let got: Vec<TouchVerdict> = sites(&body)
            .into_iter()
            .filter(|(site, _)| matches!(site, TouchSite::VertexVertex(a, _) if at(a)))
            .map(|(_, v)| v)
            .collect();
        assert!(
            !got.is_empty() && got.iter().all(|v| *v == TouchVerdict::Crossing),
            "{got:?}"
        );
    }

    /// **Every planar touch kind reads both ways through the one
    /// analysis.** The block standing against the bracket's wall and
    /// the block seated in its inner corner (`contact1_touch_cones`)
    /// make every kind as a REST; the first block sunk into the bracket's leg
    /// makes the first three as CROSSINGS, and the straddling part
    /// (`bool4r2_probes`) makes a vertex on a face and an edge in a face
    /// as crossings. Each kind is pinned on its own finding here — the
    /// census row reports only the first refusal it reaches.
    #[test]
    fn every_touch_kind_reads_rest_and_crossing() {
        let rests = [
            touch_verdicts(&bracket_and_block((1.0, 2.5), (1.2, 3.0), (0.0, 1.0))),
            touch_verdicts(&bracket_and_block((1.0, 2.0), (1.0, 2.0), (0.2, 0.8))),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        assert!(rests.iter().all(|(_, v)| v.is_rest()), "{rests:?}");
        let crossings = [
            touch_verdicts(&bracket_and_block((0.0, 1.5), (1.0, 3.0), (0.0, 1.0))),
            touch_verdicts(&bracket_and_block((0.5, 1.5), (1.0, 3.0), (0.0, 1.0))),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        for kind in [
            "VertexVertex",
            "VertexOnEdge",
            "EdgeEdgeOverlap",
            "VertexOnFace",
            "EdgeFaceOverlap",
        ] {
            assert!(
                rests.iter().any(|(k, _)| *k == kind),
                "{kind} rests: {rests:?}"
            );
            assert!(
                crossings.contains(&(kind, TouchVerdict::Crossing)),
                "{kind}: {crossings:?}"
            );
        }
    }

    /// The bracket with a parallelepiped `p + u·a + v·b + w·c` grafted
    /// beside it (`det[a, b, c] > 0`).
    fn bracket_and_corner(p: [f64; 3], a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Body<f64> {
        let tol = Tol::witness();
        let mut bracket_only = crate::test_support_fixtures::prism_z::<f64>(
            &[
                (0.0, 0.0),
                (3.0, 0.0),
                (3.0, 1.0),
                (1.0, 1.0),
                (1.0, 3.0),
                (0.0, 3.0),
            ],
            0.0,
            1.0,
            tol,
        )
        .body;
        let part = crate::test_support_fixtures::mapped_cube::<f64>(
            move |u, v, w| {
                Point3::new(
                    p[0] + u * a[0] + v * b[0] + w * c[0],
                    p[1] + u * a[1] + v * b[1] + w * c[1],
                    p[2] + u * a[2] + v * b[2] + w * c[2],
                )
            },
            tol,
        );
        crate::instance::graft_disjoint(&mut bracket_only, &part).unwrap();
        bracket_only
    }

    /// **A long edge is read at its far end** (the dual review's
    /// dipping-sliver probe). A corner rests on the bracket's wall
    /// `x = 1` with a 3 cm edge off the wall and a 0.8 m edge DIPPING
    /// into the bracket by `e` at its far end — 13 and 24 times the
    /// zero threshold, past the escalation one. Read as a direction at
    /// the corner's shortest edge the dip would be `e · 0.03 / 0.8`,
    /// inside the zero band, "on the wall", and the corner would read as
    /// a rest over material it enters. Read as the far end's distance
    /// from the wall the dip is definite, and neither vertex-on-face
    /// touch is a rest.
    #[test]
    fn a_long_ray_dipping_into_the_other_is_never_a_rest() {
        let band = Band::linear(Tol::witness()).unwrap();
        for times in [13.0, 24.0] {
            let e = times * band.zero();
            let body = bracket_and_corner(
                [1.0, 2.0, 0.5],
                [0.03, 0.0, 0.0],
                [-e, 0.8, 0.0],
                [0.0, 0.0, 0.3],
            );
            let vf: Vec<TouchVerdict> = touch_verdicts(&body)
                .into_iter()
                .filter(|&(k, _)| k == "VertexOnFace")
                .map(|(_, v)| v)
                .collect();
            assert!(vf.len() >= 2, "{times}: {vf:?}");
            assert!(vf.iter().all(|v| !v.is_rest()), "{times}× zero: {vf:?}");
        }
    }

    /// **A touch whose side is in band refuses in band.** The same
    /// corner with the long edge's far end standing off the wall by the
    /// geometric mean of the band's two thresholds: read at its own
    /// length that is in band, so both vertex-on-face touches are
    /// [`TouchVerdict::InBand`]. (End to end the exact sweeps escalate
    /// on that far point first; this row pins the analysis's reading.)
    #[test]
    fn a_ray_whose_far_point_is_in_band_reads_in_band() {
        let band = Band::linear(Tol::witness()).unwrap();
        let e = (band.zero() * band.escalate()).sqrt();
        let body = bracket_and_corner(
            [1.0, 2.0, 0.5],
            [0.03, 0.0, 0.0],
            [e, 0.8, 0.0],
            [0.0, 0.0, 0.3],
        );
        let vf: Vec<TouchVerdict> = touch_verdicts(&body)
            .into_iter()
            .filter(|&(k, _)| k == "VertexOnFace")
            .map(|(_, v)| v)
            .collect();
        assert!(!vf.is_empty(), "{vf:?}");
        assert!(vf.iter().all(|v| *v == TouchVerdict::InBand), "{vf:?}");
    }

    // ---- Far dips, pinned per site where a direction read at a short
    // chord once hid them (each row goes red if a face is read through
    // the directions at the touch instead of its piece's vertices), and the
    // cost direction (ordinary rests stay rests).

    /// The placement findings of `body`: every solid-pair refusal and
    /// interference, rendered.
    fn placements_of(body: &Body<f64>) -> Vec<String> {
        crate::validate_pseudomanifold(body, &ContactRecords::default(), Tol::witness())
            .err()
            .unwrap_or_default()
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    ValidationError::CensusUndecidable {
                        a: EntityId::Solid(_),
                        ..
                    } | ValidationError::InstanceInterference { .. }
                )
            })
            .map(|e| format!("{e:?}"))
            .collect()
    }

    /// The piece's points of the face of `v`'s link whose outward
    /// normal is `m`.
    fn piece_at(body: &Body<f64>, v: VertexKey, m: Vec3<f64>) -> Vec<Point3<f64>> {
        let band = Band::linear(Tol::witness()).unwrap();
        let geo = snapshot(body);
        let link = vertex_link(body, &geo, v, band).expect("a planar corner");
        let face = link
            .sectors
            .iter()
            .find(|f| (f.outward - m).norm() < 1e-9)
            .expect("the face");
        face.wedges.iter().flat_map(|&(a, b)| [a, b]).collect()
    }

    /// The link of `v`'s solid at `v`'s point key alone, no tie read.
    fn vertex_link(
        body: &Body<f64>,
        geo: &Geo<f64>,
        v: VertexKey,
        band: Band,
    ) -> Result<Link<f64>, TouchVerdict> {
        let keys = BTreeSet::from([geo.vpoint[&v]]);
        let solid = census_entity_solid(body, EntityId::Vertex(v)).expect("a solid's vertex");
        Link::at_point(
            body,
            geo,
            &Ties::default(),
            solid,
            &keys,
            geo.vmap[&v],
            band,
        )
    }

    /// **A notched bracket rests against the wall, read through its
    /// piece.** The L-bracket with a notch cut into its leg's end,
    /// `[0.4, 0.6] × [2.6, 3]`, a block standing against its inner wall
    /// `x = 1` as in `contact1_touch_cones`. At the leg's corner `(1, 3)`
    /// the bottom face reaches back across the wall plane `x = 1` (its
    /// foot runs to `x = 3`) and behind the notch; its piece there holds
    /// neither — not the foot's far corner, not the corner `(0, 3)` the
    /// notch shadows — and lies on the bracket's side of the wall. Every
    /// touch is a rest and the pair clears. Read whole, the face refused
    /// it.
    #[test]
    fn a_notched_bracket_rests_against_the_wall() {
        let tol = Tol::witness();
        const NOTCHED: [(f64, f64); 10] = [
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.6, 3.0),
            (0.6, 2.6),
            (0.4, 2.6),
            (0.4, 3.0),
            (0.0, 3.0),
        ];
        let mut body = crate::test_support_fixtures::prism_z::<f64>(&NOTCHED, 0.0, 1.0, tol).body;
        let part =
            crate::test_support_fixtures::brick::<f64>((1.0, 2.5), (1.2, 3.0), (0.0, 1.0), tol);
        crate::instance::graft_disjoint(&mut body, &part).unwrap();
        let corner = body
            .vertices
            .iter()
            .filter(|(_, v)| (body.points[v.point] - Point3::new(1.0, 3.0, 0.0)).norm() < 1e-9)
            .map(|(k, _)| k)
            .find(|&k| {
                geo_faces(&body, k)
                    .iter()
                    .any(|&m| (m - Vec3::new(1.0, 0.0, 0.0)).norm() < 1e-9)
            })
            .expect("the bracket's corner");
        let piece = piece_at(&body, corner, Vec3::new(0.0, 0.0, -1.0));
        assert!(piece.iter().all(|q| q.x <= 1.0 + 1e-9), "{piece:?}");
        for hidden in [Point3::new(3.0, 0.0, 0.0), Point3::new(0.0, 3.0, 0.0)] {
            assert!(
                piece.iter().all(|&q| (q - hidden).norm() > 1e-9),
                "{hidden:?} is not seen from the corner: {piece:?}"
            );
        }
        let got = sites(&body);
        assert!(!got.is_empty());
        assert!(got.iter().all(|(_, t)| t.is_rest()), "{got:?}");
        assert_eq!(placements_of(&body), Vec::<String>::new());
    }

    /// The outward normals of the faces around vertex `v`.
    fn geo_faces(body: &Body<f64>, v: VertexKey) -> Vec<Vec3<f64>> {
        snapshot(body)
            .vertex_faces
            .get(&v)
            .into_iter()
            .flatten()
            .filter_map(|&f| crate::face_normal::face_outward_normal(body, f))
            .map(|m| m.vec())
            .collect()
    }

    /// **A holed block rests beside a brick, read through its piece.**
    /// A block `[0, 4] × [0, 2] × [0, 2]` drilled through by the square
    /// `[1.5, 2.5] × [0.5, 1.5]`, a brick standing flush against its
    /// face `x = 0`. At the corner `(0, 0, 2)` the top face's piece
    /// stops at the hole: the far corner `(4, 2, 2)`, straight behind
    /// the hole's centre, is not in it, and the hole's corners cast
    /// windows onto the face's far edges. Every touch is a rest and the
    /// pair clears.
    #[test]
    fn a_holed_block_rests_beside_a_brick() {
        let tol = Tol::witness();
        let mut body = crate::test_support_fixtures::holed_block::<f64>(4.0, &[2.0], tol);
        crate::test_support_fixtures::describe_as_intersections(&mut body, tol);
        let part =
            crate::test_support_fixtures::brick::<f64>((-1.0, 0.0), (0.0, 2.0), (0.0, 2.0), tol);
        crate::instance::graft_disjoint(&mut body, &part).unwrap();
        let corner = body
            .vertices
            .iter()
            .filter(|(_, v)| (body.points[v.point] - Point3::new(0.0, 0.0, 2.0)).norm() < 1e-9)
            .map(|(k, _)| k)
            .find(|&k| {
                geo_faces(&body, k)
                    .iter()
                    .any(|&m| (m - Vec3::new(-1.0, 0.0, 0.0)).norm() < 1e-9)
            })
            .expect("the holed block's corner");
        let piece = piece_at(&body, corner, Vec3::new(0.0, 0.0, 1.0));
        assert!(
            piece
                .iter()
                .all(|&q| (q - Point3::new(4.0, 2.0, 2.0)).norm() > 1e-9),
            "the far corner is behind the hole: {piece:?}"
        );
        assert!(
            piece
                .iter()
                .any(|q| (q.x - 4.0).abs() < 1e-9 && q.y > 1e-9 && q.y < 2.0 - 1e-9),
            "a window on the far edge: {piece:?}"
        );
        let got = sites(&body);
        assert!(!got.is_empty());
        assert!(got.iter().all(|(_, t)| t.is_rest()), "{got:?}");
        assert_eq!(placements_of(&body), Vec::<String>::new());
    }

    /// **A wall rest off by a band width never rests.** The block
    /// standing against the bracket's wall, its face on the wall turned
    /// about its vertical edge at `y = 1.2` so its far corners stand off the wall
    /// (or into it) by the geometric mean of the band's two thresholds:
    /// the exact separating plane, `x = 1`, is read in band at those
    /// corners. The bracket's bottom and top faces reach back across
    /// that plane. No touch reads as a rest and the pair does not
    /// clear.
    #[test]
    fn a_wall_rest_off_by_a_band_width_never_rests() {
        let band = Band::linear(Tol::witness()).unwrap();
        let e = (band.zero() * band.escalate()).sqrt();
        for e in [e, -e] {
            let body = bracket_and_corner(
                [1.0, 1.2, 0.0],
                [1.5, 0.0, 0.0],
                [e, 1.8, 0.0],
                [0.0, 0.0, 1.0],
            );
            let got = sites(&body);
            assert!(got.iter().all(|(_, t)| !t.is_rest()), "{e}: {got:?}");
            assert!(!placements_of(&body).is_empty(), "{e}: cleared");
        }
    }

    /// A prism over `profile`, `z ∈ [0, 1]`, with a brick grafted beside
    /// it.
    fn prism_and_brick(
        profile: &[(f64, f64)],
        x: (f64, f64),
        y: (f64, f64),
        z: (f64, f64),
    ) -> Body<f64> {
        let tol = Tol::witness();
        let mut body = crate::test_support_fixtures::prism_z::<f64>(profile, 0.0, 1.0, tol).body;
        let part = crate::test_support_fixtures::brick::<f64>(x, y, z, tol);
        crate::instance::graft_disjoint(&mut body, &part).unwrap();
        body
    }

    /// **An edge lying behind the touch point never blocks its face's
    /// piece.** A brick `[2, 3] × [0, 1] × [1, 2]` stands on a U-channel's
    /// top face. Its bottom edge `y = 1` lies in that face, and the touch
    /// is read at `(2.5, 1, 1)`, where one of the piece's probe rays runs
    /// along `+x`: the slot's floor edge `(1, 1)–(2, 1)` lies on that
    /// ray's line, BEHIND the point. It can never meet the ray; read as
    /// undecided, it refused the rest. Every touch is a rest and the
    /// pair clears.
    #[test]
    fn a_brick_on_a_u_channel_rests_past_an_edge_behind_the_touch() {
        const U: [(f64, f64); 8] = [
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 2.0),
            (2.0, 2.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ];
        let body = prism_and_brick(&U, (2.0, 3.0), (0.0, 1.0), (1.0, 2.0));
        let got = sites(&body);
        assert!(
            got.iter()
                .any(|(s, _)| matches!(s, TouchSite::EdgeInFace(..))),
            "{got:?}"
        );
        assert!(got.iter().all(|(_, t)| t.is_rest()), "{got:?}");
        assert_eq!(placements_of(&body), Vec::<String>::new());
    }

    /// **The same, on a comb, at an edge and at a corner.** A brick
    /// `[0.25, 0.75] × [0.75, 1.25] × [1, 2]` stands on a comb's top face
    /// over its first tooth's inner corner `(0.5, 1)`, overhanging the
    /// gap between two teeth. The edge-in-face touch at `(0.25, 1, 1)`
    /// and the comb's corner on the brick's face at `(0.5, 1, 1)` each
    /// build pieces whose probe rays run along the teeth's collinear
    /// root edges behind the point. Every touch is a rest and the pair
    /// clears.
    #[test]
    fn a_brick_on_a_comb_rests_past_edges_behind_the_touch() {
        const COMB: [(f64, f64); 12] = [
            (0.0, 0.0),
            (4.0, 0.0),
            (4.0, 2.0),
            (3.5, 2.0),
            (3.5, 1.0),
            (2.5, 1.0),
            (2.5, 2.0),
            (1.5, 2.0),
            (1.5, 1.0),
            (0.5, 1.0),
            (0.5, 2.0),
            (0.0, 2.0),
        ];
        let body = prism_and_brick(&COMB, (0.25, 0.75), (0.75, 1.25), (1.0, 2.0));
        let got = sites(&body);
        for kind in ["EdgeInFace", "VertexOnFace"] {
            assert!(
                got.iter().any(|(s, _)| format!("{s:?}").starts_with(kind)),
                "{kind}: {got:?}"
            );
        }
        assert!(got.iter().all(|(_, t)| t.is_rest()), "{got:?}");
        assert_eq!(placements_of(&body), Vec::<String>::new());
    }

    /// **A guest seated into a reflex edge is read by that edge's
    /// convexity.** A prism over `(1, 1), (0.5, 1.5), (0.2, 1.2), (0.7,
    /// 0.7)`, `z ∈ [0.25, 0.75]`, its corner `(1, 1)` on the L-bracket's
    /// reflex edge and its body reaching into the bracket's leg. At the
    /// guest's corner the bracket's link holds the reflex edge's two
    /// faces, and the guest's pieces read inside the bracket's material
    /// across that edge: a crossing. Read as if the edge were convex,
    /// the corner would rest although the guest sits inside the bracket.
    #[test]
    fn a_guest_seated_into_a_reflex_edge_is_not_a_rest() {
        let tol = Tol::witness();
        const L: [(f64, f64); 6] = [
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ];
        let mut body = crate::test_support_fixtures::prism_z::<f64>(&L, 0.0, 1.0, tol).body;
        let guest = crate::test_support_fixtures::prism_z::<f64>(
            &[(1.0, 1.0), (0.5, 1.5), (0.2, 1.2), (0.7, 0.7)],
            0.25,
            0.75,
            tol,
        )
        .body;
        crate::instance::graft_disjoint(&mut body, &guest).unwrap();
        let got = sites(&body);
        assert!(!got.is_empty());
        assert!(got.iter().all(|(_, t)| !t.is_rest()), "{got:?}");
        assert!(
            got.iter().any(|(_, t)| *t == TouchVerdict::Crossing),
            "{got:?}"
        );
    }

    /// A prism over `profile`, `z` its extrusion, through `map`, into
    /// `body` (a new solid).
    fn mapped_prism(
        body: &mut Body<f64>,
        profile: &[(f64, f64)],
        z: (f64, f64),
        map: impl Fn(f64, f64, f64) -> Point3<f64>,
    ) {
        use crate::test_support_fixtures::{FaceGeometry, describe_as_intersections, prism_ops};
        prism_ops(
            body,
            profile,
            z,
            map,
            FaceGeometry::Certified,
            Tol::witness(),
        );
        describe_as_intersections(body, Tol::witness());
    }

    /// `part` over a floor slab `[-1, 2]² × [-1, 0]`.
    fn on_floor(part: impl Fn(&mut Body<f64>)) -> Body<f64> {
        let mut body = Body::<f64>::new();
        let square = [(-1.0, -1.0), (2.0, -1.0), (2.0, 2.0), (-1.0, 2.0)];
        mapped_prism(&mut body, &square, (-1.0, 0.0), |x, y, z| {
            Point3::new(x, y, z)
        });
        let mut other = Body::<f64>::new();
        part(&mut other);
        crate::instance::graft_disjoint(&mut body, &other).unwrap();
        body
    }

    fn sites(body: &Body<f64>) -> Vec<(TouchSite, TouchVerdict)> {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let geo = snapshot(body);
        let contacts = ContactRecords::default();
        let findings = census_and_certify(body, &contacts, band, tol, None);
        let ties = Ties::of(&Declared::index(&geo, &contacts), &findings);
        findings
            .iter()
            .filter_map(|e| match e {
                ValidationError::UnrecordedContact { contact, .. } => TouchSite::of(contact),
                _ => None,
            })
            .map(|site| (site, site.verdict(body, &geo, &ties, band)))
            .collect()
    }

    /// **A reflex notch is read at its face's far corner** (the delta
    /// review's P1). An L-plate, 0.8 m square less a 3 cm notch, rests its
    /// reflex notch vertex on the floor; its bottom face is sheared so
    /// the far corner, about 1.09 m from the notch, dips below the floor
    /// by 2 to 24 times the zero threshold. The notch's chords are 3 cm,
    /// and read as directions at those the dip is a fraction of the band
    /// — "on the floor". The bottom face's piece at the notch holds the
    /// far corner, whose distance is not, and no touch reads as a rest.
    #[test]
    fn a_reflex_notch_dipping_far_off_is_never_a_rest() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (s, n) = (0.8, 0.03);
        let prof = [
            (0.0, 0.0),
            (s, 0.0),
            (s, s - n),
            (s - n, s - n),
            (s - n, s),
            (0.0, s),
        ];
        let v = s - n;
        for times in [2.0, 5.0, 13.0, 24.0] {
            let phi = times * band.zero() / 1.09;
            let body = on_floor(|b| {
                mapped_prism(b, &prof, (0.0, 0.03), |x, y, z| {
                    let t = phi * ((x - v) + (y - v)) / 2f64.sqrt();
                    Point3::new(x, y, z + t)
                })
            });
            let got = sites(&body);
            assert!(!got.is_empty(), "{times}");
            assert!(
                got.iter().all(|(_, t)| !t.is_rest()),
                "{times}× zero: {got:?}"
            );
        }
    }

    /// **A convex corner is read at its faces' far vertices** (the delta
    /// review's P5). A plate's 90° tip, 3 cm chords, rests on the floor;
    /// the plate runs 1 m back and its bottom dips toward the back by
    /// 5 to 24 times the zero threshold. No touch reads as a rest.
    #[test]
    fn a_convex_corner_dipping_far_off_is_never_a_rest() {
        let band = Band::linear(Tol::witness()).unwrap();
        let prof = [
            (1.0, 0.5),
            (0.97, 0.53),
            (0.0, 1.0),
            (0.0, 0.0),
            (0.97, 0.47),
        ];
        for times in [5.0, 13.0, 24.0] {
            let phi = times * band.zero();
            let body = on_floor(|b| {
                mapped_prism(b, &prof, (0.0, 0.03), |x, y, z| {
                    Point3::new(x, y, z + phi * (x - 1.0))
                })
            });
            let got = sites(&body);
            assert!(!got.is_empty(), "{times}");
            assert!(
                got.iter().all(|(_, t)| !t.is_rest()),
                "{times}× zero: {got:?}"
            );
        }
    }

    /// **An edge in a face is read at its faces' far vertices.** A
    /// plate's 3 cm front edge lies in the floor; the plate runs 1 m back
    /// and its bottom dips toward the back by 13 and 24 times the zero
    /// threshold. Read as a direction at the edge's 3 cm the bottom
    /// face's in-face direction is on the floor; its piece's far vertices
    /// dip, and the edge-in-face touch is not a rest.
    #[test]
    fn an_edge_in_a_face_dipping_far_off_is_never_a_rest() {
        let band = Band::linear(Tol::witness()).unwrap();
        let prof = [(1.0, 0.485), (1.0, 0.515), (0.0, 1.0), (0.0, 0.0)];
        for times in [13.0, 24.0] {
            let phi = times * band.zero();
            let body = on_floor(|b| {
                mapped_prism(b, &prof, (0.0, 0.03), |x, y, z| {
                    Point3::new(x, y, z + phi * (x - 1.0))
                })
            });
            let got = sites(&body);
            assert!(
                got.iter()
                    .any(|(s, _)| matches!(s, TouchSite::EdgeInFace(..))),
                "{times}: {got:?}"
            );
            let edges: Vec<TouchVerdict> = got
                .into_iter()
                .filter(|(s, _)| matches!(s, TouchSite::EdgeInFace(..)))
                .map(|(_, t)| t)
                .collect();
            assert!(
                edges.iter().all(|t| !t.is_rest()),
                "{times}× zero: {edges:?}"
            );
        }
    }

    /// Where a synthetic link's point sits on its faces.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum At {
        /// At the vertex with this index.
        Vertex(usize),
        /// Inside the edge `edges[0]` names.
        Edge,
        /// Inside the face.
        Face,
    }

    /// A link built from bare geometry: each face `(outward, loops)`
    /// over `verts` (walked with its interior on the left), its piece
    /// built by [`face_piece`]; `edges[0]`'s ends name the edge `p` is
    /// inside, at [`At::Edge`].
    #[allow(clippy::type_complexity)]
    fn synthetic(
        p: Point3<f64>,
        at: At,
        verts: Vec<Point3<f64>>,
        faces: &[(Vec3<f64>, Vec<Vec<usize>>)],
        edges: &[(Vec<usize>, [usize; 2])],
        band: Band,
    ) -> Result<Link<f64>, TouchVerdict> {
        let mut sectors = Vec::new();
        for (m, loops) in faces {
            let rings: Vec<Vec<Point3<f64>>> = loops
                .iter()
                .map(|ring| ring.iter().map(|&i| verts[i]).collect())
                .collect();
            let mut place = PieceAt::Inside;
            for (r, ring) in loops.iter().enumerate() {
                let n = ring.len();
                for k in 0..n {
                    match at {
                        At::Vertex(i) if ring[k] == i => place = PieceAt::Corner(r, k),
                        At::Edge => {
                            let ends = &edges[0].0;
                            let (x, y) = (ring[k], ring[(k + 1) % n]);
                            if (x == ends[0] && y == ends[1]) || (x == ends[1] && y == ends[0]) {
                                place = PieceAt::Edge(r, k);
                            }
                        }
                        _ => {}
                    }
                }
            }
            let piece = face_piece(p, *m, &rings, place, &[], band)?;
            sectors.push(Sector {
                surface: Surface::Plane {
                    origin: verts[loops[0][0]],
                    normal: *m,
                    u_ref: m.cross(Vec3::new(1.0, 1.0, 1.0)).normalize(),
                },
                sense: true,
                outward: *m,
                apex: p,
                wedges: piece.chunks_exact(2).map(|w| (w[0], w[1])).collect(),
                closed: matches!(place, PieceAt::Inside),
            });
        }
        Link::of(p, sectors)
    }

    /// A floor face through the origin, `[-2, 2]²` at `z = 0`, its
    /// material below.
    fn floor_link(band: Band) -> Link<f64> {
        let verts = vec![
            Point3::new(-2.0, -2.0, 0.0),
            Point3::new(2.0, -2.0, 0.0),
            Point3::new(2.0, 2.0, 0.0),
            Point3::new(-2.0, 2.0, 0.0),
        ];
        synthetic(
            Point3::new(0.0, 0.0, 0.0),
            At::Face,
            verts,
            &[(Vec3::new(0.0, 0.0, 1.0), vec![vec![0, 1, 2, 3]])],
            &[],
            band,
        )
        .expect("a floor")
    }

    /// A shallow pit's apex at the origin: three edges rising `h` over
    /// 1 m at 120° apart, bounding three triangles whose outward normals
    /// point up (material below the pit's surface).
    fn shallow_pit(h: f64, band: Band) -> Link<f64> {
        let q = |deg: f64| {
            let t = deg.to_radians();
            Point3::new(t.cos(), t.sin(), h)
        };
        let verts = vec![Point3::new(0.0, 0.0, 0.0), q(0.0), q(120.0), q(240.0)];
        // Triangle k is (apex, q_k, q_{k+1}), counterclockwise from above.
        let tri = |k: usize| {
            let (a, b) = (1 + k, 1 + (k + 1) % 3);
            let m = (verts[a] - verts[0]).cross(verts[b] - verts[0]).normalize();
            (m, vec![vec![0, a, b]])
        };
        let faces = [tri(0), tri(1), tri(2)];
        let edges = [(vec![1], [2, 0]), (vec![2], [0, 1]), (vec![3], [1, 2])];
        synthetic(verts[0], At::Vertex(0), verts.clone(), &faces, &edges, band).expect("a pit")
    }

    /// **A barely reflex corner is read at its faces' far vertices.** The
    /// apex of a shallow pit — every edge reflex by `400·zero` over a
    /// metre, the complement a thin convex cone — touching a floor whose
    /// material is below it: the pit's material is below its surface
    /// too, so the two overlap, and the answer is a crossing. Read at a
    /// short lever the edges would read flat, the pit a half-space lying
    /// on the floor, and the touch a rest.
    #[test]
    fn a_shallow_pit_is_read_at_its_far_vertices() {
        let band = Band::linear(Tol::witness()).unwrap();
        let pit = shallow_pit(400.0 * band.zero(), band);
        assert_eq!(
            rest::judge(&pit, &floor_link(band), band),
            TouchVerdict::Crossing
        );
    }

    /// **The cuts a piece needs: a pinwheel.** Two pyramids with their
    /// apex at the origin, over parallelograms in the plane `z = 1` — `A`
    /// over `(0, 0), (3, 0), (1, 1), (−2, 1)` and `B` over `(1, 0), (1,
    /// 3), (0, 1), (0, −2)` — overlap in the cone over the unit square,
    /// and no face of one crosses a face of the other: at each corner of
    /// the square one pyramid's edge lies in a face of the other, which
    /// runs on past it. Every face's middle lies off the square, so read
    /// whole every face reads apart. Cut at the other's edges lying on
    /// it, each face's piece along a side of the square reads inside the
    /// other: a crossing. Red where a sector is not cut at the rays lying
    /// on it — in band, not a rest: a face read whole meets the other's
    /// edge lying on it at a point on that edge's ray, which rule (1)
    /// reads neither inside nor outside the face, and notes. So the cut
    /// buys the crossings, and no rest stands on it.
    #[test]
    fn a_pinwheel_of_two_pyramids_crosses_through_its_cuts() {
        let band = Band::linear(Tol::witness()).unwrap();
        let pyramid = |base: [(f64, f64); 4]| {
            let mut verts = vec![Point3::new(0.0, 0.0, 0.0)];
            verts.extend(base.map(|(x, y)| Point3::new(x, y, 1.0)));
            let faces: Vec<(Vec3<f64>, Vec<Vec<usize>>)> = (0..4)
                .map(|k| {
                    let (a, b) = (1 + k, 1 + (k + 1) % 4);
                    // The material is inside the cone: each side's
                    // outward normal is `−(a × b)`, its loop `(0, b, a)`.
                    let m = -(verts[a] - verts[0]).cross(verts[b] - verts[0]).normalize();
                    (m, vec![vec![0, b, a]])
                })
                .collect();
            synthetic(verts[0], At::Vertex(0), verts.clone(), &faces, &[], band)
                .expect("a pyramid's link")
        };
        let a = pyramid([(0.0, 0.0), (3.0, 0.0), (1.0, 1.0), (-2.0, 1.0)]);
        let b = pyramid([(1.0, 0.0), (1.0, 3.0), (0.0, 1.0), (0.0, -2.0)]);
        assert_eq!(rest::judge(&a, &b, band), TouchVerdict::Crossing);
        assert_eq!(rest::judge(&b, &a, band), TouchVerdict::Crossing);
    }

    /// **A link whose faces do not close is refused, not read.** A
    /// square pyramid stands on its apex on the floor. With its four
    /// sides it rests. With one side left out, the three left bound no
    /// region of directions: each bound of the missing side is the bound
    /// of one sector only, and crossing-parity reads nothing true of
    /// them. That link refuses `Unanalysed` ([`Link::closes`]). Red
    /// where every link is taken to close: the open fan's parity then
    /// reads the floor's directions inside the pyramid, a crossing.
    #[test]
    fn a_pyramid_missing_a_side_is_not_read() {
        let band = Band::linear(Tol::witness()).unwrap();
        let pyramid = |skip: Option<usize>| {
            let mut verts = vec![Point3::new(0.0, 0.0, 0.0)];
            verts.extend(
                [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)]
                    .map(|(x, y)| Point3::new(x, y, 1.0)),
            );
            let faces: Vec<(Vec3<f64>, Vec<Vec<usize>>)> = (0..4)
                .filter(|&k| Some(k) != skip)
                .map(|k| {
                    let (a, b) = (1 + k, 1 + (k + 1) % 4);
                    let m = -(verts[a] - verts[0]).cross(verts[b] - verts[0]).normalize();
                    (m, vec![vec![0, b, a]])
                })
                .collect();
            synthetic(verts[0], At::Vertex(0), verts.clone(), &faces, &[], band)
                .expect("a pyramid's faces")
        };
        assert!(matches!(
            rest::judge(&pyramid(None), &floor_link(band), band),
            TouchVerdict::Rest(_)
        ));
        assert_eq!(
            rest::judge(&pyramid(Some(0)), &floor_link(band), band),
            TouchVerdict::Unanalysed
        );
    }

    /// **A tie's distance is not read as a side.** The floor, against
    /// another solid's face lying face down `t` below it (ten band
    /// widths), whose corner stands on another key of the point, decided
    /// one point with the floor's (a tie of `t`). Read about the floor's
    /// point, the face lies `t` inside the floor; but `t` is only how
    /// far the tie moved its corner, so the reading over the tie refuses
    /// in band. Red where the slack is zero: the face's probe then reads
    /// inside the floor, a crossing on the tie alone.
    #[test]
    fn a_face_a_tie_below_the_floor_reads_in_band() {
        let band = Band::linear(Tol::witness()).unwrap();
        let t = 10.0 * band.zero();
        let verts = [(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)]
            .map(|(x, y)| Point3::new(x, y, -t))
            .to_vec();
        let face = synthetic(
            Point3::new(0.0, 0.0, -t),
            At::Face,
            verts,
            &[(Vec3::new(0.0, 0.0, -1.0), vec![vec![0, 3, 2, 1]])],
            &[],
            band,
        )
        .expect("a face");
        let face = Link::of(Point3::new(0.0, 0.0, 0.0), face.sectors).expect("a link");
        assert!(face.tie > 0.0);
        assert_eq!(
            rest::judge(&face, &floor_link(band), band),
            TouchVerdict::InBand
        );
    }

    /// **A slit is named, not read as a wall.** Two faces lying on each
    /// other in the plane `x = 0` over the edge along `y`: one rectangle
    /// above the edge, once with its outward normal `−x` and once with
    /// `+x`, against the floor. Each face's probe lies on the other face,
    /// so the link refuses as degenerate. The same face above the edge
    /// beside a coplanar one below it, both `−x`, is a flat wall whose
    /// material, `x > 0`, overlaps the floor's: a crossing.
    #[test]
    fn a_slit_reads_degenerate() {
        let band = Band::linear(Tol::witness()).unwrap();
        let verts = vec![
            Point3::new(0.0, -1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 1.0),
            Point3::new(0.0, -1.0, 1.0),
            Point3::new(0.0, 1.0, -1.0),
            Point3::new(0.0, -1.0, -1.0),
        ];
        let x = Vec3::new(1.0, 0.0, 0.0);
        let above = (-x, vec![vec![1, 0, 3, 2]]);
        let link = |other: (Vec3<f64>, Vec<Vec<usize>>)| {
            synthetic(
                Point3::new(0.0, 0.0, 0.0),
                At::Edge,
                verts.clone(),
                &[above.clone(), other],
                &[(vec![0, 1], [0, 1])],
                band,
            )
            .expect("a link")
        };
        let floor = floor_link(band);
        assert_eq!(
            rest::judge(&link((x, vec![vec![0, 1, 2, 3]])), &floor, band),
            TouchVerdict::Degenerate
        );
        assert_eq!(
            rest::judge(&link((-x, vec![vec![0, 1, 4, 5]])), &floor, band),
            TouchVerdict::Crossing
        );
    }

    /// **A corner whose edges are not numbers refuses on scale.** A
    /// brick shrunk to 1e-170 m (its chords underflow) and one blown up
    /// to 1e200 m (they overflow): the corner's link refuses as
    /// [`TouchVerdict::ChordScale`], the rescale recourse.
    #[test]
    fn a_corner_out_of_scale_refuses_on_scale() {
        let band = Band::linear(Tol::witness()).unwrap();
        for scale in [1e-170, 1e200] {
            let mut body = crate::test_support_fixtures::brick::<f64>(
                (0.0, 1.0),
                (0.0, 1.0),
                (0.0, 1.0),
                Tol::witness(),
            );
            for p in body.points.values_mut() {
                *p = Point3::new(p.x * scale, p.y * scale, p.z * scale);
            }
            let geo = snapshot(&body);
            let (v, _) = body.vertices.iter().next().unwrap();
            assert!(
                matches!(
                    vertex_link(&body, &geo, v, band),
                    Err(TouchVerdict::ChordScale)
                ),
                "{scale}"
            );
        }
    }

    fn rot(axis: [f64; 3], ang: f64) -> impl Fn([f64; 3]) -> [f64; 3] + Copy {
        let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        let k = [axis[0] / n, axis[1] / n, axis[2] / n];
        let (c, s) = (ang.cos(), ang.sin());
        move |v: [f64; 3]| {
            let kv = k[0] * v[0] + k[1] * v[1] + k[2] * v[2];
            let cr = [
                k[1] * v[2] - k[2] * v[1],
                k[2] * v[0] - k[0] * v[2],
                k[0] * v[1] - k[1] * v[0],
            ];
            [
                v[0] * c + cr[0] * s + k[0] * kv * (1.0 - c),
                v[1] * c + cr[1] * s + k[1] * kv * (1.0 - c),
                v[2] * c + cr[2] * s + k[2] * kv * (1.0 - c),
            ]
        }
    }

    type Pose = (Vec<(f64, f64)>, (f64, f64), fn(f64, f64, f64) -> [f64; 3]);

    /// **Ordinary rests stay rests** (the delta review's battery — the
    /// cost direction). Each pose, under the
    /// identity and two generic rotations, reads a rest at every touch
    /// site and carries no finding but the unrecorded touches.
    #[test]
    fn ordinary_rests_stay_rests() {
        const L: [(f64, f64); 6] = [
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ];
        let sq = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let bracket: Pose = (L.to_vec(), (0.0, 1.0), |x, y, z| [x, y, z]);
        let cases: Vec<(&str, Pose, Pose)> = vec![
            (
                "block standing on the wall",
                bracket.clone(),
                (sq.clone(), (0.0, 1.0), |u, v, w| {
                    [1.0 + 1.5 * u, 1.2 + 1.8 * v, w]
                }),
            ),
            (
                "block seated in the inner corner",
                bracket.clone(),
                (sq.clone(), (0.2, 0.8), |u, v, w| [1.0 + u, 1.0 + v, w]),
            ),
            (
                "box edge on a box face",
                (sq.clone(), (-1.0, 0.0), |x, y, z| {
                    [4.0 * x - 2.0, 4.0 * y - 2.0, z]
                }),
                (sq.clone(), (0.0, 1.0), |u, v, w| {
                    let s2 = 0.5f64.sqrt();
                    [(u - v) * s2 * 0.5, 0.5 - w * 0.5, (u + v) * s2 * 0.5]
                }),
            ),
            (
                "tilted cube corner on a cube corner",
                (sq.clone(), (-1.0, 0.0), |x, y, z| [x, y, z]),
                (sq.clone(), (0.0, 1.0), |u, v, w| {
                    let s = core::f64::consts::FRAC_1_SQRT_2;
                    let a = [s, -s, 0.0];
                    let b = [
                        0.408_248_290_463_863,
                        0.408_248_290_463_863,
                        -0.816_496_580_927_726,
                    ];
                    let c = [0.577_350_269_189_625_8; 3];
                    let d = [u * 0.3, v * 0.3, w * 0.3];
                    [
                        1.0 + d[0] * a[0] + d[1] * a[1] + d[2] * a[2],
                        1.0 + d[0] * b[0] + d[1] * b[1] + d[2] * b[2],
                        d[0] * c[0] + d[1] * c[1] + d[2] * c[2],
                    ]
                }),
            ),
            (
                "2.6 m rod on the bracket top",
                bracket.clone(),
                (sq.clone(), (0.0, 0.01), |u, v, w| {
                    [0.2 + 2.6 * u, 0.4 + 0.01 * v, 1.0 + w]
                }),
            ),
            (
                "1.9 m rod in the inner corner",
                bracket.clone(),
                (sq.clone(), (0.0, 1.0), |u, v, w| {
                    [1.0 + 1.9 * u, 1.0 + 0.01 * v, 0.3 + 0.01 * w]
                }),
            ),
            (
                "1 mm plate on the leg top",
                bracket.clone(),
                (sq.clone(), (0.0, 1.0), |u, v, w| {
                    [0.1 + 0.8 * u, 1.1 + 1.8 * v, 1.0 + 0.001 * w]
                }),
            ),
        ];
        for (name, a, b) in &cases {
            for (axis, ang) in [
                ([0.0, 0.0, 1.0], 0.0),
                ([1.0, 2.0, 3.0], 0.7),
                ([-0.3, 1.0, 0.2], 2.1),
            ] {
                let r = rot(axis, ang);
                let mut body = Body::<f64>::new();
                let (pa, za, ma) = a.clone();
                mapped_prism(&mut body, &pa, za, move |x, y, z| {
                    let q = r(ma(x, y, z));
                    Point3::from_array(q)
                });
                let mut other = Body::<f64>::new();
                let (pb, zb, mb) = b.clone();
                mapped_prism(&mut other, &pb, zb, move |x, y, z| {
                    let q = r(mb(x, y, z));
                    Point3::from_array(q)
                });
                crate::instance::graft_disjoint(&mut body, &other).unwrap();
                let got = sites(&body);
                assert!(!got.is_empty(), "{name} @ {ang}");
                assert!(
                    got.iter().all(|(_, t)| t.is_rest()),
                    "{name} @ {ang}: {got:?}"
                );
                let others: Vec<String> = crate::validate_pseudomanifold(
                    &body,
                    &ContactRecords::default(),
                    Tol::witness(),
                )
                .err()
                .unwrap_or_default()
                .iter()
                .filter(|e| !matches!(e, ValidationError::UnrecordedContact { .. }))
                .map(|e| format!("{e:?}"))
                .collect();
                assert!(others.is_empty(), "{name} @ {ang}: {others:?}");
            }
        }
    }

    /// **An obtuse sector is read at its face's far vertices.** A prism
    /// over `(0,0), (1,0), (1,1), (−1,1), (−1,δ)` rests its corner
    /// `(0,0)` on the floor; the corner's bottom sector is just under
    /// 180° and the bottom face is sheared so its far edge dips 2, 5,
    /// 30 and 500 times the zero threshold below the floor. Both of the
    /// sector's bounding edges end within ε of the floor — read through
    /// them, the corner rested at every dip. The bottom face is read by
    /// its vertices, and its far ones leave the floor: in band at the
    /// small dips, decidedly below at the large ones, and the corner's
    /// vertex-on-face touch is never a rest. The pair-level wedge along
    /// the dipping face refuses too.
    #[test]
    fn an_obtuse_sector_is_read_through_its_rays() {
        let band = Band::linear(Tol::witness()).unwrap();
        let prof = [
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (-1.0, 1.0),
            (-1.0, 0.01),
        ];
        for times in [2.0, 5.0, 30.0, 500.0] {
            let phi = times * band.zero();
            let body = on_floor(|b| {
                mapped_prism(b, &prof, (0.0, 0.03), |x, y, z| {
                    Point3::new(x, y, z - phi * y)
                })
            });
            let geo = snapshot(&body);
            let got = sites(&body);
            let corner: Vec<&TouchVerdict> = got
                .iter()
                .filter(|(site, _)| match site {
                    TouchSite::VertexOnFace(v, _) => geo
                        .vmap
                        .get(v)
                        .is_some_and(|q| q.x.abs() < 1e-9 && q.y.abs() < 1e-9),
                    _ => false,
                })
                .map(|(_, t)| t)
                .collect();
            assert_eq!(corner.len(), 1, "{times}× zero: {got:?}");
            assert!(!corner[0].is_rest(), "{times}× zero: {got:?}");
            assert!(
                got.iter()
                    .any(|(site, t)| matches!(site, TouchSite::EdgeInFace(..)) && !t.is_rest()),
                "{times}× zero, the pair-level wedge: {got:?}"
            );
        }
    }

    /// **The obtuse sector on a synthetic link.** The same corner built
    /// from bare geometry — the sheared bottom face and the two side
    /// faces meeting at it — dipping 500 times the zero threshold over
    /// the floor face: the bottom face's pieces read below the floor at
    /// their far points, and with every reading decided the touch is a
    /// crossing.
    #[test]
    fn an_obtuse_sector_on_a_synthetic_link_is_not_a_rest() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (phi, delta, h) = (500.0 * band.zero(), 0.01, 0.03);
        let at = |x: f64, y: f64, z: f64| Point3::new(x, y, z - phi * y);
        let verts = vec![
            at(0.0, 0.0, 0.0),
            at(1.0, 0.0, 0.0),
            at(1.0, 1.0, 0.0),
            at(-1.0, 1.0, 0.0),
            at(-1.0, delta, 0.0),
            at(0.0, 0.0, h),
            at(1.0, 0.0, h),
            at(-1.0, delta, h),
        ];
        let normal = |a: usize, b: usize, c: usize| {
            (verts[b] - verts[a]).cross(verts[c] - verts[b]).normalize()
        };
        let faces = [
            (normal(0, 4, 3), vec![vec![0, 4, 3, 2, 1]]),
            (normal(0, 1, 6), vec![vec![0, 1, 6, 5]]),
            (normal(4, 0, 5), vec![vec![4, 0, 5, 7]]),
        ];
        let edges = [(vec![1], [0, 1]), (vec![4], [0, 2]), (vec![5], [1, 2])];
        let part = synthetic(verts[0], At::Vertex(0), verts.clone(), &faces, &edges, band)
            .expect("a corner");
        assert_eq!(
            rest::judge(&part, &floor_link(band), band),
            TouchVerdict::Crossing
        );
    }

    /// **An edge cross reads both ways through its wedges.** Two slabs
    /// crossed in a plus over the same `z` range (two solids) cross each
    /// other's edges in the planes `z = 0` and `z = 1` with the two
    /// materials on the SAME side of each plane: every cross reads as a
    /// crossing. A bar resting across a block's top edge, `[0.5, 1] ×
    /// [−0.5, 2.5] × [2, 2.2]` on `[0, 2]³`, crosses the block's top
    /// edges with its material above `z = 2` and the block's below:
    /// every cross reads as a rest. Whether an overlap can show itself
    /// ONLY at edge crosses end to end is not settled — in every pose
    /// measured an overlapping cross comes with an edge lying in a face,
    /// a pierce or a vertex inside — so the verdict is pinned here, at
    /// the site.
    #[test]
    fn an_edge_cross_reads_rest_and_crossing() {
        let brick = |x: (f64, f64), y: (f64, f64), z: (f64, f64)| {
            crate::test_support_fixtures::brick::<f64>(x, y, z, Tol::witness())
        };
        let pair = |a: Body<f64>, b: &Body<f64>| {
            let mut body = a;
            crate::instance::graft_disjoint(&mut body, b).unwrap();
            body
        };
        let crosses = |body: &Body<f64>| -> Vec<TouchVerdict> {
            sites(body)
                .into_iter()
                .filter(|(site, _)| matches!(site, TouchSite::EdgeCross(..)))
                .map(|(_, v)| v)
                .collect()
        };
        let plus = crosses(&pair(
            brick((0.0, 3.0), (1.0, 2.0), (0.0, 1.0)),
            &brick((1.0, 2.0), (0.0, 3.0), (0.0, 1.0)),
        ));
        assert!(!plus.is_empty());
        assert!(
            plus.iter().all(|v| *v == TouchVerdict::Crossing),
            "{plus:?}"
        );
        let bar = crosses(&pair(
            brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            &brick((0.5, 1.0), (-0.5, 2.5), (2.0, 2.2)),
        ));
        assert!(!bar.is_empty());
        assert!(bar.iter().all(|v| v.is_rest()), "{bar:?}");
    }

    /// **The obtuse-sector pose, end to end, where the gate separates.**
    /// The part of [`an_obtuse_sector_is_read_through_its_rays`] sits on
    /// the floor slab with neither extent inside the other's, so the
    /// extent gate separates both orderings and only the touch analysis
    /// stands between the pair and a clear. Across dips from 2 to 30
    /// times the zero threshold and sectors from 0.6° short of flat to
    /// a few hundred band widths short of it, a face dipping into the
    /// floor never clears: decidedly below the floor, its far corners
    /// are inside the floor; in band, the sweeps escalate and the pair
    /// is refused as left unchecked.
    #[test]
    fn an_obtuse_sector_never_clears_a_dip_where_the_gate_separates() {
        let band = Band::linear(Tol::witness()).unwrap();
        // Every placement finding, as the pair it names and its reason.
        let placements = |body: &Body<f64>| -> Vec<(EntityId, EntityId, String)> {
            crate::validate_pseudomanifold(body, &ContactRecords::default(), Tol::witness())
                .err()
                .unwrap_or_default()
                .iter()
                .filter_map(|e| match e {
                    ValidationError::CensusUndecidable {
                        a: a @ EntityId::Solid(_),
                        b: b @ EntityId::Solid(_),
                        what,
                    } => Some((*a, *b, (*what).to_owned())),
                    ValidationError::InstanceInterference { outer, inner, .. } => Some((
                        EntityId::Solid(*outer),
                        EntityId::Solid(*inner),
                        "interference".to_owned(),
                    )),
                    _ => None,
                })
                .collect()
        };
        // The nearer-flat sectors scale with the run's band: the
        // fixture needs the corner's vertical edge, 3 cm tall, to read
        // decidedly bent.
        for delta in [0.01, 3000.0 * band.escalate(), 300.0 * band.escalate()] {
            let prof = [
                (0.0, 0.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (-1.0, 1.0),
                (-1.0, delta),
            ];
            for times in [2.0, 5.0, 30.0] {
                let phi = times * band.zero();
                let body = on_floor(|b| {
                    mapped_prism(b, &prof, (0.0, 0.03), |x, y, z| {
                        Point3::new(x, y, z - phi * y)
                    })
                });
                let got = placements(&body);
                let solids: Vec<EntityId> =
                    body.solids().map(|(k, _)| EntityId::Solid(k)).collect();
                let (floor, part) = (solids[0], solids[1]);
                assert!(
                    !got.is_empty(),
                    "a dip of {times}·zero at δ = {delta} cleared"
                );
                if times > 10.0 {
                    // Decidedly below the floor: the far corners are in it.
                    assert_eq!(got, [(floor, part, "interference".to_owned())]);
                } else {
                    // In band: the probe cannot place the far corners
                    // and the sweeps escalate on them, so the pair's one
                    // refusal is its boundaries left unchecked.
                    assert_eq!(
                        got,
                        [(floor, part, Undecided::Unexamined.what().to_owned())]
                    );
                }
            }
        }
    }

    /// **A tip tilted about one chord is read at its long face.** A
    /// plate's 90° tip, 3 cm chords, rests on the floor; the bottom face
    /// is tilted about ONE chord, so only the other chord carries the
    /// dip, 12 times the zero threshold per metre. That chord bounds the
    /// long bottom face (its far vertices about 1.1 m off: decidedly
    /// below the floor) and a short side face (about 5 cm: on the
    /// floor); read at the short side alone, the tip reads as a rest.
    /// Both chords are tried, and the tip is not a rest either way.
    #[test]
    fn a_tip_tilted_about_one_chord_is_never_a_rest() {
        let band = Band::linear(Tol::witness()).unwrap();
        let phi = 12.0 * band.zero();
        let prof = [
            (1.0, 0.5),
            (0.97, 0.53),
            (0.0, 1.0),
            (0.0, 0.0),
            (0.97, 0.47),
        ];
        for side in [-1.0, 1.0] {
            let body = on_floor(|b| {
                mapped_prism(b, &prof, (0.0, 0.03), |x, y, z| {
                    let d = (-(x - 1.0) + side * (y - 0.5)) / 2f64.sqrt();
                    Point3::new(x, y, z - phi * d)
                })
            });
            let geo = snapshot(&body);
            let got = sites(&body);
            let tip: Vec<&TouchVerdict> = got
                .iter()
                .filter(|(site, _)| match site {
                    TouchSite::VertexOnFace(v, _) => geo
                        .vmap
                        .get(v)
                        .is_some_and(|q| (q.x - 1.0).abs() < 1e-9 && (q.y - 0.5).abs() < 1e-9),
                    _ => false,
                })
                .map(|(_, t)| t)
                .collect();
            assert!(!tip.is_empty(), "{side}: {got:?}");
            assert!(tip.iter().all(|t| !t.is_rest()), "{side}: {tip:?}");
        }
    }

    /// The cuts of an edge in a face's plane where the face's boundary
    /// crosses it, each decide pinned at the verdict it feeds, and the
    /// cells those cuts bound.
    /// **Pass 2′ passes a vertex clear of a curved edge by, unread.** An
    /// arc of the unit circle over 0°–100° and a lone vertex at the
    /// circle's centre, inside the arc's box and a metre off it: the walk
    /// clears the pair and the census records none of the reading's
    /// decisions (`pm_census_curved_speed_hi`, its first) for it. A lone
    /// vertex on the arc at 50° is read, and is the unrecorded contact.
    /// Red when the walk's skip is dropped and every candidate is read.
    #[test]
    fn pass_two_prime_passes_a_clear_vertex_by_unread() {
        use geom_core::k_stats::Bracket;
        let tol = Tol::witness();
        let c = geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let (t0, t1) = (0.0, 100.0_f64.to_radians());
        for (q, read) in [
            (Point3::new(0.0, 0.0, 0.0), false),
            (c.eval(50.0_f64.to_radians()), true),
        ] {
            let mut body = Body::<f64>::new();
            let born = body.mvfs(c.eval(t0), true).unwrap();
            body.mev(
                MevSite::Lone {
                    r#loop: born.r#loop,
                },
                c.eval(t1),
                crate::EdgeCurveSpec::arc_of_circle(c.clone(), t0, t1).unwrap(),
                tol,
            )
            .unwrap();
            body.mvfs(q, true).unwrap();
            let bracket = Bracket::open();
            let errors = census_and_certify(&body, &ContactRecords::default(), band(), tol, None);
            let reads = bracket
                .finish()
                .verdicts
                .iter()
                .filter(|v| v.predicate == "pm_census_curved_speed_hi")
                .count();
            assert_eq!(reads > 0, read, "{q:?}: read {reads} times; {errors:?}");
            let found = errors.iter().any(|e| {
                matches!(
                    e,
                    ValidationError::UnrecordedContact {
                        contact: CensusContact::VertexOnEdge { .. },
                        ..
                    }
                )
            });
            assert_eq!(found, read, "{q:?}: {errors:?}");
        }
    }

    mod crossing_cuts {
        use super::*;
        use crate::test_support_fixtures::prism_z;

        fn band() -> Band {
            Band::linear(Tol::witness()).unwrap()
        }

        /// A post whose top at `z = 0.5` is `cap`, under a shelf
        /// `[0, 0.9] × [0, 0.30] × [0.5, 0.54]`; the cap face and the
        /// shelf's `y = 0.30` underside edge, which runs `x = 0.9 → 0`.
        fn lap(cap: &[(f64, f64)]) -> (Body<f64>, FaceKey, EdgeKey) {
            let tol = Tol::witness();
            let post = prism_z::<f64>(cap, 0.0, 0.5, tol);
            let shelf = prism_z::<f64>(
                &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
                0.5,
                0.54,
                tol,
            );
            let mut body = post.body;
            crate::instance::graft_disjoint(&mut body, &shelf.body).unwrap();
            let geo = snapshot(&body);
            let edge = geo
                .edges
                .iter()
                .find(|e| {
                    let ends = [e.p0, e.p0 + e.dir * e.len];
                    ends.iter().all(|p| p.y == 0.30 && p.z == 0.5) && close(e.len, 0.9)
                })
                .expect("the shelf's y = 0.30 underside edge")
                .key;
            (body, post.top_face, edge)
        }

        /// `boundary_crossings` of `edge` against `face`: each cut as
        /// the `x` of its point and what sits there, and what it pushed.
        fn crossings(
            body: &Body<f64>,
            face: FaceKey,
            edge: EdgeKey,
        ) -> (Option<Vec<(f64, CutAt)>>, Vec<ValidationError>) {
            let geo = snapshot(body);
            let e = geo.line_edge(edge).unwrap();
            let f = planar_face(&geo, face).unwrap();
            let mut errors = Vec::new();
            let cuts = boundary_crossings(body, e, f, band(), &mut errors);
            let at_x = |c: Cut<f64>| ((e.p0 + e.dir * c.s).x, c.at);
            (cuts.map(|c| c.into_iter().map(at_x).collect()), errors)
        }

        fn close(a: f64, b: f64) -> bool {
            (a - b).abs() < 1e-12
        }

        /// The escalations `errors` carries, by predicate name.
        fn escalated(errors: &[ValidationError]) -> Vec<&'static str> {
            errors
                .iter()
                .filter_map(|e| match e {
                    ValidationError::CensusEscalated { cause } => cause.predicate,
                    _ => None,
                })
                .collect()
        }

        /// **A face whose boundary crosses the edge away from any
        /// vertex.** The cap `[0.1, 0.3] × [0.2, 0.42]` sits under the
        /// shelf's `y = 0.30` edge, whose own midpoint (`x = 0.45`)
        /// lies outside it. The cap's side edges cross the shelf edge
        /// at `x = 0.3` and `x = 0.1`, the edge is cut at both, and of
        /// its three cells only the one in the cap is returned — the two
        /// outside it are not, and the one returned is bounded at the
        /// two crossings, never at the edge's own ends.
        #[test]
        fn a_boundary_crossing_away_from_any_vertex_bounds_the_cell() {
            let cap = [(0.1, 0.2), (0.3, 0.2), (0.3, 0.42), (0.1, 0.42)];
            let (body, face, edge) = lap(&cap);
            let geo = snapshot(&body);
            let e = geo.line_edge(edge).unwrap();
            let f = planar_face(&geo, face).unwrap();
            let mut errors = Vec::new();
            let cells = ef_overlap_cells(&body, e, f, &geo, band(), &mut errors);
            assert!(errors.is_empty(), "{errors:?}");
            let got: Vec<_> = cells
                .iter()
                .map(|c| {
                    let x = |cut: Cut<f64>| (e.p0 + e.dir * cut.s).x;
                    (x(c.lo), c.lo.at, x(c.hi), c.hi.at, c.mid)
                })
                .collect();
            assert_eq!(cells.len(), 1, "the one cell in the cap: {got:?}");
            let (lo, lo_at, hi, hi_at, mid) = got[0];
            assert!(close(lo, 0.3) && close(hi, 0.1), "{got:?}");
            assert!(
                matches!((lo_at, hi_at), (CutAt::Crossing(_), CutAt::Crossing(_))),
                "both bounds are crossings: {got:?}"
            );
            assert!(
                close(mid.x, 0.2) && close(mid.y, 0.3) && close(mid.z, 0.5),
                "{got:?}"
            );
        }

        /// **A straight boundary edge is cut where it crosses.** Both
        /// ends of a boundary edge definitely across the line: a
        /// crossing. An end ON the line (the cap corner `(0.3, 0.3)`):
        /// no crossing through that edge's interior; the corner is a
        /// boundary vertex, cut by the vertex cuts. Both ends on one
        /// side (the cap's top and bottom): nothing.
        #[test]
        fn a_crossing_is_cut_where_both_ends_lie_definitely_across() {
            let (body, face, edge) = lap(&[(0.1, 0.2), (0.3, 0.2), (0.3, 0.42), (0.1, 0.42)]);
            let (cuts, errors) = crossings(&body, face, edge);
            let cuts = cuts.expect("decided");
            assert!(errors.is_empty(), "{errors:?}");
            let xs: Vec<f64> = cuts.iter().map(|c| c.0).collect();
            assert!(
                cuts.len() == 2
                    && xs.iter().any(|&x| close(x, 0.1))
                    && xs.iter().any(|&x| close(x, 0.3))
                    && cuts.iter().all(|c| matches!(c.1, CutAt::Crossing(_))),
                "{cuts:?}"
            );
            let (body, face, edge) =
                lap(&[(0.1, 0.2), (0.3, 0.2), (0.3, 0.3), (0.2, 0.42), (0.1, 0.42)]);
            let (cuts, errors) = crossings(&body, face, edge);
            let cuts = cuts.expect("decided");
            assert!(errors.is_empty(), "{errors:?}");
            assert!(
                cuts.len() == 1 && close(cuts[0].0, 0.1),
                "only the side edge that crosses away from a vertex: {cuts:?}"
            );
            // A corner `12ε` across the line — just past the escalation
            // threshold — still makes its side edge a crossing.
            let near = 0.3 + 12.0 * band().zero();
            let (body, face, edge) = lap(&[
                (0.1, 0.2),
                (0.3, 0.2),
                (0.3, near),
                (0.2, 0.42),
                (0.1, 0.42),
            ]);
            let (cuts, errors) = crossings(&body, face, edge);
            let cuts = cuts.expect("decided");
            assert!(errors.is_empty(), "{errors:?}");
            assert!(
                cuts.len() == 2 && cuts.iter().any(|c| (c.0 - 0.3).abs() < 1e-9),
                "the side edge whose corner is 12ε across is cut: {cuts:?}"
            );
        }

        /// **A crossing within the band escalates.** A cap corner
        /// `3ε` above the shelf edge's line, reached by a side edge
        /// from well below it: whether that edge crosses the line is
        /// not decidable, and the lane refuses, typed. The side edge's
        /// end is read first, and its distance from the line is in the
        /// band (`curve_carrier_gap`).
        #[test]
        fn a_crossing_within_the_band_escalates() {
            let lift = 0.3 + 3.0 * band().zero();
            let (body, face, edge) = lap(&[
                (0.1, 0.2),
                (0.3, 0.2),
                (0.3, lift),
                (0.2, 0.42),
                (0.1, 0.42),
            ]);
            let (cuts, errors) = crossings(&body, face, edge);
            assert!(cuts.is_none(), "{cuts:?}");
            assert!(
                escalated(&errors).contains(&"curve_carrier_gap"),
                "{errors:?}"
            );
        }

        /// **A crossing is cut only strictly inside the span.** A
        /// crossing at the edge's own end (the cap side `x = 0.9`) is
        /// that end's cut, not a new one; a crossing past the end (a
        /// slanted side meeting the line at `x ≈ 0.917`) is none; a
        /// crossing in the band of the end refuses the lane: the edge's
        /// end, read first, lies in the band of the side's line
        /// (`curve_carrier_gap`).
        #[test]
        fn a_crossing_is_cut_only_strictly_inside_the_span() {
            let (body, face, edge) = lap(&[(0.9, 0.2), (1.1, 0.2), (1.1, 0.42), (0.9, 0.42)]);
            let (cuts, errors) = crossings(&body, face, edge);
            assert!(
                errors.is_empty() && cuts.as_ref().is_some_and(Vec::is_empty),
                "at the end: {cuts:?} {errors:?}"
            );
            let (body, face, edge) = lap(&[(0.7, 0.2), (1.05, 0.2), (0.85, 0.35), (0.7, 0.35)]);
            let (cuts, errors) = crossings(&body, face, edge);
            let cuts = cuts.expect("decided");
            assert!(errors.is_empty(), "{errors:?}");
            assert!(
                cuts.len() == 1 && close(cuts[0].0, 0.7),
                "past the end: only the far side's crossing: {cuts:?}"
            );
            let x = 0.9 + 3.0 * band().zero();
            let (body, face, edge) = lap(&[(x, 0.2), (1.1, 0.2), (1.1, 0.42), (x, 0.42)]);
            let (cuts, errors) = crossings(&body, face, edge);
            assert!(cuts.is_none(), "{cuts:?}");
            assert!(
                escalated(&errors).contains(&"curve_carrier_gap"),
                "{errors:?}"
            );
        }

        /// **A boundary edge wholly past an end is no cut.** The cap
        /// reaches past the shelf edge's end, where a notch puts a corner
        /// `3ε` above the edge's LINE. The two notch edges lie wholly
        /// beyond the edge's end, so their meetings with it read past its
        /// span and nothing escalates, while the cap side at `x = 0.1` is
        /// still cut.
        #[test]
        fn a_boundary_edge_wholly_past_an_end_is_not_read() {
            let notch = 0.3 + 3.0 * band().zero();
            let (body, face, edge) = lap(&[
                (0.1, 0.2),
                (1.3, 0.2),
                (1.3, 0.42),
                (1.2, notch),
                (1.1, 0.42),
                (0.1, 0.42),
            ]);
            let (cuts, errors) = crossings(&body, face, edge);
            assert!(errors.is_empty(), "{errors:?}");
            let cuts = cuts.expect("decided");
            assert!(cuts.len() == 1 && close(cuts[0].0, 0.1), "{cuts:?}");
        }

        /// **A spline boundary edge is read by the walk.** The cap of
        /// [`a_crossing_is_cut_where_both_ends_lie_definitely_across`],
        /// its side at `x = 0.3` re-carried as a degree-1 spline along the
        /// same chord: the shelf edge is cut there as the line was, now a
        /// curved edge's crossing, and at `x = 0.1`; nothing is pushed.
        #[test]
        fn a_spline_boundary_is_cut_where_it_crosses_the_edge() {
            use geom::{Curve3, NurbsCurve3};
            use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
            use geom_core::spline::{KnotVector, Knots};
            let (mut body, face, edge) = lap(&[(0.1, 0.2), (0.3, 0.2), (0.3, 0.42), (0.1, 0.42)]);
            let side = body
                .edges()
                .find_map(|(k, e)| {
                    let v0 = body.get_half_edge(e.he_plus)?.start;
                    let v1 = body.half_edge_end(e.he_plus)?;
                    let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                    let (p0, p1): (Point3<f64>, Point3<f64>) = (p(v0), p(v1));
                    (p0.x == 0.3 && p1.x == 0.3 && p0.z == 0.5 && p1.z == 0.5)
                        .then_some((k, e.he_plus, e.he_minus, p0, p1))
                })
                .expect("the cap's side at x = 0.3");
            let (key, plus, minus, q0, q1) = side;
            let surface = |he| {
                let f = body.face_of_half_edge(he).unwrap();
                body.get_face(f).unwrap().surface
            };
            let pair = SurfacePair::new(surface(plus), surface(minus));
            let knots = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            let spline = NurbsCurve3::new(Knots::from(knots), vec![q0, q1], vec![1.0; 2])
                .expect("the chord as a degree-1 spline");
            let spec = EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair,
                    witness: q0.lerp(q1, 0.5),
                },
                carrier: Curve3::Nurbs(std::sync::Arc::new(spline)),
                param_start: 0.0,
                param_end: 1.0,
            };
            body.set_edge_curve(key, spec, Tol::witness())
                .expect("a degree-1 spline along the chord certifies");
            let (cuts, errors) = crossings(&body, face, edge);
            assert!(errors.is_empty(), "{errors:?}");
            let cuts = cuts.expect("decided");
            assert!(
                cuts.len() == 2
                    && cuts.iter().any(
                        |c| (c.0 - 0.3).abs() < 1e-9 && matches!(c.1, CutAt::CurvedCrossing(_))
                    )
                    && cuts
                        .iter()
                        .any(|c| close(c.0, 0.1) && matches!(c.1, CutAt::Crossing(_))),
                "{cuts:?}"
            );
        }

        /// A conic boundary edge is cut where it crosses the edge: the
        /// half disc of radius 1 under a unit cube's bottom edge
        /// `y = 0.5, x ∈ [0.5, 1.5]` is cut once, where the arc crosses
        /// it at `x = √0.75`; the circle's other root, `x = −√0.75`, is
        /// past the edge's end.
        #[test]
        fn a_conic_boundary_is_cut_at_its_roots() {
            let tol = Tol::witness();
            let mut body = half_disc_cap_and_far_cube();
            let cube = cube_at(Vec3::new(0.5, 0.5, 0.0), tol);
            crate::instance::graft_disjoint(&mut body, &cube).unwrap();
            let geo = snapshot(&body);
            let disc = geo
                .faces
                .iter()
                .find(|f| f.boundary.len() == 2)
                .expect("the half disc")
                .key;
            let edge = geo
                .edges
                .iter()
                .find(|e| {
                    let ends = [e.p0, e.p0 + e.dir * e.len];
                    ends.iter().all(|p| p.y == 0.5 && p.z == 0.0)
                })
                .expect("the cube's y = 0.5 bottom edge")
                .key;
            let (cuts, errors) = crossings(&body, disc, edge);
            assert!(errors.is_empty(), "{errors:?}");
            let cuts = cuts.expect("decided");
            assert!(
                cuts.len() == 1
                    && (cuts[0].0 - 0.75_f64.sqrt()).abs() < 1e-9
                    && matches!(cuts[0].1, CutAt::CurvedCrossing(_)),
                "{cuts:?}"
            );
        }

        /// **The touch analysis reads every cell of an edge in a
        /// face.** An L-plate's bottom edge `y = 0, x ∈ [0, 1]` rests on
        /// a floor slotted across it, so the edge meets the floor in two
        /// cells, one at each end. The plate's bottom face is tilted
        /// about that edge so the far end of the L's upright dips `12ε`
        /// below the floor. At each cell the two links are the same
        /// planes, the plate's two faces' half-discs and the floor's
        /// disc, so each cell reads the tilt alike: decided where the
        /// faces reach far (the floor's own far points against the
        /// plate's bottom plane), within ε inside the ball both links
        /// fill. Neither cell is a rest nor a crossing, and the site,
        /// read over both, is no rest in either build.
        #[test]
        fn the_touch_analysis_reads_every_cell_of_an_edge_in_a_face() {
            let band = band();
            let (w, phi) = (0.02, 12.0 * band.zero());
            for upright_at_start in [true, false] {
                let (floor, plate) = if upright_at_start {
                    (
                        vec![
                            (-1.0, -1.0),
                            (2.0, -1.0),
                            (2.0, 2.0),
                            (0.5, 2.0),
                            (0.5, -0.5),
                            (0.04, -0.5),
                            (0.04, 2.0),
                            (-1.0, 2.0),
                        ],
                        vec![
                            (0.0, 0.0),
                            (1.0, 0.0),
                            (1.0, w),
                            (w, w),
                            (w, 1.0),
                            (0.0, 1.0),
                        ],
                    )
                } else {
                    (
                        vec![
                            (-1.0, -1.0),
                            (2.0, -1.0),
                            (2.0, 2.0),
                            (0.96, 2.0),
                            (0.96, -0.5),
                            (0.5, -0.5),
                            (0.5, 2.0),
                            (-1.0, 2.0),
                        ],
                        vec![
                            (0.0, 0.0),
                            (1.0, 0.0),
                            (1.0, 1.0),
                            (1.0 - w, 1.0),
                            (1.0 - w, w),
                            (0.0, w),
                        ],
                    )
                };
                let mut body = Body::<f64>::new();
                mapped_prism(&mut body, &floor, (-1.0, 0.0), Point3::new);
                let mut part = Body::<f64>::new();
                mapped_prism(&mut part, &plate, (0.0, 0.03), |x, y, z| {
                    Point3::new(x, y, z - phi * y)
                });
                crate::instance::graft_disjoint(&mut body, &part).unwrap();
                let geo = snapshot(&body);
                let e = geo
                    .edges
                    .iter()
                    .find(|e| {
                        let ends = [e.p0, e.p0 + e.dir * e.len];
                        ends.iter().all(|p| p.y == 0.0 && p.z == 0.0)
                    })
                    .expect("the plate's resting edge");
                let top = geo
                    .faces
                    .iter()
                    .find(|f| {
                        f.loops[0].len() == 8 && f.loops[0].iter().all(|v| geo.vmap[v].z == 0.0)
                    })
                    .expect("the floor's top");
                let mut errors = Vec::new();
                let cells = ef_overlap_cells(&body, e, top, &geo, band, &mut errors);
                assert!(errors.is_empty() && cells.len() == 2, "{errors:?}");
                let verdicts: Vec<TouchVerdict> = cells
                    .iter()
                    .map(|c| {
                        match (
                            Link::on_edge(&body, &geo, e.key, c.mid, band),
                            Link::in_face(&body, &geo, top.key, c.mid, band),
                        ) {
                            (Ok(a), Ok(b)) => rest::judge(&a, &b, band),
                            (Err(v), _) | (_, Err(v)) => v,
                        }
                    })
                    .collect();
                assert!(
                    verdicts.iter().all(|v| *v == TouchVerdict::InBand),
                    "each cell reads the dip in band: {verdicts:?}"
                );
                let site = TouchSite::EdgeInFace(e.key, top.key);
                let verdict = site.verdict(&body, &geo, &Ties::default(), band);
                assert_eq!(verdict, TouchVerdict::InBand);
            }
        }

        /// **A spiric boundary is read by the meeting reading's walk.**
        /// The cube beside the spiric cap lies inside the ball the whole
        /// arc lies in but clear of the arc itself: nothing is cut or
        /// pushed. A cube straddling the arc has an edge in the cap's
        /// plane that crosses it: cut there, on the oval.
        #[test]
        fn a_spiric_boundary_is_cut_where_it_meets_the_edge_and_clear_elsewhere() {
            use std::f64::consts::FRAC_PI_2;
            let tol = Tol::witness();
            // The cube's edge in the cap's plane along `z = z0`.
            let in_plane = |body: &Body<f64>, z0: f64| -> (FaceKey, EdgeKey) {
                let geo = snapshot(body);
                let cap = geo
                    .faces
                    .iter()
                    .find(|f| f.boundary.len() == 2)
                    .expect("the spiric cap")
                    .key;
                let edge = geo
                    .edges
                    .iter()
                    .find(|e| {
                        let ends = [e.p0, e.p0 + e.dir * e.len];
                        e.f_plus != cap && ends.iter().all(|p| p.x == 0.5 && p.z == z0)
                    })
                    .expect("a cube edge in the cap's plane")
                    .key;
                (cap, edge)
            };
            let near = spiric_cap_and_near_cube();
            let (cap, edge) = in_plane(&near, -0.5);
            let (cuts, errors) = crossings(&near, cap, edge);
            assert!(
                errors.is_empty() && cuts.as_ref().is_some_and(Vec::is_empty),
                "{cuts:?} {errors:?}"
            );
            let mut across = spiric_cap(2.0, 1.0, 0.5, (-FRAC_PI_2, FRAC_PI_2));
            let cube = cube_at(Vec3::new(0.5, 2.5, -0.5), tol);
            crate::instance::graft_disjoint(&mut across, &cube).unwrap();
            let (cap, edge) = in_plane(&across, -0.5);
            let geo = snapshot(&across);
            let e = geo.line_edge(edge).unwrap();
            let f = planar_face(&geo, cap).unwrap();
            let mut errors = Vec::new();
            let cuts = boundary_crossings(&across, e, f, band(), &mut errors);
            assert!(errors.is_empty(), "{errors:?}");
            let cuts = cuts.expect("decided");
            let oval = geom::Curve3::Spiric {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
                major_radius: 2.0,
                minor_radius: 1.0,
                offset: 0.5,
            };
            assert!(!cuts.is_empty(), "the straddling edge is cut");
            for cut in &cuts {
                assert!(matches!(cut.at, CutAt::CurvedCrossing(_)), "{cuts:?}");
                let q = e.p0 + e.dir * cut.s;
                let off = (0..=4096)
                    .map(|i| {
                        let v =
                            -std::f64::consts::PI + std::f64::consts::TAU * f64::from(i) / 4096.0;
                        (oval.eval(v) - q).norm()
                    })
                    .fold(f64::INFINITY, f64::min);
                assert!(off < 1e-3, "a cut on the oval: {q:?} lies {off:e} off it");
            }
        }

        /// A planar cap bounded by one ellipse arc `(t0, t1)` of
        /// `carrier`, cut from `cyl` by the plane through the ellipse,
        /// and the chord closing it; the cap's face.
        fn ellipse_cap(
            cyl: CylFrame,
            carrier: geom::Curve3<f64>,
            (t0, t1): (f64, f64),
        ) -> (Body<f64>, FaceKey) {
            use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
            let geom::Curve3::Ellipse {
                center,
                axis,
                u_ref,
                ..
            } = carrier
            else {
                panic!("an ellipse carrier")
            };
            let tol = Tol::witness();
            let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
            let mut body = Body::<f64>::new();
            let seed = body.mvfs(p0, true).expect("a seed");
            let plane = body.add_surface(Surface::Plane {
                origin: center,
                normal: axis,
                u_ref,
            });
            let cyl = body.add_surface(cyl.surface());
            let arc = body
                .mev(
                    MevSite::Lone {
                        r#loop: seed.r#loop,
                    },
                    p1,
                    EdgeCurveSpec {
                        description: EdgeDescriptionSpec::Intersection {
                            pair: geom_brep::SurfacePair::new(cyl, plane),
                            witness: carrier.eval(geom::mid_param(t0, t1)),
                        },
                        carrier,
                        param_start: t0,
                        param_end: t1,
                    },
                    tol,
                )
                .expect("the arc");
            let chord = body
                .mef(
                    MefSite::Chords {
                        he1: arc.he_minus,
                        he2: arc.he_plus,
                    },
                    EdgeCurveSpec::line_between(p1, p0),
                    FaceSurface::Shared {
                        key: plane,
                        sense: true,
                    },
                    tol,
                )
                .expect("the chord closes the cap");
            let face = [chord.face, seed.face]
                .into_iter()
                .find(|&f| {
                    let geo = snapshot(&body);
                    planar_face(&geo, f).is_some()
                        && body.get_face(f).is_some_and(|d| d.surface == plane)
                })
                .expect("the cap");
            (body, face)
        }

        /// An edge from `a` to `b` in `body`'s census snapshot shape,
        /// keyed on any of `body`'s own entities: the overlap lane reads
        /// only its geometry.
        fn edge_from(body: &Body<f64>, a: Point3<f64>, b: Point3<f64>) -> EdgeGeo<f64> {
            let geo = snapshot(body);
            let any = &geo.edges[0];
            EdgeGeo {
                p0: a,
                p1: b,
                points: [None; 2],
                dir: (b - a).normalize(),
                len: (b - a).norm(),
                ..*any
            }
        }

        /// A cell's two bounds, as their `y` and what sits there.
        type CellAlongY = (f64, CutAt, f64, CutAt);

        /// The `y` of every cell's bounds and their kinds.
        fn cells_along_y(
            body: &Body<f64>,
            face: FaceKey,
            e: &EdgeGeo<f64>,
        ) -> (Vec<CellAlongY>, Vec<ValidationError>) {
            let geo = snapshot(body);
            let f = planar_face(&geo, face).unwrap();
            let mut errors = Vec::new();
            let cells = ef_overlap_cells(body, e, f, &geo, band(), &mut errors);
            let y = |c: Cut<f64>| (e.p0 + e.dir * c.s).y;
            (
                cells
                    .iter()
                    .map(|c| (y(c.lo), c.lo.at, y(c.hi), c.hi.at))
                    .collect(),
                errors,
            )
        }

        /// **A steep ellipse's crossing near an arc end is cut** (the
        /// half-ellipse `a = 20, b = 1`, arc `[π/2, 3π/2]`, closed by the
        /// chord `x = 0`). The edge runs on `x = −19ε`: it crosses the
        /// arc `19ε` from each end along the arc — clear of the end in
        /// metres — but only `0.95ε` from it in the carrier's parameter
        /// times its minor semi-axis. Both crossings are cut, and the
        /// edge's interval inside the cap is one cell between them.
        #[test]
        fn a_steep_ellipse_crossing_near_an_arc_end_is_cut() {
            use std::f64::consts::FRAC_PI_2;
            let carrier = geom::Curve3::Ellipse {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                semi_u: 20.0,
                semi_v: 1.0,
                u_ref: Vec3::unit_x(),
            };
            let (body, cap) = ellipse_cap(
                CylFrame::tilted(1.0, 0.05_f64.acos()),
                carrier,
                (FRAC_PI_2, 3.0 * FRAC_PI_2),
            );
            let x = -19.0 * band().zero();
            let e = edge_from(&body, Point3::new(x, -1.2, 0.0), Point3::new(x, 4.0, 0.0));
            let (cells, errors) = cells_along_y(&body, cap, &e);
            assert!(errors.is_empty(), "{errors:?}");
            assert!(
                cells.len() == 1
                    && (cells[0].0 + 1.0).abs() < 1e-6
                    && (cells[0].2 - 1.0).abs() < 1e-6
                    && matches!(
                        (cells[0].1, cells[0].3),
                        (CutAt::CurvedCrossing(_), CutAt::CurvedCrossing(_))
                    ),
                "one cell, crossing to crossing: {cells:?}"
            );
        }

        /// The same, tilted: a face in the plane `z = 20x` bounded by an
        /// arc of the cylinder `x² + y² = 0.01` (an ellipse of semi-axes
        /// `√4.01` and `0.1`), from `8ε` short of `θ = π/2` round to `8ε`
        /// past `3π/2`, and its chord. The edge on `x = z = 0` crosses the
        /// arc at `θ = π/2` and `3π/2`, `0.8ε` from the ends by the minor
        /// meter and `16ε` along the arc. An edge reaching past the
        /// crossing at `y = 0.1` has one cell inside, bounded there; an
        /// edge ending inside has no cell straddling it.
        #[test]
        fn a_tilted_ellipse_crossing_near_an_arc_end_is_cut() {
            use std::f64::consts::FRAC_PI_2;
            let major = 4.01_f64.sqrt();
            let normal = Vec3::new(-20.0, 0.0, 1.0).normalize();
            let carrier = geom::Curve3::Ellipse {
                center: Point3::origin(),
                axis: normal,
                semi_u: major,
                semi_v: 0.1,
                u_ref: Vec3::new(1.0, 0.0, 20.0).normalize(),
            };
            let d = 8.0 * band().zero();
            let (body, cap) = ellipse_cap(
                CylFrame::canonical(0.1),
                carrier,
                (FRAC_PI_2 - d, 3.0 * FRAC_PI_2 + d),
            );
            let on_y = |y0: f64, y1: f64| {
                edge_from(&body, Point3::new(0.0, y0, 0.0), Point3::new(0.0, y1, 0.0))
            };
            let (cells, errors) = cells_along_y(&body, cap, &on_y(-0.05, 0.3));
            assert!(errors.is_empty(), "{errors:?}");
            assert!(
                cells.len() == 1
                    && (cells[0].0 + 0.05).abs() < 1e-9
                    && (cells[0].2 - 0.1).abs() < 1e-6
                    && matches!(cells[0].3, CutAt::CurvedCrossing(_)),
                "the interval inside, bounded at the crossing: {cells:?}"
            );
            let (cells, errors) = cells_along_y(&body, cap, &on_y(0.0, 0.15));
            assert!(errors.is_empty(), "{errors:?}");
            assert!(
                cells.len() == 1 && (cells[0].2 - 0.1).abs() < 1e-6,
                "no cell straddles the crossing: {cells:?}"
            );
        }

        /// **A boundary edge reaching past an end is read**: a cap side
        /// running from past the shelf edge's start (`x = 0.95`) to inside
        /// its span crosses the edge at `x ≈ 0.859`, and is cut there.
        #[test]
        fn a_boundary_edge_reaching_past_an_end_is_read() {
            let (body, face, edge) = lap(&[(0.5, 0.2), (0.95, 0.2), (0.75, 0.42), (0.5, 0.42)]);
            let (cuts, errors) = crossings(&body, face, edge);
            assert!(errors.is_empty(), "{errors:?}");
            let cuts = cuts.expect("decided");
            let x = 0.95 - 0.2 * (0.1 / 0.22);
            assert!(
                cuts.len() == 2
                    && cuts.iter().any(|c| (c.0 - x).abs() < 1e-12)
                    && cuts.iter().any(|c| close(c.0, 0.5)),
                "{cuts:?}"
            );
        }
    }

    /// **Two points at one position are not one point**: the notched
    /// block's pinch half clears with no records while its tip copies
    /// share the cut vertex's point; rebinding one copy to a fresh,
    /// bit-equal point leaves a coincidence of values, which the
    /// census refuses as an unrecorded vertex–vertex contact.
    #[test]
    fn a_tip_copy_on_its_own_bit_equal_point_is_an_unrecorded_contact() {
        let notched = [
            (0.0, 0.0),
            (8.0, 0.0),
            (8.0, 2.0),
            (7.0, 1.0),
            (6.0, 1.0),
            (5.0, 2.0),
            (4.0, 1.0),
            (3.0, 2.0),
            (0.0, 2.0),
        ];
        let tol = Tol::witness();
        let fx = crate::test_support_fixtures::prism::<f64>(&notched, 1.0, tol);
        let operand = crate::test_support::finished("the fixture", fx.body.clone(), tol);
        let plane = crate::test_support_fixtures::split_plane(
            Point3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            tol,
        );
        let result = crate::split(&operand, &plane, tol).expect("the notched block splits");
        let mut above = result.above.body().expect("above has material").clone();
        let tip = Point3::new(4.0, 1.0, 0.0);
        let copies: Vec<VertexKey> = above
            .vertices()
            .filter(|(_, v)| {
                let p = above.points[v.point];
                (p.x, p.y, p.z) == (tip.x, tip.y, tip.z)
            })
            .map(|(k, _)| k)
            .collect();
        assert_eq!(copies.len(), 2, "two tip copies at z = 0");
        let none = ContactRecords::default();
        assert_eq!(
            crate::validate_pseudomanifold(&above, &none, tol),
            Ok(()),
            "copies on one point are structural sharing"
        );

        above
            .move_vertices(&[copies[1]], tip)
            .expect("the copy resolves");
        assert_ne!(
            above.vertices[copies[0]].point, above.vertices[copies[1]].point,
            "the move minted a fresh point"
        );
        let errors = crate::validate_pseudomanifold(&above, &none, tol)
            .expect_err("bit-equal values never glue");
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::UnrecordedContact {
                    contact: CensusContact::VertexVertex { a, b },
                    ..
                } if [*a, *b] == [copies[0], copies[1]] || [*a, *b] == [copies[1], copies[0]]
            )),
            "{errors:?}"
        );
    }

    /// **An edge lying in a face, bounded at points it shares with that
    /// face's corners, is structural** — the edge-on-face bound rung's
    /// shared-point arm. A wedge stands on its ridge along the diagonal
    /// of a kite prism's top face; the ridge's two ends sit on the
    /// kite's corner points. Each bound of the ridge's overlap with the
    /// top face is a ridge vertex on one point with a corner of that
    /// face, and nothing is declared.
    #[test]
    fn an_edge_in_a_face_bounded_on_its_corners_points_is_structural() {
        use crate::test_support_fixtures::{
            FaceGeometry, describe_as_intersections, identity_map, prism_ops,
        };
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        prism_ops(
            &mut body,
            &[(0.0, 0.0), (1.0, -1.0), (2.0, 0.0), (1.0, 1.0)],
            (0.0, 1.0),
            identity_map,
            FaceGeometry::Certified,
            tol,
        );
        // The wedge's profile in the (y, z) plane, extruded along x: a
        // cyclic permutation of the axes, so exact and orientation
        // preserving. Its ridge runs (0, 0, 1) → (2, 0, 1).
        prism_ops(
            &mut body,
            &[(0.0, 1.0), (0.5, 2.0), (-0.5, 2.0)],
            (0.0, 2.0),
            |x, y, z| Point3::new(z, x, y),
            FaceGeometry::Certified,
            tol,
        );
        describe_as_intersections(&mut body, tol);
        let at = |x: f64, y: f64, z: f64| -> Vec<VertexKey> {
            body.vertices()
                .filter(|(_, v)| {
                    let p = body.points[v.point];
                    (p.x, p.y, p.z) == (x, y, z)
                })
                .map(|(k, _)| k)
                .collect()
        };
        let ends = [at(0.0, 0.0, 1.0), at(2.0, 0.0, 1.0)];
        assert!(
            ends.iter().all(|e| e.len() == 2),
            "a corner and a ridge end at each: {ends:?}"
        );
        let none = ContactRecords::default();
        let errors = crate::validate_pseudomanifold(&body, &none, tol)
            .expect_err("on distinct points the touch is unrecorded");
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::UnrecordedContact {
                    contact: CensusContact::EdgeFaceOverlap { .. },
                    ..
                }
            )),
            "{errors:?}"
        );

        // Rebind each ridge end onto the kite corner's point: the copies
        // an op would mint share it.
        for pair in &ends {
            let point = body.vertices[pair[0]].point;
            let old = body.vertices[pair[1]].point;
            body.vertices[pair[1]].point = point;
            body.remove_point_if_orphaned(old);
        }
        assert_eq!(
            crate::validate_pseudomanifold(&body, &none, tol),
            Ok(()),
            "the ridge in the top face is bounded on shared points"
        );
    }
}

/// **The reach rules read a torn boundary as a torn body**: each tears
/// one record a sound face's or edge's reach passes through and asserts
/// the panic names the link and the premise, where the old reads
/// answered "no claim" (`None`). The sound reach is read first.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_reach_rows {
    use crate::body::Body;
    use crate::boolean::boxes::BoxFrame;
    use crate::entity::{EntityId, FaceKey, GeomRef, HalfEdgeKey, LoopBoundary};
    use crate::live::OPERATORS_KEEP_LINKS;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};
    use geom_core::{Band, Tol};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn first_member(body: &Body<f64>, face: FaceKey) -> HalfEdgeKey {
        let outer = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("the fixture's outer loop is a cycle");
        };
        first
    }

    fn reach(body: &Body<f64>, face: FaceKey) -> bool {
        super::face_reach_in(body, face, band(), &BoxFrame::World).is_some()
    }

    /// The face's surface, `boundary_reach` and the edge rule, on a cube
    /// face (the boundary hull arm): a torn surface, outer loop, curve
    /// and edge-end point.
    #[test]
    fn the_boundary_hull_panics_on_a_torn_surface_loop_and_curve() {
        let body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let face = body.faces().next().map(|(k, _)| k).unwrap();
        assert!(reach(&body, face), "the sound face has a reach");

        let mut torn = body.clone();
        let surface = torn.get_face(face).unwrap().surface;
        torn.surfaces.remove(surface);
        let named = format!(
            "{}'s surface names {}",
            EntityId::Face(face),
            GeomRef::Surface(surface)
        );
        assert_torn_op_panics(
            "face_reach_in (surface)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| reach(b, face),
        );

        let mut torn = body.clone();
        let outer = torn.get_face(face).unwrap().outer;
        torn.loops.remove(outer);
        let named = format!(
            "{}'s outer names {}",
            EntityId::Face(face),
            EntityId::Loop(outer)
        );
        assert_torn_op_panics(
            "face_reach_in (loop)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| reach(b, face),
        );

        let mut torn = body.clone();
        let edge = torn.get_half_edge(first_member(&torn, face)).unwrap().edge;
        assert!(
            super::edge_reach_in(&torn, edge, &BoxFrame::World).is_some(),
            "the sound edge has a reach"
        );
        let curve = torn.get_edge(edge).unwrap().curve;
        torn.curves.remove(curve);
        let named = format!(
            "{}'s curve names {}",
            EntityId::Edge(edge),
            GeomRef::Curve(curve)
        );
        assert_torn_op_panics(
            "face_reach_in (curve)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| reach(b, face),
        );
        assert_torn_op_panics(
            "edge_reach_in (curve)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| super::edge_reach_in(b, edge, &BoxFrame::World).is_some(),
        );

        // An edge end's point, which the edge rule reads before the curve.
        let mut torn = body.clone();
        let v = torn.get_half_edge(first_member(&torn, face)).unwrap().start;
        let point = torn.get_vertex(v).unwrap().point;
        torn.points.remove(point);
        let named = format!(
            "{}'s point names {}",
            EntityId::Vertex(v),
            GeomRef::Point(point)
        );
        assert_torn_op_panics(
            "face_reach_in (point)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| reach(b, face),
        );
    }

    /// `boundary_axial` on a cone wall (the cone arm, where the axial
    /// window is the only boundary read): a torn curve.
    #[test]
    fn the_cone_window_panics_on_a_torn_curve() {
        let (mut body, face) =
            crate::boolean::boxes::tests::cone_wall(30.0_f64.to_radians(), 0.0, 1.0, 0.5, 1.0);
        assert!(reach(&body, face), "the sound cone wall has a reach");
        let edge = body.get_half_edge(first_member(&body, face)).unwrap().edge;
        let curve = body.get_edge(edge).unwrap().curve;
        body.curves.remove(curve);
        let named = format!(
            "{}'s curve names {}",
            EntityId::Edge(edge),
            GeomRef::Curve(curve)
        );
        assert_torn_op_panics(
            "face_reach_in (cone axial)",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| reach(b, face),
        );
    }

    /// `boundary_axial`, on a cylinder wall sheet (the slab arm, which
    /// reads the axial window before the boundary hull): a torn point.
    #[test]
    fn the_axial_window_panics_on_a_torn_point() {
        let mut body = Body::<f64>::new();
        let face = crate::test_support_fixtures::cyl_wall_sheet(
            &mut body,
            crate::test_support_fixtures::CylFrame::canonical(1.0),
            (0.2, 1.4),
            (0.0, 1.0),
            Tol::witness(),
        );
        assert!(reach(&body, face), "the sound wall has a reach");
        let v = body.get_half_edge(first_member(&body, face)).unwrap().start;
        let point = body.get_vertex(v).unwrap().point;
        body.points.remove(point);
        let named = format!(
            "{}'s point names {}",
            EntityId::Vertex(v),
            GeomRef::Point(point)
        );
        assert_torn_op_panics(
            "face_reach_in (axial)",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| reach(b, face),
        );
    }
}

/// **The census reads which touch a record is from its two cells'
/// carriers**: an edge–face record on a line and a plane is confirmed
/// by reconstruction, one off it at a conventional vertex of its edge,
/// and a face–face record on two carriers along their tangent line.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod records_by_carrier {
    use super::*;
    use crate::boolean::{Cell, ContactRecords};
    use crate::test_support_fixtures::{CylFrame, brick, cyl_wall_sheet, plant_disc_face};
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn one(body: &Body<f64>, x: Cell, y: Cell) -> ContactRecords {
        [crate::Cited::new(
            crate::Contact::new(body, x, y).unwrap(),
            crate::Cites::decided(0),
        )]
        .into_iter()
        .collect()
    }

    fn census(body: &Body<f64>, records: &ContactRecords) -> Vec<ValidationError> {
        census_and_certify(body, records, band(), Tol::witness(), None)
    }

    fn stale_edge_face(errors: &[ValidationError], e: EdgeKey, f: FaceKey) -> bool {
        errors.iter().any(|x| {
            *x == ValidationError::StaleContactDeclaration {
                declaration: StaleDeclaration::EdgeFace { edge: e, face: f },
            }
        })
    }

    fn unsupported_edge(errors: &[ValidationError], e: EdgeKey) -> bool {
        errors.iter().any(|x| {
            matches!(x, ValidationError::CensusUnsupported {
                subject: CensusSubject::Entity(EntityId::Edge(k)), ..
            } if *k == e)
        })
    }

    /// The edge of `body` between two points.
    fn edge_at(body: &Body<f64>, p: [f64; 3], q: [f64; 3]) -> EdgeKey {
        let at = |v: VertexKey| {
            let r = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            [r.x, r.y, r.z]
        };
        body.edges()
            .find(|(_, e)| {
                let ends =
                    [e.he_plus, e.he_minus].map(|h| at(body.get_half_edge(h).unwrap().start));
                ends == [p, q] || ends == [q, p]
            })
            .map(|(k, _)| k)
            .expect("the edge")
    }

    /// A face of `body` whose plane holds every boundary vertex at
    /// `z`, and whose outward normal is `up`.
    fn face_at_z(body: &Body<f64>, z: f64, up: bool) -> FaceKey {
        body.faces()
            .find(|(_, f)| match body.get_surface(f.surface) {
                Some(geom::Surface::Plane { origin, normal, .. }) => {
                    normal.z.abs() > 0.5
                        && (origin.z - z).abs() < 1e-12
                        && ((normal.z > 0.0) == f.sense) == up
                }
                _ => false,
            })
            .map(|(k, _)| k)
            .expect("the face")
    }

    /// Two bricks, the second standing on the first's top: the second's
    /// bottom edge along y = 0.25 lies in the first's top face over
    /// x ∈ (0.5, 1). Its record is confirmed; the first's bottom edge
    /// against the second's top face, two metres off its plane, is
    /// stale. Red when the line-and-plane lane reads no overlap cell
    /// (the in-plane edge outside the region would confirm); the ends'
    /// residuals are pinned by `edge_face_residual`, whose edge projects
    /// into the face.
    #[test]
    fn a_straight_edge_in_a_plane_is_confirmed_by_reconstruction() {
        let tol = Tol::witness();
        let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let top = brick::<f64>((0.5, 1.5), (0.25, 0.75), (1.0, 2.0), tol);
        crate::graft_disjoint_all_keyed(&mut body, &top).unwrap();
        let on = edge_at(&body, [0.5, 0.25, 1.0], [1.5, 0.25, 1.0]);
        let a_top = face_at_z(&body, 1.0, true);
        let errors = census(&body, &one(&body, Cell::Edge(on), Cell::Face(a_top)));
        assert!(
            !stale_edge_face(&errors, on, a_top) && !unsupported_edge(&errors, on),
            "the edge lying in the face is confirmed: {errors:?}"
        );
        let off = edge_at(&body, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let b_top = face_at_z(&body, 2.0, true);
        let errors = census(&body, &one(&body, Cell::Edge(off), Cell::Face(b_top)));
        assert!(
            stale_edge_face(&errors, off, b_top),
            "an edge off the face's plane is stale: {errors:?}"
        );
        // Off the face's region, in its plane: the second brick's far
        // bottom edge, x = 1.5, against the first's top.
        let past = edge_at(&body, [1.5, 0.25, 1.0], [1.5, 0.75, 1.0]);
        let errors = census(&body, &one(&body, Cell::Edge(past), Cell::Face(a_top)));
        assert!(
            stale_edge_face(&errors, past, a_top),
            "an edge in the plane outside the region is stale: {errors:?}"
        );
    }

    /// A disc's circle, closed at one conventional vertex, under a
    /// brick standing over it: the record of the circle and the brick's
    /// bottom face is confirmed at that vertex, and backs the vertex's
    /// own touch there; against the brick's top face, a metre above the
    /// circle's plane and parallel to it, it is stale. Red when the
    /// conventional vertex is not read as the edge's, or the record
    /// backs nothing.
    #[test]
    fn a_curved_edge_on_a_face_is_read_at_its_conventional_vertex() {
        let tol = Tol::witness();
        let mut body = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol);
        let host = face_at_z(&body, 1.0, true);
        let at = body
            .half_edges()
            .find(|(_, h)| body.get_loop(h.parent_loop).unwrap().face == host)
            .map(|(k, _)| k)
            .unwrap();
        let disc = plant_disc_face(&mut body, at, Point3::new(1.0, 1.0, 1.0), 0.5, tol);
        let circle = disc.edge;
        let lid = brick::<f64>((1.25, 2.5), (0.5, 1.5), (1.0, 2.0), tol);
        crate::graft_disjoint_all_keyed(&mut body, &lid).unwrap();
        let under = face_at_z(&body, 1.0, false);
        let errors = census(&body, &one(&body, Cell::Edge(circle), Cell::Face(under)));
        assert!(
            !stale_edge_face(&errors, circle, under) && !unsupported_edge(&errors, circle),
            "the circle's conventional vertex lies in the lid's underside: {errors:?}"
        );
        let vertex = conventional_vertices(&snapshot(&body), circle);
        assert_eq!(vertex.len(), 1, "one conventional vertex");
        assert!(
            !errors
                .iter()
                .any(|e| matches!(e, ValidationError::UnrecordedContact {
                contact: CensusContact::VertexOnFace { vertex: v, face }, ..
            } if *v == vertex[0] && *face == under)),
            "the record backs the vertex's touch: {errors:?}"
        );
        let lid_top = face_at_z(&body, 2.0, true);
        let errors = census(&body, &one(&body, Cell::Edge(circle), Cell::Face(lid_top)));
        assert!(
            stale_edge_face(&errors, circle, lid_top) && !unsupported_edge(&errors, circle),
            "a circle parallel to a face a metre off is stale: {errors:?}"
        );
    }

    /// A cylinder sheet beside a brick whose face x = `x` is tangent to
    /// it along the ruling through (1, 0) at `x = 1`: the face–face
    /// record is confirmed along that line, and contradicted when the
    /// brick stands clear. Red when two carriers are read as a patch.
    #[test]
    fn a_face_pair_on_two_carriers_is_confirmed_along_its_tangent_line() {
        let tol = Tol::witness();
        let read = |x: f64| {
            let mut body = Body::<f64>::new();
            let wall = cyl_wall_sheet(
                &mut body,
                CylFrame::canonical(1.0),
                (-0.5, 0.5),
                (0.0, 1.0),
                tol,
            );
            let block = brick::<f64>((x, x + 1.0), (-0.5, 0.5), (0.0, 1.0), tol);
            let keys = crate::graft_disjoint_all_keyed(&mut body, &block).unwrap();
            let side = block
                .faces()
                .find(|(_, f)| match block.get_surface(f.surface) {
                    Some(geom::Surface::Plane { origin, normal, .. }) => {
                        normal.x.abs() > 0.5 && (origin.x - x).abs() < 1e-12
                    }
                    _ => false,
                })
                .and_then(|(k, _)| keys.face(k))
                .expect("the block's face x = x");
            let errors = census(&body, &one(&body, Cell::Face(wall), Cell::Face(side)));
            (errors, wall, side)
        };
        let named = |errors: &[ValidationError], a: FaceKey, b: FaceKey| {
            errors
                .iter()
                .filter(|e| match e {
                    ValidationError::ContactContradicted { declaration, .. } => {
                        [declaration.a, declaration.b] == [a, b]
                    }
                    ValidationError::CensusUnsupported {
                        subject: CensusSubject::FacePair(x, y),
                        ..
                    } => [*x, *y] == [a, b],
                    ValidationError::StaleContactDeclaration {
                        declaration: StaleDeclaration::Patch { face_a, face_b },
                    } => [*face_a, *face_b] == [a, b],
                    _ => false,
                })
                .cloned()
                .collect::<Vec<_>>()
        };
        let (errors, wall, side) = read(1.0);
        let (a, b) = (wall.min(side), wall.max(side));
        assert_eq!(named(&errors, a, b), vec![], "tangent: {errors:?}");
        let (errors, wall, side) = read(1.5);
        let (a, b) = (wall.min(side), wall.max(side));
        assert!(
            named(&errors, a, b).iter().any(|e| matches!(
                e,
                ValidationError::ContactContradicted {
                    declaration: crate::contact::DeclaredContact {
                        class: crate::contact::ContactClass::Tangent,
                        ..
                    },
                    ..
                }
            )),
            "clear: the curve touch is contradicted: {errors:?}"
        );
    }
    /// **One structural bound does not stand in for a collinear
    /// overlap's record.** The unit cube and a brick over x ∈ (0, 0.5),
    /// y ∈ (−1, 0), z ∈ (−1, 0), its two corners at the origin on one
    /// point: their edges along x overlap over (0, 0.5), bounded by the
    /// shared origin and by the brick's corner (0.5, 0, 0) on the
    /// cube's edge, whose `(vertex, edge)` record backs it. With the
    /// edge–edge record the body certifies; without it the overlap
    /// refuses, and nothing else does. Red if the structural carve-out
    /// of [`ee_collinear_lane`] reads either bound shared, not both.
    #[test]
    fn an_overlap_with_one_shared_bound_is_owed_its_edge_edge_record() {
        let tol = Tol::witness();
        let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let c = brick::<f64>((0.0, 0.5), (-1.0, 0.0), (-1.0, 0.0), tol);
        crate::graft_disjoint_all_keyed(&mut body, &c).unwrap();
        let at = |body: &Body<f64>, p: [f64; 3]| -> Vec<VertexKey> {
            body.vertices()
                .filter(|(_, v)| {
                    let r = body.points[v.point];
                    [r.x, r.y, r.z] == p
                })
                .map(|(k, _)| k)
                .collect()
        };
        let origin = at(&body, [0.0, 0.0, 0.0]);
        assert_eq!(origin.len(), 2, "a corner of each at the origin");
        let onto = body.vertices[origin[0]].point;
        body.share_point(&origin, onto).expect("both resolve");
        let tip = at(&body, [0.5, 0.0, 0.0]);
        assert_eq!(tip.len(), 1, "the brick's corner on the cube's edge");
        let tip = tip[0];
        let a_edge = edge_at(&body, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let c_edge = edge_at(&body, [0.0, 0.0, 0.0], [0.5, 0.0, 0.0]);
        let row = |x: Cell, y: Cell, k: u32| {
            crate::Cited::new(
                crate::Contact::new(&body, x, y).unwrap(),
                crate::Cites::decided(k),
            )
        };
        let bound = row(Cell::Vertex(tip), Cell::Edge(a_edge), 0);
        let overlap = row(Cell::Edge(a_edge), Cell::Edge(c_edge), 1);
        let recorded: ContactRecords = [bound.clone(), overlap].into_iter().collect();
        assert_eq!(
            census(&body, &recorded),
            vec![],
            "the bound's record and the overlap's: the body certifies"
        );
        let bound_alone: ContactRecords = [bound].into_iter().collect();
        let errors = census(&body, &bound_alone);
        assert!(
            matches!(
                errors[..],
                [ValidationError::UnrecordedContact {
                    contact: CensusContact::EdgeEdgeOverlap { a, b },
                    ..
                }] if [a, b] == [a_edge, c_edge] || [a, b] == [c_edge, a_edge]
            ),
            "one shared bound and one recorded: the overlap refuses: {errors:?}"
        );
    }
}

/// **A record a later edit made incident is stale.** The census takes a
/// body's records as its caller hands them, and a public edit after
/// they were made can turn a contact into structure: the merge of two
/// coplanar faces makes a vertex of the one it kills lie on the kept
/// face's boundary, and the join after it makes a vertex of one joined
/// edge an end of the edge it keeps.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod incident_after_an_edit {
    use super::*;
    use crate::boolean::{Cell as BCell, ContactRecords};
    use crate::test_support_fixtures::prism_z;
    use geom_core::Tol;

    fn at(body: &Body<f64>, p: [f64; 3]) -> VertexKey {
        body.vertex_points()
            .find(|(_, q)| [q.x, q.y, q.z] == p)
            .map(|(k, _)| k)
            .expect("the vertex")
    }

    /// The prism over a square with a station at (1, 0.5): its x = 1
    /// side is two coplanar faces, which the merge makes one, and the
    /// joins make its top and bottom one edge each.
    #[test]
    fn a_record_the_merge_and_join_made_incident_is_stale() {
        let tol = Tol::witness();
        let p = prism_z::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 0.5), (1.0, 1.0), (0.0, 1.0)],
            0.0,
            1.0,
            tol,
        );
        let mut body = p.body;
        let (low, high) = (p.side_faces[1], p.side_faces[2]);
        let near = at(&body, [1.0, 0.0, 1.0]);
        let far = at(&body, [1.0, 1.0, 1.0]);
        let edge_from = |v: VertexKey| {
            let m = at(&body, [1.0, 0.5, 1.0]);
            body.edges()
                .find(|(_, e)| {
                    let ends =
                        [e.he_plus, e.he_minus].map(|h| body.get_half_edge(h).unwrap().start);
                    ends.contains(&v) && ends.contains(&m)
                })
                .map(|(k, _)| k)
                .unwrap()
        };
        let (e_near, e_far) = (edge_from(near), edge_from(far));
        let pairs = [
            (BCell::Vertex(far), BCell::Edge(e_near)),
            (BCell::Vertex(near), BCell::Edge(e_far)),
            (BCell::Vertex(at(&body, [1.0, 1.0, 0.0])), BCell::Face(low)),
            (BCell::Vertex(at(&body, [1.0, 0.0, 0.0])), BCell::Face(high)),
        ];
        let records: ContactRecords = pairs
            .iter()
            .map(|&(x, y)| {
                crate::Cited::new(
                    crate::Contact::new(&body, x, y).unwrap(),
                    crate::Cites::decided(0),
                )
            })
            .collect();
        body.merge_coplanar_faces(tol)
            .expect("the x = 1 sides merge");
        let mut made_incident = 0;
        for &(x, y) in &pairs {
            let live = match y {
                BCell::Edge(e) => body.get_edge(e).is_some(),
                BCell::Face(f) => body.get_face(f).is_some(),
                BCell::Vertex(_) => unreachable!("a vertex record's other cell"),
            };
            if live {
                assert!(
                    crate::boolean::incident(&body, [x, y]),
                    "{x:?}, {y:?}: the edit made the live pair incident"
                );
                made_incident += 1;
            }
        }
        assert_eq!(made_incident, 2, "the kept edge and the kept face");
        let band = Band::linear(tol).unwrap();
        let errors = census_and_certify(&body, &records, band, tol, None);
        for &(x, y) in &pairs {
            let want = match (x, y) {
                (BCell::Vertex(vertex), BCell::Edge(edge)) => {
                    StaleDeclaration::VertexOnEdge { vertex, edge }
                }
                (BCell::Vertex(vertex), BCell::Face(face)) => {
                    StaleDeclaration::VertexOnFace { vertex, face }
                }
                _ => unreachable!("vertex records"),
            };
            assert!(
                errors.contains(&ValidationError::StaleContactDeclaration { declaration: want }),
                "{want:?} is stale: {errors:?}"
            );
        }
    }
}

/// **What a face–face record on two carriers confirms**
/// ([`crate::boolean::tangent_pair_at_rest`]): faces apart along the
/// line, the line off a face, materials overlapping in an internal
/// tangency, no closed-form locus; and what a straight edge–face record
/// backs.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod two_carrier_records {
    use super::*;
    use crate::boolean::{Cell, ContactRecords};
    use crate::test_support_fixtures::{CylFrame, brick, cyl_wall_sheet};
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn one(body: &Body<f64>, x: Cell, y: Cell) -> ContactRecords {
        [crate::Cited::new(
            crate::Contact::new(body, x, y).unwrap(),
            crate::Cites::decided(0),
        )]
        .into_iter()
        .collect()
    }

    /// Whether any finding names the pair `a`, `b` (either order).
    fn names(errors: &[ValidationError], a: FaceKey, b: FaceKey) -> bool {
        let is = |x: FaceKey, y: FaceKey| (x, y) == (a, b) || (x, y) == (b, a);
        errors.iter().any(|e| match e {
            ValidationError::ContactContradicted { declaration, .. } => {
                is(declaration.a, declaration.b)
            }
            ValidationError::CensusUnsupported {
                subject: CensusSubject::FacePair(x, y),
                ..
            } => is(*x, *y),
            ValidationError::StaleContactDeclaration {
                declaration: StaleDeclaration::Patch { face_a, face_b },
            } => is(*face_a, *face_b),
            ValidationError::CensusUndecidable {
                a: EntityId::Face(x),
                b: EntityId::Face(y),
                ..
            } => is(*x, *y),
            _ => false,
        })
    }

    /// A cylinder sheet (radius 1 about z, angles `u`, heights `v`) and
    /// a brick whose face x = 1 lies on the plane tangent to the
    /// cylinder along the ruling through (1, 0), the brick spanning
    /// heights `bz`; the census of the face-face record of the two.
    fn tangent_plane(
        u: (f64, f64),
        v: (f64, f64),
        bz: (f64, f64),
    ) -> (Vec<ValidationError>, FaceKey, FaceKey) {
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let wall = cyl_wall_sheet(&mut body, CylFrame::canonical(1.0), u, v, tol);
        let block = brick::<f64>((1.0, 2.0), (-0.5, 0.5), bz, tol);
        let keys = crate::graft_disjoint_all_keyed(&mut body, &block).unwrap();
        let side = block
            .faces()
            .find(|(_, f)| match block.get_surface(f.surface) {
                Some(geom::Surface::Plane { origin, normal, .. }) => {
                    normal.x.abs() > 0.5 && (origin.x - 1.0).abs() < 1e-12
                }
                _ => false,
            })
            .and_then(|(k, _)| keys.face(k))
            .unwrap();
        let errors = census_and_certify(
            &body,
            &one(&body, Cell::Face(wall), Cell::Face(side)),
            band(),
            tol,
            None,
        );
        (errors, wall, side)
    }

    /// The sheet and the brick span disjoint heights (0..1 and 5..6):
    /// the two faces are four metres apart along their tangent line, and
    /// the record is named. Red when the table runs over the union of
    /// the two extents rather than where they meet.
    #[test]
    fn a_two_carrier_record_of_faces_four_metres_apart_is_not_confirmed() {
        let (errors, wall, side) = tangent_plane((-0.5, 0.5), (0.0, 1.0), (5.0, 6.0));
        assert!(names(&errors, wall, side), "{errors:?}");
    }

    /// The sheet on the cylinder's far side (angles 2..3 rad), about two
    /// metres from the brick face: the tangent ruling is not in the
    /// sheet, and the record is named. Red when the line is never placed
    /// in the faces.
    #[test]
    fn a_two_carrier_record_of_faces_two_metres_apart_is_not_confirmed() {
        let (errors, wall, side) = tangent_plane((2.0, 3.0), (0.0, 1.0), (0.0, 1.0));
        assert!(names(&errors, wall, side), "{errors:?}");
    }

    /// A plate whose top face (z = 1) is notched across the tangent
    /// ruling's middle, and a cylinder sheet resting on it along y = 0
    /// over x in (0.5, 3.5): the ruling runs through the face's interior
    /// at both ends of that span and over the notch (x in (1.5, 2.5)) at
    /// its middle. The faces' interiors meet, and the record is
    /// confirmed. Red when the confirm reads the overlap's middle alone.
    #[test]
    fn a_two_carrier_record_whose_line_misses_a_notch_at_its_middle_is_confirmed() {
        let tol = Tol::witness();
        let notched = [
            (0.0, -1.0),
            (4.0, -1.0),
            (4.0, 1.0),
            (2.5, 1.0),
            (2.5, -0.5),
            (1.5, -0.5),
            (1.5, 1.0),
            (0.0, 1.0),
        ];
        let mut body = crate::test_support_fixtures::prism::<f64>(&notched, 1.0, tol).body;
        let top = body
            .faces()
            .find(|(_, f)| match body.get_surface(f.surface) {
                Some(geom::Surface::Plane { origin, normal, .. }) => {
                    normal.z.abs() > 0.5 && (origin.z - 1.0).abs() < 1e-12
                }
                _ => false,
            })
            .map(|(k, _)| k)
            .unwrap();
        let frame = CylFrame {
            origin: geom_core::Point3::new(0.0, 0.0, 2.0),
            axis: geom_core::Vec3::unit_x(),
            radius: 1.0,
            u_ref: -geom_core::Vec3::unit_z(),
        };
        let wall = cyl_wall_sheet(&mut body, frame, (-0.5, 0.5), (0.5, 3.5), tol);
        body.get_face_mut(wall).unwrap().sense = true;
        let placed = |x: f64| {
            crate::boolean::face_placement(
                &body,
                top,
                geom_core::Point3::new(x, 0.0, 1.0),
                band(),
                "test_notch",
            )
        };
        assert!(
            matches!(
                placed(2.0),
                Ok(crate::boolean::CurvedPlacement::Trim(Some(
                    FaceContainment::Out
                )))
            ) && matches!(
                placed(1.0),
                Ok(crate::boolean::CurvedPlacement::Trim(Some(
                    FaceContainment::In
                )))
            ),
            "the ruling is over the notch at x = 2 and in the face at x = 1"
        );
        let errors = census_and_certify(
            &body,
            &one(&body, Cell::Face(wall), Cell::Face(top)),
            band(),
            tol,
            None,
        );
        assert!(!names(&errors, wall, top), "{errors:?}");
    }

    /// The brick's face x = 1 + 3ε, past the zero band of the
    /// cylinder's tangent plane and inside its escalation edge: whether
    /// the two carriers touch is undecided, and the record's confirm
    /// escalates rather than reading it either way.
    #[test]
    fn a_two_carrier_record_in_the_band_escalates() {
        let tol = Tol::witness();
        let x = 1.0 + 3.0 * tol.eps();
        let mut body = Body::<f64>::new();
        let wall = cyl_wall_sheet(
            &mut body,
            CylFrame::canonical(1.0),
            (-0.5, 0.5),
            (0.0, 1.0),
            tol,
        );
        let block = brick::<f64>((x, x + 1.0), (-0.5, 0.5), (0.0, 1.0), tol);
        let keys = crate::graft_disjoint_all_keyed(&mut body, &block).unwrap();
        let side = block
            .faces()
            .find(|(_, f)| match block.get_surface(f.surface) {
                Some(geom::Surface::Plane { origin, normal, .. }) => {
                    normal.x.abs() > 0.5 && (origin.x - x).abs() < 1e-15
                }
                _ => false,
            })
            .and_then(|(k, _)| keys.face(k))
            .unwrap();
        let errors = census_and_certify(
            &body,
            &one(&body, Cell::Face(wall), Cell::Face(side)),
            band(),
            tol,
            None,
        );
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::CensusEscalated { .. }))
                && !names(&errors, wall, side),
            "{errors:?}"
        );
    }

    /// A straight edge's record backs nothing of its own (claim 1(a)):
    /// the second brick's bottom edge lies in the first's top over
    /// x in (0.5, 1), and its end (0.5, 0.25, 1) rests on that top's
    /// interior. With the edge-face record alone that vertex's touch is
    /// still unrecorded. Red when the record backs its edge's end
    /// vertices (a `conventional_vertices` that drops its filter is
    /// green on every other census, tier-3 and ops row).
    #[test]
    fn a_straight_edge_face_record_backs_no_touch_at_its_ends() {
        use crate::test_support_fixtures::brick;
        let tol = Tol::witness();
        let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let top = brick::<f64>((0.5, 1.5), (0.25, 0.75), (1.0, 2.0), tol);
        crate::graft_disjoint_all_keyed(&mut body, &top).unwrap();
        let at = |p: [f64; 3]| {
            body.vertices()
                .find(|(_, v)| {
                    let q = body.get_point(v.point).unwrap();
                    [q.x, q.y, q.z] == p
                })
                .map(|(k, _)| k)
                .unwrap()
        };
        let (v0, v1) = (at([0.5, 0.25, 1.0]), at([1.5, 0.25, 1.0]));
        let edge = body
            .edges()
            .find(|(_, e)| {
                let ends = [e.he_plus, e.he_minus].map(|h| body.get_half_edge(h).unwrap().start);
                ends == [v0, v1] || ends == [v1, v0]
            })
            .map(|(k, _)| k)
            .unwrap();
        let a_top = body
            .faces()
            .find(|(_, f)| match body.get_surface(f.surface) {
                Some(geom::Surface::Plane { origin, normal, .. }) => {
                    normal.z.abs() > 0.5
                        && (origin.z - 1.0).abs() < 1e-12
                        && ((normal.z > 0.0) == f.sense)
                }
                _ => false,
            })
            .map(|(k, _)| k)
            .unwrap();
        let errors = census_and_certify(
            &body,
            &one(&body, Cell::Edge(edge), Cell::Face(a_top)),
            band(),
            tol,
            None,
        );
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::UnrecordedContact {
                    contact: CensusContact::VertexOnFace { vertex, face },
                    ..
                } if *vertex == v0 && *face == a_top
            )),
            "{errors:?}"
        );
    }

    /// Two cylinder sheets with crossing axes have no closed-form
    /// tangent locus: their face-face record is refused typed, naming
    /// the pair and the contact lane. Pins `confirm_face_pair`'s
    /// `NotCertifiable` arm, which no other row in the topo suite
    /// reaches (a mutant that drops it is green there).
    #[test]
    fn a_two_carrier_record_with_no_closed_form_locus_refuses_typed() {
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let a = cyl_wall_sheet(
            &mut body,
            CylFrame::canonical(1.0),
            (-0.5, 0.5),
            (0.0, 1.0),
            tol,
        );
        let b = cyl_wall_sheet(
            &mut body,
            CylFrame::tilted(1.0, 0.3),
            (-0.5, 0.5),
            (0.0, 1.0),
            tol,
        );
        let errors = census_and_certify(
            &body,
            &one(&body, Cell::Face(a), Cell::Face(b)),
            band(),
            tol,
            None,
        );
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::CensusUnsupported {
                    subject: CensusSubject::FacePair(x, y),
                    cause: CensusUnsupportedCause::ContactLane(
                        crate::contact::ContactRefusal::NotCertifiable { .. }
                    ),
                } if (*x, *y) == (a.min(b), a.max(b)) || (*x, *y) == (a, b) || (*x, *y) == (b, a)
            )),
            "{errors:?}"
        );
    }

    /// Two sheets of two solids, internally tangent along x = 1: radius
    /// 1 about the z axis, its material inside (sense true), and radius
    /// 0.5 about (0.5, 0), its material OUTSIDE (sense false) — the
    /// cusp's senses (`tier3_tests::cusp_prism`) on two parts. The two
    /// materials overlap in the crescent between the circles. The
    /// normals at the line are opposed, so only the sign of `κ_rel` read
    /// against the material sides refuses the record, which would
    /// otherwise clear the backstop's finding for exactly that pair.
    #[test]
    fn an_interpenetrating_internal_tangency_is_not_confirmed() {
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let a = cyl_wall_sheet(
            &mut body,
            CylFrame::canonical(1.0),
            (-0.5, 0.5),
            (0.0, 1.0),
            tol,
        );
        let mut frame = CylFrame::canonical(0.5);
        frame.origin = geom_core::Point3::new(0.5, 0.0, 0.0);
        let b = cyl_wall_sheet(&mut body, frame, (-0.5, 0.5), (0.0, 1.0), tol);
        body.get_face_mut(a).unwrap().sense = true;
        body.get_face_mut(b).unwrap().sense = false;
        let bare = census_and_certify(&body, &ContactRecords::default(), band(), tol, None);
        assert!(
            names(&bare, a, b),
            "unrecorded, the backstop names the pair: {bare:?}"
        );
        let recorded = census_and_certify(
            &body,
            &one(&body, Cell::Face(a), Cell::Face(b)),
            band(),
            tol,
            None,
        );
        assert!(
            names(&recorded, a, b),
            "recorded, the record is refused: {recorded:?}"
        );
    }

    /// A face–face record whose sheet and brick face abut along their
    /// tangent ruling at the one height z = 1: the extents meet at an
    /// end of each, where the interiors do not meet, and the record is
    /// stale, not refused.
    #[test]
    fn a_two_carrier_record_whose_extents_meet_at_one_point_is_stale() {
        let (errors, wall, side) = tangent_plane((-0.5, 0.5), (0.0, 1.0), (1.0, 2.0));
        let stale = ValidationError::StaleContactDeclaration {
            declaration: StaleDeclaration::Patch {
                face_a: wall.min(side),
                face_b: wall.max(side),
            },
        };
        let flipped = ValidationError::StaleContactDeclaration {
            declaration: StaleDeclaration::Patch {
                face_a: wall.max(side),
                face_b: wall.min(side),
            },
        };
        assert!(
            (errors.contains(&stale) || errors.contains(&flipped))
                && !errors.iter().any(|e| matches!(
                    e,
                    ValidationError::CensusUnsupported {
                        subject: CensusSubject::FacePair(..),
                        ..
                    }
                )),
            "{errors:?}"
        );
    }

    /// **How the tangent line is read through a plate's top face**
    /// (`boolean::line_through_both_faces`): a prism plate with top face
    /// z = 1 and outline `outline`, a cylinder sheet (axis x through
    /// (0, 0, 2), radius 1) resting on it along y = 0 over x in `wx`,
    /// angles `u` from the ruling; the census of the face–face record in
    /// either cell order, at three bands.
    mod line_through_faces {
        use super::*;

        struct Rig {
            body: Body<f64>,
            wall: FaceKey,
            top: FaceKey,
        }

        fn rig(outline: &[(f64, f64)], wx: (f64, f64), u: (f64, f64)) -> Rig {
            let tol = Tol::witness();
            let mut body = crate::test_support_fixtures::prism::<f64>(outline, 1.0, tol).body;
            let top = body
                .faces()
                .find(|(_, f)| match body.get_surface(f.surface) {
                    Some(geom::Surface::Plane { origin, normal, .. }) => {
                        normal.z.abs() > 0.5 && (origin.z - 1.0).abs() < 1e-12
                    }
                    _ => false,
                })
                .map(|(k, _)| k)
                .unwrap();
            let frame = CylFrame {
                origin: geom_core::Point3::new(0.0, 0.0, 2.0),
                axis: geom_core::Vec3::unit_x(),
                radius: 1.0,
                u_ref: -geom_core::Vec3::unit_z(),
            };
            let wall = cyl_wall_sheet(&mut body, frame, u, wx, tol);
            body.get_face_mut(wall).unwrap().sense = true;
            Rig { body, wall, top }
        }

        fn bands() -> [Band; 3] {
            [
                Band::new(1e-10, 1e-9).unwrap(),
                Band::linear(Tol::witness()).unwrap(),
                Band::new(1e-6, 1e-5).unwrap(),
            ]
        }

        /// The census of the record at each band, both cell orders:
        /// `(band's escalation width, swapped, errors)`.
        fn census(r: &Rig) -> Vec<(f64, bool, Vec<ValidationError>)> {
            let mut out = Vec::new();
            for b in bands() {
                for swap in [false, true] {
                    let (x, y) = if swap {
                        (r.top, r.wall)
                    } else {
                        (r.wall, r.top)
                    };
                    let rec = one(&r.body, Cell::Face(x), Cell::Face(y));
                    out.push((
                        b.escalate(),
                        swap,
                        census_and_certify(&r.body, &rec, b, Tol::witness(), None),
                    ));
                }
            }
            out
        }

        fn stale(errors: &[ValidationError]) -> bool {
            errors.iter().any(|e| {
                matches!(
                    e,
                    ValidationError::StaleContactDeclaration {
                        declaration: StaleDeclaration::Patch { .. }
                    }
                )
            })
        }

        /// Every row confirms the record (nothing names the pair).
        fn confirmed(outline: &[(f64, f64)], wx: (f64, f64)) {
            let r = rig(outline, wx, (-0.5, 0.5));
            for (b, swap, errors) in census(&r) {
                assert!(
                    !names(&errors, r.wall, r.top),
                    "band {b:e} swap {swap}: {errors:?}"
                );
            }
        }

        /// Every row reads the record stale.
        fn read_stale(outline: &[(f64, f64)], wx: (f64, f64), u: (f64, f64)) {
            let r = rig(outline, wx, u);
            for (b, swap, errors) in census(&r) {
                assert!(stale(&errors), "band {b:e} swap {swap}: {errors:?}");
            }
        }

        /// A plate whose top face crosses y = 0 only over a tab
        /// x in (lo, hi); the rest of the outline is y < −0.5.
        fn tab(lo: f64, hi: f64) -> Vec<(f64, f64)> {
            vec![
                (0.0, -1.0),
                (4.0, -1.0),
                (4.0, -0.5),
                (hi, -0.5),
                (hi, 1.0),
                (lo, 1.0),
                (lo, -0.5),
                (0.0, -0.5),
            ]
        }

        const PLAIN: [(f64, f64); 4] = [(0.0, -1.0), (4.0, -1.0), (4.0, 1.0), (0.0, 1.0)];

        /// The interiors meet along x in (1.1, 1.4) only, a run between
        /// two of the certificate schedule's stations over the overlap
        /// [0, 4]: the record is confirmed. Red when the line is read
        /// at the schedule's stations rather than once per run between
        /// the faces' boundary crossings.
        #[test]
        fn a_tab_between_schedule_stations_is_confirmed() {
            confirmed(&tab(1.1, 1.4), (-1.0, 5.0));
        }

        /// A tab holding the schedule's station x = 1: confirmed.
        #[test]
        fn a_tab_on_a_schedule_station_is_confirmed() {
            confirmed(&tab(0.9, 1.1), (-1.0, 5.0));
        }

        /// The notched witness in every row: confirmed.
        #[test]
        fn the_notch_is_confirmed_in_every_row() {
            confirmed(
                &[
                    (0.0, -1.0),
                    (4.0, -1.0),
                    (4.0, 1.0),
                    (2.5, 1.0),
                    (2.5, -0.5),
                    (1.5, -0.5),
                    (1.5, 1.0),
                    (0.0, 1.0),
                ],
                (0.5, 3.5),
            );
        }

        /// An overlap 1e-4 long (the sheet over x in (3.9999, 5) on a
        /// plain plate [0, 4]), the line inside the plate along it:
        /// confirmed.
        #[test]
        fn a_short_overlap_is_confirmed() {
            confirmed(&PLAIN, (3.9999, 5.0));
        }

        /// The sheet over a plain plate, the line inside both along the
        /// whole overlap: confirmed.
        #[test]
        fn a_plain_plate_is_confirmed() {
            confirmed(&PLAIN, (0.5, 3.5));
        }

        /// The sheet's material outside its cylinder, so its sides
        /// contradict a contact with the plate: `(ε, swapped, errors)`
        /// of each row.
        fn census_opposed(outline: &[(f64, f64)]) -> (Rig, Vec<(f64, bool, Vec<ValidationError>)>) {
            let mut r = rig(outline, (0.5, 3.5), (-0.5, 0.5));
            r.body.get_face_mut(r.wall).unwrap().sense = false;
            let rows = census(&r);
            (r, rows)
        }

        /// The line inside both faces and the sheet's sides wrong: the
        /// record is contradicted in every row.
        #[test]
        fn a_line_through_both_faces_with_its_sides_wrong_is_contradicted() {
            let (r, rows) = census_opposed(&PLAIN);
            for (b, swap, errors) in rows {
                assert!(
                    names_contradicted(&errors, r.wall, r.top) && !stale(&errors),
                    "band {b:e} swap {swap}: {errors:?}"
                );
            }
        }

        /// The same sheet over a plate whose top face stops
        /// at y = −0.5: the extents along the line meet, but the line
        /// is in no run of the plate's face, so the two do not meet and
        /// the record is stale, whatever the sides. Red when the
        /// sides are read before the line is placed in the faces.
        #[test]
        fn a_line_off_a_face_is_stale_whatever_its_sides() {
            let (r, rows) = census_opposed(&[(0.0, -1.0), (4.0, -1.0), (4.0, -0.5), (0.0, -0.5)]);
            for (b, swap, errors) in rows {
                assert!(
                    stale(&errors) && !names_contradicted(&errors, r.wall, r.top),
                    "band {b:e} swap {swap}: {errors:?}"
                );
            }
        }

        fn names_contradicted(errors: &[ValidationError], a: FaceKey, b: FaceKey) -> bool {
            errors.iter().any(|e| {
                matches!(
                    e,
                    ValidationError::ContactContradicted { declaration, .. }
                        if [declaration.a, declaration.b] == [a, b]
                            || [declaration.a, declaration.b] == [b, a]
                )
            })
        }

        /// The census's reading of the record on `rig(outline, wx)` with
        /// the sheet's `sense`, in both cell orders, at the witness band:
        /// `stale`, `contradicted(..)`, `escalated(..)`, `unsupported`,
        /// or `confirmed` where nothing names the pair.
        fn reading(outline: &[(f64, f64)], wx: (f64, f64), sense: bool) -> [String; 2] {
            let mut r = rig(outline, wx, (-0.5, 0.5));
            r.body.get_face_mut(r.wall).unwrap().sense = sense;
            let b = Band::linear(Tol::witness()).unwrap();
            let (a, t) = (r.wall, r.top);
            let is = |x: FaceKey, y: FaceKey| (x, y) == (a, t) || (x, y) == (t, a);
            [(a, t), (t, a)].map(|(x, y)| {
                let errors = census_and_certify(
                    &r.body,
                    &one(&r.body, Cell::Face(x), Cell::Face(y)),
                    b,
                    Tol::witness(),
                    None,
                );
                let read: Vec<String> = errors
                    .iter()
                    .filter_map(|e| match e {
                        ValidationError::StaleContactDeclaration {
                            declaration: StaleDeclaration::Patch { face_a, face_b },
                        } if is(*face_a, *face_b) => Some("stale".to_owned()),
                        ValidationError::ContactContradicted {
                            declaration,
                            margin,
                            ..
                        } if is(declaration.a, declaration.b) => {
                            Some(format!("contradicted({:?})", margin.predicate))
                        }
                        ValidationError::CensusEscalated { cause } => {
                            Some(format!("escalated({:?})", cause.predicate))
                        }
                        ValidationError::CensusUnsupported { .. } => Some("unsupported".into()),
                        _ => None,
                    })
                    .collect();
                if read.is_empty() {
                    "confirmed".into()
                } else {
                    read.join("+")
                }
            })
        }

        /// Each `(what, outline, wx, sense, want)`: both cell orders'
        /// readings start with `want`.
        /// `(what, outline, wx, sense, want)`.
        type ClosedFormRow<'w> = (String, Vec<(f64, f64)>, (f64, f64), bool, &'w str);

        fn closed_form(rows: Vec<ClosedFormRow<'_>>) {
            let bad: Vec<String> = rows
                .into_iter()
                .filter_map(|(what, outline, wx, sense, want)| {
                    let got = reading(&outline, wx, sense);
                    (!got.iter().all(|g| g.starts_with(want)))
                        .then(|| format!("{what}: {got:?}, want {want}"))
                })
                .collect();
            assert!(bad.is_empty(), "{bad:#?}");
        }

        /// The plate's edge at y = d, the line at y = 0, the sheet's
        /// sides wrong and right. The line is outside the face for
        /// d ≤ zero (stale, whatever the sides), in band between zero
        /// and escalate (escalated), inside past escalate (the sides
        /// decide: contradicted or confirmed).
        #[test]
        fn an_edge_offset_from_the_line_reads_in_closed_form() {
            let b = Band::linear(Tol::witness()).unwrap();
            let (z, e) = (b.zero(), b.escalate());
            let mut rows = Vec::new();
            for (d, wrong, right) in [
                (-1e-2, "stale", "stale"),
                (-10.0 * e, "stale", "stale"),
                (-2.0 * e, "stale", "stale"),
                (-0.5 * z, "stale", "stale"),
                (0.0, "stale", "stale"),
                (0.5 * z, "stale", "stale"),
                (0.5 * (z + e), "escalated", "escalated"),
                (2.0 * e, "contradicted", "confirmed"),
                (10.0 * e, "contradicted", "confirmed"),
                (1e-2, "contradicted", "confirmed"),
            ] {
                let o = vec![(0.0, -1.0), (4.0, -1.0), (4.0, d), (0.0, d)];
                rows.push((
                    format!("d {d:e} wrong"),
                    o.clone(),
                    (0.5, 3.5),
                    false,
                    wrong,
                ));
                rows.push((format!("d {d:e} right"), o, (0.5, 3.5), true, right));
            }
            closed_form(rows);
        }

        /// The sheet starting at x = 4 + s on the plain plate [0, 4],
        /// its sides wrong: the extents overlap by −s, so the record is
        /// contradicted past escalate, escalated in band, stale within
        /// zero or beyond the plate.
        #[test]
        fn an_extent_end_offset_reads_in_closed_form() {
            let b = Band::linear(Tol::witness()).unwrap();
            let (z, e) = (b.zero(), b.escalate());
            closed_form(
                [
                    (-10.0 * e, "contradicted"),
                    (-0.5 * (z + e), "escalated"),
                    (-0.5 * z, "stale"),
                    (0.0, "stale"),
                    (0.5 * z, "stale"),
                    (10.0 * e, "stale"),
                ]
                .into_iter()
                .map(|(s, want)| {
                    (
                        format!("s {s:e}"),
                        PLAIN.to_vec(),
                        (4.0 + s, 5.0),
                        false,
                        want,
                    )
                })
                .collect(),
            );
        }

        /// A tab of width w across the line, and a notch apex grazing
        /// it, the sheet's sides wrong: contradicted past escalate; the
        /// graze is stale. (A tab within the band has an edge the prism
        /// fixture refuses to build.)
        #[test]
        fn a_tab_width_reads_in_closed_form() {
            let b = Band::linear(Tol::witness()).unwrap();
            let e = b.escalate();
            let mut rows: Vec<_> = [(10.0 * e, "contradicted"), (1e-3, "contradicted")]
                .into_iter()
                .map(|(w, want)| {
                    (
                        format!("w {w:e}"),
                        tab(1.1, 1.1 + w),
                        (-1.0, 5.0),
                        false,
                        want,
                    )
                })
                .collect();
            rows.push((
                "graze".into(),
                vec![
                    (0.0, -1.0),
                    (4.0, -1.0),
                    (4.0, -0.5),
                    (2.0, 0.0),
                    (0.0, -0.5),
                ],
                (-1.0, 5.0),
                false,
                "stale",
            ));
            closed_form(rows);
        }

        /// The line runs along the plate's edge y = 0: no run of it is
        /// inside the face, and the record is stale.
        #[test]
        fn the_line_along_the_plates_edge_is_stale() {
            read_stale(
                &[(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (0.0, 1.0)],
                (0.5, 3.5),
                (-0.5, 0.5),
            );
        }

        /// The line grazes the outline's only point on y ≥ 0, the apex
        /// (2, 0) of a notch: the interiors meet nowhere, and the record
        /// is stale.
        #[test]
        fn the_line_grazing_a_vertex_is_stale() {
            read_stale(
                &[
                    (0.0, -1.0),
                    (4.0, -1.0),
                    (4.0, -0.5),
                    (2.0, 0.0),
                    (0.0, -0.5),
                ],
                (-1.0, 5.0),
                (-0.5, 0.5),
            );
        }

        /// The plate's edge 3ε past the line (y ≤ 3ε at the witness
        /// band), so the plate reads every station in band, and the
        /// sheet over angles (0.05, 0.5), off the ruling, so the sheet
        /// decides every station out: the record is stale in both cell
        /// orders. Red when a station returns the first face's
        /// escalation without reading the second.
        #[test]
        fn a_station_the_second_face_decides_out_reads_stale_in_both_orders() {
            let y = 3.0 * Tol::witness().eps();
            let r = rig(
                &[(0.0, -1.0), (4.0, -1.0), (4.0, y), (0.0, y)],
                (0.5, 3.5),
                (0.05, 0.5),
            );
            let b = Band::linear(Tol::witness()).unwrap();
            for swap in [false, true] {
                let (x, z) = if swap {
                    (r.top, r.wall)
                } else {
                    (r.wall, r.top)
                };
                let errors = census_and_certify(
                    &r.body,
                    &one(&r.body, Cell::Face(x), Cell::Face(z)),
                    b,
                    Tol::witness(),
                    None,
                );
                assert!(stale(&errors), "swap {swap}: {errors:?}");
            }
        }

        /// Two spikes (slope 5) whose apexes stand 75ε above the line,
        /// near its ends (x = 0.6 and x = 3.4), and a wide tab (2, 2.4):
        /// each spike holds the line over two runs 15ε long, cut at its
        /// apex, whose middles lie about 7ε from its sides, so the plate
        /// reads them in band and they are held; the wide tab's run
        /// answers, whichever way the line runs. Red when the first held
        /// station's refusal is returned before a later station is read.
        #[test]
        fn a_held_station_does_not_stop_a_later_one_answering() {
            let h = 75.0 * Tol::witness().eps();
            let outline = [
                (0.0, -1.0),
                (4.0, -1.0),
                (4.0, -0.5),
                (3.5, -0.5),
                (3.4, h),
                (3.3, -0.5),
                (2.4, -0.5),
                (2.4, 1.0),
                (2.0, 1.0),
                (2.0, -0.5),
                (0.7, -0.5),
                (0.6, h),
                (0.5, -0.5),
                (0.0, -0.5),
            ];
            let r = rig(&outline, (-1.0, 5.0), (-0.5, 0.5));
            let b = Band::linear(Tol::witness()).unwrap();
            let line = crate::boolean::tangent_pair_at_rest(&r.body, r.wall, r.top, b);
            let Ok(crate::boolean::RestTangent::Line { carrier, overlap }) = line else {
                unreachable!("the sheet and the plate derive their tangent line");
            };
            let held = core::cell::Cell::new(0);
            let read = crate::boolean::line_through_both_faces(
                [(&r.body, r.top), (&r.body, r.wall)],
                &carrier,
                overlap,
                b,
                |_, _, out: &mut Vec<()>| {
                    held.set(held.get() + 1);
                    out.push(());
                },
            );
            assert!(
                matches!(read, Ok(true)) && held.get() >= 1,
                "{read:?}, {} refusals held",
                held.get()
            );
        }

        /// The plate's edge 3ε past the line along its length, so the
        /// plate reads every station inside the sheet in band, and the
        /// sheet on the ruling, so it reads them in: no station is
        /// decided, and the reading refuses rather than answer. Red when
        /// a reading that holds every station answers `false`.
        #[test]
        fn a_line_every_station_of_which_is_held_refuses() {
            let y = 3.0 * Tol::witness().eps();
            let r = rig(
                &[(0.0, -1.0), (4.0, -1.0), (4.0, y), (0.0, y)],
                (0.5, 3.5),
                (-0.5, 0.5),
            );
            let b = Band::linear(Tol::witness()).unwrap();
            let line = crate::boolean::tangent_pair_at_rest(&r.body, r.wall, r.top, b);
            let Ok(crate::boolean::RestTangent::Line { carrier, overlap }) = line else {
                unreachable!("the sheet and the plate derive their tangent line");
            };
            let read = crate::boolean::line_through_both_faces(
                [(&r.body, r.top), (&r.body, r.wall)],
                &carrier,
                overlap,
                b,
                |_, _, out: &mut Vec<()>| out.push(()),
            );
            assert!(
                matches!(read, Err(ref held) if !held.is_empty()),
                "{read:?}"
            );
        }

        /// Delta-review probes for PR 5006's boundary cuts: each row
        /// compares `line_through_both_faces` (one face read twice, so
        /// the answer is that face's own) and the census against an
        /// oracle that scans 20001 points of the line.
        mod review_5006_delta {
            use super::*;

            fn esc() -> f64 {
                Band::linear(Tol::witness()).unwrap().escalate()
            }

            /// `(read, oracle)`: the shared reading over `overlap` of
            /// the line through `(0, y, z)` along +x, `face` read twice;
            /// whether any scanned point is decided `In`.
            fn read_line(
                body: &Body<f64>,
                face: FaceKey,
                y: f64,
                z: f64,
                overlap: (f64, f64),
            ) -> (Result<bool, usize>, bool) {
                let b = Band::linear(Tol::witness()).unwrap();
                let origin = geom_core::Point3::new(0.0, y, z);
                let dir = geom_core::Vec3::unit_x();
                let carrier = geom::Curve3::Line { origin, dir };
                let read = crate::boolean::line_through_both_faces(
                    [(body, face), (body, face)],
                    &carrier,
                    overlap,
                    b,
                    |_, _, out: &mut Vec<()>| out.push(()),
                )
                .map_err(|h| h.len());
                let n = 20000;
                let oracle = (0..=n).any(|i| {
                    let t = overlap.0 + (overlap.1 - overlap.0) * f64::from(i) / f64::from(n);
                    matches!(
                        crate::boolean::face_placement(
                            body,
                            face,
                            origin + dir * t,
                            b,
                            "probe_oracle"
                        ),
                        Ok(crate::boolean::CurvedPlacement::Trim(Some(
                            FaceContainment::In
                        )))
                    )
                });
                (read, oracle)
            }

            fn top_of(outline: &[(f64, f64)]) -> (Body<f64>, FaceKey) {
                let r = rig(outline, (-1.0, 5.0), (-0.5, 0.5));
                (r.body, r.top)
            }

            fn verdict(tag: &str, read: Result<bool, usize>, oracle: bool) -> String {
                let bad =
                    matches!(read, Ok(false)) && oracle || matches!(read, Ok(true)) && !oracle;
                let s = format!(
                    "{tag} eps {:e}: read {read:?} oracle {oracle}{}",
                    Tol::witness().eps(),
                    if bad { "  <-- DISAGREE" } else { "" }
                );
                eprintln!("{s}");
                s
            }

            #[test]
            fn d1_tabs_a_few_bands_wide() {
                let mut rows = Vec::new();
                for k in [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 10.0, 100.0] {
                    let w = k * esc();
                    let Ok((body, top)) = std::panic::catch_unwind(|| top_of(&tab(1.1, 1.1 + w)))
                    else {
                        eprintln!("d1 tab {k}esc: fixture refused");
                        continue;
                    };
                    let (read, scanned) = read_line(&body, top, 0.0, 1.0, (0.0, 4.0));
                    // The scan's spacing (2e-4) passes over a tab this
                    // narrow, so the tab's middle is read as well: In
                    // there is a run of the line inside the face, and a
                    // middle in band is a run the reading may hold.
                    let middle = crate::boolean::face_placement(
                        &body,
                        top,
                        geom_core::Point3::new(1.1 + w / 2.0, 0.0, 1.0),
                        Band::linear(Tol::witness()).unwrap(),
                        "probe_oracle",
                    );
                    let oracle = scanned
                        || matches!(
                            middle,
                            Ok(crate::boolean::CurvedPlacement::Trim(Some(
                                FaceContainment::In
                            )))
                        );
                    let in_band = middle.is_err();
                    let tag = format!("d1 tab {k}esc");
                    if in_band && read.is_err() {
                        rows.push(format!("{tag}: middle in band, read held"));
                    } else {
                        rows.push(verdict(&tag, read, oracle));
                    }
                    // the census, both orders
                    let r = rig(&tab(1.1, 1.1 + w), (-1.0, 5.0), (-0.5, 0.5));
                    for swap in [false, true] {
                        let (x, y) = if swap {
                            (r.top, r.wall)
                        } else {
                            (r.wall, r.top)
                        };
                        let errors = census_and_certify(
                            &r.body,
                            &one(&r.body, Cell::Face(x), Cell::Face(y)),
                            Band::linear(Tol::witness()).unwrap(),
                            Tol::witness(),
                            None,
                        );
                        let st = stale(&errors);
                        let nm = names(&errors, r.wall, r.top);
                        let s = format!(
                            "d1c tab {k}esc swap {swap}: stale {st} named {nm} oracle {oracle}{}",
                            if st && oracle { "  <-- DISAGREE" } else { "" }
                        );
                        eprintln!("{s}");
                        rows.push(s);
                    }
                }
                assert!(!rows.iter().any(|r| r.contains("DISAGREE")), "{rows:#?}");
            }

            /// A roof: the face is below both lines y = s(x-1) and
            /// y = -s(x-3), apex (2, s) a vertex; and a single shallow
            /// edge y = s(x-2) whose face holds the line for x > 2, no
            /// vertex in (2, 4).
            #[test]
            fn d2_shallow_edges() {
                let mut rows = Vec::new();
                for s in [1e-2, 1e-4, 1e-6, 1e-7, 3e-8, 1e-8, 3e-9] {
                    let roof = [(0.0, -1.0), (4.0, -1.0), (4.0, -s), (2.0, s), (0.0, -s)];
                    let Ok((body, top)) = std::panic::catch_unwind(|| top_of(&roof)) else {
                        eprintln!("d2 s {s:e}: fixture refused");
                        continue;
                    };
                    let (read, oracle) = read_line(&body, top, 0.0, 1.0, (0.0, 4.0));
                    rows.push(verdict(&format!("d2 roof s {s:e}"), read, oracle));
                    let ramp = [(0.0, -1.0), (4.0, -1.0), (4.0, 2.0 * s), (0.0, -2.0 * s)];
                    let Ok((body, top)) = std::panic::catch_unwind(|| top_of(&ramp)) else {
                        continue;
                    };
                    let (read, oracle) = read_line(&body, top, 0.0, 1.0, (0.0, 4.0));
                    rows.push(verdict(&format!("d2 ramp s {s:e}"), read, oracle));
                    // a long ramp, its crossing far from the origin
                    let far = [(0.0, -1.0), (40.0, -1.0), (40.0, 2.0 * s), (0.0, -38.0 * s)];
                    let Ok((body, top)) = std::panic::catch_unwind(|| top_of(&far)) else {
                        continue;
                    };
                    let (read, oracle) = read_line(&body, top, 0.0, 1.0, (0.0, 40.0));
                    rows.push(verdict(&format!("d2 far ramp s {s:e}"), read, oracle));
                }
                assert!(!rows.iter().any(|r| r.contains("DISAGREE")), "{rows:#?}");
            }

            /// The half-disc cap (z = 0) and a cylinder sheet (axis y
            /// through (0, 0, 1)) resting on it along x = 0: the line runs
            /// through the cap over y in (0, 1), but both cap vertices
            /// project to y = 0 along it.
            #[test]
            fn d4_an_arc_bulging_past_its_vertices() {
                let tol = Tol::witness();
                let (mut body, cap, _) = crate::test_support_fixtures::half_disc_cap(0.0, tol);
                let frame = CylFrame {
                    origin: geom_core::Point3::new(0.0, 0.0, 1.0),
                    axis: geom_core::Vec3::unit_y(),
                    radius: 1.0,
                    u_ref: -geom_core::Vec3::unit_z(),
                };
                let wall = cyl_wall_sheet(&mut body, frame, (-0.5, 0.5), (-2.0, 2.0), tol);
                body.get_face_mut(wall).unwrap().sense = true;
                let b = Band::linear(tol).unwrap();
                let line = crate::boolean::tangent_pair_at_rest(&body, wall, cap, b);
                let desc = match &line {
                    Ok(crate::boolean::RestTangent::Line { overlap, .. }) => {
                        format!("Line overlap {overlap:?}")
                    }
                    Ok(crate::boolean::RestTangent::Apart) => "Apart".to_owned(),
                    Ok(crate::boolean::RestTangent::NoLine) => "NoLine".to_owned(),
                    Err(e) => format!("Err {e:?}"),
                };
                let errors = census_and_certify(
                    &body,
                    &one(&body, Cell::Face(wall), Cell::Face(cap)),
                    b,
                    tol,
                    None,
                );
                let s = format!(
                    "d4 eps {:e}: rest {desc}; stale {}; {errors:?}",
                    tol.eps(),
                    stale(&errors)
                );
                eprintln!("{s}");
                assert!(!stale(&errors), "{s}");
            }

            /// The unit half-disc cap (arc through +y, chord y = 0) and
            /// the line y = 1 - d: tangent to the arc at d = 0, a chord
            /// of depth d otherwise; and y = 0, along the chord.
            #[test]
            fn d3_conic_trims() {
                let mut rows = Vec::new();
                let (body, cap, _) =
                    crate::test_support_fixtures::half_disc_cap(0.0, Tol::witness());
                for k in [0.0, 0.5, 1.0, 1.5, 2.0, 3.0, 10.0, 1e3] {
                    let d = k * esc();
                    let (read, oracle) = read_line(&body, cap, 1.0 - d, 0.0, (-2.0, 2.0));
                    rows.push(verdict(&format!("d3 depth {k}esc"), read, oracle));
                }
                for y in [0.0, 0.5, -1e-3] {
                    let (read, oracle) = read_line(&body, cap, y, 0.0, (-2.0, 2.0));
                    rows.push(verdict(&format!("d3 y {y}"), read, oracle));
                }
                // the line through the plane at an angle: not in the plane
                assert!(!rows.iter().any(|r| r.contains("DISAGREE")), "{rows:#?}");
            }
        }
    }
}

/// **What the edge–face confirm reads off a line and a plane.**
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod edge_face_residual {
    use super::*;
    use crate::Contact;
    use crate::boolean::{Cell, ContactRecords};
    use crate::test_support_fixtures::brick;
    use geom_core::Tol;

    fn one(body: &Body<f64>, x: Cell, y: Cell) -> ContactRecords {
        [crate::Cited::new(
            crate::Contact::new(body, x, y).unwrap(),
            crate::Cites::decided(0),
        )]
        .into_iter()
        .collect()
    }

    /// **The residual is what makes a parallel edge above a face
    /// stale.** The upper brick's top edge along y = 0.25, at z = 2,
    /// projects into the lower brick's top face (z = 1) over
    /// x in (0.5, 1): the containment door reads that projection In,
    /// so only the ends' residuals refuse the record.
    #[test]
    fn a_parallel_edge_above_a_face_projecting_into_it_is_stale() {
        let tol = Tol::witness();
        let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let top = brick::<f64>((0.5, 1.5), (0.25, 0.75), (1.0, 2.0), tol);
        crate::graft_disjoint_all_keyed(&mut body, &top).unwrap();
        let at = |v: VertexKey| {
            let r = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            [r.x, r.y, r.z]
        };
        let (p, q) = ([0.5, 0.25, 2.0], [1.5, 0.25, 2.0]);
        let edge = body
            .edges()
            .find(|(_, e)| {
                let ends =
                    [e.he_plus, e.he_minus].map(|h| at(body.get_half_edge(h).unwrap().start));
                ends == [p, q] || ends == [q, p]
            })
            .map(|(k, _)| k)
            .unwrap();
        let face = body
            .faces()
            .find(|&(k, f)| {
                matches!(body.get_surface(f.surface),
                    Some(geom::Surface::Plane { origin, normal, .. })
                        if normal.z.abs() > 0.5 && (origin.z - 1.0).abs() < 1e-12)
                    && body.vertices().any(|(v, _)| {
                        at(v) == [0.0, 0.0, 1.0]
                            && Contact::new(&body, Cell::Vertex(v), Cell::Face(k)).is_err()
                    })
            })
            .map(|(k, _)| k)
            .unwrap();
        let errors = census_and_certify(
            &body,
            &one(&body, Cell::Edge(edge), Cell::Face(face)),
            Band::linear(tol).unwrap(),
            tol,
            None,
        );
        assert!(
            errors.contains(&ValidationError::StaleContactDeclaration {
                declaration: StaleDeclaration::EdgeFace { edge, face },
            }),
            "an edge a metre above the face is stale: {errors:?}"
        );
    }
}

/// **A point touch** (C3): a cylinder sheet's bottom rim, the arc of the
/// unit circle about z at z = 0 over azimuths `u`, against a brick face
/// x = `x` (y over `y`, z over −0.5..1.5), read by the record confirm.
/// At x = 1 the circle touches the face's plane at (1, 0, 0).
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod point_touch {
    use super::*;
    use crate::boolean::Cell;
    use crate::test_support_fixtures::{CylFrame, brick, cyl_wall_sheet};
    use geom_core::Tol;

    fn rim_on_face(u: (f64, f64), x: f64, y: (f64, f64)) -> Result<bool, ValidationError> {
        rim_on_brick(CylFrame::canonical(1.0), u, x, y, (-0.5, 1.5))
    }

    /// The rim at v = 0 of `frame` over `u` against the face x = `x` of
    /// a brick on its +x side, the face's region y over `y`, z over `z`.
    fn rim_on_brick(
        frame: CylFrame,
        u: (f64, f64),
        x: f64,
        y: (f64, f64),
        z: (f64, f64),
    ) -> Result<bool, ValidationError> {
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        cyl_wall_sheet(&mut body, frame, u, (0.0, 1.0), tol);
        let xyz = |p: Point3<f64>| [p.x, p.y, p.z];
        let (p, q) = (xyz(frame.at(u.0, 0.0)), xyz(frame.at(u.1, 0.0)));
        let at = |v: VertexKey| xyz(*body.get_point(body.get_vertex(v).unwrap().point).unwrap());
        let rim = body
            .edges()
            .find(|(_, e)| {
                let ends =
                    [e.he_plus, e.he_minus].map(|h| at(body.get_half_edge(h).unwrap().start));
                ends == [p, q] || ends == [q, p]
            })
            .map(|(k, _)| k)
            .expect("the bottom rim");
        let block = brick::<f64>((x, x + 1.0), y, z, tol);
        let keys = crate::graft_disjoint_all_keyed(&mut body, &block).unwrap();
        let side = block
            .faces()
            .find(|(_, f)| match block.get_surface(f.surface) {
                Some(geom::Surface::Plane { origin, normal, .. }) => {
                    normal.x.abs() > 0.5 && (origin.x - x).abs() <= 1e-15 * x.abs().max(1.0)
                }
                _ => false,
            })
            .and_then(|(k, _)| keys.face(k))
            .expect("the brick's face x = x");
        RecordConfirm::of(&body, Band::linear(tol).unwrap(), None)
            .meets(crate::Contact::new(&body, Cell::Edge(rim), Cell::Face(side)).unwrap())
    }

    /// Touching at a point of the arc's interior, inside the face: the
    /// record is confirmed. Red where the census has no point lane (the
    /// record refused typed).
    #[test]
    fn a_circle_touching_a_plane_inside_both_cells_is_confirmed() {
        assert_eq!(
            rim_on_face((-0.5, 0.5), 1.0, (-0.5, 0.5)),
            Ok(true),
            "the rim touches the face at (1, 0, 0)"
        );
    }

    /// Each reading the lane certifies, failed alone, reads stale: the
    /// face half a metre off (the carriers apart), five centimetres in
    /// (the circle crossing the plane), the face's region clear of the
    /// touch point, and the arc clear of it.
    #[test]
    fn a_circle_off_crossing_or_beside_a_plane_face_is_stale() {
        for (what, u, x, y) in [
            ("apart", (-0.5, 0.5), 1.5, (-0.5, 0.5)),
            ("crossing", (-0.5, 0.5), 0.95, (-0.5, 0.5)),
            ("the point outside the face", (-0.5, 0.5), 1.0, (0.1, 0.6)),
            ("the point outside the arc", (0.1, 0.6), 1.0, (-0.5, 0.5)),
            ("the point at the arc's end", (0.0, 0.6), 1.0, (-0.5, 0.5)),
        ] {
            assert_eq!(rim_on_face(u, x, y), Ok(false), "{what}");
        }
    }

    /// The face 3ε past the touch: whether the carriers coincide there
    /// is undecided, and the confirm escalates.
    #[test]
    fn a_circle_touch_in_the_band_escalates() {
        let x = 1.0 + 3.0 * Tol::witness().eps();
        assert!(
            matches!(
                rim_on_face((-0.5, 0.5), x, (-0.5, 0.5)),
                Err(ValidationError::CensusEscalated { .. })
            ),
            "{:?}",
            rim_on_face((-0.5, 0.5), x, (-0.5, 0.5))
        );
    }

    /// The touch point on the face's boundary is not inside it: on an
    /// edge of the face, and at its corner, the record is stale.
    #[test]
    fn a_circle_touching_a_plane_face_on_its_boundary_is_stale() {
        for (what, y, z) in [
            ("on the face's edge y = 0", (0.0, 1.0), (-0.5, 0.5)),
            ("at the face's corner", (0.0, 1.0), (0.0, 1.0)),
        ] {
            assert_eq!(
                rim_on_brick(CylFrame::canonical(1.0), (-0.5, 0.5), 1.0, y, z),
                Ok(false),
                "{what}"
            );
        }
    }

    /// A unit circle nearly parallel to the plane it touches, its
    /// sagitta over the radius `2k·ε` (`k` the band's escalate over
    /// zero): `κ_rel` clears the band and the touch point sits off the
    /// circle's centre plane by about `4kε`, which the reading keeps.
    /// At `3ε` the separation is undecided; at `0.5ε` the circle lies in
    /// the plane through its centre (a curve touch, refused typed), or
    /// stands apart a metre off or `1.5kε` off.
    #[test]
    fn a_circle_nearly_parallel_to_a_plane_is_read_at_its_touch_point() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (eps, k) = (band.zero(), band.escalate() / band.zero());
        for (what, sagitta, off, want) in [
            ("sagitta 2kε, touching", 2.0 * k, None, "met"),
            ("sagitta 3ε, touching", 3.0, None, "escalated"),
            (
                "sagitta 0.5ε, through the centre",
                0.5,
                Some(0.0),
                "unsupported",
            ),
            ("sagitta 0.5ε, a metre off", 0.5, Some(1.0), "stale"),
            ("sagitta 0.5ε, 1.5kε off", 0.5, Some(1.5 * k * eps), "stale"),
        ] {
            let frame = CylFrame::tilted(1.0, (2.0 * sagitta * eps).acos());
            let tp = frame.at::<f64>(0.0, 0.0);
            let got = match rim_on_brick(
                frame,
                (-0.5, 0.5),
                off.unwrap_or(tp.x),
                (tp.y - 0.5, tp.y + 0.5),
                (tp.z - 0.5, tp.z + 0.5),
            ) {
                Ok(true) => "met",
                Ok(false) => "stale",
                Err(ValidationError::CensusEscalated { .. }) => "escalated",
                Err(ValidationError::CensusUnsupported { .. }) => "unsupported",
                Err(e) => panic!("{what}: {e:?}"),
            };
            assert_eq!(got, want, "{what}");
        }
    }
}

/// Pass 5′ ([`sweep_curved_edge_edge`]): a curved edge's interior
/// meeting another edge's, unrecorded, is found; recorded, or apart, it
/// is not.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod curved_edge_edge {
    use super::*;
    use crate::boolean::{Cell, ContactRecords};
    use crate::euler::{FaceSurface, MefSite, MevSite};
    use crate::test_support_fixtures::{brick, unit_cyl_sheet};
    use geom::{Curve3, Surface};
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
    use geom_core::{Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn census(body: &Body<f64>, records: &ContactRecords) -> Vec<ValidationError> {
        census_and_certify(body, records, band(), Tol::witness(), None)
    }

    fn edge_pair(body: &Body<f64>, a: EdgeKey, b: EdgeKey) -> ContactRecords {
        [crate::Cited::new(
            crate::Contact::new(body, Cell::Edge(a), Cell::Edge(b)).unwrap(),
            crate::Cites::decided(0),
        )]
        .into_iter()
        .collect()
    }

    /// The planar half-disc cap in `z = 0`: the unit circle's arc over
    /// `[0, π]` and its chord. Returns the body and the arc.
    fn half_disc_cap() -> (Body<f64>, EdgeKey) {
        let mut body = Body::<f64>::new();
        let arc = add_half_disc(&mut body, Point3::origin(), Vec3::unit_z(), Vec3::unit_x());
        (body, arc)
    }

    /// A planar half-disc cap of radius 1 about `center` in the plane
    /// normal to `axis`, its arc over `[0, π]` from `u_ref` toward
    /// `axis × u_ref`, closed by its chord, as a lamina of its own in
    /// `body`. Returns the arc.
    fn add_half_disc(
        body: &mut Body<f64>,
        center: Point3<f64>,
        axis: Vec3<f64>,
        u_ref: Vec3<f64>,
    ) -> EdgeKey {
        let tol = Tol::witness();
        let circle = Curve3::Circle {
            center,
            axis,
            radius: 1.0,
            u_ref,
        };
        let (p0, p1) = (circle.eval(0.0), circle.eval(std::f64::consts::PI));
        let seed = body.mvfs(p0, true).expect("a seed");
        let plane = body.add_surface(Surface::Plane {
            origin: center,
            normal: axis,
            u_ref,
        });
        let cyl = body.add_surface(Surface::Cylinder {
            origin: center,
            axis,
            radius: 1.0,
            u_ref,
        });
        let arc = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p1,
                EdgeCurveSpec {
                    description: EdgeDescriptionSpec::Intersection {
                        pair: SurfacePair::new(cyl, plane),
                        witness: circle.eval(std::f64::consts::FRAC_PI_2),
                    },
                    carrier: circle,
                    param_start: 0.0,
                    param_end: std::f64::consts::PI,
                },
                tol,
            )
            .expect("the arc");
        body.mef(
            MefSite::Chords {
                he1: arc.he_minus,
                he2: arc.he_plus,
            },
            EdgeCurveSpec::line_between(p1, p0),
            FaceSurface::Shared {
                key: plane,
                sense: true,
            },
            tol,
        )
        .expect("the chord closes the cap");
        arc.edge
    }

    /// The edge of `body` between two points.
    fn edge_at(body: &Body<f64>, p: Point3<f64>, q: Point3<f64>) -> EdgeKey {
        let at = |v: VertexKey| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let near = |a: Point3<f64>, b: Point3<f64>| (a - b).norm() < 1e-12;
        body.edges()
            .find(|(_, e)| {
                let (a, b) = (
                    at(body.get_half_edge(e.he_plus).unwrap().start),
                    at(body.get_half_edge(e.he_minus).unwrap().start),
                );
                (near(a, p) && near(b, q)) || (near(a, q) && near(b, p))
            })
            .map(|(k, _)| k)
            .expect("the edge")
    }

    fn unrecorded_crossings(errors: &[ValidationError]) -> Vec<(EdgeKey, EdgeKey)> {
        errors
            .iter()
            .filter_map(|e| match e {
                ValidationError::UnrecordedContact {
                    contact: CensusContact::EdgeEdgeCross { a, b, side: None },
                    ..
                } => Some((*a, *b)),
                _ => None,
            })
            .collect()
    }

    /// **A line edge through an arc's interior, unrecorded, is found at
    /// tier 3′.** A brick whose vertical corner edge stands on the
    /// half-disc cap's arc at 45°, the brick outside the disc: the corner
    /// edge and the arc meet at `(√½, √½, 0)`, inside both, and the
    /// census pushes that crossing unrecorded. Recorded, the census finds
    /// nothing there and the record is confirmed; with the brick 1 mm
    /// out, nothing meets. Red when tier 3′'s edge-edge pass reads lines
    /// only.
    #[test]
    fn an_unrecorded_curved_edge_meeting_is_found_at_tier_three_prime() {
        let tol = Tol::witness();
        let s = 0.5_f64.sqrt();
        for (gap, meets) in [(0.0, true), (1e-3, false)] {
            let (mut body, arc) = half_disc_cap();
            let c = s + gap;
            let block = brick::<f64>((c, c + 1.0), (c, c + 1.0), (-0.5, 0.5), tol);
            crate::graft_disjoint_all_keyed(&mut body, &block).unwrap();
            let corner = edge_at(&body, Point3::new(c, c, -0.5), Point3::new(c, c, 0.5));
            let errors = census(&body, &ContactRecords::default());
            let want = if meets { vec![(arc, corner)] } else { vec![] };
            assert_eq!(unrecorded_crossings(&errors), want, "{gap}: {errors:?}");
            if meets {
                let errors = census(&body, &edge_pair(&body, arc, corner));
                assert!(
                    unrecorded_crossings(&errors).is_empty()
                        && !errors
                            .iter()
                            .any(|e| matches!(e, ValidationError::StaleContactDeclaration { .. })),
                    "the record backs the crossing and is confirmed: {errors:?}"
                );
            }
        }
    }

    /// **Two arcs meeting inside both, unrecorded, are found.** A second
    /// half-disc in the plane `x = 0`, about `(0, 1, 1)`, hangs its arc
    /// down to touch the first cap's arc at `(0, 1, 0)`, the two arcs
    /// crossing there at right angles and the two caps meeting nowhere
    /// else; recorded, the touch is backed and the record confirmed.
    #[test]
    fn two_arcs_meeting_inside_both_are_found_unrecorded() {
        let (mut body, a) = half_disc_cap();
        let b = add_half_disc(
            &mut body,
            Point3::new(0.0, 1.0, 1.0),
            Vec3::unit_x(),
            -Vec3::unit_y(),
        );
        let errors = census(&body, &ContactRecords::default());
        assert_eq!(unrecorded_crossings(&errors), vec![(a, b)], "{errors:?}");
        let errors = census(&body, &edge_pair(&body, a, b));
        assert!(
            unrecorded_crossings(&errors).is_empty()
                && !errors
                    .iter()
                    .any(|e| matches!(e, ValidationError::StaleContactDeclaration { .. })),
            "the record backs the touch and is confirmed: {errors:?}"
        );
    }

    /// **Two arcs of one circle overlapping, unrecorded, are an
    /// `EdgeEdgeOverlap`.** A cylinder wall sheet over `[0.2, 1.6]`
    /// standing on the half-disc cap's rim: its bottom arc lies along
    /// the cap's arc. Recorded, the overlap is backed.
    #[test]
    fn two_arcs_of_one_circle_overlapping_are_found_unrecorded() {
        let (mut body, arc) = half_disc_cap();
        unit_cyl_sheet(
            &mut body,
            None,
            (0.2, 1.6),
            (0.0, 1.0),
            true,
            Tol::witness(),
        );
        let rim = body
            .edges()
            .map(|(k, _)| k)
            .find(|&k| {
                k != arc
                    && body
                        .get_edge(k)
                        .and_then(|d| body.get_curve_geom(d.curve))
                        .and_then(|g| g.certified())
                        .is_some_and(|c| {
                            matches!(c.carrier(), Curve3::Circle { center, .. } if center.z == 0.0)
                        })
            })
            .expect("the sheet's bottom rim");
        let overlaps = |errors: &[ValidationError]| {
            errors
                .iter()
                .filter_map(|e| match e {
                    ValidationError::UnrecordedContact {
                        contact: CensusContact::EdgeEdgeOverlap { a, b },
                        ..
                    } => Some((*a, *b)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        };
        let errors = census(&body, &ContactRecords::default());
        assert_eq!(overlaps(&errors), vec![(arc, rim)], "{errors:?}");
        let errors = census(&body, &edge_pair(&body, arc, rim));
        assert!(overlaps(&errors).is_empty(), "{errors:?}");
    }
}

/// Pass 5's edge pairs read as segments ([`ee_meeting`]), never by
/// their lines alone: JOIN's near-tangent census witnesses, by their
/// exact coordinates, at the band they were measured in.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod edge_pairs_as_segments {
    use super::*;

    const AT_1E9: (f64, f64) = (1e-9, 1e-8);
    const AT_1E12: (f64, f64) = (1e-12, 1e-11);

    fn band((zero, escalate): (f64, f64)) -> Band {
        Band::new(zero, escalate).unwrap()
    }

    fn pt([x, y, z]: [f64; 3]) -> Point3<f64> {
        Point3::new(x, y, z)
    }

    fn seg(a: [f64; 3], b: [f64; 3]) -> EdgeGeo<f64> {
        segment((pt(a), pt(b)))
    }

    /// Whether the two edges' interiors meet, as the confirm pass reads
    /// it, with how many escalations the reading pushed.
    fn meet(ea: &EdgeGeo<f64>, eb: &EdgeGeo<f64>, at: (f64, f64)) -> (Option<bool>, usize) {
        let mut errors = Vec::new();
        let met = interiors_meet(ea, eb, band(at), &mut errors);
        (met, errors.len())
    }

    /// **Segments 9.8e-2 apart whose lines pass 1.8e-9 apart do not
    /// meet** (`w345 nt e0 a8 d1e-8 pc ∩`): the lines' closest point lies
    /// far outside an edge, which settles the pair whatever the in-band
    /// gap reads.
    #[test]
    fn segments_far_apart_whose_lines_pass_in_band_do_not_meet() {
        let ea = seg(
            [1.931_851_652_578_136_6, -0.517_638_090_205_041_4, 0.0],
            [0.0; 3],
        );
        let eb = seg(
            [
                -1.906_360_207_767_812_3,
                0.612_773_461_219_337_5,
                4.440_892_098_500_626e-16,
            ],
            [0.380_541_574_951_277_66, 0.0, 0.0],
        );
        assert_eq!(
            meet(&ea, &eb, AT_1E9),
            (Some(false), 0),
            "the pair, at ε = 1e-9"
        );
    }

    /// **Two edges ending on one point key, a few tenths of a
    /// microradian apart, meet only there**: their lines are read from
    /// that point, so the crossing is the point itself, at an end of
    /// both. Read through the closed form from the edges' starts, its
    /// rounding put the crossing 1.1e-9 off the point for the
    /// near-tangent census's pair (`notch307 nt e0 a0 d1e-7 pc ∩`, two
    /// edges continuing each other), in band at ε = 1e-9, and 5.7e-10
    /// off it for two edges arriving from one side, a definite interior
    /// crossing at ε = 1e-12.
    #[test]
    fn two_edges_ending_on_one_point_key_meet_only_there() {
        let at_v = [2.0, 1.0, 1.0];
        let continuing = (
            [3.788_854_381_999_840_4, 1.894_427_190_999_920_2, 1.0],
            [
                0.211_145_528_124_683,
                0.105_572_988_751_076_98,
                0.999_999_999_999_999_8,
            ],
        );
        let one_side = (
            [1.579_619_220_230_039_6, 0.789_809_610_115_019_8, 1.0],
            [0.166_424_127_480_476_3, 0.083_212_391_164_477_65, 1.0],
        );
        let mut keys = slotmap::SlotMap::<PointKey, ()>::with_key();
        let [far_a, far_b, v] = [(); 3].map(|()| Some(keys.insert(())));
        for (label, (a, b), at) in [
            ("continuing", continuing, AT_1E9),
            ("continuing", continuing, AT_1E12),
            ("from one side", one_side, AT_1E12),
        ] {
            let ea = EdgeGeo {
                points: [far_a, v],
                ..seg(a, at_v)
            };
            let eb = EdgeGeo {
                points: [far_b, v],
                ..seg(b, at_v)
            };
            assert!(
                (ee_cross_point(&ea, &eb) - pt(at_v)).norm() == 0.0,
                "{label}: the crossing is the shared point"
            );
            assert_eq!(meet(&ea, &eb, at), (Some(false), 0), "{label}, band {at:?}");
            assert_eq!(
                meet(&eb, &ea, at),
                (Some(false), 0),
                "{label} swapped, band {at:?}"
            );
        }
    }

    /// **Two edges leaving one point in opposite directions, collinear
    /// to 1e-8, are apart** (`vee300 nt e0 a0 d1e-8 cp ∖`): their
    /// parallel reading is in band, but they overlap nowhere, so neither
    /// of its verdicts finds a meeting.
    #[test]
    fn two_edges_end_to_end_through_one_point_are_apart() {
        let ea = seg([1.714_285_707_620_968_8, 0.0, 1.0], [2.0, 0.5, 1.0]);
        let eb = seg(
            [2.992_277_876_713_668, 2.236_486_284_248_919, 1.0],
            [2.0, 0.5, 1.0],
        );
        assert_eq!(meet(&ea, &eb, AT_1E9), (Some(false), 0), "the pair");
        assert_eq!(meet(&eb, &ea, AT_1E9), (Some(false), 0), "swapped");
    }

    /// **Two near-parallel segments 3.15e-8 apart are apart**
    /// (`w345 nt e0 a6 d1e-8 pc ∪`): the parallel reading is in band,
    /// and the overlap reading holds the short edge's whole run 3.15e-8
    /// clear of the long one.
    #[test]
    fn near_parallel_segments_three_bands_apart_are_apart() {
        let ea = seg(
            [1.999_999_999_999_999_8, -0.535_898_384_862_245_2, 1.0],
            [1.931_851_652_578_136_4, -0.517_638_090_205_041_3, 1.0],
        );
        let eb = seg(
            [0.0, 0.0, 1.0],
            [
                1.931_851_652_578_136_4,
                -0.517_638_090_205_041_7,
                0.999_999_968_473_814_6,
            ],
        );
        assert_eq!(meet(&ea, &eb, AT_1E9), (Some(false), 0), "the pair");
        assert_eq!(meet(&eb, &ea, AT_1E9), (Some(false), 0), "swapped");
    }

    /// **Two near-parallel segments crossing at their middles keep the
    /// parallel escalation**: the overlap reading finds them within the
    /// zero, so the in-band screen is not settled.
    #[test]
    fn near_parallel_segments_crossing_keep_their_escalation() {
        let ea = seg([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let eb = seg([0.0, -2.5e-9, 0.0], [1.0, 2.5e-9, 0.0]);
        assert_eq!(meet(&ea, &eb, AT_1E9), (None, 1), "the pair");
    }

    /// The overlap of `ea` and `eb` as the collinear lane reads it, in
    /// both orders, each with how many escalations it pushed.
    fn overlaps(ea: &EdgeGeo<f64>, eb: &EdgeGeo<f64>) -> [(bool, usize); 2] {
        [(ea, eb), (eb, ea)].map(|(a, b)| {
            let mut errors = Vec::new();
            let on = collinear_overlap(a, b, band(AT_1E9), &mut errors)
                .flatten()
                .is_some();
            (on, errors.len())
        })
    }

    /// **A short piece tilted within the zero overlaps a long one in
    /// either order**: read off the short piece's extended line at the
    /// long one's start, 0.36 away, its tilt levered the gap to
    /// 1.26e-9, in band.
    #[test]
    fn a_short_piece_tilted_within_the_zero_overlaps_a_long_one_in_either_order() {
        let short = seg([0.0; 3], [0.03, 0.0, 0.03 * 3.5e-9]);
        let long = seg([-0.36, 0.0, 0.0], [0.5, 0.0, 0.0]);
        assert_eq!(overlaps(&short, &long), [(true, 0); 2]);
    }

    /// **A long edge whose start lies on a short edge's line is read
    /// where the two run side by side** (`shallow200 nt e0 a3 d1e-7 pc ∪`):
    /// their projections overlap by 1.2e-8, over which the edges stand
    /// 2.1e-7 apart, so they do not overlap. Read at the long edge's
    /// start, on the short edge's line, the gap was zero and the pair an
    /// undeclared overlap.
    #[test]
    fn a_long_edge_starting_on_a_short_edges_line_is_read_beside_it() {
        let short = seg(
            [4.0, 1.0, 1.0],
            [3.996_756_400_362_216_7, 0.999_351_280_072_443_3, 1.0],
        );
        let long = seg(
            [2.0, 0.6, 1.0],
            [
                3.996_756_411_954_752_5,
                0.999_351_282_390_950_5,
                0.999_999_787_207_216,
            ],
        );
        assert_eq!(overlaps(&short, &long), [(false, 0); 2]);
    }

    /// **The overlap gap is the least offset over the run**, not the
    /// offset at its start: a unit piece whose start stands 1.5ε off a
    /// long edge and whose end 0.5ε (a 1ε turn, parallel Zero) overlaps
    /// it in either order. Read at the run's start alone, the gap lands
    /// in band and the pair escalates.
    #[test]
    fn the_overlap_gap_is_the_least_offset_over_the_run() {
        let long = seg([-1.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
        let short = seg([0.0, 1.5e-9, 0.0], [1.0, 0.5e-9, 0.0]);
        assert_eq!(overlaps(&short, &long), [(true, 0); 2]);
    }

    /// **Nearly parallel edges sharing no point key, crossing well
    /// inside both spans, are never read apart at `f64`**: each pair
    /// crosses at least `3Kε` inside both edges (measured exactly), at
    /// 2.8e-9 to 1.1e-7 rad. Read through the closed form from the
    /// edges' starts, its rounding put the crossing a definite span
    /// outside one edge at ε = 1e-12. Each reach and the gap are now
    /// read off an enclosure of their rounding, so where that rounding
    /// reaches the band the reading escalates, as `Interval`'s does.
    #[test]
    fn nearly_parallel_crossings_sharing_no_point_key_are_never_read_apart() {
        let pairs = [
            (
                (
                    [-0.57969420202416, 1.88862539895253, 1.125319280928707],
                    [-0.8888391370099953, 1.9912650048598994, 1.3512645386746862],
                ),
                (
                    [-0.8185163607393474, 1.9679170509070156, 1.2998676175168766],
                    [-0.8377476620810586, 1.974302057180716, 1.3139232301007768],
                ),
            ),
            (
                (
                    [0.9094635217007113, 0.795634715001027, -1.9525321423009292],
                    [0.906583440932391, 0.7871642771827733, -1.9439503768151147],
                ),
                (
                    [1.291972490856439, 1.9206095283007654, -3.0922925809420705],
                    [0.8310534972376821, 0.565027566261542, -1.7188941070352404],
                ),
            ),
            (
                (
                    [1.0194388820881284, 1.624640164893315, -0.018033492456674075],
                    [0.91675493431915, 1.5146244538520608, -0.027124376056758173],
                ),
                (
                    [1.0604926322270467, 1.668625203918349, -0.014398894659049784],
                    [
                        0.09459421682116398,
                        0.6337604301239435,
                        -0.09991245475486416,
                    ],
                ),
            ),
        ];
        for (i, ((a0, a1), (b0, b1))) in pairs.into_iter().enumerate() {
            let (ea, eb) = (seg(a0, a1), seg(b0, b1));
            for (label, (x, y)) in [("as built", (&ea, &eb)), ("swapped", (&eb, &ea))] {
                let (met, pushed) = meet(x, y, AT_1E12);
                assert_eq!(
                    (met, pushed > 0),
                    (None, true),
                    "pair {i} {label}: escalated, never a definite no"
                );
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod split_wedge_tests {
    use super::*;
    use crate::test_support_fixtures::{brick, split_plane};
    use crate::{AtRestBody, BooleanResult, SplitPart, split, union};
    use geom_core::Tol;

    /// **The split door hands back no coincident edges whose wedges
    /// overlap**, so its at-rest gate reads no line's wedges: the pinch
    /// of two bricks united along `(1, 1, z)`, split across the line, and
    /// along planes through it, leaves each side's coincident edges on
    /// the line bounding disjoint wedges ([`line_wedges`]). It pins the
    /// measurement the Boolean gate's PR took over every split side of
    /// the suites at three ε (no overlap); a split that pairs a pinch's
    /// sheets as the material does not goes red here.
    #[test]
    fn a_split_through_a_pinch_keeps_its_wedges_disjoint() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let at_rest = |b| AtRestBody::validate(b, tol).unwrap();
        let a = at_rest(brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol));
        let b = at_rest(brick::<f64>((1.0, 2.0), (1.0, 2.0), (0.0, 1.0), tol));
        let BooleanResult::Body(pinch) = union(&a, &b, tol).unwrap() else {
            panic!("the pinch is empty");
        };
        let mut read = 0;
        for (what, origin, normal) in [
            ("across the line", [1.0, 1.0, 0.5], [0.0, 0.0, 1.0]),
            (
                "along a diagonal through it",
                [1.0, 1.0, 0.0],
                [1.0, -1.0, 0.0],
            ),
            ("along a face through it", [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]),
        ] {
            let plane = split_plane(
                Point3::new(origin[0], origin[1], origin[2]),
                Vec3::new(normal[0], normal[1], normal[2]),
                tol,
            );
            let r = split(&pinch.body, &plane, tol).unwrap_or_else(|e| panic!("{what}: {e:?}"));
            for side in [r.above, r.below] {
                let SplitPart::Body(body) = side else {
                    continue;
                };
                let pairs = collinear_overlaps(&body, band).unwrap();
                read += pairs.len();
                assert_eq!(
                    line_wedges(&body, &pairs, band),
                    Ok(None),
                    "{what}: a side's coincident edges bound overlapping wedges"
                );
            }
        }
        assert!(
            read > 0,
            "a split side kept the pinch's two edges on the line"
        );
    }
}
