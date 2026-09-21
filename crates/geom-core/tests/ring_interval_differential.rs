//! **Differential lane: the C9 ring against the repo's two other
//! interval implementations** (M5 PR 2, acceptance family 2).
//!
//! The exact-arithmetic fuzz (`ring_interval_fuzz.rs`) proves the ring
//! *sound*. This file proves it is not soundly useless: on every shared
//! operation its enclosure **contains** the enclosure produced by an
//! independently written, independently certified interval library, so
//! the ring is a conservative widening of a known-good answer rather
//! than a differently-shaped one — and, because the two types carry
//! their refusal in different channels, that on every shared operation
//! the two **refuse the same inputs**.
//!
//! Two oracles, both dev-only:
//!
//! - [`interval_transcendentals::DInterval`] — the in-house rigorous
//!   unit (issue #115), a **dev-dependency** of `geom-core`. It is
//!   libm-only, so it adds nothing to the kernel's runtime dependency
//!   graph (`cargo tree -p geom-core -e normal` is unchanged: `libm`).
//!   It carries decorations and division exactness witnesses — precisely
//!   the tightness craft the ring declines to duplicate — which makes it
//!   the right upper reference for "how much conservatism did we buy?".
//! - `geom_core::Interval`, the certification scalar. The type
//!   compiles in every build; the `interval` cargo feature gates the
//!   lane-trait impls above this crate and the interval test files,
//!   which is why this lane — a test file — carries
//!   `#[cfg(feature = "interval")]` and its sibling does not. Since
//!   M5 PR 1 its backend is
//!   `interval-transcendentals` too — so this lane and the one above now
//!   share an arithmetic core, and it is the *scalar wrapper* (poison
//!   convention, `Real` lifting, `powi` routing) that is differentially
//!   compared, not a second independent library. The independent check on
//!   the arithmetic itself is the exact-comparator fuzz.
//!
//! # What is asserted, and the asymmetry that is not
//!
//! For `+ − × ÷` and negation the ring **contains** the oracle, with one
//! characterised exception: the ring's algebraic rules (the zero
//! annihilator `[0,0]·x = [0,0]`, and the sign clamp that keeps a
//! provably-nonpositive product's upper bound at exactly `0` where a
//! widened round-to-nearest reports `+5e-324`) can pull an endpoint to
//! *exactly zero*. Tightening to anything else fails the test.
//!
//! For `powi` and `sqr` only **overlap** is asserted: the two libraries
//! run structurally different multiplication chains, so neither
//! dominates — the ring is observed up to one step tighter on some
//! inputs and wider on others. Their soundness there is settled by the
//! exact-arithmetic fuzz, not by this comparison.
//!
//! `sqr` is compared against the oracle's `powi(2)`, which is the same
//! operation: the oracle has no separate squaring entry point, and its
//! `powi` carries the same tight even-power rule (lower bound exactly
//! `0.0` on a zero-straddling input) that `sqr` exists to give the ring.
//! It is an overlap comparison and not a containment one because the
//! oracle reaches the square through its binary-exponentiation chain,
//! which pads the base squaring and the accumulator multiply
//! separately: on a subnormal input `[-5e-324, 5e-324]` the ring's one
//! pad gives `[0, 5e-324]` and the oracle's two give `[0, 1e-323]`, so
//! the ring is the tighter of the two without any algebraic rule
//! firing.
//!
//! Where either side poisons or empties, the **endpoint** comparison is
//! counted and skipped rather than asserted: the two libraries make
//! different — both honest — choices about extended division and
//! overflow. The **verdict** comparison below runs first and never
//! skips, so those are exactly the cases it carries alone.
//!
//! Each lane prints the **conservatism it measured**: the maximum
//! endpoint disagreement, in representable steps, over the dominating
//! ops with finite endpoints. One step is the design target (one outward
//! pad per operation); the observed value is in the test output.
//!
//! # The verdict comparison, and its closed allowlist
//!
//! The two types put their refusal in different places. The ring has two
//! states and no decoration channel: its refusal is a NaN pair, read by
//! `RingInterval::is_poison`. `DInterval` keeps sound endpoints and
//! degrades a decoration, so its refusal is `dec < Def` (NaI and the
//! empty set sit at `Ill` and `Trv`, both below it); the `Interval`
//! scalar spells the same threshold `!Interval::is_certified()`.
//! Containing endpoints therefore says nothing about whether the two
//! arithmetics agree on *when they refuse*, which is the property every
//! consumer that branches on `is_poison()` rests on.
//!
//! So, per operation and **before** the endpoint comparison, each lane
//! asserts `ring.is_poison() == oracle refuses`, over
//! `+ − × ÷ neg sqr powi` — `powi` at eleven exponents covering both
//! signs and both chain shapes ([`EXPONENTS`]: the powers of two whose
//! chain only squares, the mixed-bit ones whose chain also multiplies,
//! and negative exponents of each). Every disagreement must fall in
//! [`ALLOWLIST`], a closed list of characterised classes, each
//! recognised by a predicate on the operation's **inputs**, each
//! naming the DIRECTION it runs in, and each counted and printed beside
//! the number of comparisons its predicate holds of. A disagreement
//! matching no class — or matching one that runs the other way — fails
//! the lane and prints `fuzz::replay()`.
//!
//! **Two mechanisms, four classes.** Classes 1–3 are one fact: the ring
//! poisons on an indeterminate IEEE corner (`0 · ±inf` in `×`,
//! `±inf / ±inf` in `÷`, and `0 · ±inf` once more at a multiply inside
//! `powi`'s chain) that reaches its NaN-propagating corner reduction,
//! where the backend resolves the same corner by convention —
//! `mul_lo`/`mul_hi` answer `0` for a zero operand, and the
//! `f64::min`/`f64::max` fold drops a NaN corner — and keeps a
//! decoration of `Dac`.
//!
//! Class 4 runs the other way, and is a pad count rather than a corner.
//! For a NEGATIVE exponent both types divide by the positive power, and
//! the backend's `pow_pos` seeds its accumulator at `[1, 1]` and
//! multiplies where the ring seeds at the first set bit — one outward
//! pad more. Below the normal floor a pad is an absolute `5e-324`
//! rather than a relative step, so that extra pad is the whole
//! difference between a positive power that touches zero (the backend:
//! the reciprocal divides by a zero-touching bracket and is `Trv`) and
//! one that does not (the ring: a one-signed divisor, and the quotient
//! certifies). `[5e-324, 1].powi(-1)` is the smallest witness — ring
//! `[0.999…, inf]`, backend `[1, inf]` at `Trv`.
//!
//! So the direction is a per-class fact rather than a property of the
//! whole allowlist: classes 1–3 are the ring poisoning where the
//! backend certifies, class 4 the backend refusing where the ring
//! certifies, and [`Class::direction`] makes each of those executable.
//! **No production call site reaches class 4**: every ring exponent in
//! `crates/*/src` is positive — `2`…`5`, in `props/quad.rs`'s
//! quadrature weights and `offset_fit.rs`'s `w.powi(3)`.
//!
//! Two shapes a reader expects here and will not find, each with its
//! own row in `crates/geom-core/tests/ring0_review_probes.rs`:
//!
//! - **`point(±inf)`** is not a class. Both types refuse it —
//!   `RingInterval::point` is poison and `DInterval::point` is NaI — so
//!   it is an agreement.
//! - **`inf − inf`** is unreachable. Both constructors refuse a bracket
//!   whose closed side sits at infinity, so `lo < +inf` and `hi > -inf`
//!   hold of every operand, and the ring's `finish` and the backend's
//!   `make` preserve both; no sum or difference of two valid brackets
//!   forms the indeterminate difference.
//!
//! **Overflow is a third such shape for `+ − × ÷`** — an overflowed
//! corner saturates to `±inf` in both types and both report the honest
//! unbounded side — **and is not one for `powi`**, where an overflow
//! inside the chain supplies the `inf` of class 3 and an underflow to
//! the subnormal floor is class 4.
//!
//! **Division agrees wherever it refuses for the reason the ring was
//! specified to refuse**: a divisor that straddles or touches zero is
//! ring poison and is `Trv` in the backend (the empty set, for `[0,0]`).
//! That agreement is counted in its own right rather than merely left
//! unrefuted — `div-touching-zero` in the report line, witnessed
//! deterministically by the corner-corpus test.
//!
//! The corpus reaches the classes: a quarter of the fuzz rounds draw
//! both endpoints from [`CORNERS`] (signed zeros and infinities,
//! subnormals, and magnitudes that overflow under `×` and `powi`), and
//! [`verdict_allowlist_is_closed_over_the_corner_corpus`] runs the same
//! verdict check exhaustively over every bracket pair that corpus forms,
//! with no randomness at all — the witness this lane's anti-vacuity
//! claim rests on, written down rather than hunted for.

