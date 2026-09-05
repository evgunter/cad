//! **R1's unit-level probes of M10-8's rules A and B** (claim 7:
//! soundness where the rules DO fire), derived from the claims rather
//! than from the unit's rows. Every row is a deterministic fixture.
//!
//! The rows that ASSERT are the soundness ones (a rule may never decide
//! a non-identity, and clause 1 must stand in front of rule A); they run
//! at `with_session` (rules A+B on) and at explicit rule sets.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use geom_core::interval::Interval;
use geom_core::predicate::{Band, Sign};
use geom_core::real::Real;
use geom_core::sym::with_session_rules;
use geom_core::{Decide, ParamSymbol, Sym, SymBudget, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn p(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param(ParamSymbol::of(name), Interval::from_bounds(lo, hi))
}

fn lit(x: f64) -> Sym<Interval> {
    <Sym<Interval> as Real>::from_f64(x)
}

fn sign_of(m: Sym<Interval>) -> Result<Sign, ()> {
    m.sign_within(band()).map_err(|_| ())
}

/// **Clause 1 in front of rule A**: `sqrt(X)² − X` with `X` STRADDLING
/// zero over the box. The interval `sqrt` CLAMPS a partially negative
/// argument to `[0, hi]` and records the violation only in the
/// decoration, so the value channel's answer is `Invalid`, and the
/// tier must not answer `Zero` through rule A — the expression has no
/// real value on half the box.
#[test]
fn r1_rule_a_never_fires_on_a_straddling_argument() {
    for (lo, hi) in [(-1.0, 4.0), (-4.0, -1.0), (-1e-3, 1e-3)] {
        let (s, counts) = with_session_rules(budget(), SymRules::all(), || {
            let x = p("x", lo, hi);
            let r = x.sqrt();
            sign_of(r * r - x)
        });
        assert_ne!(
            s,
            Ok(Sign::Zero),
            "[{lo}, {hi}]: a straddling sqrt is Invalid, never Zero"
        );
        assert_eq!(counts.symbolic_zero, 0, "[{lo}, {hi}]: {counts:?}");
    }
}

/// **Rule A on an in-domain box**: `sqrt(X)² − X` decides Zero at every
/// width where `X ≥ 0` over the whole box — and the SAME residual with
/// the rules off stays numeric (it widens), which is the property the
/// rule buys.
#[test]
fn r1_rule_a_decides_zero_at_every_width_and_off_it_widens() {
    for half in [1e-12_f64, 1e-3, 0.5, 2.0] {
        let build = || {
            let x = p("x", 3.0 - half, 3.0 + half);
            // `y` stays positive at every width: the interval `y*y` is a
            // MULTIPLICATION (not the tight `powi`), so a straddling `y`
            // makes `x² + y² + 1` straddle zero, and clause 1 then
            // refuses the theorem however sound rule A is — measured at
            // y = 1 ± 2, and worth knowing about the arc family's `v·v`.
            let y = p("y", 4.0 - half, 4.0 + half);
            let arg = x * x + y * y + lit(1.0);
            let s = arg.sqrt();
            (
                sign_of(s * s - arg),
                sign_of(s.powi(4) - arg * arg),
                sign_of(s * s * s - arg * s),
            )
        };
        let (on, c_on) = with_session_rules(budget(), SymRules::all(), build);
        assert_eq!(
            on,
            (Ok(Sign::Zero), Ok(Sign::Zero), Ok(Sign::Zero)),
            "half={half}"
        );
        assert_eq!(c_on.symbolic_zero, 3, "half={half}: {c_on:?}");
        let (off, c_off) = with_session_rules(budget(), SymRules::none(), build);
        assert_eq!(
            c_off.symbolic_zero, 0,
            "half={half}: rules off, no rule fires"
        );
        if half >= 0.5 {
            assert!(
                off.0 != Ok(Sign::Zero),
                "half={half}: with the rules off a wide box widens the residual: {off:?}"
            );
        }
    }
}

/// **Rule A must not decide a NON-identity**: `sqrt(X)² − Y` with `Y`
/// a different form equal to `X` only at the nominal, and `sqrt(X)·
/// sqrt(Y) − X` (two different atoms), never decide symbolically.
#[test]
fn r1_rule_a_never_claims_a_coincidence() {
    let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
        let x = p("x", 2.0, 2.0);
        let y = p("y", 2.0, 2.0);
        let a = sign_of(x.sqrt() * x.sqrt() - y);
        let b = sign_of(x.sqrt() * y.sqrt() - x);
        let c = sign_of((x + lit(1.0)).sqrt().powi(2) - x);
        (a, b, c)
    });
    // All three are numerically zero (or definite) at the point; none
    // is a theorem.
    let _ = out;
    assert_eq!(counts.symbolic_zero, 0, "{counts:?}");
}

