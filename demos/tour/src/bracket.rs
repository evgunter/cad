//! **The bracket** — an L outline with one r = 0.5 tangent fillet at
//! its inner corner, extruded 0.75, as a recipe DOCUMENT.
//!
//! The outline is written once, in the PATHS algebra, and lifted into
//! a `Node::Profile` by [`LoopProgram::from_recorded`] — the seam
//! between the two authoring surfaces, and the spelling Python's
//! `Node.profile(outline)` takes. Frame, profile, extrude, then the
//! trim and the break below: that is the document the gallery writes,
//! and the body the tour renders is its last node's value.
//!
//! **Oracle**: `V = 0.75 · (5 + r²(1 − π/4))`. The L is two 1-wide
//! legs of length 3 sharing a unit square, area 5, and rounding the
//! reflex corner ADDS the region between the corner and the arc.
//!
//! # The wall: the leg ends, trimmed flush and broken by name
//!
//! What this part wants next is both leg ends trimmed flush to ONE
//! plane, `x + y = 2.75`, and the cut edges broken. In the document
//! that is two nodes: [`Node::Split`] along the plane, whose `below`
//! output is the corner piece (the offcuts are `above`), and
//! [`Node::Chamfer`] reading that output, on its section chords.
//!
//! The split builds, and [`split_and_break`] asserts what it builds:
//! the halves partition the body (each offcut is a trapezoid prism of
//! area `(3 − 2.75) + 1/2`), and because the plane crosses BOTH legs
//! each cap face is crossed twice, so each of the corner piece's four
//! cap chords is named by its ends — `[SectionEdge, Fragment(Ends)]` —
//! and a selector reaches them by role path alone.
//!
//! The chamfer builds: each chord's band ends where it meets a side
//! wall of its leg, cut off in that wall's plane in a chord at 45° to
//! it, and the corner piece loses `4·(d²/2)·√2`, which the scene
//! asserts; the offcuts' chords chamfer the same way, each inside its
//! own solid. The fillet builds too, each band cut off in an arc of the
//! side wall's elliptic section of its cylinder, at `4·(1 − π/4)·r²·√2`
//! on either half. Both section faces' whole rims chamfer too, which
//! the scene asserts and renders. Each corner of a section face is a
//! turn, two of its three edges requested, whose dihedrals differ — a
//! right angle at the cap chord, 45° or 135° at the section edge on the
//! side wall — so one band's foot on the side wall's cap edge is
//! further, and that band runs past the mitre, cut off by the other's
//! face there: the overrun (`crates/sweep/README.md`, "Where a straight
//! band ends, and where it turns").
//!
//! The outline's decimal-via ancestor lives on as the large-K lint's
//! litmus fixture (`tools/k-lint/tests/litmus.rs`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, SQRT_2};
use pncad::prelude::AuthoredNode;

use pncad::document::{FrameMove, Operand, TargetAxis};
use pncad::geom_core::Tol;
use pncad::prelude::{
    CancelToken, Dimension, Doc, DocEdit, EntityKind, EvalOptions, Evaluation, Formula,
    LoopProgram, NamePat, Node, Open, ProfileProgram, RecipeNodeId, SegPat, SegTag, Selector,
    SplitHalf, StableName, Start, ValuePayload, apply, evaluate, p2, select,
};
use pncad::profile::ClosedLoop;
use pncad::topo::{Body, mass_properties};

use crate::scalar::Scalar;
use crate::{SceneBody, TourStop, View};

/// The fillet's radius at the inner corner.
const R: f64 = 0.5;
/// The extrusion depth.
const DEPTH: f64 = 0.75;
/// The trim plane is `x + y = CUT`: past both fillet tangent points
/// (`x + y = 2.5`) and short of both leg ends (`x + y = 3`), so it
/// crosses each leg once and the fillet not at all.
const CUT: f64 = 2.75;
/// The setback the walls' blends ask for.
const SETBACK: f64 = 0.1;

fn len(v: f64) -> Formula {
    Formula::literal(v, Dimension::Length).expect("a length")
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

fn insert(doc: &mut Doc<ProfileProgram>, node: AuthoredNode, tol: Tol) -> RecipeNodeId {
    let applied = apply(
        doc,
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        tol,
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
    let profile = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            loops: vec![
                LoopProgram::from_recorded(&outline(tol).program)
                    .expect("a literal recording lifts"),
            ],
            ids: Vec::new(),
            on: None,
        }),
        tol,
    );
    let body = insert(
        &mut doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(DEPTH),
        },
        tol,
    );
    (doc, body)
}

