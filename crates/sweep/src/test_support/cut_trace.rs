//! **What a boolean built through a sphere cut-in leaves of the cut**:
//! nothing. A cut-in divides a sphere face along a meridian of its own
//! chart, and the output merge glues the cut away with the pieces it
//! divided, so the result carries no edge the operands' faces did not
//! (DESIGN `[topology]`'s seam clause, read on the result at rest).
//!
//! A cut-in cuts only sphere faces, so the reading is of a built
//! result's sphere faces; a torus, cylinder or cone run keeps its authored
//! cuts by design (the sweep README's "Walls: one per run", C12.5), and
//! the output merge's skip of one is no trace of a cut:
//!
//! - the output merge skipped no group of sphere faces;
//! - no edge divides two sphere faces worn on one surface record, so
//!   their count is the maximal one;
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

use topo::BooleanResult;

/// The reading (module docs) of `r`, a built result; `label` names the
/// op in the panic. An empty result has nothing to read.
///
/// # Panics
///
/// Where any clause fails.
pub fn assert_no_cut_trace(label: &str, r: &BooleanResult<f64>) {
    let Some(out) = r.body() else {
        return;
    };
    let body = &*out.body;
    let on_sphere = |f| matches!(body.face_surface(f), Some(geom::Surface::Sphere { .. }));
    let skipped: Vec<_> = out
        .naming
        .merge_skipped
        .iter()
        .filter(|s| s.faces.iter().any(|&f| on_sphere(f)))
        .collect();
    assert!(
        skipped.is_empty(),
        "{label}: the output merge skipped sphere faces {skipped:?}"
    );
    let point = |v| topo::readback::vertex_point(body, v).unwrap();
    for (e, _) in body.edges() {
        let sides = topo::readback::edge_sides(body, e).unwrap();
        let (f1, f2) = sides.faces();
        let (k1, k2) = sides.surfaces();
        if k1 != k2 || !on_sphere(f1) {
            continue;
        }
        assert_eq!(
            f1, f2,
            "{label}: {e:?} divides two faces worn on one record, so the result is not maximal"
        );
        let Some(&geom::Surface::Sphere { frame, radius }) = body.face_surface(f1) else {
            continue;
        };
        let (u, v) = body.edge_vertices(e).unwrap();
        let curve = body
            .get_curve_geom(body.get_edge(e).unwrap().curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap();
        let (t0, t1) = curve.params();
        let mid = curve.carrier().mid_point(t0, t1);
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
