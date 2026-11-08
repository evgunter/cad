//! Review probes for PR 6356 (Transform retires into the placement
//! editor). Each prints what it observed; the asserts are the claims.
#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use crate::common;
use common::{len, session_insert};
use pncad::document::{Doc, DocEdit, Node, Operand, RecipeNodeId};
use pncad::geom_core::Tol;
use viewer::matetool::MateTool;
use viewer::session::{DocSession, PatternRuleSpec, SessionOp};

const A: [f64; 3] = [0.04, 0.02, 0.01];

fn session(tol: Tol) -> DocSession {
    common::inline_without_door(Doc::empty_derived("probe-6356", tol), tol)
}

fn door_of(doc: &Doc<pncad::document::ProfileProgram>, node: RecipeNodeId) -> String {
    match MateTool::moving(doc, node, None) {
        Ok(mut tool) => {
            let place = tool.take_place();
            format!("{:?} / {place:?}", tool.door())
        }
        Err(event) => format!("refused: {event}"),
    }
}

/// P1: the Transform tool picked on a PROFILE (no body) — does it open
/// a door, or say `NotPlaceable`?
#[test]
fn p1_transform_on_a_profile_opens_no_door() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let (profile, _) = common::box_in(&mut session, A);
    let doc = session.committed_doc();
    let said = door_of(doc, profile);
    eprintln!("P1 profile -> {said}");
    assert!(
        MateTool::moving(doc, profile, None).is_err(),
        "a profile is no body to place, yet the Transform tool opened {said}"
    );
}

/// P2: the Transform tool picked on a whole SPLIT in the tree.
#[test]
fn p2_transform_on_a_whole_split() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let plane = common::cut_across(&session, body, A[2] / 2.0);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            plane,
        },
    );
    let doc = session.committed_doc();
    let said = door_of(doc, split);
    eprintln!("P2 split -> {said}");
    if let Ok(mut tool) = MateTool::moving(doc, split, None)
        && let Some(place) = tool.take_place()
    {
        let outcome = session.perform(place);
        eprintln!("P2 its Place -> {:?}", outcome.refusal);
    }
}

/// P3: the Transform tool picked on a body a PATTERN reads.
#[test]
fn p3_transform_on_a_patterned_body() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let family = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 3,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: len(A[0] * 2.0),
            },
        },
    );
    let doc = session.committed_doc();
    eprintln!(
        "P3 family node {family:?}: {:?}",
        doc.node(family).map(|n| matches!(n, Node::Place { .. }))
    );
    eprintln!("P3 body -> {}", door_of(doc, body));
    eprintln!("P3 family -> {}", door_of(doc, family));
    let edit = MateTool::moving(doc, body, None).ok();
    eprintln!(
        "P3 body door == family's Edit: {}",
        edit.as_ref()
            .and_then(|t| t.door())
            .is_some_and(|door| door.placement == family)
    );
}

/// P4: the stacked shape through Place: `Place` of a COPY.
#[test]
fn p4_place_of_a_copy_into_its_space_lands() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let (mut session, copy) = common::world_copy_in(&session, body, tol);
    let doc = session.committed_doc();
    eprintln!("P4 copy {copy:?} space {:?}", doc.node_space(copy));
    let outcome = session.perform(SessionOp::Place {
        body: Operand::Node(copy),
    });
    eprintln!("P4 refusal {:?} made {:?}", outcome.refusal, outcome.made);
    if let Some(made) = outcome.made {
        let doc = session.committed_doc();
        eprintln!(
            "P4 made {:?} reads copy: {:?}",
            doc.node(made).map(|n| matches!(n, Node::Place { .. })),
            doc.copied_read(made) == doc.read_of_node(copy)
        );
    }
    assert!(
        outcome.refusal.is_some(),
        "a Place of a copy into its own space (the retired stacked shape) lands through SessionOp::Place"
    );
}

/// P5: a FIRST pattern of a placed body picked as itself commits no
/// `SetParam`: no feature gesture re-points a placement (Ev, PR 6353).
#[test]
fn p5_a_pattern_takeover_writes_set_param() {
    let tol = Tol::witness();
    let doc = Doc::empty_derived("probe-6356", tol);
    let mut s0 = common::inline_without_door(doc, tol);
    let body = common::xy_box_in(&mut s0, A);
    let (doc, _) = common::placed(s0.committed_doc(), body, tol);
    let (doc, _) = common::placed(&doc, body, tol);
    let mut session = DocSession::inline(doc, tol);
    let outcome = session.perform(SessionOp::AddPattern {
        input: body.into(),
        count: 2,
        rule: PatternRuleSpec::Linear {
            direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
            spacing: len(A[0] * 2.0),
        },
    });
    let kinds: Vec<&str> = outcome
        .committed
        .iter()
        .map(|e| match e {
            DocEdit::SetParam { .. } => "SetParam",
            DocEdit::InsertNode { .. } => "InsertNode",
            _ => "other",
        })
        .collect();
    eprintln!(
        "P5 first pattern edits: {kinds:?} / {:?}",
        outcome.committed
    );
    assert!(!kinds.contains(&"SetParam"), "{kinds:?}");
}

/// P6: Place on a body already placed is a second placement of it.
#[test]
fn p6_place_on_a_placed_body_is_a_second_placement() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let (mut session, _copy) = common::world_copy_in(&session, body, tol);
    let outcome = session.perform(SessionOp::Place { body: body.into() });
    eprintln!(
        "P6 refusal: {:?}",
        outcome.refusal.as_ref().map(|r| r.to_string())
    );
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
}
