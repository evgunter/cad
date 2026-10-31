//! `contfp` — point-in-face with typed ON verdicts (ch. 13's `contfv`
//! extern-out-parameter idiom as a proper sum type): the second-level
//! case codes of the reduction sweep. The point is assumed on the
//! face's plane (the caller's crossing/on-plane decision precedes).
//!
//! Ladder: an ON verdict fires only at a trilean **Zero** (exact within
//! ε — where declared/structural coincidences land, e.g. a crossing
//! point computed from shared geometry); the sliver band escalates
//! typed (F6); definite margins walk on. Interior/exterior then comes
//! from one walk over the outer loop and every ring, which reads each
//! edge on its own CARRIER ([`crate::splitting::containment::carrier_loop_side`]): a line is its
//! chord, a circle or ellipse arc is crossed on its conic, and a spiric
//! or spline arc piece by piece on its own carrier.

use geom_brep::recourse::{LeverOnly, Reading, RefusedArm, SizedPass};
use geom_core::k_stats::{Magnitude, decide_magnitude_reported};
use geom_core::{Band, Decide, Indeterminate, Margin, MarginDiag, Point3, Sign, Vec3};

use super::region::RegionRefusal;
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, LoopBoundary, LoopKey, VertexKey};
use crate::live::{linked, proven};
use crate::ray_walk::ParityRows;
use crate::splitting::containment::placement_sized;
use crate::splitting::containment::{
    BoundaryRows, ConicRows, OffLoop, OnEdge, OnLoop, carrier_loop, carrier_loop_side,
};
use crate::splitting::pieced_arc::ArcRows;
use crate::splitting::{Escalation, LoopDecision, PointInLoopError, Uncrossable};
use crate::validate::decide;

/// The typed `contfp` verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaceContainment {
    /// Strictly outside the face.
    Out,
    /// Strictly within the face interior.
    In,
    /// On the interior of a boundary edge (the edge to split).
    OnEdge(EdgeKey),
    /// Coincident with a boundary vertex.
    OnVertex(VertexKey),
}

/// **The question a placement escalated on** (D4 ¶1 (i)): the closed
/// type an escalation of [`contfp`] and the curved doors beside it
/// carries, so its ending is an exhaustive match over the decision rather
/// than a lookup by predicate name ([`Self::ending`]). The planar loop
/// walk's own questions are [`LoopDecision`]'s, and `contfp`'s boundary
/// pre-pass asks the walk's [`LoopDecision::Boundary`] on the same
/// readings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainDecision {
    /// A question of the planar loop walk.
    Loop(LoopDecision),
    /// Whether an arc's carrier ends sit on its stored vertices, asked
    /// where the point lies within the band of an arc's end but clear of
    /// both vertices. Its margin is the end's offset from its vertex, not
    /// the point's distance, so no tolerance it gives places the point.
    ArcEnd,
    /// Whether a loop's arcs are arcs of one circle: the gap between
    /// circles, folding centres, radii and axes. One circle and clearly
    /// different circles are both read.
    OneCircle,
    /// Whether the point lies on a curved face's surface. Its pass set is
    /// the caller's: off the surface is a definite `Out` where the caller
    /// asks whether the face holds the point, and a contradiction where
    /// the caller placed the point on the surface (a certified root at
    /// `reduce::wall_crossing`, a residual there). No one sign set is the
    /// decision's, so no margin gives a tolerance to tighten below.
    Carrier,
}

impl From<LoopDecision> for ContainDecision {
    fn from(decision: LoopDecision) -> Self {
        Self::Loop(decision)
    }
}

/// What a placement refusal's decision decides, as a clause: `decision`'s
/// own, or, where the refusal names none, the placement question itself.
#[must_use]
pub const fn placement_subject(decision: Option<ContainDecision>) -> &'static str {
    match decision {
        Some(ContainDecision::Loop(d)) => d.subject(),
        Some(ContainDecision::ArcEnd) => {
            "whether an arc's end lies where its boundary's corner is stored"
        }
        Some(ContainDecision::OneCircle) => "whether a loop's arcs are arcs of one circle",
        Some(ContainDecision::Carrier) => "whether a point lies on the face's surface",
        None => "whether a point lies inside a face, on its boundary, or outside it",
    }
}

/// The geometry lever of a placement refusal, after "Recourse: " — the one
/// source every rendering of it reads: `decision`'s own, or, where the
/// refusal names none (the point-in-solid door's escalation, the sphere
/// region's, a cone face's nappe read at the Boolean), the placement
/// question's own lever.
#[must_use]
pub const fn placement_lever(decision: Option<ContainDecision>) -> &'static str {
    match decision {
        Some(ContainDecision::Loop(d)) => d.lever(),
        Some(ContainDecision::ArcEnd) => "move the point clear of the arc's end",
        Some(ContainDecision::OneCircle) => {
            "put the loop's arcs on one circle or on clearly different ones"
        }
        Some(ContainDecision::Carrier) => {
            "move the point exactly onto the face's surface or clearly off it"
        }
        None => "move the point clearly inside or outside the face",
    }
}

/// **The escalations each site raises**: every decision paired with each
/// way its sites' readings can stand, and `None` for a refusal that names
/// no decision. A pair not here is raised nowhere.
pub const CONTAINMENT_RAISED: [(Option<ContainDecision>, Escalation); 10] = {
    use ContainDecision as C;
    use Escalation::{Margin, Straddle};
    use LoopDecision as L;
    [
        (None, Margin),
        (Some(C::Loop(L::Boundary)), Margin),
        (Some(C::Loop(L::Boundary)), Straddle),
        (Some(C::Loop(L::Ray)), Margin),
        (Some(C::Loop(L::ArcSpan)), Margin),
        (Some(C::Loop(L::ArcSpan)), Straddle),
        (Some(C::Loop(L::Plane)), Margin),
        (Some(C::ArcEnd), Margin),
        (Some(C::OneCircle), Margin),
        (Some(C::Carrier), Margin),
    ]
};

impl ContainDecision {
    /// Every decision, for the rows that sample them.
    pub const ALL: [Self; 7] = [
        Self::Loop(LoopDecision::Boundary),
        Self::Loop(LoopDecision::Ray),
        Self::Loop(LoopDecision::ArcSpan),
        Self::Loop(LoopDecision::Plane),
        Self::ArcEnd,
        Self::OneCircle,
        Self::Carrier,
    ];

    /// The one ending an escalation of this decision carries, read at
    /// `reading` (D4 ¶1 (i)), from the shared table.
    #[must_use]
    pub fn ending(self, escalation: Escalation, diag: &Indeterminate, reading: Reading) -> String {
        let arm = escalation.arm(diag);
        let lever = placement_lever(Some(self));
        match self {
            Self::Loop(d) => d.ending(escalation, diag, reading),
            Self::ArcEnd | Self::Carrier => LeverOnly { lever }.recourse(arm, reading),
            Self::OneCircle => placement_sized(lever, "gap between circles", SizedPass::AnySign)
                .recourse(arm, reading),
        }
    }

    /// The decision's lever alone on `escalation`'s arm, with the
    /// unreadable-margin note on a poisoned margin: its ending at a door
    /// that offers no tolerance for it (the Boolean's, `refusal_routes`),
    /// read at the build.
    #[must_use]
    pub fn lever_ending(self, escalation: Escalation, diag: &Indeterminate) -> String {
        LeverOnly {
            lever: placement_lever(Some(self)),
        }
        .recourse(escalation.arm(diag), Reading::Build)
    }
}

/// The one ending of an escalation that names `decision` or, `None`, no
/// decision (the unnamed lever alone), read at `reading`.
#[must_use]
pub fn placement_ending(
    decision: Option<ContainDecision>,
    escalation: Escalation,
    diag: &Indeterminate,
    reading: Reading,
) -> String {
    match decision {
        Some(decision) => decision.ending(escalation, diag, reading),
        None => unnamed_placement_ending(escalation.arm(diag), reading),
    }
}

