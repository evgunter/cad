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
    /// Builds a vector from its components. A `const fn` (the doctest
    /// at [`super::Point3::new`] reads a constant of each of the four
    /// types).
    pub const fn new(x: T, y: T) -> Self {
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
    /// Builds a vector from its components. A `const fn` (the doctest
    /// at [`super::Point3::new`] reads a constant of each of the four
    /// types).
    pub const fn new(x: T, y: T, z: T) -> Self {
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
    /// `onto`: the triple product `(onto × self) × onto / |onto|²`.
    ///
    /// The association is part of the contract, exactly as it is for
    /// [`Vec3::project_onto`] (the M2 watchlist's D9 hazard). Order:
    /// `onto.cross(self)` first, in [`Vec3::cross`]'s own association;
    /// that crossed with `onto`; then **one division per component** by
    /// [`Vec3::norm_squared`]. Call sites must use this method, never
    /// re-derive a rejection with their own grouping.
    ///
    /// **`self` is mentioned once — and `onto` is amplified.** That is
    /// the whole trade, and both halves are load-bearing:
    ///
    /// - **The gain, on `self`.** `self − self.project_onto(onto)` is
    ///   the same vector over the reals but names `self` twice, so at an
    ///   enclosure scalar it charges about `2·width(self)` to the
    ///   components ALONG `onto`, where the rejection does not depend on
    ///   `self` at all and its true width is zero. The triple product
    ///   charges nothing there: rejecting an enclosure of half-width
    ///   1e-9 from `+z` gives a `z` component of width exactly zero
    ///   rather than 4e-9. Per component, over the corpus in
    ///   `geom-core/tests/props1_evidence.rs`, an exact `onto` against a
    ///   `self` carrying width narrows by 0.95× to 4× plus seven
    ///   components that become exactly zero.
    /// - **The cost, on `onto`.** `onto` is named three times here as it
    ///   was three times before, but two of those mentions are now cross
    ///   products rather than one scalar quotient, so a `self`-shaped
    ///   width in `onto` is AMPLIFIED rather than merely repeated. Where
    ///   `onto` itself carries width the shipped rejection is up to
    ///   **34× wider** than the subtractive spelling on that corpus, and
    ///   a randomized sweep with a zero-straddling `onto` component
    ///   reaches 1022×.
    ///
    /// **So `onto` is expected to be an exact or narrow direction**, and
    /// a wide `onto` is the case where the subtractive spelling was the
    /// tighter one. That is the shape every caller in this kernel has —
    /// a stored axis or a unit normal as `onto`, a computed and often
    /// wide vector as `self` — which is why the trade is taken this way
    /// round. At `f64` an exactly parallel pair rejects to exactly the
    /// zero vector.
    ///
    /// # The two rounding claims, measured
    ///
    /// One metric throughout: **ulps of the vector's largest
    /// component**, so a 100 m coordinate and a 1 mm coordinate are held
    /// to the same absolute scale. Measured in
    /// `geom-core/tests/props1_evidence.rs` and, adversarially, in
    /// `props1_review_rows.rs`.
    ///
    /// - **Orthogonal to `onto`**: `|reject · onto|` is at most
    ///   **3.5e-17** of `|self|·|onto|` on that corpus. The cross
    ///   products put the result in the plane through the origin normal
    ///   to `onto` by construction, so this is a property of the
    ///   spelling and not of the inputs.
    /// - **`project + reject` returns `self`** to within **4 ulps** of
    ///   the largest component — 1 ulp on the corpus, 4 over 200 000
    ///   adversarial pairs (near-parallel, near-orthogonal, magnitudes
    ///   from 1e-8 to 1e8). Judged per component instead, the same sweep
    ///   reaches 65536 ulps of a component that is itself near zero,
    ///   which is why the metric is stated. A caller who needs the split
    ///   to re-sum bit-exactly must keep `self` and subtract rather than
    ///   adding the two halves back.
    ///
    /// **Total.** A zero (or poisoned) `onto` yields all-poison
    /// components through the 0/0 division. The overflow and underflow
    /// bands are NOT [`Vec3::project_onto`]'s: that method's numerator
    /// is `self.dot(onto)`, this one's is `(onto × self) × onto`, which
    /// scales as `|onto|²·|self|` and therefore leaves the finite range
    /// sooner from both ends. `onto` near 1e150 with `|self|` near 1e20
    /// overflows the numerator to `±∞` and returns infinities where the
    /// subtractive spelling was exact; `onto` below ~1e-140 pushes the
    /// numerator into the subnormals and then to zero, so a `self`
    /// entirely orthogonal to `onto` rejects to a silent exact zero.
    /// Both bands are far outside the session box (D4 ¶4) — the same
    /// posture and the same kind of boundary [`Vec3::normalize`]'s doc
    /// note states for its own ~1e154 band — and both are pinned in
    /// `props1_review_rows.rs`.
    pub fn reject_from(self, onto: Self) -> Self {
        onto.cross(self).cross(onto) / onto.norm_squared()
    }

    /// An orthonormal basis completing `self` (a **unit** vector) to a
    /// right-handed frame: returns `(b1, b2)` with `(b1, b2, self)`
    /// orthonormal and right-handed (`b1 × b2 = self` up to rounding).
    ///
    /// **The construction.** Cross the normal `n = self` with a world
    /// axis chosen by one comparison on `n.z`, and normalize:
    ///
    /// ```text
    /// b1 = normalize(e_z × n)  when |n.z| ≤ max(|n.x|, |n.y|),
    ///      normalize(e_y × n)  otherwise
    /// b2 = n × b1
    /// ```
    ///
    /// The comparison is an order on the normal's own components — "is
    /// `n` nearer the equator than the poles" — so it is
    /// SCALE-INVARIANT, introduces no constant, and cannot overflow at
    /// any magnitude (only `abs` and `max` enter it).
    ///
    /// `b1 × b2 = b1 × (n × b1) = n·(b1·b1) − b1·(b1·n) = n` exactly in
    /// ℝ, so the frame is right-handed by construction rather than by a
    /// sign convention. **No sign is transferred anywhere**, which is
    /// the whole point: [`Real::copysign`]'s enclosure arm must hull at
    /// any zero-containing sign, so a construction whose seam runs
    /// through the equator hulls the frame of every vertical wall.
    ///
    /// **Why this comparison and this side of the cross**, in the terms
    /// a user would state them: a vertical wall `n = (c, s, 0)` takes
    /// `e_z` and gets `b1 = (−s, c, 0)` — horizontal in the plane —
    /// with `b2 = e_z`, up; a horizontal cap `n = ±e_z` takes `e_y` and
    /// gets `b1 = ±e_x`, `b2 = e_y`. Those are the frames a draughtsman
    /// would draw.
    ///
    /// **Conditioning: no seam on the sphere.**
    /// `|e_z × n|² = n.x² + n.y²` and `|e_y × n|² = n.x² + n.z²`. On
    /// the `e_z` arm `n.z² ≤ max(n.x², n.y²) ≤ n.x² + n.y²`, so
    /// `‖n‖² ≤ 2(n.x² + n.y²)` and `|e_z × n|² ≥ ‖n‖²/2`. On the `e_y`
    /// arm `n.z²` strictly exceeds both `n.x²` and `n.y²`, so
    /// `‖n‖² < 3n.z²` and `|e_y × n|² ≥ n.z² > ‖n‖²/3`. The
    /// normalization is therefore well conditioned at every direction,
    /// unit or not — there is no direction at which this is
    /// near-degenerate, and in particular nothing happens at the
    /// equator `n.z = 0`, where every vertical wall of every extrusion
    /// lives.
    ///
    /// **The discontinuity, documented honestly.** One must exist (no
    /// continuous global frame on the sphere — hairy ball). Here it is
    /// the 45° cone `|n.z| = max(|n.x|, |n.y|)`: crossing it turns the frame
    /// about `n` by a quarter turn, never a flip. **What is on that
    /// cone matters more than that it exists**: no axis direction, no
    /// axis-aligned face, no vertical wall and no horizontal cap. That
    /// is what makes the seam affordable at an enclosure scalar, where
    /// a normal whose components are noise around a seam cannot be
    /// decided. An order over all three components — smallest magnitude
    /// wins — puts the seam through EVERY axis direction instead (two
    /// components tied at zero), which measures worse on exactly the
    /// geometry a CAD kernel is made of. Consumers wanting a *stable*
    /// frame across parameter changes store the frame (`u_ref`) as
    /// data, per D2; this constructor is for *making* that data.
    ///
    /// **The comparison is a value-level door.** It goes through
    /// [`Real::select_le_zero`], applied componentwise to the two
    /// candidate vectors — three scalar calls on the SAME decision, so
    /// an undecided enclosure answers with the hull of the two
    /// candidate VECTORS rather than a box over three independent
    /// choices. The door's tie-break keys on a value zero rather than a
    /// zero's sign bit, which is what lets an enclosure decide it;
    /// spelling the choice as `copysign` on the difference would be a
    /// total order at `f64` and a hull at every point tie.
    ///
    /// **Evaluation order (fixed, D9).** Exactly as written:
    ///
    /// ```text
    /// c_y = normalize(( n.z,  0,   −n.x))          // e_y × n
    /// c_z = normalize((−n.y,  n.x,  0  ))          // e_z × n
    /// b1  = select(|n.z| − max(|n.x|, |n.y|), c_z, c_y)  // ties → e_z
    /// b2  = n × b1
    /// ```
    ///
    /// **Each candidate is normalized before the selection, not after
    /// it**, and that ordering is load-bearing at `Interval`. Selecting
    /// first and normalizing once would divide a straddled tie's HULL
    /// by its own norm enclosure, and that hull can contain the zero
    /// vector — an unbounded, `Trv` answer to a question that is real
    /// at every point of the box, which `docs/DUAL-DESIGN.md` DL6
    /// forbids. Normalizing first makes the undecided answer the hull
    /// of two unit vectors: bounded, decorated `Def`, and containing
    /// whichever frame the `f64` program picked. At `f64` the two
    /// orderings are bit-identical, since exactly one candidate is
    /// read.
    ///
    /// **The unread candidate may be poison, and that is by design.**
    /// `e_k × n` is the zero vector exactly when `n` is parallel to
    /// `e_k`, so `normalize` poisons `c_z` at `n = ±e_z` and `c_y` at
    /// `n = ±e_y`. The comparison guarantees such a candidate is never
    /// the one selected, at ANY magnitude: the `e_z` arm needs
    /// `max(|n.x|, |n.y|) ≥ |n.z|`, which makes `c_z` the zero vector
    /// only for the zero vector itself, and the `e_y` arm needs
    /// `|n.z| > max(|n.x|, |n.y|)`, which is impossible where `c_y` is
    /// zero (`n.x = n.z = 0`) — and [`Real::select_le_zero`] propagates poison only from
    /// the arm it reads. A poisoned INPUT still poisons everything,
    /// through the decision.
    ///
    /// **When the answer is unbounded, and why that is honest.**
    /// `normalize` reads each candidate's OWN norm, so an unbounded
    /// answer needs the comparison undecided AND a candidate that is
    /// the zero vector somewhere in the box. Those two together force
    /// the box to contain the ZERO VECTOR: `c_z` vanishes only where
    /// `n.x = n.y = 0`, which puts `max(|n.x|, |n.y|)` at zero, and an
    /// undecided comparison then puts `|n.z|` at zero too; `c_y`
    /// vanishes only where `n.x = n.z = 0`, and an undecided comparison
    /// there puts `n.y` at zero as well. The zero vector names no
    /// direction, so a box containing it poses no question at that
    /// point and DL6 is satisfied — the construction manufactures a
    /// non-real only where one entered.
    ///
    /// Both squares are the tight square (`powi(2)`), not the product
    /// `n·n`: at `Interval` the product treats the two factors as
    /// independent, so an enclosure straddling zero acquires a spurious
    /// negative lower bound.
    ///
    /// **Precondition (conventional, unchecked):** `self` is unit —
    /// same posture as unit-`dir` curve data; tier-3 certification owns
    /// the invariant. A non-unit input is still total and still says
    /// something exact: `b1` is unit and orthogonal to `n` whatever
    /// `‖n‖` is, `b2 = n × b1` carries `‖n‖`, and `b1 × b2 = n` holds —
    /// an ORTHOGONAL pair that is not orthonormal. What the precondition
    /// buys is orthonormality itself; the conditioning bound
    /// and the never-selected-poison argument hold at any magnitude,
    /// because the comparison is scale-invariant. A poisoned input
    /// propagates poison, and the zero vector — which names no
    /// direction — poisons.
    pub fn orthonormal_basis(self) -> (Self, Self) {
        let zero = T::zero();
        let cy = Self::new(self.z, zero, -self.x).normalize();
        let cz = Self::new(-self.y, self.x, zero).normalize();
        let d = self.z.abs() - self.x.abs().max(self.y.abs());
        let b1 = Self::select(d, cz, cy);
        (b1, self.cross(b1))
    }

    /// [`Real::select_le_zero`] componentwise on one decision — `when_le`
    /// if `d ≤ 0`, else `when_gt`.
    ///
    /// The SAME `d` steers all three components, which is what makes an
    /// undecided enclosure the hull of the two candidate VECTORS rather
    /// than a box over three independent choices.
    fn select(d: T, when_le: Self, when_gt: Self) -> Self {
        Self::new(
            d.select_le_zero(when_le.x, when_gt.x),
            d.select_le_zero(when_le.y, when_gt.y),
            d.select_le_zero(when_le.z, when_gt.z),
        )
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

        /// The TANGENT channel of the frame, against its closed form —
        /// the channel the value-channel bit row above cannot reach.
        ///
        /// Away from the 45° cone the axis choice is locally constant,
        /// so `b1 = v/‖v‖` for the fixed `v = e_k × n`, which is LINEAR
        /// in `n`; differentiating,
        ///
        /// ```text
        /// v'   = e_k × n'
        /// b1'  = v'/‖v‖ − v·(v·v')/‖v‖³
        /// b2'  = n' × b1 + n × b1'
        /// ```
        ///
        /// Well conditioned everywhere on the sphere: `‖v‖² = 1 − n_k²`
        /// is at least `‖n‖²/3` on both arms of the comparison, which
        /// is the whole reason the comparison is what it is. The row
        /// therefore
        /// needs no near-degenerate exclusion — only the cone itself,
        /// where the derivative does not exist and the program's answer
        /// is one side's (`prop_assume!` below drops a draw within a
        /// whisker of it).
        ///
        /// **What it catches that the value channel cannot**: a wrong
        /// normalization derivative, a dropped `v·(v·v')` term, or a
        /// tangent that followed the unchosen candidate — every one of
        /// which leaves all six `f64` bits identical.
        ///
        /// **The tangents are NOT 1.** `Dual::variable` gives every
        /// input a tangent of `1.0`, at which the product rule and the
        /// power rule agree bit-for-bit; independent random tangents are
        /// what make this a test of the rule.
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
            // Off the seam, where the axis choice is locally constant.
            prop_assume!((n.z.abs() - n.x.abs().max(n.y.abs())).abs() > 1e-6);
            let t = Vec3::new(tx, ty, tz);
            let axis = if n.z.abs() <= n.x.abs().max(n.y.abs()) {
                Vec3::new(0.0, 0.0, 1.0)
            } else {
                Vec3::new(0.0, 1.0, 0.0)
            };
            let w = axis.cross(n);
            let wd = axis.cross(t);
            let norm = w.norm();
            let b1 = w / norm;
            let b1d = wd / norm - w * (w.dot(wd) / (norm * norm * norm));
            let b2d = t.cross(b1) + n.cross(b1d);
            let nd = Vec3::new(
                Dual::new(n.x, tx),
                Dual::new(n.y, ty),
                Dual::new(n.z, tz),
            );
            let (d1, d2) = nd.orthonormal_basis();
            for (got, want, which) in [
                (d1.x.deriv, b1d.x, "b1.x"),
                (d1.y.deriv, b1d.y, "b1.y"),
                (d1.z.deriv, b1d.z, "b1.z"),
                (d2.x.deriv, b2d.x, "b2.x"),
                (d2.y.deriv, b2d.y, "b2.y"),
                (d2.z.deriv, b2d.z, "b2.z"),
            ] {
                let scale = want.abs().max(1.0);
                prop_assert!(
                    (got - want).abs() <= 1e-12 * scale,
                    "{} tangent {} vs closed form {} \
                     (n = {:?}, tx = {}, ty = {}, tz = {})",
                    which, got, want, (n.x, n.y, n.z), tx, ty, tz
                );
            }
        }

        /// **The pinned `f64` spelling, swept bitwise**: `b1` is
        /// `normalize(e_z × n)` when `|n.z| ≤ max(|n.x|, |n.y|)` and
        /// `normalize(e_y × n)` otherwise, and `b2` is `n × b1`. The
        /// reference below writes that out with a raw `if` — the
        /// spelling the constructor may not use, since a value branch
        /// does not survive an enclosure scalar — so the two
        /// derivations are independent and the row measures the door
        /// rather than restating it.
        ///
        /// The sweep is the drawn direction plus the edge set the
        /// construction meets: the axes and the equator with both
        /// signed zeros in `z` (the bits an enclosure cannot see), the
        /// poles, and the 45° CONE `|n.z| = max(|n.x|, |n.y|)` — the
        /// discontinuity,
        /// and the only place a reference and a spelling can disagree
        /// by a rotation about `n` rather than by an ulp.
        ///
        /// Poison is out of scope on purpose (NaN bits are not a
        /// contract — `project_reject_basis_poison` owns that door), so
        /// a case whose reference frame is not finite is skipped.
        #[test]
        fn orthonormal_basis_matches_the_pinned_spelling_bitwise(v in vec3()) {
            /// The comparison, with the branch written out.
            fn reference(n: Vec3<f64>) -> (Vec3<f64>, Vec3<f64>) {
                let axis = if n.z.abs() <= n.x.abs().max(n.y.abs()) {
                    Vec3::new(-n.y, n.x, 0.0)
                } else {
                    Vec3::new(n.z, 0.0, -n.x)
                };
                let b1 = axis.normalize();
                (b1, n.cross(b1))
            }
            let mut cases = vec![v, v.normalize(), Vec3::new(v.x, v.y, 0.0)];
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
            // The 45° cone itself and both sides of it, at several
            // azimuths and both hemispheres.
            let half = core::f64::consts::FRAC_1_SQRT_2;
            for d in [0.0f64, 1e-12, -1e-12] {
                for (x, y) in [(1.0f64, 0.0f64), (0.0, 1.0), (0.6, 0.8), (-0.6, 0.8)] {
                    for s in [1.0f64, -1.0] {
                        let z = s * (half + d);
                        let r = (1.0 - z * z).max(0.0).sqrt();
                        cases.push(Vec3::new(x * r, y * r, z));
                    }
                }
            }
            for n in cases {
                let (w1, w2) = reference(n);
                let finite = |a: Vec3<f64>| a.x.is_finite() && a.y.is_finite() && a.z.is_finite();
                if !finite(w1) || !finite(w2) {
                    continue;
                }
                let (g1, g2) = n.orthonormal_basis();
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

    /// The conventional frames at the axes, at a vertical wall and at a
    /// horizontal cap, continuity away from the seam, and the seam
    /// itself — the 45° cone `|n.z| = max(|n.x|, |n.y|)`.
    #[test]
    fn orthonormal_basis_poles_walls_and_the_cone() {
        // A horizontal cap: `|n.z| > max(|n.x|, |n.y|)` takes the `e_y` arm, and
        // the frame is the one a draughtsman draws — `x` across, `y` up
        // the page.
        let (b1, b2) = Vec3::<f64>::unit_z().orthonormal_basis();
        assert_eq!((b1.x, b1.y, b1.z), (1.0, 0.0, 0.0));
        assert_eq!((b2.x, b2.y, b2.z), (0.0, 1.0, 0.0));
        let (b1, b2) = (-Vec3::<f64>::unit_z()).orthonormal_basis();
        assert_eq!((b1.x, b1.y, b1.z), (-1.0, 0.0, 0.0));
        assert_eq!((b2.x, b2.y, b2.z), (0.0, 1.0, 0.0));
        // The other four axis directions, each an exact frame.
        for (n, w1, w2) in [
            (Vec3::unit_x(), Vec3::unit_y(), Vec3::<f64>::unit_z()),
            (-Vec3::<f64>::unit_x(), -Vec3::unit_y(), Vec3::unit_z()),
            (Vec3::<f64>::unit_y(), -Vec3::unit_x(), Vec3::unit_z()),
            (-Vec3::<f64>::unit_y(), Vec3::unit_x(), Vec3::unit_z()),
        ] {
            let (b1, b2) = n.orthonormal_basis();
            assert_eq!((b1.x, b1.y, b1.z), (w1.x, w1.y, w1.z), "b1 at {n:?}");
            assert_eq!((b2.x, b2.y, b2.z), (w2.x, w2.y, w2.z), "b2 at {n:?}");
        }
        // A vertical wall — the whole equator, where `n.z² = 0` takes
        // the `e_z` arm: `b1` is horizontal in the plane and `b2` is
        // up. The old sign-transfer construction put its seam here.
        let wall = Vec3::new(0.6, 0.8, 0.0);
        let (b1, b2) = wall.orthonormal_basis();
        assert_eq!((b1.x, b1.y, b1.z), (-0.8, 0.6, 0.0));
        assert_eq!((b2.x, b2.y, b2.z), (0.0, 0.0, 1.0));
        // Near a pole (the direction the naive `1/(1 + n.z)` spelling
        // cancels on): still orthonormal to a few ulps.
        let n = Vec3::new(1e-9, -1e-9, -1.0).normalize();
        let (b1, b2) = n.orthonormal_basis();
        assert!((b1.norm() - 1.0).abs() <= 1e-14);
        assert!((b2.norm() - 1.0).abs() <= 1e-14);
        assert!(b1.dot(b2).abs() <= 1e-14);
        assert!(b1.dot(n).abs() <= 1e-14);
        assert!(b2.dot(n).abs() <= 1e-14);
        // Continuity across the equator, which is no longer a seam: two
        // normals a picometre either side of `n.z = 0` give frames a
        // picometre apart.
        let above = Vec3::new(0.6, 0.8, 1e-12).normalize();
        let below = Vec3::new(0.6, 0.8, -1e-12).normalize();
        let (a1, _) = above.orthonormal_basis();
        let (c1, _) = below.orthonormal_basis();
        assert!(
            (a1.x - c1.x).abs() <= 1e-11,
            "a1.x = {}, c1.x = {}",
            a1.x,
            c1.x
        );
        assert!((a1.y - c1.y).abs() <= 1e-11);
        assert!((a1.z - c1.z).abs() <= 1e-11);
        // The seam that does exist: the 45° cone `|n.z| = max(|n.x|, |n.y|)`, which
        // carries no axis direction and no axis-aligned face. Crossing
        // it turns the frame a QUARTER TURN about `n` — the two
        // candidates are orthogonal there, since
        // `c_z · c_y = −n.y·n.z/((1 − n.z²)^½(1 − n.y²)^½)` and this
        // fixture's `n.y` is zero — and never a flip. Both sides are
        // exact right-handed frames of the same plane.
        let half = core::f64::consts::FRAC_1_SQRT_2;
        let on = |z: f64| Vec3::new((1.0 - z * z).sqrt(), 0.0, z);
        let left = on(half - 1e-12);
        let right = on(half + 1e-12);
        let (l1, l2) = left.orthonormal_basis();
        let (r1, r2) = right.orthonormal_basis();
        assert!(
            l1.dot(r1).abs() <= 1e-9,
            "the cone's rotation is not a quarter turn: {}",
            l1.dot(r1)
        );
        assert!((l1.dot(r2).abs() - 1.0).abs() <= 1e-9);
        for (b1, b2, n) in [(l1, l2, left), (r1, r2, right)] {
            let cross = b1.cross(b2);
            assert!((cross.x - n.x).abs() <= 1e-14);
            assert!((cross.y - n.y).abs() <= 1e-14);
            assert!((cross.z - n.z).abs() <= 1e-14);
            assert!(b1.dot(n).abs() <= 1e-14);
            assert!(b2.dot(n).abs() <= 1e-14);
        }
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

    /// The basis construction at the interval scalar: instantiates, the
    /// orthonormality residuals enclose 0 for a point-enclosure unit
    /// input (the containment form of the `f64` properties above), and
    /// the two answers the axis order gives an enclosure — decided and
    /// hulled — are both honest.
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
        // A `z`-straddling enclosure at a WALL is not a seam any more:
        // `|n.z|` is nowhere near `max(|n.x|, |n.y|)`, so it DECIDES and
        // the frame comes back exactly horizontal.
        let straddle = Vec3::new(
            Interval::from_f64(0.6),
            Interval::from_f64(0.8),
            Interval::from_bounds(-1e-12, 1e-12),
        );
        let (s1, _) = straddle.orthonormal_basis();
        assert!(
            s1.z.lo() == 0.0 && s1.z.hi() == 0.0,
            "a straddling n.z still widens the frame: [{}, {}]",
            s1.z.lo(),
            s1.z.hi()
        );
        assert!((s1.x.hi() - s1.x.lo()) <= 8.0 * f64::EPSILON);
        // The seam that IS one: an enclosure straddling the 45° cone
        // `|n.z| = max(|n.x|, |n.y|)`. The door hulls the two candidate frames —
        // bounded, decorated `Def`, never a manufactured non-real — and
        // the hull contains BOTH of the frames the box's points take.
        let half = core::f64::consts::FRAC_1_SQRT_2;
        let tie = Vec3::new(
            Interval::from_f64(half),
            Interval::from_f64(0.0),
            Interval::from_bounds(half - 1e-6, half + 1e-6),
        );
        let (t1, t2) = tie.orthonormal_basis();
        for (e, which) in [
            (t1.x, "b1.x"),
            (t1.y, "b1.y"),
            (t1.z, "b1.z"),
            (t2.x, "b2.x"),
            (t2.y, "b2.y"),
            (t2.z, "b2.z"),
        ] {
            assert!(
                e.lo().is_finite() && e.hi().is_finite() && e.is_certified(),
                "{which} at the tie is not a usable enclosure: [{}, {}]",
                e.lo(),
                e.hi()
            );
        }
        for z in [half - 1e-6, half + 1e-6] {
            let (f1, _) = Vec3::new(half, 0.0, z).normalize().orthonormal_basis();
            for (e, v, which) in [
                (t1.x, f1.x, "b1.x"),
                (t1.y, f1.y, "b1.y"),
                (t1.z, f1.z, "b1.z"),
            ] {
                assert!(
                    e.lo() <= v && v <= e.hi(),
                    "{which}: the f64 frame at n.z = {z} gives {v}, outside [{}, {}]",
                    e.lo(),
                    e.hi()
                );
            }
        }
    }

    /// **Bounded and certified over every `n.z` enclosure** — one-sided,
    /// straddling zero, strictly signed and degenerate, the enclosures
    /// a subdivision driver actually produces at a wall — and over a
    /// tight enclosure straddling the 45° cone, where the answer is the
    /// hull of two unit candidates, and over a whole meridian, which
    /// the comparison still decides.
    ///
    /// **The limit, measured rather than implied**: an unbounded answer
    /// needs a box containing the ZERO VECTOR, which names no direction
    /// (the constructor's docs derive this). The row measures that too.
    ///
    /// The second half is a REGRESSION GUARD with teeth on the one
    /// ordering decision the construction makes: it measures the
    /// select-THEN-normalize spelling directly and requires it to be
    /// unbounded at a straddled cone. The hull of two un-normalized
    /// candidates contains the zero vector there, so dividing it by its
    /// own norm enclosure manufactures a non-real from a question that
    /// is real at every point of the box (DL6). Normalizing each
    /// candidate FIRST makes the same answer the hull of two unit
    /// vectors. If someone reorders those two steps, this reds instead
    /// of going quiet.
    #[cfg(feature = "interval")]
    #[test]
    fn orthonormal_basis_is_bounded_over_z_enclosures() {
        use crate::interval::Interval;
        use crate::real::Bounds;

        let iv = Interval::from_f64;
        let ivb = Interval::from_bounds;
        let half = core::f64::consts::FRAC_1_SQRT_2;
        let zs = [
            ("[0,0]", ivb(0.0, 0.0)),
            ("[-1e-9,1e-9]", ivb(-1e-9, 1e-9)),
            ("[0,0.5]", ivb(0.0, 0.5)),
            ("[-0.5,0]", ivb(-0.5, 0.0)),
            ("[0.9,1]", ivb(0.9, 1.0)),
            ("[-1,-0.9]", ivb(-1.0, -0.9)),
            ("[0,1e-30]", ivb(0.0, 1e-30)),
            // Tight, straddling the cone: the hull of two unit
            // candidates, which must still be bounded and certified.
            ("cone±1e-9", ivb(half - 1e-9, half + 1e-9)),
            ("-cone±1e-9", ivb(-half - 1e-9, -half + 1e-9)),
        ];
        for (name, z) in zs {
            for (x, y) in [(0.0f64, 1.0f64), (1.0, 0.0), (0.6, 0.8)] {
                let r = (1.0 - z.hi() * z.hi()).max(0.0).sqrt();
                let (b1, b2) = Vec3::new(iv(x * r), iv(y * r), z).orthonormal_basis();
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
        // A whole meridian at the azimuth whose `e_y` candidate
        // degenerates: still DECIDED, because `max(|n.x|, |n.y|)` is 1
        // there and `|n.z|` never exceeds it.
        let (wide, _) = Vec3::new(iv(0.0), iv(1.0), ivb(0.0, 1.0)).orthonormal_basis();
        for (e, want, which) in [(wide.x, -1.0, "b1.x"), (wide.y, 0.0, "b1.y")] {
            assert!(
                e.lo() == want && e.hi() == want,
                "{which} over a whole meridian: [{}, {}] is not the exact {want}",
                e.lo(),
                e.hi()
            );
        }
        // The limit: a box containing the zero vector, which names no
        // direction. Recorded, not demanded.
        let (origin, _) =
            Vec3::new(ivb(-1.0, 1.0), ivb(-1.0, 1.0), ivb(-1.0, 1.0)).orthonormal_basis();
        println!(
            "note: n = [-1, 1]^3 (contains the zero vector) gives b1.x = [{}, {}] \
             (bounded: {})",
            origin.x.lo(),
            origin.x.hi(),
            origin.x.lo().is_finite() && origin.x.hi().is_finite()
        );
        // The guard: a straddled cone, where the two un-normalized
        // candidates' hull contains the zero vector.
        let n = Vec3::new(iv(half), iv(0.0), ivb(half - 1e-9, half + 1e-9));
        let d = n.z.abs() - n.x.abs().max(n.y.abs());
        let cz = Vec3::new(-n.y, n.x, Interval::zero());
        let cy = Vec3::new(n.z, Interval::zero(), -n.x);
        let hull = Vec3::new(
            d.select_le_zero(cz.x, cy.x),
            d.select_le_zero(cz.y, cy.y),
            d.select_le_zero(cz.z, cy.z),
        );
        let late = hull.normalize();
        assert!(
            !late.x.lo().is_finite() || !late.x.hi().is_finite() || !late.x.is_certified(),
            "select-then-normalize is supposed to fail at a straddled cone; it gave \
             [{}, {}] certified = {} — if this now holds, the guard is stale",
            late.x.lo(),
            late.x.hi(),
            late.x.is_certified()
        );
        // …and the shipped ordering is bounded and certified there.
        let (b1, _) = n.orthonormal_basis();
        assert!(b1.x.lo().is_finite() && b1.x.hi().is_finite() && b1.x.is_certified());
    }

    /// **The equator, at the input the sign-transfer construction could
    /// not answer.** A VERTICAL plane's normal has `n.z = 0`, so
    /// `|n.z| = 0 ≤ max(|n.x|, |n.y|)` DECIDES: the frame is the EXACT in-plane
    /// horizontal, not a bounded hull of two hemispheres. The
    /// comparison reads a squared value and not a sign bit, so `+0.0`
    /// and `−0.0` give the same answer and a point enclosure of either
    /// decides.
    ///
    /// The horizontal cap `n = ±e_z` is the other exact case, at the
    /// far end of the same comparison (`|n.z| = 1 > 0`).
    #[cfg(feature = "interval")]
    #[test]
    fn orthonormal_basis_at_a_vertical_plane_and_a_cap_is_exact() {
        use crate::interval::Interval;
        use crate::real::Bounds;

        let iv = |x: f64| Interval::from_f64(x);
        let exact = |e: Interval, want: f64, which: &str, n: (f64, f64, f64)| {
            assert!(
                e.lo() == want && e.hi() == want,
                "{which} at n = {n:?}: [{}, {}] is not the exact {want}",
                e.lo(),
                e.hi()
            );
        };
        for zero in [0.0f64, -0.0] {
            // Axis-aligned vertical walls: exact frames, both signs of
            // the zero, in both directions.
            for (x, y) in [(0.0f64, 1.0f64), (0.0, -1.0), (1.0, 0.0), (-1.0, 0.0)] {
                let n = Vec3::new(iv(x), iv(y), iv(zero));
                let (b1, b2) = n.orthonormal_basis();
                for (e, want, which) in [
                    (b1.x, -y, "b1.x"),
                    (b1.y, x, "b1.y"),
                    (b1.z, 0.0, "b1.z"),
                    (b2.x, 0.0, "b2.x"),
                    (b2.y, 0.0, "b2.y"),
                    (b2.z, 1.0, "b2.z"),
                ] {
                    exact(e, want, which, (x, y, zero));
                }
            }
            // A wall off the axes: not exact (the normalization rounds),
            // but tight — no hemisphere is hulled in.
            let (b1, _) = Vec3::new(iv(0.6), iv(0.8), iv(zero)).orthonormal_basis();
            for (e, which) in [(b1.x, "b1.x"), (b1.y, "b1.y"), (b1.z, "b1.z")] {
                assert!(
                    e.hi() - e.lo() <= 8.0 * f64::EPSILON,
                    "{which} at a wall is not tight: [{}, {}]",
                    e.lo(),
                    e.hi()
                );
            }
        }
        // The horizontal cap, both poles and both zeros in x and y.
        for zx in [0.0f64, -0.0] {
            for s in [1.0f64, -1.0] {
                let n = Vec3::new(iv(zx), iv(zx), iv(s));
                let (b1, b2) = n.orthonormal_basis();
                for (e, want, which) in [
                    (b1.x, s, "b1.x"),
                    (b1.y, 0.0, "b1.y"),
                    (b1.z, 0.0, "b1.z"),
                    (b2.x, 0.0, "b2.x"),
                    (b2.y, 1.0, "b2.y"),
                    (b2.z, 0.0, "b2.z"),
                ] {
                    exact(e, want, which, (zx, zx, s));
                }
            }
        }
    }
}
