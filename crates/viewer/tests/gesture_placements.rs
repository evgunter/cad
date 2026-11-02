//! **A gesture's delete undoes its takeover**: what a takeover gesture
//! re-pointed from the body it took over — a copy's source, a `⊔`
//! operand, the product slot — its delete hands back, so adding a
//! feature to a placed body and deleting it leaves the world as it was.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::len;
use pncad::document::{Doc, Node, Operand, ProfileProgram, RecipeNodeId, VarId};
use pncad::geom_core::Tol;
use viewer::session::{DocSession, PatternRuleSpec, SessionOp};

const SIDE: f64 = 0.01;

/// A box and its one copy in the world, placed by hand, landed: the
/// session, the box and the copy.
fn placed_box(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived("placed-box", tol), tol);
    let body = common::xy_box_in(&mut session, [SIDE; 3]);
    let (mut session, copy) = common::world_copy_in(&session, body, tol);
    session.pump();
    (session, body, copy)
}

/// The variable `copy`'s placement copies.
fn source_of(doc: &Doc<ProfileProgram>, copy: RecipeNodeId) -> Option<VarId> {
    match doc.node(copy) {
        Some(Node::Place { source, .. }) => Some(source.read),
        _ => None,
    }
}

/// **`doc` holds the nodes `before` held, each as it was, and draws
/// what it drew**: the identity on the world.
fn assert_world_as(doc: &Doc<ProfileProgram>, before: &Doc<ProfileProgram>, what: &str) {
    let mut ids = doc.ids().to_vec();
    let mut was = before.ids().to_vec();
    ids.sort_unstable();
    was.sort_unstable();
    assert_eq!(ids, was, "{what}: the same nodes");
    for id in ids {
        assert_eq!(
            doc.node(id),
            before.node(id),
            "{what}: node {id:?} as it was"
        );
    }
    assert_eq!(
        viewer::world::drawn_nodes(doc),
        viewer::world::drawn_nodes(before),
        "{what}: the same bodies drawn"
    );
}

/// A fillet of one edge of the copy `copy` of `body`.
fn fillet_of(session: &DocSession, body: RecipeNodeId, copy: RecipeNodeId) -> SessionOp {
    let eval = session.evaluation().expect("the inline seam landed");
    let names = pncad::select::all_edges(eval, body);
    SessionOp::AddFillet {
        target: copy,
        radius: len(0.001),
        selection: names[..1].to_vec(),
    }
}

/// **Add a fillet to a placed body, delete it, and the body is placed
/// as it was**: the fillet takes the copy's source over; its delete
/// re-points the source back to the box in the same action, deletes
/// the fillet alone, and leaves every node as the add found it. The
/// button prices that one node, undo is one step back to the filleted
/// copy, and redo one step forward again. Deleting the box itself still
/// takes the copy: nothing upstream is left to hand it to.
///
/// *Breaks if* the delete cascades the copy with the fillet (the body
/// leaves the world), hands it back to anything but the box, or splits
/// the action across history steps.
#[test]
fn deleting_a_feature_hands_its_copy_back_to_the_body() {
    let tol = Tol::witness();
    let (mut session, body, copy) = placed_box(tol);
    let before = session.committed_doc().clone();
    let boxed = source_of(&before, copy).expect("the copy places the box");
    let op = fillet_of(&session, body, copy);
    let fillet = common::session_insert(&mut session, op);
    let filleted = session.committed_doc().output(fillet, 0);
    assert_eq!(
        source_of(session.committed_doc(), copy),
        filleted,
        "the add takes the copy's source over"
    );

    let affordance = session.delete_affordance(fillet);
    assert_eq!(
        affordance.cascade,
        vec![fillet],
        "the button prices the fillet alone"
    );
    assert_eq!(affordance.hover, None, "and names no dependent");
    let steps = session.history().len();
    let out = session.perform(SessionOp::DeleteNode { node: fillet });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = session.committed_doc();
    assert_eq!(
        source_of(doc, copy),
        Some(boxed),
        "the copy places the box again"
    );
    assert!(doc.node(fillet).is_none(), "the fillet is gone");
    assert_world_as(doc, &before, "add then delete");
    assert_eq!(session.history().len(), steps + 1, "one action");

    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert_eq!(
        source_of(session.committed_doc(), copy),
        filleted,
        "one undo restores the fillet and the copy reading it"
    );
    assert!(session.perform(SessionOp::Redo).refusal.is_none());
    assert_world_as(
        session.committed_doc(),
        &before,
        "one redo deletes it again",
    );

    let out = session.perform(SessionOp::DeleteNode { node: body });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert!(
        session.committed_doc().node(copy).is_none(),
        "deleting the body takes its copy"
    );
}

