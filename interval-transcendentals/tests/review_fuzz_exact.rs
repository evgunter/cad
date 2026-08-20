//! **Exact-rational fuzz of the exactness witnesses** of `÷`, `×` and
//! `sqrt` — the three operations that share one magnitude-gated 2Prod
//! witness, which is the thing that can lie. (Not "every operation whose
//! truth is exactly computable": `+ −` truth is exactly computable too,
//! and they are excluded for a different reason, given below.) The
//! division half was adopted verbatim
//! (bar the case counts) from the M5 PR 1 adversarial review's scratch
//! harness — an independent derivation, which is exactly its regression
//! value: it shares no code with the implementation it checks. The `×`
//! and `sqrt` halves are built on that same independent comparison
//! primitive.
//!
//! Two properties per operation. F1: whenever the result comes back
//! DEGENERATE (a point `[v, v]`, i.e. the exactness witness fired on
//! both endpoints), `v` must be the true value in exact rational
//! arithmetic — `q·b = a`, `v = a·b`, `v·v = a`. F2: the true value
//! always lies in `[lo, hi]`, checked by exact integer comparison and
//! never by f64 re-evaluation.
//!
//! **Why `×` and `sqrt` are here and `+ −` are not.** The containment
//! violation this class of test exists for was found in `mul_exact`
//! (`src/round.rs`, `docs/derivations.md` §3): a witness that LIED below
//! the 2Prod validity floor. The response at the time was this file, for
//! division only — the sibling of the site that had the bug, while `mul`
//! itself and `sqrt`, which share the identical witness, got nothing.
//! They have it now. Addition does not, and the reason is structural
//! rather than a deferral: TwoSum's error term is exactly representable
//! for ALL finite doubles with no underflow proviso (`derivations.md`
//! §1 **Lemma P0**), so `add_lo`/`add_hi` have no validity floor to get
//! wrong. What that buys is **F1** — a witness that cannot lie. It does
//! NOT buy F2, and F2 for `+ −` is covered only by `pad_contract.rs`'s
//! upper bound and by the oracle tier's `certify_arith`; mutating
//! `add_lo`/`add_hi` to bare round-to-nearest leaves the whole cheap
//! tier green, and only `certify_arith` reds. That is a real asymmetry
//! with `÷ × sqrt`, it is recorded rather than argued away, and the
//! reason it is not closed here is that the exact-rational comparator
//! this file is built on **cannot serve addition**: aligning `2^1023`
//! with `2^-1074` needs ~2100 bits and a u128 holds 128
//! (`crates/geom-core/tests/ring_interval_fuzz.rs` reaches the same
//! conclusion independently and generalises its own comparator to get
//! past it).
//!
//! Why it earns a place in CI: it needs **no oracle library**, so it runs
//! in the kernel's own pipeline with no C toolchain (README
//! "Certification", ci.yml's `interval-backend` job).
//!
//! # Depth and seed
//!
//! Counts are multiples of `test_utils::fuzz`'s EFFORT dial and the seed
//! VARIES per run — both logged unconditionally by `fuzz::start`, so a
//! red run always names the draw that produced it. The shipped level is
//! a smoke sweep of ~63k cases across ALL FOUR lanes (a cheaper sweep
//! that dropped a lane would lose the witness-floor and subnormal
//! coverage lanes 2-4 exist for). `CAD_FUZZ_EFFORT=280` restores the
//! full 17.5M-case sweep the adversarial review ran, which the dial
//! replaces — it used to be an `#[ignore]`d twin of this file's one
//! test:
//!
//! ```text
//! CAD_FUZZ_EFFORT=280 cargo test --test review_fuzz_exact -- --nocapture
//! ```
//!
//! Any run is bit-reproducible from the seed it logged:
//! `CAD_FUZZ_SEED=0x… cargo test --test review_fuzz_exact -- --nocapture`.
//!
//! # The witness floor is not part of the sweep
//!
//! The sweep counts degenerate (exact) results and asserts a floor on
//! them. Read as a coverage claim that would be the trap
//! `test_utils::fuzz`'s taxonomy names: a counterexample search with an
//! anti-vacuity floor bolted on, whose sample count then carries one
//! obligation that is safe to cut and one that is not. It is not that
//! here, because "a division that comes out exact" is concisely
//! constructible — shape 2, *a witness you can write down*. So it IS
//! written down, in
//! [`the_exactness_witness_fires_on_written_down_exact_divisions`], and
//! asserted every run at no cost and with no draw involved. What
//! survives inside the sweep is a pure ANTI-VACUITY floor on lane 2's
//! generator (does it still construct exact divisions at the rate it
//! was built to?), scaled from the same constants as the loops and left
//! with the ~33x margin it has always had.

