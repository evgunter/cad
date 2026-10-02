//! Reviewer probe (PR #3805, third delta): does `EdgeCurve::certify`
//! (the gate `Body::set_edge_curve` runs) admit an ellipse stored with a
//! NEGATIVE `minor`? Mount as `mod reviewer_signed_minor;` in
//! `crates/geom-brep/tests/all.rs` with
//! `#[path = "../../../probes/reach_3805_signed_minor_certify.rs"]`.
//!
//! The plane z = 0 cut of a cylinder (radius 1) whose axis leans 0.5 rad
//! in xz: the ellipse a = 1/cos 0.5, b = 1 about the origin. Stored as
//! (a, b) and as (a, −b) (the same locus, traversed the other way, its
//! quarter arc ending at (0, −b) instead of (0, b)).

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, SurfaceKey};
use geom_core::{Band, Point3, Tol, Vec3};
use slotmap::SlotMap;

#[test]
fn reviewer_a_negative_minor_ellipse_at_the_certify_gate() {
    let band = Band::linear(Tol::witness()).unwrap();
    let alpha: f64 = 0.5;
    let (a, b) = (1.0 / alpha.cos(), 1.0);
    let mut surfaces: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let plane = surfaces.insert(Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    });
    let cyl = surfaces.insert(Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::new(alpha.sin(), 0.0, alpha.cos()),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    });
    for minor in [b, -b] {
        let e = Curve3::Ellipse {
            center: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major: a,
            minor,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1: plane,
                s2: cyl,
                witness: e.eval(core::f64::consts::FRAC_PI_4),
            },
            carrier: e.clone(),
            param_start: 0.0,
            param_end: core::f64::consts::FRAC_PI_2,
        };
        let got = EdgeCurve::certify(
            spec,
            e.eval(0.0),
            e.eval(core::f64::consts::FRAC_PI_2),
            |k| surfaces.get(k).cloned(),
            band,
        );
        println!("minor {minor}: {:?}", got.as_ref().map(|_| "certified"));
    }
}
