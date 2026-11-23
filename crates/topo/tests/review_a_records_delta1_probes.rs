//! Delta review probes on PR 7124 at bbb6955172: which records a
//! finished body holds unconfirmed, and which bar it names.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use geom_core::{Point3, Tol, Vec3};
use topo::{
    Assembled, AtRestBody, Body, BooleanResult, Built, Cell, Cited, Cites, Contact,
    ContactRecords, FinishedBody as _, finish, finish_assembled, union,
};

fn tol() -> Tol {
    Tol::witness()
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    common::finished("brick", common::brick::<f64>(x, y, z, tol()), tol())
}

fn kiss() -> AtRestBody<f64> {
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let BooleanResult::Body(k) = union(&a, &b, tol()).unwrap() else {
        panic!("empty");
    };
    k.body
}

fn faces_on(body: &Body<f64>, axis: usize, at: f64) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(f, _)| {
            matches!(
                body.face_surface(*f),
                Some(topo::Surface::Plane { frame })
                    if [frame.axis().x, frame.axis().y, frame.axis().z][axis].abs() == 1.0
                        && [frame.origin().x, frame.origin().y, frame.origin().z][axis] == at
            )
        })
        .map(|(f, _)| f)
        .collect()
}

fn with(r: &ContactRecords, body: &Body<f64>, x: Cell, y: Cell) -> ContactRecords {
    let mut r = r.clone();
    r.extend([Cited::new(Contact::new(body, x, y).unwrap(), Cites::decided(0))]);
    r
}

fn say(what: &str, got: &Result<AtRestBody<f64>, topo::NotFinished>) {
    match got {
        Ok(f) => eprintln!(
            "PROBE {what}: FINISHED, {} records, bar {:?}",
            f.records().rows().len(),
            f.bar()
        ),
        Err(e) => eprintln!(
            "PROBE {what}: refused {}",
            format!("{e:?}").chars().take(400).collect::<String>()
        ),
    }
}

/// B, A: records a bare finish is handed.
#[test]
fn delta_records_handed_to_a_bare_finish() {
    let k = kiss();
    let body = Body::clone(&k);
    let held = k.records().clone();
    let x1 = faces_on(&body, 0, 1.0);
    let z1 = faces_on(&body, 2, 1.0);
    // The first pass's M1 rows: faces on one plane, opposed, meeting at a point.
    let sa = body.solid_of_face(x1[0]);
    let other = |fs: &[topo::FaceKey]| *fs.iter().find(|f| body.solid_of_face(**f) != sa).unwrap();
    say(
        "x=1 faces of the two cubes",
        &finish(Built::of(body.clone()).recording(with(&held, &body, Cell::Face(x1[0]), Cell::Face(other(&x1)))), tol()),
    );
    let za = *z1.iter().find(|f| body.solid_of_face(**f) == sa).unwrap();
    say(
        "z=1 faces of the two cubes",
        &finish(Built::of(body.clone()).recording(with(&held, &body, Cell::Face(za), Cell::Face(other(&z1)))), tol()),
    );
    // A record naming a face the body does not hold: a key from a bigger arena.
    let mut big = Body::clone(&brick((5.0, 6.0), (5.0, 6.0), (5.0, 6.0)));
    for i in 0..4 {
        let c = brick((10.0 + 2.0 * f64::from(i), 11.0 + 2.0 * f64::from(i)), (0.0, 1.0), (0.0, 1.0));
        topo::graft_disjoint_all_keyed(&mut big, &c).unwrap();
    }
    let missing = big
        .faces()
        .map(|(f, _)| f)
        .find(|f| body.face_surface(*f).is_none())
        .expect("a key the kiss does not hold");
    say(
        "a face the body does not hold, against x=1",
        &finish(Built::of(body.clone()).recording(with(&held, &body, Cell::Face(missing), Cell::Face(x1[0]))), tol()),
    );
    let vmissing = big
        .vertices()
        .map(|(v, _)| v)
        .find(|v| topo::readback::vertex_point(&body, *v).is_err())
        .expect("a vertex key the kiss does not hold");
    say(
        "a vertex the body does not hold, on x=1",
        &finish(Built::of(body.clone()).recording(with(&held, &body, Cell::Vertex(vmissing), Cell::Face(x1[0]))), tol()),
    );
    // J: a finished body's clone handed bare.
    say("Built::of(clone of the kiss)", &finish(Built::of(body.clone()), tol()));
    // The held record twice.
    let mut dup = held.clone();
    dup.extend(held.rows().iter().cloned());
    say("the held record twice", &finish(Built::of(body.clone()).recording(dup), tol()));
}

