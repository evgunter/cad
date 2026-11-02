//! Review probes for PR 6300 (the placement editor), lane
//! `placing-dual-6300-r1`. Each row states the claim it tries to
//! falsify; a red row is a finding.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::asm;
use pncad::document::{
    Coord, Dimension, Frame, MateRef, Node, Operand, RecipeNodeId, SlotId, TargetAxis, VarKind,
};
use pncad::geom_core::Tol;
use pncad::geom_core::linalg::{Affine3, Mat3, Vec3};
use viewer::matetool::{MateChoice, MateTool};
use viewer::session::{BundleVerdict, DocSession, SessionOp};

fn authored(
    session: &DocSession,
    placement: RecipeNodeId,
) -> Vec<pncad::document::Constraint<pncad::document::Formula>> {
    let doc = session.committed_doc();
    match doc.node(placement).map(|node| node.authored(doc)) {
        Some(Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. }) => {
            constraints
        }
        other => panic!("{placement:?} is a placement: {other:?}"),
    }
}

fn pose(session: &DocSession, placement: RecipeNodeId, tol: Tol) -> Frame {
    let (doc, _) = session.landed_pair().expect("landed");
    common::solve(session, doc, tol)
        .placement(doc, placement)
        .expect("a pose")
}

fn close(a: &Frame, b: &Frame) -> bool {
    a.columns
        .iter()
        .flatten()
        .chain(&a.translation)
        .zip(b.columns.iter().flatten().chain(&b.translation))
        .all(|(x, y)| (x - y).abs() < 1e-9)
}

fn choice(kind: VarKind, flip: bool) -> MateChoice {
    MateChoice {
        kind,
        flip,
        values: Vec::new(),
    }
}

