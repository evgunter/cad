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
use pncad::geom_core::{Point3, Tol, Vec3};
use viewer::camera::{Camera, CameraError};
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

/// A view from `eye` pointed at `look_at`, a 1280x800 window with
/// world +z up on screen — or +y, looking along z, where +z has no
/// screen direction.
fn view_at(eye: [f64; 3], look_at: [f64; 3]) -> View {
    let height = 800.0;
    let along_z = eye[0] == look_at[0] && eye[1] == look_at[1];
    View {
        eye: Point3::new(eye[0], eye[1], eye[2]),
        look_at: Point3::new(look_at[0], look_at[1], look_at[2]),
        metres_per_pixel_at_one_metre: 2.0 * (core::f64::consts::FRAC_PI_8).tan() / height,
        up: if along_z {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        },
        window_px: [1280.0, height],
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
    datums::draws(doc, &evaluation, view_from(eye)).drawn
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
    // Every RULED line runs along an axis: the six barbs — two per
    // head, and +x wears two heads — are the only segments that may
    // not, so the count below is what "the grid follows the frame"
    // means once the arrows are subtracted.
    assert_eq!(off_axis, 6, "only the six barbs may leave the axes");
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
        off_axis, 6,
        "two barbs per head (one head on +y, two on +x) have to point off \
         both axes, or the arrow is drawn on top of a grid line and shows \
         nothing",
    );
}

/// **The two arms are one length, and +x wears two heads.**
///
/// A grid is symmetric under a quarter turn, so a frame drawn with two
/// identical arrows would name the PAIR of directions without naming
/// which of them the sketch's x is — and that is the difference
/// between a frame and the plane it lies in. The difference is carried
/// by the HEAD, not the arm: two arms of unequal length read as an
/// unbalanced mark, and a head is the part of an arrow a reader
/// actually sees against a ruling that runs along its shaft.
///
/// **The values that make this false**: arms of two lengths, the
/// same number of barbs rooted on each axis, or two +x heads drawn at
/// (nearly) one place, which is one head drawn twice.
#[test]
fn a_frames_arms_match_and_the_x_head_is_doubled() {
    let (doc, tol) = evaluated(vec![frame(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
    )]);
    let segments = &draws(&doc, tol, [0.0, -0.15, 0.1])[0].segments;
    // The marks found by what they ARE, not by their index in the
    // list: an arm is a segment leaving the origin along one of the
    // frame's two axes (the normal tick also leaves the origin and is
    // excluded by running along neither); a barb is a segment that
    // starts ON an axis, away from the origin, and leaves it. A
    // positional read is a test that breaks on a drawing change
    // instead of on a behaviour change.
    let on_axis = |p: [f64; 3], axis: usize| {
        p[axis] > 1.0e-12 && (0..3).all(|i| i == axis || p[i].abs() < 1.0e-12)
    };
    let mut arms: Vec<(usize, f64)> = Vec::new();
    let mut barbs = [0usize; 2];
    // Per +x barb: where along x its tip is, and how far back it runs.
    let mut x_heads: Vec<(f64, f64)> = Vec::new();
    for pair in segments.chunks_exact(2) {
        let d = [
            pair[1][0] - pair[0][0],
            pair[1][1] - pair[0][1],
            pair[1][2] - pair[0][2],
        ];
        let n = (d[0].powi(2) + d[1].powi(2) + d[2].powi(2)).sqrt();
        let along = (0..3).find(|&i| (d[i] / n).abs() > 1.0 - 1.0e-9);
        if reach(&[pair[0]], [0.0, 0.0, 0.0]) < 1.0e-12 {
            if let Some(axis @ (0 | 1)) = along {
                arms.push((axis, n));
            }
        } else if along.is_none() {
            for (axis, count) in barbs.iter_mut().enumerate() {
                if on_axis(pair[0], axis) {
                    *count += 1;
                }
            }
            if on_axis(pair[0], 0) {
                x_heads.push((pair[0][0], pair[0][0] - pair[1][0]));
            }
        }
    }
    arms.sort_by_key(|&(axis, _)| axis);
    assert_eq!(arms.len(), 2, "one arm per axis, got {arms:?}");
    let (x_arm, y_arm) = (arms[0].1, arms[1].1);
    assert!(
        (x_arm - y_arm).abs() <= 1.0e-9 * x_arm,
        "the arms have to be one length from the origin, not {x_arm:e} m \
         against {y_arm:e} m",
    );
    assert_eq!(
        barbs,
        [4, 2],
        "+x carries two heads and +y one, or the picture is symmetric \
         under a quarter turn",
    );
    // The two +x heads are two, not one drawn twice: their tips stand
    // apart along the axis by a real share of a head's length.
    let head = x_heads
        .iter()
        .map(|&(_, back)| back)
        .fold(0.0_f64, f64::max);
    let (near, far) = x_heads
        .iter()
        .fold((f64::INFINITY, 0.0_f64), |(lo, hi), &(tip, _)| {
            (lo.min(tip), hi.max(tip))
        });
    assert!(
        head > 0.0 && far - near >= 0.5 * head,
        "the +x tips sit {:e} m apart against a {head:e} m head — the \
         doubled head has collapsed into one",
        far - near,
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
        let window = per_pixel * view.window_px[0];
        assert!(
            reach(segments, [0.0, 0.0, 0.0]) >= window * 0.5,
            "at {height:.5} m the patch reached {:.5} m, inside the {window:.5} m window",
            reach(segments, [0.0, 0.0, 0.0]),
        );
        height *= 0.5;
    }
}

