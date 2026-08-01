//! M5 PR 10 — the blinded review's schema-break probes, ADOPTED into
//! the PR (charter C): the version door attacked at eight header
//! spellings, at both version boundaries, and at every ordering of
//! version-vs-body diagnostics.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{PersistError, REGENERATE_RECOURSE, SCHEMA_VERSION, load};

const V1: &str = include_str!("golden/v1_golden.cad");
const V2: &str = include_str!("golden/v2_golden.cad");

fn body_of(text: &str) -> &str {
    text.split_once('\n').unwrap().1
}

/// v1 refuses SchemaTooOld with the pinned message, recourse once.
#[test]
fn review_v1_refuses_with_the_exact_message() {
    let err = load(V1).expect_err("v1 must refuse");
    let msg = err.to_string();
    assert!(matches!(
        err,
        PersistError::SchemaTooOld {
            found: 1,
            supported: 2,
            missing: 1
        }
    ));
    assert!(
        msg.contains("schema v1 is older than this build reads (supported v2)"),
        "{msg}"
    );
    assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
}

/// Version 0: refused, and NOT conflated with too-old (it is
/// UnknownSchema — v0 never existed). Note the message wording.
#[test]
fn review_version_zero_refuses() {
    let text = format!("schema: 0\n{}", body_of(V2));
    let err = load(&text).expect_err("v0 must refuse");
    println!("v0 -> {err:?}: {err}");
    assert!(matches!(err, PersistError::UnknownSchema { found: 0, .. }));
}

/// Version 3 (too new): UnknownSchema naming both versions, never
/// SchemaTooOld, never a body parse.
#[test]
fn review_version_three_is_too_new_even_with_garbage_body() {
    let text = "schema: 3\n%%% not json %%%\n";
    let err = load(text).expect_err("v3 must refuse");
    assert!(
        matches!(err, PersistError::UnknownSchema { found: 3, newest }
            if newest == SCHEMA_VERSION),
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
        let err = load(&text).expect_err("must refuse");
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
        load(&text),
        Err(PersistError::SchemaTooOld { found: 1, .. })
    ));
}

/// v1 header over a BROKEN body: still the version diagnostic.
#[test]
fn review_v1_header_with_broken_body_is_too_old() {
    let text = "schema: 1\n{\"snapshot\": [1,2,\n";
    assert!(matches!(
        load(text),
        Err(PersistError::SchemaTooOld { found: 1, .. })
    ));
}

/// The CONVERSE: a v2 file with a broken body gets the BODY
/// diagnostic (Parse), not a version arm.
#[test]
fn review_v2_header_with_broken_body_gets_the_body_diagnostic() {
    let text = "schema: 2\n{\"snapshot\": [1,2,\n";
    let err = load(text).expect_err("must refuse");
    assert!(matches!(err, PersistError::Parse { .. }), "{err:?}");
}

/// **The break's honest edge, pinned.** A v2 header over the v1
/// golden's BODY loads clean — because v1 → v2 changed the recipe
/// VOCABULARY, not the wire format, so a v1 body is valid v2 JSON.
///
/// That is inherent to a version break with no format change, and no
/// door can close it: the header is the only place the version is
/// recorded, so a hand-edited header IS a v2 file by definition. It is
/// not a hole in the version door — the door refuses every file that
/// still SAYS v1 — and it costs nothing, because a v1 body carries no
/// construct v2 rejects. Stated so nobody reads the break as stronger
/// than it is.
#[test]
fn review_a_hand_edited_v2_header_over_a_v1_body_loads() {
    let text = format!("schema: 2\n{}", body_of(V1));
    match load(&text) {
        Ok(_) => {}
        Err(PersistError::ToleranceConflict { .. }) => {
            // The eps door is LAST: the bytes still parsed, validated
            // and replayed, which is the whole claim here.
        }
        Err(e) => panic!(
            "a v1 body under a v2 header no longer loads ({e:?}) — the format DID change: \
             say so in the schema docs and give v2 a real migration story"
        ),
    }
}
