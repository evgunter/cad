//! M3 PR 6a, D9 (exports): mass properties + tessellation/STL on 3′
//! corpus bodies — M2's export machinery meets the touching class.
//! Watertight self-check (`check_mesh`) where export succeeds; STLs
//! land in `REVIEW_STL_DIR` (or a temp dir).
//!
//! **Admesh posture (documented, verified 2026-07-23)**: on both
//! assemblies admesh reports every facet connected, zero holes, and
//! the exact volume — the touching class exports watertight — but
//! counts `Number of parts = 2`: a touching assembly IS two closed
//! parts (the kiss point/tangent edge does not fuse the patches).
//! The repo's `check_admesh.sh` enforces the single-part convention
//! of the die e2e, so these STLs are deliberately NOT added to the
//! gate's acceptance directory; the per-part watertightness is
//! asserted here through `check_mesh` + `signed_volume` instead.
//! Nothing about the class is unexportable — the multi-part count is
//! the honest shape of a 3′ assembly.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use mesh::validate::{check_mesh, signed_volume};
use topo::{BooleanResult, mass_properties, union, validate_pseudomanifold};

mod common;
use common::brick;
use geom_core::Tol;

fn stl_dir() -> String {
    let dir = std::env::var("REVIEW_STL_DIR").unwrap_or_else(|_| {
        std::env::temp_dir()
            .join("m3_pr6_stl")
            .to_string_lossy()
            .into_owned()
    });
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// The corner-kiss assembly (certified 3′): mass properties exact;
/// tessellation + mesh self-check + STL. Two shells kissing at one
/// point tessellate as two closed components — `check_mesh` accepts
/// (the kiss point is one POSITION but the shells' patches keep their
/// own vertices; edge-manifoldness holds per component) and the mesh
/// volume matches, so the class IS exportable; the STL feeds the
/// external admesh row.
#[test]
fn corner_kiss_assembly_exports() {
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let BooleanResult::Body(r) = union(&a, &b, Tol::witness()).unwrap() else {
        panic!("kiss union is a body");
    };
    assert_eq!(
        validate_pseudomanifold(&r.body, &r.contacts, Tol::witness()),
        Ok(())
    );
    // Shell count pinned alongside exact volume: admesh's parts=N alone
    // cannot distinguish a legit assembly from a wrongly-disconnected
    // one-part result (review R7b).
    assert_eq!(r.body.shells().count(), 2);
    let m = mass_properties(&r.body, Tol::witness()).unwrap();
    assert_eq!(m.volume, 2.0);
    assert_eq!(m.surface_area, 12.0);
    let mesh = mesh::tessellate(&r.body, 1e-2, Tol::witness()).unwrap();
    check_mesh(&mesh).unwrap();
    let v = signed_volume(&mesh);
    assert!((v - 2.0).abs() < 1e-9, "mesh volume {v}");
    let path = format!("{}/corner_kiss.stl", stl_dir());
    let mut file = std::fs::File::create(&path).unwrap();
    stl::write_binary(&mesh, &stl::BinaryOptions::default(), &mut file).unwrap();
}

/// The tangent-edge assembly (certified 3′ with the D3-reconstructed
/// coincident-edge segment): exact mass properties, watertight mesh,
/// STL for admesh.
#[test]
fn tangent_edge_assembly_exports() {
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick((1.0, 2.0), (0.0, 1.0), (1.0, 2.0));
    let BooleanResult::Body(r) = union(&a, &b, Tol::witness()).unwrap() else {
        panic!("tangent-edge union is a body");
    };
    assert_eq!(
        validate_pseudomanifold(&r.body, &r.contacts, Tol::witness()),
        Ok(())
    );
    // Shell count pinned alongside exact volume: admesh's parts=N alone
    // cannot distinguish a legit assembly from a wrongly-disconnected
    // one-part result (review R7b).
    assert_eq!(r.body.shells().count(), 2);
    let m = mass_properties(&r.body, Tol::witness()).unwrap();
    assert_eq!(m.volume, 2.0);
    assert_eq!(m.surface_area, 12.0);
    let mesh = mesh::tessellate(&r.body, 1e-2, Tol::witness()).unwrap();
    check_mesh(&mesh).unwrap();
    let v = signed_volume(&mesh);
    assert!((v - 2.0).abs() < 1e-9, "mesh volume {v}");
    let path = format!("{}/tangent_edge.stl", stl_dir());
    let mut file = std::fs::File::create(&path).unwrap();
    stl::write_binary(&mesh, &stl::BinaryOptions::default(), &mut file).unwrap();
}
