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

use test_utils::f6::{assert_f6, assert_f6_every_variant};

/// An in-band margin with a named predicate — the shape a contact
/// refusal actually carries out of the verification ladder.
fn in_band() -> Indeterminate {
    Indeterminate {
        margin: MarginDiag::Value(3e-11),
        band: Band::new(1e-12, 1e-9).expect("zero < escalate"),
        predicate: Some("side_of_plane"),
    }
}

test_utils::f6_variants! {
    /// `ContactRefusal`'s census: one ident per variant, feeding both
    /// the wildcard-free `match` rustc checks and the identifier roster
    /// the weld compares against the rendered cases. The mechanism and
    /// what it does NOT weld are documented on
    /// [`test_utils::f6::assert_f6_every_variant`].
    const CONTACT_REFUSAL: ContactRefusal =
        [Contradicted, Escalated, Undeclared, NotCertifiable];
}

/// Every `Debug` field name `ContactRefusal`'s payloads carry, as the
/// punctuation a dump would print — the whole payload vocabulary, not
/// the subset one row happens to construct.
///
/// **What this roster is worth, stated honestly.** It is not what
/// catches an arm that starts printing `{self:?}`: these are struct
/// variants, so a full dump carries `{`, which
/// [`test_utils::f6::assert_f6`] bans unconditionally, and it equals
/// the value's own `Debug`, which the same helper refuses. What these
/// entries buy over that is a BRACE-FREE field token in an otherwise
/// prose sentence, and they are unwelded to the enum, so a payload
/// field added to an existing variant leaves them short in silence.
const CONTACT_REFUSAL_FIELDS: &[&str] = &["diag:", "steer:", "what:"];

test_utils::f6_variants! {
    /// [`ReadbackError`]'s census — see [`CONTACT_REFUSAL`].
    const READBACK_ERROR: ReadbackError = [Dangling, NoCanonicalFrame, NoCarrier];
}

/// Every `Debug` field name [`ReadbackError`]'s payloads carry, and
/// what that is worth — see [`CONTACT_REFUSAL_FIELDS`]. `NoCarrier` is
/// a UNIT variant, so its dump carries no brace and its whole
/// fingerprint is the identifier the roster above holds.
const READBACK_ERROR_FIELDS: &[&str] = &["what:", "carrier:"];

/// Every arm names the contact situation and carries the TWO-arm
/// menu — except `NotCertifiable`, where a declaration cannot move the
/// configuration into the certifiable set and the menu would be a
/// false lead.
#[test]
fn contact_refusal_display_names_its_content_not_its_struct() {
    let cases = [
        (
            ContactRefusal::Contradicted {
                diag: in_band(),
                steer: None,
            },
            // The one reason true at every contradiction site; the
            // predicate and margin are developer detail in `Debug`.
            vec![
                "contradicted",
                topo::CONTRADICTION_REASON,
                topo::CONTRADICTION_RECOURSE,
            ],
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
    assert_f6_every_variant(&cases, &CONTACT_REFUSAL, &[], CONTACT_REFUSAL_FIELDS);
    // A `Fit` steer rides the contradiction rather than replacing the
    // recourse: the deferral is extra steering. It renders in the
    // user's words (`FIT_DEFERRAL_FOR_USERS`), never as the wire
    // sentence with its `Fit { gap }` variant name — which is the
    // brace this row now also refuses.
    let steered = ContactRefusal::Contradicted {
        diag: in_band(),
        steer: Some(topo::FIT_DEFERRAL),
    };
    let shown = steered.to_string();
    for want in [topo::CONTRADICTION_RECOURSE, topo::FIT_DEFERRAL_FOR_USERS] {
        assert!(shown.contains(want), "{shown:?} is missing {want:?}");
    }
    for dump in CONTACT_REFUSAL.identifiers() {
        assert!(!shown.contains(dump), "{shown:?} leaks the variant name");
    }
    assert!(!shown.contains(topo::FIT_DEFERRAL), "{shown:?}");
    assert!(!shown.contains('{'), "{shown:?}");
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
    let cases = [
        (
            ReadbackError::Dangling {
                what: DanglingRef::Entity(EntityId::Face(FaceKey::default())),
            },
            vec!["face", "does not resolve", "stale", "lineage"],
        ),
        (
            ReadbackError::Dangling {
                what: DanglingRef::Geometry(GeomRef::Surface(SurfaceKey::default())),
            },
            vec!["surface", "does not resolve", "live entity"],
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
    assert_f6_every_variant(&cases, &READBACK_ERROR, &[], READBACK_ERROR_FIELDS);
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
            // Deliberately ONE identifier, not an enum mirror: this row
            // is about one variant's sentence being true at three
            // raising sites, so the ban list is that variant and the
            // field roster is what its own payload would dump.
            &["TogetherEdgeDisagreement"],
            &["edge:", "gap:"],
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
