//! M5 PR 9 (C12.2): the second-order sector lane, end to end on the
//! tangent-plane graze — the configuration whose first-order sector
//! data ties everywhere along the contact ruling.
//!
//! The fixture is PR 5's extruded disc (two half-walls sharing one
//! cylinder surface, meridian ruling edges at (±0.5, 0)) split by the
//! VERTICAL TANGENT plane x = 0.5: the plane contains the meridian
//! edge, both rim arcs depart exactly in-plane (the first-order
//! departure trilean honestly returns On), and the wall faces' local
//! normals are plane-parallel at the ON vertices (the old
//! `TangencyUnsupported` door). Before PR 9 this refused at first
//! order; now the second-order lane classifies it — the arcs curve
//! definitely toward Below, the walls definitely bend off the plane —
//! and whatever the pipeline ultimately refuses on is a DOWNSTREAM
//! honest verdict (the one-sided graze has no two-sided split), never
//! the first-order tie.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tolerance, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;
use topo::splitting::{SplitPlane, SplitReduceError, split};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The PR 5 disc: two half-circle arcs (bulge 1), radius 0.5 —
/// extrudes to a cylinder whose two wall faces share ONE cylinder
/// surface, with meridian ruling edges at (±0.5, 0).
fn cylinder_body() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(-0.5, 0.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(0.5, 0.0),
            bulge: 1.0,
        },
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0)).unwrap().body
}

#[test]
fn the_tangent_graze_resolves_past_first_order() {
    // The old doors must NOT fire: no TangencyUnsupported (the wall
    // definitely bends off the plane — tangent_sector_osculation),
    // no ConsecutiveOnSectors (the arcs' in-plane departures resolve
    // at second order — tangent_sector_order2), no SliverSector on
    // those predicates. What remains is the documented one-sided
    // graze residue, refused DOWNSTREAM (the degenerate section /
    // finish net) — a tangent plane cannot two-side a convex body.
    use geom_core::k_stats::{start_verdict_log, take_verdict_log};
    let body = cylinder_body();
    let plane = SplitPlane {
        origin: Point3::new(0.5, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    };
    start_verdict_log();
    let out = split(&body, &plane);
    let v = take_verdict_log();
    // The second-order lane ran, by name (telemetry from birth).
    for name in ["tangent_sector_order2", "tangent_sector_osculation"] {
        assert!(
            v.iter().any(|x| x.predicate == name),
            "{name} never reached the funnel (recorded: {:?})",
            v.iter()
                .map(|x| x.predicate)
                .collect::<std::collections::BTreeSet<_>>()
        );
    }
    // The pipeline runs ALL the way to the documented one-sided
    // tangency net: the TangentIntersection section chords were
    // minted and CERTIFIED (the jet gate ran inside a real body),
    // the section polygon then honestly bounds zero area — the
    // degenerate side has no material. The graze refuses THERE, with
    // the recourse, never at the first-order tie.
    let err = out.expect_err("a tangent plane cannot two-side a convex body");
    let msg = format!("{err}");
    assert!(
        msg.contains("zero area") && msg.contains("one-sided tangency"),
        "the graze must reach the degenerate-section net: {msg}"
    );
    assert!(msg.contains("declare the coincidence"), "{msg}");
}

#[test]
fn an_off_ruling_tangent_plane_still_grazes_honestly() {
    // The same tangent plane rotated to touch mid-wall (azimuth π/2,
    // the ruling (0, 0.5, z) — no vertex there): the graze root lands
    // mid-arc, gets inserted as a double-root vertex, and the
    // neighborhood at THAT vertex ties at first order the same way.
    // Pin: never the first-order refusals.
    let body = cylinder_body();
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.5, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
    };
    match split(&body, &plane) {
        Err(topo::splitting::SplitError::Reduce(
            e @ (SplitReduceError::TangencyUnsupported { .. }
            | SplitReduceError::ConsecutiveOnSectors { .. }),
        )) => {
            panic!("second order owns the graze: {e}")
        }
        Ok(_) | Err(_) => {}
    }
}
