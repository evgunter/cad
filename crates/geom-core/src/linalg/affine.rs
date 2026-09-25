//! Affine maps of the 3-D affine space.
//!
//! See the [module docs](super) for the affine/linear split: an affine map
//! is a linear part plus a translation. The linear part is the map's
//! *differential* — constant, because affine maps are exactly the maps
//! whose differential is constant — and is therefore what pushes tangent
//! vectors forward ([`Affine3::transform_vec`]); the translation is felt
//! only by points ([`Affine3::transform_point`]).

use core::convert::Infallible;
use core::ops::Mul;

use crate::linalg::{Mat3, Point3, Vec3};
use crate::real::Real;

/// An affine map of the 3-D affine space: `p ↦ linear·p + translation`.
#[derive(Clone, Copy, Debug)]
pub struct Affine3<T: Real> {
    /// The linear part — the map's differential.
    pub linear: Mat3<T>,
    /// The translation part — the image of the coordinate origin, as a
    /// displacement from it.
    pub translation: Vec3<T>,
}

impl<T: Real> Affine3<T> {
    /// Builds an affine map from its linear part and translation. A
    /// `const fn` (the doctest at [`Point3::new`] reads a constant
    /// placement built through it).
    ///
    /// Everything it stores, it stores VERBATIM. Where a caller builds
    /// a placement from a base point — the frame witness's
    /// [`OrthoFrame::to_affine`](crate::OrthoFrame::to_affine) is where
    /// that happens — the translation is the subtraction
    /// `origin − Point3::origin()`: componentwise `x − (+0.0)`, which
    /// IEEE 754 leaves at `x` for every bit pattern (`−0.0`, `±inf` and
    /// NaN payloads included, which the corpus row in this file's tests
    /// measures), so a placement read back from the stored map, columns
    /// and translation, is bitwise the one that was written.
    pub const fn from_parts(linear: Mat3<T>, translation: Vec3<T>) -> Self {
        Self {
            linear,
            translation,
        }
    }

    /// The identity map.
    pub fn identity() -> Self {
        Self::from_parts(Mat3::identity(), Vec3::zero())
    }

    /// The same affine map read at another scalar: the linear part
    /// through [`Mat3::map`], the translation through [`Vec3::map`]. A
    /// structural map — no arithmetic, so exact whenever `f` is
    /// (`Real::from_f64` carries a stored `f64` placement to any
    /// evaluation scalar).
    ///
    /// ONE body with [`Self::try_map`]: this is that walk under an `f`
    /// that cannot refuse. The twelve components' placement — three
    /// columns then the translation — is therefore written once for
    /// both directions, and a transposed `c1`/`c2` cannot be true of
    /// one and false of the other.
    #[must_use]
    pub fn map<U: Real>(self, f: impl Fn(T) -> U) -> Affine3<U> {
        // An `f` that cannot refuse gives the error type `Infallible`,
        // discharged by matching the empty enum.
        self.try_map(|c| Ok::<U, Infallible>(f(c)))
            .unwrap_or_else(|never| match never {})
    }

    /// The same affine map read at another scalar where the read may
    /// REFUSE: all twelve components through `f` — the linear part
    /// through [`Mat3::try_map`], then the translation through
    /// [`Vec3::try_map`], each kept in its place — and the FIRST
    /// refusal returned, with no component after it consulted.
    /// Structural like [`Self::map`]: no arithmetic, so exact whenever
    /// `f` is.
    ///
    /// This is the direction a scalar-crossing read needs, where
    /// whether a component has an image is decided by the scalar's
    /// type rather than by its value.
    ///
    /// # Errors
    ///
    /// Whatever `f` refuses with, at the first component it refuses
    /// on.
    pub fn try_map<U: Real, E>(self, f: impl Fn(T) -> Result<U, E>) -> Result<Affine3<U>, E> {
        Ok(Affine3::from_parts(
            self.linear.try_map(&f)?,
            self.translation.try_map(&f)?,
        ))
    }

    /// The pure translation by `v` (identity linear part).
    pub fn translation(v: Vec3<T>) -> Self {
        Self::from_parts(Mat3::identity(), v)
    }

    /// The rotation by `angle` radians (right-hand rule) about the axis
    /// through `point` with direction `axis` — revolve's constructor
    /// (the M0 watchlist item, landing with its first consumer).
    ///
    /// Semantically `T(q) ∘ R ∘ T(−q)` for `q` the displacement of
    /// `point` from the coordinate origin; computed directly as
    /// `linear = R` ([`Mat3::rotation_about`], which **normalizes the
    /// axis internally** — a zero/poisoned axis yields a map that is
    /// poison in every entry, same contract; what poison *looks* like is
    /// the scalar's business, and it differs: all-NaN at `f64`, entire
    /// `[−∞, ∞]` at `Interval`. Either way nothing the map produces is
    /// ever a certified finite value) and `translation = (I − R)·q`,
    /// one application of
    /// the anchor operator ([`Mat3::identity_minus_rotation_about`]) to
    /// the anchor displacement, in exactly that order (D9). Fixed
    /// points: the axis line, up to rounding.
    ///
    /// **The anchor is mentioned once.** The equivalent `q − R·q` is
    /// the same point over the reals, but it subtracts and re-adds the
    /// anchor, and interval arithmetic cannot cancel a repeated
    /// operand: at `T = Interval` that spelling returns the identity
    /// map (`angle = 0`) carrying `2·width(point)` of translation,
    /// which `transform_point` then adds to every point the map
    /// touches. Here the factors that vanish with the angle multiply
    /// the anchor instead: `≈ θ·width(q)` near zero, and at `angle = 0`
    /// exactly zero at `f64`. At `Interval` "exactly zero" is not
    /// available — the backend's `sin` at the exact point `0` encloses
    /// `[−2e-323, 2e-323]`, so the operator carries subnormal dust that
    /// no spelling here can remove. That dust still *multiplies* the
    /// anchor, so the residue is proportional to the anchor's own scale
    /// (`|q| + width(q)`, ~2.6e-322 for a metre-scale anchor) rather
    /// than to `width(q)` alone — small, but not independent of the
    /// operand, and not a constant.
    pub fn rotation_about_axis(point: Point3<T>, axis: Vec3<T>, angle: T) -> Self {
        let q = point - Point3::origin();
        Self::from_parts(
            Mat3::rotation_about(axis, angle),
            Mat3::identity_minus_rotation_about(axis, angle) * q,
        )
    }

