//! **A gesture on a copy reads the copy; a gesture on a body offers the
//! re-point; a delete splices by continuation** (Ev, `[ev]` PR 6353; PR
//! 6580's recommendation: every placement is a copy to the viewer).
//!
//! - A feature picked on a placed copy reads that copy alone, in its
//!   space: no placement's source is written, the other copies are
//!   untouched, and the result says it belongs to that copy, with a way
//!   to the original body (`viewer::frame::placement_offers`).
//! - A feature on a body picked as itself makes a new body and offers,
//!   as one explicit step, to re-point the variable the body's
//!   placements share onto it (`SessionOp::RepointPlacements`).
//! - A placement Place makes in the body's own space is a copy like any
//!   other: badged, listed, and the way to its body.
//! - A delete hands each reader of the deleted node to the body it
//!   continues (`pncad::document::splice`), so adding a feature and
//!   deleting it leaves the document as it was.
//!
//! The delete rows keep PR 6280's review probes, re-spelled to the
//! ruled behaviour.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::drawn;
use common::{displayed_face_at, len, plate_index};
use pncad::document::{
    AuthoredNode, Dimension, Doc, DocEdit, Formula, Node, Operand, OperandSlot, ProfileProgram,
    RecipeNodeId, SlotId, VarId,
};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
use viewer::session::{DocSession, FaceSelection, PatternRuleSpec, Selection, SessionOp};
use viewer::tree::RowStatus;

/// The bracket: 20 × 20 × 10 mm, centred on its sketch origin.
const SIZE: [f64; 3] = [0.02, 0.02, 0.01];

/// **The document as a reader sees it**: every node written as authored
/// (anonymous reads as their definitions, so a re-minted selection
/// reads the same), the product, and what is drawn.
type World = (
    Vec<(RecipeNodeId, Option<AuthoredNode>)>,
    Option<VarId>,
    Vec<RecipeNodeId>,
);

fn world(doc: &Doc<ProfileProgram>) -> World {
    let mut ids = doc.ids().to_vec();
    ids.sort_unstable();
    (
        ids.into_iter()
            .map(|id| (id, doc.node(id).map(|node| node.written(doc))))
            .collect(),
        doc.product(),
        viewer::world::drawn_nodes(doc),
    )
}

/// What `copy` copies.
fn source_of(doc: &Doc<ProfileProgram>, copy: RecipeNodeId) -> Option<VarId> {
    doc.copied_read(copy)
}

/// Whether `edits` write a placement's source.
fn writes_a_source(edits: &[DocEdit<ProfileProgram>]) -> bool {
    edits.iter().any(|edit| {
        matches!(
            edit,
            DocEdit::SetParam {
                slot: SlotId::Operand(OperandSlot::Source),
                ..
            }
        )
    })
}

