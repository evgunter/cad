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
/// `work/sym/real-margin-dependency-widening` names). The web
/// assertion's margin is AFFINE in the study's parameters — `web −
/// floor = 1e-4 + 2·Δhalf_spacing − Δr_a − Δr_b` — so its TRUE range
/// over the box at scale `s` of the study is exact arithmetic: `1e-4 ±
/// 1.6e-4·s` (the spacing's ±5e-5·s doubled, and each radius's ±3σ =
/// ±3e-5·s). The flip therefore first enters the box at `s = 0.625`.
/// At the measured ceiling (`s ≈ 0.2632`) the true margin is `[5.79e-5,
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
