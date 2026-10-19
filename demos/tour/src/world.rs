//! **Placements**: every copy of a body the tour puts somewhere is a
//! `Node::place` of it, constrained by what one of the placement doors
//! computes over the body's own geometry —
//!
//! - [`place_in_world`]: the copy in the world where it stands
//!   ([`stand`]: a plane mate on a face no variable reaches, or a full
//!   revolve's own axis, with constant-zero values), so it follows
//!   every variable its body reads;
//! - [`in_world`]: the copy in the world at a pose ([`place_at`]);
//! - [`against`]: the copy placed against another body's geometry at a
//!   frame in that body's own coordinates ([`place_against`]) — how a
//!   construction combined with another joins its space.
//!
//! A placement is a sequence of edits applied in order (the insert, the
//! mate, one value per freedom the mate leaves); [`Placed::edits`] is
//! that sequence, so a scene that exports its edit log replays exactly
//! what it applied.

use pncad::document::pose::{PoseDef, PoseRole, RoleRead};
use pncad::document::{
    BodyRead, Doc, DocEdit, EvalOptions, Frame, Node, Operand, ProfileProgram, RecipeNodeId, Value,
    apply, frame_at, place_against, place_at, stand,
};
use pncad::document::{Coord, Formula, Mate, Target};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;

/// What a placement left: the document, the placement it minted, and
/// the edits it applied, in order.
pub struct Placed {
    pub doc: Doc<ProfileProgram>,
    pub placement: RecipeNodeId,
    pub edits: Vec<DocEdit<ProfileProgram>>,
}

/// One copy of `body` in the world where it stands ([`stand`]).
///
/// # Panics
///
/// If the copy offers no geometry to stand on, or an edit refuses.
pub fn place_in_world(doc: &Doc<ProfileProgram>, body: impl Into<Operand>, tol: Tol) -> Placed {
    let body = body.into();
    let placed = stand(doc, &body, &EvalOptions::default(), tol)
        .unwrap_or_else(|e| panic!("the copy stands over its geometry: {e}"));
    written(doc, body, placed.mate, placed.values, tol)
}

/// One copy of `body` in the world at `at` ([`place_at`]; at the
/// identity, [`place_in_world`]).
///
/// # Panics
///
/// If the copy offers no geometry to place by, or an edit refuses.
pub fn in_world(
    doc: &Doc<ProfileProgram>,
    body: impl Into<Operand>,
    at: &Frame,
    tol: Tol,
) -> Placed {
    if *at == Frame::IDENTITY {
        return place_in_world(doc, body, tol);
    }
    let body = body.into();
    let placed = place_at(doc, &body, at, &EvalOptions::default(), tol)
        .unwrap_or_else(|e| panic!("the copy places at its pose: {e}"));
    written(doc, body, placed.mate, placed.values, tol)
}

/// [`in_world`] by the "place part" gesture ([`place_at`]) at every
/// pose, the identity included, its values rewritten by `values` before
/// they are written (a spin read off a variable, say).
///
/// # Panics
///
/// As [`in_world`]'s.
pub fn in_world_valued(
    doc: &Doc<ProfileProgram>,
    body: impl Into<Operand>,
    at: &Frame,
    values: impl FnOnce(Vec<(Coord, Formula)>) -> Vec<(Coord, Formula)>,
    tol: Tol,
) -> Placed {
    let body = body.into();
    let placed = place_at(doc, &body, at, &EvalOptions::default(), tol)
        .unwrap_or_else(|e| panic!("the copy places at its pose: {e}"));
    written(doc, body, placed.mate, values(placed.values), tol)
}

/// A copy of `body` placed against `target`'s geometry at `at` — the
/// copy's own frame in `target`'s own coordinates ([`place_against`]).
/// The placement's output is the copy, in `target`'s space; a target
/// that is itself a placed copy carries `at` by its motion.
///
/// # Panics
///
/// If either offers no geometry to place by, or an edit refuses.
pub fn against(
    doc: &Doc<ProfileProgram>,
    target: impl Into<Operand>,
    body: impl Into<Operand>,
    at: &Frame,
    tol: Tol,
) -> Placed {
    against_valued(doc, target, body, at, |values| values, tol)
}

/// [`against`], its values rewritten by `values` before they are
/// written (a joint's spin read off a variable, say). The values are
/// charted on the two bodies' own frames.
///
/// # Panics
///
/// As [`against`]'s.
pub fn against_valued(
    doc: &Doc<ProfileProgram>,
    target: impl Into<Operand>,
    body: impl Into<Operand>,
    at: &Frame,
    values: impl FnOnce(Vec<(Coord, Formula)>) -> Vec<(Coord, Formula)>,
    tol: Tol,
) -> Placed {
    let (target, body) = (target.into(), body.into());
    let placed = place_against(doc, &target, &body, at, &EvalOptions::default(), tol)
        .unwrap_or_else(|e| panic!("the copy places against its target: {e}"));
    written(doc, body, placed.mate, values(placed.values), tol)
}

/// **The frame at `origin` with axes `u` and `v`** (`v` made orthogonal
/// to `u`, both normalised; the normal `u × v`).
///
/// # Panics
///
/// If `u` and `v` span no plane.
pub fn frame(origin: [f64; 3], u: [f64; 3], v: [f64; 3]) -> Frame {
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let unit = |a: [f64; 3]| {
        let n = dot(a, a).sqrt();
        assert!(n > 0.0, "the frame's axes span a plane");
        a.map(|c| c / n)
    };
    let x = unit(u);
    let along = dot(v, x);
    let y = unit([
        v[0] - x[0] * along,
        v[1] - x[1] * along,
        v[2] - x[2] * along,
    ]);
    let z = [
        x[1] * y[2] - x[2] * y[1],
        x[2] * y[0] - x[0] * y[2],
        x[0] * y[1] - x[1] * y[0],
    ];
    Frame {
        columns: [x, y, z],
        translation: origin,
    }
}