    /// Applies the map to a point: `linear·p + translation`, where `p`'s
    /// coordinates are read as the displacement from the coordinate
    /// origin (the chart identification), the linear part is applied
    /// (fixed matrix-vector order, see [`Mat3`]'s `Mul`), and the
    /// translation is added componentwise — in exactly that order (D9).
    pub fn transform_point(self, p: Point3<T>) -> Point3<T> {
        let q = self.linear * Vec3::new(p.x, p.y, p.z);
        Point3::new(
            q.x + self.translation.x,
            q.y + self.translation.y,
            q.z + self.translation.z,
        )
    }

    /// Applies the map to a tangent vector: the linear part only — the
    /// pushforward is the differential, and a translation's differential
    /// is zero, so displacements/directions/normals never feel it.
    pub fn transform_vec(self, v: Vec3<T>) -> Vec3<T> {
        self.linear * v
    }

    /// The inverse map `(L, t)⁻¹ = (L⁻¹, −(L⁻¹·t))`, with `L⁻¹` via the
    /// adjugate ([`Mat3::inverse`]) and the translation computed in
    /// exactly that order (apply `L⁻¹`, then negate).
    ///
    /// **Total.** A singular linear part poisons every entry through the
    /// zero-determinant division (see [`Mat3::inverse`]); the poison then
    /// reaches the translation through `L⁻¹·t`. Invertibility is a
    /// predicate-layer decision, not this method's.
    pub fn inverse(self) -> Self {
        let li = self.linear.inverse();
        Self::from_parts(li, -(li * self.translation))
    }
}

