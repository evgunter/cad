//! Review probes (R2, PRs 4809/4820): the boolean's minted-pair door.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use geom_core::Tol;
use topo::{AtRestBody, Backing, BooleanBody, BooleanResult, Cell, ContactKind, union, validate_pseudomanifold};

fn tol() -> Tol {
    Tol::witness()
}

fn box_of(x: (f64, f64), y: (f64, f64), z: (f64, f64), what: &str) -> AtRestBody<f64> {
    common::finished(what, common::brick(x, y, z, tol()), tol())
}

fn union_of(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> BooleanBody<f64> {
    match union(a, b, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    }
}

fn carried_op(op: topo::BooleanOp, a: &BooleanBody<f64>, b: &AtRestBody<f64>)
    -> Result<BooleanResult<f64>, topo::BooleanError> {
    let inputs = topo::BooleanInputs { records_a: a.contacts.carried(), ..topo::BooleanInputs::default() };
    topo::boolean_op_with(op, &a.body, b, &inputs, topo::SweepStrategy::Realized, tol())
}

fn kiss() -> (BooleanBody<f64>, u32) {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (0.0, 1.0), "b");
    let base = union_of(&a, &b);
    let ee = base.contacts.kinds().position(|k| matches!(k, ContactKind::EdgeEdge(..))).expect("ee");
    (base, u32::try_from(ee).unwrap())
}

fn report(what: &str, base_ee: u32, got: Result<BooleanResult<f64>, topo::BooleanError>) {
    match got {
        Ok(BooleanResult::Body(out)) => {
            eprintln!("R2PROBE {what}: Ok; records:");
            for r in out.contacts.rows() {
                let cites: Vec<Backing> = r.cites.iter().collect();
                eprintln!("R2PROBE   {:?} cites {:?}", r.cells().map(|c| match c { Cell::Vertex(_) => "V", Cell::Edge(_) => "E", Cell::Face(_) => "F" }), cites);
            }
            for c in &out.coincidences { eprintln!("R2PROBE   coincidence {c:?}"); }
            let at_rest = validate_pseudomanifold(&out.body, &out.contacts, tol());
            eprintln!("R2PROBE   at rest: {at_rest:?}");
            let minted = out.contacts.rows().iter().any(|r| {
                matches!(r.cells(), [Cell::Vertex(_), Cell::Vertex(_)])
                    && r.cites.iter().any(|c| c == Backing::Carried { input: 0, record: base_ee })
                    && r.cites.iter().any(|c| matches!(c, Backing::Decided(_)))
            });
            eprintln!("R2PROBE   minted pair cites ee + a decision: {minted}");
        }
        other => eprintln!("R2PROBE {what}: {other:?}"),
    }
}

/// One tool face cuts both kissing edges at z = 1/2.
#[test]
fn r2_subtract_one_face_cuts_both_kissing_edges() {
    let (base, ee) = kiss();
    let tool = box_of((-1.0, 3.0), (-1.0, 3.0), (-1.0, 0.5), "tool");
    report("subtract one face", ee, carried_op(topo::BooleanOp::Subtract, &base, &tool));
}

/// A notch across the kiss: each edge cut twice, middle removed.
#[test]
fn r2_subtract_notch_across_the_kiss() {
    let (base, ee) = kiss();
    let tool = box_of((0.5, 1.5), (0.5, 1.5), (0.25, 0.5), "notch");
    report("notch across the kiss", ee, carried_op(topo::BooleanOp::Subtract, &base, &tool));
}

/// Kissing edges overlapping over z in [1, 2] (a: z in [0, 2], b: z in
/// [1, 3]); a notch across the kiss removes the whole overlap, so the
/// edge-edge record lands nowhere and both edges are divided with their
/// remainder removed: consumed.
#[test]
fn r2_notch_removes_the_whole_overlap() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 2.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (1.0, 3.0), "b");
    let base = union_of(&a, &b);
    eprintln!("R2PROBE overlap base kinds: {:?}", base.contacts.kinds().collect::<Vec<_>>().len());
    let ee = base.contacts.kinds().position(|k| matches!(k, ContactKind::EdgeEdge(..)));
    eprintln!("R2PROBE overlap base ee at {ee:?}");
    let tool = box_of((0.5, 1.5), (0.5, 1.5), (0.9, 2.1), "notch");
    report("notch removes the overlap", ee.map_or(99, |e| u32::try_from(e).unwrap()), carried_op(topo::BooleanOp::Subtract, &base, &tool));
    let tool = box_of((0.5, 1.5), (0.5, 1.5), (1.2, 1.8), "notch");
    report("notch inside the overlap", ee.map_or(99, |e| u32::try_from(e).unwrap()), carried_op(topo::BooleanOp::Subtract, &base, &tool));
}
