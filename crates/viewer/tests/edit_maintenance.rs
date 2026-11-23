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
use pncad::document::Formula;
use test_utils::refusal::tagged;

use editor_core::{Attr, Rgba8};
use pncad::document::{
    Dimension, Doc, DocEdit, FreeVar, LoopProgram, Maintenance, Node, ProfileProgram, ProgramStep,
    ProgramTarget, RecipeNodeId, SlotId, SpokenName, SpokenNode, StepArg, VarName,
};
use pncad::geom_core::Tol;
use pncad::prelude::{EntityKind, RoleSeg, StableName};
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
        panic!(
            "node {} is an extrude",
            test_utils::refusal::tag(node.0.digest())
        );
    };
    let Some(Node::Profile(program)) = doc
        .defined_by(*profile)
        .and_then(|(profile, _)| doc.node(profile))
    else {
        panic!("an extrude's operand is a profile");
    };
    let piece = program
        .pieces(&doc.var_env::<f64>(), Tol::witness())
        .expect("the profile replays")
        .edge(loop_index, segment)
        .expect("the position is the profile's");
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Lateral(piece.into())],
    }
}

/// A unit-high extrude of `loops` on its own plane; `(doc, profile,
/// extrude)`.
fn extruded(
    doc: &Doc<ProfileProgram>,
    loops: Vec<LoopProgram<Formula>>,
) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let tol = Tol::witness();
    let (doc, profile) = common::inserted(
        doc,
        Node::Profile(ProfileProgram {
            loops,
            ids: Vec::new(),
            on: None,
        }),
        tol,
    );
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
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

/// A shell of `at` opened at `face` — a payload carrier of one name.
fn shell_on(
    doc: &Doc<ProfileProgram>,
    at: RecipeNodeId,
    face: StableName,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    common::inserted(
        doc,
        Node::Shell {
            thickness: common::len(0.01),
            open: pncad::document::Operand::select(at, vec![face]),
        },
        Tol::witness(),
    )
}

/// The selection a shell reads its open face through.
fn face_of(doc: &Doc<ProfileProgram>, shell: RecipeNodeId) -> pncad::document::VarId {
    match doc.node(shell) {
        Some(Node::Shell { open, .. }) => *open,
        other => panic!("a shell, got {other:?}"),
    }
}

