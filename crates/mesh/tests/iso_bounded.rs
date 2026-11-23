//! **A cylinder or sphere face bounded by iso lines alone** — a pipe wall
//! notched or holed, a ball less a wedge — is routed to the chart lane as
//! `tessellate::Lane::IsoBounded` (`walk::runs_on_iso_lines`), where a
//! cylinder face lays no lattice rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind;
use geom_core::{Affine3, Point3, Vec3};
use sweep::test_support::{brick, finished};
use topo::{AtRestBody, Body};

use crate::reach_probes::{geometry, op_of, posed, rod, tol};

/// The triangles `mesh` holds on `body`'s cylinder faces.
fn cylinder_triangles(body: &Body<f64>, mesh: &mesh::Mesh) -> usize {
    mesh.patches
        .iter()
        .filter(|p| body.face_surface(p.face).unwrap().kind() == SurfaceKind::Cylinder)
        .map(|p| p.triangles.len())
        .sum()
}

/// `body` at `delta`: it meshes, `check_mesh` and the body's geometry
/// hold ([`geometry`]), and its cylinder faces' triangles.
fn meshed_cylinder_triangles(what: &str, body: &Body<f64>, delta: f64) -> usize {
    let mesh = mesh::tessellate(body, delta, tol())
        .unwrap_or_else(|e| panic!("{what}, δ = {delta:e}: meshes, got {e:?}"));
    mesh::validate::check_mesh(&mesh)
        .unwrap_or_else(|e| panic!("{what}, δ = {delta:e}: check_mesh, got {e:?}"));
    geometry(body, &mesh, delta).unwrap_or_else(|e| panic!("{what}, δ = {delta:e}: {e}"));
    cylinder_triangles(body, &mesh)
}

/// A bar `x × (−w, w) × z` turned `turn` about the pipe's axis.
fn bar(x: (f64, f64), w: f64, z: (f64, f64), turn: f64) -> AtRestBody<f64> {
    posed(
        "the bar",
        brick(x, (-w, w), z, tol()),
        Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), turn),
    )
}

/// The notched and holed walls of a pipe of radius `r` over
/// `z ∈ (z0, z0 + h)`, each named: a notch down from the top rim across
/// the chart's seam and beside it, a narrow and a wide one, two notches,
/// and a bar through (a notch across the seam and a hole).
fn walls(r: f64, z0: f64, h: f64) -> Vec<(String, Body<f64>)> {
    let pipe = finished("the pipe", rod(r, z0, h), tol());
    let top = z0 + h;
    let notch = |w: f64, depth: f64, turn: f64| {
        bar((0.0, 1.1 * r), w * r, (top - depth * h, top + 0.5 * h), turn)
    };
    let mut out: Vec<(String, Body<f64>)> = [
        ("a notch across the seam", notch(0.3, 0.5, 0.0)),
        ("a notch beside the seam", notch(0.3, 0.5, 1.0)),
        ("a narrow deep notch", notch(0.1, 0.875, 1.0)),
        ("a wide shallow notch", notch(0.6, 0.125, 1.0)),
    ]
    .into_iter()
    .map(|(what, b)| (what.to_owned(), op_of(&pipe, &b, "A ∖ B")))
    .collect();
    let two = topo::union(&notch(0.3, 0.5, 1.0), &notch(0.3, 0.5, 2.5), tol()).unwrap();
    let two = finished("two bars", Body::clone(&two.body().unwrap().body), tol());
    out.push(("two notches".to_owned(), op_of(&pipe, &two, "A ∖ B")));
    let through = bar((-1.1 * r, 1.1 * r), 0.3 * r, (z0 + 0.4 * h, z0 + 0.6 * h), 0.0);
    out.push(("a bar through".to_owned(), op_of(&pipe, &through, "A ∖ B")));
    out
}

/// FNV-1a over a mesh's positions' bits, its patches' faces and
/// triangles, and its boundary polylines.
fn digest(mesh: &mesh::Mesh) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut eat = |x: u64| {
        for b in x.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
    };
    for p in &mesh.positions {
        eat(p.x.to_bits());
        eat(p.y.to_bits());
        eat(p.z.to_bits());
    }
    for patch in &mesh.patches {
        for t in &patch.triangles {
            for &i in t {
                eat(u64::from(i));
            }
        }
    }
    for b in &mesh.boundaries {
        for &i in &b.points {
            eat(u64::from(i));
        }
    }
    h
}

#[test]
fn probe_counts() {
    for (pipe, r, z0, h) in [("unit", 1.0, -2.0, 4.0), ("slender", 0.1, -2.0, 4.0), ("large", 10.0, -2.0, 4.0)] {
        let plain = rod(r, z0, h);
        for rel in [0.1, 0.01, 0.001] {
            let delta = rel * r;
            let reference = meshed_cylinder_triangles("plain", &plain, delta);
            let at_target = meshed_cylinder_triangles("plain", &plain, delta * 0.5);
            for (what, body) in walls(r, z0, h) {
                let n = meshed_cylinder_triangles(&what, &body, delta);
                #[allow(clippy::cast_precision_loss)]
                let ratio = n as f64 / at_target as f64;
                let d = digest(&mesh::tessellate(&body, delta, tol()).unwrap());
                println!("PROBE\t{pipe}\t{what}\t{delta}\t{n}\t{reference}\t{at_target}\t{ratio:.3}\t{d:016x}");
            }
        }
    }
}
