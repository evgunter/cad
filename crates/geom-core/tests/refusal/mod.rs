//! The certification refusal as the decision door reports it.
//!
//! `m5_pr1_refusal_conservation`, `review_m5_pr1_launder` and
//! `review_m0_pr4` all ask whether an [`Interval`] refuses to classify
//! *because it is not certified* — the `Invalid` margin, not merely a
//! straddling enclosure — and each grew its own copy of the question.
//! One copy, here.
// A module of the aggregated `all` binary rather than a child of each
// including suite, so it no longer inherits their crate-root allows.
#![allow(clippy::expect_used)]
#![allow(dead_code)] // loaded once per consumer; each uses a subset
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

use geom_core::{Band, Decide, Indeterminate, Interval, MarginDiag, Sign};

/// A fixed pure band, built via `Band::new` so no consumer touches the
/// global tolerance. `sign_within` answers the `Invalid` margin before
/// it reads the band, so the choice does not move the verdict.
fn band() -> Band {
    Band::new(1e-9, 1e-8).expect("valid band")
}

fn verdict(x: Interval) -> Result<Sign, Indeterminate> {
    x.sign_within(band()).map(|d| d.sign)
}

/// Whether `x` refuses to classify because it is not certified.
pub fn refuses_as_invalid(x: Interval) -> bool {
    matches!(
        verdict(x),
        Err(Indeterminate {
            margin: MarginDiag::INVALID,
            ..
        })
    )
}

/// Asserts [`refuses_as_invalid`], naming the site and the verdict it
/// got instead.
#[track_caller]
pub fn assert_refused(name: &str, x: Interval) {
    assert!(
        refuses_as_invalid(x),
        "{name}: expected a refusal, got {:?}",
        verdict(x)
    );
}
