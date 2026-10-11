//! Reviewer probe for PR 4746: a ball (R = 10 mm, poles on ±z) with the
//! cap x > 8 cut away, its sphere split at the equator into two faces on
//! ONE spherical surface. Each face holds a chart pole in its interior
//! (the C12 (7) shape validity does not refuse yet, as the rim-only cap
//! fixtures do), and neither pole lies on any loop.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

#[test]
fn review_4746_side_cut_equator_split_imports() {
    let tol = Tol::witness();
    let p = format!(
        "{}/tests/fixtures/review-4746/side_cut_equator_split.step",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&p).unwrap();
    let r = 0.010_f64;
    let h = 0.002_f64;
    let exact = core::f64::consts::PI * (4.0 * r * r * r - h * h * (3.0 * r - h)) / 3.0;
    match import_step(&text, &ImportOptions::default(), tol) {
        Ok(StepImport::Solid { body, .. }) => {
            eprintln!(
                "IMPORTED: tier3 {:?} volume {:?} exact {exact}",
                topo::validate_geometric(&body, tol),
                topo::mass_properties(&body, tol).map(|m| m.volume)
            );
        }
        Ok(_) => eprintln!("IMPORTED: not a solid"),
        Err(e) => eprintln!("REFUSED: {e:?}"),
    }
}
