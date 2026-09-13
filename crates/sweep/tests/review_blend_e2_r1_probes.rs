//! **Review probes, BLEND unit 2 (PR 2122)** — the rewritten
//! `Extruded::top` / `Extruded::bottom` field docs read off BUILT
//! bodies, for both signs of `w · n`, through both extrusion doors, on
//! an axis-aligned and a tilted sketch plane.
//!
//! The sentences under test (`crates/sweep/src/extrude.rs`, the two
//! field docs): `top` is the cap on the sketch plane translated by `w`,
//! outward along `w`, on the `−n` side of the sketch plane under
//! `w · n < 0`; `bottom` is on the sketch plane itself, outward opposite
//! `w`; the profile's canonical winding sits on `top` exactly when
//! `w · n > 0` and on `bottom` otherwise. Every reading here is taken
//! from stored data — vertex positions, the cap's stored plane and the
//! face's `sense` bit — never from a recomputed Newell normal, so a row
//! going red means the body and the sentence disagree, not that two
//! derivations of the same thing do.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane, ValidatedProfile};
use sweep::{Extruded, Extrusion, extrude};
use topo::{Body, FaceKey, LoopBoundary, LoopKey, validate_geometric};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The 6-vertex L: no rotation of its reversed chain equals the
/// forward one (the signed area flips), so "which cap carries the
/// canonical winding" is a question with exactly one answer.
fn l_loop() -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 1.0),
        p2(1.0, 1.0),
        p2(1.0, 2.0),
        p2(0.0, 2.0),
    ])
}

/// A small square, counterclockwise as written; validation makes it
/// a hole (clockwise) when it sits inside the outer loop. Four
/// vertices, not a two-arc circle: a two-vertex chain's winding lives
/// in its bulges and its vertex polygon has no signed area to read.
fn square_hole(lo: f64, hi: f64) -> ProfileLoop<f64> {
    ProfileLoop::polygon([p2(lo, lo), p2(hi, lo), p2(hi, hi), p2(lo, hi)])
}

fn validated(plane: SketchPlane<f64>, loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(plane, loops).validate(Tol::witness()).unwrap()
}

/// The sketch planes the rows run on: the canonical one and a rigid
/// frame whose normal has three nonzero components, so a claim about
/// `n` cannot pass by being a claim about world `z`.
fn planes() -> Vec<(&'static str, SketchPlane<f64>)> {
    vec![
        ("xy", SketchPlane::xy()),
        (
            "tilted",
            SketchPlane::from_frame(
                Point3::new(0.3, -0.2, 0.7),
                Vec3::new(1.0, 1.0, 0.0).normalize(),
                Vec3::new(-1.0, 1.0, 2.0).normalize(),
            ),
        ),
    ]
}

/// Start-vertex positions of a loop's cycle in `next` order.
fn cycle_points(body: &Body<f64>, lp: LoopKey) -> Vec<Point3<f64>> {
    let LoopBoundary::Cycle { first } = body.get_loop(lp).unwrap().boundary else {
        panic!("cap loop has no cycle");
    };
    body.loop_cycle(first)
        .unwrap()
        .iter()
        .map(|&he| {
            let v = body.get_half_edge(he).unwrap().start;
            *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
        })
        .collect()
}

/// A cap's outward normal read off STORED data: the plane's chart
/// normal, flipped by the face's `sense` bit.
fn stored_outward(body: &Body<f64>, face: FaceKey) -> Vec3<f64> {
    let f = body.get_face(face).unwrap();
    let Surface::Plane { normal, .. } = body.get_surface(f.surface).unwrap() else {
        panic!("an extrusion cap is a plane");
    };
    if f.sense { *normal } else { -*normal }
}

/// Signed offsets of a cap's outer-loop vertices from the sketch
/// plane along `n`.
fn offsets_along_n(body: &Body<f64>, face: FaceKey, plane: &SketchPlane<f64>) -> Vec<f64> {
    let o = plane.origin();
    let n = plane.normal();
    cycle_points(body, body.get_face(face).unwrap().outer)
        .iter()
        .map(|p| (*p - o).dot(n))
        .collect()
}

/// A world point back in sketch coordinates.
fn in_sketch(plane: &SketchPlane<f64>, p: Point3<f64>) -> Point2<f64> {
    let d = p - plane.origin();
    p2(d.dot(plane.u()), d.dot(plane.v()))
}

fn near2(a: Point2<f64>, b: Point2<f64>) -> bool {
    (a.x - b.x).abs() < 1e-9 && (a.y - b.y).abs() < 1e-9
}

/// `a` equals some rotation of `b`.
fn cyclic_eq(a: &[Point2<f64>], b: &[Point2<f64>]) -> bool {
    a.len() == b.len()
        && (0..a.len()).any(|k| (0..a.len()).all(|i| near2(a[(i + k) % a.len()], b[i])))
}

/// `[v0, v_{n−1}, …, v1]` — the profile crate's reversal of a chain.
fn reversed(c: &[Point2<f64>]) -> Vec<Point2<f64>> {
    let n = c.len();
    (0..n).map(|j| c[(n - j) % n]).collect()
}

/// Twice the signed area of a closed chain in sketch coordinates:
/// positive iff counterclockwise viewed from the tip of `n`.
fn signed_area2(c: &[Point2<f64>]) -> f64 {
    let n = c.len();
    (0..n)
        .map(|i| {
            let (a, b) = (c[i], c[(i + 1) % n]);
            a.x * b.y - b.x * a.y
        })
        .sum()
}

