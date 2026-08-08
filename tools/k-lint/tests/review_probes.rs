//! ADVERSARIAL REVIEW PROBES (PR #239, branch m7/klint-floor-review-probes).
//!
//! Not part of the shipped contract — these pin the REVIEW's findings
//! about the rule-2 cap (Finding M7-F1) and the ε-coupled allow-list
//! posture, so the blind window and the fail-loud claim stay
//! executable statements instead of prose.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use k_lint::{
    BASELINE_FLOOR_MARGIN, PROXIMITY_FACTOR, Reason, lint_sample, proximity_above_threshold,
};

/// The OLD (uncapped) rule-2 definite arm, reconstructed for
/// comparison.
fn old_rule2_flags(m: f64, band_escalate: f64) -> bool {
    m.abs() < band_escalate * PROXIMITY_FACTOR
}

/// M7-F1 blind window, characterized: at ε = 1e-6 (band_escalate
/// 1e-5), definite ε-independent margins in [4.0e-5, 1.0e-3) were
/// flagged by the old rule 2 and pass CLEAN under every new rule.
/// Rule 1 cannot see them (definite outcome), rule 3 cannot (above
/// floor), rule 4 cannot (not allow-listed).
#[test]
fn m7_f1_blind_window_at_1e6_exists_and_is_bounded() {
    let (zero, esc) = (1e-6, 1e-5);
    // Window edges: [floor, 10²·Kε).
    assert!((proximity_above_threshold(esc) - BASELINE_FLOOR_MARGIN).abs() < 1e-20);
    // Representative points: 4.5e-5 (4.5× the escalation threshold!),
    // 1e-4, 9.9e-4 — old rule flagged all three, new rules pass all
    // three, and no other rule picks them up.
    for m in [4.5e-5, 1e-4, 9.9e-4] {
        assert!(old_rule2_flags(m, esc), "old rule 2 flagged {m:e}");
        assert!(
            lint_sample("volume_backstop", m, zero, esc, "positive").is_empty(),
            "new rules pass {m:e} clean at eps=1e-6 — the blind window"
        );
    }
    // Window edges are exclusive/inclusive as claimed: just below the
    // floor both rules still fire; at/above 1e-3 the old rule was
    // already clean.
    assert_eq!(
        lint_sample("volume_backstop", 3.9e-5, zero, esc, "positive"),
        vec![Reason::NearBandAbove, Reason::BelowBaselineFloor]
    );
    assert!(!old_rule2_flags(1.0e-3, esc));
}

/// The window is EMPTY at 1e-9 and 1e-12: there the raw threshold
/// (10²·Kε = 1e-6 / 1e-9) sits below the floor, the cap is inert, and
/// old and new rule 2 agree on every margin.
#[test]
fn m7_f1_blind_window_is_empty_at_tight_rows() {
    for (zero, esc) in [(1e-9, 1e-8), (1e-12, 1e-11)] {
        assert!((proximity_above_threshold(esc) - esc * PROXIMITY_FACTOR).abs() < 1e-30);
        for m in [4.5e-5, 1e-4, 9.9e-4, 3.0e-6, 5.0e-7, 2.0e-8] {
            let reasons = lint_sample("p", m, zero, esc, "positive");
            assert_eq!(
                old_rule2_flags(m, esc),
                reasons.contains(&Reason::NearBandAbove),
                "old and new rule 2 must agree at eps={zero:e} for {m:e}"
            );
        }
    }
}

/// The allow-list fail-loud claim, attacked numerically: a NEW
/// ε-coupled predicate (recording 1024·ε − width, NOT on the
/// allow-list) with mid-range headroom is SILENT at the 1e-6 row —
/// the cap widened this silence from (1e-3, 1.024e-3] to
/// (4e-5, 1.024e-3], i.e. from ~2% to ~96% of its operating range.
/// The loudness the module docs promise is carried ENTIRELY by the
/// tighter rows, where the same physics scales the margin under the
/// floor and both metre rules fire.
#[test]
fn new_eps_coupled_predicate_is_silent_at_1e6_loud_at_tight_rows() {
    // 1e-6 row: headroom 5e-4 (mid-range, ~half of 1024·ε). Old rule 2
    // would have flagged it (5e-4 < 1e-3); the capped lint is silent.
    assert!(old_rule2_flags(5e-4, 1e-5));
    assert!(
        lint_sample("new_quad_family", 5e-4, 1e-6, 1e-5, "positive").is_empty(),
        "a NEW eps-coupled predicate mid-range at eps=1e-6 passes SILENTLY"
    );
    // The same physical family at the 1e-9 / 1e-12 rows: headroom
    // scales with ε (5e-7, 5e-10) and lands under BOTH metre rules —
    // this is where the fail-loud promise is actually kept.
    for (m, zero, esc) in [(5e-7, 1e-9, 1e-8), (5e-10, 1e-12, 1e-11)] {
        assert_eq!(
            lint_sample("new_quad_family", m, zero, esc, "positive"),
            vec![Reason::NearBandAbove, Reason::BelowBaselineFloor],
            "the tight rows must catch the new eps-coupled family"
        );
    }
}
