//! **The gather finishes the aggregate once, and runs the census once.**
//!
//! The gather reads its sources and takes none: a source in the
//! product's frame is read as its value holds it, and one standing under
//! another frame is mapped into the product's as construction state
//! (`Framed::read_in`, `ReadIn::Mapped`), which nothing finishes on its
//! own. The aggregate is what finishes, once, and a refusal of it names
//! the product, not a placement (`ProductError::ProductInvalid`).
//!
//! The finish is counted where it runs (`topo::finishes_on_this_thread`),
//! so a finish reached through the read (a mapped source ended as a leaf)
//! is seen as surely as one written in the gather.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    CancelToken, DocumentId, EvalOptions, Evaluation, Node, ProfileDoc, RecipeNodeId, evaluate,
};
use fixture::{len, on_frame, square};
use geom_core::Tol;
use test_utils::source::{blanked, code_only, required_matches};

const PRODUCT: &str = include_str!("../src/product.rs");
const AT_REST: &str = include_str!("../src/checks/at_rest.rs");
const CHECKS: &str = include_str!("../src/checks.rs");

/// A unit block, a construction in a space of its own.
fn block(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.5, 0.5, 0.5)],
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// **Three copies of a block in the world, each under its own frame,
/// gathered: one finish.** The gather computes in the first copy's
/// frame and maps the other two into it; neither mapped source finishes,
/// and the aggregate finishes once. Red if a source is finished where it
/// is read (two finishes more).
#[cfg(debug_assertions)]
#[test]
fn a_gather_over_mapped_sources_finishes_once() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive("product-finishes-once"), tol);
    let (doc, source) = block(doc);
    let mut doc = doc;
    for at in [[0.0, 0.0, 0.0], [3.0, 0.0, 0.0], [0.0, 3.0, 0.0]] {
        let (next, _) = editor_core::test_support::in_world(
            doc,
            source,
            &editor_core::Frame::translation(at),
            tol,
        );
        doc = next;
    }
    let doc = fixture::assemble_world_copies(doc);
    let ev: Evaluation<f64> = evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    let maps = editor_core::EvalCounters::on_this_thread();
    let before = topo::finishes_on_this_thread();
    let product = editor_core::product_recorded(&doc, &ev, tol).expect("the product gathers");
    let finishes = topo::finishes_on_this_thread() - before;
    let mapped = editor_core::EvalCounters::on_this_thread()
        .since(maps)
        .rigid_maps;
    assert_eq!(product.copies.len(), 3, "the three copies gather");
    assert_eq!(mapped, 2, "the second and third copies are mapped in");
    assert_eq!(finishes, 1, "one finish, of the aggregate");
}

/// **A gather finishes its aggregate at one site and validates nothing
/// else** in its own text. Red if a per-piece pass comes back (a second
/// `T::finish`, or a validator call of the gather's own).
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
