//! GUI-4 R2 consumer suite — an independent derivation of the PR's
//! claims, driven through the public doors only (the review charter's
//! own rows, not re-readings of the unit's).
//!
//! What these rows add over the unit's own suites:
//!
//! - **The part-coordinate pullback is checked against an independent
//!   oracle**: the standalone part document's own `face_frame`, which
//!   never saw a placement. The unit's coincidence assertions are
//!   solver-vs-authored (the solve enforces exactly what the proposal
//!   authored, so a wrong pullback would still "coincide"); these rows
//!   are authored-vs-part-truth.
//! - **The solved seat is checked geometrically**, by ray, with the
//!   expected world coordinates derived from the fixture's constants —
//!   independent of the alignment data and of `solve_document`.
//! - **A rotating probe pick**, exercising the LINEAR half of the
//!   inverse-ray transform (the unit's displaced-pick rows are
//!   translation-only, which a transposed or forward linear map would
//!   survive).
//! - Adversarial flows: a contradictory second mate on one pair; a
//!   mate landing while a free-move GESTURE is in flight; hide
//!   surviving a mate that supersedes the same instance's probe;
//!   save-as into a partless directory; the threaded seam resolving
//!   through the same `Arc<dyn PartResolver>`.
//!
//! Every row is a static fixture (no RNG anywhere — nothing here is a
//! fuzzer, per the suite-cost rules).

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use common::asm;
use pncad::document::{
    AxisSense, CancelToken, EvalOptions, Frame, MatePrimitive, PartResolver, evaluate,
};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::select::{ContactClass, Ray};
use pncad::workspace::Workspace;
use viewer::matetool::{MateChoice, MateTool, admitted_classes};
use viewer::session::{DocSession, FaceSelection, Refusal, SessionOp};
use viewer::tree::RowStatus;

/// The seat choice every committing row uses.
fn seat() -> MateChoice {
    MateChoice {
        class: ContactClass::Rest,
        primitive: MatePrimitive::FrameCoincidence,
        sense: AxisSense::Opposed,
        clocking: None,
    }
}

/// Pick through the session's real display-aware ray path.
fn pick(session: &DocSession, ray: &Ray) -> FaceSelection {
    let index = asm::index_of(session);
    let (_, eval) = session.landed_pair().expect("landed");
    index
        .face_at_for(eval, ray, &session.display_view())
        .expect("the pick answers")
        .expect("the ray hits")
}

/// The two seat picks: post_b's top cap (down), the shelf's underside
/// (up).
fn seat_picks(session: &DocSession, bench: &asm::Bench) -> (FaceSelection, FaceSelection) {
    let a = pick(
        session,
        &asm::down_at(
            asm::POST_B_AT[0] + asm::POST_SECTION / 2.0,
            asm::POST_B_AT[1] + asm::POST_SECTION / 2.0,
        ),
    );
    assert_eq!(a.node, bench.post_b, "pick a is post_b");
    let b = pick(
        session,
        &Ray {
            origin: Point3::new(
                asm::SHELF_AT[0] + asm::SHELF_LENGTH / 2.0,
                asm::SHELF_AT[1] + asm::SHELF_DEPTH / 2.0,
                -1.0,
            ),
            dir: Vec3::new(0.0, 0.0, 1.0),
        },
    );
    assert_eq!(b.node, bench.shelf_i, "pick b is the shelf");
    (a, b)
}

