//! Stations: a vertex inside a run of one carrier, which a sweep never
//! builds (crate README, "Walls: one per run") and a boolean's cut
//! can leave. [`cut_stations`] puts one back by hand, for a row about
//! what a later op does with it; [`station_vertices`] finds them.
//!
//! Not absorbed: [`super::cap_rims`] walks a cap's rims and reads
//! their descriptions, and finds no vertex.

use geom::Curve3;
use geom_core::{Point3, Tol};
use topo::{Body, EdgeKey, VertexKey};

/// `body` with the line edge `rim` cut at each of `points`, which lie
/// on it: stations put back by hand, the shape a boolean's cut leaves.
/// Each cut lands on the piece the previous one minted, so the pieces
/// are in the edge's own order.
pub fn cut_stations(
    mut body: Body<f64>,
    rim: EdgeKey,
    points: &[Point3<f64>],
    t: Tol,
) -> Body<f64> {
    let curve = body.get_edge(rim).unwrap().curve;
    let c = body
        .get_curve_geom(curve)
        .unwrap()
        .certified()
        .unwrap()
        .clone();
    let Curve3::Line { origin, dir } = *c.carrier() else {
        panic!("cut_stations cuts a line edge")
    };
    let mut us: Vec<f64> = points.iter().map(|&p| (p - origin).dot(dir)).collect();
    us.sort_by(f64::total_cmp);
    let mut rest = rim;
    for u in us {
        rest = body.split_edge(rest, u, t).unwrap().new_edge;
    }
    body
}

/// Every vertex of `body` that is a station: two distinct edges whose
/// carriers are one line or one circle, to 1e-9, between two faces on
/// different surfaces. Curved carriers included, which
/// `topo::joinable_vertices` does not yet read
/// (`work/fuse/curved-joinable-vertices-are-left-unjoined.md`). Two
/// faces on ONE surface at such a vertex are a full revolve's two
/// π-bands meeting at a pole, which keeps valence 2 by construction:
/// a curved join that row owns, not a station.
pub fn station_vertices(body: &Body<f64>) -> Vec<VertexKey> {
    let carrier = |e: EdgeKey| {
        let curve = body.get_edge(e).unwrap().curve;
        body.get_curve_geom(curve)
            .and_then(topo::CurveGeom::certified)
            .map(|c| c.carrier().clone())
    };
    let close = |a: Point3<f64>, b: Point3<f64>| (a - b).norm() < 1e-9;
    let one = |a: &Curve3<f64>, b: &Curve3<f64>| match (a, b) {
        (
            Curve3::Line {
                origin: o1,
                dir: d1,
            },
            Curve3::Line {
                origin: o2,
                dir: d2,
            },
        ) => d1.cross(*d2).norm() < 1e-9 && d1.cross(*o2 - *o1).norm() < 1e-9,
        (
            Curve3::Circle {
                center: c1,
                axis: a1,
                radius: r1,
                ..
            },
            Curve3::Circle {
                center: c2,
                axis: a2,
                radius: r2,
                ..
            },
        ) => close(*c1, *c2) && a1.cross(*a2).norm() < 1e-9 && (r1 - r2).abs() < 1e-9,
        _ => false,
    };
    body.vertices()
        .map(|(v, _)| v)
        .filter(|&v| {
            let es = body.edges_of_vertex(v).unwrap();
            let keys: Vec<_> = body
                .faces_of_vertex(v)
                .unwrap()
                .iter()
                .map(|&f| body.get_face(f).unwrap().surface)
                .collect();
            es.len() == 2
                && keys.len() == 2
                && keys[0] != keys[1]
                && es[0] != es[1]
                && matches!((carrier(es[0]), carrier(es[1])), (Some(a), Some(b)) if one(&a, &b))
        })
        .collect()
}