test_utils::gated_to![
    "crates/geom-core/src/ring_interval.rs",
    "crates/geom-core/src/interval.rs",
    "interval-transcendentals/src/",
];

use geom_core::RingInterval;
use interval_transcendentals::{DInterval, Decoration};
use test_utils::fuzz;

fn finite(rng: &mut fuzz::Rng) -> f64 {
    loop {
        let x = f64::from_bits(rng.next_u64());
        if x.is_finite() {
            return x;
        }
    }
}

/// A finite value in a moderate exponent window — the regime
/// certification actually runs in, where neither library is distracted
/// by overflow.
fn moderate(rng: &mut fuzz::Rng) -> f64 {
    let m = rng.next_u64() & 0xf_ffff_ffff_ffff;
    let e = (1023 + (rng.next_u64() % 121) as i32 - 60) as u64;
    let s = rng.next_u64() & (1 << 63);
    f64::from_bits(s | (e << 52) | m)
}

/// The adversarial endpoint corpus: every value from which an
/// indeterminate corner, an overflow or an underflow-to-zero can be
/// built. Signed zeros and infinities (the `0 · inf` and `inf / inf`
/// corners), both subnormal extremes and `MIN_POSITIVE` (whose squares
/// underflow to zero), and magnitudes whose squares and cubes overflow.
const CORNERS: [f64; 16] = [
    f64::NEG_INFINITY,
    -f64::MAX,
    -1e300,
    -1.0,
    -f64::MIN_POSITIVE,
    -5e-324,
    -0.0,
    0.0,
    5e-324,
    f64::MIN_POSITIVE,
    1e-160,
    1.0,
    1e160,
    1e300,
    f64::MAX,
    f64::INFINITY,
];

/// Which corpus a round draws from. Moderate keeps half the rounds — it
/// is the window the conservatism measurement is meaningful in — the
/// full-range and adversarial corpora take a quarter each.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Regime {
    Moderate,
    FullRange,
    Adversarial,
}

