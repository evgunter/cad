//! **The unit-only door for a document parameter** — `DocParam::
//! with_display_unit` and the `DocEdit::SetDocParamUnit` that routes
//! through it.
//!
//! A parameter's notation sits on the DECLARATION, beside `dim` and
//! `distribution`. Before this door the only way to change it was
//! `SetDocParam`, which is create-or-replace: a caller rebuilding the
//! parameter from `(dim, value)` to re-spell its unit deletes the E1/E2
//! annotation with no refusal and no diagnostic. The first row here
//! writes that trap down; every row after it is the door that avoids
//! it.
//!
//! **The reading these rows pin.** Changing a parameter's KIND is a
//! redeclaration — `with_value`'s argument, and why a value edit
//! refuses a kind change rather than performing one. Changing its
//! NOTATION is not: `DocParam::bit_eq` already excludes `display_unit`
//! as presentation metadata, the same ruling `Expr::bit_eq` makes, so a
//! unit edit changes nothing `bit_eq` sees. `bit_eq_is_blind_to_the_notation_edit`
//! is that reading executed rather than asserted in prose.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/doc.rs",
    "crates/editor-core/src/edit.rs",
    "crates/editor-core/src/expr.rs",
    "crates/editor-core/src/persist/check.rs",
];

use editor_core::{
    Dimension, Distribution, DocEdit, DocParam, DocumentId, EditError, ParamName, ProfileDoc,
    UnitSym, apply, load, save,
};
use geom_core::Tol;

fn p(name: &str) -> ParamName {
    ParamName::new(name)
}

fn mm() -> UnitSym {
    UnitSym::from_def(&quantity::MM.def())
}

fn deg() -> UnitSym {
    UnitSym::from_def(&quantity::DEG.def())
}

fn sigma() -> Distribution {
    Distribution::Normal { sigma: 1e-5 }
}

