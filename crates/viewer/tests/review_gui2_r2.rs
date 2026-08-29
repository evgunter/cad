//! **R2's independent consumer suite for GUI-2** (viewport selection).
//!
//! An independent derivation of what the unit claims, not a re-reading
//! of `select_pick.rs`: different documents (a stepped block, two
//! transforms of one extrude, a split), different aspect ratios,
//! different cursors, and a hand-written evaluation seam that holds a
//! run outstanding so the landed (doc, eval) PAIR can be observed while
//! the shown document is ahead of it.
//!
//! Fixtures are authored here on purpose — `tests/common` is derived
//! from `viewer::scene`'s own constants, and a review suite that read
//! them would be checking the implementation against itself
//! (`memories/review-and-dependency-policy.md`).
//!
//! Rows marked **EVIDENCE** assert nothing about the subject and exist
//! to print what the review measured; they are not gates
//! (`memories/test-suite-cost.md`) and should be dropped or given
//! assertions if they survive a fix pass.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::sync::{Arc, Mutex};

use pncad::document::{
    Dimension, Doc, DocEdit, Evaluation, Expr, LoopProgram, Node, PatternKind, ProfileProgram,
    RecipeNodeId, apply,
};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::prelude::StableName;
use pncad::profile::SketchPlane;
use pncad::select::{Ray, Resolution};
use viewer::camera::Camera;
use viewer::evalseam::{EvalDone, EvalRequest, EvalService, InlineEvaluator};
use viewer::input::{InputMap, PickAction, PointerButton, ViewportEvent, ViewportSize};
use viewer::pick::{IdMap, PatchId, PickIndex};
use viewer::scene::DisplayTolerance;
use viewer::session::{DocSession, FaceSelection, Selection, SessionOp};
use viewer::{cursor_projection, pick};

// -------------------------------------------------------------------
// Fixtures, authored through the ordinary edit doors.
// -------------------------------------------------------------------

fn tol() -> Tol {
    Tol::witness()
}

fn delta() -> DisplayTolerance {
    DisplayTolerance::new(3.0e-4).expect("a positive delta")
}

fn scalar(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("a finite scalar")
}

fn length(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a finite length")
}

fn insert(
    doc: &Doc<ProfileProgram>,
    node: Node<ProfileProgram>,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol()).expect("the insert applies");
    let id = applied.record.minted.expect("an insert mints an id");
    (applied.doc, id)
}

/// A rectangle profile in the XY plane, `w` by `h`, at the origin.
fn rectangle(w: f64, h: f64) -> Node<ProfileProgram> {
    Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)]).expect("finite corners"),
        ],
    })
}

fn translated(input: RecipeNodeId, dx: f64, dy: f64, dz: f64) -> Node<ProfileProgram> {
    Node::Transform {
        input,
        translation: [length(dx), length(dy), length(dz)],
        rotation_axis: [scalar(0.0), scalar(0.0), scalar(1.0)],
        rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite"),
    }
}

/// One extruded slab, `w` × `h` × `t`, its own document. Deliberately
/// NOT the shipped plate: a different body, so the un-projection rows
/// below are not tuned to the fixture the unit was written against.
fn slab(w: f64, h: f64, t: f64, label: &str) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived(label, tol());
    let (doc, profile) = insert(&doc, rectangle(w, h));
    let (doc, extrude) = insert(
        &doc,
        Node::Extrude {
            profile,
            distance: length(t),
        },
    );
    (doc, extrude)
}

/// A slab plus TWO transforms of it, both of which are product roots.
///
/// Legal by the root invariants (coverage plus ancestor-freedom: the
/// extrude is an ancestor of both sinks and is itself no root), and it
/// is the shape that makes one `StableName` drawn twice — `Transform`
/// is a pass-through that contributes no role segment, so both copies
/// carry the extrude's names.
fn two_placements() -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let (doc, extrude) = slab(0.03, 0.02, 0.01, "r2-two-placements");
    let (doc, left) = insert(&doc, translated(extrude, 0.0, 0.0, 0.0));
    let (doc, right) = insert(&doc, translated(extrude, 0.10, 0.0, 0.0));
    (doc, left, right)
}

/// A linear pattern of `count` small blocks — several bodies under one
/// node, and a structural slot that can consume one of them.
fn pattern_of(count: i64) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let (doc, extrude) = slab(0.015, 0.015, 0.010, "r2-pattern");
    let (doc, pattern) = insert(
        &doc,
        Node::Pattern {
            input: extrude,
            count: Expr::count(count),
            kind: PatternKind::Linear {
                direction: [scalar(1.0), scalar(0.0), scalar(0.0)],
                spacing: length(0.04),
            },
        },
    );
    (doc, pattern)
}

fn index_at(session: &DocSession, d: DisplayTolerance) -> PickIndex {
    let (doc, eval) = session.landed_pair().expect("an evaluation has landed");
    let generation = session
        .landed_generation()
        .expect("a landed evaluation has a generation");
    PickIndex::build(doc, eval, generation, d, session.tol()).expect("the fixture indexes")
}

fn landed_index(session: &DocSession) -> PickIndex {
    index_at(session, delta())
}

/// A δ coarse enough that the gallery ring tessellates cheaply — the
/// e2e row's subject is the selection walk, not the facet count.
fn coarse() -> DisplayTolerance {
    DisplayTolerance::new(2.0e-3).expect("a positive delta")
}

fn evaluation(session: &DocSession) -> &Evaluation<f64> {
    session.evaluation().expect("an evaluation has landed")
}

/// A ray straight down the −z axis through `(x, y)`, starting above
/// anything these fixtures build.
fn down_at(x: f64, y: f64) -> Ray {
    Ray {
        origin: Point3::new(x, y, 5.0),
        dir: Vec3::new(0.0, 0.0, -1.0),
    }
}

/// A camera looking at a box, at `aspect`.
fn camera_on(doc_bounds: &bvh::Aabb, aspect: f64) -> Camera {
    Camera::framing(doc_bounds, aspect).expect("a non-degenerate box frames")
}

fn box_of(min: [f64; 3], max: [f64; 3]) -> bvh::Aabb {
    bvh::Aabb {
        min_x: min[0],
        min_y: min[1],
        min_z: min[2],
        max_x: max[0],
        max_y: max[1],
        max_z: max[2],
    }
}

/// The pixel a world point projects to, `+y` down, in `viewport`.
fn cursor_of(camera: &Camera, point: Point3<f64>, viewport: ViewportSize) -> [f64; 2] {
    let aspect = viewport.aspect().expect("a positive aspect");
    let ndc = camera
        .project(point, aspect)
        .expect("the projection is defined")
        .expect("the point is in front of the eye");
    [
        (ndc[0] + 1.0) * 0.5 * viewport.width_px,
        (1.0 - ndc[1]) * 0.5 * viewport.height_px,
    ]
}

// -------------------------------------------------------------------
// 1. The un-projection (claim: `ray_through` inverts `project`)
// -------------------------------------------------------------------

