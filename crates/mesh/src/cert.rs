//! Per-triangle closed-form deviation certificates — the
//! certified-conservative half of the chordal promise (crate docs).
//!
//! Each bound is a statement in exact arithmetic about the triangle's
//! vertex data; every emitted triangle must certify ≤ δ or
//! tessellation fails loudly. Derivations (also in the PR log):
//!
//! - **Cylinder** (radius r): for p = Σλᵢxᵢ with |xᵢ⊥| = r, convexity
//!   gives |p⊥| ≤ r (no outward deviation) and
//!   dist(p, cyl) = r − |p⊥| ≤ r − dist(axis, T). Exact line–triangle
//!   distance ⇒ certificate `r − dist(axis, T)`.
//! - **Sphere** (center c, radius r): vertices on the sphere put the
//!   triangle inside the ball; dist(p, sphere) = r − |p − c| ≤
//!   r − dist(c, T). Certificate `r − dist(c, T)` (exact
//!   point–triangle distance). Handles pole-fan triangles with no UV
//!   involvement.
//! - **Cone** (half-angle α): with slant coords vᵢ ≥ 0, radii
//!   ρᵢ = vᵢ·sin α, and azimuth span Δu ≤ π over the non-apex
//!   vertices (the apex has ρ = 0 and drops out of the radial sum),
//!   the slant residual m(p) = cos α·ρ_p − sin α·h_p of p = Σλᵢxᵢ
//!   satisfies −cos α·sin α·v_maxᵀ·(1 − cos(Δu/2)) ≤ m ≤ 0, and
//!   dist(p, cone) ≤ |m| (perpendicular distance to the generator ray
//!   at p's azimuth; the foot's slant coordinate
//!   ρ_p·sin α + h_p·cos α ≥ 0 on v ≥ 0 patches). The **triangle-local**
//!   v_maxᵀ makes the bound scale with local radius, so apex-fan
//!   triangles certify tightly.
//! - **Torus** (R, r): the doubly-curved interpolation bound
//!   `crate::sizing::torus_grid_steps` derives and inverts (its (★)):
//!   with `Δu`, `Δv` the triangle's UV extents and the second-partial
//!   sups over the triangle's own φ range — `A = max |R + r·cos φ|`
//!   (the absolute value is what keeps it a sup on a spindle torus,
//!   `R < r`, where `R + r·cos φ` crosses zero), `B = r·max |sin φ|`,
//!   `C = r` — Taylor with the integral remainder
//!   at each point, Popoviciu's variance bound and Cauchy–Schwarz give
//!   `‖S − ΠS‖ ≤ (A·Δu² + 2B·Δu·Δv + C·Δv²)/8`. Per direction, and
//!   attained: on the outer equator (φ = 0, where `P_θθ ∥ P_φφ` and
//!   `P_θφ = 0`) a cell-half's hypotenuse midpoint deviates by exactly
//!   this to second order, measured below. Certificate
//!   `(A·Δu² + 2B·Δu·Δv + C·Δv²)/8`.
//! - **Plane**: affine — deviation 0, no certificate needed.
//!
//! Two additive slacks sit outside all bounds, documented in the crate
//! docs: boundary vertices carry the kernel's ≤ ε carrier residual
//! (they lie on certified carriers, not exactly on the surface), and
//! f64 rounding of the evaluations. Sizing targets δ/2 so neither
//! decides in practice.

use geom_core::{Point3, Vec3};

/// Exact point–triangle distance (the standard closest-point-on-
/// triangle case analysis; total and branch-complete).
pub fn dist_point_triangle(p: Point3<f64>, a: Point3<f64>, b: Point3<f64>, c: Point3<f64>) -> f64 {
    let ab = b - a;
    let ac = c - a;
    let ap = p - a;
    let d1 = ab.dot(ap);
    let d2 = ac.dot(ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return ap.norm(); // vertex a
    }
    let bp = p - b;
    let d3 = ab.dot(bp);
    let d4 = ac.dot(bp);
    if d3 >= 0.0 && d4 <= d3 {
        return bp.norm(); // vertex b
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let t = d1 / (d1 - d3);
        return (ap - ab * t).norm(); // edge ab
    }
    let cp = p - c;
    let d5 = ab.dot(cp);
    let d6 = ac.dot(cp);
    if d6 >= 0.0 && d5 <= d6 {
        return cp.norm(); // vertex c
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let t = d2 / (d2 - d6);
        return (ap - ac * t).norm(); // edge ac
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let t = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return (bp - (c - b) * t).norm(); // edge bc
    }
    // Interior: distance along the normal.
    let denom = 1.0 / (va + vb + vc);
    let s = vb * denom;
    let t = vc * denom;
    (ap - (ab * s + ac * t)).norm()
}

