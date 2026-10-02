//! **The bracket** — an L outline with one r = 0.5 tangent fillet at
//! its inner corner, extruded 0.75, as a recipe DOCUMENT.
//!
//! The outline is written once, in the PATHS algebra, and lifted into
//! a `Node::Profile` by [`LoopProgram::from_recorded`] — the seam
//! between the two authoring surfaces, and the spelling Python's
//! `Node.profile(outline)` takes. Frame, profile, extrude: that is the
//! document the gallery writes, and the body the tour renders is its
//! extrude node's value.
//!
//! **Oracle**: `V = 0.75 · (5 + r²(1 − π/4))`. The L is two 1-wide
//! legs of length 3 sharing a unit square, area 5, and rounding the
//! reflex corner ADDS the region between the corner and the arc.
//!
//! # The wall: the leg ends, trimmed flush and broken by name
//!
//! What this part wants next is both leg ends trimmed flush to ONE
//! plane, `x + y = 2.75`, and the cut edges broken. In the document
//! that is three nodes: [`Node::Split`] along the plane, [`Node::Part`]
//! keeping the corner piece (`Below`; the offcuts are `Above`), and
//! [`Node::Chamfer`] on that half's section chords.
//!
//! The split builds, and [`split_and_break`] asserts what it builds:
//! the halves partition the body (each offcut is a trapezoid prism of
//! area `(3 − 2.75) + 1/2`), and because the plane crosses BOTH legs
//! each cap face is crossed twice, so each of the corner piece's four
//! cap chords is named by its ends — `[SectionEdge, Fragment(Ends)]` —
//! and a selector reaches them by role path alone.
//!
//! The chamfer does not build. It refuses `UnsupportedRunOut`: a
//! plane–plane band ends only at a trivalent corner whose three edges
//! are all requested, and every corner of a section face carries a
//! body edge the selection does not name. That holds for any subset of
//! a split half's section edges, so no plane offset, setback or tilt
//! moves it; measured over the half (corner piece, offcuts), the
//! selection (the four cap chords, the four section edges across the
//! legs' side walls, all eight) and the verb (chamfer, fillet), the
//! corner piece refuses `UnsupportedRunOut` or, for all eight,
//! `ChainNotG1`, and the offcuts — one solid of two shells — refuse
//! `UnsupportedBody` before any chain is read
//! (`work/band/a-plane-plane-blend-cannot-end-at-an-unrequested-corner.md`,
//! `work/band/a-blend-refuses-a-solid-of-several-shells.md`). The
//! probe is live: when it builds, the document ends in the chamfer and
//! its oracle is the one written in [`split_and_break`].

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, SQRT_2};

use pncad::document::{NodeErrorKind, PartSelect, RefusingReach};
use pncad::geom_core::Tol;
use pncad::prelude::{
    BlendError, CancelToken, Datum, Dimension, Doc, DocEdit, EntityKind, EvalOptions, Evaluation,
    Expr, LoopProgram, NamePat, Node, Open, ProfileProgram, RecipeNodeId, SegPat, SegTag, Selector,
    SplitHalf, Start, ValuePayload, apply, evaluate, p2, select,
};
use pncad::profile::ClosedLoop;
use pncad::topo::{Body, mass_properties};

use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};

/// The fillet's radius at the inner corner.
const R: f64 = 0.5;
/// The extrusion depth.
const DEPTH: f64 = 0.75;
/// The trim plane is `x + y = CUT`: past both fillet tangent points
/// (`x + y = 2.5`) and short of both leg ends (`x + y = 3`), so it
/// crosses each leg once and the fillet not at all.
const CUT: f64 = 2.75;
/// The setback the wall's chamfer asks for.
const SETBACK: f64 = 0.1;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a length")
}
fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("a scalar")
}

/// The outline, in the PATHS algebra.
///
/// `fillet` computes the tangent points `(1.5, 1)` and `(1, 1.5)` and
/// declares both joints tangent. The corner `(1, 1)` is never
/// authored: `toward` fixes each RAY exactly (an `.angle(PI)` would
/// carry `sin(PI) = 1.22e-16` into both trim vertices), and the
/// filleted side ends at its far vertex through `.to(p)`.
fn outline(tol: Tol) -> ClosedLoop<f64> {
    Open.at(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0), tol)
        .expect("bracket base")
        .line_to(p2(3.0, 1.0), tol)
        .expect("bracket riser")
        .toward(-1.0, 0.0, tol)
        .expect("west, exactly")
        .fillet(R, tol)
        .expect("bracket fillet fits")
        .toward(0.0, 1.0, tol)
        .expect("north, exactly")
        .to(p2(1.0, 3.0), tol)
        .expect("the filleted side ends at its far vertex")
        .line_to(p2(0.0, 3.0), tol)
        .expect("bracket top")
        .line_to(Start, tol)
        .expect("bracket seam")
}

fn insert(doc: &mut Doc<ProfileProgram>, node: Node<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let applied = apply(
        doc,
        &DocEdit::InsertNode {
            node: Box::new(node),
        },
        tol,
        &RefusingReach,
    )
    .expect("the edit applies");
    *doc = applied.doc;
    applied.record.minted.expect("insert mints an id")
}

fn eval(doc: &Doc<ProfileProgram>, tol: Tol) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), tol)
}

/// The bracket's recipe and its extrude node.
fn document(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("bracket", tol);
    let frame = insert(
        &mut doc,
        Node::Datum(Datum::Frame {
            origin: [len(0.0), len(0.0), len(0.0)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }),
        tol,
    );
    let profile = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane: frame,
            loops: vec![
                LoopProgram::from_recorded(&outline(tol).program)
                    .expect("a literal recording lifts"),
            ],
            ids: Vec::new(),
        }),
        tol,
    );
    let body = insert(
        &mut doc,
        Node::Extrude {
            profile,
            distance: len(DEPTH),
        },
        tol,
    );
    (doc, body)
}