/// **A plane seen at a grazing angle is ruled across the window and
/// out toward its horizon**, not in a square that ends partway up it.
///
/// The eye is a metre from the looked-at point and five degrees above
/// the plane. The two bottom corners of the window look down onto the
/// plane, and where they land has to be inside the ruling; the top of
/// the window looks above the horizon, so the ruling runs away from
/// the eye until cells shrink below legibility — several metres here.
///
/// **The value that makes this false** is a patch sized as if the
/// plane faced the eye: 2.2 windows at the looked-at depth, which
/// reaches about 1.5 m along the plane and ends in plain view.
#[test]
fn a_grazing_view_is_ruled_to_the_window_and_toward_the_horizon() {
    let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])]);
    let elevation = 5.0_f64.to_radians();
    let eye = [0.0, -elevation.cos(), elevation.sin()];
    let view = view_at(eye, [0.0, 0.0, 0.0]);
    let segments = &drawn_under(&doc, tol, view)[0].segments;
    let ruled: Vec<[f64; 3]> = segments
        .chunks_exact(2)
        .filter(|pair| pair[0][2].abs() < 1.0e-12 && pair[1][2].abs() < 1.0e-12)
        .flatten()
        .copied()
        .collect();
    assert!(!ruled.is_empty(), "the plane ruled nothing");
    let bound = |axis: usize, pick: fn(f64, f64) -> f64, start: f64| {
        ruled.iter().map(|p| p[axis]).fold(start, pick)
    };
    let (x_lo, x_hi) = (
        bound(0, f64::min, f64::INFINITY),
        bound(0, f64::max, f64::NEG_INFINITY),
    );
    let (y_lo, y_hi) = (
        bound(1, f64::min, f64::INFINITY),
        bound(1, f64::max, f64::NEG_INFINITY),
    );
    // The window's bottom corners, cast onto the plane by hand: the
    // view looks along +y and down, with +x to the right.
    let forward = [0.0, elevation.cos(), -elevation.sin()];
    let up = [0.0, elevation.sin(), elevation.cos()];
    let half_x = view.window_px[0] * 0.5 * view.metres_per_pixel_at_one_metre;
    let half_y = view.window_px[1] * 0.5 * view.metres_per_pixel_at_one_metre;
    for side in [-1.0_f64, 1.0] {
        let dir = [
            side * half_x,
            forward[1] - up[1] * half_y,
            forward[2] - up[2] * half_y,
        ];
        let t = -eye[2] / dir[2];
        let hit = [eye[0] + dir[0] * t, eye[1] + dir[1] * t];
        assert!(
            (x_lo..=x_hi).contains(&hit[0]) && (y_lo..=y_hi).contains(&hit[1]),
            "the window's bottom corner sees {hit:?}, outside the ruling \
             x {x_lo:.3}..{x_hi:.3}, y {y_lo:.3}..{y_hi:.3}",
        );
    }
    assert!(
        y_hi > 2.5,
        "the ruling stops {y_hi:.3} m along the plane, in plain view below the horizon",
    );
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
    let segments = &datums::draws(&doc, &evaluation, view).drawn[0].segments;
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
        let segments = &datums::draws(&doc, &evaluation, view).drawn[0].segments;
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
    let window = view.metres_per_pixel_at_one_metre * 0.15 * view.window_px[0];
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
    datums::draws(doc, &evaluation, view).drawn
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

/// **A window whose width is not a number of pixels rules no
/// patch**, while the marks that do not measure in windows stay.
///
/// [`View::window_px`] is the caller's number, and the patch is the
/// one plane mark sized from it — so this is the door where a viewport that
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
    view.window_px[0] = f64::NAN;
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

/// **A window that sees less than one cell rules the whole cells
/// around what it sees**, and never nothing.
///
/// The ruling rounds its region OUTWARD to lattice lines, so a window
/// inside one cell draws that cell's four sides, and a window
/// straddling a lattice line draws the two cells either side of it —
/// three lines a direction. Rounded inward, the first window would
/// rule no line at all: the plane would be on screen with nothing
/// drawn to say so, which is the hole view-relative sizing exists to
/// close.
///
/// **The value that makes this false** is a ruling that rounds either
/// end inward: the first aim then loses both of its lines per
/// direction, the second its outer two.
#[test]
fn a_window_inside_one_cell_rules_that_cell() {
    let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])]);
    // The eye a fixed tenth of a metre above the looked-at point, so
    // the pitch does not move as the aim does and can be solved for
    // once.
    let per_pixel = view_at([0.0, 0.0, 0.1], [0.0, 0.0, 0.0]).metres_per_pixel_at_one_metre * 0.1;
    let pitch = grid_pitch(per_pixel).expect("a positive finite scale has a rung");
    // Aimed at the middle of a cell, then at a lattice line. The
    // plane's normal is +z, so `basis` gives `u = +y` and `v = -x`
    // and a look_at of `[-a, a, 0]` puts both plane coordinates at
    // `a`.
    for (multiple, want) in [(1.5_f64, 4), (2.0, 6)] {
        let aim = multiple * pitch;
        let look_at = [-aim, aim, 0.0];
        let mut view = view_at([look_at[0], look_at[1], 0.1], look_at);
        view.window_px = [4.0, 4.0];
        // The premise, asserted rather than assumed: looking straight
        // down, a four-pixel window sees a square two pixels' worth of
        // plane either side of the aim, well inside half a cell.
        let half = 2.0 * per_pixel;
        assert!(
            half < pitch * 0.5,
            "this row needs a window narrower than a cell: {half:e} m against {pitch:e} m",
        );
        let segments = &drawn_under(&doc, tol, view)[0].segments;
        let ruled: Vec<_> = segments
            .chunks_exact(2)
            .filter(|pair| pair[0][2].abs() < 1.0e-12 && pair[1][2].abs() < 1.0e-12)
            .collect();
        assert_eq!(
            ruled.len(),
            want,
            "a window centred {multiple} pitches from the origin ruled {} lines: {:?}",
            ruled.len(),
            ruled.first(),
        );
    }
}