fn regime(round: usize) -> Regime {
    match round % 4 {
        0 | 2 => Regime::Moderate,
        1 => Regime::FullRange,
        _ => Regime::Adversarial,
    }
}

fn ordered(rng: &mut fuzz::Rng, regime: Regime) -> Ends {
    let (a, b) = match regime {
        Regime::Moderate => (moderate(rng), moderate(rng)),
        Regime::FullRange => (finite(rng), finite(rng)),
        Regime::Adversarial => (
            CORNERS[rng.below(CORNERS.len())],
            CORNERS[rng.below(CORNERS.len())],
        ),
    };
    if a <= b {
        Ends::new(a, b)
    } else {
        Ends::new(b, a)
    }
}

/// Rounds per lane at EFFORT 1. The lane's own "ran too thin" floor is
/// derived from this, so cutting depth can never silently gut the check.
fn rounds() -> usize {
    fuzz::scaled(37_500)
}

/// The monotone integer key of an `f64` (IEEE total order restricted to
/// non-NaN): `key(next_up(x)) == key(x) + 1` everywhere, `±0` share the
/// key `0`. Lets endpoint disagreement be measured in **representable
/// steps** rather than in a relative error that would be meaningless in
/// the subnormal range.
fn ord_key(x: f64) -> i64 {
    let b = x.to_bits() as i64;
    if b < 0 { i64::MIN.wrapping_sub(b) } else { b }
}

/// Distance between two non-NaN `f64`s in representable steps.
fn steps(a: f64, b: f64) -> u64 {
    ord_key(a).abs_diff(ord_key(b))
}

// -------------------------------------------------- inputs and ops

/// The endpoint pair handed to BOTH constructors. The allowlist reads
/// these — never the results — so a class is a statement about which
/// inputs the two arithmetics disagree on, which is what a consumer can
/// check of its own call sites.
#[derive(Clone, Copy)]
struct Ends {
    lo: f64,
    hi: f64,
}

impl Ends {
    fn new(lo: f64, hi: f64) -> Self {
        Self { lo, hi }
    }

    /// This pair as the ring builds it. The chain model reads the ring
    /// through its own public ops rather than restating its pads.
    fn ring(self) -> RingInterval {
        RingInterval::from_bounds(self.lo, self.hi)
    }

    /// The endpoints of a ring value, or `None` for poison.
    fn of(r: RingInterval) -> Option<Self> {
        (!r.is_poison()).then(|| Self::new(r.lo(), r.hi()))
    }

    /// The exact zero enclosure, which both `Mul` rules answer with
    /// `[0,0]` before any corner is formed. Pinned against the ring's
    /// own annihilator by
    /// [`the_exact_zero_annihilator_is_the_rings_own`].
    fn is_exact_zero(self) -> bool {
        self.lo == 0.0 && self.hi == 0.0
    }

    /// An endpoint that is exactly zero (either sign) — the left factor
    /// of a `0 · inf` corner.
    fn has_zero_endpoint(self) -> bool {
        self.lo == 0.0 || self.hi == 0.0
    }

    /// An infinite endpoint — the right factor of a `0 · inf` corner and
    /// both factors of an `inf / inf` one.
    fn is_unbounded(self) -> bool {
        self.lo.is_infinite() || self.hi.is_infinite()
    }

    fn spans_zero(self) -> bool {
        self.lo <= 0.0 && self.hi >= 0.0
    }

    /// The `0 · ±inf` corner, asked of a pair the ring is about to
    /// multiply: an exactly-zero endpoint on one side, an infinite one
    /// on the other, and neither side the exact zero the annihilator
    /// answers first. Class 1 asks it of the operands; class 3 asks it
    /// of [`powi_chain`]'s multiply, one level in.
    fn times_forms_an_indeterminate_corner(self, other: Self) -> bool {
        !self.is_exact_zero()
            && !other.is_exact_zero()
            && ((self.has_zero_endpoint() && other.is_unbounded())
                || (other.has_zero_endpoint() && self.is_unbounded()))
    }

    /// Whether `self^|n|`, as the RING computes it, is one-signed but
    /// sits within the chain's pad count of zero — class 4's input
    /// predicate.
    ///
    /// The backend reaches the same power through `pow_pos`, which
    /// seeds its accumulator at `[1, 1]` and multiplies where the ring
    /// seeds at the first set bit: one outward pad more, and the rest
    /// of the two chains pad step for step. Below the normal floor a
    /// pad is an absolute `5e-324`, so the two powers' smaller
    /// endpoints can differ by at most one step per chain operation —
    /// which is the bound here, and is what separates a backend power
    /// that touches zero (reciprocal `Trv`) from a ring power that
    /// does not (reciprocal certified).
    fn positive_power_is_within_a_pad_of_zero(self, n: i32) -> bool {
        let (power, corner) = powi_chain(self, n);
        let Some(p) = power else { return false };
        if corner || !p.strictly_one_signed() {
            return false;
        }
        let squarings = (i32::BITS - 1) - (n.unsigned_abs() | 1).leading_zeros();
        let pads = f64::from(squarings + n.unsigned_abs().count_ones() + 1);
        p.lo.abs().min(p.hi.abs()) <= pads * f64::from_bits(1)
    }

    fn strictly_one_signed(self) -> bool {
        self.lo > 0.0 || self.hi < 0.0
    }

    /// Whether both constructors accept this pair. They refuse the same
    /// set — a NaN endpoint, an inverted bracket, or a closed side at
    /// infinity — so an input either is a bracket in both types or is
    /// refused by both.
    fn is_a_bracket(self) -> bool {
        self.lo <= self.hi && self.lo != f64::INFINITY && self.hi != f64::NEG_INFINITY
    }
}