/// The unnamed placement's ending on `arm`, read at `reading`: its lever
/// alone, with the unreadable-margin note on a poisoned margin — the
/// `None` arm of [`placement_ending`], for a refusal that holds its
/// readings in a shape of its own rather than as one escalation.
#[must_use]
pub fn unnamed_placement_ending(arm: RefusedArm<'_>, reading: Reading) -> String {
    LeverOnly {
        lever: placement_lever(None),
    }
    .recourse(arm, reading)
}

/// Typed refusal of [`contfp`].
///
/// `Clone`/`PartialEq` because a consumer CARRIES this refusal rather
/// than restating it: the tier-3′ census holds it inside
/// [`CensusUnsupportedCause::Containment`](crate::CensusUnsupportedCause::Containment),
/// and [`ValidationError`](crate::ValidationError) is a cloneable,
/// comparable value.
#[derive(Debug, Clone, PartialEq)]
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(ContainErrorKind), vis(pub(crate)), derive(strum::EnumIter))
)]
pub enum ContainError {
    /// A margin landed in the sliver band — the pair is
    /// ill-conditioned at this ε.
    Escalated {
        /// The question it escalated on; `None` where the point-in-solid
        /// door or the sphere region raised it, which name none.
        decision: Option<ContainDecision>,
        /// How its refused reading stands.
        escalation: Escalation,
        /// The escalation diagnostics (named predicate inside).
        diag: Indeterminate,
    },
    /// No ray of the walk's schedule settled — each grazed or gave
    /// nothing to read ([`crate::ray_walk::NoRaySettled`]) — a planar
    /// loop's, or a sphere face's region ([`super::sphere_region`]).
    RayExhausted,
    /// The face the caller passed does not resolve in this body.
    StaleFace(FaceKey),
    /// A loop of the face is a lone vertex: it bounds no region a
    /// point could be placed against.
    EmptyLoop(LoopKey),
    /// The in-plane walk refused a loop
    /// ([`PointInLoopError::CorruptLoop`], carried).
    LoopUnreadable(LoopKey),
    /// The walk could not read a loop at this point: an edge of it it
    /// could not cross stood in the way of every ray.
    Uncrossable(Uncrossable),
    /// A curved face's chart read refused. The reads are the solid
    /// door's own, so the refusal is its, carried whole.
    Curved(super::solid_contain::PointInSolidError),
    /// The point lies in the band of an arc's end, and the vertex pass
    /// placed it definitely clear of both of the edge's stored corners
    /// ([`ContainDecision::ArcEnd`]'s definite arm, D4 ¶1 (iv)). Two
    /// definite readings that disagree, carried with the arc's ends
    /// read against those corners.
    ArcEndClearOfCorners(Box<ArcEndGaps>),
}

/// **An arc's two carrier ends read against its edge's two stored
/// corners** (`bool_contact_arc_end_vertex`), every one decided: what
/// [`ContainError::ArcEndClearOfCorners`] carries. The two passes of
/// [`boundary_pre_pass`] part only by as much as an end sits off its
/// corner: the boundary reading puts the point at an end on its raw
/// chord to the end's point, the distance the vertex pass reads to the
/// corner. So where every end lies at a corner the parting is the band's,
/// reachable only at `K < 2`. Certification pins each end within the
/// run's zero band of its corner (`geom_brep`'s
/// `carrier_endpoint_{start,end}`), and the ends here are read through
/// the same `Curve3::eval`, so an end off both corners is read where a
/// caller's band is finer than the run's, or where the certifier's
/// distance and this reading's part by rounding across a band whose
/// width `(K − 1)·zero` is narrower than that rounding: the kernel's two
/// readings of one end disagreeing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArcEndGaps {
    /// `gaps[i][j]`: the carrier's start (`i = 0`) or end (`i = 1`)
    /// against the edge's start (`j = 0`) or end (`j = 1`) corner, in
    /// the loop's order, with the margin each was decided on.
    pub(crate) gaps: [[(Magnitude, MarginDiag); 2]; 2],
    /// The band they were decided against.
    pub(crate) band: Band,
}

impl ArcEndGaps {
    /// The end of the arc that lies definitely off both corners, as its
    /// gap to its own corner and then to the other: its own is the one
    /// the other end does not lie at, or the one of its own index where
    /// that end lies at neither. `None` where every end lies at a corner.
    fn off_end(&self) -> Option<(MarginDiag, MarginDiag)> {
        let off =
            |end: &[(Magnitude, MarginDiag); 2]| end.iter().all(|(m, _)| *m == Magnitude::Positive);
        let i = self.gaps.iter().position(off)?;
        let other = self.gaps[1 - i];
        let own = other
            .iter()
            .position(|(m, _)| *m == Magnitude::Zero)
            .map_or(i, |j| 1 - j);
        Some((self.gaps[i][own].1, self.gaps[i][1 - own].1))
    }

    /// Whether an end of the arc lies definitely off both corners.
    #[must_use]
    pub fn off_corner(&self) -> bool {
        self.off_end().is_some()
    }
}

impl core::fmt::Display for ArcEndGaps {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let zero = self.band.zero();
        match self.off_end() {
            Some((own, far)) => write!(
                f,
                "the end lies {own:e} m from its own corner, {far:e} m from the other"
            ),
            None => write!(
                f,
                "each end lies within ±{zero:e} of one corner, a parting this K cannot resolve"
            ),
        }
    }
}

impl From<PointInLoopError> for ContainError {
    fn from(e: PointInLoopError) -> Self {
        match e {
            PointInLoopError::Escalated {
                decision,
                escalation,
                diag,
                ..
            } => Self::Escalated {
                decision: Some(decision.into()),
                escalation,
                diag,
            },
            PointInLoopError::RayExhausted { .. } => Self::RayExhausted,
            PointInLoopError::CorruptLoop { r#loop } => Self::LoopUnreadable(r#loop),
            PointInLoopError::Uncrossable(u) => Self::Uncrossable(u),
            PointInLoopError::OffPlane(_) => {
                unreachable!("contfp reads loops through carrier_loop, which certifies no plane")
            }
        }
    }
}

// Each arm names WHAT STOPPED, and the repair that moves it where one
// does, because a consumer that carries this refusal renders it verbatim
// and adds no sentence of its own. `RayExhausted` and `Uncrossable` want
// different repairs — move the geometry, re-model the loop — so
// one shared tail would name the wrong one.
//
// `Escalated` renders its margin's payload and its decision's one ending
// at a build ([`placement_ending`]); `Curved` delegates to the carried
// refusal's own `Display`.
impl core::fmt::Display for ContainError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Escalated {
                decision,
                escalation,
                diag,
            } => write!(
                f,
                "contfp: {}",
                diag.undecided(
                    placement_subject(*decision),
                    placement_ending(*decision, *escalation, diag, Reading::Build),
                )
            ),
            Self::RayExhausted => write!(f, "contfp: {}", crate::ray_walk::NoRaySettled),
            Self::StaleFace(face) => {
                write!(f, "contfp: face {face:?} does not resolve in this body")
            }
            Self::EmptyLoop(lp) => write!(
                f,
                "contfp: loop {lp:?} of the face is a lone vertex, which bounds no region \
                 to place a point against"
            ),
            Self::LoopUnreadable(lp) => write!(
                f,
                "contfp: the in-plane walk could not read loop {lp:?} of the face"
            ),
            Self::Curved(e) => write!(f, "contfp: {e}"),
            Self::Uncrossable(u) => write!(
                f,
                "contfp: {u}. Recourse: model the outline with lines, circles or ellipses"
            ),
            Self::ArcEndClearOfCorners(gaps) => write!(
                f,
                "contfp: the point lies at an arc's end clear of its corners: {gaps}. Recourse: {}",
                placement_lever(Some(ContainDecision::ArcEnd))
            ),
        }
    }
}

impl std::error::Error for ContainError {}

