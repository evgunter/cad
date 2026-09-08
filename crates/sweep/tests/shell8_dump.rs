//! **SHELL-8's differential instrument: the HOLLOW-and-OPENED corpus.**
//! Prints a deterministic dump of every fixture whose sealed arm leaves
//! several solids and whose rim surgery then runs in one of them — the
//! shapes whose lift door is read per solid rather than over the whole
//! result — plus the sealed hollow fixtures they are built from. The
//! same file compiled at the merge base and at the head is diffed line
//! by line; it asserts nothing beyond "the fixture builds", the diff is
//! the verdict. Kept free of every symbol the unit adds, so it compiles
//! on both trees.
//!
//! Run with `--nocapture`, grep `[dump8]`, sort and diff across trees.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::{Body, FaceKey, VertexKey};

use crate::verbs_shell::{hollow_box, outer_and_void, two_void_box, vessel};

fn tol() -> Tol {
    Tol::witness()
}

fn face_of_he(body: &Body<f64>, he: topo::HalfEdgeKey) -> FaceKey {
    let lp = body.get_half_edge(he).unwrap().parent_loop;
    body.get_loop(lp).unwrap().face
}

fn faces_at(body: &Body<f64>, v: VertexKey) -> Vec<FaceKey> {
    let Some(em) = body.get_vertex(v).unwrap().emanating else {
        return Vec::new();
    };
    let mut out: Vec<FaceKey> = body
        .vertex_orbit(em)
        .unwrap()
        .into_iter()
        .map(|he| face_of_he(body, he))
        .collect();
    out.sort();
    out.dedup();
    out
}

fn dump(label: &str, body: &Body<f64>) {
    println!(
        "[dump8] {label}: solids={} shells={} faces={} edges={} vertices={}",
        body.solids().count(),
        body.shells().count(),
        body.faces().count(),
        body.edges().count(),
        body.vertices().count()
    );
    for (k, f) in body.faces() {
        println!(
            "[dump8] {label}: face {k:?} shell={:?} sense={} rings={} surface_key={:?} surface={:?}",
            f.shell,
            f.sense,
            f.rings.len(),
            f.surface,
            body.get_surface(f.surface)
        );
    }
    for (k, e) in body.edges() {
        let (fa, fb) = (face_of_he(body, e.he_plus), face_of_he(body, e.he_minus));
        let start = body.get_half_edge(e.he_plus).unwrap().start;
        let end = body.half_edge_end(e.he_plus).unwrap();
        let c = body
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .unwrap();
        println!(
            "[dump8] {label}: edge {k:?} faces=({fa:?},{fb:?}) verts=({start:?},{end:?}) carrier={:?} params={:?} description={:?}",
            c.carrier(),
            c.params(),
            c.description()
        );
    }
    for (k, v) in body.vertices() {
        let faces = faces_at(body, k);
        let mut surfaces: Vec<_> = faces
            .iter()
            .map(|f| body.get_face(*f).unwrap().surface)
            .collect();
        surfaces.sort();
        surfaces.dedup();
        println!(
            "[dump8] {label}: vertex {k:?} point={:?} faces={faces:?} distinct_surfaces={surfaces:?}",
            body.get_point(v.point).unwrap()
        );
    }
    match topo::mass_properties(body, tol()) {
        Ok(p) => println!(
            "[dump8] {label}: volume={:?} pad={:?} area={:?}",
            p.volume, p.volume_pad, p.surface_area
        ),
        Err(e) => println!("[dump8] {label}: props Err {e:?}"),
    }
    println!(
        "[dump8] {label}: tier3={:?}",
        topo::validate_geometric(body, tol())
    );
}

/// The whole CHART of `shell` whose plane is normal to `axis` (a unit
/// world direction) and sits at `value` along it — every face wearing
/// it, since a full revolve splits a cap into two half-discs and the
/// rim surgery lifts a chart as one.
fn cap(
    body: &Body<f64>,
    shell: topo::ShellKey,
    axis: geom_core::Vec3<f64>,
    value: f64,
) -> Vec<FaceKey> {
    for &face in &body.get_shell(shell).unwrap().faces {
        let f = body.get_face(face).unwrap();
        let Some(geom::Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) else {
            continue;
        };
        if normal.cross(axis).norm() > 1e-9 {
            continue;
        }
        if (geom_core::Vec3::new(origin.x, origin.y, origin.z).dot(axis) - value).abs() < 1e-9 {
            let chart = f.surface;
            return body
                .faces()
                .filter(|(_, g)| g.surface == chart)
                .map(|(k, _)| k)
                .collect();
        }
    }
    panic!("no cap of {shell:?} normal to {axis:?} at {value}")
}

fn opened(label: &str, body: &Body<f64>, t: f64, faces: &[FaceKey]) {
    match topo::shell_open(body, t, faces, tol()) {
        Ok(s) => dump(label, &s.body),
        Err(e) => println!("[dump8] {label}: shell_open Err {e}"),
    }
}

/// Every hollow operand this crate builds, sealed and then opened on
/// each of its walls in turn.
#[test]
fn shell8_dump_hollow_and_opened_corpus() {
    let z = geom_core::Vec3::new(0.0, 0.0, 1.0);
    let y = geom_core::Vec3::new(0.0, 1.0, 0.0);

    // ---- The planar hollow box: `PlanesTogether` on the way in, the
    // per-chart door on the lift. ----
    let hollow = hollow_box();
    dump("hollow box operand", &hollow);
    let (outer, void) = outer_and_void(&hollow);
    opened("hollow box sealed", &hollow, 0.05, &[]);
    opened(
        "hollow box, outer lid open",
        &hollow,
        0.05,
        &cap(&hollow, outer, z, 4.0),
    );
    opened(
        "hollow box, void ceiling open",
        &hollow,
        0.05,
        &cap(&hollow, void, z, 3.75),
    );

    // ---- Two voids in one solid: three thin solids, then a
    // designation on each wall in turn. ----
    let (two, _) = two_void_box();
    dump("two-void box operand", &two);
    opened("two-void box sealed", &two, 0.15, &[]);
    let shells: Vec<topo::ShellKey> = two.shells().map(|(k, _)| k).collect();
    for (i, &shell) in shells.iter().enumerate() {
        let at = if i == 0 { 4.0 } else { 3.0 };
        opened(
            &format!("two-void box, shell {i} top open"),
            &two,
            0.15,
            &cap(&two, shell, z, at),
        );
    }

    // ---- The hollow vessel: `ChartsTogether` on the way in, and a
    // lift whose door was read over the whole result body and is now
    // read over the designated face's own solid. ----
    let hollow_vessel = topo::shell(&vessel(1.0, 2.0), 0.1, tol())
        .expect("the vessel hollows")
        .body;
    dump("hollow vessel operand", &hollow_vessel);
    let (outer, void) = outer_and_void(&hollow_vessel);
    opened("hollow vessel sealed", &hollow_vessel, 0.02, &[]);
    opened(
        "hollow vessel, outer lid open",
        &hollow_vessel,
        0.02,
        &cap(&hollow_vessel, outer, y, 2.0),
    );
    opened(
        "hollow vessel, void ceiling open",
        &hollow_vessel,
        0.02,
        &cap(&hollow_vessel, void, y, 1.9),
    );
}
