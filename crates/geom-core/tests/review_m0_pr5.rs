//! Adversarial e2e review artifact for M0 PR 5 (`Dual<T>`, 2026-07-15/16),
//! salvaged from the M0 orchestrator session scratchpad and promoted into
//! CI 2026-07-16 (archived at
//! `references/review-artifacts-m0/pr5-review-demos/`). These are
//! **independent derivations** — do not "simplify" them to match shipped
//! fixtures; the independence is the regression value.
//!
//! Sections mirror the review charter parts 1.1–1.5, plus the review
//! extras (6: interval chain-rule width, 7: the `powi(2)` fix the review
//! proposed — since adopted in `src/dual.rs`, so section 6's quality
//! checks assert the *fixed* behavior). `Dual<Interval>` sections are
//! gated on the `interval` feature; everything else runs in the default
//! build.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::excessive_precision, clippy::approx_constant)]

#[cfg(feature = "interval")]
use geom_core::predicate::Sign;
use geom_core::predicate::{Band, Decide};
use geom_core::real::Real;
use geom_core::{Dual, Dual64, Point2, Vec2};
use num_dual::DualNum;

type Nd = num_dual::Dual64;

fn ulp_dist(a: f64, b: f64) -> u64 {
    fn ord(x: f64) -> i64 {
        let i = x.to_bits() as i64;
        if i < 0 { i64::MIN.wrapping_sub(i) } else { i }
    }
    (i128::from(ord(a)) - i128::from(ord(b))).unsigned_abs() as u64
}

// ---------------------------------------------------------------------
// The M6-flavored stackup pipeline: a part feature point p0, rotated by
// the joint angle theta and translated by the carriage offset, measured
// as perpendicular distance to a fixed datum line. Real linalg
// (Point2/Vec2, perp_dot), ends in abs() — a kink op in the pipeline on
// purpose. The rotate-then-translate pose is spelled out rather than
// assembled from a shipped affine type: the independence is the
// regression value, and this association order is the one the analytic
// derivative below differentiates.
// ---------------------------------------------------------------------
fn stackup<T: Real>(theta: T) -> T {
    let p0 = Point2::new(T::from_f64(2.0), T::from_f64(0.5));
    let carriage = Vec2::new(T::from_f64(1.5), T::from_f64(-0.25));
    let (s, c) = theta.sin_cos();
    let p = Point2::new(
        (c * p0.x - s * p0.y) + carriage.x,
        (s * p0.x + c * p0.y) + carriage.y,
    );
    // datum line through la, unit direction ld (3-4-5 exact unit);
    // la sits near the orbit so the signed distance crosses zero (an
    // abs kink exists in theta)
    let la = Point2::new(T::from_f64(1.4), T::from_f64(0.0));
    let ld = Vec2::new(T::from_f64(0.6), T::from_f64(0.8));
    (p - la).perp_dot(ld).abs()
}

/// Analytic d/dtheta at f64: p = R p0 + t; dp = R' p0; d|s|/dθ = sign(s)·(dp × ld).
fn stackup_dtheta_analytic(theta: f64) -> f64 {
    let (s, c) = (libm::sin(theta), libm::cos(theta));
    let (p0x, p0y) = (2.0, 0.5);
    let px = c * p0x - s * p0y + 1.5;
    let py = s * p0x + c * p0y - 0.25;
    let dpx = -s * p0x - c * p0y;
    let dpy = c * p0x - s * p0y;
    let (ldx, ldy) = (0.6, 0.8);
    let signed = (px - 1.4) * ldy - (py - 0.0) * ldx;
    let dsigned = dpx * ldy - dpy * ldx;
    signed.signum() * dsigned
}

// ---------------------------------------------------------------------
// 1. M6-flavored stackup: Dual64 deriv vs analytic vs central diff.
// ---------------------------------------------------------------------

