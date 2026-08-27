//! `contfp` — point-in-face with typed ON verdicts (ch. 13's `contfv`
//! extern-out-parameter idiom as a proper sum type): the second-level
//! case codes of the reduction sweep. The point is assumed on the
//! face's plane (the caller's crossing/on-plane decision precedes).
//!
//! Ladder: an ON verdict fires only at a trilean **Zero** (exact within
//! ε — where declared/structural coincidences land, e.g. a crossing
//! point computed from shared geometry); the sliver band escalates
//! typed (F6); definite margins walk on. Interior/exterior then comes
//! from the PR 3 ray-parity trilean ([`point_in_loop`]) over the outer
//! loop and every ring.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::splitting::{LoopContainment, PointInLoopError, point_in_loop};
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
#[derive(Debug)]
pub enum ContainError {
    /// A margin landed in the sliver band — the pair is
    /// ill-conditioned at this ε.
    Escalated(Indeterminate),
    /// The ray-parity schedule exhausted (every ray grazed).
    RayExhausted,
    /// The face's topology could not be walked.
    Corrupt,
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

    if let Some(on) = boundary_pre_pass(body, &loops, q, EdgeChords::All, band)? {
        return Ok(on);
    }

    // Interior/exterior: inside the outer loop AND outside every ring.
    // WHICH walk reads a loop is the loop's own shape — an all-arc
    // loop bounds a disc and is read by its radius ([`loop_disc`]),
    // everything else by the ray-parity polygon walk.
    let inside = |lk| -> Result<LoopContainment, ContainError> {
        if let Some(disc) = loop_disc(body, lk, band)? {
            return disc_side(disc, q, band);
        }
        Ok(point_in_loop(body, lk, normal, q, band)?)
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

/// The circle a [`loop_disc`] loop bounds — its own type, because
/// three components of one datum read better named than positional.
#[derive(Clone, Copy)]
struct LoopCircle<T> {
    /// The circle's centre.
    center: Point3<T>,
    /// Its plane normal (sign-free: only `cross` reads it).
    axis: Vec3<T>,
    /// Its radius, in metres.
    radius: T,
}

/// The circle a loop bounds when EVERY one of its edges is an arc of
/// one circle — the planar analog of the curved door's iso-bounded
/// class ([`curved_face_containment`]). `None` on any other boundary
/// shape: the honest remainder the caller's ray-parity walk owns.
///
/// **The walk needs this because a polygon cannot express an arc.**
/// [`point_in_loop`]'s contract is the planar POLYGON through a loop's
/// vertices — a cylinder cap's loop has two vertices, so its polygon
/// is the segment between them, the disc's whole interior reads `Out`,
/// and a box driven through a cap unions as two disjoint solids with
/// the overlap counted twice.
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
/// of them `q` lies on. Anything the row does not call `Zero` — an
/// escalation included — is simply not this class.
fn loop_disc<T: Decide>(
    body: &Body<T>,
    r#loop: crate::entity::LoopKey,
    band: Band,
) -> Result<Option<LoopCircle<T>>, ContainError> {
    let mut circle: Option<LoopCircle<T>> = None;
    for (_, he, _) in loop_cycle_points(body, r#loop)? {
        let edge_key = body.get_half_edge(he).ok_or(ContainError::Corrupt)?.edge;
        let carrier = body
            .get_edge(edge_key)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(crate::null::CurveGeom::certified)
            .map(|c| c.carrier().clone());
        let Some(geom::Curve3::Circle {
            center,
            axis,
            radius,
            ..
        }) = carrier
        else {
            return Ok(None);
        };
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
                let d = (center - c0).norm() + (radius - r0).abs() + axis.cross(a0).norm() * r0;
                match decide("bool_face_disc_carrier", Margin::of(d), band) {
                    Ok(Sign::Zero) => {}
                    Ok(Sign::Positive | Sign::Negative) | Err(_) => return Ok(None),
                }
            }
        }
    }
    Ok(circle)
}

/// Which side of a [`loop_disc`] circle `q` lies on. `q` is on the
/// face's plane by [`contfp`]'s contract, so the in-plane radial
/// distance is the whole question.
fn disc_side<T: Decide>(
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
/// The chord rows are gated to [`EdgeChords::LinesAndArcs`]: a `Line`
/// boundary IS its chord, a `Circle` boundary gets its own exact arc
/// rows, and any other carrier gets no verdict rather than a chord's.
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
    boundary_pre_pass(body, &loops, q, EdgeChords::LinesAndArcs, band)
}

/// Which boundary edges the shared pre-pass may decide `OnEdge`
/// through the chord rows. **`Circle` carriers are decided by their
/// own exact arc rows in both modes** — an arc's chord is a different
/// curve, and on a planar face the chord of a rim arc runs through the
/// face INTERIOR, so a chord verdict there is not conservative, it is
/// wrong: it names a boundary edge for a point that is strictly
/// inside. What the two modes still disagree about is the remainder —
/// the carriers with neither an exact row nor a chord that is their
/// own curve.
#[derive(Clone, Copy, PartialEq, Eq)]
enum EdgeChords {
    /// `Line` and every non-circular conic through the chord rows
    /// ([`contfp`]'s posture for the carriers it has always run:
    /// exact for a line boundary, conservative-by-band for the rest).
    All,
    /// `Line` carriers through the chord rows, every carrier without
    /// an exact row undecided. The curved chart's walk: a rim arc is
    /// a boundary a curved face genuinely has, and answering about its
    /// chord would be answering about a different curve.
    LinesAndArcs,
}

