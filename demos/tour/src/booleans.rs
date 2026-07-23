//! The tour's ONLY boolean call sites — a deliberately thin,
//! centralized wrapper over `topo::{union, subtract}` (M3 PR 5) so
//! that any API shift in PR 5's in-flight review pass is a one-file
//! adaptation here.
//!
//! Also holds the tier-3 posture pass for boolean outputs: seam edges
//! carry chord-line descriptions (the documented PR 3 description
//! gap), so tier 3 at rest runs on a clone whose edges are upgraded to
//! `Intersection` descriptions through the certified `set_edge_curve`
//! path — the honest upgrade op itself is a PR 6 obligation.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use topo::{Body, BooleanError, BooleanResult, EdgeCurveSpec, EdgeGeometry};

/// A ∪* B — refusals surface to the caller; every result goes through
/// the scene builders' exact-volume oracle before it ships.
pub fn try_union(a: &Body<f64>, b: &Body<f64>) -> Result<BooleanResult<f64>, BooleanError> {
    topo::union(a, b)
}

/// A ∖* B (same posture as [`try_union`]).
pub fn try_subtract(a: &Body<f64>, b: &Body<f64>) -> Result<BooleanResult<f64>, BooleanError> {
    topo::subtract(a, b)
}

// The chord-normalization workaround (`normalize_edges_to_chords`)
// that used to live here is RETIRED: PR 5's extrude-operand
// description remap makes raw extrude output boolean-consumable,
// proven end-to-end by the PR 5.5 review's die e2e
// (crates/stl/tests/review_m3_pr55_e2e.rs).

/// Tier-3 posture pass (module docs): re-describes every edge of a
/// planar-faced body as the `Intersection` of its two adjacent faces'
/// surfaces (witness at the chord midpoint), through the certified
/// `set_edge_curve` path. Mirrors the kernel test suite's documented
/// posture for boolean outputs.
pub fn upgrade_edges_to_intersections(body: &mut Body<f64>) {
    let edges: Vec<_> = body.edges().map(|(k, e)| (k, e.clone())).collect();
    for (edge_key, edge) in edges {
        let face_surface = |body: &Body<f64>, he| {
            let he_data = body.get_half_edge(he).unwrap();
            let loop_data = body.get_loop(he_data.parent_loop).unwrap();
            body.get_face(loop_data.face).unwrap().surface
        };
        let s1 = face_surface(body, edge.he_plus);
        let s2 = face_surface(body, edge.he_minus);
        let start = body.get_half_edge(edge.he_plus).unwrap().start;
        let end = body.half_edge_end(edge.he_plus).unwrap();
        let p0 = *body
            .get_point(body.get_vertex(start).unwrap().point)
            .unwrap();
        let p1 = *body.get_point(body.get_vertex(end).unwrap().point).unwrap();
        let mut spec = EdgeCurveSpec::line_between(p0, p1);
        spec.description = EdgeGeometry::Intersection {
            s1,
            s2,
            witness: p0.lerp(p1, 0.5),
        };
        body.set_edge_curve(edge_key, spec).unwrap();
    }
}
