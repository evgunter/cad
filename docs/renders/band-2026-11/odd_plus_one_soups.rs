//! Dumps the odd-plus-one chamfer scenes of `band_mixed_corner_odd_plus_one` (PR 7038), each before and after, as triangle soups for `draw_odd_plus_one.py`. It reads the suite's own fixtures, so it runs as a module of `crates/sweep/tests/all.rs` (add `#[path = "../../../docs/renders/band-2026-11/odd_plus_one_soups.rs"] mod odd_plus_one_soups;` locally, do not commit that line), as an ignored test, `ODD_PLUS_ONE_OUT` naming the output directory.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]
use core::f64::consts::PI;
use geom::Surface;
use std::io::Write;
use sweep::chamfer::chamfer_edges;
use sweep::test_support::at_rest;
use topo::{Body, EdgeKey};

use crate::band_mixed_corner::{boreless, flush, flush_at, inset, tol};
use crate::band_mixed_corner_odd_plus_one::bracket_request;

fn dump(body: &Body<f64>, path: &str) {
    let m = mesh::tessellate(body, 2e-3, tol()).expect("tessellate");
    let mut f = std::fs::File::create(path).unwrap();
    for p in &m.positions {
        writeln!(f, "v {} {} {}", p.x, p.y, p.z).unwrap();
    }
    for patch in &m.patches {
        let kind = match body.face_surface(patch.face) {
            Some(Surface::Plane { .. }) => 0,
            Some(Surface::Cylinder { .. }) => 1,
            Some(Surface::Torus { .. }) => 2,
            Some(Surface::Sphere { .. }) => 3,
            _ => 4,
        };
        for t in &patch.triangles {
            writeln!(f, "f {} {} {} {kind}", t[0], t[1], t[2]).unwrap();
        }
    }
}

fn scene(dir: &str, name: &str, body: &Body<f64>, edges: &[EdgeKey], s: f64) {
    let out = chamfer_edges(&at_rest(body, tol()), edges, s, tol())
        .map_err(|r| r.error)
        .unwrap();
    dump(body, &format!("{dir}/{name}-before.soup"));
    dump(&out.body.into_body(), &format!("{dir}/{name}-after.soup"));
}

#[test]
#[ignore = "scratch render dump"]
fn odd_plus_one_render_dump() {
    let dir = std::env::var("ODD_PLUS_ONE_OUT").unwrap();
    let (body, [odd, y, _]) = flush::<f64>();
    scene(&dir, "odd-plus-one-flush", &body, &[odd, y], 0.1);
    let (body, [odd, y, _]) = inset::<f64>();
    scene(&dir, "odd-plus-one-inset", &body, &[odd, y], 0.1);
    let (body, [odd, y, _]) = flush_at::<f64>(2.0 * PI / 3.0);
    scene(&dir, "odd-plus-one-flush-120", &body, &[odd, y], 0.1);
    let body = boreless::<f64>();
    scene(
        &dir,
        "odd-plus-one-bracket",
        &body,
        &bracket_request(&body),
        0.05,
    );
}
