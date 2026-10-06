//! **Holes meeting at one vertex of a face build one body whatever order
//! their members fold in.**
//!
//! A plate `[0, 3] × [0, 2] × [0, 1]` with k prisms standing in it,
//! whose footprints on its top are k holes meeting at one point
//! `C = (1.5, 1, 1)`. Each prism is a right prism along an axis tilted
//! out of its own footprint, so above the top the prisms move apart and
//! touch nowhere; below it they cross inside the plate. When k ≥ 3
//! prisms fold before the plate, their union's vertex at `C` pierces
//! the top with k Out runs, and the pierce's ring hangs one strut per
//! run round one ring vertex. The struts' order round it is the order
//! the top's loop passes its corners at `C`, so in any but the runs'
//! angular order the loop crosses itself there.
//!
//! The rows, each in every member order:
//! - two, three and four wedges;
//! - three wedges clustered on one side, which leave the top a reflex
//!   sector at `C`;
//! - an L-shaped hole with its reflex corner at `C`, and two wedges in
//!   the quadrant it leaves.
//!
//! Each order asserts its counts, closed-form volume, tiers 3 and 3′,
//! that every face's corners at one point are angularly disjoint, one
//! vertex at `C`, and the body of the first order, compared by geometry.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::common;

use common::{FaceGeometry, brick, describe_as_intersections, finished, prism_ops};
use geom_core::{Point3, Tol, Vec3};
use topo::{AtRestBody, Body, BooleanResult, union, validate_geometric, validate_pseudomanifold};

fn t() -> Tol {
    Tol::witness()
}

/// The point the holes meet at.
const C: [f64; 3] = [1.5, 1.0, 1.0];

/// A hole: its footprint on the plate's top (counterclockwise from +z,
/// one corner at `C`), the direction its axis leans towards, and the
/// prism's reach below `C` and its length, along the axis.
struct Hole {
    footprint: Vec<(f64, f64)>,
    lean: f64,
    below: f64,
    length: f64,
}

/// The tilted frame a hole's prism stands on: origin `below` under `C`
/// along the axis `n`, `u` horizontal, `u × v = n`.
fn frame(h: &Hole) -> ([f64; 3], [f64; 3], [f64; 3], [f64; 3]) {
    let (s, c) = h.lean.to_radians().sin_cos();
    let k = (1.0f64 + 0.09).sqrt();
    let n = [0.3 * c / k, 0.3 * s / k, 1.0 / k];
    let u = [-s, c, 0.0];
    let v = [
        n[1] * u[2] - n[2] * u[1],
        n[2] * u[0] - n[0] * u[2],
        n[0] * u[1] - n[1] * u[0],
    ];
    let o = [0, 1, 2].map(|i| C[i] - h.below * n[i]);
    (o, u, v, n)
}

/// The prism's profile in its frame: the footprint projected along the
/// axis.
fn profile(h: &Hole) -> Vec<(f64, f64)> {
    let (o, u, v, _) = frame(h);
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    h.footprint
        .iter()
        .map(|&(x, y)| {
            let d = [x - o[0], y - o[1], 1.0 - o[2]];
            (dot(d, u), dot(d, v))
        })
        .collect()
}

/// The prism's volume above the plate's top: the profile's area times
/// the axial length left above the top at the profile's centroid (the
/// height above the top is affine over the profile).
fn above(h: &Hole) -> f64 {
    let (o, u, v, n) = frame(h);
    let p = profile(h);
    let (mut a, mut cx, mut cy) = (0.0, 0.0, 0.0);
    for i in 0..p.len() {
        let ((x0, y0), (x1, y1)) = (p[i], p[(i + 1) % p.len()]);
        let w = x0 * y1 - x1 * y0;
        a += w / 2.0;
        cx += (x0 + x1) * w / 6.0;
        cy += (y0 + y1) * w / 6.0;
    }
    let z = o[2] + (cx / a) * u[2] + (cy / a) * v[2];
    a * (h.length - (1.0 - z) / n[2])
}