/// This scene's recipe, as a document the GUI can open: the bracket,
/// trimmed flush at `x + y = CUT`, its corner piece's four cap chords
/// chamfered by name, placed in the world.
pub fn gallery_document(tol: Tol) -> Doc<ProfileProgram> {
    let (doc, body) = document(tol);
    let trimmed = trimmed_and_broken(&doc, body, tol);
    let placed = crate::world::place_in_world(&trimmed.doc, trimmed.chamfer, tol);
    crate::world::set_product(&placed.doc, &[Operand::Node(placed.placement)], tol).doc
}

/// The trim and the break, as nodes over the bracket's extrude.
struct Trimmed {
    doc: Doc<ProfileProgram>,
    split: RecipeNodeId,
    chords: Vec<StableName>,
    chamfer: RecipeNodeId,
}

/// The split's `half` output.
fn half(split: RecipeNodeId, half: SplitHalf) -> Operand {
    Operand::output(split, half.port())
}

/// The cap chords of a half, each named by its two end vertices under
/// the cap face it lies in. Selected in the half's read: a name tells
/// apart only what its variable holds, so the half is the read's.
fn chord_selector() -> Selector {
    Selector::of(NamePat::of_kind(EntityKind::Edge).path(vec![
        SegPat::tag(SegTag::SectionEdge),
        SegPat::tag(SegTag::Fragment),
    ]))
}

/// The read of the split's `half`.
fn half_read(
    doc: &Doc<ProfileProgram>,
    split: RecipeNodeId,
    half: SplitHalf,
) -> pncad::document::VarId {
    doc.output(split, half.port())
        .expect("a split defines a port per half")
}

/// [`Node::Split`] along `x + y = CUT`, and [`Node::Chamfer`] on the
/// corner piece's four cap chords.
fn trimmed_and_broken(doc: &Doc<ProfileProgram>, body: RecipeNodeId, tol: Tol) -> Trimmed {
    let mut doc = doc.clone();
    // The plane `x + y = CUT`: the bracket's frame at its origin corner
    // (`z` up its depth, `x` along its base), its `z` turned a quarter
    // turn about `y` onto `+x` and an eighth about `z` onto `x + y`,
    // then slid `CUT` along `x`.
    let tool = pncad::document::pose::RoleRead::through(
        crate::world::frame_on(
            &doc,
            body,
            [[0.0; 3], [0.0, 0.0, DEPTH], [3.0, 0.0, 0.0]],
            &[
                FrameMove::Spin(TargetAxis::Y, crate::world::turn(4, false)),
                FrameMove::Spin(TargetAxis::Z, crate::world::turn(8, false)),
                FrameMove::Slide(TargetAxis::X, len(CUT)),
            ],
            tol,
        ),
        pncad::document::pose::PoseRole::Xy,
    );
    let split = insert(
        &mut doc,
        Node::Split {
            target: body.into(),
            tool,
        },
        tol,
    );
    let chords = select(
        &eval(&doc, tol),
        half_read(&doc, split, SplitHalf::Below),
        &chord_selector(),
    );
    let chamfer = insert(
        &mut doc,
        Node::chamfer(half(split, SplitHalf::Below), len(SETBACK), chords.clone()),
        tol,
    );
    Trimmed {
        doc,
        split,
        chords,
        chamfer,
    }
}

/// The split's `half`, read off its value at the half's port.
fn half_at(ev: &Evaluation<f64>, split: RecipeNodeId, half: SplitHalf) -> Body<f64> {
    ev.value(split)
        .expect("the split evaluated")
        .payload
        .output_body(u32::from(half.port()))
        .unwrap_or_else(|e| panic!("the {half:?} half holds material: {e}"))
        .body()
        .clone()
}

fn body_at<S: Scalar>(ev: &Evaluation<S>, id: RecipeNodeId, tol: Tol) -> Body<S> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => crate::world::in_its_space(b, tol),
        other => panic!("expected a body, got {other:?}"),
    }
}

fn volume(body: &Body<f64>, tol: Tol) -> f64 {
    mass_properties(&crate::booleans::at_rest(body, tol), tol, crate::printed())
        .expect("certified mass properties")
        .enclosure
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
    body_at(&ev, body, tol)
}

