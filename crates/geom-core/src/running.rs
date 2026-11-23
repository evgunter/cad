//! **Running error bounds** — a value carried with a first-order bound
//! on its accumulated rounding, for the meters that charge an
//! evaluation chain's own error rather than a fixed count of ulps of a
//! term bound.
//!
//! Each correctly rounded operation adds `u·|result|`, `u` the unit
//! roundoff ([`UNIT_ROUNDOFF`]), and carries its operands' bounds
//! through its partial derivatives; second-order terms (`u²` of the
//! same magnitudes) are dropped. A product of exact zeros, or a
//! subtraction that lands on an exact zero, therefore charges nothing,
//! which is what lets an axis-aligned chain report the zero error it
//! has. Each operation evaluates its value as the plain expression
//! does, so a shadowed value is bit-identical to the plain one.
//!
//! It is a rounding ESTIMATE in the `f64` lane's sense, not an
//! enclosure: at `Interval` the value is already an enclosure of the
//! chain and the bound rides along as a second one.

use core::ops::{Add, Mul, Neg, Sub};

use crate::spline::SpanLocate;
use crate::{Band, Decide, Real, Sign, Vec3};

/// The unit roundoff of `f64`, `u = 2⁻⁵³`: half an ulp of `1`, the
/// relative error bound of one correctly rounded operation.
pub const UNIT_ROUNDOFF: f64 = f64::EPSILON * 0.5;

/// How many half-ulps of a term bound a short evaluation chain is
/// charged ([`rounding_charge`]): a chain from the inputs — a
/// difference, a squared norm, a product, a sum of a few terms — whose
/// every rounding is half an ulp of a quantity the term bound dominates;
/// sixteen is that chain's count with room. A ROUNDING estimate, the `f64` lane's contract, not an
/// enclosure: the `Interval` lane carries its own.
pub const SHORT_CHAIN_ULPS: f64 = 16.0;

/// How many units of roundoff each component of a computed unit
/// direction, or of a point placed at a scale, carries
/// ([`direction_rounding`]): its evaluation and normalization, a few
/// correctly rounded operations each. A ROUNDING estimate, as
/// [`SHORT_CHAIN_ULPS`] is; its guard measures it against exact
/// arithmetic (`the_direction_charge_holds_a_normalized_directions_rounding`).
pub const DIRECTION_UNITS: f64 = 4.0;

/// **The rounding a computed direction or point carries at `scale`**:
/// [`DIRECTION_UNITS`] units of roundoff of it. A unit direction's
/// component is charged it at `scale = 1`; a point placed at a reach is
/// charged it at that reach. The one home of the charge.
#[must_use]
pub fn direction_rounding<T: Real>(scale: T) -> T {
    T::from_f64(DIRECTION_UNITS * UNIT_ROUNDOFF) * scale
}

/// The rounding charged against a term bound `terms`:
/// [`SHORT_CHAIN_ULPS`] half-ulps of it.
#[must_use]
pub fn rounding_charge<T: Real>(terms: T) -> T {
    T::from_f64(SHORT_CHAIN_ULPS * f64::EPSILON * 0.5) * terms
}

/// A value and a first-order bound on its accumulated rounding (module
/// docs).
#[derive(Debug, Clone, Copy)]
pub struct Rounded<T> {
    /// The value, bit-identical to the plain expression's.
    pub value: T,
    /// A first-order bound on `|value − exact|`, `exact` the same
    /// expression in real arithmetic on the same inputs.
    pub error: T,
}

impl<T: Real> Rounded<T> {
    /// An input taken as exact.
    pub fn exact(value: T) -> Self {
        Self {
            value,
            error: T::zero(),
        }
    }

    fn charged(value: T, error: T) -> Self {
        Self {
            value,
            error: error + T::from_f64(UNIT_ROUNDOFF) * value.abs(),
        }
    }

    /// `self / d` for an EXACT divisor `d`.
    #[must_use]
    pub fn div_exact(self, d: T) -> Self {
        Self::charged(self.value / d, self.error / d.abs())
    }

    /// `self / d`: the quotient's share of each operand's bound, over
    /// the room `d` keeps from zero ([`Divisor`]).
    #[must_use]
    pub fn quotient(self, d: Divisor<T>) -> Self {
        let Divisor { d, room } = d;
        let value = self.value / d.value;
        Self {
            value,
            error: (self.error + value.abs() * d.error) / room
                + T::from_f64(UNIT_ROUNDOFF) * value.abs(),
        }
    }

    /// `self²`, as `self.value.powi(2)` (the tight square).
    #[must_use]
    pub fn square(self) -> Self {
        Self::charged(
            self.value.powi(2),
            T::from_f64(2.0) * self.value.abs() * self.error,
        )
    }

    /// `√(a² + b²)`, as `(a.powi(2) + b.powi(2)).sqrt()`. The norm is
    /// 1-Lipschitz in each argument, and its own evaluation (two
    /// squares, a sum of non-negatives, a square root) is off by at most
    /// `2u` of the result — a bound with no division, so it holds at
    /// the origin.
    #[must_use]
    pub fn hypot(self, o: Self) -> Self {
        let value = (self.value.powi(2) + o.value.powi(2)).sqrt();
        Self {
            value,
            error: self.error + o.error + T::from_f64(2.0 * UNIT_ROUNDOFF) * value,
        }
    }