/// **A pattern's delete hands the product slot back**: the product
/// reads the copy, a pattern of the copy takes the slot over through a
/// `⊔` of its family, and deleting the pattern takes the `⊔` with it
/// and points the slot at the copy again, as one action.
///
/// *Breaks if* the delete leaves the product reading nothing, or reads
/// the `⊔` it deleted.
#[test]
fn deleting_a_pattern_hands_the_product_back() {
    let tol = Tol::witness();
    let (mut session, _body, copy) = placed_box(tol);
    common::product_is(&mut session, &[copy]);
    let before = viewer::world::product_reads(session.committed_doc());
    assert_eq!(
        before,
        vec![session.committed_doc().output(copy, 0).expect("the copy")],
        "the product reads the copy"
    );
    let row = common::session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: Operand::Node(copy),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: common::scl3([1.0, 0.0, 0.0]),
                spacing: len(2.0 * SIDE),
            },
        },
    );
    assert_ne!(
        viewer::world::product_reads(session.committed_doc()),
        before,
        "the pattern takes the product over"
    );
    let steps = session.history().len();
    let out = session.perform(SessionOp::DeleteNode { node: row });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = session.committed_doc();
    assert!(doc.node(row).is_none(), "the pattern is gone");
    assert_eq!(
        viewer::world::product_reads(doc),
        before,
        "the product reads the copy again"
    );
    assert_eq!(session.history().len(), steps + 1, "one action");
}

// ---- Review rows (PR 6280 full review lane) ----

use pncad::document::{Dimension, Formula};
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
use viewer::session::FaceSelection;

/// The world as a reader sees it: every node, the product, what is drawn.
fn world(doc: &Doc<ProfileProgram>) -> World {
    let mut ids = doc.ids().to_vec();
    ids.sort_unstable();
    (
        ids.into_iter()
            .map(|id| (id, doc.node(id).cloned()))
            .collect(),
        doc.product(),
        viewer::world::drawn_nodes(doc),
    )
}

/// What differs between two worlds, said briefly.
fn differs(a: &World, b: &World) -> Option<String> {
    if a == b {
        return None;
    }
    let ids = |w: &World| w.0.iter().map(|(id, _)| *id).collect::<Vec<_>>();
    let extra: Vec<_> =
        a.0.iter()
            .filter(|(id, _)| !ids(b).contains(id))
            .map(|(id, n)| (id, n.as_ref().map(kind)))
            .collect();
    let missing: Vec<_> =
        b.0.iter()
            .filter(|(id, _)| !ids(a).contains(id))
            .map(|(id, n)| (id, n.as_ref().map(kind)))
            .collect();
    let changed: Vec<_> =
        a.0.iter()
            .filter(|(id, n)| b.0.iter().any(|(j, m)| j == id && m != n))
            .map(|(id, n)| (id, n.as_ref().map(kind)))
            .collect();
    Some(format!(
        "extra {extra:?} missing {missing:?} changed {changed:?} product {:?} vs {:?} drawn {:?} vs {:?}",
        a.1, b.1, a.2, b.2
    ))
}

fn kind(n: &Node<ProfileProgram>) -> String {
    format!("{n:?}")
        .split([' ', '(', '{'])
        .next()
        .unwrap_or("")
        .to_string()
}

type World = (
    Vec<(RecipeNodeId, Option<Node<ProfileProgram>>)>,
    Option<VarId>,
    Vec<RecipeNodeId>,
);

/// Add `op`, delete what it made, and ask the world is as it was, and
/// that undo/redo are one step each and exact. Answers each failure.
fn round_trip(session: &mut DocSession, op: SessionOp, what: &str) -> Vec<String> {
    round_trip_at(session, op, what, false)
}

