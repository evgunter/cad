//! R1 adversarial review probes for PR #306 (rational span meter).
//! Soundness: for every constructible rational carrier, the meter must
//! never exceed the densely sampled true minimum speed; positive claims
//! must be genuinely positive; the refusal boundary must track the real
//! speed minimum, not a chord artifact.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::neg_cmp_op_on_partial_ord
)]

use geom_core::Point3;
use geom_core::spline::KnotVector;
use geom_curves::NurbsCurve3;

/// Densely sample the true speed over the curve's own domain (uniform
/// plus a golden-ratio jitter pass, `n` samples each) and return the
/// observed minimum. `n >= 4001` in every caller.
fn sampled_min_speed(c: &NurbsCurve3<f64>, n: usize) -> f64 {
    let (t0, t1) = c.domain();
    let mut lo = f64::INFINITY;
    for i in 0..=n {
        #[allow(clippy::cast_precision_loss)]
        let f = i as f64 / n as f64;
        for t in [
            t0 + (t1 - t0) * f,
            t0 + (t1 - t0) * ((f + 0.618_033_988_749) % 1.0),
        ] {
            let s = c.deriv(t).norm();
            if s < lo {
                lo = s;
            }
        }
    }
    lo
}

/// The soundness invariant: bound <= sampled true min (relative slack
/// for f64 rounding only). Panics with the worst ratio on violation.
/// Returns (meter, sampled_min).
fn assert_sound(name: &str, c: &NurbsCurve3<f64>, n: usize) -> (f64, f64) {
    let m = c.speed_lower_bound();
    let lo = sampled_min_speed(c, n);
    if m.is_nan() {
        return (m, lo);
    }
    let slack = 1e-9 * lo.abs().max(1.0) + 1e-12;
    assert!(
        m <= lo + slack,
        "{name}: UNSOUND — meter {m} exceeds sampled true min {lo} (ratio {})",
        m / lo
    );
    (m, lo)
}

fn quad(mid: Point3<f64>, w: [f64; 3]) -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    NurbsCurve3::new(
        kv,
        vec![Point3::new(0.0, 0.0, 0.0), mid, Point3::new(1.0, 0.0, 0.0)],
        w.to_vec(),
    )
    .unwrap()
}

#[test]
fn extreme_weight_ratios_stay_sound() {
    let mut worst: (f64, String) = (0.0, String::new());
    for w1 in [1e-6, 1e-4, 1e-2, 1.0 + 1e-12, 1e2, 1e4, 1e6] {
        for w0 in [1e-3, 1.0, 1e3] {
            let name = format!("quad w0={w0} w1={w1}");
            let c = quad(Point3::new(0.5, 0.4, 0.1), [w0, w1, w0]);
            let (m, lo) = assert_sound(&name, &c, 4001);
            if m.is_finite() && lo > 0.0 && m / lo > worst.0 {
                worst = (m / lo, name);
            }
        }
    }
    // Mixed extreme weights on INTERIOR control points, degree 4.
    let kv =
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0], 4).unwrap();
    for w in [
        [1.0, 1e-6, 1e6, 1e-6, 1.0],
        [1.0, 1e6, 1e-6, 1e6, 1.0],
        [1e-5, 1.0, 1e5, 1.0, 1e-5],
        [1.0, 1e-3, 1.0, 1e3, 1.0],
    ] {
        let c = NurbsCurve3::<f64>::new(
            kv.clone(),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.3, 0.2, -0.1),
                Point3::new(0.5, -0.2, 0.1),
                Point3::new(0.7, 0.2, 0.0),
                Point3::new(1.0, 0.0, 0.0),
            ],
            w.to_vec(),
        )
        .unwrap();
        assert_sound(&format!("deg4 w={w:?}"), &c, 6001);
    }
    eprintln!(
        "worst positive ratio bound/true_min: {} ({})",
        worst.0, worst.1
    );
    assert!(worst.0 <= 1.0 + 1e-9, "ratio above 1 is unsound");
}

