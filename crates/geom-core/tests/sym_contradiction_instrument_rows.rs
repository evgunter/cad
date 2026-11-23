//! **The two channels, checked where an answer is returned** (ERROR-DESIGN
//! E12): planted rows for the enclosure check on a symbolic `Zero` at
//! `Sym<Interval>` and for the test instrument that asks the tier about
//! the margins the numbers decided (`geom_core::sym::contradiction`).
//!
//! The contradictions are planted with one parameter symbol bound at two
//! values: the symbol's form is one indeterminate, so `x₁ − x₂` is the
//! zero form while its value channel reads `x₁`'s number minus `x₂`'s.
//! Each row names the change that turns it red.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::contradiction::{self, Event, Report};
use geom_core::sym::revalue::record;
use geom_core::sym::{Rung, with_session_retry};
use geom_core::{
    Decide, Interval, ParamSymbol, Real, SpanLocate, Sym, SymBudget, SymCounts, SymRetry, SymRules,
};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::new(1.0e-9, 1.0e-8).unwrap()
}

/// `x` bound at the point `v`.
fn x_at(v: f64) -> Sym<Interval> {
    Sym::param(
        ParamSymbol::new(test_utils::symbol_id("x")),
        Interval::from_bounds(v, v),
    )
}

/// `c^5` as a left-associated chain less `c²·c³`, `c = 1/3`: the zero
/// form only past the shipped ring's 256 bits (`sym_9_retry_rows`), so
/// the first attempt freezes the two spellings apart and a 512-bit retry
/// closes it.
fn past_the_ring() -> Sym<Interval> {
    let c = || Sym::<Interval>::from_f64(1.0 / 3.0);
    let chain = |k: usize| (1..k).fold(c(), |acc, _| acc * c());
    chain(5) - chain(2) * chain(3)
}

fn ring_512() -> SymRetry {
    SymRetry {
        bits: Some(512),
        ..SymRetry::none()
    }
}

/// One decision of `build`'s margin in a session with `retry`, and the
/// instrument's report of it when `instrument`.
fn decide(
    retry: SymRetry,
    instrument: bool,
    build: impl FnOnce() -> Sym<Interval>,
) -> (Option<Sign>, SymCounts, Option<Report>) {
    let ((sign, report), counts) = with_session_retry(budget(), SymRules::shipped(), retry, || {
        let (out, decisions) =
            record(|| geom_core::k_stats::decide("sym_contradiction", Margin::of(build()), band()));
        let report = instrument.then(|| contradiction::check(&decisions).expect("in session"));
        (out.ok(), report)
    });
    (sign, counts, report)
}

fn found(report: &Report) -> Vec<(Event, Rung, u8)> {
    report
        .contradictions
        .iter()
        .map(|c| (c.event, c.rung, c.attempt))
        .collect()
}

/// **The algebra against a certified enclosure.** `x(1) − x(2)` reads
/// `[-1, -1]`, a definite sign, and its form is zero at the plain rung:
/// the instrument reports [`Event::Enclosure`]. The decision returns the
/// numeric sign and the tier is asked nothing for it. Red if `check`
/// reads the witness as inexact (it would say [`Event::Rounded`]), or
/// stops asking the algebra.
#[test]
fn an_algebraic_zero_under_a_certified_sign_is_an_enclosure_contradiction() {
    let (sign, counts, report) = decide(SymRetry::none(), true, || x_at(1.0) - x_at(2.0));
    let report = report.unwrap();
    assert_eq!(sign, Some(Sign::Negative), "the numeric sign is the answer");
    assert_eq!((counts.symbolic, counts.numeric), (0, 1), "{counts:?}");
    assert_eq!(report.checked, 1, "{report:?}");
    assert_eq!(found(&report), [(Event::Enclosure, Rung::Plain, 0)]);
}

/// **A retry's zero under a certified sign**: `c⁵ − c²·c³` plus the
/// planted `x(1) − x(2)` is zero only at the 512-bit retry, so the
/// instrument, which offers the whole ladder, reports it at attempt 1,
/// and its walks leave the session's receipt as a run without it reads.
/// Red if the instrument's ladder stops after the first attempt (no
/// report), or if `check` stops restoring the receipt (`retried` moves).
#[test]
fn a_retrys_zero_under_a_certified_sign_is_reported_at_its_attempt() {
    let planted = || past_the_ring() + (x_at(1.0) - x_at(2.0));
    let (sign, counts, report) = decide(ring_512(), true, planted);
    let report = report.unwrap();
    assert_eq!(sign, Some(Sign::Negative));
    assert_eq!(
        found(&report),
        [(Event::Enclosure, Rung::Early, 1)],
        "{report:?}"
    );
    let (_, bare, _) = decide(ring_512(), false, planted);
    assert_eq!(counts, bare, "the instrument writes nothing to the receipt");
    // Without the ladder the same margin is no zero at any rung.
    let (_, _, report) = decide(SymRetry::none(), true, planted);
    assert_eq!(found(&report.unwrap()), []);
}

