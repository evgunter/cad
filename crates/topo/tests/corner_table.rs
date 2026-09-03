//! The corner-aligned table — CAPABILITY PINS (M4 PR 5).
//!
//! History: PR #82 planted two tripwires in `demo_tripwires.rs`
//! (Ev's #71 question — "is the corner-aligned table possible
//! yet?"). M4 PR 5's Declare + GeomSource opened the class exactly as
//! predicted and the PRIMARY tripwire FIRED: with the flush contacts
//! DECLARED, the full four-leg corner-aligned table builds
//! tier-2-exactly (the declared rung glues each leg's flush faces
//! into the top's sides, so every later union sees maximal operands).
//! Per the tripwire's baked instructions the demo (tour stop 8) ships
//! true corner-aligned legs and the straddle narration is retired;
//! these tests pin the capability at the kernel level.
//!
//! Post-retirement posture (the M4 PR 5 narrowing, ladder rung (b)):
//! the SAME unions WITHOUT declarations now refuse loudly
//! (`UndeclaredCoincidence`) — value-equality never glues; the
//! refusal is pinned below too.
//!
//! Dyadic dimensions throughout (exact volume oracles): top
//! [0,4]×[0,3]×[1,1.25]; legs 0.5×0.5, z ∈ [0,1.125].

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{flush_declarations, prism_z};
use geom_core::Decide;
use geom_core::Tol;
use topo::validate::{validate_closed, validate_geometric};
use topo::{BooleanError, BooleanResult, BooleanResultKind, mass_properties, union, union_with};

const TOP_VOL: f64 = 4.0 * 3.0 * 0.25; // 3.0
const PER_LEG_GAIN: f64 = 0.5 * 0.5 * 1.125 - 0.5 * 0.5 * 0.125; // 0.25

fn top<T: Decide>() -> topo::Body<T> {
    prism_z::<T>(&[(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (0.0, 3.0)], 1.0, 1.25).body
}

fn leg<T: Decide>(cx: f64, cy: f64) -> topo::Body<T> {
    let (x0, x1) = if cx == 0.0 {
        (0.0, 0.5)
    } else {
        (cx - 0.5, cx)
    };
    let (y0, y1) = if cy == 0.0 {
        (0.0, 0.5)
    } else {
        (cy - 0.5, cy)
    };
    prism_z::<T>(&[(x0, y0), (x1, y0), (x1, y1), (x0, y1)], 0.0, 1.125).body
}

/// Capability pin: one corner-aligned leg unions tier-2-exactly WITH
/// its flush contacts declared, and the declared rung MERGES the
/// flush faces into the top's sides.
#[test]
fn corner_aligned_leg_union_tier2_exact() {
    let a = top::<f64>();
    let b = leg(4.0, 3.0);
    let r = union_with(&a, &b, &flush_declarations(&a, &b), Tol::witness())
        .expect("REGRESSION: declared corner-aligned leg union refused");
    let BooleanResult::Body(bb) = r else {
        panic!("nonempty overlap cannot be Empty")
    };
    assert_eq!(bb.kind, BooleanResultKind::Seamed);
    assert_eq!(validate_closed(&bb.body), Ok(()));
    assert_eq!(bb.body.shells().count(), 1);
    assert_eq!(
        mass_properties(&bb.body, Tol::witness()).unwrap().volume,
        TOP_VOL + PER_LEG_GAIN,
        "exact dyadic volume"
    );
    // Tier-3 posture probe (the PR #82 SECONDARY tripwire watched a
    // DescriptionNotAdjacent gap on in-plane seam edges; the declared
    // merge consumes those edges): record where the result lands.
    match validate_geometric(&bb.body, Tol::witness()) {
        Ok(()) => {}
        Err(errs) => panic!("tier-3 verdict on the declared merge result: {errs:?}"),
    }
}

/// The fired tripwire, pinned as a capability: the FULL four-leg
/// corner-aligned table builds with declared flush contacts — exact
/// volume, one shell, tier 2 green at every step.
#[test]
fn corner_aligned_table_four_legs_builds() {
    let mut table = top::<f64>();
    for (cx, cy) in [(4.0, 3.0), (4.0, 0.0), (0.0, 3.0), (0.0, 0.0)] {
        let l = leg(cx, cy);
        let r = union_with(&table, &l, &flush_declarations(&table, &l), Tol::witness())
            .expect("declared corner-aligned leg union refused");
        let BooleanResult::Body(bb) = r else {
            panic!("nonempty overlap cannot be Empty")
        };
        table = bb.body;
    }
    assert_eq!(validate_closed(&table), Ok(()));
    assert_eq!(table.shells().count(), 1);
    assert_eq!(
        mass_properties(&table, Tol::witness()).unwrap().volume,
        TOP_VOL + 4.0 * PER_LEG_GAIN,
        "exact dyadic volume, all four legs"
    );
}

/// The narrowing pin (ladder rung (b), post-retirement): the SAME
/// single-leg union WITHOUT declarations refuses loudly — equal bits
/// without shared source or declared intent never glue.
#[test]
fn undeclared_corner_leg_refuses_loudly() {
    match union(&top::<f64>(), &leg(4.0, 3.0), Tol::witness()) {
        Err(BooleanError::UndeclaredCoincidence { .. }) => {}
        other => panic!("undeclared flush union must refuse UndeclaredCoincidence, got {other:?}"),
    }
}

/// Interval lane: conservatism acceptable, wrongness never.
#[cfg(feature = "interval")]
#[test]
fn corner_aligned_leg_interval_refuses_or_exact() {
    let a = top::<geom_core::Interval>();
    let b = leg(4.0, 3.0);
    match union_with(&a, &b, &flush_declarations(&a, &b), Tol::witness()) {
        Err(_) => {} // conservative refusal: acceptable
        Ok(BooleanResult::Body(bb)) => {
            assert_eq!(validate_closed(&bb.body), Ok(()));
        }
        Ok(BooleanResult::Empty) => panic!("nonempty overlap returned Empty on Interval"),
    }
}
