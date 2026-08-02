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
//! - **Torus** (R, r): linear-interpolation error against the chart
//!   Hessian bound M = R + 2r (|S_uu| ≤ R + r, |S_uv| ≤ r, |S_vv| = r
//!   ⇒ |D²S(w,w)| ≤ (R+2r)|w|²): with the affine Taylor expansion at
//!   the longest-UV-edge midpoint every triangle point is within
//!   (√3/2)·L of the expansion point, so ‖S − ΠS‖ ≤ 2·(M/2)·(3/4)L² =
//!   (3/4)·M·L². Certificate `(3/4)(R+2r)·L_uv²`.
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

/// Torus certificate: `(3/4)(R + 2r) · L_uv²` with L the longest UV
/// edge (module docs).
pub fn cert_torus(major: f64, minor: f64, uv: [[f64; 2]; 3]) -> f64 {
    let edge2 = |i: usize, j: usize| -> f64 {
        let du = uv[i][0] - uv[j][0];
        let dv = uv[i][1] - uv[j][1];
        du.powi(2) + dv.powi(2)
    };
    let l2 = edge2(0, 1).max(edge2(1, 2)).max(edge2(2, 0));
    0.75 * (major + 2.0 * minor) * l2
}
