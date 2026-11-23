//! **Review B of PR 7535 (the datum's viewer half)**: rows written to
//! break the PR's claims. Each row states the claim it tests; a row
//! that goes red is a finding.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use pncad::document::{RecipeNodeId, VarId, VarKind};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
use viewer::matetool::MateTool;
use viewer::session::{DocSession, FaceSelection, Selection, SessionOp};
use viewer::tools::{ToolKind, Tools};

fn cap(node: RecipeNodeId, body: RecipeNodeId, end: CapEnd) -> FaceSelection {
    FaceSelection {
        name: StableName {
            kind: EntityKind::Face,
            node: body,
            path: vec![RoleSeg::Cap(end)],
        },
        node,
        body: 0,
    }
}

fn datum_in(session: &mut DocSession, space: VarId, _kind: VarKind) -> RecipeNodeId {
    let outcome = session.perform(SessionOp::AddDatum { space });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let datum = outcome.made.expect("the op makes the datum");
    session.pump();
    datum
}

/// A datum in the block's space mated flush on its top cap, offset `by`.
fn mated_datum(tol: Tol, by: Option<f64>) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let (doc, block, _) = viewer::test_support::boss_on_block("review-b", tol);
    let space = doc.node_space(block).expect("the block is in its space");
    let mut session = DocSession::inline(doc, tol);
    let datum = datum_in(&mut session, space, VarKind::Plane);
    let mut tool = MateTool::edit(session.doc(), datum).expect("the editor opens");
    tool.add_mate(VarKind::Plane);
    assert!(
        tool.pick(session.doc(), cap(block, block, CapEnd::End))
            .is_none()
    );
    if let Some(by) = by {
        tool.set_offset(0, Some(viewer::test_support::len(by)));
    }
    let (doc, ev) = session.landed_pair().expect("landed");
    let proposal = tool.proposal(doc, ev, tol).expect("proposes");
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    (session, block, datum)
}

fn datum_z(session: &DocSession, datum: RecipeNodeId) -> f64 {
    let (doc, ev) = session.landed_pair().expect("landed");
    let drawn = viewer::sketch::committed(doc, ev, 1e-4, None);
    let d = drawn
        .drawn
        .iter()
        .find(|d| d.node == datum)
        .expect("the datum is drawn");
    d.loops[0].drawn_on(&d.plane)[0].z
}

/// Claim 2b: a datum in a space shown only through a copy is drawn where
/// the copy is. The boss's space is shown only through its world copy,
/// which stands `depth` up the world's z.
#[test]
fn b_a_datum_in_a_space_shown_through_a_copy_is_drawn_where_the_copy_is() {
    let tol = Tol::witness();
    let (doc, _, boss) = viewer::test_support::boss_on_block("review-b-copy", tol);
    let space = doc.node_space(boss).expect("the boss is in its space");
    let depth = viewer::test_support::BOSS_BLOCK[2];
    let mut session = DocSession::inline(doc, tol);
    let datum = datum_in(&mut session, space, VarKind::Plane);
    let z = datum_z(&session, datum);
    assert!(
        (z - depth).abs() < 1e-12,
        "the datum at the boss space's origin is drawn on the boss's copy, \
         at z = {depth}; it is drawn at z = {z}"
    );
}

/// Claim 2c: a datum never enters the picture of bodies or the product.
#[test]
fn b_a_datum_is_no_body_of_the_picture_or_the_product() {
    let tol = Tol::witness();
    let (session, _, datum) = mated_datum(tol, None);
    let picture = viewer::world::Picture::of(session.doc());
    assert!(
        !picture
            .drawn
            .iter()
            .chain(&picture.product)
            .any(|d| d.of_node(datum)),
        "the datum is no drawn body"
    );
}

/// Claim 3c: re-opening a datum and proposing at once writes the bundle
/// it read, and nothing else.
#[test]
fn b_reopening_a_datum_writes_the_bundle_it_read() {
    let tol = Tol::witness();
    for by in [None, Some(0.005)] {
        let (mut session, _, datum) = mated_datum(tol, by);
        let before = session.doc().node(datum).cloned();
        let tool = MateTool::edit(session.doc(), datum).expect("re-opens");
        let (doc, ev) = session.landed_pair().expect("landed");
        let proposal = tool.proposal(doc, ev, tol).expect("proposes");
        let outcome = session.perform(proposal.op());
        assert!(outcome.refusal.is_none(), "{by:?}: {:?}", outcome.refusal);
        assert_eq!(
            session.doc().node(datum).cloned(),
            before,
            "{by:?}: the bundle is as it was"
        );
    }
}

