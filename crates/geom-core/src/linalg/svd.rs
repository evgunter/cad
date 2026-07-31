//! Fixed-shape SVD for the underdetermined systems of the SSI marcher
//! (C12.8's second linalg addition; M5 PR 7 is its first and only
//! consumer, per the no-speculative-abstraction rule).
//!
//! # What this is for
//!
//! Hoffmann §6.2/§6.3.2 traces a surface–surface intersection by solving
//! a sequence of **underdetermined** linear systems of one fixed shape:
//!
//! - **2×3** for an implicit pair in ℝ³ — rows `∇f`, `∇g`, unknown a
//!   derivative `r⁽ᵐ⁾ ∈ ℝ³` (Eq. 6.1, p. 209).
//! - **3×4** for a parametric pair in ℝ⁴ — the Jacobian of
//!   `G₁(u₁,v₁) − G₂(u₂,v₂)`, unknown a derivative in parameter space
//!   (§6.3.2, pp. 222–223).
//!
//! Both are `M × N` with `N = M + 1`. Three things are wanted from each:
//!
//! 1. the **null direction** (the last right singular vector) — the
//!    curve tangent, `∇f × ∇g` in the ℝ³ case;
//! 2. the **smallest singular value** σ_min (Hoffmann's σ₂ at 2×3) — the
//!    transversality signal, which M5 PR 7 bands as a named Q1 trilean
//!    with σ_max as its lever arm and refuses toward C7 in-band;
//! 3. the **minimum-norm solution** of `A x = b` — which is exactly
//!    Hoffmann's Frenet choice of the free coefficient (see below).
//!
//! # Why the minimum-norm solution *is* the Frenet choice
//!
//! For a unit-speed local parameterization `r(s)` the general solution
//! of `A r⁽ᵐ⁾ = b_m` is `x₀ + γ_m t` with `t` the null direction and
//! `x₀ ⊥ t` the minimum-norm solution. Unit speed (`r′·r′ = 1`) forces
//! the free coefficient outright, in any dimension:
//!
//! - differentiating once: `r′·r″ = 0`, so **γ₂ = 0** — the minimum-norm
//!   solution *is* `r″`;
//! - differentiating twice: `r′·r‴ + r″·r″ = 0`, so
//!   **γ₃ = −κ²** with `κ = ‖r″‖` — `r‴ = x₀ − κ² t`.
//!
//! Those are Hoffmann's pp. 212–214 choices (`γ₂ = 0`, `γ₃ = −κ²`),
//! here derived from arc length alone rather than from the ℝ³ Frenet
//! frame, which is why the same stepper serves the ℝ⁴ trace unchanged.
//!
//! # Shape, determinism, and why this is `f64`-only
//!
//! Like [`lsq`](super::lsq) (C6's f64 selection lane) this module is
//! **`f64`-only**: the marcher is a candidate generator whose product is
//! certified afterwards at any `T`, and the algorithm needs value
//! branches (a zero Householder column, a zero Jacobi off-diagonal)
//! that the comparison-free [`Real`](crate::real::Real) surface
//! deliberately cannot express. Nothing here decides topology; the C2
//! certificate does, generically, downstream.
//!
//! Determinism (D9): **fixed reflection and rotation order throughout**
//! — Householder columns ascending, Jacobi index pairs
//! `(0,1), (0,2), …` ascending inside a **fixed** [`JACOBI_SWEEPS`]
//! sweep count (no convergence test, so the operation count depends on
//! the shape alone), reductions summed ascending. No pivoting, no
//! reordering, no magnitude-driven branch: the same input bits give the
//! same output bits. `libm` supplies the only transcendental (`sqrt`,
//! via [`f64::sqrt`], which is IEEE-exact).
//!
//! # Method
//!
//! `A` is short and wide, so the work happens on `Aᵀ` (`N × M`, tall):
//!
//! 1. **Householder QR** of `Aᵀ = Q·[R; 0]`, `Q` orthogonal `N × N`,
//!    `R` upper triangular `M × M`. Hence `A = [L | 0]·Qᵀ` with
//!    `L = Rᵀ` lower triangular.
//! 2. **One-sided Jacobi SVD** of the small triangular `L = U Σ Wᵀ`.
//! 3. Therefore `A = U·[Σ | 0]·diag(Wᵀ, 1)·Qᵀ`, i.e. the right singular
//!    vectors are `V = Q·diag(W, 1)`. The **null direction is the last
//!    column of `Q`** — available before any Jacobi work and untouched
//!    by it.
//! 4. The minimum-norm solve does **not** go through `Σ⁺` (which would
//!    poison at a zero singular value and lose accuracy at a small
//!    one): with `x = Q z`, `A x = b` reads `L z₍₀..M₎ = b` and
//!    minimum norm forces `z_M = 0`, so `x = Q·[L⁻¹b; 0]` by forward
//!    substitution — exact in exact arithmetic, better conditioned in
//!    floating point, and poison (NaN) exactly when `L` is singular.
//!
//! # Singular values are **not** sorted
//!
//! One-sided Jacobi leaves the columns in their own order and this
//! module does not permute them: a permutation would have to compare
//! magnitudes, and the sorted-ness would then be a property callers
//! silently depend on. [`Svd::sigma`] is paired index-wise with
//! [`Svd::u_col`]/[`Svd::v_col`]; the two order-free readings callers
//! actually want, [`Svd::sigma_min`] and [`Svd::sigma_max`], are folded
//! in ascending index order and are the supported interface.
//!
//! # Indexing
//!
//! Fixed-size arrays with loop bounds that are the arrays' own lengths
//! — the [`lsq`](super::lsq) justification, one step stronger because
//! the shapes are compile-time constants. No `Vec`, no allocation, no
//! input-dependent index anywhere.

