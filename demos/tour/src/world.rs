//! **One copy of a body placed in the world** — the "place part"
//! gesture, spelled through the façade's own doors: a loose
//! `Node::place` of the body, then `place_at`'s constraints added to
//! its bundle — a plane mate on the copy's first planar face to the
//! world plane that face lies in at the pose, and the three values
//! that put the copy at the pose in that mate's chart (or, for a copy
//! with no planar face, the pin `place_at` falls back to).
//!
//! A mate's values name it by the id its own edit mints, so the
//! gesture is four edits applied in order, each against the document
//! the one before it left; [`Placed::edits`] is that sequence, so a
//! scene that exports its edit log replays exactly what it applied.

use pncad::document::{
    BodyRead, Doc, DocEdit, EvalOptions, Formula, Frame, Node, Operand, ProfileProgram,
    RecipeNodeId, RefusingReach, Value, apply, place_at,
};
use pncad::geom_core::Tol;

/// What the gesture left: the document, the placement it minted, and
/// the edits it applied, in order.
pub struct Placed {
    pub doc: Doc<ProfileProgram>,
    pub placement: RecipeNodeId,
    pub edits: Vec<DocEdit<ProfileProgram>>,
}

/// One copy of `body` placed in the world where it stands (the
/// identity), its copy evaluated under the default options.
pub fn place_in_world(
    doc: &Doc<ProfileProgram>,
    body: impl Into<BodyRead<Formula>>,
    tol: Tol,
) -> Placed {
    place_at_pose(doc, body, &Frame::IDENTITY, &EvalOptions::default(), tol)
}

/// One copy of `body` placed in the world at `pose`, its copy
/// evaluated under `opts` (an instance's copy needs the store's
/// resolver).
///
/// # Panics
///
/// On any refusal of the gesture or its edits, naming it: a demo whose
/// copy does not place is a demo with a bug in it.
pub fn place_at_pose(
    doc: &Doc<ProfileProgram>,
    body: impl Into<BodyRead<Formula>>,
    pose: &Frame,
    opts: &EvalOptions,
    tol: Tol,
) -> Placed {
    let mut edits = Vec::new();
    let mut step = |doc: &Doc<ProfileProgram>, edit: DocEdit<ProfileProgram>| {
        let applied = apply(doc, &edit, tol, &RefusingReach)
            .unwrap_or_else(|err| panic!("the placement's edit applies: {err}"));
        edits.push(edit);
        applied
    };
    let inserted = step(
        doc,
        DocEdit::InsertNode {
            node: Box::new(Node::place(vec![body.into()])),
            fresh: Vec::new(),
        },
    );
    let placement = inserted.record.minted.expect("an insert mints an id");
    let shape = match inserted.doc.node(placement) {
        Some(Node::Place { shapes, .. }) => shapes[0].read,
        other => panic!("a placement was inserted, got {other:?}"),
    };
    let placed = place_at(&inserted.doc, &Operand::Var(shape), pose, opts, tol)
        .unwrap_or_else(|refusal| panic!("the copy places over its geometry: {refusal}"));
    let mated = step(
        &inserted.doc,
        DocEdit::AddConstraint {
            placement,
            mate: Box::new(placed.mate),
            fresh: Vec::new(),
        },
    );
    let mate = mated.record.minted_constraints[0];
    let mut doc = mated.doc;
    for (coord, by) in placed.values {
        doc = step(
            &doc,
            DocEdit::AddValue {
                placement,
                value: Box::new(Value { mate, coord, by }),
                fresh: Vec::new(),
            },
        )
        .doc;
    }
    Placed {
        doc,
        placement,
        edits,
    }
}
