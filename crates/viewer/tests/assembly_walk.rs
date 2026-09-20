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
//!
//! # Why this walk runs on a FIXTURE, and where the real gallery is
//! exercised
//!
//! The walk needs, in one document: unconstrained instances (to hide
//! and probe), two matable faces, and no pre-existing mate. **No
//! single real gallery document has that shape** — the tour's stand
//! has all three instances mate-constrained (the free-move stage is
//! unreachable there), and its flat-pack has no second part to mate
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
use pncad::document::{AxisSense, ClassAdmission, Frame, MatePrimitive};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::select::{ContactClass, Ray, face_frame};
use viewer::display::AdmissionFault;
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
        instance: bench.post_b,
        frame: Frame::translation([0.03, 0.0, 0.0]),
    });
    let outcome = session.perform(SessionOp::CommitFreeMove {
        instance: bench.post_b,
    });
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
    let post_top = viewer::session::FaceSelection {
        name: post_top.name.clone(),
        node: post_top.node,
        body: post_top.body,
    };
    tool.pick(post_top.clone());
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
    tool.pick(shelf_bottom.clone());
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
    let [superseded] = &outcome.withdrawn.superseded[..] else {
        panic!(
            "exactly one placement is superseded: {:?}",
            outcome.withdrawn.superseded
        )
    };
    assert_eq!(
        superseded.instance, bench.post_b,
        "the mate's landing discards the probe, in the same outcome"
    );
    assert!(
        matches!(
            &superseded.cause,
            AdmissionFault::MateConstrained { instance, mates }
                if *instance == bench.post_b && !mates.is_empty()
        ),
        "and the outcome carries WHY it went, not only which went — the \
         fault's own PAYLOAD, which is what would go red if the prune paired \
         the right fault with the wrong instance: {}",
        superseded.cause
    );
    assert!(session.display().free_move_of(bench.post_b).is_none());

    // ── 9. The PLACEMENT IS SOLVED: the two picked FACES coincide in
    // world space — the FULL frame, not origins alone: origins meet,
    // chart axes oppose (the chosen sense), and the carriers' own
    // roll references agree under the solve's opposed flip. The
    // frames the mate carries are the faces' NAMES, so the poses read
    // off the landed evaluation — each instance's body placed by the
    // solve — are the one witness: what the alignment declares of
    // the faces it names, the placed faces show.
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
    let world_b = pose_b.origin;
    close3(
        world_a - Point3::new(0.0, 0.0, 0.0),
        world_b - Point3::new(0.0, 0.0, 0.0),
        "the mated faces' ORIGINS coincide",
    );
    close3(
        pose_a.axis,
        pose_b.axis * -1.0,
        "the chart AXES meet opposed, as the chosen sense declares",
    );
    close3(
        pose_a.u_ref.expect("a cap fixes a reference"),
        pose_b.u_ref.expect("a cap fixes a reference") * -1.0,
        "the roll REFERENCES meet under the solve's opposed flip at zero \
         clocking (the flip reverses reference and axis together, keeping \
         the pair's handedness proper)",
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
        assert!(
            row.note.is_none(),
            "a Rest mate carries no standing caveat: {row:?}"
        );
    }
    // …and the A5 at-rest verdict landed with the evaluation: the
    // seated Rest declaration is minted and certified — the
    // verification does not die at the commit.
    assert_eq!(
        session.at_rest(),
        Some(&viewer::session::AtRestBadge::Certified { minted: 1 }),
        "the mated assembly certifies at rest with its one declaration"
    );

    // ── 10. SAVE and REOPEN. The document round-trips WITH the mate.
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
        !session.display().hidden().is_empty(),
        "the walk still holds layer-3 state at save time"
    );
    let outcome = session.perform(SessionOp::Save(bench.asm_path.clone()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let reopened = asm::open_bench(&bench, tol);
    assert!(
        reopened.display().hidden().is_empty(),
        "hide died with the session (documentation, not a gate — see above)"
    );
    assert!(
        reopened.display().free_move_of(bench.post_b).is_none(),
        "so did the probe (documentation, not a gate — see above)"
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