/// **The pullback oracle**: the proposal's part-coordinate frames must
/// equal the standalone part document's own `face_frame` answer — a
/// derivation that never involved a placement, an inverse, or the
/// assembly at all. A forward-instead-of-inverse pullback differs from
/// this oracle by twice the placement translation (0.12 m on the
/// post's x, 0.16 m on the shelf's y); an identity pullback differs by
/// the translation itself.
#[test]
fn proposal_frames_agree_with_the_standalone_part_documents() {
    let tol = Tol::witness();
    let bench = asm::bench("r2oracle", tol);
    let session = asm::open_bench(&bench, tol);
    let (a, b) = seat_picks(&session, &bench);
    let mut tool = MateTool::new();
    tool.pick(a);
    tool.pick(b);
    let (doc, eval) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, eval, tol, seat())
        .expect("the seat proposes");

    // The oracle: resolve each pinned part from the store (the same
    // door the evaluator uses), evaluate it STANDALONE, and read the
    // same cap's frame in the part's own coordinates.
    let store = Workspace::open(&bench.dir).expect("the store opens");
    let oracle = |doc_ref, local: &pncad::prelude::StableName| {
        let part = PartResolver::resolve(&store, doc_ref, tol).expect("the part resolves");
        let ev = evaluate::<f64>(
            &part,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        let tip = *part.roots().first().expect("the part has a root");
        pncad::select::face_frame(&ev, tip, local).expect("the cap has a frame")
    };
    let close = |got: [f64; 3], want: Point3<f64>, label: &str| {
        assert!(
            (got[0] - want.x).abs() < 1e-9
                && (got[1] - want.y).abs() < 1e-9
                && (got[2] - want.z).abs() < 1e-9,
            "{label}: {got:?} vs {want:?}"
        );
    };
    let close_v = |got: [f64; 3], want: Vec3<f64>, label: &str| {
        assert!(
            (got[0] - want.x).abs() < 1e-9
                && (got[1] - want.y).abs() < 1e-9
                && (got[2] - want.z).abs() < 1e-9,
            "{label}: {got:?} vs {want:?}"
        );
    };
    let post = oracle(&bench.post, &bench.post_top);
    close(
        proposal.alignment.a.origin,
        post.origin,
        "a.origin is the part's own cap origin",
    );
    close_v(proposal.alignment.a.axis, post.axis, "a.axis");
    close_v(
        proposal.alignment.a.reference,
        post.u_ref.expect("the cap fixes a reference"),
        "a.reference",
    );
    let shelf = oracle(&bench.shelf, &bench.shelf_bottom);
    close(
        proposal.alignment.b.origin,
        shelf.origin,
        "b.origin is the part's own cap origin (the shelf placement's \
         0.08 m y-translation was divided out, not applied)",
    );
    close_v(proposal.alignment.b.axis, shelf.axis, "b.axis");
    close_v(
        proposal.alignment.b.reference,
        shelf.u_ref.expect("the cap fixes a reference"),
        "b.reference",
    );
}

