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

/// **The shipped set is A0 + rule C in the early walk, and nothing
/// else** — one default, carried by `SymbolicDials::default()` and
/// `with_session` alike. Pinned as the measured decision it is: A/B
/// over the top residual add no discharge on the documents; A/B per
/// node are minutes per replay on the BigInt ring.
#[test]
fn m10_8_the_shipped_set_is_a0_and_rule_c() {
    let s = SymRules::shipped();
    assert_eq!(SymRules::default(), s, "one default");
    assert!(s.const_fold, "A0 ships");
    assert!(s.early && s.signed_root, "rule C ships, in the early walk");
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
/// form and never less** — the bracket at a box M10-7's tier cannot
/// certify whole: more symbolic decisions, fewer numeric ones, at least
/// as many leaves certified.
#[test]
fn m10_8_the_shipped_set_discharges_more_on_the_bracket() {
    let tol = Tol::witness();
    let doc = r2_bracket(1.0e-7, tol).0;
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

/// **The two-hole plate's ceiling**: M10-7's `7.81e-7` is the PLAIN
/// tier's number at the default epsilon and `7.81e2 · ε` at every row
/// (the ceiling is the numeric channel's and scales with the band;
/// measured `7.8e-4`, `7.8e-7`, `7.8e-10` at `1e-6`, `1e-9`, `1e-12`).
/// Under the shipped set it is `K_PLATE_SHIPPED · ε` — PLATE_SENTENCE
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
    let shipped = 7.81e2 * eps;
    assert!(
        s_lo <= shipped && shipped <= s_hi,
        "the shipped bracket [{s_lo:e}, {s_hi:e}] must contain {shipped:e} (K_PLATE_SHIPPED · eps)"
    );
    assert!(
        s_lo >= lo,
        "the shipped tier never certifies less than the plain one"
    );
}

/// **The filleted bracket's ceiling MOVES**: M10-7's "factor exactly
/// 1.0" (tier on == off) held because M10-7's tier reached nothing on
/// this document; the PLAIN tier certifies the bracket whole below
/// `K_BRACKET_PLAIN · ε` and the shipped tier below `K_BRACKET_SHIPPED
/// · ε` — a factor of BRACKET_FACTOR, from the constant fold alone
/// (`sqrt(1)^k` and `sqrt` of exact-square dyadics froze every rim
/// form). Both brackets are asserted at both ends.
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
    let plain = 37.4 * eps;
    assert!(
        lo <= plain && plain <= hi,
        "the plain bracket [{lo:e}, {hi:e}] must contain {plain:e} (K_BRACKET_PLAIN · eps)"
    );
    let (s_lo, s_hi) = measured_ceiling(
        "bracket, shipped tier",
        &at,
        SymRules::shipped(),
        tol,
        eps * 1.0e-2,
        eps * 1.0e4,
    );
    let shipped = 388.0 * eps;
    assert!(
        s_lo <= shipped && shipped <= s_hi,
        "the shipped bracket [{s_lo:e}, {s_hi:e}] must contain {shipped:e} (K_BRACKET_SHIPPED · eps)"
    );
    assert!(
        s_lo > hi,
        "the shipped ceiling lies strictly above the plain one: [{s_lo:e}, {s_hi:e}] vs [{lo:e}, {hi:e}]"
    );
}
