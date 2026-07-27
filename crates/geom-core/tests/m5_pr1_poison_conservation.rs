//! **Poison conservation across the M5 PR 1 backend swap.**
//!
//! The `Interval` scalar's transcendental backend moved from `inari` to
//! the in-repo `interval-transcendentals` crate. Endpoint VALUES were
//! allowed to move (outward pads instead of MPFR-tight rounding); the
//! poison channel was not. This file is the enumerated pin for that: for
//! every operation on the scalar's surface, at a domain-violating or
//! otherwise poisoned input, the wrapper must classify exactly as the
//! inari-backed wrapper did — `Indeterminate` with [`MarginDiag::Invalid`]
//! wherever the decoration falls below `Def`, and a *definite* verdict
//! wherever the pre-swap kernel produced one.
//!
//! Three properties are pinned per function, because the contract has
//! three parts (M0 PR 4, `src/interval.rs` module docs):
//!
//! 1. **Partial domain miss clamps and poisons**: the value stays a
//!    plausible enclosure, and the decoration alone records the
//!    violation, so the result refuses to decide.
//! 2. **Full domain miss goes empty**, which is also poison.
//! 3. **NaI and Empty propagate through every entry point** — poison
//!    never gets laundered by an operation, including the ones (`powi`
//!    with `n == 0`, `abs`, `floor`) whose value would otherwise be
//!    independent of the input.
//!
//! The `Decide` verdict is the load-bearing assertion throughout: this
//! file deliberately checks *decisions*, not endpoints, because
//! decisions are what the swap was forbidden to change.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Bounds, Decide, Indeterminate, Interval, MarginDiag, Real, Sign};

/// The fixed pure band used throughout (identical to `interval.rs`'s
/// `band_1e9`): built via `Band::new`, so this file never touches the
/// global tolerance and may hold many `#[test]`s.
fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

/// Asserts that `x` refuses to classify *because it is poisoned* — the
/// `Invalid` margin, not merely a straddling enclosure.
#[track_caller]
fn assert_poisoned(name: &str, x: Interval) {
    match x.sign_within(band()) {
        Err(Indeterminate {
            margin: MarginDiag::Invalid,
            ..
        }) => {}
        other => panic!("{name}: expected poison-refusal, got {other:?}"),
    }
}

/// Asserts that `x` classifies definitely — the pre-swap verdict for
/// every healthy input in this file.
#[track_caller]
fn assert_decides(name: &str, x: Interval, expect: Sign) {
    match x.sign_within(band()) {
        Ok(s) if s == expect => {}
        other => panic!("{name}: expected Ok({expect:?}), got {other:?}"),
    }
}

fn nai() -> Interval {
    Interval::from_f64(f64::NAN)
}

/// The canonical empty enclosure: a full domain miss.
fn empty() -> Interval {
    Interval::from_bounds(-4.0, -1.0).sqrt()
}

/// A `Trv`-poisoned but perfectly plausible-looking enclosure — the
/// dangerous case the decoration channel exists for. `sqrt([-1, 4])`
/// clamps to `[0, 2]`, whose bounds betray nothing.
fn clamped() -> Interval {
    Interval::from_bounds(-1.0, 4.0).sqrt()
}

#[test]
fn the_poison_fixtures_are_what_they_claim_to_be() {
    assert!(nai().lo().is_nan() && nai().hi().is_nan());
    assert!(empty().lo().is_nan() && empty().hi().is_nan());
    // The clamped one is the interesting fixture: healthy-looking bounds,
    // poisoned decoration.
    assert_eq!((clamped().lo(), clamped().hi()), (0.0, 2.0));
    assert_poisoned("sqrt([-1, 4]) clamp", clamped());
    assert_poisoned("NaI", nai());
    assert_poisoned("Empty", empty());
}