impl ContainError {
    /// An escalation of `decision` on its own in-band or unreadable
    /// margin.
    fn on(decision: impl Into<ContainDecision>, diag: Indeterminate) -> Self {
        Self::Escalated {
            decision: Some(decision.into()),
            escalation: Escalation::Margin,
            diag,
        }
    }
}

/// The panic for a kernel driver whose own face key a containment door
/// answered [`ContainError::StaleFace`]: a driver reads its faces out of
/// the body, so they resolve.
#[track_caller]
pub(crate) fn driver_face_stale(face: FaceKey) -> ! {
    unreachable!(
        "a kernel driver asked a containment door about face {face:?}, which does not \
         resolve: a driver reads its faces out of the body, and {}",
        crate::live::NAMES_ONLY_LIVE
    )
}

/// `face`'s outer loop and rings, the face the caller's key.
fn face_loops<T: Decide>(body: &Body<T>, face: FaceKey) -> Result<Vec<LoopKey>, ContainError> {
    let face_data = body.get_face(face).ok_or(ContainError::StaleFace(face))?;
    Ok(core::iter::once(face_data.outer)
        .chain(face_data.rings.iter().copied())
        .collect())
}

/// **`contfp`** — classifies point `q` (already on the plane of `face`,
/// with unit plane normal `normal`) against the face. Sweep order is
/// deterministic: the boundary pre-pass ([`boundary_pre_pass`] —
/// vertices over ALL loops first, then edge interiors over all loops)
/// decides every ON case before ray parity runs.
///
/// # Errors
///
/// [`ContainError`] — sliver escalations or unwalkable topology.
pub fn contfp<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<FaceContainment, ContainError> {
    contfp_decided(body, face, normal, q, band).map(|(at, _)| at)
}

/// [`contfp`], with the margin that decided an `OnEdge` or `OnVertex`
/// verdict (`None` for `In` and `Out`, which decide no coincidence).
///
/// # Errors
///
/// As [`contfp`].
pub(crate) fn contfp_decided<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<(FaceContainment, Option<MarginDiag>), ContainError> {
    let loops = face_loops(body, face)?;

    let read = match boundary_pre_pass(body, &loops, q, band)? {
        PrePass::On(on, margin) => return Ok((on, Some(margin))),
        PrePass::Off(read) => read,
    };

    // Interior/exterior: inside the outer loop AND outside every ring,
    // each read on its edges' own carriers by a walk that trusts the
    // pre-pass above — `q` is definitely off every edge — so it answers
    // inside or outside.
    let inside = |(lk, read): &ReadLoop<T>| -> Result<bool, ContainError> {
        Ok(carrier_loop_side(*lk, read, normal, q, band)?)
    };
    let Some((outer, rings)) = read.split_first() else {
        unreachable!("the pre-pass reads every loop of the face, and a face has its outer loop")
    };
    if !inside(outer)? {
        return Ok((FaceContainment::Out, None));
    }
    for ring in rings {
        if inside(ring)? {
            return Ok((FaceContainment::Out, None));
        }
    }
    Ok((FaceContainment::In, None))
}

/// The circle a disc-class loop bounds — its own type, because three
/// components of one datum read better named than positional.
///
/// Its centre and radius are readable crate-wide: tier 3's check 9
/// decides two disc-class loops against each other from them (the
/// centre distance against the radii's sum and difference), a question
/// about a PAIR of loops that no one point's walk can ask.
#[derive(Clone, Copy)]
pub(crate) struct LoopCircle<T: geom_core::Real> {
    /// The circle's centre.
    pub(crate) center: Point3<T>,
    /// Its plane normal (sign-free: only `cross` reads it).
    axis: Vec3<T>,
    /// Its radius, in metres.
    pub(crate) radius: T,
}

/// **The circle this loop bounds, if it bounds one**: `Some` when every
/// edge is an arc of ONE circle, so the loop's region is that circle's
/// disc exactly — the planar analog of the curved door's iso-bounded
/// class ([`curved_face_containment`]) — and `None` for a loop with a
/// line, a non-circular conic, a null edge, or arcs of two circles.
///
/// The circle is read from the first edge; every later edge must agree
/// with it on one metre-valued row folding centre offset, radius
/// difference and axis tilt (levered at the radius). The axis enters
/// only through `cross`, which is blind to its sign — two arcs of one
/// circle may run in opposite senses and are still arcs of the same
/// point set, which is all this asks.
///
/// **That row classifies the loop, not a point.** Arcs whose circles
/// agree to within the band are read as one circle; check 9 decides a
/// PAIR of disc-class loops from the circles' centres and radii, a
/// question the band's own width already bounds. A definite
/// disagreement is `None`; an ESCALATION escalates, exactly as every
/// row of [`curved_face_containment`] does.
///
/// # Errors
///
/// [`ContainError`] — a carrier-agreement escalation, or a loop this
/// walk cannot read.
pub(crate) fn loop_circle<T: Decide>(
    body: &Body<T>,
    r#loop: crate::entity::LoopKey,
    band: Band,
) -> Result<Option<LoopCircle<T>>, ContainError> {
    // The WHOLE cycle is walked: an escalation on a later edge pair
    // escalates even after an earlier edge has ruled the class out.
    let mut circle: Option<LoopCircle<T>> = None;
    let mut one_circle = true;
    for (_, he, _) in loop_cycle_points(body, r#loop)? {
        let edge_key = proven(&body.half_edges, he, EntityId::HalfEdge).edge;
        let edge = linked(
            &body.edges,
            edge_key,
            EntityId::Edge,
            EntityId::HalfEdge(he),
            "edge",
        );
        let carrier = body
            .edge_curve_linked(edge_key, edge)
            .certified()
            .map(|c| c.carrier().clone());
        match carrier {
            Some(geom::Curve3::Circle {
                center,
                axis,
                radius,
                ..
            }) => match circle {
                None => {
                    circle = Some(LoopCircle {
                        center,
                        axis,
                        radius,
                    });
                }
                Some(c) => {
                    let (c0, a0, r0) = (c.center, c.axis, c.radius);
                    let d = (center - c0).norm() + (radius - r0).abs() + axis.cross(a0).norm() * r0;
                    match decide("bool_face_disc_carrier", Margin::of(d), band) {
                        Ok(Sign::Zero) => {}
                        Ok(Sign::Positive | Sign::Negative) => one_circle = false,
                        Err(diag) => {
                            return Err(ContainError::on(ContainDecision::OneCircle, diag));
                        }
                    }
                }
            },
            // A line, a non-circular conic, a spiric or a spline, or null
            // scaffolding (which tier 2 refuses upstream): not one
            // circle's arc.
            Some(
                geom::Curve3::Line { .. }
                | geom::Curve3::Ellipse { .. }
                | geom::Curve3::Spiric { .. }
                | geom::Curve3::Nurbs(_),
            )
            | None => one_circle = false,
        }
    }
    Ok(circle.filter(|_| one_circle))
}

/// **Boundary containment** for an on-carrier point against a CURVED
/// face: the shared boundary pre-pass ([`boundary_pre_pass`]), and
/// nothing after it. `None` is the honest remainder: this walk answers
/// about the BOUNDARY only, and the interior/exterior question belongs
/// to [`curved_face_containment`].
///
/// # Errors
///
/// [`ContainError`] — sliver escalations or unwalkable topology.
pub(super) fn curved_boundary_containment<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    q: Point3<T>,
    band: Band,
) -> Result<Option<(FaceContainment, MarginDiag)>, ContainError> {
    let loops = face_loops(body, face)?;
    Ok(match boundary_pre_pass(body, &loops, q, band)? {
        PrePass::On(on, margin) => Some((on, margin)),
        PrePass::Off(_) => None,
    })
}

