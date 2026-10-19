//! **A carrier's own resolution over a window of its parameter**: a
//! bound on how far a point it evaluates there can lie from the locus
//! its construction meant, read relative to the window, never to the
//! world origin.
//!
//! The coordinates' own rounding at the model's extent, `u` times a
//! point's distance from the origin, is not charged: every margin in
//! the kernel carries it, and it is what the band's noise headroom `K`
//! presumes (DESIGN.md Q1, "a few ulp at the model's extent"). What is
//! charged is what a carrier's stored form amplifies past that: its
//! points are summed from terms as long as its semi-axes, so an
//! ellipse whose centre lies `A` off the arc it serves (a plane `θ` off
//! a cylinder's axis stores one about `r/θ` away) resolves the arc only
//! to about `u·A`, however close to the origin the arc lies. Read so,
//! the resolution is the same wherever the part sits.

use geom::{Curve3, CurveKind};
use geom_core::SpanLocate;
use geom_core::running::UNIT_ROUNDOFF;

use geom::carrier_speed::speed_hi;

/// The roundings [`resolution`] charges per unit of its scale, `u`
/// times the length of the terms a point of the carrier is summed from:
///
/// - **evaluation**, at most 14: per coordinate, `sin_cos` (1), the
///   computed `v_ref = axis × u_ref` (2), the two products and the
///   frame scaling (3) and the two sums (1), at most 7 per term; the
///   point's Euclidean miss is at most `√3 < 2` times that;
/// - **construction**, fitted: the section tables build a carrier from
///   its two surfaces, and its points hold only to some multiple of `u`
///   times the length of those terms. No closed form is derived for
///   it. `the_resolution_bounds_each_section_tables_carrier` holds the
///   whole allowance (this bound plus the coordinates' `4ε·|p|`) at ten
///   times the exact miss on every kind the tables mint a conic or a line
///   for. At 32 the worst miss is `0.080` of it (a sphere's circle; a
///   cylinder's ellipses, far centres included, `0.023`); at 16 the
///   sphere's reaches `0.119` and the row reds, so 32 is the smallest
///   power of two that keeps that tenfold margin, not a derived count.
const RESOLUTION_ROUNDINGS: f64 = 32.0;

