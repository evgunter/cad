//! `replace_face_offset` — the face-replacement primitive: one face's
//! surface becomes its certified offset, and the face's own boundary is
//! re-described against the moved chart.
//!
//! The **offset** of a surface `S` at signed distance `d` is the normal
//! pushforward `S_d(u, v) = S(u, v) + d·n(u, v)` along the stored chart
//! normal (`geom_brep::offset_surface`'s definition, unchanged here).
//! Positive `d` moves along that normal; the face's `sense` bit takes no
//! part, because the offset is a statement about the SURFACE, not about
//! which side of it carries material.
//!
//! # What moves and what does not
//!
//! **The neighbours' surfaces are untouched.** Only the named face's
//! surface is replaced; every other face keeps the chart it had.
//!
//! **That premise is this door's, not the verb's**, and it is what
//! bounds the door: composing it over a whole boundary transports each
//! corner once per chart, which is the offset body's corner only where
//! the normals are mutually perpendicular. At an OBLIQUE junction it is
//! not, and [`ReplaceFaceError::ReanchorOffCarrier`] is what refuses to
//! build the difference (#1081). A body whose faces are ALL PLANES has
//! another route — [`crate::offset_planes_together`], which moves every
//! chart at once and solves each corner simultaneously; this door is
//! what everything else still takes.
//!
//! What must then be re-derived is everything the replaced chart
//! carries:
//!
//! - the face's boundary edges' carriers and descriptions,
//! - the points of the vertices those edges end at,
//! - the parameter range of every edge that ends at one of those
//!   vertices without lying on the face's boundary — its carrier is
//!   unchanged, because the surfaces holding it did not move; only
//!   where the edge stops did.
//!
//! # The carrier lanes
//!
//! An edge on the boundary lies ON the replaced surface, so the offset
//! ACTS on its carrier. That action is closed-form per (surface kind,
//! carrier kind) and this module states it once, in [`transport_curve`]:
//!
//! - **Plane** — a rigid translation by `d·normal`; every carrier kind
//!   transports, and the translation is exact.
//! - **Cylinder** — a rigid translation of a chart LINE (or a spline
//!   lying along one) by `d·radial`, and a radius update for a coaxial
//!   circle.
//! - **Cone** — `geom_brep::ConeOffset`'s action, read rather than
//!   re-derived: a generator translates by the action's own pointwise
//!   displacement, and a parallel re-mints at the shifted `v` about the
//!   action's own SLID apex. The mint and this door therefore cannot
//!   drift; they are one derivation with three faces.
//! - **Sphere** — the homothety of ratio `(R + d)/R` about the centre.
//! - **Torus** — a tube-radius update, either on a meridian (tube)
//!   circle or on a parallel.
//! - **NURBS** (the `Approx` mint's operand) — a translation by `d·n₀`,
//!   `n₀` the chart normal at the domain midpoint. This lane is exact
//!   only where the chart normal is constant; elsewhere it costs
//!   `d·|n − n₀|`, which is the quantity the run's band classifies at
//!   `set_edge_curve`. The door does not pre-empt that classification —
//!   the certified gate is the honest meter, and a face whose budget is
//!   spent refuses there by name.
//!
//! **On the mirror nappe the cone's action is not the per-point chart
//! normal, and that is the mint's contract rather than this door's
//! choice.** The pushforward follows the CONTINUOUS EXTENSION of the
//! opening nappe's normal field, which is what makes the action a pure
//! parameter shift; following the per-point normal would split the
//! double cone. So a `v < 0` face's material moves `−d` along its own
//! chart normal, and the door does not refuse the nappe.
//!
//! **The translating lanes accept a spline carrier**, not only a line:
//! a translated control net is exact structure, so a `Curve3::Nurbs`
//! on a plane, a cylinder ruling, a cone generator or a fitted chart
//! transports with everything else. What the lane does NOT claim is
//! that the transported spline still lies on the untouched surface its
//! description may name — the attach layer's certification is the net
//! for that, and it is the honest one, because the residual it measures
//! is exactly the quantity in question.
//!
//! An `IsoCurve` on a NURBS chart takes a different, exact route: its
//! carrier is the FIT's own boundary row (`geom_brep::nurbs_iso`), which
//! lands the carrier in the fit's spline space — the degree AND the
//! refined interior knots — by construction rather than by elevating and
//! refining the old carrier into it.
//!
//! # Where it refuses
//!
//! The C5 table is the boundary: an intrinsic description whose pair
//! `(new kind, neighbour kind)` has no route arm cannot be re-stated,
//! and the door refuses naming the pair rather than storing a
//! description nothing can certify. `Approx × anything` has no arm, so a
//! fitted face's intrinsically-described boundary is exactly where this
//! door stops.
//!
//! # The apex window
//!
//! Offsetting a cone shifts its `v` parameterization by `d·cot α`
//! (`geom_brep::offset`'s derivation). A face bounded on the opening
//! nappe by `v ≥ v_min` therefore lands on `v ≥ v_min + d·cot α`, and a
//! window that crosses zero images the MIRROR nappe — geometry the
//! offset mint is right to produce and this door must not silently call
//! a face's offset. The margined predicate `offset_apex_window` decides
//! `inf(v-window) + d·cot α` before anything is minted — and its mirror
//! `−(sup(v-window) + d·cot α)` on the OTHER nappe, which is not an
//! extra case but the same statement: revolve aims every cone's chart
//! axis at `+a₃`, so a downward-opening cone sweeps `v < 0` and its
//! window's near end is its supremum. A window that already reaches the
//! apex has no single nappe to offset and refuses on the same variant.
//!
//! # Discipline
//!
//! Every decision — the mint, the refusals, the whole boundary plan —
//! runs read-only against the incoming body. Mutation then runs on a
//! clone in the attach layer's order (surface, then edge descriptions,
//! then the whole-body pcurve mint), the clone is validated once, and
//! only a valid clone is adopted. The body is untouched on every `Err`.

use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeGeometry, SurfaceKind};
use geom_core::k_stats::decide;
use geom_core::{Affine3, Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Tol, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, VertexKey};
use crate::euler::{EulerOpError, FaceSurface};
use crate::geometry::SurfaceKey;
use crate::pcurves::{PcurveMintError, mint_pcurves};
use crate::props::PropsQuadLane;
use crate::validate::{ValidationError, validate_closed};

