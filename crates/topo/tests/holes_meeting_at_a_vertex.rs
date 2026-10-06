//! **Holes meeting at one vertex of a face build one body whatever order
//! their members fold in.**
//!
//! A plate `[0, 3] × [0, 2] × [0, 1]` with k prisms standing in it,
//! whose footprints on its top are k holes meeting at one point
//! `MEET = (1.5, 1, 1)`. Each prism is a right prism along an axis tilted
//! out of its own footprint, so above the top the prisms move apart and
//! touch nowhere; below it they cross inside the plate. When k ≥ 3
//! prisms fold before the plate, their union's vertex at `MEET` pierces
//! the top with k Out runs, and the pierce's ring hangs one strut per
//! run round one ring vertex. The struts' order round it is the order
//! the top's loop passes its corners at `MEET`, so in any but the runs'
//! angular order the loop crosses itself there.
//!
//! The rows, each in every member order:
//! - two, three and four wedges;
//! - three wedges clustered on one side, which leave the top a reflex
//!   sector at `MEET`;
//! - an L-shaped hole with its reflex corner at `MEET`, and two wedges in
//!   the quadrant it leaves.
//!
//! Each order asserts its counts, closed-form volume, tiers 3 and 3′,
//! that every face's corners at one point are angularly disjoint, one
//! vertex at `MEET`, and the body of the first order, compared by geometry.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{Hole, MEET, corners_disjoint, ell, wedge};
use common::{FaceGeometry, brick, describe_as_intersections, finished, prism_ops};
use geom_core::{Point3, Tol};
use topo::{AtRestBody, Body, BooleanResult, union, validate_geometric, validate_pseudomanifold};

fn t() -> Tol {
    Tol::witness()
}

/// A hole's prism, built over its profile through the tilted frame.
fn prism(h: &Hole) -> AtRestBody<f64> {
    let [o, u, v, n] = h.frame();
    let mut body = Body::<f64>::new();
    prism_ops(
        &mut body,
        &h.profile(),
        (0.0, h.length),
        |x, y, z| {
            let p = [0, 1, 2].map(|i| o[i] + x * u[i] + y * v[i] + z * n[i]);
            Point3::new(p[0], p[1], p[2])
        },
        FaceGeometry::Certified,
        t(),
    );
    describe_as_intersections(&mut body, t());
    finished("a tilted prism", body, t())
}

type Point = (i64, i64, i64);

fn at(p: Point3<f64>) -> Point {
    let n = |x: f64| (x * 1e6).round() as i64;
    (n(p.x), n(p.y), n(p.z))
}

/// The cycles of a face's loops.
fn loops(body: &Body<f64>, f: &topo::Face) -> Vec<Vec<topo::HalfEdgeKey>> {
    core::iter::once(f.outer)
        .chain(f.rings.iter().copied())
        .map(|l| match body.get_loop(l).unwrap().boundary {
            topo::LoopBoundary::Cycle { first } => body.loop_cycle(first).unwrap(),
            topo::LoopBoundary::Empty { .. } => panic!("a closed body has no empty loop"),
        })
        .collect()
}

/// A body's geometry, key-free: each face by the points of its loops'
/// vertices, each edge by its ends' points, as sorted multisets.
fn shape(body: &Body<f64>) -> (Vec<Vec<Point>>, Vec<[Point; 2]>) {
    let pt = |he| at(body.half_edge_start_point(he).unwrap());
    let mut faces: Vec<Vec<Point>> = body
        .faces()
        .map(|(_, f)| {
            let mut ps: Vec<Point> = loops(body, f).into_iter().flatten().map(pt).collect();
            ps.sort_unstable();
            ps
        })
        .collect();
    faces.sort_unstable();
    let mut edges: Vec<[Point; 2]> = body
        .edges()
        .map(|(_, e)| {
            let mut ps = [e.he_plus, e.he_minus].map(pt);
            ps.sort_unstable();
            ps
        })
        .collect();
    edges.sort_unstable();
    (faces, edges)
}

/// Every order of `0..n`.
fn orders(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![Vec::new()];
    }
    let mut out = Vec::new();
    for order in orders(n - 1) {
        for i in 0..n {
            let mut o = order.clone();
            o.insert(i, n - 1);
            out.push(o);
        }
    }
    out
}

