//! STEP-import route for the S28 guard false refusal.
#![allow(clippy::all, clippy::pedantic)]
use mesh::tessellate;
use mesh::validate::check_mesh;
use step_import::{ImportOptions, StepImport, import_step};

fn main() {
    for path in std::env::args().skip(1) {
        let text = std::fs::read_to_string(&path).unwrap();
        let name = path.rsplit('/').next().unwrap().to_string();
        let imported = match import_step(&text, &ImportOptions::default()) {
            Ok(i) => i,
            Err(e) => {
                println!("{name}: IMPORT REFUSED {e:?}");
                continue;
            }
        };
        let StepImport::Solid { body, .. } = imported else {
            println!("{name}: not a solid");
            continue;
        };
        for d in [2.0e-3, 5.0e-4] {
            match tessellate(&body, d) {
                Ok(m) => {
                    let n: usize = m.patches.iter().map(|p| p.triangles.len()).sum();
                    match check_mesh(&m) {
                        Ok(()) => println!("{name} d={d}: OK {n} tris, check_mesh CLEAN"),
                        Err(e) => println!("{name} d={d}: OK {n} tris, check_mesh DIRTY {e:?}"),
                    }
                }
                Err(e) => println!("{name} d={d}: REFUSED {e:?}"),
            }
        }
    }
}
