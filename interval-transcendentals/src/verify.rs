//! Kani proof harnesses (`cfg(kani)` only — never compiled by cargo).
//!
//! What these are and are NOT: see `docs/verification.md`. In one line,
//! Kani is a bit-precise bounded model checker with NO model of the real
//! numbers, so it discharges the crate's *discrete* obligations — the
//! representation invariant, every case split, the neighbor-step
//! encoding lemma behind P3 — exhaustively over all 2^64 bit patterns
//! per `f64` input, and leaves the real-arithmetic lemmas (P1/P2/P3's
//! composition step, libm's accuracy, the π enclosures) to the paper
//! proofs in `docs/derivations.md`.
//!
//! Run: `cargo kani` (see docs/verification.md for the toolchain pin).

use crate::interval::{DInterval, Decoration};
use crate::round::{
    add_hi, add_lo, div_hi, div_lo, down1, mul_hi, mul_lo, step_down, step_up, sub_hi, sub_lo, up1,
};

// ---------------------------------------------------------------------
// Generators
// ---------------------------------------------------------------------

/// An arbitrary decoration.
fn any_dec() -> Decoration {
    match kani::any::<u8>() % 5 {
        0 => Decoration::Ill,
        1 => Decoration::Trv,
        2 => Decoration::Def,
        3 => Decoration::Dac,
        _ => Decoration::Com,
    }
}

/// An arbitrary *valid* `DInterval`: any of the three shapes (NaI, empty,
/// nonempty), with completely unconstrained `f64` endpoints in the
/// nonempty case — inf, subnormal and signed zero all included.
fn any_interval() -> DInterval {
    match kani::any::<u8>() % 4 {
        0 => DInterval::nai(),
        1 => DInterval::empty(),
        _ => {
            let lo: f64 = kani::any();
            let hi: f64 = kani::any();
            let x = DInterval::from_bounds(lo, hi);
            // `from_bounds` returns NaI for ill-formed pairs; keep those
            // out of this arm so the nonempty case is really exercised.
            kani::assume(!x.is_nai());
            let cap = any_dec();
            x.with_dec_capped(cap)
        }
    }
}

/// A finite `f64` in the interval — the symbolic "point of the enclosure"
/// used to state containment properties.
fn any_point_of(x: &DInterval) -> f64 {
    let p: f64 = kani::any();
    kani::assume(!p.is_nan() && x.lo <= p && p <= x.hi);
    p
}

/// The representation invariant of `DInterval`, as stated in its doc
/// comment. Everything the crate emits must satisfy it.
fn rep_invariant(x: &DInterval) -> bool {
    if x.dec == Decoration::Ill {
        return x.lo.is_nan() && x.hi.is_nan();
    }
    if x.lo.is_nan() || x.hi.is_nan() {
        // Empty: BOTH bounds NaN, decoration exactly Trv.
        return x.lo.is_nan() && x.hi.is_nan() && x.dec == Decoration::Trv;
    }
    if !(x.lo <= x.hi) {
        return false;
    }
    if x.lo == f64::INFINITY || x.hi == f64::NEG_INFINITY {
        return false;
    }
    if !(x.lo.is_finite() && x.hi.is_finite()) && x.dec > Decoration::Dac {
        return false; // Com requires boundedness
    }
    true
}

// ---------------------------------------------------------------------
// Tier A — discrete lemmas of the rounding layer (no reals involved)
// ---------------------------------------------------------------------

