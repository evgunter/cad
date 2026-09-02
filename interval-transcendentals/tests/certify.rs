//! Differential certification against inari(+gmp/MPFR) as oracle.
//!
//! Soundness direction: truth ⊆ oracle ⊆ mine (oracle enclosures are
//! correctly rounded, hence contain truth; ours must contain the
//! oracle's). Millions of property cases per run; tightness ratios are
//! reported per function (visible with `--nocapture`).
//!
//! **The seed VARIES per run** (`test_utils::fuzz`, the tree's one
//! harness). Every test here is a counterexample search — `∀ sampled x.
//! oracle ⊆ ours` — which is the harness's shape 1, so a fixed seed
//! would mean certifying the same few million points forever no matter
//! how often the lane ran. It ran rarely enough that this mattered:
//! pinned, a decade of firings re-checks one sample; varying, each
//! firing is a fresh several million. The seed is logged unconditionally
//! and `CAD_FUZZ_SEED=0x…` replays any run exactly.
//!
//! Gated on `oracle-inari` (M5 PR 1 fix pass): the oracle is inari with
//! its bundled GMP/MPFR C build, and making it optional is what lets the
//! kernel's CI run this crate's oracle-free tier without a C toolchain.
//!
//! **CI runs this lane.** `ci.yml`'s `oracle-certify` job fires it
//! whenever anything under `interval-transcendentals/src/`, `tests/`,
//! `Cargo.toml` or `Cargo.lock` changes (`scripts/ci-filter.py`'s
//! `ORACLE_PATHS`), at `CAD_FUZZ_EFFORT=8`. To run it by hand as well:
//! `RUSTFLAGS="-C target-cpu=x86-64-v3" cargo test --release --features
//! oracle-inari` (the flag is inari's AVX+FMA floor, not ours).
//!
//! This lane owns the SOUNDNESS direction for the transcendentals: their
//! truth is not computable without a multi-precision reference, so a pad
//! that is too small is invisible to every cheap-tier row. It also owns
//! the upper bound on the whole enclosure, via `Tightness`' ceiling
//! below — the cheap tier's `pad_contract.rs` bounds each pad, this
//! bounds the result — and it bounds it in **two** directions, because
//! one of them does not imply the other: a scale-free width RATIO, and
//! an absolute PER-ENDPOINT step distance that a ratio cannot see
//! wherever the oracle's own width is already large.
#![cfg(feature = "oracle-inari")]

mod common;

use common::Bound::{Divergent, Scored};
use common::{Ceiling, Tightness, assert_contains, gen_interval, steps, to_inari};
use interval_transcendentals::DInterval;
use test_utils::fuzz;

// Shipped case counts BEFORE `fuzz::scaled` multiplies them by
// CAD_FUZZ_EFFORT. At effort 1 the whole file is ~4.0M cases in ~7s
// (release) — and the lane it runs on spends 234s building GMP and MPFR
// from C source before it gets here (#480). The cases are 3% of the job,
// so depth here is very nearly free: the CI lane sets CAD_FUZZ_EFFORT=8
// and pays ~56s for 32M cases against that same fixed build cost.
const CASES_UNARY: usize = 400_000;
const CASES_BINARY: usize = 300_000;

/// Magnitude windows (binade exponents) swept per case batch: everyday
/// values, subnormal/tiny, huge (argument-reduction stress).
///
/// **The RATIO CEILING depends on window 3's `emin` and on the other
/// three windows' `emax`, and this is the only place that says so.**
/// (Only the ratio ceiling. `Tightness`' zero-tolerance assert on
/// `mine_unbounded_oracle_bounded` is constrained by something else
/// entirely — a few-ulp band around each `tan` pole, deep inside these
/// windows — and that constraint is recorded on the field itself.) Ratios can exceed any ceiling only through a FALSE extremum
/// capture, which pins a bound at ±1 where the true image is a sliver.
/// Its onset is `|x| ≈ 2^32` (`consts::grid_possibly_hits`), and window
/// 3 — the one whose accumulator carries NO ceiling — begins at `2^30`.
/// The ceiling-carrying windows top out at `2^8`, twenty-four binades
/// below the onset.
///
/// So: **raising a ceiling-carrying window's `emax` past ~30, or
/// raising window 3's `emin`, produces a red on a sound enclosure.**
/// That is not a reason never to do it; it is a reason to move the
/// ceiling in the same commit, with `Tightness::report`'s derivation
/// re-read. The `4·10^15` figure the crate's prose quotes is the wrong
/// number for this purpose — it is total degradation, seven binades
/// later.
// Upper cap 1022: log_mag's mantissa factor reaches 2, and 2·2^1022 is
// still finite; endpoints may still overflow to +inf via width addition,
// which is a deliberate unbounded-interval test case.
const WINDOWS: [(i32, i32); 4] = [(-8, 8), (-1074, -960), (-60, 4), (30, 1022)];