#[test]
fn stackup_dual64_deriv_matches_analytic_and_central_diff() {
    let mut max_rel_analytic = 0.0f64;
    let mut max_rel_cd = 0.0f64;
    for i in 0..2000 {
        let theta = -3.1 + 6.2 * f64::from(i) / 1999.0;
        let d = stackup(Dual::variable(theta));
        let analytic = stackup_dtheta_analytic(theta);
        // skip near the kink (signed distance ~ 0) where the derivative jumps
        let signed_dist = d.value;
        if signed_dist.abs() < 1e-3 {
            continue;
        }
        let rel_a = (d.deriv - analytic).abs() / (1.0 + analytic.abs());
        max_rel_analytic = max_rel_analytic.max(rel_a);
        let h = 1e-6;
        let cd = (stackup(theta + h) - stackup(theta - h)) / (2.0 * h);
        let rel_c = (d.deriv - cd).abs() / (1.0 + cd.abs());
        max_rel_cd = max_rel_cd.max(rel_c);
    }
    assert!(
        max_rel_analytic < 1e-12,
        "Dual64 deriv vs analytic d/dθ: max rel err {max_rel_analytic:.3e}"
    );
    assert!(
        max_rel_cd < 1e-7,
        "Dual64 deriv vs central differences: max rel err {max_rel_cd:.3e}"
    );
}

// ---------------------------------------------------------------------
// 2. Value-channel fidelity: plain f64 vs Dual64 bit-identity, hunting
// divergence on the nasty inputs.
// ---------------------------------------------------------------------

#[test]
fn value_channel_bit_identity_hunt() {
    let specials = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        0.5,
        -0.5,
        1e-308,
        -1e-308,
        1e308,
        -1e308,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MIN_POSITIVE,
        core::f64::consts::PI,
        -core::f64::consts::FRAC_PI_2,
        1e17,
        -1e17,
    ];
    let seeds = [0.0, 1.0, f64::NAN, f64::INFINITY, -1e300];
    let mut bad = Vec::new();
    for &x in &specials {
        for &dx in &seeds {
            let a = Dual::new(x, dx);
            let unary: [(&str, f64, Dual64); 10] = [
                ("neg", -x, -a),
                ("sqrt", Real::sqrt(x), a.sqrt()),
                ("abs", Real::abs(x), a.abs()),
                ("sin", Real::sin(x), Real::sin(a)),
                ("cos", Real::cos(x), Real::cos(a)),
                ("tan", Real::tan(x), a.tan()),
                ("asin", Real::asin(x), a.asin()),
                ("acos", Real::acos(x), a.acos()),
                ("atan", Real::atan(x), a.atan()),
                ("sin_cos.0", Real::sin_cos(x).0, a.sin_cos().0),
            ];
            for (name, plain, dual) in unary {
                if plain.to_bits() != dual.value.to_bits() {
                    bad.push(format!(
                        "{name}({x:?}) seed {dx:?}: {plain:?} vs {:?}",
                        dual.value
                    ));
                }
            }
            for n in [-7, -2, -1, 0, 1, 2, 3, 8, i32::MIN, i32::MAX] {
                let plain = <f64 as Real>::powi(x, n);
                let dual = a.powi(n);
                if plain.to_bits() != dual.value.to_bits() {
                    bad.push(format!(
                        "powi({x:?},{n}) seed {dx:?}: {plain:?} vs {:?}",
                        dual.value
                    ));
                }
            }
            for &y in &specials {
                let b = Dual::new(y, 1.0);
                let binary: [(&str, f64, Dual64); 7] = [
                    ("add", x + y, a + b),
                    ("sub", x - y, a - b),
                    ("mul", x * y, a * b),
                    ("div", x / y, a / b),
                    ("atan2", Real::atan2(x, y), Real::atan2(a, b)),
                    ("min", Real::min(x, y), Real::min(a, b)),
                    ("max", Real::max(x, y), Real::max(a, b)),
                ];
                for (name, plain, dual) in binary {
                    if plain.to_bits() != dual.value.to_bits() {
                        bad.push(format!(
                            "{name}({x:?},{y:?}): {plain:?} vs {:?}",
                            dual.value
                        ));
                    }
                }
            }
        }
    }
    assert!(
        bad.is_empty(),
        "value-channel divergences (incl. NaN^0, sqrt(0), abs(-0), ties, quadrants): \
         {} cases, first: {:?}",
        bad.len(),
        bad.first()
    );

    // Whole-pipeline bit-identity at random-ish points.
    for i in 0..20000 {
        let theta = -6.0 + 12.0 * f64::from(i) / 19999.0;
        let plain = stackup(theta);
        let dual = stackup(Dual::variable(theta));
        assert!(
            plain.to_bits() == dual.value.to_bits(),
            "stackup pipeline value diverges at θ={theta}: {plain:?} vs {:?}",
            dual.value
        );
    }
}