/// **`carrier`'s resolution over `t0 ..= t1`** (metres): the
/// [`RESOLUTION_ROUNDINGS`] roundings of the terms its points are
/// summed from — a conic's semi-axes, twice its [`speed_hi`] at most; a
/// line's reach `|dir|·τ` — and one of the parameter, `u·τ` times
/// [`speed_hi`], `τ` the window's larger end in magnitude. `None` for a
/// kind with no honest bound here: a spiric, whose radial root loses
/// its digits where the oval runs flat across the trace, and a spline,
/// whose fit is certified at its own tolerance.
pub fn resolution<T: SpanLocate>(carrier: &Curve3<T>, t0: T, t1: T) -> Option<T> {
    let speed = speed_hi(carrier)?;
    let tau = t0.abs().max(t1.abs());
    // `speed_hi` answers a line and the conics alone.
    let reach = if carrier.kind() == CurveKind::Line {
        tau
    } else {
        T::from_f64(2.0)
    };
    Some(T::from_f64(UNIT_ROUNDOFF) * speed * (T::from_f64(RESOLUTION_ROUNDINGS) * reach + tau))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom::Surface;
    use geom_core::{Band, Point3, Vec3};

    use super::*;
    use crate::Reach;
    use crate::exact_q::Q;
    use crate::intersect::{
        PlaneConeSection, PlaneCylinderSection, PlaneSphereSection, PlaneTorusSection,
        plane_cone_section, plane_cylinder_section, plane_sphere_section, plane_torus_section,
    };

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    fn q3(v: Vec3<f64>) -> [Q; 3] {
        [Q::of(v.x), Q::of(v.y), Q::of(v.z)]
    }

    fn dot(a: &[Q; 3], b: &[Q; 3]) -> Q {
        a[0].mul(&b[0]).add(&a[1].mul(&b[1])).add(&a[2].mul(&b[2]))
    }

    /// `|p − o|²` and `((p − o)·a)²/|a|²` exactly: the squared distance
    /// from `o` and the squared height along `a`.
    fn split(p: Point3<f64>, o: Point3<f64>, a: Vec3<f64>) -> (Q, Q) {
        let (d, a) = (q3(p - o), q3(a));
        let h = dot(&d, &a);
        (dot(&d, &d), h.mul(&h).div(&dot(&a, &a)))
    }

    /// The point's exact residual against `s` and the rate that turns
    /// it into a distance to first order, read in `f64` (a cofactor, so
    /// its own rounding moves the distance by a relative `u`).
    fn residual(s: &Surface<f64>, p: Point3<f64>) -> (Q, f64) {
        match *s {
            Surface::Plane { origin, normal, .. } => {
                (dot(&q3(p - origin), &q3(normal)), normal.norm())
            }
            Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            } => {
                let (dd, hh) = split(p, origin, axis);
                let r = Q::of(radius);
                let perp = (p - origin).cross(axis).norm() / axis.norm();
                (dd.sub(&hh).sub(&r.mul(&r)), perp + radius)
            }
            Surface::Sphere { center, radius, .. } => {
                let (dd, _) = split(p, center, Vec3::unit_z());
                let r = Q::of(radius);
                (dd.sub(&r.mul(&r)), (p - center).norm() + radius)
            }
            Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            } => {
                // ρ⊥²·cos²α − h²·sin²α = (ρ⊥ cos α − h sin α)(ρ⊥ cos α + h sin α),
                // its first factor the distance to the nappe.
                let (dd, hh) = split(p, apex, axis);
                let (s, c) = half_angle.sin_cos();
                let (cc, ss) = (Q::of(c).mul(&Q::of(c)), Q::of(s).mul(&Q::of(s)));
                let perp = (p - apex).cross(axis).norm() / axis.norm();
                let h = (p - apex).dot(axis) / axis.norm();
                (
                    dd.sub(&hh).mul(&cc).sub(&hh.mul(&ss)),
                    perp * c + h.abs() * s,
                )
            }
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                ..
            } => {
                // (|d|² + R² − r²)² − 4R²ρ⊥² = (D² − r²)(D′² − r²), D and
                // D′ the distances to the tube's near and far centre points.
                let (dd, hh) = split(p, center, axis);
                let (rr, big) = (Q::of(minor_radius), Q::of(major_radius));
                let (r2, big2) = (rr.mul(&rr), big.mul(&big));
                let s = dd.add(&big2).sub(&r2);
                let four = Q::of(4.0);
                let f = s.mul(&s).sub(&four.mul(&big2).mul(&dd.sub(&hh)));
                let d = p - center;
                let h = d.dot(axis) / axis.norm();
                let perp = d.cross(axis).norm() / axis.norm();
                let near = ((perp - major_radius).powi(2) + h * h).sqrt();
                let far2 = (perp + major_radius).powi(2) + h * h;
                (f, (near + minor_radius) * (far2 - minor_radius.powi(2)))
            }
            _ => unreachable!("the rows read only the analytic kinds"),
        }
    }

    /// The point at `t` on `carrier`, held off both `surfaces` to its
    /// resolution plus the coordinates' own rounding at the point's
    /// distance from the origin (`4ε·|p|`, two roundings of each of
    /// three coordinates, which the bound leaves to the band), and the
    /// worst ratio of miss to that allowance.
    fn held(carrier: &Curve3<f64>, t: f64, surfaces: [&Surface<f64>; 2]) -> f64 {
        let p = carrier.eval(t);
        let bound =
            resolution(carrier, t, t).unwrap() + 4.0 * f64::EPSILON * (p - Point3::origin()).norm();
        let mut worst = 0.0_f64;
        for s in surfaces {
            let (q, rate) = residual(s, p);
            let within = |b: f64| {
                let edge = 0.99 * b * rate;
                q.within(-edge, edge)
            };
            assert!(
                within(bound),
                "{carrier:?} at {t} misses {s:?} past its allowance {bound:e}"
            );
            // The least fraction of the allowance the miss lies within,
            // bisected to a 2⁻²⁰ of it.
            let (mut lo, mut hi) = (0.0, 1.0);
            for _ in 0..20 {
                let mid = 0.5 * (lo + hi);
                if within(bound * mid) {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            worst = worst.max(hi);
        }
        worst
    }

    /// Each carrier at parameters about its nearest principal vertex,
    /// as the join seams a chord's, and across a wide arc.
    fn sweep(carrier: &Curve3<f64>, surfaces: [&Surface<f64>; 2]) -> f64 {
        let mut worst = 0.0_f64;
        for theta in [0.0, 0.4, 1.3, 2.2, 3.0, -1.0] {
            let seamed = carrier
                .seamed_near(theta)
                .unwrap_or_else(|| carrier.clone());
            for dt in [-0.5, -0.05, -1e-4, 0.0, 1e-4, 0.05, 0.5] {
                worst = worst.max(held(&seamed, dt, surfaces));
            }
        }
        worst
    }

    fn line_sweep(line: &Curve3<f64>, surfaces: [&Surface<f64>; 2]) -> f64 {
        [-3.0, -0.7, 0.0, 0.2, 1.5]
            .into_iter()
            .map(|t| held(line, t, surfaces))
            .fold(0.0, f64::max)
    }

    fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
        let n = normal.normalize();
        let u = if n.x.abs() < 0.9 {
            Vec3::unit_x()
        } else {
            Vec3::unit_y()
        };
        Surface::Plane {
            origin,
            normal: n,
            u_ref: (u - n * u.dot(n)).normalize(),
        }
    }

    fn tilted(t: f64, phi: f64) -> Vec3<f64> {
        Vec3::new(phi.cos(), phi.sin(), t)
    }

    /// **The resolution bounds every carrier the section tables mint**,
    /// against the exact miss of its evaluated points off both surfaces
    /// it is cut from (exact rationals on the stored data; a cone's
    /// `cos α`, `sin α` as `libm` rounds them). The poses lie about the
    /// origin, so the coordinates' rounding at the model's extent, which
    /// the bound leaves to the band (module docs), is no larger than the
    /// carrier's own. Planes from square to `1e-6` off a cylinder's axis
    /// and off-axis (centres to `1e6` away), a frustum's ellipses toward
    /// its parabola and its axis-normal circle, a sphere's small and
    /// great circles, a torus's meridian and concentric circles, and the
    /// rulings of a cylinder and a cone. Each miss must lie under a tenth
    /// of its allowance, so the constant halved reds the row.
    #[test]
    fn the_resolution_bounds_each_section_tables_carrier() {
        let mut worst = Vec::new();
        let cyl = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let reach = |at| Reach::Measured { at, lever: 1.0 };
        let mut w = 0.0_f64;
        for t in [1.0, 0.3, 1e-2, 1e-4, 1e-6] {
            for (off, phi) in [(0.0, 0.4), (0.37, 1.9), (0.9, 5.1)] {
                let n = tilted(t, phi);
                let cut = plane(Point3::new(off * phi.cos(), off * phi.sin(), 0.5), n);
                match plane_cylinder_section(&cut, &cyl, &reach(Point3::origin()), band()).unwrap()
                {
                    PlaneCylinderSection::TiltedEllipse(c) | PlaneCylinderSection::Rim(c) => {
                        w = w.max(sweep(&c, [&cut, &cyl]));
                    }
                    other => panic!("t {t}: {other:?}"),
                }
            }
        }
        let rim = plane(Point3::new(0.0, 0.0, 0.3), Vec3::unit_z());
        match plane_cylinder_section(&rim, &cyl, &reach(Point3::origin()), band()).unwrap() {
            PlaneCylinderSection::Rim(c) => w = w.max(sweep(&c, [&rim, &cyl])),
            other => panic!("square: {other:?}"),
        }
        let along = plane(Point3::new(0.3, 0.0, 0.0), Vec3::new(1.0, 0.2, 0.0));
        match plane_cylinder_section(&along, &cyl, &reach(Point3::origin()), band()).unwrap() {
            PlaneCylinderSection::ParallelLines { l1, l2 } => {
                worst.push((
                    "line",
                    line_sweep(&l1, [&along, &cyl]).max(line_sweep(&l2, [&along, &cyl])),
                ));
            }
            other => panic!("along: {other:?}"),
        }
        worst.push(("cylinder", w));

        let cone = Surface::Cone {
            apex: Point3::new(0.0, 0.0, 2.0),
            axis: Vec3::new(0.0, 0.0, -1.0),
            half_angle: 0.5_f64.atan(),
            u_ref: Vec3::unit_x(),
        };
        let mut w = 0.0_f64;
        let parabolic = core::f64::consts::FRAC_PI_2 - 0.5_f64.atan();
        for f in [0.0, 0.3, 0.9, 0.999, 0.99999] {
            for phi in [0.0_f64, 2.3] {
                let tilt = f * parabolic;
                let n = Vec3::new(tilt.sin() * phi.cos(), tilt.sin() * phi.sin(), tilt.cos());
                let cut = plane(Point3::new(0.0, 0.0, 0.6), n);
                let got =
                    plane_cone_section(&cut, &cone, &reach(Point3::new(0.0, 0.0, 0.6)), band());
                match got.unwrap() {
                    PlaneConeSection::TiltedEllipse(c) | PlaneConeSection::AxisNormalCircle(c) => {
                        w = w.max(sweep(&c, [&cut, &cone]));
                    }
                    other => panic!("f {f}: {other:?}"),
                }
            }
        }
        let through = plane(Point3::new(0.0, 0.0, 2.0), Vec3::new(1.0, 0.3, 0.0));
        match plane_cone_section(&through, &cone, &reach(Point3::new(0.0, 0.0, 1.0)), band())
            .unwrap()
        {
            PlaneConeSection::ApexLinePair { l1, l2 } => {
                let l = line_sweep(&l1, [&through, &cone]).max(line_sweep(&l2, [&through, &cone]));
                let prior = worst[0].1;
                worst[0].1 = prior.max(l);
            }
            other => panic!("through the apex: {other:?}"),
        }
        worst.push(("cone", w));

        let ball = Surface::Sphere {
            center: Point3::new(0.2, -0.1, 0.3),
            radius: 1.3,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let mut w = 0.0_f64;
        for (d, n) in [
            (0.0, Vec3::new(0.0, 0.0, 1.0)),
            (0.7, Vec3::new(1.0, 2.0, 2.0)),
            (1.25, Vec3::new(-0.3, 0.1, 1.0)),
        ] {
            let n = n.normalize();
            let cut = plane(Point3::new(0.2, -0.1, 0.3) + n * d, n);
            match plane_sphere_section(&cut, &ball, band()).unwrap() {
                PlaneSphereSection::Circle(c) => w = w.max(sweep(&c, [&cut, &ball])),
                other => panic!("d {d}: {other:?}"),
            }
        }
        worst.push(("sphere", w));

        let torus = Surface::Torus {
            center: Point3::new(0.1, 0.2, -0.3),
            axis: Vec3::unit_y(),
            major_radius: 2.0,
            minor_radius: 0.5,
            u_ref: Vec3::unit_x(),
        };
        let mut w = 0.0_f64;
        for cut in [
            plane(Point3::new(0.1, 0.2, -0.3), Vec3::unit_z()),
            plane(Point3::new(0.1, 0.2, -0.3), Vec3::new(1.0, 0.0, 1.0)),
            plane(Point3::new(0.1, 0.2, -0.3), Vec3::unit_y()),
            plane(Point3::new(0.1, 0.5, -0.3), Vec3::unit_y()),
        ] {
            match plane_torus_section(&cut, &torus, 3.0, band()).unwrap() {
                PlaneTorusSection::MeridianCircles { c1, c2 }
                | PlaneTorusSection::ConcentricCircles { c1, c2 } => {
                    w = w.max(sweep(&c1, [&cut, &torus]).max(sweep(&c2, [&cut, &torus])));
                }
                other => panic!("{cut:?}: {other:?}"),
            }
        }
        worst.push(("torus", w));

        println!("worst miss over the bound, per kind: {worst:?}");
        for (kind, ratio) in worst {
            assert!(ratio < 0.1, "{kind}: a miss at {ratio} of the bound");
        }
    }

    /// **The resolution is the same wherever the part sits.** A carrier
    /// moved across the model reads the same bound, and a far centre is
    /// charged its distance from the arc.
    #[test]
    fn the_resolution_reads_the_window_not_the_origin() {
        let circle = |x0: f64| Curve3::Circle {
            center: Point3::new(x0, 0.0, 0.0),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let at_origin = resolution(&circle(0.0), 0.0, 0.1).unwrap();
        for x0 in [30.0, 1e3, 3e4] {
            assert_eq!(
                resolution(&circle(x0), 0.0, 0.1).unwrap(),
                at_origin,
                "x0 {x0}"
            );
        }
        let far = Curve3::Ellipse {
            center: Point3::new(-1e5, 0.0, 0.0),
            axis: Vec3::unit_z(),
            semi_u: 1e5,
            semi_v: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let rho = resolution(&far, -1e-3, 1e-3).unwrap();
        assert!(rho > 2e-10 && rho < 1e-9, "{rho:e}");
        let spiric = Curve3::Spiric {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
            major_radius: 2.0,
            minor_radius: 1.0,
            offset: 0.5,
        };
        assert_eq!(resolution(&spiric, 0.0, 1.0), None);
    }
}