/// The bracket, two world copies of it placed by hand (the first at the
/// identity, the second 0.1 m along y), the product reading the first:
/// the session, the bracket and the two copies.
fn two_world_copies(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived("copy-reads", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    let doc = session.committed_doc().clone();
    let (doc, first) = common::world_copy(&doc, body, tol);
    let (doc, second) =
        viewer::test_support::inserted(&doc, common::world_place(Operand::Node(body)), tol);
    let doc = drawn::pinned(doc, second, &drawn::translated([0.0, 0.1, 0.0]));
    let mut session = common::inline_without_door(doc, tol);
    let product = session.perform(SessionOp::SetProduct {
        bodies: vec![Operand::Node(first)],
    });
    assert!(product.refusal.is_none(), "{:?}", product.refusal);
    session.pump();
    (session, body, first, second)
}

/// The vertical edges of `node`, by their names in the landed run.
fn lateral_edges(session: &DocSession, node: RecipeNodeId) -> Vec<StableName> {
    pncad::select::all_edges(session.evaluation().expect("landed"), node)
        .into_iter()
        .filter(|name| matches!(name.path.as_slice(), [RoleSeg::LateralEdge(_)]))
        .collect()
}

/// `body`'s top cap, picked on the drawn `on`.
fn top(body: RecipeNodeId, on: RecipeNodeId) -> FaceSelection {
    FaceSelection {
        name: StableName {
            kind: EntityKind::Face,
            node: body,
            path: vec![RoleSeg::Cap(CapEnd::End)],
        },
        node: on,
        body: 0,
    }
}

/// `body`'s `+x` wall picked on `on`, as a ray toward `−x` meets it.
fn plus_x_wall(session: &DocSession, y: f64) -> FaceSelection {
    let index = plate_index(session);
    displayed_face_at(
        session,
        &index,
        &common::along_x(-1.0, y, SIZE[2] / 2.0),
    )
}

/// The gestures a copy is picked for, each named, with whether deleting
/// what it made is its inverse (a pattern's, a split's and a fused
/// pattern's is undo).
fn gestures_on(
    session: &DocSession,
    body: RecipeNodeId,
    copy: RecipeNodeId,
) -> Vec<(&'static str, SessionOp, bool)> {
    let edge = lateral_edges(session, copy)[0].clone();
    let row = || PatternRuleSpec::Linear {
        direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
        spacing: len(2.0 * SIZE[0]),
    };
    vec![
        (
            "fillet",
            SessionOp::AddFillet {
                target: copy,
                radius: len(0.001),
                selection: vec![edge.clone()],
            },
            true,
        ),
        (
            "chamfer",
            SessionOp::AddChamfer {
                target: copy,
                distance: len(0.001),
                selection: vec![edge],
            },
            true,
        ),
        (
            "mirror",
            SessionOp::AddMirror {
                body: copy.into(),
                plane: plus_x_wall(session, 0.0),
            },
            true,
        ),
        (
            "sketch on face",
            SessionOp::SketchOnFace {
                face: top(body, copy),
                loops: vec![common::rectangle_loop([0.0, 0.0], 0.004, 0.004)],
                distance: len(0.003),
                spin: Formula::zero(Dimension::Angle),
            },
            true,
        ),
        (
            "split",
            SessionOp::AddSplit {
                target: copy.into(),
                plane: common::cut_across(session, copy, SIZE[2] / 2.0),
            },
            false,
        ),
        (
            "pattern",
            SessionOp::AddPattern {
                input: copy.into(),
                count: 2,
                rule: row(),
            },
            false,
        ),
        (
            "fused pattern",
            SessionOp::AddFusedPattern {
                input: copy.into(),
                count: 2,
                rule: row(),
            },
            false,
        ),
    ]
}

/// **A feature picked on a copy reads that copy alone** (Ev, PR 6353):
/// for each feature gesture, picked on the first of two world copies of
/// the bracket, the gesture writes no placement's source, both copies
/// still place the bracket, the second copy is drawn as it was, and what
/// the gesture made reads the first copy; the result says it belongs to
/// that copy and offers the way to the bracket. Deleting what a fillet,
/// a chamfer, a mirror or a sketch made leaves the document as it was;
/// a split's, a pattern's and a fused pattern's inverse is one undo.
///
/// *Breaks if* a seat reads through a copy to its body (the result reads
/// the bracket, and the copies are re-pointed), a gesture writes a
/// placement, or a delete is not the gesture's inverse.
#[test]
fn a_feature_picked_on_a_copy_reads_that_copy_alone() {
    let tol = Tol::witness();
    let mut bad = Vec::new();
    let (probe, body, first, _) = two_world_copies(tol);
    let count = gestures_on(&probe, body, first).len();
    for at in 0..count {
        let (mut session, body, first, second) = two_world_copies(tol);
        let (name, op, inverse) = gestures_on(&session, body, first).swap_remove(at);
        let before = world(session.committed_doc());
        let read = session.committed_doc().output(body, 0);
        let out = session.perform(op);
        if let Some(refusal) = out.refusal {
            bad.push(format!("{name}: refused {refusal:?}"));
            continue;
        }
        session.pump();
        let doc = session.committed_doc();
        if writes_a_source(&out.committed) {
            bad.push(format!("{name}: wrote a source: {:?}", out.committed));
        }
        if [first, second].map(|copy| source_of(doc, copy)) != [read; 2] {
            bad.push(format!("{name}: a copy no longer places the bracket"));
        }
        if !viewer::world::is_drawn(doc, second) {
            bad.push(format!("{name}: the second copy is not drawn"));
        }
        // What it made on the copy: a node built on it, or a family
        // repeating it.
        let on_first: Vec<RecipeNodeId> = out
            .minted
            .iter()
            .copied()
            .filter(|&node| {
                viewer::world::built_on_copy(doc, node) == Some((first, body))
                    || source_of(doc, node) == doc.output(first, 0)
            })
            .collect();
        if on_first.is_empty() {
            bad.push(format!("{name}: nothing it made reads the first copy"));
        }
        // Each says it is on a placed copy, with the way back: to the
        // body, or for a family of the copy, to the copy it repeats.
        for node in on_first {
            match viewer::frame::placement_offers(doc, node) {
                (Some(text), offers)
                    if text.contains("placed copy")
                        && offers.iter().any(|(_, op)| {
                            *op == SessionOp::Select(Selection::Node(body))
                                || *op == SessionOp::Select(Selection::Node(first))
                        }) => {}
                other => bad.push(format!("{name}: says no copy, or no way back: {other:?}")),
            }
        }
        if inverse {
            let made = out.made.expect("made");
            let deleted = session.perform(SessionOp::DeleteNode { node: made });
            if let Some(refusal) = deleted.refusal {
                bad.push(format!("{name}: delete refused {refusal:?}"));
                continue;
            }
            if world(session.committed_doc()) != before {
                bad.push(format!("{name}: add then delete is not the identity"));
            }
        } else {
            assert!(session.perform(SessionOp::Undo).refusal.is_none());
            if world(session.committed_doc()) != before {
                bad.push(format!("{name}: one undo does not restore"));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **A feature on a body picked as itself offers the re-point, and
/// writes nothing to its placements** (Ev, PR 6353): both world copies
/// read one variable, the bracket's output — the variable they share.
/// A fillet of the bracket picked as itself makes `F = fillet(bracket)`
/// and writes no placement; `F` offers, as one explicit step, to
/// re-point both placements onto it. Accepted, it is one action: both
/// copies place `F`, the product reading the first copy still reads it,
/// and one undo takes the re-point back alone.
///
/// *Breaks if* the gesture re-points silently, the offer is missing or
/// names other placements, or the re-point is not one action.
#[test]
fn a_feature_on_a_body_picked_as_itself_offers_the_re_point() {
    let tol = Tol::witness();
    let (mut session, body, first, second) = two_world_copies(tol);
    let doc = session.committed_doc();
    let shared = doc.output(body, 0).expect("the bracket's body");
    assert_eq!(
        [first, second].map(|copy| source_of(doc, copy)),
        [Some(shared); 2],
        "the placements share one variable"
    );
    let edge = lateral_edges(&session, body)[0].clone();
    let out = session.perform(SessionOp::AddFillet {
        target: body,
        radius: len(0.001),
        selection: vec![edge],
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert!(!writes_a_source(&out.committed), "{:?}", out.committed);
    let fillet = out.made.expect("the fillet");
    let after = session.committed_doc().clone();
    assert_eq!(
        [first, second].map(|copy| source_of(&after, copy)),
        [Some(shared); 2],
        "no placement is re-pointed unasked"
    );
    let offer = viewer::world::repoint_offer(&after, fillet).expect("the fillet offers");
    assert_eq!(offer.body, shared, "the variable the placements share");
    assert_eq!(offer.placements, [first, second], "both placements");
    let (said, offers) = viewer::frame::placement_offers(&after, fillet);
    assert_eq!(said, None, "a body picked as itself is on no copy");
    let repoint = SessionOp::RepointPlacements {
        body: Operand::Var(shared),
        onto: fillet,
    };
    let [(label, op)] = &offers[..] else {
        panic!("one offer: {offers:?}")
    };
    assert_eq!(*op, repoint, "the offer is the re-point");
    assert!(label.starts_with("Re-point the 2 placements of "), "{label}");

    let steps = session.history().len();
    let done = session.perform(repoint);
    assert!(done.refusal.is_none(), "{:?}", done.refusal);
    assert_eq!(session.history().len(), steps + 1, "one action");
    session.pump();
    let doc = session.committed_doc();
    let filleted = doc.output(fillet, 0);
    assert_eq!(
        [first, second].map(|copy| source_of(doc, copy)),
        [filleted; 2],
        "both copies place the fillet"
    );
    assert_eq!(doc.product(), doc.output(first, 0), "the product reads the copy");
    assert_eq!(
        viewer::world::repoint_offer(doc, fillet),
        None,
        "nothing is left to offer"
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(
        session.committed_doc().bit_eq(&after),
        "one undo takes the re-point back alone"
    );
}

/// **A placement Place makes in the body's own space is a copy** (PR
/// 6580's recommendation): the bracket Placed twice, both copies
/// standing in its own space. The bracket is badged placed, its copies
/// are both placements, each copy's original is the bracket, a pick on
/// a copy seats the copy, and the placement editor's `on` side reads
/// the bracket off a pick on the copy, so a rest of the second copy on
/// the first's top lands. A pattern's family in the body's own space is
/// a copy family of it too.
///
/// *Breaks if* an own-space placement is left out of the copies (no
/// badge, no original) or the editor's `on` side reads the copy itself
/// (the door refuses `NotOnTheCopy`).
#[test]
fn a_place_into_the_bodys_own_space_is_a_copy() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("own-space", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let place = |session: &mut DocSession| {
        let out = session.perform(SessionOp::Place { body: body.into() });
        assert!(out.refusal.is_none(), "{:?}", out.refusal);
        session.pump();
        out.made.expect("a placement")
    };
    let first = place(&mut session);
    let second = place(&mut session);
    let doc = session.committed_doc();
    assert_eq!(doc.node_space(first), doc.node_space(body), "in its own space");
    assert!(viewer::world::is_placed(doc, body), "badged placed");
    assert_eq!(viewer::world::placements_of(doc, body), [first, second]);
    for copy in [first, second] {
        assert_eq!(viewer::world::original_of(doc, copy), body);
        assert_eq!(
            viewer::world::copy_source(doc, copy, 0).map(|source| source.operation),
            Some(body)
        );
        assert_eq!(viewer::world::seat_read(doc, copy, Some(0)).0, copy);
    }
    let row = common::row_of(&session.tree_rows(), body).clone();
    assert!(row.placed, "the tree badges the bracket");

    let op = common::transformed(&mut session, second, &common::slides([0.05, 0.0, 0.0]));
    assert!(session.perform(op).refusal.is_none());
    session.pump();
    let index = plate_index(&session);
    let underside = displayed_face_at(&session, &index, &common::up_at(0.05, 0.0));
    let on_top = displayed_face_at(&session, &index, &common::down_at(0.0, 0.0));
    assert_eq!((underside.node, on_top.node), (second, first), "premise");
    drawn::rest_on(&mut session, underside, on_top);
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    let (lo, _) = drawn::extent(&mut session, second);
    assert!((lo[2] - SIZE[2]).abs() < 1e-12, "the second stands on the first: {lo:?}");
}

/// **Designer A's worked example** (PR 6580): bracket `B`, Placed twice
/// (`C1`, `C2`, in `B`'s space); `C2` rested on `C1`'s top by the
/// editor; a fillet picked on `C1`; then a fillet of `B` picked as
/// itself with its re-point accepted.
/// - The fillet on `C1` is `fillet(C1)` in `B`'s space, said to belong
///   to `C1` with a way to `B`; `C2` stays where it stood.
/// - The fillet on `B` writes no placement and offers the two; accepted,
///   both copies place `F`, `C2`'s mate is re-seated on `F`'s faces
///   (its `on` side reads `F`, its `to` side `F`'s name for `C1`'s top),
///   and `fillet(C1)`'s edge is rebound onto `F`'s name for it: every
///   row well, `C2` where it stood.
/// - Deleting `F` hands both copies, their mates and `fillet(C1)` back
///   to `B`: the document is the one before the fillet of `B`.
#[test]
fn designer_as_worked_example() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("bracket", tol), tol);
    let bracket = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let mut copies = Vec::new();
    for _ in 0..2 {
        let out = session.perform(SessionOp::Place {
            body: bracket.into(),
        });
        assert!(out.refusal.is_none(), "{:?}", out.refusal);
        copies.push(out.made.expect("a placement"));
    }
    let [c1, c2] = copies[..] else { unreachable!() };
    let op = common::transformed(&mut session, c2, &common::slides([0.05, 0.0, 0.0]));
    assert!(session.perform(op).refusal.is_none());
    session.pump();
    let index = plate_index(&session);
    let underside = displayed_face_at(&session, &index, &common::up_at(0.05, 0.0));
    let c1_top = displayed_face_at(&session, &index, &common::down_at(0.0, 0.0));
    drawn::rest_on(&mut session, underside, c1_top);
    let stood = drawn::extent(&mut session, c2);

    // A fillet picked on C1.
    let edges = lateral_edges(&session, c1);
    let out = session.perform(SessionOp::AddFillet {
        target: c1,
        radius: len(0.001),
        selection: vec![edges[0].clone()],
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert!(!writes_a_source(&out.committed));
    let on_c1 = out.made.expect("fillet(C1)");
    let doc = session.committed_doc();
    assert_eq!(doc.node_space(on_c1), doc.node_space(bracket), "in B's space");
    assert_eq!(viewer::world::built_on_copy(doc, on_c1), Some((c1, bracket)));
    let (said, offers) = viewer::frame::placement_offers(doc, on_c1);
    assert!(said.is_some_and(|text| text.contains("placed copy")));
    assert!(
        offers
            .iter()
            .any(|(_, op)| *op == SessionOp::Select(Selection::Node(bracket))),
        "a way to B: {offers:?}"
    );
    assert_eq!(drawn::extent(&mut session, c2), stood, "C2 stands");

    // A fillet of B picked as itself, its re-point accepted.
    let before_f = world(session.committed_doc());
    let out = session.perform(SessionOp::AddFillet {
        target: bracket,
        radius: len(0.001),
        selection: vec![edges[1].clone()],
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert!(!writes_a_source(&out.committed));
    let f = out.made.expect("F");
    let offer = viewer::world::repoint_offer(session.committed_doc(), f).expect("F offers");
    assert_eq!(offer.placements, [c1, c2]);
    let done = session.perform(SessionOp::RepointPlacements {
        body: Operand::Var(offer.body),
        onto: f,
    });
    assert!(done.refusal.is_none(), "{:?}", done.refusal);
    session.pump();
    let doc = session.committed_doc();
    assert_eq!(
        [c1, c2].map(|copy| source_of(doc, copy)),
        [doc.output(f, 0); 2],
        "both copies place F"
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "after the re-point: {row:?}");
    }
    assert_eq!(drawn::extent(&mut session, c2), stood, "C2 stands");

    // Delete F: it continues B.
    let deleted = session.perform(SessionOp::DeleteNode { node: f });
    assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
    session.pump();
    let doc = session.committed_doc();
    assert_eq!(
        [c1, c2].map(|copy| source_of(doc, copy)),
        [doc.output(bracket, 0); 2],
        "both copies place B again"
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "after the delete: {row:?}");
    }
    assert_eq!(drawn::extent(&mut session, c2), stood, "C2 stands");
    assert!(
        world(session.committed_doc()) == before_f,
        "the document is the one before the fillet of B"
    );
}

/// **Delete splices by continuation** (DM6; PR 6280's review probes,
/// re-spelled):
/// - a chain on a copy the product reads, fillet then chamfer of the
///   fillet: deleting the fillet hands the chamfer to the copy, its edge
///   carried back to the copy's name for it, and the product reads the
///   chamfer still (the review's chain row);
/// - a copy of a copy, filleted: deleting the fillet is the identity
///   (M1, `review_copy_of_copy`);
/// - deleting a placement takes the copy of it with it: a placement
///   continues nothing (m1, `review_deleting_a_duplicate_hands_nothing_back`);
/// - a `⊔` left with one operand continues it: the product `⊔` of two
///   copies, one deleted, reads the other.
///
/// *Breaks if* a takeover is inferred from node shape (a placement in
/// its source's space read as a takeover), or a continuation is missed.
#[test]
fn delete_splices_by_continuation() {
    let tol = Tol::witness();

    // The chain.
    let (mut session, _, first, _) = two_world_copies(tol);
    let edges = lateral_edges(&session, first);
    let fillet = common::session_insert(
        &mut session,
        SessionOp::AddFillet {
            target: first,
            radius: len(0.001),
            selection: vec![edges[0].clone()],
        },
    );
    session.pump();
    let carried = pncad::select::all_edges(session.evaluation().expect("landed"), fillet)
        .into_iter()
        .find(|name| {
            matches!(name.path.as_slice(),
                [RoleSeg::From { of, .. }] if *of.name() == edges[1])
        })
        .expect("the fillet carries a second edge of the copy");
    let chamfer = common::session_insert(
        &mut session,
        SessionOp::AddChamfer {
            target: fillet,
            distance: len(0.0005),
            selection: vec![carried],
        },
    );
    session.pump();
    let out = session.perform(SessionOp::DeleteNode { node: fillet });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    let doc = session.committed_doc();
    assert!(doc.node(chamfer).is_some(), "the chamfer is handed over");
    assert_eq!(doc.product(), doc.output(chamfer, 0), "the product reads it");
    let Some(Node::Chamfer { selection, .. }) = doc.node(chamfer) else {
        panic!("a chamfer")
    };
    let select = doc.selection(*selection).expect("a selection");
    assert_eq!(Some(select.body), doc.output(first, 0), "on the copy");
    assert_eq!(select.names, [edges[1].clone()], "by the copy's name");
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }

    // A copy of a copy.
    let (session, _, first, _) = two_world_copies(tol);
    let (doc, again) = viewer::test_support::inserted(
        session.committed_doc(),
        common::world_place(Operand::Node(first)),
        tol,
    );
    let doc = drawn::pinned(doc, again, &drawn::translated([0.0, 0.0, 0.05]));
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let before = world(session.committed_doc());
    let edge = lateral_edges(&session, again)[0].clone();
    let made = common::session_insert(
        &mut session,
        SessionOp::AddFillet {
            target: again,
            radius: len(0.001),
            selection: vec![edge],
        },
    );
    let out = session.perform(SessionOp::DeleteNode { node: made });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert!(
        world(session.committed_doc()) == before,
        "add then delete on a copy of a copy is the identity"
    );

    // A placement deleted takes its copy.
    let mut session = common::inline_without_door(Doc::empty_derived("dup", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let placed = common::session_insert(&mut session, SessionOp::Place { body: body.into() });
    let (doc, world_copy) = viewer::test_support::inserted(
        session.committed_doc(),
        common::world_place(Operand::Node(placed)),
        tol,
    );
    let mut session = common::inline_without_door(doc, tol);
    let affordance = session.delete_affordance(placed);
    assert_eq!(affordance.cascade, [world_copy, placed], "priced");
    let out = session.perform(SessionOp::DeleteNode { node: placed });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert!(
        session.committed_doc().node(world_copy).is_none(),
        "the copy of the deleted placement goes with it"
    );

    // A `⊔` left with one operand.
    let (mut session, _, first, second) = two_world_copies(tol);
    let product = session.perform(SessionOp::SetProduct {
        bodies: vec![Operand::Node(first), Operand::Node(second)],
    });
    assert!(product.refusal.is_none(), "{:?}", product.refusal);
    let assembled = session
        .committed_doc()
        .product()
        .and_then(|var| session.committed_doc().operation_of(var))
        .expect("the product's `⊔`");
    let affordance = session.delete_affordance(second);
    assert_eq!(affordance.cascade, [assembled, second], "the `⊔` collapses");
    let out = session.perform(SessionOp::DeleteNode { node: second });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = session.committed_doc();
    assert_eq!(doc.product(), doc.output(first, 0), "the product reads the first");
}
