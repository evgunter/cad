//! **What an accepted edit did that the user did not ask for, carried
//! to the chrome** (`crates/editor-core/REFERENCES.md` DM7).
//!
//! The edit door reports a stranded payload name, a stranded
//! appearance key, a declaration left with no consumer and a name
//! rewritten in place on `Applied::maintenance`. The log keeps only
//! the cluster acts, because replay re-derives the rest, so the
//! session's outcome is the one road the other rows have to a user.
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

use editor_core::{Attr, Rgba8};
use pncad::document::{
    Datum, Dimension, Doc, DocEdit, DocParam, LoopProgram, Maintenance, Node, ParamName,
    ProfileProgram, ProgramStep, ProgramTarget, RETIRED_FLOOR, RecipeNodeId, SitedRef, SlotId,
    StepArg, cascade_delete_order,
};
use pncad::geom_core::Tol;
use pncad::prelude::{EntityKind, ProfileEdgeRef, RoleSeg, StableName};
use viewer::frame::{self, StatusUpdate};
use viewer::session::{DocSession, OpOutcome, SessionOp};

/// The face name `node` mints for its lateral wall `(loop, segment)`.
fn wall(node: RecipeNodeId, loop_index: u32, segment: u32) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Lateral(ProfileEdgeRef {
            loop_index,
            segment,
        })],
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
    let (doc, profile) =
        common::inserted(&doc, Node::Profile(ProfileProgram { plane, loops }), tol);
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
        StatusUpdate::Show(line) => line.text().to_owned(),
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
    let named = wall(victim, 0, 0);
    let (doc, carrier) = frame_on(&doc, kept, named.clone());

    let mut session = DocSession::inline(doc, Tol::witness());
    let op = SessionOp::DeleteNode { node: victim };
    let outcome = session.perform(op.clone());
    let expected = vec![Maintenance::Strand {
        node: carrier,
        name: named,
    }];
    assert_eq!(outcome.maintenance, expected);
    assert_line_words(&line_after(&outcome, op), &expected);
    assert!(
        session.committed_doc().node(carrier).is_some(),
        "the report is a report: the carrier is untouched"
    );

    // **And the tree says it again**, on every run the strand stands:
    // the carrier fails to evaluate on the name and its row carries
    // the fault. This is why a strand may sit under a refusal
    // (`frame::maintenance_notice`, `Retold::Again`) — the line is not
    // its only telling.
    session.pump();
    let (landed, eval) = session.landed_pair().expect("the delete lands");
    let row = viewer::tree::rows(landed, Some(eval))
        .into_iter()
        .find(|row| row.id == carrier)
        .expect("the carrier has a row");
    assert_eq!(
        row.status.message(),
        Some(
            "node 6 failed: the derived frame's face name failed to resolve: the face name's \
             minting node 5 is no longer in the document (node 5 was deleted) — the repair is \
             an explicit rebind"
        )
    );
    assert_eq!(
        (carrier.0, victim.0),
        (6, 5),
        "the premise the literal names"
    );
}

/// **A maintenance row rides beside a refusal exactly when nothing
/// else will ever say it** — the rule `frame::frame_status` states,
/// pinned both ways over every worded arm.
///
/// A strand's carrier fails on every run and its tree row says why
/// (the row above), so it stays under the refusal. An orphaned
/// declaration and a rebound name evaluate cleanly by design, and a
/// stranded appearance key's loss is drawn nowhere in this viewer, so
/// the line is their only telling and they ride beside it.
#[test]
fn a_maintenance_row_rides_beside_a_refusal_when_nothing_else_will_say_it() {
    use viewer::session::{Refusal, Step};

    let rows = [
        Maintenance::Strand {
            node: RecipeNodeId(3),
            name: wall(RecipeNodeId(7), 0, 0),
        },
        Maintenance::StrandedAppearance {
            name: wall(RecipeNodeId(7), 0, 2),
        },
        Maintenance::OrphanedDeclare {
            declare: RecipeNodeId(5),
        },
        Maintenance::Rebound {
            from: wall(RecipeNodeId(9), 0, 0),
            to: wall(RecipeNodeId(9), 0, 1),
        },
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
        [
            frame::Retold::Again,
            frame::Retold::Never,
            frame::Retold::Never,
            frame::Retold::Never,
        ],
        "strand, stranded appearance, orphaned declaration, rebound"
    );

    let refusal = Refusal::NothingToDo {
        direction: Step::Undo,
    };
    let StatusUpdate::Show(line) =
        frame::frame_status(&notices, &[SessionOp::Undo], Some(&refusal))
    else {
        panic!("a refusing frame shows its refusal");
    };
    assert_eq!(
        line.text(),
        "nothing to undo \u{2022} the appearance store holds an attachment under a face name \
         minted by node 7; this edit removed what it denoted (its minting node, or the profile \
         segment it named), so the name resolves to nothing until it is rebound or cleared \
         \u{2022} node 5 declares contacts and this edit deleted the last node that consumed \
         it, so no node consumes the declaration until a boolean or union names it again \
         \u{2022} a face name minted by node 9 was rewritten in place to the face name minted \
         by node 9 that draws the same step's segment under the reshaped profile program, so \
         every carrier of the name still denotes what it did"
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
    let painted = wall(victim, 0, 2);
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
    let doc = paint(&doc, &wall(kept, 0, 0));

    let mut session = DocSession::inline(doc, tol);
    let op = SessionOp::DeleteNode { node: victim };
    let outcome = session.perform(op.clone());
    let expected = vec![Maintenance::StrandedAppearance { name: painted }];
    assert_eq!(
        outcome.maintenance, expected,
        "the deleted node's key, and not the kept block's"
    );
    assert_line_words(&line_after(&outcome, op), &expected);
}

/// Two overlapping blocks, a declaration of one pair of their walls, and a union
/// consuming it; `(doc, union, declare)`.
fn declared_union(doc: &Doc<ProfileProgram>) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let tol = Tol::witness();
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = block(&doc, 0.5);
    let (doc, declare) = common::inserted(
        &doc,
        Node::declare_rest(vec![(
            SitedRef::new(a, wall(a, 0, 0)),
            SitedRef::new(b, wall(b, 0, 0)),
        )]),
        tol,
    );
    let (doc, union) = common::inserted(
        &doc,
        Node::Union {
            members: vec![a, b],
            declare: Some(declare),
        },
        tol,
    );
    (doc, union, declare)
}