/// What a run of one of the 4-step ops (`PAD_ULPS = 4`: the seven
/// transcendentals) may look like.
///
/// **`max_ratio`.** The structural worst is `≈ 4·pad + 1 = 17`, by the
/// steps-to-ratio conversion whose ONE home is [`Tightness::report`] —
/// not restated here, nor in `powi_ceiling`, which reads the same rule
/// off it. Measured maxima across seeds and efforts run 8–12
/// (`atan` at 12 under an adversarial hunt over binade edges). 64 sits
/// ~3.8x over the derivation and ~5x over the measurement — loose enough
/// that no legitimate draw reaches it, tight enough that a gross
/// widening cannot hide under it. It is deliberately NOT attributed per
/// function: which function tops out at which value is seed-dependent,
/// and a per-function number reads as more precise than it is.
///
/// **`min_ratio_fraction`.** Measured comparable-ratio yields on the
/// ceiling-carrying accumulators, at effort 2: `sin`/`cos`/`atan`/
/// `atan2` 100%, `tan` 81%, `asin`/`acos` 80%, and on the arithmetic
/// side `add`/`sub` 99.8%, `mul` 88%, `sqrt` 57%, `div` 52% — the floor
/// is set against `div`, the lowest, with better than 2x margin. Its job
/// is to catch a collapse, not to certify a yield.
///
/// **`max_steps_when_oracle_exact`.** An entitlement, not a
/// measurement: this class is EMPTY for all seven transcendentals
/// (measured 0 of 600 000 per function — inari's transcendental
/// enclosures are essentially never degenerate, the values being
/// irrational), so the number is what a point input whose value happened
/// to be exactly representable would be entitled to — `step_down(v, 4)`
/// to `step_up(v, 4)`, 8 steps — doubled for a clip or zero-rung shift.
const TRANSCENDENTAL: fn() -> Ceiling = || Ceiling {
    max_ratio: Scored(64.0),
    min_ratio_fraction: Scored(0.25),
    max_steps_when_oracle_exact: Scored(16),
    max_endpoint_steps: Scored(8),
    unbounded_when_oracle_bounded: Scored(0),
};

/// **The huge window, for the six unary functions whose endpoints there
/// are still the correctly-rounded ones.** The RATIO is unscorable by
/// `semantics-diffs` D3 — this is where extremum/pole localization is
/// documented to degrade to the trivial enclosure — but that degradation
/// is a claim about WIDTH, and it does not follow that an endpoint may
/// wander: `asin`/`acos`/`atan`/`tan` measure 4 steps there and `sqrt` 1,
/// the same numbers as their everyday windows. Scoring what is scorable
/// is the point of this row: before it, `wi == 3` carried no ceiling of
/// any kind and the endpoint half of the bound was excluded by an
/// argument nobody had made.
const TRANSCENDENTAL_HUGE: fn() -> Ceiling = || Ceiling {
    max_ratio: Divergent("semantics-diffs D3: localization degrades to the trivial enclosure"),
    min_ratio_fraction: Divergent("D3 again: how many draws stay comparable is the degradation"),
    max_steps_when_oracle_exact: Scored(16),
    max_endpoint_steps: Scored(8),
    unbounded_when_oracle_bounded: Divergent(
        "tan's honest pole refusal returns [-inf, inf] within a few ulps of every pole",
    ),
};

/// **The huge window for `sin`/`cos`, where the endpoints diverge too.**
/// False extremum capture pins an endpoint at ±1 where the true image is
/// a sliver — sound, and as far from the correctly-rounded endpoint as
/// the codomain allows (measured 9.2e18 steps). Its onset is `|x| ≈ 2^32`
/// and this window starts at `2^30`, which is exactly why the other four
/// windows can score the endpoint metric.
const TRIG_HUGE: fn() -> Ceiling = || Ceiling {
    max_ratio: Divergent("semantics-diffs D3: localization degrades to the trivial enclosure"),
    min_ratio_fraction: Divergent("D3 again: how many draws stay comparable is the degradation"),
    max_steps_when_oracle_exact: Scored(16),
    max_endpoint_steps: Divergent("false extremum capture above |x| ~ 2^32 pins an endpoint at ±1"),
    unbounded_when_oracle_bounded: Scored(0),
};

