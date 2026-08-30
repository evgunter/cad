//! Linear maps of the 3-D tangent space.
//!
//! See the [module docs](super) for the affine/linear split: a matrix here
//! is a linear endomorphism of the tangent space — it acts on [`Vec3`],
//! never directly on points (affine maps, which also move points, are
//! [`super::Affine3`]).
//!
//! Storage is column-major through *named column fields* (`c0`, `c1`,
//! `c2`), each a vector — the columns are the images of the standard basis
//! vectors, which is also why matrix-vector application reads as a linear
//! combination of columns. No array storage and no indexing, per the
//! module-level policy (D9: no panic paths).

use core::ops::Mul;

use crate::linalg::Vec3;
use crate::real::Real;

/// A linear map of the 3-D tangent space, stored as three named columns.
#[derive(Clone, Copy, Debug)]
pub struct Mat3<T: Real> {
    /// The first column — the image of the basis vector (1, 0, 0).
    pub c0: Vec3<T>,
    /// The second column — the image of the basis vector (0, 1, 0).
    pub c1: Vec3<T>,
    /// The third column — the image of the basis vector (0, 0, 1).
    pub c2: Vec3<T>,
}

impl<T: Real> Mat3<T> {
    /// Builds a matrix from its columns (the images of the basis
    /// vectors).
    pub fn from_cols(c0: Vec3<T>, c1: Vec3<T>, c2: Vec3<T>) -> Self {
        Self { c0, c1, c2 }
    }

