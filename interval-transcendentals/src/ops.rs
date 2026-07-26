//! Endpoint-exact operations (no rounding, hence no pads): abs, min, max,
//! floor, copysign-style sign transfer, plus the interval hull.

use crate::interval::{DInterval, Decoration};

impl DInterval {
    /// `|x|`: exact (endpoint selection only).
    pub fn abs(self) -> Self {
        if let Some(p) = Self::propagate1(&self) {
            return p;
        }
        let (lo, hi) = if self.lo >= 0.0 {
            (self.lo, self.hi)
        } else if self.hi <= 0.0 {
            (-self.hi, -self.lo)
        } else {
            (0.0, f64::max(-self.lo, self.hi))
        };
        Self::make(lo, hi, self.dec)
    }

    /// Pointwise minimum `min(x, y)`: exact.
    pub fn min_i(self, rhs: Self) -> Self {
        if let Some(p) = Self::propagate2(&self, &rhs) {
            return p;
        }
        Self::make(
            self.lo.min(rhs.lo),
            self.hi.min(rhs.hi),
            self.dec.min(rhs.dec),
        )
    }

    /// Pointwise maximum `max(x, y)`: exact.
    pub fn max_i(self, rhs: Self) -> Self {
        if let Some(p) = Self::propagate2(&self, &rhs) {
            return p;
        }
        Self::make(
            self.lo.max(rhs.lo),
            self.hi.max(rhs.hi),
            self.dec.min(rhs.dec),
        )
    }

    /// `floor(x)`: exact endpoints. Decoration: `floor` is defined
    /// everywhere but discontinuous at integers; if both endpoints share
    /// one floor value the function is constant on the box (continuous,
    /// bounded → keep up to `Com`), otherwise a jump lies inside → `Def`.
    pub fn floor(self) -> Self {
        if let Some(p) = Self::propagate1(&self) {
            return p;
        }
        let (flo, fhi) = (self.lo.floor(), self.hi.floor());
        // `floor(lo) == floor(hi)` iff no integer lies in `(lo, hi]` iff
        // floor is constant (hence continuous and bounded) on the box.
        let op_dec = if flo == fhi {
            Decoration::Com
        } else {
            Decoration::Def
        };
        Self::make(flo, fhi, self.dec.min(op_dec))
    }

    /// Convex hull of two nonempty intervals (utility for callers; empty
    /// operands are ignored, NaI poisons).
    pub fn hull(self, rhs: Self) -> Self {
        if self.is_nai() || rhs.is_nai() {
            return Self::nai();
        }
        if self.is_empty() {
            return rhs;
        }
        if rhs.is_empty() {
            return self;
        }
        Self::make(
            self.lo.min(rhs.lo),
            self.hi.max(rhs.hi),
            self.dec.min(rhs.dec),
        )
    }

    /// Intersection (both must be nonempty or the result is empty; NaI
    /// poisons). Decoration is capped at `Trv`: 1788 gives set operations
    /// no functional meaning, so nothing stronger may be asserted.
    pub fn intersection(self, rhs: Self) -> Self {
        if self.is_nai() || rhs.is_nai() {
            return Self::nai();
        }
        if self.is_empty() || rhs.is_empty() {
            return Self::empty();
        }
        let lo = self.lo.max(rhs.lo);
        let hi = self.hi.min(rhs.hi);
        if lo > hi {
            return Self::empty();
        }
        Self::make(lo, hi, Decoration::Trv)
    }
}
