//! **The axis channel's rows** (`topo::AxisSource`): the per-component
//! token of `docs/AXIS-DECLARATION-DESIGN.md`, its attach door, and
//! every kernel door that carries, composes or drops it.
//!
//! The fixture is the unit brick with two faces relabelled onto axis-
//! bearing surfaces — a cylinder and a cone — so every row runs on two
//! axis rows on different keys, and a door that carried one and lost
//! the other is visible.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::brick;
use geom::Surface;
use geom_core::{Affine3, Point3, Tol, Vec3};
use topo::{
    AxisAttachError, AxisPlacement, AxisRecord, AxisSource, Body, FaceSurface, SurfaceKey,
    graft_disjoint, transform_rigid,
};

fn aside() -> Affine3<f64> {
    Affine3::translation(Vec3::new(3.0, 0.0, 0.0))
}

fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.5, 0.5, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.25,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn cone() -> Surface<f64> {
    Surface::Cone {
        apex: Point3::new(0.5, 0.5, -1.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        half_angle: 0.25,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The unit brick with two faces on axis-bearing surfaces, and those
/// two surface keys, cylinder first.
fn two_axis_faces() -> (Body<f64>, [SurfaceKey; 2]) {
    let mut b = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let faces: Vec<_> = b.faces().map(|(k, _)| k).take(2).collect();
    let cyl = b
        .set_face_surface_stranding_for_tests(
            faces[0],
            FaceSurface::New {
                surface: cylinder(),
                sense: true,
            },
        )
        .unwrap();
    let cone = b
        .set_face_surface_stranding_for_tests(
            faces[1],
            FaceSurface::New {
                surface: cone(),
                sense: true,
            },
        )
        .unwrap();
    (b, [cyl, cone])
}

/// Both keys stamped from ONE recipe-level axis — the configuration the
/// channel exists to tell apart from two cylinders that merely measure
/// coaxial.
fn stamped() -> (Body<f64>, [SurfaceKey; 2], AxisSource) {
    let (mut b, keys) = two_axis_faces();
    let axis = AxisSource::from_lowered(b"the-document's-axis-D");
    for k in keys {
        b.set_surface_axis_source(k, axis.clone()).unwrap();
    }
    (b, keys, axis)
}

/// The attach door takes an axis-bearing surface, and refuses a stale
/// key and a plane — both caller bugs, neither recorded.
#[test]
fn the_attach_door_takes_axes_and_refuses_planes_and_stale_keys() {
    let (mut b, [cyl, stale_after]) = two_axis_faces();
    let axis = AxisSource::from_lowered(b"D");
    assert_eq!(b.surface_axis_record(cyl), None, "opt-in: nothing attached");
    b.set_surface_axis_source(cyl, axis.clone()).unwrap();
    assert_eq!(b.surface_axis_source(cyl), Some(&axis));

    let plane = b
        .surfaces()
        .find(|(_, s)| matches!(s, Surface::Plane { .. }))
        .map(|(k, _)| k)
        .unwrap();
    assert_eq!(
        b.set_surface_axis_source(plane, axis.clone()),
        Err(AxisAttachError::NoAxisOnKind)
    );
    assert_eq!(
        b.surface_axis_record(plane),
        None,
        "a refusal records nothing"
    );

    // Re-surfacing the cone's face orphans its surface: that key is
    // stale from here on.
    let face = b
        .faces()
        .find(|(_, f)| f.surface == stale_after)
        .map(|(k, _)| k)
        .unwrap();
    b.set_face_surface_stranding_for_tests(
        face,
        FaceSurface::New {
            surface: cone(),
            sense: true,
        },
    )
    .unwrap();
    assert!(b.get_surface(stale_after).is_none());
    assert_eq!(
        b.set_surface_axis_source(stale_after, axis),
        Err(AxisAttachError::StaleKey)
    );
}

/// **The ratified staleness table, as token equality** — each row of
/// `docs/AXIS-DECLARATION-DESIGN.md`'s table on two tokens of one
/// recipe-level axis, built by hand. This is the token's algebra only:
/// which `(node, index)` a real placement stamps is the recipe layer's
/// choice, above this crate, so the table on carriers stamped by the
/// placement door is `editor-core`'s
/// `eval::wire::place_tests::the_staleness_table_through_the_placement_door`.
#[test]
fn the_staleness_table_is_token_equality() {
    let d = AxisSource::from_lowered(b"D");
    // Neither placed: equal.
    assert_eq!(d, d.clone());
    // Both placed by one node and map: equal.
    assert_eq!(d.placed(7, 0), d.placed(7, 0));
    // One placed: differs, and names the placement that broke it.
    let moved = d.placed(7, 0);
    assert_ne!(d, moved);
    assert!(d.same_base(&moved), "one axis, moved: the stale case");
    assert_eq!(moved.placements(), &[AxisPlacement { node: 7, index: 0 }]);
    // Both placed, by different chains — another node, another pattern
    // placement, or the same two placements in another order: differs.
    assert_ne!(d.placed(7, 0), d.placed(8, 0));
    assert_ne!(d.placed(7, 0), d.placed(7, 1));
    assert_ne!(d.placed(7, 0).placed(8, 0), d.placed(8, 0).placed(7, 0));
    // Two unrelated axes differ at the base, placed alike or not.
    let e = AxisSource::from_lowered(b"E");
    assert_ne!(d.placed(7, 0), e.placed(7, 0));
    assert!(!d.same_base(&e));
}

/// The base stays out of `Debug`; the chain is printed.
#[test]
fn debug_prints_the_chain_and_not_the_base() {
    let shown = format!(
        "{:?}",
        AxisSource::from_lowered(b"secret-axis").placed(42, 3)
    );
    assert!(!shown.contains("secret"), "{shown}");
    assert!(shown.contains("<11 bytes>"), "{shown}");
    assert!(
        shown.contains("node: 42") && shown.contains("index: 3"),
        "{shown}"
    );
}

/// **A placement moves the axis, so it clears the row** — to `Cleared`,
/// not to absence — and the recipe layer's re-stamp (spelled here
/// through the public door `compose_placed` uses) discharges it with
/// the composed token.
#[test]
fn a_transform_clears_the_axis_rows_and_the_restamp_composes() {
    let (b, keys, axis) = stamped();
    let mut placed = transform_rigid(&b, &aside(), Tol::witness()).unwrap();
    for k in keys {
        assert_eq!(placed.surface_axis_record(k), Some(&AxisRecord::Cleared));
        assert_eq!(placed.surface_axis_source(k), None);
    }
    for k in keys {
        let src = b.surface_axis_source(k).unwrap().placed(77, 0);
        placed.set_surface_axis_source(k, src).unwrap();
    }
    let [cyl, cone] = keys;
    assert_eq!(
        placed.surface_axis_source(cyl),
        placed.surface_axis_source(cone),
        "both carriers placed by one chain still share the axis"
    );
    assert_eq!(placed.surface_axis_source(cyl), Some(&axis.placed(77, 0)));
}

/// A `Cleared` row survives a second placement and a reversal: a lost
/// re-stamp stays nameable however far the body travels.
#[test]
fn a_cleared_row_survives_a_second_transform_and_a_revert() {
    let (b, keys, _) = stamped();
    let once = transform_rigid(&b, &aside(), Tol::witness()).unwrap();
    let twice = transform_rigid(&once, &aside(), Tol::witness()).unwrap();
    let reverted = twice.revert().unwrap();
    for k in keys {
        assert_eq!(reverted.surface_axis_record(k), Some(&AxisRecord::Cleared));
    }
}

/// A reversal moves no axis — it negates planes only — so the rows
/// ride it verbatim, where N6's `orient` tag flips.
#[test]
fn a_revert_carries_the_axis_rows_verbatim() {
    let (b, keys, axis) = stamped();
    let reverted = b.revert().unwrap();
    for k in keys {
        assert_eq!(reverted.surface_axis_source(k), Some(&axis));
    }
}

/// **The graft carries every row, both arms**: a transplanted
/// description's axis came from where it came from, and a pending
/// re-stamp stays pending. The destination's own rows are untouched.
#[test]
fn a_graft_carries_source_and_cleared_rows() {
    let tol = Tol::witness();
    let (mut dst, dst_keys, axis) = stamped();
    let (src, src_keys, _) = stamped();
    // One source row left pending, one re-stamped: both arms travel.
    let mut src = transform_rigid(&src, &aside(), tol).unwrap();
    let restamped = axis.placed(5, 2);
    src.set_surface_axis_source(src_keys[0], restamped.clone())
        .unwrap();
    let native: Vec<_> = dst.surfaces().map(|(k, _)| k).collect();
    graft_disjoint(&mut dst, &src, tol).unwrap();

    let mut carried: Vec<_> = dst
        .surfaces()
        .filter(|(k, _)| !native.contains(k))
        .filter_map(|(k, _)| dst.surface_axis_record(k).cloned())
        .collect();
    carried.sort_by_key(|r| format!("{r:?}"));
    assert_eq!(
        carried,
        vec![AxisRecord::Cleared, AxisRecord::Source(restamped)],
        "exactly the source body's two rows, one per arm"
    );
    for k in dst_keys {
        assert_eq!(dst.surface_axis_source(k), Some(&axis));
    }
}

/// Re-surfacing a face orphans its old surface, and the orphan door
/// drops the row with it: the old key stops answering.
#[test]
fn the_orphan_door_drops_the_row() {
    let (mut b, [cyl, _], _) = stamped();
    let face = b
        .faces()
        .find(|(_, f)| f.surface == cyl)
        .map(|(k, _)| k)
        .unwrap();
    let fresh = b
        .set_face_surface_stranding_for_tests(
            face,
            FaceSurface::New {
                surface: Surface::Cylinder {
                    origin: Point3::new(0.5, 0.5, 0.0),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    radius: 0.3,
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                },
                sense: true,
            },
        )
        .unwrap();
    assert!(b.get_surface(cyl).is_none(), "the old surface was orphaned");
    assert_eq!(b.surface_axis_record(cyl), None);
    assert_eq!(
        b.surface_axis_record(fresh),
        None,
        "a new description has no axis row until one is attached"
    );
}
