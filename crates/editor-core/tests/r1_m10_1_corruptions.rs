//! **R1 review probes for M10-1, corruption half**: hand-planted
//! defects in v15 text, each of which must refuse at LOAD — typed,
//! never a best-effort load (spec §2; review claim 3).
//!
//! The PR's own rows corrupt the SNAPSHOT; the unique rows here also
//! corrupt the EDIT LOG of a saved file (load replays the log through
//! the edit door), plant a `distribution` where the schema has no such
//! field, and misspell a form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    Dimension, Distribution, DistributionFault, DocEdit, DocParam, DocumentId, EditError,
    ParamName, PersistError, ProfileDoc, load, save,
};
use geom_core::Tol;

fn annotated_doc(sigma: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive("r1-corrupt"), Tol::witness());
    editor_core::apply(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("s"),
            value: DocParam::continuous_with(
                Dimension::Length,
                1.0,
                Distribution::Normal { sigma },
            ),
        },
        Tol::witness(),
    )
    .expect("applies")
    .doc
}

/// Every snapshot-level shape corruption refuses typed at LOAD with
/// the fault the save door would name.
#[test]
fn planted_snapshot_corruptions_refuse_typed_at_load() {
    let text = save(&annotated_doc(0.01), &[], Tol::witness()).expect("saves");
    let cases: [(&str, &str, DistributionFault); 3] = [
        (
            "zero sigma",
            "\"sigma\": 0.0",
            DistributionFault::SigmaNotPositive { sigma: 0.0 },
        ),
        (
            "negative sigma",
            "\"sigma\": -0.5",
            DistributionFault::SigmaNotPositive { sigma: -0.5 },
        ),
        (
            "tiny negative sigma",
            "\"sigma\": -1e-300",
            DistributionFault::SigmaNotPositive { sigma: -1e-300 },
        ),
    ];
    for (label, replacement, fault) in cases {
        let corrupt = text.replace("\"sigma\": 0.01", replacement);
        assert_ne!(corrupt, text, "{label}: the corruption must land");
        match load(&corrupt, Tol::witness()) {
            Err(PersistError::Distribution { name, fault: got }) => {
                assert_eq!(name.0, "s", "{label}");
                assert_eq!(got, fault, "{label}");
            }
            other => panic!("{label}: must refuse typed, got {other:?}"),
        }
    }
}

/// A bounds corruption (support no longer containing the nominal)
/// refuses at LOAD naming lo and hi.
#[test]
fn a_planted_bounds_corruption_refuses_at_load() {
    let doc = {
        let doc = ProfileDoc::empty(DocumentId::derive("r1-corrupt-b"), Tol::witness());
        editor_core::apply(
            &doc,
            &DocEdit::SetDocParam {
                name: ParamName::new("b"),
                value: DocParam::continuous_with(
                    Dimension::Length,
                    1.0,
                    Distribution::Uniform { lo: -0.25, hi: 0.5 },
                ),
            },
            Tol::witness(),
        )
        .expect("applies")
        .doc
    };
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let corrupt = text.replace("\"lo\": -0.25", "\"lo\": 0.125");
    assert_ne!(corrupt, text, "the corruption must land");
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Distribution { name, fault }) => {
            assert_eq!(name.0, "b");
            assert_eq!(
                fault,
                DistributionFault::NominalOutsideSupport { lo: 0.125, hi: 0.5 }
            );
        }
        other => panic!("must refuse typed, got {other:?}"),
    }
}

/// **The log door**: a corrupt distribution planted in a saved file's
/// EDIT LOG (not its snapshot) still refuses at LOAD — load replays
/// the log through the same edit door `apply` runs.
#[test]
fn a_corrupt_distribution_in_a_saved_edit_log_refuses_at_load() {
    let base = ProfileDoc::empty(DocumentId::derive("r1-corrupt-log"), Tol::witness());
    let edit = DocEdit::SetDocParam {
        name: ParamName::new("s"),
        value: DocParam::continuous_with(
            Dimension::Length,
            1.0,
            Distribution::Normal { sigma: 0.01 },
        ),
    };
    let text = save(&base, std::slice::from_ref(&edit), Tol::witness())
        .expect("a valid snapshot+log saves");
    let corrupt = text.replace("\"sigma\": 0.01", "\"sigma\": -2.0");
    assert_ne!(corrupt, text, "the corruption must land (in the LOG)");
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::EditReplay { index, error }) => {
            assert_eq!(index, 0);
            assert_eq!(
                error,
                EditError::InvalidDistribution {
                    name: ParamName::new("s"),
                    fault: DistributionFault::SigmaNotPositive { sigma: -2.0 },
                }
            );
        }
        other => panic!("a corrupt LOG must refuse at load, got {other:?}"),
    }
}

/// Strict serde: a misspelled form and a stray field both refuse to
/// PARSE — there is no lenient reading of an unknown distribution.
#[test]
fn unknown_forms_and_stray_fields_refuse_to_parse() {
    let text = save(&annotated_doc(0.01), &[], Tol::witness()).expect("saves");
    // A form the vocabulary does not have.
    let unknown_form = text.replace("\"Normal\"", "\"Gaussian\"");
    assert_ne!(unknown_form, text);
    assert!(
        load(&unknown_form, Tol::witness()).is_err(),
        "an unknown form must refuse, not load as something else"
    );
    // A stray field inside a known form (deny_unknown_fields).
    let stray = text.replace("\"sigma\": 0.01", "\"sigma\": 0.01, \"mu\": 3.0");
    assert_ne!(stray, text);
    assert!(
        load(&stray, Tol::witness()).is_err(),
        "a stray field must refuse (strict serde)"
    );
    // A distribution key on a COUNT param: the spelling does not
    // exist, so the file cannot say it (E11.3 by unrepresentability).
    let count_doc = {
        let doc = ProfileDoc::empty(DocumentId::derive("r1-corrupt-c"), Tol::witness());
        editor_core::apply(
            &doc,
            &DocEdit::SetDocParam {
                name: ParamName::new("n"),
                value: DocParam::Count { value: 3 },
            },
            Tol::witness(),
        )
        .expect("applies")
        .doc
    };
    let count_text = save(&count_doc, &[], Tol::witness()).expect("saves");
    let annotated_count = count_text.replace(
        "\"value\": 3",
        "\"value\": 3, \"distribution\": {\"Normal\": {\"sigma\": 1.0}}",
    );
    assert_ne!(annotated_count, count_text);
    assert!(
        load(&annotated_count, Tol::witness()).is_err(),
        "a Count parameter cannot even SPELL a distribution"
    );
}
