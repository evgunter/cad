use super::PcurveMintError;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, LoopKey};
use geom_brep::PcurveCertifyError;
use geom_core::predicate::{Band, BandError, COINCIDENCE_RECOURSE};
use geom_core::{Indeterminate, MarginDiag};

/// **The recourse claim for the carrier tier 3 renders whole.**
/// `ValidationError::Pcurve { finding }` is literally
/// `"tier 3: {finding}"`, and four more wrappers (`ShellError`,
/// `TransformError`, `ReplaceFaceError`, `MergeCoplanarError`)
/// contribute a phrase each, so whatever this enum fails to say is
/// absent from the message a user reads at five doors.
///
/// **Three arms are asserted differently, because they render a
/// carrier whole and contribute no prose of their own.** The rule
/// is that an arm is checked TRANSITIVELY only where the carrier
/// has an enforcement row of its own:
///
/// - `Escalated` carries an `Indeterminate`, whose `Display` ends
///   in [`COINCIDENCE_RECOURSE`] on every margin arm, so the
///   recourse is asserted directly.
/// - `Band` carries a `BandError`, whose own arms are covered by
///   `geom_core`'s `every_band_error_arm_names_a_recourse`; what is
///   asserted here is the delegation itself — that the arm renders
///   the carrier whole rather than summarising it.
/// - `Certify` carries a `PcurveCertifyError`, which has **no such
///   row**, so the chain's claim is unproved at that hop and this
///   row does not pretend otherwise: it asserts the delegation and
///   nothing about the recourse.
///
/// `Stale` is the caller's own key, and states the fact with no
/// recourse and no defect claim (D2 row 4), which is what it is held
/// to.
///
/// **A floor, not a proof**, on the terms
/// `every_chart_region_arm_names_a_recourse` states: a vocabulary
/// check cannot tell a recourse from a sentence containing one of
/// its words, and an arm whose recourse uses a word not listed
/// fails it honestly — extend the list in the same change. What it
/// catches is the arm added with no second clause at all. The
/// payloads below are keys, which render verb-free, so what the row
/// measures is the variant's own clause.
#[test]
fn every_pcurve_mint_error_arm_names_a_recourse() {
    // A vocabulary, not a part-of-speech test: an arm that points
    // at a named lever rather than using an imperative satisfies
    // the claim the same way.
    const RECOURSE_WORDS: &[&str] = &[
        "read", "repair", "re-mint", "ask", "hold", "describe", "report",
    ];
    let cause = Indeterminate {
        margin: MarginDiag::value(5e-9),
        band: Band::new(1e-9, 1e-8).unwrap(),
        // UNDECIDED-NAME: the cause every PcurveMintError arm's recourse is read for
        predicate: Some("pcurve_recourse_probe"),
        terminal_sliver: false,
    };
    let band_error = BandError::Empty {
        zero: 1e-8,
        escalate: 1e-9,
    };
    let certify_error = PcurveCertifyError::Incidence(geom_brep::IncidenceRefusal::off_chart(
        geom::SurfaceKind::Sphere,
        geom::CurveKind::Line,
        "the line's points are not on the chart (its distance from the chart is past the band somewhere in its span)",
        None,
    ));
    let arms = [
        PcurveMintError::Stale {
            role: "half_edge",
            key: EntityId::HalfEdge(HalfEdgeKey::default()),
        },
        PcurveMintError::NoCarrier {
            half_edge: HalfEdgeKey::default(),
        },
        PcurveMintError::Certify {
            half_edge: HalfEdgeKey::default(),
            error: certify_error.clone(),
        },
        PcurveMintError::LoopDiscontinuity {
            half_edge: HalfEdgeKey::default(),
        },
        PcurveMintError::LoopNotClosed {
            face: FaceKey::default(),
        },
        PcurveMintError::JointWithoutRoom {
            face: FaceKey::default(),
            r#loop: LoopKey::default(),
            half_edge: HalfEdgeKey::default(),
        },
        PcurveMintError::MissingCache {
            half_edge: HalfEdgeKey::default(),
        },
        PcurveMintError::Unminted {
            face: FaceKey::default(),
        },
        PcurveMintError::RowInterval {
            half_edge: HalfEdgeKey::default(),
        },
        PcurveMintError::Escalated {
            half_edge: HalfEdgeKey::default(),
            cause,
        },
        PcurveMintError::Band(band_error),
        PcurveMintError::PlaceholderChart {
            face: FaceKey::default(),
        },
    ];
    assert_eq!(arms.len(), 12, "an arm was added without a row here");
    for arm in &arms {
        let msg = arm.render(geom_brep::recourse::Reading::Build);
        match arm {
            // A caller's own key states the fact: no recourse, and
            // no defect claim.
            PcurveMintError::Stale { .. } => {
                assert_eq!(test_utils::refusal::recourse_markers(&msg), 0, "{msg}");
                assert!(!msg.contains("defect") && !msg.contains("torn"), "{msg}");
            }
            PcurveMintError::Escalated { .. } => {
                assert!(msg.contains(COINCIDENCE_RECOURSE), "{msg}");
            }
            PcurveMintError::Band(_) => {
                assert!(msg.contains(&band_error.to_string()), "{msg}");
            }
            PcurveMintError::Certify { .. } => {
                assert!(
                    msg.contains(&certify_error.render(geom_brep::recourse::Reading::Build)),
                    "{msg}"
                );
            }
            _ => {
                let lower = msg.to_lowercase();
                assert!(
                    RECOURSE_WORDS.iter().any(|w| lower.contains(w)),
                    "no recourse in: {msg}"
                );
            }
        }
    }
}

/// A fitted-lane escalation (the hull limb's, the one it raises),
/// rendered through the mint's `Certify` arm, ends in its limb's own
/// labelled recourse at a build.
#[test]
fn a_fitted_hull_escalation_renders_its_own_recourse_through_the_mint() {
    let finding = PcurveMintError::Certify {
        half_edge: HalfEdgeKey::default(),
        error: PcurveCertifyError::FittedEscalated {
            decision: geom_brep::SsiDecision::Limb(geom_brep::SsiLimb::HullSupChart),
            cause: Indeterminate {
                margin: MarginDiag::value(5e-9),
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("ssi_hull_sup_chart"),
                terminal_sliver: false,
            },
        },
    };
    assert_eq!(
        finding.render(geom_brep::recourse::Reading::Build),
        "the pcurve of half-edge HalfEdgeKey(null) failed certification: pcurve \
             certification: the fitted lane's certificate escalated — whether the fitted carrier \
             passes limb 2 (control-hull sup-norm bound through its pcurve) is undecided: \
             margin 5e-9 lies inside the ambiguity band (1e-9, 1e-8). Recourse: loosen the tolerance, as a last resort; this refusal may indicate a \
             kernel bug worth reporting"
    );
}