/// **A margin that is not zero is reported by no one**: `x(1) + x(2)`.
#[test]
fn a_margin_that_is_not_zero_is_no_contradiction() {
    let (sign, _, report) = decide(SymRetry::none(), true, || x_at(1.0) + x_at(2.0));
    let report = report.unwrap();
    assert_eq!(sign, Some(Sign::Positive));
    assert_eq!((report.checked, found(&report)), (1, vec![]));
}

/// **A zero the tier returns lies in its own enclosure** — the planted
/// breach: `x(1) − x(1 + 5e-10)` reads `[-5e-10, -5e-10]`, inside the
/// zero band, so the numeric channel answers no definite sign and the
/// tier proves the form zero over an enclosure that excludes 0. Red if
/// the check on a proven zero (`sym::discharge_checked`) goes.
#[test]
#[cfg_attr(not(debug_assertions), ignore = "the check is a debug_assert!")]
#[should_panic(expected = "at the Plain rung (attempt 0) and its certified enclosure")]
fn a_proven_zero_whose_enclosure_excludes_zero_panics() {
    let _ = decide(SymRetry::none(), false, || x_at(1.0) - x_at(1.0 + 5.0e-10));
}

/// **And at a retry**: the same breach where only the 512-bit attempt
/// proves the zero. Red if the check is taken on the first attempt's
/// zeros alone.
#[test]
#[cfg_attr(not(debug_assertions), ignore = "the check is a debug_assert!")]
#[should_panic(expected = "(attempt 1) and its certified enclosure")]
fn a_retrys_zero_whose_enclosure_excludes_zero_panics() {
    let _ = decide(ring_512(), false, || {
        past_the_ring() + (x_at(1.0) - x_at(1.0 + 5.0e-10))
    });
}

/// **`Decide::free_identity` is the exception**, at the scalar: a
/// measured bound `[5e-10, 6e-10]`, in the zero band and clear of 0,
/// carrying an identity that discharges. The tier proves it zero, the
/// check reads the identity's free parameter and passes it, and the
/// instrument leaves it out under a definite sign too. Red if the check
/// stops reading the DAG for `SymOp::Free` (the first case panics), or
/// the instrument stops leaving such margins out (the second reports).
#[test]
fn a_free_identity_zero_is_checked_against_no_enclosure() {
    let identity = |at: f64| {
        move || {
            Sym::<Interval>::from_f64(at)
                .enclosure_hull(Sym::from_f64(at + 1.0e-10))
                .free_identity(|free| {
                    let t = free(Sym::from_f64(0.5));
                    t - t
                })
        }
    };
    let (sign, counts, _) = decide(SymRetry::none(), false, identity(5.0e-10));
    assert_eq!(
        (sign, counts.symbolic),
        (Some(Sign::Zero), 1),
        "proven zero over a bound clear of 0: {counts:?}"
    );
    let (sign, _, report) = decide(SymRetry::none(), true, identity(1.0));
    let report = report.unwrap();
    assert_eq!(sign, Some(Sign::Positive));
    assert_eq!((report.free, found(&report)), (1, vec![]), "{report:?}");
}

/// The instrument outside a session has nothing to read.
#[test]
fn the_instrument_outside_a_session_answers_none() {
    let (_, decisions) = record(|| x_at(1.0).sign_within(band()).map(|d| d.sign));
    assert_eq!(contradiction::check(&decisions), None);
}

/// **A deferring session's reader is handed no zero its enclosure
/// excludes**: the planted breach taken in a deferring session, where the
/// decision is the numeric `Zero` and the tier is asked only when a
/// reader discharges it (`deferred::discharge`). Red if the deferred
/// discharge skips the check.
#[test]
#[cfg_attr(not(debug_assertions), ignore = "the check is a debug_assert!")]
#[should_panic(expected = "at the Plain rung (attempt 0) and its certified enclosure")]
fn a_deferred_discharge_whose_enclosure_excludes_zero_panics() {
    use geom_core::sym::deferred::{discharge, with_deferred_session};
    let _ = with_deferred_session(budget(), SymRules::shipped(), SymRetry::none(), || {
        let d = geom_core::k_stats::decide_reported(
            "sym_contradiction",
            Margin::of(x_at(1.0) - x_at(1.0 + 5.0e-10)),
            band(),
        )
        .expect("in the zero band");
        discharge(d.margin)
    });
}

