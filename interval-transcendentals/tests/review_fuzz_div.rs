//! **Exact-rational fuzz of the division exactness witness** (adopted
//! from the M5 PR 1 adversarial review's scratch harness).
//!
//! The witness added in M5 PR 1 claims: when `fma(q, b, -a) == 0` above
//! the 2Prod validity floor, `q = RN(a/b)` is the *exact* quotient, so
//! `div_lo`/`div_hi` may return it with no outward pad. If that claim is
//! ever wrong, the interval silently stops containing the truth — the
//! one failure mode an interval library may not have.
//!
//! This file checks it **without any oracle library**, which is the
//! point: it is the pad tripwire that can run in the kernel's own CI
//! with no C toolchain (README "Certification"). The comparator is exact
//! rational arithmetic on `u128`. Every finite nonzero `f64` is
//! `±m·2^e` with `m < 2^53`; for `a = ±ma·2^ea` and `b = ±mb·2^eb`,
//!
//! ```text
//!   a/b == q   ⟺   ma·2^ea·mq·... — i.e. ma·2^(ea-eb) == mq·mb·2^(eq-eb)
//! ```
//!
//! which is decided by comparing `ma << s1` against `mq·mb << s2` for
//! non-negative shifts `s1`, `s2` chosen to clear the exponent
//! difference. `mq·mb < 2^106` fits `u128`, and the shifts are rejected
//! when they would overflow, so the comparator never lies — it either
//! decides exactly or declines the case.
//!
//! Four lanes, each seeded (`xorshift64*`) and therefore bit-reproducible:
//! random binades, exact-quotient-biased pairs, power-of-two divisors,
//! and subnormal/extreme magnitudes. The default run is a reduced subset;
//! the full 17.5M-case sweep the review ran is behind `#[ignore]`:
//!
//! ```text
//! cargo test --test review_fuzz_div -- --ignored --nocapture
//! ```
//!
//! Two properties per case, both absolute:
//! 1. **No unsound firing** — if the implementation returns an unpadded
//!    endpoint, the exact comparator must agree the quotient is exact.
//! 2. **Containment** — `[div_lo, div_hi]` must bracket the true `a/b`,
//!    decided exactly (not by comparing to another f64 division).

/// Cases per lane in the default run (~500k total across four lanes).
const DEFAULT_PER_LANE: u64 = 125_000;
/// Cases per lane in the `--ignored` full sweep (17.5M total).
const FULL_PER_LANE: u64 = 4_375_000;

/// `xorshift64*` — seeded, tiny, dependency-free, bit-reproducible.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

/// A finite nonzero `f64` decomposed as `sign · mantissa · 2^exponent`
/// with an *integer* mantissa (subnormals included).
struct Parts {
    neg: bool,
    mant: u128,
    exp: i32,
}

fn decompose(x: f64) -> Option<Parts> {
    if !x.is_finite() || x == 0.0 {
        return None;
    }
    let bits = x.to_bits();
    let neg = bits >> 63 == 1;
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let frac = u128::from(bits & 0x000f_ffff_ffff_ffff);
    // Normal: (2^52 + frac)·2^(biased-1075). Subnormal: frac·2^-1074.
    let (mant, exp) = if biased == 0 {
        (frac, -1074)
    } else {
        (frac + (1u128 << 52), biased - 1075)
    };
    Some(Parts { neg, mant, exp })
}

