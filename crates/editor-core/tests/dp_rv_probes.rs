//! **Review probes (lane `dp-rv`) for the notation door.** Not the
//! unit's own suite — these are the claims the PR body makes that its
//! six rows do not execute: symmetry of the three refusals across
//! `apply` / `Doc::replay` / `load` of a HAND-EDITED log, `diff.rs`'s
//! blindness to the notation edit, the `continuous_with` reversion the
//! filed sweep row asserts, and the one-Scalar-row measurement the
//! Python façade's "no `Scalar` arm" argument rests on.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    Dimension, Distribution, Doc, DocEdit, DocParam, DocumentId, EditError, ParamName,
    PersistError, ProfileDoc, UnitSym, apply, load, save,
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

fn fixture() -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive("dp-rv-probes"), Tol::witness());
    for (name, value) in [
        (
            "wall",
            DocParam::continuous_with(
                Dimension::Length,
                0.003,
                Distribution::Normal { sigma: 1e-5 },
            ),
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
        .unwrap()
        .doc;
    }
    doc
}

/// The declaring log the fixture is replayable from, so `Doc::replay`
/// can be asked the same questions `apply` is.
fn declaring_log() -> Vec<DocEdit<editor_core::ProfileProgram>> {
    vec![
        DocEdit::SetDocParam {
            name: p("wall"),
            value: DocParam::continuous_with(
                Dimension::Length,
                0.003,
                Distribution::Normal { sigma: 1e-5 },
            ),
        },
        DocEdit::SetDocParam {
            name: p("sweep"),
            value: DocParam::continuous(Dimension::Angle, 1.5),
        },
        DocEdit::SetDocParam {
            name: p("ribs"),
            value: DocParam::Count { value: 4 },
        },
    ]
}

/// Rewrite the EDITS half of a saved file (the snapshot half is left
/// alone, so a name or symbol that also appears in a declaration is
/// not touched).
fn patch_edits(text: &str, from: &str, to: &str) -> String {
    let at = text.find("\"edits\"").expect("the file has an edits array");
    let (head, tail) = text.split_at(at);
    format!("{head}{}", tail.replace(from, to))
}

/// **Claim 2.** Each refusal is the same typed `EditError` at all
/// three doors: `apply`, `Doc::replay`, and `load` of a file whose
/// edit log was edited by hand into the refusing shape.
#[test]
fn the_three_refusals_are_symmetric_across_apply_replay_and_load() {
    let doc = fixture();
    let cases: [(&str, &str, EditError); 3] = [
        (
            "\"name\": \"wall\"",
            "\"name\": \"nonesuch\"",
            EditError::DocParamNotDeclared {
                name: p("nonesuch"),
            },
        ),
        (
            "\"name\": \"wall\"",
            "\"name\": \"ribs\"",
            EditError::DocParamCountHasNoUnit { name: p("ribs") },
        ),
        (
            "\"unit\": \"mm\"",
            "\"unit\": \"deg\"",
            EditError::DocParamUnitMismatch {
                name: p("wall"),
                unit: Dimension::Angle,
                declared: Dimension::Length,
            },
        ),
    ];
    // A legal file carrying one notation edit; each case bends its log.
    let legal = save(
        &doc,
        &[DocEdit::SetDocParamUnit {
            name: p("wall"),
            unit: mm(),
        }],
        Tol::witness(),
    )
    .expect("the legal log saves");
    for (from, to, want) in cases {
        let direct = match &want {
            EditError::DocParamNotDeclared { name }
            | EditError::DocParamCountHasNoUnit { name } => DocEdit::SetDocParamUnit {
                name: name.clone(),
                unit: mm(),
            },
            _ => DocEdit::SetDocParamUnit {
                name: p("wall"),
                unit: deg(),
            },
        };
        // Door 1: `apply`.
        assert_eq!(
            apply(&doc, &direct, Tol::witness()).expect_err("apply refuses"),
            want,
            "apply's refusal"
        );
        // Door 2: `Doc::replay` from empty over the declaring log plus
        // the refusing edit.
        let mut log = declaring_log();
        log.push(direct.clone());
        assert_eq!(
            Doc::replay(DocumentId::derive("dp-rv-probes"), &log, Tol::witness())
                .expect_err("replay refuses"),
            want,
            "replay's refusal"
        );
        // Door 3: `load` of a hand-edited file.
        let bent = patch_edits(&legal, from, to);
        match load(&bent, Tol::witness()).expect_err("load refuses") {
            PersistError::EditReplay { index, error } => {
                assert_eq!(index, 0, "the refusing edit is named by index");
                assert_eq!(error, want, "load's refusal is the same typed error");
            }
            other => panic!("load refused with {other:?}, not EditReplay"),
        }
        // Door 4: and `save` of the same log refuses identically.
        match save(&doc, &[direct], Tol::witness()).expect_err("save refuses") {
            PersistError::EditReplay { error, .. } => assert_eq!(error, want, "save's refusal"),
            other => panic!("save refused with {other:?}"),
        }
    }
}

