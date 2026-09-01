//! **GUI-4 reviewer suite (R1)** — an independent derivation of the
//! unit's claims, written against the public doors and NOT read off the
//! shipped rows.
//!
//! Two things this file is here for that the shipped suites do not do:
//!
//! 1. **The mate tool's world→part pull-back is re-derived here.** The
//!    shipped rows assert the mated frames COINCIDE in world after the
//!    solve — but the solve derives the placement FROM the authored
//!    alignment, so that assertion holds for any alignment the tool
//!    mints, right or wrong. It is self-consistent, not a check. This
//!    file recomputes the expected part-coordinate frame from the
//!    picked face's world pose and the instance's placement with its
//!    own arithmetic, and compares component by component — including
//!    the components the placement actually moves, and with a ROTATED
//!    placement, where the inverse's linear part is exercised at all.
//!    (Measured at review time: replacing `placement.affine().inverse()`
//!    with the identity left all 17 shipped assembly rows green.)
//! 2. **The probe's pick path under a ROTATION.** The shipped rows move
//!    the probe by a translation only, and never read the hit POINT, so
//!    the inverse-ray transform's linear part and the hit's map back to
//!    world are both unexercised. Both are asserted here.
//!
//! Everything else is the adversarial residue: hide × probe × mate in
//! combination, a store that changes UNDER an open session, save-as into
//! a directory with no parts, the two gesture-order refusals, and the
//! identity-commit edge.
//!
//! No fuzzing, no seeds: every witness here is one that can be written
//! down ([[test-suite-cost]]'s second shape), and the rows are merged so
//! each expensive fixture is built once. Assertions are labelled so a
//! red row names its own property.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use std::f64::consts::FRAC_PI_2;

use common::{asm};
use pncad::document::{
    AxisSense, ClassAdmission, DocEdit, DocumentId, Frame, MatePrimitive, Node, PatternKind,
    ProfileDoc, ProfileProgram, RecipeNodeId, apply, assemble, class_admission, parse_expr,
    solve_document,
};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::select::{ContactClass, Ray, face_frame};
use pncad::workspace::Workspace;
use viewer::display::DisplayFault;
use viewer::matetool::{MateChoice, MateTool, admitted_classes};
use viewer::scene::SceneMesh;
use viewer::session::{DocSession, FaceSelection, Refusal, SessionOp};
use viewer::tree::RowStatus;

/// The choice every committing row here uses.
fn rest_choice() -> MateChoice {
    MateChoice {
        class: ContactClass::Rest,
        primitive: MatePrimitive::FrameCoincidence,
        sense: AxisSense::Opposed,
        clocking: None,
    }
}