/// `step_down`/`step_up` really do move outward (or stay put at the
/// saturating infinities), for every non-NaN input and every k <= 4 —
/// the pad widths this crate actually uses.
///
/// NOTE the `unwind` attribute and the CONCRETE `k`: `step_down` is a
/// `for _ in 0..k` loop, and with a symbolic `k: u32` CBMC unwinds it
/// structurally without bound (it does not use the `assume(k <= 4)` to
/// cut the unwinding, only to kill paths). Bounded model checking needs
/// the trip count fixed at the harness. See docs/verification.md.
#[kani::proof]
#[kani::unwind(6)]
fn a1_steps_move_outward() {
    let x: f64 = kani::any();
    kani::assume(!x.is_nan());
    // The four pad widths this crate actually uses, plus the identity.
    for k in [0u32, 1, 2, 3, 4] {
        assert!(step_down(x, k) <= x);
        assert!(step_up(x, k) >= x);
        assert!(!step_down(x, k).is_nan());
        assert!(!step_up(x, k).is_nan());
    }
}

/// Lemma P3's load-bearing DISCRETE claim (docs/derivations.md §1): the
/// signed-magnitude bit encoding is a unit-step isometry, i.e. one
/// neighbor step changes libm-test's `bitdist` by exactly 1 — subnormals
/// and binade boundaries included. This is the half of P3 that does not
/// mention the real `t`, and it is the half that is easy to get wrong.
#[kani::proof]
fn a2_neighbor_step_is_one_bitdist() {
    let x: f64 = kani::any();
    kani::assume(x.is_finite());
    kani::assume(x != f64::MAX && x != f64::MIN);
    let up = up1(x);
    kani::assume(up.is_finite());
    let d = (up.to_bits() as i64).wrapping_sub(x.to_bits() as i64).abs();
    // Same sign class (the sign-crossing pair is exactly ±0.0, which the
    // proof of P3 handles separately: those differ by 2^63 in encoding).
    if x.is_sign_positive() == up.is_sign_positive() {
        assert!(d == 1, "one neighbor step must be one encoded step");
    } else {
        assert!(x == 0.0 && up != 0.0, "only the zero pair may cross sign");
    }
}

/// `step_down(x, k)` is exactly `k` neighbor steps (ties the code to the
/// `k` that Lemma P3 reasons about), for k <= 4 within a sign class.
#[kani::proof]
#[kani::unwind(6)]
fn a3_step_k_is_k_neighbor_steps() {
    let x: f64 = kani::any();
    kani::assume(x.is_finite() && x > 0.0);
    kani::assume(step_up(x, 4).is_finite());
    let bits = x.to_bits();
    assert!(step_up(x, 0).to_bits() == bits);
    assert!(step_up(x, 1).to_bits() == bits + 1);
    assert!(step_up(x, 2).to_bits() == bits + 2);
    assert!(step_up(x, 3).to_bits() == bits + 3);
    assert!(step_up(x, 4).to_bits() == bits + 4);
}

// ---------------------------------------------------------------------
// Tier B — representation invariant and case analysis (exhaustive over
// all f64 bit patterns; specs are RELATIVE to the round.rs primitives)
// ---------------------------------------------------------------------

/// No arithmetic operation can construct an invalid interval. This is
/// `make()`'s `debug_assert!` turned into a theorem over every pair of
/// intervals in the type, including every poison shape.
#[kani::proof]
#[kani::unwind(6)]
fn b1_arithmetic_preserves_the_representation_invariant() {
    let x = any_interval();
    let y = any_interval();
    kani::assume(rep_invariant(&x) && rep_invariant(&y));
    assert!(rep_invariant(&(x + y)), "add");
    assert!(rep_invariant(&(x - y)), "sub");
    assert!(rep_invariant(&(-x)), "neg");
    assert!(rep_invariant(&(x * y)), "mul");
    assert!(rep_invariant(&(x / y)), "div");
}

/// Same, for the endpoint-exact ops and the set ops.
#[kani::proof]
#[kani::unwind(6)]
fn b2_exact_ops_preserve_the_representation_invariant() {
    let x = any_interval();
    let y = any_interval();
    kani::assume(rep_invariant(&x) && rep_invariant(&y));
    assert!(rep_invariant(&x.abs()), "abs");
    assert!(rep_invariant(&x.min_i(y)), "min");
    assert!(rep_invariant(&x.max_i(y)), "max");
    assert!(rep_invariant(&x.floor()), "floor");
    assert!(rep_invariant(&x.hull(y)), "hull");
    assert!(rep_invariant(&x.intersection(y)), "intersection");
    assert!(rep_invariant(&x.sqrt()), "sqrt");
}

