//! **What an accepted edit did that the user did not ask for, carried
//! to the chrome** (`crates/editor-core/REFERENCES.md` DM7).
//!
//! The edit door reports a stranded payload name and a stranded
//! appearance key on `Applied::maintenance`, and a mate that joined
//! two groups reports the offset it cleared; a value edit reports nothing, because a
//! profile's names are its minted steps and no value moves one. The log
//! keeps only the edits, because replay re-derives every row, so the
//! session's outcome is the one road the rows have to a user.
//! Each row here drives a real session through one of the viewer's
//! edit doors, asserts the rows on `OpOutcome::maintenance`, and reads
//! the status line the frame would compose from that outcome through
//! `frame::outcome_notices` — the call `app` makes per operation.
//!
//! Nothing here evaluates: a report is a statement about the edit, and
//! the session's inline evaluator runs only inside `pump`.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use test_utils::refusal::tagged;

use editor_core::{Attr, Rgba8};
use pncad::document::{
    Datum, Dimension, Doc, DocEdit, DocParam, LoopProgram, Maintenance, Node, ParamName,
    ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, SlotId, SpokenName, SpokenNode,
    StepArg,
};
use pncad::geom_core::Tol;
use pncad::prelude::{EntityKind, ProfileEdgeRef, RoleSeg, StableName};
use pncad::select::{PieceRole, StepId};
use viewer::frame::{self, RankedVerdict};
use viewer::session::{DocSession, OpOutcome, SessionOp};

/// The face name the extrude `node` mints for its lateral wall at
/// canonical `(loop, segment)`, spelled by the piece that position is
/// under `doc`'s current values.
fn wall(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    loop_index: usize,
    segment: usize,
) -> StableName {
    let Some(Node::Extrude { profile, .. }) = doc.node(node) else {
        panic!("node {} is an extrude", test_utils::refusal::tag(node.0));
    };
    let Some(Node::Profile(program)) = doc.node(*profile) else {
        panic!("an extrude's operand is a profile");
    };
    let piece = program
        .pieces(&doc.param_env::<f64>(), Tol::witness())
        .expect("the profile replays")
        .edge(loop_index, segment)
        .expect("the position is the profile's");
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Lateral(piece.into())],
    }
}

/// A unit-high extrude of `loops` on the xy frame; `(doc, profile,
/// extrude)`.
fn extruded(
    doc: &Doc<ProfileProgram>,
    loops: Vec<LoopProgram>,
) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let tol = Tol::witness();
    let (doc, plane) = common::inserted(doc, common::xy_frame(), tol);
    let (doc, profile) = common::inserted(
        &doc,
        Node::Profile(ProfileProgram {
            plane,
            loops,
            ids: Vec::new(),
        }),
        tol,
    );
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(1.0),
        },
        tol,
    );
    (doc, profile, extrude)
}

/// A unit square at `(x0, 0)`, extruded; `(doc, extrude)`.
fn block(doc: &Doc<ProfileProgram>, x0: f64) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let (doc, _, extrude) = extruded(doc, vec![common::rectangle_loop([x0, 0.0], 1.0, 1.0)]);
    (doc, extrude)
}

/// A derived frame on `at` carrying `face` — a payload carrier of one
/// name.
fn frame_on(
    doc: &Doc<ProfileProgram>,
    at: RecipeNodeId,
    face: StableName,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    common::inserted(
        doc,
        Node::Datum(Datum::FaceFrame {
            at,
            face,
            spin: common::ang(0.0),
        }),
        Tol::witness(),
    )
}

/// **The status line the frame composes from `outcome`**, as `app`
/// composes it: the outcome's notices, ranked by `frame_status` over
/// the one operation that produced it.
fn line_after(outcome: &OpOutcome, op: SessionOp) -> String {
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let notices: Vec<frame::Message> = frame::outcome_notices(outcome).collect();
    match frame::frame_status(&notices, &[op], None) {
        RankedVerdict::Show(line) => line.text().to_owned(),
        other => panic!("an accepted edit with maintenance shows its rows, got {other:?}"),
    }
}

