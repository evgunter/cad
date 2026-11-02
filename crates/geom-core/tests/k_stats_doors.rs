//! The public classification doors write ONE recording stream.
//!
//! `decide`, `decide_reported` and `decide_flagged`
//! share a private `classify` (`decide_reported` hands back the verdict
//! whole, its reporting margin beside the sign), and the gate doors (`decide_positive`, `decide_negative`, `decide_nonzero`,
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
//! **CI EXECUTES BOTH HALVES.** The ungated tests run on every merge;
//! `every_door_names_its_own_sample_for_the_recording_scalar` carries
//! the `probe` gate, and this file is rostered in
//! `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`) and run under the
//! DEFAULT selection by `scripts/k_probe_sweep.sh`, whose tally is floored
//! by `--check-executed`. The name-channel claim is therefore a gate. By
//! hand:
//! `cargo test -p geom-core --features probe --test all -- k_stats_doors::`.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::Tol;
use geom_core::k_stats::{
    Bracket, Escalation, Magnitude, NonzeroSign, Verdict, decide, decide_flagged, decide_magnitude,
    decide_negative, decide_nonzero, decide_positive, decide_reported, gate_measured,
};
use geom_core::{
    Band, Decided, ErrorTextReading, Indeterminate, Interval, Margin, MarginDiag, Sign,
};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The fixture interleaves the doors deliberately — a run that grouped
/// them would pass under any per-door ordering.
#[test]
fn the_doors_share_one_verdict_stream_in_decision_order() {
    let b = band();
    let mid = f64::midpoint(b.zero(), b.escalate());
    let bracket = Bracket::open();
    assert_eq!(decide("door_a", Margin::of(1.0f64), b), Ok(Sign::Positive));
    assert_eq!(decide("door_b", Margin::of(-1.0f64), b), Ok(Sign::Negative));
    assert_eq!(
        decide_flagged("door_c", 1.0f64, b, "test fixture: door interleaving"),
        Ok(Sign::Positive)
    );
    let half = b.zero() / 2.0;
    assert_eq!(
        decide_reported("door_r", Margin::of(half), b),
        Ok(Decided {
            sign: Sign::Zero,
            margin: MarginDiag::value(half),
        })
    );
    // One indeterminate per door: escalated outcomes are not verdicts,
    // so none of these may appear or shift the positions after them —
    // they are the frame's OTHER channel, in their own decision order.
    let d = decide("door_d", Margin::of(mid), b).unwrap_err();
    let e = decide_flagged("door_e", mid, b, "test fixture: door interleaving").unwrap_err();
    let f = decide("door_f", Margin::of(mid), b).unwrap_err();
    let s = decide_reported("door_s", Margin::of(mid), b).unwrap_err();
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
                predicate: "door_r",
                sign: Sign::Zero
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
            Escalation { source: s },
        ]
    );
    assert_eq!(
        recorded
            .escalations
            .iter()
            .map(Escalation::predicate)
            .collect::<Vec<_>>(),
        ["door_d", "door_e", "door_f", "door_s"]
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
    decide("name_b", Margin::of(Probe(-1.0)), b).unwrap();
    decide_flagged("name_c", Probe(1.0), b, "test fixture: name channel").unwrap();
    decide_reported("name_d", Margin::of(Probe(1.0)), b).unwrap();
    let names: Vec<&str> = take_samples().iter().map(|s| s.predicate).collect();
    assert_eq!(names, vec!["name_a", "name_b", "name_c", "name_d"]);
}