/// Poison never launders: a NaI operand forces NaI out, and an empty
/// operand forces empty (or NaI) out — across the whole arithmetic
/// surface, for arbitrary partners.
#[kani::proof]
#[kani::unwind(6)]
fn b3_poison_propagates() {
    let y = any_interval();
    kani::assume(rep_invariant(&y));
    let n = DInterval::nai();
    assert!((n + y).is_nai() && (y + n).is_nai());
    assert!((n * y).is_nai() && (y * n).is_nai());
    assert!((n / y).is_nai() && (y / n).is_nai());
    assert!(n.hull(y).is_nai() && y.hull(n).is_nai());
    assert!(n.intersection(y).is_nai());
    let e = DInterval::empty();
    assert!((e + y).is_empty() || (e + y).is_nai());
    assert!((e * y).is_empty() || (e * y).is_nai());
}

/// Interval multiplication encloses the padded product of EVERY pair of
/// points of the operands — not just the four corners the implementation
/// happens to test. `p`, `q` are symbolic points, so this is the real
/// containment statement, relative to `mul_lo`/`mul_hi` (whose own
/// soundness is Tier C / the paper proofs).
#[kani::proof]
#[kani::unwind(6)]
fn b4_mul_encloses_every_point_product() {
    let x = any_interval();
    let y = any_interval();
    kani::assume(rep_invariant(&x) && rep_invariant(&y));
    kani::assume(!x.is_nai() && !y.is_nai() && !x.is_empty() && !y.is_empty());
    let p = any_point_of(&x);
    let q = any_point_of(&y);
    let r = x * y;
    kani::assume(!r.is_nai());
    assert!(r.lo <= mul_lo(p, q), "lower bound misses an interior product");
    assert!(r.hi >= mul_hi(p, q), "upper bound misses an interior product");
}

/// The flagship: interval DIVISION encloses the padded quotient of every
/// point pair with a nonzero divisor — including every zero-touching and
/// zero-straddling shape, which is where the case analysis lives
/// (`div_touching_zero`'s four-way sign match).
#[kani::proof]
#[kani::unwind(6)]
fn b5_div_encloses_every_point_quotient() {
    let x = any_interval();
    let y = any_interval();
    kani::assume(rep_invariant(&x) && rep_invariant(&y));
    kani::assume(!x.is_nai() && !y.is_nai() && !x.is_empty() && !y.is_empty());
    let p = any_point_of(&x);
    let q = any_point_of(&y);
    kani::assume(q != 0.0 && q.is_finite());
    let r = x / y;
    kani::assume(!r.is_empty() && !r.is_nai());
    assert!(r.lo <= div_lo(p, q), "lower bound misses an interior quotient");
    assert!(r.hi >= div_hi(p, q), "upper bound misses an interior quotient");
}

/// Addition and subtraction: same containment shape.
#[kani::proof]
#[kani::unwind(6)]
fn b6_add_sub_enclose_every_point() {
    let x = any_interval();
    let y = any_interval();
    kani::assume(rep_invariant(&x) && rep_invariant(&y));
    kani::assume(!x.is_nai() && !y.is_nai() && !x.is_empty() && !y.is_empty());
    let p = any_point_of(&x);
    let q = any_point_of(&y);
    let s = x + y;
    kani::assume(!s.is_nai());
    assert!(s.lo <= add_lo(p, q) && s.hi >= add_hi(p, q));
    let d = x - y;
    kani::assume(!d.is_nai());
    assert!(d.lo <= sub_lo(p, q) && d.hi >= sub_hi(p, q));
}