/// **The status line the frame composes from `outcome`**, as `app`
/// composes it: the outcome's notices, ranked by `frame_status` over
/// the one operation that produced it.
fn line_after(outcome: &OpOutcome, op: SessionOp) -> String {
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let notices: Vec<frame::Message> = frame::outcome_notices(outcome).collect();
    match frame::frame_status_untargeted(&notices, &[op], None) {
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

/// **A delete that strands a selected name reports it, and the line
/// says so.** A shell of the kept block names a wall of the other
/// one; deleting the other leaves the shell holding a name whose
/// minting node is gone. The delete is legal (a name is not an edge),
/// and before this row the session answered it with nothing.
#[test]
fn a_delete_that_strands_a_payload_name_reaches_the_line() {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("maint-delete-strand", Tol::witness());
    let (doc, kept) = block(&doc, 0.0);
    let (doc, victim) = block(&doc, 4.0);
    let named = wall(&doc, victim, 0, 0);
    let (doc, carrier) = shell_on(&doc, kept, named.clone());

    let expected = vec![Maintenance::StrandedSelection {
        var: doc.spoken_var(face_of(&doc, carrier)),
        readers: vec![doc.spoken(carrier)],
        name: doc.spoken_name(&named),
        took: pncad::document::Took::Node,
    }];
    let mut session = DocSession::inline(doc, Tol::witness());
    let op = SessionOp::DeleteNode { node: victim };
    let outcome = session.perform(op.clone());
    assert_eq!(
        crate::fixture::without_anonymous(&outcome.maintenance),
        expected
    );
    assert_line_words(&line_after(&outcome, op), &expected);
    assert!(
        session.committed_doc().node(carrier).is_some(),
        "the report is a report: the carrier is untouched"
    );
}

/// **Every worded maintenance row rides beside a refusal**, each by its
/// own arm: none can show that anything will say it again
/// (`frame::maintenance_notices` gives each arm's reason).
#[test]
fn every_maintenance_row_rides_beside_a_refusal() {
    use viewer::session::{Refusal, Step};

    let face = |node: u64| {
        SpokenName::absent(StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, tagged(node)),
            path: vec![],
        })
    };
    let rows = [
        Maintenance::Strand {
            node: SpokenNode::absent(RecipeNodeId::new(0, tagged(3))),
            name: face(7),
            took: pncad::document::Took::Node,
        },
        Maintenance::StrandedAppearance {
            body: pncad::document::SpokenVar::new(editor_core::VarId::new(0, tagged(9)), None),
            name: face(8),
            took: pncad::document::Took::Node,
        },
    ];
    let notices: Vec<frame::Message> = frame::maintenance_notices(&rows).collect();
    assert_eq!(notices.len(), rows.len(), "every arm here is worded");
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
        frame::frame_status_untargeted(&notices, &[SessionOp::Undo], Some(&refusal))
    else {
        panic!("a refusing frame shows its refusal");
    };
    assert_eq!(
        line.text(),
        "nothing to undo \u{2022} node 000000000003 carries a name for the face of node \
         000000000007; this edit deleted node 000000000007, which minted the name, so the \
         name resolves to nothing until it is rebound \u{2022} the appearance store holds an \
         attachment under a name for the face of node 000000000008 read in \
         #0:0000000000090000; this edit deleted node 000000000008, which minted the name, so \
         the name resolves to nothing until it is rebound, re-attached in a live body or \
         cleared"
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
                body: crate::fixture::painted_in(doc, name),
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
        body: doc.spoken_var(crate::fixture::painted_in(&doc, &painted)),
        name: doc.spoken_name(&painted),
        took: pncad::document::Took::Node,
    }];
    let mut session = DocSession::inline(doc, tol);
    let op = SessionOp::DeleteNode { node: victim };
    let outcome = session.perform(op.clone());
    assert_eq!(
        crate::fixture::without_anonymous(&outcome.maintenance),
        expected,
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
    let (doc, _) = shell_on(&doc, extrude, wall(&doc, extrude, 0, 0));
    let (doc, _) = shell_on(&doc, extrude, wall(&doc, extrude, 0, 1));
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
    assert_eq!(
        crate::fixture::without_anonymous(&outcome.maintenance),
        Vec::new()
    );
    assert_eq!(
        frame::frame_status_untargeted(
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
/// nothing — the GUI user dragging a variable past a sense change
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
    assert_eq!(
        crate::fixture::without_anonymous(&previewed.maintenance),
        Vec::new()
    );
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
    let Some(Node::Profile(base)) = doc.node(profile) else {
        panic!("the fixture's profile")
    };
    let base = viewer::sketch::written_program(&doc, base);
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
fn corner(filleted: bool) -> LoopProgram<Formula> {
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

/// **A fillet inserted before a framed leg is counted before it lands
/// and reported after.** The path editor keeps every step and inserts
/// a fillet at the corner below the right wall: the fillet's run out
/// takes that wall's segment, so the kept leg the frame names is no
/// longer drawn. The door's own report before the commit — the verdict
/// the editor asks for before a change lands — names the frame's
/// strand, and the landed edit reports the same row to the line, the
/// kept leg said at its row in the program the edit made.
#[test]
fn a_fillet_inserted_before_a_framed_leg_is_counted_and_reported() {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("maint-profile-shadow", Tol::witness());
    let (doc, profile, extrude) = extruded(&doc, vec![corner(false)]);
    let right = wall(&doc, extrude, 0, 1);
    let (doc, carrier) = shell_on(&doc, extrude, right.clone());
    let Some(Node::Profile(base)) = doc.node(profile) else {
        panic!("the fixture's profile")
    };
    let base = viewer::sketch::written_program(&doc, base);
    let mut ids = base.kept_in_place();
    ids[0].insert(3, None);
    ids[0].insert(5, None);
    let before = doc.clone();
    let mut session = DocSession::inline(doc, Tol::witness());
    let counted = session
        .edit_profile_report(profile, &base, vec![corner(true)], ids.clone())
        .expect("the reshaping is legal");
    let op = SessionOp::EditProfile {
        node: profile,
        base,
        loops: vec![corner(true)],
        ids,
    };
    let outcome = session.perform(op.clone());
    let expected = vec![Maintenance::StrandedSelection {
        var: before.spoken_var(face_of(&before, carrier)),
        readers: vec![before.spoken(carrier)],
        name: before
            .spoken_name(&right)
            .steps_respoken(session.committed_doc()),
        took: pncad::document::Took::Piece,
    }];
    assert_eq!(counted, expected, "the report before the change lands");
    assert_eq!(
        crate::fixture::without_anonymous(&outcome.maintenance),
        expected
    );
    assert_line_words(&line_after(&outcome, op), &expected);
}

/// **A variable edit through a state that draws nothing reports
/// nothing.** A driven circular hole at radius zero encloses nothing,
/// so the profile does not validate and nothing is named — the frame
/// on the hole's wall resolves to nothing until the radius comes back,
/// and then to that wall again. No name was moved, so none is reported
/// — written through the variable panel's door.
#[test]
fn a_parameter_edit_through_a_degenerate_hole_reports_nothing() {
    let tol = Tol::witness();
    let hole_r = VarName::from_static("hole_r");
    let doc = common::declared(
        "maint-param-strand",
        &hole_r,
        FreeVar::continuous(Dimension::Length, 0.3),
        tol,
    );
    let square = common::rectangle_loop([0.0, 0.0], 2.0, 2.0);
    let hole = LoopProgram::Circle {
        centre: [common::len(1.0), common::len(1.0)],
        radius: pncad::document::Formula::named(hole_r.clone(), Dimension::Length),
    };
    let (doc, _, extrude) = extruded(&doc, vec![square, hole]);
    let (doc, _) = shell_on(&doc, extrude, wall(&doc, extrude, 1, 0));

    let mut session = DocSession::inline(doc, tol);
    let op = SessionOp::SetVariable {
        var: common::var_of(session.committed_doc(), hole_r.as_str()),
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
    assert_eq!(
        crate::fixture::without_anonymous(&outcome.maintenance),
        Vec::new()
    );
    assert_eq!(
        frame::frame_status_untargeted(
            &frame::outcome_notices(&outcome).collect::<Vec<_>>(),
            &[op],
            None
        ),
        RankedVerdict::Clear,
        "an accepted edit with nothing to report clears the line"
    );
}

/// `block`'s extrude with its typed depth's anonymous variable given
/// `distribution`.
fn toleranced_block(
    seed: &str,
    distribution: Option<pncad::document::Distribution>,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived(seed, Tol::witness());
    let (doc, extrude) = block(&doc, 0.0);
    let depth = doc
        .slot(extrude, SlotId::Distance)
        .expect("the extrude reads its depth");
    assert!(doc.var_name(depth).is_none(), "the premise: a typed depth");
    let doc = pncad::document::apply(
        &doc,
        &DocEdit::SetVarDistribution {
            var: depth.into(),
            distribution,
        },
        Tol::witness(),
    )
    .expect("a length takes a normal")
    .doc;
    (doc, extrude)
}

/// **Retyping a slot whose own variable carried a tolerance says the
/// tolerance went** (review r2 MINOR-3): the expression door re-lowers
/// the slot, retiring its anonymous variable, and that variable was an
/// analysis axis (VR8). One that carried none retires as quietly as
/// every typed value does (`Maintenance::is_silent_retirement`).
#[test]
fn retyping_a_toleranced_slot_says_its_tolerance_went() {
    let normal = pncad::document::Distribution::Normal { sigma: 0.001 };
    for (seed, distribution) in [
        ("maint-toleranced", Some(normal)),
        ("maint-untoleranced", None),
    ] {
        let (doc, extrude) = toleranced_block(seed, distribution);
        let mut session = DocSession::inline(doc, Tol::witness());
        let op = SessionOp::SetSlotExpression {
            node: extrude,
            slot: SlotId::Distance,
            text: "2 mm".to_owned(),
        };
        let outcome = session.perform(op.clone());
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        let retired: Vec<&Maintenance> = outcome
            .maintenance
            .iter()
            .filter(|row| matches!(row, Maintenance::AnonymousVarRemoved { .. }))
            .collect();
        assert!(
            matches!(
                retired[..],
                [Maintenance::AnonymousVarRemoved { distribution: held, .. }] if *held == distribution
            ),
            "{seed}: the depth's variable retires with what it carried: {retired:?}"
        );
        if distribution.is_some() {
            let line = line_after(&outcome, op);
            assert!(
                line.contains("the tolerance it carried went with it"),
                "{seed}: {line}"
            );
        } else {
            assert_quiet(&outcome, op);
        }
    }
}

/// **The values a mate fixes leave the draft before it lands, not after
/// it.** Seating post_b on the shelf fixes freedoms its bundle wrote
/// values on: the placement editor's value rows are the freedoms the
/// seat leaves, so the slide along the seat's normal and the chain of
/// spins are not among them, and the change writes the bundle the panel
/// showed — no value is dropped behind the person's back, so none is
/// reported dropped.
///
/// *Breaks if* a value row lists a freedom the seat fixes, or the bundle
/// drops a value the draft did not show going.
#[test]
fn a_mates_fixed_freedoms_leave_the_draft_before_apply() {
    use common::asm;
    use pncad::document::{Coord, TargetAxis};
    use viewer::matetool::MateTool;

    let tol = Tol::witness();
    let bench = asm::bench("dropped-once", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (a, b) = asm::seat_picks(&session, &bench);
    asm::unpin(&mut session, bench.post_b);
    let mut tool = MateTool::new();
    tool.pick(session.doc(), a);
    tool.pick(session.doc(), b);
    common::into_to_space(&session, &mut tool);
    let proposal =
        common::proposed(&session, &tool, asm::seat_choice()).expect("the seat proposes");
    let coords: Vec<Coord<pncad::document::Operand>> = proposal
        .values
        .iter()
        .map(|row| row.coord.clone())
        .collect();
    assert_eq!(
        coords,
        [
            Coord::slide(TargetAxis::X),
            Coord::slide(TargetAxis::Y),
            Coord::SPIN
        ],
        "a rest along z leaves two slides and its spin"
    );
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        !outcome
            .maintenance
            .iter()
            .any(|m| matches!(m, Maintenance::ValueDropped { .. })),
        "the bundle drops nothing the draft did not show: {:?}",
        outcome.maintenance
    );
}

/// The names `rows` report the document keeping with their referent
/// gone, each with what holds it and each once
/// (`Maintenance::stranded_name`, the kernel's classing).
fn stranded(rows: &[Maintenance]) -> Vec<pncad::document::NameCarrier<'_>> {
    let mut held = Vec::new();
    for name in rows.iter().filter_map(Maintenance::stranded_name) {
        if !held.contains(&name) {
            held.push(name);
        }
    }
    held
}

/// How many names [`stranded`] answers.
fn stranded_names(rows: &[Maintenance]) -> usize {
    stranded(rows).len()
}

/// The names the selection `shell` reads holds.
fn selected(doc: &Doc<ProfileProgram>, shell: RecipeNodeId) -> Vec<StableName> {
    doc.selection(face_of(doc, shell))
        .expect("a shell reads a selection")
        .names
        .to_vec()
}

/// **The delete button's count is the count of the rows its click
/// reports**, for a selected name and for an appearance key. A shell of
/// the kept block names a wall of the victim, which is also painted.
#[test]
fn the_delete_button_counts_the_names_its_click_strands() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("button-strand-count", tol);
    let (doc, kept) = block(&doc, 0.0);
    let (doc, victim) = block(&doc, 4.0);
    let named = wall(&doc, victim, 0, 0);
    let (doc, _) = shell_on(&doc, kept, named.clone());
    let tag = test_utils::refusal::tag(victim.0.digest());

    let one = DocSession::inline(doc.clone(), tol).delete_affordance(victim);
    assert_eq!(one.stranded, 1, "the shell's name");
    assert_eq!(one.label, format!("Delete Extrude {tag}, stranding 1 name"));
    assert_eq!(
        one.hover.as_deref(),
        Some("Strands 1 name that the document keeps and that resolves to nothing until rebound")
    );

    let (doc, _) = common::edited(
        &doc,
        DocEdit::SetAppearance {
            body: crate::fixture::painted_in(&doc, &named),
            name: wall(&doc, victim, 0, 2),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        tol,
    );
    // A shell of the victim itself goes with it, and its name with it.
    let (doc, _) = shell_on(&doc, victim, wall(&doc, victim, 0, 1));
    let mut session = DocSession::inline(doc, tol);
    let two = session.delete_affordance(victim);
    assert_eq!(two.stranded, 2, "the shell's name and the paint's key");
    assert_eq!(
        two.label,
        format!("Delete Extrude {tag} and 1 dependent feature, stranding 2 names")
    );
    assert_eq!(
        two.hover.as_deref(),
        Some(
            "Also deletes 1 feature that depends on it: 1 × Shell\nStrands 2 names that the \
             document keeps and that resolve to nothing until rebound"
        )
    );
    let outcome = session.perform(SessionOp::DeleteNode { node: victim });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        outcome
            .maintenance
            .iter()
            .any(|row| matches!(row, Maintenance::StrandedAppearance { .. })),
        "the premise: one of the two is the appearance key: {:?}",
        outcome.maintenance
    );
    assert_eq!(
        stranded_names(&outcome.maintenance),
        two.stranded,
        "the button said what the click reported"
    );

    assert_eq!(
        session.delete_affordance(victim).stranded,
        0,
        "asked again of the document the click left, which holds no victim"
    );
    let kept_only = session.delete_affordance(kept);
    assert_eq!(kept_only.stranded, 0, "the kept block minted no held name");
    assert!(
        !kept_only.label.contains("stranding"),
        "a zero is not said: {}",
        kept_only.label
    );
}

/// **A strand on a carrier the cascade goes on to delete is not
/// counted.** Two shells read one block, and the first names a face of
/// the second; deleting the block deletes the second shell, which
/// strands the first's name, and then the first.
#[test]
fn the_delete_button_does_not_count_a_strand_on_a_carrier_it_deletes() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("button-transient", tol);
    let (doc, target) = block(&doc, 0.0);
    // The carrier is the older shell, so the cascade reaches it last.
    let placeholder = wall(&doc, target, 0, 1);
    let (doc, carrier) = shell_on(&doc, target, placeholder.clone());
    let (doc, minter) = shell_on(&doc, target, wall(&doc, target, 0, 0));
    let of_minter = StableName {
        kind: EntityKind::Face,
        node: minter,
        path: Vec::new(),
    };
    let (doc, _) = common::edited(
        &doc,
        DocEdit::Rebind {
            body: doc.output(target, 0),
            from: placeholder,
            to: of_minter.clone(),
        },
        tol,
    );
    assert_eq!(selected(&doc, carrier), [of_minter], "the premise");
    let mut session = DocSession::inline(doc.clone(), tol);
    let affordance = session.delete_affordance(target);
    assert_eq!(affordance.cascade.last(), Some(&target));
    assert_eq!(affordance.cascade.len(), 3, "both shells go with it");

    let outcome = session.perform(SessionOp::DeleteNode { node: target });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    // Each committed edit's own rows, summed: what a count taken step
    // by step would say.
    let mut at = doc;
    let mut stepwise = 0;
    for edit in &outcome.committed {
        let applied = pncad::document::apply(&at, edit, tol).expect("the committed edit replays");
        stepwise += stranded_names(&applied.maintenance);
        at = applied.doc;
    }
    assert_eq!(
        stepwise, 1,
        "the premise: a step of the cascade strands the carrier's name"
    );
    assert_eq!(affordance.stranded, 0, "the carrier went with it");
    assert_eq!(stranded_names(&outcome.maintenance), 0);
    assert_eq!(
        affordance.label,
        format!(
            "Delete Extrude {} and 2 dependent features",
            test_utils::refusal::tag(target.0.digest())
        )
    );
}

/// **A name handed back to the continued body is not counted, and a
/// name the deleted node minted is.** The outer shell reads the inner
/// one and holds two names of it: one the inner carried in whole from
/// the block, one it minted. Deleting the inner hands the outer to the
/// block; both names' minting node is the deleted one and their
/// carrier survives.
#[test]
fn the_delete_button_does_not_count_a_name_the_delete_hands_back() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("button-handed-back", tol);
    let (doc, base) = block(&doc, 0.0);
    let (doc, inner) = shell_on(&doc, base, wall(&doc, base, 0, 0));
    let of_base = wall(&doc, base, 0, 1);
    let carried = StableName {
        kind: EntityKind::Face,
        node: inner,
        path: vec![RoleSeg::From {
            read: doc.output(base, 0).expect("the block's body"),
            of: of_base.clone().into(),
        }],
    };
    let minted = StableName {
        kind: EntityKind::Face,
        node: inner,
        path: Vec::new(),
    };
    let (doc, outer) = common::inserted(
        &doc,
        Node::Shell {
            thickness: common::len(0.01),
            open: pncad::document::Operand::select(inner, vec![carried, minted.clone()]),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    let affordance = session.delete_affordance(inner);
    assert_eq!(
        affordance.cascade,
        [inner],
        "the outer shell is handed over"
    );
    assert_eq!(affordance.stranded, 1, "the minted name alone");

    let outcome = session.perform(SessionOp::DeleteNode { node: inner });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        stranded(&outcome.maintenance)
            .into_iter()
            .map(|held| held.name().clone())
            .collect::<Vec<_>>(),
        std::slice::from_ref(&minted),
        "the click reports the minted name and not the carried one"
    );
    let mut held = selected(session.committed_doc(), outer);
    held.sort();
    let mut want = vec![of_base, minted];
    want.sort();
    assert_eq!(held, want, "the carried name returned to the block's");
}

/// **A name in a node's payload is counted**: an instance's interface
/// record names a cap of a block of this document, on two crossings,
/// and the block is deleted. The record is the one payload that holds
/// names; every other carrier here is a selection or the store.
///
/// The record holds the name twice and the click reports it twice.
/// The button says one name: `Rebind` rewrites every site that names
/// it, so it is one thing to repair.
#[test]
fn the_delete_button_counts_a_name_an_instances_record_holds() {
    use crate::fixture::resolver::{PartStore, in_world};
    use pncad::document::{FaceName, InterfaceCrossing, InterfaceRecord, NameCarrier};
    use pncad::prelude::CapEnd;

    let tol = Tol::witness();
    let part: Doc<ProfileProgram> = Doc::empty_derived("button-record-part", tol);
    let (doc_ref, body) = PartStore::new().insert_part(block(&part, 0.0), tol);
    let doc: Doc<ProfileProgram> = Doc::empty_derived("button-record", tol);
    let (doc, target) = block(&doc, 0.0);
    let outer = StableName {
        kind: EntityKind::Face,
        node: target,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let face = |name: StableName| FaceName::new(name).expect("a face name");
    let crossing = |cap| InterfaceCrossing::Mate {
        outer: face(outer.clone()),
        inner: face(in_world(body, cap)),
    };
    let (doc, instance) = common::inserted(
        &doc,
        Node::instantiate_part_with(
            doc_ref,
            InterfaceRecord {
                crossings: vec![crossing(CapEnd::Start), crossing(CapEnd::End)],
            },
            pncad::document::Operand::Var(pncad::document::VarId::WORLD),
            editor_core::stand().bundle(),
        ),
        tol,
    );

    let mut session = DocSession::inline(doc, tol);
    let affordance = session.delete_affordance(target);
    assert_eq!(affordance.cascade, [target], "a name is not a read");
    assert_eq!(affordance.stranded, 1, "the record's one name");

    let outcome = session.perform(SessionOp::DeleteNode { node: target });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        outcome
            .maintenance
            .iter()
            .filter(|row| matches!(row, Maintenance::Strand { .. }))
            .count(),
        2,
        "the premise: the record holds the name on two crossings, a row each"
    );
    assert_eq!(
        stranded(&outcome.maintenance),
        [NameCarrier::Payload {
            node: instance,
            name: &outer
        }],
        "the click reports the instance's payload name, and nothing else stranded"
    );
}

/// **A name a hand-over takes out of reach is counted, though no node
/// the delete removes minted it.** Two blocks are slid into a third's
/// space and fused, the fusion is cut from the third, and a shell of
/// the cut names a wall of the first block. Deleting the fusion takes
/// the cut with it and hands the shell to the third block, which the
/// first block is not upstream of.
#[test]
fn the_delete_button_counts_a_name_its_hand_over_takes_out_of_reach() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("button-reach", tol);
    let (doc, target) = block(&doc, 0.0);
    let (doc, first) = block(&doc, 4.0);
    let (doc, second) = block(&doc, 8.0);
    let (doc, placed_first) = crate::fixture::slid(doc, target, first, [0.25, 0.0, 0.0]);
    let (doc, placed_second) = crate::fixture::slid(doc, target, second, [0.5, 0.0, 0.0]);
    let (doc, fused) = common::inserted(
        &doc,
        Node::Union {
            members: pncad::document::Bodies::spelled(vec![
                placed_first.into(),
                placed_second.into(),
            ]),
        },
        tol,
    );
    let (doc, cut) = common::inserted(
        &doc,
        Node::Subtract {
            from: target.into(),
            tool: fused.into(),
        },
        tol,
    );
    let named = wall(&doc, first, 0, 0);
    let (doc, carrier) = shell_on(&doc, cut, named.clone());

    let mut session = DocSession::inline(doc, tol);
    let affordance = session.delete_affordance(fused);
    assert_eq!(
        affordance.cascade,
        [cut, fused],
        "the cut goes with its tool"
    );
    assert!(
        !affordance.cascade.contains(&first),
        "the name's minting node is not deleted"
    );
    assert_eq!(affordance.stranded, 1);

    let outcome = session.perform(SessionOp::DeleteNode { node: fused });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let strands: Vec<_> = outcome
        .maintenance
        .iter()
        .filter(|row| row.stranded_name().is_some())
        .collect();
    assert!(
        matches!(
            strands.as_slice(),
            [Maintenance::StrandedSelection {
                took: pncad::document::Took::Reach,
                ..
            }]
        ),
        "one strand, by reach: {strands:?}"
    );
    assert_eq!(
        stranded(&outcome.maintenance),
        [pncad::document::NameCarrier::Select {
            var: face_of(session.committed_doc(), carrier),
            name: &named
        }]
    );
}

/// **The button answers for the committed document it is asked of**,
/// across every way that document changes or is replaced: at each step
/// it says what a session opened fresh on the same document says.
#[test]
fn the_delete_button_is_never_a_document_behind() {
    use pncad::document::Label;

    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("button-current", tol);
    let (doc, kept) = block(&doc, 0.0);
    let (doc, victim) = block(&doc, 4.0);
    let (doc, carrier) = shell_on(&doc, kept, wall(&doc, victim, 0, 0));
    let mut session = DocSession::inline(doc, tol);

    let check = |session: &DocSession, stranded: usize, step: &str| {
        let asked = session.delete_affordance(victim);
        let fresh =
            DocSession::inline(session.committed_doc().clone(), tol).delete_affordance(victim);
        assert_eq!(asked.stranded, stranded, "after {step}");
        assert_eq!(
            (asked.label, asked.hover, asked.cascade),
            (fresh.label, fresh.hover, fresh.cascade),
            "after {step}: what a fresh session says of the same document"
        );
    };
    let perform = |session: &mut DocSession, op: SessionOp| {
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    };

    check(&session, 1, "opening");
    perform(&mut session, SessionOp::DeleteNode { node: carrier });
    check(&session, 0, "deleting the carrier");
    perform(&mut session, SessionOp::Undo);
    check(&session, 1, "undo");
    perform(&mut session, SessionOp::Redo);
    check(&session, 0, "redo");
    perform(&mut session, SessionOp::Undo);
    perform(
        &mut session,
        SessionOp::SetLabel {
            node: victim,
            label: Some(Label::new("the victim").expect("a label")),
        },
    );
    check(&session, 1, "a label");
    perform(&mut session, SessionOp::Reevaluate);
    check(&session, 1, "a re-evaluation");

    let dir = common::tempdir("button-current");
    let file = dir.join("with-carrier.pncad");
    perform(&mut session, SessionOp::Save(file.clone()));
    check(&session, 1, "a save");
    // The other document: the same nodes without the carrier, opened
    // over a history whose root state is the one just asked.
    perform(&mut session, SessionOp::DeleteNode { node: carrier });
    let bare = dir.join("bare.pncad");
    perform(&mut session, SessionOp::SaveAsNewDocument(bare));
    check(&session, 0, "a fork");

    // An open replaces the history, whose root state has the id the
    // outgoing root had: a session asked at its root of the bare
    // document, then opened on the one with the carrier.
    let mut other = DocSession::inline(session.committed_doc().clone(), tol);
    check(&other, 0, "opening the bare document");
    perform(&mut other, SessionOp::Open(file));
    check(&other, 1, "an open");
}