/// The same for the 1-step ops (`+ − × ÷ sqrt`): structural worst
/// `4·1 + 1 = 5`, measured 3, ceiling 8. Here the oracle-exact class is
/// NOT empty — 112 859 draws for `sqrt`, 1 149 for `add` — and our
/// widest enclosure on it measured **2 steps** against an entitlement of
/// 2 (`down1`..`up1`); the allowance of 8 is 4x that.
const ARITHMETIC: fn() -> Ceiling = || Ceiling {
    max_ratio: Scored(8.0),
    min_ratio_fraction: Scored(0.25),
    max_steps_when_oracle_exact: Scored(8),
    max_endpoint_steps: Scored(4),
    unbounded_when_oracle_bounded: Scored(0),
};

/// The exponents `certify_powi` draws.
const POWI_EXPS: [i32; 12] = [0, 1, 2, 3, 4, 5, 7, 12, -1, -2, -3, 31];

/// What `x^n` is entitled to, **as a function of the exponent** — and
/// the domain that entitlement is derived on.
///
/// **Why no constant is the right answer here.** `powi` is not a padded
/// primitive: it is directed binary exponentiation over `mul_lo`/`mul_hi`
/// (plus one interval division when `n < 0`), so its looseness is a
/// COMPOSITION of `mul` pads and grows with the exponent. A constant
/// fitted to the widest draw of a mixed-exponent sweep is ~360x too
/// loose for the `n = 1` draws sharing that sweep, and every component
/// step is already bounded by `certify_arith` and `pad_contract.rs`
/// — what was missing was the composed bound, and it is derivable.
///
/// **The domain, first, because it is what the derivation is about.**
/// `u` below is a RELATIVE bound on one padded multiplication, and a
/// relative bound on a one-step pad only exists where a step is a fixed
/// fraction of the value — i.e. in the NORMAL range, where
/// `ulp(v)/|v| ∈ [2^-53, 2^-52]`. Below `2^-1022` the spacing is
/// absolute (`2^-1074`) and one step is `2^-1074/|v|` relative: at
/// `2^-1024` that is `2^-50`, **four normal ulps**, and at the bottom of
/// the subnormal range it is 1. So every number below holds on the
/// class where the ladder runs in the normal range, and
/// [`powi_normal_domain`] is the per-draw predicate for it — a
/// mechanism, not a window index.
///
/// **The derivation, by induction over the ladder.** Write `u` for the
/// relative deviation one padded multiplication may introduce: `mul_lo`
/// / `mul_hi` take the round-to-nearest product and step it one
/// representable place outward when it is inexact, so on the domain
/// above they land within **1.5 ulp** of the exact product,
/// `u <= 1.5·2^-52 = 3·2^-53`.
///
/// The ladder holds a squared base and an accumulator. Let `e(v)` bound
/// the relative deviation of a computed value `v` from its true one.
///
/// - The base starts EXACT (`e = 0`: it is the operand endpoint) and is
///   squared `t` times; squaring doubles its operand's deviation and
///   adds one pad, so `e(x^{2^k}) <= 2·e(x^{2^{k-1}}) + u = (2^k − 1)·u`.
/// - The accumulator starts at `1.0` and takes one multiply per SET BIT
///   of `m`: `acc ← acc · x^{2^k}`, so `e(acc) <= e(acc) + (2^k − 1)·u + u`.
///
/// Summing the set bits `k` of `m` — `Σ 2^k = m`, one `u` per set bit,
/// and one `u` more for the first `acc ← 1.0 · base`, whose product is
/// exact but whose `mul_exact` 2Prod witness is gated on the product
/// MAGNITUDE (`2^-960`) and declines below it — gives
///
/// ```text
/// e(m) <= m·u
/// ```
///
/// for every `m >= 1`, whatever the bit pattern of `m`: `m = 2` is
/// `(2−1)u` for the squaring plus `1u` for the accumulator multiply,
/// `= 2u`; `m = 3` is `1u + (2−1)u + 1u = 3u`; `m = 5` is
/// `1u + (4−1)u + 1u = 5u`; `m = 7` is `1u + 1u + (2−1)u + 1u + (4−1)u
/// = 7u`. **The recurrence a reader checks is the one above**, per
/// value, not a single `e(a+b) <= e(a) + e(b) + u` over exponents —
/// that form is also true but does not give these numbers, because it
/// forgets that the squared base is REUSED and its deviation counted
/// once, not once per use.
///
/// In representable STEPS at the result — one step is at least `2^-53`
/// relative — that is `3m` steps per endpoint, and a negative exponent's
/// reciprocal preserves relative width and pads 1.5 ulp on each side,
/// for `3` more.
///
/// **From steps to a width ratio**: the conversion has one home,
/// [`Tightness::report`]. Applied to `pad = 3m` it reads `12·|n| + 1`,
/// and `12·|n| + 13` with the reciprocal. The oracle-exact class is scored
/// absolutely instead, at the same `2·pad` steps of width.
///
/// **Measured against it at effort 8, on the normal domain** (worst
/// ratio / entitlement, worst endpoint steps / entitlement):
///
/// ```text
/// n:        1      2      3      4      5      7     12      31     -1     -2     -3
/// ratio   3/13   2/25   6/37   8/49  13/61  20/85  38/145  124/373  6/25   6/37  12/49
/// steps    1/8   1/14   3/20   5/26   8/32  13/44   25/74   74/188   3/14   4/20   7/26
/// ```
///
/// **Every margin is stated here, in the same shape as the other
/// ceilings in this file**: the tightest is `n = 31` at 3.0x on the
/// ratio and 2.5x on the endpoint count; every other exponent sits at
/// 4x or more, and the negative exponents — which before the domain
/// split climbed with volume (14 at effort 2, 17 at effort 8, 18 at
/// effort 64) because window 1's subnormal draws were inside this class
/// — now sit at 4.2x and do not move with the seed.
///
/// **`n = 0` carries no ratio and is not thereby unguarded.** `powi`
/// returns `[1, 1]` without touching the ladder, so every draw lands in
/// the oracle-exact bucket and a width ratio does not exist for it. The
/// class is pinned exactly at the draw site instead, which is stronger
/// than any ratio: the answer must be the point `[1, 1]` itself.
fn powi_ceiling(n: i32) -> Ceiling {
    let pad = 3.0 * f64::from(n.abs()) + if n < 0 { 3.0 } else { 0.0 };
    Ceiling {
        max_ratio: if n == 0 {
            Divergent("powi(x, 0) is the point one: no width to compare, pinned as a value")
        } else {
            Scored(4.0 * pad + 1.0)
        },
        // Measured comparable-ratio yields per exponent on the normal
        // domain, at effort 8: 86% at `n = 1` (the rest oracle-exact),
        // 100% at every other positive exponent, and 54-56% at the
        // negative ones, where a large-magnitude operand reciprocates to
        // an unbounded oracle enclosure. One floor covers all of them
        // with better than 2x margin against the lowest.
        min_ratio_fraction: if n == 0 {
            Divergent("no ratios in this class at all; the oracle-exact count is its census")
        } else {
            Scored(0.25)
        },
        #[allow(clippy::cast_possible_truncation)]
        max_steps_when_oracle_exact: Scored((2.0 * pad) as i128),
        // `pad` counts RELATIVE steps — the ladder's pads are applied at
        // the intermediates and carried to the result by multiplication,
        // not stamped on the result — so converting one to representable
        // steps costs the same factor the ratio conversion pays for a
        // binade crossing (`Tightness::report`), plus one step for the
        // oracle's own rounding and one for ours. (The transcendentals'
        // pads need no such conversion: `step_down(v, 4)` is already a
        // step count at the result.)
        #[allow(clippy::cast_possible_truncation)]
        max_endpoint_steps: Scored(2 * (pad as i128) + 2),
        unbounded_when_oracle_bounded: Scored(0),
    }
}

