//! Booleans, part 1 (M3 PR 4): **reduction + classification across two
//! bodies** — ch. 15 §§15.4–15.6 re-derived under our conventions, with
//! the TOG 1986 second witness supplying the unprinted on-edge
//! machinery (Tables II/III). Scope is BINDING (M3-PLAN PR item 4):
//! reduction sweep, the three ON-sets as declared-contact records
//! (F1/F2), vertex-vertex sector classification, vertex-on-face
//! classification with the ring insertion, on-edge machinery, and
//! paired null-edge insertion with explicit cross-body correspondence
//! keys (F9/F12). NO joining, NO result generation, NO containment
//! fallback — those are PR 5. Both operands are functionally untouched
//! (annotated clones come back in [`BooleanReduction`]).
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
pub(crate) mod combine;
pub mod contact_verify;
mod contain;
mod finish;
pub(crate) mod insert;
mod join;
mod ops;
pub mod plane_eq;
pub(crate) mod recl;
pub(crate) mod reduce;
mod rest;
pub(crate) mod sectors;
pub mod solid_contain;
pub mod tables;
pub mod voids;
pub(crate) mod vtxfac;
mod zip;

use geom_core::{
    Band, BandError, Bounds, COINCIDENCE_RECOURSE, Decide, Indeterminate, MarginDiag, Point3, Real,
    Tol,
};

use crate::body::Body;
use crate::chord_join::SplitJoinError;
use crate::contact::ContactClass;
use crate::entity::{EdgeKey, FaceKey, ShellKey, VertexKey};
use crate::euler::EulerOpError;
use crate::merge_faces::MergeCoplanarError;
use crate::revert::RevertError;
use crate::validate::ValidationError;

pub use carrier_eq::{CarrierDesc, CarrierEqError, CarrierRelation, carrier_eq};
pub use contain::{ContainError, FaceContainment, contfp, curved_face_containment};
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
pub use rest::{
    TangentLocus, TangentLocusError, carrier_pair_relation, carrier_pair_verdict, face_carrier,
    flush_pair_relation, tangent_locus,
};
pub use solid_contain::{PointInSolidError, SolidContainment, point_in_solid};
pub use voids::{VoidContainment, VoidEvidence, VoidInsertError, VoidInserted, insert_void};

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
    /// Cross-operand declared face pairs, each naming its class:
    /// classification treats a `Rest` pair's carriers as the same
    /// carrier (orientation decided, contradiction refused —
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

/// One declared cross-operand face pair AND the class it asserts.
///
/// The class rides the pair rather than a parallel list because the
/// two are one fact: "these faces are in contact, of THIS kind". A
/// pair whose class had to be looked up elsewhere could be read
/// without it, and reading a declaration without its class is how a
/// `Tangent` intent gets verified against the conformal table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FacePairDeclaration {
    /// The A-operand face.
    pub a: FaceKey,
    /// The B-operand face.
    pub b: FaceKey,
    /// The asserted class.
    pub class: ContactClass,
}

impl FacePairDeclaration {
    /// A declared pair. There is no class-less constructor: the class
    /// is an argument at every mint site, so "I forgot the class" is a
    /// compile error rather than a silent `Rest`.
    pub fn new(a: FaceKey, b: FaceKey, class: ContactClass) -> Self {
        Self { a, b, class }
    }

    /// The `Rest` pair — the conformal class S1's planar declarations
    /// have always meant, spelled out.
    pub fn rest(a: FaceKey, b: FaceKey) -> Self {
        Self::new(a, b, ContactClass::Rest)
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
    map: std::collections::BTreeMap<(FaceKey, FaceKey), ContactClass>,
}

impl DeclaredPairs {
    pub(crate) fn build(decls: &BooleanDeclarations) -> Self {
        Self {
            map: decls
                .coincident_faces
                .iter()
                .map(|d| ((d.a, d.b), d.class))
                .collect(),
        }
    }

