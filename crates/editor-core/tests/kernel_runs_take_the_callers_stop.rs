//! **The evaluations a door runs take its caller's stop**
//! ([`editor_core::Stop`]): a caller that cancels its token, or lowers
//! its member budget, stops the evaluation the door runs inside it (a
//! drive's witness, a Monte-Carlo run's nominal build, a split's read of
//! the document), and the door refuses with how its run stopped instead
//! of reading a prefix.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::block;
use crate::fixture;
use crate::m10_3_driver_interval::slab;
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, DriveRefusal, drive};
use editor_core::mc::{McConfig, McRefusal, monte_carlo};
use editor_core::{
    CancelToken, Coord, DocumentId, Formula, ProfileDoc, SplitError, Stop, Stopped, TargetAxis,
};
use geom_core::Tol;
use std::collections::BTreeSet;

/// The caller's token, canceled before the door is asked.
fn canceled() -> Stop {
    let cancel = CancelToken::new();
    cancel.cancel();
    Stop::canceled_by(&cancel)
}

/// **Canceling the caller stops a drive**: the witness build runs on
/// the caller's token, so the drive refuses as stopped. Red if the
/// drive mints a token of its own: the witness builds and the drive
/// runs on.
#[test]
fn a_canceled_caller_stops_a_drive() {
    let doc = slab(1.0, 0.1);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let refused = drive(
        &doc,
        &analyzed,
        &DriveConfig::default(),
        &canceled(),
        Tol::witness(),
    );
    assert_eq!(
        refused.err(),
        Some(DriveRefusal::Stopped(Stopped::Canceled)),
        "the drive refuses as canceled"
    );
}

/// **Canceling the caller stops a Monte-Carlo run**, at its nominal
/// build, for the drive's reason.
#[test]
fn a_canceled_caller_stops_a_monte_carlo_run() {
    let doc = slab(1.0, 0.1);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let refused = monte_carlo(
        &doc,
        &analyzed,
        &McConfig::default(),
        &canceled(),
        Tol::witness(),
    );
    assert_eq!(
        refused.err(),
        Some(McRefusal::Stopped(Stopped::Canceled)),
        "the run refuses as canceled"
    );
}

/// **A caller's lowered budget exhausts the drive's witness**, and the
/// drive says so typed: the slab with a three-member family beside it,
/// driven under a budget of 2. Red if the drive builds its witness at
/// the default budget: the family fits, and the drive runs on.
#[test]
fn a_lowered_budget_exhausts_a_drives_witness() {
    let doc = slab(1.0, 0.1);
    let (doc, i) = fixture::index(doc, "i", 3);
    let (doc, cell) = block(doc, (5.0, 5.5), (0.0, 0.5), 0.5);
    let slide = Formula::mul(fixture::scalar_of(i), fixture::len(1.0)).unwrap();
    let (doc, _family) = fixture::family_of(doc, cell, vec![(Coord::slide(TargetAxis::X), slide)]);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let driven = |stop: &Stop| {
        drive(
            &doc,
            &analyzed,
            &DriveConfig::default(),
            stop,
            Tol::witness(),
        )
    };
    let nominal: editor_core::Evaluation<f64> = editor_core::evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &editor_core::EvalOptions::default(),
        Tol::witness(),
    );
    assert_eq!(
        nominal.outcome,
        editor_core::EvalOutcome::Completed,
        "the family fits the default budget: only the lowered one stops it"
    );
    let refused = driven(&Stop {
        member_budget: 2,
        ..Stop::default()
    });
    assert_eq!(
        refused.err(),
        Some(DriveRefusal::Stopped(Stopped::Exhausted { budget: 2 })),
        "the drive refuses as out of the caller's budget"
    );
}

/// Two disjoint placed blocks, and the cut that splits the second out:
/// its profile, its extrude and its placement.
fn two_blocks() -> (ProfileDoc, BTreeSet<editor_core::RecipeNodeId>) {
    let doc = ProfileDoc::empty(DocumentId::derive("stop-split"), Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let doc = fixture::place(doc, a).0;
    let before: BTreeSet<_> = doc.ids().iter().copied().collect();
    let (doc, b) = block(doc, (10.0, 10.5), (0.0, 0.5), 1.0);
    let second: BTreeSet<_> = doc
        .ids()
        .iter()
        .copied()
        .filter(|id| !before.contains(id))
        .collect();
    let doc = fixture::place(doc, b).0;
    let cut = fixture::with_placements(&doc, &second);
    (doc, cut)
}

/// **Canceling the caller stops the run inside a split**: the split
/// reads the document's world through an evaluation on the caller's
/// token, so it refuses as stopped and writes nothing. Red if the split
/// mints a token of its own: the read completes and the split succeeds.
#[test]
fn a_canceled_caller_stops_the_run_inside_a_split() {
    let (doc, cut) = two_blocks();
    let split = |stop: &Stop| {
        editor_core::split(
            &doc,
            &cut,
            DocumentId::derive("stop-split-part"),
            Tol::witness(),
            None,
            stop,
        )
    };
    assert!(
        split(&Stop::default()).is_ok(),
        "the cut is legal: only the stop can refuse it"
    );
    let refused = split(&canceled());
    assert!(
        matches!(refused, Err(SplitError::Stopped(Stopped::Canceled))),
        "the split refuses as canceled: {:?}",
        refused.err()
    );
}