/// The per-draw domain predicate of the derivation above: every value the
/// ladder touches is in the NORMAL range, where one representable step is
/// a bounded fraction of the value.
///
/// **It is computed from the DRAW, not from the answer**, and not from a
/// window index: the ladder's intermediates are `|x|^j` for `j <= |n|`
/// (and, for `n < 0`, the reciprocal of the last one), so the whole
/// ladder stays normal exactly when each nonzero endpoint is normal and
/// `|n · log2|x||` stays inside the normal exponent range. Two mechanisms
/// leave it, and they are the two the exclusions are about: a **subnormal
/// operand** (window 1 is entirely subnormal, and there one step is up to
/// `2^-50` relative rather than `2^-52`), and **saturation** — an
/// intermediate that overflows to `±inf` or underflows to `0`, which is
/// what `|n·log2|x||` past the range means and what the huge window's
/// documented `~2^50` looseness is made of.
///
/// A zero endpoint is admitted: `mul_lo`'s own `a == 0` arm is exact
/// there, so no relative step size is needed.
fn powi_ladder_stays_normal(x: &DInterval, n: i32) -> bool {
    let m = f64::from(n.abs());
    [x.lo(), x.hi()].into_iter().all(|v| {
        if v == 0.0 {
            return true;
        }
        v.is_finite() && v.abs() >= f64::MIN_POSITIVE && (m * libm::log2(v.abs())).abs() <= 1021.0
    })
}

