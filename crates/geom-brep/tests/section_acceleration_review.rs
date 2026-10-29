//! **A section's bend in a curved face reads the curve's own
//! acceleration** (PR 5968 review): `implicit_section_acceleration`
//! against the section traced by Newton steps, for a plane through a
//! cylinder, a cone and a sphere (the reference curved, `∇²F ≠ 0`
//! along the tangent), a cylinder through a cylinder, and a plane as
//! the reference (where it is `implicit_graph_acceleration`). Each pose
//! is also checked against a closed form where one is short: a plane
//! cutting a sphere in a circle of radius `r` has geodesic curvature
//! `√(R² − r²)/(R r)`. And `tied_germ_turn` orders two germs the
//! way their traced sections leave each other.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::{
    implicit_gradient, implicit_graph_acceleration, implicit_residual,
    implicit_section_acceleration, tied_germ_turn,
};
use geom_core::{Band, Point3, Sign, Tol, Vec3};

fn unit(v: Vec3<f64>) -> Vec3<f64> {
    v / v.norm()
}

/// The point of `f ∩ g` on the plane through `p + s·d` normal to `d`,
/// by Newton steps from `p + s·d`.
fn on_section(
    f: &Surface<f64>,
    g: &Surface<f64>,
    p: Point3<f64>,
    d: Vec3<f64>,
    s: f64,
) -> Point3<f64> {
    let mut x = p + d * s;
    for _ in 0..60 {
        let (rf, rg) = (implicit_residual(f, x), implicit_residual(g, x));
        let (gf, gg) = (implicit_gradient(f, x), implicit_gradient(g, x));
        // Steps in the plane normal to d: x += a·u + b·w.
        let u = unit(gf - d * gf.dot(d));
        let w = d.cross(u);
        let (a11, a12, a21, a22) = (gf.dot(u), gf.dot(w), gg.dot(u), gg.dot(w));
        let det = a11 * a22 - a12 * a21;
        let a = (-rf * a22 + rg * a12) / det;
        let b = (-rg * a11 + rf * a21) / det;
        x = x + u * a + w * b;
    }
    x
}

/// The section's acceleration along `across` at `p`, traced.
fn traced(
    f: &Surface<f64>,
    g: &Surface<f64>,
    p: Point3<f64>,
    d: Vec3<f64>,
    across: Vec3<f64>,
) -> f64 {
    let h = 1e-4;
    let (a, b) = (on_section(f, g, p, d, h), on_section(f, g, p, d, -h));
    ((a - p) + (b - p)).dot(across) / (h * h)
}

fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
    let n = unit(normal);
    let seed = if n.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    Surface::Plane {
        origin,
        normal: n,
        u_ref: unit(seed - n * seed.dot(n)),
    }
}

fn frame(axis: Vec3<f64>) -> Vec3<f64> {
    let seed = if axis.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    unit(seed - axis * seed.dot(axis))
}

