//! **Circle arcs read off a vertex's orbit**, for the reduction's
//! on-carrier certificates ([`super::reduce`]): which arc of a body
//! leaves a vertex along a given tangent.

use geom_core::{Band, Decide, Indeterminate, Margin, Sign, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, VertexKey};
use crate::live::{linked, proven};
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
/// The error is the alignment's escalation: a tangent in band of `dir`
/// is neither along it nor off it.
///
/// # Panics
///
/// Where `u`, a vertex the caller read out of `body`, has an orbit that
/// does not walk, or a record on it does not resolve (D2 row 4).
#[track_caller]
pub(super) fn arcs_along<T: Decide>(
    body: &Body<T>,
    u: VertexKey,
    dir: Vec3<T>,
    band: Band,
) -> Result<Vec<ArcStep<T>>, Indeterminate> {
    let orbit = body.vertex_orbit_linked(u);
    let d = dir.normalize();
    let mut found: Vec<ArcStep<T>> = Vec::new();
    for he in orbit {
        let to = body.proven_half_edge_end(he);
        let e = proven(&body.half_edges, he, EntityId::HalfEdge).edge;
        let edge = linked(
            &body.edges,
            e,
            EntityId::Edge,
            EntityId::HalfEdge(he),
            "edge",
        );
        let Some(curve) = body.edge_curve_linked(e, edge).certified() else {
            continue;
        };
        let geom::Curve3::Circle { radius, .. } = *curve.carrier() else {
            continue;
        };
        let (tangent, arrival) = curve.walk_tangents(he == edge.he_plus);
        let tangent = tangent.normalize();
        // Off the tangent's line rules the arc out whatever its
        // direction; an arc decided backward is ruled out whatever its
        // offset. Only an arc neither rules out escalates.
        let off = decide(
            "bool_arc_along",
            Margin::levered(tangent.cross(d).norm(), radius),
            band,
        );
        if off == Ok(Sign::Positive) {
            continue;
        }
        match decide(
            "bool_arc_ahead",
            Margin::levered(tangent.dot(d), radius),
            band,
        ) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Negative | Sign::Zero) => continue,
            Err(diag) => return Err(diag),
        }
        off?;
        if found.iter().all(|f| f.edge != e) {
            found.push(ArcStep {
                edge: e,
                to,
                arrival,
            });
        }
    }
    Ok(found)
}

/// **What [`arcs_along`] escalates on.** An arc it rules out by one
/// decision never escalates on the other: one decided off the tangent's
/// line whatever its direction, one decided backward whatever its
/// offset. The quarter sheet's bottom arc leaves `(1, 0, 0)` along `+y`.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod rows {
    use super::arcs_along;
    use crate::body::Body;
    use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
    use core::f64::consts::FRAC_PI_2;
    use geom_core::{Band, Point3, Tol, Vec3};

    #[test]
    fn an_arc_ruled_out_by_one_decision_does_not_escalate_on_the_other() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the witness band");
        let mut y = Body::<f64>::new();
        cyl_wall_sheet(
            &mut y,
            CylFrame::canonical(1.0),
            (0.0, FRAC_PI_2),
            (0.0, 1.0),
            tol,
        );
        let p = y
            .vertex_points()
            .find(|(_, q)| (*q - Point3::new(1.0, 0.0, 0.0)).norm() < 1e-12)
            .map(|(k, _)| k)
            .expect("the sheet's corner");
        let tilt = (band.zero() + band.escalate()) / 2.0;
        let arcs = |dir: Vec3<f64>| arcs_along(&y, p, dir, band);
        // Backward, its offset in band: decided backward.
        assert_eq!(
            arcs(Vec3::new(0.0, -1.0, tilt)).map(|a| a.len()),
            Ok(0),
            "backward"
        );
        // Square to it, its direction in band: decided off the line.
        assert_eq!(
            arcs(Vec3::new(tilt, 0.0, 1.0)).map(|a| a.len()),
            Ok(0),
            "square"
        );
        // Ahead with its offset in band: neither rules it out.
        assert!(arcs(Vec3::new(0.0, 1.0, tilt)).is_err(), "ahead, in band");
        assert_eq!(arcs(Vec3::unit_y()).map(|a| a.len()), Ok(1), "along it");
    }
}
