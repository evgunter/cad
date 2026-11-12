//! Rows for a spline wall's paths ([`super::SplineWall`]): the parity of
//! a path across an island's boundary against the island's membership
//! on the wall's chart.

use std::sync::Arc;

use geom_core::spline::KnotVector;
use geom_core::{Point3, Sign};

use super::cone_islands::band;
use super::spline_wall::SplineWall;
use super::{LoopArc, SplineArc, path_parity};

/// A bent wall: one biquadratic span over the unit chart, bulged off
/// the plane `y = 0` in its middle and sheared along `x`, so neither
/// iso family is straight.
fn wall() -> geom::NurbsSurface<f64> {
    let clamped = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let (u, v) = (f64::from(i) / 2.0, f64::from(j) / 2.0);
            let bulge = if i == 1 && j == 1 { 0.8 } else { 0.0 };
            let lift = if i == 1 { 0.3 } else { 0.0 };
            control.push(Point3::new(
                2.0 * u + 0.4 * v * v,
                bulge + 0.2 * u * v,
                1.5 * v + lift,
            ));
        }
    }
    geom::NurbsSurface::new(clamped(), clamped(), control, vec![1.0; 9]).unwrap()
}

/// The island between the chart's iso curves `u ∈ {0.3, 0.7}` and
/// `v ∈ {0.25, 0.65}`.
const ISLAND: ((f64, f64), (f64, f64)) = ((0.3, 0.7), (0.25, 0.65));

fn inside((u, v): (f64, f64)) -> bool {
    let ((u0, u1), (v0, v1)) = ISLAND;
    u0 < u && u < u1 && v0 < v && v < v1
}

/// The island's four sides, each an iso curve of `chart` over its span:
/// the two `u` sides, then the two `v` sides.
fn island(chart: &geom::NurbsSurface<f64>) -> Vec<LoopArc<f64>> {
    let ((u0, u1), (v0, v1)) = ISLAND;
    let swapped = chart.transposed();
    let side = |net: &geom::NurbsSurface<f64>, at: f64, span: (f64, f64)| {
        let iso = geom_brep::interior_iso_u(net, at).unwrap();
        LoopArc::Spline(
            SplineArc::on_by_construction(&geom::Curve3::Nurbs(Arc::new(iso)), span).unwrap(),
        )
    };
    vec![
        side(chart, u0, (v0, v1)),
        side(chart, u1, (v0, v1)),
        side(&swapped, v0, (u0, u1)),
        side(&swapped, v1, (u0, u1)),
    ]
}

/// **A path on a bent spline wall crosses an island's boundary by the
/// oracle's parity**: between every pair of a grid of chart points clear
/// of the island's iso curves, every decided path agrees with the two
/// ends' membership of the island on the chart, every pair is offered
/// its two paths, nearly every path decides, and both parities are read.
#[test]
fn a_spline_wall_path_crosses_an_island_by_the_oracle_parity() {
    let chart = wall();
    let arcs = island(&chart);
    let wall = SplineWall::of(&geom::Surface::Nurbs(Arc::new(chart.clone()))).unwrap();
    let mut points = Vec::new();
    for u in [0.1, 0.45, 0.6, 0.9] {
        for v in [0.12, 0.4, 0.55, 0.85] {
            points.push((u, v));
        }
    }
    let (mut decided, mut asked, mut odd) = (0usize, 0usize, 0usize);
    for (i, &a) in points.iter().enumerate() {
        for &b in &points[i + 1..] {
            let ends = (chart.eval(a.0, a.1), chart.eval(b.0, b.1));
            let paths = wall.paths(ends, band()).unwrap();
            // A pair on one iso curve has one path, along it.
            let want_paths = if a.0 == b.0 || a.1 == b.1 { 1 } else { 2 };
            assert_eq!(paths.len(), want_paths, "{a:?} → {b:?}: its paths");
            for path in paths {
                asked += 1;
                if let Some(got) = path_parity(&path, &arcs, None, band()).unwrap() {
                    assert_eq!(got, inside(a) != inside(b), "{a:?} → {b:?}");
                    decided += 1;
                    odd += usize::from(got);
                }
            }
        }
    }
    assert!(
        decided * 100 >= asked * 95 && odd * 5 > decided && odd < decided,
        "{decided} of {asked} paths decided, {odd} odd"
    );
}