/// Claim 6: the verdict before Apply equals what Apply does. A point
/// mate over two flat faces (SideUnevaluated, PR 6284's MINOR-2) is a
/// verdict the panel says in the refusal voice; Apply must then not land.
#[test]
fn r1_a_faulting_verdict_is_an_apply_that_does_not_land() {
    let tol = Tol::witness();
    let bench = asm::bench("r1-fault", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (post_top, shelf_bottom) = asm::seat_picks(&session, &bench);
    let mut tool = MateTool::new();
    tool.pick(session.doc(), post_top);
    tool.pick(session.doc(), shelf_bottom);
    let proposal = common::proposed(&session, &tool, choice(VarKind::Point, false))
        .expect("two faces propose as points");
    let verdict = session.bundle_verdict(&proposal.op());
    let says_stands = matches!(verdict, BundleVerdict::Stands(_));
    let states = session.history().len();
    let outcome = session.perform(proposal.op());
    let landed = outcome.refusal.is_none() && session.history().len() == states + 1;
    std::fs::remove_dir_all(&bench.dir).expect("removable");
    assert!(
        says_stands == landed,
        "verdict {verdict:?} but Apply landed={landed} ({:?})",
        outcome.refusal
    );
}

/// Claim 9: removing a mate keeps the copy where it stands. post_b's +x
/// side face rests on the shelf's underside, so its x axis stands on the
/// target's z (the chain's gimbal pole), spun 30 degrees about it; the
/// rest removed, the freed chain is read back off that pose.
#[test]
fn r1_removing_a_rest_that_stands_x_on_z_keeps_the_copy_where_it_stands() {
    let tol = Tol::witness();
    let bench = asm::bench("r1-gimbal", tol);
    let mut session = asm::open_bench(&bench, tol);
    let side = asm::pick_face(
        &session,
        &common::along_x(-1.0, asm::POST_SECTION / 2.0, asm::POST_HEIGHT / 2.0),
    );
    assert_eq!(side.node, bench.post_b, "the +x side face is post_b's");
    let shelf_bottom = asm::shelf_underside(&session);
    let mut tool = MateTool::new();
    tool.pick(session.doc(), side);
    tool.pick(session.doc(), shelf_bottom);
    let proposal =
        common::proposed(&session, &tool, choice(VarKind::Plane, true)).expect("the rest proposes");
    let mate = common::commit_mate(&mut session, proposal.op());
    let spin = authored(&session, bench.post_b)
        .iter()
        .find_map(|c| c.value().filter(|v| v.coord == Coord::SPIN).map(|_| c.id))
        .expect("a spin");
    let outcome = session.perform(SessionOp::SetSlot {
        node: bench.post_b,
        slot: SlotId::ValueSpin { constraint: spin },
        value: viewer::props::SlotValue::of(Dimension::Angle, 30.0_f64.to_radians())
            .expect("a value"),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let before = pose(&session, bench.post_b, tol);
    assert!(
        (before.columns[0][2].abs() - 1.0).abs() < 1e-9,
        "the premise: post_b's x stands on z: {before:?}"
    );
    let outcome = session.perform(SessionOp::RemoveConstraint { mate });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let after = pose(&session, bench.post_b, tol);
    let bundle = authored(&session, bench.post_b);
    std::fs::remove_dir_all(&bench.dir).expect("removable");
    assert!(
        close(&after, &before),
        "post_b jumped: {after:?} was {before:?}; bundle {bundle:?}"
    );
}

/// Claim 9 / the kernel door: `read_back` with no chart reads the chain
/// x, y, z off a rotation. At the chain's pole (the copy's x on the
/// target's z) the chain still has an answer (any split of the turn
/// between x and z); the one read must turn as the rotation does.
#[test]
fn r1_read_back_at_the_chains_pole_turns_as_the_rotation_does() {
    let (s, c) = 30.0_f64.to_radians().sin_cos();
    // Rz(30°)·Ry(-90°), exactly: x → z, y → (−s, c, 0), z → (−c, −s, 0).
    let r = Mat3::from_cols(
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(-s, c, 0.0),
        Vec3::new(-c, -s, 0.0),
    );
    let at = Affine3::from_parts(r, Vec3::zero());
    let coords = [
        Coord::Spin(Some(TargetAxis::X)),
        Coord::Spin(Some(TargetAxis::Y)),
        Coord::Spin(Some(TargetAxis::Z)),
    ];
    let band = pncad::geom_core::Band::linear(Tol::witness()).expect("a band");
    let read = editor_core::mate::read_back(None, &at, &coords, band);
    let [Some(x), Some(y), Some(z)] = read[..] else {
        panic!("the chain reads: {read:?}")
    };
    let rx = |a: f64| {
        let (s, c) = a.sin_cos();
        Mat3::from_cols(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, c, s),
            Vec3::new(0.0, -s, c),
        )
    };
    let ry = |a: f64| {
        let (s, c) = a.sin_cos();
        Mat3::from_cols(
            Vec3::new(c, 0.0, -s),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(s, 0.0, c),
        )
    };
    let rz = |a: f64| {
        let (s, c) = a.sin_cos();
        Mat3::from_cols(
            Vec3::new(c, s, 0.0),
            Vec3::new(-s, c, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        )
    };
    let back = rz(z) * ry(y) * rx(x);
    let e = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
    let off = e
        .iter()
        .map(|&v| (back * v - r * v).norm())
        .fold(0.0, f64::max);
    assert!(
        off < 1e-9,
        "the chain read ({x}, {y}, {z}) turns {off} away from the rotation"
    );
}

/// Claim 1, Create door: one undo takes the document back exactly.
#[test]
fn r1_the_create_door_undoes_to_the_exact_prior_document() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(
        pncad::document::Doc::empty_derived("r1-create-undo", tol),
        tol,
    );
    let body = common::xy_box_in(&mut session, [0.02, 0.03, 0.01]);
    session.pump();
    let before = session.committed_doc().clone();
    let tool = MateTool::create(Operand::Node(body));
    let (doc, ev) = session.landed_pair().expect("landed");
    let proposal = tool.proposal(doc, ev, tol).expect("proposes");
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let outcome = session.perform(SessionOp::Undo);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(session.committed_doc().bit_eq(&before), "undo is exact");
}

/// Claim 9: `RemoveConstraint` of an id the bundle does not hold is
/// refused typed, never a panic.
#[test]
fn r1_removing_an_unknown_constraint_refuses_typed() {
    let tol = Tol::witness();
    let bench = asm::bench("r1-unknown", tol);
    let mut session = asm::open_bench(&bench, tol);
    let bogus = pncad::document::ConstraintId::placeholder(999);
    let outcome = session.perform(SessionOp::RemoveConstraint {
        mate: MateRef::new(bench.post_b, bogus),
    });
    std::fs::remove_dir_all(&bench.dir).expect("removable");
    assert!(outcome.refusal.is_some(), "refused");
}
