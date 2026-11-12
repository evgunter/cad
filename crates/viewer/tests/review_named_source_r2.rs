//! Review probes for PR 6856 (lane `placing-dual-6856-r2`), viewer side.
//! Each row asserts what the PR or VARIABLES VR4 promise; a red row is a
//! finding.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use pncad::document::{
    Doc, Formula, Node, Operand, ProfileProgram, RecipeNodeId, VarDef, VarId, VarName,
};
use pncad::geom_core::Tol;
use pncad::prelude::RoleSeg;
use viewer::session::{DocSession, PatternRuleSpec, Refusal, SessionOp};
use viewer::tree::RowStatus;

const SIZE: [f64; 3] = [0.02, 0.02, 0.01];

fn landed(session: &mut DocSession, op: SessionOp) -> RecipeNodeId {
    let out = session.perform(op);
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    out.made.expect("a node")
}

fn vars(doc: &Doc<ProfileProgram>) -> Vec<(VarId, VarDef, Option<VarName>)> {
    doc.var_ids()
        .iter()
        .map(|&id| {
            (
                id,
                doc.var(id).expect("live").def().clone(),
                doc.var_name(id).cloned(),
            )
        })
        .collect()
}

fn nodes(doc: &Doc<ProfileProgram>) -> Vec<String> {
    let mut ids = doc.ids().to_vec();
    ids.sort_unstable();
    ids.into_iter()
        .map(|id| format!("{id:?}: {:?}", doc.node(id).map(|n| n.written(doc))))
        .collect()
}

/// Claim 3: placing B twice (naming `bracket`), filleting B, accepting
/// the offer and deleting F is the identity — on the nodes AND on the
/// variable table (every variable's definition and name as before the
/// fillet, and no variable added or lost).
#[test]
fn probe_accept_then_delete_f_is_the_identity_on_the_variables() {
    let tol = Tol::witness();
    let mut session =
        common::inline_without_door(Doc::empty_derived("r2-identity", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    session.pump();
    for _ in 0..2 {
        let op = common::place_named(session.committed_doc(), body);
        landed(&mut session, op);
    }
    let bracket = session.committed_doc().var_named("bracket").expect("named");
    let before = (nodes(session.committed_doc()), vars(session.committed_doc()));
    let edge = pncad::select::all_edges(session.evaluation().expect("landed"), body)
        .into_iter()
        .find(|name| matches!(name.path.as_slice(), [RoleSeg::LateralEdge(_)]))
        .expect("a lateral edge");
    let f = landed(
        &mut session,
        SessionOp::AddFillet {
            target: body,
            radius: common::len(0.001),
            selection: vec![edge],
        },
    );
    landed(
        &mut session,
        SessionOp::RepointPlacements {
            source: bracket,
            onto: f.into(),
        },
    );
    let out = session.perform(SessionOp::DeleteNode { node: f });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    let after = (nodes(session.committed_doc()), vars(session.committed_doc()));
    assert_eq!(after.0, before.0, "nodes");
    assert_eq!(after.1, before.1, "variables");
}

/// A member placement keeps its index on the placement and reads a
/// source of the family. The re-point / "copy of" row's op redefining
/// that source onto a plain body must refuse (or write a document that
/// saves).
#[test]
fn probe_repointing_a_member_placements_source_onto_a_body() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("r2-member", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let other = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let family = landed(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: common::len(2.0 * SIZE[0]),
            },
        },
    );
    let doc = session.committed_doc().clone();
    let var = doc.output(family, 0).expect("the family's output");
    let space = doc.node_space(family).expect("its space");
    let (doc, member) = viewer::test_support::inserted(
        &doc,
        Node::place(
            pncad::document::BodyRead::indexed(Operand::Var(var), vec![Formula::count(0)]),
            Operand::Var(space),
            pncad::document::stand().bundle(),
        ),
        tol,
    );
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let source = session
        .committed_doc()
        .copied_read(member)
        .expect("a source");
    // What the placement editor's "copy of" row pushes with `other`
    // selected.
    let out = session.perform(SessionOp::RepointPlacements {
        source,
        onto: other.into(),
    });
    if out.refusal.is_some() {
        return;
    }
    session.pump();
    let saved = pncad::document::save(session.committed_doc(), &[], tol);
    let rows: Vec<_> = session
        .tree_rows()
        .into_iter()
        .filter(|row| row.status != RowStatus::Ok)
        .collect();
    assert!(
        saved.is_ok() && rows.is_empty(),
        "the session admitted the re-point; save: {:?}; rows not ok: {rows:?}",
        saved.map(|_| ())
    );
}

