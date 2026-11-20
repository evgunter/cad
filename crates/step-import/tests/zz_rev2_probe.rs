//! Review probe (rev2): import scratch files named by REV2_DIR.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

#[test]
fn rev2_import_scratch_files() {
    let dir = std::env::var("REV2_DIR").unwrap_or_else(|_| {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/rev2-probe").to_owned()
    });
    let mut names: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "step"))
        .collect();
    names.sort();
    for p in names {
        let text = std::fs::read_to_string(&p).unwrap();
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        match import_step(&text, &ImportOptions::default(), Tol::witness()) {
            Ok(StepImport::Solid {
                ref normalizations, ..
            }) => {
                println!(
                    "REV2 {name}: OK solid; ledger {:?}",
                    normalizations
                        .iter()
                        .map(|n| (n.face, format!("{:?}", n.kind)))
                        .collect::<Vec<_>>()
                );
            }
            Ok(StepImport::Wireframe { curves, .. }) => {
                println!("REV2 {name}: wireframe {curves:?}")
            }
            Err(e) => println!("REV2 {name}: ERR {e}"),
        }
    }
}
