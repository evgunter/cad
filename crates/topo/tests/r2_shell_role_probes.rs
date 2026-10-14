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
    s.chars().take(2000).collect()
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

/// A needle `1 × w × w`: every pair across it is `w` apart, but every
/// face is only `w` wide, so a planar face's certified interior point
/// may not exist; does check 7 still read its orientation?
#[test]
#[ignore = "review probe"]
fn r2_needles() {
    let tol = Tol::witness();
    let e = tol.eps();
    for m in [10.5, 12.0, 15.0, 19.0, 21.0, 25.0, 30.0, 40.0] {
        let w = m * e;
        let built = std::panic::catch_unwind(|| common::brick::<f64>((0.0, 1.0), (0.0, w), (0.0, w), tol));
        let Ok(raw) = built else {
            println!("R2N eps={e:e} m={m} needle does not build");
            continue;
        };
        let solo = AtRestBody::validate(raw.clone(), tol).map(|_| ());
        let inv = AtRestBody::validate(raw.revert(), tol).map(|_| ());
        println!(
            "R2N eps={e:e} m={m} solo={} inv={} classify={:?}",
            short(&solo),
            short(&inv),
            classify_shells(&raw, tol).map(|c| c.iter().map(|c| c.role).collect::<Vec<_>>()),
        );
    }
}

/// The inverted needle's faces: each face's plane and its vertices.
#[test]
#[ignore = "review probe"]
fn r2_needle_faces() {
    let tol = Tol::witness();
    let w = 30.0 * tol.eps();
    let raw = common::brick::<f64>((0.0, 1.0), (0.0, w), (0.0, w), tol).revert();
    for (k, f) in raw.faces() {
        let s = raw.get_surface(f.surface);
        let mut vs = Vec::new();
        let lp = raw.get_loop(f.outer).expect("loop");
        if let topo::LoopBoundary::Cycle { first } = lp.boundary {
            let mut h = first;
            loop {
                let he = raw.get_half_edge(h).expect("he");
                vs.push(format!("{:?}", he.start));
                h = he.next;
                if h == first {
                    break;
                }
            }
        }
        println!("R2F face={k:?} sense={} surface={s:?} verts={vs:?}", f.sense);
    }
}
