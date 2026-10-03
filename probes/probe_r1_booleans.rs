//! Reviewer probe (reach-dual3977-r1): contact9's needle poses widened —
//! needle edge scale s ∈ {1e-3, 1}, the whole fixture scaled ×{1e-3, 1, 1e3}
//! (the dip stays 500·ε), both operand orders, all four ops, each result
//! and its inside-out twin (`Body::revert`) through tier 3, the sliver's
//! inner point through `point_in_solid`, and the ∩ reused as an operand.
//! The oracle is the closed form from the corners (never the kernel).
//! Mounted in tests/all.rs as `mod probe_r1_booleans;`. Prints PROBE lines.
use crate::common;
use geom_core::{Band, Point3, Tol};
use topo::{Body, intersect, mass_properties, point_in_solid, subtract, union, validate_geometric};

fn at(e: [[f64; 3]; 3], u: f64, v: f64, w: f64) -> Point3<f64> {
    let [a, b, c] = e;
    Point3::new(u * a[0] + v * b[0] + w * c[0], u * a[1] + v * b[1] + w * c[1], u * a[2] + v * b[2] + w * c[2])
}
fn det(e: [[f64; 3]; 3]) -> f64 {
    let [a, b, c] = e;
    a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0])
}
fn short(r: Result<(), Vec<topo::ValidationError>>) -> String {
    match r { Ok(()) => "Ok".into(), Err(v) => format!("{:?}", v).chars().take(50).collect() }
}
fn line(tag: &str, b: &Body<f64>, exact: f64, q: Option<Point3<f64>>, tol: Tol) {
    let v = mass_properties(b, tol).map(|p| p.volume).unwrap_or(f64::NAN);
    let t3 = short(validate_geometric(b, tol));
    let inv = b.revert().map(|r| short(validate_geometric(&r, tol))).unwrap_or_else(|e| format!("revert {e:?}"));
    let pin = q.map(|q| format!("{:?}", point_in_solid(b, q, Band::linear(tol).unwrap(), tol))).unwrap_or_default();
    println!("PROBE|{tag}|exact={exact:+.4e}|f64={v:+.4e}|tier3={t3}|inverted_tier3={inv}|q={pin}");
}

#[test]
fn probe_r1_needle_family() {
    let tol = Tol::witness();
    let dip = 500.0 * tol.eps();
    println!("PROBE|eps={}", tol.eps());
    for &k in &[1e-3, 1.0, 1e3] {
        let block = common::brick::<f64>((0.0, 20.0 * k), (0.0, 20.0 * k), (-20.0 * k, 0.0), tol);
        let bv = 8000.0 * k * k * k;
        for &s in &[1e-3, 1.0] {
            let s = s * k;
            let e = [[7.0 * k, 7.0 * k, -dip], [-s, 0.0, s], [0.0, -s, s]];
            let needle = common::mapped_cube(move |u, v, w| at(e, u, v, w), tol);
            let d = det(e);
            let cap = d * dip * dip / (6.0 * s * s);
            let q = at(e, 0.99, 0.1 * dip / s, 0.1 * dip / s);
            let tag = format!("k={k:e}|s={s:e}");
            let res = |r: Result<topo::BooleanResult<f64>, topo::BooleanError>| -> Option<Body<f64>> {
                r.ok().and_then(|r| r.body().map(|b| b.body.clone()))
            };
            for (op, exact, a, b, qin) in [
                ("AnB", cap, &block, &needle, true),
                ("BnA", cap, &needle, &block, true),
                ("AuB", bv + d - cap, &block, &needle, true),
                ("BuA", bv + d - cap, &needle, &block, true),
                ("A-B", bv - cap, &block, &needle, false),
                ("B-A", d - cap, &needle, &block, false),
            ] {
                let r = match op { "AnB" | "BnA" => intersect(a, b, tol), "AuB" | "BuA" => union(a, b, tol), _ => subtract(a, b, tol) };
                let _ = qin;
                match res(r) {
                    Some(body) => {
                        line(&format!("{tag}|{op}"), &body, exact, Some(q), tol);
                        if op == "AnB" {
                            // the sliver reused: (A∩B) ∪ B is B, B − (A∩B) is B less the sliver.
                            match res(union(&body, &needle, tol)) {
                                Some(u) => line(&format!("{tag}|(AnB)uB"), &u, d, Some(q), tol),
                                None => println!("PROBE|{tag}|(AnB)uB|no body"),
                            }
                            match res(subtract(&needle, &body, tol)) {
                                Some(u) => line(&format!("{tag}|B-(AnB)"), &u, d - cap, Some(q), tol),
                                None => println!("PROBE|{tag}|B-(AnB)|no body"),
                            }
                        }
                    }
                    None => println!("PROBE|{tag}|{op}|no body"),
                }
            }
        }
    }
}