/// Claim 3b: the editor's line names the datum's own side, and a datum
/// picked whole by what it is said as.
#[test]
fn b_the_line_names_the_datums_sides() {
    let tol = Tol::witness();
    let (doc, block, _) = viewer::test_support::boss_on_block("review-b-line", tol);
    let space = doc.node_space(block).expect("in its space");
    let mut session = DocSession::inline(doc, tol);
    let base = datum_in(&mut session, space, VarKind::Plane);
    let datum = datum_in(&mut session, space, VarKind::Axis);
    let mut tool = MateTool::edit(session.doc(), datum).expect("opens");
    tool.add_mate(VarKind::Axis);
    tool.pick_datum(session.doc(), base);
    let line = tool.line(0, pncad::document::Speaker::of(session.doc()));
    assert!(line.contains("the datum's z axis"), "{line}");
    let said = viewer::tree::spelled(&session.doc().spoken(base));
    assert!(line.contains(&said), "{line} names {said}");
    // The edited datum is no far side of its own.
    tool.pick_datum(session.doc(), datum);
    let again = tool.line(0, pncad::document::Speaker::of(session.doc()));
    assert_eq!(line, again, "picking the datum itself changes nothing");
}

/// A datum picked whole in another space than the copy's target is held
/// by the editor's off-target guard, as a face picked there is.
#[test]
fn b_a_datum_in_another_space_is_off_target() {
    let tol = Tol::witness();
    let (doc, block, boss) = viewer::test_support::boss_on_block("review-b-off", tol);
    let boss_space = doc.node_space(boss).expect("in its space");
    let mut session = DocSession::inline(doc, tol);
    let datum = datum_in(&mut session, boss_space, VarKind::Plane);
    let doc = session.doc();
    let copy = doc
        .placements()
        .into_iter()
        .find(|&p| {
            doc.copied_read(p)
                .is_some_and(|read| doc.operation_of(doc.through_sources(read).read) == Some(block))
        })
        .expect("the block's copy");
    assert_eq!(doc.node_space(copy), Some(VarId::WORLD), "premise");
    let mut tool = MateTool::moving(doc, copy, None).expect("opens on the copy");
    tool.add_mate(VarKind::Plane);
    assert!(tool.pick(doc, cap(copy, block, CapEnd::Start)).is_none());
    tool.pick_datum(doc, datum);
    let (doc, ev) = session.landed_pair().expect("landed");
    let off = tool.off_target(doc, ev);
    let proposed = tool.proposal(doc, ev, tol).map(|p| p.op());
    let committed = proposed
        .as_ref()
        .ok()
        .map(|op| session.perform(op.clone()).refusal);
    eprintln!(
        "off_target: {off:?}\nproposal: {:?}\ncommit refusal: {committed:?}",
        proposed.as_ref().err()
    );
    let (doc, ev) = session.landed_pair().expect("landed");
    let _ = (doc, ev);
    assert!(
        !off.is_empty(),
        "a far side read in the boss's space is off the world target"
    );
}

/// Claim 1: deleting a datum a mirror reads takes the mirror with it
/// (DM6: a node that continues nothing takes its readers), and the
/// panel's affordance says so before the click.
#[test]
fn b_deleting_a_datum_a_mirror_reads_cascades_and_says_so() {
    let tol = Tol::witness();
    let (mut session, block, datum) = mated_datum(tol, None);
    let mut tools = Tools::new();
    tools.open(ToolKind::Mirror);
    let doc = session.committed_doc();
    let notices = tools.feed(
        doc,
        Err(viewer::pickcache::NotIndexed::Absent),
        &[
            SessionOp::Select(Selection::Node(block)),
            SessionOp::Select(Selection::Node(datum)),
        ],
    );
    assert!(notices.is_empty(), "{notices:?}");
    let op = tools.mirror().expect("open").op().expect("commits");
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let mirror = outcome.made.expect("the mirror is made");
    // The cascade is the document's: no evaluation of the mirror is
    // waited on.
    assert!(session.delete_refusal(datum).is_none(), "the delete lands");
    let affordance = session.delete_affordance(datum);
    assert!(
        affordance.cascade.contains(&mirror),
        "the mirror goes with the datum: {affordance:?}"
    );
    assert!(affordance.hover.is_some(), "and the hover says so");
    let outcome = session.perform(SessionOp::DeleteNode { node: datum });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(session.doc().node(mirror).is_none());
    // One undo restores both.
    session.perform(SessionOp::Undo);
    assert!(session.doc().node(datum).is_some() && session.doc().node(mirror).is_some());
}