/// **The solved seat, checked by ray with hand-derived coordinates.**
/// FrameCoincidence at Opposed sense seats the post's top cap onto the
/// shelf's underside, so after the commit the post hangs below the
/// shelf: an upward ray at the shelf's underside centre must now meet
/// post_b first, 0.05 m further down. Nothing here reads the
/// alignment, the proposal, or `solve_document` — the expectation is
/// the fixture's own constants.
#[test]
fn the_solved_seat_hangs_the_post_under_the_shelf() {
    let tol = Tol::witness();
    let bench = asm::bench("r2seatgeom", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (a, b) = seat_picks(&session, &bench);
    let mut tool = MateTool::new();
    tool.pick(a);
    tool.pick(b);
    let (doc, eval) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, eval, tol, seat())
        .expect("the seat proposes");
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();

    let centre = [
        asm::SHELF_AT[0] + asm::SHELF_LENGTH / 2.0,
        asm::SHELF_AT[1] + asm::SHELF_DEPTH / 2.0,
    ];
    let index = asm::index_of(&session);
    let (_, eval) = session.landed_pair().expect("landed");
    let view = session.display_view();
    // Upward ray from below: post_b's bottom cap, now at z = -height.
    let up = Ray {
        origin: Point3::new(centre[0], centre[1], -1.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    };
    let under = index
        .pick_for(eval, &up, &view)
        .expect("the pick answers")
        .expect("something hangs under the shelf");
    assert_eq!(under.node, bench.post_b, "the post seated under the shelf");
    assert!(
        (under.point.z - (-asm::POST_HEIGHT)).abs() < 1e-9,
        "its bottom cap sits one post-height below the shelf's \
         underside: z = {}",
        under.point.z
    );
    // Downward ray from above: the shelf's top face, where it always
    // was — the seat moved the post, not the shelf.
    let over = index
        .pick_for(eval, &asm::down_at(centre[0], centre[1]), &view)
        .expect("the pick answers")
        .expect("the shelf is still there");
    assert_eq!(over.node, bench.shelf_i);
    assert!(
        (over.point.z - asm::SHELF_THICKNESS).abs() < 1e-9,
        "the shelf's top face is untouched: z = {}",
        over.point.z
    );
    // And the post vacated its authored spot.
    assert!(
        index
            .pick_for(
                eval,
                &asm::down_at(
                    asm::POST_B_AT[0] + asm::POST_SECTION / 2.0,
                    asm::POST_B_AT[1] + asm::POST_SECTION / 2.0,
                ),
                &view,
            )
            .expect("the pick answers")
            .is_none(),
        "nothing remains at post_b's authored position"
    );
}

/// **A rotating probe is picked where it is drawn** — the linear half
/// of the inverse-ray transform. The probe rotates post_b a quarter
/// turn about z and translates it; the row asserts a hit at the
/// hand-computed drawn position (with the hit point mapped back to
/// world), and a miss at the authored one. A forward-instead-of-
/// inverse map, or a transposed linear inverse, fails the hit.
#[test]
fn a_rotating_probe_is_picked_at_its_drawn_position() {
    let tol = Tol::witness();
    let bench = asm::bench("r2rotpick", tol);
    let mut session = asm::open_bench(&bench, tol);
    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    // p' = R(π/2 about z)·p + (0.12, 0, 0): the authored box
    // x ∈ [0.06, 0.08], y ∈ [0, 0.02] maps to x' ∈ [0.10, 0.12],
    // y' ∈ [0.06, 0.08]; z is unchanged.
    let outcome = session.perform(SessionOp::PreviewFreeMove {
        frame: Frame::rotate_then_translate(
            [0.0, 0.0, 1.0],
            std::f64::consts::FRAC_PI_2,
            [0.12, 0.0, 0.0],
        ),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.perform(SessionOp::CommitFreeMove);

    let index = asm::index_of(&session);
    let (_, eval) = session.landed_pair().expect("landed");
    let view = session.display_view();
    let hit = index
        .pick_for(eval, &asm::down_at(0.11, 0.07), &view)
        .expect("the pick answers")
        .expect("the rotated probe is under the drawn-position ray");
    assert_eq!(hit.node, bench.post_b);
    assert!(
        (hit.point.x - 0.11).abs() < 1e-9
            && (hit.point.y - 0.07).abs() < 1e-9
            && (hit.point.z - asm::POST_HEIGHT).abs() < 1e-9,
        "the hit point is reported in WORLD (drawn) coordinates: {:?}",
        hit.point
    );
    assert!(
        index
            .pick_for(
                eval,
                &asm::down_at(
                    asm::POST_B_AT[0] + asm::POST_SECTION / 2.0,
                    asm::POST_B_AT[1] + asm::POST_SECTION / 2.0,
                ),
                &view,
            )
            .expect("the pick answers")
            .is_none(),
        "the authored position is vacated while the probe stands"
    );
}

/// **Two picks on one instance — two DIFFERENT faces — still refuse
/// `SamePick`** (the unit's row uses the same face twice; a mate
/// between two faces of one body is just as much a self-mate).
#[test]
fn two_different_faces_of_one_instance_refuse_same_pick() {
    let tol = Tol::witness();
    let bench = asm::bench("r2sameinst", tol);
    let session = asm::open_bench(&bench, tol);
    let top = pick(
        &session,
        &asm::down_at(
            asm::POST_B_AT[0] + asm::POST_SECTION / 2.0,
            asm::POST_B_AT[1] + asm::POST_SECTION / 2.0,
        ),
    );
    // A side face, picked with a horizontal ray at half height.
    let side = pick(
        &session,
        &Ray {
            origin: Point3::new(1.0, asm::POST_B_AT[1] + asm::POST_SECTION / 2.0, 0.025),
            dir: Vec3::new(-1.0, 0.0, 0.0),
        },
    );
    assert_eq!(side.node, bench.post_b, "the side ray hit post_b");
    assert_ne!(side.name, top.name, "two different faces");
    let mut tool = MateTool::new();
    tool.pick(top);
    tool.pick(side);
    let (doc, eval) = session.landed_pair().expect("landed");
    assert!(
        matches!(
            tool.proposal(doc, eval, tol, seat()),
            Err(viewer::matetool::MateToolError::SamePick { instance })
                if instance == bench.post_b
        ),
        "a mate needs a PAIR of instances"
    );
}

/// **A second, contradictory mate on the same pair fails typed and is
/// recoverable.** The document door accepts the edit (it is a legal
/// recipe); evaluation refuses the pair's empty coset intersection
/// with a typed fault on a mate row; undo restores the working
/// document. No crash, no silent success.
#[test]
fn a_contradictory_second_mate_fails_typed_and_undo_recovers() {
    let tol = Tol::witness();
    let bench = asm::bench("r2contra", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (a, b) = seat_picks(&session, &bench);
    let mut tool = MateTool::new();
    tool.pick(a.clone());
    tool.pick(b.clone());
    let (doc, eval) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, eval, tol, seat())
        .expect("the seat proposes");
    session.perform(proposal.op());
    session.pump();
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }

    // The same pair again, at a DIFFERENT alignment: shift the b-side
    // origin so the two frame coincidences cannot both hold.
    let mut shifted = proposal.alignment;
    shifted.b.origin[0] += 0.01;
    let outcome = session.perform(SessionOp::AddMate {
        a: proposal.a.clone(),
        b: proposal.b.clone(),
        class: ContactClass::Rest,
        alignment: shifted,
    });
    assert!(
        outcome.refusal.is_none(),
        "the recipe accepts the edit; evaluation is where it fails: {:?}",
        outcome.refusal
    );
    session.pump();
    let rows = session.tree_rows();
    let failed: Vec<_> = rows
        .iter()
        .filter(|row| matches!(row.status, RowStatus::Failed { .. }))
        .collect();
    assert!(
        !failed.is_empty(),
        "a contradictory pair must fail SOME row typed; all rows: {rows:?}"
    );
    for row in &failed {
        if let RowStatus::Failed { message } = &row.status {
            assert!(!message.is_empty(), "the fault carries a message");
        }
    }

    // Undo removes the contradiction; the assembly works again.
    let outcome = session.perform(SessionOp::Undo);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "after undo: {row:?}");
    }
}

