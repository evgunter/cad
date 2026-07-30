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

/// The plane-crossing roots of a **conic** carrier over its span
/// (M1 fix, M5 PR 5 review): the signed distance along the carrier is
/// the sinusoid `d(θ) = D + R·cos(θ − φ)` with `D = (center − q)·n̂`,
/// `R·cos φ = s_u·(û·n̂)`, `R·sin φ = s_v·(v̂·n̂)` (`s_u/s_v` the
/// semi-axes — `r/r` for a circle). Unlike a line, a conic edge can
/// cross the plane an EVEN number of times between same-side
/// endpoints (the belly case) or once beyond an ON endpoint — so
/// crossing detection is **root-based, endpoint-verdict-free**:
///
/// 1. `split_conic_belly_graze` — margin `R − |D|` (meters): Negative
///    ⇒ the carrier never meets the split plane — no crossing;
///    Positive ⇒ two distinct roots `φ ± acos(−D/R)`; Zero ⇒ the
///    plane grazes the carrier's extremum — ONE (double) root, whose
///    insertion (if in-span) leaves a same-side ON contact for the
///    sector adjudication (rule (b)'s AOA/BOB — the established graze
///    net); in-band ⇒ typed escalation (F6).
/// 2. Each root, translated into `[t₀, t₀ + τ)`, is classified
///    against the span by `split_conic_crossing_root` — the two
///    margins `(t − t₀)·meter` and `(t₁ − t)·meter` (meters at the
///    conservative minor-semi-axis meter): both Positive ⇒ a genuine
///    interior crossing (returned for insertion); any Zero ⇒ the root
///    sits at an existing endpoint vertex (the ON-endpoint belly case
///    — the vertex sweep already recorded it, nothing to insert); any
///    Negative ⇒ outside the span; in-band ⇒ typed escalation (the
///    crossing grazes an edge end — an ill-conditioned operand/plane
///    pair).
/// 3. Two interior roots are ordered ascending through
///    `split_conic_root_order` (never a raw comparison); ε-close
///    roots collapse to one insertion (Zero) or escalate (in-band).
///
/// Downstream, `split_edge`'s own interiority trilean and child
/// certification re-verify every insertion — this lane proposes,
/// never silently commits.
fn conic_crossing_roots<T: Decide>(
    carrier: &geom_curves::Curve3<T>,
    t0: T,
    t1: T,
    plane: &SplitPlane<T>,
    band: Band,
) -> Result<Option<Result<Vec<T>, geom_core::Indeterminate>>, ()> {
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
    // 1. Does the sinusoid reach zero at all — and how many roots?
    let both_roots = match decide("split_conic_belly_graze", r - d0.abs(), band) {
        Ok(Sign::Negative) => return Ok(None),
        Ok(Sign::Positive) => true,
        // Graze: the double root, processed once (processing both
        // would split twice at coincident parameters and escalate on
        // the second interiority check — same refusal, worse site).
        Ok(Sign::Zero) => false,
        Err(diag) => return Ok(Some(Err(diag))),
    };
    let phi = b.atan2(a);
    // Clamped acos (rounding can push the ratio a hair outside ±1 at
    // the graze boundary; min/max are Real lattice ops).
    let arg = ((T::zero() - d0) / r)
        .min(T::one())
        .max(T::zero() - T::one());
    let delta = arg.acos();
    let tau = T::tau();
    // The conservative meter (radians → meters): the minor semi-axis.
    let meter = s_v;
    let mut roots: Vec<T> = Vec::with_capacity(2);
    let candidates: [Option<T>; 2] = if both_roots {
        [Some(phi + delta), Some(phi - delta)]
    } else {
        [Some(phi + delta), None]
    };
    for c in candidates.into_iter().flatten() {
        let t = t0 + (c - t0).reduce_periodic(tau);
        let mut interior = true;
        for margin in [(t - t0) * meter, (t1 - t) * meter] {
            match decide("split_conic_crossing_root", margin, band) {
                Ok(Sign::Positive) => {}
                // At an endpoint vertex (already swept) or outside
                // the span: nothing to insert for this root.
                Ok(Sign::Zero | Sign::Negative) => {
                    interior = false;
                    break;
                }
                Err(diag) => return Ok(Some(Err(diag))),
            }
        }
        if interior {
            roots.push(t);
        }
    }
    // Ascending insertion order, decided through the trilean door
    // (never a raw scalar comparison).
    if roots.len() == 2 {
        match decide(
            "split_conic_root_order",
            (roots[1] - roots[0]) * meter,
            band,
        ) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Negative) => roots.swap(0, 1),
            // Coincident-but-both-interior roots: a graze the graze
            // margin called Positive — insert once.
            Ok(Sign::Zero) => {
                roots.truncate(1);
            }
            Err(diag) => return Ok(Some(Err(diag))),
        }
    }
    Ok(Some(Ok(roots)))
}