    /// The class the (operand-tagged) face pair is declared under, if
    /// any. Same-operand pairs are never declared here
    /// (operand-internal coplanarity is the producing op's merge, not
    /// this op's).
    pub(crate) fn class_of(
        &self,
        o1: Operand,
        f1: FaceKey,
        o2: Operand,
        f2: FaceKey,
    ) -> Option<ContactClass> {
        match (o1, o2) {
            (Operand::A, Operand::B) => self.map.get(&(f1, f2)).copied(),
            (Operand::B, Operand::A) => self.map.get(&(f2, f1)).copied(),
            _ => None,
        }
    }

    /// Whether the pair is declared as the CONFORMAL class — the
    /// question the classification stages actually ask (a `Tangent`
    /// pair does not license same-carrier treatment).
    pub(crate) fn declares_rest(&self, o1: Operand, f1: FaceKey, o2: Operand, f2: FaceKey) -> bool {
        self.class_of(o1, f1, o2, f2) == Some(ContactClass::Rest)
    }
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
}

impl<T: Real> BooleanReduction<T> {
    /// The minted null edges of one operand's clone, insertion order —
    /// PR 5 (joining) walks each solid's scaffolding separately; this
    /// is the per-operand view of [`Self::null_edges`].
    pub fn null_edges_of(&self, operand: Operand) -> impl Iterator<Item = &BoolNullEdgeRecord<T>> {
        self.null_edges.iter().filter(move |r| r.operand == operand)
    }
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
    CurvedBooleanUnsupported {
        /// The offending operand and face.
        operand: Operand,
        /// The face.
        face: FaceKey,
        /// Its surface kind — the C5 table row the refusal cites.
        kind: geom_brep::SurfaceKind,
    },
    /// A sweep event definitely lands on a CURVED face away from its
    /// boundary, a vertex sits ON a curved surface, or a curved-carrier
    /// edge cannot be cleared against a curved face: the curved PIERCE
    /// door — point-in-face trim containment on a curved chart at
    /// boolean classification, and the v-on-curved-face ring insertion
    /// behind it — does not exist yet (the M5 envelope's frontier; the
    /// C5 table routes the SECTIONS, this is the crossing layer). The
    /// **definite** half of a two-tolerance pair: the very same
    /// clearance margin one band-width away escalates as
    /// [`BooleanError::Escalated`] on `bool_line_cylinder_clearance`
    /// instead, and both halves quote the band and end on the shared
    /// recourse.
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
    /// point parameter and got one without. `Circle` and `Ellipse`
    /// reach here: the operand gate admits them, so this is a
    /// **narrower** condition than the gate's, at a later stage — which
    /// is exactly why it is its own variant rather than a second reach
    /// of [`Self::CurvedEdgeUnsupported`]. One doc and one message
    /// covering both could name neither.
    PointSplitCarrierUnsupported {
        /// The offending operand and edge.
        operand: Operand,
        /// The edge.
        edge: EdgeKey,
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
    /// A reduction/classification predicate escalated (in-band margin):
    /// the operand pair is ill-conditioned at this ε — a genuine
    /// sliver (F6). Never a snap, never a guess.
    Escalated {
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
    /// instead of re-running any decide on the error path.
    UndeclaredCoincidence {
        /// The escalation site's diagnostics.
        diag: Indeterminate,
        /// The coincident face pair, each with the operand it lives
        /// in. Cross-operand at the classification sites; both
        /// entries share one operand at the F7 maximal-faces gate
        /// (two faces of ONE body coinciding without shared source).
        pair: [(Operand, FaceKey); 2],
        /// The decided orientation ([`PlaneRelation::SameOriented`]
        /// or [`PlaneRelation::SameOpposite`], never `Distinct`) —
        /// the relation a declaration of this pair would assert.
        relation: PlaneRelation,
    },
    /// A declared coincidence contradicts the geometry (the declared
    /// pair's planes are definitely distinct) — the recipe's intent
    /// cannot be realized; refused loudly, never glued (M4 PR 5).
    DeclarationContradicted {
        /// The contradicting predicate's diagnostics.
        diag: Indeterminate,
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
        /// The margin that decided, and its predicate.
        margin: Indeterminate,
        /// Extra recourse steering when the counter-evidence has a
        /// named remedy (AQ6's designed-clearance arm).
        steer: Option<&'static str>,
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
    /// The gate asks two questions and both have to answer yes: does
    /// this face's KIND have a wired arm, and can this face REACH the
    /// other operand at all. The second is decided at box-level
    /// conservatism (`reduce::first_unsupported_pair`), which is why
    /// the payload names a pair rather than a body: a cone or a torus
    /// whose box clears every face of the other operand cannot enter
    /// a crossing, a section or a germ pair, so the operation does
    /// not depend on its kind and the gate says nothing about it.
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
    /// - **Sphere**: LIVE since M5 S13 — the `(Plane, Sphere)` germ
    ///   arm (exact C5 Circle) plus the extent-certified fallback
    ///   re-cut; no longer gated here.
    /// - **Cone / torus**: the germ-pair join dispatch wires
    ///   `(Plane, Cylinder)` and `(Plane, Sphere)` only (PR 9c
    ///   deviation 1 lineage). The cyl×sphere fitted-chord window's
    ///   blocker MOVED at M6-2: `Pcurve::Fitted` now exists and
    ///   certifies at rest (the SSI enclosure/certify stack is no
    ///   longer `f64`-only), so what is left is the JOIN LANE itself —
    ///   `run_azimuth_window`/`chart_pcurve` have no cyl×sphere window
    ///   analog, and building one is banked past M6 (M6-PLAN: the
    ///   windows chase the lift).
    /// - **NURBS**: no edge×NURBS-face crossing layer at all
    ///   (deviation 5), and the fallback's extent test is unwritable
    ///   for the kind ([`BooleanError::NurbsExtentUnsupported`]).
    ///
    /// These are refused UP FRONT because their downstream failure is
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
        /// The op this refusal is specific to (never `Union`), or
        /// `None` when the kind has no arm under any op.
        op: Option<BooleanOp>,
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
    /// **A germ pair whose section frame has no arm.** The join's
    /// matcher asks each germ pair for the LOCUS its germ line rides,
    /// and the answer drives which facing test runs: a straight locus
    /// takes the chord test, a conic locus the rotational-sense test.
    /// "No frame" therefore MEANS "the locus is straight", and only a
    /// PLANE×PLANE pair earns that answer structurally — its section
    /// is a line by construction. Every other pair either has a
    /// section arm that names its conic, or has no arm at all; the
    /// second case is refused here rather than defaulting into the
    /// straight-chord test, which would mint a wrong chord silently
    /// the moment the germ-pair dispatch widens.
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
    /// precedent).
    RestZipUnsupported {
        /// The precise sub-frontier.
        what: &'static str,
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
    /// vol(∖) ≤ vol A — checked at the op gate with the exact planar
    /// `mass_properties` (the review's volume-inequality backstop,
    /// decided on the INVARIANT LANE — outside the length seam,
    /// Evan's #213 layering ruling). A certified violation is a
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

impl core::fmt::Display for BooleanError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Band(e) => write!(f, "boolean_reduce: invalid band: {e}"),
            Self::CurvedBooleanUnsupported {
                operand,
                face,
                kind,
            } => write!(
                f,
                "boolean_reduce: face {face:?} of operand {operand:?} is a {} — the \
                 classification this site required has no wired boolean arm for the \
                 kind in this build (the refusal retires one table arm at a time, \
                 never wholesale). Pairs involving this kind route per \
                 geom_brep::intersect::route; where a route is already implemented \
                 at the INTERSECTION layer (plane×NURBS), what is missing here is \
                 the boolean's own crossing layer for the kind — edge×face sweep \
                 events, curved \
                 trim containment, and the fitted chord join lane. The curved \
                 containment/pierce door covers the sphere half; the blocker is the \
                 fitted-chord join lane, which has no cyl×sphere azimuth-window \
                 analog to read",
                kind.name()
            ),
            Self::CurvedPierceUnsupported {
                operand,
                face,
                edge,
                band,
            } => write!(
                f,
                "boolean_reduce: edge {edge:?} of operand {operand:?} definitely meets \
                 curved face {face:?} away from a shared boundary (clearance classified \
                 against band [zero {:e}, escalate {:e}]) — the curved pierce door \
                 (point-in-face trim containment on a curved chart, and the ring \
                 insertion behind it) does not exist yet — this is the typed \
                 frontier of the supported envelope. The same margin one band-width \
                 away escalates as a sliver instead; {}",
                band.zero(),
                band.escalate(),
                COINCIDENCE_RECOURSE
            ),
            Self::CurvedEdgeUnsupported { operand, edge } => write!(
                f,
                "boolean_reduce: edge {edge:?} of operand {operand:?} has a rung-3 \
                 (Nurbs) carrier — refused at the operand gate: rung-3 edges are what \
                 the curved zip MINTS, not what it consumes"
            ),
            Self::PointSplitCarrierUnsupported { operand, edge } => write!(
                f,
                "boolean_reduce: edge {edge:?} of operand {operand:?} must be split at \
                 an event point, and its carrier has no exact point parameter — only a \
                 Line does. The operand gate admits Circle and Ellipse; this lane is \
                 narrower, and refuses rather than solving for the parameter"
            ),
            Self::ScaffoldingOperand { operand, edge } => write!(
                f,
                "boolean_reduce: operand {operand:?} carries null scaffolding at {edge:?} \
                 (mid-surgery body)"
            ),
            Self::NonMaximalFaces { operand, edge } => write!(
                f,
                "boolean_reduce: operand {operand:?} has coincident adjacent faces across edge \
                 {edge:?} (not maximal-faced); run merge_coplanar_faces explicitly first"
            ),
            Self::CurvedPairUnsupported {
                op,
                operand,
                face,
                kind,
                other_face,
                other_kind,
            } => write!(
                f,
                "boolean: face {face:?} of operand {operand:?} is a {} and its box MAY \
                 INTERSECT face {other_face:?} ({}) of operand {:?}{}. Box overlap is a \
                 MAY, not a DOES: both boxes are supersets of their faces, so the two \
                 may in exact geometry be disjoint — what the kernel cannot do is rule \
                 the meeting out, and it has no seam lane for the ({}, {}) germ pair if \
                 they do meet. A face of this kind whose box CLEARS the other operand \
                 does not gate the operation at all. The refusal is PER PAIR: \
                 plane×CYLINDER and plane×SPHERE subtract and intersect are live \
                 (blind and through holes, exact closed-form volumes, tier 3, both \
                 sweep strategies). What is still refused is blocked on a JOIN lane, \
                 not on revert: a cone or torus germ pair has no seam lane at all — the \
                 germ-pair dispatch wires (Plane, Cylinder) and (Plane, Sphere) only, \
                 and a cyl×sphere fitted-chord window has no window analog to read — \
                 and a NURBS face has no crossing layer. The refusal is UP FRONT and \
                 structural because the downstream failure is SILENT, not typed: with \
                 no crossings found the pipeline falls through to vertex-probed \
                 containment, and a curved face leaves the other solid between its \
                 vertices with no vertex noticing. Recourse: move the two apart, or \
                 express the cut with cylindrical or spherical tooling, or wait on the \
                 join lane",
                kind.name(),
                other_kind.name(),
                operand.other(),
                op.map_or(String::new(), |op| format!(" under {op:?}")),
                kind.name(),
                other_kind.name(),
            ),
            Self::NurbsExtentUnsupported { operand, face } => write!(
                f,
                "boolean fallback: face {face:?} of operand {operand:?} is NURBS-surfaced \
                 and the no-crossings containment fallback's curved-extent test cannot be \
                 written for the kind: implicit_residual(Nurbs) is poison, so a certified \
                 extent would have to be argued through a foot point plus a bound on how \
                 far the patch reaches past it, and no such argument has been written. \
                 Projection is not the obstacle: NurbsSurface::project is generic \
                 over any bracket-carrying scalar, so the Interval lane is \
                 available. The class is re-gated HERE, typed and pinned, so a future \
                 NURBS body constructor cannot re-open the vertex-probe silence. \
                 Recourse: write the NURBS extent test, then retire this gate per \
                 class"
            ),
            Self::FallbackExtentUnsupported {
                operand,
                face,
                what,
            } => write!(
                f,
                "boolean fallback: the curved-extent scan cannot certify the \
                 no-crossings configuration at face {face:?} of operand {operand:?}: \
                 {what}. The vertex-probed answer a curved boundary defeats is never \
                 given; refused typed instead"
            ),
            Self::GermFrameUnsupported {
                a_face,
                a_kind,
                b_face,
                b_kind,
            } => write!(
                f,
                "boolean join: the germ pair (face {a_face:?} of A, a {}; face \
                 {b_face:?} of B, a {}) has no section-frame arm, so the locus its \
                 germ line rides is unknown. A missing frame MEANS a straight locus \
                 and selects the chord facing test — an answer only a plane×plane \
                 pair earns structurally — so a pair without an arm is refused here \
                 rather than defaulted into it. Recourse: wire the pair's section \
                 arm, or express the cut with tooling whose germ pairs are wired",
                a_kind.name(),
                b_kind.name(),
            ),
            Self::Pcurves { source } => write!(
                f,
                "boolean: the result's pcurve mint pass refused (curved results carry \
                 certified per-half-edge pcurves at rest): {source}"
            ),
            Self::Escalated { diag } => write!(
                f,
                "boolean_reduce: predicate escalated ({diag}); the operand pair is \
                 ill-conditioned at this tolerance — never resolved by snapping"
            ),
            Self::UndeclaredCoincidence { diag, .. } => {
                f.write_str(
                    "boolean_reduce: geometric coincidence (exact or within tolerance) without \
                     a shared recipe source or declared intent (",
                )?;
                // The rung-4 definite arm synthesizes `MarginDiag::Invalid`
                // for a decided-zero offset (plane_eq keeps the decision
                // machinery); rendering that payload verbatim would claim a
                // poisoned margin on clean geometry. Say the honest thing
                // instead: the measure is definitely zero (S6 review,
                // MAJOR-1).
                if matches!(diag.margin, MarginDiag::Invalid) {
                    match diag.predicate {
                        Some(name) => write!(
                            f,
                            "predicate '{name}' definite: the coincidence measure is \
                             exactly zero — the geometry coincides"
                        )?,
                        None => f.write_str(
                            "the coincidence measure is exactly zero — the geometry coincides",
                        )?,
                    }
                } else {
                    write!(f, "{}", diag.payload())?;
                }
                write!(
                    f,
                    "); coincidence is structural or declared, never inferred from values — \
                     {COINCIDENCE_RECOURSE}"
                )
            }
            Self::ContactContradicted {
                declaration,
                margin,
                steer,
            } => write!(
                f,
                "boolean op: the declared {} contact between faces {:?} and {:?} is \
                 contradicted by {} — every definite verdict wins over every declaration; \
                 {}{}",
                declaration.class.name(),
                declaration.a,
                declaration.b,
                margin.payload(),
                crate::contact::CONTACT_RECOURSE,
                steer.map(|s| format!(" — {s}")).unwrap_or_default(),
            ),
            Self::DeclarationContradicted { diag } => write!(
                f,
                "boolean op: a declared coincidence contradicts the geometry ({diag}) — the \
                 declared pair's planes are definitely distinct; fix the declaration or the \
                 geometry, the op never glues a lie"
            ),
            Self::UnsupportedDeclarationClass { class } => write!(
                f,
                "boolean op: a declared contact of class {} lies outside the envelope this \
                 op's classification acts on (Rest on the plane/sphere/cylinder carrier \
                 inventory; Tangent where the closed-form witness lane reaches — \
                 plane×cylinder along a ruling, parallel cylinders) — the declaration is \
                 refused at the door rather than ignored inside",
                class.name()
            ),
            Self::InvalidDeclaration { operand, what } => write!(
                f,
                "boolean op: invalid declaration payload on operand {operand:?}: {what}"
            ),
            Self::PairingMismatch { a_vertex, b_vertex } => write!(
                f,
                "boolean_reduce: null-edge pairing mismatch at vertex pair \
                 ({a_vertex:?}, {b_vertex:?}): a surviving crossing-record pair is not \
                 cyclically adjacent in both neighborhoods (the 15.11 invariant's guarded \
                 refusal)"
            ),
            Self::ClassificationInvariant { what } => {
                write!(
                    f,
                    "boolean_reduce: classification invariant violated: {what}"
                )
            }
            Self::CorruptOperand { operand, vertex } => write!(
                f,
                "boolean_reduce: neighborhood of vertex {vertex:?} in operand {operand:?} \
                 could not be walked"
            ),
            Self::CrossingInsertion {
                operand,
                edge,
                source,
            } => write!(
                f,
                "boolean_reduce: crossing insertion refused on edge {edge:?} of operand \
                 {operand:?}: {source}"
            ),
            Self::Euler(e) => write!(f, "boolean_reduce: euler operation refused: {e}"),
            Self::Join(e) => write!(f, "boolean op: joining refused: {e}"),
            Self::RestZipUnsupported { what } => write!(
                f,
                "boolean op: declared-REST union zip: {what} — a named \
                 sub-frontier of the boundary-on-boundary REST lane (planar declared \
                 contacts whose seam splits cleanly are covered); \
                 {COINCIDENCE_RECOURSE}"
            ),
            Self::JoinDesync { what } => write!(
                f,
                "boolean op: A/B lockstep invariant violated: {what} (kernel bug or corrupt \
                 reduction)"
            ),
            Self::TornComponent { operand, shell } => write!(
                f,
                "boolean op: component {shell:?} of operand {operand:?} carries section faces \
                 of both sides (kernel bug)"
            ),
            Self::Containment(e) => write!(f, "boolean op: containment fallback: {e}"),
            Self::Revert(e) => write!(f, "boolean op: revert of the ∖ B side refused: {e}"),
            Self::SeamOrientation { a_face, b_face } => write!(
                f,
                "boolean op: seam cycles of faces {a_face:?}/{b_face:?} are not antiparallel \
                 (orientation chain broke — kernel bug)"
            ),
            Self::ZipCorrespondence { what } => write!(
                f,
                "boolean op: seam zip correspondence failed: {what} (kernel bug)"
            ),
            Self::Merge(e) => write!(f, "boolean op: coplanar-merge output stage refused: {e}"),
            Self::ResultInvalid { errors } => write!(
                f,
                "boolean op: finished result failed a tier gate ({} finding(s), first: {:?}) — \
                 kernel bug, no invalid body is returned",
                errors.len(),
                errors.first()
            ),
            Self::ResultVolumeImplausible { which, got, bound } => write!(
                f,
                "boolean op: kernel invariant violated — this is a bug in the kernel, not in \
                 your geometry: {which} failed (got {got}, bound {bound}); no such body is \
                 returned. Please report it, with the model that produced it"
            ),
            Self::UnrepresentableResult => write!(
                f,
                "boolean op: the result would be unbounded (complement operands) — no boundary \
                 representation exists"
            ),
            Self::GraftRecertify(e) => write!(
                f,
                "boolean op: grafted edge description failed re-certification: {e}"
            ),
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
    reduce::gate_operand_pairs(a_operand, b_operand, band)?;
    reduce::gate_maximal_faces(a_operand, Operand::A, band)?;
    reduce::gate_maximal_faces(b_operand, Operand::B, band)?;

    let mut a = a_operand.clone();
    let mut b = b_operand.clone();
    let mut acc = reduce::ContactAcc::default();
    // The suite's door takes no declarations: the traced sweep runs
    // the undeclared posture, where the frontier doors are verbatim.
    let declared = DeclaredPairs::default();
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
    verify_declared_contacts(a_operand, b_operand, decls, band)?;
    let declared = DeclaredPairs::build(decls);
    reduce::gate_operand_pairs(a_operand, b_operand, band)?;
    reduce::gate_maximal_faces(a_operand, Operand::A, band)?;
    reduce::gate_maximal_faces(b_operand, Operand::B, band)?;

    let mut a = a_operand.clone();
    let mut b = b_operand.clone();

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
        let out =
            insert::insert_null_pairs(&mut a, &mut b, c, &a_sectors, &b_sectors, &records, band)?;
        null_edges.extend(out.edges);
        null_pairs.extend(out.pairs);
    }

    Ok(BooleanReduction {
        op,
        a,
        b,
        contacts,
        null_edges,
        null_pairs,
        pierce_rings,
    })
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
/// Both Same± verdicts pass: this door verifies the CARRIER claim (the
/// classification's own question), and aligned coincidence is the
/// merge stage's legitimate flush-wall answer. Refusing containment is
/// the contact record's job, one level up.
fn verify_declared_contacts<T: Decide>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
    band: Band,
) -> Result<(), BooleanError> {
    for &FacePairDeclaration {
        a: fa,
        b: fb,
        class,
    } in &decls.coincident_faces
    {
        match class {
            ContactClass::Rest => verify_rest_declaration(a, fa, b, fb, band)?,
            ContactClass::Tangent => verify_tangent_declaration(a, fa, b, fb, band)?,
        }
    }
    Ok(())
}

/// The `Rest` half of [`verify_declared_contacts`]: the carrier
/// ladder in its declared posture — a definitely-different carrier
/// contradicts, an in-band residue is bridged (C4), a sliver
/// escalates.
fn verify_rest_declaration<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<(), BooleanError> {
    // A carrier kind the ladder cannot describe: `validate_
    // declarations` has already had its say about which kinds this
    // op accepts, so there is nothing left to add here.
    let Some(outcome) = rest::carrier_pair_relation(a, fa, b, fb, true, band) else {
        return Ok(());
    };
    match outcome {
        Ok(_) => Ok(()),
        Err(carrier_eq::CarrierEqError::Contradicted(diag)) => {
            Err(BooleanError::ContactContradicted {
                declaration: crate::contact::DeclaredContact {
                    a: fa,
                    b: fb,
                    class: ContactClass::Rest,
                },
                steer: contact_verify::fit_steer(&diag),
                margin: diag,
            })
        }
        Err(carrier_eq::CarrierEqError::Escalated(diag)) => Err(BooleanError::Escalated { diag }),
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

/// The `Tangent` half of [`verify_declared_contacts`] — admitted
/// exactly where the DEV-1 closed-form witness lane reaches
/// ([`rest::tangent_locus`]: plane×cylinder along a ruling, parallel
/// cylinders), and refused typed everywhere else:
///
/// 1. **The conformal screen.** The carrier ladder runs first in its
///    DETECTOR posture: a pair it can call one carrier — structurally
///    (rung 1) or geometrically (rung 4's coincidence refusal) — is
///    `Rest`-shaped, and a `Tangent` claim on a conformal pair is
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
                    margin: Indeterminate {
                        margin: MarginDiag::Invalid,
                        band,
                        predicate: Some("contact_tangent_conformal"),
                    },
                });
            }
            // One carrier, geometrically (the detector's coincidence
            // refusal): the diag carries the margins that decided.
            Err(carrier_eq::CarrierEqError::Undeclared { diag, .. }) => {
                return Err(BooleanError::ContactContradicted {
                    declaration,
                    steer: None,
                    margin: diag,
                });
            }
            Err(carrier_eq::CarrierEqError::Escalated(diag)) => {
                return Err(BooleanError::Escalated { diag });
            }
            // Unreachable with `declared: false`; refuse loudly anyway.
            Err(carrier_eq::CarrierEqError::Contradicted(diag)) => {
                return Err(BooleanError::Escalated { diag });
            }
        }
    }
    // 2. The DEV-1 witness locus.
    let surface_of = |body: &Body<T>, f: FaceKey, operand| {
        body.get_face(f)
            .and_then(|face| body.get_surface(face.surface))
            .cloned()
            .ok_or(BooleanError::InvalidDeclaration {
                operand,
                what: "declared face lost its surface",
            })
    };
    let sa = surface_of(a, fa, Operand::A)?;
    let sb = surface_of(b, fb, Operand::B)?;
    let (origin, dir) = match rest::tangent_locus(&sa, &sb, band) {
        Ok(rest::TangentLocus::Line { origin, dir }) => (origin, dir),
        Err(rest::TangentLocusError::Escalated(diag)) => {
            return Err(BooleanError::Escalated { diag });
        }
        Err(rest::TangentLocusError::NotTangent { .. }) => {
            return Err(BooleanError::ContactContradicted {
                declaration,
                steer: None,
                margin: Indeterminate {
                    margin: MarginDiag::Invalid,
                    band,
                    predicate: Some("tangent_locus_gap"),
                },
            });
        }
        Err(rest::TangentLocusError::Unsupported { .. }) => {
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
                margin: diag,
            })
        }
        Err(crate::contact::ContactRefusal::Escalated { diag })
        | Err(crate::contact::ContactRefusal::Undeclared { diag }) => {
            Err(BooleanError::Escalated { diag })
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
    // carrier the classification's certified ladder describes —
    // plane, sphere or cylinder (`carrier_eq`'s inventory). Kinds
    // outside it (cone, torus, NURBS) refuse typed at this door;
    // undeclared touching refuses forever at the classification
    // frontiers — the door only widens what a VERIFIED declaration
    // can unlock. Per-class geometric admission (the `Tangent`
    // witness lane) is `verify_declared_contacts`' half of the door.
    let inventory_face = |body: &Body<T>, f: FaceKey, operand| -> Result<(), BooleanError> {
        let face = body
            .get_face(f)
            .ok_or_else(|| bad(operand, "declared face key does not resolve"))?;
        match body.get_surface(face.surface) {
            Some(
                geom::Surface::Plane { .. }
                | geom::Surface::Sphere { .. }
                | geom::Surface::Cylinder { .. },
            ) => Ok(()),
            Some(_) => Err(bad(
                operand,
                "declared face's carrier is outside the certified inventory \
                 (plane, sphere, cylinder)",
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

    /// S6 (two-tolerance, D4 ¶1 addendum): the boolean coincidence
    /// pair — `UndeclaredCoincidence` (exactly-on OR in-band, per the
    /// plane-identity rung 4) and `Escalated` (in-band elsewhere) —
    /// carries the shared recourse fragment, exactly once per message.
    #[test]
    fn coincidence_pair_carries_the_shared_recourse_once() {
        let diag = |margin| Indeterminate {
            margin,
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some("bool_plane_offset"),
        };
        // The escalated arm: recourse rides the Indeterminate carrier —
        // for every margin shape, including Invalid (the reachable
        // bool_plane_orient Zero path synthesizes one; S6 review,
        // MINOR-1).
        for margin in [MarginDiag::Value(5e-9), MarginDiag::Invalid] {
            let msg = BooleanError::Escalated { diag: diag(margin) }.to_string();
            assert_eq!(msg.matches(COINCIDENCE_RECOURSE).count(), 1, "{msg}");
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
        for margin in [MarginDiag::Invalid, MarginDiag::Value(5e-9)] {
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
            diag: diag(MarginDiag::Invalid),
            pair,
            relation: PlaneRelation::SameOpposite,
        }
        .to_string();
        assert!(msg.contains("exactly zero"), "{msg}");
        assert!(!msg.contains("margin is invalid"), "{msg}");
    }

    /// The M5 S1 sub-frontier refusal follows the two-tolerance
    /// message shape: it names the lane and the precise sub-frontier
    /// and composes the shared recourse exactly once.
    #[test]
    fn rest_zip_unsupported_carries_the_shared_recourse_once() {
        let msg = BooleanError::RestZipUnsupported {
            what: "contact patch face carries rings",
        }
        .to_string();
        assert_eq!(msg.matches(COINCIDENCE_RECOURSE).count(), 1, "{msg}");
        assert!(msg.contains("contact patch face carries rings"), "{msg}");
        assert!(msg.contains("declared-REST union zip"), "{msg}");
    }
}