/// **What `datum_view` answers for a window it takes**: the camera's
/// eye, target and up, the window's two sides in their own order, and
/// the vertical field over the vertical pixel count.
///
/// Through the door, because every other row here drives a
/// hand-built [`View`] (`view_at`) and so agrees with the door by
/// construction and never through it. A wide window and a tall one,
/// so a door that swapped the sides, or scaled by the larger one,
/// reds one of them.
#[test]
fn datum_view_reports_the_camera_and_the_window_it_is_given() {
    let camera = common::framed(16.0 / 9.0);
    for (width_px, height_px) in [(1280.0, 720.0), (600.0, 900.0)] {
        let view = datum_view(
            &camera,
            ViewportSize {
                width_px,
                height_px,
            },
        )
        .expect("a finite window with area has a view");
        assert_eq!(view.window_px, [width_px, height_px]);
        // Component-wise: the geometry types carry no `PartialEq`.
        let (eye, target, up) = (camera.eye(), camera.target(), camera.up());
        assert_eq!([view.eye.x, view.eye.y, view.eye.z], [eye.x, eye.y, eye.z]);
        assert_eq!(
            [view.look_at.x, view.look_at.y, view.look_at.z],
            [target.x, target.y, target.z],
        );
        assert_eq!([view.up.x, view.up.y, view.up.z], [up.x, up.y, up.z]);
        let scale = 2.0 * (camera.fov_y() * 0.5).tan() / height_px;
        assert!(
            (view.metres_per_pixel_at_one_metre - scale).abs() <= scale * 1.0e-15,
            "a {width_px} x {height_px} window scaled {} against {scale}",
            view.metres_per_pixel_at_one_metre,
        );
    }
}

/// **`datum_view` refuses a window that is not a number of pixels,
/// in the camera's own words.**
///
/// The claim is not that it refuses — an `Option` would carry that —
/// but that what a caller reads is what the SIBLING door on the same
/// two quantities says. `Camera::ray_through` takes the same
/// `ViewportSize` and answers a `CameraError` about it, naming the
/// side that was not a number and carrying its value; this door is
/// the other one on those inputs, and a caller holding both must not
/// have to learn two vocabularies for one fact.
///
/// Compared through `Debug` rather than by `==`, because half of
/// these values are `NaN` and a `CameraError` carrying one is not
/// equal to itself.
#[test]
fn datum_view_refuses_a_window_the_way_the_cameras_own_door_does() {
    let camera = refusing_camera();
    for (width_px, height_px) in [
        (1280.0, f64::NAN),
        (f64::NAN, 800.0),
        (1280.0, 0.0),
        (0.0, 800.0),
        (1280.0, f64::INFINITY),
        (f64::INFINITY, 800.0),
    ] {
        let viewport = ViewportSize {
            width_px,
            height_px,
        };
        let Err(refused) = datum_view(&camera, viewport) else {
            panic!("a {width_px} x {height_px} px window came back as a view");
        };
        let Err(sibling) = camera.ray_through([0.0, 0.0], viewport) else {
            panic!("a {width_px} x {height_px} px window is one the camera's own door takes");
        };
        assert_eq!(
            format!("{refused:?}"),
            format!("{sibling:?}"),
            "a {width_px} x {height_px} px window: this door says {refused}, the camera's says {sibling}",
        );
        // And the words reach a reader, which is the whole of what a
        // named refusal buys over a `None`. Asserted per ARM: both of
        // `CameraError`'s sentences happen to contain the word
        // "viewport", so one predicate over both checks neither.
        let words = refused.to_string();
        match refused {
            CameraError::NotFinite { what, value } => {
                // The side named is the side that was wrong, which is
                // the claim a single shared `what` would break.
                let offending = if width_px.is_finite() {
                    "viewport height"
                } else {
                    "viewport width"
                };
                assert_eq!(
                    what, offending,
                    "a {width_px} x {height_px} px window was refused as {words}",
                );
                assert!(
                    words.contains(what) && words.contains(&value.to_string()),
                    "the refusal reads {words}, which does not carry both {what} and {value}",
                );
            }
            other => {
                assert_eq!(
                    other,
                    CameraError::UnusableBounds,
                    "a {width_px} x {height_px} px window was refused as {words}",
                );
                assert!(
                    words.contains("viewport aspect"),
                    "the zero-area refusal reads {words}, which never reaches the viewport",
                );
            }
        }
    }
}

/// **A camera for the rows above**: finite, small, and nothing else
/// about it matters — every refusal they measure is about the window.
fn refusing_camera() -> Camera {
    Camera::new(
        Point3::new(0.0, 0.0, 0.0),
        0.15,
        0.0,
        0.0,
        core::f64::consts::FRAC_PI_4,
        0.05,
    )
    .expect("a finite camera")
}