    /// `√self`, as `self.value.sqrt()`. Two bounds hold on the operand's
    /// share, `δ/(2√x)` (first order) and `√δ` (`|√a − √b| ≤ √|a − b|`,
    /// which holds at the origin), and the smaller is taken; the root's
    /// own rounding adds `u` of the result.
    #[must_use]
    pub fn sqrt(self) -> Self {
        let value = self.value.sqrt();
        let share = (self.error / (T::from_f64(2.0) * value)).min(self.error.sqrt());
        Self::charged(value, share)
    }

    /// `when_le` where `decision ≤ 0`, else `when_gt`: [`Real::select_le_zero`]
    /// on the value and the bound alike.
    #[must_use]
    pub fn select_le_zero(decision: T, when_le: Self, when_gt: Self) -> Self {
        Self {
            value: decision.select_le_zero(when_le.value, when_gt.value),
            error: decision.select_le_zero(when_le.error, when_gt.error),
        }
    }

    /// An upper bound on `|exact|`: the magnitude plus the bound.
    #[must_use]
    pub fn magnitude(self) -> T {
        self.value.abs() + self.error
    }

    /// A lower bound on `|exact|`: the magnitude less the bound, at
    /// least zero.
    #[must_use]
    pub fn room(self) -> T {
        (self.value.abs() - self.error).max(T::zero())
    }

    /// The lower end of the enclosure `value ± error`, signed. At `f64`
    /// it is a first-order estimate of a lower bound on the exact value,
    /// as the bound is, and its own subtraction rounds to nearest; at
    /// `Interval` it is an enclosure of one.
    #[must_use]
    pub fn lower_end(self) -> T {
        self.value - self.error
    }

    /// The upper end of the enclosure `value ± error`, signed, read as
    /// [`Rounded::lower_end`] is at each scalar.
    #[must_use]
    pub fn upper_end(self) -> T {
        self.value + self.error
    }

    /// This value with `charge` added to its bound: a further bound on
    /// its distance from what the caller reads it as, beside its own
    /// rounding. A charge is a size, so its magnitude is what is added:
    /// a bound only ever grows here.
    #[must_use]
    pub fn with_charge(self, charge: T) -> Self {
        Self {
            value: self.value,
            error: self.error + charge.abs(),
        }
    }

    /// The value moved toward zero by its bound, and no further: the
    /// [`Rounded::room`] with the value's sign, zero where the bound
    /// holds zero. A sign decided on it is the exact value's.
    #[must_use]
    pub fn toward_zero(self) -> T {
        let room = self.room();
        self.value.select_le_zero(T::zero() - room, room)
    }
}

impl<T: Decide> Rounded<T> {
    /// This value as a divisor, where its [`Rounded::room`] is positive:
    /// the enclosure `value ± error` lies wholly off zero, read as an
    /// enclosure's sign is ([`Band::enclosure_sign`]). `None` where it
    /// does not, since there the exact value may be zero and a quotient
    /// by it bounds nothing.
    ///
    /// A `None` picks the caller's answer, so the read is a logged
    /// decision under the caller's `name` ([`crate::k_stats::decide`]).
    /// An undecided room is recorded in the verdict log and read as
    /// `None`: the caller still picks its answer, and nothing refuses.
    #[must_use]
    pub fn divisor(self, name: &'static str) -> Option<Divisor<T>> {
        let room = self.room();
        let clear =
            crate::k_stats::decide(name, crate::Margin::sign_of(room), Band::enclosure_sign())
                .is_ok_and(|sign| sign == Sign::Positive);
        clear.then_some(Divisor { d: self, room })
    }
}

/// A [`Rounded`] value whose magnitude is past its own bound by `room`,
/// so its exact value is not zero: the one argument
/// [`Rounded::quotient`] divides by. Built only by
/// [`Rounded::divisor`].
#[derive(Debug, Clone, Copy)]
pub struct Divisor<T> {
    d: Rounded<T>,
    room: T,
}

impl<T: Real> Add for Rounded<T> {
    type Output = Self;

    fn add(self, o: Self) -> Self {
        Self::charged(self.value + o.value, self.error + o.error)
    }
}

impl<T: Real> Sub for Rounded<T> {
    type Output = Self;

    fn sub(self, o: Self) -> Self {
        Self::charged(self.value - o.value, self.error + o.error)
    }
}

impl<T: Real> Neg for Rounded<T> {
    type Output = Self;

    /// Exact: the bound is unchanged.
    fn neg(self) -> Self {
        Self {
            value: T::zero() - self.value,
            error: self.error,
        }
    }
}

impl<T: Real> Mul for Rounded<T> {
    type Output = Self;

    fn mul(self, o: Self) -> Self {
        Self::charged(
            self.value * o.value,
            self.value.abs() * o.error + o.value.abs() * self.error,
        )
    }
}

impl Rounded<f64> {
    /// **One blend `self + (y − self)·t`**, as
    /// [`SpanLocate::homogeneous_step`] evaluates it at `f64`: the
    /// difference, its product by `t` and the sum, each rounded, no fused
    /// multiply-add. The step is affine in its operands, so its first-order
    /// share of their bounds is `|1 − t|·e_self + |t|·e_y + |y − self|·e_t`,
    /// and its own roundings add `u·|y − self|·|t|` (the difference's,
    /// through the product), `u·|(y − self)·t|` (the product's, none where
    /// `t` is a power of two, whose product is exact) and `u·|result|` (the
    /// sum's). Sharper than the same step through [`Sub`], [`Mul`] and
    /// [`Add`], which charge `self`'s bound twice (`1 + |t|`, not `1 − t`).
    #[must_use]
    pub fn step(self, y: Self, t: Self) -> Self {
        let d = y.value - self.value;
        let p = d * t.value;
        let value = self.value + p;
        let product = if power_of_two(t.value) { 0.0 } else { p.abs() };
        Self {
            value,
            error: (1.0 - t.value).abs() * self.error
                + t.value.abs() * y.error
                + d.abs() * t.error
                + UNIT_ROUNDOFF * (d.abs() * t.value.abs() + product + value.abs()),
        }
    }
}

