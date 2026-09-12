//! R2 import-door e2e: drive a STEP file whose walls are RATIONAL
//! cylinder surfaces through the public import door and report what a
//! consumer sees (verdict, carried width, wall time).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use std::path::PathBuf;
use step_import::{ImportOptions, import_step};

fn fixture(rel: &str) -> String {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "fixtures", rel]
        .iter()
        .collect();
    std::fs::read_to_string(&path).expect("fixture reads")
}

#[test]
fn r2_import_door_rational_wall() {
    let text = fixture("wild/stepcode/dm1-id-214.stp");
    let t0 = std::time::Instant::now();
    let out = import_step(&text, &ImportOptions::default(), Tol::witness());
    let secs = t0.elapsed().as_secs_f64();
    match out {
        Ok(_) => println!("R2 IMPORT dm1: Ok (FIRST-CLASS) in {secs:.2}s"),
        Err(e) => println!("R2 IMPORT dm1: REFUSED in {secs:.2}s\n{e}"),
    }
}
