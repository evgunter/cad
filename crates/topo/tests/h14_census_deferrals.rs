//! **H14 acceptance — the census's record-keyed deferrals.**
//!
//! Placeholder header; filled in with the unit.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom_core::Point3;
use topo::{Body, ContactRecords, VfContact, validate_pseudomanifold};

fn cube_at(sx: f64, dx: f64, dy: f64, dz: f64) -> Body<f64> {
    common::mapped_cube(|x, y, z| Point3::new(sx * x + dx, sx * y + dy, sx * z + dz))
}

fn assembly(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut out = a.clone();
    topo::graft_disjoint(&mut out, b).unwrap();
    out
}

/// Every boundary vertex of `f`, as (key, point).
fn face_verts(body: &Body<f64>, f: topo::FaceKey) -> Vec<(topo::VertexKey, Point3<f64>)> {
    let face = body.get_face(f).unwrap();
    let mut out = Vec::new();
    for &lk in core::iter::once(&face.outer).chain(&face.rings) {
        let l = body.get_loop(lk).unwrap();
        let topo::LoopBoundary::Cycle { first } = l.boundary else {
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            let v = body.get_half_edge(he).unwrap().start;
            let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            out.push((v, p));
        }
    }
    out
}

/// The **embedded-instance** fixture: a unit cube sitting entirely
/// inside a 4 m cube, flush at `z = 0`, with its four bottom corners
/// declared v-on-f on the big cube's bottom face. Every record is
/// geometrically TRUE, so the confirm pass has nothing to say.
fn embedded() -> (Body<f64>, ContactRecords) {
    let outer = cube_at(4.0, 0.0, 0.0, 0.0);
    let inner = cube_at(1.0, 1.0, 1.0, 0.0);
    let body = assembly(&outer, &inner);
    // The big cube's bottom face: four boundary vertices, all at
    // z = 0, spanning the full 4 m.
    let big_bottom = body
        .faces()
        .map(|(f, _)| f)
        .find(|&f| {
            let vs = face_verts(&body, f);
            vs.len() == 4
                && vs.iter().all(|(_, p)| p.z.abs() < 1e-12)
                && vs.iter().any(|(_, p)| p.x > 3.9)
        })
        .expect("the 4 m cube has a bottom face");
    let mut records = ContactRecords::default();
    for (v, p) in body
        .faces()
        .map(|(f, _)| f)
        .flat_map(|f| face_verts(&body, f))
    {
        if p.z.abs() < 1e-12 && p.x > 0.9 && p.x < 2.1 && p.y > 0.9 && p.y < 2.1 {
            let rec = VfContact {
                vertex: v,
                face: big_bottom,
            };
            if !records.b_on_a.contains(&rec) {
                records.b_on_a.push(rec);
            }
        }
    }
    assert_eq!(records.b_on_a.len(), 4, "four declared corners");
    (body, records)
}

#[test]
fn probe_embedded_instance_today() {
    let (body, records) = embedded();
    let r = validate_pseudomanifold(&body, &records);
    println!("PROBE embedded: {r:?}");
}

#[test]
fn probe_embedded_instance_undeclared() {
    let (body, _) = embedded();
    let r = validate_pseudomanifold(&body, &ContactRecords::default());
    println!("PROBE embedded undeclared: {r:?}");
}