/// Typed refusal of the face-replacement door. Scalar payloads echo the
/// classified quantity's ingredients — data, not a decision (the offset
/// door's echo convention, one layer up).
#[derive(Clone, Debug)]
pub enum ReplaceFaceError<T: Real> {
    /// `face` does not resolve in the body.
    StaleFace {
        /// The unresolvable face.
        face: FaceKey,
    },
    /// The body's own referential coherence broke mid-plan (a key that
    /// resolved once did not resolve again) — a kernel bug, surfaced
    /// rather than swallowed.
    Corrupt,
    /// The analytic offset mint refused: the radius floor, the torus
    /// ring convention, non-closure, or an escalation.
    Offset {
        /// The face whose surface refused.
        face: FaceKey,
        /// The geometry door's typed refusal, verbatim.
        error: geom_brep::OffsetError<T>,
    },
    /// The approximating-surface fit refused: the regularity or
    /// collapse meter, a rational operand, the refinement budget, or a
    /// certificate limb.
    Fit {
        /// The face whose fit refused.
        face: FaceKey,
        /// The fit door's typed refusal, verbatim.
        error: geom_brep::OffsetFitError,
    },
    /// The face carries a NURBS surface but this scalar has no fit
    /// lane, so the offset cannot be minted at all. Not a pass — the
    /// same posture tier 3 takes on an unre-derivable certificate.
    ApproxLaneUnsupported {
        /// The face whose kind needs the (`f64`-only) fit lane.
        face: FaceKey,
    },
    /// **The operand's surface key is SHARED.** Another face carries
    /// the same surface, so replacing this face would re-point the
    /// boundary's descriptions at the fresh key while the sharer keeps
    /// the old chart — the shared seam would name one face's surface
    /// and lie on the other's. Replacing a shared chart is a
    /// multi-face operation and this door is one face wide.
    SharedSurfaceKey {
        /// The face the door was called on.
        face: FaceKey,
        /// One other face carrying the same surface key.
        other: FaceKey,
    },
    /// No face was named.
    EmptyGroup,
    /// The named faces do not all carry ONE chart, so there is no
    /// single surface to offset.
    GroupChartsDiffer {
        /// The first named face.
        face: FaceKey,
        /// The first face carrying a different surface.
        other: FaceKey,
    },
    /// The face carries the "not yet described" placeholder surface,
    /// which has no locus to offset.
    PlaceholderSurface {
        /// The face carrying the placeholder.
        face: FaceKey,
    },
    /// **The apex-window predicate.** The face's `v`-window, shifted by
    /// the cone offset's `d·cot α`, reaches or crosses the apex: the
    /// minted cone's nappe attribution flips inside the window, so the
    /// mint is not this face's offset.
    ApexWindow {
        /// The face whose window crosses.
        face: FaceKey,
        /// The window's infimum on the base chart, in meters of slant.
        v_min: T,
        /// The window's supremum on the base chart.
        v_max: T,
        /// The parameter shift `d·cot α` the offset applies.
        shift: T,
    },
    /// The face carries a cone but has no boundary carrier to read a
    /// `v`-window off. Refusing is the only honest answer: inventing a
    /// window would decide the apex predicate on made-up data.
    ApexWindowUnknown {
        /// The cone-faced face with no derivable window.
        face: FaceKey,
    },
    /// **The C5 boundary.** An intrinsic description's pair — the moved
    /// surface's kind against the neighbour's — has no route arm, so
    /// the edge cannot be re-stated as an intersection of the two.
    NeighborPairUnroutable {
        /// The edge that cannot be re-described.
        edge: EdgeKey,
        /// The replaced face's new surface kind.
        kind: SurfaceKind,
        /// The untouched neighbour's kind.
        other_kind: SurfaceKind,
    },
    /// **The bounded-chart boundary.** A fitted chart covers exactly
    /// its own parameter window, so a boundary edge it does not carry
    /// as one of its own rows cannot follow the moved face: the
    /// neighbour that holds the edge would have to EXTEND to meet it,
    /// and a bounded chart does not extend. Named per edge, with what
    /// the edge presented.
    FittedBoundaryUnsupported {
        /// The edge the fitted chart cannot carry.
        edge: EdgeKey,
        /// What the edge presented, in the door's own words.
        what: &'static str,
    },
    /// The pair routes, but this door does not mint a carrier for the
    /// (surface kind, carrier kind) combination the edge presents. A
    /// scope statement, not a geometric verdict: the combination is
    /// named so the unit that needs it knows what to build.
    CarrierLaneUnsupported {
        /// The edge whose carrier this door cannot transport.
        edge: EdgeKey,
        /// What the door could not do, in its own words.
        what: &'static str,
    },
    /// The fitted chart's boundary-row extraction refused: an interior
    /// iso parameter, an escalated coincidence, or a row the spline
    /// layer would not build.
    IsoRow {
        /// The edge whose row could not be extracted.
        edge: EdgeKey,
        /// The extraction door's typed refusal, verbatim.
        error: geom_brep::IsoRowError<T>,
    },
    /// A NURBS structure operation on a carrier or a fit row refused.
    Structure {
        /// The edge whose carrier could not be built.
        edge: EdgeKey,
        /// The spline layer's typed refusal.
        error: geom_core::spline::SplineError,
    },
    /// Two boundary edges meeting at one vertex transport it to
    /// definitely different points: the re-derivation is not coherent,
    /// so no point is written.
    VertexDisagreement {
        /// The vertex the edges disagree about.
        vertex: VertexKey,
        /// The measured gap between the two transported points, in
        /// meters.
        gap: T,
    },
    /// **The PER-FACE door's finding**: an edge ending at a moved
    /// vertex, but not on the replaced face's boundary, could not be
    /// re-anchored — its new endpoint is definitely off its own
    /// carrier, which did not move because the surfaces holding it did
    /// not.
    ///
    /// This is the gate that stands between the per-face door and an
    /// OBLIQUE junction's wrong body: composing that door over a whole
    /// boundary transports each corner once per chart, which is not the
    /// point an offset body needs, and the `gap` is the difference.
    /// `offset_planes_together` solves such corners simultaneously and
    /// does not reach here; everything outside that door's scope —
    /// every curved corner — still does.
    ///
    /// The simultaneous door has its OWN disagreement finding,
    /// [`ReplaceFaceError::TogetherEdgeDisagreement`]; this variant is
    /// not reused for it.
    ReanchorOffCarrier {
        /// The edge whose carrier the moved vertex left.
        edge: EdgeKey,
        /// The measured distance from the moved point to the carrier,
        /// in meters.
        gap: T,
    },
    /// **The simultaneous door's scope gate**: a face it was asked to
    /// move is not a plane. Its corner solve is three plane equations,
    /// and a curved face has no such equation — the C5-table work that
    /// follows this unit is where those corners land. Until then they
    /// refuse where they always did, at
    /// [`ReplaceFaceError::ReanchorOffCarrier`].
    TogetherNonPlanar {
        /// The face that is not a plane.
        face: FaceKey,
        /// What it carries instead.
        kind: SurfaceKind,
    },
    /// **The simultaneous door's other scope gate**: a face of the body
    /// was not in the moving set. Every corner's answer depends on all
    /// the planes meeting it, so a set the door was not told about in
    /// full is a set it cannot solve against.
    TogetherPartialSet {
        /// A face of the body that no chart move named.
        face: FaceKey,
    },
    /// **A corner the simultaneous door cannot solve.** Either fewer
    /// than three distinct planes meet there, or every triple of them
    /// is singular, or — the valence-past-3 shape — the planes do not
    /// concur after the offset, so no point satisfies them all. Refused
    /// rather than solved on a subset and hoped over: a corner placed
    /// off one of its own planes is a wrong body no tier catches.
    TogetherCorner {
        /// The vertex.
        vertex: VertexKey,
        /// How many distinct planes meet there.
        planes: usize,
        /// Which of the shapes above it is.
        what: &'static str,
    },
    /// **The simultaneous door: two chart moves disagree about one
    /// face**, or one chart's faces do not all wear the same surface.
    TogetherChartMixed {
        /// The face whose surface differs.
        face: FaceKey,
        /// The chart's first face, whose surface it should have worn.
        other: FaceKey,
    },
    /// **The simultaneous door: a face was named by more than one chart
    /// move**, so its offset is two different numbers.
    TogetherFaceRepeated {
        /// The face named twice.
        face: FaceKey,
    },
    /// **The simultaneous door: two corner solves disagree about where
    /// an edge ends.** Distinct from
    /// [`ReplaceFaceError::ReanchorOffCarrier`], which is the per-face
    /// door's finding about a moved vertex leaving an UNMOVED
    /// neighbour's carrier. This one is the simultaneous door's: the
    /// edge's own line was carried to where its two moved planes put
    /// it, and the far endpoint — solved independently, against a
    /// different triple of planes — did not land on it. Two solves
    /// agreeing is the claim; this is it failing.
    TogetherEdgeDisagreement {
        /// The edge whose two ends were solved apart.
        edge: EdgeKey,
        /// How far the far endpoint missed the line, in meters.
        gap: T,
    },
    /// **The axial door's kind gate**: a face wears a surface that is
    /// not a plane, cylinder, cone or sphere. The axial reduction reads
    /// each surface as a line or a circle in the meridian half-plane,
    /// and a torus or a NURBS is neither — such bodies keep the
    /// per-chart door and the refusal it gives them.
    TogetherAxialUnsupported {
        /// The face.
        face: FaceKey,
        /// What it carries.
        kind: SurfaceKind,
    },
    /// **The axial door's axis gate**: a face's surface is one of the
    /// door's kinds but is not a surface of revolution about the body's
    /// axis — a cylinder skew to it, a sphere centred off it, a plane
    /// cutting it obliquely. The meridian reduction has no coordinates
    /// for such a corner, so it refuses rather than solving in a frame
    /// the geometry does not live in.
    TogetherNotAxial {
        /// The face.
        face: FaceKey,
        /// Which shape it is.
        what: &'static str,
    },
    /// **A curved corner the axial door cannot solve.** Fewer than two
    /// profile constraints and not an axis pole; a pair too tangent to
    /// resolve the corner against its own edge chords; surfaces that do
    /// not concur after the offset; or an azimuth more than one plane
    /// through the axis over-determines. Refused rather than solved on
    /// a subset: a corner placed off one of its own surfaces is a wrong
    /// body no tier catches.
    TogetherAxialCorner {
        /// The vertex.
        vertex: VertexKey,
        /// How many distinct surfaces meet there.
        surfaces: usize,
        /// Which of the shapes above it is.
        what: &'static str,
    },
    /// **An edge the axial door cannot re-derive.** Its carrier kind,
    /// or its chart's offset map, is outside the closed forms this door
    /// carries — a mapped description on a chart whose motion is a
    /// radius change, a circular edge whose plane is not normal to the
    /// axis, a carrier that is neither a line nor a circle.
    TogetherAxialEdge {
        /// The edge.
        edge: EdgeKey,
        /// Which shape it is.
        what: &'static str,
    },
    /// A margined predicate escalated: the margin landed in the
    /// ambiguity band or was poisoned (escalate-never-guess, D4 ¶3).
    Escalated {
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// An attach-layer door refused the planned mutation.
    Op {
        /// The edge the attach door refused (absent for the surface).
        edge: Option<EdgeKey>,
        /// The attach layer's typed refusal.
        error: EulerOpError,
    },
    /// The whole-body pcurve mint refused on the re-described body.
    Pcurve {
        /// The mint's typed refusal.
        source: PcurveMintError,
    },
    /// The re-described clone is not tier-2 valid, so it is discarded.
    ResultNotClosed {
        /// The validator's report.
        errors: Vec<ValidationError>,
    },
}

impl<T: Real> core::fmt::Display for ReplaceFaceError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StaleFace { face } => {
                write!(f, "replace_face_offset: {face:?} does not resolve")
            }
            Self::Corrupt => write!(
                f,
                "replace_face_offset: the body's referential coherence broke mid-plan (kernel bug)"
            ),
            Self::Offset { face, error } => {
                write!(f, "replace_face_offset: {face:?}'s offset refused: {error}")
            }
            Self::Fit { face, error } => write!(
                f,
                "replace_face_offset: {face:?}'s approximating-surface fit refused: {error}"
            ),
            Self::ApproxLaneUnsupported { face } => write!(
                f,
                "replace_face_offset: {face:?} carries a NURBS surface and this scalar has no \
                 fit lane, so its offset cannot be minted"
            ),
            Self::SharedSurfaceKey { face, other } => write!(
                f,
                "replace_face_offset: {face:?}'s surface is shared with {other:?}, so replacing \
                 it would leave the sharer on the old chart while the shared boundary names \
                 the new one — replacing a shared chart is a multi-face operation"
            ),
            Self::EmptyGroup => write!(
                f,
                "replace_faces_offset: no face was named, so there is no chart to replace"
            ),
            Self::GroupChartsDiffer { face, other } => write!(
                f,
                "replace_faces_offset: {face:?} and {other:?} carry different surfaces — a \
                 group is one chart's faces, not an arbitrary set"
            ),
            Self::PlaceholderSurface { face } => write!(
                f,
                "replace_face_offset: {face:?} carries the not-yet-described placeholder \
                 surface, which has no locus to offset"
            ),
            Self::ApexWindow {
                face,
                v_min,
                v_max,
                shift,
            } => write!(
                f,
                "replace_face_offset: {face:?}'s v-window [{v_min:?}, {v_max:?}] (m of slant) \
                 shifted by the cone offset's d·cot α ({shift:?} m) reaches or crosses the apex \
                 — the minted cone's nappe attribution flips inside the window, so it is not \
                 this face's offset"
            ),
            Self::ApexWindowUnknown { face } => write!(
                f,
                "replace_face_offset: {face:?} carries a cone but has no boundary carrier, so \
                 the apex-window predicate has no window to decide over"
            ),
            Self::NeighborPairUnroutable {
                edge,
                kind,
                other_kind,
            } => write!(
                f,
                "replace_face_offset: {edge:?} cannot be re-described — {}",
                geom_brep::intersect::route(*kind, *other_kind).refusal(*kind, *other_kind)
            ),
            Self::FittedBoundaryUnsupported { edge, what } => write!(
                f,
                "replace_face_offset: {edge:?} is on a fitted face's boundary but is not one of \
                 the fit's own rows ({what}) — a bounded chart covers only its own window, so \
                 the untouched neighbour holding this edge would have to extend to meet the \
                 moved face"
            ),
            Self::CarrierLaneUnsupported { edge, what } => write!(
                f,
                "replace_face_offset: {edge:?}'s carrier is outside this door's transport \
                 lanes ({what})"
            ),
            Self::IsoRow { edge, error } => write!(
                f,
                "replace_face_offset: {edge:?}'s row could not be extracted from the fitted \
                 chart: {error}"
            ),
            Self::Structure { edge, error } => write!(
                f,
                "replace_face_offset: {edge:?}'s new carrier is not valid spline structure: \
                 {error}"
            ),
            Self::VertexDisagreement { vertex, gap } => write!(
                f,
                "replace_face_offset: two boundary edges transport {vertex:?} to points \
                 {gap:?} m apart — the re-derivation is not coherent"
            ),
            Self::ReanchorOffCarrier { edge, gap } => write!(
                f,
                "replace_face_offset: {edge:?} ends at a moved vertex that is {gap:?} m off its \
                 own carrier, so its parameter cannot be re-anchored"
            ),
            Self::TogetherAxialUnsupported { face, kind } => write!(
                f,
                "offset_charts_together: {face:?} carries a {}, which the axial reduction has \
                 no meridian curve for — this body keeps the per-chart door",
                kind.name()
            ),
            Self::TogetherNotAxial { face, what } => write!(
                f,
                "offset_charts_together: {face:?} is {what}, so it is not a surface of \
                 revolution about this body's axis and its corners have no meridian coordinates"
            ),
            Self::TogetherAxialCorner {
                vertex,
                surfaces,
                what,
            } => write!(
                f,
                "offset_charts_together: {vertex:?} has {surfaces} distinct moved surfaces and \
                 {what}"
            ),
            Self::TogetherAxialEdge { edge, what } => write!(
                f,
                "offset_charts_together: {edge:?} cannot be re-derived — {what}"
            ),
            Self::Escalated { source } => {
                write!(f, "replace_face_offset escalated: {source}")
            }
            Self::Op { edge, error } => match edge {
                Some(e) => write!(
                    f,
                    "replace_face_offset: the attach door refused {e:?}: {error}"
                ),
                None => write!(f, "replace_face_offset: the attach door refused: {error}"),
            },
            Self::Pcurve { source } => {
                write!(f, "replace_face_offset: the pcurve mint refused: {source}")
            }
            Self::TogetherChartMixed { face, other } => write!(
                f,
                "offset_planes_together: {face:?} does not wear the same surface as its chart's \
                 {other:?} — a chart move names ONE chart"
            ),
            Self::TogetherFaceRepeated { face } => write!(
                f,
                "offset_planes_together: {face:?} is named by more than one chart move, so its \
                 offset is two different numbers"
            ),
            Self::TogetherEdgeDisagreement { edge, gap } => write!(
                f,
                "offset_planes_together: {edge:?}'s two ends were solved {gap:?} m apart — the \
                 far corner's own solve did not land on the line its two moved planes carry"
            ),
            Self::TogetherNonPlanar { face, kind } => write!(
                f,
                "offset_planes_together: {face:?} carries a {kind:?}, and this door solves \
                 corners as plane equations — a curved corner has none"
            ),
            Self::TogetherPartialSet { face } => write!(
                f,
                "offset_planes_together: {face:?} is a face of the body that no chart move \
                 named — every corner's answer depends on all the planes meeting it, so a \
                 partial set has corners this door cannot solve"
            ),
            Self::TogetherCorner {
                vertex,
                planes,
                what,
            } => write!(
                f,
                "offset_planes_together: the corner at {vertex:?} ({planes} distinct planes) \
                 has no offset point — {what}"
            ),
            Self::ResultNotClosed { errors } => write!(
                f,
                "replace_face_offset: the re-described body is not tier-2 valid ({} errors); \
                 the clone is discarded",
                errors.len()
            ),
        }
    }
}

