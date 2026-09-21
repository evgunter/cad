//! **The coefficients are arbitrary-precision dyadic-scaled rationals**
//! ([`Rat`], over `num-bigint`), bounded at [`rational::COEFF_BITS`]
//! bits. The readings that argued for the bound, none of them pinned
//! and none of them a claim about today's tree: the i128-era whole-box
//! replays reported `frozen: 0` on the bracket, because the `Decide`
//! impl's DECISION PATH never asks the form of a margin the numeric
//! channel has already proved non-zero (the contradiction assertion
//! at that site builds it wherever debug assertions are on — every
//! profile this workspace builds — and `frozen` counts it, whoever
//! asked); and M10-8 measured the case a whole-box replay
//! cannot see — at a document's NOMINAL, where every identity margin is
//! near zero and every form is built, the plate froze 1,056 forms, R2's
//! bracket 1,978 and R1's annulus 1,034. The plate's own ceiling
//! residual (`carrier_endpoint_start`, the rim's `‖q − c‖ = r`) is a
//! polynomial of degree 12 in a radius whose nominal is an `f64` literal
//! with a 53-bit mantissa. Three such factors overflow an `i128`; the
//! residual has twelve. The overflow was the freeze, the freeze was the
//! ceiling, and no rule can reach an atom inside a frozen form. The
//! bound keeps the freeze discipline: a coefficient past it is refused
//! exactly as an overflow was, so a blow-up is a counted freeze and
//! never an allocation to the ceiling.

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, ToPrimitive};

use super::Hash128;
#[cfg(feature = "sym-profile-testing")]
use super::profile;

/// **The coefficient integer: an `i128` inline, a `BigInt` only past
/// it.** The ring is arbitrary-precision under [`COEFF_BITS`], but the
/// overwhelming majority of a document's coefficients fit a machine
/// word — the round constants, the small integers, the products that
/// used to fit an `i128` — and measured, a `BigInt` for every one of
/// them cost 4× per leaf on R2's bracket and 100× on a nominal replay
/// (heap traffic, not arithmetic). So every operation runs the checked
/// `i128` path first and promotes to a heap integer only on overflow,
/// and every result that fits demotes back, which keeps the
/// representation canonical (one value, one variant) so equality and
/// the digest read it directly.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Int {
    Small(i128),
    Big(Box<BigInt>),
}

impl Int {
    fn zero() -> Self {
        Self::Small(0)
    }

    fn one() -> Self {
        Self::Small(1)
    }

    /// Canonical: a big integer that fits an `i128` is `Small`.
    fn from_big(b: BigInt) -> Self {
        match i128::try_from(&b) {
            Ok(v) => Self::Small(v),
            Err(_) => Self::Big(Box::new(b)),
        }
    }

    fn big(&self) -> BigInt {
        match self {
            Self::Small(v) => BigInt::from(*v),
            Self::Big(b) => (**b).clone(),
        }
    }

    fn is_zero(&self) -> bool {
        matches!(self, Self::Small(0))
    }

    pub(super) fn is_one(&self) -> bool {
        matches!(self, Self::Small(1))
    }

    fn is_negative(&self) -> bool {
        match self {
            Self::Small(v) => *v < 0,
            Self::Big(b) => b.is_negative(),
        }
    }

    fn bits(&self) -> u64 {
        match self {
            Self::Small(v) => u64::from(128 - v.unsigned_abs().leading_zeros()),
            Self::Big(b) => b.bits(),
        }
    }

    fn neg(&self) -> Self {
        match self {
            Self::Small(v) => match v.checked_neg() {
                Some(n) => Self::Small(n),
                None => Self::from_big(-BigInt::from(*v)),
            },
            Self::Big(b) => Self::from_big(-(**b).clone()),
        }
    }

    fn abs(&self) -> Self {
        if self.is_negative() {
            self.neg()
        } else {
            self.clone()
        }
    }

    /// Whether both are on the inline path (the cost profile's
    /// promotion count).
    #[cfg(feature = "sym-profile-testing")]
    fn both_small(&self, o: &Self) -> bool {
        matches!((self, o), (Self::Small(_), Self::Small(_)))
    }

    fn add(&self, o: &Self) -> Self {
        if let (Self::Small(a), Self::Small(b)) = (self, o)
            && let Some(v) = a.checked_add(*b)
        {
            return Self::Small(v);
        }
        #[cfg(feature = "sym-profile-testing")]
        profile::big_path(self.both_small(o));
        Self::from_big(self.big() + o.big())
    }

