//! The successful explicit-[`Tolerance::init`] path.
//!
//! This lives in its own integration-test binary — i.e. its own process —
//! because the global tolerance commits exactly once per process: the lib
//! test binary's single global-state test exercises the `get()`-from-env
//! path, so the init-first path needs a fresh process. This file must
//! contain exactly ONE `#[test]` (a second would race it for first touch
//! of the global).
//!
//! That premise is load-bearing and was once quietly broken: `tests/all.rs`
//! aggregated every suite into ONE binary to save CI link time, which put
//! this test in the same process as six other suites that touch
//! `Tolerance`. `cargo test -p geom-core` then failed on a clean tree —
//! invisibly, because running it FILTERED still passes (nothing else runs
//! to claim the lock first), and because CI stayed green: nextest forks
//! per test, which `ci.yml` calls out as "strictly safer for the tolerance
//! OnceLock". It now has its own `[[test]]` target again, and
//! `all.rs`'s aggregation guard exempts it BY NAME so re-adding it is a
//! deliberate act rather than an accident.
//!
//! Note this binary passes even under a malformed `CAD_TOLERANCE_EPS`, by
//! design: an explicit `init` never consults the environment. The loud
//! failure for a bogus env value lives in the lib test binary's
//! `global_get_env_sanity_and_once_semantics`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tolerance, ToleranceError};

#[test]
fn explicit_init_commits_once_and_ignores_env() {
    let tolerance = Tolerance::with_eps(2.5e-9);
    assert_eq!(Tolerance::init(tolerance), Ok(()));

    // get() returns the explicitly installed value (env vars, if any, are
    // never consulted on this path)...
    assert_eq!(Tolerance::get(), tolerance);
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
    assert_eq!(Tolerance::get(), tolerance);
}
