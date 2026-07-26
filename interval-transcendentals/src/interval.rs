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
        let dec = if lo.is_finite() && hi.is_finite() {
            Decoration::Com
        } else {
            Decoration::Dac
        };
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

    pub fn decoration(&self) -> Decoration {
        self.dec
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
