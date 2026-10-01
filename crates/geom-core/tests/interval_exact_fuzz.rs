//! **Exact-arithmetic soundness fuzz of certification arithmetic** (M5 PR 2,
//! acceptance family 1).
//!
//! Every containment claim below is checked against the **exact** value
//! of the operation over the reals — never against a second
//! floating-point evaluation, which would only prove interval arithmetic agrees
//! with the thing it is supposed to bound.
//!
//! # Comparator provenance
//!
//! The technique — decompose `f64` into `(sign, odd mantissa, exponent)`
//! and compare by exact integer arithmetic, with division handled by
//! cross-multiplication — is adopted from the M5 PR 1 adversarial
//! review's harness, `interval-transcendentals/tests/review_fuzz_exact.rs`
//! (branch `ev/m5-pr1-interval-adoption`), and its `decomp` is
//! reproduced here. Its `cmp_mag` cross-multiplication suffices for
//! division and multiplication (two 53-bit mantissas multiply into
//! 106 bits, which `u128` holds), but **not** for addition and
//! subtraction: aligning `2^1023` with `2^-1074` needs ~2100 bits. So
//! the comparator is generalised to a small fixed-width signed integer
//! ([`Big`], 72 × 64 = 4608 bits, no allocation) that represents every
//! exact `f64 ± f64` and `f64 · f64` result exactly and uniformly. Same
//! technique, wider register.
//!
//! # What is checked
//!
//! For each of `+ − × ÷` and `powi`:
//! - **Point soundness**: `point(a) ∘ point(b)` contains the exact real
//!   `a ∘ b` (both endpoints compared exactly), and contains the `f64`
//!   round-to-nearest result (legitimate here — these four operations
//!   are correctly rounded by IEEE 754, unlike transcendentals; see the
//!   `Bounds` docs on why that assertion is *not* made for libm
//!   results).
//! - **Interval soundness**: for random brackets, the exact `x ∘ y` of
//!   sampled members is contained.
//! - **Refusal paths**: NaN/inverted construction, and division by a
//!   divisor that straddles or touches zero.
//!
//! # Depth
//!
//! Counts are multiples of `test_utils::fuzz`'s EFFORT dial and the seed
//! varies per run (both logged by `fuzz::start`). The shipped level is a
//! smoke sweep; `CAD_FUZZ_EFFORT=64` restores roughly the full sweep this
//! file used to keep behind an `#[ignore]`d twin, and the dial replaces
//! that twin entirely:
//!
//! ```text
//! CAD_FUZZ_EFFORT=64 cargo test -p geom-core --test all -- interval_exact_fuzz --nocapture
//! ```

test_utils::gated_to![
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/real.rs"
];

use geom_core::Bounds;
use geom_core::Interval;
use geom_core::interval::certification::Certification;
use std::cmp::Ordering;
use test_utils::fuzz;

// ---------------------------------------------------------------- Big

/// Limb count: the widest exact value handled is a product of two
/// finite `f64`s, spanning exponents `[-2148, 2048]` — 4196 bits.
const LIMBS: usize = 72;
/// The fixed binary point: `value = Σ mag[i]·2^(64i) · 2^OFFSET`.
const OFFSET: i32 = -2200;

/// A fixed-width signed binary integer, exact for every value this test
/// forms. Sign-magnitude, little-endian limbs.
#[derive(Clone, Copy)]
struct Big {
    neg: bool,
    mag: [u64; LIMBS],
}

impl Big {
    fn zero() -> Self {
        Self {
            neg: false,
            mag: [0; LIMBS],
        }
    }

