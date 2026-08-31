//! CERT-5 review lane R1 import-door probes (blinded adversarial
//! review of PR 1314, frozen head 3fc450d6).
//!
//! **Adopted into the unit by merge, authorship kept.** The dm1 row
//! was written to fail against the PR's claimed residual; its window
//! now carries the re-measured figure, and the note on that row says
//! what the discrepancy was.
//!
//! Two rows: (1) dm1 imported by the reviewer — the PR's residual
//! claim (a refusal with no floor under it) and the
//! wall-time claim, re-measured rather than believed; (2) the
//! reviewer's OWN rational wall (a 60-degree-arc loft at six
//! stations, off-grid interior v knots) round-tripped through STEP —
//! what an import CONSUMER sees post-fix on a wall the unit's
//! fixtures never shaped.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Affine3, Point2, Vec3};
use profile::RawLoop;
use profile::{ProfileLoop, ProfileVertex};
use std::path::PathBuf;
use step_import::{ImportOptions, StepImportError, import_step};

#[test]
fn dm1_residual_and_wall_time_remeasured() {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        "wild",
        "stepcode",
        "dm1-id-214.stp",
    ]
    .iter()
    .collect();
    let text = std::fs::read_to_string(&path).unwrap();
    let t0 = std::time::Instant::now();
    let out = import_step(&text, &ImportOptions::default(), Tol::witness());
    let dt = t0.elapsed();
    match out {
        Err(StepImportError::TierInvalid { solid, errors }) => {
            eprintln!("CERT5-R1 dm1: TierInvalid solid {solid:?} in {dt:?}: {errors:?}");
            let text = format!("{errors:?}");
            assert!(
                text.contains("QuadratureBudget"),
                "dm1's refusal must still be the quadrature budget: {text}"
            );
            // Extract the width from the debug text.
            let w = text
                .split("width_len:")
                .nth(1)
                .and_then(|s| s.trim().split([',', ' ']).next())
                .and_then(|s| s.parse::<f64>().ok())
                .expect("the refusal must carry a width");
            eprintln!("CERT5-R1 dm1: width_len {w:e}");
            // **The re-measured residual.** The PR originally claimed
            // 1.5435e-6. That figure was taken mid-development, after
            // the hull blocks were knot-aligned but BEFORE the shared
            // area rule was — and the meter is
            // `flux.width() / (3·area_mid)`, so it moved when the
            // DENOMINATOR did. The flux enclosure is bit-identical
            // across that change (1.960408001025648e-9); what changed
            // is that the area stopped being inflated by the hull rule
            // its straddling cells used to take, and then tightened
            // again when the rule began intersecting both bounds.
            //
            // The window is wide because this row's claim is the
            // DISPOSITION — dm1 still refuses, and nowhere near the
            // 2.7e-4 floor it used to sit on — not the digit. The digit
            // lives in the PR description, where it can be argued.
            assert!(
                (1.5e-6..2.5e-6).contains(&w),
                "dm1 must still refuse, at a width that is the schedule running \
                 out rather than the retired 2.7e-4 floor: {w:e}"
            );
        }
        other => panic!("dm1 must still refuse at the at-rest gate, got {other:?}"),
    }
}

#[test]
fn own_rational_wall_roundtrips_through_the_import_door() {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x, y), bulge);
    let section = vec![ProfileLoop::new(vec![
        v(-1.0, -1.0, 0.0),
        v(1.0, -1.0, 0.267_949_192_431_122_7),
        v(1.0, 1.0, 0.0),
        v(-1.0, 1.0, 0.0),
    ])];
    let n = 6usize;
    let sections: Vec<_> = (0..n).map(|_| section.clone()).collect();
    #[allow(clippy::cast_precision_loss)]
    let places: Vec<Affine3<f64>> = (0..n)
        .map(|k| Affine3::translation(Vec3::new(0.0, 0.0, 2.0 * k as f64 / (n - 1) as f64)))
        .collect();
    let lofted = sweep::loft_body::<f64>(&sections, &places, 2, Tol::witness()).expect("lofts");
    let native = topo::mass_properties(&lofted.body, Tol::witness())
        .expect("the native balloon certifies (the e2e probe pins this separately)");
    let text = step_export::step_string(
        &lofted.body,
        &step_export::StepOptions::default(),
        Tol::witness(),
    )
    .expect("the arc-bearing loft exports");
    let t0 = std::time::Instant::now();
    let imported = import_step(&text, &ImportOptions::default(), Tol::witness());
    let dt = t0.elapsed();
    match imported {
        Ok(step_import::StepImport::Solid { body, .. }) => {
            let m = topo::mass_properties(&body, Tol::witness())
                .expect("the imported twin certifies too");
            eprintln!(
                "CERT5-R1 roundtrip: import+gate in {dt:?}; native {} +- {}, imported {} +- {}",
                native.volume, native.volume_pad, m.volume, m.volume_pad
            );
            // The two enclosures must overlap: same solid.
            assert!(
                (m.volume - native.volume).abs() <= m.volume_pad + native.volume_pad,
                "round-trip volume disagrees: native {} +- {}, imported {} +- {}",
                native.volume,
                native.volume_pad,
                m.volume,
                m.volume_pad
            );
        }
        other => panic!(
            "the reviewer's rational wall must import first-class post-fix \
             (off-grid knots in both directions through the import door): {other:?}"
        ),
    }
}
