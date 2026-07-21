//! Reduction step (ch. 14 `splitgenerate`, re-derived): the planar
//! gate (F5), the cached vertex-vs-plane trilean sweep (F6), and
//! crossing insertion through the certified `split_edge` lane.

use geom_core::{Band, Decide, Sign};
use slotmap::SecondaryMap;

use super::{PlaneSide, SplitPlane, SplitReduceError};
use crate::body::Body;
use crate::entity::VertexKey;
use crate::null::CurveGeom;
use crate::validate::decide;

/// F5: refuse any non-`Plane` face, any non-`Line` edge carrier, and
/// any pre-existing null scaffolding — before a single classification.
pub(super) fn gate_planar<T: Decide>(body: &Body<T>) -> Result<(), SplitReduceError> {
    for (face_key, face) in body.faces() {
        match body.get_surface(face.surface) {
            Some(geom_surfaces::Surface::Plane { .. }) => {}
            _ => return Err(SplitReduceError::CurvedBooleanUnsupported { face: face_key }),
        }
    }
    for (edge_key, edge) in body.edges() {
        match body.get_curve_geom(edge.curve) {
            Some(CurveGeom::Certified(curve)) => match curve.carrier() {
                geom_curves::Curve3::Line { .. } => {}
                _ => return Err(SplitReduceError::CurvedEdgeUnsupported { edge: edge_key }),
            },
            _ => return Err(SplitReduceError::ScaffoldingOperand { edge: edge_key }),
        }
    }
    Ok(())
}

/// F6: classify every vertex against the plane through the
/// `split_vertex_side` trilean (margin = signed distance in meters,
/// linear band), caching the verdict per vertex — one predicate site,
/// one evaluation per vertex (the book recomputes per incident edge).
/// In-band ⇒ typed [`SplitReduceError::SliverVertex`], never a snap.
pub(super) fn classify_vertices<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    band: Band,
) -> Result<(SecondaryMap<VertexKey, PlaneSide>, Vec<VertexKey>), SplitReduceError> {
    let mut sides = SecondaryMap::new();
    let mut on_vertices = Vec::new();
    for (vertex_key, vertex) in body.vertices() {
        let p = *body
            .get_point(vertex.point)
            .ok_or(SplitReduceError::CorruptOperand { vertex: vertex_key })?;
        let margin = (p - plane.origin).dot(plane.normal);
        let side = match decide("split_vertex_side", margin, band) {
            Ok(Sign::Negative) => PlaneSide::Below,
            Ok(Sign::Positive) => PlaneSide::Above,
            Ok(Sign::Zero) => {
                on_vertices.push(vertex_key);
                PlaneSide::On
            }
            Err(diag) => {
                return Err(SplitReduceError::SliverVertex {
                    vertex: vertex_key,
                    diag,
                });
            }
        };
        sides.insert(vertex_key, side);
    }
    Ok((sides, on_vertices))
}

/// Crossing insertion: every edge whose cached endpoint verdicts are
/// strictly opposite is split at the interpolated carrier parameter
/// `t = t₀ + (t₁ − t₀)·d₁/(d₁ − d₂)` — comparison-free evaluation
/// (safe: the strict-opposite-signs decision bounds the denominator
/// away from zero), certified by `split_edge` itself (the
/// `split_edge_param_interior` trilean + full child re-certification —
/// the honest lane the raw book formula lacks). The new vertex is ON
/// **by construction** (declared coincidence): its verdict is cached
/// without re-measuring.
pub(super) fn insert_crossings<T: Decide>(
    body: &mut Body<T>,
    plane: &SplitPlane<T>,
    sides: &mut SecondaryMap<VertexKey, PlaneSide>,
    on_vertices: &mut Vec<VertexKey>,
) -> Result<(), SplitReduceError> {
    // Snapshot: splitting adds edges; only operand edges can cross
    // (each child keeps one old endpoint and gains an ON endpoint).
    let snapshot: Vec<_> = body.edges().map(|(k, e)| (k, e.clone())).collect();
    for (edge_key, edge) in snapshot {
        let start = |body: &Body<T>, he| {
            body.get_half_edge(he)
                .map(|h: &crate::entity::HalfEdge| h.start)
        };
        let u = start(body, edge.he_plus).ok_or(SplitReduceError::Euler(
            crate::euler::EulerOpError::StaleKey {
                key: crate::entity::EntityId::Edge(edge_key),
            },
        ))?;
        let v = start(body, edge.he_minus).ok_or(SplitReduceError::Euler(
            crate::euler::EulerOpError::StaleKey {
                key: crate::entity::EntityId::Edge(edge_key),
            },
        ))?;
        let crossing = matches!(
            (sides[u], sides[v]),
            (PlaneSide::Above, PlaneSide::Below) | (PlaneSide::Below, PlaneSide::Above)
        );
        if !crossing {
            continue;
        }
        // Evaluation lane (no comparisons): signed distances of the two
        // endpoints, linear along the line carrier, root at
        // s = d₁/(d₁ − d₂) of the [t₀, t₁] span. d₁ belongs to
        // start(he_plus) = the t₀ endpoint (forward contract).
        let dist = |body: &Body<T>, vk: VertexKey| -> Option<T> {
            let p = *body.get_point(body.get_vertex(vk)?.point)?;
            Some((p - plane.origin).dot(plane.normal))
        };
        let (Some(d1), Some(d2)) = (dist(body, u), dist(body, v)) else {
            return Err(SplitReduceError::CorruptOperand { vertex: u });
        };
        let curve = match body.get_curve_geom(edge.curve) {
            Some(CurveGeom::Certified(c)) => *c,
            _ => return Err(SplitReduceError::ScaffoldingOperand { edge: edge_key }),
        };
        let (t0, t1) = curve.params();
        let t = t0 + (t1 - t0) * (d1 / (d1 - d2));
        // Any refusal here (in practice the certification lane's
        // strict-row ResidualExceeded) gets the crossing site attached;
        // the typed Euler error stays nested whole.
        let created =
            body.split_edge(edge_key, t)
                .map_err(|source| SplitReduceError::CrossingInsertion {
                    edge: edge_key,
                    endpoints: (u, v),
                    source,
                })?;
        sides.insert(created.vertex, PlaneSide::On);
        on_vertices.push(created.vertex);
    }
    Ok(())
}
