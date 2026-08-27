//! R2 REVIEW PROBES (lane-local, NOT part of PR #1073).
//!
//! Claim-1 attack: the D2 bit-diff row measures a cylinder seam at
//! r = 2 (0 ULP) and a plane iso (1907 ULP). These probes measure the
//! SAME legacy-vs-collapsed delta on the seam classes the row does
//! not name: a sphere seam, a cone seam, and a cylinder seam at a
//! radius where the quadratic term of the implicit form does NOT
//! round away.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{
    CERT_SAMPLES, EdgeCurve, EdgeCurveSpec, EdgeGeometry, implicit_residual, sample_param,
};
use geom_core::{Band, Point3, Tol, Vec3};

fn table(
    surfs: Vec<Surface<f64>>,
) -> (
    Vec<geom_brep::SurfaceKey>,
    impl Fn(geom_brep::SurfaceKey) -> Option<Surface<f64>>,
) {
    let mut map: slotmap::SlotMap<geom_brep::SurfaceKey, Surface<f64>> =
        slotmap::SlotMap::with_key();
    let keys: Vec<geom_brep::SurfaceKey> = surfs.into_iter().map(|s| map.insert(s)).collect();
    (keys, move |k| map.get(k).cloned())
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn eps() -> f64 {
    Tol::witness().get().eps
}

fn ulps(a: f64, b: f64) -> i64 {
    let (x, y) = (a.to_bits() as i64, b.to_bits() as i64);
    (x - y).abs()
}

/// The pre-collapse SEAM meter, verbatim (main's check 4 seam arm):
/// endpoint pins, then per sample the implicit residual and the two
/// seam predicates. `implicit_residual` is called through the crate's
/// own export, so the legacy value is bitwise the shipped one.
fn legacy_seam_max(
    spec: &EdgeCurveSpec<f64>,
    surface: &Surface<f64>,
    anchor: Point3<f64>,
    axis: Vec3<f64>,
    u_ref: Vec3<f64>,
    start: Point3<f64>,
    end: Point3<f64>,
) -> f64 {
    let (t0, t1) = (spec.param_start, spec.param_end);
    let mut m = spec.carrier.eval(t0).distance(start);
    m = m.max(spec.carrier.eval(t1).distance(end));
    let v_ref = axis.cross(u_ref);
    for i in 0..CERT_SAMPLES {
        let p = spec.carrier.eval(sample_param(t0, t1, i));
        m = m.max(implicit_residual(surface, p).abs());
        // seam_frame, replicated: w is the radial component.
        let q = p - anchor;
        let w = q - axis * q.dot(axis);
        m = m.max(w.dot(v_ref).abs());
        m = m.max((0.0 - w.dot(u_ref)).max(0.0).abs());
    }
    m
}

fn seam_delta(
    surface: Surface<f64>,
    anchor: Point3<f64>,
    axis: Vec3<f64>,
    u_ref: Vec3<f64>,
    carrier: Curve3<f64>,
    t0: f64,
    t1: f64,
) -> (i64, f64, f64) {
    let (keys, lookup) = table(vec![surface.clone()]);
    let p0 = carrier.eval(t0);
    let p1 = carrier.eval(t1);
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::Seam { surface: keys[0] },
        carrier,
        param_start: t0,
        param_end: t1,
    };
    let cert = EdgeCurve::certify(spec.clone(), p0, p1, &lookup, band())
        .expect("this seam class certified on main and must still certify")
        .certificate()
        .clone();
    let legacy = legacy_seam_max(&spec, &surface, anchor, axis, u_ref, p0, p1);
    (
        ulps(cert.max_residual, legacy),
        legacy,
        cert.max_residual,
    )
}

/// Sphere seam meridian, in-band radial drift 0.25 eps — the D2 row's
/// own perturbation posture, on a chart class the row does not name.
#[test]
fn r2_probe_sphere_seam_bit_move() {
    let r = 2.0;
    let d = eps() * 0.25;
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: r,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    // Meridian in the x-z halfplane (x > 0): a circle about -y so the
    // traversal runs south-to-north, radius r + d (radial drift).
    let carrier = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: r + d,
        u_ref: Vec3::unit_x(),
    };
    let (delta, legacy, now) = seam_delta(
        sphere,
        Point3::origin(),
        Vec3::unit_z(),
        Vec3::unit_x(),
        carrier,
        -1.2,
        1.3,
    );
    panic!("R2 sphere-seam row: legacy {legacy:e} m, now {now:e} m, delta {delta} ULP");
}

/// Cone seam meridian, in-band normal drift 0.25 eps.
#[test]
fn r2_probe_cone_seam_bit_move() {
    let half_angle = 0.5_f64;
    let d = eps() * 0.25;
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle,
        u_ref: Vec3::unit_x(),
    };
    let (s, c) = half_angle.sin_cos();
    // Ruling direction in the seam halfplane; drift along the outward
    // normal (c, 0, -s).
    let dir = Vec3::new(s, 0.0, c);
    let normal = Vec3::new(c, 0.0, -s);
    let carrier = Curve3::Line {
        origin: Point3::origin() + normal * d,
        dir,
    };
    let (delta, legacy, now) = seam_delta(
        cone,
        Point3::origin(),
        Vec3::unit_z(),
        Vec3::unit_x(),
        carrier,
        0.5,
        2.0,
    );
    panic!("R2 cone-seam row: legacy {legacy:e} m, now {now:e} m, delta {delta} ULP");
}

/// Cylinder seam at r = 1e-4: same fixture SHAPE as the D2 row's
/// cylinder-seam, different radius — the quadratic term d^2/(2r) of
/// the legacy implicit form no longer rounds away against r^2's ULP.
#[test]
fn r2_probe_small_cylinder_seam_bit_move() {
    let r = 1e-4;
    let d = eps() * 0.25;
    let cyl = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    };
    let carrier = Curve3::Line {
        origin: Point3::new(r + d, 0.0, 0.0),
        dir: Vec3::unit_z(),
    };
    let (delta, legacy, now) = seam_delta(
        cyl,
        Point3::origin(),
        Vec3::unit_z(),
        Vec3::unit_x(),
        carrier,
        0.0,
        3.0,
    );
    panic!("R2 small-cylinder-seam row: legacy {legacy:e} m, now {now:e} m, delta {delta} ULP");
}