/// **A mate landing while a free-move GESTURE is in flight kills the
/// gesture** (the unit's rows supersede a COMMITTED probe; the
/// in-flight arm is `prune`'s third clause and deserves its own
/// exercise). Afterwards the preview op refuses `NoFreeMove` — the
/// gesture died with its instance's eligibility.
#[test]
fn a_landing_mate_kills_an_in_flight_gesture() {
    let tol = Tol::witness();
    let bench = asm::bench("r2gesture", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (a, b) = seat_picks(&session, &bench);
    let mut tool = MateTool::new();
    tool.pick(a);
    tool.pick(b);
    let (doc, eval) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, eval, tol, seat())
        .expect("the seat proposes");

    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    session.perform(SessionOp::PreviewFreeMove {
        frame: Frame::translation([0.02, 0.0, 0.0]),
    });
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    // The in-flight gesture is dead: a further preview has nothing to
    // preview into.
    let outcome = session.perform(SessionOp::PreviewFreeMove {
        frame: Frame::translation([0.03, 0.0, 0.0]),
    });
    assert!(
        matches!(
            outcome.refusal,
            Some(Refusal::Display(viewer::display::DisplayFault::NoFreeMove))
        ),
        "{:?}",
        outcome.refusal
    );
    // And nothing of the gesture leaked into a committed value.
    assert!(session.display().free_move_of(bench.post_b).is_none());
}

