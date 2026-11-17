//! **`NurbsCurve3::ders1_from` reads one side of a knot**
//! (`Curve3::ders1_from`): at a crease, each side's own derivative,
//! where `ders1` reads the span above for both; and at an enclosure that
//! may lie on either side, the hull of both, which encloses each.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/geom/src/curves/",];

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

/// Whether the enclosure `i` holds `x`.
fn holds(i: Interval, x: f64) -> bool {
    i.lo() <= x && x <= i.hi()
}

/// A degree-2 spline creased at `0.5` (a double knot): its leg below
/// arrives at the crease along `−x`, its leg above leaves it along `−y`.
fn creased() -> NurbsCurve3<f64> {
    NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![
            Point3::new(0.2, 0.2, 0.0),
            Point3::new(0.1, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, -0.1, 0.0),
            Point3::new(0.0, -0.2, 0.0),
        ],
        vec![1.0; 5],
    )
    .unwrap()
}

/// **At a crease each side reads its own derivative.** From below, the
/// leg arriving along `−x` (`2·(P₂ − P₁)/0.5 = (−0.4, 0, 0)`); from above,
/// the leg leaving along `−y` (`(0, −0.4, 0)`), at `f64` and at a point
/// `Interval` alike; away from the knot both sides are one span's. Red
/// where the step down to the span below is lost (both sides read the
/// span above, as `ders1` does).
#[test]
fn a_crease_reads_each_sides_own_derivative() {
    let c = creased();
    let close = |d: Vec3<f64>, want: [f64; 3]| (d - Vec3::from_array(want)).norm() <= 1e-12;
    let (_, below) = c.ders1_from(0.5, true);
    let (_, above) = c.ders1_from(0.5, false);
    assert!(close(below, [-0.4, 0.0, 0.0]), "from below: {below:?}");
    assert!(close(above, [0.0, -0.4, 0.0]), "from above: {above:?}");
    let ci = c.map_scalar(<Interval as Real>::from_f64);
    let (_, below_i) = ci.ders1_from(Interval::from_f64(0.5), true);
    assert!(
        holds(below_i.x, -0.4) && holds(below_i.y, 0.0),
        "from below at Interval: {below_i:?}"
    );
    let (_, inside_below) = c.ders1_from(0.25, true);
    let (_, inside_above) = c.ders1_from(0.25, false);
    assert!(
        (inside_below - inside_above).norm() == 0.0,
        "away from a knot, one span: {inside_below:?} against {inside_above:?}"
    );
}

/// **An enclosure astride the crease reads both sides.** The parameter
/// `[0.49, 0.51]` may lie in either leg, so neither side's derivative
/// holds every member: from either side it reads `ders1`'s hull over
/// both spans, which contains each leg's derivative at the crease. Red
/// where the located span's first is read alone.
#[test]
fn an_enclosure_astride_the_crease_reads_both_sides() {
    let ci = creased().map_scalar(<Interval as Real>::from_f64);
    let t = Interval::from_bounds(0.49, 0.51);
    for below in [true, false] {
        let (_, d) = ci.ders1_from(t, below);
        assert!(
            holds(d.x, -0.4) && holds(d.y, -0.4) && holds(d.x, 0.0) && holds(d.y, 0.0),
            "below = {below}: {d:?} holds both legs' derivatives"
        );
    }
}
