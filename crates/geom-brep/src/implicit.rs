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
//! complete locus, per `geom`'s surface conventions); its residual is
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

use geom::Surface;
use geom_core::{Point3, Real, Vec3};

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
            (d2 - radius.powi(2)) / (two * radius)
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let (_, w) = axial_radial(p, origin, axis);
            (w.norm_squared() - radius.powi(2)) / (two * radius)
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
            // d and h straddle zero at legitimate on-locus points (the
            // tube's top/bottom circle, the equatorial plane): tight
            // squares via powi(2), not d·d/h·h — bit-identical at f64
            // and the dual value channel, honest [0, hi] enclosures at
            // the interval scalar (the norm_squared rationale, M2 PR 3
            // fix pass). minor_radius is positive conventional data —
            // its square is tight either way; powi(2) keeps the
            // square-discipline tripwire's scope clean.
            (d.powi(2) + h.powi(2) - minor_radius.powi(2)) / (two * minor_radius)
        }
        // STAYS poison after M5 PR 3 gave the variant a payload: a NURBS
        // carrier has no implicit form — foot-point machinery (C2.1,
        // M5 PR 4) owns that story, not this module.
        Surface::Nurbs(_) => poison(),
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
        // STAYS poison after M5 PR 3 gave the variant a payload: a NURBS
        // carrier has no implicit form — foot-point machinery (C2.1,
        // M5 PR 4) owns that story, not this module.
        Surface::Nurbs(_) => poison_vec(),
    }
}

/// The local curvature lever arm of `s` at `p` (module docs): the
/// smallest local radius of curvature, `f64::MAX` for a plane (the
/// practical `min` identity — see the module docs for why not `+∞`),
/// poison for [`Surface::Nurbs`].
///
/// Per kind: sphere/cylinder — the radius; cone — the radial distance
/// ρ of `p` from the axis (a conservative bound on the osculating
/// radius ρ/cos α; smaller arms escalate *more*, which is the safe
/// direction); torus — the minor radius (the tube's curvature
/// dominates a ring torus).
pub fn curvature_lever_arm<T: Real>(s: &Surface<T>, p: Point3<T>) -> T {
    match *s {
        Surface::Plane { .. } => T::from_f64(f64::MAX),
        Surface::Sphere { radius, .. } | Surface::Cylinder { radius, .. } => radius,
        Surface::Cone { apex, axis, .. } => {
            let (_, w) = axial_radial(p, apex, axis);
            w.norm()
        }
        Surface::Torus { minor_radius, .. } => minor_radius,
        // STAYS poison after M5 PR 3 gave the variant a payload: a NURBS
        // carrier has no implicit form — foot-point machinery (C2.1,
        // M5 PR 4) owns that story, not this module.
        Surface::Nurbs(_) => poison(),
    }
}

