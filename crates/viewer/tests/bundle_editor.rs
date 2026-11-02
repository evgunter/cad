//! **The placement editor**: a draft of one placement's whole bundle,
//! applied as one edit through one of two doors — Create on a body not
//! yet placed, Edit on a placement, keeping its ids — with the verdict
//! judged before Apply, a stale draft reloaded, no value reading a
//! variable re-written as a measured number, and a mate struck out
//! through the tree's `remove` measured where the copy stands.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::asm;
use pncad::document::{
    Coord, Dimension, Frame, MateRef, Node, Operand, RecipeNodeId, SlotId, TargetAxis, VarKind,
    VarName,
};
use pncad::geom_core::Tol;
use viewer::matetool::{MateChoice, MateTool, MateToolEvent, PlaceDoor, ValueFrom};
use viewer::session::{BundleVerdict, DocSession, Refusal, SessionOp};

/// post_b's bundle as authored.
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

/// The id and the number of the value on `coord` in `placement`'s bundle.
fn value_on(
    session: &DocSession,
    placement: RecipeNodeId,
    coord: Coord,
) -> (pncad::document::ConstraintId, String) {
    authored(session, placement)
        .iter()
        .find_map(|c| {
            c.value()
                .filter(|v| v.coord == coord)
                .map(|v| (c.id, v.by.to_string()))
        })
        .unwrap_or_else(|| panic!("a value on {coord:?}"))
}

/// The seat's plane rest, through the editor on post_b's copy.
fn seated(session: &DocSession, bench: &asm::Bench) -> MateTool {
    let (post_top, shelf_bottom) = asm::seat_picks(session, bench);
    let mut tool = MateTool::new();
    assert!(tool.pick(session.doc(), post_top).is_none());
    assert!(tool.pick(session.doc(), shelf_bottom).is_none());
    tool
}

fn rest() -> MateChoice {
    MateChoice {
        kind: VarKind::Plane,
        flip: true,
        values: Vec::new(),
    }
}

