//! Exact arithmetic for a number in a unit: the integers, the real
//! numbers a unit's factor makes of them, and π to as many bits as a
//! decision asks for.
//!
//! A unit's factor is a rational, or a rational times π
//! ([`crate::units`]). A number written in a unit is a decimal, and the
//! float it is is the decimal times the factor, rounded once
//! ([`round_to_f64`]). A canonical float told in a unit is the float
//! divided by the factor, and its decimal at some number of figures is
//! that quotient rounded once ([`Real::scaled`]). Each is a
//! step function of an exact real, decided on a rational enclosure of
//! it: exact where the factor is rational, and where it carries π, at
//! enclosures of π refined until both ends give one answer. That
//! refinement ends, because every answer here steps only at rationals
//! and a nonzero rational times π, or over π, is irrational, so it is
//! never at a step.
//!
//! **What a decision costs.** Nearly every one is decided with π to 56
//! bits and every number in a `u128`; the next most are decided at 112.
//! The bits a decision needs grow with how closely the exact real comes
//! to a step, about 3.3 bits for each figure a decimal shares with an
//! irrational boundary, and π past 2048 bits is computed once per
//! precision reached and kept. A decimal the text door reads is cut to
//! its first [`crate::DECIDING_FIGURES`] figures and a sticky rest
//! ([`crate::UnitDef::canonical`]), so its length costs nothing past
//! that, a run of zeros or nines among them included, save one class: a
//! decimal in `in`, `deg` or `pi rad` that agrees through all of those
//! figures with a rounding boundary, a repeating decimal in `in` and an
//! irrational one in `deg` and `pi rad`. No number a person types or
//! pastes, a float spells, or a file carries agrees with one of those by
//! chance; it has to be computed to, and it is decided at its full
//! length. Measured (an optimized build without debug assertions, CPU
//! time, one fresh process each, a shared box): a random decimal of a
//! million figures, and `1.` then a million zeros then `1`, a few
//! milliseconds in `mm`, `in` and `deg`; a degree decimal that agrees
//! with a boundary through 1,000 figures, 30 ms; through 3,000, 0.29 s;
//! through 10,000, 3.7 s, growing as the square of its length.

use core::cmp::Ordering;
use std::borrow::Cow;
use std::sync::{Mutex, OnceLock};

/// An unsigned integer of any size: little-endian 32-bit limbs, the
/// most significant one nonzero (zero has none).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Nat(Vec<u32>);

impl Nat {
    pub(crate) const fn zero() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn from_u64(value: u64) -> Self {
        let mut n = Self(vec![value as u32, (value >> 32) as u32]);
        n.trim();
        n
    }

    /// The integer a string of ASCII decimal digits spells.
    pub(crate) fn from_digits(digits: &str) -> Self {
        let mut n = Self::zero();
        for chunk in digits.as_bytes().chunks(9) {
            let mut value = 0u32;
            for &b in chunk {
                value = value * 10 + u32::from(b - b'0');
            }
            n.mul_small_add(10u32.pow(chunk.len() as u32), value);
        }
        n
    }

    /// The ASCII decimal digits of this integer; empty for zero.
    pub(crate) fn to_digits(&self) -> String {
        let mut chunks = Vec::new();
        let mut n = self.clone();
        while !n.is_zero() {
            let (q, r) = n.div_small(1_000_000_000);
            chunks.push(r);
            n = q;
        }
        let mut out = String::new();
        for (i, chunk) in chunks.iter().rev().enumerate() {
            if i == 0 {
                out.push_str(&chunk.to_string());
            } else {
                out.push_str(&format!("{chunk:09}"));
            }
        }
        out
    }

    fn trim(&mut self) {
        while self.0.last() == Some(&0) {
            self.0.pop();
        }
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.0.is_empty()
    }

    /// The number of bits below the leading one, plus one; zero for zero.
    pub(crate) fn bits(&self) -> u64 {
        match self.0.last() {
            None => 0,
            Some(&top) => (self.0.len() as u64 - 1) * 32 + u64::from(32 - top.leading_zeros()),
        }
    }

