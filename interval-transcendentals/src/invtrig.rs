//! Inverse trigonometry: `asin`, `acos`, `atan`, `atan2`.
//! Pads as in `trig.rs` (4 outward steps per libm endpoint value;
//! derivations §2/P3). atan2 is the tight one: its libm CI bit-distance
//! bound is 2 (not 1 like the rest), so it needs 3 of the 4 steps —
//! margin 1, called out in the §2 table.
//! Domain-clamp semantics follow the kernel's inari contract: a partial
//! domain miss CLAMPS the input and poisons the decoration to `Trv`; a
//! full miss is Empty (`Trv`); the clamp never decides anything.

use core::ops::ControlFlow;

use crate::interval::{DInterval, Decoration};
use crate::round::{step_down, step_up};
use crate::trig::PAD_ULPS;

/// Upper bound of π/2 (true π/2 lies above the f64 constant, see
/// `consts.rs`' direction facts).
fn half_pi_hi() -> f64 {
    core::f64::consts::FRAC_PI_2.next_up()
}

/// Upper bound of π, and its negation as a lower bound. The range clips
/// of `acos` (`[0, π]`) and `atan2` (`(-π, π]`) both need them, and both
/// need them to be the SAME number as each other: a sound clip must lie
/// outside the true range, so an enclosure of π from above is the only
/// legal choice and there is no room for two spellings of it.
fn pi_hi() -> f64 {
    core::f64::consts::PI.next_up()
}

fn neg_pi_lo() -> f64 {
    (-core::f64::consts::PI).next_down()
}

impl DInterval {
    /// Shared prologue of [`Self::asin`] and [`Self::acos`]: both are
    /// defined exactly on `[-1, 1]`, so both dispose of the same three
    /// cases in the same way — poison/empty propagates, a FULL domain
    /// miss is `Empty`, and a PARTIAL miss clamps the input to `[-1, 1]`
    /// and drops the decoration to `Trv` so the clamp can never decide
    /// anything (the kernel's contract, `lib.rs`).
    ///
    /// `Continue` carries the clamped endpoints and the result
    /// decoration; `Break` carries a finished answer the caller must
    /// return as-is. (`ControlFlow` rather than `Result`: neither arm is
    /// an error, and `Err` naming the successful early return would say
    /// the opposite of what happens.) What each function keeps for
    /// itself is the part that genuinely differs: which endpoint feeds
    /// which bound (`asin` increases, `acos` decreases) and the range it
    /// clips to.
    fn clamp_to_unit(self) -> ControlFlow<Self, (f64, f64, Decoration)> {
        if let Some(p) = Self::propagate1(&self) {
            return ControlFlow::Break(p);
        }
        if self.lo > 1.0 || self.hi < -1.0 {
            return ControlFlow::Break(Self::empty());
        }
        let inside = self.lo >= -1.0 && self.hi <= 1.0;
        let op_dec = if inside {
            Decoration::Com
        } else {
            Decoration::Trv
        };
        ControlFlow::Continue((self.lo.max(-1.0), self.hi.min(1.0), self.dec.min(op_dec)))
    }

    /// Enclosure of `asin` over `self ∩ [-1, 1]` (monotone increasing).
    pub fn asin(self) -> Self {
        let (a, b, dec) = match self.clamp_to_unit() {
            ControlFlow::Continue(v) => v,
            ControlFlow::Break(early) => return early,
        };
        // Range ⊆ [-π/2, π/2]: clip pads to the π/2 enclosure's outer bounds.
        let lo = step_down(libm::asin(a), PAD_ULPS).max(-half_pi_hi());
        let hi = step_up(libm::asin(b), PAD_ULPS).min(half_pi_hi());
        Self::make(lo, hi, dec)
    }

