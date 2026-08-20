//! Endpoint-exact operations (no rounding, hence no pads): `abs`, `min`,
//! `max`, `floor`, plus the two set operations `hull` and `intersection`.
//!
//! **`copysign` is deliberately NOT here**, though the kernel's `Real`
//! surface has one. Sign transfer on an interval is not endpoint
//! selection: a `sign` operand that CONTAINS zero has no sign to
//! transfer, and the answer is a hull of `±|self|` whose decoration is
//! capped at `Def`. That reasoning, and the signed-zero handling that
//! goes with it, live where the trait is implemented —
//! `crates/geom-core/src/interval.rs` — built out of `abs`, `hull` and
//! negation from this module. Whether it belongs down here instead is
//! part of the open `RingInterval`-vs-`Interval` question and is not
//! settled by moving it.

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
    ///
    /// DELIBERATE DIVERGENCE (docs/semantics-diffs.md D7): the result
    /// keeps `min` of the operand decorations, where IEEE 1788 and inari
    /// give set operations `Trv`. `min(dec)` never exceeds either input,
    /// so no poison is laundered — it just lets clean values stay clean
    /// through hull-shaped code paths.
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

#[cfg(test)]
mod tests {
    use super::*;

    fn di(lo: f64, hi: f64) -> DInterval {
        DInterval::from_bounds(lo, hi)
    }

    #[test]
    fn abs_endpoint_selection_is_exact() {
        let neg = di(-3.0, -1.0).abs();
        assert_eq!((neg.lo(), neg.hi()), (1.0, 3.0));
        let strad = di(-2.0, 5.0).abs();
        assert_eq!((strad.lo(), strad.hi()), (0.0, 5.0));
        let pos = di(1.0, 4.0).abs();
        assert_eq!((pos.lo(), pos.hi()), (1.0, 4.0));
        assert_eq!(pos.decoration(), Decoration::Com);
        assert!(DInterval::nai().abs().is_nai());
        assert!(DInterval::empty().abs().is_empty());
    }

    #[test]
    fn min_max_pointwise_exact_and_dec_min() {
        let a = di(1.0, 4.0);
        let mut b = di(2.0, 3.0);
        b.dec = Decoration::Def; // simulate upstream poison level
        let mn = a.min_i(b);
        assert_eq!((mn.lo(), mn.hi()), (1.0, 3.0));
        assert_eq!(mn.decoration(), Decoration::Def, "dec = min of operands");
        let mx = a.max_i(b);
        assert_eq!((mx.lo(), mx.hi()), (2.0, 4.0));
        assert!(a.min_i(DInterval::nai()).is_nai());
        assert!(a.max_i(DInterval::empty()).is_empty());
    }

    #[test]
    fn floor_constant_box_is_com_jump_is_def() {
        // No integer in (lo, hi]: constant on the box -> Com.
        let c = di(2.25, 2.75).floor();
        assert_eq!(
            (c.lo(), c.hi(), c.decoration()),
            (2.0, 2.0, Decoration::Com)
        );
        // Jump inside -> Def.
        let j = di(2.25, 3.25).floor();
        assert_eq!(
            (j.lo(), j.hi(), j.decoration()),
            (2.0, 3.0, Decoration::Def)
        );
        // Integer RIGHT endpoint with lo below it: floor(lo) != floor(hi) -> Def.
        let r = di(2.25, 3.0).floor();
        assert_eq!(
            (r.lo(), r.hi(), r.decoration()),
            (2.0, 3.0, Decoration::Def)
        );
        // Integer singleton: constant restriction -> Com (D8: inari says
        // Dac here, charging the ambient discontinuity; see semantics-diffs).
        let s = di(3.0, 3.0).floor();
        assert_eq!(
            (s.lo(), s.hi(), s.decoration()),
            (3.0, 3.0, Decoration::Com)
        );
        // Negative side exactness.
        let n = di(-2.5, -2.25).floor();
        assert_eq!((n.lo(), n.hi()), (-3.0, -3.0));
    }

    #[test]
    fn hull_keeps_min_dec_d7_pinned() {
        // D7 (semantics-diffs.md): hull deliberately keeps min(dec),
        // where 1788/inari would give Trv. Pin the divergence.
        let a = di(1.0, 2.0);
        let mut b = di(5.0, 6.0);
        b.dec = Decoration::Dac;
        let h = a.hull(b);
        assert_eq!((h.lo(), h.hi()), (1.0, 6.0));
        assert_eq!(h.decoration(), Decoration::Dac, "min(dec), NOT Trv (D7)");
        // Empty operands are ignored; NaI poisons.
        let he = a.hull(DInterval::empty());
        assert_eq!(
            (he.lo(), he.hi(), he.decoration()),
            (1.0, 2.0, Decoration::Com)
        );
        assert!(DInterval::nai().hull(a).is_nai());
    }

    #[test]
    fn intersection_trv_cap_and_taxonomy() {
        let a = di(1.0, 4.0);
        let i = a.intersection(di(3.0, 9.0));
        assert_eq!((i.lo(), i.hi()), (3.0, 4.0));
        assert_eq!(
            i.decoration(),
            Decoration::Trv,
            "set op capped at Trv (1788)"
        );
        assert!(a.intersection(di(5.0, 6.0)).is_empty(), "disjoint -> empty");
        assert!(a.intersection(DInterval::empty()).is_empty());
        assert!(a.intersection(DInterval::nai()).is_nai());
        // Touching at a point is a singleton, not empty.
        let t = a.intersection(di(4.0, 7.0));
        assert_eq!((t.lo(), t.hi()), (4.0, 4.0));
    }
}
