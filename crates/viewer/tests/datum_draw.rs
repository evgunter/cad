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

use crate::common;

use common::{inserted, len3, scl3, square};
use pncad::document::{
    CancelToken, Datum, Doc, DocumentId, EvalOptions, Node, ProfileProgram, evaluate,
};
use pncad::geom_core::{Point3, Tol};
use viewer::camera::Camera;
use viewer::datums::{self, DatumKind, View, datum_view, grid_pitch};
use viewer::input::ViewportSize;

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

/// The largest distance between any drawn point and `centre`, or
/// `NaN` if any of them is not a distance.
///
/// The fold is NOT `f64::max`, which answers with the other operand
/// against a `NaN` and would report a drawing containing `[NaN, NaN,
/// NaN]` as reaching however far its finite positions do. This is the
/// instrument the refusal rows measure with, so a substitution inside
/// it reads as a passing assertion.
fn reach(segments: &[[f64; 3]], centre: [f64; 3]) -> f64 {
    segments
        .iter()
        .map(|p| {
            let (dx, dy, dz) = (p[0] - centre[0], p[1] - centre[1], p[2] - centre[2]);
            (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
        })
        .fold(0.0_f64, |acc, d| {
            if acc.is_nan() || d.is_nan() {
                f64::NAN
            } else {
                acc.max(d)
            }
        })
}

/// Every segment's length, as a list, so a row can say what a drawing
/// is made of rather than only that its numbers are finite.
fn segment_lengths(segments: &[[f64; 3]]) -> Vec<f64> {
    segments
        .chunks_exact(2)
        .map(|pair| {
            let (a, b) = (pair[0], pair[1]);
            ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
        })
        .collect()
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
        "two barbs per arrow have to point off both axes, or the arrow \
         is drawn on top of a grid line and shows nothing",
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
    // The square's frame is a datum and DOES draw; the profile drawn
    // on it is the ordinary node this row is about.
    let (doc, tol) = evaluated(vec![
        common::xy_frame(),
        square(pncad::document::RecipeNodeId(0), 0.02),
        point([0.0, 0.0, 0.0]),
    ]);
    let drawn = draws(&doc, tol, [0.0, -0.15, 0.1]);
    assert_eq!(drawn.len(), 2, "the frame and the point, not the profile");
    assert!(
        drawn.iter().any(|d| d.kind == DatumKind::Point),
        "the point draws: {drawn:?}"
    );
    assert!(
        drawn.iter().any(|d| d.kind == DatumKind::Frame),
        "the frame draws: {drawn:?}"
    );
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
        let pitch = grid_pitch(per_pixel).expect("a positive finite scale has a rung");
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
        let pitch = grid_pitch(per_pixel).expect("a positive finite scale has a rung");
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

/// **`grid_pitch` refuses a scale that is not a positive length**, and
/// reads every one that is.
///
/// The row above asserts what the drawing does with the refusal; this
/// one asserts the refusal, at the door, over the inputs a `View` can
/// actually put through it. `f64::MAX` is in the list because the
/// overflow happens INSIDE the function — the scale is finite and the
/// span it wants is not — which is the case a caller checking its own
/// argument would miss.
///
/// **The value that makes this false** is any `Some(_)` on the first
/// list: on the rung side a refused scale used to answer
/// `f64::MIN_POSITIVE`, a number indistinguishable at the call site
/// from a reading of a very close plane.
#[test]
fn grid_pitch_refuses_a_scale_that_is_not_a_positive_length() {
    for bad in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        0.0,
        -1.0e-3,
        f64::MAX,
    ] {
        assert_eq!(
            grid_pitch(bad),
            None,
            "grid_pitch({bad:e}) answered with a rung",
        );
    }
    for good in [f64::MIN_POSITIVE, 1.0e-9, 1.0e-3, 1.0, 1.0e6, 1.0e300] {
        let pitch = grid_pitch(good);
        assert!(
            pitch.is_some_and(|p| p.is_finite() && p > 0.0),
            "grid_pitch({good:e}) answered {pitch:?} for a positive finite scale",
        );
    }
}

/// **A view that lends a datum no scale draws NOTHING**, for EVERY
/// kind, rather than a lattice of infinities.
///
/// Every mark this module draws is a pixel count read into world
/// metres against the view, and a view can fail to lend one: with the
/// eye at the far corner of representable space the eye-to-datum
/// distance overflows to infinity, so world-per-pixel is infinite and
/// there is no span to scale a mark by. The only honest answer is no
/// mark. A substituted size is a number the module did not compute,
/// and nothing downstream can tell it from one it did.
///
/// **All four kinds, in one document, because the sibling kinds are
/// where this was nearly missed.** The plane and the frame go through
/// the pitch; the axis and the point never touch it and reach the
/// same infinity through their own arithmetic. A row over a one-datum
/// document would assert the invariant for a quarter of the module
/// and read as if it covered all of it.
///
/// **The values that make this false** (measured on `main`, this same
/// eye): a plane draws 390 positions of `NaN` — 97 ruled lines each
/// way at a substituted `f64::MIN_POSITIVE` pitch, plus a normal tick
/// — a frame draws those and its arms, an axis draws 6 positions the
/// first of which is `[NaN, -inf, NaN]`, and a point draws 6 the
/// first of which is `[-inf, 0.0, 0.0]`.
#[test]
fn a_view_with_no_finite_scale_draws_nothing() {
    let (doc, tol) = evaluated(vec![
        plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        axis([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        point([0.0, 0.0, 0.0]),
    ]);
    let drawn = draws(&doc, tol, [f64::MAX, f64::MAX, f64::MAX]);
    assert_eq!(drawn.len(), 4, "the fixture is meant to cover every kind");
    for d in &drawn {
        assert!(
            d.segments.is_empty(),
            "an infinite world-per-pixel drew {} positions for a {}, the first at {:?}",
            d.segments.len(),
            d.kind.label(),
            d.segments.first(),
        );
    }
}

/// **A frame the view cannot RULE still says which way it is turned.**
///
/// The two marks are scaled at two different points — the ruling at
/// the patch's centre, which is what the camera is aimed at, and the
/// arms at the frame's own origin — so they are two different depths
/// and a refusal of one is not a refusal of the other. Looking at a
/// point out at the end of the number line, from a camera a decimetre
/// off the origin, is exactly that case: the centre's scale overflows
/// and the origin's is an ordinary hundredth of a metre.
///
/// **The value that makes this false** is an empty drawing: a frame
/// that dropped its arms with its patch would lose a mark it could
/// have drawn, where a point and an axis at the same origin both draw
/// normally.
#[test]
fn a_frame_keeps_its_arms_when_only_the_patch_has_no_scale() {
    let (doc, tol) = evaluated(vec![frame(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
    )]);
    let evaluation = evaluate(
        &doc,
        None,
        &CancelToken::default(),
        &EvalOptions::default(),
        tol,
    );
    let view = view_at([0.0, 0.0, 0.1], [f64::MAX, 0.0, 0.0]);
    // The premise, asserted rather than assumed: the eye-to-centre
    // distance overflows, so the patch's scale is refused, while the
    // eye-to-origin distance is a tenth of a metre.
    let to_centre = reach(&[[f64::MAX, 0.0, 0.0]], [0.0, 0.0, 0.1]);
    assert!(
        grid_pitch(view.metres_per_pixel_at_one_metre * to_centre).is_none(),
        "this row needs a looked-at point whose scale overflows, not {to_centre:e} m",
    );
    let segments = &datums::draws(&doc, &evaluation, view)[0].segments;
    assert!(
        !segments.is_empty(),
        "the frame drew nothing, so it lost its arms with its patch",
    );
    for p in segments {
        assert!(
            p.iter().all(|c| c.is_finite()),
            "the frame drew {p:?}, which is not a position",
        );
    }
    // **The arms specifically, not just something.** Stated
    // structurally rather than against a copy of `FRAME_ARM_PX`,
    // which is private and would go stale silently: this frame's
    // normal is +z, so its tick is the only mark ON the z axis and
    // the arms are the only ones that leave it.
    assert!(
        segments
            .iter()
            .any(|p| p[0].abs() > 0.0 || p[1].abs() > 0.0),
        "the frame drew only its normal tick — the arms went with the patch",
    );
    // And what is drawn is a screen-sized mark at the ORIGIN's scale,
    // not a patch that slipped through: the refused patch at this
    // view would have been ~1e305 m across.
    let reached = reach(segments, [0.0, 0.0, 0.0]);
    assert!(
        (0.0..1.0).contains(&reached),
        "the frame reached {reached:e} m — that is patch-sized, not a mark",
    );
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
        let pitch = grid_pitch(per_pixel).expect("a positive finite scale has a rung");
        // The normal tick is anchored on the origin and says nothing
        // about the ruling, so it is dropped — by SHAPE, not by
        // position. It used to be the last pair by construction;
        // since each mark refuses on its own scale it may not be
        // drawn at all, and a slice off the end would silently take a
        // ruled line with it (and underflow on an empty drawing). The
        // tick is the one pair that leaves the plane.
        let ruled: Vec<&[[f64; 3]]> = segments
            .chunks_exact(2)
            .filter(|pair| pair[0][2].abs() < 1.0e-12 && pair[1][2].abs() < 1.0e-12)
            .collect();
        assert!(
            !ruled.is_empty(),
            "looking at {look_at:?}, the plane ruled nothing to check",
        );
        for pair in ruled {
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

/// Every drawing this document makes under `view`.
fn drawn_under(doc: &Doc<ProfileProgram>, tol: Tol, view: View) -> Vec<datums::DatumDraw> {
    let evaluation = evaluate(
        doc,
        None,
        &CancelToken::default(),
        &EvalOptions::default(),
        tol,
    );
    datums::draws(doc, &evaluation, view)
}

/// One of each kind at the same origin, for the rows that ask what a
/// whole picture does under a view that has gone wrong.
fn one_of_each(origin: [f64; 3]) -> Vec<Node<ProfileProgram>> {
    vec![
        plane(origin, [0.0, 0.0, 1.0]),
        frame(origin, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        axis(origin, [0.0, 1.0, 0.0]),
        point(origin),
    ]
}

/// Every position of every drawing, with the kind that made it.
fn every_position(drawn: &[datums::DatumDraw]) -> Vec<(DatumKind, [f64; 3])> {
    drawn
        .iter()
        .flat_map(|d| d.segments.iter().map(move |p| (d.kind, *p)))
        .collect()
}

/// **A datum out at the end of the number line draws nothing, rather
/// than a ruling whose lines have no length.**
///
/// The plane here is `z = 0` and its origin is a point on it at
/// `x = f64::MAX`. The camera is aimed at the world origin, which is
/// ALSO on that plane — so the patch's centre is an ordinary point a
/// decimetre from the eye and every scale door says yes. What cannot
/// survive is the datum's own magnitude: the patch's ends are
/// `cv ± half` in the plane's coordinates with `cv ≈ 1.8e308` and
/// `half ≈ 0.26`, and `half` is far below the spacing of the
/// representable numbers there, so both ends round to `cv`.
///
/// **Two values make this false, and the second is why the row does
/// not stop at finiteness.**
///
/// - `[-inf, NaN, NaN]`, twice per plane-like kind: an INCLUSIVE
///   range over a count cast from `inf - inf` rules one line at
///   `inf * pitch`, because the saturating cast reads a NaN
///   difference as the integer zero that means "one line fits".
/// - **27 zero-length segments** per plane-like kind, every one at
///   `[0, y, 0]`: finite, in the right plane, and not lines. A gate
///   that asks only `is_finite` passes them, and an assertion that
///   asks only `is_finite` gets EASIER as the answer degrades —
///   which is what a ruling collapsed onto its own centre is.
#[test]
fn a_datum_at_the_end_of_the_number_line_rules_no_line_at_infinity() {
    let far = [f64::MAX, 0.0, 0.0];
    let (doc, tol) = evaluated(one_of_each(far));
    let view = view_at([0.0, -0.15, 0.1], [0.0, 0.0, 0.0]);
    let drawn = drawn_under(&doc, tol, view);
    let stray: Vec<_> = every_position(&drawn)
        .into_iter()
        .filter(|(_, p)| !p.iter().all(|c| c.is_finite()))
        .collect();
    assert!(
        stray.is_empty(),
        "the drawing left {} positions that are not positions: {:?}",
        stray.len(),
        &stray[..stray.len().min(4)],
    );
    // The half this row exists for: what IS drawn has to be drawn.
    for d in &drawn {
        let lengths = segment_lengths(&d.segments);
        let dead = lengths.iter().filter(|n| n.is_nan() || **n <= 0.0).count();
        assert_eq!(
            dead,
            0,
            "the {} drew {dead} of {} segments with no length, the first pair at {:?}",
            d.kind.label(),
            lengths.len(),
            d.segments.first(),
        );
    }
    // At this magnitude every mark refuses, which is the right answer
    // and also an answer a TOTAL refusal would satisfy. So the
    // premise is asserted against the same four datums brought back
    // to the origin, under the same view: nothing above is a fact
    // about the module declining to draw.
    let (near_doc, near_tol) = evaluated(one_of_each([0.0, 0.0, 0.0]));
    for (far_drawn, near_drawn) in drawn.iter().zip(&drawn_under(&near_doc, near_tol, view)) {
        assert_eq!(
            far_drawn.kind, near_drawn.kind,
            "the two drawings disagree on order"
        );
        assert!(
            far_drawn.segments.is_empty(),
            "the {} drew {} positions at f64::MAX, the first at {:?}",
            far_drawn.kind.label(),
            far_drawn.segments.len(),
            far_drawn.segments.first(),
        );
        assert!(
            !near_drawn.segments.is_empty(),
            "the {} drew nothing at the ORIGIN, so this row proves nothing about f64::MAX",
            near_drawn.kind.label(),
        );
    }
}

/// **A camera aimed at something that is not a place draws nothing
/// that reads it, and everything that does not.**
///
/// `look_at` reaches the patch's centre, the axis's centre and
/// nothing else, so the refusal has to land on exactly three of the
/// four kinds: a point's mark is scaled at the point's own position
/// and is still a mark.
///
/// **The value that makes this false** is a drawn `[NaN, NaN, NaN]`.
/// It arrives through the scale rather than through the geometry:
/// `f64::max` returns the other operand against a NaN, so a floor at
/// `f64::MIN_POSITIVE` turns a depth that is not a number into a
/// legitimate positive length and every door downstream says yes over
/// a centre that is still NaN.
#[test]
fn a_look_at_that_is_not_a_place_draws_only_the_marks_that_ignore_it() {
    let (doc, tol) = evaluated(one_of_each([0.0, 0.0, 0.0]));
    let view = view_at([0.0, -0.15, 0.1], [f64::NAN, 0.0, 0.0]);
    let drawn = drawn_under(&doc, tol, view);
    let stray: Vec<_> = every_position(&drawn)
        .into_iter()
        .filter(|(_, p)| !p.iter().all(|c| c.is_finite()))
        .collect();
    assert!(
        stray.is_empty(),
        "the drawing left {} positions that are not positions: {:?}",
        stray.len(),
        &stray[..stray.len().min(4)],
    );
    // What each kind is left with, against the same four datums under
    // an aim that is a place. A plane's tick, a frame's arms and a
    // point's cross are all scaled at the datum's OWN origin and are
    // not in doubt; a plane's ruling and an axis's whole segment are
    // scaled at the centre and are.
    let aimed = drawn_under(&doc, tol, view_at([0.0, -0.15, 0.1], [0.0, 0.0, 0.0]));
    for (bad, good) in drawn.iter().zip(&aimed) {
        assert_eq!(bad.kind, good.kind, "the two drawings disagree on order");
        let (blind, seeing) = (
            reach(&bad.segments, [0.0, 0.0, 0.0]),
            reach(&good.segments, [0.0, 0.0, 0.0]),
        );
        match bad.kind {
            // The segment IS the drawing, and it is centred on the
            // looked-at point: there is no second mark to keep.
            DatumKind::Axis => assert!(
                bad.segments.is_empty(),
                "the axis drew {:?} around a centre that is NaN",
                bad.segments.first(),
            ),
            // Untouched: a point's mark never reads `look_at`, so
            // dropping it would be over-refusal.
            DatumKind::Point => assert!(
                (blind - seeing).abs() < 1.0e-15,
                "the point's mark moved with the aim: {blind:e} m against {seeing:e} m",
            ),
            // Left with the origin-scaled marks alone. Stated as a
            // reach rather than a position count, which would be a
            // second copy of the arrowhead's shape: the patch spans
            // upwards of half the window and these marks are around a
            // hundred pixels, so a quarter separates them by a
            // decade.
            kind => {
                assert!(
                    !bad.segments.is_empty(),
                    "the {} lost its origin-scaled marks too",
                    kind.label()
                );
                assert!(
                    blind < seeing * 0.25,
                    "the {} reached {blind:e} m against an aimed {seeing:e} m — that is patch-sized",
                    kind.label(),
                );
            }
        }
    }
}

/// **The eye exactly on a datum draws nothing, rather than a mark
/// `1e-307 m` across.**
///
/// Reachable by flying the camera into a plane. A floor at a hair
/// above zero keeps the scale total and is argued by the division it
/// would guard against — but no consumer divides: every one of them
/// takes the scale as an `Option` and draws nothing.
///
/// **The value that makes this false** is an arm of
/// `f64::MIN_POSITIVE * POINT_ARM_PX * 0.5` — six positions of a
/// cross whose size came from the floor rather than from the view.
#[test]
fn a_datum_at_the_eye_draws_no_mark() {
    let eye = [0.03, -0.15, 0.1];
    let (doc, tol) = evaluated(one_of_each(eye));
    let view = view_at(eye, eye);
    for d in drawn_under(&doc, tol, view) {
        assert!(
            d.segments.is_empty(),
            "the {} drew {:?} at zero depth",
            d.kind.label(),
            d.segments.first(),
        );
    }
}

/// **A window whose larger side is not a number of pixels rules no
/// patch**, while the marks that do not measure in windows stay.
///
/// [`View::viewport_px`] is the caller's number, and the patch is the
/// one mark sized from it — so this is the door where a viewport that
/// is not one has to be refused, and the normal tick, which is sized
/// in pixels at the origin, is the mark that proves the refusal was
/// the patch's alone.
///
/// **The value that makes this false** is a ruled line: a floor at
/// one pixel turns a viewport that is not a number into a one-pixel
/// window and rules the patch such a window would have.
#[test]
fn a_viewport_that_is_not_a_number_of_pixels_rules_nothing() {
    let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])]);
    let mut view = view_at([0.0, -0.15, 0.1], [0.0, 0.0, 0.0]);
    view.viewport_px = f64::NAN;
    let segments = &drawn_under(&doc, tol, view)[0].segments;
    // The plane is z = 0, so a ruled line is the pair that stays on
    // it and the normal tick is the pair that leaves it — the shape
    // the ruling rows already read this drawing by.
    let ruled = segments
        .chunks_exact(2)
        .filter(|pair| pair[0][2].abs() < 1.0e-12 && pair[1][2].abs() < 1.0e-12)
        .count();
    assert_eq!(
        ruled, 0,
        "a viewport that is not a number ruled {ruled} lines"
    );
    assert!(
        !segments.is_empty(),
        "the normal tick went with the patch — it is sized at the origin, not in windows",
    );
}

/// **A patch that contains no multiple of the pitch rules NO line**,
/// where a patch that contains exactly one rules that one.
///
/// The two are the same integer zero out of `last - first`, and a
/// float→int cast cannot separate them: it saturates, so a negative
/// difference reads as the count that means "one line fits".
///
/// **The value that makes this false** is a line at `2 * pitch`,
/// `0.005 m` from a patch about `9e-4 m` wide — a ruling of a patch
/// that lies entirely between two lattice lines.
///
/// **Both aims are driven**, because "ruled none" is satisfied by
/// anything that declines to rule at all: a guard that refused a
/// four-pixel viewport outright would turn the first half green for
/// the wrong reason. The second half is the same view moved onto a
/// lattice line, where one line per direction is the answer.
#[test]
fn a_patch_between_two_lattice_lines_rules_neither() {
    let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])]);
    // The eye a fixed tenth of a metre above the looked-at point, so
    // the pitch does not move as the aim does and can be solved for
    // once.
    let per_pixel = view_at([0.0, 0.0, 0.1], [0.0, 0.0, 0.0]).metres_per_pixel_at_one_metre * 0.1;
    let pitch = grid_pitch(per_pixel).expect("a positive finite scale has a rung");
    // Aimed at the middle of a cell, then at a lattice line. The
    // plane's normal is +z, so `basis` gives `u = +y` and `v = -x`
    // and a look_at of `[-a, a, 0]` puts both patch coordinates at
    // `a`.
    for (multiple, want) in [(1.5_f64, 0), (2.0, 2)] {
        let aim = multiple * pitch;
        let look_at = [-aim, aim, 0.0];
        let mut view = view_at([look_at[0], look_at[1], 0.1], look_at);
        view.viewport_px = 4.0;
        // The premise, asserted rather than assumed, and read from
        // the module rather than restated: the patch's half-width is
        // `viewport_px * patch_cover() * 0.5 * per_pixel`, so a
        // four-pixel window's patch is a thousandth of a cell and
        // holds at most the one lattice line it straddles.
        let half = view.viewport_px * datums::patch_cover() * 0.5 * per_pixel;
        assert!(
            half < pitch * 0.5,
            "this row needs a patch narrower than a cell: {half:e} m against {pitch:e} m",
        );
        let segments = &drawn_under(&doc, tol, view)[0].segments;
        let ruled: Vec<_> = segments
            .chunks_exact(2)
            .filter(|pair| pair[0][2].abs() < 1.0e-12 && pair[1][2].abs() < 1.0e-12)
            .collect();
        assert_eq!(
            ruled.len(),
            want,
            "a patch centred {multiple} pitches from the origin ruled {} lines: {:?}",
            ruled.len(),
            ruled.first(),
        );
    }
}

