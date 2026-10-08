//! **The planar and axial offset doors end with the join**
//! (`docs/DESIGN.md`, maximal edges). The axial door moves every chart
//! of a body that holds one joinable vertex (a rim split at its
//! middle), which the move carries along the rim's moved carrier; only
//! the join the door ends with takes it: the door returns the join, and
//! the body it leaves holds no joinable vertex. The planar door solves
//! each MOVED corner from three planes, so it refuses a joinable vertex
//! on a moved plane; one on an edge whose two planes stay put (distance
//! 0) it carries unmoved, and the join it ends with takes it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Tol};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, ChartMove, EdgeKey, FaceKey};

/// `e` split at its parameter middle; the new vertex.
fn split_middle(body: &mut Body<f64>, e: EdgeKey, tol: Tol) -> topo::VertexKey {
    let (t0, t1) = body
        .get_curve_geom(body.get_edge(e).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .params();
    body.split_edge(e, 0.5 * (t0 + t1), tol).unwrap().vertex
}

/// Every chart of `body` moved by `d`: one move per surface key.
fn every_chart(body: &Body<f64>, d: f64) -> Vec<ChartMove<f64>> {
    let mut by_key: std::collections::BTreeMap<topo::SurfaceKey, Vec<FaceKey>> =
        std::collections::BTreeMap::new();
    for (f, face) in body.faces() {
        by_key.entry(face.surface).or_default().push(f);
    }
    by_key
        .into_values()
        .map(|faces| ChartMove { faces, distance: d })
        .collect()
}

/// **The planar door refuses a joinable vertex it must move**: it
/// solves each moved corner from the planes meeting there, so a vertex
/// of valence 2 on a straight edge whose planes both move (two planes)
/// fixes no point, and the door refuses the body before anything moves.
/// On a plain brick it reports no join.
#[test]
fn offset_planes_together_refuses_a_body_holding_a_joinable_vertex() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut body = topo::test_support::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let e = body.edges().next().unwrap().0;
    let v = split_middle(&mut body, e, tol);
    let moves = every_chart(&body, -0.1);
    assert!(matches!(
        topo::offset_planes_together(&mut body, &moves, band, tol),
        Err(topo::ReplaceFaceError::TogetherCorner { vertex, planes: 2, .. }) if vertex == v
    ));
    // On the brick itself, the door answers and leaves maximal edges.
    let mut brick = topo::test_support::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let moves = every_chart(&brick, -0.1);
    let out = topo::offset_planes_together(&mut brick, &moves, band, tol).unwrap();
    assert!(out.joins.is_empty());
    assert!(topo::joinable_vertices(&brick, band).unwrap().is_empty());
}

#[test]
fn offset_charts_together_ends_with_the_join() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut body = revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol,
    );
    // A rim circle, split where nothing else meets it.
    let rim = body
        .edges()
        .find(|(_, d)| {
            matches!(
                body.get_curve_geom(d.curve)
                    .and_then(topo::CurveGeom::certified)
                    .map(|c| c.carrier().kind()),
                Some(geom::CurveKind::Circle)
            )
        })
        .unwrap()
        .0;
    let v = split_middle(&mut body, rim, tol);
    assert_eq!(topo::joinable_vertices(&body, band).unwrap(), vec![v]);
    let moves = every_chart(&body, -0.1);
    let out = topo::offset_charts_together(&mut body, &moves, band, tol).unwrap();
    assert_eq!(out.joins.len(), 1);
    assert_eq!(out.joins[0].vertex, v);
    assert!(topo::joinable_vertices(&body, band).unwrap().is_empty());
}

/// **The join is the call's scope's, and no wider.** Two bricks side by
/// side, an edge of the SECOND split at its middle: a joinable vertex
/// in a solid the call does not move. Offsetting the first solid alone
/// writes nothing of the second (`offset_together::Scope`), and the
/// join it ends with is the scope's too: it reports no join, and the
/// second solid, its split vertex included, is bitwise as found.
#[test]
fn an_offset_joins_nothing_outside_its_scope() {
    use crate::shell8_common::{beside, deep_dump, faces_of, solid_of};
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let brick = topo::test_support::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let mut body = beside(&brick, &brick, 2.0);
    let first = body.solids().next().unwrap().0;
    let second = body.solids().nth(1).unwrap().0;
    let e = body
        .edges()
        .find(|(_, d)| solid_of(&body, body.face_of_half_edge(d.he_plus).unwrap()) == second)
        .unwrap()
        .0;
    let v = split_middle(&mut body, e, tol);
    assert_eq!(topo::joinable_vertices(&body, band).unwrap(), vec![v]);
    let before = deep_dump(&body, second);
    let moves: Vec<ChartMove<f64>> = faces_of(&body, first)
        .into_iter()
        .map(|f| ChartMove {
            faces: vec![f],
            distance: -0.1,
        })
        .collect();
    let out = topo::offset_planes_together(&mut body, &moves, band, tol).unwrap();
    assert!(
        out.joins.is_empty(),
        "no join outside the scope: {:?}",
        out.joins
    );
    assert!(
        body.get_vertex(v).is_some(),
        "the other solid's vertex stands"
    );
    assert_eq!(
        deep_dump(&body, second),
        before,
        "the other solid is untouched"
    );
}

/// **The planar door ends with the join.** A brick with one edge split:
/// the split edge's two faces stay put (distance 0) and every other face
/// moves in. The split vertex is no moved corner, so the door carries it
/// as it stands, and the join it ends with takes it back: one join
/// reported, none left, and the brick's cells. Red when the door drops
/// its join.
#[test]
fn offset_planes_together_ends_with_the_join() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut body = topo::test_support::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let e = body.edges().next().unwrap().0;
    let still: Vec<FaceKey> = {
        let d = body.get_edge(e).unwrap();
        [d.he_plus, d.he_minus]
            .into_iter()
            .map(|h| body.face_of_half_edge(h).unwrap())
            .collect()
    };
    let v = split_middle(&mut body, e, tol);
    let moves: Vec<ChartMove<f64>> = body
        .faces()
        .map(|(f, _)| ChartMove {
            faces: vec![f],
            distance: if still.contains(&f) { 0.0 } else { -0.1 },
        })
        .collect();
    let out = topo::offset_planes_together(&mut body, &moves, band, tol).unwrap();
    assert_eq!(out.joins.len(), 1, "{:?}", out.joins);
    assert_eq!(out.joins[0].vertex, v);
    assert!(topo::joinable_vertices(&body, band).unwrap().is_empty());
    assert_eq!(body.vertices().count(), 8, "the brick's corners");
    assert_eq!(body.edges().count(), 12, "the brick's edges");
}
