//! **SHELL-10 review, R2 differential instrument** — a corpus of the
//! reviewer's choosing beside the unit's four: sf2b's revolved shapes
//! (the drum, the cone frustum, the bellied pot, the two bellied
//! vases) sealed and opened at their top, and the two simultaneous
//! doors called DIRECTLY on two- and four-solid bodies naming one
//! solid. Prints `[r2dump]` lines: the whole body's cache rows (SHELL-9's
//! reading) and every solid's deep dump (SHELL-8's). Uses nothing
//! SHELL-10 added, so the same file compiles at the merge base; the
//! sorted diff across trees is the verdict.
//!
//! Run with `--nocapture --test-threads=1` (a parallel run interleaves
//! lines and the grep then drops them on one side).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom_core::{Point2, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, SolidKey};

use crate::shell8_common::{band, beside, cap, charts_of, deep_dump, tol};
use crate::shell9_rows::rows;
use crate::verbs_shell::{boxy, tube, vessel};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn revolved(lp: ProfileLoop<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the meridian validates");
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

fn drum() -> Body<f64> {
    revolved(ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(3.0 / 64.0, 0.0), 0.0),
        ProfileVertex::new(p2(3.0 / 64.0, 8.0 / 64.0), 0.0),
        ProfileVertex::new(p2(0.0, 8.0 / 64.0), 0.0),
    ]))
}

fn cone_frustum() -> Body<f64> {
    revolved(ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(4.0 / 64.0, 0.0), 0.0),
        ProfileVertex::new(p2(2.0 / 64.0, 8.0 / 64.0), 0.0),
        ProfileVertex::new(p2(0.0, 8.0 / 64.0), 0.0),
    ]))
}

fn bellied(r0: f64, y0: f64, r1: f64, y1: f64, cy: f64, sign: f64) -> Body<f64> {
    let c = p2(0.0, cy);
    let (dx0, dy0) = (r0 - c.x, y0 - c.y);
    let (dx1, dy1) = (r1 - c.x, y1 - c.y);
    let sweep = (dx0 * dy1 - dy0 * dx1).atan2(dx0 * dx1 + dy0 * dy1);
    let bulge = sign * (sweep / 4.0).tan();
    revolved(RawLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(r0, y0), bulge),
        ProfileVertex::new(p2(r1, y1), 0.0),
        ProfileVertex::new(p2(0.0, y1), 0.0),
    ]))
}

fn sphere_zone_vase() -> Body<f64> {
    bellied(3.0 / 64.0, 0.0, 3.0 / 64.0, 8.0 / 64.0, 4.0 / 64.0, 1.0)
}

fn torus_belly_vase() -> Body<f64> {
    bellied(3.0 / 64.0, 0.0, 3.0 / 64.0, 8.0 / 64.0, 4.0 / 64.0, -1.0)
}

fn bellied_pot() -> Body<f64> {
    let (foot, y_foot, r_neck, y_mouth) = (4.0 / 64.0, 1.0 / 64.0, 3.0 / 64.0, 8.0 / 64.0);
    let c = p2(0.0, y_mouth / 2.0);
    let (dx0, dy0) = (foot - c.x, y_foot - c.y);
    let (dx1, dy1) = (r_neck - c.x, y_mouth - c.y);
    let sweep = (dx0 * dy1 - dy0 * dx1).atan2(dx0 * dx1 + dy0 * dy1);
    revolved(RawLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(foot, 0.0), 0.0),
        ProfileVertex::new(p2(foot, y_foot), (sweep / 4.0).tan()),
        ProfileVertex::new(p2(r_neck, y_mouth), 0.0),
        ProfileVertex::new(p2(0.0, y_mouth), 0.0),
    ]))
}

fn dump(label: &str, body: &Body<f64>) {
    println!(
        "[r2dump] {label}: solids={} shells={} faces={} edges={} vertices={} rows={}",
        body.solids().count(),
        body.shells().count(),
        body.faces().count(),
        body.edges().count(),
        body.vertices().count(),
        body.pcurves().count()
    );
    for row in rows(body) {
        println!("[r2dump] {label}: row {row}");
    }
    for (i, (solid, _)) in body.solids().enumerate() {
        for row in deep_dump(body, solid) {
            println!("[r2dump] {label}: solid {i} {row}");
        }
    }
}

fn shelled(label: &str, body: &Body<f64>, t: f64, open: &[topo::FaceKey]) {
    match topo::shell_open(body, t, open, tol()) {
        Ok(s) => dump(label, &s.body),
        Err(e) => println!("[r2dump] {label}: shell_open Err {e}"),
    }
}

fn moves_of(body: &Body<f64>, solid: SolidKey, d: f64) -> Vec<topo::ChartMove<f64>> {
    charts_of(body, solid)
        .into_iter()
        .map(|faces| topo::ChartMove { faces, distance: d })
        .collect()
}

#[test]
fn shell10_r2_dump_corpus() {
    let y = Vec3::new(0.0, 1.0, 0.0);
    let t = 0.5 / 64.0;
    for (name, body) in [
        ("drum", drum()),
        ("cone frustum", cone_frustum()),
        ("bellied pot", bellied_pot()),
        ("sphere-zone vase", sphere_zone_vase()),
        ("torus-belly vase", torus_belly_vase()),
    ] {
        dump(&format!("{name} operand"), &body);
        let shell = body.shells().next().unwrap().0;
        let top = cap(&body, shell, y, 8.0 / 64.0);
        shelled(&format!("{name} sealed"), &body, t, &[]);
        shelled(&format!("{name} opened top"), &body, t, &top);
    }

    // The doors, directly, naming one solid of several.
    let mut pair = beside(&vessel(1.0, 2.0), &boxy(2.0, 3.0, 4.0), 10.0);
    topo::mint_pcurves(&mut pair, tol()).unwrap();
    let solids: Vec<SolidKey> = pair.solids().map(|(k, _)| k).collect();
    let mut work = pair.clone();
    topo::offset_charts_together(&mut work, &moves_of(&pair, solids[0], -0.05), band(), tol())
        .unwrap();
    dump("axial door, vessel of vessel+box", &work);
    let mut work = pair.clone();
    topo::offset_planes_together(&mut work, &moves_of(&pair, solids[1], -0.05), band(), tol())
        .unwrap();
    dump("planar door, box of vessel+box", &work);

    let mut four = beside(&boxy(2.0, 3.0, 4.0), &vessel(1.0, 2.0), 10.0);
    four = beside(&four, &tube(0.6, 1.0, 2.0), 20.0);
    four = beside(&four, &boxy(2.0, 2.0, 2.0), 30.0);
    topo::mint_pcurves(&mut four, tol()).unwrap();
    let solids: Vec<SolidKey> = four.solids().map(|(k, _)| k).collect();
    let mut work = four.clone();
    topo::offset_charts_together(&mut work, &moves_of(&four, solids[2], -0.05), band(), tol())
        .unwrap();
    dump("axial door, tube of four", &work);
    let mut work = four.clone();
    topo::offset_planes_together(&mut work, &moves_of(&four, solids[3], -0.05), band(), tol())
        .unwrap();
    dump("planar door, second box of four", &work);
    shelled("four sealed", &four, 0.05, &[]);
}