impl<T: Real> std::error::Error for ReplaceFaceError<T> {}

// ---------------------------------------------------------------------
// The transport lanes
// ---------------------------------------------------------------------

/// The offset's action on one curve that lies on `old`, at the curve's
/// own parameterization: the transported carrier, plus the translation
/// vector when the action WAS a rigid translation (a `MappedCurve`'s
/// placement composes with that and with nothing else).
///
/// `mid` is the curve's mid-parameter point, which is what selects the
/// ruling/nappe on the kinds whose chart normal varies with position.
type Transported<T> = Option<(Curve3<T>, Option<Vec3<T>>)>;

/// Why a transport could not be produced — kept distinct from "this
/// lane does not exist", which is what `Ok(None)` says.
enum TransportError {
    /// A named margined predicate escalated.
    Escalated(Indeterminate),
    /// The transported control net is not valid spline structure. A
    /// STRUCTURE failure, never a scope one: the lane exists and ran.
    Structure(geom_core::spline::SplineError),
}

fn transport_curve<T: Decide>(
    old: &Surface<T>,
    d: T,
    curve: &Curve3<T>,
    mid: Point3<T>,
    band: Band,
) -> Result<Transported<T>, TransportError> {
    Ok(match old {
        Surface::Plane { normal, .. } => {
            let delta = *normal * d;
            Some((
                translate_curve(curve, delta).map_err(TransportError::Structure)?,
                Some(delta),
            ))
        }
        Surface::Cylinder { origin, axis, .. } => match curve {
            Curve3::Line { .. } | Curve3::Nurbs(_) => {
                let radial = (mid - *origin).reject_from(*axis).normalize();
                let delta = radial * d;
                Some((
                    translate_curve(curve, delta).map_err(TransportError::Structure)?,
                    Some(delta),
                ))
            }
            Curve3::Circle {
                center,
                axis: ca,
                radius,
                u_ref,
            } => Some((
                Curve3::Circle {
                    center: *center,
                    axis: *ca,
                    radius: *radius + d,
                    u_ref: *u_ref,
                },
                None,
            )),
            Curve3::Ellipse { .. } => None,
        },
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            // ONE derivation, read from `geom_brep`: the apex slide,
            // the parameter shift and the pointwise displacement are
            // three faces of the mint's own action, and this door reads
            // all three rather than re-deriving any (the drift that
            // costs is a second copy, not a second call).
            let action = geom_brep::ConeOffset::new(*apex, *axis, *half_angle, d);
            let (sin_a, cos_a) = half_angle.sin_cos();
            match curve {
                // A chart line is a generator, and the action's
                // displacement is constant along one (the azimuth does
                // not vary), so the transport is rigid.
                Curve3::Line { .. } | Curve3::Nurbs(_) => {
                    let delta = action.displacement(mid);
                    Some((
                        translate_curve(curve, delta).map_err(TransportError::Structure)?,
                        Some(delta),
                    ))
                }
                // A parallel: `v` shifts by the action's own shift, and
                // the circle re-mints against the action's own APEX —
                // the slide the mint applies and this arm used to omit.
                Curve3::Circle {
                    center,
                    axis: ca,
                    u_ref,
                    ..
                } => {
                    let v = (*center - *apex).dot(*axis) / cos_a;
                    let v_new = v + action.shift();
                    Some((
                        Curve3::Circle {
                            center: action.apex() + *axis * (v_new * cos_a),
                            axis: *ca,
                            radius: (v_new * sin_a).abs(),
                            u_ref: *u_ref,
                        },
                        None,
                    ))
                }
                Curve3::Ellipse { .. } => None,
            }
        }
        // The sphere's offset IS the homothety of ratio `(R + d)/R`
        // about the centre, so every curve on it transports by that one
        // map — no per-kind case analysis, only per-kind arithmetic.
        Surface::Sphere { center, radius, .. } => {
            let k = (*radius + d) / *radius;
            homothety(curve, *center, k)
                .map_err(TransportError::Structure)?
                .map(|c| (c, None))
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => match curve {
            Curve3::Circle {
                center: c,
                axis: ca,
                radius,
                u_ref,
            } => {
                // Meridian or parallel is a question about the STORED
                // circle's frame against the torus'; the margin is the
                // sine/cosine split at the half-turn, and a carrier
                // whose axis sits between the two is not a curve either
                // arm describes.
                let along = ca.dot(*axis);
                let parallel = matches!(
                    decide(
                        "offset_torus_carrier_axis",
                        Margin::of(along.abs() - T::from_f64(0.5)),
                        band,
                    )
                    .map_err(TransportError::Escalated)?,
                    Sign::Positive
                );
                if parallel {
                    // A parallel: recover `v` from the stored circle
                    // (`radius = R + r·cos v`, axial offset `r·sin v`)
                    // and re-mint it at `r + d`.
                    let cos_v = (*radius - *major_radius) / *minor_radius;
                    let sin_v = (*c - *center).dot(*axis) / *minor_radius;
                    let r_new = *minor_radius + d;
                    Some((
                        Curve3::Circle {
                            center: *center + *axis * (r_new * sin_v),
                            axis: *ca,
                            radius: *major_radius + r_new * cos_v,
                            u_ref: *u_ref,
                        },
                        None,
                    ))
                } else {
                    // A meridian (tube) circle: the tube centre is
                    // fixed and the radius moves with the minor.
                    Some((
                        Curve3::Circle {
                            center: *c,
                            axis: *ca,
                            radius: *minor_radius + d,
                            u_ref: *u_ref,
                        },
                        None,
                    ))
                }
            }
            _ => None,
        },
        // The fitted lane: the chart normal at the domain midpoint is
        // the one direction this door has, and the cost of using it is
        // `d·|n − n₀|` — classified downstream by the certified gate,
        // never assumed away here.
        Surface::Nurbs(n) => match mid_domain_normal(n) {
            None => None,
            Some(n0) => {
                let delta = n0 * d;
                Some((
                    translate_curve(curve, delta).map_err(TransportError::Structure)?,
                    Some(delta),
                ))
            }
        },
        Surface::Approx(_) => None,
    })
}