/// The endpoint-exact ops are exactly correct (no relativization needed:
/// abs/min/max/hull/intersection involve no rounding at all).
#[kani::proof]
#[kani::unwind(6)]
fn b7_exact_ops_are_exactly_correct() {
    let x = any_interval();
    let y = any_interval();
    kani::assume(rep_invariant(&x) && rep_invariant(&y));
    kani::assume(!x.is_nai() && !y.is_nai() && !x.is_empty() && !y.is_empty());
    let p = any_point_of(&x);
    let q = any_point_of(&y);
    assert!(x.abs().contains(p.abs()), "abs");
    assert!(x.min_i(y).contains(p.min(q)), "min");
    assert!(x.max_i(y).contains(p.max(q)), "max");
    assert!(x.floor().contains(p.floor()), "floor");
    assert!(x.hull(y).contains(p) && x.hull(y).contains(q), "hull");
}

/// The interval-square poison contract (memories/interval-square-poison.md):
/// an EVEN power of a zero-straddling interval must have lower bound
/// exactly `+0.0`, so a downstream `sqrt` can never see a spurious
/// negative. Proven for every straddling interval and n in {2, 4, 6}.
#[kani::proof]
#[kani::unwind(8)]
fn b8_even_powi_of_a_straddling_interval_has_lo_exactly_zero() {
    let x = any_interval();
    kani::assume(rep_invariant(&x) && !x.is_nai() && !x.is_empty());
    kani::assume(x.lo <= 0.0 && x.hi >= 0.0);
    let n: i32 = kani::any();
    kani::assume(n == 2 || n == 4 || n == 6);
    let r = x.powi(n);
    kani::assume(!r.is_nai());
    assert!(r.lo == 0.0, "even power of a straddling interval must floor at 0");
    assert!(r.lo.is_sign_positive(), "and at +0.0, not -0.0");
    // The sqrt of that can therefore never be poisoned.
    assert!(!r.sqrt().is_nai());
}

/// `sqrt` never emits a negative lower bound, for any input interval —
/// the other half of the poison contract, and the one the pad could
/// break (a downward pad of a tiny positive root must clamp at 0).
#[kani::proof]
fn b9_sqrt_lower_bound_is_never_negative() {
    let x = any_interval();
    kani::assume(rep_invariant(&x) && !x.is_nai() && !x.is_empty());
    let r = x.sqrt();
    kani::assume(!r.is_empty() && !r.is_nai());
    assert!(r.lo >= 0.0, "sqrt lower bound went negative");
}

/// `with_dec_capped` only ever LOWERS the decoration and never touches
/// the bounds — the "poison is never laundered" primitive, over every
/// interval shape and every cap.
#[kani::proof]
fn b10_dec_cap_never_launders() {
    let x = any_interval();
    kani::assume(rep_invariant(&x));
    let cap = any_dec();
    let r = x.with_dec_capped(cap);
    assert!(rep_invariant(&r));
    assert!(r.dec <= x.dec, "capping must never raise the decoration");
    if !x.is_nai() && !x.is_empty() && !r.is_nai() {
        assert!(r.lo.to_bits() == x.lo.to_bits() && r.hi.to_bits() == x.hi.to_bits());
    }
}

// ---------------------------------------------------------------------
// Tier C — soundness of the rounding layer against EXACT arithmetic,
// in the f32 scaled model.
//
// Kani has no reals, so `mul_lo(a, b) <= a*b` (exact) is not directly
// statable over f64. It IS statable one precision down: the exact
// product of two f32 values is always representable in f64 (24+24 = 48
// mantissa bits, and the exponent range fits with room to spare, even
// for subnormal factors), so `f64` serves as an EXACT oracle for `f32`
// arithmetic. These harnesses verify the *same algorithm* at f32 and
// prove the enclosure property against the true real product/quotient.
//
// What transfers: the algorithm's shape (the validity-floor gate, the
// zero conventions, which side pads). What does NOT transfer: the f64
// instance itself, and the specific constant `2^-960`. See
// docs/verification.md.
// ---------------------------------------------------------------------