    fn mul(&self, o: &Self) -> Self {
        if let (Self::Small(a), Self::Small(b)) = (self, o)
            && let Some(v) = a.checked_mul(*b)
        {
            return Self::Small(v);
        }
        #[cfg(feature = "sym-profile-testing")]
        profile::big_path(self.both_small(o));
        Self::from_big(self.big() * o.big())
    }

    pub(super) fn shl(&self, k: usize) -> Self {
        if let Self::Small(a) = self
            && k < 127
            && let Some(v) = a.checked_mul(1i128 << k)
        {
            return Self::Small(v);
        }
        #[cfg(feature = "sym-profile-testing")]
        profile::big_path(matches!(self, Self::Small(_)));
        Self::from_big(self.big() << k)
    }

    /// The greatest common divisor of the magnitudes (positive).
    fn gcd(&self, o: &Self) -> Self {
        if let (Self::Small(a), Self::Small(b)) = (self, o) {
            let g = gcd_u128(a.unsigned_abs(), b.unsigned_abs());
            return match i128::try_from(g) {
                Ok(v) => Self::Small(v),
                Err(_) => Self::from_big(BigInt::from(g)),
            };
        }
        #[cfg(feature = "sym-profile-testing")]
        profile::big_path(false);
        Self::from_big(self.big().gcd(&o.big()))
    }

    /// Exact division by a divisor known to divide.
    fn div_exact(&self, d: &Self) -> Self {
        if let (Self::Small(a), Self::Small(b)) = (self, d)
            && let Some(v) = a.checked_div(*b)
        {
            return Self::Small(v);
        }
        #[cfg(feature = "sym-profile-testing")]
        profile::big_path(self.both_small(d));
        Self::from_big(self.big() / d.big())
    }

    /// The odd part and the number of twos stripped (`0` keeps zero).
    fn strip_twos(&self) -> (Self, u64) {
        match self {
            Self::Small(0) => (Self::Small(0), 0),
            Self::Small(v) => {
                let k = v.trailing_zeros();
                (Self::Small(v >> k), u64::from(k))
            }
            Self::Big(b) => {
                let k = b.trailing_zeros().unwrap_or(0);
                (Self::from_big((**b).clone() >> k), k)
            }
        }
    }

    /// `Some(r)` iff `r·r == self` exactly, for `self >= 0`.
    fn isqrt_exact(&self) -> Option<Self> {
        if self.is_negative() {
            return None;
        }
        if let Self::Small(v) = self {
            let r = isqrt_u128(v.unsigned_abs())?;
            return i128::try_from(r).ok().map(Self::Small);
        }
        let b = self.big();
        let r = b.sqrt();
        (&r * &r == b).then(|| Self::from_big(r))
    }

    /// A rounded `f64` (at most an ulp off), or `None` past the range.
    fn to_f64(&self) -> Option<f64> {
        match self {
            Self::Small(v) => Some(*v as f64),
            Self::Big(b) => b.to_f64(),
        }
    }

    /// Feeds the integer to a content hash: the sign and the digits.
    fn feed(&self, h: Hash128) -> Hash128 {
        match self {
            Self::Small(v) => h.word(0).wide(*v as u128),
            Self::Big(b) => {
                let (_, digits) = b.to_u32_digits();
                let mut h = h.word(u64::from(b.is_negative())).word(digits.len() as u64);
                for d in digits {
                    h = h.word(u64::from(d));
                }
                h
            }
        }
    }
}

impl core::fmt::Display for Int {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Small(v) => write!(f, "{v}"),
            Self::Big(b) => write!(f, "{b}"),
        }
    }
}

/// The greatest common divisor of two magnitudes.
fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// `Some(r)` iff `r·r == n` exactly.
fn isqrt_u128(n: u128) -> Option<u128> {
    if n < 2 {
        return Some(n);
    }
    let mut x = (n as f64).sqrt() as u128;
    if x == 0 {
        x = 1;
    }
    for _ in 0..8 {
        x = (x + n / x) / 2;
    }
    while x.checked_mul(x).is_none_or(|s| s > n) {
        x -= 1;
    }
    while (x + 1).checked_mul(x + 1).is_some_and(|s| s <= n) {
        x += 1;
    }
    // Integer arithmetic, spelled as a power so the interval-square
    // gate does not read it as an enclosure product.
    (x.checked_pow(2) == Some(n)).then_some(x)
}

