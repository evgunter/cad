//! D290 R1 review probes (local-only signal): the end-to-end exercise of
//! the `on_domain` door from a caller's seat, the `offset_fit` knot
//! digest for the merge-base differential, and a measurement of what the
//! pinned ends buy a caller that evaluates at the carrier's own `t1`.

use geom::{NurbsCurve2, NurbsSurface};
use geom_brep::offset_fit::fit_offset_at;
use geom_core::Point2;
use geom_core::spline::KnotVector;

use crate::shared::fixture::quarter_cylinder;
use crate::shared::tol::band;

/// The old `offset_fit::rescaled_knots` spelling (pinned ends, interior
/// `lo + span * k`), kept here so the SAME base surface is built at the
/// merge base and at the head — the digest row below compares the fit's
/// knots, not this helper.
fn pinned_rescale(kv: &KnotVector, lo: f64, hi: f64) -> KnotVector {
    let span = hi - lo;
    let n = kv.knots().len();
    let p = kv.degree();
    let scaled: Vec<f64> = kv
        .knots()
        .iter()
        .enumerate()
        .map(|(i, t)| {
            if i <= p {
                lo
            } else if i + p + 1 >= n {
                hi
            } else {
                lo + span * *t
            }
        })
        .collect();
    KnotVector::clamped(scaled, p).unwrap()
}

fn bits(v: &[f64]) -> Vec<String> {
    v.iter().map(|k| format!("{:016x}", k.to_bits())).collect()
}

/// The PR body's uncommitted digest, reproduced: `quarter_cylinder(1.25,
/// 0.75)` rebuilt on `[0.3, 0.9] × [0.2, 1.7]`, `fit_offset_at(d = 0.3,
/// tol = 3e-4)`, both knot vectors printed as bits. Run with
/// `--nocapture` at the merge base and at the head; the two outputs
/// must be identical.
#[test]
fn d290_offset_fit_knot_digest() {
    let base = quarter_cylinder(1.25, 0.75);
    let ku = pinned_rescale(base.knots_u(), 0.3, 0.9);
    let kv = pinned_rescale(base.knots_v(), 0.2, 1.7);
    let base =
        NurbsSurface::new(ku, kv, base.control().to_vec(), base.weights().to_vec()).unwrap();
    let (fit, _) = fit_offset_at(&base, 0.3, 3e-4, band()).unwrap();
    println!(
        "D290-DIGEST u ({}) {}",
        fit.knots_u().knots().len(),
        bits(fit.knots_u().knots()).join(" ")
    );
    println!(
        "D290-DIGEST v ({}) {}",
        fit.knots_v().knots().len(),
        bits(fit.knots_v().knots()).join(" ")
    );
    println!(
        "D290-DIGEST domain_u {:?} domain_v {:?}",
        fit.knots_u().domain(),
        fit.knots_v().domain()
    );
}

