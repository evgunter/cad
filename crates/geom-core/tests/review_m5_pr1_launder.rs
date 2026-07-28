//! **Poison-laundering attempts over the public `Dual<Interval>`
//! surface.** Adopted from the M5 PR 1 adversarial review's scratch
//! harness (its F3), an independent derivation kept for its regression
//! value; `m5_pr1_poison_conservation.rs` pins the same contract at the
//! bare `Interval` scalar.
//!
//! Attempts to launder poison (Trv/Empty/NaI) into a deciding verdict
//! through every value-independent or freshly-constructed-interval path:
//! the kink selectors (via the public Dual<Interval> ops that call them),
//! copysign, and the value-independent ops (powi(0)/abs/floor). Those
//! paths are the dangerous ones precisely because they build FRESH
//! intervals — `[1,1]`, `[0,0]`, `ENTIRE` — that owe nothing to the
//! input's history unless the implementation puts it back.
//!
//! The final test is an ADDITION beyond the review harness: running it
//! surfaced a real laundering door in `Dual::powi(0)`'s derivative
//! channel. See its own comment — it is pre-existing and
//! backend-independent, so it is pinned as-is rather than fixed here.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Decide, Indeterminate, MarginDiag, Sign};
use geom_core::{Bounds, Dual, Interval, Real};

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

#[track_caller]
fn assert_still_poisoned(name: &str, x: Interval) {
    match x.sign_within(band()) {
        Err(Indeterminate {
            margin: MarginDiag::Invalid,
            ..
        }) => {}
        other => panic!("LAUNDERED at {name}: {other:?}"),
    }
}

fn clamped() -> Interval {
    Interval::from_bounds(-1.0, 4.0).sqrt() // [0,2] @ Trv
}
fn nai() -> Interval {
    Interval::from_f64(f64::NAN)
}
fn empty() -> Interval {
    Interval::from_bounds(-4.0, -1.0).sqrt()
}
fn h(x: f64) -> Interval {
    Interval::from_f64(x)
}
fn dvar(v: Interval) -> Dual<Interval> {
    Dual::new(v, h(1.0))
}

#[test]
fn laundering_attempts_all_fail() {
    for (tag, p) in [("Trv", clamped()), ("NaI", nai()), ("Empty", empty())] {
        // Value-independent results must not resurrect a decoration.
        assert_still_poisoned(&format!("{tag}: powi(0)"), p.powi(0));
        assert_still_poisoned(&format!("{tag}: abs"), p.abs());
        assert_still_poisoned(&format!("{tag}: floor"), p.floor());
        // Kink selectors build FRESH intervals ([1,1], [0,0], ENTIRE) —
        // the classic laundering vector. Reach them through the public
        // Dual<Interval> ops; BOTH channels must stay poisoned.
        let d = dvar(p);
        for (op, r) in [
            ("Dual::abs", d.abs()),
            ("Dual::floor", d.floor()),
            ("Dual::min", Real::min(d, dvar(h(5.0)))),
            ("Dual::min other-side", Real::min(dvar(h(5.0)), d)),
            ("Dual::max", Real::max(d, dvar(h(-5.0)))),
            ("Dual::copysign mag", d.copysign(dvar(h(2.0)))),
            ("Dual::copysign sign", dvar(h(2.0)).copysign(d)),
            (
                "Dual::copysign straddling sign",
                d.copysign(Dual::new(Interval::from_bounds(-1.0, 1.0), h(0.0))),
            ),
        ] {
            assert_still_poisoned(&format!("{tag}: {op}.value"), r.value);
            assert_still_poisoned(&format!("{tag}: {op}.deriv"), r.deriv);
        }
        // Poison confined to the DERIVATIVE channel: the value channel
        // may stay healthy (that is the design), but the deriv channel
        // must not shed it through a selector's fresh interval.
        let dp = Dual::new(h(2.0), p);
        for (op, r) in [
            ("deriv-poison Dual::abs", dp.abs()),
            ("deriv-poison Dual::min", Real::min(dp, dvar(h(5.0)))),
            ("deriv-poison Dual::copysign", dp.copysign(dvar(h(3.0)))),
        ] {
            assert_still_poisoned(&format!("{tag}: {op}.deriv"), r.deriv);
        }
    }

    // Healthy sanity rows: the paths above do not poison healthy inputs.
    let m = Real::min(dvar(h(2.0)), dvar(h(5.0)));
    assert!(m.value.sign_within(band()).is_ok());
    assert!(m.deriv.sign_within(band()).is_ok());
    // Bounds stays poison-visible: NaN brackets for empty AND NaI.
    assert!(empty().lo().is_nan() && empty().hi().is_nan());
    assert!(nai().lo().is_nan() && nai().hi().is_nan());
}

/// **Beyond the review harness** (found while adopting it): the one
/// laundering door that is actually open, and why it is pinned rather
/// than fixed here.
///
/// `Dual::powi` (crates/geom-core/src/dual.rs) sets `deriv = T::zero()`
/// unconditionally when `n == 0`, because d(x⁰)/dx is the constant 0.
/// Mathematically right, and also a fresh constant: a NaI or `Trv` input
/// yields a derivative that classifies cleanly. The VALUE channel does
/// propagate poison (asserted above), and `Decide for Dual` reads only
/// the value channel, so nothing in the kernel branches on this today —
/// but a consumer that certified a derivative would be reading a
/// laundered one.
///
/// This is M0-era `Dual` behavior, untouched by the M5 PR 1 backend swap
/// (dual.rs has no diff in this PR), and closing it is a `Dual` contract
/// question rather than an interval one. Pinned **as it actually
/// behaves** so the gap is recorded rather than implied away, and so a
/// future fix has to come past this test deliberately.
#[test]
fn powi_zero_launders_the_derivative_channel_pre_existing() {
    for (tag, p) in [("Trv", clamped()), ("NaI", nai()), ("Empty", empty())] {
        let both = Dual::new(p, p).powi(0);
        // Value channel: poison conserved, as contracted.
        assert_still_poisoned(&format!("{tag}: Dual::powi(0).value"), both.value);
        // Derivative channel: the fresh zero classifies. NOT an
        // endorsement — if this ever refuses, the door was closed;
        // update this test and the comment above it.
        assert_eq!(
            both.deriv.sign_within(band()),
            Ok(Sign::Zero),
            "{tag}: Dual::powi(0).deriv — known pre-existing laundering door"
        );
    }
    // The contrast that shows the gap is specific, not systemic: every
    // other exponent conserves derivative poison.
    for (tag, p) in [("Trv", clamped()), ("NaI", nai()), ("Empty", empty())] {
        for n in [1i32, 2, 3, -1, -2] {
            let d = Dual::new(h(2.0), p).powi(n);
            assert_still_poisoned(&format!("{tag}: Dual::powi({n}).deriv"), d.deriv);
        }
    }
}
