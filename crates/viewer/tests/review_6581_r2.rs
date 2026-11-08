//! **Review probes for PR 6581** (lane `placing-dual-6581-r2`). Each row
//! states a claim of the PR and goes red where the head breaks it.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::len;
use pncad::document::{
    Bodies, BodyRead, Doc, Formula, Node, Operand, ProfileProgram, RecipeNodeId, VarId,
};
use pncad::geom_core::Tol;
use viewer::session::{DocSession, PatternRuleSpec, SessionOp};

const SIZE: [f64; 3] = [0.02, 0.02, 0.01];

/// The bracket in its own session, landed.
fn bracket(name: &str, tol: Tol) -> (DocSession, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived(name, tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    session.pump();
    (session, body)
}

fn place(session: &mut DocSession, body: RecipeNodeId) -> RecipeNodeId {
    let out = session.perform(SessionOp::Place { body: body.into() });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    out.made.expect("a placement")
}

fn with(
    session: &DocSession,
    node: pncad::document::AuthoredNode,
    tol: Tol,
) -> (DocSession, RecipeNodeId) {
    let (doc, id) = viewer::test_support::inserted(session.committed_doc(), node, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    (session, id)
}

/// **The offer's button does what it says** (`world::repoint_offer`
/// leaves out a placement the offering body reads; the op it sends,
/// `RepointPlacements { body, onto }`, re-points `placements_of_var`
/// whole). The bracket Placed twice in its own space; `U = B ∪ C1`. The
/// offer on `U` names `C2` alone; accepting it must re-point `C2` and
/// leave `C1`.
#[test]
fn review_the_offer_excluding_a_cycle_is_what_the_button_does() {
    let tol = Tol::witness();
    let (mut session, body) = bracket("offer-cycle", tol);
    let c1 = place(&mut session, body);
    let c2 = place(&mut session, body);
    let b = Operand::Node(body);
    let (mut session, u) = with(
        &session,
        Node::Union {
            members: Bodies::Spelled(vec![b.clone().into(), Operand::Node(c1).into()]),
        },
        tol,
    );
    let doc = session.committed_doc().clone();
    let offer = viewer::world::repoint_offer(&doc, u).expect("U offers");
    assert_eq!(offer.placements, [c2], "premise: the offer leaves C1 out");
    let (_, offers) = viewer::frame::placement_offers(&doc, u);
    let (label, op) = offers
        .iter()
        .find(|(_, op)| matches!(op, SessionOp::RepointPlacements { .. }))
        .expect("the button")
        .clone();
    assert!(label.starts_with("Re-point the 1 placement "), "{label}");
    let out = session.perform(op);
    assert!(
        out.refusal.is_none(),
        "the button the offer shows refuses: {:?}",
        out.refusal
    );
    let doc = session.committed_doc();
    assert_eq!(doc.copied_read(c2), doc.output(u, 0), "C2 re-pointed");
    assert_eq!(doc.copied_read(c1), doc.output(body, 0), "C1 left");
}

/// The family of two the pattern gesture makes of the bracket picked
/// as itself, and a plain Place of the bracket: the session, the
/// family node, the placement.
fn family_and_copy(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId) {
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
    (session, family, copy)
}

fn member0(doc: &Doc<ProfileProgram>, family: RecipeNodeId) -> BodyRead<Formula> {
    let var: VarId = doc.output(family, 0).expect("the family's output");
    BodyRead::indexed(Operand::Var(var), vec![Formula::count(0)])
}

/// **An operator left with one operand continues that operand** (DM6),
/// where the operand is a family member, `xs[0]`: `∪[xs[0], C]` with
/// `C` deleted. A world copy read the union, one body; after the delete
/// it must still copy one body, member 0, not the whole family.
#[test]
fn review_a_union_left_with_a_member_continues_the_member() {
    let tol = Tol::witness();
    let (session, family, copy) = family_and_copy(tol);
    let m0 = member0(session.committed_doc(), family);
    let (session, u) = with(
        &session,
        Node::Union {
            members: Bodies::Spelled(vec![m0, Operand::Node(copy).into()]),
        },
        tol,
    );
    let (mut session, reader) = with(&session, common::world_place(Operand::Node(u)), tol);
    let out = session.perform(SessionOp::DeleteNode { node: copy });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    let doc = session.committed_doc();
    assert_member_read(doc, reader, family);
}

/// The world copy `reader` survives and copies one body: member 0, not
/// the whole family.
fn assert_member_read(doc: &Doc<ProfileProgram>, reader: RecipeNodeId, family: RecipeNodeId) {
    let Some(Node::Place { source, .. }) = doc.node(reader) else {
        panic!("the world copy is gone")
    };
    assert!(
        source.is_indexed() || Some(source.read) != doc.output(family, 0),
        "the world copy now copies the whole family, not member 0: {source:?}"
    );
}

/// **A cut whose tool is gone continues its target** (DM6), where the
/// target is a family member: `xs[0] − C`, `C` deleted.
#[test]
fn review_a_cut_of_a_member_continues_the_member() {
    let tol = Tol::witness();
    let (session, family, copy) = family_and_copy(tol);
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
    let out = session.perform(SessionOp::DeleteNode { node: copy });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = session.committed_doc();
    assert_member_read(doc, reader, family);
}

/// **A fillet's delete with its input read twice by one operator**:
/// `∪[F, C]` where `F = fillet(C)`... spelled here as `∪[F, B]` with
/// `F = fillet(B)`: deleting `F` hands the union `[B, B]`; the delete
/// must not refuse.
#[test]
fn review_deleting_a_fillet_beside_its_own_input() {
    let tol = Tol::witness();
    let (mut session, body) = bracket("twice", tol);
    let edge = pncad::select::all_edges(session.evaluation().expect("landed"), body)[0].clone();
    let fillet = common::session_insert(
        &mut session,
        SessionOp::AddFillet {
            target: body,
            radius: len(0.001),
            selection: vec![edge],
        },
    );
    let (mut session, _u) = with(
        &session,
        Node::Union {
            members: Bodies::Spelled(vec![
                Operand::Node(fillet).into(),
                Operand::Node(body).into(),
            ]),
        },
        tol,
    );
    let out = session.perform(SessionOp::DeleteNode { node: fillet });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
}

/// The bracket's lateral edges as `node`'s landed table names them.
fn laterals(session: &DocSession, node: RecipeNodeId) -> Vec<pncad::prelude::StableName> {
    pncad::select::all_edges(session.evaluation().expect("landed"), node)
        .into_iter()
        .filter(|name| {
            matches!(
                name.path.as_slice(),
                [pncad::prelude::RoleSeg::LateralEdge(_)]
            )
        })
        .collect()
}

/// **Rebinds are complete, two features deep** (claim 3; the forward
/// takeover rebinds "at whatever depth the selection embeds a copy's
/// read", `Recording::take_over`). The bracket `B` and its world copy
/// `C`; `G = fillet(C)`, then `K = chamfer(G)` on an edge `G` carries
/// from `C`. Then `F = fillet(B)` picked as itself, its re-point
/// accepted, then `F` deleted. Every row is well after the re-point and
/// after the delete, and the delete leaves the document as it was
/// before `F`.
#[test]
fn review_a_chain_on_a_copy_survives_the_re_point_and_its_delete() {
    let tol = Tol::witness();
    let (session, body) = bracket("deep", tol);
    let (doc, copy) = common::world_copy(session.committed_doc(), body, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let edges = laterals(&session, copy);
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
                [pncad::prelude::RoleSeg::From { of, .. }] if *of.name() == edges[1])
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
            assert_eq!(row.status, viewer::tree::RowStatus::Ok, "{when}: {row:?}");
        }
    };
    ok(&session, "premise");
    let before: Vec<_> = {
        let doc = session.committed_doc();
        let mut ids = doc.ids().to_vec();
        ids.sort_unstable();
        ids.into_iter()
            .map(|id| (id, doc.node(id).map(|node| node.written(doc))))
            .collect()
    };
    let third = laterals(&session, body)[2].clone();
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
    let after: Vec<_> = {
        let doc = session.committed_doc();
        let mut ids = doc.ids().to_vec();
        ids.sort_unstable();
        ids.into_iter()
            .map(|id| (id, doc.node(id).map(|node| node.written(doc))))
            .collect()
    };
    assert!(after == before, "the delete of F is not the identity");
}