/// A ray straight up from below at `(x, y)`.
fn up_at(x: f64, y: f64) -> Ray {
    Ray {
        origin: Point3::new(x, y, -1.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    }
}

/// **This file's own inverse.** A rigid frame's world→part map, written
/// out longhand rather than called through `Affine3::inverse`, so the
/// comparison below is against arithmetic the code under review does
/// not share: for `p ↦ R·p + t` with `R` orthonormal, the inverse is
/// `q ↦ Rᵀ·(q − t)`.
fn world_to_part(frame: &Frame, world: [f64; 3]) -> [f64; 3] {
    let [c0, c1, c2] = frame.columns;
    let d = [
        world[0] - frame.translation[0],
        world[1] - frame.translation[1],
        world[2] - frame.translation[2],
    ];
    // Rᵀ·d — the rows of R are the columns' components.
    [
        c0[0] * d[0] + c0[1] * d[1] + c0[2] * d[2],
        c1[0] * d[0] + c1[1] * d[1] + c1[2] * d[2],
        c2[0] * d[0] + c2[1] * d[1] + c2[2] * d[2],
    ]
}

/// The same for a direction: no translation.
fn world_to_part_vec(frame: &Frame, world: [f64; 3]) -> [f64; 3] {
    let [c0, c1, c2] = frame.columns;
    [
        c0[0] * world[0] + c0[1] * world[1] + c0[2] * world[2],
        c1[0] * world[0] + c1[1] * world[1] + c1[2] * world[2],
        c2[0] * world[0] + c2[1] * world[1] + c2[2] * world[2],
    ]
}

fn close(got: [f64; 3], want: [f64; 3], eps: f64, what: &str) {
    for i in 0..3 {
        assert!(
            (got[i] - want[i]).abs() < eps,
            "{what}: component {i} — got {got:?}, want {want:?}"
        );
    }
}

/// Pick one face through the real cursor path under the session's
/// display view.
fn pick(session: &DocSession, index: &viewer::pick::PickIndex, ray: &Ray) -> FaceSelection {
    let (_, eval) = session.landed_pair().expect("a landed evaluation");
    index
        .face_at_for(eval, ray, &session.display_view())
        .expect("the pick answers")
        .expect("the ray hits something")
}

// ── 1. The mate tool's pull-back, re-derived ──────────────────────

/// The alignment the tool mints must be the picked face's world pose
/// carried into the instance's OWN coordinates — every component, for a
/// translated instance and for a rotated one.
///
/// The rotated half is the load-bearing half: with a translation-only
/// placement the inverse's linear part is the identity, so a missing or
/// mis-directed inverse only moves the origin, and only in the axes the
/// translation touches.
#[test]
fn r1_the_minted_alignment_is_the_placement_inverse_of_the_picked_world_pose() {
    let tol = Tol::witness();
    let bench = asm::bench("r1pullback", tol);
    // A second assembly document in the SAME store, whose post carries
    // a genuinely rotated placement (a quarter turn about z, then a
    // shift), so the pull-back exercises Rᵀ and not just −t.
    //
    // It is authored here rather than driven through the session
    // because **the viewer has no `SessionOp` that sets a placement**:
    // an instance's pose is either what the file already said or what
    // a mate solves, so a rotated instance can only be arranged from
    // outside. (A finding in its own right; here it is just why this
    // fixture exists.)
    let rotated = Frame::rotate_then_translate([0.0, 0.0, 1.0], FRAC_PI_2, [-0.05, 0.04, 0.0]);
    let mut ws = Workspace::open(&bench.dir).expect("the store opens");
    let mut doc = ProfileDoc::empty(DocumentId::derive("r1-rotated-bench"), tol);
    let insert = |doc: &mut ProfileDoc, node: Node<ProfileProgram>| {
        let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the insert applies");
        *doc = applied.doc;
        applied.record.minted.expect("an insert mints an id")
    };
    let rot_post = insert(&mut doc, Node::instantiate_part(bench.post));
    let applied = apply(
        &doc,
        &DocEdit::SetPlacement {
            node: rot_post,
            frame: rotated,
        },
        tol,
    )
    .expect("the placement applies");
    doc = applied.doc;
    let rot_shelf = insert(&mut doc, Node::instantiate_part(bench.shelf));
    let applied = apply(
        &doc,
        &DocEdit::SetPlacement {
            node: rot_shelf,
            frame: Frame::translation(asm::SHELF_AT),
        },
        tol,
    )
    .expect("the placement applies");
    doc = applied.doc;
    let path = ws.create(&doc, tol).expect("the rotated assembly stores");

    let mut session = DocSession::inline(pncad::document::Doc::empty_derived("r1-boot", tol), tol);
    let outcome = session.perform(SessionOp::Open(path));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    for row in session.tree_rows() {
        assert_eq!(
            row.status,
            RowStatus::Ok,
            "the rotated bench resolves: {row:?}"
        );
    }

    let index = asm::index_of(&session);
    // The rotated post's top cap, picked from above at its DRAWN spot.
    // The quarter turn about z maps the part's (x, y) footprint
    // [0,s]×[0,s] to [−s,0]×[0,s]; the shift puts its centre here.
    let s = asm::POST_SECTION;
    let post_a_top = pick(
        &session,
        &index,
        &asm::down_at(-0.05 - s / 2.0, 0.04 + s / 2.0),
    );
    assert_eq!(
        post_a_top.node, rot_post,
        "the rotated instance is where the placement puts it"
    );
    let shelf_bottom = pick(
        &session,
        &index,
        &up_at(
            asm::SHELF_AT[0] + asm::SHELF_LENGTH / 2.0,
            asm::SHELF_AT[1] + asm::SHELF_DEPTH / 2.0,
        ),
    );
    assert_eq!(shelf_bottom.node, rot_shelf);

    let mut tool = MateTool::new();
    tool.pick(post_a_top.clone());
    tool.pick(shelf_bottom.clone());
    let (doc, eval) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, eval, tol, rest_choice())
        .expect("the tool proposes");

    // The independent derivation: the picked face's WORLD pose, read
    // through the same shipped door, pulled back with this file's own
    // arithmetic against the placement the solve reports.
    let poses = solve_document(doc, tol);
    for (side_name, pick_ref, minted) in [
        ("a", &post_a_top, proposal.alignment.a),
        ("b", &shelf_bottom, proposal.alignment.b),
    ] {
        let pose = face_frame(eval, pick_ref.node, &pick_ref.name).expect("the face has a pose");
        let placement = poses
            .placement(doc, pick_ref.node)
            .expect("the instance is placed");
        let u_ref = pose.u_ref.expect("a cap fixes a roll reference");
        let want_origin = world_to_part(&placement, [pose.origin.x, pose.origin.y, pose.origin.z]);
        let want_axis = world_to_part_vec(&placement, [pose.axis.x, pose.axis.y, pose.axis.z]);
        let want_ref = world_to_part_vec(&placement, [u_ref.x, u_ref.y, u_ref.z]);
        close(
            minted.origin,
            want_origin,
            1e-12,
            &format!("side {side_name}: minted origin is the pulled-back world origin"),
        );
        close(
            minted.axis,
            want_axis,
            1e-12,
            &format!("side {side_name}: minted axis is the pulled-back world axis"),
        );
        close(
            minted.reference,
            want_ref,
            1e-12,
            &format!("side {side_name}: minted reference is the pulled-back world reference"),
        );
    }

    // And an absolute pin that does not depend on the tool at all: the
    // post's top cap sits at the CENTRE of the part's top face in the
    // part's own coordinates, whatever the instance's placement is.
    close(
        proposal.alignment.a.origin,
        [s / 2.0, s / 2.0, asm::POST_HEIGHT],
        1e-9,
        "the post's top-cap frame in PART coordinates",
    );

    // The edit lands once and the document stays green.
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1, "exactly one committed edit");
    session.pump();
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "after the mate: {row:?}");
    }
}

