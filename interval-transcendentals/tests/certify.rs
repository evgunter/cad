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

use common::{Ceiling, Tightness, assert_contains, gen_interval, to_inari};
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
/// **`max_ratio`.** The structural worst is `≈ 4·pad + 1 = 17` — see
/// `Tightness::report` for why it is `4·pad`, not `2·pad`: the ratio is
/// on widths and an outward step across a binade boundary is worth two
/// oracle ulps. Measured maxima across seeds and efforts run 8–12
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
    max_ratio: 64.0,
    min_ratio_fraction: 0.25,
    max_steps_when_oracle_exact: 16,
    max_endpoint_steps: Some(8),
};

/// The same for the 1-step ops (`+ − × ÷ sqrt`): structural worst
/// `4·1 + 1 = 5`, measured 3, ceiling 8. Here the oracle-exact class is
/// NOT empty — 112 859 draws for `sqrt`, 1 149 for `add` — and our
/// widest enclosure on it measured **2 steps** against an entitlement of
/// 2 (`down1`..`up1`); the allowance of 8 is 4x that.
const ARITHMETIC: fn() -> Ceiling = || Ceiling {
    max_ratio: 8.0,
    min_ratio_fraction: 0.25,
    max_steps_when_oracle_exact: 8,
    max_endpoint_steps: Some(4),
};

/// The exponents `certify_powi` draws.
const POWI_EXPS: [i32; 12] = [0, 1, 2, 3, 4, 5, 7, 12, -1, -2, -3, 31];

/// What `x^n` is entitled to, **as a function of the exponent**.
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
/// **The derivation, by induction over the ladder.** Write `u` for the
/// relative deviation one padded multiplication may introduce: `mul_lo`
/// / `mul_hi` take the round-to-nearest product and step it one
/// representable place outward when it is inexact, so they land within
/// **1.5 ulp** of the exact product, `u <= 1.5·2^-52 = 3·2^-53`. Let
/// `e(k)` bound the relative deviation of our computed `x^k` from the
/// true one. The ladder only ever forms `x^{a+b}` from `x^a` and `x^b`,
/// so `e(a+b) <= e(a) + e(b) + u`; and **`e(1) <= u`, not `0`** — the
/// first accumulator step is `1.0 · base`, whose product is exact, but
/// `mul_exact`'s 2Prod witness is gated on the product MAGNITUDE
/// (`2^-960`, where the residual itself can underflow), so in the
/// subnormal window it declines and the step pads anyway. The induction
/// then gives
///
/// ```text
/// e(m) <= m·u
/// ```
///
/// for every `m >= 1`, whatever the bit pattern of `m` (walked on the
/// ladder for `m = 2, 3, 5, 7`: `2u, 3u, 5u, 7u`). In representable
/// STEPS at the result — one step is at least `2^-53` relative — that is
/// `3m` steps per endpoint, and a negative exponent's reciprocal
/// preserves relative width and pads 1.5 ulp on each side, for `3` more.
///
/// The `e(1) = 0` reading is the one this row's first draft carried, and
/// the instrument refuted it before the ceiling landed: at `n = 1` the
/// subnormal window yields 2-step widths on an oracle-exact value and
/// ratios to 3.0, which an entitlement of zero steps calls a defect.
///
/// **From steps to a width ratio** by `Tightness::report`'s rule: our
/// width exceeds a correctly-rounded one by at most `2·pad` outward
/// steps and a step across a binade boundary is worth two oracle ulps,
/// so the ratio is bounded by `4·pad + 1` — `12·|n| + 1`, and
/// `12·|n| + 13` with the reciprocal. The oracle-exact class is scored
/// absolutely instead, at the same `2·pad` steps of width.
///
/// **Measured against it at effort 8** (1.8M draws, worst ratio per
/// exponent against the entitlement): `n = 1` 3.0/13, `2` 2.0/25, `3`
/// 6.0/37, `4` 8.0/49, `5` 13.0/61, `7` 20.0/85, `12` 39.0/145, `31`
/// 120.0/373, `−1` 17.0/25, `−2` 6.0/37, `−3` 12.0/49. The measured
/// worst tracks `≈ 4·|n|` where the derivation allows `12·|n|`, so the
/// entitlement sits ~3x over the draws at every exponent — and it is
/// the DERIVATION that is the ceiling here, not a fitted multiple of
/// the measurement: a draw above it says the induction above is wrong.
///
/// **`n = 0` carries no ceiling and is not thereby unguarded.** `powi`
/// returns `[1, 1]` without touching the ladder, so every draw lands in
/// the oracle-exact bucket and a width ratio does not exist for it — a
/// `Ceiling` over that accumulator would be a `max` over nothing, which
/// is the vacuity `min_ratio_fraction` exists to refuse. The class is
/// pinned exactly at the draw site instead, which is stronger than any
/// ratio: the answer must be the point `[1, 1]` itself.
fn powi_ceiling(n: i32) -> Option<Ceiling> {
    if n == 0 {
        return None;
    }
    let pad = 3.0 * f64::from(n.abs()) + if n < 0 { 3.0 } else { 0.0 };
    Some(Ceiling {
        max_ratio: 4.0 * pad + 1.0,
        // Measured comparable-ratio yields over the three
        // ceiling-carrying windows, at effort 8: 87% at `n = 1` (the
        // rest oracle-exact) and 100% at every other positive exponent;
        // 39–47% at the negative ones, where a subnormal-window operand
        // reciprocates to an unbounded oracle enclosure about half the
        // time. Two floors rather than one, each with better than 2x
        // margin against its own class — a single floor set for the
        // negative exponents would be vacuous for the positive ones.
        min_ratio_fraction: if n < 0 { 0.15 } else { 0.25 },
        #[allow(clippy::cast_possible_truncation)]
        max_steps_when_oracle_exact: (2.0 * pad) as i128,
        // `pad` counts RELATIVE steps — the ladder's pads are applied at
        // the intermediates and carried to the result by multiplication,
        // not stamped on the result — so converting one to representable
        // steps costs the same binade factor of 2 the ratio derivation
        // pays, plus one step for the oracle's own rounding and one for
        // ours. (The transcendentals' pads need no such conversion:
        // `step_down(v, 4)` is already a step count at the result.)
        #[allow(clippy::cast_possible_truncation)]
        max_endpoint_steps: Some(2 * (pad as i128) + 2),
    })
}