/// The lane's decision "Place on a pattern-placed body shares the
/// pattern's source": a pattern of B, then Place B. Records what the
/// session does without a name (expected `SharedVarNeedsName`), and that
/// the named Place shares the pattern's source.
#[test]
fn probe_place_on_a_pattern_placed_body() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("r2-pattern", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let family = landed(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: common::len(2.0 * SIZE[0]),
            },
        },
    );
    let doc = session.committed_doc().clone();
    let pattern_source = doc.copied_read(family).expect("the pattern reads a source");
    let plain = SessionOp::place_where_it_stands(&doc, body.into()).expect("placeable");
    let out = session.perform(plain);
    assert!(
        matches!(&out.refusal, Some(Refusal::Edit(e))
            if matches!(**e, pncad::document::EditError::SharedVarNeedsName { .. })),
        "{:?}",
        out.refusal
    );
    let named = common::place_named(&doc, body);
    let copy = landed(&mut session, named);
    let doc = session.committed_doc();
    assert_eq!(doc.copied_read(copy), Some(pattern_source));
    assert_eq!(doc.var_named("bracket"), Some(pattern_source));
}

/// A revolve's node defines its body beside its axis, so the viewer's
/// body seat reads it as `Operand::Var(body)` (`session::as_body`), and
/// `Node::place` passes a `Var` read through unwrapped. Claim 1 hunt:
/// the pattern gesture, the combine's off-space placement and Place on a
/// revolve must all still land (each placement through a source).
#[test]
fn probe_gestures_that_place_a_revolve_still_land() {
    use core::f64::consts::TAU;
    use pncad::document::{Dimension, LoopProgram};
    let tol = Tol::witness();
    let fresh = || {
        let mut session = DocSession::inline(Doc::empty_derived("r2-revolve", tol), tol);
        let out = session.perform(SessionOp::NewDocument {
            name: "r2-revolve".to_owned(),
        });
        assert!(out.refusal.is_none(), "{:?}", out.refusal);
        let profile = common::session_insert(
            &mut session,
            SessionOp::AddProfile {
                loops: vec![LoopProgram::Circle {
                    centre: [common::len(0.3), Formula::zero(Dimension::Length)],
                    radius: common::len(0.07),
                }],
            },
        );
        let revolve = common::session_insert(
            &mut session,
            SessionOp::AddRevolve {
                profile,
                axis_through: [
                    Formula::zero(Dimension::Length),
                    Formula::zero(Dimension::Length),
                ],
                axis_heading: Formula::turn_over(4).expect("a quarter turn"),
                angle: common::ang(TAU),
            },
        );
        session.pump();
        (session, revolve)
    };
    let mut bad = Vec::new();
    {
        let (mut session, revolve) = fresh();
        let op = SessionOp::place_where_it_stands(session.committed_doc(), revolve.into())
            .expect("placeable");
        if let Some(r) = session.perform(op).refusal {
            bad.push(format!("Place: {r:?}"));
        }
    }
    {
        let (mut session, revolve) = fresh();
        let out = session.perform(SessionOp::AddPattern {
            input: revolve.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: common::len(1.0),
            },
        });
        if let Some(r) = out.refusal {
            bad.push(format!("AddPattern: {r:?}"));
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}
