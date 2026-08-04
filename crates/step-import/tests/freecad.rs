//! M7-2 acceptance: the FreeCAD-authored foreign corpus.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

/// Temporary triage while the legs land: what each fixture does today.
#[test]
fn triage() {
    for name in common::FREECAD_FIXTURES {
        let text = common::freecad_fixture(name);
        match step_import::import_step(&text, &step_import::ImportOptions::default()) {
            Ok(step_import::StepImport::Solid {
                body,
                eps_in,
                normalizations,
            }) => {
                let props = topo::mass_properties(&body);
                println!(
                    "{name}: OK census {:?} eps_in {eps_in:e} vol {:?} norm {} v1 {:?} v2 {:?} v3 {:?}",
                    common::census(&body),
                    props.map(|p| p.volume),
                    normalizations.len(),
                    topo::validate(&body).is_ok(),
                    topo::validate_closed(&body).err(),
                    topo::validate_geometric(&body).err(),
                );
            }
            Ok(_) => println!("{name}: wireframe"),
            Err(e) => println!("{name}: REFUSED {e}"),
        }
    }
}
