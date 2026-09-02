//! Vectors of the 2-D/3-D linear (tangent) spaces.
//!
//! See the [module docs](super) for the affine/linear split: these are the
//! tangent-side types — displacements, directions, normals — the things
//! that add, negate, and scale. Locations are [`super::Point2`] /
//! [`super::Point3`].

use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::real::Real;

/// A vector of the 2-D linear (tangent) space.
#[derive(Clone, Copy, Debug)]
pub struct Vec2<T: Real> {
    /// The x component.
    pub x: T,
    /// The y component.
    pub y: T,
}

/// A vector of the 3-D linear (tangent) space.
#[derive(Clone, Copy, Debug)]
pub struct Vec3<T: Real> {
    /// The x component.
    pub x: T,
    /// The y component.
    pub y: T,
    /// The z component.
    pub z: T,
}

impl<T: Real> Vec2<T> {
    /// Builds a vector from its components.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// The same vector read at another scalar: `f` applied to each
    /// component, in `x, y` order. A structural map — no arithmetic,
    /// so it is exact whenever `f` is (`Real::from_f64`,
    /// `Dual::constant`).
    #[must_use]
    pub fn map<U: Real>(self, f: impl Fn(T) -> U) -> Vec2<U> {
        Vec2::new(f(self.x), f(self.y))
    }

    /// The zero vector (the additive identity).
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    /// The first standard basis vector, (1, 0).
    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero())
    }

    /// The second standard basis vector, (0, 1).
    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one())
    }

    /// The dot product, evaluated exactly as `(x·x′) + (y·y′)` — one
    /// product per component, one addition; the association is fixed (D9).
    /// Swapping the arguments commutes each product and leaves the
    /// summation order unchanged, so `a.dot(b)` and `b.dot(a)` are
    /// bit-identical at every scalar whose `Mul` and `Add` are bitwise
    /// commutative — see [`Vec3::dot`] for which scalars that is, and
    /// which one it is not pinned for.
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    /// The perp-dot product `x·y′ − y·x′` (evaluated exactly in that fixed
    /// order) — the scalar 2-D analogue of the cross product: the signed
    /// area of the parallelogram spanned by `self` and `rhs`, positive when
    /// `rhs` lies counterclockwise of `self`. Equivalently: the dot of
    /// `self`'s +90° rotation (−y, x) with `rhs`.
    pub fn perp_dot(self, rhs: Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }

    /// The squared Euclidean norm: `x² + y²` via the tight square
    /// [`Real::powi`]`(2)` per component, in [`Vec2::dot`]'s fixed
    /// association order.
    ///
    /// Why `powi(2)` and not `self.dot(self)` (M2 PR 4): at `f64` and
    /// the dual value channel the two are bit-identical (`powi(2)` is
    /// one multiplication), but at the interval scalar plain
    /// multiplication of a zero-straddling enclosure by *itself* yields
    /// a spurious negative lower bound (the product does not know its
    /// factors are one variable), and the downstream `sqrt` then
    /// degrades the decoration to the poison channel — so the
    /// `norm`/`distance` of any rounding-width difference would
    /// escalate. The tight square keeps every term's true range
    /// (non-negative), exactly the ratified `pown` rationale.
    pub fn norm_squared(self) -> T {
        self.x.powi(2) + self.y.powi(2)
    }

    /// The Euclidean norm, exactly `self.norm_squared().sqrt()` — no fused
    /// hypot (see `real.rs` on why fused conveniences are excluded).
    pub fn norm(self) -> T {
        self.norm_squared().sqrt()
    }

    /// The unit vector in this direction, exactly `self / self.norm()`
    /// (one division per component).
    ///
    /// **Total.** The zero vector yields all-NaN components (0/0), and a
    /// poisoned input propagates poison — per the crate's totality policy
    /// (`real.rs`): poison flows through values, and the predicate layer /
    /// residual certification is where it is caught. Components beyond
    /// ~1e154 overflow `norm_squared` to ∞ and collapse the result toward
    /// zero; symmetrically, components below ~1e-162 underflow
    /// `norm_squared` to 0 and blow the result up to ±∞ (not NaN). Both
    /// ends are far outside the session box (D4 ¶4), same posture as
    /// `powi`'s extreme-exponent note.
    pub fn normalize(self) -> Self {
        self / self.norm()
    }

    /// The componentwise minimum (bounding-box support). Inherits
    /// [`Real::min`]'s lattice-not-control-flow contract: ties keep
    /// `self`'s component, and **NaN propagates** per component — a
    /// poisoned coordinate poisons the bound rather than being silently
    /// dropped.
    pub fn min(self, rhs: Self) -> Self {
        Self::new(self.x.min(rhs.x), self.y.min(rhs.y))
    }

    /// The componentwise maximum. Same contract as [`Vec2::min`]: ties
    /// keep `self`'s component, NaN propagates per component.
    pub fn max(self, rhs: Self) -> Self {
        Self::new(self.x.max(rhs.x), self.y.max(rhs.y))
    }
}

impl<T: Real> Vec3<T> {
    /// Builds a vector from its components.
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// The same vector read at another scalar: `f` applied to each
    /// component, in `x, y, z` order (see [`Vec2::map`]).
    #[must_use]
    pub fn map<U: Real>(self, f: impl Fn(T) -> U) -> Vec3<U> {
        Vec3::new(f(self.x), f(self.y), f(self.z))
    }

