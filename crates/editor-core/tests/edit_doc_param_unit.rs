//! **The notation door for a document parameter** — `DocParam::
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
//! **The reading these rows pin** is stated once, in
//! `DocParam::with_display_unit`'s rustdoc, and executed here by
//! `bit_eq_is_blind_to_the_notation_edit` and
//! `diff_is_blind_to_the_notation_edit` rather than restated in prose.
//!
//! # Where the twin rows live
//!
//! `m10_1_r2_probes.rs` §6 pins the same carry-forward CLASS over the
//! VALUE field: the same trap through create-or-replace, then
//! `SetDocParamValue` avoiding it. **The two sections cross-cite rather
//! than merge**, deliberately: that suite is an independent derivation
//! of M10-1's error-analysis claims, gated to `distribution.rs`,
//! `analysis.rs` and `measure.rs`, and these rows are about the edit
//! vocabulary and are gated to `doc.rs` / `edit.rs`. Folding one into
//! the other would put a row where its gate does not reach the file it
//! is about. What IS matched is their STRENGTH: the trap row below
//! asserts what the older twin asserts — that the analysis then reads
//! the parameter as FIXED — because a deleted annotation whose
//! consequence is not shown is a field going `None`, not a defect.
//! `work/edit/doc-param-distribution-edit-has-no-door` will take the
//! same shape at the third field.
//!
//! # Rows adopted from the review lane
//!
//! The rows below `the_three_refusals_are_symmetric_across_apply_replay_and_load`
//! were written by the style review's probe lane (`review/dp-rv`,
//! `2b6bd62e4`, `crates/editor-core/tests/dp_rv_probes.rs`) against
//! claims this PR's body made and its own rows did not execute. They
//! are kept in their author's words and numbering; the one that
//! asserted the create-or-replace door still ADMITS a mismatched
//! pairing is inverted, because that finding was accepted and the door
//! now refuses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/doc.rs",
    "crates/editor-core/src/edit.rs",
    "crates/editor-core/src/expr.rs",
    "crates/editor-core/src/parse.rs",
    "crates/editor-core/src/persist/check.rs",
];

