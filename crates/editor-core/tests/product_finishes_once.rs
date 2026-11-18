//! **The gather finishes the aggregate once, and runs the census once.**
//!
//! The gather reads its sources and takes none: a source in the
//! product's frame is grafted as its value holds it, and one standing
//! under another frame is grafted as its image, its data moved into the
//! product's and the session box asked of it (`Framed::graft_in`,
//! `topo::boolean::graft_placed`), which finishes nothing. The
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
/// gathered: one finish.** The gather computes in the first copy's frame
/// and grafts the other two into it through their relative motion
/// (`topo::boolean::graft_placed`): each image is construction state, mapped and
/// asked the session box, and the aggregate is what finishes, once. Red
/// if a mapped source is finished on its image (`topo::LeafBody::moved`,
/// two more), or a source is finished again where it is taken.
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
        finishes, 1,
        "one finish, of the aggregate: a mapped source is grafted as \
         construction state, not finished on its image"
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
    let (doc, b) = block(doc);
    // The second block joins the first's space, half a block over.
    let (doc, b) = fixture::against(
        doc,
        a,
        b,
        &editor_core::Frame::translation([0.5, 0.25, 0.0]),
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

/// **A copy moved past the session box refuses at the box, typed**: the
/// door that grafts it asks the box of its image
/// (`topo::boolean::graft_placed`). A block placed 2 km off evaluates as
/// its placement, in its own frame; the `Assemble` that grafts it into
/// the first copy's frame moves it, and refuses with the session box's
/// finding alone. The refusal names that `Assemble`, the operation whose
/// graft moved the copy, not the far copy's placement: no `NodeErrorKind`
/// arm names a member (the product's own gather does name it:
/// [`a_member_past_the_box_refuses_the_product_at_its_source`]).
#[test]
fn a_copy_moved_past_the_box_refuses_at_the_box() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive("a-copy-past-the-box"), tol);
    let (doc, source) = block(doc);
    let (doc, _near) = editor_core::test_support::in_world(
        doc,
        source,
        &editor_core::Frame::translation([0.0; 3]),
        tol,
    );
    let (doc, far) = editor_core::test_support::in_world(
        doc,
        source,
        &editor_core::Frame::translation([2000.0, 0.0, 0.0]),
        tol,
    );
    let doc = fixture::assemble_world_copies(doc);
    let ev: Evaluation<f64> = evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    assert!(
        ev.node_error(far).is_none(),
        "the far copy's placement evaluates in its own frame"
    );
    let refusals: Vec<_> = doc
        .ids()
        .into_iter()
        .filter_map(|id| ev.node_error(id))
        .collect();
    let [refused] = &refusals[..] else {
        panic!("one node refuses: {refusals:?}")
    };
    assert!(
        matches!(doc.node(refused.node), Some(Node::Assemble { .. })),
        "the Assemble reading the copy refuses"
    );
    match &refused.kind {
        editor_core::NodeErrorKind::NotFinished { findings } => assert!(
            matches!(
                &findings[..],
                [topo::ValidationError::OutsideSessionBox { .. }]
            ),
            "the session box is the move's one refusal: {findings:?}"
        ),
        other => panic!("the Assemble refuses at the box: {other:?}"),
    }
}

/// `doc` with its product the family `row`, every member of it.
fn product_of_family(doc: ProfileDoc, row: RecipeNodeId) -> ProfileDoc {
    editor_core::apply(
        &doc,
        &editor_core::DocEdit::SetProduct {
            product: Some(row.into()),
        },
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the product reads the family: {e}"))
    .doc
}

/// A block placed in the world at `at`, and a row of `count` copies of
/// that copy along `x`, `spacing` apart: the document and the row.
fn row_in_world(
    name: &str,
    at: &editor_core::Frame,
    spacing: f64,
    count: i64,
) -> (ProfileDoc, RecipeNodeId) {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive(name), tol);
    let (doc, source) = block(doc);
    let (doc, copy) = editor_core::test_support::in_world(doc, source, at, tol);
    let (doc, _, row) = fixture::linear(doc, copy, [1.0, 0.0, 0.0], spacing, count);
    (product_of_family(doc, row), row)
}