    /// The zero vector (the additive identity).
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    /// The first standard basis vector, (1, 0, 0).
    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero(), T::zero())
    }

    /// The second standard basis vector, (0, 1, 0).
    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one(), T::zero())
    }

    /// The third standard basis vector, (0, 0, 1).
    pub fn unit_z() -> Self {
        Self::new(T::zero(), T::zero(), T::one())
    }

    /// The dot product, evaluated exactly as `((x·x′) + (y·y′)) + (z·z′)`
    /// — the association is fixed (D9). Swapping the arguments commutes
    /// each componentwise product and permutes nothing in that sum, so
    /// `a.dot(b)` and `b.dot(a)` are bit-identical **at every scalar
    /// whose `Mul` and `Add` are themselves bitwise commutative**.
    ///
    /// Which scalars those are, stated rather than assumed: `f64` (IEEE
    /// `*` and `+` commute at every finite input, ±0 included; NaN
    /// payload propagation is unspecified) and the `f64`-carrier
    /// scalars over it, such as `k_stats::Probe`. The
    /// `dot_symmetry_bit_exact` proptest below pins it there, sampling
    /// `1.0e-3..1.0e3` — so 0, −0, inf and subnormals are covered by
    /// the argument, not by the proptest. It is **not** pinned for
    /// `Interval`, whose `Mul`/`Add` delegate to the enclosure backend:
    /// expected to hold, asserted nowhere in-tree. A caller relying on bit equality at an
    /// enclosure scalar owes that assertion.
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// The cross product, each component evaluated exactly as the fixed
    /// two-product difference
    /// `(y·z′ − z·y′, z·x′ − x·z′, x·y′ − y·x′)`.
    ///
    /// Antisymmetry holds up to the sign of zero: IEEE negation of a
    /// rounded difference is exact (`fl(u − w) = −fl(w − u)` as values),
    /// so `a.cross(b)` and `−b.cross(a)` are value-equal componentwise,
    /// differing at most in zero signs where products cancel exactly.
    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    /// The squared Euclidean norm: `x² + y² + z²` via the tight square
    /// [`Real::powi`]`(2)` per component, in [`Vec3::dot`]'s fixed
    /// association order. See [`Vec2::norm_squared`] for why the tight
    /// square replaces `self.dot(self)` (bit-identical at `f64` and the
    /// dual value channel; interval-lane decoration honesty — M2 PR 4).
    pub fn norm_squared(self) -> T {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// The Euclidean norm, exactly `self.norm_squared().sqrt()` — no fused
    /// hypot (see `real.rs` on why fused conveniences are excluded).
    pub fn norm(self) -> T {
        self.norm_squared().sqrt()
    }

    /// The unit vector in this direction, exactly `self / self.norm()`
    /// (one division per component).
    ///
    /// **Total.** The zero vector yields all-NaN components (0/0), and a
    /// poisoned input propagates poison — per the crate's totality policy
    /// (`real.rs`). Components beyond ~1e154 overflow `norm_squared` to ∞
    /// and collapse the result toward zero; symmetrically, components
    /// below ~1e-162 underflow `norm_squared` to 0 and blow the result up
    /// to ±∞ (not NaN). Both ends are far outside the session box
    /// (D4 ¶4), same posture as `powi`'s extreme-exponent note.
    pub fn normalize(self) -> Self {
        self / self.norm()
    }

    /// The componentwise minimum (bounding-box support). Inherits
    /// [`Real::min`]'s lattice-not-control-flow contract: ties keep
    /// `self`'s component, and **NaN propagates** per component — a
    /// poisoned coordinate poisons the bound rather than being silently
    /// dropped.
    pub fn min(self, rhs: Self) -> Self {
        Self::new(self.x.min(rhs.x), self.y.min(rhs.y), self.z.min(rhs.z))
    }

    /// The componentwise maximum. Same contract as [`Vec3::min`]: ties
    /// keep `self`'s component, NaN propagates per component.
    pub fn max(self, rhs: Self) -> Self {
        Self::new(self.x.max(rhs.x), self.y.max(rhs.y), self.z.max(rhs.z))
    }

    /// The orthogonal projection of `self` onto the line spanned by
    /// `onto`, evaluated exactly as
    /// `onto * (self.dot(onto) / onto.norm_squared())` — the dot and the
    /// squared norm in their fixed associations, **one** scalar division,
    /// then the componentwise scale, in that order (D9).
    ///
    /// The association is part of the contract (the M2 watchlist's D9
    /// hazard): call sites must use this method — never re-derive
    /// `(v·n/n·n)·n` with their own grouping, which rounds differently.
    /// `onto` need not be unit (the quotient normalizes); for a *known*
    /// unit `onto` the division still happens, deliberately — one code
    /// path, one rounding story.
    ///
    /// **Total.** A zero (or poisoned) `onto` yields all-poison
    /// components through the 0/0 division, per the crate's totality
    /// policy. Components of `onto` beyond ~1e154 overflow
    /// `norm_squared` to ∞, collapsing the coefficient — and hence the
    /// projection — to a silent zero (∞ is not f64 poison); the
    /// symmetric underflow end blows it up instead. Both bands are far
    /// outside the session box (D4 ¶4) — the same posture and
    /// boundaries as [`Vec3::normalize`]'s doc note.
    pub fn project_onto(self, onto: Self) -> Self {
        onto * (self.dot(onto) / onto.norm_squared())
    }

    /// The orthogonal rejection of `self` from the line spanned by
    /// `onto`: exactly `self - self.project_onto(onto)` (the
    /// componentwise subtraction of [`Vec3::project_onto`]'s result —
    /// the one sanctioned association, same D9 contract). The result is
    /// orthogonal to `onto` up to rounding; `project + reject = self` up
    /// to one rounding per component.
    ///
    /// **Total.** Poison propagates from [`Vec3::project_onto`].
    pub fn reject_from(self, onto: Self) -> Self {
        self - self.project_onto(onto)
    }

    /// An orthonormal basis completing `self` (a **unit** vector) to a
    /// right-handed frame: returns `(b1, b2)` with `(b1, b2, self)`
    /// orthonormal and right-handed (`b1 × b2 = self` up to rounding).
    ///
    /// This is the **branchless Pixar construction** (Duff, Burgess,
    /// Christensen, Hery, Kensler, Liani, Villemin, *Building an
    /// Orthonormal Basis, Revisited*, JCGT 6(1), 2017), the ratified
    /// resolution of the M0 watchlist's "orthonormal-basis is a
    /// value-branch" concern: there is **no value branch to guard** —
    /// the construction is a fixed straight-line arithmetic sequence
    /// whose only sign decision is [`Real::copysign`], a total value
    /// operation. No predicate is needed because no branch exists;
    /// evaluation is deterministic and bit-identical across
    /// instantiations in the value channel by the same argument as any
    /// other fixed formula.
    ///
    /// **Derivation.** For `s = ±1` matching the sign of `n.z`, the
    /// reflection `R` through the plane bisecting `s·e_z` and `n` maps
    /// `s·e_z ↦ n`; its other two (sign-adjusted) columns are then unit,
    /// mutually orthogonal, and orthogonal to `n` by orthogonality of
    /// the reflection. Writing `a = −1/(s + n.z)` collapses the
    /// reflection's columns to the closed forms below (the paper's §3
    /// algebra); the sign flip keeps the frame right-handed on both
    /// hemispheres AND keeps `s + n.z` away from zero — the naive
    /// single-branch formula divides by `1 + n.z`, which cancels
    /// catastrophically near `n = −e_z` (the classic failure direction).
    ///
    /// **Evaluation order (fixed, D9), and why it is spelled this way.**
    /// The denominator's MAGNITUDE is computed first and its sign is
    /// applied separately, because `s` and `n.z` are correlated —
    /// `s + n.z` is `±(1 + |n.z|)`, never near zero — and an enclosure
    /// scalar that evaluates each occurrence of `s` independently cannot
    /// see that. Writing the sum literally hands `Interval` a
    /// zero-containing denominator for every `n.z = [0, 0]` (issue
    /// #1157: every axis-aligned VERTICAL plane), which divides to
    /// `[−∞, +∞]` decorated `Trv` — a manufactured non-real from inputs
    /// that pose a perfectly real question, which `docs/DUAL-DESIGN.md`
    /// DL6 forbids in a certified lane. So:
    ///
    /// ```text
    /// s  = 1.copysign(n.z)
    /// r  = 1/(1 + s·n.z)          // = 1/|s + n.z| = 1/(1 + |n.z|) ∈ (0, 1]
    /// br = (n.x·n.y)·r
    /// b1 = (1 − (n.x²)·r, −br, −(s·n.x))
    /// b2 = (−(s·br), s − s·((n.y²)·r), −n.y)
    /// ```
    ///
    /// each component exactly as parenthesized.
    ///
    /// **`f64` is bit-identical to the `a = −1/(s + n.z)` spelling this
    /// replaced, and that is measured, not derived** (`vec.rs`'s
    /// `orthonormal_basis_matches_the_duff_spelling_bitwise` sweeps the
    /// unit sphere plus the axis/equator edge set, signed zeros
    /// included). The derivation the measurement confirms: `1 + s·n.z`
    /// is the exact magnitude `|s + n.z|` (negation is exact and
    /// addition is sign-symmetric), `1/s = s` exactly for `s = ±1`, so
    /// `a = −s·r`; each component above then differs from its old
    /// spelling only by multiplications by `±1`, which are exact and
    /// sign-symmetric including on zeros.
    ///
    /// **What the new spelling buys at `Interval`.** `r`'s denominator
    /// is `1 + |n.z|`, whose enclosure is `≥ 1` for EVERY `n.z`
    /// enclosure — including the one-sided and straddling ones, which
    /// is the whole point: `|·|` is a total, monotone map that needs no
    /// sign decision, whereas `s · n.z` needs `copysign` to have
    /// DECIDED a sign, and `Interval::copysign` is strict on both sides
    /// (`interval.rs`), so it must return `[−1, 1]` for any `n.z`
    /// enclosure touching zero. Writing the denominator as `1 + s·n.z`
    /// therefore reintroduced the same defect one enclosure out from
    /// the one #1157 filed: at `n.z = [0, 1]` — an ordinary one-sided
    /// enclosure of a NON-NEGATIVE `z` — it gives `[0, 2]` and `r`
    /// unbounded and `Trv`, while `1 + |n.z|` gives `[1, 2]` and
    /// `r = [0.5, 1]`. The two spellings are bit-identical at `f64`
    /// (`s · n.z ≡ |n.z|` there, signed zeros included), which is
    /// exactly why the `f64` bitwise row cannot see the difference and
    /// `orthonormal_basis_is_bounded_over_z_enclosures` exists.
    ///
    /// `b1` loses
    /// `s` entirely except in `b1.z`: at `n.z = [0, 0]` the enclosure is
    /// then the exact hull of the two frames the equator's sign flip
    /// admits — `(1 − n.x²·r, −n.x·n.y·r, ±n.x)` — rather than a wide
    /// or non-real one, and for a vertical plane with `n.x = 0` (the
    /// `newell_plane` case) it is the EXACT frame. `b2` keeps both of
    /// its `s` occurrences: they are what makes its `f64` bits identical
    /// (a widened `b2.y` would flip a signed zero at `n = (0, ±1, −0.0)`),
    /// and both production consumers — `newell.rs` and
    /// `step-import`'s `recognize.rs` — take `b1` and discard `b2`.
    ///
    /// Both squares are the tight square (`powi(2)`), not the product
    /// `n·n`: at `Interval` the product treats the two factors as
    /// independent, so an enclosure straddling zero — every direction
    /// near an equator — acquires a spurious negative lower bound. Nor
    /// is `powi(2)` unconditionally narrower: on this backend it is 1
    /// ulp wider on each side once the square falls below `2^-960`, i.e.
    /// `|n.x| < 2^-480`, which no unit direction reaches
    /// (`scripts/gates/interval-square-allowlist.sh` carries the
    /// measurement).
    ///
    /// **Discontinuity, documented honestly:** the frame flips across
    /// the equator `n.z = 0` (`s` jumps) — the construction is
    /// deterministic and well-conditioned everywhere on the sphere, but
    /// not continuous as a function of `n` there (no continuous global
    /// frame on the sphere exists — hairy-ball; the seam had to go
    /// somewhere, and `copysign`'s kink conventions carry it honestly
    /// through duals and intervals). At `Interval` an `n.z` enclosure
    /// containing zero cannot tell the two sides apart — a point
    /// enclosure `[0, 0]` carries no sign bit — so the honest answer
    /// there is the hull of both frames, which is what `b1.z` and `b2`
    /// widen to. Consumers wanting a *stable* conventional frame across
    /// parameter changes store the frame (`u_ref`) as data, per D2 —
    /// this constructor is for *making* that data.
    ///
    /// **Precondition (conventional, unchecked):** `self` is unit. A
    /// non-unit input yields a well-defined but non-orthonormal pair
    /// (no poison, no check — same posture as unit-`dir` curve data;
    /// tier-3 certification owns the invariant). A poisoned input
    /// propagates poison.
    pub fn orthonormal_basis(self) -> (Self, Self) {
        let s = T::one().copysign(self.z);
        let r = T::one() / (T::one() + self.z.abs());
        let br = (self.x * self.y) * r;
        let b1 = Self::new(T::one() - self.x.powi(2) * r, -br, -(s * self.x));
        let b2 = Self::new(-(s * br), s - s * (self.y.powi(2) * r), -self.y);
        (b1, b2)
    }
}

