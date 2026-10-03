//! **Edge-edge membership at a reflex dihedral wedge.** Two prisms over
//! fan profiles that share their corner at the origin, so their
//! vertical corner edges coincide along `z ∈ [0.3, 1]`: `a` a wedge of
//! `θa` from `+x`, `z ∈ [0, 1]`; `b` a wedge of `θb` from `φb`,
//! `z ∈ [0.3, 1.6]`. Either wedge, or both, may be reflex. Where a flank
//! of `b` runs along a flank plane of `a`, the coplanar pair is
//! flush-declared for each operand order. The closed form is the
//! profiles' common area times the common height.
//!
//! `rw_battery` prints one [`outcome`] line per pose, with no flank
//! directions parallel; `rw_coplanar_battery` puts a flank of `b` along
//! each of `a`'s flank directions and their opposites; `rw_detail` runs
//! the one pose `RW_CASE="<θa> <θb> <φb> <op>"` names (`cargo test -p
//! sweep --test all rw_detail -- --ignored --nocapture`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::test_support::{flush_declarations, prism_z};
use topo::{Body, BooleanDeclarations, BooleanError, BooleanResult};

use crate::common::differential::{area, ccw, clip_convex, outcome};

fn tol() -> Tol {
    Tol::witness()
}

/// The fan triangles of a wedge of `theta` degrees from `phi` at radius
/// `r`, each spanning at most 60°, wound counter-clockwise.
fn fan(phi: f64, theta: f64, r: f64) -> Vec<[(f64, f64); 3]> {
    let k = (theta / 60.0).ceil() as usize;
    let at = |i: usize| {
        let t = (phi + theta * i as f64 / k as f64).to_radians();
        (r * t.cos(), r * t.sin())
    };
    (0..k).map(|i| [(0.0, 0.0), at(i), at(i + 1)]).collect()
}

/// The wedge's profile: the corner, then the fan's rim.
fn profile(phi: f64, theta: f64, r: f64) -> Vec<(f64, f64)> {
    let tris = fan(phi, theta, r);
    let mut p = vec![(0.0, 0.0), tris[0][1]];
    p.extend(tris.iter().map(|t| t[2]));
    ccw(p)
}

const A_Z: (f64, f64) = (0.0, 1.0);
const B_Z: (f64, f64) = (0.3, 1.6);
const A_R: f64 = 2.0;
const B_R: f64 = 1.3;

/// The six runs: each op in both operand orders.
const OPS: [&str; 6] = ["I_ab", "I_ba", "U_ab", "U_ba", "S_ab", "S_ba"];

struct Pose {
    a: Body<f64>,
    b: Body<f64>,
    /// The flush declarations for the `(a, b)` and `(b, a)` orders.
    d: (BooleanDeclarations, BooleanDeclarations),
    want: [f64; 6],
}

fn pose(theta_a: f64, theta_b: f64, phi_b: f64) -> Pose {
    let a = prism_z::<f64>(&profile(0.0, theta_a, A_R), A_Z.0, A_Z.1, tol()).body;
    let b = prism_z::<f64>(&profile(phi_b, theta_b, B_R), B_Z.0, B_Z.1, tol()).body;
    let (ta, tb) = (fan(0.0, theta_a, A_R), fan(phi_b, theta_b, B_R));
    let common: f64 = ta
        .iter()
        .flat_map(|p| tb.iter().map(move |q| (p, q)))
        .map(|(p, q)| {
            let c = clip_convex(p, q);
            if c.len() < 3 { 0.0 } else { area(&c) }
        })
        .sum();
    let sum = |t: &[[(f64, f64); 3]]| t.iter().map(|x| area(x)).sum::<f64>();
    let va = sum(&ta) * (A_Z.1 - A_Z.0);
    let vb = sum(&tb) * (B_Z.1 - B_Z.0);
    let vi = common * (A_Z.1.min(B_Z.1) - A_Z.0.max(B_Z.0));
    let d = (
        flush_declarations(&a, &b, tol()),
        flush_declarations(&b, &a, tol()),
    );
    Pose {
        a,
        b,
        d,
        want: [vi, vi, va + vb - vi, va + vb - vi, va - vi, vb - vi],
    }
}

