//! **The mate tool, replayed headlessly** (GUI-4 deliverable 4): two
//! sequential picks through the real cursor path, the admission
//! exposure, the typed refusals, exactly one committed edit, and the
//! pick-vanish degradation.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::asm;
use common::{ang, len, scl};
use pncad::document::{ClassAdmission, MateSide, RecipeNodeId, solve_document};
use pncad::geom_core::{Point3, Tol};
use pncad::select::{ContactClass, Ray, RoleSeg, face_frame};
use viewer::matetool::{MateTool, MateToolError, MateToolEvent, MateToolState, admitted_classes};
use viewer::session::{DocSession, FaceSelection, PatternRuleSpec, SessionOp};

/// Pick a face through the session's real ray path.
fn pick_at(session: &DocSession, ray: &Ray) -> FaceSelection {
    let index = asm::index_of(session);
    let (_, eval) = session.landed_pair().expect("landed");
    index
        .face_at_for(eval, ray, &session.display_view())
        .expect("the pick answers")
        .expect("the ray hits")
}

/// The two picks every committing row starts from: post_b's top cap,
/// then the shelf's underside.
fn two_picks(session: &DocSession, bench: &asm::Bench) -> (FaceSelection, FaceSelection) {
    let post_top = pick_at(
        session,
        &asm::down_at(
            asm::POST_B_AT[0] + asm::POST_SECTION / 2.0,
            asm::POST_B_AT[1] + asm::POST_SECTION / 2.0,
        ),
    );
    assert_eq!(post_top.node, bench.post_b);
    let shelf_bottom = pick_at(
        session,
        &asm::up_at(
            asm::SHELF_AT[0] + asm::SHELF_LENGTH / 2.0,
            asm::SHELF_AT[1] + asm::SHELF_DEPTH / 2.0,
        ),
    );
    assert_eq!(shelf_bottom.node, bench.shelf_i);
    (post_top, shelf_bottom)
}

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
    let (post_top, shelf_bottom) = two_picks(&session, &bench);

    // The two sequential picks, held in tool state.
    let mut tool = MateTool::new();
    assert_eq!(tool.state(), &MateToolState::Idle);
    tool.pick(post_top.clone());
    assert!(matches!(tool.state(), MateToolState::One(_)));
    tool.pick(shelf_bottom.clone());
    assert!(matches!(tool.state(), MateToolState::Two { .. }));

    // The proposal: frames derived through `face_frame`, pulled back
    // into part coordinates through each instance's placement.
    let (doc, eval) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, eval, tol, asm::seat())
        .expect("the seat proposes");
    assert_eq!(proposal.class, ContactClass::Rest);
    assert_eq!(proposal.admission, ClassAdmission::Mints);
    // Pick a is post_b's top cap: its derived frame is in the POST's
    // own coordinates (the placement was divided out), so its origin
    // sits on the part's top plane, not at the instance's world spot.
    assert!(
        (proposal.alignment.a.origin[2] - asm::POST_HEIGHT).abs() < 1e-12,
        "part coordinates: {:?}",
        proposal.alignment.a.origin
    );

    // EXACTLY one committed edit, through the session's one door.
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    session.pump();

    // The placement is SOLVED from the mate: the two picked frames
    // now coincide in world space (composition through the solved
    // poses reproduces the coincidence the alignment declares).
    let (doc, _) = session.landed_pair().expect("landed");
    let poses = solve_document(doc, tol);
    let placed_a = poses
        .placement(doc, bench.post_b)
        .expect("post_b is placed")
        .affine::<f64>();
    let placed_b = poses
        .placement(doc, bench.shelf_i)
        .expect("the shelf is placed")
        .affine::<f64>();
    let [ax, ay, az] = proposal.alignment.a.origin;
    let [bx, by, bz] = proposal.alignment.b.origin;
    let world_a = placed_a.transform_point(Point3::new(ax, ay, az));
    let world_b = placed_b.transform_point(Point3::new(bx, by, bz));
    for (got, want) in [
        (world_a.x, world_b.x),
        (world_a.y, world_b.y),
        (world_a.z, world_b.z),
    ] {
        assert!(
            (got - want).abs() < 1e-9,
            "the mated frames coincide: {world_a:?} vs {world_b:?}"
        );
    }
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
    let (doc, eval) = session.landed_pair().expect("landed");
    let (post_top, shelf_bottom) = two_picks(&session, &bench);

    // No picks yet: NotTwoPicks.
    let tool = MateTool::new();
    assert!(matches!(
        tool.proposal(doc, eval, tol, asm::seat()),
        Err(MateToolError::NotTwoPicks)
    ));

    // Both picks on ONE instance: SamePick, refused at the tool
    // rather than authored into the self-mate the solve would refuse.
    let mut tool = MateTool::new();
    tool.pick(post_top.clone());
    tool.pick(post_top.clone());
    assert!(matches!(
        tool.proposal(doc, eval, tol, asm::seat()),
        Err(MateToolError::SamePick { head }) if head == bench.post_b
    ));

    // A vanished pick refuses typed when asked WITHOUT a reconcile
    // (the reconcile row below is the graceful path): the deleted
    // instance is no longer an instance, and the refusal says so.
    let mut tool = MateTool::new();
    tool.pick(post_top);
    tool.pick(shelf_bottom);
    let mut session2 = asm::open_bench(&bench, tol);
    session2.perform(SessionOp::DeleteNode {
        node: bench.shelf_i,
    });
    session2.pump();
    let (doc2, eval2) = session2.landed_pair().expect("landed");
    assert!(matches!(
        tool.proposal(doc2, eval2, tol, asm::seat()),
        Err(MateToolError::NotAnInstancePick {
            side: MateSide::B,
            ..
        })
    ));
}

