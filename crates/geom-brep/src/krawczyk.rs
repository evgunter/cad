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

/// An upper bound on `|v|` over the enclosure, `NaN` where it does not
/// certify ([`Certification::mag`]).
pub(crate) fn mag(v: Interval) -> f64 {
    Certification::mag(v)
}

/// The midpoint of each enclosure of `v`.
pub(crate) fn midpoint(v: &[Interval; 3]) -> [f64; 3] {
    v.map(|c| c.lo() + (c.hi() - c.lo()) * 0.5)
}

/// The midpoint of each enclosure of `m`.
pub(crate) fn midpoints(m: &[[Interval; 3]; 3]) -> [[f64; 3]; 3] {
    m.map(|row| midpoint(&row))
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

/// **How far a solution in a box can lie from a point of it**, in the
/// scaled coordinates `diag(scale)·x`: for `e` in `X`, `f` an enclosure
/// of `F(e)` and `jac` of `J` over `X`, every solution `z` in `X` has
/// `‖diag(scale)·(z − e)‖∞` at most the bound returned. `F(z) − F(e) =
/// M·(z − e)` with `M` the mean of `J` along the segment, which lies in
/// `J(X)` entrywise as `X` is convex; with `Y` the inverse of `J`'s
/// midpoint and `S = diag(scale)`, `q = ‖I − S·Y·J(X)·S⁻¹‖∞ < 1` bounds
/// `‖(S·Y·M·S⁻¹)⁻¹‖∞` by `1/(1 − q)`, so `‖S·(z − e)‖∞ ≤
/// ‖S·Y·F(e)‖∞/(1 − q)`. Existence is not claimed: `e` may lie on `X`'s
/// side. `None` where `q` does not certify below one.
pub(crate) fn solution_reach(
    f: [Interval; 3],
    jac: &[[Interval; 3]; 3],
    scale: [f64; 3],
) -> Option<f64> {
    let y = inverse3(midpoints(jac))?;
    let sy: [[Interval; 3]; 3] = core::array::from_fn(|r| {
        core::array::from_fn(|c| Interval::point(scale[r]) * Interval::point(y[r][c]))
    });
    let (mut q, mut reach) = (0.0f64, 0.0f64);
    for (r, syr) in sy.iter().enumerate() {
        let mut row = Interval::point(0.0);
        for c in 0..3 {
            let mut v = Interval::point(if r == c { 1.0 } else { 0.0 });
            for k in 0..3 {
                v = v - syr[k] * jac[k][c] / Interval::point(scale[c]);
            }
            if !v.is_certified() {
                return None;
            }
            row = row + Interval::point(mag(v));
        }
        let syf = (0..3).fold(Interval::point(0.0), |acc, k| acc + syr[k] * f[k]);
        if !(row.is_certified() && syf.is_certified()) {
            return None;
        }
        q = q.max(row.hi());
        reach = reach.max(mag(syf));
    }
    if q >= 1.0 {
        return None;
    }
    let bound = Interval::point(reach) / (Interval::point(1.0) - Interval::point(q));
    bound.is_certified().then(|| bound.hi())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use geom_core::Interval;

    use super::solution_reach;

    /// `J(X) = diag([½, 3/2])`, its midpoint the identity, so `q = ½`.
    /// The member `½·I` puts the solution of `F(e) = (1, 0, 0)` at
    /// distance 2 from `e`, twice `‖Y·F(e)‖`: the bound must reach it.
    #[test]
    fn the_reach_holds_the_farthest_member_of_the_jacobian() {
        let o = Interval::from_bounds(0.0, 0.0);
        let d = Interval::from_bounds(0.5, 1.5);
        let jac = [[d, o, o], [o, d, o], [o, o, d]];
        let one = Interval::from_bounds(1.0, 1.0);
        let r = solution_reach([one, o, o], &jac, [1.0; 3]).expect("q = ½ contracts");
        assert!(
            r >= 2.0,
            "the solution at distance 2 is within the reach: {r}"
        );
        assert!(
            r < 2.0 + 1e-12,
            "and the reach is the bound's, not wider: {r}"
        );
    }

    /// `J(X) = diag([−½, 5/2])`, its midpoint the identity, so `q = 3/2`:
    /// a singular member, so no reach.
    #[test]
    fn no_reach_where_the_jacobian_holds_a_singular_member() {
        let o = Interval::from_bounds(0.0, 0.0);
        let d = Interval::from_bounds(-0.5, 2.5);
        let jac = [[d, o, o], [o, d, o], [o, o, d]];
        let one = Interval::from_bounds(1.0, 1.0);
        let r = solution_reach([one, o, o], &jac, [1.0; 3]);
        assert!(r.is_none(), "q > 1 bounds nothing: {r:?}");
    }
}
