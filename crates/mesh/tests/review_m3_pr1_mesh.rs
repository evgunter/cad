//! M3 PR 1 adversarial review, mesh-consumer half: tessellation must
//! refuse (typed, never skip) a body carrying null-edge scaffolding,
//! and must work again once the scaffold is consumed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tolerance};
use mesh::{TessellateError, tessellate};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, MevSite, NewVertexSide};
use geom_core::Tol;

/// TARGET 2 (tessellation door): a null strut on an extruded prism
/// closes tessellation with the typed NullScaffoldEdge - the mesh walk
/// must not silently drop the zero-length edge - and killing the
/// scaffold reopens it with the identical mesh.
#[test]
fn tessellate_refuses_null_scaffold_typed() {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(1.0, 1.0),
            Point2::new(0.0, 1.0),
        ])],
    )
    .validate(Tol::witness())
    .unwrap();
    let mut body: Body<f64> = extrude(&profile, Extrusion::Distance(1.0)).unwrap().body;
    let mesh_before = tessellate(&body, 0.01).unwrap();
    let v = body.vertices().next().map(|(k, _)| k).unwrap();
    let he = body.get_vertex(v).unwrap().emanating.unwrap();
    let created = body
        .mev_null(MevSite::Fan { he1: he, he2: he }, NewVertexSide::Above)
        .unwrap();
    let err = tessellate(&body, 0.01).unwrap_err();
    assert!(
        matches!(err, TessellateError::NullScaffoldEdge { edge } if edge == created.edge),
        "tessellation must refuse the null edge typed, got {err:?}"
    );
    // Consuming the scaffold reopens tessellation. NOTE (pinned): the
    // mev_null/kev roundtrip is NOT byte-neutral - the site loop's
    // anchor rotates, so one face triangulates with a different (still
    // valid) diagonal. Positions and triangle counts are identical;
    // replay determinism (same history -> same mesh) is unaffected.
    body.kev(created.he_plus).unwrap();
    let mesh_after = tessellate(&body, 0.01).unwrap();
    assert_eq!(
        format!("{:?}", mesh_before.positions),
        format!("{:?}", mesh_after.positions)
    );
    assert_eq!(
        mesh::validate::triangle_count(&mesh_before),
        mesh::validate::triangle_count(&mesh_after)
    );
    mesh::validate::check_mesh(&mesh_after).unwrap();
}
