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
mod combine;
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
pub(crate) mod vtxfac;
mod zip;

use geom_core::{
    Band, BandError, Bounds, COINCIDENCE_RECOURSE, Decide, Indeterminate, MarginDiag, Real,
};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, ShellKey, VertexKey};
use crate::euler::EulerOpError;
use crate::merge_faces::MergeCoplanarError;
use crate::revert::RevertError;
use crate::splitting::join::SplitJoinError;
use crate::validate::ValidationError;

pub use contain::{ContainError, FaceContainment, contfp};
pub use join::CompletedPolygonPair;
pub use ops::{
    BooleanBody, BooleanNaming, BooleanResult, BooleanResultKind, OperandKeys, boolean_op_with,
    intersect, intersect_with, subtract, subtract_with, union, union_with,
};
pub use plane_eq::{PlaneDesc, PlaneEqError, PlaneIdentity, PlaneRelation, oriented_plane_eq};
#[cfg(feature = "sweep-testing")]
pub use reduce::PlantedDegradation;
pub use reduce::{SweepStrategy, SweepTrace};
pub use solid_contain::{PointInSolidError, SolidContainment, point_in_solid};

/// Which regularized boolean is being computed — threaded through the
/// classifier because on-case lumping (Eq. 15.3) is op-dependent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// The three ON-sets as **declared-contact records** (F1/F2): emitted
/// by the pipeline as it discovers each contact — these are the future
/// tier-3′ declarations. Deterministic discovery order, deduplicated.
#[derive(Clone, Debug, Default)]
pub struct ContactRecords {
    /// Coincident vertex pairs (`sonvv`).
    pub vv: Vec<VvContact>,
    /// Vertices of A on faces of B (`sonva`).
    pub a_on_b: Vec<VfContact>,
    /// Vertices of B on faces of A (`sonvb`).
    pub b_on_a: Vec<VfContact>,
}

