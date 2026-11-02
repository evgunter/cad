//! **An angle and the form it was written in** (D10
//! `[variables.literals]`: a rational fraction of a turn is a constant).
//!
//! A rotation reads an angle only through its sine and cosine. An angle
//! written as a rational fraction of a turn ([`Angle::from_turns`]) gives
//! them from the fraction, never from the rounded radian: a multiple of
//! a quarter turn gives `0` and `±1`, a sine or cosine of `±1/2` (the
//! other rational values, Niven) gives `±1/2`, and every other value is
//! read off the first octant by the turn's symmetries, so `turn/4`
//! turns a coordinate axis onto another exactly at every scalar. An
//! angle of no such form ([`Angle::radians`]) gives the sine and
//! cosine of its value, as a rotation always has.

use crate::Real;

/// **A rational fraction of a turn**, `num / den` in lowest terms with
/// `den ≥ 1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Turns {
    num: i64,
    den: u64,
}

impl Turns {
    /// `num / den` turns, reduced; `None` for a zero denominator.
    #[must_use]
    pub fn new(num: i64, den: u64) -> Option<Self> {
        if den == 0 {
            return None;
        }
        let g = gcd(num.unsigned_abs(), den);
        let den = den / g;
        let num = num / i64::try_from(g).ok()?;
        Some(Self { num, den })
    }

    /// **`n` quarter turns**, reduced — [`Self::new`]`(n, 4)`, whose
    /// denominator is a literal four and so cannot refuse.
    #[must_use]
    pub fn quarters(n: i64) -> Self {
        // `g` divides four, so it is 1, 2 or 4 and its signed cast exact.
        let g = gcd(n.unsigned_abs(), 4);
        Self {
            num: n / g.cast_signed(),
            den: 4 / g,
        }
    }

    /// The numerator, carrying the sign.
    #[must_use]
    pub fn num(self) -> i64 {
        self.num
    }

    /// The denominator, at least 1.
    #[must_use]
    pub fn den(self) -> u64 {
        self.den
    }

    /// Half this many turns.
    fn half(self) -> Option<Self> {
        Self::new(self.num, self.den.checked_mul(2)?)
    }

    /// **The sine and cosine of this many turns at `T`**, or `None`
    /// where the fraction is too fine to read at `T` exactly (a
    /// denominator past `2^51`). The turn is taken modulo one and split
    /// into whole quarter turns and the rest; a rest of zero is exact,
    /// a rest past an eighth is read as its complement's, and a rest
    /// of a twelfth has sine `1/2` exactly. The quarter turns then swap
    /// and negate the pair, which is exact.
    fn sin_cos<T: Real>(self) -> Option<(T, T)> {
        const EXACT: u64 = 1 << 51;
        if self.den > EXACT {
            return None;
        }
        let den = self.den;
        // `r` in [0, den): the turn modulo one, in units of 1/den.
        let r = u64::try_from(self.num.rem_euclid(i64::try_from(den).ok()?)).ok()?;
        // In units of 1/(4·den): `quarter` whole quarter turns and the
        // rest `s` in [0, den).
        let quarter = (4 * r) / den;
        let s = 4 * r - quarter * den;
        let (sin, cos) = if s == 0 {
            (T::zero(), T::one())
        } else {
            // The rest is `s/(4·den)` turns. Past an eighth it is read
            // as its complement `(den − s)/(4·den)`, whose sine is its
            // cosine.
            let (at, swapped) = if 2 * s > den {
                (den - s, true)
            } else {
                (s, false)
            };
            let angle = T::tau() * T::from_f64(at as f64) / T::from_f64((4 * den) as f64);
            let (sin, cos) = if 3 * at == den {
                // A twelfth of a turn: the sine is one half exactly.
                (T::sin_of_a_twelfth(), angle.cos())
            } else {
                angle.sin_cos()
            };
            if swapped { (cos, sin) } else { (sin, cos) }
        };
        Some(match quarter {
            0 => (sin, cos),
            1 => (cos, -sin),
            2 => (-sin, -cos),
            _ => (-cos, sin),
        })
    }

