//! Probe: a vertex through a declared seat.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{Pose, posed_boxes};
use geom_core::Tol;
use std::collections::BTreeMap;
use topo::{AtRestBody, BooleanInputs, BooleanOp, ContactRecords, SolidKey, ValidationError};

fn t() -> Tol {
    Tol::witness()
}

fn census(body: &topo::Body<f64>) -> impl FnMut(&ContactRecords) -> Vec<ValidationError> + '_ {
    |records| {
        topo::validate_pseudomanifold(body, records, t())
            .err()
            .unwrap_or_default()
    }
}

/// The bolt standing on the plate, one body of two solids, and its
/// seat recorded.
fn seat() -> (AtRestBody<f64>, ContactRecords) {
    let body = posed_boxes(
        "a bolt on a plate",
        &[
            [(-0.2, 0.2), (-0.2, 0.2), (0.2, 1.2)],
            [(-1.0, 1.0), (-1.0, 1.0), (0.0, 0.2)],
        ],
        &Pose::rest(),
        t(),
    );
    let groups: BTreeMap<SolidKey, u32> = body.solids().map(|(s, _)| s).zip(0..).collect();
    let recorded = topo::record_at_rest(
        &body,
        &ContactRecords::default(),
        census(&body),
        &|s| groups.get(&s).copied(),
        0,
    );
    assert!(recorded.findings.is_empty(), "{:?}", recorded.findings);
    (body, recorded.records)
}

fn block(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    posed_boxes("a block", &[[x, y, z]], &Pose::rest(), t())
}

#[test]
fn probe() {
    let (seat, records) = seat();
    eprintln!("records: {:?}", records.kinds().collect::<Vec<_>>());
    let post = block((-0.1, 0.1), (-0.1, 0.1), (0.1, 0.4));
    let bead = block((0.15, 0.35), (-0.1, 0.1), (0.1, 0.4));
    let side = block((0.15, 0.35), (-0.1, 0.1), (0.6, 0.8));
    let ledge = block((0.2, 0.5), (-0.5, 0.5), (0.1, 0.4));
    let pin = block((-0.1, 0.1), (-0.1, 0.1), (-0.5, 0.5));
    let run = |op, a: &AtRestBody<f64>, b: &AtRestBody<f64>, inputs: &BooleanInputs| {
        topo::boolean_op_with(op, a, b, inputs, topo::SweepStrategy::Realized, t())
    };
    let as_b = BooleanInputs {
        records_b: records.carried(),
        ..BooleanInputs::default()
    };
    let as_a = BooleanInputs {
        records_a: records.carried(),
        ..BooleanInputs::default()
    };
    let show = |what: &str, r: Result<topo::BooleanResult<f64>, topo::BooleanError>| match r {
        Ok(topo::BooleanResult::Empty) => eprintln!("{what}: Empty"),
        Ok(topo::BooleanResult::Body(b)) => eprintln!(
            "{what}: body, {} solids, records {:?}, volume {:?}, faces {}, tier3 {:?}, tier3' {:?}",
            b.body.solids().count(),
            b.contacts.kinds().collect::<Vec<_>>(),
            topo::mass_properties(&b.body, t(), topo::test_support::pinned_resolution(t()))
                .map(|m| m.met().map(|m| m.volume)),
            b.body.faces().count(),
            topo::validate_geometric(&b.body, t()).map_err(|e| e.len()),
            topo::validate_pseudomanifold(&b.body, &b.contacts, t()),
        ),
        Err(e) => eprintln!("{what}: {e:?}"),
    };
    let ops = [
        ("post − seat", BooleanOp::Subtract, &post, &seat, &as_b),
        ("seat − post", BooleanOp::Subtract, &seat, &post, &as_a),
        ("post ∩ seat", BooleanOp::Intersect, &post, &seat, &as_b),
        ("seat ∩ post", BooleanOp::Intersect, &seat, &post, &as_a),
        ("seat ∪ post", BooleanOp::Union, &seat, &post, &as_a),
        ("seat ∪ side", BooleanOp::Union, &seat, &side, &as_a),
        ("seat ∪ bead", BooleanOp::Union, &seat, &bead, &as_a),
        ("bead ∪ seat", BooleanOp::Union, &bead, &seat, &as_b),
        ("bead − seat", BooleanOp::Subtract, &bead, &seat, &as_b),
        ("seat − bead", BooleanOp::Subtract, &seat, &bead, &as_a),
        ("bead ∩ seat", BooleanOp::Intersect, &bead, &seat, &as_b),
        ("seat ∪ ledge", BooleanOp::Union, &seat, &ledge, &as_a),
        ("seat − ledge", BooleanOp::Subtract, &seat, &ledge, &as_a),
        ("seat − pin", BooleanOp::Subtract, &seat, &pin, &as_a),
        ("pin − seat", BooleanOp::Subtract, &pin, &seat, &as_b),
        (
            "seat − ledge bare",
            BooleanOp::Subtract,
            &seat,
            &ledge,
            &BooleanInputs::default(),
        ),
        ("ledge − seat", BooleanOp::Subtract, &ledge, &seat, &as_b),
        ("ledge ∩ seat", BooleanOp::Intersect, &ledge, &seat, &as_b),
        (
            "post − seat bare",
            BooleanOp::Subtract,
            &post,
            &seat,
            &BooleanInputs::default(),
        ),
    ];
    for (what, op, a, b, inputs) in ops {
        show(what, run(op, a, b, inputs));
    }
}
