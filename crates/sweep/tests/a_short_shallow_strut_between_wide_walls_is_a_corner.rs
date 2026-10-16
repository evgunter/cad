//! **A short, shallow edge between wide faces is a corner.** The wedge
//! between an edge's two faces is levered at how far the faces run
//! (`topo::wedge_reach`), not at how long the edge is: a dihedral is how
//! far the faces separate as they leave the edge.
//!
//! The fixture is a plate one metre across and `h = 1e4·ε` thick whose
//! profile turns by `α = 5e-5` at the origin, so the strut there is an
//! edge `h` long between two walls a metre wide at dihedral `α`. Levered
//! at the strut's own length the wedge is `α·h = ε/2`, inside the zero
//! band; levered at the walls' extent it is about `α · 1 m`, far past
//! `K·ε` at every ε the suite runs. So the strut rests as the walls'
//! `Intersection`, and tier 3 demands nothing else of it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{DihedralClass, EdgeDescription};
use geom_core::{Band, Point2, Point3, Tol};
use topo::Body;

/// The turn at the origin, radians.
const TURN: f64 = 5e-5;

/// The plate: `(−1, 0) → (0, 0) → (1, TURN) → (1, 1) → (−1, 1)`,
/// extruded `h` along `+z`.
fn plate(h: f64, tol: Tol) -> Body<f64> {
    let verts = [
        (-1.0, 0.0),
        (0.0, 0.0),
        (1.0, TURN),
        (1.0, 1.0),
        (-1.0, 1.0),
    ]
    .map(|(x, y)| (Point2::new(x, y), 0.0))
    .to_vec();
    sweep::test_support::prism(verts, h, tol)
}

/// The strut at the turn: the edge from the origin to `(0, 0, h)`.
fn strut_at_the_turn(body: &Body<f64>, h: f64, tol: Tol) -> topo::EdgeKey {
    let at = |p: Point3<f64>, q: Point3<f64>| p.distance(q) <= tol.eps();
    let (o, top) = (Point3::origin(), Point3::new(0.0, 0.0, h));
    body.edges()
        .find(|&(_, e)| {
            let he = body.get_half_edge(e.he_plus).unwrap();
            let point = |v| topo::readback::vertex_point(body, v).unwrap();
            let (a, b) = (
                point(he.start),
                point(body.half_edge_end(e.he_plus).unwrap()),
            );
            (at(a, o) && at(b, top)) || (at(a, top) && at(b, o))
        })
        .map(|(k, _)| k)
        .expect("the strut at the turn")
}

#[test]
fn a_short_shallow_strut_between_wide_walls_rests_as_their_intersection() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let h = 1e4 * tol.eps();
    let body = plate(h, tol);
    assert_eq!(
        topo::validate_geometric(&body, tol),
        Ok(()),
        "the plate is valid at rest"
    );
    let strut = strut_at_the_turn(&body, h, tol);
    let curve = body
        .get_curve_geom(body.get_edge(strut).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap();
    assert!(
        matches!(curve.description(), EdgeDescription::Intersection { .. }),
        "the strut rests as the walls' intersection, got {:?}",
        curve.description()
    );

    let [f, g] = topo::edge_faces(&body, strut);
    let surface = |f| {
        body.get_surface(body.get_face(f).unwrap().surface)
            .unwrap()
            .clone()
    };
    let (sf, sg) = (surface(f), surface(g));
    let mid = Point3::new(0.0, 0.0, h / 2.0);
    let reach = topo::edge_wedge_reach(&body, strut).unwrap();
    assert!(
        reach >= 1.0,
        "the walls reach a metre from the strut: {reach}"
    );
    assert_eq!(
        geom_brep::classify_dihedral(&sf, &sg, mid, reach, band),
        Ok(DihedralClass::Transverse),
        "levered at the walls' extent the turn is a corner"
    );
    assert_eq!(
        geom_brep::classify_dihedral(&sf, &sg, mid, h, band),
        Ok(DihedralClass::Smooth),
        "levered at the strut's own length the same turn reads smooth"
    );
}

/// **The fillet battery reads the same strut as a corner.** Its link
/// convexity is the wedge between the strut's two walls, levered at
/// their extent (`topo::wedge_reach`) like every other reader of it: at
/// the strut's own length the turn's margin `α·h = ε/2` would refuse
/// `TangentialEdge`, at the walls' metre it is a definite convex
/// corner, and the fillet's refusal, if any, is not that one.
#[test]
fn the_fillet_battery_reads_the_short_shallow_strut_as_a_corner() {
    let tol = Tol::witness();
    let h = 1e4 * tol.eps();
    let body = plate(h, tol);
    let strut = strut_at_the_turn(&body, h, tol);
    let at_rest = sweep::test_support::at_rest(&body, tol);
    // A radius whose trimlines clear the plate's other features at
    // every ε row: smaller ones refuse at 1e-6 on the strut's caps and
    // corners, each a reading of its own, never `TangentialEdge`.
    let out = sweep::blend::fillet_edges(&at_rest, &[strut], 0.9, tol)
        .unwrap_or_else(|e| panic!("the strut fillets as the walls' corner: {e:?}"));
    assert_eq!(
        topo::validate_geometric(&out.body, tol),
        Ok(()),
        "the filleted plate is valid at rest"
    );
}