use interval_transcendentals::DInterval;
use test_utils::fuzz;

/// Decompose a finite nonzero f64 into (negative, odd_mantissa u128, exp)
/// with value = sign * m * 2^e and m odd (trailing zeros stripped).
fn decomp(x: f64) -> (bool, u128, i32) {
    assert!(x.is_finite() && x != 0.0);
    let bits = x.to_bits();
    let neg = bits >> 63 == 1;
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & 0xf_ffff_ffff_ffff;
    let (mut m, mut e) = if biased == 0 {
        (frac as u128, -1074)
    } else {
        ((frac | (1 << 52)) as u128, biased - 1075)
    };
    let tz = m.trailing_zeros() as i32;
    m >>= tz;
    e += tz;
    (neg, m, e)
}

fn bitlen(m: u128) -> i32 {
    128 - m.leading_zeros() as i32
}

/// Compare |m1*2^e1| vs |m2*2^e2| for odd m (exact). Returns Ordering.
fn cmp_mag(m1: u128, e1: i32, m2: u128, e2: i32) -> std::cmp::Ordering {
    let p1 = bitlen(m1) + e1; // msb position + 1
    let p2 = bitlen(m2) + e2;
    if p1 != p2 {
        return p1.cmp(&p2);
    }
    // Same msb position: align to common exponent. d = e1 - e2 = l2 - l1,
    // so the shifted mantissa's bitlen is max(l1, l2) <= 107 — fits u128.
    let d = e1 - e2;
    if d >= 0 {
        (m1 << d).cmp(&m2)
    } else {
        m1.cmp(&(m2 << (-d)))
    }
}

/// The shell every exact comparison in this file shares: the cases
/// decided by CLASS rather than by magnitude — infinities, zero, and
/// opposite signs — with only the magnitude arm left to the caller,
/// which supplies `|x|` (as an odd mantissa and exponent) against
/// whatever it is comparing to.
///
/// One home for it because there were two, line for line, with a
/// comment on one saying it was "the general-purpose form" of the other
/// — which is the rule asking for exactly this.
///
/// `target_neg` is the sign of the value being compared against; the
/// caller's arm answers about MAGNITUDES only and the shell applies the
/// sign flip.
fn cmp_f64_vs_signed(
    x: f64,
    target_neg: bool,
    cmp_abs: impl FnOnce(u128, i32) -> std::cmp::Ordering,
) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;
    if x == f64::INFINITY {
        return Greater;
    }
    if x == f64::NEG_INFINITY {
        return Less;
    }
    if x == 0.0 {
        return if target_neg { Greater } else { Less };
    }
    let (xneg, xm, xe) = decomp(x);
    match (xneg, target_neg) {
        (false, true) => Greater,
        (true, false) => Less,
        (false, false) => cmp_abs(xm, xe),
        (true, true) => cmp_abs(xm, xe).reverse(),
    }
}

/// Exact signed compare of x (an f64) vs the rational p = pm*2^pe with
/// sign pneg (p nonzero). Returns Ordering of x relative to p. Used by
/// the `x` lanes.
fn cmp_f64_vs_rat(x: f64, pneg: bool, pm: u128, pe: i32) -> std::cmp::Ordering {
    cmp_f64_vs_signed(x, pneg, |xm, xe| cmp_mag(xm, xe, pm, pe))
}