/// **A value reading a variable survives a mate committed beside it**,
/// and Apply is one undo step that keeps every id it loaded. post_b's
/// slide along `x` reads `px`; the seat's rest leaves that slide, so the
/// value keeps reading `px` — never the number it measures — while the
/// constants on the freedoms the rest changes the meaning of are
/// measured where post_b stands.
#[test]
fn a_value_reading_a_variable_is_kept_beside_a_new_mate() {
    let tol = Tol::witness();
    let bench = asm::bench("bundle-var", tol);
    let mut session = asm::open_bench(&bench, tol);
    let name = VarName::new("px").expect("a name");
    let outcome = session.perform(SessionOp::DeclareVar {
        name,
        value: pncad::document::FreeVar::continuous(Dimension::Length, asm::POST_B_AT[0]),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let (slide_x, _) = value_on(&session, bench.post_b, Coord::Slide(TargetAxis::X));
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: bench.post_b,
        slot: SlotId::ValueSlide {
            constraint: slide_x,
        },
        text: "px".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let (_, px) = value_on(&session, bench.post_b, Coord::Slide(TargetAxis::X));
    let before = session.committed_doc().clone();
    let states = session.history().len();

    let tool = seated(&session, &bench);
    let proposal = common::proposed(&session, &tool, rest()).expect("the rest proposes");
    let row = proposal
        .values
        .iter()
        .find(|row| row.coord == Coord::Slide(TargetAxis::X))
        .expect("a rest along z leaves the slide along x");
    assert_eq!(row.from, ValueFrom::Loaded(slide_x), "loaded, not measured");
    common::commit_mate(&mut session, proposal.op());

    let (id, by) = value_on(&session, bench.post_b, Coord::Slide(TargetAxis::X));
    assert_eq!(id, slide_x, "Apply keeps the loaded value's id");
    assert_eq!(by, px, "the value still reads px");
    assert_eq!(session.history().len(), states + 1, "one history state");
    let outcome = session.perform(SessionOp::Undo);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        session.committed_doc().bit_eq(&before),
        "one undo takes the whole bundle back"
    );
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

/// **A draft over a bundle the document has since changed is stale**:
/// the survival step loads it again, said, and an Apply of the old one
/// is refused rather than written over the bundle the placement holds.
#[test]
fn a_stale_draft_reloads_and_its_apply_is_refused() {
    let tol = Tol::witness();
    let bench = asm::bench("bundle-stale", tol);
    let mut session = asm::open_bench(&bench, tol);
    let mut tool = seated(&session, &bench);
    let old = common::proposed(&session, &tool, rest()).expect("the rest proposes");
    let (slide_y, _) = value_on(&session, bench.post_b, Coord::Slide(TargetAxis::Y));
    let outcome = session.perform(SessionOp::SetSlot {
        node: bench.post_b,
        slot: SlotId::ValueSlide {
            constraint: slide_y,
        },
        value: viewer::props::SlotValue::of(Dimension::Length, 0.01).expect("a value"),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let refused = session.perform(old.op());
    assert!(
        matches!(&refused.refusal, Some(Refusal::BundleStale { node }) if node.id() == bench.post_b),
        "the old draft is not written: {:?}",
        refused.refusal
    );
    let (doc, ev) = session.landed_pair().expect("landed");
    let events = tool.reconcile(doc, ev);
    assert!(
        matches!(&events[..], [MateToolEvent::Reloaded { placement }] if placement.id() == bench.post_b),
        "the draft reloads, said: {events:?}"
    );
    assert!(
        tool.rows().is_empty(),
        "the reloaded draft is the bundle as it is: no mate, no pick"
    );
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

/// **The verdict comes before Apply**: a draft whose second mate rests
/// the same faces again repeats the first's equations, and the scratch
/// recording says so without a history state; the draft with one rest
/// stands its copy.
#[test]
fn the_verdict_names_a_repeated_equation_before_apply() {
    let tol = Tol::witness();
    let bench = asm::bench("bundle-verdict", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (post_top, shelf_bottom) = asm::seat_picks(&session, &bench);
    let tool = seated(&session, &bench);
    let one = common::proposed(&session, &tool, rest()).expect("one rest proposes");
    assert!(
        matches!(session.bundle_verdict(&one.op()), BundleVerdict::Stands(_)),
        "one rest stands its copy"
    );
    let mut two = tool.chosen(rest()).expect("a rest");
    two.add_mate(VarKind::Plane);
    two.pick(session.doc(), post_top);
    two.pick(session.doc(), shelf_bottom);
    let twice = common::proposed(&session, &two, rest()).expect("two rests propose");
    assert_eq!(twice.mates.len(), 2);
    let states = session.history().len();
    let verdict = session.bundle_verdict(&twice.op());
    let BundleVerdict::Refused(Refusal::Edit(said)) = &verdict else {
        panic!("a second rest of the same faces is the door's refusal: {verdict:?}")
    };
    assert!(
        matches!(
            &**said,
            pncad::document::EditError::MateRefused { fault, .. }
                if matches!(**fault, pncad::document::MateFault::Overconstrained { .. })
        ),
        "the door says the repeated equation: {said:?}"
    );
    assert_eq!(
        session.history().len(),
        states,
        "the verdict records nothing"
    );
    // Apply does what the verdict said: the same refusal, nothing recorded.
    let before = session.committed_doc().clone();
    let outcome = session.perform(twice.op());
    assert_eq!(
        outcome.refusal.as_ref().map(ToString::to_string),
        Some(Refusal::Edit(said.clone()).to_string()),
        "Apply refuses as the verdict said"
    );
    assert!(session.committed_doc().bit_eq(&before), "nothing changed");
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

/// **A copy that would not stand is not applied from the editor**: a
/// point mate read off two flat faces does not evaluate
/// (`SideUnevaluated`), so the verdict is the placement's fault, and
/// Apply is withheld with it, nothing recorded.
#[test]
fn an_apply_whose_copy_would_not_stand_is_withheld() {
    let tol = Tol::witness();
    let bench = asm::bench("bundle-withheld", tol);
    let mut session = asm::open_bench(&bench, tol);
    let tool = seated(&session, &bench);
    let point = MateChoice {
        kind: VarKind::Point,
        flip: false,
        values: Vec::new(),
    };
    let proposal = common::proposed(&session, &tool, point).expect("two faces propose as points");
    let verdict = session.bundle_verdict(&proposal.op());
    assert!(
        matches!(&verdict, BundleVerdict::Faults(fault)
            if matches!(**fault, pncad::document::MateFault::SideUnevaluated { .. })),
        "a point read off a flat face does not evaluate: {verdict:?}"
    );
    let states = session.history().len();
    let outcome = session.perform(proposal.op());
    assert!(
        matches!(&outcome.refusal, Some(Refusal::PlacementFails { fault: Some(fault) })
            if matches!(**fault, pncad::document::MateFault::SideUnevaluated { .. })),
        "Apply is withheld with the verdict's fault: {:?}",
        outcome.refusal
    );
    assert_eq!(session.history().len(), states, "nothing is recorded");
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

/// **The kernel's one chain read turns as the rotation does near its
/// pole** (`editor_core::mate::chain_turns`): a turn whose `x` stands a
/// hair off `z`, where the general branch reads it, and one exactly on
/// it, where the pole branch does — each read back into the chain
/// `Rz·Ry·Rx` and compared with the rotation it was read off.
#[test]
fn the_chain_read_turns_as_the_rotation_does_at_and_near_its_pole() {
    use pncad::geom_core::linalg::{Mat3, Vec3};
    let about = |axis: Vec3<f64>, a: f64| {
        let (s, c) = a.sin_cos();
        let turn = |v: Vec3<f64>| v * c + axis.cross(v) * s + axis * (axis.dot(v) * (1.0 - c));
        Mat3::from_cols(
            turn(Vec3::unit_x()),
            turn(Vec3::unit_y()),
            turn(Vec3::unit_z()),
        )
    };
    let chain = |[x, y, z]: [f64; 3]| {
        about(Vec3::unit_z(), z) * about(Vec3::unit_y(), y) * about(Vec3::unit_x(), x)
    };
    for off in [0.0, 1.0e-6, 1.0e-3] {
        let r = chain([0.4, -std::f64::consts::FRAC_PI_2 + off, 0.7]);
        let read = editor_core::mate::chain_turns(&r);
        let back = chain(read);
        let gap = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()]
            .iter()
            .map(|&e| (back * e - r * e).norm())
            .fold(0.0, f64::max);
        assert!(
            gap < 1.0e-8,
            "{off} off the pole: the chain read {read:?} turns {gap} away"
        );
    }
}

/// **The tree's `remove`**: a value is refused — its freedom is always
/// set — and a mate is struck out of the editor's draft, the freedoms it
/// frees measured where the copy stands. post_b, rested under the shelf
/// and spun a third of a quarter turn about the shelf's normal, so off
/// every axis of the target, keeps its pose once its rest is removed:
/// the bundle is written by the slides its geometry leaves, never the
/// structural door's zeros.
#[test]
fn removing_a_mate_from_a_turned_copy_keeps_it_where_it_stands() {
    let tol = Tol::witness();
    let bench = asm::bench("bundle-remove", tol);
    let mut session = asm::open_bench(&bench, tol);
    let tool = seated(&session, &bench);
    let proposal = common::proposed(&session, &tool, rest()).expect("the rest proposes");
    let mate = common::commit_mate(&mut session, proposal.op());
    let (spin, _) = value_on(&session, bench.post_b, Coord::SPIN);
    let outcome = session.perform(SessionOp::SetSlot {
        node: bench.post_b,
        slot: SlotId::ValueSpin { constraint: spin },
        value: viewer::props::SlotValue::of(Dimension::Angle, 30.0_f64.to_radians())
            .expect("a value"),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let pose = |session: &DocSession| -> Frame {
        let (doc, _) = session.landed_pair().expect("landed");
        common::solve(session, doc, tol)
            .placement(doc, bench.post_b)
            .expect("post_b has a pose")
    };
    let before = pose(&session);

    let refused = session.perform(SessionOp::RemoveConstraint {
        mate: MateRef::new(bench.post_b, spin),
    });
    assert!(
        matches!(&refused.refusal, Some(Refusal::ValueNotRemoved { node }) if node.id() == bench.post_b),
        "a value is not removed: {:?}",
        refused.refusal
    );
    let (doc, ev) = session.landed_pair().expect("landed");
    let struck = MateTool::edit_striking(doc, mate).expect("the rest is a mate");
    assert!(
        struck.rows()[0].struck(),
        "the tree's remove opens it struck"
    );
    let draft = struck
        .proposal(doc, ev, tol)
        .expect("the struck draft proposes");
    assert!(draft.mates.is_empty(), "no live mate");
    assert!(
        draft
            .values
            .iter()
            .any(|row| row.from == ValueFrom::Measured),
        "the freed freedoms are measured: {:?}",
        draft.values
    );
    let outcome = session.perform(SessionOp::RemoveConstraint { mate });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    assert!(
        authored(&session, bench.post_b)
            .iter()
            .all(|c| c.mate().is_none()),
        "the rest is gone"
    );
    let after = pose(&session);
    let close = |a: [f64; 3], b: [f64; 3]| a.iter().zip(&b).all(|(x, y)| (x - y).abs() < 1e-9);
    assert!(
        close(after.translation, before.translation)
            && (0..3).all(|c| close(after.columns[c], before.columns[c])),
        "post_b stands where it stood: {after:?} was {before:?}"
    );
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

/// **The Create door on a body not yet placed**: no mate, its copy where
/// the body stands, in the body's own space — one placement inserted,
/// one undo step.
#[test]
fn the_create_door_places_a_body_where_it_stands() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(
        pncad::document::Doc::empty_derived("bundle-create", tol),
        tol,
    );
    let body = common::xy_box_in(&mut session, [0.02, 0.03, 0.01]);
    session.pump();
    assert!(
        viewer::world::placements_of(session.committed_doc(), body).is_empty(),
        "the premise: the body lands unplaced"
    );
    let tool = MateTool::create(Operand::Node(body));
    assert!(matches!(tool.door(), Some(PlaceDoor::Create { .. })));
    let (doc, ev) = session.landed_pair().expect("landed");
    let proposal = tool.proposal(doc, ev, tol).expect("no mate proposes");
    assert!(proposal.mates.is_empty());
    assert_eq!(
        proposal.target,
        doc.operand_space(&Operand::Node(body)),
        "into the body's own space"
    );
    assert!(
        proposal
            .values
            .iter()
            .all(|row| row.by.literal_value() == Some(0.0) || row.by.to_string() == "0"),
        "the copy stands where the body stands: {:?}",
        proposal.values
    );
    let states = session.history().len();
    let placed = session.committed_doc().placements();
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    assert_eq!(session.history().len(), states + 1, "one history state");
    let doc = session.committed_doc();
    let new: Vec<_> = doc
        .placements()
        .into_iter()
        .filter(|p| !placed.contains(p))
        .collect();
    let [copy] = new[..] else {
        panic!("one placement inserted: {new:?}")
    };
    assert!(
        matches!(doc.node(copy), Some(Node::Place { source, .. })
            if doc.operand_space(&Operand::Node(body)) == doc.node_space(copy)
                && Some(source.read) == doc.outputs(body).first().copied()),
        "a copy of the body in its own space: {:?}",
        doc.node(copy)
    );
}

/// **A kept chart keeps its slides**: post_b rested under the shelf, the
/// shelf then turned 60° about `x`, so the rest's target leans past the
/// diagonal and the orientation nearest where post_b now stands is no
/// longer the loaded one. The Edit door opened untouched still lists the
/// loaded slides — the chart is the bundle's — and its Apply writes
/// nothing.
#[test]
fn an_untouched_draft_over_a_tilted_target_keeps_its_slides() {
    let tol = Tol::witness();
    let bench = asm::bench("bundle-tilted", tol);
    let mut session = asm::open_bench(&bench, tol);
    let tool = seated(&session, &bench);
    let proposal = common::proposed(&session, &tool, rest()).expect("the rest proposes");
    common::commit_mate(&mut session, proposal.op());
    let loaded: Vec<Coord> = authored(&session, bench.post_b)
        .iter()
        .filter_map(|c| c.value().map(|v| v.coord))
        .collect();
    let (tilt, _) = value_on(&session, bench.shelf_i, Coord::Spin(Some(TargetAxis::X)));
    let outcome = session.perform(SessionOp::SetSlot {
        node: bench.shelf_i,
        slot: SlotId::ValueSpin { constraint: tilt },
        value: viewer::props::SlotValue::of(Dimension::Angle, 60.0_f64.to_radians())
            .expect("a value"),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let (doc, ev) = session.landed_pair().expect("landed");
    let draft = MateTool::edit(doc, bench.post_b)
        .expect("post_b is a placement")
        .proposal(doc, ev, tol)
        .expect("the loaded bundle proposes");
    let coords: Vec<Coord> = draft.values.iter().map(|row| row.coord).collect();
    assert_eq!(coords, loaded, "the loaded slides and spin");
    assert!(
        draft.dropped.is_empty(),
        "nothing drops: {:?}",
        draft.dropped
    );
    let states = session.history().len();
    let outcome = session.perform(draft.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        session.history().len(),
        states,
        "an untouched draft writes nothing"
    );
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

/// **A Create draft over a body placed since is stale**: its Apply,
/// sent again once the body has its copy, is refused rather than
/// placing the body a second time.
#[test]
fn a_create_draft_over_a_body_placed_since_is_refused() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(
        pncad::document::Doc::empty_derived("bundle-create-stale", tol),
        tol,
    );
    let body = common::xy_box_in(&mut session, [0.02, 0.03, 0.01]);
    session.pump();
    let (doc, ev) = session.landed_pair().expect("landed");
    let proposal = MateTool::create(Operand::Node(body))
        .proposal(doc, ev, tol)
        .expect("no mate proposes");
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let states = session.history().len();
    let again = session.perform(proposal.op());
    assert!(
        matches!(again.refusal, Some(Refusal::BundleStale { .. })),
        "a second Create of the body is stale: {:?}",
        again.refusal
    );
    assert_eq!(session.history().len(), states, "nothing is recorded");
}

/// Declare the angle `name` at `at` and let `post_b`'s value `coord`
/// read it.
fn reads_angle(session: &mut DocSession, bench: &asm::Bench, coord: Coord, name: &str, at: f64) {
    let outcome = session.perform(SessionOp::DeclareVar {
        name: VarName::new(name).expect("a name"),
        value: pncad::document::FreeVar::continuous(Dimension::Angle, at),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let (constraint, _) = value_on(session, bench.post_b, coord);
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: bench.post_b,
        slot: SlotId::ValueSpin { constraint },
        text: name.to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
}

/// Set the angle `name` to `at`, landed.
fn set_angle(session: &mut DocSession, name: &str, at: f64) {
    let var = common::var_of(session.committed_doc(), name);
    let outcome = session.perform(SessionOp::SetVariable {
        var,
        value: viewer::props::SlotValue::Continuous(at),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
}

/// post_b's turn, landed.
fn turn_of(session: &DocSession, bench: &asm::Bench, tol: Tol) -> [[f64; 3]; 3] {
    let (doc, _) = session.landed_pair().expect("landed");
    common::solve(session, doc, tol)
        .placement(doc, bench.post_b)
        .expect("post_b has a pose")
        .columns
}

/// **A carried turn writes the exact quarter turns between the two
/// zeros**: post_b turned 100° about `z` before its rest, so the rest's
/// orientation is a quarter turn (`x` on `y`) and the spin's zero stands
/// a quarter turn from the chain's. Its spin reading `pa`, the rest
/// removed: the chain's turn about `z` reads `-pa + turn/4`, written
/// exactly, and `pa` turns the freed copy as it turned the rested one.
#[test]
fn a_carried_turn_writes_the_quarter_turns_between_the_zeros() {
    let tol = Tol::witness();
    let bench = asm::bench("bundle-carry-quarter", tol);
    let mut session = asm::open_bench(&bench, tol);
    // Picked before the turn moves the post off the rays.
    let (post_top, shelf_bottom) = asm::seat_picks(&session, &bench);
    let (spin_z, _) = value_on(&session, bench.post_b, Coord::Spin(Some(TargetAxis::Z)));
    let outcome = session.perform(SessionOp::SetSlot {
        node: bench.post_b,
        slot: SlotId::ValueSpin { constraint: spin_z },
        value: viewer::props::SlotValue::of(Dimension::Angle, 100.0_f64.to_radians())
            .expect("a value"),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let mut tool = MateTool::new();
    tool.pick(session.doc(), post_top);
    tool.pick(session.doc(), shelf_bottom);
    let proposal = common::proposed(&session, &tool, rest()).expect("the rest proposes");
    let mate = common::commit_mate(&mut session, proposal.op());
    reads_angle(&mut session, &bench, Coord::SPIN, "pa", 0.0);
    set_angle(&mut session, "pa", 0.3);
    let rested = turn_of(&session, &bench, tol);
    let outcome = session.perform(SessionOp::RemoveConstraint { mate });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let z = viewer::matetool::value_text(
        session.committed_doc(),
        &authored(&session, bench.post_b)
            .iter()
            .find_map(|c| {
                c.value()
                    .filter(|v| v.coord == Coord::Spin(Some(TargetAxis::Z)))
                    .map(|v| v.by.clone())
            })
            .expect("the chain turns about z"),
    );
    let freed = turn_of(&session, &bench, tol);
    set_angle(&mut session, "pa", -0.4);
    let turned = turn_of(&session, &bench, tol);
    std::fs::remove_dir_all(&bench.dir).expect("removable");
    assert!(
        z.contains("pa") && z.contains("turn"),
        "the turn about z reads pa and a quarter turn: {z}"
    );
    let close = |a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]| {
        a.iter()
            .flatten()
            .zip(b.iter().flatten())
            .all(|(x, y)| (x - y).abs() < 1e-9)
    };
    assert!(
        close(&freed, &rested),
        "the freed copy stands as the rested one: {freed:?} {rested:?}"
    );
    assert!(!close(&turned, &freed), "pa still turns the freed copy");
}

/// **A turn the kernel cannot state as a chain link is said dropped**:
/// post_b's +x side face rests on the shelf's underside, so its `x`
/// stands on `z` — the chain's pole, where the chain's turns about `x`
/// and `z` turn about one line. Its spin reading `pa`, the rest struck
/// out: `pa` is carried onto no freedom, and the draft says it dropped.
#[test]
fn a_turn_at_the_chains_pole_is_said_dropped() {
    let tol = Tol::witness();
    let bench = asm::bench("bundle-carry-pole", tol);
    let mut session = asm::open_bench(&bench, tol);
    let side = asm::pick_face(
        &session,
        &common::along_x(-1.0, asm::POST_SECTION / 2.0, asm::POST_HEIGHT / 2.0),
    );
    let mut tool = MateTool::new();
    tool.pick(session.doc(), side);
    tool.pick(session.doc(), asm::shelf_underside(&session));
    let proposal = common::proposed(&session, &tool, rest()).expect("the rest proposes");
    let mate = common::commit_mate(&mut session, proposal.op());
    reads_angle(&mut session, &bench, Coord::SPIN, "pa", 0.3);
    let (doc, ev) = session.landed_pair().expect("landed");
    let struck = MateTool::edit_striking(doc, mate)
        .expect("the rest is a mate")
        .proposal(doc, ev, tol)
        .expect("the struck draft proposes");
    let read = |rows: &[viewer::matetool::ValueRow]| {
        rows.iter()
            .filter(|row| viewer::matetool::value_text(doc, &row.by).contains("pa"))
            .map(|row| row.coord)
            .collect::<Vec<_>>()
    };
    assert!(
        read(&struck.values).is_empty(),
        "pa is carried onto no freedom: {:?}",
        struck.values
    );
    assert_eq!(
        read(&struck.dropped),
        [Coord::SPIN],
        "pa is said dropped: {:?}",
        struck.dropped
    );
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}