#[test]
fn near_cusp_families_never_cross_the_truth() {
    // A repeated-interior-point cusp, opened by eps: as eps -> 0 the
    // true min speed -> 0. The meter must never cross the truth, and
    // where it claims positive the truth must be positive.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    for eps in [1.0, 0.3, 0.1, 0.03, 0.01, 1e-3, 1e-4, 1e-6, 0.0] {
        let c = NurbsCurve3::<f64>::new(
            kv.clone(),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(1.0 + eps, 1.0 + 0.5 * eps, 0.0),
                Point3::new(2.0, 0.2, 0.0),
            ],
            vec![1.0, 0.6, 2.0, 1.0],
        )
        .unwrap();
        let (m, lo) = assert_sound(&format!("near-cusp eps={eps}"), &c, 8001);
        if m > 0.0 {
            assert!(
                lo > 0.0,
                "eps={eps}: positive meter {m} but sampled min {lo}"
            );
        }
        eprintln!("eps={eps}: meter={m}, sampled_min={lo}");
    }
    // Approach from the true turn-around side: end returns to start,
    // then pulled apart by eps.
    let kv2 = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    for eps in [0.5, 0.1, 1e-2, 1e-4, 0.0] {
        let c = NurbsCurve3::<f64>::new(
            kv2.clone(),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.5, 1.0, 0.0),
                Point3::new(eps, eps, 0.0),
            ],
            vec![1.0, 0.5, 1.0],
        )
        .unwrap();
        assert_sound(&format!("turnaround eps={eps}"), &c, 6001);
    }
}

#[test]
fn structure_extremes_stay_sound() {
    // Tiny knot interval: span length 1e-12 (huge speeds).
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1e-12, 1e-12, 1e-12], 2).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.5, 0.3, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
        vec![1.0, 3.0, 1.0],
    )
    .unwrap();
    assert_sound("tiny-span", &c, 4001);

    // Mixed span lengths: 1e-9 next to 1.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1e-9, 1.0, 1.0, 1.0], 2).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1e-9, 1e-9, 0.0),
            Point3::new(0.5, 0.2, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
        vec![1.0, 2.0, 0.5, 1.0],
    )
    .unwrap();
    assert_sound("mixed-span-lengths", &c, 8001);

    // Control points at large offset (1e8) — the centering term's job.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let off = 1e8;
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(off, off, off),
            Point3::new(off + 0.5, off + 0.3, off),
            Point3::new(off + 1.0, off, off),
        ],
        vec![1.0, 0.5, 1.0],
    )
    .unwrap();
    let (m, lo) = assert_sound("offset-1e8", &c, 4001);
    eprintln!("offset-1e8: meter={m}, sampled_min={lo}");

    // High degree (7), single span, rough weights.
    let kv = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ],
        7,
    )
    .unwrap();
    let pts: Vec<Point3<f64>> = (0..8)
        .map(|i| {
            let t = f64::from(i) / 7.0;
            Point3::new(t, 0.2 * (t * 9.0).sin(), 0.1 * (t * 5.0).cos())
        })
        .collect();
    let c = NurbsCurve3::<f64>::new(kv, pts, vec![1.0, 0.4, 2.5, 0.7, 1.8, 0.3, 2.0, 1.0]).unwrap();
    assert_sound("degree-7", &c, 8001);
}

#[test]
fn randomized_rational_nets_are_sound() {
    // Deterministic LCG fuzz: 200 curves, degree 2..=4, 1..=3 spans,
    // log-uniform weights in [1e-4, 1e4].
    let mut state: u64 = 0x243F_6A88_85A3_08D3;
    let mut next = move || {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)]
        let f = ((state >> 11) as f64) / ((1u64 << 53) as f64);
        f
    };
    for case in 0..200 {
        let p = 2 + (next() * 3.0) as usize; // 2..=4
        let spans = 1 + (next() * 3.0) as usize; // 1..=3
        let mut interior: Vec<f64> = (1..spans)
            .map(|i| {
                f64::from(u32::try_from(i).unwrap()) / f64::from(u32::try_from(spans).unwrap())
            })
            .collect();
        let mut kv = vec![0.0; p + 1];
        kv.append(&mut interior);
        kv.extend(std::iter::repeat_n(1.0, p + 1));
        let kv = KnotVector::clamped(kv, p).unwrap();
        let n_ctrl = p + spans;
        let pts: Vec<Point3<f64>> = (0..n_ctrl)
            .map(|_| Point3::new(next() * 4.0 - 2.0, next() * 4.0 - 2.0, next() * 4.0 - 2.0))
            .collect();
        let ws: Vec<f64> = (0..n_ctrl)
            .map(|_| 10f64.powf(next() * 8.0 - 4.0))
            .collect();
        let c = NurbsCurve3::<f64>::new(kv, pts, ws).unwrap();
        assert_sound(&format!("fuzz case {case}"), &c, 4001);
    }
}

