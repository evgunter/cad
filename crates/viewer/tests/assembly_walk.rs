//! **The exit-demo walk** (GUI-4 deliverable 5) — the v1 GUI
//! program's acceptance sequence, end-to-end through the typed
//! vocabulary on the gallery-shaped assembly:
//!
//! open → resolve → the tree shows the instances → hide one →
//! free-move an unconstrained one (visibly distinct, asserted as a
//! value) → two face picks → the admitted class chosen → ONE
//! committed mate edit → the placement solved → the free-move
//! superseded → save/reopen round-trips the document with every piece
//! of layer-3 state gone.
//!
//! One test, deliberately: this row IS the program's acceptance
//! evidence, and its readability is part of the deliverable. Each
//! stage is a numbered block whose assertions say what the stage
//! claims; the helpers live in `common::asm` so the walk reads as the
//! walk.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use common::asm;
use pncad::document::{AxisSense, ClassAdmission, Frame, MatePrimitive, solve_document};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::select::{ContactClass, Ray};
use viewer::matetool::{MateChoice, MateTool, MateToolState, admitted_classes};
use viewer::scene::SceneMesh;
use viewer::session::SessionOp;
use viewer::tree::RowStatus;

#[test]
fn the_exit_demo_walk() {
    let tol = Tol::witness();

    // ── 1. The gallery assembly: a workspace directory holding two
    // part documents and the assembly that pins them, exactly the
    // shape `demo-tour gallery` writes into `gallery/assembly/`.
    let bench = asm::bench("walk", tol);

    // ── 2. OPEN through the typed door. The open path wires the
    // resolver over the opened file's OWN directory (the directory
    // rule), so the assembly evaluates instead of refusing.
    let mut session = asm::open_bench(&bench, tol);
    assert_eq!(
        session.resolve_dir().expect("the resolver is wired"),
        bench.dir,
        "resolution is against the opened document's directory"
    );

    // ── 3. RESOLVE: the tree shows the three instances, every row ok.
    let rows = session.tree_rows();
    assert_eq!(rows.len(), 3);
    for row in &rows {
        assert_eq!(row.kind, "InstantiatePart");
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }

    // ── 4. HIDE one instance (post_a). The scene and the pick index
    // drop it; the tree and the document keep it. It STAYS hidden for
    // the rest of the walk, so the reopen at the end demonstrates the
    // state dying with the session.
    let index = asm::index_of(&session);
    let full_triangles = index
        .scene_for(&session.display_view())
        .expect("a scene")
        .stats()
        .triangles;
    session.perform(SessionOp::SetInstanceHidden {
        instance: bench.post_a,
        hidden: true,
    });
    let view = session.display_view();
    assert!(
        index.scene_for(&view).expect("a scene").stats().triangles < full_triangles,
        "the picture drops the hidden instance"
    );
    let (_, eval) = session.landed_pair().expect("landed");
    let over_post_a = asm::down_at(asm::POST_SECTION / 2.0, asm::POST_SECTION / 2.0);
    assert!(
        index
            .pick_for(eval, &over_post_a, &view)
            .expect("answers")
            .is_none(),
        "the pick index drops it too"
    );
    assert!(
        session.tree_rows().iter().any(|row| row.id == bench.post_a),
        "the tree keeps it"
    );
    assert!(
        session.doc().node(bench.post_a).is_some(),
        "so does the document"
    );

    // ── 5. FREE-MOVE the completely-unconstrained post_b: the G3 fit
    // probe. Eligibility is derived from the document (no mate names
    // it); the gesture is preview-stream + one committed value; the
    // probed part is drawn DISPLACED and MARKED — the honesty
    // treatment, asserted as a value in the scene rows.
    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    session.perform(SessionOp::PreviewFreeMove {
        frame: Frame::translation([0.03, 0.0, 0.0]),
    });
    let outcome = session.perform(SessionOp::CommitFreeMove);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let probed = index.scene_for(&session.display_view()).expect("a scene");
    assert_eq!(probed.stats().probe_parts, 1, "one probed part");
    assert!(
        probed.flags().contains(&SceneMesh::FLAG_PROBE),
        "…and its corners carry the distinctness flag"
    );

    // ── 6. TWO PICKS through the real cursor path — the GUI-2
    // single-select vocabulary, held in tool state. The first pick is
    // the probed post's top cap AT ITS DRAWN (probed) position; the
    // second is the shelf's underside, picked from below.
    let mut tool = MateTool::new();
    let view = session.display_view();
    let (_, eval) = session.landed_pair().expect("landed");
    let post_top = index
        .pick_for(
            eval,
            &asm::down_at(
                asm::POST_B_AT[0] + 0.03 + asm::POST_SECTION / 2.0,
                asm::POST_B_AT[1] + asm::POST_SECTION / 2.0,
            ),
            &view,
        )
        .expect("answers")
        .expect("the probed post is picked where it is drawn");
    assert_eq!(post_top.node, bench.post_b);
    tool.pick(viewer::session::FaceSelection {
        name: post_top.name.clone(),
        node: post_top.node,
        body: post_top.body,
    });
    let shelf_bottom = index
        .face_at_for(
            eval,
            &Ray {
                origin: Point3::new(
                    asm::SHELF_AT[0] + asm::SHELF_LENGTH / 2.0,
                    asm::SHELF_AT[1] + asm::SHELF_DEPTH / 2.0,
                    -1.0,
                ),
                dir: Vec3::new(0.0, 0.0, 1.0),
            },
            &view,
        )
        .expect("answers")
        .expect("the shelf's underside is picked");
    assert_eq!(shelf_bottom.node, bench.shelf_i);
    tool.pick(shelf_bottom);
    assert!(matches!(tool.state(), MateToolState::Two { .. }));

    // ── 7. The ADMITTED CLASS, exposed through the kernel's own
    // table: Rest mints; Tangent solves but has no at-rest record.
    let classes = admitted_classes();
    assert_eq!(classes[0].class, ContactClass::Rest);
    assert_eq!(classes[0].admission, ClassAdmission::Mints);
    assert!(matches!(
        classes[1].admission,
        ClassAdmission::NoAtRestRecord { .. }
    ));

    // ── 8. ONE COMMITTED MATE EDIT. The proposal derives the
    // alignment frames from the picked faces (world pose through
    // `face_frame`, pulled into part coordinates through each
    // instance's placement); the session commits exactly one edit —
    // and the SAME outcome reports the free-move superseded:
    // discarded, not zeroed.
    let (doc, eval) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(
            doc,
            eval,
            tol,
            MateChoice {
                class: ContactClass::Rest,
                primitive: MatePrimitive::FrameCoincidence,
                sense: AxisSense::Opposed,
                clocking: None,
            },
        )
        .expect("the seat proposes");
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1, "exactly one committed edit");
    assert_eq!(
        outcome.superseded,
        vec![bench.post_b],
        "the mate's landing discards the probe, in the same outcome"
    );
    assert!(session.display().free_move_of(bench.post_b).is_none());

    // ── 9. The PLACEMENT IS SOLVED: the two picked frames coincide
    // in world space, and the instance is drawn at the solved
    // placement with no probe marking anywhere.
    session.pump();
    let (doc, _) = session.landed_pair().expect("landed");
    let poses = solve_document(doc, tol);
    let placed_a = poses
        .placement(doc, bench.post_b)
        .expect("post_b is solved")
        .affine::<f64>();
    let placed_b = poses
        .placement(doc, bench.shelf_i)
        .expect("the shelf is placed")
        .affine::<f64>();
    let [ax, ay, az] = proposal.alignment.a.origin;
    let [bx, by, bz] = proposal.alignment.b.origin;
    let world_a = placed_a.transform_point(Point3::new(ax, ay, az));
    let world_b = placed_b.transform_point(Point3::new(bx, by, bz));
    assert!(
        (world_a.x - world_b.x).abs() < 1e-9
            && (world_a.y - world_b.y).abs() < 1e-9
            && (world_a.z - world_b.z).abs() < 1e-9,
        "the mated frames coincide: {world_a:?} vs {world_b:?}"
    );
    let solved_index = asm::index_of(&session);
    let solved_scene = solved_index
        .scene_for(&session.display_view())
        .expect("a scene");
    assert_eq!(
        solved_scene.stats().probe_parts,
        0,
        "no probe survives the mate"
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }

    // ── 10. SAVE and REOPEN. The document round-trips WITH the mate;
    // every piece of layer-3 state — the hidden instance, the (already
    // superseded) probe, the tool's held picks — is gone, because none
    // of it was ever the document's.
    assert!(
        !session.display().hidden().is_empty(),
        "the walk still holds layer-3 state at save time"
    );
    let outcome = session.perform(SessionOp::Save(bench.asm_path.clone()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let reopened = asm::open_bench(&bench, tol);
    assert!(
        reopened.display().hidden().is_empty(),
        "hide died with the session"
    );
    assert!(
        reopened.display().free_move_of(bench.post_b).is_none(),
        "so did the probe"
    );
    let rows = reopened.tree_rows();
    assert_eq!(rows.len(), 4, "three instances and the mate");
    assert!(rows.iter().any(|row| row.kind == "Mate"));
    for row in &rows {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    // The solved placement survives the round trip bit-for-bit at the
    // assertion tolerance: the mate is the document, the probe never
    // was.
    let (doc2, _) = reopened.landed_pair().expect("landed");
    let re_placed = solve_document(doc2, tol)
        .placement(doc2, bench.post_b)
        .expect("post_b is solved after reopen")
        .affine::<f64>();
    let re_world = re_placed.transform_point(Point3::new(ax, ay, az));
    assert!(
        (re_world.x - world_a.x).abs() < 1e-12
            && (re_world.y - world_a.y).abs() < 1e-12
            && (re_world.z - world_a.z).abs() < 1e-12,
        "the solved placement round-trips: {re_world:?} vs {world_a:?}"
    );
}