/// Exact line–triangle distance: project the triangle onto the plane
/// through `o` orthogonal to the unit direction `d` (a projection of a
/// triangle is a triangle) — the line collapses to the origin of that
/// plane, so the distance is a point–triangle distance.
pub fn dist_line_triangle(
    o: Point3<f64>,
    d: Vec3<f64>,
    a: Point3<f64>,
    b: Point3<f64>,
    c: Point3<f64>,
) -> f64 {
    let proj = |p: Point3<f64>| -> Point3<f64> {
        let w = p - o;
        Point3::origin() + (w - d * w.dot(d))
    };
    dist_point_triangle(Point3::origin(), proj(a), proj(b), proj(c))
}

/// Cylinder certificate: `r − dist(axis, T)`, clamped at 0.
pub fn cert_cylinder(origin: Point3<f64>, axis: Vec3<f64>, r: f64, tri: [Point3<f64>; 3]) -> f64 {
    (r - dist_line_triangle(origin, axis, tri[0], tri[1], tri[2])).max(0.0)
}

/// Sphere certificate: `r − dist(center, T)`, clamped at 0.
pub fn cert_sphere(center: Point3<f64>, r: f64, tri: [Point3<f64>; 3]) -> f64 {
    (r - dist_point_triangle(center, tri[0], tri[1], tri[2])).max(0.0)
}

/// Cone certificate: `cos α · sin α · v_maxᵀ · (1 − cos(Δu/2))` over
/// the non-apex vertices' azimuth span Δu (module docs); +∞ if
/// Δu > π (the derivation's precondition — certifiably violated
/// geometry fails loudly, never silently).
pub fn cert_cone(half_angle: f64, uv: [[f64; 2]; 3], pole: [bool; 3]) -> f64 {
    let mut u_min = f64::INFINITY;
    let mut u_max = f64::NEG_INFINITY;
    let mut v_max: f64 = 0.0;
    for i in 0..3 {
        v_max = v_max.max(uv[i][1].abs());
        if !pole[i] {
            u_min = u_min.min(uv[i][0]);
            u_max = u_max.max(uv[i][0]);
        }
    }
    let du = u_max - u_min;
    if du.is_nan() || du > core::f64::consts::PI {
        return f64::INFINITY;
    }
    let (s_a, c_a) = (half_angle.sin(), half_angle.cos());
    c_a * s_a * v_max * (1.0 - (du * 0.5).cos())
}

/// Torus certificate: `(A·Δu² + 2B·Δu·Δv + C·Δv²)/8` over the
/// triangle's UV extents, with the second-partial sups taken over the
/// triangle's own φ range (module docs; the derivation is
/// `crate::sizing::torus_grid_steps`'). Total for every `R, r > 0`:
/// `A` is `max |R + r·cos φ|`, so a spindle torus's window across
/// `R + r·cos φ = 0` reads the larger lobe rather than a signed value.
/// NaN-sticky: a poisoned corner poisons the certificate rather than
/// being dropped by a `max`.
pub fn cert_torus(major: f64, minor: f64, uv: [[f64; 2]; 3]) -> f64 {
    if uv.iter().flatten().any(|x| x.is_nan()) {
        return f64::NAN;
    }
    let (mut u0, mut u1, mut v0, mut v1) = (
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
    );
    for [u, v] in uv {
        u0 = u0.min(u);
        u1 = u1.max(u);
        v0 = v0.min(v);
        v1 = v1.max(v);
    }
    torus_chord_bound(
        abs_radial_max(major, minor, v0, v1),
        minor * abs_sin_max(v0, v1),
        minor,
        u1 - u0,
        v1 - v0,
    )
}

