//! **FORK-DM4 unit 1: a boolean keys its names by the read** — the
//! rows REFERENCES DM4 and DM5 pin on the three nodes.
//!
//! - A repeated read glues: `Union([A, A])` and `Intersect([A, A])`
//!   build `A`, every row named as `A`'s through the one read;
//!   `Subtract { from: A, tool: A }` is the typed empty body.
//! - A union or intersect of one body builds that body, with no boolean
//!   run; of none, the typed empty body.
//! - Two members read out of one operation (a split's two halves) are
//!   two reads, so they are named apart and their union builds.
//! - A three-member intersect over members two of which share a face
//!   plane folds through the intersect lane.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    Bodies, BooleanValue, CancelToken, EntityKind, EvalOptions, Evaluation, ExtrudeSide, Node,
    Operand, ProfileDoc, RecipeNodeId, RoleSeg, StableName, ValuePayload, VarId, evaluate,
};
use fixture::{insert, len, on_frame, scl, table};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A box `[x0, x1] × [y0, y1] × [0, h]`.
fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    h: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(h),
            side: ExtrudeSide::Along,
        },
    )
}

fn unit_block(name: &str) -> (ProfileDoc, RecipeNodeId) {
    block(
        ProfileDoc::empty_derived(name, Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        1.0,
    )
}

fn volume(ev: &Evaluation<f64>, id: RecipeNodeId) -> f64 {
    topo::mass_properties(crate::corpus::body_of(ev, id), Tol::witness())
        .expect("mass properties")
        .volume
}

fn is_empty(ev: &Evaluation<f64>, id: RecipeNodeId) -> bool {
    matches!(
        ev.value(id).map(|v| &v.payload),
        Some(ValuePayload::Boolean(BooleanValue::Empty))
    )
}

fn spelled(reads: &[RecipeNodeId]) -> Bodies<Operand> {
    Bodies::Spelled(reads.iter().map(|&r| r.into()).collect())
}

/// Every name in `at`'s table, sorted.
fn names(ev: &Evaluation<f64>, at: RecipeNodeId) -> Vec<StableName> {
    let mut n: Vec<StableName> = table(ev, at).iter().map(|(n, _)| n.clone()).collect();
    n.sort();
    n
}

/// `at`'s names are exactly `a`'s, each carried as `From { read, of }`,
/// and the body named by `at` itself.
fn assert_named_as_a_through(ev: &Evaluation<f64>, at: RecipeNodeId, a: RecipeNodeId, read: VarId) {
    let mut expected: Vec<StableName> = names(ev, a)
        .into_iter()
        .filter(|of| of.kind != EntityKind::Body)
        .map(|of| fixture::member_entity(at, read, of.clone(), of.kind))
        .collect();
    expected.push(StableName {
        kind: EntityKind::Body,
        node: at,
        path: vec![RoleSeg::OutputBody],
    });
    expected.sort();
    assert_eq!(
        names(ev, at),
        expected,
        "every row is A's, through its read"
    );
}

#[test]
fn a_union_of_a_read_with_itself_is_that_body_named_as_its() {
    let (doc, a) = unit_block("dm4-self-union");
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: spelled(&[a, a]),
            declare: Vec::new(),
        },
    );
    let ev = run(&doc);
    assert!(
        ev.value(u).is_some(),
        "A ∪ A builds: {:?}",
        ev.node_error(u)
    );
    assert_eq!(volume(&ev, u), 1.0);
    assert_named_as_a_through(&ev, u, a, fixture::out(&doc, a));
}

#[test]
fn an_intersect_of_a_read_with_itself_is_that_body_named_as_its() {
    let (doc, a) = unit_block("dm4-self-intersect");
    let (doc, i) = insert(
        doc,
        Node::Intersect {
            members: spelled(&[a, a]),
            declare: Vec::new(),
        },
    );
    let ev = run(&doc);
    assert!(
        ev.value(i).is_some(),
        "A ∩ A builds: {:?}",
        ev.node_error(i)
    );
    assert_eq!(volume(&ev, i), 1.0);
    assert_named_as_a_through(&ev, i, a, fixture::out(&doc, a));
}

#[test]
fn a_subtract_of_a_read_from_itself_is_the_typed_empty_body() {
    let (doc, a) = unit_block("dm4-self-subtract");
    let (doc, s) = insert(
        doc,
        Node::Subtract {
            from: a.into(),
            tool: a.into(),
            declare: Vec::new(),
        },
    );
    let ev = run(&doc);
    assert!(is_empty(&ev, s), "A − A is empty: {:?}", ev.node_error(s));
}