/// One-sided Jacobi sweeps, **fixed** (D9: the operation count is a
/// function of the shape, never of the values). Twelve sweeps is far
/// past convergence for the 2×2 and 3×3 triangular factors this module
/// decomposes — Jacobi converges quadratically and these shapes are
/// machine-converged in three or four — and the surplus costs a few
/// dozen flops per marcher step.
pub const JACOBI_SWEEPS: usize = 12;

/// The SVD of a fixed-shape `M × N` matrix with `N = M + 1` — the
/// underdetermined shape of Hoffmann's derivative systems.
///
/// Construct with [`Svd::new`]; see the module docs for the method, the
/// determinism contract, and the deliberate absence of singular-value
/// sorting. Use the aliases [`Svd2x3`] and [`Svd3x4`].
#[derive(Clone, Copy, Debug)]
pub struct Svd<const M: usize, const N: usize> {
    /// Householder `Q` of `Aᵀ`, row-major: `q[i][j]` is row `i`,
    /// column `j`. Its last column is the null direction.
    q: [[f64; N]; N],
    /// `L = Rᵀ`, lower triangular, row-major.
    l: [[f64; M]; M],
    /// Singular values of `L` (= of `A`), in Jacobi column order.
    sigma: [f64; M],
    /// Left singular vectors of `L` (= of `A`), row-major columns.
    u: [[f64; M]; M],
    /// Right singular vectors of `L`, row-major columns; the right
    /// singular vectors of `A` are `Q·diag(W, 1)`.
    w: [[f64; M]; M],
}

/// The 2×3 shape: an implicit pair marched in ℝ³ (Hoffmann Eq. 6.1).
pub type Svd2x3 = Svd<2, 3>;

/// The 3×4 shape: a parametric pair traced in ℝ⁴ (Hoffmann §6.3.2).
pub type Svd3x4 = Svd<3, 4>;

