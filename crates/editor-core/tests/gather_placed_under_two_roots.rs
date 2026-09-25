//! **One body placed under two roots refuses at the recipe** (GATHER).
//!
//! A transform and a part selection mint no name (N1), so two product
//! roots that reach one node through them alone would both carry that
//! node's names into the product's one table. The gather refuses that
//! shape as `ProductError::PlacedUnderTwoRoots`, naming the node and
//! both roots, before it reads any root's value. The rows below build
//! the shape through every edge the check follows, the mated document
//! the solve accepts, and the placements beside it that are legal and
//! still gather.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, Datum, DocEdit, DocumentId, EntityKind, Entry,
    EvalOptions, Evaluation, Expr, MateFrame, MatePrimitive, MateRole, NameTable, Node, PartSelect,
    PatternKind, ProductError, ProductErrorKind, ProfileDoc, RecipeNodeId, SplitHalf, StableName,
    product, product_named,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{head_at, insert, len, on_frame, scl, solve, step, xform};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    fixture::run(doc, &EvalOptions::default())
}

/// A `w`×`w`×`h` block at the origin; answers the extrude.
fn block(doc: ProfileDoc, w: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(h),
        },
    )
}

fn shifted(doc: ProfileDoc, input: RecipeNodeId, dx: f64) -> (ProfileDoc, RecipeNodeId) {
    insert(doc, xform(input, [dx, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0))
}

/// The refusal's four fields, or a panic naming what came back.
fn placed_twice(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
) -> (RecipeNodeId, Option<PartSelect>, RecipeNodeId, RecipeNodeId) {
    match product(doc, ev, Tol::witness()) {
        Err(ProductError::PlacedUnderTwoRoots {
            placed,
            select,
            first,
            second,
        }) => (placed, select, first, second),
        other => panic!(
            "expected PlacedUnderTwoRoots, got {:?}",
            other.map(|b| b.solids().count())
        ),
    }
}

/// **Two transforms of one extrude**, both roots: the refusal names
/// the extrude and the two transforms, in root order, and says so in
/// the recipe's words rather than as a name collision.
#[test]
fn two_transforms_of_one_extrude_refuse_naming_the_extrude_and_both_roots() {
    let doc = ProfileDoc::empty_derived("gather-two-roots-xform", Tol::witness());
    let (doc, extrude) = block(doc, 1.0, 1.0);
    let (doc, t1) = shifted(doc, extrude, 2.0);
    let (doc, t2) = shifted(doc, extrude, 4.0);
    assert_eq!(
        doc.roots(),
        &[t1, t2][..],
        "the premise: both transforms root"
    );
    let ev = run(&doc);

    let err = product(&doc, &ev, Tol::witness()).expect_err("one body under two roots");
    assert_eq!(err.kind(), ProductErrorKind::PlacedUnderTwoRoots);
    let message = err.to_string();
    for needle in [
        format!("node {}'s body", extrude.0),
        format!("two roots, {} and {}", t1.0, t2.0),
    ] {
        assert!(
            message.contains(&needle),
            "{needle:?} missing from {message:?}"
        );
    }
    assert!(
        !message.contains("collides"),
        "the recipe's vocabulary, not the name table's: {message}"
    );
    assert_eq!(placed_twice(&doc, &ev), (extrude, None, t1, t2));
}

/// **The refusal reads the recipe, not the evaluation.** An evaluation
/// taken before the second transform existed has no entry for it, so a
/// check sited after the roots are read would refuse `UnknownNode`.
#[test]
fn the_refusal_precedes_every_root_read() {
    let doc = ProfileDoc::empty_derived("gather-two-roots-early", Tol::witness());
    let (doc, extrude) = block(doc, 1.0, 1.0);
    let (doc, t1) = shifted(doc, extrude, 2.0);
    let stale = run(&doc);
    let (doc, t2) = shifted(doc, extrude, 4.0);
    assert!(
        stale.result(t2).is_none(),
        "the premise: the evaluation predates t2"
    );
    assert_eq!(placed_twice(&doc, &stale), (extrude, None, t1, t2));
}

/// **Chains that meet above the minter** name the node where they
/// meet: an extrude moved once, and that move moved twice, places the
/// MOVE under two roots.
#[test]
fn chains_that_meet_at_a_transform_name_that_transform() {
    let doc = ProfileDoc::empty_derived("gather-two-roots-chain", Tol::witness());
    let (doc, extrude) = block(doc, 1.0, 1.0);
    let (doc, t0) = shifted(doc, extrude, 1.5);
    let (doc, t1) = shifted(doc, t0, 2.0);
    let (doc, t2) = shifted(doc, t0, 4.0);
    let ev = run(&doc);
    let (placed, select, first, second) = placed_twice(&doc, &ev);
    assert_eq!(
        placed, t0,
        "the meeting point nearest the roots, not {extrude:?}"
    );
    assert_eq!((select, first, second), (None, t1, t2));
}

/// A block split at x = 0.5; answers (doc, split).
fn split_block(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, target) = block(doc, 1.0, 1.0);
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.5), len(0.0), len(0.0)],
            normal: [scl(1.0), scl(0.0), scl(0.0)],
        }),
    );
    insert(
        doc,
        Node::Split {
            target,
            tool: plane,
        },
    )
}

