//! `contfp` — point-in-face with typed ON verdicts (ch. 13's `contfv`
//! extern-out-parameter idiom as a proper sum type): the second-level
//! case codes of the reduction sweep. The point is assumed on the
//! face's plane (the caller's crossing/on-plane decision precedes).
//!
//! Ladder: an ON verdict fires only at a trilean **Zero** (exact within
//! ε — where declared/structural coincidences land, e.g. a crossing
//! point computed from shared geometry); the sliver band escalates
//! typed (F6); definite margins walk on. Interior/exterior then comes
//! from the PR 3 ray-parity trilean ([`point_in_loop`]) over the outer
//! loop and every ring.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::splitting::{LoopContainment, PointInLoopError, point_in_loop};
use crate::validate::decide;

/// The typed `contfp` verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaceContainment {
    /// Strictly outside the face.
    Out,
    /// Strictly within the face interior.
    In,
    /// On the interior of a boundary edge (the edge to split).
    OnEdge(EdgeKey),
    /// Coincident with a boundary vertex.
    OnVertex(VertexKey),
}

/// Typed refusal of [`contfp`].
#[derive(Debug)]
pub enum ContainError {
    /// A margin landed in the sliver band — the pair is
    /// ill-conditioned at this ε.
    Escalated(Indeterminate),
    /// The ray-parity schedule exhausted (every ray grazed).
    RayExhausted,
    /// The face's topology could not be walked.
    Corrupt,
}

impl From<PointInLoopError> for ContainError {
    fn from(e: PointInLoopError) -> Self {
        match e {
            PointInLoopError::Escalated { diag, .. } => Self::Escalated(diag),
            PointInLoopError::RayExhausted { .. } => Self::RayExhausted,
            PointInLoopError::CorruptLoop { .. } => Self::Corrupt,
        }
    }
}

/// **`contfp`** — classifies point `q` (already on the plane of `face`,
/// with unit plane normal `normal`) against the face. Sweep order is
/// deterministic: the boundary pre-pass ([`boundary_pre_pass`] —
/// vertices over ALL loops first, then edge interiors over all loops)
/// decides every ON case before ray parity runs.
///
/// # Errors
///
/// [`ContainError`] — sliver escalations or unwalkable topology.
pub fn contfp<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<FaceContainment, ContainError> {
    let face_data = body.get_face(face).ok_or(ContainError::Corrupt)?;
    let loops: Vec<_> = core::iter::once(face_data.outer)
        .chain(face_data.rings.iter().copied())
        .collect();

    if let Some(on) = boundary_pre_pass(body, &loops, q, EdgeChords::All, band)? {
        return Ok(on);
    }

    // Interior/exterior: inside the outer loop AND outside every ring.
    match point_in_loop(body, face_data.outer, normal, q, band)? {
        LoopContainment::Out => return Ok(FaceContainment::Out),
        LoopContainment::In => {}
        LoopContainment::OnBoundary => {
            return Err(ContainError::Escalated(invalid(
                band,
                "bool_contfp_boundary",
            )));
        }
    }
    for &ring in &face_data.rings {
        match point_in_loop(body, ring, normal, q, band)? {
            LoopContainment::Out => {}
            LoopContainment::In => return Ok(FaceContainment::Out),
            LoopContainment::OnBoundary => {
                return Err(ContainError::Escalated(invalid(
                    band,
                    "bool_contfp_boundary",
                )));
            }
        }
    }
    Ok(FaceContainment::In)
}

/// **Boundary-only containment** for an on-carrier point against a
/// CURVED face: the shared boundary pre-pass ([`boundary_pre_pass`]),
/// and nothing after it, with the chord rows gated to **`Line`
/// carriers only** (a line's chord IS the edge, so the rows decide
/// exactly; a curved boundary edge is not its chord and gets no
/// verdict here). `None` is the honest remainder: interior/exterior
/// classification on a curved chart does not exist at boolean
/// classification, and the caller's typed frontier door says so.
///
/// # Errors
///
/// [`ContainError`] — sliver escalations or unwalkable topology.
pub(super) fn curved_boundary_containment<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    q: Point3<T>,
    band: Band,
) -> Result<Option<FaceContainment>, ContainError> {
    let face_data = body.get_face(face).ok_or(ContainError::Corrupt)?;
    let loops: Vec<_> = core::iter::once(face_data.outer)
        .chain(face_data.rings.iter().copied())
        .collect();
    boundary_pre_pass(body, &loops, q, EdgeChords::LinesOnly, band)
}