    /// The value, where it fits a `u128`.
    fn to_u128(&self) -> Option<u128> {
        (self.0.len() <= 4).then(|| {
            self.0
                .iter()
                .rev()
                .fold(0u128, |acc, &limb| (acc << 32) | u128::from(limb))
        })
    }

    pub(crate) fn from_u128(value: u128) -> Self {
        let mut n = Self((0..4).map(|i| (value >> (32 * i)) as u32).collect());
        n.trim();
        n
    }

    /// The value, where it fits a `u64`.
    #[cfg(test)]
    pub(crate) fn to_u64(&self) -> Option<u64> {
        match self.0.as_slice() {
            [] => Some(0),
            [lo] => Some(u64::from(*lo)),
            [lo, hi] => Some(u64::from(*lo) | (u64::from(*hi) << 32)),
            _ => None,
        }
    }

    /// `self · m + a`.
    fn mul_small_add(&mut self, m: u32, a: u32) {
        let mut carry = u64::from(a);
        for limb in &mut self.0 {
            let t = u64::from(*limb) * u64::from(m) + carry;
            *limb = t as u32;
            carry = t >> 32;
        }
        if carry != 0 {
            self.0.push(carry as u32);
        }
        self.trim();
    }

    pub(crate) fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        let mut out = vec![0u32; self.0.len() + other.0.len()];
        for (i, &a) in self.0.iter().enumerate() {
            let mut carry = 0u64;
            for (j, &b) in other.0.iter().enumerate() {
                let t = u64::from(a) * u64::from(b) + u64::from(out[i + j]) + carry;
                out[i + j] = t as u32;
                carry = t >> 32;
            }
            let mut k = i + other.0.len();
            while carry != 0 {
                let t = u64::from(out[k]) + carry;
                out[k] = t as u32;
                carry = t >> 32;
                k += 1;
            }
        }
        let mut n = Self(out);
        n.trim();
        n
    }

    pub(crate) fn add(&self, other: &Self) -> Self {
        let (long, short) = if self.0.len() >= other.0.len() {
            (self, other)
        } else {
            (other, self)
        };
        let mut out = Vec::with_capacity(long.0.len() + 1);
        let mut carry = 0u64;
        for (i, &a) in long.0.iter().enumerate() {
            let t = u64::from(a) + u64::from(short.0.get(i).copied().unwrap_or(0)) + carry;
            out.push(t as u32);
            carry = t >> 32;
        }
        if carry != 0 {
            out.push(carry as u32);
        }
        Self(out)
    }

    /// `self − other`, for `self ≥ other`.
    pub(crate) fn sub(&self, other: &Self) -> Self {
        let mut n = self.clone();
        n.sub_assign(other);
        n
    }

    /// `self ← self − other`, for `self ≥ other`.
    fn sub_assign(&mut self, other: &Self) {
        assert!(
            self.cmp_value(other) != Ordering::Less,
            "a natural number is subtracted only from one at least as large"
        );
        let mut borrow = 0i64;
        for (i, a) in self.0.iter_mut().enumerate() {
            if i >= other.0.len() && borrow == 0 {
                break;
            }
            let mut t = i64::from(*a) - i64::from(other.0.get(i).copied().unwrap_or(0)) - borrow;
            borrow = i64::from(t < 0);
            if t < 0 {
                t += 1 << 32;
            }
            *a = t as u32;
        }
        self.trim();
    }

    pub(crate) fn shl(&self, k: u64) -> Self {
        if self.is_zero() {
            return Self::zero();
        }
        let (limbs, bits) = ((k / 32) as usize, (k % 32) as u32);
        let mut out = vec![0u32; limbs];
        if bits == 0 {
            out.extend_from_slice(&self.0);
        } else {
            let mut carry = 0u32;
            for &a in &self.0 {
                out.push((a << bits) | carry);
                carry = a >> (32 - bits);
            }
            if carry != 0 {
                out.push(carry);
            }
        }
        Self(out)
    }

    /// `⌊self / 2^k⌋`.
    pub(crate) fn shr(&self, k: u64) -> Self {
        let limbs = (k / 32) as usize;
        if limbs >= self.0.len() {
            return Self::zero();
        }
        let bits = (k % 32) as u32;
        let rest = &self.0[limbs..];
        let mut out = Vec::with_capacity(rest.len());
        for (i, &a) in rest.iter().enumerate() {
            let next = rest.get(i + 1).copied().unwrap_or(0);
            out.push(if bits == 0 {
                a
            } else {
                (a >> bits) | (next << (32 - bits))
            });
        }
        let mut n = Self(out);
        n.trim();
        n
    }

    pub(crate) fn cmp_value(&self, other: &Self) -> Ordering {
        self.0
            .len()
            .cmp(&other.0.len())
            .then_with(|| self.0.iter().rev().cmp(other.0.iter().rev()))
    }

    /// `(⌊self / d⌋, self mod d)` for a small nonzero `d`.
    pub(crate) fn div_small(&self, d: u32) -> (Self, u32) {
        let mut out = vec![0u32; self.0.len()];
        let mut rem = 0u64;
        for (i, &a) in self.0.iter().enumerate().rev() {
            let t = (rem << 32) | u64::from(a);
            out[i] = (t / u64::from(d)) as u32;
            rem = t % u64::from(d);
        }
        let mut n = Self(out);
        n.trim();
        (n, rem as u32)
    }

    /// `self · m`.
    fn mul_u64(&self, m: u64) -> Self {
        let mut out = Vec::with_capacity(self.0.len() + 2);
        let mut carry = 0u128;
        for &a in &self.0 {
            let t = u128::from(a) * u128::from(m) + carry;
            out.push(t as u32);
            carry = t >> 32;
        }
        while carry != 0 {
            out.push(carry as u32);
            carry >>= 32;
        }
        let mut n = Self(out);
        n.trim();
        n
    }

    /// The top 128 bits of `self >> shift`, for `self >> shift < 2^128`.
    fn top_u128(&self, shift: u64) -> u128 {
        let Some(top) = self.shr(shift).to_u128() else {
            unreachable!("a shifted value this module asks for is below 2^128")
        };
        top
    }

    /// `(⌊self / d⌋, self mod d)` for a nonzero `d` and a quotient below
    /// `2^64`: the quotient estimated from the leading bits, under by at
    /// most a few, then corrected.
    pub(crate) fn div_rem(&self, d: &Self) -> (u64, Self) {
        assert!(!d.is_zero(), "no number is divided by zero");
        if let (Some(a), Some(b)) = (self.to_u128(), d.to_u128()) {
            let Ok(q) = u64::try_from(a / b) else {
                unreachable!("a quotient this module asks for is below 2^64")
            };
            return (q, Self::from_u128(a % b));
        }
        let shift = d.bits().saturating_sub(64);
        let d_top = d.top_u128(shift);
        let a_top = self.top_u128(shift);
        // d ≥ d_top·2^shift and d < (d_top + 1)·2^shift, so this is no
        // more than the quotient, and short of it by at most two.
        let estimate = a_top / (d_top + u128::from(shift > 0));
        let Ok(mut q) = u64::try_from(estimate) else {
            unreachable!("a quotient this module asks for is below 2^64")
        };
        let mut rem = self.sub(&d.mul_u64(q));
        while rem.cmp_value(d) != Ordering::Less {
            rem.sub_assign(d);
            q += 1;
        }
        (q, rem)
    }

    /// `10^k`: from a table kept between calls up to the powers a float
    /// can ask for, computed past it.
    pub(crate) fn pow10(k: u64) -> Cow<'static, Self> {
        const KEPT: usize = 700;
        static TABLE: OnceLock<Vec<Nat>> = OnceLock::new();
        let table = TABLE.get_or_init(|| {
            core::iter::successors(Some(Self::from_u64(1)), |n| {
                let mut next = n.clone();
                next.mul_small_add(10, 0);
                Some(next)
            })
            .take(KEPT)
            .collect()
        });
        match usize::try_from(k).ok().and_then(|k| table.get(k)) {
            Some(power) => Cow::Borrowed(power),
            None => {
                let mut n = table[KEPT - 1].clone();
                let mut left = k - (KEPT as u64 - 1);
                while left > 0 {
                    let step = left.min(9);
                    n.mul_small_add(10u32.pow(step as u32), 0);
                    left -= step;
                }
                Cow::Owned(n)
            }
        }
    }
}

