//! Demo-upgrade tripwires + capability pins (Evan's PR #71 question,
//! 2026-07-23): "is the table with legs actually aligned to the
//! corners possible yet? if not, add a tripwire so it upgrades when
//! possible."
//!
//! Findings, encoded below as three tests:
//! 1. ONE corner-aligned leg (outer faces flush with the top's sides
//!    via shared coordinate VALUES, 0.125 overlap into the top) now
//!    UNIONS tier-2-exactly — the mixed collinear+transversal seam
//!    class opened at the PR 5.5 fix pass; the demo's straddle
//!    workaround predates the retry. Pinned as a capability. BUT the
//!    result fails tier 3 (`DescriptionNotAdjacent` on the seam
//!    edges lying IN the shared plane — a plane∥plane "intersection"
//!    is degenerate there, so no honest Intersection description
//!    exists; recorded as a known gap, loud not silent).
//! 2. The FULL table is still impossible — the second leg refuses
//!    `NonMaximalFaces`, because the first union's flush faces do
//!    not merge: the two side planes are equal-but-INDEPENDENT
//!    descriptions, and the ratified coincidence ladder's rung (b)
//!    says value-equality never glues. The missing piece is DECLARED
//!    coincidence as recipe data — exactly M4 PR 5's Declare +
//!    GeomSource (NAMING-DESIGN N6). THE TRIPWIRE watches this: when
//!    the four-leg table builds, the test FAILS with upgrade
//!    instructions for demos/tour stop 8. That failure is the wire
//!    firing, not a regression (precedent: die_blocked's
//!    self-promoting guard).
//! 3. Interval lane: conservative refusal acceptable, silent
//!    wrongness never.
//!
//! Dyadic dimensions throughout (exact volume oracles): top
//! [0,4]×[0,3]×[1,1.25]; legs 0.5×0.5, z ∈ [0,1.125].

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism_z;
use geom_core::Decide;
use topo::validate::{validate_closed, validate_geometric};
use topo::{BooleanResult, BooleanResultKind, mass_properties};

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

/// Capability pin: one corner-aligned leg unions tier-2-exactly.
/// The trailing tier-3 assertion is a SECONDARY tripwire: it pins
/// today's DescriptionNotAdjacent gap and fails when tier 3 starts
/// passing — at which point tighten this test to assert Ok and
/// update the gap note in docs/M4-LOG.md.
#[test]
fn corner_aligned_leg_union_tier2_exact() {
    let r = topo::union(&top::<f64>(), &leg(4.0, 3.0)).expect(
        "REGRESSION: the single corner-aligned leg union (mixed \
         collinear+transversal seam, open since the PR 5.5 fix pass) \
         refused",
    );
    let BooleanResult::Body(bb) = r else {
        panic!("nonempty overlap cannot be Empty")
    };
    assert_eq!(bb.kind, BooleanResultKind::Seamed);
    assert_eq!(validate_closed(&bb.body), Ok(()));
    assert_eq!(bb.body.shells().count(), 1);
    assert_eq!(
        mass_properties(&bb.body).unwrap().volume,
        TOP_VOL + PER_LEG_GAIN,
        "exact dyadic volume"
    );
    match validate_geometric(&bb.body) {
        Err(errs)
            if errs
                .iter()
                .all(|e| format!("{e:?}").contains("DescriptionNotAdjacent")) => {}
        Ok(()) => panic!(
            "SECONDARY TRIPWIRE: the mixed-class seam result now passes \
             tier 3 — the flush-plane seam-edge description gap closed. \
             Tighten this test to assert validate_geometric == Ok and \
             update the gap note (docs/M4-LOG.md, 2026-07-23)."
        ),
        Err(other) => panic!("tier-3 failure moved off the pinned gap class: {other:?}"),
    }
}

/// THE TRIPWIRE: the full corner-aligned table. Today the SECOND leg
/// refuses NonMaximalFaces (the first union's flush faces cannot
/// merge — equal-but-independent descriptions, ladder rung (b)).
/// The expected opener is M4 PR 5 (Declare + GeomSource: shared
/// recipe source ⇒ declared rung ⇒ merge glues).
#[test]
fn tripwire_corner_aligned_table_four_legs() {
    let first = topo::union(&top::<f64>(), &leg(4.0, 3.0))
        .expect("single-leg capability regressed — see the pin test");
    let BooleanResult::Body(bb) = first else {
        panic!("nonempty overlap cannot be Empty")
    };
    match topo::union(&bb.body, &leg(4.0, 0.0)) {
        Err(e) => {
            assert!(
                format!("{e:?}").contains("NonMaximalFaces"),
                "second-leg refusal moved off the documented \
                 NonMaximalFaces class — update this tripwire AND the \
                 M4-LOG note: {e:?}"
            );
        }
        Ok(BooleanResult::Body(second)) => {
            let vol = mass_properties(&second.body).map(|m| m.volume);
            panic!(
                "TRIPWIRE FIRED (not a regression): the second \
                 corner-aligned leg now unions (volume {vol:?}, expected \
                 {}). The full corner-aligned table is buildable — \
                 upgrade demos/tour stop 8 to ship TRUE corner-aligned \
                 legs (Evan, PR #71), verify all four legs with exact \
                 volumes + watertight STL there, then retire this \
                 tripwire and the straddle narration.",
                TOP_VOL + 2.0 * PER_LEG_GAIN
            );
        }
        Ok(BooleanResult::Empty) => panic!("nonempty overlap returned Empty"),
    }
}

/// Interval lane: conservatism acceptable, wrongness never.
#[cfg(feature = "interval")]
#[test]
fn corner_aligned_leg_interval_refuses_or_exact() {
    match topo::union(&top::<geom_core::Interval>(), &leg(4.0, 3.0)) {
        Err(_) => {} // conservative refusal: acceptable
        Ok(BooleanResult::Body(bb)) => {
            assert_eq!(validate_closed(&bb.body), Ok(()));
        }
        Ok(BooleanResult::Empty) => panic!("nonempty overlap returned Empty on Interval"),
    }
}