/// Folds the plate (member 0) and `holes`' prisms by union in every
/// order and asserts each result (module docs).
fn every_order(label: &str, holes: &[Hole], counts: [usize; 3]) {
    let mut members = vec![finished(
        "the plate",
        brick::<f64>((0.0, 3.0), (0.0, 2.0), (0.0, 1.0), t()),
        t(),
    )];
    members.extend(holes.iter().map(prism));
    let volume = 6.0 + holes.iter().map(Hole::above).sum::<f64>();
    let c = at(Point3::new(MEET[0], MEET[1], MEET[2]));
    let mut first = None;
    for order in orders(members.len()) {
        let what = format!("{label}, member order {order:?} (0 = plate)");
        let mut body = members[order[0]].clone();
        let mut contacts = None;
        for (k, &i) in order.iter().enumerate().skip(1) {
            match union(&body, &members[i], t()) {
                Ok(BooleanResult::Body(r)) => {
                    body = r.body;
                    contacts = Some(r.contacts);
                }
                Ok(BooleanResult::Empty) => panic!("{what}: step {k} is empty"),
                Err(e) => panic!("{what}: step {k} refused: {e:?}"),
            }
        }
        let got = [
            body.faces().count(),
            body.edges().count(),
            body.vertices().count(),
        ];
        assert_eq!(got, counts, "{what}: faces, edges, vertices");
        assert_eq!(validate_geometric(&body, t()), Ok(()), "{what}: tier 3");
        assert!(
            validate_pseudomanifold(&body, &contacts.unwrap(), t()).is_ok(),
            "{what}: tier 3′"
        );
        let v = topo::mass_properties(&body, t()).unwrap().volume;
        assert!(
            (v - volume).abs() < 1e-9,
            "{what}: volume {v}, closed form {volume}"
        );
        assert_eq!(corners_disjoint(&body), Ok(()), "{what}: corners");
        let s = shape(&body);
        let at_c = body
            .vertices()
            .filter(|&(k, _)| at(topo::readback::vertex_point(&body, k).unwrap()) == c)
            .count();
        assert_eq!(at_c, 1, "{what}: vertices at MEET");
        match &first {
            None => first = Some(s),
            Some(f) => assert!(*f == s, "{what}: a different body from the first order"),
        }
    }
}

/// **Two, three and four wedges meeting at one vertex of the top build
/// one body in every member order.**
#[test]
fn wedges_meeting_at_a_vertex_build_one_body_in_every_member_order() {
    every_order(
        "two wedges",
        &[wedge(0.0, 50.0, 0), wedge(120.0, 170.0, 1)],
        [14, 30, 19],
    );
    every_order(
        "three wedges",
        &[
            wedge(0.0, 50.0, 0),
            wedge(120.0, 170.0, 1),
            wedge(240.0, 290.0, 2),
        ],
        [18, 39, 24],
    );
    every_order(
        "four wedges",
        &[
            wedge(0.0, 50.0, 0),
            wedge(90.0, 140.0, 1),
            wedge(180.0, 230.0, 2),
            wedge(270.0, 320.0, 3),
        ],
        [22, 48, 29],
    );
}

/// **Holes meeting at a vertex with a reflex sector there build one body
/// in every member order**: three wedges on one side, leaving the top a
/// reflex sector between them, and an L-shaped hole whose reflex corner
/// is the vertex, with two wedges in the quadrant it leaves.
#[test]
fn holes_with_a_reflex_sector_at_their_vertex_build_one_body_in_every_member_order() {
    every_order(
        "three wedges on one side",
        &[
            wedge(0.0, 40.0, 0),
            wedge(60.0, 100.0, 1),
            wedge(120.0, 160.0, 2),
        ],
        [18, 39, 24],
    );
    every_order(
        "an L and two wedges",
        &[ell(), wedge(190.0, 220.0, 1), wedge(235.0, 260.0, 2)],
        [21, 48, 30],
    );
}

/// The plate.
fn plate() -> AtRestBody<f64> {
    finished(
        "the plate",
        brick::<f64>((0.0, 3.0), (0.0, 2.0), (0.0, 1.0), t()),
        t(),
    )
}

fn body(what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(r)) => r.body,
        Ok(BooleanResult::Empty) => panic!("{what}: empty"),
        Err(e) => panic!("{what}: refused: {e:?}"),
    }
}

