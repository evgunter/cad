//! `NurbsCurve2/3::on_domain` — the same curve on another parameter
//! domain: the domain is the requested pair bit for bit, the net and
//! the weights are the source's verbatim, the interior knots are the
//! affine image of a NON-unit source's, the locus is the source's
//! reparametrized, and the only refusal is the knot door's, named as
//! the domain's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve2, NurbsCurve3};
use geom_core::spline::{KnotVector, SplineError};
use geom_core::{Point2, Point3};

/// A degree-2 rational curve on `[1, 3]` with a double interior knot
/// and non-unit weights — every piece of structure the door has to
/// carry, on a source domain that is not the unit interval so the
/// pull-back `(k − a)/(b − a)` is exercised.
fn source() -> NurbsCurve2<f64> {
    let knots = KnotVector::clamped(vec![1.0, 1.0, 1.0, 1.5, 2.0, 2.0, 3.0, 3.0, 3.0], 2).unwrap();
    let control = vec![
        Point2::new(0.0, 0.0),
        Point2::new(0.3, 1.1),
        Point2::new(1.2, 0.9),
        Point2::new(1.7, -0.4),
        Point2::new(2.5, 0.2),
        Point2::new(3.0, 1.0),
    ];
    let weights = vec![1.0, 0.7, 1.3, 2.0, 0.9, 1.0];
    NurbsCurve2::new(knots, control, weights).unwrap()
}

/// The target: `0.3 + (0.9 − 0.3)` is an ulp above `0.9`, so a computed
/// end would miss it.
const TARGET: (f64, f64) = (0.3, 0.9);

#[test]
fn on_domain_pins_the_ends_carries_the_structure_and_reparametrizes() {
    let src = source();
    let (a, b) = src.domain();
    let (lo, hi) = TARGET;
    assert_eq!((lo + (hi - lo)).to_bits(), hi.to_bits() + 1);
    let moved = src.on_domain(lo, hi).unwrap();

    // Domain bit for bit; degree and counts unchanged.
    let (mlo, mhi) = moved.domain();
    assert_eq!((mlo.to_bits(), mhi.to_bits()), (lo.to_bits(), hi.to_bits()));
    assert_eq!(moved.degree(), src.degree());
    assert_eq!(moved.knots().knots().len(), src.knots().knots().len());

    // Interior knots: the affine image through the source's own
    // `[a, b]`; the double knot stays double.
    let span = hi - lo;
    let want: Vec<u64> = src
        .knots()
        .interior()
        .iter()
        .map(|k| (lo + span * ((k - a) / (b - a))).to_bits())
        .collect();
    let got: Vec<u64> = moved
        .knots()
        .interior()
        .iter()
        .map(|k| k.to_bits())
        .collect();
    assert_eq!(got, want);
    assert_eq!(got[1], got[2], "the double knot is still double");

    // Net and weights verbatim.
    let net = |c: &[Point2<f64>]| {
        c.iter()
            .map(|p| (p.x.to_bits(), p.y.to_bits()))
            .collect::<Vec<_>>()
    };
    assert_eq!(net(moved.control()), net(src.control()));
    assert_eq!(moved.weights(), src.weights());

    // The locus: the source reparametrized. Bit for bit at the ends
    // (exact end multiplicity makes every de Boor weight 0 or 1 there
    // on both sides), and to the evaluator's rounding at the dyadic
    // samples whose pull-back is exact on BOTH sides, so the two
    // evaluations are of one mathematical parameter.
    let bits = |p: Point2<f64>| (p.x.to_bits(), p.y.to_bits());
    assert_eq!(bits(moved.eval(lo)), bits(src.eval(a)));
    assert_eq!(bits(moved.eval(hi)), bits(src.eval(b)));
    let mut compared = 0_u32;
    for i in 1..32_u32 {
        let s = f64::from(i) / 32.0;
        let t = lo + span * s;
        let u = a + (b - a) * s;
        if (t - lo) / span != s || (u - a) / (b - a) != s {
            continue;
        }
        compared += 1;
        let (p, q) = (moved.eval(t), src.eval(u));
        assert!(
            (p.x - q.x).abs() < 1e-14 && (p.y - q.y).abs() < 1e-14,
            "s = {s}: {p:?} vs {q:?}"
        );
    }
    assert!(compared >= 8, "only {compared} of 31 pull-backs were exact");
}

/// The refusal is the knot door's, named as the domain's — carried
/// through unchanged, not re-wrapped — and a 3-D curve goes through the
/// same door.
#[test]
fn on_domain_refuses_as_the_domain_and_serves_both_dimensions() {
    let src = source();
    assert_eq!(
        src.on_domain(0.9, 0.3).unwrap_err(),
        SplineError::DomainInvalid { lo: 0.9, hi: 0.3 }
    );
    // The fragment, not the whole string: the rendering carries a
    // recourse clause behind the condition
    // (`every_spline_error_arm_names_a_recourse`), and a full-string
    // pin here would make that clause unwritable rather than checking
    // anything about the door.
    let msg = src.on_domain(0.5, f64::NAN).unwrap_err().to_string();
    assert!(
        msg.contains("the domain [0.5, NaN] is not a finite increasing interval of finite width"),
        "the domain door's own refusal is not what was rendered: {msg}"
    );

    let knots = KnotVector::clamped(vec![1.0, 1.0, 2.0, 3.0, 3.0], 1).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.5),
        Point3::new(2.0, 0.0, 1.0),
    ];
    let src3 = NurbsCurve3::new(knots, control.clone(), vec![1.0, 2.0, 1.0]).unwrap();
    let (lo, hi) = TARGET;
    let moved = src3.on_domain(lo, hi).unwrap();
    let (mlo, mhi) = moved.domain();
    assert_eq!((mlo.to_bits(), mhi.to_bits()), (lo.to_bits(), hi.to_bits()));
    assert_eq!(moved.knots().interior(), &[lo + (hi - lo) * 0.5]);
    let net3 = |c: &[Point3<f64>]| {
        c.iter()
            .map(|p| (p.x.to_bits(), p.y.to_bits(), p.z.to_bits()))
            .collect::<Vec<_>>()
    };
    assert_eq!(net3(moved.control()), net3(&control));
    assert_eq!(moved.weights(), src3.weights());
    assert_eq!(
        src3.on_domain(1.0, 1.0).unwrap_err(),
        SplineError::DomainInvalid { lo: 1.0, hi: 1.0 }
    );
}