/// [`RingInterval::powi`]'s chain for `|n|`, walked with the ring's OWN
/// `sqr` and `Mul`: ascending bit order, the accumulator squared after
/// each bit, the first set bit seeding the result directly. Returns the
/// positive power and whether any MULTIPLY in the chain formed the
/// `0 · ±inf` corner.
///
/// That multiply is the ring's only way to poison a power: `sqr` pairs
/// a bracket with itself, whose four corners are two squares, so it
/// cannot form the indeterminate product.
///
/// No pad rule is restated here — only the association is, and
/// [`the_chain_model_reproduces_powi`] reds if the ring changes it.
fn powi_chain(base: Ends, n: i32) -> (Option<Ends>, bool) {
    let b = base.ring();
    let mut result = b;
    let mut seeded = false;
    let mut acc = b;
    let mut e = n.unsigned_abs();
    let mut corner = false;
    while e > 0 {
        if e & 1 == 1 {
            if seeded {
                if let (Some(x), Some(y)) = (Ends::of(result), Ends::of(acc)) {
                    corner |= x.times_forms_an_indeterminate_corner(y);
                }
                result = result * acc;
            } else {
                result = acc;
                seeded = true;
            }
        }
        e >>= 1;
        if e > 0 {
            acc = acc.sqr();
        }
    }
    (Ends::of(result), corner)
}

impl core::fmt::Display for Ends {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{:e}, {:e}]", self.lo, self.hi)
    }
}

/// The exponents every `powi` comparison runs at. Both signs, and both
/// chain shapes at each: a power of two only squares (`2`, and `-2`),
/// a mixed-bit exponent also multiplies (`3 5 6 7 31`, and `-3`), `±1`
/// is the bare base and its bare reciprocal, and `0` is the constant.
/// Classes 3 and 4 live on opposite signs of this list, so sampling one
/// sign hides one of them.
const EXPONENTS: [i32; 11] = [-3, -2, -1, 0, 1, 2, 3, 5, 6, 7, 31];

/// The operations both types share. `Sqr` is the ring's `sqr` against
/// the oracle's `powi(2)`; `Powi` carries its exponent because the
/// allowlist's `powi` classes read it.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Sqr,
    Powi(i32),
}

impl Op {
    /// One tally slot per operation; `powi` shares one across exponents.
    const SLOTS: usize = 7;
    const SLOT_NAMES: [&'static str; Self::SLOTS] =
        ["add", "sub", "mul", "div", "neg", "sqr", "powi"];

    fn slot(self) -> usize {
        match self {
            Op::Add => 0,
            Op::Sub => 1,
            Op::Mul => 2,
            Op::Div => 3,
            Op::Neg => 4,
            Op::Sqr => 5,
            Op::Powi(_) => 6,
        }
    }

    /// Whether the ring is expected to CONTAIN the oracle's bracket.
    /// False for the two ops whose multiplication chains differ
    /// structurally from the ring's, `powi` and `sqr` (see the module
    /// docs).
    fn dominates(self) -> bool {
        !matches!(self, Op::Powi(_) | Op::Sqr)
    }

    /// Whether the second operand is meaningful — the unary ops carry a
    /// copy of the first, and printing it would invent an argument.
    fn is_binary(self) -> bool {
        matches!(self, Op::Add | Op::Sub | Op::Mul | Op::Div)
    }
}

impl core::fmt::Display for Op {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Op::Powi(n) => write!(f, "powi({n})"),
            other => f.write_str(Op::SLOT_NAMES[other.slot()]),
        }
    }
}

// ------------------------------------------------------- the allowlist

/// Which way a verdict disagreement runs.
///
/// The module's claim about direction is a per-class fact, and this is
/// what makes it executable: a class excuses a disagreement only in
/// the direction it names, so a backend that started refusing where
/// class 1, 2 or 3 holds would fail the lane rather than be waved
/// through by a predicate that happens to hold of its inputs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    /// The ring poisons where the backend certifies.
    RingPoisons,
    /// The backend refuses where the ring certifies.
    BackendRefuses,
}

impl Direction {
    fn of(ring_poisons: bool) -> Self {
        if ring_poisons {
            Self::RingPoisons
        } else {
            Self::BackendRefuses
        }
    }
}

/// One characterised class of verdict disagreement, recognised by a
/// predicate over the operation and its input brackets and running in
/// one named direction.
struct Class {
    name: &'static str,
    direction: Direction,
    holds: fn(Op, Ends, Ends) -> bool,
}

