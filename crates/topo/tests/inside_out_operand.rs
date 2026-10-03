//! **An inside-out operand refuses at the Boolean's operand gate.** A
//! prism over a clockwise profile is closed and passes tiers 1 and 2,
//! but its faces point inward: tier 3's check 7 decides its volume
//! negative. Consumed, it is the complement of the wedge it bounds, and
//! ∖ and ∩ answered each other's volume.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, flush_declarations};
use geom_core::{Bounds, Decide, Interval, Tol};
use topo::{
    AtRestPolicy, Body, BooleanError, BooleanResult, Operand, intersect_with, mass_properties,
    subtract_with, union_with,
};

/// The triangle (0,0), 80°, 190° on the unit circle, counterclockwise
/// when `ccw`.
fn wedge_profile(ccw: bool) -> [(f64, f64); 3] {
    let at = |deg: f64| (deg.to_radians().cos(), deg.to_radians().sin());
    if ccw {
        [(0.0, 0.0), at(80.0), at(190.0)]
    } else {
        [(0.0, 0.0), at(190.0), at(80.0)]
    }
}

/// The brick `(0,1)×(0,1)×(0.5,1.5)` and the wedge prism over
/// z ∈ (0.5, 1), flush with it at z = 0.5.
fn operands<T: Decide + AtRestPolicy>(ccw: bool, tol: Tol) -> (Body<T>, Body<T>) {
    (
        brick::<T>((0.0, 1.0), (0.0, 1.0), (0.5, 1.5), tol),
        common::prism_z::<T>(&wedge_profile(ccw), 0.5, 1.0, tol).body,
    )
}

type Op<T> = fn(
    &Body<T>,
    &Body<T>,
    &topo::BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<T>, BooleanError>;

fn ops<T: Decide + Bounds + AtRestPolicy>() -> [(&'static str, Op<T>); 3] {
    [
        ("∖", subtract_with::<T>),
        ("∩", intersect_with::<T>),
        ("∪", union_with::<T>),
    ]
}

/// Every op, in both operand orders, refuses the clockwise wedge as
/// the inside-out operand it is, naming the operand. Red before the
/// gate: ∖ answered 0.0352 and ∩ 0.9648 (each the other's volume), and
/// ∪ was refused `ResultVolumeImplausible` by the backstop.
fn refuses_inside_out<T: Decide + Bounds + AtRestPolicy>(scalar: &str) {
    let tol = Tol::witness();
    let (brick, wedge) = operands::<T>(false, tol);
    for (name, op) in ops::<T>() {
        for (a, b, inside_out) in [(&brick, &wedge, Operand::B), (&wedge, &brick, Operand::A)] {
            let decls = flush_declarations(a, b, tol);
            match op(a, b, &decls, tol) {
                Err(BooleanError::InsideOutOperand { operand, .. }) => assert_eq!(
                    operand, inside_out,
                    "{scalar} {name}: the refusal names the inside-out operand"
                ),
                other => panic!("{scalar} {name}: want InsideOutOperand, got {other:?}"),
            }
        }
    }
}

#[test]
fn an_inside_out_operand_refuses_in_every_op_at_f64() {
    refuses_inside_out::<f64>("f64");
}

#[test]
fn an_inside_out_operand_refuses_in_every_op_at_interval() {
    refuses_inside_out::<Interval>("Interval");
}

/// **The counterclockwise control answers the true volumes**, read off
/// the profile outside the kernel: the wedge's part in the brick's
/// quadrant is the triangle (0,0), its 80° corner, and where its chord
/// crosses x = 0, over a height of 0.5.
#[test]
fn the_counterclockwise_wedge_answers_the_true_volumes() {
    let tol = Tol::witness();
    let [_, p, q] = wedge_profile(true);
    let y0 = p.1 - p.0 * (q.1 - p.1) / (q.0 - p.0);
    let inside = 0.5 * (0.5 * p.0 * y0);
    let wedge = 0.5 * (0.5 * 110f64.to_radians().sin());
    let (brick, prism) = operands::<f64>(true, tol);
    let decls = flush_declarations(&brick, &prism, tol);
    for ((name, op), want) in
        ops::<f64>()
            .into_iter()
            .zip([1.0 - inside, inside, 1.0 + wedge - inside])
    {
        let Ok(BooleanResult::Body(out)) = op(&brick, &prism, &decls, tol) else {
            panic!("{name}: the outward wedge builds");
        };
        let got = mass_properties(&out.body, tol).unwrap().volume;
        assert!(
            (got - want).abs() < 1e-12,
            "{name}: volume {got}, want {want}"
        );
    }
    let at_interval = operands::<Interval>(true, tol);
    let decls = flush_declarations(&at_interval.0, &at_interval.1, tol);
    for (name, op) in ops::<Interval>() {
        assert!(
            matches!(
                op(&at_interval.0, &at_interval.1, &decls, tol),
                Ok(BooleanResult::Body(_))
            ),
            "Interval {name}: the outward wedge builds"
        );
    }
}

/// **A small inside-out part beside a larger ordinary one**, in one
/// operand: the body's total volume is positive, so a reading of the
/// whole operand sees nothing. The gate reads each solid before the
/// pipeline merges them, and names the operand. Red if the operand is
/// read as one merged solid.
#[test]
fn an_inside_out_part_beside_an_ordinary_one_refuses() {
    let tol = Tol::witness();
    let mut parts = Body::<f64>::new();
    for (profile, z) in [
        (
            [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)].to_vec(),
            (0.0, 2.0),
        ),
        (
            wedge_profile(false)
                .iter()
                .map(|&(x, y)| (x + 5.0, y))
                .collect(),
            (0.5, 1.0),
        ),
    ] {
        common::prism_ops(
            &mut parts,
            &profile,
            z,
            common::identity_map,
            common::FaceGeometry::Certified,
            tol,
        );
    }
    common::describe_as_intersections(&mut parts, tol);
    assert!(
        mass_properties(&parts, tol).unwrap().volume > 0.0,
        "the body's total is positive, so it hides the part's sign"
    );
    let cutter = brick::<f64>((0.5, 1.5), (0.5, 1.5), (0.5, 2.5), tol);
    for (name, op) in ops::<f64>() {
        match op(&parts, &cutter, &topo::BooleanDeclarations::none(), tol) {
            Err(BooleanError::InsideOutOperand {
                operand: Operand::A,
                ..
            }) => {}
            other => panic!("{name}: want InsideOutOperand on A, got {other:?}"),
        }
    }
}
