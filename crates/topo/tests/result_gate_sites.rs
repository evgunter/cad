//! **Every site that builds a Boolean result gates it at tier 3.** The
//! door builds a [`BooleanBody`] at three sites — the seamed join, the
//! two-operand fallback (`Assembly`, `Voided`) and the single-operand
//! fallback (`OperandA`, `OperandB`) — and each passes its result
//! through `ops::gate` before returning it. One pose per result kind,
//! each asserting the kind it reaches and that the body it returns
//! carries tier 3's `Validated` verdict, so a site that skipped the
//! gate (sorting pieces only) goes red here.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use topo::{
    AtRestBody, AtRestOutcome, BooleanBody, BooleanOp, BooleanResult, BooleanResultKind,
    boolean_op_with,
};

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    let tol = Tol::witness();
    common::finished("a brick", common::brick::<f64>(x, y, z, tol), tol)
}

fn body_of(what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>) -> BooleanBody<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => b,
        other => panic!("{what}: a body, got {:?}", other.map(|_| "empty")),
    }
}

#[test]
fn every_result_kind_carries_tier_3s_verdict() {
    let tol = Tol::witness();
    let unit = (0.0, 1.0);
    let big = brick((0.0, 4.0), (0.0, 4.0), (0.0, 4.0));
    let inner = brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let corner = brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let overlap = brick((1.0, 3.0), (1.0, 3.0), (1.0, 3.0));
    let near = brick(unit, unit, unit);
    let far = brick((5.0, 6.0), unit, unit);
    let none = topo::BooleanDeclarations::none();
    let run = |op, a: &AtRestBody<f64>, b: &AtRestBody<f64>| {
        boolean_op_with(op, a, b, &none, topo::SweepStrategy::Realized, tol)
    };
    for (what, op, a, b, kind) in [
        (
            "corner ∪",
            BooleanOp::Union,
            &corner,
            &overlap,
            BooleanResultKind::Seamed,
        ),
        (
            "disjoint ∪",
            BooleanOp::Union,
            &near,
            &far,
            BooleanResultKind::Assembly,
        ),
        (
            "nested ∖",
            BooleanOp::Subtract,
            &big,
            &inner,
            BooleanResultKind::Voided,
        ),
        (
            "nested ∪",
            BooleanOp::Union,
            &big,
            &inner,
            BooleanResultKind::OperandA,
        ),
        (
            "nested ∩",
            BooleanOp::Intersect,
            &big,
            &inner,
            BooleanResultKind::OperandB,
        ),
    ] {
        let out = body_of(what, run(op, a, b));
        assert_eq!(out.kind, kind, "{what}: the site this pose reaches");
        assert_eq!(
            out.body.outcome(),
            AtRestOutcome::Validated,
            "{what}: the result carries tier 3's verdict"
        );
    }
}
