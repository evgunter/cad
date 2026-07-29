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

/// The face's vertex-extent box, padded: every vertex of the outer
/// loop and every ring (lone vertices included). Contains the face's
/// closed point set — straight edges make the polygon lie in its
/// vertex hull — and, with the pad, every point the exact predicates
/// could still accept against it.
pub(super) fn face_box<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    pad: f64,
) -> Result<Aabb, BooleanError> {
    let f = body.get_face(face).ok_or(corrupt("face box: face lost"))?;
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

/// The edge's vertex-extent box, padded: the two endpoint points
/// (post-gate the carrier is a `Line`, so the edge's locus is the
/// chord between them up to certification residual — inside the pad).
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
    Ok(Aabb::from_points([a, b])
        .unwrap_or_else(Aabb::poison)
        .padded(pad))
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