/// The quadratic form `dᵀ (∇²F) d` of [`implicit_residual`]'s Hessian
/// at `p`, along direction `d` (NOT normalized — the form is
/// homogeneous of degree 2 in `d`). With `d` a unit surface tangent,
/// `dᵀ∇²F d / |∇F|` is the surface's **normal curvature** along `d`
/// (signed against the outward implicit gradient) — the second-order
/// jet datum C7's tangency schedule and the second-order sector
/// trilean consume (M5 PR 9).
///
/// Derived per kind from the module-doc forms (fixed order, D9);
/// squares of possibly-zero components go through `powi(2)` (the
/// interval-square rule). Honest poison at surface singularities
/// (cone axis, torus axis) and for [`Surface::Nurbs`].
pub fn implicit_hessian_form<T: Real>(s: &Surface<T>, p: Point3<T>, d: Vec3<T>) -> T {
    match *s {
        // F = (p − o)·n̂: linear, Hessian 0.
        Surface::Plane { .. } => T::zero(),
        // F = (|q|² − r²)/2r: ∇²F = I/r.
        Surface::Sphere { radius, .. } => d.norm_squared() / radius,
        // F = (|w|² − r²)/2r, w = q − a(q·a): ∇²F = (I − aaᵀ)/r.
        Surface::Cylinder { axis, radius, .. } => {
            let d_ax = d.dot(axis);
            (d.norm_squared() - d_ax.powi(2)) / radius
        }
        // F = |w|·cos α − |h|·sin α: away from h = 0 the |h| term is
        // linear; ∇²(|w|) = (I − aaᵀ − ŵŵᵀ)/ρ. Poison on the axis
        // (ρ = 0), as the gradient already is.
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (_, c_a) = half_angle.sin_cos();
            let (_, w) = axial_radial(p, apex, axis);
            let rho = w.norm();
            let w_hat = w / rho;
            let d_ax = d.dot(axis);
            let d_w = d.dot(w_hat);
            (d.norm_squared() - d_ax.powi(2) - d_w.powi(2)) * c_a / rho
        }
        // F = ((ρ − R)² + h² − r²)/2r: ∇²F = ((I − aaᵀ − ŵŵᵀ)·(ρ − R)/ρ
        // + ŵŵᵀ + aaᵀ)/r. Poison on the axis (ρ = 0).
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (_, w) = axial_radial(p, center, axis);
            let rho = w.norm();
            let w_hat = w / rho;
            let d_ax = d.dot(axis);
            let d_w = d.dot(w_hat);
            let d_perp2 = d.norm_squared() - d_ax.powi(2) - d_w.powi(2);
            (d_perp2 * (rho - major_radius) / rho + d_w.powi(2) + d_ax.powi(2)) / minor_radius
        }
        Surface::Nurbs(_) => poison(),
    }
}

/// The **largest normal-curvature magnitude** of `s` at `p` over its
/// tangent plane (1/meters) — the direction-free second-order datum
/// the C12.2 tangent-contact descent classifies against (M5 PR 9):
/// zero iff the surface osculates its tangent plane (locally
/// plane-like — the under-determined case), definitely positive iff
/// it bends off it in SOME direction.
///
/// Branch-free (no basis choice — D9/equivariance): the Hessian is
/// assembled from six [`implicit_hessian_form`] evaluations by
/// polarization; the tangent-plane restriction's eigen extremum comes
/// from the two invariants `tr_r = tr H − n̂ᵀHn̂` and
/// `det_r = n̂ᵀ adj(H) n̂` (the adjugate identity), giving
/// `|λ|max = |tr_r/2| + √((tr_r/2)² − det_r)`, divided by `|∇F|`.
/// Poison in, poison out (singular points, `Nurbs`).
pub fn implicit_max_normal_curvature<T: Real>(s: &Surface<T>, p: Point3<T>) -> T {
    let g = implicit_gradient(s, p);
    let n_hat = g / g.norm();
    let ex = Vec3::new(T::one(), T::zero(), T::zero());
    let ey = Vec3::new(T::zero(), T::one(), T::zero());
    let ez = Vec3::new(T::zero(), T::zero(), T::one());
    let hxx = implicit_hessian_form(s, p, ex);
    let hyy = implicit_hessian_form(s, p, ey);
    let hzz = implicit_hessian_form(s, p, ez);
    let two = T::from_f64(2.0);
    let hxy = (implicit_hessian_form(s, p, ex + ey) - hxx - hyy) / two;
    let hyz = (implicit_hessian_form(s, p, ey + ez) - hyy - hzz) / two;
    let hxz = (implicit_hessian_form(s, p, ex + ez) - hxx - hzz) / two;
    // Restricted trace: tr H − n̂ᵀHn̂.
    let n_form = implicit_hessian_form(s, p, n_hat);
    let tr_r = hxx + hyy + hzz - n_form;
    // Restricted determinant: n̂ᵀ adj(H) n̂ (cofactors, fixed order).
    let adj_xx = hyy * hzz - hyz.powi(2);
    let adj_yy = hxx * hzz - hxz.powi(2);
    let adj_zz = hxx * hyy - hxy.powi(2);
    let adj_xy = hxz * hyz - hxy * hzz;
    let adj_yz = hxy * hxz - hyz * hxx;
    let adj_xz = hxy * hyz - hyy * hxz;
    let det_r = adj_xx * n_hat.x.powi(2)
        + adj_yy * n_hat.y.powi(2)
        + adj_zz * n_hat.z.powi(2)
        + two
            * (adj_xy * n_hat.x * n_hat.y
                + adj_yz * n_hat.y * n_hat.z
                + adj_xz * n_hat.x * n_hat.z);
    let half_tr = tr_r / two;
    // (tr/2)² − det ≥ 0 in ℝ (a real symmetric restriction); the
    // clamp guards rounding, powi keeps the interval square tight.
    let disc = (half_tr.powi(2) - det_r).max(T::zero());
    (half_tr.abs() + disc.sqrt()) / g.norm()
}

