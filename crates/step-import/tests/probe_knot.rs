//! REVIEW PROBE (B8b): a REACHABLE hostile knot multiplicity.
//! `entities.rs::knot_vector` run-length-expands the file's stated
//! multiplicity with no bound. Is that reachable, and what happens?
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::time::{Duration, Instant};
use step_import::{ImportOptions, import_step};

fn box_step() -> String {
    std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/freecad/box.step"
    ))
    .unwrap()
}

fn run(tag: &str, text: &str) {
    let t = Instant::now();
    let owned = text.to_owned();
    let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        import_step(&owned, &ImportOptions::default()).map(|i| i.eps_in())
    }));
    let took = t.elapsed();
    match &out {
        Ok(Ok(_)) => println!("{tag}: imported in {took:?}"),
        Ok(Err(e)) => println!(
            "{tag}: refused in {took:?}: {}",
            &e.to_string()[..120.min(e.to_string().len())]
        ),
        Err(_) => println!("{tag}: PANICKED in {took:?}"),
    }
    assert!(out.is_ok(), "{tag}: PANICKED");
    assert!(took < Duration::from_secs(30), "{tag}: HUNG ({took:?})");
}

/// Replace the box's edge #21 carrier with a B-spline curve of the
/// given multiplicity list — a curve type inside the subset.
fn with_bspline(mults: &str, knots: &str, degree: usize) -> String {
    box_step().replace(
        "#26 = LINE('',#27,#28);",
        &format!(
            "#26 = B_SPLINE_CURVE_WITH_KNOTS('',{degree},(#23,#25),\
             .UNSPECIFIED.,.F.,.F.,({mults}),({knots}),.UNSPECIFIED.);"
        ),
    )
}

// IGNORED BY DEFAULT: the hostile multiplicities ABORT the process
// (16 GB allocation failure -> SIGABRT), which is the finding. Run
// with `--ignored` to reproduce.
#[test]
#[ignore = "aborts the test process: unbounded knot allocation (finding MINOR-1)"]
fn hostile_knot_multiplicities() {
    // Sanity: a well-formed degree-1 spline over the two vertices.
    run("well-formed bspline", &with_bspline("2,2", "0.,1.", 1));
    // Hostile: multiplicities that expand to ~4 billion knots.
    run(
        "mult 2e9 each",
        &with_bspline("2000000000,2000000000", "0.,1.", 1),
    );
    // Hostile: one huge multiplicity.
    run("mult 1e9", &with_bspline("1000000000,2", "0.,1.", 1));
    // Hostile: usize::MAX-ish.
    run(
        "mult 1e18",
        &with_bspline("1000000000000000000,2", "0.,1.", 1),
    );
}