/// Operand-internal contact records carried by recipe intent (F5, M4
/// PR 5): coincidences WITHIN one operand — typically a reused 3′
/// body's surviving declarations — re-entering this op as declared
/// data. Keys are that operand's; the op remaps survivors into result
/// keys (same strict drop rule as discovered records: a record whose
/// entity was consumed drops).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CarriedContacts {
    /// Coincident vertex pairs within the operand.
    pub vv: Vec<VvContact>,
    /// Vertex-on-face rests within the operand.
    pub vf: Vec<VfContact>,
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BooleanDeclarations {
    /// Cross-operand coincident-plane face pairs `(A face, B face)`:
    /// classification treats each pair's planes as the same plane
    /// (orientation decided, contradiction refused —
    /// `plane_eq` rung 2), and the result's merge stage glues the
    /// pair's surviving coplanar-adjacent material (N3 `Merged`).
    pub coincident_faces: Vec<(FaceKey, FaceKey)>,
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

/// The classification stages' symmetric declared-face-pair index
/// (crate-internal): normalized `(A face, B face)` rows.
#[derive(Debug, Default)]
pub(crate) struct DeclaredPairs {
    set: std::collections::BTreeSet<(FaceKey, FaceKey)>,
}

impl DeclaredPairs {
    pub(crate) fn build(decls: &BooleanDeclarations) -> Self {
        Self {
            set: decls.coincident_faces.iter().copied().collect(),
        }
    }

    /// Whether the (operand-tagged) face pair is declared coincident.
    /// Same-operand pairs are never declared here (operand-internal
    /// coplanarity is the producing op's merge, not this op's).
    pub(crate) fn contains(&self, o1: Operand, f1: FaceKey, o2: Operand, f2: FaceKey) -> bool {
        match (o1, o2) {
            (Operand::A, Operand::B) => self.set.contains(&(f1, f2)),
            (Operand::B, Operand::A) => self.set.contains(&(f2, f1)),
            _ => false,
        }
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
    /// An edge carrier is not a `Line` (F5).
    CurvedEdgeUnsupported {
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
    UndeclaredCoincidence {
        /// The escalation site's diagnostics.
        diag: Indeterminate,
    },
    /// A declared coincidence contradicts the geometry (the declared
    /// pair's planes are definitely distinct) — the recipe's intent
    /// cannot be realized; refused loudly, never glued (M4 PR 5).
    DeclarationContradicted {
        /// The contradicting predicate's diagnostics.
        diag: Indeterminate,
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
    /// Subtract/intersect met a CURVED operand (M5 PR 9): these ops
    /// route regions through `revert` (A∖B ≡ A∩revert(B), §15.9), and
    /// the revert lane — curved-surface orientation flips plus the
    /// pcurve re-mint behind them — is planar-only in this build.
    /// M5 PR 9c EXECUTED that lane and returned it as a ratified-
    /// representation question rather than an implementation task: an
    /// orientation-reversed cylinder, cone or torus has no
    /// representation to write down at all (the chart normal is odd in
    /// the radius, so it is outward for either sign), and a reversed
    /// SPHERE is representable only as the negative-radius sphere the
    /// `radius > 0` convention rejects and the `2r`-metered consumers
    /// invert — `RevertError::UnsupportedSurface` carries the scoped
    /// proof and the review's executed probe.
    ///
    /// **That question is now RATIFIED and landed** (M5 S10,
    /// 2026-08-02): option (a), [`crate::entity::Face::sense`], with
    /// the outward-normal consumer audit threaded behind it. S10 stopped
    /// at the contract plus the audit, so this refusal outlives it by
    /// exactly one unit: wiring `revert` to flip the bit is the
    /// immediately following unit, and THAT is what retires this
    /// variant's reachability for the analytic surfaces. UNION stays
    /// the live curved boolean meanwhile, and M5 PR 12's die pips stay
    /// gated on the wiring unit rather than on a design question. A
    /// front-door refusal: no reduction work happens first.
    CurvedOpUnsupported {
        /// The refused op (never `Union`).
        op: BooleanOp,
        /// The operand carrying the curved face.
        operand: Operand,
        /// The first curved face met (face-arena order).
        face: FaceKey,
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
    /// `mass_properties` (the review's volume-inequality backstop). A
    /// certified violation is a wrong-component kernel bug surfaced
    /// loudly, never a panic.
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
                 kind in this build (the refusal retires per C5 table arm, never \
                 wholesale — C12.1). Pairs involving this kind route per \
                 geom_brep::intersect::route; where a route is already implemented \
                 at the INTERSECTION layer (plane×NURBS since PR 7b), what is \
                 missing here is the boolean's own crossing layer for the kind — \
                 edge×face sweep events, curved trim containment, and the fitted \
                 chord join lane. M5 PR 9c landed the SPHERE half of the curved \
                 containment/pierce door and reported the fitted-chord join lane \
                 still open behind Pcurve::Fitted, whose certification envelope \
                 needs the SSI enclosure stack lifted off f64 (M5-LOG PR 9c, \
                 deviations 1-2)",
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
                 insertion behind it) does not exist yet: the M5 envelope's typed \
                 frontier. The same margin one band-width away escalates as a sliver \
                 instead (F6); {}",
                band.zero(),
                band.escalate(),
                COINCIDENCE_RECOURSE
            ),
            Self::CurvedEdgeUnsupported { operand, edge } => write!(
                f,
                "boolean_reduce: edge {edge:?} of operand {operand:?} has a rung-3 \
                 (Nurbs) carrier — rung-3 INPUT operands are outside the M5 envelope \
                 (rung-3 edges are what the curved zip MINTS, not what it consumes)"
            ),
            Self::ScaffoldingOperand { operand, edge } => write!(
                f,
                "boolean_reduce: operand {operand:?} carries null scaffolding at {edge:?} \
                 (mid-surgery body)"
            ),
            Self::NonMaximalFaces { operand, edge } => write!(
                f,
                "boolean_reduce: operand {operand:?} has coincident adjacent faces across edge \
                 {edge:?} (not maximal-faced, F7); run merge_coplanar_faces explicitly first"
            ),
            Self::CurvedOpUnsupported { op, operand, face } => write!(
                f,
                "boolean: {op:?} on a curved operand (operand {operand:?}, first curved \
                 face {face:?}) is not wired in this build — subtract/intersect route \
                 regions through revert (A∖B ≡ A∩revert(B)), and the curved revert \
                 lane (curved-surface orientation flips + the pcurve re-mint) is \
                 planar-only, and M5 PR 9c executed it and found no representation \
                 for a reversed curved surface that this build may write: outward \
                 for either sign of the radius on the cylinder/cone/torus, and on \
                 the sphere only the convention-violating negative radius (see \
                 RevertError's scoped proof) — closing it is a ratified \
                 representation change, not an implementation task, so PR 12's die \
                 pips stay gated on it. UNION is the live curved boolean; split the \
                 work as unions meanwhile"
            ),
            Self::Pcurves { source } => write!(
                f,
                "boolean: the result's pcurve mint pass refused (curved results carry \
                 certified per-half-edge pcurves at rest, M5 PR 9): {source}"
            ),
            Self::Escalated { diag } => write!(
                f,
                "boolean_reduce: predicate escalated ({diag}); the operand pair is \
                 ill-conditioned at this tolerance — never resolved by snapping"
            ),
            Self::UndeclaredCoincidence { diag } => {
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
            Self::DeclarationContradicted { diag } => write!(
                f,
                "boolean op: a declared coincidence contradicts the geometry ({diag}) — the \
                 declared pair's planes are definitely distinct; fix the declaration or the \
                 geometry, the op never glues a lie"
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
                 refusal, F12)"
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
                "boolean op: declared-REST union zip (M5 S1): {what} — a named \
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
                "boolean op: result volume implausible — {which} violated (got {got}, bound \
                 {bound}) — wrong-component kernel bug, no such body is returned"
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
) -> Result<BooleanReduction<T>, BooleanError> {
    boolean_reduce_declared(op, a_operand, b_operand, &BooleanDeclarations::none())
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
) -> Result<BooleanReduction<T>, BooleanError> {
    boolean_reduce_declared_strategy(op, a_operand, b_operand, decls, SweepStrategy::Realized)
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
) -> Result<(SweepTrace, SweepTrace), BooleanError> {
    sweep_traces_with_pad(a_operand, b_operand, strategy, plant, None)
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
) -> Result<(SweepTrace, SweepTrace), BooleanError> {
    let band = Band::linear()?;
    reduce::gate_planar(a_operand, Operand::A)?;
    reduce::gate_planar(b_operand, Operand::B)?;
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
        &mut acc,
        band,
        strategy,
        &ab_knobs,
        Some(&mut ab),
    )?;
    reduce::sweep_direction(
        &mut b,
        &mut a,
        Operand::B,
        &mut acc,
        band,
        strategy,
        &ba_knobs,
        Some(&mut ba),
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
) -> Result<BooleanReduction<T>, BooleanError> {
    let band = Band::linear()?;
    validate_declarations(a_operand, b_operand, decls)?;
    let declared = DeclaredPairs::build(decls);
    reduce::gate_planar(a_operand, Operand::A)?;
    reduce::gate_planar(b_operand, Operand::B)?;
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
        &mut acc,
        band,
        strategy,
        &knobs,
        None,
    )?;
    reduce::sweep_direction(
        &mut b,
        &mut a,
        Operand::B,
        &mut acc,
        band,
        strategy,
        &knobs,
        None,
    )?;
    let contacts = acc.finish();

    let mut null_edges = Vec::new();
    let mut null_pairs = Vec::new();
    let mut pierce_rings = Vec::new();

    // Vertex-on-face classification (sonva then sonvb, as 15.5).
    for &c in &contacts.a_on_b {
        let out =
            vtxfac::classify_vertex_on_face(&mut a, &mut b, Operand::A, c, op, &declared, band)?;
        null_edges.extend(out.edges);
        null_pairs.extend(out.pairs);
        pierce_rings.extend(out.ring);
    }
    for &c in &contacts.b_on_a {
        let out =
            vtxfac::classify_vertex_on_face(&mut b, &mut a, Operand::B, c, op, &declared, band)?;
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
/// operand, and declared faces must be planes. A dangling declaration
/// is a caller bug refused before any classification runs — never a
/// silent drop (F5's no-silent-drop contract).
fn validate_declarations<T: Decide>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
) -> Result<(), BooleanError> {
    let bad = |operand, what| BooleanError::InvalidDeclaration { operand, what };
    let planar_face = |body: &Body<T>, f: FaceKey, operand| -> Result<(), BooleanError> {
        let face = body
            .get_face(f)
            .ok_or_else(|| bad(operand, "declared face key does not resolve"))?;
        match body.get_surface(face.surface) {
            Some(geom_surfaces::Surface::Plane { .. }) => Ok(()),
            Some(_) => Err(bad(operand, "declared face is not a plane")),
            None => Err(bad(operand, "declared face lost its surface")),
        }
    };
    for &(fa, fb) in &decls.coincident_faces {
        planar_face(a, fa, Operand::A)?;
        planar_face(b, fb, Operand::B)?;
    }
    let carried = |body: &Body<T>, c: &CarriedContacts, operand| -> Result<(), BooleanError> {
        for pair in &c.vv {
            if body.get_vertex(pair.a).is_none() || body.get_vertex(pair.b).is_none() {
                return Err(bad(operand, "carried v-v vertex key does not resolve"));
            }
            if pair.a == pair.b {
                return Err(bad(operand, "carried v-v pair names one vertex twice"));
            }
        }
        for rest in &c.vf {
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
        for margin in [MarginDiag::Invalid, MarginDiag::Value(5e-9)] {
            let msg = BooleanError::UndeclaredCoincidence { diag: diag(margin) }.to_string();
            assert_eq!(msg.matches(COINCIDENCE_RECOURSE).count(), 1, "{msg}");
        }
        // The synthesized-Invalid definite arm renders the honest
        // statement, never the poisoned-margin text (S6 review,
        // MAJOR-1).
        let msg = BooleanError::UndeclaredCoincidence {
            diag: diag(MarginDiag::Invalid),
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
        assert!(msg.contains("M5 S1"), "{msg}");
    }
}
