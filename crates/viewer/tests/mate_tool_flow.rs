//! **The mate tool, replayed headlessly** (GUI-4 deliverable 4): two
//! sequential picks through the real cursor path, the admission
//! exposure, the typed refusals, exactly one committed edit, and the
//! pick-vanish degradation.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::asm;
use pncad::document::{ClassAdmission, MateSide, solve_document};
use pncad::geom_core::{Point3, Tol};
use pncad::select::{ContactClass, Ray};
use viewer::matetool::{MateTool, MateToolError, MateToolEvent, MateToolState, admitted_classes};
use viewer::session::{DocSession, FaceSelection, SessionOp};

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
        Err(MateToolError::SamePick { instance }) if instance == bench.post_b
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
