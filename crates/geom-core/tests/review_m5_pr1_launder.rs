//! **Poison-laundering attempts over the public `Dual<Interval>`
//! surface** (adopted from the M5 PR 1 adversarial review's scratch
//! harness).
//!
//! `crates/geom-core/tests/m5_pr1_poison_conservation.rs` pins
//! conservation at the bare `Interval` scalar. This file attacks the
//! *composed* scalar, because that is where laundering has the most
//! places to hide: `Dual<Interval>` has two channels, and an operation
//! that reconstructs one of them from constants — `powi(0)` returning
//! `[1,1]`, `abs`/`floor` discarding sign or fraction, the kink
//! selectors picking a tangent by *comparing* values — is exactly the
//! shape that could hand back a clean-looking result built from a
//! poisoned input.
//!
//! The attack surface is enumerated as 3 poison shapes × the operations
//! whose outputs are value-independent enough to be reconstructible.
//! Every row asserts the same thing: the result still refuses to decide,
//! through the public `Decide` door, in **both** channels where the
//! channel is observable.
//!
//! The closing test is the control: healthy inputs must still decide.
//! A conservation suite that poisons everything proves nothing.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{
    Band, Bounds, Decide, DualInterval, Indeterminate, Interval, MarginDiag, Real, Sign,
};

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

/// The three poison shapes, by name, as bare `Interval`s.
fn poison_shapes() -> Vec<(&'static str, Interval)> {
    vec![
        ("NaI", Interval::from_f64(f64::NAN)),
        // Full domain miss.
        ("Empty", Interval::from_bounds(-4.0, -1.0).sqrt()),
        // The dangerous one: plausible bounds [0, 2], poisoned decoration.
        ("clamped Trv", Interval::from_bounds(-1.0, 4.0).sqrt()),
    ]
}

#[track_caller]
fn assert_refuses(name: &str, x: Interval) {
    match x.sign_within(band()) {
        Err(Indeterminate {
            margin: MarginDiag::Invalid,
            ..
        }) => {}
        other => panic!("{name}: LAUNDERED — expected poison-refusal, got {other:?}"),
    }
}

#[track_caller]
fn assert_decides(name: &str, x: Interval, expect: Sign) {
    match x.sign_within(band()) {
        Ok(s) if s == expect => {}
        other => panic!("{name}: expected Ok({expect:?}), got {other:?}"),
    }
}

/// `Dual`'s `Decide` delegates to the value channel, so a poisoned VALUE
/// must refuse whatever the derivative looks like — including through
/// the operations that rebuild a value from constants.
#[test]
fn value_channel_poison_survives_every_value_reconstructing_op() {
    type Op = fn(DualInterval) -> DualInterval;
    let ops: Vec<(&str, Op)> = vec![
        // The classic laundering door: x⁰ = 1 regardless of x.
        ("powi(0)", |d| d.powi(0)),
        ("powi(1)", |d| d.powi(1)),
        ("powi(2)", |d| d.powi(2)),
        // Value-shape-discarding ops.
        ("abs", Real::abs),
        ("floor", Real::floor),
        ("neg", |d| -d),
        ("sqrt", Real::sqrt),
        ("sin", Real::sin),
        ("cos", Real::cos),
        ("atan", Real::atan),
        // Arithmetic against a clean operand.
        ("mul by zero", |d| {
            d * DualInterval::constant(Interval::zero())
        }),
        ("sub self", |d| d - d),
        ("div by self", |d| d / d),
    ];
    for (shape, p) in poison_shapes() {
        for (name, op) in &ops {
            // Poison in the value channel, clean derivative.
            let d = DualInterval::new(p, Interval::one());
            let r = op(d);
            assert_refuses(&format!("{name} of {shape} (value channel)"), r.value);
        }
    }
}

/// The kink selectors are the subtle half: they choose or hull a
/// *derivative* by comparing *values*, so a poisoned steering value must
/// cap the tangent's trustworthiness. The selectors themselves are
/// crate-internal, so they are driven here exactly as a consumer reaches
/// them — through `Dual<Interval>`'s `abs`, `floor`, `min`, `max` and
/// `copysign` — and the assertions are on the DERIVATIVE channel, which
/// `Decide` does not look at. A poisoned tangent that came back clean
/// would be invisible to the value-channel tests above.
#[test]
fn kink_selectors_never_launder_the_steering_values_poison() {
    let clean_dual = DualInterval::variable(Interval::from_bounds(1.0, 2.0));
    for (shape, p) in poison_shapes() {
        // Poison in the VALUE that steers the selector, clean derivative:
        // the selection was made by comparing untrustworthy values, so the
        // tangent it produced is untrustworthy too.
        let steered = DualInterval::new(p, Interval::one());
        for (name, r) in [
            ("abs", steered.abs()),
            ("floor", steered.floor()),
            ("min(poison, clean)", Real::min(steered, clean_dual)),
            ("min(clean, poison)", Real::min(clean_dual, steered)),
            ("max(poison, clean)", Real::max(steered, clean_dual)),
            ("max(clean, poison)", Real::max(clean_dual, steered)),
            (
                "copysign(poison, clean)",
                Real::copysign(steered, clean_dual),
            ),
            (
                "copysign(clean, poison)",
                Real::copysign(clean_dual, steered),
            ),
        ] {
            assert_refuses(&format!("{name} deriv steered by {shape}"), r.deriv);
        }
    }
}

