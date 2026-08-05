//! REVIEW PROBE (B5b): measure the ACTUAL relative volume agreement
//! against the independently re-run oracle numbers.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use step_import::{ImportOptions, StepImport, import_step};

/// (fixture, oracle volume mm3) — from MY OWN freecadcmd re-run on the
/// UPSTREAM candidates/ originals, not from the committed sidecars.
const REORACLE: [(&str, f64); 7] = [
    ("adafruit/328_2500mAh_battery.step", 22986.629999999997),
    ("adafruit/1982_MPR121.step", 987.4217718980681),
    ("adafruit/805_slide_switch.step", 279.23999999999995),
    ("adafruit/931_OLED_128x32_I2C.step", 1401.7572671999997),
    ("adafruit/64_Halfsize_Breadboard.step", 44986.25),
    ("nist/nist_ftc_09_asme1_rd.stp", 136445.10482466163),
    ("stepcode/sg1-c5-214.stp", 355877.88282913784),
];

#[test]
fn measured_relative_volume_agreement() {
    let mut worst = 0.0f64;
    for (name, oracle) in REORACLE {
        let path = format!("{}/tests/fixtures/wild/{name}", env!("CARGO_MANIFEST_DIR"));
        let text = std::fs::read_to_string(path).unwrap();
        let StepImport::Solid { body, .. } = import_step(&text, &ImportOptions::default()).unwrap()
        else {
            panic!("{name}: a solid")
        };
        let v = topo::mass_properties(&body).unwrap().volume * 1e9;
        let rel = (v - oracle).abs() / oracle.abs();
        worst = worst.max(rel);
        println!("{name}: kernel {v} vs re-run oracle {oracle} -> rel {rel:e}");
    }
    println!("WORST relative disagreement: {worst:e}");
    assert!(worst <= 1e-11, "claimed budget 1e-11 relative");
}