/// **A member placed past the session box refuses the product at its
/// source, typed** (`ProductError::Frame`): a row of two blocks 2 km
/// apart, read by the product whole. Each member stands in its own frame
/// and evaluates; the gather computes in the first's, and the door that
/// grafts the second into it (`topo::boolean::graft_placed`) asks the box
/// of its image and refuses there, naming the row the member is read
/// from. *Breaks if* that ask is dropped: the aggregate's own finish then
/// refuses the product as a whole (`ProductError::ProductInvalid`), with
/// no source named.
#[test]
fn a_member_past_the_box_refuses_the_product_at_its_source() {
    let tol = Tol::witness();
    let (doc, row) = row_in_world(
        "a-member-past-the-box",
        &editor_core::Frame::translation([0.0; 3]),
        2000.0,
        2,
    );
    let ev: Evaluation<f64> = evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    assert!(ev.node_error(row).is_none(), "the row evaluates, member by member");
    match editor_core::product_recorded(&doc, &ev, tol) {
        Err(editor_core::ProductError::Frame { node, kind }) => {
            assert_eq!(node, row, "the refusal names the row the member is read from");
            assert!(
                matches!(
                    &*kind,
                    editor_core::NodeErrorKind::NotFinished { findings }
                        if matches!(&findings[..], [topo::ValidationError::OutsideSessionBox { .. }])
                ),
                "with the session box's finding alone: {kind:?}"
            );
        }
        other => panic!("the product refuses at the far member: {other:?}"),
    }
}

/// A quarter turn about `z`, then `by`.
fn turned(by: [f64; 3]) -> editor_core::Frame {
    editor_core::Frame {
        columns: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        translation: by,
    }
}

/// **Localizing interference reads each copy into the product's frame
/// once**, however many interfering pairs it is in: a block placed in
/// the world under a turned frame (so the product's frame is not its
/// space's) and a row of three copies of it 0.4 apart, every pair of the
/// four overlapping, so each copy is in several interfering pairs. A
/// copy moved into the product's frame is a moved leaf, which runs one
/// finish ([`a_moved_leaf_runs_one_finish`]). The checks run 31 finishes,
/// measured; read once per pair, as before `checks::at_rest`'s
/// `Operands`, they run 39. *Breaks if* a copy is read per pair.
#[cfg(debug_assertions)]
#[test]
fn localizing_reads_each_copy_into_the_products_frame_once() {
    let tol = Tol::witness();
    let (doc, _) = row_in_world("localize-once", &turned([3.0, 1.0, 2.0]), 0.4, 3);
    let ev: Evaluation<f64> = evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    let product = editor_core::product_recorded(&doc, &ev, tol).expect("the row gathers");
    assert!(
        editor_core::in_space(&product).is_some(),
        "the premise: the product computes in a frame that is not its space's"
    );
    let before = topo::finishes_on_this_thread();
    let report = editor_core::run_checks(
        &doc,
        &ev,
        &editor_core::ChecksConfig::default(),
        &editor_core::Stop::default(),
        tol,
    )
    .unwrap_or_else(|e| panic!("the checks run: {e}"));
    let finishes = topo::finishes_on_this_thread() - before;
    let found = fixture::interference(&report.findings);
    assert!(found.len() >= 3, "every pair of the row interferes: {found:?}");
    assert!(
        found
            .iter()
            .all(|f| matches!(f.overlap, editor_core::Overlap::Bounded { .. })),
        "and each overlap is bounded: {found:?}"
    );
    assert_eq!(
        finishes, 31,
        "the checks' finishes, each copy read into the product's frame once"
    );
}
