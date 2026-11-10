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
//!   continues (`editor_core::continuation::splice`), so adding a feature and
//!   deleting it leaves the document as it was.
//!
//! The delete rows keep PR 6280's review probes, re-spelled to the
//! ruled behaviour.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::drawn;
use common::{displayed_face_at, len, plate_index};
use pncad::document::Bodies;
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
    displayed_face_at(session, &index, &common::along_x(-1.0, y, SIZE[2] / 2.0))
}

/// The gestures a copy is picked for, each named, with whether deleting
/// what it made is its inverse (a pattern's, a split's and a fused
/// pattern's is undo).
fn gestures_on(
    session: &DocSession,
    body: RecipeNodeId,
    copy: RecipeNodeId,
    other: RecipeNodeId,
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
        (
            "combine with the other copy",
            SessionOp::AddBoolean {
                spec: viewer::session::BooleanSpec::Union(vec![copy.into(), other.into()]),
            },
            false,
        ),
    ]
}

/// The bracket Placed twice in its own space, the second slid 0.1 m
/// along y: the session, the bracket and the two copies.
fn two_own_space_copies(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived("own-space", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let mut copies = Vec::new();
    for _ in 0..2 {
        let out = session.perform(SessionOp::Place { body: body.into() });
        assert!(out.refusal.is_none(), "{:?}", out.refusal);
        session.pump();
        copies.push(out.made.expect("a placement"));
    }
    let op = common::transformed(&mut session, copies[1], &common::slides([0.0, 0.1, 0.0]));
    assert!(session.perform(op).refusal.is_none());
    session.pump();
    (session, body, copies[0], copies[1])
}

/// [`two_world_copies`] with the product a `⊔` reading both.
fn two_world_copies_in_a_product(
    tol: Tol,
) -> (DocSession, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let (mut session, body, first, second) = two_world_copies(tol);
    let product = session.perform(SessionOp::SetProduct {
        bodies: vec![Operand::Node(first), Operand::Node(second)],
    });
    assert!(product.refusal.is_none(), "{:?}", product.refusal);
    session.pump();
    (session, body, first, second)
}

/// **Whether the pane says `node` is on the copy `copy` of `body`**, with
/// the way back to `body` ([`viewer::frame::placement_offers`]).
fn says_on_copy(doc: &Doc<ProfileProgram>, node: RecipeNodeId, body: RecipeNodeId) -> bool {
    match viewer::frame::placement_offers(doc, node) {
        (Some(text), offers) => {
            text.contains("placed copy")
                && offers
                    .iter()
                    .any(|(_, op)| *op == SessionOp::Select(Selection::Node(body)))
        }
        (None, _) => false,
    }
}

/// **Every feature gesture picked on the first copy of `make`'s
/// fixture** reads that copy alone: no placement's source written, both
/// copies placing the bracket, the second drawn, and the node the
/// gesture made — the one the app selects after it, so the one the
/// pane speaks of — said to be on the first copy, with the way to the
/// bracket. Deleting what a fillet, a chamfer, a mirror or a sketch
/// made is the identity; a split's, a pattern's, a fused pattern's and
/// a combine's inverse is one undo. Answers the failures.
fn every_gesture_on(
    make: impl Fn(Tol) -> (DocSession, RecipeNodeId, RecipeNodeId, RecipeNodeId),
) -> Vec<String> {
    let tol = Tol::witness();
    let mut bad = Vec::new();
    let (probe, body, first, second) = make(tol);
    let count = gestures_on(&probe, body, first, second).len();
    for at in 0..count {
        let (mut session, body, first, second) = make(tol);
        let (name, op, inverse) = gestures_on(&session, body, first, second).swap_remove(at);
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
        let made = out.made.expect("made");
        // Built on the first copy, or on a family repeating it, or the
        // family itself.
        let repeats_first =
            |copy: RecipeNodeId| copy == first || source_of(doc, copy) == doc.output(first, 0);
        let on_first = viewer::world::built_on_copy(doc, made)
            .is_some_and(|(copy, _)| repeats_first(copy))
            || repeats_first(made);
        if !on_first {
            bad.push(format!("{name}: what it made does not read the first copy"));
        }
        if !says_on_copy(doc, made, body) {
            bad.push(format!(
                "{name}: the pane does not say the made node is on the copy, with the way \
                 to the bracket: {:?}",
                viewer::frame::placement_offers(doc, made)
            ));
        }
        for row in session.tree_rows() {
            if row.status != RowStatus::Ok {
                bad.push(format!("{name}: {row:?}"));
            }
        }
        if inverse {
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
    bad
}

/// **A feature picked on a world copy reads that copy alone** (Ev, PR
/// 6353), every gesture ([`every_gesture_on`]).
///
/// *Breaks if* a seat reads through a copy to its body (the result reads
/// the bracket, and the copies are re-pointed), a gesture writes a
/// placement, the made node does not say it is on the copy (a sketch's
/// profile), or a delete is not the gesture's inverse.
#[test]
fn a_feature_picked_on_a_copy_reads_that_copy_alone() {
    let bad = every_gesture_on(two_world_copies);
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The same over two copies Place made in the bracket's own space
/// (PR 6581's review, R1 and R2).
#[test]
fn a_feature_picked_on_an_own_space_copy_reads_that_copy_alone() {
    let bad = every_gesture_on(two_own_space_copies);
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The same over two world copies the product `⊔` reads: the `⊔` seat
/// is the takeover's, and the delete hands it back (PR 6581's review,
/// R1).
#[test]
fn a_feature_picked_on_a_copy_in_the_product_reads_that_copy_alone() {
    let bad = every_gesture_on(two_world_copies_in_a_product);
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
    assert!(
        label.starts_with("Re-point the 2 placements of "),
        "{label}"
    );

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
    assert_eq!(
        doc.product(),
        doc.output(first, 0),
        "the product reads the copy"
    );
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
    assert_eq!(
        doc.node_space(first),
        doc.node_space(body),
        "in its own space"
    );
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
    assert!(
        (lo[2] - SIZE[2]).abs() < 1e-12,
        "the second stands on the first: {lo:?}"
    );
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
    assert_eq!(
        doc.node_space(on_c1),
        doc.node_space(bracket),
        "in B's space"
    );
    assert_eq!(
        viewer::world::built_on_copy(doc, on_c1),
        Some((c1, bracket))
    );
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
    assert_eq!(
        doc.product(),
        doc.output(chamfer, 0),
        "the product reads it"
    );
    let Some(Node::Chamfer { selection, .. }) = doc.node(chamfer) else {
        panic!("a chamfer")
    };
    let select = doc.selection(*selection).expect("a selection");
    assert_eq!(Some(select.body.read), doc.output(first, 0), "on the copy");
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
    assert_eq!(
        doc.product(),
        doc.output(first, 0),
        "the product reads the first"
    );
}

/// The bracket in a session of its own, landed.
fn bracket(name: &str, tol: Tol) -> (DocSession, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived(name, tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    session.pump();
    (session, body)
}

/// One Place of `body`, landed.
fn place(session: &mut DocSession, body: RecipeNodeId) -> RecipeNodeId {
    let out = session.perform(SessionOp::Place { body: body.into() });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    out.made.expect("a placement")
}

/// `node` inserted by hand into `session`'s document: a fresh landed
/// session and the node.
fn with(session: &DocSession, node: AuthoredNode, tol: Tol) -> (DocSession, RecipeNodeId) {
    let (doc, id) = viewer::test_support::inserted(session.committed_doc(), node, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    (session, id)
}

/// **The offer's button does what the offer says** (PR 6581's review,
/// R1 m1 and R2 M1): the bracket Placed twice, then a body built on the
/// bracket and one of its own copies — by hand, `U = B ∪ C1`, and through
/// the combine gesture with `C1` slid aside. The offer leaves `C1` out,
/// since `U` reads it; the button re-points `C2` alone, and lands.
///
/// *Breaks if* the action re-derives a placement list other than the
/// offer's (the button is refused `WouldCycle` on `C1`).
#[test]
fn the_offer_and_its_button_re_point_the_same_placements() {
    let tol = Tol::witness();
    for gesture in [false, true] {
        let (mut session, body) = bracket("offer-cycle", tol);
        let c1 = place(&mut session, body);
        let c2 = place(&mut session, body);
        let (mut session, u) = if gesture {
            let op = common::transformed(&mut session, c1, &common::slides([0.01, 0.0, 0.0]));
            assert!(session.perform(op).refusal.is_none());
            session.pump();
            let u = common::session_insert(
                &mut session,
                SessionOp::AddBoolean {
                    spec: viewer::session::BooleanSpec::Union(vec![body.into(), c1.into()]),
                },
            );
            session.pump();
            (session, u)
        } else {
            with(
                &session,
                Node::Union {
                    members: pncad::document::Bodies::Spelled(vec![
                        Operand::Node(body).into(),
                        Operand::Node(c1).into(),
                    ]),
                },
                tol,
            )
        };
        let doc = session.committed_doc().clone();
        let offer = viewer::world::repoint_offer(&doc, u).expect("U offers");
        assert_eq!(offer.placements, [c2], "gesture={gesture}: C1 left out");
        let (_, offers) = viewer::frame::placement_offers(&doc, u);
        let (label, op) = offers
            .into_iter()
            .find(|(_, op)| matches!(op, SessionOp::RepointPlacements { .. }))
            .expect("the button");
        assert!(label.starts_with("Re-point the 1 placement "), "{label}");
        let out = session.perform(op);
        assert!(
            out.refusal.is_none(),
            "gesture={gesture}: {:?}",
            out.refusal
        );
        let doc = session.committed_doc();
        assert_eq!(doc.copied_read(c2), doc.output(u, 0), "C2 re-pointed");
        assert_eq!(doc.copied_read(c1), doc.output(body, 0), "C1 left");
    }
}

/// **Delete is the inverse two features deep** (PR 6581's review, R2
/// M2): the bracket `B` and its world copy `C`; `G = fillet(C)`, then
/// `K = chamfer(G)` on an edge `G` carries from `C`; then `F = fillet(B)`
/// with its re-point accepted, which rebinds `K`'s edge inside `G`'s
/// name. Deleting `F` carries that name back too: every row is well,
/// and the document is the one before `F`.
///
/// *Breaks if* the splice's rebinds stop at selections whose body is a
/// handed copy, missing names that carry a copy's entity in from deeper
/// (`resolve::derivation_reads`).
#[test]
fn a_chain_on_a_copy_survives_the_re_point_and_its_delete() {
    let tol = Tol::witness();
    let (session, body) = bracket("deep", tol);
    let (doc, copy) = common::world_copy(session.committed_doc(), body, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let edges = lateral_edges(&session, copy);
    let g = common::session_insert(
        &mut session,
        SessionOp::AddFillet {
            target: copy,
            radius: len(0.001),
            selection: vec![edges[0].clone()],
        },
    );
    session.pump();
    let carried = pncad::select::all_edges(session.evaluation().expect("landed"), g)
        .into_iter()
        .find(|name| {
            matches!(name.path.as_slice(),
                [RoleSeg::From { of, .. }] if *of.name() == edges[1])
        })
        .expect("G carries a second edge of the copy");
    let k = common::session_insert(
        &mut session,
        SessionOp::AddChamfer {
            target: g,
            distance: len(0.0005),
            selection: vec![carried],
        },
    );
    session.pump();
    let ok = |session: &DocSession, when: &str| {
        for row in session.tree_rows() {
            assert_eq!(row.status, RowStatus::Ok, "{when}: {row:?}");
        }
    };
    ok(&session, "premise");
    let before = world(session.committed_doc());
    let third = lateral_edges(&session, body)[2].clone();
    let f = common::session_insert(
        &mut session,
        SessionOp::AddFillet {
            target: body,
            radius: len(0.001),
            selection: vec![third],
        },
    );
    session.pump();
    let offer = viewer::world::repoint_offer(session.committed_doc(), f).expect("F offers");
    let out = session.perform(SessionOp::RepointPlacements {
        body: Operand::Var(offer.body),
        onto: f,
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    ok(&session, "after the re-point");
    let out = session.perform(SessionOp::DeleteNode { node: f });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    assert!(session.committed_doc().node(k).is_some(), "K survives");
    ok(&session, "after the delete");
    let after = world(session.committed_doc());
    assert!(
        after.0 == before.0,
        "the delete of F is the identity on the nodes"
    );
}

/// The family of two the pattern gesture makes of the bracket picked
/// as itself, and a plain Place of the bracket: the session, the family
/// and the placement.
fn family_and_copy(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let (mut session, body) = bracket("member", tol);
    let out = session.perform(SessionOp::AddPattern {
        input: body.into(),
        count: 2,
        rule: PatternRuleSpec::Linear {
            direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
            spacing: len(2.0 * SIZE[0]),
        },
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    let family = out.made.expect("the family");
    let copy = place(&mut session, body);
    (session, body, family, copy)
}

/// Member 0 of `family`, read by index.
fn member0(doc: &Doc<ProfileProgram>, family: RecipeNodeId) -> pncad::document::BodyRead<Formula> {
    let var = doc.output(family, 0).expect("the family's output");
    pncad::document::BodyRead::indexed(Operand::Var(var), vec![Formula::count(0)])
}

/// **A continuation is a body read whole** (PR 6581's review, R2 M3):
/// - `∪[xs[0], C]` with `C` deleted keeps `xs[0]` as its one operand, so
///   a world copy reading the union still copies one body, not the
///   family;
/// - `xs[0] − C` with the tool `C` deleted continues member 0: a copy of
///   the cut, whose source is written whole, refuses typed
///   (`Refusal::Unspliced`, naming the copy), and nothing is deleted;
/// - a fillet over a family deleted hands a `∪` reading its member 0
///   the family's member 0, and refuses, typed, while a copy of its
///   member 0 stands: a placement's source is written whole.
///
/// *Breaks if* a delete hands a reader of one member the whole family.
#[test]
fn a_continuation_is_a_body_read_whole() {
    let tol = Tol::witness();
    let (session, _, family, copy) = family_and_copy(tol);
    let m0 = member0(session.committed_doc(), family);
    let (session, u) = with(
        &session,
        Node::Union {
            members: pncad::document::Bodies::Spelled(vec![m0, Operand::Node(copy).into()]),
        },
        tol,
    );
    let (mut session, reader) = with(&session, common::world_place(Operand::Node(u)), tol);
    let out = session.perform(SessionOp::DeleteNode { node: copy });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    let doc = session.committed_doc();
    assert_eq!(
        source_of(doc, reader),
        doc.output(u, 0),
        "the copy reads the union still"
    );
    let Some(Node::Union {
        members: pncad::document::Bodies::Spelled(members),
    }) = doc.node(u)
    else {
        panic!("the union stays")
    };
    assert!(
        matches!(&members[..], [one] if one.is_indexed() && Some(one.read) == doc.output(family, 0)),
        "with member 0 its one operand: {members:?}"
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }

    // A cut of member 0 whose tool goes continues member 0: a copy of
    // the cut, whose source the slot door writes whole, refuses typed,
    // naming the copy; nothing is deleted.
    let (session, _, family, copy) = family_and_copy(tol);
    let m0 = member0(session.committed_doc(), family);
    let (session, cut) = with(
        &session,
        Node::Subtract {
            from: m0,
            tool: Operand::Node(copy).into(),
        },
        tol,
    );
    let (mut session, reader) = with(&session, common::world_place(Operand::Node(cut)), tol);
    let before = session.committed_doc().clone();
    let refusal = session.delete_refusal(copy);
    assert!(
        matches!(&refusal, Some(viewer::Refusal::Unspliced { node, .. }) if node.id() == reader),
        "{refusal:?}"
    );
    let out = session.perform(SessionOp::DeleteNode { node: copy });
    assert!(
        matches!(&out.refusal, Some(viewer::Refusal::Unspliced { node, .. }) if node.id() == reader),
        "the op refuses as the button says: {:?}",
        out.refusal
    );
    assert!(session.committed_doc().bit_eq(&before), "nothing deleted");

    // A fillet over the family, read by member: a list operand is handed
    // the same member of the family; a placement's source, which the slot
    // door writes whole, refuses typed.
    let (mut session, body, family, copy) = family_and_copy(tol);
    let edge = lateral_edges(&session, body)[..1].to_vec();
    let fillet = common::session_insert(
        &mut session,
        SessionOp::AddFillet {
            target: family,
            radius: len(0.001),
            selection: edge,
        },
    );
    session.pump();
    let f0 = member0(session.committed_doc(), fillet);
    let (session, u) = with(
        &session,
        Node::Union {
            members: pncad::document::Bodies::Spelled(vec![f0.clone(), Operand::Node(copy).into()]),
        },
        tol,
    );
    let (mut session, member_copy) = with(
        &session,
        Node::place(
            f0,
            Operand::Var(VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
        tol,
    );
    let refusal = session.delete_refusal(fillet);
    assert!(
        matches!(&refusal, Some(viewer::Refusal::Unspliced { node, .. }) if node.id() == member_copy),
        "the copy of a member refuses: {refusal:?}"
    );
    let out = session.perform(SessionOp::DeleteNode { node: member_copy });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let out = session.perform(SessionOp::DeleteNode { node: fillet });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = session.committed_doc();
    let Some(Node::Union {
        members: pncad::document::Bodies::Spelled(members),
    }) = doc.node(u)
    else {
        panic!("the union stays")
    };
    assert!(
        matches!(&members[..], [one, _] if one.is_indexed() && Some(one.read) == doc.output(family, 0)),
        "handed member 0 of the family: {members:?}"
    );
}

/// **The pane's "on this copy" reaches down a chain and through a
/// member copy, and "Go to the original body" reaches the body** (PR
/// 6581's review, R1 n6, R2 m5):
/// - `chamfer(fillet(C))` says it is on `C`, with the way to `B`;
/// - a copy of member 0 of `B`'s pattern says it is a placed copy of `B`,
///   and a fillet on it says it is on that copy, the way back to `B`;
/// - a copy of a copy is a placed copy of `B`, not of the copy it reads.
#[test]
fn on_this_copy_reaches_down_a_chain_and_to_the_body() {
    let tol = Tol::witness();
    let (mut session, body, first, _) = two_world_copies(tol);
    let edges = lateral_edges(&session, first);
    let g = common::session_insert(
        &mut session,
        SessionOp::AddFillet {
            target: first,
            radius: len(0.001),
            selection: vec![edges[0].clone()],
        },
    );
    session.pump();
    let carried = pncad::select::all_edges(session.evaluation().expect("landed"), g)
        .into_iter()
        .find(|name| matches!(name.path.as_slice(), [RoleSeg::From { .. }]))
        .expect("a carried edge");
    let k = common::session_insert(
        &mut session,
        SessionOp::AddChamfer {
            target: g,
            distance: len(0.0005),
            selection: vec![carried],
        },
    );
    let doc = session.committed_doc();
    assert_eq!(viewer::world::built_on_copy(doc, k), Some((first, body)));
    assert!(
        says_on_copy(doc, k, body),
        "{:?}",
        viewer::frame::placement_offers(doc, k)
    );

    let (session, body, family, _) = family_and_copy(tol);
    let m0 = member0(session.committed_doc(), family);
    let (mut session, member) = with(
        &session,
        Node::place(
            m0,
            Operand::Var(VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
        tol,
    );
    let doc = session.committed_doc();
    assert_eq!(
        viewer::world::original_of(doc, member),
        body,
        "through the family"
    );
    assert!(
        says_on_copy(doc, member, body),
        "{:?}",
        viewer::frame::placement_offers(doc, member)
    );
    let edge = pncad::select::all_edges(session.evaluation().expect("landed"), member)[0].clone();
    let on_member = common::session_insert(
        &mut session,
        SessionOp::AddFillet {
            target: member,
            radius: len(0.001),
            selection: vec![edge],
        },
    );
    let doc = session.committed_doc();
    assert!(says_on_copy(doc, on_member, body));

    let (session, body, first, _) = two_world_copies(tol);
    let (session, again) = with(&session, common::world_place(Operand::Node(first)), tol);
    let doc = session.committed_doc();
    assert_eq!(viewer::world::original_of(doc, again), body);
    assert!(says_on_copy(doc, again, body));
}

// ---- PR 6581's re-check (lane `placing-recheck-6581`), pinned to the fixed behaviour ----

/// Whether `name` is `target` or carries it in at any depth.
fn carries(name: &StableName, target: &StableName) -> bool {
    name == target
        || name.path.iter().any(|seg| match seg {
            RoleSeg::From { of, .. } => carries(of.name(), target),
            _ => false,
        })
}

fn edge_carrying(session: &DocSession, on: RecipeNodeId, target: &StableName) -> StableName {
    pncad::select::all_edges(session.evaluation().expect("landed"), on)
        .into_iter()
        .find(|name| carries(name, target))
        .unwrap_or_else(|| panic!("{on:?} carries {target:?}"))
}

fn face_carrying(session: &DocSession, on: RecipeNodeId, target: &StableName) -> StableName {
    pncad::select::all_faces(session.evaluation().expect("landed"), on)
        .into_iter()
        .find(|name| carries(name, target))
        .unwrap_or_else(|| panic!("{on:?} carries face {target:?}"))
}

fn top_name(body: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: body,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    }
}

fn rows_ok(session: &DocSession, when: &str) -> Vec<String> {
    session
        .tree_rows()
        .into_iter()
        .filter(|row| row.status != RowStatus::Ok)
        .map(|row| format!("{when}: {row:?}"))
        .collect()
}

/// fillet(C) → chamfer → sketch on its top → fillet of what the sketch
/// made, on the copy `copy` of `body`: answers the made nodes.
fn mixed_chain(
    session: &mut DocSession,
    body: RecipeNodeId,
    copy: RecipeNodeId,
) -> Vec<RecipeNodeId> {
    let edges = lateral_edges(session, body);
    let e0 = edge_carrying(session, copy, &edges[0]);
    let g = common::session_insert(
        session,
        SessionOp::AddFillet {
            target: copy,
            radius: len(0.001),
            selection: vec![e0],
        },
    );
    session.pump();
    let e1 = edge_carrying(session, g, &edges[1]);
    let k = common::session_insert(
        session,
        SessionOp::AddChamfer {
            target: g,
            distance: len(0.0005),
            selection: vec![e1],
        },
    );
    session.pump();
    let face = face_carrying(session, k, &top_name(body));
    let out = session.perform(SessionOp::SketchOnFace {
        face: FaceSelection {
            name: face,
            node: k,
            body: 0,
        },
        loops: vec![common::rectangle_loop([0.0, 0.0], 0.004, 0.004)],
        distance: len(0.003),
        spin: Formula::zero(Dimension::Angle),
    });
    assert!(out.refusal.is_none(), "sketch: {:?}", out.refusal);
    session.pump();
    let made: Vec<RecipeNodeId> = out.minted.clone();
    let fused = *made
        .iter()
        .find(|&&n| {
            matches!(
                session.committed_doc().node(n),
                Some(Node::Union { .. } | Node::Assemble { .. })
            )
        })
        .unwrap_or_else(|| panic!("the sketch's union among {made:?}"));
    let e2 = edge_carrying(session, fused, &edges[2]);
    let h = common::session_insert(
        session,
        SessionOp::AddFillet {
            target: fused,
            radius: len(0.0005),
            selection: vec![e2],
        },
    );
    session.pump();
    vec![g, k, fused, h]
}

fn fillet_and_repoint(session: &mut DocSession, body: RecipeNodeId) -> RecipeNodeId {
    let e3 = lateral_edges(session, body)[3].clone();
    let f = common::session_insert(
        session,
        SessionOp::AddFillet {
            target: body,
            radius: len(0.001),
            selection: vec![e3],
        },
    );
    session.pump();
    let offer = viewer::world::repoint_offer(session.committed_doc(), f).expect("F offers");
    let out = session.perform(SessionOp::RepointPlacements {
        body: Operand::Var(offer.body),
        onto: f,
    });
    assert!(out.refusal.is_none(), "re-point: {:?}", out.refusal);
    session.pump();
    f
}

/// **Delete is the inverse four features deep** (re-check, claim 2): a mixed chain (fillet, chamfer, sketch, fillet) on
/// the copy BEFORE the re-point; F = fillet(B), re-point, delete F: the
/// document (nodes, product, drawn) is the one before F, rows well.
#[test]
fn a_mixed_chain_before_the_repoint_survives_its_delete() {
    let tol = Tol::witness();
    let (session, body) = bracket("rc-deep-before", tol);
    let (doc, copy) = common::world_copy(session.committed_doc(), body, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let chain = mixed_chain(&mut session, body, copy);
    let mut bad = rows_ok(&session, "premise");
    let before = world(session.committed_doc());
    let f = fillet_and_repoint(&mut session, body);
    bad.extend(rows_ok(&session, "after the re-point"));
    let out = session.perform(SessionOp::DeleteNode { node: f });
    assert!(out.refusal.is_none(), "delete: {:?}", out.refusal);
    session.pump();
    bad.extend(rows_ok(&session, "after the delete"));
    for n in &chain {
        assert!(session.committed_doc().node(*n).is_some(), "{n:?} survives");
    }
    let after = world(session.committed_doc());
    assert!(bad.is_empty(), "{bad:#?}");
    assert!(after.0 == before.0, "nodes differ after the delete");
    assert_eq!(after.1, before.1, "product");
    assert_eq!(after.2, before.2, "drawn");
}

/// **The same chain built after the re-point** (re-check): built on the copy AFTER the re-point (its
/// names now carry F's), then F deleted: the chain survives and every
/// row is well.
#[test]
fn a_mixed_chain_after_the_repoint_survives_its_delete() {
    let tol = Tol::witness();
    let (session, body) = bracket("rc-deep-after", tol);
    let (doc, copy) = common::world_copy(session.committed_doc(), body, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let f = fillet_and_repoint(&mut session, body);
    let chain = mixed_chain(&mut session, body, copy);
    let mut bad = rows_ok(&session, "premise");
    let out = session.perform(SessionOp::DeleteNode { node: f });
    assert!(out.refusal.is_none(), "delete: {:?}", out.refusal);
    session.pump();
    for n in &chain {
        assert!(session.committed_doc().node(*n).is_some(), "{n:?} survives");
    }
    bad.extend(rows_ok(&session, "after the delete"));
    assert!(bad.is_empty(), "{bad:#?}");
}

/// **A cut of a member nothing reads goes with its tool** (re-check m1):
/// `xs[0] − C`, nothing reading it. The cut continues member 0 and has
/// no reader to hand it to, so the delete lands.
#[test]
fn a_cut_of_a_member_nobody_reads_deletes_its_tool() {
    let tol = Tol::witness();
    let (session, _, family, copy) = family_and_copy(tol);
    let m0 = member0(session.committed_doc(), family);
    let (mut session, cut) = with(
        &session,
        Node::Subtract {
            from: m0,
            tool: Operand::Node(copy).into(),
        },
        tol,
    );
    let out = session.perform(SessionOp::DeleteNode { node: copy });
    assert!(
        out.refusal.is_none(),
        "the cut {cut:?} has no reader: {}",
        out.refusal.map(|r| r.to_string()).unwrap_or_default()
    );
    assert!(
        session.committed_doc().node(cut).is_none(),
        "the cut goes with its tool"
    );
}

/// **A cut of a member read by a list hands it the member** (re-check
/// m1): `xs[0] − C` read only as a member of `∪[cut, xs[1]]`. A list
/// keeps indices (`SetMembers`), so the union is handed `xs[0]`, by the
/// rule a fillet over a family's member reader takes.
#[test]
fn a_cut_of_a_member_read_by_a_list_is_handed_the_member() {
    let tol = Tol::witness();
    let (session, _, family, copy) = family_and_copy(tol);
    let doc = session.committed_doc();
    let (m0, m1) = (member0(doc, family), member_k(doc, family, 1));
    let (session, cut) = with(
        &session,
        Node::Subtract {
            from: m0,
            tool: Operand::Node(copy).into(),
        },
        tol,
    );
    let (mut session, u) = with(
        &session,
        Node::Union {
            members: Bodies::Spelled(vec![Operand::Node(cut).into(), m1]),
        },
        tol,
    );
    let out = session.perform(SessionOp::DeleteNode { node: copy });
    assert!(
        out.refusal.is_none(),
        "the union {u:?} is a list: {}",
        out.refusal.map(|r| r.to_string()).unwrap_or_default()
    );
    let doc = session.committed_doc();
    assert!(doc.node(cut).is_none(), "the cut goes, continuing member 0");
    let Some(Node::Union {
        members: pncad::document::Bodies::Spelled(members),
    }) = doc.node(u)
    else {
        panic!("the union stays")
    };
    let family_out = doc.output(family, 0);
    assert!(
        matches!(&members[..], [a, b] if a.is_indexed() && b.is_indexed()
            && Some(a.read) == family_out && Some(b.read) == family_out),
        "the union is handed member 0 beside member 1: {members:?}"
    );
}

/// **No widening** (re-check): `∩[xs[0], C]` read by a world copy; deleting
/// C keeps member 0, the copy reads the ∩, never the family.
#[test]
fn an_intersection_left_with_a_member_keeps_it() {
    let tol = Tol::witness();
    let (session, _, family, copy) = family_and_copy(tol);
    let m0 = member0(session.committed_doc(), family);
    let (session, i) = with(
        &session,
        Node::Intersect {
            members: Bodies::Spelled(vec![m0, Operand::Node(copy).into()]),
        },
        tol,
    );
    let (mut session, reader) = with(&session, common::world_place(Operand::Node(i)), tol);
    let out = session.perform(SessionOp::DeleteNode { node: copy });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = session.committed_doc();
    assert_eq!(doc.copied_read(reader), doc.output(i, 0), "reads the ∩");
}

/// **On this copy at depth** (re-check, claim 4): a copy of a copy, four features deep with a sketch, says it
/// is on the copy and goes to B; a copy of a member copy reaches B.
#[test]
fn on_this_copy_at_depth_and_through_member_copies() {
    let tol = Tol::witness();
    let (session, body) = bracket("rc-depth", tol);
    let (doc, c1) = common::world_copy(session.committed_doc(), body, tol);
    let (doc, c2) =
        viewer::test_support::inserted(&doc, common::world_place(Operand::Node(c1)), tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let chain = mixed_chain(&mut session, body, c2);
    let doc = session.committed_doc();
    let mut bad = Vec::new();
    for &n in &chain {
        let (said, offers) = viewer::frame::placement_offers(doc, n);
        let ok = said.as_deref().is_some_and(|s| s.contains("placed copy"))
            && offers
                .iter()
                .any(|(_, op)| *op == SessionOp::Select(Selection::Node(body)));
        if !ok || viewer::world::built_on_copy(doc, n).map(|(c, _)| c) != Some(c2) {
            bad.push(format!(
                "{n:?}: {said:?} {offers:?} {:?}",
                viewer::world::built_on_copy(doc, n)
            ));
        }
    }
    for id in doc.ids() {
        if matches!(doc.node(id), Some(Node::Profile(_))) {
            let (said, _) = viewer::frame::placement_offers(doc, id);
            if !said.as_deref().is_some_and(|s| s.contains("placed copy")) {
                bad.push(format!("profile {id:?}: {said:?}"));
            }
        }
    }

    let (session, body, family, _) = family_and_copy(tol);
    let m1 = member_k(session.committed_doc(), family, 1);
    let (session, mc) = with(
        &session,
        Node::place(
            m1,
            Operand::Var(VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
        tol,
    );
    let (session, mcc) = with(&session, common::world_place(Operand::Node(mc)), tol);
    let doc = session.committed_doc();
    if viewer::world::original_of(doc, mcc) != body {
        bad.push(format!(
            "copy of a member copy: original {:?}",
            viewer::world::original_of(doc, mcc)
        ));
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// **The offer leaves out a cycle through a mate** (re-check, claim 1). C1 is mated onto C2; U = B ∪ C1 reads
/// C2 through C1's pose. C3 is free. The offer must leave C1 and C2 out,
/// and its button must land.
#[test]
fn the_offer_leaves_out_a_placement_read_through_a_mate() {
    let tol = Tol::witness();
    let (mut session, body) = bracket("rc-mate-cycle", tol);
    let place = |session: &mut DocSession| {
        let out = session.perform(SessionOp::Place { body: body.into() });
        assert!(out.refusal.is_none(), "{:?}", out.refusal);
        session.pump();
        out.made.expect("a copy")
    };
    let c1 = place(&mut session);
    let c2 = place(&mut session);
    let c3 = place(&mut session);
    let bottom = StableName {
        kind: EntityKind::Face,
        node: body,
        path: vec![RoleSeg::Cap(CapEnd::Start)],
    };
    common::commit_mate(
        &mut session,
        SessionOp::AddMate {
            placement: c1,
            mate: Box::new(crate::fixture::rest(
                Operand::select(Operand::Node(body), vec![bottom]),
                Operand::select(Operand::Node(c2), vec![top_name(body)]),
            )),
            values: Vec::new(),
            into_target_space: false,
        },
    );
    let (mut session, u) = with(
        &session,
        Node::Union {
            members: Bodies::Spelled(vec![Operand::Node(body).into(), Operand::Node(c1).into()]),
        },
        tol,
    );
    let doc = session.committed_doc().clone();
    let offer = viewer::world::repoint_offer(&doc, u).expect("U offers");
    assert_eq!(offer.placements, [c3], "C1 and C2 left out");
    let (_, offers) = viewer::frame::placement_offers(&doc, u);
    let (_, op) = offers
        .into_iter()
        .find(|(_, op)| matches!(op, SessionOp::RepointPlacements { .. }))
        .expect("the button");
    let out = session.perform(op);
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = session.committed_doc();
    assert_eq!(doc.copied_read(c3), doc.output(u, 0));
}

/// **A split of a body only placements read makes `above ⊔ below` and
/// offers the re-point** (re-check m2; Ev's 6353 ruling, as accepted):
/// the bracket's world copy is its only reader. The split writes the `⊔`
/// of its halves — the new body, as a fillet makes `fillet(B)` — writes
/// no placement, and the `⊔` offers to re-point the copy onto it.
#[test]
fn a_split_of_a_body_only_placements_read_offers_the_re_point() {
    let tol = Tol::witness();
    let (session, body) = bracket("rc-split", tol);
    let (doc, copy) = common::world_copy(session.committed_doc(), body, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let plane = common::cut_across(&session, body, SIZE[2] / 2.0);
    let out = session.perform(SessionOp::AddSplit {
        target: body.into(),
        plane,
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert!(!writes_a_source(&out.committed), "no placement is written");
    session.pump();
    let doc = session.committed_doc();
    let both = *out
        .minted
        .iter()
        .find(|&&n| matches!(doc.node(n), Some(Node::Assemble { .. })))
        .expect("above ⊔ below");
    let offer = viewer::world::repoint_offer(doc, both).expect("the ⊔ offers");
    assert_eq!(offer.placements, [copy], "the copy");
    assert_eq!(Some(offer.body), doc.output(body, 0));
}

/// **A feature on a mirror of the copy survives the re-point and its
/// delete** (re-check m3): the forward re-point rebinds down `names::verbatim_edge`
/// chains (`take_over::chains`: a copy, a mirror, a split of the copy);
/// the splice's `rebinds` closes over `doc.placements()` alone. A mirror
/// of the copy C (`C ∪ image`), and a fillet on that union picking an
/// edge the IMAGE carries; then F = fillet(B), re-point, delete F: the
/// document must be the one before F, every row well. Red at head
/// `4103fdbc04` AFTER THE RE-POINT (the forward door leaves G stale);
/// the delete itself restores the nodes.
#[test]
#[ignore = "work/doors/a-takeover-leaves-a-feature-on-a-mirror-of-the-copy-stale.md"]
fn a_feature_on_a_mirror_of_the_copy_survives_the_repoint_and_its_delete() {
    let tol = Tol::witness();
    let (session, body) = bracket("rc-mirror-chain", tol);
    let (doc, copy) = common::world_copy(session.committed_doc(), body, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let index = common::plate_index(&session);
    let plane =
        common::displayed_face_at(&session, &index, &common::along_x(-1.0, 0.0, SIZE[2] / 2.0));
    assert_eq!(plane.node, copy, "the wall is picked on the copy");
    let out = session.perform(SessionOp::AddMirror {
        body: copy.into(),
        plane,
    });
    assert!(out.refusal.is_none(), "mirror: {:?}", out.refusal);
    session.pump();
    let image = out.made.expect("the mirror");
    let doc = session.committed_doc();
    let image_out = doc.output(image, 0).expect("the image");
    let union = *out
        .minted
        .iter()
        .find(|&&n| matches!(doc.node(n), Some(Node::Union { .. })))
        .expect("C ∪ image");
    // An edge F (which fillets lateral 3) leaves alone: lateral 0's image.
    let keep = lateral_edges(&session, body)[0].clone();
    let edge = pncad::select::all_edges(session.evaluation().expect("landed"), union)
        .into_iter()
        .find(|name| {
            matches!(name.path.as_slice(),
                [RoleSeg::From { read, of }] if *read == image_out
                    && carries(of.name(), &keep))
        })
        .expect("an image lateral edge on the union");
    let g = common::session_insert(
        &mut session,
        SessionOp::AddFillet {
            target: union,
            radius: len(0.0005),
            selection: vec![edge],
        },
    );
    session.pump();
    let mut bad = rows_ok(&session, "premise");
    let before = world(session.committed_doc());
    let f = fillet_and_repoint(&mut session, body);
    bad.extend(rows_ok(&session, "after the re-point"));
    let out = session.perform(SessionOp::DeleteNode { node: f });
    assert!(out.refusal.is_none(), "delete: {:?}", out.refusal);
    session.pump();
    assert!(session.committed_doc().node(g).is_some(), "G survives");
    bad.extend(rows_ok(&session, "after the delete"));
    let after = world(session.committed_doc());
    assert!(
        after.0 == before.0,
        "nodes differ after the delete; rows: {bad:#?}"
    );
    assert!(bad.is_empty(), "{bad:#?}");
}

/// Member `k` of `family`, read by index.
fn member_k(
    doc: &Doc<ProfileProgram>,
    family: RecipeNodeId,
    k: i64,
) -> pncad::document::BodyRead<Formula> {
    let var = doc.output(family, 0).expect("the family's output");
    pncad::document::BodyRead::indexed(Operand::Var(var), vec![Formula::count(k)])
}