// ---------------------------------------------------------------------
// 3. Derivative never branches / no tangent→value crosstalk (f64 half;
// the Dual<Interval> half lives in the gated module below).
// ---------------------------------------------------------------------

#[test]
fn tangent_never_influences_decisions_or_values() {
    let band = Band::new(1e-9, 1e-8).expect("band");
    // 3a. Decide identical to value-alone, adversarial tangents.
    let values = [
        1.0,
        -1.0,
        0.0,
        5e-9,
        -9.9e-9,
        2e-8,
        f64::NAN,
        f64::INFINITY,
        1e-300,
    ];
    let tangents = [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        1e308,
        -1e308,
        0.0,
        f64::from_bits(0x7ff8_dead_beef_dead),
    ];
    for &v in &values {
        let alone = v.sign_within(band);
        for &t in &tangents {
            let dual = Dual::new(v, t).sign_within(band);
            assert_eq!(
                dual, alone,
                "Dual64 Decide must equal value-alone Decide: value {v:?} tangent {t:?}"
            );
        }
    }

    // 3b. Crosstalk probe: value bits must be a function of value bits
    // alone — vary the tangent wildly (tagged NaN payloads included),
    // values must not move by a single bit, through every op and the
    // whole pipeline.
    let payload_a = f64::from_bits(0x7ff8_1111_2222_3333);
    let payload_b = f64::from_bits(0xfff8_aaaa_bbbb_cccc);
    for i in 0..5000 {
        let x = -20.0 + 40.0 * f64::from(i) / 4999.0;
        let y = 3.7 - x * 0.3;
        for (ta, tb) in [(1.0, -1e307), (payload_a, payload_b), (0.0, f64::INFINITY)] {
            let (a1, a2) = (Dual::new(x, ta), Dual::new(x, tb));
            let (b1, b2) = (Dual::new(y, ta), Dual::new(y, tb));
            let pairs: [(&str, f64, f64); 12] = [
                ("sqrt", a1.abs().sqrt().value, a2.abs().sqrt().value),
                ("tan", a1.tan().value, a2.tan().value),
                (
                    "atan2",
                    Real::atan2(a1, b1).value,
                    Real::atan2(a2, b2).value,
                ),
                ("mul", (a1 * b1).value, (a2 * b2).value),
                ("div", (a1 / b1).value, (a2 / b2).value),
                ("min", Real::min(a1, b1).value, Real::min(a2, b2).value),
                ("max", Real::max(a1, b1).value, Real::max(a2, b2).value),
                ("abs", a1.abs().value, a2.abs().value),
                ("powi", a1.powi(3).value, a2.powi(3).value),
                (
                    "asin",
                    (a1 * Dual::constant(0.04)).asin().value,
                    (a2 * Dual::constant(0.04)).asin().value,
                ),
                ("sin_cos", a1.sin_cos().0.value, a2.sin_cos().0.value),
                ("pipeline", stackup(a1).value, stackup(a2).value),
            ];
            for (name, v1, v2) in pairs {
                assert!(
                    v1.to_bits() == v2.to_bits(),
                    "tangent→value crosstalk in {name} at x={x}: {v1:?} vs {v2:?}"
                );
            }
        }
    }

    // 3c. Tagged-payload leak: the tangent's NaN payload must never
    // appear in the value channel even when the value itself goes NaN.
    let tagged = f64::from_bits(0x7ff8_dead_beef_dead);
    let a = Dual::new(f64::NAN, tagged);
    let leaked = [a.sqrt().value, a.tan().value, (a * a).value, a.abs().value]
        .iter()
        .any(|v| v.to_bits() == tagged.to_bits());
    assert!(!leaked, "tangent NaN payload leaked into a value channel");
}

// ---------------------------------------------------------------------
// 4. Kink semantics at the edges (f64 half).
// ---------------------------------------------------------------------

#[test]
fn f64_kink_semantics() {
    // f64 abs at signed zeros / NaN.
    let pz = Dual::new(0.0, 3.0).abs();
    let nz = Dual::new(-0.0, 3.0).abs();
    assert!(
        pz.value.to_bits() == 0.0f64.to_bits()
            && nz.value.to_bits() == 0.0f64.to_bits()
            && pz.deriv == 3.0
            && nz.deriv == 3.0,
        "abs at ±0.0 must give value +0.0 bitwise and factor +1 both: {pz:?} {nz:?}"
    );
    let nan = Dual::new(f64::NAN, 3.0).abs();
    assert!(
        nan.value.is_nan() && nan.deriv.is_nan(),
        "abs at NaN value: both channels NaN"
    );
    // min/max ties: derivative follows self, bit-checked.
    let a = Dual::new(1.0, f64::from_bits(0x400921FB54442D18)); // tagged deriv
    let b = Dual::new(1.0, 99.0);
    assert!(
        Real::min(a, b).deriv.to_bits() == a.deriv.to_bits()
            && Real::max(a, b).deriv.to_bits() == a.deriv.to_bits()
            && Real::min(b, a).deriv.to_bits() == b.deriv.to_bits(),
        "min/max tie must keep self's tangent bitwise"
    );
}