/// The doubly-curved chord bound `(A·Δu² + 2B·Δu·Δv + C·Δv²)/8` on
/// the second-partial sups `A, B, C` and the UV extents `Δu, Δv` —
/// the one arithmetic [`cert_torus`] certifies with and
/// `crate::sizing::torus_grid_steps` inverts, so the two cannot drift
/// apart by a constant (`sizing`'s
/// `torus_grid_steps_and_cert_torus_are_an_inverse_pair`).
pub(crate) fn torus_chord_bound(a: f64, b: f64, c: f64, du: f64, dv: f64) -> f64 {
    (a * du.powi(2) + 2.0 * b * du * dv + c * dv.powi(2)) / 8.0
}

/// `max |R + r·cos φ|` over `[v0, v1]`: the endpoints, plus `R + r`
/// where the interval holds a multiple of 2π and `|R − r|` where it
/// holds an odd multiple of π (every interior extremum of
/// `R + r·cos φ` is one of those, and `|·|` peaks at an extremum or
/// an endpoint).
fn abs_radial_max(major: f64, minor: f64, v0: f64, v1: f64) -> f64 {
    use core::f64::consts::{PI, TAU};
    let mut m = (major + minor * v0.cos())
        .abs()
        .max((major + minor * v1.cos()).abs());
    if (v1 / TAU).floor() >= (v0 / TAU).ceil() {
        m = m.max(major + minor);
    }
    if ((v1 - PI) / TAU).floor() >= ((v0 - PI) / TAU).ceil() {
        m = m.max((major - minor).abs());
    }
    m
}

