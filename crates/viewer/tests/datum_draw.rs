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

fn frame(origin: [f64; 3], u: [f64; 3], v: [f64; 3]) -> Node<ProfileProgram> {
    Node::Datum(Datum::Frame {
        origin: len3(origin),
        u: scl3(u),
        v: scl3(v),
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

/// **A frame's grid is ruled on the FRAME's axes, not on a display
/// convention.**
///
/// This is the row that makes the frame worth drawing at all. Two
/// frames on one surface, turned a quarter turn from each other, are
/// the same plane to `Datum::Plane` and to [`datums::basis`] — which
/// picks in-plane directions off the normal alone, so it hands both of
/// them the SAME pair. If the frame drew through that convention, the
/// two pictures would be identical and the drawing would be hiding the
/// only thing the datum carries.
///
/// The claim is checked where it is falsifiable: a frame turned 30°
/// (not a multiple of the quarter turn a square grid is symmetric
/// under) rules lines along its own axes, so every drawn direction is
/// parallel to one of them.
#[test]
fn a_frames_grid_follows_its_own_axes() {
    let (cos, sin) = (30.0_f64.to_radians().cos(), 30.0_f64.to_radians().sin());
    let (doc, tol) = evaluated(vec![frame(
        [0.0, 0.0, 0.0],
        [cos, sin, 0.0],
        [-sin, cos, 0.0],
    )]);
    let drawn = draws(&doc, tol, [0.0, -0.15, 0.1]);
    assert_eq!(drawn.len(), 1);
    assert_eq!(drawn[0].kind, DatumKind::Frame);
    let segments = &drawn[0].segments;
    assert!(segments.len() >= 6, "{} positions", segments.len());
    let mut along_u = 0;
    let mut along_v = 0;
    let mut along_n = 0;
    let mut off_axis = 0;
    for pair in segments.chunks_exact(2) {
        let d = [
            pair[1][0] - pair[0][0],
            pair[1][1] - pair[0][1],
            pair[1][2] - pair[0][2],
        ];
        let n = (d[0].powi(2) + d[1].powi(2) + d[2].powi(2)).sqrt();
        assert!(n > 0.0, "a zero-length segment");
        let unit = [d[0] / n, d[1] / n, d[2] / n];
        let on_u = (unit[0] * cos + unit[1] * sin).abs();
        let on_v = (unit[0] * -sin + unit[1] * cos).abs();
        if on_u > 1.0 - 1.0e-9 {
            along_u += 1;
        } else if on_v > 1.0 - 1.0e-9 {
            along_v += 1;
        } else if unit[2].abs() > 1.0 - 1.0e-9 {
            along_n += 1;
        } else {
            // The arrow barbs, whose whole job is to point off both
            // axes — `a_frames_arrows_cannot_hide_in_its_grid` is the
            // row that owns them.
            off_axis += 1;
        }
    }
    // Every RULED line runs along an axis: the four barbs are the only
    // segments that may not, so the count below is what "the grid
    // follows the frame" means once the arrows are subtracted.
    assert_eq!(off_axis, 4, "only the four barbs may leave the axes");
    assert!(along_u > 0 && along_v > 0, "{along_u} / {along_v}");
    // The third line of the triad: a frame says which side is up as
    // well as which way it is turned, so a reader can see which way an
    // extrude off it will go.
    assert_eq!(along_n, 1, "one normal tick, not {along_n}");
}

/// **A frame's arrows are VISIBLE against its own grid.**
///
/// The row the first cut of this needed and did not have. The arms are
/// drawn along the frame's axes, and the grid is ruled from the origin
/// along those same axes — so a grid line passes exactly through the
/// origin in x and in y, and a bare arm lies exactly on top of one.
/// The earlier rows all passed: the arms ran the right directions, at
/// the right lengths, from the right point. They were also invisible,
/// which driving the app found and no assertion did.
///
/// So the claim is stated as what a reader can actually see: some part
/// of the mark points along NEITHER axis, and is therefore somewhere
/// the ruling cannot be.
#[test]
fn a_frames_arrows_cannot_hide_in_its_grid() {
    let (doc, tol) = evaluated(vec![frame(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
    )]);
    let segments = &draws(&doc, tol, [0.0, -0.15, 0.1])[0].segments;
    let off_axis = segments
        .chunks_exact(2)
        .filter(|pair| {
            let d = [
                pair[1][0] - pair[0][0],
                pair[1][1] - pair[0][1],
                pair[1][2] - pair[0][2],
            ];
            let n = (d[0].powi(2) + d[1].powi(2) + d[2].powi(2)).sqrt();
            // Off BOTH axes and off the normal: a segment along any of
            // the three could be mistaken for the ruling or the tick.
            n > 0.0
                && (d[0] / n).abs() < 1.0 - 1.0e-9
                && (d[1] / n).abs() < 1.0 - 1.0e-9
                && (d[2] / n).abs() < 1.0 - 1.0e-9
        })
        .count();
    assert_eq!(
        off_axis, 4,
        "two barbs per arrow have to point off both axes, or the arrow          is drawn on top of a grid line and shows nothing",
    );
}

/// **The two arrows are unequal, so the drawing says which axis is x.**
///
/// A grid is symmetric under a quarter turn, so a frame drawn with two
/// arms of one length would name the PAIR of directions without naming
/// which of them the sketch's x is — and that is the difference
/// between a frame and the plane it lies in.
#[test]
fn a_frames_arms_name_which_axis_is_x() {
    let (doc, tol) = evaluated(vec![frame(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
    )]);
    let segments = &draws(&doc, tol, [0.0, -0.15, 0.1])[0].segments;
    // The arms found by what they ARE — a segment leaving the origin
    // along one of the frame's two axes — rather than by their index in
    // the list. The arrow barbs moved that index once already, and a
    // positional read is a test that breaks on a drawing change instead
    // of on a behaviour change. The normal tick also leaves the origin
    // and is excluded by running along neither axis.
    let mut arms: Vec<(usize, f64)> = Vec::new();
    for pair in segments.chunks_exact(2) {
        if reach(&[pair[0]], [0.0, 0.0, 0.0]) > 1.0e-12 {
            continue;
        }
        let d = [pair[1][0], pair[1][1], pair[1][2]];
        let n = (d[0].powi(2) + d[1].powi(2) + d[2].powi(2)).sqrt();
        if (d[0] / n).abs() > 1.0 - 1.0e-9 {
            arms.push((0, n));
        } else if (d[1] / n).abs() > 1.0 - 1.0e-9 {
            arms.push((1, n));
        }
    }
    arms.sort_by_key(|&(axis, _)| axis);
    assert_eq!(arms.len(), 2, "one arm per axis, got {arms:?}");
    let (x_arm, y_arm) = (arms[0].1, arms[1].1);
    assert!(
        x_arm > y_arm * 1.2,
        "the x arm ({x_arm:e} m) has to read as longer than the y one \
         ({y_arm:e} m) or the picture is symmetric under a quarter turn",
    );
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

/// **The ruling sits on ONE LATTICE, wherever the camera looks.**
///
/// The patch follows the view, so its centre moves; the ruling must
/// not. Lines are laid at multiples of the pitch measured from the
/// datum's ORIGIN, so panning slides the window over a fixed lattice
/// instead of dragging the lattice along — which is what stops the
/// grid swimming under the cursor.
///
/// Stated as "every drawn line's coordinate is a multiple of the
/// pitch" rather than "a line passes through the origin", because the
/// second is only true while the origin is inside the patch: look a
/// metre away and the origin is simply not on screen. The lattice
/// claim holds everywhere and is the one the design actually makes.
#[test]
fn the_ruling_is_anchored_on_the_origin_not_on_the_view() {
    let origin = [0.0, 0.0, 0.0];
    let (doc, tol) = evaluated(vec![plane(origin, [0.0, 0.0, 1.0])]);
    // Offsets deliberately incommensurate with any 1-2-5 pitch: a
    // ruling dragged along by the view would land on multiples of the
    // pitch PLUS one of these, and none of them divides.
    for look_at in [
        [0.0, 0.0, 0.0],
        [0.0173, 0.0311, 0.0],
        [-0.1137, 0.0719, 0.0],
        [0.9431, -1.3177, 0.0],
    ] {
        let eye = [look_at[0], look_at[1] - 0.15, 0.1];
        let view = view_at(eye, look_at);
        let evaluation = evaluate(
            &doc,
            None,
            &CancelToken::default(),
            &EvalOptions::default(),
            tol,
        );
        let segments = &datums::draws(&doc, &evaluation, view)[0].segments;
        // The pitch this view asks for: the plane is z = 0 and the
        // looked-at point is on it, so the patch centre IS `look_at`.
        let per_pixel = view.metres_per_pixel_at_one_metre * reach(&[look_at], eye);
        let pitch = grid_pitch(per_pixel);
        // The last pair is the normal tick, which is anchored on the
        // origin by construction and says nothing about the ruling.
        for pair in segments[..segments.len() - 2].chunks_exact(2) {
            // Whichever coordinate the line holds constant is the one
            // the lattice indexes.
            let held = if (pair[0][0] - pair[1][0]).abs() < 1.0e-12 {
                pair[0][0]
            } else {
                pair[0][1]
            };
            let index = held / pitch;
            assert!(
                (index - index.round()).abs() < 1.0e-6,
                "looking at {look_at:?}, a line sits at {held:e} m — \
                 {index} pitches from the origin, not a whole number",
            );
        }
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
