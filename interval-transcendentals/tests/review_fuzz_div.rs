//! **Exact-rational fuzz of the division exactness witness.** Adopted
//! verbatim (bar the case counts) from the M5 PR 1 adversarial review's
//! scratch harness — an independent derivation, which is exactly its
//! regression value: it shares no code with the implementation it checks.
//!
//! F1: whenever division returns a DEGENERATE point [q, q] (the exactness
//! witness fired on both endpoints), q*b must equal a in exact rational
//! arithmetic. F2: the true quotient a/b always lies in [lo, hi], checked
//! by exact integer comparison (never by f64 re-evaluation).
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
//! CAD_FUZZ_EFFORT=280 cargo test --test review_fuzz_div -- --nocapture
//! ```
//!
//! Any run is bit-reproducible from the seed it logged:
//! `CAD_FUZZ_SEED=0x… cargo test --test review_fuzz_div -- --nocapture`.
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

/// Exact signed compare of x (an f64) vs the rational p = pm*2^pe with
/// sign pneg (p nonzero). Returns Ordering of x relative to p.
///
/// Kept from the review harness verbatim although `check_case`'s
/// specialized closure supersedes it: it is the general-purpose form of
/// the same comparison, and the next property test that needs one should
/// reuse it rather than re-derive the bit twiddling.
#[allow(dead_code)]
fn cmp_f64_vs_rat(x: f64, pneg: bool, pm: u128, pe: i32) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;
    if x == f64::INFINITY {
        return Greater;
    }
    if x == f64::NEG_INFINITY {
        return Less;
    }
    if x == 0.0 {
        return if pneg { Greater } else { Less };
    }
    let (xneg, xm, xe) = decomp(x);
    match (xneg, pneg) {
        (false, true) => Greater,
        (true, false) => Less,
        (false, false) => cmp_mag(xm, xe, pm, pe),
        (true, true) => cmp_mag(xm, xe, pm, pe).reverse(),
    }
}

/// The oracle: check [lo, hi] = point(a)/point(b) against the EXACT
/// rational a/b. Containment: lo <= a/b <= hi as rationals, i.e.
/// lo*b <=> a with sign handling. Exactness (degenerate result): a == q*b.
fn check_case(a: f64, b: f64) {
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
    let cmp_x_vs_q = |x: f64| -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        if x == f64::INFINITY {
            return Greater;
        }
        if x == f64::NEG_INFINITY {
            return Less;
        }
        if x == 0.0 {
            return if qneg { Greater } else { Less };
        }
        let (xneg, xm, xe) = decomp(x);
        match (xneg, qneg) {
            (false, true) => Greater,
            (true, false) => Less,
            _ => {
                // |x| vs |a|/|b|  <=>  |x|*|b| vs |a|
                let ord = cmp_mag(xm * bm, xe + be, am, ae);
                if xneg { ord.reverse() } else { ord }
            }
        }
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
const LANE3_PER_WINDOW: usize = 1_786; // full sweep: 500_000, x8 windows

/// Anti-vacuity floor on the total case count: a generator that started
/// rejecting nearly everything (or a lane silently skipped) fails here
/// rather than reporting green on nothing. Scaled from the same
/// constants as the loops, so it tracks the dial exactly; the shipped
/// run clears it by ~1.75x, as the full sweep always did.
const CASE_FLOOR: usize = 35_714;

/// Anti-vacuity floor on LANE 2's generator specifically — does it still
/// construct divisions that come out exact, at the rate it was built to?
/// This is NOT the exactness-witness coverage claim (that is written
/// down as a static fixture below, see the module docs); it is the
/// tripwire for lane 2 quietly degenerating into lane 1. The shipped run
/// clears it by ~33x, so a varying seed cannot bring it near.
const DEGENERATE_FLOOR: usize = 357;

#[test]
fn fuzz_div_witness_soundness_and_containment() {
    let mut rng = fuzz::start("review_fuzz_div");
    let mut n = 0usize;
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
            degenerate += 1;
        }
        check_case(a, b);
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
        check_case(a, b);
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
                check_case(a, b);
                n += 1;
            }
            let bs = subnormal(&mut rng);
            if ok_pair(a, bs) {
                check_case(a, bs);
                n += 1;
            }
        }
    }
    // Lane 4: targeted edges.
    for a in [
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
    ] {
        for b in [
            f64::MAX,
            -f64::MAX,
            f64::MIN_POSITIVE,
            f64::from_bits(1),
            1.0,
            -1.0,
            2.0,
            0.5,
            3.0,
            f64::from_bits(0x03f0_0000_0000_0000),
        ] {
            if ok_pair(a, b) {
                check_case(a, b);
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
    println!("review_fuzz_div: checked {n} cases, {degenerate} degenerate-exact");
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
/// witness fired on both endpoints — and `check_case` re-derives that
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
        check_case(a, b);
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
