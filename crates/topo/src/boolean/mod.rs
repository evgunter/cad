//! Booleans, part 1 (M3 PR 4): **reduction + classification across two
//! bodies** — ch. 15 §§15.4–15.6 re-derived under our conventions, with
//! the TOG 1986 second witness supplying the unprinted on-edge
//! machinery (Tables II/III). Scope is BINDING (M3-PLAN PR item 4):
//! reduction sweep, the three ON-sets as declared-contact records
//! (F1/F2), vertex-vertex sector classification, vertex-on-face
//! classification with the ring insertion, on-edge machinery, and
//! paired null-edge insertion with explicit cross-body correspondence
//! keys (F9/F12). [`boolean_reduce`] itself stops there: both operands
//! are functionally untouched and the annotated clones come back in
//! [`BooleanReduction`]. Joining, result generation and the
//! containment fallback are the module's too — they arrived with PR 5
//! and after, and [`BooleanError`] carries their refusals — so a
//! reader deciding what belongs in this file should read the pipeline
//! below, not this paragraph, as its boundary.
//!
//! Pipeline of [`boolean_reduce`]:
//!
//! 1. **Gates**: per-arm since M5 PR 9 (C12.1 —
//!    [`BooleanError::CurvedBooleanUnsupported`] retires per C5 table
//!    arm; Plane/Cylinder/Sphere/Nurbs faces pass, Cone/Torus refuse);
//!    no scaffolding operands; **maximal faces (F7)** via the
//!    coincidence ladder — adjacent faces sharing a surface key or with
//!    bit-equal oriented planes ([`plane_eq`]) refuse as
//!    [`BooleanError::NonMaximalFaces`]; *numeric* coplanarity never
//!    triggers the precondition (it is not coincidence; if it bites, it
//!    bites later as a typed escalation — the ladder's honest shape).
//! 2. **Reduction sweep** (`reduce`): edge×face, BOTH directions,
//!    candidate generation through the `bvh` tree since M5 PR 8 (the
//!    tree prunes, predicates decide — `reduce` module docs; the
//!    brute-force scan survives as [`SweepStrategy::Idealized`] under
//!    the differential suite), with `contfv`/`contfp` as typed
//!    trilean case codes. Proper
//!    crossings insert vertices via the certified `split_edge` lane;
//!    edge-on-edge crossings are discovered as edge-face events landing
//!    ON an edge (both edges split → a v-v pair); coplanar edge-face
//!    pairs are skipped — their edge-edge events are caught when the
//!    edge meets the face's noncoplanar NEIGHBOR faces (tested).
//!    Every contact the sweep discovers or creates is emitted as a
//!    **declared-contact record** ([`ContactRecords`]) — the future
//!    tier-3′ declarations; nothing is ever scanned-for after the fact.
//! 3. **Classification** (`sectors`/`recl`/`tables`/`vtxfac`): v-v
//!    pairs via the all-pairs sector intersection search (Programs
//!    15.7–15.9 re-derived), on-sector reclassification (15.10 in
//!    full), on-edge reclassification (TOG Tables II/III as typed
//!    decision tables); v-on-f pairs via the ch. 14 classifier deltas
//!    (plane := the pierced face's plane, OUT/IN from
//!    [`geom_brep::enters_material`] — never 15.7's printed labels)
//!    plus the null-edge **ring insertion** into the pierced face.
//! 4. **Paired null-edge insertion** (`insert`): per consecutive
//!    surviving crossing-record pair, one null edge in each solid, F9
//!    attributes ([`crate::null::NullEdge`], below ≙ IN, above ≙ OUT)
//!    and explicit A↔B correspondence keys as data
//!    ([`NullEdgePairRecord`]) — never correlated array order
//!    (`ssortnulledges` is engineered out). The 15.11
//!    consecutive-pairing invariant is guarded at runtime (the pair
//!    must be cyclically adjacent in BOTH neighborhoods) and stressed
//!    by the 4-crossing fixtures (F12).
//!
//! # The 15.7 sign resolution (F3)
//!
//! Program 15.7 prints `IN = +1` with `s = comp(dot(feq, ref))` — i.e.
//! positive dot ⇒ IN, coherent only for an INWARD feq. TOG §2/§6.1
//! fixes outward normals (our ratified convention), so the printed
//! labeling is the suspect side. We derive from `enters_material`:
//! `dot(dir, n_outward) < 0 ⇒ Enters ⇒ IN`; positive ⇒ OUT. Mirror
//! tests pin both directions on brick fixtures.

pub(crate) mod boxes;
pub mod carrier_eq;
mod circle_sphere;
mod circle_torus;
pub(crate) mod combine;
pub mod contact_verify;
mod contain;
mod discard;
// The variant roster the sample-coverage row reads (test builds only).
#[cfg(test)]
pub(crate) use contain::ContainErrorKind;
mod finish;
pub(crate) mod insert;
mod join;
mod ops;
pub(crate) mod section_cert;
#[cfg(any(test, feature = "test-support"))]
pub(crate) use ops::no_crossings_certificates;
pub(crate) use ops::volume_backstop;
#[cfg(any(test, feature = "test-support"))]
pub(crate) use ops::{ChartCache, section_report};
pub mod plane_eq;
#[cfg(test)]
mod r2_probes;
pub(crate) mod recl;
pub(crate) mod reduce;
pub(crate) mod refusal_routes;
pub(crate) use refusal_routes::PlaneDoor;
pub use refusal_routes::{
    BooleanDecision, Coincide, Contradiction, CrossingDecision, DeclarationRead, LeverArm,
    NeighbourOffset, PlaneRung, RestZipFrontier, SectionRadius, SectorRung, SelfCheck, Settling,
    SphereQuestion, TorusConvention, WallRung,
};
mod rest;
mod rim_wedge;
pub(crate) mod sectors;
mod shell_witness;
pub mod solid_contain;
mod surface_group;
pub mod tables;
pub mod voids;
pub(crate) mod vtxfac;
mod zip;

use geom_core::{
    Band, BandError, Bounds, COINCIDENCE_RECOURSE, Decide, Indeterminate, KERNEL_DEFECT_ENDING,
    MarginDiag, Point3, Real, Tol,
};

use crate::body::Body;
use crate::chord_join::SplitJoinError;
use crate::contact::{BooleanCoincidence, ContactClass};
use crate::entity::{EdgeKey, FaceKey, ShellKey, VertexKey};
use crate::euler::EulerOpError;
use crate::merge_faces::MergeCoplanarError;
use crate::revert::RevertError;
use crate::validate::ValidationError;

pub use carrier_eq::{CarrierDesc, CarrierEqError, CarrierRelation, carrier_eq};
pub use contain::{ContainError, FaceContainment, contfp, curved_face_containment};
// Crate-internal: tier 3's check 9 decides two whole-circle loops
// against each other (its contact arm 4) on the same loop
// classification this module's own walk dispatches on.
pub(crate) use contain::loop_circle;
pub use discard::{DiscardRow, HeldEdge, fragment_root};
pub use join::CompletedPolygonPair;
pub use ops::{
    BooleanBody, BooleanNaming, BooleanResult, BooleanResultKind, OperandKeys, boolean_op_with,
    intersect, intersect_with, subtract, subtract_with, union, union_with,
};
pub use plane_eq::{PlaneDesc, PlaneEqError, PlaneIdentity, PlaneRelation, oriented_plane_eq};
#[cfg(feature = "sweep-testing")]
pub use reduce::PlantedDegradation;
pub use reduce::{SweepStrategy, SweepTrace};
// LIB-SEL2 (SELECT-DESIGN §3b; #304 review MINOR-1): THE flush-pair
// verify door — descriptions, oriented sources and the verification
// arm in one function, shared by the REST lane's verify-at-use and
// the detector's candidate-generation mode BY CONSTRUCTION.
pub use contact_verify::{contact_pair_verdict, tangent_pair_relation};
pub use rest::{carrier_pair_relation, carrier_pair_verdict, face_carrier, flush_pair_relation};
pub use solid_contain::{
    PointInSolidError, SolidContainment, SolidFaces, point_in_solid, point_in_solid_faces,
    point_in_solid_of,
};
pub use voids::{
    VoidContainment, VoidEvidence, VoidInsertError, VoidInserted, insert_void, insert_voids,
};

/// What one of this crate's decisions decides, in the words a flip
/// report states in place of the predicate's name (which is routing,
/// kept to `Debug`): a clause with no colon or dash of its own. `None`
/// for a predicate this crate has no words for yet, or does not own.
///
/// A flip report has only the name, so this lookup is by name; a
/// refusal carries its decision and reads the words there
/// ([`BooleanDecision::subject`]). Each row is the words of the one
/// decision every raise of that name is routed to, read from that
/// decision's closed type where it has one. A name raised under two
/// decisions has no words here, since neither decision's words are
/// true of it: `bool_contact_vertex` (the contact sweep's
/// [`BooleanDecision::VertexOnVertex`], and containment's boundary
/// pre-pass) and `bool_contact_arc` (the split point on its circle, and
/// the same pre-pass).
#[must_use]
pub fn decision_words(predicate: &str) -> Option<&'static str> {
    if let Some(words) = crate::sector_shape::rung_words(predicate) {
        return Some(words);
    }
    Some(match predicate {
        "bool_point_in_solid_plane" => "which side of a face's plane a point lies on",
        // The coincidences these names decide.
        "bool_vertex_face_side" => Coincide::VertexOnFace.subject(),
        "bool_conic_face_plane_offset" => Coincide::EdgeOnPlane.subject(),
        "bool_line_cylinder_clearance" => Coincide::EdgeOnCurvedFace.subject(),
        "bool_sector_within" => Coincide::Sectors.subject(),
        "bool_ee_collinear" => Coincide::EdgeOnEdge.subject(),
        "bool_plane_parallel" => PlaneRung::Parallel.subject(),
        "bool_plane_orient" => PlaneRung::Orientation.subject(),
        "carrier_cyl_axis_parallel" => "whether the two cylinders' axes are parallel",
        crate::query::DATUM_UNIT_NORM => geom_core::DIRECTION_LENGTH_SUBJECT,
        "bool_pierce_normal_on_chart" => BooleanDecision::PierceOnFace.subject(),
        // `geom`'s torus convention, which the pierce point's normal
        // reads before it differentiates the torus.
        "torus_tube_positive" => TorusConvention::Tube.subject(),
        "ring_torus_convention" => TorusConvention::Ring.subject(),
        "split_edge_param_interior" | "split_conic_crossing_root" | "bool_wall_root_in_span" => {
            CrossingDecision::OnEdge.subject()
        }
        "split_conic_root_order" => CrossingDecision::Order.subject(),
        "bool_split_span_period" => BooleanDecision::ArcSpan.subject(),
        "bool_face_disc_carrier"
        | "bool_contact_arc_end_vertex"
        | "bool_curved_contain_carrier"
        | "bool_curved_contain_period"
        | "bool_wall_trim"
        | "bool_wall_junction"
        | "bool_wall_outline_reach"
        | "bool_wall_piece_span"
        | "bool_wall_rim_level"
        | "bool_wrap_rim"
        | "bool_wall_section_tilt"
        | "bool_wall_trim_period"
        | "bool_wall_iso_meridian"
        | "bool_wall_iso_rim"
        | "bool_wall_section_seat"
        | "bool_sphere_iso_meridian"
        | "bool_sphere_iso_rim"
        | "bool_torus_trim_major_period"
        | "bool_torus_trim_minor_period"
        | "bool_sphere_trim"
        | "bool_sphere_trim_antipode"
        | "bool_sphere_trim_latitude"
        | "bool_sphere_trim_meridian_span"
        | "bool_sphere_trim_period"
        | "bool_sphere_trim_pole"
        | "bool_sphere_trim_pole_end"
        | "bool_sphere_trim_pole_interior"
        | "bool_torus_chart_affine"
        | "bool_torus_chart_box"
        | "bool_torus_chart_closure"
        | "bool_torus_frame_radius"
        | "bool_torus_trim"
        | "bool_cone_chart_box"
        | "bool_cone_group_slant"
        | "bool_cone_trim"
        | "bool_cone_trim_nappe"
        | "bool_cone_trim_period"
        | "bool_cone_trim_side"
        | "bool_ray_cone_apex"
        | "bool_ray_cone_nappe"
        | "point_in_loop_segment"
        | "point_in_loop_boundary"
        | "point_in_loop_side"
        | "point_in_loop_advance"
        | "point_in_loop_arm"
        | "point_in_arc_loop_segment"
        | "point_in_arc_loop_boundary"
        | "point_in_arc_loop_boundary_disagreement"
        | "point_in_arc_loop_side"
        | "point_in_arc_loop_advance"
        | "point_in_arc_loop_arm"
        | "point_in_arc_loop_reach"
        | "point_in_arc_loop_conic_span"
        | "point_in_arc_loop_conic_on"
        | "point_in_arc_loop_conic_end"
        | "point_in_arc_loop_conic_trim"
        | "point_in_arc_loop_conic_straddle"
        | "point_in_arc_loop_conic_window"
        | "point_in_arc_loop_conic_disc"
        | "point_in_arc_loop_conic_advance" => BooleanDecision::Containment.subject(),
        _ => return None,
    })
}

/// Which regularized boolean is being computed — threaded through the
/// classifier because on-case lumping (Eq. 15.3) is op-dependent.
///
/// `Hash`/`Ord` are derived because this is also the DOCUMENT layer's
/// operation (re-exported, never re-minted), where a node's fields are
/// keyed and ordered. Ordering is declaration order and carries no
/// meaning — nothing may read it as a ranking of the operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BooleanOp {
    /// A ∪* B.
    Union,
    /// A ∩* B.
    Intersect,
    /// A ∖* B.
    Subtract,
}

impl BooleanOp {
    /// **Every operation this enum names**, in declaration order — the
    /// one enumeration, owned where the exhaustive matches live.
    ///
    /// A list cannot be derived from a match in safe Rust, so SOMEONE
    /// writes it by hand; the only question is where. Written here, it
    /// sits in the crate whose exhaustive matches over `BooleanOp`
    /// (`finish::kept_side`, `tables::eq15_3_lump`) fail to compile on
    /// a fourth operation — so the author adding one is already in this
    /// module with the list in front of them, and the
    /// `all_is_every_operation` census below puts a second visit right
    /// beside it. **Neither forces the edit**: what they force is that
    /// the author is here and has to decide, and the census's own doc
    /// measures how far short of forcing it stops. A copy in a
    /// downstream crate gets not even that. The enum is closed, so a
    /// consumer's own exhaustive match does fence THAT consumer; but
    /// nothing ties an array literal to a variant list, so a downstream
    /// list of three stays three with no error anywhere and no author
    /// standing over it.
    ///
    /// So this is the list downstream reads instead of writing its own
    /// — `crates/editor-core`'s wire table and the viewer's operation
    /// buttons both iterate it — and the ordering caveat on the type
    /// holds for it too: it is declaration order, and a consumer that
    /// renders it renders an arbitrary order, not a ranked one.
    pub const ALL: &'static [BooleanOp] =
        &[BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract];
}

/// Which operand a key belongs to (keys are body-lineage-scoped;
/// cross-body records must say which arena they index — F9).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operand {
    /// The first operand (left of the op).
    A,
    /// The second operand.
    B,
}

/// A trilean side code against the *other* solid's boundary — the
/// boolean analogue of `PlaneSide`, derived from `enters_material`
/// (module docs): `Enters ⇒ In`, `Exits ⇒ Out`, `Tangent ⇒ On`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SideCode {
    /// Definitely inside the other solid's material.
    In,
    /// Tangent/coincident (resolved away by reclassification).
    On,
    /// Definitely outside.
    Out,
}

impl SideCode {
    /// The transition partner (In ↔ Out); `On` has none.
    pub fn opposite(self) -> Self {
        match self {
            Self::In => Self::Out,
            Self::Out => Self::In,
            Self::On => Self::On,
        }
    }
}

/// A coincident vertex pair — one `sonvv` record: declared contact
/// between vertex `a` of body A and vertex `b` of body B.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VvContact {
    /// The A-side vertex (key into the A clone).
    pub a: VertexKey,
    /// The B-side vertex (key into the B clone).
    pub b: VertexKey,
}

/// A vertex-on-face record (`sonva`/`sonvb`): `vertex` (in the operand
/// named by the containing list) lies within `face` of the other body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VfContact {
    /// The piercing vertex.
    pub vertex: VertexKey,
    /// The pierced face of the other body.
    pub face: FaceKey,
}

/// A **certified curve touch** (C3): two faces meeting along the
/// locus carried by `witness`.
///
/// `witness` is the seam EDGE whose carrier IS the contact locus, not
/// a free point: the edge's own description already pins its witness
/// at `carrier(mid)` (the S2 contract), so naming the edge inherits
/// that pin instead of minting a second, unpinned one. Certification
/// is per-locus — the jet schedule of CURVED-DESIGN C7 applied to the
/// face pair along the carrier ([`tangent_pair_relation`]) — and its
/// strength equals its skeleton: samples plus hull bounds, refusing
/// typed outside the certifiable lane rather than sampling harder.
/// Endpoints are bounded by vertex records or by the locus's own
/// closure; a bound without a backing vertex record is
/// `UndeclaredContact`, never inferred.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CurveContact {
    /// The A-side face.
    pub face_a: FaceKey,
    /// The B-side face.
    pub face_b: FaceKey,
    /// The edge whose carrier is the witnessed locus.
    pub witness: crate::entity::EdgeKey,
}

/// A **certified conformal patch** (C3): two faces meeting over a
/// two-dimensional region.
///
/// **Not yet certifiable, stated as a posture rather than discovered
/// as a gap.** The record's certification obligation is structural
/// carrier identity (rung 2 or 3, never "value-equal") plus opposed
/// senses plus region overlap **in the shared chart** with
/// definitely-positive area. The third condition needs a trim-region
/// overlap predicate in (u,v) that does not exist: the planar census's
/// containment machinery run in chart space. Until it does, every
/// door that would certify a `PatchContact` refuses
/// [`crate::contact::ContactRefusal::NotCertifiable`] — the type is
/// here so the vocabulary is complete and the obligation is written
/// down, not so a caller can mint an unbacked blessing. An
/// area-SAMPLED certifier is rejected outright: sampling can miss a
/// trim hole and certify a contact that is not there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PatchContact {
    /// The A-side face.
    pub face_a: FaceKey,
    /// The B-side face.
    pub face_b: FaceKey,
}

/// The ON-sets as **declared-contact records** (F1/F2): emitted
/// by the pipeline as it discovers each contact — these are the future
/// tier-3′ declarations. Deterministic discovery order, deduplicated.
///
/// `PartialEq` is load-bearing, not a convenience: D9's bit-identical
/// replay promises that a rerun reproduces the RECORDS bit-identically
/// (C4's replay clause), and a replay row that can only compare naming
/// is checking a shadow of that promise. Comparing records compares
/// arena keys, which is exactly right here — replay reruns the same
/// pipeline on the same input, so key identity is part of what
/// determinism means.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ContactRecords {
    /// Coincident vertex pairs (`sonvv`).
    pub vv: Vec<VvContact>,
    /// Vertices of A on faces of B (`sonva`).
    pub a_on_b: Vec<VfContact>,
    /// Vertices of B on faces of A (`sonvb`).
    pub b_on_a: Vec<VfContact>,
    /// Curve-granularity contacts (C3).
    pub curves: Vec<CurveContact>,
    /// Patch-granularity contacts (C3) — see [`PatchContact`] for the
    /// not-yet-certifiable posture this list ships under.
    pub patches: Vec<PatchContact>,
}

/// Operand-internal contact records carried by recipe intent (F5, M4
/// PR 5): coincidences WITHIN one operand — typically a reused 3′
/// body's surviving declarations — re-entering this op as declared
/// data. Keys are that operand's; the op remaps survivors into result
/// keys (same strict drop rule as discovered records: a record whose
/// entity was consumed drops).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CarriedContacts {
    /// Coincident vertex pairs within the operand.
    pub vv: Vec<CarriedVv>,
    /// Vertex-on-face rests within the operand.
    pub vf: Vec<CarriedVf>,
}

/// A carried vertex-vertex declaration: the pair AND the class it
/// asserts.
///
/// The class is a FIELD, not an `Option` with a `Rest` default: a
/// declaration without a class is unrepresentable, because defaulting
/// it would let a `Tangent` intent silently re-enter an op as a
/// conformal one — exactly the value-inferred coincidence C4's
/// invariant forbids.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarriedVv {
    /// The coincident pair.
    pub pair: VvContact,
    /// The class the carried declaration asserts.
    pub class: ContactClass,
}

/// A carried vertex-on-face declaration ([`CarriedVv`] for why the
/// class is not defaultable).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarriedVf {
    /// The vertex-on-face rest.
    pub rest: VfContact,
    /// The class the carried declaration asserts.
    pub class: ContactClass,
}

impl CarriedContacts {
    /// True iff nothing is carried.
    pub fn is_empty(&self) -> bool {
        self.vv.is_empty() && self.vf.is_empty()
    }
}

