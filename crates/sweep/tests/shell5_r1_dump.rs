//! **R1 review instrument for SHELL-5 (PR #2159): the single-shell
//! differential.** Prints a deterministic dump of every single-shell
//! operand the shell verb takes in this crate's own fixture vocabulary
//! (sealed and opened, planar and revolved), so the SAME file compiled
//! at the merge base and at the PR head can be diffed line by line.
//! It asserts nothing beyond "the fixture builds"; the diff is the
//! verdict. Kept free of every symbol this PR adds so it compiles on
//! both trees.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, FaceKey};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn prism(pts: &[(f64, f64)], h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a polygon is a valid profile");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("a polygon extrudes")
        .body
}

fn revolved(pts: &[(f64, f64)]) -> Body<f64> {
    let lp = ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the meridian is a valid profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body
}

/// Faces whose plane has the given unit normal (chart normal, not
/// outward) and plane constant `n·x = c`.
fn planes_where(body: &Body<f64>, n: (f64, f64, f64), c: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (normal.x - n.0).abs() < 1e-9 && (normal.y - n.1).abs() < 1e-9
                        && (normal.z - n.2).abs() < 1e-9
                        && (origin.x * n.0 + origin.y * n.1 + origin.z * n.2 - c).abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .collect()
}

fn dump(label: &str, out: &Result<topo::Shelled<f64>, topo::ShellError<f64>>) {
    match out {
        Err(e) => println!("[dump] {label}: Err {e}"),
        Ok(s) => {
            let b = &s.body;
            println!(
                "[dump] {label}: solids={} shells={} faces={} edges={} vertices={}",
                b.solids().count(),
                b.shells().count(),
                b.faces().count(),
                b.edges().count(),
                b.vertices().count()
            );
            for (k, sh) in b.shells() {
                println!("[dump] {label}: shell {k:?} solid={:?} faces={:?}", sh.solid, sh.faces);
            }
            for (k, f) in b.faces() {
                println!(
                    "[dump] {label}: face {k:?} shell={:?} sense={} rings={} surface={:?}",
                    f.shell,
                    f.sense,
                    f.rings.len(),
                    b.get_surface(f.surface)
                );
            }
            for (k, e) in b.edges() {
                println!("[dump] {label}: edge {k:?} curve={:?}", b.get_curve_geom(e.curve));
            }
            for (k, v) in b.vertices() {
                println!("[dump] {label}: vertex {k:?} point={:?}", b.get_point(v.point));
            }
            let n = &s.naming;
            println!(
                "[dump] {label}: naming outer={:?} inner={:?} inner_edges={:?} inner_vertices={:?} rims={:?} dead={:?}",
                n.outer, n.inner, n.inner_edges, n.inner_vertices, n.rims, n.dead
            );
            match topo::mass_properties(b, Tol::witness()) {
                Ok(p) => println!("[dump] {label}: volume={:?} area={:?}", p.volume, p.surface_area),
                Err(e) => println!("[dump] {label}: props Err {e:?}"),
            }
            println!(
                "[dump] {label}: tier3={:?}",
                topo::validate_geometric(b, Tol::witness())
            );
        }
    }
}

/// The corpus. Run with `--nocapture`, grep `[dump]`, diff across trees.
#[test]
fn r1_dump_single_shell_corpus() {
    let tol = Tol::witness();
    let boxy = prism(&[(0.0, 0.0), (2.0, 0.0), (2.0, 3.0), (0.0, 3.0)], 4.0);
    let ell = prism(
        &[(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (1.0, 1.0), (1.0, 3.0), (0.0, 3.0)],
        2.0,
    );
    let vessel = revolved(&[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0)]);
    let tube = revolved(&[(0.6, 0.0), (1.0, 0.0), (1.0, 2.0), (0.6, 2.0)]);
    // A bellied pot: a meridian with a sphere-like belly is what the
    // teapot tour scene is made of; here a cone + cylinder + caps
    // stands in (the meridian is a polyline, so every wall is exact).
    let pot = revolved(&[(0.0, 0.0), (0.8, 0.0), (1.2, 1.0), (1.0, 2.0), (0.0, 2.0)]);

    dump("box sealed", &topo::shell(&boxy, 0.25, tol));
    dump("ell sealed", &topo::shell(&ell, 0.2, tol));
    dump("vessel sealed", &topo::shell(&vessel, 0.2, tol));
    dump("tube sealed", &topo::shell(&tube, 0.1, tol));
    dump("pot sealed", &topo::shell(&pot, 0.15, tol));

    let top = planes_where(&boxy, (0.0, 0.0, 1.0), 4.0);
    dump("box opened top", &topo::shell_open(&boxy, 0.25, &top, tol));
    let bottom = planes_where(&boxy, (0.0, 0.0, -1.0), 0.0);
    let mut both = top.clone();
    both.extend(bottom);
    dump("box opened top+bottom", &topo::shell_open(&boxy, 0.25, &both, tol));
    let ell_top = planes_where(&ell, (0.0, 0.0, 1.0), 2.0);
    dump("ell opened top", &topo::shell_open(&ell, 0.2, &ell_top, tol));
    let vessel_top = planes_where(&vessel, (0.0, 1.0, 0.0), 2.0);
    dump("vessel opened top", &topo::shell_open(&vessel, 0.2, &vessel_top, tol));
    let pot_top = planes_where(&pot, (0.0, 1.0, 0.0), 2.0);
    dump("pot opened top", &topo::shell_open(&pot, 0.15, &pot_top, tol));
    let tube_top = planes_where(&tube, (0.0, 1.0, 0.0), 2.0);
    dump("tube opened top", &topo::shell_open(&tube, 0.1, &tube_top, tol));
}
