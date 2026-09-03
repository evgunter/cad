//! **R2 review probes for MESH-12** (issue 1588, measurement 2): the
//! two-level rim row through the import door, reproduced from this
//! lane's own fixtures (`fixtures/mesh12r2/gen_two_level_rim.py`, the
//! generator committed beside its output). A half-cap whose rim is two
//! on-sphere arcs at latitudes `v` and `v + Δv`, junction at the mean
//! latitude, `R·Δv = f·ε` for `f ∈ {0.5, 1.0, 1.5}` at the default
//! band. The claim under review: `import_step` refuses `f ≥ 1` at the
//! pcurve re-mint (`pcurve_loop_continuity`) before any props decide
//! runs, and the `f = 0.5` shape imports, meshes watertight and is
//! quiet under `examine_chart_coherence`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use std::collections::HashMap;

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn fixture(name: &str) -> String {
    let path = format!(
        "{}/tests/fixtures/mesh12r2/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {path}: {e}"))
}

/// **At and over the band the import door refuses at the re-mint**, and
/// the refusal names `pcurve_loop_continuity`, not a props decide.
#[test]
fn r2_the_two_level_rim_row_at_and_over_the_band_refuses_at_the_pcurve_remint() {
    let tol = Tol::witness();
    for name in ["two_level_rim_f1.step", "two_level_rim_f1p5.step"] {
        let r = import_step(&fixture(name), &ImportOptions::default(), tol);
        let text = match &r {
            Ok(StepImport::Solid { .. }) => "imported as a solid".to_owned(),
            Ok(other) => format!("imported: {other:?}"),
            Err(e) => format!("{e:?}"),
        };
        println!("R2-IMPORT {name} (eps {:e}): {text}", tol.eps());
        assert!(
            text.contains("pcurve_loop_continuity"),
            "{name}: expected the pcurve re-mint's refusal, got {text}"
        );
        assert!(
            !text.contains("props_"),
            "{name}: a props decide ran before the re-mint: {text}"
        );
    }
}

/// **Inside the band the shape imports, meshes watertight, and the
/// examination is quiet.**
#[test]
fn r2_the_two_level_rim_row_inside_the_band_imports_meshes_and_is_quiet() {
    let tol = Tol::witness();
    let name = "two_level_rim_f0p5.step";
    let Ok(StepImport::Solid { body, .. }) =
        import_step(&fixture(name), &ImportOptions::default(), tol)
    else {
        panic!("{name} must import as a solid at eps {:e}", tol.eps());
    };
    topo::validate_geometric(&body, tol).unwrap_or_else(|e| panic!("{name} tier 3: {e:?}"));
    let report = topo::examine_chart_coherence(&body, tol);
    println!(
        "R2-IMPORT {name}: findings={:?} unexamined={:?}",
        report.findings, report.unexamined
    );
    assert!(report.findings.is_empty(), "{:?}", report.findings);
    assert!(report.unexamined.is_empty(), "{:?}", report.unexamined);
    let mp = topo::mass_properties(&body, tol).unwrap_or_else(|e| panic!("{name}: {e:?}"));
    println!(
        "R2-IMPORT {name}: volume={:e} pad={:e}",
        mp.volume, mp.volume_pad
    );
    for delta in [1.0e-3, 1.0e-4] {
        let m = mesh::tessellate(&body, delta, tol)
            .unwrap_or_else(|e| panic!("{name} at δ={delta}: {e:?}"));
        let mut uses: HashMap<(u32, u32), u32> = HashMap::new();
        let mut tris = 0usize;
        for fp in &m.patches {
            for t in &fp.triangles {
                tris += 1;
                for k in 0..3 {
                    let (a, b) = (t[k], t[(k + 1) % 3]);
                    *uses.entry((a.min(b), a.max(b))).or_insert(0) += 1;
                }
            }
        }
        let open: Vec<_> = uses.iter().filter(|(_, n)| **n != 2).collect();
        println!(
            "R2-IMPORT {name} δ={delta}: {tris} triangles, {} edges, {} not used exactly twice",
            uses.len(),
            open.len()
        );
        assert!(open.is_empty(), "not watertight at δ={delta}: {open:?}");
    }
}