/// **A feature picked on an own-space copy reads that copy alone**
/// (claim 1, "world and own-space alike"; the PR's row covers world
/// copies): the bracket Placed twice in its own space, the second slid
/// clear. Each gesture on the first writes no source, both copies still
/// place the bracket, the second is still drawn, and what it made reads
/// the first.
#[test]
fn review_a_feature_on_an_own_space_copy_reads_that_copy_alone() {
    let tol = Tol::witness();
    let fresh = || {
        let (mut session, body) = bracket("own-space-gestures", tol);
        let c1 = place(&mut session, body);
        let c2 = place(&mut session, body);
        let op = common::transformed(&mut session, c2, &common::slides([0.1, 0.0, 0.0]));
        assert!(session.perform(op).refusal.is_none());
        session.pump();
        (session, body, c1, c2)
    };
    let row = || PatternRuleSpec::Linear {
        direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::Y),
        spacing: len(2.0 * SIZE[1]),
    };
    let gestures = |session: &DocSession, body: RecipeNodeId, c1: RecipeNodeId| {
        let edge = laterals(session, c1)[0].clone();
        vec![
            (
                "fillet",
                SessionOp::AddFillet {
                    target: c1,
                    radius: len(0.001),
                    selection: vec![edge.clone()],
                },
            ),
            (
                "chamfer",
                SessionOp::AddChamfer {
                    target: c1,
                    distance: len(0.001),
                    selection: vec![edge],
                },
            ),
            (
                "sketch on face",
                SessionOp::SketchOnFace {
                    face: viewer::session::FaceSelection {
                        name: pncad::prelude::StableName {
                            kind: pncad::prelude::EntityKind::Face,
                            node: body,
                            path: vec![pncad::prelude::RoleSeg::Cap(pncad::prelude::CapEnd::End)],
                        },
                        node: c1,
                        body: 0,
                    },
                    loops: vec![common::rectangle_loop([0.0, 0.0], 0.004, 0.004)],
                    distance: len(0.003),
                    spin: Formula::zero(pncad::document::Dimension::Angle),
                },
            ),
            (
                "split",
                SessionOp::AddSplit {
                    target: c1.into(),
                    plane: common::cut_across(session, c1, SIZE[2] / 2.0),
                },
            ),
            (
                "pattern",
                SessionOp::AddPattern {
                    input: c1.into(),
                    count: 2,
                    rule: row(),
                },
            ),
            (
                "fused pattern",
                SessionOp::AddFusedPattern {
                    input: c1.into(),
                    count: 2,
                    rule: row(),
                },
            ),
        ]
    };
    let (probe, body, c1, _) = fresh();
    let count = gestures(&probe, body, c1).len();
    let mut bad = Vec::new();
    for at in 0..count {
        let (mut session, body, c1, c2) = fresh();
        let (name, op) = gestures(&session, body, c1).swap_remove(at);
        let read = session.committed_doc().output(body, 0);
        let out = session.perform(op);
        if let Some(refusal) = out.refusal {
            bad.push(format!("{name}: refused {refusal:?}"));
            continue;
        }
        session.pump();
        let doc = session.committed_doc();
        if out.committed.iter().any(|edit| {
            matches!(
                edit,
                pncad::document::DocEdit::SetParam {
                    slot: pncad::document::SlotId::Operand(pncad::document::OperandSlot::Source),
                    ..
                }
            )
        }) {
            bad.push(format!("{name}: wrote a source"));
        }
        if [c1, c2].map(|copy| doc.copied_read(copy)) != [read; 2] {
            bad.push(format!("{name}: a copy no longer places the bracket"));
        }
        if !viewer::world::is_drawn(doc, c2) {
            bad.push(format!("{name}: the second copy is not drawn"));
        }
        let reads_c1 = out.minted.iter().any(|&node| {
            viewer::world::built_on_copy(doc, node) == Some((c1, body))
                || doc.copied_read(node) == doc.output(c1, 0)
        });
        if !reads_c1 {
            bad.push(format!("{name}: nothing it made reads the first copy"));
        }
        for row in session.tree_rows() {
            if row.status != viewer::tree::RowStatus::Ok {
                bad.push(format!("{name}: {row:?}"));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The same through the combine gesture: the bracket picked as itself
/// combined with its own Place copy `C1`. The union offers `C2` alone,
/// and its button refuses.
#[test]
fn review_the_combine_of_a_body_and_its_copy_offers_a_button_that_works() {
    let tol = Tol::witness();
    let (mut session, body) = bracket("offer-cycle-combine", tol);
    let c1 = place(&mut session, body);
    let c2 = place(&mut session, body);
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
    let doc = session.committed_doc().clone();
    let offer = viewer::world::repoint_offer(&doc, u).expect("U offers");
    assert_eq!(offer.placements, [c2], "premise: the offer leaves C1 out");
    let (_, offers) = viewer::frame::placement_offers(&doc, u);
    let (_, op) = offers
        .into_iter()
        .find(|(_, op)| matches!(op, SessionOp::RepointPlacements { .. }))
        .expect("the button");
    let out = session.perform(op);
    assert!(
        out.refusal.is_none(),
        "the button refuses: {:?}",
        out.refusal
    );
}
