//! **How a carved body's contact edges are described**, counted: the
//! edges storing the intrinsic tangency ([`intrinsic_edges`]), and the
//! contact edges stored as a conventional chart image instead
//! ([`chart_contact_edges`]) — the two descriptions the must-carry
//! rule chooses between for a definitely-smooth join. What a suite
//! CHECKS of a body it built, reading each edge's stored description
//! and evaluating nothing, so it routes here beside
//! [`super::cap_rims`] ([`super`]'s routing rule).
//!
//! `contact_edge_must_carry`, `must_carry_rule` and their review probes
//! each count the same thing, and a counter that drifts between them is
//! two instruments reporting one number.
//!
//! **Deliberately not absorbed**, and the whole of it:
//!
//! - `must_carry_rule`'s `chart_images_among`, a count over a NAMED
//!   set of edges rather than the whole body, which is its point.

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

/// How many edges of a body store a non-seam chart image — the
/// must-carry rule's UNDER-DETERMINED description of a contact edge,
/// and also whatever other chart-described edges the body carries (a
/// boolean's own rims among them), so a row reading it pins the count
/// of the whole body.
pub fn chart_contact_edges(body: &Body<f64>) -> usize {
    body.edges()
        .filter(|(_, e)| {
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(|c| c.description()),
                Some(EdgeDescription::Chart(c)) if !c.seam
            )
        })
        .count()
}
