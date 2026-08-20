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
//! bounds the result.
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
};

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
        tight.record(&mine, &oracle);
    }
    tight.report("atan2", Some(TRANSCENDENTAL()));
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
    let mut tight = Tightness::default();
    let mut tight_huge = Tightness::default();
    let exps: [i32; 12] = [0, 1, 2, 3, 4, 5, 7, 12, -1, -2, -3, 31];
    for i in 0..fuzz::scaled(CASES_BINARY) {
        let wi = i % 4;
        let w = WINDOWS[wi];
        let x = gen_interval(&mut rng, w.0, w.1);
        let n = exps[(rng.next_u64() % exps.len() as u64) as usize];
        let mine = x.powi(n);
        let oracle = to_inari(&x).powi(n);
        assert_contains(
            &format!("powi case {i} x={x:?} n={n}"),
            &mine,
            &oracle,
            false,
        );
        if wi == 3 { &mut tight_huge } else { &mut tight }.record(&mine, &oracle);
    }
    // No ceiling, and this one is a DEFERRAL rather than an
    // unguardable — S134 / §D row D78 schedules it. `powi`'s enclosure
    // is a COMPOSITION of `mul` pads (binary exponentiation, plus a
    // division for a negative exponent), so what it is entitled to is a
    // function of the exponent, not a constant; an exponent-dependent
    // ceiling is derivable and is not derived here. The measured worst
    // moves with the seed (117 at effort 1, 122 at effort 2, |n| <= 31),
    // which is exactly why a fitted constant would be the wrong answer.
    // Each COMPONENT step is bounded by `certify_arith` and
    // `pad_contract.rs` already, so this is a gap in the composed bound,
    // not in the pads.
    tight.report("powi", None);
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