fn half(doc: ProfileDoc, of: RecipeNodeId, h: SplitHalf) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Part {
            of,
            select: PartSelect::SplitHalf(h),
        },
    )
}

/// **The same half selected twice** refuses, and carries the
/// selection; the message says which half.
#[test]
fn one_half_under_two_roots_refuses_naming_the_half() {
    let (doc, split) = split_block("gather-two-roots-half");
    let (doc, p1) = half(doc, split, SplitHalf::Above);
    let (doc, p2) = half(doc, split, SplitHalf::Above);
    let (doc, moved) = shifted(doc, p2, 3.0);
    let ev = run(&doc);
    let err = product(&doc, &ev, Tol::witness()).expect_err("one half under two roots");
    assert!(
        err.to_string()
            .contains(&format!("the above half of node {}", split.0)),
        "{err}"
    );
    assert_eq!(
        placed_twice(&doc, &ev),
        (
            split,
            Some(PartSelect::SplitHalf(SplitHalf::Above)),
            p1,
            moved
        )
    );
}

/// **A pattern moved whole, beside one of its instances**: the whole
/// placement overlaps every selection, so the pair refuses and the
/// shared selection is `None` — one of the two took the value whole.
#[test]
fn a_whole_pattern_and_one_of_its_instances_refuse() {
    let doc = ProfileDoc::empty_derived("gather-two-roots-pattern", Tol::witness());
    let (doc, extrude) = block(doc, 1.0, 1.0);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: extrude,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, whole) = insert(doc, xform(pattern, [0.0, 5.0, 0.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, one) = insert(
        doc,
        Node::Part {
            of: pattern,
            select: PartSelect::Instance(Expr::count(1)),
        },
    );
    let ev = run(&doc);
    assert_eq!(placed_twice(&doc, &ev), (pattern, None, whole, one));
}

/// **One instance selected by two roots**: the refusal carries the
/// selection, and the message says WHICH instance.
#[test]
fn one_instance_under_two_roots_refuses_naming_the_instance() {
    let doc = ProfileDoc::empty_derived("gather-two-roots-instance", Tol::witness());
    let (doc, extrude) = block(doc, 1.0, 1.0);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: extrude,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let pick = |doc| {
        insert(
            doc,
            Node::Part {
                of: pattern,
                select: PartSelect::Instance(Expr::count(1)),
            },
        )
    };
    let (doc, first) = pick(doc);
    let (doc, second) = pick(doc);
    let ev = run(&doc);
    let err = product(&doc, &ev, Tol::witness()).expect_err("instance 1 twice");
    assert!(
        err.to_string()
            .contains(&format!("instance `1` of node {}", pattern.0)),
        "{err}"
    );
    assert_eq!(
        placed_twice(&doc, &ev),
        (
            pattern,
            Some(PartSelect::Instance(Expr::count(1))),
            first,
            second
        )
    );
}

/// **The legal placements beside those shapes still gather.** One
/// root over one transform; two transforms of one body unioned into
/// one root; the two DIFFERENT halves of one split as two roots; and
/// two different instances of one pattern as two roots. Each is
/// checked for the face count it should gather — six per block, five
/// per half-block plus its cut — so a check that refused one of them,
/// or a gather that dropped a body, reds here.
#[test]
fn legal_placements_still_gather() {
    let faces = |doc: &ProfileDoc| {
        product(doc, &run(doc), Tol::witness())
            .map(|b| b.faces().count())
            .map_err(|e| e.to_string())
    };

    let doc = ProfileDoc::empty_derived("gather-one-root", Tol::witness());
    let (doc, extrude) = block(doc, 1.0, 1.0);
    let (doc, t1) = shifted(doc, extrude, 2.0);
    assert_eq!(doc.roots(), &[t1][..]);
    assert_eq!(faces(&doc), Ok(6), "one transform, one root");

    let (doc, t2) = shifted(doc, extrude, 4.0);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: vec![t1, t2],
            declare: None,
        },
    );
    assert_eq!(doc.roots(), &[union][..], "the union consumes both moves");
    assert_eq!(faces(&doc), Ok(12), "two disjoint members, one root");

    let (doc, split) = split_block("gather-two-halves");
    let (doc, above) = half(doc, split, SplitHalf::Above);
    let (doc, below) = half(doc, split, SplitHalf::Below);
    assert_eq!(doc.roots(), &[above, below][..]);
    assert_eq!(
        faces(&doc),
        Ok(12),
        "two halves are two bodies, not one twice"
    );

    let doc = ProfileDoc::empty_derived("gather-two-instances", Tol::witness());
    let (doc, extrude) = block(doc, 1.0, 1.0);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: extrude,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let pick = |doc, i| {
        insert(
            doc,
            Node::Part {
                of: pattern,
                select: PartSelect::Instance(Expr::count(i)),
            },
        )
    };
    let (doc, _) = pick(doc, 0);
    let (doc, _) = pick(doc, 2);
    assert_eq!(faces(&doc), Ok(12), "two instances are two bodies");
}

