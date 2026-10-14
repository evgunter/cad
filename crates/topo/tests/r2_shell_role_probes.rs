//! Review R2 probes for PR 5119 (not pins): oblique slabs at and just
//! above `Kε`, inside out, as a cavity and beside a far brick; and the
//! reader's cost on many-face shells. Run with `--ignored --nocapture`.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Point3, Tol};
use topo::{AtRestBody, Body, ShellRole, classify_shells};

use crate::common;

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

fn slab(t: f64, r: [[f64; 3]; 3], c: [f64; 3], tol: Tol) -> Body<f64> {
    common::mapped_cube::<f64>(
        move |x, y, z| {
            let p = [x - 0.5, y - 0.5, (z - 0.5) * t];
            Point3::new(
                c[0] + r[0][0] * p[0] + r[0][1] * p[1] + r[0][2] * p[2],
                c[1] + r[1][0] * p[0] + r[1][1] * p[1] + r[1][2] * p[2],
                c[2] + r[2][0] * p[0] + r[2][1] * p[1] + r[2][2] * p[2],
            )
        },
        tol,
    )
}

fn short<E: core::fmt::Debug>(e: &E) -> String {
    let s = format!("{e:?}");
    s.chars().take(110).collect()
}

const POSES: [([f64; 3], f64); 7] = [
    ([0.0, 0.0, 1.0], 0.0),
    ([1.0, 0.0, 0.0], 0.3),
    ([1.0, 1.0, 0.0], 0.7853981633974483),
    ([1.0, 2.0, 3.0], 1.1),
    ([0.3, -0.7, 0.2], 2.4),
    ([1.0, 1.0, 1.0], 0.9553166181245093),
    ([0.0, 1.0, 0.0], 1.5707963267948966),
];

#[test]
#[ignore = "review probe"]
fn r2_oblique_slabs_at_and_above_k_eps() {
    let tol = Tol::witness();
    let e = tol.eps();
    let big = AtRestBody::validate(
        common::brick::<f64>((-1.5, 1.5), (-1.5, 1.5), (-1.5, 1.5), tol),
        tol,
    )
    .expect("brick");
    let far = AtRestBody::validate(
        common::brick::<f64>((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol),
        tol,
    )
    .expect("far");
    let mut bad = 0;
    for m in [10.0, 10.001, 10.01, 10.05, 10.2, 10.5, 12.0] {
        for (i, (axis, ang)) in POSES.iter().enumerate() {
            for c in [[0.1, 0.0, 0.0], [0.0, 0.0, 0.0]] {
                let t = m * e;
                let r = rot(*axis, *ang);
                let Ok(raw) = std::panic::catch_unwind(|| slab(t, r, c, tol)) else {
                    println!("R2 eps={e:e} m={m} pose={i} c={c:?} slab does not build");
                    continue;
                };
                let solo = AtRestBody::validate(raw.clone(), tol);
                let inv = AtRestBody::validate(raw.revert(), tol).map(|_| ());
                let line_solo = match &solo {
                    Ok(_) => "ok".to_string(),
                    Err(v) => short(v),
                };
                let Ok(s) = solo else {
                    println!("R2 eps={e:e} m={m} pose={i} c={c:?} solo={line_solo} inv={}", short(&inv));
                    continue;
                };
                let cav = topo::subtract(&big, &s, tol).map(|r| {
                    r.body()
                        .map(|b| classify_shells(&b.body, tol).map(|cs| cs.iter().map(|c| c.role).collect::<Vec<_>>()))
                });
                let cav_ok = matches!(&cav, Ok(Some(Ok(roles))) if roles == &[ShellRole::Outer, ShellRole::Void]);
                let uni = topo::union(&s, &far, tol).map(|r| {
                    r.body().map(|b| b.body.solids().map(|(_, s)| s.shells.len()).collect::<Vec<_>>())
                });
                let uni_ok = matches!(&uni, Ok(Some(v)) if v == &[1, 1]);
                let inv_ok = matches!(&inv, Err(v) if v.len() == 1 && matches!(v[0], topo::ValidationError::NegativeVolume { .. }));
                if !(cav_ok && uni_ok && inv_ok) {
                    bad += 1;
                }
                println!(
                    "R2 eps={e:e} m={m} pose={i} c={c:?} solo=ok inv={} cav={} uni={}",
                    if inv_ok { "NegVol".into() } else { short(&inv) },
                    if cav_ok { "Void".into() } else { short(&cav) },
                    if uni_ok { "two".into() } else { short(&uni) },
                );
            }
        }
    }
    println!("R2 eps={e:e} rows not as expected: {bad}");
}

fn ngon_prism(n: usize, r: f64, z: (f64, f64), tol: Tol) -> Body<f64> {
    let profile: Vec<(f64, f64)> = (0..n)
        .map(|k| {
            let a = core::f64::consts::TAU * k as f64 / n as f64;
            (r * a.cos(), r * a.sin())
        })
        .collect();
    common::prism_z::<f64>(&profile, z.0, z.1, tol).body
}

#[test]
#[ignore = "review probe"]
fn r2_many_face_cost() {
    let tol = Tol::witness();
    for n in [16, 64, 256] {
        let t0 = std::time::Instant::now();
        let p = AtRestBody::validate(ngon_prism(n, 0.5, (-0.5, 0.5), tol), tol).expect("prism");
        let t_val = t0.elapsed();
        let big = AtRestBody::validate(
            common::brick::<f64>((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0), tol),
            tol,
        )
        .expect("brick");
        let t1 = std::time::Instant::now();
        let cav = topo::subtract(&big, &p, tol);
        let t_sub = t1.elapsed();
        let body = cav.expect("cavity").body().expect("nonempty").body.clone();
        let t2 = std::time::Instant::now();
        let roles = classify_shells(&body, tol).map(|cs| cs.iter().map(|c| c.role).collect::<Vec<_>>());
        let t_cls = t2.elapsed();
        let t3 = std::time::Instant::now();
        let again = AtRestBody::validate(Body::clone(&body), tol).map(|_| ());
        let t_rest = t3.elapsed();
        println!(
            "R2 cost n={n} validate_prism={t_val:?} subtract={t_sub:?} classify={t_cls:?} at_rest_cavity={t_rest:?} roles={roles:?} rest={again:?}"
        );
    }
}