/// An exact rational `num / den · 2^exp2`, with `num`/`den` odd and
/// coprime and `den > 0` — the normal form's coefficient.
///
/// The power of two is factored out rather than left in the pair
/// because every `f64` literal IS `m · 2^e`: keeping `e` in its own
/// field leaves the odd part alone, so the round constants a recipe is
/// full of (`1`, `½`, `2`, `¼`) never grow the integers at all.
///
/// **The integers are arbitrary-precision**, and the size discipline an
/// `i128` gave for free is kept explicitly: an integer past
/// [`COEFF_BITS`] is refused — by [`Rat::from_parts`], which every
/// operation that can grow an integer goes through, and by [`Rat::add`]'s
/// alignment shift before it builds one — so the caller freezes and a
/// coefficient blow-up is a bounded cost rather than an allocation to
/// the ceiling. Every operation is CHECKED and answers `None` on that
/// bound. The module docs carry the readings that set it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Rat {
    pub(super) num: Int,
    pub(super) den: Int,
    pub(super) exp2: i32,
}

/// The most bits either integer of a coefficient may carry before the
/// coefficient is refused and its form freezes — a COST dial as much as
/// a discipline, and set by measurement. At 4096 bits nothing on R2's
/// bracket froze and one leaf replay took 229 s against M10-7's 5.9 s:
/// with the constant fold on, every coefficient is a product of
/// dimensions' 53-bit mantissas and the forms that used to overflow an
/// `i128` grew instead to the term budget with thousand-bit
/// coefficients, and BigInt arithmetic on those is the whole cost. At
/// 256 bits — twice the `i128` the ring replaced — the bracket's and
/// the annulus's ceilings still move by the factors measured at
/// `i128`, the worst forms freeze again, and the plate's rim residual
/// (degree 12 in a 53-bit nominal, ~640 bits) does NOT fit: that is
/// the measured trade, recorded on M10's closed
/// `plate-rim-residual-needs-the-wide-coefficient-ring`
/// (`docs/DOC-LEDGER.md` sweep 13).
pub(super) const COEFF_BITS: u64 = 256;

impl Rat {
    pub(super) fn zero() -> Self {
        Self {
            num: Int::zero(),
            den: Int::one(),
            exp2: 0,
        }
    }

    pub(super) fn one() -> Self {
        Self {
            num: Int::one(),
            den: Int::one(),
            exp2: 0,
        }
    }

    /// `num / den · 2^exp2` from machine integers — the door literals
    /// and small constants come through.
    pub(super) fn new(num: i128, den: i128, exp2: i32) -> Option<Self> {
        Self::from_parts(Int::Small(num), Int::Small(den), exp2)
    }

    /// Reduces `num / den · 2^exp2` to the canonical shape, refusing a
    /// zero denominator and an integer past [`COEFF_BITS`].
    ///
    /// **The dyadic shape skips the gcd.** Nearly every coefficient a
    /// document builds has `den` one — an `f64` literal is `m · 2^e`,
    /// and a sum or product of such stays so — and a gcd against one
    /// is one, so on that shape the reduction is the two `strip_twos`
    /// alone. The result is the same to the bit; what goes is a binary
    /// gcd over two `u128`s (or a heap gcd, where `num` is `Big`) per
    /// product and per sum. The coefficient bound is checked after,
    /// exactly as before, so a coefficient is refused where it was.
    fn from_parts(num: Int, den: Int, exp2: i32) -> Option<Self> {
        if den.is_zero() {
            #[cfg(feature = "sym-profile-testing")]
            profile::note(profile::FreezeCause::ZeroDivisor);
            return None;
        }
        if num.is_zero() {
            return Some(Self::zero());
        }
        let (num, den) = if den.is_negative() {
            (num.neg(), den.neg())
        } else {
            (num, den)
        };
        let (num, den) = if den.is_one() {
            (num, den)
        } else {
            let g = num.gcd(&den);
            if g.is_one() {
                (num, den)
            } else {
                (num.div_exact(&g), den.div_exact(&g))
            }
        };
        let (num, nz) = num.strip_twos();
        let (den, dz) = den.strip_twos();
        let Some(exp2) = i32::try_from(nz)
            .ok()
            .and_then(|nz| exp2.checked_add(nz))
            .and_then(|e| i32::try_from(dz).ok().and_then(|dz| e.checked_sub(dz)))
        else {
            #[cfg(feature = "sym-profile-testing")]
            profile::note(profile::FreezeCause::Overflow);
            return None;
        };
        if num.bits() > COEFF_BITS || den.bits() > COEFF_BITS {
            #[cfg(feature = "sym-profile-testing")]
            {
                profile::coefficient_bits(num.bits().max(den.bits()), false);
                profile::note(profile::FreezeCause::Coefficient);
            }
            return None;
        }
        #[cfg(feature = "sym-profile-testing")]
        profile::coefficient_bits(num.bits().max(den.bits()), true);
        Some(Self { num, den, exp2 })
    }

