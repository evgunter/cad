//! **Adopted adversarial-review scratch suite for the C9 ring** (M5 PR 2
//! review, APPROVE-WITH-FIX-PASS). Recreated faithfully from the review's
//! description after its scratch worktree was collected.
//!
//! Its regression value is that it is an **independent derivation**: it
//! shares no *comparator* code with `ring_interval_fuzz.rs`. (It used to
//! carry its own SplitMix64 as a second axis of independence; since the
//! 2026-08-13 fuzz audit both suites draw from the shared
//! `test_utils::fuzz` stream, which varies its seed per run — a moving
//! stream buys more than a second fixed one did. The lane structure and
//! the exact comparators below remain this file's own.) Different
//! comparators:
//!
//! - **`×`, `÷`, `powi`**: `u128` dyadic cross-comparison. Every finite
//!   `f64` is `±m·2^e` with `m` odd; a product of two of them is
//!   `±(m₁m₂)·2^(e₁+e₂)` with the mantissa product inside `u128`, so
//!   bounds are compared to exact truth by integer magnitude comparison,
//!   and quotients by cross-multiplication.
//! - **`+`, `−`**: **TwoSum distillation**. `a + b` is exactly `s + e`
//!   (`s` the rounded sum, `e` its representable error), so the sign of
//!   `x − (a+b)` is the sign of the leading component of the
//!   nonoverlapping expansion produced by growing `[−e, −s]` by `x`
//!   (Shewchuk GROW-EXPANSION; each step is an exact TwoSum). An
//!   **overflow-decides-sign guard** covers the two cases the expansion
//!   cannot represent: if the rounded sum is infinite the true sum is
//!   beyond `MAX` and its sign decides against any finite bound; if the
//!   expansion's leading term overflows, its sign is the answer.
//!
//! Lanes: sign-clamp boundary pool, catastrophic-alignment `±`,
//! tiny-divisor `÷` with signed-zero refusal, exact dyadic chains,
//! `powi` exact oracle plus structural pins, and the zero-annihilator /
//! infinite-bracket edge probe.
//!
//! The shipped run is a smoke-level subset. `CAD_FUZZ_EFFORT=64` restores
//! roughly the review's full counts, which is what the `#[ignore]`d
//! full-scale twin used to do:
//!
//! ```text
//! CAD_FUZZ_EFFORT=64 cargo test -p geom-core --test all -- review_m5_pr2_scratch --nocapture
//! ```

test_utils::gated_to![
    "crates/geom-core/src/ring_interval.rs",
    "crates/geom-core/src/real.rs"
];

use geom_core::RingInterval;
use std::cmp::Ordering;
use test_utils::fuzz;

fn subnormal(rng: &mut fuzz::Rng) -> f64 {
    let m = (rng.next_u64() & 0xf_ffff_ffff_ffff) | 1;
    let s = rng.next_u64() & (1 << 63);
    f64::from_bits(s | m)
}

/// Finite `f64` with a chosen unbiased exponent.
fn at_exp(rng: &mut fuzz::Rng, e: i32) -> f64 {
    let m = rng.next_u64() & 0xf_ffff_ffff_ffff;
    let biased = (e + 1023).clamp(1, 2046) as u64;
    let s = rng.next_u64() & (1 << 63);
    f64::from_bits(s | (biased << 52) | m)
}

// ------------------------------------------------- exact comparators

/// `(negative, odd mantissa, exponent)` with `value = ±m·2^e`; zero maps
/// to `m = 0`.
fn split(x: f64) -> (bool, u128, i32) {
    debug_assert!(x.is_finite());
    let b = x.to_bits();
    let neg = b >> 63 == 1;
    if x == 0.0 {
        return (neg, 0, 0);
    }
    let biased = ((b >> 52) & 0x7ff) as i32;
    let frac = b & 0xf_ffff_ffff_ffff;
    let (mut m, mut e) = if biased == 0 {
        (u128::from(frac), -1074)
    } else {
        (u128::from(frac | (1 << 52)), biased - 1075)
    };
    let tz = m.trailing_zeros() as i32;
    m >>= tz;
    e += tz;
    (neg, m, e)
}

