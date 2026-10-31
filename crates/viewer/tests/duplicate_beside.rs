//! **A duplicate is drawn beside the original as the person sees it**:
//! a second placement of the stepped copy, written by values at the
//! picked copy's evaluated motion, with no mate.
//!
//! Every row reads the DRAWN duplicate's extent against the box's
//! literal dimensions and the step the door authored: on a copy moved
//! off the identity, on a mated copy, on a turned copy (the step runs
//! along the copy's own `x`), and on a body the tree names with two
//! copies (one duplicate beside each).

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::drawn::{self, assert_drawn, corners, spacing_of};
use pncad::document::{Doc, Frame, Node, Operand, RecipeNodeId};
use pncad::geom_core::Tol;
use viewer::combine::DuplicateFault;
use viewer::session::{DocSession, Refusal, Selection, SessionOp};
use viewer::tools::{ToolKind, Tools};

/// The block: 40 × 20 × 10 mm, centred on the sketch origin and
/// extruded up.
const BLOCK: [f64; 3] = [0.04, 0.02, 0.01];
/// Half the block's plan.
const HALF: [f64; 2] = [BLOCK[0] / 2.0, BLOCK[1] / 2.0];
/// The small block a mate rests on the block's top: 20 × 10 × 6 mm.
const SMALL: [f64; 3] = [0.02, 0.01, 0.006];

/// The block's volume.
fn volume_of(size: [f64; 3]) -> f64 {
    size.iter().product()
}

/// A session holding the block, landed.
fn block(tol: Tol) -> (DocSession, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived("duplicate-start", tol), tol);
    let block = common::xy_box_in(&mut session, BLOCK);
    session.pump();
    (session, block)
}

/// **The duplicate tool fed `pick` and its commit performed**: the
/// stepped copy, and the placements the duplicate minted beside it.
fn duplicate_picking(
    session: &mut DocSession,
    pick: Selection,
    beside: Option<RecipeNodeId>,
) -> (RecipeNodeId, Vec<RecipeNodeId>) {
    let mut tools = Tools::new();
    tools.open(ToolKind::Duplicate);
    assert!(
        tools
            .feed(session.committed_doc(), None, &[SessionOp::Select(pick)])
            .is_empty(),
        "the pick is taken"
    );
    let tool = tools.duplicate().expect("open");
    assert_eq!(tool.beside(), beside, "the copy the pick landed on");
    duplicated(session, tool.op().expect("the seat is filled"))
}

/// `op` performed and landed: the stepped copy, and the placements
/// minted beside it.
fn duplicated(session: &mut DocSession, op: SessionOp) -> (RecipeNodeId, Vec<RecipeNodeId>) {
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let doc = session.committed_doc();
    let placements = outcome.minted[1..]
        .iter()
        .copied()
        .filter(|&n| matches!(doc.node(n), Some(Node::Place { .. })))
        .collect();
    (outcome.minted[0], placements)
}

/// **Whether `placement`'s bundle holds a mate**.
fn mated(session: &DocSession, placement: RecipeNodeId) -> bool {
    let Some(Node::Place { constraints, .. }) = session.committed_doc().node(placement) else {
        panic!("{placement:?} is a placement");
    };
    constraints.iter().any(|c| c.mate().is_some())
}

/// **A duplicate of a copy moved off the identity is drawn beside that
/// copy**: the block's copy pinned at x = 0.1 m is drawn over
/// x ∈ [0.08, 0.12], and its duplicate, picked there, one step further
/// along +x — not beside where the block stands in its own space.
#[test]
fn a_duplicate_of_a_moved_copy_is_drawn_beside_it() {
    let tol = Tol::witness();
    let (session, block) = block(tol);
    let (mut session, copy) =
        drawn::moved_copy(&session, block, &drawn::translated([0.1, 0.0, 0.0]), tol);
    let picked = common::asm::pick_face(&session, &common::down_at(0.1, 0.0));
    assert_eq!(picked.node, copy, "the ray met the moved copy");
    let (stepped, placements) =
        duplicate_picking(&mut session, Selection::Face(picked), Some(copy));
    let [placement] = placements[..] else {
        panic!("one duplicate placed: {placements:?}");
    };
    assert!(!mated(&session, placement), "the duplicate holds no mate");
    let step = spacing_of(&session, stepped);
    let [x, y] = HALF;
    assert_drawn(
        &mut session,
        copy,
        [0.1 - x, -y, 0.0],
        [0.1 + x, y, BLOCK[2]],
        volume_of(BLOCK),
        "the original, where it stood",
    );
    assert_drawn(
        &mut session,
        placement,
        [0.1 - x + step, -y, 0.0],
        [0.1 + x + step, y, BLOCK[2]],
        volume_of(BLOCK),
        "the duplicate, one step along +x of the copy",
    );
    assert!(0.1 - x + step > 0.1 + x, "clear of the original");
}

