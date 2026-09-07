//! **M10-10's positive pins** — the form-level mechanism (rule D: trig
//! of `atan` made exact; rules A/B per node made affordable), asserted
//! as the measured STATE it is: which of the plate's four identity
//! residuals it discharges and through which rule, what bounds each
//! measured document now, and that the tier with the algebra OFF is
//! M10-9's (`m10_9_pins_interval` holds M10-9's rows under
//! `SymRules::without_the_algebra`, which is that claim in assertable
//! form).
//!
//! The gating half; `m10_10_evidence_interval` is the evidence half
//! (`#[ignore]`d) and `m10_8_harness` the shared probe. Every number
//! here is ε-RELATIVE, measured at `1e-6`, `1e-9` and `1e-12`, and
//! every ceiling row asserts BOTH ends of the measured bracket.
//!
//! **What moved, and what did not.** On the plate three of the four
//! residuals the staged walk names go: `carrier_on_surface_2` (72 → 0
//! numeric at the nominal) and `witness_on_surface_2` (8 → 0) as
//! THEOREMS of rule D with rules A/B per node, and
//! `carrier_matches_mapped_source` (64 → 0) through the DOOR — rule D
//! makes the two spellings' trig meet at every sample, and the rim
//! identity `‖q − c‖ = r` the registrant states is what closes it, so
//! the count is `registered`. The fourth, `pcurve_map_residual`,
//! stays numeric at every sample: its residual carries the chart's
//! phase `atan2(0, ‖a_r‖)` from the cylinder chart derivation, and
//! `atan2(0, X) = 0` needs the sign of `X`, which no value-free rule
//! reads (`work/m10/pcurve-chart-phase-is-atan2-of-the-start-radial`).
//! The plate's whole-certifying ceiling moves from `7.81e2 · ε` to
//! `1.25e3 · ε` and is bounded by that one residual at all three rows;
//! with it passed (the staged dial, evidence-only) the plate certifies
//! 0.237–0.263 of its real study and the first refusal beyond is the
//! assertion's own REAL margin.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use geom_core::sym::report::ShapeOutcome;
use geom_core::{SymRules, Tol};

use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::{certifies_whole, nominal_box, over_band_set};

/// A named study with its measured bracket: a multiple of ε that
/// certifies whole and one that refuses, both asserted.
type Bracketed<'a> = (&'static str, f64, f64, &'a dyn Fn(f64) -> ProfileDoc);

/// **The shipped set carries the algebra, and the algebra is the only
/// difference from M10-9's set.**
#[test]
fn m10_10_the_shipped_set_carries_the_algebra() {
    let s = SymRules::shipped();
    assert_eq!(SymRules::default(), s, "one default");
    assert!(
        s.trig_of_atan && s.early_ab && s.sqrt_square && s.pythagoras,
        "rule D and rules A/B per node ship: {s:?}"
    );
    assert!(
        s.early && s.const_fold && s.registered,
        "on top of the early walk, A0 and the door: {s:?}"
    );
    assert!(
        !s.signed_root,
        "rule C stays dial-off: no rule here reads a value"
    );
    let off = SymRules::without_the_algebra();
    assert!(
        !off.trig_of_atan && !off.early_ab && !off.sqrt_square && !off.pythagoras,
        "the algebra off: {off:?}"
    );
    assert_eq!(
        SymRules {
            trig_of_atan: true,
            early_ab: true,
            sqrt_square: true,
            pythagoras: true,
            ..off
        },
        s,
        "`without_the_algebra` differs from `shipped` in the four algebra dials and nothing else"
    );
}

/// The per-predicate split of one nominal replay:
/// `predicate -> [theorem, gated, registered, numeric]`.
fn split_at_the_nominal(rules: SymRules, tol: Tol) -> BTreeMap<&'static str, [u64; 4]> {
    let doc = crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let (shapes, _, _) = replay(&doc, &nominal_box(&analyzed), rules, tol);
    let mut table: BTreeMap<&'static str, [u64; 4]> = BTreeMap::new();
    for s in &shapes {
        let row = table.entry(s.predicate).or_default();
        row[match s.outcome {
            ShapeOutcome::Theorem => 0,
            ShapeOutcome::SignGated => 1,
            ShapeOutcome::Registered => 2,
            _ => 3,
        }] += 1;
    }
    table
}