/// f32 mirror of `round.rs`'s 2Prod validity floor. The f64 code gates at
/// `2^-960` against a literature floor of `2^-969` (= 2*E_min + p, i.e.
/// 2*(-1022) + 53 + ...); the f32 analogue of the literature floor is
/// `2^-105` (2*(-126) + 24 + ...), and we keep the same 9-binade margin.
const F32_TWO_PROD_VALID_MIN: f32 = f32::from_bits(0x0C80_0000); // 2^-102

fn mul_exact32(a: f32, b: f32, r: f32) -> bool {
    r.abs() >= F32_TWO_PROD_VALID_MIN && f32::mul_add(a, b, -r) == 0.0
}

fn mul_lo32(a: f32, b: f32) -> f32 {
    if a == 0.0 || b == 0.0 {
        return 0.0;
    }
    let r = a * b;
    if mul_exact32(a, b, r) { r } else { r.next_down() }
}

fn mul_hi32(a: f32, b: f32) -> f32 {
    if a == 0.0 || b == 0.0 {
        return 0.0;
    }
    let r = a * b;
    if mul_exact32(a, b, r) { r } else { r.next_up() }
}

/// FULL soundness of the padded product, at f32, against the exact real
/// product (computed in f64, where it is exact). No assumption, no
/// oracle, no sampling: all 2^64 input pairs.
#[kani::proof]
fn c1_mul_pads_enclose_the_exact_product() {
    let a: f32 = kani::any();
    let b: f32 = kani::any();
    kani::assume(a.is_finite() && b.is_finite());
    // EXACT for f32 factors, and always finite (|a*b| <= ~1.2e77).
    let exact = f64::from(a) * f64::from(b);
    let lo = mul_lo32(a, b);
    let hi = mul_hi32(a, b);
    // No finiteness assumption: a saturating bound is still an enclosure
    // (-inf <= exact <= +inf), and a bound saturating the WRONG way would
    // fail these comparisons, which is exactly what we want it to do.
    assert!(f64::from(lo) <= exact, "lower pad crossed the true product");
    assert!(f64::from(hi) >= exact, "upper pad crossed the true product");
}

/// The historical bug, reproduced as a proof obligation: the first
/// version of `mul_exact` gated on `is_normal()` instead of the 2Prod
/// validity floor, and the differential harness caught a barely-normal
/// product of a subnormal factor whose FMA residual underflowed — the
/// witness LIED and the pad was skipped (`round.rs`'s
/// `TWO_PROD_VALID_MIN` comment). This harness is expected to FAIL, and
/// the counterexample it prints is that bug.
#[cfg(kani_mutation_is_normal)]
#[kani::proof]
fn c1_mut_is_normal_gate_is_unsound() {
    fn mul_exact_buggy(a: f32, b: f32, r: f32) -> bool {
        r.is_normal() && f32::mul_add(a, b, -r) == 0.0
    }
    let a: f32 = kani::any();
    let b: f32 = kani::any();
    kani::assume(a.is_finite() && b.is_finite());
    let r = a * b;
    // Search NARROWED to the region the historical bug lived in: a
    // product that is normal (so an `is_normal()` gate waves it through)
    // but below the 2Prod validity floor (so its FMA residual can itself
    // underflow and the witness can lie). The narrowing is what makes
    // this terminate in seconds rather than hours; it exhibits the bug
    // class, it does not search blind.
    kani::assume(r.is_normal() && r.abs() < F32_TWO_PROD_VALID_MIN);
    let exact = f64::from(a) * f64::from(b);
    let lo = if a == 0.0 || b == 0.0 {
        0.0
    } else if mul_exact_buggy(a, b, r) {
        r
    } else {
        r.next_down()
    };
    assert!(f64::from(lo) <= exact, "lower pad crossed the true product");
}