/// **The shared boundary pre-pass**: vertex coincidence over ALL
/// loops FIRST — an edge-interior verdict must never shadow a vertex
/// coincidence, across loops exactly as within one (the invariant
/// [`contfp`] always stated; running it per loop let an outer-edge
/// hit shadow a ring vertex, fixed here with its red-then-green row
/// below) — then edge interiors over all loops, each edge on the row
/// its carrier has: a `Line` is the distance to its closed segment
/// ([`crate::ray_walk::on_segment`]), a circle or an ellipse is asked its
/// own conic and trim, and a spiric or a spline is read piece by piece
/// on its own carrier — all through
/// [`crate::splitting::containment::LoopEdge::contact`], the one
/// boundary reading the carrier walk runs too, so a point this pass
/// places off an edge is off it for the walk. This is [`contfp`]'s ONE
/// boundary pass: the walk after it trusts it and runs none of its own.
/// Rows, one home: `bool_contact_vertex`, `bool_contact_edge{,_length}`,
/// — for the conic disposition — `bool_contact_arc{,_span,_end,_trim}`,
/// for the spiric `bool_contact_spiric{,_end,_clear,_leaf}`, and for the
/// spline `bool_contact_spline{,_end,_clear,_leaf}`.
fn boundary_pre_pass<T: Decide>(
    body: &Body<T>,
    loops: &[LoopKey],
    q: Point3<T>,
    band: Band,
) -> Result<PrePass<T>, ContainError> {
    for &lk in loops {
        let cycle = loop_cycle_points(body, lk)?;
        for (v, _, p) in &cycle {
            if let Some(margin) = super::one_vertex_at(q, *p, band)
                .map_err(|diag| ContainError::on(LoopDecision::Boundary, diag))?
            {
                return Ok(PrePass::On(FaceContainment::OnVertex(*v), margin));
            }
        }
    }
    let mut read = Vec::with_capacity(loops.len());
    for &lk in loops {
        let OnLoop { lp, edge: i, on } = match carrier_loop(body, lk, ROWS.conic, band)?
            .place_off(q, ROWS, band)
            .map_err(|e| ContainError::Escalated {
                decision: Some(LoopDecision::Boundary.into()),
                escalation: e.escalation,
                diag: e.diag,
            })? {
            Ok(off) => {
                read.push((lk, off));
                continue;
            }
            Err(on) => on,
        };
        match on {
            OnEdge::On(margin) => {
                return Ok(PrePass::On(FaceContainment::OnEdge(lp.keys[i]), margin));
            }
            // Within the band of a curved edge's END, which the vertex
            // pass above placed definitely clear of both of this edge's
            // vertices. The end is read exactly (a circle through its
            // radius, an ellipse, a spiric or a spline as a distance from
            // `q`), so the two passes disagree only by as much as the
            // carrier's ends sit off the stored vertices, on its own row
            // (`bool_contact_arc_end_vertex`). Certification pins each
            // carrier end within the run's zero band of its vertex
            // (`geom_brep`'s `carrier_endpoint_{start,end}` rows), and the
            // ends are read through the same `Curve3::eval`; [`ArcEndGaps`]
            // says where each of its arms is read. An end in band of a
            // corner escalates on that margin; with every one decided,
            // the finding carries them.
            OnEdge::End => {
                let Some(carrier_ends) = lp.edges[i].carrier_ends() else {
                    unreachable!("only a curved arc reads End")
                };
                let ([c0, c1], v0, v1) = (
                    carrier_ends,
                    lp.verts[i],
                    lp.verts[(i + 1) % lp.verts.len()],
                );
                let gap = |c: Point3<T>, v: Point3<T>| {
                    decide_magnitude_reported(END_VERTEX, Margin::norm3(v - c), band)
                        .map_err(|diag| ContainError::on(ContainDecision::ArcEnd, diag))
                };
                let gaps = [[gap(c0, v0)?, gap(c0, v1)?], [gap(c1, v0)?, gap(c1, v1)?]];
                return Err(ContainError::ArcEndClearOfCorners(Box::new(ArcEndGaps {
                    gaps,
                    band,
                })));
            }
        }
    }
    Ok(PrePass::Off(read))
}

/// A loop the boundary pass read `q` off every edge of.
type ReadLoop<T> = (LoopKey, OffLoop<T>);

/// What [`boundary_pre_pass`] found: a boundary verdict, or every loop
/// read on its carriers — with `q` definitely off each of their edges —
/// for the walk that follows.
enum PrePass<T: geom_core::Real> {
    On(FaceContainment, MarginDiag),
    Off(Vec<ReadLoop<T>>),
}

/// The distance from a curved edge's carrier end to one of the edge's
/// stored vertices — a question about the body, apart from `q`'s own
/// distance to that end (`bool_contact_arc_end`,
/// `bool_contact_spiric_end`, `bool_contact_spline_end`).
const END_VERTEX: &str = "bool_contact_arc_end_vertex";

/// The pre-pass's rows for a straight edge: [`crate::ray_walk::on_segment`]
/// reads `segment` (the edge's own length, the degeneracy gate) and
/// `boundary` (the distance from `q` to the closed segment) and nothing
/// else, so `side` and `advance` are never minted.
const EDGE_ROWS: ParityRows = ParityRows {
    segment: "bool_contact_edge_length",
    boundary: "bool_contact_edge",
    side: "bool_contact_edge_side",
    advance: "bool_contact_edge_advance",
};

/// The pre-pass's rows ([`crate::splitting::containment::LoopEdge::contact`]).
const ROWS: BoundaryRows = BoundaryRows {
    line: &EDGE_ROWS,
    conic: ConicRows::bounded("bool_contact_arc_span", "bool_contact_arc_straddle"),
    spiric: ArcRows {
        end: "bool_contact_spiric_end",
        clear: "bool_contact_spiric_clear",
        on: "bool_contact_spiric",
        leaf: "bool_contact_spiric_leaf",
    },
    spline: ArcRows {
        end: "bool_contact_spline_end",
        clear: "bool_contact_spline_clear",
        on: "bool_contact_spline",
        leaf: "bool_contact_spline_leaf",
    },
};

/// **Point-in-face containment on a CURVED face** — the face-level
/// analog of the solid door's hit test, and the door the curved sweep
/// arm's frontier names.
///
/// The boundary walk runs first and unchanged
/// ([`curved_boundary_containment`]): an ON verdict is an ON verdict
/// whatever the surface is. Then the CARRIER: a face is a subset of its
/// surface, so a point definitely off the surface is definitely outside
/// the face, and saying so here is what keeps the region reading below
/// from answering about a point that is not on the surface at all.
///
/// Only then does this ask the interior question, of the face's region
/// read from its boundary edges, rings included, by the one reading the
/// ray lane takes too: [`super::sphere_region`], [`super::torus_region`]
/// and, for a cylinder or a cone, [`super::ruled_region`]
/// ([`sphere_face_containment`], [`torus_face_containment`],
/// [`ruled_face_containment`]). Nothing is read in a chart there. A
/// spline face has neither an implicit carrier nor a closed-form region,
/// so its carrier and its trim are both read on its chart
/// ([`super::spline_region`]).
///
/// `None` is therefore the honest remainder — a boundary edge the
/// reading has no closed form for, a spline face whose reading did not
/// settle, or a point on the region's boundary that the boundary walk
/// did not place — and the caller keeps its typed frontier door there.
///
/// # Errors
///
/// [`ContainError`] — sliver escalations or unwalkable topology.
pub fn curved_face_containment<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    q: Point3<T>,
    band: Band,
) -> Result<Option<FaceContainment>, ContainError> {
    Ok(match curved_face_placement(body, face, q, band)? {
        CurvedPlacement::OffCarrier => Some(FaceContainment::Out),
        CurvedPlacement::Trim(v) => v,
    })
}

