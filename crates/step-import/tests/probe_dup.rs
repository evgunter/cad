//! REVIEW PROBE (B3): the duplicate-solid rule. When do two
//! representations name ONE solid vs two?
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;
use common::census;
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn box_step() -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/freecad/box.step"
    );
    std::fs::read_to_string(path).unwrap()
}

fn report(tag: &str, text: &str) {
    match import_step(text, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { body, .. }) => {
            println!(
                "{tag}: SOLID census (solids,shells,faces,edges,verts)={:?} tier3={:?}",
                census(&body),
                topo::validate_geometric(&body, Tol::witness()).map(|()| "ok")
            );
        }
        Ok(StepImport::Wireframe { .. }) => println!("{tag}: WIREFRAME"),
        Err(e) => println!("{tag}: REFUSES: {e}"),
    }
}

#[test]
fn duplicate_solid_rules() {
    let base = box_step();
    report("baseline box", &base);

    // E1: the SAME MANIFOLD_SOLID_BREP (#15) named by a second, plain
    // SHAPE_REPRESENTATION — the ftc_09 shape. One solid is right.
    let e1 = base.replace(
        "#10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#15),#165);",
        "#10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#15),#165);\n#900 = SHAPE_REPRESENTATION('',(#11,#15),#165);",
    );
    assert_ne!(e1, base);
    report("E1 one MSB, two representations", &e1);

    // E2: TWO DISTINCT MSB entities over the same CLOSED_SHELL (#16):
    // identical geometry, same placement — the nasty twin. Two solids
    // (entity identity), or a typed refusal; never a silent collapse.
    let e2 = base.replace(
        "#10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#15),#165);",
        "#10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#15,#901),#165);\n#901 = MANIFOLD_SOLID_BREP('',#16);",
    );
    assert_ne!(e2, base);
    report("E2 twin MSBs, same shell, same placement", &e2);

    // E3: the same MSB listed twice in ONE representation.
    let e3 = base.replace("(#11,#15)", "(#11,#15,#15)");
    assert_ne!(e3, base);
    report("E3 one MSB listed twice in one rep", &e3);
}