/// **The plane through `origin` with normal `normal`, read off `body`'s
/// own geometry** ([`frame_at`] through its `xy`): a plane written in
/// the body's own coordinates, as a split's tool reads one.
///
/// # Panics
///
/// If `normal` is zero or `body` offers no frame to read.
pub fn plane_on(
    doc: &Doc<ProfileProgram>,
    body: impl Into<Operand>,
    origin: [f64; 3],
    normal: [f64; 3],
    tol: Tol,
) -> RoleRead<Operand> {
    let seed = if normal[0].abs() <= normal[1].abs().max(normal[2].abs()) {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let nn = normal.iter().map(|c| c * c).sum::<f64>();
    let k = (seed[0] * normal[0] + seed[1] * normal[1] + seed[2] * normal[2]) / nn;
    let u = [0, 1, 2].map(|i| seed[i] - normal[i] * k);
    let v = [
        normal[1] * u[2] - normal[2] * u[1],
        normal[2] * u[0] - normal[0] * u[2],
        normal[0] * u[1] - normal[1] * u[0],
    ];
    let at = frame(origin, u, v);
    let read = frame_at(doc, &body.into(), &at, &EvalOptions::default(), tol)
        .unwrap_or_else(|e| panic!("the body offers a frame to read the plane off: {e}"));
    RoleRead::through(read, PoseRole::Xy)
}

/// **The axis through `origin` along `dir`, read off `body`'s own
/// geometry** ([`frame_at`] through its `z`): a pattern's axis written in
/// the body's own coordinates.
///
/// # Panics
///
/// As [`plane_on`]'s.
pub fn axis_on(
    doc: &Doc<ProfileProgram>,
    body: impl Into<Operand>,
    origin: [f64; 3],
    dir: [f64; 3],
    tol: Tol,
) -> RoleRead<Operand> {
    RoleRead::through(plane_on(doc, body, origin, dir, tol).read, PoseRole::Z)
}

/// **A copy of `body` placed against `target` by its own axis**, the
/// axis of `body`'s face `face` (a tube's torus, whose axis is the
/// tube's own `z`), mated to the `z` of a frame read off `target`
/// standing at `at` ([`frame_at`]) — the axis form of [`against`], for a
/// copy whose own axis the door does not find (a tube with a window).
/// The copy's own frame is `at` up to a turn about that axis, so `at`
/// is a translation and the two values the mate leaves are zero.
///
/// # Panics
///
/// If `at` turns, or a door refuses.
pub fn against_on_axis(
    doc: &Doc<ProfileProgram>,
    target: impl Into<Operand>,
    body: impl Into<Operand>,
    face: StableName,
    at: &Frame,
    tol: Tol,
) -> Placed {
    assert_eq!(
        at.columns,
        Frame::IDENTITY.columns,
        "an axis placement written by translation alone"
    );
    let (target, body) = (target.into(), body.into());
    let frame = frame_at(doc, &target, at, &EvalOptions::default(), tol)
        .unwrap_or_else(|e| panic!("the target offers a frame to read: {e}"));
    let mate = Mate {
        on: Operand::Pose(Box::new(PoseDef::Axis {
            of: Operand::select(body.clone(), vec![face]),
        })),
        to: Target::Pose(Operand::Pose(Box::new(PoseDef::Slide {
            pose: RoleRead::through(frame.clone(), PoseRole::Z),
            along: RoleRead::through(frame, PoseRole::Z),
            by: Formula::zero(pncad::document::Dimension::Length),
        }))),
        class: pncad::select::ContactClass::Rest,
    };
    let values = vec![
        (
            Coord::Slide(pncad::document::SlideAxis::Z),
            Formula::zero(pncad::document::Dimension::Length),
        ),
        (
            Coord::Spin,
            Formula::zero(pncad::document::Dimension::Angle),
        ),
    ];
    written(doc, body, mate, values, tol)
}

/// The placement of `body` written as edits: the insert, the mate, and
/// each value against the mate's minted id.
fn written(
    doc: &Doc<ProfileProgram>,
    body: Operand,
    mate: Mate<Formula>,
    values: Vec<(Coord, Formula)>,
    tol: Tol,
) -> Placed {
    let mut edits = Vec::new();
    let mut step = |doc: &Doc<ProfileProgram>, edit: DocEdit<ProfileProgram>| {
        let applied = apply(doc, &edit, tol)
            .unwrap_or_else(|err| panic!("the placement's edit applies: {err}"));
        edits.push(edit);
        applied
    };
    let inserted = step(
        doc,
        DocEdit::InsertNode {
            node: Box::new(Node::place(vec![BodyRead::plain(body)])),
            fresh: Vec::new(),
        },
    );
    let placement = inserted.record.minted.expect("an insert mints an id");
    let mated = step(
        &inserted.doc,
        DocEdit::AddConstraint {
            placement,
            mate: Box::new(mate),
            fresh: Vec::new(),
        },
    );
    let mate = mated.record.minted_constraints[0];
    let doc = values.into_iter().fold(mated.doc, |doc, (coord, by)| {
        step(
            &doc,
            DocEdit::AddValue {
                placement,
                value: Box::new(Value { mate, coord, by }),
                fresh: Vec::new(),
            },
        )
        .doc
    });
    Placed {
        doc,
        placement,
        edits,
    }
}