impl<const M: usize, const N: usize> Svd<M, N> {
    /// Decompose `a`, given row-major (`a[i]` is row `i`, of length
    /// `N`).
    ///
    /// **Total**: a non-finite entry propagates to NaN singular values,
    /// a NaN null direction, and NaN solutions — never a panic, never a
    /// silent substitution. `N == M + 1` is the intended shape; other
    /// shapes decompose correctly as far as the algebra goes, but only
    /// then is the last column of `Q` a null direction, so
    /// [`Svd::null_direction`] documents that precondition.
    // Column-major traversal of row-major fixed-size arrays: the loop
    // variable indexes the *inner* dimension across rows, which no
    // iterator adaptor expresses (the `lsq` precedent).
    #[allow(clippy::needless_range_loop)]
    pub fn new(a: [[f64; N]; M]) -> Self {
        // ---- Totality guard: poison in, poison OUT ----
        //
        // A partially-poisoned input would otherwise reach the
        // Householder loop, skip the reflections whose norms are NaN,
        // and leave a `Q` that is still the identity in those columns —
        // i.e. hand back a clean finite "null direction" derived from
        // nothing. That is the one failure mode worse than NaN. So a
        // single non-finite entry poisons the whole decomposition, and
        // every reading of it (σ, tangent, solve) is NaN.
        let mut finite = true;
        for row in a.iter() {
            for v in row.iter() {
                finite &= v.is_finite();
            }
        }
        if !finite {
            return Self {
                q: [[f64::NAN; N]; N],
                l: [[f64::NAN; M]; M],
                sigma: [f64::NAN; M],
                u: [[f64::NAN; M]; M],
                w: [[f64::NAN; M]; M],
            };
        }

        // ---- Aᵀ, N × M, row-major ----
        let mut b = [[0.0f64; M]; N];
        for (i, brow) in b.iter_mut().enumerate() {
            for (j, bij) in brow.iter_mut().enumerate() {
                // i < N and j < M: `a` is [[f64; N]; M], so a[j] is a
                // row of length N and a[j][i] is in range.
                *bij = a[j][i];
            }
        }

        // ---- Householder QR of B (= Aᵀ), columns ascending ----
        let mut q = [[0.0f64; N]; N];
        for (i, qrow) in q.iter_mut().enumerate() {
            qrow[i] = 1.0;
        }
        for k in 0..M {
            // The reflected column tail x = B[k..N][k].
            let mut nrm2 = 0.0f64;
            for i in k..N {
                nrm2 += b[i][k] * b[i][k];
            }
            let nrm = nrm2.sqrt();
            // A zero (or poisoned) tail leaves the reflection at the
            // identity: nothing to eliminate, and no 0/0.
            if nrm.is_nan() || nrm <= 0.0 {
                continue;
            }
            // alpha = −sign(B[k][k])·‖x‖ — the cancellation-free sign
            // choice; `copysign` makes ±0 deterministic.
            let alpha = -nrm.copysign(b[k][k]);
            let mut v = [0.0f64; N];
            v[k] = b[k][k] - alpha;
            for i in (k + 1)..N {
                v[i] = b[i][k];
            }
            let mut vtv = 0.0f64;
            for vi in v.iter() {
                vtv += vi * vi;
            }
            if vtv.is_nan() || vtv <= 0.0 {
                continue;
            }
            // B ← (I − 2vvᵀ/vᵀv)·B, columns k..M (columns < k are
            // already zero below row k and unaffected).
            for j in k..M {
                let mut dot = 0.0f64;
                for i in k..N {
                    dot += v[i] * b[i][j];
                }
                let f = 2.0 * dot / vtv;
                for i in k..N {
                    b[i][j] -= f * v[i];
                }
            }
            // Q ← Q·(I − 2vvᵀ/vᵀv) = Q − (Qv)·(2/vᵀv)·vᵀ.
            for qrow in q.iter_mut() {
                let mut dot = 0.0f64;
                for (i, vi) in v.iter().enumerate() {
                    dot += qrow[i] * vi;
                }
                let f = 2.0 * dot / vtv;
                for (i, vi) in v.iter().enumerate() {
                    qrow[i] -= f * vi;
                }
            }
            // Exact zeros below the diagonal (the reflection produces
            // them up to rounding; writing them keeps the triangular
            // solve's structure exact).
            b[k][k] = alpha;
            for brow in b.iter_mut().skip(k + 1) {
                brow[k] = 0.0;
            }
        }

        // ---- L = Rᵀ, lower triangular M × M ----
        let mut l = [[0.0f64; M]; M];
        for (i, lrow) in l.iter_mut().enumerate() {
            for (j, lij) in lrow.iter_mut().enumerate() {
                // R is the top M × M block of B; L = Rᵀ.
                *lij = b[j][i];
            }
        }

        // ---- One-sided Jacobi SVD of L ----
        let (sigma, u, w) = jacobi_svd::<M>(l);

        Self { q, l, sigma, u, w }
    }

