//! review-3 probe (PR 4256): adversarial scenes against the germ, and a
//! dump of every cell's edge_classes for a main/head diff. Ignored; run with
//! R3_OUT=<file> R3_RANDOM=<n>.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code, unused)]

use crate::common::meeting::{MEET, PLATE, Pose, at, posed_box, posed_pyramid, poses};
use geom_core::{Tol, Vec3};
use topo::{AtRestBody, BooleanResult, Operand, SideCode, intersect, readback, subtract, union, validate_geometric};

include!("../../../review3-probes/r3_scenes.rs");

fn local(pose: &Pose, w: Vec3<f64>) -> [f64; 3] {
    let o = pose.at([0.0; 3]);
    let cols = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]].map(|e| pose.at(e) - o);
    cols.map(|cv| cv.x * w.x + cv.y * w.y + cv.z * w.z)
}

#[test]
#[ignore]
fn r3_probe() {
    use std::io::Write;
    let t = Tol::witness();
    let random: usize = std::env::var("R3_RANDOM").ok().and_then(|s| s.parse().ok()).unwrap_or(6);
    let path = std::env::var("R3_OUT").unwrap_or("/tmp/r3.txt".into());
    let mut f = std::io::BufWriter::new(std::fs::File::create(&path).unwrap());
    let (mut wrong, mut missing, mut doubled, mut built, mut tier, mut panics) = (0, 0, 0, 0, 0, 0);
    let mut errs: std::collections::BTreeMap<String, usize> = Default::default();
    let mut wrong_cells = Vec::new();

    for pose in poses() {
        let mut skipped = Vec::new();
        let scenes = r3_scenes(&pose, t, random, &mut skipped);
        for s in &skipped {
            writeln!(f, "SKIP {} {s}", pose.label).unwrap();
        }
        let meet = at(pose.at(MEET));
        for sc in &scenes {
            for (pn, x, xg) in &sc.probes {
                let y = &sc.y;
                let yg = &sc.yg;
                for (what, a, ag, b, bg, k) in [
                    ("x-y", x, xg, y, yg, 0), ("y-x", y, yg, x, xg, 1),
                    ("xUy", x, xg, y, yg, 2), ("yUx", y, yg, x, xg, 3),
                    ("xNy", x, xg, y, yg, 4), ("yNx", y, yg, x, xg, 5),
                ] {
                    let cell = format!("{}|{}|{}|{}", sc.label, pn, pose.label, what);
                    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match k {
                        0 | 1 => subtract(a, b, t),
                        2 | 3 => union(a, b, t),
                        _ => intersect(a, b, t),
                    }));
                    let r = match r {
                        Err(e) => {
                            panics += 1;
                            let m = e.downcast_ref::<String>().cloned().or(e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default();
                            writeln!(f, "{cell}|PANIC {}", m.chars().take(160).collect::<String>()).unwrap();
                            continue;
                        }
                        Ok(r) => r,
                    };
                    let r = match r {
                        Ok(BooleanResult::Body(r)) => r,
                        Ok(BooleanResult::Empty) => { writeln!(f, "{cell}|EMPTY").unwrap(); continue; }
                        Err(e) => {
                            let kind = format!("{e:?}");
                            let kind = kind.split(|c: char| !c.is_alphanumeric()).next().unwrap().to_string();
                            *errs.entry(kind.clone()).or_default() += 1;
                            writeln!(f, "{cell}|ERR {e:?}").unwrap();
                            continue;
                        }
                    };
                    built += 1;
                    if validate_geometric(&r.body, t).is_err() { tier += 1; }
                    let mut rows: Vec<String> = r.naming.edge_classes.iter().map(|row| format!("{:?}/{:?}/{:?}/{}/{:?}", row.operand, row.vertex, row.edge, row.starts, row.class)).collect();
                    rows.sort();
                    writeln!(f, "{cell}|BODY {}", rows.join(" ")).unwrap();
                    for (op, own, other) in [(Operand::A, a, bg), (Operand::B, b, ag)] {
                        for (v, _) in own.vertices() {
                            let pv = readback::vertex_point(own, v).unwrap();
                            if at(pv) != meet { continue; }
                            for (e, _) in own.edges() {
                                let edge = own.get_edge(e).unwrap();
                                let plus = own.get_half_edge(edge.he_plus).unwrap();
                                let next = own.get_half_edge(plus.next).unwrap();
                                for (starts, here, far) in [(true, plus.start, next.start), (false, next.start, plus.start)] {
                                    if here != v { continue; }
                                    let want = match other.side(local(&pose, readback::vertex_point(own, far).unwrap() - pv)) { 0 => SideCode::In, 1 => SideCode::On, _ => SideCode::Out };
                                    let got: Vec<SideCode> = r.naming.edge_classes.iter().filter(|row| row.operand == op && row.vertex == v && row.edge == e && row.starts == starts).map(|row| row.class).collect();
                                    match got.as_slice() {
                                        [] => { missing += 1; writeln!(f, "MISSING {cell} {op:?} {e:?} want {want:?}").unwrap(); }
                                        [g] if *g == want => {}
                                        [g] => { wrong += 1; wrong_cells.push(format!("{cell} {op:?} {e:?} got {g:?} want {want:?}")); writeln!(f, "WRONG {cell} {op:?} {e:?} got {g:?} want {want:?}").unwrap(); }
                                        many => { doubled += 1; writeln!(f, "DOUBLED {cell} {op:?} {e:?} {many:?}").unwrap(); }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let summary = format!("built {built} tier3-fail {tier} wrong {wrong} missing {missing} doubled {doubled} panics {panics} errs {errs:?}");
    writeln!(f, "SUMMARY {summary}").unwrap();
    eprintln!("SUMMARY {summary}");
    for w in wrong_cells.iter().take(30) { eprintln!("WRONG {w}"); }
}
