//! **A cylinder resting on a box's top grazes it along a line, and the
//! Boolean never builds the graze wrong.** The cylinder of radius ½
//! about `y`, `0 ≤ y ≤ 1`, and a box whose top `z = −½` (or bottom
//! `z = ½`) holds the line where the wall is tangent to it, `x = 0`.
//! The wall's germ at that line lies in the box's face, whose side is
//! the second order's (`boolean::join`'s `germ_side` reads it Zero and
//! nothing).
//!
//! - Where the box reaches past both rims, the touch line runs rim to
//!   rim, and each op builds at its closed form, the touch a line of
//!   no volume: `∪` the sum, `∩` empty, `∖` the operand. The union's
//!   two solids touch along the line, and its tier 3′ census reads
//!   them `CensusUndecidable { MayOverlap }`: it cannot yet tell
//!   whether a curved face of one touches the other.
//! - Where the box's face ends inside the wall, the touch line ends on
//!   the wall at the box's edges, and each op refuses
//!   `CurvedPierceUnsupported` by name: the box's edge pierces the wall
//!   tangentially.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::membership::{self, Region, cuboid};
use geom_core::{Point2, Point3, Tol};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{
    AtRestBody, BooleanError, BooleanResult, ContactRecords, Undecidable, ValidationError,
    intersect, subtract, union, validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

/// The cylinder's volume.
const CYL: f64 = core::f64::consts::PI / 4.0;

fn cylinder() -> AtRestBody<f64> {
    finished(
        "the cylinder",
        revolved_about_y(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(0.5, 0.0), 0.0),
                (Point2::new(0.5, 1.0), 0.0),
                (Point2::new(0.0, 1.0), 0.0),
            ],
            Revolution::Full,
            tol(),
        ),
        tol(),
    )
}

fn the_cylinder() -> Region {
    Box::new(|p: Point3<f64>| (0.5 - p.x.hypot(p.z)).min(p.y).min(1.0 - p.y))
}

fn a_box(y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished(
        "the box",
        topo::test_support::brick((-1.0, 1.0), y, z, tol()),
        tol(),
    )
}

/// **Past both rims, every op builds at its closed form.**
#[test]
fn a_box_past_both_rims_builds_every_op_at_its_closed_form() {
    let (y, z) = ((-0.5, 1.5), (-1.5, -0.5));
    let (cyl, tool) = (cylinder(), a_box(y, z));
    let size = 2.0 * 2.0 * 1.0;
    let lattice = (
        Point3::new(-1.05, -0.55, -1.55),
        Point3::new(1.05, 1.55, 0.55),
    );
    let b = move || cuboid((-1.0, 1.0), y, z);
    let body = |r: Result<BooleanResult<f64>, BooleanError>, what: &str| match r {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("{what}: {other:?}"),
    };
    let union_region = move || -> Region { Box::new(move |p| the_cylinder()(p).max(b()(p))) };
    for (op, r) in [
        ("cylinder ∪ box", union(&cyl, &tool, tol())),
        ("box ∪ cylinder", union(&tool, &cyl, tol())),
    ] {
        let u = body(r, op);
        membership::holds_short_of_tier_3_prime(op, &u, CYL + size, &union_region(), lattice, 9);
        let census = validate_pseudomanifold(&u, &ContactRecords::default(), tol());
        let Err(errors) = &census else {
            panic!("{op}: the census decides the line touch: {census:?}");
        };
        assert!(
            errors.iter().all(|e| matches!(
                e,
                ValidationError::CensusUndecidable {
                    reason: Undecidable::MayOverlap,
                    ..
                }
            )),
            "{op}: {errors:?}"
        );
    }
    for (op, r, want, region) in [
        (
            "cylinder ∖ box",
            subtract(&cyl, &tool, tol()),
            CYL,
            the_cylinder(),
        ),
        ("box ∖ cylinder", subtract(&tool, &cyl, tol()), size, b()),
    ] {
        membership::holds(op, &body(r, op), want, &region, lattice, 9);
    }
    for (op, r) in [
        ("cylinder ∩ box", intersect(&cyl, &tool, tol())),
        ("box ∩ cylinder", intersect(&tool, &cyl, tol())),
    ] {
        assert!(matches!(r, Ok(BooleanResult::Empty)), "{op}: {r:?}");
    }
}

/// **A box face ending inside the wall refuses every op by name**, on
/// the box's top below the wall and its bottom above it.
#[test]
fn a_box_face_ending_inside_the_wall_refuses_every_op_by_name() {
    let cyl = cylinder();
    for (what, z) in [("below", (-1.5, -0.5)), ("above", (0.5, 1.5))] {
        let tool = a_box((0.25, 0.75), z);
        for (op, r) in [
            ("cylinder ∪ box", union(&cyl, &tool, tol())),
            ("box ∪ cylinder", union(&tool, &cyl, tol())),
            ("cylinder ∩ box", intersect(&cyl, &tool, tol())),
            ("box ∩ cylinder", intersect(&tool, &cyl, tol())),
            ("cylinder ∖ box", subtract(&cyl, &tool, tol())),
            ("box ∖ cylinder", subtract(&tool, &cyl, tol())),
        ] {
            assert!(
                matches!(r, Err(BooleanError::CurvedPierceUnsupported { .. })),
                "the box {what}, {op}: {r:?}"
            );
        }
    }
}