/// Declared coincidence intents threaded into ONE boolean call (F5 —
/// declarations are recipe data on the consuming node; M4 PR 5). The
/// kernel-level form is arena keys; the recipe layer resolves its
/// `Declare` name pairs into these through the operands' name tables.
///
/// Every key is validated at the op door (live, and planar for
/// faces) — a dangling declaration is a typed refusal
/// ([`BooleanError::InvalidDeclaration`]), never a silent drop.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BooleanDeclarations {
    /// Cross-operand declared face pairs, each naming what it asserts:
    /// classification treats a `Rest` or continuation pair's carriers as
    /// the same carrier (orientation decided, contradiction refused —
    /// [`mod@carrier_eq`] rung 2), and the result's merge stage glues the
    /// pair's surviving coplanar-adjacent material (N3 `Merged`).
    pub coincident_faces: Vec<FacePairDeclaration>,
    /// Contacts carried within operand A.
    pub carried_a: CarriedContacts,
    /// Contacts carried within operand B.
    pub carried_b: CarriedContacts,
}

impl BooleanDeclarations {
    /// The no-declarations value (the plain 2-argument ops).
    pub fn none() -> Self {
        Self::default()
    }

    /// True iff nothing is declared.
    pub fn is_empty(&self) -> bool {
        self.coincident_faces.is_empty() && self.carried_a.is_empty() && self.carried_b.is_empty()
    }
}

/// One declared cross-operand face pair AND the coincidence it asserts.
///
/// The class rides the pair rather than a parallel list because the
/// two are one fact: "these faces are in contact, of THIS kind", or
/// "these faces are one surface carried on". A pair whose class had to
/// be looked up elsewhere could be read without it, and reading a
/// declaration without its class is how a `Tangent` intent gets
/// verified against the conformal table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FacePairDeclaration {
    /// The A-operand face.
    pub a: FaceKey,
    /// The B-operand face.
    pub b: FaceKey,
    /// The asserted coincidence.
    pub class: BooleanCoincidence,
}

impl FacePairDeclaration {
    /// A declared pair. There is no class-less constructor: the class
    /// is an argument at every mint site, so "I forgot the class" is a
    /// compile error rather than a silent `Rest`. A [`ContactClass`]
    /// passes as its [`BooleanCoincidence::Contact`].
    pub fn new(a: FaceKey, b: FaceKey, class: impl Into<BooleanCoincidence>) -> Self {
        Self {
            a,
            b,
            class: class.into(),
        }
    }

    /// The `Rest` pair: one carrier, opposed senses.
    pub fn rest(a: FaceKey, b: FaceKey) -> Self {
        Self::new(a, b, ContactClass::Rest)
    }

    /// The continuation pair: one carrier, aligned senses.
    pub fn continuation(a: FaceKey, b: FaceKey) -> Self {
        Self::new(a, b, BooleanCoincidence::Continuation)
    }
}

/// The classification stages' symmetric declared-face-pair index
/// (crate-internal): normalized `(A face, B face)` rows KEYED to their
/// class.
///
/// A map, not a set: the classification stages must be able to ask
/// "declared as WHAT", and a set can only answer "declared at all" —
/// which is the same erasure the payload change exists to remove. A
/// duplicate pair declared under two classes is a caller bug refused
/// at the door by an EXPLICIT check in `validate_declarations`, so the
/// last write here is never reached with disagreeing classes. (That
/// check exists because this sentence was measured vacuous: it held
/// only while every non-`Rest` class refused wholesale, and would have
/// become silent last-write-wins the day the op grew a second class
/// arm.)
#[derive(Debug, Default)]
pub(crate) struct DeclaredPairs {
    map: std::collections::BTreeMap<(FaceKey, FaceKey), BooleanCoincidence>,
    /// What the declaration door VERIFIED, `(A face, B face)`.
    verified: VerifiedDeclarations,
    /// The ORDERED one-sided cover ([`Self::one_sided`]): `(parent,
    /// target)`, where the parent face's carrier is certified to lie in
    /// one closed side of the target's. C4 states the certificate one
    /// way, so the key is directed.
    one_sided: std::collections::BTreeSet<(OperandFace, OperandFace)>,
}

/// A face tagged with the operand it belongs to, ordered A before B;
/// the one-sided cover's key half.
type OperandFace = (bool, FaceKey);

/// `(o, f)` as a cover key half: `true` for A.
fn tagged(o: Operand, f: FaceKey) -> OperandFace {
    (o == Operand::A, f)
}

/// **Which structural tangencies certify a GLOBAL side** (C4's strut
/// source). A `TangentIntersection` edge certifies that its two faces'
/// carriers are tangent ALONG THE EDGE — a local fact. The cover needs a
/// global one: the strut face's (`parent`'s) carrier lies in one closed
/// side of its partner's (`partner`'s) carrier, which the door verified
/// one carrier with the target. Local implies global exactly for these
/// carrier-kind pairs, and each is admitted for its own reason:
///
/// - **cylinder or sphere tangent to a plane**: both are convex
///   surfaces, and a convex surface lies in the closed half-space of
///   any plane tangent to it. The cylinder or sphere is on one closed
///   side of the plane.
/// - **plane tangent to a cylinder or sphere**: every point of a plane
///   tangent to a cylinder is at least the radius from the axis (the
///   plane's distance to the axis IS the radius, tangency being where
///   it is attained). Every point of a plane tangent to a sphere is at
///   least the radius from the centre. So the plane lies in the
///   cylinder's or sphere's closed exterior, one closed side.
///
/// Every other pair gives no cover, and its crossing stays a typed
/// frontier. A cone's tangent plane passes through the apex and leaves
/// the second nappe on its other side. A torus has tangent planes that
/// cut it. A spline (a strut minted by STEP adoption or by a blend)
/// carries no global convexity the strut could stand on.
fn strut_certifies_side(parent: geom_brep::SurfaceKind, partner: geom_brep::SurfaceKind) -> bool {
    use geom_brep::SurfaceKind::{Cylinder, Plane, Sphere};
    matches!(
        (parent, partner),
        (Cylinder | Sphere, Plane) | (Plane, Cylinder | Sphere)
    )
}

/// What [`verify_declared_contacts`] certified, per declared pair
/// `(A face, B face)`: the certificates, not the claims.
#[derive(Debug, Default)]
pub(crate) struct VerifiedDeclarations {
    /// `Rest` and continuation pairs the carrier ladder called ONE
    /// carrier, with the sense the class demands.
    pub(crate) one_carrier: std::collections::BTreeSet<(FaceKey, FaceKey)>,
    /// `Tangent` pairs the witness lane verified.
    pub(crate) tangent: std::collections::BTreeSet<(FaceKey, FaceKey)>,
}

impl DeclaredPairs {
    /// The index over `decls`, with the door's certificates and the
    /// one-sided cover they and the operands' structural tangencies
    /// derive ([`Self::one_sided`]).
    pub(crate) fn build<T: Real>(
        decls: &BooleanDeclarations,
        verified: VerifiedDeclarations,
        a: &Body<T>,
        b: &Body<T>,
    ) -> Self {
        let mut pairs = Self::from_verified(decls, verified);
        // A structural tangency on either operand, between a strut face
        // and a partner the door verified one carrier with a face of
        // the other operand. Each direction is its own certificate,
        // admitted only for the kinds where the strut's local tangency
        // is a global side ([`strut_certifies_side`]): strut face →
        // other face when the strut face lies on one side of the
        // partner, other face → strut face when the partner lies on
        // one side of the strut face.
        let (struts_a, struts_b) = (tangent_struts(a), tangent_struts(b));
        for &(fa, fb) in &pairs.verified.one_carrier {
            for (o, struts, k, target) in [
                (Operand::A, &struts_a, fa, (Operand::B, fb)),
                (Operand::B, &struts_b, fb, (Operand::A, fa)),
            ] {
                for &(f, f_kind, partner, partner_kind) in struts {
                    if partner != k {
                        continue;
                    }
                    let (strut_face, other_face) = (tagged(o, f), tagged(target.0, target.1));
                    // The strut face's carrier on one side of the
                    // partner's, which IS the other face's carrier.
                    if strut_certifies_side(f_kind, partner_kind) {
                        pairs.one_sided.insert((strut_face, other_face));
                    }
                    // The partner's carrier (the other face's) on one
                    // side of the strut face's.
                    if strut_certifies_side(partner_kind, f_kind) {
                        pairs.one_sided.insert((other_face, strut_face));
                    }
                }
            }
        }
        pairs
    }

    /// [`Self::build`] over a one-carrier certificate set alone: no
    /// verified `Tangent` and no operand struts.
    #[cfg(test)]
    pub(crate) fn without_struts(
        decls: &BooleanDeclarations,
        one_carrier: std::collections::BTreeSet<(FaceKey, FaceKey)>,
    ) -> Self {
        Self::from_verified(
            decls,
            VerifiedDeclarations {
                one_carrier,
                tangent: std::collections::BTreeSet::new(),
            },
        )
    }

    /// The index over `decls` with the door's certificates and the
    /// cover they alone derive: [`Self::build`] without the operands'
    /// structural tangencies.
    pub(crate) fn from_verified(
        decls: &BooleanDeclarations,
        verified: VerifiedDeclarations,
    ) -> Self {
        // Both directions for a verified one-carrier pair (one carrier:
        // each lies in, so on one closed side of, the other) and for a
        // verified `Tangent` pair (the witness lane's kinds, plane ×
        // cylinder along a ruling and parallel cylinders, each lie in
        // one closed side of the other — the separation invariant).
        let one_sided: std::collections::BTreeSet<(OperandFace, OperandFace)> = verified
            .one_carrier
            .iter()
            .chain(&verified.tangent)
            .flat_map(|&(fa, fb)| {
                let (x, y) = (tagged(Operand::A, fa), tagged(Operand::B, fb));
                [(x, y), (y, x)]
            })
            .collect();
        Self {
            map: decls
                .coincident_faces
                .iter()
                .map(|d| ((d.a, d.b), d.class))
                .collect(),
            verified,
            one_sided,
        }
    }

    /// `(f1, f2)` as the `(A face, B face)` key, for a cross-operand
    /// pair; `None` for a same-operand one, which is never declared here
    /// (operand-internal coplanarity is the producing op's merge, not
    /// this op's).
    fn key(o1: Operand, f1: FaceKey, o2: Operand, f2: FaceKey) -> Option<(FaceKey, FaceKey)> {
        match (o1, o2) {
            (Operand::A, Operand::B) => Some((f1, f2)),
            (Operand::B, Operand::A) => Some((f2, f1)),
            _ => None,
        }
    }

    /// Whether the (operand-tagged) pair is a `Rest` or continuation
    /// declaration the door VERIFIED as one carrier — the certificate,
    /// not the claim.
    pub(crate) fn verified_one_carrier(
        &self,
        o1: Operand,
        f1: FaceKey,
        o2: Operand,
        f2: FaceKey,
    ) -> bool {
        Self::key(o1, f1, o2, f2).is_some_and(|k| self.verified.one_carrier.contains(&k))
    }

    /// **The crossing layer's one-sided cover** (C4): is the PARENT
    /// face `f1`'s carrier certified to lie in one closed side of the
    /// target `f2`'s? Directed: the answer for `(f1, f2)` says nothing
    /// about `(f2, f1)`.
    ///
    /// The certificate has exactly these sources, and no value reading:
    /// a verified `Rest` or continuation (residual ≡ 0), a verified
    /// `Tangent` (the witness lane's separation invariant), or a
    /// structural tangency — an edge described `TangentIntersection` —
    /// on EITHER operand, from the parent to a face the door verified
    /// one carrier with the target, on a carrier-kind pair where that
    /// tangency is a global side ([`strut_certifies_side`]).
    pub(crate) fn one_sided(&self, o1: Operand, f1: FaceKey, o2: Operand, f2: FaceKey) -> bool {
        o1 != o2 && self.one_sided.contains(&(tagged(o1, f1), tagged(o2, f2)))
    }

    /// The coincidence the (operand-tagged) face pair is declared under,
    /// if any.
    pub(crate) fn class_of(
        &self,
        o1: Operand,
        f1: FaceKey,
        o2: Operand,
        f2: FaceKey,
    ) -> Option<BooleanCoincidence> {
        Self::key(o1, f1, o2, f2).and_then(|k| self.map.get(&k).copied())
    }

    /// Whether the pair is declared ONE carrier (`Rest` or a
    /// continuation) — the question the classification stages actually
    /// ask (a `Tangent` pair does not license same-carrier treatment).
    pub(crate) fn declares_one_carrier(
        &self,
        o1: Operand,
        f1: FaceKey,
        o2: Operand,
        f2: FaceKey,
    ) -> bool {
        self.class_of(o1, f1, o2, f2)
            .is_some_and(BooleanCoincidence::is_one_carrier)
    }

    /// Whether the pair is declared `Tangent`.
    pub(crate) fn declares_tangent(
        &self,
        o1: Operand,
        f1: FaceKey,
        o2: Operand,
        f2: FaceKey,
    ) -> bool {
        self.class_of(o1, f1, o2, f2) == Some(BooleanCoincidence::TANGENT)
    }
}

/// Every structural tangency of `body`, as `(face, its kind, other
/// face, its kind)` both ways: the two faces across an edge described
/// `TangentIntersection` of exactly their two surfaces.
fn tangent_struts<T: Real>(
    body: &Body<T>,
) -> Vec<(
    FaceKey,
    geom_brep::SurfaceKind,
    FaceKey,
    geom_brep::SurfaceKind,
)> {
    let mut out = Vec::new();
    for (_, edge) in body.edges() {
        let (Some(f1), Some(f2)) = (
            body.face_of_half_edge(edge.he_plus),
            body.face_of_half_edge(edge.he_minus),
        ) else {
            continue;
        };
        let (Some(s1), Some(s2)) = (
            body.get_face(f1).map(|f| f.surface),
            body.get_face(f2).map(|f| f.surface),
        ) else {
            continue;
        };
        let Some(curve) = body
            .get_curve_geom(edge.curve)
            .and_then(crate::null::CurveGeom::certified)
        else {
            continue;
        };
        if let geom_brep::EdgeDescription::TangentIntersection { s1: d1, s2: d2, .. } =
            curve.description()
            && f1 != f2
            && Body::<T>::cites_pair((*d1, *d2), s1, s2)
            && let (Some(k1), Some(k2)) = (body.get_surface(s1), body.get_surface(s2))
        {
            let (k1, k2) = (
                geom_brep::SurfaceKind::of(k1),
                geom_brep::SurfaceKind::of(k2),
            );
            out.push((f1, k1, f2, k2));
            out.push((f2, k2, f1, k1));
        }
    }
    out
}

/// The **germ** a null-edge half faces (F9 as data, PR 5): every
/// surviving crossing record — a section-polygon edge emanating from
/// the classified vertex — lies on the intersection line of one A-face
/// and one B-face, and each null edge's two halves are spliced facing
/// its two germs. The joining step matches halves across sites by this
/// identity (same face pair, opposite record parity — the book's
/// he1↔he2 "opposite roles" test carried as data), never by slot
/// position or dynamic face lookups.
#[derive(Clone, Copy, Debug)]
pub struct HalfGerm<T: Real> {
    /// The half-edge facing this germ.
    pub he: crate::entity::HalfEdgeKey,
    /// The A-body face whose plane carries the germ line.
    pub a_face: FaceKey,
    /// The B-body face whose plane carries the germ line.
    pub b_face: FaceKey,
    /// The germ's outgoing direction along the line (unit; points away
    /// from the site toward the polygon edge's other end) — the datum
    /// the joining's mutual-facing test decides on (`bool_join_facing`).
    pub dir: geom_core::Vec3<T>,
}

/// One minted boolean null edge with its F9 side attribute (below ≙ IN
/// copy, above ≙ OUT copy — identity derived from the F3 chain, never
/// slot position) and its two germ facings.
#[derive(Clone, Copy, Debug)]
pub struct BoolNullEdgeRecord<T: Real> {
    /// Which operand's clone the keys index.
    pub operand: Operand,
    /// The classified vertex whose neighborhood minted this edge.
    pub at_vertex: VertexKey,
    /// The null edge.
    pub edge: EdgeKey,
    /// F9 attribute: `below_end` = IN-side copy, `above_end` = OUT-side.
    pub attr: crate::null::NullEdge,
    /// A dangling strut (single-sector double crossing, or a pierced-
    /// face ring null edge).
    pub dangling: bool,
    /// The two germ facings ([`HalfGerm`]), in mint order (the from-
    /// germ first).
    pub germs: [HalfGerm<T>; 2],
}

/// The site a corresponding null-edge pair was minted at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairSite {
    /// A vertex-vertex classification at this declared contact.
    VertexVertex(VvContact),
    /// A vertex of A piercing a face of B.
    VertexAOnFaceB(VfContact),
    /// A vertex of B piercing a face of A.
    VertexBOnFaceA(VfContact),
}

/// **Explicit cross-body correspondence** (F9/F12): the A-side null
/// edge and the B-side null edge minted together for one section-
/// polygon vertex — as data, never correlated array order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NullEdgePairRecord {
    /// The A-clone null edge.
    pub a_edge: EdgeKey,
    /// The B-clone null edge.
    pub b_edge: EdgeKey,
    /// Where the pair was minted.
    pub site: PairSite,
}

/// A pierced-face ring insertion record: the lone ring vertex minted
/// inside the pierced face at the pierce point (the `vtxfacclassify`
/// delta 3 — see `vtxfac` module docs for the designed Euler sequence).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PierceRingRecord {
    /// Which operand's clone holds the pierced face.
    pub operand: Operand,
    /// The pierced face.
    pub face: FaceKey,
    /// The ring vertex at the pierce point.
    pub ring_vertex: VertexKey,
}

/// The result of [`boolean_reduce`]: both operands' annotated clones
/// plus every record the PR 5 joining step consumes.
#[derive(Debug)]
pub struct BooleanReduction<T: Real> {
    /// The op the classification was performed for.
    pub op: BooleanOp,
    /// The annotated A clone (crossings split, null edges inserted).
    pub a: Body<T>,
    /// The annotated B clone.
    pub b: Body<T>,
    /// The declared-contact records (the three ON-sets).
    pub contacts: ContactRecords,
    /// Every null edge minted, both operands, insertion order.
    pub null_edges: Vec<BoolNullEdgeRecord<T>>,
    /// The cross-body correspondence pairs.
    pub null_pairs: Vec<NullEdgePairRecord>,
    /// Pierced-face ring insertions.
    pub pierce_rings: Vec<PierceRingRecord>,
    /// `(A face, B face)`, in clone keys, sorted and deduplicated: each
    /// pair of coincident faces whose Eq. 15.3 lump keeps one copy of
    /// a region they share and drops the other, so the result holds
    /// either face's region through the other (`BooleanNaming::covered`).
    pub covered: Vec<(FaceKey, FaceKey)>,
    /// For each covered pair met at a vertex both operands hold, the
    /// kept copy's edges that run into the dropped face and bound the
    /// held region from outside ([`HeldEdge`]), sorted and
    /// deduplicated. A vertex-on-face contact records none: the face
    /// holds no vertex there for a fragment to be told by.
    pub held: Vec<HeldEdge>,
}

impl<T: Real> BooleanReduction<T> {
    /// Opens a surgery scope on each operand body — the join's
    /// declaration that tier 1 is paid once per operand at its end
    /// rather than once per Euler operator inside it
    /// ([`crate::surgery`]).
    ///
    /// **Guardless, and this is one of the two sites in the crate
    /// where it has to be.** A [`crate::Surgery`] would borrow
    /// `self.a` for the scope's whole span, and the join takes the
    /// WHOLE reduction — both bodies, the contacts, the null-edge
    /// records — so the guard and the call cannot both exist. What
    /// makes the pair safe here is that the reduction is a local of
    /// the pipeline that opened it: a refusal on the way drops it, and
    /// no later call can reach a body a scope was left open on.
    pub(crate) fn enter_join_surgery(&mut self) {
        self.a.enter_surgery();
        self.b.enter_surgery();
    }

    /// Closes both scopes [`Self::enter_join_surgery`] opened, sweeping
    /// each operand **only if the join succeeded**.
    ///
    /// A refusal mid-join leaves a partially carved operand that no
    /// door undertook to certify — and the REST lane puts the pristine
    /// clones back over it — so the sweep is the success path's.
    pub(crate) fn leave_join_surgery(&mut self, joined: bool) {
        if joined {
            self.a.leave_surgery_and_sweep();
            self.b.leave_surgery_and_sweep();
        } else {
            self.a.leave_surgery();
            self.b.leave_surgery();
        }
    }

    /// The minted null edges of one operand's clone, insertion order —
    /// PR 5 (joining) walks each solid's scaffolding separately; this
    /// is the per-operand view of [`Self::null_edges`].
    pub fn null_edges_of(&self, operand: Operand) -> impl Iterator<Item = &BoolNullEdgeRecord<T>> {
        self.null_edges.iter().filter(move |r| r.operand == operand)
    }
}

