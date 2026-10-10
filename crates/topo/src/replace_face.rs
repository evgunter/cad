//! `replace_face_offset` — the face-replacement primitive: one face's
//! surface becomes its certified offset, and the face's own boundary is
//! re-described against the moved chart.
//!
//! The **offset** of a surface `S` at signed distance `d` is the normal
//! pushforward `S_d(u, v) = S(u, v) + d·n(u, v)`
//! (`geom_brep::offset_surface`'s definition). At THIS door `d` is
//! along the chart normal AT THE FACES BEING MOVED, which is the stored
//! normal for every kind but a cone below its apex, where the face's
//! own normal is the stored field negated. The door turns `d` onto the
//! mint's convention at its entrance, by the chart's nappe
//! ([`crate::offset_nappe::group_nappe`]), and everything downstream —
//! the mint, the parameter shift, the transport and the apex-window
//! gate — reads that one turned number. The face's `sense` bit takes no
//! part: the offset is a statement about the SURFACE, not about which
//! side of it carries material.
//!
//! **So the door takes construction state, not a finished body.** Its
//! argument is stated against a chart alone, and a door of that kind
//! takes a [`Body`], tier 2 in and tier 2 out; a door whose argument
//! means something about material takes an [`crate::AtRestBody`]
//! (`crates/topo/README.md`, "Shell and offset surgery"). Its result
//! becomes finished only through [`crate::AtRestBody::validate`]: on an
//! inside-out body the chart moves as it would on any other, and the
//! result refuses there as the operand would, `NegativeVolume`.
//!
//! # What moves and what does not
//!
//! **The neighbours' surfaces are untouched.** Only the named face's
//! surface is replaced; every other face keeps the chart it had.
//!
//! **This door is one chart of the general simultaneous door**
//! ([`crate::offset_surfaces_together`]), and its body is that door's:
//! every chart named is minted first, an edge is planned against the
//! moved surfaces on both of its sides, and a corner is solved against
//! every surface meeting it as moved. What follows is stated for one
//! moved chart; where a second moved chart is the edge's other side,
//! the edge is transported where both sides' offset actions agree on it
//! (a G1 pair holding the move, two coplanar charts moved alike) and is
//! otherwise the section of the two MOVED surfaces through C5.
//!
//! **What the move does to an edge depends on the neighbour.** An edge
//! between the moved surface and a held one is their SECTION
//! ([`crate::offset_derive`]): the C5 arm's closed form, or for a plane
//! against a spline wall the wall's exact row or the march's certified
//! branch, nearest the old edge and running with it. Rigid transport
//! (the carrier lanes below) is the section only where the held surface
//! is carried onto itself by the move — a plane cap moved along its
//! normal beside a wall that contains it — and the door transports
//! there and nowhere else between distinct surfaces. Two sides of one
//! chart (a seam, a wrap, an iso image) move with the chart. An edge so
//! derived is stated as the two surfaces' `Intersection`, its sketch
//! record dropped: it is not a curve the sketch drew.
//!
//! **A moved corner is solved, not transported.** Where every held
//! surface around a corner holds the move, the transports put it where
//! it is. Anywhere else it is a root: of the moved surface along each
//! untouched edge meeting the corner, of each held surface along each
//! moved edge — the corner slides along a slanted seam rather than
//! along the moved surface's normal — and every candidate must agree
//! with every other ([`ReplaceFaceError::VertexDisagreement`]). A
//! surface grazing the edge, or meeting it nowhere near the corner,
//! refuses by the root's own verdict
//! ([`ReplaceFaceError::CornerSection`]). Along a spline carrier, an
//! end within ε of the surface is the root
//! ([`ReplaceFaceError::ReanchorPastCarrierEnd`] past ε). Along a
//! derived spline section the root is sought from the section's end at
//! that corner, the section running with the old edge, and a surface
//! its lane does not root contributes no root. A spline or fitted
//! surface is never the one rooted, so a moved fitted face's corner
//! stands on the other surfaces' roots along its sections; a moved
//! surface that no root at its corner lies on refuses the corner by
//! name ([`ReplaceFaceError::CornerSection`]).
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
//! That parameter is the root the corner was solved by, or, at a
//! transported corner, read per carrier kind: an analytic carrier's
//! closed-form inverse anchored at the endpoint's old parameter
//! (`Curve3::param_near` — line, circle, ellipse, spiric), and a spline
//! carrier's Newton foot from the same anchor
//! (`geom_brep::NurbsLane::carrier_foot`, read off
//! [`crate::AtRestPolicy::nurbs_lane`]). What refuses is
//! [`ReplaceFaceError::VertexDisagreement`] (the point the read names
//! is not the corner), [`ReplaceFaceError::ReanchorPastCarrierEnd`] (the
//! corner lies past the carrier's end),
//! [`ReplaceFaceError::ReanchorInconclusive`] (Newton did not converge)
//! and [`ReplaceFaceError::NurbsLaneUnsupported`] (the scalar holds no
//! lane, or no section lane for a spline edge's section or root).
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
//! double cone. So a `v < 0` face's surface moves `−d` along its own
//! chart normal — which is why this door turns the caller's number
//! before the mint sees it, from the chart's own decided nappe. What it
//! REFUSES is a chart with no nappe to turn onto: a face whose corners
//! reach its apex, or a chart whose faces do not all lie on one side of
//! it ([`ReplaceFaceError::NappeStraddles`]).
//!
//! **The translating lanes accept a spline carrier**, not only a line:
//! a translated control net is exact structure, so a `Curve3::Nurbs`
//! on a plane, a cylinder ruling, a cone generator or a fitted chart
//! transports with everything else.
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
//! description nothing can certify. A pair that routes is then asked
//! about its POSE (`geom_brep::route_pose`): the arms are
//! configuration-scoped, and an offset can carry the moved surface out
//! of the configuration its arm serves — a wedge cap through a cone's
//! apex, moved off it, cuts a hyperbola — so the door refuses that
//! pose by the arm's own grounds rather than admitting it on the kind
//! pair's. A served pose whose section the door does not derive, has
//! no branch near the edge or is a tangency there refuses by the
//! section's own verdict ([`ReplaceFaceError::EdgeSection`]), and an
//! edge still scaffolded that the move tilts against a distinct
//! neighbour refuses by name ([`ReplaceFaceError::DeclaredEdgeTilted`]).
//! A fitted face routes as its fit: its edge with a held plane is
//! derived as their section, a row of its fit beside an analytic face
//! is extracted from the new fit, and every other edge of it refuses by
//! name ([`ReplaceFaceError::FittedBoundaryUnsupported`] lists them).
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
//! window's near end is its supremum. WHICH nappe that is comes from
//! [`crate::offset_nappe::face_nappe`], the one home the door itself
//! reads to turn `d`; the window says only whether that nappe's near
//! end has cleared the apex, and one that has not — because it
//! straddles, touches, or carries a face of the other nappe — refuses
//! on the same variant.
//!
//! **That shift is where a TRANSPORTED rim lands, and a derived one need
//! not.** A rim that is a section with a held surface stays on that
//! surface while the apex slides, so once the boundary is derived the
//! window is read again, off the moved cone and the carriers the face
//! now has, and its near end must still clear the moved apex — the same
//! predicate and the same refusal, stated on the base chart.
//!
//! # Discipline
//!
//! Every decision — the mint, the refusals, the whole boundary plan —
//! runs read-only against the incoming body. Mutation then runs on a
//! clone in the attach layer's order (surface, then edge descriptions,
//! then the whole-body pcurve mint), the clone is validated once at
//! tier 2 — closed, every edge certified on its charts — and only a
//! clone that passes is adopted. The body is untouched on every `Err`.
//! Tier 2 does not read orientation: a move that carries a face through
//! its neighbours, inverting the body, is adopted, and the at-rest
//! validator refuses it on the rings the move left outside the face
//! ([`crate::ValidationError::RingOutsideOuter`];
//! `crates/sweep/tests/a_move_through_a_neighbour_inverts_the_body.rs`).

use std::sync::Arc;

use geom::SurfaceKind;
use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec, Nappe};
use geom_core::k_stats::decide;
use geom_core::{
    Affine3, Band, BandError, Decide, Indeterminate, KERNEL_DEFECT_ENDING,
    KERNEL_OR_FILE_DEFECT_ENDING, Margin, NO_DECLARATION_RECOURSE, NOT_YET_ENDING, Point3, Real,
    Sign, Tol, Vec3,
};

use crate::attach::Rechart;
use crate::body::Body;
use crate::chart_groups::ChartGroups;
use crate::entity::{EdgeKey, EntityId, FaceKey, GeomRef, SolidKey, VertexKey};
use crate::euler::EulerOpError;
use crate::geometry::SurfaceKey;
use crate::live::{dangling_link, linked, proven};
use crate::pcurves::{PcurveMintError, mint_pcurves};
use crate::validate::{ValidationError, validate_closed};

