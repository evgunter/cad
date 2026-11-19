//! **The placed Boolean door is the pair door** (`[placement.frame]`,
//! fork 288 stage 1): `topo::boolean_placed` over two operands in the
//! computing frame builds what `topo::boolean_op_with` builds, bit for
//! bit, since the plain door is the placed door with no motion; and over
//! an operand placed under a motion it builds what the pair door builds
//! over that operand moved first (`FinishedBody::moved`), since the move
//! is the door's own.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::{Point3, RigidMotion, Tol, UnitVec3, Vec3};
use topo::test_support::{brick, finished};
use topo::{
    AtRestBody, BooleanInputs, BooleanOp, BooleanResult, FinishedBody as _, Placed, SweepStrategy,
};

/// A result's points and face count, as bits.
fn bits(result: &BooleanResult<f64>) -> String {
    match result {
        BooleanResult::Empty => "empty".to_owned(),
        BooleanResult::Body(b) => format!(
            "{} faces {:?}",
            b.body.faces().count(),
            b.body
                .points()
                .map(|(_, p)| *p)
                .collect::<Vec<Point3<f64>>>()
        ),
    }
}

fn blocks(tol: Tol) -> (AtRestBody<f64>, AtRestBody<f64>) {
    (
        finished(
            "A",
            brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
            tol,
        ),
        finished(
            "B",
            brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
            tol,
        ),
    )
}

/// A turn of 0.3 rad about `z` through the origin, then half a unit
/// along `x` and `y`: B overlaps A, turned.
fn motion() -> RigidMotion<f64> {
    let band = geom_core::Band::linear(Tol::witness()).expect("a band");
    let z = UnitVec3::new(Vec3::new(0.0, 0.0, 1.0), "the turn's axis", band).expect("a unit");
    let turn = RigidMotion::rotation(z, 0.3).expect("a turn");
    RigidMotion::translation(Vec3::new(0.5, 0.5, 0.0))
        .expect("a shift")
        .compose(turn)
        .expect("a motion")
}

const OPS: [BooleanOp; 3] = [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract];

#[test]
fn the_placed_door_with_no_motion_is_the_pair_door_bit_for_bit() {
    let tol = Tol::witness();
    let (a, b) = blocks(tol);
    let b = b.moved(&motion(), tol).expect("B moves");
    for op in OPS {
        let inputs = BooleanInputs::default();
        let plain = topo::boolean_op_with(op, &a, &b, &inputs, SweepStrategy::Realized, tol)
            .expect("the pair door builds");
        let placed = topo::boolean_placed(
            op,
            (Placed::own(&a), Placed::own(&b)),
            &inputs,
            SweepStrategy::Realized,
            tol,
        )
        .unwrap_or_else(|e| panic!("{op:?}: the placed door builds: {e:?}"));
        assert_eq!(bits(&plain), bits(&placed), "{op:?}");
    }
}

#[test]
fn a_placed_operand_builds_what_its_moved_body_builds_bit_for_bit() {
    let tol = Tol::witness();
    let (a, b) = blocks(tol);
    let moved = b.moved(&motion(), tol).expect("B moves");
    for op in OPS {
        let inputs = BooleanInputs::default();
        let by_hand = topo::boolean_op_with(op, &a, &moved, &inputs, SweepStrategy::Realized, tol)
            .expect("the pair door builds");
        let placed = topo::boolean_placed(
            op,
            (Placed::own(&a), Placed::at(&b, motion())),
            &inputs,
            SweepStrategy::Realized,
            tol,
        )
        .unwrap_or_else(|e| panic!("{op:?}: the placed door builds: {e:?}"));
        assert_eq!(bits(&by_hand), bits(&placed), "{op:?}");
    }
}
