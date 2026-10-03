//! The demo K sweep (`demo-tour k-probe`) at each tolerance row: it
//! exits 0, and the bodies whose volume it reads as a BRACKET are
//! exactly the ones listed per ε below.
//!
//! A body comes back as a bracket when its gate certified the sign and
//! its quadrature schedule cannot reach the reporting target
//! `1024·ε`; the lane says so after round 0 (`props::quad`'s
//! `last_round_refuses`), and that decision is metered as a
//! `props_quad_last_round` sample, so a bracket shows in the CSV as a
//! `negative` row of that predicate. The lily's swept leaves are
//! rational walls along a cubic spine whose last-round Taylor bound is
//! about 1.5e-8 m of mean displacement, so they certify at 1e-6 and
//! 1e-9 and bracket at 1e-12.
//!
//! One process per ε (`Tolerance` is a OnceLock), so each row spawns
//! the binary, as `eps_regression.rs` does.
#![cfg(feature = "probe")]

use std::collections::BTreeSet;
use std::process::Command;

/// Runs the sweep at `eps` and returns the labels it bracketed and the
/// outcomes of `demo/lily`'s `props_quad_last_round` samples.
fn sweep(eps: &str) -> (BTreeSet<String>, Vec<String>) {
    let csv = std::env::temp_dir().join(format!(
        "demo-tour-k-probe-brackets-{eps}-{}.csv",
        std::process::id()
    ));
    let out = Command::new(env!("CARGO_BIN_EXE_demo-tour"))
        .arg("k-probe")
        .arg(&csv)
        .env("CAD_TOLERANCE_EPS", eps)
        .output()
        .expect("spawn demo-tour");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "k-probe at eps {eps} exited {:?}; stderr tail:\n{}",
        out.status.code(),
        stderr
            .lines()
            .rev()
            .take(20)
            .collect::<Vec<_>>()
            .join("\n")
    );
    let bracketed = stderr
        .lines()
        .filter(|l| l.contains("a bracket"))
        .map(|l| {
            l.split(": ")
                .nth(2)
                .unwrap_or_else(|| panic!("unparsed bracket line: {l}"))
                .to_string()
        })
        .collect();
    let rows = std::fs::read_to_string(&csv).expect("read the k-probe csv");
    let _ = std::fs::remove_file(&csv);
    let last_round = rows
        .lines()
        .filter(|r| r.starts_with("demo/lily,props_quad_last_round,"))
        .map(|r| r.rsplit(',').next().unwrap_or_default().to_string())
        .collect();
    (bracketed, last_round)
}

fn leaves() -> BTreeSet<String> {
    ["lily_leaf_b", "lily_leaf_c"]
        .into_iter()
        .map(String::from)
        .collect()
}

#[test]
fn k_probe_brackets_the_swept_leaves_at_eps_1e_12() {
    let (bracketed, last_round) = sweep("1e-12");
    assert_eq!(bracketed, leaves(), "bracketed bodies at 1e-12");
    assert!(
        !last_round.is_empty() && last_round.iter().all(|o| o == "negative"),
        "the lily's last-round samples at 1e-12: {last_round:?}"
    );
}

#[test]
fn k_probe_measures_every_body_at_eps_1e_9() {
    let (bracketed, last_round) = sweep("1e-9");
    assert_eq!(bracketed, BTreeSet::new(), "bracketed bodies at 1e-9");
    assert!(
        last_round.iter().all(|o| o != "negative"),
        "the lily's last-round samples at 1e-9: {last_round:?}"
    );
}

#[test]
fn k_probe_measures_every_body_at_eps_1e_6() {
    let (bracketed, _) = sweep("1e-6");
    assert_eq!(bracketed, BTreeSet::new(), "bracketed bodies at 1e-6");
}
