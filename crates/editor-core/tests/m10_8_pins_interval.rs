//! **M10-8's positive pins**, asserting the measured STATE of the atom
//! algebra rather than a hoped-for number — the gating half of the
//! arc-family measurement (`m10_8_arc_family_interval` is the evidence
//! half, `#[ignore]`d).
//!
//! The §1 measurement filed all three rules (`geom_core::SymRules`),
//! and these rows pin why: the shipped tier is the M10-7 quotient form
//! exactly (the rules are off by default), and even switched ON the
//! rules discharge NO decision the plain form did not on any of the
//! three documents — the arc-family subforms freeze before a
//! top-residual reduction can reach them. The geom-core suite pins that
//! the rules ARE correct on forms that fit the budget; these rows pin
//! that the corpus does not exercise them.
//!
//! These are the M10-7 rows re-cut: M10-7 pinned the plate at `7.81e-7`
//! and the filleted bracket at "factor exactly 1.0" (tier on == off).
//! Both HOLD under the shipped tier, and are re-cut here as the positive
//! statement that the atom algebra moves neither.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, SymbolicDials, drive};
use geom_core::{SymRules, Tol};

use crate::m10_7_plate::plate;
use crate::m10_7_r2_probes_interval::bracket as r2_bracket;

/// The dials with a chosen rule set, the tier on.
fn dials(rules: SymRules) -> SymbolicDials {
    SymbolicDials {
        rules,
        ..SymbolicDials::default()
    }
}

/// Whether `doc` certifies its WHOLE analyzed box in one leaf under
/// `rules`.
fn certifies_whole(doc: &ProfileDoc, rules: SymRules, tol: Tol) -> bool {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    drive(
        doc,
        &analyzed,
        &DriveConfig {
            max_depth: 0,
            max_leaves: 1,
            symbolic: dials(rules),
            ..DriveConfig::default()
        },
        tol,
    )
    .is_ok_and(|v| v.receipt().certified == 1)
}

/// **The shipped tier is the M10-7 quotient form — the atom algebra is
/// off by default.** `SymRules::default()` (what `SymbolicDials::default`
/// carries) has every rule off, so a default drive serializes exactly
/// what a rules-off drive does, on curved geometry and straight alike.
#[test]
fn m10_8_the_atom_algebra_is_filed_off_by_default() {
    assert_eq!(
        SymRules::default(),
        SymRules::none(),
        "the shipped rule set is empty — the algebra is filed, not shipped"
    );
    let tol = Tol::witness();
    let docs = [
        ("plate", plate(5.0e-5 * 1.0e-6, 1.0e-5 * 1.0e-6, tol).0),
        ("bracket", r2_bracket(1.0e-6, tol).0),
    ];
    for (name, doc) in docs {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let serialize = |rules: SymRules| {
            drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    max_leaves: 64,
                    symbolic: dials(rules),
                    ..DriveConfig::default()
                },
                tol,
            )
            .expect("the nominal builds")
            .serialize()
        };
        assert_eq!(
            serialize(SymRules::default()),
            serialize(SymRules::none()),
            "{name}: the default drive is the rules-off drive"
        );
    }
}

/// **The atom algebra discharges NO decision the plain form did not, on
/// the filleted bracket.** Switched fully on (A+B+C), the drive answers
/// exactly the same decision counts as the plain quotient form — the
/// arc-family subforms freeze before the top-residual reduction can
/// reach them, so no rule fires. This is the "factor 1.0" M10-7 pinned,
/// re-cut as the receipt equality that the algebra moves nothing here.
#[test]
fn m10_8_the_algebra_discharges_nothing_extra_on_the_bracket() {
    let tol = Tol::witness();
    let doc = r2_bracket(1.0e-6, tol).0;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let decisions = |rules: SymRules| {
        drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: 64,
                symbolic: dials(rules),
                ..DriveConfig::default()
            },
            tol,
        )
        .expect("the bracket builds")
        .decisions()
    };
    let all = decisions(SymRules::all());
    let plain = decisions(SymRules::none());
    assert_eq!(
        all, plain,
        "the atom algebra reaches no frozen arc-family subform: {all:?} vs {plain:?}"
    );
}

/// **The two-hole plate's ceiling is UNMOVED by the algebra**, on or
/// off, and it is bounded by the arc rim's endpoint fact no rule
/// reaches (`carrier_endpoint_start`, the `|r − sqrt(D)|` whose radius
/// sqrt is buried inside the outer distance sqrt). A box just inside the
/// ceiling certifies under both the full algebra and none; a box just
/// outside certifies under neither. M10-7's `7.81e-7` pin, re-cut.
#[test]
fn m10_8_the_plate_ceiling_is_unmoved_by_the_algebra() {
    let tol = Tol::witness();
    let at = |scale: f64| plate(5.0e-5 * scale, 1.0e-5 * scale, tol).0;
    for (scale, expect) in [(5.0e-7, true), (1.0e-6, false)] {
        let doc = at(scale);
        assert_eq!(
            certifies_whole(&doc, SymRules::all(), tol),
            expect,
            "plate at {scale:e} with the full algebra"
        );
        assert_eq!(
            certifies_whole(&doc, SymRules::none(), tol),
            expect,
            "plate at {scale:e} without it — the ceiling is the same either way"
        );
    }
}

/// **The filleted bracket's whole-box certification is unmoved too.**
/// The algebra does not certify a box the plain form refuses, nor the
/// reverse — the ceiling is a property of the numeric channel here, not
/// of the atom family, because the family's forms freeze.
#[test]
fn m10_8_the_bracket_ceiling_is_unmoved_by_the_algebra() {
    let tol = Tol::witness();
    let doc = r2_bracket(1.0e-7, tol).0;
    assert_eq!(
        certifies_whole(&doc, SymRules::all(), tol),
        certifies_whole(&doc, SymRules::none(), tol),
        "the algebra moves no bracket ceiling — the arc-family forms freeze"
    );
}
