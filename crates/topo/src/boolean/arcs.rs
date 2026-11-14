//! **Circle arcs read off a vertex's orbit**, for the reduction's
//! on-carrier certificates ([`super::reduce`]): which arc of a body
//! leaves a vertex along a given tangent.

use geom_core::{Band, Decide, Indeterminate, Margin, Sign, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, VertexKey};
use crate::live::{Proven, linked, proven};
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
    let at = body.resolve_vertex_point(u, Proven);
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
        let geom::Curve3::Circle(_) = curve.carrier() else {
            continue;
        };
        let (tangent, arrival) = curve.walk_tangents(he == edge.he_plus);
        let tangent = tangent.normalize();
        // A tilt of the tangent about `u` moves the arc, so it is levered
        // at the arc's farthest reach from `u`.
        let arm = geom_brep::ExtentBall::of_cover_from(
            crate::cover::edge_cover_linked(body, e, edge).as_ref().ok(),
            at,
        )
        .lever();
        // Off the tangent's line rules the arc out whatever its
        // direction; an arc decided backward is ruled out whatever its
        // offset. Only an arc neither rules out escalates.
        let off = decide(
            "bool_arc_along",
            Margin::levered(tangent.cross(d).norm(), arm),
            band,
        );
        if off == Ok(Sign::Positive) {
            continue;
        }
        match decide("bool_arc_ahead", Margin::levered(tangent.dot(d), arm), band) {
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

    /// **A short arc of a large circle is levered at its own reach from
    /// the vertex.** The bottom arc of a sheet of radius `100` over
    /// `10⁻³` rad leaves `(100, 0, 0)` along `+y` and reaches `0.1` from
    /// it. A direction tilted `ε/2` off its tangent moves the arc by
    /// `ε/20` over that reach, in the band, so the arc is found along it;
    /// levered at its whole circle's ball, radius `100`, the same tilt
    /// reads `50ε`, past `Kε`, and rules the arc out.
    #[test]
    fn a_short_arc_of_a_large_circle_is_levered_at_its_own_reach() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the witness band");
        let mut y = Body::<f64>::new();
        cyl_wall_sheet(
            &mut y,
            CylFrame::canonical(100.0),
            (0.0, 1e-3),
            (0.0, 1.0),
            tol,
        );
        let p = y
            .vertex_points()
            .find(|(_, q)| (*q - Point3::new(100.0, 0.0, 0.0)).norm() < 1e-9)
            .map(|(k, _)| k)
            .expect("the sheet's corner");
        let tilt = band.zero() / 2.0;
        assert!(
            100.0 * tilt > band.escalate(),
            "the whole circle's lever leaves the band"
        );
        assert_eq!(
            arcs_along(&y, p, Vec3::new(0.0, 1.0, tilt), band).map(|a| a.len()),
            Ok(1),
            "the arc tilted within the band over its own reach"
        );
    }

    /// **The arc is levered from its vertex, not about its own ball's
    /// centre.** A half-turn arc of the unit circle leaves `(1, 0, 0)`
    /// along `+y` and reaches `2` from it, the far end of a diameter;
    /// its cover's ball has a radius `r` below that. A tilt `t` with
    /// `t·r` in the zero band but `t·2` past it moves the arc's far end
    /// out of the band, so the arc is not found along it; levered at the
    /// ball's radius, with no pivot, the same tilt reads Zero and finds
    /// it.
    #[test]
    fn a_half_turn_arc_is_levered_at_its_far_end_from_the_vertex() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the witness band");
        let mut y = Body::<f64>::new();
        cyl_wall_sheet(
            &mut y,
            CylFrame::canonical(1.0),
            (0.0, core::f64::consts::PI),
            (0.0, 1.0),
            tol,
        );
        let start = Point3::new(1.0, 0.0, 0.0);
        let p = y
            .vertex_points()
            .find(|(_, q)| (*q - start).norm() < 1e-12)
            .map(|(k, _)| k)
            .expect("the sheet's corner");
        let arc = y
            .edges
            .iter()
            .map(|(k, _)| k)
            .find(|&k| {
                let e = &y.edges[k];
                y.edge_curve_linked(k, e)
                    .certified()
                    .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Circle(_)))
                    && crate::cover::edge_cover(&y, k)
                        .is_ok_and(|c| c.distance_range(&geom::cover::Site::Point(start)).0 < 1e-9)
            })
            .expect("the bottom arc");
        let cover = crate::cover::edge_cover(&y, arc).expect("its cover");
        let ball = geom_brep::ExtentBall::of_covered(Some(&cover))
            .lever()
            .length();
        let far = geom_brep::ExtentBall::of_cover_from(Some(&cover), start)
            .lever()
            .length();
        let tilt = band.zero() * (1.0 / ball + 1.0 / far) / 2.0;
        assert!(
            tilt * ball < band.zero() && tilt * far > band.zero(),
            "the tilt sits between the two arms: ball {ball}, far {far}"
        );
        assert_ne!(
            arcs_along(&y, p, Vec3::new(0.0, 1.0, tilt), band).map(|a| a.len()),
            Ok(1),
            "the arc's far end leaves the band"
        );
    }
}