// ---- the mated document the solve accepts ----

const BASE_HEIGHT: f64 = 1.0;

fn part_block(label: &str, w: f64, h: f64) -> ProfileDoc {
    block(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        w,
        h,
    )
    .0
}

/// A `Rest` seat of `b`'s bottom cap onto `a`'s top cap.
fn seat(a: editor_core::SitedFace, b: editor_core::SitedFace) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a,
        b,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame {
                origin: [1.0, 1.0, BASE_HEIGHT],
                axis: [0.0, 0.0, 1.0],
                reference: [1.0, 0.0, 0.0],
            },
            b: MateFrame {
                origin: [0.0, 0.0, 0.0],
                axis: [0.0, 0.0, -1.0],
                reference: [1.0, 0.0, 0.0],
            },
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

/// **The item's own document.** One instance `top`, fed into two
/// transforms, each mated to its own base. The solve accepts it — both
/// mates determine, and nothing is faulted — and the gather refuses it
/// in the recipe's words: `top` is placed under `t1` and `t2`.
#[test]
fn one_instance_mated_through_two_transforms_solves_and_refuses_at_the_gather() {
    let mut store = PartStore::default();
    let base_ref = store.insert(
        part_block("gather-two-roots-base", 3.0, BASE_HEIGHT),
        Tol::witness(),
    );
    let top_ref = store.insert(part_block("gather-two-roots-top", 1.0, 3.0), Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("gather-two-roots-mated"), Tol::witness());
    let (doc, base1) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, base2) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, t1) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, t2) = insert(doc, xform(top, [0.0, 0.0, 20.0], [0.0, 0.0, 1.0], 0.0));
    let mate = |doc, base, at| {
        let (doc, m) = step(
            doc,
            DocEdit::InsertNode {
                node: seat(
                    head_at(base, in_part(base, CapEnd::End)),
                    head_at(at, in_part(top, CapEnd::Start)),
                ),
            },
        );
        (doc, m.expect("the mate inserts"))
    };
    let (doc, m1) = mate(doc, base1, t1);
    let (doc, m2) = mate(doc, base2, t2);

    let poses = solve(&doc, &opts, Tol::witness());
    for node in [base1, base2, top, m1, m2] {
        assert!(
            poses.fault(node).is_none(),
            "the premise: the solve accepts ({node:?}: {:?})",
            poses.fault(node)
        );
    }
    assert_eq!(
        (poses.role(m1), poses.role(m2)),
        (Some(MateRole::Determining), Some(MateRole::Determining)),
        "both seats determine"
    );

    let ev = fixture::run(&doc, &opts);
    for node in [t1, t2] {
        assert!(
            matches!(ev.result(node), Some(editor_core::NodeResult::Ok(_))),
            "{node:?} evaluates: {:?}",
            ev.result(node)
        );
    }
    let (placed, select, first, second) = placed_twice(&doc, &ev);
    assert_eq!(placed, top, "the instance, not a base or a mate");
    assert_eq!((select, first, second), (None, t1, t2));
}