    /// The singular values, **unsorted** (Jacobi column order — see the
    /// module docs); paired index-wise with [`Svd::u_col`] and
    /// [`Svd::v_col`].
    pub fn sigma(&self) -> [f64; M] {
        self.sigma
    }

    /// The smallest singular value — Hoffmann's σ₂ at the 2×3 shape:
    /// the **transversality signal**. Folded ascending (D9).
    ///
    /// NaN if any singular value is NaN, which is the honest reading of
    /// a poisoned input: every downstream band test then fails.
    pub fn sigma_min(&self) -> f64 {
        let mut acc = f64::INFINITY;
        for s in self.sigma.iter() {
            // NaN-propagating: `f64::min` deliberately *swallows* NaN
            // (returns the other operand), which is exactly wrong for a
            // certificate-facing reading — a poisoned singular value
            // must fail every band, not hide behind its neighbour.
            acc = if s.is_nan() || acc.is_nan() {
                f64::NAN
            } else {
                acc.min(*s)
            };
        }
        acc
    }

    /// The largest singular value — the natural **lever arm** for a
    /// dimensionless band on [`Svd::sigma_min`]. Folded ascending (D9).
    pub fn sigma_max(&self) -> f64 {
        let mut acc = f64::NEG_INFINITY;
        for s in self.sigma.iter() {
            acc = if s.is_nan() || acc.is_nan() {
                f64::NAN
            } else {
                acc.max(*s)
            };
        }
        acc
    }

    /// The null direction: the last right singular vector, unit-length
    /// by construction (a column of an orthogonal `Q`).
    ///
    /// **Meaningful only at the underdetermined shape `N = M + 1`**, the
    /// shape this module exists for: there the null space of a
    /// full-rank `A` is exactly one dimension and this is it. The sign
    /// is whatever the fixed Householder sign convention produces —
    /// deterministic, but callers that need a *consistent* orientation
    /// along a march must orient it themselves against the previous
    /// step (M5 PR 7's stepper does).
    pub fn null_direction(&self) -> [f64; N] {
        let mut out = [0.0f64; N];
        for (i, o) in out.iter_mut().enumerate() {
            // Column N−1 of Q; N ≥ 1 for every instantiation.
            *o = self.q[i][N - 1];
        }
        out
    }

    /// Column `j` of `U` (the left singular vectors of `A`), paired with
    /// `sigma()[j]`. Out-of-range `j` yields the all-NaN vector rather
    /// than a panic.
    pub fn u_col(&self, j: usize) -> [f64; M] {
        let mut out = [f64::NAN; M];
        if j < M {
            for (i, o) in out.iter_mut().enumerate() {
                *o = self.u[i][j];
            }
        }
        out
    }

    /// Column `j` of `V` (the right singular vectors of `A`), paired
    /// with `sigma()[j]` for `j < M`; `j == N − 1` is the null
    /// direction. `V = Q·diag(W, 1)`. Out-of-range `j` yields the
    /// all-NaN vector rather than a panic.
    pub fn v_col(&self, j: usize) -> [f64; N] {
        if j == N - 1 {
            return self.null_direction();
        }
        let mut out = [f64::NAN; N];
        if j < M {
            for (i, o) in out.iter_mut().enumerate() {
                let mut acc = 0.0f64;
                for (k, _) in self.w.iter().enumerate() {
                    acc += self.q[i][k] * self.w[k][j];
                }
                *o = acc;
            }
        }
        out
    }

