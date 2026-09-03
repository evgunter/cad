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

/// **The door found while adopting the review harness — now CLOSED**
/// (#126, Ev's option (a), fixed in the M5 PR 4 fix pass).
///
/// `Dual::powi` at `n == 0` used to set `deriv = T::zero()`
/// unconditionally — mathematically the constant 0, but a *fresh*
/// constant: a NaI or `Trv` input yielded a derivative that classified
/// cleanly (this test pinned that laundering while it stood). The fix
/// routes the zero through `powi_zero_deriv_factor(value) · deriv`
/// (dual.rs), so the tangent is still exactly zero for every
/// describable value but inherits the VALUE channel's poison —
/// poison-in-poison-out per channel pair. This test now pins the FIXED
/// behavior: both channels of a poisoned `powi(0)` stay poisoned.
#[test]
fn powi_zero_conserves_both_channels_poison_fixed_126() {
    for (tag, p) in [("Trv", clamped()), ("NaI", nai()), ("Empty", empty())] {
        let both = Dual::new(p, p).powi(0);
        // Value channel: poison conserved, as always contracted.
        assert_still_poisoned(&format!("{tag}: Dual::powi(0).value"), both.value);
        // Derivative channel: poison conserved too (the #126 fix) —
        // the zero is tainted by the value's poison, never fresh.
        assert_still_poisoned(&format!("{tag}: Dual::powi(0).deriv"), both.deriv);
    }
    // A healthy pair keeps the honest exact-zero tangent (the fix must
    // not manufacture poison for describable values).
    let clean = dvar(h(2.0)).powi(0);
    assert_eq!(clean.deriv.sign_within(band()), Ok(Sign::Zero));
    assert!(clean.value.sign_within(band()).is_ok());
    // The nonzero-exponent contrast (kept from the pre-fix pin): every
    // other exponent conserves derivative poison too.
    for (tag, p) in [("Trv", clamped()), ("NaI", nai()), ("Empty", empty())] {
        for n in [1i32, 2, 3, -1, -2] {
            let d = Dual::new(h(2.0), p).powi(n);
            assert_still_poisoned(&format!("{tag}: Dual::powi({n}).deriv"), d.deriv);
        }
    }
}
