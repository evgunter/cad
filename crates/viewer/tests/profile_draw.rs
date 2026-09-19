//! **What a committed profile draws** (`viewer::sketch::committed`):
//! the loops of every profile node the landed evaluation validated,
//! run against a real document and a real evaluation with no renderer.
//!
//! What these rows hold is where the drawing comes from — the VALUE,
//! placed on the plane the value carries — and the two ways a profile
//! node draws nothing: its evaluation refused, or a form is editing it
//! and draws it itself.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::{frame, inserted, square};
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

/// A document holding one frame at `origin` (world x and y as its
/// axes) with a square profile on it, answering the document and the
/// profile's id.
fn square_at(
    doc: &Doc<ProfileProgram>,
    origin: [f64; 3],
    u: [f64; 3],
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let (doc, plane) = inserted(doc, frame(origin, u, [0.0, 1.0, 0.0]), tol);
    inserted(&doc, square(plane, SIDE), tol)
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
/// The frame sits half a metre up the z axis, so a drawing placed on
/// a default plane — or on none — would land at z = 0 and red the
/// placement half. The loop is the square's four corners, closed:
/// the value has no open chain to leave a leg out of.
#[test]
fn a_committed_profile_is_drawn_on_the_plane_its_value_carries() {
    let tol = Tol::witness();
    let doc = Doc::empty_derived("profile-draw", tol);
    let (doc, profile) = square_at(&doc, [0.0, 0.0, 0.5], [1.0, 0.0, 0.0], tol);
    let evaluation = evaluated(&doc, tol);

    let drawn = sketch::committed(&doc, &evaluation, CHORD, None);
    assert!(drawn.undrawn.is_empty(), "{:?}", drawn.undrawn);
    assert_eq!(
        drawn.drawn.iter().map(|p| p.node).collect::<Vec<_>>(),
        vec![profile],
        "the profile and only the profile: the frame is a datum, drawn elsewhere",
    );
    let committed = &drawn.drawn[0];
    assert_eq!(committed.loops.len(), 1);
    let square = &committed.loops[0];
    assert!(square.closed, "a validated loop is closed");
    // The canonical form starts at the lex-min vertex and runs
    // counterclockwise, which for this square is the order it was
    // authored in.
    assert_eq!(
        square.points,
        vec![[0.0, 0.0], [SIDE, 0.0], [SIDE, SIDE], [0.0, SIDE]],
    );
    for [x, y] in &square.points {
        let world = committed.plane.to_world(Point2::new(*x, *y));
        assert!(
            (world.z - 0.5).abs() < 1.0e-12,
            "a corner of the square is drawn at z = {}, off its frame",
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
    let (doc, first) = square_at(&doc, [0.0; 3], [1.0, 0.0, 0.0], tol);
    let (doc, second) = square_at(&doc, [0.0, 0.0, 0.1], [1.0, 0.0, 0.0], tol);
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
/// The frame's two axes are parallel, which the evaluator refuses, so
/// the profile on it has no plane and no value. A healthy profile
/// beside it is the control: the pass still draws what did evaluate.
#[test]
fn a_profile_whose_evaluation_refused_draws_nothing() {
    let tol = Tol::witness();
    let doc = Doc::empty_derived("profile-draw-refused", tol);
    let (doc, refused) = square_at(&doc, [0.0; 3], [0.0, 1.0, 0.0], tol);
    let (doc, healthy) = square_at(&doc, [0.0; 3], [1.0, 0.0, 0.0], tol);
    let evaluation = evaluated(&doc, tol);
    assert!(
        evaluation.value(refused).is_none(),
        "the fixture: a profile on a degenerate frame has no value",
    );

    let drawn = sketch::committed(&doc, &evaluation, CHORD, None);
    assert_eq!(
        drawn.drawn.iter().map(|p| p.node).collect::<Vec<_>>(),
        vec![healthy],
    );
    assert!(drawn.undrawn.is_empty(), "{:?}", drawn.undrawn);
}