#[test]
fn a_vanished_pick_degrades_the_tool_one_step_typed() {
    let tol = Tol::witness();
    let bench = asm::bench("matevanish", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (post_top, shelf_bottom) = two_picks(&session, &bench);
    let mut tool = MateTool::new();
    tool.pick(post_top.clone());
    tool.pick(shelf_bottom);

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
            side: MateSide::B,
            ..
        }
    ));
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
            side: MateSide::A,
            ..
        }
    ));
    assert_eq!(tool.state(), &MateToolState::Idle);
}

/// The linear spacing the pattern rows step post_b by — wide enough
/// that copy 1 stands clear of copy 0 and of the shelf, so a ray
/// reaches exactly one of them.
const PATTERN_STEP: f64 = 0.04;

/// Pattern post_b twice along +x and answer the pattern node.
fn patterned_post(session: &mut DocSession, bench: &asm::Bench) -> RecipeNodeId {
    let outcome = session.perform(SessionOp::AddPattern {
        input: bench.post_b,
        count: 2,
        rule: PatternRuleSpec::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(PATTERN_STEP),
        },
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    *outcome
        .committed
        .first()
        .and_then(|edit| match edit {
            pncad::document::DocEdit::InsertNode { .. } => session.committed_doc().roots().last(),
            _ => None,
        })
        .expect("the pattern node is a root")
}

/// A pick on pattern copy `i` of the patterned post's top cap.
fn copy_pick(session: &DocSession, i: u32) -> FaceSelection {
    pick_at(
        session,
        &asm::down_at(
            asm::POST_B_AT[0] + f64::from(i) * PATTERN_STEP + asm::POST_SECTION / 2.0,
            asm::POST_B_AT[1] + asm::POST_SECTION / 2.0,
        ),
    )
}

/// **A pattern-placed head is a member and mates.** The A11 vocabulary
/// admits `Pattern` + `Instance(i)` over a live instance and the solve
/// places it; this row is the viewer authoring one by picking.
#[test]
fn a_pattern_placed_pick_mates_through_an_instance_headed_reference() {
    let tol = Tol::witness();
    let bench = asm::bench("matepattern", tol);
    let mut session = asm::open_bench(&bench, tol);
    let pattern = patterned_post(&mut session, &bench);

    let copy_one = copy_pick(&session, 1);
    assert_eq!(copy_one.node, pattern, "the ray met the pattern's body");
    let shelf_bottom = pick_at(
        &session,
        &asm::up_at(
            asm::SHELF_AT[0] + asm::SHELF_LENGTH / 2.0,
            asm::SHELF_AT[1] + asm::SHELF_DEPTH / 2.0,
        ),
    );
    let mut tool = MateTool::new();
    tool.pick(copy_one.clone());
    tool.pick(shelf_bottom.clone());
    let (doc, eval) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, eval, tol, asm::seat())
        .expect("a pattern copy is a member");

    // The reference is `Instance(i)`-headed, on the pattern node —
    // which is what makes it a MEMBER rather than a bare pattern head.
    assert_eq!(proposal.a.node, pattern);
    assert!(
        matches!(
            proposal.a.path.first(),
            Some(RoleSeg::Instance { i: 1, .. })
        ),
        "the copy rides in the head: {:?}",
        proposal.a.path.first()
    );

    // The alignment is in the MASTER's part coordinates — the same
    // numbers copy 0 would author, because the pattern's derived
    // offset is the solve's to apply and not the tool's to bake in.
    assert!(
        (proposal.alignment.a.origin[2] - asm::POST_HEIGHT).abs() < 1e-12,
        "part coordinates: {:?}",
        proposal.alignment.a.origin
    );
    let mut zero = MateTool::new();
    zero.pick(copy_pick(&session, 0));
    zero.pick(shelf_bottom.clone());
    let from_zero = zero
        .proposal(doc, eval, tol, asm::seat())
        .expect("copy 0 is a member too");
    assert_eq!(
        from_zero.alignment.a.origin, proposal.alignment.a.origin,
        "every copy of one pattern is the same part"
    );

    // And it SOLVES: one committed edit, then the two picked faces
    // coincide in the world the evaluation draws. A tool that folded
    // the pattern offset into the authored frame would land copy 1 one
    // step away from the shelf.
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    session.pump();
    let (doc, eval) = session.landed_pair().expect("landed");
    assert!(
        solve_document(doc, tol).fault(pattern).is_none(),
        "the pattern member solves"
    );
    let a = face_frame(eval, copy_one.node, &copy_one.name).expect("copy 1's cap");
    let b = face_frame(eval, shelf_bottom.node, &shelf_bottom.name).expect("the shelf's underside");
    for (got, want) in [
        (a.origin.x, b.origin.x),
        (a.origin.y, b.origin.y),
        (a.origin.z, b.origin.z),
    ] {
        assert!(
            (got - want).abs() < 1e-9,
            "the mated faces meet in the world: {:?} vs {:?}",
            a.origin,
            b.origin
        );
    }
    for row in session.tree_rows() {
        assert_eq!(row.status, viewer::tree::RowStatus::Ok, "{row:?}");
    }
}