    /// The **minimum-norm** solution of `A x = b` — the solution
    /// orthogonal to [`Svd::null_direction`], i.e. Hoffmann's γ = 0
    /// choice (module docs).
    ///
    /// Computed by forward substitution on the triangular factor, not
    /// through `Σ⁺`: exact in exact arithmetic, better conditioned in
    /// floating point, and **poison (NaN) exactly when the system is
    /// rank-deficient** — a marcher step that silently produced a
    /// garbage derivative would be far worse than one that produces
    /// NaN and fails the band that follows.
    // Forward substitution needs both `i` (row of `l`) and `j < i`
    // (already-solved entries of `y`) as indices; the `lsq` precedent.
    #[allow(clippy::needless_range_loop)]
    pub fn solve_min_norm(&self, b: &[f64; M]) -> [f64; N] {
        // L y = b, L lower triangular, ascending order.
        let mut y = [0.0f64; M];
        for i in 0..M {
            let mut acc = b[i];
            for j in 0..i {
                acc -= self.l[i][j] * y[j];
            }
            y[i] = acc / self.l[i][i];
        }
        // x = Q·[y; 0].
        let mut x = [0.0f64; N];
        for (i, xi) in x.iter_mut().enumerate() {
            let mut acc = 0.0f64;
            for (j, yj) in y.iter().enumerate() {
                acc += self.q[i][j] * yj;
            }
            *xi = acc;
        }
        x
    }

    /// `U·Σ·Vᵀ`, row-major — the reconstruction identity, for tests and
    /// for anyone auditing the decomposition.
    pub fn reconstruct(&self) -> [[f64; N]; M] {
        let mut out = [[0.0f64; N]; M];
        for (i, orow) in out.iter_mut().enumerate() {
            for (j, oij) in orow.iter_mut().enumerate() {
                let mut acc = 0.0f64;
                for k in 0..M {
                    let vkj = self.v_col(k);
                    acc += self.u[i][k] * self.sigma[k] * vkj[j];
                }
                *oij = acc;
            }
        }
        out
    }
}

