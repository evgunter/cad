//! Forward trigonometry: `sin`, `cos`, `sin_cos`, `tan`.
//!
//! Pad: `PAD_ULPS = 4` outward steps on every libm endpoint value.
//! Derivation (docs/derivations.md §2): rust-lang/libm's CI enforces
//! ≤ 1 ulp (of expected) deviation from MPFR for sin/cos/tan (musl, full
//! Payne–Hanek argument reduction, so endpoint values are accurate for
//! ALL finite arguments); worst-case unit conversion gives ≤ 2·ulp(true),
//! and Lemma P2 turns k error-ulps into 2k outward steps → 4. The harness
//! then hammers the chain against the MPFR oracle.
//!
//! Extremum capture: a conservative grid test (consts.rs) decides
//! "possibly contains a max/min point"; a *false* is a proof of absence
//! (monotone-piece shortcut valid), a *true* pins the exact bound ±1.0
//! (representable, always an enclosure of the true extremum). For huge
//! arguments (|x| ≳ 1e16) the grid test degrades to "always possibly",
//! so sin/cos return [-1, 1] — maximally loose, never wrong.

use crate::consts::{frac_pi_2, grid_possibly_hits, neg_frac_pi_2, pi, tau};
use crate::interval::{DInterval, Decoration};
use crate::round::{step_down, step_up};

pub(crate) const PAD_ULPS: u32 = 4;

impl DInterval {
    /// Enclosure of `sin` over the interval. Defined, continuous, and
    /// bounded everywhere: decoration is `Com` for bounded inputs, `Dac`
    /// for unbounded ones.
    pub fn sin(self) -> Self {
        if let Some(p) = Self::propagate1(&self) {
            return p;
        }
        let bounded = self.lo.is_finite() && self.hi.is_finite();
        let dec = self.dec.min(if bounded {
            Decoration::Com
        } else {
            Decoration::Dac
        });
        if !bounded {
            return Self::make(-1.0, 1.0, dec);
        }
        // Maxima of sin lie on the grid π/2 + 2πk; minima on −π/2 + 2πk.
        let max_possible = grid_possibly_hits(&self, frac_pi_2(), tau());
        let min_possible = grid_possibly_hits(&self, neg_frac_pi_2(), tau());
        let (sa, sb) = (libm::sin(self.lo), libm::sin(self.hi));
        let mut lo = f64::min(step_down(sa, PAD_ULPS), step_down(sb, PAD_ULPS));
        let mut hi = f64::max(step_up(sa, PAD_ULPS), step_up(sb, PAD_ULPS));
        if min_possible {
            lo = -1.0;
        }
        if max_possible {
            hi = 1.0;
        }
        // True image ⊆ [-1, 1]: clipping the pads is sound and makes the
        // extremum bounds exact.
        Self::make(lo.max(-1.0), hi.min(1.0), dec)
    }

    /// Enclosure of `cos`. Same structure as [`Self::sin`]; maxima on
    /// the grid 2πk, minima on π + 2πk.
    pub fn cos(self) -> Self {
        if let Some(p) = Self::propagate1(&self) {
            return p;
        }
        let bounded = self.lo.is_finite() && self.hi.is_finite();
        let dec = self.dec.min(if bounded {
            Decoration::Com
        } else {
            Decoration::Dac
        });
        if !bounded {
            return Self::make(-1.0, 1.0, dec);
        }
        let zero = Self::make(0.0, 0.0, Decoration::Com);
        let max_possible = grid_possibly_hits(&self, zero, tau());
        let min_possible = grid_possibly_hits(&self, pi(), tau());
        let (ca, cb) = (libm::cos(self.lo), libm::cos(self.hi));
        let mut lo = f64::min(step_down(ca, PAD_ULPS), step_down(cb, PAD_ULPS));
        let mut hi = f64::max(step_up(ca, PAD_ULPS), step_up(cb, PAD_ULPS));
        if min_possible {
            lo = -1.0;
        }
        if max_possible {
            hi = 1.0;
        }
        Self::make(lo.max(-1.0), hi.min(1.0), dec)
    }

    /// `(sin, cos)` pair — literally the two component calls, mirroring
    /// the kernel `Real::sin_cos` bit-identity contract (the pair IS the
    /// components; no fused path exists to disagree with).
    pub fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
    }

    /// Enclosure of `tan`. Poles at π/2 + πk: if a pole is POSSIBLY in
    /// the interval (conservative grid test), the result is the entire
    /// line with decoration `Trv` — a sound refusal that also covers the
    /// huge-argument regime (|x| ≳ 1e16), where pole-freedom can no
    /// longer be proven. A proven pole-free interval is a monotone
    /// branch: padded endpoint values, decoration up to `Com`.
    pub fn tan(self) -> Self {
        if let Some(p) = Self::propagate1(&self) {
            return p;
        }
        let bounded = self.lo.is_finite() && self.hi.is_finite();
        if !bounded || grid_possibly_hits(&self, frac_pi_2(), pi()) {
            let mut r = Self::entire();
            r.dec = self.dec.min(Decoration::Trv);
            return r;
        }
        let lo = step_down(libm::tan(self.lo), PAD_ULPS);
        let hi = step_up(libm::tan(self.hi), PAD_ULPS);
        Self::make(lo, hi, self.dec.min(Decoration::Com))
    }
}