// ---------------------------------------------------------------------
// 5. Oracle honesty: do the num-dual bounds bind?
// ---------------------------------------------------------------------

/// Measure real ulp gaps op by op over a dense sweep, vs the shipped test
/// suite's allowances; then show a sign-flip / wrong-formula mutant lands
/// far outside them (the bounds would catch real derivative bugs).
#[test]
fn num_dual_oracle_bounds_bind() {
    let mut max_gap: Vec<(&str, u64)> = vec![
        ("sin", 0),
        ("cos", 0),
        ("tan", 0),
        ("atan", 0),
        ("asin", 0),
        ("atan2", 0),
        ("powi", 0),
    ];
    let mut flip_caught = 0u32;
    let mut flip_total = 0u32;
    let mut mutant_caught = 0u32;
    let mut mutant_total = 0u32;
    for i in 0..20000 {
        let x = -20.0 + 40.0 * f64::from(i) / 19999.0;
        let a = Dual::variable(x);
        let na = Nd::new(x, 1.0);
        let gaps = [
            (0, Real::sin(a).deriv, na.sin().eps),
            (1, Real::cos(a).deriv, na.cos().eps),
            (2, a.tan().deriv, na.tan().eps),
            (3, a.atan().deriv, na.atan().eps),
        ];
        for (idx, ours, theirs) in gaps {
            if ours.is_nan() || theirs.is_nan() {
                continue;
            }
            let d = ulp_dist(ours, theirs);
            if d > max_gap[idx].1 {
                max_gap[idx].1 = d;
            }
            // Would a sign-flipped derivative pass the 64/128-ulp bound?
            if ours != 0.0 && ours.abs() > 1e-8 {
                flip_total += 1;
                if ulp_dist(-ours, theirs) > 128 {
                    flip_caught += 1;
                }
            }
        }
        // asin on its domain
        let xa = x / 21.0;
        let d = ulp_dist(Dual::variable(xa).asin().deriv, Nd::new(xa, 1.0).asin().eps);
        if d > max_gap[4].1 {
            max_gap[4].1 = d;
        }
        // atan2
        let y = 3.0 + 0.1 * x;
        let d = ulp_dist(
            Real::atan2(Dual::variable(y), Dual::constant(x)).deriv,
            Nd::new(y, 1.0).atan2(Nd::new(x, 0.0)).eps,
        );
        if d > max_gap[5].1 {
            max_gap[5].1 = d;
        }
        // powi
        let xb = 0.2 + (x + 20.0) / 40.0 * 2.8;
        for n in [-5, -2, 2, 5] {
            let ours = Dual::variable(xb).powi(n).deriv;
            let theirs = Nd::new(xb, 1.0).powi(n).eps;
            let d = ulp_dist(ours, theirs);
            if d > max_gap[6].1 {
                max_gap[6].1 = d;
            }
        }
        // Mutant check: tan' as (1 - t²) instead of (1 + t²) — must
        // violate the 128-ulp tan bound at generic x.
        let t = Real::tan(x);
        let mutant = 1.0 - t * t;
        if !t.is_nan() && t.abs() > 0.05 {
            mutant_total += 1;
            if ulp_dist(mutant, na.tan().eps) > 128 {
                mutant_caught += 1;
            }
        }
    }
    let allowances = [64u64, 64, 128, 64, 64, 128, 64];
    assert!(
        max_gap.iter().zip(allowances).all(|((_, g), a)| *g <= a),
        "observed gaps exceed the suite's allowances: {max_gap:?}"
    );
    assert!(
        flip_caught == flip_total,
        "a sign-flipped derivative would slip the bounds: {flip_caught}/{flip_total}"
    );
    assert!(
        mutant_caught == mutant_total,
        "the 1−t² tan mutant would slip the bounds: {mutant_caught}/{mutant_total}"
    );
}