/// Exact three-way comparison of the real numbers `a/b` and `q`, or
/// `None` when the comparator declines (overflow risk — never a lie).
///
/// `a/b` vs `q` has the sign structure of `a` vs `q·b`; with all three
/// decomposed as integer-mantissa × power of two, that is
/// `ma·2^ea` vs `mq·mb·2^(eq+eb)`, i.e. a comparison of two integers
/// after shifting both up by the same amount to clear the exponents.
fn cmp_quotient(a: f64, b: f64, q: f64) -> Option<core::cmp::Ordering> {
    use core::cmp::Ordering;
    let (pa, pb, pq) = (decompose(a)?, decompose(b)?, decompose(q)?);
    // Sign first: a/b and q agree in sign iff (neg_a ^ neg_b) == neg_q.
    let quotient_neg = pa.neg ^ pb.neg;
    if quotient_neg != pq.neg {
        // Different signs and both nonzero: ordering follows the sign.
        return Some(if quotient_neg {
            Ordering::Less
        } else {
            Ordering::Greater
        });
    }
    // Magnitudes: |a|·2^ea  vs  |q|·|b|·2^(eq+eb).
    let lhs_m = pa.mant;
    let rhs_m = pq.mant.checked_mul(pb.mant)?; // < 2^106, always fits
    let (lhs_e, rhs_e) = (pa.exp, pq.exp + pb.exp);
    // Shift the smaller exponent up so both sides share exponent
    // min(lhs_e, rhs_e); decline if a shift would overflow u128.
    let base = lhs_e.min(rhs_e);
    let (ls, rs) = ((lhs_e - base) as u32, (rhs_e - base) as u32);
    let lhs = shift_checked(lhs_m, ls)?;
    let rhs = shift_checked(rhs_m, rs)?;
    let ord = lhs.cmp(&rhs);
    // Negative magnitudes reverse the ordering of the real values.
    Some(if quotient_neg { ord.reverse() } else { ord })
}

fn shift_checked(m: u128, s: u32) -> Option<u128> {
    if s >= 128 {
        return None;
    }
    let shifted = m.checked_shl(s)?;
    // Reject a shift that dropped bits off the top.
    if (shifted >> s) != m {
        None
    } else {
        Some(shifted)
    }
}

/// The implementation under test, re-derived here from the public
/// surface: `[a,a] / [b,b]` is exactly `[div_lo(a,b), div_hi(a,b)]` for
/// one-signed nonzero `b`, so a point/point division exposes the
/// rounding layer without making its internals public.
fn div_endpoints(a: f64, b: f64) -> (f64, f64) {
    use interval_transcendentals::DInterval;
    let r = DInterval::point(a) / DInterval::point(b);
    (r.lo(), r.hi())
}

/// One case: check no-unsound-firing and containment, exactly.
/// Returns `true` when the comparator was able to decide.
fn check(a: f64, b: f64) -> bool {
    if !a.is_finite() || !b.is_finite() || a == 0.0 || b == 0.0 {
        return false;
    }
    let (lo, hi) = div_endpoints(a, b);
    if !lo.is_finite() || !hi.is_finite() {
        return false; // overflow to an unbounded side: nothing to decide
    }
    // Property 1: an unpadded (exact-claimed) result must BE exact.
    if lo == hi {
        match cmp_quotient(a, b, lo) {
            Some(core::cmp::Ordering::Equal) => {}
            Some(other) => panic!(
                "UNSOUND WITNESS: div claimed {a:e} / {b:e} == {lo:e} exactly, \
                 but exact comparison says truth is {other:?} than that"
            ),
            None => return false,
        }
        return true;
    }
    // Property 2: containment, decided exactly on both sides.
    let below = cmp_quotient(a, b, lo);
    let above = cmp_quotient(a, b, hi);
    match (below, above) {
        (Some(bl), Some(ab)) => {
            assert!(
                bl != core::cmp::Ordering::Less,
                "CONTAINMENT VIOLATION: {a:e} / {b:e} is below lo = {lo:e}"
            );
            assert!(
                ab != core::cmp::Ordering::Greater,
                "CONTAINMENT VIOLATION: {a:e} / {b:e} is above hi = {hi:e}"
            );
            true
        }
        _ => false,
    }
}

/// Lane 1: uniformly random bit patterns, filtered to finite nonzero.
fn lane_random(n: u64, decided: &mut u64) {
    let mut rng = Rng(0x5DEE_CE66_D125_0001);
    for _ in 0..n {
        let a = f64::from_bits(rng.next_u64());
        let b = f64::from_bits(rng.next_u64());
        if check(a, b) {
            *decided += 1;
        }
    }
}

