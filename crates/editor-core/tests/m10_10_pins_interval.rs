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
//! here is measured at `1e-6`, `1e-9` and `1e-12`. The link, the
//! bracket and the pad are still ε-RELATIVE (each is bounded by an
//! identity residual); the plate and the annulus are NOT any more —
//! their ceilings are fractions of their REAL studies, bounded by
//! real margins.
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
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use geom_core::{SymRules, Tol};

use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::{certifies_whole, over_band_set};

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
        s.trig_of_atan && s.early_ab && s.sqrt_square && s.pythagoras && s.common_factor,
        "rule D, rules A/B per node and rule E ship: {s:?}"
    );
    assert!(
        s.canonical_root && s.decision_read,
        "rule G and the decision read ship: {s:?}"
    );
    assert!(
        s.early && s.const_fold && s.registered,
        "on top of the early walk, A0 and the door: {s:?}"
    );
    assert!(
        !s.signed_root,
        "rule C's own fold at `sqrt`/`abs` stays dial-off: it is inert at the shipped ring \
         width and costs ~2x. The decision read is the value read that DOES ship, at the \
         ops rule C never reached, and it has a dial of its own"
    );
    let off = SymRules::without_the_algebra();
    assert!(
        !off.trig_of_atan
            && !off.early_ab
            && !off.sqrt_square
            && !off.pythagoras
            && !off.common_factor
            && !off.manifest_sign
            && !off.canonical_root
            && !off.abs_square
            && !off.root_magnitude
            && !off.root_quotient
            && !off.decision_read,
        "the algebra off: {off:?}"
    );
    // ELEVEN dials, and the last three are rule G's own conjuncts.
    // Eight of them have been the list since DECIDE-3: rule E
    // (the quotient's common factor, SYM-5), rule F (the manifest sign,
    // SYM-8), rule G (the canonical root) and the decision read are
    // form-level algebra in the early walk like the other four, and
    // `without_the_algebra` is M10-9's tier bit for bit, which had none
    // of them. `m10_9_pins_interval` holds M10-9's rows under it. A
    // dial left out of this list makes the differential one against a
    // tier that never existed — SYM-8's review found exactly that, with
    // rule F on BOTH sides of it.
    //
    // **`abs_square` and `root_magnitude` are the ninth and tenth**
    // (SYM-9): rule G's companion rewrite `|X|² = X²` and its magnitude
    // door `sqrt(R²) = |R|`, each read as a conjunction with
    // `canonical_root`, so neither can turn a step ON in a tier that
    // has rule G shut and `without_the_algebra` is the same TIER it was
    // before they existed. They are here because they must be the same
    // VALUE too: a tier shut two ways is one `SymRules` and rows
    // compare them (`m10_8_pins_interval`'s `a0_alone`).
    //
    // **`root_quotient` is the eleventh**: rule G's exact quotient — a
    // root over `N/D` with `D | N` minted over the polynomial quotient
    // — read as a conjunction with `canonical_root` for the same reason.
    //
    // **Both sets are written out WHOLE, field by field**, with no
    // rest pattern: a dial added to `SymRules` is then a compile error
    // here until this row says which side it is on, and a dial left
    // ON in `without_the_algebra` reds the first assertion rather than
    // hiding behind a `..` the second one filled from it.
    assert_eq!(
        off,
        SymRules {
            sqrt_square: false,
            pythagoras: false,
            const_fold: true,
            early: true,
            early_ab: false,
            trig_of_atan: false,
            signed_root: false,
            common_factor: false,
            manifest_sign: false,
            canonical_root: false,
            abs_square: false,
            root_magnitude: false,
            root_quotient: false,
            decision_read: false,
            registered: true,
        },
        "`without_the_algebra` is M10-9's tier: A0 and the early walk and the door, and \
         nothing of the algebra"
    );
    assert_eq!(
        s,
        SymRules {
            sqrt_square: true,
            pythagoras: true,
            const_fold: true,
            early: true,
            early_ab: true,
            trig_of_atan: true,
            signed_root: false,
            common_factor: true,
            manifest_sign: true,
            canonical_root: true,
            abs_square: true,
            root_magnitude: true,
            root_quotient: true,
            decision_read: true,
            registered: true,
        },
        "`shipped` is `without_the_algebra` with the eleven algebra dials on, and nothing else"
    );
}

/// The plate's per-predicate split at the nominal
/// (`m10_8_harness::split_at_the_nominal`).
fn split_at_the_nominal(rules: SymRules, tol: Tol) -> BTreeMap<&'static str, [u64; 4]> {
    let doc = crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0;
    crate::m10_8_harness::split_at_the_nominal(&doc, rules, tol)
}

