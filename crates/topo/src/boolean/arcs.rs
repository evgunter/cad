//! **Circle arcs read off a vertex's orbit**, shared by the reduction's
//! on-carrier certificates ([`super::reduce`]) and the declared-REST
//! zip ([`super::rest`]): which arc of a body leaves a vertex along a
//! given tangent, and how far a point misses a circle.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::BooleanError;
use crate::body::Body;
use crate::entity::{EdgeKey, VertexKey};
use crate::validate::decide;

/// One circle arc [`arcs_along`] found: the edge, the vertex it arrives
/// at, and its tangent there in the direction of travel.
#[derive(Clone, Copy, Debug)]
pub(super) struct ArcStep<T: geom_core::Real> {
    pub(super) edge: EdgeKey,
    pub(super) to: VertexKey,
    pub(super) arrival: Vec3<T>,
}

/// The circle arcs of `body` leaving `u` whose departure tangent is
/// decided ALIGNED with `dir`. A circle through a point with a given
/// tangent there is fixed up to its radius, and two arcs of a valid
/// body leaving one vertex along one tangent on one circle would
/// overlap, so a caller reads more than one arc as a refusal.
///
/// The outer error is corruption (an unwalkable orbit); the inner one
/// is the alignment's escalation: a tangent in band of `dir` is
/// neither along it nor off it.
pub(super) fn arcs_along<T: Decide>(
    body: &Body<T>,
    u: VertexKey,
    dir: Vec3<T>,
    band: Band,
) -> Result<Result<Vec<ArcStep<T>>, Indeterminate>, BooleanError> {
    let corrupt = |what| BooleanError::ClassificationInvariant { what };
    let Some(anchor) = body.get_vertex(u).and_then(|vd| vd.emanating) else {
        return Ok(Ok(Vec::new()));
    };
    let orbit = body
        .vertex_orbit(anchor)
        .ok_or_else(|| corrupt("arc lookup: vertex orbit not walkable"))?;
    let d = dir.normalize();
    let mut found: Vec<ArcStep<T>> = Vec::new();
    for he in orbit {
        let Some(to) = body.half_edge_end(he) else {
            return Err(corrupt("arc lookup: orbit half has no end"));
        };
        let e = body
            .get_half_edge(he)
            .ok_or_else(|| corrupt("arc lookup: orbit half no longer resolves"))?
            .edge;
        let edge = body
            .get_edge(e)
            .ok_or_else(|| corrupt("arc lookup: orbit edge no longer resolves"))?;
        let Some(curve) = body
            .get_curve_geom(edge.curve)
            .and_then(crate::null::CurveGeom::certified)
        else {
            continue;
        };
        let geom::Curve3::Circle { radius, .. } = *curve.carrier() else {
            continue;
        };
        let (tangent, arrival) = curve.walk_tangents(he == edge.he_plus);
        let tangent = tangent.normalize();
        let off = match decide(
            "bool_arc_along",
            Margin::levered(tangent.cross(d).norm(), radius),
            band,
        ) {
            Ok(s) => s,
            Err(diag) => return Ok(Err(diag)),
        };
        let ahead = match decide(
            "bool_arc_ahead",
            Margin::levered(tangent.dot(d), radius),
            band,
        ) {
            Ok(s) => s,
            Err(diag) => return Ok(Err(diag)),
        };
        if off == Sign::Zero && ahead == Sign::Positive && found.iter().all(|f| f.edge != e) {
            found.push(ArcStep {
                edge: e,
                to,
                arrival,
            });
        }
    }
    Ok(Ok(found))
}

/// The distance from `p` to the circle (`center`, unit `axis`,
/// `radius`): `√(h² + (ρ − r)²)`, `h` its height over the circle's plane
/// and `ρ` its distance from the axis.
pub(super) fn circle_miss<T: Decide>(
    p: Point3<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
) -> T {
    let off = p - center;
    let h = off.dot(axis);
    let rho = (off - axis * h).norm();
    (h.powi(2) + (rho - radius).powi(2)).sqrt()
}
