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

use crate::common;

use common::asm;
use pncad::document::{Frame, RecipeNodeId, product};
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use viewer::display::{self, AdmissionFault, DisplayFault};
use viewer::frame;
use viewer::scene::SceneMesh;
use viewer::session::{DocSession, Refusal, SessionOp};
use viewer::tree::RowStatus;

/// Every `Node::Mate` the session's document holds, document order —
/// read off the recipe rather than off `display::mates_naming`, so a
/// row asserting a fault's `mates` payload is not checking that
/// function against itself.
fn mate_nodes(session: &DocSession) -> Vec<RecipeNodeId> {
    let doc = session.doc();
    doc.order()
        .iter()
        .copied()
        .filter(|&id| matches!(doc.node(id), Some(pncad::document::Node::Mate { .. })))
        .collect()
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
    let status_of = |id: RecipeNodeId| common::status_of(&rows, id);
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
    let at_post_b = asm::over_post_b();
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

/// Two instances of one part consumed by a single boolean: the drawn
/// root fuses their material, so no display operation can address
/// either separately. The session, the two instances and the fusing
/// root — one fixture, because two hand-built copies of it is how two
/// rows come to disagree about what "fused" is.
fn fused_pair(tag: &str, tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let bench = asm::bench(tag, tol);
    let mut ws = pncad::workspace::Workspace::open(&bench.dir).expect("the store opens");
    let mut doc = pncad::document::ProfileDoc::empty(
        pncad::document::DocumentId::derive(&format!("gui4-{tag}")),
        tol,
    );
    let a = common::insert_into(
        &mut doc,
        pncad::document::Node::instantiate_part(bench.post),
        tol,
    );
    let b = common::insert_into(
        &mut doc,
        pncad::document::Node::instantiate_part(bench.post),
        tol,
    );
    let weld = common::insert_into(
        &mut doc,
        pncad::document::Node::Boolean {
            op: pncad::document::BooleanOp::Union,
            a,
            b,
            declare: None,
        },
        tol,
    );
    let path = ws.create(&doc, tol).expect("the fused assembly stores");
    let mut session = DocSession::inline(
        pncad::document::Doc::empty_derived(&format!("gui4-{tag}-boot"), tol),
        tol,
    );
    assert!(session.perform(SessionOp::Open(path)).refusal.is_none());
    session.pump();
    (session, a, b, weld)
}

#[test]
fn fused_geometry_refuses_both_display_ops_typed() {
    // Neither instance can be hidden or probed separately, and both
    // ops say so typed instead of accepting and drawing nothing
    // different (the propagation rule's refusing half).
    let tol = Tol::witness();
    let (mut session, a, b, weld) = fused_pair("fused", tol);
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
            Some(Refusal::Display(DisplayFault::Admission(AdmissionFault::FusedGeometry {
                instance,
                root,
                others,
            }))) => {
                assert!(instance == a || instance == b);
                assert_eq!(root, weld, "the refusal names the fusing root");
                assert_eq!(others.len(), 1, "…and the other instance");
            }
            other => panic!("{label}: expected FusedGeometry, got {other:?}"),
        }
    }
}

