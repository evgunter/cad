//! **The evaluations a door runs take its caller's stop**
//! ([`editor_core::Stop`]): a caller that cancels its token stops the
//! drive's witness and the Monte-Carlo lane's nominal build, and the
//! door refuses with how its run stopped instead of reading a prefix.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::m10_3_driver_interval::slab;
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, DriveRefusal, drive};
use editor_core::mc::{McConfig, McRefusal, monte_carlo};
use editor_core::{CancelToken, Stop, Stopped};
use geom_core::Tol;

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
