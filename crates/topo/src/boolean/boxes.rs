//! Certified-conservative boxes for the reduction sweep's candidate
//! generation (M5 PR 8, C10): planar-face and `Line`-edge boxes from
//! **vertex extents** plus the sweep pad. The tree these feed only
//! ever PRUNES — every box here must contain everything the exact
//! predicates could accept for its entity, and [`sweep_pad`] is sized
//! for exactly that (its derivation below). No Q1 predicate runs on a
//! box; classification is untouched (`reduce` module docs).
//!
//! An allowlisted [`geom_core::Bounds`] seam (ratified 2026-07-29 —
//! see geom-core `real.rs`, Bounds scope rule; the C10 tree is the
//! subdivision driver): coordinates enter as `[lo(), hi()]` brackets,
//! poison flows to the poison box, which never prunes.

use bvh::Aabb;
use geom_core::{Band, Bounds, Decide, Point3};

use super::BooleanError;
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, LoopKey, VertexKey};

/// The sweep's box pad in meters — what candidate generation must add
/// so pruning can never lose an accepted pair. Derivation (each term
/// against the sweep's accept conditions in `reduce`):
///
/// - an event point classifies ON the face plane / boundary only
///   within `band.zero()` (`bool_vertex_face_side`, `contfp`'s
///   `bool_contact_*` sites) — one `zero` for the point-to-face gap;
/// - vertices sit on their carriers only up to attachment-time
///   certification (`Certificate::max_residual ≤ ε`, the same run
///   tolerance the linear band's `zero` is built from) — one more
///   `zero` per side for vertex-extent honesty;
/// - `band.escalate()` on top dominates every remaining f64 slop
///   (crossing-point interpolation and `eval` rounding are
///   session-box-scale ulps, orders below it) and keeps near-boundary
///   escalation zones inside candidate range.
///
/// The sum is deliberately generous — a bigger pad only admits more
/// candidates (conservative direction); it never changes any answer
/// (the differential suite pins that).
pub(super) fn sweep_pad(band: Band) -> f64 {
    band.escalate() + 2.0 * band.zero()
}

fn corrupt(what: &'static str) -> BooleanError {
    BooleanError::ClassificationInvariant { what }
}