/// Claim 1: a datum's label survives an edit of its bundle.
#[test]
fn b_a_datums_label_survives_an_edit() {
    let tol = Tol::witness();
    let (mut session, block, datum) = mated_datum(tol, None);
    let outcome = session.perform(SessionOp::SetLabel {
        node: datum,
        label: Some(pncad::document::Label::new("shelf").expect("a label")),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let mut tool = MateTool::edit(session.doc(), datum).expect("re-opens");
    tool.set_offset(0, Some(viewer::test_support::len(0.002)));
    let _ = block;
    let (doc, ev) = session.landed_pair().expect("landed");
    let proposal = tool.proposal(doc, ev, tol).expect("proposes");
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        session
            .doc()
            .label(datum)
            .map(ToString::to_string)
            .as_deref(),
        Some("shelf")
    );
}

/// Claim 5: New datum, a datum's offset, and a split reading a datum are
/// one undo step each, and the drawing follows.
#[test]
fn b_undo_redo_one_step_each_and_the_drawing_follows() {
    let tol = Tol::witness();
    let (mut session, block, datum) = mated_datum(tol, Some(0.005));
    let depth = viewer::test_support::BOSS_BLOCK[2];
    // The block's space is the block's own; its top cap stands at depth.
    let z0 = datum_z(&session, datum);
    // Offset changed to 0.003: one step.
    let mut tool = MateTool::edit(session.doc(), datum).expect("re-opens");
    tool.set_offset(0, Some(viewer::test_support::len(0.003)));
    let (doc, ev) = session.landed_pair().expect("landed");
    let outcome = session.perform(tool.proposal(doc, ev, tol).expect("proposes").op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let z1 = datum_z(&session, datum);
    assert!((z1 - z0 + 0.002).abs() < 1e-12, "{z0} -> {z1}");
    // A split reading the datum: one step.
    let mut tools = Tools::new();
    tools.open(ToolKind::Split);
    let notices = tools.feed(
        session.committed_doc(),
        Err(viewer::pickcache::NotIndexed::Absent),
        &[
            SessionOp::Select(Selection::Node(block)),
            SessionOp::Select(Selection::Node(datum)),
        ],
    );
    assert!(notices.is_empty(), "{notices:?}");
    let outcome = session.perform(tools.split().expect("open").op().expect("commits"));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let split = outcome.made.expect("the split is made");
    session.pump();
    session.perform(SessionOp::Undo);
    session.pump();
    assert!(session.doc().node(split).is_none(), "undo takes the split");
    assert!((datum_z(&session, datum) - z1).abs() < 1e-12);
    session.perform(SessionOp::Undo);
    session.pump();
    assert!(
        (datum_z(&session, datum) - z0).abs() < 1e-12,
        "undo takes the offset change"
    );
    assert!((z0 - (depth + 0.005)).abs() < 1e-12, "premise: {z0}");
    session.perform(SessionOp::Undo);
    session.pump();
    assert!(
        (datum_z(&session, datum) - 0.0).abs() < 1e-12,
        "undo takes the mate: the datum is at its space's origin"
    );
    session.perform(SessionOp::Undo);
    session.pump();
    assert!(session.doc().node(datum).is_none(), "undo takes New datum");
    let (doc, ev) = session.landed_pair().expect("landed");
    assert!(
        !viewer::sketch::committed(doc, ev, 1e-4, None)
            .drawn
            .iter()
            .any(|d| d.node == datum),
        "and its drawing"
    );
    for _ in 0..4 {
        session.perform(SessionOp::Redo);
    }
    session.pump();
    assert!(
        session.doc().node(split).is_some(),
        "redo restores all four"
    );
    assert!((datum_z(&session, datum) - z1).abs() < 1e-12);
}

/// A datum offset from a datum picked whole stands off it along its z.
#[test]
fn b_a_datum_offset_from_a_datum_stands_off_it() {
    let tol = Tol::witness();
    let (mut session, block, base) = mated_datum(tol, Some(0.005));
    let space = session.doc().node_space(block).expect("in its space");
    let z_base = datum_z(&session, base);
    let datum = datum_in(&mut session, space, VarKind::Plane);
    let mut tool = MateTool::edit(session.doc(), datum).expect("opens");
    tool.add_mate(VarKind::Plane);
    tool.pick_datum(session.doc(), base);
    tool.set_offset(0, Some(viewer::test_support::len(0.003)));
    let (doc, ev) = session.landed_pair().expect("landed");
    let outcome = session.perform(tool.proposal(doc, ev, tol).expect("proposes").op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let z = datum_z(&session, datum);
    assert!(
        (z - (z_base + 0.003)).abs() < 1e-12,
        "{z_base} + 0.003 != {z}"
    );
}

/// A datum picked whole and then deleted while the editor holds it is
/// said gone, as a picked face's node is (`MateToolError::PickGone`).
#[test]
fn b_a_held_datum_deleted_is_said_gone() {
    let tol = Tol::witness();
    let (mut session, block, base) = mated_datum(tol, None);
    let space = session.doc().node_space(block).expect("in its space");
    let datum = datum_in(&mut session, space, VarKind::Plane);
    let mut tool = MateTool::edit(session.doc(), datum).expect("opens");
    tool.add_mate(VarKind::Plane);
    tool.pick_datum(session.doc(), base);
    let outcome = session.perform(SessionOp::DeleteNode { node: base });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let (doc, ev) = session.landed_pair().expect("landed");
    let err = tool.proposal(doc, ev, tol).map(|p| p.op());
    if let Ok(op) = &err {
        let refusal = session.perform(op.clone()).refusal;
        eprintln!("commit after delete: {refusal:?}");
    }
    assert!(
        matches!(err, Err(viewer::matetool::MateToolError::PickGone { .. })),
        "{err:?}"
    );
}
