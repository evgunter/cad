//! Issue #93 review pins — the doubly-nested island chain (adversarial
//! review 2026-07-25, constructions adopted verbatim from the review
//! probes). One general-position chain (no value-equal plane pair
//! anywhere), two pins:
//!
//! - **#105 exactness pin**: on pre-#93 main the second union (pillar
//!   into the plate patch inside the tube's hole) SILENTLY returned
//!   22.5 (exact 22.4375) passing tiers 1/2/3′ — a fail-loud
//!   violation; the #93 winding + laringmv repairs make it exact.
//! - **#106 typed-refusal pin**: intersecting the exact chain with a
//!   slab across the tube's midriff puts island(tube outer) ⊃
//!   ring(tube inner) ⊃ island(pillar) on the slab faces; the depth-2
//!   annulus defeats the consecutive-triple centroid generator (every
//!   candidate lands in the hole → none certifies) and the resolver
//!   refuses TYPED at the anchor-exhaustion arm. The arm is
//!   load-bearing; the residue is refusal-only, never wrongness.
//!
//! Depth-1 control (no pillar): same chain intersects exactly.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism_z;
use topo::{
    Body, BooleanBody, BooleanError, BooleanResult, intersect, mass_properties, subtract, union,
    validate, validate_closed, validate_pseudomanifold,
};

/// Tube: outer [1,3]², hole [1.5,2.5]², z ∈ [0.5, 3] (cutter strictly
/// taller so the subtract pierces cleanly).
fn tube() -> Body<f64> {
    let outer = prism_z::<f64>(&[(1.0, 1.0), (3.0, 1.0), (3.0, 3.0), (1.0, 3.0)], 0.5, 3.0);
    let cutter = prism_z::<f64>(
        &[(1.5, 1.5), (2.5, 1.5), (2.5, 2.5), (1.5, 2.5)],
        0.25,
        3.25,
    );
    let BooleanResult::Body(t) = subtract(&outer.body, &cutter.body).expect("tube") else {
        panic!("tube subtract emptied");
    };
    t.body
}

/// Plate [0,4]² × [0,1] ∪ tube: exact 22.0 (plate 16 + tube walls
/// above the plate, annulus 3 × 2).
fn plate_with_tube() -> Body<f64> {
    let plate = prism_z::<f64>(&[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)], 0.0, 1.0);
    let BooleanResult::Body(u1) = union(&plate.body, &tube()).expect("plate|tube") else {
        panic!("plate|tube emptied");
    };
    let v = mass_properties(&u1.body).expect("u1 mass").volume;
    assert!((v - 22.0).abs() < 1e-12, "u1 volume {v} != 22.0 exact");
    u1.body
}

/// Slab across the tube's midriff, z ∈ [1.375, 2.375] — every plane
/// value distinct from every operand plane (general position, no
/// declarations involved).
fn slab() -> Body<f64> {
    prism_z::<f64>(
        &[(-1.0, -1.0), (5.0, -1.0), (5.0, 5.0), (-1.0, 5.0)],
        1.375,
        2.375,
    )
    .body
}

fn tiers(bb: &BooleanBody<f64>, label: &str) {
    assert_eq!(validate(&bb.body), Ok(()), "{label}: tier 1");
    assert_eq!(validate_closed(&bb.body), Ok(()), "{label}: tier 2");
    assert_eq!(
        validate_pseudomanifold(&bb.body, &bb.contacts),
        Ok(()),
        "{label}: tier 3′"
    );
}

/// #105 (main-was-silently-wrong regression): the pillar union is
/// exact on this branch. On main @ 75166b8 this union returned 22.5
/// with all tiers green — the pillar∩plate overlap (0.5×0.5×0.25 =
/// 0.0625) double-counted, a SILENT wrong body from three plain
/// general-position unions. Exact: plate 16 + tube walls 6 + pillar
/// above plate 0.25×1.75 = 22.4375.
#[test]
fn issue105_doubly_nested_union_exact() {
    let u1 = plate_with_tube();
    let pillar = prism_z::<f64>(
        &[(1.75, 1.75), (2.25, 1.75), (2.25, 2.25), (1.75, 2.25)],
        0.75,
        2.75,
    );
    let BooleanResult::Body(u2) = union(&u1, &pillar.body).expect("|pillar") else {
        panic!("|pillar emptied");
    };
    tiers(&u2, "u2");
    let v = mass_properties(&u2.body).expect("u2 mass").volume;
    assert!(
        (v - 22.4375).abs() < 1e-12,
        "REGRESSION toward issue #105: pillar union volume {v} != 22.4375 \
         exact (main silently returned 22.5)"
    );
}

/// #106 (completeness-gap pin): the depth-2 nested intersect refuses
/// TYPED at the anchor-exhaustion arm — never a silent body. If this
/// starts succeeding, verify the volume is EXACTLY 3.25 (annulus 3 +
/// pillar 0.25, slab height 1) with all tiers green, then retire this
/// pin as the #106 closure.
#[test]
fn issue106_depth2_nested_intersect_refuses_typed() {
    let u1 = plate_with_tube();
    let pillar = prism_z::<f64>(
        &[(1.75, 1.75), (2.25, 1.75), (2.25, 2.25), (1.75, 2.25)],
        0.75,
        2.75,
    );
    let BooleanResult::Body(u2) = union(&u1, &pillar.body).expect("|pillar") else {
        panic!("|pillar emptied");
    };
    match intersect(&u2.body, &slab()) {
        Err(BooleanError::JoinDesync { what }) => {
            assert_eq!(
                what,
                "neither section loop's regions hold a classifiable anchor \
                 (vertices, edge midpoints, and verified interior candidates \
                 all exhausted)",
                "depth-2 refusal moved off the anchor-exhaustion arm"
            );
        }
        Err(e) => panic!("depth-2 refusal moved off JoinDesync: {e:?}"),
        Ok(BooleanResult::Body(bb)) => {
            let v = mass_properties(&bb.body).map(|m| m.volume);
            panic!(
                "PIN FIRED (not necessarily a regression): the depth-2 \
                 nested intersect now builds (volume {v:?}, exact 3.25). \
                 Verify exactness + tiers, then retire this pin as the \
                 issue #106 closure."
            );
        }
        Ok(BooleanResult::Empty) => panic!("nonempty overlap returned Empty"),
    }
}

/// Depth-1 control: without the pillar the same intersect builds
/// exactly (annulus 3 × slab height 1) — the #93 coverage boundary is
/// precisely between depth 1 and depth 2.
#[test]
fn depth1_nested_intersect_control_exact() {
    let u1 = plate_with_tube();
    match intersect(&u1, &slab()) {
        Ok(BooleanResult::Body(bb)) => {
            tiers(&bb, "depth-1 control");
            let v = mass_properties(&bb.body).expect("mass").volume;
            assert!((v - 3.0).abs() < 1e-12, "control volume {v} != 3.0 exact");
        }
        other => panic!("depth-1 control did not build: {other:?}"),
    }
}
