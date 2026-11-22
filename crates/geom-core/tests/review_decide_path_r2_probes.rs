//! Reviewer probes on the enclosure check and the contradiction
//! instrument (PR 7394, frozen head 539a76e1). Each row states what it
//! found; the ones marked FINDING document a hole, not a contract.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::contradiction;
use geom_core::sym::deferred::{discharge, with_deferred_session};
use geom_core::sym::revalue::record;
use geom_core::sym::with_session_retry;
use geom_core::{Decide, Dual, Interval, Real, SpanLocate, Sym, SymBudget, SymRetry, SymRules};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::new(1.0e-9, 1.0e-8).unwrap()
}

fn iv(lo: f64, hi: f64) -> Interval {
    Interval::from_bounds(lo, hi)
}

/// FINDING (claim 3, "a value minted outside the session"): an opaque
/// value minted before the session takes sequence 0; the session resets
/// the sequence, so its first opaque value has the SAME node id. Their
/// difference is the zero form while the value channel reads two
/// different enclosures. With the difference in the zero band the tier
/// answers a proven `Zero` for two different reals, and the enclosure
/// check is what catches it (it panics here). On the merge base the
/// same `Zero` is returned silently.
#[test]
#[should_panic(expected = "its certified enclosure")]
fn an_opaque_minted_outside_the_session_collides_with_the_sessions_first() {
    let outside = Sym::<Interval>::opaque(iv(1.0e-12, 2.0e-12));
    let ((), _) = with_session_retry(budget(), SymRules::shipped(), SymRetry::none(), || {
        let inside = Sym::<Interval>::opaque(iv(4.0e-12, 5.0e-12));
        assert_eq!(outside.node(), inside.node(), "the ids collide");
        let out = geom_core::k_stats::decide("probe_collide", Margin::of(outside - inside), band());
        panic!("returned without the check firing: {out:?}");
    });
}

/// FINDING (claim 3, the deferring session): the same false zero, taken
/// in a deferring session and discharged by a reader, is a proven Zero
/// the tier hands back at `Sym<Interval>` with no enclosure check: the
/// reader is told the two different reals are proven equal.
#[test]
fn a_deferred_discharge_proves_the_collision_with_no_check() {
    let outside = Sym::<Interval>::opaque(iv(1.0e-12, 2.0e-12));
    let (proven, counts) =
        with_deferred_session(budget(), SymRules::shipped(), SymRetry::none(), || {
            let inside = Sym::<Interval>::opaque(iv(4.0e-12, 5.0e-12));
            let d = (outside - inside).sign_within(band()).unwrap();
            assert_eq!(d.sign, Sign::Zero, "the numeric channel reads Zero in the band");
            discharge(d.margin)
        });
    assert!(proven, "the reader is handed a proof: {counts:?}");
}

/// FINDING (claim 3, nested scalar): at `Sym<Dual<Interval>>` the
/// witness is exact but `Dual` does not forward `enclosure_probe`, so
/// the check is skipped and the in-band false zero is returned proven.
#[test]
fn at_sym_dual_interval_the_check_reads_no_enclosure() {
    let (sign, counts) =
        with_session_retry(budget(), SymRules::shipped(), SymRetry::none(), || {
            let x = |v: f64| {
                Sym::<Dual<Interval>>::param(
                    geom_core::ParamSymbol::new(test_utils::symbol_id("x")),
                    Dual::constant(iv(v, v)),
                )
            };
            let m = x(1.0) - x(1.0 + 5.0e-10);
            assert_eq!(<Dual<Interval> as Real>::WITNESS, geom_core::Witness::Exact);
            m.sign_within(band()).map(|d| d.sign)
        });
    assert_eq!(sign, Ok(Sign::Zero), "{counts:?}");
    assert_eq!(counts.symbolic, 1, "the tier proved it: {counts:?}");
}

/// FINDING (claim 4): the exception is "the DAG reaches a `Free`", not
/// "this margin is a `free_identity`". A margin that merely CONTAINS a
/// free identity's result as a sub-term escapes the check: here a
/// planted false zero (`x(1) − x(1 + 5e-10)`) plus `f − f` for a free
/// identity `f`. Its enclosure excludes 0 and it is returned proven.
#[test]
fn a_margin_that_contains_a_free_identity_escapes_the_check() {
    let (sign, counts) =
        with_session_retry(budget(), SymRules::shipped(), SymRetry::none(), || {
            let x = |v: f64| {
                Sym::<Interval>::param(
                    geom_core::ParamSymbol::new(test_utils::symbol_id("x")),
                    iv(v, v),
                )
            };
            let f = Sym::<Interval>::from_f64(0.0).free_identity(|free| {
                let t = free(Sym::from_f64(0.5));
                t - t
            });
            let m = (x(1.0) - x(1.0 + 5.0e-10)) + (f - f);
            let (lo, hi) = m.value().enclosure_probe().unwrap();
            assert!(hi < 0.0, "the enclosure excludes 0: [{lo:e}, {hi:e}]");
            m.sign_within(band()).map(|d| d.sign)
        });
    assert_eq!(sign, Ok(Sign::Zero), "{counts:?}");
}

/// Claim 4, the other direction: a free identity whose closure never
/// calls the minting function carries no `Free` in its DAG, so a
/// measured bound clear of 0 under a discharging identity panics (the
/// check fires on a use the door's docs do not forbid).
#[test]
#[should_panic(expected = "its certified enclosure")]
fn a_free_identity_that_never_mints_its_variable_is_not_recognised() {
    let _ = with_session_retry(budget(), SymRules::shipped(), SymRetry::none(), || {
        let c = Sym::<Interval>::from_f64(0.5);
        Sym::<Interval>::from_f64(5.0e-10)
            .enclosure_hull(Sym::from_f64(6.0e-10))
            .free_identity(|_free| c - c)
            .sign_within(band())
            .map(|d| d.sign)
    });
}

/// Claim 7: the instrument reports nothing about a contradiction whose
/// only proof is on a retry attempt the 20k cap refuses, and its report
/// carries no field saying an attempt was not offered. (Recorded as a
/// reading: the report has `checked`, `free`, `absent`, `contradictions`.)
#[test]
fn the_instruments_report_has_no_cap_field() {
    let r = contradiction::Report::default();
    let contradiction::Report {
        checked: _,
        free: _,
        absent: _,
        contradictions: _,
    } = r;
}

/// Claim 7, "absent": the outside-session collision above is NOT absent
/// to the instrument — the outside node's id is in the session's table
/// (the inside value recorded it) — and under a definite sign the
/// instrument reports an Enclosure contradiction for it.
#[test]
fn the_collision_is_present_to_the_instrument() {
    let outside = Sym::<Interval>::opaque(iv(1.0, 2.0));
    let (report, _) = with_session_retry(budget(), SymRules::shipped(), SymRetry::none(), || {
        let inside = Sym::<Interval>::opaque(iv(4.0, 5.0));
        let (_, decisions) = record(|| {
            geom_core::k_stats::decide("probe_collide", Margin::of(outside - inside), band())
        });
        contradiction::check(&decisions).unwrap()
    });
    assert_eq!(report.absent, 0, "{report:?}");
    assert_eq!(report.contradictions.len(), 1, "{report:?}");
}