/// Whether `x` is a normal power of two, so a product by it is exact
/// (to first order: an underflow is not charged here, as nowhere in this
/// module).
fn power_of_two(x: f64) -> bool {
    x.is_normal() && x.to_bits() & ((1 << 52) - 1) == 0
}

/// **A homogeneous point out of homogeneous form**, the weight its last
/// channel: each coordinate `h_c / w` with its bound,
/// `(e_c + |h_c / w|·e_w)/(|w| − e_w) + u·|h_c / w|` ([`Rounded::quotient`]),
/// and the weight's channel the exact `1`. `None` where the weight's
/// bound reaches it ([`Rounded::divisor`], under `name`): there the exact
/// weight may be zero and no bound holds.
#[must_use]
pub fn dehomogenized<const N: usize>(
    h: [Rounded<f64>; N],
    name: &'static str,
) -> Option<[Rounded<f64>; N]> {
    let w = h.last()?.divisor(name)?;
    Some(core::array::from_fn(|c| {
        if c + 1 == N {
            Rounded::exact(1.0)
        } else {
            h[c].quotient(w)
        }
    }))
}

/// **A value at any scalar, with its value channel's rounding**: the
/// [`Rounded`] arithmetic carried for a value that is not itself an
/// `f64`. The bound is a size, so it is an `f64` at every scalar, and it
/// means one thing at each:
///
/// - at `f64` (and the `Probe` that records it), a first-order bound on
///   `|value − exact|`, a [`Rounded<f64>`]'s;
/// - at `Interval`, zero: the value is an enclosure that holds the exact
///   value already, and its arithmetic charges nothing here;
/// - at `Dual`, the value channel's bound at the value's own witness (its
///   `f64`'s, or zero over `Interval`); the tangent is not charged;
/// - at `Sym`, its value channel's, at its base's witness: a `Sym<f64>`
///   evaluates as the `f64` it wraps, a `Sym<Interval>` as the enclosure.
///
/// The bound is charged only where the scalar's comparison is a
/// measurement ([`Witness::Inexact`](crate::Witness::Inexact)), so at an
/// enclosure it is zero by construction: every door that charges reads
/// the witness, and nothing else builds one. Each door evaluates the
/// value as the plain expression at `T` does, and charges the bound by
/// the same [`Rounded<f64>`] door on the value channel's middle, which at
/// a measuring scalar is that channel's own `f64`.
#[derive(Debug, Clone, Copy)]
pub struct Tracked<T> {
    value: T,
    rounding: f64,
}

impl<T: SpanLocate> Tracked<T> {
    /// Whether this scalar's arithmetic rounds uncharged, so its doors
    /// carry a bound.
    fn tracks() -> bool {
        T::WITNESS == crate::Witness::Inexact
    }

    /// The value channel's middle with this bound: the operand of the
    /// [`Rounded<f64>`] door that charges a measuring scalar.
    fn shadow(self) -> Rounded<f64> {
        Rounded {
            value: self
                .value
                .structure_bracket()
                .map_or(f64::NAN, crate::bracket_mid),
            error: self.rounding,
        }
    }

    /// `value` by `op` at `T`, its bound by `charge` on the operands'
    /// shadows where the scalar measures, else zero.
    fn charged<const N: usize>(
        value: T,
        operands: [Self; N],
        charge: impl FnOnce([Rounded<f64>; N]) -> Rounded<f64>,
    ) -> Self {
        let rounding = if Self::tracks() {
            charge(operands.map(Self::shadow)).error
        } else {
            0.0
        };
        Self { value, rounding }
    }

    /// An input taken as exact.
    pub fn exact(value: T) -> Self {
        Self {
            value,
            rounding: 0.0,
        }
    }

    /// A ratio of two differences of exact values, `(a − b)/(c − d)`, formed
    /// at `T` by the caller: three roundings of it at a measuring scalar
    /// (the two differences and the quotient, each `u` of it relative).
    pub fn ratio(value: T) -> Self {
        let rounding = if Self::tracks() {
            3.0 * UNIT_ROUNDOFF * Self::exact(value).shadow().value.abs()
        } else {
            0.0
        };
        Self { value, rounding }
    }

    /// The value.
    pub fn value(self) -> T {
        self.value
    }

    /// **[`Rounded::step`] at `T`**: the value by
    /// [`SpanLocate::homogeneous_step`], the bound by the `f64` step.
    #[must_use]
    pub fn step(self, y: Self, t: Self) -> Self {
        Self::charged(
            T::homogeneous_step(self.value, y.value, t.value),
            [self, y, t],
            |[x, y, t]| x.step(y, t),
        )
    }

