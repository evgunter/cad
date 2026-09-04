//! SHELL-1 R1 bit-identity dump (probe branch only). Writes a
//! bit-faithful dump of every fixture body to
//! `$SHELL1_BITDUMP_DIR/<name>.txt`; run at the merge base (with the
//! `shelled` helper re-spelled for the old door) and at the head, then
//! `diff -r`. Unarmed when the variable is unset.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use std::fmt::Write as _;

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, FaceKey, LoopBoundary, ShellError};

// HEAD spelling. At the merge base this helper reads
// `fn shelled(r: Result<Body<f64>, ShellError<f64>>) -> Body<f64> { r.unwrap() }`.
fn shelled(r: Result<topo::Shelled<f64>, ShellError<f64>>) -> Body<f64> {
    r.unwrap().body
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// tan(pi/8): the 3-4-5 belly arc, centred on the axis.
const BULGE: f64 = 0.414_213_562_373_095_1;

fn dump_dir() -> Option<std::path::PathBuf> {
    std::env::var_os("SHELL1_BITDUMP_DIR").map(Into::into)
}

fn polygon(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::new(pts.iter().map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0)).collect())
}

fn bulged(pts: &[(f64, f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::new(pts.iter().map(|&(x, y, b)| ProfileVertex::new(p2(x, y), b)).collect())
}

fn revolved_loop(lp: ProfileLoop<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()).unwrap();
    revolve(
        &profile,
        RevolveAxis { origin: p2(0.0, 0.0), dir: Vec2::new(0.0, 1.0) },
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn extruded(loops: Vec<ProfileLoop<f64>>, h: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), loops).validate(Tol::witness()).unwrap();
    extrude(&profile, Extrusion::Distance(h), Tol::witness()).unwrap().body
}

fn plane_chart_at_y(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

fn plane_chart_at_z(body: &Body<f64>, z: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.z - z).abs() < 1e-12 && normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .collect()
}

/// Bit-faithful dump in key iteration order (the shellfix1_bitdump shape).
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
        let p = body.get_vertex(k).and_then(|v| body.get_point(v.point)).unwrap();
        let _ = writeln!(s, "v {k:?} {:x} {:x} {:x}", p.x.to_bits(), p.y.to_bits(), p.z.to_bits());
    }
    for (k, e) in body.edges() {
        let geomstr = body
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .map(|g| format!("{:?} params {:?}", g.carrier(), g.params()))
            .unwrap_or_else(|| "null".into());
        let _ = writeln!(s, "e {k:?} {geomstr}");
    }
    for (k, f) in body.faces() {
        let surf = body.get_surface(f.surface).map(|x| format!("{x:?}")).unwrap_or_else(|| "?".into());
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
                    let h = body.get_half_edge(he).unwrap();
                    let p = body.get_vertex(h.start).and_then(|vv| body.get_point(vv.point)).unwrap();
                    format!("{:?}:{:?}({:x},{:x},{:x})", he, h.edge, p.x.to_bits(), p.y.to_bits(), p.z.to_bits())
                })
                .collect();
            let _ = writeln!(s, "  loop {lk:?} {}", pts.join(" "));
        }
    }
    for (k, sh) in body.shells() {
        let _ = writeln!(s, "s {k:?} faces={:?}", sh.faces);
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
fn shell1_r1_bitdump_corpus() {
    if dump_dir().is_none() {
        println!("[shell1_r1_bitdump] SHELL1_BITDUMP_DIR unset; clean skip");
        return;
    }
    let tol = Tol::witness();
    let boxy = extruded(vec![polygon(&[(0.0, 0.0), (2.0, 0.0), (2.0, 3.0), (0.0, 3.0)])], 4.0);
    let top = plane_chart_at_z(&boxy, 4.0);
    let bottom = plane_chart_at_z(&boxy, 0.0);
    let both: Vec<FaceKey> = top.iter().chain(&bottom).copied().collect();
    write_dump("box_sealed", &shelled(topo::shell(&boxy, 0.25, 1e-6, tol)));
    write_dump("box_cup", &shelled(topo::shell_open(&boxy, 0.25, &top, 1e-6, tol)));
    write_dump("box_tube", &shelled(topo::shell_open(&boxy, 0.25, &both, 1e-6, tol)));

    let vessel = revolved_loop(polygon(&[(0.0, 0.0), (0.5, 0.0), (0.5, 0.4), (0.0, 0.4)]));
    let top = plane_chart_at_y(&vessel, 0.4);
    let bottom = plane_chart_at_y(&vessel, 0.0);
    let both: Vec<FaceKey> = top.iter().chain(&bottom).copied().collect();
    write_dump("vessel_sealed", &shelled(topo::shell(&vessel, 0.05, 1e-6, tol)));
    write_dump("vessel_cup", &shelled(topo::shell_open(&vessel, 0.05, &top, 1e-6, tol)));
    write_dump("vessel_both", &shelled(topo::shell_open(&vessel, 0.05, &both, 1e-6, tol)));

    let tube = revolved_loop(polygon(&[(0.30, 0.0), (0.50, 0.0), (0.50, 0.40), (0.30, 0.40)]));
    let top = plane_chart_at_y(&tube, 0.40);
    write_dump("tube_sealed", &shelled(topo::shell(&tube, 0.05, 1e-6, tol)));
    write_dump("tube_cup", &shelled(topo::shell_open(&tube, 0.05, &top, 1e-6, tol)));

    let slab = extruded(
        vec![
            polygon(&[(0.0, 0.0), (1.0, 0.0), (1.0, 0.8), (0.0, 0.8)]),
            polygon(&[(0.35, 0.25), (0.65, 0.25), (0.65, 0.55), (0.35, 0.55)]),
        ],
        0.3,
    );
    let top = plane_chart_at_z(&slab, 0.3);
    write_dump("slab_cup", &shelled(topo::shell_open(&slab, 0.04, &top, 1e-6, tol)));

    let belly = revolved_loop(bulged(&[(0.0, 0.0, 0.0), (4.0 / 64.0, 0.0, 0.0), (4.0 / 64.0, 1.0 / 64.0, BULGE), (3.0 / 64.0, 8.0 / 64.0, 0.0), (0.0, 8.0 / 64.0, 0.0)]));
    let top = plane_chart_at_y(&belly, 8.0 / 64.0);
    write_dump("belly_sealed", &shelled(topo::shell(&belly, 1.0 / 128.0, 1e-6, tol)));
    write_dump("belly_cup", &shelled(topo::shell_open(&belly, 1.0 / 128.0, &top, 1e-6, tol)));

    let vase = revolved_loop(polygon(&[
        (0.0, 0.0), (0.21, 0.0), (0.21, 0.07), (0.34, 0.07), (0.34, 0.19), (0.11, 0.19), (0.11, 0.31), (0.0, 0.31),
    ]));
    let bottom = plane_chart_at_y(&vase, 0.0);
    write_dump("vase_bottom", &shelled(topo::shell_open(&vase, 0.02, &bottom, 1e-6, tol)));
}
