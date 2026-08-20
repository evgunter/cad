//! **Differential lane: the C9 ring against the repo's two other
//! interval implementations** (M5 PR 2, acceptance family 2).
//!
//! The exact-arithmetic fuzz (`ring_interval_fuzz.rs`) proves the ring
//! *sound*. This file proves it is not soundly useless: on every shared
//! operation its enclosure **contains** the enclosure produced by an
//! independently written, independently certified interval library, so
//! the ring is a conservative widening of a known-good answer rather
//! than a differently-shaped one.
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
//! - `geom_core::Interval`, the `interval`-feature evaluation scalar,
//!   behind `#[cfg(feature = "interval")]`. Since M5 PR 1 its backend is
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
//! For `powi` only **overlap** is asserted: the two libraries run
//! structurally different multiplication chains, so neither dominates —
//! the ring is observed up to one step tighter on some inputs and wider
//! on others. Its soundness there is settled by the exact-arithmetic
//! fuzz, not by this comparison.
//!
//! Where either side poisons or empties, the case is counted and skipped
//! rather than asserted: the two libraries make different — both
//! honest — choices about extended division and overflow.
//!
//! Each lane prints the **conservatism it measured**: the maximum
//! endpoint disagreement, in representable steps, over the dominating
//! ops with finite endpoints. One step is the design target (one outward
//! pad per operation); the observed value is in the test output.

use geom_core::RingInterval;
use interval_transcendentals::DInterval;
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

fn ordered(rng: &mut fuzz::Rng, moderate_window: bool) -> (f64, f64) {
    let (a, b) = if moderate_window {
        (moderate(rng), moderate(rng))
    } else {
        (finite(rng), finite(rng))
    };
    if a <= b { (a, b) } else { (b, a) }
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

/// Outcome of one comparison, for honest reporting.
#[derive(Default)]
struct Tally {
    wider: u64,
    identical: u64,
    tighter: u64,
    skipped: u64,
    max_lo_steps: u64,
    max_hi_steps: u64,
}

impl Tally {
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
    fn check(&mut self, r: RingInterval, olo: f64, ohi: f64, what: &str) {
        self.check_mode(r, olo, ohi, what, true);
    }

    /// `dominates = false` asserts only overlap. Used for `powi`, where
    /// the two libraries run **structurally different multiplication
    /// chains** (the ring seeds the accumulator at the first set bit;
    /// the oracle's chain differs), so neither result dominates the
    /// other by a rounding-step argument — the ring is observed up to a
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
    }
}

#[test]
fn ring_contains_dinterval_on_every_shared_op() {
    let mut rng = fuzz::start("ring_interval_differential::dinterval");
    let n = rounds();
    let mut t = Tally::default();
    for round in 0..n {
        let moderate_window = round % 2 == 0;
        let (alo, ahi) = ordered(&mut rng, moderate_window);
        let (blo, bhi) = ordered(&mut rng, moderate_window);
        let r = RingInterval::from_bounds(alo, ahi);
        let s = RingInterval::from_bounds(blo, bhi);
        let d = DInterval::from_bounds(alo, ahi);
        let e = DInterval::from_bounds(blo, bhi);
        let pairs: [(RingInterval, DInterval, &str); 5] = [
            (r + s, d + e, "add"),
            (r - s, d - e, "sub"),
            (r * s, d * e, "mul"),
            (r / s, d / e, "div"),
            (-r, -d, "neg"),
        ];
        for (ring, oracle, what) in pairs {
            if oracle.is_nai() || oracle.is_empty() {
                t.skipped += 1;
                continue;
            }
            t.check(ring, oracle.lo(), oracle.hi(), what);
        }
        for n in [-3i32, 0, 1, 2, 3, 5] {
            let oracle = d.powi(n);
            if oracle.is_nai() || oracle.is_empty() {
                t.skipped += 1;
                continue;
            }
            t.check_mode(r.powi(n), oracle.lo(), oracle.hi(), "powi", false);
        }
    }
    t.report("DInterval");
    // COVERAGE FLOOR, kept proportional to the round count: each round
    // offers 11 comparisons and historically ~30% survived the skip
    // filters (1M verdicts from 300k rounds). Three per round is that
    // ratio with headroom; below it the lane is not comparing anything.
    assert!(
        t.wider + t.identical + t.tighter > 3 * n as u64,
        "lane ran too thin: {} verdicts over {n} rounds — {}",
        t.wider + t.identical + t.tighter,
        fuzz::replay()
    );
}

#[cfg(feature = "interval")]
#[test]
fn ring_contains_the_interval_scalar_on_every_shared_op() {
    use geom_core::{Bounds, Interval};

    let mut rng = fuzz::start("ring_interval_differential::interval_scalar");
    let n = rounds();
    let mut t = Tally::default();
    for round in 0..n {
        let moderate_window = round % 2 == 0;
        let (alo, ahi) = ordered(&mut rng, moderate_window);
        let (blo, bhi) = ordered(&mut rng, moderate_window);
        let r = RingInterval::from_bounds(alo, ahi);
        let s = RingInterval::from_bounds(blo, bhi);
        let d = Interval::from_bounds(alo, ahi);
        let e = Interval::from_bounds(blo, bhi);
        let pairs: [(RingInterval, Interval, &str); 5] = [
            (r + s, d + e, "add"),
            (r - s, d - e, "sub"),
            (r * s, d * e, "mul"),
            (r / s, d / e, "div"),
            (-r, -d, "neg"),
        ];
        for (ring, oracle, what) in pairs {
            t.check(ring, Bounds::lo(oracle), Bounds::hi(oracle), what);
        }
        for n in [-3i32, 0, 1, 2, 3, 5] {
            let oracle = geom_core::Real::powi(d, n);
            t.check_mode(
                r.powi(n),
                Bounds::lo(oracle),
                Bounds::hi(oracle),
                "powi",
                false,
            );
        }
    }
    t.report("Interval scalar");
    // COVERAGE FLOOR, proportional to the round count (see the sibling
    // lane's note).
    assert!(
        t.wider + t.identical + t.tighter > 3 * n as u64,
        "lane ran too thin: {} verdicts over {n} rounds — {}",
        t.wider + t.identical + t.tighter,
        fuzz::replay()
    );
}
