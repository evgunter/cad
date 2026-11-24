//! **The point of a set's convex hull nearest the origin**, at `f64`:
//! a direction a certificate then reads for itself (a spline patch's
//! norm cover, `topo`'s plane through two wedges at a vertex).

/// **The iterates of Wolfe's min-norm-point algorithm on `points`**
/// (P. Wolfe, "Finding the nearest point in a polytope", 1976): each
/// major iterate is the nearest point of the affine hull of a corral of
/// at most four of the points, and the last is the convex hull's point
/// nearest the origin, up to the arithmetic. Every step commutes with a
/// rotation of the points. Capped at 64 major iterates; empty for no
/// points.
///
/// At `f64`, so its callers take only a DIRECTION from it and certify
/// their own reading along that direction: an iterate that rounding
/// leaves short of the optimum costs tightness, never soundness.
#[must_use]
pub fn min_norm_point_iterates(points: &[[f64; 3]]) -> Vec<[f64; 3]> {
    let dot = |p: &[f64; 3], q: &[f64; 3]| p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
    let combine = |set: &[usize], w: &[f64]| -> [f64; 3] {
        core::array::from_fn(|i| set.iter().zip(w).map(|(&k, a)| a * points[k][i]).sum())
    };
    let scale = points.iter().map(|p| dot(p, p)).fold(0.0f64, f64::max);
    let Some(first) = (0..points.len())
        .min_by(|&a, &b| dot(&points[a], &points[a]).total_cmp(&dot(&points[b], &points[b])))
    else {
        return Vec::new();
    };
    let mut set = vec![first];
    let mut w = vec![1.0];
    let mut x = points[first];
    let mut out = vec![x];
    for _ in 0..64 {
        let Some(j) =
            (0..points.len()).min_by(|&a, &b| dot(&points[a], &x).total_cmp(&dot(&points[b], &x)))
        else {
            break;
        };
        // Optimal: no point lies further toward the origin than `x`.
        if dot(&x, &x) - dot(&points[j], &x) <= 1e-15 * scale || set.contains(&j) {
            break;
        }
        set.push(j);
        w.push(0.0);
        // Minor cycles: move toward the corral's affine minimizer,
        // dropping the points whose weight it drives to zero.
        for _ in 0..8 {
            let Some(lambda) = affine_minimizer(points, &set) else {
                break;
            };
            if lambda.iter().all(|&l| l > 0.0) {
                w = lambda;
                break;
            }
            let theta = w
                .iter()
                .zip(&lambda)
                .filter(|(_, l)| **l <= 0.0)
                .map(|(a, l)| a / (a - l))
                .fold(1.0f64, f64::min);
            for (a, l) in w.iter_mut().zip(&lambda) {
                *a = (1.0 - theta) * *a + theta * l;
            }
            let kept: Vec<(usize, f64)> = set
                .iter()
                .zip(&w)
                .filter(|(_, a)| **a > 1e-14)
                .map(|(&k, &a)| (k, a))
                .collect();
            let total: f64 = kept.iter().map(|(_, a)| a).sum();
            if kept.is_empty() || total.is_nan() || total <= 0.0 {
                return out;
            }
            set = kept.iter().map(|(k, _)| *k).collect();
            w = kept.iter().map(|(_, a)| a / total).collect();
        }
        let next = combine(&set, &w);
        if !next.iter().all(|v| v.is_finite()) || dot(&next, &next) >= dot(&x, &x) {
            break;
        }
        x = next;
        out.push(x);
    }
    out
}

/// The weights `λ` (summing to 1) of the point of `set`'s affine hull
/// nearest the origin, from the bordered Gram system; `None` when it is
/// singular (the corral is affinely dependent).
fn affine_minimizer(points: &[[f64; 3]], set: &[usize]) -> Option<Vec<f64>> {
    let n = set.len();
    let dot = |p: &[f64; 3], q: &[f64; 3]| p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
    // [G 1; 1ᵀ 0] [λ; μ] = [0; 1], solved by elimination with pivoting.
    let m = n + 1;
    let mut a = vec![vec![0.0f64; m + 1]; m];
    for (r, &i) in set.iter().enumerate() {
        for (c, &j) in set.iter().enumerate() {
            a[r][c] = dot(&points[i], &points[j]);
        }
        a[r][n] = 1.0;
        a[n][r] = 1.0;
    }
    a[n][m] = 1.0;
    let norm = a
        .iter()
        .flat_map(|r| r.iter())
        .fold(0.0f64, |s, v| s.max(v.abs()));
    for col in 0..m {
        let piv = (col..m).max_by(|&p, &q| a[p][col].abs().total_cmp(&a[q][col].abs()))?;
        if a[piv][col].abs().is_nan() || a[piv][col].abs() <= 1e-13 * norm {
            return None;
        }
        a.swap(col, piv);
        let pivot_row = a[col].clone();
        for (r, row) in a.iter_mut().enumerate() {
            if r != col {
                let f = row[col] / pivot_row[col];
                for (cell, p) in row.iter_mut().zip(&pivot_row).skip(col) {
                    *cell -= f * p;
                }
            }
        }
    }
    let lambda: Vec<f64> = (0..n).map(|r| a[r][m] / a[r][r]).collect();
    lambda.iter().all(|l| l.is_finite()).then_some(lambda)
}

/// The convex hull of `points`' point nearest the origin: the last of
/// [`min_norm_point_iterates`], `None` for no points.
#[must_use]
pub fn min_norm_point(points: &[[f64; 3]]) -> Option<[f64; 3]> {
    min_norm_point_iterates(points).last().copied()
}
