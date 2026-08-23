//! Adversarial e2e review artifact for M0 PR 4 (the `Interval` scalar,
//! 2026-07-15/16), salvaged from the M0 orchestrator session scratchpad
//! and promoted into CI 2026-07-16 (archived at
//! `references/review-artifacts-m0/pr4-review-demos/`). These are
//! **independent derivations** — do not "simplify" them to match shipped
//! fixtures; the independence is the regression value.
//!
//! Salvage adaptations:
//!
//! - The review's raw-inari decoration-monotonicity table needed a direct
//!   `inari` dependency (it probed inari itself, not this crate) and is
//!   dropped.
//! - The review's `libm_hunt` binary (400k-sample random hunts for libm
//!   results outside tight enclosures) is dropped: it was exploratory and
//!   assertion-free; the documented certification hazard it illustrated
//!   is pinned deterministically by
//!   `powi_f64_lane_is_contained_by_the_padded_enclosure` instead —
//!   which, since the backend swap, pins the INVERSE property: a
//!   pad-based backend contains its own `f64` lane op-for-op, so the
//!   divergence witness cannot exist. That row's own doc carries the
//!   inversion and what it does and does not guard.
//! - The review flagged that an EMPTY residual passed the documented
//!   `hi() <= eps` certification check (poison laundering). That hole was
//!   closed after the review: [`geom_core::Bounds`] now surfaces empty as
//!   NaN, indistinguishable from NaI. The certification test below
//!   asserts the *fixed* behavior.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
// The laundering table's closure type is the review artifact's shape; the
// negated `hi() <= eps` IS the certification check under test (its NaN
// behavior is the point); 0.7071... is a deliberate not-quite-exact seed.
#![allow(
    clippy::type_complexity,
    clippy::neg_cmp_op_on_partial_ord,
    clippy::approx_constant
)]

use geom_core::{
    Band, Bounds, Decide, Indeterminate, Interval, MarginDiag, Point2, Real, Sign, Vec2,
};

// ---------------------------------------------------------------- helpers

/// The review-run band, constructed purely (never the global tolerance).
fn band() -> Band {
    Band::new(1e-9, 1e-8).expect("valid band")
}

fn iv(lo: f64, hi: f64) -> Interval {
    Interval::from_bounds(lo, hi)
}

fn refuses_as_invalid(x: Interval) -> bool {
    matches!(
        x.sign_within(band()),
        Err(Indeterminate {
            margin: MarginDiag::Invalid,
            ..
        })
    )
}

// ------------------------------------------------- demo 1: the Q1 replay

/// A real evaluation pipeline, generic over `Real` alone: the point on the
/// unit circle centered at (1, 0.5) at angle t, classified against the
/// line y = x. Margin = signed distance = ((p - O)·n)/|n|, n = (-1, 1).
fn circle_line_margin<T: Real>(t: T) -> T {
    let c = Point2::new(T::from_f64(1.0), T::from_f64(0.5));
    let (s, co) = t.sin_cos();
    let p = c + Vec2::new(co, s);
    let n = Vec2::new(T::from_f64(-1.0), T::from_f64(1.0));
    let inv_len = T::one() / n.dot(n).sqrt();
    (p - Point2::origin()).dot(n) * inv_len
}

struct DriverStats {
    positive: usize,
    negative: usize,
    zero: usize,
    refused: Vec<(f64, f64, MarginDiag)>,
}

/// The hand-rolled Q1 subdivision driver: replay the SAME generic pipeline
/// at Interval over a parameter box, bisecting on Indeterminate.
fn subdivide(lo: f64, hi: f64, b: Band, depth: usize, stats: &mut DriverStats) {
    let t = Interval::from_bounds(lo, hi);
    match circle_line_margin(t).sign_within(b) {
        Ok(Sign::Positive) => stats.positive += 1,
        Ok(Sign::Negative) => stats.negative += 1,
        Ok(Sign::Zero) => stats.zero += 1,
        Err(ind) => {
            if depth == 0 {
                stats.refused.push((lo, hi, ind.margin));
                return;
            }
            let mid = 0.5 * (lo + hi);
            subdivide(lo, mid, b, depth - 1, stats);
            subdivide(mid, hi, b, depth - 1, stats);
        }
    }
}

