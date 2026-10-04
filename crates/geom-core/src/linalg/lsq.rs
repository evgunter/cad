//! Dense small least squares for the fitting systems (M5 PR 4, C12.8's
//! first consumer) — the Book's Eqs. 9.65–9.67 shape (`NᵀN`-style normal
//! equations) plus the square collocation solve interpolation needs.
//!
//! # This is C6's f64 lane — structure machinery only
//!
//! Everything here is `f64`: these solves *select cache structure*
//! (control points of fitted curves), they never decide topology and
//! never instantiate at a generic scalar. Raw `f64` comparisons below
//! are structure selection under C6, not predicate work.
//!
//! # D9 posture (binding, per the linalg charter)
//!
//! - **Fixed elimination order, no magnitude pivoting.** The fitting
//!   systems have fixed shapes (collocation and normal matrices of
//!   B-spline bases); elimination runs in ascending index order,
//!   always. A numerically-degenerate system is the typed
//!   [`LsqError::LsqDegenerate`] refusal — never a reorder, never a
//!   best-effort answer.
//! - **Fixed association.** Every reduction accumulates in ascending
//!   index order exactly as written; the orders are stated per
//!   function.
//! - **No allocation surprises.** Plain `Vec`s sized from the inputs;
//!   no external BLAS (nondeterminism). The interpolating fit's
//!   collocation matrix is banded and can be large (a marched SSI
//!   branch runs to thousands of samples), so it has its own banded
//!   LU ([`factor_banded`], one factorisation for any number of
//!   right-hand sides); every other square system stays on the dense
//!   [`solve_square`].
//! - **Totality.** No panics: internal indexing is justified by the
//!   validated shapes; malformed shapes are typed refusals; non-finite
//!   input reaches the pivot tests as NaN and refuses there
//!   (fail-loud, D4 ¶2).

use crate::readable::Readable;

/// A typed least-squares refusal (fail-loud; the kernel never panics).
#[derive(Clone, Debug, PartialEq)]
pub enum LsqError {
    /// The system is numerically degenerate under the FIXED elimination
    /// order: a Cholesky pivot that is not strictly positive, or an LU
    /// pivot that is zero — including the NaN both become on non-finite
    /// input. Refusal, never a reorder (D9: no magnitude pivoting).
    LsqDegenerate {
        /// Index of the offending pivot (elimination step).
        pivot_index: usize,
        /// The offending pivot value (NaN when input was poisoned).
        pivot: f64,
    },
    /// A matrix row's length differs from the first row's.
    RowLengthMismatch {
        /// Index of the offending row.
        row: usize,
        /// Its length.
        len: usize,
        /// The length required (from row 0).
        expected: usize,
    },
    /// The right-hand-side row count differs from the matrix row count,
    /// or an RHS row's width differs from the first RHS row's.
    RhsShapeMismatch {
        /// RHS rows supplied.
        rhs_rows: usize,
        /// Matrix rows they must match.
        matrix_rows: usize,
    },
    /// The row/column shape rules out the requested solve. For
    /// [`solve_normal`] this is the literal underdetermined case
    /// (`rows < cols`: no unique minimizer — typed refusal rather than
    /// a pseudo-inverse guess). **[`solve_square`] reuses the variant
    /// for ANY non-square matrix, `rows > cols` included** — read it
    /// there as "not the square shape this solve requires", with the
    /// offending dimensions carried.
    Underdetermined {
        /// Equation rows.
        rows: usize,
        /// Unknown columns.
        cols: usize,
    },
    /// A banded matrix ([`factor_banded`]) whose row windows do not
    /// have the shape its LU needs: a window that starts left of the
    /// previous row's (the first-nonzero property, which keeps the
    /// factor's fill inside every window), a window that runs past the
    /// last column, or a window list whose length differs from the row
    /// count.
    BandShape {
        /// The first row whose window breaks the shape.
        row: usize,
    },
    /// An empty matrix (no rows, or rows of zero length).
    Empty,
}

impl core::fmt::Display for LsqError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LsqError::LsqDegenerate { pivot_index, pivot } => write!(
                f,
                "lsq: degenerate system (pivot {pivot_index} = {} under the fixed elimination \
                 order). Recourse: supply rows that determine the unknowns — a zero pivot \
                 means two of them do not, and a non-finite one means a value upstream is not \
                 a number",
                Readable(*pivot)
            ),
            LsqError::RowLengthMismatch { row, len, expected } => write!(
                f,
                "lsq: row {row} has length {len}, expected {expected}. Recourse: supply rows \
                 of one length"
            ),
            LsqError::RhsShapeMismatch {
                rhs_rows,
                matrix_rows,
            } => write!(
                f,
                "lsq: rhs has {rhs_rows} rows, matrix has {matrix_rows}. Recourse: supply one \
                 right-hand-side row per matrix row, all of one width"
            ),
            LsqError::Underdetermined { rows, cols } => write!(
                f,
                "lsq: underdetermined ({rows} rows < {cols} cols). Recourse: supply at least \
                 as many rows as columns — this solve has no unique minimizer below that, and \
                 reordering for a pivot is not offered (D9)"
            ),
            LsqError::BandShape { row } => write!(
                f,
                "lsq: row {row}'s band window does not fit the banded LU (it starts left of \
                 the previous row's, runs past the last column, or has no row). Recourse: \
                 supply one window per row, each starting no earlier than the one before and \
                 ending inside the matrix"
            ),
            LsqError::Empty => f.write_str(
                "lsq: empty system. Recourse: supply a matrix with at least one row of \
                 nonzero length",
            ),
        }
    }
}