/// Pairs `(reference, other, p)` with `p` on both.
/// A named pose: the reference surface, the other, and their shared point.
type Pose = (&'static str, Surface<f64>, Surface<f64>, Point3<f64>);

fn poses() -> Vec<Pose> {
    let z = Vec3::new(0.0, 0.0, 1.0);
    let mut out = Vec::new();
    for &r in &[0.5, 1.0, 3.0] {
        let cyl = Surface::Cylinder {
            origin: Point3::origin(),
            axis: z,
            radius: r,
            u_ref: frame(z),
        };
        let p = Point3::new(r * 0.6, r * 0.8, 0.3);
        for m in [
            Vec3::new(0.3, -0.2, 1.0),
            Vec3::new(-0.7, 0.4, 0.5),
            Vec3::new(0.1, 0.9, 0.2),
        ] {
            out.push(("plane through a cylinder", cyl.clone(), plane(p, m), p));
            out.push(("cylinder as the other", plane(p, m), cyl.clone(), p));
        }
    }
    // A cone, apex at the origin, half-angle 0.5, about z.
    for &alpha in &[0.3, 0.5, 1.1] {
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: z,
            half_angle: alpha,
            u_ref: frame(z),
        };
        let v = 1.7;
        let (s, c) = (alpha.sin(), alpha.cos());
        let p = Point3::new(v * s * 0.28, v * s * 0.96, v * c);
        for m in [
            Vec3::new(0.3, -0.2, 1.0),
            Vec3::new(-0.7, 0.4, 0.5),
            Vec3::new(1.0, 0.1, 0.05),
        ] {
            out.push(("plane through a cone", cone.clone(), plane(p, m), p));
        }
    }
    // A sphere.
    let sphere = Surface::Sphere {
        center: Point3::new(0.1, -0.2, 0.3),
        radius: 2.0,
        axis: z,
        u_ref: frame(z),
    };
    let p = Point3::new(0.1, -0.2, 0.3) + unit(Vec3::new(0.3, 0.5, 0.8)) * 2.0;
    for m in [Vec3::new(0.3, -0.2, 1.0), Vec3::new(-0.7, 0.4, 0.5)] {
        out.push(("plane through a sphere", sphere.clone(), plane(p, m), p));
    }
    // A cylinder through a cylinder: the second's axis tilted, its
    // surface through p.
    let cyl = Surface::Cylinder {
        origin: Point3::origin(),
        axis: z,
        radius: 1.0,
        u_ref: frame(z),
    };
    let p = Point3::new(0.6, 0.8, 0.3);
    let axis = unit(Vec3::new(1.0, 0.2, 0.3));
    let off = unit(Vec3::new(0.2, -0.9, 0.4));
    let radial = unit(off - axis * off.dot(axis));
    let other = Surface::Cylinder {
        origin: p - radial * 0.7,
        axis,
        radius: 0.7,
        u_ref: frame(axis),
    };
    out.push(("cylinder through a cylinder", cyl.clone(), other.clone(), p));
    out.push(("cylinder through a cylinder, swapped", other, cyl, p));
    out
}

#[test]
fn a_sections_bend_in_a_curved_face_is_its_traced_acceleration() {
    let mut worst: f64 = 0.0;
    for (what, f, g, p) in poses() {
        assert!(implicit_residual(&f, p).abs() < 1e-12, "{what}: off f");
        assert!(implicit_residual(&g, p).abs() < 1e-12, "{what}: off g");
        let nf = unit(implicit_gradient(&f, p));
        let d = unit(nf.cross(implicit_gradient(&g, p)));
        for d in [d, -d] {
            for across in [nf.cross(d), d.cross(nf)] {
                let got = implicit_section_acceleration(&f, &g, p, d, across);
                let want = traced(&f, &g, p, d, across);
                let err = (got - want).abs() / want.abs().max(1.0);
                worst = worst.max(err);
                assert!(err < 1e-5, "{what}: formula {got}, traced {want}");
            }
        }
    }
    eprintln!("worst relative error {worst:e}");
}

#[test]
fn against_a_plane_it_is_the_graph_acceleration() {
    for (what, f, g, p) in poses() {
        if !matches!(f, Surface::Plane { .. }) {
            continue;
        }
        let nf = unit(implicit_gradient(&f, p));
        let d = unit(nf.cross(implicit_gradient(&g, p)));
        let across = nf.cross(d);
        let a = implicit_section_acceleration(&f, &g, p, d, across);
        let b = implicit_graph_acceleration(&g, p, d, across);
        assert!(
            (a - b).abs() <= 1e-12 * b.abs().max(1.0),
            "{what}: {a} vs {b}"
        );
    }
}

/// A plane at distance `h` from a sphere's centre cuts a circle of
/// radius `r = √(R² − h²)`, whose geodesic curvature on the sphere is
/// `h/(R r)`, toward the side of the plane's foot.
#[test]
fn a_plane_through_a_sphere_bends_at_the_small_circles_geodesic_curvature() {
    let z = Vec3::new(0.0, 0.0, 1.0);
    let big_r = 2.0;
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: big_r,
        axis: z,
        u_ref: frame(z),
    };
    for &h in &[0.0, 0.3, 1.0, 1.9, 1.999] {
        let r = (big_r * big_r - h * h).sqrt();
        let p = Point3::new(r, 0.0, h);
        let cut = plane(Point3::new(0.0, 0.0, h), z);
        let nf = Vec3::new(r, 0.0, h) / big_r;
        let d = Vec3::new(0.0, 1.0, 0.0);
        // Toward the circle's centre, in the sphere's tangent plane:
        // upward along the sphere, away from the equator.
        let across = nf.cross(d);
        let got = implicit_section_acceleration(&sphere, &cut, p, d, across);
        // The circle's curvature vector, toward its centre.
        let want = Vec3::new(-1.0 / r, 0.0, 0.0).dot(across);
        assert!(
            (got - want).abs() < 1e-12 * want.abs().max(1.0),
            "h = {h}: {got} vs {want}"
        );
    }
}

