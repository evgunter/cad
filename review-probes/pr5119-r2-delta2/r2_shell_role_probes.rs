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
                let inv = AtRestBody::validate(raw.revert().expect("revert"), tol).map(|_| ());
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
        let inv = AtRestBody::validate(raw.revert().expect("revert"), tol).map(|_| ());
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
    let raw = common::brick::<f64>((0.0, 1.0), (0.0, w), (0.0, w), tol).revert().expect("revert");
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

/// Delta check: a needle CAVITY at a boolean door, `brick ∖ needle`.
/// Check 7 reads the solid off its outer shell; check 10 reads the
/// needle shell's role, whose rays and volume are the needle class's.
/// How does the door type it, and how does a needle SOLID type there?
#[test]
#[ignore = "review probe"]
fn r2_needle_cavity_and_solid_at_the_door() {
    let tol = Tol::witness();
    let e = tol.eps();
    let big = AtRestBody::validate(
        common::brick::<f64>((-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), tol),
        tol,
    )
    .expect("brick");
    for m in [10.5, 15.0, 21.0, 30.0, 40.0] {
        let w = m * e;
        let a = AtRestBody::validate(
            common::brick::<f64>((0.0, 1.0), (0.5, 0.5 + w), (-0.5, 1.5), tol),
            tol,
        )
        .expect("a wall");
        let b = AtRestBody::validate(
            common::brick::<f64>((0.0, 1.0), (-0.5, 1.5), (0.5, 0.5 + w), tol),
            tol,
        )
        .expect("a wall");
        let needle = topo::intersect(&a, &b, tol);
        let needle_line = match &needle {
            Ok(r) => match r.body() {
                Some(_) => "built".to_owned(),
                None => "empty".to_owned(),
            },
            Err(err) => format!("{err}"),
        };
        println!("R2D eps={e:e} m={m} a∩b needle: {}", needle_line.chars().take(400).collect::<String>());
        let slab = AtRestBody::validate(
            common::brick::<f64>((0.0, 1.0), (0.5, 0.8), (0.5, 0.5 + w), tol),
            tol,
        );
        let fill = AtRestBody::validate(
            common::brick::<f64>((0.0, 1.0), (0.5 + w, 0.8), (0.5, 0.5 + w), tol),
            tol,
        );
        let line = match (slab, fill) {
            (Ok(slab), Ok(fill)) => match topo::subtract(&big, &slab, tol) {
                Ok(r) => match r.body() {
                    Some(c1) => match topo::union(&c1.body, &fill, tol) {
                        Ok(r2) => format!(
                            "built {:?}",
                            r2.body().map(|b| b.body.solids().map(|(_, s)| s.shells.len()).collect::<Vec<_>>())
                        ),
                        Err(err) => format!("{err:?}").chars().take(300).collect::<String>()
                            + " || "
                            + &format!("{err}").chars().take(400).collect::<String>(),
                    },
                    None => "slab cut empty".to_owned(),
                },
                Err(err) => format!("slab cut refused: {err}"),
            },
            (s, f) => format!("operands: slab ok={} fill ok={}", s.is_ok(), f.is_ok()),
        };
        println!("R2D eps={e:e} m={m} needle cavity: {line}");
    }
}

fn posed_box(lo: [f64; 3], hi: [f64; 3], r: [[f64; 3]; 3], c: [f64; 3], tol: Tol) -> Body<f64> {
    common::mapped_cube::<f64>(
        move |x, y, z| {
            let p = [
                lo[0] + (hi[0] - lo[0]) * x,
                lo[1] + (hi[1] - lo[1]) * y,
                lo[2] + (hi[2] - lo[2]) * z,
            ];
            Point3::new(
                c[0] + r[0][0] * p[0] + r[0][1] * p[1] + r[0][2] * p[2],
                c[1] + r[1][0] * p[0] + r[1][1] * p[1] + r[1][2] * p[2],
                c[2] + r[2][0] * p[0] + r[2][1] * p[1] + r[2][2] * p[2],
            )
        },
        tol,
    )
}

