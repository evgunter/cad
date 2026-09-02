//! **What a datum draws** (`viewer::datums`): the wireframe rows, run
//! against a real document and a real evaluation with no renderer.
//!
//! The module's whole content is invented geometry — a plane is
//! infinite and a point has no extent — so what these rows check is
//! the invention's own contract: it comes from the value the
//! evaluation produced, it is placed where the datum is, it is sized
//! against the picture rather than against the world, and it says
//! which way a plane faces.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use common::{inserted, len3, scl3, square};
use pncad::document::{
    CancelToken, Datum, Doc, DocumentId, EvalOptions, Node, ProfileProgram, evaluate,
};
use pncad::geom_core::Tol;
use viewer::datums::{self, DatumKind, FALLBACK_EXTENT};

/// A document holding just the datums given, plus its evaluation.
fn evaluated(nodes: Vec<Node<ProfileProgram>>) -> (Doc<ProfileProgram>, Tol) {
    let tol = Tol::witness();
    let mut doc = Doc::empty(DocumentId::derive("datum-draw"), tol);
    for node in nodes {
        let (next, _) = inserted(&doc, node, tol);
        doc = next;
    }
    (doc, tol)
}

/// Every drawing, at `extent`.
fn draws(doc: &Doc<ProfileProgram>, tol: Tol, extent: f64) -> Vec<datums::DatumDraw> {
    let evaluation = evaluate(
        doc,
        None,
        &CancelToken::default(),
        &EvalOptions::default(),
        tol,
    );
    datums::draws(doc, &evaluation, extent)
}

/// The largest distance between any drawn point and `centre`.
fn reach(segments: &[[f64; 3]], centre: [f64; 3]) -> f64 {
    segments
        .iter()
        .map(|p| {
            let (dx, dy, dz) = (p[0] - centre[0], p[1] - centre[1], p[2] - centre[2]);
            (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
        })
        .fold(0.0_f64, f64::max)
}

/// A plane datum, a point datum and an axis datum.
fn plane(origin: [f64; 3], normal: [f64; 3]) -> Node<ProfileProgram> {
    Node::Datum(Datum::Plane {
        origin: len3(origin),
        normal: scl3(normal),
    })
}

fn axis(origin: [f64; 3], direction: [f64; 3]) -> Node<ProfileProgram> {
    Node::Datum(Datum::Axis {
        origin: len3(origin),
        direction: scl3(direction),
    })
}

fn point(position: [f64; 3]) -> Node<ProfileProgram> {
    Node::Datum(Datum::Point {
        position: len3(position),
    })
}

/// **Each of the three kinds draws, and says which it is.**
///
/// In document order, so what a reader sees in the viewport is
/// ordered the way the feature tree lists it.
#[test]
fn every_datum_kind_draws_in_document_order() {
    let (doc, tol) = evaluated(vec![
        plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        axis([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        point([0.001, 0.002, 0.003]),
    ]);
    let drawn = draws(&doc, tol, 0.02);
    let kinds: Vec<DatumKind> = drawn.iter().map(|d| d.kind).collect();
    assert_eq!(
        kinds,
        vec![DatumKind::Plane, DatumKind::Axis, DatumKind::Point],
    );
    for d in &drawn {
        assert!(
            !d.segments.is_empty() && d.segments.len().is_multiple_of(2),
            "{:?} drew {} positions — a line list is pairs",
            d.kind,
            d.segments.len(),
        );
        assert!(
            doc.node(d.node).is_some(),
            "a drawing names a node the document does not have",
        );
    }
}

/// **A node that is not a datum draws nothing.**
///
/// The row that keeps this from being "draw anything that evaluates
/// to a datum value": the profile below is an ordinary node, and a
/// document full of geometry must not sprout construction marks.
#[test]
fn only_datum_nodes_draw() {
    let (doc, tol) = evaluated(vec![square(0.02), point([0.0, 0.0, 0.0])]);
    let drawn = draws(&doc, tol, 0.02);
    assert_eq!(drawn.len(), 1, "only the point is a datum");
    assert_eq!(drawn[0].kind, DatumKind::Point);
}

/// **A drawing is placed where its datum is**, not at the origin.
#[test]
fn a_datum_is_drawn_at_its_own_position() {
    let at = [0.05, -0.03, 0.011];
    let (doc, tol) = evaluated(vec![point(at)]);
    let drawn = draws(&doc, tol, 0.02);
    let segments = &drawn[0].segments;
    // Every arm of the cross is within its own half-length of the
    // position, so the mark is AT the point rather than near it.
    assert!(
        reach(segments, at) <= 0.02,
        "a point's mark strayed {:.4} m from the point at extent 0.02",
        reach(segments, at),
    );
    // And it is not at the world origin, which is what a placement bug
    // would silently produce.
    assert!(reach(segments, [0.0; 3]) > 0.05);
}

/// **A datum is sized against the PICTURE, not against the world.**
///
/// The whole reason `draws` takes an extent: a plane on a 2 mm boss
/// and one on a 2 m plate have to be drawn at the same size relative
/// to what they sit beside, because a plane has no size of its own to
/// be drawn at.
#[test]
fn a_datums_size_scales_with_the_scene() {
    let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])]);
    let small = reach(&draws(&doc, tol, 0.01)[0].segments, [0.0; 3]);
    let large = reach(&draws(&doc, tol, 1.0)[0].segments, [0.0; 3]);
    let ratio = large / small;
    assert!(
        (ratio - 100.0).abs() < 1.0e-9,
        "a hundredfold scene drew a {ratio:.3}-fold datum",
    );
}

