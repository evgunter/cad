//! **The gather finishes the aggregate once, and runs the census once.**
//!
//! The gather reads its sources and takes none. It computes in its first
//! source's frame (`Framed::computing_among`). A source in that
//! frame is grafted as its value holds it, and one standing under another
//! is grafted as its image, its data moved into the product's frame and
//! the session box asked of it (`Framed::graft_into`,
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
/// gathered: no finish.** The three stand under three maps, so the
/// gather computes in their space and grafts each into it through its
/// placement (`topo::boolean::graft_placed`): each image is construction
/// state, mapped and asked the session box. The aggregate was finished
/// at the `⊔`'s door, and the gather reads that body. Red if a mapped
/// source is finished on its image (`topo::LeafBody::moved`, one more
/// each), a source is finished again where it is taken, or the gather
/// finishes the aggregate a second time.
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
        finishes, 0,
        "none: a mapped source is grafted as construction state, not \
         finished on its image, and the gather reads the body the `⊔`'s \
         own finish decided"
    );
}

/// **A gather finishes its aggregate at one site and validates nothing
/// else** in its own text. Red if a per-piece pass comes back (a second
/// finish, or a validator call of the gather's own).
#[test]
fn the_gather_finishes_the_aggregate_once() {
    let code = blanked(code_only, "editor-core/src/product.rs", PRODUCT);
    let what = "editor-core/src/product.rs (code view)";
    let finishes = required_matches(&code, what, "T::finish_assembled(");
    assert_eq!(finishes.len(), 1, "{what}: one finish, of the aggregate");
    for gate in [
        "T::finish(",
        "topo::finish(",
        "topo::finish_assembled(",
        "validate_geometric(",
        "validate_pseudomanifold(",
    ] {
        assert_eq!(
            code.matches(gate).count(),
            0,
            "{what}: `{gate}` is called outside the aggregate's finish"
        );
    }
}

