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
//! "Certification", ci.yml's `interval-backend` job). The default run is
//! a reduced seeded subset of ~500k cases across all four lanes; the full
//! 17.5M-case sweep the review ran is behind `#[ignore]`:
//!
//! ```text
//! cargo test --test review_fuzz_div -- --ignored --nocapture
//! ```
//!
//! Both modes are bit-reproducible: one seed, `xorshift64*`, no threads.

use interval_transcendentals::DInterval;

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

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
    fn f64_raw(&mut self) -> f64 {
        f64::from_bits(self.next())
    }
    /// Finite f64 with exponent forced near `center` (+/- 32 binades).
    fn f64_near_exp(&mut self, center: i32) -> f64 {
        let m = self.next() & 0xf_ffff_ffff_ffff;
        let e = (center + 1023 + (self.next() % 65) as i32 - 32).clamp(0, 2046) as u64;
        let s = self.next() & (1 << 63);
        f64::from_bits(s | (e << 52) | m)
    }
    fn subnormal(&mut self) -> f64 {
        let m = (self.next() & 0xf_ffff_ffff_ffff) | 1;
        let s = self.next() & (1 << 63);
        f64::from_bits(s | m)
    }
}

fn ok_pair(a: f64, b: f64) -> bool {
    a.is_finite() && b.is_finite() && b != 0.0
}

/// Divisor applied to every lane's case count in the default run; `1`
/// is the review's full sweep. Chosen so the default lands near 500k
/// total while keeping ALL FOUR lanes (a cheaper sweep that dropped a
/// lane would lose the witness-floor and subnormal coverage that lanes
/// 2-4 exist for).
const REDUCED_DIVISOR: u64 = 35;

fn fuzz(divisor: u64, label: &str) {
    let mut rng = Rng(0x9e3779b97f4a7c15);
    let mut n = 0u64;
    let mut degenerate = 0u64;
    // Lane 1: raw random bit patterns (full exponent sweep, subnormals,
    // signed zeros in the numerator, everything).
    for _ in 0..(6_000_000u64 / divisor) {
        let (a, b) = (rng.f64_raw(), rng.f64_raw());
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
    for _ in 0..(4_000_000u64 / divisor) {
        let mq = ((rng.next() & 0x3f_ffff) | 1) as f64; // odd, <= 22 bits
        let mb = ((rng.next() & 0x3fff_ffff) | 1) as f64; // odd, <= 30 bits
        let scale_b = ((rng.next() % 600) as i32) - 300;
        let scale_q = ((rng.next() % 2400) as i32) - 1360; // reaches subnormal & near-max products
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
        for _ in 0..(500_000u64 / divisor) {
            let a = rng.f64_near_exp(center);
            let bc = (rng.next() % 41) as i32 - 20;
            let b = rng.f64_near_exp(bc);
            if ok_pair(a, b) && b != 0.0 {
                check_case(a, b);
                n += 1;
            }
            let bs = rng.subnormal();
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
    // Floors scale with the run, so the reduced mode is still a real
    // test rather than a smoke check: a harness that stopped generating
    // cases, or a witness that stopped firing, fails here either way.
    assert!(n > 10_000_000 / divisor, "{label}: coverage floor: n={n}");
    assert!(
        degenerate > 100_000 / divisor,
        "{label}: witness rarely fired: {degenerate}"
    );
    println!("{label}: checked {n} cases, {degenerate} degenerate-exact");
}

#[test]
fn fuzz_div_witness_soundness_and_containment() {
    fuzz(REDUCED_DIVISOR, "review_fuzz_div (reduced)");
}

/// The full 17.5M-case sweep exactly as the adversarial review ran it
/// (minutes, not seconds — hence `#[ignore]`).
#[test]
#[ignore = "full 17.5M-case sweep; run with --ignored"]
fn fuzz_div_witness_soundness_and_containment_full() {
    fuzz(1, "review_fuzz_div (full)");
}

fn pow2(e: i32) -> f64 {
    if e >= -1022 {
        f64::from_bits(((e + 1023) as u64) << 52)
    } else {
        // subnormal power of two
        f64::from_bits(1u64 << (e + 1074).max(0))
    }
}
