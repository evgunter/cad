//! The offset-fit doors' `OffsetFitError::Band` arm, reached through
//! the real doors from a tolerance the run's own validator admits.
//!
//! The doors take the run's ε as the `Tol` witness alone and derive the
//! meters' band from it, so the only way to reach the arm is to COMMIT
//! a pathological tolerance. The global commits once per process, so
//! each arm is an `#[ignore]`d probe re-exec'd in its own process by
//! `the_band_arm_is_reachable_through_the_doors` — `geom-core`'s
//! `band_tolerance.rs` pattern. The spawner touches no global itself,
//! and a probe is inert in an ordinary run.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom_brep::offset_fit::{OffsetFitError, approx_offset_surface, certify_offset, fit_offset};
use geom_core::tolerance::{DEFAULT_K, Tolerance};
use geom_core::{BandError, BandField, Tol};

use crate::shared::fixture::quarter_cylinder;

/// Commits `(eps, k)` through the validating door and hands back the
/// witness. Panics if the validator refuses, which is the premise.
fn commit(eps: f64, k: f64) -> Tol {
    Tolerance::init(Tolerance { eps, k })
        .expect("the run's own validator admits this pair — that is the premise");
    let tol = Tol::witness();
    assert_eq!((tol.eps(), tol.k()), (eps, k), "the committed pair is ours");
    tol
}

/// Drives the three doors that take a base at `tol` and checks each
/// refuses with exactly `want`, rendered with the one recourse.
fn every_door_refuses(tol: Tol, want: BandError) {
    let base = quarter_cylinder(1.0, 1.0);
    let fit = fit_offset(&base, 0.1, tol).map(|_| ());
    let certify = certify_offset(&base, &base, 0.1, tol).map(|_| ());
    let mint = approx_offset_surface(Arc::new(base.clone()), 0.1, tol).map(|_| ());
    for (door, got) in [
        ("fit_offset", fit),
        ("certify_offset", certify),
        ("approx_offset_surface", mint),
    ] {
        let Err(error) = got else {
            panic!("{door} did not refuse at eps={:e}", tol.eps());
        };
        assert_eq!(error, OffsetFitError::Band(want), "{door}");
        let msg = error.to_string();
        assert!(
            msg.contains("Recourse: run at a less extreme tolerance"),
            "{door}: {msg}"
        );
        assert!(!msg.contains(&want.to_string()), "{door}: {msg}");
    }
}

/// PROBE (own process). The OVERFLOW arm: ε = `f64::MAX` at the
/// ratified default K, so K·ε is not a threshold.
#[test]
#[ignore]
fn probe_overflow_arm_through_the_doors() {
    let tol = commit(f64::MAX, DEFAULT_K);
    every_door_refuses(
        tol,
        BandError::InvalidValue {
            field: BandField::Escalate,
            value: f64::INFINITY,
        },
    );
    println!("PROBE overflow-through-the-doors OK");
}

/// PROBE (own process). The COLLAPSE arm: the subnormal ε = 2⁻¹⁰²³ with
/// the least admitted K, so K·ε rounds back onto ε.
#[test]
#[ignore]
fn probe_collapse_arm_through_the_doors() {
    let eps = f64::from_bits(1u64 << 51);
    let tol = commit(eps, 1.0f64.next_up());
    every_door_refuses(
        tol,
        BandError::Empty {
            zero: eps,
            escalate: eps,
        },
    );
    println!("PROBE collapse-through-the-doors OK at eps={eps:e}");
}

/// **Both arms of `Band::linear` reach the offset-fit doors' `Band`
/// arm**, each from a tolerance the run's validator admits. Each row is
/// a re-exec'd child because each commits a different global.
#[test]
fn the_band_arm_is_reachable_through_the_doors() {
    for probe in [
        "probe_overflow_arm_through_the_doors",
        "probe_collapse_arm_through_the_doors",
    ] {
        spawn_probe(probe);
    }
}

/// Re-execs this binary at one `#[ignore]`d probe, in its own process.
/// Names the probe by MODULE PATH: `tests/all.rs` aggregates every
/// suite into one binary, so libtest sees it as `<module>::<probe>`.
fn spawn_probe(probe: &str) {
    let exe = std::env::current_exe().expect("test exe path");
    let filter = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::{probe}"),
        None => probe.to_string(),
    };
    let out = std::process::Command::new(&exe)
        .args([filter.as_str(), "--ignored", "--exact", "--nocapture"])
        .env_remove("CAD_TOLERANCE_EPS")
        .env_remove("CAD_AMBIGUITY_K")
        .output()
        .expect("the probe process spawns");
    assert!(
        out.status.success(),
        "{probe} failed:\n{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("1 passed"),
        "{probe} did not RUN (a filter that matches nothing also exits 0):\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
}
