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
    assert!(
        !s.early_ab
            && !s.trig_of_atan
            && !s.sqrt_square
            && !s.pythagoras
            && !s.common_factor
            && !s.manifest_sign
            && !s.registered,
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
/// and the door shut.** "The algebra" is the six early-walk dials
/// `without_the_algebra` names — rules A/B per node, rule D, rule E and
/// rule F — and the census above asserts each of them off by name, so a
/// seventh rule added to the early walk without a line here reds.
fn a0_alone() -> SymRules {
    SymRules {
        registered: false,
        ..SymRules::without_the_algebra()
    }
}

/// **The shipped set is inert on straight geometry**: the M10-3 slab
/// has no `sqrt` of a constant and no `sqrt` of a square to fold, so a
/// shipped drive serializes byte for byte what M10-7's tier did.
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
    assert_eq!(
        run(SymRules::shipped()),
        run(SymRules::none()),
        "straight geometry: the shipped tier is M10-7's, bit for bit"
    );
}
