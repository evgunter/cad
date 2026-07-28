//! **The C2.2 shape rehearsal** (M5 PR 2, `docs/M5-PR2-SPEC.md` family
//! 4): the acceptance the whole PR exists for.
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
//! the near-exact ones M5 PR 4 must certify — so the observed magnitude
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
//! - `f₁(P) = |P − c|² − r²` (on the sphere about the center), and
//! - `f₂(P) = (P − c)·n` (in the circle's plane).
//!
//! # The pipeline, which is PR 4's in miniature
//!
//! 1. Per 90° arc, lift the control data to homogeneous Bernstein
//!    coefficients **in the ring** (`RingInterval`).
//! 2. Form the composite's numerator coefficients by exact Bernstein
//!    products — degree 2 × degree 2 = degree 4, with the binomial
//!    weights (`C(2,i)C(2,j)/C(4,k)`, several of which are not
//!    representable in `f64`) computed as ring quotients.
//! 3. Hull-bound those coefficients with `spline::hull` over a degree-4
//!    Bézier knot vector — no evaluation, no sampling.
//! 4. Divide by the strictly-positive hull bound of the denominator
//!    `W²` to get the residual in meters² (the ring refuses a
//!    zero-touching divisor, so this step cannot silently succeed on
//!    degenerate weights).
//! 5. Falsify by dense sampling: every sampled residual must lie inside
//!    the bound.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use geom_core::spline::{KnotVector, hull};
use geom_core::{Point3, RingInterval, Vec3};
use geom_curves::{Curve3, NurbsCurve3};

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
    let knots = KnotVector::clamped(
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

/// Binomial coefficients used by the Bernstein product.
const BIN2: [f64; 3] = [1.0, 2.0, 1.0];
const BIN4: [f64; 5] = [1.0, 4.0, 6.0, 4.0, 1.0];

/// Product of two degree-2 Bernstein polynomials, as degree-4 Bernstein
/// coefficients:
/// `(AB)_k = Σ_{i+j=k} [C(2,i)·C(2,j)/C(4,k)] · a_i · b_j`.
///
/// The weights `1/4`, `1/6` are not all representable in `f64`, so each
/// is formed as a ring quotient — an enclosure, not a rounded constant.
/// Fixed association (D9): ascending `k`, ascending `i`, the term order
/// written below.
fn bern2_mul(a: [RingInterval; 3], b: [RingInterval; 3]) -> [RingInterval; 5] {
    let mut out = [RingInterval::zero(); 5];
    for k in 0..5 {
        let mut acc = RingInterval::zero();
        for i in 0..3 {
            let j = k as i32 - i as i32;
            if !(0..3).contains(&j) {
                continue;
            }
            let j = j as usize;
            let w = RingInterval::point(BIN2[i] * BIN2[j]) / RingInterval::point(BIN4[k]);
            acc = acc + a[i] * b[j] * w;
        }
        out[k] = acc;
    }
    out
}

/// A degree-`n` Bézier knot vector on `[0, 1]` — the structure the
/// per-arc composite coefficients live on.
fn bezier_kv(degree: usize) -> KnotVector {
    let mut knots = vec![0.0; degree + 1];
    knots.extend(std::iter::repeat_n(1.0, degree + 1));
    KnotVector::clamped(knots, degree).unwrap()
}

/// The homogeneous, center-shifted Bernstein coefficients of one 90°
/// arc: `g_d,i = w_i·(P_d,i − c_d)` for each coordinate, plus the weight
/// coefficients `W_i = w_i`.
///
/// Center-shifting *before* the products is what keeps the composite
/// honest: forming `|P|² − 2P·c + |c|² − r²` instead would cancel
/// catastrophically in the coefficients, and an interval bound reports
/// that cancellation as width rather than hiding it.
fn arc_coeffs(curve: &NurbsCurve3<f64>, arc: usize) -> ([[RingInterval; 3]; 3], [RingInterval; 3]) {
    let c = center();
    let ctrl = curve.control();
    let w = curve.weights();
    let cc = [c.x, c.y, c.z];
    let mut g = [[RingInterval::zero(); 3]; 3];
    let mut wc = [RingInterval::zero(); 3];
    for i in 0..3 {
        let idx = 2 * arc + i;
        let p = ctrl[idx];
        let wi = RingInterval::point(w[idx]);
        wc[i] = wi;
        for (d, pd) in [p.x, p.y, p.z].into_iter().enumerate() {
            g[d][i] = wi * (RingInterval::point(pd) - RingInterval::point(cc[d]));
        }
    }
    (g, wc)
}

/// The per-arc certificate: hull bounds on both implicit residuals, in
/// meters² and meters respectively, plus the denominator's own bound.
struct ArcBound {
    sphere: f64,
    plane: f64,
    w2_lo: f64,
}

fn arc_bound(curve: &NurbsCurve3<f64>, arc: usize) -> ArcBound {
    let (g, wc) = arc_coeffs(curve, arc);
    let n = axis();
    let nd = [n.x, n.y, n.z];
    let r = RingInterval::point(RADIUS);

    // f₁ numerator: Σ_d G_d² − r²·W², degree 4.
    let w2 = bern2_mul(wc, wc);
    let mut num = [RingInterval::zero(); 5];
    for gd in g {
        let sq = bern2_mul(gd, gd);
        for k in 0..5 {
            num[k] = num[k] + sq[k];
        }
    }
    let r2 = r * r;
    for k in 0..5 {
        num[k] = num[k] - r2 * w2[k];
    }

    // f₂ numerator: Σ_d n_d·G_d, degree 2 (the plane condition).
    let mut pl = [RingInterval::zero(); 3];
    for (d, ndd) in nd.into_iter().enumerate() {
        let nk = RingInterval::point(ndd);
        for i in 0..3 {
            pl[i] = pl[i] + g[d][i] * nk;
        }
    }

    // Hull bounds — no evaluation anywhere below this line.
    let kv4 = bezier_kv(4);
    let kv2 = bezier_kv(2);
    let num_hull = hull::domain_hull(&kv4, &num);
    let w2_hull = hull::domain_hull(&kv4, &w2);
    let pl_hull = hull::domain_hull(&kv2, &pl);
    let w_hull = hull::domain_hull(&kv2, &wc);
    assert!(w2_hull.lo() > 0.0, "denominator hull must exclude zero");

    // The ring refuses a zero-touching divisor, so these divisions are
    // the certificate that the rational composite is bounded at all.
    ArcBound {
        sphere: (num_hull / w2_hull).mag(),
        plane: (pl_hull / w_hull).mag(),
        w2_lo: w2_hull.lo(),
    }
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
    let (mut worst_sphere, mut worst_plane) = (0.0f64, 0.0f64);
    let (mut max_sampled_sphere, mut max_sampled_plane) = (0.0f64, 0.0f64);
    for arc in 0..4 {
        let b = arc_bound(&curve, arc);
        assert!(b.sphere.is_finite() && b.plane.is_finite(), "poisoned bound");
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