/// What a draw OFF the normal domain is entitled to, in the step domain,
/// which is where the subnormal range has a bound at all.
///
/// **This covers `|n| = 1` and deliberately not more.** At `|n| = 1` the
/// ladder is one `mul_lo(1.0, x)` (plus one division when `n = -1`), so
/// nothing can saturate and the only thing off the normal domain is the
/// SCALE: a pad moves an endpoint by at most 1.5 steps at its own
/// magnitude, and the rest is a monotone map, which carries a RELATIVE
/// deviation. So the count at the result is the count at the operand
/// scaled by the ratio of relative step sizes `rel(v) = ulp(v)/|v|`:
///
/// ```text
/// steps <= 1.5 · ops · rel(operand) / rel(result) + 2
/// ```
///
/// At R2's corner — `x = 2^-1024 + 2 steps`, `n = -1`, whose reciprocal
/// is a normal number just under `MAX` — that is
/// `1.5 · 2 · (2^-50 / 2^-53) + 2 = 26` against a measured 9, where the
/// normal-range formula would have entitled 6 and called the draw a
/// defect. The subnormal step size is the mechanism, and it is the term
/// the normal-range `u` does not have.
///
/// At `|n| >= 2`, leaving the normal domain means an intermediate
/// SATURATED, and no relative bound survives saturation: `pow_mag_lo`
/// pinned at `MAX` (or at `0`) is not a padded answer, it is a clamped
/// one. Those draws are argued rather than scored — see the
/// `powi[off-normal-domain]` report — and their census is printed.
fn powi_subnormal_entitlement(x: &DInterval, mine: &DInterval, n: i32) -> f64 {
    let rel = |v: f64| {
        let a = v.abs();
        if a == 0.0 || !a.is_finite() {
            return 1.0;
        }
        (a.next_up() - a) / a
    };
    let mag = |iv: &DInterval| {
        let (a, b) = (iv.lo().abs(), iv.hi().abs());
        let m = if a == 0.0 {
            b
        } else if b == 0.0 {
            a
        } else {
            a.min(b)
        };
        if m == 0.0 { f64::MIN_POSITIVE } else { m }
    };
    let ops = f64::from(n.abs()) + if n < 0 { 1.0 } else { 0.0 };
    1.5 * ops * (rel(mag(x)) / rel(mag(mine))).max(1.0) + 2.0
}

fn drive_unary(
    label: &str,
    ceiling: fn() -> Ceiling,
    huge_ceiling: fn() -> Ceiling,
    mine_f: impl Fn(DInterval) -> DInterval,
    oracle_f: impl Fn(inari::DecInterval) -> inari::DecInterval,
) {
    // The label is mixed into the stream, so the seven unary functions
    // still draw DIFFERENT cases from one another within a run — what
    // the seven distinct literal seeds used to buy — while the run as a
    // whole moves.
    let mut rng = fuzz::start(&format!("certify::{label}"));
    // Split reporting: the huge-magnitude window (index 3) exercises the
    // documented localization degradation (semantics-diffs D3) and would
    // otherwise swamp the everyday-regime statistics.
    let mut tight = Tightness::default();
    let mut tight_huge = Tightness::default();
    for i in 0..fuzz::scaled(CASES_UNARY) {
        let wi = i % 4;
        let w = WINDOWS[wi];
        let x = gen_interval(&mut rng, w.0, w.1);
        let mine = mine_f(x);
        let oracle = oracle_f(to_inari(&x));
        assert_contains(&format!("{label} case {i} x={x:?}"), &mine, &oracle, false);
        if wi == 3 { &mut tight_huge } else { &mut tight }.record(&mine, &oracle);
    }
    tight.report(label, ceiling());
    // The huge window carries the ceiling it can carry, dimension by
    // dimension: its RATIO is the documented degradation (D3) and says
    // so, while its endpoints are still scored for every function whose
    // endpoints there are the correctly-rounded ones — see
    // `TRANSCENDENTAL_HUGE` against `TRIG_HUGE`.
    tight_huge.report(&format!("{label}[huge-window]"), huge_ceiling());
}