/// The closed allowlist. A verdict disagreement outside it — or inside
/// a class but running the other way — fails the lane; see the module
/// docs for the two mechanisms behind the four, and for the classes
/// that are NOT here.
const CLASSES: usize = 4;
const ALLOWLIST: [Class; CLASSES] = [
    // `a · b` where one bracket has an exactly-zero endpoint and the
    // other an infinite one, and neither is the exact zero the
    // annihilator answers first: the `0 · ±inf` corner is NaN, which the
    // ring's corner reduction propagates to poison, while `mul_lo`/
    // `mul_hi` answer `0` for a zero operand and the backend returns a
    // `Dac` bracket.
    Class {
        name: "mul-zero-times-infinite",
        direction: Direction::RingPoisons,
        holds: |op, a, b| op == Op::Mul && a.times_forms_an_indeterminate_corner(b),
    },
    // `a / b` with both brackets unbounded and the divisor proven
    // one-signed (a divisor touching zero is refused by both): the
    // `±inf / ±inf` corner is NaN, which the ring propagates and the
    // backend's `f64::min`/`f64::max` fold silently drops, leaving a
    // sound `Dac` bracket built from the remaining corners.
    Class {
        name: "div-infinite-over-infinite",
        direction: Direction::RingPoisons,
        holds: |op, a, b| {
            op == Op::Div && a.is_unbounded() && b.is_unbounded() && b.strictly_one_signed()
        },
    },
    // `a.powi(n)`, `n > 0`, whose chain performs a MULTIPLY on a pair
    // that forms the `0 · ±inf` corner above — class 1 one level in.
    // The chain is walked with the ring's own ops, so this holds of
    // exactly the inputs whose positive power the ring poisons: the
    // exponents that only square (`1`, `2`, and every other power of
    // two) never reach it, and a finite magnitude that overflows under
    // squaring supplies the `inf` without the base being unbounded
    // (`[-MAX, -0.0].powi(3)` is the smallest witness).
    Class {
        name: "powi-chain-forms-the-zero-times-infinite-corner",
        direction: Direction::RingPoisons,
        holds: |op, a, _b| match op {
            Op::Powi(n) => n > 0 && powi_chain(a, n).1,
            _ => false,
        },
    },
    // `a.powi(n)`, `n < 0`, on a base whose positive power the ring
    // keeps one-signed but within the chain's pad count of zero. The
    // backend's `pow_pos` carries one outward pad more, which below the
    // normal floor is the whole value: its positive power touches zero,
    // its reciprocal divides by a zero-touching bracket and is `Trv`,
    // and the ring's reciprocal certifies. The one class that runs
    // backend-refuses; `[5e-324, 1].powi(-1)` is the smallest witness.
    Class {
        name: "powi-negative-reciprocal-of-a-power-a-pad-from-zero",
        direction: Direction::BackendRefuses,
        holds: |op, a, _b| match op {
            Op::Powi(n) => n < 0 && a.positive_power_is_within_a_pad_of_zero(n),
            _ => false,
        },
    },
];

// ----------------------------------------------------------- oracles

/// The backend a lane compares the ring against: an interval type whose
/// refusal is a decoration read rather than a NaN pair.
trait Oracle: Copy {
    /// The lane's label in the report, and the name its seed is drawn
    /// under.
    const LABEL: &'static str;
    const SEED_NAME: &'static str;

    fn from_bounds(lo: f64, hi: f64) -> Self;
    /// This backend's refusal: `dec < Def`, however it spells it.
    fn refuses(self) -> bool;
    /// The bracket to compare endpoints against, or `None` where the
    /// backend has no endpoints (NaI, the empty set).
    fn bracket(self) -> Option<(f64, f64)>;
    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;
    fn neg(self) -> Self;
    fn powi(self, n: i32) -> Self;
}

impl Oracle for DInterval {
    const LABEL: &'static str = "DInterval";
    const SEED_NAME: &'static str = "ring_interval_differential::dinterval";

    fn from_bounds(lo: f64, hi: f64) -> Self {
        DInterval::from_bounds(lo, hi)
    }

    fn refuses(self) -> bool {
        self.is_nai() || self.is_empty() || self.decoration() < Decoration::Def
    }

    fn bracket(self) -> Option<(f64, f64)> {
        (!self.is_nai() && !self.is_empty()).then(|| (self.lo(), self.hi()))
    }

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }

    fn mul(self, rhs: Self) -> Self {
        self * rhs
    }

    fn div(self, rhs: Self) -> Self {
        self / rhs
    }

    fn neg(self) -> Self {
        -self
    }

    fn powi(self, n: i32) -> Self {
        DInterval::powi(self, n)
    }
}

#[cfg(feature = "interval")]
impl Oracle for geom_core::Interval {
    const LABEL: &'static str = "Interval scalar";
    const SEED_NAME: &'static str = "ring_interval_differential::interval_scalar";

    fn from_bounds(lo: f64, hi: f64) -> Self {
        geom_core::Interval::from_bounds(lo, hi)
    }

    fn refuses(self) -> bool {
        !self.is_certified()
    }

    fn bracket(self) -> Option<(f64, f64)> {
        use geom_core::Bounds;
        // The scalar's `Bounds` door is deliberately decoration-blind and
        // reports NaN for NaI and the empty set, which `check_mode`
        // already counts as a skip — so this lane hands every result on
        // and lets the one guard do the filtering.
        Some((Bounds::lo(self), Bounds::hi(self)))
    }

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }

    fn mul(self, rhs: Self) -> Self {
        self * rhs
    }

    fn div(self, rhs: Self) -> Self {
        self / rhs
    }

    fn neg(self) -> Self {
        -self
    }

    fn powi(self, n: i32) -> Self {
        geom_core::Real::powi(self, n)
    }
}

// ------------------------------------------------------------- tally

