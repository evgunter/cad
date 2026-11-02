//! **The mate tool, replayed headlessly** (GUI-4 deliverable 4): two
//! sequential picks through the real cursor path, the typed refusals, one committed action (the mate and
//! its values), and the pick-vanish degradation.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use pncad::document::{Dimension, Formula};

use common::asm;
use common::{ang, len, scl};
use pncad::document::{MateSide, RecipeNodeId, Said};
use pncad::geom_core::Tol;
use pncad::select::{RoleSeg, face_frame};
use viewer::matetool::{MateTool, MateToolError, MateToolEvent, MateToolState};
use viewer::session::{DocSession, FaceSelection, PatternRuleSpec, SessionOp};

#[test]
fn two_picks_one_choice_one_committed_edit() {
    let tol = Tol::witness();
    let bench = asm::bench("matetool", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (post_top, shelf_bottom) = asm::seat_picks(&session, &bench);

    // The two sequential picks, held in tool state.
    let mut tool = MateTool::new();
    assert_eq!(tool.state(), &MateToolState::Idle);
    tool.pick(session.doc(), post_top.clone());
    assert!(matches!(tool.state(), MateToolState::One(_)));
    tool.pick(session.doc(), shelf_bottom.clone());
    assert!(matches!(tool.state(), MateToolState::Two { .. }));
    // post_b's world mate off, so the mate places it: its bundle is the
    // empty one.
    asm::unpin(&mut session, bench.post_b);

    // The proposal: each side IS the picked face read as a plane, by the
    // part's own name, resolved at every evaluation.
    let (doc, _) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, asm::seat_choice())
        .expect("the seat proposes");
    assert_eq!(proposal.placement, bench.post_b, "the on pick's copy");
    assert_eq!(proposal.values, asm::middle_seat_values());
    // The on pick is post_b's top cap: the side names the POST's own
    // cap row, not numbers read at the instance's world spot.
    assert_eq!(
        asm::side_face(doc, &proposal.on),
        Some(bench.post_top.clone()),
        "the on side is the post's own cap face"
    );
    assert_eq!(
        asm::side_face(doc, &proposal.to),
        Some(bench.shelf_bottom.clone()),
        "the to side is the shelf's own underside"
    );
    assert_eq!(
        proposal.mate,
        crate::fixture::rest(proposal.on.clone(), proposal.to.clone()),
        "two planes, the target flipped: the faces rest against each other"
    );

    // post_b stands in a space of its own and the shelf in the world:
    // the plain commit never moves the copy as a side effect, so the
    // door refuses the mate, naming both spaces, and nothing is recorded.
    let before = session.committed_doc().clone();
    let refused = session.perform(proposal.op());
    let own = before.node_space(bench.post_b).expect("post_b is placed");
    assert!(
        matches!(
            refused.refusal.as_ref(),
            Some(viewer::session::Refusal::Edit(error))
                if matches!(
                    error.as_ref(),
                    pncad::document::EditError::SpaceMismatch { a, b, .. }
                        if [a.id(), b.id()].contains(&own)
                            && [a.id(), b.id()].contains(&pncad::document::VarId::WORLD)
                )
        ),
        "a mate across two spaces refuses unless the move is asked for: {:?}",
        refused.refusal
    );
    assert!(refused.committed.is_empty(), "nothing is recorded");
    assert!(
        session.committed_doc().bit_eq(&before),
        "the document stands"
    );

    // One committed action, through the session's one door, into
    // post_b's bundle: the mate and its values, the copy moved into the
    // shelf's space by name.
    let mate = common::commit_mate(&mut session, proposal.op_into_target_space());
    assert_eq!(mate.placement, bench.post_b, "the mate places post_b");

    // The placement is SOLVED from the mate: the two picked FACES now
    // rest in world space — their poses read off the landed evaluation
    // lie in one plane, facing each other — and the values put post_b
    // under the shelf's middle.
    assert_faces_rest(&session, &post_top, &shelf_bottom, "the seat");
    asm::assert_solved_at(
        &session,
        bench.post_b,
        &asm::seated_at(asm::SHELF_LENGTH / 2.0),
        "the values seat post_b",
    );
    // And every row still evaluates (the mate node included).
    for row in session.tree_rows() {
        assert_eq!(row.status, viewer::tree::RowStatus::Ok, "{row:?}");
    }
}

#[test]
fn the_tool_refuses_typed_what_the_picks_do_not_admit() {
    let tol = Tol::witness();
    let bench = asm::bench("materefuse", tol);
    let session = asm::open_bench(&bench, tol);
    let (doc, _) = session.landed_pair().expect("landed");
    let (post_top, shelf_bottom) = asm::seat_picks(&session, &bench);

    // No picks yet: NotTwoPicks.
    let tool = MateTool::new();
    assert!(matches!(
        tool.proposal(doc, asm::seat_choice()),
        Err(MateToolError::NotTwoPicks)
    ));

    // Both picks on ONE instance: SamePick, refused at the tool
    // rather than authored into a mate of the copy on itself.
    let mut tool = MateTool::new();
    tool.pick(session.doc(), post_top.clone());
    tool.pick(session.doc(), post_top.clone());
    assert!(matches!(
        tool.proposal(doc, asm::seat_choice()),
        Err(MateToolError::SamePick { head }) if head.id() == bench.post_b
    ));

    // A vanished pick refuses typed when asked WITHOUT a reconcile
    // (the reconcile row below is the graceful path): the deleted
    // instance is no longer in the document, and the refusal says so.
    let mut tool = MateTool::new();
    tool.pick(session.doc(), post_top);
    tool.pick(session.doc(), shelf_bottom);
    let mut session2 = asm::open_bench(&bench, tol);
    session2.perform(SessionOp::DeleteNode {
        node: bench.shelf_i,
    });
    session2.pump();
    let (doc2, _) = session2.landed_pair().expect("landed");
    assert!(matches!(
        tool.proposal(doc2, asm::seat_choice()),
        Err(MateToolError::PickGone {
            side: MateSide::To,
            ..
        })
    ));
}

