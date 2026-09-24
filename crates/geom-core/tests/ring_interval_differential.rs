//! **Differential lane: the C9 ring against the arithmetic it wraps**
//! (M5 PR 2, acceptance family 2).
//!
//! The exact-arithmetic fuzz (`ring_interval_fuzz.rs`) proves the ring
//! *sound*. This file proves the newtype is a **faithful forwarding**:
//! on every shared operation the ring's bracket is bit-identical to the
//! backend's and the two refuse exactly the same inputs. A ring
//! operation that stopped forwarding — a pad applied on top, an
//! algebraic rule re-introduced, an association of its own — reds here,
//! whatever it did to the endpoints.
//!
//! Two oracles, both dev-only:
//!
//! - [`interval_transcendentals::DInterval`] — the rigorous unit
//!   (issue #115) the ring is a newtype over, and a **dev-dependency**
//!   of `geom-core` for this lane as well as a normal one for the
//!   kernel. It carries decorations and exactness witnesses, and the
//!   ring's surface is a re-spelling of them for certification code.
//! - `geom_core::Interval`, the certification scalar, over the same
//!   backend. The type compiles in every build; the `interval` cargo
//!   feature gates the lane-trait impls above this crate and the
//!   interval test files, which is why this lane — a test file —
//!   carries `#[cfg(feature = "interval")]` and its sibling does not.
//!   What the second lane adds is the *scalar wrapper* (poison
//!   convention, `Real` lifting, `powi` routing): it must reach the
//!   same bracket the ring does, through a different surface.
//!
//! # What is asserted
//!
//! Per operation over `+ − × ÷ neg sqr powi` — `powi` at eleven
//! exponents covering both signs and both chain shapes ([`EXPONENTS`])
//! — and **before** any endpoint comparison, each lane asserts
//! `ring.is_poison() == oracle refuses`. The two put their refusal in
//! the same channel now (`dec < Def`, spelled
//! `!Interval::is_certified()` at the scalar), so the claim is exact
//! agreement with no exceptions. Then the endpoints: **bit-identical,
//! both ends**, `sqr` against the oracle's `powi(2)`, which is the
//! same call.
//!
//! Where the backend has no endpoints to report — NaI and the empty
//! set, whose bounds are NaN — the endpoint comparison is counted and
//! skipped; the verdict comparison runs first and never skips, so
//! those cases are carried by it alone.
//!
//! Each lane prints the conservatism it measured, in representable
//! steps. It is **zero** by construction now and asserted so: the ring
//! buys no conservatism over the backend because it *is* the backend.
//!
//! # The allowlist that used to be here
//!
//! Until RING-2 the ring was a second arithmetic — one unconditional
//! outward ulp per operation, a sign clamp, a zero annihilator, and a
//! NaN-propagating corner reduction — and this lane carried a closed
//! allowlist of four characterised classes where the two disagreed on
//! the *verdict*: the ring poisoning on an indeterminate IEEE corner
//! (`0 · ±inf` under `×`, `±inf / ±inf` under `÷`, and `0 · ±inf` at a
//! multiply inside `powi`'s chain) that the backend resolves by
//! convention, and, running the other way, a negative `powi` whose
//! positive power the backend pads one step further into zero. **All
//! four collapse to nothing**, because there is no second arithmetic
//! left to disagree: the ring reaches those corners through the
//! backend's own `mul_lo`/`mul_hi` and `pow_pos`. The record of what
//! each class was, with its measured counts, is RING-0's (PR 2993);
//! the corners themselves are still swept here and are now
//! agreements, and `ring0_review_probes.rs` writes the interesting
//! ones down by hand.
//!
//! **Division agrees wherever it refuses for the reason the ring was
//! specified to refuse**: a divisor that straddles or touches zero is
//! `Trv` in the backend (the empty set, for `[0,0]`), which is the
//! ring's poison. That agreement is counted in its own right rather
//! than merely left unrefuted — `div-touching-zero` in the report
//! line, witnessed deterministically by the corner-corpus test.
//!
//! The corpus reaches the corners: a quarter of the fuzz rounds draw
//! both endpoints from [`CORNERS`] (signed zeros and infinities,
//! subnormals, and magnitudes that overflow under `×` and `powi`), and
//! [`verdicts_and_endpoints_agree_over_the_corner_corpus`] runs the
//! same comparison exhaustively over every bracket pair that corpus
//! forms, with no randomness at all.

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
///
/// Two sibling lists of this shape exist and are deliberately not
/// shared: `ring_interval_fuzz.rs`'s `edges` (signed zeros and exact
/// dyadics, for an exactness comparison that has no use for an
/// infinity) and `interval-transcendentals`' `review_fuzz_exact.rs`
/// `EDGE_MAGNITUDES` (which adds the 2Prod witness floor, and sits in
/// another workspace). Each is chosen for the property its lane
/// asserts; adding a value here is a reason to read the other two, not
/// a reason to assume they follow.
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

