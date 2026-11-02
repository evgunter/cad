//! **Where a drawn copy stands**: a placement pinned at a pose, and
//! the corners, extent and volume of what a copy draws, read off the
//! landed evaluation.

use pncad::document::{Doc, EvalOptions, Frame, ProfileProgram, RecipeNodeId};
use pncad::geom_core::Tol;
use pncad::prelude::ValuePayload;
use viewer::session::DocSession;

/// `doc` with `placement` pinned at `pose`, by values alone
/// ([`crate::fixture::pin`]).
pub fn pinned(
    doc: Doc<ProfileProgram>,
    placement: RecipeNodeId,
    pose: &Frame,
) -> Doc<ProfileProgram> {
    crate::fixture::pin(doc, placement, pose, &EvalOptions::default())
}

/// The frame translated by `at`.
pub fn translated(at: [f64; 3]) -> Frame {
    Frame {
        translation: at,
        ..Frame::IDENTITY
    }
}

/// **`body`'s one world copy pinned at `pose`**: a landed session over
/// `session`'s document with that copy's bundle replaced, and the copy.
/// A body nothing copies is placed into the world by hand first
/// ([`super::world_copy`]).
pub fn moved_copy(
    session: &DocSession,
    body: RecipeNodeId,
    pose: &Frame,
    tol: Tol,
) -> (DocSession, RecipeNodeId) {
    let (doc, copy) = super::world_copy(session.committed_doc(), body, tol);
    let mut session = super::inline_without_door(pinned(doc, copy, pose), tol);
    session.pump();
    (session, copy)
}

/// `node`'s evaluated body's points, sorted.
pub fn corners(session: &mut DocSession, node: RecipeNodeId) -> Vec<[f64; 3]> {
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    let value = eval
        .value(node)
        .unwrap_or_else(|| panic!("{node:?} evaluated: {:?}", eval.node_error(node)));
    let ValuePayload::Body(body) = &value.payload else {
        panic!("{node:?} is a body, not {:?}", value.payload)
    };
    let mut points: Vec<[f64; 3]> = crate::fixture::in_space(body)
        .points()
        .map(|(_, p)| p.to_array())
        .collect();
    points.sort_by(|a, b| a.partial_cmp(b).expect("finite points"));
    points
}

/// The least and greatest corner of `points`.
pub fn bounds(points: &[[f64; 3]]) -> ([f64; 3], [f64; 3]) {
    points.iter().fold(
        ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]),
        |(lo, hi), p| {
            (
                [lo[0].min(p[0]), lo[1].min(p[1]), lo[2].min(p[2])],
                [hi[0].max(p[0]), hi[1].max(p[1]), hi[2].max(p[2])],
            )
        },
    )
}

/// The box the drawn copy `copy` spans, least and greatest corner.
pub fn extent(session: &mut DocSession, copy: RecipeNodeId) -> ([f64; 3], [f64; 3]) {
    bounds(&corners(session, copy))
}

/// **What the drawn copy `copy` shows is the box `lo`..`hi` of volume
/// `volume`**: its extent to 1e-12 m, its volume to one part in 10⁹.
pub fn assert_drawn(
    session: &mut DocSession,
    copy: RecipeNodeId,
    lo: [f64; 3],
    hi: [f64; 3],
    volume: f64,
    what: &str,
) {
    let (got_lo, got_hi) = extent(session, copy);
    let near = |a: [f64; 3], b: [f64; 3]| a.iter().zip(&b).all(|(x, y)| (x - y).abs() <= 1e-12);
    assert!(
        near(got_lo, lo) && near(got_hi, hi),
        "{what}: drawn over {got_lo:?}..{got_hi:?}, want {lo:?}..{hi:?}"
    );
    let got = super::body_volume(session, copy, Tol::witness());
    assert!(
        super::near(got, volume),
        "{what}: drawn volume {got}, want {volume}"
    );
}

/// **The copy `on` is a face of, rested on the plane of `to`** through
/// the mate tool's own flow: a plane mate, flipped so the two faces
/// meet, committed and landed.
pub fn rest_on(
    session: &mut DocSession,
    on: viewer::session::FaceSelection,
    to: viewer::session::FaceSelection,
) {
    let mut mate = viewer::matetool::MateTool::new();
    mate.pick(session.doc(), on);
    mate.pick(session.doc(), to);
    let proposal = super::proposed(
        session,
        &mate,
        viewer::matetool::MateChoice {
            kind: pncad::document::VarKind::Plane,
            flip: true,
            values: Vec::new(),
        },
    )
    .expect("a rest proposes");
    super::commit_mate(session, proposal.op());
}

/// A duplicate's step: the length of the slides its stepped placement
/// moves the copy by, along `viewer::combine::STEP_DIRECTION`, which
/// the door authors literal.
pub fn spacing_of(session: &DocSession, copy: RecipeNodeId) -> f64 {
    use pncad::document::{Coord, Expr, Node};
    let doc = session.committed_doc();
    let Some(Node::Place { constraints, .. }) = doc.node(copy) else {
        panic!("a duplicate is a stepped placement");
    };
    let slides: Vec<f64> = constraints
        .iter()
        .filter_map(|c| c.value())
        .filter(|value| matches!(value.coord, Coord::Slide(_)))
        .map(|value| doc.written(&Expr::var(value.by, value.coord.dimension())))
        .filter(|written| !written.is_zero())
        .map(|written| {
            written
                .literal_value()
                .unwrap_or_else(|| panic!("the door authors a literal step: {written:?}"))
        })
        .collect();
    match slides[..] {
        [one] => one.abs(),
        _ => slides.iter().map(|s| s * s).sum::<f64>().sqrt(),
    }
}

/// **A box of `size` split at half its height**, landed: the session,
/// the box (its one world copy still standing whole at the identity)
/// and the split.
pub fn split_box(size: [f64; 3], tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let mut session = super::inline_without_door(Doc::empty_derived("split-halves", tol), tol);
    let body = super::xy_box_in(&mut session, size);
    session.pump();
    let split = super::session_insert(
        &mut session,
        viewer::session::SessionOp::AddSplit {
            target: body.into(),
            origin: super::len3([0.0, 0.0, size[2] / 2.0]),
            normal: super::scl3([0.0, 0.0, 1.0]),
        },
    );
    session.pump();
    (session, body, split)
}

/// **One placement into the world drawing `half` of `split`**, pinned
/// at `pose`: the landed session and the placement.
pub fn half_placed(
    session: &DocSession,
    split: RecipeNodeId,
    half: pncad::select::SplitHalf,
    pose: &Frame,
    tol: Tol,
) -> (DocSession, RecipeNodeId) {
    use pncad::document::{BodyRead, Node, Operand, VarId};
    let (doc, placement) = crate::fixture::insert(
        session.committed_doc().clone(),
        Node::place(
            BodyRead::plain(Operand::output(split, half.port())),
            Operand::Var(VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
    );
    let mut session = super::inline_without_door(pinned(doc, placement, pose), tol);
    session.pump();
    (session, placement)
}
