//! **The exit-demo walk** (GUI-4 deliverable 5) — the v1 GUI
//! program's acceptance sequence, end-to-end through the typed
//! vocabulary on the gallery-shaped assembly:
//!
//! open → resolve → the tree shows the instances → hide one →
//! free-move an unconstrained one (visibly distinct, asserted as a
//! value) → two face picks → ONE committed mate edit → the placement solved → the free-move
//! superseded → save/reopen round-trips the document with every piece
//! of layer-3 state gone.
//!
//! One test, deliberately: this row IS the program's acceptance
//! evidence, and its readability is part of the deliverable. Each
//! stage is a numbered block whose assertions say what the stage
//! claims; the helpers live in `common::asm` so the walk reads as the
//! walk.
//!
//! # Why this walk runs on a FIXTURE, and where the real gallery is
//! exercised
//!
//! The walk needs, in one document: an instance to hide and one to
//! probe, in a space of its own, two matable faces, and no
//! pre-existing mate. **No single real gallery document has that
//! shape** — the tour's stand has all three instances in the world's
//! space (the free-move stage is unreachable there), and its flat-pack has no second part to mate
//! the patterned posts to. So the walk authors a gallery-SHAPED
//! workspace through the same public doors the tour uses (parts
//! beside the assembly, id-named files, `Workspace::create`) and
//! walks the FULL sequence on it; the REAL gallery is exercised
//! per-item where its documents allow — hosted CI's render lane runs
//! `demo-tour gallery` and drives `examples/r1_gallery_probe.rs`
//! over its actual output (every document opens and resolves; hide
//! and free-move take visible effect or refuse typed, the flat-pack's
//! patterned instances included).

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::asm;
use pncad::document::Frame;
use pncad::geom_core::{Tol, Vec3};
use pncad::select::face_frame;
use viewer::display::AdmissionFault;
use viewer::matetool::{MateSeat, MateTool};
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

    // ── 3. RESOLVE: the tree shows the three instances, each its own
    // placement, every row ok.
    let rows = session.tree_rows();
    assert_eq!(
        rows.len(),
        4,
        "three instances, each its own placement, and the product's ⊔"
    );
    for row in &rows {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    assert_eq!(asm::instance_rows(&session).len(), 3);

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
    session.perform(SessionOp::SetBodyOn {
        node: bench.post_a,
        on: false,
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

    // ── 5. FREE-MOVE post_b: the G3 fit probe over a shown space (DI5).
    // It is given a space of its own first, out of the world; the
    // gesture is preview-stream + one committed value; the probed part
    // is drawn DISPLACED and MARKED — the honesty treatment, asserted as
    // a value in the scene rows. The probe draws the post where its pin
    // stood it, displaced.
    asm::unpin(&mut session, bench.post_b);
    let index = asm::index_of(&session);
    session.perform(SessionOp::BeginFreeMove { node: bench.post_b });
    session.perform(SessionOp::PreviewFreeMove {
        node: bench.post_b,
        frame: asm::where_b_stood(Frame::translation([0.03, 0.0, 0.0])),
    });
    let outcome = session.perform(SessionOp::CommitFreeMove { node: bench.post_b });
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
    let post_top = common::displayed_face_at(
        &session,
        &index,
        &asm::down_at(
            asm::POST_B_AT[0] + 0.03 + asm::POST_SECTION / 2.0,
            asm::POST_B_AT[1] + asm::POST_SECTION / 2.0,
        ),
    );
    assert_eq!(
        post_top.node, bench.post_b,
        "the probed post is picked where it is drawn"
    );
    tool.pick(session.doc(), post_top.clone());
    let shelf_bottom = common::displayed_face_at(&session, &index, &asm::under_shelf());
    assert_eq!(shelf_bottom.node, bench.shelf_i);
    tool.pick(session.doc(), shelf_bottom.clone());
    assert!(
        tool.rows()[0].side(pncad::document::MateSide::To).pick(MateSeat::One).is_some(),
        "both seats are held"
    );

    // ── 7. ONE COMMITTED MATE ACTION. The proposal reads each picked
    // face as a plane — post_b's top cap `on`, the shelf's underside
    // `to`, flipped — and the seat's values; the session commits, as one
    // action, post_b re-targeted into the shelf's space (the world) and
    // its bundle written whole: the mate and its three values. post_b's
    // own space is then empty, so that edit reports the free-move
    // superseded: discarded, not zeroed.
    let own = session
        .committed_doc()
        .node_space(bench.post_b)
        .expect("post_b is placed");
    assert!(session.display().free_move_of(own).is_some());
    let proposal = common::proposed(&session, &tool, asm::seat_choice())
        .expect("the seat proposes");
    let outcome = session.perform(proposal.op_into_target_space());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        outcome.committed.len(),
        2,
        "the re-target, and the bundle written whole with the mate and its values: {:?}",
        outcome.committed
    );
    let [superseded] = &outcome.withdrawn.superseded[..] else {
        panic!(
            "exactly one frame is superseded: {:?}",
            outcome.withdrawn.superseded
        )
    };
    assert_eq!(
        superseded.held,
        viewer::display::Held::Node(bench.post_b),
        "the mate's edit discards the probe"
    );
    assert!(
        matches!(
            &superseded.cause,
            AdmissionFault::NotShown { space, .. } if space.id() == own
        ),
        "and the outcome carries WHY it went, not only which went — the \
         fault's own PAYLOAD, which is what would go red if the prune paired \
         the right fault with the wrong space: {}",
        superseded.cause
    );
    assert!(session.display().free_move_of(own).is_none());
    session.pump();

    // ── 8. The PLACEMENT IS SOLVED: the two picked FACES coincide in
    // world space as the seat says — the planes rest: the faces' normals
    // oppose and the cap lies in the underside's plane — and post_b
    // stands where the values put it, unturned. The mate carries the
    // faces' NAMES, so the poses read off the landed evaluation — each
    // instance's body placed by the solve — are the one witness.
    session.pump();
    let (_, eval) = session.landed_pair().expect("landed");
    let pose_a = face_frame(eval, post_top.node, &post_top.name).expect("the cap has a pose");
    let pose_b =
        face_frame(eval, shelf_bottom.node, &shelf_bottom.name).expect("the underside has a pose");
    let close3 = |got: Vec3<f64>, want: Vec3<f64>, what: &str| {
        assert!(
            (got.x - want.x).abs() < 1e-9
                && (got.y - want.y).abs() < 1e-9
                && (got.z - want.z).abs() < 1e-9,
            "{what}: {got:?} vs {want:?}"
        );
    };
    let world_a = pose_a.origin;
    close3(
        pose_a.axis,
        pose_b.axis * -1.0,
        "the faces' normals meet opposed: the target is flipped",
    );
    assert!(
        (pose_a.origin - pose_b.origin).dot(pose_b.axis).abs() < 1e-9,
        "the cap lies in the underside's plane"
    );
    asm::assert_solved_at(
        &session,
        bench.post_b,
        &asm::seated_at(asm::SHELF_LENGTH / 2.0),
        "the values seat post_b under the shelf's middle",
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
    // …and the at-rest badge landed with the evaluation: the
    // seat is recorded at the door and certified — the verification
    // does not die at the commit.
    assert_eq!(
        session.at_rest(),
        Some(&viewer::session::AtRestBadge {
            loud: 0,
            quiet: 0,
            could_not_look: 0,
        }),
        "the mated assembly certifies at rest"
    );

    // ── 9. SAVE and REOPEN. The document round-trips WITH the mate.
    //
    // "Layer-3 state is gone" is scoped honestly here: the persistence
    // schema has NO field that could carry display state, so the
    // guarantee is structural unrepresentability — stronger than any
    // row, and NOT one, because a freshly opened session's display
    // state is empty by construction and the two asserts below cannot
    // go red. They stay as executable documentation of the claim; what
    // this stage actually GATES is the document half: the mate
    // round-trips, every row re-resolves, and the solved placement is
    // reproduced.
    assert!(
        !session.display().toggled().is_empty(),
        "the walk still holds layer-3 state at save time"
    );
    let outcome = session.perform(SessionOp::Save(bench.asm_path.clone()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let reopened = asm::open_bench(&bench, tol);
    assert!(
        reopened.display().toggled().is_empty(),
        "hide died with the session (documentation, not a gate — see above)"
    );
    assert!(
        crate::common::asm::frame_of(&reopened, bench.post_b).is_none(),
        "so did the probe (documentation, not a gate — see above)"
    );
    let rows = reopened.tree_rows();
    assert_eq!(
        rows.len(),
        4,
        "three instances, the mate a child of post_b's row, and the product's ⊔"
    );
    assert!(
        common::row_of(&rows, bench.post_b)
            .constraints
            .iter()
            .any(|child| child.said.ends_with(": mate"))
    );
    for row in &rows {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    // The solved placement survives the round trip bit-for-bit at the
    // assertion tolerance: the mate is the document, the probe never
    // was.
    let (_, eval2) = reopened.landed_pair().expect("landed");
    let re_world = face_frame(eval2, post_top.node, &post_top.name)
        .expect("the cap has a pose after reopen")
        .origin;
    assert!(
        (re_world.x - world_a.x).abs() < 1e-12
            && (re_world.y - world_a.y).abs() < 1e-12
            && (re_world.z - world_a.z).abs() < 1e-12,
        "the solved placement round-trips: {re_world:?} vs {world_a:?}"
    );
}
