//! PROBE (base pin): prints every fixture's verdict verbatim. Replaced
//! by the real rows once the base messages are recorded.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::{Band, Point3, Tol};
use topo::{
    Body, ContactRecords, FaceKey, SolidContainment, VfContact, VoidContainment, VoidEvidence,
    insert_void, validate_pseudomanifold,
};

const L_PROFILE: [(f64, f64); 6] = [
    (0.0, 0.0),
    (3.0, 0.0),
    (3.0, 1.0),
    (1.0, 1.0),
    (1.0, 3.0),
    (0.0, 3.0),
];

fn assembly(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut out = a.clone();
    topo::graft_disjoint(&mut out, b, Tol::witness()).unwrap();
    out
}

/// Every vertex of `body` whose point satisfies `pick`.
fn vertices_where(body: &Body<f64>, pick: impl Fn(Point3<f64>) -> bool) -> Vec<topo::VertexKey> {
    body.vertices()
        .filter(|(_, v)| pick(*body.get_point(v.point).unwrap()))
        .map(|(k, _)| k)
        .collect()
}

/// The L-bracket (issue 750): the container over the L profile, the
/// part resting flat on the inner wall `x = 1`; `dx` shifts the part.
fn lbracket(declared: bool, dx: f64) -> (Body<f64>, ContactRecords) {
    let l = common::prism_z::<f64>(&L_PROFILE, 0.0, 1.0);
    let wall: FaceKey = l.side_faces[3];
    let part = common::brick::<f64>((1.0 + dx, 2.0 + dx), (1.2, 2.0), (0.2, 0.8));
    let body = assembly(&l.body, &part);
    let mut records = ContactRecords::default();
    if declared {
        for v in vertices_where(&body, |p| {
            (p.x - (1.0 + dx)).abs() < 1e-12 && (1.1..2.1).contains(&p.y)
        }) {
            records.b_on_a.push(VfContact {
                vertex: v,
                face: wall,
            });
        }
        assert_eq!(records.b_on_a.len(), 4);
    }
    (body, records)
}

fn cube(side: f64, dx: f64, dy: f64, dz: f64) -> Body<f64> {
    common::mapped_cube(|x, y, z| Point3::new(side * x + dx, side * y + dy, side * z + dz))
}

/// h14's embedded fixture: 1 m in 4 m, flush at z = 0, four v-on-f.
fn embedded() -> (Body<f64>, ContactRecords) {
    let body = assembly(&cube(4.0, 0.0, 0.0, 0.0), &cube(1.0, 1.0, 1.0, 0.0));
    let big_bottom = body
        .faces()
        .map(|(f, _)| f)
        .find(|&f| {
            let face = body.get_face(f).unwrap();
            let l = body.get_loop(face.outer).unwrap();
            let topo::LoopBoundary::Cycle { first } = l.boundary else {
                return false;
            };
            body.loop_cycle(first).unwrap().into_iter().all(|he| {
                let v = body.get_half_edge(he).unwrap().start;
                let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                p.z.abs() < 1e-12 && !(0.9..2.1).contains(&p.x)
            })
        })
        .unwrap();
    let mut records = ContactRecords::default();
    for v in vertices_where(&body, |p| {
        p.z.abs() < 1e-12 && (0.9..2.1).contains(&p.x) && (0.9..2.1).contains(&p.y)
    }) {
        records.b_on_a.push(VfContact {
            vertex: v,
            face: big_bottom,
        });
    }
    assert_eq!(records.b_on_a.len(), 4);
    (body, records)
}

/// A hollow 3 m cube (void 1..2) with a part floating in the void.
fn cavity() -> Body<f64> {
    let mut dst = common::brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 3.0));
    let hole = common::brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let (solid, _) = dst.solids().next().unwrap();
    let evidence = VoidEvidence {
        shells: hole
            .shells()
            .map(|(s, _)| (s, VoidContainment::Probed(SolidContainment::In)))
            .collect(),
    };
    insert_void(&mut dst, solid, hole, &evidence, Tol::witness()).unwrap();
    let part = common::brick::<f64>((1.2, 1.8), (1.2, 1.8), (1.2, 1.8));
    assembly(&dst, &part)
}

/// A U-shaped block (planar pocket open on +y) with a part in the pocket.
fn pocket() -> Body<f64> {
    let u = common::prism_z::<f64>(
        &[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 3.0),
            (2.0, 3.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ],
        0.0,
        1.0,
    );
    let part = common::brick::<f64>((1.2, 1.8), (1.5, 2.5), (0.2, 0.8));
    assembly(&u.body, &part)
}

/// A slab through a cube: every vertex of the part on the cube's faces.
fn all_on_boundary() -> Body<f64> {
    let container = common::brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let part = common::brick::<f64>((0.0, 2.0), (0.5, 1.5), (0.5, 1.5));
    assembly(&container, &part)
}

fn report(name: &str, body: &Body<f64>, records: &ContactRecords) {
    println!("=== {name}");
    match validate_pseudomanifold(body, records, Tol::witness()) {
        Ok(()) => println!("Ok(())"),
        Err(errs) => {
            println!("{} errors", errs.len());
            for e in &errs {
                println!("DEBUG: {e:?}");
                println!("DISPLAY: {e}");
            }
        }
    }
}

#[test]
fn probe_base_verdicts() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    println!(
        "eps={} zero={} escalate={}",
        tol.eps(),
        band.zero(),
        band.escalate()
    );
    let (b, r) = lbracket(true, 0.0);
    report("lbracket declared", &b, &r);
    let (b, r) = lbracket(false, 0.0);
    report("lbracket undeclared", &b, &r);
    let (b, r) = embedded();
    report("embedded declared", &b, &r);
    report("cavity", &cavity(), &ContactRecords::default());
    report("pocket", &pocket(), &ContactRecords::default());
    report(
        "all_on_boundary",
        &all_on_boundary(),
        &ContactRecords::default(),
    );
    let delta = (band.zero() * band.escalate()).sqrt();
    let (b, r) = lbracket(false, delta);
    report(&format!("band edge delta={delta}"), &b, &r);
}