fn prism(h: &Hole) -> AtRestBody<f64> {
    let (o, u, v, n) = frame(h);
    let mut body = Body::<f64>::new();
    prism_ops(
        &mut body,
        &profile(h),
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

/// A wedge over the sector `a0..a1` (degrees) of radius 0.4 about `C`,
/// leaning along its bisector; `k` staggers its reach and length.
fn wedge(a0: f64, a1: f64, k: usize) -> Hole {
    let at = |a: f64| {
        let (s, c) = a.to_radians().sin_cos();
        (C[0] + 0.4 * c, C[1] + 0.4 * s)
    };
    Hole {
        footprint: vec![(C[0], C[1]), at(a0), at(a1)],
        lean: (a0 + a1) / 2.0,
        below: 0.5 - 0.03 * k as f64,
        length: 1.5 - 0.11 * k as f64,
    }
}

/// An L-shaped hole whose reflex corner is `C`, leaving the quadrant
/// x < 1.5, y < 1 free, leaning away from it.
fn ell() -> Hole {
    Hole {
        footprint: vec![
            (1.0, 1.0),
            (1.5, 1.0),
            (1.5, 0.5),
            (2.0, 0.5),
            (2.0, 1.5),
            (1.0, 1.5),
        ],
        lean: 45.0,
        below: 0.43,
        length: 1.37,
    }
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

/// Every face's corners at one point are angularly disjoint about its
/// Newell normal: a loop through a point twice or more passes its
/// corners there without crossing. Each corner sweeps counterclockwise
/// from its leaving edge to its arriving one, the face on the left.
fn corners_disjoint(body: &Body<f64>) -> Result<(), String> {
    use std::f64::consts::TAU;
    for (fk, f) in body.faces() {
        let cycles = loops(body, f);
        let pt = |he| body.half_edge_start_point(he).unwrap();
        let outer: Vec<Point3<f64>> = cycles[0].iter().map(|&he| pt(he)).collect();
        let mut n = Vec3::new(0.0, 0.0, 0.0);
        for i in 0..outer.len() {
            let (a, b) = (outer[i], outer[(i + 1) % outer.len()]);
            n.x += (a.y - b.y) * (a.z + b.z);
            n.y += (a.z - b.z) * (a.x + b.x);
            n.z += (a.x - b.x) * (a.y + b.y);
        }
        let n = n.normalize();
        let seed = if n.x.abs() < 0.9 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            Vec3::new(0.0, 1.0, 0.0)
        };
        let u = n.cross(seed).normalize();
        let v = n.cross(u);
        let angle = |d: Vec3<f64>| d.dot(v).atan2(d.dot(u)).rem_euclid(TAU);
        let mut corners: BTreeMap<Point, Vec<(f64, f64)>> = BTreeMap::new();
        for cycle in &cycles {
            let m = cycle.len();
            for i in 0..m {
                let (prev, here, next) = (
                    pt(cycle[(i + m - 1) % m]),
                    pt(cycle[i]),
                    pt(cycle[(i + 1) % m]),
                );
                let from = angle(next - here);
                let sweep = (angle(prev - here) - from).rem_euclid(TAU);
                corners.entry(at(here)).or_default().push((from, sweep));
            }
        }
        for (p, cs) in corners {
            for (i, &(a, sa)) in cs.iter().enumerate() {
                for &(b, sb) in &cs[i + 1..] {
                    if (b - a).rem_euclid(TAU) < sa - 1e-9 || (a - b).rem_euclid(TAU) < sb - 1e-9 {
                        return Err(format!("{fk:?}: two corners at {p:?} overlap"));
                    }
                }
            }
        }
    }
    Ok(())
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
    let volume = 6.0 + holes.iter().map(above).sum::<f64>();
    let c = at(Point3::new(C[0], C[1], C[2]));
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
        assert_eq!(at_c, 1, "{what}: vertices at C");
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
