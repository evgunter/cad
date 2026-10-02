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
//! chord, a circle or ellipse arc is crossed on its conic, and an edge
//! on a carrier with no crossing row refuses typed wherever it could
//! matter.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopKey, VertexKey};
use crate::ray_parity::ParityRows;
use crate::splitting::containment::{
    BoundaryRows, CarrierLoop, ConicRows, EdgeContact, carrier_loop, carrier_loop_side,
};
use crate::splitting::{PointInLoopError, Uncrossable};
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
    Escalated(Indeterminate),
    /// The ray-parity schedule exhausted (every ray grazed).
    RayExhausted,
    /// The face's topology could not be walked.
    Corrupt,
    /// The walk could not read a loop at this point: an edge of it it
    /// has no crossing row for stood in the way of every ray.
    Uncrossable(Uncrossable),
}

impl From<PointInLoopError> for ContainError {
    fn from(e: PointInLoopError) -> Self {
        match e {
            PointInLoopError::Escalated { diag, .. } => Self::Escalated(diag),
            PointInLoopError::RayExhausted { .. } => Self::RayExhausted,
            PointInLoopError::CorruptLoop { .. } => Self::Corrupt,
            PointInLoopError::Uncrossable(u) => Self::Uncrossable(u),
        }
    }
}

// Each arm names WHAT STOPPED and the repair that moves it, because a
// consumer that carries this refusal renders it verbatim and adds no
// sentence of its own. The three non-escalated arms want three
// different repairs — re-model the loop, move the point or lower ε,
// repair the body — so one shared tail would name the wrong one for
// two of them.
//
// `Escalated` delegates to [`Indeterminate`]'s own `Display`, which
// already composes the named predicate, the margin it metred and the
// shared two-tolerance recourse; restating any of that here would
// double it.
impl core::fmt::Display for ContainError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Escalated(diag) => write!(f, "contfp: {diag}"),
            Self::RayExhausted => write!(
                f,
                "contfp: every direction of the parity schedule grazed the face's \
                 boundary, so no ray read a definite crossing count — the point sits \
                 within ε of the boundary at this tolerance; move the point off the \
                 boundary or lower the tolerance"
            ),
            Self::Corrupt => write!(
                f,
                "contfp: the face's topology is not a walkable cycle of resolvable \
                 geometry — a loop, half-edge, vertex or point reference does not \
                 resolve; repair the body's topology before asking it a containment \
                 question"
            ),
            Self::Uncrossable(u) => write!(
                f,
                "contfp: {u}. Recourse: model the outline with lines, circles or ellipses"
            ),
        }
    }
}

