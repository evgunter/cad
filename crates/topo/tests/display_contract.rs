//! **The Display contract for topo's façade-carried refusals**
//! (#1111): a consumer renders a refusal through the kernel's own
//! words rather than composing a sentence about somebody else's
//! failure, so every arm must state what happened in prose — and must
//! never read as the `Debug` struct dump.
//!
//! The variant identifier and the field-name punctuation are the
//! dump's fingerprints; asserting their ABSENCE is what keeps a future
//! `write!(f, "{self:?}")` from passing these tests.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Indeterminate, MarginDiag};
use topo::readback::{DanglingRef, ReadbackError};
use topo::{ContactRefusal, EdgeKey, EntityId, FaceKey, GeomRef, ReplaceFaceError, SurfaceKey};

/// An in-band margin with a named predicate — the shape a contact
/// refusal actually carries out of the verification ladder.
fn in_band() -> Indeterminate {
    Indeterminate {
        margin: MarginDiag::Value(3e-11),
        band: Band::new(1e-12, 1e-9).expect("zero < escalate"),
        predicate: Some("side_of_plane"),
    }
}

/// Asserts the F6 shape over one rendering: the wanted content is
/// present, no variant identifier leaks, no Debug punctuation, and the
/// sentence is not simply the dump.
fn assert_f6<E: core::fmt::Debug + core::fmt::Display>(err: &E, wants: &[&str], dumps: &[&str]) {
    let shown = err.to_string();
    for want in wants {
        assert!(
            shown.contains(want),
            "{err:?} renders as {shown:?}, missing {want:?}"
        );
    }
    for dump in dumps {
        assert!(
            !shown.contains(dump),
            "{err:?} renders as {shown:?} — that is the variant name, i.e. a struct dump"
        );
    }
    assert!(
        !shown.contains('{') && !shown.contains("diag:") && !shown.contains("what:"),
        "{err:?} renders as {shown:?} — that is Debug punctuation, not a sentence"
    );
    assert_ne!(shown, format!("{err:?}"));
}

/// Every arm names the contact situation and carries the TWO-arm
/// menu — except `NotCertifiable`, where a declaration cannot move the
/// configuration into the certifiable set and the menu would be a
/// false lead.
#[test]
fn contact_refusal_display_names_its_content_not_its_struct() {
    let dumps = ["Contradicted", "Escalated", "Undeclared", "NotCertifiable"];
    let cases = [
        (
            ContactRefusal::Contradicted {
                diag: in_band(),
                steer: None,
            },
            vec!["contradicted", "side_of_plane", topo::CONTACT_RECOURSE],
        ),
        (
            ContactRefusal::Escalated { diag: in_band() },
            vec!["escalated", "side_of_plane", topo::CONTACT_RECOURSE],
        ),
        (
            ContactRefusal::Undeclared { diag: in_band() },
            vec!["touch", "declaration", topo::CONTACT_RECOURSE],
        ),
        (
            ContactRefusal::NotCertifiable {
                what: "the supports meet at no definite angle",
            },
            vec!["certifiable set", "the supports meet at no definite angle"],
        ),
    ];
    for (err, wants) in cases {
        assert_f6(&err, &wants, &dumps);
    }
    // A `Fit` steer rides the contradiction rather than replacing the
    // menu: the deferral is extra steering, not the recourse. Its own
    // sentence names the `Fit { gap }` variant, so this arm is checked
    // for content and dumps but not for the brace fingerprint — the
    // brace is prose here, and the check that matters is that the
    // rendering is still not the `Debug` dump.
    let steered = ContactRefusal::Contradicted {
        diag: in_band(),
        steer: Some(topo::FIT_DEFERRAL),
    };
    let shown = steered.to_string();
    for want in [topo::CONTACT_RECOURSE, topo::FIT_DEFERRAL] {
        assert!(shown.contains(want), "{shown:?} is missing {want:?}");
    }
    for dump in dumps {
        assert!(!shown.contains(dump), "{shown:?} leaks the variant name");
    }
    assert_ne!(shown, format!("{steered:?}"));
    // The bare `Indeterminate` Display ends in the three-arm
    // coincidence sentence, whose "lower the tolerance" arm is wrong at
    // a contact site. Composing the payload instead is what keeps it
    // out.
    assert!(
        !ContactRefusal::Undeclared { diag: in_band() }
            .to_string()
            .contains("lower the tolerance")
    );
}