/// As the other surface comes tangent to the reference, `∇G·across →
/// 0`: the bend grows without bound, as the traced section's does (a
/// plane nearly tangent to a sphere cuts a small circle).
#[test]
fn a_nearly_tangent_section_bends_without_bound_and_a_tangent_one_is_unbounded() {
    let z = Vec3::new(0.0, 0.0, 1.0);
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: z,
        u_ref: frame(z),
    };
    let p = Point3::new(0.0, 0.0, 1.0);
    let d = Vec3::new(0.0, 1.0, 0.0);
    let across = Vec3::new(0.0, 0.0, 1.0).cross(d);
    for &tilt in &[1e-1_f64, 1e-3, 1e-6] {
        let m = Vec3::new(tilt.sin(), 0.0, tilt.cos());
        let g = plane(p, m);
        let got = implicit_section_acceleration(&sphere, &g, p, d, across);
        let want = traced(&sphere, &g, p, d, across);
        eprintln!("tilt {tilt:e}: {got:e} traced {want:e}");
        // The small circle's geodesic curvature is `cot(tilt)`; the
        // traced reading holds only where its step is short of the
        // circle.
        assert!(
            (got.abs() - 1.0 / tilt.tan()).abs() < 1e-9 * got.abs(),
            "{tilt}: {got}"
        );
        if tilt > 1e-2 {
            assert_eq!(got.signum(), want.signum(), "{tilt}: {got} vs {want}");
        }
    }
    let flat = plane(p, z);
    let got = implicit_section_acceleration(&sphere, &flat, p, d, across);
    assert!(!got.is_finite(), "tangent: {got}");
    let band = Band::linear(Tol::witness()).unwrap();
    assert!(tied_germ_turn(z, d, [0.0, got], 1.0, band).is_err());
}

/// Two germs along one tangent in a sphere: a plane's section and a
/// cylinder's, each traced, and `tied_germ_turn` reads which leaves
/// counterclockwise about the outward normal.
#[test]
fn tied_germs_in_a_sphere_turn_as_their_traced_sections_do() {
    let band = Band::linear(Tol::witness()).unwrap();
    let z = Vec3::new(0.0, 0.0, 1.0);
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.5,
        axis: z,
        u_ref: frame(z),
    };
    let p = Point3::new(0.0, 0.0, 1.5);
    let normal = z;
    let mut rows = 0;
    for &(ta, tb) in &[
        (0.2_f64, 0.5_f64),
        (0.5, 0.2),
        (-0.3, 0.4),
        (0.7, -0.7),
        (1.2, 1.1),
    ] {
        // Two planes through p, both containing the x direction, tilted
        // about x by ta and tb: their sections leave p along ±x.
        let g0 = plane(p, Vec3::new(0.0, ta.sin(), ta.cos()));
        let g1 = plane(p, Vec3::new(0.0, tb.sin(), tb.cos()));
        for t in [Vec3::new(1.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0)] {
            let across = normal.cross(t);
            let b0 = implicit_section_acceleration(&sphere, &g0, p, t, across);
            let b1 = implicit_section_acceleration(&sphere, &g1, p, t, across);
            let sign = tied_germ_turn(normal, t, [b0, b1], 0.1, band).unwrap();
            // Traced: the side of germ 1's point off germ 0's, about
            // the normal, a short way along.
            let s = 1e-2;
            let (q0, q1) = (
                on_section(&sphere, &g0, p, t, s),
                on_section(&sphere, &g1, p, t, s),
            );
            let ccw = t.cross(q1 - q0).dot(normal);
            let want = if ccw > 0.0 {
                Sign::Positive
            } else {
                Sign::Negative
            };
            assert_eq!(sign, want, "{ta}/{tb} along {t:?}: {b0} {b1}");
            rows += 1;
        }
    }
    assert_eq!(rows, 10);
}