fn check_div_case(a: f64, b: f64) {
    let r = DInterval::point(a) / DInterval::point(b);
    assert!(!r.is_nai() && !r.is_empty(), "a={a:e} b={b:e}: poisoned?");
    let (lo, hi) = (r.lo(), r.hi());
    assert!(lo <= hi, "a={a:e} b={b:e}: inverted [{lo:e}, {hi:e}]");
    if a == 0.0 {
        assert!(lo <= 0.0 && 0.0 <= hi, "a=0: zero not contained");
        return;
    }
    // Exact rational a/b: sign and magnitude-compare helpers work on
    // products, so express "lo <= a/b" as a comparison of lo against the
    // rational (a, 1/b) — equivalently compare lo*b against a with the
    // divisor's sign flipping the order. Do it directly: q = a/b as a
    // rational has sign aneg^bneg and magnitude (am*2^ae)/(bm*2^be).
    // cmp(x, a/b) == cmp(x*b, a) if b > 0 else reversed; x*b is exact as
    // a rational product (xm*bm, xe+be).
    let (aneg, am, ae) = decomp(a);
    let (bneg, bm, be) = decomp(b);
    let qneg = aneg ^ bneg;
    let cmp_x_vs_q = |x: f64| {
        // |x| vs |a|/|b|  <=>  |x|*|b| vs |a|, and `x*b` is exact as a
        // rational product (xm*bm, xe+be).
        cmp_f64_vs_signed(x, qneg, |xm, xe| cmp_mag(xm * bm, xe + be, am, ae))
    };
    // F2 containment.
    assert!(
        cmp_x_vs_q(lo) != std::cmp::Ordering::Greater,
        "CONTAINMENT LO VIOLATED: a={a:e}({:#x}) b={b:e}({:#x}) lo={lo:e}",
        a.to_bits(),
        b.to_bits()
    );
    assert!(
        cmp_x_vs_q(hi) != std::cmp::Ordering::Less,
        "CONTAINMENT HI VIOLATED: a={a:e}({:#x}) b={b:e}({:#x}) hi={hi:e}",
        a.to_bits(),
        b.to_bits()
    );
    // F1 witness soundness: a degenerate result claims exactness.
    if lo == hi {
        assert!(
            cmp_x_vs_q(lo) == std::cmp::Ordering::Equal,
            "UNSOUND EXACTNESS WITNESS: a={a:e}({:#x}) b={b:e}({:#x}) q={lo:e}({:#x})",
            a.to_bits(),
            b.to_bits(),
            lo.to_bits()
        );
    }
}

// The stream itself is `fuzz::Rng` (the workspace's single xorshift64*,
// seeded per run); what stays local is only the SHAPING these four lanes
// need, which nothing else in the tree wants.

fn f64_raw(rng: &mut fuzz::Rng) -> f64 {
    f64::from_bits(rng.next_u64())
}

/// Finite f64 with exponent forced near `center` (+/- 32 binades).
fn f64_near_exp(rng: &mut fuzz::Rng, center: i32) -> f64 {
    let m = rng.next_u64() & 0xf_ffff_ffff_ffff;
    let e = (center + 1023 + (rng.next_u64() % 65) as i32 - 32).clamp(0, 2046) as u64;
    let s = rng.next_u64() & (1 << 63);
    f64::from_bits(s | (e << 52) | m)
}

fn subnormal(rng: &mut fuzz::Rng) -> f64 {
    let m = (rng.next_u64() & 0xf_ffff_ffff_ffff) | 1;
    let s = rng.next_u64() & (1 << 63);
    f64::from_bits(s | m)
}

fn ok_pair(a: f64, b: f64) -> bool {
    a.is_finite() && b.is_finite() && b != 0.0
}

// Shipped case counts, one per lane, BEFORE `fuzz::scaled` multiplies
// them by the EFFORT dial. Each is the adversarial review's full sweep
// divided by 280, so `CAD_FUZZ_EFFORT=280` reproduces that sweep (what
// the deleted `#[ignore]`d twin used to do) and the shipped level is
// about an eighth of the fixed-seed "reduced" run this replaced.
//
// ALL FOUR lanes are kept at every effort, which is why the dial scales
// them together instead of any lane being dropped: a cheaper sweep that
// dropped one would lose the witness-floor and subnormal coverage lanes
// 2-4 exist for.
const LANE1_RAW_PAIRS: usize = 21_429; // full sweep: 6_000_000
const LANE2_EXACT_DIVISIONS: usize = 14_286; // full sweep: 4_000_000
// One per-window count for all three operations' lane 3, not two: this
// value carries provenance (the M5 PR 1 adversarial review's 500 000 per
// window, divided by 280 so `CAD_FUZZ_EFFORT=280` reproduces it), and a
// second, chosen number beside it was two names for one concept with
// drifted values.
const LANE3_PER_WINDOW: usize = 1_786; // full sweep: 500_000, x8 windows

