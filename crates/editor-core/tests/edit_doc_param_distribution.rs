//! **The annotation door for a document parameter** — the third field
//! of a `DocParam::Continuous` declaration, and the edit that writes
//! it while carrying the rest forward.
//!
//! The declaration has four fields; two already have a narrow edit
//! (`SetDocParamValue` through `DocParam::with_value`,
//! `SetDocParamUnit` through `DocParam::with_display_unit`) and this
//! is the third. Without this door the only way to add or change an
//! E1/E2 annotation is `SetDocParam`, which is create-or-replace: the
//! authoring spelling for an annotated parameter,
//! `DocParam::continuous_with`, writes the CANONICAL notation, so a
//! parameter authored in millimetres reverts to metres the moment
//! anyone annotates it — with no refusal and no diagnostic. The first
//! row below is that fact, and every row after it is the door that
//! avoids it.
//!
//! The mirror-image trap — re-spelling a notation through
//! create-or-replace and losing the annotation — is pinned in
//! `edit_doc_param_unit.rs`, whose `annotating_through_create_or_
//! replace_reverts_the_notation` is this row's own finding written
//! from the other side. These rows cross-cite rather than restate it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/doc.rs",
    "crates/editor-core/src/edit.rs",
    "crates/editor-core/src/distribution.rs",
    "crates/editor-core/src/persist/check.rs",
];

use editor_core::{
    Dimension, Distribution, DocEdit, DocParam, DocumentId, ParamName, ProfileDoc, UnitSym, apply,
};
use geom_core::Tol;
use quantity::{MM, WrittenLength};

fn p(name: &str) -> ParamName {
    ParamName::new(name)
}

fn mm() -> UnitSym {
    UnitSym::from_def(&MM.def())
}

fn sigma() -> Distribution {
    Distribution::Normal { sigma: 1e-5 }
}

/// The notation a parameter is written in.
fn notation(doc: &ProfileDoc, name: &str) -> UnitSym {
    match doc.params()[&p(name)] {
        DocParam::Continuous { display_unit, .. } => display_unit,
        DocParam::Count { .. } => panic!("{name} is continuous"),
    }
}

/// A document declaring `wall` — a Length authored in MILLIMETRES,
/// unannotated — plus an annotated `bore` and a `ribs` Count.
fn fixture() -> ProfileDoc {
    let mut doc = ProfileDoc::empty(
        DocumentId::derive("edit-doc-param-distribution"),
        Tol::witness(),
    );
    for (name, value) in [
        (
            "wall",
            DocParam::written_length(WrittenLength::in_unit(3.0, MM)),
        ),
        (
            "bore",
            DocParam::continuous_with(Dimension::Length, 0.01, sigma()),
        ),
        ("ribs", DocParam::Count { value: 4 }),
    ] {
        doc = apply(
            &doc,
            &DocEdit::SetDocParam {
                name: p(name),
                value,
            },
            Tol::witness(),
        )
        .expect("the fixture parameters are valid")
        .doc;
    }
    doc
}

/// **The row this door exists for.** Annotating a standing parameter
/// keeps the rest of its declaration — here, the notation it was
/// authored in.
#[test]
fn annotating_a_standing_parameter_keeps_its_notation() {
    let before = fixture();
    assert_eq!(
        notation(&before, "wall"),
        mm(),
        "the fixture parameter is written in millimetres to begin with"
    );
    let DocParam::Continuous { dim, value, .. } = before.params()[&p("wall")] else {
        panic!("wall is continuous")
    };
    let after = apply(
        &before,
        &DocEdit::SetDocParam {
            name: p("wall"),
            value: DocParam::continuous_with(dim, value, sigma()),
        },
        Tol::witness(),
    )
    .expect("the annotation applies")
    .doc;
    assert!(
        after.params()[&p("wall")]
            .distribution()
            .is_some_and(|d| d.bit_eq(&sigma())),
        "the annotation landed"
    );
    assert_eq!(
        notation(&after, "wall"),
        mm(),
        "and the notation the parameter was authored in survived it"
    );
}