/// The chart normal at the centre of a NURBS surface's own domain.
fn mid_domain_normal<T: Decide>(s: &NurbsSurface<T>) -> Option<Vec3<T>> {
    let (u0, u1) = s.knots_u().domain();
    let (v0, v1) = s.knots_v().domain();
    let jet = s.ders(T::from_f64((u0 + u1) * 0.5), T::from_f64((v0 + v1) * 0.5));
    let n = jet.du.cross(jet.dv).normalize();
    (!n.x.is_poison() && !n.y.is_poison() && !n.z.is_poison()).then_some(n)
}

/// `curve` translated by `delta` — exact on every carrier kind (a
/// translation acts on the stored anchor and leaves every frame,
/// radius and weight alone).
pub(crate) fn translate_curve<T: Real>(
    curve: &Curve3<T>,
    delta: Vec3<T>,
) -> Result<Curve3<T>, geom_core::spline::SplineError> {
    Ok(match curve {
        Curve3::Line { origin, dir } => Curve3::Line {
            origin: *origin + delta,
            dir: *dir,
        },
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => Curve3::Circle {
            center: *center + delta,
            axis: *axis,
            radius: *radius,
            u_ref: *u_ref,
        },
        Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => Curve3::Ellipse {
            center: *center + delta,
            axis: *axis,
            major: *major,
            minor: *minor,
            u_ref: *u_ref,
        },
        Curve3::Nurbs(n) => Curve3::Nurbs(Arc::new(NurbsCurve3::new(
            n.knots().clone(),
            n.control().iter().map(|p| *p + delta).collect(),
            n.weights().to_vec(),
        )?)),
    })
}