/// The trim and the break: asserts what the split builds, the chamfer
/// of the corner piece's four cap chords, and of both section faces'
/// whole rims, at their closed forms; answers the sentence the stop's
/// note carries and the body it renders.
fn split_and_break(trimmed: &Trimmed, body: RecipeNodeId, tol: Tol) -> Broken {
    let Trimmed {
        doc,
        split,
        chords,
        chamfer,
    } = trimmed;
    let (split, chamfer) = (*split, *chamfer);
    // The offcuts are the split's other output, read for the walls.
    let (corner, offcuts) = (half(split, SplitHalf::Below), half(split, SplitHalf::Above));
    let ev = eval(doc, tol);

    // The halves partition the body: each offcut is a trapezoid of
    // parallel sides 3 − CUT and 4 − CUT across a unit-wide leg.
    let whole = volume(&body_at(&ev, body, tol), tol);
    let off = volume(&half_at(&ev, split, SplitHalf::Above), tol);
    let kept = volume(&half_at(&ev, split, SplitHalf::Below), tol);
    assert_volume("the offcuts", off, 2.0 * DEPTH * ((3.0 - CUT) + 0.5));
    assert_volume("the halves' sum", off + kept, whole);

    // A cap crossed twice: each chord is named by its two end
    // vertices, under the cap face it lies in.
    let edges = |path: Vec<SegPat>| Selector::of(NamePat::of_kind(EntityKind::Edge).path(path));
    assert_eq!(
        chords.len(),
        4,
        "two legs x two caps, each chord named by its ends: {chords:?}"
    );
    let mut rim = [
        chords.clone(),
        select(
            &ev,
            half_read(doc, split, SplitHalf::Below),
            &edges(vec![SegPat::tag(SegTag::SectionEdge)]),
        ),
    ]
    .concat();
    rim.sort();
    assert_eq!(rim.len(), 8, "two section faces, four edges each: {rim:?}");
    let off_chords = select(
        &ev,
        half_read(doc, split, SplitHalf::Above),
        &chord_selector(),
    );

    // Each chord's chamfer, ending on the leg's two parallel side walls
    // a unit apart, is a right-angle prism of section d²/2 that a 45°
    // chord crosses in √2 — exact while the strip stays short of the
    // fillet's tangent points, d·√2 < CUT − 2.5. Each end is cut off in
    // its side wall, oblique to the chord.
    let delta_v = 4.0 * SETBACK * SETBACK / 2.0 * SQRT_2;
    let broken = volume(&body_at(&ev, chamfer, tol), tol);
    assert_volume("the chamfered corner piece", broken, kept - delta_v);
    // The offcuts' chords likewise: two solids, each chord's band carved
    // inside its own and cut off at its leg's side walls.
    let mut off_doc = doc.clone();
    let off_chamfer = insert(
        &mut off_doc,
        Node::chamfer(offcuts.clone(), len(SETBACK), off_chords.clone()),
        tol,
    );
    let off_broken = volume(&body_at(&eval(&off_doc, tol), off_chamfer, tol), tol);
    assert_volume("the chamfered offcuts", off_broken, off - delta_v);

    // Filleted, each band's section is the region between a right
    // dihedral and its ball, over the same √2 between parallel walls;
    // each end is an arc of the wall's ellipse, `r / cos 45°` long.
    let delta_f = 4.0 * (1.0 - PI / 4.0) * SETBACK * SETBACK * SQRT_2;
    let rounded = |doc: &Doc<ProfileProgram>, of: Operand, chords: Vec<StableName>| {
        let mut doc = doc.clone();
        let fillet = insert(&mut doc, Node::fillet(of, len(SETBACK), chords), tol);
        volume(&body_at(&eval(&doc, tol), fillet, tol), tol)
    };
    let filleted = rounded(doc, corner.clone(), chords.clone());
    assert_volume("the filleted corner piece", filleted, kept - delta_f);
    let off_filleted = rounded(doc, offcuts, off_chords);
    assert_volume("the filleted offcuts", off_filleted, off - delta_f);

    // Both section faces' whole rims: each corner of a section face is
    // a turn whose dihedrals differ — a right angle at the cap chord,
    // 45° or 135° at the section edge — so the band whose foot on the
    // side wall's cap edge is further runs past the mitre, cut off by
    // the other band's face there (the overrun).
    let mut rims_doc = doc.clone();
    let rims = insert(&mut rims_doc, Node::chamfer(corner, len(SETBACK), rim), tol);
    let rimmed = volume(&body_at(&eval(&rims_doc, tol), rims, tol), tol);
    assert_volume(
        "the corner piece, both section faces' rims chamfered",
        rimmed,
        kept - 2.0 * rim_taken(SETBACK),
    );
    Broken {
        note: format!(
            "split at x + y = {CUT}: offcuts V = {off:.6}, corner piece V = {kept:.6}, sum = \
             whole; its four cap chords, named by their ends, chamfer at d = {SETBACK} to V = \
             {broken:.6} (less 4·(d²/2)·√2), as the offcuts' do to V = {off_broken:.6}; \
             filleted at r = {SETBACK}, each band cut off in an elliptic arc, they reach V = \
             {filleted:.6} and {off_filleted:.6} (less 4·(1 − π/4)·r²·√2); and both section \
             faces' whole rims chamfer to V = {rimmed:.6}, each corner's far band cut off past \
             its mitre by the near band's side wall (the overrun)"
        ),
        doc: rims_doc,
        rims,
    }
}