    /// `12·t` modulo 12 where it is a whole number: the turn in
    /// twelfths, the steps at which a sine or cosine can be rational.
    fn twelfths(self) -> Option<i64> {
        let den = i64::try_from(self.den).ok()?;
        let r = self.num.rem_euclid(den);
        let twelve = r.checked_mul(12)?;
        (twelve % den == 0).then_some(twelve / den)
    }

    /// **The sine, in halves, where it is rational** — `0`, `±1/2` or
    /// `±1` (Niven: no other rational is the sine of a rational fraction
    /// of a turn), as `−2..=2`; `None` where the sine is irrational.
    #[must_use]
    pub fn sin_halves(self) -> Option<i8> {
        const SIN: [Option<i8>; 12] = [
            Some(0),
            Some(1),
            None,
            Some(2),
            None,
            Some(1),
            Some(0),
            Some(-1),
            None,
            Some(-2),
            None,
            Some(-1),
        ];
        SIN.get(usize::try_from(self.twelfths()?).ok()?)
            .copied()
            .flatten()
    }

    /// **The cosine, in halves, where it is rational**: the sine a
    /// quarter turn on.
    #[must_use]
    pub fn cos_halves(self) -> Option<i8> {
        Self::new(
            self.num
                .checked_mul(4)?
                .checked_add(i64::try_from(self.den).ok()?)?,
            self.den.checked_mul(4)?,
        )?
        .sin_halves()
    }

    /// **The tangent where it is rational** — `0` or `±1`, as `−1..=1`;
    /// `None` where it is irrational or has no value (an odd number of
    /// quarter turns).
    #[must_use]
    pub fn tan_rational(self) -> Option<i8> {
        const TAN: [Option<i8>; 8] = [
            Some(0),
            Some(1),
            None,
            Some(-1),
            Some(0),
            Some(1),
            None,
            Some(-1),
        ];
        let den = i64::try_from(self.den).ok()?;
        let eighths = self.num.rem_euclid(den).checked_mul(8)?;
        if eighths % den != 0 {
            return None;
        }
        TAN.get(usize::try_from(eighths / den).ok()?)
            .copied()
            .flatten()
    }

    /// Whether both the sine and cosine are rational: a whole number
    /// of quarter turns.
    fn on_a_quarter(self) -> bool {
        4 % self.den == 0
    }
}

/// **`n` at `T`, for `|n| < 2^64`**: one exact literal up to `2^53`,
/// past it `hi · 2^32 + lo` with both halves exact, so the one rounding
/// is the sum's (outward at `Interval`).
#[allow(clippy::cast_precision_loss)]
fn integer<T: Real>(n: i128) -> T {
    const EXACT: i128 = 1 << 53;
    if n.abs() <= EXACT {
        return T::from_f64(n as f64);
    }
    let (hi, lo) = (n >> 32, n & 0xffff_ffff);
    T::from_f64(hi as f64) * T::from_f64(4_294_967_296.0) + T::from_f64(lo as f64)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// **An angle at the scalar `T`**: its value in radians, and the
/// rational fraction of a turn it was written as, where it was.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle<T> {
    radians: T,
    turns: Option<Turns>,
}

impl<T: Real> Angle<T> {
    /// An angle of `radians`, of no written form: its sine and cosine
    /// are its value's.
    pub fn radians(radians: T) -> Self {
        Self {
            radians,
            turns: None,
        }
    }

    /// **`turns` of a turn**: the fraction its sine and cosine are read
    /// from, and the value it stands for, `τ · num / den`, evaluated at
    /// `T`. The numerator and denominator enter `T` exactly and `τ` is
    /// [`Real::tau`], so at `Interval` the value encloses the true
    /// radian.
    pub fn from_turns(turns: Turns) -> Self {
        Self {
            radians: T::tau() * integer::<T>(i128::from(turns.num()))
                / integer::<T>(i128::from(turns.den())),
            turns: Some(turns),
        }
    }

    /// **`turns` of a turn, whose value at `T` is `radians`**: the
    /// value as the caller evaluated the fraction, and the fraction its
    /// sine and cosine are read from — for a caller that holds the
    /// value already; [`Self::from_turns`] evaluates it.
    pub fn of_turns(radians: T, turns: Turns) -> Self {
        Self {
            radians,
            turns: Some(turns),
        }
    }

