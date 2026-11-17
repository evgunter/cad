//! **An unknown field inside a persisted chain program refuses TYPED
//! at the real load door**, which is where `deny_unknown_fields` earns
//! its place now that it sits on `ProgramStep` / `ProgramArcData` /
//! `LoopProgram` / `ProfileProgram` rather than on deleted wire
//! mirrors. Adopted from the review's probe branch (lane `wire-rv`,
//! PR #2738), which is why its rows carry that prefix.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::Formula;
use editor_core::{PersistError, REGENERATE_RECOURSE, load};
use geom_core::Tol;

/// `tests/corpus/die_tool.pncad` with its header line kept and one
/// extra key injected into the named-field object the caller picks.
///
/// **Its ε line is re-stamped to the process's own first.** The loader
/// reconciles the file's recorded ε against the running one and
/// refuses a disagreement, and this suite runs at every CI ε row, so
/// the committed 1e-9 would refuse `ToleranceConflict` at the 1e-6 and
/// 1e-12 rows — before serde ever reached the injected field, for the
/// control, and for the wrong reason everywhere else. The document's
/// STRUCTURE is what these rows are about and it is ε-independent, so
/// the ε line is re-stamped by the re-stamp every suite shares
/// (`editor_core::test_support::restamped_at`).
fn doctored(anchor: &str, injected: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("corpus")
        .join("die_tool.pncad");
    let raw = std::fs::read_to_string(path).expect("readable");
    let raw = editor_core::test_support::restamped_at(&raw, Tol::witness());
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
    let json = r#"{"Toward":{"dx":{"Quantity":{"value":1.0,"dim":"Scalar","unit":""}},
                   "dy":{"Quantity":{"value":0.0,"dim":"Scalar","unit":""}},"dz":1}}"#;
    let err = serde_json::from_str::<editor_core::ProgramStep<Formula>>(json)
        .expect_err("an unknown step field is refused");
    assert!(err.to_string().contains("dz"), "{err}");
}