/// The two `Dangling` lanes read as different facts — a stale or
/// foreign handle versus a geometry reference dangling inside the
/// body — and the keys render through the crate's own noun functions.
#[test]
fn readback_error_display_names_its_content_not_its_struct() {
    let dumps = ["Dangling", "NoCanonicalFrame", "NoCarrier"];
    let cases = [
        (
            ReadbackError::Dangling {
                what: DanglingRef::Entity(EntityId::Face(FaceKey::default())),
            },
            vec!["read-back", "face", "stale", "lineage"],
        ),
        (
            ReadbackError::Dangling {
                what: DanglingRef::Geometry(GeomRef::Surface(SurfaceKey::default())),
            },
            vec!["read-back", "surface", "live entity"],
        ),
        (
            ReadbackError::NoCanonicalFrame { carrier: "NURBS" },
            vec!["NURBS", "no canonical frame", "convention"],
        ),
        (
            ReadbackError::NoCarrier,
            vec!["scaffolding", "at rest", "reach rest"],
        ),
    ];
    for (err, wants) in cases {
        assert_f6(&err, &wants, &dumps);
    }
}

/// **`TogetherEdgeDisagreement`'s sentence is true at every meter that
/// raises it** (VERBS-RIMCAP fix pass). THREE sites raise the variant:
/// `offset_together_edge_agreement` and `offset_axial_edge_agreement`
/// meter an independently solved ENDPOINT against the edge's carrier,
/// while `offset_axial_edge_on_surface` meters the minted carrier's
/// own MIDPOINT against a moved surface — no endpoint pair is compared
/// there at all. The pre-fix text asserted the endpoint mechanism
/// unconditionally ("two ends were solved {gap} m apart — the far
/// corner's own solve did not land on the carrier…"), which was FALSE
/// at the midpoint meter — the very site the sphere lune raised
/// through, and the sentence that originally misled `torax_axial`'s
/// module doc. The errors are CONSTRUCTED with each site's payload
/// (the payloads mean different lengths: an endpoint's miss off the
/// carrier; the lune's measured midpoint residual `gap = t = 0.05`)
/// and the rendering is pinned to carry the payload's own fields and
/// to name BOTH mechanisms rather than asserting one of them for all
/// three sites.
#[test]
fn together_edge_disagreement_display_is_true_at_all_three_meters() {
    // As `offset_axial_edge_agreement` (param_on) raises it: the
    // endpoint's distance off the minted carrier.
    let endpoint_meter = ReplaceFaceError::<f64>::TogetherEdgeDisagreement {
        edge: EdgeKey::default(),
        gap: 1.25e-9,
    };
    // As `offset_axial_edge_on_surface` raises it: the carrier
    // midpoint's residual to a moved surface — the lune's old door,
    // gap = the whole wall thickness.
    let midpoint_meter = ReplaceFaceError::<f64>::TogetherEdgeDisagreement {
        edge: EdgeKey::default(),
        gap: 0.05,
    };
    for (err, gap) in [(endpoint_meter, "1.25e-9"), (midpoint_meter, "0.05")] {
        // The payload's own fields render, no struct dump, and the
        // sentence covers the endpoint AND the carrier-off-surface
        // mechanisms.
        assert_f6(
            &err,
            &["carrier", "moved surface", "endpoint", "midpoint", gap],
            &["TogetherEdgeDisagreement"],
        );
        // The wrong mechanism stays gone: a sentence that asserts the
        // endpoint story unconditionally is false at the midpoint
        // meter, one of this variant's own raising sites.
        let shown = err.to_string();
        assert!(
            !shown.contains("two ends") && !shown.contains("far corner"),
            "the Display asserts the endpoint mechanism for every site again: {shown:?}"
        );
    }
}
