//! Delta re-read probes for PR 7124 at `bbb6955172`.
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
/// handed to the gate as the body's one record: what holds it.
fn held_false_records(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> usize {
    let (body, [ca, cb]) = joined(a, b);
    let mut held = 0;
    let mut tally: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for &x in &ca {
        for &y in &cb {
            let Ok(contact) = Contact::new(&body, x, y) else {
                continue;
            };
            let mut records = ContactRecords::default();
            records.extend([Cited::new(contact, Cites::decided(0))]);
            let kind = |c: Cell| match c {
                Cell::Face(f) => format!("F:{:?}", body.face_surface(f).map(geom::Surface::kind)),
                Cell::Edge(_) => "E".to_owned(),
                Cell::Vertex(_) => "V".to_owned(),
            };
            let key = match topo::finish(Built::of(body.clone()).recording(records), tol()) {
                Ok(done) => {
                    held += 1;
                    format!("HELD {:?} {} x {}", done.bar(), kind(x), kind(y))
                }
                Err(e) => format!("refused {:?}", names(e.as_slice())),
            };
            *tally.entry(key).or_default() += 1;
        }
    }
    for (k, n) in &tally {
        if k.starts_with("HELD") {
            eprintln!("D-1 {what}: {n} x {k}");
        }
    }
    let refused: usize = tally
        .iter()
        .filter(|(k, _)| !k.starts_with("HELD"))
        .map(|(_, n)| n)
        .sum();
    eprintln!("D-1 {what}: {held} false record(s) held, {refused} refused");
    held
}

/// D-1: no record of two cells of bodies that touch nowhere is held.
#[test]
fn red_no_record_between_bodies_that_touch_nowhere_is_held() {
    let block = finished(
        "block",
        brick::<f64>((10.0, 11.0), (0.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );
    let a = rod(0.0, 0.0, 0.5, 0.0, 1.0);
    let far_rod = rod(20.0, 0.0, 0.5, 0.0, 1.0);
    let held = held_false_records("rod and a block 9.5 away", &a, &block)
        + held_false_records("two rods 19 apart", &a, &far_rod);
    assert_eq!(
        held, 0,
        "the gate held a record of two cells that touch nowhere"
    );
}

/// D-4: bodies that touch, joined; what the join records and names,
/// and whether the plain gate holds the same records over the same
/// body (as a later door, or a leaf's move, would ask).
#[test]
fn probe_d4_true_touches_through_the_join_and_then_the_plain_gate() {
    let plate = || {
        finished(
            "plate",
            brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (-1.0, 0.0), tol()),
            tol(),
        )
    };
    let side = || {
        finished(
            "side",
            brick::<f64>((0.5, 1.5), (-1.0, 1.0), (0.0, 1.0), tol()),
            tol(),
        )
    };
    let scenes: Vec<(&str, AtRestBody<f64>, AtRestBody<f64>)> = vec![
        (
            "a rod standing on a plate",
            rod(0.0, 0.0, 0.5, 0.0, 1.0),
            plate(),
        ),
        (
            "two rods end to end",
            rod(0.0, 0.0, 0.5, 0.0, 1.0),
            rod(0.0, 0.0, 0.5, 1.0, 1.0),
        ),
        (
            "a rod on a wider rod",
            rod(0.0, 0.0, 0.5, 0.0, 1.0),
            rod(0.0, 0.0, 0.8, 1.0, 1.0),
        ),
        (
            "two rods side by side",
            rod(0.0, 0.0, 0.5, 0.0, 1.0),
            rod(1.0, 0.0, 0.5, 0.0, 1.0),
        ),
        (
            "a rod against a block's side",
            rod(0.0, 0.0, 0.5, 0.0, 1.0),
            side(),
        ),
    ];
    for (what, a, b) in scenes {
        let mut join = Assembled::<f64>::new();
        join.graft_finished(&a, None, tol()).unwrap();
        join.graft_finished(&b, None, tol()).unwrap();
        let done = match topo::finish_assembled(join, tol()) {
            Ok(done) => done,
            Err(e) => {
                eprintln!("D-4 {what}: the join refuses {:?}", names(e.as_slice()));
                continue;
            }
        };
        let records = done.body.records().clone();
        let bar = done.body.bar();
        let again = topo::finish(
            Built::of(topo::FinishedBody::into_body(done.body.clone())).recording(records.clone()),
            tol(),
        );
        let moved = topo::LeafBody::moved(
            &done.body,
            &geom_core::RigidMotion::translation(geom_core::Vec3::new(3.0, 0.0, 0.0)).unwrap(),
            tol(),
        );
        eprintln!(
            "D-4 {what}: join {} record(s), bar {bar:?}, findings {:?}; plain gate over the same: {:?}; moved as a leaf: {:?}",
            records.rows().len(),
            names(&done.findings),
            again
                .as_ref()
                .map(|d| d.bar())
                .map_err(|e| names(e.as_slice())),
            moved
                .as_ref()
                .map(|d| d.bar())
                .map_err(|e| names(e.as_slice())),
        );
        match topo::union(&a, &b, tol()) {
            Ok(topo::BooleanResult::Body(u)) => eprintln!(
                "D-4 {what}: union {} record(s), bar {:?}",
                u.contacts().rows().len(),
                u.body.bar()
            ),
            Ok(_) => eprintln!("D-4 {what}: union empty"),
            Err(e) => eprintln!(
                "D-4 {what}: union refuses: {}",
                format!("{e}").chars().take(200).collect::<String>()
            ),
        }
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
