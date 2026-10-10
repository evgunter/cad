use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};
#[test]
fn review_4573_cert_rel() {
    for (name, r, b) in [
        ("nearpolar_split.step", 0.010, 1.55f64),
        ("nearpolar_nosplit.step", 0.010, 1.55),
        ("polesplit_split.step", 0.010, 0.5),
        ("polesplit_nosplit.step", 0.010, 0.5),
    ] {
        let path = format!(
            "{}/tests/fixtures/cert1-r1/{name}",
            env!("CARGO_MANIFEST_DIR")
        );
        let Ok(StepImport::Solid { body, .. }) = import_step(
            &std::fs::read_to_string(&path).unwrap(),
            &ImportOptions::default(),
            Tol::witness(),
        ) else {
            panic!()
        };
        let mp = topo::mass_properties(&body, Tol::witness()).unwrap();
        let h = r * (1.0 - b.sin());
        let exact = core::f64::consts::PI * h * h * (3.0 * r - h) / 6.0;
        let radial = r * core::f64::consts::PI * r * r * (1.0 - b.sin());
        eprintln!(
            "{name}: rel-to-volume {:.3e} rel-to-radial {:.3e}",
            (mp.volume - exact).abs() / exact,
            (mp.volume - exact).abs() / radial
        );
    }
}
