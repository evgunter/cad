//! End-to-end check that the run-configured ambiguity multiplier K
//! ([`geom_core::tolerance::ENV_K`], M2 PR 7) reaches [`Band::linear`]:
//! re-exec this binary with the env var set (the OnceLock commits per
//! process, so the override needs its own process — the same pattern
//! as the multi-ε suites).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Tolerance};

/// Printer probe for the re-exec (ignored in normal runs).
#[test]
#[ignore]
fn print_band_ratio() {
    let band = Band::linear().unwrap();
    println!("KPROBE ratio={}", band.escalate() / band.zero());
    println!("KPROBE k={}", Tolerance::get().k);
}

#[test]
fn env_k_reaches_band_scaling() {
    let exe = std::env::current_exe().unwrap();
    let out = std::process::Command::new(&exe)
        .args(["print_band_ratio", "--ignored", "--exact", "--nocapture"])
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