fn cap_chain(t: &Extruded<f64>, plane: &SketchPlane<f64>, lp: LoopKey) -> Vec<Point2<f64>> {
    cycle_points(&t.body, lp)
        .into_iter()
        .map(|p| in_sketch(plane, p))
        .collect()
}

/// Both doors reaching the same signed `w · n`.
fn doors(plane: &SketchPlane<f64>, d: f64) -> Vec<(&'static str, Extrusion<f64>)> {
    vec![
        ("Distance", Extrusion::Distance(d)),
        ("Vector", Extrusion::Vector(plane.normal() * d)),
    ]
}

/// **`top` is the far end, `bottom` the near end — for both signs.**
/// `top`'s vertices sit at signed offset `d` along `n` (so on the `−n`
/// side, below `bottom`, when `d < 0`); `bottom`'s sit on the sketch
/// plane; the stored outward normals run along `w` and against it.
#[test]
fn cap_planes_are_the_sweep_ends_for_both_signs_both_doors_two_planes() {
    for (pname, plane) in planes() {
        let vp = validated(plane, vec![l_loop()]);
        for d in [0.8, -0.8] {
            for (door, ext) in doors(&plane, d) {
                let tag = format!("{pname}/{door}/d = {d}");
                let t = extrude(&vp, ext, Tol::witness()).unwrap();
                assert_eq!(validate_geometric(&t.body, Tol::witness()), Ok(()), "{tag}");
                let w = plane.normal() * d;
                let w_hat = w.normalize();

                // Positions: top at offset d, bottom at offset 0.
                let top = offsets_along_n(&t.body, t.top, &plane);
                let bot = offsets_along_n(&t.body, t.bottom, &plane);
                assert!(
                    top.iter().all(|&h| (h - d).abs() < 1e-9),
                    "{tag}: top {top:?}"
                );
                assert!(bot.iter().all(|&h| h.abs() < 1e-9), "{tag}: bottom {bot:?}");
                if d < 0.0 {
                    // The sentence's `−n` half, stated as the body shows it.
                    assert!(
                        top.iter().all(|&h| h < 0.0),
                        "{tag}: top not on the −n side"
                    );
                    assert!(
                        top.iter().all(|&h| h < bot[0]),
                        "{tag}: top not below bottom along n"
                    );
                }

                // Stored outward normals: top along w, bottom against it.
                let on_top = stored_outward(&t.body, t.top).dot(w_hat);
                let on_bot = stored_outward(&t.body, t.bottom).dot(w_hat);
                assert!(on_top > 1.0 - 1e-9, "{tag}: top outward · ŵ = {on_top}");
                assert!(on_bot < -1.0 + 1e-9, "{tag}: bottom outward · ŵ = {on_bot}");
            }
        }
    }
}

/// **The canonical winding sits on `top` iff `w · n > 0`, on `bottom`
/// iff `w · n < 0`** — read as the cap's outer cycle (and its hole
/// ring) being a rotation of the profile's canonical chain, while the
/// other cap's is a rotation of the reversed chain. The signed area is
/// checked beside the chain match so the row cannot pass on a chain
/// that happens to be its own reversal.
#[test]
fn canonical_winding_is_on_top_iff_w_dot_n_positive() {
    for (pname, plane) in planes() {
        let vp = validated(plane, vec![l_loop(), square_hole(0.3, 0.7)]);
        let outer: Vec<Point2<f64>> = vp.loops()[0].vertices().iter().map(|v| v.pos()).collect();
        let hole: Vec<Point2<f64>> = vp.loops()[1].vertices().iter().map(|v| v.pos()).collect();
        assert!(
            signed_area2(&outer) > 0.0,
            "canonical outer is counterclockwise"
        );
        assert!(signed_area2(&hole) < 0.0, "canonical hole is clockwise");

        for d in [0.8, -0.8] {
            for (door, ext) in doors(&plane, d) {
                let tag = format!("{pname}/{door}/d = {d}");
                let t = extrude(&vp, ext, Tol::witness()).unwrap();
                assert_eq!(validate_geometric(&t.body, Tol::witness()), Ok(()), "{tag}");
                let (carrier, other) = if d > 0.0 {
                    (t.top, t.bottom)
                } else {
                    (t.bottom, t.top)
                };
                let (cf, of) = (
                    t.body.get_face(carrier).unwrap(),
                    t.body.get_face(other).unwrap(),
                );
                assert_eq!(cf.rings.len(), 1, "{tag}");
                assert_eq!(of.rings.len(), 1, "{tag}");

                let c_outer = cap_chain(&t, &plane, cf.outer);
                let c_ring = cap_chain(&t, &plane, cf.rings[0]);
                let o_outer = cap_chain(&t, &plane, of.outer);
                let o_ring = cap_chain(&t, &plane, of.rings[0]);

                assert!(
                    cyclic_eq(&c_outer, &outer),
                    "{tag}: carrier outer ≠ canonical"
                );
                assert!(
                    cyclic_eq(&c_ring, &hole),
                    "{tag}: carrier ring ≠ canonical hole"
                );
                assert!(signed_area2(&c_outer) > 0.0, "{tag}: carrier outer not CCW");
                assert!(
                    cyclic_eq(&o_outer, &reversed(&outer)),
                    "{tag}: other outer ≠ reversed"
                );
                assert!(
                    cyclic_eq(&o_ring, &reversed(&hole)),
                    "{tag}: other ring ≠ reversed hole"
                );
                assert!(signed_area2(&o_outer) < 0.0, "{tag}: other outer not CW");
            }
        }
    }
}
