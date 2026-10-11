//! Two faces of one sphere whose own loops each hold both chart poles
//! encode no side alone (`MaterialSign::Unencoded`), and meet along a
//! seam edge. Flipping BOTH keeps the seam consistent, so tier 3 must
//! read the side off the region they form together, or the body passes
//! and measures each face's complement.
//!
//! The six ball pairs are the silent cases of the PR 4573 flip review.

use geom_core::{Tol, Vec3};
use std::f64::consts::PI;
use sweep::test_support::{ball_poled, finished};
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

fn v([x, y, z]: [f64; 3]) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

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

fn unencoded(body: &Body<f64>, face: FaceKey, tol: Tol) -> bool {
    let f = body.faces().find(|(k, _)| *k == face).unwrap().1;
    let (outer, _) = topo::props::loop_edges(body, f.outer).unwrap();
    let band = geom_core::Band::linear(tol).unwrap();
    f.rings.is_empty()
        && geom_brep::props::boundary_material_sign(
            body.get_surface(f.surface).unwrap(),
            &outer,
            band,
        ) == Ok(geom_brep::props::MaterialSign::Unencoded)
}

/// The pairs of faces on one sphere surface that share an edge and
/// each encode no side alone.
fn seam_siblings(body: &Body<f64>, tol: Tol) -> Vec<[FaceKey; 2]> {
    let mut pairs = Vec::new();
    for (k, f) in body.faces() {
        if !matches!(
            body.get_surface(f.surface),
            Some(geom::Surface::Sphere { .. })
        ) || !unencoded(body, k, tol)
        {
            continue;
        }
        let (_, hes) = topo::props::loop_edges(body, f.outer).unwrap();
        for h in hes {
            let other = body.face_of_half_edge(body.mate(h).unwrap()).unwrap();
            let of = body.faces().find(|(kk, _)| *kk == other).unwrap().1;
            if other > k && of.surface == f.surface && unencoded(body, other, tol) {
                let pair = [k, other];
                if !pairs.contains(&pair) {
                    pairs.push(pair);
                }
            }
        }
    }
    pairs
}

#[test]
fn seam_siblings_flipped_together_are_refused_by_name() {
    let tol = Tol::witness();
    for p in &PAIRS {
        let (body, want) = build(p, tol);
        let got = topo::mass_properties(&body, tol)
            .unwrap_or_else(|e| panic!("{}: the unflipped body does not measure: {e:?}", p.label))
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
            let flipped = body
                .flipped_face_sense_for_tests(x)
                .and_then(|b| b.flipped_face_sense_for_tests(y))
                .unwrap();
            let errs = topo::validate_geometric(&flipped, tol)
                .err()
                .unwrap_or_else(|| {
                    panic!(
                        "{}: flipping both siblings {x:?} {y:?} passes tier 3 (measures {:?})",
                        p.label,
                        topo::mass_properties(&flipped, tol).map(|m| m.volume)
                    )
                });
            for face in [x, y] {
                assert!(
                    errs.iter().any(
                        |e| matches!(e, ValidationError::CurvedSenseInverted { face: f } if *f == face)
                    ),
                    "{}: flipping both siblings does not name {face:?} CurvedSenseInverted: {errs:?}",
                    p.label
                );
            }
        }
    }
}
