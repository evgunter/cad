//! **R1 review probe for MESH-10 (issue 1562): an independent D9
//! digest.** One FNV-1a hash over every position bit pattern, every
//! triangle index and the triangle/position counts of a fixed corpus
//! of bodies, at three chord tolerances. The corpus and the instrument
//! are this reviewer's, not the unit's, and the file uses only the
//! public mesh door so that it compiles unchanged on the merge base:
//! run it there and here under the same ambient eps and the printed
//! digest must be identical (D9 — no mesh byte moves on a body that
//! meshed before).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use crate::common;

use common::witness_bodies;
use common::*;
use geom_core::Tol;
use topo::Body;

fn fnv(h: &mut u64, x: u64) {
    for b in x.to_le_bytes() {
        *h ^= u64::from(b);
        *h = h.wrapping_mul(0x0100_0000_01b3);
    }
}

/// Hash one body's mesh at one delta; `None` when the body refuses
/// (hashed as a distinguished value so a refusal that becomes a mesh
/// moves the digest).
fn hash_body(body: &Body<f64>, delta: f64) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    match mesh::tessellate(body, delta, Tol::witness()) {
        Ok(m) => {
            fnv(&mut h, m.positions.len() as u64);
            fnv(
                &mut h,
                m.patches.iter().map(|q| q.triangles.len()).sum::<usize>() as u64,
            );
            for p in &m.positions {
                fnv(&mut h, p.x.to_bits());
                fnv(&mut h, p.y.to_bits());
                fnv(&mut h, p.z.to_bits());
            }
            for q in &m.patches {
                for t in &q.triangles {
                    fnv(&mut h, t[0] as u64);
                    fnv(&mut h, t[1] as u64);
                    fnv(&mut h, t[2] as u64);
                }
            }
        }
        Err(e) => {
            fnv(&mut h, 0xdead_beef);
            for b in format!("{e:?}").bytes() {
                fnv(&mut h, u64::from(b));
            }
        }
    }
    h
}

#[test]
fn r1_d9_digest_over_a_reviewer_corpus() {
    let corpus: Vec<(&str, Body<f64>)> = vec![
        ("l_prism", l_prism()),
        ("holed_prism", holed_prism()),
        ("rounded_prism", rounded_prism()),
        ("ball", ball()),
        ("cone", cone()),
        ("cone_wedge", cone_wedge(1.0, 1.2)),
        ("sphere_wedge", sphere_wedge(1.2)),
        ("washer", washer()),
        ("donut", donut()),
        ("wedge", wedge()),
        ("axis_wedge", axis_wedge()),
        ("keyway", witness_bodies::keyway().0),
    ];
    let mut total: u64 = 0xcbf2_9ce4_8422_2325;
    for delta in [0.05_f64, 0.1, 0.3] {
        for (name, body) in &corpus {
            let h = hash_body(body, delta);
            println!("R1-D9 delta={delta} {name} {h:016x}");
            fnv(&mut total, h);
        }
    }
    println!("R1-D9 TOTAL {total:016x}");
}