    /// The identity map.
    pub fn identity() -> Self {
        Self::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z())
    }

    /// The transpose. Pure field shuffling — no arithmetic, so
    /// bit-exact, and an exact involution.
    pub fn transpose(self) -> Self {
        Self::from_cols(
            Vec3::new(self.c0.x, self.c1.x, self.c2.x),
            Vec3::new(self.c0.y, self.c1.y, self.c2.y),
            Vec3::new(self.c0.z, self.c1.z, self.c2.z),
        )
    }

    /// The determinant, evaluated exactly as the scalar triple product
    /// `c0 · (c1 × c2)`: the cross first (its own fixed associations, see
    /// [`Vec3::cross`]), then the fixed-association dot ([`Vec3::dot`]).
    /// D9: this evaluation order is part of the contract.
    pub fn determinant(self) -> T {
        self.c0.dot(self.c1.cross(self.c2))
    }

    /// The rotation by `angle` radians about `axis` (right-hand rule),
    /// via the Rodrigues form `R = cosθ·I + sinθ·[n]× + (1 − cosθ)·n nᵀ`
    /// for the *normalized* axis `n`.
    ///
    /// **The axis is normalized internally** ([`Vec3::normalize`]).
    /// Total: a zero (or poisoned) axis therefore yields an all-NaN
    /// matrix — deliberately, per the crate's totality policy. The
    /// alternative of trusting the caller to pre-normalize was rejected:
    /// a silently unnormalized axis *scales* everything it rotates, a
    /// far worse bug than visible poison.
    ///
    /// Evaluation order (fixed, D9), with `(s, c) = angle.sin_cos()` and
    /// `t = 1 − c`: each off-diagonal entry is `((t·nᵢ)·nⱼ) ± (s·nₖ)` and
    /// each diagonal entry is `(t·(nᵢ²)) + c`, exactly as parenthesized.
    /// The diagonal's square is the tight square (`powi(2)`), taken
    /// before the `t` scale: at `Interval` the plain `(t·nᵢ)·nᵢ` treats
    /// the two `nᵢ` factors as independent, so an axis component whose
    /// enclosure straddles zero gives the diagonal a spurious sign
    /// excursion. Unlike the off-diagonals — genuine mixed products,
    /// which stay as written — the diagonal is a scaled square, and `t`
    /// is arbitrary, so this association is not the `f64` product's:
    /// the two differ by a rounding and that difference is visible in
    /// `f64` output.
    /// Orthogonality and unit determinant hold to rounding, not exactly.
    pub fn rotation_about(axis: Vec3<T>, angle: T) -> Self {
        let n = axis.normalize();
        let (s, c) = angle.sin_cos();
        let t = T::one() - c;
        let (x, y, z) = (n.x, n.y, n.z);
        Self::from_cols(
            Vec3::new(t * x.powi(2) + c, t * x * y + s * z, t * x * z - s * y),
            Vec3::new(t * x * y - s * z, t * y.powi(2) + c, t * y * z + s * x),
            Vec3::new(t * x * z + s * y, t * y * z - s * x, t * z.powi(2) + c),
        )
    }

    /// `I − R` for the rotation `R = rotation_about(axis, angle)` — the
    /// operator that carries an anchor displacement to the translation
    /// of the rotation anchored there (`Affine3::rotation_about_axis`).
    ///
    /// **Assembled so the vanishing factor multiplies**, which is the
    /// whole point of the method existing: `I − R` is `−s·[n]× − t·[n]×²`
    /// with `s = sin θ` and `t = 1 − cos θ`, so every entry already
    /// carries a factor that vanishes with the angle — but only if it is
    /// *built* that way. Spelled `Mat3::identity() − rotation_about(…)`
    /// the diagonal would come out as `1 − (t·nᵢ² + c)`, two near-unit
    /// quantities differenced, and its ulp-of-1 cancellation error would
    /// swamp the entry's own magnitude (`≈ θ²/2`) for every angle below
    /// `θ ≈ 1e-8`. Here instead:
    ///
    /// - **The factors come from the half angle**: `t = 2·sin²(θ/2)` and
    ///   `s = 2·sin(θ/2)·cos(θ/2)`, one `sin_cos` of `θ/2`. Both are
    ///   exact identities for the full-angle forms, and both make
    ///   `sin(θ/2)` a syntactic factor of every entry — so the operator
    ///   vanishes with the angle *by construction* rather than by
    ///   cancellation. The full-angle `1 − cos θ` cannot: its enclosure
    ///   at the exact point `θ = 0` is `[0, 4.44e-16]` (the interval
    ///   `cos` rounds outward from 1), a floor that has nothing to do
    ///   with the angle. This form's is `[0, 2.5e-323]`.
    /// - `t` uses the **tight square** `powi(2)`: `sin(θ/2)`'s enclosure
    ///   straddles zero near `θ = 0`, and `hs·hs` would return a
    ///   straddling product where the square is one-sided.
    /// - The diagonal is `t·(nⱼ² + nₖ²)` — the two *other* squared
    ///   components, which is `t·(1 − nᵢ²)` for a normalized axis, but
    ///   without the cancellation: near a coordinate axis `nᵢ` rounds to
    ///   1 and `1 − nᵢ²` collapses to exactly zero, while the sum of
    ///   squares stays tight. They agree over the reals through
    ///   `|n| = 1`, exactly so on the coordinate axes.
    /// - The off-diagonals are the entries of `R` negated, term for term
    ///   in the same order — the same expressions, evaluated on the
    ///   half-angle `s` and `t`.
    ///
    /// The `f64` zero-angle case is therefore exactly the zero matrix.
    /// The `Interval` one is not *bitwise* zero — `sin`'s enclosure at
    /// the exact point 0 is `[−2e-323, 2e-323]` rather than `[0, 0]`, a
    /// backend property no spelling here can undo — but it is zero to
    /// within subnormal dust, and, decisively, **independent of the
    /// operand it multiplies**.
    ///
    /// Same totality contract as [`Mat3::rotation_about`]: the axis is
    /// normalized internally, so a zero or poisoned axis yields an
    /// all-NaN operator (`0·NaN` is NaN — the poison survives even the
    /// zero-angle case). Evaluation order fixed as written (D9).
    pub fn identity_minus_rotation_about(axis: Vec3<T>, angle: T) -> Self {
        let n = axis.normalize();
        let (hs, hc) = (angle * T::from_f64(0.5)).sin_cos();
        let two = T::from_f64(2.0);
        let s = two * hs * hc;
        let t = two * hs.powi(2);
        let (x, y, z) = (n.x, n.y, n.z);
        Self::from_cols(
            Vec3::new(
                t * (y.powi(2) + z.powi(2)),
                -(t * x * y + s * z),
                -(t * x * z - s * y),
            ),
            Vec3::new(
                -(t * x * y - s * z),
                t * (x.powi(2) + z.powi(2)),
                -(t * y * z + s * x),
            ),
            Vec3::new(
                -(t * x * z + s * y),
                -(t * y * z - s * x),
                t * (x.powi(2) + y.powi(2)),
            ),
        )
    }

    /// The inverse via the adjugate: the rows of `M⁻¹` are
    /// `(c1 × c2)/det`, `(c2 × c0)/det`, `(c0 × c1)/det`, with `det`
    /// computed as `c0 · (c1 × c2)` — bit-identical to
    /// [`Mat3::determinant`], reusing the already-computed cross — one
    /// reciprocal `1/det`, and one multiply per entry (fixed order, D9).
    ///
    /// **Total.** A singular map divides by a zero determinant: entries
    /// become non-finite (±∞, or NaN where a zero adjugate entry meets
    /// the infinite reciprocal) and flow onward as poison per the crate's
    /// totality policy. A *near*-singular map returns finite entries
    /// magnified by 1/det — deciding whether a map is invertible enough
    /// is a predicate-layer question (determinant sign and margin), not
    /// this method's.
    pub fn inverse(self) -> Self {
        // Rows of the adjugate-over-det inverse.
        let r0 = self.c1.cross(self.c2);
        let r1 = self.c2.cross(self.c0);
        let r2 = self.c0.cross(self.c1);
        // Bit-identical to determinant(): same cross, same dot.
        let inv_det = T::one() / self.c0.dot(r0);
        Self::from_cols(
            Vec3::new(r0.x, r1.x, r2.x) * inv_det,
            Vec3::new(r0.y, r1.y, r2.y) * inv_det,
            Vec3::new(r0.z, r1.z, r2.z) * inv_det,
        )
    }
}