/// Every corner AND every face centre of two different boxes, at four
/// aspect ratios including two the unit never used (a tall pane and a
/// square one), un-projects onto the ray through its own pixel.
///
/// This is `select_pick`'s residual property re-derived on other
/// geometry: the residual is measured perpendicular to the ray and
/// scaled by the distance travelled, so it is a relative bound rather
/// than an absolute one that a small fixture would pass for free.
#[test]
fn every_projected_point_lies_on_the_ray_through_its_own_pixel() {
    let boxes = [
        box_of([0.0, 0.0, 0.0], [0.06, 0.04, 0.008]),
        box_of([-1.5, 2.0, -0.25], [3.5, 2.5, 4.0]),
    ];
    let panes = [
        ViewportSize {
            width_px: 1920.0,
            height_px: 1080.0,
        },
        ViewportSize {
            width_px: 600.0,
            height_px: 900.0,
        },
        ViewportSize {
            width_px: 512.0,
            height_px: 512.0,
        },
        ViewportSize {
            width_px: 2560.0,
            height_px: 720.0,
        },
    ];
    let mut checked = 0usize;
    for b in &boxes {
        for pane in panes {
            let aspect = pane.aspect().expect("a positive aspect");
            let camera = camera_on(b, aspect);
            let mut points: Vec<Point3<f64>> = Vec::new();
            for x in [b.min_x, b.max_x] {
                for y in [b.min_y, b.max_y] {
                    for z in [b.min_z, b.max_z] {
                        points.push(Point3::new(x, y, z));
                    }
                }
            }
            let mid = |lo: f64, hi: f64| 0.5 * (lo + hi);
            points.push(Point3::new(
                mid(b.min_x, b.max_x),
                mid(b.min_y, b.max_y),
                b.max_z,
            ));
            points.push(Point3::new(b.min_x, mid(b.min_y, b.max_y), b.min_z));
            for point in points {
                let cursor = cursor_of(&camera, point, pane);
                let ray = camera
                    .ray_through(cursor, pane)
                    .expect("a finite cursor un-projects");
                let to_point = point - ray.origin;
                let along = ray.dir.dot(to_point);
                assert!(along > 0.0, "the point is in front of the eye at {pane:?}");
                let perpendicular = to_point - ray.dir * along;
                let residual = perpendicular.dot(perpendicular).sqrt();
                assert!(
                    residual < 1.0e-12 * along,
                    "residual {residual} at distance {along}, pane {pane:?}"
                );
                checked += 1;
            }
        }
    }
    assert!(
        checked >= 80,
        "the sweep really visited its points: {checked}"
    );
}

/// Un-project then re-project: a cursor anywhere in the pane names a
/// ray whose points project back to that same cursor.
///
/// The direction `select_pick` does not take, and the one that catches
/// an aspect applied to the wrong axis: the forward row starts from a
/// point that is already inside the frustum, so a swapped aspect can
/// still land it on its own ray, while this one pins the whole
/// screen→world→screen loop at cursors chosen independently of any
/// geometry.
#[test]
fn a_cursor_survives_un_projection_and_re_projection() {
    let pane = ViewportSize {
        width_px: 1440.0,
        height_px: 900.0,
    };
    let aspect = pane.aspect().expect("a positive aspect");
    let camera = camera_on(&box_of([0.0, 0.0, 0.0], [0.06, 0.04, 0.008]), aspect);
    let mut rng = test_utils::fuzz::start("gui2-r2-cursor-roundtrip");
    for _ in 0..test_utils::fuzz::scaled(64) {
        let cursor = [
            rng.range(0.0, pane.width_px),
            rng.range(0.0, pane.height_px),
        ];
        let ray = camera
            .ray_through(cursor, pane)
            .expect("a cursor un-projects");
        for t in [0.05, 0.3, 1.0] {
            let point = ray.origin + ray.dir * t;
            let back = cursor_of(&camera, point, pane);
            let dx = (back[0] - cursor[0]).abs();
            let dy = (back[1] - cursor[1]).abs();
            assert!(
                dx < 1.0e-8 && dy < 1.0e-8,
                "cursor {cursor:?} came back as {back:?} at t={t} ({})",
                test_utils::fuzz::replay()
            );
        }
    }
}

/// The pane centre is the view axis, and the four edge midpoints are
/// symmetric about it — the framing check the unit takes only at the
/// centre.
#[test]
fn the_centre_is_the_view_axis_and_the_edges_are_symmetric() {
    let pane = ViewportSize {
        width_px: 1000.0,
        height_px: 400.0,
    };
    let aspect = pane.aspect().expect("a positive aspect");
    let camera = camera_on(&box_of([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]), aspect);
    let axis = camera
        .ray_through([500.0, 200.0], pane)
        .expect("the centre un-projects");
    let forward = camera.forward();
    for (got, want) in [
        (axis.dir.x, forward.x),
        (axis.dir.y, forward.y),
        (axis.dir.z, forward.z),
    ] {
        assert!((got - want).abs() < 1.0e-12, "{got} vs {want}");
    }
    // Left and right of centre lean by equal and opposite amounts
    // about the view axis.
    let left = camera
        .ray_through([250.0, 200.0], pane)
        .expect("un-projects");
    let right = camera
        .ray_through([750.0, 200.0], pane)
        .expect("un-projects");
    let lean = |r: &Ray| r.dir.dot(camera.right());
    assert!(
        (lean(&left) + lean(&right)).abs() < 1.0e-12,
        "left {} and right {} are not symmetric",
        lean(&left),
        lean(&right)
    );
    assert!(lean(&right) > 0.0, "the right half of the pane leans right");
    // Up and down likewise, about the camera's up axis, and `+y` in
    // pixels is DOWN.
    let top = camera
        .ray_through([500.0, 100.0], pane)
        .expect("un-projects");
    let bottom = camera
        .ray_through([500.0, 300.0], pane)
        .expect("un-projects");
    assert!(
        top.dir.dot(camera.up()) > 0.0,
        "a cursor near the top of the pane looks UP"
    );
    assert!(
        (top.dir.dot(camera.up()) + bottom.dir.dot(camera.up())).abs() < 1.0e-12,
        "top and bottom are not symmetric"
    );
}

/// Every non-finite input and every area-less pane is refused, one
/// coordinate at a time — the unit's row checks two of the eight.
#[test]
fn each_non_finite_input_and_each_area_less_pane_is_refused() {
    let camera = camera_on(&box_of([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]), 1.0);
    let good = ViewportSize {
        width_px: 800.0,
        height_px: 600.0,
    };
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            camera.ray_through([bad, 1.0], good).is_err(),
            "cursor x {bad}"
        );
        assert!(
            camera.ray_through([1.0, bad], good).is_err(),
            "cursor y {bad}"
        );
        assert!(
            camera
                .ray_through(
                    [1.0, 1.0],
                    ViewportSize {
                        width_px: bad,
                        height_px: 600.0
                    }
                )
                .is_err(),
            "viewport width {bad}"
        );
        assert!(
            camera
                .ray_through(
                    [1.0, 1.0],
                    ViewportSize {
                        width_px: 800.0,
                        height_px: bad
                    }
                )
                .is_err(),
            "viewport height {bad}"
        );
    }
    for pane in [
        ViewportSize {
            width_px: 0.0,
            height_px: 600.0,
        },
        ViewportSize {
            width_px: 800.0,
            height_px: 0.0,
        },
        ViewportSize {
            width_px: 0.0,
            height_px: 0.0,
        },
        ViewportSize {
            width_px: -800.0,
            height_px: 600.0,
        },
    ] {
        assert!(
            camera.ray_through([1.0, 1.0], pane).is_err(),
            "pane {pane:?}"
        );
    }
}

// -------------------------------------------------------------------
// 2. `cursor_projection` — the transform the GPU pass samples through
// -------------------------------------------------------------------