/// **The selection rides down through a transform below a part.**
/// Instance 0 read through a transform of the pattern, beside instance
/// 2 read off the pattern itself: two different instances, so two
/// bodies. A walk that dropped the selection at the transform would
/// read the first chain as taking the pattern WHOLE and refuse.
#[test]
fn rv_selection_rides_down_through_a_transform() {
    let doc = ProfileDoc::empty_derived("rv-ride", Tol::witness());
    let (doc, b) = block(doc, 1.0, 1.0);
    let (doc, p) = insert(
        doc,
        Node::Pattern {
            input: b,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, tp) = shifted(doc, p, 0.0);
    let pick = |doc, of, i| {
        insert(
            doc,
            Node::Part {
                of,
                select: PartSelect::Instance(Expr::count(i)),
            },
        )
    };
    let (doc, _) = pick(doc, tp, 0);
    let (doc, _) = pick(doc, p, 2);
    assert_eq!(
        product(&doc, &run(&doc), Tol::witness())
            .map(|b| b.faces().count())
            .map_err(|e| e.to_string()),
        Ok(12)
    );
}

// ---- a split's halves as roots over a tie the plane separates ----
//
// Two halves of one split taken as two `Part` roots place no body
// twice, and gather the product the split root gathers. The mechanism
// is the separated-piece mark (`NameTable`'s `separated` field). The
// rows below pin the merge through every verbatim edge, and pin that
// two roots carrying ONE entity still refuse.

/// A 4×4×4 block less a cutter whose prongs, each `(y0, y1)`, cross the
/// x = 4 wall at z ∈ [1, 3]: the prongs' far ends leave cap fragments
/// no covariant qualifier separates, one tie candidate per prong.
/// Answers the subtract.
fn cutter(doc: ProfileDoc, prongs: &[(f64, f64)]) -> (ProfileDoc, RecipeNodeId) {
    let (doc, a) = block(doc, 4.0, 4.0);
    let lo = prongs.first().expect("a prong").0;
    let hi = prongs.last().expect("a prong").1;
    let mut pts = vec![(2.0, lo), (6.0, lo), (6.0, hi)];
    let mut rev: Vec<(f64, f64)> = prongs.iter().rev().copied().collect();
    let top = rev.remove(0);
    pts.push((2.0, top.1));
    pts.push((2.0, top.0));
    let mut prev_lo = top.0;
    for (l, h) in rev {
        pts.push((5.0, prev_lo));
        pts.push((5.0, h));
        pts.push((2.0, h));
        pts.push((2.0, l));
        prev_lo = l;
    }
    pts.pop();
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![pts],
    );
    let (doc, c) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(2.0),
        },
    );
    insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Subtract,
            a,
            b: c,
            declare: None,
        },
    )
}

/// The cutter's subtract split at y = `y`: the document with the split
/// as the only root, and the same document with both halves taken as
/// `Part` roots instead. Answers (split-root doc, halves doc,
/// subtract, above, below).
fn halves_over_a_tie(
    label: &str,
    prongs: &[(f64, f64)],
    y: f64,
) -> (
    ProfileDoc,
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, sub) = cutter(doc, prongs);
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(y), len(0.0)],
            normal: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (whole, split) = insert(
        doc,
        Node::Split {
            target: sub,
            tool: plane,
        },
    );
    assert_eq!(whole.roots(), &[split][..], "the premise: the split alone");
    let (doc, above) = half(whole.clone(), split, SplitHalf::Above);
    let (doc, below) = half(doc, split, SplitHalf::Below);
    assert_eq!(
        doc.roots(),
        &[above, below][..],
        "the premise: two Part roots"
    );
    (whole, doc, sub, above, below)
}

/// The subtract's tied FACE names in `table`, each with its candidate
/// count.
fn tied_faces(table: &NameTable, sub: RecipeNodeId) -> Vec<(StableName, usize)> {
    table
        .iter()
        .filter(|(n, _)| n.node == sub && n.kind == EntityKind::Face)
        .filter_map(|(n, e)| match e {
            Entry::Tied(es) => Some((n.clone(), es.len())),
            Entry::Unique(_) => None,
        })
        .collect()
}

