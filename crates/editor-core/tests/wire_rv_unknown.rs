//! REVIEW PROBE (lane `wire-rv`, PR #2738): an unknown field inside a
//! persisted chain program still refuses TYPED at the real load door,
//! after `deny_unknown_fields` moved from the deleted wire mirrors onto
//! `ProgramStep` / `ProgramArcData` / `LoopProgram`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{PersistError, REGENERATE_RECOURSE, load};
use geom_core::Tol;

/// `tests/corpus/die_tool.pncad` with its `schema:`/`id:` header line
/// kept and one extra key injected into the named-field object the
/// caller picks.
fn doctored(anchor: &str, injected: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("corpus")
        .join("die_tool.pncad");
    let raw = std::fs::read_to_string(path).expect("readable");
    let (header, body) = raw.split_once('\n').expect("a header line");
    let at = body.find(anchor).expect("the anchor is in the corpus");
    let open = at + anchor.len();
    format!("{header}\n{}{}{}", &body[..open], injected, &body[open..])
}

#[test]
fn wire_rv_an_unknown_field_in_an_arc_spec_refuses_typed() {
    let text = doctored("\"Bulge\": {", "\"bogus_spec_field\": 1,");
    match load(&text, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            let msg = err.to_string();
            assert!(msg.contains("bogus_spec_field"), "{msg}");
            assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
        }
        other => panic!("an unknown arc-spec field did not refuse typed: {other:?}"),
    }
}

#[test]
fn wire_rv_an_unknown_field_in_a_loop_program_refuses_typed() {
    let text = doctored("\"Chain\": [", "");
    // The unmodified doctoring is a control: with nothing injected the
    // document must still load, so a red above is the injection and not
    // the doctoring.
    load(&text, Tol::witness()).expect("the control document still loads");

    let text = doctored("\"Profile\": {", "\"bogus_profile_field\": 1,");
    match load(&text, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            let msg = err.to_string();
            assert!(msg.contains("bogus_profile_field"), "{msg}");
        }
        other => panic!("an unknown profile field did not refuse typed: {other:?}"),
    }
}

/// The step level: `ProgramStep`'s only named-field variants are the
/// three fused arc verbs and `Toward`, and the checked-in corpus
/// carries none of them, so this one goes through serde directly.
#[test]
fn wire_rv_an_unknown_field_in_a_step_refuses() {
    let json = r#"{"Toward":{"dx":{"Literal":{"value":1.0,"dim":"Scalar","unit":""}},
                   "dy":{"Literal":{"value":0.0,"dim":"Scalar","unit":""}},"dz":1}}"#;
    let err = serde_json::from_str::<editor_core::ProgramStep>(json)
        .expect_err("an unknown step field is refused");
    assert!(err.to_string().contains("dz"), "{err}");
}
