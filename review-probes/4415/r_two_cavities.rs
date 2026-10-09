//! Reviewer probe (PR 4415): P0 repro, and a result with two in-band
//! cavities of different V/A. `R_D1`, `R_D2` tilts; `R_MODE` = p0|two|one.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Point3, Tol};
use topo::test_support as fx;
use topo::{AtRestBody, BooleanDeclarations, BooleanError};
type V3 = [f64; 3];
fn dot(a: V3, b: V3) -> f64 { a[0]*b[0]+a[1]*b[1]+a[2]*b[2] }
fn cross(a: V3, b: V3) -> V3 { [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]] }
fn add(a: V3, b: V3) -> V3 { [a[0]+b[0], a[1]+b[1], a[2]+b[2]] }
fn scale(a: V3, s: f64) -> V3 { a.map(|c| c*s) }
fn unit(a: V3) -> V3 { scale(a, 1.0/dot(a,a).sqrt()) }
fn basis(m: V3) -> (V3, V3) {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 { [0.0,0.0,1.0] } else { [1.0,0.0,0.0] };
    let u = unit(cross(seed, m)); (u, cross(m, u))
}
fn pose(d: f64) -> V3 {
    let e = unit([2.0,1.0,0.0]); let (p1,p2) = basis(e);
    let al = std::f64::consts::TAU * 3.25 / 16.0;
    unit(add(add(scale(p1, al.cos()), scale(p2, al.sin())), scale(e, d)))
}
const NOTCH: [(f64,f64);5] = [(0.0,0.0),(4.0,0.0),(4.0,2.0),(2.0,1.0),(0.0,2.0)];
fn tol() -> Tol { Tol::witness() }
fn at_rest(b: topo::Body<f64>) -> AtRestBody<f64> { AtRestBody::validate(b, tol()).unwrap() }
fn sc() -> f64 { std::env::var("R_S2").map_or(1.0, |s| s.parse().unwrap()) }
fn prism(profile: &[(f64,f64)], dx: f64) -> AtRestBody<f64> {
    let k = if dx > 0.0 { sc() } else { 1.0 };
    let p: Vec<(f64,f64)> = profile.iter().map(|&(x,y)| (k*x+dx,k*y)).collect();
    at_rest(fx::prism::<f64>(&p, k, tol()).body)
}
fn cube(d: f64, dx: f64) -> AtRestBody<f64> {
    let k = if dx > 0.0 { sc() } else { 1.0 };
    let m = pose(d); let (u,w) = basis(m); let v = [2.0*k+dx,k,k];
    at_rest(fx::mapped_cube::<f64>(move |x,y,z| {
        let p = add(v, add(scale(u,k*(-2.0+4.0*x)), add(scale(w,k*(-2.0+4.0*y)), scale(m,k*4.0*z))));
        Point3::new(p[0],p[1],p[2])
    }, tol()))
}
fn brick(lo: f64, hi: f64, xhi: f64) -> AtRestBody<f64> {
    at_rest(fx::brick::<f64>((lo, xhi), (lo, hi), (lo, hi), tol()))
}
fn show(what: &str, r: &Result<topo::BooleanResult<f64>, BooleanError>) {
    match r {
        Ok(res) => {
            let n = res.body().map(|b| b.body.shells().count());
            println!("{what}: OK shells = {n:?}");
        }
        Err(e) => println!("{what}: ERR {e:?}\n  DISPLAY: {e}"),
    }
}
fn main() {
    let dc = BooleanDeclarations::default();
    let d1: f64 = std::env::var("R_D1").map_or(1e-8, |s| s.parse().unwrap());
    let d2: f64 = std::env::var("R_D2").map_or(2e-8, |s| s.parse().unwrap());
    let mode = std::env::var("R_MODE").unwrap_or_else(|_| "p0".into());
    println!("eps = {:e}", tol().eps());
    match mode.as_str() {
        "p0" => {
            let piece = prism(&[(2.0,0.0),(4.0,0.0),(4.0,2.0),(2.0,1.0)], 0.0);
            let r = topo::intersect_with(&piece, &cube(d1, 0.0), &dc, tol());
            show(&format!("P0 piece ∩ cube d={d1:e}"), &r);
            if let Ok(res) = &r && let Some(b) = res.body() {
                let mp = topo::mass_properties(&b.body, tol());
                println!("  mass {:?}", mp.map(|m| (m.volume, m.surface_area)));
                println!("  pseudomanifold {:?}", topo::validate_pseudomanifold(&b.body, &b.contacts, tol()).map(|_| ()));
            }
            let full = prism(&NOTCH, 0.0);
            show(&format!("notch ∩ cube d={d1:e}"), &topo::intersect_with(&full, &cube(d1, 0.0), &dc, tol()));
        }
        "two" | "one" => {
            let two = mode == "two";
            let mut a = brick(-6.0, 10.0, if two { 30.0 } else { 10.0 });
            let mut b = brick(-7.0, 11.0, if two { 31.0 } else { 11.0 });
            let sub = |x: &AtRestBody<f64>, y: &AtRestBody<f64>| -> AtRestBody<f64> {
                let r = topo::subtract_with(x, y, &dc, tol()).expect("sub builds");
                r.body().expect("non-empty").body.clone()
            };
            a = sub(&a, &prism(&NOTCH, 0.0));
            b = sub(&b, &cube(d1, 0.0));
            if two {
                a = sub(&a, &prism(&NOTCH, 20.0));
                b = sub(&b, &cube(d2, 20.0));
            }
            show(&format!("{mode}: A ∪ B d1={d1:e} d2={d2:e}"), &topo::union_with(&a, &b, &dc, tol()));
            show(&format!("{mode}: B ∪ A d1={d1:e} d2={d2:e}"), &topo::union_with(&b, &a, &dc, tol()));
        }
        _ => panic!("mode"),
    }
}