/// Anti-vacuity floor on the total case count: a generator that started
/// rejecting nearly everything (or a lane silently skipped) fails here
/// rather than reporting green on nothing. Scaled from the same
/// constants as the loops, so it tracks the dial exactly; the shipped
/// run clears it by ~1.75x, as the full sweep always did.
const CASE_FLOOR: usize = 35_714;

/// Anti-vacuity floor on LANE 2's generator specifically, counted from a
/// counter LANE 2 ALONE increments — does it still construct divisions
/// that come out exact, at the rate it was built to?
/// This is NOT the exactness-witness coverage claim (that is written
/// down as a static fixture below, see the module docs); it is the
/// tripwire for lane 2 quietly degenerating into lane 1. The shipped run
/// clears it by ~33x, so a varying seed cannot bring it near.
const DEGENERATE_FLOOR: usize = 357;

#[test]
fn fuzz_div_witness_soundness_and_containment() {
    let mut rng = fuzz::start("review_fuzz_exact::div");
    let mut n = 0usize;
    // Two counters, not one. The floor below is about LANE 2's
    // constructed-exact generator, so lane 1's incidental exact results
    // must not be able to satisfy it — they are rare (a random-bit-
    // pattern quotient is exact with probability ~2^-47), so today the
    // single counter happened to work, which is correct by accident and
    // not what the assert says.
    let mut degenerate_lane1 = 0usize;
    let mut degenerate = 0usize;
    // Lane 1: raw random bit patterns (full exponent sweep, subnormals,
    // signed zeros in the numerator, everything).
    for _ in 0..fuzz::scaled(LANE1_RAW_PAIRS) {
        let (a, b) = (f64_raw(&mut rng), f64_raw(&mut rng));
        if !ok_pair(a, b) {
            continue;
        }
        let r = DInterval::point(a) / DInterval::point(b);
        if a != 0.0 && r.lo() == r.hi() {
            degenerate_lane1 += 1;
        }
        check_div_case(a, b);
        n += 1;
    }
    // Lane 2: constructed EXACT divisions a = q*b with small mantissas,
    // across the exponent range including the 2^-960 witness floor and
    // the subnormal zone (witness must refuse; containment must hold).
    for _ in 0..fuzz::scaled(LANE2_EXACT_DIVISIONS) {
        let mq = ((rng.next_u64() & 0x3f_ffff) | 1) as f64; // odd, <= 22 bits
        let mb = ((rng.next_u64() & 0x3fff_ffff) | 1) as f64; // odd, <= 30 bits
        let scale_b = ((rng.next_u64() % 600) as i32) - 300;
        let scale_q = ((rng.next_u64() % 2400) as i32) - 1360; // reaches subnormal & near-max products
        let b = mb * pow2(scale_b);
        let q = mq * pow2(scale_q);
        let a = q * b; // exact when the product mantissa fits (<= 52 bits) and no over/underflow rounding
        if !ok_pair(a, b) || a == 0.0 {
            continue;
        }
        let r = DInterval::point(a) / DInterval::point(b);
        if r.lo() == r.hi() {
            degenerate += 1;
        }
        check_div_case(a, b);
        n += 1;
    }
    // Lane 3: magnitude-window pairs around the witness floor 2^-960,
    // the subnormal boundary, and MAX.
    for center in [-960i32, -1022, -900, -480, 0, 480, 900, 1020] {
        for _ in 0..fuzz::scaled(LANE3_PER_WINDOW) {
            let a = f64_near_exp(&mut rng, center);
            let bc = (rng.next_u64() % 41) as i32 - 20;
            let b = f64_near_exp(&mut rng, bc);
            if ok_pair(a, b) && b != 0.0 {
                check_div_case(a, b);
                n += 1;
            }
            let bs = subnormal(&mut rng);
            if ok_pair(a, bs) {
                check_div_case(a, bs);
                n += 1;
            }
        }
    }
    // Lane 4: targeted edges, from the one list of magnitudes worth
    // naming (`EDGE_MAGNITUDES` below) rather than a second copy of it.
    for a in EDGE_MAGNITUDES {
        for b in EDGE_MAGNITUDES {
            if ok_pair(a, b) {
                check_div_case(a, b);
                n += 1;
            }
        }
    }
    // Both floors scale with the dial, so the shipped smoke level is
    // still a real test rather than a self-satisfied one: a harness that
    // stopped generating cases, or a lane 2 that stopped constructing
    // exact divisions, fails here either way.
    assert!(
        n > fuzz::scaled(CASE_FLOOR),
        "coverage floor: only n={n} cases generated — {}",
        fuzz::replay()
    );
    assert!(
        degenerate > fuzz::scaled(DEGENERATE_FLOOR),
        "lane 2 rarely produced an exact division: {degenerate} — {}",
        fuzz::replay()
    );
    println!(
        "review_fuzz_exact div: checked {n} cases, {degenerate} degenerate-exact \
         in lane 2 (+{degenerate_lane1} incidental in lane 1)"
    );
}