/// π enclosed: `[lo, hi] / 2^bits`, by Machin's formula,
/// `π = 16 atan(1/5) − 4 atan(1/239)`.
///
/// Each series term is the exact floor of its value at a scale of
/// `2^(bits + GUARD)` (the floor of a floor divided by an integer is
/// the floor of the quotient), so a sum of `n` terms is short of its
/// value by less than `n`, and the terms left off once one floors to
/// zero alternate and decrease, so they add less than one more.
fn machin(bits: u64) -> (Nat, Nat) {
    const GUARD: u64 = 32;
    let scale = bits + GUARD;
    // (Σ even terms, Σ odd terms, how far the alternating sum may be off)
    let atan_inverse = |x: u32| {
        let x2 = x.pow(2);
        let mut t = Nat::from_u64(1).shl(scale).div_small(x).0;
        let (mut even, mut odd, mut k) = (Nat::zero(), Nat::zero(), 0u32);
        while !t.is_zero() {
            let term = t.div_small(2 * k + 1).0;
            if k % 2 == 0 {
                even = even.add(&term);
            } else {
                odd = odd.add(&term);
            }
            t = t.div_small(x2).0;
            k += 1;
        }
        (even, odd, u64::from(k) + 1)
    };
    let (e5, o5, n5) = atan_inverse(5);
    let (e239, o239, n239) = atan_inverse(239);
    let sixteen = Nat::from_u64(16);
    let four = Nat::from_u64(4);
    let plus = sixteen.mul(&e5).add(&four.mul(&o239));
    let minus = sixteen.mul(&o5).add(&four.mul(&e239));
    let approx = plus.sub(&minus);
    let slack = Nat::from_u64(16 * n5 + 4 * n239);
    let lo = approx.sub(&slack).shr(GUARD);
    let hi = approx.add(&slack).shr(GUARD).add(&Nat::from_u64(1));
    (lo, hi)
}