/// **Rule B, same argument only**: `sin²θ + cos²θ − 1` is a theorem at
/// every width for a compound argument; `sin²θ + cos²φ − 1` is never;
/// `sin⁴θ − (1 − cos²θ)²` is (an even power above two); `sinθ − cosθ`
/// is not (odd powers are out of the rule's shape); `sin θ + cos θ −
/// 1` at θ = 0 exactly is a NUMERIC zero, not a theorem.
#[test]
fn r1_rule_b_shape() {
    for half in [1e-9_f64, 0.3, 3.0] {
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let t = p("t", 0.7 - half, 0.7 + half);
            let u = p("u", 0.2 - half, 0.2 + half);
            let theta = t * u + lit(0.5);
            let (s, c) = theta.sin_cos();
            let (s2, _) = u.sin_cos();
            (
                sign_of(s * s + c * c - lit(1.0)),
                sign_of(s2 * s2 + c * c - lit(1.0)),
                sign_of(s.powi(4) - (lit(1.0) - c * c).powi(2)),
                sign_of(s - c),
            )
        });
        assert_eq!(out.0, Ok(Sign::Zero), "half={half}: the Pythagorean pair");
        assert_eq!(out.2, Ok(Sign::Zero), "half={half}: sin⁴ = (1 − cos²)²");
        assert_eq!(
            counts.symbolic_zero, 2,
            "half={half}: exactly the two identities: {counts:?}"
        );
        if half >= 0.3 {
            assert_ne!(out.1, Ok(Sign::Zero), "half={half}: mixed arguments widen");
        }
    }
    let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
        let t = p("t", 0.0, 0.0);
        let (s, c) = t.sin_cos();
        sign_of(s + c - lit(1.0))
    });
    assert_eq!(out, Ok(Sign::Zero), "numerically zero at θ = 0");
    assert_eq!(counts.symbolic_zero, 0, "and not a theorem: {counts:?}");
}

/// **Rule B on a POISONED argument**: `sin²θ + cos²θ − 1` with `θ =
/// 1/(x − x)` has no value; clause 1's form-side half must refuse it
/// even though the rewrite would produce the zero polynomial.
#[test]
fn r1_rule_b_never_fires_through_poison() {
    let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
        let x = p("x", 1.0, 2.0);
        let theta = lit(1.0) / (x - x);
        let (s, c) = theta.sin_cos();
        sign_of(s * s + c * c - lit(1.0))
    });
    assert_ne!(out, Ok(Sign::Zero), "a poisoned argument never certifies");
    assert_eq!(counts.symbolic_zero, 0, "{counts:?}");
}

/// **Rule A through a reciprocal**: `1/sqrt(X)² − 1/X` and the arc's
/// own shape `(v·v)/sqrt(v·v)² − 1` (the normalized frame's `u·u = 1`)
/// are theorems at every width where `v·v > 0`.
#[test]
fn r1_rule_a_reaches_the_normalized_frame_shape_on_a_small_form() {
    for half in [1e-9_f64, 0.25] {
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let vx = p("vx", 3.0 - half, 3.0 + half);
            let vy = p("vy", 4.0 - half, 4.0 + half);
            let vv = vx * vx + vy * vy;
            let n = vv.sqrt();
            let (ux, uy) = (vx / n, vy / n);
            (
                sign_of(ux * ux + uy * uy - lit(1.0)),
                sign_of(lit(1.0) / (n * n) - lit(1.0) / vv),
            )
        });
        assert_eq!(out, (Ok(Sign::Zero), Ok(Sign::Zero)), "half={half}");
        assert_eq!(counts.symbolic_zero, 2, "half={half}: {counts:?}");
    }
}

/// **R1's A0 fold is exact and value-free**: `sqrt(c)` of a literal that
/// is a perfect square folds to the literal root, so `sqrt(4)·x − 2·x`
/// and `sqrt(6.4e-5)·x − 8e-3·x` (the plate's own constant) are
/// theorems; `sqrt(2)·x − 1.4142·x` is NOT (no rational root); `abs(−3)
/// − 3` is; and a NEGATIVE literal never folds.
#[test]
fn r1_a0_constant_fold_is_exact() {
    let rules = SymRules {
        const_fold: true,
        ..SymRules::none()
    };
    let (out, counts) = with_session_rules(budget(), rules, || {
        let x = p("x", 1.0, 2.0);
        let plate_edge = lit(8.0e-3) * lit(8.0e-3);
        (
            sign_of(lit(4.0).sqrt() * x - lit(2.0) * x),
            sign_of(plate_edge.sqrt() * x - lit(8.0e-3) * x),
            sign_of(lit(2.0).sqrt() * x - lit(1.4142) * x),
            sign_of(lit(-3.0).abs() - lit(3.0)),
            sign_of(lit(-4.0).sqrt() - lit(2.0)),
        )
    });
    assert_eq!(out.0, Ok(Sign::Zero));
    assert_eq!(out.1, Ok(Sign::Zero));
    assert_ne!(out.2, Ok(Sign::Zero));
    assert_eq!(out.3, Ok(Sign::Zero));
    assert_ne!(out.4, Ok(Sign::Zero));
    assert_eq!(counts.symbolic_zero, 3, "{counts:?}");
}
