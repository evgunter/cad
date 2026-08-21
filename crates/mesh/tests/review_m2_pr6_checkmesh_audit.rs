//! M2 PR 6 adversarial review — watertightness oracles and an audit of
//! `check_mesh` itself (assignment 2): hand-built broken meshes must be
//! rejected typed; degenerate configurations (coarse washer, concentric
//! slit annuli, seam welds) must pass the full battery.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{axis_y, ball, check_mesh_acceptance, cone, donut, p2, validated, washer, wedge};
use geom_core::Point3;
use mesh::validate::{MeshError, check_mesh};
use mesh::{FacePatch, Mesh, tessellate};
use profile::ProfileLoop;
use profile::RawLoop;
use sweep::{Revolution, revolve};
use geom_core::Tol;

/// A hand-built mesh from raw positions and one patch of triangles
/// (face key borrowed from a real body — check_mesh never reads it).
fn hand_mesh(positions: Vec<Point3<f64>>, triangles: Vec<[u32; 3]>) -> Mesh {
    let body = ball();
    let fk = body.faces().next().unwrap().0;
    Mesh {
        positions,
        patches: vec![FacePatch {
            face: fk,
            triangles,
        }],
        boundaries: Vec::new(),
    }
}

fn tetra_positions() -> Vec<Point3<f64>> {
    vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, 1.0),
    ]
}

/// Outward-wound tetrahedron (reference valid mesh).
fn tetra_tris() -> Vec<[u32; 3]> {
    vec![[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]]
}

#[test]
fn survives_checkmesh_accepts_valid_tetrahedron() {
    let m = hand_mesh(tetra_positions(), tetra_tris());
    assert_eq!(check_mesh(&m), Ok(()));
    assert!(mesh::validate::signed_volume(&m) > 0.0);
}

#[test]
fn survives_checkmesh_rejects_open_fan() {
    // Drop one tetra face: three edges become boundary edges.
    let mut t = tetra_tris();
    t.pop();
    let m = hand_mesh(tetra_positions(), t);
    assert!(matches!(
        check_mesh(&m),
        Err(MeshError::BoundaryEdge { .. })
    ));
}

#[test]
fn survives_checkmesh_rejects_flipped_winding() {
    let mut t = tetra_tris();
    t[3] = [2, 1, 3]; // flip one face
    let m = hand_mesh(tetra_positions(), t);
    assert!(matches!(
        check_mesh(&m),
        Err(MeshError::MismatchedWinding { .. })
    ));
}

#[test]
fn survives_checkmesh_rejects_duplicated_triangle() {
    let mut t = tetra_tris();
    t.push(t[0]);
    let m = hand_mesh(tetra_positions(), t);
    assert!(matches!(
        check_mesh(&m),
        Err(MeshError::NonManifoldEdge { .. })
    ));
}

#[test]
fn survives_checkmesh_rejects_degenerate_sliver() {
    let mut t = tetra_tris();
    t.push([1, 1, 2]); // repeated index
    let m = hand_mesh(tetra_positions(), t);
    assert!(matches!(
        check_mesh(&m),
        Err(MeshError::DegenerateTriangle { .. })
    ));
}

#[test]
fn survives_checkmesh_rejects_index_out_of_range() {
    let t = vec![[0, 1, 9]];
    let m = hand_mesh(tetra_positions(), t);
    assert!(matches!(
        check_mesh(&m),
        Err(MeshError::IndexOutOfRange { index: 9 })
    ));
}

#[test]
fn survives_checkmesh_rejects_t_junction() {
    // Square 0-1-2-3 plus midpoint 4 on edge (1,2): left side one
    // triangle uses the full edge, right side splits it — every
    // T-junction shows up as odd/boundary edge use.
    let pos = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(1.0, 0.5, 0.0),
    ];
    let t = vec![[0, 1, 2], [0, 2, 3], [1, 4, 0], [4, 2, 0]];
    let m = hand_mesh(pos, t);
    assert!(check_mesh(&m).is_err(), "T-junction must not validate");
}

