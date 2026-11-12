//! **Place, then edit live** (`crates/viewer/README.md`, *A create
//! gesture commits the node*): a pick on a body picked as itself is
//! Place, a body the View pane shows again beside its copy included, so
//! a second placement is made the way the first is; and the placement
//! editor commits each change as one `SetBundle` and one undo step,
//! holding a change the door refuses as a draft over the document until
//! a later change makes the whole of it pass, or Discard drops it. Each
//! row drives the editor and the tools the way the chrome does: a change,
//! its offer to the session's door, the op performed, and the answer.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use pncad::document::{
    Constraint, Coord, Doc, Formula, Node, ProfileProgram, RecipeNodeId, TargetAxis,
};
use pncad::geom_core::Tol;
use viewer::matetool::MateTool;
use viewer::session::{DocSession, Refusal, Selection, SessionOp};
use viewer::tools::{ToolKind, Tools};

/// The small box, standing on the origin and taller than the wide one.
const SMALL: [f64; 3] = [0.02, 0.03, 0.01];
/// The wide box, whose top is clear of the small one's footprint at
/// [`WIDE_TOP`].
const WIDE: [f64; 3] = [0.04, 0.05, 0.005];
/// Where a downward ray meets the wide box's top and nothing else.
const WIDE_TOP: [f64; 2] = [0.015, 0.02];

/// `placement`'s bundle as authored — what the editor loads.
fn authored(doc: &Doc<ProfileProgram>, placement: RecipeNodeId) -> Vec<Constraint<Formula>> {
    match doc.node(placement).map(|node| node.authored(doc)) {
        Some(Node::Place { constraints, .. }) => constraints,
        other => panic!("{placement:?} is a placement: {other:?}"),
    }
}

/// The placements that copy `body`, in document order.
fn copies_of(doc: &Doc<ProfileProgram>, body: RecipeNodeId) -> Vec<RecipeNodeId> {
    doc.placements()
        .into_iter()
        .filter(|&p| doc.copied_read(p) == doc.read_of_node(body))
        .collect()
}

/// The number the slide along `axis` of `placement`'s bundle holds.
fn slide_of(doc: &Doc<ProfileProgram>, placement: RecipeNodeId, axis: TargetAxis) -> f64 {
    let Some(Node::Place { constraints, .. }) = doc.node(placement).map(|node| node.written(doc))
    else {
        panic!("{placement:?} is a placement")
    };
    constraints
        .iter()
        .filter_map(|c| c.value())
        .find(|v| v.coord == Coord::slide(axis))
        .map(|v| {
            if v.by.is_zero() {
                0.0
            } else {
                v.by.literal_value().expect("a literal slide")
            }
        })
        .expect("a value on the slide")
}

fn session(label: &str, tol: Tol) -> DocSession {
    common::inline_without_door(pncad::document::Doc::empty_derived(label, tol), tol)
}

/// **The editor's change offered as the panel offers it**: against the
/// landed pair, judged by the session's door.
// The door answers the session's own `Refusal`, whatever its width.
#[allow(clippy::result_large_err)]
fn offered(session: &DocSession, tool: &mut MateTool) -> Option<SessionOp> {
    let (doc, ev) = session.landed_pair().expect("landed");
    tool.offer(doc, ev, session.tol(), |op| session.bundle_door(op))
}

/// **The editor's change landed**: `op` performed and pumped, and the
/// tools answered as the chrome answers them. Answers whether it
/// committed.
fn landed(session: &mut DocSession, tools: &mut Tools, op: SessionOp) -> bool {
    let outcome = session.perform(op.clone());
    session.pump();
    let committed = outcome.refusal.is_none();
    tools.answered(&op, outcome.made, committed, session.committed_doc());
    committed
}

/// The face a downward ray at `at` meets in the picture as shown.
fn face_at(session: &DocSession, at: [f64; 2]) -> Selection {
    let index = common::asm::index_of(session);
    Selection::Face(common::displayed_face_at(
        session,
        &index,
        &common::down_at(at[0], at[1]),
    ))
}

/// **A pick fed to the open tools**, as the chrome feeds a click, and
/// the Place it asked for, if it asked for one.
fn fed(session: &DocSession, tools: &mut Tools, pick: Selection) -> Option<SessionOp> {
    let index = common::asm::index_of(session);
    let declined = tools.feed(session.doc(), Ok(&index), &[SessionOp::Select(pick)]);
    assert!(declined.is_empty(), "the pick is taken: {declined:?}");
    tools.take_place()
}

