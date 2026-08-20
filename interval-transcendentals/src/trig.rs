//! Forward trigonometry: `sin`, `cos`, `sin_cos`, `tan`.
//!
//! Pad: `PAD_ULPS = 4` outward steps on every libm endpoint value.
//! Derivation (docs/derivations.md §2, Lemma P3): libm's CI enforces a
//! BIT-DISTANCE of ≤ 1 from the correctly rounded MPFR reference for
//! sin/cos/tan/asin/acos/atan and ≤ 2 for atan2 (precision.rs at tag
//! libm-v0.2.16; musl provenance, full Payne–Hanek reduction, so the
//! bound spans all magnitudes). k bit-steps from RN(t) need k+1 outward
//! steps to enclose t (P3) → sin family needs 2, atan2 needs 3;
//! PAD_ULPS = 4 covers all with margin 2 (atan2: margin 1). The harness
//! hammers the chain against the MPFR oracle.
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
    /// Shared body of [`Self::sin`] and [`Self::cos`], which differ only
    /// in their libm entry point and in where their extrema sit. Both
    /// are total, continuous and bounded, so the decoration is
    /// [`Decoration::continuous_on`] and nothing here can poison.
    ///
    /// `max_at`/`min_at` are the phase offsets of the maxima and minima;
    /// the period is `2π` for both. `grid_possibly_hits` is
    /// conservative in the safe direction: a `false` is a PROOF that no
    /// extremum lies in the box (so the padded endpoint values bound the
    /// monotone pieces), and a `true` pins the bound at the exact
    /// representable extremum ±1.0. For an unbounded box — and, via the
    /// grid test degrading, for `|x| ≳ 4·10^15` — that is the trivial
    /// enclosure `[-1, 1]`: maximally loose, never wrong.
    ///
    /// True image ⊆ [-1, 1], so clipping the pads to it is sound and is
    /// what makes the captured extremum bounds exact.
    fn sinusoid(self, f: fn(f64) -> f64, max_at: Self, min_at: Self) -> Self {
        if let Some(p) = Self::propagate1(&self) {
            return p;
        }
        let dec = self.dec.min(Decoration::continuous_on(self.is_bounded()));
        if !self.is_bounded() {
            return Self::make(-1.0, 1.0, dec);
        }
        let max_possible = grid_possibly_hits(&self, max_at, tau());
        let min_possible = grid_possibly_hits(&self, min_at, tau());
        let (va, vb) = (f(self.lo), f(self.hi));
        let mut lo = f64::min(step_down(va, PAD_ULPS), step_down(vb, PAD_ULPS));
        let mut hi = f64::max(step_up(va, PAD_ULPS), step_up(vb, PAD_ULPS));
        if min_possible {
            lo = -1.0;
        }
        if max_possible {
            hi = 1.0;
        }
        Self::make(lo.max(-1.0), hi.min(1.0), dec)
    }

    /// Enclosure of `sin` over the interval: maxima on the grid
    /// `π/2 + 2πk`, minima on `−π/2 + 2πk`.
    pub fn sin(self) -> Self {
        self.sinusoid(libm::sin, frac_pi_2(), neg_frac_pi_2())
    }

    /// Enclosure of `cos`: maxima on the grid `2πk`, minima on `π + 2πk`.
    pub fn cos(self) -> Self {
        let zero = Self::make(0.0, 0.0, Decoration::Com);
        self.sinusoid(libm::cos, zero, pi())
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
        if !self.is_bounded() || grid_possibly_hits(&self, frac_pi_2(), pi()) {
            let mut r = Self::entire();
            r.dec = self.dec.min(Decoration::Trv);
            return r;
        }
        let lo = step_down(libm::tan(self.lo), PAD_ULPS);
        let hi = step_up(libm::tan(self.hi), PAD_ULPS);
        Self::make(lo, hi, self.dec.min(Decoration::Com))
    }
}