// ── 2. The probe's pick path under a rotation ─────────────────────

/// A rotated free-move probe: the drawn picture, the pick, the hit
/// POINT in world, and the ray parameter must all agree with the
/// displaced position — and the authored position must go quiet.
#[test]
fn r1_a_rotated_probe_is_drawn_picked_and_reported_in_world() {
    let tol = Tol::witness();
    let bench = asm::bench("r1rotprobe", tol);
    let mut session = asm::open_bench(&bench, tol);
    let index = asm::index_of(&session);
    let s = asm::POST_SECTION;

    // A quarter turn about z about the coordinate origin, then a shift
    // that lands the post well clear of everything: the part occupies
    // world x ∈ [0.06, 0.08], y ∈ [0, 0.02] before the probe; the turn
    // sends (x, y) ↦ (−y, x), and the shift moves it to a fresh spot.
    let shift = [0.20, -0.10, 0.0];
    let probe = Frame::rotate_then_translate([0.0, 0.0, 1.0], FRAC_PI_2, shift);
    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    let outcome = session.perform(SessionOp::PreviewFreeMove { frame: probe });
    assert!(
        outcome.refusal.is_none(),
        "a rotation is a rigid motion: {:?}",
        outcome.refusal
    );
    session.perform(SessionOp::CommitFreeMove);
    let view = session.display_view();

    // Where the probed post's top-cap centre is DRAWN: take the
    // un-probed world centre and push it through the probe frame.
    let unmoved_centre = Point3::new(
        asm::POST_B_AT[0] + s / 2.0,
        asm::POST_B_AT[1] + s / 2.0,
        asm::POST_HEIGHT,
    );
    let drawn = probe.affine::<f64>().transform_point(unmoved_centre);

    // The scene draws it there.
    let scene = index.scene_for(&view).expect("a scene builds");
    assert_eq!(scene.stats().probe_parts, 1, "one probed part");
    assert!(
        scene.flags().contains(&SceneMesh::FLAG_PROBE),
        "the probe's corners carry the distinctness flag"
    );
    assert!(
        scene.bounds().max_x >= drawn.x - 1e-9,
        "the drawn bounds reach the rotated probe: {:?}",
        scene.bounds()
    );

    // The pick answers there, and the HIT POINT comes back in world.
    let (_, eval) = session.landed_pair().expect("landed");
    let ray = asm::down_at(drawn.x, drawn.y);
    let hit = index
        .pick_for(eval, &ray, &view)
        .expect("the pick answers")
        .expect("the rotated probe is under the ray");
    assert_eq!(hit.node, bench.post_b, "the probed instance answers");
    assert!(
        (hit.point.z - drawn.z).abs() < 1e-9
            && (hit.point.x - drawn.x).abs() < 1e-9
            && (hit.point.y - drawn.y).abs() < 1e-9,
        "the hit point is reported in WORLD (the drawn position), not in the \
         instance's display-local frame: {:?} vs {drawn:?}",
        hit.point
    );
    // …and `t` is the true world distance, which is what makes it
    // comparable against the unmoved batch's hits.
    assert!(
        (hit.t - (ray.origin.z - drawn.z)).abs() < 1e-9,
        "t is the world ray parameter: {} vs {}",
        hit.t,
        ray.origin.z - drawn.z
    );

    // The authored position no longer answers for this instance.
    let quiet = index
        .pick_for(
            eval,
            &asm::down_at(asm::POST_B_AT[0] + s / 2.0, asm::POST_B_AT[1] + s / 2.0),
            &view,
        )
        .expect("the pick answers");
    assert!(
        quiet.is_none_or(|h| h.node != bench.post_b),
        "the probe moved away from the authored spot"
    );
}

// ── 3. Hide × probe × mate, in combination ───────────────────────

