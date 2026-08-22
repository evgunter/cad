//! Export the M2 acceptance bodies as binary STL for external
//! verification (the CI admesh job): builds each body through the
//! public profile/sweep APIs, pre-flights our own validators
//! (tier 1–3 incl. the +V invariant, `mesh::validate::check_mesh`,
//! `signed_volume > 0`), then writes `<outdir>/<name>.stl`.
//!
//! Usage: `cargo run -p stl --example export_acceptance -- <outdir>`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// The same acceptance-body builders the export tests use (shared by
// include — examples and integration tests are sibling targets).
#[path = "../tests/common/mod.rs"]
mod common;

use geom_core::Tol;
use mesh::validate::{check_mesh, signed_volume, triangle_count};

fn main() {
    let tol = Tol::witness();
    let outdir = std::env::args()
        .nth(1)
        .expect("usage: export_acceptance <outdir>");
    std::fs::create_dir_all(&outdir).expect("create outdir");
    for (name, body, delta) in common::acceptance_bodies() {
        topo::validate_geometric(&body, tol)
            .unwrap_or_else(|e| panic!("{name}: tier-3 validation failed: {e:?}"));
        let props = topo::mass_properties(&body, tol).expect("mass properties");
        let mesh = mesh::tessellate(&body, delta, tol).expect("tessellate");
        check_mesh(&mesh).unwrap_or_else(|e| panic!("{name}: check_mesh failed: {e:?}"));
        let v = signed_volume(&mesh);
        assert!(v > 0.0, "{name}: mesh signed volume must be positive");
        let path = format!("{outdir}/{name}.stl");
        let mut file = std::fs::File::create(&path).expect("create stl file");
        stl::write_binary(&mesh, &stl::BinaryOptions::default(), &mut file).expect("write stl");
        println!(
            "exported {path}: {} triangles, V_exact = {:.6}, V_mesh = {v:.6}, A_exact = {:.6}",
            triangle_count(&mesh),
            props.volume,
            props.surface_area,
        );
    }
}