/// **The two halves gather, as the split does.** Both documents gather;
/// their product tables are EQUAL, so each name the plane separated is
/// one `Entry::Tied` over the candidates of both halves; and that tie is
/// the one the split's own table holds, with every candidate. The `Part`
/// holding one candidate still publishes the separated name `Unique`.
/// Answers the separated tie's candidate count.
///
/// The tables compare with `==`, entity keys included, which holds
/// because the roots `[Above, Below]` graft in the order the split root
/// grafts its bodies (`SplitHalf::output_body`); the aggregate's keys
/// are minted in graft order. Reordering the roots would reorder the
/// keys under the same names.
fn halves_gather_as_the_split_does(label: &str, prongs: &[(f64, f64)], y: f64) -> usize {
    let (whole, halves, sub, above, below) = halves_over_a_tie(label, prongs, y);
    let whole_ev = run(&whole);
    let (split_body, split_names) =
        product_named(&whole, &whole_ev, Tol::witness()).expect("the split root gathers");
    let ev = run(&halves);
    let (halves_body, halves_names) =
        product_named(&halves, &ev, Tol::witness()).expect("the two halves gather");
    assert_eq!(
        halves_body.faces().count(),
        split_body.faces().count(),
        "the same geometry"
    );
    assert_eq!(
        halves_names, split_names,
        "the same product name table, row for row"
    );
    let split_table = &whole_ev
        .value(whole.roots()[0])
        .expect("the split evaluated")
        .name_table;
    let published = |part: RecipeNodeId, name: &StableName| {
        ev.value(part)
            .expect("the half evaluated")
            .name_table
            .lookup(name)
            .cloned()
    };
    let (name, count) = tied_faces(&halves_names, sub)
        .into_iter()
        .find(|(n, _)| {
            matches!(published(above, n), Some(Entry::Unique(_)))
                || matches!(published(below, n), Some(Entry::Unique(_)))
        })
        .expect("a tie the plane separated, published Unique by one half");
    assert!(
        matches!(split_table.lookup(&name), Some(Entry::Tied(es)) if es.len() == count),
        "the product's tie holds every candidate the split's does"
    );
    count
}

/// **One candidate in each half.** The U-cutter's two prongs, split
/// between them at y = 2: both `Part`s publish the tied name `Unique`,
/// and the product holds it tied over both.
#[test]
fn split_halves_as_roots_over_a_one_one_tie_gather_one_tie() {
    let count =
        halves_gather_as_the_split_does("gather-halves-one-one", &[(1.0, 1.5), (2.5, 3.0)], 2.0);
    assert_eq!(count, 2, "one candidate from each half");
}

/// **One candidate in one half, two in the other.** The E-cutter's
/// three prongs, split at y = 1.5 between the first and the second: the
/// lower half publishes the name `Unique`, the upper half `Tied` over
/// two, and the product holds it tied over all three.
#[test]
fn split_halves_as_roots_over_a_one_two_tie_gather_one_tie() {
    let count = halves_gather_as_the_split_does(
        "gather-halves-one-two",
        &[(0.5, 1.0), (1.75, 2.25), (3.0, 3.5)],
        1.5,
    );
    assert_eq!(count, 3, "one candidate from one half, two from the other");
}

/// The rows `names` holds under `name`, or a panic.
fn row(names: &NameTable, name: &StableName) -> Entry {
    names.lookup(name).cloned().expect("the name has a row")
}

