//! The public classification doors write ONE recording stream.
//!
//! `decide`, `decide_flagged` and `decide_invariant` share a private
//! `classify`, and the gate doors (`decide_positive`, `decide_nonzero`,
//! `gate_measured`) share the one write to the escalation channel with
//! it, so the predicate-name channel and both verdict channels are
//! written in one place. These suites pin the observable consequence:
//! the same verdicts, in the same order, with each door's own name,
//! whichever door a decision came through.
//!
//! **Order is the property, not presence.** `editor_core::resolve::vdiff`
//! compares two runs' verdict logs POSITIONALLY, so a stream with the
//! right multiset in the wrong order is a wrong diff rather than a slow
//! one. Both fixtures therefore assert an exact vector.
//!
//! These use the public API only — deliberately, so the same suite can
//! be run against a tree where the doors carry three separate bodies and
//! against one where they delegate, and the two compared.
//!
//! It also holds one row that is not about the doors but about the
//! vocabulary they record in: the third of the discharge vocabulary's
//! seam pins,
//! `every_discharge_kind_retags_its_sample_with_a_token_of_its_own`,
//! which is here because its subject
//! (`geom_core::k_stats::SampleOutcome`) exists only under `probe` and
//! this is a probe suite CI RUNS. `sym::discharge_pins` carries the
//! argument for all three rows.
//!
//! **CI EXECUTES BOTH HALVES.** The ungated tests run on every merge;
//! `every_door_names_its_own_sample_for_the_recording_scalar` and the
//! seam row above carry the `probe` gate, and this file is rostered in
//! `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`) and run under the
//! DEFAULT selection by `scripts/k_probe_sweep.sh`, whose tally is floored
//! by `--check-executed`. The name-channel claim is therefore a gate. By
//! hand:
//! `cargo test -p geom-core --features probe --test all -- k_stats_doors::`.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::Tol;
use geom_core::k_stats::{
    Bracket, Escalation, NonzeroSign, Verdict, decide, decide_flagged, decide_invariant,
    decide_nonzero, decide_positive, gate_measured,
};
use geom_core::{Band, Margin, MarginDiag, Sign};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The fixture interleaves the doors deliberately — a run that grouped
/// them would pass under any per-door ordering.
#[test]
fn the_three_doors_share_one_verdict_stream_in_decision_order() {
    let b = band();
    let mid = f64::midpoint(b.zero(), b.escalate());
    let bracket = Bracket::open();
    assert_eq!(decide("door_a", Margin::of(1.0f64), b), Ok(Sign::Positive));
    assert_eq!(decide_invariant("door_b", -1.0f64, b), Ok(Sign::Negative));
    assert_eq!(
        decide_flagged("door_c", 1.0f64, b, "test fixture: door interleaving"),
        Ok(Sign::Positive)
    );
    // One indeterminate per door: escalated outcomes are not verdicts,
    // so none of these may appear or shift the positions after them —
    // they are the frame's OTHER channel, in their own decision order.
    let d = decide("door_d", Margin::of(mid), b).unwrap_err();
    let e = decide_flagged("door_e", mid, b, "test fixture: door interleaving").unwrap_err();
    let f = decide_invariant("door_f", mid, b).unwrap_err();
    assert_eq!(
        decide_flagged("door_g", 0.0f64, b, "test fixture: door interleaving"),
        Ok(Sign::Zero)
    );
    assert_eq!(decide("door_h", Margin::of(-1.0f64), b), Ok(Sign::Negative));
    let recorded = bracket.finish();
    assert_eq!(
        recorded.verdicts,
        vec![
            Verdict {
                predicate: "door_a",
                sign: Sign::Positive
            },
            Verdict {
                predicate: "door_b",
                sign: Sign::Negative
            },
            Verdict {
                predicate: "door_c",
                sign: Sign::Positive
            },
            Verdict {
                predicate: "door_g",
                sign: Sign::Zero
            },
            Verdict {
                predicate: "door_h",
                sign: Sign::Negative
            },
        ]
    );
    assert_eq!(
        recorded.escalations,
        vec![
            Escalation { source: d },
            Escalation { source: e },
            Escalation { source: f },
        ]
    );
    assert_eq!(
        recorded
            .escalations
            .iter()
            .map(Escalation::predicate)
            .collect::<Vec<_>>(),
        ["door_d", "door_e", "door_f"]
    );
}

