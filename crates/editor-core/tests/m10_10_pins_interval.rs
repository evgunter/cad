//! **M10-10's positive pins** — the form-level mechanism (rule D: trig
//! of `atan` made exact, and with amendment A1 `atan2` of the zero
//! form over a manifestly non-negative form, and `sin`/`cos` at an
//! exact half-multiple of π; rules A/B per node made affordable),
//! asserted as the measured STATE it is: which of the plate's four
//! identity residuals it discharges and through which rule, what
//! bounds each measured document now, and that the tier with the
//! algebra OFF is M10-9's (`m10_9_pins_interval` holds M10-9's rows
//! under `SymRules::without_the_algebra`, which is that claim in
//! assertable form).
//!
//! The gating half; `m10_10_evidence_interval` is the evidence half
//! (`#[ignore]`d) and `m10_8_harness` the shared probe. Every number
//! here is measured at `1e-6`, `1e-9` and `1e-12`, and every ceiling
//! row asserts BOTH ends of the measured bracket. The link, the
//! bracket and the pad are still ε-RELATIVE (each is bounded by an
//! identity residual); the plate and the annulus are NOT any more —
//! their ceilings are fractions of their REAL studies, bounded by
//! real margins, and are pinned per ε row in those units.
//!
//! **What moved, and what did not.** On the plate all four residuals
//! the staged walk names go: `carrier_on_surface_2` (72 → 0 numeric at
//! the nominal) and `witness_on_surface_2` (8 → 0) as THEOREMS of rule
//! D with rules A/B per node; `carrier_matches_mapped_source` (64 → 0)
//! through the DOOR — rule D makes the two spellings' trig meet at
//! every sample, and the rim identity `‖q − c‖ = r` the registrant
//! states is what closes it, so the count is `registered`; and
//! `pcurve_map_residual` (36 → 0) through the door too, once rule D's
//! A1 folds take the chart's phase — `atan2(0, r²/sqrt(r²))` from the
//! cylinder chart derivation is `atan2` of the zero form over a form
//! non-negative BY SYNTAX, so it is the zero form (no sign read: the
//! `r² = 0` box is one clause 1 refuses), and on the negative frame
//! the `+ π` the branch-stabilized azimuth adds leaves `cos π = −1`
//! outright. The plate's whole-certifying ceiling moves from
//! `1.25e3 · ε` to 0.2368 (ε = 1e-6), 0.2631 (1e-9), 0.2631 (1e-12) of
//! its REAL study — the staged walk's own end, to the bisection step —
//! and is bounded by `assert_bound`'s ENCLOSURE: dependency widening
//! of the document's own web margin, which is affine and positive over
//! the whole box there (`m10_10_the_plates_ceiling_is_dependency_widening_not_a_flip`);
//! the leaves certify up to the real flip at 0.625 of the study.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use geom_core::sym::report::ShapeOutcome;
use geom_core::{SymRules, Tol};

use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::{certifies_whole, nominal_box, over_band_set};

/// A named study with its measured bracket: a scale that certifies
/// whole and one that refuses, both asserted. The scale's UNIT is the
/// closure's: a multiple of ε for the ε-relative documents, a
/// fraction of the real study for the plate and the annulus.
type Bracketed<'a> = (&'static str, f64, f64, &'a dyn Fn(f64) -> ProfileDoc);

/// A named study at the refusing end of its bracket, with the
/// over-band set expected there.
type Bounded<'a> = (
    &'static str,
    f64,
    &'a [&'a str],
    &'a dyn Fn(f64) -> ProfileDoc,
);

/// The ε row this run is on, as the index into a three-row table
/// (`1e-6`, `1e-9`, `1e-12`); any other ε has no measured row and
/// fails loud rather than reading a neighbour's.
fn eps_row(eps: f64) -> usize {
    [1.0e-6, 1.0e-9, 1.0e-12]
        .iter()
        .position(|&e| (eps / e - 1.0).abs() < 1.0e-3)
        .unwrap_or_else(|| panic!("no measured row at eps = {eps:e}: measure one and add it"))
}

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

