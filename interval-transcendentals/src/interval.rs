//! The decorated interval type: an enclosure `[lo, hi]` of a true real
//! value plus an IEEE 1788 decoration that acts as the poison channel
//! (the kernel's contract: decoration below `Def` never decides topology;
//! see `crates/geom-core/src/interval.rs` module docs on `main`).
//!
//! Decoration discipline mirrors 1788: a returned decoration is a *lower
//! bound* on the true property level — returning a weaker decoration than
//! the tightest correct one is always sound (1788 §8; it only ever
//! poisons more, never less). This crate exploits that in exactly the
//! places where a conservative containment test cannot be sure (see
//! `docs/semantics-diffs.md`).

/// IEEE 1788 decorations, ordered `Ill < Trv < Def < Dac < Com`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Decoration {
    /// Not an interval (NaI): ill-formed construction, permanent poison.
    Ill,
    /// Trivial: nothing is asserted (domain was violated somewhere, or
    /// the interval is empty).
    Trv,
    /// The function is defined on the whole input box.
    Def,
    /// Defined and continuous on the whole input box.
    Dac,
    /// Common: defined, continuous, and bounded on a bounded input box.
    Com,
}

impl Decoration {
    /// The decoration a TOTAL, CONTINUOUS function earns on an input box:
    /// `Com` when the box is bounded, `Dac` when it is not — `Com` is
    /// definitionally unavailable on an unbounded box (1788 §8, and the
    /// [`DInterval`] type invariant).
    ///
    /// **Every site in the crate that CHOOSES between the two asks
    /// this**: `sin`, `cos`, `atan`, `atan2`, `pow_pos`, and
    /// [`DInterval::from_bounds`], where a freshly built interval earns
    /// its own decoration the same way. The other `Dac` mentions in this
    /// file are not that choice and are deliberately not routed here —
    /// [`DInterval::entire`] names one specific value, and
    /// [`DInterval::make`] CAPS a decoration the caller already chose.
    /// Operations that are not total (`sqrt`, `asin`, `acos`, `tan`)
    /// choose from their domain test instead.
    pub(crate) fn continuous_on(bounded: bool) -> Self {
        if bounded { Self::Com } else { Self::Dac }
    }
}

/// A decorated interval: `[lo, hi]` encloses the true real value of the
/// computation; `dec` is the poison channel.
///
/// Invariants (enforced by constructors, relied on everywhere):
/// - NaI:   `lo`/`hi` NaN and `dec == Ill`;
/// - Empty: `lo`/`hi` NaN and `dec == Trv`;
/// - else:  `lo <= hi`, neither is NaN, `lo < +inf`, `hi > -inf`; `dec`
///   may be any level `>= Trv` (a nonempty interval can carry `Trv` from
///   an upstream domain clamp), and is `<= Dac` whenever a bound is
///   infinite (`Com` requires boundedness).
///
/// No `PartialEq`, deliberately (same rationale as the kernel scalar:
/// NaI is not equal to itself and enclosure equality is not a geometric
/// question).
#[derive(Debug, Clone, Copy)]
pub struct DInterval {
    pub(crate) lo: f64,
    pub(crate) hi: f64,
    pub(crate) dec: Decoration,
}

impl DInterval {
    /// Not-an-Interval: the ill-formed poison value.
    pub fn nai() -> Self {
        Self {
            lo: f64::NAN,
            hi: f64::NAN,
            dec: Decoration::Ill,
        }
    }

    /// The empty set (decoration `Trv`).
    pub fn empty() -> Self {
        Self {
            lo: f64::NAN,
            hi: f64::NAN,
            dec: Decoration::Trv,
        }
    }

    /// The whole real line (decoration `Dac`: unbounded, so never `Com`).
    pub fn entire() -> Self {
        Self {
            lo: f64::NEG_INFINITY,
            hi: f64::INFINITY,
            dec: Decoration::Dac,
        }
    }

    /// Singleton enclosure of a finite `x`; NaN and ±inf are not real
    /// numbers and have no enclosure — they construct NaI, loudly poison
    /// from the start (kernel `from_f64` contract).
    pub fn point(x: f64) -> Self {
        if x.is_finite() {
            Self {
                lo: x,
                hi: x,
                dec: Decoration::Com,
            }
        } else {
            Self::nai()
        }
    }

    /// Enclosure from explicit bounds. Infinite endpoints of the correct
    /// sign are allowed (unbounded enclosures, decoration capped at
    /// `Dac`); NaN endpoints, inverted bounds, or an interval with no
    /// real member (`[+inf, +inf]`, `[-inf, -inf]`) construct NaI.
    pub fn from_bounds(lo: f64, hi: f64) -> Self {
        if lo.is_nan() || hi.is_nan() || lo > hi || lo == f64::INFINITY || hi == f64::NEG_INFINITY {
            return Self::nai();
        }
        let dec = Decoration::continuous_on(lo.is_finite() && hi.is_finite());
        Self { lo, hi, dec }
    }

