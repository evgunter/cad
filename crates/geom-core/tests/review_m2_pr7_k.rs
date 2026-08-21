//! M2 PR 7 e2e review — the run-configured ambiguity multiplier K:
//! a margin in (10ε, 25ε) must be DEFINITE at the default K = 10 and
//! flip to indeterminate (escalated) under `CAD_AMBIGUITY_K=25`
//! (re-exec pattern — the `Tolerance` OnceLock commits per process);
//! invalid env values must surface as recorded typed
//! `ToleranceEnvError`s while the run falls back to the default
//! (never a silent acceptance of K ≤ 1 / NaN / garbage).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Band, Decide, Sign, Tolerance};

/// Probe (ignored in normal runs): classify a margin of 17ε and print
/// the outcome plus the recorded env errors.
#[test]
#[ignore]
fn print_k_flip_probe() {
    let t = Tol::witness().get();
    let margin = 17.0 * t.eps;
    let band = Band::linear(Tol::witness()).unwrap();
    let out = match margin.sign_within(band) {
        Ok(Sign::Positive) => "positive".to_string(),
        Ok(Sign::Zero) => "zero".to_string(),
        Ok(Sign::Negative) => "negative".to_string(),
        Err(e) => format!("indeterminate({e})"),
    };
    println!("KFLIP k={} outcome={out}", t.k);
    println!("KFLIP env_errors={}", Tolerance::env_init_errors().len());
}

fn run_probe(env: &[(&str, &str)]) -> String {
    let exe = std::env::current_exe().unwrap();
    let mut cmd = std::process::Command::new(&exe);
    // Self re-exec: name the probe by MODULE PATH, not by bare fn name.
    // `tests/all.rs` aggregates every suite into one binary, so libtest sees
    // this probe as `<this_module>::print_k_flip_probe`. Stripping the leading crate name off
    // `module_path!()` yields the right filter in the aggregated layout AND in
    // a standalone one (where `module_path!()` has no `::` at all).
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::print_k_flip_probe"),
        None => "print_k_flip_probe".to_string(),
    };
    cmd.args([probe.as_str(), "--ignored", "--exact", "--nocapture"]);
    for (k, v) in env {
        cmd.env(k, v);
    }
    let out = cmd.output().unwrap();
    assert!(out.status.success(), "probe run failed: {out:?}");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// The decision on a 17ε margin flips definite → indeterminate when
/// K goes 10 → 25: K is genuinely load-bearing in the shipped
/// `sign_within` door, not just plumbed into `Band` accessors.
#[test]
fn margin_between_10_and_25_eps_flips_with_k() {
    let default = run_probe(&[]);
    assert!(
        default.contains("k=10") && default.contains("outcome=positive"),
        "17ε must be definite at K = 10; got:\n{default}"
    );
    let widened = run_probe(&[("CAD_AMBIGUITY_K", "25")]);
    assert!(
        widened.contains("k=25") && widened.contains("outcome=indeterminate"),
        "17ε must escalate at K = 25; got:\n{widened}"
    );
}

/// Invalid env K values: the process still runs (get() is total) at
/// the DEFAULT K, and the failure is recorded as a typed env error —
/// checked end-to-end for K ∈ {0, 1, −5, NaN, garbage}.
#[test]
fn invalid_env_k_values_fall_back_and_record_typed() {
    for bad in ["0", "1", "-5", "NaN", "ten"] {
        let out = run_probe(&[("CAD_AMBIGUITY_K", bad)]);
        assert!(
            out.contains("k=10"),
            "invalid K {bad:?} must fall back to the default; got:\n{out}"
        );
        assert!(
            out.contains("env_errors=1"),
            "invalid K {bad:?} must record a typed env error; got:\n{out}"
        );
    }
}
