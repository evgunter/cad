//! **Review probes, PR 6300 (lane `placing-dual-6300-r2`)**, at
//! `a945aa17be`. Each row is a claim of the PR made to go red if it is
//! false. Rows marked `#[ignore]` are red at the frozen head and are
//! kept as the finding's evidence.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::asm;
use pncad::document::{
    Coord, Dimension, Formula, MateRef, Node, RecipeNodeId, SlotId, TargetAxis, VarKind, VarName,
};
use pncad::geom_core::Tol;
use viewer::matetool::{MateChoice, MateTool};
use viewer::session::{BundleVerdict, DocSession, SessionOp};

fn authored(
    session: &DocSession,
    placement: RecipeNodeId,
) -> Vec<pncad::document::Constraint<Formula>> {
    let doc = session.committed_doc();
    match doc.node(placement).map(|node| node.authored(doc)) {
        Some(Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. }) => {
            constraints
        }
        other => panic!("{placement:?} is a placement: {other:?}"),
    }
}

/// The formula on `coord`, as its text says it.
fn by_on(session: &DocSession, placement: RecipeNodeId, coord: Coord) -> String {
    authored(session, placement)
        .iter()
        .find_map(|c| {
            c.value()
                .filter(|v| v.coord == coord)
                .map(|v| v.by.to_string())
        })
        .unwrap_or_else(|| panic!("a value on {coord:?}"))
}

fn said(session: &DocSession, placement: RecipeNodeId) -> Vec<String> {
    authored(session, placement)
        .iter()
        .filter_map(|c| c.value().map(|v| format!("{:?} = {}", v.coord, v.by)))
        .collect()
}

fn id_on(
    session: &DocSession,
    placement: RecipeNodeId,
    coord: Coord,
) -> pncad::document::ConstraintId {
    authored(session, placement)
        .iter()
        .find_map(|c| c.value().filter(|v| v.coord == coord).map(|_| c.id))
        .unwrap_or_else(|| panic!("a value on {coord:?}: {:?}", said(session, placement)))
}

fn rest() -> MateChoice {
    MateChoice {
        kind: VarKind::Plane,
        flip: true,
        values: Vec::new(),
    }
}

fn seated(session: &DocSession, bench: &asm::Bench) -> MateTool {
    let (post_top, shelf_bottom) = asm::seat_picks(session, bench);
    let mut tool = MateTool::new();
    assert!(tool.pick(session.doc(), post_top).is_none());
    assert!(tool.pick(session.doc(), shelf_bottom).is_none());
    tool
}