/// **Place a body, show it again, pick it, and get a second placement**:
/// the first pick on the body is Place, and the editor opens on the
/// placement it made; the copy moved aside and the body turned on in the
/// View pane, a pick on the body as itself is Place again, which makes a
/// second placement of the same body beside the first.
#[test]
fn a_body_shown_again_and_picked_is_placed_a_second_time() {
    let tol = Tol::witness();
    let mut session = session("place-live-second", tol);
    let body = common::xy_box_in(&mut session, SMALL);
    session.pump();
    let mut tools = Tools::new();
    tools.open(ToolKind::Mate);
    let place = fed(&session, &mut tools, face_at(&session, [0.0, 0.0]))
        .expect("a pick on an unplaced body asks for Place");
    assert_eq!(
        place,
        SessionOp::place_where_it_stands(session.committed_doc(), body.into())
            .expect("a body in a space")
    );
    assert!(landed(&mut session, &mut tools, place), "Place lands");
    let first = tools
        .mate()
        .and_then(|editor| editor.door())
        .map(|door| door.placement)
        .expect("the editor opened on the placement Place made");
    assert_eq!(
        copies_of(session.committed_doc(), body),
        [first],
        "the placement copies the body"
    );
    tools.close();

    // The copy moved aside, and the body shown again beside it.
    let op = common::transformed(&mut session, first, &common::slides([0.1, 0.0, 0.0]));
    assert!(session.perform(op).refusal.is_none());
    let shown = session.perform(SessionOp::SetBodyOn {
        node: body,
        on: true,
    });
    assert!(shown.refusal.is_none(), "{:?}", shown.refusal);
    session.pump();
    let picked = face_at(&session, [0.0, 0.0]);
    let Selection::Face(face) = &picked else {
        unreachable!("a face pick")
    };
    assert_eq!(face.node, body, "premise: the ray meets the body as itself");

    tools.open(ToolKind::Mate);
    let place = fed(&session, &mut tools, picked).expect("the body picked as itself is Place");
    assert_eq!(
        place,
        SessionOp::place_where_it_stands(session.committed_doc(), body.into())
            .expect("a body in a space")
    );
    let states = session.history().len();
    assert!(
        landed(&mut session, &mut tools, place),
        "the second Place lands"
    );
    assert_eq!(session.history().len(), states + 1, "one undo step");
    let second = tools
        .mate()
        .and_then(|editor| editor.door())
        .map(|door| door.placement)
        .expect("the editor opened on the second placement");
    assert_ne!(second, first, "a second placement, not the first again");
    assert_eq!(
        copies_of(session.committed_doc(), body),
        [first, second],
        "both placements copy the body"
    );
    // The second copy is drawn, where Place stood it: the identity of
    // the body's own space, where the body stood. With the body hidden
    // again, a pick there reaches the copy.
    let hidden = session.perform(SessionOp::SetBodyOn {
        node: body,
        on: false,
    });
    assert!(hidden.refusal.is_none(), "{:?}", hidden.refusal);
    session.pump();
    let doc = session.committed_doc();
    assert!(
        viewer::world::is_drawn(doc, second),
        "the second copy is drawn"
    );
    let (landed, ev) = session.landed_pair().expect("landed");
    assert_eq!(
        pncad::document::SolvedPoses::of(landed, ev).relative(second),
        Some(pncad::document::Frame::IDENTITY),
        "it stands where the body stands"
    );
    let Selection::Face(face) = face_at(&session, [0.0, 0.0]) else {
        unreachable!("a face pick")
    };
    assert_eq!(face.node, second, "a pick where the body stood reaches it");

    // A pick on a copy still opens that copy's placement.
    tools.close();
    tools.open(ToolKind::Mate);
    let on_copy = face_at(&session, [0.1, 0.0]);
    assert!(fed(&session, &mut tools, on_copy).is_none(), "no Place");
    assert_eq!(
        tools
            .mate()
            .and_then(|editor| editor.door())
            .map(|door| door.placement),
        Some(first),
        "a pick on the moved copy edits its placement"
    );
}

/// The small box placed and the editor open on its copy, the wide box
/// beside it in a space of its own: the session, the editor and the
/// copy's placement.
fn small_placed_beside_wide(label: &str, tol: Tol) -> (DocSession, MateTool, RecipeNodeId) {
    let mut session = session(label, tol);
    let small = common::xy_box_in(&mut session, SMALL);
    let _wide = common::xy_box_in(&mut session, WIDE);
    session.pump();
    let (tool, copy) = common::place_and_edit(&mut session, small.into());
    (session, tool, copy)
}