/// Typed refusal of the face-replacement door. Scalar payloads echo the
/// classified quantity's ingredients — data, not a decision (the offset
/// door's echo convention, one layer up).
#[derive(Clone, Debug)]
pub enum ReplaceFaceError<T: Real> {
    /// The run's tolerance admits no linear band, so no margined
    /// predicate on the face-replacement doors has a verdict to give.
    /// [`replace_faces_offset`] derives its band at the door from the
    /// tolerance witness alone; this is that derivation's refusal. Both
    /// of `Band::linear`'s arms reach it from a tolerance the run's
    /// validator admits (an ε within a factor K of `f64::MAX`, or a
    /// subnormal ε with K near 1).
    Band {
        /// The band constructor's typed refusal.
        error: BandError,
    },
    /// `face`, a key the caller handed over, does not resolve in the
    /// body. A key the body's own records hold that does not resolve is
    /// a torn body, and the door panics naming the record (D2 row 4).
    StaleFace {
        /// The unresolvable face.
        face: FaceKey,
    },
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
    /// The face carries a NURBS surface and the mint was handed no
    /// fit door, so the offset cannot be minted at all. Not a pass —
    /// the same posture tier 3 takes on an unre-derivable certificate.
    ///
    /// Where `Some` comes from, and what its absence means:
    /// [`crate::AtRestPolicy::offset_fit_lane`].
    ApproxLaneUnsupported {
        /// The face whose kind needs the (`f64`-only) fit door.
        face: FaceKey,
        /// The scalar the mint ran at ([`geom_core::Real::NAME`]).
        scalar: &'static str,
    },
    /// **The operand's surface key is SHARED within its solid.** Another
    /// face of the same solid carries the same surface, so replacing the
    /// named faces would re-point their boundary's descriptions at the
    /// fresh key while the sharer keeps the old chart — a seam between
    /// them would name one face's surface and lie on the other's.
    /// Replacing a solid's wearers of a chart is a whole-group
    /// operation: name every one.
    SharedSurfaceKey {
        /// The face the door was called on.
        face: FaceKey,
        /// A face of the same solid carrying the same surface key.
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
    /// **The nappe predicate** (`offset_nappe`, at
    /// [`crate::offset_nappe`]). Either this cone face's own corners do
    /// not all stand strictly on one side of its apex, or the faces of
    /// one chart do not agree on a nappe — in both readings there is no
    /// single side for the offset's sign to be turned onto. Refused
    /// rather than guessed, at both offset doors.
    NappeStraddles {
        /// The face with no nappe, or the first group member that
        /// disagreed.
        face: FaceKey,
        /// That face's least corner station, echoed as data.
        station_min: T,
        /// Its greatest.
        station_max: T,
        /// Which of the two readings it is.
        what: &'static str,
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
        /// The replaced face's new surface kind. A fitted face names
        /// [`SurfaceKind::Approx`] here and routed as its fit's kind,
        /// [`SurfaceKind::Nurbs`], which the message says.
        kind: SurfaceKind,
        /// The untouched neighbour's kind, read as `kind` is.
        other_kind: SurfaceKind,
    },
    /// **The C5 boundary, asked about the POSE.** The pair has a route
    /// arm, but that arm is configuration-scoped and the moved surface
    /// stands against its untouched neighbour in a pose the arm does
    /// not serve — an offset wedge cap no longer through a cone's apex
    /// or a torus's axis, say — so the edge cannot be re-stated as an
    /// intersection of the two either.
    NeighborPoseUnroutable {
        /// The edge that cannot be re-described.
        edge: EdgeKey,
        /// The replaced face's new surface kind.
        kind: SurfaceKind,
        /// The untouched neighbour's kind.
        other_kind: SurfaceKind,
        /// Why the pose is not served: the arm's own refusal text
        /// where it gave one (a general-rung routing, an operand guard
        /// that fired before the pose was classified), and
        /// `route_pose`'s statement where the arm's refusal carries no
        /// text (unequal cylinder radii, a torus off its ring
        /// convention).
        why: &'static str,
    },
    /// **The fitted face's boundary this door does not carry.** A moved
    /// fitted face keeps an edge it meets a distinct held plane along
    /// (their section, derived) and a row of its fit whose other side
    /// is analytic. It refuses, named per edge with what the edge
    /// presented:
    /// - a row of its fit shared with a spline or another fitted face,
    ///   which would have to move with it;
    /// - a curve on its fit that does not run along its fitted rows;
    /// - a curve still under construction (a scaffold edge);
    /// - a seam the face shares with itself.
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
    /// **An edge between the moved surface and a held one has no
    /// section this door can state**: the C5 arm or the spline-wall
    /// march refused, the section has no branch near the edge, or it
    /// is a tangency there. The edge is that section wherever the held
    /// surface is not carried onto itself by the move, so nothing else
    /// stands in for it.
    EdgeSection {
        /// The edge whose section was refused.
        edge: EdgeKey,
        /// The replaced face's new surface kind.
        kind: SurfaceKind,
        /// The held neighbour's kind.
        other_kind: SurfaceKind,
        /// The section's own verdict.
        verdict: crate::offset_derive::SectionVerdict,
    },
    /// **A moved corner is not a root of the moved surface along an
    /// edge meeting it** (or of a held surface along a moved edge): the
    /// surface grazes the edge's carrier, does not meet it near the
    /// corner, or meets it where this door solves no root.
    CornerSection {
        /// The corner.
        vertex: VertexKey,
        /// The edge the root was sought along.
        edge: EdgeKey,
        /// The root's own verdict.
        verdict: crate::offset_derive::CornerVerdict<T>,
    },
    /// **A scaffolded edge between two distinct surfaces that the move
    /// tilts.** The edge is still under construction — its sketch
    /// record IS its description — and the held neighbour is not carried
    /// onto itself by the move, so the moved edge is a section the
    /// record does not describe and there is no other description to
    /// state it by. Unreachable at rest, where tier 3's transience
    /// fence refuses a scaffold.
    DeclaredEdgeTilted {
        /// The scaffolded edge.
        edge: EdgeKey,
    },
    /// **A moved vertex ran past the end of a spline carrier.** Newton's
    /// foot on an untouched edge from the endpoint's old parameter is
    /// stopped by the domain clamp at the carrier's end, `gap` from the
    /// moved point; or a corner's root along a spline carrier — an
    /// untouched edge's, or a derived plane × fit section's, whose
    /// domain ends at the fit's window — lies past the carrier's near
    /// end, `gap` from the surface. This door re-anchors an
    /// endpoint on the carrier the edge has and does not extend one, so
    /// an outward offset meets this wherever the carrier ends at the
    /// moved face — a lofted wall's seam, whose carrier ends where its
    /// patch does, a carrier minted only as long as its edge, and a
    /// section of a fit no wider than its face.
    ReanchorPastCarrierEnd {
        /// The edge whose carrier ends short of the moved vertex.
        edge: EdgeKey,
        /// The distance from the moved point to the carrier's end, in
        /// meters.
        gap: T,
    },
    /// **The offset collapses an untouched edge.** An edge beside the
    /// moved face ends at a moved vertex, and the re-anchored end
    /// reaches or passes the edge's other end — an inward move as deep
    /// as the wall beside it is tall, say. The attach door's span check
    /// decides it on a line or a spline
    /// ([`geom_brep::CertifyError::IntervalNotForward`]), and a spline
    /// carrier's clamp at its far end decides a move past it; either
    /// way it is the move's length, not a span the kernel minted wrong.
    /// A periodic carrier's non-forward span may be a turn chosen
    /// wrongly, so it stays the attach door's [`ReplaceFaceError::Op`].
    ReanchorCollapse {
        /// The edge the move collapses or reverses.
        edge: EdgeKey,
        /// The offset distance, as given.
        offset: T,
    },
    /// A spline carrier's foot-point Newton did not converge from the
    /// moved endpoint's old parameter. No foot is offered in its place.
    ReanchorInconclusive {
        /// The edge whose carrier the foot was sought on.
        edge: EdgeKey,
        /// The projection's own refusal, verbatim.
        error: geom::ProjectionInconclusive,
    },
    /// An edge ending at a moved vertex rides a spline carrier, and the
    /// scalar the door ran at holds no NURBS lane to read its foot
    /// point with ([`crate::AtRestPolicy::nurbs_lane`] answers `None` —
    /// a dual, DL1). A fact about the scalar, not the body.
    NurbsLaneUnsupported {
        /// The edge whose carrier needs the lane.
        edge: EdgeKey,
        /// The scalar the door ran at ([`geom_core::Real::NAME`]).
        scalar: &'static str,
    },
    /// **The simultaneous door's scope gate**: a face it was asked to
    /// move is not a plane. Its corner solve is three plane equations,
    /// and a curved face has no such equation;
    /// [`crate::offset_surfaces_together`] roots those corners.
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
    ///
    /// Raised by BOTH simultaneous doors, which is why its Display
    /// names the OPERATION rather than one of them: a message that said
    /// `offset_planes_together` under a body of revolution would send a
    /// reader to the wrong file.
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
    /// **The simultaneous door: an edge's re-derived geometry
    /// disagrees with a moved surface or carrier by `gap`.** Distinct
    /// from [`ReplaceFaceError::VertexDisagreement`], which is the
    /// per-face door's finding about one corner's candidates. This one
    /// is the simultaneous door's verification net over every edge it re-derives, and THREE
    /// meters raise it — two about an endpoint, one about the carrier
    /// itself:
    ///
    /// - `offset_together_edge_agreement` (the planar door): the far
    ///   endpoint — solved independently, against a different triple
    ///   of planes — read onto the carried line; `gap` is its distance
    ///   off that line. Two solves agreeing is the claim; this is it
    ///   failing.
    /// - `offset_axial_edge_agreement` (the axial door's `param_on`):
    ///   a moved endpoint read onto the minted carrier; `gap` is the
    ///   endpoint's distance off it — the same two-solves claim, one
    ///   carrier kind wider.
    /// - `offset_axial_edge_on_surface` (the axial door): the minted
    ///   carrier's own MIDPOINT metered against each of the two moved
    ///   surfaces the edge separates; `gap` is the midpoint's residual
    ///   to the surface that refused. No endpoint pair is compared at
    ///   this meter at all — the carrier itself stands off a surface
    ///   it claims to lie in. (The sphere lune's pre-RIMCAP refusal
    ///   was this meter, on the axis edge between its two moved caps —
    ///   the site an earlier draft of this doc misdescribed as an
    ///   endpoint disagreement.)
    TogetherEdgeDisagreement {
        /// The edge whose re-derived geometry failed verification.
        edge: EdgeKey,
        /// The disagreement the raising meter measured, in meters.
        gap: T,
    },
    /// **The axial door's kind gate**: a face wears a surface that is
    /// not a plane, cylinder, cone, sphere or TORUS. The axial reduction
    /// reads each surface as a line or a circle in the meridian
    /// half-plane, and a NURBS or a fitted chart is neither — such
    /// bodies take [`crate::offset_surfaces_together`] instead.
    ///
    /// A coaxial torus IS one of the kinds: its meridian is the circle
    /// centred `(R, h_c)`, the sphere's circle centred `(0, h_c)` with
    /// one more number. An off-axis torus refuses at the axis gate
    /// below, not here.
    TogetherAxialUnsupported {
        /// The face.
        face: FaceKey,
        /// What it carries.
        kind: SurfaceKind,
    },
    /// **The axial door's axis gate**: a face's surface is one of the
    /// door's kinds but is not a surface of revolution about the body's
    /// axis — a cylinder skew to it, a sphere or torus centred off it, a
    /// plane cutting it obliquely. The meridian reduction has no coordinates
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
        /// The edge the attach door refused; absent for the re-chart,
        /// whose refusal names the edges itself.
        edge: Option<EdgeKey>,
        /// The attach layer's typed refusal.
        error: EulerOpError,
    },
    /// The whole-body pcurve mint refused on the re-described body.
    Pcurve {
        /// The mint's typed refusal.
        source: PcurveMintError,
    },
    /// The join the public offset doors end with
    /// ([`Body::join_edges`]) refused on the offset body. The body is
    /// untouched.
    Join {
        /// Why the join refused.
        refusal: crate::boolean::JoinRefusal,
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
            // The carrier's own repairs (set a positive ε; raise ε or
            // K) are addressed to a caller choosing a band's
            // thresholds; a caller here holds a valid tolerance whose
            // derived band failed anyway, and a less extreme ε forms
            // a band at any admitted K.
            Self::Band { .. } => write!(
                f,
                "the run's tolerance is too extreme for the ambiguity band above it to form. \
                 Recourse: run at a less extreme tolerance"
            ),
            Self::StaleFace { .. } => write!(
                f,
                "the face to offset is not in the body. Recourse: name a face of this body"
            ),
            // Each door's refusal names what it is about itself.
            Self::Offset { error, .. } => write!(f, "{error}"),
            Self::Fit { error, .. } => write!(f, "{error}"),
            Self::IsoRow { error, .. } => write!(f, "{error}"),
            Self::ApproxLaneUnsupported { scalar, .. } => write!(
                f,
                "the face is a spline surface, whose offset is fitted, and the {scalar} scalar \
                 this run uses cannot fit one; only {holders} can. Recourse: offset the face at \
                 {holders}",
                holders = geom_brep::ScalarList(geom_brep::OFFSET_FIT_DOOR_HOLDERS),
            ),
            Self::SharedSurfaceKey { .. } => write!(
                f,
                "the face shares its surface with another face of the same solid, and the \
                 faces on one surface move together. Recourse: offset every face on that \
                 surface at once"
            ),
            Self::EmptyGroup => write!(
                f,
                "no face was named, so there is nothing to offset. Recourse: name a face"
            ),
            Self::GroupChartsDiffer { .. } => write!(
                f,
                "the faces named to offset together lie on different surfaces, and one offset \
                 moves one surface. Recourse: name only faces that share a surface"
            ),
            Self::PlaceholderSurface { .. } => write!(
                f,
                "the face has no surface described yet, so there is nothing to offset. \
                 {KERNEL_OR_FILE_DEFECT_ENDING}"
            ),
            Self::ApexWindow { shift, .. } => write!(
                f,
                "an offset that shifts the cone face {shift:?} m along its slant carries part \
                 of it to or past the cone's apex, where it is no longer this face's offset. \
                 Recourse: use a smaller offset"
            ),
            Self::NappeStraddles {
                station_min,
                station_max,
                what,
                ..
            } => write!(
                f,
                "{what}; the corners run from {station_min:?} m to {station_max:?} m about the \
                 cone's apex. {NOT_YET_ENDING}"
            ),
            Self::ApexWindowUnknown { .. } => write!(
                f,
                "the cone face has no boundary curve to read its extent from, so whether an \
                 offset reaches the apex cannot be decided. {KERNEL_OR_FILE_DEFECT_ENDING}"
            ),
            Self::NeighborPairUnroutable {
                kind, other_kind, ..
            } => {
                // A fitted surface routes as its fit, a spline: say so,
                // since the arm that refused is the spline one.
                let as_fit = |k: SurfaceKind| match k {
                    SurfaceKind::Approx => " (met as its spline fit)",
                    _ => "",
                };
                write!(
                    f,
                    "the offset {} face{} would meet a {} neighbour{} along a curve the kernel \
                     cannot describe yet, so the edge between them cannot follow. \
                     {NOT_YET_ENDING}",
                    kind.adjective(),
                    as_fit(*kind),
                    other_kind.adjective(),
                    as_fit(*other_kind),
                )
            }
            Self::NeighborPoseUnroutable {
                kind, other_kind, ..
            } => write!(
                f,
                "the offset {} face would stand against a {} neighbour in a position whose \
                 meeting curve the kernel cannot describe yet, so the edge between them cannot \
                 follow. {NOT_YET_ENDING}",
                kind.adjective(),
                other_kind.adjective()
            ),
            Self::FittedBoundaryUnsupported { what, .. } => write!(
                f,
                "an edge of the fitted face is {what}, which the face cannot carry through the \
                 move. {NOT_YET_ENDING}"
            ),
            Self::CarrierLaneUnsupported { what, .. } => write!(
                f,
                "an edge beside the face cannot follow the offset: {what}. {NOT_YET_ENDING}"
            ),
            // The carriers moved are valid splines already, so the
            // spline layer's reason (in `Debug`) is the kernel's.
            Self::Structure { .. } => write!(
                f,
                "an edge's moved curve is not a valid spline. {KERNEL_DEFECT_ENDING}"
            ),
            Self::VertexDisagreement { gap, .. } => write!(
                f,
                "two edges move one corner of the face to points {gap:?} m apart, so the \
                 corner has no one place to go. {NOT_YET_ENDING}"
            ),
            Self::EdgeSection {
                kind,
                other_kind,
                verdict,
                ..
            } => write!(
                f,
                "the offset {} face meets a {} neighbour along a curve the kernel cannot \
                 state: {verdict}. {NOT_YET_ENDING}",
                kind.adjective(),
                other_kind.adjective()
            ),
            Self::CornerSection { verdict, .. } => write!(
                f,
                "a corner of the face has no place on an edge beside it: {verdict}. \
                 {NOT_YET_ENDING}"
            ),
            Self::DeclaredEdgeTilted { .. } => write!(
                f,
                "an edge still under construction lies between the face and a neighbour the \
                 offset tilts it against, so the moved edge is no longer the curve its sketch \
                 drew. {NOT_YET_ENDING}"
            ),
            Self::ReanchorPastCarrierEnd { gap, .. } => write!(
                f,
                "a corner of the face moves {gap:?} m past the end of the curve of an edge \
                 beside it, and the offset does not lengthen that curve. {NOT_YET_ENDING}"
            ),
            Self::ReanchorCollapse { offset, .. } => write!(
                f,
                "an offset of {offset:?} m moves a corner of the face to or past the far end of \
                 an edge beside it, which would leave that edge no length. Recourse: use an \
                 offset shorter than that edge"
            ),
            Self::ReanchorInconclusive { .. } => write!(
                f,
                "a moved corner of the face could not be placed on the curve of an edge beside \
                 it. {NOT_YET_ENDING}"
            ),
            Self::NurbsLaneUnsupported { scalar, .. } => write!(
                f,
                "an edge beside the face is a spline, and the {scalar} scalar this run uses \
                 cannot place a moved corner on one. Recourse: offset the body at a \
                 certifying scalar"
            ),
            Self::TogetherAxialUnsupported { kind, .. } => write!(
                f,
                "a face of this body of revolution lies on a {} surface, whose offset has no \
                 profile curve to move. {NOT_YET_ENDING}",
                kind.adjective()
            ),
            Self::TogetherNotAxial { what, .. } => write!(
                f,
                "a face is {what}, so it does not turn about the body's axis and its corners \
                 cannot be moved in profile. {NOT_YET_ENDING}"
            ),
            Self::TogetherAxialCorner { surfaces, what, .. } => write!(
                f,
                "a corner where {surfaces} moved surfaces meet has no offset point: {what}. \
                 {NOT_YET_ENDING}"
            ),
            Self::TogetherAxialEdge { what, .. } => write!(
                f,
                "an edge of the body cannot follow the offset: {what}. {NOT_YET_ENDING}"
            ),
            // No offset door takes a declaration, so the escalation's
            // own lever is the geometry alone.
            Self::Escalated { source } => write!(
                f,
                "whether the offset face and its edges land where they should is too close to \
                 call: {}",
                source.under(NO_DECLARATION_RECOURSE)
            ),
            Self::Op {
                edge: Some(_),
                error,
            } => {
                write!(
                    f,
                    "an edge beside the moved face could not be rebuilt: {error}"
                )
            }
            Self::Op { edge: None, error } => write!(
                f,
                "the moved faces could not be put onto their offset surface: {error}"
            ),
            Self::Pcurve { source } => write!(
                f,
                "the offset body's edges could not be parametrized on their faces: {source}"
            ),
            Self::Join { refusal } => write!(f, "offsetting the face: {refusal}"),
            Self::TogetherChartMixed { .. } => write!(
                f,
                "a move names faces that do not lie on one surface, and a move moves one \
                 surface. Recourse: give each surface's faces a move of their own"
            ),
            Self::TogetherFaceRepeated { .. } => write!(
                f,
                "a face is named by more than one move, so its offset would be two numbers. \
                 Recourse: name each face in one move"
            ),
            Self::TogetherEdgeDisagreement { gap, .. } => write!(
                f,
                "an edge's new curve is {gap:?} m out of agreement with the moved faces: a \
                 solved endpoint stands off its curve, or the curve's midpoint stands off a moved \
                 surface. {NOT_YET_ENDING}"
            ),
            Self::TogetherNonPlanar { kind, .. } => write!(
                f,
                "a face to move lies on a {} surface, and moving the corners together needs \
                 every face to be flat. {NOT_YET_ENDING}",
                kind.adjective()
            ),
            Self::TogetherPartialSet { .. } => write!(
                f,
                "a face of the body is in no move, and each corner's new place depends on \
                 every face meeting it. Recourse: name every face, with an offset of zero for \
                 a face that stays"
            ),
            Self::TogetherCorner { planes, what, .. } => write!(
                f,
                "a corner where {planes} planes meet has no offset point: {what}. \
                 {NOT_YET_ENDING}"
            ),
            Self::ResultNotClosed { errors } => write!(
                f,
                "the offset body is not valid ({}), so it is discarded. {KERNEL_DEFECT_ENDING}",
                error_count(errors.len())
            ),
        }
    }
}

impl<T: Real> std::error::Error for ReplaceFaceError<T> {}

/// "1 error", "3 errors": a validator report's size, as a refusal says it.
pub(crate) fn error_count(n: usize) -> String {
    if n == 1 {
        "1 error".to_owned()
    } else {
        format!("{n} errors")
    }
}

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
    nappe: Nappe,
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
            // Neither an ellipse nor a spiric lies on a cylinder.
            Curve3::Ellipse { .. } | Curve3::Spiric { .. } => None,
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
                    let delta = action.displacement(nappe, mid);
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
                // Neither an ellipse nor a spiric lies on a cone.
                Curve3::Ellipse { .. } | Curve3::Spiric { .. } => None,
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
        Curve3::Spiric {
            center,
            axis,
            u_ref,
            major_radius,
            minor_radius,
            offset,
        } => Curve3::Spiric {
            center: *center + delta,
            axis: *axis,
            u_ref: *u_ref,
            major_radius: *major_radius,
            minor_radius: *minor_radius,
            offset: *offset,
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
        // A line, an ellipse or a spiric does not lie on a sphere, so
        // a carrier of any of those kinds is not a curve this map was
        // derived for.
        Curve3::Line { .. } | Curve3::Ellipse { .. } | Curve3::Spiric { .. } => return Ok(None),
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
    /// Where a transport put the edge's two ends; `None` for a derived
    /// section, whose ends are its corners' and whose parameters are
    /// read there.
    ends: Option<(Point3<T>, Point3<T>)>,
    /// The surface keys on the edge's two sides, the moved chart's
    /// standing for the surface it moves to.
    sides: [SurfaceKey; 2],
    /// A section refused: raised once the corners are solved, which name
    /// a move that runs past an edge's end more precisely than the
    /// section that then has no branch.
    refused: Option<ReplaceFaceError<T>>,
}

/// Replaces `face`'s surface with its certified offset at signed
/// distance `d` and re-describes the face's boundary against the moved
/// chart (module docs).
///
/// `d` is along the chart's normal AT THIS FACE. On a cone's mirror
/// nappe that is the negation of the stored `v > 0` field the mint
/// moves along, and the door turns it (`crate::offset_nappe`) before
/// anything is minted.
///
/// The run's ε arrives as the [`Tol`] witness alone, and the door
/// derives the run's linear band from it once, so one call classifies
/// at one ε by construction. The analytic kinds mint in closed form
/// and read the band only for their margined decisions (the mint's
/// radius floor and torus ring convention, this door's apex window and
/// nappe); the NURBS lane hands the witness
/// to the fit door (`geom_brep::approx_offset_surface`), which fits to
/// the run's ε_precision and meters at the band it derives from the
/// same witness. The boundary re-derivation reads the same band.
///
/// The body is **untouched on every `Err`**: the mint, the refusals and
/// the whole boundary plan are decided read-only, the mutation runs on
/// a clone, and the clone is adopted only after it validates at tier 2
/// (closure and certification, not orientation: module docs,
/// "Discipline").
///
/// The door **ends with the join** (`docs/DESIGN.md`, maximal edges):
/// the moved body is joined on the clone before it is adopted, and the
/// joins are returned ([`OffsetOutcome`]).
///
/// # Errors
///
/// [`ReplaceFaceError`] — [`ReplaceFaceError::Band`] when the run's
/// tolerance forms no linear band, the offset door's own refusals, the
/// fit door's, the apex-window predicate, the C5 routing boundary, the
/// carrier lanes' scope, a re-derivation the attach layer's
/// certification rejects, and a clone that does not validate.
pub fn replace_face_offset<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    face: FaceKey,
    d: T,
    tol: Tol,
) -> Result<OffsetOutcome, ReplaceFaceError<T>> {
    replace_faces_offset(body, &[face], d, tol)
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
/// **The whole group is the solid's.** What the door protects is that
/// no edge joins a re-keyed wearer to one left on the old key. Every
/// edge lies in one shell and so in one solid, so a wearer on another
/// solid shares no edge with the group and keeps the old chart: a
/// chart is body-wide, and the group is its wearers within the solids
/// `faces` lie on.
///
/// `faces` must be exactly those wearers — not a subset (the refusal
/// above) and not a mixture of charts
/// ([`ReplaceFaceError::GroupChartsDiffer`]).
///
/// The door **ends with the join** (`docs/DESIGN.md`, maximal edges):
/// the moved body is joined on the clone before it is adopted, and the
/// joins are returned ([`OffsetOutcome`]).
///
/// # Errors
///
/// [`ReplaceFaceError`] — [`replace_face_offset`]'s, plus the group
/// gates.
pub fn replace_faces_offset<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    faces: &[FaceKey],
    d: T,
    tol: Tol,
) -> Result<OffsetOutcome, ReplaceFaceError<T>> {
    replace_faces_offset_staged(body, faces, d, tol, true).map(|joins| OffsetOutcome { joins })
}

/// **Each boundary edge's planned verdict** for moving one non-cone
/// `face` by `d`: the door's own mint and `plan_edge`, one entry per
/// edge in the door's order, each `Ok(None)` where the edge plans,
/// `Ok(Some(refusal))` where its section refusal is deferred to the
/// corners, and `Err` where planning it refuses outright. The door
/// stops at the first `Err` and raises a deferred refusal only once the
/// corners are solved, so a row about one edge's own verdict reads it
/// here rather than behind whichever edge answers first.
///
/// # Panics
///
/// On a cone face (its nappe and apex window are the door's, not
/// this hook's), a stale face, or an offset that does not mint.
#[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
#[doc(hidden)]
#[allow(clippy::panic, clippy::expect_used, clippy::type_complexity)]
pub fn offset_edge_plans_for_tests<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    d: T,
    tol: Tol,
) -> Vec<(
    EdgeKey,
    Result<Option<ReplaceFaceError<T>>, ReplaceFaceError<T>>,
)> {
    let band = Band::linear(tol).expect("the witness band");
    let face_data = body.get_face(face).expect("a live face");
    let old_key = face_data.surface;
    let old_surface = body.face_surface_linked(face, face_data).clone();
    assert!(
        !matches!(old_surface, Surface::Cone { .. }),
        "a cone's nappe and apex window are the door's"
    );
    let new_surface = mint_offset(
        face,
        &old_surface,
        d,
        band,
        tol,
        <T as crate::props::AtRestPolicy>::offset_fit_lane(),
    )
    .unwrap_or_else(|e| panic!("the offset mints: {e}"));
    let faces = [face];
    let chart = MovedChart {
        faces: &faces,
        old_key,
        shift: apex_shift(&old_surface, d),
        old_surface,
        new_surface,
        nappe: Nappe::Opening,
        d,
    };
    group_boundary(body, &faces)
        .into_iter()
        .map(|edge| {
            let plan = plan_edge(
                body,
                edge,
                core::slice::from_ref(&chart),
                band,
                tol,
                T::section_lane(),
            );
            (edge, plan.map(|p| p.refused))
        })
        .collect()
}

/// **The lever each derived section hands its corners' roots** for
/// moving one non-cone `face` by `d`: every boundary edge the move
/// derives, once, with its planned carrier and the arm
/// [`incident_edges`] gives the lane ([`corner_arm`]).
///
/// # Panics
///
/// As [`offset_edge_plans_for_tests`], and where an edge's plan refuses.
#[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
#[doc(hidden)]
#[allow(clippy::panic, clippy::expect_used)]
pub fn offset_corner_arms_for_tests<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    d: T,
    tol: Tol,
) -> Vec<(EdgeKey, Curve3<T>, T)> {
    let band = Band::linear(tol).expect("the witness band");
    let face_data = body.get_face(face).expect("a live face");
    let old_key = face_data.surface;
    let old_surface = body.face_surface_linked(face, face_data).clone();
    let new_surface = mint_offset(
        face,
        &old_surface,
        d,
        band,
        tol,
        <T as crate::props::AtRestPolicy>::offset_fit_lane(),
    )
    .unwrap_or_else(|e| panic!("the offset mints: {e}"));
    let faces = [face];
    let chart = MovedChart {
        faces: &faces,
        old_key,
        shift: apex_shift(&old_surface, d),
        old_surface,
        new_surface,
        nappe: Nappe::Opening,
        d,
    };
    let boundary = group_boundary(body, &faces);
    let plans: Vec<EdgePlan<T>> = boundary
        .iter()
        .map(|&edge| {
            plan_edge(
                body,
                edge,
                core::slice::from_ref(&chart),
                band,
                tol,
                T::section_lane(),
            )
            .unwrap_or_else(|e| panic!("{edge:?} plans: {e}"))
        })
        .collect();
    let mut keys: Vec<VertexKey> = Vec::new();
    for plan in &plans {
        for v in [plan.start, plan.end] {
            if !keys.contains(&v) {
                keys.push(v);
            }
        }
    }
    let mut out: Vec<(EdgeKey, Curve3<T>, T)> = Vec::new();
    for group in group_by_point(body, keys) {
        let old_point = *body
            .get_point(proven(&body.vertices, group[0], EntityId::Vertex).point)
            .expect("a live vertex's point");
        let incident = incident_edges(body, &plans, &boundary, &group, old_point)
            .unwrap_or_else(|e| panic!("the corner's edges read: {e}"));
        for inc in incident.into_iter().filter(|i| i.derived) {
            if !out.iter().any(|(e, ..)| *e == inc.edge) {
                out.push((inc.edge, inc.carrier, inc.extent));
            }
        }
    }
    out
}

