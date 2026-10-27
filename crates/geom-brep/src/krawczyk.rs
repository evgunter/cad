//! **The Krawczyk operator on a square system in ℝ³**, in certification
//! arithmetic: the pieces the SSI lane's boxes in ℝ⁴ (a face held, three
//! free coordinates) and the curve × face table's spline-face arm (the
//! state `(t, u, v)`) share.
//!
//! For `F: ℝ³ → ℝ³` over a box `X` with midpoint `m`, `J(X)` an enclosure
//! of `F`'s Jacobian over `X` and `Y` any real matrix (the inverse of
//! `J`'s midpoint, which only preconditions),
//! `K = m − Y·F(m) + (I − Y·J(X))·(X − m)` holds every solution in `X`;
//! where `K` lies strictly inside `X`, `X` holds exactly one, and every
//! matrix in `J(X)` is invertible.

use geom_core::interval::certification::Certification;
use geom_core::{Bounds, Interval};

/// The determinant of a 3×3 interval matrix.
pub(crate) fn det3(m: [[Interval; 3]; 3]) -> Interval {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

/// The inverse of a 3×3 matrix, `None` where it is singular or not
/// finite.
pub(crate) fn inverse3(m: [[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    let c =
        |r0: usize, r1: usize, c0: usize, c1: usize| m[r0][c0] * m[r1][c1] - m[r0][c1] * m[r1][c0];
    let det = m[0][0] * c(1, 2, 1, 2) - m[0][1] * c(1, 2, 0, 2) + m[0][2] * c(1, 2, 0, 1);
    if !det.is_finite() || det == 0.0 {
        return None;
    }
    let inv = [
        [c(1, 2, 1, 2), -c(0, 2, 1, 2), c(0, 1, 1, 2)],
        [-c(1, 2, 0, 2), c(0, 2, 0, 2), -c(0, 1, 0, 2)],
        [c(1, 2, 0, 1), -c(0, 2, 0, 1), c(0, 1, 0, 1)],
    ];
    let out = inv.map(|row| row.map(|v| v / det));
    out.iter().flatten().all(|v| v.is_finite()).then_some(out)
}

/// The midpoint of each enclosure of `m`.
pub(crate) fn midpoints(m: &[[Interval; 3]; 3]) -> [[f64; 3]; 3] {
    m.map(|row| row.map(|v| v.lo() + (v.hi() - v.lo()) * 0.5))
}

/// **The operator's image** of the box `x` (module docs), `f` an
/// enclosure of `F` at `x`'s midpoint and `jac` of `J` over `x`. `None`
/// where `J`'s midpoint is singular or the image does not certify.
pub(crate) fn image(
    x: [(f64, f64); 3],
    f: [Interval; 3],
    jac: &[[Interval; 3]; 3],
) -> Option<[(f64, f64); 3]> {
    let m = x.map(|(a, b)| a + (b - a) * 0.5);
    let y = inverse3(midpoints(jac))?;
    let d: [Interval; 3] =
        core::array::from_fn(|k| Interval::from_bounds(x[k].0, x[k].1) - Interval::point(m[k]));
    let mut out = x;
    for r in 0..3 {
        let mut v = Interval::point(m[r]);
        for c in 0..3 {
            v = v - Interval::point(y[r][c]) * f[c];
        }
        for c in 0..3 {
            let mut yj = Interval::point(if r == c { 1.0 } else { 0.0 });
            for q in 0..3 {
                yj = yj - Interval::point(y[r][q]) * jac[q][c];
            }
            v = v + yj * d[c];
        }
        if !v.is_certified() {
            return None;
        }
        out[r] = (v.lo(), v.hi());
    }
    Some(out)
}

/// `|f|`, enclosed.
pub(crate) fn norm(f: [Interval; 3]) -> Interval {
    (f[0].powi(2) + f[1].powi(2) + f[2].powi(2)).sqrt()
}

/// **The meet of two enclosures of one vector**, coordinate by
/// coordinate ([`Certification::meet`]): `None` where two certified
/// enclosures share no point, which no two sound enclosures of one value
/// can, so an argument behind one of them is wrong.
pub(crate) fn meet3(a: [Interval; 3], b: [Interval; 3]) -> Option<[Interval; 3]> {
    let mut out = a;
    for k in 0..3 {
        if !(a[k].is_certified() && b[k].is_certified()) {
            continue;
        }
        let m = a[k].meet(b[k]);
        if !m.is_certified() {
            return None;
        }
        out[k] = m;
    }
    Some(out)
}