/// **The copy's top flushed with the wide box's**, picked change by
/// change and each change offered: the first pick holds a draft whose
/// `to` seat is empty, and the second one the door refuses, the wide box
/// standing in another space. Answers the editor holding that refusal.
fn flushed_across_spaces(session: &DocSession, mut tool: MateTool) -> MateTool {
    let Selection::Face(top) = face_at(session, [0.0, 0.0]) else {
        unreachable!("a face pick")
    };
    let Selection::Face(wide_top) = face_at(session, WIDE_TOP) else {
        unreachable!("a face pick")
    };
    assert!(tool.pick(session.doc(), top).is_none());
    assert!(
        offered(session, &mut tool).is_none(),
        "half a mate lands nothing"
    );
    assert!(
        matches!(tool.pending(), Some(Refusal::Draft(_))),
        "a mate with an empty seat is held, said: {:?}",
        tool.pending()
    );
    tool.set_sense(0, false);
    assert!(offered(session, &mut tool).is_none());
    assert!(tool.pick(session.doc(), wide_top).is_none());
    assert!(offered(session, &mut tool).is_none(), "the door refuses");
    assert!(
        matches!(
            tool.pending(),
            Some(Refusal::Edit(error))
                if matches!(**error, pncad::document::EditError::SpaceMismatch { .. })
        ),
        "the mate reads another space, said: {:?}",
        tool.pending()
    );
    tool
}

/// **A `to` picked in another space is held, then "Into <space>" and
/// the mate land as two steps** (Ev, `[ev]` PR 6600): the flush the door
/// refuses lands nothing and is held; "Into" the wide box's space is a
/// target change of its own, which lands alone, the copy's bundle as it
/// was; the held flush is then made over it and lands as the second
/// step, one `SetBundle` holding it.
#[test]
fn a_held_cross_space_mate_lands_after_into_its_space_in_two_steps() {
    let tol = Tol::witness();
    let (mut session, tool, copy) = small_placed_beside_wide("place-live-held", tol);
    let before = session.committed_doc().clone();
    let states = session.history().len();
    let tool = flushed_across_spaces(&session, tool);
    assert_eq!(session.history().len(), states, "nothing held lands");
    assert!(
        session.committed_doc().bit_eq(&before),
        "the document is untouched"
    );
    let (doc, ev) = session.landed_pair().expect("landed");
    let off = tool.off_target(doc, ev);
    let [(0, wide_space)] = off[..] else {
        panic!("the flush's row reads the wide box's space: {off:?}");
    };
    let own = doc.node_space(copy).expect("the copy is placed");
    assert_ne!(wide_space, own, "premise: two spaces");

    let mut tools = Tools::new();
    tools.open(ToolKind::Mate);
    *tools.mate_mut().expect("open") = tool;
    tools.mate_mut().expect("open").into_space(wide_space);
    let into = offered(&session, tools.mate_mut().expect("open")).expect("the target passes");
    assert!(
        matches!(&into, SessionOp::SetBundle { placement, target, constraints, base }
            if *placement == copy
                && *target == viewer::session::PlaceTarget::space(wide_space)
                && *constraints == base.constraints),
        "the target alone, the bundle as it was: {into:?}"
    );
    assert!(landed(&mut session, &mut tools, into), "the target lands");
    assert_eq!(session.history().len(), states + 1, "as one undo step");
    assert_eq!(
        session.committed_doc().node_space(copy),
        Some(wide_space),
        "the copy is into the wide box's space"
    );
    let flush = offered(&session, tools.mate_mut().expect("open")).expect("the held flush passes");
    assert!(
        matches!(&flush, SessionOp::SetBundle { placement, constraints, .. }
            if *placement == copy && constraints.iter().filter(|c| c.mate().is_some()).count() == 1),
        "one SetBundle holding the flush the door refused: {flush:?}"
    );
    assert!(landed(&mut session, &mut tools, flush), "the flush lands");
    assert_eq!(session.history().len(), states + 2, "the second undo step");
    let editor = tools.mate().expect("the editor stays open");
    assert!(editor.pending().is_none(), "nothing held once it passes");
    assert_eq!(
        editor.rows().len(),
        1,
        "the editor shows the bundle it wrote"
    );
    assert_eq!(
        editor.door().map(|door| door.base.constraints.clone()),
        Some(authored(session.committed_doc(), copy)),
        "loaded from the bundle as it now is"
    );
}