/// **A scene with no extent still draws a datum somebody can see.**
///
/// A fresh document whose first act is a datum has nothing on screen
/// to be sized against, and zero is not a size. The fallback is the
/// module's, stated once, so this row asserts the behaviour rather
/// than a second copy of the number.
#[test]
fn an_extentless_scene_falls_back_rather_than_drawing_nothing() {
    let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])]);
    let at_fallback = reach(&draws(&doc, tol, FALLBACK_EXTENT)[0].segments, [0.0; 3]);
    for degenerate in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        let got = reach(&draws(&doc, tol, degenerate)[0].segments, [0.0; 3]);
        assert!(
            (got - at_fallback).abs() < 1.0e-12,
            "extent {degenerate} drew reach {got}, not the fallback's {at_fallback}",
        );
        assert!(got > 0.0, "extent {degenerate} drew a datum of no size");
    }
}

/// **A plane says which way it faces**, and the tick that says so
/// leaves the plane.
///
/// The one asymmetry in a plane's drawing: everything else is in the
/// plane and would look the same from either side, so without this a
/// reader could not tell a normal from its negation — which is the
/// difference between the two halves an extrude or a split lands in.
#[test]
fn a_plane_draws_a_tick_along_its_normal() {
    let normal = [0.0, 0.0, 1.0];
    let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], normal)]);
    let segments = &draws(&doc, tol, 0.02)[0].segments;
    let off_plane: Vec<&[f64; 3]> = segments.iter().filter(|p| p[2].abs() > 1.0e-12).collect();
    assert_eq!(
        off_plane.len(),
        1,
        "exactly one drawn point leaves the plane — the normal tick's far end",
    );
    assert!(
        off_plane[0][2] > 0.0,
        "the tick points along the normal, not against it",
    );
}

/// **A plane's grid is in the plane it names**, whichever way that
/// plane faces.
///
/// The basis is chosen from the normal, and the arm it seeds from is
/// picked to keep the cross product away from zero. A normal aligned
/// with a world axis is the case that would break a fixed seed, so
/// all three are driven.
#[test]
fn a_planes_grid_lies_in_the_plane_for_every_axis_aligned_normal() {
    for normal in [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] {
        let origin = [0.004, -0.002, 0.006];
        let (doc, tol) = evaluated(vec![plane(origin, normal)]);
        let segments = &draws(&doc, tol, 0.02)[0].segments;
        // Every point but the normal tick's far end has zero
        // component along the normal, measured from the origin.
        let mut off = 0usize;
        for p in segments {
            let d = (p[0] - origin[0]) * normal[0]
                + (p[1] - origin[1]) * normal[1]
                + (p[2] - origin[2]) * normal[2];
            if d.abs() > 1.0e-12 {
                off += 1;
            }
        }
        assert_eq!(off, 1, "normal {normal:?}: {off} points left the plane");
    }
}

/// **An axis is drawn along its own direction**, centred on its
/// origin.
#[test]
fn an_axis_is_drawn_along_its_direction() {
    let origin = [0.001, 0.002, 0.003];
    let (doc, tol) = evaluated(vec![axis(origin, [0.0, 0.0, 2.0])]);
    let segments = &draws(&doc, tol, 0.02)[0].segments;
    // The first pair is the axis line itself: its two ends straddle
    // the origin along z and share the origin's other components.
    let (a, b) = (segments[0], segments[1]);
    assert!(a[2] < origin[2] && b[2] > origin[2], "{a:?} .. {b:?}");
    for i in [0, 1] {
        assert!((a[i] - origin[i]).abs() < 1.0e-12);
        assert!((b[i] - origin[i]).abs() < 1.0e-12);
    }
    // The direction arrived unnormalized (length 2) and the drawing
    // is symmetric about the origin either way.
    assert!(((b[2] - origin[2]) + (a[2] - origin[2])).abs() < 1.0e-12);
}

/// **A datum whose evaluation produced no value draws nothing** —
/// there is nothing to draw, and the tree's own badge is what says
/// why. A degenerate normal is the reachable way to get one.
#[test]
fn a_failed_datum_draws_nothing() {
    let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], [0.0, 0.0, 0.0])]);
    let drawn = draws(&doc, tol, 0.02);
    assert!(
        drawn.is_empty(),
        "a datum that did not evaluate drew {} wireframes",
        drawn.len(),
    );
}