#[test]
fn q1_replay_loop_in_miniature() {
    let b = band();

    // Step 1: f64 evaluation + decision at a healthy point.
    assert_eq!(
        circle_line_margin(2.0f64).sign_within(b),
        Ok(Sign::Positive)
    );

    // Step 2: replay the SAME code at Interval over the box [1, 3], which
    // contains the true root t* ~= 1.1468. Subdivide on refusal.
    let mut stats = DriverStats {
        positive: 0,
        negative: 0,
        zero: 0,
        refused: vec![],
    };
    subdivide(1.0, 3.0, b, 40, &mut stats);
    // The partition must contain both signs (root inside), and the
    // refused boxes must be tiny (only the true near-root slivers).
    assert!(
        stats.positive > 0 && stats.negative > 0,
        "expected both definite signs across the root: {} Positive, {} Negative",
        stats.positive,
        stats.negative
    );
    for (lo, hi, margin) in &stats.refused {
        assert!(
            hi - lo <= 1e-6,
            "refused box [{lo:e}, {hi:e}] is not a sliver ({margin:?}) — driver did not converge"
        );
    }

    // Consistency: every decided sub-box agrees with an f64 sample inside
    // it (spot-check midpoints of a fresh coarse pass).
    for k in 0..64u32 {
        let lo = 1.0 + 2.0 * (f64::from(k) / 64.0);
        let hi = 1.0 + 2.0 * (f64::from(k + 1) / 64.0);
        if let Ok(sign) = circle_line_margin(Interval::from_bounds(lo, hi)).sign_within(b) {
            let f = circle_line_margin(0.5 * (lo + hi)).sign_within(b);
            assert_eq!(
                f,
                Ok(sign),
                "interval box [{lo},{hi}] decided {sign:?} but the f64 midpoint disagrees"
            );
        }
    }
}

// ------------------------------------------- demo 2: poison-channel abuse

