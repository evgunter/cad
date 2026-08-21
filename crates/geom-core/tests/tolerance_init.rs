//! The successful explicit-[`Tolerance::init`] path.
//!
//! The global tolerance commits exactly ONCE PER PROCESS, and this suite
//! needs to be the first thing to touch it. `tests/all.rs` aggregates
//! every suite into one binary (one process under `cargo test`), and
//! other suites there touch `Tolerance` — so this cannot simply assert
//! against the ambient global: whichever suite ran first would have
//! committed it, and `init` would return `AlreadyInitialized`.
//!
//! So the assertions run in a FRESH PROCESS, via the same self-re-exec
//! pattern the multi-ε suites use (`ambiguity_k_env.rs`,
//! `review_m2_pr7_k.rs`, `review_m0_pr2.rs`): an `#[ignore]`d
//! probe holds the real assertions, and the driver test below re-execs
//! this binary filtered to exactly that probe, where it is the only test
//! running and therefore the first toucher of the lock.
//!
//! Note this passes even under a malformed `CAD_TOLERANCE_EPS`, by
//! design: an explicit `init` never consults the environment. The loud
//! failure for a bogus env value lives in the lib test binary's
//! `global_get_env_sanity_and_once_semantics`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tolerance, ToleranceError};
use geom_core::Tol;

/// The real assertions. `#[ignore]`d because it is only valid as the sole
/// test in a process; the driver below supplies that.
#[test]
#[ignore]
fn explicit_init_probe() {
    let tolerance = Tolerance::with_eps(2.5e-9);
    assert_eq!(Tolerance::init(tolerance), Ok(()));

    // get() returns the explicitly installed value (env vars, if any, are
    // never consulted on this path)...
    assert_eq!(Tol::witness().get(), tolerance);
    // ...and no env error is recorded.
    assert!(Tolerance::env_init_errors().is_empty());

    // A second init fails with a typed error carrying current + attempted.
    let attempted = Tolerance::with_eps(1e-6);
    assert_eq!(
        Tolerance::init(attempted),
        Err(ToleranceError::AlreadyInitialized {
            current: tolerance,
            attempted,
        })
    );

    // The committed value is unchanged.
    assert_eq!(Tol::witness().get(), tolerance);
    println!("TOLERANCE_INIT_PROBE_OK");
}

#[test]
fn explicit_init_commits_once_and_ignores_env() {
    let exe = std::env::current_exe().unwrap();
    // Name the probe by MODULE PATH, not bare fn name: aggregated into
    // `tests/all.rs` libtest sees it as `<this_module>::explicit_init_probe`,
    // while standalone `module_path!()` has no `::` at all.
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::explicit_init_probe"),
        None => "explicit_init_probe".to_string(),
    };
    let out = std::process::Command::new(&exe)
        .args([probe.as_str(), "--ignored", "--exact", "--nocapture"])
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success() && text.contains("TOLERANCE_INIT_PROBE_OK"),
        "explicit-init probe failed in its fresh process:\n{text}\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