/// Crossing insertion (M1 fix — even-crossing completeness):
///
/// - **Line carriers** keep the M3 rule BIT-IDENTICALLY: a crossing
///   iff the cached endpoint verdicts are strictly opposite (complete
///   for lines — the affine distance has one root, and an ON endpoint
///   IS that root), split at the exact interpolation
///   `t = t₀ + (t₁ − t₀)·d₁/(d₁ − d₂)` (comparison-free; safe: the
///   strict-opposite-signs decision bounds the denominator away from
///   zero).
/// - **Conic carriers** use the root-based lane
///   ([`conic_crossing_roots`]), INDEPENDENT of endpoint verdicts:
///   same-side endpoints with a belly crossing the plane twice get
///   BOTH crossing vertices; an ON endpoint with one interior
///   crossing gets it; grazes land as single ON contacts for the
///   rule (b) adjudication. Two roots split the parent then its
///   trailing child (ascending — the second root lives on the child's
///   span).
///
/// Both lanes are certified by `split_edge` itself (the
/// `split_edge_param_interior` trilean + full child re-certification —
/// the honest lane the raw book formula lacks). New vertices are ON
/// **by construction** (declared coincidence): their verdicts are
/// cached without re-measuring.
pub(super) fn insert_crossings<T: Decide>(
    body: &mut Body<T>,
    plane: &SplitPlane<T>,
    sides: &mut SecondaryMap<VertexKey, PlaneSide>,
    on_vertices: &mut Vec<VertexKey>,
) -> Result<(), SplitReduceError> {
    let band = geom_core::Band::linear()?;
    // Snapshot: splitting adds edges; only operand edges can cross (a
    // split child's remaining crossing is handled through the parent's
    // precomputed root list below).
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
        let curve = match body.get_curve_geom(edge.curve) {
            Some(CurveGeom::Certified(c)) => c.clone(),
            _ => return Err(SplitReduceError::ScaffoldingOperand { edge: edge_key }),
        };
        let (t0, t1) = curve.params();
        let roots: Vec<T> = match conic_crossing_roots(curve.carrier(), t0, t1, plane, band) {
            Ok(None) => continue,
            Ok(Some(Ok(roots))) => roots,
            Ok(Some(Err(diag))) => {
                return Err(SplitReduceError::CrossingEscalated {
                    edge: edge_key,
                    diag,
                });
            }
            Err(()) => {
                // Line lane (the M3 path, bit-identical): endpoint
                // verdicts strictly opposite ⇒ one interpolated root.
                let crossing = matches!(
                    (sides[u], sides[v]),
                    (PlaneSide::Above, PlaneSide::Below) | (PlaneSide::Below, PlaneSide::Above)
                );
                if !crossing {
                    continue;
                }
                let dist = |body: &Body<T>, vk: VertexKey| -> Option<T> {
                    let p = *body.get_point(body.get_vertex(vk)?.point)?;
                    Some((p - plane.origin).dot(plane.normal))
                };
                let (Some(d1), Some(d2)) = (dist(body, u), dist(body, v)) else {
                    return Err(SplitReduceError::CorruptOperand { vertex: u });
                };
                vec![t0 + (t1 - t0) * (d1 / (d1 - d2))]
            }
        };
        // Insert ascending: the first split leaves the parent with
        // [t₀, tₐ] and mints the trailing child [tₐ, t₁]; the second
        // root (if any) lives on that CHILD.
        let mut target = edge_key;
        for t in roots {
            // Any refusal here (in practice the certification lane's
            // strict-row ResidualExceeded) gets the crossing site
            // attached; the typed Euler error stays nested whole.
            let created = body.split_edge(target, t).map_err(|source| {
                SplitReduceError::CrossingInsertion {
                    edge: target,
                    endpoints: (u, v),
                    source,
                }
            })?;
            sides.insert(created.vertex, PlaneSide::On);
            on_vertices.push(created.vertex);
            target = created.new_edge;
        }
    }
    Ok(())
}
