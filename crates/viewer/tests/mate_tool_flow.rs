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
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::select::{ContactClass, Ray, RoleSeg, face_frame};
use viewer::matetool::{MateTool, MateToolError, MateToolEvent, MateToolState, admitted_classes};
use viewer::session::{DatumSpec, DocSession, FaceSelection, PatternRuleSpec, SessionOp};

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
    let pattern = common::insert(
        session,
        SessionOp::AddPattern {
            input: bench.post_b,
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(PATTERN_STEP),
            },
        },
    );
    session.pump();
    pattern
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
    assert_eq!(proposal.a.name.node, pattern);
    assert!(
        matches!(
            proposal.a.name.path.first(),
            Some(RoleSeg::Instance { i: 1, .. })
        ),
        "the copy rides in the head: {:?}",
        proposal.a.name.path.first()
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
    let moved = common::insert(
        &mut session,
        SessionOp::AddTransform {
            input: bench.post_b,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    session.pump();
    let pattern = common::insert(
        &mut session,
        SessionOp::AddPattern {
            input: moved,
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(PATTERN_STEP),
            },
        },
    );
    session.pump();

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
    let axis = common::insert(
        session,
        SessionOp::AddDatum {
            datum: DatumSpec::Axis {
                origin: common::len3([0.0; 3]),
                direction: common::scl3([0.0, 1.0, 0.0]),
            },
        },
    );
    session.pump();
    let pattern = common::insert(
        session,
        SessionOp::AddPattern {
            input: bench.post_b,
            count: 2,
            rule: PatternRuleSpec::Circular {
                axis,
                step: ang(QUARTER_TURN),
            },
        },
    );
    session.pump();
    pattern
}

/// The master entity a pattern copy's pick names, from the
/// `Instance(i)` qualifier its head carries.
fn master_face(pick: &FaceSelection) -> pncad::prelude::StableName {
    match pick.name.path.first() {
        Some(RoleSeg::Instance { of, .. }) => (**of).clone(),
        other => panic!("an Instance-qualified head: {other:?}"),
    }
}

/// **A ROTATED copy authors its master's frame, in every channel.**
/// The linear rows move only the alignment's origin; a circular rule
/// folds a rotation into the pattern-derived offset, so reading the
/// copy's own pose would corrupt `axis` and `reference` as well. Both
/// copies of one spun pattern author one frame — the master's.
#[test]
fn a_circular_pattern_copy_authors_the_masters_unrotated_frame() {
    let tol = Tol::witness();
    let bench = asm::bench("matespun", tol);
    let mut session = asm::open_bench(&bench, tol);
    let pattern = spun_post(&mut session, &bench);

    // Copy 0 stands where post_b did (the rule's identity step): its
    // cap faces +z and is picked from above.
    let half = asm::POST_SECTION / 2.0;
    let copy_zero = pick_at(
        &session,
        &asm::down_at(asm::POST_B_AT[0] + half, asm::POST_B_AT[1] + half),
    );
    // Copy 1's cap faces +x, under the ground plane: picked along -x.
    let copy_one = pick_at(
        &session,
        &Ray {
            origin: Point3::new(1.0, asm::POST_B_AT[1] + half, -(asm::POST_B_AT[0] + half)),
            dir: Vec3::new(-1.0, 0.0, 0.0),
        },
    );
    assert_eq!(copy_zero.node, pattern, "the ray met the pattern's body");
    assert_eq!(copy_one.node, pattern, "the ray met the pattern's body");
    assert!(
        matches!(
            copy_one.name.path.first(),
            Some(RoleSeg::Instance { i: 1, .. })
        ),
        "the copy rides in the head: {:?}",
        copy_one.name.path.first()
    );
    // The two rays met ONE part-local face on two copies — without
    // which the frames below would be compared across faces.
    assert_eq!(
        master_face(&copy_zero),
        master_face(&copy_one),
        "one part-local face, two copies"
    );

    let shelf_bottom = pick_at(
        &session,
        &asm::up_at(
            asm::SHELF_AT[0] + asm::SHELF_LENGTH / 2.0,
            asm::SHELF_AT[1] + asm::SHELF_DEPTH / 2.0,
        ),
    );
    let (doc, eval) = session.landed_pair().expect("landed");
    let proposal_of = |copy: &FaceSelection| {
        let mut tool = MateTool::new();
        tool.pick(copy.clone());
        tool.pick(shelf_bottom.clone());
        tool.proposal(doc, eval, tol, asm::seat())
            .expect("a pattern copy is a member")
    };
    let spun = proposal_of(&copy_one);
    let unspun = proposal_of(&copy_zero);

    // The ROTATED channels FIRST — they are what a linear rule cannot
    // move and what a naive read of the copy's own pose corrupts.
    assert_eq!(
        spun.alignment.a.axis, unspun.alignment.a.axis,
        "axis: every copy of one pattern is the same part"
    );
    assert_eq!(
        spun.alignment.a.reference, unspun.alignment.a.reference,
        "reference: every copy of one pattern is the same part"
    );
    assert_eq!(
        spun.alignment.a.origin, unspun.alignment.a.origin,
        "origin: every copy of one pattern is the same part"
    );

    // And in ABSOLUTE terms, in the post's own coordinates: the cap
    // sits on the part's top plane with its normal along the part's
    // +z and its roll reference across it. The quarter turn would
    // carry that normal onto the part's x axis.
    let frame = spun.alignment.a;
    assert!(
        (frame.origin[2] - asm::POST_HEIGHT).abs() < 1e-12,
        "part coordinates: {:?}",
        frame.origin
    );
    assert!(
        (frame.axis[2].abs() - 1.0).abs() < 1e-12,
        "the cap's normal is the part's z: {:?}",
        frame.axis
    );
    assert!(
        frame.reference[2].abs() < 1e-12,
        "the roll reference is across the part's z: {:?}",
        frame.reference
    );

    // And it SOLVES: the spun copy's cap meets the shelf's underside
    // in the world the evaluation draws, which is the pattern's
    // rotation being applied exactly once.
    let outcome = session.perform(spun.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    session.pump();
    let (doc, eval) = session.landed_pair().expect("landed");
    assert!(
        solve_document(doc, tol).fault(pattern).is_none(),
        "the spun member solves"
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
}