/// [`curved_face_containment`] with its two kinds of `Out` kept apart.
///
/// A point outside a curved face is outside for one of two reasons, and
/// a caller that put the point ON the carrier must not read them as one:
///
/// - [`CurvedPlacement::OffCarrier`]: the point is definitely off the
///   face's SURFACE. For a caller that certified the point onto that
///   surface — a certified root, a residual that decided `Zero` — this
///   CONTRADICTS its own certificate, and the only honest answer is to
///   keep its typed door.
/// - `Trim(Some(FaceContainment::Out))`: the point is on the surface and
///   the face's region excludes it — a sibling face's incidence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CurvedPlacement {
    /// Definitely off the face's carrier surface.
    OffCarrier,
    /// On the carrier (or not tested yet, where the boundary walk
    /// already answered): the trim's verdict, `None` for no verdict.
    Trim(Option<FaceContainment>),
}

/// **Where a point lies against a face of any kind**: the one read
/// that places a vertex-on-face record, at rest by the census's confirm
/// and in a boolean's carry onto the pieces of a divided face, and a
/// section witness in its face. On a plane, `q` is first decided on the
/// plane by `on_plane` (the caller's predicate) and `OffCarrier` is that
/// decision's nonzero, then [`contfp`] places it; on a curved face its
/// region places it ([`curved_face_placement`]). `Trim(None)` is the
/// curved door's remainder, a face it does not read.
///
/// # Errors
///
/// [`PlacementError::OnPlane`] where the on-plane margin is in band, and
/// [`PlacementError::Door`] carrying the containment door's own refusal.
pub(crate) fn face_placement<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    q: Point3<T>,
    band: Band,
    on_plane: &'static str,
) -> Result<CurvedPlacement, PlacementError> {
    let f = proven(&body.faces, face, EntityId::Face);
    match *body.face_surface_linked(face, f) {
        geom::Surface::Plane { origin, normal, .. } => {
            match decide(on_plane, Margin::of((q - origin).dot(normal)), band) {
                Ok(Sign::Zero) => contfp(body, face, normal, q, band)
                    .map(|at| CurvedPlacement::Trim(Some(at)))
                    .map_err(PlacementError::Door),
                Ok(Sign::Positive | Sign::Negative) => Ok(CurvedPlacement::OffCarrier),
                Err(diag) => Err(PlacementError::OnPlane(diag)),
            }
        }
        _ => curved_face_placement(body, face, q, band).map_err(PlacementError::Door),
    }
}

/// Why [`face_placement`] placed nothing.
#[derive(Clone, Debug)]
pub(crate) enum PlacementError {
    /// The point's margin from the face's plane is in band.
    OnPlane(Indeterminate),
    /// The containment door refused, its refusal carried whole.
    Door(ContainError),
}

/// The placement behind [`curved_face_containment`] (see
/// [`CurvedPlacement`]); the public door is its projection.
///
/// # Errors
///
/// As [`curved_face_containment`].
pub(crate) fn curved_face_placement<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    q: Point3<T>,
    band: Band,
) -> Result<CurvedPlacement, ContainError> {
    curved_face_placement_decided(body, face, q, band).map(|(at, _)| at)
}

/// [`curved_face_placement`], with the margin that decided a boundary
/// verdict (`OnEdge`, `OnVertex`), as [`contfp_decided`] keeps it.
///
/// # Errors
///
/// As [`curved_face_containment`].
pub(crate) fn curved_face_placement_decided<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    q: Point3<T>,
    band: Band,
) -> Result<(CurvedPlacement, Option<MarginDiag>), ContainError> {
    if let Some((v, margin)) = curved_boundary_containment(body, face, q, band)? {
        return Ok((CurvedPlacement::Trim(Some(v)), Some(margin)));
    }
    curved_interior_placement(body, face, q, band).map(|at| (at, None))
}

/// [`curved_face_placement`] past the boundary pre-pass.
fn curved_interior_placement<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    q: Point3<T>,
    band: Band,
) -> Result<CurvedPlacement, ContainError> {
    let face_data = proven(&body.faces, face, EntityId::Face);
    match *body.face_surface_linked(face, face_data) {
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let w = q - origin;
            let radial = w - axis * w.dot(axis);
            ruled_face_containment(body, face, radial.norm() - radius, q, band)
        }
        // The whole DOUBLE cone ([`geom_brep::cone_elevation`] with no
        // nappe), because that is the surface the face's carrier states:
        // a point on the mirror nappe is ON the carrier, and the face's
        // region puts it outside the face.
        geom::Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let elevation = geom_brep::cone_elevation(apex, axis, half_angle, None, q);
            ruled_face_containment(body, face, elevation, q, band)
        }
        geom::Surface::Sphere { center, radius, .. } => {
            sphere_face_containment(body, face, center, radius, q, band)
        }
        geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => torus_face_containment(
            body,
            face,
            TorusCarrier {
                center,
                axis,
                major_radius,
                minor_radius,
            },
            q,
            band,
        ),
        geom::Surface::Nurbs(_) | geom::Surface::Approx(_) => {
            super::spline_region::spline_face_placement(body, face, q, band)
        }
        geom::Surface::Plane { .. } => Ok(CurvedPlacement::Trim(None)),
    }
}

/// The SPHERE arm of [`curved_face_containment`], reached after the
/// shared boundary walk.
///
/// The CARRIER first, as on every chart (a face is a subset of its
/// surface, so a point definitely off the sphere is definitely outside
/// the face), then the face's region, read from its boundary arcs by the
/// one reading the ray lane takes too ([`super::sphere_region`]). A face
/// closed on its own surface is handed to this door as a trimmed one and
/// read the same way: every edge of it is a seam, so its region is the
/// whole sphere.
///
/// `None` is the honest remainder: a boundary edge that is not a circle
/// arc, or a point on the region's boundary that the walk above did not
/// place. A region no ray settles refuses as a planar face's does.
fn sphere_face_containment<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    center: Point3<T>,
    radius: T,
    q: Point3<T>,
    band: Band,
) -> Result<CurvedPlacement, ContainError> {
    match decide(
        "bool_curved_contain_carrier",
        Margin::of((q - center).norm() - radius),
        band,
    ) {
        Ok(Sign::Zero) => {}
        Ok(Sign::Positive | Sign::Negative) => return Ok(CurvedPlacement::OffCarrier),
        Err(diag) => return Err(ContainError::on(ContainDecision::Carrier, diag)),
    }
    let region = match super::sphere_region::sphere_face_region(body, face, center, radius) {
        Ok(Some(region)) => region,
        Ok(None) => return Ok(CurvedPlacement::Trim(None)),
        Err(e) => return Err(solid_err(e)),
    };
    region_placement(face, region.contains(q, band))
}

/// A region reading's verdict ([`super::sphere_region`],
/// [`super::torus_region`]) as this door states it: `None` on the
/// region's boundary, which the boundary walk did not place.
fn region_placement(
    face: FaceKey,
    reading: Result<Option<bool>, RegionRefusal>,
) -> Result<CurvedPlacement, ContainError> {
    match reading {
        Ok(Some(true)) => Ok(CurvedPlacement::Trim(Some(FaceContainment::In))),
        Ok(Some(false)) => Ok(CurvedPlacement::Trim(Some(FaceContainment::Out))),
        Ok(None) => Ok(CurvedPlacement::Trim(None)),
        Err(RegionRefusal::Escalated(diag)) => Err(ContainError::Escalated {
            decision: None,
            escalation: Escalation::Margin,
            diag,
        }),
        Err(RegionRefusal::RayExhausted { .. }) => Err(ContainError::RayExhausted),
        Err(e @ (RegionRefusal::WoundPastPeriod | RegionRefusal::Unreadable(_))) => {
            Err(solid_err(e.of_face(face)))
        }
    }
}

/// One torus carrier's data, as [`torus_face_containment`] reads it.
struct TorusCarrier<T: geom_core::Real> {
    center: Point3<T>,
    axis: Vec3<T>,
    major_radius: T,
    minor_radius: T,
}