/// **Hide survives the mate that supersedes the probe.** Probe post_b,
/// hide it, then land the mate: the probe is discarded (reported), the
/// hidden state stands (hide is about visibility, not placement), and
/// the drawn scene still omits the instance at its new solved
/// placement.
#[test]
fn hide_survives_the_mate_that_discards_the_probe() {
    let tol = Tol::witness();
    let bench = asm::bench("r2hidemate", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (a, b) = seat_picks(&session, &bench);
    let mut tool = MateTool::new();
    tool.pick(a);
    tool.pick(b);
    let (doc, eval) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, eval, tol, seat())
        .expect("the seat proposes");

    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    session.perform(SessionOp::PreviewFreeMove {
        frame: Frame::translation([0.02, 0.0, 0.0]),
    });
    session.perform(SessionOp::CommitFreeMove);
    session.perform(SessionOp::SetInstanceHidden {
        instance: bench.post_b,
        hidden: true,
    });
    // Hidden AND probed: the pick index offers it nowhere.
    let index = asm::index_of(&session);
    let (_, eval) = session.landed_pair().expect("landed");
    let view = session.display_view();
    for x in [
        asm::POST_B_AT[0] + asm::POST_SECTION / 2.0,
        asm::POST_B_AT[0] + 0.02 + asm::POST_SECTION / 2.0,
    ] {
        assert!(
            index
                .pick_for(
                    eval,
                    &asm::down_at(x, asm::POST_B_AT[1] + asm::POST_SECTION / 2.0),
                    &view
                )
                .expect("the pick answers")
                .is_none(),
            "a hidden instance is offered neither moved nor unmoved"
        );
    }

    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.superseded, vec![bench.post_b]);
    assert!(session.display().is_hidden(bench.post_b), "hide stands");
    session.pump();
    let index = asm::index_of(&session);
    let scene = index
        .scene_for(&session.display_view())
        .expect("a scene survives");
    assert_eq!(scene.stats().probe_parts, 0, "no probe survives the mate");
    // The solved placement is under the shelf; hidden means the
    // upward ray finds the SHELF's underside, not the seated post.
    let (_, eval) = session.landed_pair().expect("landed");
    let under = index
        .pick_for(
            eval,
            &Ray {
                origin: Point3::new(
                    asm::SHELF_AT[0] + asm::SHELF_LENGTH / 2.0,
                    asm::SHELF_AT[1] + asm::SHELF_DEPTH / 2.0,
                    -1.0,
                ),
                dir: Vec3::new(0.0, 0.0, 1.0),
            },
            &session.display_view(),
        )
        .expect("the pick answers")
        .expect("the shelf's underside answers");
    assert_eq!(
        under.node, bench.shelf_i,
        "the seated post stays hidden at its new placement"
    );
}

