//! TESS-CAP-DIAG scratch survey (diagnostic branch only): what the
//! STEP import door does with a spherical cap stated three ways
//! (`fixtures/tess-cap-diag/gen_cap.py`), and what `mesh::tessellate`
//! does with whatever it adopts. Prints; asserts nothing.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn fixture(name: &str) -> String {
    let p = format!(
        "{}/tests/fixtures/tess-cap-diag/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("reading {p}: {e}"))
}

#[test]
fn q4d_import_of_a_spherical_cap() {
    let tol = Tol::witness();
    for name in ["rimonly2.step", "rimonly1.step", "seamed.step"] {
        println!("\n=== import {name}");
        let imported = import_step(&fixture(name), &ImportOptions::default(), tol);
        let body = match imported {
            Err(e) => {
                println!("  import REFUSED: {e:?}");
                continue;
            }
            Ok(StepImport::Solid { body, .. }) => body,
            Ok(other) => {
                let d = format!("{other:?}");
                println!("  import: not a Solid: {}", &d[..d.len().min(300)]);
                continue;
            }
        };
        println!(
            "  census V={} E={} F={}",
            body.vertices().count(),
            body.edges().count(),
            body.faces().count()
        );
        for (fk, face) in body.faces() {
            let (outer, _) = topo::props::loop_edges(&body, face.outer).unwrap();
            let s = format!("{:?}", body.get_surface(face.surface).unwrap());
            println!(
                "  face {fk:?} {} sense={} rings={} outer edges={}",
                s.split([' ', '{']).next().unwrap_or(""),
                face.sense,
                face.rings.len(),
                outer.len()
            );
        }
        println!(
            "  tiers: t1={:?} t2={:?} t3={:?}",
            topo::validate(&body),
            topo::validate_closed(&body),
            topo::validate_geometric(&body, tol)
        );
        match topo::mass_properties(&body, tol) {
            Ok(mp) => println!(
                "  props: volume={:.9e} area={:.9e}",
                mp.volume, mp.surface_area
            ),
            Err(e) => println!("  props: REFUSED {e:?}"),
        }
        let t = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            mesh::tessellate(&body, 1e-4, tol)
        }));
        match t {
            Err(p) => {
                let s = p
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
                    .unwrap_or_default();
                println!("  tessellate: PANIC {}", &s[..s.len().min(300)]);
            }
            Ok(Err(e)) => println!("  tessellate: REFUSED {e:?}"),
            Ok(Ok(m)) => {
                println!(
                    "  tessellate: OK triangles={} check_mesh={:?} signed_volume={:.9e}",
                    mesh::validate::triangle_count(&m),
                    mesh::validate::check_mesh(&m),
                    mesh::validate::signed_volume(&m)
                );
                for p in &m.patches {
                    println!("      patch {:?}: {} triangles", p.face, p.triangles.len());
                }
            }
        }
    }
}