/// Poison produced mid-stream in ordinary pipelines must refuse to
/// classify, and no arithmetic laundering attempt may wash it clean.
#[test]
fn poison_laundering_hunt() {
    // Poison sources.
    let sources: Vec<(&str, Interval)> = vec![
        (
            "sqrt of straddling [-1,4] (clamped, Trv)",
            iv(-1.0, 4.0).sqrt(),
        ),
        (
            "sqrt of fully-negative [-4,-1] (empty)",
            iv(-4.0, -1.0).sqrt(),
        ),
        ("asin of [0.5, 1.5] (clamped, Trv)", iv(0.5, 1.5).asin()),
        ("asin of [1.5, 2] (empty)", iv(1.5, 2.0).asin()),
        (
            "atan2 of origin-containing box",
            Real::atan2(iv(-1.0, 1.0), iv(-1.0, 1.0)),
        ),
        (
            "atan2 exactly at origin point",
            Real::atan2(Interval::from_f64(0.0), Interval::from_f64(0.0)),
        ),
        ("tan across the pole [1.5, 1.6]", iv(1.5, 1.6).tan()),
        (
            "division by zero-straddling [1,2]/[-1,1]",
            iv(1.0, 2.0) / iv(-1.0, 1.0),
        ),
        ("NaI from from_f64(NaN)", Interval::from_f64(f64::NAN)),
        ("NaI from reversed from_bounds(2,1)", iv(2.0, 1.0)),
    ];

    let healthy = Interval::from_f64(3.0);
    // Laundering attempts: operations that might plausibly reset the
    // decoration or collapse the value to something classifiable.
    let attempts: Vec<(&str, Box<dyn Fn(Interval) -> Interval>)> = vec![
        ("identity", Box::new(|p| p)),
        ("p * [0,0]", Box::new(|p| p * Interval::zero())),
        ("[0,0] * p", Box::new(|p| Interval::zero() * p)),
        ("p.powi(0)", Box::new(|p| p.powi(0))),
        ("p.powi(2)", Box::new(|p| p.powi(2))),
        ("p - p", Box::new(|p| p - p)),
        ("p / p", Box::new(|p| p / p)),
        ("abs(p)", Box::new(|p| p.abs())),
        ("sin(p) (bounded range)", Box::new(|p| p.sin())),
        ("atan(p) (total op)", Box::new(|p| p.atan())),
        (
            "min(p, healthy) with healthy smaller",
            Box::new(move |p| Real::min(p, Interval::from_f64(-7.0))),
        ),
        (
            "max(p, healthy) with healthy larger",
            Box::new(move |p| Real::max(p, Interval::from_f64(7.0))),
        ),
        (
            "min(healthy, p)",
            Box::new(move |p| Real::min(Interval::from_f64(-7.0), p)),
        ),
        (
            "healthy + [0,0]*p",
            Box::new(move |p| Interval::from_f64(3.0) + Interval::zero() * p),
        ),
        (
            "(p*p + h*h).sqrt() hypot-style",
            Box::new(move |p| (p * p + healthy * healthy).sqrt()),
        ),
        (
            "sqrt(abs(p)) re-entering a partial domain",
            Box::new(|p| p.abs().sqrt()),
        ),
    ];

    for (sname, p) in &sources {
        // The source itself must refuse.
        assert!(
            refuses_as_invalid(*p),
            "source '{sname}' did not refuse: bounds [{:e}, {:e}] -> {:?}",
            p.lo(),
            p.hi(),
            p.sign_within(band())
        );
        for (aname, f) in &attempts {
            let r = f(*p);
            // Push the value into decisively-positive territory; if the
            // poison were laundered, this WOULD classify Positive.
            let probe = r.abs() + Interval::from_f64(100.0);
            assert!(
                refuses_as_invalid(probe),
                "LAUNDERING: source '{sname}', attempt '{aname}': result [{:e}, {:e}], \
                 probe [{:e}, {:e}] -> {:?}",
                r.lo(),
                r.hi(),
                probe.lo(),
                probe.hi(),
                probe.sign_within(band())
            );
        }
    }
}

// -------------------------------------- demo 3: Bounds and certification

#[test]
fn residual_certification_and_the_closed_poison_hole() {
    let eps = 1e-9;

    // Healthy residual certification: |hypot(3,4) - 5| at interval type.
    let h = (Interval::from_f64(3.0).powi(2) + Interval::from_f64(4.0).powi(2)).sqrt();
    let residual = (h - Interval::from_f64(5.0)).abs();
    assert!(
        residual.hi() <= eps,
        "exact-case residual must certify: [{:e}, {:e}]",
        residual.lo(),
        residual.hi()
    );

    // Transcendental residual: |sin^2 x + cos^2 x - 1| certifies at every x.
    let x = Interval::from_f64(123.456);
    let (s, c) = x.sin_cos();
    let r2 = (s * s + c * c - Interval::one()).abs();
    assert!(
        r2.hi() <= eps,
        "|sin2+cos2-1| must certify: [{:e}, {:e}]",
        r2.lo(),
        r2.hi()
    );

    // THE HOLE PROBE (found open at review, closed since): a residual
    // pipeline poisoned to EMPTY mid-stream must NOT pass the documented
    // `hi() <= eps` certification. Bounds now surfaces empty as NaN —
    // indistinguishable from NaI, failing every certification comparison.
    let poisoned_computed = Interval::from_f64(-1.0).sqrt(); // empty
    let poisoned_residual = (poisoned_computed - Interval::from_f64(0.0)).abs();
    assert!(
        poisoned_residual.lo().is_nan() && poisoned_residual.hi().is_nan(),
        "empty residual must surface as NaN through Bounds, got [{:e}, {:e}]",
        poisoned_residual.lo(),
        poisoned_residual.hi()
    );
    assert!(
        !(poisoned_residual.hi() <= eps),
        "EMPTY RESIDUAL PASSES CERTIFICATION — the review hole reopened"
    );

    // NaI residual fails certification the same way.
    let nai_residual = (Interval::from_f64(f64::NAN) - Interval::from_f64(0.0)).abs();
    assert!(nai_residual.lo().is_nan() && nai_residual.hi().is_nan());
    assert!(
        !(nai_residual.hi() <= eps),
        "NaI RESIDUAL PASSES CERTIFICATION"
    );
}