#[test]
fn certify_sin() {
    drive_unary(
        "sin",
        TRANSCENDENTAL,
        TRIG_HUGE,
        DInterval::sin,
        inari::DecInterval::sin,
    );
}

#[test]
fn certify_cos() {
    drive_unary(
        "cos",
        TRANSCENDENTAL,
        TRIG_HUGE,
        DInterval::cos,
        inari::DecInterval::cos,
    );
}

#[test]
fn certify_tan() {
    drive_unary(
        "tan",
        TRANSCENDENTAL,
        TRANSCENDENTAL_HUGE,
        DInterval::tan,
        inari::DecInterval::tan,
    );
}

#[test]
fn certify_asin() {
    drive_unary(
        "asin",
        TRANSCENDENTAL,
        TRANSCENDENTAL_HUGE,
        DInterval::asin,
        inari::DecInterval::asin,
    );
}

#[test]
fn certify_acos() {
    drive_unary(
        "acos",
        TRANSCENDENTAL,
        TRANSCENDENTAL_HUGE,
        DInterval::acos,
        inari::DecInterval::acos,
    );
}

#[test]
fn certify_atan() {
    drive_unary(
        "atan",
        TRANSCENDENTAL,
        TRANSCENDENTAL_HUGE,
        DInterval::atan,
        inari::DecInterval::atan,
    );
}

#[test]
fn certify_sqrt() {
    drive_unary(
        "sqrt",
        ARITHMETIC,
        ARITHMETIC,
        DInterval::sqrt,
        inari::DecInterval::sqrt,
    );
}

#[test]
fn certify_atan2() {
    let mut rng = fuzz::start("certify::atan2");
    let mut tight = Tightness::default();
    // The D4 divergence, split out so that each half is scored by the
    // instrument that fits it. Over an ORIGIN-CONTAINING box this crate
    // returns the full `[-π, π]` hull where inari returns a
    // quadrant-tight enclosure, deliberately
    // (`docs/semantics-diffs.md` D4: at the `Trv` such a box carries,
    // the value cannot decide anything anyway). The width RATIO stays
    // small — both enclosures are wide — while the ENDPOINTS sit as far
    // from the oracle's as the codomain allows, measured at 9.2e18
    // representable steps. So this class keeps the ratio ceiling and
    // drops the endpoint bound, and the exclusion is on the INPUT, which
    // is what the per-endpoint bound was owed.
    let mut tight_origin = Tightness::default();
    for i in 0..fuzz::scaled(CASES_BINARY) {
        let w = WINDOWS[i % 4];
        let y = gen_interval(&mut rng, w.0, w.1);
        let x = gen_interval(&mut rng, w.0, w.1);
        let mine = y.atan2(x);
        let oracle = to_inari(&y).atan2(to_inari(&x));
        // Known-conservative oracle class (docs/semantics-diffs.md D2):
        // y touching 0 from above against x < 0 — inari says Dac, the
        // restriction is genuinely continuous (Com). Only there may our
        // decoration exceed the oracle's.
        let exception = x.hi() < 0.0 && y.lo() == 0.0;
        assert_contains(
            &format!("atan2 case {i} y={y:?} x={x:?}"),
            &mine,
            &oracle,
            exception,
        );
        if x.contains(0.0) && y.contains(0.0) {
            &mut tight_origin
        } else {
            &mut tight
        }
        .record(&mine, &oracle);
    }
    tight.report("atan2", TRANSCENDENTAL());
    tight_origin.report(
        "atan2[origin-box]",
        Ceiling {
            max_endpoint_steps: Divergent(
                "semantics-diffs D4: the origin box's [-pi, pi] hull, not a quadrant-tight enclosure",
            ),
            ..TRANSCENDENTAL()
        },
    );
}

