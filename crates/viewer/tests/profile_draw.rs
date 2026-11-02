//! **What a committed profile draws** (`viewer::sketch::committed`):
//! the loops of every profile node the landed evaluation validated,
//! run against a real document and a real evaluation with no renderer.
//!
//! What these rows hold is where the drawing comes from — the VALUE,
//! on the profile's own plane — and the two ways a profile
//! node draws nothing: its evaluation refused, or a form is editing it
//! and draws it itself.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::{inserted, rectangle};
use pncad::document::{
    CancelToken, Doc, EvalOptions, Evaluation, ProfileProgram, RecipeNodeId, evaluate,
};
use pncad::geom_core::{Point2, Tol};
use viewer::sketch;

/// The flattening tolerance, in metres. A square has no arcs, so the
/// rows here do not depend on it.
const CHORD: f64 = 1.0e-4;

/// The side of every square these rows draw, in metres.
const SIDE: f64 = 0.02;

/// A document with a square profile of `SIDE` at `origin` on its own
/// plane added, answering the document and the profile's id.
fn square_at(
    doc: &Doc<ProfileProgram>,
    origin: [f64; 2],
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    inserted(doc, rectangle(origin, SIDE, SIDE), tol)
}

fn evaluated(doc: &Doc<ProfileProgram>, tol: Tol) -> Evaluation<f64> {
    evaluate(
        doc,
        None,
        &CancelToken::default(),
        &EvalOptions::default(),
        tol,
    )
}

/// **A committed profile is drawn from its landed value, where the
/// value puts it.**
///
/// The value is on the profile's own plane, so every corner is drawn
/// at its own z = 0. The loop is the square's four corners, closed:
/// the value has no open chain to leave a leg out of.
#[test]
fn a_committed_profile_is_drawn_on_the_plane_its_value_carries() {
    let tol = Tol::witness();
    let doc = Doc::empty_derived("profile-draw", tol);
    let (doc, profile) = square_at(&doc, [0.0, 0.0], tol);
    let evaluation = evaluated(&doc, tol);

    let drawn = sketch::committed(&doc, &evaluation, CHORD, None);
    assert!(drawn.undrawn.is_empty(), "{:?}", drawn.undrawn);
    assert_eq!(
        drawn.drawn.iter().map(|p| p.node).collect::<Vec<_>>(),
        vec![profile],
        "the profile and only the profile",
    );
    let committed = &drawn.drawn[0];
    assert_eq!(committed.loops.len(), 1);
    let square = &committed.loops[0];
    assert!(square.end.closes(), "a validated loop is closed");
    // The canonical form starts at the authored vertex and runs
    // counterclockwise, which for this square is the order it was
    // authored in.
    assert_eq!(
        square.points,
        vec![[0.0, 0.0], [SIDE, 0.0], [SIDE, SIDE], [0.0, SIDE]],
    );
    for [x, y] in &square.points {
        let world = committed.plane.to_world(Point2::new(*x, *y));
        assert!(
            world.z.abs() < 1.0e-12,
            "a corner of the square is drawn at z = {}, off its own plane",
            world.z,
        );
    }
}

/// **The profile a form edits is left to the form.** Two profiles
/// are committed; excepting one leaves exactly the other, and
/// excepting nothing draws both — so the exception is what removed
/// the one, and not a pass that draws nothing.
#[test]
fn the_profile_being_edited_is_the_one_left_out() {
    let tol = Tol::witness();
    let doc = Doc::empty_derived("profile-draw-except", tol);
    let (doc, first) = square_at(&doc, [0.0, 0.0], tol);
    let (doc, second) = square_at(&doc, [0.1, 0.0], tol);
    let evaluation = evaluated(&doc, tol);
    let nodes = |except| {
        sketch::committed(&doc, &evaluation, CHORD, except)
            .drawn
            .iter()
            .map(|p| p.node)
            .collect::<Vec<_>>()
    };
    assert_eq!(nodes(None), vec![first, second], "document order");
    assert_eq!(nodes(Some(first)), vec![second]);
    assert_eq!(nodes(Some(second)), vec![first]);
}

