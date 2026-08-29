//! R2 E2E: near-polar half-cap twins through the STEP import door.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

#[test]
fn r2_e2e_near_polar_halfcaps() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../r2work/steps");
    let mut dirs: Vec<_> = std::fs::read_dir(root)
        .expect("r2work/steps")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();
    dirs.sort();
    for d in dirs {
        let label = d.file_name().unwrap().to_string_lossy().to_string();
        // exact volume: recompute from the generated gen.py constants
        let genpy = std::fs::read_to_string(d.join("gen.py")).unwrap();
        let b: f64 = genpy
            .lines()
            .find(|l| l.starts_with("B = "))
            .unwrap()
            .split_whitespace()
            .nth(2)
            .unwrap()
            .parse()
            .unwrap();
        let rr = 10.0f64;
        let h = rr * (1.0 - b.sin());
        let exact = core::f64::consts::PI * h * h * (3.0 * rr - h) / 6.0 * 1e-9;
        for twin in ["halfcap.step", "halfcap_nosplit.step"] {
            let txt = std::fs::read_to_string(d.join(twin)).unwrap();
            let line = match import_step(&txt, &ImportOptions::default(), Tol::witness()) {
                Err(e) => format!("IMPORT REFUSED {e:?}"),
                Ok(StepImport::Wireframe { .. }) => "NOT A SOLID (wireframe)".to_string(),
                Ok(StepImport::Solid { body, .. }) => match topo::validate_geometric(&body, Tol::witness()) {
                    Err(e) => format!("tier3 REFUSED {e:?}"),
                    Ok(()) => match topo::mass_properties(&body, Tol::witness()) {
                        Err(e) => format!("mass_props REFUSED {e:?}"),
                        Ok(mp) => format!(
                            "CERTIFIED vol={:.9e} pad={:e} exact={exact:.9e} rel={:+.4e}",
                            mp.volume,
                            mp.volume_pad,
                            (mp.volume - exact) / exact
                        ),
                    },
                },
            };
            println!("  {label:<24} {:<20} {line}", twin.trim_end_matches(".step"));
        }
    }
}