/// **Claim 4.** `diff.rs` is blind to the notation edit, as the PR
/// body claims — it compares parameters with `bit_eq`.
#[test]
fn diff_is_blind_to_the_notation_edit() {
    let before = fixture();
    let after = apply(
        &before,
        &DocEdit::SetDocParamUnit {
            name: p("wall"),
            unit: mm(),
        },
        Tol::witness(),
    )
    .unwrap()
    .doc;
    assert_ne!(
        before.params()[&p("wall")],
        after.params()[&p("wall")],
        "the notation really moved"
    );
    assert!(
        before.diff(&after).is_empty(),
        "diff reports nothing: {:?}",
        before.diff(&after)
    );
}

/// **Claim 7.** The saved bytes carry the edit and its table symbol,
/// and a load/re-save is byte-identical.
#[test]
fn the_notation_edit_round_trips_the_bytes() {
    let doc = fixture();
    let edits = [DocEdit::SetDocParamUnit {
        name: p("wall"),
        unit: mm(),
    }];
    let text = save(&doc, &edits, Tol::witness()).expect("saves");
    assert!(
        text.contains("SetDocParamUnit") && text.contains("\"unit\": \"mm\""),
        "the wire form is the derive's, symbol and all"
    );
    let loaded = load(&text, Tol::witness()).expect("loads");
    let again = save(&loaded.snapshot, &loaded.edits, Tol::witness()).expect("re-saves");
    assert_eq!(again, text, "byte-identical round trip");
    // An off-table symbol refuses at the token, not as a wrong unit.
    let bent = patch_edits(&text, "\"unit\": \"mm\"", "\"unit\": \"furlong\"");
    assert!(
        load(&bent, Tol::witness()).is_err(),
        "an off-table notation refuses"
    );
}

/// **Claim 8** (the filed sweep row): `continuous_with` writes the
/// canonical notation, so annotating a parameter authored in mm
/// through create-or-replace reverts it to metres.
#[test]
fn annotating_through_create_or_replace_reverts_the_notation() {
    let doc = fixture();
    let in_mm = apply(
        &doc,
        &DocEdit::SetDocParamUnit {
            name: p("wall"),
            unit: mm(),
        },
        Tol::witness(),
    )
    .unwrap()
    .doc;
    let DocParam::Continuous { dim, value, .. } = in_mm.params()[&p("wall")] else {
        panic!("continuous")
    };
    let reannotated = apply(
        &in_mm,
        &DocEdit::SetDocParam {
            name: p("wall"),
            value: DocParam::continuous_with(dim, value, Distribution::Normal { sigma: 2e-5 }),
        },
        Tol::witness(),
    )
    .unwrap()
    .doc;
    match reannotated.params()[&p("wall")] {
        DocParam::Continuous { display_unit, .. } => assert_eq!(
            display_unit,
            UnitSym::canonical_for(Dimension::Length),
            "the notation reverted to canonical — the filed row's claim"
        ),
        DocParam::Count { .. } => panic!("continuous"),
    }
}

/// **Claim 6.** The Python façade has no `Scalar` arm because the
/// table has exactly one Scalar row. Nothing else guards that number.
#[test]
fn the_table_has_exactly_one_scalar_row() {
    let scalar: Vec<_> = quantity::UNITS
        .iter()
        .filter(|r| r.quantity() == quantity::UnitQuantity::Scalar)
        .collect();
    assert_eq!(
        scalar.len(),
        1,
        "the façade's 'a scalar parameter has one notation' argument holds only while this is 1"
    );
    assert_eq!(
        UnitSym::from_def(scalar[0]),
        UnitSym::canonical_for(Dimension::Scalar),
        "and it is the canonical one, so a scalar parameter is already written in it"
    );
}

/// **The sibling door is not swept.** `EditError::DocParamUnitMismatch`'s
/// rustdoc says the pairing fault is now refused "at the edit door,
/// before it can reach a document at all". It is refused at the
/// NOTATION door; the create-or-replace door still accepts a
/// mismatched pair into a live document, and the refusal only arrives
/// at save/load.
#[test]
fn the_create_or_replace_door_still_admits_a_mismatched_pairing() {
    let doc = fixture();
    let crooked = DocParam::Continuous {
        dim: Dimension::Length,
        value: 0.003,
        display_unit: deg(),
        distribution: None,
    };
    let after = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: p("wall"),
            value: crooked,
        },
        Tol::witness(),
    )
    .expect("create-or-replace accepts a length written in degrees")
    .doc;
    match after.params()[&p("wall")] {
        DocParam::Continuous { display_unit, .. } => assert_eq!(
            display_unit,
            deg(),
            "the mismatched pairing IS in the document"
        ),
        DocParam::Count { .. } => panic!("continuous"),
    }
    // And only the persistence validator says so.
    assert!(
        matches!(
            save(&after, &[], Tol::witness()),
            Err(PersistError::DisplayUnit { .. })
        ),
        "the fault surfaces at save, not at the edit door"
    );
}
