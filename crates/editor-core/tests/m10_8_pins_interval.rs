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
//! minutes per replay. Every number here is ε-RELATIVE (the ceilings
//! are the numeric channel's and scale with `CAD_TOLERANCE_EPS`, one
//! draw per hosted run — the first cut of the plate pin wrote a
//! constant and was red at `1e-12`), and every ceiling row asserts both
//! of its ends so a `false == false` cannot pass for a measurement.
//!
//! These are the M10-7 rows re-cut: M10-7 pinned the plate at `7.81e-7`
//! (at the default epsilon) and the filleted bracket at "factor exactly
//! 1.0" (tier on == off). The plate's number holds for the PLAIN tier
//! and moves under the shipped one; the bracket's "factor 1.0" is gone
//! — the shipped tier certifies the bracket whole an order of magnitude
//! wider than M10-7's — and both facts are pinned as the positive
//! statement, with the factor.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use geom_core::{SymRules, Tol};

use crate::m10_7_plate::plate;
use crate::m10_7_r2_probes_interval::bracket as r2_bracket;
use crate::m10_8_harness::{ceiling, dials};

/// **The shipped set is A0 alone** — one default, carried by
/// `SymbolicDials::default()` and `with_session` alike. Pinned as the
/// measured decision it is (`geom_core::SymRules::shipped`'s docs carry
/// the numbers): A/B over the top residual add no discharge on the
/// documents; A/B per node are minutes per replay; rule C's early walk
/// folds on no document and moves no ceiling while costing 2× per leaf.
#[test]
fn m10_8_the_shipped_set_is_a0_alone() {
    let s = SymRules::shipped();
    assert_eq!(SymRules::default(), s, "one default");
    assert!(s.const_fold, "A0 ships");
    assert!(
        !s.early && !s.signed_root,
        "rule C is built and dial-selectable, and does not ship (inert, 2x per leaf)"
    );
    assert!(!s.early_ab, "per-node A/B does not ship (cost)");
    assert!(
        !s.sqrt_square && !s.pythagoras,
        "top-residual A/B do not ship (inert on the documents)"
    );
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

/// **On curved geometry the shipped set discharges MORE than the plain
/// form and never less** — the bracket at `1e2 · ε` of its study, a box
/// between the plain tier's ceiling (`3.7e1 · ε`) and the shipped
/// one's (`3.9e2 · ε`): more symbolic decisions, fewer numeric ones, at
/// least as many leaves certified, at every ε row.
#[test]
fn m10_8_the_shipped_set_discharges_more_on_the_bracket() {
    let tol = Tol::witness();
    let doc = r2_bracket(1.0e2 * tol.eps(), tol).0;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let run = |rules: SymRules| {
        let v = drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: 4,
                symbolic: dials(rules),
                ..DriveConfig::default()
            },
            tol,
        )
        .expect("the bracket builds");
        (v.receipt().certified, v.decisions())
    };
    let (shipped_cert, shipped) = run(SymRules::shipped());
    let (plain_cert, plain) = run(SymRules::none());
    println!(
        "   shipped {shipped:?} certified {shipped_cert}; plain {plain:?} certified {plain_cert}"
    );
    assert!(
        shipped.symbolic_zero + shipped.sign_gated > plain.symbolic_zero + plain.sign_gated,
        "the shipped tier discharges more: {shipped:?} vs {plain:?}"
    );
    assert!(shipped.numeric < plain.numeric);
    assert!(shipped_cert >= plain_cert);
}

/// Bisects the whole-certifying ceiling of `doc_at` under `rules` on a
/// log scale between `lo` and `hi`, asserting BOTH ends (the small box
/// certifies, the large one refuses) so the bracket is a measurement
/// and not a `false == false`.
fn measured_ceiling(
    what: &str,
    doc_at: &dyn Fn(f64) -> editor_core::ProfileDoc,
    rules: SymRules,
    tol: Tol,
    lo: f64,
    hi: f64,
) -> (f64, f64) {
    let (c_lo, c_hi, per) = ceiling(doc_at, rules, tol, lo, hi, 10);
    assert!(
        !c_lo.is_nan(),
        "{what}: the sweep's small end {lo:e} must certify whole at eps={:e}",
        tol.eps()
    );
    assert!(
        c_hi.is_finite(),
        "{what}: the sweep's large end {hi:e} must refuse at eps={:e}",
        tol.eps()
    );
    println!(
        "{what} at eps={:e}: certifies at {c_lo:e}, refuses at {c_hi:e} ({per:.2}s per probe)",
        tol.eps()
    );
    (c_lo, c_hi)
}

