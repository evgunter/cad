//! Closed-form implicit residuals, gradients, and curvature lever arms
//! for the analytic surfaces — the evaluation substrate of D4 ¶2
//! certification and the dihedral predicate.
//!
//! Every analytic surface kind has a closed-form implicit function whose
//! zero set is the surface. This module exposes it in **dimensionally
//! honest, linearized** form (the M2 PR 1 review's contract): each
//! residual is a length in meters, agreeing with the true signed
//! distance to first order near the surface, so it classifies directly
//! against the run's linear ε — no squared-distance bands, no hidden
//! unit mismatches. The `(q² − r²)/2r` shape is used wherever the
//! natural form is a squared distance: it equals `(|q| − r)·(|q| + r)/2r
//! ≈ |q| − r` near the surface and is smooth through the center (no
//! `abs` kink for the Dual lane to trip on).
//!
//! **Gradients, not chart normals.** [`implicit_gradient`] is the
//! implicit function's spatial gradient — unit-magnitude on the surface,
//! defined off the chart entirely. This is the PR 1 reviewer's contract
//! honored structurally: nothing here ever calls `Surface::normal`, so
//! the cone-apex poison (a chart evaluation) is unreachable from the
//! certification layer. The gradient is still honestly poison exactly
//! where the *surface* is singular (the cone apex has no tangent plane;
//! the gradient's `w/ρ` normalization poisons on the axis) — that is the
//! correct answer, not a limitation.
//!
//! # Formulas (fixed evaluation orders, D9)
//!
//! With `q` the point relative to the surface's anchor, `h = q·axis` the
//! axial component, `w = q − axis·h` the radial component (computed as a
//! vector difference — never as `√(|q|² − h²)`, whose cancellation could
//! go negative and poison), and `ρ = |w|`:
//!
//! | kind | residual (meters) | gradient |
//! |---|---|---|
//! | plane | `(p − origin)·normal` | `normal` |
//! | sphere | `(\|p − c\|² − r²)/2r` | `(p − c)/r` |
//! | cylinder | `(ρ² − r²)/2r` | `w/r` |
//! | cone | `ρ·cos α − \|h\|·sin α` | `(w/ρ)·cos α − axis·copysign(sin α, h)` |
//! | torus | `((ρ − R)² + h² − r²)/2r` | `((ρ − R)·(w/ρ) + axis·h)/r` |
//!
//! The cone's `|h|` makes the residual vanish on **both** nappes (the
//! complete locus, per `geom-surfaces`' conventions); its residual is
//! the exact perpendicular distance to the generator line in the
//! meridian half-plane through the point. The `Nurbs` placeholder
//! yields poison throughout (representable ≠ implemented — the poison
//! fails certification loudly, D4 ¶2).
//!
//! # Curvature lever arms
//!
//! [`curvature_lever_arm`] is the local "feature scale" an angular
//! comparison at a point turns on (D4 ¶1: an angle means displacement
//! only through a lever arm): the smallest local radius of curvature of
//! the surface at the point. A plane has none — it contributes `+∞`,
//! which is the identity of the `min` lattice the dihedral predicate
//! folds arms with. The cone's arm is the radial distance ρ (→ 0 at the
//! apex: near the apex every feature is tiny and angular classification
//! honestly escalates).

use geom_core::{Point3, Real, Vec3};
use geom_surfaces::Surface;

/// The scalar's poison value (NaN at `f64`, NaI at the interval scalar).
fn poison<T: Real>() -> T {
    T::from_f64(f64::NAN)
}

/// The all-poison vector.
fn poison_vec<T: Real>() -> Vec3<T> {
    let nan = poison::<T>();
    Vec3::new(nan, nan, nan)
}

/// The axial/radial decomposition `(h, w)` of `p` relative to an anchor
/// point and unit axis: `q = p − anchor`, `h = q·axis`,
/// `w = q − axis·h`. Shared by every axisymmetric form below (fixed
/// order, D9).
fn axial_radial<T: Real>(p: Point3<T>, anchor: Point3<T>, axis: Vec3<T>) -> (T, Vec3<T>) {
    let q = p - anchor;
    let h = q.dot(axis);
    let w = q - axis * h;
    (h, w)
}

/// The linearized implicit residual of `p` against `s`, in meters (the
/// module-doc table). Zero on the surface; agrees with the signed
/// distance to first order near it. Total: poison in, poison out;
/// [`Surface::Nurbs`] yields poison.
pub fn implicit_residual<T: Real>(s: &Surface<T>, p: Point3<T>) -> T {
    let two = T::from_f64(2.0);
    match *s {
        Surface::Plane { origin, normal, .. } => (p - origin).dot(normal),
        Surface::Sphere { center, radius, .. } => {
            let d2 = (p - center).norm_squared();
            (d2 - radius * radius) / (two * radius)
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let (_, w) = axial_radial(p, origin, axis);
            (w.norm_squared() - radius * radius) / (two * radius)
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (s_a, c_a) = half_angle.sin_cos();
            let (h, w) = axial_radial(p, apex, axis);
            w.norm() * c_a - h.abs() * s_a
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (h, w) = axial_radial(p, center, axis);
            let rho = w.norm();
            let d = rho - major_radius;
            (d * d + h * h - minor_radius * minor_radius) / (two * minor_radius)
        }
        Surface::Nurbs => poison(),
    }
}