/// Poison in the DERIVATIVE channel must reach the derivative channel of
/// every result — the direction that is easy to lose, since most
/// operations' value channels never touch `deriv`.
#[test]
fn derivative_channel_poison_survives_the_chain_rule() {
    for (shape, p) in poison_shapes() {
        let d = DualInterval::new(Interval::from_bounds(1.0, 2.0), p);
        // The value channel is clean and must still decide...
        assert_decides(
            &format!("{shape}: clean value still decides"),
            d.value,
            Sign::Positive,
        );
        // ...while every derivative that consumed the poisoned one stays
        // poisoned.
        for (name, r) in [
            ("mul", d * DualInterval::variable(Interval::from_f64(2.0))),
            ("add", d + DualInterval::variable(Interval::from_f64(2.0))),
            ("sqrt", d.sqrt()),
            ("sin", d.sin()),
            ("powi(2)", d.powi(2)),
            ("abs", d.abs()),
        ] {
            assert_refuses(&format!("{name} of deriv-{shape}"), r.deriv);
        }
    }
}

/// `powi(0)` deserves its own row, and it is the one place this file
/// finds a real gap — in the DERIVATIVE channel, and **not** one the M5
/// PR 1 backend swap introduced.
///
/// `Dual::powi` (crates/geom-core/src/dual.rs) computes
/// `deriv = T::zero()` unconditionally when `n == 0`, because d(x⁰)/dx
/// is the constant 0. That is mathematically right and it is also a
/// laundering door: the zero is a *fresh constant*, so a NaI or `Trv`
/// input yields a derivative that classifies cleanly. The value channel
/// does propagate poison (pinned below and in
/// `m5_pr1_poison_conservation.rs`), and `Decide` for `Dual` reads only
/// the value channel, so nothing in the kernel currently branches on
/// this — but a consumer that certified a derivative would be reading a
/// laundered one.
///
/// This is backend-independent M0-era `Dual` behavior, unchanged by the
/// swap, and fixing it is a `Dual` contract question rather than an
/// interval one. It is pinned here **as it actually behaves** so that
/// the gap is recorded rather than implied away, and so that a future
/// fix has to come past this test deliberately.
#[test]
fn powi_zero_launders_the_derivative_channel_pre_existing() {
    for (shape, p) in poison_shapes() {
        let both = DualInterval::new(p, p).powi(0);
        // Value channel: poison conserved, as contracted.
        assert_refuses(&format!("powi(0) value of {shape}"), both.value);
        // Derivative channel: the fresh zero classifies. Pinned as the
        // known gap described above — NOT an endorsement.
        assert_eq!(
            both.deriv.sign_within(band()),
            Ok(Sign::Zero),
            "powi(0) deriv of {shape}: pre-existing Dual laundering door \
             (dual.rs `n == 0` returns a fresh T::zero()); if this now \
             refuses, the door was closed — update this test and the \
             comment above it"
        );
        // Value clean, derivative poisoned: the value is [1,1] and DOES
        // decide (x⁰ really is 1); the derivative is the same fresh zero.
        let dv = DualInterval::new(Interval::from_bounds(1.0, 2.0), p).powi(0);
        assert_decides(
            &format!("powi(0) clean value of {shape}"),
            dv.value,
            Sign::Positive,
        );
    }
}

/// Every OTHER exponent conserves derivative poison — the contrast that
/// shows the `n == 0` gap above is specific, not systemic.
#[test]
fn nonzero_exponents_conserve_derivative_poison() {
    for (shape, p) in poison_shapes() {
        for n in [1i32, 2, 3, -1, -2] {
            let d = DualInterval::new(Interval::from_bounds(1.0, 2.0), p).powi(n);
            assert_refuses(&format!("powi({n}) deriv of {shape}"), d.deriv);
        }
    }
}

/// Control: the healthy rows still decide, in both channels. Without
/// this, every assertion above is satisfiable by poisoning everything.
#[test]
fn healthy_duals_still_decide_in_both_channels() {
    let x = DualInterval::variable(Interval::from_f64(2.0));
    for (name, r) in [
        ("identity", x),
        ("powi(0)", x.powi(0)),
        ("powi(2)", x.powi(2)),
        ("sqrt", x.sqrt()),
        ("abs", x.abs()),
        ("mul", x * DualInterval::constant(Interval::from_f64(3.0))),
        ("add", x + DualInterval::constant(Interval::from_f64(3.0))),
    ] {
        assert_decides(&format!("{name} value"), r.value, Sign::Positive);
        // The derivative is a real enclosure, not poison: it must
        // classify (to Zero for the constant-derivative cases, Positive
        // otherwise) rather than refuse.
        assert!(
            r.deriv.sign_within(band()).is_ok(),
            "{name}: healthy derivative must classify, got {:?}",
            r.deriv.sign_within(band())
        );
        assert!(!r.deriv.lo().is_nan(), "{name}: healthy derivative is NaN");
    }
    // The kink selectors on clean inputs, through the same public door.
    let a = DualInterval::variable(Interval::from_bounds(1.0, 2.0));
    let b = DualInterval::constant(Interval::from_bounds(5.0, 6.0));
    for (name, r) in [
        ("abs", a.abs()),
        ("min separated", Real::min(a, b)),
        ("max separated", Real::max(a, b)),
        ("copysign", Real::copysign(a, b)),
    ] {
        assert!(
            r.deriv.sign_within(band()).is_ok(),
            "{name}: clean steering must yield a classifying tangent, got {:?}",
            r.deriv.sign_within(band())
        );
    }
    // A plateau's floor jacobian is exactly zero — a definite verdict,
    // not a refusal.
    let plateau = DualInterval::variable(Interval::from_bounds(2.2, 2.8));
    assert_eq!(
        plateau.floor().deriv.sign_within(band()),
        Ok(Sign::Zero),
        "a plateau's floor jacobian is exactly zero"
    );
}
