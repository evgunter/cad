//! Differential certification against inari(+gmp/MPFR) as oracle.
//!
//! Soundness direction: truth ⊆ oracle ⊆ mine (oracle enclosures are
//! correctly rounded, hence contain truth; ours must contain the
//! oracle's). Millions of seed-pinned property cases; tightness ratios
//! are reported per function (visible with `--nocapture`).

mod common;

use common::{Rng, Tightness, assert_contains, gen_interval, to_inari};
use interval_transcendentals::DInterval;

const CASES_UNARY: u64 = 400_000;
const CASES_BINARY: u64 = 300_000;

/// Magnitude windows (binade exponents) swept per case batch: everyday
/// values, subnormal/tiny, huge (argument-reduction stress).
// Upper cap 1022: log_mag's mantissa factor reaches 2, and 2·2^1022 is
// still finite; endpoints may still overflow to +inf via width addition,
// which is a deliberate unbounded-interval test case.
const WINDOWS: [(i32, i32); 4] = [(-8, 8), (-1074, -960), (-60, 4), (30, 1022)];

fn drive_unary(
    label: &str,
    seed: u64,
    mine_f: impl Fn(DInterval) -> DInterval,
    oracle_f: impl Fn(inari::DecInterval) -> inari::DecInterval,
) {
    let mut rng = Rng(seed);
    // Split reporting: the huge-magnitude window (index 3) exercises the
    // documented localization degradation (semantics-diffs D3) and would
    // otherwise swamp the everyday-regime statistics.
    let mut tight = Tightness::default();
    let mut tight_huge = Tightness::default();
    for i in 0..CASES_UNARY {
        let wi = (i % 4) as usize;
        let w = WINDOWS[wi];
        let x = gen_interval(&mut rng, w.0, w.1);
        let mine = mine_f(x);
        let oracle = oracle_f(to_inari(&x));
        assert_contains(&format!("{label} case {i} x={x:?}"), &mine, &oracle, false);
        if wi == 3 { &mut tight_huge } else { &mut tight }.record(&mine, &oracle);
    }
    tight.report(label);
    tight_huge.report(&format!("{label}[huge-window]"));
}

#[test]
fn certify_sin() {
    drive_unary("sin", 0x5EED_0001, DInterval::sin, inari::DecInterval::sin);
}

#[test]
fn certify_cos() {
    drive_unary("cos", 0x5EED_0002, DInterval::cos, inari::DecInterval::cos);
}

#[test]
fn certify_tan() {
    drive_unary("tan", 0x5EED_0003, DInterval::tan, inari::DecInterval::tan);
}

#[test]
fn certify_asin() {
    drive_unary(
        "asin",
        0x5EED_0004,
        DInterval::asin,
        inari::DecInterval::asin,
    );
}

#[test]
fn certify_acos() {
    drive_unary(
        "acos",
        0x5EED_0005,
        DInterval::acos,
        inari::DecInterval::acos,
    );
}

#[test]
fn certify_atan() {
    drive_unary(
        "atan",
        0x5EED_0006,
        DInterval::atan,
        inari::DecInterval::atan,
    );
}

#[test]
fn certify_sqrt() {
    drive_unary(
        "sqrt",
        0x5EED_0007,
        DInterval::sqrt,
        inari::DecInterval::sqrt,
    );
}

#[test]
fn certify_atan2() {
    let mut rng = Rng(0x5EED_0008);
    let mut tight = Tightness::default();
    for i in 0..CASES_BINARY {
        let w = WINDOWS[(i % 4) as usize];
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
    tight.report("atan2");
}

#[test]
fn certify_powi() {
    let mut rng = Rng(0x5EED_0009);
    let mut tight = Tightness::default();
    let exps: [i32; 12] = [0, 1, 2, 3, 4, 5, 7, 12, -1, -2, -3, 31];
    for i in 0..CASES_BINARY {
        let w = WINDOWS[(i % 4) as usize];
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
        tight.record(&mine, &oracle);
    }
    tight.report("powi");
}

#[test]
fn certify_arith() {
    let mut rng = Rng(0x5EED_000A);
    let mut tights: [Tightness; 4] = Default::default();
    for i in 0..CASES_BINARY {
        let w = WINDOWS[(i % 4) as usize];
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
        t.report(l);
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
