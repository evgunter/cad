//! Regression pin for #99: the full tour must run green at every
//! supported tolerance row. **Green means exit 0, and that is the
//! whole contract** — [`run_tour`] asserts nothing else, and nothing
//! else is available to assert.
//!
//! The tour has no clean-refusal exit. Every typed refusal it can meet
//! on the scene path is a panic by construction (`run_body` panics on
//! a failed validation tier, a failed tessellation, a failed STL write
//! and any STEP refusal at all); its only `std::process::exit(2)`
//! sites are the two feature-gated modes reporting a usage error, and
//! neither runs from here. So "a working run or a typed refusal" is
//! not a distinction this pin can draw, and asserting `success()`
//! loses nothing: a demo whose scenes are all inside the kernel's
//! shipped surface has no refusal that is not a regression.
//!
//! **That posture is RULED (#795, Ev), and `success()` is the
//! contract this pin keeps.** A demo may panic when the kernel does
//! something unexpected, and it may narrate a frontier it DECLARES up
//! front at the scene — a declared frontier still exits 0, so no
//! second exit convention is needed here and none was built. An
//! undeclared refusal still panics. The forward obligation rides with
//! whoever declares the first one: it must fail loudly when it stops
//! refusing. `work/code-quality/demo-typed-refusal-exit-convention.md`
//! holds the ruling.
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
        {
            let mut tail: Vec<_> = String::from_utf8_lossy(&output.stdout)
                .lines()
                .rev()
                .take(5)
                .map(str::to_owned)
                .collect();
            tail.reverse();
            tail.join("\n")
        },
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