/// **A duplicate of a mated copy is drawn beside it, and is not
/// mated**: a small block standing 0.1 m along x is rested on the
/// block's top by the mate tool; its duplicate stands one step along
/// +x on the same plane, written by values alone.
#[test]
fn a_duplicate_of_a_mated_copy_is_drawn_beside_it_unmated() {
    let tol = Tol::witness();
    let (mut session, base) = block(tol);
    let small = common::xy_box_in(&mut session, SMALL);
    session.pump();
    let (mut session, small_copy) =
        drawn::moved_copy(&session, small, &drawn::translated([0.1, 0.0, 0.0]), tol);
    let index = common::plate_index(&session);
    let underside = common::displayed_face_at(&session, &index, &common::up_at(0.1, 0.0));
    let top = common::displayed_face_at(&session, &index, &common::down_at(0.0, 0.0));
    assert_eq!(underside.node, small_copy, "the small block's underside");
    assert_eq!(
        top.node,
        common::copy_of(session.committed_doc(), base),
        "the block's top"
    );
    drawn::rest_on(&mut session, underside, top);
    assert!(mated(&session, small_copy), "the copy is mated");
    let picked = common::asm::pick_face(&session, &common::down_at(0.1, 0.0));
    assert_eq!(picked.node, small_copy, "the ray met the seated copy");
    let (stepped, placements) =
        duplicate_picking(&mut session, Selection::Face(picked), Some(small_copy));
    let [placement] = placements[..] else {
        panic!("one duplicate placed: {placements:?}");
    };
    assert!(!mated(&session, placement), "the duplicate holds no mate");
    let step = spacing_of(&session, stepped);
    let (x, y, z) = (SMALL[0] / 2.0, SMALL[1] / 2.0, BLOCK[2]);
    assert_drawn(
        &mut session,
        placement,
        [0.1 - x + step, -y, z],
        [0.1 + x + step, y, z + SMALL[2]],
        volume_of(SMALL),
        "the duplicate, seated where the mate seats the original, one step along",
    );
}

/// **A turned copy's duplicate steps along the copy's own `x`**: the
/// copy turned a quarter turn about z stands with its 40 mm side along
/// world y, and its duplicate lands one step along world +y beside it.
/// The step is the body's width along its OWN `x`, so only that
/// direction is a clearance: along world +x it would be the 20 mm side,
/// and a step measured off 40 mm would leave a different gap per turn.
#[test]
fn a_turned_copys_duplicate_steps_along_the_copys_own_x() {
    let tol = Tol::witness();
    let (session, block) = block(tol);
    let turned = Frame {
        columns: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.1, 0.05, 0.0],
    };
    let (mut session, copy) = drawn::moved_copy(&session, block, &turned, tol);
    let (stepped, placements) = duplicate_picking(&mut session, Selection::Node(copy), Some(copy));
    let [placement] = placements[..] else {
        panic!("one duplicate placed: {placements:?}");
    };
    let step = spacing_of(&session, stepped);
    let [x, y] = HALF;
    assert_drawn(
        &mut session,
        copy,
        [0.1 - y, 0.05 - x, 0.0],
        [0.1 + y, 0.05 + x, BLOCK[2]],
        volume_of(BLOCK),
        "the turned original",
    );
    assert_drawn(
        &mut session,
        placement,
        [0.1 - y, 0.05 - x + step, 0.0],
        [0.1 + y, 0.05 + x + step, BLOCK[2]],
        volume_of(BLOCK),
        "the duplicate, one step along the copy's own x, which is world +y",
    );
}