/// Asserts `b` is tier-3 valid with `volume`, its corners disjoint, and
/// that a block across the meeting point unions with it.
fn sound(what: &str, b: &AtRestBody<f64>, volume: f64) {
    assert_eq!(validate_geometric(b, t()), Ok(()), "{what}: tier 3");
    let v = topo::mass_properties(b, t()).unwrap().volume;
    assert!(
        (v - volume).abs() < 1e-9,
        "{what}: volume {v}, expected {volume}"
    );
    assert_eq!(corners_disjoint(b), Ok(()), "{what}: corners");
    let block = finished(
        "a block across the meeting point",
        brick::<f64>((1.21, 1.77), (0.68, 1.31), (0.86, 1.52), t()),
        t(),
    );
    body(&format!("{what}, then ∪ a block"), union(b, &block, t()));
}

/// **The plate against the union of the holes' prisms, in every op and
/// both orders.** The plate's top, less the holes, is a face whose ring
/// passes the meeting point once per hole:
/// - U − P (the prisms above the top) and both intersections (inside
///   it) build sound;
/// - P − U, in one boolean, has its zips fuse the meeting point twice,
///   and the zip crosses two corners of that ring first. With two holes
///   it builds as on main: two rings through one vertex, the zips'
///   shape for holes meeting at a point. With three or more the ring
///   passes the point three times or more, and it refuses
///   `PinchOfManyHolesInOneRing`. Which shape holes meeting at a point
///   take is open (`work/join/two-representations-of-holes-meeting-at-a-point.md`);
/// - the plate less each prism in turn builds the same volume sound,
///   the meeting point a vertex per hole on one ring.
#[test]
fn the_plate_against_the_holes_union_builds_sound_or_refuses_typed_in_every_op() {
    use topo::{intersect, subtract};
    let rows: [(&str, Vec<Hole>); 5] = [
        (
            "two wedges",
            vec![wedge(0.0, 50.0, 0), wedge(120.0, 170.0, 1)],
        ),
        (
            "three wedges",
            vec![
                wedge(0.0, 50.0, 0),
                wedge(120.0, 170.0, 1),
                wedge(240.0, 290.0, 2),
            ],
        ),
        (
            "four wedges",
            vec![
                wedge(0.0, 50.0, 0),
                wedge(90.0, 140.0, 1),
                wedge(180.0, 230.0, 2),
                wedge(270.0, 320.0, 3),
            ],
        ),
        (
            "three wedges on one side",
            vec![
                wedge(0.0, 40.0, 0),
                wedge(60.0, 100.0, 1),
                wedge(120.0, 160.0, 2),
            ],
        ),
        (
            "an L and two wedges",
            vec![ell(), wedge(190.0, 220.0, 1), wedge(235.0, 260.0, 2)],
        ),
    ];
    let p = plate();
    for (label, holes) in rows {
        let prisms: Vec<_> = holes.iter().map(prism).collect();
        let u = prisms[1..].iter().fold(prisms[0].clone(), |u, q| {
            body(&format!("{label}: the prisms' union"), union(&u, q, t()))
        });
        let above: f64 = holes.iter().map(Hole::above).sum();
        let inside = topo::mass_properties(&u, t()).unwrap().volume - above;
        sound(
            &format!("{label}: U − P"),
            &body(label, subtract(&u, &p, t())),
            above,
        );
        for (what, r) in [
            ("P ∩ U", intersect(&p, &u, t())),
            ("U ∩ P", intersect(&u, &p, t())),
        ] {
            sound(&format!("{label}: {what}"), &body(label, r), inside);
        }
        match (holes.len(), subtract(&p, &u, t())) {
            (2, r) => {
                let b = body(&format!("{label}: P − U"), r);
                let counts = [b.faces().count(), b.edges().count(), b.vertices().count()];
                assert_eq!(
                    counts,
                    [18, 41, 25],
                    "{label}: P − U, faces, edges, vertices"
                );
                assert_eq!(
                    validate_geometric(&b, t()),
                    Ok(()),
                    "{label}: P − U, tier 3"
                );
                let v = topo::mass_properties(&b, t()).unwrap().volume;
                assert!(
                    (v - (6.0 - inside)).abs() < 1e-9,
                    "{label}: P − U, volume {v}"
                );
            }
            (k, r) => assert!(
                matches!(
                    r,
                    Err(topo::BooleanError::PinchOfManyHolesInOneRing { holes, .. }) if holes == k
                ),
                "{label}: P − U refuses PinchOfManyHolesInOneRing with {k} holes, got {:?}",
                r.map(|_| ())
            ),
        }
        let seq = prisms.iter().fold(p.clone(), |b, q| {
            body(&format!("{label}: P less each prism"), subtract(&b, q, t()))
        });
        sound(&format!("{label}: P less each prism"), &seq, 6.0 - inside);
    }
}
