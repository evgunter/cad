//! R1 review probe (MESH-3, PR 1460): reproduce the en-route finding
//! that `halfcap_eps7.step` imports Pass but panics `closing_column`'s
//! S22 detector in `mesh::tessellate` at every ambient band. Review
//! probe only; not part of the unit.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

#[test]
fn r1_probe_halfcap_eps7_panics_the_s22_detector() {
    let path = format!(
        "{}/tests/fixtures/halfcap/halfcap_eps7.step",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&path).unwrap();
    let eps = Tol::witness().get().eps;
    let Ok(StepImport::Solid { body, .. }) =
        import_step(&text, &ImportOptions::default(), Tol::witness())
    else {
        panic!("halfcap_eps7 no longer imports at eps {eps:e}");
    };
    println!("halfcap_eps7 imported Pass at ambient eps {eps:e}");
    for delta in [0.002f64, 0.0005, 0.0001, 0.00002] {
    let b2 = body.clone();
    let out = std::panic::catch_unwind(move || mesh::tessellate(&b2, delta, Tol::witness()));
    print!("  delta {delta:e}: ");
    match out {
        Ok(Ok(m)) => println!("tessellated OK ({} patches), check_mesh: {:?}", m.patches.len(), mesh::validate::check_mesh(&m).is_ok()),
        Ok(Err(e)) => println!("typed refusal: {e:?}"),
        Err(p) => {
            let msg = p
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            println!("PANICKED at: {}", &msg[..msg.len().min(80)]);
        }
    }
    }
}
