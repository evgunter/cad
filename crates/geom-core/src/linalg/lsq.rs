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
//!   no external BLAS (nondeterminism), no banded exploitation yet —
//!   correctness first; the systems are small (fitting-sized), and a
//!   banded path can join later without changing the contract.
//! - **Totality.** No panics: internal indexing is justified by the
//!   validated shapes; malformed shapes are typed refusals; non-finite
//!   input reaches the pivot tests as NaN and refuses there
//!   (fail-loud, D4 ¶2).

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
    /// An empty matrix (no rows, or rows of zero length).
    Empty,
}

impl core::fmt::Display for LsqError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LsqError::LsqDegenerate { pivot_index, pivot } => write!(
                f,
                "lsq: degenerate system (pivot {pivot_index} = {pivot} under the fixed elimination order)"
            ),
            LsqError::RowLengthMismatch { row, len, expected } => {
                write!(f, "lsq: row {row} has length {len}, expected {expected}")
            }
            LsqError::RhsShapeMismatch {
                rhs_rows,
                matrix_rows,
            } => write!(f, "lsq: rhs has {rhs_rows} rows, matrix has {matrix_rows}"),
            LsqError::Underdetermined { rows, cols } => {
                write!(f, "lsq: underdetermined ({rows} rows < {cols} cols)")
            }
            LsqError::Empty => f.write_str("lsq: empty system"),
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
}
