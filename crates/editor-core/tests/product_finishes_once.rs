//! **The gather finishes the aggregate once, and runs the census once.**
//!
//! Every piece the gather reads is already finished: a body slot holds
//! the scalar's finished type (`Framed<T>` stores `T::Finished`), so no
//! piece is re-gated on its own and a refusal of the aggregate names the
//! product, not a placement (`ProductError::ProductInvalid`). These rows
//! hold the shape no behavioural row can see: an extra pass over the
//! same geometry changes no verdict.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source::{blanked, code_only, required_matches};

const PRODUCT: &str = include_str!("../src/product.rs");
const AT_REST: &str = include_str!("../src/checks/at_rest.rs");
const CHECKS: &str = include_str!("../src/checks.rs");

/// **A gather finishes its aggregate at one site and validates nothing
/// else.** Red if a per-piece pass comes back (a second `T::finish`, or
/// a validator call of the gather's own).
#[test]
fn the_gather_finishes_the_aggregate_once() {
    let code = blanked(code_only, "editor-core/src/product.rs", PRODUCT);
    let what = "editor-core/src/product.rs (code view)";
    let finishes = required_matches(&code, what, "T::finish(aggregate");
    assert_eq!(finishes.len(), 1, "{what}: one finish, of the aggregate");
    for gate in [
        "T::finish(",
        "topo::finish(",
        "validate_geometric(",
        "validate_pseudomanifold(",
    ] {
        assert!(
            code.matches(gate).count() <= usize::from(gate == "T::finish("),
            "{what}: `{gate}` is called outside the aggregate's finish"
        );
    }
}

/// **The at-rest census runs in the gather, and only there.** The
/// gather hands the tier-3′ census to the recording door once
/// (`topo::record_at_rest`, whose next pass confirms what it records)
/// and keeps its findings on the product (`Product::census`); the
/// at-rest check reads them and calls no census of its own. Red if the
/// check runs the census again, or the gather calls it from a second
/// site: an extra pass over one product changes no verdict, so no
/// behavioural row can see it.
#[test]
fn the_census_runs_in_the_gather_and_never_at_the_check() {
    let census = "T::census_at_rest(";
    let product = blanked(code_only, "editor-core/src/product.rs", PRODUCT);
    let calls = required_matches(&product, "editor-core/src/product.rs (code view)", census);
    assert_eq!(calls.len(), 1, "the gather calls the census from one site");
    for (path, source) in [
        ("editor-core/src/checks/at_rest.rs", AT_REST),
        ("editor-core/src/checks.rs", CHECKS),
    ] {
        let code = blanked(code_only, path, source);
        assert!(
            !code.contains("census_at_rest") && !code.contains("record_at_rest("),
            "{path}: the check reads the gather's census and runs none"
        );
    }
}
