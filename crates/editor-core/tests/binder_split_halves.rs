//! **A split of each member yields each member's halves** (D10
//! `[repetition]`): `{k in xs: split(xs[k])}` is a family of splits, and
//! a reader of a half its binder yields reads that half of every member,
//! a family of bodies, as a reader of one split's half reads that half.

#![allow(clippy::expect_used, clippy::panic)]

use crate::blocks::block;
use crate::fixture::{self, insert};
use editor_core::{
    Bodies, BodyRead, Node, NodeResult, Operand, ProfileDoc, RecipeNodeId, Spell, SplitHalf,
    TargetAxis, VarId,
};
use geom_core::Tol;

/// A unit box, a row of three of it two apart along x, and each member
/// split a quarter up, the binder yielding both halves. The document and
/// the halves' variables, above then below.
fn split_each() -> (ProfileDoc, [VarId; 2]) {
    let (doc, halves, _) = split_each_along(TargetAxis::X);
    (doc, halves)
}

/// [`split_each`] with the row along `axis`, and the row's index.
fn split_each_along(axis: TargetAxis) -> (ProfileDoc, [VarId; 2], VarId) {
    let doc = ProfileDoc::empty_derived("binder-split-halves", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, k, row) = fixture::row(doc, the_box, ("k", 3), axis, 2.0);
    let tool = fixture::plane_on(&doc, the_box, [0.0, 0.0, 0.25], [0.0, 0.0, 1.0]);
    let target = fixture::member_here(&doc, row.into());
    let (doc, split) = insert(doc, Node::Split { target, tool });
    let doc = fixture::yielded(doc, split);
    let half = |half: SplitHalf| {
        doc.output(split, half.port())
            .expect("a split defines a port per half")
    };
    let halves = [half(SplitHalf::Above), half(SplitHalf::Below)];
    (doc, halves, k)
}

/// The union of every member of the yield `half`, `[…half]`.
fn union_of(doc: ProfileDoc, half: VarId) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![Spell::All(BodyRead::plain(Operand::Var(half)))]),
        },
    )
}

fn volume(b: &topo::Body<f64>) -> f64 {
    let measured = topo::test_support::finished("the measured body", b.clone(), Tol::witness());
    topo::test_support::pinned_volume("the measured body", &measured, Tol::witness()).volume
}

/// **A yielded half is that half of every member**: a union of the upper
/// halves is three bodies of three quarters each, and of the lower
/// halves three of a quarter each.
///
/// *Breaks if* a reader of a split-of-each-member's half refuses
/// `HalfOfAFamily` (the half read off the family of splits whole), or
/// reads one member's half for the family.
#[test]
fn a_union_of_a_yielded_half_is_that_half_of_every_member() {
    let (doc, [above, below]) = split_each();
    for (half, each) in [(above, 0.75), (below, 0.25)] {
        let (doc, union) = union_of(doc.clone(), half);
        let ev = fixture::run(&doc, &Default::default());
        let body = match ev.result(union) {
            Some(NodeResult::Ok(v)) => v.payload.output_body(0).expect("one body").body().clone(),
            other => panic!("the union of the yielded half evaluates: {other:?}"),
        };
        let got = volume(&body);
        assert!(
            (got - 3.0 * each).abs() < 1e-9,
            "three halves of {each} each, got {got}"
        );
    }
}

/// **One member's half, read by index**: `above[1]` is the second
/// member's upper half, three quarters of a box, as a union of it alone
/// reads it.
///
/// *Breaks if* an indexed read of a yielded half reads the family of
/// splits whole and refuses, or reads another member.
#[test]
fn one_members_yielded_half_reads_by_index() {
    let (doc, [above, _]) = split_each();
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: Bodies::spelled(vec![BodyRead::indexed(
                Operand::Var(above),
                vec![editor_core::Formula::count(1)],
            )]),
        },
    );
    let ev = fixture::run(&doc, &Default::default());
    let framed = match ev.result(union) {
        Some(NodeResult::Ok(v)) => v.payload.output_body(0).expect("one body").clone(),
        other => panic!("the member's half evaluates: {other:?}"),
    };
    let got = volume(framed.body());
    assert!(
        (got - 0.75).abs() < 1e-9,
        "three quarters of a box, got {got}"
    );
    let lo = fixture::in_space(&framed)
        .points()
        .map(|(_, p)| p.to_array()[0])
        .fold(f64::INFINITY, f64::min);
    assert!(
        (lo - 2.0).abs() < 1e-9,
        "the second member, from x = 2: {lo}"
    );
}

/// A row of three of a unit box placed in the world, two apart along
/// `axis`, each member split by the world plane a quarter up, the binder
/// yielding both halves and the product set to `half`. The document, its
/// evaluation and the row's index.
fn world_product(
    axis: TargetAxis,
    half: SplitHalf,
) -> (ProfileDoc, editor_core::Evaluation<f64>, VarId) {
    let doc = ProfileDoc::empty_derived("binder-split-halves-product", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, placed) = fixture::in_world(
        doc,
        the_box,
        &editor_core::Frame::translation([0.0, 0.0, 0.0]),
    );
    let (doc, k, row) = fixture::row(doc, placed, ("k", 3), axis, 2.0);
    let tool = fixture::plane_on(&doc, placed, [0.0, 0.0, 0.25], [0.0, 0.0, 1.0]);
    let target = fixture::member_here(&doc, row.into());
    let (doc, split) = insert(doc, Node::Split { target, tool });
    let doc = fixture::yielded(doc, split);
    let read = doc
        .output(split, half.port())
        .expect("a split defines a port per half");
    let doc = doc
        .apply(
            &editor_core::DocEdit::SetProduct {
                product: Some(Operand::Var(read)),
            },
            Tol::witness(),
        )
        .expect("a yielded half is a family of bodies in the world")
        .doc;
    let ev = fixture::run(&doc, &Default::default());
    (doc, ev, k)
}