/// What a public offset door did to the body's topology: the joins it
/// ended with ([`Body::join_edges`], `docs/DESIGN.md`, maximal edges),
/// in the order made, each killed `vertex` and `gone` edge held by
/// `kept`. Empty where the moved body held no joinable vertex.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OffsetOutcome {
    /// The joins, in the order made.
    pub joins: Vec<crate::boolean::EdgeJoin>,
}

/// The join an offset door ends with where `join` is set, on its staged
/// body before it is adopted, over the vertices `within` holds: the
/// door's own scope, so a vertex of an entity the call does not write
/// is neither joined nor read ([`Body::join_edges_within`]). The
/// staging is the door's, so a refusal discards the half-joined clone
/// with it.
pub(crate) fn staged_join<T: Decide + crate::props::AtRestPolicy>(
    staged: &mut Body<T>,
    join: bool,
    tol: Tol,
    within: &dyn Fn(crate::VertexKey) -> bool,
) -> Result<Vec<crate::boolean::EdgeJoin>, ReplaceFaceError<T>> {
    if !join {
        return Ok(Vec::new());
    }
    let band = Band::linear(tol).map_err(|error| ReplaceFaceError::Band { error })?;
    staged
        .join_edges_within(band, tol, within)
        .map_err(|refusal| ReplaceFaceError::Join {
            refusal: crate::boolean::JoinRefusal::of(&refusal),
        })
}

/// [`replace_faces_offset`], ending with the join where `join` is set.
/// Unset, the result is construction state a later step must join: the
/// shell's lift offset, which keys its naming rows by the moved body's
/// cells.
pub(crate) fn replace_faces_offset_staged<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    faces: &[FaceKey],
    d: T,
    tol: Tol,
    join: bool,
) -> Result<Vec<crate::boolean::EdgeJoin>, ReplaceFaceError<T>> {
    offset_charts_staged(body, &[(faces, d)], tol, join)
}

/// One chart the door moves, resolved and minted before anything is
/// written.
pub(crate) struct MovedChart<'a, T: Real> {
    /// Its wearers.
    faces: &'a [FaceKey],
    old_key: SurfaceKey,
    old_surface: Surface<T>,
    new_surface: Surface<T>,
    nappe: Nappe,
    /// The offset in the mint's convention: the caller's number turned
    /// by the chart's nappe.
    d: T,
    /// The cone's `d·cot α` parameter shift; zero on every other kind.
    shift: T,
}

/// The moved chart wearing `key`, if one does.
fn moved_of<'c, 'a, T: Real>(
    charts: &'c [MovedChart<'a, T>],
    key: SurfaceKey,
) -> Option<&'c MovedChart<'a, T>> {
    charts.iter().find(|c| c.old_key == key)
}

/// **The offset door's one body**: every chart `moves` names — its
/// wearers and its signed distance — moved AT ONCE, every new surface
/// minted before any edge is planned. An edge between a moved chart and
/// a held one, or between two moved charts, is planned against the
/// moved surfaces on both sides ([`plan_edge`]), and every moved corner
/// is solved against every surface meeting it as moved
/// ([`solve_corners`]). One move is [`replace_faces_offset`]; every
/// chart of a solid is [`crate::offset_surfaces_together`].
pub(crate) fn offset_charts_staged<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    moves: &[(&[FaceKey], T)],
    tol: Tol,
    join: bool,
) -> Result<Vec<crate::boolean::EdgeJoin>, ReplaceFaceError<T>> {
    // The one band every decision below classifies at, derived from the
    // same witness the fit door reads.
    let band = Band::linear(tol).map_err(|error| ReplaceFaceError::Band { error })?;
    // ---- Decide: the groups. ----
    if moves.is_empty() {
        return Err(ReplaceFaceError::EmptyGroup);
    }
    let mut solids: Vec<SolidKey> = Vec::new();
    for &(faces, _) in moves {
        let Some(&face) = faces.first() else {
            return Err(ReplaceFaceError::EmptyGroup);
        };
        let old_key = body
            .get_face(face)
            .ok_or(ReplaceFaceError::StaleFace { face })?
            .surface;
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
        // The group must be the WHOLE group within its solids: a wearer
        // left behind there could share an edge with a re-keyed one.
        // Every member resolved above; its `shell` is a link of its record.
        for &member in faces {
            let shell = proven(&body.faces, member, EntityId::Face).shell;
            let solid = linked(
                &body.shells,
                shell,
                EntityId::Shell,
                EntityId::Face(member),
                "shell",
            )
            .solid;
            if !solids.contains(&solid) {
                solids.push(solid);
            }
        }
    }
    let scope = crate::offset_together::Scope::of_solids(body, &solids).faces_in_scope();
    let groups = ChartGroups::within(body, scope).unwrap_or_else(|face| {
        unreachable!("{face:?}, walked out of its solid's shells, resolved in that walk")
    });
    let mut charts: Vec<MovedChart<'_, T>> = Vec::with_capacity(moves.len());
    for &(faces, d) in moves {
        let face = faces[0];
        let face_data = proven(&body.faces, face, EntityId::Face);
        let old_key = face_data.surface;
        if let Some(&other) = groups.of(old_key).iter().find(|k| !faces.contains(k)) {
            return Err(ReplaceFaceError::SharedSurfaceKey { face, other });
        }
        let old_surface = body.face_surface_linked(face, face_data).clone();
        // **The cone's mirror nappe is a consumer obligation, and this is
        // where this door discharges it.** `geom_brep::ConeOffset`'s
        // action moves the surface along the OPENING nappe's normal
        // field, so a mirror-nappe face's surface moves `−d` along its
        // own chart normal; `d` arrives at this door along the FACE's
        // chart normal, so the two conventions are opposite below the
        // apex and the number has to be turned over before it reaches
        // the mint. The nappe is decided at its one home, from the
        // face's own corners.
        //
        // EVERY face of the group is decided and the answers are agreed
        // ([`crate::offset_nappe::group_nappe`]), because the door moves
        // a chart and a chart's faces need not share a nappe.
        //
        // Only a cone is asked: every other chart has ONE sheet, on
        // which the face's own normal and the mint's stored field are
        // the same direction.
        //
        // Below this line `d` is the mint's own convention, not the door's.
        let nappe = match old_surface {
            Surface::Cone { .. } => crate::offset_nappe::group_nappe(body, faces, band)?,
            _ => Nappe::Opening,
        };
        let d = nappe.turn(d);
        let new_surface = mint_offset(
            face,
            &old_surface,
            d,
            band,
            tol,
            <T as crate::props::AtRestPolicy>::offset_fit_lane(),
        )?;

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
            // The nappe decides which end of the window is the one
            // nearest the apex; the window decides whether that end has
            // CLEARED the apex. One predicate, on the near end alone,
            // and a window that has not cleared it (because it
            // straddles, touches, or carries a face of the other nappe)
            // is refused before the collapse margin is taken.
            let esc = |source| ReplaceFaceError::Escalated { source };
            let (v_near, sense) = match nappe {
                Nappe::Opening => (v_min, T::one()),
                Nappe::Mirror => (v_max, -T::one()),
            };
            match decide("offset_apex_nappe", Margin::of(v_near * sense), band).map_err(esc)? {
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
        charts.push(MovedChart {
            faces,
            old_key,
            old_surface,
            new_surface,
            nappe,
            d,
            shift,
        });
    }
    // ---- Decide: the boundary plan. ----
    let mut boundary: Vec<EdgeKey> = Vec::new();
    for chart in &charts {
        for edge in group_boundary(body, chart.faces) {
            if !boundary.contains(&edge) {
                boundary.push(edge);
            }
        }
    }
    let mut plans: Vec<EdgePlan<T>> = Vec::with_capacity(boundary.len());
    for &edge in &boundary {
        plans.push(plan_edge(
            body,
            edge,
            &charts,
            band,
            tol,
            T::section_lane(),
        )?);
    }

    // ---- Decide: where each moved corner lands, and the agreement. ----
    let corners = solve_corners(
        body,
        &plans,
        &boundary,
        &charts,
        band,
        tol,
        T::section_lane(),
    )?;
    let moved: Vec<(VertexKey, Point3<T>)> = corners
        .groups
        .iter()
        .flat_map(|(group, point)| group.iter().map(|&v| (v, *point)))
        .collect();
    let vertex_groups = corners.groups.clone();
    for plan in &mut plans {
        if let Some(refused) = plan.refused.take() {
            return Err(refused);
        }
        let rooted = |v| corners.rooted.contains(&v);
        if plan.ends.is_none() {
            read_ends(plan, &moved, None, band, tol, T::nurbs_lane())?;
        } else if rooted(plan.start) || rooted(plan.end) {
            // A transported edge whose corner a root placed ends there.
            let seeds = (plan.spec.param_start, plan.spec.param_end);
            read_ends(plan, &moved, Some(seeds), band, tol, T::nurbs_lane())?;
        }
    }
    // ---- Decide: the apex window again, on the boundary as derived. ----
    // The window above shifts the face's old rims by the action's own
    // shift, which is where a TRANSPORTED rim lands. A derived rim is a
    // section with a neighbour and lands where that surface puts it,
    // so the window is read again off the moved cone and the carriers
    // the face now has: its near end must still clear the moved apex.
    for chart in &charts {
        let Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } = chart.new_surface
        else {
            continue;
        };
        let face = chart.faces[0];
        let own = group_boundary(body, chart.faces);
        let mut window: Option<(T, T)> = None;
        for plan in plans.iter().filter(|p| own.contains(&p.edge)) {
            let (lo, hi) = cone_v_range(
                &plan.spec.carrier,
                plan.spec.param_start,
                plan.spec.param_end,
                apex,
                axis,
                half_angle.cos(),
            );
            window = Some(match window {
                None => (lo, hi),
                Some((a, b)) => (a.min(lo), b.max(hi)),
            });
        }
        let (v_min, v_max) = window.ok_or(ReplaceFaceError::ApexWindowUnknown { face })?;
        let (v_near, sense) = match chart.nappe {
            Nappe::Opening => (v_min, T::one()),
            Nappe::Mirror => (v_max, -T::one()),
        };
        match decide("offset_apex_window", Margin::of(v_near * sense), band)
            .map_err(|source| ReplaceFaceError::Escalated { source })?
        {
            Sign::Positive => {}
            // Stated on the base chart, as the gate above states it: the
            // moved window less the shift.
            Sign::Zero | Sign::Negative => {
                return Err(ReplaceFaceError::ApexWindow {
                    face,
                    v_min: v_min - chart.shift,
                    v_max: v_max - chart.shift,
                    shift: chart.shift,
                });
            }
        }
    }

    // ---- Decide: the incident edges that only need re-anchoring. ----
    let anchored = plan_reanchors(body, &boundary, &corners, band, tol, T::nurbs_lane())?;

    // ---- Mutation, on a clone (infallible decisions are done). ----
    //
    // The clone is under a surgery scope for the whole of it: the
    // setters below are this door's operator sequence, and the door's
    // whole-body check is the tier-2 gate the clone is adopted on.
    let mut staged = body.clone();
    let mut work = staged.begin_surgery();
    // Each group moves onto one new chart: it wore one surface before
    // and wears one after, which is what keeps its shared seams
    // describable, and a plan's old key stands for that chart.
    let mut recharts: Vec<Rechart<T>> = Vec::with_capacity(charts.len());
    for chart in &charts {
        recharts.push(offset_rechart(
            &work,
            chart.new_surface.clone(),
            chart.faces,
        )?);
    }
    let specs: Vec<(EdgeKey, EdgeCurveSpec<T>)> = plans
        .into_iter()
        .map(|plan| (plan.edge, plan.spec))
        .collect();
    // The edges this call rewrites. A vertex neither of whose edges is
    // one of them stands as the operand stated it, and is not this
    // call's to join, so these edges' ends are every vertex the closing
    // join reads.
    let written: Vec<EdgeKey> = specs
        .iter()
        .map(|(e, _)| *e)
        .chain(anchored.iter().map(|(e, _, _)| *e))
        .collect();
    move_points_then_rechart(&mut work, &vertex_groups, recharts, &specs, tol)?;
    // A row is stated over its edge's interval, which ends at the
    // edge's vertices, so the move stales every row of an edge that
    // ends at a moved vertex. They go before the re-anchors' site mints
    // could keep them; the closing mint re-derives every face. That
    // includes a fitted row on a neighbour that keeps its chart, which
    // the closing mint would otherwise carry (`pcurves::carry_rows`):
    // its image ends on the chart point of the vertex where it was
    // stated, so once that vertex is displaced past the band the carry
    // re-certifies it against an edge that ends elsewhere and refuses.
    // A vertex in `moved` that lands within the band of where it was
    // keeps a carryable row that this drop loses
    // (`work/shell/replace-faces-offset-drops-rows-at-a-vertex-the-move-leaves-in-place`).
    let staled: Vec<_> = work
        .half_edges
        .values()
        .filter(|h| moved.iter().any(|&(v, _)| v == h.start))
        .flat_map(|h| {
            let edge = proven(&work.edges, h.edge, EntityId::Edge);
            [edge.he_plus, edge.he_minus]
        })
        .collect();
    work.drop_rows(staled);
    for (edge, spec, unwound) in anchored {
        work.set_edge_curve(edge, spec, tol)
            .map_err(|error| match (error, unwound) {
                // The edge was forward before the re-anchor and its
                // carrier has no turn to choose, so a span that is not
                // forward is the move's length.
                (
                    EulerOpError::Certification {
                        error: geom_brep::CertifyError::IntervalNotForward { .. },
                    },
                    Some(offset),
                ) => ReplaceFaceError::ReanchorCollapse { edge, offset },
                (error, _) => ReplaceFaceError::Op {
                    edge: Some(edge),
                    error: error.from_driver(),
                },
            })?;
    }
    mint_pcurves(&mut work, tol).map_err(|source| ReplaceFaceError::Pcurve { source })?;
    work.sweep_and_close();
    validate_closed(&staged).map_err(|errors| ReplaceFaceError::ResultNotClosed { errors })?;
    let ends: std::collections::BTreeSet<crate::VertexKey> = written
        .iter()
        .filter_map(|&e| staged.get_edge(e))
        .flat_map(|e| [e.he_plus, e.he_minus])
        .filter_map(|h| staged.get_half_edge(h).map(|h| h.start))
        .collect();
    let joins = staged_join(&mut staged, join, tol, &|v| ends.contains(&v))?;
    body.adopt(staged);
    Ok(joins)
}

/// The offset surface for `old`: the analytic mint, or the fit door's
/// certified `Approx` where the kind is not closed under offset.
///
/// `offset_fit` is that door ([`geom_brep::OffsetFitLane`]), handed in
/// as a parameter; what a `None` means is
/// [`crate::AtRestPolicy::offset_fit_lane`]'s subject. `None` is not a
/// pass — a caller that cannot mint the offset refuses with
/// [`ReplaceFaceError::ApproxLaneUnsupported`].
// `band` is the one [`replace_faces_offset`] derived from `tol`: the
// analytic arm classifies at it, and the fit door re-derives the same
// band from the witness it is handed.
fn mint_offset<T: Decide>(
    face: FaceKey,
    old: &Surface<T>,
    d: T,
    band: Band,
    tol: Tol,
    offset_fit: Option<geom_brep::OffsetFitLane<T>>,
) -> Result<Surface<T>, ReplaceFaceError<T>> {
    if let Surface::Nurbs(base) = old {
        if base.is_placeholder() {
            return Err(ReplaceFaceError::PlaceholderSurface { face });
        }
        return match offset_fit {
            None => Err(ReplaceFaceError::ApproxLaneUnsupported {
                face,
                scalar: T::NAME,
            }),
            Some(lane) => lane
                .mint(Arc::clone(base), d, tol)
                .map(Surface::Approx)
                .map_err(|error| ReplaceFaceError::Fit { face, error }),
        };
    }
    // `d` is `geom_brep::offset_surface`'s own convention here, turned
    // at the door by the face's nappe (`crate::offset_nappe`) — the
    // mint is nappe-blind by contract and this call does not re-read it.
    geom_brep::offset_surface(old, d, band)
        .map_err(|error| ReplaceFaceError::Offset { face, error })
}

/// **Whether the arm serves this pose** at the C5 gate: the moved
/// surface against the untouched one, read over the edge's carrier on
/// `[t0, t1]` ([`geom_brep::Reach::Span`]). Every arm is levered by the
/// carrier's exact per-carrier distance from its pivot, never by a ball
/// around the edge: a looser lever decides an in-band pose as served
/// ([`geom_brep::intersect::route_pose`]).
fn pose_route<T: Decide>(
    moved: &Surface<T>,
    other: &Surface<T>,
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
    band: Band,
) -> Result<geom_brep::intersect::PairRoute, geom_brep::SectionError> {
    let reach = geom_brep::Reach::Span {
        carrier: carrier.clone(),
        t0,
        t1,
    };
    geom_brep::intersect::route_pose(moved, other, &reach, band)
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
    for edge in group_boundary(body, group) {
        let edge_data = proven(&body.edges, edge, EntityId::Edge);
        let curve = body.edge_curve_linked(edge, edge_data).certified()?;
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
        // A spiric lies on no cone, and this range is asked of a cone
        // face's own boundary edges — every one certified on its chart
        // at rest — so a spiric here is a kernel bug, not a body's.
        Curve3::Spiric { .. } => {
            unreachable!("cone_v_range: a spiric carrier bounds no cone face")
        }
    }
}

/// The group's boundary edges, in face-then-loop-then-cycle order,
/// each once — including the seams INTERNAL to the group, which move
/// with the chart exactly as its outer edges do. Every member resolved
/// at the door, and the walk from it reads links, so a miss panics
/// naming the record ([`Body::face_cycles_linked`]).
#[track_caller]
fn group_boundary<T: geom_core::Decide>(body: &Body<T>, group: &[FaceKey]) -> Vec<EdgeKey> {
    let mut out: Vec<EdgeKey> = Vec::new();
    for &face in group {
        for he in body.face_cycles_linked(face) {
            let edge = proven(&body.half_edges, he, EntityId::HalfEdge).edge;
            if !out.contains(&edge) {
                out.push(edge);
            }
        }
    }
    out
}