/// **The exactness witness, on cases written down rather than hunted
/// for.** Shape 2 of `test_utils::fuzz`'s taxonomy: the class "a
/// division that is exact" is concisely constructible, so constructing
/// it costs nothing and holds on 100% of runs, where searching for it
/// would hold on ~99% and would make the sweep's sample count carry an
/// anti-monotone obligation.
///
/// Every row is an exact quotient by construction (the mantissa product
/// fits in 53 bits and neither operand is near an overflow or underflow
/// boundary), so `point(a)/point(b)` must come back DEGENERATE — the
/// witness fired on both endpoints — and `check_div_case` re-derives that
/// exactness against exact rational arithmetic.
#[test]
fn the_exactness_witness_fires_on_written_down_exact_divisions() {
    for (a, b) in [
        (6.0, 2.0),
        (1.0, 2.0),
        (1.0, 4.0),
        (-7.5, 2.5),
        (3.0, 1.0),
        (3.0, -1.0),
        // 2^53 / 2 — the widest mantissa an f64 quotient can carry.
        (9_007_199_254_740_992.0, 2.0),
        // Large and small magnitudes, still exact and still well away
        // from the 2^-960 witness floor.
        (f64::MAX, 2.0),
        (pow2(-500), pow2(-100)),
        (pow2(700), pow2(200)),
    ] {
        let r = DInterval::point(a) / DInterval::point(b);
        assert!(
            r.lo() == r.hi(),
            "exactness witness did NOT fire on the exact division \
             a={a:e}({:#x}) b={b:e}({:#x}): got [{:e}, {:e}]",
            a.to_bits(),
            b.to_bits(),
            r.lo(),
            r.hi()
        );
        check_div_case(a, b);
    }
}

fn pow2(e: i32) -> f64 {
    if e >= -1022 {
        f64::from_bits(((e + 1023) as u64) << 52)
    } else {
        // subnormal power of two
        f64::from_bits(1u64 << (e + 1074).max(0))
    }
}

// ---------------------------------------------------------------------
// `×` and `sqrt`: the two operations that share `mul_exact`'s witness
// and, until now, none of its testing.
// ---------------------------------------------------------------------

/// Exact compare of the f64 `x` against the true product `a·b` (both
/// operands finite and nonzero). `decomp` returns odd mantissas of at
/// most 53 bits, so their product is under 2^106 and stays in a u128.
fn cmp_x_vs_product(x: f64, a: f64, b: f64) -> std::cmp::Ordering {
    let (aneg, am, ae) = decomp(a);
    let (bneg, bm, be) = decomp(b);
    cmp_f64_vs_rat(x, aneg ^ bneg, am * bm, ae + be)
}

/// Exact compare of `x·x` against `a`, both nonnegative and finite.
/// Same width argument as [`cmp_x_vs_product`]: `xm*xm` is under 2^106.
fn cmp_square_vs(x: f64, a: f64) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;
    if x == 0.0 || a == 0.0 {
        // Zero on either side settles it: `x·x` and `a` are both
        // nonnegative, so the nonzero one is the greater.
        return if x == 0.0 && a == 0.0 {
            Equal
        } else if x == 0.0 {
            Less
        } else {
            Greater
        };
    }
    let (_, xm, xe) = decomp(x);
    let (_, am, ae) = decomp(a);
    cmp_mag(xm * xm, 2 * xe, am, ae)
}

