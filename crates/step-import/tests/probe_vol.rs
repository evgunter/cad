//! REVIEW PROBE (B5b): measure the ACTUAL relative volume agreement
//! against the independently re-run oracle numbers.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use step_import::{ImportOptions, StepImport, import_step};
use geom_core::Tol;

/// (fixture, oracle volume mm3) — from MY OWN freecadcmd re-run on the
/// UPSTREAM candidates/ originals, not from the committed sidecars.
const REORACLE: [(&str, f64); 9] = [
    ("adafruit/328_2500mAh_battery.step", 22986.629999999997),
    ("adafruit/1982_MPR121.step", 987.4217718980681),
    ("adafruit/805_slide_switch.step", 279.23999999999995),
    ("adafruit/931_OLED_128x32_I2C.step", 1401.7572671999997),
    ("adafruit/64_Halfsize_Breadboard.step", 44986.25),
    ("nist/nist_ftc_09_asme1_rd.stp", 136445.10482466163),
    ("stepcode/sg1-c5-214.stp", 355877.88282913784),
    // The two band-class fixtures (imports-class since M7-5's seam
    // re-mint; freecadcmd re-run 2026-08-07 on the upstream
    // candidates/ originals). "Imported but unmeasurable" —
    // RingOnCurvedFace — is exactly the state the old band refusal
    // existed to keep out, so these two rows are the re-mint's
    // imported⇒measurable obligation, executed.
    ("nist/nist_ftc_11_asme1_rb.stp", 5129.38613642268),
    ("occ-oss/cq_red_cube_blue_cylinder.step", 1785.3981633974481),
];

#[test]
fn measured_relative_volume_agreement() {
    let eps = geom_core::Tol::witness().get().eps;
    let mut worst = 0.0f64;
    let mut compared = 0;
    for (name, oracle) in REORACLE {
        let path = format!("{}/tests/fixtures/wild/{name}", env!("CARGO_MANIFEST_DIR"));
        let text = std::fs::read_to_string(path).unwrap();
        // Outside the wild corpus's certifying window (`[1e-10, 1e-8]`,
        // tabulated in `wild.rs`) some of these refuse typed — at
        // ε = 1e-12 `nist_ftc_09` refuses at the D7 ladder, the
        // documented floor, and that is the CORRECT answer there. This
        // probe's claim is about agreement, so it narrows to the
        // fixtures that reached a volume and holds those to the same
        // budget: a real assertion at every ε, rather than either a
        // failure on a documented refusal or a green tick for nothing.
        let body = match import_step(&text, &ImportOptions::default()) {
            Ok(StepImport::Solid { body, .. }) => body,
            Ok(other) => panic!("{name}: a solid, got {other:?}"),
            Err(e) => {
                println!("{name}: refuses typed at ambient ε {eps:e}: {e}");
                continue;
            }
        };
        // A body that DID import must still be measurable. "Imported,
        // but declines to compute a volume" is precisely the state the
        // periodic-band refusal exists to keep out of this corpus.
        let v = topo::mass_properties(&body, Tol::witness())
            .unwrap_or_else(|e| panic!("{name}: imported but has no volume: {e:?}"))
            .volume
            * 1e9;
        let rel = (v - oracle).abs() / oracle.abs();
        worst = worst.max(rel);
        compared += 1;
        println!("{name}: kernel {v} vs re-run oracle {oracle} -> rel {rel:e}");
    }
    println!("WORST relative disagreement over {compared} fixtures: {worst:e}");
    assert!(compared > 0, "no fixture reached a volume at ε {eps:e}");
    assert!(worst <= 1e-11, "claimed budget 1e-11 relative");
}