/// **Save-as into a directory without the parts.** The resolver
/// follows the file (the disclosed judgment call); what the session
/// then SHOWS is pinned here: the landed picture answers the landed
/// run, and the memo's pin-verified reuse means an already-resolved
/// part does not go red merely because the new directory lacks it —
/// but a FRESH session over the moved file refuses every resolution
/// typed (the directory rule, enforced where resolution actually
/// happens).
#[test]
fn save_as_rebinds_the_resolver_and_a_fresh_open_enforces_the_rule() {
    let tol = Tol::witness();
    let bench = asm::bench("r2saveas", tol);
    let mut session = asm::open_bench(&bench, tol);
    let elsewhere = bench.dir.join("elsewhere");
    std::fs::create_dir_all(&elsewhere).expect("the sibling directory creates");
    let moved = elsewhere.join("moved.pncad");
    let outcome = session.perform(SessionOp::Save(moved.clone()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        session.resolve_dir().expect("still wired"),
        elsewhere,
        "the resolver followed the file"
    );
    session.pump();
    // A fresh session over the moved file: every resolution refuses
    // typed — the parts are not beside THIS file.
    let mut fresh = DocSession::inline(pncad::document::Doc::empty_derived("r2-boot", tol), tol);
    let outcome = fresh.perform(SessionOp::Open(moved));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    fresh.pump();
    for row in fresh.tree_rows() {
        assert!(
            matches!(row.status, RowStatus::Failed { .. }),
            "the moved document must not resolve against the old \
             directory: {row:?}"
        );
    }
}

/// **The threaded seam resolves through the same resolver handle** —
/// the `Arc<dyn PartResolver>` crossing the worker-thread boundary
/// (compile-time `Send + Sync`, exercised at runtime here: same
/// assembly, same directory rule, through `ThreadEvaluator`).
#[test]
fn the_threaded_seam_resolves_the_assembly_too() {
    let tol = Tol::witness();
    let bench = asm::bench("r2threaded", tol);
    let mut session = DocSession::new(
        pncad::document::Doc::empty_derived("r2-threaded-boot", tol),
        tol,
        Box::new(viewer::ThreadEvaluator::spawn().expect("the worker spawns")),
    );
    let outcome = session.perform(SessionOp::Open(bench.asm_path.clone()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    for _ in 0..30_000 {
        session.pump();
        if !session.busy() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    assert!(!session.busy(), "the threaded run lands");
    let rows = session.tree_rows();
    assert_eq!(rows.len(), 3);
    for row in &rows {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
}

/// The admission exposure cannot drift from the kernel table for any
/// class it offers (a divergence probe: the verdicts are READ, and
/// this row fails if a restatement ever creeps in), and it offers
/// every class the kernel enum can currently name.
#[test]
fn admitted_classes_match_the_kernel_table_exactly() {
    let classes = admitted_classes();
    for entry in &classes {
        assert_eq!(
            entry.admission,
            pncad::document::class_admission(entry.class),
            "{:?}",
            entry.class
        );
    }
    let named: Vec<&str> = classes.iter().map(|e| e.class.name()).collect();
    assert_eq!(
        named,
        vec!["Rest", "Tangent"],
        "every nameable class is offered (this row is the tripwire \
         that fires when the kernel enum grows one)"
    );
}

/// PROBE (evidence row, kept because its outcome is a pinned fact a
/// future change should trip over): deleting a part FILE mid-session
/// and re-evaluating does NOT go red — the memo's pin-verified reuse
/// answers the instantiate nodes without consulting the store again
/// (same pin, same bytes; the PR's memo-safety argument, running in
/// the other direction). The rule is enforced where resolution
/// happens: a FRESH session over the same file refuses typed (the
/// unit's own missing-part row). If this row starts failing, the memo
/// key grew a resolver component — a semantic change worth noticing.
#[test]
fn a_mid_session_file_deletion_is_absorbed_by_the_memo() {
    let tol = Tol::witness();
    let bench = asm::bench("r2memodel", tol);
    let mut session = asm::open_bench(&bench, tol);
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    std::fs::remove_file(bench.dir.join(format!("{}.pncad", bench.post.id)))
        .expect("the post file removes");
    let outcome = session.perform(SessionOp::Reevaluate);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    for row in session.tree_rows() {
        assert_eq!(
            row.status,
            RowStatus::Ok,
            "the landed memo answers; deletion surfaces at the next \
             fresh resolution: {row:?}"
        );
    }
}

/// The save-as re-evaluation's OTHER direction, which is the one that
/// makes the rebind-and-re-evaluate rationale real: a document whose
/// resolutions FAILED (opened beside no parts) recovers to green when
/// saved into the directory that has them — failed resolutions are
/// not memoized as if they were answers.
#[test]
fn save_as_into_the_store_recovers_a_failed_resolution() {
    let tol = Tol::witness();
    let bench = asm::bench("r2saverec", tol);
    let elsewhere = bench.dir.join("elsewhere");
    std::fs::create_dir_all(&elsewhere).expect("the sibling directory creates");
    let moved = elsewhere.join("moved.pncad");
    std::fs::copy(&bench.asm_path, &moved).expect("the assembly copies");
    let mut session = DocSession::inline(pncad::document::Doc::empty_derived("r2-boot", tol), tol);
    session.perform(SessionOp::Open(moved));
    session.pump();
    assert!(
        session
            .tree_rows()
            .iter()
            .all(|row| matches!(row.status, RowStatus::Failed { .. })),
        "opened beside no parts: every instance refuses"
    );
    // A second store holding the PARTS (and not the assembly — saving
    // it back beside the original would collide on the assembly's own
    // id; see the report's note on save-a-copy).
    let store2 = bench.dir.join("store2");
    std::fs::create_dir_all(&store2).expect("the second store creates");
    for id in [&bench.post.id, &bench.shelf.id] {
        let name = format!("{id}.pncad");
        std::fs::copy(bench.dir.join(&name), store2.join(&name)).expect("the part copies");
    }
    let outcome = session.perform(SessionOp::Save(store2.join("recovered.pncad")));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    for row in session.tree_rows() {
        assert_eq!(
            row.status,
            RowStatus::Ok,
            "the rebind re-resolves what had failed: {row:?}"
        );
    }
}