/// The transform maps the cursor's OWN pixel onto the 1×1 target's
/// full `[−1, 1]²`, so a world point one pixel away lands on the
/// target's edge and one two pixels away lands outside it.
///
/// **The property the unit's row cannot see.** That row asserts only
/// that the hit point lands inside the target — but the hit point is
/// the cursor's own point, which sits at the transform's fixed point,
/// where every scale and every sign is invisible. Planting `* -sx` or
/// `* (sy * 100.0)` in `cursor_projection` leaves the whole shipped
/// suite green; both redden this row, which is what makes the sign and
/// scale half of issue #1097's checklist a headless question after all.
#[test]
fn the_sampled_pixel_is_one_pixel_wide_and_correctly_oriented() {
    let pane = ViewportSize {
        width_px: 1024.0,
        height_px: 768.0,
    };
    let aspect = pane.aspect().expect("a positive aspect");
    let camera = camera_on(&box_of([0.0, 0.0, 0.0], [0.06, 0.04, 0.008]), aspect);
    let matrix = camera.view_projection(aspect).expect("defined");
    let vp = matrix.map(|column| column.map(|v| v as f32));
    let centre = [pane.width_px * 0.5, pane.height_px * 0.5];
    let ndc = |c: [f64; 2]| {
        [
            (2.0 * c[0] / pane.width_px - 1.0) as f32,
            (1.0 - 2.0 * c[1] / pane.height_px) as f32,
        ]
    };
    let sampled = cursor_projection(
        &vp,
        ndc(centre),
        [pane.width_px as f32, pane.height_px as f32],
    );
    // A world point on the ray through a cursor `n` pixels to the
    // right of the sampled one must land at target-x ≈ `n` — one at
    // the edge, two outside.
    let at = |dx: f64, dy: f64| {
        let ray = camera
            .ray_through([centre[0] + dx, centre[1] + dy], pane)
            .expect("un-projects");
        let p = ray.origin + ray.dir * 0.2;
        let v = [p.x as f32, p.y as f32, p.z as f32, 1.0];
        let mut out = [0.0f32; 4];
        for (row, slot) in out.iter_mut().enumerate() {
            *slot = sampled[0][row] * v[0]
                + sampled[1][row] * v[1]
                + sampled[2][row] * v[2]
                + sampled[3][row] * v[3];
        }
        [out[0] / out[3], out[1] / out[3]]
    };
    let origin = at(0.0, 0.0);
    assert!(
        origin[0].abs() < 1.0e-3 && origin[1].abs() < 1.0e-3,
        "the cursor's own pixel is the target's centre: {origin:?}"
    );
    // The sampled square is ONE pixel across and centred on the
    // cursor, so its right edge is half a pixel out.
    let edge = at(0.5, 0.0);
    assert!(
        (edge[0] - 1.0).abs() < 1.0e-2,
        "half a pixel right is the target's right edge, got {}",
        edge[0]
    );
    let outside = at(1.0, 0.0);
    assert!(
        outside[0] > 1.5,
        "a whole pixel right is outside the target: {}",
        outside[0]
    );
    // Screen `+y` is DOWN and clip `+y` is UP, so a cursor half a pixel
    // BELOW the sampled one lands at target-y ≈ −1. A sign error here
    // is one of the three failure modes #1097 lists.
    let below = at(0.0, 0.5);
    assert!(
        (below[1] + 1.0).abs() < 1.0e-2,
        "half a pixel down is the target's bottom edge, got {}",
        below[1]
    );
}

// -------------------------------------------------------------------
// 3. The id map
// -------------------------------------------------------------------

/// `IdMap` is a bijection on keys this suite makes up, independent of
/// any tessellation: every id inverts, every key maps forward, NOTHING
/// is reserved in both directions, and an id past the end is `None`.
#[test]
fn the_id_map_is_a_bijection_over_keys_of_its_own() {
    let keys: Vec<PatchId> = (0..3)
        .flat_map(|node| {
            (0..2).flat_map(move |body| {
                (0..4).map(move |patch| PatchId {
                    node: RecipeNodeId(node),
                    body,
                    patch,
                })
            })
        })
        .collect();
    let map = IdMap::build(keys.clone()).expect("distinct keys assign");
    assert_eq!(map.len(), keys.len());
    assert!(!map.is_empty());
    assert_eq!(map.ids().count(), keys.len());
    for key in &keys {
        let id = map.id_of(*key).expect("every offered key has an id");
        assert_ne!(id, IdMap::NOTHING, "no key is assigned the miss value");
        assert_eq!(map.key_of(id), Some(*key), "id {id} does not invert");
    }
    for id in map.ids() {
        let key = map.key_of(id).expect("an assigned id names a key");
        assert_eq!(map.id_of(key), Some(id), "key {key:?} does not round-trip");
    }
    assert_eq!(map.key_of(IdMap::NOTHING), None, "NOTHING names no patch");
    let past_the_end = u32::try_from(keys.len()).expect("small") + 1;
    assert_eq!(
        map.key_of(past_the_end),
        None,
        "an unassigned id names nothing"
    );
    assert_eq!(map.key_of(u32::MAX), None);
    // The empty map is the degenerate case, and it is not a panic.
    let empty = IdMap::build(Vec::new()).expect("no keys is a map");
    assert!(empty.is_empty());
    assert_eq!(empty.key_of(1), None);
}

/// Two keys differing in ONE field each — node, body, or patch — get
/// different ids. A collision here is the silent wrong-face pick the
/// mapping exists to make impossible, and a map that keyed on only two
/// of the three fields would still pass a round-trip check.
#[test]
fn keys_differing_in_any_single_field_never_share_an_id() {
    let base = PatchId {
        node: RecipeNodeId(4),
        body: 1,
        patch: 2,
    };
    let neighbours = [
        PatchId {
            node: RecipeNodeId(5),
            ..base
        },
        PatchId { body: 0, ..base },
        PatchId { patch: 3, ..base },
    ];
    let mut all = vec![base];
    all.extend(neighbours);
    let map = IdMap::build(all.clone()).expect("distinct keys assign");
    let ids: Vec<u32> = all
        .iter()
        .map(|k| map.id_of(*k).expect("assigned"))
        .collect();
    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            assert_ne!(ids[i], ids[j], "{:?} and {:?} share an id", all[i], all[j]);
        }
    }
    // And a repeat is refused rather than quietly folded together.
    let mut repeated = all;
    repeated.push(base);
    assert!(IdMap::build(repeated).is_err(), "a repeated key is refused");
}

/// A three-instance pattern: patch 0 of each instance body is a
/// distinct id, the map is stable across two builds of one generation,
/// and every drawn id inverts to a patch of a body the document really
/// has.
#[test]
fn a_three_instance_pattern_gives_every_instance_its_own_ids() {
    let (doc, pattern) = pattern_of(3);
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    let first_patch_ids: Vec<u32> = (0..3)
        .map(|body| {
            index
                .ids()
                .id_of(PatchId {
                    node: pattern,
                    body,
                    patch: 0,
                })
                .unwrap_or_else(|| panic!("instance {body} draws a patch 0"))
        })
        .collect();
    assert_eq!(
        first_patch_ids
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        3,
        "three instances, three ids for patch 0: {first_patch_ids:?}"
    );
    // Re-indexing the same generation at the same δ is the same map.
    let again = landed_index(&session);
    assert_eq!(index.ids(), again.ids(), "re-tessellation moved the ids");
    // Every id belongs to a body the pattern actually has.
    for id in index.ids().ids() {
        let key = index.ids().key_of(id).expect("assigned");
        assert_eq!(key.node, pattern, "the pattern is the only root");
        assert!(key.body < 3, "body {} is outside the pattern", key.body);
    }
}

// -------------------------------------------------------------------
// 4. The ray path, and the two paths agreeing
// -------------------------------------------------------------------

