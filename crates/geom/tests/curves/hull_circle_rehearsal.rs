//! **The C2.2 shape rehearsal** (M5 PR 2, acceptance family 4): the
//! acceptance the whole PR exists for — since M5 PR 4 running
//! ON the promoted substrate (`geom_core::spline::compose`) instead of
//! test-local Bernstein plumbing. The numbers this test publishes must
//! reproduce the PR 2 rehearsal's bit-for-bit (the refactor pins that
//! below); the prose history lives in this file's PR 2 revision.
//!
//! A fitted cache is certified by bounding the *residual composite*
//! `f ∘ C` — the analytic surface/curve's implicit function evaluated
//! along the fitted NURBS — over every span, not by sampling it
//! (C2.2 limb 2, OQ2: hull bounds are an entry requirement). This test
//! runs that shape end to end on the one case whose answer is known
//! exactly: the §7.3 rational-quadratic circle, which lies on
//! [`Curve3::Circle`]'s locus **identically in ℝ**. The residual is
//! therefore exactly zero in exact arithmetic, and the bound this
//! machinery produces measures nothing but the `f64` representation of
//! the control data plus the ring's own conservatism. If hull bounds
//! were too coarse to certify an exact case, they would be useless for
//! the near-exact ones M5 PR 4 certifies — so the observed magnitude
//! is the number this test exists to publish.
//!
//! # Why the *implicit* residual and not a coordinate difference
//!
//! The two curves do not share a parameter: the NURBS parameter `t` maps
//! to the angle by `θ(t) = 2·atan(…)`. A coordinate-wise difference
//! would therefore need a transcendental parameter map, which the C9
//! ring deliberately cannot express — and whose own error would swamp
//! the number being measured. The implicit residual has no such problem
//! and *is* the C2.2 certificate's limb-1 quantity ("max over the
//! schedule of |f₁(C(t))|, |f₂(C(t))|"). Both of the circle's implicit
//! limbs are checked:
//!
//! - `f₁(P) = |P − c|² − r²` (on the sphere about the center — the
//!   compose module's [`ImplicitSurface::Sphere`]), and
//! - `f₂(P) = (P − c)·n` (in the circle's plane —
//!   [`ImplicitSurface::Plane`]).
//!
//! The pipeline the PR 2 revision of this file spelled out inline
//! (homogeneous ring lift, center-shift before products, exact
//! Bernstein products with ring-quotient binomials, hull bounds, the
//! zero-refusing denominator division) is now `compose`'s module
//! contract, verbatim — the per-arc handling maps to the composite's
//! per-span bounds (the circle's arcs *are* its Bézier spans).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use geom::{Curve3, NurbsCurve3};
use geom_core::spline::compose::{self, CurveRingData, ImplicitSurface};
use geom_core::{Point3, RingInterval, Vec3};

const SQRT2_2: f64 = core::f64::consts::FRAC_1_SQRT_2;
const RADIUS: f64 = 2.5;

fn axis() -> Vec3<f64> {
    Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0)
}

fn u_ref() -> Vec3<f64> {
    Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0)
}

fn v_ref() -> Vec3<f64> {
    // axis × u_ref, exact: (2, −1, −2)/3.
    Vec3::new(2.0 / 3.0, -1.0 / 3.0, -2.0 / 3.0)
}

fn center() -> Point3<f64> {
    Point3::new(-0.5, 4.0, 1.25)
}

/// The §7.3 full circle: four rational-quadratic 90° arcs, `w₁ = √2/2`.
/// Identical fixture to `nurbs_differential.rs` (PR 3's acceptance), so
/// this test bounds the residual of a curve another suite already pins
/// against `Curve3::Circle` pointwise.
fn nurbs_circle() -> NurbsCurve3<f64> {
    let (c, r, x, y) = (center(), RADIUS, u_ref(), v_ref());
    let p = |cx: f64, cy: f64| c + x * (r * cx) + y * (r * cy);
    let control = vec![
        p(1.0, 0.0),
        p(1.0, 1.0),
        p(0.0, 1.0),
        p(-1.0, 1.0),
        p(-1.0, 0.0),
        p(-1.0, -1.0),
        p(0.0, -1.0),
        p(1.0, -1.0),
        p(1.0, 0.0),
    ];
    let weights = vec![1.0, SQRT2_2, 1.0, SQRT2_2, 1.0, SQRT2_2, 1.0, SQRT2_2, 1.0];
    let knots = geom_core::spline::KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    NurbsCurve3::new(knots, control, weights).unwrap()
}

fn analytic_circle() -> Curve3<f64> {
    Curve3::Circle {
        center: center(),
        axis: axis(),
        radius: RADIUS,
        u_ref: u_ref(),
    }
}

/// The per-arc certificate: hull bounds on both implicit residuals, in
/// meters² and meters respectively, plus the denominator's own bound —
/// read off the compose module's per-span forms (arc = Bézier span).
struct ArcBound {
    sphere: f64,
    plane: f64,
    w2_lo: f64,
}

