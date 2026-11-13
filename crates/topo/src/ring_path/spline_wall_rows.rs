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
fn clamped() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
}

fn wall() -> geom::NurbsSurface<f64> {
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
    for a in [
        (0.1, 0.12),
        (0.45, 0.4),
        (0.9, 0.55),
        (0.6, 0.85),
        (0.9, 0.12),
    ] {
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
    let mut weights = vec![1.0; 9];
    weights[4] = 0.5;
    let rational =
        geom::NurbsSurface::new(clamped(), clamped(), chart.control().to_vec(), weights).unwrap();
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

fn kv(knots: Vec<f64>, degree: usize) -> KnotVector {
    KnotVector::clamped(knots, degree).unwrap()
}

/// The spline arc of `curve` over its domain, on its wall by
/// construction.
fn spline(curve: geom::NurbsCurve3<f64>) -> LoopArc<f64> {
    let span = curve.knot_values().domain();
    LoopArc::Spline(
        SplineArc::on_by_construction(&geom::Curve3::Nurbs(Arc::new(curve)), span).unwrap(),
    )
}

/// The straight spline arc from `a` to `b`.
fn segment(a: Point3<f64>, b: Point3<f64>) -> LoopArc<f64> {
    spline(
        geom::NurbsCurve3::new(kv(vec![0.0, 0.0, 1.0, 1.0], 1), vec![a, b], vec![1.0, 1.0])
            .unwrap(),
    )
}

/// The arc of the unit circle about `z` at height `z`, from azimuth `a0`
/// to `a1` less than half a turn on, as a rational quadratic.
fn unit_arc(z: f64, (a0, a1): (f64, f64)) -> LoopArc<f64> {
    let (half, mid) = (0.5 * (a1 - a0), 0.5 * (a0 + a1));
    let at = |t: f64, r: f64| Point3::new(r * t.cos(), r * t.sin(), z);
    spline(
        geom::NurbsCurve3::new(
            kv(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2),
            vec![at(a0, 1.0), at(mid, 1.0 / half.cos()), at(a1, 1.0)],
            vec![1.0, half.cos(), 1.0],
        )
        .unwrap(),
    )
}

/// Whether `(x, y)` lies in the polygon `poly`, by the even-odd rule.
fn in_polygon(poly: &[(f64, f64)], (x, y): (f64, f64)) -> bool {
    let mut inside = false;
    for (i, &a) in poly.iter().enumerate() {
        let b = poly[(i + 1) % poly.len()];
        if (a.1 > y) != (b.1 > y) && x < a.0 + (y - a.1) * (b.0 - a.0) / (b.1 - a.1) {
            inside = !inside;
        }
    }
    inside
}

/// Every pair of `points` (each with its side of the island `arcs`)
/// read by every path the wall offers: the paths asked, the paths that
/// decided, the pairs offered none, and each decided reading that
/// disagrees with the two ends' sides.
fn read_pairs(
    wall: &SplineWall<f64>,
    arcs: &[LoopArc<f64>],
    points: &[(Point3<f64>, bool)],
) -> (usize, usize, usize, Vec<String>) {
    let (mut asked, mut decided, mut unpathed) = (0, 0, 0);
    let mut wrong = Vec::new();
    for (i, a) in points.iter().enumerate() {
        for b in &points[i + 1..] {
            let Ok(paths) = wall.paths((a.0, b.0), band()) else {
                continue;
            };
            unpathed += usize::from(paths.is_empty());
            for path in paths {
                asked += 1;
                if let Ok(Some(odd)) = path_parity(&path, arcs, None, band()) {
                    decided += 1;
                    if odd != (a.1 != b.1) {
                        wrong.push(format!("{:?} → {:?}: odd {odd}", a.0, b.0));
                    }
                }
            }
        }
    }
    (asked, decided, unpathed, wrong)
}

/// **A path on a distorted wall crosses a slanted island by the
/// even-odd oracle**: a flat wall whose chart is rational along `u`,
/// two spans each way and unevenly spaced, so its iso curves are
/// straight but its parameters are far from the plane's; and an island
/// that is a non-convex pentagon none of whose sides is an iso curve.
/// Between every pair of a grid of points that includes the wall's rim
/// and both sides of each knot line, no decided path disagrees with the
/// polygon's membership, and most decide.
#[test]
fn a_path_on_a_distorted_wall_crosses_a_slanted_island_by_the_even_odd_oracle() {
    let (xs, ys) = ([0.0, 0.05, 0.6, 1.0], [0.0, 0.5, 0.55, 1.0]);
    let wu = [1.0, 0.7, 1.3, 1.0];
    let control: Vec<_> = xs
        .iter()
        .flat_map(|&x| ys.iter().map(move |&y| Point3::new(x, y, 0.0)))
        .collect();
    let chart = geom::NurbsSurface::new(
        kv(vec![0.0, 0.0, 0.0, 0.3, 1.0, 1.0, 1.0], 2),
        kv(vec![0.0, 0.0, 0.0, 0.6, 1.0, 1.0, 1.0], 2),
        control,
        (0..16).map(|k| wu[k / 4]).collect(),
    )
    .unwrap();
    let (knot_x, knot_y) = (chart.eval(0.3, 0.5).x, chart.eval(0.5, 0.6).y);
    let wall = SplineWall::of(&geom::Surface::Nurbs(Arc::new(chart))).unwrap();
    let poly = [
        (0.2, 0.25),
        (0.8, 0.2),
        (0.6, 0.5),
        (0.75, 0.8),
        (0.25, 0.7),
    ];
    let arcs: Vec<_> = (0..poly.len())
        .map(|i| {
            let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
            segment(Point3::new(a.0, a.1, 0.0), Point3::new(b.0, b.1, 0.0))
        })
        .collect();
    let mut points = Vec::new();
    for x in [0.0, knot_x - 1e-6, knot_x + 1e-6, 0.68, 1.0] {
        for y in [0.0, 0.4, knot_y - 1e-6, knot_y + 1e-6, 0.97] {
            points.push((Point3::new(x, y, 0.0), in_polygon(&poly, (x, y))));
        }
    }
    let (asked, decided, unpathed, wrong) = read_pairs(&wall, &arcs, &points);
    assert!(wrong.is_empty(), "{wrong:#?}");
    assert!(
        unpathed == 0 && decided * 10 >= asked * 9,
        "{decided} of {asked} paths decided, {unpathed} pairs offered none"
    );
}

/// **An island across a closed wall's seam is read, but not from a
/// point just below the seam.** A whole cylinder as one rational spline
/// wall, closed along `u` with its seam at azimuth 0, and an island
/// over azimuths `±0.5` across it. Between points on either side of the
/// island every path decides, and as the island's membership says. A
/// point 0.01 below the seam has no foot (the projection settles toward
/// `u = 0` and stops there,
/// `work/sectarms/a-closed-spline-walls-projection-stops-at-its-seam.md`),
/// so it is offered no path: it says nothing, never a wrong side.
#[test]
fn an_island_across_a_closed_walls_seam_is_read_but_not_from_just_below_the_seam() {
    let s = std::f64::consts::FRAC_1_SQRT_2;
    let ring = [
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
        (-1.0, 1.0),
        (-1.0, 0.0),
        (-1.0, -1.0),
        (0.0, -1.0),
        (1.0, -1.0),
        (1.0, 0.0),
    ];
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for (i, &(x, y)) in ring.iter().enumerate() {
        for z in [0.0, 1.0] {
            control.push(Point3::new(x, y, z));
            weights.push(if i % 2 == 0 { 1.0 } else { s });
        }
    }
    let chart = geom::NurbsSurface::new(
        kv(
            vec![
                0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
            ],
            2,
        ),
        kv(vec![0.0, 0.0, 1.0, 1.0], 1),
        control,
        weights,
    )
    .unwrap();
    let wall = SplineWall::of(&geom::Surface::Nurbs(Arc::new(chart))).unwrap();
    let (t0, t1, z0, z1) = (-0.5, 0.5, 0.3, 0.7);
    let at = |t: f64, z: f64| Point3::new(t.cos(), t.sin(), z);
    let arcs = vec![
        unit_arc(z0, (t0, t1)),
        segment(at(t1, z0), at(t1, z1)),
        unit_arc(z1, (t0, t1)),
        segment(at(t0, z1), at(t0, z0)),
    ];
    let side = |t: f64, z: f64| (at(t, z), t0 < t && t < t1 && z0 < z && z < z1);
    let mut points = Vec::new();
    for t in [0.01, 0.2, 0.6, 3.0, -2.0, -0.3] {
        for z in [0.1, 0.5] {
            points.push(side(t, z));
        }
    }
    let (asked, decided, unpathed, wrong) = read_pairs(&wall, &arcs, &points);
    assert!(wrong.is_empty(), "{wrong:#?}");
    assert!(
        unpathed == 0 && decided == asked,
        "{decided} of {asked} paths decided, {unpathed} pairs offered none"
    );
    let below = at(-0.01, 0.5);
    for &(other, _) in &points {
        assert!(
            wall.paths((below, other), band()).unwrap().is_empty(),
            "a point just below the seam has no foot, and no path to {other:?}"
        );
    }
}

/// A wall arched along `u`: a parabola of radius of curvature 1/3 at
/// its crest, `u = 0.5`, carried straight along `v`.
fn arched() -> geom::NurbsSurface<f64> {
    let control = [(0.0, 0.0), (1.0, 3.0), (2.0, 0.0)]
        .into_iter()
        .flat_map(|(x, z)| [0.0, 0.5, 1.0].map(|y| Point3::new(x, y, z)))
        .collect();
    geom::NurbsSurface::new(clamped(), clamped(), control, vec![1.0; 9]).unwrap()
}

/// **A wall path's lean is levered no further than its arriving piece
/// reaches or turns.** The arm a path states is the lesser of the chord
/// of the piece it arrives along and that iso curve's radius of
/// curvature where it stops: the chord on the gently bent wall, the
/// radius where a path climbs the arched wall to its crest. And a
/// product net `wu ⊗ wv`, both factors varying, is no wall the paths
/// read.
#[test]
fn a_wall_paths_lean_is_levered_within_its_arriving_pieces_chord_and_turn() {
    let (mut tighter, mut shorter) = (0, 0);
    for (chart, a, b) in [
        (wall(), (0.1, 0.12), (0.9, 0.85)),
        (wall(), (0.45, 0.4), (0.5, 0.45)),
        (arched(), (0.05, 0.2), (0.5, 0.6)),
    ] {
        let wall = SplineWall::of(&geom::Surface::Nurbs(Arc::new(chart.clone()))).unwrap();
        let ends = (chart.eval(a.0, a.1), chart.eval(b.0, b.1));
        let paths = wall.paths(ends, band()).unwrap();
        assert_eq!(paths.len(), 2, "{a:?} → {b:?}: its paths");
        // The first path arrives along `u` from the corner `(a.u, b.v)`,
        // the second along `v` from `(b.u, a.v)`.
        for (path, corner, along_u) in [
            (&paths[0], (a.0, b.1), true),
            (&paths[1], (b.0, a.1), false),
        ] {
            let reach = path.reach.unwrap().length();
            let chord = (ends.1 - chart.eval(corner.0, corner.1)).norm();
            let jet = chart.ders(b.0, b.1);
            let (d1, d2) = if along_u {
                (jet.du, jet.duu)
            } else {
                (jet.dv, jet.dvv)
            };
            let radius = d1.norm().powi(3) / d1.cross(d2).norm();
            assert!(
                reach <= chord * (1.0 + 1e-12) && reach <= radius * (1.0 + 1e-12),
                "{a:?} → {b:?}: the arm {reach} against the chord {chord} and the radius {radius}"
            );
            assert!(
                reach >= 0.999 * chord.min(radius),
                "{a:?} → {b:?}: the arm {reach} is the lesser of {chord} and {radius}"
            );
            tighter += usize::from(radius < chord);
            shorter += usize::from(chord < radius);
        }
    }
    assert!(
        tighter > 0 && shorter > 0,
        "{tighter} pieces turn tighter than their chords and {shorter} do not"
    );
    let (wu, wv) = ([1.0, 0.7, 1.0], [1.0, 1.5, 1.0]);
    let product = geom::NurbsSurface::new(
        clamped(),
        clamped(),
        wall().control().to_vec(),
        (0..9).map(|k| wu[k / 3] * wv[k % 3]).collect(),
    )
    .unwrap();
    assert!(
        SplineWall::of(&geom::Surface::Nurbs(Arc::new(product))).is_none(),
        "a product net with both factors varying"
    );
}