/// **The per-instance section is drawn for a fused instance and its
/// display controls are not** — the two gates are two different tests,
/// and the properties pane reads both.
///
/// `display::instance_check` is the KIND test: it decides whether
/// there is a section at all, and a fused instance is a live instance
/// of a part, so there is one. What the hide toggle inside it pushes
/// runs the FULL admission test, which refuses it. A toggle gated on
/// the section's test alone is therefore drawn usable over a refusal
/// the op will give — the state this row pins, together with the
/// sentence the reader gets for it.
#[test]
fn a_fused_instances_section_is_drawn_and_its_display_controls_are_refused() {
    let tol = Tol::witness();
    let (mut session, a, b, weld) = fused_pair("fusedgate", tol);

    assert!(
        display::instance_check(session.doc(), a).is_ok(),
        "a fused instance is still an instance, so the section has a subject"
    );
    let fault = display::display_check(session.doc(), a)
        .expect_err("…and no display operation can address it separately");
    assert!(
        matches!(&fault, AdmissionFault::FusedGeometry { instance, root, .. }
            if *instance == a && *root == weld),
        "{fault:?}"
    );

    // The disabled toggle's words and the refused click's are ONE
    // sentence: the control shows this fault, and the op answers it.
    let refusal = session
        .perform(SessionOp::SetInstanceHidden {
            instance: a,
            hidden: true,
        })
        .refusal
        .expect("the op refuses a fused instance");
    assert_eq!(
        fault.to_string(),
        refusal.to_string(),
        "the pre-click sentence is the post-click one"
    );
    // The mapping itself, planted: what a reader is told under the
    // disabled toggle. The coupling above survives any rewording of
    // the fault; this line does not.
    assert_eq!(
        fault.to_string(),
        format!(
            "instance {}'s geometry is fused into node {} together with instance(s) {} — \
             a display operation cannot address it separately",
            a.0, weld.0, b.0
        )
    );

    // And the free-move probe below the toggle answers the SAME fault
    // — `free_move_check` runs the display test first — which is why
    // the section says it once rather than under the probe's heading.
    assert_eq!(
        display::free_move_check(session.doc(), a)
            .expect_err("the probe refuses it too")
            .to_string(),
        fault.to_string()
    );
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
    common::commit_mate(
        &mut session,
        asm::seat_op(
            &bench,
            bench.post_b,
            ContactClass::Tangent,
            asm::middle_seat_alignment(),
        ),
    );
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

/// **An id the document does not hold**, which is not the same
/// refusal as a node of the wrong kind — the row was named for the
/// wrong one of the two until they were spelled apart.
#[test]
fn hide_refuses_an_id_the_document_does_not_hold() {
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
            Some(Refusal::Display(DisplayFault::Admission(
                AdmissionFault::NoSuchNode { .. }
            )))
        ),
        "an id the document does not hold is NOT the wrong-kind refusal — \
         the two are spelled apart because a user holding display state on \
         the id reads a different sentence for each: {:?}",
        outcome.refusal
    );
}

