//! Two faces of one sphere whose own loops each hold both chart poles
//! encode no side alone (`MaterialSign::Unencoded`), and meet along a
//! seam edge. Flipping BOTH keeps the seam consistent, so tier 3 must
//! read the side off the region they form together, or the body passes
//! and measures each face's complement.
//!
//! The six ball pairs are the silent cases of the PR 4573 flip review.

#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

use crate::seam_region_common::{
    assert_refused, flipped, seam_siblings, sphere_faces, sphere_of, split_at_meridian, unencoded,
    v,
};
use geom::{Curve3, Surface};
use geom_core::{Point3, Tol, Vec3};
use std::f64::consts::PI;
use sweep::test_support::{ball_poled, brick, finished};
use topo::{Body, FaceKey, ValidationError};

#[derive(Clone, Copy)]
enum Op {
    Union,
    AMinusB,
}

struct Pair {
    label: &'static str,
    rb: f64,
    off: [f64; 3],
    pole_a: [f64; 3],
    pole_b: [f64; 3],
    op: Op,
}

const PAIRS: [Pair; 6] = [
    Pair {
        label: "#1 union",
        rb: 0.9356108706211743,
        off: [
            0.34687804548123913,
            -0.11616117805350519,
            1.3291277707606621,
        ],
        pole_a: [
            -0.6847601426471676,
            0.7225669750674681,
            -0.09487103659009685,
        ],
        pole_b: [-0.6379874658023172, -0.5755013108457552, 0.5116348646192466],
        op: Op::Union,
    },
    Pair {
        label: "#3 union",
        rb: 1.0866230267366443,
        off: [0.8079135377130012, 0.8828400076296696, -0.35713660328631297],
        pole_a: [
            -0.016334632038737213,
            -0.8322212543306335,
            -0.5542029985813015,
        ],
        pole_b: [
            0.8961109023330294,
            -0.021917098778545842,
            0.4432887225060157,
        ],
        op: Op::Union,
    },
    Pair {
        label: "#4 union",
        rb: 0.525925988272081,
        off: [
            -0.022814886360734896,
            0.6781597940678736,
            -0.8014064137549094,
        ],
        pole_a: [
            -0.005934143799678982,
            -0.9699741869032156,
            0.2431354821469103,
        ],
        pole_b: [
            0.6969720855905454,
            -0.5260051078873466,
            -0.48738951402752423,
        ],
        op: Op::Union,
    },
    Pair {
        label: "#17 union",
        rb: 1.1959135639498397,
        off: [-1.3334541896017382, 1.0735179545549363, -0.8172805636479489],
        pole_a: [
            0.5222936654658721,
            -0.026058772639571216,
            0.8523674485705931,
        ],
        pole_b: [
            -0.07064512144720063,
            -0.9011135035102402,
            -0.42778934139037483,
        ],
        op: Op::Union,
    },
    Pair {
        label: "#18 a-b",
        rb: 0.7254575054967098,
        off: [0.0897451077221434, 0.4111325960039603, -0.18472392430297094],
        pole_a: [
            -0.13237988189924904,
            -0.569230505405213,
            -0.8114506753860438,
        ],
        pole_b: [
            -0.9903860839086974,
            -0.10648145141571043,
            -0.08830121915578605,
        ],
        op: Op::AMinusB,
    },
    Pair {
        label: "#19 union",
        rb: 0.5854290847432405,
        off: [-0.8551461469261208, 0.669588404305887, 0.30159480295542057],
        pole_a: [
            -0.30871868629230154,
            -0.31702851688007083,
            -0.8967640114426861,
        ],
        pole_b: [
            0.15061005583483716,
            -0.39954451737138835,
            0.9042570374179519,
        ],
        op: Op::Union,
    },
];

/// The lens two balls of radii `r1`, `r2` at distance `d` share.
fn lens(r1: f64, r2: f64, d: f64) -> f64 {
    PI * (r1 + r2 - d).powi(2)
        * (d * d + 2.0 * d * r2 - 3.0 * r2 * r2 + 2.0 * d * r1 + 6.0 * r1 * r2 - 3.0 * r1 * r1)
        / (12.0 * d)
}

fn build(p: &Pair, tol: Tol) -> (Body<f64>, f64) {
    let a = finished("a", ball_poled(1.0, v([0.0; 3]), v(p.pole_a), tol), tol);
    let b = finished("b", ball_poled(p.rb, v(p.off), v(p.pole_b), tol), tol);
    let d = v(p.off).norm();
    let shared = lens(1.0, p.rb, d);
    let ball = |r: f64| 4.0 / 3.0 * PI * r.powi(3);
    let (res, want) = match p.op {
        Op::Union => (
            topo::boolean::union(&a, &b, tol),
            ball(1.0) + ball(p.rb) - shared,
        ),
        Op::AMinusB => (topo::boolean::subtract(&a, &b, tol), ball(1.0) - shared),
    };
    let body = Body::clone(
        &res.unwrap_or_else(|e| panic!("{}: the boolean refused: {e:?}", p.label))
            .body()
            .unwrap_or_else(|| panic!("{}: the boolean returned no body", p.label))
            .body,
    );
    (body, want)
}

