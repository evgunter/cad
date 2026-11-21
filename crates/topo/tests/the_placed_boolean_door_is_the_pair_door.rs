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
        let (placed, _) = topo::boolean_placed(
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
        let (placed, _) = topo::boolean_placed(
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

/// **A placed operand whose image stands past the session box refuses
/// at the door, naming it**: B placed 1800 m along `x` refuses as
/// `PlacedBooleanError::Placed` for operand B, with the session box's
/// finding alone, before any Boolean runs.
#[test]
fn a_placed_operand_past_the_box_refuses_naming_it() {
    let tol = Tol::witness();
    let (a, b) = blocks(tol);
    let far = RigidMotion::translation(Vec3::new(1800.0, 0.0, 0.0)).expect("a shift");
    let refused = topo::boolean_placed(
        BooleanOp::Union,
        (Placed::own(&a), Placed::at(&b, far)),
        &BooleanInputs::default(),
        SweepStrategy::Realized,
        tol,
    );
    assert!(
        matches!(
            &refused,
            Err(topo::PlacedBooleanError::Placed { operand: topo::Operand::B, findings })
                if matches!(&findings[..], [topo::ValidationError::OutsideSessionBox { .. }])
        ),
        "B's image past the box refuses, naming B: {:?}",
        refused.as_ref().map(|_| ())
    );
}

/// **Under a boxed motion, the placed door builds what the pair door
/// builds over the operand moved by hand, bit for bit**, at `Interval`:
/// B turned `0.3 ± ε/1000` rad about `z` and shifted half a unit along
/// each axis, so no face of B lies on a face of A, and each of `∪ ∩ ⊖`
/// builds, the same enclosure either way. (At a width of `ε` the turn
/// already refuses a crossing's endpoint certificate, both ways.) One
/// row per operation.
fn boxed_placement_builds_alike(op: BooleanOp) {
    use geom_core::{Interval, Real as _};
    let tol = Tol::witness();
    let fin = |what, x, y, z| finished(what, brick::<Interval>(x, y, z, tol), tol);
    let a = fin("A", (0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = fin("B", (0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let band = geom_core::Band::linear(tol).expect("a band");
    let z = UnitVec3::new(
        Vec3::new(
            Interval::from_f64(0.0),
            Interval::from_f64(0.0),
            Interval::from_f64(1.0),
        ),
        "the turn's axis",
        band,
    )
    .expect("a unit");
    // A thousandth of the session's ε: a boxed turn the band decides
    // at every ε the suites run.
    let width = tol.eps() * 1e-3;
    let turn =
        RigidMotion::rotation(z, Interval::from_bounds(0.3 - width, 0.3 + width)).expect("a turn");
    let motion = RigidMotion::translation(Vec3::new(
        Interval::from_f64(0.5),
        Interval::from_f64(0.5),
        Interval::from_f64(0.5),
    ))
    .expect("a shift")
    .compose(turn)
    .expect("a motion");
    let shown = |r: Result<BooleanResult<Interval>, String>| match r {
        Ok(BooleanResult::Empty) => "empty".to_owned(),
        Ok(BooleanResult::Body(b)) => format!(
            "{} faces {:?}",
            b.body.faces().count(),
            b.body
                .points()
                .map(|(_, p)| format!("{p:?}"))
                .collect::<Vec<_>>()
        ),
        Err(e) => format!("refused {e}"),
    };
    let inputs = BooleanInputs::default();
    let by_hand = match b.moved(&motion, tol) {
        Ok(moved) => topo::boolean_op_with(op, &a, &moved, &inputs, SweepStrategy::Realized, tol)
            .map_err(|e| format!("{e:?}")),
        Err(e) => Err(format!("move {e:?}")),
    };
    let placed = topo::boolean_placed(
        op,
        (Placed::own(&a), Placed::at(&b, motion)),
        &inputs,
        SweepStrategy::Realized,
        tol,
    )
    .map(|(result, _)| result)
    .map_err(|e| match e {
        topo::PlacedBooleanError::Placed { findings, .. } => format!("move {findings:?}"),
        topo::PlacedBooleanError::Boolean(e) => format!("{e:?}"),
    });
    let (by_hand, placed) = (shown(by_hand), shown(placed));
    assert!(placed.contains(" faces "), "{op:?} builds: {placed}");
    assert_eq!(by_hand, placed, "{op:?}");
}

#[test]
fn a_boxed_placement_unites_as_its_moved_body_does_at_interval() {
    boxed_placement_builds_alike(BooleanOp::Union);
}

#[test]
fn a_boxed_placement_intersects_as_its_moved_body_does_at_interval() {
    boxed_placement_builds_alike(BooleanOp::Intersect);
}

#[test]
fn a_boxed_placement_subtracts_as_its_moved_body_does_at_interval() {
    boxed_placement_builds_alike(BooleanOp::Subtract);
}