/// [`round_trip`], deleting the first node the add minted (the
/// gesture's root) where `root`, else the node it made.
fn round_trip_at(session: &mut DocSession, op: SessionOp, what: &str, root: bool) -> Vec<String> {
    let mut bad = Vec::new();
    session.pump();
    let before = world(session.committed_doc());
    let out = session.perform(op);
    if let Some(r) = out.refusal {
        return vec![format!("{what}: add refused {r:?}")];
    }
    let made = if root {
        *out.minted
            .iter()
            .find(|id| session.committed_doc().node(**id).is_some())
            .expect("minted")
    } else {
        out.made.expect("made")
    };
    session.pump();
    let added = world(session.committed_doc());
    let aff = session.delete_affordance(made);
    let steps = session.history().len();
    let out = session.perform(SessionOp::DeleteNode { node: made });
    if let Some(r) = out.refusal {
        return vec![format!("{what}: delete refused {r:?}")];
    }
    let doc = session.committed_doc();
    let mut gone: Vec<RecipeNodeId> = added
        .0
        .iter()
        .map(|(id, _)| *id)
        .filter(|id| doc.node(*id).is_none())
        .collect();
    let mut priced = aff.cascade.clone();
    priced.sort_unstable();
    gone.sort_unstable();
    if priced != gone {
        bad.push(format!("{what}: priced {priced:?} gone {gone:?}"));
    }
    if session.history().len() != steps + 1 {
        bad.push(format!("{what}: not one action"));
    }
    if let Some(d) = differs(&world(session.committed_doc()), &before) {
        bad.push(format!("{what}: add then delete: {d}"));
    }
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    if let Some(d) = differs(&world(session.committed_doc()), &added) {
        bad.push(format!("{what}: undo: {d}"));
    }
    assert!(session.perform(SessionOp::Redo).refusal.is_none());
    if let Some(d) = differs(&world(session.committed_doc()), &before) {
        bad.push(format!("{what}: redo: {d}"));
    }
    bad
}

fn cap(node: RecipeNodeId, on: RecipeNodeId) -> FaceSelection {
    FaceSelection {
        name: StableName {
            kind: EntityKind::Face,
            node,
            path: vec![RoleSeg::Cap(CapEnd::End)],
        },
        node: on,
        body: 0,
    }
}

fn transform(input: Operand) -> SessionOp {
    SessionOp::AddTransform {
        input,
        translation: [
            len(0.03),
            Formula::zero(Dimension::Length),
            Formula::zero(Dimension::Length),
        ],
        rotation_axis: common::scl3([0.0, 0.0, 1.0]),
        rotation_angle: Formula::zero(Dimension::Angle),
    }
}

fn row(input: Operand) -> PatternRuleSpec {
    let _ = input;
    PatternRuleSpec::Linear {
        direction: common::scl3([1.0, 0.0, 0.0]),
        spacing: len(2.0 * SIDE),
    }
}

