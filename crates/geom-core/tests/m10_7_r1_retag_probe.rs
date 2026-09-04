//! **M10-7 R1: the K re-tag door under a scalar that records nothing.**
//! `Sym<Interval>` in a `probe` build calls `retag_symbolic_zero` on a
//! symbolic Zero, but `Interval::sign_within` pushes no sample, so the
//! re-tag lands on whatever sample is LAST in the sink.
#![cfg(all(feature = "probe", feature = "interval"))]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::{SampleOutcome, decide, start_recording, take_samples};
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session;
use geom_core::{Interval, ParamSymbol, Probe, Real, Sym, SymBudget, Tol};

#[test]
fn r1_a_symbolic_zero_at_sym_interval_retags_an_unrelated_probe_sample() {
    let band = Band::linear(Tol::witness()).unwrap();
    start_recording();
    // An unrelated, DEFINITE POSITIVE sample from the recording scalar.
    let pos = decide("unrelated_positive", Margin::of(Probe(1.0)), band);
    assert_eq!(pos, Ok(Sign::Positive));
    // Then a symbolic Zero at `Sym<Interval>`, which records nothing.
    let (out, counts) = with_session(
        SymBudget {
            max_terms: 4096,
            max_degree: 128,
        },
        || {
            let x: Sym<Interval> = Sym::param(ParamSymbol::of("x"), Interval::from_f64(0.5));
            decide("identity_at_interval", Margin::of(x - x), band)
        },
    );
    assert_eq!(out, Ok(Sign::Zero));
    assert_eq!(counts.symbolic_zero, 1);
    let samples = take_samples();
    assert_eq!(samples.len(), 1, "Interval records nothing: {samples:?}");
    println!("the unrelated sample now reads: {:?}", samples[0]);
    assert_eq!(
        (samples[0].predicate, samples[0].outcome),
        ("unrelated_positive", SampleOutcome::Definite(Sign::Positive)),
        "a symbolic Zero at Sym<Interval> must not re-tag another scalar's sample"
    );
}
