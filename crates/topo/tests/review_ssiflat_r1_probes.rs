//! Independent-review probe for VERBS-SSIFLAT (PR #931): the four
//! margin shapes a `PcurveCertifyError` can carry (a value, an
//! enclosure, a poisoned margin, no margin) render pairwise
//! distinguishably in Display. A poisoned margin says so in words, an
//! enclosure carries both endpoints, a value one, a hole none, and every
//! escalation renders the band it was judged against. Red if two shapes
//! collapse into one rendering (the #925 conflation, one layer up).
//!
//! The interval-lane `ssi_hull_sup` escalation's own payload is held by
//! `review_ssiflat_r2_probes` at an explicit band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::PcurveCertifyError;

/// Probe 2: the four margin shapes render pairwise distinguishably.
/// The #925 conflation was exactly two shapes (an honest enclosure and
/// true poison) collapsing into one rendering; this row goes red if
/// any two collapse again — at THIS layer or in a future "helpful"
/// Display rewrite.
#[test]
fn the_four_margin_shapes_render_pairwise_distinguishably() {
    use geom_core::{Band, Indeterminate, MarginDiag};
    // AMENDED (fix pass): the shapes now live behind two doors rather
    // than one `Option<MarginDiag>` field — escalations carry the
    // classifier's whole `Indeterminate` (margin AND band AND
    // predicate), definite/structural refusals carry a named
    // `FittedMagnitude` or nothing. The probe's claim is unchanged and
    // is if anything sharper: no two of these may render alike.
    let band = Band::new(1e-12, 1e-11).unwrap();
    let escalated = |margin: MarginDiag| {
        PcurveCertifyError::FittedEscalated {
            decision: geom_brep::SsiDecision::Limb(geom_brep::SsiLimb::HullSup),
            cause: Indeterminate {
                margin,
                band,
                predicate: Some("probe"),
                terminal_sliver: false,
            },
        }
        .to_string()
    };
    let value = escalated(MarginDiag::value(1.5e-12));
    let enclosure = escalated(MarginDiag::enclosure(1.5e-12, 2.5e-12));
    let poison = escalated(MarginDiag::INVALID);
    let hole = PcurveCertifyError::FittedCertificate {
        limb: None,
        what: "probe",
        magnitude: None,
    }
    .to_string();

    // The enclosure renders BOTH endpoints (a reader needs the width).
    assert!(
        enclosure.contains("1.5e-12") && enclosure.contains("2.5e-12"),
        "the enclosure must surface both endpoints: {enclosure}"
    );
    // Poison says so in words, and no honest shape claims poison.
    assert!(
        poison.contains("poison") || poison.contains("invalid"),
        "{poison}"
    );
    for honest in [&value, &enclosure, &hole] {
        assert!(
            !honest.contains("poison") && !honest.contains("NaN"),
            "an honest margin must not read as poison: {honest}"
        );
    }
    // Every escalation renders its BAND too — a margin without the band
    // it was judged against cannot be read.
    for e in [&value, &enclosure, &poison] {
        assert!(
            e.contains("1e-12") && e.contains("1e-11"),
            "an escalation must render the band it was judged against: {e}"
        );
    }
    // All four renderings are pairwise distinct.
    let all = [&value, &enclosure, &poison, &hole];
    for (i, a) in all.iter().enumerate() {
        for b in all.iter().skip(i + 1) {
            assert_ne!(a, b, "two margin shapes collapsed into one rendering");
        }
    }
}
