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

use common::meeting::{
    Hole, MEET, PLATE, corners_disjoint, cycles_of, ell_and_wedges, four_wedges, inner_rows, notch,
    notch_rows, three_wedges, two_wedges, wedge, wedges_on_one_side,
};
use common::{FaceGeometry, describe_as_intersections, finished, prism_ops};
use geom_core::{Point3, Tol};
use topo::{AtRestBody, Body, BooleanResult, union, validate_geometric, validate_pseudomanifold};

fn t() -> Tol {
    Tol::witness()
}

type Point = (i64, i64, i64);

fn at(p: Point3<f64>) -> Point {
    let n = |x: f64| (x * 1e6).round() as i64;
    (n(p.x), n(p.y), n(p.z))
}

/// A body's geometry, key-free: each face by the points of its loops'
/// vertices, each edge by its ends' points, as sorted multisets.
fn shape(body: &Body<f64>) -> (Vec<Vec<Point>>, Vec<[Point; 2]>) {
    let pt = |he| at(body.half_edge_start_point(he).unwrap());
    let mut faces: Vec<Vec<Point>> = body
        .faces()
        .map(|(_, f)| {
            let mut ps: Vec<Point> = cycles_of(body, f).into_iter().flatten().map(pt).collect();
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
    let rest = Pose::rest();
    let mut members = vec![posed_box("the plate", PLATE, &rest)];
    members.extend(holes.iter().map(|h| posed_prism(h, &rest)));
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
    every_order("two wedges", &two_wedges(), [14, 30, 19]);
    every_order("three wedges", &three_wedges(), [18, 39, 24]);
    every_order("four wedges", &four_wedges(), [22, 48, 29]);
}

/// **Holes meeting at a vertex with a reflex sector there build one body
/// in every member order**: three wedges on one side, leaving the top a
/// reflex sector between them, and an L-shaped hole whose reflex corner
/// is the vertex, with two wedges in the quadrant it leaves.
#[test]
fn holes_with_a_reflex_sector_at_their_vertex_build_one_body_in_every_member_order() {
    every_order(
        "three wedges on one side",
        &wedges_on_one_side(),
        [18, 39, 24],
    );
    every_order("an L and two wedges", &ell_and_wedges(), [21, 48, 30]);
}

/// A rigid motion of the whole scene: `x ↦ r x + t`.
struct Pose {
    label: &'static str,
    r: [[f64; 3]; 3],
    t: [f64; 3],
}

impl Pose {
    /// The turn by `angle` about `axis`, then the shift `t`.
    fn turn(label: &'static str, axis: [f64; 3], angle: f64, t: [f64; 3]) -> Self {
        let l = axis.iter().map(|a| a * a).sum::<f64>().sqrt();
        let [x, y, z] = axis.map(|a| a / l);
        let (s, c) = angle.sin_cos();
        let d = 1.0 - c;
        let r = [
            [c + x * x * d, x * y * d - z * s, x * z * d + y * s],
            [y * x * d + z * s, c + y * y * d, y * z * d - x * s],
            [z * x * d - y * s, z * y * d + x * s, c + z * z * d],
        ];
        Self { label, r, t }
    }

    /// The pose then `self`.
    fn after(self, first: &Self) -> Self {
        let r = [0, 1, 2]
            .map(|i| [0, 1, 2].map(|j| (0..3).map(|k| self.r[i][k] * first.r[k][j]).sum::<f64>()));
        let t =
            [0, 1, 2].map(|i| (0..3).map(|k| self.r[i][k] * first.t[k]).sum::<f64>() + self.t[i]);
        Self {
            label: self.label,
            r,
            t,
        }
    }

    fn at(&self, p: [f64; 3]) -> Point3<f64> {
        let q = [0, 1, 2].map(|i| (0..3).map(|k| self.r[i][k] * p[k]).sum::<f64>() + self.t[i]);
        Point3::new(q[0], q[1], q[2])
    }
}

impl Pose {
    /// The identity.
    fn rest() -> Self {
        Self::turn("at rest", [0.0, 0.0, 1.0], 0.0, [0.0, 0.0, 0.0])
    }
}

/// The scene's poses: at rest, turned about the top's normal, turned in
/// general, and flipped (the top facing −z) at rest and turned.
fn poses() -> Vec<Pose> {
    let flip = || {
        Pose::turn(
            "flipped",
            [1.0, 0.0, 0.0],
            std::f64::consts::PI,
            [0.0, 0.0, 0.0],
        )
    };
    vec![
        Pose::rest(),
        Pose::turn("turned about z", [0.0, 0.0, 1.0], 0.65, [0.0, 0.0, 0.0]),
        Pose::turn("turned", [1.0, 2.0, 3.0], 0.7, [0.3, -0.2, 0.5]),
        flip(),
        Pose::turn(
            "flipped and turned",
            [-2.0, 1.0, 1.0],
            1.1,
            [-0.4, 0.1, 0.3],
        )
        .after(&flip()),
    ]
}

/// A box `[x, y, z]` placed by `pose`.
fn posed_box(what: &str, b: [(f64, f64); 3], pose: &Pose) -> AtRestBody<f64> {
    let [(x0, x1), (y0, y1), z] = b;
    let mut body = Body::<f64>::new();
    prism_ops(
        &mut body,
        &[(x0, y0), (x1, y0), (x1, y1), (x0, y1)],
        z,
        |x, y, z| pose.at([x, y, z]),
        FaceGeometry::Certified,
        t(),
    );
    describe_as_intersections(&mut body, t());
    finished(what, body, t())
}

/// A hole's prism placed by `pose`.
fn posed_prism(h: &Hole, pose: &Pose) -> AtRestBody<f64> {
    let [o, u, v, n] = h.frame();
    let mut body = Body::<f64>::new();
    prism_ops(
        &mut body,
        &h.profile(),
        (0.0, h.length),
        |x, y, z| pose.at([0, 1, 2].map(|i| o[i] + x * u[i] + y * v[i] + z * n[i])),
        FaceGeometry::Certified,
        t(),
    );
    describe_as_intersections(&mut body, t());
    finished("a tilted prism", body, t())
}

fn body(what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(r)) => r.body,
        Ok(BooleanResult::Empty) => panic!("{what}: empty"),
        Err(e) => panic!("{what}: refused: {e:?}"),
    }
}