/// **The pane's guard does not cover this door, and the gap is the
/// INFINITE extent.**
///
/// Both rows this pair closes say the app never reaches
/// `datum_view`'s refusal, because `viewport_ui` returns when
/// `ViewportSize::aspect` refuses a pane with no area. `aspect` asks
/// whether both sides are above zero — and `inf` is above zero, so a
/// pane of infinite extent HAS an aspect and reaches this door. The
/// premise holds for zero and for `NaN` and not in general, which is
/// why this door owes its own answer rather than inheriting one.
#[test]
fn the_panes_aspect_guard_admits_an_extent_this_door_refuses() {
    for (width_px, height_px) in [(1280.0, f64::INFINITY), (f64::INFINITY, 800.0)] {
        let viewport = ViewportSize {
            width_px,
            height_px,
        };
        assert!(
            viewport.aspect().is_some(),
            "a {width_px} x {height_px} px pane has no aspect, so the pane's guard covers this door after all",
        );
        assert!(
            datum_view(&refusing_camera(), viewport).is_err(),
            "and this door took it",
        );
    }
    // The other two shapes, for the contrast the sentence above
    // makes: these the pane's guard does stop.
    for (width_px, height_px) in [(1280.0, 0.0), (1280.0, f64::NAN)] {
        assert!(
            ViewportSize {
                width_px,
                height_px
            }
            .aspect()
            .is_none(),
            "a {width_px} x {height_px} px pane has an aspect",
        );
    }
}

/// **A `View` built by hand still refuses at every door below it**,
/// which is what makes `datum_view`'s refusal a door hardening rather
/// than the only thing standing between this module and a `NaN`.
///
/// `View`'s fields are public — the suite drives them directly — so a
/// window that is not a number of pixels can still be written into
/// one. What each mark does with it is the module's own contract, and
/// the two sides are not symmetric: the height divides the field of
/// view, so a height that is not a number leaves no scale ANYWHERE
/// and nothing draws; a width that is not one costs only the marks
/// measured in windows, which is the axis's whole drawing and the
/// plane's ruling.
#[test]
fn a_hand_built_view_that_is_not_pixels_still_draws_no_invented_mark() {
    for (width_px, height_px) in [(1280.0, f64::NAN), (f64::NAN, 800.0)] {
        let view = View {
            eye: Point3::new(0.0, 0.0, 0.15),
            look_at: Point3::new(0.0, 0.0, 0.0),
            metres_per_pixel_at_one_metre: 2.0 * (core::f64::consts::FRAC_PI_8).tan() / height_px,
            up: Vec3::new(0.0, 1.0, 0.0),
            window_px: [width_px, height_px],
        };
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
    }
}