/// **The admission test's kind half answers the two states apart**,
/// which is the whole of its signature: an absent id and a node of
/// another kind are different news, and a door that answered `bool`
/// could not say which it had met. Asserted at `instance_check`
/// itself rather than only through the ops above, because the door is
/// `pub` and the next caller is the one that will need them apart.
#[test]
fn instance_check_tells_an_absent_node_from_a_wrong_kind() {
    let tol = Tol::witness();
    let bench = asm::bench("instcheck", tol);
    let mut session = asm::open_bench(&bench, tol);
    // One node of another kind, authored through the ordinary door so
    // the wrong-kind arm is driven by a node a user can really select.
    let mate = common::commit_mate(
        &mut session,
        asm::seat_op(
            &bench,
            bench.post_a,
            ContactClass::Tangent,
            asm::middle_seat_alignment(),
        ),
    );
    let doc = session.doc();

    assert_eq!(
        display::instance_check(doc, bench.post_b),
        Ok(()),
        "a live InstantiatePart is admitted"
    );
    assert_eq!(
        display::instance_check(doc, mate),
        Err(AdmissionFault::NotAnInstance { node: mate }),
        "a node that IS in the document and is not an instance is the \
         wrong-kind refusal, naming itself"
    );
    let absent = RecipeNodeId(9_999);
    assert_eq!(
        display::instance_check(doc, absent),
        Err(AdmissionFault::NoSuchNode { node: absent }),
        "an id the document does not hold is the ABSENT refusal, not \
         the wrong-kind one — the two are the sentences a person reads"
    );

    // **The point of the split, asserted as the thing a person reads.**
    // Two arms of one enum prove nothing on their own; what the door
    // exists to buy is that the two states reach an operation's reader
    // as DIFFERENT sentences, so the renderings are pinned here and not
    // only the variants.
    let absent_says = display::instance_check(doc, absent)
        .expect_err("absent refuses")
        .to_string();
    let wrong_kind_says = display::instance_check(doc, mate)
        .expect_err("a mate is not an instance")
        .to_string();
    assert_eq!(
        absent_says, "node 9999 is not in the document",
        "the absent id's sentence says the id denotes nothing"
    );
    assert_eq!(
        wrong_kind_says,
        format!("node {} is not a part instance", mate.0),
        "the wrong-kind sentence says something IS there and is the \
         wrong thing"
    );
    assert_ne!(
        absent_says, wrong_kind_says,
        "a door that collapsed these two would render one sentence for \
         both, which is the whole defect this signature closes"
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
    let mate = common::commit_mate(
        &mut session,
        asm::seat_op(
            &bench,
            bench.post_a,
            ContactClass::Rest,
            asm::middle_seat_alignment(),
        ),
    );
    // Both mate participants refuse, naming the mate.
    for constrained in [bench.post_a, bench.shelf_i] {
        let outcome = session.perform(SessionOp::BeginFreeMove {
            instance: constrained,
        });
        match outcome.refusal {
            Some(Refusal::Display(DisplayFault::Admission(AdmissionFault::MateConstrained {
                instance,
                mates,
            }))) => {
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
    // A node that EXISTS and is not an instance — the mate authored
    // above — refuses for being the wrong KIND. That is the arm the
    // block below was labelled for and never drove.
    let outcome = session.perform(SessionOp::BeginFreeMove { instance: mate });
    assert!(
        matches!(
            outcome.refusal,
            Some(Refusal::Display(DisplayFault::Admission(
                AdmissionFault::NotAnInstance { .. }
            )))
        ),
        "{:?}",
        outcome.refusal
    );
    // And an id the document does not hold refuses for being ABSENT.
    let outcome = session.perform(SessionOp::BeginFreeMove {
        instance: RecipeNodeId(9_999),
    });
    assert!(
        matches!(
            outcome.refusal,
            Some(Refusal::Display(DisplayFault::Admission(
                AdmissionFault::NoSuchNode { .. }
            )))
        ),
        "an id the document does not hold is NOT the wrong-kind refusal — \
         the two are spelled apart because a user holding display state on \
         the id reads a different sentence for each: {:?}",
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
            instance: bench.post_b,
            frame: Frame::translation([dx, 0.0, 0.0]),
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    }
    let outcome = session.perform(SessionOp::CommitFreeMove {
        instance: bench.post_b,
    });
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
            .pick_for(eval, &asm::over_post_b(), &view)
            .expect("the pick answers")
            .is_none(),
        "nothing is picked where the probe moved away from"
    );

    // A cancel restores the committed picture.
    session.perform(SessionOp::BeginFreeMove {
        instance: bench.post_b,
    });
    session.perform(SessionOp::PreviewFreeMove {
        instance: bench.post_b,
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
        let outcome = session.perform(SessionOp::PreviewFreeMove {
            instance: bench.post_b,
            frame: bad,
        });
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
        instance: bench.post_b,
        frame: Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.5, [0.01, 0.0, 0.0], common::band())
            .expect("a literal axis has a definite direction"),
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
        instance: bench.post_b,
        frame: Frame::translation([0.04, 0.0, 0.0]),
    });
    session.perform(SessionOp::CommitFreeMove {
        instance: bench.post_b,
    });
    assert!(session.display().free_move_of(bench.post_b).is_some());

    // The mate lands on post_b: ONE committed edit, and the probe is
    // superseded IN THE SAME OUTCOME.
    let outcome = session.perform(asm::seat_op(
        &bench,
        bench.post_b,
        ContactClass::Rest,
        asm::middle_seat_alignment(),
    ));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    let [superseded] = &outcome.withdrawn.superseded[..] else {
        panic!(
            "exactly one placement is superseded: {:?}",
            outcome.withdrawn.superseded
        )
    };
    assert_eq!(
        superseded.instance, bench.post_b,
        "the supersession is reported, not inferred"
    );
    // The PAYLOAD, not the variant: the variant is what the op this row
    // just performed already implies, and what would go red if `prune`
    // paired the right fault with the wrong instance is this.
    let AdmissionFault::MateConstrained { instance, mates } = &superseded.cause else {
        panic!(
            "a mate landing supersedes with its own fault: {}",
            superseded.cause
        )
    };
    assert_eq!(*instance, bench.post_b, "the fault names the same instance");
    assert_eq!(
        mates,
        &mate_nodes(&session),
        "and names the mate that landed, read off the recipe"
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
        .pick_for(eval, &asm::over_post_b(), &session.display_view())
        .expect("the pick answers");
    assert!(
        hit.is_none_or(|h| h.node != bench.post_b),
        "the solved placement superseded the authored spot"
    );
}

/// **A hide the document stops admitting is DROPPED, and the outcome
/// says so** — the second half of what `DisplayState::prune` withdraws,
/// and the half that used to be undone in silence.
///
/// Both arms of `display_check` are here because they are different
/// news and the fault is what tells them apart: a FUSE puts material
/// the user took out of the picture back into it, and a DELETE takes
/// the instance with the hide. A user who reports "the part I hid is
/// visible again" is reporting the first one.
#[test]
fn a_hide_the_picture_can_no_longer_honour_is_dropped_and_reported() {
    let tol = Tol::witness();

    // ── The FUSE arm. The hidden post is unioned with the other one,
    // so no display op can address it separately any more.
    let bench = asm::bench("hidefused", tol);
    let mut session = asm::open_bench(&bench, tol);
    assert!(
        session
            .perform(SessionOp::SetInstanceHidden {
                instance: bench.post_b,
                hidden: true,
            })
            .refusal
            .is_none()
    );
    assert!(session.display().is_hidden(bench.post_b));

    let outcome = session.perform(SessionOp::AddBoolean {
        op: pncad::document::BooleanOp::Union,
        a: bench.post_b,
        b: bench.post_a,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let [dropped] = &outcome.withdrawn.dropped_hides[..] else {
        panic!(
            "the fuse drops exactly one hide: {:?}",
            outcome.withdrawn.dropped_hides
        )
    };
    assert_eq!(dropped.instance, bench.post_b);
    assert!(
        matches!(dropped.cause, AdmissionFault::FusedGeometry { .. }),
        "and the outcome carries WHY the part is drawn again: {}",
        dropped.cause
    );
    assert!(
        !session.display().is_hidden(bench.post_b),
        "the hide is gone from the state, not merely reported"
    );
    assert!(
        outcome.withdrawn.superseded.is_empty(),
        "a dropped hide is not a supersession and does not ride that field"
    );

    // ── The DELETE arm. Same class of fact, different sentence: the
    // instance is not drawn again, it is gone.
    let bench = asm::bench("hidedeleted", tol);
    let mut session = asm::open_bench(&bench, tol);
    assert!(
        session
            .perform(SessionOp::SetInstanceHidden {
                instance: bench.post_b,
                hidden: true,
            })
            .refusal
            .is_none()
    );
    let outcome = session.perform(SessionOp::DeleteNode { node: bench.post_b });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let [dropped] = &outcome.withdrawn.dropped_hides[..] else {
        panic!(
            "the delete drops exactly one hide: {:?}",
            outcome.withdrawn.dropped_hides
        )
    };
    assert_eq!(dropped.instance, bench.post_b);
    assert!(
        matches!(dropped.cause, AdmissionFault::NoSuchNode { .. }),
        "an absent node is spelled apart from a wrong-kind one, because \
         the sentence the user reads is the difference: {}",
        dropped.cause
    );
    assert!(session.display().hidden().is_empty());
}

/// **A document REPLACEMENT takes every hide and every placement and
/// reports none of them — by decision, not by omission.**
///
/// `DisplayState::prune`'s withdrawals are news because they are a
/// SIDE EFFECT of an act about something else: the user mated two
/// parts and lost a hand placement they never offered to give up.
/// `Open` and `NewDocument` are the act itself — display state is
/// state of a session over ONE document (G3), so a user who replaces
/// the document has asked for exactly this, and a per-instance notice
/// would report the act back to the person who performed it.
///
/// The clause at `DisplayState::clear` carries the argument, including
/// the half that makes it a typing fact rather than a taste in
/// wording: a `Withdrawn` names an instance and a fault ABOUT a
/// document, and the only document left to ask is the replacement,
/// where these ids mean other nodes or none.
///
/// **The in-flight drag is the one thing the door no longer takes**,
/// and the same clause is why: a drag dissolved under the pointer is
/// the half-acted state a refusal exists to prevent, and the paragraph
/// above is the reason a REPORT could not have been the answer for it
/// either. So the door refuses while a probe is in flight
/// (`SessionOp::permitted_during_free_move`) and takes everything else
/// in silence once there is no drag to dissolve — which is the split
/// this row now asserts, in that order.
///
/// This row is what goes red if the silence is ever widened back into
/// an oversight — it asserts both halves, that everything went and
/// that nothing was said about it — and if the refusal in front of it
/// is ever removed.
#[test]
fn a_document_replacement_takes_all_display_state_and_reports_none_of_it() {
    let tol = Tol::witness();
    let bench = asm::bench("replacequiet", tol);
    let mut session = asm::open_bench(&bench, tol);

    // One of each kind of display state the door can take: a hide, a
    // COMMITTED free-move placement, and a drag still in flight.
    assert!(
        session
            .perform(SessionOp::SetInstanceHidden {
                instance: bench.post_a,
                hidden: true,
            })
            .refusal
            .is_none()
    );
    for op in [
        SessionOp::BeginFreeMove {
            instance: bench.post_b,
        },
        SessionOp::PreviewFreeMove {
            instance: bench.post_b,
            frame: Frame::translation([0.02, 0.0, 0.0]),
        },
        SessionOp::CommitFreeMove {
            instance: bench.post_b,
        },
        SessionOp::BeginFreeMove {
            instance: bench.shelf_i,
        },
        SessionOp::PreviewFreeMove {
            instance: bench.shelf_i,
            frame: Frame::translation([0.0, 0.03, 0.0]),
        },
    ] {
        let outcome = session.perform(op.clone());
        assert!(outcome.refusal.is_none(), "{op:?}: {:?}", outcome.refusal);
    }
    assert_eq!(session.display().hidden().len(), 1);
    assert!(session.display().free_move_of(bench.post_b).is_some());
    assert_eq!(session.display().probing(), Some(bench.shelf_i));

    // With the drag in flight the door does not open at all, and the
    // refusal names the drag rather than the file: nothing of the
    // outgoing document is touched, so there is nothing to have been
    // silent about.
    let held = session.display().revision();
    let refused = session.perform(SessionOp::Open(bench.asm_path.clone()));
    assert!(
        matches!(
            refused.refusal,
            Some(Refusal::Display(DisplayFault::FreeMoveInFlight))
        ),
        "a replacement under a live probe refuses, in the free move's own \
         vocabulary: {:?}",
        refused.refusal
    );
    assert!(
        session.display().hidden().len() == 1
            && session.display().free_move_of(bench.post_b).is_some()
            && session.display().probing() == Some(bench.shelf_i)
            && session.display().revision() == held,
        "…and takes nothing on the way out — a refusal that cleared \
         anything would be the defect with a sentence in front of it"
    );

    // The user ends the drag themselves, which is the remedy the
    // refusal names.
    assert!(session.perform(SessionOp::CancelFreeMove).refusal.is_none());
    assert!(session.display().probing().is_none());
    let before = session.display().revision();

    // Reopening the SAME file is still a replacement: the session's
    // subject is installed afresh, and the ids it carries are minted
    // by that document rather than inherited from the one that went.
    let outcome = session.perform(SessionOp::Open(bench.asm_path.clone()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        frame::Withdrawal::all(&outcome.withdrawn).count(),
        0,
        "a replacement reports no withdrawal of any kind — asserted \
         through the one fan-out the chrome uses, so a FOURTH kind is \
         covered by this row the day it exists: {:?}",
        outcome.withdrawn
    );
    assert!(
        session.display().hidden().is_empty()
            && session.display().free_move_of(bench.post_b).is_none(),
        "…and it took the hide and the committed placement anyway, which \
         is the asymmetry this row records as decided"
    );
    assert!(
        session.display().revision() > before,
        "the reset was visible, so the chrome's rebuild key moved — the \
         one thing the quiet door still says, and it says it to the \
         chrome rather than to the user"
    );
}