/// Composition `a * b` — **apply `b` first, then `a`**, matching the
/// matrix convention: `(a * b).transform_point(p) =
/// a.transform_point(b.transform_point(p))` in exact arithmetic (in
/// floating point the two sides differ by reassociation rounding).
///
/// Concretely: `linear = a.linear · b.linear`, `translation = a.linear ·
/// b.translation + a.translation` (fixed order, D9).
///
/// Worked example: let `r` = rotation by +π/2 about the z axis through
/// the origin (`from_parts(Mat3::rotation_about(unit_z, π/2),
/// Vec3::zero())`) and `t` = `translation((1, 0, 0))`. Then `(t * r)`
/// maps the point (1, 0, 0) to t(r((1, 0, 0))) = t((0, 1, 0)) =
/// **(1, 1, 0)** — rotate first, then shift — while `(r * t)` maps it to
/// r(t((1, 0, 0))) = r((2, 0, 0)) = **(0, 2, 0)**.
impl<T: Real> Mul for Affine3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::from_parts(
            self.linear * rhs.linear,
            self.linear * rhs.translation + self.translation,
        )
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::Dual64;
    use core::cell::{Cell, RefCell};
    use proptest::prelude::*;

    /// Coordinate strategy — see `vec.rs::tests::coord` for the range
    /// rationale (session-box-like magnitudes, no −0.0).
    fn coord() -> impl Strategy<Value = f64> {
        (1.0e-3..1.0e3f64, any::<bool>()).prop_map(|(m, neg)| if neg { -m } else { m })
    }

    fn vec3() -> impl Strategy<Value = Vec3<f64>> {
        (coord(), coord(), coord()).prop_map(|(x, y, z)| Vec3::new(x, y, z))
    }

    fn point3() -> impl Strategy<Value = Point3<f64>> {
        (coord(), coord(), coord()).prop_map(|(x, y, z)| Point3::new(x, y, z))
    }

    /// Rigid motions (rotation + translation): condition number 1, so
    /// inverse/composition error analysis needs no condition factor —
    /// the justification for restricting the inverse round-trip test to
    /// this family rather than deriving a condition-aware bound for
    /// arbitrary (possibly near-singular) generated matrices.
    fn rigid3() -> impl Strategy<Value = Affine3<f64>> {
        (vec3(), -10.0..10.0f64, vec3())
            .prop_map(|(axis, theta, t)| Affine3::from_parts(Mat3::rotation_about(axis, theta), t))
    }

    #[test]
    fn identity_fixes_points_and_vectors() {
        // Identity linear part applies bit-exactly on −0.0-free inputs
        // (see mat.rs's identity test); adding the zero translation is
        // likewise bit-exact there.
        let p = Point3::new(1.5, -2.25, 3.0e3);
        let q = Affine3::identity().transform_point(p);
        assert_eq!(q.x.to_bits(), p.x.to_bits());
        assert_eq!(q.y.to_bits(), p.y.to_bits());
        assert_eq!(q.z.to_bits(), p.z.to_bits());
    }

    #[test]
    fn transform_vec_ignores_translation() {
        // The pushforward of a pure translation is the identity on
        // tangent vectors — bit-exact, since the linear part is I.
        let t = Affine3::translation(Vec3::new(5.0, -7.0, 11.0));
        let v = Vec3::new(1.5, -2.25, 3.0e3);
        let w = t.transform_vec(v);
        assert_eq!(w.x.to_bits(), v.x.to_bits());
        assert_eq!(w.y.to_bits(), v.y.to_bits());
        assert_eq!(w.z.to_bits(), v.z.to_bits());
        // ... while the same map moves every point.
        let p = Point3::origin();
        let q = t.transform_point(p);
        assert_eq!((q.x, q.y, q.z), (5.0, -7.0, 11.0));
    }

    #[test]
    fn compose_order_worked_example() {
        // The exact example from the `Mul` doc comment, with exactly
        // representable inputs; sin/cos of π/2 are not exact (fl(π/2) is
        // not π/2), hence the small tolerances.
        let r = Affine3::from_parts(
            Mat3::rotation_about(Vec3::unit_z(), core::f64::consts::FRAC_PI_2),
            Vec3::zero(),
        );
        let t = Affine3::translation(Vec3::new(1.0, 0.0, 0.0));
        let p = Point3::new(1.0, 0.0, 0.0);
        let tr = (t * r).transform_point(p);
        assert!((tr.x - 1.0).abs() <= 1e-15 && (tr.y - 1.0).abs() <= 1e-15);
        assert!(tr.z.abs() <= 1e-15);
        let rt = (r * t).transform_point(p);
        assert!((rt.x - 0.0).abs() <= 1e-15 && (rt.y - 2.0).abs() <= 1e-15);
        assert!(rt.z.abs() <= 1e-15);
    }

    #[test]
    fn inverse_of_singular_map_is_poison() {
        // Zero linear part: every inverse entry is NaN (see mat.rs), and
        // the poison reaches the translation through L⁻¹·t.
        let a = Affine3::from_parts(
            Mat3::from_cols(Vec3::<f64>::zero(), Vec3::zero(), Vec3::zero()),
            Vec3::new(1.0, 2.0, 3.0),
        );
        let inv = a.inverse();
        for c in [inv.linear.c0, inv.linear.c1, inv.linear.c2] {
            assert!(c.x.is_nan() && c.y.is_nan() && c.z.is_nan());
        }
        assert!(
            inv.translation.x.is_nan() && inv.translation.y.is_nan() && inv.translation.z.is_nan()
        );
    }

    proptest! {
        /// A·A⁻¹ ≈ id on points, for rigid motions (see `rigid3` on why
        /// the family is restricted — condition number 1). Error budget:
        /// L⁻¹ entries err ≤ ~1e-13 (adjugate arithmetic of near-unit
        /// entries plus the det ≈ 1 reciprocal, see mat.rs budgets);
        /// applying the pair to coordinates and translations ≤ 1e3
        /// scales that to ~1e-10, plus matrix-vector and translation
        /// roundings of order EPSILON·1e3 ≈ 2e-13. Asserted at 1e-8 —
        /// two orders of headroom over the ~1e-10 estimate for the
        /// unmodeled constants.
        #[test]
        fn rigid_inverse_round_trips_points(a in rigid3(), p in point3()) {
            let inv = a.inverse();
            let round = (a * inv).transform_point(p);
            prop_assert!((round.x - p.x).abs() <= 1e-8);
            prop_assert!((round.y - p.y).abs() <= 1e-8);
            prop_assert!((round.z - p.z).abs() <= 1e-8);
            // And through the other application order.
            let back = a.inverse().transform_point(a.transform_point(p));
            prop_assert!((back.x - p.x).abs() <= 1e-8);
            prop_assert!((back.y - p.y).abs() <= 1e-8);
            prop_assert!((back.z - p.z).abs() <= 1e-8);
        }

        /// The composition law (a·b)(p) = a(b(p)) — bounded, not
        /// bit-exact: composing first regroups the double sum
        /// Σⱼ Aᵢⱼ (Σₖ Bⱼₖ pₖ + tⱼ) (same terms, different association),
        /// so the two sides differ by reassociation rounding. All values
        /// stay ≤ ~6e3 for rigid inputs with coordinates ≤ 1e3, so ~20
        /// roundings of ≤ EPSILON·6e3 ≈ 1.3e-12 each bound the gap by
        /// ~3e-11; asserted at 1e-9 for slack.
        #[test]
        fn compose_matches_sequential_application(
            a in rigid3(),
            b in rigid3(),
            p in point3(),
        ) {
            let composed = (a * b).transform_point(p);
            let sequential = a.transform_point(b.transform_point(p));
            prop_assert!((composed.x - sequential.x).abs() <= 1e-9);
            prop_assert!((composed.y - sequential.y).abs() <= 1e-9);
            prop_assert!((composed.z - sequential.z).abs() <= 1e-9);
        }

        /// Pushforward consistency: transform_vec agrees with the
        /// point-difference definition of the differential, df(v) =
        /// f(p + v) − f(p), exactly in real arithmetic for affine f.
        /// In floating point the right-hand side cancels the (≤ 1e3)
        /// translation between two ≤ ~6e3 values: budget ~10 roundings
        /// of ≤ EPSILON·6e3 ≈ 1.3e-12; asserted at 1e-9.
        #[test]
        fn pushforward_is_point_difference(a in rigid3(), p in point3(), v in vec3()) {
            let direct = a.transform_vec(v);
            let differenced = a.transform_point(p + v) - a.transform_point(p);
            prop_assert!((direct.x - differenced.x).abs() <= 1e-9);
            prop_assert!((direct.y - differenced.y).abs() <= 1e-9);
            prop_assert!((direct.z - differenced.z).abs() <= 1e-9);
        }

        /// rotation_about_axis fixes every point of its axis line (up to
        /// rounding: coordinates ≤ 1e3 through ~10 roundings each side
        /// ⇒ ~1e-12 budget, asserted at 1e-9) and agrees with the
        /// composed conjugation T(q)·R·T(−q) it is documented to equal
        /// (the two differ by reassociation rounding only).
        #[test]
        fn rotation_about_axis_fixes_axis_and_matches_conjugation(
            p in point3(),
            axis in vec3(),
            theta in -10.0..10.0f64,
            s in -3.0..3.0f64,
            q in point3(),
        ) {
            let rot = Affine3::rotation_about_axis(p, axis, theta);
            // Every point of the axis line is fixed.
            let on_axis = p + axis * s;
            let moved = rot.transform_point(on_axis);
            prop_assert!((moved.x - on_axis.x).abs() <= 1e-9);
            prop_assert!((moved.y - on_axis.y).abs() <= 1e-9);
            prop_assert!((moved.z - on_axis.z).abs() <= 1e-9);
            // Agreement with the explicit conjugation on arbitrary points.
            let disp = p - Point3::origin();
            let conjugated = Affine3::translation(disp)
                * Affine3::from_parts(Mat3::rotation_about(axis, theta), Vec3::zero())
                * Affine3::translation(-disp);
            let ours = rot.transform_point(q);
            let theirs = conjugated.transform_point(q);
            prop_assert!((ours.x - theirs.x).abs() <= 1e-9);
            prop_assert!((ours.y - theirs.y).abs() <= 1e-9);
            prop_assert!((ours.z - theirs.z).abs() <= 1e-9);
            // Distance to the axis point is preserved (rigidity).
            prop_assert!((ours.distance(p) - q.distance(p)).abs() <= 1e-9);
        }
    }

    /// rotation_about_axis worked example: quarter turn about the
    /// vertical line through (1, 0, 0) maps the origin to (1, −1, 0)
    /// (right-hand rule about +z), and poison flows from a zero axis.
    #[test]
    fn rotation_about_axis_worked_example_and_poison() {
        let rot = Affine3::rotation_about_axis(
            Point3::new(1.0f64, 0.0, 0.0),
            Vec3::unit_z(),
            core::f64::consts::FRAC_PI_2,
        );
        let image = rot.transform_point(Point3::origin());
        assert!((image.x - 1.0).abs() <= 1e-15);
        assert!((image.y - -1.0).abs() <= 1e-15);
        assert!(image.z.abs() <= 1e-15);
        // Zero axis: all-NaN map (Mat3::rotation_about's documented
        // poison), translation poisoned through the anchor operator.
        let bad = Affine3::rotation_about_axis(Point3::new(1.0f64, 2.0, 3.0), Vec3::zero(), 1.0);
        assert!(bad.translation.x.is_nan());
    }

    /// The anchored rotation at `angle = 0` carries **no width from its
    /// anchor** — the enclosure form of "it is the identity".
    ///
    /// The translation is the whole story. `R` is `I` at angle zero, so
    /// over the reals `q − R·q` is zero; but that spelling mentions the
    /// anchor twice and interval arithmetic cannot cancel a repeated
    /// operand — `x − x = [lo − hi, hi − lo]` — so it leaves the
    /// identity map carrying **2·width(anchor)** per component, which
    /// `transform_point` then adds to every point it touches.
    /// `(I − R)·q` pays the anchor once, against an operator every entry
    /// of which has `sin(θ/2)` as a syntactic factor
    /// ([`Mat3::identity_minus_rotation_about`]).
    ///
    /// **What is asserted, and why it is not literal zero.** At `f64`
    /// the translation is bitwise `0.0`. At `Interval` it is not: the
    /// backend's `sin` at the exact point `0` encloses `[−2e-323,
    /// 2e-323]` rather than `[0, 0]`, so the operator carries subnormal
    /// dust, and no spelling on this side can remove it. What the row
    /// pins instead is the property that actually failed — the residue
    /// **stops tracking the anchor's WIDTH**: the anchor enclosure is
    /// widened by six orders between the two measurements and the
    /// translation width does not move. Under `q − R·q` it moves by
    /// exactly those six orders.
    ///
    /// **"Does not move" is a statement about this range, not a law.**
    /// The dust multiplies the anchor, so the residue is proportional
    /// to the anchor's overall scale — `|q| + width(q)` — and the two
    /// rows below only look flat because `|q| ≈ 3` dominates a
    /// half-width of `1e-9` or `1e-3`. Measured across the whole range,
    /// on this fixture's `(1, 2, −3)` anchor: 2.6e-322 flat from
    /// half-width 0 through 1e-3, 4e-322 at half-width 1, and
    /// **1.68e-316 at half-width 1e6** — five orders over the `1e-320`
    /// bound this row asserts. The bound is therefore stated with its
    /// domain: anchors whose width does not dominate their magnitude,
    /// which is every anchor inside the session box (D4 ¶4). What the
    /// bound is NOT is a constant floor independent of the operand, and
    /// the widened row below tests proportionality-to-width only.
    ///
    /// The axis is oblique **and itself an enclosure**: the claim is not
    /// "the axis happens to normalize exactly", it is that the vanishing
    /// factor multiplies, which holds whatever the axis is. The row is
    /// ε-free — it measures enclosure width and asserts no tolerance, so
    /// it reads identically at every tolerance row.
    #[test]
    fn zero_angle_anchored_rotation_carries_no_anchor_width() {
        use crate::interval::Interval;
        use crate::real::Bounds;

        let width = |e: Interval| e.hi() - e.lo();
        // Subdivision-scale and box-scale anchor enclosures: six orders
        // apart, so anything proportional to the anchor's width is
        // visible as a six-order difference between the two rows.
        let mut residue = [0.0f64; 2];
        for (row, h) in [1.0e-9f64, 1.0e-3].into_iter().enumerate() {
            let wide = |c: f64| Interval::from_bounds(c - h, c + h);
            let anchor = Point3::new(wide(1.0), wide(2.0), wide(-3.0));
            assert!(
                width(anchor.x) >= h,
                "FIXTURE: the anchor must carry width, else the row is vacuous"
            );
            // Oblique, and wide in its own right.
            let axis = Vec3::new(wide(1.0), wide(-2.0), wide(2.0));

            let rot = Affine3::rotation_about_axis(anchor, axis, Interval::zero());
            let w = width(rot.translation.x)
                .max(width(rot.translation.y))
                .max(width(rot.translation.z));
            residue[row] = w;
            // Domain-scoped: the operator's dust MULTIPLIES the anchor,
            // so this bound holds for anchors whose width does not
            // dominate their magnitude (|q| ≈ 3 here against half-widths
            // 1e-9 and 1e-3). It is exceeded — 1.68e-316 — once the
            // half-width reaches 1e6; see this row's doc comment.
            assert!(
                w <= 1.0e-320,
                "zero-angle translation is {w:e} wide on an anchor of width {:e} \
                 — the anchor is being subtracted and re-added rather than \
                 multiplied by a vanishing operator",
                width(anchor.x),
            );
        }
        assert!(
            residue[1] <= 16.0 * residue[0].max(f64::MIN_POSITIVE),
            "the zero-angle translation width tracks the anchor width \
             ({:e} at half-width 1e-9 against {:e} at 1e-3)",
            residue[0],
            residue[1],
        );

        // The guard with teeth: the retired spelling, measured on this
        // very fixture, must still be two anchor widths. If it ever
        // stops being, the bound above has stopped discriminating and
        // this row reds instead of going quiet.
        //
        // Bounded on BOTH sides, because the lower bound alone is
        // satisfied by garbage: a poisoned or degenerate `R` makes the
        // retired width `+inf`, and `inf >= 1.9·w` passes vacuously
        // while measuring nothing. The upper bound says the fixture
        // still produces a finite `2·width(anchor)` and not an
        // entire-interval collapse.
        let h = 1.0e-9;
        let wide = |c: f64| Interval::from_bounds(c - h, c + h);
        let anchor = Point3::new(wide(1.0), wide(2.0), wide(-3.0));
        let q = anchor - Point3::origin();
        let linear = Mat3::rotation_about(
            Vec3::new(wide(1.0), wide(-2.0), wide(2.0)),
            Interval::zero(),
        );
        let retired = q - linear * q;
        let paid = width(retired.x);
        assert!(
            paid >= 1.9 * width(anchor.x) && paid <= 10.0 * width(anchor.x),
            "the `q − R·q` spelling is supposed to pay 2·width(anchor) = {:e} here; \
             it paid {paid:e} — under the lower bound the guard is stale (the \
             arithmetic learned to cancel), over the upper one it is vacuous (a \
             poisoned or entire R measures nothing)",
            2.0 * width(anchor.x),
        );

        // The same at `f64`, where the operator is exactly zero and the
        // anchored identity is therefore bitwise the identity.
        let anchor = Point3::new(1.0f64, 2.0, -3.0);
        let rot = Affine3::rotation_about_axis(anchor, Vec3::new(1.0, -2.0, 2.0), 0.0);
        // Zero, though two components carry the negative sign (the
        // off-diagonal entries are negations, and `−(0 − 0)` is `−0.0`):
        // the same number, and it adds exactly, which is what the
        // bitwise round trip below actually needs.
        assert_eq!(rot.translation.x, 0.0);
        assert_eq!(rot.translation.y, 0.0);
        assert_eq!(rot.translation.z, 0.0);
        let image = rot.transform_point(anchor);
        assert_eq!(image.x.to_bits(), anchor.x.to_bits());
        assert_eq!(image.y.to_bits(), anchor.y.to_bits());
        assert_eq!(image.z.to_bits(), anchor.z.to_bits());
    }

    /// The same claim off the exact-zero point: at small angles the
    /// anchor-attributable width **scales with the angle** rather than
    /// sitting on a constant floor.
    ///
    /// `(I − R)·q` has entries of size `|sin θ|` and `1 − cos θ`, so
    /// the width the anchor contributes is `≈ θ·width(q)` — it goes to
    /// zero with the angle, continuously into the identity case above.
    /// The subtract-then-re-add spelling pays `2·width(q)` at *every*
    /// angle, so the upper bounds below are three to seven orders under
    /// it. Both sides are asserted: the lower bound is what keeps the
    /// row from passing vacuously if the width ever collapsed to a
    /// constant (a floor would break the proportionality, not the
    /// ceiling).
    ///
    /// Axis `+z` and anchor `(1, 2, −3)` make the arithmetic readable:
    /// `translation.x = t·q.x + s·q.y`, so the width is
    /// `(|t| + |s|)·width(q) ≈ θ·width(q)` plus the interval `cos`'s
    /// own ulp-scale enclosure at the point angle — 1.1e-16 against
    /// `|q|`, five orders below the smallest bound here, which is why
    /// the row does not need a floor term.
    #[test]
    fn small_angle_anchored_rotation_width_scales_with_the_angle() {
        use crate::interval::Interval;
        use crate::real::Bounds;

        let width = |e: Interval| e.hi() - e.lo();
        let h = 1.0e-6;
        let wide = |c: f64| Interval::from_bounds(c - h, c + h);
        let anchor = Point3::new(wide(1.0), wide(2.0), wide(-3.0));
        let qw = width(anchor.x);
        let axis = Vec3::new(Interval::zero(), Interval::zero(), Interval::one());

        let mut measured = [0.0f64; 3];
        for (i, theta) in [1.0e-2f64, 1.0e-4, 1.0e-6].into_iter().enumerate() {
            let rot = Affine3::rotation_about_axis(anchor, axis, Interval::from_f64(theta));
            let w = width(rot.translation.x)
                .max(width(rot.translation.y))
                .max(width(rot.translation.z));
            measured[i] = w;
            assert!(
                w <= 2.0 * theta * qw,
                "at angle {theta:e} the translation is {w:e} wide against \
                 anchor width {qw:e} — expected ≈ θ·width(q) = {:e}",
                theta * qw,
            );
            assert!(
                w >= 0.5 * theta * qw,
                "at angle {theta:e} the translation is {w:e} wide, under the \
                 ≈ θ·width(q) = {:e} the anchor must still contribute — the row \
                 is measuring the wrong thing",
                theta * qw,
            );
        }
        // Proportionality across two decades of angle, stated as a
        // ratio so a constant floor cannot satisfy it.
        assert!(
            measured[0] >= 50.0 * measured[1] && measured[1] >= 50.0 * measured[2],
            "widths {:e} / {:e} / {:e} at angles 1e-2 / 1e-4 / 1e-6 are not \
             scaling with the angle",
            measured[0],
            measured[1],
            measured[2],
        );
    }

    /// A deterministic generator for the frame sweep (no dev-dependency,
    /// reproducible to the bit): log-uniform magnitudes in
    /// `[1e-6, 1e6]`, random sign, one coordinate in sixteen a signed
    /// zero.
    struct Lcg(u64);

    impl Lcg {
        fn next(&mut self) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            self.0 >> 11
        }

        #[allow(clippy::cast_precision_loss)]
        fn coord(&mut self) -> f64 {
            let r = self.next();
            if r.is_multiple_of(16) {
                return if r & 16 == 0 { 0.0 } else { -0.0 };
            }
            let m = 10f64.powf(-6.0 + 12.0 * (self.next() as f64) / ((1u64 << 53) as f64));
            if r & 32 == 0 { m } else { -m }
        }

        fn point(&mut self) -> Point3<f64> {
            Point3::new(self.coord(), self.coord(), self.coord())
        }

        fn vec(&mut self) -> Vec3<f64> {
            Vec3::new(self.coord(), self.coord(), self.coord())
        }
    }

    /// The placements the storage row sweeps — THE corpus for what
    /// [`Affine3::from_parts`] keeps of what it was handed: every sign
    /// pattern of
    /// zeros over all nine components; a general non-orthonormal frame
    /// with each single component replaced by `+0.0` and by `−0.0` in
    /// turn; a subnormal, a huge value, `±inf` and NaN in a slot of
    /// each of origin, `u` and `v`; and a 2000-frame generated sweep of
    /// non-unit, non-orthogonal axes.
    pub(super) fn frame_corpus() -> Vec<(Point3<f64>, Vec3<f64>, Vec3<f64>)> {
        let mut corpus = Vec::new();
        for m in 0..512u32 {
            let z = |k: u32| if m & (1 << k) == 0 { 0.0 } else { -0.0 };
            corpus.push((
                Point3::new(z(0), z(1), z(2)),
                Vec3::new(z(3), z(4), z(5)),
                Vec3::new(z(6), z(7), z(8)),
            ));
        }
        let general = [0.3, -1.2, 2.5, -4.0, 0.25, 1.0e-3, 7.0, -0.5, 2.0];
        for k in 0..9 {
            for z in [0.0, -0.0] {
                let mut w = general;
                w[k] = z;
                corpus.push((
                    Point3::new(w[6], w[7], w[8]),
                    Vec3::new(w[0], w[1], w[2]),
                    Vec3::new(w[3], w[4], w[5]),
                ));
            }
        }
        for s in [5e-324, 1.0e308, f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            corpus.push((
                Point3::new(s, 1.0, -0.0),
                Vec3::new(s, 0.5, -2.0),
                Vec3::new(-1.0, s, 3.0),
            ));
        }
        let mut r = Lcg(0x9e37_79b9_7f4a_7c15);
        for _ in 0..2000 {
            corpus.push((r.point(), r.vec(), r.vec()));
        }
        corpus
    }

    /// The twelve components in the order the walk visits them: the
    /// three columns, each `x, y, z`, then the translation. Written
    /// out by hand, so it is an INDEPENDENT statement of where each
    /// component belongs — a transposition inside the walk
    /// disagrees with it, which is the whole reason the walk rows
    /// compare against this and not against another call to the walk.
    fn components<T: Real>(a: Affine3<T>) -> [T; 12] {
        let (l, t) = (a.linear, a.translation);
        [
            l.c0.x, l.c0.y, l.c0.z, l.c1.x, l.c1.y, l.c1.z, l.c2.x, l.c2.y, l.c2.z, t.x, t.y, t.z,
        ]
    }

    fn bits(a: Affine3<f64>) -> [u64; 12] {
        components(a).map(f64::to_bits)
    }

    /// A placement with twelve DISTINCT components and no symmetry —
    /// deliberately not a frame, so a transposition cannot be hidden by
    /// an axis coincidence, and so a refusal can name which component
    /// it came from by its value.
    fn distinct() -> Affine3<f64> {
        Affine3::from_parts(
            Mat3::from_cols(
                Vec3::new(1.0, 2.0, 3.0),
                Vec3::new(4.0, 5.5, -6.0),
                Vec3::new(-7.25, 0.5, 8.0),
            ),
            Vec3::new(10.0, 11.0, 12.0),
        )
    }

    /// Both walks land every component where the hand spelling puts
    /// it, to the bit, over the storage corpus — signed zeros in
    /// every pattern, a subnormal, `1e308`, both infinities and one
    /// NaN, plus a 2000-frame generated sweep. (The corpus carries a
    /// single NaN bit pattern; three DISTINCT payloads are swept by
    /// `map_returns_the_hand_walk_bits_at_all_three_levels` below.)
    ///
    /// `map` and `try_map` share one body, so this row is what pins
    /// that fact to something outside itself: each is measured against
    /// `components`, a third spelling neither of them calls. A `c1`/`c2`
    /// transposition in the shared walk reds both assertions, and if
    /// `map` is ever given a body of its own again the two stay
    /// independent receipts rather than one tautology.
    #[test]
    fn both_walks_place_the_twelve_components_where_the_hand_spelling_does() {
        for (o, u, v) in frame_corpus() {
            let a = Affine3::from_parts(Mat3::from_cols(u, v, u.cross(v)), o - Point3::origin());
            let want = bits(a);
            assert_eq!(bits(a.map(|x: f64| x)), want, "map at {o:?} {u:?} {v:?}");
            let walk = a
                .try_map(|x: f64| Ok::<f64, Infallible>(x))
                .unwrap_or_else(|never| match never {});
            assert_eq!(bits(walk), want, "try_map at {o:?} {u:?} {v:?}");
        }
    }

    /// The same claim across scalars, over the SAME corpus: lifting to
    /// `Dual64` carries each component's bits into the value channel
    /// and a zero derivative beside it, in both directions and in the
    /// same places. Signed zeros, the subnormal, the infinities and
    /// the NaN are lifted cross-scalar here and nowhere else.
    ///
    /// The channels are read off `components` and NOT through `map`. A
    /// readout that is itself the walk would apply the walk's own
    /// transposition a second time and cancel it — measured: with
    /// `Mat3::try_map` mutated to swap `c0` and `c1`, this row passed
    /// while it read through `map` and reds now.
    #[test]
    fn both_walks_lift_to_another_scalar_in_the_same_places() {
        let channels = |d: Affine3<Dual64>| {
            let c = components(d);
            (c.map(|x| x.value.to_bits()), c.map(|x| x.deriv.to_bits()))
        };
        for (o, u, v) in frame_corpus() {
            let a = Affine3::from_parts(Mat3::from_cols(u, v, u.cross(v)), o - Point3::origin());
            let want = bits(a);
            let door = channels(a.map(Dual64::from_f64));
            let walk = channels(
                a.try_map(|x: f64| Ok::<Dual64, Infallible>(Dual64::from_f64(x)))
                    .unwrap_or_else(|never| match never {}),
            );
            assert_eq!(door.0, want, "map at {o:?} {u:?} {v:?}");
            assert_eq!(walk.0, want, "try_map at {o:?} {u:?} {v:?}");
            assert_eq!(door.1, [0.0_f64.to_bits(); 12]);
            assert_eq!(walk, door);
        }
    }

    /// The same placement at `Dual64`, lifted BY HAND — component by
    /// component, calling no walk. A row that walks back DOWN needs a
    /// source the walk did not build: lift through `map` and a
    /// transposed walk would transpose twice and cancel its own error.
    fn lifted_by_hand(a: Affine3<f64>) -> Affine3<Dual64> {
        let v = |w: Vec3<f64>| {
            Vec3::new(
                Dual64::from_f64(w.x),
                Dual64::from_f64(w.y),
                Dual64::from_f64(w.z),
            )
        };
        Affine3::from_parts(
            Mat3::from_cols(v(a.linear.c0), v(a.linear.c1), v(a.linear.c2)),
            v(a.translation),
        )
    }

    /// The walk runs DOWNWARD too, and a refusal-shaped `f` that does
    /// not refuse is still a successful walk: from a hand-built
    /// `Dual64` placement back to `f64` through an `f` that rejects any
    /// component carrying a derivative. A constant lift carries none,
    /// so nothing refuses and all twelve components arrive at their
    /// source bits, in their places.
    ///
    /// The other cross-scalar rows go `f64` → `Dual64`; this is the
    /// only one that goes the other way, and "scalar-agnostic" is a
    /// claim about both directions.
    #[test]
    fn try_map_walks_back_down_to_f64_when_nothing_refuses() {
        for (o, u, v) in frame_corpus() {
            let a = Affine3::from_parts(Mat3::from_cols(u, v, u.cross(v)), o - Point3::origin());
            let back: Affine3<f64> = lifted_by_hand(a)
                .try_map(|x: Dual64| {
                    if x.deriv.to_bits() == 0.0_f64.to_bits() {
                        Ok(x.value)
                    } else {
                        Err(())
                    }
                })
                .expect("a constant lift carries no derivative, so nothing refuses");
            assert_eq!(bits(back), bits(a), "at {o:?} {u:?} {v:?}");
        }
    }

    /// The fallible walk returns the FIRST refusal and consults
    /// nothing after it.
    ///
    /// For each component in turn, `f` accepts everything before it and
    /// refuses from there on — so every later component would refuse
    /// too, and a walk that returned the last refusal, or ran `f` over
    /// all twelve and picked, would carry out component 11's value and
    /// twelve calls. What comes out is the k-th component's own value
    /// (the components are distinct, so the value names which one) and
    /// exactly `k + 1` calls.
    #[test]
    fn try_map_returns_the_first_refusal_and_consults_nothing_after_it() {
        let a = distinct();
        for k in 0..12usize {
            let calls = Cell::new(0usize);
            let got = a.try_map(|x: f64| {
                let i = calls.get();
                calls.set(i + 1);
                if i < k { Ok(x) } else { Err(x) }
            });
            match got {
                Ok(_) => panic!("component {k} refused, so the walk must not answer Ok"),
                Err(e) => assert_eq!(
                    e.to_bits(),
                    components(a)[k].to_bits(),
                    "the refusal carried out is component {k}'s"
                ),
            }
            assert_eq!(calls.get(), k + 1, "f runs once per component up to {k}");
        }
    }

    /// A walk that never refuses answers `Ok` and runs `f` exactly
    /// twelve times — the count that makes the short-circuit row
    /// above a measurement rather than an accident of the closure.
    #[test]
    fn try_map_without_a_refusal_visits_all_twelve_components_once() {
        let a = distinct();
        let calls = Cell::new(0usize);
        let out = a.try_map(|x: f64| {
            calls.set(calls.get() + 1);
            Ok::<f64, ()>(x)
        });
        assert_eq!(bits(out.unwrap()), bits(a));
        assert_eq!(calls.get(), 12);
    }

    #[test]
    fn from_parts_stores_the_columns_and_the_origin_bitwise() {
        // The map stores what it was handed: the three columns are the
        // caller's values, and the translation `origin − Point3::origin()`
        // is `x − (+0.0)`, which is `x` to the bit — signed zeros,
        // infinities and NaN payloads included. The corpus is
        // deliberately non-unit and non-orthogonal, because this claim
        // is about storage and not about frames.
        for (o, u, v) in frame_corpus() {
            let n = u.cross(v);
            let a = Affine3::from_parts(Mat3::from_cols(u, v, n), o - Point3::origin());
            let want =
                [u.x, u.y, u.z, v.x, v.y, v.z, n.x, n.y, n.z, o.x, o.y, o.z].map(f64::to_bits);
            assert_eq!(bits(a), want, "at {o:?} {u:?} {v:?}");
        }
    }

    // ---- The respelling oracle ----------------------------------------
    //
    // `map` IS `try_map` under an `f` that cannot refuse, so what `map`
    // returns is measured against a walk written out by hand at each of
    // the three levels, calling nothing in this crate. Adopted from the
    // style review's probe for this unit, which is where the corpus and
    // the impure-`f` trace below were designed.

    /// The walk at each level, transcribed by hand. The oracle: `map`
    /// must return what these return, in every bit and at every level.
    /// Nothing here may be rewritten to call the doors it measures.
    fn hand_vec3_map<T: Real, U: Real>(v: Vec3<T>, f: impl Fn(T) -> U) -> Vec3<U> {
        Vec3::new(f(v.x), f(v.y), f(v.z))
    }

    fn hand_mat3_map<T: Real, U: Real>(m: Mat3<T>, f: impl Fn(T) -> U) -> Mat3<U> {
        Mat3::from_cols(
            hand_vec3_map(m.c0, &f),
            hand_vec3_map(m.c1, &f),
            hand_vec3_map(m.c2, &f),
        )
    }

    fn hand_affine3_map<T: Real, U: Real>(a: Affine3<T>, f: impl Fn(T) -> U) -> Affine3<U> {
        Affine3::from_parts(
            hand_mat3_map(a.linear, &f),
            hand_vec3_map(a.translation, &f),
        )
    }

    /// Twelve awkward bit patterns, including THREE DISTINCT NaN
    /// payloads — quiet, quiet with a payload, and a signalling-shaped
    /// one with the sign bit set. A structural walk must carry a NaN's
    /// payload and sign, not merely "a NaN", and one pattern cannot
    /// show that.
    fn nasty() -> [f64; 12] {
        [
            0.0,
            -0.0,
            5e-324,
            -5e-324,
            // The largest subnormal, spelled by its bits: the boundary
            // the smallest normal sits one ulp above, and a decimal
            // literal for it is longer than it is legible.
            f64::from_bits(0x000f_ffff_ffff_ffff),
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::from_bits(0x7ff8_0000_0000_0000),
            f64::from_bits(0x7ff8_0000_dead_beef),
            f64::from_bits(0xfff4_0000_0000_0001),
            1.0,
            -1.0e308,
        ]
    }

    /// The twelve patterns slid through all twelve slots, so every
    /// pattern occupies every component in turn: a walk that mishandles
    /// one pattern in one place has nowhere to hide.
    fn shifted_corpus() -> Vec<Affine3<f64>> {
        let n = nasty();
        (0..12)
            .map(|shift| {
                let c: [f64; 12] = core::array::from_fn(|i| n[(i + shift) % 12]);
                Affine3::from_parts(
                    Mat3::from_cols(
                        Vec3::new(c[0], c[1], c[2]),
                        Vec3::new(c[3], c[4], c[5]),
                        Vec3::new(c[6], c[7], c[8]),
                    ),
                    Vec3::new(c[9], c[10], c[11]),
                )
            })
            .collect()
    }

    /// `map` returns the hand walk's bits at all three levels, over the
    /// shifted corpus, under an `f` that cannot round (the identity)
    /// and one that touches every pattern's sign bit including the
    /// zeros' and the NaNs' (a bitwise negation).
    #[test]
    fn map_returns_the_hand_walk_bits_at_all_three_levels() {
        let flip = |x: f64| f64::from_bits(x.to_bits() ^ (1u64 << 63));
        for a in shifted_corpus() {
            assert_eq!(bits(a.map(|x| x)), bits(hand_affine3_map(a, |x| x)));
            assert_eq!(bits(a.map(flip)), bits(hand_affine3_map(a, flip)));
            // The two levels beneath, reassembled so the same readout
            // serves: a `Mat3`/`Vec3` disagreement cannot be absorbed by
            // the level above.
            let door = Affine3::from_parts(a.linear.map(flip), a.translation.map(flip));
            let hand = Affine3::from_parts(
                hand_mat3_map(a.linear, flip),
                hand_vec3_map(a.translation, flip),
            );
            assert_eq!(bits(door), bits(hand));
        }
    }

    /// `map` calls `f` in the hand walk's order, exactly twelve times.
    ///
    /// `f` here is impure: its answer depends on how many times it has
    /// been called, so a change in ORDER or COUNT changes the twelve
    /// results as well as the trace. That is what makes this a
    /// measurement of the respelling rather than of the arithmetic —
    /// the `Infallible` wrapper adds a closure layer, and this row is
    /// what says the layer changed neither.
    /// One walk over a placement under a caller-supplied `f`: the shape
    /// the respelling trace measures the door and the hand spelling
    /// through, so both go through one signature.
    type WalkUnder<'a> = &'a dyn Fn(Affine3<f64>, &dyn Fn(f64) -> f64) -> Affine3<f64>;

    #[test]
    fn map_calls_an_impure_f_in_the_same_order_and_as_often() {
        let a = distinct();
        let trace = |via: WalkUnder<'_>| {
            let seen = RefCell::new(Vec::new());
            let out = via(a, &|x| {
                let mut s = seen.borrow_mut();
                s.push(x.to_bits());
                #[allow(clippy::cast_precision_loss)]
                let n = s.len() as f64;
                x * 1000.0 + n
            });
            (bits(out), seen.into_inner())
        };
        let door = trace(&|a, f| a.map(f));
        let hand = trace(&|a, f| hand_affine3_map(a, f));
        assert_eq!(door.1, hand.1, "the order f sees the components in");
        assert_eq!(door.1.len(), 12, "twelve calls, no more");
        assert_eq!(door.0, hand.0, "the twelve results land in the same places");
    }
}
