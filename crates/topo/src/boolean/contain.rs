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
//! chord, a circle or ellipse arc is crossed on its conic, a spiric arc
//! on its oval, and a spline edge, which has no crossing row, refuses
//! typed wherever it could matter.

use geom_brep::recourse::{LeverOnly, Reading, SizedPass};
use geom_core::k_stats::Magnitude;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::sphere_region::RegionRefusal;
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, LoopBoundary, LoopKey, VertexKey};
use crate::live::{linked, proven};
use crate::ray_walk::ParityRows;
use crate::splitting::containment::placement_sized;
use crate::splitting::containment::{
    BoundaryRows, CarrierLoop, ConicRows, EdgeContact, carrier_loop, carrier_loop_side,
};
use crate::splitting::spiric_arc::SpiricRows;
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
    /// Whether a cylinder face's azimuth window sweeps clearly less than
    /// a full turn: its gap to one, at the radius. Only a positive gap
    /// gives the trim a window to read.
    WindowPeriod,
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
        Some(ContainDecision::WindowPeriod) => {
            "whether a cylinder face sweeps less than a full turn"
        }
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
        Some(ContainDecision::WindowPeriod) => "keep the wall clearly short of a full turn",
        None => "move the point clearly inside or outside the face",
    }
}

/// **The escalations each site raises**: every decision paired with each
/// way its sites' readings can stand, and `None` for a refusal that names
/// no decision. A pair not here is raised nowhere.
pub const CONTAINMENT_RAISED: [(Option<ContainDecision>, Escalation); 13] = {
    use ContainDecision as C;
    use Escalation::{Decided, Margin, Straddle};
    use LoopDecision as L;
    [
        (None, Margin),
        (Some(C::Loop(L::Boundary)), Margin),
        (Some(C::Loop(L::Boundary)), Straddle),
        (Some(C::Loop(L::Boundary)), Decided),
        (Some(C::Loop(L::Ray)), Margin),
        (Some(C::Loop(L::ArcSpan)), Margin),
        (Some(C::Loop(L::ArcSpan)), Straddle),
        (Some(C::Loop(L::Plane)), Margin),
        (Some(C::ArcEnd), Margin),
        (Some(C::ArcEnd), Decided),
        (Some(C::OneCircle), Margin),
        (Some(C::Carrier), Margin),
        (Some(C::WindowPeriod), Margin),
    ]
};

impl ContainDecision {
    /// Every decision, for the rows that sample them.
    pub const ALL: [Self; 8] = [
        Self::Loop(LoopDecision::Boundary),
        Self::Loop(LoopDecision::Ray),
        Self::Loop(LoopDecision::ArcSpan),
        Self::Loop(LoopDecision::Plane),
        Self::ArcEnd,
        Self::OneCircle,
        Self::Carrier,
        Self::WindowPeriod,
    ];

    /// The one ending an escalation of this decision carries, read at
    /// `reading` (D4 ¶1 (i)), from the shared table.
    #[must_use]
    pub fn ending(self, escalation: Escalation, diag: &Indeterminate, reading: Reading) -> String {
        let arm = escalation.arm(diag);
        let lever = placement_lever(Some(self));
        match self {
            Self::Loop(d) => d.ending(escalation, diag, reading),
            Self::ArcEnd | Self::Carrier => LeverOnly { lever }.recourse(arm),
            Self::OneCircle => placement_sized(lever, "gap between circles", SizedPass::AnySign)
                .recourse(arm, reading),
            Self::WindowPeriod => {
                placement_sized(lever, "gap", SizedPass::Positive).recourse(arm, reading)
            }
        }
    }