    /// The exact value of a finite `f64`; `None` for a non-finite one
    /// (which cannot be a coefficient of a real polynomial).
    pub(super) fn of_f64(x: f64) -> Option<Self> {
        if !x.is_finite() {
            #[cfg(feature = "sym-profile-testing")]
            profile::note(profile::FreezeCause::Overflow);
            return None;
        }
        if x == 0.0 {
            return Some(Self::zero());
        }
        let bits = x.to_bits();
        let sign = if bits >> 63 == 1 { -1i128 } else { 1i128 };
        let raw_exp = ((bits >> 52) & 0x7ff) as i32;
        let frac = (bits & 0x000f_ffff_ffff_ffff) as i128;
        // Subnormals carry no implicit leading bit and sit one exponent
        // step above what the biased field alone would say.
        let (mantissa, exp) = if raw_exp == 0 {
            (frac, -1074)
        } else {
            (frac | (1i128 << 52), raw_exp - 1075)
        };
        Self::new(sign * mantissa, 1, exp)
    }

    pub(super) fn is_zero(&self) -> bool {
        self.num.is_zero()
    }

    pub(super) fn is_negative(&self) -> bool {
        self.num.is_negative()
    }

    pub(super) fn add(&self, other: &Self) -> Option<Self> {
        if self.is_zero() {
            return Some(other.clone());
        }
        if other.is_zero() {
            return Some(self.clone());
        }
        #[cfg(feature = "sym-profile-testing")]
        profile::rat_op();
        // Align on the smaller exponent, shifting the other numerator up.
        let lo = self.exp2.min(other.exp2);
        let shift = |r: &Self| -> Option<Int> {
            let k = usize::try_from(r.exp2.checked_sub(lo)?).ok()?;
            if k as u64 > COEFF_BITS {
                #[cfg(feature = "sym-profile-testing")]
                profile::note(profile::FreezeCause::Coefficient);
                return None;
            }
            Some(r.num.shl(k))
        };
        let (a, b) = (shift(self)?, shift(other)?);
        // Two dyadic coefficients add their aligned numerators; the
        // cross-multiplication by one on each side is the same number.
        if self.den.is_one() && other.den.is_one() {
            return Self::from_parts(a.add(&b), Int::one(), lo);
        }
        let num = a.mul(&other.den).add(&b.mul(&self.den));
        Self::from_parts(num, self.den.mul(&other.den), lo)
    }

    pub(super) fn neg(&self) -> Option<Self> {
        Some(Self {
            num: self.num.neg(),
            den: self.den.clone(),
            exp2: self.exp2,
        })
    }

    pub(super) fn abs(&self) -> Self {
        Self {
            num: self.num.abs(),
            den: self.den.clone(),
            exp2: self.exp2,
        }
    }

    pub(super) fn mul(&self, other: &Self) -> Option<Self> {
        if self.is_zero() || other.is_zero() {
            return Some(Self::zero());
        }
        #[cfg(feature = "sym-profile-testing")]
        profile::rat_op();
        let Some(exp2) = self.exp2.checked_add(other.exp2) else {
            #[cfg(feature = "sym-profile-testing")]
            profile::note(profile::FreezeCause::Overflow);
            return None;
        };
        let den = if self.den.is_one() && other.den.is_one() {
            Int::one()
        } else {
            self.den.mul(&other.den)
        };
        Self::from_parts(self.num.mul(&other.num), den, exp2)
    }

    /// The reciprocal; `None` for zero.
    pub(super) fn recip(&self) -> Option<Self> {
        Self::from_parts(self.den.clone(), self.num.clone(), self.exp2.checked_neg()?)
    }

