//! Interval-lane extrusion (feature `interval`): the new geometry paths
//! instantiated at the certified scalar — exact dyadic fixtures decide
//! definitely from point enclosures, build end-to-end through the public
//! op, and pass all three validation tiers.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::EdgeGeometry;
use geom_core::{Bounds, Interval, Point2, Real, Tolerance};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{validate, validate_closed, validate_geometric};
use geom_core::Tol;

fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(Interval::from_f64(x), Interval::from_f64(y))
}

#[test]
fn interval_l_profile_extrudes_and_passes_all_tiers() {
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 1.0),
        p2(1.0, 1.0),
        p2(1.0, 2.0),
        p2(0.0, 2.0),
    ]);
    let vp = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let t = extrude(&vp, Extrusion::Distance(Interval::from_f64(1.5))).unwrap();
    assert_eq!(validate(&t.body), Ok(()));
    assert_eq!(validate_closed(&t.body), Ok(()));
    assert_eq!(validate_geometric(&t.body), Ok(()));
    assert_eq!(t.body.vertices().count(), 12);
    assert_eq!(t.body.edges().count(), 18);
    assert_eq!(t.body.faces().count(), 8);
    // Corner joins upgraded at the interval scalar too.
    for &edge in &t.strut_edges[0] {
        let curve = t.body.get_edge(edge).unwrap().curve;
        assert!(matches!(
            t.body
                .get_curve_geom(curve)
                .unwrap()
                .certified()
                .unwrap()
                .description(),
            EdgeGeometry::Intersection { .. }
        ));
    }
    // A raised corner's enclosure is the exact point it was built from
    // (dyadic data throughout).
    let strut = t.strut_edges[0][0];
    let he = t.body.get_edge(strut).unwrap().he_plus;
    let top_v = t.body.half_edge_end(he).unwrap();
    let p = t
        .body
        .get_point(t.body.get_vertex(top_v).unwrap().point)
        .unwrap();
    assert_eq!(p.z.lo(), 1.5);
    assert_eq!(p.z.hi(), 1.5);
}

#[test]
fn interval_disc_extrudes_a_shared_cylinder() {
    // The arc paths (circle carriers, cylinder side patches, cosurface
    // sharing) at the interval scalar. (Was `#[ignore]`d behind PR 3
    // review B1 — `norm`/`distance` squared straddling-zero enclosures
    // through plain interval multiplication and poisoned the decoration
    // through `sqrt`; geom-core's tight per-component `powi(2)` fix
    // landed in the PR 3 fix pass and this build now certifies clean.)
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(-0.5, 0.0), Interval::from_f64(1.0)),
        ProfileVertex::new(p2(0.5, 0.0), Interval::from_f64(1.0)),
    ]);
    let vp = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let t = extrude(&vp, Extrusion::Distance(Interval::from_f64(1.0))).unwrap();
    assert_eq!(validate(&t.body), Ok(()));
    assert_eq!(validate_closed(&t.body), Ok(()));
    assert_eq!(validate_geometric(&t.body), Ok(()));
    // One shared cylinder + two cap planes.
    assert_eq!(t.body.surfaces().count(), 3);
    let k0 = t.body.get_face(t.side_faces[0][0]).unwrap().surface;
    let k1 = t.body.get_face(t.side_faces[0][1]).unwrap().surface;
    assert_eq!(k0, k1);
    assert!(matches!(
        t.body.get_surface(k0).unwrap(),
        Surface::Cylinder { .. }
    ));
}