impl std::error::Error for ContainError {}

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
    let face_data = body.get_face(face).ok_or(ContainError::Corrupt)?;
    let loops: Vec<_> = core::iter::once(face_data.outer)
        .chain(face_data.rings.iter().copied())
        .collect();

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
    let (outer, rings) = read.split_first().ok_or(ContainError::Corrupt)?;
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
        let edge_key = body.get_half_edge(he).ok_or(ContainError::Corrupt)?.edge;
        let carrier = body
            .get_edge(edge_key)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(crate::null::CurveGeom::certified)
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
                        Err(diag) => return Err(ContainError::Escalated(diag)),
                    }
                }
            },
            // A line, a non-circular conic, or null scaffolding (which
            // the operand gate refuses upstream): not one circle's arc.
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
    let face_data = body.get_face(face).ok_or(ContainError::Corrupt)?;
    let loops: Vec<_> = core::iter::once(face_data.outer)
        .chain(face_data.rings.iter().copied())
        .collect();
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
/// ([`crate::ray_parity::on_segment`]), and a circle or an ellipse is asked its
/// own conic and trim — both through
/// [`crate::splitting::containment::LoopEdge::contact`], the one
/// boundary reading the carrier walk runs too, so a point this pass
/// places off an edge is off it for the walk. A spiric or spline edge gets no verdict: its
/// chord is a different curve, and the region walk refuses inside a
/// ball its locus lies in. This is [`contfp`]'s ONE boundary pass: the
/// walk after it trusts it and runs none of its own. Rows, one home:
/// `bool_contact_vertex`, `bool_contact_edge{,_length}`, and — for the
/// conic disposition — `bool_contact_arc{,_span,_end,_trim}`.
fn boundary_pre_pass<T: Decide>(
    body: &Body<T>,
    loops: &[LoopKey],
    q: Point3<T>,
    band: Band,
) -> Result<PrePass<T>, ContainError> {
    for &lk in loops {
        let cycle = loop_cycle_points(body, lk)?;
        for (v, _, p) in &cycle {
            if super::one_vertex(q, *p, band).map_err(ContainError::Escalated)? {
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
                .map_err(ContainError::Escalated)?
            {
                EdgeContact::On => return Ok(PrePass::On(FaceContainment::OnEdge(lp.keys[i]))),
                EdgeContact::Off | EdgeContact::Carrier | EdgeContact::Unread => {}
                // Within the band of a conic's END, which the vertex pass
                // above placed definitely clear of both of this edge's
                // vertices. The end is read exactly (a circle through its
                // radius, an ellipse as a distance from `q`), so the two
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
                    let Some(carrier_ends) = edge.conic_ends() else {
                        unreachable!("only a conic arc reads End")
                    };
                    for c in carrier_ends {
                        for v in [ends.0, ends.1] {
                            if let Err(diag) = decide(END_VERTEX, Margin::norm3(v - c), band) {
                                return Err(ContainError::Escalated(diag));
                            }
                        }
                    }
                    return Err(ContainError::Escalated(crate::invalid_margin::invalid(
                        band, END_VERTEX,
                    )));
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

/// The distance from a conic edge's carrier end to one of the edge's
/// stored vertices — a question about the body, apart from `q`'s own
/// distance to that end (`bool_contact_arc_end`).
const END_VERTEX: &str = "bool_contact_arc_end_vertex";

/// The pre-pass's rows for a straight edge: [`crate::ray_parity::on_segment`]
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
/// stepped outline the rectangle over-covers. A wall closed by a
/// tilted section takes its height extreme inside an edge, the
/// rectangle then misstates the face in BOTH directions, and this door
/// answers `None` rather than a verdict it cannot stand behind.
///
/// A face that ALONE wraps the azimuth has no window to trim by, and is
/// served as the full-turn BAND
/// ([`super::solid_contain::full_turn_outline`], the route both doors
/// ask first): membership is the height window alone.
///
/// `None` is therefore the honest remainder throughout — a chart with no
/// arm (NURBS), a chart form the trim cannot express (a ringed face, a
/// non-iso boundary, a wrapped face outside the band class, or a window
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
    let face_data = body.get_face(face).ok_or(ContainError::Corrupt)?;
    if !face_data.rings.is_empty() {
        return Ok(CurvedPlacement::Trim(None));
    }
    let (origin, axis, radius, u_ref) = match body.get_surface(face_data.surface) {
        Some(&geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        }) => (origin, axis, radius, u_ref),
        Some(&geom::Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        }) => return sphere_face_containment(body, face, center, radius, axis, u_ref, q, band),
        Some(&geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        }) => {
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
        Some(&geom::Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        }) => {
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
        Some(geom::Surface::Plane { .. } | geom::Surface::Nurbs(_) | geom::Surface::Approx(_))
        | None => return Ok(CurvedPlacement::Trim(None)),
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
        Err(diag) => return Err(ContainError::Escalated(diag)),
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
    // doors call): this door serves the band and the rectangle only.
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
                    Err(diag) => return Err(ContainError::Escalated(diag)),
                }
                let outline = super::solid_contain::wall_outline(
                    body, face, origin, axis, radius, az, h, band,
                )
                .map_err(solid_err)?;
                if !matches!(outline, super::solid_contain::WallOutline::Rectangle { .. }) {
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

/// The SPHERE chart's arm of [`curved_face_containment`], reached after
/// the shared boundary walk and the ring test.
///
/// Same three steps as the cylinder arm, in the same order and for the
/// same reasons: the CARRIER first (a face is a subset of its surface,
/// so a point definitely off the sphere is definitely outside the
/// face — and the trim below is parameter-domain work that premises an
/// on-chart point), then the chart rectangle, then membership in it.
///
/// The class test and the rectangle are one call
/// ([`super::solid_contain::sphere_chart_trim`]) rather than the
/// cylinder's two: on a sphere the two questions are the same question.
/// Whether every boundary edge is a rim or a meridian is exactly
/// whether the `[azimuth] × [latitude]` window describes the face, and
/// the invariant that keeps it exact — no pole strictly inside a
/// meridian edge, where latitude stops being monotone — is checked
/// while those edges are being classified. `None` is the honest
/// remainder throughout.
///
/// A FULL-PERIOD azimuth window (a cap, or a latitude band) is served
/// as the ray lane serves it: every azimuth is in the face, so the
/// latitude window alone decides.
#[allow(clippy::too_many_arguments)] // one chart datum, each argument named
fn sphere_face_containment<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
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
        Err(diag) => return Err(ContainError::Escalated(diag)),
    }
    let trim = match super::solid_contain::sphere_chart_trim(body, face, center, radius, axis, band)
    {
        Ok(Some(t)) => t,
        // A face the rectangle cannot express is the honest
        // remainder, not corruption of the caller's query.
        Ok(None) => return Ok(CurvedPlacement::Trim(None)),
        Err(e) => return Err(solid_err(e)),
    };
    match super::solid_contain::point_on_sphere_in_face(
        face, center, radius, axis, u_ref, &trim, q, band,
    ) {
        Ok(Some(true)) => Ok(CurvedPlacement::Trim(Some(FaceContainment::In))),
        Ok(Some(false)) => Ok(CurvedPlacement::Trim(Some(FaceContainment::Out))),
        Ok(None) => Ok(CurvedPlacement::Trim(None)),
        Err(e) => Err(solid_err(e)),
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
        Err(diag) => return Err(ContainError::Escalated(diag)),
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
        Err(diag) => return Err(ContainError::Escalated(diag)),
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
            ContainError::Escalated(diag)
        }
        _ => ContainError::Corrupt,
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
/// is exact and one row covers both ways off the carrier. Its NEGATIVE
/// arm is the reason the body is shared rather than transcribed — a
/// negative distance is impossible, so that arm is not a verdict but a
/// broken invariant, and a copy of the row that folded it in with the
/// definite-positive one would silently answer "off the carrier" where
/// this one escalates.
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
    match decide("bool_contact_arc", Margin::of(d), band) {
        Ok(Sign::Zero) => {
            let w = q - center;
            let radial = w - axis * w.dot(axis);
            Ok(Some((radial, radial.norm())))
        }
        Ok(Sign::Positive) => Ok(None),
        Ok(Sign::Negative) => Err(crate::invalid_margin::invalid(band, "bool_contact_arc")),
        Err(diag) => Err(diag),
    }
}

/// The loop's (start vertex, half-edge, point) cycle. An empty/lone
/// loop yields `Corrupt` (a face boundary must be a cycle here).
#[allow(clippy::type_complexity)]
fn loop_cycle_points<T: Decide>(
    body: &Body<T>,
    lk: crate::entity::LoopKey,
) -> Result<Vec<(VertexKey, crate::entity::HalfEdgeKey, Point3<T>)>, ContainError> {
    let loop_data = body.get_loop(lk).ok_or(ContainError::Corrupt)?;
    let crate::entity::LoopBoundary::Cycle { first } = loop_data.boundary else {
        return Err(ContainError::Corrupt);
    };
    let mut out = Vec::new();
    for he in body.loop_cycle(first).ok_or(ContainError::Corrupt)? {
        let start = body.get_half_edge(he).ok_or(ContainError::Corrupt)?.start;
        let p = *body
            .get_point(body.get_vertex(start).ok_or(ContainError::Corrupt)?.point)
            .ok_or(ContainError::Corrupt)?;
        out.push((start, he, p));
    }
    if out.is_empty() {
        return Err(ContainError::Corrupt);
    }
    Ok(out)
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