// ---------------------------------------------------------------------
// 7 (f64 half). The powi(2) fix the review proposed, validated: powi(2)
// must be bit-identical to x*x at f64, so using it in the deriv formulas
// is a no-op at Dual64 (while giving the tight square at Dual<Interval>).
// ---------------------------------------------------------------------

#[test]
fn f64_powi2_is_bit_identical_to_multiplication() {
    let mut bad = 0;
    for i in 0..100000u64 {
        let x = f64::from_bits(0x3f80_0000_0000_0000_u64.wrapping_add(i * 0x0000_1234_5678_9abc));
        if <f64 as Real>::powi(x, 2).to_bits() != (x * x).to_bits() {
            bad += 1;
        }
    }
    for x in [
        0.0,
        -0.0,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        1e308,
        1e-308,
    ] {
        if <f64 as Real>::powi(x, 2).to_bits() != (x * x).to_bits() {
            bad += 1;
        }
    }
    assert_eq!(bad, 0, "f64 Real::powi(x,2) must be bit-identical to x*x");
}

// ---------------------------------------------------------------------
// The Dual<Interval> halves of sections 1, 3, 4, 6, 7.
// ---------------------------------------------------------------------

#[cfg(feature = "interval")]
mod dual_interval {
    use geom_core::real::Bounds;
    use geom_core::{Interval, MarginDiag};

    use super::*;

    /// Section 1, interval half: the same generic stackup at
    /// Dual<Interval> over theta-boxes — the deriv enclosure must bracket
    /// the analytic derivative at every theta in the box (away from the
    /// abs kink), and a kink-straddling box must enclose BOTH one-sided
    /// derivatives.
    #[test]
    fn stackup_dual_interval_deriv_encloses_analytic() {
        let boxes = [
            (0.2, 0.4),
            (1.0, 1.05),
            (-2.0, -1.9),
            (2.9, 3.1),
            (0.0, 0.5),
        ];
        for &(lo, hi) in &boxes {
            let d = stackup(Dual::variable(Interval::from_bounds(lo, hi)));
            let (dlo, dhi) = (d.deriv.lo(), d.deriv.hi());
            for j in 0..=400 {
                let theta = lo + (hi - lo) * f64::from(j) / 400.0;
                let a = stackup_dtheta_analytic(theta);
                assert!(
                    dlo <= a && a <= dhi,
                    "box [{lo},{hi}]: analytic {a} outside [{dlo},{dhi}] at θ={theta}"
                );
            }
        }

        // A box straddling the abs kink: find it via the raw signed
        // expression, then check the enclosure covers both one-sided
        // derivatives.
        let signed = |t: f64| {
            let (s, c) = (libm::sin(t), libm::cos(t));
            (c * 2.0 - s * 0.5 + 1.5 - 1.4) * 0.8 - (s * 2.0 + c * 0.5 - 0.25) * 0.6
        };
        let mut kink = None;
        for i in 0..60000 {
            let a = -3.1 + 6.2 * f64::from(i) / 59999.0;
            let b = -3.1 + 6.2 * f64::from(i + 1) / 59999.0;
            if signed(a) * signed(b) < 0.0 {
                kink = Some((a, b));
                break;
            }
        }
        let (a, b) = kink.expect("the signed distance must cross zero somewhere in a full turn");
        let d = stackup(Dual::variable(Interval::from_bounds(a - 0.01, b + 0.01)));
        let (dlo, dhi) = (d.deriv.lo(), d.deriv.hi());
        let left = stackup_dtheta_analytic(a - 0.009);
        let right = stackup_dtheta_analytic(b + 0.009);
        assert!(
            dlo <= left && left <= dhi && dlo <= right && right <= dhi,
            "kink-straddling box must enclose BOTH one-sided derivatives: \
             enclosure [{dlo},{dhi}] vs one-sided {left} / {right}"
        );
    }