#[test]
fn certify_powi() {
    let mut rng = fuzz::start("certify::powi");
    // One accumulator per exponent, because the entitlement is a function
    // of the exponent (`powi_ceiling`): a single accumulator can only
    // carry the ceiling of its loosest member, which for `|n| <= 31` is
    // 30x the entitlement of the `n = 1` draws sharing it. The exponent
    // travels WITH its accumulator rather than beside it in a parallel
    // array: adding an exponent to `POWI_EXPS` then cannot silently pair
    // a class with another class's ceiling.
    let mut tight: Vec<(i32, Tightness)> = POWI_EXPS
        .iter()
        .map(|&n| (n, Tightness::default()))
        .collect();
    // The draws OFF the derivation's domain (`powi_normal_domain`): a
    // subnormal intermediate, or a saturating overflow. They are not
    // unscored — each is checked against `powi_subnormal_entitlement` at
    // the draw, which is a per-draw bound and therefore stronger than an
    // accumulator ceiling — but they do not belong under a ceiling
    // derived for the normal range, and this accumulator exists so their
    // census is visible rather than absent.
    let mut off_domain = Tightness::default();
    let mut off_domain_cases = 0u64;
    for i in 0..fuzz::scaled(CASES_BINARY) {
        let w = WINDOWS[i % 4];
        let x = gen_interval(&mut rng, w.0, w.1);
        let ei = (rng.next_u64() % POWI_EXPS.len() as u64) as usize;
        let (n, _) = tight[ei];
        let mine = x.powi(n);
        let oracle = to_inari(&x).powi(n);
        assert_contains(
            &format!("powi case {i} x={x:?} n={n}"),
            &mine,
            &oracle,
            false,
        );
        // The `n = 0` class, pinned as a value rather than as a ratio
        // (`powi_ceiling`): the ladder is not entered and the answer is
        // the point one, whatever the operand's magnitude.
        if n == 0 && !mine.is_empty() {
            assert_eq!(
                (mine.lo(), mine.hi()),
                (1.0, 1.0),
                "powi case {i} x={x:?} n=0 returned {mine:?}, not the point one"
            );
        }
        if n == 0 || powi_ladder_stays_normal(&x, n) {
            tight[ei].1.record(&mine, &oracle);
            continue;
        }
        off_domain_cases += 1;
        off_domain.record(&mine, &oracle);
        // Off the normal domain the bound is per draw, because the
        // entitlement is a function of the operand's own step size. It
        // covers `|n| = 1`, where nothing can saturate; at higher
        // exponents leaving the domain IS saturation, argued in
        // `off_domain`'s report below. Only the class where both
        // enclosures are bounded carries a step distance at all.
        if n.abs() != 1 {
            continue;
        }
        let (Some(iv), false) = (oracle.interval(), mine.is_empty()) else {
            continue;
        };
        if iv.is_empty() || !iv.wid().is_finite() || !(mine.hi() - mine.lo()).is_finite() {
            continue;
        }
        let allowed = powi_subnormal_entitlement(&x, &mine, n);
        let worst = steps(mine.lo(), iv.inf()).max(steps(iv.sup(), mine.hi()));
        assert!(
            worst as f64 <= allowed,
            "powi case {i} x={x:?} n={n}: an endpoint sat {worst} steps outside the \
             correctly-rounded one, off the normal domain, where the step-domain \
             entitlement is {allowed:.1}. That entitlement is derived from the \
             operand's own step size (`powi_subnormal_entitlement`); a draw above it \
             says the derivation is wrong, not that the number is too small."
        );
    }
    for (n, acc) in &mut tight {
        acc.report(&format!("powi[n={n}]"), powi_ceiling(*n));
    }
    // The off-domain accumulator prints its census and scores nothing by
    // accumulator: every draw in it was already checked per draw above,
    // and the two dimensions a ceiling would carry are exactly the ones
    // the class diverges on. The RATIO diverges through the documented
    // negative-exponent overflow saturation — `pow_mag_lo` saturating at
    // `MAX` against a sub-subnormal truth, ~2^50 — and the UNBOUNDED
    // class is reachable here for the same reason at the other end: at
    // `x` one step above `2^-1024` with `n = -1`, `down1(x)` is exactly
    // `2^-1024` and its reciprocal overflows to `+inf` against a bounded
    // oracle. Both are sound, and neither is a number the pads entitle.
    assert!(
        off_domain_cases > 0,
        "no draw left the normal domain: window 1 is entirely subnormal, so this \
         split has stopped selecting and the per-draw bound above is vacuous"
    );
    off_domain.report(
        "powi[off-normal-domain]",
        Ceiling {
            max_ratio: Divergent("negative-exponent overflow saturation: 1/MAX against a sub-subnormal truth"),
            min_ratio_fraction: Divergent("the saturating draws are the class; how many stay comparable is not a claim"),
            max_steps_when_oracle_exact: Scored(16),
            max_endpoint_steps: Divergent("scored per draw against `powi_subnormal_entitlement` instead"),
            unbounded_when_oracle_bounded: Divergent(
                "the overflow pad: 1/down1(2^-1024 + 1 step) is +inf where the truth is just under MAX",
            ),
        },
    );
}