/// Lane 2: biased toward EXACT quotients — build `a = q·b` from small
/// integer mantissas so the witness is constantly asked to fire.
fn lane_exact_biased(n: u64, decided: &mut u64) {
    let mut rng = Rng(0x1234_5678_9ABC_DEF1);
    for _ in 0..n {
        let mq = (rng.next_u64() % (1 << 26)) as f64 + 1.0;
        let mb = (rng.next_u64() % (1 << 26)) as f64 + 1.0;
        let eq = ((rng.next_u64() % 120) as i32) - 60;
        let eb = ((rng.next_u64() % 120) as i32) - 60;
        let sign = if rng.next_u64() & 1 == 0 { 1.0 } else { -1.0 };
        let q = sign * mq * exp2i(eq);
        let b = mb * exp2i(eb);
        // a = q·b is exact whenever mq·mb < 2^53 and no over/underflow.
        let a = q * b;
        if check(a, b) {
            *decided += 1;
        }
        // Also probe the near-miss neighbours, where the witness must NOT fire.
        if check(a.next_up(), b) {
            *decided += 1;
        }
        if check(a.next_down(), b) {
            *decided += 1;
        }
    }
}

/// Lane 3: power-of-two divisors — always exact, the `v / ‖v‖`
/// axis-aligned case that motivated the witness.
fn lane_pow2_divisor(n: u64, decided: &mut u64) {
    let mut rng = Rng(0x0BAD_C0FF_EE0D_D001);
    for _ in 0..n {
        let a = f64::from_bits(rng.next_u64());
        let e = ((rng.next_u64() % 200) as i32) - 100;
        let b = exp2i(e) * if rng.next_u64() & 1 == 0 { 1.0 } else { -1.0 };
        if check(a, b) {
            *decided += 1;
        }
        // The identity case the kernel's exact-order band depends on.
        if check(1.0, 1.0) {
            *decided += 1;
        }
    }
}

/// Lane 4: subnormal and extreme-binade magnitudes, where the 2Prod
/// validity floor is supposed to make the witness DECLINE.
fn lane_extremes(n: u64, decided: &mut u64) {
    let mut rng = Rng(0xFEED_FACE_CAFE_0001);
    for _ in 0..n {
        let pick = |r: &mut Rng| -> f64 {
            let m = (r.next_u64() % (1 << 52)) as f64 + 1.0;
            let e = match r.next_u64() % 4 {
                0 => -1074 + (r.next_u64() % 60) as i32, // subnormal-ish
                1 => -1022 + (r.next_u64() % 40) as i32,
                2 => 900 + (r.next_u64() % 100) as i32, // near overflow
                _ => -60 + (r.next_u64() % 120) as i32,
            };
            let s = if r.next_u64() & 1 == 0 { 1.0 } else { -1.0 };
            s * m * exp2i(e)
        };
        let (a, b) = (pick(&mut rng), pick(&mut rng));
        if check(a, b) {
            *decided += 1;
        }
    }
}

/// `2^e` without `powi`/`powf` rounding concerns.
fn exp2i(e: i32) -> f64 {
    libm::exp2(f64::from(e))
}

fn run(per_lane: u64, label: &str) {
    let mut decided = 0u64;
    lane_random(per_lane, &mut decided);
    lane_exact_biased(per_lane, &mut decided);
    lane_pow2_divisor(per_lane, &mut decided);
    lane_extremes(per_lane, &mut decided);
    println!("{label}: {decided} cases decided exactly (4 lanes × {per_lane})");
    // The comparator declines on overflow-risky shapes; if it declined
    // essentially everywhere the test would be vacuous.
    assert!(
        decided > per_lane,
        "{label}: comparator decided only {decided} cases — harness broken"
    );
}

#[test]
fn div_witness_is_sound_and_containing_reduced() {
    run(DEFAULT_PER_LANE, "review_fuzz_div (reduced)");
}

/// The full 17.5M-case sweep the adversarial review ran (minutes, not
/// seconds — hence `#[ignore]`).
#[test]
#[ignore = "full 17.5M-case sweep; run with --ignored"]
fn div_witness_is_sound_and_containing_full() {
    run(FULL_PER_LANE, "review_fuzz_div (full)");
}