/// A ray down onto a slab names its top face; the name resolves in the
/// landed run; the id map inverts that id back to the same name AND to
/// the same patch the ray hit.
///
/// The patch half is what makes this an inverse claim rather than a
/// name-only one: the shipped agreement row asserts node, body and
/// name but never `patch`, so a key list built with a shifted patch
/// index passes it.
#[test]
fn the_ray_path_and_the_id_map_invert_each_other_patch_included() {
    let (doc, extrude) = slab(0.03, 0.02, 0.01, "r2-ray-inverse");
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    let hit = index
        .pick(evaluation(&session), &down_at(0.015, 0.010))
        .expect("no refusal")
        .expect("a ray down onto the slab hits it");
    assert_eq!(hit.node, extrude, "the only root is the extrude");
    let ids = index.ids_of(&hit.name);
    assert_eq!(ids.len(), 1, "one root, one body: the name is drawn once");
    let id = ids[0];
    let key = index.ids().key_of(id).expect("a drawn id names a patch");
    assert_eq!(key.node, hit.node);
    assert_eq!(key.body, hit.body);
    // The patch the id names is a patch of the mesh that was picked
    // against, and its own name is the ray's answer.
    let part = index
        .parts()
        .iter()
        .find(|p| p.node() == key.node && p.body() == key.body)
        .expect("the hit body is a drawn part");
    assert!(
        key.patch < part.mesh().patches.len(),
        "patch {} is outside the drawn mesh ({} patches)",
        key.patch,
        part.mesh().patches.len()
    );
    let by_patch = part.patch_names(evaluation(&session));
    assert_eq!(
        by_patch[key.patch].as_ref().expect("the patch is named"),
        &hit.name,
        "the id's patch is not the patch the ray hit"
    );
}

/// A sweep of cursors over the visible face: every cursor that hits
/// answers a name that is drawn under some id, and every such id
/// inverts to a patch whose own name is that answer.
///
/// A counterexample search (`memories/test-suite-cost`'s first shape):
/// the seed varies, the count rides the EFFORT dial, and cutting it
/// loses detection power rather than correctness. The anti-vacuity
/// witness is NOT drawn from the same sample — it is a static list of
/// cursors projected from points known to be on the body, so the two
/// obligations do not share one count.
#[test]
fn cursors_across_the_pane_never_disagree_with_the_id_map() {
    let (doc, _) = slab(0.03, 0.02, 0.01, "r2-cursor-sweep");
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    let pane = ViewportSize {
        width_px: 900.0,
        height_px: 700.0,
    };
    let aspect = pane.aspect().expect("a positive aspect");
    let camera = camera_on(&box_of([0.0, 0.0, 0.0], [0.03, 0.02, 0.01]), aspect);
    let mut rng = test_utils::fuzz::start("gui2-r2-cursor-sweep");
    let n = test_utils::fuzz::scaled(120);
    // The static witness: cursors projected from points on the slab's
    // top face, which hit by construction on every run.
    let witness: Vec<[f64; 2]> = [
        Point3::new(0.005, 0.005, 0.01),
        Point3::new(0.015, 0.010, 0.01),
        Point3::new(0.025, 0.015, 0.01),
    ]
    .into_iter()
    .map(|p| cursor_of(&camera, p, pane))
    .collect();
    let mut hits = 0usize;
    let random = (0..n).map(|_| {
        [
            rng.range(0.0, pane.width_px),
            rng.range(0.0, pane.height_px),
        ]
    });
    for cursor in witness.iter().copied().chain(random) {
        let ray = camera.ray_through(cursor, pane).expect("un-projects");
        let Some(hit) = index.pick(evaluation(&session), &ray).expect("no refusal") else {
            continue;
        };
        hits += 1;
        let ids = index.ids_of(&hit.name);
        assert!(
            !ids.is_empty(),
            "the ray answered a name nothing is drawn under, at {cursor:?} ({})",
            test_utils::fuzz::replay()
        );
        for id in ids {
            let name = index
                .name_of(*id)
                .expect("a drawn id has a name slot")
                .as_ref()
                .expect("the patch is named");
            assert_eq!(
                name,
                &hit.name,
                "id {id} inverts to another name, at {cursor:?} ({})",
                test_utils::fuzz::replay()
            );
        }
    }
    assert!(
        hits >= witness.len(),
        "the static witness cursors did not all hit: {hits} ({})",
        test_utils::fuzz::replay()
    );
}

// -------------------------------------------------------------------
// 5. One name drawn twice — the highlight's disambiguation
// -------------------------------------------------------------------

/// Two `Transform` roots over one extrude draw the SAME stable names
/// twice, which is legal: the root set is the DAG's sink set and the
/// shared extrude is an ancestor of both sinks, not a root.
///
/// EVIDENCE for the row below — it reports the shape rather than
/// gating it, and would be dropped if the highlight row it justifies
/// ever stops depending on the shape being reachable.
#[test]
fn one_name_can_be_drawn_under_two_ids() {
    let (doc, left, right) = two_placements();
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    let mut shared: Vec<(StableName, Vec<u32>)> = Vec::new();
    for id in index.ids().ids() {
        if let Some(Ok(name)) = index.name_of(id) {
            let ids = index.ids_of(name);
            if ids.len() > 1 && !shared.iter().any(|(n, _)| n == name) {
                shared.push((name.clone(), ids.to_vec()));
            }
        }
    }
    println!(
        "EVIDENCE two_placements: roots {left:?}/{right:?}, {} names drawn more than once \
         out of {} ids",
        shared.len(),
        index.ids().len()
    );
    assert!(
        !shared.is_empty(),
        "two transforms of one extrude were expected to share names"
    );
}

/// **The highlight must mark the patch that was picked.** A face on
/// the second placement is selected; the id the highlight lights must
/// be an id of THAT node and body, not of the other placement that
/// happens to carry the same stable name.
///
/// The selection value carries `node` and `body` precisely so this
/// question has an answer; `pick::highlight` reads only `name`.
///
/// **Was RED against the reviewed head — MAJOR-1, written as the gate
/// it should become.** The fix pass made `pick::highlight` narrow by
/// `(node, body)` through `PickIndex::ids_of_target`, so the attribute
/// is gone and this row gates.
#[test]
fn the_highlight_marks_the_selected_bodys_patch_not_another_with_the_same_name() {
    let (doc, _left, right) = two_placements();
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    // A ray onto the RIGHT placement's top face (it sits at x ≈ 0.10).
    let hit = index
        .pick(evaluation(&session), &down_at(0.115, 0.010))
        .expect("no refusal")
        .expect("the right placement is hit");
    assert_eq!(hit.node, right, "the ray really hit the right placement");
    let selection = Selection::Face(FaceSelection {
        name: hit.name.clone(),
        node: hit.node,
        body: hit.body,
    });
    let marked = pick::highlight(&index, &selection, None);
    assert_ne!(marked.selected, IdMap::NOTHING, "something is marked");
    let key = index
        .ids()
        .key_of(marked.selected)
        .expect("the marked id names a patch");
    assert_eq!(
        (key.node, key.body),
        (hit.node, hit.body),
        "the highlight marked {key:?}, which is not the picked body"
    );
}

// -------------------------------------------------------------------
// 6. The op vocabulary, driven as a consumer
// -------------------------------------------------------------------