    /// **A homogeneous point out of homogeneous form** at `T`: each
    /// coordinate's value `h_c / w` at `T`, its bound [`dehomogenized`]'s,
    /// and the weight's channel the exact `1`. `None` where the weight's
    /// bound reaches it; at an enclosure, where nothing is charged, never.
    #[must_use]
    pub fn dehomogenized<const N: usize>(h: [Self; N], name: &'static str) -> Option<[Self; N]> {
        let w = h.last()?.value;
        let roundings = if Self::tracks() {
            dehomogenized(h.map(Self::shadow), name)?.map(|c| c.error)
        } else {
            [0.0; N]
        };
        Some(core::array::from_fn(|c| Self {
            value: if c + 1 == N { T::one() } else { h[c].value / w },
            rounding: roundings[c],
        }))
    }

    /// **This value read in `f64`**: its value channel's middle, off the
    /// exact value by its bound and by the half-width of its bracket (the
    /// whole of it at an enclosure, nothing at `f64`). The poison reads
    /// NaN.
    #[must_use]
    pub fn at_middle(self) -> Rounded<f64> {
        let Some((lo, hi)) = self.value.structure_bracket() else {
            return Rounded {
                value: f64::NAN,
                error: f64::NAN,
            };
        };
        Rounded {
            value: crate::bracket_mid((lo, hi)),
            error: self.rounding + (hi - lo) * 0.5,
        }
    }

    /// The bound on the value channel's rounding (type docs); zero at an
    /// enclosure.
    pub fn rounding(self) -> f64 {
        self.rounding
    }
}

impl<T: SpanLocate> Sub for Tracked<T> {
    type Output = Self;

    fn sub(self, o: Self) -> Self {
        Self::charged(self.value - o.value, [self, o], |[a, b]| a - b)
    }
}

impl<T: SpanLocate> Mul for Tracked<T> {
    type Output = Self;

    fn mul(self, o: Self) -> Self {
        Self::charged(self.value * o.value, [self, o], |[a, b]| a * b)
    }
}

/// The components of an exact vector.
pub fn exact_vec<T: Real>(v: Vec3<T>) -> [Rounded<T>; 3] {
    [v.x, v.y, v.z].map(Rounded::exact)
}