/// **A view that draws nothing of a datum SAYS SO, and a document
/// with no datums says something else.**
///
/// This is the row's whole shape and it is a DIFFERENCE, not an
/// emptiness: three documents-and-views are measured here and two of
/// them put no datum geometry on the screen. A row asserting only
/// that the segment list is empty would pass on all three and pin
/// neither fact.
///
/// - Four datums out at `f64::MAX`: **four vanished**. Emptied by
///   THREE refusals at once rather than by one, which is measured
///   rather than argued — `a_plane_can_lose_its_extent_while_every_
///   point_of_it_still_has_a_scale` beside this one splits them.
/// - The same four at the origin with the eye exactly on them,
///   reachable by flying the camera into a plane: **four vanished**,
///   and this one IS a pure want of scale — every point of every
///   datum is at a depth of exactly zero.
/// - A document holding no datums at all: **none vanished**, because
///   there was nothing to draw and that is a different sentence.
#[test]
fn how_many_datums_this_view_drew_nothing_of_is_a_fact_the_caller_is_handed() {
    let view = view_at([0.0, -0.15, 0.1], [0.0, 0.0, 0.0]);
    let (far_doc, far_tol) = evaluated(one_of_each([f64::MAX, 0.0, 0.0]));
    let far = drawn_draws(&far_doc, far_tol, view);
    assert_eq!(far.drawn.len(), 4, "the fixture covers every kind");
    assert_eq!(
        far.vanished(),
        4,
        "a datum at the end of the number line drew {:?}",
        far.drawn
            .iter()
            .map(|d| d.segments.len())
            .collect::<Vec<_>>(),
    );

    // The eye ON the datums, which is the other reachable cause: a
    // depth of exactly zero lends no scale at any of their points.
    let (near_doc, near_tol) = evaluated(one_of_each([0.0, 0.0, 0.0]));
    let on_it = view_at([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    let eye_on = drawn_draws(&near_doc, near_tol, on_it);
    assert_eq!(eye_on.vanished(), 4, "the eye is on every one of them");

    // The premise both halves rest on: the same four datums, drawn
    // from somewhere the view CAN scale them, vanish none. Without
    // this the two rows above are satisfied by a module that draws
    // nothing ever.
    let ordinary = drawn_draws(&near_doc, near_tol, view);
    assert_eq!(
        ordinary.vanished(),
        0,
        "the same four datums vanished from an ordinary view, so nothing above is a fact about this view",
    );

    // And the other side of the distinction the count exists to
    // make: no datums at all is not four datums nobody can see.
    let (empty_doc, empty_tol) = evaluated(Vec::new());
    let nothing = drawn_draws(&empty_doc, empty_tol, on_it);
    assert!(nothing.drawn.is_empty(), "the document holds no datums");
    assert_eq!(
        nothing.vanished(),
        0,
        "a document with no datums reported datums it drew nothing of",
    );
}

/// **A plane can lose its EXTENT while every point of it still has a
/// SCALE** — which is why the count is named for what was drawn and
/// not for what was scaled.
///
/// The two predicates are different sets and this row is the witness.
/// Measured on a plane `z = 0` whose origin sits at `x = 1e100`, with
/// an ordinary camera a decimetre from an ordinary looked-at point —
/// so nothing about the VIEW is extreme and the whole of the extremity
/// is the datum's own coordinate:
///
/// - the patch centre is the looked-at point, `0.18 m` from the eye,
///   so the ruling is scaled and `rule_patch` runs;
/// - the datum's ORIGIN is `1e100` from the eye, which is still a
///   depth, so the normal tick is scaled and drawn;
/// - and one ruled direction still comes out no ruling, because its
///   two endpoints are `cv ± half` with `half ≈ 0.26 m` against a
///   spacing of representable numbers around `1e100` of about
///   `2e84` — so both ends round onto `cv`.
///
/// **The same plane at `f64::MAX` is empty for two reasons at once**,
/// which is what makes it the wrong witness for either: the tick's
/// depth overflows to `inf` and it refuses for want of a scale, and
/// one direction's `cv / pitch` overflows, which refuses the whole
/// patch on the finiteness guard before any extent is asked about.
/// At `1e100` the extent loss below is what is live.
///
/// **What makes this falsifiable**: if `rule_patch` emitted the
/// zero-length segments instead of refusing them, the drawing here
/// would carry pairs of no length and the plane at `f64::MAX` would
/// stop being empty at all.
#[test]
fn a_plane_can_lose_its_extent_while_every_point_of_it_still_has_a_scale() {
    let view = view_at([0.0, -0.15, 0.1], [0.0, 0.0, 0.0]);
    let ruled_and_ticked = |magnitude: f64| {
        let (doc, tol) = evaluated(vec![plane([magnitude, 0.0, 0.0], [0.0, 0.0, 1.0])]);
        let drawn = drawn_draws(&doc, tol, view);
        assert_eq!(drawn.drawn.len(), 1, "one plane");
        let segments = drawn.drawn[0].segments.clone();
        // The plane is `z = 0`, so a ruled line is the pair that stays
        // on it and the normal tick is the pair that leaves it.
        let ruled = segments
            .chunks_exact(2)
            .filter(|pair| pair[0][2].abs() < 1.0e-12 && pair[1][2].abs() < 1.0e-12)
            .count();
        (
            ruled,
            segments.len() / 2 - ruled,
            drawn.vanished(),
            segments,
        )
    };

    // The control: a magnitude at which nothing is lost.
    let (near_ruled, near_ticks, near_vanished, _) = ruled_and_ticked(1.0e15);
    assert!(near_ruled > 1, "the control ruled {near_ruled} lines");
    assert_eq!(near_ticks, 1, "and drew its normal tick");
    assert_eq!(near_vanished, 0, "and has not vanished");

    let (ruled, ticks, vanished, segments) = ruled_and_ticked(1.0e100);
    assert_eq!(
        ticks, 1,
        "the tick went, so the ORIGIN has no scale and this row is about the wrong thing",
    );
    assert!(
        ruled < near_ruled,
        "this plane ruled {ruled} lines against the control's {near_ruled}: no extent was lost",
    );
    // The half this row exists for: what survived is a drawing, so the
    // refusal was a refusal and not a collapse emitted as geometry.
    let dead = segment_lengths(&segments)
        .iter()
        .filter(|n| n.is_nan() || **n <= 0.0)
        .count();
    assert_eq!(
        dead,
        0,
        "the plane drew {dead} of {} segments with no length, the first pair at {:?}",
        segments.len() / 2,
        segments.first(),
    );
    assert_eq!(
        vanished, 0,
        "a plane that drew its tick and a line has not vanished",
    );

    // And the far end, where the same plane is empty for three
    // reasons and this one is only the third.
    let (far_ruled, far_ticks, far_vanished, far_segments) = ruled_and_ticked(f64::MAX);
    assert!(
        far_segments.is_empty() && far_ruled == 0 && far_ticks == 0,
        "the plane at f64::MAX drew {far_segments:?}",
    );
    assert_eq!(far_vanished, 1, "and so it vanished");
}

/// Every drawing this document makes under `view`, with the count of
/// the ones that came out empty still attached.
fn drawn_draws(doc: &Doc<ProfileProgram>, tol: Tol, view: View) -> datums::DatumDraws {
    let evaluation = evaluate(
        doc,
        None,
        &CancelToken::default(),
        &EvalOptions::default(),
        tol,
    );
    datums::draws(doc, &evaluation, view)
}

/// **A partial drawing is not a vanished one.**
///
/// The count's own boundary, and the reason it is not "how many
/// datums had a mark refused": a plane whose RULING has no extent
/// still ticks its normal, and something of it is on the screen. A
/// count that took that as vanished would badge a picture a reader
/// can see.
#[test]
fn a_datum_that_drew_some_of_itself_has_not_vanished() {
    // A window that is not a number of pixels: the patch is measured
    // in windows and goes, the normal tick is measured at the origin
    // and stays. The same view the ruling row beside this one uses.
    let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])]);
    let mut view = view_at([0.0, -0.15, 0.1], [0.0, 0.0, 0.0]);
    view.window_px[0] = f64::NAN;
    let drawn = drawn_draws(&doc, tol, view);
    assert_eq!(drawn.drawn.len(), 1, "one plane");
    let ruled = drawn.drawn[0]
        .segments
        .chunks_exact(2)
        .filter(|pair| pair[0][2].abs() < 1.0e-12 && pair[1][2].abs() < 1.0e-12)
        .count();
    assert_eq!(ruled, 0, "the premise: this plane's ruling is gone");
    assert!(
        !drawn.drawn[0].segments.is_empty(),
        "the premise: its normal tick is not",
    );
    assert_eq!(drawn.vanished(), 0, "and so it has not vanished");
}

// ---------------------------------------------------------------
// Which in-plane directions a datum is drawn along
// ---------------------------------------------------------------