#[test]
fn survives_checkmesh_rejects_double_collapsed_fan_fallout() {
    // The pole-fan argument drops exactly ONE triangle per collapsed
    // side. If two adjacent CDT triangles both collapsed (the case the
    // walk's 2-entries-per-junction structure forbids), dropping both
    // leaves a hole — verify check_mesh would catch that fallout, so
    // the validator is a genuine backstop for the structural argument.
    //
    // The "exactly one per side" premise is CONDITIONAL — it needs the
    // interior grid to separate the walk's two pole entries, which
    // `curved::pole_columns` is what supplies (issue #678). This row is
    // unaffected: it hand-builds the fallout mesh rather than meshing a
    // body, so its own case stays exactly as true as it was. What #678
    // changes is that the validator is NOT the only backstop any more —
    // `tessellate` never runs it, so the same argument now also carries
    // a `debug_assert` over the emitted patch (D2 addendum row 5).
    let mut t = tetra_tris();
    t.pop();
    t.pop(); // two adjacent faces gone: a slit of boundary edges
    let m = hand_mesh(tetra_positions(), t);
    assert!(matches!(
        check_mesh(&m),
        Err(MeshError::BoundaryEdge { .. })
    ));
}

// ---- degenerate real-body configurations ----------------------------

#[test]
fn survives_washer_at_very_coarse_delta() {
    // π/4 chord cap ⇒ 8-gon rims; slit annuli at their coarsest.
    let body = washer();
    let pi = core::f64::consts::PI;
    for delta in [0.9, 0.45] {
        check_mesh_acceptance(&body, delta, Some((3.0 * pi, 12.0 * pi)));
    }
}

#[test]
fn survives_concentric_slit_annuli() {
    // U-profile revolved fully: the bottom plane carries one wide slit
    // annulus, the top plane two concentric slit annuli (separate
    // faces, same plane) — parity and seam cancellation stress.
    let lp = ProfileLoop::polygon([
        p2(1.0, 0.0),
        p2(4.0, 0.0),
        p2(4.0, 1.0),
        p2(3.0, 1.0),
        p2(3.0, 0.5),
        p2(2.0, 0.5),
        p2(2.0, 1.0),
        p2(1.0, 1.0),
    ]);
    let body = revolve(&validated(vec![lp]), axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    for delta in [0.7, 0.08] {
        check_mesh_acceptance(&body, delta, None);
    }
}

#[test]
fn survives_seam_weld_no_bitwise_duplicate_positions() {
    // A welded seam means the u=0 and u=2π traversals share mesh
    // vertex ids — an unwelded seam would mint bitwise-equal duplicate
    // positions. Assert every position is bitwise-unique.
    for body in [ball(), cone(), donut(), washer(), wedge()] {
        let mesh = tessellate(&body, 0.07, Tol::witness()).unwrap();
        let mut seen = std::collections::HashSet::new();
        for p in &mesh.positions {
            let key = (p.x.to_bits(), p.y.to_bits(), p.z.to_bits());
            assert!(
                seen.insert(key),
                "bitwise-duplicate mesh position {p:?} (unwelded seam?)"
            );
        }
    }
}

#[test]
fn survives_seam_segments_shared_by_both_wall_sides() {
    // Probe the weld directly: every boundary polyline segment of the
    // donut (all of whose edges are seam meridians or rims) is an edge
    // of exactly two kept triangles.
    let body = donut();
    let mesh = tessellate(&body, 0.06, Tol::witness()).unwrap();
    let mut uses: std::collections::HashMap<(u32, u32), u32> = std::collections::HashMap::new();
    for patch in &mesh.patches {
        for tri in &patch.triangles {
            for k in 0..3 {
                let (a, b) = (tri[k], tri[(k + 1) % 3]);
                *uses.entry((a.min(b), a.max(b))).or_insert(0) += 1;
            }
        }
    }
    for bp in &mesh.boundaries {
        for w in bp.points.windows(2) {
            assert_eq!(
                uses.get(&(w[0].min(w[1]), w[0].max(w[1]))),
                Some(&2),
                "seam segment not consumed by both traversals"
            );
        }
    }
}