/// Partial domain misses: the value clamps to the domain, and the
/// decoration alone carries the violation. Enumerated over every
/// partial-domain function on the scalar's surface.
#[test]
fn partial_domain_misses_clamp_and_refuse_to_decide() {
    let cases: Vec<(&str, Interval)> = vec![
        ("sqrt([-1, 4])", Interval::from_bounds(-1.0, 4.0).sqrt()),
        ("asin([-2, 0.5])", Interval::from_bounds(-2.0, 0.5).asin()),
        ("asin([0.5, 2])", Interval::from_bounds(0.5, 2.0).asin()),
        ("acos([-2, 0.5])", Interval::from_bounds(-2.0, 0.5).acos()),
        ("acos([0.5, 2])", Interval::from_bounds(0.5, 2.0).acos()),
        // tan over an enclosure containing a pole: no finite range is
        // honest, so the backend refuses with Entire + Trv.
        (
            "tan([1, 2]) over π/2",
            Interval::from_bounds(1.0, 2.0).tan(),
        ),
        // Division by a zero-straddling enclosure.
        (
            "[1, 2] / [-1, 1]",
            Interval::from_bounds(1.0, 2.0) / Interval::from_bounds(-1.0, 1.0),
        ),
        // atan2 over a box containing the origin: undefined at a point
        // of the box.
        (
            "atan2 origin box",
            Interval::from_bounds(-1.0, 1.0).atan2(Interval::from_bounds(-1.0, 1.0)),
        ),
    ];
    for (name, x) in cases {
        assert_poisoned(name, x);
    }
}

/// Full domain misses are empty, and empty is poison.
#[test]
fn full_domain_misses_are_empty_and_refuse_to_decide() {
    let cases: Vec<(&str, Interval)> = vec![
        ("sqrt([-4, -1])", Interval::from_bounds(-4.0, -1.0).sqrt()),
        ("asin([2, 3])", Interval::from_bounds(2.0, 3.0).asin()),
        ("acos([-3, -2])", Interval::from_bounds(-3.0, -2.0).acos()),
        (
            "[1, 2] / [0, 0]",
            Interval::from_bounds(1.0, 2.0) / Interval::zero(),
        ),
        ("atan2(0, 0)", Interval::zero().atan2(Interval::zero())),
    ];
    for (name, x) in cases {
        assert!(x.lo().is_nan() && x.hi().is_nan(), "{name}: not empty");
        assert_poisoned(name, x);
    }
}

/// Every entry point propagates NaI, Empty, and a clamped-`Trv` input —
/// the exhaustive sweep. Unary operations first; each is applied to all
/// three poison fixtures.
#[test]
fn every_unary_operation_conserves_poison() {
    type UnaryOp = fn(Interval) -> Interval;
    let ops: Vec<(&str, UnaryOp)> = vec![
        ("sqrt", Real::sqrt),
        ("abs", Real::abs),
        ("floor", Real::floor),
        ("sin", Real::sin),
        ("cos", Real::cos),
        ("tan", Real::tan),
        ("asin", Real::asin),
        ("acos", Real::acos),
        ("atan", Real::atan),
        ("neg", |x| -x),
        // powi at several exponents, INCLUDING n == 0 — the classic
        // laundering door (x⁰ = 1 would erase the history).
        ("powi(0)", |x| x.powi(0)),
        ("powi(1)", |x| x.powi(1)),
        ("powi(2)", |x| x.powi(2)),
        ("powi(-2)", |x| x.powi(-2)),
        // The projections are defined off sin_cos; pin them too.
        ("sin_cos.0", |x| x.sin_cos().0),
        ("sin_cos.1", |x| x.sin_cos().1),
    ];
    for (name, op) in ops {
        for (label, fixture) in [
            ("NaI", nai()),
            ("Empty", empty()),
            ("clamped Trv", clamped()),
        ] {
            assert_poisoned(&format!("{name} of {label}"), op(fixture));
        }
    }
}

