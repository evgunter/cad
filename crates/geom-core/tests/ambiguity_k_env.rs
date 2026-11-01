//! End-to-end check that the run-configured ambiguity multiplier K
//! ([`geom_core::tolerance::ENV_K`], M2 PR 7) reaches [`Band::linear`]:
//! re-exec this binary with the env var set (the OnceLock commits per
//! process, so the override needs its own process — the same pattern
//! as the multi-ε suites).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Band;
use geom_core::Tol;

/// Printer probe for the re-exec (ignored in normal runs).
#[test]
#[ignore]
fn print_band_ratio() {
    let band = Band::linear(Tol::witness()).unwrap();
    println!("KPROBE ratio={}", band.escalate() / band.zero());
    println!("KPROBE k={}", Tol::witness().get().k);
}

#[test]
fn env_k_reaches_band_scaling() {
    let exe = std::env::current_exe().unwrap();
    // Self re-exec: name the probe by MODULE PATH, not by bare fn name.
    // `tests/all.rs` aggregates every suite into one binary, so libtest sees
    // this probe as `<this_module>::print_band_ratio`. Stripping the leading crate name off
    // `module_path!()` yields the right filter in the aggregated layout AND in
    // a standalone one (where `module_path!()` has no `::` at all).
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::print_band_ratio"),
        None => "print_band_ratio".to_string(),
    };
    let out = std::process::Command::new(&exe)
        .args([probe.as_str(), "--ignored", "--exact", "--nocapture"])
        .env("CAD_AMBIGUITY_K", "25")
        .output()
        .unwrap();
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains("KPROBE ratio=25") && text.contains("KPROBE k=25"),
        "expected K = 25 to reach the band; got:\n{text}"
    );
}