/// **`Sym<Dual<Interval>>` is an exact witness, and its zeros are checked
/// too**: `Dual` reads its enclosure off its value channel, which is what
/// it decides on. Red if `Dual` reads no enclosure (the check then fires
/// on the missing one) or the check skips this scalar.
#[test]
#[cfg_attr(not(debug_assertions), ignore = "the check is a debug_assert!")]
#[should_panic(expected = "at the Plain rung (attempt 0) and its certified enclosure")]
fn a_zero_at_sym_dual_interval_whose_enclosure_excludes_zero_panics() {
    type D = geom_core::Dual<Interval>;
    let x = |v: f64| -> Sym<D> {
        Sym::param(
            ParamSymbol::new(test_utils::symbol_id("x")),
            D::constant(Interval::from_bounds(v, v)),
        )
    };
    let _ = with_session_retry(budget(), SymRules::shipped(), SymRetry::none(), || {
        geom_core::k_stats::decide(
            "sym_contradiction",
            Margin::of(x(1.0) - x(1.0 + 5.0e-10)),
            band(),
        )
    });
}

/// Two registrations the witness admits (`lo = a − 0.2` and `hi = a +
/// 0.2` each meet `a`'s enclosure at an endpoint), so the registry makes
/// `lo − hi` the zero form over `[-0.6, -0.2]`.
fn registered_apart() -> (Sym<Interval>, Sym<Interval>) {
    let a = Sym::param(
        ParamSymbol::new(test_utils::symbol_id("a")),
        Interval::from_bounds(0.9, 1.1),
    );
    let lo = a - Sym::from_f64(0.2);
    let hi = a + Sym::from_f64(0.2);
    let tol = geom_core::Tol::witness();
    assert_eq!(
        (lo.register_equal(a, tol), hi.register_equal(a, tol)),
        (
            geom_core::sym::SymRegistration::Recorded,
            geom_core::sym::SymRegistration::Recorded
        ),
        "the touching enclosures satisfy the witness"
    );
    (lo, hi)
}

/// **The check at the registry door's rung**: the registered `lo − hi`
/// scaled into the zero band (`[-6e-11, -2e-11]`), a zero only the door
/// proves, over an enclosure clear of 0. Red if the check is not taken on
/// the door's zeros.
#[test]
#[cfg_attr(not(debug_assertions), ignore = "the check is a debug_assert!")]
#[should_panic(expected = "at the Door rung (attempt 0) and its certified enclosure")]
fn a_doors_zero_whose_enclosure_excludes_zero_panics() {
    let _ = with_session_retry(budget(), SymRules::shipped(), SymRetry::none(), || {
        let (lo, hi) = registered_apart();
        geom_core::k_stats::decide(
            "sym_contradiction",
            Margin::of((lo - hi) * Sym::from_f64(1.0e-10)),
            band(),
        )
    });
}

/// **A registration only a retry's door reaches, under a definite sign**:
/// `c⁵ − c²·c³` (zero at the 512-bit retry alone) plus the registered
/// `lo − hi`. The instrument reports it at attempt 1. Red if the
/// instrument's door stops at the first attempt.
#[test]
fn a_registration_only_a_retrys_door_reaches_is_reported_at_its_attempt() {
    let ((sign, report), _) = with_session_retry(budget(), SymRules::shipped(), ring_512(), || {
        let (lo, hi) = registered_apart();
        let (out, decisions) = record(|| {
            geom_core::k_stats::decide(
                "sym_contradiction",
                Margin::of(past_the_ring() + (lo - hi)),
                band(),
            )
        });
        (
            out.ok(),
            contradiction::check(&decisions).expect("in session"),
        )
    });
    assert_eq!(sign, Some(Sign::Negative));
    assert_eq!(
        found(&report),
        [(Event::Registration, Rung::Door, 1)],
        "{report:?}"
    );
}

/// **What the free-identity exception reads, today**: a node whose DAG
/// reaches a `SymOp::Free`, not "a margin the door made". Both directions
/// are pinned as the tree's truth (`discharge_checked` says why the one
/// caller reaches neither):
///
/// - a margin built FROM a free identity's result (`f − f` beside a
///   planted breach) is exempt, so its excluding enclosure passes;
/// - an identity whose closure never calls `free` is not recognised, so
///   its bound clear of 0 fires the check.
#[test]
fn a_margin_built_from_a_free_identity_is_exempt_from_the_check() {
    let (sign, counts, _) = decide(SymRetry::none(), false, || {
        let f = Sym::<Interval>::from_f64(5.0e-10)
            .enclosure_hull(Sym::from_f64(6.0e-10))
            .free_identity(|free| {
                let t = free(Sym::from_f64(0.5));
                t - t
            });
        (f - f) + (x_at(1.0) - x_at(1.0 + 5.0e-10))
    });
    assert_eq!(
        (sign, counts.symbolic),
        (Some(Sign::Zero), 1),
        "proven over an enclosure clear of 0, unchecked: {counts:?}"
    );
}