/// The endpoint pair handed to BOTH constructors.
#[derive(Clone, Copy)]
struct Ends {
    lo: f64,
    hi: f64,
}

impl Ends {
    fn new(lo: f64, hi: f64) -> Self {
        Self { lo, hi }
    }

    fn spans_zero(self) -> bool {
        self.lo <= 0.0 && self.hi >= 0.0
    }

    /// Whether both constructors accept this pair. They refuse the same
    /// set — a NaN endpoint, an inverted bracket, or a closed side at
    /// infinity — because the ring's constructor IS the backend's, so
    /// an input either is a bracket in both types or is refused by both.
    fn is_a_bracket(self) -> bool {
        self.lo <= self.hi && self.lo != f64::INFINITY && self.hi != f64::NEG_INFINITY
    }
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
/// A negative exponent is a division, so it is the one shape that can
/// mint a refusal out of a certified base; sampling one sign hides it.
const EXPONENTS: [i32; 11] = [-3, -2, -1, 0, 1, 2, 3, 5, 6, 7, 31];

/// The operations both types share. `Sqr` is the ring's `sqr` against
/// the oracle's `powi(2)` — the same call, which is what the ring's
/// `sqr` forwards to; `Powi` carries its exponent for the message.
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

// ----------------------------------------------------------- oracles

/// The arithmetic a lane compares the ring against: the backend, and
/// the scalar wrapper over the same backend.
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
        // reports NaN for NaI and the empty set, which `check_ends`
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
    identical: u64,
    differing: u64,
    skipped: u64,
    max_lo_steps: u64,
    max_hi_steps: u64,
    /// Verdict comparisons per operation slot — the anti-vacuity number
    /// for the verdict half, and how a dropped row shows up.
    verdicts: [u64; Op::SLOTS],
    verdicts_agreed: u64,
    /// Divisions both types refuse because the divisor is not proven
    /// away from zero — the agreement the ring's `Div` doc claims.
    div_touching_zero_agreed: u64,
}

impl Tally {
    /// The verdict comparison: does the ring poison exactly where the
    /// backend refuses? Runs before the endpoint comparison and never
    /// skips.
    ///
    /// There is no allowlist and no exception. The ring's poison is the
    /// backend's `dec < Def` read through one accessor, so a
    /// disagreement means an operation stopped forwarding.
    fn verdict(&mut self, op: Op, a: Ends, b: Ends, ring: RingInterval, oracle_refuses: bool) {
        self.verdicts[op.slot()] += 1;
        assert_eq!(
            ring.is_poison(),
            oracle_refuses,
            "{op}: ring {} but backend {} on a = {a}{} — the ring's poison IS the backend's \
             decoration, so this operation is no longer forwarding — {}",
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
            fuzz::replay()
        );
        self.verdicts_agreed += 1;
        if op == Op::Div && oracle_refuses && a.is_a_bracket() && b.is_a_bracket() && b.spans_zero()
        {
            self.div_touching_zero_agreed += 1;
        }
    }