/// Bar at a join whose census did not decide a meeting between arrivals.
#[test]
fn delta_bar_at_a_join_with_an_undecided_meeting() {
    let band = geom_core::Band::linear(tol()).unwrap();
    let off = 0.5 * (band.zero() + band.escalate());
    let block = |x: (f64, f64), y: (f64, f64), z: (f64, f64)| {
        common::finished(
            "block",
            common::prism_z::<f64>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1, tol()).body,
            tol(),
        )
    };
    for (name, top) in [
        ("corner, in band", ((1.0, 3.0), (1.0, 3.0), (off, 1.0 + off))),
        ("apart", ((5.0, 6.0), (5.0, 6.0), (0.0, 1.0))),
        ("resting", ((0.5, 1.5), (0.5, 1.5), (1.0, 2.0))),
    ] {
        let mut j = Assembled::<f64>::new();
        j.graft_finished(&block((0.0, 2.0), (0.0, 2.0), (0.0, 1.0)), None, tol()).unwrap();
        j.graft_finished(&block(top.0, top.1, top.2), None, tol()).unwrap();
        match finish_assembled(j, tol()) {
            Ok(c) => eprintln!(
                "PROBE join {name}: bar {:?}, {} records, findings {}",
                c.body.bar(),
                c.body.records().rows().len(),
                format!("{:?}", c.findings).chars().take(300).collect::<String>()
            ),
            Err(e) => eprintln!("PROBE join {name}: refused {e:?}"),
        }
    }
    // A join of one arrival holding nothing, and of_solids of the kiss.
    let mut one = Assembled::<f64>::new();
    one.graft_finished(&brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)), None, tol()).unwrap();
    let c = finish_assembled(one, tol()).unwrap();
    eprintln!("PROBE join of one bare arrival: bar {:?}", c.body.bar());
    let c = finish_assembled(Assembled::of_solids(Body::clone(&kiss())), tol()).unwrap();
    eprintln!(
        "PROBE of_solids(kiss): bar {:?}, {} records, findings {:?}",
        c.body.bar(),
        c.body.records().rows().len(),
        c.findings
    );
}

/// A mirror of the kiss: records and bar.
#[test]
fn delta_a_mirror_keeps_records_and_bar() {
    let k = kiss();
    let r = topo::Reflection::across(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), tol()).unwrap();
    let m = k.moved_by(&topo::Isometry::Improper(r), tol()).unwrap();
    eprintln!(
        "PROBE mirror: {} -> {} records, bar {:?} -> {:?}",
        k.records().rows().len(),
        m.records().rows().len(),
        k.bar(),
        m.bar()
    );
    let leaf = topo::LeafBody::moved_by(&k, &topo::Isometry::Improper(r), tol());
    eprintln!(
        "PROBE mirror as leaf: {:?}",
        leaf.map(|l| (l.records().rows().len(), l.bar())).map_err(|e| format!("{e:?}").chars().take(200).collect::<String>())
    );
    // Re-finish the mirror image from scratch with its records: does the gate confirm?
    let again = finish(Built::of(Body::clone(&m)).recording(m.records().clone()), tol());
    say("mirror image re-finished with its records", &again);
}
