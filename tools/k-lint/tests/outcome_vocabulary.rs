//! **The lint's accepted outcome tokens ARE the recorder's** — pinned
//! across the tool boundary, because that boundary has already drifted
//! once and the drift was invisible.
//!
//! # What happened, and why a test lives here
//!
//! `geom_core::k_stats::SampleOutcome` grew a `SymbolicZero` variant
//! (ERROR-DESIGN E12, the symbolic identity tier), serialized into the
//! K sweep's CSV as `symbolic_zero`. `k_lint::ACCEPTED_OUTCOMES` did
//! not learn it. The lint's policy for an unknown token is to refuse
//! the file as HARNESS BREAKAGE — deliberately, so that a sweep-format
//! drift cannot score a population silently CLEAN — so from that commit
//! on every driver CSV was refused at its first sample row and the E6
//! driver population was linted zero times, at every ε.
//!
//! Nobody saw it, because the CI row that reports the lint's status
//! could not fail (a `PIPESTATUS` capture taken after the variable it
//! reads had already been clobbered). Two independent defects, and the
//! shape they compose into is the one worth pinning against: a
//! vocabulary shared by a WRITER and a READER that live in different
//! cargo workspaces, kept in agreement by hand.
//!
//! # What this test can and cannot say
//!
//! It enumerates `SampleOutcome::ALL` — whose own completeness is
//! pinned inside `geom-core` by matching on every variant — and asserts
//! that each variant's `token()` is a token this lint accepts, and that
//! the lint accepts nothing else. So a new variant reds HERE, in the
//! tool that would otherwise start refusing every file it is handed.
//!
//! It does NOT check that the sweep's CSV writers call `token()`; that
//! is a separate claim, and the writers are ordinary test harnesses in
//! the kernel workspace which this crate cannot see. What it does is
//! make the shared vocabulary have one definition and one pin.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::SampleOutcome;
use k_lint::ACCEPTED_OUTCOMES;

#[test]
fn every_recorded_outcome_is_a_token_this_lint_accepts() {
    for outcome in SampleOutcome::ALL {
        let token = outcome.token();
        assert!(
            ACCEPTED_OUTCOMES.contains(&token),
            "the recorder writes {outcome:?} as `{token}` and this lint refuses that token as \
             harness breakage — so every sweep file containing one is refused unread, and the \
             population it carries is never linted. Add `{token}` to `ACCEPTED_OUTCOMES` and \
             give it an arm in `lint_sample` that says what rules it answers to.\n\
             accepted: {ACCEPTED_OUTCOMES:?}"
        );
    }
}

#[test]
fn this_lint_accepts_nothing_the_recorder_cannot_write() {
    let written: Vec<&str> = SampleOutcome::ALL.iter().map(|o| o.token()).collect();
    for token in ACCEPTED_OUTCOMES {
        assert!(
            written.contains(&token),
            "`{token}` is accepted here but no `SampleOutcome` serializes to it — either the \
             recorder dropped a variant and this list is stale, or the token was never real. \
             An accepted token nothing writes is a hole in the harness-breakage rule.\n\
             written: {written:?}"
        );
    }
}

/// The counts have to agree too, which is the cheap way to catch a
/// duplicate entry on either side.
#[test]
fn the_two_vocabularies_are_the_same_size() {
    assert_eq!(
        ACCEPTED_OUTCOMES.len(),
        SampleOutcome::ALL.len(),
        "the lint accepts {} tokens and the recorder writes {} outcomes",
        ACCEPTED_OUTCOMES.len(),
        SampleOutcome::ALL.len()
    );
}

/// **A `symbolic_zero` row parses, counts in its own column, and
/// answers to no rule.** The regression this file exists for, stated as
/// behaviour rather than as a list comparison.
#[test]
fn a_symbolic_zero_row_is_linted_rather_than_refused() {
    let csv = "shape,predicate,margin,band_zero,band_escalate,outcome\n\
               driver/slab_narrow,witness_at_mid_parameter,0e0,1e-100,1e-50,symbolic_zero\n\
               driver/slab_narrow,carrier_endpoint_start,1e-3,1e-9,1e-8,positive\n";
    let scan = k_lint::lint_csv(csv).expect(
        "a `symbolic_zero` row is a row this lint understands; refusing it as harness \
         breakage is what disarmed the E6 driver gate",
    );
    assert_eq!(scan.scanned, 2, "both rows counted");
    assert_eq!(scan.symbolic, 1, "the symbolic row counted in its own column");
    assert!(
        scan.flags.is_empty(),
        "a symbolic zero was never classified against the band, so it can be no rule's \
         finding: {:?}",
        scan.flags
    );
}
