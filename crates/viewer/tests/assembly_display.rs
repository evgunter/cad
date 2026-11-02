//! **Assemblies in the viewer: the resolver, hide, and free-move**
//! (GUI-4 deliverables 1–3, headless).
//!
//! The fixture is a gallery-shaped workspace on disk (`common::asm`):
//! two part documents beside the assembly that pins them. The rows
//! here pin the three display-layer claims — the open path wires a
//! resolver under the directory rule; a hidden instance leaves the
//! picture and the pick index but never the document or the tree; a
//! free-move probe admits only a shown space other than the world (DI5), moves every copy
//! in it as one, draws visibly distinct, and is DISCARDED when the
//! landing of a pin to the world says the space is gone.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use pncad::document::Bodies;

use common::asm;
use pncad::document::{Frame, RecipeNodeId, VarId, product};
use pncad::geom_core::Tol;
use viewer::display::{self, AdmissionFault, DisplayFault};
use viewer::frame;
use viewer::marks;
use viewer::pickindex::IdMap;
use viewer::scene::SceneMesh;
use viewer::session::{DocSession, Refusal, Selection, SessionOp};
use viewer::tree::RowStatus;

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
    assert_eq!(
        rows.len(),
        4,
        "three instances, each its own placement, and the product's ⊔"
    );
    for row in &rows {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    assert_eq!(asm::instance_rows(&session).len(), 3);
    let (doc, eval) = session.landed_pair().expect("landed");
    let body = product(doc, eval, tol).expect("the product gathers");
    assert_eq!(body.shells().count(), 3, "two posts and a shelf");
}