/// `curve` under the homothety of ratio `k` about `c` — the sphere
/// offset's own map.
fn homothety<T: Real>(
    curve: &Curve3<T>,
    c: Point3<T>,
    k: T,
) -> Result<Option<Curve3<T>>, geom_core::spline::SplineError> {
    let map = |p: Point3<T>| c + (p - c) * k;
    Ok(Some(match curve {
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => Curve3::Circle {
            center: map(*center),
            axis: *axis,
            radius: *radius * k,
            u_ref: *u_ref,
        },
        Curve3::Nurbs(n) => Curve3::Nurbs(Arc::new(NurbsCurve3::new(
            n.knots().clone(),
            n.control().iter().map(|p| map(*p)).collect(),
            n.weights().to_vec(),
        )?)),
        // A line or an ellipse does not lie on a sphere, so a carrier
        // of either kind is not a curve this map was derived for.
        Curve3::Line { .. } | Curve3::Ellipse { .. } => return Ok(None),
    }))
}

// ---------------------------------------------------------------------
// The door
// ---------------------------------------------------------------------

/// One boundary edge's whole re-derivation, decided before anything is
/// written.
struct EdgePlan<T: Real> {
    edge: EdgeKey,
    spec: EdgeCurveSpec<T>,
    start: VertexKey,
    end: VertexKey,
    p_start: Point3<T>,
    p_end: Point3<T>,
}

/// Replaces `face`'s surface with its certified offset at signed
/// distance `d` and re-describes the face's boundary against the moved
/// chart (module docs).
///
/// `tolerance` is the fit door's parameter and is consulted only on the
/// NURBS lane, where the offset is not closed-form; the analytic kinds
/// mint exactly and ignore it.
///
/// The body is **untouched on every `Err`**: the mint, the refusals and
/// the whole boundary plan are decided read-only, the mutation runs on
/// a clone, and the clone is adopted only after it validates.
///
/// # Errors
///
/// [`ReplaceFaceError`] — the offset door's own refusals, the fit
/// door's, the apex-window predicate, the C5 routing boundary, the
/// carrier lanes' scope, a re-derivation the attach layer's
/// certification rejects, and a clone that does not validate.
pub fn replace_face_offset<T: Decide + PropsQuadLane>(
    body: &mut Body<T>,
    face: FaceKey,
    d: T,
    tolerance: f64,
    band: Band,
    tol: Tol,
) -> Result<(), ReplaceFaceError<T>> {
    replace_faces_offset(body, &[face], d, tolerance, band, tol)
}

/// [`replace_face_offset`] for a CHART: every face carrying one surface
/// key, replaced together.
///
/// A surface can be worn by more than one face — a full revolve splits
/// its wall into two bands over one cylinder, `step-import`'s adoption
/// shares keys outright — and such a chart cannot be replaced one face
/// at a time: the fresh key would leave the sharer on the old surface
/// while their shared seam named the new one, and a `Seam` description
/// requires ONE surface on both sides, so there is no re-description
/// that repairs it afterwards. That is what
/// [`ReplaceFaceError::SharedSurfaceKey`] refuses, and this door is the
/// capability the refusal points at: name the whole group, and the
/// chart moves as one.
///
/// `faces` must be exactly the set of faces carrying the chart — not a
/// subset (the refusal above) and not a mixture of charts
/// ([`ReplaceFaceError::GroupChartsDiffer`]).
///
/// # Errors
///
/// [`ReplaceFaceError`] — [`replace_face_offset`]'s, plus the group
/// gates.
pub fn replace_faces_offset<T: Decide + PropsQuadLane>(
    body: &mut Body<T>,
    faces: &[FaceKey],
    d: T,
    tolerance: f64,
    band: Band,
    tol: Tol,
) -> Result<(), ReplaceFaceError<T>> {
    // ---- Decide: the group. ----
    let Some(&face) = faces.first() else {
        return Err(ReplaceFaceError::EmptyGroup);
    };
    let face_data = body
        .get_face(face)
        .ok_or(ReplaceFaceError::StaleFace { face })?;
    let old_key = face_data.surface;
    for &member in &faces[1..] {
        let data = body
            .get_face(member)
            .ok_or(ReplaceFaceError::StaleFace { face: member })?;
        if data.surface != old_key {
            return Err(ReplaceFaceError::GroupChartsDiffer {
                face,
                other: member,
            });
        }
    }
    // The group must be the WHOLE group: a chart with a face left
    // behind is the incoherence this door exists to avoid.
    if let Some((other, _)) = body
        .faces()
        .find(|(k, f)| !faces.contains(k) && f.surface == old_key)
    {
        return Err(ReplaceFaceError::SharedSurfaceKey { face, other });
    }
    let old_surface = body
        .get_surface(old_key)
        .ok_or(ReplaceFaceError::Corrupt)?
        .clone();
    let new_surface = mint_offset(face, &old_surface, d, tolerance, band)?;

    // ---- Decide: the apex window (cones only). ----
    let shift = apex_shift(&old_surface, d);
    if let Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = old_surface
    {
        let (v_min, v_max) = group_cone_v_window(body, faces, apex, axis, half_angle.cos())
            .ok_or(ReplaceFaceError::ApexWindowUnknown { face })?;
        // Which nappe the face lives on decides which end of its window
        // is the one nearest the apex — and a window that already
        // straddles the apex is a face with no single nappe to offset.
        let esc = |source| ReplaceFaceError::Escalated { source };
        let low = decide("offset_apex_nappe", Margin::of(v_min), band).map_err(esc)?;
        let high = decide("offset_apex_nappe", Margin::of(v_max), band).map_err(esc)?;
        let (v_near, sense) = match (low, high) {
            (Sign::Positive, _) => (v_min, T::one()),
            (_, Sign::Negative) => (v_max, -T::one()),
            // Zero at either end, or opposite signs: the window reaches
            // the apex before any offset is applied.
            _ => {
                return Err(ReplaceFaceError::ApexWindow {
                    face,
                    v_min,
                    v_max,
                    shift,
                });
            }
        };
        // `inf(v-window) + d·cot α > 0` on the opening nappe, and its
        // mirror on the other — one margin, signed by the nappe.
        let realized = (v_near + shift) * sense;
        match decide("offset_apex_window", Margin::of(realized), band).map_err(esc)? {
            Sign::Positive => {}
            Sign::Zero | Sign::Negative => {
                return Err(ReplaceFaceError::ApexWindow {
                    face,
                    v_min,
                    v_max,
                    shift,
                });
            }
        }
    }

    // ---- Decide: the boundary plan. ----
    let boundary = group_boundary(body, faces).ok_or(ReplaceFaceError::Corrupt)?;
    let mut plans: Vec<EdgePlan<T>> = Vec::with_capacity(boundary.len());
    for &edge in &boundary {
        plans.push(plan_edge(
            body,
            edge,
            faces,
            &old_surface,
            old_key,
            &new_surface,
            d,
            shift,
            band,
        )?);
    }

    // ---- Decide: where each moved vertex lands, and the agreement. ----
    // Every boundary edge transports its own endpoints, so a vertex on
    // k edges gets k candidate points. They are checked PAIRWISE, not
    // each against the first: a star comparison passes a spread of up
    // to 2ε (two points each within ε of the centre but 2ε from each
    // other), and the claim being made is that the re-derivation is
    // coherent, which is a statement about the whole set.
    let mut candidates: Vec<(VertexKey, Vec<Point3<T>>)> = Vec::new();
    for plan in &plans {
        for (vertex, point) in [(plan.start, plan.p_start), (plan.end, plan.p_end)] {
            match candidates.iter_mut().find(|(v, _)| *v == vertex) {
                Some((_, points)) => points.push(point),
                None => candidates.push((vertex, vec![point])),
            }
        }
    }
    let mut moved: Vec<(VertexKey, Point3<T>)> = Vec::new();
    for (vertex, points) in candidates {
        for (i, a) in points.iter().enumerate() {
            for b in &points[i + 1..] {
                let gap = a.distance(*b);
                match decide(
                    "offset_vertex_agreement",
                    Margin::of(T::from_f64(tol.eps()) - gap),
                    band,
                )
                .map_err(|source| ReplaceFaceError::Escalated { source })?
                {
                    Sign::Positive | Sign::Zero => {}
                    Sign::Negative => {
                        return Err(ReplaceFaceError::VertexDisagreement { vertex, gap });
                    }
                }
            }
        }
        let Some(point) = points.first().copied() else {
            return Err(ReplaceFaceError::Corrupt);
        };
        moved.push((vertex, point));
    }

    // ---- Decide: the incident edges that only need re-anchoring. ----
    let anchored = plan_reanchors(body, &boundary, &moved, band, tol)?;

    // ---- Mutation, on a clone (infallible decisions are done). ----
    let mut work = body.clone();
    // `FaceSurface::New` mints a fresh arena key, so every planned
    // description that names the replaced surface is re-pointed at it
    // before it is attached — the same re-description step the stale-key
    // rule forces on any surface replacement.
    let new_key = work
        .set_face_surface(face, FaceSurface::New(new_surface))
        .map_err(|error| ReplaceFaceError::Op { edge: None, error })?;
    // The rest of the chart's faces adopt the SAME key: the group wore
    // one surface before and wears one after, which is what keeps their
    // shared seams describable.
    for &member in &faces[1..] {
        work.set_face_surface(member, FaceSurface::Shared(new_key))
            .map_err(|error| ReplaceFaceError::Op { edge: None, error })?;
    }
    for (vertex, point) in &moved {
        let old_point = work
            .get_vertex(*vertex)
            .ok_or(ReplaceFaceError::Corrupt)?
            .point;
        let new_point = work.add_point(*point);
        work.get_vertex_mut(*vertex)
            .ok_or(ReplaceFaceError::Corrupt)?
            .point = new_point;
        work.remove_point_if_orphaned(old_point);
    }
    for mut plan in plans {
        plan.spec.description = remap_description(plan.spec.description, old_key, new_key);
        work.set_edge_curve(plan.edge, plan.spec, tol)
            .map_err(|error| ReplaceFaceError::Op {
                edge: Some(plan.edge),
                error,
            })?;
    }
    for (edge, spec) in anchored {
        work.set_edge_curve(edge, spec, tol)
            .map_err(|error| ReplaceFaceError::Op {
                edge: Some(edge),
                error,
            })?;
    }
    mint_pcurves(&mut work, tol).map_err(|source| ReplaceFaceError::Pcurve { source })?;
    validate_closed(&work).map_err(|errors| ReplaceFaceError::ResultNotClosed { errors })?;

    *body = work;
    Ok(())
}

