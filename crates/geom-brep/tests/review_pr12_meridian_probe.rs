//! Blinded-review probe (NOT for merge): the coaxiality theorem's
//! counterexample class — a torus is tangent to a cylinder (and to a
//! sphere) along its MERIDIAN circle, and the torus is NOT a surface
//! of revolution about that circle's axis.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::{
    EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, SurfaceKey, tangent_certificate_lane,
};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};
use slotmap::SlotMap;

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

/// Torus R=1, r=0.2 about z; meridian circle at azimuth 0: center
/// (1,0,0), axis y, radius 0.2. A sphere of radius 0.2 centred at
/// (1,0,0) is tangent to the torus along that WHOLE circle, and the
/// torus is not coaxial with the circle.
#[test]
fn probe_meridian_sphere_torus() {
    let torus = Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major_radius: 1.0,
        minor_radius: 0.2,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let sphere = Surface::Sphere {
        center: Point3::new(1.0, 0.0, 0.0),
        radius: 0.2,
        axis: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let circle = Curve3::Circle {
        center: Point3::new(1.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 1.0, 0.0),
        radius: 0.2,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    // Sanity: the circle lies on both surfaces (residual 0 at samples)
    for i in 0..9 {
        let t = 2.0 * core::f64::consts::PI * f64::from(i) / 9.0;
        let p = circle.eval(t);
        let rt = geom_brep::implicit_residual(&torus, p);
        let rs = geom_brep::implicit_residual(&sphere, p);
        assert!(rt.abs() < 1e-12 && rs.abs() < 1e-12, "on both: {rt} {rs}");
    }
    assert!(
        tangent_certificate_lane(&circle, &torus, &sphere),
        "the lane ADMITS the pair (no gate)"
    );
    let mut surfaces: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let k1 = surfaces.insert(torus);
    let k2 = surfaces.insert(sphere);
    let quarter = core::f64::consts::FRAC_PI_2;
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::TangentIntersection {
            s1: k1,
            s2: k2,
            witness: circle.eval(quarter / 2.0),
        },
        carrier: circle,
        param_start: 0.0,
        param_end: quarter,
    };
    let (p0, p1) = (spec.carrier.eval(0.0), spec.carrier.eval(quarter));
    match EdgeCurve::certify(spec, p0, p1, |k| surfaces.get(k).cloned(), band()) {
        Ok(c) => println!(
            "MERIDIAN PROBE: certifies; max_residual {:e}",
            c.certificate().max_residual
        ),
        Err(e) => println!("MERIDIAN PROBE: refuses: {e:?}"),
    }
}
