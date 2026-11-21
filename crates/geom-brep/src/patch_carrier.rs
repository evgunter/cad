//! **A NURBS patch's certified distance from an analytic surface** — the
//! patch-side twin of the curve × face composite arm
//! ([`crate::curve_face::composite`]): one number, an upper bound on how
//! far any point of the WHOLE patch lies from the surface's locus, read
//! off coefficients with no evaluation and no sampling.
//!
//! # What it bounds, and why that is whole-patch
//!
//! The patch's control net is written in the surface's chart frame in
//! certification arithmetic (the frame orthonormalized from the stored
//! axis and reference direction, [`crate::curve_face::composite`]'s
//! `framed_form`), and the surface's canonical form `f` (plane `z`,
//! cylinder `x² + y² − R²`, sphere `|ξ|² − r²`, double cone
//! `cos²α·(x² + y²) − sin²α·z²`, torus quartic) is composed with the
//! patch as a tensor-product rational Bernstein form `f ∘ S = N/D`, `D` a
//! power of the weight function
//! ([`geom_core::spline::compose::patch::canonical_patch_composite`]).
//! The patch's knot-span cells cover its domain, and on each cell:
//!
//! - `|f|` is at most the largest `|N_k/D_k|`: `N` and `D` have one
//!   bidegree, so with every `D_k > 0` (certified, or the cell refuses)
//!   `N/D = Σ (B_k·D_k/D)·(N_k/D_k)` is a convex combination of the
//!   coefficient ratios
//!   ([`geom_core::spline::compose::patch::PatchComposite::cell_bound`]);
//! - the cell's points lie in the convex hull of its rational Bézier
//!   control points `(X_k, Y_k, Z_k)/W_k` (positive weights make the
//!   rational basis a nonnegative partition of unity), so the box and
//!   the chord support of those points bound the cell's axial distance
//!   `ρ` from below and above and its height `z`;
//! - the chart's conversion from `|f|` to metres
//!   ([`crate::pcurve_cache::projected`]'s `Meters`, the one statement of
//!   it, read here as the curve arms read it) turns those three into an
//!   upper bound on the distance of every point of the cell from the
//!   locus: `|f|/(ρ + R)` on a cylinder and its sphere twin, on a cone
//!   the lesser of `|f|` over the generator lever `ρ cos α + z sin α`
//!   (where it decides positive) and `√|f|`, and `|f|` over the core-circle and
//!   `(ρ + R)² − r²` levers on a torus. Each is an inequality at every
//!   point of the cell, never a linearization.
//!
//! The largest cell bound is the patch's. Every operation is interval
//! arithmetic with outward rounding, and the final bound is the upper end
//! of an enclosure, so it is rounded up: a bound read here holds for the
//! real patch the `f64` net names, against the real surface the stored
//! fields name.
//!
//! The cone's locus is the DOUBLE cone ([`geom::Surface::Cone`]'s
//! complete locus), so both of its readings bound the distance and the
//! lesser is taken ([`ConeLocus::Double`]): the opening nappe's lever away
//! from the apex, the apex root near it, where `|f|` is small and the
//! lever is not. A patch on the other nappe reads the apex root alone.
//!
//! # Refusal
//!
//! `NaN` — which fails every `≤ ε` comparison (D4 ¶2) — off an analytic
//! chart, on a spindle sheet, for a net or a frame that does not
//! certify, and for a cell whose bound does not (a weight coefficient not
//! certified positive, a lever not certified positive on a torus).

use geom::{NurbsSurface, Surface};
use geom_core::interval::certification::Certification as _;
use geom_core::interval::max_bound;
use geom_core::spline::compose::patch::{PatchSpans, canonical_patch_composite};
use geom_core::spline::compose::tensor::SurfaceCertData;
use geom_core::{Bounds, Interval};

use crate::curve_face::composite::framed_form;
use crate::pcurve_cache::projected::{ConeLocus, part_reach};

/// A certified upper bound in metres on the distance of every point of
/// `patch` from `surface`'s locus (module docs); `NaN` where none is
/// certified.
#[must_use]
pub fn patch_distance_sup(patch: &NurbsSurface<f64>, surface: &Surface<f64>) -> f64 {
    distance_sup_on(patch, surface, ConeLocus::Double)
}