    /// Checks the ring bracket against the oracle bracket for the same
    /// operation on the same inputs: **bit-identical, both ends**.
    ///
    /// Containment would be the weaker claim, and it was the right one
    /// while the ring ran its own arithmetic. It does not any more, so
    /// anything other than the same bits is a pad, a clamp or an
    /// association the newtype has grown — and each of those is the
    /// second arithmetic this type was retired for being.
    ///
    /// `-0.0` and `0.0` are compared by bits deliberately: the sign of
    /// a zero endpoint is part of what the backend returns, and a
    /// forwarding that flipped it would be a change to the value the
    /// ring hands its consumers.
    fn check_ends(&mut self, r: RingInterval, olo: f64, ohi: f64, what: &str) {
        if r.is_poison() || olo.is_nan() || ohi.is_nan() {
            self.skipped += 1;
            return;
        }
        if r.lo().to_bits() == olo.to_bits() && r.hi().to_bits() == ohi.to_bits() {
            self.identical += 1;
        } else {
            self.differing += 1;
        }
        if r.lo().is_finite() && olo.is_finite() {
            self.max_lo_steps = self.max_lo_steps.max(steps(r.lo(), olo));
        }
        if r.hi().is_finite() && ohi.is_finite() {
            self.max_hi_steps = self.max_hi_steps.max(steps(r.hi(), ohi));
        }
        assert!(
            self.differing == 0,
            "{what}: ring [{:e}, {:e}] is not the backend's [{olo:e}, {ohi:e}] — {}",
            r.lo(),
            r.hi(),
            fuzz::replay()
        );
    }

    fn endpoint_comparisons(&self) -> u64 {
        self.identical + self.differing
    }

    fn report(&self, label: &str) {
        println!(
            "[{label}] {} bit-identical, {} differing, {} skipped; max endpoint \
             disagreement {} / {} steps",
            self.identical, self.differing, self.skipped, self.max_lo_steps, self.max_hi_steps
        );
        let per_op: Vec<String> = Op::SLOT_NAMES
            .iter()
            .zip(self.verdicts)
            .map(|(name, n)| format!("{name} {n}"))
            .collect();
        println!(
            "[{label}] verdicts by op: {}; {} agreed, 0 disagreements permitted; \
             div-touching-zero agreed-refusals {}",
            per_op.join(", "),
            self.verdicts_agreed,
            self.div_touching_zero_agreed
        );
    }