/// Which site raised [`BooleanError::CurvedPairUnsupported`]. The three
/// share one meaning — this pair of face kinds has no sound lane under
/// this op — and differ in when it is known.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PairRefusalSite {
    /// The operand gate, up front: a kind with no wired arm whose box
    /// may meet an undeclared face of the other operand.
    OperandGate,
    /// The ∖/∩ front door, up front: a kind with no revert seam lane
    /// whose box may meet a face of the other operand.
    RevertRoster,
    /// The crossings path's interior-loop guard, after the reduction:
    /// the section certificate (`section_cert`) could not certify a face
    /// pair free of a closed loop interior to both faces — an
    /// intractable pose, a tangency, a certified interior loop, or an
    /// undecided witness.
    InteriorLoopGuard,
}

/// Typed failure of [`boolean_reduce`]; the operands are never touched.
#[derive(Debug)]
pub enum BooleanError {
    /// The run's tolerance cannot form a valid band (D4 residue).
    Band(BandError),
    /// A face's kind has no wired boolean arm at the classification
    /// site that met it (M5 PR 9: the F5 planar-only gate retired PER
    /// C5 TABLE ARM — `Plane`/`Cylinder`/`Sphere`/`Nurbs` faces pass
    /// the operand gate and pair-level refusals fire where an arm is
    /// actually exercised, citing the table's routing; `Cone`/`Torus`
    /// keep the gate refusal — no wired arm involves them).
    ///
    /// What is missing at the raising site is the boolean's own
    /// crossing layer for the kind — edge×face sweep events, curved trim
    /// containment, and the fitted-chord join lane — even where
    /// `geom_brep::intersect::route` already implements the pair at the
    /// INTERSECTION layer (plane×NURBS). One raising site is the
    /// germ-pair JOIN dispatch's catch-all (`join::bool_connect`), so
    /// a `(Sphere, Sphere)` or `(Cylinder, Sphere)` germ reaches it too,
    /// not only a cone or torus one; the pairs that dispatch does wire
    /// are stated once, at `meeting_recourse`. The pair-general
    /// SECTION-FRAME dispatch beside it is wider, but a frame is not a
    /// join arm: it names the locus's centre and axis for the facing
    /// test and moves no seam lane.
    ///
    /// `kind` is one half of the pair and says nothing about the other,
    /// so the Display never reads as "this kind is unsupported": a
    /// sphere reaches here from a sphere×sphere germ while plane×sphere
    /// is live. The Display is written for the person holding the
    /// mouse: what could not be done and the recourse, never this
    /// routing.
    CurvedBooleanUnsupported {
        /// The offending operand and face.
        operand: Operand,
        /// The face.
        face: FaceKey,
        /// Its surface kind — the C5 table row the refusal cites.
        kind: geom_brep::SurfaceKind,
    },
    /// A pierced face's torus is definitely outside the ring convention
    /// (D3: a horn or spindle torus, or one with no tube), a shape the
    /// kernel has no representation for. The decided arm of the
    /// convention half whose in-band arm is
    /// [`BooleanDecision::Torus`], and it ends in the same lever.
    DegenerateTorus {
        /// The pierced face's operand.
        operand: Operand,
        /// The pierced face.
        face: FaceKey,
        /// The half of the convention that refused.
        convention: TorusConvention,
        /// Its decided verdict.
        verdict: geom_brep::recourse::Refused,
    },
    /// A pierce sector's FIRST-ORDER material verdict could not be
    /// resolved against the pierced face's curvature: the bound leaves
    /// the face within about `2·sqrt(band/R)` radians of tangent (`R`
    /// the face's smallest radius of curvature), or its reach is too
    /// short to witness its slope, so the largest separation from the
    /// face the first-order term certifies past the sagitta does not
    /// clear the band (`boolean::sectors::side_code` carries the
    /// argument and the witness). The **definite** half of a
    /// two-tolerance pair on `bool_pierce_sector_side_curved`; an
    /// in-band charge escalates as [`BooleanError::Escalated`] on the
    /// same predicate instead.
    ///
    /// A refusal, never a guess: the slope's sign is one the band
    /// cannot resolve, and a side read from it would be a guess at the
    /// topology. The side of a near-tangent bound is second order, so
    /// the kernel's own way through is the second-order sector trilean
    /// (`geom_brep::enters_material_order2`), which the declared-
    /// `Tangent` lump already consumes and which no lane wires into
    /// this verdict yet; the user's is the decision's lever
    /// ([`refusal_routes::PIERCE_CURVATURE`], shared with the in-band
    /// arm, [`BooleanDecision::PierceCurvature`]).
    CurvedSectorSideUnsupported {
        /// The charge's decided verdict at the sector's arm.
        verdict: geom_brep::recourse::Refused,
    },
    /// A sweep event definitely lands on a CURVED face away from its
    /// boundary, a vertex sits ON a curved surface, or a curved-carrier
    /// edge cannot be cleared against a curved face, and the curved
    /// PIERCE door cannot take it. **That door now exists for one
    /// family** — a LINE carrier definitely crossing a CYLINDER wall,
    /// whose crossing parameters come from the certified line × wall
    /// quadratic and whose landing point the chart trim places — so
    /// what this variant reports is the REST of the family: a tangency
    /// (not a crossing at any order the lane sees), a CIRCLE carrier
    /// against a wall (a degree-2 trigonometric residual with no root
    /// lane in this tree), a SPHERE face, an undeclared on-carrier
    /// edge, or a trim the chart door declines to express (the M5
    /// envelope's frontier; the C5 table routes the SECTIONS, this is
    /// the crossing layer). The
    /// **definite** half of a two-tolerance pair: the very same
    /// clearance margin one band-width away escalates as
    /// [`BooleanError::Escalated`] on `bool_line_cylinder_clearance`
    /// instead ([`Coincide::EdgeOnCurvedFace`]), and both halves end on
    /// the coincidence's levers: the declared-cover rung takes a
    /// declaration of the edge's face against the curved one, and this
    /// definite half names no tolerance.
    CurvedPierceUnsupported {
        /// The operand whose edge met the curved face.
        operand: Operand,
        /// The curved face (in the other operand).
        face: FaceKey,
        /// The edge.
        edge: EdgeKey,
        /// The band the clearance margins were classified against.
        band: Band,
    },
    /// The operand gate (F5) refused a rung-3 (`Nurbs`) carrier in an
    /// INPUT operand: rung-3 edges are what the curved zip MINTS, not
    /// what it consumes.
    CurvedEdgeUnsupported {
        /// The offending operand and edge.
        operand: Operand,
        /// The edge.
        edge: EdgeKey,
    },
    /// The both-edges-split point lane needs a carrier with an exact
    /// point parameter and got one without. `Line` and `Circle` have
    /// one; `Ellipse` reaches here, because the operand gate admits it,
    /// so this is a **narrower** condition than the gate's, at a later
    /// stage — which is exactly why it is its own variant rather than a
    /// second reach of [`Self::CurvedEdgeUnsupported`]. One doc and one
    /// message covering both could name neither.
    PointSplitCarrierUnsupported {
        /// The offending operand and edge.
        operand: Operand,
        /// The edge.
        edge: EdgeKey,
    },
    /// **Point-in-face on a loop no walk expresses at the point.** The
    /// in-plane walk reads each edge on its own carrier — a line as its
    /// chord, a circle or ellipse arc on its conic — and has no crossing
    /// row for a spiric or spline edge. It answers along any ray that
    /// definitely misses a ball holding such an edge; a point where
    /// every ray could meet one — a crossing of that edge could change
    /// the answer — is refused here rather than guessed.
    ArcLoopContainmentUnsupported {
        /// The operand whose face carries the loop.
        operand: Operand,
        /// The loop no walk expresses at the point.
        r#loop: crate::entity::LoopKey,
    },
    /// An operand already carries null scaffolding (mid-surgery body).
    ScaffoldingOperand {
        /// The offending operand and edge.
        operand: Operand,
        /// The edge.
        edge: EdgeKey,
    },
    /// F7: two adjacent faces of one operand are structurally or
    /// declaredly coplanar — the operand is not maximal-faced; run
    /// `merge_coplanar_faces` explicitly first.
    NonMaximalFaces {
        /// The offending operand.
        operand: Operand,
        /// The shared edge whose two faces coincide.
        edge: EdgeKey,
    },
    /// F7: two neighbouring faces of one operand lie on one plane with
    /// no shared source, at a decided zero offset or one in band. The
    /// maximal-faces gate compares faces of ONE operand, which no
    /// face-pair declaration names, so the refusal ends in the gate's
    /// own lever ([`refusal_routes::NEIGHBOUR_OFFSET`]) and offers no
    /// declaration.
    CoplanarNeighbours {
        /// The operand both faces belong to.
        operand: Operand,
        /// The two faces.
        faces: [FaceKey; 2],
        /// The offset the plane ladder's offset rung refused.
        offset: NeighbourOffset,
    },
    /// A vertex sector's bounding chord has **no finite length**: its
    /// components overflow the norm (past ~1e154), or one of them is
    /// not a number. Distinct from [`BooleanError::Escalated`] on
    /// purpose — nothing about this is a band question, and no
    /// tolerance lever reaches it.
    NonFiniteSectorChord {
        /// The vertex being classified.
        vertex: VertexKey,
        /// The sector's face.
        face: FaceKey,
    },
    /// A vertex sector's bounding chord has a length that **underflowed
    /// out of the format**: its components are too small for the norm
    /// to hold (below ~1e-162 at `f64`), so it measures exactly zero
    /// while still naming a direction. Distinct from
    /// [`BooleanError::Escalated`] on purpose — nothing about this is
    /// a band question, and no tolerance lever reaches it.
    UnderflowedSectorChord {
        /// The vertex being classified.
        vertex: VertexKey,
        /// The sector's face.
        face: FaceKey,
    },
    /// A reduction/classification predicate escalated (in-band margin):
    /// the operand pair is ill-conditioned at this ε — a genuine
    /// sliver (F6). Never a snap, never a guess.
    Escalated {
        /// The decision that escalated, which the refusal's ending
        /// follows from.
        decision: BooleanDecision,
        /// The predicate's escalation diagnostics.
        diag: Indeterminate,
    },
    /// Two entities are geometrically coincident-or-near without a
    /// shared recipe source or declared intent backing the coincidence
    /// (F6/N6): near-coincidence NEVER silently becomes contact.
    ///
    /// Since LIB-PYG5 (register R3, SELECT-DESIGN §3d) the refusal
    /// keeps what the raise site held: the face PAIR whose coincidence
    /// lacked intent, each face tagged with its operand, plus the
    /// orientation the ladder decided before refusing — so a document
    /// layer can name the candidate declaration in the refusal itself
    /// instead of re-running any decide on the error path. Two faces
    /// of ONE operand are [`BooleanError::CoplanarNeighbours`] instead.
    UndeclaredCoincidence {
        /// The escalation site's diagnostics.
        diag: Indeterminate,
        /// The coincident face pair, a face of each operand, each with
        /// the operand it lives in.
        pair: [(Operand, FaceKey); 2],
        /// The decided orientation ([`PlaneRelation::SameOriented`]
        /// or [`PlaneRelation::SameOpposite`], never `Distinct`) —
        /// the relation a declaration of this pair would assert.
        relation: PlaneRelation,
    },
    /// A declared coincidence contradicts the geometry (the declared
    /// pair's carriers are definitely distinct) — the recipe's intent
    /// cannot be realized; refused loudly, never glued (M4 PR 5).
    DeclarationContradicted {
        /// The fact that contradicted the declaration.
        fact: Contradiction,
    },
    /// A declared CONTACT meets definite counter-evidence at the op
    /// (C4's verify-at-use): the pair, the class it claimed, and the
    /// margin that decided.
    ///
    /// Beside [`Self::DeclarationContradicted`] rather than replacing
    /// it: that variant is the classification ladder's refusal of a
    /// coincidence claim, this one is the CONTACT tables' refusal of a
    /// contact claim, and they carry different evidence. The same
    /// finding fires at the at-rest gate as
    /// `ValidationError::ContactContradicted` — one story, two gates.
    ContactContradicted {
        /// The face pair and class that were declared.
        declaration: crate::contact::DeclaredContact,
        /// The fact that contradicted it, where the carrier ladder found
        /// the two carriers distinct (a `Rest` declaration); `None` where
        /// the counter-evidence is the tangent lane's, which the
        /// margin's predicate labels.
        fact: Option<Contradiction>,
        /// The margin that decided, and its predicate.
        margin: Indeterminate,
        /// Extra recourse steering when the counter-evidence has a
        /// named remedy (AQ6's designed-clearance arm).
        steer: Option<&'static str>,
    },
    /// A declared CONTINUATION meets definite counter-evidence at the
    /// op (C4's continuation clause): the two faces are not one carrier,
    /// or their senses are opposed — which makes them a `Rest` pair, not
    /// a continuation.
    ///
    /// Beside [`Self::ContactContradicted`] rather than inside it: a
    /// continuation is not a contact, and that variant's payload is a
    /// contact claim.
    ContinuationContradicted {
        /// The A-operand face.
        a: FaceKey,
        /// The B-operand face.
        b: FaceKey,
        /// The fact that contradicted it, where the carrier ladder found
        /// the two carriers distinct; `None` where the senses did, which
        /// the margin's predicate labels.
        fact: Option<Contradiction>,
        /// The margin that decided, and its predicate.
        margin: Indeterminate,
    },
    /// A declaration names a contact class in a configuration this
    /// op's classification cannot act on: a `Tangent` pair outside
    /// the DEV-1 closed-form witness lane (plane×cylinder along a
    /// ruling, parallel cylinders) — no witness locus derives, so no
    /// verification can run and no arm can consume the claim. Refused
    /// at the door rather than carried into stages that would ignore
    /// it — the vocabulary is wider than this op's envelope, and the
    /// gap is typed, not silent.
    UnsupportedDeclarationClass {
        /// The class that was declared.
        class: ContactClass,
    },
    /// **A declared shared rim the routing classified as the wedge-π
    /// SEAM** (`docs/MATE-7-TANGENCY-DESIGN.md`, RATIFIED).
    ///
    /// The π arm IS built — this refusal is not a gap in the design and
    /// not a gap in the classification, which ran and answered. Two
    /// things are true at once and the message says both: a wedge-π rim
    /// is a seam of one composite wall and takes NO declaration, so a
    /// `Tangent` claim on it is the wrong instrument; and the join
    /// wiring that would consume this verdict while zipping is not
    /// built, so the op cannot proceed on it either.
    ///
    /// Kept distinct from [`BooleanError::RimCuspArmUnbuilt`] because
    /// the two are different facts. One variant covering both would
    /// have to call the π arm unbuilt, which is false.
    RimSeamNotDeclarable {
        /// The declaration whose face pair carries the rim.
        declaration: crate::contact::DeclaredContact,
    },
    /// **A declared shared rim routed to the cusp family, which is
    /// DEFINED but not BUILT** (the same ruling).
    ///
    /// Wedge 0 or 2π: the material pinches to a knife edge or opens to
    /// a circular slit. This is the declared-cusp family, and it is the
    /// arm the ruling deliberately left unbuilt. Two pieces are missing
    /// for every pair that reaches it: a tangent-locus arm for the
    /// pair's surface kinds (it is raised only where
    /// [`geom_brep::tangent_locus`] answers `Unsupported`), and the
    /// consumer that builds the cusp or slit edge from a locus, which is
    /// unbuilt for every locus shape. The A11-rider shape: the design is
    /// settled and the refusal points at the ruling that settles it.
    RimCuspArmUnbuilt {
        /// The declaration whose face pair carries the rim.
        declaration: crate::contact::DeclaredContact,
        /// Which end of the wedge range the rim's material subtends —
        /// the routing's own verdict, taken on the geometry.
        wedge: geom_brep::MaterialWedge,
    },
    /// A [`BooleanDeclarations`] payload references an entity that
    /// does not resolve in its operand (stale/foreign key, or a
    /// non-planar declared face) — a caller bug, refused before any
    /// classification runs (M4 PR 5).
    InvalidDeclaration {
        /// The operand whose key failed.
        operand: Operand,
        /// What was wrong.
        what: &'static str,
    },
    /// The 15.11 consecutive-pairing invariant failed: a surviving
    /// crossing-record pair is not cyclically adjacent in both
    /// neighborhoods (F12's guarded refusal — see `insert`).
    PairingMismatch {
        /// The A-side vertex of the neighborhood.
        a_vertex: VertexKey,
        /// The B-side vertex.
        b_vertex: VertexKey,
    },
    /// A classification invariant failed (e.g. a surviving record
    /// without one IN and one OUT code per side) — a kernel bug
    /// surfaced loudly, not silently mis-joined.
    ClassificationInvariant {
        /// Human-oriented description of the violated invariant.
        what: &'static str,
    },
    /// A traversal failed: an operand is not a well-formed closed
    /// solid at this site.
    CorruptOperand {
        /// The operand.
        operand: Operand,
        /// The vertex whose neighborhood could not be walked.
        vertex: VertexKey,
    },
    /// `split_edge` refused while inserting a crossing (site attached,
    /// inner error whole).
    CrossingInsertion {
        /// The operand whose edge was being split.
        operand: Operand,
        /// The edge.
        edge: EdgeKey,
        /// The underlying Euler refusal.
        source: EulerOpError,
    },
    /// **A germ PAIR with no boolean seam lane**, refused at the
    /// operand gate.
    ///
    /// The gate asks THREE questions, and this refusal is what a pair
    /// that answered wrong on all three earns: does this face's KIND
    /// have a wired arm; can this face REACH the other operand at all;
    /// and, failing both, do the caller's DECLARATIONS speak for this
    /// particular pair. Reach is decided at box-level conservatism
    /// (`reduce::first_unsupported_pair`), which is why the payload
    /// names a pair rather than a body: a cone or a torus whose box
    /// clears every face of the other operand cannot enter a crossing,
    /// a section or a germ pair, so the operation does not depend on
    /// its kind and the gate says nothing about it.
    ///
    /// The third question is the newest and the narrowest. A
    /// declaration is the author supplying the verdict a germ arm would
    /// have supplied, verified at the front door before this gate runs;
    /// a pair it covers is one the pipeline has an answer for. What may
    /// be declared is bounded by the certified carrier inventory, so a
    /// kind with no rung there can never be covered here — which is why
    /// this refusal still names cones and NURBS unconditionally.
    ///
    /// **The overlap that DID fire is a may, not a does.** Boxes are
    /// supersets, so the two faces named here may in exact geometry
    /// be disjoint; the refusal claims only that the kernel cannot
    /// rule the meeting out, and that if they do meet it has no arm
    /// for the pair.
    ///
    /// The `op` field carries the op when the kind is admitted for
    /// OTHERS and refused for this one — `Nurbs` under ∖ and ∩ — and
    /// is `None` when no op has an arm for the kind.
    ///
    /// **What refuses, per class, and why it refuses HERE.** The
    /// blocker left is a JOIN lane, not `revert`:
    ///
    /// - **Sphere**: not gated here — the `(Plane, Sphere)` germ arm
    ///   (exact C5 Circle) plus the extent-certified fallback re-cut.
    /// - **Torus**: not gated up front — its pairs meet the same typed
    ///   doors under ∖ and ∩ as under ∪: the crossing layer's, the
    ///   no-crossings section pass, the join catch-all below, and this
    ///   variant at [`PairRefusalSite::InteriorLoopGuard`] for a torus
    ///   pair the section certificate cannot clear.
    /// - **Cone**: the germ-pair JOIN dispatch —
    ///   `join::bool_connect`'s match on the two germ faces'
    ///   surfaces — wires only the pairs `meeting_recourse` names, and
    ///   no cone or torus pair. Its catch-all raises
    ///   [`BooleanError::CurvedBooleanUnsupported`], not this error,
    ///   and a torus, `(Sphere, Sphere)` or `(Cylinder, Sphere)` germ
    ///   lands there too.
    ///
    ///   **A wider dispatch sits beside it and must not be confused
    ///   with it.** `join::pair_section_frame` — the pair-general
    ///   SECTION-FRAME dispatch — additionally names
    ///   `(Sphere, Sphere)`, `(Cylinder, Cylinder)` (as a proven
    ///   STRAIGHT locus, or its own pinch door) and, behind a DECLARED
    ///   coaxiality no production caller can supply,
    ///   `(Cylinder, Sphere)`. But a frame is not a join arm: that
    ///   dispatch names the locus's centre and axis for the rotational
    ///   facing test, which is a strictly smaller thing than a seam
    ///   lane. Naming a frame for a pair does not move this refusal.
    ///
    ///   The cyl×sphere fitted-chord
    ///   window's blocker MOVED at M6-2: `Pcurve::Fitted` now exists
    ///   and certifies at rest (the SSI enclosure/certify stack is no
    ///   longer `f64`-only), so what is left is the JOIN LANE itself —
    ///   `run_azimuth_window`/`chart_pcurve` have no cyl×sphere window
    ///   analog, and building one is still banked. **The exact coaxial
    ///   classification does not retire this**, and the sentence is
    ///   re-verified rather than moved: the coaxial arm's locus is two
    ///   exact CIRCLES and needs no fitted chord at all, so it gives
    ///   the window nothing to read. What would retire the sentence is
    ///   the window itself, for the TRANSVERSAL poses that march (the
    ///   DECLARED-coaxial classification, `geom_brep::cylinder_sphere_section`,
    ///   and its germ-frame arm exist and do not change this).
    /// - **NURBS**: no edge×NURBS-face crossing layer at all
    ///   (deviation 5), and the fallback's extent test is unwritable
    ///   for the kind ([`BooleanError::NurbsExtentUnsupported`]).
    ///
    /// **Three sites raise it, and [`PairRefusalSite`] says which.** The
    /// operand gate and the ∖/∩ revert roster refuse UP FRONT, on the
    /// operands' kinds and boxes. The crossings path's interior-loop
    /// guard (`ops::interior_loop_verdict`) refuses AFTER the pipeline
    /// would have answered: a face pair, one of them not a plane, that
    /// the section certificate cannot clear of a loop no edge event
    /// marks (the reduction saw crossings elsewhere, and the join and
    /// face-region propagation cannot see the loop).
    ///
    /// The up-front refusals exist because their downstream failure is
    /// **silent, not typed**: with no crossings found the pipeline
    /// falls through to vertex-probed containment, and a curved face
    /// can leave the other solid between its vertices without any
    /// vertex noticing. The executed witness — a ball half-buried in a
    /// slab, metered as if wholly contained — is pinned as
    /// `finding_sphere_class_containment_fallback_is_wrong_today` in
    /// `crates/sweep/tests/m5_s12_curved_ops.rs`, with its merge-base
    /// reproduction recorded. That defect predates S12 and stands on
    /// **union** too; S12 deliberately does not change ∪'s behaviour
    /// (a revert-wiring unit is the wrong place to re-cut the
    /// containment fallback), so ∪ is not gated here and the row is
    /// what keeps it visible.
    CurvedPairUnsupported {
        /// The op this refusal is specific to, or `None` when the kind
        /// has no arm under any op. `Union` is named only by the
        /// interior-loop guard ([`PairRefusalSite::InteriorLoopGuard`]).
        op: Option<BooleanOp>,
        /// Which site refused: the operand gate, the revert roster, or
        /// the crossings path's interior-loop guard.
        site: PairRefusalSite,
        /// The operand carrying the face whose kind has no arm.
        operand: Operand,
        /// That face — the first such in face-arena order.
        face: FaceKey,
        /// Its kind: the half of the germ pair with no arm.
        kind: geom_brep::SurfaceKind,
        /// The other operand's face whose box it may meet.
        other_face: FaceKey,
        /// That face's kind: the other half of the germ pair.
        other_kind: geom_brep::SurfaceKind,
    },
    /// The containment fallback's curved-EXTENT scan (M5 S13) met a
    /// NURBS face. The extent test is UNWRITABLE for the kind with
    /// what exists — `implicit_residual` is poison on a NURBS surface
    /// and the only foot-point projection
    /// (`NurbsSurface::project`) had no lane off `f64`. **Half of that
    /// is now false**: M6-2 lifted the projection to any
    /// bracket-carrying scalar (`impl<T: Bounds> NurbsSurface<T>`), so
    /// the Interval-lane objection is retired. What still blocks the
    /// extent test is the test ITSELF: `implicit_residual` is poison on
    /// a NURBS surface, so a certified extent needs a written
    /// projection-based extent argument — a foot point plus a bound on
    /// how far the patch can reach past it — which nothing has
    /// derived. The class stays RE-GATED at the fallback, explicitly
    /// and pinned: a future NURBS body constructor inherits this typed
    /// refusal, never the vertex-probe silence the S12 finding
    /// executed.
    NurbsExtentUnsupported {
        /// The operand carrying the NURBS face.
        operand: Operand,
        /// The face.
        face: FaceKey,
    },
    /// The containment fallback's curved-extent scan (M5 S13) met a
    /// configuration it cannot certify: the no-crossings question
    /// ("which operand contains the other?") would otherwise be
    /// answered by vertex probes a curved boundary defeats (the S12
    /// finding), so anything the certified extents cannot clear is
    /// refused typed here — never guessed.
    FallbackExtentUnsupported {
        /// The operand whose sphere group (or face) the scan stopped at.
        operand: Operand,
        /// The face the refusal cites.
        face: FaceKey,
        /// The precise uncertifiable sub-configuration.
        what: &'static str,
    },
    /// **Two spheres of the two solids meet** — neither clearly apart
    /// nor one strictly inside the other — at the curved-extent scan,
    /// which runs only where the crossing layer found no edge crossing a
    /// face. Whatever the two spheres share lies off every edge, so the
    /// join's sphere-pair arm (the radical plane, `join::bool_connect`)
    /// had no chord to run, and the scan, which reads the surfaces,
    /// cannot certify either shell's side of the other's boundary. It is
    /// the decided refusal of [`SphereQuestion::Nested`], and ends as
    /// that question's escalation does ([`refusal_routes::SPHERES`]).
    SpheresMeet {
        /// The operand whose sphere face the scan stopped at.
        operand: Operand,
        /// The sphere face.
        face: FaceKey,
        /// The nesting clearance's decided verdict: zero (band-decided,
        /// a smaller tolerance may decide it positive) or negative (the
        /// boundaries cross).
        verdict: geom_brep::recourse::Refused,
    },
    /// **A germ pair whose section frame has no arm.** The join's
    /// matcher asks each germ pair for the LOCUS its germ line rides,
    /// and the answer drives which facing test runs: a straight locus
    /// takes the chord test, a conic locus the rotational-sense test.
    /// "No frame" therefore MEANS "the locus is straight", and a pair
    /// EARNS that answer only by proof: a plane×plane section is a line
    /// by construction, a plane×cylinder one is a line where the C5
    /// table says so, and a cylinder pair's is rulings exactly when its
    /// axes are parallel. Every other pair either has a section arm
    /// that names its conic, or has no arm at all; the second case is
    /// refused here rather than defaulting into the straight-chord
    /// test, which would mint a wrong chord silently the moment the
    /// germ-pair dispatch widens.
    GermFrameUnsupported {
        /// The A-side germ face.
        a_face: FaceKey,
        /// Its kind — the A half of the germ pair.
        a_kind: geom_brep::SurfaceKind,
        /// The B-side germ face.
        b_face: FaceKey,
        /// Its kind — the B half of the germ pair.
        b_kind: geom_brep::SurfaceKind,
    },
    /// **A germ pair of two cylinder walls whose axes definitely
    /// INTERSECT** — the frame dispatch's named sub-case of "no
    /// frame", and the door the intersecting equal-radius family
    /// (Steinmetz and every re-posed copy of it) stops at.
    ///
    /// Two shapes live behind these axes, and the dispatch is allowed
    /// to name neither:
    ///
    /// - **Equal radii.** The section is the two bisector-plane
    ///   ellipses. They are not two disjoint loci: the bisector
    ///   planes' common line runs through the axes' meeting point `p`
    ///   along `â₁ × â₂`, is perpendicular to both axes, and therefore
    ///   meets BOTH walls at `p ± r·n̂`, where `n̂ = unit(â₁ × â₂)`.
    ///   The UNIT is load-bearing off 90°: `‖â₁ × â₂‖ = sin θ`, so the
    ///   raw cross product understates the offset by that factor and
    ///   only coincides with the pinch points on the perpendicular
    ///   pose. So the two ellipses
    ///   CROSS at those two points, for every member of the family and
    ///   at every pose — four arcs meeting at two valence-4 PINCH
    ///   vertices. A pinch is not a conic frame, and a frame dispatch
    ///   keyed on surface kinds alone has no point with which to
    ///   select which branch a germ rides.
    /// - **Unequal radii.** The locus is a space quartic — canal
    ///   territory, the general rung — and has no conic frame at all.
    ///
    /// Which of the two holds is a RADIUS-equality question, and
    /// radius equality is structural or declared and never inferred
    /// from values (`geom_brep::RadiusEvidence`, the coincidence
    /// ladder). The germ site reads the answer off the lowered
    /// parameter-identity channel (`crate::param_source`) and this
    /// refusal carries it as `evidence`: `Declared` names the
    /// equal-radius pinch as a PROVEN configuration — the closed form
    /// was constructed and verified against the geometry on the way —
    /// while `None` leaves the question open and refuses on the axis
    /// relation alone. Neither reads the radii. Recourse: a chord lane
    /// that can walk a self-intersecting section, or geometry whose
    /// germ pairs are wired.
    GermFrameCylinderPinch {
        /// The A-side germ face.
        a_face: FaceKey,
        /// The B-side germ face.
        b_face: FaceKey,
        /// The radius-equality evidence the germ site read off the
        /// lowered parameter-identity channel
        /// (`crate::param_source`) — which of the two shapes named in
        /// the message the locus actually has.
        evidence: geom_brep::RadiusEvidence,
    },
    /// An underlying Euler operation refused.
    Euler(EulerOpError),
    /// The result body's pcurve mint pass refused (M5 PR 9: curved
    /// results carry certified per-half-edge pcurves at rest — the
    /// PR 6 contract; loud rather than shipping uncertified caches,
    /// D4 ¶2).
    Pcurves {
        /// The typed pcurve-pass refusal, nested whole.
        source: crate::pcurves::PcurveMintError,
    },
    /// The joining stage's chord machinery refused (PR 5; nested
    /// whole — includes `UnpairedLooseEnds` and `SectionLoopMixed`).
    Join(SplitJoinError),
    /// The declared-REST union zip (M5 S1) recognized its frontier —
    /// a declared boundary-on-boundary REST contact — but the
    /// configuration is a named sub-frontier the lane does not cover
    /// (no speculative region algebra is built for it); refused
    /// typed, never a laundered catch-all (the `SkippedMerge`
    /// precedent). The pair is declared and verified already, so no
    /// declaration is offered; each sub-frontier ends as
    /// [`RestZipFrontier::ending`] gives it.
    RestZipUnsupported {
        /// The precise sub-frontier.
        what: RestZipFrontier,
    },
    /// The A/B lockstep invariant failed during joining, finishing, or
    /// the combine door (a kernel bug or corrupt reduction, loudly).
    JoinDesync {
        /// Which lockstep invariant broke.
        what: &'static str,
    },
    /// One distributed component carries section faces of both sides
    /// (kernel bug, loudly).
    TornComponent {
        /// The operand whose clone tore.
        operand: Operand,
        /// The offending shell.
        shell: ShellKey,
    },
    /// No witness of a shell the other operand's boundary does not cut
    /// — its vertices, its edges' midpoints, an interior point of each
    /// planar face — decides which side of that boundary the shell lies
    /// on: each lies ON it or too near it to say (`shell_witness`'s
    /// module docs). Two operands that are one body reach this.
    ShellWitnessExhausted {
        /// The operand whose shell was probed.
        operand: Operand,
        /// The shell, in that operand's working copy.
        shell: ShellKey,
        /// Witnesses that read `OnBoundary`.
        on_boundary: usize,
        /// Witnesses whose reading was in-band.
        in_band: usize,
        /// The first in-band reading: evidence about one witness, not
        /// the cause, which is that none decided.
        first_in_band: Option<PointInSolidError>,
    },
    /// The containment fallback / uncut-component probe refused (F8).
    Containment(PointInSolidError),
    /// `revert` refused on the ∖ B side.
    Revert(RevertError),
    /// The two seam cycles of a polygon pair are not antiparallel —
    /// the orientation chain broke (kernel bug, loudly).
    SeamOrientation {
        /// The A section face.
        a_face: FaceKey,
        /// The B section face (result key).
        b_face: FaceKey,
    },
    /// The seam zip's record-keyed correspondence could not be
    /// resolved (kernel bug or corrupt records, loudly).
    ZipCorrespondence {
        /// What failed to resolve.
        what: &'static str,
    },
    /// The F7 output stage (`merge_coplanar_faces`) refused.
    Merge(MergeCoplanarError),
    /// The finished result failed a tier gate (kernel bug, loudly —
    /// no invalid body is ever returned).
    ResultInvalid {
        /// The validator's findings.
        errors: Vec<ValidationError>,
    },
    /// A `Seamed` result's volume violates a set-theoretic bound —
    /// vol(∩) ≤ min(vol A, vol B), vol(∪) ≥ max(vol A, vol B),
    /// vol(∖) ≤ vol A — checked at the op gate against the bodies'
    /// certified mass properties (the review's volume-inequality backstop,
    /// decided on the INVARIANT LANE — outside the length seam,
    /// Ev's #213 layering ruling). A certified violation is a
    /// **kernel invariant** failure — the Corrupt class: a bug in the
    /// kernel, never in the caller's geometry — surfaced as this typed
    /// error, never a panic and never a validity refusal.
    ResultVolumeImplausible {
        /// Which inequality failed (e.g. "vol(A ∖ B) ≤ vol(A)").
        which: &'static str,
        /// The result volume, Debug-formatted (the scalar is generic).
        got: String,
        /// The violated operand-volume bound, Debug-formatted.
        bound: String,
    },
    /// The volume backstop could not measure one of the three bodies
    /// its bounds compare — the operands and the result — so it cannot
    /// say whether the result is the right one, and no body is
    /// returned. `source` is a valid face the property layer has no
    /// measurement for: an inventory gap, a quadrature that could not
    /// certify its own convergence, a tolerance that forms no band.
    VolumeUnmeasured {
        /// The operand that would not measure; `None` is the result.
        operand: Option<Operand>,
        /// The property layer's refusal, whole.
        source: crate::props::MassPropsError,
    },
    /// The volume backstop found a body whose structure does not
    /// resolve where its volume is measured — a key that names
    /// nothing, a placeholder edge. Tier 3 reads the same refusals the
    /// same way (one reading, `validate::classify_mass_props`). On the
    /// result that is a **kernel defect**, the Corrupt class
    /// [`BooleanError::ResultVolumeImplausible`] is in; on an operand
    /// it is the kernel's or the file's the operand came from.
    VolumeCorrupt {
        /// The operand whose structure does not resolve; `None` is the
        /// result.
        operand: Option<Operand>,
        /// The property layer's refusal, whole.
        source: crate::props::MassPropsError,
    },
    /// The volume backstop measured all three bodies, refined their
    /// enclosures as far as the quadrature's schedule reaches, and
    /// still could not decide whether the bound named by `which` holds:
    /// what the enclosures leave open, metered as a boundary
    /// displacement, is certified larger than the model's resolution.
    /// The result may be right; nothing here can say, so no body is
    /// returned.
    VolumeUndecided {
        /// The bound left open (e.g. "vol(A ∖ B) ≤ vol(A)").
        which: &'static str,
    },
    /// The result would be unbounded (only reachable with complement
    /// operands, e.g. ∪ of a body with its own complement) — no
    /// boundary representation exists for it.
    UnrepresentableResult,
    /// Re-certifying a grafted edge description against the combined
    /// body's surfaces refused (the combine door's remap lane —
    /// bitwise-identical inputs make this unreachable for well-formed
    /// grafts; loud, never a dangling reference).
    GraftRecertify(geom_brep::CertifyError),
}