#[test]
fn certify_arith() {
    let mut rng = fuzz::start("certify::arith");
    let mut tights: [Tightness; 4] = Default::default();
    for i in 0..fuzz::scaled(CASES_BINARY) {
        let w = WINDOWS[i % 4];
        let a = gen_interval(&mut rng, w.0, w.1);
        let b = gen_interval(&mut rng, w.0, w.1);
        let (ia, ib) = (to_inari(&a), to_inari(&b));
        let pairs = [
            (a + b, ia + ib, 0usize),
            (a - b, ia - ib, 1),
            (a * b, ia * ib, 2),
            (a / b, ia / ib, 3),
        ];
        for (mine, oracle, k) in pairs {
            assert_contains(
                &format!("arith[{k}] case {i} a={a:?} b={b:?}"),
                &mine,
                &oracle,
                false,
            );
            tights[k].record(&mine, &oracle);
        }
    }
    for (t, l) in tights.iter_mut().zip(["add", "sub", "mul", "div"]) {
        t.report(l, ARITHMETIC());
    }
}

#[test]
fn certify_constants() {
    // Our π-family enclosures must contain inari's (which contain truth).
    let pairs = [
        (interval_transcendentals::pi(), inari::DecInterval::PI),
        (
            interval_transcendentals::frac_pi_2(),
            inari::DecInterval::FRAC_PI_2,
        ),
    ];
    for (mine, oracle) in pairs {
        assert_contains("constant", &mine, &oracle, false);
    }
    // Direction fact from docs/derivations.md §6, checked hard: inari's
    // correctly-rounded π enclosure is exactly [PI, next_up(PI)], which
    // proves PI < π < next_up(PI).
    let pi_i = inari::Interval::PI;
    assert!(core::f64::consts::PI <= pi_i.inf() && pi_i.sup() <= core::f64::consts::PI.next_up());
    // τ enclosure correctness follows by exact power-of-two scaling:
    // TAU = 2·PI exactly, next_up(2x) = 2·next_up(x) for normal x, so
    // TAU < 2π < next_up(TAU). Assert the premises and the bounds used.
    assert_eq!(core::f64::consts::TAU, 2.0 * core::f64::consts::PI);
    let tau = interval_transcendentals::tau();
    assert_eq!(tau.lo(), core::f64::consts::TAU);
    assert_eq!(tau.hi(), core::f64::consts::TAU.next_up());
    assert_eq!(
        core::f64::consts::TAU.next_up(),
        2.0 * core::f64::consts::PI.next_up()
    );
}

#[test]
fn certify_exact_ops() {
    // Endpoint-exact operations (abs/floor/min/max) differentially
    // certified like everything else. floor carries the D8 allowlist:
    // constant-on-box with an integer left endpoint is Com for us
    // (restriction-continuity) but Dac for inari (ambient), see
    // docs/semantics-diffs.md.
    let mut rng = fuzz::start("certify::exact_ops");
    for i in 0..fuzz::scaled(CASES_BINARY) {
        let w = WINDOWS[i % 4];
        let a = gen_interval(&mut rng, w.0, w.1);
        let b = gen_interval(&mut rng, w.0, w.1);
        let (ia, ib) = (to_inari(&a), to_inari(&b));
        assert_contains(&format!("abs case {i} a={a:?}"), &a.abs(), &ia.abs(), false);
        let floor_d8 = a.lo().floor() == a.hi().floor() && a.lo() == a.lo().floor();
        assert_contains(
            &format!("floor case {i} a={a:?}"),
            &a.floor(),
            &ia.floor(),
            floor_d8,
        );
        assert_contains(
            &format!("min case {i} a={a:?} b={b:?}"),
            &a.min_i(b),
            &ia.min(ib),
            false,
        );
        assert_contains(
            &format!("max case {i} a={a:?} b={b:?}"),
            &a.max_i(b),
            &ia.max(ib),
            false,
        );
    }
}