/// The offset surface for `old`: the analytic mint, or the fit lane's
/// certified `Approx` where the kind is not closed under offset.
fn mint_offset<T: Decide + PropsQuadLane>(
    face: FaceKey,
    old: &Surface<T>,
    d: T,
    tolerance: f64,
    band: Band,
) -> Result<Surface<T>, ReplaceFaceError<T>> {
    if let Surface::Nurbs(base) = old {
        if base.is_placeholder() {
            return Err(ReplaceFaceError::PlaceholderSurface { face });
        }
        return match T::approx_offset_surface(Arc::clone(base), d, tolerance, band) {
            None => Err(ReplaceFaceError::ApproxLaneUnsupported { face }),
            Some(Ok(s)) => Ok(s),
            Some(Err(error)) => Err(ReplaceFaceError::Fit { face, error }),
        };
    }
    geom_brep::offset_surface(old, d, band)
        .map_err(|error| ReplaceFaceError::Offset { face, error })
}

/// The cone offset's `v` shift `d·cot α`; zero on every other kind (no
/// other chart's parameterization moves under offset).
fn apex_shift<T: Real>(old: &Surface<T>, d: T) -> T {
    match old {
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => geom_brep::ConeOffset::new(*apex, *axis, *half_angle, d).shift(),
        _ => T::zero(),
    }
}

/// `face`'s `v`-window on a cone chart: the hull of its BOUNDARY
/// carriers' `v`-ranges.
///
/// `v` is an affine functional of position on a cone chart
/// (`v = (p − apex)·axis / cos α`), so each carrier's range is closed
/// form — endpoints for a line, centre ± amplitude for a conic, the
/// control net's own hull for a spline (the convex-hull property). And
/// a coordinate's extremes over a compact chart region are attained on
/// its boundary, so the hull of the boundary's ranges IS the face's
/// window. Nothing is sampled and nothing is padded.
///
/// `None` when the face has no boundary carrier to read.
fn group_cone_v_window<T: Decide>(
    body: &Body<T>,
    group: &[FaceKey],
    apex: Point3<T>,
    axis: Vec3<T>,
    cos_a: T,
) -> Option<(T, T)> {
    let mut window: Option<(T, T)> = None;
    for edge in group_boundary(body, group)? {
        let edge_data = body.get_edge(edge)?;
        let curve = body
            .get_curve_geom(edge_data.curve)
            .and_then(crate::null::CurveGeom::certified)?;
        let (t0, t1) = curve.params();
        let (lo, hi) = cone_v_range(curve.carrier(), t0, t1, apex, axis, cos_a);
        window = Some(match window {
            None => (lo, hi),
            Some((a, b)) => (a.min(lo), b.max(hi)),
        });
    }
    window
}

/// The `v`-range of one carrier on a cone chart (see
/// [`group_cone_v_window`]).
fn cone_v_range<T: Decide>(
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
    apex: Point3<T>,
    axis: Vec3<T>,
    cos_a: T,
) -> (T, T) {
    let v_of = |p: Point3<T>| (p - apex).dot(axis) / cos_a;
    match carrier {
        // `v` is affine in `t`, so the endpoints are the extremes.
        Curve3::Line { .. } => {
            let (a, b) = (v_of(carrier.eval(t0)), v_of(carrier.eval(t1)));
            (a.min(b), a.max(b))
        }
        // A conic's `v` is `centre ± amplitude·cos(θ − φ)`: the
        // amplitude is the semi-axes' own components along the cone
        // axis. Taken over the FULL period, which is conservative on a
        // sub-arc and exact on a closed rim.
        Curve3::Circle {
            center,
            axis: ca,
            radius,
            u_ref,
        } => {
            let v_ref = ca.cross(*u_ref);
            let amp = ((u_ref.dot(axis) * *radius).powi(2) + (v_ref.dot(axis) * *radius).powi(2))
                .sqrt()
                / cos_a;
            let c = v_of(*center);
            (c - amp, c + amp)
        }
        Curve3::Ellipse {
            center,
            axis: ca,
            major,
            minor,
            u_ref,
        } => {
            let v_ref = ca.cross(*u_ref);
            let amp = ((u_ref.dot(axis) * *major).powi(2) + (v_ref.dot(axis) * *minor).powi(2))
                .sqrt()
                / cos_a;
            let c = v_of(*center);
            (c - amp, c + amp)
        }
        // The convex-hull property: the image lies in the hull of the
        // control polygon, and an affine functional's range over a hull
        // is its range over the vertices.
        Curve3::Nurbs(n) => {
            let mut lo: Option<T> = None;
            let mut hi: Option<T> = None;
            for p in n.control() {
                let v = v_of(*p);
                lo = Some(lo.map_or(v, |x: T| x.min(v)));
                hi = Some(hi.map_or(v, |x: T| x.max(v)));
            }
            (lo.unwrap_or_else(T::zero), hi.unwrap_or_else(T::zero))
        }
    }
}

/// The group's boundary edges, in face-then-loop-then-cycle order,
/// each once — including the seams INTERNAL to the group, which move
/// with the chart exactly as its outer edges do.
fn group_boundary<T: Real>(body: &Body<T>, group: &[FaceKey]) -> Option<Vec<EdgeKey>> {
    let mut out: Vec<EdgeKey> = Vec::new();
    for &face in group {
        boundary_edges_into(body, face, &mut out)?;
    }
    Some(out)
}

/// `face`'s boundary edges appended to `out`, each once.
fn boundary_edges_into<T: Real>(
    body: &Body<T>,
    face: FaceKey,
    out: &mut Vec<EdgeKey>,
) -> Option<()> {
    let face_data = body.get_face(face)?;
    for lk in core::iter::once(face_data.outer).chain(face_data.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body.get_loop(lk)?.boundary else {
            continue;
        };
        for he in body.loop_cycle(first)? {
            let edge = body.get_half_edge(he)?.edge;
            if !out.contains(&edge) {
                out.push(edge);
            }
        }
    }
    Some(())
}

