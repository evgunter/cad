//! Reviewer probe (reach-dual3977-r2): the corner-sliver pose widened
//! through the public API. Mounted temporarily as
//! `crates/topo/tests/probe_r2_e2e.rs` with `mod probe_r2_e2e;` in all.rs.
//! Oracle: my own closed form V∩ = (2x − d)·d²/6 (scale-free in s), and
//! membership read off the parametrisation, never the kernel.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::common;
use geom_core::{Band, Point3, Tol};
use topo::{
    intersect, mass_properties, point_in_solid, subtract, union, validate_geometric, Body,
    SolidContainment,
};

fn rot(p: [f64; 3], th: f64, shift: f64) -> Point3<f64> {
    let (c, s) = (th.cos(), th.sin());
    Point3::new(c * p[0] - s * p[1] + shift, s * p[0] + c * p[1] + shift, p[2] + 0.3 * shift)
}

#[test]
fn probe_corner_sliver_widened() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let eps = tol.eps();
    let mut bad = 0;
    for &k in &[1e-3, 1.0, 1e3] {
        for &(th, shift) in &[(0.0, 0.0), (0.5236, 0.0), (0.0, 37.0), (1.1, 4.0)] {
            for &srel in &[1e-3, 1.0] {
                let (x, l, s) = (7.0 * k, 20.0 * k, srel * k);
                let d = 500.0 * eps;
                if s < 50.0 * eps { continue; }
                let e = [[x, x, -d], [-s, 0.0, s], [0.0, -s, s]];
                let at = move |u: f64, v: f64, w: f64| {
                    rot([u * e[0][0] + v * e[1][0] + w * e[2][0],
                         u * e[0][1] + v * e[1][1] + w * e[2][1],
                         u * e[0][2] + v * e[1][2] + w * e[2][2]], th, shift)
                };
                let block = common::mapped_cube(
                    move |u, v, w| rot([u * l, v * l, (w - 1.0) * l], th, shift), tol);
                let needle = common::mapped_cube(at, tol);
                let det = s * s * (2.0 * x - d);
                let vi = (2.0 * x - d) * d * d / 6.0;
                let va = l * l * l;
                let want = [("A∩B", vi), ("B∩A", vi), ("A∪B", va + det - vi), ("B∪A", va + det - vi),
                            ("A−B", va - vi), ("B−A", det - vi)];
                let rs = [intersect(&block, &needle, tol), intersect(&needle, &block, tol),
                          union(&block, &needle, tol), union(&needle, &block, tol),
                          subtract(&block, &needle, tol), subtract(&needle, &block, tol)];
                for ((op, v), r) in want.iter().zip(rs) {
                    let tag = format!("eps={eps:e} k={k:e} th={th} shift={shift} s={s:e} {op}");
                    let body: Body<f64> = match r {
                        Ok(r) => match r.body() { Some(b) => b.body.clone(), None => { println!("PROBE {tag}: no body"); continue; } },
                        Err(e) => { println!("PROBE {tag}: refused: {e}"); bad += 1; continue; }
                    };
                    let g = validate_geometric(&body, tol);
                    let m = mass_properties(&body, tol).map(|p| p.volume);
                    // Membership: inside the sliver (f=0.5) and in the needle above the plane (f=1.5).
                    let mut mem = Vec::new();
                    for &(u, f) in &[(0.6, 0.5), (0.95, 0.5), (0.6, 1.5), (0.95, 1.5)] {
                        let vw = f * d * u / s / 2.0;
                        let q = at(u, vw, vw);
                        let in_i = f < 1.0;
                        let expect_in = match *op { "A∩B" | "B∩A" => in_i, "A−B" => false,
                                                    "B−A" => !in_i, _ => true };
                        let got = point_in_solid(&body, q, band, tol);
                        let ok = matches!((&got, expect_in), (Ok(SolidContainment::In), true) | (Ok(SolidContainment::Out), false));
                        if !ok { mem.push(format!("u={u} f={f} got {got:?} want_in={expect_in}")); }
                    }
                    if op.contains('∩') {
                        let far = rot([3.0 * l, 3.0 * l, 3.0 * l], th, shift);
                        let got = point_in_solid(&body, far, band, tol);
                        if !matches!(got, Ok(SolidContainment::Out)) { mem.push(format!("far point (3L,3L,3L) got {got:?} want Out")); }
                    }
                    let rel = m.as_ref().map(|m| (m - v).abs() / v.abs()).unwrap_or(f64::NAN);
                    let flag = g.is_err() || !mem.is_empty();
                    if flag { bad += 1; }
                    println!("PROBE {tag}: oracle={v:e} read={m:?} rel={rel:.2e} tier3={g:?} mem_bad={mem:?}{}", if flag {" <--"} else {""});
                }
            }
        }
    }
    println!("PROBE e2e flagged: {bad}");
}