/// **THREE OF THE PLATE'S FOUR, at the nominal, each through its own
/// rule.** With the algebra off the split is M10-9's; with it on,
/// `carrier_on_surface_2` and `witness_on_surface_2` are theorems
/// outright (rule D with A/B per node, no value read, no axiom),
/// `carrier_matches_mapped_source` is discharged through the door at
/// every sample (rule D meets the two spellings; the rim identity
/// closes it), and `pcurve_map_residual` is numeric at every sample
/// under both. `symbolic_zero` does not fall anywhere: the plain form
/// is asked first, so the algebra can only add.
#[test]
fn m10_10_three_of_the_four_discharge_at_the_nominal_and_the_fourth_is_the_chart_phase() {
    let tol = Tol::witness();
    let off = split_at_the_nominal(SymRules::without_the_algebra(), tol);
    let on = split_at_the_nominal(SymRules::shipped(), tol);
    let row = |t: &BTreeMap<&'static str, [u64; 4]>, p: &str| {
        t.get(p)
            .copied()
            .unwrap_or_else(|| panic!("no {p} decisions"))
    };
    // M10-9's split, unchanged with the algebra off.
    assert_eq!(row(&off, "carrier_matches_mapped_source"), [180, 0, 8, 64]);
    assert_eq!(row(&off, "carrier_on_surface_2"), [108, 0, 0, 72]);
    assert_eq!(row(&off, "witness_on_surface_2"), [12, 0, 0, 8]);
    assert_eq!(row(&off, "pcurve_map_residual"), [0, 0, 0, 36]);
    // The algebra on.
    assert_eq!(
        row(&on, "carrier_matches_mapped_source"),
        [180, 0, 72, 0],
        "rule D meets the pushforward and the carrier at every sample; the rim identity \
         the door states closes it, so every one of the 72 is REGISTERED"
    );
    assert_eq!(
        row(&on, "carrier_on_surface_2"),
        [180, 0, 0, 0],
        "the cylinder residual at the carrier's samples: a THEOREM of rule D with A/B"
    );
    assert_eq!(
        row(&on, "witness_on_surface_2"),
        [20, 0, 0, 0],
        "the cylinder residual at the strut witness: a THEOREM of rule D with A/B"
    );
    assert_eq!(
        row(&on, "pcurve_map_residual"),
        [0, 0, 0, 36],
        "the chart's phase `atan2(0, ‖a_r‖)` stands: numeric at every sample \
         (work/m10/pcurve-chart-phase-is-atan2-of-the-start-radial)"
    );
    for (p, before) in &off {
        let after = row(&on, p);
        assert!(
            after[0] >= before[0] && after[1] == 0 && before[1] == 0,
            "{p}: the plain form is asked first, so `symbolic_zero` never falls and \
             nothing is sign-gated: {before:?} -> {after:?}"
        );
    }
}

/// **THE CEILINGS UNDER THE SHIPPED SET, pinned to the bisection
/// bracket at both ends**, and the bound at each ceiling + δ, measured
/// at ε = 1e-6, 1e-9 and 1e-12 (each ceiling ∝ ε to within one
/// bisection step; the bracket below is the union over the three rows):
///
/// | document | certifies at | refuses at | over-band set at ceiling + δ | moved |
/// | --- | --- | --- | --- | --- |
/// | two-hole plate | `1.2496e3 · ε` | `1.2506e3 · ε` | `{pcurve_map_residual}` `[0, 1.0004 · ε]` | 1.60× (from `7.81e2`) |
/// | R1 annulus | `1.2455e3 · ε` | `1.2470e3 · ε` | `{pcurve_map_residual}` `[0, 1.0002 · ε]` | 1.60× (from `7.81e2`) |
/// | R2 link | `4.930e2 · ε` | `4.934e2 · ε` | `{carrier_matches_mapped_source}` `[0, 1.0004 · ε]` | unmoved |
/// | R2 filleted bracket | `3.870e2 · ε` | `3.874e2 · ε` | `{carrier_matches_mapped_source}` `[0, 1.0003 · ε]` | unmoved |
/// | R2 rounded pad | `2.4990e3 · ε` | `2.5010e3 · ε` | `{line_span}` `[−1.0005 · ε, 1.0005 · ε]` | 1.20× (from `2.083e3`) |
///
/// Every ceiling still scales with ε: the residual that bounds each
/// document is an identity residual (the chart's phase on the plate and
/// the annulus; the scaffold pushforward on the link and the bracket,
/// whose arcs' carrier frames rule D does not reach; the fillet's
/// identity-shaped `line_span` on the pad), not a real margin.
#[test]
fn m10_10_the_ceilings_under_the_shipped_set_are_the_measured_brackets() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let docs: [Bracketed<'_>; 5] = [
        ("two_hole_plate", 1.2496e3, 1.2506e3, &|s: f64| {
            crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0
        }),
        ("r1_annulus", 1.2455e3, 1.2470e3, &|s: f64| {
            crate::m10_8_r1_probes_interval::annulus(s, tol).0
        }),
        ("r2_link", 4.930e2, 4.934e2, &|s: f64| {
            crate::m10_9_r2_probes_interval::link(s, tol).0
        }),
        ("r2_filleted_bracket", 3.870e2, 3.874e2, &|s: f64| {
            crate::m10_7_r2_probes_interval::bracket(s, tol).0
        }),
        ("r2_rounded_pad", 2.4990e3, 2.5010e3, &|s: f64| {
            crate::m10_8_r2_probes_interval::pad(s, tol).0
        }),
    ];
    for (name, lo, hi, at) in docs {
        assert!(
            certifies_whole(&at(lo * eps), SymRules::shipped(), tol),
            "{name}: {lo:e}·ε is inside the measured bracket and must certify whole at \
             eps={eps:e} — if this fails the ceiling FELL"
        );
        assert!(
            !certifies_whole(&at(hi * eps), SymRules::shipped(), tol),
            "{name}: {hi:e}·ε is past the measured bracket and must refuse at eps={eps:e} — \
             if this passes the ceiling ROSE and the table above is stale"
        );
    }
}

