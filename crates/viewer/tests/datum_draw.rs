//! **What a datum draws** (`viewer::datums`): the wireframe rows, run
//! against a real document and a real evaluation with no renderer.
//!
//! The module's whole content is invented geometry — a plane is
//! infinite and a point has no extent — so what these rows check is
//! the invention's own contract: it comes from the value the
//! evaluation produced, it is placed where the datum is, it is sized
//! against the VIEW rather than against the world — so no zoom can
//! open a grid cell wide enough to swallow the window — and it says
//! which way a plane faces.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use common::{inserted, len3, scl3, square};
use pncad::document::{
    CancelToken, Datum, Doc, DocumentId, EvalOptions, Node, ProfileProgram, evaluate,
};
use pncad::geom_core::{Point3, Tol};
use viewer::datums::{self, DatumKind, View, grid_pitch};

/// A document holding just the datums given.
fn evaluated(nodes: Vec<Node<ProfileProgram>>) -> (Doc<ProfileProgram>, Tol) {
    let tol = Tol::witness();
    let mut doc = Doc::empty(DocumentId::derive("datum-draw"), tol);
    for node in nodes {
        let (next, _) = inserted(&doc, node, tol);
        doc = next;
    }
    (doc, tol)
}

/// A view from `eye`, looking at a 1280-pixel window with a 45° field.
///
/// The numbers are the app's own (`app::datum_view`): the vertical
/// field over the vertical pixel count.
fn view_from(eye: [f64; 3]) -> View {
    view_at(eye, [0.0, 0.0, 0.0])
}

/// A view from `eye` pointed at `look_at`.
fn view_at(eye: [f64; 3], look_at: [f64; 3]) -> View {
    let height = 800.0;
    View {
        eye: Point3::new(eye[0], eye[1], eye[2]),
        look_at: Point3::new(look_at[0], look_at[1], look_at[2]),
        metres_per_pixel_at_one_metre: 2.0 * (core::f64::consts::FRAC_PI_8).tan() / height,
        viewport_px: 1280.0,
    }
}

/// Every drawing, seen from `eye`.
fn draws(doc: &Doc<ProfileProgram>, tol: Tol, eye: [f64; 3]) -> Vec<datums::DatumDraw> {
    let evaluation = evaluate(
        doc,
        None,
        &CancelToken::default(),
        &EvalOptions::default(),
        tol,
    );
    datums::draws(doc, &evaluation, view_from(eye))
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
    let drawn = draws(&doc, tol, [0.0, -0.15, 0.1]);
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
    let drawn = draws(&doc, tol, [0.0, -0.15, 0.1]);
    assert_eq!(drawn.len(), 1, "only the point is a datum");
    assert_eq!(drawn[0].kind, DatumKind::Point);
}

/// **A point's mark is AT the point**, not at the world origin, and
/// it is the same handful of pixels at any distance.
#[test]
fn a_datum_is_drawn_at_its_own_position() {
    let at = [0.05, -0.03, 0.011];
    let eye = [0.0, -0.2, 0.1];
    let (doc, tol) = evaluated(vec![point(at)]);
    let segments = &draws(&doc, tol, eye)[0].segments;
    // The arms are screen-sized, so the bound is stated in the view's
    // own units rather than as a length somebody guessed: a few tens
    // of pixels, whatever that is in metres from here.
    let per_pixel = view_from(eye).metres_per_pixel_at_one_metre * reach(&[at], eye);
    assert!(
        reach(segments, at) < per_pixel * 40.0,
        "a point's mark spanned {:.5} m, more than 40 px from here",
        reach(segments, at),
    );
    assert!(reach(segments, [0.0; 3]) > 0.05, "and not at the origin");
}

/// **Zooming in cannot land inside a grid cell** — the failure the
/// view-relative sizing exists to prevent.
///
/// A world-fixed grid keeps its pitch as the eye closes in, so past
/// some distance one cell fills the window and the plane vanishes
/// with nothing on screen to say it is there. This drives the eye from
/// a metre away down to a tenth of a millimetre and asserts that the
/// drawn pitch stays a readable fraction of the window at every step,
/// which is what "you never end up inside a hole" means in numbers.
#[test]
fn no_zoom_leaves_the_eye_inside_a_grid_cell() {
    let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])]);
    let mut height = 1.0_f64;
    while height > 1.0e-4 {
        let view = view_from([0.0, 0.0, height]);
        let per_pixel = view.metres_per_pixel_at_one_metre * height;
        let pitch = grid_pitch(per_pixel);
        let pitch_px = pitch / per_pixel;
        assert!(
            (20.0..=400.0).contains(&pitch_px),
            "at {height:.5} m the grid ruled one cell every {pitch_px:.1} px",
        );
        // And the drawing itself covers the window at that distance.
        let segments = &draws(&doc, tol, [0.0, 0.0, height])[0].segments;
        let window = per_pixel * view.viewport_px;
        assert!(
            reach(segments, [0.0, 0.0, 0.0]) >= window * 0.5,
            "at {height:.5} m the patch reached {:.5} m, inside the {window:.5} m window",
            reach(segments, [0.0, 0.0, 0.0]),
        );
        height *= 0.5;
    }
}