/// [`Vec3::dot`]'s order, with its running bound.
pub fn dot<T: Real>(a: [Rounded<T>; 3], b: [Rounded<T>; 3]) -> Rounded<T> {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// [`Vec3::cross`]'s order, with its running bound.
pub fn cross<T: Real>(a: [Rounded<T>; 3], b: [Rounded<T>; 3]) -> [Rounded<T>; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// `|v|² − 1` for an EXACT vector `v`, pivoted on its largest component
/// `m` as `(m − 1)(m + 1) + (the other two squares)`: a unit axis
/// vector then evaluates with no charge at all, where `Σvᵢ² − 1` would
/// charge three roundings of `1`.
pub fn unit_defect<T: Real>(v: Vec3<T>) -> Rounded<T> {
    let one = Rounded::exact(T::one());
    let [x, y, z] = exact_vec(v);
    let pivoted = |m: Rounded<T>, a: Rounded<T>, b: Rounded<T>| {
        (m - one) * (m + one) + (a.square() + b.square())
    };
    let (ax, ay, az) = (v.x.abs(), v.y.abs(), v.z.abs());
    let on_xy = Rounded::select_le_zero(ax - ay, pivoted(y, x, z), pivoted(x, y, z));
    Rounded::select_le_zero(ax.max(ay) - az, pivoted(z, x, y), on_xy)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// **[`DIRECTION_UNITS`]'s guard.** A unit direction computed by
    /// `Vec3::normalize` from exact inputs is off the true one by at most
    /// `DIRECTION_UNITS·u` in each component: its norm (three squares, two
    /// sums, a root) and its division round a few times each. Read
    /// exactly, `c² · Σ vⱼ² − vᵢ²` for each computed component `c` is the
    /// error times `(c + t)·Σ vⱼ²`, `t` the true component, so the error is
    /// that expansion over `2|c|·Σ vⱼ²` to first order. Over directions of
    /// every octant and scale, the worst stays under the charge, and is
    /// more than one `u`: the charge is neither short nor vacuous.
    #[test]
    fn the_direction_charge_holds_a_normalized_directions_rounding() {
        use crate::exact::Expansion;
        let mut seed = 0x9e37_79b9_7f4a_7c15u64;
        let mut next = || {
            seed = seed
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            #[allow(clippy::cast_precision_loss)]
            let x = (seed >> 11) as f64 / (1u64 << 53) as f64;
            2.0 * x - 1.0
        };
        let mut worst = 0.0f64;
        for k in 0..20_000 {
            let scale = [1e-3, 1.0, 1e3][k % 3];
            let v = Vec3::new(next() * scale, next() * scale, next() * scale);
            if v.norm() < 1e-2 * scale {
                continue;
            }
            let c = v.normalize();
            let sum = [v.x, v.y, v.z].iter().fold(0.0, |s, &x| s + x * x);
            for (ci, vi) in [(c.x, v.x), (c.y, v.y), (c.z, v.z)] {
                if ci.abs() < 1e-3 {
                    continue;
                }
                let mut e = Expansion::default();
                let square = Expansion::product(ci, ci);
                for vj in [v.x, v.y, v.z] {
                    e.add_expansion(&square.scaled(vj).scaled(vj));
                }
                e.add_expansion(&Expansion::product(vi, vi).negated());
                let residual: f64 = e.components().iter().sum();
                worst = worst.max(residual.abs() / (2.0 * ci.abs() * sum) / UNIT_ROUNDOFF);
            }
        }
        assert!(
            worst <= DIRECTION_UNITS,
            "a normalized component off by {worst} u, past the charge's {DIRECTION_UNITS}"
        );
        assert!(
            worst > 1.0,
            "the worst is {worst} u: the charge would be vacuous"
        );
    }

    /// **A divisor within its own rounding of zero divides nothing.** The
    /// vertex-section sine at a torus saddle reads `2e-16 ± 8.9e-16`:
    /// its exact value may be zero, so it is no divisor, and neither is
    /// an exact zero or a value exactly at its bound. Clear of its
    /// bound, the quotient's bound covers both operands' over the room
    /// left: `(1 ± 0) / (2 ± 1)` is `0.5` within `0.5 / 1` and a rounding.
    #[test]
    fn a_divisor_must_clear_its_own_bound() {
        let at = |value: f64, error: f64| Rounded { value, error };
        for (d, label) in [
            (at(2e-16, 8.881_784_197_001_252e-16), "inside its bound"),
            (at(0.0, 8.881_784_197_001_252e-16), "zero with a bound"),
            (at(0.0, 0.0), "an exact zero"),
            (at(-0.25, 0.25), "exactly at its bound"),
        ] {
            assert!(
                d.divisor("running_test_divisor").is_none(),
                "{label}: {d:?} is no divisor"
            );
        }
        let q = at(1.0, 0.0).quotient(at(2.0, 1.0).divisor("running_test_divisor").unwrap());
        assert_eq!(q.value, 0.5, "the value is the plain quotient's");
        assert!(
            (q.error - (0.5 + 0.5 * UNIT_ROUNDOFF)).abs() <= f64::EPSILON,
            "the bound is the operands' share over the room: {q:?}"
        );
        assert!(
            at(-3.0, 1.0).divisor("running_test_divisor").is_some(),
            "a negative divisor clear of its bound"
        );
    }

    /// **The enclosure's ends hold the exact value, on its two sides.**
    /// `0.1 + 0.2` rounds up past the exact sum of its stored operands
    /// (read exactly as an expansion): the lower end lies below the
    /// exact sum and the upper end above it, both strictly, and so do
    /// the negation's ends about the negated sum. A charge widens both
    /// ends by itself, and leaves the value.
    #[test]
    fn the_enclosure_ends_hold_the_exact_value() {
        use crate::exact::Expansion;
        let r = Rounded::exact(0.1_f64) + Rounded::exact(0.2);
        let mut sum = Expansion::default();
        sum.add_expansion(&Expansion::product(0.1, 1.0));
        sum.add_expansion(&Expansion::product(0.2, 1.0));
        let exact = |x: f64| {
            let mut d = sum.clone();
            d.add_expansion(&Expansion::product(x, 1.0).negated());
            d.components().iter().sum::<f64>()
        };
        assert!(exact(r.value) < 0.0, "the fixture rounds up: {r:?}");
        assert!(
            exact(r.lower_end()) > 0.0,
            "the lower end {} is below the exact sum",
            r.lower_end()
        );
        assert!(
            exact(r.upper_end()) < 0.0,
            "the upper end {} is above the exact sum",
            r.upper_end()
        );
        let n = -r;
        assert_eq!(n.error, r.error, "negation is exact");
        assert!(
            n.lower_end() < -r.value && -r.value < n.upper_end() && n.upper_end() == -r.lower_end(),
            "the negation's ends are the ends negated: {n:?}"
        );
        let w = r.with_charge(0.25);
        assert_eq!(w.value, r.value, "a charge leaves the value");
        assert_eq!(
            (w.lower_end(), w.upper_end()),
            (r.value - (r.error + 0.25), r.value + (r.error + 0.25)),
            "a charge widens both ends by itself"
        );
        assert_eq!(
            r.with_charge(-0.25).error,
            w.error,
            "a negative charge widens as its size does, never shrinks"
        );
    }

    /// **An axis-aligned chain reports no error; a rounded one reports
    /// at least its rounding.** The unit defect of `ẑ` is exactly zero
    /// with a zero bound. For `(0.6, 0.8, 0)`, whose squares round, the
    /// bound covers the gap between the `f64` value and the exact
    /// defect `0.6² + 0.8² − 1` of the stored (rounded) components,
    /// evaluated here in exact dyadic arithmetic on the bits.
    #[test]
    fn the_bound_is_zero_on_exact_chains_and_covers_rounded_ones() {
        let z = unit_defect(Vec3::new(0.0, 0.0, 1.0));
        assert_eq!((z.value, z.error), (0.0, 0.0), "ẑ's defect is exact");
        let neg = unit_defect(Vec3::new(0.0, -1.0, 0.0));
        assert_eq!((neg.value, neg.error), (0.0, 0.0), "−ŷ's defect is exact");
        let (a, b) = (0.6_f64, 0.8_f64);
        let d = unit_defect(Vec3::new(a, b, 0.0));
        // Exact: a = ma·2^ea, b likewise; a² + b² − 1 over 2^(2·min e).
        let exact = |x: f64| {
            let bits = x.to_bits();
            let e = i64::try_from((bits >> 52) & 0x7ff).unwrap() - 1075;
            let m = i128::from((bits & ((1 << 52) - 1)) | (1 << 52));
            (m, e)
        };
        let ((ma, ea), (mb, eb)) = (exact(a), exact(b));
        assert_eq!(ea, eb, "the fixture's components share an exponent");
        let scale = -2 * ea; // the squares are m²·2^(2e); 1 is 2^scale·2^(2e)
        let num = ma * ma + mb * mb - (1_i128 << scale);
        let exact_defect = num as f64 * 2f64.powi(i32::try_from(2 * ea).unwrap());
        assert!(
            (d.value - exact_defect).abs() <= d.error && d.error > 0.0,
            "(0.6, 0.8): f64 defect {} ± {} against the exact {exact_defect}",
            d.value,
            d.error
        );
    }

    /// A value with its exact counterpart, an expansion, for the rows
    /// that judge a bound against exact arithmetic.
    #[derive(Clone)]
    struct Judged {
        got: Rounded<f64>,
        exact: crate::exact::Expansion,
    }

    impl Judged {
        fn exact(x: f64) -> Self {
            Self {
                got: Rounded::exact(x),
                exact: crate::exact::Expansion::product(x, 1.0),
            }
        }

        /// `self + (y − self)·t` for an exact `t`, both ways.
        fn step(&self, y: &Self, t: f64) -> Self {
            let mut exact = self.exact.clone();
            exact.add_expansion(&y.exact.scaled(t));
            exact.add_expansion(&self.exact.scaled(t).negated());
            Self {
                got: self.got.step(y.got, Rounded::exact(t)),
                exact,
            }
        }

        /// `|got − exact|` over the bound.
        fn ratio(&self) -> f64 {
            let mut gap = self.exact.clone();
            gap.add(-self.got.value);
            let gap: f64 = gap.components().iter().sum();
            gap.abs() / self.got.error
        }

        /// The exact value, to `f64`.
        fn value(&self) -> f64 {
            self.exact.components().iter().sum()
        }
    }

    /// A seeded uniform draw on `[-1, 1)`.
    fn draws(mut seed: u64) -> impl FnMut() -> f64 {
        move || {
            seed = seed
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            #[allow(clippy::cast_precision_loss)]
            let x = (seed >> 11) as f64 / (1u64 << 53) as f64;
            2.0 * x - 1.0
        }
    }

    /// **[`Rounded::step`]'s bound holds the step's drift**, against the
    /// same steps in exact arithmetic: de Casteljau rows of four to six
    /// points, halved up to twenty levels and cut at a fraction that is
    /// no power of two, at `1e-3` to `1e6` of the origin, with rows whose
    /// points all but cancel (`x (1 + 1e-12·r)`), so the bound carried
    /// is the inputs' rounding as well as each step's own. No value drifts
    /// past its bound.
    #[test]
    fn a_steps_bound_holds_its_drift_against_exact_arithmetic() {
        let mut next = draws(0x5eed_57e9);
        let mut worst = 0.0f64;
        for case in 0..600 {
            let scale = [1e-3, 1.0, 1e3, 1e6][case % 4];
            let far = if case % 3 == 0 { 1e6 } else { 0.0 };
            let n = 4 + case % 3;
            let base = next() * scale + far;
            let mut row: Vec<Judged> = (0..n)
                .map(|_| {
                    let x = if case % 5 == 0 {
                        base * (1.0 + 1e-12 * next())
                    } else {
                        next() * scale + far
                    };
                    // An input that carries its own rounding: a product.
                    let a = Judged::exact(x);
                    let b = Judged::exact(1.0 + next() * 0.5);
                    let mut exact = crate::exact::Expansion::default();
                    for &c in a.exact.components() {
                        exact.add_expansion(&b.exact.scaled(c));
                    }
                    Judged {
                        got: a.got * b.got,
                        exact,
                    }
                })
                .collect();
            let fraction = 0.5 + 0.4 * next();
            for level in 0..20 {
                let t = if level == 7 { fraction } else { 0.5 };
                row = row.windows(2).map(|w| w[0].step(&w[1], t)).collect();
                if row.len() == 1 {
                    let first = row[0].clone();
                    row = vec![first.clone(), first.step(&first, 0.5), first];
                }
                for (k, j) in row.iter().enumerate() {
                    let r = j.ratio();
                    assert!(
                        r <= 1.0,
                        "case {case} level {level} point {k}: {:?} drifts {r} of its bound from {}",
                        j.got,
                        j.value()
                    );
                    worst = worst.max(r);
                }
            }
        }
        assert!(worst > 0.0, "no step rounded at all");
    }

    /// **A step's bound is not loose by half**: over steps of operands
    /// in `[-1, 1)` by fractions in `[0.05, 0.95)`, whose difference,
    /// product and sum all round, some step drifts more than half its
    /// bound from the exact step on the stored operands, so a bound
    /// halved no longer holds it.
    #[test]
    fn a_steps_bound_comes_within_half_of_its_drift() {
        let mut next = draws(0x5eed_57ea);
        let mut worst = 0.0f64;
        for _ in 0..4000 {
            let (x, y, t) = (next(), next(), 0.05 + 0.9 * next().abs());
            let s = Judged::exact(x).step(&Judged::exact(y), t);
            worst = worst.max(s.ratio());
        }
        assert!(
            (0.5..=1.0).contains(&worst),
            "the worst step drifts {worst} of its bound, outside (½, 1]"
        );
    }

    /// **Each of a step's own roundings is owed**, at a fixture where it
    /// decides: `x + (y − x)·t` with `x` all but cancelling the product,
    /// so the sum rounds by almost nothing while the difference and the
    /// product each round near their worst; the drift from the exact step
    /// is past the bound less its product's term (and past it less its
    /// difference's), and within the whole bound.
    #[test]
    fn a_steps_product_and_difference_roundings_are_each_owed() {
        let (x, y, t) = (
            -1.042_363_492_101_883_8,
            0.968_262_008_516_658_2,
            0.518_244_490_440_225_8,
        );
        let s = Judged::exact(x).step(&Judged::exact(y), t);
        let mut gap = s.exact.clone();
        gap.add(-s.got.value);
        let drift = gap.components().iter().sum::<f64>().abs();
        let d = y - x;
        let (product, difference) = (UNIT_ROUNDOFF * (d * t).abs(), UNIT_ROUNDOFF * d.abs() * t);
        assert!(
            drift <= s.got.error,
            "the step drifts {drift:e} past its bound {:?}",
            s.got
        );
        assert!(
            drift > s.got.error - product && drift > s.got.error - difference,
            "the fixture no longer stresses the terms: drift {drift:e}, bound {:?}",
            s.got
        );
    }

    /// **A ratio's own rounding is owed**: `t = (u − lo)/(hi − lo)` from
    /// exact knots, whose two differences and quotient round together by
    /// more than two units of it, and a step by it whose sum cancels. The
    /// drift from the step by the true ratio (`|v·(hi − lo) − x·(hi − lo)
    /// − (y − x)·(u − lo)|` over `hi − lo`, read exactly) is past the bound
    /// less `t`'s share `|y − x|·3u·t`, and within the whole bound
    /// ([`Tracked::ratio`]).
    #[test]
    fn a_ratios_own_rounding_is_owed() {
        use crate::exact::Expansion;
        let (x, y) = (-1.010_251_997_330_188, 0.999_333_217_241_132_9);
        let (u, lo, hi) = (
            0.959_954_238_683_149,
            0.551_813_048_816_387_2,
            1.360_711_498_187_997_3,
        );
        let t = (u - lo) / (hi - lo);
        let v = Tracked::exact(x).step(Tracked::exact(y), Tracked::ratio(t));
        let mut n = Expansion::default();
        for (a, b) in [
            (v.value(), hi),
            (-v.value(), lo),
            (-x, hi),
            (x, lo),
            (-y, u),
            (y, lo),
            (x, u),
            (-x, lo),
        ] {
            n.add_expansion(&Expansion::product(a, b));
        }
        let drift = n.components().iter().sum::<f64>().abs() / (hi - lo);
        let share = (y - x).abs() * 3.0 * UNIT_ROUNDOFF * t;
        assert!(
            drift <= v.rounding(),
            "the step drifts {drift:e} past its bound {v:?}"
        );
        assert!(
            drift > v.rounding() - share,
            "the fixture no longer stresses t's rounding: drift {drift:e}, bound {v:?}"
        );
    }

    /// **A step by a ratio that rounds to a power of two holds its
    /// drift**: `t = (1 − a)/(2 − a)` for `a = 2⁻⁶⁰` rounds to the exact
    /// `0.5` (both differences round to `1` and `2`), so the product by it
    /// is exact while `t` itself is off the true ratio. The step by
    /// [`Tracked::ratio`] of it, judged against the true ratio in exact
    /// arithmetic (`|v·(2 − a) − x·(2 − a) − (y − x)·(1 − a)|` over
    /// `2 − a`), holds its drift: the product charges nothing, and `t`'s
    /// rounding is still charged.
    #[test]
    fn a_step_by_a_ratio_rounded_to_a_power_of_two_holds_its_drift() {
        use crate::exact::Expansion;
        let a = 2f64.powi(-60);
        let t = (1.0 - a) / (2.0 - a);
        assert_eq!(t, 0.5, "the fixture's ratio rounds to the half");
        let mut next = draws(0x5eed_7a71);
        let mut worst = 0.0f64;
        for _ in 0..2000 {
            let (x, y) = (next() * 1e6, next() * 1e6);
            let v = Tracked::exact(x).step(Tracked::exact(y), Tracked::ratio(t));
            let mut n = Expansion::product(v.value(), 2.0);
            n.add_expansion(&Expansion::product(v.value(), -a));
            n.add_expansion(&Expansion::product(x, -2.0));
            n.add_expansion(&Expansion::product(x, a));
            for (c, s) in [(y, 1.0), (-x, 1.0), (y, -a), (-x, -a)] {
                n.add_expansion(&Expansion::product(c, -s));
            }
            let drift = n.components().iter().sum::<f64>().abs() / (2.0 - a);
            let r = drift / v.rounding();
            assert!(r <= 1.0, "{x} → {y}: drifts {r} of its bound {v:?}");
            worst = worst.max(r);
        }
        assert!(worst > 0.0, "no step drifted");
    }

    /// **A point out of homogeneous form holds its drift**
    /// ([`dehomogenized`]): homogeneous points whose coordinates are of
    /// `1e-3` to `1e6`, some all but cancelling, and whose weight is a
    /// product less a value it all but cancels against, so its rounding
    /// runs from none to most of it, each against `H_c / W` read exactly
    /// (`|h_c·W − H_c·w|` over `|w·W|`). Every quotient with a bound holds
    /// its drift; one whose weight's bound reaches the weight has none.
    #[test]
    fn a_dehomogenized_points_bound_holds_its_drift() {
        use crate::exact::Expansion;
        let mut next = draws(0x05ee_dd40);
        let (mut worst, mut refused) = (0.0f64, 0usize);
        for case in 0..4000 {
            let scale = [1e-3, 1.0, 1e3, 1e6][case % 4];
            // The weight: `a·b − c`, `c` near `a·b` by a share that runs
            // from one to `1e-17` of it, past its own rounding.
            let (a, b) = (1.0 + next().abs(), 1.0 + next().abs());
            let share = 10f64.powf(-17.0 * next().abs());
            let c = a * b * (1.0 - share);
            let w = Rounded::exact(a) * Rounded::exact(b) - Rounded::exact(c);
            let mut wx = Expansion::product(a, b);
            wx.add(-c);
            // The coordinates: products by the weight's factors, so each
            // carries a rounding of its own.
            let coordinate = |x: f64| {
                let h = Rounded::exact(x) * Rounded::exact(a);
                (h, Expansion::product(x, a))
            };
            let base = next() * scale;
            let xs = [next() * scale, base, base * (1.0 + 1e-13 * next())];
            let hs = xs.map(coordinate);
            let Some(p) = dehomogenized([hs[0].0, hs[1].0, hs[2].0, w], "running_test_divisor")
            else {
                refused += 1;
                assert!(
                    w.room() <= 0.0,
                    "case {case}: a weight {w:?} clear of its bound read no point"
                );
                continue;
            };
            assert_eq!(p[3].value, 1.0, "the weight's channel is the exact one");
            let wv: f64 = wx.components().iter().sum();
            for (k, (h, hx)) in hs.iter().enumerate() {
                let mut n = wx.scaled(h.value);
                n.add_expansion(&hx.scaled(w.value).negated());
                let n: f64 = n.components().iter().sum();
                let drift = n.abs() / (w.value * wv).abs();
                let r = drift / p[k].error;
                assert!(
                    r <= 1.0,
                    "case {case} coordinate {k}: {:?} over {w:?} drifts {r} of its bound",
                    h
                );
                worst = worst.max(r);
            }
        }
        assert!(
            refused > 0 && worst > 0.0,
            "{refused} refused, worst {worst}"
        );
    }

    /// **A weight whose bound reaches it gives no point**: the weight
    /// `2⁻⁵⁰ ± 2⁻⁴⁹` (a cancellation that left less than its rounding),
    /// and one exactly at its bound, read out of homogeneous form, have no
    /// bound at all; the same weight clear of its bound has one, and it
    /// covers the weight's share over the room left.
    #[test]
    fn a_weight_whose_bound_reaches_it_gives_no_point() {
        let at = |value: f64, error: f64| Rounded { value, error };
        let h = at(1.0, 0.0);
        for (w, label) in [
            (at(2f64.powi(-50), 2f64.powi(-49)), "inside its bound"),
            (at(0.25, 0.25), "exactly at its bound"),
            (at(-0.25, 0.5), "negative, inside its bound"),
        ] {
            assert!(
                dehomogenized([h, w], "running_test_divisor").is_none(),
                "{label}: {w:?} gave a point"
            );
        }
        let p = dehomogenized([h, at(0.5, 0.25)], "running_test_divisor").unwrap();
        assert_eq!(p[0].value, 2.0, "the value is the plain quotient's");
        assert!(
            (p[0].error - (2.0 * 0.25 / 0.25 + 2.0 * UNIT_ROUNDOFF)).abs() <= f64::EPSILON,
            "the bound is the weight's share over the room: {:?}",
            p[0]
        );
    }

    /// **A tracked value charges at a measuring scalar and nowhere
    /// else**: the same chain (a difference, a product, a step, a point
    /// out of homogeneous form) at `f64` carries the [`Rounded<f64>`]
    /// bound of its value, bit for bit; at `Interval` it carries none, and
    /// its value is the enclosure's own arithmetic.
    #[test]
    fn a_tracked_value_charges_where_the_scalar_measures() {
        use crate::{Bounds, Interval};
        fn chain<T: SpanLocate>(x: T, y: T, t: T) -> [Tracked<T>; 2] {
            let (x, y) = (Tracked::exact(x), Tracked::exact(y));
            let w = (y - x) * y;
            let h = x.step(w, Tracked::ratio(t));
            let [p, _] = Tracked::dehomogenized([h, w], "running_test_divisor").unwrap();
            [p, w]
        }
        let (x, y, t) = (0.1, 0.7, 0.3);
        let [p, w] = chain(x, y, t);
        let rw = (Rounded::exact(y) - Rounded::exact(x)) * Rounded::exact(y);
        let rh = Rounded::exact(x).step(rw, Rounded::exact(t).with_charge(3.0 * UNIT_ROUNDOFF * t));
        let rp = dehomogenized([rh, rw], "running_test_divisor").unwrap()[0];
        assert_eq!(
            (w.value().to_bits(), w.rounding().to_bits()),
            (rw.value.to_bits(), rw.error.to_bits()),
            "the weight is the Rounded chain's"
        );
        assert_eq!(
            (p.value().to_bits(), p.rounding().to_bits()),
            (rp.value.to_bits(), rp.error.to_bits()),
            "the point is the Rounded chain's"
        );
        assert!(p.rounding() > 0.0, "the chain rounds");
        let [pi, wi] = chain(
            Interval::from_f64(x),
            Interval::from_f64(y),
            Interval::from_f64(t),
        );
        assert_eq!(
            (pi.rounding(), wi.rounding()),
            (0.0, 0.0),
            "an enclosure charges nothing"
        );
        assert!(
            Bounds::lo(pi.value()) <= p.value()
                && p.value() <= Bounds::hi(pi.value())
                && pi.at_middle().error > 0.0,
            "the enclosure holds the f64 value, and reads at its middle off by its half-width"
        );
    }
}