/// The face's certified box, padded. Planar faces: the vertex-extent
/// box (straight edges make the polygon lie in its vertex hull).
/// Cylinder faces (M5 PR 9): the vertex hull is NOT a superset (the
/// wall's belly bulges past its chords), so the box is the full
/// cylinder slab over the face's axial range — axial extents from the
/// boundary edges' own certified boxes (the axial coordinate is
/// linear, so its face extremes lie on the boundary), widened by the
/// full radius in EVERY coordinate (deliberately loose; a bigger box
/// only admits candidates — the conservative direction).
///
/// **KNOWN GAP — `Nurbs` faces reach the vertex-hull fallthrough and
/// the hull is NOT a superset for them.** This doc used to say other
/// curved kinds "fall through to the vertex hull only if they reach
/// here at all (the operand gate refuses them first)". That is no
/// longer true: [`super::reduce::gate_planar`] admits
/// `Surface::Nurbs(_)`, and a patch's interior bulges past the hull of
/// its boundary vertices exactly as a sphere's belly bulges past its
/// poles — the same reasoning the `Cylinder` and `Sphere` arms above
/// are written to respect. So this function can currently hand
/// `Bvh::overlapping` a box that prunes a pair the exact predicate
/// would have examined, which breaks the conservative-superset
/// contract (see the module docs) and turns what should be
/// `curved_face_arm`'s typed refusal into a silently wrong result.
/// [`edge_box`] already guards the matching case by poisoning its
/// `Nurbs` arm; this one has no such arm. The sound constructor exists
/// unused: `geom_surfaces::boxes::nurbs_surface_aabb` returns the
/// control-net hull. Reachable today via **Union** with a lofted
/// operand (`boolean_op_with` refuses non-planar operands only for
/// Subtract and Intersect); no corpus document exercises it, which is
/// why it has not bitten. `tests/m5_pr8_bvh_diff.rs` cannot catch it —
/// every scenario there is built from planar bricks.
pub(super) fn face_box<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    pad: f64,
) -> Result<Aabb, BooleanError> {
    let f = body.get_face(face).ok_or(corrupt("face box: face lost"))?;
    if let Some(geom_surfaces::Surface::Cylinder {
        origin,
        axis,
        radius,
        ..
    }) = body.get_surface(f.surface)
    {
        let (origin, axis) = (*origin, *axis);
        let radius = radius.hi();
        let mut h_min = f64::INFINITY;
        let mut h_max = f64::NEG_INFINITY;
        let mut walk = |lk: LoopKey| -> Result<(), BooleanError> {
            let l = body.get_loop(lk).ok_or(corrupt("face box: loop lost"))?;
            let LoopBoundary::Cycle { first } = l.boundary else {
                return Ok(());
            };
            for he in body
                .loop_cycle(first)
                .ok_or(corrupt("face box: unwalkable loop"))?
            {
                let ek = body
                    .get_half_edge(he)
                    .ok_or(corrupt("face box: half-edge lost"))?
                    .edge;
                let eb = edge_box(body, ek, 0.0)?;
                for &(x, y, z) in &[
                    (eb.min_x, eb.min_y, eb.min_z),
                    (eb.max_x, eb.max_y, eb.max_z),
                    (eb.min_x, eb.min_y, eb.max_z),
                    (eb.min_x, eb.max_y, eb.min_z),
                    (eb.max_x, eb.min_y, eb.min_z),
                    (eb.min_x, eb.max_y, eb.max_z),
                    (eb.max_x, eb.min_y, eb.max_z),
                    (eb.max_x, eb.max_y, eb.min_z),
                ] {
                    let h = (x - origin.x.lo()) * axis.x.lo()
                        + (y - origin.y.lo()) * axis.y.lo()
                        + (z - origin.z.lo()) * axis.z.lo();
                    h_min = h_min.min(h);
                    h_max = h_max.max(h);
                }
            }
            Ok(())
        };
        walk(f.outer)?;
        for &ring in &f.rings {
            walk(ring)?;
        }
        if !(h_min.is_finite() && h_max.is_finite()) {
            return Ok(Aabb::poison());
        }
        // Pad the axial range too (interval-lane bracket slop is
        // dominated by the pad; conservative direction).
        h_min -= pad;
        h_max += pad;
        let along = |c: f64, a: f64| (c + a * h_min, c + a * h_max);
        let (x0, x1) = along(origin.x.lo(), axis.x.lo());
        let (y0, y1) = along(origin.y.lo(), axis.y.lo());
        let (z0, z1) = along(origin.z.lo(), axis.z.lo());
        return Ok(Aabb {
            min_x: x0.min(x1) - radius,
            min_y: y0.min(y1) - radius,
            min_z: z0.min(z1) - radius,
            max_x: x0.max(x1) + radius,
            max_y: y0.max(y1) + radius,
            max_z: z0.max(z1) + radius,
        }
        .padded(pad));
    }
    // Sphere faces (M5 S13): the vertex hull is NOT a superset (a
    // band's belly bulges past its poles and seam arcs), so the box is
    // the whole ball `center ± r` — deliberately loose, the
    // conservative direction (a bigger box only admits candidates).
    if let Some(&geom_surfaces::Surface::Sphere { center, radius, .. }) =
        body.get_surface(f.surface)
    {
        let r = radius.hi();
        return Ok(Aabb {
            min_x: center.x.lo() - r,
            min_y: center.y.lo() - r,
            min_z: center.z.lo() - r,
            max_x: center.x.hi() + r,
            max_y: center.y.hi() + r,
            max_z: center.z.hi() + r,
        }
        .padded(pad));
    }
    let mut points: Vec<Point3<T>> = Vec::new();
    let mut push_loop = |lk: LoopKey| -> Result<(), BooleanError> {
        let l = body.get_loop(lk).ok_or(corrupt("face box: loop lost"))?;
        match l.boundary {
            LoopBoundary::Empty { vertex } => points.push(vertex_point(body, vertex)?),
            LoopBoundary::Cycle { first } => {
                for he in body
                    .loop_cycle(first)
                    .ok_or(corrupt("face box: unwalkable loop"))?
                {
                    let start = body
                        .get_half_edge(he)
                        .ok_or(corrupt("face box: half-edge lost"))?
                        .start;
                    points.push(vertex_point(body, start)?);
                }
            }
        }
        Ok(())
    };
    push_loop(f.outer)?;
    for &ring in &f.rings {
        push_loop(ring)?;
    }
    Ok(Aabb::from_points(points)
        .unwrap_or_else(Aabb::poison)
        .padded(pad))
}