/// **`datum_view` carries a window that is not a number of pixels
/// through as one**, rather than repairing it into a one-pixel
/// window.
///
/// The camera's own doors already refuse this shape by name
/// (`camera::finite("viewport height", …)`), and the app's caller
/// returns before this function when a pane has no area — but this is
/// a public door and the scale it hands back is what every mark in
/// the module is a multiple of.
///
/// **The values that make this false** are `2 * tan(fov/2) / 1.0` for
/// the height and the HEIGHT for the width: `f64::max` returns the
/// other operand against a NaN, so `width.max(height)` answers with
/// whichever of the two is a number.
#[test]
fn datum_view_does_not_repair_a_viewport_that_is_not_pixels() {
    let camera = Camera::new(
        Point3::new(0.0, 0.0, 0.0),
        0.15,
        0.0,
        0.0,
        core::f64::consts::FRAC_PI_4,
        0.05,
    )
    .expect("a finite camera");
    for (width_px, height_px) in [(1280.0, f64::NAN), (f64::NAN, 800.0)] {
        let view = datum_view(
            &camera,
            ViewportSize {
                width_px,
                height_px,
            },
        );
        assert!(
            view.viewport_px.is_nan(),
            "a window {width_px} x {height_px} px reported a larger side of {}",
            view.viewport_px,
        );
        // The refusal is not a fact about the struct, it is the
        // reason no mark is invented under it. The height divides the
        // field of view, so a height that is not a number leaves no
        // scale anywhere and NOTHING draws; a width that is not one
        // costs only the marks measured in windows, which is the
        // axis's whole drawing and the plane's ruling.
        let (doc, tol) = evaluated(one_of_each([0.0, 0.0, 0.0]));
        let drawn = drawn_under(&doc, tol, view);
        // A fixture check and nothing more: `draws` pushes one
        // `DatumDraw` per datum node whatever it draws, so this
        // counts the document, not the drawing. What each kind DREW
        // is the loop below.
        assert_eq!(drawn.len(), 4, "the fixture is meant to cover every kind");
        for d in &drawn {
            let patch_sized = height_px.is_nan() || d.kind == DatumKind::Axis;
            assert_eq!(
                d.segments.is_empty(),
                patch_sized,
                "the {} drew {:?} for a {width_px} x {height_px} px window",
                d.kind.label(),
                d.segments.first(),
            );
        }
        if height_px.is_nan() {
            assert!(
                !view.metres_per_pixel_at_one_metre.is_finite(),
                "a height that is not a number lent a scale of {}",
                view.metres_per_pixel_at_one_metre,
            );
        }
    }
}