/// A fitted planar curve, re-expressed on a carrier interval from a
/// caller's seat, evaluated: the domain is the carrier's bit for bit,
/// the locus is unchanged (eval at `lo + (hi − lo)·s` vs the fit at
/// `s`), and the end points are the interpolated end points EXACTLY at
/// the carrier's own ends — which is what a computed end misses.
#[test]
fn d290_e2e_fitted_curve_on_a_carrier_interval() {
    let pts: Vec<Point2<f64>> = (0..9)
        .map(|i| {
            let x = f64::from(i) * 0.37;
            Point2::new(x, (1.3 * x).sin() + 0.2 * x)
        })
        .collect();
    let fit = NurbsCurve2::<f64>::interpolate(&pts, 3).unwrap();
    assert_eq!(fit.domain(), (0.0, 1.0));
    // A carrier interval whose computed end misses hi by an ulp.
    let (t0, t1) = (0.3_f64, 0.9_f64);
    assert_eq!((t0 + (t1 - t0)).to_bits(), t1.to_bits() + 1);
    let on = fit.on_domain(t0, t1).unwrap();
    assert_eq!(
        (on.domain().0.to_bits(), on.domain().1.to_bits()),
        (t0.to_bits(), t1.to_bits())
    );
    assert_eq!(on.degree(), fit.degree());
    assert_eq!(on.control().len(), fit.control().len());
    // The locus is unchanged, up to the knot map's rounding.
    let mut worst = 0.0_f64;
    for i in 0..=64 {
        let s = f64::from(i) / 64.0;
        let p = fit.eval(s);
        let q = on.eval(t0 + (t1 - t0) * s);
        worst = worst.max((p.x - q.x).abs().max((p.y - q.y).abs()));
    }
    println!("D290-E2E worst locus drift over 65 samples: {worst:e}");
    assert!(worst < 1e-12, "locus moved: {worst:e}");
    // At the carrier's own ends the curve IS the interpolated end points
    // (clamped ends, exact domain).
    let (a, b) = (on.eval(t0), on.eval(t1));
    let (pa, pb) = (pts[0], pts[8]);
    println!(
        "D290-E2E end residuals: {:e} {:e}",
        (a.x - pa.x).abs().max((a.y - pa.y).abs()),
        (b.x - pb.x).abs().max((b.y - pb.y).abs())
    );

    // Contrast: the pre-D290 computed-end spelling on the same pair —
    // its domain is NOT the carrier's, and evaluating at the carrier's
    // `t1` reads the last span an ulp short of its end.
    let computed: Vec<f64> = fit
        .knots()
        .knots()
        .iter()
        .map(|k| t0 + (t1 - t0) * k)
        .collect();
    let computed = KnotVector::clamped(computed, fit.degree()).unwrap();
    let old = NurbsCurve2::new(computed, fit.control().to_vec(), fit.weights().to_vec()).unwrap();
    println!(
        "D290-E2E computed-end domain: {:?} (carrier {:?}); hi bits differ by {}",
        old.domain(),
        (t0, t1),
        old.domain().1.to_bits() as i64 - t1.to_bits() as i64
    );
    let ob = old.eval(t1);
    println!(
        "D290-E2E computed-end residual at the carrier's t1: {:e}",
        (ob.x - pb.x).abs().max((ob.y - pb.y).abs())
    );
    assert_ne!(old.domain().1.to_bits(), t1.to_bits());

    // Round trip: back onto [0, 1] — is the door an involution on the
    // interior knots? (Ergonomics measurement, not a claim of the unit.)
    let back = on.on_domain(0.0, 1.0).unwrap();
    let drift: Vec<i64> = back
        .knots()
        .knots()
        .iter()
        .zip(fit.knots().knots())
        .map(|(x, y)| x.to_bits() as i64 - y.to_bits() as i64)
        .collect();
    println!("D290-E2E round-trip knot drift in ulps: {drift:?}");

    // A NON-unit source: the right half of the split, on its own domain
    // `[u, 1]`, re-expressed onto the carrier, must still reproduce the
    // locus at the affine image of its parameters.
    let (_, right) = fit.split_at(0.4).unwrap();
    let (a, b) = right.domain();
    let on_r = right.on_domain(t0, t1).unwrap();
    let mut worst = 0.0_f64;
    for i in 0..=32 {
        let s = f64::from(i) / 32.0;
        let p = right.eval(a + (b - a) * s);
        let q = on_r.eval(t0 + (t1 - t0) * s);
        worst = worst.max((p.x - q.x).abs().max((p.y - q.y).abs()));
    }
    println!("D290-E2E non-unit source worst locus drift: {worst:e}");
    assert!(worst < 1e-12, "non-unit source moved: {worst:e}");
}

/// The door refuses a degenerate carrier as the DOMAIN's defect, and a
/// tiny carrier at a large magnitude refuses through the clamp clauses
/// instead of minting — the two refusal shapes a caller meets.
#[test]
fn d290_e2e_refusals_from_a_callers_seat() {
    use geom_core::spline::{KnotVectorIssue, SplineError};
    let pts: Vec<Point2<f64>> = (0..6)
        .map(|i| Point2::new(f64::from(i), f64::from(i * i)))
        .collect();
    let fit = NurbsCurve2::<f64>::interpolate(&pts, 2).unwrap();
    for (lo, hi) in [(1.0, 1.0), (2.0, 1.0), (f64::NAN, 1.0), (0.0, f64::INFINITY)] {
        let e = fit.on_domain(lo, hi).unwrap_err();
        assert!(
            matches!(
                e,
                SplineError::KnotVectorInvalid {
                    reason: KnotVectorIssue::DomainInvalid { .. }
                }
            ),
            "{lo} {hi}: {e}"
        );
        println!("D290-E2E refusal ({lo}, {hi}): {e}");
    }
    // A carrier one ulp wide: every interior image collapses.
    let lo = 1.0_f64;
    let hi = f64::from_bits(lo.to_bits() + 1);
    let e = fit.on_domain(lo, hi).unwrap_err();
    println!("D290-E2E one-ulp carrier: {e}");
    assert!(!matches!(
        e,
        SplineError::KnotVectorInvalid {
            reason: KnotVectorIssue::DomainInvalid { .. }
        }
    ));
}
