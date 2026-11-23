//! Delta-2 review probes, PR 7124 at 8e9af35fce3: false records of kinds
//! no lane reads, whose cells no separating axis parts.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop};
use sweep::ExtrudeSide;
use sweep::test_support::{finished, sketch_at};
use sweep::{Extrusion, extrude};
use topo::{
    Assembled, AtRestBody, Body, BooleanResult, Built, Cell, Cited, Cites, Contact,
    ContactRecords, FaceKey,
};

fn tol() -> Tol {
    Tol::witness()
}

fn rod(r: f64, z0: f64, h: f64) -> AtRestBody<f64> {
    let at = |t: f64| Point2::new(r * t.cos(), r * t.sin());
    let circle: ProfileLoop<f64> =
        bulge_loop(vec![(at(0.0), 1.0), (at(core::f64::consts::PI), 1.0)]);
    let profile = Profile::new(sketch_at(z0), vec![circle]).validate(tol()).unwrap();
    let body: Body<f64> = extrude(
        &profile,
        Extrusion::Distance { depth: h, side: ExtrudeSide::Along },
        tol(),
    )
    .unwrap()
    .body
    .into_body();
    finished("rod", body, tol())
}

fn block(x: (f64, f64), y: (f64, f64), z0: f64, h: f64) -> AtRestBody<f64> {
    let c = |x, y| (Point2::new(x, y), 0.0);
    let body: Body<f64> = sweep::test_support::prism_at(
        vec![c(x.0, y.0), c(x.1, y.0), c(x.1, y.1), c(x.0, y.1)],
        z0,
        h,
        tol(),
    );
    finished("block", body, tol())
}

fn kind(body: &Body<f64>, f: FaceKey) -> geom::SurfaceKind {
    body.face_surface(f).map(geom::Surface::kind).unwrap()
}

fn say(what: &str, r: &Result<AtRestBody<f64>, topo::NotFinished>) {
    match r {
        Ok(done) => eprintln!("PROBE {what}: HELD, bar {:?}", done.bar()),
        Err(e) => eprintln!(
            "PROBE {what}: refused {}",
            format!("{:?}", e.as_slice()).chars().take(300).collect::<String>()
        ),
    }
}

#[test]
fn delta2_nested_cells_no_axis_parts() {
    let BooleanResult::Body(tube) = topo::subtract(&rod(1.0, 0.0, 1.0), &rod(0.5, -0.5, 2.0), tol())
        .unwrap()
    else {
        panic!("empty tube")
    };
    let tube = tube.body;
    for (name, inner) in [
        ("block in the bore", block((-0.2, 0.2), (-0.2, 0.2), 0.4, 0.2)),
        ("thin rod in the bore", rod(0.2, 0.4, 0.2)),
    ] {
        let mut j = Assembled::<f64>::new();
        let kt = j.graft_finished(&tube, None, tol()).unwrap();
        let ki = j.graft_finished(&inner, None, tol()).unwrap();
        let done = topo::finish_assembled(j, tol()).unwrap();
        eprintln!("PROBE {name}: join bar {:?}, findings {:?}", done.body.bar(), done.findings);
        let body = topo::FinishedBody::into_body(done.body);
        let cyls: Vec<FaceKey> = tube
            .faces()
            .filter_map(|(f, _)| kt.face(f))
            .filter(|&f| kind(&body, f) == geom::SurfaceKind::Cylinder)
            .collect();
        let inner_faces: Vec<FaceKey> = inner.faces().filter_map(|(f, _)| ki.face(f)).collect();
        for &c in &cyls {
            for &f in &inner_faces {
                let mut records = ContactRecords::default();
                records.extend([Cited::new(
                    Contact::new(&body, Cell::Face(c), Cell::Face(f)).unwrap(),
                    Cites::decided(0),
                )]);
                let got = topo::finish(Built::of(body.clone()).recording(records), tol());
                say(&format!("{name}: tube cyl {c:?} × inner {:?} {f:?}", kind(&body, f)), &got);
            }
        }
        // A vertex of the inner body against each tube cylinder.
        let v = inner.vertices().next().map(|(v, _)| ki.vertex(v).unwrap()).unwrap();
        for &c in &cyls {
            let mut records = ContactRecords::default();
            records.extend([Cited::new(
                Contact::new(&body, Cell::Vertex(v), Cell::Face(c)).unwrap(),
                Cites::decided(0),
            )]);
            say(&format!("{name}: inner vertex × tube cyl {c:?}"), &topo::finish(Built::of(body.clone()).recording(records), tol()));
        }
    }
}