/// Which arm of [`BooleanError`] refused — the discriminant alone,
/// with no payload.
///
/// [`BooleanError`] derives `Debug` and nothing else: its arms carry
/// arena keys, nested kernel refusals and margin diagnostics whose
/// scalars are floats, so the error is neither `Clone` nor `PartialEq`
/// and a consumer that wants the CLASS of a refusal back out of it has
/// only the prose to substring-match. This projection drops exactly
/// the part that cannot be cloned or compared, so the class rides
/// where the error itself cannot: into a `Clone + PartialEq` finding
/// record, a hash key, a test assertion, an FFI tag map.
///
/// One variant per [`BooleanError`] arm, and [`BooleanError::kind`]
/// matches exhaustively — an arm added to the error reds `kind` itself,
/// here in this crate. It reds nothing downstream, because no consumer
/// maps this enum yet: today it appears only as a field type on
/// `editor_core::CheckEvidence` and in this module's tests.
///
/// A variant HERE with no arm behind it is a phantom: nothing
/// constructs it, so no test can reach it. Since there is no
/// downstream map to red, this module's tests carry the visit that
/// does — an exhaustive match over this enum, which names the phantom
/// at compile time. The fix at that red is to delete the phantom,
/// never to give it a tag: a tag minted for a phantom publishes a name
/// no refusal can ever carry.
///
/// Deliberately NOT `Ord`. The declaration order mirrors
/// [`BooleanError`]'s for reading, and nothing depends on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum BooleanErrorKind {
    /// [`BooleanError::Band`].
    Band,
    /// [`BooleanError::CurvedBooleanUnsupported`].
    CurvedBooleanUnsupported,
    /// [`BooleanError::DegenerateTorus`].
    DegenerateTorus,
    /// [`BooleanError::CurvedSectorSideUnsupported`].
    CurvedSectorSideUnsupported,
    /// [`BooleanError::CurvedPierceUnsupported`].
    CurvedPierceUnsupported,
    /// [`BooleanError::CurvedEdgeUnsupported`].
    CurvedEdgeUnsupported,
    /// [`BooleanError::PointSplitCarrierUnsupported`].
    PointSplitCarrierUnsupported,
    /// [`BooleanError::ArcLoopContainmentUnsupported`].
    ArcLoopContainmentUnsupported,
    /// [`BooleanError::ScaffoldingOperand`].
    ScaffoldingOperand,
    /// [`BooleanError::NonMaximalFaces`].
    NonMaximalFaces,
    /// [`BooleanError::CoplanarNeighbours`].
    CoplanarNeighbours,
    /// [`BooleanError::NonFiniteSectorChord`].
    NonFiniteSectorChord,
    /// [`BooleanError::UnderflowedSectorChord`].
    UnderflowedSectorChord,
    /// [`BooleanError::Escalated`].
    Escalated,
    /// [`BooleanError::UndeclaredCoincidence`].
    UndeclaredCoincidence,
    /// [`BooleanError::DeclarationContradicted`].
    DeclarationContradicted,
    /// [`BooleanError::ContactContradicted`].
    ContactContradicted,
    /// [`BooleanError::ContinuationContradicted`].
    ContinuationContradicted,
    /// [`BooleanError::UnsupportedDeclarationClass`].
    UnsupportedDeclarationClass,
    /// [`BooleanError::RimSeamNotDeclarable`].
    RimSeamNotDeclarable,
    /// [`BooleanError::RimCuspArmUnbuilt`].
    RimCuspArmUnbuilt,
    /// [`BooleanError::InvalidDeclaration`].
    InvalidDeclaration,
    /// [`BooleanError::PairingMismatch`].
    PairingMismatch,
    /// [`BooleanError::ClassificationInvariant`].
    ClassificationInvariant,
    /// [`BooleanError::CorruptOperand`].
    CorruptOperand,
    /// [`BooleanError::CrossingInsertion`].
    CrossingInsertion,
    /// [`BooleanError::CurvedPairUnsupported`].
    CurvedPairUnsupported,
    /// [`BooleanError::NurbsExtentUnsupported`].
    NurbsExtentUnsupported,
    /// [`BooleanError::FallbackExtentUnsupported`].
    FallbackExtentUnsupported,
    /// [`BooleanError::SpheresMeet`].
    SpheresMeet,
    /// [`BooleanError::GermFrameUnsupported`].
    GermFrameUnsupported,
    /// [`BooleanError::GermFrameCylinderPinch`].
    GermFrameCylinderPinch,
    /// [`BooleanError::Euler`].
    Euler,
    /// [`BooleanError::Pcurves`].
    Pcurves,
    /// [`BooleanError::Join`].
    Join,
    /// [`BooleanError::RestZipUnsupported`].
    RestZipUnsupported,
    /// [`BooleanError::JoinDesync`].
    JoinDesync,
    /// [`BooleanError::TornComponent`].
    TornComponent,
    /// [`BooleanError::ShellWitnessExhausted`].
    ShellWitnessExhausted,
    /// [`BooleanError::Containment`].
    Containment,
    /// [`BooleanError::Revert`].
    Revert,
    /// [`BooleanError::SeamOrientation`].
    SeamOrientation,
    /// [`BooleanError::ZipCorrespondence`].
    ZipCorrespondence,
    /// [`BooleanError::Merge`].
    Merge,
    /// [`BooleanError::ResultInvalid`].
    ResultInvalid,
    /// [`BooleanError::ResultVolumeImplausible`].
    ResultVolumeImplausible,
    /// [`BooleanError::VolumeUnmeasured`].
    VolumeUnmeasured,
    /// [`BooleanError::VolumeCorrupt`].
    VolumeCorrupt,
    /// [`BooleanError::VolumeUndecided`].
    VolumeUndecided,
    /// [`BooleanError::UnrepresentableResult`].
    UnrepresentableResult,
    /// [`BooleanError::GraftRecertify`].
    GraftRecertify,
}

/// Which of the volume backstop's three bodies a refusal is about.
fn backstop_subject(operand: Option<Operand>) -> &'static str {
    match operand {
        Some(Operand::A) => "the first solid",
        Some(Operand::B) => "the second solid",
        None => "the result",
    }
}

impl BooleanError {
    /// An escalation of the coincidence `which` between parts of the two
    /// solids, at a site whose door read the pair's declaration as
    /// `read` ahead of it ([`BooleanDecision::Coincidence`]): the refusal
    /// offers a declaration exactly where `read` says one would settle
    /// the question. A site whose question is not a coincidence between
    /// the two solids names its own [`BooleanDecision`] instead.
    pub(crate) const fn coincidence(
        which: Coincide,
        read: DeclarationRead,
        diag: Indeterminate,
    ) -> Self {
        Self::Escalated {
            decision: BooleanDecision::Coincidence(which, read),
            diag,
        }
    }