/// The TORUS chart's arm of [`curved_face_containment`], reached after
/// the shared boundary walk.
///
/// The same steps as the sphere arm, in the same order and for the same
/// reasons: the CARRIER first (the point's elevation off the tube,
/// `√((ρ − R)² + h²) − r`, which is exact distance to a ring torus), then
/// the face's region, read from its boundary edges by the one reading the
/// ray lane takes too ([`super::torus_region`]). This door is asked about
/// ONE face, so it reads that face's own region whatever group it sits
/// in: a donut's two faces are each bounded by the parallels they share,
/// and a face closed on its own surface has only seams, so its region is
/// the whole torus.
///
/// `None` is the honest remainder: a boundary edge that lies in no plane,
/// or a point on the region's boundary that the walk above did not place.
/// A region no path settles refuses as a planar face's does.
fn torus_face_containment<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    carrier: TorusCarrier<T>,
    q: Point3<T>,
    band: Band,
) -> Result<CurvedPlacement, ContainError> {
    let TorusCarrier {
        center,
        axis,
        major_radius,
        minor_radius,
    } = carrier;
    let elevation =
        super::solid_contain::torus_elevation(center, axis, major_radius, minor_radius, q);
    match decide("bool_curved_contain_carrier", Margin::of(elevation), band) {
        Ok(Sign::Zero) => {}
        Ok(Sign::Positive | Sign::Negative) => return Ok(CurvedPlacement::OffCarrier),
        Err(diag) => return Err(ContainError::on(ContainDecision::Carrier, diag)),
    }
    let region = match super::torus_region::torus_face_region(body, face) {
        Ok(Some(region)) => region,
        Ok(None) => return Ok(CurvedPlacement::Trim(None)),
        Err(e) => return Err(solid_err(e)),
    };
    region_placement(face, region.contains(q, band))
}

/// The CYLINDER and CONE arm of [`curved_face_containment`], reached
/// after the shared boundary walk.
///
/// The same steps as the sphere arm, in the same order and for the same
/// reasons: the CARRIER first (`elevation`, the point's distance off the
/// wall or the double cone), then the face's region, read from its
/// boundary edges by the one reading the ray lane takes too
/// ([`super::ruled_region`]). A cone's apex is read like any other point
/// of it.
///
/// `None` is the honest remainder: a boundary edge that is not a line, a
/// circle or an ellipse, or a point on the region's boundary that the
/// walk above did not place. A region no path settles refuses as a
/// planar face's does.
fn ruled_face_containment<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    elevation: T,
    q: Point3<T>,
    band: Band,
) -> Result<CurvedPlacement, ContainError> {
    match decide("bool_curved_contain_carrier", Margin::of(elevation), band) {
        Ok(Sign::Zero) => {}
        Ok(Sign::Positive | Sign::Negative) => return Ok(CurvedPlacement::OffCarrier),
        Err(diag) => return Err(ContainError::on(ContainDecision::Carrier, diag)),
    }
    let region = match super::ruled_region::ruled_face_region(body, face) {
        Ok(Some(region)) => region,
        Ok(None) => return Ok(CurvedPlacement::Trim(None)),
        Err(e) => return Err(solid_err(e)),
    };
    region_placement(face, region.contains(q, band))
}

/// **Whether `q` lies on `face`'s side of its surface cycles `keep`**:
/// the face's region reading ([`curved_face_containment`]'s door for its
/// kind) with its boundary cut to those cycles' edges
/// ([`super::region::keep_cycles`]), so a cycle is read alone, or a
/// torus's essential pair together. `q` is a point of the face's surface,
/// which the caller certifies (no carrier decision is read here).
/// `Some(Some(side))`, `Some(None)` where `q` is on one of the cycles,
/// and `None` where no door reads the face: a plane, a spline chart, or
/// an edge of a kind its door does not admit (`admits`, each door's).
///
/// # Errors
///
/// The door's refusal ([`RegionRefusal`]): `q` in the band of an edge
/// kept, no path decided, or an arc wound past a turn.
pub(crate) fn cycle_side<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    keep: &[usize],
    q: Point3<T>,
    band: Band,
) -> Result<Option<Option<bool>>, RegionRefusal> {
    let face_data = proven(&body.faces, face, EntityId::Face);
    match *body.face_surface_linked(face, face_data) {
        geom::Surface::Cylinder { .. } | geom::Surface::Cone { .. } => {
            match super::ruled_region::ruled_face_region(body, face) {
                Ok(Some(region)) => region.keep_cycles(keep).contains(q, band).map(Some),
                Ok(None) | Err(_) => Ok(None),
            }
        }
        geom::Surface::Sphere { center, radius, .. } => {
            match super::sphere_region::sphere_face_region(body, face, center, radius) {
                Ok(Some(region)) => region.keep_cycles(keep).contains(q, band).map(Some),
                Ok(None) | Err(_) => Ok(None),
            }
        }
        geom::Surface::Torus { .. } => match super::torus_region::torus_face_region(body, face) {
            Ok(Some(region)) => region.keep_cycles(keep).contains(q, band).map(Some),
            Ok(None) | Err(_) => Ok(None),
        },
        geom::Surface::Plane { .. } | geom::Surface::Nurbs(_) | geom::Surface::Approx(_) => {
            Ok(None)
        }
    }
}

pub(super) fn solid_err(e: super::solid_contain::PointInSolidError) -> ContainError {
    match e {
        super::solid_contain::PointInSolidError::Escalated { diag, .. } => {
            ContainError::Escalated {
                decision: None,
                escalation: Escalation::Margin,
                diag,
            }
        }
        e => ContainError::Curved(e),
    }
}

/// Is `q` ON the circle `(center, axis, radius)`? `Some((radial,
/// |radial|))` on it — the radial offset from the axis, which the
/// angular half of an arc question is measured from — and `None`
/// definitely off it.
///
/// **`bool_contact_arc` has one body, and this is it.** The row's
/// quantity is the exact distance from the point to the circle: the
/// radial miss and the axial miss are orthogonal, so their hypotenuse
/// is exact and one row covers both ways off the carrier. A distance
/// has no negative sign, so the row is a magnitude
/// ([`geom_core::k_stats::decide_magnitude`]).
///
/// Its caller, the boolean reduction's point split, turns a definite
/// miss into a broken-invariant refusal, because a split point was
/// placed on that carrier by an exact row upstream. The boundary
/// pre-pass reads a circle edge through the point-on-span reading
/// instead ([`crate::splitting::containment::LoopEdge::contact`]), whose
/// gap row reads this distance at the point's exact foot
/// (`work/cleave/point-on-circle-is-a-second-point-on-carrier-reading.md`).
pub(super) fn point_on_circle<T: Decide>(
    q: Point3<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    band: Band,
) -> Result<Option<(Vec3<T>, T)>, Indeterminate> {
    let d = crate::splitting::containment::circle_miss(q, center, axis, radius);
    // A `sqrt`: the magnitude door's precondition.
    match geom_core::k_stats::decide_magnitude("bool_contact_arc", Margin::of(d), band)? {
        Magnitude::Zero => {
            let w = q - center;
            let radial = w - axis * w.dot(axis);
            Ok(Some((radial, radial.norm())))
        }
        Magnitude::Positive => Ok(None),
    }
}

/// The loop's (start vertex, half-edge, point) cycle, for a loop key
/// read out of a face record. A lone-vertex loop is
/// [`ContainError::EmptyLoop`]: it bounds no region.
#[allow(clippy::type_complexity)]
fn loop_cycle_points<T: Decide>(
    body: &Body<T>,
    lk: LoopKey,
) -> Result<Vec<(VertexKey, crate::entity::HalfEdgeKey, Point3<T>)>, ContainError> {
    let first = match proven(&body.loops, lk, EntityId::Loop).boundary {
        LoopBoundary::Cycle { first } => first,
        LoopBoundary::Empty { .. } => return Err(ContainError::EmptyLoop(lk)),
    };
    Ok(body
        .loop_walk(first)
        .closed("loop", first)
        .into_iter()
        .map(|he| {
            let start = proven(&body.half_edges, he, EntityId::HalfEdge).start;
            let p = body.linked_vertex_point(start, EntityId::HalfEdge(he), "start");
            (start, he, p)
        })
        .collect())
}