/// One boundary edge's re-derivation: the description re-stated against
/// the moved chart, the carrier transported, the endpoints read off the
/// transported carrier.
#[allow(clippy::too_many_arguments)]
fn plan_edge<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    group: &[FaceKey],
    old_surface: &Surface<T>,
    old_key: SurfaceKey,
    new_surface: &Surface<T>,
    d: T,
    shift: T,
    band: Band,
) -> Result<EdgePlan<T>, ReplaceFaceError<T>> {
    let edge_data = body.get_edge(edge).ok_or(ReplaceFaceError::Corrupt)?;
    let he_plus = edge_data.he_plus;
    let start = body
        .get_half_edge(he_plus)
        .ok_or(ReplaceFaceError::Corrupt)?
        .start;
    let end = body
        .half_edge_end(he_plus)
        .ok_or(ReplaceFaceError::Corrupt)?;
    let curve = body
        .get_curve_geom(edge_data.curve)
        .and_then(crate::null::CurveGeom::certified)
        .ok_or(ReplaceFaceError::Corrupt)?;
    let (t0, t1) = curve.params();
    let old_carrier = curve.carrier().clone();
    let description = *curve.description();
    let mid = old_carrier.eval((t0 + t1) * T::from_f64(0.5));

    // The one description that gets an EXACT carrier rather than a
    // transported one: an iso-curve of a fitted chart is a row of the
    // fit's own control net, so extracting it lands the carrier in the
    // fit's spline space — its degree and its refined interior knots —
    // without elevating or refining anything.
    if let (
        EdgeGeometry::IsoCurve {
            surface, u, v0, v1, ..
        },
        Surface::Approx(approx),
    ) = (&description, new_surface)
        && *surface == old_key
    {
        // The seam this row carries is shared with whatever face
        // sits on the other side. If THAT face is a bounded chart
        // too, it would have to move with this one to keep holding
        // the edge — which is a body-wide offset, not a
        // face-replacement, and this door says so rather than
        // storing a row the neighbour's own lane will reject.
        let (fa, fb) = edge_faces(body, edge).ok_or(ReplaceFaceError::Corrupt)?;
        let other = if group.contains(&fa) { fb } else { fa };
        if !group.contains(&other)
            && matches!(
                body.get_surface(
                    body.get_face(other)
                        .ok_or(ReplaceFaceError::Corrupt)?
                        .surface
                ),
                Some(Surface::Nurbs(_) | Surface::Approx(_))
            )
        {
            return Err(ReplaceFaceError::FittedBoundaryUnsupported {
                edge,
                what: "a seam shared with another bounded chart",
            });
        }
        // The extraction itself lives in `geom_brep::nurbs_iso`, beside
        // `boundary_iso_u` and its asserting rows: the door's lane is
        // the call, not the arithmetic. PR-2's schedule for this lane
        // is the fitted-boundary note below — until every chart
        // bounding an edge moves together, the only reachable exit
        // past this call is a refusal.
        let (row, u_domain) = geom_brep::iso_boundary_row(approx.fit(), *u, band)
            .map_err(|error| ReplaceFaceError::IsoRow { edge, error })?;
        let carrier = Curve3::Nurbs(Arc::new(row));
        let p_start = carrier.eval(*v0);
        let p_end = carrier.eval(*v1);
        return Ok(EdgePlan {
            edge,
            spec: EdgeCurveSpec {
                description: EdgeGeometry::IsoCurve {
                    surface: old_key,
                    u: u_domain,
                    v0: *v0,
                    v1: *v1,
                },
                carrier,
                param_start: *v0,
                param_end: *v1,
            },
            start,
            end,
            p_start,
            p_end,
        });
    }

    // Past the fit's own rows, a fitted face's boundary has nowhere to
    // go: every remaining lane transports a carrier off the chart that
    // is supposed to hold it.
    if matches!(new_surface, Surface::Approx(_)) {
        return Err(ReplaceFaceError::FittedBoundaryUnsupported {
            edge,
            what: match description {
                EdgeGeometry::IsoCurve { .. } => "an iso-curve of a neighbour's chart",
                EdgeGeometry::Seam { .. } => "a periodic seam",
                EdgeGeometry::MappedCurve(_) => {
                    "a mapped rim (a v-row is not an `IsoCurve`, which is u-const by definition)"
                }
                EdgeGeometry::Intersection { .. } | EdgeGeometry::TangentIntersection { .. } => {
                    "an intrinsic intersection with an untouched neighbour"
                }
            },
        });
    }

    let (carrier, delta) = transport_curve(old_surface, d, &old_carrier, mid, band)
        .map_err(|e| match e {
            TransportError::Escalated(source) => ReplaceFaceError::Escalated { source },
            TransportError::Structure(error) => ReplaceFaceError::Structure { edge, error },
        })?
        .ok_or(ReplaceFaceError::CarrierLaneUnsupported {
            edge,
            what: "this (surface kind, carrier kind) pair has no closed-form offset action",
        })?;
    let new_mid = carrier.eval((t0 + t1) * T::from_f64(0.5));

    let new_description = match description {
        // A seam names a surface and nothing else — no parameter to
        // shift, unlike an iso-curve's `v` on a cone. Stated rather
        // than left to the fall-through so the contrast is on the page.
        EdgeGeometry::Seam { surface } if surface == old_key => {
            EdgeGeometry::Seam { surface: old_key }
        }
        EdgeGeometry::IsoCurve {
            surface, u, v0, v1, ..
        } if surface == old_key => EdgeGeometry::IsoCurve {
            surface: old_key,
            u,
            v0: v0 + shift,
            v1: v1 + shift,
        },
        EdgeGeometry::Intersection { s1, s2, .. }
        | EdgeGeometry::TangentIntersection { s1, s2, .. }
            if s1 == old_key || s2 == old_key =>
        {
            let other = if s1 == old_key { s2 } else { s1 };
            let other_kind =
                SurfaceKind::of(body.get_surface(other).ok_or(ReplaceFaceError::Corrupt)?);
            let kind = SurfaceKind::of(new_surface);
            if !geom_brep::intersect::route(kind, other_kind).implemented {
                return Err(ReplaceFaceError::NeighborPairUnroutable {
                    edge,
                    kind,
                    other_kind,
                });
            }
            let tangent = matches!(description, EdgeGeometry::TangentIntersection { .. });
            let (n1, n2) = if s1 == old_key {
                (old_key, s2)
            } else {
                (s1, old_key)
            };
            if tangent {
                EdgeGeometry::TangentIntersection {
                    s1: n1,
                    s2: n2,
                    witness: new_mid,
                }
            } else {
                EdgeGeometry::Intersection {
                    s1: n1,
                    s2: n2,
                    witness: new_mid,
                }
            }
        }
        EdgeGeometry::MappedCurve(mapped) => {
            let delta = delta.ok_or(ReplaceFaceError::CarrierLaneUnsupported {
                edge,
                what: "a mapped description whose surface's offset is not a rigid translation",
            })?;
            EdgeGeometry::MappedCurve(translate_mapped(mapped, delta).ok_or(
                ReplaceFaceError::CarrierLaneUnsupported {
                    edge,
                    what: "a rotation-family mapped description (its trajectory does not translate)",
                },
            )?)
        }
        // A description naming only OTHER surfaces still moves with the
        // face — its carrier transports, and whether the untouched
        // surface it names still holds the moved locus is a question
        // the attach layer's certification answers, not this door.
        other => other,
    };

    Ok(EdgePlan {
        edge,
        spec: EdgeCurveSpec {
            description: new_description,
            carrier: carrier.clone(),
            param_start: t0,
            param_end: t1,
        },
        start,
        end,
        p_start: carrier.eval(t0),
        p_end: carrier.eval(t1),
    })
}

/// `mapped` under the translation `delta` — the placement's own
/// translation absorbs it. `None` on the rotation family, whose
/// trajectory is not a rigid function of the placement's translation.
pub(crate) fn translate_mapped<T: Real>(
    mapped: geom_brep::MappedCurve<T>,
    delta: Vec3<T>,
) -> Option<geom_brep::MappedCurve<T>> {
    let shifted = |place: Affine3<T>| Affine3::from_parts(place.linear, place.translation + delta);
    Some(match mapped {
        geom_brep::MappedCurve::PlacedSegment { segment, place } => {
            geom_brep::MappedCurve::PlacedSegment {
                segment,
                place: shifted(place),
            }
        }
        geom_brep::MappedCurve::ExtrudedPoint { point, place, vec } => {
            geom_brep::MappedCurve::ExtrudedPoint {
                point,
                place: shifted(place),
                vec,
            }
        }
        _ => return None,
    })
}