/// **ALL FOUR OF THE PLATE'S, at the nominal, each through its own
/// rule.** With the algebra off the split is M10-9's; with it on,
/// `carrier_on_surface_2` and `witness_on_surface_2` are theorems
/// outright (rule D with A/B per node, no value read, no axiom),
/// `carrier_matches_mapped_source` is discharged through the door at
/// every sample (rule D meets the two spellings; the rim identity
/// closes it), and `pcurve_map_residual` is discharged through the
/// door at every sample too: rule D's A1 folds take the chart's phase
/// (`atan2(0, r²/sqrt(r²))` is the zero form; `cos π` on the negative
/// frame is `−1`), and what is left is the rim identity again.
/// `symbolic_zero` does not fall anywhere: the plain form is asked
/// first, so the algebra can only add.
#[test]
fn m10_10_all_four_discharge_at_the_nominal_and_the_chart_phase_is_the_doors() {
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
        [0, 0, 36, 0],
        "the chart's phase `atan2(0, r²/sqrt(r²))` folds to the zero form (A1) and `cos π` \
         on the negative frame to −1; the rim identity the door states closes the rest, \
         so every one of the 36 is REGISTERED"
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

/// **THE ε-RELATIVE CEILINGS UNDER THE SHIPPED SET, pinned to the
/// bisection bracket at both ends**, measured at ε = 1e-6, 1e-9 and
/// 1e-12 (each ∝ ε to within one bisection step; the bracket below is
/// the union over the three rows). These three documents are still
/// bounded by an identity residual, so their ceilings still scale
/// with ε:
///
/// | document | certifies at | refuses at | over-band set at ceiling + δ | moved |
/// | --- | --- | --- | --- | --- |
/// | R2 link | `4.930e2 · ε` | `4.934e2 · ε` | `{carrier_matches_mapped_source}` `[0, 1.0004 · ε]` | unmoved |
/// | R2 filleted bracket | `3.870e2 · ε` | `3.874e2 · ε` | `{carrier_matches_mapped_source}` `[0, 1.0003 · ε]` | unmoved |
/// | R2 rounded pad | `2.4990e3 · ε` | `2.5010e3 · ε` | `{line_span}` `[−1.0005 · ε, 1.0005 · ε]` | 1.20× (from `2.083e3`) |
///
/// The scaffold pushforward stands on the link and the bracket because
/// their arcs' carrier FRAMES do not fit the per-node cap (rule D
/// meets the trig; the frame's 1,020-term form does not reduce); the
/// fillet's identity-shaped `line_span` bounds the pad.
#[test]
fn m10_10_the_eps_relative_ceilings_under_the_shipped_set_are_the_measured_brackets() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let docs: [Bracketed<'_>; 3] = [
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

/// **THE PLATE AND THE ANNULUS CERTIFY A FRACTION OF THEIR REAL
/// STUDIES, bounded by REAL margins** — pinned per ε row, in units of
/// the study (the scale `1.0` IS the study: ±0.05 mm of spacing and
/// σ = 0.01 mm on each radius on the plate; R1's annulus at its own
/// real widths), both bracket ends asserted:
///
/// | document | ε | certifies at | refuses at | over-band set at ceiling + δ |
/// | --- | --- | --- | --- | --- |
/// | two-hole plate | `1e-6` | 0.2368 | 0.2369 | `{assert_bound}` `[9.99e-6, 1.90e-4]` |
/// | two-hole plate | `1e-9` | 0.2630 | 0.2632 | `{assert_bound}` `[−2.09e-9, 2.00e-4]` |
/// | two-hole plate | `1e-12` | 0.2630 | 0.2632 | `{assert_bound}` `[−9.12e-9, 2.00e-4]` |
/// | R1 annulus | `1e-6` | 0.6962 | 0.6965 | `{dihedral_wedge}` `[9.99e-6, 5.31e-2]` |
/// | R1 annulus | `1e-9` | 0.8416 | 0.8420 | `{arc_diameter_clearance}` `[−5.61e-8, 8.44e-4]` |
/// | R1 annulus | `1e-12` | 0.8414 | 0.8420 | `{arc_diameter_clearance}` `[−3.99e-8, 8.44e-4]` |
///
/// The plate's rows are the staged walk's own end (0.2368, 0.2630,
/// 0.2631 with every identity residual passed by the dial) to the
/// bisection step: the shipped tier is AT the end of the walk, and
/// passing further residuals moves nothing. What bounds it is the
/// document's web assertion's ENCLOSURE — dependency widening of a
/// real margin, not a flip (the row below this one pins the
/// arithmetic): the margin is affine and positive over the whole box
/// at the ceiling while its enclosure straddles zero at `1e-9` and
/// `1e-12` and sits in the band at `1e-6`. The annulus at `1e-6` is
/// likewise bounded by its dihedral margin's enclosure at the band's
/// floor, and at the two finer rows by its arc-diameter
/// clearance flipping. The ceilings stopped scaling with ε on both.
#[test]
fn m10_10_the_plate_and_the_annulus_certify_a_fraction_of_their_real_studies() {
    let tol = Tol::witness();
    let row = eps_row(tol.eps());
    let plate_rows = [(0.2368, 0.2369), (0.2630, 0.2632), (0.2630, 0.2632)];
    let annulus_rows = [(0.6962, 0.6965), (0.8416, 0.8420), (0.8414, 0.8420)];
    let docs: [Bracketed<'_>; 2] = [
        (
            "two_hole_plate",
            plate_rows[row].0,
            plate_rows[row].1,
            &|s: f64| crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0,
        ),
        (
            "r1_annulus",
            annulus_rows[row].0,
            annulus_rows[row].1,
            &|s: f64| crate::m10_8_r1_probes_interval::annulus(s, tol).0,
        ),
    ];
    for (name, lo, hi, at) in docs {
        assert!(
            certifies_whole(&at(lo), SymRules::shipped(), tol),
            "{name}: {lo} of the real study is inside the measured bracket and must certify \
             whole at eps={:e} — if this fails the ceiling FELL",
            tol.eps()
        );
        assert!(
            !certifies_whole(&at(hi), SymRules::shipped(), tol),
            "{name}: {hi} of the real study is past the measured bracket and must refuse at \
             eps={:e} — if this passes the ceiling ROSE and the table above is stale",
            tol.eps()
        );
    }
}

/// **AND WHAT BOUNDS EACH, at ceiling + δ**, on the three documents a
/// gate can afford to name the set for (the shape report renders
/// every blocked residual; on the bracket and the pad that is
/// minutes). Asserted as a SET, so a second predicate joining it is a
/// failure rather than a silent change of subject. The plate and the
/// annulus read their row's scale and set from the table above.
#[test]
fn m10_10_the_bound_at_ceiling_plus_delta_is_named_per_document() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let row = eps_row(eps);
    let plate_hi = [0.2369, 0.2632, 0.2632][row];
    let annulus_hi = [0.6965, 0.8420, 0.8420][row];
    let annulus_bound: &[&str] = [
        &["dihedral_wedge"],
        &["arc_diameter_clearance"],
        &["arc_diameter_clearance"],
    ][row];
    let docs: [Bounded<'_>; 3] = [
        ("two_hole_plate", plate_hi, &["assert_bound"], &|s: f64| {
            crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0
        }),
        ("r1_annulus", annulus_hi, annulus_bound, &|s: f64| {
            crate::m10_8_r1_probes_interval::annulus(s, tol).0
        }),
        (
            "r2_link",
            4.934e2 * eps,
            &["carrier_matches_mapped_source"],
            &|s: f64| crate::m10_9_r2_probes_interval::link(s, tol).0,
        ),
    ];
    for (name, hi, expected, at) in docs {
        let doc = at(hi);
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

/// **THE PLATE'S CEILING IS DEPENDENCY WIDENING OF A REAL MARGIN, NOT
/// A FLIP** (R2's MAJOR, by execution; the class
/// `work/m10/real-margin-dependency-widening` names). The web
/// assertion's margin is AFFINE in the study's parameters — `web −
/// floor = 1e-4 + 2·Δhalf_spacing − Δr_a − Δr_b` — so its TRUE range
/// over the box at scale `s` of the study is exact arithmetic: `1e-4 ±
/// 1.6e-4·s` (the spacing's ±5e-5·s doubled, and each radius's ±3σ =
/// ±3e-5·s). The flip therefore first enters the box at `s = 0.625`.
/// At the pinned ceiling (`s ≈ 0.2632`) the true margin is `[5.79e-5,
/// 1.42e-4]`, positive everywhere, while the enclosure the replay
/// reports for `assert_bound` straddles zero (`1e-9`, `1e-12`) or sits
/// in the band (`1e-6`) — widened by ~6e-5 on each side. That widening
/// is what bounds the whole-certifying ceiling; the flip is what the
/// LEAVES certify up to (the whole drive's refusals refine to
/// `{assert_bound}` alone at every depth: both reviews' rows).
#[test]
fn m10_10_the_plates_ceiling_is_dependency_widening_not_a_flip() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let row = eps_row(eps);
    let s = [0.2369, 0.2632, 0.2632][row];
    // The true margin over the box at the refusing end of the bracket.
    let (true_lo, true_hi) = (1.0e-4 - 1.6e-4 * s, 1.0e-4 + 1.6e-4 * s);
    assert!(true_lo > 0.0, "the true margin is positive at s = {s}");
    let flip_enters_at: f64 = 1.0e-4 / 1.6e-4;
    assert!((flip_enters_at - 0.625).abs() < 1.0e-12);
    assert!(
        s < flip_enters_at,
        "the ceiling is well inside the flip-free box"
    );
    let doc = crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let (shapes, _, _) = replay(&doc, &ParamBox::of(&analyzed), SymRules::shipped(), tol);
    let set = over_band_set(&shapes);
    let ab = set
        .iter()
        .find(|e| e.predicate == "assert_bound")
        .expect("assert_bound is over the band at ceiling + δ");
    let (lo, hi) = ab.enclosure;
    println!(
        "   s = {s}: true margin [{true_lo:.4e}, {true_hi:.4e}], enclosure [{lo:.4e}, {hi:.4e}], \
         widened {:.3e} below and {:.3e} above",
        true_lo - lo,
        hi - true_hi
    );
    // The enclosure reaches into the band (or past zero) while the
    // true margin never comes within 5e-5 of it.
    assert!(
        lo < 10.0 * eps,
        "the enclosure's lower end is what refuses: {lo:e} against a band of 10·ε"
    );
    let (below, above) = (true_lo - lo, hi - true_hi);
    assert!(
        (4.0e-5..=8.0e-5).contains(&below) && (4.0e-5..=8.0e-5).contains(&above),
        "the widening is ~6e-5 on each side (measured 5.8e-5): {below:e} / {above:e}"
    );
}