fn check_mul_case(a: f64, b: f64) {
    use std::cmp::Ordering::*;
    let r = DInterval::point(a) * DInterval::point(b);
    assert!(!r.is_nai() && !r.is_empty(), "a={a:e} b={b:e}: poisoned?");
    let (lo, hi) = (r.lo(), r.hi());
    assert!(lo <= hi, "a={a:e} b={b:e}: inverted [{lo:e}, {hi:e}]");
    if a == 0.0 || b == 0.0 {
        // Kahan's corner convention: an exactly-zero factor makes the
        // true product the real number 0, so the enclosure is the exact
        // point 0 — not a padded bracket around it.
        assert!(
            lo == 0.0 && hi == 0.0,
            "a={a:e} b={b:e}: zero factor must give the exact point 0, got [{lo:e}, {hi:e}]"
        );
        return;
    }
    assert!(
        cmp_x_vs_product(lo, a, b) != Greater,
        "CONTAINMENT LO VIOLATED: a={a:e}({:#x}) b={b:e}({:#x}) lo={lo:e}",
        a.to_bits(),
        b.to_bits()
    );
    assert!(
        cmp_x_vs_product(hi, a, b) != Less,
        "CONTAINMENT HI VIOLATED: a={a:e}({:#x}) b={b:e}({:#x}) hi={hi:e}",
        a.to_bits(),
        b.to_bits()
    );
    if lo == hi {
        assert!(
            cmp_x_vs_product(lo, a, b) == Equal,
            "UNSOUND EXACTNESS WITNESS: a={a:e}({:#x}) b={b:e}({:#x}) r={lo:e}({:#x})",
            a.to_bits(),
            b.to_bits(),
            lo.to_bits()
        );
    }
}

fn check_sqrt_case(a: f64) {
    use std::cmp::Ordering::*;
    let r = DInterval::point(a).sqrt();
    if a < 0.0 {
        // Full domain miss on a point: Empty, and nothing to compare.
        assert!(r.is_empty(), "a={a:e}: negative radicand must be Empty");
        return;
    }
    assert!(!r.is_nai() && !r.is_empty(), "a={a:e}: poisoned?");
    let (lo, hi) = (r.lo(), r.hi());
    assert!(
        0.0 <= lo && lo <= hi && hi.is_finite(),
        "a={a:e}: bad enclosure [{lo:e}, {hi:e}]"
    );
    assert!(
        cmp_square_vs(lo, a) != Greater,
        "CONTAINMENT LO VIOLATED: a={a:e}({:#x}) lo={lo:e}",
        a.to_bits()
    );
    assert!(
        cmp_square_vs(hi, a) != Less,
        "CONTAINMENT HI VIOLATED: a={a:e}({:#x}) hi={hi:e}",
        a.to_bits()
    );
    if lo == hi {
        assert!(
            cmp_square_vs(lo, a) == Equal,
            "UNSOUND EXACTNESS WITNESS: a={a:e}({:#x}) s={lo:e}({:#x})",
            a.to_bits(),
            lo.to_bits()
        );
    }
}

// Shipped counts, same shape as the division lanes: multiples of the
// EFFORT dial, so `CAD_FUZZ_EFFORT=280` deepens these by the same factor
// as everything else in the file.
const LANE_MUL_RAW: usize = 20_000;
const LANE_MUL_EXACT: usize = 10_000;
const LANE_SQRT_RAW: usize = 20_000;
const LANE_SQRT_EXACT: usize = 10_000;

/// Anti-vacuity floors, in the same spirit as the division lanes'. The
/// shipped run clears the case floors by ~2.35x (mul: 58.8k against
/// 25k) and ~2.15x (sqrt: 53.8k against 25k), and the degenerate floors
/// by ~7.2x and ~8.3x — measured over varying-seed runs at effort 1,
/// whose spread was under 2% on every column.
///
/// The degenerate counters count **lane 2 alone**. Measured, lane 1's
/// incidental contribution is exactly 0 for all three operations, which
/// is what the probability argument predicts (a random-bit-pattern
/// product is exact with probability ~54·2^-53) — but a counter that is
/// right because the other contributor happens to be empty is right by
/// accident, and the assert names lane 2.
///
/// They do NOT certify a rate, and a real change in the numbers is a
/// thing to look at, not to restore.
const MUL_CASE_FLOOR: usize = 25_000;
const SQRT_CASE_FLOOR: usize = 25_000;
const MUL_DEGENERATE_FLOOR: usize = 1_000;
const SQRT_DEGENERATE_FLOOR: usize = 1_000;