/// The edge's certified box, padded. `Line` carriers: the two
/// endpoints (the locus is the chord up to certification residual —
/// inside the pad). Conic carriers (M5 PR 9): the FULL conic's
/// center-±-amplitude box hulled with the endpoints — a superset of
/// any arc of it (an arc's belly bulges past its chord; the full-turn
/// box is deliberately loose, the conservative direction). `Nurbs`
/// carriers land on the poison box (never prunes) until a rung-3
/// operand gate admits them with a control-hull box.
pub(super) fn edge_box<T: Decide + Bounds>(
    body: &Body<T>,
    edge: EdgeKey,
    pad: f64,
) -> Result<Aabb, BooleanError> {
    let e = body.get_edge(edge).ok_or(corrupt("edge box: edge lost"))?;
    let start_of = |he| -> Result<Point3<T>, BooleanError> {
        let vk = body
            .get_half_edge(he)
            .ok_or(corrupt("edge box: half-edge lost"))?
            .start;
        vertex_point(body, vk)
    };
    let (a, b) = (start_of(e.he_plus)?, start_of(e.he_minus)?);
    let vertex_box = Aabb::from_points([a, b]).unwrap_or_else(Aabb::poison);
    let carrier = body
        .get_curve_geom(e.curve)
        .and_then(crate::null::CurveGeom::certified)
        .map(geom_brep::EdgeCurve::carrier);
    let conic = match carrier {
        Some(geom_curves::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        }) => Some((*center, *axis, *radius, *radius, *u_ref)),
        Some(geom_curves::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        }) => Some((*center, *axis, *major, *minor, *u_ref)),
        Some(geom_curves::Curve3::Nurbs(_)) => return Ok(Aabb::poison()),
        Some(geom_curves::Curve3::Line { .. }) | None => None,
    };
    let boxed = match conic {
        None => vertex_box,
        Some((c, axis, sa, sb, u_ref)) => {
            // Per-coordinate amplitude |û_i|·a + |v̂_i|·b of the full
            // conic (v̂ = axis × û), hulled with the endpoint box.
            let v_ref = axis.cross(u_ref);
            let reach = |ui: f64, vi: f64| ui.abs() * sa.hi() + vi.abs() * sb.hi();
            let rx = reach(u_ref.x.hi(), v_ref.x.hi());
            let ry = reach(u_ref.y.hi(), v_ref.y.hi());
            let rz = reach(u_ref.z.hi(), v_ref.z.hi());
            let full = Aabb {
                min_x: c.x.lo() - rx,
                min_y: c.y.lo() - ry,
                min_z: c.z.lo() - rz,
                max_x: c.x.hi() + rx,
                max_y: c.y.hi() + ry,
                max_z: c.z.hi() + rz,
            };
            Aabb {
                min_x: full.min_x.min(vertex_box.min_x),
                min_y: full.min_y.min(vertex_box.min_y),
                min_z: full.min_z.min(vertex_box.min_z),
                max_x: full.max_x.max(vertex_box.max_x),
                max_y: full.max_y.max(vertex_box.max_y),
                max_z: full.max_z.max(vertex_box.max_z),
            }
        }
    };
    Ok(boxed.padded(pad))
}

fn vertex_point<T: Decide + Bounds>(
    body: &Body<T>,
    v: VertexKey,
) -> Result<Point3<T>, BooleanError> {
    body.get_vertex(v)
        .and_then(|vd| body.get_point(vd.point))
        .copied()
        .ok_or(corrupt("face/edge box: vertex point lost"))
}
