//! Shared acceptance-body builders for the STL export suites (via the
//! public profile/sweep APIs only — the same shapes as the mesh
//! acceptance suites and the CI export example).
#![allow(dead_code)] // each consumer uses a subset
#![allow(unreachable_pub)] // `mod`-included by several test binaries; `pub` is how each names it

use geom_core::Tol;
use geom_core::{Point2, Point3, Vec2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, BooleanResult};

pub fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

pub fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .unwrap()
}

/// An axis-aligned raw-extrude brick spanning `x × y × z`.
///
/// Its sketch plane is the world xy frame lifted to `z.0`, so the
/// extrusion runs +z and the body is exactly the closed box.
pub fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(x.0, y.0),
        Point2::new(x.1, y.0),
        Point2::new(x.1, y.1),
        Point2::new(x.0, y.1),
    ]);
    extrude(
        &validated_on(
            SketchPlane::from_frame(
                Point3::new(0.0, 0.0, z.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            lp,
        ),
        Extrusion::Distance(z.1 - z.0),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// [`validated`] on an explicit sketch plane rather than `xy`.
fn validated_on(plane: SketchPlane<f64>, lp: ProfileLoop<f64>) -> ValidatedProfile<f64> {
    Profile::new(plane, vec![lp])
        .validate(Tol::witness())
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
    // ONE split, both halves kept: `tiltedcut` extrudes a cylinder and
    // splits it, returning both sides of that single computation.
    // Calling it once per half ran the whole extrude+split twice and
    // discarded one half each time.
    let (tiltedcut_above, tiltedcut_below) = tiltedcut();
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
        // M5 PR 11: the trimmed-face lane's acceptance shapes under the
        // external admesh gate — the tiltedcut halves (conic-trimmed
        // cylinder walls) and boss∪plate (the first transverse curved
        // boolean; curved-planar shared seam arcs).
        ("tiltedcut_above", tiltedcut_above, 1e-2),
        ("tiltedcut_below", tiltedcut_below, 1e-2),
        ("boss_plate", boss_plate(), 1e-2),
    ]
}

/// M5 shape (i): a cylinder split by a tilted plane through the
/// mid-height axis point (exact `Ellipse` section carriers; the walls
/// tessellate through the PR 11 trimmed lane).
pub fn tiltedcut() -> (Body<f64>, Body<f64>) {
    use topo::splitting::{SplitPart, SplitPlane, split};
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(-1.0, 0.0), 1.0),
        ProfileVertex::new(p2(1.0, 0.0), 1.0),
    ]);
    let cylinder = extrude(
        &validated(vec![lp]),
        Extrusion::Distance(2.5),
        Tol::witness(),
    )
    .unwrap()
    .body;
    let phi: f64 = 0.3;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 1.25),
        normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
    };
    let result = split(&cylinder, &plane, Tol::witness()).unwrap();
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides carry material");
    };
    (above.clone(), below.clone())
}

/// M5 shape (ii): cylinder boss ∪ plate — the boss's three 120° wall
/// arcs pierce the plate's top face; the seam is three exact circle
/// arcs shared between the protruding walls and the ringed top face.
pub fn boss_plate() -> Body<f64> {
    use geom_core::Affine3;
    let plate_loop = ProfileLoop::polygon([p2(0.0, 0.0), p2(4.0, 0.0), p2(4.0, 4.0), p2(0.0, 4.0)]);
    let plate = extrude(
        &validated(vec![plate_loop]),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap()
    .body;
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th = deg.to_radians();
        p2(2.0 + 0.5 * th.cos(), 2.0 + 0.5 * th.sin())
    };
    let boss_loop = ProfileLoop::new(vec![
        ProfileVertex::new(at(0.0), b120),
        ProfileVertex::new(at(120.0), b120),
        ProfileVertex::new(at(240.0), b120),
    ]);
    let sketch = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.4)));
    let boss_profile = Profile::new(sketch, vec![boss_loop])
        .validate(Tol::witness())
        .unwrap();
    let boss = extrude(&boss_profile, Extrusion::Distance(1.2), Tol::witness())
        .unwrap()
        .body;
    let out = topo::union(&plate, &boss, Tol::witness()).unwrap();
    match out {
        BooleanResult::Body(bb) => bb.body,
        other => panic!("the boss union yields a body, got {other:?}"),
    }
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
            .validate(Tol::witness())
            .unwrap(),
        Extrusion::Distance(2.125),
        Tol::witness(),
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
        .validate(Tol::witness())
        .unwrap(),
        Extrusion::Distance(2.125),
        Tol::witness(),
    )
    .unwrap()
    .body;
    match topo::intersect(&a, &z, Tol::witness()) {
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
    extrude(
        &validated(vec![lp]),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// 4×4 square with a centered circular hole (two-vertex closed
/// carrier), genus 1.
pub fn holed_prism() -> Body<f64> {
    let outer = ProfileLoop::polygon([p2(-2.0, -2.0), p2(2.0, -2.0), p2(2.0, 2.0), p2(-2.0, 2.0)]);
    let hole = ProfileLoop::new(vec![
        ProfileVertex::new(p2(1.0, 0.0), 1.0),
        ProfileVertex::new(p2(-1.0, 0.0), 1.0),
    ]);
    extrude(
        &validated(vec![outer, hole]),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap()
    .body
}

pub fn ball() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

pub fn cone() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

pub fn washer() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

pub fn donut() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(2.0, -0.5), 1.0),
        ProfileVertex::new(p2(2.0, 0.5), 1.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

pub fn wedge() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap()
    .body
}
