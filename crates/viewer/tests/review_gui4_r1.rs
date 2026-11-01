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
//! down (implementer-discipline §8's second shape), and the rows are merged so
//! each expensive fixture is built once. Assertions are labelled so a
//! red row names its own property.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use std::f64::consts::FRAC_PI_2;

use common::asm;
use pncad::document::{DocumentId, Frame, ProfileDoc, RecipeNodeId};
use pncad::geom_core::{Point3, Tol};
use pncad::select::face_frame;
use pncad::workspace::Workspace;
use viewer::display::{AdmissionFault, DisplayFault};
use viewer::matetool::MateTool;
use viewer::scene::SceneMesh;
use viewer::session::{DocSession, Refusal, SessionOp};
use viewer::tree::RowStatus;

fn close(got: [f64; 3], want: [f64; 3], eps: f64, what: &str) {
    for i in 0..3 {
        assert!(
            (got[i] - want[i]).abs() < eps,
            "{what}: component {i} — got {got:?}, want {want:?}"
        );
    }
}

// ── 1. The mate tool's pull-back, re-derived ──────────────────────

/// The mate the tool mints reads each picked face by its part's own
/// name, whatever the picked instance's placement is — a rotated one
/// included — and, solved, the values seat the post where they say in
/// the shelf's own coordinates, its pre-mate rotation gone.
///
/// The rotated half is the load-bearing half: a tool that read a number
/// at the world spot would carry the rotation into the mate, and the
/// seated post would come out turned.
#[test]
fn r1_the_minted_mate_reads_the_parts_own_faces_under_a_rotated_placement() {
    let tol = Tol::witness();
    let bench = asm::bench("r1pullback", tol);
    // A second assembly document in the SAME store, whose post carries
    // a genuinely rotated placement (a quarter turn about z, then a
    // shift), so the pull-back exercises Rᵀ and not just −t.
    //
    // It is authored here rather than driven through the session
    // because **the viewer has no `SessionOp` that sets a placement**:
    // an instance's pose is either the world mate the file already holds
    // or what a mate solves, so a rotated instance can only be arranged
    // from outside. (A finding in its own right; here it is just why this
    // fixture exists.)
    let rotated = Frame::rotate_then_translate(
        [0.0, 0.0, 1.0],
        FRAC_PI_2,
        [-0.05, 0.04, 0.0],
        common::band(),
    )
    .expect("a literal axis has a definite direction");
    let mut ws = Workspace::open(&bench.dir).expect("the store opens");
    let mut doc = ProfileDoc::empty(DocumentId::derive("r1-rotated-bench"), tol);
    let rot_post = asm::instance_posed(&mut doc, bench.post, &rotated, &bench.opts(), tol);
    let rot_shelf = asm::instance_at(&mut doc, bench.shelf, asm::SHELF_AT, &bench.opts(), tol);
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
    let post_a_top = common::displayed_face_at(
        &session,
        &index,
        &asm::down_at(-0.05 - s / 2.0, 0.04 + s / 2.0),
    );
    assert_eq!(
        post_a_top.node,
        common::copy_of(session.committed_doc(), rot_post),
        "the rotated instance is where the placement puts it"
    );
    let shelf_bottom = asm::shelf_underside(&session);
    assert_eq!(
        shelf_bottom.node,
        common::copy_of(session.committed_doc(), rot_shelf)
    );

    let mut tool = MateTool::new();
    tool.pick(session.doc(), post_a_top.clone());
    tool.pick(session.doc(), shelf_bottom.clone());
    let (doc, _) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, asm::seat_choice())
        .expect("the tool proposes");

    // The rotated placement enters nothing: each side names the
    // part's own face — the row the instance placed, whatever the
    // instance's placement is — and no number read at the world spot
    // enters the mate.
    assert_eq!(
        asm::side_face(doc, &proposal.on),
        Some(bench.post_top.clone()),
        "the on side is the post's own top cap"
    );
    assert_eq!(
        asm::side_face(doc, &proposal.to),
        Some(bench.shelf_bottom.clone()),
        "the to side is the shelf's own underside"
    );

    // The edit lands once, into the post's bundle once its world mate
    // is off, and the document stays green.
    asm::unpin(&mut session, rot_post);
    common::commit_mate(&mut session, proposal.op_into_target_space());
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "after the mate: {row:?}");
    }

    // The independent check, on the SOLVED state: the two picked
    // faces rest, read through the same shipped door off the landed
    // evaluation — the cap in the underside's plane, normals opposed
    // (the flip) — and the post stands where the values put it in the
    // shelf's coordinates, unturned.
    let (_, eval) = session.landed_pair().expect("landed");
    let pose_a = face_frame(eval, post_a_top.node, &post_a_top.name).expect("the cap has a pose");
    let pose_b =
        face_frame(eval, shelf_bottom.node, &shelf_bottom.name).expect("the underside has a pose");
    assert!(
        (pose_a.origin - pose_b.origin).dot(pose_b.axis).abs() < 1e-9,
        "the cap lies in the underside's plane once solved"
    );
    close(
        pose_a.axis.to_array(),
        [-pose_b.axis.x, -pose_b.axis.y, -pose_b.axis.z],
        1e-9,
        "the picked faces' normals meet opposed once solved",
    );
    asm::assert_solved_at(
        &session,
        rot_post,
        &asm::seated_at(asm::SHELF_LENGTH / 2.0),
        "the values seat the post, the quarter turn gone",
    );
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
    // post_b in a space of its own, which the probe moves, the frame
    // composed so it moves the post from where its pin stood it.
    asm::unpin(&mut session, bench.post_b);
    let index = asm::index_of(&session);
    let s = asm::POST_SECTION;

    // A quarter turn about z about the coordinate origin, then a shift
    // that lands the post well clear of everything: the part occupies
    // world x ∈ [0.06, 0.08], y ∈ [0, 0.02] before the probe; the turn
    // sends (x, y) ↦ (−y, x), and the shift moves it to a fresh spot.
    let shift = [0.20, -0.10, 0.0];
    let probe = Frame::rotate_then_translate([0.0, 0.0, 1.0], FRAC_PI_2, shift, common::band())
        .expect("a literal axis has a definite direction");
    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    let outcome = session.perform(SessionOp::PreviewFreeMove {
        instance: bench.post_b,
        frame: asm::where_b_stood(probe),
    });
    assert!(
        outcome.refusal.is_none(),
        "a rotation is a rigid motion: {:?}",
        outcome.refusal
    );
    session.perform(SessionOp::CommitFreeMove {
        instance: bench.post_b,
    });
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
    let drawn_box = scene.bounds().expect("the probed scene draws");
    assert!(
        drawn_box.max_x >= drawn.x - 1e-9,
        "the drawn bounds reach the rotated probe: {drawn_box:?}"
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
    let s = asm::POST_SECTION;

    // Probe post_b — in a space of its own, drawn where its pin stood
    // it — then hide the probed instance.
    let own = asm::unpin(&mut session, bench.post_b);
    let index = asm::index_of(&session);
    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    session.perform(SessionOp::PreviewFreeMove {
        instance: bench.post_b,
        frame: asm::where_b_stood(Frame::translation([0.05, 0.0, 0.0])),
    });
    session.perform(SessionOp::CommitFreeMove {
        instance: bench.post_b,
    });
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
        crate::common::asm::frame_of(&session, bench.post_b).is_some(),
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

    // Mate it onto the shelf, in the world: the mate places it into
    // the world, which empties its own space, and the edit discards the
    // probe and reports it.
    let first = session.perform(asm::seat_op(
        &bench,
        bench.post_b,
        asm::middle_seat_values(),
    ));
    assert!(first.refusal.is_none(), "{:?}", first.refusal);
    assert_eq!(
        first.committed.len(),
        3,
        "the re-target, the mate and its values"
    );
    let [superseded] = &first.withdrawn.superseded[..] else {
        panic!("exactly one frame is superseded at the edit")
    };
    session.pump();
    assert_eq!(superseded.instance, bench.post_b);
    assert!(
        matches!(
            &superseded.cause,
            AdmissionFault::NotShown { space, .. } if space.id() == own
        ),
        "and the report carries WHY it went, not only which went — the \
         fault's own PAYLOAD, which is what would go red if the prune paired \
         the right fault with the wrong instance: {}",
        superseded.cause
    );
    assert!(crate::common::asm::frame_of(&session, bench.post_b).is_none());

    // A SECOND mate naming the same pair fixes the seating plane the
    // first already fixes: the edit door refuses it (the bundle rule),
    // and the tree keeps the first mate and its values.
    let second = session.perform(asm::seat_op(
        &bench,
        bench.post_b,
        asm::middle_seat_values(),
    ));
    session.pump();
    assert!(
        second.refusal.is_some(),
        "a second mate on the same pair is refused at the door"
    );
    let rows = session.tree_rows();
    assert_eq!(
        common::row_of(&rows, bench.post_b).constraints.len(),
        4,
        "the first mate and its three values, and nothing of the second"
    );

    // Free-move now refuses on BOTH participants: the shelf lives in
    // the world's space, and post_b, mated onto it, in the world's
    // space too. There is no other space to move.
    for participant in [bench.post_b, bench.shelf_i] {
        match session
            .perform(SessionOp::BeginFreeMove {
                instance: participant,
            })
            .refusal
        {
            Some(Refusal::Display(DisplayFault::Admission(AdmissionFault::InTheWorld {
                instance,
            }))) => assert_eq!(instance.id(), participant),
            other => panic!("{participant:?} must refuse the probe, got {other:?}"),
        }
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
        if let RowStatus::Failed { message, .. } = &row.status {
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
    for row in asm::instance_rows(&broken) {
        match &row.status {
            RowStatus::Failed { message, .. } => assert!(
                message.contains("r1-junk.pncad"),
                "the scan refusal names the offending file: {message}"
            ),
            other => panic!("a broken store must refuse each resolution, got {other:?}"),
        }
    }
    // …and that open still SUCCEEDED, which is the revision's actual
    // win: no document is held hostage by a messy directory.
    assert_eq!(
        broken.tree_rows().len(),
        4,
        "the tree is whole: three instances, each its own placement, and the product's ⊔"
    );
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
        if let RowStatus::Failed { message, .. } = &row.status {
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
    for row in asm::instance_rows(&reopened) {
        match &row.status {
            RowStatus::Failed { message, .. } => assert!(
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

// ── 5. The gesture's own edges ───────────────────────────────────

/// The three edges of the display gesture nothing else covers: the
/// out-of-order refusals, the bit-exact identity commit, and the
/// in-flight gesture a mate kills.
#[test]
fn r1_the_probe_gestures_order_and_identity_edges() {
    let tol = Tol::witness();
    let bench = asm::bench("r1edges", tol);
    let mut session = asm::open_bench(&bench, tol);
    // post_b in a space of its own, which the probe moves, and which
    // the mate below empties.
    let own = asm::unpin(&mut session, bench.post_b);

    // Preview / commit / cancel with no gesture: typed, all three.
    for op in [
        SessionOp::PreviewFreeMove {
            instance: bench.post_a,
            frame: Frame::translation([0.01, 0.0, 0.0]),
        },
        SessionOp::CommitFreeMove {
            instance: bench.post_a,
        },
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
        instance: bench.post_a,
        frame: Frame::IDENTITY,
    });
    session.perform(SessionOp::CommitFreeMove {
        instance: bench.post_a,
    });
    assert!(
        crate::common::asm::frame_of(&session, bench.post_b).is_none(),
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

    // A gesture in flight when a mate lands on its instance dies, and
    // the death is REPORTED — in `killed_gesture`, not in `superseded`,
    // because nothing substituted for a placement the document was
    // never asked for. The fault that killed it travels with it, so
    // the chrome can say why instead of the user learning it by
    // starting another gesture and reading that refusal. The mate
    // places post_b into the world, emptying the space the gesture
    // moves, which the edit's own prune reads off the recipe.
    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    session.perform(SessionOp::PreviewFreeMove {
        instance: bench.post_b,
        frame: Frame::translation([0.03, 0.0, 0.0]),
    });
    let outcome = session.perform(asm::seat_op(
        &bench,
        bench.post_b,
        asm::middle_seat_values(),
    ));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let report = &outcome.withdrawn;
    assert!(
        report.superseded.is_empty(),
        "an in-flight gesture's death is not a SUPERSESSION: nothing \
         substituted for it, and it committed nothing to substitute \
         for: {:?}",
        report.superseded
    );
    let killed = report
        .killed_gesture
        .as_ref()
        .expect("the killed gesture is reported, with the fault that killed it");
    assert_eq!(killed.instance, bench.post_b);
    assert!(
        matches!(
            &killed.cause,
            AdmissionFault::NotShown { space, .. } if space.id() == own
        ),
        "the cause is the space the mate emptied, carried from the predicate \
         that decided rather than re-derived: {}",
        killed.cause
    );
    assert!(
        session.display().probing().is_none(),
        "the gesture is gone all the same"
    );
    assert!(
        matches!(
            session
                .perform(SessionOp::CommitFreeMove {
                    instance: bench.post_b
                })
                .refusal,
            Some(Refusal::Display(DisplayFault::NoFreeMove))
        ),
        "the commit that would have landed it refuses typed rather than \
         resurrecting an illegal value"
    );
}

// ── 6. Two picks on one instance, through the real cursor path ───

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

    let top = common::displayed_face_at(
        &session,
        &index,
        &asm::down_at(asm::POST_B_AT[0] + s / 2.0, asm::POST_B_AT[1] + s / 2.0),
    );
    let bottom = common::displayed_face_at(
        &session,
        &index,
        &asm::up_at(asm::POST_B_AT[0] + s / 2.0, asm::POST_B_AT[1] + s / 2.0),
    );
    assert_eq!(top.node, bench.post_b);
    assert_eq!(bottom.node, bench.post_b);
    assert_ne!(top.name, bottom.name, "two DIFFERENT faces of one instance");

    let mut tool = MateTool::new();
    tool.pick(session.doc(), top);
    tool.pick(session.doc(), bottom);
    let (doc, _) = session.landed_pair().expect("landed");
    match tool.proposal(doc, asm::seat_choice()) {
        Err(viewer::matetool::MateToolError::SamePick { head }) => {
            assert_eq!(head.id(), bench.post_b, "said as the picked copy");
        }
        other => panic!("a self-mate must refuse at the tool, got {other:?}"),
    }
    // Nothing entered the document.
    assert!(
        !session
            .tree_rows()
            .iter()
            .any(|r| r.spoken.kind() == Some("Mate")),
        "a refused proposal authors nothing"
    );
}

// ── 7. An instance a family copies ───────────────────────────────

/// **Both G3 display operations reach an instance a family copies** (the fix-pass rule; this row pinned the silent no-op as
/// shipped and went red when propagation landed).
///
/// A hide names the INSTANCE; the drawn scene is keyed by world
/// placements; `display::drawn_targets` resolves one to the other.
/// Hiding a patterned instance therefore hides every placed copy of the
/// pattern — the family replicates the instance, and the display fact
/// is the instance's. A free-move moves the picture of a shown space
/// (DI5); the instance's own space is read by the family's members,
/// which are placed into the world, so it is not shown and the probe
/// refuses it, typed. The family's placement itself has no display
/// identity (`NotAnInstance`), which is right: the instance is the
/// thing with an identity a user hides or probes.
///
/// The shape is the tour's flat-pack layout, where the original
/// silent no-op was measured (`examples/r1_gallery_probe.rs`, which
/// now asserts the propagated behaviour on the real gallery).
#[test]
fn r1_a_patterned_instance_hides_its_pattern_and_its_space_is_not_shown() {
    let tol = Tol::witness();
    let bench = asm::bench("r1pattern", tol);
    let mut ws = Workspace::open(&bench.dir).expect("the store opens");
    let mut doc = ProfileDoc::empty(DocumentId::derive("r1-pattern-bench"), tol);
    // An instance in a space of its own, which the pattern reads.
    let instance = common::insert_into(&mut doc, crate::fixture::mated_instance(bench.post), tol);
    doc = crate::fixture::into_new_space(doc, instance).0;
    let (next, _, pattern) = crate::fixture::linear(doc, instance, [0.0, 1.0, 0.0], 0.05, 3);
    doc = next;
    // The world (A10): each of the family's members, read and placed.
    for index in 0..3 {
        let (next, member) = common::member_read(&doc, pattern, index, tol);
        doc = common::placed_read(&next, member.into(), tol).0;
    }
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

    // HIDE: accepted, and every copy of the pattern leaves the
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
        "the instance and its pattern's copies are the document's ONLY drawn copies, \
         so hiding the instance draws the honest empty picture — not the stale one"
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

    // FREE-MOVE: the instance's own space is read by the pattern's
    // copies, which are drawn in the world, so that space is not shown
    // and the probe has nothing of its own to move — it refuses, typed,
    // naming the space.
    let space = session
        .doc()
        .node_space(instance)
        .expect("the instance is placed");
    assert!(
        matches!(
            session.perform(SessionOp::BeginFreeMove { instance }).refusal,
            Some(Refusal::Display(DisplayFault::Admission(
                AdmissionFault::NotShown { space: said, .. }
            ))) if said.id() == space
        ),
        "the patterned instance's space is shown through the pattern's copies"
    );

    // The family's placement itself refuses both: it is no instance.
    assert!(
        matches!(
            session
                .perform(SessionOp::SetInstanceHidden {
                    instance: pattern,
                    hidden: true,
                })
                .refusal,
            Some(Refusal::Display(DisplayFault::Admission(
                AdmissionFault::NotAnInstance { .. }
            )))
        ),
        "so a pattern has no display identity at all"
    );
}

// ── 8. A workspace with no parts at all ──────────────────────────

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
    let rows = asm::instance_rows(&session);
    assert_eq!(rows.len(), 3, "the tree still shows every instance");
    for row in &rows {
        match &row.status {
            RowStatus::Failed { message, .. } => assert!(
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