/// **What chamfering one section face's whole rim at `d` takes from its
/// leg.** In the face's own frame — `u` across it from the side wall
/// `y = 0` (which meets it at 45°) toward `y = 1` (at 135°), `z` up, `w`
/// into the leg — the leg is `w ≥ 0`, `w ≤ u ≤ √2 + w`, `0 ≤ z ≤
/// DEPTH`, and the four chamfer planes, each through its edge's two
/// trimlines `d` in on either face, keep `z + w ≥ d`, `DEPTH − z + w ≥
/// d`, `u + (√2 − 1)·w ≥ d` and `√2 − u + (√2 + 1)·w ≥ d`. The cap
/// chords' planes read `z` alone and the section edges' `u` alone, so at
/// depth `w < d` the kept slice is a rectangle, `DEPTH − 2d + 2w` tall
/// and `√2 − 2d + 2√2·w` wide until both side planes leave the side
/// walls at `w = d/√2`, then `√2` wide; from `w = d` on nothing is
/// taken. The chamfer takes `∫₀ᵈ (DEPTH·√2 − slice) dw`. At each corner
/// the two planes' line is the mitre, and the far plane's trace on the
/// side wall past it is the overrun.
fn rim_taken(d: f64) -> f64 {
    let (p, q, t) = (DEPTH - 2.0 * d, SQRT_2 - 2.0 * d, d / SQRT_2);
    let narrow = p * q * t + (SQRT_2 * p + q) * t * t + 4.0 * SQRT_2 / 3.0 * t * t * t;
    let full = SQRT_2 * (p * (d - t) + d * d - t * t);
    DEPTH * SQRT_2 * d - narrow - full
}

/// The trim and the break's narration, and the corner piece with both
/// section faces' whole rims chamfered, which the stop renders.
struct Broken {
    note: String,
    doc: Doc<ProfileProgram>,
    rims: RecipeNodeId,
}

/// The bracket's stop: the document's corner piece, both section faces'
/// whole rims chamfered.
pub fn stop(tol: Tol) -> TourStop {
    let (doc, node) = document(tol);
    let ev = eval(&doc, tol);
    assert_volume(
        "the bracket",
        volume(&body_at(&ev, node, tol), tol),
        DEPTH * (5.0 + R * R * (1.0 - PI / 4.0)),
    );
    let trimmed = trimmed_and_broken(&doc, node, tol);
    let Broken { note, doc, rims } = split_and_break(&trimmed, node, tol);
    let ev = eval(&doc, tol);
    let body = body_at(&ev, rims, tol);
    TourStop {
        name: "bracket",
        caption: String::new(),
        montage: true,
        story: "L-bracket with a filleted inner corner (polyline + tangent arc profile), trimmed \
                flush and its cut edges chamfered",
        ops: "PATHS algebra (toward/fillet/far-end anchor) -> LoopProgram::from_recorded -> \
              Node::Profile -> Node::Extrude -> Node::Split -> Node::Part -> Node::Chamfer",
        delta: 1e-2,
        note: Some(note),
        view: View {
            elev: 32.0,
            azim: -55.0,
            up: 'z',
        },
        bodies: vec![SceneBody::plain("bracket", [0.36, 0.56, 0.86], body).named(&ev, rims)],
    }
}

#[cfg(test)]
mod tests {
    /// The stop builds: the bracket and its halves at their closed
    /// forms, the corner piece's cap chords chamfered at `4·(d²/2)·√2`
    /// less, and both section faces' whole rims chamfered at
    /// `rim_taken` less per face.
    #[test]
    fn the_stop_builds_its_chamfered_corner_piece() {
        let stop = super::stop(pncad::geom_core::Tol::witness());
        assert_eq!(stop.bodies.len(), 1, "the chamfered corner piece");
    }
}
