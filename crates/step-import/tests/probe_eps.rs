//! REVIEW PROBE (B2): is the [1e-10, 1e-8] floor a TEST window or a
//! silent widening of the import gate? Discriminating experiment: hand
//! the importer declared uncertainties far below the floor and check
//! whether eps_in comes back RAISED.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn box_step() -> String {
    std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/freecad/box.step"
    ))
    .unwrap()
}

#[test]
fn declared_uncertainty_is_never_raised_to_a_floor() {
    let base = box_step();
    // box.step declares 1.E-07 in millimetres -> 1e-10 m.
    for (declared, want_m) in [
        ("1.E-07", 1e-10f64),
        ("1.E-09", 1e-12),
        ("1.E-12", 1e-15),
        ("1.E-20", 1e-23),
        ("1.E-03", 1e-6),
    ] {
        let m = base.replace("1.E-07", declared);
        assert!(m.contains(declared));
        match import_step(&m, &ImportOptions::default(), Tol::witness()) {
            Ok(StepImport::Solid { eps_in, .. }) => {
                println!("declared {declared} mm -> eps_in {eps_in:e} m (want {want_m:e})");
                assert!(
                    (eps_in - want_m).abs() <= 1e-12 * want_m.max(1e-30),
                    "eps_in was ALTERED: {eps_in:e} vs declared {want_m:e}"
                );
            }
            Ok(other) => panic!("not a solid: {other:?}"),
            Err(e) => println!("declared {declared} mm -> REFUSES: {e}"),
        }
    }
    // And the explicit override, below and above the window.
    for over in [1e-15f64, 1e-3] {
        match import_step(
            &base,
            &ImportOptions {
                eps_in: Some(over),
                ..ImportOptions::default()
            },
            Tol::witness(),
        ) {
            Ok(i) => {
                println!("override {over:e} -> eps_in {:e}", i.eps_in());
                assert_eq!(i.eps_in(), over, "the override must be verbatim");
            }
            Err(e) => println!("override {over:e} -> REFUSES: {e}"),
        }
    }
}