#[test]
fn seam_siblings_flipped_together_are_refused_by_name() {
    let tol = Tol::witness();
    for p in &PAIRS {
        let (body, want) = build(p, tol);
        let got = topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished("the measured body", topo::Body::clone(&body), tol),
            tol,
        )
        .volume;
        assert!(
            (got - want).abs() <= 1e-9 * want,
            "{}: the unflipped body measures {got}, the closed form is {want}",
            p.label
        );
        let siblings = seam_siblings(&body, tol);
        assert!(
            !siblings.is_empty(),
            "{}: the fixture no longer holds a pair of seam siblings that encode no side",
            p.label
        );
        for [x, y] in siblings {
            assert_refused(p.label, &body, &[x, y], &[x, y], tol);
        }
    }
}

#[test]
fn a_flip_of_one_seam_sibling_is_named_by_the_regions_side() {
    let tol = Tol::witness();
    for p in &PAIRS {
        let (body, _) = build(p, tol);
        for [x, y] in seam_siblings(&body, tol) {
            for face in [x, y] {
                assert_refused(p.label, &body, &[face], &[face], tol);
            }
        }
    }
}

/// A ball cut by a slab whose planes both cross the ball's seam: its two
/// lunes each keep both poles on their loops, and together they form a
/// band bounded by two section circles, whose side is the sign of its
/// area.
#[test]
fn a_holed_region_reads_its_side_by_its_area() {
    let tol = Tol::witness();
    let ball = finished(
        "ball",
        ball_poled(1.0, v([0.0; 3]), v([0.0, 1.0, 0.0]), tol),
        tol,
    );
    let slab = finished(
        "slab",
        brick((-0.5, 0.5), (-2.0, 2.0), (-2.0, 2.0), tol),
        tol,
    );
    let body = Body::clone(
        &topo::boolean::intersect(&ball, &slab, tol)
            .unwrap()
            .body()
            .unwrap()
            .body,
    );
    let siblings = seam_siblings(&body, tol);
    assert_eq!(siblings.len(), 1, "the band is one pair of seam siblings");
    let [x, y] = siblings[0];
    let f = body.faces().find(|(k, _)| *k == x).unwrap().1;
    assert_eq!(
        topo::props::loop_edges(&body, f.outer).unwrap().0.len(),
        6,
        "each lune meets both slab planes and both seams"
    );
    for set in [vec![x], vec![y], vec![x, y]] {
        assert_refused("slab ∩ ball", &body, &set, &set, tol);
    }
}

/// A whole ball is two lunes that close the sphere: their region has no
/// boundary, so its side is the orientation of the shell they close,
/// which the volume check reads, and the sense check stays silent.
#[test]
fn a_whole_sphere_is_left_to_the_volume_check() {
    let tol = Tol::witness();
    let ball = finished(
        "ball",
        ball_poled(1.0, v([0.0; 3]), v([0.0, 1.0, 0.0]), tol),
        tol,
    );
    let faces = sphere_faces(&ball);
    assert_eq!(faces.len(), 2, "a ball is two lunes");
    let errs = topo::validate_geometric(&flipped(&ball, &faces), tol).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::NegativeVolume { .. })),
        "the inverted ball is a negative volume: {errs:?}"
    );
    assert!(
        !errs
            .iter()
            .any(|e| matches!(e, ValidationError::CurvedSenseInverted { .. })),
        "a region with no boundary encodes no side: {errs:?}"
    );
}