    /// Enclosure of `acos` over `self ∩ [-1, 1]` (monotone DECREASING —
    /// the lower bound comes from the upper endpoint).
    pub fn acos(self) -> Self {
        let (a, b, dec) = match self.clamp_to_unit() {
            ControlFlow::Continue(v) => v,
            ControlFlow::Break(early) => return early,
        };
        // Range ⊆ [0, π]: clip pads (true values ≥ 0, ≤ π < next_up(PI)).
        let lo = step_down(libm::acos(b), PAD_ULPS).max(0.0);
        let hi = step_up(libm::acos(a), PAD_ULPS).min(pi_hi());
        Self::make(lo, hi, dec)
    }

    /// Enclosure of `atan` (total on ℝ, monotone increasing, bounded).
    pub fn atan(self) -> Self {
        if let Some(p) = Self::propagate1(&self) {
            return p;
        }
        let dec = self.dec.min(Decoration::continuous_on(self.is_bounded()));
        let lo = if self.lo == f64::NEG_INFINITY {
            -half_pi_hi()
        } else {
            step_down(libm::atan(self.lo), PAD_ULPS).max(-half_pi_hi())
        };
        let hi = if self.hi == f64::INFINITY {
            half_pi_hi()
        } else {
            step_up(libm::atan(self.hi), PAD_ULPS).min(half_pi_hi())
        };
        Self::make(lo, hi, dec)
    }

    /// Enclosure of the four-quadrant arctangent `atan2(self = y, x)`,
    /// range `(-π, π]`, convention `atan2(0, x<0) = +π` (IEEE 1788).
    ///
    /// Cases: (1) origin in the box → undefined point: full-range hull,
    /// `Trv` (Empty if the box IS the origin); (2) box crosses the branch
    /// cut (negative x-axis approached from y < 0 with y = 0 present) →
    /// full-range hull, `Def` (defined, discontinuous); (3) otherwise the
    /// function is continuous on the box and edge-monotone: padded corner
    /// evaluation, `Com`/`Dac`.
    pub fn atan2(self, x: Self) -> Self {
        if let Some(p) = Self::propagate2(&self, &x) {
            return p;
        }
        let y = self;
        let dec_in = y.dec.min(x.dec);
        let full = || (neg_pi_lo(), pi_hi());
        // Case 1: origin in the box.
        if y.lo <= 0.0 && y.hi >= 0.0 && x.lo <= 0.0 && x.hi >= 0.0 {
            if y.lo == 0.0 && y.hi == 0.0 && x.lo == 0.0 && x.hi == 0.0 {
                return Self::empty(); // atan2 undefined at the only point
            }
            let (lo, hi) = full();
            return Self::make(lo, hi, dec_in.min(Decoration::Trv));
        }
        // Case 2: branch cut crossed (x < 0 present at y = 0, approached
        // from below). Origin is excluded here, so x.hi < 0 necessarily.
        if x.lo < 0.0 && y.lo < 0.0 && y.hi >= 0.0 {
            let (lo, hi) = full();
            return Self::make(lo, hi, dec_in.min(Decoration::Def));
        }
        // Case 3: continuous on the box; extrema at corners (all edges
        // are monotone; see docs/derivations.md §4). Zero y-endpoints are
        // normalized to +0.0: the real number 0 against x < 0 must give
        // +π, not the -0.0 branch's -π (soundness, not taste).
        let norm = |v: f64| if v == 0.0 { 0.0 } else { v };
        let (y0, y1) = (norm(y.lo), norm(y.hi));
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        for yy in [y0, y1] {
            for xx in [x.lo, x.hi] {
                let v = libm::atan2(yy, xx);
                lo = lo.min(step_down(v, PAD_ULPS));
                hi = hi.max(step_up(v, PAD_ULPS));
            }
        }
        lo = lo.max(neg_pi_lo());
        hi = hi.min(pi_hi());
        let op_dec = Decoration::continuous_on(y.is_bounded() && x.is_bounded());
        Self::make(lo, hi, dec_in.min(op_dec))
    }
}