/// One boundary edge's re-derivation: the description re-stated against
/// the moved chart, the carrier transported, the endpoints read off the
/// transported carrier — or, between two distinct moved charts,
/// [`plan_between_moved`].
fn plan_edge<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    charts: &[MovedChart<'_, T>],
    band: Band,
    tol: Tol,
    section_lane: Option<crate::offset_derive::SectionLane<T>>,
) -> Result<EdgePlan<T>, ReplaceFaceError<T>> {
    let edge_data = proven(&body.edges, edge, EntityId::Edge);
    let sides = edge_side_keys(body, edge, edge_data);
    let he_plus = edge_data.he_plus;
    let start = linked(
        &body.half_edges,
        he_plus,
        EntityId::HalfEdge,
        EntityId::Edge(edge),
        "he_plus",
    )
    .start;
    let end = body.proven_half_edge_end(he_plus);
    let Some(curve) = body.edge_curve_linked(edge, edge_data).certified() else {
        return Err(ReplaceFaceError::CarrierLaneUnsupported {
            edge,
            what: "it has no curve to move",
        });
    };
    let mover = match (moved_of(charts, sides[0]), moved_of(charts, sides[1])) {
        (Some(a), Some(b)) if a.old_key != b.old_key => {
            return plan_between_moved(
                (edge, start, end, sides),
                curve,
                (a, b),
                band,
                tol,
                section_lane,
            );
        }
        (Some(mover), _) | (None, Some(mover)) => mover,
        (None, None) => unreachable!("{edge:?} bounds a moved chart, which is why it was planned"),
    };
    let (group, old_key, old_surface, new_surface, nappe, d, shift) = (
        mover.faces,
        mover.old_key,
        &mover.old_surface,
        &mover.new_surface,
        mover.nappe,
        mover.d,
        mover.shift,
    );
    let (t0, t1) = curve.params();
    let old_carrier = curve.carrier().clone();
    let description = curve.description().clone();
    let mid = curve.mid_point();

    // The one description that gets an EXACT carrier rather than a
    // transported one: an iso-curve of a fitted chart is a row of the
    // fit's own control net, so extracting it lands the carrier in the
    // fit's spline space — its degree and its refined interior knots —
    // without elevating or refining anything.
    if let (EdgeDescription::Chart(c), Surface::Approx(approx)) = (&description, new_surface)
        && c.surface == old_key
        && let geom_brep::Pcurve::IsoLine { p0, pl } = c.pcurve
    {
        // An iso image on a DESCRIPTION is u-const by construction —
        // `EdgeDescriptionSpec::iso` is the only door that mints one,
        // and it fixes `u` and moves `v`. (The u-moving `IsoLine` the
        // cap-rim lane mints is a stored CACHE, never a description.)
        let (u, v0, v1) = (p0.x, p0.y + pl.y * t0, p0.y + pl.y * t1);
        // The seam this row carries is shared with whatever face
        // sits on the other side. If THAT face is a bounded chart
        // too, it would have to move with this one to keep holding
        // the edge — which is a body-wide offset, not a
        // face-replacement, and this door says so rather than
        // storing a row the neighbour's own lane will reject.
        let (fa, fb) = crate::readback::edge_sides_of(body, edge, edge_data).faces();
        let other = if group.contains(&fa) { fb } else { fa };
        if !group.contains(&other) {
            let what =
                match body.face_surface_linked(other, proven(&body.faces, other, EntityId::Face)) {
                    Surface::Nurbs(_) => Some("a row of this fit shared with a spline face"),
                    Surface::Approx(_) => Some("a row of this fit shared with another fitted face"),
                    _ => None,
                };
            if let Some(what) = what {
                return Err(ReplaceFaceError::FittedBoundaryUnsupported { edge, what });
            }
        }
        // The extraction itself lives in `geom_brep::nurbs_iso`, beside
        // `boundary_iso_u` and its asserting rows: the door's lane is
        // the call, not the arithmetic.
        // **The fourth home of the same question** (PCURVE P-1b, found
        // in review). This early return mints its own spec and never
        // reaches `carried_declaration` below, so it too would answer
        // `declared: None` and destroy the record. There is no `delta`
        // here to transport with — the carrier is extracted from the
        // NEW fit's control net rather than transported — and a fit's
        // offset is not a rigid translation in any case, so the honest
        // answer for a declared locus is the same refusal the other
        // arms give rather than a silent drop.
        //
        // Latent today: an iso boundary of a face's own fit is minted
        // by `nurbs_iso_derive`, which declares nothing, so no current
        // fixture carries a declaration here. Written anyway, because
        // "no fixture reaches it" is exactly what was true of the
        // boundary lane's drop until a cap offset reached it.
        if curve.authority().is_declared() {
            return Err(ReplaceFaceError::CarrierLaneUnsupported {
                edge,
                what: "its declared sketch record cannot follow the face, whose offset is not a \
                       rigid shift",
            });
        }
        let (row, u_domain) = geom_brep::iso_boundary_row(approx.fit(), u, band)
            .map_err(|error| ReplaceFaceError::IsoRow { edge, error })?;
        let carrier = Curve3::Nurbs(Arc::new(row));
        let ends = Some((carrier.eval(v0), carrier.eval(v1)));
        return Ok(EdgePlan {
            edge,
            spec: EdgeCurveSpec {
                description: EdgeDescriptionSpec::iso(old_key, u_domain, v0, v1, v0, v1),
                carrier,
                param_start: v0,
                param_end: v1,
            },
            start,
            end,
            ends,
            sides,
            refused: None,
        });
    }

    // Past the fit's own rows, a fitted face keeps only the edges it
    // meets a DISTINCT held surface along, each derived below as the
    // section of the fit with that surface where the pair routes (a
    // plane); every other lane transports a carrier off the chart that
    // is supposed to hold it.
    if matches!(new_surface, Surface::Approx(_)) {
        let what = match description {
            EdgeDescription::Chart(ref c) if c.surface == old_key => {
                Some("a curve on this face's fit that does not run along its fitted rows")
            }
            EdgeDescription::Scaffold(_) => Some("a curve still under construction"),
            EdgeDescription::Chart(_)
            | EdgeDescription::Intersection { .. }
            | EdgeDescription::TangentIntersection { .. } => {
                (sides[0] == sides[1]).then_some("a seam the fitted face shares with itself")
            }
        };
        if let Some(what) = what {
            return Err(ReplaceFaceError::FittedBoundaryUnsupported { edge, what });
        }
    }

    // **Between the moved surface and a distinct held one, the edge is
    // their section** — transported only where the held surface is
    // carried onto itself by the move (`offset_derive`'s shortcut).
    let other_key = if sides[0] == old_key {
        sides[1]
    } else {
        sides[0]
    };
    if other_key != old_key {
        let held = body.get_surface(other_key).unwrap_or_else(|| {
            dangling_link(EntityId::Edge(edge), "face", GeomRef::Surface(other_key))
        });
        let extent = geom_brep::edge_extent(
            &old_carrier,
            t0,
            t1,
            old_carrier.eval(t0).distance(old_carrier.eval(t1)),
        );
        if !crate::offset_derive::holds_the_move(old_surface, held, d, band) {
            return derive_edge(
                (edge, start, end, sides),
                (curve, &description, (t0, t1)),
                (old_key, other_key, new_surface, held),
                extent,
                band,
                section_lane,
            );
        }
    }

    let (carrier, delta) = transport_curve(old_surface, nappe, d, &old_carrier, mid, band)
        .map_err(|e| match e {
            TransportError::Escalated(source) => ReplaceFaceError::Escalated { source },
            TransportError::Structure(error) => ReplaceFaceError::Structure { edge, error },
        })?
        .ok_or(ReplaceFaceError::CarrierLaneUnsupported {
            edge,
            what: "its kind of curve has no exact offset on this kind of surface",
        })?;
    let new_mid = carrier.mid_point(t0, t1);

    // **The declaring pushforward travels with the face** (PCURVE
    // P-1b), and it travels the same way whichever arm below the
    // edge's description takes — so it is answered once, here, rather
    // than three times inside the match.
    //
    // U2 split what used to be one datum in two. The LOCUS is a chart
    // image, stated in the chart's own coordinates, so the offset
    // re-parameterizes the chart and the image needs no transport —
    // that argument is what let this unit retire the *"not a rigid
    // translation"* refusal for conventional edges, and it is right.
    // The DECLARATION beside it is the other half: a `MappedCurve`,
    // sketch data under a 3-SPACE placement, which is exactly the
    // thing that must be carried bodily with the face. Before the
    // collapse that payload WAS the description and went down the
    // scaffolding arm, which translated it; writing `declared: None`
    // at its new home silently destroyed it for every edge the fence
    // had converted.
    //
    // Dropping it also flips `EdgeAuthority::is_declared`, which tier
    // 3's prefer-intrinsic rules read — a verdict change.
    //
    // The `delta` requirement is the pre-collapse one, unchanged and
    // for the pre-collapse reason: a pushforward can only be carried
    // when the offset is a rigid translation of a family that
    // translates. It is asked for ONLY when a declaration is actually
    // present, so an edge whose locus nothing declared still crosses a
    // non-translating offset freely — which is what the retirement
    // bought, and this keeps it.
    let carried_declaration =
        || -> Result<Option<geom_brep::MappedCurve<T>>, ReplaceFaceError<T>> {
            match curve.authority() {
                geom_brep::EdgeAuthority::Derived => Ok(None),
                geom_brep::EdgeAuthority::Declared(mc) => {
                    let delta = delta.ok_or(ReplaceFaceError::CarrierLaneUnsupported {
                        edge,
                        what: "its declared sketch record cannot follow the face, whose offset \
                               is not a rigid shift",
                    })?;
                    Ok(Some(translate_mapped(mc, delta).ok_or(
                        ReplaceFaceError::CarrierLaneUnsupported {
                            edge,
                            what: "it was swept by a rotation, and its sweep does not shift with \
                                   the face",
                        },
                    )?))
                }
            }
        };

    // Whether the section of the moved chart and an untouched neighbour
    // is one this kernel can state.
    let neighbour_section = |other: SurfaceKey| -> Result<(), ReplaceFaceError<T>> {
        let other_surface = body.get_surface(other).unwrap_or_else(|| {
            dangling_link(EntityId::Edge(edge), "description", GeomRef::Surface(other))
        });
        let other_kind = other_surface.kind();
        let kind = new_surface.kind();
        if !geom_brep::intersect::route(kind, other_kind).implemented {
            return Err(ReplaceFaceError::NeighborPairUnroutable {
                edge,
                kind,
                other_kind,
            });
        }
        // The kind pair routes; the arm is asked whether it serves
        // THIS pose — the moved surface against the untouched one,
        // read over the edge's own reach.
        let posed = pose_route(new_surface, other_surface, &carrier, t0, t1, band).map_err(
            |e| match e {
                geom_brep::SectionError::Escalated(source)
                | geom_brep::SectionError::RadiusEscalated { diag: source, .. } => {
                    ReplaceFaceError::Escalated { source }
                }
                // `route_pose` returns only an escalation or a
                // dispatch naming the wrong arm or seat — this
                // kernel's own bug, not the body's (its `# Errors`);
                // every other variant is answered inside it and
                // never returned.
                other @ (geom_brep::SectionError::WrongLane { .. }
                | geom_brep::SectionError::UnequalRadii
                | geom_brep::SectionError::DegenerateOperand { .. }
                | geom_brep::SectionError::BeyondOperandExtent { .. }
                | geom_brep::SectionError::CoincidentSurfaces
                | geom_brep::SectionError::DegenerateTorus
                | geom_brep::SectionError::RoutesToGeneralRung { .. }
                | geom_brep::SectionError::Carrier(_)
                | geom_brep::SectionError::Spiric(_)) => unreachable!(
                    "{edge:?}: `route_pose` answers only an escalation or a misdispatch, and \
                 returned {other:?}"
                ),
            },
        )?;
        if !posed.implemented {
            return Err(ReplaceFaceError::NeighborPoseUnroutable {
                edge,
                kind,
                other_kind,
                why: posed.note,
            });
        }
        Ok(())
    };

    let new_description = match description {
        // A wrap edge on the moved chart is stated anew there: its
        // image is DERIVED from the transported carrier against the
        // new chart (`image: None`), as a construction states one,
        // rather than shifted, and the flag travels: its two halves
        // still bound the one face.
        EdgeDescription::Chart(ref c) if c.surface == old_key && c.wrap => {
            EdgeDescriptionSpec::Chart {
                surface: old_key,
                image: None,
                wrap: true,
                declared: carried_declaration()?,
            }
        }
        // Every other image on the MOVED chart shifts with the chart's
        // own offset action: `d·cot α` in `v` on a cone, zero on every
        // other kind.
        EdgeDescription::Chart(ref c) if c.surface == old_key => EdgeDescriptionSpec::Chart {
            surface: old_key,
            image: Some(shift_chart_v(&c.pcurve, shift).ok_or(
                ReplaceFaceError::CarrierLaneUnsupported {
                    edge,
                    what: "it is drawn on a fitted or cone surface, where the offset has no \
                           exact shift for it",
                },
            )?),
            wrap: false,
            declared: carried_declaration()?,
        },
        EdgeDescription::Intersection { s1, s2, .. }
        | EdgeDescription::TangentIntersection { s1, s2, .. }
            if s1 == old_key || s2 == old_key =>
        {
            let other = if s1 == old_key { s2 } else { s1 };
            neighbour_section(other)?;
            let tangent = matches!(description, EdgeDescription::TangentIntersection { .. });
            let (n1, n2) = if s1 == old_key {
                (old_key, s2)
            } else {
                (s1, old_key)
            };
            if tangent {
                EdgeDescriptionSpec::TangentIntersection {
                    s1: n1,
                    s2: n2,
                    witness: new_mid,
                }
            } else {
                EdgeDescriptionSpec::Intersection {
                    s1: n1,
                    s2: n2,
                    witness: new_mid,
                }
            }
        }
        // **A refusal the collapse retired for DERIVED conventional
        // edges only — narrowed, after the wider claim was published
        // and proved wrong** (PCURVE P-1b).
        //
        // Both `what`s below say the same thing about the same thing:
        // a pushforward is stated in 3-SPACE, so it has to be carried
        // bodily with the face it hangs off, and it can only be
        // carried when the offset is a rigid translation of a family
        // that translates.
        //
        // The wider claim was that a CHART IMAGE is stated in the
        // chart's own coordinates — the offset re-parameterizes the
        // chart and leaves the image untouched — so the question does
        // not arise at all and these refusals stop firing for every
        // conventional edge at rest. **The premise is right and the
        // conclusion overreached.** U2 did not delete the pushforward,
        // it MOVED it: out of the description, into the authority
        // record beside the image (Q3). The image needs no transport;
        // the declaration beside it does, and `carried_declaration`
        // above raises this same statement from that arm. So what the
        // retirement actually bought is narrower and still worth
        // having: an edge the KERNEL derived — a seam, an iso
        // boundary, a cap rim, anything with no declaring sketch
        // entity — now crosses a non-translating offset freely, where
        // before it refused.
        //
        // The wider claim looked true only because this lane was
        // silently dropping the declaration (`declared: None`), so
        // nothing was left to ask the transport question of. That is
        // recorded rather than quietly narrowed, because it shipped in
        // this PR as a "verdict that moved" and the two `demos/tour`
        // teapot rows were re-baselined onto it.
        //
        // The arm below is NOT dead code: the scaffolding door is
        // still real for edges whose surfaces do not exist yet, and it
        // is unreachable for a body AT REST because tier 3's
        // transience fence (`ValidationError::ScaffoldAtRest`) refuses
        // a scaffold there. `demos/tour`'s
        // `the_not_a_rigid_translation_door_is_unreachable_at_rest`
        // asserts both halves on the fixtures: no scaffolds (this arm
        // unreachable) AND declarations present (the other arm is what
        // answers).
        EdgeDescription::Scaffold(mapped) => {
            let delta = delta.ok_or(ReplaceFaceError::CarrierLaneUnsupported {
                edge,
                what: "its sketch record cannot follow the face, whose offset is not a rigid shift",
            })?;
            EdgeDescriptionSpec::Scaffold(translate_mapped(mapped, delta).ok_or(
                ReplaceFaceError::CarrierLaneUnsupported {
                    edge,
                    what: "it was swept by a rotation, and its sweep does not shift with the face",
                },
            )?)
        }
        // An image in an untouched neighbour's chart.
        EdgeDescription::Chart(ref c) => {
            let declared = carried_declaration()?;
            if declared.is_none() {
                neighbour_section(c.surface)?;
            }
            crate::offset_restate::held_neighbour_image(old_key, c.surface, declared, new_mid)
        }
        EdgeDescription::Intersection { s1, s2, witness } => {
            EdgeDescriptionSpec::Intersection { s1, s2, witness }
        }
        EdgeDescription::TangentIntersection { s1, s2, witness } => {
            EdgeDescriptionSpec::TangentIntersection { s1, s2, witness }
        }
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
        ends: Some((carrier.eval(t0), carrier.eval(t1))),
        sides,
        refused: None,
    })
}

/// The surface keys on `edge`'s two sides.
fn edge_side_keys<T: Real>(
    body: &Body<T>,
    edge: EdgeKey,
    data: &crate::entity::Edge,
) -> [SurfaceKey; 2] {
    let (a, b) = crate::readback::edge_sides_of(body, edge, data).faces();
    [a, b].map(|f| proven(&body.faces, f, EntityId::Face).surface)
}

/// **An edge between two distinct charts that both move.** Where the
/// pair's relative motion carries it rigidly it is transported: each
/// side's offset action is a closed form on its own chart, and where
/// the two put every point of the edge in the same place — a G1 pair
/// whose normals agree along the edge and whose distances agree, two
/// coplanar charts moved alike — that image lies on both moved
/// surfaces, so it is their section. Anywhere else the edge is the
/// section of the two MOVED surfaces through C5, seeded by the old
/// carrier for branch and sense ([`derive_edge`]).
fn plan_between_moved<T: Decide>(
    (edge, start, end, sides): (EdgeKey, VertexKey, VertexKey, [SurfaceKey; 2]),
    curve: &geom_brep::EdgeCurve<T>,
    (a, b): (&MovedChart<'_, T>, &MovedChart<'_, T>),
    band: Band,
    tol: Tol,
    section_lane: Option<crate::offset_derive::SectionLane<T>>,
) -> Result<EdgePlan<T>, ReplaceFaceError<T>> {
    let (t0, t1) = curve.params();
    let old = curve.carrier();
    let description = curve.description().clone();
    if matches!(description, EdgeDescription::Scaffold(_)) {
        return Err(ReplaceFaceError::DeclaredEdgeTilted { edge });
    }
    if let Some((carrier, delta)) = carried_alike(edge, old, (t0, t1), (a, b), band, tol)? {
        let mid = carrier.mid_point(t0, t1);
        // Both charts move, so an image in either has slid within it and
        // is derived afresh from the moved carrier; a declaration rides
        // the edge's own rigid shift.
        let carried = |mc| {
            let delta = delta.ok_or(ReplaceFaceError::CarrierLaneUnsupported {
                edge,
                what: "its declared sketch record cannot follow the face, whose offset is not a \
                       rigid shift",
            })?;
            translate_mapped(mc, delta).ok_or(ReplaceFaceError::CarrierLaneUnsupported {
                edge,
                what: "it was swept by a rotation, and its sweep does not shift with the face",
            })
        };
        let description = crate::offset_restate::restate(
            description,
            curve.authority(),
            [(a.old_key, true), (b.old_key, true)],
            true,
            mid,
            carried,
        )?;
        return Ok(EdgePlan {
            edge,
            spec: EdgeCurveSpec {
                description,
                carrier: carrier.clone(),
                param_start: t0,
                param_end: t1,
            },
            start,
            end,
            ends: Some((carrier.eval(t0), carrier.eval(t1))),
            sides,
            refused: None,
        });
    }
    let extent = geom_brep::edge_extent(old, t0, t1, old.eval(t0).distance(old.eval(t1)));
    derive_edge(
        (edge, start, end, sides),
        (curve, &description, (t0, t1)),
        (a.old_key, b.old_key, &a.new_surface, &b.new_surface),
        extent,
        band,
        section_lane,
    )
}