/// A face of one sphere split from pole to pole along the meridian
/// through the middle of its lune, by `mef`: the region it belongs to
/// then holds three faces across three seams.
fn split_along_a_meridian(
    body: &mut Body<f64>,
    face: FaceKey,
    clear_of: (Vec3<f64>, f64),
    tol: Tol,
) {
    let f = body.faces().find(|(k, _)| *k == face).unwrap().1.clone();
    let (center, radius, pole) = sphere_of(body, face);
    let (edges, _) = topo::props::loop_edges(body, f.outer).unwrap();
    let near = |a: Point3<f64>, b: Point3<f64>| (a - b).norm() < 1e-9;
    let mut seam = None;
    let mut inside = None;
    for e in &edges {
        let Curve3::Circle {
            center: c,
            axis: n,
            radius: r,
            u_ref,
        } = e.carrier
        else {
            continue;
        };
        if near(c, center) && (r - radius).abs() < 1e-9 && n.dot(pole).abs() < 1e-9 {
            seam = Some(n.normalize());
        } else {
            let mid = 0.5 * (e.t0 + e.t1);
            inside = Some(c + u_ref * (r * mid.cos()) + n.cross(u_ref) * (r * mid.sin()));
        }
    }
    let n = seam.expect("a seam meridian on the loop");
    let w = if n.dot(inside.expect("a section arc on the loop") - center) > 0.0 {
        n
    } else {
        -n
    };
    // The lune's interior directions, from one seam half to the other;
    // the first meridian clear of the cut.
    let half = n.cross(pole);
    let meridian = |phi: f64| w * phi.cos() + half * phi.sin();
    let clear = |dir: Vec3<f64>| {
        (1..32).all(|i| {
            let t = PI * f64::from(i) / 32.0;
            let q = center + (pole * t.cos() + dir * t.sin()) * radius;
            (q - Point3::new(0.0, 0.0, 0.0) - clear_of.0).norm() > clear_of.1 + 0.05
        })
    };
    let dir = (-8..=8)
        .map(|k| meridian(f64::from(k) * PI / 20.0))
        .find(|&d| clear(d))
        .expect("a meridian across the lune clear of the cut");
    split_at_meridian(body, face, dir, tol);
}

#[test]
fn a_chain_of_three_siblings_reads_one_side() {
    let tol = Tol::witness();
    let p = &PAIRS[0];
    let (mut body, want) = build(p, tol);
    let [x, _] = seam_siblings(&body, tol)[0];
    let on_a = match body.get_surface(body.faces().find(|(k, _)| *k == x).unwrap().1.surface) {
        Some(Surface::Sphere { center, .. }) => {
            (*center - Point3::new(0.0, 0.0, 0.0)).norm() < 1e-9
        }
        _ => panic!("not a sphere face"),
    };
    let other = if on_a {
        (v(p.off), p.rb)
    } else {
        (v([0.0; 3]), 1.0)
    };
    split_along_a_meridian(&mut body, x, other, tol);
    topo::validate_geometric(&body, tol).expect("the split body is valid");
    let got = topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", topo::Body::clone(&body), tol),
        tol,
    )
    .volume;
    assert!(
        (got - want).abs() <= 1e-9 * want,
        "the split body measures {got}, the closed form is {want}"
    );
    let chain: Vec<FaceKey> =
        seam_siblings(&body, tol)
            .into_iter()
            .flatten()
            .fold(Vec::new(), |mut acc, f| {
                if !acc.contains(&f) {
                    acc.push(f);
                }
                acc
            });
    assert_eq!(
        chain.len(),
        3,
        "three siblings encode no side alone: {chain:?}"
    );
    for mask in 1..8_u32 {
        let set: Vec<FaceKey> = (0..3)
            .filter(|i| mask & (1 << i) != 0)
            .map(|i| chain[i])
            .collect();
        assert_refused(p.label, &body, &set, &set, tol);
    }
}

/// A ball unioned with a brick whose face plane holds the ball's polar
/// axis keeps one 180° lune with both poles on its loop. Its flip is
/// the brick with a hemispherical dimple: the same faces, loops and
/// surfaces, a valid body. Tier 3 accepts it and the mass properties
/// measure the dimpled brick.
#[test]
fn a_lone_lune_through_both_poles_flips_into_another_valid_body() {
    let tol = Tol::witness();
    let ball = finished(
        "ball",
        ball_poled(1.0, v([0.0; 3]), v([0.0, 1.0, 0.0]), tol),
        tol,
    );
    let block = finished(
        "block",
        brick((-3.0, 0.0), (-2.0, 2.0), (-2.0, 2.0), tol),
        tol,
    );
    let body = Body::clone(
        &topo::boolean::union(&ball, &block, tol)
            .unwrap()
            .body()
            .unwrap()
            .body,
    );
    let faces = sphere_faces(&body);
    assert_eq!(faces.len(), 1, "one lune is left of the ball");
    assert!(
        unencoded(&body, faces[0], tol),
        "the lune's loop holds both poles"
    );
    let half = 2.0 * PI / 3.0;
    let measure = |b: &Body<f64>| {
        topo::validate_geometric(b, tol).expect("tier 3 accepts the body");
        topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished("the measured body", topo::Body::clone(b), tol),
            tol,
        )
        .volume
    };
    let boss = measure(&body);
    assert!(
        (boss - (48.0 + half)).abs() <= 1e-9 * boss,
        "the boss measures {boss}"
    );
    let dimple = measure(&body.flipped_face_sense_for_tests(faces[0]).unwrap());
    assert!(
        (dimple - (48.0 - half)).abs() <= 1e-9 * dimple,
        "the flipped lune measures the dimpled brick, {dimple}"
    );
}