/// **Each row's own sentence, one notice apiece**, is what the line
/// carries — split back on the notice boundary, so a row the line
/// dropped or merged fails here by name.
fn assert_line_words(line: &str, rows: &[Maintenance]) {
    let words: Vec<String> = rows.iter().map(ToString::to_string).collect();
    assert_eq!(
        line.split(frame::NOTICE_SEPARATOR).collect::<Vec<_>>(),
        words,
        "the line carries each maintenance row in its own words, in order"
    );
}

/// **A delete that strands a payload name reports it, and the line
/// says so.** A frame on the kept block names a wall of the other
/// one; deleting the other leaves the frame holding a name whose
/// minting node is gone. The delete is legal (a name is not an edge),
/// and before this row the session answered it with nothing.
#[test]
fn a_delete_that_strands_a_payload_name_reaches_the_line() {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("maint-delete-strand", Tol::witness());
    let (doc, kept) = block(&doc, 0.0);
    let (doc, victim) = block(&doc, 4.0);
    let named = wall(&doc, victim, 0, 0);
    let (doc, carrier) = frame_on(&doc, kept, named.clone());

    let expected = vec![Maintenance::Strand {
        node: doc.spoken(carrier),
        name: doc.spoken_name(&named),
    }];
    let mut session = DocSession::inline(doc, Tol::witness());
    let op = SessionOp::DeleteNode { node: victim };
    let outcome = session.perform(op.clone());
    assert_eq!(outcome.maintenance, expected);
    assert_line_words(&line_after(&outcome, op), &expected);
    assert!(
        session.committed_doc().node(carrier).is_some(),
        "the report is a report: the carrier is untouched"
    );
}

/// **Every worded maintenance row rides beside a refusal**, each by its
/// own arm: none can show that anything will say it again
/// (`frame::maintenance_notice` gives each arm's reason).
#[test]
fn every_maintenance_row_rides_beside_a_refusal() {
    use viewer::session::{Refusal, Step};

    let face = |node: u64| {
        SpokenName::absent(StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(tagged(node)),
            path: vec![],
        })
    };
    let rows = [
        Maintenance::Strand {
            node: SpokenNode::absent(RecipeNodeId(tagged(3))),
            name: face(7),
        },
        Maintenance::StrandedAppearance { name: face(8) },
    ];
    let notices: Vec<frame::Message> = rows
        .iter()
        .map(|row| frame::maintenance_notice(row).expect("every arm here is worded"))
        .collect();
    assert_eq!(
        notices
            .iter()
            .map(frame::Message::retold)
            .collect::<Vec<_>>(),
        [frame::Retold::Never; 2],
        "strand, stranded appearance"
    );

    let refusal = Refusal::NothingToDo {
        direction: Step::Undo,
    };
    let RankedVerdict::Show(line) =
        frame::frame_status(&notices, &[SessionOp::Undo], Some(&refusal))
    else {
        panic!("a refusing frame shows its refusal");
    };
    assert_eq!(
        line.text(),
        "nothing to undo \u{2022} node 000000000003 carries a name for the face of node \
         000000000007; this edit removed what it denoted (a node or profile step it names), so \
         the name resolves to nothing until it is rebound \u{2022} the appearance store holds \
         an attachment under a name for the face of node 000000000008; this edit removed \
         what it denoted (a node or profile step it names), so the name resolves to nothing \
         until it is rebound or cleared"
    );
}

/// **A delete that strands an appearance key reports it** — the
/// second carrier, with no node of its own.
#[test]
fn a_delete_that_strands_an_appearance_key_reaches_the_line() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("maint-delete-paint", tol);
    let (doc, kept) = block(&doc, 0.0);
    let (doc, victim) = block(&doc, 4.0);
    let painted = wall(&doc, victim, 0, 2);
    let paint = |doc: &Doc<ProfileProgram>, name: &StableName| {
        common::edited(
            doc,
            DocEdit::SetAppearance {
                name: name.clone(),
                attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
            },
            tol,
        )
        .0
    };
    let doc = paint(&doc, &painted);
    let doc = paint(&doc, &wall(&doc, kept, 0, 0));

    let expected = vec![Maintenance::StrandedAppearance {
        name: doc.spoken_name(&painted),
    }];
    let mut session = DocSession::inline(doc, tol);
    let op = SessionOp::DeleteNode { node: victim };
    let outcome = session.perform(op.clone());
    assert_eq!(
        outcome.maintenance, expected,
        "the deleted node's key, and not the kept block's"
    );
    assert_line_words(&line_after(&outcome, op), &expected);
}