#[test]
fn a_union_or_intersect_of_one_body_is_that_body() {
    let (doc, a) = unit_block("dm4-one-member");
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: spelled(&[a]),
            declare: Vec::new(),
        },
    );
    let (doc, i) = insert(
        doc,
        Node::Intersect {
            members: spelled(&[a]),
            declare: Vec::new(),
        },
    );
    let ev = run(&doc);
    let read = fixture::out(&doc, a);
    for at in [u, i] {
        assert_eq!(volume(&ev, at), 1.0, "{at}: {:?}", ev.node_error(at));
        assert_named_as_a_through(&ev, at, a, read);
    }
}

#[test]
fn a_union_or_intersect_of_no_body_is_the_typed_empty_body() {
    let doc = ProfileDoc::empty_derived("dm4-no-member", Tol::witness());
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(Vec::new()),
            declare: Vec::new(),
        },
    );
    let (doc, i) = insert(
        doc,
        Node::Intersect {
            members: Bodies::Spelled(Vec::new()),
            declare: Vec::new(),
        },
    );
    let ev = run(&doc);
    for at in [u, i] {
        assert!(is_empty(&ev, at), "{at}: {:?}", ev.node_error(at));
        assert!(table(&ev, at).iter().next().is_none(), "no rows");
    }
}

/// **A split's two halves are two reads** (the finding the item was
/// opened on): their union is named member by member and builds the
/// block again, a declared rest contact across the section.
#[test]
fn the_union_of_a_splits_two_halves_builds_the_block() {
    let (doc, target) = unit_block("dm4-halves");
    let (doc, plane) = insert(
        doc,
        Node::Datum(editor_core::Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.5)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: target.into(),
            tool: plane.into(),
        },
    );
    let (above, below) = (
        doc.output(split, 0).expect("the upper half"),
        doc.output(split, 1).expect("the lower half"),
    );
    let section = |side: editor_core::SplitHalf| StableName {
        kind: EntityKind::Face,
        node: split,
        path: vec![RoleSeg::SectionFace { side, section: 0 }],
    };
    let declare = editor_core::declare_rest(vec![(
        editor_core::SitedRef::new(above, section(editor_core::SplitHalf::Above)),
        editor_core::SitedRef::new(below, section(editor_core::SplitHalf::Below)),
    )]);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![Operand::output(split, 0), Operand::output(split, 1)]),
            declare,
        },
    );
    let ev = run(&doc);
    assert!(
        ev.value(u).is_some(),
        "the halves rejoin: {:?}",
        ev.node_error(u)
    );
    assert_eq!(volume(&ev, u), 1.0);
    let reads: std::collections::BTreeSet<VarId> = table(&ev, u)
        .iter()
        .filter_map(|(n, _)| match n.path.first() {
            Some(RoleSeg::From { read, .. }) => Some(*read),
            _ => None,
        })
        .collect();
    assert_eq!(
        reads,
        [above, below].into_iter().collect(),
        "each half's entities are keyed by its own read"
    );
}

/// **The review check the item names**: three members, two of which
/// share a face plane, fold through the intersect lane. `[0,2]×[0,1]×[0,1]`
/// ∩ `[1,3]×[0,1]×[0,1]` ∩ `[0,3]×[0,1]×[0,1/2]` is `[1,2]×[0,1]×[0,1/2]`;
/// every member's floor lies on z = 0 and the first two share y = 0,
/// y = 1 and z = 1, all declared as continuations of one another.
#[test]
fn a_three_member_intersect_with_coincident_faces_folds() {
    let doc = ProfileDoc::empty_derived("dm4-three-intersect", Tol::witness());
    let (doc, a) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let (doc, b) = block(doc, (1.0, 3.0), (0.0, 1.0), 1.0);
    let (doc, c) = block(doc, (0.0, 3.0), (0.0, 1.0), 0.5);
    let ev = run(&doc);
    let pair = |p, q| {
        editor_core::find_flush_candidates(&ev, &doc, p, q, Tol::witness())
            .expect("definite findings")
    };
    let mut declare = Vec::new();
    for (p, q) in [(a, b), (a, c), (b, c)] {
        declare.extend(editor_core::declared_pairs(&pair(p, q)));
    }
    let (doc, i) = insert(
        doc,
        Node::Intersect {
            members: spelled(&[a, b, c]),
            declare,
        },
    );
    let ev = run(&doc);
    assert!(
        ev.value(i).is_some(),
        "the intersect folds: {:?}",
        ev.node_error(i)
    );
    assert_eq!(volume(&ev, i), 0.5);
}
