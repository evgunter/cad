//! **The annotation door for a document parameter** — `DocParam::
//! with_distribution` and the `DocEdit::SetDocParamDistribution` that
//! routes through it.
//!
//! The declaration has four fields; two already had a narrow edit
//! (`SetDocParamValue` through `DocParam::with_value`,
//! `SetDocParamUnit` through `DocParam::with_display_unit`) and this
//! is the third. Without this door the only way to add, change or
//! clear an E1/E2 annotation is `SetDocParam`, which is
//! create-or-replace: the authoring spelling for an annotated
//! parameter, `DocParam::continuous_with`, writes the CANONICAL
//! notation, so a parameter authored in millimetres reverts to metres
//! the moment anyone annotates it — with no refusal and no
//! diagnostic. The first row below is that fact, and every row after
//! it is the door that avoids it.
//!
//! **The ruling these rows execute**: there is ONE annotation door and
//! `None` clears. The argument is stated once, in
//! `DocParam::with_distribution`'s rustdoc, and run here by
//! `clearing_is_the_same_door_and_keeps_the_rest_of_the_declaration`
//! rather than restated in prose.
//!
//! # Where the twin rows live
//!
//! `edit_doc_param_unit.rs` holds the mirror-image trap — re-spelling
//! a notation through create-or-replace and losing the annotation —
//! and its `annotating_through_create_or_replace_reverts_the_notation`
//! is this file's own finding, written from the other side by the lane
//! that found it. `m10_1_r2_probes.rs` §6 pins the same carry-forward
//! CLASS over the VALUE field. These rows cross-cite both rather than
//! restate them: each suite is gated to the files it is about, and
//! folding them together would put a row where its gate does not
//! reach.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/doc.rs",
    "crates/editor-core/src/edit.rs",
    "crates/editor-core/src/distribution.rs",
    "crates/editor-core/src/persist/check.rs",
];