fn kind<E: core::fmt::Debug>(r: &Result<(), Vec<E>>) -> String {
    match r {
        Ok(()) => "ok".into(),
        Err(v) => {
            let s = format!("{v:?}");
            s.split([' ', '{', '(']).nth(0).map(|_| s.chars().take(160).collect()).unwrap_or_default()
        }
    }
}

/// Second delta: the needle bar, axis-aligned and posed, far and near, both
/// orientations, at rest; plus the lane-free doors on the plain bar.
#[test]
#[ignore = "review probe"]
fn r2_needle_bars_posed() {
    let tol = Tol::witness();
    let e = tol.eps();
    let centres = [[0.0, 0.0, 0.0], [37.0, -21.0, 13.0], [1234.5, -987.25, 3333.0]];
    let (mut ok, mut bad) = (0, 0);
    for m in [10.5, 12.0, 15.0, 21.0, 30.0] {
        let w = m * e;
        for (i, (axis, ang)) in POSES.iter().enumerate() {
            for (j, c) in centres.iter().enumerate() {
                let Ok(raw) = std::panic::catch_unwind(|| {
                    posed_box([0.0, 0.0, 0.0], [1.0, w, w], rot(*axis, *ang), *c, tol)
                }) else {
                    println!("R2B eps={e:e} m={m} pose={i} c={j} does not build");
                    continue;
                };
                let plain = AtRestBody::validate(raw.clone(), tol).map(|_| ());
                let inv = AtRestBody::validate(raw.revert().expect("revert"), tol).map(|_| ());
                let inv_ok = matches!(&inv, Err(v) if v.len() == 1 && matches!(v[0], topo::ValidationError::NegativeVolume { .. }));
                if plain.is_ok() && inv_ok {
                    ok += 1;
                } else {
                    bad += 1;
                    println!("R2B eps={e:e} m={m} pose={i} c={j} plain={} inv={}", kind(&plain), kind(&inv));
                }
                if i == 0 && j == 0 {
                    let s = topo::validate_geometric_structural(&raw, tol).map(|_| ());
                    let cs = topo::classify_shells_structural(&raw, tol).map(|v| v.iter().map(|c| c.role).collect::<Vec<_>>());
                    println!("R2S eps={e:e} m={m} structural={} classify_structural={cs:?}", kind(&s));
                }
            }
        }
    }
    println!("R2B eps={e:e} read={ok} not={bad}");
}

