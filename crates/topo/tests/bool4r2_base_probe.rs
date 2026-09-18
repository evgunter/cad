//! Red-first reproduction: the head suite's fixtures with their verdicts
//! printed verbatim, written against the merge base's API only so the
//! same file runs at 592c951dc (registered there in a detached worktree)
//! and at the reviewed head. The `BASE` prefix names the file, not the
//! tree it ran on.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::{Band, Point3, Tol};
use topo::{
    Body, ContactRecords, SolidContainment, ValidationError, VfContact, VoidContainment,
    VoidEvidence, insert_void, validate_pseudomanifold,
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

fn vertices_where(body: &Body<f64>, pick: impl Fn(Point3<f64>) -> bool) -> Vec<topo::VertexKey> {
    body.vertices()
        .filter(|(_, v)| pick(*body.get_point(v.point).unwrap()))
        .map(|(k, _)| k)
        .collect()
}

fn point_of(body: &Body<f64>, v: topo::VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

fn lbracket(declared: bool, dx: f64) -> (Body<f64>, ContactRecords) {
    let l = common::prism_z::<f64>(&L_PROFILE, 0.0, 1.0, Tol::witness());
    let wall = l.side_faces[3];
    let part = common::brick::<f64>((1.0 + dx, 2.0 + dx), (1.2, 2.0), (0.2, 0.8), Tol::witness());
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
    common::mapped_cube(
        |x, y, z| Point3::new(side * x + dx, side * y + dy, side * z + dz),
        Tol::witness(),
    )
}

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
                let p = point_of(&body, body.get_half_edge(he).unwrap().start);
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

fn cavity() -> Body<f64> {
    let mut dst = common::brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 3.0), Tol::witness());
    let hole = common::brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), Tol::witness());
    let (solid, _) = dst.solids().next().unwrap();
    let evidence = VoidEvidence {
        shells: hole
            .shells()
            .map(|(s, _)| (s, VoidContainment::Probed(SolidContainment::In)))
            .collect(),
    };
    insert_void(&mut dst, solid, hole, &evidence, Tol::witness()).unwrap();
    let part = common::brick::<f64>((1.2, 1.8), (1.2, 1.8), (1.2, 1.8), Tol::witness());
    assembly(&dst, &part)
}

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
        Tol::witness(),
    );
    let part = common::brick::<f64>((1.2, 1.8), (1.5, 2.5), (0.2, 0.8), Tol::witness());
    assembly(&u.body, &part)
}

fn all_on_boundary() -> Body<f64> {
    let container = common::brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), Tol::witness());
    let part = common::brick::<f64>((0.0, 2.0), (0.5, 1.5), (0.5, 1.5), Tol::witness());
    assembly(&container, &part)
}

fn straddle() -> Body<f64> {
    let l = common::prism_z::<f64>(&L_PROFILE, 0.0, 1.0, Tol::witness());
    let part = common::brick::<f64>((0.5, 1.5), (1.0, 3.0), (0.0, 1.0), Tol::witness());
    assembly(&l.body, &part)
}

fn report(name: &str, verdict: Result<(), Vec<ValidationError>>) {
    match verdict {
        Ok(()) => println!("BASE {name}: Ok(())"),
        Err(errs) => {
            println!("BASE {name}: {} errors", errs.len());
            for e in errs {
                println!("BASE {name}:   {e:?}");
            }
        }
    }
}

#[test]
fn base_verdicts() {
    let tol = Tol::witness();
    let (b, r) = lbracket(true, 0.0);
    report("l_declared", validate_pseudomanifold(&b, &r, tol));
    let (b, _) = lbracket(false, 0.0);
    report(
        "l_undeclared",
        validate_pseudomanifold(&b, &ContactRecords::default(), tol),
    );
    let (b, r) = embedded();
    report("embedded_declared", validate_pseudomanifold(&b, &r, tol));
    report(
        "embedded_undeclared",
        validate_pseudomanifold(&b, &ContactRecords::default(), tol),
    );
    report(
        "cavity",
        validate_pseudomanifold(&cavity(), &ContactRecords::default(), tol),
    );
    report(
        "pocket",
        validate_pseudomanifold(&pocket(), &ContactRecords::default(), tol),
    );
    report(
        "all_on_boundary",
        validate_pseudomanifold(&all_on_boundary(), &ContactRecords::default(), tol),
    );
    let band = Band::linear(tol).unwrap();
    let delta = (band.zero() * band.escalate()).sqrt();
    let (b, _) = lbracket(false, delta);
    report(
        "band_edge",
        validate_pseudomanifold(&b, &ContactRecords::default(), tol),
    );
    let (b, _) = lbracket(false, 10.0 * band.escalate());
    report(
        "past_band",
        validate_pseudomanifold(&b, &ContactRecords::default(), tol),
    );
    report(
        "straddle",
        validate_pseudomanifold(&straddle(), &ContactRecords::default(), tol),
    );
}

#[test]
fn base_vertex_straddle() {
    let tol = Tol::witness();
    let l = common::prism_z::<f64>(&L_PROFILE, 0.0, 1.0, Tol::witness());
    let wall = l.side_faces[3];
    let part = common::mapped_cube(
        |u, v, w| {
            Point3::new(
                1.4 - 0.2 * (u + v + w),
                1.8 - 0.3 * v + 0.1 * u - 0.05 * w,
                0.3 + 0.3 * w - 0.1 * u + 0.05 * v,
            )
        },
        Tol::witness(),
    );
    let body = assembly(&l.body, &part);
    let errors =
        validate_pseudomanifold(&body, &ContactRecords::default(), tol).expect_err("refuses");
    report("vertex_straddle_undeclared", Err(errors.clone()));
    let mut records = ContactRecords::default();
    for e in &errors {
        if let ValidationError::UndeclaredContact {
            contact: topo::CensusContact::VertexOnFace { vertex, face },
            ..
        } = e
        {
            assert_eq!(*face, wall);
            records.b_on_a.push(VfContact {
                vertex: *vertex,
                face: *face,
            });
        }
    }
    report(
        "vertex_straddle_declared",
        validate_pseudomanifold(&body, &records, tol),
    );
}
