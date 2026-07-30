//! Reduction step (ch. 14 `splitgenerate`, re-derived): the operand
//! gate (F5 → the C5 table, M5 PR 5), the cached vertex-vs-plane
//! trilean sweep (F6), and crossing insertion through the certified
//! `split_edge` lane — with the conic crossing-root lane for
//! circle/ellipse carriers.

use geom_core::{Band, Decide, Sign};
use slotmap::SecondaryMap;

use super::{PlaneSide, SplitPlane, SplitReduceError};
use crate::body::Body;
use crate::entity::VertexKey;
use crate::null::CurveGeom;
use crate::validate::decide;

/// The operand gate — the M3 planar gate refactored onto THE C5
/// dispatch table (M5 PR 5, C12.1): a face passes iff the split
/// pipeline executes its `(kind × plane)` arm — `Plane` (the M2/M3
/// seam, bit-identical) and `Cylinder` (the rung-2 conic lane landed
/// here). Every other kind refuses typed, **citing its rung routing**
/// (`CurvedBooleanUnsupported` retires per arm, never wholesale).
/// Edge carriers: `Line`/`Circle`/`Ellipse` pass (the crossing and
/// split lanes handle all three); `Nurbs` refuses typed (rung 3,
/// unimplemented until SSI). Pre-existing null scaffolding refuses as
/// ever.
pub(super) fn gate_operand<T: Decide>(body: &Body<T>) -> Result<(), SplitReduceError> {
    for (face_key, face) in body.faces() {
        let Some(surface) = body.get_surface(face.surface) else {
            return Err(SplitReduceError::CurvedBooleanUnsupported {
                face: face_key,
                kind: geom_brep::SurfaceKind::Nurbs,
            });
        };
        let kind = geom_brep::SurfaceKind::of(surface);
        match kind {
            geom_brep::SurfaceKind::Plane | geom_brep::SurfaceKind::Cylinder => {}
            geom_brep::SurfaceKind::Cone
            | geom_brep::SurfaceKind::Sphere
            | geom_brep::SurfaceKind::Torus
            | geom_brep::SurfaceKind::Nurbs => {
                return Err(SplitReduceError::CurvedBooleanUnsupported {
                    face: face_key,
                    kind,
                });
            }
        }
    }
    for (edge_key, edge) in body.edges() {
        match body.get_curve_geom(edge.curve) {
            Some(CurveGeom::Certified(curve)) => match curve.carrier() {
                geom_curves::Curve3::Line { .. }
                | geom_curves::Curve3::Circle { .. }
                | geom_curves::Curve3::Ellipse { .. } => {}
                geom_curves::Curve3::Nurbs(_) => {
                    return Err(SplitReduceError::CurvedEdgeUnsupported { edge: edge_key });
                }
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

/// The crossing parameter of a **conic** carrier against the plane
/// (M5 PR 5): the signed distance along the carrier is the sinusoid
/// `d(θ) = D + R·cos(θ − φ)` with
/// `D = (center − q)·n̂`, `R·cos φ = s_u·(û·n̂)`, `R·sin φ = s_v·(v̂·n̂)`
/// (`s_u/s_v` the semi-axes — `r/r` for a circle). Strictly-opposite
/// endpoint verdicts guarantee exactly one root inside `(t₀, t₁)`
/// (≤ 2 roots per period, odd count in the span, span ≤ τ by the
/// winding gate): both branch candidates `φ ± acos(−D/R)` are
/// translated into `[t₀, t₀ + τ)` and the in-span one is selected by
/// the named trilean `split_conic_crossing_root` (margin the span
/// headroom `(t₁ − t₊)` metered at the carrier's conservative meter —
/// radius / minor semi-axis); in-band ⇒ typed escalation (the crossing
/// grazes the edge end: an ill-conditioned operand/plane pair).
/// Downstream, `split_edge`'s own interiority trilean and child
/// certification re-verify whatever is returned — this selection can
/// propose, never silently commit.
fn conic_crossing_param<T: Decide>(
    carrier: &geom_curves::Curve3<T>,
    t0: T,
    t1: T,
    plane: &SplitPlane<T>,
    band: Band,
) -> Result<Result<T, geom_core::Indeterminate>, ()> {
    let (center, axis, u_ref, s_u, s_v) = match *carrier {
        geom_curves::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => (center, axis, u_ref, radius, radius),
        geom_curves::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => (center, axis, u_ref, major, minor),
        geom_curves::Curve3::Line { .. } | geom_curves::Curve3::Nurbs(_) => return Err(()),
    };
    let v_ref = axis.cross(u_ref);
    let d0 = (center - plane.origin).dot(plane.normal);
    let a = u_ref.dot(plane.normal) * s_u;
    let b = v_ref.dot(plane.normal) * s_v;
    let r = (a * a + b * b).sqrt();
    let phi = b.atan2(a);
    // Clamped acos (rounding can push the ratio a hair outside ±1 when
    // the plane grazes the extremum; min/max are Real lattice ops).
    let arg = ((T::zero() - d0) / r)
        .min(T::one())
        .max(T::zero() - T::one());
    let delta = arg.acos();
    let tau = T::tau();
    // Both branch roots, translated into [t0, t0 + τ).
    let tp = t0 + (phi + delta - t0).reduce_periodic(tau);
    let tm = t0 + (phi - delta - t0).reduce_periodic(tau);
    // The conservative meter (radians → meters): the minor semi-axis.
    let meter = s_v;
    match decide("split_conic_crossing_root", (t1 - tp) * meter, band) {
        Ok(Sign::Positive) => Ok(Ok(tp)),
        Ok(Sign::Zero | Sign::Negative) => Ok(Ok(tm)),
        Err(diag) => Ok(Err(diag)),
    }
}

/// Crossing insertion: every edge whose cached endpoint verdicts are
/// strictly opposite is split at the carrier's plane-crossing
/// parameter — for line carriers the exact interpolation
/// `t = t₀ + (t₁ − t₀)·d₁/(d₁ − d₂)` (comparison-free; safe: the
/// strict-opposite-signs decision bounds the denominator away from
/// zero), for conic carriers the closed-form sinusoid root
/// ([`conic_crossing_param`]) — in both lanes certified by
/// `split_edge` itself (the `split_edge_param_interior` trilean + full
/// child re-certification — the honest lane the raw book formula
/// lacks). The new vertex is ON **by construction** (declared
/// coincidence): its verdict is cached without re-measuring.
pub(super) fn insert_crossings<T: Decide>(
    body: &mut Body<T>,
    plane: &SplitPlane<T>,
    sides: &mut SecondaryMap<VertexKey, PlaneSide>,
    on_vertices: &mut Vec<VertexKey>,
) -> Result<(), SplitReduceError> {
    let band = geom_core::Band::linear()?;
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
        let curve = match body.get_curve_geom(edge.curve) {
            Some(CurveGeom::Certified(c)) => c.clone(),
            _ => return Err(SplitReduceError::ScaffoldingOperand { edge: edge_key }),
        };
        let (t0, t1) = curve.params();
        let t = match conic_crossing_param(curve.carrier(), t0, t1, plane, band) {
            Ok(Ok(t)) => t,
            Ok(Err(diag)) => {
                return Err(SplitReduceError::CrossingEscalated {
                    edge: edge_key,
                    diag,
                });
            }
            Err(()) => {
                // Line lane (the M3 path, bit-identical): signed
                // distances of the two endpoints, linear along the
                // carrier, root at s = d₁/(d₁ − d₂) of the [t₀, t₁]
                // span. d₁ belongs to start(he_plus) = the t₀ endpoint
                // (forward contract).
                let dist = |body: &Body<T>, vk: VertexKey| -> Option<T> {
                    let p = *body.get_point(body.get_vertex(vk)?.point)?;
                    Some((p - plane.origin).dot(plane.normal))
                };
                let (Some(d1), Some(d2)) = (dist(body, u), dist(body, v)) else {
                    return Err(SplitReduceError::CorruptOperand { vertex: u });
                };
                t0 + (t1 - t0) * (d1 / (d1 - d2))
            }
        };
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