/// All four arcs' bounds from ONE composite build per implicit limb —
/// PR 4's one-call story where PR 2 had a hand-rolled pipeline per arc.
fn arc_bounds(curve: &NurbsCurve3<f64>) -> Vec<ArcBound> {
    let c = center();
    let n = axis();
    let coords = curve.ring_coords();
    let data = CurveRingData::new(curve.knots(), curve.weights(), &coords).unwrap();

    let sphere = compose::implicit_composite(
        &data,
        &ImplicitSurface::Sphere {
            center: [c.x, c.y, c.z],
            radius: RADIUS,
        },
    )
    .unwrap();
    let plane = compose::implicit_composite(
        &data,
        &ImplicitSurface::Plane {
            point: [c.x, c.y, c.z],
            normal: [n.x, n.y, n.z],
        },
    )
    .unwrap();

    let sphere_bounds = sphere.span_bounds();
    let plane_bounds = plane.span_bounds();
    let w2_hulls: Vec<RingInterval> = sphere.den.span_hulls();
    (0..sphere_bounds.len())
        .map(|arc| ArcBound {
            sphere: sphere_bounds[arc].mag(),
            plane: plane_bounds[arc].mag(),
            w2_lo: w2_hulls[arc].lo(),
        })
        .collect()
}

/// The residual oracle: the same implicit forms, evaluated at `f64` on a
/// point of the curve. Independent of every hull/ring code path.
fn residuals_at(p: Point3<f64>) -> (f64, f64) {
    let c = center();
    let d = p - c;
    let sphere = d.dot(d) - RADIUS * RADIUS;
    let plane = d.dot(axis());
    (sphere, plane)
}

#[test]
fn c2_2_rehearsal_circle_residual_hull_bound_is_sound_and_tight() {
    let curve = nurbs_circle();
    let bounds = arc_bounds(&curve);
    assert_eq!(bounds.len(), 4, "four 90° arcs = four Bézier spans");
    let (mut worst_sphere, mut worst_plane) = (0.0f64, 0.0f64);
    let (mut max_sampled_sphere, mut max_sampled_plane) = (0.0f64, 0.0f64);
    for (arc, b) in bounds.iter().enumerate() {
        assert!(
            b.sphere.is_finite() && b.plane.is_finite(),
            "poisoned bound"
        );
        worst_sphere = worst_sphere.max(b.sphere);
        worst_plane = worst_plane.max(b.plane);
        // Soundness by falsification: dense sampling inside the arc.
        let (t0, t1) = (f64::from(arc as u32) / 4.0, f64::from(arc as u32 + 1) / 4.0);
        for k in 0..=512 {
            let t = t0 + (t1 - t0) * (f64::from(k) / 512.0);
            let (s, pl) = residuals_at(curve.eval(t));
            assert!(
                s.abs() <= b.sphere,
                "arc {arc}: sampled sphere residual {s:e} exceeds bound {:e}",
                b.sphere
            );
            assert!(
                pl.abs() <= b.plane,
                "arc {arc}: sampled plane residual {pl:e} exceeds bound {:e}",
                b.plane
            );
            max_sampled_sphere = max_sampled_sphere.max(s.abs());
            max_sampled_plane = max_sampled_plane.max(pl.abs());
        }
        assert!(b.w2_lo > 0.4, "arc {arc}: denominator bound {:e}", b.w2_lo);
    }
    // Tightness, the number this test exists to publish. The residual is
    // exactly zero in ℝ, so the whole bound is f64 representation error
    // plus ring conservatism; anything near the fp scale of r² = 6.25 m²
    // means hull bounds are usable for the exactness-adjacent cases M5
    // PR 4 certifies.
    let fp_scale = RADIUS * RADIUS * f64::EPSILON;
    println!(
        "[C2.2 rehearsal] sphere residual hull bound {worst_sphere:.3e} m² \
         ({:.1} ulps of r² = {:.3e}); densest sampled max {max_sampled_sphere:.3e} m². \
         Plane residual hull bound {worst_plane:.3e} m; densest sampled max \
         {max_sampled_plane:.3e} m.",
        worst_sphere / fp_scale,
        fp_scale
    );
    assert!(
        worst_sphere < 64.0 * fp_scale,
        "sphere bound {worst_sphere:e} is far above the fp scale {fp_scale:e}"
    );
    assert!(
        worst_plane < 64.0 * RADIUS * f64::EPSILON,
        "plane bound {worst_plane:e} is far above the fp scale"
    );
    // The refactor pin: the promoted compose module reproduces the PR 2
    // test-local pipeline BIT-identically (same lifts, same association
    // orders, same hull folds — captured 2026-07-27 from the pre-refactor
    // revision of this file at full precision).
    assert_eq!(
        worst_sphere.to_bits(),
        0x3d1f_8000_0000_0009, // 2.7977620220553973e-14
        "sphere bound drifted from the PR 2 rehearsal: {worst_sphere:.17e}"
    );
    assert_eq!(
        worst_plane.to_bits(),
        0x3ce9_74b2_334f_2349, // 2.8261664256307962e-15
        "plane bound drifted from the PR 2 rehearsal: {worst_plane:.17e}"
    );
}

#[test]
fn the_two_circles_are_the_same_locus() {
    // What licenses reading the implicit residual as the difference from
    // `Curve3::Circle`: the analytic curve satisfies the same implicit
    // forms to fp scale, so a bound on the NURBS residual bounds the
    // NURBS's departure from *that* locus.
    let analytic = analytic_circle();
    let tol = 64.0 * RADIUS * RADIUS * f64::EPSILON;
    for k in 0..=2048 {
        let theta = core::f64::consts::TAU * f64::from(k) / 2048.0;
        let (s, pl) = residuals_at(analytic.eval(theta));
        assert!(s.abs() <= tol, "analytic sphere residual {s:e}");
        assert!(pl.abs() <= tol, "analytic plane residual {pl:e}");
    }
}
