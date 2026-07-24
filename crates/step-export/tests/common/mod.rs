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

/// A revolved ball — sphere surface, deliberately OUTSIDE the M4
/// analytic subset (drives the typed `UnsupportedSurface` refusal).
pub fn ball() -> Body<f64> {
    use profile::ProfileVertex;
    use sweep::{Revolution, RevolveAxis, revolve};
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
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    revolve(&profile, axis, Revolution::Full).unwrap().body
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
