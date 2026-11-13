//! Dumps the two bracket bodies of `band_mixed_corner` as triangle soups for `draw_soup.py`. Run it as an ignored test in a throwaway crate with `sweep` (test-support) and `mesh`, `BRACKET3_OUT` naming the output directory.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]
use geom::Surface;
use geom_core::{Point2, Tol};
use std::io::Write;
use sweep::blend::build::fillet_edges;
use sweep::chamfer::chamfer_edges;
use sweep::test_support::{at_rest, corners, cylinder_of_arcs_at, prism_at, realized};
use topo::Body;
use topo::boolean::BooleanOp;

fn tol() -> Tol { Tol::witness() }

fn bracket(bore: bool) -> Body<f64> {
    let plate = prism_at(corners(&[(0.0, 0.0), (6.0, 0.0), (6.0, 4.0), (0.0, 4.0)]), 0.0, 0.5, tol());
    let boss = prism_at(corners(&[(0.0, 0.5), (0.5, 0.5), (0.5, 3.5), (0.0, 3.5)]), 0.25, 1.75, tol());
    let body = realized(BooleanOp::Union, &plate, &boss, tol());
    if !bore { return body; }
    let bore = cylinder_of_arcs_at(2, 0.3, Point2::new(3.0, 2.0), -0.5, 1.5, tol());
    realized(BooleanOp::Subtract, &body, &bore, tol())
}

fn dump(body: &Body<f64>, path: &str) {
    let m = mesh::tessellate(body, 2e-3, tol()).expect("tessellate");
    let mut f = std::fs::File::create(path).unwrap();
    for p in &m.positions { writeln!(f, "v {} {} {}", p.x, p.y, p.z).unwrap(); }
    for patch in &m.patches {
        let kind = match body.face_surface(patch.face) {
            Some(Surface::Plane { .. }) => 0,
            Some(Surface::Cylinder { .. }) => 1,
            Some(Surface::Torus { .. }) => 2,
            Some(Surface::Sphere { .. }) => 3,
            _ => 4,
        };
        for t in &patch.triangles { writeln!(f, "f {} {} {} {kind}", t[0], t[1], t[2]).unwrap(); }
    }
}

#[test]
#[ignore = "scratch render dump"]
fn band_bracket3_render_dump() {
    let dir = std::env::var("BRACKET3_OUT").unwrap();
    let b = bracket(true);
    let mut all = topo::query::all_edges(&b); all.sort_unstable();
    let f = fillet_edges(&at_rest(&b, tol()), &all, 0.05, tol()).map_err(|r| r.error).unwrap();
    dump(&f.body.clone().into_body(), &format!("{dir}/bracket-fillet.soup"));
    let b = bracket(false);
    let mut all = topo::query::all_edges(&b); all.sort_unstable();
    let c = chamfer_edges(&at_rest(&b, tol()), &all, 0.05, tol()).map_err(|r| r.error).unwrap();
    dump(&c.body.clone().into_body(), &format!("{dir}/bracket-chamfer.soup"));
}
