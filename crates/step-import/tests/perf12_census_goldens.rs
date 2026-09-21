//! **The import census golden**: every STEP fixture the import door
//! puts through the tier-3′ census (`gate3` →
//! `topo::validate_pseudomanifold_certificate_certified`) — the
//! step-export solids and the FreeCAD set, `twobody_importexport` and
//! `compound_two` among them the multi-solid rows — with the import's
//! outcome serialized and compared byte-exact against a committed
//! golden per ε row.
//!
//! An `Ok` row records the body's arena counts; a refusal records the
//! error vector one finding per line. A candidate pre-filter in the
//! census must leave every row byte-identical (the editor-core twin,
//! `perf12_census_goldens`, says why the K-funnel's log is not part of
//! the pin).
//!
//! # Re-blessing
//!
//! `PERF12_BLESS_CENSUS=1` at each ε row (`CAD_TOLERANCE_EPS=1e-6`,
//! default, `1e-12`); inspect the diff; commit it WITH the change it
//! records.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{FREECAD_FIXTURES, SOLID_FIXTURES, arena_census, fixture, freecad_fixture};
use geom_core::Tol;
use step_import::{ImportContact, ImportOptions, StepImport, import_step};

const CENSUS_GOLDENS: &[(&str, &str, &str)] = &[
    (
        "1e-6",
        include_str!("golden/perf12_census_1e-6.txt"),
        "tests/golden/perf12_census_1e-6.txt",
    ),
    (
        "1e-9",
        include_str!("golden/perf12_census_1e-9.txt"),
        "tests/golden/perf12_census_1e-9.txt",
    ),
    (
        "1e-12",
        include_str!("golden/perf12_census_1e-12.txt"),
        "tests/golden/perf12_census_1e-12.txt",
    ),
];

/// `common::import_fixture`'s options, verbatim: the kiss assembly
/// declares its vertex rest, every other fixture imports bare.
fn options(name: &str) -> ImportOptions {
    if name.contains("kiss_assembly") {
        ImportOptions {
            declared_contacts: vec![ImportContact::VertexRest {
                at: [1.0, 1.0, 1.0],
            }],
            ..ImportOptions::default()
        }
    } else {
        ImportOptions::default()
    }
}

fn row(label: &str, text: &str, options: &ImportOptions, out: &mut String) {
    out.push_str(&format!("## {label}\n"));
    match import_step(text, options, Tol::witness()) {
        Ok(StepImport::Solid { body, .. }) => {
            let (solids, shells, faces, edges, vertices) = arena_census(&body);
            out.push_str(&format!(
                "import: Ok(Solid) solids={solids} shells={shells} faces={faces} edges={edges} vertices={vertices}\n"
            ));
        }
        Ok(StepImport::Wireframe { .. }) => out.push_str("import: Ok(Wireframe)\n"),
        Err(step_import::StepImportError::TierInvalid { solid, errors }) => {
            out.push_str(&format!(
                "import: Err(TierInvalid) solid={solid:?} {} findings\n",
                errors.len()
            ));
            for e in &errors {
                out.push_str(&format!("{e:?}\n"));
            }
        }
        Err(e) => out.push_str(&format!("import: Err({e:?})\n")),
    }
}

fn census_text() -> String {
    let mut out = String::new();
    for name in SOLID_FIXTURES {
        row(name, &fixture(name, "step"), &options(name), &mut out);
    }
    for name in FREECAD_FIXTURES {
        row(
            &format!("freecad/{name}"),
            &freecad_fixture(name),
            &ImportOptions::default(),
            &mut out,
        );
    }
    out
}

#[test]
fn the_import_census_verdicts_are_goldened_bit_exact() {
    let eps = format!("{:e}", Tol::witness().eps());
    let Some(&(_, golden, path)) = CENSUS_GOLDENS.iter().find(|(row, _, _)| *row == eps) else {
        panic!(
            "no committed import census golden for eps={eps}; the blessed rows are {}. Bless \
             it with PERF12_BLESS_CENSUS=1 and commit the file WITH the change it records.",
            CENSUS_GOLDENS
                .iter()
                .map(|(r, _, _)| *r)
                .collect::<Vec<_>>()
                .join(", ")
        );
    };
    let text = census_text();
    if std::env::var("PERF12_BLESS_CENSUS").is_ok() {
        std::fs::write(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path),
            &text,
        )
        .expect("bless writes");
        panic!(
            "import census golden for eps={eps} re-blessed — commit the file WITH the change \
             it records, then rerun without the env var"
        );
    }
    if text != golden {
        let first = text
            .lines()
            .zip(golden.lines())
            .position(|(a, b)| a != b)
            .map_or_else(
                || "a length change".to_string(),
                |i| format!("line {}", i + 1),
            );
        panic!(
            "the import census verdicts drifted from their committed golden at {first} \
             (eps={eps}): the census is deciding differently. Read the diff, decide whether \
             the new verdict is right, and re-bless deliberately (PERF12_BLESS_CENSUS=1)."
        );
    }
}
