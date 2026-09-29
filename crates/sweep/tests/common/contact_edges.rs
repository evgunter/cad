//! **How many of a carved body's edges store the intrinsic tangency**
//! ([`intrinsic_edges`]) — the description the must-carry rule stores
//! on a jet-determinate smooth join. What a suite CHECKS of a body it
//! built, reading each edge's stored description and evaluating
//! nothing, so it routes here beside [`super::cap_rims`] ([`super`]'s
//! routing rule). Shared because a counter that drifts between suites
//! is two instruments reporting one number.
//!
//! **Deliberately not absorbed**, and the whole of it: nothing. The
//! reads of the same description that are not this count — a
//! collection of the edges' keys, a station-by-station walk of each
//! described edge through the rule — are different reads and stay with
//! their rows.

use geom_brep::EdgeDescription;
use geom_core::Real;
use topo::Body;

/// How many edges of a body store the intrinsic tangency
/// (`EdgeDescription::TangentIntersection`), at any scalar.
pub fn intrinsic_edges<T: Real>(body: &Body<T>) -> usize {
    body.edges()
        .filter(|(_, e)| {
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(|c| c.description()),
                Some(EdgeDescription::TangentIntersection { .. })
            )
        })
        .count()
}