/// The other direction ([`a_margin_built_from_a_free_identity_is_exempt_from_the_check`]).
#[test]
#[cfg_attr(not(debug_assertions), ignore = "the check is a debug_assert!")]
#[should_panic(expected = "certified enclosure [5e-10, 6e-10] excludes 0")]
fn an_identity_that_never_mints_its_variable_is_checked() {
    let _ = decide(SymRetry::none(), false, || {
        Sym::<Interval>::from_f64(5.0e-10)
            .enclosure_hull(Sym::from_f64(6.0e-10))
            .free_identity(|_| x_at(1.0) - x_at(1.0))
    });
}

/// **A frozen walk reads as agreement**: the planted contradiction
/// `(x(1) + 1)² − (x(2)² + 2·x(2) + 1)` (zero form, value −5) at a
/// budget of one term freezes `x + 1` before the two sides can cancel,
/// so the instrument checks the decision and reports nothing. What that report
/// speaks for is the instrument's module doc's first item.
#[test]
fn a_frozen_walk_is_checked_and_reports_nothing() {
    let starved = SymBudget {
        max_terms: 1,
        max_degree: 2,
    };
    let one = || Sym::<Interval>::from_f64(1.0);
    let planted = move || {
        (x_at(1.0) + one()) * (x_at(1.0) + one())
            - (x_at(2.0) * x_at(2.0) + Sym::from_f64(2.0) * x_at(2.0) + one())
    };
    let ((sign, report), counts) =
        geom_core::sym::with_session_rules(starved, SymRules::shipped(), || {
            let (out, decisions) = record(|| {
                geom_core::k_stats::decide("sym_contradiction", Margin::of(planted()), band())
            });
            (
                out.ok(),
                contradiction::check(&decisions).expect("in session"),
            )
        });
    assert_eq!(sign, Some(Sign::Negative));
    assert_eq!(counts.frozen, 0, "the decision built no form: {counts:?}");
    assert_eq!(
        (report.checked, found(&report)),
        (1, vec![]),
        "checked, frozen, and silent: {report:?}"
    );
    let (_, _, full) = decide(SymRetry::none(), true, planted);
    assert_eq!(
        found(&full.unwrap()).len(),
        1,
        "at the shipped budget it reports"
    );
}

/// **The instrument is asked last**: a decision in the session after
/// `contradiction::check` would read the forms the instrument walked, so
/// the session refuses it. Red if a decision may follow the instrument.
#[test]
#[cfg_attr(not(debug_assertions), ignore = "the guard is a debug_assert!")]
#[should_panic(expected = "a decision after `contradiction::check`")]
fn a_decision_after_the_instrument_panics() {
    let _ = with_session_retry(budget(), SymRules::shipped(), SymRetry::none(), || {
        let (_, decisions) = record(|| {
            geom_core::k_stats::decide(
                "sym_contradiction",
                Margin::of(x_at(1.0) - x_at(2.0)),
                band(),
            )
        });
        let _ = contradiction::check(&decisions);
        geom_core::k_stats::decide(
            "sym_contradiction",
            Margin::of(x_at(1.0) + x_at(2.0)),
            band(),
        )
    });
}

/// **A retry the cap withholds is counted**: past the instrument's cap on
/// a retry attempt's memo the attempt is not offered, and the report
/// says how many decisions went unasked at it. Two definite margins past
/// the ring with a retry cap of one form: the first is asked at the
/// retry; the second finds the attempt full.
#[test]
fn a_retry_the_cap_withholds_is_counted() {
    let capped = SymRetry {
        bits: Some(512),
        max_forms: 1,
        ..SymRetry::none()
    };
    let ((), report) = {
        let ((r, ()), _) = with_session_retry(budget(), SymRules::shipped(), capped, || {
            let (_, decisions) = record(|| {
                let _ = geom_core::k_stats::decide(
                    "sym_contradiction",
                    Margin::of(past_the_ring() + (x_at(1.0) - x_at(2.0))),
                    band(),
                );
                let _ = geom_core::k_stats::decide(
                    "sym_contradiction",
                    Margin::of(past_the_ring() * Sym::from_f64(3.0) + (x_at(1.0) - x_at(3.0))),
                    band(),
                );
            });
            (contradiction::check(&decisions).expect("in session"), ())
        });
        ((), r)
    };
    assert_eq!(report.checked, 2, "{report:?}");
    assert!(
        report.withheld >= 1,
        "the full attempt is counted: {report:?}"
    );
}
