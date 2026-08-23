//! `sqrt` and `powi`. Not transcendental, but part of the enclosure
//! surface the kernel consumes — and `powi` carries a soundness-critical
//! tightness contract: an even power of a zero-straddling interval has
//! lower bound EXACTLY 0.0 (no pad), so downstream `sqrt` never sees a
//! spurious negative (the rule ci.yml's "interval-square powi(2)
//! allowlist" step gates in the kernel).

use crate::interval::{DInterval, Decoration};
use crate::round::{down1, mul_hi, mul_lo, two_prod_witness, up1};

impl DInterval {
    /// Enclosure of `sqrt` over `self ∩ [0, ∞)`. Partial domain miss
    /// clamps and poisons to `Trv`; full miss is Empty. `f64::sqrt` is
    /// correctly rounded (IEEE 754), so 1 outward step per endpoint
    /// suffices (Lemma P1); endpoints that are provably exact squares
    /// (FMA witness) take no pad at all.
    pub fn sqrt(self) -> Self {
        if let Some(p) = Self::propagate1(&self) {
            return p;
        }
        if self.hi < 0.0 {
            return Self::empty();
        }
        let inside = self.lo >= 0.0;
        let op_dec = if inside {
            Decoration::Com
        } else {
            Decoration::Trv
        };
        let a = self.lo.max(0.0);
        let lo = sqrt_lo(a);
        let hi = if self.hi == f64::INFINITY {
            f64::INFINITY
        } else {
            sqrt_hi(self.hi)
        };
        Self::make(lo, hi, self.dec.min(op_dec))
    }

    /// Enclosure of the `n`-th integer power. Tight contracts:
    /// - even `n`, zero-straddling input: lower bound exactly `0.0`;
    /// - `n == 0`: value `[1, 1]`, input decoration/NaI/Empty propagate
    ///   (poison flows through every exponent — kernel contract);
    /// - `n < 0`: reciprocal of the positive power (division supplies the
    ///   pole semantics: `Trv` + unbounded when 0 is inside).
    pub fn powi(self, n: i32) -> Self {
        if let Some(p) = Self::propagate1(&self) {
            return p;
        }
        if n == 0 {
            return Self::make(1.0, 1.0, self.dec);
        }
        let m = (i64::from(n)).unsigned_abs();
        let pos = self.pow_pos(m);
        if n > 0 {
            pos
        } else {
            Self::make(1.0, 1.0, self.dec) / pos
        }
    }

    /// `self^m` for `m >= 1`, by sign/parity case split over directed
    /// binary exponentiation.
    fn pow_pos(self, m: u64) -> Self {
        let dec = self.dec.min(Decoration::continuous_on(self.is_bounded()));
        if m.is_multiple_of(2) {
            let (lo, hi) = if self.lo <= 0.0 && self.hi >= 0.0 {
                // Straddles zero: inf of range is EXACTLY 0 (attained).
                (0.0, pow_mag_hi(f64::max(-self.lo, self.hi), m))
            } else {
                let (min_mag, max_mag) = if self.lo > 0.0 {
                    (self.lo, self.hi)
                } else {
                    (-self.hi, -self.lo)
                };
                (pow_mag_lo(min_mag, m), pow_mag_hi(max_mag, m))
            };
            Self::make(lo, hi, dec)
        } else {
            // Odd power: monotone increasing; x^m = sign(x)·|x|^m.
            let lo = signed_pow_lo(self.lo, m);
            let hi = signed_pow_hi(self.hi, m);
            Self::make(lo, hi, dec)
        }
    }
}

fn sqrt_lo(a: f64) -> f64 {
    let s = a.sqrt();
    // `s·s = a` witnessed by round.rs's one 2Prod test, gated on the
    // radicand because that IS the product magnitude here (derivations §3
    // derives one floor for the mul, div and sqrt witnesses alike).
    if two_prod_witness(s, s, a, a) {
        s // provably exact: sqrt(a) is representable
    } else {
        // True sqrt >= 0 always; the pad may not cross below zero.
        down1(s).max(0.0)
    }
}

fn sqrt_hi(b: f64) -> f64 {
    let s = b.sqrt();
    if two_prod_witness(s, s, b, b) {
        s
    } else {
        up1(s)
    }
}

/// Lower bound of `x^m` for `x >= 0` (directed binary exponentiation:
/// products of nonnegative lower bounds, each padded down, stay lower
/// bounds inductively; underflow saturates soundly at 0).
fn pow_mag_lo(x: f64, m: u64) -> f64 {
    let mut acc = 1.0_f64;
    let mut base = x;
    let mut m = m;
    while m > 0 {
        if m % 2 == 1 {
            acc = mul_lo(acc, base).max(0.0);
        }
        m /= 2;
        if m > 0 {
            base = mul_lo(base, base).max(0.0);
        }
    }
    acc
}

/// Upper bound of `x^m` for `x >= 0` (mirror; overflow saturates at +inf).
fn pow_mag_hi(x: f64, m: u64) -> f64 {
    let mut acc = 1.0_f64;
    let mut base = x;
    let mut m = m;
    while m > 0 {
        if m % 2 == 1 {
            acc = mul_hi(acc, base);
        }
        m /= 2;
        if m > 0 {
            base = mul_hi(base, base);
        }
    }
    acc
}

fn signed_pow_lo(x: f64, m: u64) -> f64 {
    if x >= 0.0 {
        pow_mag_lo(x, m)
    } else {
        -pow_mag_hi(-x, m)
    }
}

fn signed_pow_hi(x: f64, m: u64) -> f64 {
    if x >= 0.0 {
        pow_mag_hi(x, m)
    } else {
        -pow_mag_lo(-x, m)
    }
}