use editor_core::{
    AnalysisPolicy, CarryForwardDoor, Dimension, DisplayUnitRefusal, Distribution, Doc, DocEdit,
    DocParam, DocParamValue, DocumentId, EditError, ParamName, PersistError, ProfileDoc, UnitSym,
    analyzed_box, apply, load, save,
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
            &editor_core::RefusingReach,
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
        &editor_core::RefusingReach,
    )
    .expect("create-or-replace accepts it")
    .doc;
    assert_eq!(
        after.params()[&p("wall")].distribution(),
        None,
        "the create-or-replace door deleted the annotation the caller never named"
    );
    // The older twin's strength (`m10_1_r2_probes.rs` section 6): the
    // deletion is not a field going `None`, it is the analysis now
    // reading a varying parameter as FIXED.
    let axis = analyzed_box(&after, &AnalysisPolicy::default())
        .get(&p("wall"))
        .copied()
        .expect("the parameter has an axis");
    assert!(
        axis.offsets.is_fixed(),
        "a notation rebuild turned a varying parameter into a fixed one: {axis:?}"
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
        &editor_core::RefusingReach,
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
    // The trap row's mirror at the same strength: the analysis still
    // reads the parameter as varying.
    let axis = analyzed_box(&after, &AnalysisPolicy::default())
        .get(&p("wall"))
        .copied()
        .expect("the parameter has an axis");
    assert!(
        !axis.offsets.is_fixed(),
        "the parameter is still varying: {axis:?}"
    );
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
    // The two refusals are the DOOR's, and typed: a caller routing them
    // reads which applies rather than re-deriving it.
    assert_eq!(
        annotated.with_display_unit(deg()),
        Err(DisplayUnitRefusal::Mismatch {
            unit: Dimension::Angle,
            declared: Dimension::Length,
        }),
        "a unit that does not measure the declared dimension, and it says so"
    );
    assert_eq!(
        DocParam::Count { value: 4 }.with_display_unit(mm()),
        Err(DisplayUnitRefusal::CountHasNoNotation),
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
            &editor_core::RefusingReach,
        )
        .expect_err("refused")
    };
    // Nothing to carry forward — and the refusal names the door the
    // caller actually used, not "a carry-forward edit".
    assert_eq!(
        refuse("nonesuch", mm()),
        EditError::DocParamNotDeclared {
            name: p("nonesuch"),
            door: CarryForwardDoor::Notation,
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
    // The undeclared-name sentence says WHICH door, so the two
    // carry-forward edits do not render one indistinguishable refusal.
    let notation = refuse("nonesuch", mm()).to_string();
    assert!(
        notation.contains("a notation edit")
            && notation.contains(editor_core::edit::UNDECLARED_PARAM_RECOURSE),
        "the sentence names the notation door and keeps its recourse: {notation:?}"
    );
    let value = apply(
        &doc,
        &DocEdit::SetDocParamValue {
            name: p("nonesuch"),
            value: DocParamValue::Continuous(1.0),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect_err("the value door refuses the same undeclared name")
    .to_string();
    assert!(
        value.contains("a value edit")
            && value.contains(editor_core::edit::UNDECLARED_PARAM_RECOURSE),
        "and the value door names itself, with the same recourse: {value:?}"
    );
    assert_ne!(
        notation, value,
        "one arm, two sentences — the door is what differs"
    );
}

/// **The mismatch sentence, in the right ORDER.** `DocParamUnitMismatch`
/// carries two dimensions and renders both, so a rendering that swapped
/// them would still contain both words and still name the parameter —
/// invisible to the row above. This one pins each dimension to the
/// clause it belongs in, so the swap reds.
///
/// It also holds the sentence in step with `PersistError::DisplayUnit`'s
/// over the same fault. They are the same shape — *declared X but the
/// display unit measures Y* — with ONE deliberate word of difference:
/// the validator says "its display unit" because there the unit is
/// already stored on the document, and the edit door says "the display
/// unit offered" because there it is an argument that never reached
/// one. A change to either that does not reach the other is red here.
#[test]
fn the_mismatch_sentence_says_which_dimension_is_which() {
    let doc = fixture();
    let edit = apply(
        &doc,
        &DocEdit::SetDocParamUnit {
            name: p("wall"),
            unit: deg(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect_err("refused")
    .to_string();
    let validator = PersistError::DisplayUnit {
        name: p("wall"),
        unit: Dimension::Angle,
        declared: Dimension::Length,
    }
    .to_string();
    for (who, text) in [("the edit door", &edit), ("the validator", &validator)] {
        let declared = text
            .find("declared length")
            .unwrap_or_else(|| panic!("{who} states the DECLARED dimension after `declared`, not the offered one: {text:?}"));
        let measured = text.find("measures angle").unwrap_or_else(|| {
            panic!("{who} states what the OFFERED unit measures after `measures`: {text:?}")
        });
        assert!(
            declared < measured,
            "{who} states the declaration before the offer: {text:?}"
        );
    }
    assert!(
        edit.contains("display unit offered") && validator.contains("its display unit"),
        "the one word that differs is the one the doc says differs: {edit:?} / {validator:?}"
    );
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
        &editor_core::RefusingReach,
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
    let text = save(
        &snapshot,
        &editor_core::LoggedEdit::bare_all(&edits),
        Tol::witness(),
    )
    .expect("a legal log saves");
    let loaded = load(&text, Tol::witness()).expect("and loads");
    assert_eq!(loaded.edits.len(), 2, "the log round-tripped");
    assert_eq!(
        loaded.edits[0],
        editor_core::LoggedEdit::from(edits[0].clone()),
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
        save(
            &snapshot,
            &editor_core::LoggedEdit::bare_all(&bad),
            Tol::witness()
        )
        .is_err(),
        "save refuses a log that cannot replay"
    );
}

// ---------------------------------------------------------------
// Rows adopted from the style review's probe lane (`review/dp-rv`,
// `2b6bd62e4`). Their author's words and claim numbering are kept.
// ---------------------------------------------------------------

/// The declaring log the fixture is replayable from, so `Doc::replay`
/// can be asked the same questions `apply` is.
fn declaring_log() -> Vec<DocEdit<editor_core::ProfileProgram>> {
    vec![
        DocEdit::SetDocParam {
            name: p("wall"),
            value: DocParam::continuous_with(Dimension::Length, 0.003, sigma()),
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
                door: CarryForwardDoor::Notation,
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
        }
        .into()],
        Tol::witness(),
    )
    .expect("the legal log saves");
    for (from, to, want) in cases {
        let direct = match &want {
            EditError::DocParamNotDeclared { name, .. }
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
            apply(&doc, &direct, Tol::witness(), &editor_core::RefusingReach)
                .expect_err("apply refuses"),
            want,
            "apply's refusal"
        );
        // Door 2: `Doc::replay` from empty over the declaring log plus
        // the refusing edit.
        let mut log = declaring_log();
        log.push(direct.clone());
        assert_eq!(
            Doc::replay(
                DocumentId::derive("edit-doc-param-unit"),
                &editor_core::LoggedEdit::bare_all(&log),
                Tol::witness()
            )
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
        match save(&doc, &[direct.into()], Tol::witness()).expect_err("save refuses") {
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
        &editor_core::RefusingReach,
    )
    .expect("applies")
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
    let text = save(
        &doc,
        &editor_core::LoggedEdit::bare_all(&edits),
        Tol::witness(),
    )
    .expect("saves");
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
        &editor_core::RefusingReach,
    )
    .expect("the notation edit applies")
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
        &editor_core::RefusingReach,
    )
    .expect("create-or-replace applies")
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
        .filter(|r| UnitSym::from_def(r).measures() == Dimension::Scalar)
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

/// **The sibling door, swept.** The review found this row's claim
/// inverted: `EditError::DocParamUnitMismatch`'s rustdoc said the
/// pairing fault was refused "at the edit door, before it can reach a
/// document at all", while create-or-replace still let a mismatched
/// pair into a live document and only save/load objected. The finding
/// was accepted and `write_doc_param` now asks `measures()` too, so
/// this row is the same probe with its verdict flipped: EVERY door
/// that writes a declaration refuses the pair.
///
/// The validator's arm is not dead behind it. A mismatched pairing can
/// still arrive from a FILE — the wire form carries `dim` and the
/// symbol independently — and the second half here is that path.
#[test]
fn the_create_or_replace_door_refuses_a_mismatched_pairing() {
    let doc = fixture();
    let crooked = DocParam::Continuous {
        dim: Dimension::Length,
        value: 0.003,
        display_unit: deg(),
        distribution: None,
    };
    assert_eq!(
        apply(
            &doc,
            &DocEdit::SetDocParam {
                name: p("wall"),
                value: crooked,
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect_err("a length written in degrees is refused at the edit door"),
        EditError::DocParamUnitMismatch {
            name: p("wall"),
            unit: Dimension::Angle,
            declared: Dimension::Length,
        },
        "the same typed refusal the notation door gives"
    );
    // The document is unchanged, so no in-memory document can hold a
    // parameter no file could carry.
    assert_eq!(
        doc.params()[&p("wall")].dim(),
        Dimension::Length,
        "the refusal left the declaration alone"
    );
    // Still reachable from a FILE, which is why the validator keeps
    // its arm: the snapshot's `dim` and its symbol are independent
    // tokens there.
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    let bent = text.replacen("\"display_unit\": \"m\"", "\"display_unit\": \"deg\"", 1);
    assert_ne!(bent, text, "the snapshot really carried a metre notation");
    assert!(
        matches!(
            load(&bent, Tol::witness()),
            Err(PersistError::DisplayUnit { .. })
        ),
        "a hand-edited file still meets the validator"
    );
}