/// The whole cursor path as a consumer drives it: events in, ops out,
/// ops performed, selection and hover read back. Hover never touches
/// the selection; a second pick replaces the first; a click on empty
/// space clears it; leaving clears only the hover.
#[test]
fn the_event_stream_drives_selection_and_hover_through_typed_ops() {
    let (doc, extrude) = slab(0.03, 0.02, 0.01, "r2-op-vocabulary");
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    let pane = ViewportSize {
        width_px: 800.0,
        height_px: 600.0,
    };
    let aspect = pane.aspect().expect("a positive aspect");
    let camera = camera_on(&box_of([0.0, 0.0, 0.0], [0.03, 0.02, 0.01]), aspect);
    let map = InputMap::default();

    // A cursor certainly on the body, and one certainly off it.
    let on = cursor_of(&camera, Point3::new(0.015, 0.010, 0.01), pane);
    let off = [2.0, 2.0];

    let stream = [
        ViewportEvent::Hover { pos_px: on },
        ViewportEvent::Click {
            button: PointerButton::Primary,
            pos_px: on,
        },
    ];
    for action in viewer::input::pick_stream(&map, &stream) {
        let op = index
            .op_for(evaluation(&session), &camera, pane, action)
            .expect("the cursor un-projects and the pick answers");
        session.perform(op);
    }
    let first = session
        .selection()
        .face()
        .expect("a face is selected")
        .clone();
    assert_eq!(first.node, extrude);
    assert_eq!(
        session.hover().expect("the hover landed too").name,
        first.name
    );
    assert_eq!(session.selection().node(), Some(extrude));

    // Hovering elsewhere moves the hover and leaves the selection.
    let op = index
        .op_for(evaluation(&session), &camera, pane, PickAction::Hover(off))
        .expect("an off-body cursor still un-projects");
    assert!(
        matches!(op, SessionOp::Hover(None)),
        "a miss hovers nothing, got {op:?}"
    );
    session.perform(op);
    assert!(session.hover().is_none());
    assert_eq!(
        session.selection().face().expect("still selected").name,
        first.name,
        "a hover moved the selection"
    );

    // A click on empty space clears the selection.
    let op = index
        .op_for(evaluation(&session), &camera, pane, PickAction::Select(off))
        .expect("un-projects");
    assert!(
        matches!(op, SessionOp::Select(Selection::None)),
        "a click on empty space clears, got {op:?}"
    );
    session.perform(op);
    assert_eq!(*session.selection(), Selection::None);

    // Leaving the pane clears the hover and nothing else.
    session.perform(SessionOp::Select(Selection::Node(extrude)));
    session.perform(SessionOp::Hover(Some(first.clone())));
    let op = index
        .op_for(evaluation(&session), &camera, pane, PickAction::ClearHover)
        .expect("clearing needs no ray");
    assert!(matches!(op, SessionOp::Hover(None)), "got {op:?}");
    session.perform(op);
    assert!(session.hover().is_none());
    assert_eq!(*session.selection(), Selection::Node(extrude));
}

/// Two picks on two different faces leave exactly one selection — the
/// single-select ruling as a property of the state, checked by walking
/// the whole op vocabulary rather than by reading the enum.
#[test]
fn picking_again_replaces_rather_than_accumulates() {
    let (doc, _) = slab(0.03, 0.02, 0.01, "r2-single-select");
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    let eval = evaluation(&session);
    let top = index
        .face_at(eval, &down_at(0.015, 0.010))
        .expect("no refusal")
        .expect("the top face");
    let side = index
        .face_at(
            eval,
            &Ray {
                origin: Point3::new(-1.0, 0.010, 0.005),
                dir: Vec3::new(1.0, 0.0, 0.0),
            },
        )
        .expect("no refusal")
        .expect("a wall");
    assert_ne!(top.name, side.name, "the fixture offers two distinct faces");
    session.perform(SessionOp::Select(Selection::Face(top)));
    session.perform(SessionOp::Select(Selection::Face(side.clone())));
    match session.selection() {
        Selection::Face(face) => assert_eq!(face.name, side.name),
        other => panic!("expected one face selection, got {other:?}"),
    }
}

/// The primary button is the select binding and moves no camera; the
/// navigation buttons pick nothing. Both mappings read one stream.
#[test]
fn the_two_mappings_partition_the_event_stream() {
    let map = InputMap::default();
    let camera = camera_on(&box_of([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]), 1.0);
    let pane = ViewportSize {
        width_px: 800.0,
        height_px: 600.0,
    };
    let events = [
        ViewportEvent::Hover { pos_px: [1.0, 2.0] },
        ViewportEvent::Click {
            button: PointerButton::Primary,
            pos_px: [1.0, 2.0],
        },
        ViewportEvent::Click {
            button: PointerButton::Middle,
            pos_px: [1.0, 2.0],
        },
        ViewportEvent::Click {
            button: PointerButton::Secondary,
            pos_px: [1.0, 2.0],
        },
        ViewportEvent::Leave,
        ViewportEvent::Drag {
            button: PointerButton::Middle,
            shift: false,
            alt: false,
            delta_px: [4.0, 4.0],
        },
        ViewportEvent::Scroll { units: 1.0 },
    ];
    for event in &events {
        let camera_op = map.map(event, pane, &camera);
        let pick_action = map.pick(event);
        assert!(
            camera_op.is_none() || pick_action.is_none(),
            "{event:?} is read by both mappings"
        );
    }
    assert_eq!(
        viewer::input::pick_stream(&map, &events),
        vec![
            PickAction::Hover([1.0, 2.0]),
            PickAction::Select([1.0, 2.0]),
            PickAction::ClearHover,
        ],
        "only the primary click, the hover and the leave pick"
    );
}

// -------------------------------------------------------------------
// 7. Tree ↔ viewport unity, and survival
// -------------------------------------------------------------------

/// One value: a viewport pick lights the owning feature's tree row and
/// fills the property panel, and a tree click needs no pick.
#[test]
fn the_viewport_and_the_tree_read_one_selection() {
    let (doc, extrude) = slab(0.03, 0.02, 0.01, "r2-unity");
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    let face = index
        .face_at(evaluation(&session), &down_at(0.015, 0.010))
        .expect("no refusal")
        .expect("a hit");
    session.perform(SessionOp::Select(Selection::Face(face)));
    assert_eq!(session.selection().node(), Some(extrude));
    let rows = session.tree_rows();
    assert!(
        rows.iter().any(|row| row.id == extrude),
        "the owning feature is a tree row"
    );
    assert!(
        !session.slot_rows().is_empty(),
        "the panel shows the owning feature's slots"
    );
    // The other direction needs no viewport at all.
    session.perform(SessionOp::Select(Selection::Node(extrude)));
    assert_eq!(session.selection().node(), Some(extrude));
    assert!(!session.slot_rows().is_empty());
}

/// Deleting the selected face's feature leaves the selection standing,
/// typed and unresolved, with the dependent affordances off.
#[test]
fn deleting_the_owning_feature_leaves_a_typed_unresolved_selection() {
    let (doc, extrude) = slab(0.03, 0.02, 0.01, "r2-delete");
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    let face = index
        .face_at(evaluation(&session), &down_at(0.015, 0.010))
        .expect("no refusal")
        .expect("a hit");
    session.perform(SessionOp::Select(Selection::Face(face.clone())));
    assert!(session.standing().live(), "the fresh pick is live");

    let outcome = session.perform(SessionOp::DeleteNode { node: extrude });
    assert!(outcome.refusal.is_none(), "the delete applied: {outcome:?}");
    session.pump();

    match session.selection() {
        Selection::Face(still) => assert_eq!(*still, face, "the selection was cleared or edited"),
        other => panic!("the selection did not survive: {other:?}"),
    }
    let standing = session.standing();
    assert!(!standing.live(), "a deleted feature's face is not live");
    let verdict = standing.unresolved().expect("a typed unresolved verdict");
    assert!(
        !matches!(verdict, Resolution::Resolved(_)),
        "the verdict is a refusal, not a resolution"
    );
    assert!(session.slot_rows().is_empty(), "the affordances are off");
    // Nothing crashed and nothing was silently cleared.
    assert!(session.hover().is_none());
}