/// **The choice's own refusals come before any pick is read**: a kind
/// no face pick reads and a value on a freedom the kind fixes are facts
/// about the CHOICE alone, so the tool refuses them the way it refuses
/// a class outside the vocabulary — here with the picks' instance
/// deleted out from under the tool, which the pick door would refuse
/// were it reached. Which freedoms a kind leaves is the panel's
/// reading (`MateFreedom::left_by`).
#[test]
fn the_tool_refuses_the_choices_own_faults_before_any_pick() {
    use pncad::document::{Coord, TargetAxis, VarKind};
    let tol = Tol::witness();
    let bench = asm::bench("matetable", tol);
    let session = asm::open_bench(&bench, tol);
    let (post_top, shelf_bottom) = asm::seat_picks(&session, &bench);
    let mut tool = MateTool::new();
    tool.pick(session.doc(), post_top);
    tool.pick(session.doc(), shelf_bottom);
    let mut gone = asm::open_bench(&bench, tol);
    gone.perform(SessionOp::DeleteNode {
        node: bench.shelf_i,
    });
    gone.pump();
    let (doc, _) = gone.landed_pair().expect("landed");
    let mut choice = asm::seat_choice();
    choice.kind = VarKind::Frame;
    assert!(
        matches!(
            tool.proposal(doc, choice),
            Err(MateToolError::KindNotOffered {
                kind: VarKind::Frame
            })
        ),
        "a frame is no pose one face pick reads"
    );
    let mut choice = asm::seat_choice();
    choice.kind = VarKind::Axis;
    let Err(MateToolError::NotAFreedom { coord, kind }) = tool.proposal(doc, choice) else {
        panic!("an axis mate leaves one slide, not the seat's two");
    };
    assert_eq!((coord, kind), (Coord::Slide(TargetAxis::Y), VarKind::Axis));
    // And a choice whose every part is admitted reaches the pick door,
    // which is what "before any pick" means here.
    assert!(matches!(
        tool.proposal(doc, asm::seat_choice()),
        Err(MateToolError::PickGone {
            side: MateSide::To,
            ..
        })
    ));
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

#[test]
fn a_vanished_pick_degrades_the_tool_one_step_typed() {
    let tol = Tol::witness();
    let bench = asm::bench("matevanish", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (post_top, shelf_bottom) = asm::seat_picks(&session, &bench);
    let mut tool = MateTool::new();
    tool.pick(session.doc(), post_top.clone());
    tool.pick(session.doc(), shelf_bottom.clone());
    let held_line = tool.line(viewer::frame::landed_speaker(session.landed_pair()));

    // The SECOND pick's instance is deleted out from under the tool.
    session.perform(SessionOp::DeleteNode {
        node: bench.shelf_i,
    });
    session.pump();
    let (doc, eval) = session.landed_pair().expect("landed");
    let events = tool.reconcile(doc, eval);
    // Degraded ONE step: back to the one-pick state, the first pick
    // held, the drop typed with the side and the resolution verdict.
    assert_eq!(events.len(), 1);
    assert!(matches!(
        &events[0],
        MateToolEvent::PickLost {
            side: MateSide::To,
            ..
        }
    ));
    // The notice says the pick in the panel's words on the frame
    // before — the copy as the document spoke it when it was picked —
    // though the document that dropped it no longer holds it.
    let tag = |node: RecipeNodeId| test_utils::refusal::tag(node.0.digest());
    // The shelf's underside is the start cap of the part's own extrude,
    // a node the assembly does not hold, picked on the shelf's instance.
    let said = format!(
        "the start cap of node {}, on InstantiatePart {}",
        tag(shelf_bottom.name.node),
        tag(bench.shelf_i),
    );
    assert!(held_line.ends_with(&format!("to: {said}")), "{held_line:?}");
    assert_eq!(
        Said(&events[0], viewer::frame::landed_speaker(Some((doc, eval)))).to_string(),
        format!("the to pick ({said}) no longer resolves; the tool dropped it")
    );
    match tool.state() {
        MateToolState::One(held) => assert_eq!(held.pick().face(), Some(&post_top)),
        other => panic!("the tool degrades to its previous step, got {other:?}"),
    }

    // A reconcile against an unchanged pair is quiet.
    assert!(tool.reconcile(doc, eval).is_empty());
    assert!(matches!(tool.state(), MateToolState::One(_)));

    // The FIRST pick's instance goes too: Idle, typed again.
    session.perform(SessionOp::DeleteNode { node: bench.post_b });
    session.pump();
    let (doc, eval) = session.landed_pair().expect("landed");
    let events = tool.reconcile(doc, eval);
    assert_eq!(events.len(), 1);
    assert!(matches!(
        &events[0],
        MateToolEvent::PickLost {
            side: MateSide::On,
            ..
        }
    ));
    assert_eq!(tool.state(), &MateToolState::Idle);
}

/// The linear spacing the pattern rows step post_b by — wide enough
/// that copy 1 stands clear of copy 0 and of the shelf, so a ray
/// reaches exactly one of them.
const PATTERN_STEP: f64 = 0.04;

/// **What draws each of `pattern`'s members** (A10): the family stands
/// in post_b's space, the world, so it draws its own members, member 0
/// where post_b itself stands. Answers post_b, then the family once per
/// later member, in member order.
fn place_copies(
    session: &mut DocSession,
    bench: &asm::Bench,
    pattern: RecipeNodeId,
    count: usize,
) -> Vec<RecipeNodeId> {
    session.pump();
    std::iter::once(bench.post_b)
        .chain(std::iter::repeat_n(pattern, count - 1))
        .collect()
}

/// **The member indices a name's head is qualified by**, outermost
/// first — `None` for a head that is no family member.
fn member_at(seg: Option<&RoleSeg>) -> Option<Vec<u32>> {
    match seg {
        Some(RoleSeg::Member { at, .. }) => Some(at.iter().map(|&(_, i)| i).collect()),
        _ => None,
    }
}

/// Pattern post_b twice along +x ([`place_copies`]), and answer the
/// family's placement and what draws each member.
fn patterned_post(
    session: &mut DocSession,
    bench: &asm::Bench,
) -> (RecipeNodeId, Vec<RecipeNodeId>) {
    let pattern = common::session_insert(
        session,
        SessionOp::AddPattern {
            input: bench.post_b.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(PATTERN_STEP),
            },
        },
    );
    let parts = place_copies(session, bench, pattern, 2);
    (pattern, parts)
}

/// A pick on pattern copy `i` of the patterned post's top cap.
fn copy_pick(session: &DocSession, i: u32) -> FaceSelection {
    asm::pick_face(
        session,
        &asm::down_at(
            asm::POST_B_AT[0] + f64::from(i) * PATTERN_STEP + asm::POST_SECTION / 2.0,
            asm::POST_B_AT[1] + asm::POST_SECTION / 2.0,
        ),
    )
}

/// **The shelf rested on a picked copy**, through the tool: the shelf's
/// underside first — `on`, the copy the mate places — then `target`,
/// with no value named, so the gesture pins the shelf where the snap
/// puts it and its underside rests on the target's face.
fn shelf_on(
    session: &DocSession,
    shelf_bottom: &FaceSelection,
    target: &FaceSelection,
) -> viewer::matetool::MateProposal {
    let mut tool = MateTool::new();
    tool.pick(session.doc(), shelf_bottom.clone());
    tool.pick(session.doc(), target.clone());
    let (doc, _) = session.landed_pair().expect("landed");
    let mut choice = asm::seat_choice();
    choice.values = Vec::new();
    tool.proposal(doc, choice)
        .expect("the shelf's rest proposes")
}

/// **The name a target pick on a placement's copy carries**: the
/// copied body's own name for the face, which a copy carries as it is.
fn own_name(side: &pncad::document::SitedRef) -> pncad::prelude::StableName {
    side.name.clone()
}

/// **A pattern-placed copy is a target, and the shelf rests on it.** The
/// tool reads the target where it was picked — the copy's placement,
/// under the family's `Member`-headed name — and the solve reads
/// that face as the copy carries it.
#[test]
fn a_pattern_placed_pick_is_a_target_the_shelf_rests_on() {
    let tol = Tol::witness();
    let bench = asm::bench("matepattern", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (pattern, parts) = patterned_post(&mut session, &bench);

    let copy_one = copy_pick(&session, 1);
    assert_eq!(
        copy_one.node,
        common::drawn_node(session.committed_doc(), parts[1]),
        "the ray met copy 1's placement"
    );
    let shelf_bottom = asm::shelf_underside(&session);
    let proposal = shelf_on(&session, &shelf_bottom, &copy_one);
    assert_eq!(
        proposal.placement, bench.shelf_i,
        "the mate places the shelf"
    );

    // The target is read where the ray met it, `Member`-headed on the
    // family's placement.
    assert_eq!(proposal.to.at, copy_one.node);
    let own = own_name(&proposal.to);
    assert_eq!(own.node, pattern);
    assert!(
        member_at(own.path.first()) == Some(vec![1]),
        "the copy rides in the head: {:?}",
        own.path.first()
    );

    // And it SOLVES: the shelf's world mate comes off, the mate places
    // the shelf on copy 1, and the two picked faces rest in the world
    // the evaluation draws. A tool that read the master's face would
    // land the shelf one step away from copy 1.
    asm::unpin(&mut session, bench.shelf_i);
    let mate = common::commit_mate(&mut session, proposal.op_into_target_space());
    let (doc, _) = session.landed_pair().expect("landed");
    assert!(
        common::solve(&session, doc, tol)
            .fault(mate.placement)
            .is_none(),
        "the shelf on the pattern copy solves: {:?}",
        common::solve(&session, doc, tol).fault(mate.placement)
    );
    assert_faces_rest(&session, &copy_one, &shelf_bottom, "the pattern copy");
    let (_, eval) = session.landed_pair().expect("landed");
    let cap = face_frame(eval, copy_one.node, &copy_one.name).expect("copy 1's cap");
    let underside = face_frame(eval, shelf_bottom.node, &shelf_bottom.name).expect("underside");
    assert!(
        ((underside.origin - cap.origin).x).abs() < asm::SHELF_LENGTH,
        "the shelf stands over copy 1, not over the master: {:?} vs {:?}",
        underside.origin,
        cap.origin
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, viewer::tree::RowStatus::Ok, "{row:?}");
    }
}

/// **A9 — a pick on a pattern copy OVER A TRANSFORM is a target**, read
/// at the node the ray met: the copy's placement, under the pattern's
/// copy-headed name. The composed offset is the solve's to apply.
#[test]
fn a_pattern_copy_over_a_transform_is_an_instance_pick() {
    let tol = Tol::witness();
    let bench = asm::bench("matepatternnon", tol);
    let mut session = asm::open_bench(&bench, tol);
    // A transform between the instance and the pattern: the pattern's
    // input is the move's `Place`, not an `InstantiatePart`, which is
    // one of the heads `member_of` declines.
    let moved = common::session_insert(
        &mut session,
        SessionOp::AddTransform {
            input: bench.post_b.into(),
            translation: [
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: Formula::zero(Dimension::Angle),
        },
    );
    session.pump();
    let pattern = common::session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: moved.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(PATTERN_STEP),
            },
        },
    );
    let parts = place_copies(&mut session, &bench, pattern, 2);

    let copy_one = copy_pick(&session, 1);
    assert_eq!(
        copy_one.node,
        common::drawn_node(session.committed_doc(), parts[1])
    );
    let shelf_bottom = asm::shelf_underside(&session);
    let proposal = shelf_on(&session, &shelf_bottom, &copy_one);
    assert_eq!(
        proposal.to.at, copy_one.node,
        "the target is read at the copy the ray met"
    );
    let own = own_name(&proposal.to);
    assert_eq!(own.node, pattern, "and names that copy");
    assert!(
        member_at(own.path.first()) == Some(vec![1]),
        "the copy the ray hit: {:?}",
        own.path.first()
    );
    assert_eq!(
        proposal.on.at, bench.shelf_i,
        "an untransformed pick is read at its own instance"
    );
    let _ = moved;
}