/// **A gate's rejection is the funnel's escalation, on the frame.**
///
/// The gate doors exist so that a predicate whose question needs a
/// definitely-positive arm (or a definitely-nonzero discriminant) never
/// holds `decide`'s answer long enough to reject it in private. The
/// observable consequence is this: one gated rejection puts the
/// classifier's definite verdict on the verdict channel AND the gate's
/// escalation on the escalation channel, in decision order, under the
/// same name, carrying the margin the classifier decided.
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
    assert_eq!(decide_negative("gate_e", Margin::of(-1.0f64), b), Ok(()));
    let unsigned = decide_negative("gate_f", Margin::of(1.0f64), b).unwrap_err();
    let recorded = bracket.finish();
    assert_eq!(
        (
            unsigned.margin.diagnostic_f64_for_error_text(),
            unsigned.margin.rejected_sign()
        ),
        (ErrorTextReading::Value(1.0), Some(Sign::Positive)),
        "decided positive"
    );
    assert_eq!(unsigned.predicate, Some("gate_f"));

    assert_eq!(
        (
            rejected.margin.diagnostic_f64_for_error_text(),
            rejected.margin.rejected_sign()
        ),
        (ErrorTextReading::Value(-1.0), Some(Sign::Negative)),
        "decided negative"
    );
    assert_eq!(rejected.predicate, Some("gate_b"));
    assert_eq!(
        (
            zeroed.margin.diagnostic_f64_for_error_text(),
            zeroed.margin.rejected_sign()
        ),
        (ErrorTextReading::Value(0.0), Some(Sign::Zero)),
        "decided zero"
    );
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
            Verdict {
                predicate: "gate_e",
                sign: Sign::Negative,
            },
            Verdict {
                predicate: "gate_f",
                sign: Sign::Positive,
            },
        ],
        "gating leaves the verdict channel exactly as `decide` left it"
    );
    assert_eq!(
        recorded.escalations,
        [
            Escalation { source: rejected },
            Escalation { source: zeroed },
            Escalation { source: unsigned },
        ]
    );
}

/// **A rejection's text says what was decided.** A decided zero reads
/// as within the zero band and offers the tolerance its margin gives
/// (D4 ¶1 (i)) where that lies above `EPS_MIN` (D4 ¶4); a decided sign
/// on the rejected side reads as past the band, at both scalars, with no tolerance arm and no subdivision arm:
/// it is sign-certain, and a sub-box keeps its sign.
#[test]
fn a_rejection_tells_the_decided_margin_it_carries() {
    let b = band();
    let small = 0.5 * b.zero();
    let zeroed = decide_positive("gate_zero", Margin::of(small), b)
        .unwrap_err()
        .to_string();
    assert!(zeroed.contains("lies within the zero band"), "{zeroed}");
    assert_eq!(
        zeroed.contains("tighten the tolerance below"),
        b.offer_deciding(small).is_some(),
        "an offer exactly where its tolerance lies above EPS_MIN: {zeroed}"
    );

    let negative = decide_positive("gate_negative", Margin::of(-1.0f64), b)
        .unwrap_err()
        .to_string();
    let enclosed = decide_positive(
        "gate_enclosed",
        Margin::of(Interval::from_bounds(-2.0, -1.0)),
        b,
    )
    .unwrap_err()
    .to_string();
    for text in [&negative, &enclosed] {
        assert!(text.contains("lies past the ambiguity band"), "{text}");
        assert!(
            text.contains("a decided sign this decision cannot use"),
            "{text}"
        );
        assert!(!text.contains("tighten"), "sign-certain: {text}");
        assert!(!text.contains("subdivide"), "decided: {text}");
        assert!(!text.contains("invalid"), "nothing was poisoned: {text}");
    }
}

