//! SHELLFIX PR-1 R1 bit-identity dump (probe branch only). Builds the
//! fixtures the PR claims bit-identical — the sealed box, the box cup,
//! the box tube, the sealed vessel and the sealed tube (#1048's
//! acceptance shapes plus the sealed revolves) — and writes a
//! bit-faithful dump to `$SHELLFIX_BITDUMP_DIR/<name>.txt`. Run at the
//! merge base and at the head, then `diff`. Unarmed when the variable
//! is unset (explicit clean skip), the `bitdump.rs` precedent.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use std::fmt::Write as _;

use geom_core::{Band, Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, FaceKey, LoopBoundary};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn dump_dir() -> Option<std::path::PathBuf> {
    std::env::var_os("SHELLFIX_BITDUMP_DIR").map(Into::into)
}

fn boxy(w: f64, d: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(w, 0.0), 0.0),
        ProfileVertex::new(p2(w, d), 0.0),
        ProfileVertex::new(p2(0.0, d), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
}

fn vessel(r: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(r, 0.0), 0.0),
        ProfileVertex::new(p2(r, h), 0.0),
        ProfileVertex::new(p2(0.0, h), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn tube(ri: f64, ro: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(ri, 0.0), 0.0),
        ProfileVertex::new(p2(ro, 0.0), 0.0),
        ProfileVertex::new(p2(ro, h), 0.0),
        ProfileVertex::new(p2(ri, h), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn plane_face_at_z(body: &Body<f64>, z: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.z - z).abs() < 1e-9 && normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .unwrap()
}

/// Bit-faithful dump in key iteration order (identical op sequences
/// produce identical key orders) — the `bitdump.rs` shape, re-derived
/// here so this file also compiles at the merge base unmodified.
fn dump(body: &Body<f64>) -> String {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "census V={} E={} F={} L={} S={} R={}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
        body.loops().count(),
        body.shells().count(),
        body.faces().map(|(_, f)| f.rings.len()).sum::<usize>(),
    );
    for (k, _) in body.vertices() {
        let p = body
            .get_vertex(k)
            .and_then(|v| body.get_point(v.point))
            .unwrap();
        let _ = writeln!(s, "v {k:?} {} {} {}", p.x, p.y, p.z);
    }
    for (k, e) in body.edges() {
        let geomstr = body
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .map(|g| format!("{:?}", g.carrier()))
            .unwrap_or_else(|| "null".into());
        let _ = writeln!(s, "e {k:?} {geomstr}");
    }
    for (k, f) in body.faces() {
        let surf = body
            .get_surface(f.surface)
            .map(|x| format!("{x:?}"))
            .unwrap_or_else(|| "?".into());
        let _ = writeln!(s, "f {k:?} sense={} rings={} {surf}", f.sense, f.rings.len());
        for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
            let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
                let _ = writeln!(s, "  loop {lk:?} empty");
                continue;
            };
            let cyc = body.loop_cycle(first).unwrap();
            let pts: Vec<String> = cyc
                .iter()
                .map(|&he| {
                    let v = body.get_half_edge(he).unwrap().start;
                    let p = body
                        .get_vertex(v)
                        .and_then(|vv| body.get_point(vv.point))
                        .unwrap();
                    format!("({},{},{})", p.x, p.y, p.z)
                })
                .collect();
            let _ = writeln!(s, "  loop {lk:?} {}", pts.join(" "));
        }
    }
    let props = topo::mass_properties(body, Tol::witness()).unwrap();
    let _ = writeln!(
        s,
        "props V={:x} A={:x} pad={:x}",
        props.volume.to_bits(),
        props.surface_area.to_bits(),
        props.volume_pad.to_bits()
    );
    s
}

fn write_dump(name: &str, body: &Body<f64>) {
    let Some(dir) = dump_dir() else { return };
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join(format!("{name}.txt")), dump(body)).unwrap();
}

#[test]
fn shellfix1_bitdump_corpus() {
    if dump_dir().is_none() {
        println!("[shellfix1_bitdump] SHELLFIX_BITDUMP_DIR unset; clean skip");
        return;
    }
    let (w, d, h, t) = (2.0, 3.0, 4.0, 0.25);
    let body = boxy(w, d, h);
    write_dump(
        "sealed_box",
        &topo::shell(&body, t, 1e-6, band(), Tol::witness()).unwrap(),
    );
    let top = plane_face_at_z(&body, h);
    let bottom = plane_face_at_z(&body, 0.0);
    write_dump(
        "box_cup",
        &topo::shell_open(&body, t, &[top], 1e-6, band(), Tol::witness()).unwrap(),
    );
    write_dump(
        "box_tube",
        &topo::shell_open(&body, t, &[top, bottom], 1e-6, band(), Tol::witness()).unwrap(),
    );
    write_dump(
        "sealed_vessel",
        &topo::shell(&vessel(1.0, 2.0), 0.2, 1e-6, band(), Tol::witness()).unwrap(),
    );
    write_dump(
        "sealed_tube",
        &topo::shell(&tube(0.6, 1.0, 2.0), 0.1, 1e-6, band(), Tol::witness()).unwrap(),
    );
}