/// The edge's image under both sides' offset actions, where the two
/// agree: each side's transport ([`transport_curve`]) read at the
/// edge's ends and middle, and every pair within ε. `None` where
/// either side has no exact action on the carrier — a fitted or spline
/// chart's lane is a translation by its mid-domain normal, exact only
/// where that normal is constant, so it is no witness — or where the
/// two disagree.
fn carried_alike<T: Decide>(
    edge: EdgeKey,
    old: &Curve3<T>,
    (t0, t1): (T, T),
    (a, b): (&MovedChart<'_, T>, &MovedChart<'_, T>),
    band: Band,
    tol: Tol,
) -> Result<Transported<T>, ReplaceFaceError<T>> {
    let spline =
        |c: &MovedChart<'_, T>| matches!(c.old_surface, Surface::Nurbs(_) | Surface::Approx(_));
    if spline(a) || spline(b) {
        return Ok(None);
    }
    let mid = old.mid_point(t0, t1);
    let transport = |c: &MovedChart<'_, T>| {
        transport_curve(&c.old_surface, c.nappe, c.d, old, mid, band).map_err(|e| match e {
            TransportError::Escalated(source) => ReplaceFaceError::Escalated { source },
            TransportError::Structure(error) => ReplaceFaceError::Structure { edge, error },
        })
    };
    let (Some(on_a), Some((on_b, _))) = (transport(a)?, transport(b)?) else {
        return Ok(None);
    };
    let tm = (t0 + t1) * T::from_f64(0.5);
    for t in [t0, tm, t1] {
        if !gap_within_eps(on_a.0.eval(t).distance(on_b.eval(t)), tol, band)? {
            return Ok(None);
        }
    }
    Ok(Some(on_a))
}

/// **An edge between the moved surface and a neighbour the move does not
/// carry onto itself** — a held surface, or another moved one: their
/// section, nearest the old carrier and
/// running with it, stated as their `Intersection` with any sketch
/// record beside it dropped — the moved edge is not the curve a sketch
/// drew. Its ends are its corners', read once those are solved.
#[allow(clippy::type_complexity)]
fn derive_edge<T: Decide>(
    (edge, start, end, sides): (EdgeKey, VertexKey, VertexKey, [SurfaceKey; 2]),
    (curve, description, (t0, t1)): (&geom_brep::EdgeCurve<T>, &EdgeDescription<T>, (T, T)),
    (old_key, other_key, new_surface, held): (SurfaceKey, SurfaceKey, &Surface<T>, &Surface<T>),
    extent: T,
    band: Band,
    section_lane: Option<crate::offset_derive::SectionLane<T>>,
) -> Result<EdgePlan<T>, ReplaceFaceError<T>> {
    use crate::offset_derive::SectionVerdict;
    if matches!(description, EdgeDescription::Scaffold(_)) {
        return Err(ReplaceFaceError::DeclaredEdgeTilted { edge });
    }
    let (kind, other_kind) = (new_surface.kind(), held.kind());
    if !geom_brep::intersect::route(kind, other_kind).implemented {
        return Err(ReplaceFaceError::NeighborPairUnroutable {
            edge,
            kind,
            other_kind,
        });
    }
    let refused = |verdict| ReplaceFaceError::EdgeSection {
        edge,
        kind,
        other_kind,
        verdict,
    };
    if matches!(description, EdgeDescription::TangentIntersection { .. }) {
        return Err(refused(SectionVerdict::Tangent));
    }
    let old = curve.carrier();
    // The kind pair routes; the arm is asked whether it serves this
    // pose, over the edge's reach.
    let posed = pose_route(new_surface, held, old, t0, t1, band).map_err(|e| match e {
        geom_brep::SectionError::Escalated(source)
        | geom_brep::SectionError::RadiusEscalated { diag: source, .. } => {
            ReplaceFaceError::Escalated { source }
        }
        // As at `neighbour_section`: every other variant is answered
        // inside `route_pose` and never returned.
        other @ (geom_brep::SectionError::WrongLane { .. }
        | geom_brep::SectionError::UnequalRadii
        | geom_brep::SectionError::DegenerateOperand { .. }
        | geom_brep::SectionError::BeyondOperandExtent { .. }
        | geom_brep::SectionError::CoincidentSurfaces
        | geom_brep::SectionError::DegenerateTorus
        | geom_brep::SectionError::RoutesToGeneralRung { .. }
        | geom_brep::SectionError::Carrier(_)
        | geom_brep::SectionError::Spiric(_)) => unreachable!(
            "{edge:?}: `route_pose` answers only an escalation or a misdispatch, and returned \
             {other:?}"
        ),
    })?;
    if !posed.implemented {
        return Err(ReplaceFaceError::NeighborPoseUnroutable {
            edge,
            kind,
            other_kind,
            why: posed.note,
        });
    }
    let esc = |source| ReplaceFaceError::Escalated { source };
    // A fitted surface is its fit as a section operand: its spline
    // chart, as for evaluation and pcurves.
    let plane_wall = match (new_surface, held) {
        (plane @ Surface::Plane { .. }, wall) | (wall, plane @ Surface::Plane { .. }) => {
            wall.spline_chart().map(|wall| (plane, wall))
        }
        _ => None,
    };
    let section = match plane_wall {
        Some((plane, wall)) => {
            let lane = section_lane.ok_or(ReplaceFaceError::NurbsLaneUnsupported {
                edge,
                scalar: T::NAME,
            })?;
            lane.section(plane, wall, old, (t0, t1), extent, band)
                .map_err(esc)?
                .map(|c| Curve3::Nurbs(Arc::new(c)))
        }
        None => {
            crate::offset_derive::section_closed(new_surface, held, old, (t0, t1), extent, band)
                .map_err(esc)?
        }
    };
    let (carrier, refused) = match section {
        Ok(carrier) => (carrier, None),
        Err(verdict) => (old.clone(), Some(refused(verdict))),
    };
    // A section with a plane names the plane first, whichever seat
    // either surface held before; any other pair keeps the held
    // surface's seat.
    let (s1, s2) = match (plane_wall, description) {
        (Some(_), _) if matches!(held, Surface::Plane { .. }) => (other_key, old_key),
        (Some(_), _) => (old_key, other_key),
        (None, EdgeDescription::Intersection { s2, .. }) if *s2 == old_key => (other_key, old_key),
        (None, _) => (old_key, other_key),
    };
    // A derived spline section is a new curve on its own domain, running
    // with the old carrier: the old edge's parameters name nothing on
    // it, so it spans that domain until `read_ends` reads its feet. A
    // closed form keeps the old span, as `read_ends` reads it.
    let (t0, t1) = match (&carrier, &refused) {
        (Curve3::Nurbs(spline), None) => {
            let (lo, hi) = spline.domain();
            (T::from_f64(lo), T::from_f64(hi))
        }
        _ => (t0, t1),
    };
    let witness = carrier.mid_point(t0, t1);
    Ok(EdgePlan {
        edge,
        spec: EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection { s1, s2, witness },
            carrier,
            param_start: t0,
            param_end: t1,
        },
        start,
        end,
        ends: None,
        sides,
        refused,
    })
}