#[cfg(test)]
#[path = "torus_landing_rows.rs"]
mod torus_landing_rows;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::entity::LoopBoundary;
    use geom_core::Tol;

    /// Each escalation ends as its decision and its reading give it (D4
    /// ¶1 (i)), pinned by literal text over the arms each site can raise:
    /// a sized decision's lever with the tolerance its in-band margin
    /// gives on a side it passes, its lever alone on a side it refuses or
    /// on a straddle of two bounds, a lever-only decision's lever on every
    /// arm, and the unreadable-margin note on a poisoned margin alone. A
    /// build and a read at rest end alike on every one of these arms.
    #[test]
    fn an_escalation_ends_as_its_decision_gives_it() {
        use geom_brep::recourse::Reading;
        use geom_core::MarginDiag;
        let band = Band::new(1e-9, 1e-8).unwrap();
        let diag = |margin| Indeterminate {
            margin,
            band,
            predicate: Some("a_margin"),
            terminal_sliver: false,
        };
        let above = diag(MarginDiag::value(5e-9));
        let below = diag(MarginDiag::value(-5e-9));
        let poisoned = diag(MarginDiag::INVALID);
        const NOTE: &str = "an unreadable margin may indicate a kernel bug worth reporting";
        const NOTE_AT_REST: &str =
            "an unreadable margin may indicate a kernel or file defect worth reporting";
        const BOUNDARY: &str =
            "Recourse: move the point exactly onto the boundary or clearly off it";
        const RAY: &str = "Recourse: nudge the point so no boundary corner lines up with it";
        const ARC: &str =
            "Recourse: move the geometry so this arc stays clearly short of a full turn";
        const PLANE: &str =
            "Recourse: move the geometry so the face is flat and the point lies on it";
        const END: &str = "Recourse: move the point clear of the arc's end";
        const CARRIER: &str =
            "Recourse: move the point exactly onto the face's surface or clearly off it";
        const UNNAMED: &str = "Recourse: move the point clearly inside or outside the face";
        let loop_ = |d| Some(ContainDecision::Loop(d));
        let tighten = |lever: &str, size: &str| {
            format!("{lever}, or, if this {size} is intended, tighten the tolerance below 5e-10 m")
        };
        use Escalation::{Margin, Straddle};
        let rows = [
            (
                loop_(LoopDecision::Boundary),
                Margin,
                above,
                tighten(BOUNDARY, "distance"),
            ),
            (
                loop_(LoopDecision::Boundary),
                Margin,
                below,
                tighten(BOUNDARY, "distance"),
            ),
            (
                loop_(LoopDecision::Boundary),
                Margin,
                poisoned,
                format!("{BOUNDARY}; {NOTE}"),
            ),
            (
                loop_(LoopDecision::Boundary),
                Straddle,
                poisoned,
                BOUNDARY.to_owned(),
            ),
            (loop_(LoopDecision::Ray), Margin, above, RAY.to_owned()),
            (
                loop_(LoopDecision::Ray),
                Margin,
                poisoned,
                format!("{RAY}; {NOTE}"),
            ),
            (loop_(LoopDecision::ArcSpan), Margin, below, ARC.to_owned()),
            (
                loop_(LoopDecision::ArcSpan),
                Straddle,
                poisoned,
                ARC.to_owned(),
            ),
            (loop_(LoopDecision::Plane), Margin, above, PLANE.to_owned()),
            (Some(ContainDecision::ArcEnd), Margin, above, END.to_owned()),
            (
                Some(ContainDecision::ArcEnd),
                Margin,
                poisoned,
                format!("{END}; {NOTE}"),
            ),
            (
                Some(ContainDecision::OneCircle),
                Margin,
                above,
                tighten(
                    "Recourse: put the loop's arcs on one circle or on clearly different ones",
                    "gap between circles",
                ),
            ),
            (
                Some(ContainDecision::Carrier),
                Margin,
                above,
                CARRIER.to_owned(),
            ),
            (
                Some(ContainDecision::Carrier),
                Margin,
                below,
                CARRIER.to_owned(),
            ),
            (None, Margin, above, UNNAMED.to_owned()),
            (None, Margin, poisoned, format!("{UNNAMED}; {NOTE}")),
        ];
        for decision in ContainDecision::ALL {
            assert!(
                rows.iter().any(|(d, ..)| *d == Some(decision)),
                "{decision:?} has no pinned ending"
            );
        }
        for (decision, escalation, cause, ending) in rows {
            let row = format!("{decision:?} {escalation:?} {}", cause.margin);
            // A poisoned margin's note names the file at rest.
            let stored = match ending.strip_suffix(NOTE) {
                Some(lever) => format!("{lever}{NOTE_AT_REST}"),
                None => ending.clone(),
            };
            for reading in [Reading::Build, Reading::AtRest] {
                let ending = match reading {
                    Reading::Build => &ending,
                    Reading::AtRest => &stored,
                };
                assert_eq!(
                    &placement_ending(decision, escalation, &cause, reading),
                    ending,
                    "{row} ({reading:?})"
                );
            }
            // One lever source: the ending opens with the decision's lever.
            assert!(
                ending.starts_with(&format!("Recourse: {}", placement_lever(decision))),
                "{row}"
            );
            let refusal = ContainError::Escalated {
                decision,
                escalation,
                diag: cause,
            };
            assert_eq!(
                refusal.to_string(),
                format!(
                    "contfp: {} is undecided: {}. {ending}",
                    placement_subject(decision),
                    cause.payload()
                ),
                "{row}: the refusal renders its payload and the one ending"
            );
            if let Some(ContainDecision::Loop(decision)) = decision {
                let walk = PointInLoopError::Escalated {
                    r#loop: LoopKey::default(),
                    decision,
                    escalation,
                    diag: cause,
                }
                .to_string();
                assert!(walk.ends_with(&format!(". {ending}")), "{row}: {walk}");
            }
            let at_rest = crate::ValidationError::RingNestingUndecided {
                face: FaceKey::default(),
                ring: LoopKey::default(),
                source: refusal,
            }
            .to_string();
            assert!(
                at_rest.ends_with(&format!(
                    "{} is undecided at this tolerance. {stored}",
                    placement_subject(decision)
                )),
                "{row}: {at_rest}"
            );
        }
    }

    /// **The pre-pass tags the boundary question**: a point at the band's
    /// midpoint off a straight outer edge of the holed box's top face,
    /// clear of its corners, escalates on the loop walk's `Boundary`
    /// decision with its margin, and ends in the valued tighten.
    #[test]
    fn contfp_tags_a_point_in_band_of_an_edge_as_the_boundary_question() {
        use geom_brep::recourse::Reading;
        let body = crate::fixtures::ops_holed_box(Tol::witness()).body;
        let band = Band::linear(Tol::witness()).unwrap();
        let m = 0.5 * (band.zero() + band.escalate());
        let got = body
            .faces
            .iter()
            .filter(|(_, f)| !f.rings.is_empty())
            .map(|(k, _)| {
                contfp(
                    &body,
                    k,
                    Vec3::new(0.0, 0.0, 1.0),
                    Point3::new(0.5, m, 1.0),
                    band,
                )
            })
            .find(|got| matches!(got, Err(ContainError::Escalated { .. })))
            .expect("the top face escalates at its edge");
        let Err(ContainError::Escalated {
            decision,
            escalation,
            diag,
        }) = got
        else {
            unreachable!("found as an escalation")
        };
        assert_eq!(
            decision,
            Some(ContainDecision::Loop(LoopDecision::Boundary))
        );
        assert_eq!(escalation, Escalation::Margin);
        let ending = placement_ending(decision, escalation, &diag, Reading::AtRest);
        assert!(
            ending.contains("if this distance is intended, tighten the tolerance below"),
            "{ending}"
        );
    }

    /// The cross-loop shadowing row (red-then-green, M9-3 fix pass):
    /// `contfp`'s stated invariant — an edge-interior verdict never
    /// shadows a vertex coincidence — must hold ACROSS loops, not
    /// only within one. The scaffold moves a RING vertex of the holed
    /// box's ringed face to within the zero band of an OUTER edge's
    /// interior and queries that exact point: the per-loop pre-pass
    /// answered `OnEdge` (the outer loop's edge pass ran before the
    /// ring's vertex pass); the shared all-loops-vertex-first pass
    /// answers `OnVertex`. The body is a POINT SCAFFOLD only — the
    /// pre-pass consumes vertex points and chords derived from them,
    /// and nothing else of the (now geometrically inconsistent) box
    /// is read.
    #[test]
    fn a_ring_vertex_is_never_shadowed_by_an_outer_edge() {
        let holed = crate::fixtures::ops_holed_box(Tol::witness());
        let mut body = holed.body;
        let band = Band::linear(Tol::witness()).unwrap();
        // The ringed face whose outer cycle lies at z = 1 (the top).
        let (face, ring) = body
            .faces
            .iter()
            .find_map(|(k, f)| {
                let ring = *f.rings.first()?;
                let LoopBoundary::Cycle { first } = body.loops.get(f.outer)?.boundary else {
                    return None;
                };
                let top = body
                    .loop_cycle(first)?
                    .into_iter()
                    .all(|he| body.half_edge_start_point(he).is_some_and(|p| p.z == 1.0));
                (top).then_some((k, ring))
            })
            .expect("the holed box has a ringed top face");
        let ring_vertex = {
            let LoopBoundary::Cycle { first } = body.loops[ring].boundary else {
                panic!("the ring is a cycle");
            };
            body.half_edges[first].start
        };
        // Move the ring vertex within the zero band of the outer
        // edge from (0,0,1) to (1,0,1), strictly interior in span.
        let q = geom_core::Point3::new(0.5, 4e-10, 1.0);
        let pk = body.vertices[ring_vertex].point;
        body.points[pk] = q;
        let got = contfp(&body, face, geom_core::Vec3::new(0.0, 0.0, 1.0), q, band)
            .expect("the pre-pass decides");
        assert_eq!(
            got,
            FaceContainment::OnVertex(ring_vertex),
            "the ring vertex must win over the outer edge's interior"
        );
    }

    /// A loop of straight edges bounds no disc, whatever its shape:
    /// read as one, check 9 would decide it against another loop from a
    /// circle no boundary edge rides.
    #[test]
    fn a_polygon_loop_bounds_no_disc() {
        let holed = crate::fixtures::ops_holed_box(Tol::witness());
        let body = holed.body;
        let band = Band::linear(Tol::witness()).unwrap();
        let mut loops = 0;
        for (_, f) in body.faces() {
            for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
                loops += 1;
                assert!(
                    loop_circle(&body, lk, band)
                        .expect("the box walks")
                        .is_none(),
                    "a straight-edged loop bounds no disc"
                );
            }
        }
        assert!(loops > 0, "the fixture must have loops to check");
    }

    /// The point a band of `K < sqrt(2)` reads at the half-disc's arc end
    /// and clear of its corner: `0.95·zero` off the circle and `0.95·zero`
    /// along it from the end, so `≈ 1.34·zero` from the corner at the end.
    fn beside_the_arc_end(end: Point3<f64>, zero: f64) -> Point3<f64> {
        end + Vec3::new(-0.95 * zero, 0.95 * zero, 0.0)
    }

    /// **A point at an arc's end the vertex pass placed clear of its
    /// corners is a finding, not an escalation** (D4 ¶1 (iv)), through the
    /// public door on a body the public operators built.
    ///
    /// - **An end off both corners**, here at a band finer than the
    ///   run's: the cap's corner stands `off = zero/2` from its arc's
    ///   end, certified at the run's band, and `contfp` reads it at a band
    ///   of zero `off/50`. The finding leads with the gap to the end's own
    ///   corner, `off`. It logs no escalation, each end is decided once
    ///   against each corner, and it ends in the arc end's lever at a
    ///   build and through the Boolean.
    /// - **The band's own parting no longer parts them**: on a cap whose
    ///   corner is its arc's end, the boundary reading puts a point at the
    ///   end only on its raw chord to the end's point, the distance the
    ///   vertex pass reads to the corner. A point `≈ 1.34·zero` from the
    ///   corner at `K = 1.25` is clear of both, with its foot within the
    ///   zero band of the end along the arc, which the side of the end
    ///   cannot place beside a chord past the band: it escalates on that
    ///   straddle (`curve_pair_span_end_straddle`). At the run's `K` the
    ///   same point is the vertex reading's escalation.
    #[test]
    fn a_point_at_an_arc_end_clear_of_its_corners_is_a_finding() {
        use crate::boolean::{BooleanError, Operand};
        use crate::test_support_fixtures::half_disc_cap;
        use geom_core::{ErrorTextReading, k_stats::Bracket};
        let tol = Tol::witness();
        let run = Band::linear(tol).unwrap();
        let zero = run.zero();
        let lever = "Recourse: move the point clear of the arc's end";
        let read = |m: MarginDiag| match m.diagnostic_f64_for_error_text() {
            ErrorTextReading::Value(v) => v,
            other => panic!("a point reading: {other:?}"),
        };
        let finding = |what: &str, body: &Body<f64>, face, q, band| {
            let bracket = Bracket::open();
            let got = contfp(body, face, Vec3::unit_z(), q, band);
            let log = bracket.finish();
            let Err(ContainError::ArcEndClearOfCorners(gaps)) = &got else {
                panic!("{what}: the finding: {got:?}");
            };
            assert!(log.escalations.is_empty(), "{what}: no escalation: {log:?}");
            assert_eq!(
                log.verdicts
                    .iter()
                    .filter(|v| v.predicate == END_VERTEX)
                    .count(),
                4,
                "{what}: each end decided once against each corner"
            );
            let built = got.as_ref().unwrap_err().to_string();
            assert!(built.ends_with(lever), "{what}: {built}");
            let boolean = BooleanError::PointInFaceRefused {
                operand: Operand::A,
                face,
                refusal: got.as_ref().unwrap_err().clone(),
            }
            .to_string();
            assert!(boolean.ends_with(lever), "{what}: {boolean}");
            **gaps
        };
        let off = zero / 2.0;
        let fine = Band::new(off / 50.0, off / 5.0).unwrap();
        let (body, cap, end) = half_disc_cap(off, tol);
        let gaps = finding("off its corner", &body, cap, end, fine);
        let Some((own, far)) = gaps.off_end() else {
            panic!("an end lies off both corners at the finer band: {gaps:?}");
        };
        assert!(
            (read(own) - off).abs() <= 4.0 * f64::EPSILON && (read(far) - 2.0).abs() < 1e-9,
            "the finding leads with the end's own corner, `off` away: {own:e}, {far:e}"
        );
        let (body, cap, end) = half_disc_cap(0.0, tol);
        let q = beside_the_arc_end(end, zero);
        let narrow = Band::new(zero, 1.25 * zero).unwrap();
        assert!(
            matches!(
                contfp(&body, cap, Vec3::unit_z(), q, narrow),
                Err(ContainError::Escalated { diag, .. })
                    if diag.predicate == Some("curve_pair_span_end_straddle")
            ),
            "the band's own parting escalates on the end's straddle"
        );
        if run.escalate() / zero <= 1.4 {
            test_utils::vacuity::stood_down(
                "a_point_at_an_arc_end_clear_of_its_corners_is_a_finding",
                "the run's K is too narrow to read the corner gap in band",
            );
        } else {
            assert!(
                matches!(
                    contfp(&body, cap, Vec3::unit_z(), q, run),
                    Err(ContainError::Escalated {
                        decision: Some(ContainDecision::Loop(LoopDecision::Boundary)),
                        escalation: Escalation::Margin,
                        diag,
                    }) if diag.predicate == Some("bool_contact_vertex")
                ),
                "at the run's K the corner gap is in band, and escalates"
            );
        }
    }
}