/// Exact comparison of two nonzero magnitudes `m₁·2^e₁` and `m₂·2^e₂`.
/// Aligning by the difference of leading-bit positions keeps the shifted
/// mantissa inside `u128` (both inputs are at most 106 bits wide here).
fn cmp_mag(m1: u128, e1: i32, m2: u128, e2: i32) -> Ordering {
    let top1 = 128 - m1.leading_zeros() as i32 + e1;
    let top2 = 128 - m2.leading_zeros() as i32 + e2;
    if top1 != top2 {
        return top1.cmp(&top2);
    }
    let d = e1 - e2;
    if d >= 0 {
        (m1 << d).cmp(&m2)
    } else {
        m1.cmp(&(m2 << (-d)))
    }
}

/// Exact comparison of a finite-or-infinite bound `x` against the exact
/// value `±p·2^q` (`p` may be 0).
fn cmp_x_vs_term(x: f64, tneg: bool, p: u128, q: i32) -> Ordering {
    if x == f64::INFINITY {
        return Ordering::Greater;
    }
    if x == f64::NEG_INFINITY {
        return Ordering::Less;
    }
    let (xneg, xm, xe) = split(x);
    match (xm == 0, p == 0) {
        (true, true) => return Ordering::Equal,
        (true, false) => {
            return if tneg {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        }
        (false, true) => {
            return if xneg {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }
        (false, false) => {}
    }
    match (xneg, tneg) {
        (false, true) => Ordering::Greater,
        (true, false) => Ordering::Less,
        (false, false) => cmp_mag(xm, xe, p, q),
        (true, true) => cmp_mag(xm, xe, p, q).reverse(),
    }
}

/// `x` against the exact product `a·b` (both finite).
fn cmp_x_vs_prod(x: f64, a: f64, b: f64) -> Ordering {
    let (an, am, ae) = split(a);
    let (bn, bm, be) = split(b);
    if am == 0 || bm == 0 {
        return cmp_x_vs_term(x, false, 0, 0);
    }
    cmp_x_vs_term(x, an != bn, am * bm, ae + be)
}

/// `x` against the exact quotient `a/b` (`b` finite nonzero), by
/// cross-multiplication: `x <=> a/b` is `x·b <=> a`, reversed for `b < 0`.
fn cmp_x_vs_quot(x: f64, a: f64, b: f64) -> Ordering {
    if x == f64::INFINITY {
        return Ordering::Greater;
    }
    if x == f64::NEG_INFINITY {
        return Ordering::Less;
    }
    let (an, am, ae) = split(a);
    if am == 0 {
        // a/b is exactly zero.
        return cmp_x_vs_term(x, false, 0, 0);
    }
    let (bn, bm, be) = split(b);
    let (xn, xm, xe) = split(x);
    if xm == 0 {
        // x is zero: decided by the quotient's sign alone — no
        // cross-multiplication, hence NO divisor-sign reversal (applying
        // one here would double-correct).
        return if an != bn {
            Ordering::Greater
        } else {
            Ordering::Less
        };
    }
    // x·b vs a, then undo the multiplication's order flip for b < 0.
    let lhs_neg = xn != bn;
    let ord = match (lhs_neg, an) {
        (false, true) => Ordering::Greater,
        (true, false) => Ordering::Less,
        (false, false) => cmp_mag(xm * bm, xe + be, am, ae),
        (true, true) => cmp_mag(xm * bm, xe + be, am, ae).reverse(),
    };
    if b < 0.0 { ord.reverse() } else { ord }
}

/// The exact error of a rounded sum: `a + b == s + two_sum_err(a, b, s)`
/// exactly, for finite `s` (valid with no underflow caveat).
fn two_sum_err(a: f64, b: f64, s: f64) -> f64 {
    let bv = s - a;
    let av = s - bv;
    (a - av) + (b - bv)
}

/// One exact TwoSum: returns `(sum, err)` with `sum + err == a + b`.
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    (s, two_sum_err(a, b, s))
}

/// Exact ordering of a bound `x` against the true real `a + b`
/// (TwoSum distillation; module docs). `sub` negates `b` first — `f64`
/// negation is exact, so subtraction needs no separate comparator.
fn cmp_x_vs_sum(x: f64, a: f64, b: f64) -> Ordering {
    if x == f64::INFINITY {
        return Ordering::Greater;
    }
    if x == f64::NEG_INFINITY {
        return Ordering::Less;
    }
    let s = a + b;
    if !s.is_finite() {
        // Overflow decides: the true sum's magnitude exceeds MAX, so any
        // finite bound lies on the near side of it.
        return if s > 0.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    }
    let e = two_sum_err(a, b, s);
    // GROW-EXPANSION of the nonoverlapping [-e, -s] by x: the result
    // [h0, h1, q] is nonoverlapping and sums to x - (a+b) exactly.
    let (q1, h0) = two_sum(x, -e);
    let (q, h1) = two_sum(q1, -s);
    if !q.is_finite() {
        return if q > 0.0 {
            Ordering::Greater
        } else {
            Ordering::Less
        };
    }
    for c in [q, h1, h0] {
        if c > 0.0 {
            return Ordering::Greater;
        }
        if c < 0.0 {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

/// Asserts `r` brackets the exact `a + b` (or `a − b` when `sub`).
fn assert_sum_bracket(r: RingInterval, a: f64, b: f64, sub: bool, what: &str) {
    let b = if sub { -b } else { b };
    assert!(
        !r.is_poison(),
        "{what}: unexpected poison ({a:e}, {b:e}) — {}",
        fuzz::replay()
    );
    assert!(
        cmp_x_vs_sum(r.lo(), a, b) != Ordering::Greater,
        "{what} LO ABOVE TRUTH: a={a:e} b={b:e} lo={:e} — {}",
        r.lo(),
        fuzz::replay()
    );
    assert!(
        cmp_x_vs_sum(r.hi(), a, b) != Ordering::Less,
        "{what} HI BELOW TRUTH: a={a:e} b={b:e} hi={:e} — {}",
        r.hi(),
        fuzz::replay()
    );
}

/// Asserts `r` brackets the exact `a · b`.
fn assert_prod_bracket(r: RingInterval, a: f64, b: f64, what: &str) {
    assert!(
        !r.is_poison(),
        "{what}: unexpected poison ({a:e}, {b:e}) — {}",
        fuzz::replay()
    );
    assert!(
        cmp_x_vs_prod(r.lo(), a, b) != Ordering::Greater,
        "{what} LO ABOVE TRUTH: a={a:e} b={b:e} lo={:e} — {}",
        r.lo(),
        fuzz::replay()
    );
    assert!(
        cmp_x_vs_prod(r.hi(), a, b) != Ordering::Less,
        "{what} HI BELOW TRUTH: a={a:e} b={b:e} hi={:e} — {}",
        r.hi(),
        fuzz::replay()
    );
}

// ------------------------------------------------------------- lanes

/// The boundary pool: signed zeros, the smallest subnormal, the
/// normal/subnormal frontier, and the overflow frontier — every place a
/// one-step outward pad or a sign clamp can change the answer.
fn pool(rng: &mut fuzz::Rng) -> Vec<f64> {
    let mut v = vec![
        0.0,
        -0.0,
        5e-324,
        -5e-324,
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        f64::MIN_POSITIVE * (1.0 - f64::EPSILON / 2.0),
        f64::MAX,
        f64::MIN,
        1.0,
        -1.0,
    ];
    for _ in 0..8 {
        v.push(subnormal(rng));
    }
    v
}

/// Lane 1 — sign-clamp boundary fuzz. Every pair drawn from the boundary
/// pool, checked for exact containment AND for the clamp's own claim:
/// a product/quotient of two same-signed enclosures may not have a
/// lower bound below zero, and of opposite-signed ones may not have an
/// upper bound above zero.
fn lane_sign_clamp(rng: &mut fuzz::Rng, cases: usize) -> u64 {
    let p = pool(rng);
    let mut n = 0;
    for _ in 0..cases {
        let (a, b) = (p[rng.below(p.len())], p[rng.below(p.len())]);
        let (ri, si) = (RingInterval::point(a), RingInterval::point(b));
        let prod = ri * si;
        assert_prod_bracket(prod, a, b, "pool mul");
        let nonneg = (a >= 0.0 && b >= 0.0) || (a <= 0.0 && b <= 0.0);
        let nonpos = (a >= 0.0 && b <= 0.0) || (a <= 0.0 && b >= 0.0);
        if nonneg {
            assert!(
                prod.lo() >= 0.0,
                "clamp: {a:e}*{b:e} lo {:e} — {}",
                prod.lo(),
                fuzz::replay()
            );
        }
        if nonpos {
            assert!(
                prod.hi() <= 0.0,
                "clamp: {a:e}*{b:e} hi {:e} — {}",
                prod.hi(),
                fuzz::replay()
            );
        }
        // Signed zeros must not survive as a "nonzero" divisor.
        let quo = ri / si;
        if b == 0.0 {
            assert!(
                quo.is_poison(),
                "divisor {b:e} (signed zero) must poison — {}",
                fuzz::replay()
            );
        } else {
            assert!(
                !quo.is_poison(),
                "pool div: unexpected poison — {}",
                fuzz::replay()
            );
            assert!(
                cmp_x_vs_quot(quo.lo(), a, b) != Ordering::Greater,
                "pool div LO ABOVE TRUTH: {a:e}/{b:e} — {}",
                fuzz::replay()
            );
            assert!(
                cmp_x_vs_quot(quo.hi(), a, b) != Ordering::Less,
                "pool div HI BELOW TRUTH: {a:e}/{b:e} — {}",
                fuzz::replay()
            );
            if nonneg {
                assert!(
                    quo.lo() >= 0.0,
                    "clamp: {a:e}/{b:e} lo {:e} — {}",
                    quo.lo(),
                    fuzz::replay()
                );
            }
            if nonpos {
                assert!(
                    quo.hi() <= 0.0,
                    "clamp: {a:e}/{b:e} hi {:e} — {}",
                    quo.hi(),
                    fuzz::replay()
                );
            }
        }
        n += 2;
    }
    n
}

/// Lane 2 — catastrophic alignment. A huge normal against a subnormal:
/// the exact sum needs ~2100 bits, which is exactly what the TwoSum
/// distillation handles and a naive re-evaluation cannot.
fn lane_alignment(rng: &mut fuzz::Rng, cases: usize) -> u64 {
    let mut n = 0;
    for _ in 0..cases {
        let e = 967 + (rng.below(57) as i32);
        let big = at_exp(rng, e);
        let tiny = if rng.next_u64() & 1 == 0 {
            subnormal(rng)
        } else {
            f64::from_bits(1) * if rng.next_u64() & 1 == 0 { 1.0 } else { -1.0 }
        };
        let (bi, ti) = (RingInterval::point(big), RingInterval::point(tiny));
        assert_sum_bracket(bi + ti, big, tiny, false, "align add");
        assert_sum_bracket(bi - ti, big, tiny, true, "align sub");
        assert_sum_bracket(ti + bi, tiny, big, false, "align add rev");
        assert_sum_bracket(ti - bi, tiny, big, true, "align sub rev");
        n += 4;
    }
    n
}

/// Lane 3 — tiny divisors and the signed-zero refusal. A divisor one
/// step from zero is where a quotient's outward pad is largest, and a
/// bracket that merely *touches* zero (including `[-0.0, +0.0]`) must
/// refuse rather than produce a half-line.
fn lane_tiny_div(rng: &mut fuzz::Rng, cases: usize) -> u64 {
    let mut n = 0;
    for _ in 0..cases {
        let ne = rng.below(200) as i32 - 100;
        let num = at_exp(rng, ne);
        let den = match rng.below(4) {
            0 => f64::from_bits(1),
            1 => -f64::from_bits(1),
            2 => subnormal(rng),
            _ => f64::MIN_POSITIVE * if rng.next_u64() & 1 == 0 { 1.0 } else { -1.0 },
        };
        let q = RingInterval::point(num) / RingInterval::point(den);
        assert!(
            !q.is_poison(),
            "tiny divisor {den:e} must not poison — {}",
            fuzz::replay()
        );
        assert!(
            cmp_x_vs_quot(q.lo(), num, den) != Ordering::Greater,
            "tiny div LO ABOVE TRUTH: {num:e}/{den:e} — {}",
            fuzz::replay()
        );
        assert!(
            cmp_x_vs_quot(q.hi(), num, den) != Ordering::Less,
            "tiny div HI BELOW TRUTH: {num:e}/{den:e} — {}",
            fuzz::replay()
        );
        // Zero-touching brackets, every spelling.
        for (lo, hi) in [
            (0.0, 0.0),
            (-0.0, 0.0),
            (0.0f64, f64::from_bits(1)),
            (-f64::from_bits(1), 0.0),
            (-f64::from_bits(1), f64::from_bits(1)),
            (-0.0, f64::MIN_POSITIVE),
        ] {
            let d = RingInterval::from_bounds(lo, hi);
            assert!(
                (RingInterval::point(num) / d).is_poison(),
                "divisor [{lo:e}, {hi:e}] touches zero and must refuse"
            );
        }
        n += 7;
    }
    n
}

/// A short-mantissa dyadic `±m·2^e`, exact in `f64` and exactly
/// representable as `(neg, m, e)` for the `u128` comparators.
fn dyadic(rng: &mut fuzz::Rng, mant_bits: u32, exp_span: i32) -> (f64, bool, u128, i32) {
    let m = u128::from((rng.next_u64() & ((1u64 << mant_bits) - 1)) | 1);
    let e = rng.below(2 * exp_span as usize + 1) as i32 - exp_span;
    let neg = rng.next_u64() & 1 == 1;
    let mut v = m as f64;
    for _ in 0..e.abs() {
        v = if e > 0 { v * 2.0 } else { v * 0.5 };
    }
    (if neg { -v } else { v }, neg, m, e)
}

/// Lane 4 — exact dyadic chains. `(a+b)·c` and `a·b+c` on short-mantissa
/// dyadics: the ring's two-step enclosure must still contain the exact
/// real value of the whole chain, which is what certification composes.
fn lane_chains(rng: &mut fuzz::Rng, cases: usize) -> u64 {
    let mut n = 0;
    for _ in 0..cases {
        let (a, an, am, ae) = dyadic(rng, 20, 20);
        let (b, bn, bm, be) = dyadic(rng, 20, 20);
        let (c, cn, cm, ce) = dyadic(rng, 20, 20);
        let (ri, si, ti) = (
            RingInterval::point(a),
            RingInterval::point(b),
            RingInterval::point(c),
        );
        // (a+b)·c — the exact sum is a dyadic with at most ~140 bits of
        // mantissa once aligned, so it is compared through the sum
        // comparator on the two factors of the product instead: bound
        // the product's endpoints against (a+b) scaled by c.
        let chain1 = (ri + si) * ti;
        assert!(!chain1.is_poison(), "(a+b)*c poisoned — {}", fuzz::replay());
        // exact (a+b): align the two dyadics to the common exponent.
        let Some((sneg, sm, se)) = add_dyadic(an, am, ae, bn, bm, be) else {
            continue;
        };
        if sm != 0 && cm != 0 {
            let (pneg, pm, pe) = (sneg != cn, sm.checked_mul(cm), se + ce);
            if let Some(pm) = pm {
                assert!(
                    cmp_x_vs_term(chain1.lo(), pneg, pm, pe) != Ordering::Greater,
                    "(a+b)*c LO ABOVE TRUTH — {}",
                    fuzz::replay()
                );
                assert!(
                    cmp_x_vs_term(chain1.hi(), pneg, pm, pe) != Ordering::Less,
                    "(a+b)*c HI BELOW TRUTH — {}",
                    fuzz::replay()
                );
                n += 1;
            }
        }
        // a·b + c
        let chain2 = ri * si + ti;
        assert!(!chain2.is_poison(), "a*b+c poisoned — {}", fuzz::replay());
        if let Some(abm) = am.checked_mul(bm)
            && let Some((tneg, tm, te)) = add_dyadic(an != bn, abm, ae + be, cn, cm, ce)
        {
            assert!(
                cmp_x_vs_term(chain2.lo(), tneg, tm, te) != Ordering::Greater,
                "a*b+c LO ABOVE TRUTH — {}",
                fuzz::replay()
            );
            assert!(
                cmp_x_vs_term(chain2.hi(), tneg, tm, te) != Ordering::Less,
                "a*b+c HI BELOW TRUTH — {}",
                fuzz::replay()
            );
            n += 1;
        }
    }
    n
}

/// Exact sum of two dyadics `±m·2^e` as another dyadic, when the aligned
/// mantissas fit `u128` (the lane's short mantissas guarantee it for the
/// exponent spans used; otherwise the term is returned as zero-magnitude
/// and the caller skips).
fn add_dyadic(
    an: bool,
    am: u128,
    ae: i32,
    bn: bool,
    bm: u128,
    be: i32,
) -> Option<(bool, u128, i32)> {
    if am == 0 {
        return Some((bn, bm, be));
    }
    if bm == 0 {
        return Some((an, am, ae));
    }
    let e = ae.min(be);
    let (sa, sb) = ((ae - e) as u32, (be - e) as u32);
    if sa > 100 || sb > 100 {
        // Alignment beyond the lane's design range: report "no exact
        // value available" rather than a zero the caller would mistake
        // for a genuine cancellation.
        return None;
    }
    let (Some(av), Some(bv)) = (am.checked_shl(sa), bm.checked_shl(sb)) else {
        return None;
    };
    Some(match (an, bn) {
        (x, y) if x == y => (an, av.checked_add(bv)?, e),
        _ => {
            let (pos, neg) = if an { (bv, av) } else { (av, bv) };
            if pos >= neg {
                (false, pos - neg, e)
            } else {
                (true, neg - pos, e)
            }
        }
    })
}

/// Lane 5 — `powi` against an exact `u128` oracle plus the structural
/// pins the review asked for.
fn lane_powi(rng: &mut fuzz::Rng, cases: usize) -> u64 {
    let mut n = 0;
    for _ in 0..cases {
        // Odd mantissa ≤ 13 keeps m^n inside u128 for large n.
        let m = [1u128, 3, 5, 7, 9, 11, 13][rng.below(7)];
        let e = rng.below(41) as i32 - 20;
        let neg = rng.next_u64() & 1 == 1;
        let mut v = m as f64;
        for _ in 0..e.abs() {
            v = if e > 0 { v * 2.0 } else { v * 0.5 };
        }
        let base = if neg { -v } else { v };
        let x = RingInterval::point(base);
        for k in 0..=64u32 {
            let Some(mp) = m.checked_pow(k) else { break };
            let n_i = k as i32;
            let tneg = neg && (k % 2 == 1);
            let r = x.powi(n_i);
            assert!(
                !r.is_poison(),
                "{base:e}^{n_i}: unexpected poison — {}",
                fuzz::replay()
            );
            assert!(
                cmp_x_vs_term(r.lo(), tneg, mp, e * n_i) != Ordering::Greater,
                "powi LO above truth: {base:e}^{n_i} — {}",
                fuzz::replay()
            );
            assert!(
                cmp_x_vs_term(r.hi(), tneg, mp, e * n_i) != Ordering::Less,
                "powi HI below truth: {base:e}^{n_i} — {}",
                fuzz::replay()
            );
            n += 1;
            // Negative exponent: truth is 1/(±mp·2^(e·k)); cross-multiply.
            if k > 0 && k <= 20 {
                let rn = x.powi(-n_i);
                assert!(
                    !rn.is_poison(),
                    "{base:e}^-{k} poisoned — {}",
                    fuzz::replay()
                );
                let cmp = |b: f64| -> Ordering {
                    if b == f64::INFINITY {
                        return Ordering::Greater;
                    }
                    if b == f64::NEG_INFINITY {
                        return Ordering::Less;
                    }
                    let (bn2, bm2, be2) = split(b);
                    if bm2 == 0 {
                        return if tneg {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        };
                    }
                    let Some(prod) = bm2.checked_mul(mp) else {
                        return Ordering::Equal; // out of range: skip
                    };
                    let lhs_neg = bn2 != tneg;
                    let ord = if lhs_neg {
                        Ordering::Less
                    } else {
                        cmp_mag(prod, be2 + e * n_i, 1, 0)
                    };
                    if tneg { ord.reverse() } else { ord }
                };
                assert!(
                    cmp(rn.lo()) != Ordering::Greater,
                    "powi neg LO: {base:e}^-{k} — {}",
                    fuzz::replay()
                );
                assert!(
                    cmp(rn.hi()) != Ordering::Less,
                    "powi neg HI: {base:e}^-{k} — {}",
                    fuzz::replay()
                );
                n += 1;
            }
        }
        // Pin: powi(1) is bit-identical to the input bracket.
        let w = RingInterval::from_bounds(base.min(1.0), base.max(1.0));
        let p1 = w.powi(1);
        assert_eq!(
            p1.lo().to_bits(),
            w.lo().to_bits(),
            "powi(1) lo not identical — {}",
            fuzz::replay()
        );
        assert_eq!(
            p1.hi().to_bits(),
            w.hi().to_bits(),
            "powi(1) hi not identical — {}",
            fuzz::replay()
        );
        n += 1;
    }
    n
}

/// Lane 5b — the straddling-even-power pins: the lower bound is `+0.0`
/// *including its sign bit* (a `-0.0` would still compare equal to zero
/// but would sign-flip a downstream reciprocal), and `powi` never
/// escapes the naive repeated-multiplication chain's enclosure.
fn lane_powi_pins(rng: &mut fuzz::Rng, cases: usize) -> u64 {
    let mut n = 0;
    for _ in 0..cases {
        let (lo, hi) = {
            let ea = rng.below(80) as i32 - 40;
            let a = at_exp(rng, ea);
            let eb = rng.below(80) as i32 - 40;
            let b = at_exp(rng, eb);
            (-a.abs(), b.abs())
        };
        if !(lo < 0.0 && hi > 0.0) {
            continue;
        }
        let x = RingInterval::from_bounds(lo, hi);
        for k in [2i32, 4, 6, 8, 10, 12, 14, 16] {
            let p = x.powi(k);
            if p.is_poison() {
                continue;
            }
            assert_eq!(
                p.lo().to_bits(),
                0.0f64.to_bits(),
                "even power lower bound is not +0.0 for [{lo:e}, {hi:e}]^{k} — {}",
                fuzz::replay()
            );
            n += 1;
        }
        // Naive chain overlap, n = 2..16: the two enclose the same truth,
        // so they must intersect (powi is the tighter of the two).
        let mut naive = x;
        for k in 2i32..=16 {
            naive = naive * x;
            let p = x.powi(k);
            if p.is_poison() || naive.is_poison() {
                continue;
            }
            assert!(
                p.lo() <= naive.hi() && naive.lo() <= p.hi(),
                "powi({k}) and the naive chain are disjoint — {}",
                fuzz::replay()
            );
            n += 1;
        }
    }
    n
}

/// Lane 6 — the zero annihilator and the infinite-bracket edge. After
/// the fix pass a bracket whose closed side is infinite contains no real
/// number and must refuse to construct.
fn lane_edges(rng: &mut fuzz::Rng, cases: usize) -> u64 {
    let mut n = 0;
    let z = RingInterval::zero();
    for _ in 0..cases {
        let ea = rng.below(2000) as i32 - 1000;
        let a = at_exp(rng, ea);
        let x = RingInterval::point(a);
        for r in [z * x, x * z, z / x] {
            assert!(
                r.lo().to_bits() == 0.0f64.to_bits() && r.hi().to_bits() == 0.0f64.to_bits(),
                "zero annihilator did not give exact +0.0 for {a:e} — {}",
                fuzz::replay()
            );
        }
        // A wide bracket times exact zero is still exactly zero.
        let wide = RingInterval::from_bounds(-a.abs(), a.abs());
        assert!(
            (z * wide).width() == 0.0,
            "zero times a wide bracket is not exactly zero — {}",
            fuzz::replay()
        );
        n += 4;
    }
    for (lo, hi) in [
        (f64::INFINITY, f64::INFINITY),
        (f64::NEG_INFINITY, f64::NEG_INFINITY),
        (f64::INFINITY, f64::NEG_INFINITY),
    ] {
        assert!(
            RingInterval::from_bounds(lo, hi).is_poison(),
            "[{lo:e}, {hi:e}] contains no real number and must poison"
        );
        n += 1;
    }
    // The open-sided brackets stay legal.
    assert!(!RingInterval::from_bounds(f64::NEG_INFINITY, 0.0).is_poison());
    assert!(!RingInterval::from_bounds(0.0, f64::INFINITY).is_poison());
    // And an infinite-sided divisor still refuses when it touches zero.
    assert!((RingInterval::point(1.0) / RingInterval::from_bounds(0.0, f64::INFINITY)).is_poison());
    n + 3
}

/// The review's lane RATIOS at the shipped smoke scale; `CAD_FUZZ_EFFORT`
/// buys the depth the `#[ignore]`d full twin used to hold (EFFORT=64 is
/// approximately the review's original counts).
#[test]
fn review_scratch_ring_lanes() {
    let mut rng = fuzz::start("review_m5_pr2_scratch::ring_lanes");
    // Fixed lane order (D9): the stream is consumed in this order, so a
    // low-effort run is a prefix-compatible subset of a high-effort one.
    let totals = [
        (
            "sign-clamp pool",
            lane_sign_clamp(&mut rng, fuzz::scaled(9_375)),
        ),
        (
            "catastrophic align",
            lane_alignment(&mut rng, fuzz::scaled(15_625)),
        ),
        ("tiny divisor", lane_tiny_div(&mut rng, fuzz::scaled(3_125))),
        ("dyadic chains", lane_chains(&mut rng, fuzz::scaled(9_375))),
        ("powi exact", lane_powi(&mut rng, fuzz::scaled(63))),
        ("powi pins", lane_powi_pins(&mut rng, fuzz::scaled(625))),
        ("edges", lane_edges(&mut rng, fuzz::scaled(625))),
    ];
    let sum: u64 = totals.iter().map(|(_, n)| n).sum();
    let detail: Vec<String> = totals
        .iter()
        .map(|(name, n)| format!("{name} {n}"))
        .collect();
    println!(
        "[review-scratch] {sum} verdicts, 0 violations — {}",
        detail.join(", ")
    );
    assert!(sum > 0, "no lane produced a verdict — {}", fuzz::replay());
}
