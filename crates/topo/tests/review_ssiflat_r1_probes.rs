//! Independent-review probes for VERBS-SSIFLAT (PR #931), kept for
//! regression value. The PR's own row pins the MARGIN PAYLOAD
//! (`Some(Enclosure)` with both endpoints bit-exact); the row here pins
//! what the PR argued but did not pin: the four margin shapes are
//! pairwise distinguishable in Display — a poisoned margin says so in
//! words, an enclosure carries two endpoints, a value one, a hole none
//! — red if two shapes ever collapse into one rendering again (the #925
//! conflation, one layer up).
//!
//! Two probes are retired. Probe 1 (an interval-lane `ssi_hull_sup`
//! escalation renders both enclosure endpoints) reached its escalation
//! arm only at a process ε under the hull bound, which on the described
//! carrier is about `5e-15`, below every CI ε;
//! `review_ssiflat_r2_probes`' probe 2 pins the same legibility at an
//! explicit band, at any ε. Probe 3 (the `f64` lane's hull bound sits
//! strictly under the interval lane's) stood on the interval lane
//! reading `1.02e-12`, which was the width of a ring refinement taken in
//! the projective form; on the described carrier both lanes read about
//! `4e-15` to `5e-15`, and the inequality states nothing.

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
