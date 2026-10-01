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
//! an open placement question, and moving it would not settle it.

use crate::interval::{DInterval, Decoration};

impl DInterval {
    /// `|x|`: exact (endpoint selection only). Each endpoint is
    /// `f64::abs` of one of the operand's, so a zero endpoint is `+0`,
    /// as `f64::abs` gives it.
    pub fn abs(self) -> Self {
        if let Some(p) = Self::propagate1(&self) {
            return p;
        }
        let (lo, hi) = if self.lo >= 0.0 {
            (self.lo.abs(), self.hi.abs())
        } else if self.hi <= 0.0 {
            (self.hi.abs(), self.lo.abs())
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
            lower_of(self.lo, rhs.lo, f64::min),
            upper_of(self.hi, rhs.hi, f64::min),
            self.dec.min(rhs.dec),
        )
    }

    /// Pointwise maximum `max(x, y)`: exact.
    pub fn max_i(self, rhs: Self) -> Self {
        if let Some(p) = Self::propagate2(&self, &rhs) {
            return p;
        }
        Self::make(
            lower_of(self.lo, rhs.lo, f64::max),
            upper_of(self.hi, rhs.hi, f64::max),
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
            lower_of(self.lo, rhs.lo, f64::min),
            upper_of(self.hi, rhs.hi, f64::max),
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
        let lo = lower_of(self.lo, rhs.lo, f64::max);
        let hi = upper_of(self.hi, rhs.hi, f64::min);
        if lo > hi {
            return Self::empty();
        }
        Self::make(lo, hi, Decoration::Trv)
    }
}

/// A lower endpoint chosen between `x` and `y` by `pick` (`f64::min` or
/// `f64::max`), except that two zeros give `-0` in either order. `pick`
/// leaves the sign of that choice unspecified, so its own answer is a
/// fact about the codegen rather than the source (crate docs, "Signed
/// zeros").
pub(crate) fn lower_of(x: f64, y: f64, pick: fn(f64, f64) -> f64) -> f64 {
    if x == 0.0 && y == 0.0 {
        if x.is_sign_negative() { x } else { y }
    } else {
        pick(x, y)
    }
}

/// [`lower_of`] for an upper endpoint: two zeros give `+0`.
pub(crate) fn upper_of(x: f64, y: f64, pick: fn(f64, f64) -> f64) -> f64 {
    if x == 0.0 && y == 0.0 {
        if x.is_sign_positive() { x } else { y }
    } else {
        pick(x, y)
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

    /// The zero-sign rule (crate docs, "Signed zeros"): a choice between
    /// zeros of opposite sign gives `-0` below and `+0` above. Every
    /// choice is made in both candidate orders, so a door that lets
    /// `f64::min`/`max` decide reds on one of them whichever operand the
    /// codegen favours.
    #[test]
    fn a_choice_between_zeros_is_minus_below_and_plus_above_in_either_order() {
        let (minus, plus) = ((-0.0f64).to_bits(), 0.0f64.to_bits());
        let (lo_m, lo_p) = (di(-0.0, 1.0), di(0.0, 1.0));
        let (hi_m, hi_p) = (di(-1.0, -0.0), di(-1.0, 0.0));
        for (a, b) in [(lo_m, lo_p), (lo_p, lo_m)] {
            for (op, r) in [
                ("hull", a.hull(b)),
                ("intersection", a.intersection(b)),
                ("min_i", a.min_i(b)),
                ("max_i", a.max_i(b)),
            ] {
                assert_eq!(r.lo().to_bits(), minus, "{op}({a:?}, {b:?}) lower endpoint");
            }
        }
        for (a, b) in [(hi_m, hi_p), (hi_p, hi_m)] {
            for (op, r) in [
                ("hull", a.hull(b)),
                ("intersection", a.intersection(b)),
                ("min_i", a.min_i(b)),
                ("max_i", a.max_i(b)),
            ] {
                assert_eq!(r.hi().to_bits(), plus, "{op}({a:?}, {b:?}) upper endpoint");
            }
        }

        // `×` and `÷` fold four corner bounds. A zero factor or numerator
        // gives an upper corner of `+0`; a product or quotient of
        // `-2^-1074`, inexact by the witness's floor, pads up to `-0`.
        // The operand orders put the `+0` corners first, last and
        // interleaved; which of them an unruled fold gets wrong differs
        // between debug and release, so both kinds are here.
        let tiny = di(0.0, 2f64.powi(-600));
        let neg = di(-(2f64.powi(-474)), -(2f64.powi(-474)));
        let (sub, k) = (di(-5e-324, -0.0), di(1.3, 1.3));
        for (case, r) in [
            ("[-2^-1074, -0] × [1.3]", sub * k),
            ("[1.3] × [-2^-1074, -0]", k * sub),
            ("[-2^-1074, -0] ÷ [1.3]", sub / k),
            ("[0, 2^-600] × [-2^-474]", tiny * neg),
            ("[-2^-474] × [0, 2^-600]", neg * tiny),
            (
                "[0, 2^-600] ÷ [-2^474]",
                tiny / di(-(2f64.powi(474)), -(2f64.powi(474))),
            ),
            (
                "[-2^-600, 0] ÷ [2^474]",
                di(-(2f64.powi(-600)), 0.0) / di(2f64.powi(474), 2f64.powi(474)),
            ),
        ] {
            assert_eq!(
                r.hi().to_bits(),
                plus,
                "{case}: upper endpoint {:e}",
                r.hi()
            );
        }
    }

    /// `abs` gives each endpoint as `f64::abs` of an operand endpoint, so
    /// a zero comes out `+0` from either sign and from either arm.
    #[test]
    fn abs_of_a_zero_endpoint_is_plus_zero() {
        let plus = 0.0f64.to_bits();
        for x in [di(-0.0, -0.0), di(-0.0, 2.0), di(-3.0, 0.0), di(-3.0, -0.0)] {
            assert_eq!(x.abs().lo().to_bits(), plus, "abs({x:?}) lower endpoint");
        }
        let both = di(-0.0, -0.0).abs();
        assert_eq!(both.hi().to_bits(), plus, "abs([-0, -0]) upper endpoint");
    }
}