impl core::error::Error for LsqError {}

/// Validates the `rows × ?` matrix shape: nonempty, rectangular.
/// Returns the column count.
fn check_matrix(rows: &[Vec<f64>]) -> Result<usize, LsqError> {
    let Some(first) = rows.first() else {
        return Err(LsqError::Empty);
    };
    let n = first.len();
    if n == 0 {
        return Err(LsqError::Empty);
    }
    for (row, r) in rows.iter().enumerate() {
        if r.len() != n {
            return Err(LsqError::RowLengthMismatch {
                row,
                len: r.len(),
                expected: n,
            });
        }
    }
    Ok(n)
}

/// Validates the RHS against the matrix rows; returns the RHS width.
fn check_rhs(rhs: &[Vec<f64>], matrix_rows: usize) -> Result<usize, LsqError> {
    if rhs.len() != matrix_rows {
        return Err(LsqError::RhsShapeMismatch {
            rhs_rows: rhs.len(),
            matrix_rows,
        });
    }
    check_matrix(rhs)
}

/// Least-squares solve `min ‖A·x − b‖₂` per RHS column, via the normal
/// equations `AᵀA·x = Aᵀb` and a **fixed-order Cholesky** (`L·Lᵀ`,
/// ascending pivot index, no reordering) — the Eqs. 9.65–9.67 shape.
///
/// `a` is the `m × n` matrix as `m` rows; `b` the `m × k` RHS as `m`
/// rows; the result is the `n × k` solution as `n` rows. Association
/// orders (D9, fixed): `(AᵀA)ᵢⱼ = Σ_r aᵣᵢ·aᵣⱼ` ascending `r`;
/// `(Aᵀb)ᵢ = Σ_r aᵣᵢ·bᵣ` ascending `r`; each Cholesky inner product
/// `Σ_{q<j} Lᵢq·Lⱼq` ascending `q`; substitution sums ascending
/// (forward) / descending (backward) index, term order as written.
/// `sqrt` is IEEE-correctly-rounded (bit-identical everywhere, D9).
///
/// # Errors
///
/// [`LsqError`] on shape violations; [`LsqError::LsqDegenerate`] when a
/// pivot `d` fails `d > 0` (rank deficiency at fp — the NaN of poisoned
/// input fails it too and refuses loudly).
// `!(d > 0)` is deliberate (NaN-catching — NaN refuses; `d <= 0` would
// pass NaN); index loops are kept explicit because the fixed
// elimination order IS the contract here — iterator adaptors would
// obscure exactly the thing D9 pins.
#[allow(clippy::neg_cmp_op_on_partial_ord, clippy::needless_range_loop)]
pub fn solve_normal(a: &[Vec<f64>], b: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, LsqError> {
    let n = check_matrix(a)?;
    let m = a.len();
    if m < n {
        return Err(LsqError::Underdetermined { rows: m, cols: n });
    }
    let k = check_rhs(b, m)?;

    // Normal matrix G = AᵀA (dense n × n, lower triangle used) and
    // H = Aᵀb (n × k), both accumulated ascending row index r.
    let mut g = vec![0.0f64; n * n];
    let mut h = vec![vec![0.0f64; k]; n];
    for (ar, br) in a.iter().zip(b.iter()) {
        // Ascending row order r = 0..m (zip preserves it).
        for i in 0..n {
            for j in 0..=i {
                g[i * n + j] += ar[i] * ar[j];
            }
            for c in 0..k {
                h[i][c] += ar[i] * br[c];
            }
        }
    }

    // Fixed-order Cholesky G = L·Lᵀ, in place in the lower triangle.
    for j in 0..n {
        let mut d = g[j * n + j];
        for q in 0..j {
            d -= g[j * n + q] * g[j * n + q];
        }
        if !(d > 0.0) || !d.is_finite() {
            return Err(LsqError::LsqDegenerate {
                pivot_index: j,
                pivot: d,
            });
        }
        let l = d.sqrt();
        g[j * n + j] = l;
        for i in (j + 1)..n {
            let mut s = g[i * n + j];
            for q in 0..j {
                s -= g[i * n + q] * g[j * n + q];
            }
            g[i * n + j] = s / l;
        }
    }

    // L·y = H (forward), then Lᵀ·x = y (backward), per RHS column.
    let mut x = h;
    for c in 0..k {
        for i in 0..n {
            let mut s = x[i][c];
            for q in 0..i {
                s -= g[i * n + q] * x[q][c];
            }
            x[i][c] = s / g[i * n + i];
        }
        for i in (0..n).rev() {
            let mut s = x[i][c];
            for q in (i + 1)..n {
                s -= g[q * n + i] * x[q][c];
            }
            x[i][c] = s / g[i * n + i];
        }
    }
    Ok(x)
}

/// Exact solve of a **square** system `A·x = b` per RHS column, via
/// fixed-order Doolittle LU — **no pivoting** (D9: the collocation
/// matrices this serves have their shape fixed by the knot/parameter
/// structure; a zero pivot under the fixed order is the typed
/// [`LsqError::LsqDegenerate`] refusal, never a row swap).
///
/// `a` is `n × n` as rows; `b` is `n × k`; result `n × k`. Association:
/// every inner product `Σ_{q<i} Lᵢq·Uqⱼ` ascending `q`; substitutions
/// ascending/descending as in [`solve_normal`].
///
/// # Errors
///
/// [`LsqError`] on shape violations (a non-square matrix refuses as
/// [`LsqError::Underdetermined`] or [`LsqError::RowLengthMismatch`]);
/// [`LsqError::LsqDegenerate`] on a zero or non-finite pivot.
// Explicit index loops on purpose — see `solve_normal`'s note.
#[allow(clippy::needless_range_loop)]
pub fn solve_square(a: &[Vec<f64>], b: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, LsqError> {
    let n = check_matrix(a)?;
    if a.len() != n {
        return Err(LsqError::Underdetermined {
            rows: a.len(),
            cols: n,
        });
    }
    let k = check_rhs(b, n)?;

    // Doolittle in place: lu holds U on/above the diagonal, L (unit
    // diagonal implied) below. Fixed ascending elimination order.
    let mut lu = vec![0.0f64; n * n];
    for (i, row) in a.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            lu[i * n + j] = *v;
        }
    }
    for i in 0..n {
        // U row i (columns i..n): subtract the L·U prefix, ascending q.
        for j in i..n {
            let mut s = lu[i * n + j];
            for q in 0..i {
                s -= lu[i * n + q] * lu[q * n + j];
            }
            lu[i * n + j] = s;
        }
        let pivot = lu[i * n + i];
        if pivot == 0.0 || !pivot.is_finite() {
            return Err(LsqError::LsqDegenerate {
                pivot_index: i,
                pivot,
            });
        }
        // L column i (rows i+1..n).
        for r in (i + 1)..n {
            let mut s = lu[r * n + i];
            for q in 0..i {
                s -= lu[r * n + q] * lu[q * n + i];
            }
            lu[r * n + i] = s / pivot;
        }
    }

    // L·y = b (forward, unit diagonal), U·x = y (backward), per column.
    let mut x: Vec<Vec<f64>> = b.to_vec();
    for c in 0..k {
        for i in 0..n {
            let mut s = x[i][c];
            for q in 0..i {
                s -= lu[i * n + q] * x[q][c];
            }
            x[i][c] = s;
        }
        for i in (0..n).rev() {
            let mut s = x[i][c];
            for q in (i + 1)..n {
                s -= lu[i * n + q] * x[q][c];
            }
            x[i][c] = s / lu[i * n + i];
        }
    }
    Ok(x)
}

