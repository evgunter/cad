//! Two bodies that touch nowhere hold no record between them, and a
//! join names tier 3 beside a finding its census left (PR 7124's second
//! review: both rows were red on `bbb6955172`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop};
use sweep::ExtrudeSide;
use sweep::test_support::{brick, finished, sketch_at};
use sweep::{Extrusion, extrude};
use topo::{Assembled, AtRestBody, Body, Built, Cell, Cited, Cites, Contact, ContactRecords};

fn tol() -> Tol {
    Tol::witness()
}

fn rod(cx: f64, cy: f64, r: f64, z0: f64, h: f64) -> AtRestBody<f64> {
    let at = |t: f64| Point2::new(cx + r * t.cos(), cy + r * t.sin());
    let circle: ProfileLoop<f64> =
        bulge_loop(vec![(at(0.0), 1.0), (at(core::f64::consts::PI), 1.0)]);
    let profile = Profile::new(sketch_at(z0), vec![circle])
        .validate(tol())
        .unwrap();
    let body: Body<f64> = extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
    .into_body();
    finished("rod", body, tol())
}

fn names(findings: &[topo::ValidationError]) -> Vec<String> {
    findings
        .iter()
        .map(|f| {
            let s = format!("{f:?}");
            s.split([' ', '{', '(']).next().unwrap().to_owned()
        })
        .collect()
}

/// Two finished bodies far apart, joined; the joined body and the cells
/// of each arrival.
fn joined(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> (Body<f64>, [Vec<Cell>; 2]) {
    let mut join = Assembled::<f64>::new();
    let ka = join.graft_finished(a, None, tol()).unwrap();
    let kb = join.graft_finished(b, None, tol()).unwrap();
    let done = topo::finish_assembled(join, tol()).unwrap();
    assert!(done.body.records().rows().is_empty(), "they touch nowhere");
    let body = topo::FinishedBody::into_body(done.body);
    let cells = |src: &AtRestBody<f64>, k: &topo::GraftKeys| {
        let mut out: Vec<Cell> = src
            .faces()
            .filter_map(|(f, _)| k.face(f))
            .map(Cell::Face)
            .collect();
        out.extend(src.edges().filter_map(|(e, _)| k.edge(e)).map(Cell::Edge));
        out.extend(
            src.vertices()
                .filter_map(|(v, _)| k.vertex(v))
                .map(Cell::Vertex),
        );
        out
    };
    (body.clone(), [cells(a, &ka), cells(b, &kb)])
}

/// Every pair of one cell of each of two bodies that touch nowhere,
/// handed to the gate as the body's one record: each one held, as the
/// pair of its cells.
fn held_false_records(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Vec<(Cell, Cell)> {
    let (body, [ca, cb]) = joined(a, b);
    let mut held = Vec::new();
    for &x in &ca {
        for &y in &cb {
            let Ok(contact) = Contact::new(&body, x, y) else {
                continue;
            };
            let mut records = ContactRecords::default();
            records.extend([Cited::new(contact, Cites::decided(0))]);
            if topo::finish(Built::of(body.clone()).recording(records), tol()).is_ok() {
                held.push((x, y));
            }
        }
    }
    held
}

/// No record of two cells of bodies that touch nowhere is held: every
/// pair of one cell from each of a rod and a block 9.5 apart, and of two
/// rods 19 apart, planted as the joined body's one record. Red on a gate
/// that holds what a lane declines: 24 of the 560 were held at tier 3.
#[test]
fn no_record_between_bodies_that_touch_nowhere_is_held() {
    let block = finished(
        "block",
        brick::<f64>((10.0, 11.0), (0.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );
    let a = rod(0.0, 0.0, 0.5, 0.0, 1.0);
    let far_rod = rod(20.0, 0.0, 0.5, 0.0, 1.0);
    for (what, other) in [("a block 9.5 away", &block), ("a rod 19 away", &far_rod)] {
        let held = held_false_records(&a, other);
        assert!(
            held.is_empty(),
            "{what}: the gate held a record of two cells that touch nowhere: {held:?}"
        );
    }
}

/// A join's bar is tier 3 wherever its census left a finding between
/// two arrivals, with no gap involved: four curved joins that touch (a
/// rod standing on a plate, two rods end to end, two rods side by
/// side, a rod against a block's side), whose census hands back
/// unrecorded or undecided meetings. Red on a join that names its bar
/// before its census runs: each was named `Tier3Prime`.
#[test]
fn a_joins_bar_is_tier_3_beside_a_finding_on_curved_arrivals() {
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (-1.0, 0.0), tol()),
        tol(),
    );
    let side = finished(
        "side",
        brick::<f64>((0.5, 1.5), (-1.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );
    let scenes: Vec<(&str, AtRestBody<f64>, AtRestBody<f64>)> = vec![
        (
            "a rod standing on a plate",
            rod(0.0, 0.0, 0.5, 0.0, 1.0),
            plate,
        ),
        (
            "two rods end to end",
            rod(0.0, 0.0, 0.5, 0.0, 1.0),
            rod(0.0, 0.0, 0.5, 1.0, 1.0),
        ),
        (
            "two rods side by side",
            rod(0.0, 0.0, 0.5, 0.0, 1.0),
            rod(1.0, 0.0, 0.5, 0.0, 1.0),
        ),
        (
            "a rod against a block's side",
            rod(0.0, 0.0, 0.5, 0.0, 1.0),
            side,
        ),
    ];
    for (what, a, b) in scenes {
        let mut join = Assembled::<f64>::new();
        join.graft_finished(&a, None, tol()).unwrap();
        join.graft_finished(&b, None, tol()).unwrap();
        let done = topo::finish_assembled(join, tol()).unwrap();
        assert!(
            !done.findings.is_empty(),
            "{what}: the census leaves a finding"
        );
        assert_eq!(
            done.body.bar(),
            topo::Bar::Tier3,
            "{what}: named tier 3' beside {:?}",
            names(&done.findings)
        );
    }
}
