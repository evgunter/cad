//! **A chart singularity stays a vertex** (`docs/DESIGN.md`, maximal
//! edges: poles and apexes stay vertices). A full revolve's sphere
//! reaches each pole, and a cone its apex, by one wrap edge: a tip, with
//! nothing to join. The ball cut into two lunes has a valence-2 vertex
//! at each pole, whose two edges are chart edges of one surface key on
//! one iso family, two meridians: every structural reading the chart arm
//! of `joinable` takes holds there, and the join refuses on the vertex's
//! distance from the chart's singular set alone.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Tol};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;

#[test]
fn a_spheres_poles_and_a_cones_apex_are_not_joinable() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let ball = || {
        revolved_about_y(
            vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
            Revolution::Full,
            tol,
        )
    };
    let lunes = {
        let mut b = ball();
        crate::seam_region_common::two_lunes(&mut b, tol);
        b
    };
    let cone = revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol,
    );
    for (label, b, on_axis, valence) in [
        ("the ball", ball(), 2, 1),
        ("the two lunes", lunes, 2, 2),
        ("the cone", cone, 1, 1),
    ] {
        let mut singular = 0;
        for (_, d) in b.vertices() {
            let p = *b.get_point(d.point).unwrap();
            if p.x != 0.0 || p.z != 0.0 {
                continue;
            }
            singular += 1;
            let orbit = b.vertex_orbit(d.emanating.unwrap()).unwrap();
            let surfaces: Vec<_> = orbit
                .iter()
                .map(|&h| {
                    let e = b.get_edge(b.get_half_edge(h).unwrap().edge).unwrap();
                    match b
                        .get_curve_geom(e.curve)
                        .unwrap()
                        .certified()
                        .unwrap()
                        .description()
                    {
                        geom_brep::EdgeDescription::Chart(c) => c.surface,
                        other => panic!("{label}: a chart edge at the axis, got {other:?}"),
                    }
                })
                .collect();
            assert_eq!(surfaces.len(), valence, "{label}: the valence at the axis");
            assert!(
                surfaces.iter().all(|&s| s == surfaces[0]),
                "{label}: every edge at the axis on one chart: {surfaces:?}"
            );
        }
        assert_eq!(singular, on_axis, "{label}: its vertices on the axis");
        assert_eq!(
            topo::joinable_vertices(&b, band).unwrap(),
            vec![],
            "{label}"
        );
    }
}
