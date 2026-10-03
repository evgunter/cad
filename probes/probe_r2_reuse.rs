//! Reviewer probe (reach-dual3977-r2): the contact9 corner sliver
//! (1 m edges) reused as an operand. Mounted temporarily as
//! `crates/topo/tests/probe_r2_reuse.rs` with `mod probe_r2_reuse;` in all.rs.
//! Oracle: closed forms (cube volumes, sliver (14 − d)·d²/6), disjointness.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::common;
use geom_core::{Point3, Tol};
use topo::{intersect, mass_properties, subtract, union, validate_geometric, Body};

#[test]
fn probe_sliver_reused() {
    let tol = Tol::witness();
    let d = 500.0 * tol.eps();
    for &s in &[1.0, 1e-3] {
        let e = [[7.0, 7.0, -d], [-s, 0.0, s], [0.0, -s, s]];
        let at = move |u: f64, v: f64, w: f64| Point3::new(
            u * e[0][0] + v * e[1][0] + w * e[2][0],
            u * e[0][1] + v * e[1][1] + w * e[2][1],
            u * e[0][2] + v * e[1][2] + w * e[2][2]);
        let block = common::brick::<f64>((0.0, 20.0), (0.0, 20.0), (-20.0, 0.0), tol);
        let needle = common::mapped_cube(at, tol);
        let vs = (14.0 - d) * d * d / 6.0;
        let sl: Body<f64> = intersect(&block, &needle, tol).unwrap().body().unwrap().body.clone();
        println!("PROBE s={s} eps={:e} sliver tier3={:?} vol={:?} oracle={vs:e}", tol.eps(),
                 validate_geometric(&sl, tol), mass_properties(&sl, tol).map(|p| p.volume));
        // Disjoint cube C: 2 m cube at (30,30,30); big box D enclosing everything.
        let c = common::brick::<f64>((30.0, 32.0), (30.0, 32.0), (30.0, 32.0), tol);
        let big = common::brick::<f64>((-50.0, 50.0), (-50.0, 50.0), (-50.0, 50.0), tol);
        let cases: Vec<(&str, f64, _)> = vec![
            ("sliver ∪ C", 8.0 + vs, union(&sl, &c, tol)),
            ("C ∪ sliver", 8.0 + vs, union(&c, &sl, tol)),
            ("C − sliver", 8.0, subtract(&c, &sl, tol)),
            ("sliver − C", vs, subtract(&sl, &c, tol)),
            ("sliver ∩ C", 0.0, intersect(&sl, &c, tol)),
            ("C ∩ sliver", 0.0, intersect(&c, &sl, tol)),
            ("big ∩ sliver", vs, intersect(&big, &sl, tol)),
            ("big − sliver", 1e6 - vs, subtract(&big, &sl, tol)),
            ("sliver − big", 0.0, subtract(&sl, &big, tol)),
        ];
        for (what, want, r) in cases {
            let line = match r {
                Err(e) => format!("refused: {}", e.to_string().chars().take(160).collect::<String>()),
                Ok(r) => match r.body() {
                    None => "empty result".to_string(),
                    Some(b) => {
                        let b = &b.body;
                        let v = mass_properties(b, tol).map(|p| p.volume);
                        let bad = v.as_ref().map(|v| (v - want).abs() > 1e-9 * want.abs().max(1.0)).unwrap_or(true);
                        format!("vol={v:?} want={want:e} tier3={:?}{}", validate_geometric(b, tol), if bad {"  <-- WRONG"} else {""})
                    }
                },
            };
            println!("PROBE s={s} eps={:e} {what}: {line}", tol.eps());
        }
    }
}
