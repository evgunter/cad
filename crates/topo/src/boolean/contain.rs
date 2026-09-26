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
//! edge on its own CARRIER ([`point_in_carrier_loop`]): a line is its
//! chord, a circle or ellipse arc is crossed on its conic, and an edge
//! on a carrier with no crossing row refuses typed wherever it could
//! matter.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::splitting::containment::{ArcTrim, ArcTrimRows, arc_trim, point_in_carrier_loop};
use crate::splitting::{LoopContainment, PointInLoopError};
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
    /// A **loop no available walk expresses at this point**: it has an
    /// edge on a carrier the in-plane walk has no crossing row for (a
    /// spiric, a spline), and the point lies within reach of that edge,
    /// where a crossing could change the answer. A point definitely
    /// clear of the loop's reach is answered `Out` and never lands here.
    ArcLoopUnsupported {
        /// The loop whose region no available walk expresses.
        r#loop: crate::entity::LoopKey,
    },
}

impl From<PointInLoopError> for ContainError {
    fn from(e: PointInLoopError) -> Self {
        match e {
            PointInLoopError::Escalated { diag, .. } => Self::Escalated(diag),
            PointInLoopError::RayExhausted { .. } => Self::RayExhausted,
            PointInLoopError::CorruptLoop { .. } => Self::Corrupt,
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
            Self::ArcLoopUnsupported { r#loop } => write!(
                f,
                "contfp: loop {loop:?} has an edge on a spiric or spline carrier, which \
                 the in-plane walk cannot cross, and the point lies within that edge's \
                 reach, so no available walk expresses the region there — refused \
                 rather than answered; model the boundary with lines, circles or \
                 ellipses"
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

    if let Some(on) = boundary_pre_pass(body, &loops, q, band)? {
        return Ok(on);
    }

    // Interior/exterior: inside the outer loop AND outside every ring,
    // each read on its edges' own carriers. The walk's `None` is an
    // edge it cannot cross within reach of `q`: a refusal, typed.
    let inside = |lk| -> Result<LoopContainment, ContainError> {
        point_in_carrier_loop(body, lk, normal, q, band)?
            .ok_or(ContainError::ArcLoopUnsupported { r#loop: lk })
    };
    match inside(face_data.outer)? {
        LoopContainment::Out => return Ok(FaceContainment::Out),
        LoopContainment::In => {}
        LoopContainment::OnBoundary => {
            return Err(ContainError::Escalated(invalid(
                band,
                "bool_contfp_boundary",
            )));
        }
    }
    for &ring in &face_data.rings {
        match inside(ring)? {
            LoopContainment::Out => {}
            LoopContainment::In => return Ok(FaceContainment::Out),
            LoopContainment::OnBoundary => {
                return Err(ContainError::Escalated(invalid(
                    band,
                    "bool_contfp_boundary",
                )));
            }
        }
    }
    Ok(FaceContainment::In)
}

/// Which exact instrument expresses a loop's region — the
/// classification tier 3's check 9 gates its nesting arm on. [`contfp`]
/// does not ask it: the carrier walk reads every class below.
pub(crate) enum LoopShape<T: geom_core::Real> {
    /// Every edge is an arc of ONE circle: the region is that circle's
    /// disc and [`disc_side`] is exact on it.
    Disc(LoopCircle<T>),
    /// No arc anywhere: the ray-parity walk's polygon IS this loop's
    /// region, exactly.
    Polygon,
    /// Arc-bearing over at least three vertices: the polygon through
    /// them is a proper region, but it is NOT this loop's region, and
    /// saying so is this variant's whole job. An arc bowing OUTWARD
    /// leaves region between the polygon and the boundary, and a point
    /// there reads `Out` from the polygon when it is in: measured on a
    /// bored D-rod's transverse cap, whose major arc dips past the chord
    /// its vertices span and whose bore sits in the lune between them.
    /// Check 9 gates its nesting arm on [`Self::Polygon`] and
    /// [`Self::Disc`] alone for exactly that reason.
    ArcParity,
    /// Arc-bearing over fewer than three vertices: the polygon through
    /// them is a segment of ZERO AREA, so a polygon walk would answer
    /// `Out` for every interior point — a half-disc cap, a
    /// half-cylinder cap, a lens cap (two arcs of two DIFFERENT
    /// circles, no line edge at all).
    NoWalk,
}

/// The circle a disc-class loop bounds — its own type, because three
/// components of one datum read better named than positional.
///
/// Its centre and radius are readable crate-wide: tier 3's check 9
/// decides two disc-class loops against each other from them (the
/// centre distance against the radii's sum and difference), a question
/// about a PAIR of loops that [`disc_side`]'s one point cannot ask.
#[derive(Clone, Copy)]
pub(crate) struct LoopCircle<T: geom_core::Real> {
    /// The circle's centre.
    pub(crate) center: Point3<T>,
    /// Its plane normal (sign-free: only `cross` reads it).
    axis: Vec3<T>,
    /// Its radius, in metres.
    pub(crate) radius: T,
}

/// **Which exact instrument expresses this loop's region** — one
/// carrier pass over the cycle, classifying the loop rather than the
/// point.
///
/// [`crate::splitting::point_in_loop`]'s contract is the planar POLYGON
/// through a loop's vertices, so a loop with an arc in it is outside
/// its domain. Four outcomes:
///
/// - **[`LoopShape::Disc`]** — every edge is an arc of one circle. The
///   region is that circle's disc exactly; [`disc_side`] decides it.
///   The planar analog of the curved door's iso-bounded class
///   ([`curved_face_containment`]).
/// - **[`LoopShape::Polygon`]** — no arc at all: the polygon IS the
///   region.
/// - **[`LoopShape::ArcParity`]** — arcs over ≥ 3 vertices, where the
///   polygon is a proper region but not the loop's: an arc bowing
///   outward puts region between the polygon and the boundary.
/// - **[`LoopShape::NoWalk`]** — arc-bearing over < 3 vertices, where
///   the polygon has zero area.
///
/// The circle is read from the first edge; every later edge must agree
/// with it on one metre-valued row folding centre offset, radius
/// difference and axis tilt (levered at the radius). The axis enters
/// only through `cross`, which is blind to its sign — two arcs of one
/// circle may run in opposite senses and are still arcs of the same
/// point set, which is all this asks.
///
/// **That row decides nothing about `q`.** A point reaching here is
/// definitely off every boundary arc by more than the band (the
/// boundary pre-pass owns the near-boundary case), so arcs whose
/// circles agree to within the band cannot disagree about which side
/// of them `q` lies on. A definite disagreement is simply not this
/// class; an ESCALATION escalates, exactly as every row of
/// [`curved_face_containment`] does — an in-band margin is not a
/// licence to fall through to a walk whose domain this loop is
/// outside.
///
/// # Errors
///
/// [`ContainError`] — a carrier-agreement escalation, or a loop this
/// walk cannot read.
pub(crate) fn loop_shape<T: Decide>(
    body: &Body<T>,
    r#loop: crate::entity::LoopKey,
    band: Band,
) -> Result<LoopShape<T>, ContainError> {
    let cycle = loop_cycle_points(body, r#loop)?;
    let vertices = cycle.len();
    // The WHOLE cycle is walked before anything is decided: a loop's
    // first edge says nothing about its last, and a half-disc whose
    // chord comes first would otherwise pass for a polygon.
    let mut circle: Option<LoopCircle<T>> = None;
    let mut one_circle = true;
    let mut bears_arc = false;
    for (_, he, _) in cycle {
        let edge_key = body.get_half_edge(he).ok_or(ContainError::Corrupt)?.edge;
        let carrier = body
            .get_edge(edge_key)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(crate::null::CurveGeom::certified)
            .map(|c| c.carrier().clone());
        match carrier {
            // A line edge IS its chord: it costs the polygon nothing
            // and the disc class everything.
            Some(geom::Curve3::Line { .. }) => one_circle = false,
            Some(geom::Curve3::Circle {
                center,
                axis,
                radius,
                ..
            }) => {
                bears_arc = true;
                match circle {
                    None => {
                        circle = Some(LoopCircle {
                            center,
                            axis,
                            radius,
                        });
                    }
                    Some(c) => {
                        let (c0, a0, r0) = (c.center, c.axis, c.radius);
                        let d =
                            (center - c0).norm() + (radius - r0).abs() + axis.cross(a0).norm() * r0;
                        match decide("bool_face_disc_carrier", Margin::of(d), band) {
                            Ok(Sign::Zero) => {}
                            Ok(Sign::Positive | Sign::Negative) => one_circle = false,
                            Err(diag) => return Err(ContainError::Escalated(diag)),
                        }
                    }
                }
            }
            // A non-circular conic is an arc for the polygon's
            // purposes — its chord is not its locus either, and
            // elliptical caps are real (a tilted cut through a
            // cylinder). It has no exact side row here, so it is never
            // the disc class; the count decides whether the polygon
            // may stand in for it.
            Some(
                geom::Curve3::Ellipse { .. } | geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_),
            ) => {
                bears_arc = true;
                one_circle = false;
            }
            // Null scaffolding: the operand gate refuses it upstream,
            // and its chord is a segment like any other. Not the disc
            // class, and not counted as an arc — this walk does not
            // invent a refusal for a state it never sees.
            None => one_circle = false,
        }
    }
    Ok(match (one_circle, circle) {
        (true, Some(c)) => LoopShape::Disc(c),
        _ if bears_arc && vertices < 3 => LoopShape::NoWalk,
        _ if bears_arc => LoopShape::ArcParity,
        _ => LoopShape::Polygon,
    })
}

/// Which side of a [`loop_shape`] circle `q` lies on. `q` lies in the
/// circle's plane, so the in-plane radial distance is the whole
/// question.
///
/// Exact on the class: one radial margin through one decide
/// (`bool_face_disc_radius`) — `OnBoundary` on its `Zero`, an
/// escalation on an in-band margin, never a guess. That `q` is off the
/// circle by more than the band is the caller's to supply if it wants
/// a definite answer.
pub(crate) fn disc_side<T: Decide>(
    disc: LoopCircle<T>,
    q: Point3<T>,
    band: Band,
) -> Result<LoopContainment, ContainError> {
    let w = q - disc.center;
    let radial = w - disc.axis * w.dot(disc.axis);
    match decide(
        "bool_face_disc_radius",
        Margin::of(disc.radius - radial.norm()),
        band,
    ) {
        Ok(Sign::Positive) => Ok(LoopContainment::In),
        Ok(Sign::Negative) => Ok(LoopContainment::Out),
        Ok(Sign::Zero) => Ok(LoopContainment::OnBoundary),
        Err(diag) => Err(ContainError::Escalated(diag)),
    }
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
    boundary_pre_pass(body, &loops, q, band)
}

/// **The shared boundary pre-pass**: vertex coincidence over ALL
/// loops FIRST — an edge-interior verdict must never shadow a vertex
/// coincidence, across loops exactly as within one (the invariant
/// [`contfp`] always stated; running it per loop let an outer-edge
/// hit shadow a ring vertex, fixed here with its red-then-green row
/// below) — then edge interiors over all loops, each edge on the row
/// its carrier has: a `Line` is its chord, and a `Circle` is asked its
/// own carrier and trim ([`point_on_arc`]). Every other carrier gets no
/// verdict here, because its chord is a different curve — on a planar
/// face an elliptical rim's chord runs through the face INTERIOR — and
/// a point on one reaches the caller's region walk, which reads the
/// edge on its own carrier. Rows, one home: `bool_contact_vertex`,
/// `bool_contact_edge_span` (×2), `bool_contact_edge`, and — for the
/// arc disposition — `bool_contact_arc{,_span,_end,_trim}`.
fn boundary_pre_pass<T: Decide>(
    body: &Body<T>,
    loops: &[crate::entity::LoopKey],
    q: Point3<T>,
    band: Band,
) -> Result<Option<FaceContainment>, ContainError> {
    for &lk in loops {
        let cycle = loop_cycle_points(body, lk)?;
        for (v, _, p) in &cycle {
            let margin = Margin::norm3(q - *p);
            match decide("bool_contact_vertex", margin, band) {
                Ok(Sign::Zero) => return Ok(Some(FaceContainment::OnVertex(*v))),
                Ok(Sign::Positive) => {}
                Ok(Sign::Negative) => {
                    return Err(ContainError::Escalated(invalid(
                        band,
                        "bool_contact_vertex",
                    )));
                }
                Err(diag) => return Err(ContainError::Escalated(diag)),
            }
        }
    }
    for &lk in loops {
        let cycle = loop_cycle_points(body, lk)?;
        for (i, (_, he, a)) in cycle.iter().enumerate() {
            let edge_key = body.get_half_edge(*he).ok_or(ContainError::Corrupt)?.edge;
            // Which rows may decide this edge, by carrier. A chord row
            // on a conic answers about the CHORD, not the edge, so only
            // a line takes it.
            let carrier = body
                .get_edge(edge_key)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(crate::null::CurveGeom::certified)
                .map(|c| (c.carrier().clone(), c.params()));
            match carrier {
                // A `Line` boundary IS its chord: the rows below are
                // exact for it.
                Some((geom::Curve3::Line { .. }, _)) => {}
                // A `Circle` boundary takes its own exact arc rows: an
                // arc's chord runs through a planar face's interior,
                // and a rim arc's chord is a cap's diameter.
                Some((
                    geom::Curve3::Circle {
                        center,
                        axis,
                        radius,
                        u_ref,
                    },
                    (t0, t1),
                )) => {
                    if point_on_arc(q, center, axis, radius, u_ref, t0, t1, band)? == Some(true) {
                        return Ok(Some(FaceContainment::OnEdge(edge_key)));
                    }
                    continue;
                }
                // Null scaffolding is read as its chord, as the region
                // walk reads it.
                None => {}
                // No exact row: no verdict rather than a chord's.
                Some(_) => continue,
            }
            let b = cycle[(i + 1) % cycle.len()].2;
            let e = b - *a;
            let len = e.norm();
            let ehat = e.normalize();
            // Span gates: q's projection strictly interior to [a, b]
            // (endpoint neighborhoods already decided above).
            let s0 = (q - *a).dot(ehat);
            let s1 = len - s0;
            let interior = matches!(
                decide("bool_contact_edge_span", Margin::of(s0), band),
                Ok(Sign::Positive)
            ) && matches!(
                decide("bool_contact_edge_span", Margin::of(s1), band),
                Ok(Sign::Positive)
            );
            if !interior {
                continue;
            }
            let perp = Margin::norm3((q - *a).cross(ehat));
            match decide("bool_contact_edge", perp, band) {
                Ok(Sign::Zero) => return Ok(Some(FaceContainment::OnEdge(edge_key))),
                Ok(Sign::Positive) => {}
                Ok(Sign::Negative) => {
                    return Err(ContainError::Escalated(invalid(band, "bool_contact_edge")));
                }
                Err(diag) => return Err(ContainError::Escalated(diag)),
            }
        }
    }
    Ok(None)
}

/// Is `q` on the INTERIOR of the arc `(center, axis, radius, u_ref)`
/// over `[t0, t1]`? `Some(true)` on it, `Some(false)` definitely off
/// it, `None` when `q` lies within the band of an end (the vertex pass
/// owns those) or the arc spans a whole period (a closed edge, with no
/// interior this pass names).
///
/// Every margin is a length. The point's exact distance FROM THE
/// CIRCLE (`bool_contact_arc` — radial and axial residuals folded, so
/// one row covers both ways off the carrier), after the period guard
/// (`bool_contact_arc_span`); then the trim, as
/// distances in [`arc_trim`] (`bool_contact_arc_end`, the distance to
/// either end, and `bool_contact_arc_trim`, which side of the ends).
/// Neither of the trim rows is compressed near an end, so an end's
/// neighbourhood is the band's own width along the carrier on a short
/// arc and a near-full one alike.
#[allow(clippy::too_many_arguments)] // one arc datum, each argument named
pub(super) fn point_on_arc<T: Decide>(
    q: Point3<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
    t0: T,
    t1: T,
    band: Band,
) -> Result<Option<bool>, ContainError> {
    const ROWS: ArcTrimRows = ArcTrimRows {
        end: "bool_contact_arc_end",
        trim: "bool_contact_arc_trim",
    };
    match decide(
        "bool_contact_arc_span",
        Margin::levered(T::tau() - (t1 - t0), radius),
        band,
    ) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) => return Ok(None),
        Err(diag) => return Err(ContainError::Escalated(diag)),
    }
    let Some((radial, r_norm)) =
        point_on_circle(q, center, axis, radius, band).map_err(ContainError::Escalated)?
    else {
        return Ok(Some(false));
    };
    // On the carrier: the trim decides which arc of it, in the circle's
    // own frame, where it is the unit circle.
    let v_ref = axis.cross(u_ref);
    let dir = (radial.dot(u_ref) / r_norm, radial.dot(v_ref) / r_norm);
    match arc_trim(dir, (t0, t1), radius, ROWS, band).map_err(ContainError::Escalated)? {
        ArcTrim::On => Ok(Some(true)),
        ArcTrim::Off => Ok(Some(false)),
        ArcTrim::End => Ok(None),
    }
}

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
/// Only then does this ask the interior question, and only a
/// **cylinder wall of the ISO-BOUNDED class** can answer it:
///
/// - the face carries no rings (a ring is a hole the rectangle below
///   does not model, and answering `In` inside one would be wrong);
/// - every boundary edge is a RIM (a circle coaxial with the wall, at
///   the wall's own radius — a height iso-line) or a MERIDIAN (a line
///   parallel to the axis — an azimuth iso-line).
///
/// That class is what makes the chart trim EXACT: both chart
/// coordinates are monotone along every boundary edge, so the face is
/// exactly the rectangle `[az] × [h]` its boundary pins
/// ([`super::solid_contain::cylinder_chart_trim`]). A wall closed by a
/// tilted section takes its height extreme inside an edge, the
/// rectangle then misstates the face in BOTH directions, and this door
/// answers `None` rather than a verdict it cannot stand behind.
///
/// `None` is therefore the honest remainder throughout — a non-cylinder
/// chart, a chart form the trim cannot express (a ringed face, a
/// non-iso boundary, or a FULL-PERIOD azimuth window, whose cosine
/// comparison is an equivalence only under a period), or a margin on a
/// trim boundary — and the caller keeps its typed frontier door there.
/// The period case is decided HERE rather than read out of the solid
/// door's refusal: that door escalates, because a ray lane may not
/// silently skip a wall, and this door's contract is the remainder.
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
    if let Some(v) = curved_boundary_containment(body, face, q, band)? {
        return Ok(Some(v));
    }
    let face_data = body.get_face(face).ok_or(ContainError::Corrupt)?;
    if !face_data.rings.is_empty() {
        return Ok(None);
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
        _ => return Ok(None),
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
        Ok(Sign::Positive | Sign::Negative) => return Ok(Some(FaceContainment::Out)),
        Err(diag) => return Err(ContainError::Escalated(diag)),
    }
    if !iso_bounded_wall(body, face, origin, axis, radius, band)? {
        return Ok(None);
    }
    let (az, h) = match super::solid_contain::cylinder_chart_trim(body, face, origin, axis, band) {
        Ok(t) => t,
        // A window this face cannot express is the honest remainder,
        // not corruption of the caller's query.
        Err(super::solid_contain::PointInSolidError::CorruptFace { .. }) => return Ok(None),
        Err(e) => return Err(solid_err(e)),
    };
    // THE cosine-window construction's period guard, third site
    // (`point_on_wall_in_face` carries the argument).
    // A FULL-PERIOD azimuth window is a chart form this door cannot
    // express, not an ill-conditioned one: the trim's cosine
    // comparison is an equivalence only for a window narrower than a
    // period, so at a full turn there is no angular test to run and the
    // rectangle stops describing the face. The solid door escalates
    // here because its ray lane must not silently skip a wall; this
    // door's contract is the honest remainder, so the case is caught
    // BEFORE the trim rather than read out of its refusal.
    match decide(
        "bool_curved_contain_period",
        Margin::levered(T::tau() - (az.1 - az.0), radius),
        band,
    ) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) => return Ok(None),
        Err(diag) => return Err(ContainError::Escalated(diag)),
    }
    match super::solid_contain::point_on_wall_in_face(
        face, origin, axis, radius, u_ref, az, h, q, band,
    ) {
        Ok(Some(true)) => Ok(Some(FaceContainment::In)),
        Ok(Some(false)) => Ok(Some(FaceContainment::Out)),
        Ok(None) => Ok(None),
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
/// **The one place this door is stricter than the ray lane**: a
/// FULL-PERIOD azimuth window answers `None` here. The ray lane serves
/// it (every azimuth is in the face, so the window cannot exclude a
/// point), but this door's contract is the remainder and its caller
/// keeps a typed frontier there — the same posture the cylinder arm's
/// period guard takes, kept rather than widened in passing.
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
) -> Result<Option<FaceContainment>, ContainError> {
    match decide(
        "bool_curved_contain_carrier",
        Margin::of((q - center).norm() - radius),
        band,
    ) {
        Ok(Sign::Zero) => {}
        Ok(Sign::Positive | Sign::Negative) => return Ok(Some(FaceContainment::Out)),
        Err(diag) => return Err(ContainError::Escalated(diag)),
    }
    let trim = match super::solid_contain::sphere_chart_trim(body, face, center, radius, axis, band)
    {
        Ok(Some(t)) => t,
        // A face the rectangle cannot express is the honest
        // remainder, not corruption of the caller's query.
        Ok(None) => return Ok(None),
        Err(e) => return Err(solid_err(e)),
    };
    if trim.az.is_none() {
        return Ok(None);
    }
    match super::solid_contain::point_on_sphere_in_face(
        face, center, radius, axis, u_ref, &trim, q, band,
    ) {
        Ok(Some(true)) => Ok(Some(FaceContainment::In)),
        Ok(Some(false)) => Ok(Some(FaceContainment::Out)),
        Ok(None) => Ok(None),
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

/// Is every boundary edge of `face` a chart ISO-LINE of the wall — a
/// rim (coaxial circle at the wall's radius) or a meridian (line
/// parallel to the axis)? A definite non-iso edge answers `false`; an
/// in-band one escalates (the two-tolerance pair).
///
/// **Dimension.** Every margin here is a LENGTH in metres, and the two
/// kinds of quantity reach that convention differently, so they get
/// different constructors rather than one:
///
/// - a **direction disagreement** is `|â × b̂|` of two UNIT vectors —
///   dimensionless, the sine of the angle between them. Its physical
///   size is the displacement it causes at the chart's own scale, so it
///   is `Margin::levered` by the radius: that is precisely the
///   dimensionless-times-lever-arm contract.
/// - a **length disagreement** — a radius difference, an off-axis
///   offset — is ALREADY metres, so it takes `Margin::of`. Levering it
///   would multiply metres by metres and make the tolerance scale with
///   the radius: a rim would be judged coaxial on a loose scale below
///   `r = 1` and a tight one above it, which is the very drift the
///   dimension convention exists to prevent.
fn iso_bounded_wall<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    band: Band,
) -> Result<bool, ContainError> {
    let face_data = body.get_face(face).ok_or(ContainError::Corrupt)?;
    let crate::entity::LoopBoundary::Cycle { first } = body
        .get_loop(face_data.outer)
        .ok_or(ContainError::Corrupt)?
        .boundary
    else {
        return Ok(false);
    };
    let zero = |name: &'static str, m: Margin<T>| -> Result<bool, ContainError> {
        match decide(name, m, band) {
            Ok(Sign::Zero) => Ok(true),
            Ok(Sign::Positive | Sign::Negative) => Ok(false),
            Err(diag) => Err(ContainError::Escalated(diag)),
        }
    };
    // A unit-vector cross product is a SINE (dimensionless); a radius
    // or offset difference is already metres. See the header.
    let sine = |m: T| Margin::levered(m, radius);
    for he in body.loop_cycle(first).ok_or(ContainError::Corrupt)? {
        let edge = body.get_half_edge(he).ok_or(ContainError::Corrupt)?.edge;
        let carrier = body
            .get_edge(edge)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(crate::null::CurveGeom::certified)
            .map(|c| c.carrier().clone());
        match carrier {
            Some(geom::Curve3::Line { dir, .. }) => {
                if !zero("bool_wall_iso_meridian", sine(dir.cross(axis).norm()))? {
                    return Ok(false);
                }
            }
            Some(geom::Curve3::Circle {
                center,
                axis: c_axis,
                radius: c_radius,
                ..
            }) => {
                let e = center - origin;
                let off_axis = (e - axis * e.dot(axis)).norm();
                if !zero("bool_wall_iso_rim", sine(c_axis.cross(axis).norm()))?
                    || !zero("bool_wall_iso_rim", Margin::of(c_radius - radius))?
                    || !zero("bool_wall_iso_rim", Margin::of(off_axis))?
                {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        }
    }
    Ok(true)
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
/// Two callers, two different verdicts from the same three arms:
/// [`point_on_arc`] turns a definite miss into `Some(false)`, and the
/// boolean reduction's point split turns it into a broken-invariant
/// refusal, because a split point was placed on that carrier by an
/// exact row upstream.
pub(super) fn point_on_circle<T: Decide>(
    q: Point3<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    band: Band,
) -> Result<Option<(Vec3<T>, T)>, Indeterminate> {
    let w = q - center;
    let height = w.dot(axis);
    let radial = w - axis * height;
    let r_norm = radial.norm();
    let d = ((r_norm - radius).powi(2) + height.powi(2)).sqrt();
    match decide("bool_contact_arc", Margin::of(d), band) {
        Ok(Sign::Zero) => Ok(Some((radial, r_norm))),
        Ok(Sign::Positive) => Ok(None),
        Ok(Sign::Negative) => Err(invalid(band, "bool_contact_arc")),
        Err(diag) => Err(diag),
    }
}

fn invalid(band: Band, predicate: &'static str) -> Indeterminate {
    Indeterminate {
        margin: geom_core::MarginDiag::Invalid,
        band,
        predicate: Some(predicate),
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
                let top = body.loop_cycle(first)?.into_iter().all(|he| {
                    body.half_edges
                        .get(he)
                        .and_then(|h| body.vertices.get(h.start))
                        .and_then(|v| body.points.get(v.point))
                        .is_some_and(|p| p.z == 1.0)
                });
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

    /// An arc's end neighbourhood is the band's own width along the
    /// carrier, however short or long the arc: on a `w = 0.02` arc of
    /// radius 10, and on its complement, a point `s` along the carrier
    /// from an end is on or off the arc for every `s` past the band —
    /// including the `ε < s < 10ε / sin(w/2)` stretch a cosine window
    /// compresses into its zero or escalation zone — and only within
    /// the band is it the vertex pass's.
    #[test]
    fn an_arcs_end_zone_is_the_bands_own_width() {
        let band = Band::linear(Tol::witness()).unwrap();
        let eps = band.zero();
        let (r, c) = (10.0, geom_core::Point3::new(0.0, 0.0, 0.0));
        let (axis, u) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let at = |theta: f64| geom_core::Point3::new(r * theta.cos(), r * theta.sin(), 0.0);
        for (t0, t1) in [(0.0, 0.02), (0.0, core::f64::consts::TAU - 0.02)] {
            let ask = |q| point_on_arc(q, c, axis, r, u, t0, t1, band).expect("decided");
            for s in [20.0 * eps, 50.0 * eps, 500.0 * eps, 5000.0 * eps] {
                let d = s / r;
                assert_eq!(
                    ask(at(t1 - d)),
                    Some(true),
                    "w = {t1}: {s} m inside the end"
                );
                assert_eq!(ask(at(t1 + d)), Some(false), "w = {t1}: {s} m past the end");
                assert_eq!(
                    ask(at(t0 + d)),
                    Some(true),
                    "w = {t1}: {s} m inside the start"
                );
                assert_eq!(
                    ask(at(t0 - d)),
                    Some(false),
                    "w = {t1}: {s} m before the start"
                );
            }
            assert_eq!(
                ask(at(t1 + 0.5 * eps / r)),
                None,
                "w = {t1}: within the band of the end"
            );
        }
    }

    /// The disc class is a CLASS, and this is its gate: a loop of
    /// straight edges is not one, whatever its shape, so the walk that
    /// can read it keeps the loop. Without this the radius row would
    /// answer about a circle no boundary edge rides — and the `NoWalk`
    /// gate must not fire on it either, however few vertices it has.
    #[test]
    fn a_polygon_loop_is_neither_the_disc_class_nor_gated() {
        let holed = crate::fixtures::ops_holed_box(Tol::witness());
        let body = holed.body;
        let band = Band::linear(Tol::witness()).unwrap();
        let mut loops = 0;
        for (_, f) in body.faces() {
            for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
                loops += 1;
                assert!(
                    matches!(
                        loop_shape(&body, lk, band).expect("the box walks"),
                        LoopShape::Polygon
                    ),
                    "a straight-edged loop bounds no disc and needs no gate"
                );
            }
        }
        assert!(loops > 0, "the fixture must have loops to check");
    }
}