/// The three display facts interact: a hidden probe leaves the picture
/// and the pick index without losing its value; a mate on a probed
/// instance discards the value; a second mate on the same instance is
/// not silently accepted as a better answer.
#[test]
fn r1_hide_probe_and_mate_compose_without_a_silent_state() {
    let tol = Tol::witness();
    let bench = asm::bench("r1compose", tol);
    let mut session = asm::open_bench(&bench, tol);
    let index = asm::index_of(&session);
    let s = asm::POST_SECTION;

    // Probe, then hide the probed instance.
    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    session.perform(SessionOp::PreviewFreeMove {
        frame: Frame::translation([0.05, 0.0, 0.0]),
    });
    session.perform(SessionOp::CommitFreeMove);
    session.perform(SessionOp::SetInstanceHidden {
        instance: bench.post_b,
        hidden: true,
    });
    let view = session.display_view();
    let scene = index.scene_for(&view).expect("a scene");
    assert_eq!(
        scene.stats().probe_parts,
        0,
        "a hidden probe contributes no drawn probe part"
    );
    assert!(
        scene.flags().iter().all(|&f| f == 0),
        "…and no corner is marked"
    );
    let (_, eval) = session.landed_pair().expect("landed");
    assert!(
        index
            .pick_for(eval, &asm::down_at(0.06 + 0.05 + s / 2.0, s / 2.0), &view)
            .expect("the pick answers")
            .is_none(),
        "hidden beats probed in the pick index too"
    );
    assert!(
        session.display().free_move_of(bench.post_b).is_some(),
        "hiding does not destroy the probe VALUE — only the picture"
    );

    // Unhide: both facts come back together.
    session.perform(SessionOp::SetInstanceHidden {
        instance: bench.post_b,
        hidden: false,
    });
    assert_eq!(
        index
            .scene_for(&session.display_view())
            .expect("a scene")
            .stats()
            .probe_parts,
        1,
        "unhiding restores the probe's drawn marking"
    );

    // Mate it: the probe is discarded and reported in the same outcome.
    let first = session.perform(SessionOp::AddMate {
        a: asm::in_part(bench.post_b, &bench.post_top),
        b: asm::in_part(bench.shelf_i, &bench.shelf_bottom),
        class: ContactClass::Rest,
        alignment: seat(),
    });
    assert!(first.refusal.is_none(), "{:?}", first.refusal);
    assert_eq!(first.committed.len(), 1);
    assert_eq!(first.superseded, vec![bench.post_b]);
    assert!(session.display().free_move_of(bench.post_b).is_none());
    session.pump();

    // A SECOND mate naming the same pair: whatever the document layer
    // decides, the session must not report success while the tree hides
    // a refusal. Both outcomes are acceptable; a green tree over a
    // second unresolved constraint is not.
    let second = session.perform(SessionOp::AddMate {
        a: asm::in_part(bench.post_b, &bench.post_top),
        b: asm::in_part(bench.shelf_i, &bench.shelf_bottom),
        class: ContactClass::Rest,
        alignment: seat(),
    });
    session.pump();
    let rows = session.tree_rows();
    let all_ok = rows.iter().all(|r| r.status == RowStatus::Ok);
    assert!(
        second.refusal.is_some()
            || !all_ok
            || rows.iter().filter(|r| r.kind == "Mate").count() == 2,
        "a second mate on the same pair is either refused at the door or visible in \
         the tree; it is never invisible: refusal={:?} rows={rows:?}",
        second.refusal
    );

    // Free-move now refuses on BOTH participants, listing the mates.
    for constrained in [bench.post_b, bench.shelf_i] {
        match session
            .perform(SessionOp::BeginFreeMove {
                instance: constrained,
            })
            .refusal
        {
            Some(Refusal::Display(DisplayFault::MateConstrained { instance, mates })) => {
                assert_eq!(instance, constrained);
                assert!(!mates.is_empty(), "the refusal names its mates");
            }
            other => panic!("a mated instance must refuse the probe, got {other:?}"),
        }
    }
}

/// The seat alignment the composition row authors directly.
fn seat() -> pncad::document::Alignment {
    pncad::document::Alignment {
        a: pncad::document::MateFrame {
            origin: [
                asm::POST_SECTION / 2.0,
                asm::POST_SECTION / 2.0,
                asm::POST_HEIGHT,
            ],
            axis: [0.0, 0.0, 1.0],
            reference: [1.0, 0.0, 0.0],
        },
        b: pncad::document::MateFrame {
            origin: [asm::SHELF_LENGTH / 2.0, asm::SHELF_DEPTH / 2.0, 0.0],
            axis: [0.0, 0.0, -1.0],
            reference: [1.0, 0.0, 0.0],
        },
        primitive: MatePrimitive::FrameCoincidence,
        sense: AxisSense::Opposed,
        clocking: None,
    }
}

// ── 4. The store under an open session ───────────────────────────