/// Which boundary edges the shared pre-pass may decide `OnEdge`
/// through the chord rows.
#[derive(Clone, Copy, PartialEq, Eq)]
enum EdgeChords {
    /// Every boundary edge ([`contfp`]'s historical posture — its
    /// callers' events lie in the face plane, where the chord test is
    /// exact for line boundaries and conservative-by-band for the
    /// conic ones it has always run).
    All,
    /// `Line`-carrier boundary edges only (the curved chart's walk —
    /// a chord row on a conic boundary would answer about the chord,
    /// not the edge).
    LinesOnly,
}

/// **The shared boundary pre-pass**: vertex coincidence over ALL
/// loops FIRST — an edge-interior verdict must never shadow a vertex
/// coincidence, across loops exactly as within one (the invariant
/// [`contfp`] always stated; running it per loop let an outer-edge
/// hit shadow a ring vertex, fixed here with its red-then-green row
/// below) — then edge interiors over all loops. Four rows, one home:
/// `bool_contact_vertex`, `bool_contact_edge_span` (×2),
/// `bool_contact_edge`.
fn boundary_pre_pass<T: Decide>(
    body: &Body<T>,
    loops: &[crate::entity::LoopKey],
    q: Point3<T>,
    chords: EdgeChords,
    band: Band,
) -> Result<Option<FaceContainment>, ContainError> {
    for &lk in loops {
        let cycle = loop_cycle_points(body, lk)?;
        for (v, _, p) in &cycle {
            let margin = Margin::norm3(q - *p);
            match decide("bool_contact_vertex", margin, band) {
                Ok(Sign::Zero) => return Ok(Some(FaceContainment::OnVertex(*v))),
                Ok(Sign::Positive) => {}
                Ok(Sign::Negative) => {
                    return Err(ContainError::Escalated(invalid(
                        band,
                        "bool_contact_vertex",
                    )));
                }
                Err(diag) => return Err(ContainError::Escalated(diag)),
            }
        }
    }
    for &lk in loops {
        let cycle = loop_cycle_points(body, lk)?;
        for (i, (_, he, a)) in cycle.iter().enumerate() {
            let edge_key = body.get_half_edge(*he).ok_or(ContainError::Corrupt)?.edge;
            if chords == EdgeChords::LinesOnly {
                let is_line = body
                    .get_edge(edge_key)
                    .and_then(|e| body.get_curve_geom(e.curve))
                    .and_then(crate::null::CurveGeom::certified)
                    .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. }));
                if !is_line {
                    continue;
                }
            }
            let b = cycle[(i + 1) % cycle.len()].2;
            let e = b - *a;
            let len = e.norm();
            let ehat = e.normalize();
            // Span gates: q's projection strictly interior to [a, b]
            // (endpoint neighborhoods already decided above).
            let s0 = (q - *a).dot(ehat);
            let s1 = len - s0;
            let interior = matches!(
                decide("bool_contact_edge_span", Margin::of(s0), band),
                Ok(Sign::Positive)
            ) && matches!(
                decide("bool_contact_edge_span", Margin::of(s1), band),
                Ok(Sign::Positive)
            );
            if !interior {
                continue;
            }
            let perp = Margin::norm3((q - *a).cross(ehat));
            match decide("bool_contact_edge", perp, band) {
                Ok(Sign::Zero) => return Ok(Some(FaceContainment::OnEdge(edge_key))),
                Ok(Sign::Positive) => {}
                Ok(Sign::Negative) => {
                    return Err(ContainError::Escalated(invalid(band, "bool_contact_edge")));
                }
                Err(diag) => return Err(ContainError::Escalated(diag)),
            }
        }
    }
    Ok(None)
}