/// **A gate's rejected sign's word rides the payload, and is said
/// once.** Where a gate rejected a sign decided past the band, its
/// payload says what that reading is after the placement, so a door
/// that composes the payload with its own decision's ending tells it
/// ([`Indeterminate::undecided`]), and the escalation's own sentence
/// says it once. A rejected zero is told by its placement, and a
/// reading inside the band was not decided: both keep their word in the
/// tail. A reading past the band that no gate tagged (a bound reported
/// where it stands) was decided by nobody, and says no such word.
#[test]
fn a_decided_signs_word_rides_the_payload_once() {
    let b = band();
    let rejected = |margin: f64| decide_positive("gate_word", Margin::of(margin), b).unwrap_err();
    let decided = "a decided sign this decision cannot use";
    let near = "a near-coincidence";
    for cause in [
        rejected(-1.0),
        decide_positive(
            "gate_word",
            Margin::of(Interval::from_bounds(-2.0, -1.0)),
            b,
        )
        .unwrap_err(),
    ] {
        let payload = cause.payload().to_string();
        let whole = cause.to_string();
        assert_eq!(
            cause.margin.rejected_sign(),
            Some(Sign::Negative),
            "the gate's tag: {payload}"
        );
        assert!(
            payload.ends_with(&format!(" — {decided}")),
            "the payload: {payload}"
        );
        assert_eq!(whole.matches(decided).count(), 1, "said once: {whole}");
        assert!(
            whole.starts_with(&format!("{payload}; ")),
            "the tail: {whole}"
        );
        assert_eq!(
            cause.undecided("whether S", "E").to_string(),
            format!("whether S is undecided: {payload}. E"),
            "a composed ending"
        );
    }
    for (cause, tag) in [
        (rejected(0.5 * b.zero()), Some(Sign::Zero)),
        (rejected(f64::midpoint(b.zero(), b.escalate())), None),
    ] {
        let payload = cause.payload().to_string();
        assert_eq!(
            cause.margin.rejected_sign(),
            tag,
            "the gate's tag: {payload}"
        );
        assert!(
            !payload.contains(" — "),
            "no word in the payload: {payload}"
        );
        assert!(
            cause.to_string().contains(&format!(" — {near}; ")),
            "the tail's word: {cause}"
        );
    }
    for margin in [MarginDiag::value(-1.0), MarginDiag::enclosure(1.0, 2.0)] {
        let reported = Indeterminate {
            margin,
            band: b,
            predicate: Some("reported_bound"),
            terminal_sliver: false,
        };
        let payload = reported.payload().to_string();
        assert!(
            payload.ends_with(&format!(
                "lies past the ambiguity band ({:e}, {:e})",
                b.zero(),
                b.escalate()
            )),
            "an untagged reading past the band: {payload}"
        );
        assert!(!payload.contains(decided), "no word: {payload}");
        let whole = reported.to_string();
        assert!(!whole.contains(decided), "no word in the tail: {whole}");
    }
}

/// **The magnitude door** answers `Zero` or `Positive` and records the
/// verdict as `decide` does; a margin it cannot call escalates through
/// the funnel, once.
#[test]
fn the_magnitude_door_reads_zero_and_positive() {
    let b = band();
    let mid = f64::midpoint(b.zero(), b.escalate());
    let bracket = Bracket::open();
    assert_eq!(
        decide_magnitude("mag_a", Margin::of(0.0f64), b),
        Ok(Magnitude::Zero)
    );
    assert_eq!(
        decide_magnitude("mag_b", Margin::of(1.0f64), b),
        Ok(Magnitude::Positive)
    );
    let escalated = decide_magnitude("mag_c", Margin::of(mid), b).unwrap_err();
    let recorded = bracket.finish();
    assert_eq!(escalated.margin, MarginDiag::value(mid));
    assert_eq!(
        recorded.verdicts,
        [
            Verdict {
                predicate: "mag_a",
                sign: Sign::Zero,
            },
            Verdict {
                predicate: "mag_b",
                sign: Sign::Positive,
            },
        ]
    );
    assert_eq!(recorded.escalations, [Escalation { source: escalated }]);
}

