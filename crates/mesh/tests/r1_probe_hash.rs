//! R1 review probe (MESH-3, PR 1460): independent byte-stability
//! spot-check. FNV-hashes every position bit, triangle index and
//! boundary polyline of a body subset at two deltas; run at the PR
//! head and at merge-base 7b0bfab76 under the same ambient eps, the
//! printed hashes must be identical. Review probe only.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use geom_core::Tol;
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, revolve};

fn p2(x: f64, y: f64) -> geom_core::Point2<f64> {
    geom_core::Point2::new(x, y)
}

fn vp(loops: Vec<ProfileLoop<f64>>) -> profile::ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .unwrap()
}

fn axis_y() -> sweep::RevolveAxis<f64> {
    sweep::RevolveAxis {
        origin: geom_core::Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    }
}

fn fnv(bytes: impl Iterator<Item = u64>) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in bytes {
        for i in 0..8 {
            h ^= (b >> (8 * i)) & 0xff;
            h = h.wrapping_mul(0x100000001b3);
        }
    }
    h
}

fn hash_mesh(m: &mesh::Mesh) -> (u64, u64, u64) {
    let hp = fnv(m
        .positions
        .iter()
        .flat_map(|p| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()].into_iter()));
    let ht = fnv(m.patches.iter().flat_map(|p| {
        p.triangles
            .iter()
            .flat_map(|t| t.iter().map(|&i| u64::from(i)))
    }));
    let hb = fnv(m
        .boundaries
        .iter()
        .flat_map(|b| b.points.iter().map(|&i| u64::from(i))));
    (hp, ht, hb)
}

#[test]
fn r1_probe_corpus_hashes() {
    let tol = Tol::witness();
    let ball = revolve(
        &vp(vec![ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -1.0), 1.0),
            ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ])]),
        axis_y(),
        Revolution::Full,
        tol,
    )
    .unwrap()
    .body;
    let cone = revolve(
        &vp(vec![ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(1.0, 0.0), 0.0),
            ProfileVertex::new(p2(0.0, 1.5), 0.0),
        ])]),
        axis_y(),
        Revolution::Full,
        tol,
    )
    .unwrap()
    .body;
    // The rho = 0.1 band from the PR's own demonstration row.
    let rho = 0.1f64;
    let (h, rc) = (0.5f64.sin(), 0.5f64.cos());
    let yt = (1.0 - rho * rho).sqrt();
    let bulge = ((yt.atan2(rho) - h.atan2(rc)) / 4.0).tan();
    let band = revolve(
        &vp(vec![ProfileLoop::new(vec![
            ProfileVertex::new(p2(rc, h), bulge),
            ProfileVertex::new(p2(rho, yt), 0.0),
            ProfileVertex::new(p2(0.3, 1.3), 0.0),
            ProfileVertex::new(p2(1.1, 0.9), 0.0),
        ])]),
        axis_y(),
        Revolution::Full,
        tol,
    )
    .unwrap()
    .body;
    for (name, body) in [("ball", &ball), ("cone", &cone), ("band", &band)] {
        for delta in [0.1f64, 0.004] {
            let m = mesh::tessellate(body, delta, tol).unwrap();
            let (hp, ht, hb) = hash_mesh(&m);
            println!("HASH {name} d={delta}: pos {hp:016x} tri {ht:016x} bnd {hb:016x}");
        }
    }
}