/// Outcome of one comparison, for honest reporting.
#[derive(Default)]
struct Tally {
    wider: u64,
    identical: u64,
    tighter: u64,
    skipped: u64,
    max_lo_steps: u64,
    max_hi_steps: u64,
    /// Verdict comparisons per operation slot — the anti-vacuity number
    /// for the verdict half, and how a dropped row shows up.
    verdicts: [u64; Op::SLOTS],
    verdicts_agreed: u64,
    /// Verdict disagreements per [`ALLOWLIST`] class.
    allowed: [u64; CLASSES],
    /// Verdict comparisons whose INPUTS each class's predicate holds
    /// of, disagreeing or not. Printed beside `allowed`, because the
    /// gap between them is the slack in the predicate — the region
    /// where a second mechanism could hide behind a class that already
    /// excuses this direction, and the only place the lane measures
    /// it.
    holds: [u64; CLASSES],
    /// Divisions both types refuse because the divisor is not proven
    /// away from zero — the agreement the ring's `Div` doc claims.
    div_touching_zero_agreed: u64,
}

impl Tally {
    /// The verdict comparison: does the ring poison exactly where the
    /// backend refuses? Runs before the endpoint comparison and never
    /// skips.
    ///
    /// A disagreement is permitted only by a named [`ALLOWLIST`] class
    /// whose predicate holds of the INPUTS. Anything else is a new fact
    /// about the two arithmetics and fails here.
    fn verdict(&mut self, op: Op, a: Ends, b: Ends, ring: RingInterval, oracle_refuses: bool) {
        self.verdicts[op.slot()] += 1;
        let holds: [bool; CLASSES] = core::array::from_fn(|i| (ALLOWLIST[i].holds)(op, a, b));
        for (slot, held) in self.holds.iter_mut().zip(holds) {
            *slot += u64::from(held);
        }
        if ring.is_poison() == oracle_refuses {
            self.verdicts_agreed += 1;
            if op == Op::Div
                && oracle_refuses
                && a.is_a_bracket()
                && b.is_a_bracket()
                && b.spans_zero()
            {
                self.div_touching_zero_agreed += 1;
            }
            return;
        }
        let observed = Direction::of(ring.is_poison());
        let class = (0..CLASSES).find(|&i| holds[i] && ALLOWLIST[i].direction == observed);
        assert!(
            class.is_some(),
            "{op}: ring {} but backend {} on a = {a}{} — no allowlist class {} — so this is a \
             new disagreement between the two arithmetics — {}",
            if ring.is_poison() {
                "poisons"
            } else {
                "certifies"
            },
            if oracle_refuses {
                "refuses"
            } else {
                "certifies"
            },
            if op.is_binary() {
                format!(", b = {b}")
            } else {
                String::new()
            },
            match (0..CLASSES).find(|&i| holds[i]) {
                Some(i) => format!(
                    "runs {observed:?} on these inputs ({} holds, but it runs {:?})",
                    ALLOWLIST[i].name, ALLOWLIST[i].direction
                ),
                None => "holds of these inputs".to_string(),
            },
            fuzz::replay()
        );
        if let Some(i) = class {
            self.allowed[i] += 1;
        }
    }

    /// Checks the ring bracket against an oracle bracket for the same
    /// operation on the same inputs.
    ///
    /// Both are sound enclosures of the same true set, so the assertions
    /// are (1) the two **overlap** — a disjoint pair would mean one of
    /// them excludes values the other proves possible — and (2) the ring
    /// **contains** the oracle, except where the ring's algebraic rules
    /// pull an endpoint to exactly zero (below).
    ///
    /// Deliberately *not* asserted: that the ring is always the wider of
    /// the two. It is tighter wherever its algebraic rules fire — the
    /// zero annihilator (`[0,0]·x = [0,0]`) and the sign clamp that
    /// keeps a provably-nonpositive product's upper bound at exactly `0`
    /// where a widened round-to-nearest would report `+5e-324`. Those
    /// are facts about the reals, not tightness claims about
    /// floating-point, and the `tighter` counter reports how often they
    /// fire.
    ///
    /// `dominates = false` — `powi` only — asserts only overlap. There
    /// the two libraries run **structurally different multiplication
    /// chains** (the ring seeds the accumulator at the first set bit;
    /// the oracle's chain differs), so neither result dominates the
    /// other by a rounding-step argument: the ring is observed up to a
    /// step tighter on some inputs and wider on others. Both are sound;
    /// the ring's soundness is settled by the exact-arithmetic fuzz, not
    /// by this comparison.
    fn check_mode(&mut self, r: RingInterval, olo: f64, ohi: f64, what: &str, dominates: bool) {
        if r.is_poison() || olo.is_nan() || ohi.is_nan() || olo > ohi {
            self.skipped += 1;
            return;
        }
        assert!(
            r.lo() <= ohi && olo <= r.hi(),
            "{what}: ring [{:e}, {:e}] is disjoint from oracle [{olo:e}, {ohi:e}] — {}",
            r.lo(),
            r.hi(),
            fuzz::replay()
        );
        // Containment, modulo the one characterised exception: the
        // ring's algebraic rules can only ever pull an endpoint to
        // *exactly* zero. Any other tightening would be a soundness
        // claim the ring has not earned, and fails here.
        //
        // The `== 0.0` clause is a TOLERANCE, not a soundness argument:
        // it permits exactly the sign clamp's and zero annihilator's
        // legitimate zero and nothing else. Soundness of that zero is
        // carried by `ring_interval_fuzz.rs`, which compares against
        // exact arithmetic and would catch a wrong zero here.
        assert!(
            !dominates || r.lo() <= olo || r.lo() == 0.0,
            "{what}: ring lo {:e} is above oracle lo {olo:e} without the zero rule — {}",
            r.lo(),
            fuzz::replay()
        );
        assert!(
            !dominates || r.hi() >= ohi || r.hi() == 0.0,
            "{what}: ring hi {:e} is below oracle hi {ohi:e} without the zero rule — {}",
            r.hi(),
            fuzz::replay()
        );
        // Conservatism, reported not asserted: how many representable
        // steps of extra width the ring bought, over finite endpoints
        // (an overflowed ring endpoint is infinite and unmeasurable).
        // Only the dominating ops are measured — for `powi` the two
        // chains can land in different binades, where a step count is
        // not a conservatism measure at all.
        if r.lo() == olo && r.hi() == ohi {
            self.identical += 1;
        } else if r.lo() <= olo && r.hi() >= ohi {
            self.wider += 1;
        } else {
            self.tighter += 1;
        }
        if !dominates {
            return;
        }
        if r.lo().is_finite() && olo.is_finite() {
            self.max_lo_steps = self.max_lo_steps.max(steps(r.lo(), olo));
        }
        if r.hi().is_finite() && ohi.is_finite() {
            self.max_hi_steps = self.max_hi_steps.max(steps(r.hi(), ohi));
        }
    }

