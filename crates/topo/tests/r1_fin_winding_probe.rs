//! Review R1 (FLUSH shell-role reader): the fin ∪ pyramid probes 2 and 4,
//! read by an independent generalised winding number over the result's
//! planar faces (exact up to f64 rounding: every face is planar, so its
//! signed solid angle is its loops' fan sum).
//!
//! Measured at ε 1e-9 (review R1, PR 5119 head e672e97ac): the fin crown
//! ALONE (`posed_crown(fin())`, at rest on base and head) has points of
//! winding −1 near its slit's mouth, and every union of it with probes 1,
//! 2 and 4, in both orders, carries the same points. On head only
//! `x ∪ y` of probes 2 and 4 at three of the four poses refuses
//! (`NegativeVolume`); `y ∪ x`, probe 1 and the flipped pose build and
//! pass tier 3 with the region. `cargo test -p topo --test all --release
//! r1_fin_probe_windings -- --ignored --nocapture`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use common::meeting::{apex_pyramid, corners, fin, posed_crown, poses};
use geom_core::Tol;
use topo::{BooleanResult, union, validate_geometric};

fn t() -> Tol {
    Tol::witness()
}

type P = [f64; 3];
fn sub(a: P, b: P) -> P {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn dot(a: P, b: P) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: P, b: P) -> P {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn norm(a: P) -> f64 {
    dot(a, a).sqrt()
}

/// Signed solid angle of triangle (a, b, c) seen from p (Van Oosterom–Strackee).
fn tri(p: P, a: P, b: P, c: P) -> f64 {
    let (a, b, c) = (sub(a, p), sub(b, p), sub(c, p));
    let (la, lb, lc) = (norm(a), norm(b), norm(c));
    let num = dot(a, cross(b, c));
    let den = la * lb * lc + dot(a, b) * lc + dot(a, c) * lb + dot(b, c) * la;
    2.0 * num.atan2(den)
}

/// Each planar face as its loops' point lists, each oriented so the face's
/// region carries its geometric outward normal (surface normal and sense).
fn faces(body: &topo::Body<f64>) -> Vec<(String, Vec<Vec<P>>)> {
    let mut out = Vec::new();
    for (fk, f) in body.faces() {
        let geom::Surface::Plane { normal, .. } = body.get_surface(f.surface).unwrap() else {
            panic!("non-planar face {fk:?}");
        };
        let n = if f.sense {
            [normal.x, normal.y, normal.z]
        } else {
            [-normal.x, -normal.y, -normal.z]
        };
        let mut loops = Vec::new();
        for (k, l) in core::iter::once(f.outer)
            .chain(f.rings.iter().copied())
            .enumerate()
        {
            let topo::LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary else {
                continue;
            };
            let mut pts = Vec::new();
            let mut h = first;
            loop {
                let he = body.get_half_edge(h).unwrap();
                let v = body.get_vertex(he.start).unwrap();
                let q = body.get_point(v.point).unwrap();
                pts.push([q.x, q.y, q.z]);
                h = he.next;
                if h == first {
                    break;
                }
            }
            // area vector
            let mut a = [0.0; 3];
            for i in 0..pts.len() {
                let c = cross(pts[i], pts[(i + 1) % pts.len()]);
                a = [a[0] + c[0], a[1] + c[1], a[2] + c[2]];
            }
            let along = dot(a, n) > 0.0;
            if (k == 0) != along {
                pts.reverse();
            }
            loops.push(pts);
        }
        out.push((format!("{fk:?}"), loops));
    }
    out
}

fn winding(fs: &[(String, Vec<Vec<P>>)], p: P) -> f64 {
    let mut s = 0.0;
    for (_, loops) in fs {
        for l in loops {
            for i in 1..l.len() - 1 {
                s += tri(p, l[0], l[i], l[i + 1]);
            }
        }
    }
    s / (4.0 * core::f64::consts::PI)
}

type Bad = (String, f64, f64, P);
fn sample(fs: &[(String, Vec<Vec<P>>)]) -> (std::collections::BTreeMap<i64, usize>, Vec<Bad>) {
    let mut hist = std::collections::BTreeMap::<i64, usize>::new();
    let mut bad = Vec::new();
    for (name, loops) in fs {
        let l = &loops[0];
        let n = {
            let mut a = [0.0; 3];
            for i in 0..l.len() {
                let c = cross(l[i], l[(i + 1) % l.len()]);
                a = [a[0] + c[0], a[1] + c[1], a[2] + c[2]];
            }
            let m = norm(a);
            [a[0] / m, a[1] / m, a[2] / m]
        };
        let m = l.len();
        let c: P = (0..3)
            .map(|j| l.iter().map(|q| q[j]).sum::<f64>() / m as f64)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        for i in 0..m {
            for wgt in [0.2, 0.5, 0.8, 0.95, 0.99] {
                let q = [
                    c[0] + wgt * (l[i][0] - c[0]),
                    c[1] + wgt * (l[i][1] - c[1]),
                    c[2] + wgt * (l[i][2] - c[2]),
                ];
                for d in [-1e-6, -2e-7, -5e-8, 5e-8, 2e-7, 1e-6] {
                    let p = [q[0] + d * n[0], q[1] + d * n[1], q[2] + d * n[2]];
                    let w = winding(fs, p);
                    let r = w.round() as i64;
                    *hist.entry(r).or_default() += 1;
                    if r != 0 && r != 1 {
                        bad.push((name.clone(), d, w, p));
                    }
                }
            }
        }
    }
    (hist, bad)
}

#[test]
#[ignore = "review probe"]
fn r1_fin_probe_windings() {
    for pose in poses() {
        let fin_b = posed_crown(&fin(), [0.0, 0.2, -0.6], &pose, t());
        {
            let fs = faces(&fin_b);
            let (hist, bad) = sample(&fs);
            eprintln!(
                "{} the fin alone: tier3 {:?} hist {hist:?}",
                pose.label,
                validate_geometric(&fin_b, t()).is_ok()
            );
            for b in bad.iter().take(4) {
                eprintln!("  BAD {b:?}");
            }
        }
        for (k, (b, rise)) in [
            (2usize, (130.0, -0.3)),
            (4, (250.0, -0.3)),
            (1, (130.0, 0.3)),
        ] {
            let probe = apex_pyramid(&corners(b, rise, 0.5), &pose, t());
            for order in ["x ∪ y", "y ∪ x"] {
                let r = if order == "x ∪ y" {
                    union(&probe, &fin_b, t())
                } else {
                    union(&fin_b, &probe, t())
                };
                let k = format!("{k} {order}");
                let body = match r {
                    Ok(BooleanResult::Body(r)) => r.body,
                    Ok(BooleanResult::Empty) => {
                        eprintln!("{} probe {k}: empty", pose.label);
                        continue;
                    }
                    Err(e) => {
                        eprintln!(
                            "{} probe {k}: refused {:?}",
                            pose.label,
                            format!("{e:?}").chars().take(300).collect::<String>()
                        );
                        continue;
                    }
                };
                eprintln!(
                    "{} probe {k}: built, tier3 {:?}",
                    pose.label,
                    validate_geometric(&body, t()).is_ok()
                );
                let fs = faces(&body);
                // calibrate: centroid of all vertices, far point
                let far = winding(&fs, [100.0, 37.0, 11.0]);
                let mut hist = std::collections::BTreeMap::<i64, usize>::new();
                let mut bad = Vec::new();
                for (name, loops) in &fs {
                    let l = &loops[0];
                    let n = {
                        let mut a = [0.0; 3];
                        for i in 0..l.len() {
                            let c = cross(l[i], l[(i + 1) % l.len()]);
                            a = [a[0] + c[0], a[1] + c[1], a[2] + c[2]];
                        }
                        let m = norm(a);
                        [a[0] / m, a[1] / m, a[2] / m]
                    };
                    // sample points across the face: vertex-weighted interior mixes
                    let m = l.len();
                    for i in 0..m {
                        for wgt in [0.2, 0.5, 0.8] {
                            let c: P = (0..3)
                                .map(|j| l.iter().map(|q| q[j]).sum::<f64>() / m as f64)
                                .collect::<Vec<_>>()
                                .try_into()
                                .unwrap();
                            let q = [
                                c[0] + wgt * (l[i][0] - c[0]),
                                c[1] + wgt * (l[i][1] - c[1]),
                                c[2] + wgt * (l[i][2] - c[2]),
                            ];
                            for d in [-1e-6, -2e-7, -5e-8, 5e-8, 2e-7, 1e-6] {
                                let p = [q[0] + d * n[0], q[1] + d * n[1], q[2] + d * n[2]];
                                let w = winding(&fs, p);
                                let r = w.round() as i64;
                                *hist.entry(r).or_default() += 1;
                                if r != 0 && r != 1 {
                                    bad.push((name.clone(), d, w, p));
                                }
                            }
                        }
                    }
                }
                eprintln!("  far {far:.3}, hist {hist:?}, faces {}", fs.len());
                for b in bad.iter().take(8) {
                    eprintln!("  BAD {b:?}");
                }
            }
        }
    }
}