/// Second delta: `brick ∩ brick` needles at 2Kε and 3Kε in posed frames,
/// and needle cavities cut and filled, at the door; roles read back.
#[test]
#[ignore = "review probe"]
fn r2_door_needles_posed() {
    let tol = Tol::witness();
    let e = tol.eps();
    let k = tol.k();
    for f in [2.0, 3.0] {
        let w = f * k * e;
        for (i, (axis, ang)) in POSES.iter().enumerate() {
            let r = rot(*axis, *ang);
            let c = [0.1, 0.2, 0.3];
            let made = std::panic::catch_unwind(|| {
                (
                    posed_box([0.0, 0.5, -0.5], [1.0, 0.5 + w, 1.5], r, c, tol),
                    posed_box([0.0, -0.5, 0.5], [1.0, 1.5, 0.5 + w], r, c, tol),
                )
            });
            let Ok((a, b)) = made else {
                println!("R2X eps={e:e} {f}K pose={i} walls do not build");
                continue;
            };
            let (Ok(a), Ok(b)) = (AtRestBody::validate(a, tol), AtRestBody::validate(b, tol)) else {
                println!("R2X eps={e:e} {f}K pose={i} walls refuse at rest");
                continue;
            };
            let line = match topo::intersect(&a, &b, tol) {
                Ok(res) => match res.body() {
                    Some(n) => {
                        let inv = AtRestBody::validate(n.body.revert().expect("revert"), tol).map(|_| ());
                        format!("built; inverted at rest: {}", kind(&inv))
                    }
                    None => "empty".into(),
                },
                Err(err) => format!("{err}").chars().take(260).collect(),
            };
            println!("R2X eps={e:e} {f}K pose={i} a∩b: {line}");
            // the needle as a cavity: cut a slab, fill all but a w x w channel
            let big = AtRestBody::validate(posed_box([-1.0, -1.0, -1.0], [2.0, 2.0, 2.0], r, c, tol), tol).expect("big");
            let slab = AtRestBody::validate(posed_box([0.0, 0.5, 0.5], [1.0, 0.8, 0.5 + w], r, c, tol), tol);
            let fill = AtRestBody::validate(posed_box([0.0, 0.5 + w, 0.5], [1.0, 0.8, 0.5 + w], r, c, tol), tol);
            let cav = match (slab, fill) {
                (Ok(slab), Ok(fill)) => match topo::subtract(&big, &slab, tol) {
                    Ok(r1) => match r1.body() {
                        Some(c1) => match topo::union(&c1.body, &fill, tol) {
                            Ok(r2) => match r2.body() {
                                Some(b) => format!(
                                    "built shells={:?} roles={:?}",
                                    b.body.solids().map(|(_, s)| s.shells.len()).collect::<Vec<_>>(),
                                    topo::classify_shells(&b.body, tol).map(|v| v.iter().map(|c| c.role).collect::<Vec<_>>())
                                ),
                                None => "empty".into(),
                            },
                            Err(err) => format!("{err}").chars().take(260).collect(),
                        },
                        None => "cut empty".into(),
                    },
                    Err(err) => format!("cut: {err}").chars().take(200).collect(),
                },
                _ => "operands refuse".into(),
            };
            println!("R2X eps={e:e} {f}K pose={i} cavity: {cav}");
        }
    }
}

/// Second delta: the lane-free doors' words on a needle and on a needle
/// cavity (both read V/A against the band, as the walk's sums decide
/// where no lane certifies).
#[test]
#[ignore = "review probe"]
fn r2_lane_free_words() {
    let tol = Tol::witness();
    let e = tol.eps();
    let w = 30.0 * e;
    let needle = common::brick::<f64>((0.0, 1.0), (0.0, w), (0.0, w), tol);
    if let Err(v) = topo::validate_geometric_structural(&needle, tol) {
        for x in &v {
            println!("R2W eps={e:e} needle structural: {x}");
        }
    }
    let big = AtRestBody::validate(common::brick::<f64>((-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), tol), tol).expect("big");
    let slab = AtRestBody::validate(common::brick::<f64>((0.0, 1.0), (0.5, 0.8), (0.5, 0.5 + w), tol), tol).expect("slab");
    let fill = AtRestBody::validate(common::brick::<f64>((0.0, 1.0), (0.5 + w, 0.8), (0.5, 0.5 + w), tol), tol).expect("fill");
    let c1 = topo::subtract(&big, &slab, tol).expect("cut");
    let c1 = c1.body().expect("nonempty");
    let r2 = topo::union(&c1.body, &fill, tol).expect("fill");
    let b = r2.body().expect("nonempty");
    match topo::classify_shells_structural(&b.body, tol) {
        Ok(v) => println!("R2W eps={e:e} cavity classify_structural: {:?}", v.iter().map(|c| c.role).collect::<Vec<_>>()),
        Err(err) => println!("R2W eps={e:e} cavity classify_structural: {err}"),
    }
    match topo::validate_geometric_structural(&b.body, tol) {
        Ok(()) => println!("R2W eps={e:e} cavity structural: ok"),
        Err(v) => {
            for x in &v {
                println!("R2W eps={e:e} cavity structural: {x}");
            }
        }
    }
}
