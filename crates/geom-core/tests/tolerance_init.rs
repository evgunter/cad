//! The successful explicit-[`Tolerance::init`] path.
//!
//! This lives in its own integration-test binary — i.e. its own process —
//! because the global tolerance commits exactly once per process: the lib
//! test binary's single global-state test exercises the `get()`-from-env
//! path, so the init-first path needs a fresh process. This file must
//! contain exactly ONE `#[test]` (a second would race it for first touch
//! of the global).
//!
//! Note this binary passes even under a malformed `CAD_TOLERANCE_EPS`, by
//! design: an explicit `init` never consults the environment. The loud
//! failure for a bogus env value lives in the lib test binary's
//! `global_get_env_sanity_and_once_semantics`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tolerance, ToleranceError};

#[test]
fn explicit_init_commits_once_and_ignores_env() {
    let tolerance = Tolerance {
        eps: 2.5e-9,
        eps_angular: 3.5e-8,
    };
    assert_eq!(Tolerance::init(tolerance), Ok(()));

    // get() returns the explicitly installed value (env vars, if any, are
    // never consulted on this path)...
    assert_eq!(Tolerance::get(), tolerance);
    // ...and no env error is recorded.
    assert_eq!(Tolerance::env_init_error(), None);

    // A second init fails with a typed error carrying current + attempted.
    let attempted = Tolerance {
        eps: 1e-6,
        eps_angular: 1e-7,
    };
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
