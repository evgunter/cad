//! R2 review instrument for MESH-10: an independent byte digest over
//! the suite's bodies at three deltas (FNV-1a over every position bit
//! pattern, triangle index and boundary polyline). Compiles at the
//! merge base and at the head; run at both under one ambient ε and
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
use topo::Body;

fn fnv(words: impl IntoIterator<Item = u64>) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for w in words {
        for b in w.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x100_0000_01b3);
        }
    }
    h
}

fn mesh_words(m: &mesh::Mesh) -> Vec<u64> {
    let mut w: Vec<u64> = Vec::new();
    for p in &m.positions {
        w.extend([p.x.to_bits(), p.y.to_bits(), p.z.to_bits()]);
    }
    for fp in &m.patches {
        for t in &fp.triangles {
            w.extend(t.iter().map(|i| u64::from(*i)));
        }
    }
    for pl in &m.boundaries {
        w.push(pl.points.len() as u64);
        w.extend(pl.points.iter().map(|i| u64::from(*i)));
    }
    w
}

/// **Own digest instrument.** FNV-1a over every position bit pattern,
/// every triangle index and every boundary polyline of the suite's
/// bodies at three deltas — run at base and head under one ambient ε.
#[test]
fn m10r2_independent_byte_digest() {
    println!("M10R2 eps={:e}", Tol::witness().eps());
    let bodies: Vec<(&str, Body<f64>)> = vec![
        ("ball", ball()),
        ("cone", cone()),
        ("l_prism", l_prism()),
        ("holed_prism", holed_prism()),
        ("washer", washer()),
        ("donut", donut()),
        ("wedge", wedge()),
        ("sphere_wedge", sphere_wedge(2.0)),
    ];
    for (name, b) in &bodies {
        for d in [0.1f64, 0.02, 0.004] {
            let m = mesh::tessellate(b, d, Tol::witness()).unwrap();
            println!(
                "M10R2 {name} d={d} pos={} h={:016x}",
                m.positions.len(),
                fnv(mesh_words(&m))
            );
        }
    }
}
