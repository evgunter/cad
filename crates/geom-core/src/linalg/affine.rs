//! Affine maps of the 3-D affine space.
//!
//! See the [module docs](super) for the affine/linear split: an affine map
//! is a linear part plus a translation. The linear part is the map's
//! *differential* — constant, because affine maps are exactly the maps
//! whose differential is constant — and is therefore what pushes tangent
//! vectors forward ([`Affine3::transform_vec`]); the translation is felt
//! only by points ([`Affine3::transform_point`]).

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
    /// Builds an affine map from its linear part and translation.
    pub fn from_parts(linear: Mat3<T>, translation: Vec3<T>) -> Self {
        Self {
            linear,
            translation,
        }
    }

    /// The identity map.
    pub fn identity() -> Self {
        Self::from_parts(Mat3::identity(), Vec3::zero())
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
    /// axis internally** — a zero/poisoned axis yields an all-NaN map,
    /// same contract) and `translation = (I − R)·q`, one application of
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
    /// the anchor instead — exactly zero at `angle = 0`, `≈ θ·width`
    /// near it.
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
    /// is **independent of the anchor**: the anchor enclosure is
    /// widened by six orders between the two measurements and the
    /// translation width does not move. Under `q − R·q` it moves by
    /// exactly those six orders.
    ///
    /// The axis is oblique **and itself an enclosure**: the claim is not
    /// "the axis happens to normalize exactly", it is that the vanishing
    /// factor multiplies, which holds whatever the axis is. The row is
    /// ε-free — it measures enclosure width and asserts no tolerance, so
    /// it reads identically at every tolerance row.
    #[cfg(feature = "interval")]
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
        let h = 1.0e-9;
        let wide = |c: f64| Interval::from_bounds(c - h, c + h);
        let anchor = Point3::new(wide(1.0), wide(2.0), wide(-3.0));
        let q = anchor - Point3::origin();
        let linear =
            Mat3::rotation_about(Vec3::new(wide(1.0), wide(-2.0), wide(2.0)), Interval::zero());
        let retired = q - linear * q;
        assert!(
            width(retired.x) >= 1.9 * width(anchor.x),
            "the `q − R·q` spelling is supposed to pay 2·width(anchor) = {:e} here; \
             it paid {:e} — if this now holds, the guard is stale",
            2.0 * width(anchor.x),
            width(retired.x),
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
    #[cfg(feature = "interval")]
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
}