    /// Section 3, interval half: Decide on Dual<Interval> equals
    /// value-alone Decide under adversarial (NaI / empty / huge) tangents.
    #[test]
    fn dual_interval_decide_ignores_tangents() {
        let band = Band::new(1e-9, 1e-8).expect("band");
        let ivals = [
            Interval::from_bounds(1.0, 2.0),
            Interval::from_bounds(-1.0, 1.0),
            Interval::from_f64(5e-9),
            Interval::from_f64(f64::NAN),
        ];
        let itangents = [
            Interval::from_f64(f64::NAN),
            Interval::from_bounds(-1e300, 1e300),
            Interval::from_bounds(-4.0, -1.0).sqrt(), // empty
            Interval::zero(),
        ];
        for &v in &ivals {
            let alone = v.sign_within(band);
            for &t in &itangents {
                let dual = Dual::new(v, t).sign_within(band);
                assert_eq!(
                    dual, alone,
                    "Dual<Interval> Decide must equal value-alone Decide"
                );
            }
        }
    }

    /// Section 4, interval half: kink semantics — straddle hulls, poison
    /// tangents surviving the hull (NOT absorbed a la IEEE 1788), and
    /// decoration capping observable through Decide on the deriv channel.
    #[test]
    fn interval_kink_semantics_and_poison_hulls() {
        // Straddle hull factor: deriv = [-1,1]·[2,3] = [-3,3].
        let straddle = Dual::new(
            Interval::from_bounds(-1.0, 2.0),
            Interval::from_bounds(2.0, 3.0),
        )
        .abs();
        assert!(
            straddle.deriv.lo() == -3.0 && straddle.deriv.hi() == 3.0,
            "interval abs straddle deriv: [{},{}]",
            straddle.deriv.lo(),
            straddle.deriv.hi()
        );
        // Touching intervals hull (only STRICT separation selects).
        let a = Dual::new(Interval::from_bounds(1.0, 2.0), Interval::from_f64(1.0));
        let b = Dual::new(Interval::from_bounds(2.0, 3.0), Interval::from_f64(9.0));
        let m = Real::min(a, b);
        assert!(
            m.deriv.lo() == 1.0 && m.deriv.hi() == 9.0,
            "touching enclosures must hull tangents: [{},{}]",
            m.deriv.lo(),
            m.deriv.hi()
        );
        // Empty / NaI through both channels.
        let empty = Dual::variable(Interval::from_bounds(-4.0, -1.0)).sqrt();
        assert!(
            empty.value.lo().is_nan() && empty.deriv.lo().is_nan(),
            "sqrt(certainly negative): both channels empty (NaN brackets)"
        );
        let nai = Dual::variable(Interval::from_f64(f64::NAN));
        let r = nai.tan();
        assert!(
            r.value.lo().is_nan() && r.deriv.lo().is_nan(),
            "NaI must propagate through both channels of tan"
        );

        // THE poisoned-tangent-vs-1788-hull case: a clean-valued dual
        // whose TANGENT is empty (poison), min'd against an overlapping
        // dual with a clean tangent. IEEE 1788 convexHull(empty, [1,1]) =
        // [1,1] — poison silently dropped. The implementation must keep
        // the poison.
        let empty_tangent = Interval::from_bounds(-4.0, -1.0).sqrt();
        let a = Dual::new(Interval::from_bounds(1.0, 3.0), empty_tangent);
        let b = Dual::new(Interval::from_bounds(2.0, 4.0), Interval::from_f64(1.0));
        let m = Real::min(a, b);
        assert!(
            m.deriv.lo().is_nan() && !m.value.lo().is_nan(),
            "poisoned (empty) tangent in hull region must survive: deriv [{},{}]",
            m.deriv.lo(),
            m.deriv.hi()
        );
        // NaI tangent likewise.
        let a = Dual::new(
            Interval::from_bounds(1.0, 3.0),
            Interval::from_f64(f64::NAN),
        );
        let m = Real::max(a, b);
        assert!(
            m.deriv.lo().is_nan(),
            "NaI tangent in hull region must survive"
        );

        // Decoration convention probe (observable via Decide on the deriv
        // field, a plain Interval): hull of two CLEAN tangents must stay
        // classifiable (min-of-decorations keeps Com — 1788's set-op
        // convention would drop to Trv and refuse); a Trv tangent in the
        // hull caps it (refuses).
        let band = Band::new(1e-9, 1e-8).expect("band");
        let a = Dual::new(
            Interval::from_bounds(1.0, 3.0),
            Interval::from_bounds(5.0, 6.0),
        );
        let b = Dual::new(
            Interval::from_bounds(2.0, 4.0),
            Interval::from_bounds(7.0, 8.0),
        );
        let hulled = Real::min(a, b);
        assert_eq!(
            hulled.deriv.sign_within(band),
            Ok(Sign::Positive),
            "clean-tangent hull must keep a classifiable decoration"
        );
        let trv_tangent = Interval::from_bounds(-1.0, 4.0).sqrt(); // [0,2] Trv
        let a = Dual::new(Interval::from_bounds(1.0, 3.0), trv_tangent);
        let hulled = Real::min(a, b);
        assert!(
            hulled.deriv.sign_within(band).is_err(),
            "Trv tangent must cap the hull decoration (refuse to classify)"
        );
        // And a Trv VALUE caps a selected tangent even under strict
        // separation.
        let trv_value = Interval::from_bounds(-1.0, 4.0).sqrt(); // [0,2] Trv
        let lo = Dual::new(trv_value, Interval::from_f64(5.0));
        let hi = Dual::new(Interval::from_bounds(10.0, 11.0), Interval::from_f64(7.0));
        let m = Real::min(lo, hi);
        assert!(
            matches!(
                m.deriv.sign_within(band),
                Err(geom_core::Indeterminate {
                    margin: MarginDiag::Invalid,
                    ..
                })
            ) && m.deriv.lo() == 5.0
                && m.deriv.hi() == 5.0,
            "Trv VALUE must cap the selected tangent's decoration: {:?} [{},{}]",
            m.deriv.sign_within(band),
            m.deriv.lo(),
            m.deriv.hi()
        );
    }