/// π enclosed at `bits` bits: `[lo, hi] / 2^bits`, from an enclosure at
/// [`PI_KEPT`] bits, or past that from the finest any call has asked
/// for, kept between calls, so a refinement computes Machin's series
/// once per precision it reaches rather than once per decision.
fn pi(bits: u64) -> (Nat, Nat) {
    // The precision nearly every decision reads, without a lock.
    static FIRST: OnceLock<(Nat, Nat)> = OnceLock::new();
    if bits <= PI_KEPT {
        let (lo, hi) = FIRST.get_or_init(|| machin(PI_KEPT));
        return cut(lo, hi, PI_KEPT, bits);
    }
    static KEPT: Mutex<Option<(u64, Nat, Nat)>> = Mutex::new(None);
    let mut kept = KEPT
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    pi_kept(&mut kept, bits)
}

/// π at `bits` bits from `kept`, extended to `bits` first where it is
/// coarser or absent.
fn pi_kept(kept: &mut Option<(u64, Nat, Nat)>, bits: u64) -> (Nat, Nat) {
    if kept.as_ref().is_none_or(|(at, _, _)| *at < bits) {
        let (lo, hi) = machin(bits);
        *kept = Some((bits, lo, hi));
    }
    let Some((at, lo, hi)) = kept.as_ref() else {
        unreachable!("an enclosure was kept on the lines above")
    };
    cut(lo, hi, *at, bits)
}

/// `[lo, hi] / 2^at` cut to `bits ≤ at` bits, rounded outward.
fn cut(lo: &Nat, hi: &Nat, at: u64, bits: u64) -> (Nat, Nat) {
    let coarser = at - bits;
    (lo.shr(coarser), hi.shr(coarser).add(&Nat::from_u64(1)))
}

