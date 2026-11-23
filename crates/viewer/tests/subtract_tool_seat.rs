//! **The boolean tool's subtraction holds a tool list** (DESIGN
//! `[repetition]`, REFERENCES DM4): a pattern picked whole in the tree
//! at the tool seat is every member, so a plate minus a pattern of two
//! holes is one plate with two holes; a viewport pick on a drawn copy
//! is that member; and the panel says "every member" at the tool seat.
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use common::len;
use pncad::document::{BodyRead, BooleanValue, Doc, Formula, RecipeNodeId, Speaker};
use pncad::geom_core::Tol;
use pncad::prelude::ValuePayload;
use pncad::topo::BooleanOp;
use viewer::combine::BooleanTool;
use viewer::session::{BooleanSpec, DocSession, PatternRuleSpec, SessionOp};

/// The plate: x in [-0.04, 0.04], y in [-0.02, 0.02], z in [0, 0.01].
const PLATE: [f64; 3] = [0.08, 0.04, 0.01];
/// The hole, slid to x in [-0.025, -0.015] and z in [-0.01, 0.02]: a
/// 0.01 square through the plate. Its pattern's second member stands
/// 0.04 along x.
const HOLE: [f64; 3] = [0.01, 0.01, 0.03];
const HOLE_AT: [f64; 3] = [-0.02, 0.0, -0.01];
const C_VOL: f64 = PLATE[0] * PLATE[1] * PLATE[2];
/// What one hole takes out of the plate.
const OVERLAP: f64 = 0.01 * 0.01 * 0.01;

/// The plate and a pattern of two holes in its space.
fn plate_and_holes(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let mut session =
        common::inline_without_door(Doc::empty_derived("subtract-tool-seat", tol), tol);
    let plate = common::xy_box_in(&mut session, PLATE);
    let raw = common::xy_box_in(&mut session, HOLE);
    let (doc, hole) = crate::fixture::slid(session.committed_doc().clone(), plate, raw, HOLE_AT);
    let mut session = common::inline_without_door(doc, tol);
    let holes = common::session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: hole.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: len(0.04),
            },
        },
    );
    (session, plate, holes)
}

/// The volume of the one body `node` evaluated to, or a panic naming
/// what it evaluated to instead.
fn one_body(session: &mut DocSession, node: RecipeNodeId, tol: Tol) -> f64 {
    session.pump();
    let eval = session.evaluation().expect("landed");
    match eval.value(node).map(|v| &v.payload) {
        Some(ValuePayload::Boolean(BooleanValue::Body { .. })) => {}
        other => panic!("{node} is one body: {other:?}"),
    }
    common::body_volume(session, node, tol)
}

/// **A plate minus a pattern of two holes picked whole is one plate
/// with two holes**: the tree pick at the tool seat reads the pattern
/// whole, the panel says so, and the cut is `C_VOL − 2·OVERLAP`. Red if
/// the seat reads a family whole one member at a time (two plates of
/// one hole each).
#[test]
fn a_plate_minus_a_pattern_picked_whole_is_one_plate_with_two_holes() {
    let tol = Tol::witness();
    let (mut session, plate, holes) = plate_and_holes(tol);
    let doc = session.committed_doc().clone();
    let mut tool = BooleanTool::new();
    tool.set_operation(BooleanOp::Subtract);
    tool.pick(&doc, plate);
    tool.pick(&doc, holes);
    let line = tool.line(Speaker::of(&doc));
    assert!(
        line.contains(", every member"),
        "the tool seat takes the pattern whole: {line}"
    );
    let spec = tool.spec().expect("seated");
    assert_eq!(
        spec,
        BooleanSpec::Subtract {
            from: plate.into(),
            tool: vec![holes.into()],
        },
        "the pattern is read plainly, whole"
    );
    let made = common::session_insert(&mut session, SessionOp::AddBoolean { spec });
    let v = one_body(&mut session, made, tol);
    assert!(
        (v - (C_VOL - 2.0 * OVERLAP)).abs() < 1e-12,
        "one plate with two holes: {v:e}, against {:e}",
        C_VOL - 2.0 * OVERLAP
    );
}

/// **A viewport pick on a drawn copy is that member**, and a further
/// pick adds a tool rather than
/// replacing one: the plate cut by member 1 and then by the pattern
/// whole is the plate with both holes.
#[test]
fn a_viewport_pick_on_a_copy_is_that_member_and_a_further_pick_adds_a_tool() {
    let tol = Tol::witness();
    let (mut session, plate, holes) = plate_and_holes(tol);
    let doc = session.committed_doc().clone();
    let mut tool = BooleanTool::new();
    tool.set_operation(BooleanOp::Subtract);
    tool.pick(&doc, plate);
    tool.pick_at(&doc, holes, Some(1));
    tool.pick(&doc, holes);
    let member = BodyRead::indexed(holes, vec![Formula::count(1)]);
    let spec = tool.spec().expect("seated");
    assert_eq!(
        spec,
        BooleanSpec::Subtract {
            from: plate.into(),
            tool: vec![member, holes.into()],
        },
        "the drawn copy's member, then the pattern whole"
    );
    let line = tool.line(Speaker::of(&doc));
    assert!(
        line.contains("body removed 1:") && line.contains("body removed 2:"),
        "two tools, each by its place: {line}"
    );
    let made = common::session_insert(&mut session, SessionOp::AddBoolean { spec });
    let v = one_body(&mut session, made, tol);
    assert!(
        (v - (C_VOL - 2.0 * OVERLAP)).abs() < 1e-12,
        "a hole cut twice is cut once: {v:e}"
    );
}
