//! **The quotient bound's soundness clauses**, one named row each, read
//! through the curve meter (`NurbsCurve3::speed_lower_bound` and
//! `speed_upper_bound`, which read `geom_core::spline::QuotientPiece`
//! on each span's dyadic pieces). The surface doors read the same type;
//! their locality and tightness rows are in `surfaces/stretch_floor.rs`.
//!
//! Each fixture is a counterexample to the statement with its clause
//! removed — a search over random rational Bézier curves found them —
//! so the row goes red under that mutant: the bound claims past the
//! sampled truth.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/geom/src/curves/", "crates/geom-core/src/spline/",];

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real};

/// The Bézier curve of degree `pts.len() − 1` on `[0, 1]`.
fn bezier(pts: &[(f64, f64)], w: &[f64]) -> NurbsCurve3<f64> {
    let p = pts.len() - 1;
    let mut k = vec![0.0; p + 1];
    k.extend(vec![1.0; p + 1]);
    NurbsCurve3::new(
        KnotVector::clamped(k, p).unwrap(),
        pts.iter().map(|&(x, y)| Point3::new(x, y, 0.0)).collect(),
        w.to_vec(),
    )
    .unwrap()
}

/// `(inf, sup)` of `‖C′‖` sampled at 2001 parameters: the least
/// certified upper end and the greatest certified lower end, so a
/// floor above the first or a bound below the second is unsound with
/// certainty.
fn sampled(c: &NurbsCurve3<f64>) -> (f64, f64) {
    let ci = c.map_scalar(<Interval as Real>::from_f64);
    (0..=2000).fold((f64::INFINITY, 0.0f64), |(inf, sup), i| {
        let n = ci.deriv(Interval::from_f64(f64::from(i) / 2000.0)).norm();
        (inf.min(n.hi()), sup.max(n.lo()))
    })
}

/// Both doors hold against the samples.
#[track_caller]
fn holds(name: &str, c: &NurbsCurve3<f64>) -> (f64, f64) {
    let (inf, sup) = sampled(c);
    let (m, s) = (c.speed_lower_bound().get(), c.speed_upper_bound().get());
    assert!(m <= inf, "{name}: floor {m} above the sampled inf {inf}");
    assert!(s >= sup, "{name}: bound {s} below the sampled sup {sup}");
    (m, s)
}

/// **Each ratio pairs a numerator coefficient with its own weight-power
/// coefficient.** On this cubic the speed's least is `0.2825`; paired
/// with the weight power's coefficients in reverse order the floor
/// claims `0.3339`.
#[test]
fn each_ratio_pairs_its_own_weight_power_coefficient() {
    let c = bezier(
        &[
            (0.19733581520692167, 0.29575281072787274),
            (1.0628063293238148, 0.1467605042029576),
            (2.3473781038384107, -0.08164214941135728),
            (3.339604585266082, 0.1620759750919315),
        ],
        &[
            2.63237382145462,
            0.28228394867077117,
            1.1331742582165845,
            1.36385355189667,
        ],
    );
    holds("reversed pairing witness", &c);
}

/// **Each ratio divides by its own weight-power coefficient, not the
/// piece's least.** On this rational line the speed's least is
/// `0.6340`, at the heavier end; over each piece's least weight power
/// the floor claims `0.6586`.
#[test]
fn each_ratio_divides_by_its_own_weight_power_coefficient() {
    let c = bezier(
        &[
            (0.008820827041011059, 0.48838532997553874),
            (1.2729670351987803, -0.4869719549252166),
        ],
        &[2.7409622875087942, 1.0884427170239037],
    );
    holds("least weight power witness", &c);
}

/// **Numerator and weight power sit at one degree.** The same line's
/// speed peaks at `4.0208`; with the numerator left at degree `2p − 1`
/// and paired with the first `2p` of the weight power's `2p + 1`
/// coefficients the bound reads `3.8387`.
#[test]
fn numerator_and_weight_power_sit_at_one_degree() {
    let c = bezier(
        &[
            (0.008820827041011059, 0.48838532997553874),
            (1.2729670351987803, -0.4869719549252166),
        ],
        &[2.7409622875087942, 1.0884427170239037],
    );
    holds("unelevated numerator witness", &c);
}
