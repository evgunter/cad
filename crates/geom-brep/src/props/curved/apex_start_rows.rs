//! **A cone loop through the apex reads alike from every start**
//! ([`super::cone_loop_winding`]'s "Through the apex"): the loop a box
//! leaves on the wrap-tipped cone where the box's side holds the wrap
//! seam, passing the apex once between the seam vertex's two copies.

use core::f64::consts::{FRAC_PI_2, TAU};

use geom::Curve3;
use geom_core::{Band, Point3, Tol, Vec3};

use super::{AtApex, cone_loop_winding, require_cone_loop_vertices_distinct};
use crate::props::LoopEdge;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn line(from: Point3<f64>, to: Point3<f64>, ids: (u32, u32)) -> LoopEdge<f64> {
    let dir = to - from;
    LoopEdge::hand_built(
        Curve3::line(from, dir.normalize()),
        0.0,
        dir.norm(),
        true,
        ids.0,
        ids.1,
    )
}

fn circle(
    depth: f64,
    radius: f64,
    span: (f64, f64),
    forward: bool,
    ids: (u32, u32),
) -> LoopEdge<f64> {
    LoopEdge::hand_built(
        Curve3::Circle(geom::Circle {
            frame: geom::Frame {
                center: Point3::new(0.0, 0.0, -depth),
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            radius,
        }),
        span.0,
        span.1,
        forward,
        ids.0,
        ids.1,
    )
}

/// **The seam vertex's two copies, the apex between them, read from
/// every start**: the 30° cone, apex at the origin and its nappe below,
/// less the quadrant between azimuths `0` and `π/2` above depth `0.45`,
/// its wrap seam at azimuth `0`. The loop is the cut's arc `v₄ → v₅`, the
/// rulings `v₅ → apex → v₄`, the seam `v₄ → v₂`, the base circle, and
/// the seam back. Rotated to start at each of its six edges, the
/// winding's sweep is one and the vertices read distinct, the seam's
/// copies a whole turn apart: no start leaves the apex between them.
#[test]
fn a_loop_through_the_apex_reads_alike_from_every_start() {
    let t = 30f64.to_radians().tan();
    let (d1, d2) = (0.45, 1.0);
    let apex = Point3::origin();
    let v4 = Point3::new(d1 * t, 0.0, -d1);
    let v5 = Point3::new(0.0, d1 * t, -d1);
    let v2 = Point3::new(d2 * t, 0.0, -d2);
    let edges = [
        circle(d1, d1 * t, (0.0, FRAC_PI_2), true, (4, 5)),
        line(v5, apex, (5, 0)),
        line(apex, v4, (0, 4)),
        line(v4, v2, (4, 2)),
        circle(d2, d2 * t, (0.0, TAU), false, (2, 2)),
        line(v2, v4, (2, 4)),
    ];
    let axis = Vec3::new(0.0, 0.0, 1.0);
    let mut sweeps = Vec::new();
    for start in 0..edges.len() {
        let turned: Vec<_> = edges[start..]
            .iter()
            .chain(&edges[..start])
            .cloned()
            .collect();
        let w = cone_loop_winding(apex, axis, &turned, AtApex::AlongGenerator, band())
            .unwrap_or_else(|e| panic!("start {start}: {e:?}"));
        assert_eq!(
            require_cone_loop_vertices_distinct(apex, axis, &w, band()),
            Ok(()),
            "start {start}"
        );
        sweeps.push(w.total);
    }
    assert!(
        sweeps.iter().all(|s| (s - sweeps[0]).abs() < 1e-12),
        "the sweep moves with the start: {sweeps:?}"
    );
}
