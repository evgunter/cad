//! Shared acceptance-body builders for the STEP export suites, via the
//! public profile/sweep/boolean APIs only (the same shapes as the M3
//! STL review suites: bricks, the pocketed die, the corner-kiss
//! assembly, the voided subtract).
#![allow(dead_code)] // each consumer uses a subset
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tolerance, Vec3};
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanResult, BooleanResultKind, subtract, union};

fn validated(plane: SketchPlane<f64>, lp: ProfileLoop<f64>) -> ValidatedProfile<f64> {
    Profile::new(plane, vec![lp])
        .validate(Tolerance::get())
        .unwrap()
}

/// An axis-aligned brick `[x0,x1]×[y0,y1]×[z0,z1]`.
pub fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(x.0, y.0),
        Point2::new(x.1, y.0),
        Point2::new(x.1, y.1),
        Point2::new(x.0, y.1),
    ]);
    extrude(
        &validated(
            SketchPlane::from_frame(
                Point3::new(0.0, 0.0, z.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            lp,
        ),
        Extrusion::Distance(z.1 - z.0),
    )
    .unwrap()
    .body
}

/// The unit cube `[0,1]³` — the spike's 6/12/8 reference shape.
pub fn cube() -> Body<f64> {
    brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0))
}

/// A pocketed die at `[x0,x0+1]³`: unit cube minus a centered
/// 0.5×0.5×0.5 pocket opening through the TOP face (the M3 die shape;
/// exact volume 0.875). A genuine boolean result: the top face carries
/// a ring (the pocket mouth).
pub fn die(x0: f64, y0: f64, z0: f64) -> Body<f64> {
    let cube = brick((x0, x0 + 1.0), (y0, y0 + 1.0), (z0, z0 + 1.0));
    let cutter = brick(
        (x0 + 0.25, x0 + 0.75),
        (y0 + 0.25, y0 + 0.75),
        (z0 + 0.5, z0 + 1.5),
    );
    let BooleanResult::Body(b) = subtract(&cube, &cutter).unwrap() else {
        panic!("die subtract is a body");
    };
    b.body
}

/// Two pocketed dies kissing at the corner `(1,1,1)` — the M3 R6
/// assembly: one solid, TWO shells (both outward), exact volume 1.75.
pub fn kiss_assembly() -> Body<f64> {
    let d1 = die(0.0, 0.0, 0.0);
    let d2 = die(1.0, 1.0, 1.0);
    let BooleanResult::Body(assembly) = union(&d1, &d2).unwrap() else {
        panic!("kiss union is a body");
    };
    assert_eq!(assembly.body.shells().count(), 2, "two kissing shells");
    assembly.body
}

/// A∖B with B strictly inside A: `[0,3]³` minus `[1,2]³` — the Voided
/// boolean result (outer shell + reverted void shell; cavity volume 1).
pub fn voided() -> Body<f64> {
    let a = brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0));
    let b = brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let BooleanResult::Body(result) = subtract(&a, &b).unwrap() else {
        panic!("voided subtract is a body");
    };
    assert_eq!(result.kind, BooleanResultKind::Voided, "B inside A voids");
    result.body
}

// ---- the curved corpus (M5 PR 13) ------------------------------------
//
// The R5 shapes constructible at rest, chosen so that every new writer
// arm has a body behind it: CIRCLE and CYLINDRICAL_SURFACE (several),
// ELLIPSE (`cut_cylinder`), SPHERICAL_SURFACE (`ball`),
// CONICAL_SURFACE (`cone`), TOROIDAL_SURFACE (`donut`), and the
// `same_sense = .F.` arm (`notched`, `washer`). Recipes are the same
// ones the mesh/stl/sweep suites use — deliberately not new geometry.

/// The revolve axis of the corpus: the profile plane's +y through the
/// origin (so revolved bodies stand on the world Y axis).
fn revolve_y() -> sweep::RevolveAxis<f64> {
    sweep::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    }
}

/// A revolved unit ball — ONE `Surface::Sphere` carrying two half-bands
/// (V2 E2 F2), the seam meridian and its π copy as `Circle` carriers,
/// the poles as ordinary vertices. Exact volume 4π/3.
pub fn ball() -> Body<f64> {
    use profile::ProfileVertex;
    use sweep::{Revolution, revolve};
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: Point2::new(0.0, -1.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: Point2::new(0.0, 1.0),
            bulge: 0.0,
        },
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    revolve(&profile, revolve_y(), Revolution::Full)
        .unwrap()
        .body
}

/// A revolved cone: the right triangle (base radius 1, height 1)
/// swept fully — `Surface::Cone` (apex fan) plus the base `Plane`
/// disc. Exact volume π/3. The one corpus body that exercises the
/// apex-placement `CONICAL_SURFACE` encoding.
pub fn cone() -> Body<f64> {
    use sweep::{Revolution, revolve};
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    revolve(&profile, revolve_y(), Revolution::Full)
        .unwrap()
        .body
}

/// A revolved donut: the radius-½ circle centred at (2, 0) swept
/// fully — a single `Surface::Torus`, both meridians `Seam`. Exact
/// volume 2π²Rr² = π².
pub fn donut() -> Body<f64> {
    use profile::ProfileVertex;
    use sweep::{Revolution, revolve};
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: Point2::new(2.0, -0.5),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: Point2::new(2.0, 0.5),
            bulge: 1.0,
        },
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    revolve(&profile, revolve_y(), Revolution::Full)
        .unwrap()
        .body
}