/// The normals the three rows below are measured over.
///
/// Not a random spread: each member is a case one of the two
/// constructions singles out. The six world axes are where the local
/// seed rule and the kernel's door happen to agree up to a quarter
/// turn; the equator (`n.z == 0`) is the seam the kernel's door
/// documents, taken from both sides of the signed zero and from just
/// off it; `(1, 1, 1)` and `(1, 1, 0)` sit on the ties the local
/// rule's `<=` chain breaks, which is where a seed rule chosen by
/// comparison changes answer discontinuously; and the near-pole pair
/// is where the naive `1/(1 + n.z)` spelling the kernel's door
/// replaced would have cancelled.
const NORMALS: &[[f64; 3]] = &[
    [1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, -1.0],
    [1.0, 1.0, 0.0],
    [1.0, 1.0, -0.0],
    [1.0, 1.0, 1.0],
    [0.3, 0.5, 0.81],
    [1.0e-9, -1.0e-9, -1.0],
    [0.6, 0.8, 1.0e-12],
    [0.6, 0.8, -1.0e-12],
];

/// `v`, normalized the way the document's own evaluation normalizes a
/// datum's direction, and the kernel's basis for it.
///
/// The normal is re-derived rather than written down because the
/// expected basis has to be the basis OF THE VECTOR THE DRAWING SAW:
/// a literal and its normalization differ in the last bits, and a
/// direction compared at 1e-12 would not notice, but the seed the
/// equator members are chosen for is decided on `n.z`'s sign, which a
/// re-spelling can move.
fn kernel_basis(v: [f64; 3]) -> (Vec3<f64>, Vec3<f64>, Vec3<f64>) {
    let n = Vec3::new(v[0], v[1], v[2]).normalize();
    let (b1, b2) = n.orthonormal_basis();
    (n, b1, b2)
}

/// Every segment's direction, unit, with the zero-length ones refused
/// rather than normalized — a drawing that is not a set of lines is a
/// different failure and `no_datum_draws_a_point_as_a_line` owns it.
fn directions(segments: &[[f64; 3]]) -> Vec<Vec3<f64>> {
    segments
        .chunks_exact(2)
        .map(|pair| {
            let d = Vec3::new(
                pair[1][0] - pair[0][0],
                pair[1][1] - pair[0][1],
                pair[1][2] - pair[0][2],
            );
            let len = d.norm();
            assert!(len > 0.0 && len.is_finite(), "a segment of length {len}");
            Vec3::new(d.x / len, d.y / len, d.z / len)
        })
        .collect()
}

/// `a` and `b` name the same line, either way round.
fn parallel(a: Vec3<f64>, b: Vec3<f64>) -> bool {
    (a.dot(b).abs() - 1.0).abs() <= 1.0e-12
}

/// **A plane's ruling runs along the kernel's orthonormal basis.**
///
/// A plane datum carries a normal and nothing else, so the two
/// in-plane directions it is ruled along are invented — and this is
/// the row that says WHERE they are invented. They are
/// `UnitVec3::orthonormal_basis`'s, the same door the kernel builds a
/// frame from a normal with, rather than a recipe spelled in the
/// viewer: the viewer does not decide how a normal is completed to a
/// frame, and a second construction here is a second answer to a
/// question with one.
///
/// Falsifiable because the two constructions genuinely differ: at
/// `(1, 1, 0)` the local least-aligned-axis seed gives a pair turned
/// 45° from this one about the normal, which is exactly the offset a
/// square grid is NOT symmetric under.
#[test]
fn a_planes_ruling_runs_along_the_kernels_orthonormal_basis() {
    for v in NORMALS {
        let (n, b1, b2) = kernel_basis(*v);
        let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], *v)]);
        let drawn = draws(&doc, tol, [0.05, -0.15, 0.1]);
        assert_eq!(drawn.len(), 1);
        let dirs = directions(&drawn[0].segments);
        assert!(dirs.len() >= 3, "{v:?} drew {} segments", dirs.len());
        let mut ruled = 0;
        for d in &dirs {
            if parallel(*d, n) {
                continue;
            }
            assert!(
                parallel(*d, b1) || parallel(*d, b2),
                "a plane with normal {v:?} ruled a line along \
                 ({:e}, {:e}, {:e}), which is neither of the kernel's \
                 basis axes ({:e}, {:e}, {:e}) and ({:e}, {:e}, {:e})",
                d.x,
                d.y,
                d.z,
                b1.x,
                b1.y,
                b1.z,
                b2.x,
                b2.y,
                b2.z,
            );
            ruled += 1;
        }
        assert!(ruled >= 2, "{v:?} ruled {ruled} lines");
    }
}

/// **An axis datum's end ticks run across it along the kernel's first
/// basis axis.**
///
/// The same claim one dimension down: an axis carries a direction and
/// the tick's own direction is invented, so it comes from the same
/// door as the plane's ruling rather than from a second recipe.
#[test]
fn an_axis_datums_ticks_run_along_the_kernels_first_basis_axis() {
    for v in NORMALS {
        let (n, b1, _) = kernel_basis(*v);
        let (doc, tol) = evaluated(vec![axis([0.0, 0.0, 0.0], *v)]);
        let drawn = draws(&doc, tol, [0.05, -0.15, 0.1]);
        assert_eq!(drawn.len(), 1);
        let dirs = directions(&drawn[0].segments);
        let mut ticks = 0;
        for d in &dirs {
            if parallel(*d, n) {
                continue;
            }
            assert!(
                parallel(*d, b1),
                "an axis along {v:?} ticked along ({:e}, {:e}, {:e}), \
                 not along the kernel's ({:e}, {:e}, {:e})",
                d.x,
                d.y,
                d.z,
                b1.x,
                b1.y,
                b1.z,
            );
            ticks += 1;
        }
        assert_eq!(ticks, 2, "an axis draws a tick at each end");
    }
}

