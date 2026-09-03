//! R2 review probe for MESH-3, adopted: independent byte-stability
//! instrument. Prints an FNV-1a hash over every position's `to_bits`,
//! every triangle index and every boundary polyline, for a body tour
//! at three deltas — run it at two revisions under one ambient ε and
//! diff the lines.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use crate::common;
use common::*;
use geom_core::Tol;
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use topo::Body;

fn fnv(bytes: &[u8], h: &mut u64) {
    for b in bytes {
        *h ^= u64::from(*b);
        *h = h.wrapping_mul(0x100_0000_01b3);
    }
}

fn band(rho: f64) -> Body<f64> {
    let (hh, rc) = (0.5f64.sin(), 0.5f64.cos());
    let yt = (1.0 - rho * rho).sqrt();
    let bulge = ((yt.atan2(rho) - hh.atan2(rc)) / 4.0).tan();
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(rc, hh), bulge),
        ProfileVertex::new(p2(rho, yt), 0.0),
        ProfileVertex::new(p2(0.3, 1.3), 0.0),
        ProfileVertex::new(p2(1.1, 0.9), 0.0),
    ]);
    sweep::revolve(
        &validated(vec![lp]),
        axis_y(),
        sweep::Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

#[test]
fn r2_byte_stability_report() {
    let bodies: Vec<(&str, Body<f64>)> = vec![
        ("ball", ball()),
        ("cone", cone()),
        ("l_prism", l_prism()),
        ("washer", washer()),
        ("donut", donut()),
        ("sphere_wedge", sphere_wedge(2.0)),
        ("band_0.1", band(0.1)),
    ];
    let mut lines = Vec::new();
    for (name, b) in &bodies {
        for d in [0.1f64, 0.02, 0.004] {
            let m = mesh::tessellate(b, d, Tol::witness()).unwrap();
            let mut h = 0xcbf2_9ce4_8422_2325u64;
            for p in &m.positions {
                for c in [p.x, p.y, p.z] {
                    fnv(&c.to_bits().to_le_bytes(), &mut h);
                }
            }
            let mut nt = 0usize;
            for fp in &m.patches {
                for t in &fp.triangles {
                    nt += 1;
                    for i in t {
                        fnv(&i.to_le_bytes(), &mut h);
                    }
                }
            }
            for pl in &m.boundaries {
                for i in &pl.points {
                    fnv(&i.to_le_bytes(), &mut h);
                }
            }
            lines.push(format!(
                "{name} d={d} n={} t={} => {h:016x}",
                m.positions.len(),
                nt
            ));
        }
    }
    assert_eq!(
        lines.len(),
        bodies.len() * 3,
        "every body hashed at every delta"
    );
    println!("R2 HASHES\n{}", lines.join("\n"));
}
