//! REVIEW PROBE (B7b): does deviation 3(a)'s multi-`FACE_OUTER_BOUND`
//! latitude eat a REAL hole?
//!
//! The importer now lets a face state several bounds outer and breaks
//! the tie by geometry — and, on a closed periodic band where the
//! geometry genuinely cannot, by the file's bound order. The attack:
//! mark EVERY bound outer on `box_hole.step`, whose top face carries an
//! honest planar ring. If the latitude were too wide the hole would be
//! read as a second outer boundary and the volume would jump. It does
//! not: the inference answers, and the volume is unchanged to the bit.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

#[test]
fn two_outer_bounds_on_a_planar_face_with_a_real_hole() {
    let p = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/freecad/box_hole.step"
    );
    let base = std::fs::read_to_string(p).unwrap();
    // `box_hole` is one of the four cylindrical-chart FreeCAD fixtures
    // that refuse typed above the M7-2 corpus ε ceiling, so above it
    // there is no baseline to compare against and the probe's question
    // is unanswerable. What IS answerable at any ε — and is the half
    // that would actually be dangerous — is that marking every bound
    // outer never turns a file the kernel refuses into one it accepts.
    let baseline = import_step(&base, &ImportOptions::default());
    // Mark EVERY bound outer: the hole now claims to be an outer bound.
    let m = base.replace("FACE_BOUND(", "FACE_OUTER_BOUND(");
    assert_ne!(m, base);
    let Ok(StepImport::Solid { body, .. }) = baseline else {
        let eps = geom_core::Tol::witness().get().eps;
        println!(
            "baseline refuses at ambient ε {eps:e}: {:?}",
            baseline.err()
        );
        assert!(
            import_step(&m, &ImportOptions::default()).is_err(),
            "the all-outer latitude must not import a file the honest one refuses"
        );
        return;
    };
    let v0 = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    println!("baseline volume {v0}");
    match import_step(&m, &ImportOptions::default()) {
        Ok(StepImport::Solid { body, .. }) => {
            let t3 = topo::validate_geometric(&body, Tol::witness());
            let v = topo::mass_properties(&body, Tol::witness()).map(|x| x.volume);
            println!("all-outer: t3ok={} vol={v:?}", t3.is_ok());
            if t3.is_ok() {
                let v = v.unwrap();
                assert!(
                    (v - v0).abs() <= 1e-9 * v0.abs(),
                    "SILENT MISGEOMETRY {v0} -> {v}"
                );
            }
        }
        Ok(o) => panic!("{o:?}"),
        Err(e) => println!("all-outer REFUSES TYPED: {e}"),
    }
}