/// **ALL FOUR OF THE PLATE'S, at the nominal, as THEOREMS.** With the
/// algebra off the split is M10-9's; with it on,
/// `carrier_on_surface_2` and `witness_on_surface_2` are theorems
/// outright (rule D with A/B per node, no value read, no axiom), and so
/// are `carrier_matches_mapped_source` at every sample (rule D meets
/// the two spellings) and `pcurve_map_residual` at every sample (rule
/// D's A1 folds take the chart's phase — `atan2(0, r²/sqrt(r²))` is the
/// zero form; `cos π` on the negative frame is `−1`). What is left of
/// each after the trig is the rim identity `‖q − c‖ = r`, and the
/// circles store their authored radius as `|r|`, so the rim at a
/// vertex folds onto it with no registration. `symbolic_zero` does not
/// fall anywhere: the plain form is asked first, so the algebra can
/// only add.
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
    // The split with the algebra off.
    assert_eq!(row(&off, "carrier_matches_mapped_source"), [180, 0, 16, 56]);
    assert_eq!(row(&off, "carrier_on_surface_2"), [108, 0, 0, 72]);
    assert_eq!(row(&off, "witness_on_surface_2"), [12, 0, 0, 8]);
    assert_eq!(row(&off, "pcurve_map_residual"), [0, 0, 0, 36]);
    // The algebra on.
    assert_eq!(
        row(&on, "carrier_matches_mapped_source"),
        [252, 0, 0, 0],
        "rule D meets the pushforward and the carrier at every sample, and the rim at a \
         vertex folds onto the authored |r|, so every one of the 72 is a THEOREM"
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
        [36, 0, 0, 0],
        "the chart's phase `atan2(0, r²/sqrt(r²))` folds to the zero form (A1) and `cos π` \
         on the negative frame to −1; the rim folds onto |r|, so every one of the 36 is a \
         THEOREM"
    );
    assert_eq!(
        row(&on, "line_span"),
        [8, 0, 0, 0],
        "the plate's eight `line_span` comparisons are between two rational CONSTANTS, \
         which A0 decides exactly — THEOREMS, no value read \
         (`work/decide/a0-leaves-max-and-min-of-constants-opaque`'s fix)"
    );
    for (p, before) in &off {
        let after = row(&on, p);
        let discharged = |s: [u64; 4]| s[0] + s[1] + s[2];
        assert!(
            after[0] >= before[0] && discharged(after) >= discharged(*before),
            "{p}: the plain form is asked first, so `symbolic_zero` never falls, and no \
             decision is LOST — what the algebra discharges it keeps: {before:?} -> {after:?}"
        );
    }
}

/// **The plate needs no door.** Its circles store their authored
/// carrier, the radius as `|r|`, so the rim identity `‖q − c‖ = r` that
/// closed its scaffold residuals through the registered-identity door
/// is the tier's own algebra: with the door shut, the split at the
/// nominal is the shipped set's, row for row, and at `1.0e3 · ε` of the
/// real study the plate certifies whole under the shipped set, under
/// the shipped set with the door shut, and under M10-9's tier with the
/// algebra off.
#[test]
fn m10_10_the_plate_needs_no_door() {
    let tol = Tol::witness();
    assert_eq!(
        split_at_the_nominal(SymRules::shipped_without_the_door(), tol),
        split_at_the_nominal(SymRules::shipped(), tol),
        "the door shut moves no decision on the plate"
    );
    let eps = tol.eps();
    let doc = crate::m10_7_plate::plate(5.0e-5 * 1.0e3 * eps, 1.0e-5 * 1.0e3 * eps, tol).0;
    for (label, rules) in [
        ("shipped", SymRules::shipped()),
        ("door shut", SymRules::shipped_without_the_door()),
        ("algebra off", SymRules::without_the_algebra()),
    ] {
        assert!(
            certifies_whole(&doc, rules, tol),
            "{label}: the plate certifies whole at 1e3·ε"
        );
    }
}

/// **THE PLATE'S CEILING IS THE FLIP, AND THE WEB MARGIN IS READ
/// EXACTLY.** The web assertion's margin is AFFINE in the study's
/// parameters — `web − floor = 1e-4 + 2·Δhalf_spacing − Δr_a − Δr_b` —
/// so its TRUE range over the box at scale `s` of the real study is
/// `1e-4 ± 1.6e-4·s` (the spacing's ±5e-5·s doubled, and each radius's
/// ±3σ = ±3e-5·s), and the flip enters the box at `s = 0.625`. The
/// circles store their authored radius, so each radius enters the
/// margin once and the replay's enclosure of `assert_bound` IS the true
/// range: a shade under the flip nothing is over the band, and past it
/// `assert_bound` alone is, with its enclosure's lower end the true
/// one.
#[test]
fn m10_10_the_plates_ceiling_is_the_flip() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let flip_enters_at: f64 = 1.0e-4 / 1.6e-4;
    assert!((flip_enters_at - 0.625).abs() < 1.0e-12);
    let over = |s: f64| {
        let doc = crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0;
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let (shapes, _, _) = replay(&doc, &ParamBox::of(&analyzed), SymRules::shipped(), tol);
        over_band_set(&shapes)
            .into_iter()
            .filter(|e| e.over > 0)
            .collect::<Vec<_>>()
    };
    // Below the flip by twenty ε of margin: the true margin clears the
    // band, and so does every enclosure.
    let below = (1.0e-4 - 20.0 * eps) / 1.6e-4;
    let set = over(below);
    assert!(
        set.is_empty(),
        "s = {below}: nothing is over the band below the flip: {}",
        crate::m10_8_harness::render_over_band(&set)
    );
    // Past it: the true margin's lower end is negative, and the
    // enclosure's is that end.
    let past = 0.63;
    let true_lo = 1.0e-4 - 1.6e-4 * past;
    let set = over(past);
    assert_eq!(
        set.iter().map(|e| e.predicate).collect::<Vec<_>>(),
        ["assert_bound"],
        "s = {past}: the web assertion alone is over the band: {}",
        crate::m10_8_harness::render_over_band(&set)
    );
    let (lo, _) = set[0].enclosure;
    println!("   s = {past}: true lower end {true_lo:.6e}, enclosure's {lo:.6e}");
    assert!(
        (lo - true_lo).abs() < 1.0e-9,
        "the enclosure's lower end is the true margin's: {lo:e} against {true_lo:e}"
    );
}