/// A structural edit that consumes the selected face — a pattern's
/// count dropping the instance the face was picked on — leaves the
/// selection unresolved, and undoing it makes it live again.
#[test]
fn a_consumed_instance_leaves_the_selection_unresolved_until_undone() {
    let (doc, pattern) = pattern_of(3);
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    // A face on the LAST instance, which the count edit removes.
    let face = index
        .face_at(evaluation(&session), &down_at(0.0875, 0.0075))
        .expect("no refusal")
        .expect("the third instance is hit");
    assert_eq!(face.body, 2, "the picked face is on the last instance");
    session.perform(SessionOp::Select(Selection::Face(face.clone())));
    assert!(session.standing().live());

    let outcome = session.perform(SessionOp::SetSlot {
        node: pattern,
        slot: pncad::document::SlotId::Count,
        value: viewer::props::SlotValue::Count(2),
    });
    assert!(
        outcome.refusal.is_none(),
        "the count edit applied: {outcome:?}"
    );
    session.pump();
    assert_eq!(
        session.selection().face(),
        Some(&face),
        "the selection stands"
    );
    assert!(!session.standing().live(), "the consumed face is not live");
    assert!(session.standing().unresolved().is_some(), "and it is typed");

    session.perform(SessionOp::Undo);
    session.pump();
    assert_eq!(session.selection().face(), Some(&face), "undo kept it");
    assert!(
        session.standing().live(),
        "undoing the edit brings the face back"
    );
}

/// Undo across the selection's BIRTH: the pick was made on a document
/// state that undo removes, so the selection is left unresolved — and
/// redo makes it live again without re-picking.
#[test]
fn undo_across_the_selections_birth_and_back() {
    let (doc, pattern) = pattern_of(2);
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    session.perform(SessionOp::SetSlot {
        node: pattern,
        slot: pncad::document::SlotId::Count,
        value: viewer::props::SlotValue::Count(3),
    });
    session.pump();
    let index = landed_index(&session);
    let face = index
        .face_at(evaluation(&session), &down_at(0.0875, 0.0075))
        .expect("no refusal")
        .expect("the new instance is hit");
    assert_eq!(face.body, 2, "the pick is on the instance the edit created");
    session.perform(SessionOp::Select(Selection::Face(face.clone())));
    assert!(session.standing().live());

    session.perform(SessionOp::Undo);
    session.pump();
    assert_eq!(
        session.selection().face(),
        Some(&face),
        "nothing was destroyed"
    );
    assert!(!session.standing().live(), "the face's instance is gone");
    assert!(session.standing().unresolved().is_some());

    session.perform(SessionOp::Redo);
    session.pump();
    assert!(
        session.standing().live(),
        "redo brings the instance back and nothing was re-picked"
    );
}

/// A selection with no evaluation behind it is not reported live, and
/// offers no rows: "we cannot tell" is not "yes".
#[test]
fn a_selection_with_no_run_behind_it_is_not_live() {
    let (doc, extrude) = slab(0.03, 0.02, 0.01, "r2-no-run");
    let held = HeldEvaluator::new();
    let mut session = DocSession::new(doc, tol(), Box::new(held.clone()));
    session.pump();
    assert!(session.landed_pair().is_none(), "nothing has landed yet");
    session.perform(SessionOp::Select(Selection::Face(FaceSelection {
        name: StableName {
            kind: pncad::prelude::EntityKind::Face,
            node: extrude,
            path: Vec::new(),
        },
        node: extrude,
        body: 0,
    })));
    let standing = session.standing();
    assert!(!standing.live(), "no run behind it is not live");
    assert!(
        standing.unresolved().is_none(),
        "and it is not reported as vanished either"
    );
    assert!(session.slot_rows().is_empty());
}

// -------------------------------------------------------------------
// 8. The landed PAIR: doc and evaluation move together
// -------------------------------------------------------------------

/// An evaluation seam that holds every request until it is released —
/// the state a real background evaluator is in while a run is
/// outstanding, and the only way to observe the shown document being
/// AHEAD of the landed one.
#[derive(Clone, Default)]
struct HeldEvaluator {
    queue: Arc<Mutex<Vec<EvalRequest>>>,
}

impl HeldEvaluator {
    fn new() -> Self {
        Self::default()
    }

    /// Evaluate the OLDEST outstanding request and answer its result,
    /// for the caller to `land`.
    fn release_one(&self) -> Option<EvalDone> {
        let request = {
            let mut queue = self.queue.lock().expect("not poisoned");
            if queue.is_empty() {
                return None;
            }
            queue.remove(0)
        };
        let mut inline = InlineEvaluator::default();
        inline.submit(request);
        inline.poll()
    }

    fn outstanding(&self) -> usize {
        self.queue.lock().expect("not poisoned").len()
    }
}

impl EvalService for HeldEvaluator {
    fn submit(&mut self, request: EvalRequest) {
        self.queue.lock().expect("not poisoned").push(request);
    }

    fn cancel(&mut self) {}

    /// Never ready: this seam only answers through `release_one`, which
    /// is what lets a row observe the window where the shown document
    /// is ahead of the landed one.
    fn poll(&mut self) -> Option<EvalDone> {
        None
    }

    fn busy(&self) -> bool {
        self.outstanding() > 0
    }
}

/// While a run is outstanding, the landed pair is the OLD document
/// with the OLD evaluation — never the new document with the old run,
/// which is a pair no evaluation ever answered.
#[test]
fn the_landed_pair_is_never_a_run_that_never_happened() {
    let (doc, extrude) = slab(0.03, 0.02, 0.01, "r2-pairing");
    let held = HeldEvaluator::new();
    let mut session = DocSession::new(doc, tol(), Box::new(held.clone()));
    // Land the first run.
    let done = held.release_one().expect("the first request was submitted");
    session.land(done);
    let (first_doc, _) = session.landed_pair().expect("a pair landed");
    let first_nodes = first_doc.order().len();
    let first_generation = session.landed_generation().expect("a generation");

    // Edit: the shown document moves, the landed pair must not.
    session.perform(SessionOp::DeleteNode { node: extrude });
    assert!(held.outstanding() >= 1, "the edit asked for a new run");
    let (still_doc, _) = session.landed_pair().expect("the old pair stands");
    assert_eq!(
        still_doc.order().len(),
        first_nodes,
        "the landed document moved ahead of the landed evaluation"
    );
    assert!(
        still_doc.node(extrude).is_some(),
        "the landed document is the one the landed run answered"
    );
    assert_ne!(
        session.doc().order().len(),
        first_nodes,
        "the SHOWN document really did move"
    );
    assert_eq!(session.landed_generation(), Some(first_generation));

    // Release the run: now the pair moves, together.
    let done = held.release_one().expect("the edit's request");
    session.land(done);
    let (new_doc, _) = session.landed_pair().expect("the new pair");
    assert!(
        new_doc.node(extrude).is_none(),
        "the landed document is now the edited one"
    );
    assert_ne!(session.landed_generation(), Some(first_generation));
}

/// EVIDENCE — the sibling that was NOT re-paired. `tree_rows` reads the
/// SHOWN document against the landed evaluation, which is the mismatched
/// pair `landed_pair` was introduced to retire; this row records what
/// that reads like while a run is outstanding and asserts nothing about
/// whether it should change.
#[test]
fn tree_rows_still_read_the_shown_doc_against_the_old_evaluation() {
    let (doc, extrude) = slab(0.03, 0.02, 0.01, "r2-tree-pair");
    let held = HeldEvaluator::new();
    let mut session = DocSession::new(doc, tol(), Box::new(held.clone()));
    let done = held.release_one().expect("the first request");
    session.land(done);
    let before = session.tree_rows().len();
    session.perform(SessionOp::DeleteNode { node: extrude });
    let after = session.tree_rows();
    println!(
        "EVIDENCE tree_rows while a run is outstanding: {before} rows before the edit, \
         {} after; landed_pair still names {} nodes",
        after.len(),
        session
            .landed_pair()
            .map_or(0, |(doc, _)| doc.order().len())
    );
}

// -------------------------------------------------------------------
// 8b. End to end on a GALLERY document, through the shipped doors
// -------------------------------------------------------------------

