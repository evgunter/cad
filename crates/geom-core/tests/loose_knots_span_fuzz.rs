//! `LooseKnots::span_of` against its contract, over random accepted
//! lists: degree 0–4, interior runs up to `degree + 4` long (the shape
//! no `KnotVector` holds), probed at every knot, one ulp either side,
//! every midpoint, both domain ends and past them. A counterexample
//! search (shape 1): the seed varies and is logged; scale with
//! `CAD_FUZZ_EFFORT`.

#![allow(clippy::panic, clippy::unwrap_used)]

test_utils::gated_to![
    "crates/geom-core/src/spline/knots.rs",
    "crates/geom-core/src/spline/range.rs"
];

use geom_core::spline::{LooseKnots, Param};
use test_utils::fuzz;

/// A random list clamped at both ends: `degree + 1` copies of each
/// end, and up to five interior values with runs of 1 ..= `degree + 4`.
fn draw(rng: &mut fuzz::Rng) -> (Vec<f64>, usize) {
    let degree = rng.below(5);
    let lo = rng.range(-10.0, 10.0);
    let mut knots = vec![lo; degree + 1];
    let mut v = lo;
    for _ in 0..rng.below(6) {
        // A spacing from a few ulps to a few units.
        v += rng.log_uniform(1e-14, 4.0) * v.abs().max(1.0);
        knots.extend(core::iter::repeat_n(v, 1 + rng.below(degree + 4)));
    }
    v += rng.log_uniform(1e-14, 4.0) * v.abs().max(1.0);
    knots.extend(core::iter::repeat_n(v, degree + 1));
    (knots, degree)
}

fn probes(knots: &[f64]) -> Vec<f64> {
    let mut ts = vec![f64::NEG_INFINITY, f64::INFINITY];
    for &k in knots {
        ts.extend([k.next_down(), k, k.next_up()]);
    }
    for w in knots.windows(2) {
        ts.push(0.5 * (w[0] + w[1]));
    }
    let (lo, hi) = (knots[0], knots[knots.len() - 1]);
    ts.extend([lo - 1.0, hi + 1.0]);
    ts
}

#[test]
fn a_loose_span_contains_its_parameter_and_clamps_at_the_ends() {
    let mut rng = fuzz::start("loose_knots_span_fuzz");
    let mut located = 0usize;
    for _ in 0..fuzz::scaled(2_000) {
        let (knots, degree) = draw(&mut rng);
        let loose = LooseKnots::new(knots.clone(), degree).unwrap_or_else(|e| {
            panic!(
                "an end-clamped draw refused: {e} on {knots:?} ({})",
                fuzz::replay()
            )
        });
        let (first, last) = (degree, loose.control_count() - 1);
        let (lo, hi) = (knots[0], knots[knots.len() - 1]);
        for t in probes(&knots) {
            let s = loose.span_of(Param::new(t).unwrap());
            let ctx = || {
                format!(
                    "t = {t:e}, span {s}, degree {degree}, {knots:?} ({})",
                    fuzz::replay()
                )
            };
            assert!((first..=last).contains(&s), "range: {}", ctx());
            assert!(knots[s] < knots[s + 1], "an empty span: {}", ctx());
            if t < lo {
                assert_eq!(s, first, "below the domain, the first span: {}", ctx());
            } else if t >= hi {
                assert_eq!(s, last, "at or past the end, the last span: {}", ctx());
            } else {
                assert!(
                    knots[s] <= t && t < knots[s + 1],
                    "inside the domain, knots[s] <= t < knots[s + 1]: {}",
                    ctx()
                );
            }
            located += 1;
        }
    }
    println!("[fuzz] located {located} probes");
}