    /// `±m · 2^e`, exactly. Panics only on an exponent outside the
    /// representable window, which the callers' inputs cannot produce.
    fn term(neg: bool, m: u128, e: i32) -> Self {
        let mut out = Self::zero();
        if m == 0 {
            return out;
        }
        out.neg = neg;
        let shift = e - OFFSET;
        assert!(shift >= 0, "exponent {e} below the Big window");
        let (limb, bit) = ((shift as usize) / 64, (shift as usize) % 64);
        // m occupies ≤ 128 bits; with the intra-limb offset it touches
        // at most three limbs.
        let parts = [m as u64, (m >> 64) as u64];
        for (i, p) in parts.into_iter().enumerate() {
            if p == 0 {
                continue;
            }
            let idx = limb + i;
            assert!(idx + 1 < LIMBS, "exponent {e} above the Big window");
            out.mag[idx] |= p << bit;
            if bit > 0 {
                out.mag[idx + 1] |= p >> (64 - bit);
            }
        }
        out
    }

    fn is_zero(&self) -> bool {
        self.mag.iter().all(|l| *l == 0)
    }

    fn cmp_mag(&self, other: &Self) -> Ordering {
        for i in (0..LIMBS).rev() {
            match self.mag[i].cmp(&other.mag[i]) {
                Ordering::Equal => {}
                o => return o,
            }
        }
        Ordering::Equal
    }

    /// `self += other`, exact (no limb can overflow: every value stays
    /// far inside the window).
    fn add(&self, other: &Self) -> Self {
        if self.neg == other.neg {
            let mut out = *self;
            let mut carry = 0u64;
            for i in 0..LIMBS {
                let (s, c1) = out.mag[i].overflowing_add(other.mag[i]);
                let (s, c2) = s.overflowing_add(carry);
                out.mag[i] = s;
                carry = u64::from(c1) + u64::from(c2);
            }
            assert!(carry == 0, "Big overflow");
            return out;
        }
        // Opposite signs: larger magnitude minus smaller.
        let (big, small, neg) = match self.cmp_mag(other) {
            Ordering::Less => (other, self, other.neg),
            _ => (self, other, self.neg),
        };
        let mut out = *big;
        out.neg = neg;
        let mut borrow = 0u64;
        for i in 0..LIMBS {
            let (d, b1) = out.mag[i].overflowing_sub(small.mag[i]);
            let (d, b2) = d.overflowing_sub(borrow);
            out.mag[i] = d;
            borrow = u64::from(b1) + u64::from(b2);
        }
        assert!(borrow == 0, "Big underflow");
        if out.is_zero() {
            out.neg = false;
        }
        out
    }

    fn negated(&self) -> Self {
        let mut out = *self;
        if !out.is_zero() {
            out.neg = !out.neg;
        }
        out
    }

