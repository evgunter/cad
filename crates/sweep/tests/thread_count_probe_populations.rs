//! **The `probe` sample population does not move with the thread
//! count** — what k-lint counts, read at 1 and 4 threads through the
//! two doors whose walks run their faces or shells in parallel:
//! `topo::mass_properties` and `topo::classify_shells`.
//!
//! These rows belonged to `mass_props_are_thread_count_invariant` and
//! `shell_census_is_thread_count_invariant`, and they live apart for
//! one reason: the sink only exists in a `probe` build, so the probe
//! sweep (`scripts/k_probe_sweep.sh`) is the only thing that runs them,
//! and it selects by module. While they sat in those suites the sweep
//! re-ran the suites' ungated golden rows too — minutes of debug-build
//! walks the `test` job already runs at the same default ε — to reach
//! the one row nothing else executes. A file of its own is what lets
//! the sweep select exactly the probe-gated half.
//!
//! **Rostered as EXECUTED** (`scripts/gates/probe-suite-census.sh`'s
//! `RUN_FLOOR`, which `k_probe_sweep.sh` derives its default-selection
//! loop from). A row asserting the population does not move is worth
//! nothing if it only compiles: an inert pin reports the same green as
//! one that ran.
#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{arc_section, on_pool, stacked};
use geom_core::Probe;
use geom_core::Tol;
use geom_core::k_stats::{start_recording, take_samples};
use sweep::loft_body;
use topo::Body;

/// The arc loft at the recording scalar, scaled with ε as its f64
/// twins in both origin suites are.
fn probe_arc_loft() -> Body<Probe> {
    let eps = Tol::witness().get().eps;
    loft_body::<Probe>(
        &[arc_section(1.0e9 * eps), arc_section(1.0e9 * eps)],
        &stacked(&[0.0, 1.0], 1.0e9 * eps),
        1,
        Tol::witness(),
    )
    .expect("the probe arc loft lofts")
    .body
}

/// The samples `walk` records on a pool of `threads` threads.
fn population(
    threads: usize,
    walk: impl Fn() + Send + Sync,
) -> Vec<(&'static str, u64, &'static str)> {
    on_pool(threads, || {
        start_recording();
        walk();
        take_samples()
            .iter()
            .map(|s| (s.predicate, s.margin.to_bits(), s.outcome.token()))
            .collect::<Vec<_>>()
    })
}

fn assert_identical(walk: impl Fn() + Send + Sync) {
    let one = population(1, &walk);
    let four = population(4, &walk);
    assert!(
        !one.is_empty(),
        "the probe lane recorded no sample — the comparison below is vacuous"
    );
    assert_eq!(
        one, four,
        "the sample population moved with the thread count"
    );
}

/// The face walk of `topo::props`.
#[test]
fn the_mass_props_sample_population_is_identical_at_one_and_four_threads() {
    let body = probe_arc_loft();
    assert_identical(|| {
        let _ = topo::mass_properties(&body, Tol::witness());
    });
}

/// The shell walk of the census door.
#[test]
fn the_shell_census_sample_population_is_identical_at_one_and_four_threads() {
    let body = probe_arc_loft();
    assert_identical(|| {
        let _ = topo::classify_shells(&body, Tol::witness());
    });
}
