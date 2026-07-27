//! Shared acceptance-body builders for the STL export suites (via the
//! public profile/sweep APIs only — the same shapes as the mesh
//! acceptance suites and the CI export example).
#![allow(dead_code)] // each consumer uses a subset

use geom_core::{Point2, Point3, Tolerance, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, BooleanResult};

pub fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

pub fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tolerance::get())
        .unwrap()
}

pub fn axis_y() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

/// The acceptance bodies, named, in fixed order, each with its export
/// δ (used by the export tests and the CI example).
///
/// Per-body δ notes: the donut pays the quadratic CDT cost (mesh's
/// documented perf characteristic — δ = 1e-2 is ~40 s and 143k
/// triangles in debug), so it exports at a coarser δ; the cone must
/// stay at δ ≤ 1e-2 because coarser apex fans emit exactly-collinear
/// (zero-3D-area) triangles — distinct indices, so the tessellator's
/// id-degenerate drop misses them — which the STL writer refuses with
/// a typed `DegenerateTriangle` (fail loud; see
/// `degenerate_apex_fan_is_refused_typed` in the export suite).
pub fn acceptance_bodies() -> Vec<(&'static str, Body<f64>, f64)> {
    vec![
        ("l_prism", l_prism(), 1e-2),
        ("holed_prism", holed_prism(), 1e-2),
        ("ball", ball(), 1e-2),
        ("cone", cone(), 1e-2),
        ("washer", washer(), 1e-2),
        ("donut", donut(), 8e-2),
        ("wedge", wedge(), 1e-2),
        // Issue #111: the A×Z boolean's near-collinear seam boundaries,
        // whose exterior needle triangle used to leak past `check_mesh`
        // into a non-watertight STL. Here so that the EXTERNAL admesh
        // gate covers the class too, not only our own checker.
        ("az_intersect", az_intersect(), 1e-2),
    ]
}

/// A (counter-hole variant) × Z, the issue-#93 acceptance body: four
/// boundary vertices on one carrier line with one of them 1 ulp off it.
pub fn az_intersect() -> Body<f64> {
    let xy = |z: f64| {
        SketchPlane::from_frame(
            Point3::new(0.0, 0.0, z),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
    };
    let a_outline = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(0.625, 0.0),
        p2(0.8125, 1.0),
        p2(1.1875, 1.0),
        p2(1.375, 0.0),
        p2(2.0, 0.0),
        p2(1.125, 2.5),
        p2(0.875, 2.5),
    ]);
    let a_counter = ProfileLoop::polygon([p2(0.90625, 1.4375), p2(1.09375, 1.4375), p2(1.0, 2.0)]);
    let a = extrude(
        &Profile::new(xy(-0.0625), vec![a_outline, a_counter])
            .validate(Tolerance::get())
            .unwrap(),
        Extrusion::Distance(2.125),
    )
    .unwrap()
    .body;
    // Z in (u, v) = (y, z), diagonal at slope 3/5.
    let z_poly = ProfileLoop::polygon([
        p2(-0.0625, 0.0),
        p2(2.5625, 0.0),
        p2(2.5625, 0.4375),
        p2(0.6875, 0.4375),
        p2(2.5625, 1.5625),
        p2(2.5625, 2.0),
        p2(-0.0625, 2.0),
        p2(-0.0625, 1.5625),
        p2(1.8125, 1.5625),
        p2(-0.0625, 0.4375),
    ]);
    let z = extrude(
        &Profile::new(
            SketchPlane::from_frame(
                Point3::new(-0.0625, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            vec![z_poly],
        )
        .validate(Tolerance::get())
        .unwrap(),
        Extrusion::Distance(2.125),
    )
    .unwrap()
    .body;
    match topo::intersect(&a, &z) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("A×Z intersect did not produce a body ({other:?})"),
    }
}

pub fn l_prism() -> Body<f64> {
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 1.0),
        p2(1.0, 1.0),
        p2(1.0, 2.0),
        p2(0.0, 2.0),
    ]);
    extrude(&validated(vec![lp]), Extrusion::Distance(1.0))
        .unwrap()
        .body
}

/// 4×4 square with a centered circular hole (two-vertex closed
/// carrier), genus 1.
pub fn holed_prism() -> Body<f64> {
    let outer = ProfileLoop::polygon([p2(-2.0, -2.0), p2(2.0, -2.0), p2(2.0, 2.0), p2(-2.0, 2.0)]);
    let hole = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(1.0, 0.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(-1.0, 0.0),
            bulge: 1.0,
        },
    ]);
    extrude(&validated(vec![outer, hole]), Extrusion::Distance(1.0))
        .unwrap()
        .body
}

pub fn ball() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(0.0, -1.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(0.0, 1.0),
            bulge: 0.0,
        },
    ]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
        .unwrap()
        .body
}

pub fn cone() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
        .unwrap()
        .body
}

pub fn washer() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
        .unwrap()
        .body
}

pub fn donut() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(2.0, -0.5),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(2.0, 0.5),
            bulge: 1.0,
        },
    ]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
        .unwrap()
        .body
}

pub fn wedge() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
    )
    .unwrap()
    .body
}