/// The least precision π is kept at.
const PI_KEPT: u64 = 2048;

/// A positive real `n / d · π^pi`, `pi` one of −1, 0, 1.
#[derive(Clone, Debug)]
pub(crate) struct Real {
    pub(crate) n: Nat,
    pub(crate) d: Nat,
    pub(crate) pi: i8,
}

impl Real {
    /// `answer` of this real, where `answer` is a step function of a
    /// positive rational `n / d` whose steps are at rationals: exact
    /// where the real is rational, and otherwise read at enclosures of
    /// π refined until both ends answer alike (the module docs).
    pub(crate) fn decide<A: PartialEq>(&self, answer: impl Fn(&Nat, &Nat) -> A) -> A {
        if self.pi == 0 {
            return answer(&self.n, &self.d);
        }
        // Enough for most decisions with every number in a `u128`.
        let mut bits = 56;
        loop {
            let (lo, hi) = pi(bits);
            let one = Nat::from_u64(1).shl(bits);
            let (below, above) = if self.pi > 0 {
                (
                    answer(&self.n.mul(&lo), &self.d.mul(&one)),
                    answer(&self.n.mul(&hi), &self.d.mul(&one)),
                )
            } else {
                (
                    answer(&self.n.mul(&one), &self.d.mul(&hi)),
                    answer(&self.n.mul(&one), &self.d.mul(&lo)),
                )
            };
            if below == above {
                return below;
            }
            bits *= 2;
        }
    }

    /// The decimal exponent of this real's leading digit, and whether
    /// it is that power of ten exactly. `estimate` is a guess within a
    /// decade.
    pub(crate) fn decade(&self, estimate: i64) -> (i64, bool) {
        let mut k = estimate + 1;
        loop {
            match self.scaled(k) {
                (0, _) => k -= 1,
                (q, rest) => {
                    let digits = i64::from(q.ilog10());
                    return (
                        k + digits,
                        q == 10u64.pow(digits as u32) && rest == Rest::Zero,
                    );
                }
            }
        }
    }

    /// This real over `10^k`: its integer part, below `2^64`, and how
    /// what is left stands to a half.
    pub(crate) fn scaled(&self, k: i64) -> (u64, Rest) {
        self.decide(|n, d| {
            let (n, d) = scale(n, d, k);
            let (q, r) = n.div_rem(&d);
            let rest = if r.is_zero() {
                Rest::Zero
            } else {
                match r.shl(1).cmp_value(&d) {
                    Ordering::Less => Rest::BelowHalf,
                    Ordering::Equal => Rest::Half,
                    Ordering::Greater => Rest::AboveHalf,
                }
            };
            (q, rest)
        })
    }

    /// The float nearest this real, ties to even, `inf` past the format.
    pub(crate) fn to_f64(&self) -> f64 {
        f64::from_bits(self.decide(|n, d| round_to_f64(n, d).to_bits()))
    }
}

/// How what is left of a quotient stands to a half.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Rest {
    Zero,
    BelowHalf,
    Half,
    AboveHalf,
}

/// How a magnitude is rounded to an integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Magnitude {
    Floor,
    Ceil,
    /// Ties to even.
    Nearest,
    /// Ties away from zero: std's choice between two shortest digits.
    NearestAway,
}

impl Magnitude {
    /// `floor`, with what is left `rest`, rounded so.
    pub(crate) fn round(self, floor: u64, rest: Rest) -> u64 {
        let up = match (self, rest) {
            (_, Rest::Zero) | (Self::Floor, _) => false,
            (Self::Ceil, _) | (_, Rest::AboveHalf) => true,
            (_, Rest::BelowHalf) => false,
            (Self::Nearest, Rest::Half) => floor % 2 == 1,
            (Self::NearestAway, Rest::Half) => true,
        };
        floor + u64::from(up)
    }
}