    fn endpoint_comparisons(&self) -> u64 {
        self.wider + self.identical + self.tighter
    }

    fn report(&self, label: &str) {
        println!(
            "[{label}] {} wider, {} bit-identical, {} tighter (algebraic \
             rules), {} skipped; max endpoint disagreement {} / {} steps",
            self.wider,
            self.identical,
            self.tighter,
            self.skipped,
            self.max_lo_steps,
            self.max_hi_steps
        );
        let per_op: Vec<String> = Op::SLOT_NAMES
            .iter()
            .zip(self.verdicts)
            .map(|(name, n)| format!("{name} {n}"))
            .collect();
        let per_class: Vec<String> = ALLOWLIST
            .iter()
            .zip(self.allowed)
            .zip(self.holds)
            .map(|((c, n), h)| format!("{} {n} allowed / {h} holds", c.name))
            .collect();
        println!(
            "[{label}] verdicts by op: {}; {} agreed; allowlisted disagreements: {}; \
             div-touching-zero agreed-refusals {}",
            per_op.join(", "),
            self.verdicts_agreed,
            per_class.join(", "),
            self.div_touching_zero_agreed
        );
    }
}

// ------------------------------------------------------- the round

/// Every shared operation on one pair of input brackets: the verdict
/// comparison first, then the endpoint comparison the verdict does not
/// subsume. One body for all four lanes, so the two oracles and the two
/// corpora cannot drift apart.
fn compare_ops<O: Oracle>(t: &mut Tally, a: Ends, b: Ends) {
    let r = RingInterval::from_bounds(a.lo, a.hi);
    let s = RingInterval::from_bounds(b.lo, b.hi);
    let d = O::from_bounds(a.lo, a.hi);
    let e = O::from_bounds(b.lo, b.hi);
    let shared: [(Op, RingInterval, O); 6] = [
        (Op::Add, r + s, d.add(e)),
        (Op::Sub, r - s, d.sub(e)),
        (Op::Mul, r * s, d.mul(e)),
        (Op::Div, r / s, d.div(e)),
        (Op::Neg, -r, d.neg()),
        (Op::Sqr, r.sqr(), d.powi(2)),
    ];
    for (op, ring, oracle) in shared {
        // A unary op has no second operand, so it is handed its own
        // bracket rather than the round's other one: nothing may read an
        // argument the operation does not have.
        let rhs = if op.is_binary() { b } else { a };
        t.verdict(op, a, rhs, ring, oracle.refuses());
        match oracle.bracket() {
            Some((olo, ohi)) => {
                t.check_mode(ring, olo, ohi, Op::SLOT_NAMES[op.slot()], op.dominates())
            }
            None => t.skipped += 1,
        }
    }
    for n in EXPONENTS {
        let op = Op::Powi(n);
        let ring = r.powi(n);
        let oracle = d.powi(n);
        t.verdict(op, a, a, ring, oracle.refuses());
        match oracle.bracket() {
            Some((olo, ohi)) => t.check_mode(ring, olo, ohi, "powi", op.dominates()),
            None => t.skipped += 1,
        }
    }
}

/// One randomized lane: the three input regimes, every shared op.
fn fuzz_lane<O: Oracle>() {
    let mut rng = fuzz::start(O::SEED_NAME);
    let n = rounds();
    let mut t = Tally::default();
    for round in 0..n {
        let regime = regime(round);
        let a = ordered(&mut rng, regime);
        let b = ordered(&mut rng, regime);
        compare_ops::<O>(&mut t, a, b);
    }
    t.report(O::LABEL);
    // COVERAGE FLOOR, kept proportional to the round count. Each round
    // offers 17 endpoint comparisons (six shared ops and eleven
    // exponents) and the skip filters take about a fifth of them, so
    // the lane lands near 13 per round; a third of that is the floor.
    // It is a not-comparing-anything guard, not a coverage target —
    // the closure claim is the allowlist's, and the deterministic
    // corner sweep is where it cannot be starved.
    assert!(
        t.endpoint_comparisons() > 4 * n as u64,
        "lane ran too thin: {} endpoint comparisons over {n} rounds — {}",
        t.endpoint_comparisons(),
        fuzz::replay()
    );
    // The verdict half has its own floor, and it is per operation: a
    // verdict comparison never skips, so a zero here means a row was
    // dropped from `compare_ops` rather than that the corpus was
    // unlucky. `sqr` is the row this was written for.
    for (name, n) in Op::SLOT_NAMES.iter().zip(t.verdicts) {
        assert!(n > 0, "{name} compared no verdicts at all");
    }
}