/// A triangle `(0,0) → (2,0) → (1,1)`, counterclockwise, extruded, with
/// a frame on each of its first two walls; `(doc, profile, extrude)`.
fn framed_triangle(label: &str) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let triangle = LoopProgram::Chain(vec![
        ProgramStep::At(common::len2([0.0, 0.0])),
        ProgramStep::LineTo(ProgramTarget::Point(common::len2([2.0, 0.0]))),
        ProgramStep::LineTo(ProgramTarget::Point(common::len2([1.0, 1.0]))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let doc: Doc<ProfileProgram> = Doc::empty_derived(label, Tol::witness());
    let (doc, profile, extrude) = extruded(&doc, vec![triangle]);
    let (doc, _) = frame_on(&doc, extrude, wall(&doc, extrude, 0, 0));
    let (doc, _) = frame_on(&doc, extrude, wall(&doc, extrude, 0, 1));
    (doc, profile, extrude)
}

/// The apex's y slot.
fn apex_y() -> SlotId {
    SlotId::Profile {
        loop_: 0,
        step: 2,
        arg: StepArg::TargetY,
    }
}

/// **The line an edit that moved no name composes**: nothing, so the
/// batch's own verdict — clear — stands.
fn assert_quiet(outcome: &OpOutcome, op: SessionOp) {
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.maintenance, Vec::new());
    assert_eq!(
        frame::frame_status(
            &frame::outcome_notices(outcome).collect::<Vec<_>>(),
            &[op],
            None
        ),
        RankedVerdict::Clear,
        "an accepted edit with nothing to report clears the line"
    );
}

/// **A value edit that flips a loop's sense moves no name, and the
/// line says nothing.** The apex moved below the base winds the same
/// three steps clockwise; each wall is still named by the step that
/// draws it, so both frames' names still denote their walls and there
/// is nothing to report. Written through the panel's slot door.
#[test]
fn a_slot_edit_that_flips_the_sense_reports_nothing() {
    let (doc, profile, extrude) = framed_triangle("maint-slot-flip");
    let base = wall(&doc, extrude, 0, 0);
    let mut session = DocSession::inline(doc, Tol::witness());
    let op = SessionOp::SetSlot {
        node: profile,
        slot: apex_y(),
        value: viewer::props::SlotValue::Continuous(-1.0),
    };
    let outcome = session.perform(op.clone());
    assert_quiet(&outcome, op);
    assert_eq!(
        wall(session.committed_doc(), extrude, 0, 2),
        base,
        "the base is canonical wall 2 now, under the name it had"
    );
}

/// **The same flip, dragged**: the preview and the release both report
/// nothing — the GUI user dragging a parameter past a sense change
/// moves no name.
#[test]
fn a_dragged_flip_reports_nothing_at_the_release_or_before() {
    let (doc, profile, _) = framed_triangle("maint-drag-flip");
    let mut session = DocSession::inline(doc, Tol::witness());
    let slot = apex_y();
    let begun = session.perform(SessionOp::BeginGesture {
        node: profile,
        slot,
    });
    assert!(begun.refusal.is_none(), "{:?}", begun.refusal);
    let previewed = session.perform(SessionOp::PreviewGesture {
        node: profile,
        slot,
        value: -1.0,
    });
    assert!(previewed.refusal.is_none(), "{:?}", previewed.refusal);
    assert_eq!(previewed.previewed.len(), 1, "the premise: a preview ran");
    assert_eq!(previewed.maintenance, Vec::new());
    let op = SessionOp::CommitGesture {
        node: profile,
        slot,
    };
    let landed = session.perform(op.clone());
    assert_eq!(landed.committed.len(), 1, "the drag landed one edit");
    assert_quiet(&landed, op);
}

/// **The path editor's door moves no name either** when it keeps
/// every step: `EditProfile` lands its program as one `SetProgram`,
/// and a program that keeps every step and still draws every piece
/// strands nothing, whichever way its numbers turn it.
#[test]
fn a_profile_edit_that_flips_the_sense_reports_nothing() {
    let (doc, profile, _) = framed_triangle("maint-profile-flip");
    let Some(Node::Profile(base)) = doc.node(profile).cloned() else {
        panic!("the fixture's profile")
    };
    let loops = vec![LoopProgram::Chain(vec![
        ProgramStep::At(common::len2([0.0, 0.0])),
        ProgramStep::LineTo(ProgramTarget::Point(common::len2([2.0, 0.0]))),
        ProgramStep::LineTo(ProgramTarget::Point(common::len2([1.0, -1.0]))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])];
    let mut session = DocSession::inline(doc, Tol::witness());
    let op = SessionOp::EditProfile {
        node: profile,
        ids: base.kept_in_place(),
        base,
        loops,
    };
    let outcome = session.perform(op.clone());
    assert!(
        matches!(outcome.committed.as_slice(), [DocEdit::SetProgram { .. }]),
        "one whole-program edit: {:?}",
        outcome.committed
    );
    assert_quiet(&outcome, op);
}

/// A square `(0, 0) → (2, 2)` whose corner at `(2, 0)` is sharp, or
/// filleted at radius `0.5` when `filleted`: the fillet's arrival
/// `at` stands at its tangent point, and the `line` after it runs on
/// to `(2, 2)` along the fillet's run out. Each sharp step has a step
/// of the same verb in the filleted program.
fn corner(filleted: bool) -> LoopProgram {
    let toward = |dx: f64, dy: f64| ProgramStep::Toward {
        dx: common::scl(dx),
        dy: common::scl(dy),
    };
    let mut steps = vec![ProgramStep::At(common::len2([0.0, 0.0])), toward(1.0, 0.0)];
    if filleted {
        steps.extend([
            ProgramStep::Line(common::len(1.0)),
            ProgramStep::Fillet(common::len(0.5)),
            toward(0.0, 1.0),
            ProgramStep::At(common::len2([2.0, 0.5])),
            ProgramStep::Line(common::len(1.5)),
        ]);
    } else {
        steps.extend([
            ProgramStep::Line(common::len(2.0)),
            toward(0.0, 1.0),
            ProgramStep::Line(common::len(2.0)),
        ]);
    }
    steps.extend([
        ProgramStep::LineTo(ProgramTarget::Point(common::len2([0.0, 2.0]))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    LoopProgram::Chain(steps)
}

/// **A fillet inserted before a framed leg is counted before Apply and
/// reported after it.** The path editor keeps every step and inserts
/// a fillet at the corner below the right wall: the fillet's run out
/// takes that wall's segment, so the kept leg the frame names is no
/// longer drawn. The door's own pre-click report — what the Apply
/// button counts — names the frame's strand, and the landed edit
/// reports the same row to the line.
#[test]
fn a_fillet_inserted_before_a_framed_leg_is_counted_and_reported() {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("maint-profile-shadow", Tol::witness());
    let (doc, profile, extrude) = extruded(&doc, vec![corner(false)]);
    let right = wall(&doc, extrude, 0, 1);
    let (doc, carrier) = frame_on(&doc, extrude, right.clone());
    let Some(Node::Profile(base)) = doc.node(profile).cloned() else {
        panic!("the fixture's profile")
    };
    let mut ids = base.kept_in_place();
    ids[0].insert(3, None);
    ids[0].insert(5, None);
    let expected = vec![Maintenance::Strand {
        node: doc.spoken(carrier),
        name: doc.spoken_name(&right),
    }];
    let mut session = DocSession::inline(doc, Tol::witness());
    assert_eq!(
        session
            .edit_profile_report(profile, &base, vec![corner(true)], ids.clone())
            .expect("the reshaping is legal"),
        expected,
        "the count Apply shows before the click"
    );
    let op = SessionOp::EditProfile {
        node: profile,
        base,
        loops: vec![corner(true)],
        ids,
    };
    let outcome = session.perform(op.clone());
    assert_eq!(outcome.maintenance, expected);
    assert_line_words(&line_after(&outcome, op), &expected);
}

/// **A parameter edit through a state that draws nothing reports
/// nothing.** A driven circular hole at radius zero encloses nothing,
/// so the profile does not validate and nothing is named — the frame
/// on the hole's wall resolves to nothing until the radius comes back,
/// and then to that wall again. No name was moved, so none is reported
/// — written through the parameter panel's door.
#[test]
fn a_parameter_edit_through_a_degenerate_hole_reports_nothing() {
    let tol = Tol::witness();
    let hole_r = ParamName::from_static("hole_r");
    let doc = common::declared(
        "maint-param-strand",
        &hole_r,
        DocParam::continuous(Dimension::Length, 0.3),
        tol,
    );
    let square = common::rectangle_loop([0.0, 0.0], 2.0, 2.0);
    let hole = LoopProgram::Circle {
        centre: [common::len(1.0), common::len(1.0)],
        radius: pncad::document::Expr::param(hole_r.clone(), Dimension::Length),
    };
    let (doc, _, extrude) = extruded(&doc, vec![square, hole]);
    let (doc, _) = frame_on(&doc, extrude, wall(&doc, extrude, 1, 0));

    let mut session = DocSession::inline(doc, tol);
    let op = SessionOp::SetParam {
        name: hole_r,
        value: viewer::props::SlotValue::Continuous(0.0),
    };
    let outcome = session.perform(op.clone());
    assert_quiet(&outcome, op);
}

/// **An edit that moves nothing says nothing**: the ordinary value
/// edit — the apex raised, the sense kept — leaves the line to the
/// batch's own verdict.
#[test]
fn an_edit_that_renumbers_nothing_leaves_the_line_to_its_verdict() {
    let (doc, profile, _) = framed_triangle("maint-quiet");
    let mut session = DocSession::inline(doc, Tol::witness());
    let op = SessionOp::SetSlot {
        node: profile,
        slot: apex_y(),
        value: viewer::props::SlotValue::Continuous(2.0),
    };
    let outcome = session.perform(op.clone());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1, "the premise: the edit landed");
    assert_eq!(outcome.maintenance, Vec::new());
    assert_eq!(
        frame::frame_status(
            &frame::outcome_notices(&outcome).collect::<Vec<_>>(),
            &[op],
            None
        ),
        RankedVerdict::Clear,
        "an accepted edit with nothing to report clears the line"
    );
}

/// **The mate door's offset clear rides the outcome and is not worded
/// on the line** — the one arm `frame::maintenance_notice` holds
/// silent, and the reason is on that function. Every other arm is its
/// own sentence.
#[test]
fn an_offset_clear_is_carried_but_not_worded() {
    let minter = RecipeNodeId(tagged(7));
    let act = Maintenance::OffsetCleared {
        instance: SpokenNode::absent(RecipeNodeId(tagged(5))),
        offset: pncad::document::Placement::IDENTITY,
    };
    assert_eq!(frame::maintenance_notice(&act), None);
    let strand = Maintenance::Strand {
        node: SpokenNode::absent(RecipeNodeId(tagged(3))),
        name: SpokenName::absent(StableName {
            kind: EntityKind::Face,
            node: minter,
            path: vec![RoleSeg::Lateral(
                ProfileEdgeRef::Piece {
                    step: StepId(tagged(1)),
                    role: PieceRole::Leg,
                }
                .into(),
            )],
        }),
    };
    assert_eq!(
        frame::maintenance_notice(&strand).map(|notice| notice.text().to_owned()),
        Some(strand.to_string())
    );
}