    /// The decision's lever alone on `escalation`'s arm, with the
    /// unreadable-margin note on a poisoned margin: its ending at a door
    /// that offers no tolerance for it (the Boolean's, `refusal_routes`).
    #[must_use]
    pub fn lever_ending(self, escalation: Escalation, diag: &Indeterminate) -> String {
        LeverOnly {
            lever: placement_lever(Some(self)),
        }
        .recourse(escalation.arm(diag))
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
        None => LeverOnly {
            lever: placement_lever(None),
        }
        .recourse(escalation.arm(diag)),
    }
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
                "contfp: {} is undecided: {}. {}",
                placement_subject(*decision),
                diag.payload(),
                placement_ending(*decision, *escalation, diag, Reading::Build)
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
    let loops = face_loops(body, face)?;

    let read = match boundary_pre_pass(body, &loops, q, band)? {
        PrePass::On(on) => return Ok(on),
        PrePass::Off(read) => read,
    };

    // Interior/exterior: inside the outer loop AND outside every ring,
    // each read on its edges' own carriers by a walk that trusts the
    // pre-pass above — `q` is definitely off every edge — so it answers
    // inside or outside.
    let inside = |(lk, lp): &(LoopKey, CarrierLoop<T>)| -> Result<bool, ContainError> {
        Ok(carrier_loop_side(*lk, lp, normal, q, band)?)
    };
    let Some((outer, rings)) = read.split_first() else {
        unreachable!("the pre-pass reads every loop of the face, and a face has its outer loop")
    };
    if !inside(outer)? {
        return Ok(FaceContainment::Out);
    }
    for ring in rings {
        if inside(ring)? {
            return Ok(FaceContainment::Out);
        }
    }
    Ok(FaceContainment::In)
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
) -> Result<Option<FaceContainment>, ContainError> {
    let loops = face_loops(body, face)?;
    Ok(match boundary_pre_pass(body, &loops, q, band)? {
        PrePass::On(on) => Some(on),
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
/// own conic and trim, and a spiric is read piece by piece on its oval —
/// all through
/// [`crate::splitting::containment::LoopEdge::contact`], the one
/// boundary reading the carrier walk runs too, so a point this pass
/// places off an edge is off it for the walk. A spline edge gets no verdict: its
/// chord is a different curve, and the region walk refuses inside a
/// ball its locus lies in. This is [`contfp`]'s ONE boundary pass: the
/// walk after it trusts it and runs none of its own. Rows, one home:
/// `bool_contact_vertex`, `bool_contact_edge{,_length}`, — for the
/// conic disposition — `bool_contact_arc{,_span,_end,_trim}`, and for
/// the spiric `bool_contact_spiric{,_end,_clear,_leaf}`.
fn boundary_pre_pass<T: Decide>(
    body: &Body<T>,
    loops: &[LoopKey],
    q: Point3<T>,
    band: Band,
) -> Result<PrePass<T>, ContainError> {
    for &lk in loops {
        let cycle = loop_cycle_points(body, lk)?;
        for (v, _, p) in &cycle {
            if super::one_vertex(q, *p, band)
                .map_err(|diag| ContainError::on(LoopDecision::Boundary, diag))?
            {
                return Ok(PrePass::On(FaceContainment::OnVertex(*v)));
            }
        }
    }
    let mut read = Vec::with_capacity(loops.len());
    for &lk in loops {
        let lp = carrier_loop(body, lk, ROWS, band)?;
        let n = lp.verts.len();
        for (i, edge) in lp.edges.iter().enumerate() {
            let ends = (lp.verts[i], lp.verts[(i + 1) % n]);
            match edge
                .contact(ends, q, ROWS, band)
                .map_err(|e| ContainError::Escalated {
                    decision: Some(LoopDecision::Boundary.into()),
                    escalation: e.escalation,
                    diag: e.diag,
                })? {
                EdgeContact::On => return Ok(PrePass::On(FaceContainment::OnEdge(lp.keys[i]))),
                EdgeContact::Off | EdgeContact::Carrier | EdgeContact::Unread => {}
                // Within the band of a curved edge's END, which the vertex
                // pass above placed definitely clear of both of this edge's
                // vertices. The end is read exactly (a circle through its
                // radius, an ellipse or a spiric as a distance from `q`), so the two
                // passes disagree only by as much as the carrier's ends
                // sit off the stored vertices, on its own row
                // (`bool_contact_arc_end_vertex`). Certification pins each
                // carrier end to its vertex within the zero band of the
                // band the body was CERTIFIED at (`geom_brep`'s
                // `carrier_endpoint_{start,end}` rows); a body certified
                // at a coarser band than this query's carries up to that
                // band's `ε`, which is no defect, so the answer is always
                // an escalation — on that margin where it is in band, and
                // otherwise on the row itself.
                EdgeContact::End => {
                    let Some(carrier_ends) = edge.carrier_ends() else {
                        unreachable!("only a conic or spiric arc reads End")
                    };
                    for c in carrier_ends {
                        for v in [ends.0, ends.1] {
                            if let Err(diag) = decide(END_VERTEX, Margin::norm3(v - c), band) {
                                return Err(ContainError::on(ContainDecision::ArcEnd, diag));
                            }
                        }
                    }
                    // Every end decided against every vertex: the row,
                    // decided, still leaves the point in the band of the end.
                    return Err(ContainError::Escalated {
                        decision: Some(ContainDecision::ArcEnd),
                        escalation: Escalation::Decided,
                        diag: crate::invalid_margin::invalid(band, END_VERTEX),
                    });
                }
            }
        }
        read.push((lk, lp));
    }
    Ok(PrePass::Off(read))
}

/// What [`boundary_pre_pass`] found: a boundary verdict, or every loop
/// read on its carriers — with `q` definitely off each of their edges —
/// for the walk that follows.
enum PrePass<T: geom_core::Real> {
    On(FaceContainment),
    Off(Vec<(LoopKey, CarrierLoop<T>)>),
}

/// The distance from a conic or spiric edge's carrier end to one of the
/// edge's stored vertices — a question about the body, apart from `q`'s
/// own distance to that end (`bool_contact_arc_end`,
/// `bool_contact_spiric_end`).
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
    conic: ConicRows {
        span: "bool_contact_arc_span",
        on: "bool_contact_arc",
        end: "bool_contact_arc_end",
        trim: "bool_contact_arc_trim",
        straddle: "bool_contact_arc_straddle",
    },
    spiric: SpiricRows {
        end: "bool_contact_spiric_end",
        clear: "bool_contact_spiric_clear",
        on: "bool_contact_spiric",
        leaf: "bool_contact_spiric_leaf",
    },
};

/// **Point-in-face containment on a CURVED chart** — the face-level
/// analog of the solid door's chart trim
/// ([`super::solid_contain::point_on_wall_in_face`]), and the door the
/// curved sweep arm's frontier names.
///
/// The boundary walk runs first and unchanged
/// ([`curved_boundary_containment`]): an ON verdict is an ON verdict
/// whatever the chart is. Then the CARRIER: a face is a subset of its
/// surface, so a point definitely off the surface is definitely
/// outside the face, and saying so here is what keeps the
/// parameter-domain trim below from answering about a point that is
/// not on the chart at all.
///
/// Only then does this ask the interior question, per chart: the
/// sphere, torus and cone arms read their own chart windows
/// ([`sphere_face_containment`], [`torus_face_containment`],
/// [`cone_face_containment`]), and a
/// **cylinder wall of the ISO-BOUNDED class** answers it this way:
///
/// - the face carries no rings (a ring is a hole the rectangle below
///   does not model, and answering `In` inside one would be wrong);
/// - every boundary edge is a RIM (a circle coaxial with the wall, at
///   the wall's own radius — a height iso-line) or a MERIDIAN (a line
///   parallel to the axis — an azimuth iso-line);
/// - the rims sit on exactly two levels.
///
/// That is the rectangle class of the ray lane's
/// ([`super::solid_contain::wall_outline`], asked here rather than
/// restated), and it is what makes the chart trim EXACT: every ruling
/// through the window crosses the boundary once on each level, so the
/// face is exactly the rectangle `[az] × [h]` its boundary pins
/// ([`super::solid_contain::cylinder_chart_trim`]). A third level is a
/// stepped outline the rectangle over-covers.
///
/// A wall closed by a tilted section takes its height extreme inside an
/// edge, so the rectangle would misstate it in BOTH directions; it is
/// the ray lane's CHART outline instead (every boundary edge a meridian,
/// a rim or a planar section, no ring, a window narrower than a period),
/// and this door reads it the way the ray lane does: by parity along the
/// point's ruling ([`super::solid_contain::point_on_wall_in_face`]).
///
/// A face that ALONE wraps the azimuth has no window to trim by, and is
/// served as the full-turn BAND
/// ([`super::solid_contain::full_turn_outline`], the route both doors
/// ask first): membership is the height window alone.
///
/// `None` is therefore the honest remainder throughout — a chart with no
/// arm (NURBS), a chart form the trim cannot express (a ringed face, a
/// boundary outside the rectangle and chart outlines, a wrapped face
/// outside the band class, or a window
/// that reads a whole period on a face that does not wrap alone, whose
/// cosine comparison is an equivalence only under a period), or a
/// margin on a trim boundary — and the caller keeps its typed frontier
/// door there. The period case is decided HERE rather than read out of
/// the solid door's refusal: that door escalates, because a ray lane
/// may not silently skip a wall, and this door's contract is the
/// remainder.
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
///   the face's chart trim excludes it — a sibling face's incidence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CurvedPlacement {
    /// Definitely off the face's carrier surface.
    OffCarrier,
    /// On the carrier (or not tested yet, where the boundary walk
    /// already answered): the trim's verdict, `None` for no verdict.
    Trim(Option<FaceContainment>),
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
    if let Some(v) = curved_boundary_containment(body, face, q, band)? {
        return Ok(CurvedPlacement::Trim(Some(v)));
    }
    let face_data = proven(&body.faces, face, EntityId::Face);
    let surface = body.face_surface_linked(face, face_data);
    // The sphere's region reading takes rings in its stride; the chart
    // trims below do not model them.
    if !face_data.rings.is_empty() && !matches!(surface, geom::Surface::Sphere { .. }) {
        return Ok(CurvedPlacement::Trim(None));
    }
    let (origin, axis, radius, u_ref) = match surface {
        &geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => (origin, axis, radius, u_ref),
        &geom::Surface::Sphere { center, radius, .. } => {
            return sphere_face_containment(body, face, center, radius, q, band);
        }
        &geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            return torus_face_containment(
                body,
                face,
                TorusChart {
                    center,
                    axis,
                    major_radius,
                    minor_radius,
                    u_ref,
                },
                q,
                band,
            );
        }
        &geom::Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            return cone_face_containment(
                body,
                face,
                ConeChart {
                    apex,
                    axis,
                    half_angle,
                    u_ref,
                },
                q,
                band,
            );
        }
        geom::Surface::Plane { .. } | geom::Surface::Nurbs(_) | geom::Surface::Approx(_) => {
            return Ok(CurvedPlacement::Trim(None));
        }
    };
    // ON THE CHART FIRST. The trim below is parameter-domain work and
    // premises an on-wall point (`point_on_wall_in_face` says so in its
    // name): handed a point off the carrier it would answer from the
    // azimuth and height alone and call it `In`. A face is a subset of
    // its carrier, so a point definitely off the carrier is definitely
    // outside the face — decided here, before the trim runs.
    let w = q - origin;
    let radial = w - axis * w.dot(axis);
    match decide(
        "bool_curved_contain_carrier",
        Margin::of(radial.norm() - radius),
        band,
    ) {
        Ok(Sign::Zero) => {}
        Ok(Sign::Positive | Sign::Negative) => return Ok(CurvedPlacement::OffCarrier),
        Err(diag) => return Err(ContainError::on(ContainDecision::Carrier, diag)),
    }
    let (az, h) = match super::solid_contain::cylinder_chart_trim(body, face, origin, axis, band) {
        Ok(t) => t,
        // A window this face cannot express is the honest remainder,
        // not corruption of the caller's query.
        Err(super::solid_contain::PointInSolidError::CorruptFace { .. }) => {
            return Ok(CurvedPlacement::Trim(None));
        }
        Err(e) => return Err(solid_err(e)),
    };
    // The ray lane's class predicates, asked of the same face, the
    // full-turn route first (`full_turn_outline`, the one home both
    // doors call): this door serves the band, the rectangle, and the
    // chart outline a planar section bounds (read by parity along the
    // point's ruling, `point_on_wall_in_face`), never a wall it would
    // have to read as its vertex rectangle.
    let outline =
        match super::solid_contain::full_turn_outline(body, face, origin, axis, radius, h, band)
            .map_err(solid_err)?
        {
            Some(outline @ super::solid_contain::WallOutline::Band { .. }) => outline,
            Some(_) => return Ok(CurvedPlacement::Trim(None)),
            None => {
                // THE cosine-window construction's period guard, third site
                // (`point_on_wall_in_face` carries the argument). A face that
                // does not wrap alone and still reads a whole period has no
                // angular test to run; it is this door's remainder, caught
                // here rather than read out of the rectangle class's
                // escalation.
                match decide(
                    "bool_curved_contain_period",
                    Margin::levered(T::tau() - (az.1 - az.0), radius),
                    band,
                ) {
                    Ok(Sign::Positive) => {}
                    Ok(Sign::Zero | Sign::Negative) => return Ok(CurvedPlacement::Trim(None)),
                    Err(diag) => return Err(ContainError::on(ContainDecision::WindowPeriod, diag)),
                }
                let outline = super::solid_contain::wall_outline(
                    body, face, origin, axis, radius, az, h, band,
                )
                .map_err(solid_err)?;
                if !matches!(
                    outline,
                    super::solid_contain::WallOutline::Rectangle { .. }
                        | super::solid_contain::WallOutline::Chart { .. }
                ) {
                    return Ok(CurvedPlacement::Trim(None));
                }
                outline
            }
        };
    match super::solid_contain::point_on_wall_in_face(
        face, origin, axis, radius, u_ref, az, &outline, q, band,
    ) {
        Ok(Some(true)) => Ok(CurvedPlacement::Trim(Some(FaceContainment::In))),
        Ok(Some(false)) => Ok(CurvedPlacement::Trim(Some(FaceContainment::Out))),
        Ok(None) => Ok(CurvedPlacement::Trim(None)),
        Err(e) => Err(solid_err(e)),
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
    match region.contains(q, band) {
        Ok(Some(true)) => Ok(CurvedPlacement::Trim(Some(FaceContainment::In))),
        Ok(Some(false)) => Ok(CurvedPlacement::Trim(Some(FaceContainment::Out))),
        Ok(None) => Ok(CurvedPlacement::Trim(None)),
        Err(RegionRefusal::Escalated(diag)) => Err(ContainError::Escalated {
            decision: None,
            escalation: Escalation::Margin,
            diag,
        }),
        Err(RegionRefusal::RayExhausted { .. }) => Err(ContainError::RayExhausted),
        Err(e @ RegionRefusal::WoundPastPeriod) => Err(solid_err(e.of_face(face))),
    }
}