/// **The scan-at-resolution posture is bounded by the evaluation
/// memo, and this row is where that shows.**
///
/// `DirResolver` really does consult the directory when an
/// `InstantiatePart` resolves — but the seam primes every run from the
/// previous completed evaluation, so an unchanged document's
/// instantiate nodes hit the memo and never reach the resolver again.
/// The consequence, pinned below: a store that changes UNDER an open
/// session is not seen, however many times the session re-evaluates.
/// The session then shows a picture the file no longer means — which a
/// fresh open, asserted beside it, does report.
///
/// This asserts the behaviour that ships, not the behaviour the
/// posture reads as promising. If the memo is ever keyed against the
/// resolver, this row goes red and is the place to update.
#[test]
fn r1_the_memo_bounds_scan_at_resolution_a_changed_store_is_not_re_read() {
    let tol = Tol::witness();
    let bench = asm::bench("r1restore", tol);
    let mut session = asm::open_bench(&bench, tol);
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "the open resolves: {row:?}");
    }

    // Take the post document out of the store WITHOUT touching the
    // session, then ask for as many fresh evaluations as you like.
    let post_file = bench.dir.join(format!("{}.pncad", bench.post.id));
    let saved = std::fs::read_to_string(&post_file).expect("the post file reads");
    std::fs::remove_file(&post_file).expect("the post file removes");
    for _ in 0..3 {
        session.perform(SessionOp::Reevaluate);
        session.pump();
    }
    for row in session.tree_rows() {
        assert_eq!(
            row.status,
            RowStatus::Ok,
            "SHIPPED BEHAVIOUR, recorded not endorsed: the memo answers for every \
             instantiate node, so the removed part is never noticed: {row:?}"
        );
    }

    // …while a FRESH open of the same file, against the same store,
    // badges its references typed. The open session and the file
    // disagree, and only the file is right.
    let fresh = asm::open_bench(&bench, tol);
    let mut failed = 0usize;
    for row in fresh.tree_rows() {
        if let RowStatus::Failed { message } = &row.status {
            assert!(
                message.contains("no document with id"),
                "the store's own refusal, at the instantiate node: {message}"
            );
            failed += 1;
        }
    }
    assert_eq!(
        failed, 2,
        "a fresh open sees what the live session cannot: both post instances refuse"
    );

    // A store that will not SCAN refuses each resolution it actually
    // reaches, naming the offending file — the posture's real content,
    // at the only place it is observable.
    std::fs::write(&post_file, saved).expect("the post file restores");
    std::fs::write(bench.dir.join("r1-junk.pncad"), "not a document").expect("the junk writes");
    let broken = asm::open_bench(&bench, tol);
    for row in broken.tree_rows() {
        match &row.status {
            RowStatus::Failed { message } => assert!(
                message.contains("r1-junk.pncad"),
                "the scan refusal names the offending file: {message}"
            ),
            other => panic!("a broken store must refuse each resolution, got {other:?}"),
        }
    }
    // …and that open still SUCCEEDED, which is the revision's actual
    // win: no document is held hostage by a messy directory.
    assert_eq!(broken.tree_rows().len(), 3, "the tree is whole");
}

