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
/// deterministic: outer loop then rings, each in cycle order; the
/// boundary pre-pass (vertices, then edge interiors) decides every ON
/// case before ray parity runs.
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

    // Boundary pre-pass, all loops: vertex coincidence first (so an
    // edge-interior verdict can never shadow an endpoint coincidence),
    // then edge interiors.
    for &lk in &loops {
        let cycle = loop_cycle_points(body, lk)?;
        for (v, _, p) in &cycle {
            let margin = Margin::norm3(q - *p);
            match decide("bool_contact_vertex", margin, band) {
                Ok(Sign::Zero) => return Ok(FaceContainment::OnVertex(*v)),
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
        for (i, (_, he, a)) in cycle.iter().enumerate() {
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
                Ok(Sign::Zero) => {
                    let edge = body.get_half_edge(*he).ok_or(ContainError::Corrupt)?.edge;
                    return Ok(FaceContainment::OnEdge(edge));
                }
                Ok(Sign::Positive) => {}
                Ok(Sign::Negative) => {
                    return Err(ContainError::Escalated(invalid(band, "bool_contact_edge")));
                }
                Err(diag) => return Err(ContainError::Escalated(diag)),
            }
        }
    }

    // Interior/exterior: inside the outer loop AND outside every ring.
    match point_in_loop(body, face_data.outer, normal, q, band)? {
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
        match point_in_loop(body, ring, normal, q, band)? {
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

/// **Boundary containment** for an on-carrier point against a CURVED
/// face: [`contfp`]'s boundary pre-pass, and nothing after it — vertex
/// coincidence first (`bool_contact_vertex`, a 3D distance,
/// carrier-agnostic), then boundary-edge interiors.
///
/// Two carrier arms decide an edge interior, each on its own exact
/// geometry rather than on a chord:
///
/// - **`Line`**: `bool_contact_edge{,_span}` — a line's chord IS the
///   edge, so the span gates and the perpendicular distance decide
///   exactly.
/// - **`Circle`**: `bool_contact_arc{,_span}` — the exact distance
///   from the point to the CIRCLE (radial and axial residuals folded
///   into one length) says whether it is on the carrier, and a
///   branch-cut-free angular span gate says whether it is on the
///   ARC. The span test is the cosine comparison `r̂·m̂ ≥ cos(w/2)`
///   about the arc's mid-direction, metered at the arc's radius: no
///   `atan2`, no periodic reduction, so an `Interval` enclosure near
///   the parameter seam stays honest rather than poisoning
///   (`point_on_wall_in_face`'s MAJOR-1 argument, same shape).
///
/// Every other carrier gets no verdict here. `None` is the honest
/// remainder: this walk answers about the BOUNDARY only, and the
/// interior/exterior question belongs to
/// [`curved_face_containment`].
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
    // Vertex pass over ALL loops first, exactly as `contfp`: an
    // edge-interior verdict must never shadow an endpoint coincidence.
    for &lk in &loops {
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
    for &lk in &loops {
        let cycle = loop_cycle_points(body, lk)?;
        for (i, (_, he, a)) in cycle.iter().enumerate() {
            let edge_key = body.get_half_edge(*he).ok_or(ContainError::Corrupt)?.edge;
            let curve = body
                .get_edge(edge_key)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(crate::null::CurveGeom::certified)
                .cloned();
            match curve.as_ref().map(|c| c.carrier()) {
                Some(geom::Curve3::Line { .. }) => {}
                Some(&geom::Curve3::Circle {
                    center,
                    axis,
                    radius,
                    u_ref,
                }) => {
                    let (t0, t1) = curve
                        .as_ref()
                        .map_or((T::zero(), T::zero()), |c| c.params());
                    if let Some(on) = point_on_arc(q, center, axis, radius, u_ref, t0, t1, band)?
                        && on
                    {
                        return Ok(Some(FaceContainment::OnEdge(edge_key)));
                    }
                    continue;
                }
                _ => continue,
            }
            let b = cycle[(i + 1) % cycle.len()].2;
            let e = b - *a;
            let len = e.norm();
            let ehat = e.normalize();
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
/// angular span (`bool_contact_arc_span`, the cosine comparison
/// metered at the radius).
#[allow(clippy::too_many_arguments)] // one arc datum, each argument named
fn point_on_arc<T: Decide>(
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
/// chart, a chart form the trim cannot express, or a margin on a trim
/// boundary — and the caller keeps its typed frontier door there.
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
/// parallel to the axis)? Both margins are metered at the radius, the
/// chart convention. A definite non-iso edge answers `false`; an
/// in-band one escalates (the two-tolerance pair).
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
    let zero = |name: &'static str, m: T| -> Result<bool, ContainError> {
        match decide(name, Margin::levered(m, radius), band) {
            Ok(Sign::Zero) => Ok(true),
            Ok(Sign::Positive | Sign::Negative) => Ok(false),
            Err(diag) => Err(ContainError::Escalated(diag)),
        }
    };
    for he in body.loop_cycle(first).ok_or(ContainError::Corrupt)? {
        let edge = body.get_half_edge(he).ok_or(ContainError::Corrupt)?.edge;
        let carrier = body
            .get_edge(edge)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(crate::null::CurveGeom::certified)
            .map(|c| c.carrier().clone());
        match carrier {
            Some(geom::Curve3::Line { dir, .. }) => {
                if !zero("bool_wall_iso_meridian", dir.cross(axis).norm())? {
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
                if !zero("bool_wall_iso_rim", c_axis.cross(axis).norm())?
                    || !zero("bool_wall_iso_rim", c_radius - radius)?
                    || !zero("bool_wall_iso_rim", off_axis)?
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