/// A chart image with its `v` channel shifted by `shift` — the cone
/// offset's `d·cot α` parameter action, and the identity on every
/// other chart kind (`shift` is zero there).
///
/// The shift lands on the image's CONSTANT term, which is the only
/// place a `v` translation can go for an image whose moving channels
/// are the chart's own: the offset re-parameterizes the chart, it does
/// not bend the curve drawn in it. `None` for a fitted image, whose
/// `v` channel is a control net rather than a closed form — refused
/// rather than shifted point-by-point, which would author a fit this
/// door has no certificate for — and for a focal-section image, whose
/// offset is no section of the offset chart.
fn shift_chart_v<T: Real>(pcurve: &geom_brep::Pcurve<T>, shift: T) -> Option<geom_brep::Pcurve<T>> {
    use geom_brep::Pcurve;
    Some(match *pcurve {
        Pcurve::Harmonic { p0, pa, pb, pl } => Pcurve::Harmonic {
            p0: geom_core::Point2::new(p0.x, p0.y + shift),
            pa,
            pb,
            pl,
        },
        Pcurve::IsoLine { p0, pl } => Pcurve::IsoLine {
            p0: geom_core::Point2::new(p0.x, p0.y + shift),
            pl,
        },
        Pcurve::IsoArc {
            p0,
            pd,
            t0,
            angle,
            ref breaks,
        } => Pcurve::IsoArc {
            p0: geom_core::Point2::new(p0.x, p0.y + shift),
            pd,
            t0,
            angle,
            breaks: breaks.clone(),
        },
        // Both spiric images are affine in the chart's SECOND channel
        // — the cap's `v` coordinate is its constant term's, the
        // wall's is `v0` — so the shift lands on one field exactly, as
        // it does on the three arms above. The shift this door
        // computes is the cone's `d·cot α` and zero on every other
        // chart kind, so on the two charts a spiric lives on it is
        // zero; the arm is written for the action, not for the value.
        Pcurve::Spiric {
            major,
            minor,
            offset,
            ref image,
        } => Pcurve::Spiric {
            major,
            minor,
            offset,
            image: match *image {
                geom_brep::SpiricImage::Cap { p0, pm, pa } => geom_brep::SpiricImage::Cap {
                    p0: geom_core::Point2::new(p0.x, p0.y + shift),
                    pm,
                    pa,
                },
                geom_brep::SpiricImage::Wall { u0, v0, sense } => geom_brep::SpiricImage::Wall {
                    u0,
                    v0: v0 + shift,
                    sense,
                },
            },
        },
        // A focal section's image is tied to its chart through `β`, the
        // projected ellipse's eccentricity: the offset moves the curve
        // off the section it was, so no image of the same form is
        // shifted out of this one.
        // A projected image is the chart's inverse of its carrier, so the
        // offset chart's image of the offset carrier is another one.
        Pcurve::FocalSection(_) | Pcurve::Fitted(_) | Pcurve::General(_) | Pcurve::Projected(_) => {
            return None;
        }
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
    let source = match mapped.source {
        geom_brep::MappedSource::PlacedSegment { segment, place } => {
            geom_brep::MappedSource::PlacedSegment {
                segment,
                place: shifted(place),
            }
        }
        geom_brep::MappedSource::ExtrudedPoint { point, place, vec } => {
            geom_brep::MappedSource::ExtrudedPoint {
                point,
                place: shifted(place),
                vec,
            }
        }
        geom_brep::MappedSource::RevolvedPoint { .. } => return None,
    };
    Some(geom_brep::MappedCurve {
        source,
        range: mapped.range,
    })
}

/// `faces` onto one fresh chart, `surface`, each keeping the material
/// side it has now: an offset moves a chart along its own normal, so
/// the side the material lies on does not change.
pub(crate) fn offset_rechart<T: Real>(
    body: &Body<T>,
    surface: Surface<T>,
    faces: &[FaceKey],
) -> Result<Rechart<T>, ReplaceFaceError<T>> {
    // The faces are the plan's, resolved before the clone was taken.
    let sense = |face: FaceKey| proven(&body.faces, face, EntityId::Face).sense;
    let (&first, rest) = faces.split_first().ok_or(ReplaceFaceError::EmptyGroup)?;
    let mut chart = Rechart::new(surface, first, sense(first));
    for &face in rest {
        chart = chart.with(face, sense(face));
    }
    Ok(chart)
}

/// **The points move first, then one re-chart**: every group in
/// `moved` moves onto its new point, and then every chart moves in ONE
/// [`Body::set_face_surfaces_describing`] with every spec, which so
/// certifies each at the endpoints the offset leaves it. One call,
/// because an edge between two moving charts certifies on neither pair
/// of mixed charts; a spec names a chart by the key its face wears now.
/// The offset doors' shared mutation step, run on their staged clone.
///
/// A group is the vertices that move together: the moved vertices on
/// one point ([`group_by_point`]), solved once, so copies that share a
/// point before the offset share one after it. A vertex the op does
/// not move is in no group and keeps its point.
pub(crate) fn move_points_then_rechart<T: Decide + crate::props::AtRestPolicy>(
    work: &mut Body<T>,
    moved: &[(Vec<VertexKey>, Point3<T>)],
    charts: Vec<Rechart<T>>,
    specs: &[(EdgeKey, EdgeCurveSpec<T>)],
    tol: Tol,
) -> Result<(), ReplaceFaceError<T>> {
    for (vertices, point) in moved {
        if work.move_vertices(vertices, *point).is_none() {
            unreachable!(
                "{vertices:?}, which the plan resolved before the clone was taken, do not \
                 resolve in the clone: nothing removes a record during a plan, and \
                 {}",
                crate::live::NAMES_ONLY_LIVE
            );
        }
    }
    work.set_face_surfaces_describing(charts, specs, tol)
        .map_err(|error| ReplaceFaceError::Op {
            edge: None,
            error: error.from_driver(),
        })?;
    Ok(())
}

/// `vertices` grouped by the point each sits on, groups in order of
/// first appearance and each group in `vertices`' order (D9): the
/// copies of one vertex that an op moves together. Each vertex is one
/// this call resolved or read out of a record.
#[track_caller]
pub(crate) fn group_by_point<T: Real>(
    body: &Body<T>,
    vertices: impl IntoIterator<Item = VertexKey>,
) -> Vec<Vec<VertexKey>> {
    let mut groups: Vec<(crate::PointKey, Vec<VertexKey>)> = Vec::new();
    for vertex in vertices {
        let point = proven(&body.vertices, vertex, EntityId::Vertex).point;
        match groups.iter_mut().find(|(k, _)| *k == point) {
            Some((_, group)) => group.push(vertex),
            None => groups.push((point, vec![vertex])),
        }
    }
    groups.into_iter().map(|(_, group)| group).collect()
}

/// `description` with every occurrence of `old` re-pointed at `new` —
/// the stale-key step a fresh surface mint forces.
pub(crate) fn remap_description<T: Real>(
    description: EdgeDescriptionSpec<T>,
    old: SurfaceKey,
    new: SurfaceKey,
) -> EdgeDescriptionSpec<T> {
    let map = |k: SurfaceKey| if k == old { new } else { k };
    match description {
        EdgeDescriptionSpec::Intersection { s1, s2, witness } => {
            EdgeDescriptionSpec::Intersection {
                s1: map(s1),
                s2: map(s2),
                witness,
            }
        }
        EdgeDescriptionSpec::TangentIntersection { s1, s2, witness } => {
            EdgeDescriptionSpec::TangentIntersection {
                s1: map(s1),
                s2: map(s2),
                witness,
            }
        }
        EdgeDescriptionSpec::Chart {
            surface,
            image,
            wrap,
            declared,
        } => EdgeDescriptionSpec::Chart {
            surface: map(surface),
            image,
            wrap,
            declared,
        },
        EdgeDescriptionSpec::Scaffold(m) => EdgeDescriptionSpec::Scaffold(m),
    }
}

/// One re-anchored edge: its key, its new spec, and, where its carrier
/// is unwound ([`plan_reanchors`]), the distance its corner's chart
/// moved by.
type Reanchored<T> = (EdgeKey, EdgeCurveSpec<T>, Option<T>);

/// The edges that end at a moved vertex without lying on the replaced
/// face's boundary: their carriers are unchanged (the surfaces that
/// hold them did not move) and only the parameter at the moved end —
/// and, for a mapped description, the sketch endpoint that parameter
/// images — is re-anchored.
///
/// A spline carrier's parameter is a foot point read through
/// `nurbs_lane` ([`crate::AtRestPolicy::nurbs_lane`]); a scalar holding
/// none refuses with [`ReplaceFaceError::NurbsLaneUnsupported`]. A
/// move through an edge's far end is refused with the distance its
/// corner's chart moved by ([`ReplaceFaceError::ReanchorCollapse`]).
/// Each spec carries that distance where its carrier is unwound (a line
/// or a spline, with no turn to choose), the carriers whose non-forward
/// span the attach door is read as that collapse.
fn plan_reanchors<T: Decide>(
    body: &Body<T>,
    boundary: &[EdgeKey],
    corners: &Corners<T>,
    band: Band,
    tol: Tol,
    nurbs_lane: Option<geom_brep::NurbsLane<T>>,
) -> Result<Vec<Reanchored<T>>, ReplaceFaceError<T>> {
    let moved: Vec<(VertexKey, Point3<T>)> = corners
        .groups
        .iter()
        .flat_map(|(group, point)| group.iter().map(|&v| (v, *point)))
        .collect();
    let solved = &corners.solved;
    let mut out = Vec::new();
    let keys: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    for edge in keys {
        if boundary.contains(&edge) {
            continue;
        }
        let edge_data = proven(&body.edges, edge, EntityId::Edge);
        let he_plus = edge_data.he_plus;
        let start = linked(
            &body.half_edges,
            he_plus,
            EntityId::HalfEdge,
            EntityId::Edge(edge),
            "he_plus",
        )
        .start;
        let end = body.proven_half_edge_end(he_plus);
        let at = |v: VertexKey| moved.iter().find(|(k, _)| *k == v).map(|(_, p)| *p);
        let (new_start, new_end) = (at(start), at(end));
        if new_start.is_none() && new_end.is_none() {
            continue;
        }
        let offset_at = |v: VertexKey| {
            corners
                .groups
                .iter()
                .zip(&corners.offsets)
                .find(|((group, _), _)| group.contains(&v))
                .map(|(_, d)| *d)
        };
        let d = offset_at(start)
            .or_else(|| offset_at(end))
            .unwrap_or_else(|| unreachable!("{edge:?} ends at a moved corner, read off its group"));
        let Some(curve) = body.edge_curve_linked(edge, edge_data).certified() else {
            return Err(ReplaceFaceError::CarrierLaneUnsupported {
                edge,
                what: "it has no curve to re-attach",
            });
        };
        let carrier = curve.carrier().clone();
        let (mut t0, mut t1) = curve.params();
        let span = (t0, t1);
        let mut description = curve.restated_description();
        for (point, is_start) in [(new_start, true), (new_end, false)] {
            let Some(point) = point else { continue };
            let (t_old, t_other) = if is_start { span } else { (span.1, span.0) };
            let vertex = if is_start { start } else { end };
            let root = solved
                .iter()
                .find(|(e, s, _)| *e == edge && *s == is_start)
                .map(|(_, _, t)| *t);
            // Anchored at the parameter THIS ENDPOINT had, not at the
            // span's midpoint: the stored range is the traversed arc,
            // not a canonical one, so the turn it sits on is the datum
            // the re-anchor must keep. `param_near` carries why an
            // anchored read needs no branch selection.
            //
            // **THE `|δ| = π` POSE, ACCEPTED DELIBERATELY AND NOT BY
            // OMISSION.** At exactly half a turn from `t_old` the point
            // has TWO parameters within half a turn — `t_old ± π` —
            // and which one `param_near` names is `atan2`'s cut, i.e.
            // the sign bit of one dot product. `geom`'s
            // `at_the_half_turn_boundary_the_two_forms_disagree_by_a_
            // turn_and_both_are_right` measures the flip at 9 of 30
            // boundary cases. The `gap` gate immediately below CANNOT
            // see it: it asks `eval(t_new) ≈ point`, and both answers
            // satisfy that exactly — they are the same point. What
            // would differ is the STORED SPAN, by a whole turn.
            //
            // The pose is sound here because of what this door does,
            // and that is a claim about the CALLER, not about the
            // arithmetic: an offset MOVES an endpoint along its
            // carrier, it does not teleport it half a turn, so `δ`
            // stays small. Measured over the `topo`, `sweep`,
            // `editor-core`, `verbs`, `pncad` and `geom-brep` suites —
            // 789 live calls: 765 on a `Line` (no branch at all; 16 of
            // them at `Interval`), 14 on a `Circle` with
            // `max |δ| = 0.244979` rad against a boundary of `π` — an
            // order of magnitude of headroom, not a near miss — and
            // none on an `Ellipse` or a `Spiric`, whose arms share the
            // circle's tie.
            //
            // A SPLINE carrier has no branch to tie on: it is clamped
            // and open, and its read is Newton from `t_old`. What the
            // same caller premise bounds there is how far Newton has to
            // walk, and the risk is a different stationary point, not a
            // turn. 10 live calls, all on open degree-1 lofted seams
            // and M7-8 wall edges over a unit domain, `max |δ| = 0.25`.
            // A wrong stationary point is a point the gate below
            // measures, so it refuses rather than storing a wrong span
            // unless the carrier passes within ε of itself; a CLOSED
            // spline is where that read misnames its refusal
            // (`work/shell/reanchor-reads-a-closed-spline-carrier-from-the-seed-side-only.md`),
            // and `geom_brep`'s `foot_rows` pins that the seed is what
            // picks the foot.
            //
            // No refusal is added for it here. One would be a new
            // named predicate, and a new predicate's margins cannot be
            // policed on this branch while the K-telemetry probe
            // census is red (#1288) — shipping an unpoliced predicate
            // to close a gap no live call approaches is the worse
            // trade. Banked with that measurement rather than waved
            // through.
            //
            // A spline carrier has no closed-form inverse: its parameter
            // is Newton's foot from the same anchor. A foot the domain
            // clamp stopped at an end is the carrier's end, which an
            // outward move runs past — the gate below then refuses that
            // by name rather than as a point off the carrier.
            // A corner the solve placed by a root along this edge is that
            // root; any other is read off the carrier from the endpoint's
            // old parameter and must be the corner.
            let t_new = match root {
                Some(t) => t,
                None => {
                    let (t_new, clamped_at) = match &carrier {
                        Curve3::Nurbs(spline) => {
                            let lane =
                                nurbs_lane.ok_or(ReplaceFaceError::NurbsLaneUnsupported {
                                    edge,
                                    scalar: T::NAME,
                                })?;
                            let foot =
                                lane.carrier_foot(spline, point, t_old).map_err(|error| {
                                    ReplaceFaceError::ReanchorInconclusive { edge, error }
                                })?;
                            let (lo, hi) = spline.domain();
                            let clamped = foot.t == lo || foot.t == hi;
                            (T::from_f64(foot.t), clamped.then_some(foot.t))
                        }
                        Curve3::Line { .. }
                        | Curve3::Circle { .. }
                        | Curve3::Ellipse { .. }
                        | Curve3::Spiric { .. } => (
                            carrier.param_near(point, t_old).unwrap_or_else(|| {
                                unreachable!("`param_near` inverts every analytic kind")
                            }),
                            None,
                        ),
                    };
                    let gap = carrier.eval(t_new).distance(point);
                    if !gap_within_eps(gap, tol, band)? {
                        let Some(end) = clamped_at else {
                            return Err(ReplaceFaceError::VertexDisagreement { vertex, gap });
                        };
                        // The end the clamp stopped at is the edge's FAR end
                        // when the move ran through the whole edge, read off
                        // the feet of the edge's own two ends.
                        let (Curve3::Nurbs(spline), Some(lane)) = (&carrier, nurbs_lane) else {
                            unreachable!("only the spline lane clamps")
                        };
                        let foot_of = |t: T| {
                            lane.carrier_foot(spline, carrier.eval(t), t)
                                .map(|p| p.t)
                                .map_err(|error| ReplaceFaceError::ReanchorInconclusive {
                                    edge,
                                    error,
                                })
                        };
                        let (own, far) = (foot_of(t_old)?, foot_of(t_other)?);
                        return Err(if (end - far).abs() < (end - own).abs() {
                            ReplaceFaceError::ReanchorCollapse { edge, offset: d }
                        } else {
                            ReplaceFaceError::ReanchorPastCarrierEnd { edge, gap }
                        });
                    }
                    t_new
                }
            };
            // **The door RE-STATES the sketch datum; it does not
            // patch the carrier around it.** The datum is the placed
            // segment whose pushforward determined this locus, and it
            // has to end where the edge now ends.
            //
            // **Both homes of that datum, since PCURVE P-1b.** It
            // still IS the description while an edge is transient
            // (the scaffolding door), and on an edge AT REST it is the
            // AUTHORITY record beside a chart image (U2 Q3). Moving
            // only the first was this unit's own miss: an at-rest cap
            // seam re-anchored fine and kept a `declared` segment that
            // still ended at the wall's OLD radius — a provenance
            // record contradicting the geometry it claims to have
            // determined. `verbs_offd::the_untouched_cap_seams_are_
            // re_anchored` reads the datum where it now lives and is
            // what caught it.
            //
            // The refusal is mirrored onto the new home DELIBERATELY,
            // not by omission: an arc's carrier and a trajectory's
            // family are sketch data this door cannot author, and that
            // was a refusal before the collapse. Dropping the
            // declaration instead would silently flip
            // `EdgeAuthority::is_declared`, which tier 3's
            // prefer-intrinsic rules read — a verdict change, which
            // this unit does not make.
            let restate = |m| {
                move_mapped_endpoint(m, point, is_start).ok_or(
                    ReplaceFaceError::CarrierLaneUnsupported {
                        edge,
                        what: "it is drawn from a sketch arc or sweep, which the offset cannot \
                               redraw",
                    },
                )
            };
            description = match description {
                EdgeDescriptionSpec::Scaffold(m) => EdgeDescriptionSpec::Scaffold(restate(m)?),
                EdgeDescriptionSpec::Chart {
                    surface,
                    image,
                    wrap,
                    declared: Some(mc),
                } => EdgeDescriptionSpec::Chart {
                    surface,
                    image,
                    wrap,
                    declared: Some(restate(mc)?),
                },
                other => other,
            };
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
        let mid = carrier.mid_point(t0, t1);
        description = match description {
            EdgeDescriptionSpec::Intersection { s1, s2, .. } => EdgeDescriptionSpec::Intersection {
                s1,
                s2,
                witness: mid,
            },
            EdgeDescriptionSpec::TangentIntersection { s1, s2, .. } => {
                EdgeDescriptionSpec::TangentIntersection {
                    s1,
                    s2,
                    witness: mid,
                }
            }
            other => other,
        };
        // A line or a clamped spline has no turn to choose, so a span
        // that stops being forward on it is the move's length alone.
        let unwound = matches!(carrier, Curve3::Line { .. } | Curve3::Nurbs(_)).then_some(d);
        out.push((
            edge,
            EdgeCurveSpec {
                description,
                carrier,
                param_start: t0,
                param_end: t1,
            },
            unwound,
        ));
    }
    Ok(out)
}

/// The moved corners: each group of vertex copies on one point, and
/// where it lands; and the parameters the roots that placed them read
/// on the untouched edges they were sought along.
struct Corners<T: Real> {
    groups: Vec<(Vec<VertexKey>, Point3<T>)>,
    /// Beside each group, the distance the first chart moving it moved
    /// by: what a move through an edge there is refused with.
    offsets: Vec<T>,
    /// `(edge, is_start, t)`: the corner at that end of an untouched
    /// edge is its carrier at `t`.
    solved: Vec<(EdgeKey, bool, T)>,
    /// The corners placed by roots rather than by the transports.
    rooted: Vec<VertexKey>,
}

/// One edge meeting a moved corner, as the corner solve reads it.
struct Incident<T: Real> {
    edge: EdgeKey,
    /// Its carrier after the move: the plan's on the boundary, its own
    /// elsewhere.
    carrier: Curve3<T>,
    sides: [SurfaceKey; 2],
    /// Each end at the corner, with the parameter to seek it from.
    ends: Vec<(bool, T)>,
    boundary: bool,
    /// A section the move derived ([`derive_edge`]), not a carrier it
    /// transported or left.
    derived: bool,
    /// The lever a root along it decides its transversality at
    /// ([`corner_arm`]).
    extent: T,
}

/// **The lever a root along `carrier` over `[t0, t1]` is decided at**:
/// the span's extent. On a derived spline section the span is the
/// section's own domain ([`derive_edge`]), the whole curve its root is
/// isolated over.
fn corner_arm<T: Decide>(carrier: &Curve3<T>, t0: T, t1: T) -> T {
    geom_brep::edge_extent(carrier, t0, t1, carrier.eval(t0).distance(carrier.eval(t1)))
}

/// **Whether the distance `gap`, in metres, is within ε**
/// (`offset_vertex_agreement`): two points that should be one corner, a
/// corner and the edge read at it, or a carrier's end and the surface
/// it stands in for a root on. The module's one read of ε: a corner is
/// held to it on every surface.
///
/// The margin `ε − gap` is decided at the band (ε, K·ε), so a gap up to
/// 2ε reads coincident, one past ε + K·ε does not, and one between the
/// two escalates by the predicate's name.
fn gap_within_eps<T: Decide>(gap: T, tol: Tol, band: Band) -> Result<bool, ReplaceFaceError<T>> {
    decide(
        "offset_vertex_agreement",
        Margin::of(T::from_f64(tol.eps()) - gap),
        band,
    )
    .map(|s| !matches!(s, Sign::Negative))
    .map_err(|source| ReplaceFaceError::Escalated { source })
}

/// **A corner's root along the spline carrier of `edge`**, nearest
/// `seed`: the lane's isolated root, or the carrier's near end where
/// the surface lies past it within ε — ε is what a corner is held to on
/// every surface, and `read_ends` and `plan_reanchors` accept an end by
/// the same predicate. Past ε the near end refuses
/// [`ReplaceFaceError::ReanchorPastCarrierEnd`] with its gap, and a
/// move through the whole carrier [`ReplaceFaceError::ReanchorCollapse`].
///
/// The inner `Err` is a lane verdict on a DERIVED section, which
/// contributes no root (the caller keeps it for a corner nothing
/// roots): the section is held to each corner by `read_ends` within ε
/// whatever roots it, a surface kind the lane does not root (anything
/// but a plane) was never asked of it before it was seeded, and a graze
/// is decided over the section's whole domain, which on a fit wider
/// than the face reaches past the edge. On any other carrier the
/// verdict refuses the corner.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn spline_corner_root<T: Decide>(
    lane: crate::offset_derive::SectionLane<T>,
    surface: &Surface<T>,
    spline: &NurbsCurve3<T>,
    (seed, extent): (T, T),
    (vertex, edge, derived): (VertexKey, EdgeKey, bool),
    d: T,
    band: Band,
    tol: Tol,
) -> Result<Result<T, crate::offset_derive::CornerVerdict<T>>, ReplaceFaceError<T>> {
    use crate::offset_derive::SplineRoot;
    match lane.root(surface, spline, seed, extent, band) {
        Ok(SplineRoot::At(t)) => Ok(Ok(T::from_f64(t))),
        Ok(SplineRoot::Short {
            end,
            near_gap,
            near: true,
        }) if gap_within_eps(T::from_f64(near_gap), tol, band)? => Ok(Ok(T::from_f64(end))),
        Ok(SplineRoot::Short {
            near_gap,
            near: true,
            ..
        }) => Err(ReplaceFaceError::ReanchorPastCarrierEnd {
            edge,
            gap: T::from_f64(near_gap),
        }),
        Ok(SplineRoot::Short { near: false, .. }) => {
            Err(ReplaceFaceError::ReanchorCollapse { edge, offset: d })
        }
        Err(verdict) if derived => Ok(Err(verdict)),
        Err(verdict) => Err(ReplaceFaceError::CornerSection {
            vertex,
            edge,
            verdict,
        }),
    }
}

/// **Where each moved corner lands** (module docs).
///
/// A corner where ONE chart moves, and every held surface around which
/// holds that move (`offset_derive::holds_the_move`), is where the
/// transports put it. Any other corner is solved: along each edge
/// meeting it, the root of each surface there the edge does not lie
/// on, every moved surface standing as moved — a moved surface along
/// any edge, a held one along an edge with a moved side — never a
/// transported point tested afterwards. Every candidate is checked
/// against every other, PAIRWISE: a star comparison passes a spread of
/// up to 2ε, and the claim is that the re-derivation is coherent.
///
/// A spline or fitted surface is not rooted: the lane roots a plane
/// along a spline carrier and the closed forms an analytic surface. It
/// is not skipped either. A candidate lies on the surface it roots and
/// on both sides of the edge it was rooted along, and every MOVED
/// surface at the corner must lie under at least one candidate, or the
/// corner refuses naming it ([`ReplaceFaceError::CornerSection`]).
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn solve_corners<T: Decide>(
    body: &Body<T>,
    plans: &[EdgePlan<T>],
    boundary: &[EdgeKey],
    charts: &[MovedChart<'_, T>],
    band: Band,
    tol: Tol,
    section_lane: Option<crate::offset_derive::SectionLane<T>>,
) -> Result<Corners<T>, ReplaceFaceError<T>> {
    use crate::offset_derive::CornerVerdict;
    let esc = |source| ReplaceFaceError::Escalated { source };
    let is_moved = |key: SurfaceKey| moved_of(charts, key).is_some();
    let surface_of = |key: SurfaceKey| -> &Surface<T> {
        match moved_of(charts, key) {
            Some(chart) => &chart.new_surface,
            None => body.get_surface(key).unwrap_or_else(|| {
                unreachable!("{key:?} is a face's surface, read off a live face")
            }),
        }
    };
    let mut holds: Vec<(SurfaceKey, SurfaceKey, bool)> = Vec::new();
    let mut corners = Corners {
        groups: Vec::new(),
        offsets: Vec::new(),
        solved: Vec::new(),
        rooted: Vec::new(),
    };
    let mut keys: Vec<VertexKey> = Vec::new();
    for plan in plans {
        for v in [plan.start, plan.end] {
            if !keys.contains(&v) {
                keys.push(v);
            }
        }
    }
    for group in group_by_point(body, keys) {
        let vertex = group[0];
        let old_point = *body
            .get_point(proven(&body.vertices, vertex, EntityId::Vertex).point)
            .unwrap_or_else(|| unreachable!("{vertex:?}'s point is a link of a live vertex"));
        let incident = incident_edges(body, plans, boundary, &group, old_point)?;
        // The moved charts first, in the order they were named, then the
        // held ones in the order the incident edges reach them.
        // A refused section is no incident edge, and its moved side still
        // meets the corner.
        let sides: Vec<SurfaceKey> = incident.iter().flat_map(|i| i.sides).collect();
        let movers: Vec<&MovedChart<'_, T>> = charts
            .iter()
            .filter(|c| {
                plans
                    .iter()
                    .filter(|p| group.contains(&p.start) || group.contains(&p.end))
                    .any(|p| p.sides.contains(&c.old_key))
            })
            .collect();
        let Some(d) = movers.first().map(|c| c.d) else {
            unreachable!(
                "{vertex:?} ends a planned edge, and every planned edge bounds a moved chart"
            )
        };
        let mut around: Vec<SurfaceKey> = movers.iter().map(|c| c.old_key).collect();
        for k in sides {
            if !around.contains(&k) {
                around.push(k);
            }
        }
        let all_hold = match movers.as_slice() {
            [mover] => {
                let mut all_hold = true;
                for &k in around.iter().filter(|k| **k != mover.old_key) {
                    let held = match holds
                        .iter()
                        .find(|(m, h, _)| *m == mover.old_key && *h == k)
                    {
                        Some((.., held)) => *held,
                        None => {
                            let held = crate::offset_derive::holds_the_move(
                                &mover.old_surface,
                                surface_of(k),
                                mover.d,
                                band,
                            );
                            holds.push((mover.old_key, k, held));
                            held
                        }
                    };
                    all_hold &= held;
                }
                all_hold
            }
            _ => false,
        };
        let transported: Vec<Point3<T>> = plans
            .iter()
            .filter_map(|p| p.ends.map(|e| (p, e)))
            .flat_map(|(p, (a, b))| [(p.start, a), (p.end, b)])
            .filter(|(v, _)| group.contains(v))
            .map(|(_, point)| point)
            .collect();
        let points = if all_hold && !transported.is_empty() {
            transported
        } else {
            corners.rooted.extend(group.iter().copied());
            let mut points = Vec::new();
            // The surfaces some candidate lies on.
            let mut covered: Vec<SurfaceKey> = Vec::new();
            // The first lane verdict a derived section passed over.
            let mut passed: Option<(EdgeKey, CornerVerdict<T>)> = None;
            for inc in &incident {
                let on_moved = inc.sides.iter().any(|&k| is_moved(k));
                for &k in around
                    .iter()
                    .filter(|k| !inc.sides.contains(k) && (on_moved || is_moved(**k)))
                {
                    let surface = surface_of(k);
                    if matches!(surface, Surface::Nurbs(_) | Surface::Approx(_)) {
                        continue;
                    }
                    for &(is_start, seed) in &inc.ends {
                        let edge = inc.edge;
                        let corner = |verdict| ReplaceFaceError::CornerSection {
                            vertex,
                            edge,
                            verdict,
                        };
                        let t = match &inc.carrier {
                            Curve3::Nurbs(spline) => {
                                let lane =
                                    section_lane.ok_or(ReplaceFaceError::NurbsLaneUnsupported {
                                        edge,
                                        scalar: T::NAME,
                                    })?;
                                match spline_corner_root(
                                    lane,
                                    surface,
                                    spline,
                                    (seed, inc.extent),
                                    (vertex, edge, inc.derived),
                                    d,
                                    band,
                                    tol,
                                )? {
                                    Ok(t) => t,
                                    Err(verdict) => {
                                        passed.get_or_insert((edge, verdict));
                                        continue;
                                    }
                                }
                            }
                            carrier => crate::offset_derive::analytic_root(
                                surface, carrier, seed, inc.extent, band,
                            )
                            .map_err(esc)?
                            .map_err(corner)?,
                        };
                        points.push(inc.carrier.eval(t));
                        for s in inc.sides.into_iter().chain([k]) {
                            if !covered.contains(&s) {
                                covered.push(s);
                            }
                        }
                        if !inc.boundary {
                            corners.solved.push((edge, is_start, t));
                        }
                    }
                }
            }
            if points.is_empty() {
                // A corner with no root to stand on, beside a section that
                // refused, is that refusal's.
                if let Some(refused) = plans
                    .iter()
                    .filter(|p| group.contains(&p.start) || group.contains(&p.end))
                    .find_map(|p| p.refused.clone())
                {
                    return Err(refused);
                }
            }
            if let Some(bare) = movers.iter().find(|c| !covered.contains(&c.old_key)) {
                // Else the verdict of a derived section it passed over.
                if let Some((edge, verdict)) = passed {
                    return Err(ReplaceFaceError::CornerSection {
                        vertex,
                        edge,
                        verdict,
                    });
                }
                let edge = if points.is_empty() {
                    incident.first()
                } else {
                    incident.iter().find(|i| i.sides.contains(&bare.old_key))
                }
                .map_or(plans[0].edge, |i| i.edge);
                return Err(ReplaceFaceError::CornerSection {
                    vertex,
                    edge,
                    verdict: CornerVerdict::Unsupported {
                        what: if points.is_empty() {
                            "no edge meeting this corner crosses a surface the corner can be \
                             solved on"
                        } else {
                            "a moved spline or fitted surface meets this corner, and no root here \
                             lies on it"
                        },
                    },
                });
            }
            points
        };
        for (i, a) in points.iter().enumerate() {
            for b in &points[i + 1..] {
                let gap = a.distance(*b);
                if !gap_within_eps(gap, tol, band)? {
                    return Err(ReplaceFaceError::VertexDisagreement { vertex, gap });
                }
            }
        }
        corners.groups.push((group, points[0]));
        corners.offsets.push(d);
    }
    Ok(corners)
}

/// The edges meeting the vertex copies `group`, as [`solve_corners`]
/// reads them.
fn incident_edges<T: Decide>(
    body: &Body<T>,
    plans: &[EdgePlan<T>],
    boundary: &[EdgeKey],
    group: &[VertexKey],
    old_point: Point3<T>,
) -> Result<Vec<Incident<T>>, ReplaceFaceError<T>> {
    let mut out = Vec::new();
    for (edge, edge_data) in body.edges() {
        let he_plus = edge_data.he_plus;
        let start = linked(
            &body.half_edges,
            he_plus,
            EntityId::HalfEdge,
            EntityId::Edge(edge),
            "he_plus",
        )
        .start;
        let end = body.proven_half_edge_end(he_plus);
        let (at_start, at_end) = (group.contains(&start), group.contains(&end));
        if !at_start && !at_end {
            continue;
        }
        let ends_at = |seeds: (T, T)| {
            let mut ends = Vec::new();
            if at_start {
                ends.push((true, seeds.0));
            }
            if at_end {
                ends.push((false, seeds.1));
            }
            ends
        };
        if boundary.contains(&edge) {
            let Some(plan) = plans.iter().find(|p| p.edge == edge) else {
                unreachable!("{edge:?} is on the boundary, and every boundary edge has a plan")
            };
            if plan.refused.is_some() {
                continue;
            }
            let carrier = plan.spec.carrier.clone();
            let (t0, t1) = (plan.spec.param_start, plan.spec.param_end);
            let ends = match (&plan.ends, &carrier) {
                (Some(_), _) => ends_at((t0, t1)),
                // A derived spline section spans its own domain and runs
                // with the old carrier, so each corner is sought from its
                // own end of it — the seeds `read_ends` reads its feet
                // from. On a fit wider than the face a plane crossing the
                // section twice can root nearer the end than the corner;
                // the pairwise check then refuses that root loudly. An
                // analytic one is sought from the old corner's parameter.
                (None, Curve3::Nurbs(_)) => ends_at((t0, t1)),
                (None, c) => {
                    let seed = c.param_near(old_point, T::zero()).unwrap_or_else(|| {
                        unreachable!("`param_near` inverts every analytic kind")
                    });
                    ends_at((seed, seed))
                }
            };
            out.push(Incident {
                edge,
                extent: corner_arm(&carrier, t0, t1),
                carrier,
                sides: plan.sides,
                ends,
                boundary: true,
                derived: plan.ends.is_none(),
            });
        } else {
            let Some(curve) = body.edge_curve_linked(edge, edge_data).certified() else {
                return Err(ReplaceFaceError::CarrierLaneUnsupported {
                    edge,
                    what: "it has no curve to re-attach",
                });
            };
            let (t0, t1) = curve.params();
            out.push(Incident {
                edge,
                carrier: curve.carrier().clone(),
                sides: edge_side_keys(body, edge, edge_data),
                ends: ends_at((t0, t1)),
                boundary: false,
                derived: false,
                extent: corner_arm(curve.carrier(), t0, t1),
            });
        }
    }
    Ok(out)
}

/// An edge's parameters, read where its two corners landed. A derived
/// section (`seeds` `None`) reads a closed form's own inverse — the end
/// from the start across the old span, so a closed edge keeps its turn
/// — and a spline's foot from its domain's ends; a transported edge
/// reads each end from its own parameter. Each must name its corner
/// within ε, or the corner and the edge disagree.
fn read_ends<T: Decide>(
    plan: &mut EdgePlan<T>,
    moved: &[(VertexKey, Point3<T>)],
    seeds: Option<(T, T)>,
    band: Band,
    tol: Tol,
    nurbs_lane: Option<geom_brep::NurbsLane<T>>,
) -> Result<(), ReplaceFaceError<T>> {
    let edge = plan.edge;
    let at = |v: VertexKey| {
        moved
            .iter()
            .find(|(k, _)| *k == v)
            .map(|(_, p)| *p)
            .unwrap_or_else(|| unreachable!("{v:?} ends a boundary edge, so it moved"))
    };
    let (p_start, p_end) = (at(plan.start), at(plan.end));
    let carrier = &plan.spec.carrier;
    let span = plan.spec.param_end - plan.spec.param_start;
    let (t0, t1) = match carrier {
        Curve3::Nurbs(spline) => {
            let lane = nurbs_lane.ok_or(ReplaceFaceError::NurbsLaneUnsupported {
                edge,
                scalar: T::NAME,
            })?;
            let (lo, hi) = spline.domain();
            let (lo, hi) = seeds.unwrap_or((T::from_f64(lo), T::from_f64(hi)));
            let foot = |p, seed| {
                lane.carrier_foot(spline, p, seed)
                    .map(|f| T::from_f64(f.t))
                    .map_err(|error| ReplaceFaceError::ReanchorInconclusive { edge, error })
            };
            (foot(p_start, lo)?, foot(p_end, hi)?)
        }
        c => {
            let inverse = |p, near| {
                c.param_near(p, near)
                    .unwrap_or_else(|| unreachable!("`param_near` inverts every analytic kind"))
            };
            match seeds {
                Some((s0, s1)) => (inverse(p_start, s0), inverse(p_end, s1)),
                None => {
                    let t0 = inverse(p_start, T::zero());
                    (t0, inverse(p_end, t0 + span))
                }
            }
        }
    };
    for (vertex, point, t) in [(plan.start, p_start, t0), (plan.end, p_end, t1)] {
        let gap = carrier.eval(t).distance(point);
        if !gap_within_eps(gap, tol, band)? {
            return Err(ReplaceFaceError::VertexDisagreement { vertex, gap });
        }
    }
    plan.spec.param_start = t0;
    plan.spec.param_end = t1;
    // A sketch record beside the edge ends where the edge now does.
    let restate = |m, point, is_start| {
        move_mapped_endpoint(m, point, is_start).ok_or(ReplaceFaceError::CarrierLaneUnsupported {
            edge,
            what: "it is drawn from a sketch arc or sweep, which the offset cannot redraw",
        })
    };
    let restated = |m| restate(restate(m, p_start, true)?, p_end, false);
    match &mut plan.spec.description {
        EdgeDescriptionSpec::Intersection { witness, .. } => {
            *witness = plan.spec.carrier.mid_point(t0, t1);
        }
        EdgeDescriptionSpec::Chart {
            declared: Some(mc), ..
        } => *mc = restated(*mc)?,
        EdgeDescriptionSpec::Scaffold(mc) => *mc = restated(*mc)?,
        _ => {}
    }
    plan.ends = Some((p_start, p_end));
    Ok(())
}

/// `mapped` with the sketch endpoint that images `is_start` moved to
/// `point` — the authoritative sketch datum re-stated, not the carrier
/// patched around it. `None` for anything but a placed line segment.
///
/// The result is a whole segment from the moved end to the other end:
/// on a whole range that other end is the authored one, verbatim; on a
/// restricted range it is the segment's own evaluation there, since the
/// edge's end is no authored point.
fn move_mapped_endpoint<T: Real>(
    mapped: geom_brep::MappedCurve<T>,
    point: Point3<T>,
    is_start: bool,
) -> Option<geom_brep::MappedCurve<T>> {
    let geom_brep::MappedSource::PlacedSegment {
        segment: segment @ geom_brep::SketchSegment::Line { a, b },
        place,
    } = mapped.source
    else {
        return None;
    };
    let (a, b) = if mapped.range.is_whole() {
        (a, b)
    } else {
        let at = |s: T| segment.eval(mapped.range.at(s));
        (at(T::zero()), at(T::one()))
    };
    let q = place.inverse().transform_point(point);
    let moved = geom_core::Point2::new(q.x, q.y);
    Some(geom_brep::MappedCurve::whole(
        geom_brep::MappedSource::PlacedSegment {
            segment: geom_brep::SketchSegment::Line {
                a: if is_start { moved } else { a },
                b: if is_start { b } else { moved },
            },
            place,
        },
    ))
}

/// **The offset mint's fit door, as the pass takes it** — the rows that
/// say what each of its two answers costs.
///
/// [`mint_offset`] is called directly because these rows are about the
/// PARAMETER: the public doors read the scalar's own seam
/// (`crate::AtRestPolicy::offset_fit_lane`), and a row that could only reach the
/// door the seam hands it could not tell an absent door from a scalar
/// that has none.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod offset_fit_door_rows {
    use geom_brep::OffsetFitLane;
    use geom_core::{Band, Tol};

    use super::{Arc, ReplaceFaceError, Surface, mint_offset};

    /// The `+0.05` mint on the bowed patch, through whatever door the
    /// caller names.
    fn mint(door: Option<OffsetFitLane<f64>>) -> Result<Surface<f64>, ReplaceFaceError<f64>> {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let (_, face) = crate::fixtures::approx_faced_body::<f64>();
        let base = Surface::Nurbs(Arc::new(crate::fixtures::bowed_patch()));
        mint_offset(face, &base, 0.05, band, tol, door)
    }

    /// **No door: the mint refuses**, naming the face and the scalar it
    /// ran at — here `f64`, the door's own scalar, handed none — never
    /// an analytic fallback and never a pass.
    #[test]
    fn no_door_refuses_the_mint_by_name() {
        let (_, face) = crate::fixtures::approx_faced_body::<f64>();
        match mint(None) {
            Err(ReplaceFaceError::ApproxLaneUnsupported { face: f, scalar }) => {
                assert_eq!((f, scalar), (face, "f64"));
            }
            other => panic!("the absence must name the face and the scalar: {other:?}"),
        }
    }

    /// **The `f64` door mints what the free function mints**, limb for
    /// limb, bit for bit — the assertion that the body moved rather
    /// than being rewritten.
    #[test]
    fn the_f64_door_mints_the_free_function_s_surface() {
        let tol = Tol::witness();
        let Ok(Surface::Approx(through_door)) = mint(Some(OffsetFitLane::fit())) else {
            panic!("the bowed patch's offset fits at the witness tolerance");
        };
        let Ok(free) =
            geom_brep::approx_offset_surface(Arc::new(crate::fixtures::bowed_patch()), 0.05, tol)
        else {
            panic!("the free function mints the same surface");
        };
        let (a, b) = (through_door.certificate(), free.certificate());
        crate::fixtures::assert_certificates_agree("the mint door", a, b);
        assert_eq!(a.rounds, b.rounds, "the refinement history moved");
        // The fit's whole net, as bits: control points, weights and
        // both knot vectors.
        let bits = |n: &geom::NurbsSurface<f64>| -> Vec<u64> {
            n.control()
                .iter()
                .flat_map(|p| p.to_array())
                .chain(n.weights().iter().copied())
                .chain(n.knots_u().knots().iter().copied())
                .chain(n.knots_v().knots().iter().copied())
                .map(f64::to_bits)
                .collect()
        };
        assert_eq!(
            bits(through_door.fit()),
            bits(free.fit()),
            "the door's fit and the free function's differ in some bit of their nets"
        );
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod shift_chart_v_rows {
    use geom_brep::{FocalImage, Pcurve};
    use geom_core::{Point2, Vec2};

    use super::shift_chart_v;

    /// The offset's `v` shift moves a harmonic image's constant term and
    /// refuses a focal-section image, whose offset is no plane section of
    /// the offset cone.
    #[test]
    fn a_cone_section_image_has_no_shift() {
        let section = Pcurve::FocalSection(FocalImage {
            u0: 0.1,
            t0: 0.0,
            v0: 2.0,
            va: -0.4,
            vb: 0.0,
            vl: 0.0,
            beta: 0.2,
            sense: 1.0,
        });
        assert!(shift_chart_v(&section, 0.3).is_none());
        let harmonic = Pcurve::Harmonic {
            p0: Point2::new(0.0, 1.0),
            pa: Vec2::new(0.5, 0.0),
            pb: Vec2::new(0.0, 0.5),
            pl: Vec2::new(0.0, 0.0),
        };
        let Some(Pcurve::Harmonic { p0, .. }) = shift_chart_v(&harmonic, 0.25) else {
            panic!("a harmonic image shifts");
        };
        assert_eq!(p0.y, 1.25);
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod pose_reach_rows {
    use geom::{Curve3, Surface};
    use geom_brep::Reach;
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::pose_route;

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("a linear band")
    }

    /// **The reach bounds every point of an ellipse in any stored order or
    /// sign.** The mint certifies an ellipse stored with `minor > major`
    /// and one with a negative `major` (its `u_ref` flipped); the span's
    /// lever from a surface's anchor must still be at least each point's
    /// distance from it, round the whole turn. Read at `|major|`, the
    /// first falls short by `minor − major`.
    #[test]
    fn the_reach_bounds_an_ellipse_in_any_stored_frame() {
        let anchor = Point3::new(0.3, -0.2, 0.1);
        for (major, minor, u) in [(0.5, 0.9, 1.0), (-0.9, 0.5, -1.0), (0.9, -0.5, 1.0)] {
            let e = Curve3::Ellipse {
                center: Point3::new(0.1, 0.2, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                major,
                minor,
                u_ref: Vec3::new(u, 0.0, 0.0),
            };
            let reach = Reach::Span {
                carrier: e.clone(),
                t0: 0.0,
                t1: 0.5,
            }
            .lever_from(anchor);
            for k in 0..=720 {
                let t = core::f64::consts::TAU * f64::from(k) / 720.0;
                let far = (e.eval(t) - anchor).norm();
                assert!(
                    reach >= far,
                    "({major}, {minor}): the reach {reach} falls short of {far} at θ = {t}"
                );
            }
        }
    }

    /// **A near-parabola is not admitted as an ellipse by a loose
    /// lever.** Plane×cone, apex at the origin, half-angle 0.5; the
    /// plane tilted so `pn_conic_type`'s `D` levered at the edge's
    /// EXACT reach is `−k·ε`, in the band. The edge is a line whose
    /// midpoint stands 2 m from the apex, 2 m either side of it at
    /// right angles: its endpoints stand `2√2` from the apex, its ball
    /// reaches 4. Read at the ball's lever the same `D` is `−k·ε·√2`,
    /// definitely an ellipse for `k ≥ 8`, and the gate admitted a pose
    /// the arm cannot tell from a parabola.
    #[test]
    fn a_line_edges_pose_is_read_at_its_endpoints_not_its_ball() {
        let eps = band().zero();
        let alpha: f64 = 0.5;
        let cone = Surface::Cone {
            apex: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: alpha,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let edge = Curve3::Line {
            origin: Point3::new(2.0, -2.0, 0.0),
            dir: Vec3::new(0.0, 1.0, 0.0),
        };
        let tight = 8.0_f64.sqrt();
        let span = Reach::Span {
            carrier: edge.clone(),
            t0: 0.0,
            t1: 4.0,
        };
        assert!((span.lever_from(Point3::origin()) - tight).abs() < 1e-12);
        // `k = 1.2` is in the band too, and reads Zero if the lever is
        // cut below the endpoints' distance.
        for k in [1.2, 6.0, 8.0, 9.0, 9.9] {
            // `D = sin α·sin β − cos α·cos β = −cos(α + β)` for a normal
            // tilted `β` off the axis.
            let beta = (k * eps / tight).acos() - alpha;
            let plane = Surface::Plane {
                origin: Point3::new(0.0, 0.0, 1.0),
                normal: Vec3::new(beta.sin(), 0.0, beta.cos()),
                u_ref: Vec3::new(0.0, 1.0, 0.0),
            };
            let got = pose_route(&plane, &cone, &edge, 0.0, 4.0, band());
            assert!(
                matches!(got, Err(geom_brep::SectionError::Escalated(_))),
                "k = {k}: an in-band near-parabola must escalate, got {got:?}"
            );
        }
    }

    /// **The cone×cylinder pose is read at the apex.** A cone (apex at
    /// the origin, axis `z`) and a cylinder whose stored origin stands on
    /// the cone's axis 1000 m up, its axis tilted `θ` so that the edge
    /// near the apex reads the tilt in the zero band from the apex (lever
    /// 1.5). Its axis passes `1000·θ` from the apex, which `coc_coaxial`
    /// reads definite, and the arm refuses the pose. Read at the stored
    /// origin instead, the axes were coaxial, and the arm served a pose
    /// whose axes stand `1000·θ` apart at the edge.
    ///
    /// The refusal is the offset row's: the arm refuses the same way a
    /// cylinder EXACTLY parallel to the cone's axis and as far off the
    /// apex is refused, which only `coc_coaxial` can do.
    #[test]
    fn a_cone_cylinder_pose_is_read_at_the_apex() {
        let theta: f64 = 0.5 * band().zero() / 1.5;
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: 0.5,
            u_ref: Vec3::unit_x(),
        };
        let cylinder = |origin, axis| Surface::Cylinder {
            origin,
            axis,
            radius: 0.5,
            u_ref: Vec3::unit_y(),
        };
        let stored = Point3::new(0.0, 0.0, 1000.0);
        let cyl = cylinder(stored, Vec3::new(theta.sin(), 0.0, theta.cos()));
        let off = 1000.0 * theta.sin();
        let parallel = cylinder(Point3::new(-off, 0.0, 0.0), Vec3::unit_z());
        let edge = Curve3::Line {
            origin: Point3::new(1.0, 0.0, 1.0),
            dir: Vec3::unit_z(),
        };
        let refusal = |cyl: &Surface<f64>| match geom_brep::cone_cylinder_section(
            &cone,
            cyl,
            (Point3::new(1.0, 0.0, 1.1) - Point3::origin()).norm(),
            band(),
        ) {
            Err(geom_brep::SectionError::RoutesToGeneralRung {
                pair: "cone×cylinder",
                why,
            }) => why,
            other => panic!("the arm refuses the pose to the general rung: {other:?}"),
        };
        assert_eq!(
            refusal(&cyl),
            refusal(&parallel),
            "the tilted cylinder is refused by the offset row"
        );
        for (label, a, b) in [("cone, cyl", &cone, &cyl), ("cyl, cone", &cyl, &cone)] {
            let got = pose_route(a, b, &edge, 0.0, 0.1, band());
            assert!(
                matches!(got, Ok(ref r) if !r.implemented && r.note == refusal(&parallel)),
                "({label}): axes apart at the apex refuse the pose, got {got:?}"
            );
        }
    }

    /// **The cone×cylinder tilt is levered at the edge's distance from
    /// the apex.** A cone (apex at the origin, axis `z`) and a radius-½
    /// cylinder coaxial through the apex, its origin stored 1000 m along
    /// its axis, tilted so the edge (`(1, 0, 1)` to `(1, 0, 1.1)`, whose
    /// far end stands `L ≈ 1.487` from the apex) reads `k·zero`. At
    /// `k = 0.8` the arm serves the coaxial circles; at `k = 1.3` it
    /// escalates on the axis row. A lever half as long serves both, one
    /// 1.42 times as long escalates both, and one from the stored origin
    /// refuses both.
    #[test]
    fn a_cone_cylinder_tilt_is_levered_at_the_edges_distance_from_the_apex() {
        let edge = Curve3::Line {
            origin: Point3::new(1.0, 0.0, 1.0),
            dir: Vec3::unit_z(),
        };
        let lever = (Point3::new(1.0, 0.0, 1.1_f64) - Point3::origin()).norm();
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: 0.5,
            u_ref: Vec3::unit_x(),
        };
        for (k, serves) in [(0.8, true), (1.3, false)] {
            let theta: f64 = k * band().zero() / lever;
            let axis = Vec3::new(theta.sin(), 0.0, theta.cos());
            let cyl = Surface::Cylinder {
                origin: Point3::origin() + axis * 1000.0,
                axis,
                radius: 0.5,
                u_ref: Vec3::unit_y(),
            };
            for (label, a, b) in [("cone, cyl", &cone, &cyl), ("cyl, cone", &cyl, &cone)] {
                let got = pose_route(a, b, &edge, 0.0, 0.1, band());
                if serves {
                    assert!(
                        matches!(got, Ok(ref r) if r.implemented),
                        "k = {k} ({label}): the tilt is in the zero band, served: {got:?}"
                    );
                } else {
                    assert!(
                        matches!(
                            got,
                            Err(geom_brep::SectionError::Escalated(ref d))
                                if d.predicate == Some("coc_axes_parallel")
                        ),
                        "k = {k} ({label}): the tilt is in band, escalated: {got:?}"
                    );
                }
            }
        }
    }

    /// **A NURBS ruling's pose is read at its least-lever pivot.** Row
    /// A's two cylinders and ruling, the edge a degree-1 NURBS carrier
    /// on `(1, 0, z)` with UNEVEN control points: `z = (−1, 0.8, 1)`
    /// (knots `[0, 0, 0.9, 1, 1]`) and `z = (−1, 0.9, 0.95, 1)`. Its lever
    /// is the farthest control point, least at the axis point midway
    /// along the net (`√2`, the stored origin's too). Read at the foot of
    /// the control points' mean it was 1.6–1.8, and `cc_axes_parallel`
    /// read a tilt the least lever leaves in the band as definite, and
    /// served the meeting axes' ellipse pair (from `k = 8` or `9`).
    #[test]
    fn a_nurbs_rulings_pose_is_read_at_its_least_lever_pivot() {
        let eps = band().zero();
        let c1 = Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let ruling = |zs: &[f64], knots: Vec<f64>| {
            let control: Vec<Point3<f64>> = zs.iter().map(|&z| Point3::new(1.0, 0.0, z)).collect();
            let weights = vec![1.0; control.len()];
            let kv =
                geom_core::spline::KnotVector::clamped(knots, 1).expect("a clamped knot vector");
            Curve3::Nurbs(std::sync::Arc::new(
                geom::NurbsCurve3::new(kv, control, weights).expect("a degree-1 net"),
            ))
        };
        let edges = [
            (
                "nurbs3",
                ruling(&[-1.0, 0.8, 1.0], vec![0.0, 0.0, 0.9, 1.0, 1.0]),
            ),
            (
                "nurbs4",
                ruling(&[-1.0, 0.9, 0.95, 1.0], vec![0.0, 0.0, 0.9, 0.95, 1.0, 1.0]),
            ),
            // Clustered at one end with uniform knots: the curve's
            // mid-parameter point stands at `z = 0.98`, whose lever to
            // `z = −1` is 2.2 against the least lever's √2.
            (
                "nurbs5",
                ruling(
                    &[-1.0, 0.97, 0.98, 0.99, 1.0],
                    vec![0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0],
                ),
            ),
        ];
        for (name, edge) in &edges {
            for k in [1.2, 8.0, 9.0, 9.9] {
                let theta: f64 = k * eps / 2.0_f64.sqrt();
                let c2 = Surface::Cylinder {
                    origin: Point3::new(2.0, 0.0, 0.0),
                    axis: Vec3::new(theta.sin(), 0.0, theta.cos()),
                    radius: 1.0,
                    u_ref: Vec3::unit_y(),
                };
                for (label, a, b) in [("c1, c2", &c1, &c2), ("c2, c1", &c2, &c1)] {
                    let got = pose_route(a, b, edge, 0.0, 1.0, band());
                    assert!(
                        matches!(got, Err(geom_brep::SectionError::Escalated(_))),
                        "{name}, k = {k} ({label}): an in-band tilt must escalate, got {got:?}"
                    );
                }
            }
        }
    }

    /// **A ruling edge's pose is levered at its endpoints, not its
    /// ball** (row A). Two unit cylinders, the first along `z`, the
    /// second through `(2, 0, 0)` tilted `θ = k·ε/√2` in `xz`: tangent
    /// along the ruling `(1, 0, z)`, the edge `(1, 0, −1)..(1, 0, 1)`.
    /// The edge's endpoints stand `√2` from either axis's foot of its
    /// midpoint, so `cc_axes_parallel` reads `k·ε`, in the band, and the
    /// gate escalates in both orders. Levered at the edge's ball
    /// (`r + half = 2`), it read `k·ε·√2`, definitely crossing for
    /// `k ≥ 8`, and the gate served the meeting axes' ellipse pair.
    /// `k = 1.2` is in the band too, and reads Zero if the lever is cut
    /// below the endpoints' distance.
    #[test]
    fn a_ruling_edges_pose_is_read_at_its_endpoints_not_its_ball() {
        let eps = band().zero();
        let c1 = Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let edge = Curve3::Line {
            origin: Point3::new(1.0, 0.0, -1.0),
            dir: Vec3::unit_z(),
        };
        for k in [1.2, 6.0, 8.0, 9.0, 9.9] {
            let theta: f64 = k * eps / 2.0_f64.sqrt();
            let c2 = Surface::Cylinder {
                origin: Point3::new(2.0, 0.0, 0.0),
                axis: Vec3::new(theta.sin(), 0.0, theta.cos()),
                radius: 1.0,
                u_ref: Vec3::unit_y(),
            };
            for (label, a, b) in [("c1, c2", &c1, &c2), ("c2, c1", &c2, &c1)] {
                let got = pose_route(a, b, &edge, 0.0, 2.0, band());
                assert!(
                    matches!(got, Err(geom_brep::SectionError::Escalated(_))),
                    "k = {k} ({label}): an in-band tilt must escalate, got {got:?}"
                );
            }
        }
    }
}

/// The face-replacement doors over a torn body: a key the caller hands
/// over that does not resolve stays a typed [`ReplaceFaceError::StaleFace`],
/// while a record of the body naming something that does not resolve
/// panics naming that record (D2 row 4) before the body is written.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_body_rows {
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{ReplaceFaceError, Surface, replace_faces_offset};
    use crate::body::Body;
    use crate::entity::{EntityId, FaceKey, GeomRef, HalfEdgeKey, LoopBoundary, ShellKey};
    use crate::review_d18::assert_torn_op_panics;
    use crate::test_support_fixtures::geometric_cube;

    fn cube() -> (Body<f64>, FaceKey) {
        let cube = geometric_cube::<f64>(Tol::witness());
        let face = cube.mefs[2].face;
        (cube.body, face)
    }

    /// A surface key the arena no longer holds.
    fn dead_surface(body: &mut Body<f64>) -> crate::geometry::SurfaceKey {
        let dead = body.add_surface(Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        });
        body.surfaces.remove(dead);
        dead
    }

    #[test]
    fn a_stale_argument_face_stays_typed() {
        let tol = Tol::witness();
        let (mut body, face) = cube();
        let stale = FaceKey::default();
        assert!(matches!(
            replace_faces_offset(&mut body, &[face, stale], 0.1, tol),
            Err(ReplaceFaceError::StaleFace { face }) if face == stale
        ));
    }

    #[test]
    fn a_torn_shell_link_panics_naming_the_face() {
        let tol = Tol::witness();
        let (mut body, face) = cube();
        body.get_face_mut(face).unwrap().shell = ShellKey::default();
        let premise = format!(
            "{}'s shell names {}, which does not resolve",
            EntityId::Face(face),
            EntityId::Shell(ShellKey::default())
        );
        assert_torn_op_panics(
            "replace_faces_offset",
            &mut body,
            &[&premise, crate::live::NAMES_ONLY_LIVE],
            |b| replace_faces_offset(b, &[face], 0.1, tol),
        );
    }

    #[test]
    fn a_torn_chart_link_panics_naming_the_face() {
        let tol = Tol::witness();
        let (mut body, face) = cube();
        let dead = dead_surface(&mut body);
        body.get_face_mut(face).unwrap().surface = dead;
        let premise = format!(
            "{}'s surface names {}, which does not resolve",
            EntityId::Face(face),
            GeomRef::Surface(dead)
        );
        assert_torn_op_panics("replace_faces_offset", &mut body, &[&premise], |b| {
            replace_faces_offset(b, &[face], 0.1, tol)
        });
    }

    #[test]
    fn a_boundary_walk_that_breaks_panics_naming_the_walk() {
        let tol = Tol::witness();
        let (mut body, face) = cube();
        let outer = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("a cube face bounds a cycle");
        };
        body.get_half_edge_mut(first).unwrap().next = HalfEdgeKey::default();
        assert_torn_op_panics(
            "replace_faces_offset",
            &mut body,
            &["loop walk from", crate::body::WALKS_CLOSE],
            |b| replace_faces_offset(b, &[face], 0.1, tol),
        );
    }

    /// The simultaneous planar door reads each moved face's chart as a
    /// link, so a torn one panics naming it rather than refusing as if
    /// the face were not planar.
    #[test]
    fn the_planar_door_panics_on_a_torn_chart_link() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let (mut body, face) = cube();
        let moves: Vec<crate::offset_together::ChartMove<f64>> = body
            .faces()
            .map(|(k, _)| crate::offset_together::ChartMove {
                faces: vec![k],
                distance: -0.1,
            })
            .collect();
        let dead = dead_surface(&mut body);
        body.get_face_mut(face).unwrap().surface = dead;
        let premise = format!(
            "{}'s surface names {}, which does not resolve",
            EntityId::Face(face),
            GeomRef::Surface(dead)
        );
        assert_torn_op_panics("offset_planes_together", &mut body, &[&premise], |b| {
            crate::offset_planes_together(b, &moves, band, tol)
        });
    }
}

/// [`read_ends`] — the check that an edge read at its corners names
/// them, which no fixture's coherent geometry reaches.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod read_ends_rows {
    use geom::Curve3;
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{EdgePlan, ReplaceFaceError, read_ends};
    use crate::entity::{EdgeKey, VertexKey};
    use crate::geometry::SurfaceKey;

    fn plan(carrier: Curve3<f64>) -> EdgePlan<f64> {
        EdgePlan {
            edge: EdgeKey::default(),
            spec: EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    s1: SurfaceKey::default(),
                    s2: SurfaceKey::default(),
                    witness: Point3::origin(),
                },
                carrier,
                param_start: 0.0,
                param_end: 1.0,
            },
            start: VertexKey::default(),
            end: VertexKey::default(),
            ends: None,
            sides: [SurfaceKey::default(); 2],
            refused: None,
        }
    }

    /// A corner off the derived carrier: the edge and the corner
    /// disagree, by the carrier's distance from it.
    #[test]
    fn a_corner_off_the_derived_carrier_is_a_disagreement() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let line = Curve3::Line {
            origin: Point3::origin(),
            dir: Vec3::unit_x(),
        };
        let off = Point3::new(0.5, 1e-3, 0.0);
        let mut p = plan(line.clone());
        let got = read_ends(
            &mut p,
            &[(VertexKey::default(), off)],
            None,
            band,
            tol,
            None,
        );
        let Err(ReplaceFaceError::VertexDisagreement { gap, .. }) = got else {
            panic!("expected the corner's disagreement, got {got:?}");
        };
        assert!(
            (gap - 1e-3).abs() < 1e-15,
            "the gap is the corner's distance, {gap}"
        );
        // On the carrier, the same read places both ends there.
        let on = Point3::new(0.5, 0.0, 0.0);
        let mut p = plan(line);
        read_ends(&mut p, &[(VertexKey::default(), on)], None, band, tol, None)
            .expect("a corner on the carrier reads");
        assert_eq!((p.spec.param_start, p.spec.param_end), (0.5, 0.5));
    }
}