/// **Each passing change is exactly one undo step**: two values typed
/// one after the other on the copy's slides are two `SetBundle`s and two
/// history states, and one undo takes back the second alone.
#[test]
fn each_passing_change_is_one_undo_step() {
    let tol = Tol::witness();
    let (mut session, tool, copy) = small_placed_beside_wide("place-live-steps", tol);
    let mut tools = Tools::new();
    tools.open(ToolKind::Mate);
    *tools.mate_mut().expect("open") = tool;
    let slide = |axis| Coord::slide(axis);
    let at = |session: &DocSession, axis| slide_of(session.committed_doc(), copy, axis);
    assert_eq!(
        at(&session, TargetAxis::X),
        0.0,
        "premise: Place wrote zeros"
    );
    let states = session.history().len();
    for (step, axis) in [(1, TargetAxis::X), (2, TargetAxis::Y)] {
        let editor = tools.mate_mut().expect("open");
        editor.type_value(slide(axis), common::len(0.01));
        let op = offered(&session, editor).expect("a typed value passes");
        assert!(
            offered(&session, tools.mate_mut().expect("open")).is_none(),
            "one change, one offer"
        );
        assert!(landed(&mut session, &mut tools, op), "it lands");
        assert_eq!(
            session.history().len(),
            states + step,
            "change {step} is one undo step"
        );
    }
    assert_eq!(
        at(&session, TargetAxis::X),
        0.01,
        "the first change wrote x"
    );
    assert_eq!(at(&session, TargetAxis::Y), 0.01, "the second wrote y");
    let outcome = session.perform(SessionOp::Undo);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    assert_eq!(
        at(&session, TargetAxis::Y),
        0.0,
        "undo takes the second back"
    );
    assert_eq!(at(&session, TargetAxis::X), 0.01, "and leaves the first");
}

/// **Discard drops the held draft**: the editor shows the bundle as the
/// document holds it, nothing pending and nothing to offer, and the
/// document never saw the draft.
#[test]
fn discard_drops_the_held_draft() {
    let tol = Tol::witness();
    let (session, tool, copy) = small_placed_beside_wide("place-live-discard", tol);
    let states = session.history().len();
    let mut tool = flushed_across_spaces(&session, tool);
    assert!(
        !tool.rows().is_empty(),
        "premise: the draft holds a mate row"
    );
    tool.discard(session.committed_doc());
    assert!(tool.pending().is_none(), "nothing held");
    assert!(tool.rows().is_empty(), "the bundle holds no mate");
    assert_eq!(
        tool.door().map(|door| door.base.constraints.clone()),
        Some(authored(session.committed_doc(), copy)),
        "the editor shows the bundle as the document holds it"
    );
    assert!(offered(&session, &mut tool).is_none(), "nothing to offer");
    assert_eq!(session.history().len(), states, "the document never saw it");
    assert!(
        matches!(session.committed_doc().node(copy), Some(Node::Place { constraints, .. })
            if constraints.iter().all(|c| c.mate().is_none())),
        "no mate reached the copy"
    );
}

/// **Undo takes back a held draft first**, as the profile editor's: a
/// committed change stays, the held flush goes, and the tools answer the
/// placement whose draft went (`Tools::undo_pending`, which the chrome
/// spends an Undo on instead of stepping the history).
#[test]
fn an_undo_spends_the_held_draft_not_the_committed_change() {
    let tol = Tol::witness();
    let (mut session, tool, copy) = small_placed_beside_wide("place-live-undo", tol);
    let mut tools = Tools::new();
    tools.open(ToolKind::Mate);
    *tools.mate_mut().expect("open") = tool;
    let editor = tools.mate_mut().expect("open");
    editor.type_value(Coord::slide(TargetAxis::X), common::len(0.005));
    let op = offered(&session, editor).expect("a typed value passes");
    assert!(landed(&mut session, &mut tools, op), "it lands");
    let states = session.history().len();
    let held = flushed_across_spaces(&session, tools.mate().expect("open").clone());
    *tools.mate_mut().expect("open") = held;
    assert_eq!(
        tools.undo_pending(session.committed_doc()),
        Some(copy),
        "the undo is spent on the held draft"
    );
    let editor = tools.mate().expect("open");
    assert!(
        editor.rows().is_empty() && editor.pending().is_none(),
        "the flush went"
    );
    assert!(!editor.holds_changes(), "nothing left to take back");
    assert_eq!(
        tools.undo_pending(session.committed_doc()),
        None,
        "the next undo is the history's"
    );
    assert_eq!(session.history().len(), states, "the history did not step");
    assert_eq!(
        slide_of(session.committed_doc(), copy, TargetAxis::X),
        0.005,
        "the committed change stays"
    );
}

