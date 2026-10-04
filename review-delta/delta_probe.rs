//! Delta review of PR 4026: the shallow200 near-tangent pose, every op
//! and order, with an independent in-band census.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom_core::{Point3, Tol};
use topo::test_support as fixtures;
use topo::{Body, BooleanDeclarations, BooleanError, BooleanResult, mass_properties};

use crate::common::differential::outcome;
use crate::delta_inband::inband;

type V3 = [f64; 3];
fn tol() -> Tol { Tol::witness() }
fn dot(a: V3, b: V3) -> f64 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
fn cross(a: V3, b: V3) -> V3 { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] }
fn unit(a: V3) -> V3 { let n = dot(a, a).sqrt(); a.map(|c| c / n) }
fn basis(m: V3) -> (V3, V3) {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
    let u = unit(cross(seed, m));
    (u, cross(m, u))
}

/// Clip-free oracle: the shallow prism's two convex pieces clipped by
/// the cube's half-spaces is the PR row's `convex_volume`; here we just
/// report the kernel's volume and leave the oracle to the batteries.
type Op = fn(&Body<f64>, &Body<f64>, &BooleanDeclarations, Tol) -> Result<BooleanResult<f64>, BooleanError>;

pub fn shallow_pose(tilt: f64, a: f64, edge: usize) -> (Body<f64>, Body<f64>) {
    let profile = [(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (2.0, 0.6), (0.0, 1.0)];
    let v = [2.0, 0.6, 1.0];
    let edges = [[2.0, 0.4, 0.0], [-2.0, 0.4, 0.0], [0.0, 0.0, -1.0]];
    let e = unit(edges[edge]);
    let (p1, p2) = basis(e);
    let al = std::f64::consts::TAU * (a + 0.25) / 16.0;
    let m = unit([0, 1, 2].map(|t| p1[t] * al.cos() + p2[t] * al.sin() + e[t] * tilt));
    let m = unit(m);
    let (u, w) = basis(m);
    let cube = fixtures::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (-2.0 + 4.0 * x, -2.0 + 4.0 * y, 4.0 * z);
            if std::env::var("DP_ROWASSOC").is_ok() {
                Point3::new(
                    v[0] + a * u[0] + b * w[0] + c * m[0],
                    v[1] + a * u[1] + b * w[1] + c * m[1],
                    v[2] + a * u[2] + b * w[2] + c * m[2],
                )
            } else {
                Point3::new(
                    v[0] + (u[0] * a + (w[0] * b + m[0] * c)),
                    v[1] + (u[1] * a + (w[1] * b + m[1] * c)),
                    v[2] + (u[2] * a + (w[2] * b + m[2] * c)),
                )
            }
        },
        tol(),
    );
    (fixtures::prism::<f64>(&profile, 1.0, tol()).body, cube)
}

#[test]
#[ignore = "delta review probe"]
fn delta_shallow_pose() {
    let band = crate::delta_inband::band(tol());
    let tilt: f64 = std::env::var("DP_TILT").map(|s| s.parse().unwrap()).unwrap_or(1e-7);
    let a: f64 = std::env::var("DP_A").map(|s| s.parse().unwrap()).unwrap_or(3.0);
    let edge: usize = std::env::var("DP_E").map(|s| s.parse().unwrap()).unwrap_or(0);
    println!("eps={} band={band:e} tilt={tilt:e}", tol().eps());
    let (prism, cube) = shallow_pose(tilt, a, edge);
    let decls = BooleanDeclarations::default();
    let vol = |b: &Body<f64>| mass_properties(b, tol()).unwrap().volume;
    let (va, vb) = (vol(&prism), vol(&cube));
    for (order, x, y) in [("pc", &prism, &cube), ("cp", &cube, &prism)] {
        let ops: [(&str, Op); 3] = [("U", topo::union_with), ("I", topo::intersect_with), ("S", topo::subtract_with)];
        for (op, f) in ops {
            let r = f(x, y, &decls, tol());
            match &r {
                Ok(res) => match res.body() {
                    Some(bb) => {
                        let t3 = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol());
                        println!("{order} {op}: built v={} (va={va} vb={vb}) t3p={t3:?}", vol(&bb.body));
                        println!("   contacts.vv={:?} a_on_b={} b_on_a={}", bb.contacts.vv, bb.contacts.a_on_b.len(), bb.contacts.b_on_a.len());
                        for l in inband(&bb.body, &bb.contacts, tol()) {
                            println!("   IND {l}");
                        }
                    }
                    None => println!("{order} {op}: empty"),
                },
                Err(e) => println!("{order} {op}: ERR {}", format!("{e:?}").chars().take(140).collect::<String>()),
            }
        }
    }
}
