//! The three public classification doors write ONE recording stream.
//!
//! `decide`, `decide_flagged` and `decide_invariant` share a private
//! `classify`, so the predicate-name channel and the verdict channel are
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
//! **ITS PROBE-GATED CODE IS NOT EXECUTED BY CI**, and its sibling IS
//! executed: `every_door_names_its_own_sample_for_the_recording_scalar`
//! carries the feature gate and no CI row passes it, while the ungated test
//! here runs on every merge. The probe suites CI runs are rostered in
//! `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`) and run by
//! `scripts/k_probe_sweep.sh`; this file is on neither list, so the
//! name-channel claim is evidence for a reader rather than a gate. By hand:
//! `cargo test -p geom-core --features probe --test all -- k_stats_doors::`.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::Tol;
use geom_core::k_stats::{
    Verdict, decide, decide_flagged, decide_invariant, start_verdict_log, take_verdict_log,
};
use geom_core::{Band, Margin, Sign};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The fixture interleaves the doors deliberately — a run that grouped
/// them would pass under any per-door ordering.
#[test]
fn the_three_doors_share_one_verdict_stream_in_decision_order() {
    let b = band();
    let mid = f64::midpoint(b.zero(), b.escalate());
    start_verdict_log();
    assert_eq!(decide("door_a", Margin::of(1.0f64), b), Ok(Sign::Positive));
    assert_eq!(decide_invariant("door_b", -1.0f64, b), Ok(Sign::Negative));
    assert_eq!(
        decide_flagged("door_c", 1.0f64, b, "test fixture: door interleaving"),
        Ok(Sign::Positive)
    );
    // One indeterminate per door: escalated outcomes are not verdicts,
    // so none of these may appear or shift the positions after them.
    assert!(decide("door_d", Margin::of(mid), b).is_err());
    assert!(decide_flagged("door_e", mid, b, "test fixture: door interleaving").is_err());
    assert!(decide_invariant("door_f", mid, b).is_err());
    assert_eq!(
        decide_flagged("door_g", 0.0f64, b, "test fixture: door interleaving"),
        Ok(Sign::Zero)
    );
    assert_eq!(decide("door_h", Margin::of(-1.0f64), b), Ok(Sign::Negative));
    assert_eq!(
        take_verdict_log(),
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