/// One torus carrier's chart data, as [`torus_face_containment`] reads it.
struct TorusChart<T: geom_core::Real> {
    center: Point3<T>,
    axis: Vec3<T>,
    major_radius: T,
    minor_radius: T,
    u_ref: Vec3<T>,
}

/// The TORUS chart's arm of [`curved_face_containment`], reached after
/// the shared boundary walk and the ring test.
///
/// The same three steps as the cylinder and sphere arms, in the same
/// order and for the same reasons: the CARRIER first (the point's
/// elevation off the tube, `√((ρ − R)² + h²) − r`, which is exact
/// distance to a ring torus), then the chart trim, then membership in
/// it. The trim and the membership test are the solid door's own
/// ([`super::solid_contain::torus_face_windows`],
/// [`super::solid_contain::point_on_torus_in_face`]), so the face-level
/// and solid-level doors cannot disagree about which chart points a
/// torus face holds.
///
/// **What differs from the solid door is the question.** The solid door
/// asks about a closed group's UNION and lets its representative answer
/// for every member; this door is asked about ONE face, so it reads
/// that face's own windows whatever group it sits in — a donut's two
/// faces each wrap the major azimuth and split the minor angle, and
/// each window answers for its own face alone.
///
/// The remainder, per case:
///
/// - A face whose own windows the walk **cannot pin**, or cannot take at
///   all, is the honest remainder rather than corruption of the
///   caller's query, as in the cylinder arm. `None`.
/// - A coordinate the face ALONE wraps
///   ([`super::surface_group::wrap_rims`], the structural test every
///   chart shares) is NOT a remainder, as it is not on the cylinder
///   arm's full-turn band: there is no window in it to trim by. A window
///   that reads a whole period on a face that does not wrap alone is
///   refused, as is a face wrapping BOTH.
/// - A **graze** on a window edge that the boundary walk did not place
///   ON a vertex or an edge is `None`, as everywhere in this door.
fn torus_face_containment<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    chart: TorusChart<T>,
    q: Point3<T>,
    band: Band,
) -> Result<CurvedPlacement, ContainError> {
    let TorusChart {
        center,
        axis,
        major_radius,
        minor_radius,
        u_ref,
    } = chart;
    let elevation =
        super::solid_contain::torus_elevation(center, axis, major_radius, minor_radius, q);
    match decide("bool_curved_contain_carrier", Margin::of(elevation), band) {
        Ok(Sign::Zero) => {}
        Ok(Sign::Positive | Sign::Negative) => return Ok(CurvedPlacement::OffCarrier),
        Err(diag) => return Err(ContainError::on(ContainDecision::Carrier, diag)),
    }
    let (u_win, v_win) = match super::solid_contain::torus_face_windows(
        body,
        face,
        major_radius,
        minor_radius,
        band,
    ) {
        Ok(w) => w,
        Err(
            super::solid_contain::PointInSolidError::PartialTorusFace { .. }
            | super::solid_contain::PointInSolidError::CorruptFace { .. },
        ) => return Ok(CurvedPlacement::Trim(None)),
        Err(e) => return Err(solid_err(e)),
    };
    match super::solid_contain::point_on_torus_in_face(
        face,
        center,
        axis,
        major_radius,
        minor_radius,
        u_ref,
        u_win,
        v_win,
        q,
        band,
    ) {
        Ok(Some(true)) => Ok(CurvedPlacement::Trim(Some(FaceContainment::In))),
        Ok(Some(false)) => Ok(CurvedPlacement::Trim(Some(FaceContainment::Out))),
        Ok(None) => Ok(CurvedPlacement::Trim(None)),
        Err(e) => Err(solid_err(e)),
    }
}