/// **Save-as rebinds the resolver, and the rebind RE-RESOLVES** (the
/// fix-pass behaviour: the seam primes its memo only under the SAME
/// resolver, by `Arc` identity, so a rebind's next run consults the
/// new directory for every reference).
///
/// This row pinned the opposite as shipped — the rebind was inert
/// because the memo answered every instantiate node — and went red
/// when the memo gained its resolver gate, exactly as its own comment
/// said it would. It now asserts the stated intent: saving the
/// assembly into a directory holding no parts turns the live session's
/// instance rows RED, typed, naming the missing ids; a fresh open of
/// the moved file says the same.
#[test]
fn r1_save_as_rebinds_the_directory_and_the_rebind_re_resolves() {
    let tol = Tol::witness();
    let bench = asm::bench("r1saveas", tol);
    let mut session = asm::open_bench(&bench, tol);
    assert_eq!(session.resolve_dir().expect("a resolver"), bench.dir);

    let elsewhere = std::env::temp_dir().join(format!("gui4-r1-saveas-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&elsewhere);
    std::fs::create_dir_all(&elsewhere).expect("the target directory creates");
    let moved = elsewhere.join("bench.pncad");
    let outcome = session.perform(SessionOp::Save(moved.clone()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();

    assert_eq!(
        session.resolve_dir().expect("a resolver"),
        elsewhere,
        "the resolver followed the file"
    );
    let mut failed = 0usize;
    for row in session.tree_rows() {
        if let RowStatus::Failed { message } = &row.status {
            assert!(
                message.contains("no document with id"),
                "the LIVE session re-resolved against the new directory and \
                 says what it lacks: {message}"
            );
            failed += 1;
        }
    }
    assert_eq!(
        failed, 3,
        "every instance re-resolved against the partless directory and refused"
    );

    // What the file actually means in its new home.
    let mut reopened = DocSession::inline(pncad::document::Doc::empty_derived("r1-boot", tol), tol);
    assert!(
        reopened.perform(SessionOp::Open(moved)).refusal.is_none(),
        "the moved file opens"
    );
    reopened.pump();
    for row in reopened.tree_rows() {
        match &row.status {
            RowStatus::Failed { message } => assert!(
                message.contains("no document with id"),
                "the new directory has no parts, and a fresh open says so: {message}"
            ),
            other => panic!(
                "reopening the saved file in its partless directory must refuse \
                 typed, got {other:?}"
            ),
        }
    }
    let _ = std::fs::remove_dir_all(&elsewhere);
}

// ── 5. The class table, and what a Tangent commit means ──────────

/// The tool must never offer a class the doors will not execute, and
/// the verdict it shows beside a committable-but-unassemblable class
/// must be the kernel's own.
///
/// This row also PINS what a `Tangent` commit currently means, because
/// nothing else does: the document evaluates green, and the A5 at-rest
/// gate refuses it. If v1 later decides Tangent should refuse at the
/// tool instead, this row is what goes red.
#[test]
fn r1_every_offered_class_is_executable_and_a_tangent_commit_is_unassemblable() {
    let tol = Tol::witness();
    let classes = admitted_classes();
    assert!(!classes.is_empty(), "the tool offers something");
    for entry in &classes {
        assert_eq!(
            entry.admission,
            class_admission(entry.class),
            "the exposed verdict is the kernel's, for {}",
            entry.class.name()
        );
        assert_ne!(
            entry.admission,
            ClassAdmission::NotAdmitted,
            "the tool never offers a class the solve door refuses: {}",
            entry.class.name()
        );
    }
    assert!(
        classes.iter().any(|e| e.class == ContactClass::Rest)
            && classes.iter().any(|e| e.class == ContactClass::Tangent),
        "both kernel-nameable classes are offered: {classes:?}"
    );

    // A committed Tangent: green document, refused assembly.
    let bench = asm::bench("r1tangent", tol);
    let mut session = asm::open_bench(&bench, tol);
    let outcome = session.perform(SessionOp::AddMate {
        a: asm::in_part(bench.post_b, &bench.post_top),
        b: asm::in_part(bench.shelf_i, &bench.shelf_bottom),
        class: ContactClass::Tangent,
        alignment: seat(),
    });
    assert!(
        outcome.refusal.is_none(),
        "a Tangent mate commits: {:?}",
        outcome.refusal
    );
    session.pump();
    for row in session.tree_rows() {
        assert_eq!(
            row.status,
            RowStatus::Ok,
            "…and every tree row reads OK, which is the whole difficulty: {row:?}"
        );
    }
    let (doc, eval) = session.landed_pair().expect("landed");
    let text = match assemble(doc, eval, tol) {
        Err(refusal) => refusal.to_string(),
        Ok(_) => panic!("the at-rest gate must refuse a Tangent declaration"),
    };
    assert!(
        text.contains("no at-rest kernel record"),
        "the refusal is the class table's own: {text}"
    );
}

// ── 6. The gesture's own edges ───────────────────────────────────

/// The three edges of the display gesture nothing else covers: the
/// out-of-order refusals, the bit-exact identity commit, and the
/// in-flight gesture a landing mate kills.
#[test]
fn r1_the_probe_gestures_order_and_identity_edges() {
    let tol = Tol::witness();
    let bench = asm::bench("r1edges", tol);
    let mut session = asm::open_bench(&bench, tol);

    // Preview / commit / cancel with no gesture: typed, all three.
    for op in [
        SessionOp::PreviewFreeMove {
            frame: Frame::translation([0.01, 0.0, 0.0]),
        },
        SessionOp::CommitFreeMove,
        SessionOp::CancelFreeMove,
    ] {
        assert!(
            matches!(
                session.perform(op).refusal,
                Some(Refusal::Display(DisplayFault::NoFreeMove))
            ),
            "a gesture op with no gesture in flight refuses typed"
        );
    }

    // A second begin while one is open.
    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    assert!(
        matches!(
            session
                .perform(SessionOp::BeginFreeMove {
                    instance: bench.post_a,
                })
                .refusal,
            Some(Refusal::Display(DisplayFault::FreeMoveInFlight))
        ),
        "one gesture at a time"
    );

    // A bit-exact identity preview commits NOTHING — "probed to exactly
    // where the document draws it" must not leave a marked part.
    session.perform(SessionOp::PreviewFreeMove {
        frame: Frame::IDENTITY,
    });
    session.perform(SessionOp::CommitFreeMove);
    assert!(
        session.display().free_move_of(bench.post_b).is_none(),
        "an identity commit leaves no entry"
    );
    let index = asm::index_of(&session);
    assert_eq!(
        index
            .scene_for(&session.display_view())
            .expect("a scene")
            .stats()
            .probe_parts,
        0,
        "…and nothing is marked distinct"
    );

    // A gesture in flight when a mate lands on its instance dies — and
    // the death is NOT reported in `superseded` (which carries only
    // committed values). The next gesture op is the only place a caller
    // learns it, and it is typed.
    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    session.perform(SessionOp::PreviewFreeMove {
        frame: Frame::translation([0.03, 0.0, 0.0]),
    });
    let outcome = session.perform(SessionOp::AddMate {
        a: asm::in_part(bench.post_b, &bench.post_top),
        b: asm::in_part(bench.shelf_i, &bench.shelf_bottom),
        class: ContactClass::Rest,
        alignment: seat(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        outcome.superseded.is_empty(),
        "an in-flight gesture's death is not reported as a supersession — \
         recorded here as the current behaviour, not endorsed: {:?}",
        outcome.superseded
    );
    assert!(
        session.display().probing().is_none(),
        "the gesture is gone all the same"
    );
    assert!(
        matches!(
            session.perform(SessionOp::CommitFreeMove).refusal,
            Some(Refusal::Display(DisplayFault::NoFreeMove))
        ),
        "the commit that would have landed it refuses typed rather than \
         resurrecting an illegal value"
    );
}

// ── 7. Two picks on one instance, through the real cursor path ───

/// Both picks on ONE instance refuses at the tool, before any edit
/// exists — driven through the cursor path rather than by re-feeding
/// one `FaceSelection` twice, so the refusal is reached the way a user
/// reaches it (two different faces of the same part).
#[test]
fn r1_two_faces_of_one_instance_refuse_before_any_edit() {
    let tol = Tol::witness();
    let bench = asm::bench("r1samepart", tol);
    let session = asm::open_bench(&bench, tol);
    let index = asm::index_of(&session);
    let s = asm::POST_SECTION;

    let top = pick(
        &session,
        &index,
        &asm::down_at(asm::POST_B_AT[0] + s / 2.0, asm::POST_B_AT[1] + s / 2.0),
    );
    let bottom = pick(
        &session,
        &index,
        &up_at(asm::POST_B_AT[0] + s / 2.0, asm::POST_B_AT[1] + s / 2.0),
    );
    assert_eq!(top.node, bench.post_b);
    assert_eq!(bottom.node, bench.post_b);
    assert_ne!(top.name, bottom.name, "two DIFFERENT faces of one instance");

    let mut tool = MateTool::new();
    tool.pick(top);
    tool.pick(bottom);
    let (doc, eval) = session.landed_pair().expect("landed");
    match tool.proposal(doc, eval, tol, rest_choice()) {
        Err(viewer::matetool::MateToolError::SamePick { instance }) => {
            assert_eq!(instance, bench.post_b);
        }
        other => panic!("a self-mate must refuse at the tool, got {other:?}"),
    }
    // Nothing entered the document.
    assert!(
        !session.tree_rows().iter().any(|r| r.kind == "Mate"),
        "a refused proposal authors nothing"
    );
}

// ── 8. An instance a `Pattern` consumes ──────────────────────────

/// **Both G3 display operations PROPAGATE to an instance a `Pattern`
/// consumes** (the fix-pass rule; this row pinned the silent no-op as
/// shipped and went red when propagation landed).
///
/// Display state names the INSTANCE; the drawn scene is keyed by
/// product roots; `display::drawn_targets` resolves one to the other.
/// Hiding a patterned instance therefore hides every placed copy (the
/// pattern's whole drawn root), and probing it displaces and marks
/// them — the pattern replicates the instance, and the display fact
/// is the instance's. The `Pattern` node itself still has no display
/// identity (`NotAnInstance`), which is right: the instance is the
/// thing with an identity a user hides or probes.
///
/// The shape is the tour's flat-pack layout, where the original
/// silent no-op was measured (`examples/r1_gallery_probe.rs`, which
/// now asserts the propagated behaviour on the real gallery).
#[test]
fn r1_a_patterned_instance_propagates_hide_and_probe_to_the_drawn_pattern() {
    let tol = Tol::witness();
    let bench = asm::bench("r1pattern", tol);
    let scope = std::collections::BTreeMap::new();
    let mut ws = Workspace::open(&bench.dir).expect("the store opens");
    let mut doc = ProfileDoc::empty(DocumentId::derive("r1-pattern-bench"), tol);
    let applied = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::instantiate_part(bench.post),
        },
        tol,
    )
    .expect("the insert applies");
    doc = applied.doc;
    let instance = applied.record.minted.expect("an id");
    let applied = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Pattern {
                input: instance,
                count: parse_expr("3", &scope).expect("a count"),
                kind: PatternKind::Linear {
                    direction: [
                        parse_expr("0.0", &scope).expect("x"),
                        parse_expr("1.0", &scope).expect("y"),
                        parse_expr("0.0", &scope).expect("z"),
                    ],
                    spacing: parse_expr("50 mm", &scope).expect("a spacing"),
                },
            },
        },
        tol,
    )
    .expect("the pattern applies");
    doc = applied.doc;
    let pattern = applied.record.minted.expect("an id");
    let path = ws.create(&doc, tol).expect("the pattern assembly stores");

    let mut session = DocSession::inline(pncad::document::Doc::empty_derived("r1-boot", tol), tol);
    assert!(
        session.perform(SessionOp::Open(path)).refusal.is_none(),
        "the pattern assembly opens"
    );
    session.pump();
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "it resolves: {row:?}");
    }
    let index = asm::index_of(&session);
    let baseline = index
        .scene_for(&session.display_view())
        .expect("a scene")
        .stats()
        .triangles;

    // HIDE: accepted, and the pattern's whole drawn root leaves the
    // picture — hiding the instance hides its placed copies.
    assert!(
        session
            .perform(SessionOp::SetInstanceHidden {
                instance,
                hidden: true,
            })
            .refusal
            .is_none(),
        "hide is accepted on the patterned instance"
    );
    assert!(
        session.display().is_hidden(instance),
        "the state says hidden"
    );
    let hidden_scene = index.scene_for(&session.display_view()).expect("a scene");
    assert_eq!(
        hidden_scene.stats().triangles,
        0,
        "the pattern is the document's ONLY drawn root, so hiding its \
         instance draws the honest empty picture — not the stale one"
    );
    // …and the pick index dropped it with the picture.
    {
        let (_, eval) = session.landed_pair().expect("landed");
        assert!(
            index
                .pick_for(
                    eval,
                    &asm::down_at(asm::POST_SECTION / 2.0, asm::POST_SECTION / 2.0),
                    &session.display_view()
                )
                .expect("the pick answers")
                .is_none(),
            "a hidden pattern copy is out of the pick index"
        );
    }
    session.perform(SessionOp::SetInstanceHidden {
        instance,
        hidden: false,
    });
    assert_eq!(
        index
            .scene_for(&session.display_view())
            .expect("a scene")
            .stats()
            .triangles,
        baseline,
        "unhiding restores every copy"
    );

    // FREE-MOVE: eligible, committed, and every placed copy is drawn
    // displaced and marked distinct under the one probe frame.
    assert!(
        viewer::display::free_move_check(session.doc(), instance).is_ok(),
        "the patterned instance is 'completely unconstrained' by the document test"
    );
    session.perform(SessionOp::BeginFreeMove { instance });
    session.perform(SessionOp::PreviewFreeMove {
        frame: Frame::translation([0.5, 0.0, 0.0]),
    });
    assert!(
        session.perform(SessionOp::CommitFreeMove).refusal.is_none(),
        "the probe commits"
    );
    assert!(
        session.display().free_move_of(instance).is_some(),
        "the state holds the probe value"
    );
    let probed = index.scene_for(&session.display_view()).expect("a scene");
    assert_eq!(
        probed.stats().probe_parts,
        3,
        "every placed copy — one drawn part per pattern body — is marked"
    );
    assert!(
        probed.flags().contains(&SceneMesh::FLAG_PROBE),
        "…its corners carry the flag"
    );
    assert!(
        probed.bounds().max_x > 0.5,
        "…and the copies are DRAWN displaced by the probe: {:?}",
        probed.bounds()
    );

    // The `Pattern` node itself, which is what IS drawn, refuses both.
    assert!(
        matches!(
            session
                .perform(SessionOp::SetInstanceHidden {
                    instance: pattern,
                    hidden: true,
                })
                .refusal,
            Some(Refusal::Display(DisplayFault::NotAnInstance { .. }))
        ),
        "so the drawn thing has no display identity at all"
    );
}

