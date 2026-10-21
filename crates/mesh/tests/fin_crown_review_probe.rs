//! Review probe for PR 5534 (not for merge): dumps the posed crowns of
//! every `posed_crown` ring and every fin-scene cell, tessellated, with
//! each cell's outcome and volume, for out-of-tree exact checks.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use std::fmt::Write as _;
use topo::test_support::meeting::{apex_pyramid, bearing, corners, fin, posed_box, posed_crown, poses};

/// The battery's saddle crown (`a_vertex_read_again_classes_every_edge::crown`).
fn crown() -> Vec<[f64; 3]> {
    [0.0, 90.0, 180.0, 270.0]
        .iter()
        .zip([0.2, -0.2, 0.2, -0.2])
        .map(|(&b, z)| bearing(b, 0.5, z))
        .collect()
}
use topo::{BooleanResult, intersect, subtract, union};

fn t() -> Tol {
    Tol::witness()
}

fn tris(body: &topo::Body<f64>) -> String {
    let m = mesh::tessellate(body, 0.01, t()).unwrap();
    let mut s = String::from("[");
    let mut first = true;
    for p in &m.patches {
        for tr in &p.triangles {
            if !first {
                s.push(',');
            }
            first = false;
            s.push('[');
            for (k, i) in tr.iter().enumerate() {
                let q = m.positions[*i as usize];
                if k > 0 {
                    s.push(',');
                }
                write!(s, "[{:?},{:?},{:?}]", q.x, q.y, q.z).unwrap();
            }
            s.push(']');
        }
    }
    s.push(']');
    s
}

fn vol(body: &topo::Body<f64>) -> String {
    match topo::mass_properties(body, t()) {
        Ok(m) => format!("{:?}", m.volume),
        Err(e) => format!("\"{e:?}\"").replace('\n', " "),
    }
}

