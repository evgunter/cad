#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};
#[test]
fn print_import_errors() {
    for (name, r, b) in [("nearpolar_split.step", 0.010, 1.55), ("nearpolar_nosplit.step", 0.010, 1.55), ("polesplit_split.step", 0.010, 0.5)] {
        let path = format!("{}/tests/fixtures/cert1-r1/{name}", env!("CARGO_MANIFEST_DIR"));
        let s = std::fs::read_to_string(&path).unwrap();
        let Ok(StepImport::Solid { body, .. }) = import_step(&s, &ImportOptions::default(), Tol::witness()) else { panic!() };
        let mp = topo::mass_properties(&body, Tol::witness()).unwrap();
        let h = r * (1.0 - f64::sin(b));
        let exact = core::f64::consts::PI * h * h * (3.0 * r - h) / 6.0;
        let ea = core::f64::consts::PI * r * h + 0.5 * core::f64::consts::PI * (2.0 * r * h - h * h) + r * r * f64::cos(b) * 0.0;
        println!("{name}: vol rel {:+.3e} area {:.17e} (curved+half-disk {:.17e}, excludes slab)", (mp.volume - exact) / exact, mp.surface_area, ea);
    }
}
