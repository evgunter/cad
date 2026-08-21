//! Shared helpers for the acceptance suites: fixture loading, sidecar
//! parsing (both the OCC-side `EXPECT_*` and kernel-side `KERNEL_*`
//! field families), and censuses.
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
use geom_core::Tol;

/// The committed solid corpus, in `fixture_corpus()` file order.
/// `loft_prism` joined at M7-3 (14 → 15): the first NURBS-walled
/// fixture, whose row-1 exclusion reason — the
/// `B_SPLINE_SURFACE_WITH_KNOTS` vocabulary refusal — that unit
/// retired (the S9 flip recorded in `review_k3_probe.rs`).
///
/// `nonuniform_loft` and `swept_elbow` joined at the #210 corpus fold
/// (15 → 17): the exportable class #207's skin-fit fix opened — a loft
/// whose sections are NON-uniformly spaced, and the tree's first
/// curved-path `sweep_body`. Both are non-rational NURBS-walled, so
/// they put M7-3's surface arm, its IsoCurve seam rung and its rim
/// pcurve re-mint on bodies the writer could not produce until #210.
pub const SOLID_FIXTURES: [&str; 17] = [
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
    "loft_prism",
    "nonuniform_loft",
    "swept_elbow",
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
///
/// Two censuses live in one sidecar: `EXPECT_*` records what
/// FreeCAD/OCC *reports after importing* the exported file (degenerate
/// pole edges added, periodic carriers seam-split — normalisations
/// that are exactly the healing D7 forbids, so the kernel keeps its
/// own census), and `KERNEL_*` records the NATIVE body's census plus
/// its certified volume at FULL precision (the literal is the export
/// writer's round-tripping float printer's output). The `KERNEL_*`
/// fields cannot rot: step-export's `tests/kernel_sidecars.rs`
/// staleness row asserts them against the live kernel every run.
#[derive(Clone, Debug)]
pub struct Expect {
    pub solids: usize,
    pub shells: usize,
    pub faces: usize,
    pub edges: usize,
    pub vertices: usize,
    pub volume_mm3: f64,
    pub kernel_solids: usize,
    pub kernel_shells: usize,
    pub kernel_faces: usize,
    pub kernel_edges: usize,
    pub kernel_vertices: usize,
    /// Parses back to the exact bits of (native certified volume ×
    /// 1e9) — the printer's round-trip guarantee. For quadrature
    /// bodies this is the enclosure MIDPOINT at the corpus's declared
    /// uncertainty ε = 1e-9 (the enclosure is a function of ambient
    /// ε); the bracket is `± kernel_volume_pad_mm3`.
    pub kernel_volume_mm3: f64,
    /// The certified half-width of the native volume enclosure at
    /// ε = 1e-9, in mm³ — `0.0` for closed-form-only bodies.
    pub kernel_volume_pad_mm3: f64,
}

/// Parses a `.expect` sidecar's `KEY=value` lines.
pub fn expect_sidecar(name: &str) -> Expect {
    let text = fixture(name, "expect");
    let get = |key: &str| -> String {
        text.lines()
            .find_map(|l| l.strip_prefix(&format!("{key}=")).map(str::to_owned))
            .unwrap_or_else(|| panic!("{name}.expect: missing {key}= line"))
    };
    Expect {
        solids: get("EXPECT_SOLIDS").parse().unwrap(),
        shells: get("EXPECT_SHELLS").parse().unwrap(),
        faces: get("EXPECT_FACES").parse().unwrap(),
        edges: get("EXPECT_EDGES").parse().unwrap(),
        vertices: get("EXPECT_VERTICES").parse().unwrap(),
        volume_mm3: get("EXPECT_VOLUME_MM3").parse().unwrap(),
        kernel_solids: get("KERNEL_SOLIDS").parse().unwrap(),
        kernel_shells: get("KERNEL_SHELLS").parse().unwrap(),
        kernel_faces: get("KERNEL_FACES").parse().unwrap(),
        kernel_edges: get("KERNEL_EDGES").parse().unwrap(),
        kernel_vertices: get("KERNEL_VERTICES").parse().unwrap(),
        kernel_volume_mm3: get("KERNEL_VOLUME_MM3").parse().unwrap(),
        kernel_volume_pad_mm3: get("KERNEL_VOLUME_PAD_MM3").parse().unwrap(),
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
    let options = if name.contains("kiss_assembly") {
        step_import::ImportOptions {
            // The corpus's one touching assembly: its corner kiss at
            // (1, 1, 1) is DECLARED through the M9-2 import-side channel
            // (D7 step 4) — the shared tier-3′ gate then certifies the
            // touch instead of refusing it undeclared.
            declared_contacts: vec![step_import::ImportContact::VertexRest {
                at: [1.0, 1.0, 1.0],
            }],
            ..step_import::ImportOptions::default()
        }
    } else {
        step_import::ImportOptions::default()
    };
    step_import::import_step(&text, &options, Tol::witness()).unwrap_or_else(|e| panic!("importing {name}: {e}"))
}

/// The imported solid body, panicking on a wireframe disposition.
pub fn import_body(name: &str) -> (Body<f64>, f64) {
    match import_fixture(name) {
        step_import::StepImport::Solid { body, eps_in, .. } => (body, eps_in),
        step_import::StepImport::Wireframe { .. } => {
            panic!("{name} imported as a wireframe, expected a solid")
        }
    }
}

/// The committed FreeCAD 1.1.2 corpus (M7-2), in generator order.
pub const FREECAD_FIXTURES: [&str; 13] = [
    "box",
    "cylinder",
    "cone_trunc",
    "cone_apex",
    "sphere",
    "torus",
    "box_hole",
    "fuse_boxes",
    "box_fillet_edge",
    "box_fillet_corner",
    "compound_two",
    "box_importexport",
    "twobody_importexport",
];

/// A committed FreeCAD fixture's text.
pub fn freecad_fixture(name: &str) -> String {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        "freecad",
        &format!("{name}.step"),
    ]
    .iter()
    .collect();
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {path:?}: {e}"))
}
