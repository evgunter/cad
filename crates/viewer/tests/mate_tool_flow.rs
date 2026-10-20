//! **The mate tool, replayed headlessly** (GUI-4 deliverable 4): two
//! sequential picks through the real cursor path, the admission
//! exposure, the typed refusals, one committed action (the mate and
//! its values), and the pick-vanish degradation.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::asm;
use common::{ang, len, scl};
use pncad::document::{ClassAdmission, MateSide, RecipeNodeId};
use pncad::geom_core::Tol;
use pncad::select::{ContactClass, RoleSeg, face_frame};
use viewer::matetool::{MateTool, MateToolError, MateToolEvent, MateToolState, admitted_classes};
use viewer::session::{DocSession, FaceSelection, PatternRuleSpec, SessionOp};

#[test]
fn the_admission_exposure_is_the_kernels_own_table() {
    let classes = admitted_classes();
    assert_eq!(classes.len(), 2, "the two nameable classes");
    assert_eq!(classes[0].class, ContactClass::Rest);
    assert_eq!(classes[0].admission, ClassAdmission::Mints);
    assert_eq!(classes[1].class, ContactClass::Tangent);
    assert!(
        matches!(
            classes[1].admission,
            ClassAdmission::NoAtRestRecord { why } if why.contains("at rest")
        ),
        "Tangent solves but carries no at-rest record, in the table's own words"
    );
}

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
    // post_b's world pin off, so the mate places it: its bundle is the
    // empty one.
    asm::unpin(&mut session, bench.post_b);

    // The proposal: each side IS the picked face read as a plane, by the
    // part's own name, resolved at every evaluation.
    let (doc, _) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, asm::seat_choice())
        .expect("the seat proposes");
    assert_eq!(proposal.mate.class, ContactClass::Rest);
    assert_eq!(proposal.admission, ClassAdmission::Mints);
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
        crate::fixture::rest(proposal.on.clone(), proposal.to.clone(), ContactClass::Rest),
        "two planes, the target flipped: the faces rest against each other"
    );

    // One committed action, through the session's one door, into
    // post_b's bundle: the mate and its values.
    let mate = common::commit_mate(&mut session, proposal.op());
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
/// were it reached. The freedom's sentence is the kernel's reading
/// (`Coord::admitted_by`).
#[test]
fn the_tool_refuses_the_choices_own_faults_before_any_pick() {
    use pncad::document::{Coord, FreedomRefusal, TargetAxis, VarKind};
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
    let Err(MateToolError::NotAFreedom {
        coord,
        kind,
        refusal,
    }) = tool.proposal(doc, choice)
    else {
        panic!("an axis mate leaves no slide along x");
    };
    assert_eq!(
        (coord, kind, refusal),
        (
            Coord::Slide(TargetAxis::X),
            VarKind::Axis,
            FreedomRefusal::NotAFreedom
        )
    );
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
    tool.pick(session.doc(), shelf_bottom);
    let held_line = tool.state().line(session.doc());

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
    let said = format!(
        "face of InstantiatePart {}",
        test_utils::refusal::tag(bench.shelf_i.0.digest())
    );
    assert!(held_line.ends_with(&format!("to: {said}")), "{held_line:?}");
    assert_eq!(
        events[0].to_string(),
        format!("the to pick (a {said}) no longer resolves; the tool dropped it")
    );
    match tool.state() {
        MateToolState::One(held) => assert_eq!(held, &post_top),
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

/// **`pattern`'s copies placed in the world beside post_b** (A10): post_b
/// is its own placement and draws copy 0, which the pattern repeats in
/// place, so each later copy is projected and placed through the
/// session's part door. Answers post_b, then the projections, in
/// instance order.
fn place_copies(
    session: &mut DocSession,
    bench: &asm::Bench,
    pattern: RecipeNodeId,
    count: i64,
) -> Vec<RecipeNodeId> {
    let parts = std::iter::once(bench.post_b)
        .chain((1..count).map(|index| {
            common::session_insert(
                session,
                SessionOp::AddPart {
                    of: pattern,
                    select: viewer::session::PartSelectSpec::Instance(index),
                },
            )
        }))
        .collect();
    session.pump();
    parts
}

/// Pattern post_b twice along +x, each copy placed ([`place_copies`]),
/// and answer the pattern node and the copies' projections.
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
/// with every value zero, so the shelf is pinned and its underside
/// rests on the target's face.
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
    for (coord, by) in &mut choice.values {
        *by = match coord {
            pncad::document::Coord::Spin => ang(0.0),
            pncad::document::Coord::Slide(_) => len(0.0),
        };
    }
    tool.proposal(doc, choice)
        .expect("the shelf's rest proposes")
}

/// **The name a target pick on a placement's copy carries**, the copy's
/// wrapper taken off: the copied body's own name for the face.
fn own_name(side: &pncad::document::SitedFace) -> pncad::prelude::StableName {
    let name = side.name.clone().into_name();
    let (_, own) = name.copy_of().expect("a pick on a copy is wrapped");
    own.clone()
}

/// **A pattern-placed copy is a target, and the shelf rests on it.** The
/// tool reads the target where it was picked — the copy's placement,
/// under the pattern's `Instance(i)`-headed name — and the solve reads
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
        common::copy_of(session.committed_doc(), parts[1]),
        "the ray met copy 1's placement"
    );
    let shelf_bottom = asm::shelf_underside(&session);
    let proposal = shelf_on(&session, &shelf_bottom, &copy_one);
    assert_eq!(
        proposal.placement, bench.shelf_i,
        "the mate places the shelf"
    );

    // The target is read at the copy's placement, `Instance(i)`-headed
    // on the pattern node under the placement's wrap.
    assert_eq!(proposal.to.at, copy_one.node);
    let own = own_name(&proposal.to);
    assert_eq!(own.node, pattern);
    assert!(
        matches!(own.path.first(), Some(RoleSeg::Instance { i: 1, .. })),
        "the copy rides in the head: {:?}",
        own.path.first()
    );

    // And it SOLVES: the shelf's world pin comes off, the mate places
    // the shelf on copy 1, and the two picked faces rest in the world
    // the evaluation draws. A tool that read the master's face would
    // land the shelf one step away from copy 1.
    asm::unpin(&mut session, bench.shelf_i);
    let mate = common::commit_mate(&mut session, proposal.op());
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
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
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
        common::copy_of(session.committed_doc(), parts[1])
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
        matches!(own.path.first(), Some(RoleSeg::Instance { i: 1, .. })),
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
        pncad::document::Node::place(vec![pncad::document::BodyRead::plain(
            pncad::document::Operand::from(union),
        )]),
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
            post_top.name.copy_of().and_then(|(_, own)| own.path.first()),
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
        proposal.to.name.clone().into_name(),
        post_top.name,
        "by the name the union carries for post_b's face"
    );
}