/// **The shared boundary pre-pass**: vertex coincidence over ALL
/// loops FIRST — an edge-interior verdict must never shadow a vertex
/// coincidence, across loops exactly as within one (the invariant
/// [`contfp`] always stated; running it per loop let an outer-edge
/// hit shadow a ring vertex, fixed here with its red-then-green row
/// below) — then edge interiors over all loops. Which rows an edge may
/// be decided by is [`EdgeChords`]'s, and it is the ONLY thing the
/// planar and curved walks disagree about. Six rows, one home:
/// `bool_contact_vertex`, `bool_contact_edge_span` (×2),
/// `bool_contact_edge`, and — for the arc disposition —
/// `bool_contact_arc{,_span}`.
fn boundary_pre_pass<T: Decide>(
    body: &Body<T>,
    loops: &[crate::entity::LoopKey],
    q: Point3<T>,
    chords: EdgeChords,
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
            // on a conic answers about the CHORD, not the edge, so a
            // carrier with an exact row takes it and the rest are
            // routed by mode.
            let carrier = body
                .get_edge(edge_key)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(crate::null::CurveGeom::certified)
                .map(|c| (c.carrier().clone(), c.params()));
            match carrier {
                // A `Line` boundary IS its chord: the rows below are
                // exact for it in both modes.
                Some((geom::Curve3::Line { .. }, _)) => {}
                // A `Circle` boundary takes its own exact arc rows in
                // both modes. On a PLANAR face this is not a
                // refinement but a correction: an arc's chord runs
                // through the face interior, so the chord rows report
                // `OnEdge` for points that are strictly INSIDE — a
                // rim arc's chord is the cap's diameter, and every
                // event on it read as a boundary event.
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
                // No exact row: the curved walk gives no verdict
                // rather than a chord's; the planar walk keeps the
                // conservative chord row it has always run.
                _ if chords == EdgeChords::LinesAndArcs => continue,
                _ => {}
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
/// it, `None` when the angular gate lands on an endpoint (the vertex
/// pass owns those) or the arc spans a whole period (no angular
/// window to test).
///
/// Two independent margins, both lengths: the point's exact distance
/// FROM THE CIRCLE (`bool_contact_arc` — radial and axial residuals
/// folded, so one row covers both ways off the carrier), and the
/// angular span (`bool_contact_arc_span`).
///
/// The angular span is THE cosine-window construction, whose argument
/// — period guard, `r̂·m̂ ≥ cos(w/2)`, `· radius` metering, and ledger
/// row F8 — is stated once at
/// [`super::solid_contain::point_on_wall_in_face`] and shared by all
/// three of its sites.
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
    let half = T::from_f64(0.5);
    let width = t1 - t0;
    // The cosine equivalence needs a window under a period; a full
    // circle has no angular gate at all and gets no verdict here.
    match decide(
        "bool_contact_arc_span",
        Margin::levered(T::tau() - width, radius),
        band,
    ) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) => return Ok(None),
        Err(diag) => return Err(ContainError::Escalated(diag)),
    }
    let w = q - center;
    let height = w.dot(axis);
    let radial = w - axis * height;
    let r_norm = radial.norm();
    // Distance from the point to the circle: the radial miss and the
    // axial miss are orthogonal, so their hypotenuse is exact.
    let d = ((r_norm - radius).powi(2) + height.powi(2)).sqrt();
    match decide("bool_contact_arc", Margin::of(d), band) {
        Ok(Sign::Zero) => {}
        Ok(Sign::Positive) => return Ok(Some(false)),
        Ok(Sign::Negative) => {
            return Err(ContainError::Escalated(invalid(band, "bool_contact_arc")));
        }
        Err(diag) => return Err(ContainError::Escalated(diag)),
    }
    // On the carrier: the angular window decides which arc of it.
    let mid = (t0 + t1) * half;
    let (s_m, c_m) = mid.sin_cos();
    let v_ref = axis.cross(u_ref);
    let m_hat = u_ref * c_m + v_ref * s_m;
    let (_, c_h) = (width * half).sin_cos();
    let r_hat = radial / r_norm;
    match decide(
        "bool_contact_arc_span",
        Margin::levered(r_hat.dot(m_hat) - c_h, radius),
        band,
    ) {
        Ok(Sign::Positive) => Ok(Some(true)),
        Ok(Sign::Negative) => Ok(Some(false)),
        // An endpoint neighbourhood: the vertex pass owns it.
        Ok(Sign::Zero) => Ok(None),
        Err(diag) => Err(ContainError::Escalated(diag)),
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
    let Some(&geom::Surface::Cylinder {
        origin,
        axis,
        radius,
        u_ref,
    }) = body.get_surface(face_data.surface)
    else {
        return Ok(None);
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

    /// The disc class is a CLASS, and this is its gate: a loop of
    /// straight edges is not one, whatever its shape, so the walk that
    /// can read it keeps the loop. Without this the radius row would
    /// answer about a circle no boundary edge rides.
    #[test]
    fn a_polygon_loop_is_not_the_disc_class() {
        let holed = crate::fixtures::ops_holed_box(Tol::witness());
        let body = holed.body;
        let band = Band::linear(Tol::witness()).unwrap();
        let mut loops = 0;
        for (_, f) in body.faces() {
            for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
                loops += 1;
                assert!(
                    loop_disc(&body, lk, band).expect("the box walks").is_none(),
                    "a straight-edged loop bounds no disc"
                );
            }
        }
        assert!(loops > 0, "the fixture must have loops to check");
    }
}
