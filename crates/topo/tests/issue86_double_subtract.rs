//! Issue #86: a legal double-subtract (crossing slots — the second
//! subtract's operand is itself a boolean result) must SUCCEED, not
//! panic in `set_edge_curve`'s tier-1 postcondition (OrphanGeometry).
//!
//! Shape: a 3×3×1 plate; slot 1 cut along y (full run in y, upper
//! half in z), slot 2 cut along x (crossing slot 1). Where the slots
//! cross, the second boolean's coplanar-face merge absorbs faces of
//! the first result whose surfaces stay alive only through first-
//! boolean `Intersection` descriptions; `describe_minted_edges` then
//! re-describes those edges through `set_edge_curve`, dropping the
//! last references — the orphan-hygiene cascade (curve → description
//! surfaces) is what this fixture pins.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism_z;
use geom_core::Decide;
use topo::{
    Body, BooleanBody, BooleanResult, subtract, validate, validate_closed, validate_pseudomanifold,
};

fn brick<T: Decide>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

fn double_subtract_crossing_slots<T: Decide>() -> BooleanBody<T> {
    let a = brick::<T>((0.0, 3.0), (0.0, 3.0), (0.0, 1.0));
    let b1 = brick::<T>((1.0, 2.0), (-1.0, 4.0), (0.5, 1.5));
    let BooleanResult::Body(s1) = subtract(&a, &b1).expect("first subtract succeeds") else {
        panic!("first subtract yields a body");
    };
    let b2 = brick::<T>((-1.0, 4.0), (1.0, 2.0), (0.5, 1.5));
    let BooleanResult::Body(s2) =
        subtract(&s1.body, &b2).expect("second subtract succeeds (issue #86)")
    else {
        panic!("second subtract yields a body");
    };
    s2
}

#[test]
fn double_subtract_crossing_slots_succeeds() {
    let out = double_subtract_crossing_slots::<f64>();
    assert_eq!(validate(&out.body), Ok(()), "tier 1");
    assert_eq!(validate_closed(&out.body), Ok(()), "tier 2");
    assert_eq!(
        validate_pseudomanifold(&out.body, &out.contacts),
        Ok(()),
        "tier 3′"
    );
    // Shake out downstream assumptions: a rigid transform of the
    // result re-certifies cleanly.
    let moved = topo::transform_rigid(
        &out.body,
        &geom_core::Affine3::translation(geom_core::Vec3::new(1.0, 2.0, 3.0)),
    )
    .expect("transform of the double-subtract result");
    assert_eq!(validate(&moved), Ok(()), "tier 1 after transform");
    assert_eq!(validate_closed(&moved), Ok(()), "tier 2 after transform");
}

#[cfg(feature = "interval")]
mod interval {
    use super::*;

    #[test]
    fn double_subtract_crossing_slots_interval() {
        let out = double_subtract_crossing_slots::<geom_core::Interval>();
        assert_eq!(validate(&out.body), Ok(()), "tier 1 (interval)");
        assert_eq!(validate_closed(&out.body), Ok(()), "tier 2 (interval)");
    }
}