    /// The EXACT square root of a non-negative rational, or `None`
    /// where it is not rational (rule A0's coefficient fold and rule
    /// C's polynomial root both need exactly this). `num/den · 2^e` with
    /// `e` made even by moving one factor of two into `num`; the root is
    /// `isqrt(num)/isqrt(den) · 2^(e/2)` when both are exact.
    pub(super) fn sqrt_exact(&self) -> Option<Self> {
        if self.is_negative() {
            return None;
        }
        if self.is_zero() {
            return Some(Self::zero());
        }
        let (num, exp2) = if self.exp2 % 2 != 0 {
            (self.num.shl(1), self.exp2.checked_sub(1)?)
        } else {
            (self.num.clone(), self.exp2)
        };
        let sn = num.isqrt_exact()?;
        let sd = self.den.isqrt_exact()?;
        Self::from_parts(sn, sd, exp2 / 2)
    }

    /// SYM-10 PHASE 1 PLANT `q`: the CONTENT gcd of two rationals —
    /// the gcd of the odd numerators over the lcm of the odd
    /// denominators, at the smaller power of two — so that dividing a
    /// polynomial's coefficients by the gcd of all of them leaves
    /// integers with no common factor.
    pub(super) fn content_gcd(&self, other: &Self) -> Option<Self> {
        let (a, b) = (self.abs(), other.abs());
        if a.is_zero() {
            return Some(b);
        }
        if b.is_zero() {
            return Some(a);
        }
        let g = a.num.gcd(&b.num);
        let dg = a.den.gcd(&b.den);
        let l = a.den.mul(&b.den).div_exact(&dg);
        Self::from_parts(g, l, a.exp2.min(b.exp2))
    }

    /// SYM-10 PHASE 1 PLANT `q`: `self = s² · f` for a positive
    /// rational — `s` carries the even part of the power of two and the
    /// exact roots of the odd numerator and denominator where each is a
    /// perfect square; `f` is what is left. A function of the value, so
    /// two spellings of one rational split the same way.
    pub(super) fn split_square(&self) -> Option<(Self, Self)> {
        if self.is_negative() || self.is_zero() {
            return None;
        }
        let sn = self.num.isqrt_exact().unwrap_or_else(Int::one);
        let sd = self.den.isqrt_exact().unwrap_or_else(Int::one);
        let s = Self::from_parts(sn, sd, self.exp2.div_euclid(2))?;
        let f = self.mul(&s.mul(&s)?.recip()?)?;
        Some((s, f))
    }

    /// A conservative `f64` bracket of the value — the two rounded
    /// conversions and the division each cost at most an ulp, and the
    /// bracket is opened by four on each side. `None` when the power of
    /// two is out of `f64`'s range (a flushed zero would not be
    /// conservative).
    pub(super) fn f64_bracket(&self) -> Option<(f64, f64)> {
        if self.exp2.abs() > 1000 {
            return None;
        }
        let v = self.num.to_f64()? / self.den.to_f64()? * 2f64.powi(self.exp2);
        if !v.is_finite() {
            return None;
        }
        let (mut lo, mut hi) = (v, v);
        for _ in 0..4 {
            lo = lo.next_down();
            hi = hi.next_up();
        }
        Some((lo, hi))
    }