    /// An escalation of a reading metered over a lever arm at `gate`,
    /// routed by the rung that raised it
    /// ([`BooleanDecision::of_lever`]): the arm is the gate's own
    /// length, and the reading is the decision the gate meters
    /// ([`refusal_routes::LeverArm::reading`]), with what its door read
    /// of the declaration.
    pub(crate) const fn of_lever(
        gate: refusal_routes::LeverArm,
        read: DeclarationRead,
        escalation: geom_brep::LeverEscalation,
    ) -> Self {
        Self::Escalated {
            decision: BooleanDecision::of_lever(gate, read, escalation.rung),
            diag: escalation.diag,
        }
    }

    /// An escalation of a plane-identity `rung` at `door`, routed to the
    /// decision the rung asks there ([`BooleanDecision::of_plane_rung`]).
    pub(crate) const fn plane_identity(
        rung: PlaneRung,
        door: refusal_routes::PlaneDoor,
        diag: Indeterminate,
    ) -> Self {
        Self::Escalated {
            decision: BooleanDecision::of_plane_rung(rung, door),
            diag,
        }
    }

    /// The refusal of a pierced face's outward normal at a pierce point
    /// (`face_normal::face_outward_normal_at`), in the Boolean's words.
    pub(crate) fn of_pierced_normal(
        refusal: crate::face_normal::NormalAtError,
        operand: Operand,
        face: FaceKey,
    ) -> Self {
        use crate::face_normal::NormalAtError;
        match refusal {
            NormalAtError::Escalated { decision, diag } => Self::Escalated {
                decision: BooleanDecision::of_normal(decision),
                diag,
            },
            NormalAtError::DegenerateTorus {
                convention,
                verdict,
            } => Self::DegenerateTorus {
                operand,
                face,
                convention,
                verdict,
            },
            NormalAtError::OffSurface => Self::ClassificationInvariant {
                what: "pierce point definitely off the pierced face's surface",
            },
        }
    }

    /// Which arm refused, without the payload.
    ///
    /// Exhaustive over [`BooleanError`]: adding an arm there is a
    /// compile error here and in every consumer that maps this enum.
    #[must_use]
    pub fn kind(&self) -> BooleanErrorKind {
        match self {
            Self::Band(_) => BooleanErrorKind::Band,
            Self::CurvedBooleanUnsupported { .. } => BooleanErrorKind::CurvedBooleanUnsupported,
            Self::DegenerateTorus { .. } => BooleanErrorKind::DegenerateTorus,
            Self::CurvedSectorSideUnsupported { .. } => {
                BooleanErrorKind::CurvedSectorSideUnsupported
            }
            Self::CurvedPierceUnsupported { .. } => BooleanErrorKind::CurvedPierceUnsupported,
            Self::CurvedEdgeUnsupported { .. } => BooleanErrorKind::CurvedEdgeUnsupported,
            Self::PointSplitCarrierUnsupported { .. } => {
                BooleanErrorKind::PointSplitCarrierUnsupported
            }
            Self::ArcLoopContainmentUnsupported { .. } => {
                BooleanErrorKind::ArcLoopContainmentUnsupported
            }
            Self::ScaffoldingOperand { .. } => BooleanErrorKind::ScaffoldingOperand,
            Self::NonMaximalFaces { .. } => BooleanErrorKind::NonMaximalFaces,
            Self::CoplanarNeighbours { .. } => BooleanErrorKind::CoplanarNeighbours,
            Self::NonFiniteSectorChord { .. } => BooleanErrorKind::NonFiniteSectorChord,
            Self::UnderflowedSectorChord { .. } => BooleanErrorKind::UnderflowedSectorChord,
            Self::Escalated { .. } => BooleanErrorKind::Escalated,
            Self::UndeclaredCoincidence { .. } => BooleanErrorKind::UndeclaredCoincidence,
            Self::DeclarationContradicted { .. } => BooleanErrorKind::DeclarationContradicted,
            Self::ContactContradicted { .. } => BooleanErrorKind::ContactContradicted,
            Self::ContinuationContradicted { .. } => BooleanErrorKind::ContinuationContradicted,
            Self::UnsupportedDeclarationClass { .. } => {
                BooleanErrorKind::UnsupportedDeclarationClass
            }
            Self::RimSeamNotDeclarable { .. } => BooleanErrorKind::RimSeamNotDeclarable,
            Self::RimCuspArmUnbuilt { .. } => BooleanErrorKind::RimCuspArmUnbuilt,
            Self::InvalidDeclaration { .. } => BooleanErrorKind::InvalidDeclaration,
            Self::PairingMismatch { .. } => BooleanErrorKind::PairingMismatch,
            Self::ClassificationInvariant { .. } => BooleanErrorKind::ClassificationInvariant,
            Self::CorruptOperand { .. } => BooleanErrorKind::CorruptOperand,
            Self::CrossingInsertion { .. } => BooleanErrorKind::CrossingInsertion,
            Self::CurvedPairUnsupported { .. } => BooleanErrorKind::CurvedPairUnsupported,
            Self::NurbsExtentUnsupported { .. } => BooleanErrorKind::NurbsExtentUnsupported,
            Self::FallbackExtentUnsupported { .. } => BooleanErrorKind::FallbackExtentUnsupported,
            Self::SpheresMeet { .. } => BooleanErrorKind::SpheresMeet,
            Self::GermFrameUnsupported { .. } => BooleanErrorKind::GermFrameUnsupported,
            Self::GermFrameCylinderPinch { .. } => BooleanErrorKind::GermFrameCylinderPinch,
            Self::Euler(_) => BooleanErrorKind::Euler,
            Self::Pcurves { .. } => BooleanErrorKind::Pcurves,
            Self::Join(_) => BooleanErrorKind::Join,
            Self::RestZipUnsupported { .. } => BooleanErrorKind::RestZipUnsupported,
            Self::JoinDesync { .. } => BooleanErrorKind::JoinDesync,
            Self::TornComponent { .. } => BooleanErrorKind::TornComponent,
            Self::ShellWitnessExhausted { .. } => BooleanErrorKind::ShellWitnessExhausted,
            Self::Containment(_) => BooleanErrorKind::Containment,
            Self::Revert(_) => BooleanErrorKind::Revert,
            Self::SeamOrientation { .. } => BooleanErrorKind::SeamOrientation,
            Self::ZipCorrespondence { .. } => BooleanErrorKind::ZipCorrespondence,
            Self::Merge(_) => BooleanErrorKind::Merge,
            Self::ResultInvalid { .. } => BooleanErrorKind::ResultInvalid,
            Self::ResultVolumeImplausible { .. } => BooleanErrorKind::ResultVolumeImplausible,
            Self::VolumeUnmeasured { .. } => BooleanErrorKind::VolumeUnmeasured,
            Self::VolumeCorrupt { .. } => BooleanErrorKind::VolumeCorrupt,
            Self::VolumeUndecided { .. } => BooleanErrorKind::VolumeUndecided,
            Self::UnrepresentableResult => BooleanErrorKind::UnrepresentableResult,
            Self::GraftRecertify(_) => BooleanErrorKind::GraftRecertify,
        }
    }
}

impl From<BandError> for BooleanError {
    fn from(e: BandError) -> Self {
        Self::Band(e)
    }
}

impl From<EulerOpError> for BooleanError {
    fn from(e: EulerOpError) -> Self {
        Self::Euler(e)
    }
}

/// How a refusal names an operand to the person who built it: by its
/// place in the operation, never by the enum spelling.
fn operand_word(operand: Operand) -> &'static str {
    match operand {
        Operand::A => "first",
        Operand::B => "second",
    }
}

/// The operation, as the noun a refusal sentence uses.
fn op_noun(op: BooleanOp) -> &'static str {
    match op {
        BooleanOp::Union => "union",
        BooleanOp::Intersect => "intersection",
        BooleanOp::Subtract => "subtraction",
    }
}

/// A surface kind as the person holding the mouse reads it. The
/// spline kinds get one spelling everywhere a Boolean refusal names
/// them; every other kind is its own name.
fn kind_word(kind: geom_brep::SurfaceKind) -> &'static str {
    match kind {
        geom_brep::SurfaceKind::Nurbs => "spline (NURBS)",
        geom_brep::SurfaceKind::Approx => "approximated spline",
        other => other.name(),
    }
}

/// The recourse every "these faces cannot meet yet" refusal ends on.
///
/// **This is the one statement of the pairs the Boolean can join**: the
/// germ-pair JOIN dispatch (`join::bool_connect`) wires a plane face
/// against a plane, cylinder or sphere face, mirrors included, and
/// nothing else. The rustdoc that needs the set points here. The
/// operand gate's box test is conservative (a box overlap is a MAY),
/// so moving the face clear of the other solid is a real recourse even
/// where the faces never touch in exact geometry.
fn meeting_recourse(kind: &str) -> String {
    format!(
        "Recourse: reshape the parts so they meet only where a plane face meets \
         a plane, cylinder or sphere face, or move them so the {kind} face stays \
         clear of the other solid"
    )
}

impl core::fmt::Display for BooleanError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            // The band's own refusal names itself and carries its recourse.
            Self::Band(e) => write!(f, "{e}"),
            // No operand is named: the raise sites disagree on whether
            // `operand` is the face's own operand or the operand of the
            // edge that met it.
            Self::CurvedBooleanUnsupported { kind, .. } => write!(
                f,
                "the Boolean cannot yet work out where one solid's {} face meets the \
                 face of the other solid it runs into: that pairing of faces is not \
                 supported yet. {}",
                kind_word(*kind),
                meeting_recourse(kind_word(*kind)),
            ),
            Self::DegenerateTorus {
                convention,
                verdict,
                ..
            } => write!(
                f,
                "{}. {}",
                convention.refused("a torus face's", *verdict),
                convention
                    .sized()
                    .recourse(verdict.arm(), geom_brep::recourse::Reading::Build)
            ),
            Self::CurvedPierceUnsupported { operand, .. } => write!(
                f,
                "an edge of the {} operand touches or crosses a curved face of the \
                 other operand away from that face's edges, and the Boolean cannot yet \
                 settle where or whether it passes through. Recourse: \
                 {}",
                operand_word(*operand),
                geom_core::DEFINITE_COINCIDENCE_RECOURSE,
            ),
            Self::CurvedSectorSideUnsupported { verdict } => write!(
                f,
                "where an edge pierces a curved face, the Boolean cannot be sure which \
                 side of the face the material is on: {}. {}",
                match verdict {
                    geom_brep::recourse::Refused::Zero(_) => {
                        "a direction leaving the pierce point runs too close to tangent to \
                         the face, or along too short an edge, for this tolerance to tell"
                    }
                    geom_brep::recourse::Refused::Negative { .. } => {
                        "the face bends away over the edge's length by more than the edge \
                         departs from it"
                    }
                },
                refusal_routes::PIERCE_CURVATURE
                    .recourse(verdict.arm(), geom_brep::recourse::Reading::Build)
            ),
            Self::CurvedEdgeUnsupported { operand, .. } => write!(
                f,
                "an edge of the {} operand is a spline (NURBS) curve, and the Boolean \
                 cannot yet take a solid with spline edges as an input. Recourse: \
                 rebuild that solid so its edges are lines, circles or ellipses",
                operand_word(*operand),
            ),
            Self::PointSplitCarrierUnsupported { operand, .. } => write!(
                f,
                "an ellipse edge of the {} operand has to be split where the solids \
                 meet, and the Boolean can split only lines and circles there. \
                 Recourse: move the parts so that edge does not meet the other solid",
                operand_word(*operand),
            ),
            // No operand is named, for the same reason as above: some
            // raise sites carry the operand of the edge being placed, not
            // of the face whose loop has no walk.
            Self::ArcLoopContainmentUnsupported { .. } => write!(
                f,
                "the Boolean cannot yet tell what lies inside a flat face whose outline \
                 has a spiric or spline edge near the point it asked about, so it \
                 refuses rather than guess. Recourse: model the outline with lines, \
                 circles or ellipses"
            ),
            Self::ScaffoldingOperand { operand, .. } => write!(
                f,
                "the {} operand is a body left in the middle of an edit (it still \
                 carries unfinished edges), so the Boolean refuses it. This is a bug in \
                 whatever produced that body; please report it",
                operand_word(*operand),
            ),
            Self::NonMaximalFaces { operand, .. } => write!(
                f,
                "the {} operand has two neighbouring faces that lie on one surface, so \
                 the Boolean refuses it. Recourse: merge those faces first \
                 (merge_coplanar_faces)",
                operand_word(*operand),
            ),
            Self::CurvedPairUnsupported {
                site: PairRefusalSite::InteriorLoopGuard,
                op,
                operand,
                kind,
                other_kind,
                ..
            } => write!(
                f,
                "the {} operand's {} face may meet the {} operand's {} face along a closed \
                 curve that crosses no edge of either solid, and the Boolean{} cannot yet \
                 see such a meeting, so it refuses rather than guess. Recourse: move the \
                 parts so every place they meet crosses an edge of one of them, or so the \
                 {} face stays clear of the other solid",
                operand_word(*operand),
                kind_word(*kind),
                operand_word(operand.other()),
                kind_word(*other_kind),
                op.map_or(String::new(), |op| format!(" {}", op_noun(op))),
                kind_word(*kind),
            ),
            Self::CurvedPairUnsupported {
                op,
                operand,
                kind,
                other_kind,
                ..
            } => write!(
                f,
                "the {} operand's {} face may meet the {} operand's {} face, and the \
                 Boolean{} cannot yet work out where such a face meets another solid. \
                 {}",
                operand_word(*operand),
                kind_word(*kind),
                operand_word(operand.other()),
                kind_word(*other_kind),
                op.map_or(String::new(), |op| format!(" {}", op_noun(op))),
                meeting_recourse(kind_word(*kind)),
            ),
            Self::NurbsExtentUnsupported { operand, .. } => write!(
                f,
                "the Boolean found no crossing between the two solids and cannot yet \
                 check whether the {} operand's spline (NURBS) face lies inside or \
                 outside the other, so it refuses rather than guess. Recourse: make \
                 the solids' boundaries cross, or build that face from planes, \
                 cylinders or spheres",
                operand_word(*operand),
            ),
            // No operand is named: the raise sites pass the operand being
            // scanned, which is not always the one whose face the refusal cites.
            Self::FallbackExtentUnsupported { what, .. } => write!(
                f,
                "the solids' boundaries do not cross, and the Boolean cannot be sure \
                 whether one lies inside the other ({what}). Recourse: {}",
                refusal_routes::EXTENT_LEVER,
            ),
            // No operand is named, as the scan's other refusals name none.
            Self::SpheresMeet { verdict, .. } => write!(
                f,
                "the two solids' spheres {}, and where they meet lies off every edge of both \
                 solids, so the Boolean finds no crossing to join them along. {}",
                match verdict {
                    geom_brep::recourse::Refused::Zero(_) => {
                        "touch within the tolerance, the smaller inside the larger"
                    }
                    geom_brep::recourse::Refused::Negative { .. } => "cross",
                },
                refusal_routes::SPHERES
                    .recourse(verdict.arm(), geom_brep::recourse::Reading::Build)
            ),
            Self::GermFrameUnsupported { a_kind, b_kind, .. } => write!(
                f,
                "the Boolean cannot yet trace where the first operand's {} face meets \
                 the second operand's {} face, so it refuses rather than guess. {}",
                kind_word(*a_kind),
                kind_word(*b_kind),
                meeting_recourse(kind_word(*a_kind)),
            ),
            // True for BOTH radius cases: the raise site refuses on the
            // axis relation alone when no radius evidence exists, so the
            // walls may have equal radii (the section crosses itself) or
            // not (a space quartic).
            Self::GermFrameCylinderPinch { .. } => write!(
                f,
                "two cylinder walls whose axes cross meet here, and the Boolean cannot \
                 yet trace where they meet, whether their radii are equal (the curve \
                 crosses itself) or not (a curve it has no form for). Recourse: \
                 reshape the parts so these two walls do not meet each other",
            ),
            Self::Pcurves { source } => write!(
                f,
                "the result's pcurve mint pass refused (curved results carry \
                 certified per-half-edge pcurves at rest): {source}"
            ),
            Self::NonFiniteSectorChord { .. } => write!(
                f,
                "a direction the Boolean measures where the solids meet has no finite \
                 length \u{2014} its components overflow, or one of them is not a \
                 number. Recourse: {}",
                geom_core::RANGE_RECOURSE
            ),
            Self::UnderflowedSectorChord { .. } => write!(
                f,
                "a direction the Boolean measures where the solids meet is too small \
                 for its length to be represented, so it measures exactly zero; no \
                 tolerance reaches this. Recourse: {}",
                geom_core::RANGE_RECOURSE
            ),
            Self::Escalated { decision, diag } => f.write_str(&decision.render(diag)),
            Self::CoplanarNeighbours {
                operand, offset, ..
            } => write!(
                f,
                "two neighbouring faces of the {} operand lie on one plane, or nearly ({}). {}",
                operand_word(*operand),
                offset.reported().payload(),
                offset.ending()
            ),
            Self::UndeclaredCoincidence { diag, .. } => {
                f.write_str(
                    "a face of the first operand and a face of the second coincide, or nearly (",
                )?;
                // The rung-4 definite arm synthesizes `MarginKind::Invalid`
                // for a decided-zero offset (plane_eq keeps the decision
                // machinery); rendering that payload verbatim would claim a
                // poisoned margin on clean geometry. Say the honest thing
                // instead: the measure is definitely zero (S6 review,
                // MAJOR-1).
                if diag.margin.is_invalid() {
                    f.write_str(
                        "the coincidence measure is exactly zero — the geometry coincides",
                    )?;
                } else {
                    write!(f, "{}", diag.payload())?;
                }
                write!(
                    f,
                    "), and the Boolean never assumes that touching faces are the same \
                     face. Recourse: {COINCIDENCE_RECOURSE}"
                )
            }
            // The fact, where the carrier ladder found one; otherwise
            // the one reason true at every site. Never the margin
            // payload. The faces are the first and second operands'
            // (the declaration's own order).
            Self::ContactContradicted {
                declaration,
                steer,
                fact,
                ..
            } => write!(
                f,
                "the declared {} contact between the operands' faces is contradicted: {}. \
                 {}{}",
                declaration.class.name(),
                fact.map_or(crate::contact::CONTRADICTION_REASON, Contradiction::fact),
                crate::contact::CONTRADICTION_RECOURSE,
                crate::contact::steer_clause(*steer),
            ),
            Self::ContinuationContradicted { fact, .. } => write!(
                f,
                "the declared continuation between the operands' faces is contradicted: {}. {}",
                fact.map_or(
                    "the two faces are one surface facing opposite ways, which is a Rest \
                     contact rather than a continuation",
                    Contradiction::fact
                ),
                crate::contact::CONTRADICTION_RECOURSE,
            ),
            Self::DeclarationContradicted { fact } => write!(
                f,
                "a declared coincidence contradicts the geometry: {}, and the Boolean never \
                 glues a lie. {}",
                fact.fact(),
                crate::contact::CONTRADICTION_RECOURSE
            ),
            Self::UnsupportedDeclarationClass { class } => write!(
                f,
                "a declared contact of class {} lies outside the envelope this \
                 op's classification acts on (Rest on the plane/sphere/cylinder carrier \
                 inventory; Tangent where the closed-form witness lane reaches — \
                 plane×cylinder along a ruling, parallel cylinders) — the declaration is \
                 refused at the door rather than ignored inside",
                class.name()
            ),
            Self::RimSeamNotDeclarable { declaration } => write!(
                f,
                "the declared faces continue smoothly into each other across their \
                 shared rim, so they are one wall and the {} declaration on them is \
                 wrong; and the Boolean cannot yet join two solids across such a rim \
                 with or without it. There is no way through this in the kernel yet",
                declaration.class.name()
            ),
            Self::RimCuspArmUnbuilt { declaration, wedge } => write!(
                f,
                "the declared faces meet along a rim circle where the material {}, and \
                 the Boolean cannot yet verify a {} declaration there: it has no tangent \
                 locus for these two surfaces, and cannot yet build the edge where they \
                 touch. The declaration is the right one; there is no way through this \
                 in the kernel yet",
                match wedge {
                    geom_brep::MaterialWedge::Cusp => "pinches to a knife edge",
                    geom_brep::MaterialWedge::Slit => "opens to a thin slit",
                    geom_brep::MaterialWedge::Seam | geom_brep::MaterialWedge::Transverse => {
                        wedge.name()
                    }
                },
                declaration.class.name()
            ),
            Self::InvalidDeclaration { operand, what } => write!(
                f,
                "a declaration on the {} operand is invalid: {what}",
                operand_word(*operand)
            ),
            Self::PairingMismatch { a_vertex, b_vertex } => write!(
                f,
                "null-edge pairing mismatch at vertex pair \
                 ({a_vertex:?}, {b_vertex:?}): a surviving crossing-record pair is not \
                 cyclically adjacent in both neighborhoods (the 15.11 invariant's guarded \
                 refusal)"
            ),
            Self::ClassificationInvariant { what } => {
                write!(f, "classification invariant violated: {what}")
            }
            Self::CorruptOperand { operand, vertex } => write!(
                f,
                "the neighbourhood of vertex {vertex:?} in the {} operand could not be \
                 walked (a broken body)",
                operand_word(*operand)
            ),
            Self::CrossingInsertion {
                operand, source, ..
            } => write!(
                f,
                "the Boolean could not insert a crossing on an edge of the {} operand: \
                 {source}",
                operand_word(*operand)
            ),
            Self::Euler(e) => write!(f, "euler operation refused: {e}"),
            Self::Join(e) => write!(
                f,
                "the operands' sections could not be joined: {}",
                crate::chord_join::UnderBoolean(e)
            ),
            Self::RestZipUnsupported { what } => write!(
                f,
                "the Boolean cannot yet zip the two solids along their declared resting \
                 contact ({}); it zips planar contacts whose seam splits cleanly. {}",
                what.what(),
                what.ending()
            ),
            Self::JoinDesync { what } => write!(
                f,
                "A/B lockstep invariant violated: {what} (kernel bug or corrupt \
                 reduction)"
            ),
            Self::TornComponent { operand, shell } => write!(
                f,
                "component {shell:?} of the {} operand carries section faces \
                 of both sides (kernel bug)",
                operand_word(*operand)
            ),
            Self::ShellWitnessExhausted {
                operand,
                on_boundary,
                in_band,
                ..
            } => {
                write!(
                    f,
                    "the solids do not cross, and none of the {} points tried on the {} \
                     solid (corners, edge middles, flat-face interiors) tells whether it \
                     is inside the other: {on_boundary} lie on the other's boundary",
                    on_boundary + in_band,
                    operand_word(*operand)
                )?;
                if *in_band == 0 {
                    write!(f, ". Recourse: if the two are one body, use it once")
                } else {
                    write!(
                        f,
                        ", {in_band} too near it to tell. Recourse: if the two are one \
                         body, use it once; if they nearly touch, move them clearly \
                         together or apart"
                    )
                }
            }
            // The payload does not say which operand was being tested, so
            // the sentence says "one of the solids" rather than guess.
            Self::Containment(e) => write!(f, "the solids do not cross, and the Boolean {e}"),
            Self::Revert(e) => write!(f, "revert of the ∖ B side refused: {e}"),
            Self::SeamOrientation { a_face, b_face } => write!(
                f,
                "seam cycles of faces {a_face:?}/{b_face:?} are not antiparallel \
                 (orientation chain broke — kernel bug)"
            ),
            Self::ZipCorrespondence { what } => {
                write!(f, "seam zip correspondence failed: {what} (kernel bug)")
            }
            Self::Merge(e) => write!(f, "coplanar-merge output stage refused: {e}"),
            Self::ResultInvalid { errors } => write!(
                f,
                "finished result failed a tier gate ({} finding(s), first: {:?}) — \
                 kernel bug, no invalid body is returned",
                errors.len(),
                errors.first()
            ),
            Self::ResultVolumeImplausible { which, got, bound } => write!(
                f,
                "the Boolean's result broke a bound a correct result's volume always meets \
                 ({which}: got {got}, bound {bound}), so no body is returned. \
                 {KERNEL_DEFECT_ENDING}"
            ),
            Self::VolumeUnmeasured { operand, source } => {
                let reading = crate::validate::classify_mass_props(source);
                write!(
                    f,
                    "the Boolean checks its result against its inputs' volumes, and the \
                     volume of {} cannot be measured — {} — so no body is returned. {}",
                    backstop_subject(*operand),
                    reading.why,
                    reading.recourse
                )
            }
            Self::VolumeCorrupt { operand, source } => {
                let ending = match operand {
                    Some(_) => geom_core::KERNEL_OR_FILE_DEFECT_ENDING,
                    None => KERNEL_DEFECT_ENDING,
                };
                write!(
                    f,
                    "the Boolean checks its result against its inputs' volumes, and the \
                     volume of {} cannot be measured — {} — so no body is returned. {ending}",
                    backstop_subject(*operand),
                    crate::validate::classify_mass_props(source).why
                )
            }
            Self::VolumeUndecided { which } => write!(
                f,
                "the Boolean checks its result against its inputs' volumes, and measuring \
                 them as finely as the kernel can still leaves open whether {which} holds, \
                 by more than the tolerance, so no body is returned. The finest measurement \
                 grows coarser with the size of the bodies. Recourse: build at a looser \
                 tolerance, which the open range may fit inside — though on bodies large \
                 enough no tolerance does"
            ),
            Self::UnrepresentableResult => write!(
                f,
                "the result would be unbounded (complement operands) — no boundary \
                 representation exists"
            ),
            Self::GraftRecertify(e) => {
                write!(
                    f,
                    "grafted edge description failed re-certification: {}",
                    e.render(geom_brep::recourse::Reading::Build)
                )
            }
        }
    }
}

