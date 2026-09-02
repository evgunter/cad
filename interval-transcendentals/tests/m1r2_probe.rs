//! CERT-M1 R2 review probes (not for merge). Two questions about the
//! D78(a) ceiling that the delivered `certify_powi` cannot answer:
//!
//! 1. Is the huge-window "no ceiling" exclusion argued only for negative
//!    exponents? Split that accumulator by exponent sign and look.
//! 2. Does the derivation's per-multiplication `u <= 3·2^-53` survive a
//!    subnormal operand whose reciprocal is still bounded (`x` just above
//!    `2^-1024`, `n = -1`)? Count the endpoint steps and the width ratio.
#![cfg(feature = "oracle-inari")]

mod common;

use common::{Tightness, assert_contains, gen_interval, steps, to_inari};
use interval_transcendentals::DInterval;
use test_utils::fuzz;

const WINDOWS: [(i32, i32); 4] = [(-8, 8), (-1074, -960), (-60, 4), (30, 1022)];
const POWI_EXPS: [i32; 12] = [0, 1, 2, 3, 4, 5, 7, 12, -1, -2, -3, 31];

#[test]
fn probe_huge_window_split_by_sign() {
    let mut rng = fuzz::start("m1r2::huge-split");
    let mut pos = Tightness::default();
    let mut neg = Tightness::default();
    for i in 0..fuzz::scaled(300_000) {
        let w = WINDOWS[3];
        let x = gen_interval(&mut rng, w.0, w.1);
        let n = POWI_EXPS[(rng.next_u64() % POWI_EXPS.len() as u64) as usize];
        if n == 0 {
            continue;
        }
        let mine = x.powi(n);
        let oracle = to_inari(&x).powi(n);
        assert_contains(
            &format!("powi huge case {i} x={x:?} n={n}"),
            &mine,
            &oracle,
            false,
        );
        if n > 0 { &mut pos } else { &mut neg }.record(&mine, &oracle);
    }
    pos.report("powi[huge-window, n>0]", None);
    neg.report("powi[huge-window, n<0]", None);
}

#[test]
fn probe_subnormal_reciprocal_corner() {
    // x just above 2^-1024: 1/x is bounded (just below MAX) and a
    // representable step at x is 2^-1074, i.e. ~2^-50 RELATIVE — four
    // normal ulps. The derivation's `u` assumes 2^-52.
    let base = 2f64.powi(-1024);
    for k in [1u32, 2, 4, 8, 64, 1024] {
        let mut x = base;
        for _ in 0..k {
            x = x.next_up();
        }
        let xi = DInterval::point(x);
        let mine = xi.powi(-1);
        let oracle = to_inari(&xi).powi(-1);
        let iv = oracle.interval().expect("bounded");
        if !mine.hi().is_finite() {
            // The overflow pad: `down1(x)` fell to exactly 2^-1024 and its
            // reciprocal is +inf. Sound, and it is the zero-tolerance
            // `mine_unbounded_oracle_bounded` class of `Tightness`.
            println!(
                "PROBE n=-1 x=2^-1024+{k}steps: MINE UNBOUNDED [{:e},{:e}] vs oracle [{:e},{:e}]",
                mine.lo(),
                mine.hi(),
                iv.inf(),
                iv.sup()
            );
            continue;
        }
        let (lo_steps, hi_steps) = (steps(mine.lo(), iv.inf()), steps(iv.sup(), mine.hi()));
        let ratio = (mine.hi() - mine.lo()) / iv.wid();
        println!(
            "PROBE n=-1 x=2^-1024+{k}steps: mine=[{:e},{:e}] oracle=[{:e},{:e}] lo_steps={lo_steps} hi_steps={hi_steps} ratio={ratio}",
            mine.lo(),
            mine.hi(),
            iv.inf(),
            iv.sup()
        );
    }
}