/// `(n, d · 10^k)`, or `(n · 10^−k, d)` for a negative `k`.
fn scale(n: &Nat, d: &Nat, k: i64) -> (Nat, Nat) {
    let power = Nat::pow10(k.unsigned_abs());
    if k >= 0 {
        (n.clone(), d.mul(&power))
    } else {
        (n.mul(&power), d.clone())
    }
}

/// The float nearest the positive rational `n / d`, ties to even, and
/// `inf` at or past `f64::MAX` plus half its spacing; `0` for `n = 0`.
pub(crate) fn round_to_f64(n: &Nat, d: &Nat) -> f64 {
    if n.is_zero() {
        return 0.0;
    }
    // n/d is in [2^(k−1), 2^(k+1)); scale it to a quotient of 55 or 56
    // bits, so the 53 kept and the rounding bit are all in hand.
    let k = n.bits() as i64 - d.bits() as i64;
    let s = 55 - k;
    let (q, exact) = scaled_quotient(n, d, s);
    // The value is (q + r/den) · 2^−s; its leading bit has weight 2^e.
    let e = i64::from(64 - q.leading_zeros()) - 1 - s;
    // The last bit kept: 53 below the leading one, or the format's last.
    let lsb = (e - 52).max(-1074);
    let drop = lsb + s;
    let kept = if drop >= 64 { 0 } else { q >> drop };
    let half = (1..=64).contains(&drop) && (q >> (drop - 1)) & 1 == 1;
    let below = match drop {
        ..=1 => 0,
        2..=64 => q & ((1u64 << (drop - 1)) - 1),
        _ => q,
    };
    let sticky = below != 0 || !exact;
    let kept = kept + u64::from(half && (sticky || kept & 1 == 1));
    ldexp(kept as f64, lsb)
}

/// `⌊n · 2^s / d⌋` for a quotient below `2^64`, and whether it is exact:
/// in `u128` where both fit well within it, and in [`Nat`] otherwise.
fn scaled_quotient(n: &Nat, d: &Nat, s: i64) -> (u64, bool) {
    const NATIVE: u64 = 120;
    if n.bits() <= NATIVE && d.bits() <= NATIVE {
        let (Some(a), Some(b)) = (n.to_u128(), d.to_u128()) else {
            unreachable!("numbers of at most {NATIVE} bits fit a u128")
        };
        if s < 0 {
            // d · 2^−s has the bits of n less 55, which is below 128.
            let b = b << s.unsigned_abs();
            return (as_u64(a / b), a % b == 0);
        }
        let (mut q, mut r) = (a / b, a % b);
        for _ in 0..s {
            r <<= 1;
            q <<= 1;
            if r >= b {
                r -= b;
                q |= 1;
            }
        }
        return (as_u64(q), r == 0);
    }
    let (num, den) = if s >= 0 {
        (n.shl(s.unsigned_abs()), d.clone())
    } else {
        (n.clone(), d.shl(s.unsigned_abs()))
    };
    let (q, r) = num.div_rem(&den);
    (q, r.is_zero())
}

fn as_u64(q: u128) -> u64 {
    let Ok(q) = u64::try_from(q) else {
        unreachable!("a quotient scaled to 56 bits fits 64")
    };
    q
}

/// `m · 2^p`, exact where that is a float, `inf` past the format.
fn ldexp(m: f64, p: i64) -> f64 {
    let power = |p: i64| f64::from_bits(((p + 1023) as u64) << 52);
    let (mut x, mut p) = (m, p);
    while p > 1023 {
        x *= power(1023);
        p -= 1023;
        if x.is_infinite() {
            return x;
        }
    }
    while p < -1022 {
        x *= power(-1022);
        p += 1022;
    }
    x * power(p)
}

#[cfg(test)]
mod tests {
    use super::{Nat, PI_KEPT, machin, pi, pi_kept, round_to_f64};
    use core::cmp::Ordering;

    /// π to a hundred places, truncated: `P / 10^100 < π < (P + 1) / 10^100`.
    const PI_100: &str = "31415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";