fn invalid(band: Band, predicate: &'static str) -> Indeterminate {
    Indeterminate {
        margin: geom_core::MarginDiag::Invalid,
        band,
        predicate: Some(predicate),
    }
}

/// The loop's (start vertex, half-edge, point) cycle. An empty/lone
/// loop yields `Corrupt` (a face boundary must be a cycle here).
#[allow(clippy::type_complexity)]
fn loop_cycle_points<T: Decide>(
    body: &Body<T>,
    lk: crate::entity::LoopKey,
) -> Result<Vec<(VertexKey, crate::entity::HalfEdgeKey, Point3<T>)>, ContainError> {
    let loop_data = body.get_loop(lk).ok_or(ContainError::Corrupt)?;
    let crate::entity::LoopBoundary::Cycle { first } = loop_data.boundary else {
        return Err(ContainError::Corrupt);
    };
    let mut out = Vec::new();
    for he in body.loop_cycle(first).ok_or(ContainError::Corrupt)? {
        let start = body.get_half_edge(he).ok_or(ContainError::Corrupt)?.start;
        let p = *body
            .get_point(body.get_vertex(start).ok_or(ContainError::Corrupt)?.point)
            .ok_or(ContainError::Corrupt)?;
        out.push((start, he, p));
    }
    if out.is_empty() {
        return Err(ContainError::Corrupt);
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::entity::LoopBoundary;
    use geom_core::Tol;

    /// The cross-loop shadowing row (red-then-green, M9-3 fix pass):
    /// `contfp`'s stated invariant — an edge-interior verdict never
    /// shadows a vertex coincidence — must hold ACROSS loops, not
    /// only within one. The scaffold moves a RING vertex of the holed
    /// box's ringed face to within the zero band of an OUTER edge's
    /// interior and queries that exact point: the per-loop pre-pass
    /// answered `OnEdge` (the outer loop's edge pass ran before the
    /// ring's vertex pass); the shared all-loops-vertex-first pass
    /// answers `OnVertex`. The body is a POINT SCAFFOLD only — the
    /// pre-pass consumes vertex points and chords derived from them,
    /// and nothing else of the (now geometrically inconsistent) box
    /// is read.
    #[test]
    fn a_ring_vertex_is_never_shadowed_by_an_outer_edge() {
        let holed = crate::fixtures::ops_holed_box(Tol::witness());
        let mut body = holed.body;
        let band = Band::linear(Tol::witness()).unwrap();
        // The ringed face whose outer cycle lies at z = 1 (the top).
        let (face, ring) = body
            .faces
            .iter()
            .find_map(|(k, f)| {
                let ring = *f.rings.first()?;
                let LoopBoundary::Cycle { first } = body.loops.get(f.outer)?.boundary else {
                    return None;
                };
                let top = body.loop_cycle(first)?.into_iter().all(|he| {
                    body.half_edges
                        .get(he)
                        .and_then(|h| body.vertices.get(h.start))
                        .and_then(|v| body.points.get(v.point))
                        .is_some_and(|p| p.z == 1.0)
                });
                (top).then_some((k, ring))
            })
            .expect("the holed box has a ringed top face");
        let ring_vertex = {
            let LoopBoundary::Cycle { first } = body.loops[ring].boundary else {
                panic!("the ring is a cycle");
            };
            body.half_edges[first].start
        };
        // Move the ring vertex within the zero band of the outer
        // edge from (0,0,1) to (1,0,1), strictly interior in span.
        let q = geom_core::Point3::new(0.5, 4e-10, 1.0);
        let pk = body.vertices[ring_vertex].point;
        body.points[pk] = q;
        let got = contfp(&body, face, geom_core::Vec3::new(0.0, 0.0, 1.0), q, band)
            .expect("the pre-pass decides");
        assert_eq!(
            got,
            FaceContainment::OnVertex(ring_vertex),
            "the ring vertex must win over the outer edge's interior"
        );
    }
}