/// [`patch_distance_sup`] with a cone's metres read against `locus`.
fn distance_sup_on(patch: &NurbsSurface<f64>, surface: &Surface<f64>, locus: ConeLocus) -> f64 {
    let point = |x: f64| Interval::from_bounds(x, x);
    let Ok((coords, form, meters)) = framed_form(surface, patch.control(), &point) else {
        return f64::NAN;
    };
    let Ok(data) = SurfaceCertData::new(
        patch.knots_u_f64_interim(),
        patch.knots_v_f64_interim(),
        patch.weights_f64_interim(),
        &coords,
    ) else {
        return f64::NAN;
    };
    let (points, weight) = data.bernstein_channels();
    let composite = canonical_patch_composite(&points, &weight, &form);
    let (nu, nv) = composite.num.cell_counts();
    if nu == 0 || nv == 0 {
        return f64::NAN;
    }
    let upper = |x: Interval| if x.is_certified() { x.hi() } else { f64::NAN };
    let lower = |x: Interval| if x.is_certified() { x.lo() } else { f64::NAN };
    let mut worst = 0.0_f64;
    // Fixed ascending cell order (D9).
    for su in 0..nu {
        for sv in 0..nv {
            let f = composite.cell_bound(su, sv);
            let Some(controls) = PatchSpans::rational_cell_controls(
                [&points[0], &points[1], &points[2]],
                &weight,
                su,
                sv,
            ) else {
                return f64::NAN;
            };
            if !f.is_certified() || controls.is_empty() {
                return f64::NAN;
            }
            let f_sup = f.mag();
            let (rho, far, z) = part_reach(&controls);
            let bound = meters.part_bound(
                f_sup,
                (lower(rho), upper(far)),
                (lower(z.0), upper(z.1)),
                locus,
                lower,
                upper,
            );
            worst = max_bound(worst, bound);
            if worst.is_nan() {
                return worst;
            }
        }
    }
    worst
}