    /// Whether `[lo, hi] / 2^bits` holds π, against [`PI_100`]: the lower
    /// end not above `(P + 1)/10^100` and the upper not below `P/10^100`,
    /// which at 300 bits or fewer is within a hair of π itself.
    fn holds_pi(lo: &Nat, hi: &Nat, bits: u64) -> bool {
        let p = Nat::from_digits(PI_100);
        let scale = Nat::pow10(100);
        let one = Nat::from_u64(1).shl(bits);
        let lo_ok = lo
            .mul(&scale)
            .cmp_value(&p.add(&Nat::from_u64(1)).mul(&one))
            != Ordering::Greater;
        let hi_ok = hi.mul(&scale).cmp_value(&p.mul(&one)) != Ordering::Less;
        lo_ok && hi_ok
    }

    /// Machin's enclosure holds π, against digits of π written out, at
    /// every width up to what those digits decide, and is at most two
    /// units of its last place wide.
    #[test]
    fn the_enclosure_contains_pi() {
        for bits in [8, 56, 64, 112, 128, 160, 256, 300] {
            let (lo, hi) = machin(bits);
            assert!(holds_pi(&lo, &hi, bits), "{bits}");
            assert!(
                hi.sub(&lo).to_u64().is_some_and(|w| w <= 2),
                "{bits}: width"
            );
        }
    }

    /// The kept enclosure, at every width a decision asks of it, holds π
    /// and holds the finer one it is cut from: below the kept precision,
    /// at it, and past it, where the kept one is extended.
    #[test]
    fn the_kept_enclosure_contains_pi_at_every_width() {
        for bits in [56, 112, 300] {
            let (lo, hi) = pi(bits);
            assert!(holds_pi(&lo, &hi, bits), "{bits}");
        }
        // The extension is read through a kept enclosure of this test's
        // own, so its branches do not hang on what other tests asked of
        // the process-wide one: empty, kept coarser, and kept finer.
        let fine = 3 * PI_KEPT;
        let (flo, fhi) = machin(fine);
        let mut kept = None;
        for bits in [PI_KEPT + 1, 2 * PI_KEPT, PI_KEPT + 7, fine, 300] {
            let (lo, hi) = pi_kept(&mut kept, bits);
            let coarser = fine - bits;
            assert!(
                lo.shl(coarser).cmp_value(&flo) != Ordering::Greater,
                "{bits}: lo"
            );
            assert!(
                hi.shl(coarser).cmp_value(&fhi) != Ordering::Less,
                "{bits}: hi"
            );
            assert!(
                hi.sub(&lo).to_u64().is_some_and(|w| w <= 4),
                "{bits}: width"
            );
        }
        assert!(
            kept.is_some_and(|(at, _, _)| at == fine),
            "kept at the finest asked"
        );
    }

    /// Ties go to the even float at the bottom of the format and at its
    /// top, where the even neighbour of `f64::MAX` is past it.
    #[test]
    fn a_tie_rounds_to_even_at_both_ends_of_the_format() {
        let one = Nat::from_u64(1);
        let power = |k: u64| one.shl(k);
        let at = |n: u64, k: u64| round_to_f64(&Nat::from_u64(n), &power(k));
        // 2^−1075 is halfway from 0 to the least subnormal: to 0.
        assert_eq!(at(1, 1075).to_bits(), 0);
        // 3·2^−1075 is halfway from it to the next: to 2·2^−1074.
        assert_eq!(at(3, 1075).to_bits(), 2);
        // Just above the first tie: up.
        assert_eq!(round_to_f64(&power(1).add(&one), &power(1076)).to_bits(), 1);
        // f64::MAX + half its spacing is past the format; one less is MAX.
        let max_tie = power(1024).sub(&power(970));
        assert_eq!(round_to_f64(&max_tie, &one), f64::INFINITY);
        assert_eq!(round_to_f64(&max_tie.sub(&one), &one), f64::MAX);
        // Exact values are themselves.
        assert_eq!(at(3, 1), 1.5);
        assert_eq!(round_to_f64(&power(1023), &one), 2f64.powi(1023));
    }
}