#[test]
fn fuzz_mul_witness_soundness_and_containment() {
    let mut rng = fuzz::start("review_fuzz_exact::mul");
    // Split as in the division test: the floor is lane 2's.
    let (mut n, mut degenerate, mut degenerate_lane1) = (0usize, 0usize, 0usize);
    // Lane 1: raw bit patterns — full exponent sweep, subnormals, signed
    // zeros, and every over/underflowing product they produce.
    for _ in 0..fuzz::scaled(LANE_MUL_RAW) {
        let (a, b) = (f64_raw(&mut rng), f64_raw(&mut rng));
        if !a.is_finite() || !b.is_finite() {
            continue;
        }
        let r = DInterval::point(a) * DInterval::point(b);
        if a != 0.0 && b != 0.0 && r.lo() == r.hi() {
            degenerate_lane1 += 1;
        }
        check_mul_case(a, b);
        n += 1;
    }
    // Lane 2: constructed EXACT products — mantissas whose product fits
    // in 53 bits — swept across the exponent range so that the 2^-960
    // witness floor and the subnormal zone are both crossed. Below the
    // floor the witness must REFUSE (pad anyway) while containment still
    // holds; that is the case that produced the original bug.
    for _ in 0..fuzz::scaled(LANE_MUL_EXACT) {
        let ma = ((rng.next_u64() & 0x3f_ffff) | 1) as f64; // odd, <= 22 bits
        let mb = ((rng.next_u64() & 0x3fff_ffff) | 1) as f64; // odd, <= 30 bits
        let a = ma * pow2(((rng.next_u64() % 2000) as i32) - 1200);
        let b = mb * pow2(((rng.next_u64() % 2000) as i32) - 1200);
        if !a.is_finite() || !b.is_finite() {
            continue;
        }
        let r = DInterval::point(a) * DInterval::point(b);
        if a != 0.0 && b != 0.0 && r.lo() == r.hi() {
            degenerate += 1;
        }
        check_mul_case(a, b);
        n += 1;
    }
    // Lane 3: magnitude windows centred on the witness floor, the
    // subnormal boundary, and the overflow edge.
    for center in [-960i32, -1022, -900, -480, 0, 480, 900, 1020] {
        for _ in 0..fuzz::scaled(LANE3_PER_WINDOW) {
            let a = f64_near_exp(&mut rng, center);
            let bc = (rng.next_u64() % 41) as i32 - 20;
            let b = f64_near_exp(&mut rng, bc);
            if a.is_finite() && b.is_finite() {
                check_mul_case(a, b);
                n += 1;
            }
            let bs = subnormal(&mut rng);
            if a.is_finite() {
                check_mul_case(a, bs);
                n += 1;
            }
        }
    }
    // Lane 4: targeted edges, including the floor's exact bit pattern
    // and its two neighbours.
    for a in EDGE_MAGNITUDES {
        for b in EDGE_MAGNITUDES {
            check_mul_case(a, b);
            n += 1;
        }
    }
    assert!(
        n > fuzz::scaled(MUL_CASE_FLOOR),
        "coverage floor: only n={n} mul cases generated — {}",
        fuzz::replay()
    );
    assert!(
        degenerate > fuzz::scaled(MUL_DEGENERATE_FLOOR),
        "lane 2 rarely produced an exact product: {degenerate} — {}",
        fuzz::replay()
    );
    println!(
        "review_fuzz_exact mul: checked {n} cases, {degenerate} degenerate-exact \
         in lane 2 (+{degenerate_lane1} incidental in lane 1)"
    );
}