    /// The value, in radians.
    pub fn get(self) -> T {
        self.radians
    }

    /// The fraction of a turn it was written as, if any.
    pub fn turns(self) -> Option<Turns> {
        self.turns
    }

    /// **The sine and cosine**: the fraction's ([`Turns`]) where there
    /// is one, else the value's.
    pub fn sin_cos(self) -> (T, T) {
        self.turns
            .and_then(Turns::sin_cos)
            .unwrap_or_else(|| self.radians.sin_cos())
    }

    /// **`(sin θ, 1 − cos θ)` as `I − R` reads them**: from the half
    /// angle, `(2·sin(θ/2)·cos(θ/2), 2·sin²(θ/2))`, so both vanish with
    /// the angle by construction; for a whole number of quarter turns,
    /// whose sine and cosine are exact, `(sin θ, 1 − cos θ)` themselves.
    pub fn sin_versine(self) -> (T, T) {
        let two = T::from_f64(2.0);
        if let Some(turns) = self.turns {
            if turns.on_a_quarter()
                && let Some((sin, cos)) = turns.sin_cos::<T>()
            {
                return (sin, T::one() - cos);
            }
            if let Some((hs, hc)) = turns.half().and_then(Turns::sin_cos::<T>) {
                return (two * hs * hc, two * hs.powi(2));
            }
        }
        let (hs, hc) = (self.radians * T::from_f64(0.5)).sin_cos();
        (two * hs * hc, two * hs.powi(2))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{Angle, Turns, integer};
    use crate::{Bounds, Interval};

    fn turns(num: i64, den: u64) -> Turns {
        Turns::new(num, den).expect("a nonzero denominator")
    }

    fn at(num: i64, den: u64) -> Angle<f64> {
        Angle::from_turns(turns(num, den))
    }

    /// `Turns::quarters(n)` is `Turns::new(n, 4)`, reduced the same way,
    /// out to the ends of `i64`.
    #[test]
    fn quarters_are_the_fraction_over_four() {
        for n in (-9..=9).chain([i64::MIN, i64::MIN + 1, i64::MAX - 1, i64::MAX]) {
            assert_eq!(Some(Turns::quarters(n)), Turns::new(n, 4), "{n} quarters");
        }
    }

    /// **A quarter turn's value is `τ/4`**: at `f64` the rounded `π/2`
    /// bit for bit, at `Interval` an enclosure of the true `π/2` that a
    /// point could not be, its sine and cosine the exact points.
    #[test]
    fn a_quarter_turn_s_value_is_derived_from_the_fraction() {
        let a = Angle::<f64>::from_turns(turns(1, 4));
        assert_eq!(
            a.get().to_bits(),
            core::f64::consts::FRAC_PI_2.to_bits(),
            "f64 value"
        );
        assert_eq!(a.sin_cos(), (1.0, 0.0), "f64 sine and cosine");
        let i = Angle::<Interval>::from_turns(turns(1, 4));
        let r = i.get();
        assert!(
            r.lo() < r.hi() && r.lo() <= core::f64::consts::FRAC_PI_2,
            "Interval value {r:?} is a proper enclosure about π/2"
        );
        assert!(
            core::f64::consts::FRAC_PI_2 <= r.hi(),
            "Interval value {r:?}"
        );
        assert!(
            r.hi() - r.lo() <= f64::EPSILON,
            "Interval value {r:?} is τ's one-ulp enclosure quartered, no wider"
        );
        let (s, c) = i.sin_cos();
        assert_eq!(
            (s.lo(), s.hi(), c.lo(), c.hi()),
            (1.0, 1.0, 0.0, 0.0),
            "Interval sine and cosine"
        );
    }

    /// **An integer past `2^53` enters as an enclosure, not a rounding**:
    /// `2^60 + 1` has no `f64`, so `Interval` holds the two `f64`s about
    /// it, each end at most one outward ulp further, where `f64` rounds
    /// to the nearer.
    #[test]
    fn a_numerator_past_the_mantissa_is_enclosed() {
        let n = (1_i128 << 60) + 1;
        let lo = 2f64.powi(60);
        let hi = lo.next_up();
        let at_interval = integer::<Interval>(n);
        assert!(
            at_interval.lo() <= lo && hi <= at_interval.hi(),
            "Interval {at_interval:?} encloses 2^60 + 1"
        );
        assert!(
            lo.next_down() <= at_interval.lo() && at_interval.hi() <= hi.next_up(),
            "Interval {at_interval:?} is within one ulp of the f64s about 2^60 + 1"
        );
        assert_eq!(integer::<f64>(n), lo, "f64 rounds to nearest");
        assert_eq!(
            integer::<f64>(-(1_i128 << 53)),
            -(2f64.powi(53)),
            "f64 exact"
        );
    }

    /// Every whole number of quarter turns, either sign and past one
    /// turn, has its sine and cosine exactly, where the rounded radian's
    /// carry `6.1e-17`.
    #[test]
    fn a_quarter_turn_is_exact() {
        let rows: [(i64, u64, (f64, f64)); 9] = [
            (0, 1, (0.0, 1.0)),
            (1, 4, (1.0, 0.0)),
            (1, 2, (0.0, -1.0)),
            (3, 4, (-1.0, 0.0)),
            (1, 1, (0.0, 1.0)),
            (-1, 4, (-1.0, 0.0)),
            (-1, 2, (0.0, -1.0)),
            (5, 4, (1.0, 0.0)),
            (-7, 4, (1.0, 0.0)),
        ];
        for (num, den, want) in rows {
            assert_eq!(at(num, den).sin_cos(), want, "{num}/{den} turn");
        }
        assert_ne!(
            core::f64::consts::FRAC_PI_2.cos(),
            0.0,
            "the rounded radian's cosine is not zero, which is the point"
        );
    }

    /// `1 − cos` of a quarter turn is exact too, as `I − R` reads it.
    #[test]
    fn a_quarter_turns_versine_is_exact() {
        assert_eq!(at(1, 4).sin_versine(), (1.0, 1.0));
        assert_eq!(at(1, 2).sin_versine(), (0.0, 2.0));
        assert_eq!(at(-1, 4).sin_versine(), (-1.0, 1.0));
    }

    /// A twelfth's sine and a sixth's cosine are one half exactly; the
    /// irrational partner is the octant's own.
    #[test]
    fn the_rational_halves_are_exact() {
        assert_eq!(at(1, 12).sin_cos().0, 0.5);
        assert_eq!(at(1, 6).sin_cos().1, 0.5);
        assert_eq!(at(5, 12).sin_cos().0, 0.5);
        assert_eq!(at(-1, 12).sin_cos().0, -0.5);
        let (s, c) = at(1, 12).sin_cos();
        assert!(
            (c - 3f64.sqrt() / 2.0).abs() <= 2.0 * f64::EPSILON,
            "{s} {c}"
        );
    }

    /// The rational sines, cosines and tangents, and the irrational ones
    /// answered as none.
    #[test]
    fn the_rational_values_are_niven_s() {
        assert_eq!(turns(1, 12).sin_halves(), Some(1));
        assert_eq!(turns(1, 8).sin_halves(), None);
        assert_eq!(turns(3, 4).sin_halves(), Some(-2));
        assert_eq!(turns(1, 6).cos_halves(), Some(1));
        assert_eq!(turns(1, 2).cos_halves(), Some(-2));
        assert_eq!(turns(1, 12).cos_halves(), None);
        assert_eq!(turns(1, 8).tan_rational(), Some(1));
        assert_eq!(turns(-1, 8).tan_rational(), Some(-1));
        assert_eq!(turns(1, 4).tan_rational(), None);
        assert_eq!(turns(1, 12).tan_rational(), None);
    }

    /// An eighth and three eighths share one octant's numbers, swapped.
    #[test]
    fn the_octants_read_one_number() {
        let (s1, c1) = at(1, 8).sin_cos();
        let (s3, c3) = at(3, 8).sin_cos();
        assert_eq!((s3, c3), (c1, -s1));
    }

    /// A value of no form keeps its own sine and cosine.
    #[test]
    fn a_value_of_no_form_is_its_own() {
        let r = Angle::radians(0.3f64);
        assert_eq!(r.sin_cos(), 0.3f64.sin_cos());
        assert_eq!(r.sin_versine().0, 2.0 * 0.15f64.sin() * 0.15f64.cos());
    }
}