/// The other channel every door writes: the predicate name the recording
/// scalar reads. A door that stopped setting the name would tag its
/// sample with whatever the previous decision left behind — a silently
/// misattributed margin rather than a missing one.
#[test]
#[cfg(feature = "probe")]
fn every_door_names_its_own_sample_for_the_recording_scalar() {
    use geom_core::k_stats::{Probe, start_recording, take_samples};

    let b = band();
    start_recording();
    decide("name_a", Margin::of(Probe(1.0)), b).unwrap();
    decide_invariant("name_b", Probe(-1.0), b).unwrap();
    decide_flagged("name_c", Probe(1.0), b, "test fixture: name channel").unwrap();
    let names: Vec<&str> = take_samples().iter().map(|s| s.predicate).collect();
    assert_eq!(names, vec!["name_a", "name_b", "name_c"]);
}

/// **A gate's rejection is the funnel's escalation, on the frame.**
///
/// The gate doors exist so that a predicate whose question needs a
/// definitely-positive arm (or a definitely-nonzero discriminant) never
/// holds `decide`'s answer long enough to reject it in private. The
/// observable consequence is this: one gated rejection puts the
/// classifier's definite verdict on the verdict channel AND the gate's
/// `Invalid` escalation on the escalation channel, in decision order,
/// under the same name.
#[test]
fn a_rejected_gate_records_both_channels_under_its_own_name() {
    let b = band();
    let bracket = Bracket::open();
    assert_eq!(decide_positive("gate_a", Margin::of(1.0f64), b), Ok(()));
    let rejected = decide_positive("gate_b", Margin::of(-1.0f64), b).unwrap_err();
    assert_eq!(
        decide_nonzero("gate_c", Margin::of(-1.0f64), b),
        Ok(NonzeroSign::Negative)
    );
    let zeroed = decide_nonzero("gate_d", Margin::of(0.0f64), b).unwrap_err();
    let recorded = bracket.finish();

    assert_eq!(rejected.margin, MarginDiag::Invalid);
    assert_eq!(rejected.predicate, Some("gate_b"));
    assert_eq!(zeroed.margin, MarginDiag::Invalid);
    assert_eq!(zeroed.predicate, Some("gate_d"));
    assert_eq!(
        recorded.verdicts,
        [
            Verdict {
                predicate: "gate_a",
                sign: Sign::Positive,
            },
            Verdict {
                predicate: "gate_b",
                sign: Sign::Negative,
            },
            Verdict {
                predicate: "gate_c",
                sign: Sign::Negative,
            },
            Verdict {
                predicate: "gate_d",
                sign: Sign::Zero,
            },
        ],
        "gating leaves the verdict channel exactly as `decide` left it"
    );
    assert_eq!(
        recorded.escalations,
        [
            Escalation { source: rejected },
            Escalation { source: zeroed },
        ]
    );
}

/// An IN-BAND margin escalates through `classify` itself, and the gate
/// adds nothing: one escalation, carrying the real margin rather than
/// `Invalid`. A gate that re-recorded what it received would double
/// every in-band decision on the log.
#[test]
fn a_gate_over_an_in_band_margin_records_one_escalation_with_its_margin() {
    let b = band();
    let mid = f64::midpoint(b.zero(), b.escalate());
    let bracket = Bracket::open();
    let escalated = decide_positive("gate_in_band", Margin::of(mid), b).unwrap_err();
    let recorded = bracket.finish();
    assert_eq!(escalated.margin, MarginDiag::Value(mid));
    assert!(recorded.verdicts.is_empty());
    assert_eq!(
        recorded.escalations,
        [Escalation { source: escalated }],
        "the funnel recorded it once; the gate never saw a definite sign to reject"
    );
}