/// **A delete that takes a declaration's last consumer reports the
/// orphan.**
#[test]
fn a_delete_that_orphans_a_declaration_reaches_the_line() {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("maint-orphan", Tol::witness());
    let (doc, union, declare) = declared_union(&doc);

    let mut session = DocSession::inline(doc, Tol::witness());
    let op = SessionOp::DeleteNode { node: union };
    let outcome = session.perform(op.clone());
    let expected = vec![Maintenance::OrphanedDeclare { declare }];
    assert_eq!(outcome.maintenance, expected);
    assert_line_words(&line_after(&outcome, op), &expected);
}

/// **A cascade's transient is not news.** Deleting the declaration
/// cascades its union first, and that step alone reports the
/// declaration orphaned — the next step deletes it. The door's rows
/// are per edit; the outcome is the ACTION's, and the action leaves
/// nothing orphaned, so the line says nothing about it.
#[test]
fn a_cascade_reports_nothing_its_own_later_steps_took_back() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("maint-cascade", tol);
    let (doc, union, declare) = declared_union(&doc);
    assert_eq!(
        cascade_delete_order(&doc, declare),
        vec![union, declare],
        "the premise: the union goes first"
    );
    let step = pncad::document::apply(
        &doc,
        &DocEdit::DeleteNode { id: union },
        tol,
        &pncad::document::RefusingReach,
    )
    .expect("the union's delete lands");
    assert_eq!(
        step.maintenance,
        vec![Maintenance::OrphanedDeclare { declare }],
        "the premise: the cascade's first step reports the orphan"
    );

    let mut session = DocSession::inline(doc, tol);
    let outcome = session.perform(SessionOp::DeleteNode { node: declare });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 2, "both nodes went, as one action");
    assert_eq!(
        outcome.maintenance,
        Vec::new(),
        "the orphan's subject went in the same action"
    );
    assert_eq!(frame::outcome_notices(&outcome).count(), 0);
}

/// A triangle `(0,0) → (2,0) → (1,1)`, counterclockwise, extruded, with
/// a frame on each of its first two walls; `(doc, profile, extrude)`.
fn framed_triangle(label: &str) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let pt = |x: f64, y: f64| [common::len(x), common::len(y)];
    let triangle = LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, 1.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let doc: Doc<ProfileProgram> = Doc::empty_derived(label, Tol::witness());
    let (doc, profile, extrude) = extruded(&doc, vec![triangle]);
    let (doc, _) = frame_on(&doc, extrude, wall(extrude, 0, 0));
    let (doc, _) = frame_on(&doc, extrude, wall(extrude, 0, 1));
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

/// **The one move a flip below the base makes**: the base wall's name
/// follows it from canonical wall 0 to canonical wall 2, and the
/// middle wall keeps its number.
fn flipped(extrude: RecipeNodeId) -> Vec<Maintenance> {
    vec![Maintenance::Rebound {
        from: wall(extrude, 0, 0),
        to: wall(extrude, 0, 2),
    }]
}