/// One cone carrier's chart data, as [`cone_face_containment`] reads it.
struct ConeChart<T: geom_core::Real> {
    apex: Point3<T>,
    axis: Vec3<T>,
    half_angle: T,
    u_ref: Vec3<T>,
}

/// The CONE chart's arm of [`curved_face_containment`], reached after
/// the shared boundary walk and the ring test.
///
/// The same three steps as the other arms, in the same order and for
/// the same reasons: the CARRIER first, then the chart trim, then
/// membership in it. The carrier is the whole DOUBLE cone
/// ([`geom_brep::cone_elevation`] with no nappe), because
/// that is the surface the face's carrier states: a point on the
/// mirror nappe is ON the carrier and outside this face's trim — a
/// sibling face's incidence, `Trim(Out)`, never `OffCarrier` — and the
/// slant window's signed bounds are what put it outside. The trim and
/// the membership test are the solid door's own
/// ([`super::solid_contain::cone_face_trim`],
/// [`super::solid_contain::point_on_cone_in_face`]), so the face-level
/// and solid-level doors cannot disagree about which chart points a
/// cone face holds.
///
/// **What differs from the solid door is the question**, as on the
/// torus: this door is asked about ONE face, so a face that shares a
/// wrapped group with siblings reads its own azimuth window rather
/// than the group's.
///
/// The remainder, per case:
///
/// - A face whose own window the walk **cannot pin** — an apex-closed
///   face beside a sibling, whose walk reports the apex junction's wrap
///   as a full period — or cannot take at all, is the honest remainder.
///   `None`.
/// - **The apex** of a face whose slant window reaches it has no
///   tangent plane and no azimuth; the boundary walk has already placed
///   it where it is a vertex, and anywhere else it is `None`.
/// - A **graze** on a window edge that the boundary walk did not place
///   ON a vertex or an edge is `None`, as everywhere in this door.
fn cone_face_containment<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    chart: ConeChart<T>,
    q: Point3<T>,
    band: Band,
) -> Result<CurvedPlacement, ContainError> {
    let ConeChart {
        apex,
        axis,
        half_angle,
        u_ref,
    } = chart;
    let elevation = geom_brep::cone_elevation(apex, axis, half_angle, None, q);
    match decide("bool_curved_contain_carrier", Margin::of(elevation), band) {
        Ok(Sign::Zero) => {}
        Ok(Sign::Positive | Sign::Negative) => return Ok(CurvedPlacement::OffCarrier),
        Err(diag) => return Err(ContainError::on(ContainDecision::Carrier, diag)),
    }
    let (az, v, nappe) =
        match super::solid_contain::cone_face_trim(body, face, apex, axis, half_angle, band) {
            Ok(t) => t,
            Err(
                super::solid_contain::PointInSolidError::PartialConeFace { .. }
                | super::solid_contain::PointInSolidError::CorruptFace { .. },
            ) => return Ok(CurvedPlacement::Trim(None)),
            Err(e) => return Err(solid_err(e)),
        };
    match super::solid_contain::point_on_cone_in_face(
        face, apex, axis, half_angle, u_ref, az, v, nappe, q, band,
    ) {
        Ok(Some(true)) => Ok(CurvedPlacement::Trim(Some(FaceContainment::In))),
        Ok(Some(false)) => Ok(CurvedPlacement::Trim(Some(FaceContainment::Out))),
        Ok(None) => Ok(CurvedPlacement::Trim(None)),
        Err(e) => Err(solid_err(e)),
    }
}