/// A document declaring `wall` (a Length carrying a normal
/// distribution), `sweep` (an Angle) and `ribs` (a Count).
fn fixture() -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive("edit-doc-param-unit"), Tol::witness());
    for (name, value) in [
        (
            "wall",
            DocParam::continuous_with(Dimension::Length, 0.003, sigma()),
        ),
        ("sweep", DocParam::continuous(Dimension::Angle, 1.5)),
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

/// **The trap, written down.** Re-spelling a parameter's notation
/// through the create-or-replace door means assembling a whole
/// `DocParam`, and the natural spelling — `DocParam::continuous(dim,
/// value)`, then the notation — names no distribution, so the
/// annotation is deleted. No refusal, no diagnostic: the edit applies.
///
/// This row is the reason the unit door exists, and it stays red-able:
/// if `SetDocParam` ever stopped being create-or-replace, the sentence
/// above would be false and this row would say so.
#[test]
fn rebuilding_a_parameter_to_re_spell_its_unit_drops_the_distribution() {
    let before = fixture();
    assert!(
        before.params()[&p("wall")].distribution().is_some(),
        "the fixture parameter is annotated to begin with"
    );
    let DocParam::Continuous { dim, value, .. } = before.params()[&p("wall")] else {
        panic!("wall is continuous")
    };
    // The caller wants millimetres and has to restate the declaration
    // to say so. This is the whole trap: `distribution` is a field
    // nobody mentioned.
    let rebuilt = match DocParam::continuous(dim, value) {
        DocParam::Continuous {
            dim,
            value,
            distribution,
            ..
        } => DocParam::Continuous {
            dim,
            value,
            display_unit: mm(),
            distribution,
        },
        DocParam::Count { value } => DocParam::Count { value },
    };
    let after = apply(
        &before,
        &DocEdit::SetDocParam {
            name: p("wall"),
            value: rebuilt,
        },
        Tol::witness(),
    )
    .expect("create-or-replace accepts it")
    .doc;
    assert_eq!(
        after.params()[&p("wall")].distribution(),
        None,
        "the create-or-replace door deleted the annotation the caller never named"
    );
}

/// **The door.** The notation moves; the dimension, the exact value and
/// the distribution ride through untouched — `with_value`'s
/// carry-forward argument over the other field.
#[test]
fn the_unit_door_carries_the_declaration_forward() {
    let before = fixture();
    let after = apply(
        &before,
        &DocEdit::SetDocParamUnit {
            name: p("wall"),
            unit: mm(),
        },
        Tol::witness(),
    )
    .expect("a unit edit on a declared Length parameter applies")
    .doc;
    match after.params()[&p("wall")] {
        DocParam::Continuous {
            dim,
            value,
            display_unit,
            distribution,
        } => {
            assert_eq!(display_unit, mm(), "the notation moved");
            assert_eq!(dim, Dimension::Length, "the dimension is the declaration's");
            assert_eq!(
                value.to_bits(),
                0.003_f64.to_bits(),
                "the value is bit-identical"
            );
            let got = distribution.expect("the annotation SURVIVED");
            assert!(got.bit_eq(&sigma()), "and survived bit for bit: {got:?}");
        }
        DocParam::Count { .. } => panic!("still continuous"),
    }
}

/// `DocParam::with_display_unit` on its own, the carry-forward in one
/// place: the same two `None`s the edit door reports as typed
/// refusals.
#[test]
fn with_display_unit_is_the_carry_forward_and_its_refusals_are_none() {
    let annotated = DocParam::continuous_with(Dimension::Length, 0.003, sigma());
    let moved = annotated
        .with_display_unit(mm())
        .expect("mm measures a Length");
    assert!(
        moved
            .distribution()
            .is_some_and(|d| d.bit_eq(&Distribution::Normal { sigma: 1e-5 })),
        "the annotation rode through"
    );
    assert_eq!(
        annotated.with_display_unit(deg()),
        None,
        "a unit that does not measure the declared dimension has no answer"
    );
    assert_eq!(
        DocParam::Count { value: 4 }.with_display_unit(mm()),
        None,
        "a count names no notation"
    );
}

/// Each refusal by name, at the edit door.
#[test]
fn the_unit_door_refuses_typed() {
    let doc = fixture();
    let refuse = |name: &str, unit: UnitSym| {
        apply(
            &doc,
            &DocEdit::SetDocParamUnit {
                name: p(name),
                unit,
            },
            Tol::witness(),
        )
        .expect_err("refused")
    };
    // Nothing to carry forward.
    assert_eq!(
        refuse("nonesuch", mm()),
        EditError::DocParamNotDeclared {
            name: p("nonesuch")
        }
    );
    // A count is a number, not a quantity.
    assert_eq!(
        refuse("ribs", mm()),
        EditError::DocParamCountHasNoUnit { name: p("ribs") }
    );
    // The unit must MEASURE the declared dimension — the pairing the
    // save/load validator refuses a document for.
    assert_eq!(
        refuse("wall", deg()),
        EditError::DocParamUnitMismatch {
            name: p("wall"),
            unit: Dimension::Angle,
            declared: Dimension::Length,
        }
    );
    // Every refusal renders as a sentence naming the parameter.
    for e in [
        refuse("nonesuch", mm()),
        refuse("ribs", mm()),
        refuse("wall", deg()),
    ] {
        let text = e.to_string();
        assert!(
            text.contains("parameter"),
            "refusal renders as prose naming the parameter: {text:?}"
        );
    }
}

/// **The reading, executed.** A notation edit changes nothing
/// `bit_eq` sees — at the parameter and at the document — because
/// `display_unit` is presentation metadata, excluded from the
/// comparison exactly as `Expr::bit_eq` excludes a literal's. That is
/// the argument for a narrow door rather than a redeclaration: a
/// redeclaration is a change to what the parameter IS, and this is not
/// one.
#[test]
fn bit_eq_is_blind_to_the_notation_edit() {
    let before = fixture();
    let after = apply(
        &before,
        &DocEdit::SetDocParamUnit {
            name: p("wall"),
            unit: mm(),
        },
        Tol::witness(),
    )
    .expect("applies")
    .doc;
    assert!(
        before.params()[&p("wall")].bit_eq(&after.params()[&p("wall")]),
        "the parameter is the same parameter"
    );
    assert!(
        before.bit_eq(&after),
        "and the document is the same document"
    );
    // The edit is NOT a no-op, though: the stored notation moved.
    assert_ne!(
        before.params()[&p("wall")],
        after.params()[&p("wall")],
        "structural equality still sees the field bit_eq excludes"
    );
}

/// The edit persists and replays like every other `DocEdit`: saved in
/// the log, replayed through `apply` on load, and the loaded document
/// carries the new notation with the annotation still on it.
#[test]
fn the_unit_edit_saves_replays_and_loads() {
    let snapshot = fixture();
    let edits = [
        DocEdit::SetDocParamUnit {
            name: p("wall"),
            unit: mm(),
        },
        DocEdit::SetDocParamUnit {
            name: p("sweep"),
            unit: deg(),
        },
    ];
    let text = save(&snapshot, &edits, Tol::witness()).expect("a legal log saves");
    let loaded = load(&text, Tol::witness()).expect("and loads");
    assert_eq!(loaded.edits.len(), 2, "the log round-tripped");
    assert_eq!(
        loaded.edits[0],
        edits[0],
        "the edit itself round-tripped, notation and all"
    );
    match loaded.doc.params()[&p("wall")] {
        DocParam::Continuous {
            display_unit,
            distribution,
            ..
        } => {
            assert_eq!(display_unit, mm(), "replay wrote the notation");
            assert!(distribution.is_some(), "and kept the annotation");
        }
        DocParam::Count { .. } => panic!("still continuous"),
    }
    match loaded.doc.params()[&p("sweep")] {
        DocParam::Continuous { display_unit, .. } => assert_eq!(display_unit, deg()),
        DocParam::Count { .. } => panic!("still continuous"),
    }
    // A log the replay would refuse never reaches a file: the save
    // door runs the same `apply`, so the refusal is symmetric.
    let bad = [DocEdit::SetDocParamUnit {
        name: p("wall"),
        unit: deg(),
    }];
    assert!(
        save(&snapshot, &bad, Tol::witness()).is_err(),
        "save refuses a log that cannot replay"
    );
}