fn div_exact32(a: f32, b: f32, q: f32) -> bool {
    a.abs() >= F32_TWO_PROD_VALID_MIN && q.is_finite() && f32::mul_add(q, b, -a) == 0.0
}

fn div_lo32(a: f32, b: f32) -> f32 {
    if a == 0.0 {
        return 0.0;
    }
    let q = a / b;
    if div_exact32(a, b, q) { q } else { q.next_down() }
}

fn div_hi32(a: f32, b: f32) -> f32 {
    if a == 0.0 {
        return 0.0;
    }
    let q = a / b;
    if div_exact32(a, b, q) { q } else { q.next_up() }
}

/// FULL soundness of the padded quotient at f32. The exact quotient is
/// not representable, so the enclosure `lo <= a/b` is stated by
/// cross-multiplication — `lo * b` IS exact in f64 (f32 factors) — with
/// the divisor's sign flipping the inequality.
#[kani::proof]
fn c2_div_pads_enclose_the_exact_quotient() {
    let a: f32 = kani::any();
    let b: f32 = kani::any();
    kani::assume(a.is_finite() && b.is_finite() && b != 0.0);
    let lo = div_lo32(a, b);
    let hi = div_hi32(a, b);
    let (ad, bd) = (f64::from(a), f64::from(b));
    // lo <= a/b  and  hi >= a/b, cross-multiplied by b (exact products).
    if b > 0.0 {
        assert!(f64::from(lo) * bd <= ad, "lower pad crossed the true quotient");
        assert!(f64::from(hi) * bd >= ad, "upper pad crossed the true quotient");
    } else {
        assert!(f64::from(lo) * bd >= ad, "lower pad crossed the true quotient");
        assert!(f64::from(hi) * bd <= ad, "upper pad crossed the true quotient");
    }
}

fn two_sum_err32(a: f32, b: f32, s: f32) -> f32 {
    let bp = s - a;
    let ap = s - bp;
    (a - ap) + (b - bp)
}

fn add_lo32(a: f32, b: f32) -> f32 {
    let s = a + b;
    if two_sum_err32(a, b, s) == 0.0 { s } else { s.next_down() }
}

fn add_hi32(a: f32, b: f32) -> f32 {
    let s = a + b;
    if two_sum_err32(a, b, s) == 0.0 { s } else { s.next_up() }
}

/// Soundness of the padded sum at f32, against the exact real sum. The
/// exact sum of two f32s needs up to 277 bits, so unlike the product it
/// is NOT always f64-representable; the harness assumes an exponent
/// spread under 28, which is exactly the regime where the sum is exact
/// in f64 — and is also the regime containing every cancellation case,
/// where the TwoSum exactness test is load-bearing. Outside it the
/// larger operand dominates and the pad is unconditionally safe.
#[kani::proof]
fn c3_add_pads_enclose_the_exact_sum_in_the_representable_regime() {
    let a: f32 = kani::any();
    let b: f32 = kani::any();
    kani::assume(a.is_normal() && b.is_normal());
    let ea = ((a.to_bits() >> 23) & 0xff) as i32;
    let eb = ((b.to_bits() >> 23) & 0xff) as i32;
    kani::assume((ea - eb).abs() <= 28);
    let exact = f64::from(a) + f64::from(b); // exact in this regime
    let lo = add_lo32(a, b);
    let hi = add_hi32(a, b);
    assert!(f64::from(lo) <= exact, "lower pad crossed the true sum");
    assert!(f64::from(hi) >= exact, "upper pad crossed the true sum");
}