/// **The piece's mark rides the other verbatim edges, and changes no
/// lone half.** Over the U-cutter split at y = 2, the separated name is
/// tied over two in the split's product. A lone `Part` root gathers it
/// `Unique` — that half's product holds one candidate. A transformed
/// half beside the other half gathers it tied over both, as does the
/// upper half split AGAIN, clear of its candidate, beside the lower
/// half: the second split passes the marked row through unchanged.
#[test]
fn a_separated_piece_merges_through_a_transform_and_a_second_split() {
    let (whole, _, sub, _, _) =
        halves_over_a_tie("gather-halves-edges", &[(1.0, 1.5), (2.5, 3.0)], 2.0);
    let split = whole.roots()[0];
    let (_, split_names) =
        product_named(&whole, &run(&whole), Tol::witness()).expect("the split root gathers");
    let ties = tied_faces(&split_names, sub);
    assert!(
        !ties.is_empty() && ties.iter().all(|(_, count)| *count == 2),
        "the premise: every tie holds one candidate per half, {ties:?}"
    );
    let every = |names: &NameTable, what: &str, pred: &dyn Fn(Entry) -> bool| {
        for (name, _) in &ties {
            assert!(pred(row(names, name)), "{what}: {name:?}");
        }
    };
    let gathered = |doc: &ProfileDoc| {
        product_named(doc, &run(doc), Tol::witness())
            .expect("gathers")
            .1
    };

    for h in [SplitHalf::Above, SplitHalf::Below] {
        let (lone, _) = half(whole.clone(), split, h);
        every(
            &gathered(&lone),
            &format!("a lone {h:?} half gathers the separated name Unique"),
            &|e| matches!(e, Entry::Unique(_)),
        );
    }

    let (doc, above) = half(whole.clone(), split, SplitHalf::Above);
    let (doc, moved) = shifted(doc, above, 0.0);
    let (doc, below) = half(doc, split, SplitHalf::Below);
    assert_eq!(doc.roots(), &[moved, below][..], "the premise");
    every(
        &gathered(&doc),
        "a transformed half merges with the other",
        &|e| matches!(e, Entry::Tied(es) if es.len() == 2),
    );

    let (doc, above) = half(whole.clone(), split, SplitHalf::Above);
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(3.5), len(0.0)],
            normal: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, again) = insert(
        doc,
        Node::Split {
            target: above,
            tool: plane,
        },
    );
    let (resplit, below) = half(doc.clone(), split, SplitHalf::Below);
    assert_eq!(resplit.roots(), &[again, below][..], "the premise");
    every(
        &gathered(&resplit),
        "a re-split half merges with the other",
        &|e| matches!(e, Entry::Tied(es) if es.len() == 2),
    );

    // The second split's lower half holds the candidate; a `Part` of it
    // projects an already-marked row, and keeps the mark.
    let (doc, inner) = half(doc, again, SplitHalf::Below);
    let (doc, below) = half(doc, split, SplitHalf::Below);
    assert_eq!(doc.roots(), &[inner, below][..], "the premise");
    every(
        &gathered(&doc),
        "a half of a re-split half merges with the other",
        &|e| matches!(e, Entry::Tied(es) if es.len() == 2),
    );
}

/// One half of the subtract `sub` split by the plane through `origin`
/// with normal `normal`: a new split node and a `Part` over it.
fn half_of_a_split(
    doc: ProfileDoc,
    sub: RecipeNodeId,
    (origin, normal): ([f64; 3], [f64; 3]),
    h: SplitHalf,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: origin.map(len),
            normal: normal.map(scl),
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: sub,
            tool: plane,
        },
    );
    half(doc, split, h)
}

/// **Two roots carrying one entity still refuse.** Halves of two
/// DIFFERENT splits of the U-cutter's subtract overlap: both carry the
/// subtract's uncut entities in the overlap verbatim, and those rows
/// are not separated pieces — the mark must stay off them, so they go
/// in strict and collide. Each pair below refuses `Naming`; a
/// projection that marked every `Unique` row would tie them instead.
#[test]
fn halves_of_two_splits_that_overlap_still_refuse_naming() {
    let at_y = |y: f64| ([0.0, y, 0.0], [0.0, 1.0, 0.0]);
    let at_z = |z: f64| ([0.0, 0.0, z], [0.0, 0.0, 1.0]);
    let shapes = [
        (
            "above y=2, above y=2.2",
            (at_y(2.0), SplitHalf::Above),
            (at_y(2.2), SplitHalf::Above),
        ),
        (
            "above y=2, below y=3.5",
            (at_y(2.0), SplitHalf::Above),
            (at_y(3.5), SplitHalf::Below),
        ),
        (
            "above y=1.25, below z=2",
            (at_y(1.25), SplitHalf::Above),
            (at_z(2.0), SplitHalf::Below),
        ),
    ];
    // Every shape is judged before any assertion, so a regression
    // names each shape it reaches rather than the first.
    let mut wrong = Vec::new();
    for (what, (p1, h1), (p2, h2)) in shapes {
        let doc = ProfileDoc::empty_derived("gather-overlapping-halves", Tol::witness());
        let (doc, sub) = cutter(doc, &[(1.0, 1.5), (2.5, 3.0)]);
        let (doc, first) = half_of_a_split(doc, sub, p1, h1);
        let (doc, second) = half_of_a_split(doc, sub, p2, h2);
        assert_eq!(doc.roots(), &[first, second][..], "the premise: {what}");
        match product(&doc, &run(&doc), Tol::witness()) {
            Err(ProductError::Naming { .. }) => {}
            other => wrong.push(format!("{what}: {:?}", other.map(|b| b.faces().count()))),
        }
    }
    assert!(wrong.is_empty(), "expected a Naming refusal: {wrong:#?}");
}