/// Binary operations conserve poison from EITHER side.
#[test]
fn every_binary_operation_conserves_poison_from_either_side() {
    type BinaryOp = fn(Interval, Interval) -> Interval;
    let ops: Vec<(&str, BinaryOp)> = vec![
        ("add", |a, b| a + b),
        ("sub", |a, b| a - b),
        ("mul", |a, b| a * b),
        ("div", |a, b| a / b),
        ("min", Real::min),
        ("max", Real::max),
        ("atan2", Real::atan2),
        ("copysign", Real::copysign),
        ("reduce_periodic", Real::reduce_periodic),
    ];
    // A healthy operand to pair each poison fixture against.
    let healthy = Interval::from_bounds(1.0, 2.0);
    for (name, op) in ops {
        for (label, fixture) in [
            ("NaI", nai()),
            ("Empty", empty()),
            ("clamped Trv", clamped()),
        ] {
            assert_poisoned(&format!("{name}({label}, healthy)"), op(fixture, healthy));
            assert_poisoned(&format!("{name}(healthy, {label})"), op(healthy, fixture));
        }
    }
}

/// The other half of conservation: healthy inputs must still DECIDE.
/// A poison channel that poisons everything conserves poison trivially
/// and is worthless — this is the pin that the swap did not simply make
/// the scalar more pessimistic. Every case here decided definitely under
/// the inari backend and must still.
#[test]
fn healthy_inputs_still_decide_definitely() {
    let one = Interval::from_bounds(1.0, 2.0);
    let neg = Interval::from_bounds(-2.0, -1.0);
    assert_decides(
        "sqrt([1,4])",
        Interval::from_bounds(1.0, 4.0).sqrt(),
        Sign::Positive,
    );
    assert_decides("abs", neg.abs(), Sign::Positive);
    assert_decides(
        "floor([2.2, 2.8])",
        Interval::from_bounds(2.2, 2.8).floor(),
        Sign::Positive,
    );
    // floor across a step is `Def` — honest discreteness, still decides.
    assert_decides(
        "floor([2.5, 3.5])",
        Interval::from_bounds(2.5, 3.5).floor(),
        Sign::Positive,
    );
    assert_decides(
        "sin([0.1, 0.2])",
        Interval::from_bounds(0.1, 0.2).sin(),
        Sign::Positive,
    );
    assert_decides(
        "cos([0.1, 0.2])",
        Interval::from_bounds(0.1, 0.2).cos(),
        Sign::Positive,
    );
    assert_decides(
        "tan([0.1, 0.2])",
        Interval::from_bounds(0.1, 0.2).tan(),
        Sign::Positive,
    );
    assert_decides(
        "asin([0.1, 0.2])",
        Interval::from_bounds(0.1, 0.2).asin(),
        Sign::Positive,
    );
    assert_decides(
        "acos([0.1, 0.2])",
        Interval::from_bounds(0.1, 0.2).acos(),
        Sign::Positive,
    );
    assert_decides(
        "atan([0.1, 0.2])",
        Interval::from_bounds(0.1, 0.2).atan(),
        Sign::Positive,
    );
    assert_decides("atan2(y>0, x>0)", one.atan2(one), Sign::Positive);
    // The D2 case: the closed upper half-plane against x < 0. The value
    // is near +π; the decoration reaches `Com` here (it was `Dac` under
    // inari) and either way this must decide.
    assert_decides(
        "atan2([0, 1], [-2, -1]) (D2)",
        Interval::from_bounds(0.0, 1.0).atan2(Interval::from_bounds(-2.0, -1.0)),
        Sign::Positive,
    );
    assert_decides("powi(2)", neg.powi(2), Sign::Positive);
    assert_decides("powi(0)", neg.powi(0), Sign::Positive);
    assert_decides("powi(-2)", neg.powi(-2), Sign::Positive);
    assert_decides("add", one + one, Sign::Positive);
    assert_decides("sub", one - neg, Sign::Positive);
    assert_decides("mul", neg * neg, Sign::Positive);
    assert_decides("div", one / one, Sign::Positive);
    assert_decides("min", Real::min(one, one), Sign::Positive);
    assert_decides("max", Real::max(neg, one), Sign::Positive);
    assert_decides("copysign", Real::copysign(one, one), Sign::Positive);
    assert_decides(
        "reduce_periodic",
        Real::reduce_periodic(Interval::from_f64(7.0), Interval::from_f64(4.0)),
        Sign::Positive,
    );
}
