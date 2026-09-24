//! **Every at-rest finding the viewer shows fits where it is shown.**
//!
//! A kernel `topo::ValidationError` reaches the viewer as one line of a
//! gate's refusal: the at-rest badge draws `AssemblyError`'s `Display`
//! behind "at rest: " (`viewer::frame::at_rest_badge`), one finding per
//! line, and the product badge draws `ProductError::SolidInvalid` /
//! `ProductInvalid` the same way under a shorter header. These rows
//! render every arm — `topo::test_support::validation_error_samples`,
//! which `topo`'s own Display-coverage row holds to one sample per
//! variant at least — through the LONGER of the two, as a one-finding
//! refusal naming no declaration, and hold each to the budget
//! [`refusal_concision`](crate::refusal_concision) states.
//!
//! **What a row here cannot see.** A finding attributed to a mate of
//! ANOTHER document renders the route it was carried by, and the
//! `Uncertified` refusal opens with a longer header; neither is
//! rendered here. A nested payload (`{cause}`, `{source}`, `{error}`)
//! renders on the one representative value its sample carries — its
//! own enum's lengths are held by its owner's rows — and a `what` a
//! raise site writes inline, or forwards from another refusal, is not
//! among the samples (`topo`'s `UNDECIDABLE_WHATS` lists the ones
//! that are).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{AssemblyError, AtRestFinding, Attribution};

/// A kernel finding as the viewer's at-rest badge draws it: a
/// one-finding refusal, the finding naming no declaration.
fn as_the_at_rest_badge_shows_it(error: topo::ValidationError) -> String {
    let refusal = AssemblyError::AtRest {
        findings: vec![AtRestFinding {
            attribution: Attribution::Unattributed,
            error,
        }],
    };
    format!("at rest: {refusal}")
}

#[test]
fn every_at_rest_finding_renders_within_the_budget() {
    let mut problems = Vec::new();
    for error in topo::test_support::validation_error_samples() {
        let name = format!("{error:?}");
        let text = as_the_at_rest_badge_shows_it(error);
        let words = text.split_whitespace().count();
        eprintln!("MEASURE {words}: {text}");
        if words > test_utils::refusal::BUDGET {
            problems.push(format!(
                "{name} renders {words} words, over {}: {text}",
                test_utils::refusal::BUDGET
            ));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}