/// The committed gallery ring, `doc_io`'s fixture. Re-stamped with this
/// run's ε below for the same reason that suite states: a saved
/// document records the ε it was decided at, and the matrix sweeps ε.
const GALLERY_RING: &str = include_str!("gallery_ring.v16.pncad");

/// The fixture's text with this process's ε line, taken from the
/// serializer rather than spelled here.
fn gallery_at(t: Tol) -> String {
    let probe: Doc<ProfileProgram> = Doc::empty_derived("r2-gui2-eps-probe", t);
    let probe_text = pncad::document::save(&probe, &[], t).expect("an empty document saves");
    let is_eps = |line: &str| line.trim_start().starts_with("\"epsilon\":");
    let wanted = probe_text
        .lines()
        .find(|line| is_eps(line))
        .expect("a saved document records its ε");
    let mut text: String = GALLERY_RING
        .lines()
        .map(|line| if is_eps(line) { wanted } else { line })
        .collect::<Vec<&str>>()
        .join("\n");
    text.push('\n');
    text
}

/// **The e2e walk this review owed**, on a real gallery document rather
/// than a fixture written for the occasion: open it through
/// `SessionOp::Open`, frame a camera on what it draws, cast a cursor ray
/// through that camera, hover, click, read the tree and the panel off
/// the one selection value, delete the owning feature, watch the typed
/// unresolved state, and undo back to live.
///
/// Every step goes through a shipped door — no direct field access, no
/// hand-assembled `PickTarget`, no ray invented outside the camera.
/// The transcript is printed for the review; the assertions are the
/// gate.
#[test]
fn a_gallery_document_selects_survives_and_recovers_end_to_end() {
    let t = tol();
    let dir = std::env::temp_dir().join(format!(
        "r2-gui2-e2e-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos())
    ));
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    let file = dir.join("ring.pncad");
    std::fs::write(&file, gallery_at(t)).expect("the fixture is writable");

    // 1. Open, through the session's own door.
    let mut session = DocSession::inline(Doc::empty_derived("r2-gui2-e2e", t), t);
    session.pump();
    let outcome = session.perform(SessionOp::Open(file.clone()));
    assert!(outcome.refusal.is_none(), "the gallery opens: {outcome:?}");
    session.pump();
    let index = index_at(&session, coarse());
    println!(
        "E2E opened: {} tree rows, {} drawn parts, {} ids",
        session.tree_rows().len(),
        index.parts().len(),
        index.ids().len()
    );
    assert!(
        !index.ids().is_empty(),
        "the gallery draws pickable patches"
    );

    // 2. A camera framed on what is actually drawn, and a cursor at the
    //    centre of the pane — the framing points the view axis at the
    //    scene's centre, so the centre pixel meets the body.
    let scene = index.scene().expect("the parts build a scene");
    let pane = ViewportSize {
        width_px: 1280.0,
        height_px: 800.0,
    };
    let aspect = pane.aspect().expect("a positive aspect");
    let camera = camera_on(&scene.bounds(), aspect);
    // A cursor aimed at the centroid of a drawn triangle, taken from the
    // scene's OWN position buffer. The pane centre would not do: the
    // gallery body is a ring, and the middle of the pane looks through
    // the hole — an honest miss, and not what this walk is about.
    let corners = scene.positions();
    assert!(corners.len() >= 3, "the scene draws triangles");
    let target = Point3::new(
        f64::from(corners[0][0] + corners[1][0] + corners[2][0]) / 3.0,
        f64::from(corners[0][1] + corners[1][1] + corners[2][1]) / 3.0,
        f64::from(corners[0][2] + corners[1][2] + corners[2][2]) / 3.0,
    );
    let centre = cursor_of(&camera, target, pane);

    // 3. Hover, then click — through the event stream and the input
    //    mapping, exactly as the viewport does.
    let map = InputMap::default();
    let stream = [
        ViewportEvent::Hover { pos_px: centre },
        ViewportEvent::Click {
            button: PointerButton::Primary,
            pos_px: centre,
        },
    ];
    for action in viewer::input::pick_stream(&map, &stream) {
        let op = index
            .op_for(evaluation(&session), &camera, pane, action)
            .expect("the centre cursor un-projects and the pick answers");
        session.perform(op);
    }
    let face = session
        .selection()
        .face()
        .expect("a cursor aimed at a drawn triangle selects a face")
        .clone();
    println!(
        "E2E picked: node {:?} body {} name kind {:?}",
        face.node, face.body, face.name.kind
    );

    // 4. Tree ↔ viewport unity, off the ONE value.
    let owner = session.selection().node().expect("a face owns a node");
    assert_eq!(owner, face.node);
    assert!(
        session.tree_rows().iter().any(|row| row.id == owner),
        "the owning feature is a row in the tree"
    );
    let standing = session.standing();
    assert!(standing.live(), "a fresh pick on the shown run is live");
    let rows_live = session.slot_rows().len();
    println!("E2E panel: {rows_live} slot row(s) for the owning feature");

    // 5. Delete the owning feature and watch the typed unresolved state.
    let outcome = session.perform(SessionOp::DeleteNode { node: owner });
    assert!(outcome.refusal.is_none(), "the delete applies: {outcome:?}");
    session.pump();
    assert_eq!(
        session.selection().face(),
        Some(&face),
        "the selection survived the edit that removed its referent"
    );
    let gone = session.standing();
    assert!(!gone.live(), "and it is no longer live");
    let verdict = gone.unresolved().expect("a typed verdict, not an absence");
    assert!(!matches!(verdict, Resolution::Resolved(_)));
    assert!(
        session.slot_rows().is_empty(),
        "the dependent affordances are off at the source"
    );
    println!("E2E after delete: live=false, unresolved={verdict:?}");

    // 6. Undo across the edit: the same value resolves again, and
    //    nothing was re-picked.
    session.perform(SessionOp::Undo);
    session.pump();
    assert_eq!(session.selection().face(), Some(&face));
    assert!(
        session.standing().live(),
        "undo brings the referent back and the stored name resolves"
    );
    assert_eq!(
        session.slot_rows().len(),
        rows_live,
        "the panel is back to what it showed before the delete"
    );

    // 7. The index the pick ran against belongs to the run on screen,
    //    and a rebuilt one describes the new generation.
    let generation = session.landed_generation().expect("a generation");
    assert!(
        !index.current_for(Some(generation), coarse()),
        "the pre-edit index is stale after two evaluations"
    );
    let rebuilt = index_at(&session, coarse());
    assert!(rebuilt.current_for(Some(generation), coarse()));
    let _ = std::fs::remove_dir_all(&dir);
}

// -------------------------------------------------------------------
// 9. Generation and δ invalidation
// -------------------------------------------------------------------

/// An index is current only for the generation AND the δ it was built
/// under, and `None` (no run) is never current.
#[test]
fn an_index_is_current_for_exactly_one_generation_and_delta() {
    let (doc, pattern) = pattern_of(2);
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    let generation = session.landed_generation().expect("a generation");
    assert!(index.current_for(Some(generation), delta()));
    assert!(!index.current_for(None, delta()), "no run is not current");
    assert!(
        !index.current_for(
            Some(generation),
            DisplayTolerance::new(1.0e-3).expect("positive")
        ),
        "a different δ is a different tessellation"
    );
    assert!(
        !index.current_for(Some(generation.next()), delta()),
        "a later generation is not current"
    );

    session.perform(SessionOp::SetSlot {
        node: pattern,
        slot: pncad::document::SlotId::Count,
        value: viewer::props::SlotValue::Count(3),
    });
    session.pump();
    let moved = session.landed_generation().expect("a new generation");
    assert_ne!(moved, generation);
    assert!(
        !index.current_for(Some(moved), delta()),
        "the stale index is not current for the new run"
    );
    // And the rebuilt one describes the new run.
    let rebuilt = landed_index(&session);
    assert!(rebuilt.current_for(Some(moved), delta()));
    assert!(
        rebuilt.ids().len() > index.ids().len(),
        "a third instance draws more patches"
    );
}