// ── 9. A workspace with no parts at all ──────────────────────────

/// An assembly whose store is empty: the open still SUCCEEDS (a
/// document is not held hostage by its directory), and every instantiate
/// node badges typed. The negative half of the resolver contract, taken
/// through a directory that is healthy but empty rather than corrupt.
#[test]
fn r1_an_assembly_alone_in_an_empty_directory_opens_and_badges() {
    let tol = Tol::witness();
    let bench = asm::bench("r1alone", tol);
    let alone = std::env::temp_dir().join(format!("gui4-r1-alone-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&alone);
    std::fs::create_dir_all(&alone).expect("the directory creates");
    let copy = alone.join("assembly.pncad");
    std::fs::copy(&bench.asm_path, &copy).expect("the assembly copies");

    let mut session = DocSession::inline(pncad::document::Doc::empty_derived("r1-boot", tol), tol);
    let outcome = session.perform(SessionOp::Open(copy));
    assert!(
        outcome.refusal.is_none(),
        "the open succeeds even with nothing to resolve against: {:?}",
        outcome.refusal
    );
    session.pump();
    assert_eq!(
        session.resolve_dir().expect("a resolver"),
        alone,
        "the resolver is the opened file's directory"
    );
    let rows = session.tree_rows();
    assert_eq!(rows.len(), 3, "the tree still shows every instance");
    for row in &rows {
        match &row.status {
            RowStatus::Failed { message } => assert!(
                message.contains("no document with id"),
                "each instantiate refuses typed, naming the missing id: {message}"
            ),
            other => panic!("expected a typed resolution refusal, got {other:?}"),
        }
    }
    // The hide op still works on an unresolvable instance — display
    // state is about the DOCUMENT's nodes, not about what evaluated.
    let hidden = session.perform(SessionOp::SetInstanceHidden {
        instance: RecipeNodeId(rows[0].id.0),
        hidden: true,
    });
    assert!(
        hidden.refusal.is_none(),
        "hide reads the document, not the evaluation: {:?}",
        hidden.refusal
    );
    let _ = std::fs::remove_dir_all(&alone);
}