    /// Section 6: interval chain-rule width — with the adopted `powi(2)`
    /// fix, the deriv enclosures on straddling boxes stay bounded and
    /// sign-preserving instead of blowing up under the a·a dependency
    /// problem (the review demonstrated the blow-up and proposed the fix;
    /// these assert the fixed behavior AND that soundness still holds).
    #[test]
    fn interval_chain_rule_width_quality_after_the_fix() {
        // atan over a box straddling 0: 1 + a.powi(2) stays >= 1.
        let a = Dual::variable(Interval::from_bounds(-1.0, 1.1));
        let d = a.atan();
        assert!(
            d.deriv.hi().is_finite() && d.deriv.lo() >= 0.0,
            "atan deriv enclosure must stay bounded on a straddling box: [{}, {}]",
            d.deriv.lo(),
            d.deriv.hi()
        );
        // tan: sec² keeps its sign.
        let a = Dual::variable(Interval::from_bounds(-0.9, 0.9));
        let d = a.tan();
        assert!(
            d.deriv.lo() > 0.0,
            "tan deriv enclosure must keep sec² positive: [{}, {}]",
            d.deriv.lo(),
            d.deriv.hi()
        );
        // atan2 with x straddling but |(x,y)| bounded away from 0.
        let y = Dual::constant(Interval::from_f64(2.0));
        let x = Dual::variable(Interval::from_bounds(-3.0, 3.0));
        let d = Real::atan2(y, x);
        assert!(
            d.deriv.hi().is_finite() && d.deriv.lo().is_finite(),
            "atan2 deriv enclosure must stay bounded: [{}, {}]",
            d.deriv.lo(),
            d.deriv.hi()
        );
        // Soundness of the enclosure regardless of width: contains the
        // true ∂/∂x atan2(2, x) = -2 / (x² + 4) across the box.
        for j in 0..=300 {
            let xv = -3.0 + 6.0 * f64::from(j) / 300.0;
            let truth = -2.0 / (xv * xv + 4.0);
            assert!(
                d.deriv.lo() <= truth && truth <= d.deriv.hi(),
                "enclosure must contain the true derivative at x={xv}"
            );
        }
    }

    /// Section 7, interval half: powi(2) is the tight square, and the
    /// reconstructed atan derivative with it is bounded and tight.
    #[test]
    fn interval_powi2_is_the_tight_square() {
        let a = Interval::from_bounds(-1.0, 1.1);
        let sq = a.powi(2);
        assert!(
            sq.lo() == 0.0 && (sq.hi() - 1.21).abs() < 1e-15,
            "Interval powi(2) of [-1,1.1] must be [0, 1.21]: [{}, {}]",
            sq.lo(),
            sq.hi()
        );
        let denom = Interval::one() + sq;
        let fixed = Interval::one() / denom;
        assert!(
            fixed.lo().is_finite()
                && fixed.hi().is_finite()
                && fixed.lo() > 0.4
                && fixed.hi() <= 1.0,
            "fixed atan deriv enclosure over [-1,1.1] must be bounded: [{}, {}]",
            fixed.lo(),
            fixed.hi()
        );
    }
}