/// How far along y the union row places its union's copy, clear of
/// the posts and the shelf.
const UNION_SHIFT: f64 = -0.1;

/// **A pick on a UNION of placed instances is a target** read where the
/// ray met it: the union's copy, under the name the union carries for
/// its member's face (`RoleSeg::From`), and proposes a valid mate.
#[test]
fn a_pick_on_a_union_of_instances_stands_on_the_member() {
    let tol = Tol::witness();
    let bench = asm::bench("mateunion", tol);
    let session = asm::open_bench(&bench, tol);
    let mut doc = session.committed_doc().clone();
    let members = vec![
        asm::copy(&bench, &doc, bench.post_a),
        asm::copy(&bench, &doc, bench.post_b),
    ];
    let union = common::insert_into(
        &mut doc,
        pncad::document::Node::Union {
            members: pncad::document::Bodies::Spelled(members),
        },
        tol,
    );
    let post_b_read = doc.output(bench.post_b, 0);
    // The union is placed beside the posts, which stand on their own
    // pins: its copy is pinned `UNION_SHIFT` along y, clear of them.
    let (doc, union_copy) = common::inserted(
        &doc,
        crate::fixture::stood(pncad::document::BodyRead::plain(
            pncad::document::Operand::from(union),
        )),
        tol,
    );
    let doc = common::pin(
        &doc,
        union_copy,
        &pncad::document::Frame::translation([0.0, UNION_SHIFT, 0.0]),
        &bench.opts(),
        tol,
    );
    let mut ws = pncad::workspace::Workspace::open(&bench.dir).expect("the workspace opens");
    ws.resave(&doc, tol).expect("the assembly stores");
    let mut session = asm::open_bench(&bench, tol);
    session.pump();
    let post_top = asm::pick_face(
        &session,
        &asm::down_at(
            asm::POST_B_AT[0] + asm::POST_SECTION / 2.0,
            asm::POST_B_AT[1] + UNION_SHIFT + asm::POST_SECTION / 2.0,
        ),
    );
    assert_eq!(post_top.node, union_copy, "the ray met the union's copy");
    assert!(
        matches!(
            post_top.name.path.first(),
            Some(RoleSeg::From { read, .. }) if Some(*read) == post_b_read
        ),
        "the union names post_b's face as its member's: {:?}",
        post_top.name
    );
    let shelf_bottom = asm::shelf_underside(&session);
    let proposal = shelf_on(&session, &shelf_bottom, &post_top);
    assert_eq!(
        proposal.to.at, union_copy,
        "the target is read at the union's copy"
    );
    assert_eq!(
        proposal.to.name.clone(),
        post_top.name,
        "by the name the union carries for post_b's face"
    );
}