#[test]
fn fin_crown_review_dump() {
    let path = std::env::var("FIN_PROBE_OUT").expect("FIN_PROBE_OUT");
    let mut out = String::new();
    let eps = t().eps();
    let dir = |a: f64, z: f64, l: f64| [l * a.cos(), l * a.sin(), l * z];
    let dented = vec![
        dir(-0.01, 1.5e-5 * 0.06f64.sin(), 0.4),
        dir(0.05, 0.0, 1e-3),
        dir(0.5, 0.0, 0.4),
        dir(2.0, 3e-7, 0.4),
        dir(3.5, -3e-7, 0.4),
        dir(5.0, 3e-7, 0.4),
    ];
    let around = |heights: &[f64]| -> Vec<[f64; 3]> {
        let n = heights.len();
        (0..n)
            .map(|i| {
                let a = std::f64::consts::TAU * (i as f64 + 0.13 * (i % 2) as f64) / n as f64;
                let r = 1.0 + 0.1 * (i % 3) as f64;
                [r * a.cos(), r * a.sin(), heights[i]]
            })
            .collect()
    };
    let fin_builds = geom_core::Band::linear(t()).unwrap().zero() < 1e-8;
    let mut rings: Vec<(String, Vec<[f64; 3]>, [f64; 3])> = vec![("saddle crown".into(), crown(), [0.0, 0.0, -1.0])];
    if fin_builds {
        rings.push(("fin new".into(), fin(), [0.0, 0.0, -0.6]));
        rings.push(("fin old".into(), fin(), [0.0, 0.2, -0.6]));
        rings.push(("dented".into(), dented, [0.0, 0.0, -0.6]));
    }
    for (name, h) in [
        ("three valleys", vec![0.3, 0.0, 0.3, 0.0, 0.3, 0.0]),
        ("V-groove", vec![0.3, 0.0, 0.25, 0.0, 0.2]),
        ("three valleys uneven", vec![0.3, 0.0, 0.25, 0.0, 0.2, 0.0]),
    ] {
        rings.push((name.into(), around(&h), [0.0, 0.0, -1.0]));
    }
    let c2 = vec![
        [0.4760, 1.3181, 0.3518],
        [-0.7080, 0.8592, 0.1597],
        [-1.1890, 0.5653, 0.1028],
        [-1.1262, -0.2072, 0.0],
        [0.4166, -1.4394, 0.2963],
    ];
    for (name, ring) in [
        ("C2", c2),
        ("three corners", around(&[0.2, 0.0, 0.3])),
        ("four corners", around(&[0.0, 0.25, 0.3, 0.15])),
        ("five corners", around(&[0.3, 0.2, 0.25, 0.1, 0.0])),
        ("six corners", around(&[0.3, 0.05, 0.0, 0.4, 0.2, 0.25])),
    ] {
        let low = ring.iter().position(|c| c[2] == 0.0).unwrap();
        for s in [0.0, 0.6, -0.6, 0.9, -0.9] {
            let mut ring = ring.clone();
            ring[low][2] = s * eps;
            rings.push((format!("{name} at {s} eps"), ring, [0.0, 0.0, -1.0]));
        }
    }
    for pose in poses() {
        for (name, ring, bottom) in &rings {
            let b = posed_crown(ring, *bottom, &pose, t());
            let census = topo::validate_pseudomanifold(&b, &topo::ContactRecords::default(), t());
            eprintln!("RINGCENSUS {name} | {} | {}", pose.label, match &census { Ok(()) => "ok".to_string(), Err(e) => format!("{e:?}").chars().take(300).collect() });
            writeln!(
                out,
                "{{\"kind\":\"ring\",\"name\":\"{name}\",\"pose\":\"{}\",\"volume\":{},\"tris\":{}}}",
                pose.label,
                vol(&b),
                tris(&b)
            )
            .unwrap();
        }
        if !fin_builds {
            continue;
        }
        let block = posed_box("block", [(1.0, 2.0), (0.5, 1.5), (0.3, 1.5)], &pose, t());
        writeln!(out, "{{\"kind\":\"block\",\"pose\":\"{}\",\"volume\":{},\"tris\":{}}}", pose.label, vol(&block), tris(&block)).unwrap();
        for (bname, bottom) in [("new", [0.0, 0.0, -0.6]), ("old", [0.0, 0.2, -0.6])] {
            let fin_b = posed_crown(&fin(), bottom, &pose, t());
            let void = match subtract(&block, &fin_b, t()) {
                Ok(BooleanResult::Body(r)) => r.body,
                other => panic!("void: {:?}", other.map(|_| ())),
            };
            writeln!(out, "{{\"kind\":\"void\",\"bottom\":\"{bname}\",\"pose\":\"{}\",\"volume\":{},\"tris\":{}}}", pose.label, vol(&void), tris(&void)).unwrap();
            for (k, &(br, rise)) in [(10.0, 0.3), (130.0, 0.3), (130.0, -0.3), (250.0, 0.3), (250.0, -0.3)].iter().enumerate() {
                let x = apex_pyramid(&corners(br, rise, 0.5), &pose, t());
                if bname == "new" {
                    writeln!(out, "{{\"kind\":\"probe\",\"k\":{k},\"pose\":\"{}\",\"volume\":{},\"tris\":{}}}", pose.label, vol(&x), tris(&x)).unwrap();
                }
                for (scene, y) in [("fin", &fin_b), ("fin void", &void)] {
                    for (op, r) in [
                        ("x - y", subtract(&x, y, t())),
                        ("y - x", subtract(y, &x, t())),
                        ("x u y", union(&x, y, t())),
                        ("y u x", union(y, &x, t())),
                        ("x n y", intersect(&x, y, t())),
                        ("y n x", intersect(y, &x, t())),
                    ] {
                        let (outcome, v, tr) = match r {
                            Ok(BooleanResult::Body(r)) => ("built".to_string(), vol(&r.body), tris(&r.body)),
                            Ok(BooleanResult::Empty) => ("empty".to_string(), "0.0".into(), "[]".into()),
                            Err(e) => (format!("{e:?}").replace('"', "'").replace('\n', " "), "null".into(), "[]".into()),
                        };
                        writeln!(
                            out,
                            "{{\"kind\":\"cell\",\"bottom\":\"{bname}\",\"scene\":\"{scene}\",\"k\":{k},\"op\":\"{op}\",\"pose\":\"{}\",\"outcome\":\"{outcome}\",\"volume\":{v},\"tris\":{tr}}}",
                            pose.label
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
    std::fs::write(path, out).unwrap();
}

/// Does tier 3′'s census, run over the old fin crown with no contacts,
/// see the crossing the at-rest gate (tier 3 alone) passes?
#[test]
fn fin_crown_review_census_on_the_old_crown() {
    if geom_core::Band::linear(t()).unwrap().zero() >= 1e-8 {
        return;
    }
    for pose in poses() {
        for bottom in [[0.0, 0.2, -0.6], [0.0, 0.0, -0.6]] {
            let b = posed_crown(&fin(), bottom, &pose, t());
            let r = topo::validate_pseudomanifold(&b, &topo::ContactRecords::default(), t());
            eprintln!("CENSUS {bottom:?} {}: {r:?}", pose.label);
            // Measured: the old crown pierces itself at every ε the fin
            // builds at; the new one touches itself in band at 1e-9
            // (`MEET` 8.7e-10 off the bottom walls) and is clean at 1e-12.
            let in_band = t().eps() > 8.8e-10;
            assert_eq!(r.is_err(), bottom[1] != 0.0 || in_band, "{bottom:?} {}", pose.label);
        }
    }
}
