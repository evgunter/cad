//! **A built body's stored edge certificates, re-derived**: each
//! named edge re-certified against the body's own endpoints and
//! surfaces through the plane × NURBS lane, and its stored certificate
//! compared with the fresh one by `Debug` (its D9 identity). The check
//! the carrying grafts (the void door, the containment fallback's
//! assembly) are pinned with.
//!
//! What this module deliberately did NOT absorb, as the whole list:
//!
//! - `snowman`'s carrier-keyed certificate map: it matches an
//!   operand's edges to a result's, which is that suite's question.

use geom_core::{Band, Tol};
use topo::{Body, EdgeKey};

/// Asserts that every certified edge among `edges` of `body` stores the
/// certificate a fresh re-certification mints at `Band::linear(tol)`,
/// and returns how many it compared. An edge with no certified carrier
/// is skipped.
pub fn assert_certificates_fresh(
    label: &str,
    body: &Body<f64>,
    edges: impl IntoIterator<Item = EdgeKey>,
    tol: Tol,
) -> usize {
    let band = Band::linear(tol).unwrap();
    let mut compared = 0;
    for dk in edges {
        let e = body.get_edge(dk).unwrap();
        let Some(topo::CurveGeom::Certified(c)) = body.get_curve_geom(e.curve) else {
            continue;
        };
        let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let start = p(body.get_half_edge(e.he_plus).unwrap().start);
        let end = p(body.half_edge_end(e.he_plus).unwrap());
        let fresh = c
            .recertify_via(
                start,
                end,
                |k| body.get_surface(k).cloned(),
                band,
                Some(geom_brep::NurbsLane::certified()),
            )
            .unwrap_or_else(|e| panic!("{label}: edge {dk:?} re-certifies, got {e:?}"));
        assert_eq!(
            format!("{:?}", c.certificate()),
            format!("{fresh:?}"),
            "{label}: edge {dk:?} stores the fresh run's certificate"
        );
        compared += 1;
    }
    compared
}