/// Every takeover gesture, from a placed copy the product reads, and
/// from a placed copy nothing else reads.
#[test]
fn review_every_gesture_round_trips() {
    let mut bad: Vec<String> = Vec::new();
    let tol = Tol::witness();
    for product in [false, true] {
        let fresh = || {
            let (mut s, body, copy) = placed_box(tol);
            if product {
                common::product_is(&mut s, &[copy]);
            }
            (s, body, copy)
        };
        let (mut s, body, copy) = fresh();
        let op = fillet_of(&s, body, copy);
        bad.extend(round_trip(&mut s, op, &format!("fillet p={product}")));
        let (mut s, body, copy) = fresh();
        let names = pncad::select::all_edges(s.evaluation().expect("landed"), body);
        bad.extend(round_trip(
            &mut s,
            SessionOp::AddChamfer {
                target: copy,
                distance: len(0.001),
                selection: names[..1].to_vec(),
            },
            &format!("chamfer p={product}"),
        ));
        let (mut s, body, _copy) = fresh();
        bad.extend(round_trip(
            &mut s,
            SessionOp::AddMirror {
                body: body.into(),
                plane: cap(body, body),
            },
            &format!("mirror body p={product}"),
        ));
        let (mut s, body, copy) = fresh();
        bad.extend(round_trip(
            &mut s,
            SessionOp::SketchOnFace {
                face: cap(body, copy),
                loops: vec![common::rectangle_loop([0.0, 0.0], 0.002, 0.002)],
                distance: len(0.002),
                spin: Formula::zero(Dimension::Angle),
            },
            &format!("sketch p={product}"),
        ));
        let (mut s, body, _) = fresh();
        bad.extend(round_trip(
            &mut s,
            transform(body.into()),
            &format!("transform body p={product}"),
        ));
        let (mut s, _, copy) = fresh();
        bad.extend(round_trip(
            &mut s,
            transform(copy.into()),
            &format!("transform copy p={product}"),
        ));
        for on_copy in [false, true] {
            let (mut s, body, copy) = fresh();
            let input: Operand = if on_copy { copy.into() } else { body.into() };
            bad.extend(round_trip(
                &mut s,
                SessionOp::AddPattern {
                    input: input.clone(),
                    count: 2,
                    rule: row(input.clone()),
                },
                &format!("pattern copy={on_copy} p={product}"),
            ));
            let (mut s, body, copy) = fresh();
            let input: Operand = if on_copy { copy.into() } else { body.into() };
            bad.extend(round_trip(
                &mut s,
                SessionOp::AddFusedPattern {
                    input: input.clone(),
                    count: 2,
                    rule: row(input.clone()),
                },
                &format!("fused copy={on_copy} p={product}"),
            ));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// A feature on a feature on a copy: delete the middle (bottom) one.
#[test]
fn review_chain_delete_middle() {
    let tol = Tol::witness();
    let (mut s, body, copy) = placed_box(tol);
    common::product_is(&mut s, &[copy]);
    let before = world(s.committed_doc());
    let op = fillet_of(&s, body, copy);
    let f1 = common::session_insert(&mut s, op);
    s.pump();
    let names = pncad::select::all_edges(s.evaluation().expect("landed"), f1);
    let f2 = common::session_insert(
        &mut s,
        SessionOp::AddChamfer {
            target: copy,
            distance: len(0.0005),
            selection: names[names.len() - 1..].to_vec(),
        },
    );
    s.pump();
    assert_eq!(
        source_of(s.committed_doc(), copy),
        s.committed_doc().output(f2, 0)
    );
    let aff = s.delete_affordance(f1);
    assert_eq!(aff.cascade, vec![f2, f1]);
    let out = s.perform(SessionOp::DeleteNode { node: f1 });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert_eq!(
        world(s.committed_doc()),
        before,
        "delete the bottom of the chain"
    );
}

/// A copy of a copy: a transform of the world copy, then a fillet on it.
#[test]
fn review_copy_of_copy() {
    let mut bad: Vec<String> = Vec::new();
    let tol = Tol::witness();
    let (mut s, body, copy) = placed_box(tol);
    let t = common::session_insert(&mut s, transform(copy.into()));
    s.pump();
    common::product_is(&mut s, &[t]);
    let op = fillet_of(&s, body, t);
    bad.extend(round_trip(&mut s, op, "fillet on a copy of a copy"));
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **Deleting a duplicate is no takeover's inverse**: a body nothing
/// copies, duplicated (two copies in its own space), the stepped copy
/// then placed in the world by hand; deleting the stepped copy
/// re-points that world copy to the body, which no gesture ever had it
/// read, where the cascade took it before.
#[test]
fn review_deleting_a_duplicate_hands_nothing_back() {
    let tol = Tol::witness();
    let mut s = common::inline_without_door(Doc::empty_derived("dup", tol), tol);
    let body = common::xy_box_in(&mut s, [SIDE; 3]);
    s.pump();
    let out = s.perform(SessionOp::Duplicate {
        input: body.into(),
        beside: None,
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let stepped = out.made.expect("made");
    s.pump();
    let (mut s, world_copy) = common::world_copy_in(&s, stepped, tol);
    let out = s.perform(SessionOp::DeleteNode { node: stepped });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = s.committed_doc();
    assert!(
        doc.node(world_copy).is_none(),
        "the world copy of the deleted duplicate goes with it; it now places {:?} (the body is {:?})",
        source_of(doc, world_copy),
        doc.output(body, 0)
    );
}

/// The gesture's root deleted instead of what it made.
#[test]
fn review_root_round_trips() {
    let tol = Tol::witness();
    let mut bad: Vec<String> = Vec::new();
    let (mut s, body, _) = placed_box(tol);
    bad.extend(round_trip_at(
        &mut s,
        SessionOp::AddMirror {
            body: body.into(),
            plane: cap(body, body),
        },
        "mirror root",
        true,
    ));
    let (mut s, body, copy) = placed_box(tol);
    bad.extend(round_trip_at(
        &mut s,
        SessionOp::SketchOnFace {
            face: cap(body, copy),
            loops: vec![common::rectangle_loop([0.0, 0.0], 0.002, 0.002)],
            distance: len(0.002),
            spin: Formula::zero(Dimension::Angle),
        },
        "sketch root",
        true,
    ));
    let (mut s, _, copy) = placed_box(tol);
    bad.extend(round_trip_at(
        &mut s,
        SessionOp::AddFusedPattern {
            input: copy.into(),
            count: 2,
            rule: row(copy.into()),
        },
        "fused root",
        true,
    ));
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
