//! Design probe (branch `flush/needle-design-y`, not for merge): the
//! interval enclosure of a needle's volume against the V/A band, at rest,
//! oblique, inside out, and as a boolean result. Run with
//! `--ignored --nocapture` under `CAD_TOLERANCE_EPS`.
#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout, clippy::unwrap_used)]

use geom_core::{Point3, Tol};
use topo::{AtRestBody, Body};

use crate::common;

fn short<E: core::fmt::Debug>(e: &E) -> String {
    let s = format!("{e:?}");
    s.chars().take(300).collect()
}

fn rot(axis: [f64; 3], ang: f64) -> [[f64; 3]; 3] {
    let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    let (x, y, z) = (axis[0] / n, axis[1] / n, axis[2] / n);
    let (s, c) = ang.sin_cos();
    let k = 1.0 - c;
    [
        [c + x * x * k, x * y * k - z * s, x * z * k + y * s],
        [y * x * k + z * s, c + y * y * k, y * z * k - x * s],
        [z * x * k - y * s, z * y * k + x * s, c + z * z * k],
    ]
}

/// A box `[0,1] × [0,w] × [0,w]`, rotated by `r` about its own middle and
/// moved to `c`.
fn needle(w: f64, r: [[f64; 3]; 3], c: [f64; 3], tol: Tol) -> Body<f64> {
    common::mapped_cube::<f64>(
        move |x, y, z| {
            let p = [x - 0.5, (y - 0.5) * w, (z - 0.5) * w];
            Point3::new(
                c[0] + r[0][0] * p[0] + r[0][1] * p[1] + r[0][2] * p[2],
                c[1] + r[1][0] * p[0] + r[1][1] * p[1] + r[1][2] * p[2],
                c[2] + r[2][0] * p[0] + r[2][1] * p[1] + r[2][2] * p[2],
            )
        },
        tol,
    )
}

fn report(label: &str, body: &Body<f64>, tol: Tol) {
    let e = tol.eps();
    match topo::volume_enclosure_probe(body, tol) {
        Ok((v, a, lo, hi, recentred)) => println!(
            "{label}: V={v:.3e} A={a:.3e} V/A={:.3}eps enclosure=[{lo:.6e}, {hi:.6e}] width={:.3e} width/|V|={:.2e} recentred={recentred} straddles={}",
            v / a / e,
            hi - lo,
            (hi - lo) / v.abs(),
            lo <= 0.0 && hi >= 0.0
        ),
        Err(err) => println!("{label}: props refuse {}", short(&err)),
    }
    match AtRestBody::validate(body.clone(), tol) {
        Ok(_) => println!("    at rest: ok"),
        Err(errs) => println!("    at rest: {}", short(&errs)),
    }
}

#[test]
#[ignore = "design probe"]
fn needle_design_y_enclosures() {
    let tol = Tol::witness();
    let e = tol.eps();
    println!("eps={e:e} K={}", tol.k());
    let id = rot([0.0, 0.0, 1.0], 0.0);
    for m in [10.5, 12.0, 15.0, 21.0, 30.0] {
        let w = m * e;
        let raw = needle(w, id, [0.5, w / 2.0, w / 2.0], tol);
        report(&format!("axis needle w={m}eps"), &raw, tol);
        report(&format!("axis needle w={m}eps inside out"), &raw.revert(), tol);
    }
    for m in [10.5, 12.0, 15.0, 21.0, 30.0] {
        let w = m * e;
        let raw = needle(w, rot([1.0, 2.0, 3.0], 1.1), [0.3, -0.2, 0.5], tol);
        report(&format!("oblique needle w={m}eps"), &raw, tol);
        report(&format!("oblique needle w={m}eps inside out"), &raw.revert(), tol);
        let raw = needle(w, rot([0.3, -0.7, 0.2], 2.4), [37.0, -21.0, 13.0], tol);
        report(&format!("far oblique needle w={m}eps"), &raw, tol);
    }
}

#[test]
#[ignore = "design probe"]
fn needle_design_y_boolean_results() {
    let tol = Tol::witness();
    let e = tol.eps();
    println!("eps={e:e} K={}", tol.k());
    let poses: [([f64; 3], f64, [f64; 3]); 4] = [
        ([0.0, 0.0, 1.0], 0.0, [0.0; 3]),
        ([1.0, 2.0, 3.0], 1.1, [0.3, -0.2, 0.5]),
        ([0.3, -0.7, 0.2], 2.4, [1.7, 0.4, -0.9]),
        ([1.0, 1.0, 0.0], core::f64::consts::FRAC_PI_4, [0.0; 3]),
    ];
    for m in [20.0, 30.0, 15.0] {
        let w = m * e;
        for (i, (axis, ang, c)) in poses.iter().enumerate() {
            let r = rot(*axis, *ang);
            let place = move |p: [f64; 3]| {
                Point3::new(
                    c[0] + r[0][0] * p[0] + r[0][1] * p[1] + r[0][2] * p[2],
                    c[1] + r[1][0] * p[0] + r[1][1] * p[1] + r[1][2] * p[2],
                    c[2] + r[2][0] * p[0] + r[2][1] * p[1] + r[2][2] * p[2],
                )
            };
            // A ∩ B = [0,1] × [1−w,1] × [1−w,1].
            let a = common::mapped_cube::<f64>(move |x, y, z| place([x, y, z]), tol);
            let b = common::mapped_cube::<f64>(
                move |x, y, z| place([x, 1.0 - w + y * (1.0 + w), 1.0 - w + z * (1.0 + w)]),
                tol,
            );
            let (Ok(a), Ok(b)) = (
                AtRestBody::validate(a, tol),
                AtRestBody::validate(b, tol),
            ) else {
                println!("w={m}eps pose={i}: an operand refuses at rest");
                continue;
            };
            match topo::intersect(&a, &b, tol) {
                Ok(res) => match res.body() {
                    Some(body) => report(&format!("brick∩brick w={m}eps pose={i}"), &body.body, tol),
                    None => println!("brick∩brick w={m}eps pose={i}: empty"),
                },
                Err(err) => println!("brick∩brick w={m}eps pose={i}: door refuses {}", short(&err)),
            }
        }
    }
}
