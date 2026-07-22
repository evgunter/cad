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
//! 1. **Gates**: planar-only (F5, [`BooleanError::CurvedBooleanUnsupported`]);
//!    no scaffolding operands; **maximal faces (F7)** via the
//!    coincidence ladder — adjacent faces sharing a surface key or with
//!    bit-equal oriented planes ([`plane_eq`]) refuse as
//!    [`BooleanError::NonMaximalFaces`]; *numeric* coplanarity never
//!    triggers the precondition (it is not coincidence; if it bites, it
//!    bites later as a typed escalation — the ladder's honest shape).
//! 2. **Reduction sweep** (`reduce`): all-pairs edge×face, BOTH
//!    directions, quadratic (documented; BVH later per PERF-PLAN),
//!    with `contfv`/`contfp` as typed trilean case codes. Proper
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

mod combine;
mod contain;
mod finish;
pub(crate) mod insert;
mod join;
mod ops;
pub mod plane_eq;
pub(crate) mod recl;
pub(crate) mod reduce;
pub(crate) mod sectors;
pub mod solid_contain;
pub mod tables;
pub(crate) mod vtxfac;
mod zip;

use geom_core::{Band, BandError, Decide, Indeterminate, Real};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, ShellKey, VertexKey};
use crate::euler::EulerOpError;
use crate::merge_faces::MergeCoplanarError;
use crate::revert::RevertError;
use crate::splitting::join::SplitJoinError;
use crate::validate::ValidationError;

pub use contain::{FaceContainment, contfp};
pub use join::CompletedPolygonPair;
pub use ops::{BooleanBody, BooleanResult, BooleanResultKind, intersect, subtract, union};
pub use plane_eq::{PlaneDesc, PlaneEqError, PlaneRelation, oriented_plane_eq};
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
    /// A face of `operand` is not a `Plane` — M3 booleans are
    /// planar-only (F5).
    CurvedBooleanUnsupported {
        /// The offending operand and face.
        operand: Operand,
        /// The face.
        face: FaceKey,
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
    /// declared (bit-equal) description backing the coincidence (F6):
    /// near-coincidence NEVER silently becomes contact.
    UndeclaredCoincidence {
        /// The escalation site's diagnostics.
        diag: Indeterminate,
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
    /// An underlying Euler operation refused.
    Euler(EulerOpError),
    /// The joining stage's chord machinery refused (PR 5; nested
    /// whole — includes `UnpairedLooseEnds` and `SectionLoopMixed`).
    Join(SplitJoinError),
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
            Self::CurvedBooleanUnsupported { operand, face } => write!(
                f,
                "boolean_reduce: face {face:?} of operand {operand:?} is not planar — M3 \
                 booleans are planar-only (curved intersections arrive with M5 SSI)"
            ),
            Self::CurvedEdgeUnsupported { operand, edge } => write!(
                f,
                "boolean_reduce: edge {edge:?} of operand {operand:?} has a non-line carrier \
                 (planar-only, M5)"
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
            Self::Escalated { diag } => write!(
                f,
                "boolean_reduce: predicate escalated ({diag}); the operand pair is \
                 ill-conditioned at this tolerance — resolve by explicit repair/adoption, \
                 never by snapping"
            ),
            Self::UndeclaredCoincidence { diag } => write!(
                f,
                "boolean_reduce: geometric near-coincidence without a declared (bit-equal) \
                 description ({diag}); coincidence is structural or declared, never inferred \
                 from values — declare the relation or repair the geometry"
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
pub fn boolean_reduce<T: Decide>(
    op: BooleanOp,
    a_operand: &Body<T>,
    b_operand: &Body<T>,
) -> Result<BooleanReduction<T>, BooleanError> {
    let band = Band::linear()?;
    reduce::gate_planar(a_operand, Operand::A)?;
    reduce::gate_planar(b_operand, Operand::B)?;
    reduce::gate_maximal_faces(a_operand, Operand::A, band)?;
    reduce::gate_maximal_faces(b_operand, Operand::B, band)?;

    let mut a = a_operand.clone();
    let mut b = b_operand.clone();

    // Reduction sweep, both directions (A's edges first — D9 order).
    let mut acc = reduce::ContactAcc::default();
    reduce::sweep_direction(&mut a, &mut b, Operand::A, &mut acc, band)?;
    reduce::sweep_direction(&mut b, &mut a, Operand::B, &mut acc, band)?;
    let contacts = acc.finish();

    let mut null_edges = Vec::new();
    let mut null_pairs = Vec::new();
    let mut pierce_rings = Vec::new();

    // Vertex-on-face classification (sonva then sonvb, as 15.5).
    for &c in &contacts.a_on_b {
        let out = vtxfac::classify_vertex_on_face(&mut a, &mut b, Operand::A, c, op, band)?;
        null_edges.extend(out.edges);
        null_pairs.extend(out.pairs);
        pierce_rings.extend(out.ring);
    }
    for &c in &contacts.b_on_a {
        let out = vtxfac::classify_vertex_on_face(&mut b, &mut a, Operand::B, c, op, band)?;
        null_edges.extend(out.edges);
        null_pairs.extend(out.pairs);
        pierce_rings.extend(out.ring);
    }

    // Vertex-vertex classification.
    for &c in &contacts.vv {
        let a_sectors = sectors::build_sectors(&a, Operand::A, c.a, band)?;
        let b_sectors = sectors::build_sectors(&b, Operand::B, c.b, band)?;
        let mut records = sectors::pair_search(&a_sectors, &b_sectors, band)?;
        recl::recl_sectors(&mut records, &a_sectors, &b_sectors, &a, &b, op, band)?;
        recl::recl_edges(&mut records, &a_sectors, &b_sectors, &a, &b, op, band)?;
        let out = insert::insert_null_pairs(&mut a, &mut b, c, &a_sectors, &b_sectors, &records, band)?;
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
