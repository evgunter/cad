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
    let doc = ProfileDoc::empty_derived("binder-split-halves", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _k, row) = fixture::row(doc, the_box, ("k", 3), TargetAxis::X, 2.0);
    let tool = fixture::plane_on(&doc, the_box, [0.0, 0.0, 0.25], [0.0, 0.0, 1.0]);
    let target = fixture::member_here(&doc, row.into());
    let (doc, split) = insert(doc, Node::Split { target, tool });
    let doc = fixture::yielded(doc, split);
    let half = |half: SplitHalf| {
        doc.output(split, half.port())
            .expect("a split defines a port per half")
    };
    let halves = [half(SplitHalf::Above), half(SplitHalf::Below)];
    (doc, halves)
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
