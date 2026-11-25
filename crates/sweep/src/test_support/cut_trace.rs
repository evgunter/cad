//! **What a boolean built through a sphere cut-in leaves of the cut**:
//! nothing. A cut-in divides a sphere face along a meridian of its own
//! chart, and the output merge glues the cut away with the pieces it
//! divided, so the result carries no edge the operands' faces did not
//! (DESIGN `[topology]`'s seam clause, read on the result at rest).
//!
//! The reading, of any built result against its two operands:
//!
//! - no edge divides two faces worn on one surface record, so the face
//!   count is the maximal one, but for an edge lying on one an operand
//!   divides its own faces by (a run's authored cuts, which a torus,
//!   cylinder or cone run keeps by design: the sweep README's "Walls:
//!   one per run", C12.5; read at its two ends and its middle, on an
//!   operand's closed-form edge). A group the output merge skipped
//!   leaves such an edge, so it is read there;
//! - every edge with one sphere face on both sides lies on that face's
//!   own chart seam, the half-meridian at azimuth zero (the plane of its
//!   frame's axis and `u_ref`, on `u_ref`'s side), which is where the
//!   operand's wrap edge runs: its two ends and the middle of its curve,
//!   so a pole-to-pole meridian on another half, whose ends are the
//!   poles and lie on every meridian, is read too.
//!
//! Not absorbed: the sweep suites' `cap_rims` reads rims and the faces
//! across them, not seams; their `orient` reads positions off charts,
//! not which edges a face keeps.

use geom_core::Point3;
use topo::{Body, BooleanResult};

/// Each edge of `body` that divides two faces worn on one surface record:
/// its carrier and the middle of its span.
fn dividing_edges(body: &Body<f64>) -> Vec<(geom::Curve3<f64>, f64)> {
    body.edges()
        .filter_map(|(e, d)| {
            let sides = topo::readback::edge_sides(body, e).ok()?;
            let ((f1, f2), (k1, k2)) = (sides.faces(), sides.surfaces());
            if f1 == f2 || k1 != k2 {
                return None;
            }
            let curve = body.get_curve_geom(d.curve)?.certified()?;
            let (t0, t1) = curve.params();
            Some((curve.carrier().clone(), 0.5 * (t0 + t1)))
        })
        .collect()
}

/// The reading (module docs) of `r`, a built result of `operands`;
/// `label` names the op in the panic. An empty result has nothing to
/// read.
///
/// # Panics
///
/// Where any clause fails.
pub fn assert_no_cut_trace(label: &str, r: &BooleanResult<f64>, operands: [&Body<f64>; 2]) {
    let Some(out) = r.body() else {
        return;
    };
    let body = &*out.body;
    let authored: Vec<_> = operands.into_iter().flat_map(dividing_edges).collect();
    let on_authored = |p: Point3<f64>| {
        authored.iter().any(|(carrier, near)| {
            carrier.param_near(p, *near).is_some_and(|t| {
                (carrier.eval(t) - p).norm() <= 1e-9 * (p - Point3::origin()).norm().max(1.0)
            })
        })
    };
    let point = |v| topo::readback::vertex_point(body, v).unwrap();
    for (e, _) in body.edges() {
        let sides = topo::readback::edge_sides(body, e).unwrap();
        let (f1, f2) = sides.faces();
        let (k1, k2) = sides.surfaces();
        if k1 != k2 {
            continue;
        }
        let (u, v) = body.edge_vertices(e).unwrap();
        let curve = body
            .get_curve_geom(body.get_edge(e).unwrap().curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap();
        let (t0, t1) = curve.params();
        let mid = curve.carrier().mid_point(t0, t1);
        if f1 != f2 {
            assert!(
                [point(u), mid, point(v)].into_iter().all(on_authored),
                "{label}: {e:?} divides two faces worn on one record on no edge an operand \
                 divides its own by, so the result is not maximal"
            );
            continue;
        }
        let Some(&geom::Surface::Sphere { frame, radius }) = body.face_surface(f1) else {
            continue;
        };
        let (c, axis, seam) = (frame.origin(), frame.axis(), frame.u_ref());
        let across = axis.cross(seam);
        for p in [point(u), mid, point(v)] {
            let d = p - c;
            assert!(
                across.dot(d).abs() <= 1e-9 * radius.max(1.0) && seam.dot(d) >= -1e-9 * radius,
                "{label}: {e:?} lies inside its sphere face off the face's own seam: a point at \
                 {p:?}"
            );
        }
    }
}