/// **The product of a yielded half is that half of every member**, as a
/// reader of it reads it: a world product set to the upper halves is
/// three bodies of three quarters each, each named at its member's
/// place.
///
/// *Breaks if* the gather reads the family of splits as no value and
/// refuses, reads one member's half for the family, or names the halves
/// other than per member.
#[test]
fn the_product_of_a_yielded_half_is_that_half_of_every_member() {
    let (doc, ev, _) = world_product(TargetAxis::X, SplitHalf::Above);
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness())
        .unwrap_or_else(|e| panic!("the product of the yielded half gathers: {e}"));
    let body = product.body.clone().into_body();
    assert_eq!(body.solids().count(), 3, "one half-body per member");
    let got = volume(&body);
    assert!(
        (got - 3.0 * 0.75).abs() < 1e-9,
        "three halves of three quarters each, got {got}"
    );
    let mut places: Vec<String> = product
        .names
        .iter()
        .filter_map(|(name, _)| {
            let said = format!("{name:?}");
            let at = said.find("Member")?;
            Some(
                said[at..]
                    .split(" of")
                    .next()
                    .unwrap_or_default()
                    .to_owned(),
            )
        })
        .collect();
    places.sort();
    places.dedup();
    assert_eq!(
        places.len(),
        3,
        "every name is a member's, one place per member: {places:?}"
    );
}

/// **A yielded half one member of which is empty refuses the product,
/// naming that member**, in the words the evaluator refuses a reader of
/// it with: a row stacked along z above the plane a quarter up leaves the
/// lower half of members 1 and 2 empty, and the read refuses at the
/// first, `k = 1`.
///
/// *Breaks if* the gather skips an empty member's half (a second rule
/// beside the evaluator's), or refuses without naming the member.
#[test]
fn a_yielded_half_with_an_empty_member_refuses_the_product_naming_it() {
    let (doc, ev, k) = world_product(TargetAxis::Z, SplitHalf::Below);
    let refused = editor_core::product_recorded(&doc, &ev, Tol::witness())
        .map(|_| ())
        .expect_err("an empty member's half refuses the product");
    let editor_core::ProductError::ReadRefused { kind, .. } = &refused else {
        panic!("the read refuses: {refused:?}");
    };
    match kind.as_ref() {
        editor_core::NodeErrorKind::MemberRefused { at, source } => {
            assert_eq!(at, &vec![(k, 1)], "the first member with no lower half");
            assert!(
                matches!(
                    source.as_ref(),
                    editor_core::NodeErrorKind::EmptyHalf {
                        half: SplitHalf::Below,
                        ..
                    }
                ),
                "its half is empty: {source:?}"
            );
        }
        other => panic!("the member's refusal, named: {other:?}"),
    }
    assert_eq!(
        refused.kind(),
        editor_core::ProductErrorKind::ReadRefused,
        "its class"
    );
}

/// **A union of a yielded half one member of which is empty refuses,
/// naming that member**: a row stacked along z above the plane a quarter
/// up leaves the lower half of members 1 and 2 empty, and the union over
/// the lower halves refuses at the first, `k = 1`, its half empty.
///
/// *Breaks if* a reader of a yielded half skips an empty member's half,
/// or refuses without naming the member.
#[test]
fn a_union_of_a_yielded_half_with_an_empty_member_refuses_naming_it() {
    let (doc, [_, below], k) = split_each_along(TargetAxis::Z);
    let (doc, union) = union_of(doc, below);
    let ev = fixture::run(&doc, &Default::default());
    let kind = match ev.result(union) {
        Some(NodeResult::Failed(error)) => &error.kind,
        other => panic!("the union refuses: {other:?}"),
    };
    match kind {
        editor_core::NodeErrorKind::MemberRefused { at, source } => {
            assert_eq!(at, &vec![(k, 1)], "the first member with no lower half");
            assert!(
                matches!(
                    source.as_ref(),
                    editor_core::NodeErrorKind::EmptyHalf {
                        half: SplitHalf::Below,
                        ..
                    }
                ),
                "its half is empty: {source:?}"
            );
        }
        other => panic!("the member's half, named: {other:?}"),
    }
}

/// **One node's reads of a yielded half's members gather the family of
/// halves once**: a union of `[above[0], above[1], above[2]]` reads three
/// members of one family of halves, which its pass gathers once, not once
/// per read.
///
/// *Breaks if* the pass's gather of a family of halves is not kept across
/// its reads (each indexed read re-gathering every member's half).
#[cfg(debug_assertions)]
#[test]
fn one_nodes_reads_of_a_yielded_half_gather_it_once() {
    let (doc, [above, _]) = split_each();
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: Bodies::spelled(
                (0..3)
                    .map(|i| {
                        BodyRead::indexed(Operand::Var(above), vec![editor_core::Formula::count(i)])
                    })
                    .collect(),
            ),
        },
    );
    let before = editor_core::halves_gathered_on_this_thread();
    let ev = fixture::run(
        &doc,
        &editor_core::EvalOptions {
            parallel: false,
            ..Default::default()
        },
    );
    let gathered = editor_core::halves_gathered_on_this_thread() - before;
    assert!(
        matches!(ev.result(union), Some(NodeResult::Ok(_))),
        "the union of the three members evaluates: {:?}",
        ev.result(union)
    );
    assert_eq!(gathered, 1, "one family of halves, gathered once");
}