#[test]
fn a_session_with_no_backing_file_resolves_nothing_and_refuses_typed() {
    let tol = Tol::witness();
    let bench = asm::bench("noresolver", tol);
    // The same document VALUE, held in memory: no file, so no store —
    // the viewer's no-file refusal renders as the tree's badges.
    let history = viewer::docio::open(&bench.asm_path, tol).expect("the file opens");
    let mut session = DocSession::inline(history.doc().clone(), tol);
    session.pump();
    for row in asm::instance_rows(&session) {
        match &row.status {
            RowStatus::Failed { message, .. } => assert!(
                message.contains("this document has no file"),
                "the refusal names the missing file: {message}"
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
            RowStatus::Failed { message, .. } => assert!(
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
    for row in asm::instance_rows(&session) {
        match &row.status {
            RowStatus::Failed { message, .. } => assert!(
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
    for row in asm::instance_rows(&session) {
        match &row.status {
            RowStatus::Failed { message, .. } => assert!(
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
    let outcome = session.perform(SessionOp::SetBodyOn {
        node: bench.post_b,
        on: false,
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
    session.perform(SessionOp::SetBodyOn {
        node: bench.post_b,
        on: true,
    });
    let restored = index.scene_for(&session.display_view()).expect("a scene");
    assert_eq!(restored.stats().triangles, full.stats().triangles);
}

/// **A held face on a hidden instance is not marked.** Hiding edits
/// what the picture emits and not what an id means, so the face's patch
/// id outlives the hide; what the held mark reads is whether the
/// picture DRAWS it (`marks::drawn_patch`), and after the hide it does
/// not.
#[test]
fn a_held_face_on_a_hidden_instance_is_not_marked() {
    let tol = Tol::witness();
    let bench = asm::bench("held-hide", tol);
    let mut session = asm::open_bench(&bench, tol);
    let index = asm::index_of(&session);
    let face = asm::pick_face(&session, &asm::over_post_b());
    assert_eq!(face.node, bench.post_b, "the pick is on post_b");
    let held = marks::Held {
        faces: [Some(&face), None, None],
        edges: None,
        entities: [None, None],
    };
    let read = |session: &DocSession| {
        let view = session.display_view();
        let (marked, _) = marks::compose(&index, &view, &Selection::None, None, &held);
        marked.held[0]
    };
    let mark = read(&session);
    assert_ne!(mark, IdMap::NOTHING, "a drawn held face is marked");

    let outcome = session.perform(SessionOp::SetBodyOn {
        node: bench.post_b,
        on: false,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let mark = read(&session);
    assert_eq!(mark, IdMap::NOTHING, "a hidden held face is not marked");
}

/// Two instances of one part consumed by a single boolean, which is
/// placed into the world: the union reads both as bodies, so neither is
/// drawn, and the placement's copy of the union is. The session, the two
/// instances and the placement.
fn fused_pair(tag: &str, tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let bench = asm::bench(tag, tol);
    let mut ws = pncad::workspace::Workspace::open(&bench.dir).expect("the store opens");
    let mut doc = pncad::document::ProfileDoc::empty(
        pncad::document::DocumentId::derive(&format!("gui4-{tag}")),
        tol,
    );
    let a = asm::instance_at(&mut doc, bench.post, [0.0; 3], &bench.opts(), tol);
    let b = asm::instance_at(&mut doc, bench.post, [0.0; 3], &bench.opts(), tol);
    let members = vec![asm::copy(&bench, &doc, a), asm::copy(&bench, &doc, b)];
    let weld = common::insert_into(
        &mut doc,
        pncad::document::Node::Union {
            members: Bodies::Spelled(members),
        },
        tol,
    );
    let (doc, place) = common::placed_read(&doc, weld.into(), tol);
    let path = ws.create(&doc, tol).expect("the fused assembly stores");
    let mut session = DocSession::inline(
        pncad::document::Doc::empty_derived(&format!("gui4-{tag}-boot"), tol),
        tol,
    );
    assert!(session.perform(SessionOp::Open(path)).refusal.is_none());
    // The product the copy of the union alone, so no `⊔` draws the
    // instances as its pieces.
    let product = session.perform(SessionOp::SetProduct {
        bodies: vec![place.into()],
    });
    assert!(product.refusal.is_none(), "{:?}", product.refusal);
    session.pump();
    (session, a, b, place)
}

/// **A fused instance is not drawn, and the body that draws its
/// material is the one a hide takes off the picture.** The union reads
/// both instances as bodies, so the picture draws the placement's copy
/// of the union and neither instance; hiding an instance is accepted (it
/// is the state of that body's toggle) and leaves the picture as it was,
/// and hiding the copy empties it.
///
/// The runtime values that make it false: an instance drawn beside the
/// union (the scene's triangles change when it is hidden), or a hide
/// that does not reach the copy (the scene keeps its triangles).
#[test]
fn a_fused_instance_is_drawn_through_the_body_that_reads_it() {
    let tol = Tol::witness();
    let (mut session, a, b, place) = fused_pair("fused", tol);
    let doc = session.doc().clone();
    assert_eq!(
        viewer::world::drawn_nodes(&doc),
        vec![place],
        "the copy of the union is the one drawn body"
    );
    let index = asm::index_of(&session);
    let full = index.scene_for(&session.display_view()).expect("a scene");
    assert!(full.stats().triangles > 0, "the copy draws");
    for instance in [a, b] {
        let outcome = session.perform(SessionOp::SetBodyOn {
            node: instance,
            on: false,
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    }
    assert_eq!(
        index
            .scene_for(&session.display_view())
            .expect("a scene")
            .stats()
            .triangles,
        full.stats().triangles,
        "hiding a body the picture does not draw takes nothing off it"
    );
    let outcome = session.perform(SessionOp::SetBodyOn {
        node: place,
        on: false,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        index
            .scene_for(&session.display_view())
            .expect("a scene")
            .stats()
            .triangles,
        0,
        "hiding the copy takes the fused material off the picture"
    );
}

#[test]
fn the_at_rest_badge_lands_with_the_evaluation() {
    // The at-rest badge lives past the commit: the mate-less assembly
    // certifies, and so does a seated one — the seat is a touch between
    // copies the census records at the door, never an overlap.
    let tol = Tol::witness();
    let bench = asm::bench("atrest", tol);
    let mut session = asm::open_bench(&bench, tol);
    assert_eq!(
        session.at_rest(),
        Some(&viewer::session::AtRestBadge {
            loud: 0,
            quiet: 0,
            could_not_look: 0,
        }),
        "disjoint instances certify outright (A5's disjoint half)"
    );
    asm::unpin(&mut session, bench.post_b);
    common::commit_mate(
        &mut session,
        asm::seat_op(&bench, bench.post_b, asm::middle_seat_values()),
    );
    assert_eq!(
        session.at_rest(),
        Some(&viewer::session::AtRestBadge {
            loud: 0,
            quiet: 0,
            could_not_look: 0,
        }),
        "a seated mate certifies, its touch recorded at rest"
    );
}

/// **An id the document does not hold**, which is not the same
/// refusal as a node of the wrong kind — the row was named for the
/// wrong one of the two until they were spelled apart.
#[test]
fn hide_refuses_an_id_the_document_does_not_hold() {
    let tol = Tol::witness();
    let bench = asm::bench("hidewrong", tol);
    let mut session = asm::open_bench(&bench, tol);
    let outcome = session.perform(SessionOp::SetBodyOn {
        node: RecipeNodeId::new(0, 9_999),
        on: false,
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

/// **The body admission answers the two states apart**: an absent id
/// and a node that builds no body are different news, and a door that
/// answered `bool` could not say which it had met. Asserted at
/// `body_check` itself rather than only through the ops above, because
/// the door is `pub` and the next caller is the one that will need them
/// apart.
#[test]
fn body_check_tells_an_absent_node_from_one_that_builds_no_body() {
    let tol = Tol::witness();
    let bench = asm::bench("instcheck", tol);
    let mut session = asm::open_bench(&bench, tol);
    // A node that builds no body, authored through the ordinary door so
    // the arm is driven by a node a user can really select.
    let profile = common::rectangle_in(&mut session, 0.01, 0.01);
    session.pump();
    let doc = session.doc();

    assert_eq!(
        display::body_check(doc, bench.post_b),
        Ok(()),
        "an instance builds a body"
    );
    assert_eq!(
        display::body_check(doc, profile),
        Err(AdmissionFault::NotABody {
            node: doc.spoken(profile)
        }),
        "a node that IS in the document and builds no body names itself"
    );
    let absent = RecipeNodeId::new(0, test_utils::refusal::tagged(9_999));
    assert_eq!(
        display::body_check(doc, absent),
        Err(AdmissionFault::NoSuchNode {
            node: doc.spoken(absent)
        }),
        "an id the document does not hold is the ABSENT refusal"
    );
    assert_eq!(
        display::body_check(doc, absent)
            .expect_err("absent refuses")
            .to_string(),
        "node 00000000270f is not in the document",
    );
    assert_eq!(
        display::body_check(doc, profile)
            .expect_err("a profile builds no body")
            .to_string(),
        format!(
            "{} builds no body, so the picture draws nothing of it",
            doc.spoken(profile)
        ),
    );
}

#[test]
fn hide_is_never_persisted() {
    let tol = Tol::witness();
    let bench = asm::bench("hidesave", tol);
    let mut session = asm::open_bench(&bench, tol);
    session.perform(SessionOp::SetBodyOn {
        node: bench.post_b,
        on: false,
    });
    // Save the document OVER its own file and reopen it: the hidden
    // set is display state of the closed session, not of the file.
    let outcome = session.perform(SessionOp::Save(bench.asm_path.clone()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let reopened = asm::open_bench(&bench, tol);
    assert!(reopened.display().toggled().is_empty());
}

// --- free-move (deliverable 3) ------------------------------------

/// **Free-move admits a space other than the world** (DI5).
/// A copy pinned to the world refuses typed, and so does one mated onto
/// a copy in the world, as does the world copy it is mated onto: all
/// three live in the world's space. The same post, its pin taken off,
/// is in a space of its own and is admitted.
#[test]
fn free_move_accepts_only_a_space_other_than_the_world() {
    let tol = Tol::witness();
    let bench = asm::bench("fmeligible", tol);
    let mut session = asm::open_bench(&bench, tol);
    // Constrain post_a by mating it to the shelf, its world mate off.
    asm::unpin(&mut session, bench.post_a);
    common::commit_mate(
        &mut session,
        asm::seat_op(&bench, bench.post_a, asm::middle_seat_values()),
    );
    session.pump();
    // The mated post, the shelf it is mated onto, and the post the
    // world still pins all live in the world's space and refuse,
    // naming the instance.
    for pinned in [bench.post_a, bench.shelf_i, bench.post_b] {
        let outcome = session.perform(SessionOp::BeginFreeMove { node: pinned });
        match outcome.refusal {
            Some(Refusal::Display(DisplayFault::Admission(AdmissionFault::InTheWorld {
                node: instance,
            }))) => {
                assert_eq!(instance.id(), pinned);
            }
            other => panic!("a copy in the world must refuse typed, got {other:?}"),
        }
    }
    // Its pin off, the second post is in a space of its own and is admitted.
    asm::unpin(&mut session, bench.post_b);
    let outcome = session.perform(SessionOp::BeginFreeMove { node: bench.post_b });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.perform(SessionOp::CancelFreeMove);
    // A node that EXISTS and builds no body — the world — refuses for
    // that.
    let outcome = session.perform(SessionOp::BeginFreeMove {
        node: RecipeNodeId::WORLD,
    });
    assert!(
        matches!(
            outcome.refusal,
            Some(Refusal::Display(DisplayFault::Admission(
                AdmissionFault::NotABody { .. }
            )))
        ),
        "{:?}",
        outcome.refusal
    );
    // And an id the document does not hold refuses for being ABSENT.
    let outcome = session.perform(SessionOp::BeginFreeMove {
        node: RecipeNodeId::new(0, 9_999),
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

/// **Two copies placed into one space free-move as one** (DI5): a
/// frame is keyed by the space, so either copy admits the probe to the
/// one space and the frame moves both pictures. Then the shelf is
/// pinned into the world and post_b given a space of its own, one edit
/// each; the second empties the space, and that edit drops the frame,
/// saying why.
#[test]
fn two_copies_in_one_space_free_move_as_one() {
    let tol = Tol::witness();
    let bench = asm::bench("fmspace", tol);
    let mut session = asm::open_bench(&bench, tol);
    // The shelf in a space of its own, post_b mated onto it: one space.
    asm::unpin(&mut session, bench.shelf_i);
    common::commit_mate(
        &mut session,
        asm::seat_op(&bench, bench.post_b, asm::own_space_seat_values()),
    );
    session.pump();
    let space = display::free_move_check(session.doc(), session.space_evaluation(), bench.post_b)
        .expect("the mated post is in the shelf's space");
    assert_eq!(
        display::free_move_check(session.doc(), session.space_evaluation(), bench.shelf_i),
        Ok(space),
        "the shelf it is mated onto is in the same space"
    );

    let probe = Frame::translation([0.0, 0.0, 0.05]);
    for op in [
        SessionOp::BeginFreeMove { node: bench.post_b },
        SessionOp::PreviewFreeMove {
            node: bench.post_b,
            frame: probe,
        },
        SessionOp::CommitFreeMove { node: bench.post_b },
    ] {
        let outcome = session.perform(op.clone());
        assert!(outcome.refusal.is_none(), "{op:?}: {:?}", outcome.refusal);
        assert!(outcome.committed.is_empty(), "a probe commits no edit");
    }
    assert_eq!(
        session.display().free_move_of(space),
        Some(&probe),
        "the frame is the space's, keyed by it"
    );
    let view = session.display_view();
    for copy in [bench.post_b, bench.shelf_i] {
        assert_eq!(
            view.moved_nodes.get(&copy),
            Some(&probe),
            "the one frame moves every copy in the space"
        );
    }
    assert!(
        !view.moved_nodes.contains_key(&bench.post_a),
        "and nothing placed into the world"
    );

    // Driven from the other copy, the probe is the same space's: its
    // commit replaces the frame rather than adding a second.
    let again = Frame::translation([0.0, 0.0, 0.07]);
    for op in [
        SessionOp::BeginFreeMove {
            node: bench.shelf_i,
        },
        SessionOp::PreviewFreeMove {
            node: bench.shelf_i,
            frame: again,
        },
        SessionOp::CommitFreeMove {
            node: bench.shelf_i,
        },
    ] {
        let outcome = session.perform(op.clone());
        assert!(outcome.refusal.is_none(), "{op:?}: {:?}", outcome.refusal);
    }
    assert_eq!(session.display().free_move_of(space), Some(&again));
    assert_eq!(
        session.display_view().moved_nodes.get(&bench.post_b),
        Some(&again),
        "one space, one frame"
    );

    // The shelf moved into the world, resting on post_a: post_b is
    // still placed into the space, so the frame stays.
    let outcome = session.perform(asm::shelf_rest_op(&bench, bench.post_a, 0.0));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(outcome.withdrawn.superseded.is_empty());
    assert!(session.display().free_move_of(space).is_some());
    // post_b given a space of its own: the space is empty, and the
    // edit drops its frame.
    let outcome = session.perform(SessionOp::IntoOwnSpace {
        placement: bench.post_b,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let [withdrawn] = &outcome.withdrawn.superseded[..] else {
        panic!(
            "the edit drops the one frame: {:?}",
            outcome.withdrawn.superseded
        )
    };
    assert_eq!(
        withdrawn.held,
        viewer::display::Held::Node(bench.shelf_i),
        "named by the instance that committed it"
    );
    assert!(
        matches!(
            &withdrawn.cause,
            AdmissionFault::NotShown { space: said, .. } if said.id() == space
        ),
        "{}",
        withdrawn.cause
    );
    assert!(session.display().free_move_of(space).is_none());
    assert!(session.display_view().moved_nodes.is_empty());
}

/// **A frame whose copy joins another space goes, and says so** (DI5):
/// the probed post's own space holds it alone; a mate onto the shelf in
/// a space of its own places the post into the shelf's, so the post's
/// space is no longer shown and the edit drops its frame.
#[test]
fn a_frame_whose_space_empties_into_another_goes_saying_so() {
    let tol = Tol::witness();
    let bench = asm::bench("fmjoin", tol);
    let mut session = asm::open_bench(&bench, tol);
    let own = asm::unpin(&mut session, bench.post_b);
    asm::unpin(&mut session, bench.shelf_i);
    for op in [
        SessionOp::BeginFreeMove { node: bench.post_b },
        SessionOp::PreviewFreeMove {
            node: bench.post_b,
            frame: Frame::translation([0.0, 0.0, 0.05]),
        },
        SessionOp::CommitFreeMove { node: bench.post_b },
    ] {
        let outcome = session.perform(op.clone());
        assert!(outcome.refusal.is_none(), "{op:?}: {:?}", outcome.refusal);
    }
    assert!(session.display().free_move_of(own).is_some());

    let outcome = session.perform(asm::seat_op(
        &bench,
        bench.post_b,
        asm::own_space_seat_values(),
    ));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let joined = session
        .doc()
        .node_space(bench.post_b)
        .expect("the mated post is placed");
    assert_eq!(
        Some(joined),
        session.doc().node_space(bench.shelf_i),
        "the post is placed into the shelf's space"
    );
    let [withdrawn] = &outcome.withdrawn.superseded[..] else {
        panic!(
            "the edit drops the post's frame: {:?}",
            outcome.withdrawn.superseded
        )
    };
    assert_eq!(withdrawn.held, viewer::display::Held::Node(bench.post_b));
    assert!(
        matches!(
            &withdrawn.cause,
            AdmissionFault::NotShown { space, .. } if space.id() == own
        ),
        "the cause names the space it left: {}",
        withdrawn.cause
    );
    assert!(session.display().free_move_of(own).is_none());
    assert!(session.display().free_move_of(joined).is_none());
}

/// The bench with the shelf in a space of its own, post_b mated onto
/// it, and a free-move gesture in flight opened on post_b. Answers the
/// space.
fn gesture_on_a_non_root_copy(name: &str, tol: Tol) -> (asm::Bench, DocSession, VarId) {
    let bench = asm::bench(name, tol);
    let mut session = asm::open_bench(&bench, tol);
    asm::unpin(&mut session, bench.shelf_i);
    common::commit_mate(
        &mut session,
        asm::seat_op(&bench, bench.post_b, asm::own_space_seat_values()),
    );
    session.pump();
    let space = display::free_move_check(session.doc(), session.space_evaluation(), bench.post_b)
        .expect("the mated post is in the shelf's space");
    assert_eq!(Some(space), session.doc().node_space(bench.shelf_i));
    let outcome = session.perform(SessionOp::BeginFreeMove { node: bench.post_b });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    (bench, session, space)
}

/// **A gesture is re-asked by the copy it was opened on, not only by
/// its space.** post_b, mid-drag, is given a space of its own: the
/// shelf's space is still shown, but post_b is no longer placed into
/// it, so the edit kills the gesture, saying where the post went —
/// rather than letting the post's drag steer the shelf.
#[test]
fn a_gesture_whose_copy_leaves_the_space_dies_at_the_edit() {
    let tol = Tol::witness();
    let (bench, mut session, _) = gesture_on_a_non_root_copy("fmsplit", tol);
    let outcome = session.perform(SessionOp::IntoOwnSpace {
        placement: bench.post_b,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let own = session
        .doc()
        .node_space(bench.post_b)
        .expect("the post is placed");
    let killed = outcome
        .withdrawn
        .killed_gesture
        .as_ref()
        .expect("the edit kills the gesture");
    assert_eq!(killed.held, viewer::display::Held::Node(bench.post_b));
    assert!(
        matches!(
            &killed.cause,
            AdmissionFault::MovedSpace { node: instance, into }
                if instance.id() == bench.post_b && into.id() == own
        ),
        "{}",
        killed.cause
    );
    assert!(session.display().probing().is_none());
    assert!(matches!(
        session
            .perform(SessionOp::PreviewFreeMove {
                node: bench.post_b,
                frame: Frame::translation([0.0, 0.0, 0.01]),
            })
            .refusal,
        Some(Refusal::Display(DisplayFault::NoFreeMove))
    ));
}

/// **Deleting the copy a gesture was opened on kills it at the edit**,
/// typed, though the space's root survives.
#[test]
fn deleting_a_gestures_copy_kills_it_at_the_edit() {
    let tol = Tol::witness();
    let (bench, mut session, _) = gesture_on_a_non_root_copy("fmdelete", tol);
    let outcome = session.perform(SessionOp::DeleteNode { node: bench.post_b });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let killed = outcome
        .withdrawn
        .killed_gesture
        .as_ref()
        .expect("the delete kills the gesture");
    assert_eq!(killed.held, viewer::display::Held::Node(bench.post_b));
    assert!(
        matches!(&killed.cause, AdmissionFault::NoSuchNode { node } if node.id() == bench.post_b),
        "{}",
        killed.cause
    );
    assert!(session.display().probing().is_none());
}

/// **A landing's report goes with the document it is about**: a
/// replacement before the chrome takes it leaves nothing to take, since
/// its ids name the outgoing document's nodes.
#[test]
fn a_replacement_takes_the_landings_untaken_report() {
    let tol = Tol::witness();
    let bench = asm::bench("fmreplace", tol);
    let mut session = asm::open_bench(&bench, tol);
    // A gesture on post_b, seated in the shelf's own space; the shelf
    // then takes a mate the evaluation refuses, against post_a in the
    // world, which places the shelf there and leaves post_b's seat
    // reading it across two spaces: post_b refuses too, and the landing
    // kills the gesture, its report waiting for the chrome.
    let probe_then_refuse = |session: &mut DocSession| {
        for op in [
            SessionOp::IntoOwnSpace {
                placement: bench.shelf_i,
            },
            SessionOp::IntoOwnSpace {
                placement: bench.post_b,
            },
            asm::seat_op(&bench, bench.post_b, asm::middle_seat_values()),
        ] {
            let outcome = session.perform(op.clone());
            assert!(outcome.refusal.is_none(), "{op:?}: {:?}", outcome.refusal);
        }
        session.pump();
        for op in [
            SessionOp::BeginFreeMove { node: bench.post_b },
            asm::no_centre_op(&bench, bench.post_a),
        ] {
            let outcome = session.perform(op.clone());
            assert!(outcome.refusal.is_none(), "{op:?}: {:?}", outcome.refusal);
        }
        session.pump();
    };
    probe_then_refuse(&mut session);
    assert!(
        !session.take_landed_withdrawals().is_empty(),
        "the control: the landing does leave a report"
    );
    probe_then_refuse(&mut session);
    let outcome = session.perform(SessionOp::Open(bench.asm_path.clone()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        session.take_landed_withdrawals().is_empty(),
        "the outgoing document's report is not the incoming one's"
    );
}

/// **A value a gesture drags, on the bench**: a document variable
/// nothing reads, which the gesture writes. It places nothing, so no
/// picture changes. Landed before it answers.
fn drag_slot(session: &mut DocSession, _bench: &asm::Bench) -> VarId {
    let name = pncad::document::VarName::new("drag").expect("a name");
    let outcome = session.perform(SessionOp::DeclareVar {
        name,
        value: pncad::document::FreeVar::continuous(pncad::document::Dimension::Length, 0.03),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    session
        .committed_doc()
        .var_named("drag")
        .expect("the variable is declared")
}

/// A committed free-move frame on `instance`'s space.
fn commit_frame(session: &mut DocSession, instance: RecipeNodeId) {
    for op in [
        SessionOp::BeginFreeMove { node: instance },
        SessionOp::PreviewFreeMove {
            node: instance,
            frame: Frame::translation([0.0, 0.0, 0.05]),
        },
        SessionOp::CommitFreeMove { node: instance },
    ] {
        let outcome = session.perform(op.clone());
        assert!(outcome.refusal.is_none(), "{op:?}: {:?}", outcome.refusal);
    }
}

/// Open a value gesture on `var`, preview it, and pump: the landing
/// that arrives is the scratch's.
fn drag_and_land(session: &mut DocSession, var: VarId) {
    for op in [
        SessionOp::BeginVariableGesture { var },
        SessionOp::PreviewVariableGesture { var, value: 0.04 },
    ] {
        let outcome = session.perform(op.clone());
        assert!(outcome.refusal.is_none(), "{op:?}: {:?}", outcome.refusal);
    }
    session.pump();
    assert_ne!(
        session.landed_pair().map(|(doc, _)| doc),
        Some(session.committed_doc()),
        "the landing is the scratch's: the committed edit's own was overtaken"
    );
}

/// The frames the landings since the last read discarded.
fn landed_superseded(session: &mut DocSession) -> Vec<viewer::display::Withdrawn> {
    session
        .take_landed_withdrawals()
        .into_iter()
        .flat_map(|report| report.superseded)
        .collect()
}

/// **An undo under a value drag drops a frame at the edit**: frames are
/// asked of the document alone, so an undo that takes post_b's own
/// space says so at once, and the scratch landing of a drag opened
/// after it has nothing left to say.
#[test]
fn an_undo_before_a_value_drag_drops_the_frame_on_the_edit() {
    let tol = Tol::witness();
    let bench = asm::bench("fmovertaken", tol);
    let mut session = asm::open_bench(&bench, tol);
    let dragged = drag_slot(&mut session, &bench);
    let own = asm::unpin(&mut session, bench.post_b);
    commit_frame(&mut session, bench.post_b);

    // The space taken back, and then a drag.
    let outcome = session.perform(SessionOp::Undo);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let [withdrawn] = &outcome.withdrawn.superseded[..] else {
        panic!(
            "the undo drops the frame: {:?}",
            outcome.withdrawn.superseded
        )
    };
    assert_eq!(withdrawn.held, viewer::display::Held::Node(bench.post_b));
    assert!(
        matches!(
            &withdrawn.cause,
            AdmissionFault::NoSuchSpace { space, .. } if space.id() == own
        ),
        "{}",
        withdrawn.cause
    );
    assert!(session.display().free_move_of(own).is_none());
    drag_and_land(&mut session, dragged);
    assert!(
        landed_superseded(&mut session).is_empty(),
        "and the scratch landing has nothing left to say"
    );
    assert!(session.perform(SessionOp::CancelGesture).refusal.is_none());
}

/// **A frame whose copy is deleted goes at the edit**, typed: the
/// document alone says no copy is placed into the space any more.
#[test]
fn deleting_a_frames_copy_drops_it_at_the_edit() {
    let tol = Tol::witness();
    let bench = asm::bench("fmdeleteroot", tol);
    let mut session = asm::open_bench(&bench, tol);
    let own = asm::unpin(&mut session, bench.post_b);
    commit_frame(&mut session, bench.post_b);
    let outcome = session.perform(SessionOp::DeleteNode { node: bench.post_b });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let [withdrawn] = &outcome.withdrawn.superseded[..] else {
        panic!(
            "the delete drops the frame: {:?}",
            outcome.withdrawn.superseded
        )
    };
    assert!(
        matches!(&withdrawn.cause, AdmissionFault::NotShown { space, .. } if space.id() == own),
        "{}",
        withdrawn.cause
    );
    assert!(session.display().free_move_of(own).is_none());
}

/// **A gesture whose copy a refusal takes waits for the committed
/// landing**: under a free-move in flight on post_b, seated in the
/// shelf's own space, the shelf takes a mate the evaluation refuses,
/// against post_a in the world, which places the shelf there — post_b
/// stays placed into the shelf's old space, still shown, so the edit
/// keeps the gesture, and refuses where it evaluates, its seat reading
/// across two spaces. A value drag's scratch landing keeps the gesture
/// too, and the committed landing after the cancel kills it, saying the
/// post has no picture.
#[test]
fn a_refused_gestures_copy_waits_for_the_committed_landing() {
    let tol = Tol::witness();
    let bench = &asm::bench("fmgesturerefused", tol);
    let mut session = asm::open_bench(bench, tol);
    let dragged = drag_slot(&mut session, bench);
    asm::unpin(&mut session, bench.shelf_i);
    common::commit_mate(
        &mut session,
        asm::seat_op(bench, bench.post_b, asm::own_space_seat_values()),
    );
    let outcome = session.perform(SessionOp::BeginFreeMove { node: bench.post_b });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let seat = session.perform(asm::no_centre_op(bench, bench.post_a));
    assert!(seat.refusal.is_none(), "{:?}", seat.refusal);
    assert!(seat.withdrawn.killed_gesture.is_none(), "the edit keeps it");
    drag_and_land(&mut session, dragged);
    assert!(
        session
            .take_landed_withdrawals()
            .iter()
            .all(|report| report.killed_gesture.is_none()),
        "the scratch landing kills nothing over a refusal"
    );
    assert_eq!(session.display().probing(), Some(bench.post_b));

    assert!(session.perform(SessionOp::CancelGesture).refusal.is_none());
    session.pump();
    let killed: Vec<_> = session
        .take_landed_withdrawals()
        .into_iter()
        .filter_map(|report| report.killed_gesture)
        .collect();
    let [killed] = &killed[..] else {
        panic!("the committed landing kills the gesture: {killed:?}")
    };
    assert!(
        matches!(
            &killed.cause,
            AdmissionFault::NoSpace { node: instance } if instance.id() == bench.post_b
        ),
        "{}",
        killed.cause
    );
    assert!(session.display().probing().is_none());
}

/// **Deleting a gesture's space kills the gesture at the edit**,
/// typed: the space is gone, which the document alone says.
#[test]
fn deleting_a_gestures_space_kills_it() {
    let tol = Tol::witness();
    let (_, mut session, space) = gesture_on_a_non_root_copy("fmdeletegestureroot", tol);
    let (node, _) = session
        .doc()
        .defined_by(space)
        .expect("a `new_space()` defines the space");
    let outcome = session.perform(SessionOp::DeleteNode { node });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let killed = outcome
        .withdrawn
        .killed_gesture
        .as_ref()
        .expect("the delete kills the gesture");
    assert!(
        matches!(
            &killed.cause,
            AdmissionFault::NoSuchNode { .. } | AdmissionFault::NoSuchSpace { .. }
        ),
        "{}",
        killed.cause
    );
    assert!(session.display().probing().is_none());
}

/// **An undo that places a gesture's copy elsewhere kills the gesture
/// at the edit.** post_b, in a space of its own, is seated under a
/// second shelf in a space of its own, which puts it in that shelf's
/// space; a gesture opens on it there, and undoing the seat puts post_b
/// back in its own space. No landing answers the document yet, and the
/// recipe alone says where post_b is placed — and it is asked.
#[test]
fn an_undo_that_moves_a_gestures_copy_kills_it_at_the_edit() {
    let tol = Tol::witness();
    let bench = asm::bench("fmundoroot", tol);
    let mut session = asm::open_bench(&bench, tol);
    let own = asm::unpin(&mut session, bench.post_b);
    let shelf = common::instance_in(&mut session, bench.shelf.id);
    let shelf_space = asm::unpin(&mut session, shelf);
    common::commit_mate(
        &mut session,
        asm::seat_op_under(&bench, bench.post_b, shelf, asm::middle_seat_values()),
    );
    session.pump();
    assert_eq!(
        display::free_move_check(session.doc(), session.space_evaluation(), bench.post_b),
        Ok(shelf_space),
        "post_b is placed into the second shelf's space"
    );
    let outcome = session.perform(SessionOp::BeginFreeMove { node: bench.post_b });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);

    let outcome = session.perform(SessionOp::Undo);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let killed = outcome
        .withdrawn
        .killed_gesture
        .expect("the undo that moved the copy killed the gesture");
    assert!(
        matches!(
            &killed.cause,
            AdmissionFault::MovedSpace { node: instance, into }
                if instance.id() == bench.post_b && into.id() == own
        ),
        "{}",
        killed.cause
    );
    assert!(session.display().probing().is_none());
}

#[test]
fn the_probe_gesture_previews_commits_and_draws_visibly_distinct() {
    let tol = Tol::witness();
    let bench = asm::bench("fmgesture", tol);
    let mut session = asm::open_bench(&bench, tol);
    // post_b in a space of its own, which the probe
    // moves, each frame composed so it is drawn where its pin stood it.
    asm::unpin(&mut session, bench.post_b);
    let index = asm::index_of(&session);
    let baseline = index.scene_for(&session.display_view()).expect("a scene");
    assert_eq!(baseline.stats().probe_parts, 0);
    assert!(baseline.flags().iter().all(|&f| f == 0));

    // Begin, preview twice (previews replace), commit once.
    session.perform(SessionOp::BeginFreeMove { node: bench.post_b });
    for dx in [0.02, 0.05] {
        let outcome = session.perform(SessionOp::PreviewFreeMove {
            node: bench.post_b,
            frame: asm::where_b_stood(Frame::translation([dx, 0.0, 0.0])),
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    }
    let outcome = session.perform(SessionOp::CommitFreeMove { node: bench.post_b });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let committed =
        crate::common::asm::frame_of(&session, bench.post_b).expect("the probe committed");
    assert_eq!(
        *committed,
        asm::where_b_stood(Frame::translation([0.05, 0.0, 0.0])),
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
    let (probed_box, baseline_box) = (
        probed.bounds().expect("the probed picture draws"),
        baseline.bounds().expect("the baseline picture draws"),
    );
    assert!(
        (probed_box.max_x - want_max_x).abs() < 1e-6,
        "the probed part is DRAWN displaced by the probe: {} vs {want_max_x} (baseline {})",
        probed_box.max_x,
        baseline_box.max_x
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
    session.perform(SessionOp::BeginFreeMove { node: bench.post_b });
    session.perform(SessionOp::PreviewFreeMove {
        node: bench.post_b,
        frame: Frame::translation([0.0, 0.0, 0.3]),
    });
    session.perform(SessionOp::CancelFreeMove);
    assert_eq!(
        crate::common::asm::frame_of(&session, bench.post_b)
            .expect("still committed")
            .translation,
        asm::where_b_stood(Frame::translation([0.05, 0.0, 0.0])).translation
    );

    // Kept beside the document (IDENTITY DI5, `viewer::layout`): save
    // over the file, reopen, the committed probe is drawn where it was.
    let committed = *crate::common::asm::frame_of(&session, bench.post_b).expect("committed");
    session.perform(SessionOp::Save(bench.asm_path.clone()));
    let reopened = asm::open_bench(&bench, tol);
    assert_eq!(
        crate::common::asm::frame_of(&reopened, bench.post_b).copied(),
        Some(committed),
        "the committed probe reopens"
    );
}

#[test]
fn a_non_rigid_preview_refuses_typed() {
    let tol = Tol::witness();
    let bench = asm::bench("fmrigid", tol);
    let mut session = asm::open_bench(&bench, tol);
    asm::unpin(&mut session, bench.post_b);
    session.perform(SessionOp::BeginFreeMove { node: bench.post_b });
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
            node: bench.post_b,
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
        node: bench.post_b,
        frame: Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.5, [0.01, 0.0, 0.0], common::band())
            .expect("a literal axis has a definite direction"),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
}

/// **A mate that places a probed copy into another space discards its
/// frame at the edit** (DI5: the frame is dropped once its space is no
/// longer shown). The mate re-targets post_b into the shelf's space, the
/// world, and writes its bundle — two edits in one action — and the
/// space it left holds nothing.
#[test]
fn a_mate_that_moves_the_copy_discards_the_probe_value() {
    let tol = Tol::witness();
    let bench = asm::bench("fmsupersede", tol);
    let mut session = asm::open_bench(&bench, tol);
    let index = asm::index_of(&session);
    // In a space of its own, post_b's space is shown.
    let own = asm::unpin(&mut session, bench.post_b);
    // Probe post_b somewhere.
    for op in [
        SessionOp::BeginFreeMove { node: bench.post_b },
        SessionOp::PreviewFreeMove {
            node: bench.post_b,
            frame: Frame::translation([0.04, 0.0, 0.0]),
        },
        SessionOp::CommitFreeMove { node: bench.post_b },
    ] {
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    }
    assert!(session.display().free_move_of(own).is_some());

    let outcome = session.perform(asm::seat_op(
        &bench,
        bench.post_b,
        asm::middle_seat_values(),
    ));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        outcome.committed.len(),
        3,
        "the re-target, the mate and its values: {:?}",
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
        "the supersession is reported, not inferred"
    );
    // The PAYLOAD, not the variant: what would go red if `prune` paired
    // the right fault with the wrong space is this.
    let AdmissionFault::NotShown { space, .. } = &superseded.cause else {
        panic!(
            "a mate emptying the space supersedes with its own fault: {}",
            superseded.cause
        )
    };
    assert_eq!(space.id(), own, "the fault names the space it emptied");
    // DISCARDED, not zeroed: the value is gone, and the map holds no
    // identity entry standing in for it.
    assert!(session.display().free_move_of(own).is_none());
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
/// The instance goes with the hide, and the outcome says the node is
/// absent rather than one that builds no body.
#[test]
fn a_hide_the_picture_can_no_longer_honour_is_dropped_and_reported() {
    let tol = Tol::witness();

    let bench = asm::bench("hidedeleted", tol);
    let mut session = asm::open_bench(&bench, tol);
    assert!(
        session
            .perform(SessionOp::SetBodyOn {
                node: bench.post_b,
                on: false,
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
    assert_eq!(dropped.held, viewer::display::Held::Node(bench.post_b));
    assert!(
        matches!(dropped.cause, AdmissionFault::NoSuchNode { .. }),
        "an absent node is spelled apart from a wrong-kind one, because \
         the sentence the user reads is the difference: {}",
        dropped.cause
    );
    assert!(session.display().toggled().is_empty());
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
    // post_b and the shelf each in a space of its own, which the free-move moves.
    asm::unpin(&mut session, bench.post_b);
    asm::unpin(&mut session, bench.shelf_i);

    // One of each kind of display state the door can take: a hide, a
    // COMMITTED free-move placement, and a drag still in flight.
    assert!(
        session
            .perform(SessionOp::SetBodyOn {
                node: bench.post_a,
                on: false,
            })
            .refusal
            .is_none()
    );
    for op in [
        SessionOp::BeginFreeMove { node: bench.post_b },
        SessionOp::PreviewFreeMove {
            node: bench.post_b,
            frame: Frame::translation([0.02, 0.0, 0.0]),
        },
        SessionOp::CommitFreeMove { node: bench.post_b },
        SessionOp::BeginFreeMove {
            node: bench.shelf_i,
        },
        SessionOp::PreviewFreeMove {
            node: bench.shelf_i,
            frame: Frame::translation([0.0, 0.03, 0.0]),
        },
    ] {
        let outcome = session.perform(op.clone());
        assert!(outcome.refusal.is_none(), "{op:?}: {:?}", outcome.refusal);
    }
    assert_eq!(session.display().toggled().len(), 1);
    assert!(crate::common::asm::frame_of(&session, bench.post_b).is_some());
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
        session.display().toggled().len() == 1
            && crate::common::asm::frame_of(&session, bench.post_b).is_some()
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
        session.display().toggled().is_empty()
            && crate::common::asm::frame_of(&session, bench.post_b).is_none(),
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

/// **A frame on a space whose bodies a union reads stays while the
/// product draws them as its pieces, and goes, saying so, once nothing
/// draws them.** The shelf and post_b share a space with a frame held on
/// it; a placed union of the two reads both as bodies, but the
/// product's `⊔` still reads them, so the picture draws them as its
/// pieces and the space stays shown. Emptying the product leaves nothing
/// drawing them: the space is no longer shown, and that edit drops the
/// frame.
#[test]
fn a_frame_whose_bodies_are_fused_goes_once_nothing_draws_them() {
    let tol = Tol::witness();
    let bench = asm::bench("fmfuse", tol);
    let union = asm::store_union(&bench, &[bench.shelf_i, bench.post_b], tol);
    asm::store_world_copy(&bench, union, tol);
    let mut session = asm::open_bench(&bench, tol);
    asm::unpin(&mut session, bench.shelf_i);
    common::commit_mate(
        &mut session,
        asm::seat_op(&bench, bench.post_b, asm::own_space_seat_values()),
    );
    session.pump();
    let space = display::free_move_check(session.doc(), session.space_evaluation(), bench.post_b)
        .expect("the mated post is in the shelf's space: the product draws it as its piece");
    commit_frame(&mut session, bench.post_b);
    assert!(session.display().free_move_of(space).is_some());

    let outcome = session.perform(SessionOp::SetProduct { bodies: vec![] });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    for instance in [bench.shelf_i, bench.post_b] {
        assert!(
            !viewer::world::is_drawn(session.committed_doc(), instance),
            "the union reads {instance:?} as a body and nothing else draws it"
        );
    }
    let [withdrawn] = &outcome.withdrawn.superseded[..] else {
        panic!(
            "the edit drops the space's frame: {:?}",
            outcome.withdrawn.superseded
        )
    };
    assert_eq!(withdrawn.held, viewer::display::Held::Node(bench.post_b));
    assert!(
        matches!(
            &withdrawn.cause,
            AdmissionFault::NotShown { space: left, .. } if left.id() == space
        ),
        "the cause names the space the union reads: {}",
        withdrawn.cause
    );
    assert!(session.display().free_move_of(space).is_none());
}