// --------------------------------- demo 4: cross-instantiation consistency

#[test]
fn cross_instantiation_consistency_on_exact_and_single_ops() {
    // Exact cases: +, *, sqrt where the true result is representable —
    // the interval result must be the f64 point, bitwise.
    let cases: Vec<(&str, f64, Interval)> = vec![
        (
            "0.5 + 0.25",
            0.5 + 0.25,
            Interval::from_f64(0.5) + Interval::from_f64(0.25),
        ),
        (
            "3 * 4",
            12.0,
            Interval::from_f64(3.0) * Interval::from_f64(4.0),
        ),
        ("sqrt(25)", 5.0, Interval::from_f64(25.0).sqrt()),
        (
            "hypot(3,4) pipeline",
            {
                fn hyp<T: Real>(a: T, b: T) -> T {
                    (a * a + b * b).sqrt()
                }
                hyp(3.0f64, 4.0f64)
            },
            {
                fn hyp<T: Real>(a: T, b: T) -> T {
                    (a * a + b * b).sqrt()
                }
                hyp(Interval::from_f64(3.0), Interval::from_f64(4.0))
            },
        ),
    ];
    for (name, f, e) in &cases {
        assert!(
            e.lo().to_bits() == f.to_bits() && e.hi().to_bits() == f.to_bits(),
            "{name}: exact case must agree bitwise: f64 = {f:e}, interval = [{:e}, {:e}]",
            e.lo(),
            e.hi()
        );
    }

    // Inexact single ops (division is correctly rounded at f64): the f64
    // result is contained and the enclosure is at most 2 ulps wide.
    //
    // Backend change (M5 PR 1, interval-transcendentals semantics-diffs
    // D1): the budget was 1 ulp while the backend was inari, whose
    // arithmetic is directed-rounded. The present backend rounds to
    // nearest and pads each inexact endpoint one step outward, so an
    // inexact quotient is 2 ulps wide. The *exact* cases above are
    // unaffected — the backend witnesses exactness for +, *, / and sqrt
    // and takes no pad when it holds.
    for (a, b) in [(0.1, 0.2), (1.0, 3.0), (2.0, 7.0)] {
        let f = a / b;
        let e = Interval::from_f64(a) / Interval::from_f64(b);
        assert!(
            f >= e.lo() && f <= e.hi() && e.hi() <= next_up(next_up(e.lo())),
            "{a}/{b}: contained-and-2ulp failed: f64 = {f:e}, interval = [{:e}, {:e}]",
            e.lo(),
            e.hi()
        );
    }
}

fn next_up(x: f64) -> f64 {
    f64::from_bits(x.to_bits() + 1)
}