/// The quarter turn the circular row spins post_b by.
const QUARTER_TURN: f64 = core::f64::consts::FRAC_PI_2;

/// Pattern post_b twice about the world +y axis and answer the
/// family's placement.
///
/// A quarter turn about +y carries the post's top cap from `z =
/// POST_HEIGHT` facing +z to `x = POST_HEIGHT` facing +x, standing
/// copy 1 below the ground plane and clear of everything else the
/// bench draws — so one horizontal ray reaches that cap and nothing
/// else. The ROTATION is the point: a linear rule moves only the
/// origin, and a frame has two more channels.
fn spun_post(session: &mut DocSession, bench: &asm::Bench) -> RecipeNodeId {
    session.pump();
    let pattern = common::session_insert(
        session,
        SessionOp::AddPattern {
            input: bench.post_b.into(),
            count: 2,
            rule: PatternRuleSpec::Circular {
                through: common::len3([0.0; 3]),
                direction: common::scl3([0.0, 1.0, 0.0]),
                step: ang(QUARTER_TURN),
            },
        },
    );
    place_copies(session, bench, pattern, 2);
    pattern
}

/// The master entity a family member's pick names, from the `Member`
/// qualifier its head carries; a pick on no member names it itself.
fn master_face(pick: &FaceSelection) -> pncad::prelude::StableName {
    match pick.name.path.first() {
        Some(RoleSeg::Member { of, .. }) => (**of).clone(),
        _ => pick.name.clone(),
    }
}

/// **A ROTATED copy is a target whose rotation is applied once.** A
/// circular rule folds a rotation into the copy's placement, so the
/// shelf resting on the spun copy's cap faces the way that cap faces —
/// `-x`, under a cap facing `+x` — and not the way the master's does.
#[test]
fn a_circular_pattern_copy_is_a_target_whose_rotation_is_applied_once() {
    let tol = Tol::witness();
    let bench = asm::bench("matespun", tol);
    let mut session = asm::open_bench(&bench, tol);
    let pattern = spun_post(&mut session, &bench);

    // Member 0 stands where post_b does (the rule's identity step), so
    // a ray onto that cap meets two coincident copies; copy 1's cap
    // faces +x, under the ground plane, and is picked along -x.
    let half = asm::POST_SECTION / 2.0;
    let copy_one = asm::pick_face(
        &session,
        &common::along_x(-1.0, asm::POST_B_AT[1] + half, -(asm::POST_B_AT[0] + half)),
    );
    assert_eq!(
        copy_one.node, pattern,
        "the ray met the family's member: {copy_one:?}"
    );
    assert_eq!(
        member_at(copy_one.name.path.first()),
        Some(vec![1]),
        "the copy rides in the head: {:?}",
        copy_one.name
    );
    // The ray met the post's own top cap, on the spun member.
    assert_eq!(
        master_face(&copy_one),
        bench.post_top,
        "one part-local face, on the member"
    );

    let shelf_bottom = asm::shelf_underside(&session);
    let spun = shelf_on(&session, &shelf_bottom, &copy_one);

    // And it SOLVES: the shelf's underside rests on the spun copy's cap
    // in the world the evaluation draws, which is the pattern's
    // rotation being applied exactly once. The shelf's world mate comes
    // off, so the mate places the shelf against the spun copy.
    asm::unpin(&mut session, bench.shelf_i);
    let mate = common::commit_mate(&mut session, spun.op_into_target_space());
    let (doc, _) = session.landed_pair().expect("landed");
    assert!(
        common::solve(&session, doc, tol)
            .fault(mate.placement)
            .is_none(),
        "the spun member solves: {:?}",
        common::solve(&session, doc, tol).fault(mate.placement)
    );
    assert_faces_rest(&session, &copy_one, &shelf_bottom, "the spun copy");
    let (_, eval) = session.landed_pair().expect("landed");
    let underside = face_frame(eval, shelf_bottom.node, &shelf_bottom.name).expect("underside");
    assert!(
        (underside.axis.x + 1.0).abs() < 1e-9,
        "the shelf's underside faces -x, onto the spun cap: {:?}",
        underside.axis
    );
}