#[test]
fn fuzz_sqrt_witness_soundness_and_containment() {
    let mut rng = fuzz::start("review_fuzz_exact::sqrt");
    // Split as in the division test: the floor is lane 2's.
    let (mut n, mut degenerate, mut degenerate_lane1) = (0usize, 0usize, 0usize);
    // Lane 1: raw bit patterns, sign included — negatives exercise the
    // full-miss refusal rather than being filtered away.
    for _ in 0..fuzz::scaled(LANE_SQRT_RAW) {
        let a = f64_raw(&mut rng);
        if !a.is_finite() {
            continue;
        }
        let r = DInterval::point(a).sqrt();
        if a > 0.0 && r.lo() == r.hi() {
            degenerate_lane1 += 1;
        }
        check_sqrt_case(a);
        n += 1;
    }
    // Lane 2: constructed EXACT squares `a = s·s`, swept across the
    // exponent range so the witness floor and the subnormal zone are
    // crossed. Below the floor the witness must refuse even though the
    // root IS exact — the sqrt mirror of the division lane's case.
    for _ in 0..fuzz::scaled(LANE_SQRT_EXACT) {
        let ms = ((rng.next_u64() & 0x03ff_ffff) | 1) as f64; // odd, <= 26 bits
        let s = ms * pow2(((rng.next_u64() % 1200) as i32) - 700);
        let a = s * s;
        if !a.is_finite() || a == 0.0 {
            continue;
        }
        let r = DInterval::point(a).sqrt();
        if r.lo() == r.hi() {
            degenerate += 1;
        }
        check_sqrt_case(a);
        n += 1;
    }
    // Lane 3: magnitude windows, plus subnormal radicands.
    for center in [-1074i32, -1022, -960, -480, 0, 480, 1020] {
        for _ in 0..fuzz::scaled(LANE3_PER_WINDOW) {
            let a = f64_near_exp(&mut rng, center);
            if a.is_finite() {
                check_sqrt_case(a);
                n += 1;
            }
            check_sqrt_case(subnormal(&mut rng).abs());
            n += 1;
        }
    }
    // Lane 4: targeted edges.
    for a in EDGE_MAGNITUDES {
        check_sqrt_case(a);
        n += 1;
    }
    assert!(
        n > fuzz::scaled(SQRT_CASE_FLOOR),
        "coverage floor: only n={n} sqrt cases generated — {}",
        fuzz::replay()
    );
    assert!(
        degenerate > fuzz::scaled(SQRT_DEGENERATE_FLOOR),
        "lane 2 rarely produced an exact square root: {degenerate} — {}",
        fuzz::replay()
    );
    println!(
        "review_fuzz_exact sqrt: checked {n} cases, {degenerate} degenerate-exact \
         in lane 2 (+{degenerate_lane1} incidental in lane 1)"
    );
}

/// The magnitudes worth naming: the extremes, the subnormal boundary and
/// its neighbours, and the 2^-960 witness floor with the value either
/// side of it — the boundary the original containment violation sat on.
const EDGE_MAGNITUDES: [f64; 15] = [
    f64::MAX,
    -f64::MAX,
    f64::MIN_POSITIVE,
    -f64::MIN_POSITIVE,
    f64::from_bits(1),
    f64::from_bits(0x000f_ffff_ffff_ffff),
    f64::from_bits(0x03f0_0000_0000_0000), // 2^-960 exactly
    f64::from_bits(0x03f0_0000_0000_0001),
    f64::from_bits(0x03ef_ffff_ffff_ffff),
    0.0,
    -0.0,
    1.0,
    -1.0,
    2.0,
    0.5,
];

/// **The `×` and `sqrt` witnesses, on cases written down rather than
/// hunted for** — the sibling of
/// [`the_exactness_witness_fires_on_written_down_exact_divisions`], and
/// shape 2 of `test_utils::fuzz`'s taxonomy for the same reason: "a
/// product that is exact" is concisely constructible, so it is
/// constructed and holds on 100% of runs instead of being searched for.
#[test]
fn the_exactness_witnesses_fire_on_written_down_exact_products_and_squares() {
    for (a, b) in [
        (2.0, 3.0),
        (-1.5, 8.0),
        (0.5, 0.25),
        (1024.0, 1024.0),
        // 2^26 squared is 2^52 — the widest mantissa an exact f64
        // product of two equal factors can carry.
        (67_108_864.0, 67_108_864.0),
        (f64::MAX, 0.5),
        (pow2(-400), pow2(-300)),
        (pow2(400), pow2(300)),
    ] {
        let r = DInterval::point(a) * DInterval::point(b);
        assert!(
            r.lo() == r.hi(),
            "exactness witness did NOT fire on the exact product \
             a={a:e}({:#x}) b={b:e}({:#x}): got [{:e}, {:e}]",
            a.to_bits(),
            b.to_bits(),
            r.lo(),
            r.hi()
        );
        check_mul_case(a, b);
    }
    for s in [1.0, 2.0, 0.5, 3.0, 1.5, 65_536.0, pow2(-300), pow2(300)] {
        let a = s * s;
        let r = DInterval::point(a).sqrt();
        assert!(
            r.lo() == r.hi(),
            "exactness witness did NOT fire on the exact square \
             a={a:e}({:#x}), root {s:e}: got [{:e}, {:e}]",
            a.to_bits(),
            r.lo(),
            r.hi()
        );
        check_sqrt_case(a);
    }
}
