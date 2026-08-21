//! REVIEW PROBE (B6): flip an EDGE_CURVE same_sense WITHOUT flipping
//! its ORIENTED_EDGE uses — the file becomes self-inconsistent, and
//! the outcome must be honest: typed refusal or correct import, never
//! silent misgeometry.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;
use common::census;
use step_import::{ImportOptions, StepImport, import_step};
use geom_core::Tol;

fn fixture(rel: &str) -> String {
    let path = format!("{}/tests/fixtures/{rel}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(path).unwrap()
}

fn probe(tag: &str, text: &str, base_vol: f64) {
    match import_step(text, &ImportOptions::default()) {
        Ok(StepImport::Solid { body, .. }) => {
            let t1 = topo::validate(&body);
            let t2 = topo::validate_closed(&body);
            let t3 = topo::validate_geometric(&body, Tol::witness());
            let v = topo::mass_properties(&body, Tol::witness()).map(|m| m.volume);
            println!(
                "{tag}: IMPORTED census={:?} t1={t1:?} t2={t2:?} t3ok={} vol={v:?}",
                census(&body),
                t3.is_ok()
            );
            // Silent misgeometry = imports, all tiers green, volume moved.
            if t1.is_ok() && t2.is_ok() && t3.is_ok() {
                let v = v.unwrap();
                assert!(
                    (v - base_vol).abs() <= 1e-9 * base_vol.abs(),
                    "{tag}: SILENT MISGEOMETRY — tiers green but volume moved {base_vol} -> {v}"
                );
            }
        }
        Ok(StepImport::Wireframe { .. }) => println!("{tag}: wireframe?"),
        Err(e) => println!("{tag}: REFUSES TYPED: {e}"),
    }
}

#[test]
fn flipped_sense_is_never_silent_misgeometry() {
    let base = fixture("freecad/box.step");
    let StepImport::Solid { body, .. } = import_step(&base, &ImportOptions::default()).unwrap()
    else {
        panic!()
    };
    let v0 = topo::mass_properties(&body, Tol::witness()).unwrap().volume;

    // Box: flip one EDGE_CURVE sense only.
    let m = base.replace(
        "#21 = EDGE_CURVE('',#22,#24,#26,.T.);",
        "#21 = EDGE_CURVE('',#22,#24,#26,.F.);",
    );
    assert_ne!(m, base);
    probe("box sense-flip", &m, v0);

    // sg1: flip one wild .F. back to .T. (inconsistent with its uses).
    let sg1 = fixture("wild/stepcode/sg1-c5-214.stp");
    let StepImport::Solid { body, .. } = import_step(&sg1, &ImportOptions::default()).unwrap()
    else {
        panic!()
    };
    let vs = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    let m2 = sg1.replace(
        "#46=EDGE_CURVE('',#43,#45,#41,.F.) ;",
        "#46=EDGE_CURVE('',#43,#45,#41,.T.) ;",
    );
    assert_ne!(m2, sg1);
    probe("sg1 sense-flip", &m2, vs);
}
