//! **Bit identity across the coefficient-hull doors, the extended
//! corpus** — adopted from the dual review's probe. It extends
//! `coeffs_bit_identity.rs` to the cases that corpus does not reach:
//! degrees 1–5 at MINIMAL control counts (the `derivative_coeffs`
//! length floor), interior multiplicity up to the degree, knot spacings
//! across sixteen orders of magnitude, weight lanes that are unit /
//! varied / subnormal / negative / NaN / infinite, and coefficient
//! brackets of NONZERO width — 3,403 `f64` values by their bits, across
//! every door of both pairs.
//!
//! **How the expectation was obtained**: the corpus was compiled
//! against the merge-base rlib (`ea11576b4`) through the RETIRED free
//! spellings (`hull::span_hull`, `hull::domain_hull`,
//! `hull::derivative_coeffs`, …) and its digest captured; `DIGEST`
//! below is that capture. It runs on both lanes (no feature gate) and
//! reuses the shipped suite's digest fold.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::KnotVector;
use geom_core::{CertifiedEnclosure, RingInterval};

use crate::coeffs_bit_identity::{Rows, digest};

/// Every door, with the labels the retired free spellings produced.
fn drive<E: CertifiedEnclosure>(
    o: &mut Rows,
    name: &str,
    kv: &KnotVector,
    coeffs: &[E],
    w: &[f64],
) {
    let pair = kv.with_coeffs(coeffs).expect("its own vector");
    let rational = kv.with_rational_coeffs(coeffs, w).expect("its own vector");
    for index in kv.first_span()..=kv.last_span() {
        let Some(win) = pair.span(index) else {
            continue;
        };
        let rwin = rational.span(index).expect("the same vector's span");
        ri(o, &format!("{name}.hull@{index}"), win.hull());
        ri(o, &format!("{name}.hull_rat@{index}"), rwin.hull_rational());
        ri(o, &format!("{name}.dhull@{index}"), win.derivative_hull());
        o.push((
            format!("{name}.sup@{index}"),
            win.sup_norm_bound().to_bits(),
        ));
    }
    ri(o, &format!("{name}.domain"), pair.domain_hull());
    ri(
        o,
        &format!("{name}.domain_rat"),
        rational.domain_hull_rational(),
    );
    let qs = pair.derivative_coeffs();
    o.push((format!("{name}.dcoeff.len"), qs.len() as u64));
    for (i, q) in qs.iter().enumerate() {
        ri(o, &format!("{name}.dcoeff.{i}"), *q);
    }
    ri(o, &format!("{name}.ddomain"), pair.derivative_domain_hull());
    o.push((
        format!("{name}.sup_domain"),
        pair.sup_norm_bound().to_bits(),
    ));
    o.push((
        format!("{name}.sup_domain_rat"),
        rational.sup_norm_bound_rational().to_bits(),
    ));
}

fn vectors() -> Vec<(&'static str, KnotVector)> {
    let kv = |k: &[f64], p: usize| KnotVector::clamped(k.to_vec(), p).expect("valid");
    vec![
        // degree 1, minimal control count (2) — derivative_coeffs's floor
        ("x.d1min", kv(&[0.0, 0.0, 1.0, 1.0], 1)),
        // degree 1 with several interior knots
        (
            "x.d1many",
            kv(&[0.0, 0.0, 0.1, 0.4, 0.55, 0.9, 1.0, 1.0], 1),
        ),
        // degree 2, minimal
        ("x.d2min", kv(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2)),
        // degree 3, minimal (single Bezier span)
        ("x.d3min", kv(&[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3)),
        // degree 5 with multiplicities 1..5, an unequal-width domain
        (
            "x.d5",
            kv(
                &[
                    -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.5, 2.5,
                    2.5, 2.5, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0,
                ],
                5,
            ),
        ),
        // degree 4, multiplicity 4 interior (a C^0 kink at the top of the budget)
        (
            "x.d4m4",
            kv(
                &[
                    0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.75, 1.0, 1.0, 1.0, 1.0, 1.0,
                ],
                4,
            ),
        ),
        // huge / tiny knot spacings (the deriv_coeff denominator)
        (
            "x.scale",
            kv(&[0.0, 0.0, 0.0, 1e-8, 1e8, 1e8 + 1.0, 2e8, 2e8, 2e8], 2),
        ),
    ]
}

#[allow(clippy::cast_precision_loss)]
fn values(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| ((i * 13) % 17) as f64 * 0.3125 - 2.0 + i as f64 * 0.0117)
        .collect()
}