/// A revolved washer: the rectangle [1,2]×[0,1] swept fully — genus 1,
/// two annuli and two full-2π cylinder walls. Its BORE wall and its
/// under-side annulus both carry `sense: false` (S11), so this is the
/// corpus's two-`.F.` body. Exact volume 3π.
pub fn washer() -> Body<f64> {
    use sweep::{Revolution, revolve};
    let lp = ProfileLoop::polygon([
        Point2::new(1.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(2.0, 1.0),
        Point2::new(1.0, 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    revolve(&profile, revolve_y(), Revolution::Full)
        .unwrap()
        .body
}

/// M5 shape (i), the above half: a unit cylinder of height 2.5 split
/// by a plane tilted 0.3 rad through the mid-height axis point. The
/// section rim is an exact `Ellipse` (semi-axes 1 and 1/cos 0.3) —
/// the corpus's only ELLIPSE carrier. Exact volume π·1.25 (the tilted
/// plane passes through the axis midpoint, so it halves the cylinder).
pub fn cut_cylinder() -> Body<f64> {
    use profile::ProfileVertex;
    use topo::splitting::{SplitPart, SplitPlane, split};
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: Point2::new(-1.0, 0.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: Point2::new(1.0, 0.0),
            bulge: 1.0,
        },
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    let cylinder = extrude(&profile, Extrusion::Distance(2.5)).unwrap().body;
    let phi: f64 = 0.3;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 1.25),
        normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
    };
    let result = split(&cylinder, &plane).unwrap();
    let SplitPart::Body(above) = &result.above else {
        panic!("the above half carries material");
    };
    above.clone()
}

/// M5 shape (ii): a 4×4×1 plate ∪ a radius-½ cylindrical boss (three
/// 120° arc segments) rising from z = 0.4 to z = 1.6 — the first
/// transverse curved boolean. The seam is three exact circle arcs
/// shared between the protruding walls and the plate's ringed top
/// face. Exact volume 16 + π·0.25·0.6.
pub fn boss_union() -> Body<f64> {
    use geom_core::Affine3;
    use profile::ProfileVertex;
    let plate_loop = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(4.0, 0.0),
        Point2::new(4.0, 4.0),
        Point2::new(0.0, 4.0),
    ]);
    let plate = extrude(
        &validated(SketchPlane::xy(), plate_loop),
        Extrusion::Distance(1.0),
    )
    .unwrap()
    .body;
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th: f64 = deg.to_radians();
        Point2::new(2.0 + 0.5 * th.cos(), 2.0 + 0.5 * th.sin())
    };
    let boss_loop = ProfileLoop::new(vec![
        ProfileVertex {
            pos: at(0.0),
            bulge: b120,
        },
        ProfileVertex {
            pos: at(120.0),
            bulge: b120,
        },
        ProfileVertex {
            pos: at(240.0),
            bulge: b120,
        },
    ]);
    let sketch = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.4)));
    let boss_profile = Profile::new(sketch, vec![boss_loop])
        .validate(Tolerance::get())
        .unwrap();
    let boss = extrude(&boss_profile, Extrusion::Distance(1.2))
        .unwrap()
        .body;
    let BooleanResult::Body(bb) = union(&plate, &boss).unwrap() else {
        panic!("the boss union yields a body");
    };
    bb.body
}

/// The S11 notched prism: a 2×1.5 rectangle with a CONVEX 45° bulge on
/// the bottom edge and an equal CONCAVE bite on the top, extruded 1.
/// The two bulges cancel, so the exact volume is 3. The concave wall
/// is the kernel's canonical `sense: false` face — this body is why the
/// `same_sense = .F.` arm exists.
pub fn notched() -> Body<f64> {
    let b = core::f64::consts::FRAC_PI_8.tan();
    let lp = ProfileLoop::builder(Point2::new(0.0, 0.0))
        .arc_to(Point2::new(2.0, 0.0), b)
        .line_to(Point2::new(2.0, 1.5))
        .arc_to(Point2::new(0.0, 1.5), -b)
        .close();
    extrude(&validated(SketchPlane::xy(), lp), Extrusion::Distance(1.0))
        .unwrap()
        .body
}

/// **The committed-fixture corpus, in file order** — the single list
/// behind `examples/export_fixtures.rs` (which writes the `.step`
/// files), `tests/export.rs::committed_fixtures_are_byte_golden`
/// (which pins them), and `scripts/check_step.sh` (which imports every
/// `.step` in the directory into FreeCAD/OCC against its hand-authored
/// `.expect` sidecar — a `.step` WITHOUT a sidecar is a hard failure,
/// so a row added here needs one written by hand).
///
/// Order is planar-first then curved, and it is the order the files
/// were minted in; it has no semantic weight beyond keeping diffs
/// readable.
pub fn fixture_corpus() -> Vec<(&'static str, Body<f64>)> {
    vec![
        // The M4 planar set.
        ("cube", cube()),
        ("die", die(0.0, 0.0, 0.0)),
        ("kiss_assembly", kiss_assembly()),
        // The M5 PR 13 curved set.
        ("cut_cylinder", cut_cylinder()),
        ("boss_union", boss_union()),
        ("notched", notched()),
        ("washer", washer()),
        ("ball", ball()),
        ("cone", cone()),
        ("donut", donut()),
    ]
}

/// Census tuple (faces, edges, vertices) of a body — the kernel-side
/// oracle the parse-back reconstruction must match.
pub fn census(body: &Body<f64>) -> (usize, usize, usize) {
    (
        body.faces().count(),
        body.edges().count(),
        body.vertices().count(),
    )
}
