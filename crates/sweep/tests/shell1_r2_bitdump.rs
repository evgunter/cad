//! SHELL-1 R2 bit-identity dump (probe branch only). The rim-surgery
//! shapes `shellfix1_bitdump` does not carry: the revolved cup, the
//! annular cap, a one-square-holed slab and an oblique prism. Writes to
//! `$SHELL1_R2_DUMP_DIR/<name>.txt`; unarmed when unset. Run at the
//! merge base (with `.body` stripped) and at the head, then `diff`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use std::fmt::Write as _;

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, FaceKey, LoopBoundary};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn dump_dir() -> Option<std::path::PathBuf> {
    std::env::var_os("SHELL1_R2_DUMP_DIR").map(Into::into)
}

fn polygon(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    )
}

fn extruded(loops: Vec<ProfileLoop<f64>>, h: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
}

fn revolved(pts: &[(f64, f64)]) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![polygon(pts)])
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

fn chart_at_y(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

fn chart_at_z(body: &Body<f64>, z: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.z - z).abs() < 1e-12
                        && normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .collect()
}

/// `shellfix1_bitdump`'s dump, verbatim in shape so the two corpora
/// read alike.
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
        let _ = writeln!(
            s,
            "f {k:?} sense={} rings={} {surf}",
            f.sense,
            f.rings.len()
        );
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
fn shell1_r2_bitdump_corpus() {
    if dump_dir().is_none() {
        println!("[shell1_r2_bitdump] SHELL1_R2_DUMP_DIR unset; clean skip");
        return;
    }
    let tol = Tol::witness();

    let vessel = revolved(&[(0.0, 0.0), (0.5, 0.0), (0.5, 0.4), (0.0, 0.4)]);
    let cap = chart_at_y(&vessel, 0.4);
    write_dump(
        "revolved_cup",
        &topo::shell_open(&vessel, 0.05, &cap, 1e-6, tol)
            .unwrap()
            .body,
    );

    let tube = revolved(&[(0.30, 0.0), (0.50, 0.0), (0.50, 0.40), (0.30, 0.40)]);
    let annulus = chart_at_y(&tube, 0.40);
    write_dump(
        "annular_cup",
        &topo::shell_open(&tube, 0.05, &annulus, 1e-6, tol)
            .unwrap()
            .body,
    );

    let slab = extruded(
        vec![
            polygon(&[(0.0, 0.0), (1.0, 0.0), (1.0, 0.8), (0.0, 0.8)]),
            polygon(&[(0.35, 0.25), (0.65, 0.25), (0.65, 0.55), (0.35, 0.55)]),
        ],
        0.3,
    );
    let slab_top = chart_at_z(&slab, 0.3);
    write_dump(
        "holed_slab_cup",
        &topo::shell_open(&slab, 0.04, &slab_top, 1e-6, tol)
            .unwrap()
            .body,
    );

    let oblique = extruded(
        vec![polygon(&[(0.0, 0.0), (1.0, 0.0), (0.7, 0.6), (0.1, 0.6)])],
        0.5,
    );
    write_dump(
        "oblique_prism_sealed",
        &topo::shell(&oblique, 0.05, 1e-6, tol).unwrap().body,
    );
    let obl_top = chart_at_z(&oblique, 0.5);
    write_dump(
        "oblique_prism_cup",
        &topo::shell_open(&oblique, 0.05, &obl_top, 1e-6, tol)
            .unwrap()
            .body,
    );
}