/// **A world-axis plane is still ruled along the other two world
/// axes.**
///
/// The three default planes are the datums a reader sees most, and
/// what they look like is not a free choice this module may make
/// twice. It is a weaker claim than the row above — a square grid is
/// symmetric under a quarter turn, so this holds for any construction
/// whose seed is a world axis — and that is the point: it is the
/// picture rather than the pair, so it says what a READER can see and
/// a change to the pair that a reader cannot see leaves it green.
#[test]
fn a_world_axis_planes_ruling_stays_on_the_other_two_world_axes() {
    let world = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];
    for (axis_index, v) in [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
        .into_iter()
        .enumerate()
    {
        let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], v)]);
        let drawn = draws(&doc, tol, [0.05, -0.15, 0.1]);
        for d in directions(&drawn[0].segments) {
            let along = world
                .iter()
                .position(|w| parallel(d, *w))
                .unwrap_or_else(|| {
                    panic!("the {v:?} plane ruled along ({}, {}, {})", d.x, d.y, d.z)
                });
            if along == axis_index {
                // The normal tick, which is the one mark that is
                // allowed to leave the plane.
                continue;
            }
            assert_ne!(along, axis_index);
        }
    }
}

/// **No normal makes a datum draw something that is not a drawing.**
///
/// The standing guard under the two rows above, and it is green
/// whichever construction supplies the pair — its job is to hold the
/// NEXT one. Every position is a number, every segment is a line
/// rather than a point, and the ruling stays in the plane the normal
/// names.
#[test]
fn no_normal_makes_a_datum_draw_something_that_is_not_a_drawing() {
    for v in NORMALS {
        let (n, _, _) = kernel_basis(*v);
        let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], *v), axis([0.0, 0.0, 0.0], *v)]);
        let drawn = draws(&doc, tol, [0.05, -0.15, 0.1]);
        assert_eq!(drawn.len(), 2);
        for d in &drawn {
            assert!(!d.segments.is_empty(), "{v:?} drew no {}", d.kind.label());
            for p in &d.segments {
                assert!(
                    p.iter().all(|c| c.is_finite()),
                    "{v:?} drew {p:?} for a {}",
                    d.kind.label(),
                );
            }
            // `directions` refuses a zero-length or non-finite
            // segment, so calling it IS the line-not-a-point
            // assertion; what is read back is the in-plane claim.
            for dir in directions(&d.segments) {
                let on_n = dir.dot(n).abs();
                assert!(
                    on_n <= 1.0e-12 || (on_n - 1.0).abs() <= 1.0e-12,
                    "{v:?} drew a {} segment ({}, {}, {}) that is neither \
                     in the plane nor along the normal",
                    d.kind.label(),
                    dir.x,
                    dir.y,
                    dir.z,
                );
            }
        }
    }
}