/// Matrix-vector application `m * v`: the linear combination of columns
/// `((c0·v.x + c1·v.y) + c2·v.z)`, evaluated exactly in that fixed order
/// componentwise (each output component is `((c0ᵢ·x + c1ᵢ·y) + c2ᵢ·z)`),
/// D9.
impl<T: Real> Mul<Vec3<T>> for Mat3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
        self.c0 * rhs.x + self.c1 * rhs.y + self.c2 * rhs.z
    }
}

/// Matrix composition `a * b` — apply `b` first, then `a` (standard
/// function-composition order: `(a * b) * v = a * (b * v)` in exact
/// arithmetic). Column `j` of the product is `a * (column j of b)`, the
/// fixed matrix-vector order above; in floating point the two sides of
/// the composition law differ by reassociation rounding.
impl<T: Real> Mul for Mat3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::from_cols(self * rhs.c0, self * rhs.c1, self * rhs.c2)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// Coordinate strategy — see `vec.rs::tests::coord` for the range
    /// rationale (session-box-like magnitudes, no −0.0).
    fn coord() -> impl Strategy<Value = f64> {
        (1.0e-3..1.0e3f64, any::<bool>()).prop_map(|(m, neg)| if neg { -m } else { m })
    }

    fn vec3() -> impl Strategy<Value = Vec3<f64>> {
        (coord(), coord(), coord()).prop_map(|(x, y, z)| Vec3::new(x, y, z))
    }

    /// Angles in ±10 rad: several full turns, well inside libm's
    /// high-accuracy argument-reduction range.
    fn angle() -> impl Strategy<Value = f64> {
        -10.0..10.0f64
    }

    fn assert_vec3_bits_eq(a: Vec3<f64>, b: Vec3<f64>) {
        assert_eq!(a.x.to_bits(), b.x.to_bits());
        assert_eq!(a.y.to_bits(), b.y.to_bits());
        assert_eq!(a.z.to_bits(), b.z.to_bits());
    }

    /// Entrywise |a − b| ≤ bound, with the matrices as (col, row) grids.
    fn assert_mat3_entrywise_close(a: Mat3<f64>, b: Mat3<f64>, bound: f64) {
        for (ca, cb) in [(a.c0, b.c0), (a.c1, b.c1), (a.c2, b.c2)] {
            assert!(
                (ca.x - cb.x).abs() <= bound,
                "entry off by {}",
                (ca.x - cb.x).abs()
            );
            assert!(
                (ca.y - cb.y).abs() <= bound,
                "entry off by {}",
                (ca.y - cb.y).abs()
            );
            assert!(
                (ca.z - cb.z).abs() <= bound,
                "entry off by {}",
                (ca.z - cb.z).abs()
            );
        }
    }

    #[test]
    fn identity_applies_as_identity_bit_exact() {
        // I·v componentwise is ((1·x + 0·y) + 0·z) etc.: 1·x is exact,
        // the 0·_ products are signed zeros, and adding a signed zero to
        // a nonzero (or +0) value is bit-exact. Only −0.0 components
        // could differ (−0 + +0 = +0); samples avoid −0.0.
        for v in [
            Vec3::new(1.5, -2.25, 3.0e3),
            Vec3::new(-1.0e-3, 7.0, -0.125),
            Vec3::new(0.0, -3.5, 1.0e6),
        ] {
            assert_vec3_bits_eq(Mat3::identity() * v, v);
        }
    }

    #[test]
    fn determinant_of_identity_is_one() {
        // All products are of exact 0s and 1s: det(I) is exactly 1.
        assert_eq!(Mat3::<f64>::identity().determinant(), 1.0);
    }

    #[test]
    fn inverse_of_zero_matrix_is_all_nan() {
        // det = 0 and every adjugate entry is a signed zero, so each
        // entry is (±0)·(1/0 = ±∞) = NaN: the documented poison outcome
        // for a singular map.
        let z3 = Mat3::from_cols(Vec3::<f64>::zero(), Vec3::zero(), Vec3::zero()).inverse();
        for c in [z3.c0, z3.c1, z3.c2] {
            assert!(c.x.is_nan() && c.y.is_nan() && c.z.is_nan());
        }
    }

    #[test]
    fn rotation_about_zero_axis_is_all_nan() {
        // Documented contract: `rotation_about` normalizes internally, so
        // a zero axis poisons the whole matrix (normalize(0) is all-NaN
        // and every Rodrigues entry consumes an axis component) — visible
        // poison instead of a silently wrong rotation.
        let r = Mat3::rotation_about(Vec3::new(0.0f64, 0.0, 0.0), 1.0);
        for c in [r.c0, r.c1, r.c2] {
            assert!(c.x.is_nan() && c.y.is_nan() && c.z.is_nan());
        }
    }

    /// The diagonal is `(t·nᵢ²) + c`, **bit-exactly and not
    /// `((t·nᵢ)·nᵢ) + c`** — the association the doc comment states, at
    /// an input that can tell the two apart.
    ///
    /// `t = 1 − cos θ` is arbitrary, so the two spellings differ by a
    /// rounding on most oblique axes — unlike `vec.rs`'s `±1`-scaled
    /// twin, which is exact either way.
    ///
    /// **DO NOT DELETE THIS AS REDUNDANT. It is the only thing in the
    /// tree that objects.** Every committed artifact — every golden,
    /// render, STEP export and k-lint row — rotates about an axis whose
    /// components are `0` or `±1`, and there
    /// `(t·0)·0 = t·(0²) = 0` and `(t·1)·1 = t·(1²) = t` make the two
    /// associations identical. So re-associating this diagonal back
    /// would move `f64` output for real callers with an **oblique**
    /// axis while the entire committed corpus stayed byte-identical and
    /// green. That was measured, not assumed: the conversion that
    /// introduced this test changed 34.6% of random (θ, axis) diagonals
    /// and re-cut no golden anywhere.
    ///
    /// The tree's one other oblique-axis bit-exact rotation row
    /// (`editor-core/tests/asm2a_instantiate.rs`) cannot help: its
    /// oracle re-spells the caller's own expression and so moves with
    /// the code. That is smell-scan **S215**, and this doc comment is
    /// the reason it is only a finding rather than a hole.
    ///
    /// **The angles are swept, not hand-picked.** Whether a given θ
    /// separates the two spellings depends on libm's `sin_cos` to the
    /// last ulp, so a hardcoded pair is a fixture that can silently
    /// stop discriminating under a libm bump. The sweep asserts
    /// instead that *some* angle in it discriminates each of the three
    /// diagonals, which fails loudly if that ever stops being true.
    #[test]
    fn rotation_diagonal_takes_the_square_before_the_scale() {
        // The RAW axis goes in: `rotation_about` normalizes internally,
        // so handing it a pre-normalized vector normalizes twice and the
        // expectation below would be modelling a different axis. (This
        // test caught exactly that mistake being made in it.)
        let axis = Vec3::new(1.0f64, 2.0, 3.0);
        let n = axis.normalize();
        let mut discriminating = [false; 3];
        for k in 1..=64u32 {
            let theta = f64::from(k) * 0.05;
            let r = Mat3::rotation_about(axis, theta);
            // `Real::sin_cos`, NOT std's inherent `f64::sin_cos`. The
            // kernel routes transcendentals through the pure-Rust `libm`
            // crate BECAUSE the platform libm differs in the last ulp
            // (D9; `real.rs`'s `libm_vs_std_divergence_census` measures
            // it at ~3% of samples, max 1 ulp). At this precision that
            // difference is the whole test, so std's method is the wrong
            // oracle here — as this test demonstrated by rejecting it.
            let (_, c) = <f64 as Real>::sin_cos(theta);
            let t = 1.0 - c;
            let got = [r.c0.x, r.c1.y, r.c2.z];
            for (i, ni) in [n.x, n.y, n.z].into_iter().enumerate() {
                let want = t * <f64 as Real>::powi(ni, 2) + c;
                assert_eq!(
                    got[i].to_bits(),
                    want.to_bits(),
                    "diagonal {i} at theta={theta}: {} vs documented t*n^2+c {}",
                    got[i],
                    want
                );
                if want.to_bits() != ((t * ni) * ni + c).to_bits() {
                    discriminating[i] = true;
                }
            }
        }
        assert_eq!(
            discriminating, [true; 3],
            "no angle in the sweep separates ((t*n)*n)+c from (t*n^2)+c for \
             every diagonal — this test no longer guards the association"
        );
    }

    /// The **off-diagonals** of [`Mat3::rotation_about`] are
    /// `((t·nᵢ)·nⱼ) ± (s·nₖ)` and **not** `(t·(nᵢ·nⱼ)) ± (s·nₖ)` — the
    /// association the doc comment states, at an input that can tell
    /// the two apart. The sibling above pins the diagonal; with this
    /// row the whole Rodrigues form has a value pin rather than half
    /// of one.
    ///
    /// **DO NOT DELETE THIS AS REDUNDANT**, for the reason the
    /// diagonal row states at length: every committed artifact rotates
    /// about an axis whose components are `0` or `±1`, where
    /// `(t·0)·0`, `t·(0·0)`, `(t·1)·1` and `t·(1·1)` all agree, so
    /// re-associating here would move `f64` output for every
    /// **oblique**-axis caller while the entire corpus stayed
    /// byte-identical and green.
    ///
    /// The tree's one oblique-axis bit-exact rotation row outside this
    /// module (`editor-core/tests/asm2a_instantiate.rs`) cannot cover
    /// it: that oracle re-spells its own caller's expression and so
    /// moves with the code. Smell-scan **S215**, whose remaining half
    /// this row is.
    ///
    /// **What this does NOT pin, said because the oracle shares it
    /// with the subject:** [`Vec3::normalize`]. Both sides normalize
    /// the raw axis, so a change inside normalization moves both — a
    /// `vec.rs` row's job, not this one's. What is pinned here is the
    /// Rodrigues arithmetic downstream of `n`.
    ///
    /// **The angles are swept, not hand-picked**, exactly as above:
    /// the sweep asserts that *some* angle separates the two
    /// spellings for each of the six entries, which fails loudly if a
    /// libm bump ever leaves an entry unable to discriminate.
    #[test]
    fn rotation_off_diagonals_scale_by_t_before_the_second_component() {
        // The RAW axis goes in — `rotation_about` normalizes
        // internally, as the diagonal row's comment explains.
        let axis = Vec3::new(1.0f64, 2.0, 3.0);
        let n = axis.normalize();
        let (nx, ny, nz) = (n.x, n.y, n.z);
        let mut discriminating = [false; 6];
        for k in 1..=64u32 {
            let theta = f64::from(k) * 0.05;
            let r = Mat3::rotation_about(axis, theta);
            // `Real::sin_cos`, NOT std's inherent method: the kernel
            // routes transcendentals through `libm` and the two differ
            // in the last ulp, which at this precision is the whole
            // test (D9; see the diagonal row).
            let (s, c) = <f64 as Real>::sin_cos(theta);
            let t = 1.0 - c;
            // Column-major, in the order `from_cols` writes them:
            // (entry, documented spelling, the association it is not).
            let entries = [
                (r.c0.y, (t * nx) * ny + s * nz, t * (nx * ny) + s * nz),
                (r.c0.z, (t * nx) * nz - s * ny, t * (nx * nz) - s * ny),
                (r.c1.x, (t * nx) * ny - s * nz, t * (nx * ny) - s * nz),
                (r.c1.z, (t * ny) * nz + s * nx, t * (ny * nz) + s * nx),
                (r.c2.x, (t * nx) * nz + s * ny, t * (nx * nz) + s * ny),
                (r.c2.y, (t * ny) * nz - s * nx, t * (ny * nz) - s * nx),
            ];
            for (idx, (got, want, alt)) in entries.into_iter().enumerate() {
                assert_eq!(
                    got.to_bits(),
                    want.to_bits(),
                    "off-diagonal {idx} at theta={theta}: {got} vs \
                     documented ((t*ni)*nj)+-(s*nk) {want}"
                );
                if want.to_bits() != alt.to_bits() {
                    discriminating[idx] = true;
                }
            }
        }
        assert_eq!(
            discriminating, [true; 6],
            "no angle in the sweep separates ((t*ni)*nj) from (t*(ni*nj)) \
             for every off-diagonal — this test no longer guards the \
             association"
        );
    }

    proptest! {
        /// transpose ∘ transpose is the identity *bit-exactly*: transpose
        /// is pure field shuffling, no arithmetic anywhere.
        #[test]
        fn transpose_is_an_exact_involution(a in vec3(), b in vec3(), c in vec3()) {
            let m = Mat3::from_cols(a, b, c);
            let tt = m.transpose().transpose();
            assert_vec3_bits_eq(tt.c0, m.c0);
            assert_vec3_bits_eq(tt.c1, m.c1);
            assert_vec3_bits_eq(tt.c2, m.c2);
        }

        /// R·Rᵀ ≈ I for Rodrigues rotations. Error budget: the normalized
        /// axis components are within ~3 ulps relative (norm_squared: 2
        /// roundings; sqrt: 1; division: 1), sin/cos within ~1 ulp, so
        /// each R entry (≤ 3 products + 1 add on values ≤ 2) is within
        /// ~10·EPSILON ≈ 2.2e-15 absolute; an R·Rᵀ entry (3 products + 2
        /// adds of entries ≤ 1) then sits within ~3·2·(2.2e-15) + 5·EPSILON
        /// ≈ 1.4e-14. Asserted at 2e-13 (order-of-magnitude slack for the
        /// unmodeled constants). det, a triple product of near-unit
        /// columns, gets the same budget and the same asserted bound.
        #[test]
        fn rotation3_orthogonal_and_special(axis in vec3(), theta in angle()) {
            let r = Mat3::rotation_about(axis, theta);
            let p = r * r.transpose();
            assert_mat3_entrywise_close(p, Mat3::identity(), 2e-13);
            prop_assert!((r.determinant() - 1.0).abs() <= 2e-13);
        }

        /// Rodrigues about the z axis reproduces the standard 2-D
        /// rotation block — columns `(cos, sin)` and `(−sin, cos)` —
        /// embedded in the x-y plane. With n = (0, 0, 1) exactly
        /// (normalization of a unit basis vector is exact: 1/1), every
        /// `t·nᵢ·nⱼ` term is a signed zero or `t`, so the 2×2 block
        /// entries reduce to ±s and c exactly, the off-block entries to
        /// signed zeros, and the corner to (1 − c) + c — the one entry
        /// with real rounding, within ~1.5 ulp of 1 (fl(1 − c) errs
        /// ≤ EPSILON for c ≤ ½ and is exact for c ≥ ½ by Sterbenz; the
        /// re-add rounds once at ≤ EPSILON/2). Everything is therefore
        /// within 5e-16 of the embedded 2-D rotation entrywise.
        #[test]
        fn rodrigues_z_axis_matches_embedded_planar_rotation(theta in angle()) {
            let r3 = Mat3::<f64>::rotation_about(Vec3::unit_z(), theta);
            let (s, c) = theta.sin_cos();
            let embedded = Mat3::from_cols(
                Vec3::new(c, s, 0.0),
                Vec3::new(-s, c, 0.0),
                Vec3::unit_z(),
            );
            assert_mat3_entrywise_close(r3, embedded, 5e-16);
        }

        /// The rotation axis is (approximately) fixed by the rotation.
        /// R's entries err ≤ ~10·EPSILON absolute (see the orthogonality
        /// budget) against the exact rotation fixing the *exact*
        /// normalized axis; applying R to the axis (components ≤ m) adds
        /// matrix-vector roundings of order EPSILON·m. Budget ≈
        /// 3·10·EPSILON·m + 4·EPSILON·m ≈ 8e-15·m; asserted at 1e-12·m
        /// (two orders of slack — the normalization error of the axis
        /// enters both sides and mostly cancels, but is not modeled).
        #[test]
        fn rotation_fixes_its_axis(axis in vec3(), theta in angle()) {
            let r = Mat3::rotation_about(axis, theta);
            let ra = r * axis;
            let m = axis.x.abs().max(axis.y.abs()).max(axis.z.abs());
            prop_assert!((ra.x - axis.x).abs() <= 1e-12 * m);
            prop_assert!((ra.y - axis.y).abs() <= 1e-12 * m);
            prop_assert!((ra.z - axis.z).abs() <= 1e-12 * m);
        }

        /// Rotations compose associatively with application only up to
        /// reassociation rounding: (a·b)·v regroups the sum
        /// Σⱼ aᵢⱼ (Σₖ bⱼₖ vₖ) — same terms, different association — so
        /// bounded, never bit-exact, is the honest assertion. For unit
        /// rows/columns and |v| ≤ m the regrouped sums differ by
        /// ≤ ~10·EPSILON·m ≈ 2.2e-15·m; asserted at 1e-12·m.
        #[test]
        fn mat_mul_composes_with_application(
            ax1 in vec3(), th1 in angle(),
            ax2 in vec3(), th2 in angle(),
            v in vec3(),
        ) {
            let a = Mat3::rotation_about(ax1, th1);
            let b = Mat3::rotation_about(ax2, th2);
            let lhs = (a * b) * v;
            let rhs = a * (b * v);
            let m = v.x.abs().max(v.y.abs()).max(v.z.abs());
            prop_assert!((lhs.x - rhs.x).abs() <= 1e-12 * m);
            prop_assert!((lhs.y - rhs.y).abs() <= 1e-12 * m);
            prop_assert!((lhs.z - rhs.z).abs() <= 1e-12 * m);
        }

        /// Well-conditioned inverse: for a rotation, inverse ≈ transpose.
        /// The adjugate of a near-orthonormal matrix is its transpose up
        /// to entry errors of order EPSILON, and det ≈ 1 within ~2e-13
        /// (budget above), so entries agree within ~1e-13; asserted at
        /// 1e-12.
        #[test]
        fn rotation_inverse_is_transpose_within_bound(axis in vec3(), theta in angle()) {
            let r = Mat3::rotation_about(axis, theta);
            assert_mat3_entrywise_close(r.inverse(), r.transpose(), 1e-12);
        }
    }
}