fn drive_unary(
    label: &str,
    ceiling: fn() -> Ceiling,
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
    tight.report(label, Some(ceiling()));
    // No ceiling on the huge window: it is where localization is
    // DOCUMENTED to degrade to the trivial enclosure (semantics-diffs
    // D3), so its ratios are unbounded by design.
    tight_huge.report(&format!("{label}[huge-window]"), None);
}

#[test]
fn certify_sin() {
    drive_unary(
        "sin",
        TRANSCENDENTAL,
        DInterval::sin,
        inari::DecInterval::sin,
    );
}

#[test]
fn certify_cos() {
    drive_unary(
        "cos",
        TRANSCENDENTAL,
        DInterval::cos,
        inari::DecInterval::cos,
    );
}

#[test]
fn certify_tan() {
    drive_unary(
        "tan",
        TRANSCENDENTAL,
        DInterval::tan,
        inari::DecInterval::tan,
    );
}

#[test]
fn certify_asin() {
    drive_unary(
        "asin",
        TRANSCENDENTAL,
        DInterval::asin,
        inari::DecInterval::asin,
    );
}

#[test]
fn certify_acos() {
    drive_unary(
        "acos",
        TRANSCENDENTAL,
        DInterval::acos,
        inari::DecInterval::acos,
    );
}

#[test]
fn certify_atan() {
    drive_unary(
        "atan",
        TRANSCENDENTAL,
        DInterval::atan,
        inari::DecInterval::atan,
    );
}

#[test]
fn certify_sqrt() {
    drive_unary(
        "sqrt",
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
    tight.report("atan2", Some(TRANSCENDENTAL()));
    tight_origin.report(
        "atan2[origin-box]",
        Some(Ceiling {
            max_endpoint_steps: None,
            ..TRANSCENDENTAL()
        }),
    );
}

#[test]
fn certify_powi() {
    let mut rng = fuzz::start("certify::powi");
    // Split like drive_unary: the huge window (index 3) is where the
    // documented negative-exponent overflow-saturation looseness lives
    // (|x|^|n| overflows f64, so pow_mag_lo saturates at MAX and the
    // reciprocal's upper bound becomes ~1/MAX ~ 2^-1024 against a
    // sub-subnormal truth -> ratios up to ~2^50+2; sound, crude, and
    // unreachable at kernel magnitudes).
    // One accumulator per exponent, because the entitlement is a
    // function of the exponent (see `powi_ceiling`): a single
    // accumulator can only carry the ceiling of its loosest member,
    // which for `|n| <= 31` is 30x the entitlement of the `n = 1` draws
    // sharing it.
    let mut tight: [Tightness; POWI_EXPS.len()] = Default::default();
    let mut tight_huge = Tightness::default();
    for i in 0..fuzz::scaled(CASES_BINARY) {
        let wi = i % 4;
        let w = WINDOWS[wi];
        let x = gen_interval(&mut rng, w.0, w.1);
        let ei = (rng.next_u64() % POWI_EXPS.len() as u64) as usize;
        let n = POWI_EXPS[ei];
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
        if wi == 3 {
            tight_huge.record(&mine, &oracle);
        } else {
            tight[ei].record(&mine, &oracle);
        }
    }
    for (ei, t) in tight.iter_mut().enumerate() {
        let n = POWI_EXPS[ei];
        t.report(&format!("powi[n={n}]"), powi_ceiling(n));
    }
    // The huge window keeps NO ceiling, and the reason is the documented
    // negative-exponent overflow saturation above: `pow_mag_lo`
    // saturating at MAX against a sub-subnormal truth is a sound answer
    // whose ratio reaches ~2^50, which no entitlement derived from the
    // pads can cover. It is unreachable at kernel magnitudes.
    tight_huge.report("powi[huge-window]", None);
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
        t.report(l, Some(ARITHMETIC()));
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