impl std::error::Error for BooleanError {}

/// **`boolean_reduce`** — reduction + classification + paired
/// null-edge insertion across two bodies (module docs for the
/// pipeline). Functional: both operands are cloned and never touched;
/// the annotated clones come back in [`BooleanReduction`]. Joining and
/// result generation are PR 5.
///
/// Determinism (D9): gates, sweeps, contact processing, and
/// per-neighborhood classification all run in arena/discovery order —
/// no hash iteration anywhere.
///
/// # Errors
///
/// [`BooleanError`] — see each variant; the first failure wins and the
/// operands are never mutated (the clones are dropped).
pub fn boolean_reduce<T: Decide + Bounds>(
    op: BooleanOp,
    a_operand: &Body<T>,
    b_operand: &Body<T>,
    tol: Tol,
) -> Result<BooleanReduction<T>, BooleanError> {
    boolean_reduce_declared(op, a_operand, b_operand, &BooleanDeclarations::none(), tol)
}

/// [`boolean_reduce`] with declared coincidence intents (F5, M4
/// PR 5): the declared face pairs enter the classification stages'
/// plane-identity evidence; carried contacts are validated here and
/// consumed by the result stage (`ops`).
///
/// # Errors
///
/// [`BooleanError`] — including [`BooleanError::InvalidDeclaration`]
/// for payloads that do not resolve against the operands.
pub fn boolean_reduce_declared<T: Decide + Bounds>(
    op: BooleanOp,
    a_operand: &Body<T>,
    b_operand: &Body<T>,
    decls: &BooleanDeclarations,
    tol: Tol,
) -> Result<BooleanReduction<T>, BooleanError> {
    boolean_reduce_declared_strategy(
        op,
        a_operand,
        b_operand,
        decls,
        SweepStrategy::Realized,
        tol,
    )
}

/// The differential suite's sweep-level door (PERF-PLAN §4.4 / C10,
/// pins i and iii): clones the operands, runs the gates and BOTH
/// reduction sweep directions under `strategy`, and returns the
/// per-direction traces `(A→B, B→A)` — `examined` (the candidate set)
/// and `accepted` (pairs where the exact predicates accepted an
/// event). The suite pins `Realized.examined ⊇ Idealized.accepted`
/// per direction.
///
/// `plant` is pin (iii)'s failure injection: it empties ONE face box
/// of `b_operand` in the A→B direction (candidate generation loses
/// that face's events), proving the superset pin can fail. Production
/// code never passes it.
///
/// # Errors
///
/// [`BooleanError`] — the same gates and sweep refusals as
/// [`boolean_reduce`].
#[cfg(feature = "sweep-testing")]
pub fn sweep_traces<T: Decide + Bounds>(
    a_operand: &Body<T>,
    b_operand: &Body<T>,
    strategy: SweepStrategy,
    plant: Option<PlantedDegradation>,
    tol: Tol,
) -> Result<(SweepTrace, SweepTrace), BooleanError> {
    sweep_traces_with_pad(a_operand, b_operand, strategy, plant, None, tol)
}

/// [`sweep_traces`] with a PAD OVERRIDE (fix-pass pin 1b): the suite
/// proves a too-small pad (e.g. `Some(0.0)`) LOSES accepted pairs and
/// the superset comparator catches it. A deliberately breakable knob —
/// `sweep-testing` only, never production surface.
///
/// # Errors
///
/// [`BooleanError`] as [`sweep_traces`].
#[cfg(feature = "sweep-testing")]
pub fn sweep_traces_with_pad<T: Decide + Bounds>(
    a_operand: &Body<T>,
    b_operand: &Body<T>,
    strategy: SweepStrategy,
    plant: Option<PlantedDegradation>,
    pad_override: Option<f64>,
    tol: Tol,
) -> Result<(SweepTrace, SweepTrace), BooleanError> {
    let band = Band::linear(tol)?;
    // The suite's door takes no declarations: the traced sweep runs
    // the undeclared posture, where the frontier doors are verbatim —
    // and so, therefore, is the operand gate's covered-pair rung.
    let declared = DeclaredPairs::default();
    reduce::gate_operand_pairs(a_operand, b_operand, &declared, band)?;
    reduce::gate_maximal_faces(a_operand, Operand::A, band)?;
    reduce::gate_maximal_faces(b_operand, Operand::B, band)?;

    let mut a = a_operand.clone();
    let mut b = b_operand.clone();
    let mut acc = reduce::ContactAcc::default();
    let mut ab = SweepTrace::default();
    let mut ba = SweepTrace::default();
    let ab_knobs = reduce::SweepKnobs {
        plant: plant.map(|p| p.face),
        pad_override,
    };
    // The plant names a face of `b_operand`, so it applies to the A→B
    // direction only; the pad override applies to both.
    let ba_knobs = reduce::SweepKnobs {
        plant: None,
        pad_override,
    };
    reduce::sweep_direction(
        &mut a,
        &mut b,
        Operand::A,
        &declared,
        &mut acc,
        band,
        strategy,
        &ab_knobs,
        Some(&mut ab),
        tol,
    )?;
    reduce::sweep_direction(
        &mut b,
        &mut a,
        Operand::B,
        &declared,
        &mut acc,
        band,
        strategy,
        &ba_knobs,
        Some(&mut ba),
        tol,
    )?;
    Ok((ab, ba))
}

/// **The boolean pipeline through its join**, undeclared and realized:
/// the two operand clones as the join leaves them, every null edge
/// killed, before the finish, the zip and the closing mint — the
/// production sequence itself (`ops::through_the_join`), stopped there.
/// `None` where the pipeline answers without a join to stop at. Test
/// vocabulary (`topo::test_support`), for the rows that read the rows
/// a face carries at that point.
///
/// # Errors
///
/// The pipeline's refusal on the way to its join.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn through_the_join(
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
    tol: Tol,
) -> Result<Option<crate::test_support::JoinedOperands>, BooleanError> {
    Ok(
        match ops::through_the_join(
            op,
            a,
            b,
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            true,
            tol,
        )? {
            ops::Joined::Answered(_) => None,
            ops::Joined::Connected { red, .. } => Some((red.a, red.b)),
        },
    )
}

/// [`boolean_reduce_declared`] with an explicit [`SweepStrategy`] —
/// the idealized/realized door (PERF-PLAN §4.4): production always
/// runs `Realized`; the differential suite runs both and pins
/// bit-equality. Reached via [`boolean_op_with`] for full ops.
pub(crate) fn boolean_reduce_declared_strategy<T: Decide + Bounds>(
    op: BooleanOp,
    a_operand: &Body<T>,
    b_operand: &Body<T>,
    decls: &BooleanDeclarations,
    strategy: SweepStrategy,
    tol: Tol,
) -> Result<BooleanReduction<T>, BooleanError> {
    let band = Band::linear(tol)?;
    validate_declarations(a_operand, b_operand, decls)?;
    let verified = verify_declared_contacts(a_operand, b_operand, decls, band)?;
    let declared = DeclaredPairs::build(decls, verified, a_operand, b_operand);
    reduce::gate_operand_pairs(a_operand, b_operand, &declared, band)?;
    reduce::gate_maximal_faces(a_operand, Operand::A, band)?;
    reduce::gate_maximal_faces(b_operand, Operand::B, band)?;
    // The scan is `Decide`-only; its boxes are built here, at the
    // driver the 2026-07-29 amendment ratified to read brackets.
    let pad = boxes::sweep_pad(band);
    reduce::refuse_undeclared_continuations(
        a_operand,
        b_operand,
        &declared,
        band,
        pad,
        |body, face| boxes::face_box(body, face, pad),
        |body, edge| boxes::edge_box(body, edge, pad),
    )?;

    // The reduction carves both operand clones through the Euler
    // operators; tier 1 is paid once per clone at the end of the
    // phase rather than once per operator (`crate::surgery`). Two
    // guards over two locals: each owns its own borrow, so a refusal
    // on the way closes both scopes by dropping them.
    let mut carved_a = a_operand.clone();
    let mut carved_b = b_operand.clone();
    let mut a = carved_a.begin_surgery();
    let mut b = carved_b.begin_surgery();

    // Reduction sweep, both directions (A's edges first — D9 order).
    let mut acc = reduce::ContactAcc::default();
    let knobs = reduce::SweepKnobs::default();
    reduce::sweep_direction(
        &mut a,
        &mut b,
        Operand::A,
        &declared,
        &mut acc,
        band,
        strategy,
        &knobs,
        None,
        tol,
    )?;
    reduce::sweep_direction(
        &mut b,
        &mut a,
        Operand::B,
        &declared,
        &mut acc,
        band,
        strategy,
        &knobs,
        None,
        tol,
    )?;
    let contacts = acc.finish();

    let mut null_edges = Vec::new();
    let mut null_pairs = Vec::new();
    let mut pierce_rings = Vec::new();
    let mut covered = Vec::new();
    let mut held = Vec::new();

    // Vertex-on-face classification (sonva then sonvb, as 15.5).
    for &c in &contacts.a_on_b {
        let out = vtxfac::classify_vertex_on_face(
            &mut a,
            &mut b,
            Operand::A,
            c,
            op,
            &declared,
            band,
            tol,
        )?;
        null_edges.extend(out.edges);
        null_pairs.extend(out.pairs);
        pierce_rings.extend(out.ring);
        covered.extend(out.covered);
    }
    for &c in &contacts.b_on_a {
        let out = vtxfac::classify_vertex_on_face(
            &mut b,
            &mut a,
            Operand::B,
            c,
            op,
            &declared,
            band,
            tol,
        )?;
        null_edges.extend(out.edges);
        null_pairs.extend(out.pairs);
        pierce_rings.extend(out.ring);
        covered.extend(out.covered);
    }

    // Vertex-vertex classification.
    for &c in &contacts.vv {
        let a_sectors = sectors::build_sectors(&a, Operand::A, c.a, band)?;
        let b_sectors = sectors::build_sectors(&b, Operand::B, c.b, band)?;
        let mut records = sectors::pair_search(&a_sectors, &b_sectors, band)?;
        recl::recl_sectors(
            &mut records,
            &a_sectors,
            &b_sectors,
            &a,
            &b,
            op,
            &declared,
            band,
            &mut covered,
            &mut held,
        )?;
        recl::recl_edges(
            &mut records,
            &a_sectors,
            &b_sectors,
            &a,
            &b,
            op,
            &declared,
            band,
        )?;
        let out = insert::insert_null_pairs(
            &mut a, &mut b, c, &a_sectors, &b_sectors, &records, &declared, band,
        )?;
        null_edges.extend(out.edges);
        null_pairs.extend(out.pairs);
    }
    let held = border_held(held, &covered, &null_edges, &a, &b)?;

    a.sweep_and_close();
    b.sweep_and_close();
    Ok(BooleanReduction {
        op,
        a: carved_a,
        b: carved_b,
        contacts,
        null_edges,
        null_pairs,
        pierce_rings,
        covered: covered
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect(),
        held,
    })
}

/// The held edges that bound the held region from outside. A null edge
/// is the split's scaffolding, not an edge of the face, and an edge
/// whose two faces both cover the dropped face (a seam between two
/// coplanar faces of the kept copy, or a face meeting itself) lies
/// inside the held region; both are dropped. Sorted and deduplicated.
fn border_held<T: Real>(
    mut held: Vec<HeldEdge>,
    covered: &[(FaceKey, FaceKey)],
    null_edges: &[BoolNullEdgeRecord<T>],
    a: &Body<T>,
    b: &Body<T>,
) -> Result<Vec<HeldEdge>, BooleanError> {
    let mut out = Vec::with_capacity(held.len());
    held.sort_by_key(|h| (h.holder == Operand::B, h.edge, h.face, h.at));
    held.dedup();
    for h in held {
        if null_edges
            .iter()
            .any(|r| r.operand == h.holder && r.edge == h.edge)
        {
            continue;
        }
        let body = match h.holder {
            Operand::A => a,
            Operand::B => b,
        };
        let edge = body
            .get_edge(h.edge)
            .ok_or(BooleanError::ClassificationInvariant {
                what: "a held edge no longer resolves",
            })?;
        let sides = [edge.he_plus, edge.he_minus].map(|he| body.face_of_half_edge(he));
        let covers = |f: Option<FaceKey>| {
            f.is_some_and(|f| {
                covered.contains(&match h.holder {
                    Operand::A => (f, h.face),
                    Operand::B => (h.face, f),
                })
            })
        };
        if sides[0] == sides[1] || sides.iter().all(|&f| covers(f)) {
            continue;
        }
        out.push(h);
    }
    Ok(out)
}

/// Fail-loud validation of a [`BooleanDeclarations`] payload against
/// the operands (M4 PR 5): every referenced key must resolve in its
/// operand, and declared faces must sit on carriers in the certified
/// inventory (plane, sphere, cylinder). A dangling declaration is a
/// caller bug refused before any classification runs — never a
/// silent drop (F5's no-silent-drop contract).
/// **C4's verify-at-use, at the door**: EVERY declared pair is checked
/// against the geometry before the op runs — not only the pairs the
/// classification happens to walk past — per class: `Rest` down the
/// carrier ladder, `Tangent` down the DEV-1 witness lane
/// ([`verify_tangent_declaration`]).
///
/// This closes the gap C4 names by name: "a declaration that never
/// meets geometry is a silent no-op at the op". A pair naming two
/// faces that never come near each other is exactly that shape, and
/// without this pass a lie is loud when the classifier trips over it
/// and silent when it does not — which is the same lie either way.
/// The review that found this also found the reason it had gone
/// unnoticed for a milestone: the only other verify-at-use site is the
/// REST lane, which runs on Union and only when the seam produces null
/// pairs, so a Subtract with a false declaration was never verified at
/// all.
///
/// The sense is part of each one-carrier claim, as an exact bit:
/// `Rest` demands opposed senses and a continuation aligned ones, and
/// each is contradicted by the other's.
fn verify_declared_contacts<T: Decide>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
    band: Band,
) -> Result<VerifiedDeclarations, BooleanError> {
    let mut verified = VerifiedDeclarations::default();
    for &FacePairDeclaration {
        a: fa,
        b: fb,
        class,
    } in &decls.coincident_faces
    {
        match class {
            BooleanCoincidence::Contact(ContactClass::Tangent) => {
                verify_tangent_declaration(a, fa, b, fb, band)?;
                verified.tangent.insert((fa, fb));
            }
            BooleanCoincidence::Contact(ContactClass::Rest) | BooleanCoincidence::Continuation => {
                if verify_one_carrier_declaration(a, fa, b, fb, class, band)? {
                    verified.one_carrier.insert((fa, fb));
                }
            }
        }
    }
    Ok(verified)
}

/// The refusal of a one-carrier declaration whose senses contradict
/// its class: aligned under `Rest`, opposed under a continuation. The
/// sense is a structural bit, so no `decide` ran; the predicate labels
/// the finding for the reader and never enters the K funnel.
pub(super) fn sense_contradiction(
    fa: FaceKey,
    fb: FaceKey,
    class: BooleanCoincidence,
    band: Band,
) -> BooleanError {
    let margin = |predicate| Indeterminate {
        margin: MarginDiag::INVALID,
        band,
        predicate: Some(predicate),
        terminal_sliver: false,
    };
    match class {
        BooleanCoincidence::Continuation => BooleanError::ContinuationContradicted {
            a: fa,
            b: fb,
            fact: None,
            margin: margin("continuation_senses_aligned"),
        },
        BooleanCoincidence::Contact(class) => BooleanError::ContactContradicted {
            declaration: crate::contact::DeclaredContact {
                a: fa,
                b: fb,
                class,
            },
            steer: None,
            fact: None,
            margin: margin("contact_rest_senses_opposed"),
        },
    }
}

