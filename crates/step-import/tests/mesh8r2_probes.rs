//! **R2 review probes for MESH-8** (issue 868), the import-door leg:
//! issue 723's committed half-cap fixture through `import_step`, with
//! the relocated examination's numbers PRINTED rather than asserted
//! band by band, so a reader can see the band shape rather than take
//! it on the row's word.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

/// Print the examination's findings on `halfcap_eps7.step` at the
/// run's ambient ε, beside what `tessellate` does with the same body.
/// Run it at ε ∈ {1e-6, 1e-9, 1e-12}: the arc opens π · 1e-9 m, so
/// the report must be EMPTY at 1e-6; at 1e-9 the import refuses, and
/// at 1e-12 the joined arc reports once.
#[test]
fn r2r_the_halfcap_fixture_band_shape() {
    let eps = Tol::witness().eps();
    let p = format!(
        "{}/tests/fixtures/halfcap/halfcap_eps7.step",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&p).unwrap();
    // The import ends with the join (Ev, PR 4251): at the default band
    // the split vertex's reading lands in the sliver band and the file
    // refuses; at 1e-12 the vertex is joined (`poleguard.rs` pins both).
    let body = match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { body, .. }) => body,
        Ok(StepImport::Wireframe { .. }) => panic!("halfcap_eps7 is a solid"),
        Err(e) => {
            println!("HALFCAP eps={eps:e} refused: {e}");
            return;
        }
    };
    let report = topo::examine_chart_coherence(&body, Tol::witness());
    let tess = match mesh::tessellate(&body, 1.0e-3, Tol::witness()) {
        Ok(m) => format!(
            "Ok({} tris)",
            m.patches.iter().map(|p| p.triangles.len()).sum::<usize>()
        ),
        Err(e) => format!("{e:?}"),
    };
    println!(
        "HALFCAP eps={eps:e} tessellate={tess} findings={} unexamined={}",
        report.findings.len(),
        report.unexamined.len()
    );
    for f in &report.findings {
        println!(
            "HALFCAP   {:?} gap={} lever={:e} metres={:e} over_band={}",
            f.condition,
            f.gap,
            f.lever,
            f.metres,
            f.metres >= eps
        );
    }
}
