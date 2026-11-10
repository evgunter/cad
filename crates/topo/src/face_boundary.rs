//! **A face's boundary as the curve × face table's boundary pass reads
//! it** ([`geom_brep::curve_face::boundary`]): the face's edges in walk
//! order, each with its certified curve, its start vertex and its two
//! ends, and the lone vertices of its pierce rings. Every reader that asks
//! where a curve meets a face's boundary walks the face through
//! [`face_boundary`]: the boolean's crossing and overlap lanes, and the
//! at-rest tangent-line reading.
//!
//! It takes a body and a face and names no operation's state, so it is a
//! root reading (`[topo.homes]`, `crates/topo/README.md`), and it refuses
//! in its own type ([`FaceBoundaryError`]): each caller says what a lost
//! face is in its own words.

use geom_brep::curve_face::{self, BoundaryRefusal};
use geom_brep::curve_pair::Span;
use geom_core::{Band, Decide, Point3};

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, VertexKey};
use crate::live::BoundaryMember;
use crate::null::CurveGeom;

/// **Why [`face_boundary`] read no boundary.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FaceBoundaryError {
    /// The face does not resolve in the body.
    FaceLost,
}

impl FaceBoundaryError {
    /// The kernel invariant the refusal names, in the words its callers
    /// raise it with.
    pub(crate) const fn what(self) -> &'static str {
        match self {
            Self::FaceLost => "boundary pass: face lost",
        }
    }
}

/// A boundary edge as the pass reads it.
pub(crate) struct BoundaryEdge<'a, T: geom_core::Real> {
    /// The edge.
    pub(crate) key: EdgeKey,
    /// Its half-edge's start vertex, and that vertex's point.
    pub(crate) start: (VertexKey, Point3<T>),
    /// Its certified curve.
    pub(crate) curve: &'a geom_brep::EdgeCurve<T>,
    /// The edge's two vertices, its span's start first.
    pub(crate) ends: [VertexKey; 2],
}

/// `face`'s boundary as the pass reads it: its edges in walk order and
/// its lone vertices.
pub(crate) struct FaceBoundary<'a, T: geom_core::Real> {
    pub(crate) edges: Vec<BoundaryEdge<'a, T>>,
    pub(crate) lone: Vec<(VertexKey, Point3<T>)>,
}

impl<T: Decide> FaceBoundary<'_, T> {
    /// The edges as the pass takes them.
    pub(crate) fn spans(&self) -> Vec<(Point3<T>, Span<'_, T>)> {
        self.edges
            .iter()
            .map(|e| (e.start.1, Span::new(e.curve.carrier(), e.curve.params())))
            .collect()
    }

    /// The lone vertices' points, as the pass takes them.
    pub(crate) fn lone_points(&self) -> Vec<Point3<T>> {
        self.lone.iter().map(|&(_, p)| p).collect()
    }

    /// **Every meeting of `swept` with the boundary** ([`curve_face::read`]).
    pub(crate) fn read(
        &self,
        swept: &Span<'_, T>,
        band: Band,
    ) -> Result<curve_face::BoundaryReading<T>, BoundaryRefusal> {
        curve_face::read(swept, &self.spans(), &self.lone_points(), band)
    }

    /// **Every meeting of `swept` with the boundary, read past each
    /// member's refusal** ([`curve_face::located`]).
    pub(crate) fn located(&self, swept: &Span<'_, T>, band: Band) -> curve_face::Located<T> {
        curve_face::located(swept, &self.spans(), &self.lone_points(), band)
    }
}

/// `face`'s boundary in its walk order; `None` where an edge has no
/// certified curve.
///
/// # Errors
///
/// [`FaceBoundaryError::FaceLost`] where `face` does not resolve.
///
/// # Panics
///
/// Where a record past `face` does not resolve or a loop walk does not
/// close (D2 row 4): its loops, a lone vertex's point, their members, a
/// member's start point, edge and curve. Every hop from the face is a
/// link ([`crate::live::FaceView::boundary`]).
pub(crate) fn face_boundary<T: Decide>(
    y: &Body<T>,
    face: FaceKey,
) -> Result<Option<FaceBoundary<'_, T>>, FaceBoundaryError> {
    let face_data = y.face(face).ok_or(FaceBoundaryError::FaceLost)?;
    let mut boundary = FaceBoundary {
        edges: Vec::new(),
        lone: Vec::new(),
    };
    for member in face_data.boundary() {
        match member {
            BoundaryMember::Isolated { vertex, point } => boundary.lone.push((vertex, point)),
            BoundaryMember::Edge { he, half, ek, edge } => {
                let start = y.linked_vertex_point(half.start, EntityId::HalfEdge(he), "start");
                let CurveGeom::Certified(curve) = y.edge_curve_linked(ek, edge) else {
                    return Ok(None);
                };
                // A span runs forward from its `he_plus`'s start.
                let (from, to) = crate::query::edge_ends(y, ek);
                boundary.edges.push(BoundaryEdge {
                    key: ek,
                    start: (half.start, start),
                    curve,
                    ends: [from, to],
                });
            }
        }
    }
    Ok(Some(boundary))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;

    /// **A face that does not resolve is the walk's own refusal**, named
    /// in the words its callers raise it with; a live face reads its
    /// boundary in walk order. (The boolean's raising of it:
    /// `boolean::standing_rows::a_lost_boundary_face_is_the_classification_invariant`.)
    #[test]
    fn a_lost_face_refuses_in_the_walks_type_and_a_live_face_reads() {
        let mut body = crate::test_support_fixtures::brick::<f64>(
            (0.0, 1.0),
            (0.0, 1.0),
            (0.0, 1.0),
            Tol::witness(),
        );
        let face = body.faces().map(|(k, _)| k).next().unwrap();
        let read = face_boundary(&body, face)
            .unwrap()
            .expect("a brick's edges are certified");
        assert_eq!(read.edges.len(), 4, "a brick face's four edges");
        assert!(read.lone.is_empty());
        body.faces.remove(face);
        let lost = face_boundary(&body, face).map(|_| ()).unwrap_err();
        assert_eq!(lost, FaceBoundaryError::FaceLost);
        assert_eq!(lost.what(), "boundary pass: face lost");
    }
}