/// A decided `Negative` breaks the door's precondition, and the panic
/// names the predicate and the margin a reader debugging it wants.
#[test]
#[should_panic(expected = "`mag_negative` decided a magnitude Negative (margin -1e0")]
fn the_magnitude_door_panics_on_a_decided_negative() {
    let _ = decide_magnitude("mag_negative", Margin::of(-1.0f64), band());
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
    assert_eq!(escalated.margin, MarginDiag::value(mid));
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
    assert_eq!(poisoned.margin, MarginDiag::INVALID);
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

/// **A decided zero enclosure offers no subdivision.** A gate's
/// rejection of an enclosure decided zero is a decided reading: a
/// sub-box keeps its sign, so the refusal reads within the zero band
/// and names no subdivision.
#[test]
fn a_decided_zero_enclosure_rejection_offers_no_subdivision() {
    let b = band();
    let half = 0.5 * b.zero();
    let text = decide_positive(
        "gate_zero_enclosure",
        Margin::of(Interval::from_bounds(-half, half)),
        b,
    )
    .unwrap_err()
    .to_string();
    assert!(text.contains("lies within the zero band"), "{text}");
    assert!(!text.contains("subdivide"), "decided: {text}");
    assert!(!text.contains("cannot be classified"), "decided: {text}");
}

/// **The text places a margin where the classifier does, at the band's
/// edge**: a margin of exactly `escalate` on the rejected side is
/// decided, so it reads past the band and offers no tolerance.
#[test]
fn a_rejection_at_the_escalation_edge_reads_past_the_band() {
    let b = band();
    for text in [
        decide_positive("gate_edge", Margin::of(-b.escalate()), b)
            .unwrap_err()
            .to_string(),
        decide_negative("gate_edge", Margin::of(b.escalate()), b)
            .unwrap_err()
            .to_string(),
    ] {
        assert!(text.contains("lies past the ambiguity band"), "{text}");
        assert!(!text.contains("tighten"), "sign-certain: {text}");
    }
}

/// **A zero on the side a gate rejects offers no tolerance.** A smaller
/// tolerance decides such a margin onto the rejected sign, so no
/// tolerance is offered (D4 ¶1 (i)) and the text says the zero stays
/// rejected — at both scalars, for both one-sided gates. The same zero
/// on the passing side keeps its offer where one lies above `EPS_MIN`,
/// and ends saying no tolerance a run admits decides it where none does
/// (the whole sentence, said once, is `predicate`'s floor row).
#[test]
fn a_zero_on_the_rejected_side_offers_no_tolerance() {
    let b = band();
    let small = 0.5 * b.zero();
    let rejected = [
        decide_positive("gate_neg_zero", Margin::of(-small), b),
        decide_positive(
            "gate_neg_zero_enclosure",
            Margin::of(Interval::from_bounds(-small, -0.5 * small)),
            b,
        ),
        decide_negative("gate_pos_zero", Margin::of(small), b),
    ];
    for refusal in rejected {
        let text = refusal.unwrap_err().to_string();
        assert!(!text.contains("tighten"), "{text}");
        assert!(
            text.contains("a reading with no size on the side this decision accepts"),
            "{text}"
        );
    }
    let passing = decide_positive("gate_pos_zero", Margin::of(small), b).unwrap_err();
    assert_eq!(passing.margin.rejected_sign(), Some(Sign::Zero));
    let text = passing.to_string();
    assert_eq!(
        text.contains("tighten the tolerance below"),
        b.offer_deciding(small).is_some(),
        "{text}"
    );
    if b.offer_deciding(small).is_none() {
        assert!(
            text.ends_with(&geom_core::floor_note("size")),
            "a zero only a tolerance below EPS_MIN moves says so: {text}"
        );
    }
}

/// **A sign-certain rejection names the geometry.** A margin decided
/// past the band is no coincidence, so its refusal's recourse is the
/// geometry (D4 ¶1 (i)).
#[test]
fn a_past_band_rejection_offers_no_declaration() {
    let rejected = decide_positive("gate_certain", Margin::of(-1.0f64), band()).unwrap_err();
    assert_eq!(rejected.margin.rejected_sign(), Some(Sign::Negative));
    let text = rejected.to_string();
    assert!(text.contains("move the geometry"), "{text}");
}
