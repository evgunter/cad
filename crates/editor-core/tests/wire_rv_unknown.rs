//! **An unknown field inside a persisted chain program refuses TYPED
//! at the real load door**, which is where `deny_unknown_fields` earns
//! its place now that it sits on `ProgramStep` / `ProgramArcData` /
//! `LoopProgram` / `ProfileProgram` rather than on deleted wire
//! mirrors. Adopted from the review's probe branch (lane `wire-rv`,
//! PR #2738), which is why its rows carry that prefix.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{PersistError, ProfileDoc, REGENERATE_RECOURSE, load, save};
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
/// the ε line is re-stamped the way `corpus/die_composed_tour.rs` and
/// the viewer's fixture rows re-stamp theirs.
fn doctored(anchor: &str, injected: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("corpus")
        .join("die_tool.pncad");
    let raw = std::fs::read_to_string(path).expect("readable");
    let raw = restamped_at_process_epsilon(&raw);
    let (header, body) = raw.split_once('\n').expect("a header line");
    let at = body.find(anchor).expect("the anchor is in the corpus");
    let open = at + anchor.len();
    format!("{header}\n{}{}{}", &body[..open], injected, &body[open..])
}

/// `text` with its one `"epsilon":` line replaced by the line the
/// persistence layer writes for the process's own ε.
fn restamped_at_process_epsilon(text: &str) -> String {
    let tol = Tol::witness();
    let probe = save(
        &ProfileDoc::empty_derived("wire-rv-epsilon-probe", tol),
        &[],
        tol,
    )
    .expect("an empty document saves");
    let is_epsilon = |line: &str| line.trim_start().starts_with("\"epsilon\":");
    let wanted = probe
        .lines()
        .find(|line| is_epsilon(line))
        .expect("a saved document records its ε");
    assert_eq!(
        text.lines().filter(|l| is_epsilon(l)).count(),
        1,
        "the die_tool document carries exactly one ε line"
    );
    let mut out: String = text
        .lines()
        .map(|line| if is_epsilon(line) { wanted } else { line })
        .collect::<Vec<&str>>()
        .join("\n");
    out.push('\n');
    out
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