// ---- the member chain: a nested copy, and a read member ----

/// The step the nested assembly's two rules take, metres.
const NEST_STEP: f64 = 0.04;

/// **A nested-copy assembly, authored into the bench's workspace and
/// opened through the ordinary `Open` door** — which is what wires
/// the resolver, so the picks below are real rays onto real bodies.
///
/// A family over a read of member 1 of a family over the post: the
/// read between the two families chooses one inner member for the
/// outer family to replicate, which is the shape the picks below want.
/// A second read selecting inner member 0 stands beside it, so the same
/// document offers both picks this pair of rows wants: one on a nested
/// member, one on a member read. The families and the reads stand in a
/// space of their own, which no placement but the two below reaches the
/// world from: the world (A10) is the shelf, the post, the read of inner
/// member 1 and the read of outer member 1, each copied in at the
/// identity.
///
/// Returns the session and `(post instance, shelf instance, inner,
/// part, outer, loose part, outer member 1's read)`; the loose read is
/// authored but not placed.
fn nested_session(bench: &asm::Bench, tag: &str, tol: Tol) -> (DocSession, [RecipeNodeId; 7]) {
    use pncad::document::{
        BodyRead, Doc, DocumentId, Formula, Node, Operand, Placed, ProfileProgram, VarId, stand,
    };
    let mut doc: Doc<ProfileProgram> = Doc::empty(DocumentId::derive(tag), tol);
    let shelf_i = asm::instance_at(&mut doc, bench.shelf, asm::SHELF_AT, &bench.opts(), tol);
    let post_i = asm::instance_at(&mut doc, bench.post, asm::POST_B_AT, &bench.opts(), tol);
    let room = common::insert_into(&mut doc, Node::Space, tol);
    let room_space = doc.output(room, 0).expect("a new space defines its space");
    let room = Operand::Var(room_space);
    let family = |doc: Doc<ProfileProgram>, name: &str, of: RecipeNodeId, dir: [f64; 3]| {
        let (doc, k) = crate::fixture::index(doc, name, 2);
        let placed = Placed::of(
            &doc,
            Vec::new(),
            crate::fixture::slides_along(k, dir, NEST_STEP),
        );
        crate::fixture::insert(
            doc,
            Node::place(
                BodyRead::plain(Operand::Node(of)),
                room.clone(),
                placed.bundle(),
            ),
        )
    };
    let read = |doc: Doc<ProfileProgram>, of: RecipeNodeId, at: i64, into: Operand| {
        crate::fixture::insert(
            doc,
            Node::place(
                BodyRead::indexed(of, vec![Formula::count(at)]),
                into,
                stand().bundle(),
            ),
        )
    };
    let (doc, inner) = family(doc, "i", post_i, [1.0, 0.0, 0.0]);
    let (doc, part) = read(doc, inner, 1, room.clone());
    let (doc, outer) = family(doc, "j", part, [0.0, 1.0, 0.0]);
    let (doc, loose) = read(doc, inner, 0, room.clone());
    let (doc, outer_one) = read(doc, outer, 1, room.clone());
    // The instances stand on their own world mates; of the members, only
    // those no other drawn copy coincides with are copied into the
    // world: `part` (inner member 1, which outer member 0 repeats) and
    // outer member 1. Inner member 0 is where post_i itself is drawn.
    let world = Operand::Var(VarId::WORLD);
    let (doc, _) = crate::fixture::insert(
        doc,
        Node::place(
            BodyRead::plain(Operand::Node(part)),
            world.clone(),
            stand().bundle(),
        ),
    );
    let (doc, _) = crate::fixture::insert(
        doc,
        Node::place(
            BodyRead::plain(Operand::Node(outer_one)),
            world,
            stand().bundle(),
        ),
    );
    let doc = editor_core::test_support::assemble_world_copies(doc, tol);
    let mut ws = pncad::workspace::Workspace::open(&bench.dir).expect("the workspace opens");
    let path = ws.create(&doc, tol).expect("the nested assembly stores");
    let mut session = DocSession::inline(Doc::empty_derived("nested-boot", tol), tol);
    let outcome = session.perform(SessionOp::Open(path));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    // The room draws the members no read takes, where the world copies
    // stand: hidden, so the rows' rays meet the world's picture alone.
    let hidden = session.perform(SessionOp::SetSpaceHidden {
        space: room_space,
        hidden: true,
    });
    assert!(hidden.refusal.is_none(), "{:?}", hidden.refusal);
    (
        session,
        [post_i, shelf_i, inner, part, outer, loose, outer_one],
    )
}

/// The two picked faces rest in the world the evaluation draws: `a`
/// lies in `b`'s plane and the two face each other.
fn assert_faces_rest(session: &DocSession, a: &FaceSelection, b: &FaceSelection, what: &str) {
    let (_, eval) = session.landed_pair().expect("landed");
    let fa = face_frame(eval, a.node, &a.name).expect("the picked cap");
    let fb = face_frame(eval, b.node, &b.name).expect("the shelf's underside");
    assert!(
        (fa.origin - fb.origin).dot(fb.axis).abs() < 1e-9,
        "{what}: the mated faces lie in one plane: {:?} vs {:?}",
        fa.origin,
        fb.origin
    );
    assert!(
        (fa.axis + fb.axis).norm() < 1e-9,
        "{what}: the mated faces face each other: {:?} vs {:?}",
        fa.axis,
        fb.axis
    );
}

