//! Scratch probe.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use std::collections::BTreeMap;
use topo::{Body, ContactRecords, ValidationError};

fn two(
    base: &[(f64, f64)],
    (z0, z1): (f64, f64),
    top: &[(f64, f64)],
    (w0, w1): (f64, f64),
) -> Body<f64> {
    let mut body = common::prism_z::<f64>(base, z0, z1, Tol::witness()).body;
    let t = common::prism_z::<f64>(top, w0, w1, Tol::witness()).body;
    topo::graft_disjoint_all_keyed(&mut body, &t).unwrap();
    body
}

fn dump(label: &str, body: &Body<f64>) {
    let groups: BTreeMap<_, u32> = body.solids().map(|(s, _)| s).zip(0..).collect();
    let recorded = topo::record_at_rest(
        body,
        &ContactRecords::default(),
        |records| {
            topo::validate_pseudomanifold(body, records, Tol::witness())
                .err()
                .unwrap_or_default()
        },
        &|s| groups.get(&s).copied(),
        0,
    );
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let flush =
        <f64 as topo::AtRestPolicy>::flush_at_rest(body, &|s| groups.get(&s).copied(), band);
    eprintln!("== {label}");
    for r in &recorded.rows {
        eprintln!("  row {:?} {:?}", r.relation, r.cells);
    }
    for f in &recorded.findings {
        eprintln!("  finding {f:?}");
    }
    for r in &flush.rows {
        eprintln!("  flush row {:?}", r.cells);
    }
    for f in &flush.findings {
        eprintln!("  flush finding {f:?}");
    }
    let _: Vec<ValidationError> = Vec::new();
}

const SQ: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

#[test]
fn probe() {
    dump("coincident", &two(&SQ, (0.0, 1.0), &SQ, (0.0, 1.0)));
    dump("flush half", &two(&SQ, (0.0, 1.0), &SQ, (0.0, 0.5)));
    let half = [(0.0, 0.0), (0.5, 0.0), (0.5, 1.0), (0.0, 1.0)];
    dump("quarter in corner", &two(&SQ, (0.0, 1.0), &half, (0.0, 0.5)));
    dump("stacked", &two(&SQ, (0.0, 1.0), &SQ, (1.0, 2.0)));
}