#[test]
fn a_regular_carrier_that_returns_near_its_start_meters_positively() {
    // The deviation's blast radius (claim 3): a horseshoe that comes
    // back within 0.05 of its start, speed never collapsing. The
    // per-span arm should meter it positively — reversal is not
    // disqualifying under the ratified contract — and soundly.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0], 2).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.2, 0.0),
            Point3::new(2.0, 1.0, 0.0),
            Point3::new(1.0, 1.8, 0.0),
            Point3::new(0.0, 2.0, 0.0),
            Point3::new(0.02, 0.05, 0.0),
        ],
        vec![1.0, 0.8, 1.5, 0.8, 1.2, 1.0],
    )
    .unwrap();
    let (m, lo) = assert_sound("horseshoe", &c, 8001);
    eprintln!("horseshoe: meter={m}, sampled_min={lo}");
    assert!(
        m > 0.0,
        "the ratified contract says reversal alone must not refuse; meter = {m}"
    );
}

#[test]
fn the_refusal_rows_truth_measured() {
    // The PR's "cusp" row: is the speed collapse genuine?
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 0.2, 0.0),
        ],
        vec![1.0, 0.6, 2.0, 1.0],
    )
    .unwrap();
    let m = c.speed_lower_bound();
    let lo = sampled_min_speed(&c, 20001);
    eprintln!("PR cusp row: meter={m}, true sampled min speed={lo}");
    // The PR's turn-around row.
    let kv2 = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let c2 = NurbsCurve3::<f64>::new(
        kv2,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.5, 1.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        ],
        vec![1.0, 0.5, 1.0],
    )
    .unwrap();
    let m2 = c2.speed_lower_bound();
    let lo2 = sampled_min_speed(&c2, 20001);
    eprintln!("PR turnaround row: meter={m2}, true sampled min speed={lo2}");
    // Refusal-boundary tracking: pull the repeated point apart along
    // the EXIT leg direction so the dip relaxes; find where the meter
    // turns positive and verify truth is comfortably positive there.
    for eps in [0.0, 0.05, 0.1, 0.2, 0.4, 0.8] {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let c = NurbsCurve3::<f64>::new(
            kv,
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(1.0 - eps, 1.0 - 0.8 * eps, 0.0),
                Point3::new(0.0, 0.2, 0.0),
            ],
            vec![1.0, 0.6, 2.0, 1.0],
        )
        .unwrap();
        let m = c.speed_lower_bound();
        let lo = sampled_min_speed(&c, 20001);
        assert!(
            m.is_nan() || m <= lo + 1e-9,
            "eps={eps} unsound: {m} > {lo}"
        );
        eprintln!("boundary eps={eps}: meter={m}, true min={lo}");
    }
}

/// Interval-backend spot check: the Interval instantiation's meter
/// must bracket (contain) the f64 meter for the same rational carrier.
#[cfg(feature = "interval")]
#[test]
fn interval_meter_brackets_f64_meter() {
    use geom_core::{Bounds, Interval};
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let pts_f = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.5, 0.3, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    ];
    let w = vec![1.0, 0.05, 1.0];
    let cf = NurbsCurve3::<f64>::new(kv.clone(), pts_f, w.clone()).unwrap();
    let mf = cf.speed_lower_bound();
    let pi = |x: f64, y: f64, z: f64| {
        Point3::new(
            Interval::from_bounds(x, x),
            Interval::from_bounds(y, y),
            Interval::from_bounds(z, z),
        )
    };
    let ci = NurbsCurve3::<Interval>::new(
        kv,
        vec![pi(0.0, 0.0, 0.0), pi(0.5, 0.3, 0.0), pi(1.0, 0.0, 0.0)],
        w,
    )
    .unwrap();
    let mi = ci.speed_lower_bound();
    eprintln!("f64 meter={mf}, interval meter=[{}, {}]", mi.lo(), mi.hi());
    assert!(
        mi.lo() <= mf && mf <= mi.hi(),
        "interval meter [{}, {}] does not bracket f64 meter {mf}",
        mi.lo(),
        mi.hi()
    );
    assert!(mf > 0.0, "spot case should be a positive-meter case");
}

#[test]
fn a_malformed_net_poisons_the_rational_arm() {
    // Non-positive weight is refused at construction (the PR's own
    // row); a NaN control coordinate is the malformed net that CAN be
    // built, and the rational arm must propagate poison, never a
    // fabricated bound.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(f64::NAN, 0.3, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
        vec![1.0, 0.5, 1.0],
    )
    .unwrap();
    assert!(
        c.speed_lower_bound().is_nan(),
        "a NaN control point must poison the meter"
    );
}
