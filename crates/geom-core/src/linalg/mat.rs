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
