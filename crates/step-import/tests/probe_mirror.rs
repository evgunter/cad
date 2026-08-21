//! REVIEW PROBE (B4): try to smuggle a mirror through placement pairs;
//! verify a genuine rotation round-trips rigidly.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;
use common::census;
use step_import::{ImportOptions, StepImport, StepImportError, import_step};
use geom_core::Tol;

fn twobody() -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/freecad/twobody_importexport.step"
    );
    std::fs::read_to_string(path).unwrap()
}

fn solid(text: &str) -> topo::Body<f64> {
    match import_step(text, &ImportOptions::default()) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("expected a solid, got {other:?}"),
    }
}

fn vol(b: &topo::Body<f64>) -> f64 {
    topo::mass_properties(b, Tol::witness()).unwrap().volume
}

fn bbox(b: &topo::Body<f64>) -> [f64; 6] {
    let mut m = [
        f64::INFINITY,
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
    ];
    for (_, p) in b.points() {
        m[0] = m[0].min(p.x);
        m[1] = m[1].min(p.y);
        m[2] = m[2].min(p.z);
        m[3] = m[3].max(p.x);
        m[4] = m[4].max(p.y);
        m[5] = m[5].max(p.z);
    }
    m
}

/// Rewrite BOTH component target placements (#15, #19) to the given
/// axis/ref direction entities and origin, so the transform covers all
/// content.
fn place_both(axis: &str, refd: &str, origin: &str) -> String {
    twobody()
        .replace(
            "#15 = AXIS2_PLACEMENT_3D('',#16,#17,#18);",
            &format!(
                "#15 = AXIS2_PLACEMENT_3D('',#9990,#9991,#9992);\n\
                 #9990 = CARTESIAN_POINT('',({origin}));\n\
                 #9991 = DIRECTION('',({axis}));\n\
                 #9992 = DIRECTION('',({refd}));"
            ),
        )
        .replace(
            "#19 = AXIS2_PLACEMENT_3D('',#20,#21,#22);",
            &format!(
                "#19 = AXIS2_PLACEMENT_3D('',#9993,#9994,#9995);\n\
                 #9993 = CARTESIAN_POINT('',({origin}));\n\
                 #9994 = DIRECTION('',({axis}));\n\
                 #9995 = DIRECTION('',({refd}));"
            ),
        )
}

#[test]
fn mirror_smuggling_and_rigid_roundtrip() {
    let base = solid(&twobody());
    let (v0, c0, b0) = (vol(&base), census(&base), bbox(&base));
    println!("baseline: vol={v0} census={c0:?} bbox={b0:?}");

    // (1) Genuine rigid motion: rotate 90 deg about z (ref +x -> +y)
    // and translate. Volume and census must be invariant; bbox rotated.
    let rot = place_both("0.,0.,1.", "0.,1.,0.", "5.,0.,0.");
    let r = solid(&rot);
    println!(
        "rot90+t: vol={} census={:?} bbox={:?}",
        vol(&r),
        census(&r),
        bbox(&r)
    );
    assert_eq!(c0, census(&r), "rigid: census invariant");
    assert!(
        (v0 - vol(&r)).abs() <= 1e-9 * v0.abs(),
        "rigid: volume invariant"
    );
    assert!(bbox(&r) != b0, "the body genuinely moved");

    // (2) Smuggle attempt: negated axis. (0,0,-1) with ref +x is a
    // half-turn about x-ish — still right-handed, det +1. Must import
    // as a RIGID motion (volume invariant), never a mirror/skew.
    let neg = place_both("0.,0.,-1.", "1.,0.,0.", "0.,0.,0.");
    let n = solid(&neg);
    println!(
        "neg-axis: vol={} census={:?} bbox={:?}",
        vol(&n),
        census(&n),
        bbox(&n)
    );
    assert_eq!(c0, census(&n));
    assert!(
        (v0 - vol(&n)).abs() <= 1e-9 * v0.abs(),
        "neg axis is still rigid"
    );

    // (3) Smuggle attempt: axis and ref BOTH negated (a rotation by pi
    // about the projected ref? still det +1). Rigid or typed refusal.
    let dbl = place_both("0.,0.,-1.", "-1.,0.,0.", "0.,0.,0.");
    let d = solid(&dbl);
    println!("double-neg: vol={} census={:?}", vol(&d), census(&d));
    assert!(
        (v0 - vol(&d)).abs() <= 1e-9 * v0.abs(),
        "double-neg is still rigid"
    );

    // (4) Dependent-axis trick: ref parallel to axis states no frame.
    let par = place_both("0.,0.,1.", "0.,0.,1.", "5.,0.,0.");
    match import_step(&par, &ImportOptions::default()) {
        Err(StepImportError::Structure { what, .. })
        | Err(StepImportError::MalformedRecord { expected: what, .. }) => {
            println!("parallel pair refuses: {what}");
            assert!(what.contains("parallel"), "names the degeneracy: {what}");
        }
        other => panic!("a parallel axis/ref pair must refuse, got {other:?}"),
    }

    // (5) Sloppy directions beyond eps_in: printed to 3 digits. The
    // det check (or the direction window) must keep this typed, never
    // a silently skewed body. twobody's eps_in is 1e-7 mm -> tiny.
    let sloppy = place_both("0.577,0.577,0.577", "0.707,-0.707,0.", "1.,0.,0.");
    match import_step(&sloppy, &ImportOptions::default()) {
        Ok(StepImport::Solid { body, .. }) => {
            // If it imports, the composed map must have been rigid:
            // volume is the witness.
            let v = vol(&body);
            println!("sloppy dirs imported: vol={v}");
            assert!(
                (v0 - v).abs() <= 1e-9 * v0.abs(),
                "no silent skew: {v0} vs {v}"
            );
        }
        Err(e) => println!("sloppy dirs refuse typed: {e}"),
        other => panic!("unexpected: {other:?}"),
    }
}
