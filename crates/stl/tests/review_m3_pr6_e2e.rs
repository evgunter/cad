//! M3 PR 6a ADVERSARIAL REVIEW, R6 — independent consumer e2e (not a
//! rerun of the implementer's exports test): two pocketed dies built
//! by SUBTRACT, corner-kissed by UNION, then
//! validate_pseudomanifold → mass properties → tessellation →
//! check_mesh → STL, against independently computed exact oracles.
//!
//! Oracles (reviewer's derivation, scratchpad): each die is a unit
//! cube minus a 0.5×0.5 pocket of depth 0.5 cut through its top face
//! → volume 1 − 0.125 = 0.875 exactly; the kiss assembly is
//! 2 × 0.875 = 1.75 exactly, TWO shells, one vv contact at (1,1,1).
//! Surface area per die: cube 6 minus pocket mouth 0.25 plus pocket
//! walls 4×(0.5×0.5) = 1 plus pocket floor 0.25 → 7.0; assembly 14.0.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use mesh::validate::{check_mesh, signed_volume};
use topo::{
    Body, BooleanResult, ContactRecords, ValidationError, mass_properties, subtract, union,
    validate_pseudomanifold,
};

use crate::common;
use common::brick;
use geom_core::Tol;

/// A pocketed die: `[x0,x0+1]³` minus a centered 0.5×0.5×0.5 pocket
/// opening through the TOP face (cutter overshoots above). Exact
/// volume 0.875.
fn die(x0: f64, y0: f64, z0: f64) -> Body<f64> {
    let cube = brick((x0, x0 + 1.0), (y0, y0 + 1.0), (z0, z0 + 1.0));
    let cutter = brick(
        (x0 + 0.25, x0 + 0.75),
        (y0 + 0.25, y0 + 0.75),
        (z0 + 0.5, z0 + 1.5),
    );
    let BooleanResult::Body(b) = subtract(&cube, &cutter, Tol::witness()).unwrap() else {
        panic!("die subtract is a body");
    };
    assert!(b.contacts.vv.is_empty() && b.contacts.a_on_b.is_empty());
    assert_eq!(
        mass_properties(&b.body, Tol::witness()).unwrap().volume,
        0.875
    );
    b.body
}

#[test]
fn r6_pocketed_dice_kiss_e2e() {
    let d1 = die(0.0, 0.0, 0.0);
    let d2 = die(1.0, 1.0, 1.0);
    let BooleanResult::Body(assembly) = union(&d1, &d2, Tol::witness()).unwrap() else {
        panic!("kiss union is a body");
    };
    // 3′ gate: green with carried contacts, red without.
    assert_eq!(assembly.contacts.vv.len(), 1, "one corner kiss");
    assert_eq!(
        validate_pseudomanifold(&assembly.body, &assembly.contacts, Tol::witness()),
        Ok(())
    );
    let withheld =
        validate_pseudomanifold(&assembly.body, &ContactRecords::default(), Tol::witness())
            .unwrap_err();
    assert!(
        withheld
            .iter()
            .all(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "{withheld:?}"
    );
    // Structure + exact oracles (the parts=2 discipline: shell count
    // is asserted HERE, not inferred from admesh part count).
    assert_eq!(assembly.body.shells().count(), 2);
    let m = mass_properties(&assembly.body, Tol::witness()).unwrap();
    assert_eq!(m.volume, 1.75, "exact volume oracle");
    assert_eq!(m.surface_area, 14.0, "exact area oracle");
    // Tessellate → watertight mesh → exact mesh volume → STL.
    let mesh = mesh::tessellate(&assembly.body, 1e-2, Tol::witness()).unwrap();
    check_mesh(&mesh).unwrap();
    let v = signed_volume(&mesh);
    assert!((v - 1.75).abs() < 1e-9, "mesh volume {v}");
    let dir = std::env::temp_dir().join("review_m3_pr6_stl");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("pocketed_dice_kiss.stl");
    let mut file = std::fs::File::create(&path).unwrap();
    stl::write_binary(&mesh, &stl::BinaryOptions::default(), &mut file).unwrap();
    eprintln!("R6 STL written: {}", path.display());
}