impl<T: Real> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Real> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Real> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

/// Right scalar multiplication `v * s` (left multiplication `s * v` is
/// deliberately absent — see the [module docs](super)).
impl<T: Real> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

/// Componentwise scalar division `v / s` — one division per component,
/// not a reciprocal-then-multiply (one rounding per component, and the
/// natural reading at every planned scalar type). Total: `s` zero or
/// poisoned yields poison components.
impl<T: Real> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: Real> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Real> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Real> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

/// Right scalar multiplication `v * s` (left multiplication `s * v` is
/// deliberately absent — see the [module docs](super)).
impl<T: Real> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

/// Componentwise scalar division `v / s` — one division per component,
/// not a reciprocal-then-multiply (one rounding per component, and the
/// natural reading at every planned scalar type). Total: `s` zero or
/// poisoned yields poison components.
impl<T: Real> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// A coordinate with magnitude in [1e-3, 1e3] and independent sign.
    ///
    /// The range is session-box-like (D4 ¶4): it caps every product and
    /// sum within ~9 orders of magnitude of 1, so error bounds of the
    /// form `C · f64::EPSILON · magnitude` stay tight and no overflow or
    /// subnormal behavior muddies the analysis. The sign is a separate
    /// flag so `-0.0` can never be generated — bit-exactness assertions
    /// below rely on that.
    fn coord() -> impl Strategy<Value = f64> {
        (1.0e-3..1.0e3f64, any::<bool>()).prop_map(|(m, neg)| if neg { -m } else { m })
    }

    fn vec2() -> impl Strategy<Value = Vec2<f64>> {
        (coord(), coord()).prop_map(|(x, y)| Vec2::new(x, y))
    }

    fn vec3() -> impl Strategy<Value = Vec3<f64>> {
        (coord(), coord(), coord()).prop_map(|(x, y, z)| Vec3::new(x, y, z))
    }

    fn max_abs3(v: Vec3<f64>) -> f64 {
        v.x.abs().max(v.y.abs()).max(v.z.abs())
    }

    #[test]
    fn basis_vectors_are_orthonormal_exactly() {
        // Products of exact 0s and 1s and their two-term sums are exact.
        let (ex, ey, ez) = (
            Vec3::<f64>::unit_x(),
            Vec3::<f64>::unit_y(),
            Vec3::<f64>::unit_z(),
        );
        assert_eq!(ex.dot(ex), 1.0);
        assert_eq!(ey.dot(ey), 1.0);
        assert_eq!(ez.dot(ez), 1.0);
        assert_eq!(ex.dot(ey), 0.0);
        assert_eq!(ey.dot(ez), 0.0);
        // The right-handed frame: x × y = z exactly (products of 0/1).
        let c = ex.cross(ey);
        assert_eq!((c.x, c.y, c.z), (0.0, 0.0, 1.0));
        let p = Vec2::<f64>::unit_x().perp_dot(Vec2::unit_y());
        assert_eq!(p, 1.0);
    }

    #[test]
    fn zero_is_additive_identity_bit_exact() {
        // v + 0: each component is x + (+0.0), which is bit-exact for
        // every x except x == -0.0; the sample avoids -0.0.
        let v = Vec3::new(1.5, -2.25, 3.0e3);
        let w = v + Vec3::zero();
        assert_eq!(w.x.to_bits(), v.x.to_bits());
        assert_eq!(w.y.to_bits(), v.y.to_bits());
        assert_eq!(w.z.to_bits(), v.z.to_bits());
    }

    #[test]
    fn normalize_zero_vector_is_all_nan() {
        // 0 / sqrt(0) = 0/0 = NaN in every component: the documented
        // totality outcome — poison, not a panic and not a silent zero.
        let n = Vec3::<f64>::zero().normalize();
        assert!(n.x.is_nan() && n.y.is_nan() && n.z.is_nan());
        let n2 = Vec2::<f64>::zero().normalize();
        assert!(n2.x.is_nan() && n2.y.is_nan());
    }

    #[test]
    fn min_max_propagate_nan() {
        // One poisoned coordinate poisons exactly that component of the
        // bound (Real::min/max NaN propagation, not IEEE minNum).
        let v = Vec3::new(f64::NAN, 1.0, 2.0);
        let w = Vec3::new(0.0, f64::NAN, 5.0);
        let lo = v.min(w);
        let hi = v.max(w);
        assert!(lo.x.is_nan() && lo.y.is_nan());
        assert_eq!(lo.z, 2.0);
        assert!(hi.x.is_nan() && hi.y.is_nan());
        assert_eq!(hi.z, 5.0);
    }

    proptest! {
        /// Dot symmetry is bit-exact: each componentwise product commutes
        /// exactly (IEEE multiplication is commutative), and swapping the
        /// arguments does not permute the fixed summation order
        /// ((x·x′ + y·y′) + z·z′ keeps its shape under the swap), so both
        /// sides round identically at every step.
        #[test]
        fn dot_symmetry_bit_exact(a in vec3(), b in vec3()) {
            prop_assert_eq!(a.dot(b).to_bits(), b.dot(a).to_bits());
        }

        /// Same argument in 2-D.
        #[test]
        fn dot2_symmetry_bit_exact(a in vec2(), b in vec2()) {
            prop_assert_eq!(a.dot(b).to_bits(), b.dot(a).to_bits());
        }

        /// Cross antisymmetry as *values* (==), not bits: each component
        /// of `b × a` is fl(u) − fl(w) where `a × b`'s is fl(w) − fl(u),
        /// and IEEE subtraction satisfies fl(u − w) = −fl(w − u) exactly —
        /// except that an exactly cancelling difference gives +0 on both
        /// sides, whose negation is −0. Hence value equality, not
        /// bit equality, is the honest assertion.
        #[test]
        fn cross_antisymmetry(a in vec3(), b in vec3()) {
            let ab = a.cross(b);
            let ba = -b.cross(a);
            prop_assert_eq!(ab.x, ba.x);
            prop_assert_eq!(ab.y, ba.y);
            prop_assert_eq!(ab.z, ba.z);
        }

        /// perp_dot is antisymmetric as values, same argument as
        /// `cross_antisymmetry` (one fixed two-product difference).
        #[test]
        fn perp_dot_antisymmetry(a in vec2(), b in vec2()) {
            prop_assert_eq!(a.perp_dot(b), -b.perp_dot(a));
        }

        /// (a × b) · a = 0 exactly in real arithmetic. Error budget with
        /// m = max |component|: each cross component is ≤ 2m² with
        /// absolute error ≤ ~2·(u·m²) from the two products plus
        /// ≤ u·2m² from the subtraction (u = half an ulp = EPSILON/2),
        /// so ≤ 2·EPSILON·m². The dot contributes |a|·(those errors)
        /// ≤ 3·(2·EPSILON·m²)·m plus its own product/sum roundings on
        /// terms ≤ 2m³, another ≤ ~3·EPSILON·m³ — call it ≤ 12·EPSILON·m³
        /// total. Asserted at 64·EPSILON·m³ for constant-factor slack.
        #[test]
        fn cross_is_orthogonal_to_operands(a in vec3(), b in vec3()) {
            let m = max_abs3(a).max(max_abs3(b));
            let bound = 64.0 * f64::EPSILON * m.powi(3);
            prop_assert!(a.cross(b).dot(a).abs() <= bound);
            prop_assert!(a.cross(b).dot(b).abs() <= bound);
        }

        /// norm_squared is a sum of squares: every term is ≥ 0 (or NaN,
        /// excluded by generation), and adding nonnegatives rounds to a
        /// nonnegative.
        #[test]
        fn norm_squared_nonnegative(a in vec3()) {
            prop_assert!(a.norm_squared() >= 0.0);
        }

        /// |normalize(v)| = 1 to a few ulps: norm_squared carries ≤ ~2
        /// roundings, sqrt ≤ half an ulp, each component division ≤ half
        /// an ulp, and the outer norm the same again — a relative error
        /// budget of ≈ 5·EPSILON ≈ 1.1e-15. Asserted at 1e-14.
        #[test]
        fn normalize_produces_unit_norm(a in vec3()) {
            prop_assert!((a.normalize().norm() - 1.0).abs() <= 1e-14);
        }

        /// Scaling and adding are componentwise and exact per IEEE op:
        /// (v * s) / s recovers v within 1 ulp per component (two
        /// correctly rounded operations, no cancellation). Bounded, not
        /// bit-exact: v·s rounds.
        #[test]
        fn mul_div_roundtrip(v in vec3(), s in 1.0e-3..1.0e3f64) {
            let r = (v * s) / s;
            prop_assert!((r.x - v.x).abs() <= 2.0 * f64::EPSILON * v.x.abs());
            prop_assert!((r.y - v.y).abs() <= 2.0 * f64::EPSILON * v.y.abs());
            prop_assert!((r.z - v.z).abs() <= 2.0 * f64::EPSILON * v.z.abs());
        }

        /// project + reject decomposes v: the parts re-sum to v (up to
        /// one rounding per component — reject IS v − project, so the
        /// re-sum is a subtract-then-add round trip), the rejection is
        /// orthogonal to the axis, and the projection is parallel to it.
        /// Error budget for reject ⊥ onto with m = max component
        /// magnitude: the projection coefficient v·n/|n|² is
        /// O(1)-conditioned only when |n| is not tiny relative to v, so
        /// both magnitudes share one generator scale; the residual
        /// (v − proj)·n cancels values of order m²·(m²/m²) — a few
        /// roundings of m² each, asserted at 1e3·EPSILON·m² for slack
        /// across the 6-decade generator range.
        #[test]
        fn project_reject_decompose(v in vec3(), n in vec3()) {
            let p = v.project_onto(n);
            let r = v.reject_from(n);
            let m = max_abs3(v).max(max_abs3(n));
            let tol = 1e3 * f64::EPSILON * m.powi(2);
            // Orthogonality of the rejection (the load-bearing claim).
            prop_assert!(r.dot(n).abs() <= tol * (1.0 + max_abs3(v) / max_abs3(n)));
            // Parallelism of the projection: p × n ≈ 0.
            let c = p.cross(n);
            prop_assert!(max_abs3(c) <= tol * (1.0 + max_abs3(v) / max_abs3(n)));
            // Recomposition: p + r = v up to one rounding per component.
            let sum = p + r;
            prop_assert!((sum.x - v.x).abs() <= 4.0 * f64::EPSILON * m);
            prop_assert!((sum.y - v.y).abs() <= 4.0 * f64::EPSILON * m);
            prop_assert!((sum.z - v.z).abs() <= 4.0 * f64::EPSILON * m);
        }

        /// Projecting onto the projection axis is idempotent (within
        /// rounding), and projecting a vector already parallel to the
        /// axis reproduces it.
        #[test]
        fn project_idempotent(v in vec3(), n in vec3()) {
            let p = v.project_onto(n);
            let pp = p.project_onto(n);
            let m = max_abs3(v);
            prop_assert!((pp.x - p.x).abs() <= 1e-12 * m);
            prop_assert!((pp.y - p.y).abs() <= 1e-12 * m);
            prop_assert!((pp.z - p.z).abs() <= 1e-12 * m);
        }

        /// The Pixar basis over random unit vectors: orthonormality
        /// residuals within a few ulps and right-handedness
        /// (b1 × b2 = n up to rounding). Error budget: every
        /// intermediate is O(1) (unit input, |a| ≤ 1), each component
        /// carries ≤ 4 roundings, dots of near-unit vectors ≤ ~6
        /// roundings — everything sits within ~10·EPSILON ≈ 2.2e-15;
        /// asserted at 1e-14 (input normalization error adds ~5·EPSILON).
        #[test]
        fn orthonormal_basis_properties(v in vec3()) {
            let n = v.normalize();
            let (b1, b2) = n.orthonormal_basis();
            prop_assert!((b1.norm() - 1.0).abs() <= 1e-14);
            prop_assert!((b2.norm() - 1.0).abs() <= 1e-14);
            prop_assert!(b1.dot(b2).abs() <= 1e-14);
            prop_assert!(b1.dot(n).abs() <= 1e-14);
            prop_assert!(b2.dot(n).abs() <= 1e-14);
            // Right-handedness: b1 × b2 reproduces n componentwise.
            let c = b1.cross(b2);
            prop_assert!((c.x - n.x).abs() <= 1e-14);
            prop_assert!((c.y - n.y).abs() <= 1e-14);
            prop_assert!((c.z - n.z).abs() <= 1e-14);
        }

        /// The value channel of the basis construction at `Dual<f64>` is
        /// bit-identical to the plain-f64 run — the cross-instantiation
        /// contract, exercised through a real linalg consumer (the
        /// construction is a fixed formula over `Real` ops, so this
        /// holds by composition; the test guards the claim against
        /// future edits introducing a scalar-specific path).
        ///
        /// **The VALUE channel only, and the name says so on purpose.**
        /// `f64` has no tangent to be identical to, so this test cannot
        /// reach the derivative channel at all — that is
        /// [`orthonormal_basis_dual_tangent_matches_closed_form`]'s job,
        /// and the two together are what covers the construction. A
        /// fixture built with `Dual::variable` also cannot distinguish
        /// spellings of a square: its tangent is `1.0`, and `y + y` and
        /// `2·y` are equal exactly there.
        #[test]
        fn orthonormal_basis_dual_value_channel_bit_identical(v in vec3()) {
            use crate::dual::Dual;
            let n = v.normalize();
            let (b1, b2) = n.orthonormal_basis();
            let nd = Vec3::new(
                Dual::variable(n.x),
                Dual::variable(n.y),
                Dual::variable(n.z),
            );
            let (d1, d2) = nd.orthonormal_basis();
            for (ours, dual) in [
                (b1.x, d1.x), (b1.y, d1.y), (b1.z, d1.z),
                (b2.x, d2.x), (b2.y, d2.y), (b2.z, d2.z),
            ] {
                prop_assert_eq!(ours.to_bits(), dual.value.to_bits());
            }
        }

        /// The TANGENT channel of BOTH squared components — `b1.x` and
        /// `b2.y` — against their closed-form derivatives, the channel
        /// the test above cannot reach.
        ///
        /// The construction is spelled with the denominator's magnitude
        /// separated from its sign (#1157; constructor docs), and the
        /// closed form checked here is the ALGEBRAIC one it is equal to:
        /// with `a = −1/(s + n.z)` and `s` locally constant
        /// (`copysign`'s kink convention; the seam at `n.z = 0` is
        /// documented at the constructor), `b1.x = 1 − n.x²/(1 + |n.z|)
        /// = 1 + (s·n.x²)·a` and `b2.y = s − s·n.y²/(1 + |n.z|) =
        /// s + n.y²·a`, so
        ///
        /// ```text
        /// d(b1.x) = 2·s·n.x·a·tx + s·n.x²·tz/(s + n.z)²
        /// d(b2.y) = 2·n.y·a·ty   +   n.y²·tz/(s + n.z)²
        /// ```
        ///
        /// Well conditioned everywhere on the sphere: `|s + n.z|` is
        /// `1 + |n.z| ≥ 1` by construction, which is the whole reason
        /// the two-hemisphere form exists.
        ///
        /// **`b1.x` is covered here because it is the component both
        /// production callers consume** (`newell.rs` and `recognize.rs`
        /// discard `b2`), and because a closed form is the only way to
        /// check a derivative at all: `f64` has no tangent to compare
        /// against, so the value-channel test above cannot reach this.
        ///
        /// **It is also what checks the respelling in the TANGENT
        /// channel**, which the value-channel bit row cannot see: the
        /// two spellings are equal in ℝ, and a respelling that got the
        /// sign of `dr` wrong would leave every `f64` bit identical and
        /// every derivative wrong.
        ///
        /// **What this test is NOT: a guard on the square's spelling.**
        /// With `s` exactly `±1`, `Dual::mul`'s `x'·x + x·x'` and
        /// `Dual::powi`'s `(2·x)·x'` both collapse to `±2·fl(x·x')` —
        /// 0 bit differences over 300,000 samples in the live regime —
        /// so writing `b1.x`'s square either way leaves this green. It
        /// is a **correctness guard against a wrong closed form**: it
        /// reds on a wrong power, a dropped `s`, or a swapped factor.
        /// Worth having, and worth not mistaking for the other thing
        /// given where it sits.
        ///
        /// **The tangents are NOT 1.** `Dual::variable` gives every
        /// input a tangent of `1.0`, and at `ty = 1` the product rule
        /// and the power rule agree bit-for-bit (`y + y` is `2·y`) — so
        /// a fixture built that way exercises the one input at which
        /// every spelling of a square is identical. Independent random
        /// tangents are what make this a test of the rule rather than of
        /// that coincidence.
        #[test]
        fn orthonormal_basis_dual_tangent_matches_closed_form(
            v in vec3(),
            tx in -4.0f64..4.0,
            ty in -4.0f64..4.0,
            tz in -4.0f64..4.0,
        ) {
            use crate::dual::Dual;
            let n = v.normalize();
            prop_assume!(n.x.is_finite() && n.y.is_finite() && n.z.is_finite());
            let nd = Vec3::new(
                Dual::new(n.x, tx),
                Dual::new(n.y, ty),
                Dual::new(n.z, tz),
            );
            let (d1, d2) = nd.orthonormal_basis();
            let s = 1.0f64.copysign(n.z);
            let a = -1.0 / (s + n.z);
            let dsq = tz / ((s + n.z) * (s + n.z));
            let want1 = 2.0 * s * n.x * a * tx + s * n.x * n.x * dsq;
            let want2 = 2.0 * n.y * a * ty + n.y * n.y * dsq;
            for (got, want, which) in
                [(d1.x.deriv, want1, "b1.x"), (d2.y.deriv, want2, "b2.y")]
            {
                let scale = want.abs().max(1.0);
                prop_assert!(
                    (got - want).abs() <= 1e-12 * scale,
                    "{} tangent {} vs closed form {} \
                     (n = {:?}, tx = {}, ty = {}, tz = {})",
                    which, got, want, (n.x, n.y, n.z), tx, ty, tz
                );
            }
        }

        /// The **±1 scale may cross a square** — a ring fact, kept as
        /// one: `s·x` is a sign-bit flip, exact, and
        /// round-to-nearest-even is symmetric under negation, so the
        /// same rounding survives in both associations.
        ///
        /// **It no longer has a production consumer in this file.** It
        /// was `b1.x = 1 + (s·n.x²)·a`'s guard until #1157 respelled the
        /// construction to `1 − n.x²·r`, which carries no scale across
        /// its square (constructor docs). The row stays because the
        /// property is exactly what makes that respelling bit-identical
        /// — the identity is `s·(x²)·(−s·r) = −(x²·r)`, one ±1 crossing
        /// a square in each direction — and
        /// `orthonormal_basis_matches_the_duff_spelling_bitwise` is the
        /// row that would red if it ever stopped holding.
        ///
        /// **`powi(2) == x * x` at `f64` is NOT re-derived here** — it
        /// is pinned over the full edge set (subnormals, `±0`, `±∞`,
        /// `MAX`, NaN) by
        /// `sweep/tests/review_m2_pr4.rs::survives_powi2_bitwise_equals_mul_at_f64`.
        /// This test is only the scale half, and it is a property of the
        /// `±1` scale alone: `mat.rs::rotation_about`'s diagonal carries
        /// an arbitrary `t = 1 − cos θ`, does NOT have it, and is
        /// guarded separately by
        /// `mat.rs::tests::rotation_diagonal_takes_the_square_before_the_scale`.
        #[test]
        fn unit_scale_square_reassociates_exactly(x in -1e6f64..1e6) {
            // The generator reaches none of the interesting magnitudes —
            // it will essentially never draw a subnormal or a signed
            // zero and cannot draw a non-finite — so the edge set is
            // enumerated rather than sampled.
            let edges = [
                0.0f64,
                -0.0,
                f64::MIN_POSITIVE,
                f64::MIN_POSITIVE / 4.0,
                5.0e-324,
                1.0e160,
                f64::MAX,
                f64::INFINITY,
                f64::NEG_INFINITY,
            ];
            for v in core::iter::once(x).chain(edges) {
                for s in [1.0f64, -1.0] {
                    prop_assert_eq!(
                        ((s * v) * v).to_bits(),
                        (s * <f64 as Real>::powi(v, 2)).to_bits(),
                        "s = {}, x = {:e}",
                        s,
                        v
                    );
                }
            }
            for s in [1.0f64, -1.0] {
                prop_assert!(((s * f64::NAN) * f64::NAN).is_nan());
                prop_assert!((s * <f64 as Real>::powi(f64::NAN, 2)).is_nan());
            }
        }

        /// **The `f64` path did not move: measured, not derived.**
        /// #1157's fix respells the construction so an enclosure scalar
        /// can see the `s`/`n.z` correlation (constructor docs). The
        /// claim that costs nothing at `f64` is a claim about BITS, and
        /// this is where it is paid: every component of both frames,
        /// against the literal Duff spelling this replaced, to the bit.
        ///
        /// **The generator cannot reach the inputs that matter**, so the
        /// edge set is enumerated beside it: `coord()` never draws a
        /// zero or a signed zero, and `n.z = ±0.0` — the vertical-plane
        /// case #1157 is about, and the one place the two spellings
        /// could disagree on a signed zero — is exactly what the sweep
        /// would miss. Poison is out of scope here on purpose: NaN bits
        /// are not a contract (`project_reject_basis_poison` owns that).
        #[test]
        fn orthonormal_basis_matches_the_duff_spelling_bitwise(v in vec3()) {
            /// The spelling in Duff et al. §3, verbatim, as the kernel
            /// carried it before #1157 — `a = −1/(s + n.z)` with the
            /// sum written literally.
            fn duff(n: Vec3<f64>) -> (Vec3<f64>, Vec3<f64>) {
                let s = <f64 as Real>::copysign(1.0, n.z);
                let a = -1.0 / (s + n.z);
                let b = (n.x * n.y) * a;
                (
                    Vec3::new(
                        1.0 + (s * <f64 as Real>::powi(n.x, 2)) * a,
                        s * b,
                        -(s * n.x),
                    ),
                    Vec3::new(b, s + <f64 as Real>::powi(n.y, 2) * a, -n.y),
                )
            }
            let mut cases = vec![v, v.normalize(), Vec3::new(v.x, v.y, 0.0)];
            // The equator and the axes, with both signed zeros in `z`
            // (the sign bit `copysign` reads and an enclosure cannot),
            // the poles, and the near-pole direction the naive formula
            // fails on.
            for z in [0.0f64, -0.0] {
                for (x, y) in [
                    (1.0f64, 0.0f64), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0),
                    (0.6, 0.8), (-0.6, -0.8), (0.0, 0.0),
                ] {
                    cases.push(Vec3::new(x, y, z));
                }
            }
            cases.extend([
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(1e-9, -1e-9, -1.0).normalize(),
                Vec3::new(0.6, 0.8, 1e-12).normalize(),
                Vec3::new(0.6, 0.8, -1e-12).normalize(),
                Vec3::new(f64::MIN_POSITIVE, 1.0, -0.0),
            ]);
            for n in cases {
                let (g1, g2) = n.orthonormal_basis();
                let (w1, w2) = duff(n);
                for (got, want, which) in [
                    (g1.x, w1.x, "b1.x"), (g1.y, w1.y, "b1.y"), (g1.z, w1.z, "b1.z"),
                    (g2.x, w2.x, "b2.x"), (g2.y, w2.y, "b2.y"), (g2.z, w2.z, "b2.z"),
                ] {
                    prop_assert_eq!(
                        got.to_bits(), want.to_bits(),
                        "{} at n = ({:e}, {:e}, {:e}): {:e} vs {:e}",
                        which, n.x, n.y, n.z, got, want
                    );
                }
            }
        }
    }

    /// The classic failure directions ±z (where the naive `1/(1 + n.z)`
    /// construction cancels catastrophically), the equator seam, and
    /// near-pole continuity.
    #[test]
    fn orthonormal_basis_poles_and_equator() {
        // Exactly +z and −z: exact frames (all arithmetic on 0s and 1s).
        let (b1, b2) = Vec3::<f64>::unit_z().orthonormal_basis();
        assert_eq!((b1.x, b1.y, b1.z), (1.0, 0.0, -0.0));
        assert_eq!((b2.x, b2.y, b2.z), (0.0, 1.0, -0.0));
        let (b1, b2) = (-Vec3::<f64>::unit_z()).orthonormal_basis();
        assert_eq!((b1.x, b1.y, b1.z), (1.0, -0.0, -0.0));
        assert_eq!((b2.x, b2.y, b2.z), (-0.0, -1.0, -0.0));
        // Near −z (the killer for the naive formula): still orthonormal
        // to a few ulps.
        let n = Vec3::new(1e-9, -1e-9, -1.0).normalize();
        let (b1, b2) = n.orthonormal_basis();
        assert!((b1.norm() - 1.0).abs() <= 1e-14);
        assert!((b2.norm() - 1.0).abs() <= 1e-14);
        assert!(b1.dot(b2).abs() <= 1e-14);
        assert!(b1.dot(n).abs() <= 1e-14);
        assert!(b2.dot(n).abs() <= 1e-14);
        // Continuity on each side of the equator seam: two nearby
        // normals on the SAME side give nearby frames…
        let above = Vec3::new(0.6, 0.8, 1e-12).normalize();
        let above2 = Vec3::new(0.6, 0.8, 2e-12).normalize();
        let (a1, _) = above.orthonormal_basis();
        let (a1b, _) = above2.orthonormal_basis();
        assert!((a1.x - a1b.x).abs() <= 1e-9);
        assert!((a1.y - a1b.y).abs() <= 1e-9);
        assert!((a1.z - a1b.z).abs() <= 1e-9);
        // …while crossing the seam flips the frame (the documented
        // discontinuity: s jumps from +1 to −1): above the equator
        // b1.z = −(s·n.x) ≈ −0.6, below it ≈ +0.6.
        assert!((a1.z - -0.6).abs() <= 1e-9, "a1.z = {}", a1.z);
        let below = Vec3::new(0.6, 0.8, -1e-12).normalize();
        let (c1, c2) = below.orthonormal_basis();
        assert!((c1.z - 0.6).abs() <= 1e-9, "c1.z = {}", c1.z);
        // Both sides are still perfectly valid right-handed frames.
        let cross = c1.cross(c2);
        assert!((cross.x - below.x).abs() <= 1e-14);
        assert!((cross.y - below.y).abs() <= 1e-14);
        assert!((cross.z - below.z).abs() <= 1e-14);
    }

    /// Poison propagation and the zero-`onto` totality outcome for
    /// project/reject, and poison through the basis construction.
    #[test]
    fn project_reject_basis_poison() {
        let v = Vec3::new(1.0f64, 2.0, 3.0);
        let z = Vec3::<f64>::zero();
        let p = v.project_onto(z);
        assert!(p.x.is_nan() && p.y.is_nan() && p.z.is_nan());
        let r = v.reject_from(z);
        assert!(r.x.is_nan() && r.y.is_nan() && r.z.is_nan());
        let poisoned = Vec3::new(f64::NAN, 0.0, 1.0).orthonormal_basis();
        assert!(poisoned.0.x.is_nan());
        assert!(poisoned.1.x.is_nan());
    }

    /// The basis construction at the interval scalar: instantiates, and
    /// the orthonormality residuals (dot products, norm² − 1) enclose 0
    /// for a point-enclosure unit input — the containment form of the
    /// f64 properties above.
    #[cfg(feature = "interval")]
    #[test]
    fn orthonormal_basis_interval_residuals() {
        use crate::interval::Interval;
        use crate::real::Bounds;

        let contains_zero =
            |e: Interval| -> bool { e.lo() <= 0.0 && 0.0 <= e.hi() && !e.lo().is_nan() };
        // An exactly-unit direction: (1, −2, 2)/3 — the exact integer
        // triple, so |n|² − 1 itself encloses 0 tightly.
        let n = Vec3::new(
            Interval::from_f64(1.0) / Interval::from_f64(3.0),
            Interval::from_f64(-2.0) / Interval::from_f64(3.0),
            Interval::from_f64(2.0) / Interval::from_f64(3.0),
        );
        let (b1, b2) = n.orthonormal_basis();
        assert!(contains_zero(b1.dot(b2)));
        assert!(contains_zero(b1.dot(n)));
        assert!(contains_zero(b2.dot(n)));
        assert!(contains_zero(b1.norm_squared() - Interval::one()));
        assert!(contains_zero(b2.norm_squared() - Interval::one()));
        // A z-straddling enclosure crosses the seam: copysign's honest
        // two-sided behavior widens rather than deciding — no poison,
        // no branch, the enclosure just gets wide (and b1.z = −(s·x)
        // spans both frames' values).
        let straddle = Vec3::new(
            Interval::from_f64(0.6),
            Interval::from_f64(0.8),
            Interval::from_bounds(-1e-12, 1e-12),
        );
        let (s1, _) = straddle.orthonormal_basis();
        assert!(s1.z.lo() <= -0.59 && s1.z.hi() >= 0.59);
    }

    /// **The GENERAL `n.z` enclosure, not just the point one #1157
    /// filed** — the row that would have caught the partial fix.
    ///
    /// #1157 reported `n.z = [0, 0]`, and a denominator written
    /// `1 + s·n.z` fixes exactly that case and no other: `copysign` at
    /// `Interval` is strict on both sides, so ANY `n.z` enclosure
    /// touching zero yields `s = [−1, 1]` and the product straddles.
    /// `1 + |n.z|` needs no sign decision at all. The two are
    /// bit-identical at `f64`, so no `f64` row can separate them — this
    /// is the one that does.
    ///
    /// The second half is a REGRESSION GUARD with teeth: it measures
    /// the old spelling directly and requires it to be unbounded at
    /// `n.z = [0, 1]`. If someone respells the denominator back, this
    /// reds instead of going quiet.
    #[cfg(feature = "interval")]
    #[test]
    fn orthonormal_basis_is_bounded_over_z_enclosures() {
        use crate::interval::Interval;
        use crate::real::Bounds;

        let iv = Interval::from_f64;
        let ivb = Interval::from_bounds;
        // One-sided, straddling, strictly-signed and degenerate: the
        // enclosures a subdivision driver actually produces.
        let zs = [
            ("[0,0]", ivb(0.0, 0.0)),
            ("[0,1]", ivb(0.0, 1.0)),
            ("[-1,0]", ivb(-1.0, 0.0)),
            ("[-1,1]", ivb(-1.0, 1.0)),
            ("[0.5,1]", ivb(0.5, 1.0)),
            ("[-1,-0.5]", ivb(-1.0, -0.5)),
            ("[0,1e-30]", ivb(0.0, 1e-30)),
        ];
        for (name, z) in zs {
            for (x, y) in [(0.0f64, 1.0f64), (1.0, 0.0), (0.6, 0.8), (0.0, 0.0)] {
                let (b1, b2) = Vec3::new(iv(x), iv(y), z).orthonormal_basis();
                for (e, which) in [
                    (b1.x, "b1.x"),
                    (b1.y, "b1.y"),
                    (b1.z, "b1.z"),
                    (b2.x, "b2.x"),
                    (b2.y, "b2.y"),
                    (b2.z, "b2.z"),
                ] {
                    assert!(
                        e.lo().is_finite() && e.hi().is_finite(),
                        "{which} at n.z = {name}, (x, y) = ({x}, {y}) is unbounded: \
                         [{}, {}]",
                        e.lo(),
                        e.hi()
                    );
                    assert!(
                        e.is_certified(),
                        "{which} at n.z = {name}, (x, y) = ({x}, {y}) cannot decide: \
                         [{}, {}]",
                        e.lo(),
                        e.hi()
                    );
                }
            }
        }
        // The regression guard: `1 + s·n.z` — the spelling this
        // replaced — is measurably NOT bounded at `[0, 1]`, which is
        // what makes the token load-bearing rather than cosmetic.
        let z = ivb(0.0, 1.0);
        let s = Interval::one().copysign(z);
        let old = Interval::one() / (Interval::one() + s * z);
        assert!(
            !old.lo().is_finite() || !old.hi().is_finite() || !old.is_certified(),
            "the `1 + s·n.z` spelling is supposed to fail at [0, 1]; it gave \
             [{}, {}] certified = {} — if this now holds, the guard is stale",
            old.lo(),
            old.hi(),
            old.is_certified()
        );
        // …and the shipped one is bounded and certified on the same input.
        let new = Interval::one() / (Interval::one() + z.abs());
        assert!(new.lo().is_finite() && new.hi().is_finite() && new.is_certified());
    }

    /// **#1157, at the input that manufactured the poison.** A VERTICAL
    /// plane's normal has `n.z = [0, 0]`, which contains zero without
    /// straddling it, so `copysign` must return the two-sided hull
    /// `[−1, 1]` — and the old spelling then divided by it. Every
    /// component here is BOUNDED and carries a decoration that may
    /// decide (`Def` or better); the `n.x = 0` case, which is what
    /// `newell_plane` hands the extrude side-wall builder, is the EXACT
    /// frame.
    ///
    /// The residual width in `b1.z` and `b2` is not slack: at `[0, 0]`
    /// the enclosure genuinely cannot tell `+0.0` from `−0.0`, the two
    /// give different (both valid, both right-handed) frames, and the
    /// hull of the two is the honest answer. It is asserted as the hull,
    /// so a future spelling that narrowed it by DECIDING the sign would
    /// red here.
    #[cfg(feature = "interval")]
    #[test]
    fn orthonormal_basis_at_a_vertical_plane_is_bounded_and_certified() {
        use crate::interval::Interval;
        use crate::real::Bounds;

        let iv = |x: f64| Interval::from_f64(x);
        for zero in [0.0f64, -0.0] {
            for (x, y) in [(0.0f64, 1.0f64), (0.0, -1.0), (1.0, 0.0), (0.6, 0.8)] {
                let n = Vec3::new(iv(x), iv(y), iv(zero));
                let (b1, b2) = n.orthonormal_basis();
                let f = Vec3::new(x, y, zero).orthonormal_basis();
                for (e, v, which) in [
                    (b1.x, f.0.x, "b1.x"),
                    (b1.y, f.0.y, "b1.y"),
                    (b1.z, f.0.z, "b1.z"),
                    (b2.x, f.1.x, "b2.x"),
                    (b2.y, f.1.y, "b2.y"),
                    (b2.z, f.1.z, "b2.z"),
                ] {
                    assert!(
                        e.lo().is_finite() && e.hi().is_finite(),
                        "{which} at n = ({x}, {y}, {zero}) is unbounded: [{}, {}]",
                        e.lo(),
                        e.hi()
                    );
                    assert!(
                        e.is_certified(),
                        "{which} at n = ({x}, {y}, {zero}) cannot decide: [{}, {}]",
                        e.lo(),
                        e.hi()
                    );
                    assert!(
                        e.lo() <= v && v <= e.hi(),
                        "{which} at n = ({x}, {y}, {zero}): f64 {v} outside [{}, {}]",
                        e.lo(),
                        e.hi()
                    );
                }
                // The frame flip is enclosed on BOTH sides, never decided.
                assert!(
                    b1.z.lo() <= -x && x <= b1.z.hi(),
                    "b1.z at n = ({x}, {y}, {zero}) drops a hemisphere: [{}, {}]",
                    b1.z.lo(),
                    b1.z.hi()
                );
            }
            // `newell_plane`'s own case: an axis-aligned vertical plane
            // with `n.x = 0` gets the exact frame, not merely a bounded
            // one — the chart residual it feeds is then exactly zero.
            let (b1, _) = Vec3::new(iv(0.0), iv(-1.0), iv(zero)).orthonormal_basis();
            for (e, want, which) in [
                (b1.x, 1.0, "b1.x"),
                (b1.y, 0.0, "b1.y"),
                (b1.z, 0.0, "b1.z"),
            ] {
                assert!(
                    e.lo() == want && e.hi() == want,
                    "{which}: [{}, {}] is not the exact {want}",
                    e.lo(),
                    e.hi()
                );
            }
        }
    }
}
