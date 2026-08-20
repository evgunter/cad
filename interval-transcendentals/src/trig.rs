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
//! arguments (|x| ≳ 2^52 ≈ 4·10^15) the grid test degrades to "always
//! possibly", so sin/cos return [-1, 1] — maximally loose, never wrong.
//! **Looseness starts well before that**: a false "possibly" on SOME
//! inputs begins near |x| ≈ 2^32, seven binades earlier
//! (`consts::grid_possibly_hits`).

use crate::consts::{frac_pi_2, grid_possibly_hits, neg_frac_pi_2, pi, tau, zero};
use crate::interval::{DInterval, Decoration};
use crate::round::{step_down, step_up};

pub(crate) const PAD_ULPS: u32 = 4;

/// The DERIVED minimum, from `docs/derivations.md` §2: libm's CI
/// bit-distance bound is 2 for `atan2` and 1 for the rest, and Lemma P3
/// turns k bit-steps into k+1 outward steps — so 3 steps is the floor
/// the whole family must clear, set by `atan2`.
///
/// `pad_contract.rs` bounds `PAD_ULPS` from ABOVE, against the derived
/// value. This is the other side, and it is the side no test could
/// supply: a pad that is too small is a containment defect, and
/// containment for the transcendentals is only checkable against a
/// multi-precision oracle — where, measured, `PAD_ULPS = 2` still passes
/// (the bound is worst-case, and libm is usually better than its
/// bound). So the floor is asserted against the DERIVATION at compile
/// time rather than hunted for at run time.
const _: () = assert!(
    PAD_ULPS >= 3,
    "PAD_ULPS is below the derived minimum: atan2 needs 3 outward steps \
     (libm bit-distance 2, Lemma P3 gives k+1). See docs/derivations.md §2."
);

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
    /// grid test degrading totally, for `|x| ≳ 2^52` — that is the trivial
    /// enclosure `[-1, 1]`: maximally loose, never wrong.
    ///
    /// True image ⊆ [-1, 1], so clipping the pads to it is sound and is
    /// what makes the captured extremum bounds exact.
    fn sinusoid(self, f: fn(f64) -> f64, max_at: Self, min_at: Self) -> Self {
        if let Some(p) = Self::propagate1(&self) {
            return p;
        }
        let bounded = self.is_bounded();
        let dec = self.dec.min(Decoration::continuous_on(bounded));
        if !bounded {
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
        self.sinusoid(libm::cos, zero(), pi())
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
    /// huge-argument regime (|x| ≳ 2^52 ≈ 4·10^15), where pole-freedom can no
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