/// This scene's recipe, as a document the GUI can open.
pub fn gallery_document(tol: Tol) -> Doc<ProfileProgram> {
    document(tol).0
}

fn body_at<S: Scalar>(ev: &Evaluation<S>, id: RecipeNodeId) -> Body<S> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        other => panic!("expected a body, got {other:?}"),
    }
}

fn volume(body: &Body<f64>, tol: Tol) -> f64 {
    mass_properties(body, tol)
        .expect("a planar-and-cylinder body has closed-form mass properties")
        .volume
}

fn assert_volume(what: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-9 * want,
        "{what}: V = {got}, closed form {want}"
    );
}

/// The bracket, evaluated at the K-telemetry scalar: the same document
/// the tour renders.
#[cfg(feature = "probe")]
pub(crate) fn probe_body(tol: Tol) -> Body<pncad::geom_core::k_stats::Probe> {
    let (doc, body) = document(tol);
    let ev = evaluate::<pncad::geom_core::k_stats::Probe>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    body_at(&ev, body)
}

/// The wall's document: the bracket, split at `x + y = CUT`, the
/// corner piece's section chords chamfered by name. Asserts what the
/// split builds and pins what the chamfer refuses; answers the
/// sentence the stop's note carries.
fn split_and_break(doc: &Doc<ProfileProgram>, body: RecipeNodeId, tol: Tol) -> String {
    let mut doc = doc.clone();
    let tool = insert(
        &mut doc,
        Node::Datum(Datum::Plane {
            origin: [len(CUT), len(0.0), len(0.0)],
            normal: [scl(1.0), scl(1.0), scl(0.0)],
        }),
        tol,
    );
    let split = insert(&mut doc, Node::Split { target: body, tool }, tol);
    let offcuts = insert(
        &mut doc,
        Node::Part {
            of: split,
            select: PartSelect::SplitHalf(SplitHalf::Above),
        },
        tol,
    );
    let corner = insert(
        &mut doc,
        Node::Part {
            of: split,
            select: PartSelect::SplitHalf(SplitHalf::Below),
        },
        tol,
    );
    let ev = eval(&doc, tol);

    // The halves partition the body: each offcut is a trapezoid of
    // parallel sides 3 − CUT and 4 − CUT across a unit-wide leg.
    let whole = volume(&body_at(&ev, body), tol);
    let off = volume(&body_at(&ev, offcuts), tol);
    let kept = volume(&body_at(&ev, corner), tol);
    assert_volume("the offcuts", off, 2.0 * DEPTH * ((3.0 - CUT) + 0.5));
    assert_volume("the halves' sum", off + kept, whole);

    // A cap crossed twice: each chord is named by its two end
    // vertices, under the cap face it lies in.
    let chords = select(
        &ev,
        corner,
        &Selector::of(NamePat::of_kind(EntityKind::Edge).path([
            SegPat::tag(SegTag::SectionEdge),
            SegPat::tag(SegTag::Fragment),
        ])),
    );
    assert_eq!(
        chords.len(),
        4,
        "two legs x two caps, each chord named by its ends: {chords:?}"
    );

    let broken = insert(&mut doc, Node::chamfer(corner, len(SETBACK), chords), tol);
    let ev = eval(&doc, tol);
    let outcome = ev.value(broken).ok_or_else(|| {
        &ev.node_error(broken)
            .expect("a node with no value carries its refusal")
            .kind
    });
    // Each chord's chamfer, once it ends at the leg's side walls, is a
    // right-angle prism of section SETBACK²/2 between two parallel
    // walls a unit apart, which a 45° chord crosses in √2.
    let delta_v = 4.0 * SETBACK * SETBACK / 2.0 * SQRT_2;
    crate::walls::wall(
        "bracket",
        1,
        "the corner piece's four section chords, chamfered by name",
        outcome,
        |e| {
            matches!(
                e,
                NodeErrorKind::Blend {
                    error: BlendError::UnsupportedRunOut { .. },
                    ..
                }
            )
        },
        &format!(
            "end the gallery document in the chamfer: its body is the corner piece less \
             4·(d²/2)·√2 = {delta_v:.6} (d = {SETBACK})"
        ),
    );
    format!(
        "split at x + y = {CUT}: offcuts V = {off:.6}, corner piece V = {kept:.6}, sum = whole; \
         its four cap chords are named by their ends, and chamfering them by name refuses \
         UnsupportedRunOut (wall 1)"
    )
}

/// The bracket's stop, its body the document's extrude node.
pub fn stop(tol: Tol) -> Stop {
    let (doc, node) = document(tol);
    let ev = eval(&doc, tol);
    let body = body_at(&ev, node);
    assert_volume(
        "the bracket",
        volume(&body, tol),
        DEPTH * (5.0 + R * R * (1.0 - PI / 4.0)),
    );
    let note = split_and_break(&doc, node, tol);
    Stop {
        name: "bracket",
        caption: String::new(),
        montage: true,
        story: "L-bracket with a filleted inner corner (polyline + tangent arc profile)",
        ops: "PATHS algebra (toward/fillet/far-end anchor) -> LoopProgram::from_recorded -> \
              Node::Profile -> Node::Extrude",
        delta: 1e-2,
        note: Some(note),
        view: View {
            elev: 32.0,
            azim: -55.0,
            up: 'z',
        },
        bodies: vec![SceneBody::plain("bracket", [0.36, 0.56, 0.86], body).named(&ev, node)],
    }
}
