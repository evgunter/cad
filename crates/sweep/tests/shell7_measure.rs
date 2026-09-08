//! SHELL-7 measurement instrument: the full-period torus's faces, edges,
//! carriers and every vertex's incident surfaces, printed; and the
//! shell verb's answer on it. Asserts nothing beyond "it builds".

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use sweep::{TubeWindow, tube_along_arc, tube_along_arc_hollow};
use topo::{Body, FaceKey, VertexKey};

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
        "[measure] {label}: solids={} shells={} faces={} edges={} vertices={}",
        body.solids().count(),
        body.shells().count(),
        body.faces().count(),
        body.edges().count(),
        body.vertices().count()
    );
    for (k, f) in body.faces() {
        println!(
            "[measure] {label}: face {k:?} shell={:?} sense={} rings={} surface_key={:?} surface={:?}",
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
            "[measure] {label}: edge {k:?} faces=({fa:?},{fb:?}) verts=({start:?},{end:?}) carrier={:?} params={:?} description={:?}",
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
            "[measure] {label}: vertex {k:?} point={:?} faces={faces:?} distinct_surfaces={surfaces:?}",
            body.get_point(v.point).unwrap()
        );
    }
}

#[test]
fn shell7_measure_the_full_torus() {
    let tol = Tol::witness();
    let (big_r, r, w, t) = (2.0, 0.5, 0.125, 0.05);
    let solid = tube_along_arc::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        big_r,
        TubeWindow::Full,
        r,
        tol,
    )
    .expect("builds")
    .body;
    let hollow = tube_along_arc_hollow::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        big_r,
        TubeWindow::Full,
        r,
        w,
        tol,
    )
    .expect("builds")
    .body;
    dump("solid", &solid);
    dump("hollow", &hollow);
    for (what, body) in [("solid", &solid), ("hollow", &hollow)] {
        match topo::shell(body, t, tol) {
            Ok(s) => {
                println!(
                    "[measure] {what}: shell OK shells={}",
                    s.body.shells().count()
                );
                dump(&format!("{what}-shelled"), &s.body);
            }
            Err(e) => println!("[measure] {what}: shell Err {e}"),
        }
    }
}