/// The measurement gate decides nothing, so it writes no verdict — and
/// on a poisoned value it writes the same escalation the classifier
/// would have written for a poisoned margin.
#[test]
fn the_measurement_gate_records_only_its_escalation() {
    let b = band();
    let bracket = Bracket::open();
    assert_eq!(gate_measured("measured", 0.5f64, b), Ok(0.5f64));
    let poisoned = gate_measured("measured", f64::NAN, b).unwrap_err();
    let recorded = bracket.finish();
    assert_eq!(poisoned.margin, MarginDiag::Invalid);
    assert_eq!(poisoned.predicate, Some("measured"));
    assert!(
        recorded.verdicts.is_empty(),
        "no comparand, no sign, no verdict: {:?}",
        recorded.verdicts
    );
    assert_eq!(recorded.escalations, [Escalation { source: poisoned }]);
}

/// Outside every bracket a gate records nowhere and still refuses — the
/// same posture `decide` has, so an op is not obliged to be bracketed to
/// be correct.
#[test]
fn a_gate_outside_every_bracket_still_refuses() {
    let b = band();
    assert_eq!(
        decide_positive("unbracketed", Margin::of(-1.0f64), b)
            .unwrap_err()
            .predicate,
        Some("unbracketed")
    );
    assert_eq!(
        gate_measured("unbracketed", f64::NAN, b)
            .unwrap_err()
            .predicate,
        Some("unbracketed")
    );
}

/// **SEAM 3 — `Discharge` against `SampleOutcome`.** Every discharge
/// kind retags its K sample with a token of its own, and the tokens a
/// discharge writes are exactly the three this row classifies as
/// discharge kinds; the rest — the definite signs, `Indeterminate`,
/// `Invalid` — are classified margins the numeric channel wrote, and
/// no discharge reaches them.
///
/// The third of the three rows the discharge vocabulary is held
/// together by; the other two are `sym::discharge_pins`, whose module
/// docs carry the argument for all three. It is HERE, and not there,
/// because `SampleOutcome` exists only under `probe` and a
/// `probe`-gated row inside the library is compiled by CI and run by
/// nothing — this suite is rostered as one the probe sweep executes
/// (the module header above).
///
/// `geom_core::sym::discharge_sample_outcomes` is the production
/// projection `Sym::sign_within` retags through, not a copy of it.
#[test]
#[cfg(feature = "probe")]
fn every_discharge_kind_retags_its_sample_with_a_token_of_its_own() {
    use geom_core::k_stats::SampleOutcome;
    use geom_core::sym::discharge_sample_outcomes;

    // Every K token, once, classified — a DISCHARGE kind's token or
    // one of the margins the numeric channel classified. The `match`
    // is exhaustive, so a new `SampleOutcome` is a compile error here
    // until this row says which it is.
    let tokens: Vec<(SampleOutcome, bool)> = SampleOutcome::ALL
        .into_iter()
        .map(|outcome| {
            let from_a_discharge = match outcome {
                SampleOutcome::SymbolicZero
                | SampleOutcome::SignGated
                | SampleOutcome::Registered => true,
                SampleOutcome::Definite(_)
                | SampleOutcome::Indeterminate
                | SampleOutcome::Invalid => false,
            };
            (outcome, from_a_discharge)
        })
        .collect();

    let retagged = discharge_sample_outcomes();
    for (i, (kind, outcome)) in retagged.iter().enumerate() {
        assert!(
            !retagged[..i].iter().any(|(_, o)| o == outcome),
            "{kind} retags its sample with `{}`, which another discharge kind already writes \
             — so the K corpus cannot tell the two populations apart and one of them is \
             scored as the other: {retagged:?}",
            outcome.token()
        );
    }
    for (outcome, from_a_discharge) in tokens {
        assert_eq!(
            retagged.iter().any(|(_, o)| *o == outcome),
            from_a_discharge,
            "`{}` is {} and is {} by a discharge kind — a discharge-kind token no kind \
             writes is a population the sweep never records, and a numeric token a \
             discharge writes is a margin scored against a band it was never classified \
             against: {retagged:?}",
            outcome.token(),
            if from_a_discharge {
                "a discharge kind's token"
            } else {
                "not a discharge kind's token"
            },
            if retagged.iter().any(|(_, o)| *o == outcome) {
                "written"
            } else {
                "not written"
            }
        );
    }
}