/// The `Rest` and continuation half of [`verify_declared_contacts`]:
/// the carrier ladder in its declared posture — a definitely-different
/// carrier contradicts, an in-band residue is bridged (C4), a sliver
/// escalates — and then the sense bit the class demands.
///
/// `Ok(true)` when the ladder called the two faces ONE carrier with
/// that sense — the certificate the crossing layer's carrier-identity
/// rung reads, recorded once here instead of re-derived per event.
fn verify_one_carrier_declaration<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    class: BooleanCoincidence,
    band: Band,
) -> Result<bool, BooleanError> {
    // A carrier kind the ladder cannot describe: `validate_
    // declarations` has already had its say about which kinds this
    // op accepts, so there is nothing left to add here — and nothing
    // for the identity rung to read.
    let Some(outcome) = rest::carrier_pair_relation(a, fa, b, fb, true, band) else {
        return Ok(false);
    };
    let aligned = class == BooleanCoincidence::Continuation;
    match outcome {
        Ok(carrier_eq::CarrierRelation::SameOriented) if aligned => Ok(true),
        Ok(carrier_eq::CarrierRelation::SameOpposite) if !aligned => Ok(true),
        Ok(
            carrier_eq::CarrierRelation::SameOriented | carrier_eq::CarrierRelation::SameOpposite,
        ) => Err(sense_contradiction(fa, fb, class, band)),
        // The declared posture contradicts a definite difference, it
        // never answers `Distinct`: the same kernel-defect answer the
        // REST lane gives (`rest.rs`).
        Ok(carrier_eq::CarrierRelation::Distinct) => Err(BooleanError::ClassificationInvariant {
            what: "declaration door: declared rung returned Distinct instead of contradicting",
        }),
        Err(carrier_eq::CarrierEqError::Contradicted { fact, diag }) => Err(match class {
            BooleanCoincidence::Continuation => BooleanError::ContinuationContradicted {
                a: fa,
                b: fb,
                fact: Some(fact),
                margin: diag,
            },
            BooleanCoincidence::Contact(class) => BooleanError::ContactContradicted {
                declaration: crate::contact::DeclaredContact {
                    a: fa,
                    b: fb,
                    class,
                },
                steer: contact_verify::fit_steer(fact),
                fact: Some(fact),
                margin: diag,
            },
        }),
        Err(carrier_eq::CarrierEqError::Escalated { rung, diag }) => {
            Err(BooleanError::plane_identity(
                rung,
                PlaneDoor::OnPair(DeclarationRead::Spent(class)),
                diag,
            ))
        }
        // Unreachable with `declared: true`; refuse loudly anyway.
        Err(carrier_eq::CarrierEqError::Undeclared { diag, relation }) => {
            Err(BooleanError::UndeclaredCoincidence {
                diag,
                pair: [(Operand::A, fa), (Operand::B, fb)],
                relation,
            })
        }
    }
}

/// The conformal screen's carrier ladder contradicting a pair it ran
/// undeclared, which no verdict on an undeclared pair is: the kernel's
/// own check ([`SelfCheck::CarrierLadder`]).
fn screen_contradiction(diag: Indeterminate) -> BooleanError {
    BooleanError::Escalated {
        decision: BooleanDecision::SelfCheck(SelfCheck::CarrierLadder),
        diag,
    }
}

/// The `Tangent` half of [`verify_declared_contacts`] — admitted
/// exactly where the DEV-1 closed-form witness lane reaches
/// ([`geom_brep::tangent_locus`]: plane×cylinder along a ruling, parallel
/// cylinders), and refused typed everywhere else:
///
/// 1. **The conformal screen.** The carrier ladder runs first in its
///    DETECTOR posture: a pair it can call one carrier — structurally
///    (rung 1) or geometrically (rung 4's coincidence refusal) — is
///    one carrier (a `Rest` or a continuation), and a `Tangent` claim
///    on a conformal pair is
///    CONTRADICTED, not class-refused (a flush pair declared Tangent
///    is the wrong class, and the geometry says so).
/// 2. **The witness.** The closed-form locus derives, or the class is
///    refused typed ([`BooleanError::UnsupportedDeclarationClass`] —
///    outside the witness lane no verification can run, so no
///    declaration is admitted). A definitely-apart or
///    definitely-crossing pair is CONTRADICTED (the deciding row is
///    the locus lane's own `tangent_locus_gap`).
/// 3. **The C4 `Tangent` table** ([`contact_verify`]) along the
///    witness, over the pair's honest extent (both faces' boundary
///    vertices projected onto the locus).
fn verify_tangent_declaration<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<(), BooleanError> {
    let declaration = crate::contact::DeclaredContact {
        a: fa,
        b: fb,
        class: ContactClass::Tangent,
    };
    // 1. The conformal screen (detector posture).
    if let Some(outcome) = rest::carrier_pair_relation(a, fa, b, fb, false, band) {
        match outcome {
            Ok(CarrierRelation::Distinct) => {}
            // One carrier, structurally: conformal contact is Rest.
            Ok(CarrierRelation::SameOriented | CarrierRelation::SameOpposite) => {
                return Err(BooleanError::ContactContradicted {
                    declaration,
                    steer: None,
                    fact: None,
                    margin: Indeterminate {
                        margin: MarginDiag::INVALID,
                        band,
                        // Display-only by design: no `decide` ran here
                        // (the sameness was STRUCTURAL), so this label
                        // names the finding for the reader and never
                        // enters the K funnel — the
                        // `contact_rest_senses_opposed` precedent.
                        predicate: Some("contact_tangent_conformal"),
                        terminal_sliver: false,
                    },
                });
            }
            // One carrier, geometrically (the detector's coincidence
            // refusal): the diag carries the margins that decided.
            Err(carrier_eq::CarrierEqError::Undeclared { diag, .. }) => {
                return Err(BooleanError::ContactContradicted {
                    declaration,
                    steer: None,
                    fact: None,
                    margin: diag,
                });
            }
            Err(carrier_eq::CarrierEqError::Escalated { rung, diag }) => {
                return Err(BooleanError::plane_identity(
                    rung,
                    PlaneDoor::Screen(DeclarationRead::Spent(BooleanCoincidence::Contact(
                        declaration.class,
                    ))),
                    diag,
                ));
            }
            // Unreachable with `declared: false`; refuse loudly anyway.
            Err(carrier_eq::CarrierEqError::Contradicted { diag, .. }) => {
                return Err(screen_contradiction(diag));
            }
        }
    }
    // 2. The DEV-1 witness locus.
    //
    // **One resolution per declared face, carrying both halves.** The
    // carrier and the orientation BIT are two facts about the same
    // `&Face`, and everything below reads them off this one lookup: a
    // key that does not resolve refuses typed HERE, so no later stage
    // can answer from a sense it invented for a face that is not
    // there. The two ways a key can fail are different findings and
    // are named apart, in `validate_declarations`' own vocabulary: a
    // key with no FACE behind it did not resolve, and a face whose
    // surface key is stale lost its surface.
    let face_of =
        |body: &Body<T>, f: FaceKey, operand| -> Result<(geom::Surface<T>, bool), BooleanError> {
            let invalid = |what| BooleanError::InvalidDeclaration { operand, what };
            let face = body
                .get_face(f)
                .ok_or_else(|| invalid("declared face key does not resolve"))?;
            let surface = body
                .get_surface(face.surface)
                .ok_or_else(|| invalid("declared face lost its surface"))?;
            Ok((surface.clone(), face.sense))
        };
    let (sa, sense_a) = face_of(a, fa, Operand::A)?;
    let (sb, sense_b) = face_of(b, fb, Operand::B)?;
    let (origin, dir) = match geom_brep::tangent_locus(&sa, &sb, band) {
        Ok(geom_brep::TangentLocus::Line { origin, dir }) => (origin, dir),
        Err(geom_brep::TangentLocusError::Escalated(diag)) => {
            return Err(BooleanError::coincidence(
                Coincide::TangentLocus,
                DeclarationRead::Spent(BooleanCoincidence::Contact(declaration.class)),
                diag,
            ));
        }
        Err(geom_brep::TangentLocusError::NotTangent { .. }) => {
            return Err(BooleanError::ContactContradicted {
                declaration,
                steer: None,
                fact: None,
                margin: Indeterminate {
                    margin: MarginDiag::INVALID,
                    band,
                    predicate: Some("tangent_locus_gap"),
                    terminal_sliver: false,
                },
            });
        }
        Err(geom_brep::TangentLocusError::Unsupported { .. }) => {
            // **The ratified routing, before the class refusal.** A
            // pair the witness lane cannot serve may still be a
            // configuration the design has already ruled on: two faces
            // meeting along ONE circle. Where they do, the material
            // wedge decides which arm the rim earns and the refusal
            // names it, so an author reading it learns what the
            // geometry IS rather than only that a witness is missing.
            // Where there is no shared rim — or the samples cannot
            // settle one — the bare class refusal stands, verbatim.
            // An UNDECIDABLE rim identity escalates typed rather than
            // reading as "no rim here": the two are different findings
            // and only one of them means the geometry was examined and
            // cleared.
            let rim = rim_wedge::shared_rim(a, fa, b, fb, band).map_err(|diag| {
                BooleanError::coincidence(
                    Coincide::Rim,
                    DeclarationRead::Spent(BooleanCoincidence::Contact(declaration.class)),
                    diag,
                )
            })?;
            if let Some(rim) = rim {
                // The rim's own diameter is the extent every angular
                // margin here is metered at — the screen's, the
                // material arm's and the rim identification's alike:
                // it is the reach over which this contact's verdict is
                // consumed, and three stages levering against three
                // different arms would not be comparable.
                let extent = rim.radius + rim.radius;
                match rim_wedge::classify_shared_rim(&sa, sense_a, &sb, sense_b, rim, extent, band)
                {
                    // Wedge π: the arm is built and it answered; the
                    // declaration is what is wrong.
                    Ok(rim_wedge::RimRouting::Seam) => {
                        return Err(BooleanError::RimSeamNotDeclarable { declaration });
                    }
                    // Wedge 0/2π: the ruling's unbuilt arm.
                    Ok(rim_wedge::RimRouting::Cusp(wedge)) => {
                        return Err(BooleanError::RimCuspArmUnbuilt { declaration, wedge });
                    }
                    // **Definite counter-evidence CONTRADICTS**, and
                    // this is C4's invariant rather than a new rule:
                    // every definite verdict wins over every
                    // declaration. A rim whose tangent planes are
                    // definitely distinct at every station is a
                    // crossing, and a rim whose opposed sheets osculate
                    // is conformal contact — a `Rest` claim wearing a
                    // rim's clothes. Neither is a tangency, and saying
                    // so is strictly better than reporting the CLASS as
                    // unsupported: the geometry, not the kernel's
                    // coverage, is what refuses.
                    Ok(
                        routing @ (rim_wedge::RimRouting::Transverse
                        | rim_wedge::RimRouting::Lamina),
                    ) => {
                        return Err(BooleanError::ContactContradicted {
                            declaration,
                            steer: None,
                            fact: None,
                            margin: Indeterminate {
                                margin: MarginDiag::INVALID,
                                band,
                                // Display-only, the `contact_tangent_
                                // conformal` precedent: the deciding
                                // `decide` calls ran inside the routing
                                // and are already in the funnel under
                                // their own names, so this labels the
                                // FINDING for the reader rather than
                                // claiming a second measurement.
                                predicate: Some(match routing {
                                    rim_wedge::RimRouting::Lamina => "contact_tangent_rim_lamina",
                                    _ => "contact_tangent_rim_transverse",
                                }),
                                terminal_sliver: false,
                            },
                        });
                    }
                    // The samples could not settle the rim: the bare
                    // class refusal below stands, verbatim.
                    Err(_) => {}
                }
            }
            return Err(BooleanError::UnsupportedDeclarationClass {
                class: ContactClass::Tangent,
            });
        }
    };
    // 3. The C4 table along the witness, over the pair's extent.
    let mut t_lo: Option<T> = None;
    let mut t_hi: Option<T> = None;
    for (body, f, operand) in [(a, fa, Operand::A), (b, fb, Operand::B)] {
        for p in face_boundary_points(body, f, operand)? {
            let t = (p - origin).dot(dir);
            t_lo = Some(t_lo.map_or(t, |lo| t.min(lo)));
            t_hi = Some(t_hi.map_or(t, |hi| t.max(hi)));
        }
    }
    let (Some(t0), Some(t1)) = (t_lo, t_hi) else {
        return Err(BooleanError::InvalidDeclaration {
            operand: Operand::A,
            what: "declared face pair has no boundary vertex to meter the tangent witness",
        });
    };
    let carrier = geom::Curve3::Line { origin, dir };
    match contact_verify::contact_pair_verdict(
        a,
        fa,
        b,
        fb,
        ContactClass::Tangent,
        Some((&carrier, t0, t1)),
        band,
    ) {
        Ok(_) => Ok(()),
        Err(crate::contact::ContactRefusal::Contradicted { diag, steer }) => {
            Err(BooleanError::ContactContradicted {
                declaration,
                steer,
                fact: None,
                margin: diag,
            })
        }
        Err(crate::contact::ContactRefusal::Escalated { diag })
        | Err(crate::contact::ContactRefusal::Undeclared { diag }) => {
            Err(BooleanError::coincidence(
                Coincide::Contact,
                DeclarationRead::Spent(BooleanCoincidence::Contact(declaration.class)),
                diag,
            ))
        }
        Err(crate::contact::ContactRefusal::NotCertifiable { .. }) => {
            Err(BooleanError::UnsupportedDeclarationClass {
                class: ContactClass::Tangent,
            })
        }
    }
}

/// The face's boundary vertex positions (outer loop then rings, cycle
/// order; an empty loop contributes its lone vertex) — the witness-
/// extent datum of [`verify_tangent_declaration`].
fn face_boundary_points<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    operand: Operand,
) -> Result<Vec<Point3<T>>, BooleanError> {
    let bad = || BooleanError::InvalidDeclaration {
        operand,
        what: "declared face's boundary is unwalkable",
    };
    let f = body.get_face(face).ok_or_else(bad)?;
    let mut out = Vec::new();
    for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let l = body.get_loop(lk).ok_or_else(bad)?;
        let vertex_point = |vk| -> Result<Point3<T>, BooleanError> {
            body.get_vertex(vk)
                .and_then(|v| body.get_point(v.point))
                .copied()
                .ok_or_else(bad)
        };
        match l.boundary {
            crate::entity::LoopBoundary::Empty { vertex } => out.push(vertex_point(vertex)?),
            crate::entity::LoopBoundary::Cycle { first } => {
                for he in body.loop_cycle(first).ok_or_else(bad)? {
                    let start = body.get_half_edge(he).ok_or_else(bad)?.start;
                    out.push(vertex_point(start)?);
                }
            }
        }
    }
    Ok(out)
}