/// **The pitch holds still, and only steps.**
///
/// A pitch varying continuously with distance would keep the on-screen
/// spacing perfect and make every line swim under the cursor. On the
/// 1-2-5 ladder the answer is one of a discrete set, so over a
/// hundredfold zoom the grid takes a countable number of steps rather
/// than moving at every frame.
#[test]
fn the_grid_pitch_steps_rather_than_sliding() {
    let mut seen: Vec<f64> = Vec::new();
    let mut per_pixel = 1.0e-6;
    while per_pixel < 1.0e-4 {
        let pitch = grid_pitch(per_pixel);
        if seen
            .last()
            .is_none_or(|last| (last - pitch).abs() > 1.0e-15)
        {
            seen.push(pitch);
        }
        per_pixel *= 1.01;
    }
    assert!(
        seen.len() <= 8,
        "a hundredfold zoom moved the pitch {} times — it is sliding, not stepping",
        seen.len(),
    );
    // Every rung is a 1-2-5 mantissa on some decade.
    for pitch in &seen {
        let decade = 10.0_f64.powf(pitch.log10().floor());
        let mantissa = pitch / decade;
        assert!(
            [1.0, 2.0, 5.0]
                .iter()
                .any(|m| (m - mantissa).abs() < 1.0e-9),
            "pitch {pitch:e} has mantissa {mantissa}, which is not on the ladder",
        );
    }
}

/// **A grid line passes through the datum's origin**, whatever the
/// eye is doing.
///
/// The patch follows the view, so its centre moves; the RULING does
/// not. Lines are laid at multiples of the pitch from the origin, so
/// one of them is the origin itself — which is what stops the grid
/// sliding under the cursor as the camera pans.
#[test]
fn the_ruling_is_anchored_on_the_origin_not_on_the_view() {
    let origin = [0.0, 0.0, 0.0];
    let (doc, tol) = evaluated(vec![plane(origin, [0.0, 0.0, 1.0])]);
    for eye in [[0.0, 0.0, 0.1], [0.03, -0.02, 0.1], [-0.05, 0.04, 0.2]] {
        let segments = &draws(&doc, tol, eye)[0].segments;
        // A line through the origin is one whose two ends share a
        // zero coordinate on the axis it does not run along.
        let through = segments.chunks_exact(2).any(|pair| {
            (pair[0][0].abs() < 1.0e-12 && pair[1][0].abs() < 1.0e-12)
                || (pair[0][1].abs() < 1.0e-12 && pair[1][1].abs() < 1.0e-12)
        });
        assert!(
            through,
            "from {eye:?} no grid line passes through the origin"
        );
    }
}

/// **A plane says which way it faces**/// **A plane says which way it faces**, and the tick that says so
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
    let segments = &draws(&doc, tol, [0.0, -0.15, 0.1])[0].segments;
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
        let segments = &draws(&doc, tol, [0.0, -0.15, 0.1])[0].segments;
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

/// **An axis is drawn along its own direction**, reaching past the
/// window in both directions from where the eye is looking.
///
/// Centred on the eye's foot on the axis rather than on the axis's
/// origin — an axis is infinite, and a segment centred on the origin
/// would run off screen the moment the camera moved along it.
#[test]
fn an_axis_is_drawn_along_its_direction() {
    let origin = [0.001, 0.002, 0.003];
    let eye = [0.0, -0.15, 0.1];
    let (doc, tol) = evaluated(vec![axis(origin, [0.0, 0.0, 2.0])]);
    let segments = &draws(&doc, tol, eye)[0].segments;
    // The first pair is the axis line: both ends share the origin's
    // other two components, because the line runs along z alone.
    let (a, b) = (segments[0], segments[1]);
    for i in [0, 1] {
        assert!((a[i] - origin[i]).abs() < 1.0e-12, "{a:?} left the axis");
        assert!((b[i] - origin[i]).abs() < 1.0e-12, "{b:?} left the axis");
    }
    // It straddles the eye's own height on the axis, and reaches at
    // least a window each way from it.
    let foot = eye[2];
    assert!(a[2] < foot && b[2] > foot, "{a:?} .. {b:?} misses the eye");
    let view = view_from(eye);
    let window = view.metres_per_pixel_at_one_metre * 0.15 * view.viewport_px;
    assert!(
        (b[2] - a[2]) > window,
        "the axis spanned {:.4} m against a {window:.4} m window",
        b[2] - a[2],
    );
}

/// **A datum whose evaluation produced no value draws nothing** —
/// there is nothing to draw, and the tree's own badge is what says
/// why. A degenerate normal is the reachable way to get one.
#[test]
fn a_failed_datum_draws_nothing() {
    let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], [0.0, 0.0, 0.0])]);
    let drawn = draws(&doc, tol, [0.0, -0.15, 0.1]);
    assert!(
        drawn.is_empty(),
        "a datum that did not evaluate drew {} wireframes",
        drawn.len(),
    );
}
