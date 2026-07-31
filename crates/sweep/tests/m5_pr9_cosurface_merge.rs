//! M5 PR 9 (C12.5): `merge_coplanar_faces` generalizes to cosurface
//! (same-surface) merging for curved faces — the same never-numeric
//! ladder, now kind-agnostic on its hard rungs.
//!
//! The fixture is the extruded disc: its two half-wall faces share
//! ONE cylinder surface key (the cosurface run) and meet along the
//! two meridian struts — exactly the shape the boolean zip
//! manufactures when a through cut re-splits a wall. Before PR 9 the
//! op skipped curved same-key neighbors by design; now the structural
//! rung merges them: one strut dies, the other is KEPT as the
//! periodic run's parameterization cut (the classical seam-form),
//! and the result re-passes the full tier-3 gate.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tolerance};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;

fn disc_cylinder() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: Point2::new(-0.5, 0.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: Point2::new(0.5, 0.0),
            bulge: 1.0,
        },
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0)).unwrap().body
}

#[test]
fn same_key_wall_pieces_remerge_structurally() {
    let mut body = disc_cylinder();
    let faces_before = body.faces().count();
    let walls_before: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom_surfaces::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(walls_before.len(), 2, "the disc extrudes two half-walls");

    let out = body.merge_coplanar_faces().expect("the cosurface merge");
    assert!(
        out.skipped.is_empty(),
        "nothing may be silently skipped: {:?}",
        out.skipped
    );
    assert_eq!(out.groups.len(), 1, "one cosurface run: {:?}", out.groups);
    let g = &out.groups[0];
    assert_eq!(g.absorbed.len(), 1, "one wall absorbed into the other");
    // ONE strut dies (the kev); the other is KEPT as the periodic
    // run's parameterization cut — the classical seam-form (killing
    // it would close the chart with no cut). No ring is minted.
    assert_eq!(
        g.killed_edges.len(),
        1,
        "exactly one strut dies: {:?}",
        g.killed_edges
    );
    assert!(g.rings_made.is_empty(), "no ring on a curved survivor");
    assert_eq!(body.faces().count(), faces_before - 1);

    // The merged body is still a closed solid — and tier-3 valid
    // (dihedrals, pcurve caches, volume: the whole geometric gate).
    topo::validate_closed(&body).expect("the merged body stays closed");
    if let Err(errs) = topo::validate_geometric(&body) {
        panic!("the merged body must stay tier-3 valid: {errs:?}");
    }
    let walls_after = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom_surfaces::Surface::Cylinder { .. })
            )
        })
        .count();
    assert_eq!(walls_after, 1, "one maximal wall face");
}

#[test]
fn distinct_key_curved_neighbors_stay_unmerged() {
    // The never-numeric rule, curved edition: two DISTINCT-key,
    // value-different cylinder walls (a filleted block's fillet wall
    // next to nothing cylindrical) never merge — and, sharper, a body
    // with two distinct-key curved walls that are not even adjacent
    // reports no groups at all. The ladder infers nothing from
    // values; with no shared key, no shared source, and no declared
    // pair, the op is a no-op.
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: Point2::new(-0.5, 0.0),
            bulge: 0.3,
        },
        ProfileVertex {
            pos: Point2::new(0.5, 0.0),
            bulge: 0.3,
        },
    ]);
    // Two shallow arcs: a lens. The two wall faces lie on DIFFERENT
    // cylinders (different centers), so no rung licenses a merge.
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    let mut body = extrude(&profile, Extrusion::Distance(1.0)).unwrap().body;
    let out = body.merge_coplanar_faces().expect("no-op merge");
    assert!(out.groups.is_empty(), "no rung licenses: {:?}", out.groups);
    assert!(out.skipped.is_empty());
}
