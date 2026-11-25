//! **A copy drawn under a map shows every edge it faces the eye
//! with.** A part under a display map (a free-move frame, a placement
//! draft's carry) or a body frame has its faces picked on the caller's
//! ray carried back through those maps, and the occlusion probe must
//! read the edge's own crossing on that same ray, or the carry's
//! rounding can put the edge behind its own face.
//!
//! A counterexample search (implementer-discipline §8 shape 1): **an
//! edge of a moved copy is not hidden by its own face** — the cursor on
//! a visible edge's midpoint picks that edge, under rotated and
//! translated free-move frames, placement-draft carries and posed
//! copies, across the ratio of the move to the body and to the eye.
//! Beside it, a counterexample it found, pinned as a fixed row.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

test_utils::gated_to![
    "crates/viewer/src/pickindex.rs",
    "crates/viewer/src/display.rs",
    "crates/viewer/src/camera.rs",
    "crates/viewer/src/scene.rs",
    "crates/editor-core/src/resolve/pick.rs",
    "crates/geom-core/src/linalg/",
    "crates/viewer/tests/common/",
];

use std::collections::BTreeMap;

use pncad::document::{Evaluation, Frame, Operand, RecipeNodeId, VarId};
use pncad::geom_core::{Point3, Tol, Vec3};
use test_utils::fuzz;
use viewer::camera::Camera;
use viewer::display::{DisplayView, PreviewedCopy, PreviewedPlacement};
use viewer::input::ViewportSize;
use viewer::pickindex::{EdgeId, PickIndex};
use viewer::session::DocSession;

use crate::common;

/// The box every row draws: three distinct sides, so no two of its
/// faces are alike.
const SIZE: [f64; 3] = [1.0, 0.6, 0.4];

/// How far out a copy can be posed and still be drawn: a model's
/// coordinates stay inside the session box, a copy's corners included.
fn farthest_pose() -> f64 {
    0.9 * pncad::geom_core::tolerance::SESSION_BOX_EXTENT
}

fn pane() -> ViewportSize {
    ViewportSize {
        width_px: 1280.0,
        height_px: 720.0,
    }
}

fn delta() -> viewer::scene::DisplayTolerance {
    viewer::scene::DisplayTolerance::new(1.0e-3).expect("a positive delta")
}

fn unit(v: Vec3<f64>) -> Vec3<f64> {
    v * (1.0 / v.dot(v).sqrt())
}

fn random_unit(rng: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let n = v.dot(v);
        if n > 1.0e-4 && n <= 1.0 {
            return unit(v);
        }
    }
}

/// A rotation by `angle` about the unit `axis` (Rodrigues, rounded as
/// written), then the translation `t`: a free-move frame as a drag
/// leaves one, near-orthonormal to its rounding.
fn rigid(axis: Vec3<f64>, angle: f64, t: [f64; 3]) -> Frame {
    let (s, c) = angle.sin_cos();
    let k = 1.0 - c;
    let [x, y, z] = [axis.x, axis.y, axis.z];
    Frame {
        columns: [
            [c + x * x * k, y * x * k + z * s, z * x * k - y * s],
            [x * y * k - z * s, c + y * y * k, z * y * k + x * s],
            [x * z * k + y * s, y * z * k - x * s, c + z * z * k],
        ],
        translation: t,
    }
}

/// A translation of magnitude `scale` in a random direction.
fn shift(rng: &mut fuzz::Rng, scale: f64) -> [f64; 3] {
    let d = random_unit(rng);
    [d.x * scale, d.y * scale, d.z * scale]
}

/// A random rigid frame whose translation is `scale` long, rotated by
/// any angle about any axis.
fn random_rigid(rng: &mut fuzz::Rng, scale: f64) -> Frame {
    let axis = random_unit(rng);
    let angle = rng.range(-std::f64::consts::PI, std::f64::consts::PI);
    rigid(axis, angle, shift(rng, scale))
}