    /// Sign as an ordering against zero.
    fn sign(&self) -> Ordering {
        if self.is_zero() {
            Ordering::Equal
        } else if self.neg {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

/// Decompose a finite `f64` into `(negative, odd mantissa, exponent)`
/// with `value = ±m·2^e`. Reproduced from the PR 1 review harness
/// (module docs).
fn decomp(x: f64) -> (bool, u128, i32) {
    assert!(x.is_finite());
    let bits = x.to_bits();
    let neg = bits >> 63 == 1;
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & 0xf_ffff_ffff_ffff;
    if x == 0.0 {
        return (neg, 0, 0);
    }
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

/// The exact value of a finite `f64`.
fn big_of(x: f64) -> Big {
    let (neg, m, e) = decomp(x);
    Big::term(neg, m, e)
}

/// The exact product of two finite `f64`s (mantissas ≤ 53 bits each, so
/// the product fits `u128`).
fn big_prod(a: f64, b: f64) -> Big {
    let (an, am, ae) = decomp(a);
    let (bn, bm, be) = decomp(b);
    Big::term(an != bn, am * bm, ae + be)
}

/// Exact ordering of an `f64` bound against an exact value.
fn cmp_f64_vs_big(x: f64, v: &Big) -> Ordering {
    if x == f64::INFINITY {
        return Ordering::Greater;
    }
    if x == f64::NEG_INFINITY {
        return Ordering::Less;
    }
    big_of(x).add(&v.negated()).sign()
}

/// Exact ordering of an `f64` bound against the quotient `a / b`
/// (`b != 0`), by cross-multiplication: `x <=> a/b` is `x·b <=> a` with
/// the order reversed when `b < 0`.
fn cmp_f64_vs_quot(x: f64, a: f64, b: f64) -> Ordering {
    if x == f64::INFINITY {
        return Ordering::Greater;
    }
    if x == f64::NEG_INFINITY {
        return Ordering::Less;
    }
    let s = big_prod(x, b).add(&big_of(a).negated()).sign();
    if b < 0.0 { s.reverse() } else { s }
}

/// Asserts `[r.lo(), r.hi()]` brackets the exact value `v`.
fn assert_brackets(r: Interval, v: &Big, what: &str) {
    assert!(
        r.is_certified(),
        "{what}: unexpected refusal — {}",
        fuzz::replay()
    );
    assert!(
        cmp_f64_vs_big(r.lo(), v) != Ordering::Greater,
        "{what}: LO ABOVE TRUTH ({:e}) — {}",
        r.lo(),
        fuzz::replay()
    );
    assert!(
        cmp_f64_vs_big(r.hi(), v) != Ordering::Less,
        "{what}: HI BELOW TRUTH ({:e}) — {}",
        r.hi(),
        fuzz::replay()
    );
}

/// The four ring operations on degenerate points, against exact truth
/// and against the (correctly rounded) `f64` result.
fn check_point_ops(a: f64, b: f64) {
    let (pa, pb) = (Interval::point(a), Interval::point(b));
    let ba = big_of(a);
    let bb = big_of(b);

    let sum = pa + pb;
    assert_brackets(sum, &ba.add(&bb), &format!("{a:e} + {b:e}"));
    let dif = pa - pb;
    assert_brackets(dif, &ba.add(&bb.negated()), &format!("{a:e} - {b:e}"));
    let pro = pa * pb;
    assert_brackets(pro, &big_prod(a, b), &format!("{a:e} * {b:e}"));

    // The RN results are correctly rounded, hence contained.
    for (r, rn, what) in [
        (sum, a + b, "add"),
        (dif, a - b, "sub"),
        (pro, a * b, "mul"),
    ] {
        if rn.is_finite() {
            assert!(
                r.contains(rn),
                "{what}: RN result {rn:e} not contained — {}",
                fuzz::replay()
            );
        }
    }

    let quo = pa / pb;
    if b == 0.0 {
        assert!(
            !quo.is_certified(),
            "division by zero must refuse — {}",
            fuzz::replay()
        );
        return;
    }
    assert!(
        quo.is_certified(),
        "{a:e} / {b:e}: unexpected refusal — {}",
        fuzz::replay()
    );
    assert!(
        cmp_f64_vs_quot(quo.lo(), a, b) != Ordering::Greater,
        "div: LO ABOVE TRUTH a={a:e} b={b:e} lo={:e} — {}",
        quo.lo(),
        fuzz::replay()
    );
    assert!(
        cmp_f64_vs_quot(quo.hi(), a, b) != Ordering::Less,
        "div: HI BELOW TRUTH a={a:e} b={b:e} hi={:e} — {}",
        quo.hi(),
        fuzz::replay()
    );
    let rn = a / b;
    if rn.is_finite() {
        assert!(
            quo.contains(rn),
            "div: RN result {rn:e} not contained — {}",
            fuzz::replay()
        );
    }
}

/// The four ring operations on non-degenerate brackets: every exact
/// combination of sampled members must be contained.
fn check_interval_ops(a0: f64, a1: f64, b0: f64, b1: f64) {
    let (alo, ahi) = if a0 <= a1 { (a0, a1) } else { (a1, a0) };
    let (blo, bhi) = if b0 <= b1 { (b0, b1) } else { (b1, b0) };
    let a = Interval::from_bounds(alo, ahi);
    let b = Interval::from_bounds(blo, bhi);
    let mid = |lo: f64, hi: f64| {
        let m = lo + (hi - lo) * 0.5;
        if m.is_finite() && lo <= m && m <= hi {
            m
        } else {
            lo
        }
    };
    let xs = [alo, ahi, mid(alo, ahi)];
    let ys = [blo, bhi, mid(blo, bhi)];
    let (sum, dif, pro, quo) = (a + b, a - b, a * b, a / b);
    for x in xs {
        for y in ys {
            let (bx, by) = (big_of(x), big_of(y));
            assert_brackets(sum, &bx.add(&by), "interval add");
            assert_brackets(dif, &bx.add(&by.negated()), "interval sub");
            assert_brackets(pro, &big_prod(x, y), "interval mul");
            if blo > 0.0 || bhi < 0.0 {
                assert!(
                    quo.is_certified(),
                    "interval div: unexpected refusal — {}",
                    fuzz::replay()
                );
                assert!(
                    cmp_f64_vs_quot(quo.lo(), x, y) != Ordering::Greater,
                    "interval div: LO ABOVE TRUTH — {}",
                    fuzz::replay()
                );
                assert!(
                    cmp_f64_vs_quot(quo.hi(), x, y) != Ordering::Less,
                    "interval div: HI BELOW TRUTH — {}",
                    fuzz::replay()
                );
            } else {
                assert!(
                    !quo.is_certified(),
                    "zero-straddling divisor must refuse — {}",
                    fuzz::replay()
                );
            }
        }
    }
}

// -------------------------------------------------------------- lanes

/// A raw bit pattern reinterpreted as `f64` — every exponent, subnormals,
/// signed zeros, NaNs and infinities included (the callers filter).
fn f64_raw(rng: &mut fuzz::Rng) -> f64 {
    f64::from_bits(rng.next_u64())
}

/// Finite `f64` with the exponent forced near `center` (± 32 binades).
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

/// A short-mantissa dyadic `±m·2^e` with `m` odd and `< 2^10` — small
/// enough that `m^12` still fits `u128`, which is what makes the
/// `powi` lane's exact oracle possible.
fn short_dyadic(rng: &mut fuzz::Rng, exp_span: i32) -> (f64, bool, u128, i32) {
    let m = ((rng.next_u64() & 0x3ff) | 1) as u128;
    let e = (rng.next_u64() % (2 * exp_span as u64 + 1)) as i32 - exp_span;
    let neg = rng.next_u64() & 1 == 1;
    let v = (m as f64) * pow2(e) * if neg { -1.0 } else { 1.0 };
    (v, neg, m, e)
}

/// Exact power of two (no `powi`, which would round).
fn pow2(e: i32) -> f64 {
    if (-1022..=1023).contains(&e) {
        f64::from_bits(((e + 1023) as u64) << 52)
    } else if e < -1022 {
        // Subnormal: build by repeated halving from 2^-1022 (exact).
        let mut v = f64::from_bits(1u64 << 52);
        for _ in 0..(-1022 - e) {
            v *= 0.5;
        }
        v
    } else {
        f64::INFINITY
    }
}

fn sweep(rng: &mut fuzz::Rng) {
    let mut n = 0u64;

    // Lane 1: raw random bit patterns — full exponent sweep, subnormals,
    // signed zeros, everything. THE COST CENTRE of this file, and the
    // one constant to turn if it becomes the suite's most expensive
    // test: it dominates the other three lanes combined.
    //
    // Sized for BREADTH OVER RUNS rather than depth per run. While the
    // seed was pinned this was a replay corpus, so a single run's case
    // count was the whole coverage there would ever be and it had to be
    // large. With a varying seed, successive runs explore different
    // patterns, and `CAD_FUZZ_EFFORT` buys depth deliberately when a
    // change to `interval.rs` actually wants it.
    for _ in 0..fuzz::scaled(23_437) {
        let (a, b) = (f64_raw(rng), f64_raw(rng));
        if !a.is_finite() || !b.is_finite() {
            continue;
        }
        check_point_ops(a, b);
        n += 1;
    }

    // Lane 2: magnitude windows, including the subnormal boundary and
    // near-MAX (where widening meets overflow).
    for center in [-1074i32, -1022, -600, -60, 0, 60, 600, 1020] {
        for _ in 0..fuzz::scaled(1_875) {
            let a = f64_near_exp(rng, center);
            let bc = (rng.next_u64() % 41) as i32 - 20;
            let b = f64_near_exp(rng, bc);
            check_point_ops(a, b);
            let s1 = subnormal(rng);
            check_point_ops(a, s1);
            let s2 = subnormal(rng);
            check_point_ops(s2, b);
            n += 3;
        }
    }

    // Lane 3: non-degenerate brackets (the four-corner logic).
    for _ in 0..fuzz::scaled(6_250) {
        let (a0, a1) = (f64_raw(rng), f64_raw(rng));
        let (b0, b1) = (f64_raw(rng), f64_raw(rng));
        if !(a0.is_finite() && a1.is_finite() && b0.is_finite() && b1.is_finite()) {
            continue;
        }
        check_interval_ops(a0, a1, b0, b1);
        n += 1;
    }

    // Lane 4: signed zeros, exact dyadics, and targeted edges.
    //
    // A third list of this shape lives in
    // `interval_backend_differential.rs` (`CORNERS`, which adds the
    // infinities and the overflowing magnitudes its verdict classes
    // need) and a fourth in `interval-transcendentals`'
    // `review_fuzz_exact.rs` (`EDGE_MAGNITUDES`, which adds the 2Prod
    // witness floor). They are not shared: the backend is its own
    // workspace, and each list is chosen for the property its lane
    // asserts. Adding a magnitude here is a reason to look at the
    // other two, not to assume they follow.
    let edges = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        0.5,
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        f64::from_bits(1),
        f64::MAX,
        f64::MIN,
        2.0f64.powi(-1074),
    ];
    for a in edges {
        for b in edges {
            check_point_ops(a, b);
            n += 1;
        }
        for _ in 0..fuzz::scaled(313) {
            let b = f64_raw(rng);
            if b.is_finite() {
                check_point_ops(a, b);
                check_point_ops(b, a);
                n += 2;
            }
        }
    }

    println!(
        "[interval-fuzz] {n} exact-comparator cases (~{} endpoint comparisons), \
         0 containment violations",
        n * 8
    );
}

/// `powi` against an exact oracle. The base is a short-mantissa dyadic
/// `±m·2^e`, so `(±m)^n · 2^(en)` is exact in `u128` for `|n| ≤ 12`;
/// negative exponents are checked by cross-multiplication against 1.
fn check_powi(v: f64, neg: bool, m: u128, e: i32, n: i32) {
    let r = Interval::point(v).powi(n);
    let k = n.unsigned_abs();
    let mut mp: u128 = 1;
    for _ in 0..k {
        mp *= m;
    }
    let sneg = neg && (k % 2 == 1);
    if n >= 0 {
        let exact = Big::term(sneg, mp, e * n);
        assert_brackets(r, &exact, &format!("{v:e}^{n}"));
        return;
    }
    // n < 0: truth is 1 / (±mp·2^(e·k)). Compare a bound x against it by
    // cross-multiplication with the (nonzero, sign-known) denominator.
    assert!(
        r.is_certified(),
        "{v:e}^{n}: unexpected refusal — {}",
        fuzz::replay()
    );
    let one = Big::term(false, 1, 0);
    let cmp = |x: f64| -> Ordering {
        if x == f64::INFINITY {
            return Ordering::Greater;
        }
        if x == f64::NEG_INFINITY {
            return Ordering::Less;
        }
        let (xn, xm, xe) = decomp(x);
        let prod = Big::term(xn != sneg, xm * mp, xe + e * (-n));
        let s = prod.add(&one.negated()).sign();
        if sneg { s.reverse() } else { s }
    };
    assert!(
        cmp(r.lo()) != Ordering::Greater,
        "powi LO above truth — {}",
        fuzz::replay()
    );
    assert!(
        cmp(r.hi()) != Ordering::Less,
        "powi HI below truth — {}",
        fuzz::replay()
    );
}

#[test]
fn ring_ops_are_sound_against_exact_arithmetic() {
    let mut rng = fuzz::start("interval_exact_fuzz::ring_ops");
    sweep(&mut rng);
}

#[test]
fn powi_is_sound_against_exact_arithmetic() {
    let mut rng = fuzz::start("interval_exact_fuzz::powi");
    let mut n_cases = 0u64;
    for _ in 0..fuzz::scaled(5_000) {
        let (v, neg, m, e) = short_dyadic(&mut rng, 60);
        for n in -6i32..=12 {
            check_powi(v, neg, m, e, n);
            n_cases += 1;
        }
    }
    // The even-power rule on zero-straddling brackets, at scale: the
    // lower bound is exactly 0 and the upper bound dominates both ends.
    for _ in 0..fuzz::scaled(25_000) {
        let (lo, hi) = (f64_raw(&mut rng), f64_raw(&mut rng));
        if !(lo.is_finite() && hi.is_finite()) || !(lo < 0.0 && hi > 0.0) {
            continue;
        }
        let x = Interval::from_bounds(lo, hi);
        for n in [2i32, 4, 6, 8, 10] {
            let p = x.powi(n);
            if !p.is_certified() {
                continue; // overflow to an indeterminate corner: honest
            }
            assert!(
                p.lo() == 0.0,
                "even power lower bound {} != 0 — {}",
                p.lo(),
                fuzz::replay()
            );
            n_cases += 1;
        }
    }
    println!("[powi] {n_cases} exact/structural cases, 0 violations");
}

#[test]
fn refusal_paths_are_total() {
    let mut rng = fuzz::start("interval_exact_fuzz::refusal");
    let mut n = 0u64;
    for _ in 0..fuzz::scaled(25_000) {
        let a = f64_raw(&mut rng);
        let b = f64_raw(&mut rng);
        // Non-finite points are refused, and the refusal flows.
        if !a.is_finite() {
            let p = Interval::point(a);
            assert!(!p.is_certified(), "{}", fuzz::replay());
            let q = Interval::point(if b.is_finite() { b } else { 1.0 });
            for r in [p + q, q + p, p - q, p * q, p / q, q / p, -p, p.sqr()] {
                assert!(
                    !r.is_certified(),
                    "the refusal must flow — {}",
                    fuzz::replay()
                );
            }
            n += 1;
        }
        // Inverted or NaN brackets are refused.
        if a.is_finite() && b.is_finite() && a > b {
            assert!(
                !Interval::from_bounds(a, b).is_certified(),
                "inverted bracket must refuse — {}",
                fuzz::replay()
            );
            n += 1;
        }
        assert!(
            !Interval::from_bounds(f64::NAN, b).is_certified(),
            "NaN bracket must refuse — {}",
            fuzz::replay()
        );
        // Any divisor whose bracket touches zero is refused.
        if a.is_finite() && b.is_finite() {
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            if lo <= 0.0 && hi >= 0.0 {
                let d = Interval::from_bounds(lo, hi);
                assert!(
                    !(Interval::point(1.0) / d).is_certified(),
                    "divisor [{lo:e}, {hi:e}] touches zero and must refuse — {}",
                    fuzz::replay()
                );
                n += 1;
            }
        }
    }
    println!("[refusal] {n} refusal cases, 0 leaks");
}

// ------------------------------------------------- the directed helpers

/// `geom_core::interval::norm_sup` is never below the exact
/// `sup ‖·‖` of its box, `√(Σ mag²)`, across the whole exponent range:
/// sides centred from the subnormals to past the squares' overflow
/// point, with signed zeros, zero-straddling sides and subnormal ends.
/// The underflow regime is the one a round-to-nearest fold of the same
/// endpoints fails in (a tiny square rounds to zero).
#[test]
fn norm_sup_is_sound_against_exact_arithmetic() {
    use geom_core::interval::norm_sup;
    let mut rng = fuzz::start("interval_exact_fuzz::norm_sup");
    let centers = [-1074i32, -600, -540, -511, -60, 0, 60, 511, 512, 600];
    // A short-mantissa point side: three of them square and sum
    // EXACTLY, so no interval pad stands between the sum and its root
    // and the root's own rounding is all that separates the reading
    // from the norm (a side of `1` thrice is `√3`, which rounds DOWN).
    let side = |rng: &mut fuzz::Rng| {
        if rng.next_u64().is_multiple_of(4) {
            let (v, ..) = short_dyadic(rng, 4);
            return (v, v);
        }
        let c = centers[(rng.next_u64() % centers.len() as u64) as usize];
        let mut a = f64_near_exp(rng, c);
        let mut b = match rng.next_u64() % 4 {
            0 => -a,
            1 => 0.0,
            2 => subnormal(rng),
            _ => f64_near_exp(rng, c),
        };
        if rng.next_u64().is_multiple_of(8) {
            a = -0.0;
        }
        if b < a {
            core::mem::swap(&mut a, &mut b);
        }
        (a, b)
    };
    let mut checked = 0u64;
    for _ in 0..fuzz::scaled(20_000) {
        let s = [side(&mut rng), side(&mut rng), side(&mut rng)];
        let r = norm_sup(&s.map(|(lo, hi)| Interval::from_bounds(lo, hi)));
        assert!(!r.is_nan(), "{s:?}: refused — {}", fuzz::replay());
        let mut m2 = Big::zero();
        for (lo, hi) in s {
            let m = lo.abs().max(hi.abs());
            m2 = m2.add(&big_prod(m, m));
        }
        if r.is_finite() {
            assert_ne!(
                big_prod(r, r).add(&m2.negated()).sign(),
                Ordering::Less,
                "{s:?}: norm_sup {r:e} below the exact norm — {}",
                fuzz::replay()
            );
            checked += 1;
        }
    }
    println!("[norm_sup] {checked} finite readings, 0 below the exact norm");
}

/// **A real cell where round-to-nearest was below the norm.** The
/// sides are bit for bit `NurbsBoxes::deriv_box` of `S_u` over the
/// whole domain of `m5_pr7_ssi.rs`'s `certifiable_wall`, the
/// chart-speed box `plane_nurbs_ssi`'s floors and limb 3's tube pad
/// divide by. The fold those sites used to compute, `√(Σ mag²)`
/// rounded to nearest at every step, reads `1.130884609498246` there,
/// which is BELOW the exact norm. The outward reading is not.
#[test]
fn norm_sup_is_above_the_exact_norm_on_a_cell_a_rounded_fold_is_below() {
    use geom_core::interval::norm_sup;
    let side = |lo: u64, hi: u64| Interval::from_bounds(f64::from_bits(lo), f64::from_bits(hi));
    let v = [
        side(4_607_407_598_781_385_931, 4_607_407_598_781_385_934),
        side(4_595_653_203_753_948_938, 4_601_237_667_291_888_354),
        Interval::from_bounds(0.0, 0.0),
    ];
    let m = v.map(|i| i.lo().abs().max(i.hi().abs()));
    let m2 = big_prod(m[0], m[0])
        .add(&big_prod(m[1], m[1]))
        .add(&big_prod(m[2], m[2]));
    let below = |r: f64| big_prod(r, r).add(&m2.negated()).sign() == Ordering::Less;
    let fold = (m[0] * m[0] + m[1] * m[1] + m[2] * m[2]).sqrt();
    assert_eq!(
        fold, 1.130_884_609_498_246,
        "the witness is the cell it names"
    );
    assert!(
        below(fold),
        "the rounded fold {fold:e} must sit below the exact norm"
    );
    let sup = norm_sup(&v);
    assert!(!below(sup), "norm_sup {sup:e} is below the exact norm");
}

/// `geom_core::interval::div_down` is never above the exact quotient,
/// from the subnormals to the quotients that overflow.
#[test]
fn div_down_is_sound_against_exact_arithmetic() {
    use geom_core::interval::div_down;
    let mut rng = fuzz::start("interval_exact_fuzz::div_down");
    let draw = |rng: &mut fuzz::Rng| match rng.next_u64() % 4 {
        0 => subnormal(rng),
        1 => f64_near_exp(rng, -900),
        2 => f64_near_exp(rng, 900),
        _ => f64_near_exp(rng, 0),
    };
    let mut checked = 0u64;
    for _ in 0..fuzz::scaled(25_000) {
        let (a, b) = (draw(&mut rng), draw(&mut rng));
        let q = div_down(a, b);
        assert!(
            !q.is_nan(),
            "div_down({a:e}, {b:e}) refused — {}",
            fuzz::replay()
        );
        assert_ne!(
            cmp_f64_vs_quot(q, a, b),
            Ordering::Greater,
            "div_down({a:e}, {b:e}) = {q:e} is above the exact quotient — {}",
            fuzz::replay()
        );
        checked += 1;
    }
    println!("[div_down] {checked} quotients, 0 above the exact one");
}

/// `x` against the exact `f / (a·b)` for positive `a`, `b` whose
/// product stays in the normal range: the product splits exactly as
/// `p + e` (`p` rounded, `e` its fused residual, checked exact here),
/// so `x·(a·b) − f = x·p + x·e − f` is three exact products and a sum
/// the [`Big`] register holds.
fn cmp_f64_vs_quot_of_product(x: f64, f: f64, a: f64, b: f64) -> Ordering {
    let p = a * b;
    let e = a.mul_add(b, -p);
    assert_eq!(
        big_of(p)
            .add(&big_of(e))
            .add(&big_prod(a, b).negated())
            .sign(),
        Ordering::Equal,
        "the split of {a:e}·{b:e} must be exact"
    );
    big_prod(x, p)
        .add(&big_prod(x, e))
        .add(&big_of(f).negated())
        .sign()
}

/// **`offset_meters::patch_regularity`'s sine floor**, `floor /
/// (sup‖S_u‖ · sup‖S_v‖)` from below. Its old spelling, the
/// nearest-rounded quotient by the nearest-rounded product stepped one
/// ulp down, lands ABOVE the exact quotient on this triple. Its
/// current spelling, two quotients each rounded down, does not.
#[test]
fn the_sine_floor_is_below_the_exact_quotient_where_one_step_down_was_not() {
    use geom_core::interval::div_down;
    let (floor, a, b): (f64, f64, f64) = (
        1.000_844_717_948_889_5,
        1.356_818_763_603_346,
        1.528_393_971_351_443_6,
    );
    let old = (floor / (a * b)).next_down();
    assert_eq!(
        cmp_f64_vs_quot_of_product(old, floor, a, b),
        Ordering::Greater,
        "the old spelling {old:e} must sit above the exact quotient, or this is no witness"
    );
    let new = div_down(div_down(floor, a), b);
    assert_ne!(
        cmp_f64_vs_quot_of_product(new, floor, a, b),
        Ordering::Greater,
        "the sine floor {new:e} is above the exact quotient"
    );
}