/// **AND WHAT BOUNDS EACH, at ceiling + δ**, on the three documents a
/// gate can afford to name the set for (the shape report renders
/// every blocked residual; on the bracket and the pad that is
/// minutes). Asserted as a SET, so a second predicate joining it is a
/// failure rather than a silent change of subject.
#[test]
fn m10_10_the_bound_at_ceiling_plus_delta_is_named_per_document() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let docs: [(&str, f64, &[&str], &dyn Fn(f64) -> ProfileDoc); 3] = [
        (
            "two_hole_plate",
            1.2506e3,
            &["pcurve_map_residual"],
            &|s: f64| crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0,
        ),
        (
            "r1_annulus",
            1.2470e3,
            &["pcurve_map_residual"],
            &|s: f64| crate::m10_8_r1_probes_interval::annulus(s, tol).0,
        ),
        (
            "r2_link",
            4.934e2,
            &["carrier_matches_mapped_source"],
            &|s: f64| crate::m10_9_r2_probes_interval::link(s, tol).0,
        ),
    ];
    for (name, hi, expected, at) in docs {
        let doc = at(hi * eps);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let (shapes, _, _) = replay(&doc, &ParamBox::of(&analyzed), SymRules::shipped(), tol);
        let over: Vec<&'static str> = over_band_set(&shapes).iter().map(|e| e.predicate).collect();
        assert_eq!(
            over, expected,
            "{name}: at ceiling + δ the over-band set is the bound, and it is this one"
        );
    }
}

/// **The door and the algebra move the plate TOGETHER, and neither
/// alone.** Between M10-9's ceiling and M10-10's — at `1.0e3 · ε` of
/// the real study — the plate certifies whole under the shipped set
/// and refuses under each of its two halves: the algebra with the door
/// shut (rule D meets the two spellings' trig at every sample, but the
/// rim identity `‖q − c‖ = r` that closes the scaffold residual is the
/// door's), and the door with the algebra off (M10-9's tier, whose
/// door reaches the `i = 0` sample alone). This is the mechanism claim
/// in one drive triple, ε-relative.
#[test]
fn m10_10_the_door_and_the_algebra_move_the_plate_together() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let doc = crate::m10_7_plate::plate(5.0e-5 * 1.0e3 * eps, 1.0e-5 * 1.0e3 * eps, tol).0;
    assert!(
        certifies_whole(&doc, SymRules::shipped(), tol),
        "the shipped set certifies the plate whole at 1e3·ε"
    );
    assert!(
        !certifies_whole(&doc, SymRules::shipped_without_the_door(), tol),
        "the algebra with the door shut does not: the rim identity is the door's"
    );
    assert!(
        !certifies_whole(&doc, SymRules::without_the_algebra(), tol),
        "the door with the algebra off does not: M10-9's ceiling is 7.81e2·ε"
    );
}