fn run(p: &Pose, op: &str) -> Result<BooleanResult<f64>, BooleanError> {
    let (a, b, (dab, dba), t) = (&p.a, &p.b, &p.d, tol());
    match op {
        "I_ab" => topo::intersect_with(a, b, dab, t),
        "I_ba" => topo::intersect_with(b, a, dba, t),
        "U_ab" => topo::union_with(a, b, dab, t),
        "U_ba" => topo::union_with(b, a, dba, t),
        "S_ab" => topo::subtract_with(a, b, dab, t),
        _ => topo::subtract_with(b, a, dba, t),
    }
}

#[test]
#[ignore = "detail probe: RW_CASE=\"<θa> <θb> <φb> <op>\""]
fn rw_detail() {
    let case = std::env::var("RW_CASE").expect("RW_CASE");
    let w: Vec<&str> = case.split_whitespace().collect();
    let num = |i: usize| -> f64 { w[i].parse().unwrap() };
    let p = pose(num(0), num(1), num(2));
    let k = OPS.iter().position(|o| *o == w[3]).unwrap();
    println!("{case} => {}", outcome(run(&p, w[3]), p.want[k], tol()));
}

/// `a` at five wedge angles on both sides of a half-turn, `b` at five
/// more, turned about the corner in 25° steps from 7°; all six runs.
#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn rw_battery() {
    for theta_a in [90.0, 150.0, 210.0, 270.0, 315.0] {
        for theta_b in [60.0, 135.0, 200.0, 250.0, 300.0] {
            for k in 0..15 {
                let phi_b = 7.0 + 25.0 * f64::from(k);
                let p = pose(theta_a, theta_b, phi_b);
                for (op, want) in OPS.iter().zip(p.want) {
                    println!(
                        "RW {theta_a} {theta_b} {phi_b} {op} => {}",
                        outcome(run(&p, op), want, tol())
                    );
                }
            }
        }
    }
}

/// The coplanar poses: `b`'s start or end flank along `a`'s `0°` or
/// `θa` flank (overlapping) or opposite it (touching across the edge).
#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn rw_coplanar_battery() {
    for theta_a in [90.0, 150.0, 210.0, 270.0, 315.0] {
        for theta_b in [60.0, 135.0, 200.0, 250.0, 300.0] {
            for along in [0.0, theta_a, 180.0, theta_a + 180.0] {
                for phi_b in [along, along - theta_b] {
                    let p = pose(theta_a, theta_b, phi_b);
                    for (op, want) in OPS.iter().zip(p.want) {
                        println!(
                            "RWC {theta_a} {theta_b} {phi_b} {op} => {}",
                            outcome(run(&p, op), want, tol())
                        );
                    }
                }
            }
        }
    }
}

/// **Edge-edge membership reads a reflex dihedral wedge by its
/// extent.** Each pose shares the corner edge between a convex and a
/// reflex wedge, or two reflex ones, on both sides of a half-turn; the
/// last four put a flank of `b` along a flank plane of `a`, overlapping
/// it or opposite it across the edge. Every op in both operand orders
/// builds a body that passes tiers 2 and 3′ and the at-rest
/// certificate, has the closed-form volume, and is a legal operand.
#[test]
fn edge_edge_sites_at_a_reflex_wedge_build_sound() {
    for (theta_a, theta_b, phi_b) in [
        (315.0, 300.0, 357.0),
        (90.0, 250.0, 32.0),
        (270.0, 60.0, 82.0),
        (210.0, 200.0, 207.0),
        (150.0, 300.0, 157.0),
        (270.0, 135.0, 180.0),
        (90.0, 250.0, 270.0),
        (315.0, 60.0, 315.0),
        (210.0, 300.0, 390.0),
    ] {
        let p = pose(theta_a, theta_b, phi_b);
        for (op, want) in OPS.iter().zip(p.want) {
            let what = format!("θa {theta_a}° θb {theta_b}° φb {phi_b}° {op}");
            let bb = match run(&p, op) {
                Ok(BooleanResult::Body(bb)) => bb,
                other => panic!("{what}: {other:?}"),
            };
            assert_eq!(topo::validate_closed(&bb.body), Ok(()), "{what}: tier 2");
            assert_eq!(
                topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
                Ok(()),
                "{what}: tier 3′"
            );
            assert!(
                topo::validate_geometric_certificate(&bb.body, tol()).is_ok(),
                "{what}: the at-rest certificate"
            );
            let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
            assert!(
                (v - want).abs() < 1e-9,
                "{what}: volume {v} against the closed form {want}"
            );
            sweep::test_support::assert_legal_operand(&what, &bb.body, tol());
        }
    }
}