/// `description` with every occurrence of `old` re-pointed at `new` —
/// the stale-key step a fresh surface mint forces.
pub(crate) fn remap_description<T: Real>(
    description: EdgeGeometry<T>,
    old: SurfaceKey,
    new: SurfaceKey,
) -> EdgeGeometry<T> {
    let map = |k: SurfaceKey| if k == old { new } else { k };
    match description {
        EdgeGeometry::Intersection { s1, s2, witness } => EdgeGeometry::Intersection {
            s1: map(s1),
            s2: map(s2),
            witness,
        },
        EdgeGeometry::TangentIntersection { s1, s2, witness } => {
            EdgeGeometry::TangentIntersection {
                s1: map(s1),
                s2: map(s2),
                witness,
            }
        }
        EdgeGeometry::Seam { surface } => EdgeGeometry::Seam {
            surface: map(surface),
        },
        EdgeGeometry::IsoCurve { surface, u, v0, v1 } => EdgeGeometry::IsoCurve {
            surface: map(surface),
            u,
            v0,
            v1,
        },
        EdgeGeometry::MappedCurve(m) => EdgeGeometry::MappedCurve(m),
    }
}

/// The edges that end at a moved vertex without lying on the replaced
/// face's boundary: their carriers are unchanged (the surfaces that
/// hold them did not move) and only the parameter at the moved end —
/// and, for a mapped description, the sketch endpoint that parameter
/// images — is re-anchored.
fn plan_reanchors<T: Decide>(
    body: &Body<T>,
    boundary: &[EdgeKey],
    moved: &[(VertexKey, Point3<T>)],
    band: Band,
    tol: Tol,
) -> Result<Vec<(EdgeKey, EdgeCurveSpec<T>)>, ReplaceFaceError<T>> {
    let mut out = Vec::new();
    let keys: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    for edge in keys {
        if boundary.contains(&edge) {
            continue;
        }
        let edge_data = body.get_edge(edge).ok_or(ReplaceFaceError::Corrupt)?;
        let he_plus = edge_data.he_plus;
        let start = body
            .get_half_edge(he_plus)
            .ok_or(ReplaceFaceError::Corrupt)?
            .start;
        let end = body
            .half_edge_end(he_plus)
            .ok_or(ReplaceFaceError::Corrupt)?;
        let at = |v: VertexKey| moved.iter().find(|(k, _)| *k == v).map(|(_, p)| *p);
        let (new_start, new_end) = (at(start), at(end));
        if new_start.is_none() && new_end.is_none() {
            continue;
        }
        let curve = body
            .get_curve_geom(edge_data.curve)
            .and_then(crate::null::CurveGeom::certified)
            .ok_or(ReplaceFaceError::Corrupt)?;
        let carrier = curve.carrier().clone();
        let (mut t0, mut t1) = curve.params();
        let mut description = *curve.description();
        for (point, is_start) in [(new_start, true), (new_end, false)] {
            let Some(point) = point else { continue };
            let t_old = if is_start { t0 } else { t1 };
            let t_new = invert_carrier(&carrier, point, t_old).ok_or(
                ReplaceFaceError::CarrierLaneUnsupported {
                    edge,
                    what: "a re-anchored carrier that is neither a line nor a circle",
                },
            )?;
            let gap = carrier.eval(t_new).distance(point);
            match decide(
                "offset_reanchor_on_carrier",
                Margin::of(T::from_f64(tol.eps()) - gap),
                band,
            )
            .map_err(|source| ReplaceFaceError::Escalated { source })?
            {
                Sign::Positive | Sign::Zero => {}
                Sign::Negative => return Err(ReplaceFaceError::ReanchorOffCarrier { edge, gap }),
            }
            if let EdgeGeometry::MappedCurve(m) = description {
                description =
                    EdgeGeometry::MappedCurve(move_mapped_endpoint(m, point, is_start).ok_or(
                        ReplaceFaceError::CarrierLaneUnsupported {
                            edge,
                            what: "a re-anchored mapped description that is not a placed line \
                                   segment (an arc's bulge and a trajectory's family are sketch \
                                   data this door does not author)",
                        },
                    )?);
            }
            if is_start {
                t0 = t_new;
            } else {
                t1 = t_new;
            }
        }
        // **The witness follows the parameter range.** An intrinsic
        // description's witness is pinned to the edge's MID-PARAMETER
        // point, so re-anchoring an endpoint moves the point the
        // witness has to be — a stored witness from the old range
        // fails `WitnessMidpoint` at the very gate that re-attaches it.
        // The carrier did not move, so the new witness is that carrier
        // read at the new midpoint.
        let mid = carrier.eval((t0 + t1) * T::from_f64(0.5));
        description = match description {
            EdgeGeometry::Intersection { s1, s2, .. } => EdgeGeometry::Intersection {
                s1,
                s2,
                witness: mid,
            },
            EdgeGeometry::TangentIntersection { s1, s2, .. } => EdgeGeometry::TangentIntersection {
                s1,
                s2,
                witness: mid,
            },
            other => other,
        };
        out.push((
            edge,
            EdgeCurveSpec {
                description,
                carrier,
                param_start: t0,
                param_end: t1,
            },
        ));
    }
    Ok(out)
}

/// The parameter of `p` on `carrier`, on the branch nearest `near` —
/// closed form on the two kinds whose inverse is one, and `None`
/// otherwise (a spline's inversion is a solve, which is a different
/// unit's machinery).
fn invert_carrier<T: Real>(carrier: &Curve3<T>, p: Point3<T>, near: T) -> Option<T> {
    match carrier {
        Curve3::Line { origin, dir } => Some((p - *origin).dot(*dir)),
        Curve3::Circle {
            center,
            axis,
            u_ref,
            ..
        } => {
            let v_ref = axis.cross(*u_ref);
            let w = p - *center;
            let theta = w.dot(v_ref).atan2(w.dot(*u_ref));
            // Pick the 2π branch nearest the parameter this endpoint
            // had: the stored range is the traversed arc, not a
            // canonical one.
            let tau = T::tau();
            let k = ((near - theta) / tau + T::from_f64(0.5)).floor();
            Some(theta + k * tau)
        }
        Curve3::Ellipse { .. } | Curve3::Nurbs(_) => None,
    }
}

/// `mapped` with the sketch endpoint that images `is_start` moved to
/// `point` — the authoritative sketch datum re-stated, not the carrier
/// patched around it. `None` for anything but a placed line segment.
pub(crate) fn move_mapped_endpoint<T: Real>(
    mapped: geom_brep::MappedCurve<T>,
    point: Point3<T>,
    is_start: bool,
) -> Option<geom_brep::MappedCurve<T>> {
    let geom_brep::MappedCurve::PlacedSegment {
        segment: geom_brep::SketchSegment::Line { a, b },
        place,
    } = mapped
    else {
        return None;
    };
    let q = place.inverse().transform_point(point);
    let moved = geom_core::Point2::new(q.x, q.y);
    Some(geom_brep::MappedCurve::PlacedSegment {
        segment: geom_brep::SketchSegment::Line {
            a: if is_start { moved } else { a },
            b: if is_start { b } else { moved },
        },
        place,
    })
}

/// The two faces an edge separates (they coincide on a seam).
///
/// **One of two spellings in this crate, and the disclosure is the
/// resolution**: `Body::edge_faces` in `merge_faces` takes the two
/// HALF-EDGES (it already has them from the edge walk) where this one
/// takes the edge key and looks them up. Same three link hops, two
/// call shapes; the shell verb uses THIS one. Collapsing them would
/// mean giving one caller an argument it does not have.
pub(crate) fn edge_faces<T: Real>(body: &Body<T>, edge: EdgeKey) -> Option<(FaceKey, FaceKey)> {
    let e = body.get_edge(edge)?;
    let face_of =
        |he| -> Option<FaceKey> { Some(body.get_loop(body.get_half_edge(he)?.parent_loop)?.face) };
    Some((face_of(e.he_plus)?, face_of(e.he_minus)?))
}
