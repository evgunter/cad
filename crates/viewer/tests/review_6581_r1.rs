//! Review probes for PR 6581 (lane `placing-dual-6581-r1`). Each row
//! prints what it found and asserts the behaviour the PR claims, so a
//! red row is a finding.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::drawn;
use common::{displayed_face_at, len, plate_index};
use pncad::document::{
    AuthoredNode, Dimension, Doc, DocEdit, Formula, Operand, OperandSlot, ProfileProgram,
    RecipeNodeId, SlotId, VarId,
};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
use viewer::session::{BooleanSpec, DocSession, FaceSelection, PatternRuleSpec, SessionOp};

const SIZE: [f64; 3] = [0.02, 0.02, 0.01];

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

fn lateral_edges(session: &DocSession, node: RecipeNodeId) -> Vec<StableName> {
    pncad::select::all_edges(session.evaluation().expect("landed"), node)
        .into_iter()
        .filter(|name| matches!(name.path.as_slice(), [RoleSeg::LateralEdge(_)]))
        .collect()
}

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

/// The bracket Placed twice in its own space, the second slid 0.1 m
/// along y: the session, the bracket, the two copies.
fn two_own_space_copies(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived("r1-own", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let mut copies = Vec::new();
    for _ in 0..2 {
        let out = session.perform(SessionOp::Place { body: body.into() });
        assert!(out.refusal.is_none(), "{:?}", out.refusal);
        session.pump();
        copies.push(out.made.expect("a placement"));
    }
    let [first, second] = copies[..] else {
        unreachable!()
    };
    let op = common::transformed(&mut session, second, &common::slides([0.0, 0.1, 0.0]));
    assert!(session.perform(op).refusal.is_none());
    session.pump();
    (session, body, first, second)
}

/// The bracket, two world copies of it, the product `⊔` reading both.
fn two_world_copies_both_in_product(
    tol: Tol,
) -> (DocSession, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived("r1-world", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    let doc = session.committed_doc().clone();
    let (doc, first) = common::world_copy(&doc, body, tol);
    let (doc, second) =
        viewer::test_support::inserted(&doc, common::world_place(Operand::Node(body)), tol);
    let doc = drawn::pinned(doc, second, &drawn::translated([0.0, 0.1, 0.0]));
    let mut session = common::inline_without_door(doc, tol);
    let product = session.perform(SessionOp::SetProduct {
        bodies: vec![Operand::Node(first), Operand::Node(second)],
    });
    assert!(product.refusal.is_none(), "{:?}", product.refusal);
    session.pump();
    (session, body, first, second)
}

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
    let index = plate_index(session);
    let wall = displayed_face_at(session, &index, &common::along_x(-1.0, 0.0, SIZE[2] / 2.0));
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
                plane: wall,
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
                spec: BooleanSpec::Union(vec![copy.into(), other.into()]),
            },
            false,
        ),
    ]
}

/// The main claim-1 loop over a fixture; returns the failures.
fn every_gesture(
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
            bad.push(format!("{name}: wrote a source"));
        }
        if [first, second].map(|copy| doc.copied_read(copy)) != [read; 2] {
            bad.push(format!("{name}: a copy no longer places the bracket"));
        }
        let made = out.made.expect("made");
        let on_first = viewer::world::built_on_copy(doc, made).map(|(c, _)| c) == Some(first)
            || doc.copied_read(made) == doc.output(first, 0);
        let (said, _) = viewer::frame::placement_offers(doc, made);
        eprintln!(
            "{name}: made {made:?} on_first={on_first} said={said:?} committed={}",
            out.committed.len()
        );
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