/// [`spline_corner_root`] on one spline carrier, the straight segment
/// `(-1, 0, 0) → (0, 0, 0)` on `[0, 1]`: where a plane past its end is
/// taken as the corner, and what a surface the lane does not root does.
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod spline_corner {
    use geom::{NurbsCurve3, Surface};
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{ReplaceFaceError, spline_corner_root};
    use crate::entity::{EdgeKey, VertexKey};
    use crate::offset_derive::{CornerVerdict, SectionLane};

    fn segment() -> NurbsCurve3<f64> {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        NurbsCurve3::new(
            kv,
            vec![Point3::new(-1.0, 0.0, 0.0), Point3::origin()],
            vec![1.0; 2],
        )
        .unwrap()
    }

    /// The plane `x = at`, across the segment's run.
    fn across(at: f64) -> Surface<f64> {
        Surface::Plane {
            origin: Point3::new(at, 0.0, 0.0),
            normal: Vec3::unit_x(),
            u_ref: Vec3::unit_y(),
        }
    }

    /// The root nearest the segment's `t = 1` end, on a derived section
    /// or not.
    fn root(
        surface: &Surface<f64>,
        derived: bool,
    ) -> Result<Result<f64, CornerVerdict<f64>>, ReplaceFaceError<f64>> {
        let tol = Tol::witness();
        spline_corner_root(
            SectionLane::f64(),
            surface,
            &segment(),
            (1.0, 1.0),
            (VertexKey::default(), EdgeKey::default(), derived),
            0.1,
            Band::linear(tol).unwrap(),
            tol,
        )
    }

    /// The band `offset_vertex_agreement` is decided at: ε and K·ε.
    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A plane past the end by a gap whose margin `ε − gap` is within
    /// the band's zero (a gap up to 2ε): the end is the corner.
    #[test]
    fn a_gap_inside_the_zero_band_takes_the_end() {
        let eps = band().zero();
        for gap in [eps * (1.0 - 1e-3), 2.0 * eps * (1.0 - 1e-3)] {
            for derived in [true, false] {
                let got = root(&across(gap), derived);
                assert!(
                    matches!(got, Ok(Ok(t)) if t == 1.0),
                    "derived {derived}: a gap of {gap:e} is coincident, got {got:?}"
                );
            }
        }
    }

    /// Just past 2ε the margin is in the band's escalation: the corner
    /// escalates by the predicate's name rather than taking the end.
    #[test]
    fn a_gap_just_past_the_zero_band_escalates() {
        let gap = 2.0 * band().zero() * (1.0 + 1e-3);
        let got = root(&across(gap), true);
        assert!(
            matches!(
                &got,
                Err(ReplaceFaceError::Escalated { source })
                    if source.predicate == Some("offset_vertex_agreement")
            ),
            "a gap of {gap:e} escalates, got {got:?}"
        );
    }

    /// Past the escalation (a gap over ε + K·ε) the end refuses with its
    /// gap.
    #[test]
    fn a_gap_past_the_escalation_band_refuses_with_it() {
        let gap = (band().zero() + band().escalate()) * (1.0 + 1e-3);
        for derived in [true, false] {
            let got = root(&across(gap), derived);
            assert!(
                matches!(got, Err(ReplaceFaceError::ReanchorPastCarrierEnd { gap: g, .. }) if g == gap),
                "derived {derived}: a gap of {gap:e} refuses, got {got:?}"
            );
        }
    }

    /// A cylinder, which the lane roots no spline against: a derived
    /// section passes it over with the lane's verdict, any other carrier
    /// refuses the corner with it.
    #[test]
    fn a_surface_the_lane_does_not_root_passes_a_derived_section_over() {
        let cylinder = Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 0.5,
            u_ref: Vec3::unit_x(),
        };
        assert!(
            matches!(
                root(&cylinder, true),
                Ok(Err(CornerVerdict::Unsupported { .. }))
            ),
            "a derived section passes the cylinder over, got {:?}",
            root(&cylinder, true)
        );
        assert!(
            matches!(
                root(&cylinder, false),
                Err(ReplaceFaceError::CornerSection {
                    verdict: CornerVerdict::Unsupported { .. },
                    ..
                })
            ),
            "an untouched spline edge refuses the corner, got {:?}",
            root(&cylinder, false)
        );
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod move_mapped_endpoint_rows {
    use geom_core::{Affine3, Point2, Point3, Vec3};

    use super::move_mapped_endpoint;

    /// The sketch chord `(0, 0) → (4, 2)` placed at `(1, 2, 3)`.
    fn chord() -> geom_brep::MappedCurve<f64> {
        geom_brep::MappedCurve::whole(geom_brep::MappedSource::PlacedSegment {
            segment: geom_brep::SketchSegment::Line {
                a: Point2::new(0.0, 0.0),
                b: Point2::new(4.0, 2.0),
            },
            place: Affine3::translation(Vec3::new(1.0, 2.0, 3.0)),
        })
    }

    fn placed(x: f64, y: f64) -> Point3<f64> {
        Point3::new(x + 1.0, y + 2.0, 3.0)
    }

    fn ends(m: geom_brep::MappedCurve<f64>) -> [f64; 6] {
        let (p, q) = (m.eval(0.0), m.eval(1.0));
        [p.x, p.y, p.z, q.x, q.y, q.z]
    }

    /// **A moved end on a restricted range re-authors a whole chord
    /// from the moved point to the edge's other end.** The edge is the
    /// chord's `[¼, ¾]`, from `(1, ½)` to `(3, 1½)`; with either end
    /// moved, the description runs from the moved point to the edge's
    /// unmoved end, exactly, over a whole range — not from the moved
    /// point to the authored chord's far end, and not over the old
    /// range of a chord whose authored end was moved.
    #[test]
    fn a_restricted_edge_moves_its_own_end() {
        let edge = chord().restrict(0.25, 0.75);
        let moved = placed(1.0, 0.75);
        let m = move_mapped_endpoint(edge, moved, true).expect("a placed line moves its end");
        assert!(m.range.is_whole(), "the moved description is whole");
        let far = placed(3.0, 1.5);
        assert_eq!(
            ends(m),
            [moved.x, moved.y, moved.z, far.x, far.y, far.z],
            "the moved start runs to the edge's own end"
        );
        let moved = placed(3.0, 1.25);
        let m = move_mapped_endpoint(edge, moved, false).expect("a placed line moves its end");
        assert!(m.range.is_whole(), "the moved description is whole");
        let near = placed(1.0, 0.5);
        assert_eq!(
            ends(m),
            [near.x, near.y, near.z, moved.x, moved.y, moved.z],
            "the moved end runs from the edge's own start"
        );
    }

    /// On a whole range the unmoved end is the authored one, verbatim.
    #[test]
    fn a_whole_edge_keeps_its_authored_other_end() {
        let moved = placed(0.5, -0.25);
        let m = move_mapped_endpoint(chord(), moved, true).expect("a placed line moves its end");
        let geom_brep::MappedSource::PlacedSegment {
            segment: geom_brep::SketchSegment::Line { a, b },
            ..
        } = m.source
        else {
            panic!("a placed line stays one");
        };
        assert_eq!((a.x, a.y, b.x, b.y), (0.5, -0.25, 4.0, 2.0));
        assert!(m.range.is_whole());
    }
}