/// **What is still outside the vocabulary is still refused.** A
/// pattern over something that is not a live instance carries no
/// member, however `Instance(i)`-qualified its faces are.
#[test]
fn a_pattern_over_a_non_instance_is_still_not_an_instance_pick() {
    let tol = Tol::witness();
    let bench = asm::bench("matepatternnon", tol);
    let mut session = asm::open_bench(&bench, tol);
    // A transform between the instance and the pattern: the pattern's
    // input is a `Transform`, not an `InstantiatePart`, which is one
    // of the heads `member_of` declines.
    let moved = session.perform(SessionOp::AddTransform {
        input: bench.post_b,
        translation: [len(0.0), len(0.0), len(0.0)],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(0.0),
    });
    assert!(moved.refusal.is_none(), "{:?}", moved.refusal);
    session.pump();
    let moved = *session.committed_doc().roots().last().expect("a root");
    let outcome = session.perform(SessionOp::AddPattern {
        input: moved,
        count: 2,
        rule: PatternRuleSpec::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(PATTERN_STEP),
        },
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let pattern = *session.committed_doc().roots().last().expect("a root");

    let copy_one = copy_pick(&session, 1);
    assert_eq!(copy_one.node, pattern);
    let shelf_bottom = pick_at(
        &session,
        &asm::up_at(
            asm::SHELF_AT[0] + asm::SHELF_LENGTH / 2.0,
            asm::SHELF_AT[1] + asm::SHELF_DEPTH / 2.0,
        ),
    );
    let mut tool = MateTool::new();
    tool.pick(copy_one);
    tool.pick(shelf_bottom);
    let (doc, eval) = session.landed_pair().expect("landed");
    assert!(
        matches!(
            tool.proposal(doc, eval, tol, asm::seat()),
            Err(MateToolError::NotAnInstancePick {
                side: MateSide::A,
                node
            }) if node == pattern
        ),
        "a pattern over a transform carries no member"
    );
}