/// **A pick on a NESTED copy is a target, and the shelf rests on it.**
/// The target wears one `Member` per level, and the composed offset
/// `M₂(1) ∘ M₁(1)` is the solve's to apply.
#[test]
fn a_nested_copy_pick_is_a_target_and_seats() {
    let tol = Tol::witness();
    let bench = asm::bench("matenest", tol);
    let (mut session, [_post, shelf, inner, _part, outer, _loose, outer_one]) =
        nested_session(&bench, "gui4-nested-copy", tol);

    let nested = asm::pick_face(
        &session,
        &asm::down_at(
            asm::POST_B_AT[0] + NEST_STEP + asm::POST_SECTION / 2.0,
            asm::POST_B_AT[1] + NEST_STEP + asm::POST_SECTION / 2.0,
        ),
    );
    assert_eq!(
        nested.node,
        common::copy_of(session.committed_doc(), outer_one),
        "the ray met the outer family's member 1, placed"
    );
    let shelf_bottom = asm::shelf_underside(&session);
    let proposal = shelf_on(&session, &shelf_bottom, &nested);

    // The target wears one `Member` per level, outermost first, and is
    // read at the copy the ray met.
    assert_eq!(proposal.to.at, nested.node);
    let own = own_name(&proposal.to);
    assert_eq!(own.node, outer);
    assert_eq!(
        member_at(own.path.first()),
        Some(vec![1]),
        "the outer member rides in the head: {own:?}"
    );
    let Some(RoleSeg::Member { of, .. }) = own.path.first() else {
        panic!("the outer member rides in the head: {own:?}");
    };
    assert_eq!(of.node, inner, "and the inner name under it");
    assert_eq!(
        member_at(of.path.first()),
        Some(vec![1]),
        "the inner member rides in the inner head: {of:?}"
    );

    // The shelf's world mate off, so the mate places the shelf against
    // the nested copy.
    asm::unpin(&mut session, shelf);
    let mate = common::commit_mate(&mut session, proposal.op_into_target_space());
    assert_eq!(mate.placement, shelf);
    let (doc, _) = session.landed_pair().expect("landed");
    assert!(
        common::solve(&session, doc, tol)
            .fault(mate.placement)
            .is_none(),
        "the nested member solves: {:?}",
        common::solve(&session, doc, tol).fault(mate.placement)
    );
    assert_faces_rest(&session, &nested, &shelf_bottom, "a nested copy");
    for row in session.tree_rows() {
        assert_eq!(row.status, viewer::tree::RowStatus::Ok, "{row:?}");
    }
}

/// **A pick on a read of a family's member is a target, and the shelf
/// rests on it.** The read stands at the identity and renames nothing,
/// so the target's name is the family member's, read at the copy the
/// ray met.
#[test]
fn a_part_over_a_pattern_pick_is_a_target_and_seats() {
    let tol = Tol::witness();
    let bench = asm::bench("matepart", tol);
    let (mut session, [_post, shelf, inner, part, _outer, _loose, _]) =
        nested_session(&bench, "gui4-part-pick", tol);

    let picked = asm::pick_face(
        &session,
        &asm::down_at(
            asm::POST_B_AT[0] + NEST_STEP + asm::POST_SECTION / 2.0,
            asm::POST_B_AT[1] + asm::POST_SECTION / 2.0,
        ),
    );
    assert_eq!(
        picked.node,
        common::copy_of(session.committed_doc(), part),
        "the ray met the read's copy"
    );
    let shelf_bottom = asm::shelf_underside(&session);
    let proposal = shelf_on(&session, &shelf_bottom, &picked);

    // Read at the read's copy, under the FAMILY's own name: the read
    // carries every name verbatim, so the target's head is the family's
    // placement the member was minted by.
    assert_eq!(proposal.to.at, picked.node, "the copy the ray met");
    let own = own_name(&proposal.to);
    assert_eq!(own.node, inner);
    assert!(
        member_at(own.path.first()) == Some(vec![1]),
        "the copy rides in the head: {own:?}"
    );

    // The shelf's world mate off, so the mate places the shelf against
    // the read's copy.
    asm::unpin(&mut session, shelf);
    let mate = common::commit_mate(&mut session, proposal.op_into_target_space());
    assert_eq!(mate.placement, shelf);
    let (doc, _) = session.landed_pair().expect("landed");
    assert!(
        common::solve(&session, doc, tol)
            .fault(mate.placement)
            .is_none(),
        "the read member solves: {:?}",
        common::solve(&session, doc, tol).fault(mate.placement)
    );
    assert_faces_rest(&session, &picked, &shelf_bottom, "a read member");
    for row in session.tree_rows() {
        assert_eq!(row.status, viewer::tree::RowStatus::Ok, "{row:?}");
    }
}

