//! ADVERSARIAL REVIEW PROBES for M6-6 (PR #223). Review-branch only.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use profile::RawLoop;
use step_import::{ImportOptions, StepImport, import_step};
use geom_core::Tol;

fn fixture(name: &str, _ext: &str) -> String {
    std::fs::read_to_string(format!(
        "{}/tests/fixtures/freecad/{name}.step",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
}

fn import(text: &str) -> Result<StepImport, step_import::StepImportError> {
    import_step(text, &ImportOptions::default())
}

/// The freecad cylinder with its bottom rim SPLIT into two half-arcs:
/// the wall face then has THREE rim edges, outside `wall_inversion`'s
/// exactly-two-rims scope (deviation 3) — the rider must SKIP it.
fn cylinder_three_rim(wall_sense: &str) -> String {
    let text = fixture("cylinder", "step");
    // New vertex at the antipode of the seam vertex, on the bottom rim.
    let extra = "\
#100 = VERTEX_POINT('',#101);
#101 = CARTESIAN_POINT('',(-0.5,1.224646799147E-16,0.));
#102 = EDGE_CURVE('',#31,#100,#39,.T.);
#103 = EDGE_CURVE('',#100,#31,#39,.T.);
#104 = ORIENTED_EDGE('',*,*,#102,.T.);
#105 = ORIENTED_EDGE('',*,*,#103,.T.);
#106 = ORIENTED_EDGE('',*,*,#103,.F.);
#107 = ORIENTED_EDGE('',*,*,#102,.F.);
#73 = PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#7));";
    let text = text.replace(
        "#73 = PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#7));",
        extra,
    );
    // Wall loop: bottom full circle -> the two arcs.
    let text = text.replace(
        "#19 = EDGE_LOOP('',(#20,#29,#37,#44));",
        "#19 = EDGE_LOOP('',(#20,#29,#104,#105,#44));",
    );
    // Bottom cap loop: reversed traversal of the two arcs.
    let text = text.replace(
        "#61 = EDGE_LOOP('',(#62));",
        "#61 = EDGE_LOOP('',(#106,#107));",
    );
    // Wall sense.
    text.replace(
        "#17 = ADVANCED_FACE('',(#18),#45,.T.);",
        &format!("#17 = ADVANCED_FACE('',(#18),#45,{wall_sense});"),
    )
}

/// Control: the three-rim HONEST wall must import and certify green
/// (proves the surgery itself is sound).
#[test]
fn three_rim_honest_control() {
    let text = cylinder_three_rim(".T.");
    match import(&text) {
        Ok(StepImport::Solid { body, .. }) => {
            let t3 = topo::validate_geometric(&body, Tol::witness());
            println!("3RIM honest: imported, t3={t3:?}");
            assert!(
                t3.is_ok(),
                "honest three-rim cylinder must be green: {t3:?}"
            );
        }
        Ok(_) => panic!("wireframe"),
        Err(e) => panic!("honest three-rim cylinder must import: {e}"),
    }
}

/// Deviation 2/3 layering claim, EXECUTED: the flipped three-rim wall
/// is outside the rider's two-rim scope (skip, not refuse) — so the
/// kernel gate must catch the built body. If import refuses, the rider
/// caught it anyway (also fine); if import adopts, tier 3 must refuse
/// CurvedSenseInverted. GREEN-through would falsify the layering.
#[test]
fn three_rim_flipped_wall_layering() {
    let text = cylinder_three_rim(".F.");
    match import(&text) {
        Ok(StepImport::Solid { body, .. }) => {
            let t3 = topo::validate_geometric(&body, Tol::witness());
            println!("3RIM flipped: imported (rider skipped), t3={t3:?}");
            let errs = t3.expect_err("kernel gate must refuse the inverted three-rim wall");
            assert!(
                errs.iter()
                    .any(|e| matches!(e, topo::ValidationError::CurvedSenseInverted { .. })),
                "expected CurvedSenseInverted: {errs:?}"
            );
        }
        Ok(_) => panic!("wireframe"),
        Err(e) => println!("3RIM flipped: rider refused pre-body after all: {e}"),
    }
}

/// cone_trunc (two-rim cone wall — the rider's cone arm, a row the
/// substrate never executed): flipped lateral must refuse pre-body.
#[test]
fn cone_trunc_flipped_wall_refuses() {
    let text = fixture("cone_trunc", "step").replace(
        "#17 = ADVANCED_FACE('',(#18),#45,.T.);",
        "#17 = ADVANCED_FACE('',(#18),#45,.F.);",
    );
    match import(&text) {
        Ok(StepImport::Solid { body, .. }) => {
            let t3 = topo::validate_geometric(&body, Tol::witness());
            panic!("flipped cone_trunc wall imported (t3={t3:?}) — rider missed the cone arm");
        }
        Ok(_) => panic!("wireframe"),
        Err(e) => {
            let s = e.to_string();
            println!("CONE_TRUNC flip refused: {}", &s[..s.len().min(110)]);
            assert!(
                s.contains("ORIENTATION-INVERTED"),
                "typed story expected: {s}"
            );
        }
    }
}

/// The SLIP-BOTH adversary, executed: an obliquely-cut cylinder's wall
/// is trimmed by an ELLIPSE (quadrature-owned boundary). Flip the wall
/// sense in the exported text: the rider's rim collector sees no
/// axis-aligned circle pair (skip), and the kernel's curved arm gets a
/// PropsError from the conic boundary (exempt) while the quadrature
/// volume is winding-derived (bit-free). Does anything refuse?
#[test]
fn conic_trimmed_flip_slips_both_gates() {
    use geom_core::{Point2, Point3, Vec3};
    use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
    use sweep::{Extrusion, extrude};
    use topo::splitting::{SplitPart, SplitPlane, split};
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-1.0, 0.0), 1.0),
        ProfileVertex::new(Point2::new(1.0, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let cylinder = extrude(&profile, Extrusion::Distance(2.5), Tol::witness()).unwrap().body;
    let phi: f64 = 0.3;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 1.25),
        normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
    };
    let result = split(&cylinder, &plane).unwrap();
    let SplitPart::Body(cut) = &result.above else {
        panic!("above half carries material");
    };
    let options = step_export::StepOptions {
        product_name: "cutcyl".to_owned(),
        ..step_export::StepOptions::default()
    };
    let honest_text = step_export::step_string(cut, &options, Tol::witness()).unwrap();
    // Flip the FIRST cylinder wall face's sense in the text.
    let surf_id = honest_text
        .lines()
        .find(|l| l.contains("CYLINDRICAL_SURFACE"))
        .and_then(|l| l.split(" =").next())
        .expect("cylinder surface in export")
        .trim()
        .to_owned();
    let mut flipped_text = String::new();
    let mut done = false;
    for line in honest_text.lines() {
        if !done && line.contains("ADVANCED_FACE") && line.contains(&format!("{surf_id},")) {
            flipped_text.push_str(&line.replace(".T.", ".F."));
            done = true;
        } else {
            flipped_text.push_str(line);
        }
        flipped_text.push('\n');
    }
    assert!(done, "found a wall face referencing {surf_id}");
    // Honest control round-trips green.
    match import(&honest_text) {
        Ok(StepImport::Solid { body, .. }) => {
            assert!(
                topo::validate_geometric(&body, Tol::witness()).is_ok(),
                "honest control green"
            );
        }
        other => panic!("honest cut cylinder must import as solid: {other:?}"),
    }
    match import(&flipped_text) {
        Ok(StepImport::Solid { body, .. }) => {
            let t3 = topo::validate_geometric(&body, Tol::witness());
            let vol = topo::mass_properties(&body, Tol::witness()).map(|p| p.volume);
            println!("CONIC-SLIP flipped wall: imported, t3={t3:?} vol={vol:?}");
        }
        Ok(_) => panic!("wireframe"),
        Err(e) => println!("CONIC-SLIP flipped wall: refused: {e}"),
    }
}