/// The no-pivot LU of a banded square matrix, factored once by
/// [`factor_banded`] and solved against any number of right-hand sides
/// by [`BandedLu::solve`].
///
/// # Shape
///
/// Row `i` holds its structural nonzeros in the window of columns
/// `first[i] .. first[i] + width`, and `first` never decreases from one
/// row to the next (the **first-nonzero property**). A B-spline
/// collocation matrix at ascending parameters has it: row `i` is the
/// `p + 1` basis functions alive at `ū_i`, starting at its span's first
/// control index, and the span never moves left as `ū` rises. Under the
/// property Doolittle without pivoting creates no fill outside the
/// windows: `L[i][q]` is zero for `q < first[i]`, and `U[i][j]` is zero
/// for `j ≥ first[i] + width`, because every row above `i` ends its
/// window no later than row `i` does. So the factor is stored in the
/// windows themselves, `n · width` values, and costs `O(n · width²)`.
///
/// # Why no pivoting is safe here
///
/// A B-spline collocation matrix whose parameters satisfy the
/// Schoenberg–Whitney conditions (`u_j < ū_j < u_{j+p+1}` inside the
/// knot range, `N_j(ū_j) ≠ 0`) is totally positive: C. de Boor, *Total
/// positivity of the spline collocation matrix*, Indiana Univ. Math. J.
/// 25 (1976) 541–551. The averaged knot vector the fit builds (Eq. 9.8,
/// `u_{j+p}` the mean of `ū_j … ū_{j+p−1}`) satisfies them at strictly
/// ascending parameters: `u_j` is a mean of parameters below `ū_j` and
/// `u_{j+p+1}` a mean of parameters above it. Gaussian elimination
/// without pivoting on a totally positive matrix computes nonnegative
/// `L` and `U` factors for small enough unit roundoff, so nothing
/// cancels and the backward error is small with no pivoting at all:
/// C. de Boor and A. Pinkus, *Backward error analysis for totally
/// positive linear systems*, Numer. Math. 27 (1977) 485–490, whose
/// stated application is exactly this banded B-spline system.
///
/// # Identity with [`solve_square`]
///
/// The factorisation is [`solve_square`]'s Doolittle with every loop
/// restricted to the windows: each stored entry is formed from the same
/// terms in the same ascending order, less terms that are an exact
/// `±0 · x`. Dropping such a term changes a running sum only when that
/// sum is `-0.0` (`-0.0 − (−0.0)` is `+0.0`), and a sum that starts at a
/// matrix entry other than `-0.0` never becomes `-0.0` under
/// subtraction, so with no `-0.0` entry (true of basis values, which are
/// sums and products of nonnegative numbers) and finite factors the
/// factor is bit-identical to the dense one. The substitutions start
/// from the right-hand side, which may hold `-0.0`, so [`Self::solve`]
/// applies the dense solve's dropped terms in summary: a `-0.0` sum
/// becomes `+0.0` when any dropped product carries a sign bit, and a
/// dropped product against a non-finite value is applied as it stands.
/// The solution is then bit-identical to [`solve_square`]'s on the same
/// matrix in dense form, NaN where it is NaN.
#[derive(Clone, Debug)]
pub struct BandedLu {
    first: Vec<usize>,
    width: usize,
    /// `L` below the diagonal (unit diagonal implied) and `U` on and
    /// above it, row `i`'s window at `i * width ..`, column `j` at
    /// offset `j − first[i]`.
    lu: Vec<f64>,
}

