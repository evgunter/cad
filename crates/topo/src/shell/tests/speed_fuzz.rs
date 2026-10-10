//! The counterexample search behind [`super::super::spline_speed`]:
//! rational splines nobody chose, on knots nobody chose, sampled densely
//! against the speed bound the tilted read balls their pieces with.

test_utils::gated_to![
    "crates/topo/src/shell.rs",
    "crates/geom/src/curves/",
    "crates/geom-core/src/spline/",
];

use super::super::spline_speed;
use geom_core::Point3;

/// **No rational spline is faster than its bound.** Degrees 1 to 3,
/// up to eight control points in a box of side 2, weights spread over
/// three decades (the weight term carries the bound there), interior
/// knots drawn and sorted: `|C′|` at 400 parameters never exceeds
/// [`spline_speed`].
#[test]
fn no_rational_spline_outruns_its_speed_bound() {
    let mut rng = test_utils::fuzz::start("shell spline speed");
    for i in 0..test_utils::fuzz::scaled(300) {
        let degree = 1 + rng.below(3);
        let n = degree + 1 + rng.below(5);
        let mut interior: Vec<f64> = (0..n - degree - 1).map(|_| rng.range(0.05, 0.95)).collect();
        interior.sort_by(f64::total_cmp);
        let mut knots = vec![0.0; degree + 1];
        knots.extend(interior);
        knots.extend(vec![1.0; degree + 1]);
        let control: Vec<_> = (0..n)
            .map(|_| {
                Point3::new(
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                )
            })
            .collect();
        let weights = (0..n).map(|_| 10f64.powf(rng.range(-1.5, 1.7))).collect();
        let Ok(kv) = geom_core::spline::KnotVector::clamped(knots, degree) else {
            continue;
        };
        let spline = geom::NurbsCurve3::new(kv, control, weights).unwrap();
        let bound = spline_speed(&spline);
        for k in 0..=400 {
            let t = k as f64 / 400.0;
            let speed = spline.deriv(t).norm();
            assert!(
                speed <= bound * (1.0 + 1e-9),
                "#{i}: |C'({t})| = {speed} exceeds the bound {bound}; {}",
                test_utils::fuzz::replay()
            );
        }
    }
}