/// **A path arriving on a side of the island counts the crossings
/// before its arrival alone**, and leans as it arrives: to the middle of
/// the side `u = 0.7`, the path that arrives along the `v` iso through
/// it reads its start's side against the side it arrives from, and its
/// arrival is the wall's `u` direction there, signed by its travel.
#[test]
fn a_spline_wall_path_arriving_on_a_side_reads_its_starts_side() {
    let chart = wall();
    let arcs = island(&chart);
    let wall = SplineWall::of(&geom::Surface::Nurbs(Arc::new(chart.clone()))).unwrap();
    let ((_, u1), (v0, v1)) = ISLAND;
    let q = (u1, 0.5 * (v0 + v1));
    let to = chart.eval(q.0, q.1);
    let mut decided = 0;
    for a in [(0.1, 0.12), (0.45, 0.4), (0.9, 0.55), (0.6, 0.85), (0.9, 0.12)] {
        let paths = wall.paths((chart.eval(a.0, a.1), to), band()).unwrap();
        assert_eq!(paths.len(), 2, "{a:?}: its paths");
        // The first path arrives along the `v` iso, across the side; the
        // second along the side's own iso curve, which says nothing.
        let du = chart.ders(q.0, q.1).du;
        let travel = if a.0 < q.0 { 1.0 } else { -1.0 };
        assert!(
            (paths[0].arrival - du.normalize() * travel).norm() < 1e-12,
            "{a:?}: the first path arrives along the wall's u direction"
        );
        let got = path_parity(&paths[0], &arcs, Some(1), band()).unwrap();
        assert_eq!(
            got,
            Some(inside(a) != (a.0 < q.0)),
            "{a:?}: the crossings before the arrival"
        );
        decided += 1;
        assert_eq!(
            path_parity(&paths[1], &arcs, Some(1), band()).unwrap(),
            None,
            "{a:?}: a path along the side it arrives on says nothing"
        );
    }
    assert_eq!(decided, 5);
}

/// **A wall whose weights no iso curve shares is not read**, nor is a
/// plane; and a point off the wall has no path: its foot is not the
/// point.
#[test]
fn a_wall_with_no_shared_iso_weights_and_a_point_off_the_wall_are_not_read() {
    let chart = wall();
    let (knots_u, knots_v) = (
        chart.knots_u_f64_interim().clone(),
        chart.knots_v_f64_interim().clone(),
    );
    let mut weights = vec![1.0; 9];
    weights[4] = 0.5;
    let rational =
        geom::NurbsSurface::new(knots_u, knots_v, chart.control().to_vec(), weights).unwrap();
    assert!(SplineWall::of(&geom::Surface::Nurbs(Arc::new(rational))).is_none());
    let surface = geom::Surface::Nurbs(Arc::new(chart.clone()));
    let wall = SplineWall::of(&surface).unwrap();
    let (a, b) = (chart.eval(0.2, 0.2), chart.eval(0.8, 0.7));
    let off = b + surface.normal(0.8, 0.7) * 1e-3;
    assert_eq!(wall.paths((a, b), band()).unwrap().len(), 2);
    assert!(wall.paths((a, off), band()).unwrap().is_empty());
    assert!(wall.paths((off, b), band()).unwrap().is_empty());
    assert_eq!(
        crate::validate::decide(
            "split_ring_path_wall_foot",
            geom_core::Margin::of((off - b).norm()),
            band()
        ),
        Ok(Sign::Positive),
        "the point stands off the wall past the band"
    );
}