/// Factor the banded square matrix whose row `i` is `rows[i]`, standing
/// in columns `first[i] .. first[i] + rows[i].len()` — [`BandedLu`] for
/// the shape it needs and why it neither pivots nor fills.
///
/// Association: every entry's inner product `Σ_q Lᵢq·Uqⱼ` runs ascending
/// `q` over the structural nonzeros, which is [`solve_square`]'s order
/// with its exact-zero terms left out; the elimination order is fixed
/// and ascending.
///
/// # Errors
///
/// [`LsqError::Empty`] and [`LsqError::RowLengthMismatch`] for the rows;
/// [`LsqError::BandShape`] when `first` breaks the shape;
/// [`LsqError::LsqDegenerate`] on a zero or non-finite pivot, at the
/// index [`solve_square`] would name. A row whose window leaves out its
/// own diagonal makes the matrix singular (the rows from there down, or
/// up, outnumber the columns they can reach), and refuses at that row
/// with the pivot `0.0` the dense elimination computes there.
// Explicit index loops on purpose — see `solve_normal`'s note.
#[allow(clippy::needless_range_loop)]
pub fn factor_banded(first: &[usize], rows: &[Vec<f64>]) -> Result<BandedLu, LsqError> {
    let width = check_matrix(rows)?;
    let n = rows.len();
    if first.len() != n {
        return Err(LsqError::BandShape {
            row: first.len().min(n),
        });
    }
    for i in 0..n {
        let rises = i == 0 || first[i - 1] <= first[i];
        if !rises || first[i] + width > n {
            return Err(LsqError::BandShape { row: i });
        }
    }
    let mut lu: Vec<f64> = rows.iter().flatten().copied().collect();
    for i in 0..n {
        let f = first[i];
        if i < f || i >= f + width {
            return Err(LsqError::LsqDegenerate {
                pivot_index: i,
                pivot: 0.0,
            });
        }
        let row = i * width;
        // `U[t][j]` is stored when `j < first[t] + width`; past that it
        // is a structural zero.
        for q in f..i {
            let mut s = lu[row + q - f];
            for t in f..q {
                if q < first[t] + width {
                    s -= lu[row + t - f] * lu[t * width + q - first[t]];
                }
            }
            lu[row + q - f] = s / lu[q * width + q - first[q]];
        }
        for j in i..f + width {
            let mut s = lu[row + j - f];
            for t in f..i {
                if j < first[t] + width {
                    s -= lu[row + t - f] * lu[t * width + j - first[t]];
                }
            }
            lu[row + j - f] = s;
        }
        let pivot = lu[row + i - f];
        if pivot == 0.0 || !pivot.is_finite() {
            return Err(LsqError::LsqDegenerate {
                pivot_index: i,
                pivot,
            });
        }
    }
    Ok(BandedLu {
        first: first.to_vec(),
        width,
        lu,
    })
}

impl BandedLu {
    /// The matrix order `n`.
    pub fn order(&self) -> usize {
        self.first.len()
    }

