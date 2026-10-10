//! **A joinable vertex refuses at every door's operand gate at a dual**
//! (tier 3's check 11; Ev's PR 4251 ruling). A dual's scalar runs no
//! at-rest gate (`AtRestOutcome::NotRunAtThisScalar`), so a hand-split
//! body becomes an `AtRestBody` there unread, and each door reads it
//! itself (`AtRestBody::gate_unverdicted`). Each door refuses the split
//! cube typed, carrying check 11's finding at the split vertex: the split
//! and the Boolean (topo) and both blend doors (sweep).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Dual64, Point3, Real, Tol, Vec3};
use topo::{
    AtRestPolicy, BooleanDeclarations, BooleanError, BooleanOp, Operand, SplitError,
    SplitReduceError, SweepStrategy, ValidationError,
};

fn f(x: f64) -> Dual64 {
    Dual64::from_f64(x)
}

#[test]
fn a_split_operand_refuses_at_every_door_at_a_dual() {
    let tol = Tol::witness();
    let mut cube = sweep::test_support::cube::<Dual64>(1.0, tol);
    let (edge, curve) = cube.edges().next().map(|(k, d)| (k, d.curve)).unwrap();
    let (t0, t1) = cube
        .get_curve_geom(curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .params();
    let split = cube.split_edge(edge, (t0 + t1) * f(0.5), tol).unwrap();
    let want = vec![ValidationError::JoinableVertexAtRest {
        vertex: split.vertex,
    }];
    let operand = Dual64::gate_at_rest_kept(cube.clone(), tol).expect("a dual gate runs nothing");

    let other = cube
        .edges()
        .map(|(k, _)| k)
        .find(|&k| k != edge && k != split.new_edge)
        .unwrap();
    for (verb, refused) in [
        (
            "fillet",
            sweep::blend::fillet_edges(&operand, &[other], f(0.05), tol).map(|_| ()),
        ),
        (
            "chamfer",
            sweep::chamfer::chamfer_edges(&operand, &[other], f(0.05), tol).map(|_| ()),
        ),
    ] {
        match refused.map_err(|r| r.error) {
            Err(sweep::blend::BlendError::UnjoinedOperand { errors }) => {
                assert_eq!(errors, want, "{verb}: check 11's finding");
            }
            other => panic!("{verb}: want the check-11 refusal, got {other:?}"),
        }
    }

    let plane = topo::test_support::split_plane(
        Point3::new(f(0.0), f(0.0), f(0.5)),
        Vec3::new(f(0.0), f(0.0), f(1.0)),
        tol,
    );
    match topo::split(&operand, &plane, tol) {
        Err(SplitError::Reduce(SplitReduceError::UnjoinedOperand { errors })) => {
            assert_eq!(errors, want, "split: check 11's finding");
        }
        other => panic!(
            "split: want the check-11 refusal, got {:?}",
            other.map(|_| ())
        ),
    }

    let far = sweep::test_support::brick::<Dual64>((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol);
    let far = Dual64::gate_at_rest_kept(far, tol).expect("a dual gate runs nothing");
    match topo::boolean_op_with(
        BooleanOp::Union,
        &operand,
        &far,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol,
    ) {
        Err(BooleanError::UnjoinedOperand {
            operand: Operand::A,
            errors,
        }) => assert_eq!(errors, want, "union: check 11's finding"),
        other => panic!(
            "union: want the check-11 refusal, got {:?}",
            other.map(|_| ())
        ),
    }
}
