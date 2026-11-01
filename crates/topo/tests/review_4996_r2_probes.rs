//! A cell an offset door moves and then joins away: its fate is kept,
//! its image the edge the join made, so whether its record lands unread
//! is read off that image, which does not hold the vertex's geometry.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use common::brick;
use geom_core::{Point3, Tol};
use topo::construction::Construct;
use topo::{Body, Cell, Cited, Cites, Contact, ContactRecords, FaceKey, Surface, VertexKey};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> geom_core::Band {
    geom_core::Band::linear(tol()).unwrap()
}

fn plane_face(body: &Body<f64>, axis: usize, at: f64, among: &[FaceKey]) -> FaceKey {
    let found: Vec<FaceKey> = body
        .faces()
        .filter(|(k, f)| {
            among.contains(k)
                && matches!(
                    body.get_surface(f.surface),
                    Some(Surface::Plane { origin, normal, .. })
                        if [normal.x, normal.y, normal.z][axis].abs() == 1.0
                            && [origin.x, origin.y, origin.z][axis] == at
                )
        })
        .map(|(k, _)| k)
        .collect();
    let [f] = found[..] else {
        panic!("one face at {axis} = {at}: {found:?}")
    };
    f
}

fn point(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    body.vertex_points().find(|(k, _)| *k == v).unwrap().1
}

/// **A vertex the offset moves and joins away does not land its record
/// unread.**
/// Brick `a` = [0,1]³ has its edge at x = 1, z = 1 split at y = 0.5;
/// brick `b` = [1,2]×[0.5,1.5]×[1,2] is grafted with its corner on the
/// split vertex, and one vertex–vertex record pairs the two. Moving a's
/// x = 1 face in by 0.1 moves the split vertex to x = 0.9 and the door's
/// join kills it. The record's touch is gone (b's corner stays at
/// x = 1), so the carry must consume or refuse it; a landing is false.
#[test]
fn a_vertex_an_offset_moves_and_joins_away_does_not_land_unread() {
    let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let a_faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    let x1 = plane_face(&body, 0, 1.0, &a_faces);
    let z1 = plane_face(&body, 2, 1.0, &a_faces);
    let e = body
        .edges()
        .map(|(k, _)| k)
        .find(|&e| {
            let d = body.get_edge(e).unwrap();
            let fs: Vec<FaceKey> = [d.he_plus, d.he_minus]
                .into_iter()
                .map(|h| body.face_of_half_edge(h).unwrap())
                .collect();
            fs.contains(&x1) && fs.contains(&z1)
        })
        .expect("the edge x = 1, z = 1");
    let (t0, t1) = body
        .get_curve_geom(body.get_edge(e).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .params();
    let split = body.split_edge(e, 0.5 * (t0 + t1), tol()).unwrap();
    let v = split.vertex;
    assert!((point(&body, v) - Point3::new(1.0, 0.5, 1.0)).norm() < 1e-12);
    let b = brick::<f64>((1.0, 2.0), (0.5, 1.5), (1.0, 2.0), tol());
    let keys = topo::graft_disjoint_all_keyed(&mut body, &b).unwrap();
    let w = b
        .vertex_points()
        .find(|(_, p)| (*p - Point3::new(1.0, 0.5, 1.0)).norm() < 1e-12)
        .map(|(k, _)| keys.vertex(k).unwrap())
        .unwrap();
    let records: ContactRecords = [Cited::new(
        Contact::new(&body, Cell::Vertex(v), Cell::Vertex(w)).unwrap(),
        Cites::decided(0),
    )]
    .into_iter()
    .collect();

    let mut moved = Body::clone(&body);
    let out = topo::replace_face_offset(&mut moved, x1, -0.1, tol()).expect("the offset builds");
    assert_eq!(out.joins.len(), 1, "the door joins the split vertex away");
    assert_eq!(out.joins[0].vertex, v);
    // The vertex reads kept, its image the edge the join made.
    assert!(matches!(
        out.lineage().fate(Cell::Vertex(v)),
        topo::Fate::Kept { .. }
    ));
    assert_eq!(
        out.lineage().images(Cell::Vertex(v)),
        [Cell::Edge(topo::joined_edge(&out.joins, out.joins[0].kept))]
    );
    assert!(
        moved.get_vertex(v).is_none(),
        "the split vertex is joined away"
    );
    let carried = out.carry_contacts(
        &records,
        &moved,
        band(),
        Some(topo::RegionLane::certified()),
    );
    // b's corner stays at x = 1; the joined edge now runs at x = 0.9.
    assert!((point(&moved, w) - Point3::new(1.0, 0.5, 1.0)).norm() < 1e-12);
    assert_eq!(
        carried,
        Ok(ContactRecords::default()),
        "a record whose touch the offset moved away is consumed"
    );
}
