//! **RING-2 review end-to-end (R2 lane)**: certify a NURBS-faced body
//! through the façade — `validate_geometric`, `mass_properties`, a
//! tessellation with its budget meter armed — and print every limb as
//! bits, so the same program at the merge base's ring and at this head
//! shows which numbers moved and which did not.
//!
//! Run: `cargo run -p pncad --example ring2_r2_e2e --features budget,sweep/test-support`
//!
//! **The comparison it is kept for is RING-3's**, where the newtype
//! dissolves into `Interval` and every number below is taken again. It
//! prints and asserts nothing, so it is not a gate and is not built by
//! the default-feature gate either — the manifest's `required-features`
//! is what keeps it out, and the line above is what runs it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::geom_core::Tol;
use pncad::mesh::budget;
use pncad::prelude::{mass_properties, tessellate, validate_geometric};

fn main() {
    let tol = Tol::witness();
    let body = pncad::sweep::test_support::loft_prism(tol);
    println!(
        "faces {} edges {}",
        body.faces().count(),
        body.edges().count()
    );
    match validate_geometric(&body, tol) {
        Ok(()) => println!("validate_geometric: Ok"),
        Err(errs) => println!("validate_geometric: {} error(s): {errs:?}", errs.len()),
    }
    let m = mass_properties(&body, tol).expect("mass properties certify");
    println!(
        "volume {:.17e} ({:#018x}) pad {:.17e} ({:#018x})",
        m.volume,
        m.volume.to_bits(),
        m.volume_pad,
        m.volume_pad.to_bits()
    );
    println!(
        "area   {:.17e} ({:#018x}) pad {:.17e} ({:#018x})",
        m.surface_area,
        m.surface_area.to_bits(),
        m.area_pad,
        m.area_pad.to_bits()
    );
    budget::arm(budget::Mode::Sizing);
    let mesh = tessellate(&body, 6e-3, tol).expect("tessellates");
    println!(
        "triangles {} positions {}",
        mesh.patches
            .iter()
            .map(|p| p.triangles.len())
            .sum::<usize>(),
        mesh.positions.len()
    );
    for f in budget::take() {
        println!(
            "face {:?}: muu {:e} muv {:e} mvv {:e} mu1 {:e} mv1 {:e} steps {:?} cells {} grid {} \
             worst_cert {:e}",
            f.face,
            f.muu,
            f.muv,
            f.mvv,
            f.mu1,
            f.mv1,
            f.patch_steps,
            f.cells.len(),
            f.grid_cells,
            f.worst_cert
        );
    }
}