/// Claim 1 over own-space (Place-made) copies, with combine and Place
/// added to the gesture list.
#[test]
fn r1_every_gesture_on_an_own_space_copy() {
    let bad = every_gesture(two_own_space_copies);
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// Claim 1 over world copies both read by the product `⊔`, with combine
/// and Place added: a `⊔` seat is re-pointed, and the delete hands it
/// back.
#[test]
fn r1_every_gesture_on_a_world_copy_in_a_product_assembly() {
    let bad = every_gesture(two_world_copies_both_in_product);
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// Claim 2: the offer leaves out a placement the body reads (a cycle),
/// but the op it sends re-points every placement of the body. Does the
/// offered button work?
#[test]
fn r1_the_offer_and_the_action_agree_on_a_cycle() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("r1-cycle", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    let doc = session.committed_doc().clone();
    let (doc, first) = common::world_copy(&doc, body, tol);
    let (doc, second) =
        viewer::test_support::inserted(&doc, common::world_place(Operand::Node(body)), tol);
    let doc = drawn::pinned(doc, second, &drawn::translated([0.0, 0.1, 0.0]));
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    // The bracket picked as itself, unioned with its own first copy.
    let out = session.perform(SessionOp::AddBoolean {
        spec: BooleanSpec::Union(vec![body.into(), first.into()]),
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    let union = out.made.expect("the union");
    let doc = session.committed_doc().clone();
    let offer = viewer::world::repoint_offer(&doc, union);
    let (_, offers) = viewer::frame::placement_offers(&doc, union);
    eprintln!("offer: {offer:?}\nbuttons: {offers:?}");
    let Some(offer) = offer else {
        return;
    };
    assert_eq!(offer.placements, [second], "the cycle is left out");
    let (_, op) = offers
        .into_iter()
        .find(|(_, op)| matches!(op, SessionOp::RepointPlacements { .. }))
        .expect("a re-point button");
    let done = session.perform(op);
    eprintln!("pressed: refusal {:?}", done.refusal);
    assert!(
        done.refusal.is_none(),
        "the button the offer shows is refused: {:?}",
        done.refusal
    );
    let doc = session.committed_doc();
    assert_eq!(doc.copied_read(second), doc.output(union, 0));
}

/// Claim 3 order/rebind: a re-pointed pair, then a fillet picked on one
/// copy reading F's carried edge, then F deleted. The fillet on the copy
/// must be handed nothing (it reads the copy, which stays) but its edge
/// name must come back to the bracket's.
#[test]
fn r1_delete_after_repoint_rebinds_a_feature_on_a_copy() {
    let tol = Tol::witness();
    let (mut session, body, first, second) = two_world_copies_both_in_product(tol);
    let edges = lateral_edges(&session, body);
    let f = common::session_insert(
        &mut session,
        SessionOp::AddFillet {
            target: body,
            radius: len(0.001),
            selection: vec![edges[0].clone()],
        },
    );
    session.pump();
    let shared = session.committed_doc().output(body, 0).expect("shared");
    let before_repoint = world(session.committed_doc());
    let done = session.perform(SessionOp::RepointPlacements {
        body: Operand::Var(shared),
        onto: f,
    });
    assert!(done.refusal.is_none(), "{:?}", done.refusal);
    session.pump();
    // A chamfer picked on the first copy, on an edge F carries.
    let carried = pncad::select::all_edges(session.evaluation().expect("landed"), first)
        .into_iter()
        .find(|name| matches!(name.path.as_slice(), [RoleSeg::From { of, .. }] if *of.name() == edges[1]))
        .expect("the copy carries F's carried edge");
    let k = common::session_insert(
        &mut session,
        SessionOp::AddChamfer {
            target: first,
            distance: len(0.0005),
            selection: vec![carried],
        },
    );
    session.pump();
    let out = session.perform(SessionOp::DeleteNode { node: f });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    let doc = session.committed_doc();
    eprintln!("committed: {:#?}", out.committed);
    assert!(doc.node(k).is_some(), "the chamfer on the copy stays");
    let Some(pncad::document::Node::Chamfer { selection, .. }) = doc.node(k) else {
        panic!()
    };
    let select = doc.selection(*selection).expect("sel");
    assert_eq!(
        select.names,
        [edges[1].clone()],
        "rebound to the bracket's name"
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, viewer::tree::RowStatus::Ok, "{row:?}");
    }
    let _ = (second, before_repoint);
}