    /// Solve `A·x = b` per right-hand-side column against this
    /// factorisation; `b` is `n × k`, the result `n × k`. Forward
    /// substitution sums ascending, back substitution descending, as in
    /// [`solve_square`], with whose result this one is bit-identical
    /// ([`BandedLu`]).
    ///
    /// # Errors
    ///
    /// [`LsqError::RhsShapeMismatch`], [`LsqError::RowLengthMismatch`]
    /// and [`LsqError::Empty`] for a malformed `b`.
    // Explicit index loops on purpose — see `solve_normal`'s note.
    #[allow(clippy::needless_range_loop)]
    pub fn solve(&self, b: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, LsqError> {
        let n = self.order();
        let w = self.width;
        let k = check_rhs(b, n)?;
        let at = |i: usize, j: usize| self.lu[i * w + j - self.first[i]];
        let mut x: Vec<Vec<f64>> = b.to_vec();
        for c in 0..k {
            // `L[i][q]` for `q < first[i]` is `+0.0 / U[q][q]`: a zero
            // carrying the pivot's sign. The dense sum subtracts it
            // times `y_q` BEFORE the window's terms; `dropped` is the
            // rows summarised so far, `flips` whether one of those
            // products is `-0.0`, `poison` the first that is NaN.
            let (mut dropped, mut flips, mut poison) = (0usize, false, None);
            for i in 0..n {
                let f = self.first[i];
                while dropped < f {
                    let y = x[dropped][c];
                    let l = if at(dropped, dropped) < 0.0 { -0.0 } else { 0.0 };
                    if !y.is_finite() {
                        poison = poison.or(Some((l, y)));
                    } else if (l * y).is_sign_negative() {
                        flips = true;
                    }
                    dropped += 1;
                }
                let mut s = x[i][c];
                if let Some((l, y)) = poison {
                    s -= l * y;
                } else if flips {
                    s -= -0.0;
                }
                for q in f..i {
                    s -= at(i, q) * x[q][c];
                }
                x[i][c] = s;
            }
            // `U[i][j]` for `j ≥ first[i] + width` is `+0.0`; the dense
            // sum subtracts it times `x_j` AFTER the window's terms, in
            // ascending `j`, so the first NaN it meets is the lowest.
            let (mut kept, mut flips, mut poison) = (n, false, None);
            for i in (0..n).rev() {
                let end = self.first[i] + w;
                while kept > end {
                    kept -= 1;
                    let v = x[kept][c];
                    if !v.is_finite() {
                        poison = Some(v);
                    } else if v.is_sign_negative() {
                        flips = true;
                    }
                }
                let mut s = x[i][c];
                for q in (i + 1)..end {
                    s -= at(i, q) * x[q][c];
                }
                if let Some(v) = poison {
                    s -= 0.0 * v;
                } else if flips {
                    s -= -0.0;
                }
                x[i][c] = s / at(i, i);
            }
        }
        Ok(x)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn square_solve_matches_hand_computed_inverse() {
        // [2 1; 1 3]·x = [5; 10] → x = (5·3 − 10)/5 = 1, y = 3.
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let b = vec![vec![5.0], vec![10.0]];
        let x = solve_square(&a, &b).unwrap();
        assert!((x[0][0] - 1.0).abs() < 1e-14);
        assert!((x[1][0] - 3.0).abs() < 1e-14);
    }

    #[test]
    fn normal_solve_recovers_exact_solution_and_orthogonal_residual() {
        // Overdetermined 4×2 with an exactly-consistent RHS column and a
        // noisy one; the noisy column's residual must be ⊥ col(A).
        let a = vec![
            vec![1.0, 0.0],
            vec![1.0, 1.0],
            vec![1.0, 2.0],
            vec![1.0, 3.0],
        ];
        let b = vec![
            vec![2.0, 1.9],
            vec![3.0, 3.2],
            vec![4.0, 3.9],
            vec![5.0, 5.1],
        ];
        let x = solve_normal(&a, &b).unwrap();
        // Column 0: exact line 2 + t.
        assert!((x[0][0] - 2.0).abs() < 1e-12 && (x[1][0] - 1.0).abs() < 1e-12);
        // Column 1: Aᵀ(A·x − b) ≈ 0 (the normal-equations optimality).
        for j in 0..2 {
            let mut dot = 0.0;
            for (ar, br) in a.iter().zip(b.iter()) {
                let r = ar[0] * x[0][1] + ar[1] * x[1][1] - br[1];
                dot += ar[j] * r;
            }
            assert!(dot.abs() < 1e-12, "residual not orthogonal: {dot:e}");
        }
    }

    #[test]
    fn degenerate_systems_refuse_with_pivot_info() {
        // Duplicate columns → AᵀA rank 1 → second Cholesky pivot ≤ 0.
        let a = vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![3.0, 3.0]];
        let b = vec![vec![1.0], vec![2.0], vec![3.0]];
        match solve_normal(&a, &b) {
            Err(LsqError::LsqDegenerate { pivot_index: 1, .. }) => {}
            other => panic!("expected LsqDegenerate at pivot 1, got {other:?}"),
        }
        // Singular square system, fixed order: zero pivot refused, never
        // reordered (a magnitude-pivoting solver would succeed here).
        let a = vec![vec![0.0, 1.0], vec![1.0, 0.0]];
        let b = vec![vec![1.0], vec![1.0]];
        match solve_square(&a, &b) {
            Err(LsqError::LsqDegenerate {
                pivot_index: 0,
                pivot,
            }) => {
                assert_eq!(pivot, 0.0);
            }
            other => panic!("expected LsqDegenerate at pivot 0, got {other:?}"),
        }
    }

    #[test]
    fn poisoned_input_refuses_rather_than_answering() {
        let a = vec![vec![f64::NAN, 0.0], vec![0.0, 1.0], vec![1.0, 1.0]];
        let b = vec![vec![1.0], vec![1.0], vec![1.0]];
        match solve_normal(&a, &b) {
            Err(LsqError::LsqDegenerate { pivot, .. }) => assert!(pivot.is_nan()),
            other => panic!("expected NaN-pivot refusal, got {other:?}"),
        }
    }

    #[test]
    fn shape_violations_are_typed() {
        assert_eq!(solve_normal(&[], &[]), Err(LsqError::Empty));
        let a = vec![vec![1.0, 2.0], vec![1.0]];
        let b = vec![vec![1.0], vec![1.0]];
        assert_eq!(
            solve_normal(&a, &b),
            Err(LsqError::RowLengthMismatch {
                row: 1,
                len: 1,
                expected: 2
            })
        );
        let a = vec![vec![1.0, 2.0]];
        let b = vec![vec![1.0]];
        assert_eq!(
            solve_normal(&a, &b),
            Err(LsqError::Underdetermined { rows: 1, cols: 2 })
        );
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        assert_eq!(
            solve_square(&a, &[vec![1.0]]),
            Err(LsqError::RhsShapeMismatch {
                rhs_rows: 1,
                matrix_rows: 2
            })
        );
    }

    #[test]
    fn bit_replay_same_inputs_same_bits() {
        let a = vec![
            vec![1.0, 0.3, 0.09],
            vec![1.0, 0.7, 0.49],
            vec![1.0, 1.1, 1.21],
            vec![1.0, 1.9, 3.61],
        ];
        let b = vec![vec![0.1], vec![0.6], vec![1.3], vec![2.9]];
        let x1 = solve_normal(&a, &b).unwrap();
        let x2 = solve_normal(&a, &b).unwrap();
        for (r1, r2) in x1.iter().zip(x2.iter()) {
            for (v1, v2) in r1.iter().zip(r2.iter()) {
                assert_eq!(v1.to_bits(), v2.to_bits());
            }
        }
    }

    /// The nonzero basis values `N_{s−p..=s}(t)` on a raw knot slice
    /// (`knots[s] ≤ t < knots[s+1]`, the last span closed), weighted and
    /// divided by their weighted sum: a rational collocation row. Raw
    /// knots so an unclamped vector can be built too.
    fn collocation_row(knots: &[f64], p: usize, weights: &[f64], t: f64) -> (usize, Vec<f64>) {
        let last = knots.len() - p - 2;
        let s = (p..=last).rev().find(|&s| knots[s] <= t).unwrap_or(p);
        let mut n = vec![1.0f64];
        for d in 1..=p {
            let mut next = vec![0.0f64; d + 1];
            for (r, v) in n.iter().enumerate() {
                let j = s - (d - 1) + r;
                let (lo, hi) = (knots[j], knots[j + d]);
                let a = (t - lo) / (hi - lo);
                next[r] += (1.0 - a) * v;
                next[r + 1] += a * v;
            }
            n = next;
        }
        let first = s - p;
        let den: f64 = n.iter().zip(&weights[first..]).map(|(v, w)| v * w).sum();
        (
            first,
            n.iter()
                .zip(&weights[first..])
                .map(|(v, w)| v * w / den)
                .collect(),
        )
    }

    fn dense_of(first: &[usize], rows: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n = rows.len();
        first
            .iter()
            .zip(rows)
            .map(|(f, r)| {
                let mut d = vec![0.0f64; n];
                d[*f..*f + r.len()].copy_from_slice(r);
                d
            })
            .collect()
    }

    /// Bitwise comparison, NaN matching NaN.
    fn same_bits(a: &[Vec<f64>], b: &[Vec<f64>]) -> usize {
        a.iter()
            .flatten()
            .zip(b.iter().flatten())
            .filter(|(u, v)| !(u.to_bits() == v.to_bits() || (u.is_nan() && v.is_nan())))
            .count()
    }

    /// Collocation at `params` on `knots` (degree `p`, `weights`), with
    /// right-hand sides that exercise every sign of zero: a smooth
    /// column, a column of zeros of mixed sign, and a column that is
    /// zero except where it is not.
    fn identity_case(label: &str, knots: &[f64], p: usize, weights: &[f64], params: &[f64]) {
        let (first, rows): (Vec<usize>, Vec<Vec<f64>>) = params
            .iter()
            .map(|t| collocation_row(knots, p, weights, *t))
            .unzip();
        let n = rows.len();
        let b: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                let t = i as f64;
                let z = if (i * 7) % 5 < 2 { -0.0 } else { 0.0 };
                let sparse = if i % 9 == 4 { -(t.cos()) } else { z };
                vec![(0.7 * t).sin() * 3.0 - 1.0, z, sparse]
            })
            .collect();
        let dense = solve_square(&dense_of(&first, &rows), &b).unwrap();
        let banded = factor_banded(&first, &rows).unwrap().solve(&b).unwrap();
        assert_eq!(
            same_bits(&dense, &banded),
            0,
            "{label} (n = {n}): banded solution differs in bits from the dense one"
        );
    }

    fn uniform(n: usize) -> Vec<f64> {
        (0..n).map(|i| i as f64 / (n - 1) as f64).collect()
    }

    /// Eq. 9.8's averaged clamped knots for `params`.
    fn averaged(params: &[f64], p: usize) -> Vec<f64> {
        let n = params.len();
        let mut k = vec![0.0f64; p + 1];
        for j in 1..(n - p) {
            let s: f64 = params[j..j + p].iter().sum();
            k.push(s / p as f64);
        }
        k.extend(core::iter::repeat_n(1.0, p + 1));
        k
    }

    #[test]
    fn banded_lu_reproduces_the_dense_solve_bit_for_bit() {
        for &n in &[4usize, 23, 120, 301] {
            // Clamped, nonuniform parameters, cubic and quintic.
            let params: Vec<f64> = uniform(n)
                .iter()
                .map(|u| u + 0.04 * (9.0 * u).sin() * u * (1.0 - u))
                .collect();
            for p in [3usize, 5] {
                if n <= p {
                    continue;
                }
                let ones = vec![1.0; n];
                identity_case("clamped", &averaged(&params, p), p, &ones, &params);
                // Rational: positive weights scale rows and columns, which
                // keeps total positivity and the band.
                let w: Vec<f64> = (0..n).map(|i| 1.0 + 0.6 * (i as f64 * 1.3).sin()).collect();
                identity_case("rational", &averaged(&params, p), p, &w, &params);
            }
            // Unclamped: uniform knots extending past both ends, parameters
            // on the knot-free interior so the matrix stays square.
            let p = 3;
            let knots: Vec<f64> = (0..n + p + 1).map(|i| i as f64).collect();
            let interior: Vec<f64> = (0..n)
                .map(|i| p as f64 + (i as f64 + 0.5) * (n - p) as f64 / n as f64)
                .collect();
            identity_case("unclamped", &knots, p, &vec![1.0; n], &interior);
        }
    }

    /// The witness the dropped-term summary exists for: `y_0 = −0.0`
    /// stands outside the last row's window, and the dense forward sum
    /// subtracts `+0.0 · −0.0` from the last row's `−0.0`, turning it
    /// `+0.0`; the window's own terms are all `+0.0` and cannot.
    #[test]
    fn a_dropped_negative_zero_still_flips_the_sum_it_would_have_met() {
        let n = 12;
        let params = uniform(n);
        let (first, rows): (Vec<usize>, Vec<Vec<f64>>) = params
            .iter()
            .map(|t| collocation_row(&averaged(&params, 3), 3, &vec![1.0; n], *t))
            .unzip();
        let mut b = vec![vec![0.0]; n];
        b[0][0] = -0.0;
        b[n - 1][0] = -0.0;
        let dense = solve_square(&dense_of(&first, &rows), &b).unwrap();
        let banded = factor_banded(&first, &rows).unwrap().solve(&b).unwrap();
        assert_eq!(same_bits(&dense, &banded), 0, "signs of zero differ");
        // The flip is real: the last row's own terms leave `−0.0`, the
        // dropped `−0.0` product does not.
        assert!(
            dense[n - 1][0].is_sign_positive() && dense[0][0].is_sign_negative(),
            "the fixture no longer exercises the flip: {:?}",
            dense.iter().map(|r| r[0]).collect::<Vec<_>>()
        );
    }

    #[test]
    fn a_non_finite_right_hand_side_poisons_what_the_dense_solve_poisons() {
        let n = 15;
        let params = uniform(n);
        let (first, rows): (Vec<usize>, Vec<Vec<f64>>) = params
            .iter()
            .map(|t| collocation_row(&averaged(&params, 3), 3, &vec![1.0; n], *t))
            .unzip();
        for poisoned in [0usize, 7, n - 1] {
            let mut b: Vec<Vec<f64>> = (0..n).map(|i| vec![i as f64, -0.0]).collect();
            b[poisoned] = vec![f64::INFINITY, f64::NAN];
            let dense = solve_square(&dense_of(&first, &rows), &b).unwrap();
            let banded = factor_banded(&first, &rows).unwrap().solve(&b).unwrap();
            assert_eq!(same_bits(&dense, &banded), 0, "poison at {poisoned}");
        }
    }

    #[test]
    fn a_band_that_breaks_its_shape_refuses_typed() {
        let rows = vec![vec![1.0, 0.5]; 4];
        // Starts left of the row before: the first-nonzero property.
        assert_eq!(
            factor_banded(&[0, 1, 0, 2], &rows).unwrap_err(),
            LsqError::BandShape { row: 2 }
        );
        assert_eq!(
            factor_banded(&[0, 1, 2, 3], &rows).unwrap_err(),
            LsqError::BandShape { row: 3 },
            "a window past the last column"
        );
        assert_eq!(
            factor_banded(&[0, 1, 2], &rows).unwrap_err(),
            LsqError::BandShape { row: 3 },
            "one window short"
        );
    }

    /// A window that leaves out its diagonal is singular, and refuses
    /// where, and as, the dense elimination does.
    #[test]
    fn a_window_off_its_diagonal_refuses_as_the_dense_solve_does() {
        let b = vec![vec![1.0]; 4];
        for first in [[0usize, 0, 0, 2], [0, 2, 2, 2]] {
            let rows = vec![vec![1.0, 0.5]; 4];
            let dense = solve_square(&dense_of(&first, &rows), &b).unwrap_err();
            let banded = factor_banded(&first, &rows).unwrap_err();
            assert_eq!(banded, dense, "first = {first:?}");
        }
        // A numeric zero pivot inside the band refuses at its own index.
        let rows = vec![vec![1.0, 1.0], vec![1.0, 1.0], vec![1.0, 1.0]];
        let first = [0usize, 0, 1];
        assert_eq!(
            factor_banded(&first, &rows).unwrap_err(),
            solve_square(&dense_of(&first, &rows), &b[..3]).unwrap_err()
        );
    }

    #[test]
    fn one_factorisation_serves_every_right_hand_side() {
        let n = 40;
        let params = uniform(n);
        let (first, rows): (Vec<usize>, Vec<Vec<f64>>) = params
            .iter()
            .map(|t| collocation_row(&averaged(&params, 3), 3, &vec![1.0; n], *t))
            .unzip();
        let lu = factor_banded(&first, &rows).unwrap();
        let b1: Vec<Vec<f64>> = (0..n).map(|i| vec![(i as f64).sqrt()]).collect();
        let b2: Vec<Vec<f64>> = (0..n).map(|i| vec![1.0 / (1.0 + i as f64), -2.0]).collect();
        let both: Vec<Vec<f64>> = b1
            .iter()
            .zip(&b2)
            .map(|(u, v)| [&u[..], &v[..]].concat())
            .collect();
        let x1 = lu.solve(&b1).unwrap();
        let x2 = lu.solve(&b2).unwrap();
        let x = lu.solve(&both).unwrap();
        let split: Vec<Vec<f64>> = x1
            .iter()
            .zip(&x2)
            .map(|(u, v)| [&u[..], &v[..]].concat())
            .collect();
        assert_eq!(
            same_bits(&x, &split),
            0,
            "columns solved apart differ from together"
        );
        assert_eq!(
            lu.solve(&b1[..3]).unwrap_err(),
            LsqError::RhsShapeMismatch {
                rhs_rows: 3,
                matrix_rows: n
            }
        );
    }

    /// **`LsqError`'s recourse claim, made enforceable**: every arm
    /// names the lever the caller turns, exactly once and labelled.
    ///
    /// The solver is reached from the public fit door
    /// (`geom::NurbsCurve3::interpolate` / `::approximate`, through
    /// `FitError::Lsq`), which renders this carrier whole and adds five
    /// characters, so whatever an arm here does not say is simply absent
    /// from what a caller reads. This row is what lets
    /// `every_fit_error_arm_names_a_recourse` read that delegation
    /// TRANSITIVELY rather than as a claim about one chosen payload.
    ///
    /// **A floor, not a proof**: a vocabulary check cannot tell a
    /// recourse from a sentence containing a verb, so a new arm whose
    /// repair uses a word not on this list fails it honestly — extend
    /// the list in the same change.
    #[test]
    fn every_lsq_error_arm_names_a_recourse() {
        const RECOURSE_WORDS: &[&str] = &["supply", "ask", "drop"];
        let arms = [
            LsqError::LsqDegenerate {
                pivot_index: 2,
                pivot: 0.0,
            },
            LsqError::RowLengthMismatch {
                row: 1,
                len: 3,
                expected: 4,
            },
            LsqError::RhsShapeMismatch {
                rhs_rows: 2,
                matrix_rows: 3,
            },
            LsqError::Underdetermined { rows: 2, cols: 3 },
            LsqError::BandShape { row: 4 },
            LsqError::Empty,
        ];
        assert_eq!(arms.len(), 6, "an arm was added without a row here");
        for arm in &arms {
            let msg = arm.to_string();
            assert_eq!(
                test_utils::refusal::recourse_markers(&msg),
                1,
                "not exactly one labelled repair: {msg}"
            );
            let lower = msg.to_lowercase();
            assert!(
                RECOURSE_WORDS.iter().any(|w| lower.contains(w)),
                "no recourse in: {msg}"
            );
        }
    }
}