/// Negative control for Tier C: with the pad REMOVED, the same property
/// must fail. If this harness ever passes, the Tier C harnesses are
/// vacuous and prove nothing.
#[cfg(kani_mutation_no_pad)]
#[kani::proof]
fn c4_mut_unpadded_product_is_unsound() {
    let a: f32 = kani::any();
    let b: f32 = kani::any();
    kani::assume(a.is_finite() && b.is_finite());
    let exact = f64::from(a) * f64::from(b);
    let r = a * b;
    assert!(f64::from(r) <= exact, "unpadded product is a lower bound");
}

// ---------------------------------------------------------------------
// Tier A, continued — the directed primitives are ordered and never turn
// non-NaN inputs into NaN (a NaN endpoint would poison `make` into NaI,
// which is a loud failure the kernel must never see from ordinary
// arithmetic).
//
// These are split additive / multiplicative / zero-convention rather
// than bundled into one harness ON PURPOSE: the multiplicative path goes
// through `mul_exact`'s `f64::mul_add`, and bit-blasting a
// double-precision FMA dominates everything else by orders of magnitude.
// Bundled, the cheap obligations never land. See docs/VERIFICATION-TOOLS.md.
// ---------------------------------------------------------------------

/// Additive primitives only: ordered, NaN-free. No FMA anywhere on this
/// path (`two_sum_err` is three adds and two subs).
#[kani::proof]
fn a5_additive_primitives_are_ordered() {
    let a: f64 = kani::any();
    let b: f64 = kani::any();
    kani::assume(!a.is_nan() && !b.is_nan());
    kani::assume(!(a.is_infinite() && b.is_infinite() && a != b));
    assert!(add_lo(a, b) <= add_hi(a, b));
    assert!(!add_lo(a, b).is_nan() && !add_hi(a, b).is_nan());
    assert!(sub_lo(a, -b) <= sub_hi(a, -b));
    assert!(!sub_lo(a, -b).is_nan() && !sub_hi(a, -b).is_nan());
}

/// Multiplicative primitives only — this is the FMA-bound one.
#[kani::proof]
fn a6_multiplicative_primitives_are_ordered() {
    let a: f64 = kani::any();
    let b: f64 = kani::any();
    kani::assume(!a.is_nan() && !b.is_nan());
    assert!(mul_lo(a, b) <= mul_hi(a, b));
    assert!(!mul_lo(a, b).is_nan() && !mul_hi(a, b).is_nan());
}

/// The Kahan corner convention `0 · anything := 0`, including against an
/// infinite bound — the case that sidesteps the `0 × inf = NaN` trap.
#[kani::proof]
fn a7_zero_times_anything_is_exactly_zero() {
    let b: f64 = kani::any();
    kani::assume(!b.is_nan());
    assert!(mul_lo(0.0, b) == 0.0 && mul_hi(0.0, b) == 0.0);
    assert!(mul_lo(-0.0, b) == 0.0 && mul_hi(-0.0, b) == 0.0);
    assert!(mul_lo(b, 0.0) == 0.0 && mul_hi(b, 0.0) == 0.0);
}

/// TOOL CONTROL (expected to FAIL, behind `--cfg kani_tool_control`).
///
/// Tiers A and C rest on CBMC modelling `mul_add` as a genuinely FUSED
/// multiply-add — one rounding, not two. A naive `a*b + c` model would
/// make `mul_add(a, b, -(a*b))` identically zero, and both `mul_exact`
/// and `div_exact` would then be verified against a fiction. Asserting
/// that identity must therefore produce a COUNTEREXAMPLE: a pair whose
/// product is inexact, so the fused residual is nonzero. If this
/// harness ever passes, every FMA-dependent result above is void.
#[cfg(kani_tool_control)]
#[kani::proof]
fn c5_ctl_fma_is_really_fused() {
    let a: f32 = kani::any();
    let b: f32 = kani::any();
    kani::assume(a.is_normal() && b.is_normal());
    let r = a * b;
    kani::assume(r.is_normal());
    assert!(
        f32::mul_add(a, b, -r) == 0.0,
        "if this assertion holds for ALL inputs, mul_add is not fused"
    );
}
