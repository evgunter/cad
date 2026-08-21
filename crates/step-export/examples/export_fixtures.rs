//! Regenerates the committed STEP fixtures — the planar M4 set (cube,
//! M3 pocketed die, corner-kiss assembly), the M5 PR 13 curved corpus
//! (cut_cylinder, boss_union, notched, washer, ball, cone, donut), the
//! M5 PR 12 die, both halves (filleted_die, die_pips), and the NURBS
//! set (loft_prism, plus the #210 fold's nonuniform_loft and
//! swept_elbow) — the files `scripts/check_step.sh` feeds to
//! FreeCAD/OCC for the external-import acceptance, kept byte-golden by
//! `tests/export.rs::committed_fixtures_are_byte_golden`.
//!
//! Usage:
//! `cargo run -p step-export --example export_fixtures -- crates/step-export/tests/fixtures`
//!
//! Options here MUST match the test suite's `export` helper (explicit
//! uncertainty pins the file against ambient-ε env overrides).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// The same acceptance-body builders the export tests use (examples and
// integration tests are sibling targets).
#[path = "../tests/common/mod.rs"]
mod common;

use step_export::{StepOptions, write_step};
use geom_core::Tol;

fn main() {
    let tol = Tol::witness();
    let outdir = std::env::args()
        .nth(1)
        .expect("usage: export_fixtures <outdir>");
    std::fs::create_dir_all(&outdir).expect("create outdir");
    for (name, body) in common::fixture_corpus() {
        let options = StepOptions {
            product_name: name.to_owned(),
            uncertainty_m: Some(1e-9),
            ..StepOptions::default()
        };
        let path = format!("{outdir}/{name}.step");
        let mut file = std::fs::File::create(&path).expect("create fixture file");
        write_step(&body, &options, &mut file, tol).expect("write fixture");
        println!("wrote {path}");
    }
}