#[test]
fn ring_contains_dinterval_on_every_shared_op() {
    fuzz_lane::<DInterval>();
}

#[cfg(feature = "interval")]
#[test]
fn ring_contains_the_interval_scalar_on_every_shared_op() {
    fuzz_lane::<geom_core::Interval>();
}

// ------------------------------------------------- the corner corpus

/// Every bracket the corner corpus forms: each unordered pair of
/// [`CORNERS`] values, in order. Includes the ones both constructors
/// refuse (`[+inf, +inf]` and friends) — those are agreements, and
/// leaving them out would be choosing the answer.
fn corner_brackets() -> Vec<Ends> {
    let mut out = Vec::new();
    for (i, &x) in CORNERS.iter().enumerate() {
        for &y in &CORNERS[i..] {
            out.push(if x <= y {
                Ends::new(x, y)
            } else {
                Ends::new(y, x)
            });
        }
    }
    out
}

/// The verdict property over the whole corner corpus, exhaustively and
/// deterministically, for one oracle. Returns the tally so the caller
/// can assert on what it reached.
fn corner_sweep<O: Oracle>() -> Tally {
    let mut t = Tally::default();
    let brackets = corner_brackets();
    for &a in &brackets {
        for &b in &brackets {
            compare_ops::<O>(&mut t, a, b);
        }
    }
    t.report(O::LABEL);
    t
}

/// The allowlist's anti-vacuity witness, and the deterministic half of
/// its pin.
///
/// The fuzz lanes above are a counterexample search: cutting their depth
/// can only lose detection power. This one is the other shape — *at
/// least one input of each class exists, and division's refusals agree*
/// — and it is written down rather than hunted for: the corner corpus is
/// small enough to sweep exhaustively, so every class count below is a
/// fact about the two arithmetics rather than a draw.
///
/// One body for both oracles: a second copy is how one of them ends up
/// with the weaker message.
fn assert_the_allowlist_is_closed_over_the_corner_corpus<O: Oracle>() {
    let t = corner_sweep::<O>();
    for (class, n) in ALLOWLIST.iter().zip(t.allowed) {
        assert!(
            n > 0,
            "[{}] {} matched nothing in the corner corpus: either the class is dead and the \
             allowlist should lose it, or the corpus no longer reaches it",
            O::LABEL,
            class.name
        );
    }
    assert!(
        t.div_touching_zero_agreed > 0,
        "[{}] the corner corpus formed no division by a zero-touching divisor, so the \
         agreement the ring's Div doc claims is untested here",
        O::LABEL
    );
}

#[test]
fn verdict_allowlist_is_closed_over_the_corner_corpus() {
    assert_the_allowlist_is_closed_over_the_corner_corpus::<DInterval>();
}

#[cfg(feature = "interval")]
#[test]
fn verdict_allowlist_is_closed_over_the_corner_corpus_at_the_interval_scalar() {
    assert_the_allowlist_is_closed_over_the_corner_corpus::<geom_core::Interval>();
}

// ------------------------------------- the ring rules the file copies

/// Pins the one ring rule [`powi_chain`] restates — `powi`'s
/// association — against the ring's own `powi`, over the corner corpus
/// at every exponent the lanes run.
///
/// The chain model exists so classes 3 and 4 can read the multiply
/// step, which `powi` does not expose; nothing else ties the two
/// together, so a change to the association or to the seeding rule
/// would silently move the classes. It reds here instead.
#[test]
fn the_chain_model_reproduces_powi() {
    for a in corner_brackets() {
        for n in EXPONENTS {
            if n == 0 {
                continue;
            }
            let m = n.unsigned_abs() as i32;
            let (modelled, corner) = powi_chain(a, n);
            let actual = a.ring().powi(m);
            let observed = Ends::of(actual);
            assert_eq!(
                modelled.is_some(),
                observed.is_some(),
                "powi({m}) on {a}: the chain model and the ring disagree on whether the \
                 power poisons"
            );
            if let (Some(x), Some(y)) = (modelled, observed) {
                assert!(
                    x.lo.to_bits() == y.lo.to_bits() && x.hi.to_bits() == y.hi.to_bits(),
                    "powi({m}) on {a}: the chain model gives {x}, the ring gives {y}"
                );
            }
            assert_eq!(
                corner,
                actual.is_poison() && !a.ring().is_poison(),
                "powi({m}) on {a}: the chain model's corner flag and the ring's poison disagree"
            );
        }
    }
}

/// Pins the annihilator [`Ends::is_exact_zero`] names: the ring answers
/// `[0,0] · x` with `[0,0]` before any corner is formed, which is why
/// class 1 excludes an exactly-zero operand rather than counting it as
/// half a `0 · inf` pair.
#[test]
fn the_exact_zero_annihilator_is_the_rings_own() {
    let zero = Ends::new(0.0, 0.0);
    assert!(zero.is_exact_zero());
    for b in corner_brackets() {
        if !b.is_a_bracket() {
            continue;
        }
        let p = zero.ring() * b.ring();
        assert!(
            !p.is_poison() && p.lo() == 0.0 && p.hi() == 0.0,
            "[0,0] * {b} is {} — the annihilator class 1 excludes is gone",
            if p.is_poison() {
                "poison".to_string()
            } else {
                format!("[{:e}, {:e}]", p.lo(), p.hi())
            }
        );
    }
}
