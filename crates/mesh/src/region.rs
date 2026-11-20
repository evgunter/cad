//! **Which CDT faces a face's loops bound**: the even-odd reading the
//! planar and trimmed lanes classify their triangles by, and the one
//! refusal it implies, for loops that bound nothing.
//!
//! The reading is combinatorial: a boundary side's crossing count is how
//! often the loops' constraints realise that CDT edge, and the flood
//! fill toggles across a side of odd count. Which sides two traversals
//! share is decided upstream of it, by the triangulator merging chart
//! points that are bitwise equal (`crate::cdt`'s equality): a float
//! equality, on which the parities are then exact.

use std::collections::HashMap;

use spade::{
    ConstrainedDelaunayTriangulation, Point2 as SpadePoint, Triangulation,
    handles::{DirectedEdgeHandle, FixedFaceHandle, InnerTag},
};
use topo::FaceKey;

use crate::types::TessellateError;

/// A CDT edge's identity as an unordered pair of vertex indices.
pub(crate) fn edge_key(
    e: DirectedEdgeHandle<'_, SpadePoint<f64>, (), spade::CdtEdge<()>, ()>,
) -> (usize, usize) {
    let (p, q) = (e.from().fix().index(), e.to().fix().index());
    (p.min(q), p.max(q))
}

/// Each realised boundary edge's crossing count, keyed by
/// [`edge_key`]: the even-odd input to [`classify_faces`] (#116).
pub(crate) type Crossings = HashMap<(usize, usize), u32>;

/// **Whether a face's loops bound a region of its chart**, read off the
/// parities [`classify_faces`] takes: that reading toggles only across a
/// side traversed an odd number of times, so with none it marks no face
/// inside, and the loops bound no region. The one home of that fact,
/// for every lane that classifies by [`Crossings`].
///
/// Which sides two traversals share is the triangulator's: it merges
/// chart points that are bitwise equal, so two traversals through the
/// same points realise the same sides. That equality is a float input,
/// and the parities are decided exactly on what it gives. Traversals
/// whose points differ by rounding share no side, and are not this
/// refusal's.
///
/// # Errors
///
/// [`TessellateError::BoundaryBoundsNoRegion`] when no side is odd.
pub(crate) fn require_a_region(
    face: FaceKey,
    crossings: &Crossings,
) -> Result<(), TessellateError> {
    if crossings.values().all(|n| n % 2 == 0) {
        return Err(TessellateError::BoundaryBoundsNoRegion { face });
    }
    Ok(())
}

/// Even-odd interior flag per CDT face, indexed by face index (0 is
/// spade's outer face, which is outside every loop).
///
/// A breadth-first walk of the face-adjacency graph from the outer face,
/// toggling the flag exactly when it steps across a boundary segment
/// traversed an odd number of times. Purely combinatorial: the result
/// depends on the triangulation's connectivity and on integer crossing
/// counts, never on a coordinate comparison.
pub(crate) fn classify_faces(
    cdt: &ConstrainedDelaunayTriangulation<SpadePoint<f64>>,
    crossings: &Crossings,
) -> Vec<bool> {
    let toggles = |e| crossings.get(&edge_key(e)).is_some_and(|n| n % 2 == 1);
    let mut inside = vec![false; cdt.num_all_faces()];
    let mut seen = vec![false; cdt.num_all_faces()];
    let mut queue: Vec<FixedFaceHandle<InnerTag>> = Vec::new();

    // Seed: step inwards across every convex-hull edge. The hull bounds
    // the outer face, which is outside every loop; a hull edge that is
    // itself a boundary segment toggles on the way in.
    for hull in cdt.convex_hull() {
        let e = if hull.face().is_outer() {
            hull.rev()
        } else {
            hull
        };
        let Some(f) = e.face().as_inner() else {
            continue; // both sides outer: a degenerate, edge-only CDT
        };
        let i = f.fix().index();
        if !seen[i] {
            seen[i] = true;
            inside[i] = toggles(e);
            queue.push(f.fix());
        }
    }

    let mut head = 0;
    while head < queue.len() {
        let f = queue[head];
        head += 1;
        let here = inside[f.index()];
        for e in cdt.face(f).adjacent_edges() {
            let Some(g) = e.rev().face().as_inner() else {
                continue; // the outer face, already the walk's origin
            };
            let j = g.fix().index();
            if !seen[j] {
                seen[j] = true;
                inside[j] = here ^ toggles(e);
                queue.push(g.fix());
            }
        }
    }
    inside
}