/// The quarter turn the circular row spins post_b by.
const QUARTER_TURN: f64 = core::f64::consts::FRAC_PI_2;

/// Pattern post_b twice about the world +y axis and answer the
/// pattern node.
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

/// The master entity a pattern copy's pick names, from the
/// `Instance(i)` qualifier its head carries under the copy's wrap.
fn master_face(pick: &FaceSelection) -> pncad::prelude::StableName {
    let (_, own) = pick.name.copy_of().expect("a pick on a copy is wrapped");
    match own.path.first() {
        Some(RoleSeg::Instance { of, .. }) => (**of).clone(),
        other => panic!("an Instance-qualified head: {other:?}"),
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

    // Copy 0 stands where post_b does (the rule's identity step), and
    // post_b draws it: its cap faces +z and is picked from above.
    let half = asm::POST_SECTION / 2.0;
    let copy_zero = asm::pick_face(
        &session,
        &asm::down_at(asm::POST_B_AT[0] + half, asm::POST_B_AT[1] + half),
    );
    // Copy 1's cap faces +x, under the ground plane: picked along -x.
    let copy_one = asm::pick_face(
        &session,
        &common::along_x(-1.0, asm::POST_B_AT[1] + half, -(asm::POST_B_AT[0] + half)),
    );
    let doc = session.committed_doc();
    assert_eq!(copy_zero.node, bench.post_b, "post_b draws copy 0");
    assert!(
        matches!(
            doc.node(viewer::world::seat_of(doc, copy_one.node)),
            Some(pncad::document::Node::Part { of, .. })
                if doc.operation_of(*of) == Some(pattern)
        ),
        "the ray met a placed copy of the pattern: {copy_one:?}"
    );
    assert!(
        matches!(
            copy_one
                .name
                .copy_of()
                .and_then(|(_, own)| own.path.first()),
            Some(RoleSeg::Instance { i: 1, .. })
        ),
        "the copy rides in the head: {:?}",
        copy_one.name
    );
    // The two rays met ONE part-local face on two copies.
    assert_eq!(
        master_face(&copy_one),
        copy_zero.name,
        "one part-local face, two copies"
    );

    let shelf_bottom = asm::shelf_underside(&session);
    let spun = shelf_on(&session, &shelf_bottom, &copy_one);

    // And it SOLVES: the shelf's underside rests on the spun copy's cap
    // in the world the evaluation draws, which is the pattern's
    // rotation being applied exactly once. The shelf's world pin comes
    // off, so the mate places the shelf against the spun copy.
    asm::unpin(&mut session, bench.shelf_i);
    let mate = common::commit_mate(&mut session, spun.op());
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

// ---- the member chain: a nested copy, and a `Part`-selected one ----

/// The step the nested assembly's two rules take, metres.
const NEST_STEP: f64 = 0.04;

/// **A nested-copy assembly, authored into the bench's workspace and
/// opened through the ordinary `Open` door** — which is what wires
/// the resolver, so the picks below are real rays onto real bodies.
///
/// `Pattern` over `Part { Instance(1) }` over `Pattern` over the
/// post: the shape a nested copy is reachable through, since a
/// pattern's value is many bodies and a pattern's input is one. A
/// second `Part` selecting inner copy 0 is placed beside it, so the
/// same document offers both picks this pair of rows wants: one on a
/// nested copy, one on a `Part` over a pattern. The world (A10) is the
/// shelf, the outer pattern's two copies (a `Part` each) and the loose
/// `Part`.
///
/// Returns the session and `(post instance, shelf instance, inner,
/// part, outer, loose part, outer copy 1's projection)`; the loose part
/// is authored but not placed.
fn nested_session(bench: &asm::Bench, tag: &str, tol: Tol) -> (DocSession, [RecipeNodeId; 7]) {
    use pncad::document::{
        Doc, DocumentId, Formula, Node, PartSelect, PatternKind, ProfileProgram,
    };
    let mut doc: Doc<ProfileProgram> = Doc::empty(DocumentId::derive(tag), tol);
    let shelf_i = asm::instance_at(&mut doc, bench.shelf, asm::SHELF_AT, &bench.opts(), tol);
    let post_i = asm::instance_at(&mut doc, bench.post, asm::POST_B_AT, &bench.opts(), tol);
    let rule = |dir: [f64; 3]| PatternKind::Linear {
        direction: dir.map(scl),
        spacing: len(NEST_STEP),
    };
    let inner = common::insert_into(
        &mut doc,
        Node::Pattern {
            input: post_i.into(),
            count: Formula::count(2),
            kind: rule([1.0, 0.0, 0.0]),
        },
        tol,
    );
    let part = common::insert_into(
        &mut doc,
        Node::Part {
            of: inner.into(),
            select: PartSelect::Instance(Formula::count(1)),
        },
        tol,
    );
    let outer = common::insert_into(
        &mut doc,
        Node::Pattern {
            input: part.into(),
            count: Formula::count(2),
            kind: rule([0.0, 1.0, 0.0]),
        },
        tol,
    );
    let loose = common::insert_into(
        &mut doc,
        Node::Part {
            of: inner.into(),
            select: PartSelect::Instance(Formula::count(0)),
        },
        tol,
    );
    let outer_copies = [0, 1].map(|index| {
        common::insert_into(
            &mut doc,
            Node::Part {
                of: outer.into(),
                select: PartSelect::Instance(Formula::count(index)),
            },
            tol,
        )
    });
    // The instances stand on their own world pins; of the copies, only
    // those no other drawn copy coincides with are placed: `part` (inner
    // copy 1, which outer copy 0 repeats) and outer copy 1. Inner copy 0
    // is where post_i itself is drawn.
    let _ = loose;
    for body in [part, outer_copies[1]] {
        doc = common::placed_read(&doc, body.into(), &bench.opts(), tol).0;
    }
    let mut ws = pncad::workspace::Workspace::open(&bench.dir).expect("the workspace opens");
    let path = ws.create(&doc, tol).expect("the nested assembly stores");
    let mut session = DocSession::inline(Doc::empty_derived("nested-boot", tol), tol);
    let outcome = session.perform(SessionOp::Open(path));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    (
        session,
        [post_i, shelf_i, inner, part, outer, loose, outer_copies[1]],
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
/// The target wears one `Instance(i)` per level, and the composed
/// offset `M₂(1) ∘ M₁(1)` is the solve's to apply.
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
        "the ray met the outer pattern's copy 1, placed"
    );
    let shelf_bottom = asm::shelf_underside(&session);
    let proposal = shelf_on(&session, &shelf_bottom, &nested);

    // The target wears one `Instance(i)` per level, outermost first,
    // and is read at the copy the ray met.
    assert_eq!(proposal.to.at, nested.node);
    let own = own_name(&proposal.to);
    assert_eq!(own.node, outer);
    let Some(RoleSeg::Instance { i: 1, of }) = own.path.first() else {
        panic!("the outer copy rides in the head: {own:?}");
    };
    assert_eq!(of.node, inner, "and the inner name under it");
    assert!(
        matches!(of.path.first(), Some(RoleSeg::Instance { i: 1, .. })),
        "the inner copy rides in the inner head: {of:?}"
    );

    // The shelf's world pin off, so the mate places the shelf against
    // the nested copy.
    asm::unpin(&mut session, shelf);
    let mate = common::commit_mate(&mut session, proposal.op());
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

/// **A pick on a `Part` over a pattern is a target, and the shelf rests
/// on it.** The `Part` is pose-neutral and renames nothing, so the
/// target's name is the pattern's copy's, read at the copy the ray met.
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
        "the ray met the Part's copy"
    );
    let shelf_bottom = asm::shelf_underside(&session);
    let proposal = shelf_on(&session, &shelf_bottom, &picked);

    // Read at the `Part`'s copy, under the PATTERN's own name: the Part
    // carries every name verbatim, so the target's head is the pattern
    // node the copy was minted by.
    assert_eq!(proposal.to.at, picked.node, "the copy the ray met");
    let own = own_name(&proposal.to);
    assert_eq!(own.node, inner);
    assert!(
        matches!(own.path.first(), Some(RoleSeg::Instance { i: 1, .. })),
        "the copy rides in the head: {own:?}"
    );

    // The shelf's world pin off, so the mate places the shelf against
    // the Part's copy.
    asm::unpin(&mut session, shelf);
    let mate = common::commit_mate(&mut session, proposal.op());
    assert_eq!(mate.placement, shelf);
    let (doc, _) = session.landed_pair().expect("landed");
    assert!(
        common::solve(&session, doc, tol)
            .fault(mate.placement)
            .is_none(),
        "the Part-selected member solves: {:?}",
        common::solve(&session, doc, tol).fault(mate.placement)
    );
    assert_faces_rest(&session, &picked, &shelf_bottom, "a Part-selected copy");
    for row in session.tree_rows() {
        assert_eq!(row.status, viewer::tree::RowStatus::Ok, "{row:?}");
    }
}

/// **A refusal about a pick names its node as the panel does**, after
/// a rename that has not landed. The mate panel says a held pick is the
/// face of the node as the session's document speaks it; the tool reads
/// the landed pair, so its refusal is spoken again from that same
/// document where the panel draws it (`MateToolError::respoken`), and
/// the two say one label. Driven through `MateTool::proposal`: both
/// picks on one instance refuse `SamePick`. Red if the refusal says the
/// label from before the rename, or the instance by its tag alone.
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
    let said = refused.respoken(session.doc()).to_string();
    let spoken = format!(
        "InstantiatePart \"post\" ({})",
        test_utils::refusal::tag(bench.post_b.0.digest())
    );
    assert_eq!(
        said,
        format!(
            "both picks are on {spoken}, the copy the mate places; a mate relates it to \
             something else"
        )
    );
    let panel = tool.state().line(session.doc());
    assert!(
        panel.contains(&spoken),
        "the panel says the same node: {panel}"
    );
}
