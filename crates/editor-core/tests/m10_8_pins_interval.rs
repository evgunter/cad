//! **M10-8's positive pins**, asserting the measured STATE of the atom
//! algebra rather than a hoped-for number — the gating half of the
//! arc-family measurement (`m10_8_arc_family_interval` is the evidence
//! half, `#[ignore]`d; `m10_8_harness` the shared probe).
//!
//! What ships, and these rows pin: the constant fold A0 in the plain
//! form, and rule C's clause-3 fold in the early walk alongside it
//! (`geom_core::SymRules::shipped`), over an arbitrary-precision
//! coefficient ring. Rules A/B stay dial-selectable and off: over the
//! top residual they add nothing on the documents; per node they are
//! minutes per replay. The rows here pin the set itself and that it is
//! inert on straight geometry.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use geom_core::{SymRules, Tol};

use crate::m10_7_plate::plate;
use crate::m10_8_harness::dials;

/// **M10-8's set is A0 alone, alongside** — the shipped set with the
/// form-level algebra off (`SymRules::without_the_algebra`) and the
/// door shut, which is that tier bit for bit: A0 in the early walk
/// beside an untouched plain form; nothing else on. The shipped set
/// builds on it (the door, then rules A/B per node and rule D —
/// `m10_9_pins_interval`, `m10_10_pins_interval`), and one default
/// carries the shipped set.
#[test]
fn m10_8_the_a0_set_is_a0_alone() {
    let s = a0_alone();
    assert!(
        s.const_fold && s.early,
        "A0, in the early walk alongside the plain form"
    );
    assert!(
        !s.signed_root,
        "rule C is built and dial-selectable, and does not ship (inert)"
    );
    // BY CONSTRUCTION, not by a list: a dial this row forgot to name
    // would be one A0-alone silently carried. `SymRules::none()` is
    // every dial off, so A0 alone IS `none()` with the two A0 needs
    // turned on — and a ninth dial reds here the day it is added
    // rather than the day someone re-reads this assertion.
    assert_eq!(
        s,
        SymRules {
            const_fold: true,
            early: true,
            ..SymRules::none()
        },
        "nothing else: {s:?}"
    );
    assert_eq!(SymRules::default(), SymRules::shipped(), "one default");
    // A default drive serializes exactly what a `shipped()` drive does.
    let tol = Tol::witness();
    let doc = plate(5.0e-5 * 1.0e-6, 1.0e-5 * 1.0e-6, tol).0;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let run = |rules: SymRules| {
        drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: 4,
                symbolic: dials(rules),
                ..DriveConfig::default()
            },
            tol,
        )
        .expect("the plate builds")
        .serialize()
    };
    assert_eq!(run(SymRules::default()), run(SymRules::shipped()));
}

/// **M10-8's set (A0 alone) is exactly the tier with the algebra off
/// and the door shut.** "The algebra" is the early-walk dials
/// `without_the_algebra` names — rules A/B per node, rule D, rule E,
/// rule F, rule G and the decision read — and the census above asserts
/// the WHOLE dial set by construction, so a ninth rule added to the
/// early walk without a line here reds.
fn a0_alone() -> SymRules {
    SymRules {
        registered: false,
        ..SymRules::without_the_algebra()
    }
}

/// **What the shipped set reaches on straight geometry, and it is
/// eight THEOREMS.** The M10-3 slab has no `sqrt` of a constant and no
/// `sqrt` of a square to fold, so every rule of the form-level algebra
/// is inert on it — and that was the whole of this row until A0 learned
/// to decide a `max`/`min` of two rational CONSTANTS, which is what
/// the slab's conditioning floors are made of.
///
/// So the row's claim is now the one that was always load-bearing, and
/// it is stronger than byte-identity was: the eight come out of
/// `numeric` and NOTHING is gated. A comparison of two rationals is a
/// fact of the form, arithmetic in the coefficient ring; answering it
/// as a certified READ would report that fact as conditional on the
/// leaf's box, and the earlier cut of this unit did exactly that
/// (`work/decide/a0-leaves-max-and-min-of-constants-opaque` measured
/// it). Byte-identity with `none()` could not tell the two apart and
/// said "no gain" where a gain is what happened.
///
/// The third arm pins what IS still inert, and byte-identity is still
/// how: the shipped tier serialises byte for byte what **A0 alone**
/// does on this document, so rules A/B, D, E, F, G, the decision read
/// and the registered-identity door reach nothing here and the eight
/// are A0's own yield.
#[test]
fn m10_8_the_shipped_set_is_inert_on_straight_geometry() {
    use crate::m10_3_driver_interval::slab;
    let tol = Tol::witness();
    let doc = slab(1.0, 0.25);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let run = |rules: SymRules| {
        drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: 8,
                symbolic: dials(rules),
                ..DriveConfig::default()
            },
            tol,
        )
        .expect("the slab builds")
        .serialize()
    };
    let (shipped, plain) = (run(SymRules::shipped()), run(SymRules::none()));
    let line = |s: &str| {
        s.lines()
            .find(|l| l.starts_with("decisions "))
            .unwrap_or_default()
            .to_owned()
    };
    assert_eq!(
        line(&plain),
        "decisions symbolic_zero=482 numeric=263 frozen=0",
        "the plain tier on straight geometry"
    );
    assert_eq!(
        line(&shipped),
        "decisions symbolic_zero=490 numeric=255 frozen=0",
        "eight of the 263 numeric refusals are comparisons of two rational CONSTANTS, which \
         A0 decides exactly — THEOREMS, and nothing here is gated"
    );
    assert_eq!(
        shipped,
        run(SymRules {
            const_fold: true,
            ..SymRules::none()
        }),
        "and the FORM-LEVEL rules are inert on the slab: the shipped tier serialises byte \
         for byte what A0 alone does, so the eight are A0's own yield and rules A/B, D, E, \
         F, G, the decision read and the door reach nothing here"
    );
}