/// **A value edit that flips a loop's sense reports the name it
/// rebound.** The apex moved below the base winds the same three steps
/// clockwise, so the canonical numbering reverses and the base's name
/// is rewritten to follow it (`reanchor_report`). Written through the
/// panel's slot door.
#[test]
fn a_slot_edit_that_renumbers_reaches_the_line() {
    let (doc, profile, extrude) = framed_triangle("maint-slot-flip");
    let mut session = DocSession::inline(doc, Tol::witness());
    let op = SessionOp::SetSlot {
        node: profile,
        slot: apex_y(),
        value: viewer::props::SlotValue::Continuous(-1.0),
    };
    let outcome = session.perform(op.clone());
    assert_eq!(outcome.maintenance, flipped(extrude));
    assert_line_words(&line_after(&outcome, op), &flipped(extrude));
}

/// **The same flip, dragged.** A preview enters no history, so it
/// reports nothing; the release commits the one edit and reports the
/// rebound — the GUI user dragging a parameter past a numbering change
/// is the case the report exists for.
#[test]
fn a_dragged_flip_reports_at_the_release_and_not_before() {
    let (doc, profile, extrude) = framed_triangle("maint-drag-flip");
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
    assert_eq!(
        previewed.maintenance,
        Vec::new(),
        "a preview has happened to nothing yet"
    );
    let op = SessionOp::CommitGesture {
        node: profile,
        slot,
    };
    let landed = session.perform(op.clone());
    assert_eq!(landed.committed.len(), 1, "the drag landed one edit");
    assert_eq!(landed.maintenance, flipped(extrude));
    assert_line_words(&line_after(&landed, op), &flipped(extrude));
}

/// **The path editor's door reports it too.** `EditProfile` lands its
/// program as one-slot writes through an order search, a different
/// road to the history from the single-edit door; the rows ride it.
#[test]
fn a_profile_edit_that_renumbers_reaches_the_line() {
    let (doc, profile, extrude) = framed_triangle("maint-profile-flip");
    let Some(Node::Profile(base)) = doc.node(profile).cloned() else {
        panic!("the fixture's profile")
    };
    let pt = |x: f64, y: f64| [common::len(x), common::len(y)];
    let loops = vec![LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, -1.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])];
    let mut session = DocSession::inline(doc, Tol::witness());
    let op = SessionOp::EditProfile {
        node: profile,
        base,
        loops,
    };
    let outcome = session.perform(op.clone());
    assert_eq!(outcome.committed.len(), 1, "one write moved");
    assert_eq!(outcome.maintenance, flipped(extrude));
    assert_line_words(&line_after(&outcome, op), &flipped(extrude));
}

/// **A parameter edit that leaves a numbering unreadable reports the
/// strand.** A driven circular hole at radius zero encloses nothing,
/// so the profile's numbering cannot be read and the name framed on
/// the hole's wall is retired, reported — written through the
/// parameter panel's door.
#[test]
fn a_parameter_edit_that_strands_reaches_the_line() {
    let tol = Tol::witness();
    let hole_r = ParamName::new("hole_r");
    let doc = common::declared(
        "maint-param-strand",
        &hole_r,
        DocParam::continuous(Dimension::Length, 0.3),
    );
    let square = common::rectangle_loop([0.0, 0.0], 2.0, 2.0);
    let hole = LoopProgram::Circle {
        centre: [common::len(1.0), common::len(1.0)],
        radius: pncad::document::Expr::param(hole_r.clone(), Dimension::Length),
    };
    let (doc, _, extrude) = extruded(&doc, vec![square, hole]);
    let (doc, on_hole) = frame_on(&doc, extrude, wall(extrude, 1, 0));

    let mut session = DocSession::inline(doc, tol);
    let op = SessionOp::SetParam {
        name: hole_r,
        value: viewer::props::SlotValue::Continuous(0.0),
    };
    let outcome = session.perform(op.clone());
    let expected = vec![Maintenance::Strand {
        node: on_hole,
        name: wall(extrude, RETIRED_FLOOR + 1, 0),
    }];
    assert_eq!(outcome.maintenance, expected);
    assert_line_words(&line_after(&outcome, op), &expected);
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
        StatusUpdate::Clear,
        "an accepted edit with nothing to report clears the line"
    );
}

/// **A cluster act rides the outcome and is not worded on the line** —
/// the one arm `frame::maintenance_notice` holds silent, and the reason
/// is on that function. Every other arm is its own sentence.
#[test]
fn a_cluster_act_is_carried_but_not_worded() {
    let gauge = RecipeNodeId(7);
    let act =
        Maintenance::Cluster(pncad::document::ClusterMaintenance::Drop { gauge, frame: None });
    assert_eq!(frame::maintenance_notice(&act), None);
    let strand = Maintenance::Strand {
        node: RecipeNodeId(3),
        name: wall(gauge, 0, 0),
    };
    assert_eq!(
        frame::maintenance_notice(&strand).map(|notice| notice.text().to_owned()),
        Some(strand.to_string())
    );
}
