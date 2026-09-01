//! **Assemblies in the viewer: the resolver, hide, and free-move**
//! (GUI-4 deliverables 1–3, headless).
//!
//! The fixture is a gallery-shaped workspace on disk (`common::asm`):
//! two part documents beside the assembly that pins them. The rows
//! here pin the three display-layer claims — the open path wires a
//! resolver under the directory rule; a hidden instance leaves the
//! picture and the pick index but never the document or the tree; a
//! free-move probe accepts only completely-unconstrained instances,
//! draws visibly distinct, and is DISCARDED when a mate lands.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use common::{asm};
use pncad::document::{
    Alignment, AxisSense, Frame, MateFrame, MatePrimitive, RecipeNodeId, product,
};
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use viewer::display::DisplayFault;
use viewer::scene::SceneMesh;
use viewer::session::{DocSession, Refusal, SessionOp};
use viewer::tree::RowStatus;

/// The mate the refusal rows author directly: post_a's top seated on
/// the shelf's underside, frames in each part's own coordinates.
fn seat_alignment() -> Alignment {
    Alignment {
        a: MateFrame {
            origin: [
                asm::POST_SECTION / 2.0,
                asm::POST_SECTION / 2.0,
                asm::POST_HEIGHT,
            ],
            axis: [0.0, 0.0, 1.0],
            reference: [1.0, 0.0, 0.0],
        },
        b: MateFrame {
            origin: [asm::SHELF_LENGTH / 2.0, asm::SHELF_DEPTH / 2.0, 0.0],
            axis: [0.0, 0.0, -1.0],
            reference: [1.0, 0.0, 0.0],
        },
        primitive: MatePrimitive::FrameCoincidence,
        sense: AxisSense::Opposed,
        clocking: None,
    }
}

