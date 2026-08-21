//! M5 PR 10 — the blinded review's schema-break probes, ADOPTED into
//! the PR (charter C): the version door attacked at eight header
//! spellings, at both version boundaries, and at every ordering of
//! version-vs-body diagnostics.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{DocumentId, PersistError, REGENERATE_RECOURSE, SCHEMA_VERSION, load};
use geom_core::Tol;

const V1: &str = include_str!("golden/v1_golden.cad");
const V2: &str = include_str!("golden/v2_golden.cad");

fn body_of(text: &str) -> &str {
    text.split_once('\n').unwrap().1
}

/// v1 refuses SchemaTooOld with the pinned message, recourse once.
#[test]
fn review_v1_refuses_with_the_exact_message() {
    let err = load(V1, Tol::witness()).expect_err("v1 must refuse");
    let msg = err.to_string();
    assert!(matches!(
        err,
        PersistError::SchemaTooOld {
            found: 1,
            missing: 1,
            ..
        }
    ));
    assert!(
        msg.contains(&format!(
            "schema v1 is older than this build reads (supported v{SCHEMA_VERSION})"
        )),
        "{msg}"
    );
    assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
}

/// Version 0: refused, and NOT conflated with too-old (it is
/// UnknownSchema — v0 never existed). Note the message wording.
#[test]
fn review_version_zero_refuses() {
    let text = format!("schema: 0\n{}", body_of(V2));
    let err = load(&text, Tol::witness()).expect_err("v0 must refuse");
    println!("v0 -> {err:?}: {err}");
    assert!(matches!(err, PersistError::UnknownSchema { found: 0, .. }));
}

/// One past the live version (too new): UnknownSchema naming both
/// versions, never SchemaTooOld, never a body parse.
#[test]
fn review_version_three_is_too_new_even_with_garbage_body() {
    let next = u64::from(SCHEMA_VERSION) + 1;
    let text = format!("schema: {next}\n%%% not json %%%\n");
    let err = load(&text, Tol::witness()).expect_err("a too-new schema must refuse");
    assert!(
        matches!(err, PersistError::UnknownSchema { found, newest }
            if found == next && newest == SCHEMA_VERSION),
        "{err:?}"
    );
}

/// Garbage headers refuse at the header door.
#[test]
fn review_garbage_headers_refuse() {
    for h in [
        "",
        "schema:2",
        "schema:  2",
        "schema: 02",
        "schema: +2",
        "schema: 2 ",
        "SCHEMA: 2",
        "schema: two",
        "schema: 18446744073709551616",
    ] {
        let text = format!("{h}\n{}", body_of(V2));
        let err = load(&text, Tol::witness()).expect_err("must refuse");
        assert!(
            matches!(err, PersistError::Header { .. }),
            "{h:?} -> {err:?}"
        );
    }
}

/// v1 header over a V2 BODY: the version door fires first —
/// too-old beats any body consideration.
#[test]
fn review_v1_header_with_v2_body_is_too_old() {
    let text = format!("schema: 1\n{}", body_of(V2));
    assert!(matches!(
        load(&text, Tol::witness()),
        Err(PersistError::SchemaTooOld { found: 1, .. })
    ));
}

/// v1 header over a BROKEN body: still the version diagnostic.
#[test]
fn review_v1_header_with_broken_body_is_too_old() {
    let text = "schema: 1\n{\"snapshot\": [1,2,\n";
    assert!(matches!(
        load(text, Tol::witness()),
        Err(PersistError::SchemaTooOld { found: 1, .. })
    ));
}

/// The CONVERSE: a LIVE-version file with a broken body gets the BODY
/// diagnostic (Parse), not a version arm.
#[test]
fn review_v2_header_with_broken_body_gets_the_body_diagnostic() {
    // The v5 header carries the id line; the probe's subject is the
    // BODY diagnostic, so the header is well-formed.
    let id = DocumentId::derive("review-broken-body");
    let text = format!("schema: {SCHEMA_VERSION}\nid: {id}\n{{\"snapshot\": [1,2,\n");
    let text = text.as_str();
    let err = load(text, Tol::witness()).expect_err("must refuse");
    assert!(matches!(err, PersistError::Parse { .. }), "{err:?}");
}

/// **The break's honest edge, pinned.** A live-version header over
/// the v1 golden's BODY loads clean — because neither break changed
/// the wire FORMAT, only the recipe vocabulary, so an old body is
/// valid JSON under today's types.
///
/// That is inherent to a version break with no format change, and no
/// door can close it: the header is the only place the version is
/// recorded, so a hand-edited header IS a live file by definition. It
/// is not a hole in the version door — the door refuses every file
/// that still SAYS its old version — and it costs nothing here,
/// because the v1/v2 goldens carry no construct today rejects. Stated
/// so nobody reads the break as stronger than it is.
///
/// M6-5 sharpens the edge without closing it: a v2 body that DOES
/// carry a `Fillet` node fails under a v3 header, because the node
/// now requires `selection` and `deny_unknown_fields` admits no
/// default. So the hand-edit trick works exactly where the old body
/// happens to contain nothing the new vocabulary changed — which is a
/// statement about that file, not about the door.
#[test]
fn review_a_hand_edited_v2_header_over_a_v1_body_refuses_since_v4() {
    // Through v3 this probe LOADED: the breaks were vocabulary-growth
    // breaks and an old body happened to contain nothing the new
    // vocabulary changed. The v4 break (LIB-SWITCH §4h) changed the
    // PROFILE PAYLOAD's wire shape itself — stored vertices/bulges
    // died with the representation — so a hand-edited header no longer
    // smuggles an old body: the parse door refuses the retired
    // `vertices` field. The edge this probe documented is CLOSED for
    // profile-bearing files, and the refusal is typed.
    let id = DocumentId::derive("review-hand-edited-header");
    let text = format!("schema: {SCHEMA_VERSION}\nid: {id}\n{}", body_of(V1));
    match load(&text, Tol::witness()) {
        Err(PersistError::Parse { message, .. }) => assert!(
            message.contains("vertices"),
            "the refusal names the retired field: {message}"
        ),
        other => panic!(
            "a v1 body under a v4 header must refuse at parse (the profile wire shape \
             changed), got {other:?}"
        ),
    }
}