use editor_core::{
    AnalysisPolicy, CarryForwardDoor, Dimension, Distribution, DistributionFault,
    DistributionField, DistributionRefusal, Doc, DocEdit, DocParam, DocParamValue, DocumentId,
    EditError, ParamName, PersistError, ProfileDoc, UnitSym, analyzed_box, apply, load, save,
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

fn band() -> Distribution {
    Distribution::Band {
        lo: -2e-5,
        hi: 3e-5,
    }
}

/// The notation a parameter is written in.
fn notation(doc: &editor_core::ProfileDoc, name: &str) -> UnitSym {
    match doc.params()[&p(name)] {
        DocParam::Continuous { display_unit, .. } => display_unit,
        DocParam::Count { .. } => panic!("{name} is continuous"),
    }
}

/// Whether the error analysis reads the parameter as FIXED — the
/// consequence an annotation has, as opposed to a field going `Some`.
fn is_fixed(doc: &editor_core::ProfileDoc, name: &str) -> bool {
    analyzed_box(doc, &AnalysisPolicy::default())
        .get(&p(name))
        .copied()
        .expect("the parameter has an axis")
        .offsets
        .is_fixed()
}

/// The declaring log the fixture is replayable from: `wall`, a Length
/// authored in MILLIMETRES and unannotated; `bore`, annotated; `ribs`,
/// a Count.
fn declaring_log() -> Vec<DocEdit<editor_core::ProfileProgram>> {
    vec![
        DocEdit::SetDocParam {
            name: p("wall"),
            value: DocParam::written_length(WrittenLength::in_unit(3.0, MM)),
        },
        DocEdit::SetDocParam {
            name: p("bore"),
            value: DocParam::continuous_with(Dimension::Length, 0.01, sigma()),
        },
        DocEdit::SetDocParam {
            name: p("ribs"),
            value: DocParam::Count { value: 4 },
        },
    ]
}

fn fixture() -> ProfileDoc {
    let mut doc = ProfileDoc::empty(
        DocumentId::derive("edit-doc-param-distribution"),
        Tol::witness(),
    );
    for edit in declaring_log() {
        doc = apply(&doc, &edit, Tol::witness(), &editor_core::RefusingReach)
            .expect("the fixture parameters are valid")
            .doc;
    }
    doc
}

/// **The row this door exists for.** Annotating a standing parameter
/// keeps the rest of its declaration — here, the notation it was
/// authored in.
///
/// Written through the only spelling available before this door
/// (`SetDocParam` with `DocParam::continuous_with`) it FAILS: the
/// notation reverts to the canonical unit, silently. That is the
/// filed finding, and `edit_doc_param_unit.rs`'s
/// `annotating_through_create_or_replace_reverts_the_notation` holds
/// it as a standing row.
#[test]
fn annotating_a_standing_parameter_keeps_its_notation() {
    let before = fixture();
    assert_eq!(
        notation(&before, "wall"),
        mm(),
        "the fixture parameter is written in millimetres to begin with"
    );
    assert!(is_fixed(&before, "wall"), "and carries no annotation");
    let after = apply(
        &before,
        &DocEdit::SetDocParamDistribution {
            name: p("wall"),
            distribution: Some(sigma()),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("an annotation edit on a declared continuous parameter applies")
    .doc;
    assert!(
        after.params()[&p("wall")]
            .distribution()
            .is_some_and(|d| d.bit_eq(&sigma())),
        "the annotation landed, bit for bit"
    );
    assert_eq!(
        notation(&after, "wall"),
        mm(),
        "and the notation the parameter was authored in survived it"
    );
    // At the older twin's strength: the annotation is not a field
    // going `Some`, it is the analysis now reading the parameter as
    // VARYING.
    assert!(
        !is_fixed(&after, "wall"),
        "the analysis reads the annotated parameter as varying"
    );
}

/// The other two fields of the declaration ride through as well: the
/// dimension and the exact value, bit for bit.
#[test]
fn the_annotation_door_carries_the_value_forward() {
    let before = fixture();
    let after = apply(
        &before,
        &DocEdit::SetDocParamDistribution {
            name: p("wall"),
            distribution: Some(band()),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("applies")
    .doc;
    match after.params()[&p("wall")] {
        DocParam::Continuous {
            dim,
            value,
            display_unit,
            distribution,
        } => {
            assert_eq!(dim, Dimension::Length, "the dimension is the declaration's");
            assert_eq!(
                value.to_bits(),
                WrittenLength::in_unit(3.0, MM).meters().to_bits(),
                "the value is bit-identical"
            );
            assert_eq!(display_unit, mm(), "the notation is the declaration's");
            assert!(
                distribution.expect("annotated").bit_eq(&band()),
                "and the annotation is the one offered"
            );
        }
        DocParam::Count { .. } => panic!("still continuous"),
    }
    // Unlike the notation, an annotation is bit-semantic: `bit_eq`
    // sees it, so the edit reaches replay identity and `diff.rs`.
    assert!(
        !before.bit_eq(&after),
        "an annotation change is a change to what the parameter IS"
    );
    assert!(
        !before.diff(&after).is_empty(),
        "and the document diff reports it: {:?}",
        before.diff(&after)
    );
}

/// **The clearing ruling, executed.** `None` goes through the SAME
/// door, and clears while keeping the notation and the value — the
/// reading stated in `DocParam::with_distribution`'s rustdoc. A second
/// `ClearDocParamDistribution` arm would be the `SetAppearanceMeta`
/// shape, and the declaration is not a map.
#[test]
fn clearing_is_the_same_door_and_keeps_the_rest_of_the_declaration() {
    let before = fixture();
    let in_mm = apply(
        &before,
        &DocEdit::SetDocParamUnit {
            name: p("bore"),
            unit: mm(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the notation edit applies")
    .doc;
    assert!(!is_fixed(&in_mm, "bore"), "the parameter starts annotated");
    let cleared = apply(
        &in_mm,
        &DocEdit::SetDocParamDistribution {
            name: p("bore"),
            distribution: None,
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("clearing goes through the same door")
    .doc;
    assert_eq!(
        cleared.params()[&p("bore")].distribution(),
        None,
        "the annotation is gone"
    );
    assert!(
        is_fixed(&cleared, "bore"),
        "and the analysis reads the parameter as fixed again"
    );
    assert_eq!(
        notation(&cleared, "bore"),
        mm(),
        "the notation survived the clearing"
    );
    match cleared.params()[&p("bore")] {
        DocParam::Continuous { dim, value, .. } => {
            assert_eq!(dim, Dimension::Length);
            assert_eq!(value.to_bits(), 0.01_f64.to_bits(), "and so did the value");
        }
        DocParam::Count { .. } => panic!("still continuous"),
    }
    // Clearing an already-unannotated parameter is accepted: `None` is
    // a value of the field, not a removal that has to find something.
    assert!(
        apply(
            &cleared,
            &DocEdit::SetDocParamDistribution {
                name: p("bore"),
                distribution: None,
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .is_ok(),
        "writing None over None is the same write"
    );
}

/// `DocParam::with_distribution` on its own — the carry-forward in one
/// place, and the two typed refusals the edit door only routes.
#[test]
fn with_distribution_is_the_carry_forward_and_its_refusals_are_typed() {
    let written = DocParam::written_length(WrittenLength::in_unit(3.0, MM));
    let annotated = written
        .with_distribution(Some(sigma()))
        .expect("a continuous parameter takes an annotation");
    match annotated {
        DocParam::Continuous { display_unit, .. } => {
            assert_eq!(display_unit, mm(), "the notation rode through")
        }
        DocParam::Count { .. } => panic!("still continuous"),
    }
    assert_eq!(
        annotated
            .with_distribution(None)
            .expect("and clears")
            .distribution(),
        None
    );
    // A count has no field to hang one on, and says so — for a
    // clearing edit too.
    assert_eq!(
        DocParam::Count { value: 4 }.with_distribution(Some(sigma())),
        Err(DistributionRefusal::CountHasNoAnnotation)
    );
    assert_eq!(
        DocParam::Count { value: 4 }.with_distribution(None),
        Err(DistributionRefusal::CountHasNoAnnotation),
        "clearing a count's annotation is the same wrong parameter"
    );
    // The shape invariants are `Distribution::check`'s, and the fault
    // rides out whole rather than being re-derived by the caller.
    assert_eq!(
        written.with_distribution(Some(Distribution::Normal { sigma: -1.0 })),
        Err(DistributionRefusal::Invalid {
            fault: DistributionFault::SigmaNotPositive { sigma: -1.0 }
        })
    );
    assert_eq!(
        written.with_distribution(Some(Distribution::Band { lo: 1.0, hi: 2.0 })),
        Err(DistributionRefusal::Invalid {
            fault: DistributionFault::NominalOutsideSupport { lo: 1.0, hi: 2.0 }
        })
    );
    assert_eq!(
        written.with_distribution(Some(Distribution::Normal { sigma: f64::NAN })),
        Err(DistributionRefusal::Invalid {
            fault: DistributionFault::NonFinite {
                field: DistributionField::Sigma
            }
        })
    );
}

/// Each refusal by name, at the edit door.
#[test]
fn the_annotation_door_refuses_typed() {
    let doc = fixture();
    let refuse = |name: &str, distribution: Option<Distribution>| {
        apply(
            &doc,
            &DocEdit::SetDocParamDistribution {
                name: p(name),
                distribution,
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect_err("refused")
    };
    // Nothing to carry forward — and the refusal names the door the
    // caller actually used, not "a carry-forward edit".
    assert_eq!(
        refuse("nonesuch", Some(sigma())),
        EditError::DocParamNotDeclared {
            name: p("nonesuch"),
            door: CarryForwardDoor::Annotation,
        }
    );
    // A count is structural, fixed under any error analysis.
    assert_eq!(
        refuse("ribs", Some(sigma())),
        EditError::DocParamCountHasNoDistribution { name: p("ribs") }
    );
    // The E2 invariants, by the same check the persistence doors run,
    // split by CLASS: a non-finite offset is a non-finite float on a
    // document parameter, the rest are distribution shape faults.
    assert_eq!(
        refuse("wall", Some(Distribution::Uniform { lo: 1.0, hi: 2.0 })),
        EditError::InvalidDistribution {
            name: p("wall"),
            fault: DistributionFault::NominalOutsideSupport { lo: 1.0, hi: 2.0 },
        }
    );
    assert_eq!(
        refuse("wall", Some(Distribution::Normal { sigma: 0.0 })),
        EditError::InvalidDistribution {
            name: p("wall"),
            fault: DistributionFault::SigmaNotPositive { sigma: 0.0 },
        }
    );
    assert_eq!(
        refuse(
            "wall",
            Some(Distribution::Band {
                lo: f64::NEG_INFINITY,
                hi: 0.0
            })
        ),
        EditError::NonFiniteDocParam {
            name: p("wall"),
            field: editor_core::DocParamField::Offset(editor_core::DistributionField::Lo),
        },
        "a non-finite offset joins the ruled non-finite class, naming the offset, as it does at \
         the other door"
    );
    // The undeclared-name sentence says WHICH door, so the three
    // carry-forward edits do not render one indistinguishable refusal.
    let annotation = refuse("nonesuch", Some(sigma())).to_string();
    assert!(
        annotation.contains("an annotation edit")
            && annotation.contains(editor_core::edit::UNDECLARED_PARAM_RECOURSE),
        "the sentence names the annotation door and keeps its recourse: {annotation:?}"
    );
    for other in [
        DocEdit::SetDocParamValue {
            name: p("nonesuch"),
            value: DocParamValue::Continuous(1.0),
        },
        DocEdit::SetDocParamUnit {
            name: p("nonesuch"),
            unit: mm(),
        },
    ] {
        let text = apply(&doc, &other, Tol::witness(), &editor_core::RefusingReach)
            .expect_err("the sibling door refuses the same undeclared name")
            .to_string();
        assert_ne!(
            text, annotation,
            "one arm, one sentence per door — the door is what differs"
        );
    }
}

/// **F6**, the half the census does not say. `display_contract.rs`'s
/// `a_dimension_reaches_refusal_prose_as_a_word_not_as_its_variant`
/// is the ONE home for the new `EditError`'s prose contract — the
/// reason words, the punctuation ban, and that the rendering is not
/// its `Debug`. Two things sit outside it and only here: that the
/// parameter NAME is interpolated rather than described, and that the
/// DOOR's own `DistributionRefusal` renders for a caller holding the
/// `Err` without an `EditError` around it.
#[test]
fn the_count_refusal_names_its_parameter_and_the_door_renders_alone() {
    let shown = EditError::DocParamCountHasNoDistribution { name: p("ribs") }.to_string();
    assert!(
        shown.contains("parameter ribs"),
        "{shown:?} describes the parameter instead of naming it"
    );
    let door = DistributionRefusal::CountHasNoAnnotation.to_string();
    assert!(
        door.contains("structural parameter") && !door.contains("CountHasNoAnnotation"),
        "{door:?}"
    );
}

/// Rewrite the EDITS half of a saved file (the snapshot half is left
/// alone, so a name that also appears in a declaration is not
/// touched).
fn patch_edits(text: &str, from: &str, to: &str) -> String {
    let at = text.find("\"edits\"").expect("the file has an edits array");
    let (head, tail) = text.split_at(at);
    format!("{head}{}", tail.replace(from, to))
}

/// The edit persists and replays like every other `DocEdit`: set,
/// change and clear all round-trip, and the loaded document carries
/// the notation the annotations rode past.
#[test]
fn the_annotation_edit_saves_replays_and_loads() {
    let snapshot = fixture();
    let edits = [
        DocEdit::SetDocParamDistribution {
            name: p("wall"),
            distribution: Some(sigma()),
        },
        DocEdit::SetDocParamDistribution {
            name: p("wall"),
            distribution: Some(band()),
        },
        DocEdit::SetDocParamDistribution {
            name: p("bore"),
            distribution: None,
        },
    ];
    let text = save(
        &snapshot,
        &editor_core::LoggedEdit::bare_all(&edits),
        Tol::witness(),
    )
    .expect("a legal log saves");
    assert!(
        text.contains("SetDocParamDistribution") && text.contains("\"distribution\""),
        "the wire form is the derive's, symbol and all"
    );
    let loaded = load(&text, Tol::witness()).expect("and loads");
    assert_eq!(loaded.edits.len(), 3, "the log round-tripped");
    assert_eq!(
        loaded.edits,
        editor_core::LoggedEdit::bare_all(&edits),
        "each edit round-tripped, payload and all"
    );
    assert!(
        loaded.doc.params()[&p("wall")]
            .distribution()
            .is_some_and(|d| d.bit_eq(&band())),
        "replay wrote the LAST annotation"
    );
    assert_eq!(
        notation(&loaded.doc, "wall"),
        mm(),
        "and the notation survived the replay"
    );
    assert_eq!(
        loaded.doc.params()[&p("bore")].distribution(),
        None,
        "the clearing replayed too"
    );
    let again = save(&loaded.snapshot, &loaded.edits, Tol::witness()).expect("re-saves");
    assert_eq!(again, text, "byte-identical round trip");
}

/// Each refusal is the same typed `EditError` at all four doors:
/// `apply`, `Doc::replay`, `load` of a file whose edit log was bent by
/// hand, and `save` of the same log.
#[test]
fn the_refusals_are_symmetric_across_apply_replay_save_and_load() {
    let doc = fixture();
    let legal = save(
        &doc,
        &[DocEdit::SetDocParamDistribution {
            name: p("wall"),
            distribution: Some(sigma()),
        }
        .into()],
        Tol::witness(),
    )
    .expect("the legal log saves");
    let cases: [(&str, &str, DocEdit<editor_core::ProfileProgram>, EditError); 3] = [
        (
            "\"name\": \"wall\"",
            "\"name\": \"nonesuch\"",
            DocEdit::SetDocParamDistribution {
                name: p("nonesuch"),
                distribution: Some(sigma()),
            },
            EditError::DocParamNotDeclared {
                name: p("nonesuch"),
                door: CarryForwardDoor::Annotation,
            },
        ),
        (
            "\"name\": \"wall\"",
            "\"name\": \"ribs\"",
            DocEdit::SetDocParamDistribution {
                name: p("ribs"),
                distribution: Some(sigma()),
            },
            EditError::DocParamCountHasNoDistribution { name: p("ribs") },
        ),
        (
            "\"sigma\": 0.00001",
            "\"sigma\": -1.0",
            DocEdit::SetDocParamDistribution {
                name: p("wall"),
                distribution: Some(Distribution::Normal { sigma: -1.0 }),
            },
            EditError::InvalidDistribution {
                name: p("wall"),
                fault: DistributionFault::SigmaNotPositive { sigma: -1.0 },
            },
        ),
    ];
    for (from, to, direct, want) in cases {
        assert_eq!(
            apply(&doc, &direct, Tol::witness(), &editor_core::RefusingReach)
                .expect_err("apply refuses"),
            want,
            "apply's refusal"
        );
        let mut log = declaring_log();
        log.push(direct.clone());
        assert_eq!(
            Doc::replay(
                DocumentId::derive("edit-doc-param-distribution"),
                &editor_core::LoggedEdit::bare_all(&log),
                Tol::witness()
            )
            .expect_err("replay refuses"),
            want,
            "replay's refusal"
        );
        let bent = patch_edits(&legal, from, to);
        assert_ne!(bent, legal, "the patch matched something");
        match load(&bent, Tol::witness()).expect_err("load refuses") {
            PersistError::EditReplay { index, error } => {
                assert_eq!(index, 0, "the refusing edit is named by index");
                assert_eq!(error, want, "load's refusal is the same typed error");
            }
            other => panic!("load refused with {other:?}, not EditReplay"),
        }
        match save(&doc, &[direct.into()], Tol::witness()).expect_err("save refuses") {
            PersistError::EditReplay { error, .. } => assert_eq!(error, want, "save's refusal"),
            other => panic!("save refused with {other:?}"),
        }
    }
}

/// **The persistence float walk sees the EDIT's offsets.** A saved log
/// is DATA — it has not necessarily been applied by this process — so
/// the persistence doors walk every float an edit carries, and this
/// door's whole payload is floats. The refusal names the parameter and
/// the offending FIELD, through the same site vocabulary every other
/// float in the document goes through rather than a second spelling.
///
/// Reached through `save` of an in-memory log rather than through a
/// hand-bent file, because JSON cannot spell a non-finite number at
/// all: `1e400` in a file is a PARSE refusal one door earlier.
#[test]
fn a_non_finite_offset_on_the_edit_refuses_at_the_persistence_door() {
    let doc = fixture();
    let edits = [DocEdit::SetDocParamDistribution {
        name: p("wall"),
        distribution: Some(Distribution::TruncatedNormal {
            sigma: 1e-5,
            lo: f64::NEG_INFINITY,
            hi: 0.0,
        }),
    }];
    match save(
        &doc,
        &editor_core::LoggedEdit::bare_all(&edits),
        Tol::witness(),
    )
    .expect_err("save refuses")
    {
        PersistError::NonFinite { site } => {
            let shown = format!("{site:?}");
            assert!(
                shown.contains("wall") && shown.contains("Lo"),
                "the site names the parameter and the offending field: {shown}"
            );
        }
        other => panic!("save refused with {other:?}, not NonFinite"),
    }
}