/// Author the seat mate between `a_instance` and the shelf through
/// the session's one committed-edit door.
fn add_seat_mate(session: &mut DocSession, bench: &asm::Bench, a_instance: RecipeNodeId) {
    let outcome = session.perform(SessionOp::AddMate {
        a: asm::in_part(a_instance, &bench.post_top),
        b: asm::in_part(bench.shelf_i, &bench.shelf_bottom),
        class: ContactClass::Rest,
        alignment: seat_alignment(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1, "exactly one committed edit");
    session.pump();
}

// --- the resolver (deliverable 1) ---------------------------------

#[test]
fn the_open_path_wires_a_resolver_and_the_assembly_evaluates() {
    let tol = Tol::witness();
    let bench = asm::bench("resolves", tol);
    let session = asm::open_bench(&bench, tol);
    // The directory rule's positive half: the resolver is the opened
    // file's own directory.
    assert_eq!(
        session.resolve_dir().expect("a resolver is wired"),
        bench.dir
    );
    // The assembly actually evaluates: every row ok, and the product
    // gathers the three placed solids.
    let rows = session.tree_rows();
    assert_eq!(rows.len(), 3);
    for row in &rows {
        assert_eq!(row.kind, "InstantiatePart");
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    let (doc, eval) = session.landed_pair().expect("landed");
    let body = product(doc, eval, tol).expect("the product gathers");
    assert_eq!(body.shells().count(), 3, "two posts and a shelf");
}

#[test]
fn a_session_with_no_backing_file_resolves_nothing_and_refuses_typed() {
    let tol = Tol::witness();
    let bench = asm::bench("noresolver", tol);
    // The same document VALUE, held in memory: no file, no resolver —
    // the typed no-resolver refusal renders as the tree's badges.
    let history = viewer::docio::open(&bench.asm_path, tol).expect("the file opens");
    let mut session = DocSession::inline(history.doc().clone(), tol);
    session.pump();
    for row in session.tree_rows() {
        match &row.status {
            RowStatus::Failed { message } => assert!(
                message.contains("no part resolver"),
                "the refusal names the missing seam: {message}"
            ),
            other => panic!("an unresolvable instantiate row must fail typed, got {other:?}"),
        }
    }
}

#[test]
fn a_missing_part_document_refuses_typed_and_badges_the_row() {
    let tol = Tol::witness();
    let bench = asm::bench("missing", tol);
    // Remove the post document from the store: both post instances
    // must refuse typed; the shelf still evaluates.
    std::fs::remove_file(bench.dir.join(format!("{}.pncad", bench.post.id)))
        .expect("the post file removes");
    let session = asm::open_bench(&bench, tol);
    let rows = session.tree_rows();
    let status_of = |id: RecipeNodeId| {
        rows.iter()
            .find(|row| row.id == id)
            .expect("the row exists")
            .status
            .clone()
    };
    for post in [bench.post_a, bench.post_b] {
        match status_of(post) {
            RowStatus::Failed { message } => assert!(
                message.contains("no document with id"),
                "the store's own refusal reaches the badge: {message}"
            ),
            other => panic!("a missing part must fail its instance row, got {other:?}"),
        }
    }
    assert_eq!(status_of(bench.shelf_i), RowStatus::Ok);
}

#[test]
fn the_directory_rule_a_document_never_resolves_against_another_directory() {
    let tol = Tol::witness();
    let bench = asm::bench("dirrule", tol);
    // Copy ONLY the assembly file into a sibling directory. The parts
    // still exist in the original store; the rule says they are not
    // consulted — resolution is against the opened file's directory
    // and nothing else.
    let elsewhere = bench.dir.join("elsewhere");
    std::fs::create_dir_all(&elsewhere).expect("the sibling directory creates");
    let moved = elsewhere.join("bench.pncad");
    std::fs::copy(&bench.asm_path, &moved).expect("the assembly copies");
    let mut session =
        DocSession::inline(pncad::document::Doc::empty_derived("gui4-boot", tol), tol);
    let outcome = session.perform(SessionOp::Open(moved));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    for row in session.tree_rows() {
        match &row.status {
            RowStatus::Failed { message } => assert!(
                message.contains("no document with id"),
                "unresolvable — the parts are not beside THIS file: {message}"
            ),
            other => panic!("expected a typed resolution failure, got {other:?}"),
        }
    }
}

#[test]
fn a_directory_that_will_not_scan_refuses_each_resolution_typed() {
    let tol = Tol::witness();
    let bench = asm::bench("corrupt", tol);
    std::fs::write(bench.dir.join("junk.pncad"), "not a document").expect("the junk file writes");
    // The OPEN succeeds — a corrupt sibling must not hold an
    // unrelated document hostage; the store is only required to be
    // healthy when something resolves through it.
    let session = asm::open_bench(&bench, tol);
    // …and every resolution then refuses typed, carrying the store's
    // own refusal about the offending file.
    for row in session.tree_rows() {
        match &row.status {
            RowStatus::Failed { message } => assert!(
                message.contains("junk.pncad"),
                "the scan refusal names the offending file: {message}"
            ),
            other => panic!("expected a typed store refusal, got {other:?}"),
        }
    }
}

// --- hide (deliverable 2) -----------------------------------------

#[test]
fn hiding_drops_scene_and_picks_but_keeps_tree_and_document() {
    let tol = Tol::witness();
    let bench = asm::bench("hide", tol);
    let mut session = asm::open_bench(&bench, tol);
    let index = asm::index_of(&session);
    // An owned handle on the landed run, so the picks below survive
    // the session mutating between them.
    let eval = std::sync::Arc::clone(session.evaluation_arc().expect("landed"));
    let eval = &*eval;

    // Before: post_b draws and picks at its authored spot.
    let at_post_b = asm::down_at(
        asm::POST_B_AT[0] + asm::POST_SECTION / 2.0,
        asm::POST_B_AT[1] + asm::POST_SECTION / 2.0,
    );
    let full = index.scene_for(&session.display_view()).expect("a scene");
    let hit = index
        .pick_for(eval, &at_post_b, &session.display_view())
        .expect("the pick answers")
        .expect("post_b is under the ray");
    assert_eq!(hit.node, bench.post_b);

    // Hide it.
    let outcome = session.perform(SessionOp::SetInstanceHidden {
        instance: bench.post_b,
        hidden: true,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let view = session.display_view();
    let hidden_scene = index.scene_for(&view).expect("a scene");
    assert!(
        hidden_scene.stats().triangles < full.stats().triangles,
        "the drawn scene drops the hidden instance"
    );
    assert!(
        index
            .pick_for(eval, &at_post_b, &view)
            .expect("the pick answers")
            .is_none(),
        "a hidden instance is out of the pick index"
    );
    // The ids of the still-drawn instances are unchanged: hide edits
    // what is emitted, never what an id means.
    assert_eq!(
        index.ids_in(bench.post_a, 0),
        asm::index_of(&session).ids_in(bench.post_a, 0)
    );
    // The tree KEEPS the row, and the document is untouched.
    assert!(
        session.tree_rows().iter().any(|row| row.id == bench.post_b),
        "the tree keeps the hidden instance"
    );
    assert!(session.doc().node(bench.post_b).is_some());
    assert!(
        session.perform(SessionOp::Undo).refusal.is_some(),
        "hide entered no history: there is nothing to undo"
    );

    // Show it again: the picture is restored.
    session.perform(SessionOp::SetInstanceHidden {
        instance: bench.post_b,
        hidden: false,
    });
    let restored = index.scene_for(&session.display_view()).expect("a scene");
    assert_eq!(restored.stats().triangles, full.stats().triangles);
}

#[test]
fn fused_geometry_refuses_both_display_ops_typed() {
    // Two instances consumed by one boolean: neither can be hidden or
    // probed separately — the drawn root fuses their material — and
    // both ops say so typed instead of accepting and drawing nothing
    // different (the propagation rule's refusing half).
    let tol = Tol::witness();
    let bench = asm::bench("fused", tol);
    let mut ws = pncad::workspace::Workspace::open(&bench.dir).expect("the store opens");
    let mut doc =
        pncad::document::ProfileDoc::empty(pncad::document::DocumentId::derive("gui4-fused"), tol);
    let insert = |doc: &mut pncad::document::ProfileDoc,
                  node: pncad::document::Node<pncad::document::ProfileProgram>| {
        let applied =
            pncad::document::apply(doc, &pncad::document::DocEdit::InsertNode { node }, tol)
                .expect("the insert applies");
        *doc = applied.doc;
        applied.record.minted.expect("an id")
    };
    let a = insert(
        &mut doc,
        pncad::document::Node::instantiate_part(bench.post),
    );
    let b = insert(
        &mut doc,
        pncad::document::Node::instantiate_part(bench.post),
    );
    let weld = insert(
        &mut doc,
        pncad::document::Node::Boolean {
            op: pncad::document::BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    );
    let path = ws.create(&doc, tol).expect("the fused assembly stores");
    let mut session = DocSession::inline(
        pncad::document::Doc::empty_derived("gui4-fused-boot", tol),
        tol,
    );
    assert!(session.perform(SessionOp::Open(path)).refusal.is_none());
    session.pump();
    for (label, op) in [
        (
            "hide a",
            SessionOp::SetInstanceHidden {
                instance: a,
                hidden: true,
            },
        ),
        (
            "hide b",
            SessionOp::SetInstanceHidden {
                instance: b,
                hidden: true,
            },
        ),
        ("probe a", SessionOp::BeginFreeMove { instance: a }),
        ("probe b", SessionOp::BeginFreeMove { instance: b }),
    ] {
        match session.perform(op).refusal {
            Some(Refusal::Display(DisplayFault::FusedGeometry {
                instance,
                root,
                others,
            })) => {
                assert!(instance == a || instance == b);
                assert_eq!(root, weld, "the refusal names the fusing root");
                assert_eq!(others.len(), 1, "…and the other instance");
            }
            other => panic!("{label}: expected FusedGeometry, got {other:?}"),
        }
    }
}

#[test]
fn the_at_rest_badge_lands_with_the_evaluation() {
    // The A5 verdict lives past the commit: the mate-less assembly
    // certifies with nothing minted; a Rest mate certifies WITH its
    // declaration; a Tangent mate turns the badge into the gate's own
    // refusal while its tree row carries the class's standing note.
    let tol = Tol::witness();
    let bench = asm::bench("atrest", tol);
    let mut session = asm::open_bench(&bench, tol);
    assert_eq!(
        session.at_rest(),
        Some(&viewer::session::AtRestBadge::Certified { minted: 0 }),
        "disjoint instances certify outright (A5's disjoint half)"
    );
    session.perform(SessionOp::AddMate {
        a: asm::in_part(bench.post_b, &bench.post_top),
        b: asm::in_part(bench.shelf_i, &bench.shelf_bottom),
        class: ContactClass::Tangent,
        alignment: seat_alignment(),
    });
    session.pump();
    match session.at_rest() {
        Some(viewer::session::AtRestBadge::Refused { message }) => assert!(
            message.contains("no at-rest kernel record"),
            "the badge is the gate's own refusal: {message}"
        ),
        other => panic!("a Tangent mate must turn the badge red, got {other:?}"),
    }
    // …and the mate's own tree row says why, independent of any run.
    let note = session
        .tree_rows()
        .into_iter()
        .find(|row| row.kind == "Mate")
        .expect("the mate row exists")
        .note
        .expect("a Tangent mate carries its standing note");
    assert!(note.contains("Tangent"), "{note}");
}

#[test]
fn hide_refuses_a_non_instance_typed() {
    let tol = Tol::witness();
    let bench = asm::bench("hidewrong", tol);
    let mut session = asm::open_bench(&bench, tol);
    let outcome = session.perform(SessionOp::SetInstanceHidden {
        instance: RecipeNodeId(9_999),
        hidden: true,
    });
    assert!(
        matches!(
            outcome.refusal,
            Some(Refusal::Display(DisplayFault::NotAnInstance { .. }))
        ),
        "{:?}",
        outcome.refusal
    );
}

#[test]
fn hide_is_never_persisted() {
    let tol = Tol::witness();
    let bench = asm::bench("hidesave", tol);
    let mut session = asm::open_bench(&bench, tol);
    session.perform(SessionOp::SetInstanceHidden {
        instance: bench.post_b,
        hidden: true,
    });
    // Save the document OVER its own file and reopen it: the hidden
    // set is display state of the closed session, not of the file.
    let outcome = session.perform(SessionOp::Save(bench.asm_path.clone()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let reopened = asm::open_bench(&bench, tol);
    assert!(reopened.display().hidden().is_empty());
}

// --- free-move (deliverable 3) ------------------------------------

#[test]
fn free_move_accepts_only_completely_unconstrained_instances() {
    let tol = Tol::witness();
    let bench = asm::bench("fmeligible", tol);
    let mut session = asm::open_bench(&bench, tol);
    // Constrain post_a by mating it to the shelf.
    add_seat_mate(&mut session, &bench, bench.post_a);
    // Both mate participants refuse, naming the mate.
    for constrained in [bench.post_a, bench.shelf_i] {
        let outcome = session.perform(SessionOp::BeginFreeMove {
            instance: constrained,
        });
        match outcome.refusal {
            Some(Refusal::Display(DisplayFault::MateConstrained { instance, mates })) => {
                assert_eq!(instance, constrained);
                assert_eq!(mates.len(), 1, "the refusal lists the constraining mate");
            }
            other => panic!("a mate-constrained instance must refuse typed, got {other:?}"),
        }
    }
    // The uninvolved post is still eligible.
    let outcome = session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.perform(SessionOp::CancelFreeMove);
    // A non-instance refuses typed.
    let outcome = session.perform(SessionOp::BeginFreeMove {
        instance: RecipeNodeId(9_999),
    });
    assert!(
        matches!(
            outcome.refusal,
            Some(Refusal::Display(DisplayFault::NotAnInstance { .. }))
        ),
        "{:?}",
        outcome.refusal
    );
}

#[test]
fn the_probe_gesture_previews_commits_and_draws_visibly_distinct() {
    let tol = Tol::witness();
    let bench = asm::bench("fmgesture", tol);
    let mut session = asm::open_bench(&bench, tol);
    let index = asm::index_of(&session);
    let baseline = index.scene_for(&session.display_view()).expect("a scene");
    assert_eq!(baseline.stats().probe_parts, 0);
    assert!(baseline.flags().iter().all(|&f| f == 0));

    // Begin, preview twice (previews replace), commit once.
    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    for dx in [0.02, 0.05] {
        let outcome = session.perform(SessionOp::PreviewFreeMove {
            frame: Frame::translation([dx, 0.0, 0.0]),
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    }
    let outcome = session.perform(SessionOp::CommitFreeMove);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let committed = session
        .display()
        .free_move_of(bench.post_b)
        .expect("the probe committed");
    assert_eq!(
        committed.translation,
        [0.05, 0.0, 0.0],
        "the commit lands the LAST previewed frame"
    );

    // The distinctness value, asserted in the scene rows: the probed
    // part is marked and displaced; everything else is neither.
    let probed = index.scene_for(&session.display_view()).expect("a scene");
    assert_eq!(probed.stats().probe_parts, 1);
    assert!(
        probed.flags().contains(&SceneMesh::FLAG_PROBE),
        "the probe's corners carry the flag"
    );
    let marked = probed
        .flags()
        .iter()
        .filter(|&&f| f == SceneMesh::FLAG_PROBE)
        .count();
    let probe_corners: usize = index
        .parts()
        .iter()
        .filter(|part| part.node() == bench.post_b)
        .flat_map(|part| part.mesh().patches.iter())
        .map(|patch| patch.triangles.len() * 3)
        .sum();
    assert_eq!(
        marked, probe_corners,
        "exactly the probed instance's corners are marked"
    );
    // The probed box's far face is drawn at its displaced position:
    // authored x-extent end + the probe's 0.05, past the shelf's 0.10
    // that bounded the baseline.
    let want_max_x = asm::POST_B_AT[0] + asm::POST_SECTION + 0.05;
    assert!(
        (probed.bounds().max_x - want_max_x).abs() < 1e-6,
        "the probed part is DRAWN displaced by the probe: {} vs {want_max_x} (baseline {})",
        probed.bounds().max_x,
        baseline.bounds().max_x
    );

    // The pick follows the picture: the probed instance answers at
    // its drawn spot and no longer at its authored one.
    let (_, eval) = session.landed_pair().expect("landed");
    let view = session.display_view();
    let centre = [
        asm::POST_B_AT[0] + asm::POST_SECTION / 2.0,
        asm::POST_B_AT[1] + asm::POST_SECTION / 2.0,
    ];
    let hit = index
        .pick_for(eval, &asm::down_at(centre[0] + 0.05, centre[1]), &view)
        .expect("the pick answers")
        .expect("the probed instance is under the moved ray");
    assert_eq!(hit.node, bench.post_b);
    assert!(
        index
            .pick_for(eval, &asm::down_at(centre[0], centre[1]), &view)
            .expect("the pick answers")
            .is_none(),
        "nothing is picked where the probe moved away from"
    );

    // A cancel restores the committed picture.
    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    session.perform(SessionOp::PreviewFreeMove {
        frame: Frame::translation([0.0, 0.0, 0.3]),
    });
    session.perform(SessionOp::CancelFreeMove);
    assert_eq!(
        session
            .display()
            .free_move_of(bench.post_b)
            .expect("still committed")
            .translation,
        [0.05, 0.0, 0.0]
    );

    // Never persisted: save over the file, reopen, probe gone.
    session.perform(SessionOp::Save(bench.asm_path.clone()));
    let reopened = asm::open_bench(&bench, tol);
    assert!(reopened.display().free_move_of(bench.post_b).is_none());
}

#[test]
fn a_non_rigid_preview_refuses_typed() {
    let tol = Tol::witness();
    let bench = asm::bench("fmrigid", tol);
    let mut session = asm::open_bench(&bench, tol);
    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    for bad in [
        // A scale: lengths not preserved.
        Frame {
            columns: [[2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            translation: [0.0; 3],
        },
        // A mirror: improper.
        Frame {
            columns: [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            translation: [0.0; 3],
        },
        // Non-finite.
        Frame::translation([f64::NAN, 0.0, 0.0]),
    ] {
        let outcome = session.perform(SessionOp::PreviewFreeMove { frame: bad });
        assert!(
            matches!(
                outcome.refusal,
                Some(Refusal::Display(DisplayFault::NonRigidFrame { .. }))
            ),
            "{:?}",
            outcome.refusal
        );
    }
    // A rotation IS admitted (the probe is any rigid motion).
    let outcome = session.perform(SessionOp::PreviewFreeMove {
        frame: Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.5, [0.01, 0.0, 0.0]),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
}

#[test]
fn a_landing_mate_discards_the_probe_value() {
    let tol = Tol::witness();
    let bench = asm::bench("fmsupersede", tol);
    let mut session = asm::open_bench(&bench, tol);
    let index = asm::index_of(&session);
    // Probe post_b somewhere.
    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    session.perform(SessionOp::PreviewFreeMove {
        frame: Frame::translation([0.04, 0.0, 0.0]),
    });
    session.perform(SessionOp::CommitFreeMove);
    assert!(session.display().free_move_of(bench.post_b).is_some());

    // The mate lands on post_b: ONE committed edit, and the probe is
    // superseded IN THE SAME OUTCOME.
    let outcome = session.perform(SessionOp::AddMate {
        a: asm::in_part(bench.post_b, &bench.post_top),
        b: asm::in_part(bench.shelf_i, &bench.shelf_bottom),
        class: ContactClass::Rest,
        alignment: seat_alignment(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    assert_eq!(
        outcome.superseded,
        vec![bench.post_b],
        "the supersession is reported, not inferred"
    );
    // DISCARDED, not zeroed: the value is gone, and the map holds no
    // identity entry standing in for it.
    assert!(session.display().free_move_of(bench.post_b).is_none());
    session.pump();
    // The instance is drawn at its SOLVED placement, undistinguished.
    let index_after = asm::index_of(&session);
    let scene = index_after
        .scene_for(&session.display_view())
        .expect("a scene");
    assert_eq!(scene.stats().probe_parts, 0);
    assert!(scene.flags().iter().all(|&f| f == 0));
    // And the mate really moved it: the pick at the old authored spot
    // no longer answers post_b there.
    let _ = index; // (the pre-mate index is stale by generation)
    let (_, eval) = session.landed_pair().expect("landed");
    let hit = index_after
        .pick_for(
            eval,
            &asm::down_at(
                asm::POST_B_AT[0] + asm::POST_SECTION / 2.0,
                asm::POST_B_AT[1] + asm::POST_SECTION / 2.0,
            ),
            &session.display_view(),
        )
        .expect("the pick answers");
    assert!(
        hit.is_none_or(|h| h.node != bench.post_b),
        "the solved placement superseded the authored spot"
    );
}