/// The spatial gradient of [`implicit_residual`] at `p` (the module-doc
/// table): unit-magnitude on the surface, so it is the surface's normal
/// direction there — computed entirely from the implicit form, never
/// from the chart (`Surface::normal` is deliberately unreachable from
/// this layer). Honest poison where the surface itself is singular
/// (cone apex / cone axis, torus axis) and for [`Surface::Nurbs`].
pub fn implicit_gradient<T: Real>(s: &Surface<T>, p: Point3<T>) -> Vec3<T> {
    match *s {
        Surface::Plane { normal, .. } => normal,
        Surface::Sphere { center, radius, .. } => (p - center) / radius,
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let (_, w) = axial_radial(p, origin, axis);
            w / radius
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (s_a, c_a) = half_angle.sin_cos();
            let (h, w) = axial_radial(p, apex, axis);
            // w/ρ poisons on the axis (0/0) — including the apex, where
            // no tangent plane exists (honest, per the module docs).
            let w_hat = w / w.norm();
            w_hat * c_a - axis * s_a.copysign(h)
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (h, w) = axial_radial(p, center, axis);
            let rho = w.norm();
            let w_hat = w / rho;
            (w_hat * (rho - major_radius) + axis * h) / minor_radius
        }
        Surface::Nurbs => poison_vec(),
    }
}

/// The local curvature lever arm of `s` at `p` (module docs): the
/// smallest local radius of curvature, `+∞` for a plane (the `min`
/// identity), poison for [`Surface::Nurbs`].
///
/// Per kind: sphere/cylinder — the radius; cone — the radial distance
/// ρ of `p` from the axis (a conservative bound on the osculating
/// radius ρ/cos α; smaller arms escalate *more*, which is the safe
/// direction); torus — the minor radius (the tube's curvature
/// dominates a ring torus).
pub fn curvature_lever_arm<T: Real>(s: &Surface<T>, p: Point3<T>) -> T {
    match *s {
        Surface::Plane { .. } => T::from_f64(f64::INFINITY),
        Surface::Sphere { radius, .. } | Surface::Cylinder { radius, .. } => radius,
        Surface::Cone { apex, axis, .. } => {
            let (_, w) = axial_radial(p, apex, axis);
            w.norm()
        }
        Surface::Torus { minor_radius, .. } => minor_radius,
        Surface::Nurbs => poison(),
    }
}

