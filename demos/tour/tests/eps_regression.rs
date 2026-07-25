//! Regression pin for #99: the full tour must run green at every
//! supported tolerance row — a panic is always a bug; the only legal
//! outcomes are a working run or a typed refusal (which the tour
//! surfaces as a clean nonzero exit, not an abort).
//!
//! The escalation that motivated this pin: the bracket fillet's via
//! point was decimally rounded (1.146 vs the exact 1.5 − 0.5/√2 =
//! 1.1464466…), leaving the arc carrier ~2.3e-6 clear of the adjacent
//! line carriers instead of tangent — inside the carrier_line_circle
//! escalation band at CAD_TOLERANCE_EPS=1e-6, so profile validation
//! (correctly) refused the near-tangency and the demo's `.expect`
//! panicked. The data now encodes exact tangency; these tests keep the
//! whole tour honest at 1e-6 and 1e-12 alongside the default.

use std::process::Command;

fn run_tour(eps: Option<&str>) {
    let outdir = std::env::temp_dir().join(format!(
        "demo-tour-eps-pin-{}-{}",
        eps.unwrap_or("default"),
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&outdir);
    std::fs::create_dir_all(&outdir).expect("create outdir");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_demo-tour"));
    cmd.arg(&outdir);
    match eps {
        Some(e) => {
            cmd.env("CAD_TOLERANCE_EPS", e);
        }
        None => {
            cmd.env_remove("CAD_TOLERANCE_EPS");
        }
    }
    let output = cmd.output().expect("spawn demo-tour");
    let _ = std::fs::remove_dir_all(&outdir);
    assert!(
        output.status.success(),
        "tour failed at eps {:?} ({}):\n--- stdout tail ---\n{}\n--- stderr ---\n{}",
        eps,
        output.status,
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .rev()
            .take(5)
            .collect::<Vec<_>>()
            .join("\n"),
        String::from_utf8_lossy(&output.stderr),
    );
}

#[test]
fn tour_runs_green_at_default_eps() {
    run_tour(None);
}

#[test]
fn tour_runs_green_at_eps_1e_6() {
    run_tour(Some("1e-6"));
}

#[test]
fn tour_runs_green_at_eps_1e_12() {
    run_tour(Some("1e-12"));
}