/// `max |sin φ|` over `[v0, v1]`: 1 if the interval holds an odd
/// multiple of π/2, else at an endpoint.
fn abs_sin_max(v0: f64, v1: f64) -> f64 {
    use core::f64::consts::{FRAC_PI_2, PI};
    if ((v1 - FRAC_PI_2) / PI).floor() >= ((v0 - FRAC_PI_2) / PI).ceil() {
        1.0
    } else {
        v0.sin().abs().max(v1.sin().abs())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use core::f64::consts::PI;

    /// A torus on the z axis with `u_ref = x`, in the chart the
    /// certificate is stated in: `(R + r·cos v)·(cos u, sin u, 0) +
    /// (0, 0, r·sin v)`.
    fn torus_point(major: f64, minor: f64, u: f64, v: f64) -> Point3<f64> {
        let rho = major + minor * v.cos();
        Point3::new(rho * u.cos(), rho * u.sin(), minor * v.sin())
    }

    /// Exact distance from `p` to that torus: in the `(ρ, z)` half-plane
    /// the tube is the circle of radius `r` about `(R, 0)`.
    fn torus_dist(major: f64, minor: f64, p: Point3<f64>) -> f64 {
        let rho = p.x.hypot(p.y);
        (((rho - major).powi(2) + p.z * p.z).sqrt() - minor).abs()
    }

    /// The largest sampled `‖Π − P‖` over the triangle `uv` — the
    /// affine interpolant against the parametric surface at the same
    /// barycentric point (barycentric grid, `n = 8` ⇒ 45 points). This
    /// is the quantity the certificate bounds, and on a spindle torus
    /// (`R < r`) it is the only honest one: the inner lobe of the
    /// parametrisation lies INSIDE the solid, so a distance to the
    /// locus "r from the major circle" is not a distance to the chart.
    fn parametric_deviation(major: f64, minor: f64, uv: [[f64; 2]; 3]) -> f64 {
        let c = uv.map(|[u, v]| torus_point(major, minor, u, v));
        let n = 8u32;
        let mut worst: f64 = 0.0;
        for i in 0..=n {
            for j in 0..=(n - i) {
                let (li, lj) = (f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                let lk = 1.0 - li - lj;
                let u = uv[0][0] * li + uv[1][0] * lj + uv[2][0] * lk;
                let v = uv[0][1] * li + uv[1][1] * lj + uv[2][1] * lk;
                let p = torus_point(major, minor, u, v);
                let q = Point3::new(
                    c[0].x * li + c[1].x * lj + c[2].x * lk,
                    c[0].y * li + c[1].y * lj + c[2].y * lk,
                    c[0].z * li + c[1].z * lj + c[2].z * lk,
                );
                worst = worst.max((q - p).norm());
            }
        }
        worst
    }

    /// The largest sampled distance from the affine triangle over `uv`
    /// to the ring torus's locus (barycentric grid, `n = 8` ⇒ 45
    /// points) — the chordal promise's own quantity, `≤` the parametric
    /// deviation; ring tori only (`R ≥ r`), where locus and chart agree.
    fn sampled_deviation(major: f64, minor: f64, uv: [[f64; 2]; 3]) -> f64 {
        let c = uv.map(|[u, v]| torus_point(major, minor, u, v));
        let n = 8u32;
        let mut worst: f64 = 0.0;
        for i in 0..=n {
            for j in 0..=(n - i) {
                let (li, lj) = (f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                let lk = 1.0 - li - lj;
                let p = Point3::new(
                    c[0].x * li + c[1].x * lj + c[2].x * lk,
                    c[0].y * li + c[1].y * lj + c[2].y * lk,
                    c[0].z * li + c[1].z * lj + c[2].z * lk,
                );
                worst = worst.max(torus_dist(major, minor, p));
            }
        }
        worst
    }

    /// **Soundness of [`cert_torus`] on triangles of every shape**, not
    /// only the axis-aligned cell-halves the grid emits: random UV
    /// triangles inside random boxes up to 0.6 rad a side, anywhere on
    /// tori across `R/r` from 1.1 to 100, densely sampled against the
    /// exact torus distance. Popoviciu and Cauchy–Schwarz are what
    /// make the bound hold off the lattice, and this is their row.
    #[test]
    fn cert_torus_bounds_the_sampled_deviation_on_random_triangles() {
        let mut rng = test_utils::fuzz::start("cert_torus");
        for _ in 0..test_utils::fuzz::scaled(3000) {
            let minor = rng.range(0.05, 2.0);
            let major = minor * rng.range(1.1, 100.0);
            let (u0, v0) = (rng.range(-PI, PI), rng.range(-PI, PI));
            let (du, dv) = (rng.range(0.0, 0.6), rng.range(0.0, 0.6));
            let mut uv = [[0.0; 2]; 3];
            for corner in &mut uv {
                *corner = [u0 + rng.unit() * du, v0 + rng.unit() * dv];
            }
            let cert = cert_torus(major, minor, uv);
            let dev = sampled_deviation(major, minor, uv);
            assert!(
                dev <= cert * (1.0 + 1e-9) + 1e-12,
                "R {major} r {minor} uv {uv:?}: sampled deviation {dev} exceeds the certificate \
                 {cert} ({})",
                test_utils::fuzz::replay()
            );
        }
    }

    /// **The certificate's constant is not slack.** On the outer
    /// equator `P_θθ` and `P_φφ` are parallel and `P_θφ` vanishes, so a
    /// cell-half's hypotenuse midpoint deviates by exactly
    /// `(A·h_u² + C·h_v²)/8` to second order — the certificate, less
    /// the `O(h³)` mixed term the window's `max |sin|` still charges.
    /// The measured-to-certified ratio approaches 1 from below as the
    /// cell shrinks; a fudge factor on either side of the certificate
    /// moves it off 1 and reds this row.
    #[test]
    fn certificate_is_attained_on_the_outer_equator() {
        for (major, minor) in [(0.30, 0.07), (2.0, 0.5), (1.2, 1.0), (50.0, 1.0)] {
            let mut gaps = Vec::new();
            for h in [0.04, 0.02, 0.01] {
                let (hu, hv) = (h * 0.5, h * 1.5);
                let uv = [[0.0, -hv / 2.0], [hu, -hv / 2.0], [0.0, hv / 2.0]];
                let cert = cert_torus(major, minor, uv);
                let c = uv.map(|[u, v]| torus_point(major, minor, u, v));
                let mid = Point3::new(
                    (c[1].x + c[2].x) / 2.0,
                    (c[1].y + c[2].y) / 2.0,
                    (c[1].z + c[2].z) / 2.0,
                );
                let dev = torus_dist(major, minor, mid);
                let ratio = dev / cert;
                assert!(
                    ratio <= 1.0 && ratio > 0.95,
                    "R {major} r {minor} h {h}: hypotenuse-midpoint deviation is {ratio} of the \
                     certificate"
                );
                gaps.push(1.0 - ratio);
            }
            assert!(
                gaps[0] > gaps[1] && gaps[1] > gaps[2],
                "R {major} r {minor}: the gap to the certificate does not shrink with the cell: \
                 {gaps:?}"
            );
        }
    }

    /// **A spindle torus (`R < r`) certifies soundly.** `R + r·cos φ`
    /// changes sign across the inner lobe, and a certificate reading
    /// the SIGNED radial as `sup ‖P_θθ‖` under-reports there: on this
    /// window `R + r·cos_max = 0.2 − 0.989 < 0` would certify 0.0057
    /// against a sampled parametric deviation of 0.0201. The absolute
    /// sup makes the bound total for every `R, r > 0`. (The random
    /// sweep above draws `R/r ≥ 1.1`, ring tori, because its oracle is
    /// the locus distance; this row's is the parametric deviation.)
    #[test]
    fn cert_torus_is_sound_on_a_spindle_torus_across_the_inner_lobe() {
        let (major, minor) = (0.2, 1.0);
        let uv = [[0.0, PI - 0.15], [0.3, PI - 0.15], [0.0, PI + 0.15]];
        let cert = cert_torus(major, minor, uv);
        let dev = parametric_deviation(major, minor, uv);
        assert!(
            dev <= cert * (1.0 + 1e-9) + 1e-12,
            "spindle: sampled deviation {dev} exceeds certificate {cert}"
        );
        assert!(dev > 0.02, "the probe window is the one that bites ({dev})");
        let signed = (major + minor * (PI - 0.15).cos()).max(major + minor * (PI + 0.15).cos());
        let b = minor * abs_sin_max(PI - 0.15, PI + 0.15);
        let under = torus_chord_bound(signed, b, minor, 0.3, 0.3);
        assert!(
            signed < 0.0 && under < 0.5 * dev,
            "the signed radial ({under}) is no longer the under-reporting expression this \
             row exists for ({dev})"
        );
    }

    /// The window sups at their edge cases: an interval holding 2πk
    /// reads `A = R + r`, one holding an odd multiple of π reads
    /// `|R − r|` where that is the larger, and one holding π/2 + kπ
    /// reads `abs_sin_max = 1`, however far from the origin; otherwise
    /// an endpoint decides. A poisoned corner poisons the certificate.
    #[test]
    fn torus_window_sups_and_nan_stickiness() {
        let (r_big, r_small) = (2.0, 0.5);
        assert!(abs_radial_max(r_big, r_small, -0.1, 0.1) == 2.5);
        assert!(abs_radial_max(r_big, r_small, 6.0, 6.5) == 2.5);
        assert!(abs_radial_max(r_big, r_small, -13.0, -12.5) == 2.5);
        assert!(abs_radial_max(r_big, r_small, 0.1, 0.3) == 2.0 + 0.5 * 0.1_f64.cos());
        assert!(abs_radial_max(r_big, r_small, 3.0, 3.2) == 2.0 + 0.5 * 3.0_f64.cos());
        // Spindle (R < r): across φ = π the signed radial crosses zero
        // and the sup is `|R − r|` when that lies inside, else the
        // larger endpoint magnitude — never a signed value.
        assert!(abs_radial_max(0.2, 1.0, 3.0, 3.3) == 0.8);
        assert!(abs_radial_max(0.2, 1.0, 2.5, 2.8) == (0.2 + 2.8_f64.cos()).abs());
        assert!(abs_radial_max(0.2, 1.0, -0.2, 0.2) == 1.2);
        assert!(abs_sin_max(1.5, 1.6) == 1.0);
        assert!(abs_sin_max(-1.6, -1.5) == 1.0);
        assert!(abs_sin_max(4.6, 4.8) == 1.0);
        assert!((abs_sin_max(0.1, 0.3) - 0.3_f64.sin()).abs() == 0.0);
        assert!((abs_sin_max(3.0, 3.1) - 3.0_f64.sin().abs()).abs() == 0.0);
        assert!(cert_torus(1.0, 0.2, [[0.0, 0.0], [f64::NAN, 0.0], [0.0, 0.1]]).is_nan());
        assert!(cert_torus(1.0, 0.2, [[0.0, 0.0], [0.1, 0.0], [0.0, 0.1]]).is_finite());
    }
}