/// A landed session over one box and `copies` world copies of it, each
/// pinned at its pose.
fn boxes_posed(poses: &[Frame], tol: Tol) -> (DocSession, Vec<RecipeNodeId>) {
    let mut session =
        common::inline_without_door(pncad::document::Doc::empty_derived("moved-copy", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let mut doc = session.committed_doc().clone();
    let mut copies = Vec::new();
    for _ in poses {
        let (next, copy) = crate::fixture::insert(doc, common::world_place(Operand::Node(body)));
        doc = next;
        copies.push(copy);
    }
    for (copy, pose) in copies.iter().zip(poses) {
        doc = common::drawn::pinned(doc, *copy, pose);
    }
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    (session, copies)
}

/// The session's pick index, every copy drawn: a copy the evaluation
/// refused is out of the picture, and a row over it would ask nothing.
fn drawn_index(session: &DocSession) -> PickIndex {
    let index = common::index_of(session, delta());
    assert!(
        index.undrawn().is_empty(),
        "every copy is drawn: {:?}",
        index.undrawn()
    );
    index
}

fn eval_of(session: &DocSession) -> &Evaluation<f64> {
    session.evaluation().expect("an evaluation has landed")
}

/// The pixel `point` projects to, or `None` off the eye's side.
fn pixel_of(camera: &Camera, point: Point3<f64>) -> Option<[f64; 2]> {
    let aspect = pane().aspect().expect("a positive aspect");
    let ndc = camera.project(point, aspect).expect("projects")?;
    pane().cursor_of([ndc[0], ndc[1]])
}

/// The distance from pixel `p` to the segment `a`–`b`.
fn to_segment(p: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let len = dx * dx + dy * dy;
    let s = if len > 0.0 {
        (((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / len).clamp(0.0, 1.0)
    } else {
        0.0
    };
    (p[0] - a[0] - s * dx).hypot(p[1] - a[1] - s * dy)
}

/// One drawn edge of the box: its id, its world polyline, and the
/// outward normals of the two faces it bounds, read off the drawn
/// corners (at an end, the other two edges run into the box, so the
/// face spanned by this edge and one of them faces away from the
/// other).
struct DrawnEdge {
    id: EdgeId,
    points: Vec<Point3<f64>>,
    normals: [Vec3<f64>; 2],
}

fn drawn_edges(index: &PickIndex, copy: RecipeNodeId, display: &DisplayView) -> Vec<DrawnEdge> {
    let lines: Vec<(EdgeId, Vec<Point3<f64>>)> = index
        .edges_in(copy, 0)
        .iter()
        .map(|&id| (id, index.edge_polyline_for(id, display)))
        .collect();
    assert_eq!(lines.len(), 12, "a box draws twelve edges");
    let ends = |points: &[Point3<f64>]| (points[0], points[points.len() - 1]);
    let same = |a: Point3<f64>, b: Point3<f64>| (a - b).dot(a - b).sqrt() <= 1.0e-6;
    lines
        .iter()
        .map(|(id, points)| {
            let (start, end) = ends(points);
            let along = unit(end - start);
            let others: Vec<Vec3<f64>> = lines
                .iter()
                .filter(|(other, _)| other != id)
                .filter_map(|(_, other)| {
                    let (a, b) = ends(other);
                    if same(a, start) {
                        Some(unit(b - a))
                    } else if same(b, start) {
                        Some(unit(a - b))
                    } else {
                        None
                    }
                })
                .filter(|d| d.dot(along).abs() < 1.0e-6)
                .collect();
            let [d1, d2] = others[..] else {
                panic!("a box corner meets two other edges: {others:?}")
            };
            DrawnEdge {
                id: *id,
                points: points.clone(),
                normals: [d2 * -1.0, d1 * -1.0],
            }
        })
        .collect()
}

/// The midpoint of the polyline's middle segment.
fn middle(points: &[Point3<f64>]) -> Point3<f64> {
    let at = (points.len() - 1) / 2;
    let (a, b) = (points[at], points[at + 1]);
    Point3::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5, (a.z + b.z) * 0.5)
}

/// The yaw and pitch that put the eye along the unit `u` from the
/// target ([`Camera::direction_to_eye`] inverted).
fn yaw_pitch(u: Vec3<f64>) -> (f64, f64) {
    (u.y.atan2(u.x), u.z.clamp(-1.0, 1.0).asin())
}

/// How a draw maps the copy: the display view it is drawn under, and
/// what the draw was, for the message.
fn random_display(
    rng: &mut fuzz::Rng,
    copy: RecipeNodeId,
    pose: &Frame,
    scale: f64,
) -> (DisplayView, String) {
    match rng.below(5) {
        0 => {
            let frame = random_rigid(rng, scale);
            let what = format!("free-moved by {frame:?}");
            (
                DisplayView {
                    moved_nodes: BTreeMap::from([(copy, frame)]),
                    ..DisplayView::none()
                },
                what,
            )
        }
        1 => {
            let frame = Frame::translation(shift(rng, scale));
            let what = format!("free-moved by {frame:?}");
            (
                DisplayView {
                    moved_nodes: BTreeMap::from([(copy, frame)]),
                    ..DisplayView::none()
                },
                what,
            )
        }
        2 => {
            // The placement editor's draft: the copy stood elsewhere in
            // its space, that space's display frame on top.
            let stands = random_rigid(rng, scale);
            let reach = scale * rng.log_uniform(1.0e-2, 1.0);
            let space = random_rigid(rng, reach);
            let what = format!("drafted to {stands:?} under the space frame {space:?}");
            (
                DisplayView {
                    moved: BTreeMap::from([(VarId::WORLD, space)]),
                    ..DisplayView::none()
                }
                .placing(Some(PreviewedPlacement {
                    copies: BTreeMap::from([(
                        copy,
                        PreviewedCopy {
                            space: VarId::WORLD,
                            drawn: *pose,
                            stands,
                        },
                    )]),
                })),
                what,
            )
        }
        3 => {
            // Moved back from where it is posed: the frame undoes the
            // pose and lands the copy within `scale` of the origin, so
            // its translation cancels the pose's.
            let near = random_rigid(rng, scale.min(1.0));
            let back = Frame::from_affine(near.affine() * pose.affine().inverse());
            let what = format!("moved back from its pose by {back:?}");
            (
                DisplayView {
                    moved_nodes: BTreeMap::from([(copy, back)]),
                    ..DisplayView::none()
                },
                what,
            )
        }
        _ => (DisplayView::none(), "not moved".to_owned()),
    }
}

/// What one hover asked: nothing (the eye cannot see the edge, or the
/// cursor is off the pane), nothing because another edge is drawn
/// within a pixel of the cursor (the distance contest is the door's,
/// and not what these rows check), or the answer, right or wrong.
enum Hover {
    Unasked,
    Crowded,
    Right,
    Wrong(String),
}

/// **The cursor on `edge`'s midpoint, from an eye along the unit `u`
/// at `distance`, `off` from the middle of the pane**, picks `edge`.
/// At a silhouette the edge is caught from its inward side only (the
/// door's one-sided band, `PickIndex::edge_at_for`), so there the
/// cursor sits half a pixel onto the face in view.
#[allow(clippy::too_many_arguments)]
fn hover(
    index: &PickIndex,
    eval: &Evaluation<f64>,
    display: &DisplayView,
    edges: &[DrawnEdge],
    edge: &DrawnEdge,
    u: Vec3<f64>,
    distance: f64,
    off: Vec3<f64>,
) -> Hover {
    let at = middle(&edge.points);
    let [n1, n2] = edge.normals;
    let (yaw, pitch) = yaw_pitch(u);
    if pitch.abs() >= Camera::pitch_limit() {
        return Hover::Unasked;
    }
    let camera = Camera::new(at + off, distance, yaw, pitch, 0.8, 1.0).expect("a camera");
    let to_eye = unit(camera.eye() - at);
    let facing = [n1.dot(to_eye), n2.dot(to_eye)];
    if facing[0].max(facing[1]) <= 1.0e-6 {
        // Both faces turned away: the box hides this edge.
        return Hover::Unasked;
    }
    let Some(on_edge) = pixel_of(&camera, at) else {
        return Hover::Unasked;
    };
    let cursor = if facing[0] > 0.0 && facing[1] > 0.0 {
        on_edge
    } else {
        // The face in view runs from the edge away from the other
        // face's normal.
        let away = if facing[0] > 0.0 { n2 } else { n1 };
        let Some(inside) = pixel_of(&camera, at + away * (-1.0e-3 * distance)) else {
            return Hover::Unasked;
        };
        let (dx, dy) = (inside[0] - on_edge[0], inside[1] - on_edge[1]);
        let len = dx.hypot(dy);
        if len.is_nan() || len <= 1.0e-9 {
            return Hover::Unasked;
        }
        [on_edge[0] + 0.5 * dx / len, on_edge[1] + 0.5 * dy / len]
    };
    if !(0.0..pane().width_px).contains(&cursor[0]) || !(0.0..pane().height_px).contains(&cursor[1])
    {
        return Hover::Unasked;
    }
    let crowded = edges
        .iter()
        .filter(|other| other.id != edge.id)
        .any(|other| {
            let pixels: Option<Vec<[f64; 2]>> =
                other.points.iter().map(|&p| pixel_of(&camera, p)).collect();
            pixels.is_none_or(|pixels| {
                pixels
                    .windows(2)
                    .any(|pair| to_segment(cursor, pair[0], pair[1]) < 1.0)
            })
        });
    if crowded {
        return Hover::Crowded;
    }
    match index.edge_at_for(eval, &camera, pane(), cursor, display) {
        Ok(Some(pick)) if pick.id() == edge.id => Hover::Right,
        answer => Hover::Wrong(format!(
            "edge {:?} at {at:?}, eye along {u:?} at {distance} off {off:?}, \
             facing {:e} / {:e}: {answer:?}",
            edge.id, facing[0], facing[1],
        )),
    }
}

/// **An edge of a moved copy is picked where it is drawn, and is not
/// hidden by its own face.**
///
/// Each draw hovers the midpoint of one of the box's twelve edges
/// ([`hover`]) under a random display ([`random_display`]), from an eye
/// at a random distance and direction — half the draws biased to graze
/// one of the edge's two faces, from a tenth of a radian down to a
/// hundred-thousandth, where a shift of the probe ray moves that face's
/// crossing furthest along the ray. The right answer is that edge.
#[test]
fn a_moved_copys_edge_is_not_hidden_by_its_own_face() {
    let tol = Tol::witness();
    let mut rng = fuzz::start("moved-copy edge self-occlusion");
    let (mut asked, mut crowded) = (0usize, 0usize);
    let mut failures: Vec<String> = Vec::new();
    // One copy at the identity, and one posed: a posed copy is carried
    // by its body frame even when nothing moves it.
    let reach = rng.log_uniform(1.0, farthest_pose());
    let posed = random_rigid(&mut rng, reach);
    for pose in [Frame::IDENTITY, posed] {
        let (session, copies) = boxes_posed(&[pose], tol);
        let [copy] = copies[..] else { unreachable!() };
        let index = drawn_index(&session);
        let eval = eval_of(&session);
        for _ in 0..fuzz::scaled(150) {
            // The move against the body (one unit), across scales.
            let scale = rng.log_uniform(1.0e-3, 1.0e6);
            let (display, moved) = random_display(&mut rng, copy, &pose, scale);
            let edges = drawn_edges(&index, copy, &display);
            let edge = &edges[rng.below(edges.len())];
            let [n1, n2] = edge.normals;
            let along = unit(edge.points[edge.points.len() - 1] - edge.points[0]);
            let u = if rng.below(2) == 0 {
                // Graze one face, the other in view.
                let (graze, open) = if rng.below(2) == 0 {
                    (n1, n2)
                } else {
                    (n2, n1)
                };
                let sign = if rng.below(2) == 0 { 1.0 } else { -1.0 };
                let beta = rng.log_uniform(1.0e-5, 1.0e-1) * sign;
                let gamma = rng.range(-1.3, 1.3);
                unit(graze * beta.sin() + (open * gamma.cos() + along * gamma.sin()) * beta.cos())
            } else {
                random_unit(&mut rng)
            };
            let distance = rng.log_uniform(0.5, 50.0);
            let off = random_unit(&mut rng) * (distance * rng.range(0.0, 0.2));
            match hover(&index, eval, &display, &edges, edge, u, distance, off) {
                Hover::Unasked => {}
                Hover::Crowded => crowded += 1,
                Hover::Right => asked += 1,
                Hover::Wrong(what) => {
                    asked += 1;
                    failures.push(format!("{moved}, pose {pose:?}: {what}"));
                }
            }
        }
    }
    println!(
        "[moved-copy] edges asked: {asked}, crowded: {crowded}, wrong: {}",
        failures.len()
    );
    assert!(
        failures.is_empty(),
        "{} of {asked} hovered edges of a drawn copy did not pick ({}); first: {:#?}",
        failures.len(),
        fuzz::replay(),
        &failures[..failures.len().min(4)]
    );
}

/// **A copy posed far off and moved back near the origin picks every
/// edge it shows**, as a copy posed straight there does — a pinned
/// counterexample of the search above. The move's translation cancels
/// the pose's, so the probe ray carried back through both keeps none
/// of its origin's bits below that coordinate's rounding, far coarser
/// than the width a crossing near the origin certifies, and an edge
/// read in the world against its face read on the carried ray fell
/// behind it. Every edge, from a fixed spread of eye directions at two
/// distances.
#[test]
fn a_copy_moved_back_from_far_off_picks_every_edge_it_shows() {
    let tol = Tol::witness();
    let pose = rigid(unit(Vec3::new(1.0, 2.0, 3.0)), 0.7, [614.8, -101.6, 84.5]);
    let near = rigid(unit(Vec3::new(0.0, 0.0, 1.0)), 0.3, [0.2, 0.1, 0.0]);
    let back = Frame::from_affine(near.affine() * pose.affine().inverse());
    // A Fibonacci spread of eye directions.
    let golden = std::f64::consts::PI * (3.0 - 5f64.sqrt());
    let eyes: Vec<Vec3<f64>> = (0..32)
        .map(|i| {
            let z = 1.0 - (f64::from(i) + 0.5) / 16.0;
            let r = (1.0 - z * z).sqrt();
            let a = golden * f64::from(i);
            Vec3::new(r * a.cos(), r * a.sin(), z)
        })
        .collect();
    // The copy posed where the moved one is drawn, then the moved one.
    for (pose, moved) in [(near, None), (pose, Some(back))] {
        let (session, copies) = boxes_posed(&[pose], tol);
        let [copy] = copies[..] else { unreachable!() };
        let index = drawn_index(&session);
        let eval = eval_of(&session);
        let display = DisplayView {
            moved_nodes: moved.map(|back| (copy, back)).into_iter().collect(),
            ..DisplayView::none()
        };
        let edges = drawn_edges(&index, copy, &display);
        let (mut asked, mut failures) = (0usize, Vec::new());
        for edge in &edges {
            for &u in &eyes {
                for distance in [0.9, 6.7] {
                    let off = Vec3::new(0.0, 0.0, 0.0);
                    match hover(&index, eval, &display, &edges, edge, u, distance, off) {
                        Hover::Unasked | Hover::Crowded => {}
                        Hover::Right => asked += 1,
                        Hover::Wrong(what) => {
                            asked += 1;
                            failures.push(what);
                        }
                    }
                }
            }
        }
        let what = if moved.is_some() {
            "moved back"
        } else {
            "posed there"
        };
        assert!(
            asked >= 200,
            "{what}: the spread asks most of its hovers: {asked}"
        );
        assert!(
            failures.is_empty(),
            "{what}: {} of {asked} edges did not pick; first: {:#?}",
            failures.len(),
            &failures[..failures.len().min(4)]
        );
    }
}