fn validate_declarations<T: Decide>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
) -> Result<(), BooleanError> {
    let bad = |operand, what| BooleanError::InvalidDeclaration { operand, what };
    // THE C8 boundary, stated once: a declared face must sit on a
    // carrier the classification's certified ladder describes — the
    // kinds `carrier_eq` carries a rung for. Kinds outside it (cone,
    // NURBS, `Approx`) refuse typed at this door; undeclared touching
    // refuses forever at the classification frontiers — the door only
    // widens what a VERIFIED declaration can unlock. Per-class
    // geometric admission (the `Tangent` witness lane) is
    // `verify_declared_contacts`' half of the door.
    //
    // This inventory is the ONE place a kind becomes declarable, and
    // that is what makes the operand gate's covered-pair admission
    // kind-generic without being kind-blind: a face whose carrier has
    // no rung here can never appear in a surviving declaration, so it
    // can never be covered there either.
    let inventory_face = |body: &Body<T>, f: FaceKey, operand| -> Result<(), BooleanError> {
        let face = body
            .get_face(f)
            .ok_or_else(|| bad(operand, "declared face key does not resolve"))?;
        match body.get_surface(face.surface) {
            Some(
                geom::Surface::Plane { .. }
                | geom::Surface::Sphere { .. }
                | geom::Surface::Cylinder { .. }
                | geom::Surface::Torus { .. },
            ) => Ok(()),
            Some(_) => Err(bad(
                operand,
                "declared face's carrier is outside the certified inventory \
                 (plane, sphere, cylinder, torus)",
            )),
            None => Err(bad(operand, "declared face lost its surface")),
        }
    };
    for &FacePairDeclaration {
        a: fa,
        b: fb,
        class,
    } in &decls.coincident_faces
    {
        // A pair declared twice under DIFFERENT classes is a caller bug
        // refused here — the check the `DeclaredPairs` map's
        // last-write-wins build would otherwise resolve silently now
        // that the op holds two class arms.
        if decls
            .coincident_faces
            .iter()
            .any(|d| (d.a, d.b) == (fa, fb) && d.class != class)
        {
            return Err(bad(
                Operand::A,
                "one face pair is declared twice under different contact classes",
            ));
        }
        inventory_face(a, fa, Operand::A)?;
        inventory_face(b, fb, Operand::B)?;
    }
    let carried = |body: &Body<T>, c: &CarriedContacts, operand| -> Result<(), BooleanError> {
        for carried in &c.vv {
            let pair = carried.pair;
            if body.get_vertex(pair.a).is_none() || body.get_vertex(pair.b).is_none() {
                return Err(bad(operand, "carried v-v vertex key does not resolve"));
            }
            if pair.a == pair.b {
                return Err(bad(operand, "carried v-v pair names one vertex twice"));
            }
        }
        for carried in &c.vf {
            let rest = carried.rest;
            if body.get_vertex(rest.vertex).is_none() {
                return Err(bad(operand, "carried v-on-f vertex key does not resolve"));
            }
            if body.get_face(rest.face).is_none() {
                return Err(bad(operand, "carried v-on-f face key does not resolve"));
            }
        }
        Ok(())
    };
    carried(a, &decls.carried_a, Operand::A)?;
    carried(b, &decls.carried_b, Operand::B)?;
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// **A strut gives cover only where its local tangency is a global
    /// side.** The table is exhaustive over both kinds, so a kind added
    /// to `SurfaceKind` fails here until someone decides its arm. The
    /// admitted four are plane against cylinder or sphere, either way
    /// round. A spline strut (STEP adoption, blends), a cone, a torus
    /// or a fitted stand-in gives no cover, so its crossing stays a
    /// typed frontier.
    #[test]
    fn only_plane_against_cylinder_or_sphere_struts_certify_a_side() {
        use geom_brep::SurfaceKind::{self, Approx, Cone, Cylinder, Nurbs, Plane, Sphere, Torus};
        let all = [Plane, Cylinder, Cone, Sphere, Torus, Nurbs, Approx];
        let visit = |k: SurfaceKind| match k {
            Plane | Cylinder | Cone | Sphere | Torus | Nurbs | Approx => k,
        };
        let admitted: Vec<(SurfaceKind, SurfaceKind)> = all
            .iter()
            .flat_map(|&p| all.iter().map(move |&q| (visit(p), q)))
            .filter(|&(p, q)| strut_certifies_side(p, q))
            .collect();
        assert_eq!(
            admitted,
            [
                (Plane, Cylinder),
                (Plane, Sphere),
                (Cylinder, Plane),
                (Sphere, Plane)
            ]
        );
        for k in [Nurbs, Approx, Cone, Torus] {
            assert!(!strut_certifies_side(k, Plane), "{k:?} strut on a plane");
            assert!(!strut_certifies_side(Plane, k), "plane strut on a {k:?}");
        }
    }

    /// **The one-sided cover is directed** (C4 states it one way): a
    /// key inserted parent → target answers for that direction only,
    /// and a same-operand pair is never covered.
    #[test]
    fn the_one_sided_cover_is_keyed_parent_to_target() {
        let mut faces: slotmap::SlotMap<FaceKey, ()> = slotmap::SlotMap::with_key();
        let (f, g) = (faces.insert(()), faces.insert(()));
        let mut pairs = DeclaredPairs::default();
        pairs
            .one_sided
            .insert((tagged(Operand::A, f), tagged(Operand::B, g)));
        assert!(pairs.one_sided(Operand::A, f, Operand::B, g));
        assert!(!pairs.one_sided(Operand::B, g, Operand::A, f));
        assert!(!pairs.one_sided(Operand::A, f, Operand::A, g));
    }

    /// **The conformal screen's ladder contradicting an undeclared pair
    /// is the kernel's own check.** The arm is unreachable (the screen
    /// runs the ladder with `declared: false`, which contradicts
    /// nothing), so the row pins its routing: a mutant sending it to a
    /// coincidence reds here.
    #[test]
    fn a_contradiction_at_the_undeclared_screen_is_the_kernels_own_check() {
        let diag = Indeterminate {
            margin: MarginDiag::value(5e-9),
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some("bool_plane_offset"),
            terminal_sliver: false,
        };
        let err = screen_contradiction(diag);
        assert!(
            matches!(
                err,
                BooleanError::Escalated {
                    decision: BooleanDecision::SelfCheck(SelfCheck::CarrierLadder),
                    ..
                }
            ),
            "{err:?}"
        );
        assert!(err.to_string().ends_with(KERNEL_DEFECT_ENDING), "{err}");
    }

    /// **[`BooleanOp::ALL`] holds each operation once, and an
    /// operation added to the enum cannot reach a release without
    /// someone reading this row** — the idiom `VerbKind::ALL`
    /// (`crates/verbs/src/verb.rs`) and `SurfaceField::ALL`
    /// (`crates/topo/src/param_source.rs`) are held to.
    ///
    /// **What is forced**: the match below is exhaustive with no
    /// wildcard, so an operation added to the enum fails this file
    /// until it is visited here. And the no-repeats half is what makes
    /// the count a census rather than a length: with every entry
    /// distinct, a `len` equal to `ops` means `ALL` holds each of them
    /// exactly once.
    ///
    /// **What is NOT forced, measured**: `ops` itself. Every arm names
    /// the same total so that visiting means re-deciding it — but
    /// nothing checks that number against the enum, and the arm an
    /// author adds is the arm they copied. A fourth variant with the
    /// arm `Xor => 3` compiles and passes GREEN with `Xor` absent from
    /// `ALL`. The row forces the visit, not the edit. That is the
    /// idiom's hole and not this row's alone — it is inherited from the
    /// two censuses cited above — so it is filed as
    /// `work/census/all-census-idiom-forces-the-visit-not-the-update`
    /// rather than patched here in one of three places.
    #[test]
    fn all_is_every_operation() {
        let ops = match BooleanOp::Union {
            BooleanOp::Union => 3,
            BooleanOp::Intersect => 3,
            BooleanOp::Subtract => 3,
        };
        for (i, op) in BooleanOp::ALL.iter().enumerate() {
            assert!(
                !BooleanOp::ALL[..i].contains(op),
                "{op:?} appears twice in BooleanOp::ALL"
            );
        }
        assert_eq!(
            BooleanOp::ALL.len(),
            ops,
            "BooleanOp::ALL has drifted from the declaration — it holds {} operations, the enum has {ops}",
            BooleanOp::ALL.len()
        );
    }

    /// S6 (two-tolerance, D4 ¶1 addendum): the boolean coincidence
    /// pair — `UndeclaredCoincidence` (exactly-on OR in-band, per the
    /// plane-identity rung 4) and `Escalated` (in-band elsewhere) —
    /// offers the declaration exactly once per message; the escalated
    /// arm only on a margin a declaration reads.
    #[test]
    fn coincidence_pair_carries_the_shared_recourse_once() {
        let diag = |margin| Indeterminate {
            margin,
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some("bool_plane_offset"),
            terminal_sliver: false,
        };
        // The escalated arm, as the lookup mints it for an undeclared
        // pair of planar corners.
        let door = DeclaredPairs::without_struts(&BooleanDeclarations::none(), Default::default())
            .on_pair_door(
                (
                    Operand::A,
                    FaceKey::default(),
                    Operand::B,
                    FaceKey::default(),
                ),
                Some(PlaneRelation::SameOpposite),
            );
        for (margin, offers) in [(MarginDiag::value(5e-9), 1), (MarginDiag::INVALID, 0)] {
            let msg =
                BooleanError::plane_identity(PlaneRung::Parallel, door, diag(margin)).to_string();
            assert_eq!(
                msg.matches("declare the coincidence").count(),
                offers,
                "{msg}"
            );
        }
        // The undeclared arm, in BOTH sub-shapes rung 4 produces: the
        // exactly-on refusal (Invalid margin, as synthesized) and the
        // in-band refusal (Value margin) — one message, one recourse.
        // Payload for the R3 fields: a null-key pair (the message
        // renders neither keys nor relation — the typed payload is
        // the document layer's to name).
        let pair = [
            (Operand::A, FaceKey::default()),
            (Operand::B, FaceKey::default()),
        ];
        for margin in [MarginDiag::INVALID, MarginDiag::value(5e-9)] {
            let msg = BooleanError::UndeclaredCoincidence {
                diag: diag(margin),
                pair,
                relation: PlaneRelation::SameOpposite,
            }
            .to_string();
            assert_eq!(msg.matches(COINCIDENCE_RECOURSE).count(), 1, "{msg}");
        }
        // The synthesized-Invalid definite arm renders the honest
        // statement, never the poisoned-margin text (S6 review,
        // MAJOR-1).
        let msg = BooleanError::UndeclaredCoincidence {
            diag: diag(MarginDiag::INVALID),
            pair,
            relation: PlaneRelation::SameOpposite,
        }
        .to_string();
        assert!(msg.contains("exactly zero"), "{msg}");
        assert!(!msg.contains("margin is invalid"), "{msg}");
    }

    /// The non-finite chord arm gives the cause and a recourse that can
    /// WORK, and offers
    /// the coincidence recourse ZERO times — no tolerance lever
    /// reaches an overflowed chord, so naming one would be the
    /// wrong-recourse defect `memories/refusal-text-is-not-cause.md`
    /// is about.
    ///
    /// This and the `SplitReduceError` twin are the only direct pins
    /// on the two wrapper arms: there is no end-to-end row that drives
    /// a real `Body` into `sector_shape`'s rung 0, because that needs
    /// an orbit chord past ~1e154 surviving body construction. The
    /// translation itself (`SectorFault::NonFiniteChord` to this arm)
    /// is held by the exhaustive `map_err` in `sectors.rs` and by
    /// nothing else.
    #[test]
    fn non_finite_sector_chord_names_the_cause_and_no_tolerance_recourse() {
        let msg = BooleanError::NonFiniteSectorChord {
            vertex: VertexKey::default(),
            face: FaceKey::default(),
        }
        .to_string();
        assert!(msg.contains("has no finite length"), "{msg}");
        assert!(msg.contains(geom_core::RANGE_RECOURSE), "{msg}");
        assert_eq!(msg.matches(COINCIDENCE_RECOURSE).count(), 0, "{msg}");
        assert!(!msg.contains("zero length"), "{msg}");
        assert_eq!(
            BooleanError::NonFiniteSectorChord {
                vertex: VertexKey::default(),
                face: FaceKey::default(),
            }
            .kind(),
            BooleanErrorKind::NonFiniteSectorChord
        );
    }

    /// **Every M5 S1 sub-frontier refusal ends in its own lever or the
    /// frontier's ending, and offers no declaration**: the pair is
    /// declared and verified before the zip meets its sub-frontier, and
    /// a definite frontier names no tolerance. Each states its
    /// sub-frontier with no stage label, and one recourse; the holes'
    /// mismatches name the move that matches them, and the rest (the
    /// Euler operators' own refusals, and seam configurations a contact
    /// already planar can meet) say there is no way through yet.
    #[test]
    fn every_rest_zip_frontier_ends_in_its_own_lever_and_no_declaration() {
        use strum::IntoEnumIterator as _;
        use test_utils::refusal::{recourse_markers, stage_prefixes, subjectless_escalations};
        const HOLES: &str = "Recourse: make the holes inside the declared contact match, one \
                             for one and corner for corner, across the two parts";
        for what in RestZipFrontier::iter() {
            let msg = BooleanError::RestZipUnsupported { what }.to_string();
            assert_eq!(recourse_markers(&msg), 1, "{what:?}: {msg}");
            assert!(
                stage_prefixes(&msg, &[]).is_empty() && subjectless_escalations(&msg).is_empty(),
                "{what:?}: {msg}"
            );
            let holes = matches!(
                what,
                RestZipFrontier::HoleVertexUnmatched | RestZipFrontier::HoleCyclesIncongruent
            );
            let ending = if holes {
                HOLES
            } else {
                geom_core::NOT_YET_ENDING
            };
            assert!(
                msg.contains(&format!("({})", what.what()))
                    && msg.ends_with(ending)
                    && !msg.contains("declare the")
                    && !msg.contains("tolerance")
                    && !msg.contains("union zip:"),
                "{what:?}: {msg}"
            );
        }
    }

    /// **A declared face key that resolves to no face refuses typed,
    /// naming what happened.** The `Tangent` verifier reads two facts
    /// off each declared face — its carrier and its orientation bit —
    /// and takes both from ONE resolution, so a key with no face
    /// behind it cannot reach a material verdict through a sense the
    /// door invented for it. The public entry refuses such a key
    /// earlier still, which is why this row calls the verifier
    /// directly: defence in depth is only depth if the inner layer is
    /// exercised.
    ///
    /// **This is a REGRESSION pin, not an anti-vacuity row**, and the
    /// distinction is worth having in writing: the refusal is not new
    /// — the carrier resolution at this position always refused a
    /// stale key — so nothing here goes red for the fold alone. Two
    /// things together would red it: the one resolution split back
    /// into two, AND a key that resolves to a face whose surface is
    /// missing rather than to no face at all. What it therefore pins
    /// is the `what` string, because that is the half that WAS wrong:
    /// a key with no face behind it used to report the face's surface
    /// as lost, conflating the two findings `validate_declarations`
    /// already distinguishes.
    #[test]
    fn a_declared_face_that_does_not_resolve_refuses_typed() {
        let empty = Body::<f64>::new();
        let err = verify_tangent_declaration(
            &empty,
            FaceKey::default(),
            &empty,
            FaceKey::default(),
            Band::new(1e-9, 1e-8).unwrap(),
        )
        .expect_err("a key with no face behind it cannot be verified");
        assert!(
            matches!(
                err,
                BooleanError::InvalidDeclaration {
                    operand: Operand::A,
                    what: "declared face key does not resolve",
                }
            ),
            "a missing declared face is a caller bug, not a classification, and the \
             refusal says which caller bug it is: {err:?}"
        );
    }

    /// One [`BooleanError`] per arm whose payload is keys, spans,
    /// enums and `&'static str` — everything the projection can be
    /// checked on without reaching into another crate's error type.
    /// Arms nesting a foreign refusal (`Euler`, `Join`, `Merge`,
    /// `Revert`, `GraftRecertify`, `CrossingInsertion`) are absent by
    /// the same rule.
    fn sample_errors() -> Vec<BooleanError> {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let diag = Indeterminate {
            margin: MarginDiag::value(5e-9),
            band,
            predicate: Some("bool_plane_offset"),
            terminal_sliver: false,
        };
        let face = FaceKey::default();
        let edge = EdgeKey::default();
        let declaration = crate::contact::DeclaredContact {
            a: face,
            b: face,
            class: ContactClass::Rest,
        };
        vec![
            BooleanError::Band(Band::new(1.0, 1.0).unwrap_err()),
            BooleanError::CurvedBooleanUnsupported {
                operand: Operand::A,
                face,
                kind: geom_brep::SurfaceKind::Cone,
            },
            BooleanError::CurvedSectorSideUnsupported {
                verdict: geom_brep::recourse::Refused::Negative {
                    margin: MarginDiag::value(-1e-3),
                },
            },
            BooleanError::NonFiniteSectorChord {
                vertex: VertexKey::default(),
                face,
            },
            BooleanError::CurvedPierceUnsupported {
                operand: Operand::A,
                face,
                edge,
                band,
            },
            BooleanError::CurvedEdgeUnsupported {
                operand: Operand::B,
                edge,
            },
            BooleanError::PointSplitCarrierUnsupported {
                operand: Operand::A,
                edge,
            },
            BooleanError::ArcLoopContainmentUnsupported {
                operand: Operand::A,
                r#loop: crate::entity::LoopKey::default(),
            },
            BooleanError::ScaffoldingOperand {
                operand: Operand::A,
                edge,
            },
            BooleanError::NonMaximalFaces {
                operand: Operand::A,
                edge,
            },
            BooleanError::CoplanarNeighbours {
                operand: Operand::A,
                faces: [face, face],
                offset: NeighbourOffset::Undecided(diag),
            },
            BooleanError::plane_identity(
                PlaneRung::Parallel,
                DeclaredPairs::without_struts(&BooleanDeclarations::none(), Default::default())
                    .on_pair_door(
                        (Operand::A, face, Operand::B, face),
                        Some(PlaneRelation::SameOpposite),
                    ),
                diag,
            ),
            BooleanError::UndeclaredCoincidence {
                diag,
                pair: [(Operand::A, face), (Operand::B, face)],
                relation: PlaneRelation::SameOpposite,
            },
            BooleanError::DeclarationContradicted {
                fact: Contradiction::PlanesApart,
            },
            BooleanError::ContactContradicted {
                declaration,
                margin: diag,
                steer: None,
                fact: None,
            },
            BooleanError::ContinuationContradicted {
                a: face,
                b: face,
                fact: None,
                margin: diag,
            },
            BooleanError::UnsupportedDeclarationClass {
                class: ContactClass::Tangent,
            },
            BooleanError::RimSeamNotDeclarable { declaration },
            BooleanError::InvalidDeclaration {
                operand: Operand::A,
                what: "a stale key",
            },
            BooleanError::PairingMismatch {
                a_vertex: VertexKey::default(),
                b_vertex: VertexKey::default(),
            },
            BooleanError::ClassificationInvariant {
                what: "an invariant",
            },
            BooleanError::CorruptOperand {
                operand: Operand::A,
                vertex: VertexKey::default(),
            },
            BooleanError::CurvedPairUnsupported {
                op: None,
                site: PairRefusalSite::OperandGate,
                operand: Operand::A,
                face,
                kind: geom_brep::SurfaceKind::Cone,
                other_face: face,
                other_kind: geom_brep::SurfaceKind::Plane,
            },
            BooleanError::CurvedPairUnsupported {
                op: Some(BooleanOp::Union),
                site: PairRefusalSite::InteriorLoopGuard,
                operand: Operand::A,
                face,
                kind: geom_brep::SurfaceKind::Torus,
                other_face: face,
                other_kind: geom_brep::SurfaceKind::Plane,
            },
            BooleanError::NurbsExtentUnsupported {
                operand: Operand::A,
                face,
            },
            BooleanError::FallbackExtentUnsupported {
                operand: Operand::A,
                face,
                what: "an uncertifiable pose",
            },
            BooleanError::SpheresMeet {
                operand: Operand::A,
                face,
                verdict: geom_brep::recourse::Refused::Negative {
                    margin: MarginDiag::value(-0.5),
                },
            },
            BooleanError::GermFrameUnsupported {
                a_face: face,
                a_kind: geom_brep::SurfaceKind::Cone,
                b_face: face,
                b_kind: geom_brep::SurfaceKind::Torus,
            },
            BooleanError::GermFrameCylinderPinch {
                a_face: face,
                b_face: face,
                evidence: geom_brep::RadiusEvidence::None,
            },
            BooleanError::Pcurves {
                source: crate::pcurves::PcurveMintError::Corrupt,
            },
            BooleanError::RestZipUnsupported {
                what: RestZipFrontier::SlitFaceHoles,
            },
            BooleanError::JoinDesync { what: "a lockstep" },
            BooleanError::TornComponent {
                operand: Operand::A,
                shell: ShellKey::default(),
            },
            BooleanError::ShellWitnessExhausted {
                operand: Operand::B,
                shell: ShellKey::default(),
                on_boundary: 26,
                in_band: 0,
                first_in_band: None,
            },
            BooleanError::Containment(
                crate::boolean::solid_contain::PointInSolidError::RayExhausted,
            ),
            BooleanError::SeamOrientation {
                a_face: face,
                b_face: face,
            },
            BooleanError::ZipCorrespondence { what: "a record" },
            BooleanError::ResultInvalid { errors: Vec::new() },
            BooleanError::ResultVolumeImplausible {
                which: "vol(A ∖ B) ≤ vol(A)",
                got: "1.0".to_owned(),
                bound: "0.5".to_owned(),
            },
            BooleanError::VolumeUnmeasured {
                operand: None,
                source: crate::props::MassPropsError::Face {
                    face,
                    source: geom_brep::props::PropsError::Unimplemented,
                },
            },
            BooleanError::VolumeCorrupt {
                operand: Some(Operand::A),
                source: crate::props::MassPropsError::Corrupt { what: "a face key" },
            },
            BooleanError::VolumeUndecided {
                which: "vol(A ∖ B) ≤ vol(A)",
            },
            BooleanError::UnrepresentableResult,
        ]
    }

    /// **The phantom direction, closed by the compiler; the pairing
    /// direction, closed by construction.**
    ///
    /// [`BooleanError::kind`] is exhaustive over the ERROR, so an arm
    /// added there reds this crate. `label` below is exhaustive over
    /// the KIND, so a variant added to [`BooleanErrorKind`] alone reds
    /// HERE, by name, in the crate that owns both — rather than in
    /// whatever downstream crate next maps the enum, of which there are
    /// currently none. `path_error_tag`
    /// (`crates/pncad-py/src/tags.rs`) and the `VerbKind::ALL` census
    /// (`crates/verbs/src/verb.rs`) are the in-tree precedents for
    /// guarding a hand-written mirror with a compile-time visit.
    ///
    /// Neither exhaustiveness objects to an arm PROJECTED to the wrong
    /// kind, which type-checks. That is what the errors below are for:
    /// each is built, projected, and its kind's name compared with the
    /// variant name `Debug` prints for the error itself, so a
    /// mis-projected arm and a mis-labelled arm both fail here with no
    /// expected value written down twice.
    ///
    /// **The errors are spot checks, not a census.** They cover the
    /// arms whose payloads are keys, spans and `&'static str`; an arm
    /// whose payload is another module's or crate's typed refusal is
    /// not built here, so a mis-projection confined to one of those is
    /// not caught. Nothing reds when an arm is missing from this list —
    /// this row accuses no author of anything it has not measured.
    #[test]
    fn each_kind_has_an_arm_and_each_built_arm_projects_to_its_own_kind() {
        fn label(kind: BooleanErrorKind) -> &'static str {
            match kind {
                BooleanErrorKind::Band => "Band",
                BooleanErrorKind::CurvedBooleanUnsupported => "CurvedBooleanUnsupported",
                BooleanErrorKind::DegenerateTorus => "DegenerateTorus",
                BooleanErrorKind::CurvedSectorSideUnsupported => "CurvedSectorSideUnsupported",
                BooleanErrorKind::CurvedPierceUnsupported => "CurvedPierceUnsupported",
                BooleanErrorKind::CurvedEdgeUnsupported => "CurvedEdgeUnsupported",
                BooleanErrorKind::PointSplitCarrierUnsupported => "PointSplitCarrierUnsupported",
                BooleanErrorKind::ArcLoopContainmentUnsupported => "ArcLoopContainmentUnsupported",
                BooleanErrorKind::ScaffoldingOperand => "ScaffoldingOperand",
                BooleanErrorKind::NonMaximalFaces => "NonMaximalFaces",
                BooleanErrorKind::CoplanarNeighbours => "CoplanarNeighbours",
                BooleanErrorKind::NonFiniteSectorChord => "NonFiniteSectorChord",
                BooleanErrorKind::UnderflowedSectorChord => "UnderflowedSectorChord",
                BooleanErrorKind::Escalated => "Escalated",
                BooleanErrorKind::UndeclaredCoincidence => "UndeclaredCoincidence",
                BooleanErrorKind::DeclarationContradicted => "DeclarationContradicted",
                BooleanErrorKind::ContactContradicted => "ContactContradicted",
                BooleanErrorKind::ContinuationContradicted => "ContinuationContradicted",
                BooleanErrorKind::UnsupportedDeclarationClass => "UnsupportedDeclarationClass",
                BooleanErrorKind::RimSeamNotDeclarable => "RimSeamNotDeclarable",
                BooleanErrorKind::RimCuspArmUnbuilt => "RimCuspArmUnbuilt",
                BooleanErrorKind::InvalidDeclaration => "InvalidDeclaration",
                BooleanErrorKind::PairingMismatch => "PairingMismatch",
                BooleanErrorKind::ClassificationInvariant => "ClassificationInvariant",
                BooleanErrorKind::CorruptOperand => "CorruptOperand",
                BooleanErrorKind::CrossingInsertion => "CrossingInsertion",
                BooleanErrorKind::CurvedPairUnsupported => "CurvedPairUnsupported",
                BooleanErrorKind::NurbsExtentUnsupported => "NurbsExtentUnsupported",
                BooleanErrorKind::FallbackExtentUnsupported => "FallbackExtentUnsupported",
                BooleanErrorKind::SpheresMeet => "SpheresMeet",
                BooleanErrorKind::GermFrameUnsupported => "GermFrameUnsupported",
                BooleanErrorKind::GermFrameCylinderPinch => "GermFrameCylinderPinch",
                BooleanErrorKind::Euler => "Euler",
                BooleanErrorKind::Pcurves => "Pcurves",
                BooleanErrorKind::Join => "Join",
                BooleanErrorKind::RestZipUnsupported => "RestZipUnsupported",
                BooleanErrorKind::JoinDesync => "JoinDesync",
                BooleanErrorKind::TornComponent => "TornComponent",
                BooleanErrorKind::ShellWitnessExhausted => "ShellWitnessExhausted",
                BooleanErrorKind::Containment => "Containment",
                BooleanErrorKind::Revert => "Revert",
                BooleanErrorKind::SeamOrientation => "SeamOrientation",
                BooleanErrorKind::ZipCorrespondence => "ZipCorrespondence",
                BooleanErrorKind::Merge => "Merge",
                BooleanErrorKind::ResultInvalid => "ResultInvalid",
                BooleanErrorKind::ResultVolumeImplausible => "ResultVolumeImplausible",
                BooleanErrorKind::VolumeUnmeasured => "VolumeUnmeasured",
                BooleanErrorKind::VolumeCorrupt => "VolumeCorrupt",
                BooleanErrorKind::VolumeUndecided => "VolumeUndecided",
                BooleanErrorKind::UnrepresentableResult => "UnrepresentableResult",
                BooleanErrorKind::GraftRecertify => "GraftRecertify",
            }
        }
        /// The variant name `Debug` opens with.
        fn variant_of(err: &BooleanError) -> String {
            format!("{err:?}")
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect()
        }
        for err in sample_errors() {
            assert_eq!(
                label(err.kind()),
                variant_of(&err),
                "kind() projects each arm to its own kind, and label names it"
            );
        }
    }
}