/// One-sided Jacobi SVD of a square `M × M` matrix, row-major in and
/// out: returns `(σ, U, W)` with `m = U Σ Wᵀ`.
///
/// Fixed sweep count and fixed ascending index-pair order (D9); the
/// rotation is the classical symmetric-Schur form on the Gram entries
/// of the two columns, which needs no eigen-solve of its own.
#[allow(clippy::needless_range_loop)]
fn jacobi_svd<const M: usize>(m: [[f64; M]; M]) -> ([f64; M], [[f64; M]; M], [[f64; M]; M]) {
    let mut c = m;
    let mut w = [[0.0f64; M]; M];
    for (i, wrow) in w.iter_mut().enumerate() {
        wrow[i] = 1.0;
    }
    for _ in 0..JACOBI_SWEEPS {
        for p in 0..M {
            for q in (p + 1)..M {
                // Gram entries of columns p and q, ascending sums.
                let mut app = 0.0f64;
                let mut aqq = 0.0f64;
                let mut apq = 0.0f64;
                for crow in c.iter() {
                    app += crow[p] * crow[p];
                    aqq += crow[q] * crow[q];
                    apq += crow[p] * crow[q];
                }
                // Already orthogonal (or poisoned): no rotation.
                if apq.is_nan() || apq == 0.0 {
                    continue;
                }
                let zeta = (aqq - app) / (2.0 * apq);
                let t = zeta.signum() / (zeta.abs() + (1.0 + zeta * zeta).sqrt());
                let cs = 1.0 / (1.0 + t * t).sqrt();
                let sn = cs * t;
                for crow in c.iter_mut() {
                    let cp = crow[p];
                    let cq = crow[q];
                    crow[p] = cs * cp - sn * cq;
                    crow[q] = sn * cp + cs * cq;
                }
                for wrow in w.iter_mut() {
                    let wp = wrow[p];
                    let wq = wrow[q];
                    wrow[p] = cs * wp - sn * wq;
                    wrow[q] = sn * wp + cs * wq;
                }
            }
        }
    }
    // σ_j = ‖C[:,j]‖, U[:,j] = C[:,j]/σ_j.
    let mut sigma = [0.0f64; M];
    let mut u = [[0.0f64; M]; M];
    for j in 0..M {
        let mut nrm2 = 0.0f64;
        for crow in c.iter() {
            nrm2 += crow[j] * crow[j];
        }
        let nrm = nrm2.sqrt();
        sigma[j] = nrm;
        if nrm > 0.0 {
            for (i, urow) in u.iter_mut().enumerate() {
                urow[j] = c[i][j] / nrm;
            }
        } else {
            // A zero singular value leaves its left singular vector
            // undetermined; the coordinate axis is the deterministic
            // choice (documented — it affects reconstruction not at
            // all, since it is multiplied by σ = 0).
            u[j][j] = 1.0;
        }
    }
    (sigma, u, w)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mul2x3(a: &[[f64; 3]; 2], x: &[f64; 3]) -> [f64; 2] {
        let mut out = [0.0f64; 2];
        for (i, o) in out.iter_mut().enumerate() {
            let mut acc = 0.0;
            for (j, xj) in x.iter().enumerate() {
                acc += a[i][j] * xj;
            }
            *o = acc;
        }
        out
    }

    fn mul3x4(a: &[[f64; 4]; 3], x: &[f64; 4]) -> [f64; 3] {
        let mut out = [0.0f64; 3];
        for (i, o) in out.iter_mut().enumerate() {
            let mut acc = 0.0;
            for (j, xj) in x.iter().enumerate() {
                acc += a[i][j] * xj;
            }
            *o = acc;
        }
        out
    }

    #[test]
    fn null_direction_of_2x3_is_the_cross_product() {
        let f = [1.0, 2.0, 3.0];
        let g = [-2.0, 0.5, 1.0];
        let s = Svd2x3::new([f, g]);
        let n = s.null_direction();
        // ∇f × ∇g, up to sign and normalization.
        let cross = [
            f[1] * g[2] - f[2] * g[1],
            f[2] * g[0] - f[0] * g[2],
            f[0] * g[1] - f[1] * g[0],
        ];
        let cn = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
        let dot = n[0] * cross[0] / cn + n[1] * cross[1] / cn + n[2] * cross[2] / cn;
        assert!((dot.abs() - 1.0).abs() < 1e-13, "dot = {dot}");
        let nn = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        assert!((nn - 1.0).abs() < 1e-15);
    }

    #[test]
    fn min_norm_solution_solves_and_is_orthogonal_to_the_null_direction() {
        let a = [[1.0, 2.0, 3.0], [-2.0, 0.5, 1.0]];
        let b = [0.7, -1.3];
        let s = Svd2x3::new(a);
        let x = s.solve_min_norm(&b);
        let r = mul2x3(&a, &x);
        assert!((r[0] - b[0]).abs() < 1e-13, "{r:?}");
        assert!((r[1] - b[1]).abs() < 1e-13, "{r:?}");
        let n = s.null_direction();
        let d = x[0] * n[0] + x[1] * n[1] + x[2] * n[2];
        assert!(d.abs() < 1e-13, "min-norm not ⊥ null: {d}");
    }

    #[test]
    fn singular_values_and_reconstruction_2x3() {
        let a = [[3.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
        let s = Svd2x3::new(a);
        assert!((s.sigma_max() - 3.0).abs() < 1e-13, "{:?}", s.sigma());
        assert!((s.sigma_min() - 2.0).abs() < 1e-13, "{:?}", s.sigma());
        let r = s.reconstruct();
        for (i, row) in r.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                assert!((v - a[i][j]).abs() < 1e-12, "{r:?}");
            }
        }
    }

    #[test]
    fn reconstruction_3x4_on_a_general_matrix() {
        let a = [
            [1.0, 2.0, -0.5, 0.25],
            [0.0, 1.5, 2.0, -1.0],
            [3.0, -0.25, 0.75, 2.5],
        ];
        let s = Svd3x4::new(a);
        let r = s.reconstruct();
        for (i, row) in r.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                assert!((v - a[i][j]).abs() < 1e-11, "{r:?} vs {a:?}");
            }
        }
    }

    #[test]
    fn null_direction_3x4_is_annihilated_and_unit() {
        let a = [
            [1.0, 2.0, -0.5, 0.25],
            [0.0, 1.5, 2.0, -1.0],
            [3.0, -0.25, 0.75, 2.5],
        ];
        let s = Svd3x4::new(a);
        let n = s.null_direction();
        let r = mul3x4(&a, &n);
        for v in r.iter() {
            assert!(v.abs() < 1e-12, "A·n = {r:?}");
        }
        let nn = (n.iter().map(|x| x * x).sum::<f64>()).sqrt();
        assert!((nn - 1.0).abs() < 1e-15);
    }

    #[test]
    fn min_norm_solution_3x4() {
        let a = [
            [1.0, 2.0, -0.5, 0.25],
            [0.0, 1.5, 2.0, -1.0],
            [3.0, -0.25, 0.75, 2.5],
        ];
        let b = [1.0, -2.0, 0.5];
        let s = Svd3x4::new(a);
        let x = s.solve_min_norm(&b);
        let r = mul3x4(&a, &x);
        for (i, v) in r.iter().enumerate() {
            assert!((v - b[i]).abs() < 1e-12, "{r:?}");
        }
        let n = s.null_direction();
        let d: f64 = x.iter().zip(n.iter()).map(|(a, b)| a * b).sum();
        assert!(d.abs() < 1e-12, "min-norm not ⊥ null: {d}");
    }

    #[test]
    fn sigma_min_dies_on_a_tangential_pair() {
        // Two nearly-parallel gradients: σ_min → 0, σ_max → O(1). This
        // is the transversality signal M5 PR 7 bands.
        let s = Svd2x3::new([[1.0, 0.0, 0.0], [1.0, 1e-9, 0.0]]);
        assert!(s.sigma_min() < 1e-8, "{:?}", s.sigma());
        assert!(s.sigma_max() > 1.0);
    }

    #[test]
    fn poison_in_poison_out_never_panics() {
        let s = Svd2x3::new([[f64::NAN, 0.0, 0.0], [0.0, 1.0, 0.0]]);
        assert!(s.sigma_min().is_nan());
        assert!(s.sigma_max().is_nan());
        assert!(s.null_direction().iter().all(|v| v.is_nan()));
        let x = s.solve_min_norm(&[1.0, 1.0]);
        assert!(x.iter().all(|v| v.is_nan()));
        // Infinity is poison too — an unbounded gradient is not a
        // direction, and the tangent read off it would be a fiction.
        let inf = Svd3x4::new([
            [f64::INFINITY, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ]);
        assert!(inf.sigma_min().is_nan());
        assert!(inf.null_direction().iter().all(|v| v.is_nan()));
    }

    #[test]
    fn rank_deficient_system_poisons_the_solve_rather_than_guessing() {
        // Parallel gradients: the 2×3 has rank 1, so no minimum-norm
        // solution exists for a generic b — the triangular solve
        // divides by an exact zero and says so.
        let s = Svd2x3::new([[1.0, 0.0, 0.0], [2.0, 0.0, 0.0]]);
        assert!(s.sigma_min() < 1e-15, "{:?}", s.sigma());
        let x = s.solve_min_norm(&[1.0, 1.0]);
        assert!(
            x.iter().any(|v| !v.is_finite()),
            "rank-deficient solve must poison, got {x:?}"
        );
    }

    #[test]
    fn decomposition_is_bit_reproducible() {
        let a = [[1.0, 2.0, 3.0], [-2.0, 0.5, 1.0]];
        let s1 = Svd2x3::new(a);
        let s2 = Svd2x3::new(a);
        assert_eq!(s1.sigma().map(f64::to_bits), s2.sigma().map(f64::to_bits));
        assert_eq!(
            s1.null_direction().map(f64::to_bits),
            s2.null_direction().map(f64::to_bits)
        );
    }
}