/// The seam frame of an axisymmetric surface: `(w, u_ref, v_ref)` with
/// `w` the radial component of `p` relative to the surface's own
/// anchor/axis and `v_ref = axis × u_ref` — the pieces the
/// [`crate::EdgeGeometry::Seam`] residuals are built from. `None` for
/// the plane (not periodic — a seam description on it is malformed) and
/// for [`Surface::Nurbs`] (unimplemented).
pub(crate) fn seam_frame<T: Real>(
    s: &Surface<T>,
    p: Point3<T>,
) -> Option<(Vec3<T>, Vec3<T>, Vec3<T>)> {
    let (anchor, axis, u_ref) = match *s {
        Surface::Plane { .. } | Surface::Nurbs => return None,
        Surface::Cylinder {
            origin,
            axis,
            u_ref,
            ..
        } => (origin, axis, u_ref),
        Surface::Cone {
            apex, axis, u_ref, ..
        } => (apex, axis, u_ref),
        Surface::Sphere {
            center,
            axis,
            u_ref,
            ..
        } => (center, axis, u_ref),
        Surface::Torus {
            center,
            axis,
            u_ref,
            ..
        } => (center, axis, u_ref),
    };
    let (_, w) = axial_radial(p, anchor, axis);
    Some((w, u_ref, axis.cross(u_ref)))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use core::f64::consts::FRAC_PI_6;

    use proptest::prelude::*;

    use super::*;

    /// The exactly orthonormal tilted frame from PR 1's fixtures
    /// (integer Pythagorean triple over 3).
    fn t_axis() -> Vec3<f64> {
        Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0)
    }

    fn t_uref() -> Vec3<f64> {
        Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0)
    }

    fn t_center() -> Point3<f64> {
        Point3::new(-0.5, 4.0, 1.25)
    }

    fn all_curved() -> Vec<Surface<f64>> {
        vec![
            Surface::Cylinder {
                origin: t_center(),
                axis: t_axis(),
                radius: 2.5,
                u_ref: t_uref(),
            },
            Surface::Cone {
                apex: t_center(),
                axis: t_axis(),
                half_angle: FRAC_PI_6,
                u_ref: t_uref(),
            },
            Surface::Sphere {
                center: t_center(),
                radius: 2.5,
                axis: t_axis(),
                u_ref: t_uref(),
            },
            Surface::Torus {
                center: t_center(),
                axis: t_axis(),
                major_radius: 3.0,
                minor_radius: 1.25,
                u_ref: t_uref(),
            },
        ]
    }

    proptest! {
        /// On-surface points have residual ~0 and unit-magnitude
        /// gradient, for every kind, at chart-generated samples (away
        /// from chart singularities).
        #[test]
        fn residual_zero_and_gradient_unit_on_surface(
            u in -3.0..3.0f64,
            v in 0.25..1.5f64,
        ) {
            for s in all_curved() {
                let p = s.eval(u, v);
                let r = implicit_residual(&s, p);
                prop_assert!(r.abs() <= 1e-12, "{s:?}: residual {r}");
                let g = implicit_gradient(&s, p);
                prop_assert!((g.norm() - 1.0).abs() <= 1e-12, "{s:?}: |grad| {}", g.norm());
            }
            let plane = Surface::Plane { origin: t_center(), normal: t_axis(), u_ref: t_uref() };
            let p = plane.eval(u, v);
            prop_assert!(implicit_residual(&plane, p).abs() <= 1e-12);
        }

        /// The residual agrees with the true signed distance to first
        /// order: stepping δ along the gradient from an on-surface point
        /// changes the residual by ≈ δ.
        #[test]
        fn residual_is_first_order_distance(
            u in -3.0..3.0f64,
            v in 0.5..1.5f64,
            delta in -1e-4..1e-4f64,
        ) {
            for s in all_curved() {
                let p0 = s.eval(u, v);
                let g = implicit_gradient(&s, p0);
                let p = p0 + g * delta;
                let r = implicit_residual(&s, p);
                prop_assert!(
                    (r - delta).abs() <= 1e-7 * (1.0 + delta.abs()),
                    "{s:?}: residual {r} vs step {delta}"
                );
            }
        }

        /// The implicit gradient matches the chart normal (up to sign
        /// conventions they are the same direction) at regular points.
        #[test]
        fn gradient_matches_chart_normal(u in -3.0..3.0f64, v in 0.5..1.5f64) {
            for s in all_curved() {
                let p = s.eval(u, v);
                let g = implicit_gradient(&s, p);
                let n = s.normal(u, v);
                // Both unit; chart orientation for these variants is the
                // outward one, matching the gradient of "inside < 0".
                prop_assert!(g.cross(n).norm() <= 1e-10, "{s:?}");
                prop_assert!(g.dot(n) > 0.0, "{s:?}: sign flip");
            }
        }
    }

    #[test]
    fn cone_residual_covers_both_nappes_and_apex() {
        let cone = Surface::Cone {
            apex: t_center(),
            axis: t_axis(),
            half_angle: FRAC_PI_6,
            u_ref: t_uref(),
        };
        // Mirror nappe (v < 0) is on the locus too.
        let p = cone.eval(1.0, -0.75);
        assert!(implicit_residual(&cone, p).abs() <= 1e-13);
        // The apex is on the locus (residual 0, no poison)...
        assert_eq!(implicit_residual(&cone, t_center()), 0.0);
        // ...but has no tangent plane: the gradient is honest poison.
        let g = implicit_gradient(&cone, t_center());
        assert!(g.x.is_nan() && g.y.is_nan() && g.z.is_nan());
        // And the curvature arm collapses to zero at the apex.
        assert_eq!(curvature_lever_arm(&cone, t_center()), 0.0);
    }

    #[test]
    fn plane_arm_is_infinite_and_nurbs_poisons() {
        let plane: Surface<f64> = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        assert_eq!(curvature_lever_arm(&plane, Point3::origin()), f64::INFINITY);
        let n: Surface<f64> = Surface::Nurbs;
        assert!(implicit_residual(&n, Point3::origin()).is_nan());
        assert!(implicit_gradient(&n, Point3::origin()).x.is_nan());
        assert!(curvature_lever_arm(&n, Point3::origin()).is_nan());
    }

    #[test]
    fn off_surface_residuals_report_metric_distance() {
        // A unit-ish sphere: a point 0.1 outside reports ≈ +0.1 (the
        // linearized form is (d² − r²)/2r = (d − r)(d + r)/2r).
        let s = Surface::Sphere {
            center: Point3::origin(),
            radius: 2.0,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let p = Point3::new(2.1, 0.0, 0.0);
        let r = implicit_residual(&s, p);
        assert!((r - 0.1025).abs() < 1e-12); // (2.1² − 4)/4 exactly
        // Inside is negative.
        assert!(implicit_residual(&s, Point3::new(1.9, 0.0, 0.0)) < 0.0);
    }
}