fn solid_err(e: super::solid_contain::PointInSolidError) -> ContainError {
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
/// pre-pass asks the same distance of a circle edge under the same row
/// name, folded the same way
/// ([`crate::splitting::containment::LoopEdge::contact`]).
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
        const NOTE: &str =
            "an unreadable or collapsed margin may indicate a kernel bug worth reporting";
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
        const WALL: &str = "Recourse: keep the wall clearly short of a full turn";
        const UNNAMED: &str = "Recourse: move the point clearly inside or outside the face";
        let loop_ = |d| Some(ContainDecision::Loop(d));
        let tighten = |lever: &str, size: &str| {
            format!("{lever}, or, if this {size} is intended, tighten the tolerance below 5e-10 m")
        };
        use Escalation::{Decided, Margin, Straddle};
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
                Decided,
                poisoned,
                END.to_owned(),
            ),
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
            (
                Some(ContainDecision::WindowPeriod),
                Margin,
                above,
                tighten(WALL, "gap"),
            ),
            (
                Some(ContainDecision::WindowPeriod),
                Margin,
                below,
                WALL.to_owned(),
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
            for reading in [Reading::Build, Reading::AtRest] {
                assert_eq!(
                    placement_ending(decision, escalation, &cause, reading),
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
                    "{} is undecided at this tolerance. {ending}",
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
}