/// The door's own rows, with no sampled refutation in front of it: each
/// case's bound must hold against the TRUE distance read densely off the
/// patch (closed-form distance from each locus), and not be vacuous where
/// the patch is on its carrier.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::spline::KnotVector;
    use geom_core::{Point3, Vec3};

    /// A quarter turn about `+y` of a meridian profile `(ρ, y)` at
    /// `degree` over one span (Piegl–Tiller's rational surface of
    /// revolution): `u` the profile, `v` the quarter turn.
    fn revolved(profile: &[(f64, f64)], profile_w: &[f64], degree: usize) -> NurbsSurface<f64> {
        let mut knots = vec![0.0; degree + 1];
        knots.extend(vec![1.0; degree + 1]);
        let ku = KnotVector::clamped(knots, degree).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let turn = [((1.0, 0.0), 1.0), ((1.0, 1.0), w), ((0.0, 1.0), 1.0)];
        let (mut control, mut weights) = (Vec::new(), Vec::new());
        for (&(rho, y), &wp) in profile.iter().zip(profile_w) {
            for &((c, s), wt) in &turn {
                control.push(Point3::new(rho * c, y, -rho * s));
                weights.push(wp * wt);
            }
        }
        NurbsSurface::new(ku, kv, control, weights).unwrap()
    }

    const QUARTER_W: [f64; 3] = [1.0, core::f64::consts::FRAC_1_SQRT_2, 1.0];

    /// A bilinear flat patch over `[x0, x1] × [z0, z1]` at height `y`.
    fn flat(x: (f64, f64), z: (f64, f64), y: f64) -> NurbsSurface<f64> {
        let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = vec![
            Point3::new(x.0, y, z.0),
            Point3::new(x.0, y, z.1),
            Point3::new(x.1, y, z.0),
            Point3::new(x.1, y, z.1),
        ];
        NurbsSurface::new(k.clone(), k, control, vec![1.0; 4]).unwrap()
    }

    /// The unit cylinder about `+z`, extruded over `z ∈ [0, 1]` as a
    /// rational quarter arc in `u`, its radius `1 + δ·B(v)` where the cubic
    /// `B(v) = v(1 − v)(1 − 2v)` vanishes at `v = 0, ½, 1`: on the locus at
    /// every one of a 3×3 sample grid's rows, `0.096·δ` off it between.
    fn cylinder_bulged_between_samples(delta: f64) -> NurbsSurface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let arc = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let b = [0.0, 1.0 / 3.0, -1.0 / 3.0, 0.0];
        let (mut control, mut weights) = (Vec::new(), Vec::new());
        for (i, &(x, y)) in arc.iter().enumerate() {
            for (j, bj) in b.iter().enumerate() {
                let s = 1.0 + delta * bj;
                #[allow(clippy::cast_precision_loss)]
                control.push(Point3::new(s * x, s * y, j as f64 / 3.0));
                weights.push(QUARTER_W[i]);
            }
        }
        NurbsSurface::new(ku, kv, control, weights).unwrap()
    }

    fn y_axis() -> (Vec3<f64>, Vec3<f64>) {
        (Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0))
    }

    /// The axial distance `ρ` and height `h` of `p` about the line through
    /// `o` along unit `a`.
    fn rho_h(p: Point3<f64>, o: Point3<f64>, a: Vec3<f64>) -> (f64, f64) {
        let d = p - o;
        let h = d.dot(a);
        ((d - a * h).norm(), h)
    }

    /// The true distance of `p` from `surface`'s complete locus.
    fn true_distance(surface: &Surface<f64>, p: Point3<f64>) -> f64 {
        match *surface {
            Surface::Plane { frame, .. } => {
                let (origin, normal) = (frame.origin(), frame.axis());
                (p - origin).dot(normal).abs()
            }
            Surface::Cylinder { frame, radius, .. } => {
                let (origin, axis) = (frame.origin(), frame.axis());
                (rho_h(p, origin, axis).0 - radius).abs()
            }
            Surface::Sphere { frame, radius, .. } => {
                let center = frame.origin();
                ((p - center).norm() - radius).abs()
            }
            Surface::Cone {
                frame, half_angle, ..
            } => {
                let (apex, axis) = (frame.origin(), frame.axis());
                let (rho, h) = rho_h(p, apex, axis);
                let (s, c) = half_angle.sin_cos();
                // The two generator rays of the meridian half-plane.
                let ray = |dir: (f64, f64)| {
                    let t = rho * dir.0 + h * dir.1;
                    if t <= 0.0 {
                        rho.hypot(h)
                    } else {
                        (rho * dir.1 - h * dir.0).abs()
                    }
                };
                ray((s, c)).min(ray((s, -c)))
            }
            Surface::Torus {
                frame,
                major_radius,
                minor_radius,
                ..
            } => {
                let (center, axis) = (frame.origin(), frame.axis());
                let (rho, h) = rho_h(p, center, axis);
                ((rho - major_radius).hypot(h) - minor_radius).abs()
            }
            ref other => panic!("no closed-form distance for {other:?}"),
        }
    }

    /// The largest true distance over a 41 × 41 grid of the patch.
    fn dense_sup(patch: &NurbsSurface<f64>, surface: &Surface<f64>) -> f64 {
        let (u0, u1) = patch.knot_values_u().domain();
        let (v0, v1) = patch.knot_values_v().domain();
        let mut worst = 0.0_f64;
        for i in 0..=40 {
            for j in 0..=40 {
                let u = u0 + (u1 - u0) * f64::from(i) / 40.0;
                let v = v0 + (v1 - v0) * f64::from(j) / 40.0;
                worst = worst.max(true_distance(surface, patch.eval(u, v)));
            }
        }
        worst
    }

    fn torus(major: f64, minor: f64) -> Surface<f64> {
        let (axis, u_ref) = y_axis();
        Surface::Torus {
            frame: geom::Frame::new(Point3::origin(), axis, u_ref),
            major_radius: major,
            minor_radius: minor,
        }
    }

    fn sphere(radius: f64, center: Point3<f64>) -> Surface<f64> {
        let (axis, u_ref) = y_axis();
        Surface::Sphere {
            frame: geom::Frame::new(center, axis, u_ref),
            radius,
        }
    }

    fn cone(half_angle: f64, apex: Point3<f64>) -> Surface<f64> {
        let (axis, u_ref) = y_axis();
        Surface::Cone {
            frame: geom::Frame::new(apex, axis, u_ref),
            half_angle,
        }
    }

    fn cylinder_z(radius: f64) -> Surface<f64> {
        Surface::Cylinder {
            frame: geom::Frame::new(
                Point3::origin(),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            radius,
        }
    }

    /// **Every bound holds against the dense true distance**, over each
    /// kind, on and off its carrier, inside and outside it (`f` of both
    /// signs in one cell), and on the singular sets: a cylinder's axis
    /// crossing the patch, a cone's apex on it, a patch on the cone's
    /// other nappe, a sphere's centre on it. A bound that is NaN or `+∞`
    /// certifies nothing and is sound; the rows on their carrier must be
    /// finite and rounding-small.
    #[test]
    fn every_bound_holds_against_the_dense_true_distance() {
        let tube = revolved(&[(4.0, 0.0), (4.0, 1.0), (3.0, 1.0)], &QUARTER_W, 2);
        let octant = revolved(&[(2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], &QUARTER_W, 2);
        let frustum = revolved(&[(1.0, 0.0), (2.0, 1.0)], &[1.0, 1.0], 1);
        let lower_nappe = revolved(&[(1.0, -2.0), (2.0, -3.0)], &[1.0, 1.0], 1);
        let across_axis = flat((0.0, 1.2), (0.0, 1.0), 0.05);
        let through_apex = flat((-0.5, 0.5), (-0.5, 0.5), -1.0);
        let bulged = cylinder_bulged_between_samples(1e-3);
        let apex = Point3::new(0.0, -1.0, 0.0);
        let quarter = core::f64::consts::FRAC_PI_4;
        let cases: Vec<(&str, &NurbsSurface<f64>, Surface<f64>, bool)> = vec![
            ("torus on", &tube, torus(3.0, 1.0), true),
            ("torus, tube too fat", &tube, torus(3.0, 1.05), false),
            ("torus, core too small", &tube, torus(2.9, 1.0), false),
            ("sphere on", &octant, sphere(2.0, Point3::origin()), true),
            (
                "sphere, patch inside",
                &octant,
                sphere(2.3, Point3::origin()),
                false,
            ),
            (
                "sphere, centre on the patch",
                &octant,
                sphere(0.5, Point3::new(2.0, 0.0, 0.0)),
                false,
            ),
            ("cone on", &frustum, cone(quarter, apex), true),
            (
                "cone, half-angle off",
                &frustum,
                cone(quarter + 0.05, apex),
                false,
            ),
            (
                "cone, the other nappe",
                &lower_nappe,
                cone(quarter, apex),
                false,
            ),
            (
                "cone, off it on the other nappe",
                &lower_nappe,
                cone(quarter + 0.05, apex),
                false,
            ),
            (
                "cone, apex on the patch",
                &through_apex,
                cone(quarter, apex),
                false,
            ),
            (
                "cylinder, its axis by the patch",
                &across_axis,
                cylinder_z(1.0),
                false,
            ),
            (
                "cylinder, between the samples",
                &bulged,
                cylinder_z(1.0),
                false,
            ),
            (
                "plane on",
                &across_axis,
                Surface::Plane {
                    frame: geom::Frame::new(
                        Point3::new(0.0, 0.05, 0.0),
                        Vec3::new(0.0, 1.0, 0.0),
                        Vec3::new(1.0, 0.0, 0.0),
                    ),
                },
                true,
            ),
        ];
        for (what, patch, surface, on) in cases {
            let bound = patch_distance_sup(patch, &surface);
            // The oracle evaluates the patch at `f64`: its own rounding,
            // a few ulps of the patch's metre scale, is not a distance.
            let truth = dense_sup(patch, &surface) - 1e-14;
            assert!(
                bound.is_nan() || bound >= truth,
                "{what}: the bound {bound:e} is below the true distance {truth:e}"
            );
            if on {
                assert!(bound < 1e-12, "{what}: on the carrier, the bound {bound:e}");
            }
        }
    }

    /// The unit cylinder about `+z` as four rational quarter spans in `u`
    /// (double interior knots), over `z ∈ [0, 1]`, its LAST span's middle
    /// control pushed out by `bulge`.
    fn four_span_cylinder(bulge: f64) -> NurbsSurface<f64> {
        let ku = KnotVector::clamped(
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 4.0, 4.0, 4.0],
            2,
        )
        .unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let (mut control, mut weights) = (Vec::new(), Vec::new());
        for i in 0..9 {
            let theta = core::f64::consts::FRAC_PI_4 * f64::from(i);
            let (radius, w) = if i % 2 == 1 {
                (core::f64::consts::SQRT_2, core::f64::consts::FRAC_1_SQRT_2)
            } else {
                (1.0, 1.0)
            };
            let radius = if i == 7 { radius + bulge } else { radius };
            for z in [0.0, 1.0] {
                control.push(Point3::new(radius * theta.cos(), radius * theta.sin(), z));
                weights.push(w);
            }
        }
        NurbsSurface::new(ku, kv, control, weights).unwrap()
    }

    /// **Every cell is read**: a four-span cylinder bulged in its last
    /// span alone. The bound holds the bulge, and the exact twin reads
    /// rounding-small — so a fold that skipped any cell would fail one.
    #[test]
    fn every_cell_of_a_multi_span_patch_is_read() {
        let surface = cylinder_z(1.0);
        let bulged = four_span_cylinder(1e-3);
        let truth = dense_sup(&bulged, &surface);
        assert!(truth > 1e-4, "the bulge is real: {truth:e}");
        let bound = patch_distance_sup(&bulged, &surface);
        assert!(
            bound >= truth,
            "the last span's bulge is read: {bound:e} vs {truth:e}"
        );
        let exact = patch_distance_sup(&four_span_cylinder(0.0), &surface);
        assert!(exact < 1e-14, "the exact twin: {exact:e}");
    }

    /// **The double cone's reading beats the nappe's where the lever is
    /// small**: a flat patch just above the apex of a cone of half-angle
    /// 80°, so its cells' generator lever `ρ cos α + z sin α` is small
    /// against `|f|`, and the apex root `√|f|` is the tighter bound. Both
    /// hold against the true distance; the door's is strictly the lesser.
    #[test]
    fn the_double_cones_reading_beats_the_nappes_near_the_apex() {
        let patch = flat((0.1, 0.2), (0.0, 0.1), -0.999);
        let surface = cone(80.0_f64.to_radians(), Point3::new(0.0, -1.0, 0.0));
        let double = distance_sup_on(&patch, &surface, ConeLocus::Double);
        let nappe = distance_sup_on(&patch, &surface, ConeLocus::Nappe);
        let truth = dense_sup(&patch, &surface);
        assert!(
            truth <= double && double < nappe,
            "truth {truth:e} ≤ double {double:e} < nappe {nappe:e}"
        );
    }

    /// **A bulge between a 3×3 sample grid's rows is read**: the bulged
    /// cylinder sits on the unit cylinder at `v = 0, ½, 1` and `0.096·δ`
    /// off it between, and the bound reads at least that.
    #[test]
    fn a_bulge_between_the_samples_is_read() {
        let delta = 1e-3;
        let patch = cylinder_bulged_between_samples(delta);
        let surface = cylinder_z(1.0);
        for v in [0.0, 0.5, 1.0] {
            for u in [0.0, 0.5, 1.0] {
                let d = true_distance(&surface, patch.eval(u, v));
                assert!(d < 1e-15, "on the locus at the sample ({u}, {v}): {d:e}");
            }
        }
        let bound = patch_distance_sup(&patch, &surface);
        assert!(
            bound >= 0.09 * delta,
            "the bulge {:e} is read: {bound:e}",
            0.096 * delta
        );
    }
}
