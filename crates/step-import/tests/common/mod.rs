//! Shared helpers for the acceptance suites: fixture loading, sidecar
//! parsing, censuses, and the kernel-vs-OCC count table.
//!
//! **Comparison discipline (M7-1 spec §2 row 3):** every comparison in
//! these suites is counts, certified scalars, or structural
//! invariants. Nothing here pairs arena order against the writer's
//! walk order — the known trap (`memories/step-curved-subset.md`): the
//! two coincide on simple extrusions and diverge on boolean results.
#![allow(dead_code)] // each consumer uses a subset
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;

use topo::Body;

/// The committed solid corpus, in `fixture_corpus()` file order.
pub const SOLID_FIXTURES: [&str; 14] = [
    "cube",
    "die",
    "kiss_assembly",
    "cut_cylinder",
    "boss_union",
    "notched",
    "washer",
    "ball",
    "cone",
    "donut",
    "lily_lantern",
    "filleted_die",
    "die_pips",
    "composed_die",
];

/// A fixture file's text.
pub fn fixture(name: &str, ext: &str) -> String {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "..",
        "step-export",
        "tests",
        "fixtures",
        &format!("{name}.{ext}"),
    ]
    .iter()
    .collect();
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {path:?}: {e}"))
}

/// A parsed `.expect` sidecar.
#[derive(Clone, Debug)]
pub struct Expect {
    pub solids: usize,
    pub shells: usize,
    pub faces: usize,
    pub edges: usize,
    pub vertices: usize,
    /// The raw `EXPECT_VOLUME_MM3` literal (for print-precision
    /// derivation) and its parsed value.
    pub volume_literal: String,
    pub volume_mm3: f64,
}

/// Parses a `.expect` sidecar's `KEY=value` lines.
pub fn expect_sidecar(name: &str) -> Expect {
    let text = fixture(name, "expect");
    let mut get = |key: &str| -> Option<String> {
        text.lines()
            .find_map(|l| l.strip_prefix(&format!("{key}=")).map(str::to_owned))
    };
    let volume_literal = get("EXPECT_VOLUME_MM3").expect("volume line");
    Expect {
        solids: get("EXPECT_SOLIDS").unwrap().parse().unwrap(),
        shells: get("EXPECT_SHELLS").unwrap().parse().unwrap(),
        faces: get("EXPECT_FACES").unwrap().parse().unwrap(),
        edges: get("EXPECT_EDGES").unwrap().parse().unwrap(),
        vertices: get("EXPECT_VERTICES").unwrap().parse().unwrap(),
        volume_mm3: volume_literal.parse().unwrap(),
        volume_literal,
    }
}

/// The **kernel** census a faithful import must produce where it
/// differs from the sidecar's OCC-normalised edge count. The sidecars
/// record what FreeCAD/OCC *reports after import*, and on five
/// fixtures OCC adds degenerate pole edges or splits periodic
/// carriers at seams — normalisations that are exactly the healing D7
/// forbids, so the kernel keeps its own census. Each entry's kernel
/// counts are the ones the sidecar's own comments state
/// (deviation 1 in the M7-1 report). `None` means the sidecar is the
/// kernel census verbatim.
pub fn kernel_census_override(name: &str) -> Option<(usize, usize, usize)> {
    // (faces, edges, vertices)
    match name {
        // ball.expect: "Kernel census is 2 faces / 2 edges / 2
        // vertices. OCC reports 6 EDGES" (+4 degenerate pole edges).
        "ball" => Some((2, 2, 2)),
        // cone.expect: "Kernel census is 4 faces / 6 edges / 4
        // vertices; OCC reports 8 EDGES" (+2 degenerate apex edges).
        "cone" => Some((4, 6, 4)),
        // filleted_die.expect: "OCC splits the periodic corner-arc
        // carriers at their seams, so its edge and vertex counts
        // exceed the kernel's 48/24".
        "filleted_die" => Some((26, 48, 24)),
        // die_pips.expect: "Kernel census is 48 faces / 96 edges / 71
        // vertices. OCC reports 138 EDGES" (+2 per pip).
        "die_pips" => Some((48, 96, 71)),
        // composed_die.expect: "Kernel counts are 89 faces / 195
        // edges / 129 vertices" (OCC splits periodic carriers).
        "composed_die" => Some((89, 195, 129)),
        _ => None,
    }
}

/// The body's census: (solids, shells, faces, edges, vertices) —
/// plain arena counts, order-free.
pub fn census(body: &Body<f64>) -> (usize, usize, usize, usize, usize) {
    (
        body.solids().count(),
        body.shells().count(),
        body.faces().count(),
        body.edges().count(),
        body.vertices().count(),
    )
}

/// Imports a fixture's committed `.step`, panicking on refusal (the
/// suites' entry point for files that must import).
pub fn import_fixture(name: &str) -> step_import::StepImport {
    let text = fixture(name, "step");
    step_import::import_step(&text, &step_import::ImportOptions::default())
        .unwrap_or_else(|e| panic!("importing {name}: {e}"))
}

/// The imported solid body, panicking on a wireframe disposition.
pub fn import_body(name: &str) -> (Body<f64>, f64) {
    match import_fixture(name) {
        step_import::StepImport::Solid { body, eps_in } => (body, eps_in),
        step_import::StepImport::Wireframe { .. } => {
            panic!("{name} imported as a wireframe, expected a solid")
        }
    }
}