    /// The conservatism claim, asserted rather than printed: the
    /// newtype buys none, because it is the backend.
    fn assert_no_conservatism(&self, label: &str) {
        assert_eq!(
            (self.differing, self.max_lo_steps, self.max_hi_steps),
            (0, 0, 0),
            "[{label}] the ring's endpoints moved off the backend's"
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
            Some((olo, ohi)) => t.check_ends(ring, olo, ohi, Op::SLOT_NAMES[op.slot()]),
            None => t.skipped += 1,
        }
    }
    for n in EXPONENTS {
        let op = Op::Powi(n);
        let ring = r.powi(n);
        let oracle = d.powi(n);
        t.verdict(op, a, a, ring, oracle.refuses());
        match oracle.bracket() {
            Some((olo, ohi)) => t.check_ends(ring, olo, ohi, "powi"),
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
    t.assert_no_conservatism(O::LABEL);
    // COVERAGE FLOOR, kept proportional to the round count. Each round
    // offers 17 endpoint comparisons (six shared ops and eleven
    // exponents) and the skip filters take about a fifth of them, so
    // the lane lands near 13 per round; a third of that is the floor.
    // It is a not-comparing-anything guard, not a coverage target —
    // the deterministic corner sweep is where the claim cannot be
    // starved.
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
fn ring_forwards_dinterval_on_every_shared_op() {
    fuzz_lane::<DInterval>();
}

#[cfg(feature = "interval")]
#[test]
fn ring_forwards_the_interval_scalar_on_every_shared_op() {
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

/// The verdict and endpoint properties over the whole corner corpus,
/// exhaustively and deterministically, for one oracle.
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

/// The forwarding claim's anti-vacuity witness.
///
/// The fuzz lanes above are a counterexample search: cutting their
/// depth can only lose detection power. This one is the other shape —
/// *every indeterminate IEEE corner the old ring poisoned on is swept,
/// and every one of them now agrees* — and it is written down rather
/// than hunted for: the corner corpus is small enough to sweep
/// exhaustively, so the result is a fact about the arithmetic rather
/// than a draw.
///
/// One body for both oracles: a second copy is how one of them ends up
/// with the weaker message.
fn assert_the_corner_corpus_agrees<O: Oracle>() {
    let t = corner_sweep::<O>();
    t.assert_no_conservatism(O::LABEL);
    assert!(
        t.endpoint_comparisons() > 0,
        "[{}] the corner sweep compared no endpoints at all",
        O::LABEL
    );
    assert!(
        t.div_touching_zero_agreed > 0,
        "[{}] the corner corpus formed no division by a zero-touching divisor, so the \
         agreement the ring's Div doc claims is untested here",
        O::LABEL
    );
}

/// **The one place the newtype is LOOSER than the ring it replaced**,
/// pinned so the sentence is executable.
///
/// Every corpus in the tree moves tighter or not at all, and that is
/// what the PR's "zero looser" counts. It is a claim about the
/// corpora, not about the arithmetic: at the subnormal floor and at
/// the overflow ceiling the backend gives width back, because the
/// retired ring carried two rules the backend does not — a sign clamp
/// that pulled a same-signed product's far end to exactly `0`, and a
/// symmetric one-step pad that could not straddle the subnormal
/// boundary. Four corners, each one step:
///
/// * `[MIN_POSITIVE]² ` — the retired ring answered `[0, 5e-324]`, the
///   newtype `[-5e-324, 5e-324]`: one subnormal step at the LOW end,
///   the clamp's. `sqr` and `powi(4)` give the step back at the HIGH
///   end instead (`[0, 1e-323]` against `[0, 5e-324]`), which is the
///   backend's two-step pad below the 2Prod floor.
/// * `[MIN_POSITIVE, 1e-160].powi(-1)` — `4.494232837155792e307`
///   against the retired `…791e307`: one ordinary ulp at the top of
///   the range.
///
/// Neither direction is a soundness question — a wider enclosure still
/// encloses — and neither reaches a corpus: the consumer that noticed
/// the clamp at all is
/// `review_m5_pr2_scratch::review_scratch_ring_lanes`'s
/// `lane_boundary_pool`, which asserts containment against exact
/// arithmetic instead of the clamp's own rule. The retired ring's side
/// of each number is measured against a verbatim port of it in
/// `ring2_r2_probes::the_sign_clamps_one_subnormal_step_is_the_only_direction_the_newtype_gives_back`;
/// what is pinned here is the newtype's own answer, which is what a
/// future change to the backend would move.
#[test]
fn the_subnormal_and_overflow_corners_are_where_the_newtype_gives_width_back() {
    let t = f64::MIN_POSITIVE;
    let tiny = RingInterval::from_bounds(t, t);

    let product = tiny * tiny;
    assert!(!product.is_poison(), "{product:?}");
    assert_eq!(
        (product.lo(), product.hi()),
        (-5e-324, 5e-324),
        "the sign clamp is gone: the low end is one subnormal step below the retired \
         ring's exact 0"
    );

    for (what, got) in [("sqr", tiny.sqr()), ("powi(4)", tiny.powi(4))] {
        assert!(!got.is_poison(), "{what}: {got:?}");
        assert_eq!(
            (got.lo(), got.hi()),
            (0.0, 1e-323),
            "{what}: the retired ring answered [0, 5e-324] — one subnormal step \
             tighter at the top"
        );
    }

    let recip = RingInterval::from_bounds(t, 1e-160).powi(-1);
    assert!(!recip.is_poison(), "{recip:?}");
    assert_eq!(
        (recip.lo(), recip.hi()),
        (9.999_999_999_999_999e159, 4.494_232_837_155_792e307),
        "the retired ring's top end was 4.494232837155791e307 — one ulp tighter"
    );
}

#[test]
fn verdicts_and_endpoints_agree_over_the_corner_corpus() {
    assert_the_corner_corpus_agrees::<DInterval>();
}

#[cfg(feature = "interval")]
#[test]
fn verdicts_and_endpoints_agree_over_the_corner_corpus_at_the_interval_scalar() {
    assert_the_corner_corpus_agrees::<geom_core::Interval>();
}