/// **A change made while the last one lands is made over the bundle it
/// writes**, as the profile editor defers its clicks: the y typed between
/// the x's offer and its landing is neither lost nor landed with it, and
/// is the next change offered.
#[test]
fn a_change_made_while_the_last_lands_is_made_over_it() {
    let tol = Tol::witness();
    let (mut session, tool, copy) = small_placed_beside_wide("place-live-deferred", tol);
    let mut tools = Tools::new();
    tools.open(ToolKind::Mate);
    *tools.mate_mut().expect("open") = tool;
    let editor = tools.mate_mut().expect("open");
    editor.type_value(Coord::slide(TargetAxis::X), common::len(0.01));
    let first = offered(&session, editor).expect("x passes");
    editor.type_value(Coord::slide(TargetAxis::Y), common::len(0.02));
    assert!(editor.landing().is_some(), "x is still landing");
    assert!(landed(&mut session, &mut tools, first), "x lands");
    assert_eq!(slide_of(session.committed_doc(), copy, TargetAxis::Y), 0.0);
    let editor = tools.mate_mut().expect("open");
    assert!(editor.holds_changes(), "y is held over what x wrote");
    let second = offered(&session, editor).expect("y passes, over x");
    assert!(landed(&mut session, &mut tools, second), "y lands");
    let doc = session.committed_doc();
    assert_eq!(slide_of(doc, copy, TargetAxis::X), 0.01, "x as it landed");
    assert_eq!(slide_of(doc, copy, TargetAxis::Y), 0.02, "y over it");
}

/// **An undo ahead of a change formed before it refuses that change
/// stale** (`Refusal::BundleStale`): the `SetBundle` was drafted over the
/// bundle the undo took away, and is not written over the one it brought
/// back; the editor stops waiting on it.
#[test]
fn a_change_drafted_before_an_undo_is_refused_stale() {
    let tol = Tol::witness();
    let (mut session, tool, _copy) = small_placed_beside_wide("place-live-stale", tol);
    let mut tools = Tools::new();
    tools.open(ToolKind::Mate);
    *tools.mate_mut().expect("open") = tool;
    let editor = tools.mate_mut().expect("open");
    editor.type_value(Coord::slide(TargetAxis::X), common::len(0.01));
    let first = offered(&session, editor).expect("x passes");
    assert!(landed(&mut session, &mut tools, first));
    let editor = tools.mate_mut().expect("open");
    editor.type_value(Coord::slide(TargetAxis::Y), common::len(0.01));
    let second = offered(&session, editor).expect("y passes");
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    let outcome = session.perform(second.clone());
    assert!(
        matches!(outcome.refusal, Some(Refusal::BundleStale { .. })),
        "{:?}",
        outcome.refusal
    );
    tools.answered(&second, None, false, session.committed_doc());
    assert!(tools.mate().expect("open").landing().is_none());
}

/// **The editor's draft is judged against the committed document while
/// its own change still evaluates**: opened on the bundle a change just
/// committed, before the evaluation answering it lands, reconcile keeps
/// the draft — the landed pair answers the bundle before, and is not
/// what the draft was loaded from.
#[test]
fn a_draft_opened_while_a_run_is_held_is_kept_by_reconcile() {
    let tol = Tol::witness();
    let (mut session, mut tool, copy) = small_placed_beside_wide("place-live-held-run", tol);
    tool.type_value(Coord::slide(TargetAxis::X), common::len(0.01));
    let op = offered(&session, &mut tool).expect("x passes");
    assert!(session.perform(op).refusal.is_none(), "x lands");
    assert!(session.busy(), "premise: its run has not landed");
    let mut editor = MateTool::edit(session.committed_doc(), copy).expect("a placement");
    editor.add_mate(pncad::document::VarKind::Plane);
    let landed_pair = session.landed_pair().expect("an older run is landed");
    assert_ne!(
        authored(landed_pair.0, copy),
        authored(session.committed_doc(), copy),
        "premise: the landed pair answers the bundle before"
    );
    let events = editor.reconcile(session.committed_doc(), landed_pair);
    assert!(events.is_empty(), "nothing reloads: {events:?}");
    assert_eq!(editor.rows().len(), 1, "the draft is kept");
}