/// **The two-hole plate's ceiling is UNMOVED**: M10-7's `7.81e-7` is
/// `7.81e2 · ε` at every row (the ceiling is the numeric channel's and
/// scales with the band; measured `7.8e-4`, `7.8e-7`, `7.8e-10` at
/// `1e-6`, `1e-9`, `1e-12`), under the plain tier and the shipped one
/// alike — the rim residual `carrier_endpoint_start` is bounded by a
/// nested `sqrt(…)²` no shipped rule reaches
/// (`work/m10/plate-rim-residual-needs-the-wide-coefficient-ring`).
/// Both tiers' brackets are asserted at both ends and must contain the
/// same `7.81e2 · ε`.
#[test]
fn m10_8_the_plate_ceiling_under_the_plain_and_the_shipped_tier() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let at = |scale: f64| plate(5.0e-5 * scale, 1.0e-5 * scale, tol).0;
    let (lo, hi) = measured_ceiling(
        "plate, plain tier",
        &at,
        SymRules::none(),
        tol,
        eps,
        eps * 1.0e6,
    );
    let m10_7 = 7.81e2 * eps;
    assert!(
        lo <= m10_7 && m10_7 <= hi,
        "the plain bracket [{lo:e}, {hi:e}] must contain M10-7's 7.81e-7 restated at this epsilon, {m10_7:e}"
    );
    let (s_lo, s_hi) = measured_ceiling(
        "plate, shipped tier",
        &at,
        SymRules::shipped(),
        tol,
        eps,
        eps * 1.0e6,
    );
    assert!(
        s_lo <= m10_7 && m10_7 <= s_hi,
        "the shipped bracket [{s_lo:e}, {s_hi:e}] must contain the same {m10_7:e}: unmoved"
    );
    assert!(
        s_lo >= lo,
        "the shipped tier never certifies less than the plain one"
    );
}

/// **The filleted bracket's ceiling MOVES**: M10-7's "factor exactly
/// 1.0" (tier on == off) held because M10-7's tier reached nothing on
/// this document; the PLAIN tier certifies the bracket whole below
/// `3.7e1 · ε` and the shipped tier below `3.9e2 · ε` — a factor of
/// 10.4, from the constant fold alone (`sqrt(1)^k` and `sqrt` of
/// exact-square dyadics froze every rim form). Measured at `1e-6`,
/// `1e-9` and `1e-12`: plain `[3.70, 3.75]e1 · ε` at every row; shipped
/// `[3.82, 3.87]e2 · ε` at `1e-6` and `[3.87, 3.92]e2 · ε` at the other
/// two (the true ceiling sits near `3.87e2 · ε`, on either side of the
/// bisection grid), so the pin asks each ten-step bracket to OVERLAP
/// the measured band rather than to contain one number. Both brackets
/// are asserted at both ends.
#[test]
fn m10_8_the_bracket_ceiling_moves_under_the_shipped_tier() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let at = |scale: f64| r2_bracket(scale, tol).0;
    let (lo, hi) = measured_ceiling(
        "bracket, plain tier",
        &at,
        SymRules::none(),
        tol,
        eps * 1.0e-2,
        eps * 1.0e4,
    );
    let (p_lo, p_hi) = (3.65e1 * eps, 3.80e1 * eps);
    assert!(
        lo <= p_hi && hi >= p_lo,
        "the plain bracket [{lo:e}, {hi:e}] must overlap [{p_lo:e}, {p_hi:e}] (the measured [3.70, 3.75]e1 · eps)"
    );
    let (s_lo, s_hi) = measured_ceiling(
        "bracket, shipped tier",
        &at,
        SymRules::shipped(),
        tol,
        eps * 1.0e-2,
        eps * 1.0e4,
    );
    let (b_lo, b_hi) = (3.80e2 * eps, 3.95e2 * eps);
    assert!(
        s_lo <= b_hi && s_hi >= b_lo,
        "the shipped bracket [{s_lo:e}, {s_hi:e}] must overlap [{b_lo:e}, {b_hi:e}] (the measured [3.82, 3.92]e2 · eps)"
    );
    assert!(
        s_lo > hi,
        "the shipped ceiling lies strictly above the plain one: [{s_lo:e}, {s_hi:e}] vs [{lo:e}, {hi:e}]"
    );
}