/// **A profile whose evaluation refused draws nothing** — and is not
/// counted as undrawn either, because that count is about a VALUE the
/// viewport could not draw, and a refused node has none: the tree's
/// badge says why it is missing.
///
/// The refused profile is a circle whose radius reads a variable,
/// written negative once the profile is in the document, which the
/// evaluator refuses, so the profile has no value. A healthy profile
/// beside it is the control: the pass still draws what did evaluate.
#[test]
fn a_profile_whose_evaluation_refused_draws_nothing() {
    use pncad::document::{
        Dimension, DocEdit, Formula, FreeValue, FreeVar, LoopProgram, Node, VarName,
    };
    let tol = Tol::witness();
    let r = VarName::from_static("r");
    let doc = common::declared(
        "profile-draw-refused",
        &r,
        FreeVar::continuous(Dimension::Length, 0.005),
        tol,
    );
    let (doc, refused) = inserted(
        &doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [
                    Formula::zero(Dimension::Length),
                    Formula::zero(Dimension::Length),
                ],
                radius: Formula::named(r.clone(), Dimension::Length),
            }],
            ids: Vec::new(),
            on: None,
        }),
        tol,
    );
    let var = doc.var_named(r.as_str()).expect("declared");
    let (doc, _) = common::edited(
        &doc,
        DocEdit::SetVarValue {
            var: var.into(),
            value: FreeValue::Continuous(-0.005),
        },
        tol,
    );
    let (doc, healthy) = square_at(&doc, [0.0, 0.0], tol);
    let evaluation = evaluated(&doc, tol);
    assert!(
        evaluation.value(refused).is_none(),
        "the fixture: a circle of negative radius has no value",
    );

    let drawn = sketch::committed(&doc, &evaluation, CHORD, None);
    assert_eq!(
        drawn.drawn.iter().map(|p| p.node).collect::<Vec<_>>(),
        vec![healthy],
    );
    assert!(drawn.undrawn.is_empty(), "{:?}", drawn.undrawn);
}

/// **A bulge near the bottom of the exponent range draws as the line
/// validation reads it.** The arc's radius overflows, but validation
/// classifies a sub-tolerance arc a `Line` and keeps no carrier, and
/// the committed pass draws the validated classification — so the
/// loop draws, straight, and nothing is counted undrawn.
#[test]
fn a_validated_sub_tolerance_arc_draws_as_its_line() {
    use pncad::profile::{ArcData, Step, Target};
    use viewer::session::{DocSession, ProfileShape, SessionOp};

    let tol = Tol::witness();
    let mut session = DocSession::inline(Doc::empty_derived("profile-draw-undrawable", tol), tol);
    let template = ProfileShape::Path {
        steps: vec![
            Step::At(Point2::new(0.0, 0.0)),
            Step::ArcTo(ArcData::Bulge {
                target: Target::Point(Point2::new(0.01, 0.0)),
                b: 1.0e-320,
            }),
            Step::LineTo(Target::Point(Point2::new(0.005, 0.01))),
            Step::LineTo(Target::Start),
        ],
    };
    let profile = common::session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![common::shape(&template)],
        },
    );
    session.pump();
    let (doc, evaluation) = session.landed_pair().expect("the inline seam landed");
    assert!(
        matches!(
            evaluation.value(profile).map(|value| &value.payload),
            Some(pncad::document::ValuePayload::Profile(_))
        ),
        "the fixture: the evaluator validates this profile, so the flattener is what refuses",
    );
    // Validation classifies the sub-tolerance arc a line, and the
    // committed pass draws the validated classification — the loop
    // sweeps consume — so the leg draws straight.
    let drawn = sketch::committed(doc, evaluation, CHORD, None);
    assert_eq!(
        drawn.drawn.iter().map(|p| p.node).collect::<Vec<_>>(),
        vec![profile],
        "the validated loop draws"
    );
    assert!(drawn.undrawn.is_empty(), "{:?}", drawn.undrawn);
}
