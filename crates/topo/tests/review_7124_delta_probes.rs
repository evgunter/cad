//! Delta re-read probes for PR 7124 at `bbb6955172` (topo side).
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use geom_core::Tol;
use topo::{
    Assembled, AtRestBody, AtRestPolicy, BooleanResult, Built, Cell, Cited, Cites, Contact,
    ContactRecords, FinishedBody, finish, finish_assembled, union, validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

fn cube(x: f64, y: f64, z: f64) -> AtRestBody<f64> {
    finish(
        Built::of(common::brick::<f64>(
            (x, x + 1.0),
            (y, y + 1.0),
            (z, z + 1.0),
            tol(),
        )),
        tol(),
    )
    .unwrap()
}

fn names(findings: &[topo::ValidationError]) -> Vec<String> {
    let mut out: Vec<String> = findings
        .iter()
        .map(|f| {
            let s = format!("{f:?}");
            s.split([' ', '{', '(']).next().unwrap().to_owned()
        })
        .collect();
    out.sort();
    out.dedup();
    out
}

/// D-2: the bar a join names against what its census decided between
/// its arrivals.
#[test]
fn probe_d2_a_joins_bar_against_its_census() {
    for (what, dx) in [
        ("far", 3.0),
        ("kissing faces", 1.0),
        ("faces 1e-9 apart", 1.0 + 1e-9),
        ("overlapping by half", 0.5),
    ] {
        let mut join = Assembled::<f64>::new();
        join.graft_finished(&cube(0.0, 0.0, 0.0), None, tol())
            .unwrap();
        join.graft_finished(&cube(dx, 0.0, 0.0), None, tol())
            .unwrap();
        match finish_assembled(join, tol()) {
            Ok(done) => eprintln!(
                "D-2 {what}: bar {:?}, {} record(s), findings {:?}",
                done.body.bar(),
                done.body.records().rows().len(),
                names(&done.findings)
            ),
            Err(e) => eprintln!("D-2 {what}: refused {:?}", names(e.as_slice())),
        }
    }
    // One arrival grafted twice where it stands.
    let a = cube(0.0, 0.0, 0.0);
    let mut join = Assembled::<f64>::new();
    join.graft_finished(&a, None, tol()).unwrap();
    join.graft_finished(&a, None, tol()).unwrap();
    match finish_assembled(join, tol()) {
        Ok(done) => eprintln!(
            "D-2 one arrival twice: bar {:?}, {} record(s), findings {:?}",
            done.body.bar(),
            done.body.records().rows().len(),
            names(&done.findings)
        ),
        Err(e) => eprintln!("D-2 one arrival twice: refused {:?}", names(e.as_slice())),
    }
}

/// D-3: a finished body's clone handed back to the gate bare, and a
/// box scalar's leaf holding a record no gate read, joined.
#[test]
fn probe_d3_ways_round() {
    let BooleanResult::Body(kiss) =
        union(&cube(0.0, 0.0, 0.0), &cube(1.0, 1.0, 1.0), tol()).unwrap()
    else {
        panic!("empty");
    };
    let bare = finish(Built::of(topo::Body::clone(&kiss.body)), tol()).unwrap();
    eprintln!(
        "D-3 a kiss's clone finished bare: {} record(s) (it held {}), bar {:?}; tier 3' with none: {:?}",
        bare.records().rows().len(),
        kiss.body.records().rows().len(),
        bare.bar(),
        validate_pseudomanifold(&bare, &ContactRecords::default(), tol()).map_err(|e| names(&e))
    );
    // A replay at the interval scalar holding a record with no touch behind it.
    type I = geom_core::interval::Interval;
    let mut body = common::mapped_cube(common::identity_map::<I>, tol());
    common::cube_into(
        &mut body,
        |x, y, z| common::identity_map::<I>(x + 3.0, y, z),
        tol(),
    );
    let faces: Vec<_> = body.faces().map(|(f, _)| f).collect();
    let mut records = ContactRecords::default();
    records.extend([Cited::new(
        Contact::new(
            &body,
            Cell::Face(faces[0]),
            Cell::Face(faces[faces.len() - 1]),
        )
        .unwrap(),
        Cites::decided(0),
    )]);
    let leaf = <I as AtRestPolicy>::leaf(Built::of(body).recording(records), tol()).unwrap();
    eprintln!(
        "D-3 interval replay holds {} unread record(s)",
        leaf.records().rows().len()
    );
    let mut join = Assembled::<I>::new();
    join.graft_leaf(&leaf, None, tol()).unwrap();
    match finish_assembled(join, tol()) {
        Ok(done) => eprintln!(
            "D-3 joined: Ok, {} record(s), bar {:?}",
            done.body.records().rows().len(),
            done.body.bar()
        ),
        Err(e) => eprintln!("D-3 joined: refused {:?}", names(e.as_slice())),
    }
    let taken = <I as AtRestPolicy>::take(&std::sync::Arc::new(leaf), tol());
    eprintln!(
        "D-3 taken: {:?}",
        taken
            .map(|b| FinishedBody::records(&*b).rows().len())
            .map_err(|e| names(e.as_slice()))
    );
}

/// A join names tier 3′ only where its census decided every meeting
/// between two arrivals (`Bar::Tier3Prime`'s own sentence). RED on
/// `bbb6955172`: two cubes whose facing faces stand in the band apart
/// (its midpoint, at whichever ε the suite runs), each an arrival; the
/// census hands back `CensusUndecidable` on that pair and the body was
/// named `Tier3Prime`.
#[test]
fn red_a_joins_bar_is_not_tier_3_prime_where_its_census_left_a_meeting_undecided() {
    let band = geom_core::Band::linear(tol()).unwrap();
    let gap = 0.5 * (band.zero() + band.escalate());
    let mut join = Assembled::<f64>::new();
    join.graft_finished(&cube(0.0, 0.0, 0.0), None, tol())
        .unwrap();
    join.graft_finished(&cube(1.0 + gap, 0.0, 0.0), None, tol())
        .unwrap();
    let done = finish_assembled(join, tol()).unwrap();
    assert!(
        done.findings
            .iter()
            .any(|f| matches!(f, topo::ValidationError::CensusUndecidable { .. })),
        "the census leaves the pair undecided: {:?}",
        names(&done.findings)
    );
    assert_eq!(
        done.body.bar(),
        topo::Bar::Tier3,
        "named tier 3' beside {:?}",
        names(&done.findings)
    );
}