/// Weight lanes: `0` unit, `1` varied positive, `2` a near-zero
/// (subnormal-adjacent) weight, `3` a negative weight, `4` a NaN,
/// `5` an infinity.
#[allow(clippy::cast_precision_loss)]
fn weights(n: usize, lane: usize) -> Vec<f64> {
    let mut w: Vec<f64> = (0..n)
        .map(|i| match lane {
            0 => 1.0,
            _ => 0.5 + ((i * 5) % 7) as f64 * 0.25,
        })
        .collect();
    let mid = n / 2;
    match lane {
        2 => w[mid] = 5e-324,
        3 => w[mid] = -1.0,
        4 => w[mid] = f64::NAN,
        5 => w[mid] = f64::INFINITY,
        _ => {}
    }
    w
}

fn ri(o: &mut Rows, tag: &str, r: RingInterval) {
    o.push((format!("{tag}.lo"), r.lo().to_bits()));
    o.push((format!("{tag}.hi"), r.hi().to_bits()));
}

fn rows() -> Rows {
    let mut o = Vec::new();
    for (vname, kv) in vectors() {
        let n = kv.control_count();
        let vals = values(n);
        // f64 brackets
        for lane in 0..6 {
            let w = weights(n, lane);
            drive(&mut o, &format!("{vname}.f64.w{lane}"), &kv, &vals, &w);
        }
        // RingInterval brackets of NONZERO width
        #[allow(clippy::cast_precision_loss)]
        let wide: Vec<RingInterval> = vals
            .iter()
            .enumerate()
            .map(|(i, x)| {
                RingInterval::hull(
                    RingInterval::point(*x - 0.01 * (i as f64 + 1.0)),
                    RingInterval::point(*x + 0.02),
                )
            })
            .collect();
        for lane in 0..6 {
            let w = weights(n, lane);
            drive(&mut o, &format!("{vname}.ring.w{lane}"), &kv, &wide, &w);
        }
        o.push((format!("{vname}.control_count"), n as u64));
    }
    o
}

const ROW_COUNT: usize = 3403;
/// FNV-1a 64 over `"{name} {bits:#018x}\n"`, the shipped suite's digest shape.
const DIGEST: u64 = 0x9897_c316_665d_3ab0;

#[test]
fn the_extended_coefficient_corpus_is_bit_identical_to_its_retired_spelling() {
    let rows = rows();
    assert_eq!(rows.len(), ROW_COUNT, "corpus size drifted");
    assert_eq!(digest(&rows), DIGEST, "a coefficient door's bits moved");
}

/// The premise `SplineCoeffs::derivative_coeffs` rests on: every knot
/// vector this crate can build has `control_count() >= 2`, so
/// `coeffs.len() - 1` cannot underflow and the answer is never empty.
#[test]
fn every_vector_admits_at_least_two_coefficients() {
    for (name, kv) in vectors() {
        assert!(kv.control_count() >= 2, "{name}");
        let c = values(kv.control_count());
        let pair = kv.with_coeffs(&c).expect("its own vector");
        assert_eq!(
            pair.derivative_coeffs().len(),
            kv.control_count() - 1,
            "{name}"
        );
        assert!(!pair.derivative_coeffs().is_empty(), "{name}");
    }
}