fn declare(session: &mut DocSession, name: &str, dim: Dimension, at: f64) {
    let outcome = session.perform(SessionOp::DeclareVar {
        name: VarName::new(name).expect("a name"),
        value: pncad::document::FreeVar::continuous(dim, at),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
}

/// Claim 3, removal: post_b rested under the shelf, its spin about the
/// shelf's normal reading the named angle `pa`. The tree's `remove` on
/// the rest keeps the copy where it stands — but does it keep `pa`?
/// The spin about the target's own direction (`Spin(None)`) is no
/// freedom of the unmated bundle (`Spin(Some(Z))`), so the value is
/// looked up by coord, not found, and the turn about `z` is measured.
#[test]
fn r2_removing_a_mate_keeps_a_spin_reading_a_variable() {
    let tol = Tol::witness();
    let bench = asm::bench("r2-remove-var", tol);
    let mut session = asm::open_bench(&bench, tol);
    let tool = seated(&session, &bench);
    let proposal = common::proposed(&session, &tool, rest()).expect("the rest proposes");
    let mate = common::commit_mate(&mut session, proposal.op());
    declare(&mut session, "pa", Dimension::Angle, 0.3);
    let spin = id_on(&session, bench.post_b, Coord::SPIN);
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: bench.post_b,
        slot: SlotId::ValueSpin { constraint: spin },
        text: "pa".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let pa = by_on(&session, bench.post_b, Coord::SPIN);
    let before: Vec<String> = said(&session, bench.post_b);
    let outcome = session.perform(SessionOp::RemoveConstraint { mate });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let after = said(&session, bench.post_b);
    let read: Vec<String> = authored(&session, bench.post_b)
        .iter()
        .filter_map(|c| {
            c.value()
                .map(|v| viewer::matetool::value_text(session.committed_doc(), &v.by))
        })
        .collect();
    std::fs::remove_dir_all(&bench.dir).expect("removable");
    // The rest's spin turns about the shelf's own normal, `-z`, so the
    // chain's turn about `z` reads it with its sign: `-pa`, still `pa`.
    assert!(
        read.iter().any(|text| text.replace(' ', "") == "-pa"),
        "the turn about z still reads pa ({pa}) after the rest is removed: {read:?} \
         ({after:?}, was {before:?})"
    );
}

/// Claim 3, adding a mate: post_b unmated, its turn about `z` reading
/// `pa`. A rest on the shelf (normal along `z`) leaves the spin about
/// `z`; does the draft keep `pa` on it?
#[test]
fn r2_adding_a_rest_keeps_a_turn_about_its_normal_reading_a_variable() {
    let tol = Tol::witness();
    let bench = asm::bench("r2-add-var", tol);
    let mut session = asm::open_bench(&bench, tol);
    declare(&mut session, "pa", Dimension::Angle, 0.0);
    let spin_z = id_on(&session, bench.post_b, Coord::Spin(Some(TargetAxis::Z)));
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: bench.post_b,
        slot: SlotId::ValueSpin { constraint: spin_z },
        text: "pa".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let pa = by_on(&session, bench.post_b, Coord::Spin(Some(TargetAxis::Z)));
    let before = said(&session, bench.post_b);
    let tool = seated(&session, &bench);
    let proposal = common::proposed(&session, &tool, rest()).expect("the rest proposes");
    common::commit_mate(&mut session, proposal.op());
    let after = said(&session, bench.post_b);
    std::fs::remove_dir_all(&bench.dir).expect("removable");
    assert!(
        after.iter().any(|s| s.ends_with(&format!("= {pa}"))),
        "the spin about the shelf's normal still reads pa ({pa}): {after:?} (was {before:?})"
    );
}

/// Claim 6: the verdict on a draft of two rests is what Apply does.
/// The PR's own row admits either `Refused` or `Faults`; this row
/// applies the draft and asserts the outcome matches the verdict's
/// class — a refused verdict refuses, a fault verdict lands.
#[test]
fn r2_the_verdict_on_two_rests_is_what_apply_does() {
    let tol = Tol::witness();
    let bench = asm::bench("r2-verdict", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (post_top, shelf_bottom) = asm::seat_picks(&session, &bench);
    let mut two = seated(&session, &bench).chosen(rest()).expect("a rest");
    two.add_mate(VarKind::Plane);
    two.pick(session.doc(), post_top);
    two.pick(session.doc(), shelf_bottom);
    let twice = common::proposed(&session, &two, rest()).expect("two rests propose");
    let verdict = session.bundle_verdict(&twice.op());
    let before = session.committed_doc().clone();
    let states = session.history().len();
    let outcome = session.perform(twice.op());
    std::fs::remove_dir_all(&bench.dir).expect("removable");
    match verdict {
        BundleVerdict::Refused(said) => {
            let got = outcome.refusal.as_ref().map(ToString::to_string);
            assert_eq!(got, Some(said.to_string()), "the same refusal");
            assert_eq!(session.history().len(), states, "nothing recorded");
            assert!(session.committed_doc().bit_eq(&before), "nothing changed");
        }
        other => {
            assert!(
                outcome.refusal.is_none(),
                "{other:?} but {:?}",
                outcome.refusal
            );
        }
    }
}

/// Claim 1: the Create door's Apply is one undo step that restores the
/// exact prior document.
#[test]
fn r2_the_create_doors_apply_undoes_to_the_exact_prior_document() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(
        pncad::document::Doc::empty_derived("r2-create-undo", tol),
        tol,
    );
    let body = common::xy_box_in(&mut session, [0.02, 0.03, 0.01]);
    session.pump();
    let before = session.committed_doc().clone();
    let states = session.history().len();
    let tool = MateTool::create(pncad::document::Operand::Node(body));
    let (doc, ev) = session.landed_pair().expect("landed");
    let proposal = tool.proposal(doc, ev, tol).expect("no mate proposes");
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    assert_eq!(session.history().len(), states + 1, "one history state");
    let outcome = session.perform(SessionOp::Undo);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        session.committed_doc().bit_eq(&before),
        "one undo takes the copy back"
    );
}

/// Claim 1/9: the tree's `remove` is one undo step back to the exact
/// bundle with the rest.
#[test]
fn r2_a_removal_undoes_to_the_exact_prior_document() {
    let tol = Tol::witness();
    let bench = asm::bench("r2-remove-undo", tol);
    let mut session = asm::open_bench(&bench, tol);
    let tool = seated(&session, &bench);
    let proposal = common::proposed(&session, &tool, rest()).expect("the rest proposes");
    let mate = common::commit_mate(&mut session, proposal.op());
    let before = session.committed_doc().clone();
    let states = session.history().len();
    let outcome = session.perform(SessionOp::RemoveConstraint {
        mate: MateRef::new(bench.post_b, mate.id),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    assert_eq!(session.history().len(), states + 1, "one history state");
    let outcome = session.perform(SessionOp::Undo);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    std::fs::remove_dir_all(&bench.dir).expect("removable");
    assert!(
        session.committed_doc().bit_eq(&before),
        "one undo restores the rest"
    );
}

/// Claim 3, sense: flipping the loaded rest's sense changes its chart,
/// so constants are measured again — but a slide reading `px` stays.
#[test]
fn r2_turning_a_loaded_rests_sense_keeps_a_slide_reading_a_variable() {
    let tol = Tol::witness();
    let bench = asm::bench("r2-sense-var", tol);
    let mut session = asm::open_bench(&bench, tol);
    let tool = seated(&session, &bench);
    let proposal = common::proposed(&session, &tool, rest()).expect("the rest proposes");
    common::commit_mate(&mut session, proposal.op());
    declare(&mut session, "px", Dimension::Length, asm::POST_B_AT[0]);
    let slide = id_on(&session, bench.post_b, Coord::Slide(TargetAxis::X));
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: bench.post_b,
        slot: SlotId::ValueSlide { constraint: slide },
        text: "px".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let px = by_on(&session, bench.post_b, Coord::Slide(TargetAxis::X));
    let (doc, ev) = session.landed_pair().expect("landed");
    let mut edit = MateTool::edit(doc, bench.post_b).expect("a placement");
    edit.set_sense(0, false);
    let flushed = edit.proposal(doc, ev, tol).expect("a flush proposes");
    let row = flushed
        .values
        .iter()
        .find(|row| row.coord == Coord::Slide(TargetAxis::X))
        .expect("a slide along x");
    std::fs::remove_dir_all(&bench.dir).expect("removable");
    assert_eq!(row.by.to_string(), px, "{:?}", flushed.values);
}

/// Claim 8: once the Create door has placed a body, a pick on what the
/// picture shows there opens that copy's Edit door, never a second
/// Create.
#[test]
fn r2_a_body_seen_through_its_copy_opens_the_copys_edit_door() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(
        pncad::document::Doc::empty_derived("r2-seen-through", tol),
        tol,
    );
    let body = common::xy_box_in(&mut session, [0.02, 0.03, 0.01]);
    session.pump();
    let tool = MateTool::create(pncad::document::Operand::Node(body));
    let (doc, ev) = session.landed_pair().expect("landed");
    let proposal = tool.proposal(doc, ev, tol).expect("no mate proposes");
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let doc = session.committed_doc();
    let placed: Vec<_> = doc
        .placements()
        .into_iter()
        .filter(|&p| matches!(doc.node(p), Some(Node::Place { source, .. }) if Some(source.read) == doc.outputs(body).first().copied()))
        .collect();
    let [copy] = placed[..] else {
        panic!("one copy of the body: {placed:?}")
    };
    let drawn = viewer::world::drawn_nodes(doc);
    eprintln!("body {body:?} copy {copy:?} drawn {drawn:?}");
    let index = common::index_of(&session, asm::delta());
    let face = common::displayed_face_at(&session, &index, &common::down_at(0.0, 0.0));
    let mut fresh = MateTool::new();
    let event = fresh.pick(session.doc(), face.clone());
    assert!(event.is_none(), "{event:?}");
    assert!(
        matches!(fresh.door(), Some(viewer::matetool::PlaceDoor::Edit { placement, .. }) if *placement == copy),
        "picked {:?}; door {:?}",
        face.node,
        fresh.door()
    );
}

/// Claims 1, 6 and 8 end to end on two bodies: the Create door on an
/// unplaced box with a flush plane mate onto a second box's top, its
/// verdict, its Apply, and then the copy opened again through a pick —
/// the Edit door on it, whose untouched draft stands and writes nothing.
#[test]
fn r2_a_created_copy_reopens_through_a_pick_and_its_untouched_draft_stands() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(
        pncad::document::Doc::empty_derived("r2-create-mate", tol),
        tol,
    );
    let small = common::xy_box_in(&mut session, [0.02, 0.03, 0.01]);
    let _wide = common::xy_box_in(&mut session, [0.04, 0.05, 0.005]);
    session.pump();
    let index = common::index_of(&session, asm::delta());
    let top_small = common::displayed_face_at(&session, &index, &common::down_at(0.0, 0.0));
    let top_wide = common::displayed_face_at(&session, &index, &common::down_at(0.015, 0.02));
    assert_eq!(top_small.node, small, "premise: the small box's top");
    let mut tool = MateTool::new();
    assert!(tool.pick(session.doc(), top_small).is_none());
    assert!(tool.pick(session.doc(), top_wide).is_none());
    assert!(matches!(
        tool.door(),
        Some(viewer::matetool::PlaceDoor::Create { .. })
    ));
    let flush = MateChoice {
        kind: VarKind::Plane,
        flip: false,
        values: Vec::new(),
    };
    let proposal = common::proposed(&session, &tool, flush).expect("a flush proposes");
    let verdict = session.bundle_verdict(&proposal.op());
    eprintln!("create verdict {verdict:?}; values {:?}", proposal.values);
    assert!(matches!(verdict, BundleVerdict::Stands(_)), "{verdict:?}");
    let states = session.history().len();
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    assert_eq!(session.history().len(), states + 1, "one history state");
    let doc = session.committed_doc();
    let copy = doc
        .placements()
        .into_iter()
        .find(|&p| matches!(doc.node(p), Some(Node::Place { source, .. }) if Some(source.read) == doc.outputs(small).first().copied()))
        .expect("the small box's copy");
    // The flush lays the small box's top level with the wide one's, so
    // its underside, at z = -0.005, is what an up ray meets first.
    let index = common::index_of(&session, asm::delta());
    let again = common::displayed_face_at(&session, &index, &asm::up_at(0.0, 0.0));
    let mut edit = MateTool::new();
    assert!(edit.pick(session.doc(), again.clone()).is_none());
    assert!(
        matches!(edit.door(), Some(viewer::matetool::PlaceDoor::Edit { placement, .. }) if *placement == copy),
        "the pick ({:?}) opens the copy's Edit door: {:?}",
        again.node,
        edit.door()
    );
    // Drop the fresh row the pick opened: the loaded bundle alone.
    let rows = edit.rows().len();
    edit.strike(rows - 1, true);
    let (doc, ev) = session.landed_pair().expect("landed");
    let untouched = edit
        .proposal(doc, ev, tol)
        .expect("the loaded bundle proposes");
    let verdict = session.bundle_verdict(&untouched.op());
    assert!(matches!(verdict, BundleVerdict::Stands(_)), "{verdict:?}");
    let states = session.history().len();
    let outcome = session.perform(untouched.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        session.history().len(),
        states,
        "an untouched draft writes nothing"
    );
}

/// The panel's value rows (`pane/create.rs`, `mate_tool_ui`) are each a
/// field showing its formula as the person reads it
/// (`matetool::value_text`). This row reads that text on the two drafts
/// a person meets first: the Create door's (a body where it stands) and
/// the Edit door's over a just-committed rest. No row spells a variable
/// as its raw id.
#[test]
fn r2_the_panels_value_rows_are_editable_numbers() {
    let tol = Tol::witness();
    let bench = asm::bench("r2-panel-fields", tol);
    let mut session = asm::open_bench(&bench, tol);
    let tool = seated(&session, &bench);
    let proposal = common::proposed(&session, &tool, rest()).expect("the rest proposes");
    common::commit_mate(&mut session, proposal.op());
    let (doc, ev) = session.landed_pair().expect("landed");
    let edit = MateTool::edit(doc, bench.post_b).expect("a placement");
    let loaded = edit
        .proposal(doc, ev, tol)
        .expect("the loaded bundle proposes");
    let texts: Vec<String> = loaded
        .values
        .iter()
        .map(|row| viewer::matetool::value_text(doc, &row.by))
        .collect();
    std::fs::remove_dir_all(&bench.dir).expect("removable");
    assert_eq!(texts.len(), 3, "a rest leaves two slides and its spin");
    assert!(
        texts.iter().all(|text| !text.contains('#')),
        "a value row spells a variable as its raw id: {texts:?}"
    );
    assert!(
        texts
            .iter()
            .all(|text| text.chars().any(|c| c.is_ascii_digit())),
        "each loaded number reads as a number: {texts:?}"
    );
}