/// **A refusal about a pick names its node as the panel does**, after
/// a rename that has not landed. The chrome speaks the landed document:
/// the mate panel says a held pick's node as it holds it, and the tool's
/// refusal is spoken again from that same document where the panel
/// draws it (`MateToolError::respoken`), so the two say one label — the
/// landed one, which the rename has not reached. Driven through
/// `MateTool::proposal`: both picks on one instance refuse `SamePick`.
/// Red if either says the rename before it lands.
#[test]
fn a_mate_refusal_names_the_picked_node_as_the_panel_does() {
    let tol = Tol::witness();
    let bench = asm::bench("mate-refusal", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (post_top, _) = asm::seat_picks(&session, &bench);
    let mut tool = MateTool::new();
    tool.pick(session.doc(), post_top.clone());
    tool.pick(session.doc(), post_top);
    // A pick is on the drawn copy, so the panel and the refusal say
    // the copy's placement.
    let renamed = session.perform(SessionOp::SetLabel {
        node: bench.post_b,
        label: Some(pncad::document::Label::new("post").expect("a label")),
    });
    assert!(renamed.refusal.is_none(), "{:?}", renamed.refusal);
    // Not pumped: the rename has not landed, so the tool reads the pair
    // from before it.
    let (doc, _) = session.landed_pair().expect("landed");
    assert_eq!(
        doc.label(bench.post_b),
        None,
        "the premise: the landed pair predates it"
    );
    let refused = tool
        .proposal(doc, asm::seat_choice())
        .expect_err("one instance is no pair");
    // The chrome speaks the landed document, so both say the label it
    // holds — none yet — and neither says the rename before it lands.
    let said = refused.respoken(doc).to_string();
    let spoken = format!(
        "InstantiatePart {}",
        test_utils::refusal::tag(bench.post_b.0.digest())
    );
    assert_eq!(
        said,
        format!(
            "both picks are on {spoken}, the copy the mate places; a mate relates it to \
             something else"
        )
    );
    let panel = tool.line(viewer::frame::landed_speaker(session.landed_pair()));
    assert!(
        panel.contains(&spoken) && !panel.contains("\"post\""),
        "the panel says the same node, by the landed label: {panel}"
    );
}

/// **A freshly mated copy moves only by the snap.** post_b stands in the
/// world by its values, turned 10° about the world's `z` and slid 2 cm
/// along `y`; the tool rests its top cap on the shelf's underside with no
/// value named. The gesture writes the orientation nearest where post_b
/// stands, the spin the least turn leaves and the slides at post_b's own
/// coordinates, so post_b keeps its turn and its `x` and `y`, and moves
/// along the mated direction alone, until the two faces rest.
///
/// *Breaks if* the gesture re-reads the values it keeps on the mate's
/// chart (the turn or a slide jumps), or writes the identity orientation
/// and a zero spin where the copy stands turned.
#[test]
fn a_freshly_mated_copy_moves_only_by_the_snap() {
    use pncad::document::{Coord, Dimension, Node, SlotId, TargetAxis};
    let tol = Tol::witness();
    let bench = asm::bench("matetool-snap", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (post_top, shelf_bottom) = asm::seat_picks(&session, &bench);
    let turn = 10.0_f64.to_radians();
    for (coord, value) in [
        (Coord::Slide(TargetAxis::Y), (Dimension::Length, 0.02)),
        (Coord::Spin(Some(TargetAxis::Z)), (Dimension::Angle, turn)),
    ] {
        let constraint = match session.committed_doc().node(bench.post_b) {
            Some(Node::InstantiatePart { constraints, .. }) => constraints
                .iter()
                .find(|c| c.value().is_some_and(|v| v.coord == coord))
                .map(|c| c.id),
            _ => None,
        }
        .expect("post_b's pin writes the freedom");
        let slot = match coord {
            Coord::Slide(_) => SlotId::ValueSlide { constraint },
            Coord::Spin(_) => SlotId::ValueSpin { constraint },
        };
        let outcome = session.perform(SessionOp::SetSlot {
            node: bench.post_b,
            slot,
            value: viewer::props::SlotValue::of(value.0, value.1).expect("a finite value"),
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    }
    session.pump();
    let pose = |session: &DocSession| {
        let (doc, _) = session.landed_pair().expect("landed");
        common::solve(session, doc, tol)
            .placement(doc, bench.post_b)
            .expect("post_b has a pose")
    };
    let before = pose(&session);
    let mut tool = MateTool::new();
    tool.pick(session.doc(), post_top.clone());
    tool.pick(session.doc(), shelf_bottom.clone());
    let (doc, _) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(
            doc,
            viewer::matetool::MateChoice {
                kind: pncad::document::VarKind::Plane,
                flip: true,
                values: Vec::new(),
            },
        )
        .expect("the seat proposes");
    common::commit_mate(&mut session, proposal.op());
    let after = pose(&session);
    for (c, (got, was)) in after.columns.iter().zip(&before.columns).enumerate() {
        for r in 0..3 {
            assert!(
                (got[r] - was[r]).abs() < 1e-12,
                "post_b keeps its turn: column {c} {got:?} was {was:?}"
            );
        }
    }
    for i in 0..2 {
        assert!(
            (after.translation[i] - before.translation[i]).abs() < 1e-12,
            "post_b keeps its {} : {:?} was {:?}",
            ["x", "y"][i],
            after.translation,
            before.translation
        );
    }
    assert_faces_rest(&session, &post_top, &shelf_bottom, "the snapped seat");
}

/// **A face on a split's second half is mated on that half**: a block
/// split at mid-height — the split takes the unplaced block over, so its
/// upper half is drawn where the block stood — its lower half placed
/// 0.1 m along x and rested by its underside on the upper half's top.
/// The mate's `on` side reads the face on the split's LOWER output by
/// its port — read alone, the split is two bodies and the commit
/// refuses — and the half is drawn seated on the top.
#[test]
fn a_face_on_a_splits_lower_half_is_mated_on_that_half() {
    let tol = Tol::witness();
    let size = [0.04, 0.02, 0.01];
    let (session, block, split) = common::drawn::split_box(size, tol);
    let below = pncad::select::SplitHalf::Below;
    let at = common::drawn::translated([0.1, 0.0, 0.0]);
    let (mut session, half) = common::drawn::half_placed(&session, split, below, &at, tol);
    assert!(
        !viewer::world::is_drawn(session.committed_doc(), block),
        "the split took the block over"
    );
    let underside = asm::pick_face(&session, &common::up_at(0.1, 0.0));
    let top = asm::pick_face(&session, &common::down_at(0.0, 0.0));
    assert_eq!(underside.node, half, "the ray met the lower half's copy");
    assert_eq!(
        top.node, split,
        "and the upper half's top, drawn by the split"
    );
    // The upper half is drawn in the block's own space, so the rest
    // moves the half into it by name.
    let mut mate = viewer::matetool::MateTool::new();
    mate.pick(session.doc(), underside);
    mate.pick(session.doc(), top);
    let proposal = {
        let (doc, _) = session.landed_pair().expect("landed");
        mate.proposal(
            doc,
            viewer::matetool::MateChoice {
                kind: pncad::document::VarKind::Plane,
                flip: true,
                values: Vec::new(),
            },
        )
        .expect("a rest proposes")
    };
    common::commit_mate(&mut session, proposal.op_into_target_space());
    let (x, y, z) = (size[0] / 2.0, size[1] / 2.0, size[2]);
    common::drawn::assert_drawn(
        &mut session,
        half,
        [0.1 - x, -y, z],
        [0.1 + x, y, 1.5 * z],
        size.iter().product::<f64>() / 2.0,
        "the lower half, seated on the block's top",
    );
}

/// **The snap moves a turned copy only by the snap**, wherever it is
/// turned: post_b turned about `z` and tilted about `x` by nine pairs,
/// some needing a turn of their own onto the shelf and an orientation
/// other than the identity, then rested by its top cap on the shelf's
/// underside with no value named. It keeps everything the mate leaves.
///
/// *Breaks if* the gesture writes an orientation or spin the chart reads
/// back as another turn, or slides it reads along other axes.
#[test]
fn the_snap_moves_a_turned_copy_only_by_the_snap() {
    use pncad::document::{Coord, Dimension, Node, SlotId, TargetAxis};
    let tol = Tol::witness();
    let mut bad = Vec::new();
    for (zt, xt) in [
        (100.0_f64, 0.0_f64),
        (10.0, 20.0),
        (100.0, 20.0),
        (190.0, -35.0),
        (45.0, 50.0),
        (-80.0, 10.0),
        (0.0, 170.0),
        (30.0, 120.0),
        (30.0, 180.0),
    ] {
        let bench = asm::bench(&format!("matetool-snap-{zt}-{xt}"), tol);
        let mut session = asm::open_bench(&bench, tol);
        let (post_top, shelf_bottom) = asm::seat_picks(&session, &bench);
        let mut ok = true;
        for (coord, value) in [
            (Coord::Slide(TargetAxis::Y), (Dimension::Length, 0.02)),
            (
                Coord::Spin(Some(TargetAxis::Z)),
                (Dimension::Angle, zt.to_radians()),
            ),
            (
                Coord::Spin(Some(TargetAxis::X)),
                (Dimension::Angle, xt.to_radians()),
            ),
        ] {
            let constraint = match session.committed_doc().node(bench.post_b) {
                Some(Node::InstantiatePart { constraints, .. }) => constraints
                    .iter()
                    .find(|c| c.value().is_some_and(|v| v.coord == coord))
                    .map(|c| c.id),
                _ => None,
            };
            let Some(constraint) = constraint else {
                println!("no value {coord:?} on post_b");
                ok = false;
                continue;
            };
            let slot = match coord {
                Coord::Slide(_) => SlotId::ValueSlide { constraint },
                Coord::Spin(_) => SlotId::ValueSpin { constraint },
            };
            let outcome = session.perform(SessionOp::SetSlot {
                node: bench.post_b,
                slot,
                value: viewer::props::SlotValue::of(value.0, value.1).expect("a finite value"),
            });
            assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        }
        if !ok {
            bad.push(format!("{zt},{xt}: chain not present"));
            continue;
        }
        session.pump();
        let pose = |session: &DocSession| {
            let (doc, _) = session.landed_pair().expect("landed");
            common::solve(session, doc, tol)
                .placement(doc, bench.post_b)
                .expect("post_b has a pose")
        };
        let before = pose(&session);
        let mut tool = MateTool::new();
        tool.pick(session.doc(), post_top.clone());
        tool.pick(session.doc(), shelf_bottom.clone());
        let (doc, _) = session.landed_pair().expect("landed");
        let proposal = tool
            .proposal(
                doc,
                viewer::matetool::MateChoice {
                    kind: pncad::document::VarKind::Plane,
                    flip: true,
                    values: Vec::new(),
                },
            )
            .expect("the seat proposes");
        common::commit_mate(&mut session, proposal.op());
        let after = pose(&session);
        // Expected: least turn of R0·z onto +z, composed after R0.
        let r0 = before.columns;
        let f = r0[2];
        let (c, k) = (f[2], [f[1], -f[0], 0.0]); // f × z
        let s = |e: [f64; 3]| -> [f64; 3] {
            let kxe = [
                k[1] * e[2] - k[2] * e[1],
                k[2] * e[0] - k[0] * e[2],
                k[0] * e[1] - k[1] * e[0],
            ];
            let kd = k[0] * e[0] + k[1] * e[1] + k[2] * e[2];
            [0, 1, 2].map(|i| e[i] * c + kxe[i] + k[i] * kd / (1.0 + c))
        };
        let want: Vec<[f64; 3]> = r0.iter().map(|&col| s(col)).collect();
        let dcol = after
            .columns
            .iter()
            .zip(&want)
            .flat_map(|(got, want)| got.iter().zip(want).map(|(g, w)| (g - w).abs()))
            .fold(0.0_f64, f64::max);
        let h = asm::POST_HEIGHT;
        let p = |fr: &pncad::document::Frame| -> [f64; 3] {
            [0, 1, 2].map(|i| fr.translation[i] + fr.columns[2][i] * h)
        };
        let (pb, pa) = (p(&before), p(&after));
        let dxy = (pa[0] - pb[0]).abs().max((pa[1] - pb[1]).abs());
        let orient = match session.committed_doc().node(bench.post_b) {
            Some(Node::InstantiatePart { constraints, .. }) => {
                constraints.iter().find_map(|c| c.mate().map(|m| m.orient))
            }
            _ => None,
        };
        println!("snap z={zt} x={xt}: orient {orient:?} turn err {dcol:e} point xy err {dxy:e}");
        // At the antipode (x = 180°) the expected least turn has no
        // formula; only the rest and the point are checked there.
        if (dcol > 1e-9 && 1.0 + c > 1e-9) || dxy > 1e-9 {
            bad.push(format!("{zt},{xt}: turn err {dcol:e}, point err {dxy:e}"));
        }
        assert_faces_rest(&session, &post_top, &shelf_bottom, "r1 snapped seat");
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
