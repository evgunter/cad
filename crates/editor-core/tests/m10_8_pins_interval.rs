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

/// **The shipped set reaches straight geometry only through the
/// DECISION READ, and takes no theorem there.** The M10-3 slab has no
/// `sqrt` of a constant and no `sqrt` of a square to fold, so every
/// FORM-level rule is inert on it and was the whole story until the
/// read shipped (DECIDE-3): the slab's conditioning floors are
/// `max`/`min` of constants the form does not settle
/// (`work/decide/a0-leaves-max-and-min-of-constants-opaque`), and the
/// read settles eight of them over the leaf's box.
///
/// So the claim this row makes is the one that was ever load-bearing:
/// **`symbolic_zero` is unmoved and the eight come out of `numeric`**,
/// not out of the theorems. A read that re-labelled a theorem would
/// show here as `symbolic_zero` falling, which is the direction the
/// receipt may never move in; the byte-identity with `none` that used
/// to stand in for it cannot tell the two apart, and says "no gain"
/// where a gain is what happened.
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
        "decisions symbolic_zero=482 numeric=255 frozen=0 sign_gated=8",
        "the same 482 THEOREMS, and eight of the 263 numeric refusals answered by the \
         decision read as gated reads — no theorem re-labelled"
    );
    assert_eq!(
        run(SymRules::without_the_reads()),
        plain,
        "and with the read shut the shipped tier is M10-7's on straight geometry, bit for bit"
    );
}

/// **On curved geometry the shipped set discharges MORE than the plain
/// form and never less** — the bracket at `1e2 · ε` of its study, a box
/// between the plain tier's ceiling (`3.7e1 · ε`) and the shipped
/// one's (`3.9e2 · ε`). The two drives do not split alike (the shipped
/// tier certifies the box in one leaf where the plain tier splits and
/// refuses), so the comparison is per DECISION — the symbolic SHARE of
/// all decisions — and per box: at least as many leaves certified, at
/// every ε row.
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
    let share = |c: &geom_core::SymCounts| {
        (c.symbolic_zero + c.sign_gated) as f64 / c.decisions().max(1) as f64
    };
    println!(
        "   shipped {shipped:?} certified {shipped_cert} share {:.3}; plain {plain:?} certified {plain_cert} share {:.3}",
        share(&shipped),
        share(&plain)
    );
    assert!(
        share(&shipped) > share(&plain),
        "the shipped tier discharges a larger share of its decisions: {shipped:?} vs {plain:?}"
    );
    assert!(
        shipped_cert >= plain_cert,
        "and certifies at least as much of the box"
    );
    assert!(
        shipped_cert >= 1,
        "the box between the two ceilings certifies under the shipped tier"
    );
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

/// **The two-hole plate's ceiling under the plain tier and under A0
/// alone**: M10-7's `7.81e-7` is `7.81e2 · ε` at every row (the
/// ceiling is the numeric channel's and scales with the band; measured
/// `7.8e-4`, `7.8e-7`, `7.8e-10` at `1e-6`, `1e-9`, `1e-12`) under
/// both — the constant fold reaches none of the plate's arc residuals.
/// What moves it is the form-level algebra on top of the door
/// (`m10_10_pins_interval`). Both brackets are asserted at both ends
/// and must contain the same `7.81e2 · ε`.
#[test]
fn m10_8_the_plate_ceiling_under_the_plain_tier_and_a0_alone() {
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
    let (s_lo, s_hi) = measured_ceiling("plate, A0 alone", &at, a0_alone(), tol, eps, eps * 1.0e6);
    assert!(
        s_lo <= m10_7 && m10_7 <= s_hi,
        "the A0 bracket [{s_lo:e}, {s_hi:e}] must contain the same {m10_7:e}: unmoved"
    );
    assert!(s_lo >= lo, "A0 never certifies less than the plain tier");
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