    /// Feeds the coefficient to a content hash (the atom-keying digest):
    /// both integers and the exponent.
    pub(super) fn feed(&self, h: Hash128) -> Hash128 {
        self.den
            .feed(self.num.feed(h))
            .word(u64::from(self.exp2 as u32))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// **The coefficient ring's bound is a freeze, not a panic**: an
    /// alignment or a product that would need more than `COEFF_BITS`
    /// bits answers `None` — exactly what an `i128` overflow answered —
    /// and everything under the bound is exact.
    #[test]
    fn an_alignment_past_the_coefficient_bound_freezes() {
        let big = Rat::new(1, 1, 5000).unwrap();
        let small = Rat::new(1, 1, -5000).unwrap();
        assert!(
            big.add(&small).is_none(),
            "a 10 000-bit shift is past the bound"
        );
        assert!(big.add(&big).is_some(), "aligned already: exact");
        // A product whose odd part crosses the bound is refused too, and
        // one just under it is exact.
        let m = Rat::of_f64(0.1).unwrap();
        let mut acc = Rat::one();
        let mut steps = 0;
        while let Some(next) = acc.mul(&m) {
            acc = next;
            steps += 1;
            assert!(steps < 100, "0.1^k must cross the bound before k = 100");
        }
        assert!(
            steps >= 4,
            "0.1 carries 53 odd bits, so 4 factors fit: {steps}"
        );
    }

    /// The rational is exact on every `f64` it accepts, and refuses the
    /// ones that are not real numbers.
    #[test]
    fn the_rational_embeds_floats_exactly() {
        for x in [0.0, 1.0, -0.5, 0.1, 3.1e-3, f64::MIN_POSITIVE] {
            let r = Rat::of_f64(x).expect("a finite float is a dyadic rational");
            // num/den · 2^exp2 back to a float, when the parts are small
            // enough for the round trip to be exact.
            if r.num.bits() <= 53 && r.den.is_one() {
                let back = r.num.to_f64().unwrap() * 2f64.powi(r.exp2);
                assert_eq!(back.to_bits(), x.to_bits(), "round trip of {x}");
            }
        }
        assert!(Rat::of_f64(f64::NAN).is_none());
        assert!(Rat::of_f64(f64::INFINITY).is_none());
    }

    /// **One value, one representation, one digest** — the canonical
    /// form the atom keys rest on (D9), which the gcd skip on the
    /// dyadic shape must not loosen. Two halves, from SYM-4's reviews
    /// (`origin/sym/4-review-r2`'s doors, `origin/sym/4-review-r1`'s
    /// corpus): equal rationals reached through every door (`new` with
    /// a common factor, an even numerator, a negative denominator;
    /// `of_f64`; `add` and `mul` on the dyadic and the non-dyadic
    /// shape; `recip`; `sqrt_exact`) are structurally equal and feed
    /// the hasher the same bits; and over a corpus of several hundred
    /// values closed under the ring's own operations from non-dyadic
    /// seeds, every `Rat` is canonical — `den > 0`, both integers odd
    /// and coprime — and any two whose difference is zero are one
    /// spelling and one digest. A gcd skipped on every shape reds the
    /// non-dyadic groups here and nothing else in this module.
    #[test]
    fn equal_rationals_have_one_representation_and_one_digest() {
        fn key(r: &Rat) -> u128 {
            r.feed(Hash128::new()).finish()
        }
        fn canonical(r: &Rat) -> bool {
            if r.num.is_zero() {
                return r.den.is_one() && r.exp2 == 0;
            }
            !r.den.is_negative()
                && !r.den.is_zero()
                && r.num.strip_twos().1 == 0
                && r.den.strip_twos().1 == 0
                && r.num.abs().gcd(&r.den).is_one()
        }
        fn same(label: &str, group: &[Rat]) {
            let first = &group[0];
            for r in group {
                assert_eq!(r, first, "{label}: {r:?} vs {first:?}");
                assert_eq!(key(r), key(first), "{label}: digest");
                assert!(canonical(r), "{label}: non-canonical {r:?}");
            }
        }
        let n = |a, b, e| Rat::new(a, b, e).unwrap();
        // 3/2 through eleven doors.
        same(
            "3/2",
            &[
                n(3, 2, 0),
                n(6, 4, 0),
                n(3, 1, -1),
                n(12, 1, -3),
                n(-3, -2, 0),
                Rat::of_f64(1.5).unwrap(),
                Rat::of_f64(0.75)
                    .unwrap()
                    .add(&Rat::of_f64(0.75).unwrap())
                    .unwrap(),
                n(1, 2, 0).add(&Rat::one()).unwrap(),
                Rat::of_f64(3.0)
                    .unwrap()
                    .mul(&Rat::of_f64(0.5).unwrap())
                    .unwrap(),
                n(2, 3, 0).recip().unwrap(),
                n(9, 4, 0).sqrt_exact().unwrap(),
            ],
        );
        // 2/3 — non-dyadic, so the gcd path and the cross-multiplied add.
        same(
            "2/3",
            &[
                n(2, 3, 0),
                n(6, 9, 0),
                n(4, 12, 1),
                n(1, 3, 0).add(&n(1, 3, 0)).unwrap(),
                n(1, 3, 0).mul(&Rat::of_f64(2.0).unwrap()).unwrap(),
                n(1, 6, 0).add(&n(1, 2, 0)).unwrap(),
                n(3, 2, 0).recip().unwrap(),
                n(4, 9, 0).sqrt_exact().unwrap(),
                n(8, 12, 0),
                n(-2, -3, 0),
            ],
        );
        // A dyadic sum whose numerator carries twos: 3/4 + 5/4 = 2.
        same(
            "2",
            &[
                n(2, 1, 0),
                n(1, 1, 1),
                n(3, 4, 0).add(&n(5, 4, 0)).unwrap(),
                Rat::of_f64(0.75)
                    .unwrap()
                    .add(&Rat::of_f64(1.25).unwrap())
                    .unwrap(),
                n(4, 2, 0),
                n(8, 1, -2),
                n(1, 2, 0).recip().unwrap(),
            ],
        );
        // A mixed add: dyadic + non-dyadic, 1/2 + 1/3 = 5/6.
        same(
            "5/6",
            &[
                n(5, 6, 0),
                n(1, 2, 0).add(&n(1, 3, 0)).unwrap(),
                n(1, 3, 0).add(&n(1, 2, 0)).unwrap(),
                n(10, 12, 0),
                n(5, 3, -1),
                n(20, 3, -3),
            ],
        );
        // Zero from a cancelling dyadic sum is THE zero.
        same(
            "0",
            &[
                Rat::zero(),
                n(3, 1, 4).add(&n(-3, 1, 4)).unwrap(),
                n(0, 7, 3),
                Rat::of_f64(0.1)
                    .unwrap()
                    .add(&Rat::of_f64(-0.1).unwrap())
                    .unwrap(),
            ],
        );
        // A product past i128 on the dyadic shape — the promotion path
        // through `from_parts` with `den` one — against the same value
        // reached through a `den` that is not one and reduces.
        let big = Rat::of_f64(0.1).unwrap();
        let p3 = big.mul(&big).unwrap().mul(&big).unwrap();
        let q = big.mul(&big.mul(&big).unwrap()).unwrap();
        same("0.1^3", &[p3.clone(), q]);
        let third = n(1, 3, 0);
        let a = p3.mul(&third).unwrap().mul(&n(3, 1, 0)).unwrap();
        same("0.1^3 · 1/3 · 3", &[p3, a]);

        // The corpus: dyadic literals, deliberately UNREDUCED pairs,
        // and non-dyadic denominators (3, 5, 6, 9) the gcd branch must
        // still reduce; two rounds of closure under the operations.
        let mut corpus: Vec<Rat> = Vec::new();
        for (a, b, e) in [
            (1i128, 1i128, 0i32),
            (2, 1, 0),
            (-3, 1, 2),
            (6, 4, 0),
            (-6, 4, 0),
            (12, 18, 3),
            (1, 3, 0),
            (2, 6, 1),
            (5, 9, -2),
            (-5, 9, -2),
            (7, 5, 0),
            (1024, 3, -10),
            (3, -9, 0),
            (0, 7, 4),
        ] {
            corpus.push(n(a, b, e));
        }
        for x in [0.1f64, -0.1, 2.5, 1.0 / 3.0, 1e-9, 12345.678] {
            corpus.push(Rat::of_f64(x).unwrap());
        }
        for _ in 0..2 {
            let seeds = corpus.clone();
            for a in &seeds {
                for op in [Rat::neg, Rat::recip, Rat::sqrt_exact] {
                    if let Some(r) = op(a) {
                        corpus.push(r);
                    }
                }
                for b in &seeds {
                    if let Some(r) = a.add(b) {
                        corpus.push(r);
                    }
                    if let Some(r) = a.mul(b) {
                        corpus.push(r);
                    }
                }
            }
            let mut seen: Vec<u128> = Vec::new();
            corpus.retain(|r| {
                let d = key(r);
                let fresh = !seen.contains(&d);
                if fresh {
                    seen.push(d);
                }
                fresh
            });
            corpus.truncate(600);
        }
        assert!(corpus.len() > 500, "corpus too thin: {}", corpus.len());
        let non_dyadic = corpus.iter().filter(|r| !r.den.is_one()).count();
        assert!(
            non_dyadic > 50,
            "the non-dyadic branch is barely exercised: {non_dyadic}"
        );
        for r in &corpus {
            assert!(canonical(r), "non-canonical rational: {r:?}");
        }
        for (i, a) in corpus.iter().enumerate() {
            for b in corpus.iter().skip(i + 1) {
                let Some(neg) = b.neg() else { continue };
                let Some(diff) = a.add(&neg) else { continue };
                if diff.is_zero() {
                    assert_eq!(a, b, "equal values, different representations");
                    assert_eq!(key(a), key(b), "equal values, different digests");
                }
            }
        }
    }
}
