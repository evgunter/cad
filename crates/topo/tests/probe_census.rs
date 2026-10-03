//! Review probe (review/f34): dump the FULL per-sample census of both
//! twin boolean configurations at both scales, for merge-base-vs-tip
//! byte-identity diffing (T3).
//!
//! **CI EXECUTES THIS SUITE.** It is rostered in
//! `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`) and run under the
//! DEFAULT selection by `scripts/k_probe_sweep.sh`, whose tally is floored
//! by `--check-executed`, so every assertion below is a gate and a red here
//! fails the merge. By hand:
//! `cargo test -p topo --features probe --test all -- probe_census::`.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::common;

use common::brick;
use geom_core::Tol;
use geom_core::k_stats::{self, Probe};
use topo::{BooleanResult, subtract};

fn bx(s: f64, x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> topo::Body<Probe> {
    let f = |v: f64| v * s;
    brick::<Probe>(
        (f(x.0), f(x.1)),
        (f(y.0), f(y.1)),
        (f(z.0), f(z.1)),
        Tol::witness(),
    )
}

#[test]
fn dump_full_census() {
    for scale in [1e-3, 1.0] {
        let fin = |what, b| topo::test_support::finished(what, b, Tol::witness());
        let a1 = fin("a1", bx(scale, (0.0, 2.0), (0.0, 2.0), (0.0, 2.0)));
        let b1 = fin("b1", bx(scale, (1.0, 3.0), (1.0, 3.0), (1.0, 3.0)));
        let a2 = fin("a2", bx(scale, (0.0, 4.0), (0.0, 4.0), (0.0, 1.0)));
        let b2 = fin("b2", bx(scale, (1.0, 2.0), (1.0, 2.0), (-1.0, 2.0)));
        k_stats::start_recording();
        let r = match subtract(&a1, &b1, Tol::witness()).expect("corner") {
            BooleanResult::Body(b) => b,
            other => panic!("corner: {other:?}"),
        };
        topo::validate_pseudomanifold(&r.body, &topo::ContactRecords::default(), Tol::witness())
            .expect("census");
        subtract(&a2, &b2, Tol::witness()).expect("pocket");
        for s in k_stats::take_samples() {
            println!(
                "CEN {} {} {:?} {:?}",
                scale, s.predicate, s.outcome, s.margin
            );
        }
    }
}