/// The picture and the picks are one tessellation: every part's mesh is
/// the mesh its `NodePick` indexes, the scene carries one id per drawn
/// corner, and every non-`NOTHING` id in the scene is one the map
/// assigned.
#[test]
fn the_drawn_scene_carries_exactly_the_indexs_ids() {
    let (doc, _) = pattern_of(2);
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    let scene = index.scene().expect("the parts build a scene");
    assert_eq!(
        scene.ids().len(),
        scene.positions().len(),
        "one id per drawn corner"
    );
    let assigned: std::collections::BTreeSet<u32> = index.ids().ids().collect();
    let drawn: std::collections::BTreeSet<u32> = scene.ids().iter().copied().collect();
    assert!(
        !drawn.contains(&IdMap::NOTHING),
        "every drawn corner of a pickable part carries an id"
    );
    assert_eq!(drawn, assigned, "the scene draws exactly the assigned ids");
    // Triangle counts agree with the parts the index picks against.
    let from_parts: usize = index
        .parts()
        .iter()
        .flat_map(|p| p.mesh().patches.iter())
        .map(|p| p.triangles.len())
        .sum();
    assert_eq!(scene.stats().triangles, from_parts);
}

/// `SceneError::MispairedIds` fires for a part whose id list is neither
/// empty nor one per patch, in BOTH directions — short and long.
///
/// The arm ships with a fail-loud justification ("drawing it would put
/// ids on the wrong triangles — the silent wrong answer the whole id
/// mapping exists to make impossible") and no row anywhere reaches it,
/// so nothing goes red if the check is ever relaxed.
#[test]
fn a_part_whose_ids_do_not_pair_with_its_patches_is_refused() {
    let (doc, _) = slab(0.03, 0.02, 0.01, "r2-mispaired");
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    let part = index.parts().first().expect("one drawn part");
    let patches = part.mesh().patches.len();
    assert!(patches >= 2, "the fixture has patches to mis-pair");

    let short: Vec<u32> = (1..=patches as u32 - 1).collect();
    let long: Vec<u32> = (1..=patches as u32 + 1).collect();
    for (label, ids) in [("short", &short), ("long", &long)] {
        let scene = viewer::scene::SceneMesh::build_parts(
            &[viewer::scene::ScenePart {
                mesh: part.mesh(),
                ids,
                probe: None,
            }],
            delta(),
        );
        match scene {
            Err(viewer::scene::SceneError::MispairedIds {
                ids: got,
                patches: want,
            }) => {
                assert_eq!(got, ids.len(), "{label}: the refusal names the id count");
                assert_eq!(want, patches, "{label}: and the patch count");
            }
            other => panic!("{label} id list was accepted: {other:?}"),
        }
    }
    // Empty is the unpickable part, and it is NOT a mis-pairing.
    assert!(
        viewer::scene::SceneMesh::build_parts(
            &[viewer::scene::ScenePart {
                mesh: part.mesh(),
                ids: &[],
                probe: None
            }],
            delta()
        )
        .is_ok(),
        "a part with no ids at all is the gathered-product case"
    );
}

// -------------------------------------------------------------------
// 10. The façade seal (claim: no arena key crosses into layer 3)
// -------------------------------------------------------------------

/// **The selection value is clean** — `FaceSelection` is a name, a node
/// and a body index, and it round-trips through `Debug` with nothing
/// arena-shaped in it. This is the half of G1 the unit's own rows
/// assert, taken here on a document of this suite's own.
#[test]
fn the_selection_value_holds_no_arena_key() {
    let (doc, _) = slab(0.03, 0.02, 0.01, "r2-g1");
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    let face = index
        .face_at(evaluation(&session), &down_at(0.015, 0.010))
        .expect("no refusal")
        .expect("a hit");
    // Structural: the three fields are all there is, and each is a
    // layer-3 value.
    let FaceSelection { name, node, body } = face.clone();
    assert_eq!(name.kind, pncad::prelude::EntityKind::Face);
    assert!(body < 8);
    assert!(session.doc().node(node).is_some(), "the node is a live one");
    // The name is float-free and structural: it clones, compares and
    // orders as a value, which an arena key scoped to one evaluation
    // could not be relied on to do across runs.
    let mut names = vec![name.clone(), name.clone()];
    names.dedup();
    assert_eq!(names.len(), 1, "a stable name is a value with identity");
    assert!(
        session.doc().node(name.node).is_some(),
        "the minting node is live"
    );
}

/// EVIDENCE — what the widened `pncad::select` door lets a layer-3
/// consumer do with the arena keys that ride in `Resolved::entity` and
/// `Tombstone::patch`.
///
/// The PR's stated seal was that a consumer "cannot spell their type, so
/// it cannot store one in its own state". This row is the counterexample
/// as compiled code: the key is stored in an ordinary consumer field and
/// `PartialEq` makes keys minted by two DIFFERENT evaluations comparable
/// — which is the body-lineage-scoped comparison the boundary exists to
/// forbid. Asserts nothing about what the answer should be; it records
/// that the operations compile and run.
///
/// **Kept green across the fix pass, with the route narrowed.** The fix
/// stopped carrying `Resolved` as a name at all, so the original
/// spelling of this row (`Option<pncad::select::Resolved>`) no longer
/// compiles — which is the naming barrier working. The capability it
/// demonstrates survives that, through the generic field below, and
/// that is the point the fixed prose now makes: the seal prevents
/// accidents, not determined consumers.
#[test]
fn arena_keys_can_be_stored_and_compared_through_the_widened_door() {
    /// A layer-3 struct with an arena-keyed field, declared without
    /// naming `Resolved` OR `EntityRef` — inference supplies both.
    struct ConsumerState<T> {
        kept: Option<T>,
    }

    let (doc, extrude) = slab(0.03, 0.02, 0.01, "r2-seal");
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let index = landed_index(&session);
    let face = index
        .face_at(evaluation(&session), &down_at(0.015, 0.010))
        .expect("no refusal")
        .expect("a hit");
    let (landed_doc, landed_eval) = session.landed_pair().expect("a pair");
    let first = match pncad::select::resolve(
        pncad::select::RunCtx {
            doc: landed_doc,
            eval: landed_eval,
        },
        &face.name,
    ) {
        Resolution::Resolved(resolved) => resolved,
        other => panic!("the fresh pick resolves: {other:?}"),
    };
    let mut state = ConsumerState { kept: Some(first) };

    // A structural edit, a new evaluation, a new resolution — and the
    // stored key compared against the fresh one across the boundary.
    session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: pncad::document::SlotId::Distance,
        value: viewer::props::SlotValue::Continuous(0.02),
    });
    session.pump();
    let (doc2, eval2) = session.landed_pair().expect("a second pair");
    let second = match pncad::select::resolve(
        pncad::select::RunCtx {
            doc: doc2,
            eval: eval2,
        },
        &face.name,
    ) {
        Resolution::Resolved(resolved) => resolved,
        other => panic!("the face survives a distance change: {other:?}"),
    };
    let kept = state.kept.take().expect("stored");
    println!(
        "EVIDENCE seal: stored a Resolved in a consumer field; \
         cross-evaluation `kept == second` is {}; entity fields print as {:?} / {:?}",
        kept == second,
        kept.entity,
        second.entity
    );
    state.kept = Some(second);
    assert!(state.kept.is_some());
}