/// **Inverted by the M5 PR 1 backend swap — read the whole comment.**
///
/// Under the original inari backend this test was `powi_diverges_from_
/// the_tight_enclosure`: it asserted that the SAME trait call `x.powi(n)`
/// at `f64` (exponentiation by squaring, several roundings) can land
/// OUTSIDE the interval instantiation's enclosure, and it existed to pin
/// the certification-semantics rule that enclosures bracket the TRUE
/// value, not the `f64` evaluation of the same expression. inari's
/// `pown` is correctly rounded (MPFR), so its enclosure of `x^n` was
/// ~1 ulp wide while the `f64` squaring chain drifted several ulps out
/// of it — divergence witnesses were easy to find.
///
/// The present backend computes `powi` by the same square-and-multiply
/// shape, padding one step outward at every step, which makes the `f64`
/// chain contained INDUCTIVELY: each interval step's pad already covers
/// the corresponding `f64` step's rounding. A witness therefore cannot
/// exist any more, at any exponent — verified by a scan over
/// `n ∈ {2 … 200}` across the same `x` range, which found zero.
///
/// So the test now pins the property that actually holds, and names why:
/// a pad-based backend *contains its own f64 lane* op-for-op, which the
/// MPFR-tight backend did not. The certification rule the old test
/// defended is unchanged and still binding — certification code bounds
/// residuals at interval type and never asserts `f64 ∈ enclosure` — but
/// it is now a discipline that does not depend on a tightness accident.
/// **What this test is and is not** (M5 PR 1 review, MINOR-1). It pins
/// CONTAINMENT: it fails if the `f64` lane ever escapes the enclosure.
/// It is **not** a dropped-pad tripwire, and must not be relied on as
/// one — the review demonstrated exactly that by mutating the backend's
/// `pow_mag_hi` to drop both of its outward rounds, at which point this
/// test still passed, because the `f64` lane shares the same
/// round-to-nearest multiply chain and moves with the mutation. The
/// crate's own lanes caught that mutation immediately: `certify.rs`
/// (differential, against MPFR) and `edges.rs`, plus
/// `review_fuzz_exact.rs` for the division, multiplication and sqrt
/// witnesses and `pad_contract.rs` for the pads' upper bound. Those are
/// the pad tripwires; this is a lane-agreement pin.
#[test]
fn powi_f64_lane_is_contained_by_the_padded_enclosure() {
    let mut checked = 0u32;
    for n in [2i32, 3, 5, 6, 7, 9, 12, 17, 23, 31, 40] {
        let mut x = 1.0001f64;
        while x < 3.0 {
            let f = <f64 as Real>::powi(x, n);
            let e = Interval::from_f64(x).powi(n);
            assert!(
                f >= e.lo() && f <= e.hi(),
                "x={x} n={n}: f64 powi {f:e} escaped the enclosure \
                 [{:e}, {:e}] — the two lanes have diverged (for a \
                 dropped pad, see the crate's certify/edges lanes)",
                e.lo(),
                e.hi()
            );
            checked += 1;
            x += 0.000037;
        }
    }
    assert!(checked > 100_000, "the scan must actually cover ground");
}

// ------------------------------------------------- demo 5: determinism

fn pipeline(seed: f64) -> Interval {
    let x = Interval::from_f64(seed);
    let (s, c) = (x * Interval::pi() + Interval::one()).sin_cos();
    let h = (s * s + c * c).sqrt();
    let m = circle_line_margin(x.abs() + Interval::from_f64(0.25));
    Real::atan2(h + m, x.abs() + Interval::from_f64(0.5)).powi(3) / Interval::from_f64(7.0)
}

/// In-process determinism: bit-identical bounds on every re-evaluation.
/// (The review also diffed the printout across processes and runs; the
/// in-process re-evaluation is the CI-expressible half.)
#[test]
fn pipeline_bounds_are_bit_identical_across_reruns() {
    for seed in [0.0, 1.5e-9, -3.25, 1234.5, 0.7071067811865476] {
        let first = pipeline(seed);
        let key = (first.lo().to_bits(), first.hi().to_bits());
        for _ in 0..8 {
            let again = pipeline(seed);
            assert_eq!(
                (again.lo().to_bits(), again.hi().to_bits()),
                key,
                "in-process nondeterminism at seed {seed}"
            );
        }
    }
}