fn volume(b: &AtRestBody<f64>) -> f64 {
    topo::mass_properties(b, t()).unwrap().volume
}

/// Asserts `b` is tier-3 valid with `want`, its corners disjoint, and
/// that a block across the meeting point unions with it.
fn sound(what: &str, b: &AtRestBody<f64>, want: f64, pose: &Pose) {
    assert_eq!(validate_geometric(b, t()), Ok(()), "{what}: tier 3");
    let v = volume(b);
    assert!(
        (v - want).abs() < 1e-9,
        "{what}: volume {v}, expected {want}"
    );
    assert_eq!(corners_disjoint(b), Ok(()), "{what}: corners");
    let block = posed_box(
        "a block across the meeting point",
        [(1.21, 1.77), (0.68, 1.31), (0.86, 1.52)],
        pose,
    );
    body(&format!("{what}, then ∪ a block"), union(b, &block, t()));
}

/// **The plate against the union of the holes' prisms, in every op and
/// five poses.** The plate's top less the holes passes the meeting point
/// once per hole: as one ring where the holes lie inside it
/// ([`inner_rows`]), as the outer loops of several faces of one plane
/// where some notch its boundary ([`notch_rows`]).
/// - U − P (the prisms above the top) and both intersections (inside
///   it) build sound;
/// - P − U, in one boolean, has its zips fuse the meeting point twice,
///   and the zip crosses two corners of that boundary first: by `kemr`
///   on one ring, by `kef` across faces of one plane. With two holes it
///   builds as on main, and this row pins that shape, which is JOIN's
///   at-rest one for holes meeting at a point: two rings through one
///   vertex, whose corners there overlap, so it is not asserted sound
///   (`work/join/two-representations-of-holes-meeting-at-a-point.md`).
///   With three or more holes some face passes the point three times or
///   more once the copies fuse, and it refuses `PinchOfManyHolesInOneRing`,
///   counted over every vertex fused onto the point: a crossing on any
///   face there moves the top's corners to a vertex fused back later
///   ("a notch and two wedges apart" and "a wide notch and two wedges",
///   which a count at one vertex let through crossed). One geometry of
///   that class,
///   "a notch and two wedges", happens to offer no crossing at all and
///   refuses `PinchUncrossed`; it is pinned as that one pose, and the
///   grids below are the class;
/// - the plate less each prism in turn builds the same volume sound,
///   the meeting point a vertex per hole.
///
/// Volumes are inclusion-exclusion against U's, and against each hole's
/// closed form above the top where the holes lie inside it.
#[test]
fn the_plate_against_the_holes_union_builds_sound_or_refuses_typed_in_every_op() {
    use topo::{intersect, subtract};
    let inner = inner_rows().len();
    for pose in poses() {
        let p = posed_box("the plate", PLATE, &pose);
        for (r, (fixture, holes)) in inner_rows().into_iter().chain(notch_rows()).enumerate() {
            let label = format!("{fixture}, {}", pose.label);
            let prisms: Vec<_> = holes.iter().map(|h| posed_prism(h, &pose)).collect();
            let u = prisms[1..].iter().fold(prisms[0].clone(), |u, q| {
                body(&format!("{label}: the prisms' union"), union(&u, q, t()))
            });
            let inside = volume(&body(&label, intersect(&p, &u, t())));
            if r < inner {
                let above: f64 = holes.iter().map(Hole::above).sum();
                assert!(
                    (volume(&u) - above - inside).abs() < 1e-9,
                    "{label}: P ∩ U against the holes' closed form"
                );
            }
            sound(
                &format!("{label}: U − P"),
                &body(&label, subtract(&u, &p, t())),
                volume(&u) - inside,
                &pose,
            );
            for (what, r) in [
                ("P ∩ U", intersect(&p, &u, t())),
                ("U ∩ P", intersect(&u, &p, t())),
            ] {
                sound(&format!("{label}: {what}"), &body(&label, r), inside, &pose);
            }
            match (holes.len(), subtract(&p, &u, t())) {
                // This one geometry offers no crossing (no `kemr` or `kef`
                // joins an outer loop to a ring there); most of its class
                // reaches one and refuses as above (the grids).
                (_, r) if fixture == "a notch and two wedges" => assert!(
                    matches!(r, Err(topo::BooleanError::PinchUncrossed { .. })),
                    "{label}: P − U refuses PinchUncrossed, got {:?}",
                    r.map(|_| ())
                ),
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
                    let v = volume(&b);
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
            sound(
                &format!("{label}: P less each prism"),
                &seq,
                6.0 - inside,
                &pose,
            );
        }
    }
}

/// The P − U grid at `k` holes (review 3's sweep, ported): 30° holes in `k` of eight 45° slots
/// round the meeting point (the first slot always taken, as turning
/// the scene about the top's normal moves no answer), each hole a
/// wedge or a notch past the plate's edge, at rest. Every
/// configuration builds sound (tiers 3 and 3′, `corners_disjoint`, the
/// volume against P ∩ U) or refuses typed, the union of its prisms
/// included. Returns how many built.
fn grid(k: usize) -> usize {
    use topo::{intersect, subtract};
    fn slots(from: usize, k: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if cur.len() == k {
            out.push(cur.clone());
            return;
        }
        for s in from..8 {
            cur.push(s);
            slots(s + 1, k, cur, out);
            cur.pop();
        }
    }
    let mut combos = Vec::new();
    slots(1, k, &mut vec![0], &mut combos);
    let rest = Pose::rest();
    let p = posed_box("the plate", PLATE, &rest);
    let mut built = 0;
    for c in combos {
        for mask in 0..1u32 << k {
            let label = format!("slots {c:?}, notches {mask:0k$b}");
            let holes: Vec<Hole> = c
                .iter()
                .enumerate()
                .map(|(j, &s)| {
                    let a = 45.0f64.mul_add(s as f64, 5.0);
                    if mask >> j & 1 == 1 {
                        notch(a, a + 30.0, j, 2.0)
                    } else {
                        wedge(a, a + 30.0, j)
                    }
                })
                .collect();
            let prisms: Vec<_> = holes.iter().map(|h| posed_prism(h, &rest)).collect();
            let mut u = prisms[0].clone();
            let mut refused = false;
            for q in &prisms[1..] {
                match union(&u, q, t()) {
                    Ok(BooleanResult::Body(r)) => u = r.body,
                    Ok(BooleanResult::Empty) => panic!("{label}: the prisms' union is empty"),
                    Err(_) => {
                        refused = true;
                        break;
                    }
                }
            }
            if refused {
                continue;
            }
            let inside = match intersect(&p, &u, t()) {
                Ok(BooleanResult::Body(r)) => Some(volume(&r.body)),
                Ok(BooleanResult::Empty) => panic!("{label}: P ∩ U is empty"),
                Err(_) => None,
            };
            match subtract(&p, &u, t()) {
                Ok(BooleanResult::Body(r)) => {
                    built += 1;
                    let b = r.body;
                    assert_eq!(
                        validate_geometric(&b, t()),
                        Ok(()),
                        "{label}: P − U, tier 3"
                    );
                    assert!(
                        validate_pseudomanifold(&b, &r.contacts, t()).is_ok(),
                        "{label}: P − U, tier 3′"
                    );
                    assert_eq!(corners_disjoint(&b), Ok(()), "{label}: P − U, corners");
                    if let Some(i) = inside {
                        let v = volume(&b);
                        assert!((v - (6.0 - i)).abs() < 1e-8, "{label}: P − U, volume {v}");
                    }
                }
                Ok(BooleanResult::Empty) => panic!("{label}: P − U is empty"),
                Err(_) => {}
            }
        }
    }
    built
}

/// **Three holes in every slot pattern and every mix of wedges and
/// notches build P − U sound or refuse typed** ([`grid`]).
#[test]
fn every_three_hole_grid_configuration_builds_sound_or_refuses_typed() {
    grid(3);
}

/// **Four holes in every slot pattern and every mix of wedges and
/// notches build P − U sound or refuse typed** ([`grid`]).
#[test]
fn every_four_hole_grid_configuration_builds_sound_or_refuses_typed() {
    grid(4);
}