/// A conservative enclosure of [`implicit_residual`] over an ENTIRE
/// circle carrier `C(θ) = center + radius·(û·cosθ + v̂·sinθ)`,
/// `v̂ = axis × û` — the M6 door-A rider's algebra, shared with
/// `tangent.rs`'s circle arm: against a **sphere** the composed
/// squared distance is an EXACT first harmonic in θ; against a
/// **cylinder** the squared axis distance is a degree-≤2
/// trigonometric polynomial whose harmonic amplitudes bound its
/// range. Both enclose (sphere tightly, cylinder conservatively —
/// slack only ever widens the returned range, which sends more pairs
/// to the typed frontier, never fewer).
///
/// Returns `(lo, hi)` in METERS (the residual's own linearized
/// units), or `None` for kinds without the closed harmonic form
/// (cone, torus, NURBS) — the caller keeps its frontier door there.
/// Total arithmetic: poison in, poison out.
#[must_use]
pub fn circle_residual_extremes<T: Real>(
    s: &Surface<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
) -> Option<(T, T)> {
    let two = T::from_f64(2.0);
    let u = u_ref;
    let v = axis.cross(u_ref);
    let amp = |a: T, b: T| (a.powi(2) + b.powi(2)).sqrt();
    match *s {
        Surface::Plane { origin, normal, .. } => {
            let c0 = (center - origin).dot(normal);
            let a1 = radius * amp(u.dot(normal), v.dot(normal));
            Some((c0 - a1, c0 + a1))
        }
        Surface::Sphere {
            center: sc,
            radius: r,
            ..
        } => {
            // |C(θ) − sc|² = |e|² + R_c² + 2R_c(e·û cosθ + e·v̂ sinθ):
            // û ⊥ v̂ unit makes the θ-dependence a pure first
            // harmonic, so the range below is EXACT.
            let e = center - sc;
            let c0 = e.norm_squared() + radius.powi(2);
            let a1 = two * radius * amp(e.dot(u), e.dot(v));
            Some((
                (c0 - a1 - r.powi(2)) / (two * r),
                (c0 + a1 - r.powi(2)) / (two * r),
            ))
        }
        Surface::Cylinder {
            origin,
            axis: a,
            radius: r,
            ..
        } => {
            // The radial part w(θ) = perp(e) + R_c(perp(û)cosθ +
            // perp(v̂)sinθ) has |w|² of trigonometric degree ≤ 2; its
            // constant term and harmonic amplitudes are exact, and
            // |A₁ cos + B₁ sin| + |second harmonic| bounds the swing.
            // Named binding so the interval-square tripwire's grep does
            // not false-positive on `a * a.dot(x)` (vector × projection
            // coefficient, not a scalar square) — the blend.rs precedent.
            let perp = |x: Vec3<T>| {
                let along = a.dot(x);
                x - a * along
            };
            let e = perp(center - origin);
            let up = perp(u);
            let vp = perp(v);
            let c0 =
                e.norm_squared() + radius.powi(2) * (up.norm_squared() + vp.norm_squared()) / two;
            let a1 = two * radius * amp(e.dot(up), e.dot(vp));
            let a2 =
                radius.powi(2) * amp((up.norm_squared() - vp.norm_squared()) / two, up.dot(vp));
            Some((
                (c0 - a1 - a2 - r.powi(2)) / (two * r),
                (c0 + a1 + a2 - r.powi(2)) / (two * r),
            ))
        }
        Surface::Cone { .. } | Surface::Torus { .. } | Surface::Nurbs(_) => None,
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
        // Nurbs: no implicit/seam form (C2.1 foot points, M5 PR 4).
        Surface::Plane { .. } | Surface::Nurbs(_) => return None,
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
    fn plane_arm_is_max_finite_and_nurbs_poisons() {
        let plane: Surface<f64> = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        // The practical min identity — finite so the interval lane's
        // enclosure stays well-formed (module docs).
        assert_eq!(curvature_lever_arm(&plane, Point3::origin()), f64::MAX);
        let n: Surface<f64> = Surface::nurbs_placeholder();
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

    /// The M6 rider's algebra: the returned range ENCLOSES every
    /// sampled residual over the circle (all three closed-form
    /// kinds), and the sphere arm — an exact first harmonic — is
    /// TIGHT: dense sampling attains both ends to rounding.
    #[test]
    fn circle_residual_extremes_enclose_and_the_sphere_arm_is_tight() {
        let center = Point3::new(0.4, -0.2, 0.7);
        let axis = Vec3::new(1.0, 2.0, 2.0).normalize();
        let u_ref = axis.cross(Vec3::unit_z()).normalize();
        let radius = 0.8;
        let eval = |t: f64| {
            let v = axis.cross(u_ref);
            center + (u_ref * t.cos() + v * t.sin()) * radius
        };
        let kinds: Vec<Surface<f64>> = vec![
            Surface::Plane {
                origin: Point3::new(0.1, 0.0, 0.0),
                normal: Vec3::new(0.2, -1.0, 0.4).normalize(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Sphere {
                center: Point3::new(1.5, 0.3, -0.2),
                radius: 0.9,
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Cylinder {
                origin: Point3::new(-0.5, 1.0, 0.2),
                axis: Vec3::new(0.3, 0.1, 1.0).normalize(),
                radius: 0.6,
                u_ref: Vec3::unit_x(),
            },
        ];
        for s in &kinds {
            let (lo, hi) =
                circle_residual_extremes(s, center, axis, radius, u_ref).expect("closed form");
            let mut seen_lo = f64::INFINITY;
            let mut seen_hi = f64::NEG_INFINITY;
            for i in 0..4096 {
                let t = core::f64::consts::TAU * f64::from(i) / 4096.0;
                let r = implicit_residual(s, eval(t));
                assert!(
                    lo - 1e-12 <= r && r <= hi + 1e-12,
                    "sample {r} escapes [{lo}, {hi}] on {s:?}"
                );
                seen_lo = seen_lo.min(r);
                seen_hi = seen_hi.max(r);
            }
            if matches!(s, Surface::Sphere { .. } | Surface::Plane { .. }) {
                // First-harmonic arms are EXACT: sampling attains the
                // bounds to discretization error.
                assert!((seen_lo - lo).abs() < 1e-5 && (seen_hi - hi).abs() < 1e-5);
            }
        }
        // No closed form for cone/torus/NURBS: the caller keeps its
        // frontier door.
        let torus = Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: 2.0,
            minor_radius: 0.5,
            u_ref: Vec3::unit_x(),
        };
        assert!(circle_residual_extremes(&torus, center, axis, radius, u_ref).is_none());
    }
}
