//! **The gather finishes the aggregate once, and runs the census once.**
//!
//! The gather reads its sources and takes none: a source in the
//! product's frame is read as its value holds it, and one standing under
//! another frame is what its value holds, moved into the product's
//! (`Framed::read_in`, `Held::moved`), which finishes nothing. The
//! aggregate is what finishes, once, and a refusal of it names the
//! product, not a placement (`ProductError::ProductInvalid`).
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
/// gathered: three finishes.** The gather computes in the first copy's
/// frame and moves the other two into it. The block is a sweep's leaf,
/// and a moved leaf is finished on its image while the box-scalar switch
/// lives (`topo::LeafBody::moved`), so each mapped source runs one; the
/// aggregate finishes once. Red if a source is finished again where it
/// is taken.
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
    assert_eq!(
        finishes, 3,
        "one finish of the aggregate, and one of each mapped source: a moved \
         leaf is finished on its image while the box-scalar switch lives"
    );
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

/// The finishes `doc`'s evaluation runs, under `opts`.
#[cfg(debug_assertions)]
fn finishes_evaluating(doc: &ProfileDoc, opts: &EvalOptions) -> u64 {
    let before = topo::finishes_on_this_thread();
    let ev: Evaluation<f64> = evaluate(doc, None, &CancelToken::new(), opts, Tol::witness());
    assert!(
        ev.nodes
            .values()
            .all(|r| matches!(r, editor_core::NodeResult::Ok(_))),
        "every node evaluates"
    );
    topo::finishes_on_this_thread() - before
}

/// Two overlapping blocks' union, a finished value, and two copies of
/// it, each under a frame of its own.
#[cfg(debug_assertions)]
fn two_copies() -> (ProfileDoc, [RecipeNodeId; 2]) {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive("moved-finishes-none"), tol);
    let (doc, whole) = finished_value(doc);
    let (doc, a) = editor_core::test_support::in_world(
        doc,
        whole,
        &editor_core::Frame::translation([0.0, 0.0, 0.0]),
        tol,
    );
    let (doc, b) = editor_core::test_support::in_world(
        doc,
        whole,
        &editor_core::Frame::translation([0.5, 0.25, 0.5]),
        tol,
    );
    (doc, [a, b])
}

/// **A union of two finished operands in different frames runs one
/// finish**, its result's: the operand read into the computing frame is
/// moved, still finished, and is not finished again where the Boolean
/// takes it.
#[cfg(debug_assertions)]
#[test]
fn a_union_across_two_frames_runs_one_finish() {
    let (doc, [a, b]) = two_copies();
    let without = finishes_evaluating(&doc, &EvalOptions::default());
    let (doc, _) = fixture::union_over(doc, &[a, b]);
    let with = finishes_evaluating(&doc, &EvalOptions::default());
    assert_eq!(with - without, 1, "the union runs one finish, its result's");
}

/// Two overlapping unit blocks and their union: a Boolean's result, a
/// finished value.
#[cfg(debug_assertions)]
fn finished_value(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    let (doc, a) = block(doc);
    let (doc, profile) = on_frame(
        doc,
        [0.5, 0.25, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.5, 0.5, 0.5)],
    );
    let (doc, b) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    fixture::union_over(doc, &[a, b])
}

/// The computing frame's twist, about an oblique axis.
#[cfg(debug_assertions)]
fn twisted() -> EvalOptions {
    EvalOptions {
        computing_twist: Some(editor_core::ComputingTwist {
            axis: [0.0, 0.6, 0.8],
            angle: 0.7,
            by: [0.25, -0.5, 1.0],
        }),
        ..EvalOptions::default()
    }
}

/// `doc` with `node` mirrored across a plane read off it.
#[cfg(debug_assertions)]
fn mirrored(doc: &ProfileDoc, node: RecipeNodeId) -> ProfileDoc {
    let plane = fixture::plane_on(doc, node, [0.0; 3], [-1.0, 0.0, 0.0]);
    fixture::insert(
        doc.clone(),
        Node::Mirror {
            body: editor_core::Operand::output(node, 0).into(),
            plane,
        },
    )
    .0
}

/// **A mirror and the computing frame's twist of a finished value run no
/// finish**: each moves what its source holds, its verdict carried. Two
/// blocks' union is the finished value; the blocks are sweeps' leaves,
/// whose moves run one each ([`a_moved_leaf_runs_one_finish`]), so under
/// the twist the document runs the blocks' two and no more.
#[cfg(debug_assertions)]
#[test]
fn a_mirror_and_a_twist_of_a_finished_value_run_no_finish() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive("a-mirror-finishes-none"), tol);
    let (doc, whole) = finished_value(doc);
    let plain = finishes_evaluating(&doc, &EvalOptions::default());
    assert_eq!(
        finishes_evaluating(&mirrored(&doc, whole), &EvalOptions::default()),
        plain,
        "the mirror of a finished value runs no finish"
    );
    assert_eq!(
        finishes_evaluating(&doc, &twisted()),
        plain + 2,
        "the twist runs each block leaf's one finish and none for the union"
    );
}

/// **A moved leaf runs exactly one finish, until the box-scalar switch is
/// deleted** (`topo::LeafBody::moved`): a sweep's leaf moved at `f64` is
/// finished again on its image, so its held record is what the box
/// lane's taker decides of the same body.
#[cfg(debug_assertions)]
#[test]
fn a_moved_leaf_runs_one_finish() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive("a-moved-leaf-finishes-once"), tol);
    let (doc, source) = block(doc);
    let plain = finishes_evaluating(&doc, &EvalOptions::default());
    assert_eq!(
        finishes_evaluating(&mirrored(&doc, source), &EvalOptions::default()),
        plain + 1,
        "the mirror of a leaf runs one finish"
    );
    assert_eq!(
        finishes_evaluating(&doc, &twisted()),
        plain + 1,
        "the twist of a leaf runs one finish"
    );
}