    /// Internal: nonempty result with bounds and decoration; caps the
    /// decoration at `Dac` if a bound is infinite (`Com` requires bounded)
    /// and normalizes any accidental NaN bound to loud NaI.
    pub(crate) fn make(lo: f64, hi: f64, dec: Decoration) -> Self {
        if lo.is_nan() || hi.is_nan() || lo > hi {
            // Fail loud: an internal op produced non-interval bounds.
            debug_assert!(false, "make() got invalid bounds {lo} {hi}");
            return Self::nai();
        }
        let dec = if lo.is_finite() && hi.is_finite() {
            dec
        } else {
            dec.min(Decoration::Dac)
        };
        Self { lo, hi, dec }
    }

    pub fn is_nai(&self) -> bool {
        self.dec == Decoration::Ill
    }

    /// Empty set? (NaN bounds with a non-Ill decoration.)
    pub fn is_empty(&self) -> bool {
        self.lo.is_nan() && self.dec != Decoration::Ill
    }

    /// Lower bound (NaN for empty/NaI).
    pub fn lo(&self) -> f64 {
        self.lo
    }

    /// Upper bound (NaN for empty/NaI).
    pub fn hi(&self) -> f64 {
        self.hi
    }

    /// Both endpoints finite. Only meaningful on a nonempty, non-NaI
    /// interval (NaN bounds answer `false`), which is where every caller
    /// asks it: after [`Self::propagate1`]/[`Self::propagate2`].
    pub(crate) fn is_bounded(&self) -> bool {
        self.lo.is_finite() && self.hi.is_finite()
    }

    pub fn decoration(&self) -> Decoration {
        self.dec
    }

    /// Lowers the decoration to `min(self.decoration(), cap)`, keeping the
    /// bounds untouched — the "honesty floor" primitive consumers need
    /// when a *value* was selected or hulled by comparing other values
    /// whose trustworthiness must bound the result's (the kernel scalar's
    /// kink selectors are the motivating consumer).
    ///
    /// Only ever lowers, so poison is never laundered. Poison shapes are
    /// preserved: NaI stays NaI, Empty stays Empty (its `Trv` is already
    /// the minimum a non-NaI value can carry), and capping a normal
    /// interval all the way to `Ill` yields NaI — `Ill` means "not an
    /// interval", which is a shape, not a label.
    pub fn with_dec_capped(self, cap: Decoration) -> Self {
        if self.is_nai() {
            return Self::nai();
        }
        let dec = self.dec.min(cap);
        if dec == Decoration::Ill {
            return Self::nai();
        }
        if self.is_empty() {
            return Self::empty();
        }
        Self::make(self.lo, self.hi, dec)
    }

    /// Does the enclosure contain the real number `x`? (`false` for
    /// empty/NaI: they contain no real.)
    pub fn contains(&self, x: f64) -> bool {
        !self.is_nai() && !self.is_empty() && self.lo <= x && x <= self.hi
    }

    /// Propagation guard: if either operand is NaI the result is NaI; if
    /// either is empty the result is empty (with the min decoration,
    /// i.e. `Trv`). Returns `None` when normal evaluation should proceed.
    pub(crate) fn propagate2(a: &Self, b: &Self) -> Option<Self> {
        if a.is_nai() || b.is_nai() {
            return Some(Self::nai());
        }
        if a.is_empty() || b.is_empty() {
            return Some(Self::empty());
        }
        None
    }

    /// Unary flavor of [`Self::propagate2`].
    pub(crate) fn propagate1(a: &Self) -> Option<Self> {
        if a.is_nai() {
            return Some(Self::nai());
        }
        if a.is_empty() {
            return Some(Self::empty());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_dec_capped_only_lowers_and_keeps_bounds() {
        let x = DInterval::from_bounds(1.0, 2.0);
        assert_eq!(x.decoration(), Decoration::Com);
        let capped = x.with_dec_capped(Decoration::Trv);
        assert_eq!((capped.lo(), capped.hi()), (1.0, 2.0), "bounds untouched");
        assert_eq!(capped.decoration(), Decoration::Trv);
        // A cap ABOVE the current level is a no-op (min, never a raise).
        let raised = capped.with_dec_capped(Decoration::Com);
        assert_eq!(raised.decoration(), Decoration::Trv, "never launders");
    }

    #[test]
    fn with_dec_capped_preserves_poison_shapes() {
        assert!(DInterval::nai().with_dec_capped(Decoration::Com).is_nai());
        let e = DInterval::empty().with_dec_capped(Decoration::Com);
        assert!(e.is_empty() && !e.is_nai());
        assert_eq!(e.decoration(), Decoration::Trv);
        // Capping to `Ill` is the NaI shape, not a labelled interval.
        assert!(
            DInterval::from_bounds(1.0, 2.0)
                .with_dec_capped(Decoration::Ill)
                .is_nai()
        );
    }

    #[test]
    fn with_dec_capped_respects_the_unbounded_com_rule() {
        // `Com` requires boundedness: an unbounded enclosure stays `Dac`
        // even when the cap would allow more.
        let half = DInterval::from_bounds(0.0, f64::INFINITY);
        assert_eq!(half.decoration(), Decoration::Dac);
        assert_eq!(
            half.with_dec_capped(Decoration::Com).decoration(),
            Decoration::Dac
        );
    }
}
