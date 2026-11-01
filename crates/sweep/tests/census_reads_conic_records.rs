//! **The census reads a record on an ellipse edge.** The tilted drum's
//! lower part ([`drum_lower`]) is convex, so the plane through a point
//! `P` of its ellipse rim whose normal bisects the wall's and the cut
//! face's normals there supports it at `P` alone. A cube set beyond
//! that plane, its corner at `P` and its main diagonal along the
//! normal, touches the drum at `P` only: the union cuts the rim there,
//! records the corner on the cut vertex, and its output stage joins the
//! cut vertex away, so the record lands on the whole ellipse arc.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

use geom_core::{Point3, Tol, Vec3};
use topo::BooleanResult;

use crate::conic_edge_curved_face::{DRUM_RADIUS, TILT, drum_lower};
use crate::reach_wall_chord_rows::cube_at;

/// The rim point at azimuth `phi` and the supporting plane's normal
/// there.
fn rim_point(phi: f64) -> (Point3<f64>, Vec3<f64>) {
    let (s, c) = phi.sin_cos();
    let x = DRUM_RADIUS * c;
    let p = Point3::new(x, DRUM_RADIUS * s, 0.5 - x * TILT.tan());
    let wall = Vec3::new(c, s, 0.0);
    let cut = Vec3::new(TILT.sin(), 0.0, TILT.cos());
    (p, (wall + cut).normalize())
}

/// The record set of `body` holding the one `(vertex, edge)` row.
fn one_ve(
    body: &topo::Body<f64>,
    vertex: topo::VertexKey,
    edge: topo::EdgeKey,
) -> topo::ContactRecords {
    topo::test_support::records(body, [(topo::Cell::Vertex(vertex), topo::Cell::Edge(edge))])
}

/// **A cube's corner on the drum's ellipse rim is confirmed on the
/// ellipse arc, and a corner record on the other arc is stale.** Tier
/// 3′ leaves only the cross-solid curved pairs it cannot decide
/// (`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
/// Red when the census refuses a record on an ellipse edge, or reads
/// the record off the arc that holds it.
#[test]
fn a_corner_on_an_ellipse_rim_is_confirmed_on_its_arc() {
    let tol = Tol::witness();
    let drum = drum_lower();
    for phi in [FRAC_PI_2, 1.1] {
        for spin in [0.0, 0.4] {
            let at = format!("phi {phi} spin {spin}");
            let (p, m) = rim_point(phi);
            let cube = cube_at(p, m, spin, 0.3);
            let Ok(BooleanResult::Body(y)) = topo::union(&drum, &cube, tol) else {
                panic!("{at}: drum ∪ cube is a body");
            };
            let records: Vec<_> = y.contacts.vertex_edge().collect();
            let [(corner, arc)] = records[..] else {
                panic!("{at}: one (vertex, edge) record: {:?}", y.contacts);
            };
            let ellipses: Vec<_> = y
                .body
                .edges()
                .filter(|(_, e)| {
                    y.body
                        .get_curve_geom(e.curve)
                        .and_then(topo::CurveGeom::certified)
                        .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Ellipse { .. }))
                })
                .map(|(k, _)| k)
                .collect();
            assert!(
                ellipses.contains(&arc),
                "{at}: the record is on an ellipse arc"
            );
            let undecided = |errors: Vec<topo::ValidationError>| {
                errors
                    .into_iter()
                    .filter(|e| !matches!(e, topo::ValidationError::CensusUndecidable { .. }))
                    .collect::<Vec<_>>()
            };
            let found = topo::validate_pseudomanifold(&y.body, &y.contacts, tol)
                .err()
                .unwrap_or_default();
            assert_eq!(
                undecided(found),
                vec![],
                "{at}: tier 3′ confirms the record"
            );
            let other = ellipses
                .iter()
                .copied()
                .find(|&e| e != arc)
                .expect("the rim's other arc");
            let found =
                topo::validate_pseudomanifold(&y.body, &one_ve(&y.body, corner, other), tol)
                    .err()
                    .unwrap_or_default();
            assert!(
                undecided(found).iter().any(|e| matches!(
                    e,
                    topo::ValidationError::StaleContactDeclaration {
                        declaration: topo::StaleDeclaration::VertexOnEdge { .. }
                    }
                )),
                "{at}: the corner is stale on the rim's other arc"
            );
        }
    }
}

/// **A box's edge crossing the drum's ellipse rim skew is confirmed as
/// an edge-edge record.** The box's edge lies in the supporting plane
/// at `P`, turned 60° from the rim's tangent, and its two faces there
/// open 45° either side of the plane's normal, so the box touches the
/// drum at `P` alone. The union cuts both edges there and its output
/// stage joins both cut vertices away: the crossing is an edge-edge
/// record on the box's edge and the ellipse arc. Red when the census
/// refuses it, or reads it stale.
#[test]
fn a_box_edge_crossing_an_ellipse_rim_is_an_edge_edge_record() {
    let tol = Tol::witness();
    let drum = drum_lower();
    for phi in [FRAC_PI_2, 1.1] {
        let (p, m) = rim_point(phi);
        let (s, c) = phi.sin_cos();
        let tangent = Vec3::new(-s, c, TILT.tan() * s).normalize();
        let (s60, c60) = 60_f64.to_radians().sin_cos();
        let w = tangent * c60 + m.cross(tangent) * s60;
        let k = w.cross(m);
        let (u, v) = ((m + k).normalize(), (m - k).normalize());
        let l = 0.3;
        let box_ = sweep::test_support::finished(
            "the box",
            topo::test_support::mapped_cube::<f64>(
                |x, y, z| p + (v * x + u * y + w * (z - 0.5)) * l,
                tol,
            ),
            tol,
        );
        let Ok(BooleanResult::Body(y)) = topo::union(&drum, &box_, tol) else {
            panic!("phi {phi}: drum ∪ box is a body");
        };
        let crossings: Vec<_> = y.contacts.edge_edge().collect();
        assert_eq!(
            crossings.len(),
            1,
            "phi {phi}: one edge-edge record: {:?}",
            y.contacts
        );
        let found = topo::validate_pseudomanifold(&y.body, &y.contacts, tol)
            .err()
            .unwrap_or_default();
        let found: Vec<_> = found
            .into_iter()
            .filter(|e| !matches!(e, topo::ValidationError::CensusUndecidable { .. }))
            .collect();
        assert_eq!(found, vec![], "phi {phi}: tier 3′ confirms the crossing");
    }
}