/// **A patch that holds more lattice lines than the backstop rules is
/// ruled SMALLER, completely, AROUND WHAT THE READER IS LOOKING AT**
/// — not ruled as far as the backstop reaches and then handed over in
/// the shape of a whole ruling, and not shrunk onto some other part
/// of the plane.
///
/// Three claims, and each is a different way for a capped drawing to
/// be a lie.
///
/// **It closes.** Each family's lines run between the other family's
/// outermost lines, so the picture is a rectangle with four edges
/// rather than one with a side ruled off past where the lines
/// crossing it stop. Truncating the line list leaves one family
/// running to a bound the other never reached, and the two extents
/// disagree by whatever the backstop cut.
///
/// **It covers the aim.** The ruled rectangle contains the looked-at
/// point. The region a plane is ruled over is NOT centred on that
/// point — at the grazing seat that reaches the cap at all, its near
/// edge is where the bottom of the window lands and its far edge is
/// the cut-off toward the horizon — so a patch shrunk onto the
/// region's own midpoint walks away from the aim as the window grows
/// and eventually leaves the reader looking at bare plane through a
/// hole in a grid that is complete everywhere else. The tall panes
/// below are past that point: at 1920x7680, shrinking onto the
/// region's midpoint rules y from +0.32 to +5.43 and the aim is at 0.
///
/// **It stops growing.** The narrow pair fixes the premise — while
/// the backstop is slack the ruling is the window's, so tripling the
/// window triples the ruled width — and the wide pair is where it
/// bites.
///
/// Every window here is one a display can be, seen from an orbit
/// position rather than a pathology, which is what makes the cap
/// worth a row at all.
#[test]
fn a_patch_past_the_grid_backstop_is_shrunk_rather_than_truncated() {
    let (doc, tol) = evaluated(vec![plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])]);
    // Half a degree above the plane, looking at the origin: the seat
    // that rules a plane out toward its horizon and so asks for the
    // most lines a view can ask for. The aim is therefore the plane's
    // own origin, and a rectangle covering it is one covering `[0, 0]`.
    let elevation = 0.5_f64.to_radians();
    let eye = [0.0, -elevation.cos(), elevation.sin()];
    // The ruled rectangle at each window, as `[x_lo, x_hi, y_lo,
    // y_hi]`, with the two claims that hold at EVERY window checked
    // here rather than at the two the growth claim compares.
    let ruled_box = |[width_px, height_px]: [f64; 2]| -> [f64; 4] {
        let mut view = view_at(eye, [0.0, 0.0, 0.0]);
        // Both sides of the window, and the scale the window implies
        // — `datum_view` reads metres-per-pixel off the HEIGHT, so a
        // pane that is taller is finer, and a view that set one
        // without the other would not be a pane.
        view.window_px = [width_px, height_px];
        view.metres_per_pixel_at_one_metre = 2.0 * (core::f64::consts::FRAC_PI_8).tan() / height_px;
        let segments = &drawn_under(&doc, tol, view)[0].segments;
        // The plane's normal tick is the one mark off the plane.
        let ruled: Vec<[[f64; 3]; 2]> = segments
            .chunks_exact(2)
            .filter(|pair| pair[0][2].abs() < 1.0e-12 && pair[1][2].abs() < 1.0e-12)
            .map(|pair| [pair[0], pair[1]])
            .collect();
        let window = format!("{width_px}x{height_px}");
        assert!(!ruled.is_empty(), "a {window} window ruled nothing");
        // A ruled line holds one plane coordinate and runs along the
        // other, so which coordinate it holds names its family.
        let family = |held: usize| -> Vec<&[[f64; 3]; 2]> {
            ruled
                .iter()
                .filter(|line| (line[0][held] - line[1][held]).abs() < 1.0e-12)
                .collect()
        };
        let (held_x, held_y) = (family(0), family(1));
        assert_eq!(
            held_x.len() + held_y.len(),
            ruled.len(),
            "a {window} window ruled a line belonging to both families or to neither",
        );
        assert!(
            !held_x.is_empty() && !held_y.is_empty(),
            "a {window} window ruled only one family: {} holding x, {} holding y",
            held_x.len(),
            held_y.len(),
        );
        let box_of = |lines: &[&[[f64; 3]; 2]]| {
            let mut bounds = [
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
            ];
            for line in lines {
                for point in line.iter() {
                    bounds = [
                        bounds[0].min(point[0]),
                        bounds[1].max(point[0]),
                        bounds[2].min(point[1]),
                        bounds[3].max(point[1]),
                    ];
                }
            }
            bounds
        };
        let (a, b) = (box_of(&held_x), box_of(&held_y));
        assert_eq!(
            a, b,
            "at {window} the two families rule different rectangles — {a:?} against \
             {b:?} — so the drawing does not close",
        );
        assert!(
            a[0] <= 0.0 && 0.0 <= a[1] && a[2] <= 0.0 && 0.0 <= a[3],
            "at {window} the ruling is x {:.3}..{:.3}, y {:.3}..{:.3} — the reader is \
             looking at [0, 0] and there is no grid there",
            a[0],
            a[1],
            a[2],
            a[3],
        );
        // **And it is plane the window can SEE.** A shrunk patch that
        // is free to sit anywhere rules the aim's neighbourhood
        // whether or not the window reaches it, which is the same
        // untrue drawing pointed the other way. The eye looks along
        // +y from `eye`, so nothing the window sees is behind it in
        // y — give or take the one whole cell the region's bounds are
        // rounded outward by, which is read off the drawing rather
        // than assumed.
        let mut held: Vec<f64> = held_y.iter().map(|line| line[0][1]).collect();
        held.sort_by(f64::total_cmp);
        let pitch = held
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .fold(f64::INFINITY, f64::min);
        assert!(
            a[2] >= eye[1] - pitch,
            "at {window} the ruling reaches y {:.3}, behind an eye at y {:.3} by more \
             than the {pitch:.3} m cell the bounds are rounded out by",
            a[2],
            eye[1],
        );
        a
    };
    // Widening the window, which caps the ruling ACROSS the view.
    let width = |b: [f64; 4]| b[1] - b[0];
    let narrow = width(ruled_box([1280.0, 800.0]));
    let tripled = width(ruled_box([3840.0, 800.0]));
    let wide = width(ruled_box([7680.0, 800.0]));
    let doubled = width(ruled_box([15360.0, 800.0]));
    assert!(
        (tripled / narrow - 3.0).abs() < 0.05,
        "tripling the window over a {narrow:.1} m ruling gave {tripled:.1} m, \
         not three times it",
    );
    // **Where it bites, the patch stops growing rather than the line
    // list stopping.** Two statements of that, and the band between
    // them is what the constants say.
    //
    // A window asking for twice as much gets back the SAME rectangle,
    // to the bit: both are the cap's worth of lines on one lattice,
    // clamped against a region bound that did not move, so every
    // coordinate is the same product of the same two numbers. A
    // tolerance here would admit a patch that drifted a cell.
    //
    // And the wide window is short of proportional by more than a
    // tenth. The premise above says an uncapped ruling would have
    // doubled; measured, this one is 1.59 times the 3840-pixel
    // ruling against 1.99 uncapped, so `1.8` sits about a fifth of
    // the gap from either.
    assert!(
        doubled == wide && wide < tripled * 2.0 * 0.9,
        "the backstop did not bite: {tripled:.1} m at 3840 px, {wide:.1} m at 7680 px, \
         {doubled:.1} m at 15360 px",
    );
    // Heightening the window instead, which caps the ruling ALONG the
    // view — the direction whose region is asymmetric about the aim,
    // and the one a shrink toward the region's midpoint loses. The
    // `ruled_box` claims above are the assertion; these two windows
    // are here to put a capped patch in front of them.
    let along = |b: [f64; 4]| b[3] - b[2];
    let (short, tall) = (
        along(ruled_box([1920.0, 6400.0])),
        along(ruled_box([1920.0, 7680.0])),
    );
    assert!(
        short == tall,
        "a taller pane moved the ruling along the view: {short:.2} m at 6400 px, \
         {tall:.2} m at 7680 px",
    );
}