/// **The at-rest census runs inside the gather's finish, and only
/// there.** The aggregate's finish is the `⊔`'s gate
/// (`topo::AtRestPolicy::finish_assembled`), whose census records each
/// touch between two copies on the finished body and hands back its
/// findings, which the gather keeps on the product (`Product::census`);
/// the at-rest check reads them and calls no census of its own. Red if
/// the check runs the census again, or the gather calls it from a second
/// site: an extra pass over one product changes no verdict, so no
/// behavioural row can see it.
#[test]
fn the_census_runs_in_the_gather_and_never_at_the_check() {
    let product = blanked(code_only, "editor-core/src/product.rs", PRODUCT);
    for census in ["census_at_rest", "record_at_rest("] {
        assert!(
            !product.contains(census),
            "editor-core/src/product.rs: the gather's census is its finish's, not `{census}`"
        );
    }
    for (path, source) in [
        ("editor-core/src/checks/at_rest.rs", AT_REST),
        ("editor-core/src/checks.rs", CHECKS),
    ] {
        let code = blanked(code_only, path, source);
        assert!(
            !code.contains("census_at_rest")
                && !code.contains("record_at_rest(")
                && !code.contains("finish_assembled("),
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

/// **A copy placed past the session box refuses at its placement, typed,
/// before any gather moves it**: the door that places a copy asks the box
/// of where it stands (`Framed::asked_where_it_stands`,
/// `topo::session_box_at`). A block placed 2 km off, beside one at the
/// origin, under the `⊔` the product reads: the far placement refuses
/// with the session box's finding alone, and every node that reads it
/// reports that one refusal, named at the placement.
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
    // Every node that reports a refusal reports the one at the far
    // placement, and it is the session box's finding alone: the `⊔`
    // and the product refuse of nothing their own.
    let refusals: Vec<_> = doc
        .ids()
        .into_iter()
        .filter_map(|id| ev.node_error(id).map(|e| (id, e)))
        .collect();
    assert!(
        refusals.iter().any(|&(id, _)| id == far),
        "the far placement refuses: {refusals:?}"
    );
    for (id, refused) in &refusals {
        assert!(
            refused.node == far
                && matches!(
                    &refused.kind,
                    editor_core::NodeErrorKind::NotFinished { findings }
                        if matches!(&findings[..], [topo::ValidationError::OutsideSessionBox { .. }])
                ),
            "{id:?} reports the far placement's box refusal alone: {refused:?}"
        );
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

/// **A member placed past the session box refuses at its own
/// placement, and the product reads it refused** (`[placement.frame]`:
/// the door that places a copy asks where it stands). A row of two blocks
/// 2 km apart, read by the product whole: the row's second placement is
/// past the box, so the row refuses with the session box's finding
/// alone, before any gather grafts it, and the product does not build.
#[test]
fn a_member_past_the_box_refuses_at_its_placement_before_the_product() {
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
    let refused = ev.node_error(row).expect("the row refuses");
    assert!(
        matches!(
            &refused.kind,
            editor_core::NodeErrorKind::MemberRefused { source, .. }
                if matches!(
                    &**source,
                    editor_core::NodeErrorKind::NotFinished { findings }
                        if matches!(&findings[..], [topo::ValidationError::OutsideSessionBox { .. }])
                )
        ),
        "the row refuses at its far member's placement, with the session box's finding alone: {refused:?}"
    );
    let product = editor_core::product_recorded(&doc, &ev, tol);
    assert!(
        !matches!(
            product,
            Ok(_) | Err(editor_core::ProductError::Frame { .. })
        ),
        "the product reads a refused source, and no gather refuses it: {:?}",
        product.as_ref().map(|_| ())
    );
}

/// An operation over two copies of a block.
#[derive(Clone, Copy, Debug)]
enum OverCopies {
    /// `⊔`.
    Assemble,
    /// `∪`.
    Union,
    /// `⊖`, the first cut by the second.
    Subtract,
}

/// Two copies of a block placed in the world under `at`, and `op` over
/// them: the document, the copies and the operation's node.
fn copies_under(
    name: &str,
    op: OverCopies,
    at: [&editor_core::Frame; 2],
) -> (ProfileDoc, [RecipeNodeId; 2], RecipeNodeId) {
    use editor_core::Bodies;
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive(name), tol);
    let (doc, source) = block(doc);
    let (doc, a) = editor_core::test_support::in_world(doc, source, at[0], tol);
    let (doc, b) = editor_core::test_support::in_world(doc, source, at[1], tol);
    let members = || Bodies::spelled(vec![a.into(), b.into()]);
    let node = match op {
        OverCopies::Assemble => Node::Assemble { members: members() },
        OverCopies::Union => Node::Union { members: members() },
        OverCopies::Subtract => Node::Subtract {
            from: a.into(),
            tool: b.into(),
        },
    };
    let (doc, node) = fixture::insert(doc, node);
    (doc, [a, b], node)
}

fn evaluated(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// The body value `node` built.
fn built(ev: &Evaluation<f64>, node: RecipeNodeId) -> &editor_core::Framed<f64> {
    match ev.nodes.get(&node) {
        Some(editor_core::NodeResult::Ok(v)) => v.payload.output_body(0).expect("one body"),
        other => panic!("{node:?} builds: {other:?}"),
    }
}

/// **Copies at `+900 m` and `−900 m` refuse under `⊔`, `∪` and `⊖`, an
/// over-refusal pinned** (`work/onehome/an-operation-over-far-apart-copies-has-no-conditioned-computing-frame.md`). Each copy stands inside the session box,
/// but the operation computes in the first copy's frame
/// (`Framed::computing_among`), where the second stands 1800 m off, past
/// the box: the operation refuses at its placement of the second, with
/// the session box's finding alone. The `∪` and the `⊖` place it in the
/// kernel's door, and say which copy was read into which's frame
/// (`NodeErrorKind::UnfinishedLeaf`); the `⊔` grafts it, and refuses as
/// its graft does. No frame `[placement.frame]` names
/// conditions this: the space's is the world's, and a recentred one breaks
/// the box's containment (the filed row).
///
/// *Breaks if* the operation computes in the space's coordinates: the
/// copies reach 901 m there, and it builds.
fn copies_either_side_of_the_origin_refuse(op: OverCopies) {
    let (doc, [near, far], node) = copies_under(
        &format!("copies-either-side-{op:?}"),
        op,
        [
            &editor_core::Frame::translation([900.0, 0.0, 0.0]),
            &editor_core::Frame::translation([-900.0, 0.0, 0.0]),
        ],
    );
    let ev = evaluated(&doc);
    let refused = ev.node_error(node).expect("the operation refuses");
    let the_box = |findings: &topo::NotFinished| {
        matches!(
            &findings[..],
            [topo::ValidationError::OutsideSessionBox { .. }]
        )
    };
    let said = match (&refused.kind, op) {
        (editor_core::NodeErrorKind::NotFinished { findings }, OverCopies::Assemble) => {
            the_box(findings)
        }
        (
            editor_core::NodeErrorKind::UnfinishedLeaf {
                input,
                into,
                findings,
            },
            OverCopies::Union | OverCopies::Subtract,
        ) => *input == far && *into == Some(near) && the_box(findings),
        _ => false,
    };
    assert!(
        refused.node == node && said,
        "{op:?} refuses at the operation, with the session box's finding alone: {refused:?}"
    );
}

/// [`copies_either_side_of_the_origin_refuse`] under `⊔`.
#[test]
fn copies_either_side_of_the_origin_refuse_to_assemble() {
    copies_either_side_of_the_origin_refuse(OverCopies::Assemble);
}

/// [`copies_either_side_of_the_origin_refuse`] under `∪`.
#[test]
fn copies_either_side_of_the_origin_refuse_to_unite() {
    copies_either_side_of_the_origin_refuse(OverCopies::Union);
}

/// [`copies_either_side_of_the_origin_refuse`] under `⊖`.
#[test]
fn copies_either_side_of_the_origin_refuse_to_subtract() {
    copies_either_side_of_the_origin_refuse(OverCopies::Subtract);
}

/// **An operation over copies computes in its first operand's frame, bit
/// for bit** (`[placement.frame]`: "an operation over copies in its first
/// operand's"): copies at `400 m` and `402 m`. `⊔`, `∪` and `⊖` compute in
/// the first copy's frame, its map as placed, and the result's
/// coordinates are the first block's own and the second's moved 2 m along
/// `x`, exactly.
///
/// *Breaks if* the operation computes in the space's coordinates: every
/// coordinate moves 400 m.
#[test]
fn an_operation_over_copies_computes_in_its_first_operands_frame_bit_for_bit() {
    let near = editor_core::Frame::translation([400.0, 0.0, 0.0]);
    let next = editor_core::Frame::translation([402.0, 0.0, 0.0]);
    for op in [
        OverCopies::Assemble,
        OverCopies::Union,
        OverCopies::Subtract,
    ] {
        let (doc, [a, _], node) = copies_under(&format!("copies-near-{op:?}"), op, [&near, &next]);
        let ev = evaluated(&doc);
        let first = built(&ev, a);
        let value = built(&ev, node);
        assert_eq!(
            format!("{:?}", editor_core::in_space(value)),
            format!("{:?}", editor_core::in_space(first)),
            "{op:?} computes in the first copy's frame, its map as placed"
        );
        let sorted = |points: Vec<[f64; 3]>| {
            let mut points = points;
            points.sort_by(|p, q| p.partial_cmp(q).expect("finite"));
            points.dedup();
            points
        };
        let own: Vec<[f64; 3]> = first
            .body()
            .vertex_points()
            .map(|(_, p)| [p.x, p.y, p.z])
            .collect();
        let mut expected = own.clone();
        if !matches!(op, OverCopies::Subtract) {
            expected.extend(own.iter().map(|p| [p[0] + 2.0, p[1], p[2]]));
        }
        let got: Vec<[f64; 3]> = value
            .body()
            .vertex_points()
            .map(|(_, p)| [p.x, p.y, p.z])
            .collect();
        assert_eq!(
            sorted(got),
            sorted(expected),
            "{op:?}'s coordinates, in the first copy's frame"
        );
    }
}

/// **A product over copies more than the box apart refuses as its
/// aggregate, with the box's finding**: a row of two blocks at `+900 m`
/// and `−900 m`, each placed inside the box, read by the product whole.
/// The gather computes in the first's frame, where the second stands
/// 1800 m off, so its graft of the second refuses, and the product
/// refuses as its aggregate (`ProductError::ProductInvalid`, whose
/// findings are the session box's alone), not as a frame no source could
/// be read into (`ProductError::Frame`, the kernel-defect recourse). The
/// over-refusal is `work/onehome/an-operation-over-far-apart-copies-has-no-conditioned-computing-frame.md`'s.
#[test]
fn a_product_over_copies_past_the_box_apart_refuses_as_its_aggregate() {
    let tol = Tol::witness();
    let (doc, row) = row_in_world(
        "a-product-past-the-box-apart",
        &editor_core::Frame::translation([-900.0, 0.0, 0.0]),
        1800.0,
        2,
    );
    let ev = evaluated(&doc);
    assert!(
        ev.node_error(row).is_none(),
        "every member stands in the box where it is placed: {:?}",
        ev.node_error(row)
    );
    let product = editor_core::product_recorded(&doc, &ev, tol);
    assert!(
        matches!(
            &product,
            Err(editor_core::ProductError::ProductInvalid { findings })
                if matches!(findings.as_slice(), [topo::ValidationError::OutsideSessionBox { .. }])
        ),
        "the product refuses as its aggregate, with the session box's finding alone: {:?}",
        product.as_ref().map(|_| ())
    );
}

/// A part of two blocks, at `x = 0` and `x = 2` in its own world, its
/// product their `⊔`, and a document instancing it once, pinned at `at`.
fn a_sub_assembly_placed_at(
    label: &str,
    at: [f64; 3],
) -> (ProfileDoc, RecipeNodeId, editor_core::EvalOptions) {
    use fixture::resolver::{PartStore, with_resolver};
    let tol = Tol::witness();
    let part = ProfileDoc::empty(DocumentId::derive(&format!("{label}-part")), tol);
    let (part, source) = block(part);
    let (part, _) = editor_core::test_support::in_world(
        part,
        source,
        &editor_core::Frame::translation([0.0; 3]),
        tol,
    );
    let (part, _) = editor_core::test_support::in_world(
        part,
        source,
        &editor_core::Frame::translation([2.0, 0.0, 0.0]),
        tol,
    );
    let part = fixture::assemble_world_copies(part);
    let mut store = PartStore::default();
    let part = store.insert(part, tol);
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), tol);
    let (doc, instance) = fixture::insert(doc, fixture::mated_instance(part));
    let doc = fixture::pin(doc, instance, &editor_core::Frame::translation(at), &opts);
    (fixture::assemble_world_copies(doc), instance, opts)
}

/// **A sub-assembly placed far out computes in its own frame and is
/// asked the box once, where it stands** (`[placement.frame]`). A part's
/// two blocks gather in the part's own space, and the document that
/// instances it 900 m out reads the instance's product as its one
/// source, so its product computes in the instance's frame, not its
/// space's, and reaches about 903 m where it stands. Pinned 998 m out,
/// the same part reaches past the box: the instance's placement refuses,
/// with the session box's finding alone, and no gather refuses it.
#[test]
fn a_sub_assembly_placed_far_out_is_asked_once_where_it_stands() {
    let tol = Tol::witness();
    let (doc, _, opts) = a_sub_assembly_placed_at("sub-assembly-far", [900.0, 0.0, 0.0]);
    let ev: Evaluation<f64> = evaluate(&doc, None, &CancelToken::new(), &opts, tol);
    let product = editor_core::product_recorded(&doc, &ev, tol)
        .unwrap_or_else(|e| panic!("the product builds: {e:?}"));
    assert!(
        editor_core::in_space(&product).is_some(),
        "the product computes in the instance's frame"
    );
    let world = fixture::product_in_world(&doc, &ev, tol).expect("it composes");
    let reach = world
        .vertex_points()
        .map(|(_, p)| p.x.abs().max(p.y.abs()).max(p.z.abs()))
        .fold(0.0_f64, f64::max);
    assert_eq!(reach, 903.0, "its far block's far corner, where it stands");
    let (doc, instance, opts) = a_sub_assembly_placed_at("sub-assembly-past", [998.0, 0.0, 0.0]);
    let ev: Evaluation<f64> = evaluate(&doc, None, &CancelToken::new(), &opts, tol);
    let refused = ev.node_error(instance).expect("the instance refuses");
    assert!(
        refused.node == instance
            && matches!(
                &refused.kind,
                editor_core::NodeErrorKind::NotFinished { findings }
                    if matches!(&findings[..], [topo::ValidationError::OutsideSessionBox { .. }])
            ),
        "at its placement, with the session box's finding alone: {refused:?}"
    );
    assert!(
        !matches!(
            editor_core::product_recorded(&doc, &ev, tol),
            Ok(_) | Err(editor_core::ProductError::Frame { .. })
        ),
        "the product reads a refused source, and no gather refuses it"
    );
}

/// A quarter turn about `z`, then `by`.
fn turned(by: [f64; 3]) -> editor_core::Frame {
    editor_core::Frame {
        columns: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        translation: by,
    }
}

/// A block placed in the world under a turned frame, and its mirrors
/// across the planes `x = 0.7` and `x = 0.3` of its own coordinates,
/// gathered by a `⊔` the product reads. A mirror keeps its operand's
/// frame (`wire_mirror`), so all three hold one map: the product
/// computes in that frame, not its space's, reading no copy through
/// another's ([`editor_core::eval::Framed::computing_among`]). The block spans
/// `x ∈ [0, 1]` in its own coordinates and the mirrors `[0.4, 1.4]` and
/// `[−0.4, 0.6]`, so every pair of the three overlaps.
fn mirrors_in_one_turned_frame(name: &str) -> ProfileDoc {
    use editor_core::pose::{PoseDef, RoleRead};
    use editor_core::{Bodies, Operand};
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive(name), tol);
    let (doc, source) = block(doc);
    let (doc, copy) =
        editor_core::test_support::in_world(doc, source, &turned([3.0, 1.0, 2.0]), tol);
    let mirror_at = |doc: ProfileDoc, x: f64| {
        let wall = fixture::plane_on(&doc, copy, [1.0, 0.5, 0.5], [1.0, 0.0, 0.0]);
        let along = fixture::axis_on(&doc, copy, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let plane = RoleRead::plain(Operand::Pose(Box::new(PoseDef::Slide {
            pose: wall,
            along,
            by: len(x - 1.0),
        })));
        fixture::insert(
            doc,
            Node::Mirror {
                body: Operand::output(copy, 0).into(),
                plane,
            },
        )
    };
    let (doc, near) = mirror_at(doc, 0.7);
    let (doc, far) = mirror_at(doc, 0.3);
    let (doc, gathered) = fixture::insert(
        doc,
        Node::Assemble {
            members: Bodies::spelled(vec![copy.into(), near.into(), far.into()]),
        },
    );
    product_of_family(doc, gathered)
}

/// **A product in one shared frame localizes interference in that
/// frame, moving no copy**: a turned block and its two mirrors, one frame
/// among them, every pair overlapping ([`mirrors_in_one_turned_frame`]).
/// The product computes in their frame, not its space's, and the at-rest
/// intersection reads each copy where it stands in it: the checks run 9
/// finishes, measured. *Breaks if* the intersection reads the copies in
/// the space's frame rather than the product's (`checks::at_rest`), which
/// moves each into it.
#[cfg(debug_assertions)]
#[test]
fn a_product_in_one_shared_frame_localizes_in_it_moving_no_copy() {
    let tol = Tol::witness();
    let doc = mirrors_in_one_turned_frame("localize-in-a-shared-frame");
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
    assert!(
        found.len() >= 3,
        "every pair of the row interferes: {found:?}"
    );
    assert!(
        found
            .iter()
            .all(|f| matches!(f.overlap, editor_core::Overlap::Bounded { .. })),
        "and each overlap is bounded: {found:?}"
    );
    assert_eq!(
        finishes, 9,
        "the checks' finishes, no copy moved into the product's frame"
    );
}

/// **Localizing interference takes each copy once**, however many
/// interfering pairs it is in: a block placed in the world under a turned
/// frame and a row of three copies of it 0.4 apart, every pair of the four
/// overlapping, so each copy is in several interfering pairs. The product
/// computes in the first copy's frame. Localization takes each copy
/// finished in its own frame and placed in the product's frame, once
/// (`checks::at_rest::Operands`), and each pair hands its two to the placed
/// door (`topo::boolean_placed`), which moves each one not in that frame,
/// carrying its verdict, and finishes nothing. The checks run 26 finishes
/// and make 7 placements (`EvalCounters::rigid_maps`), measured. *Breaks
/// if* a copy is taken per pair: 15 placements. That the move itself is
/// made per pair is
/// `work/partframes/the-at-rest-localization-moves-each-copy-once-per-pair.md`.
#[cfg(debug_assertions)]
#[test]
fn localizing_takes_each_copy_once() {
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
        "the premise: the product computes in the first copy's frame, not its space's"
    );
    let before = topo::finishes_on_this_thread();
    let counted = editor_core::EvalCounters::on_this_thread();
    let report = editor_core::run_checks(
        &doc,
        &ev,
        &editor_core::ChecksConfig::default(),
        &editor_core::Stop::default(),
        tol,
    )
    .unwrap_or_else(|e| panic!("the checks run: {e}"));
    let finishes = topo::finishes_on_this_thread() - before;
    let placed = editor_core::EvalCounters::on_this_thread()
        .since(counted)
        .rigid_maps;
    let found = fixture::interference(&report.findings);
    assert!(
        found.len() >= 3,
        "every pair of the row interferes: {found:?}"
    );
    assert!(
        found
            .iter()
            .all(|f| matches!(f.overlap, editor_core::Overlap::Bounded { .. })),
        "and each overlap is bounded: {found:?}"
    );
    assert_eq!(
        (finishes, placed),
        (26, 7),
        "the checks' finishes, and the copies placed in the product's frame"
    );
}
