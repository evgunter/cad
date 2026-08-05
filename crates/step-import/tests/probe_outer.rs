#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use step_import::{ImportOptions, StepImport, import_step};
#[test]
fn two_outer_bounds_on_a_planar_face_with_a_real_hole() {
    let p = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/freecad/box_hole.step"
    );
    let base = std::fs::read_to_string(p).unwrap();
    let StepImport::Solid { body, .. } = import_step(&base, &ImportOptions::default()).unwrap()
    else {
        panic!()
    };
    let v0 = topo::mass_properties(&body).unwrap().volume;
    println!("baseline volume {v0}");
    // Mark EVERY bound outer: the hole now claims to be an outer bound.
    let m = base.replace("FACE_BOUND(", "FACE_OUTER_BOUND(");
    assert_ne!(m, base);
    match import_step(&m, &ImportOptions::default()) {
        Ok(StepImport::Solid { body, .. }) => {
            let t3 = topo::validate_geometric(&body);
            let v = topo::mass_properties(&body).map(|x| x.volume);
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
