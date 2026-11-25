//! **A promoted kind does not depend on which parameter the file ran
//! along the axis.** `tests/fixtures/promotion/`: a half cylinder and a
//! half cone frustum, each a rational B-spline wall of two quarter-arc
//! spans, closed by planes; the `_T` twin states the same wall with `u`
//! and `v` swapped (its arcs along `v`, its rulings along `u`). Both
//! twins import, and their wall promotes to the same kind — the cylinder
//! estimator reads its axis off the net either way, and the cone
//! estimator proposes no cone whose half-angle the band cannot tell from
//! zero, so a cylinder never reaches the cone arm as a degenerate cone
//! with its apex past the session box.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn fixture(name: &str) -> String {
    let path = format!(
        "{}/tests/fixtures/promotion/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// The kind of the one curved face `name` imports with.
fn wall_kind(name: &str) -> geom::SurfaceKind {
    let body = match import_step(&fixture(name), &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { body, .. }) => body,
        Ok(_) => panic!("{name} imports as a solid"),
        Err(e) => panic!("{name} imports: {}", e.render()),
    };
    let kinds: Vec<_> = body
        .faces()
        .filter_map(|(fk, _)| body.face_surface(fk).ok().map(geom::Surface::kind))
        .filter(|k| *k != geom::SurfaceKind::Plane)
        .collect();
    assert_eq!(kinds.len(), 1, "{name}: one curved wall: {kinds:?}");
    kinds[0]
}

/// **The half cylinder imports as a cylinder whichever way its net runs.**
#[test]
fn the_half_cylinder_promotes_to_a_cylinder_either_way_round() {
    for name in ["a_half_cyl.step", "a_half_cyl_T.step"] {
        assert_eq!(wall_kind(name), geom::SurfaceKind::Cylinder, "{name}");
    }
}

/// **The half cone imports as a cone whichever way its net runs.**
#[test]
fn the_half_cone_promotes_to_a_cone_either_way_round() {
    for name in ["a_half_cone.step", "a_half_cone_T.step"] {
        assert_eq!(wall_kind(name), geom::SurfaceKind::Cone, "{name}");
    }
}