/// **A copy at a general pose is duplicated at that pose**: the values
/// written stand the duplicate at the copy's motion to its rounding.
/// The copy is turned about an oblique axis by an angle no quarter turn
/// spells, so all three turns of the written chain are read off it, and
/// every corner of the duplicate is the block's own corner stepped
/// along its own `x` and carried by the copy's pose.
#[test]
fn a_copy_at_a_general_pose_is_duplicated_at_that_pose() {
    let tol = Tol::witness();
    let (session, block) = block(tol);
    // A turn of 0.7 rad about the unit axis (1, 2, 2)/3, by Rodrigues.
    let (k, angle) = ([1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0], 0.7_f64);
    let (s, c) = angle.sin_cos();
    let rot = |i: usize, j: usize| {
        let cross = [[0.0, -k[2], k[1]], [k[2], 0.0, -k[0]], [-k[1], k[0], 0.0]];
        let id = if i == j { 1.0 } else { 0.0 };
        id * c + s * cross[i][j] + (1.0 - c) * k[i] * k[j]
    };
    let pose = Frame {
        columns: [0, 1, 2].map(|j| [0, 1, 2].map(|i| rot(i, j))),
        translation: [0.1, -0.03, 0.02],
    };
    let carried = |p: [f64; 3]| {
        [0, 1, 2].map(|i| pose.translation[i] + (0..3).map(|j| rot(i, j) * p[j]).sum::<f64>())
    };
    let (mut session, copy) = drawn::moved_copy(&session, block, &pose, tol);
    let (stepped, placements) = duplicate_picking(&mut session, Selection::Node(copy), Some(copy));
    let [placement] = placements[..] else {
        panic!("one duplicate placed: {placements:?}");
    };
    let step = spacing_of(&session, stepped);
    let own = corners(&mut session, block);
    let sorted = |mut points: Vec<[f64; 3]>| {
        points.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
        points
    };
    let near = |got: &[[f64; 3]], want: &[[f64; 3]]| {
        got.len() == want.len()
            && got
                .iter()
                .zip(want)
                .all(|(g, w)| g.iter().zip(w).all(|(a, b)| (a - b).abs() <= 1e-9))
    };
    let original = sorted(own.iter().map(|&p| carried(p)).collect());
    let got = corners(&mut session, copy);
    assert!(near(&got, &original), "the original at the pose: {got:?}");
    let want = sorted(
        own.iter()
            .map(|&[x, y, z]| carried([x + step, y, z]))
            .collect(),
    );
    let got = corners(&mut session, placement);
    assert!(
        near(&got, &want),
        "the duplicate at the pose, stepped along its own x: {got:?} vs {want:?}"
    );
}

/// **A body named in the tree with two drawn copies is duplicated
/// beside each**: no copy is the one picked, so the duplicate is drawn
/// wherever the original is, one step along each copy's own `x`.
#[test]
fn a_body_named_in_the_tree_is_duplicated_beside_each_drawn_copy() {
    let tol = Tol::witness();
    let (session, block) = block(tol);
    let (session, first) =
        drawn::moved_copy(&session, block, &drawn::translated([0.1, 0.0, 0.0]), tol);
    let var = session
        .committed_doc()
        .output(block, 0)
        .expect("the block defines its body");
    let (doc, second) = crate::fixture::insert(
        session.committed_doc().clone(),
        Node::place(
            pncad::document::BodyRead::plain(Operand::Var(var)),
            Operand::Var(pncad::document::VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
    );
    let doc = drawn::pinned(doc, second, &drawn::translated([0.0, 0.1, 0.0]));
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let (stepped, placements) = duplicate_picking(&mut session, Selection::Node(block), None);
    let [beside_first, beside_second] = placements[..] else {
        panic!("one duplicate placed beside each copy: {placements:?}");
    };
    let step = spacing_of(&session, stepped);
    let [x, y] = HALF;
    for (placement, [cx, cy], copy) in [
        (beside_first, [0.1, 0.0], first),
        (beside_second, [0.0, 0.1], second),
    ] {
        assert_drawn(
            &mut session,
            placement,
            [cx - x + step, cy - y, 0.0],
            [cx + x + step, cy + y, BLOCK[2]],
            volume_of(BLOCK),
            &format!("the duplicate beside {copy:?}"),
        );
    }
}

/// **A duplicate named beside a copy of another body is refused,
/// typed**, and writes nothing.
#[test]
fn a_duplicate_beside_another_bodys_copy_is_refused() {
    let tol = Tol::witness();
    let (mut session, block) = block(tol);
    let other = common::xy_box_in(&mut session, SMALL);
    session.pump();
    let elsewhere = common::copy_of(session.committed_doc(), other);
    let before = session.committed_doc().clone();
    let outcome = session.perform(SessionOp::Duplicate {
        input: block.into(),
        beside: Some(elsewhere),
    });
    assert!(
        matches!(
            outcome.refusal,
            Some(Refusal::Duplicate(DuplicateFault::NotACopy { .. }))
        ),
        "{:?}",
        outcome.refusal
    );
    assert_eq!(*session.committed_doc(), before, "nothing was written");
}
