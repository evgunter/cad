//! **Distributions in the document** (ERROR-DESIGN E1/E2) — schema
//! **v15**, and the gate that refuses everything older.
//!
//! Before v15 a document parameter could not carry a `distribution`
//! key at all. A v14 reader handed one meets a field its
//! `deny_unknown_fields` document types have no name for and dies
//! inside serde rather than at the version door — which is exactly the
//! direction the gate buys. The other direction is forgiving by
//! construction (a v14 file declares no distributions, and an
//! unannotated v15 param writes no key), so the disposition is the
//! family's: the older file refuses TYPED with the regenerate
//! recourse, and the migration table stays empty.
//!
//! **Why 15.** Read by eye from main's constant at the final re-merge
//! (`git show origin/main:crates/editor-core/src/persist/mod.rs | grep
//! SCHEMA_VERSION`), because units have repeatedly had a same-number
//! claim merge CLEAN — both sides write the identical line, so git
//! never conflicts.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    Dimension, Distribution, DistributionFault, DistributionField, DocEdit, DocParam, DocumentId,
    EditError, ParamName, PersistError, ProfileDoc, REGENERATE_RECOURSE, SCHEMA_VERSION, apply,
    load, save,
};
use geom_core::Tol;

/// The prior live golden, kept as the REFUSAL fixture: a break nobody
/// can demonstrate is a break nobody can trust.
const V14: &str = include_str!("golden/v14_golden.cad");
/// One further back, to show the gate has no notion of "nearly
/// current".
const V13: &str = include_str!("golden/v13_golden.cad");

#[test]
fn schema_version_is_current() {
    // Named for the PROPERTY, not the number (the `lbret_schema_v8`
    // precedent): M10-1's own bump was v15; LIB-G16 took v16 for the
    // chamfer recipe node, and the number is what keeps moving.
    assert_eq!(SCHEMA_VERSION, 17);
}

#[test]
fn the_checked_in_older_goldens_are_really_older() {
    assert_eq!(V14.lines().next(), Some("schema: 14"));
    assert_eq!(V13.lines().next(), Some("schema: 13"));
}

/// The break, demonstrated in the direction that matters: a v14 file
/// refuses TYPED at the version door, naming the version found, the
/// version supported, and the step that does not exist.
#[test]
fn v14_refuses_too_old() {
    match load(V14, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 14);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(
                missing, 14,
                "the 14 → 15 step is the one that does not exist"
            );
        }
        other => panic!("v14 must refuse SchemaTooOld, got {other:?}"),
    }
}

#[test]
fn the_refusal_carries_the_regenerate_recourse() {
    for (label, bytes) in [("v14", V14), ("v13", V13)] {
        let msg = match load(bytes, Tol::witness()) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("{label} must refuse"),
        };
        assert!(msg.contains(REGENERATE_RECOURSE), "{label}: {msg}");
    }
}

fn doc_with(params: &[(&str, DocParam)]) -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-1-schema"), Tol::witness());
    for (name, value) in params {
        doc = apply(
            &doc,
            &DocEdit::SetDocParam {
                name: ParamName::new(*name),
                value: value.clone(),
            },
            Tol::witness(),
        )
        .expect("a valid parameter sets")
        .doc;
    }
    doc
}

fn annotated(value: f64, distribution: Distribution) -> DocParam {
    DocParam::Continuous {
        dim: Dimension::Length,
        value,
        distribution: Some(distribution),
    }
}

/// All FOUR forms on the wire at once, round-tripped bit for bit.
#[test]
fn every_distribution_form_round_trips_at_v15() {
    let doc = doc_with(&[
        (
            "band",
            annotated(1.0, Distribution::Band { lo: -0.1, hi: 0.1 }),
        ),
        (
            "uniform",
            annotated(2.0, Distribution::Uniform { lo: -0.2, hi: 0.05 }),
        ),
        (
            "normal",
            annotated(3.0, Distribution::Normal { sigma: 0.01 }),
        ),
        (
            "truncated",
            annotated(
                4.0,
                Distribution::TruncatedNormal {
                    sigma: 0.01,
                    lo: -0.03,
                    hi: 0.03,
                },
            ),
        ),
    ]);
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert_eq!(
        text.lines().next(),
        Some(&format!("schema: {SCHEMA_VERSION}")[..]),
        "a fresh save carries the CURRENT version"
    );
    for form in ["Band", "Uniform", "Normal", "TruncatedNormal"] {
        assert!(text.contains(form), "{form} is on the wire: {text}");
    }
    let back = load(&text, Tol::witness()).expect("loads").doc;
    assert!(back.bit_eq(&doc), "every form round-trips bit for bit");
}

/// INVARIANT: an unannotated parameter writes NO key — the degenerate
/// carry. A document that declares no distribution pays no bytes for
/// the feature, which is why an all-`None` v15 body is byte-identical
/// to the v14 one.
#[test]
fn an_unannotated_param_stays_absent_from_the_wire() {
    let doc = doc_with(&[("plain", DocParam::continuous(Dimension::Length, 1.0))]);
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert!(
        !text.contains("distribution"),
        "an unannotated param says nothing: {text}"
    );
}

/// `-0.0` is a different offset from `0.0`, and the format keeps it:
/// the D7 replay identity reaches into the new field.
#[test]
fn distribution_offsets_round_trip_by_bits() {
    let doc = doc_with(&[(
        "signed_zero",
        annotated(1.0, Distribution::Band { lo: -0.0, hi: 0.5 }),
    )]);
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let back = load(&text, Tol::witness()).expect("loads").doc;
    assert!(back.bit_eq(&doc));
    let other = doc_with(&[(
        "signed_zero",
        annotated(1.0, Distribution::Band { lo: 0.0, hi: 0.5 }),
    )]);
    assert!(
        !back.bit_eq(&other),
        "-0.0 and 0.0 are different offsets to `bit_eq`"
    );
}

/// The LOAD door refuses a hand-written file whose distribution breaks
/// an invariant, with the diagnostics the SAVE door refuses with — the
/// shared validator, not two mirrored door sets.
#[test]
fn a_hand_written_negative_sigma_refuses_at_load() {
    let doc = doc_with(&[("s", annotated(1.0, Distribution::Normal { sigma: 0.01 }))]);
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let corrupt = text.replace("\"sigma\": 0.01", "\"sigma\": -1.0");
    assert_ne!(corrupt, text, "the corruption must actually land");
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Distribution { name, fault }) => {
            assert_eq!(name.0, "s");
            assert_eq!(fault, DistributionFault::SigmaNotPositive { sigma: -1.0 });
        }
        other => panic!("a negative sigma must refuse at LOAD, got {other:?}"),
    }
}

/// The SAVE half — what makes "a document that would refuse to load
/// cannot be saved" true rather than aspirational. The corrupt
/// snapshot is deserialized DIRECTLY, past both the edit door and the
/// load door's validator, and then handed to `save`, which refuses it
/// with the same typed fault the load door raised above.
#[test]
fn the_same_document_refuses_at_save() {
    let doc = doc_with(&[("s", annotated(1.0, Distribution::Normal { sigma: 0.01 }))]);
    let text = save(&doc, &[], Tol::witness()).expect("the healthy document saves");
    let corrupt = text.replace("\"sigma\": 0.01", "\"sigma\": -1.0");
    assert_ne!(corrupt, text, "the corruption must actually land");
    let body: serde_json::Value = serde_json::from_str(
        corrupt
            .splitn(3, '\n')
            .nth(2)
            .expect("the JSON body follows the two header lines"),
    )
    .expect("the corrupt body is still JSON");
    let broken: ProfileDoc =
        serde_json::from_value(body["snapshot"].clone()).expect("and still a document");
    match save(&broken, &[], Tol::witness()) {
        Err(PersistError::Distribution { name, fault }) => {
            assert_eq!(name.0, "s");
            assert_eq!(fault, DistributionFault::SigmaNotPositive { sigma: -1.0 });
        }
        other => panic!("save must refuse the same fault, got {other:?}"),
    }
}

/// An UNAPPLIED edit log is DATA, and a log carrying a broken
/// distribution never reaches disk: save replays the log through the
/// edit door before writing a byte.
#[test]
fn a_broken_distribution_in_the_edit_log_refuses_at_save() {
    let doc = ProfileDoc::empty(DocumentId::derive("m10-1-log"), Tol::witness());
    let bad = DocEdit::SetDocParam {
        name: ParamName::new("s"),
        value: annotated(1.0, Distribution::Normal { sigma: -1.0 }),
    };
    match save(&doc, &[bad], Tol::witness()) {
        Err(PersistError::EditReplay { index, error }) => {
            assert_eq!(index, 0);
            assert_eq!(
                error,
                EditError::InvalidDistribution {
                    name: ParamName::new("s"),
                    fault: DistributionFault::SigmaNotPositive { sigma: -1.0 },
                }
            );
        }
        other => panic!("an unreplayable log must refuse at save, got {other:?}"),
    }
}

/// The edit door refuses every §2 invariant, typed and by name.
#[test]
fn the_edit_door_refuses_each_broken_invariant() {
    let doc = ProfileDoc::empty(DocumentId::derive("m10-1-edit"), Tol::witness());
    let set = |d: Distribution| {
        apply(
            &doc,
            &DocEdit::SetDocParam {
                name: ParamName::new("p"),
                value: annotated(1.0, d),
            },
            Tol::witness(),
        )
        .map(|_| ())
    };
    assert_eq!(
        set(Distribution::Normal { sigma: 0.0 }),
        Err(EditError::InvalidDistribution {
            name: ParamName::new("p"),
            fault: DistributionFault::SigmaNotPositive { sigma: 0.0 },
        })
    );
    assert_eq!(
        set(Distribution::Band { lo: 0.1, hi: 0.2 }),
        Err(EditError::InvalidDistribution {
            name: ParamName::new("p"),
            fault: DistributionFault::NominalOutsideSupport { lo: 0.1, hi: 0.2 },
        })
    );
    assert_eq!(
        set(Distribution::Uniform { lo: -0.2, hi: -0.1 }),
        Err(EditError::InvalidDistribution {
            name: ParamName::new("p"),
            fault: DistributionFault::NominalOutsideSupport { lo: -0.2, hi: -0.1 },
        })
    );
    assert_eq!(
        set(Distribution::TruncatedNormal {
            sigma: f64::NAN,
            lo: -0.1,
            hi: 0.1
        }),
        Err(EditError::NonFiniteDocParam {
            name: ParamName::new("p"),
        }),
        "a non-finite offset joins the non-finite class, not the shape class"
    );
    assert_eq!(
        set(Distribution::Band {
            lo: f64::NEG_INFINITY,
            hi: 0.1
        }),
        Err(EditError::NonFiniteDocParam {
            name: ParamName::new("p"),
        })
    );
    assert!(
        Distribution::Band { lo: -0.1, hi: 0.0 }.check().is_ok(),
        "asymmetric bounds are legal, including a one-sided band"
    );
    assert_eq!(
        DistributionField::Sigma.to_string(),
        "sigma",
        "the field vocabulary reads in the author's words"
    );
}

/// **Two faults at once, ONE answer, at both doors.** A parameter that
/// is `Continuous` with `dim: Count` AND carries a broken distribution
/// is corrupt twice over, and the two doors used to disagree about
/// which fault it was: the edit door checked the structural `dim`
/// first, while the load door runs the float walk and the distribution
/// walk before the snapshot invariants where the `dim` fault lives.
/// The edit door now runs the LOAD door's order — floats, then
/// distribution shape, then the structural fault — so a caller
/// comparing an edit refusal against a load refusal for the same
/// parameter sees the same class.
///
/// The doubly-corrupt case is the only one where the order is
/// observable, which is exactly why it needs pinning: no singly-corrupt
/// row can notice it.
#[test]
fn a_doubly_corrupt_param_names_the_same_fault_at_both_doors() {
    let doc = ProfileDoc::empty(DocumentId::derive("m10-1-precedence"), Tol::witness());
    let name = ParamName::new("s");
    let broken = DocParam::Continuous {
        dim: Dimension::Count,
        value: 1.0,
        distribution: Some(Distribution::Normal { sigma: -1.0 }),
    };
    assert_eq!(
        apply(
            &doc,
            &DocEdit::SetDocParam {
                name: name.clone(),
                value: broken.clone(),
            },
            Tol::witness(),
        )
        .map(|_| ()),
        Err(EditError::InvalidDistribution {
            name: name.clone(),
            fault: DistributionFault::SigmaNotPositive { sigma: -1.0 },
        }),
        "the distribution walk runs before the structural check, as it does at load"
    );
    // The same doubly-corrupt parameter through the SAVE validator,
    // built by deserializing a corrupted body past both other doors —
    // the technique the save-door row above uses, for the same reason.
    let healthy = doc_with(&[("s", annotated(1.0, Distribution::Normal { sigma: 0.01 }))]);
    let text = save(&healthy, &[], Tol::witness()).expect("the healthy document saves");
    let corrupt = text
        .replace("\"sigma\": 0.01", "\"sigma\": -1.0")
        .replace("\"dim\": \"Length\"", "\"dim\": \"Count\"");
    assert_ne!(corrupt, text, "the corruption must actually land");
    let body: serde_json::Value = serde_json::from_str(
        corrupt
            .splitn(3, '\n')
            .nth(2)
            .expect("the JSON body follows the two header lines"),
    )
    .expect("the corrupt body is still JSON");
    let doubly: ProfileDoc =
        serde_json::from_value(body["snapshot"].clone()).expect("and still a document");
    match save(&doubly, &[], Tol::witness()) {
        Err(PersistError::Distribution { name: n, fault }) => {
            assert_eq!(n, name, "the same parameter");
            assert_eq!(fault, DistributionFault::SigmaNotPositive { sigma: -1.0 });
        }
        other => panic!("expected the distribution fault, got {other:?}"),
    }
}

/// **The non-finite walk names the FIELD.** A distribution offset that
/// is not finite joins the same non-finite class every other float in
/// the document belongs to — and the site now says WHICH offset,
/// because the walk has to find one in order to answer at all. An
/// unapplied edit log is the door for this: a log is data, so it can
/// carry a parameter no edit door would have accepted.
#[test]
fn a_non_finite_offset_names_which_offset_it_was() {
    use editor_core::persist::NonFiniteSite;
    let doc = ProfileDoc::empty(DocumentId::derive("m10-1-nonfinite"), Tol::witness());
    let cases = [
        (
            Distribution::TruncatedNormal {
                sigma: f64::NAN,
                lo: -1.0,
                hi: 2.0,
            },
            DistributionField::Sigma,
        ),
        (
            Distribution::TruncatedNormal {
                sigma: 0.5,
                lo: f64::NEG_INFINITY,
                hi: 2.0,
            },
            DistributionField::Lo,
        ),
        (
            Distribution::Band {
                lo: -1.0,
                hi: f64::INFINITY,
            },
            DistributionField::Hi,
        ),
    ];
    for (dist, expected) in cases {
        let edit = DocEdit::SetDocParam {
            name: ParamName::new("p"),
            value: annotated(1.0, dist),
        };
        match save(&doc, &[edit], Tol::witness()) {
            Err(PersistError::NonFinite {
                site: NonFiniteSite::Edit { index: 0, inner },
            }) => match *inner {
                NonFiniteSite::DocParam { ref name, field } => {
                    assert_eq!(name.0, "p");
                    assert_eq!(field, Some(expected), "the site names the offending offset");
                }
                ref other => panic!("expected a doc-param site, got {other:?}"),
            },
            other => panic!("expected a non-finite refusal for {dist:?}, got {other:?}"),
        }
    }
    // The NOMINAL's own non-finiteness is the same class with no field
    // to name — the distinction the option carries.
    let edit = DocEdit::SetDocParam {
        name: ParamName::new("p"),
        value: DocParam::continuous(Dimension::Length, f64::NAN),
    };
    match save(&doc, &[edit], Tol::witness()) {
        Err(PersistError::NonFinite {
            site: NonFiniteSite::Edit { inner, .. },
        }) => assert!(
            matches!(*inner, NonFiniteSite::DocParam { field: None, .. }),
            "a broken nominal names no distribution field"
        ),
        other => panic!("expected a non-finite refusal, got {other:?}"),
    }
}

/// A param differing ONLY in distribution is a reported diff — the
/// document diff sees the new field, through the same `bit_eq` the
/// replay identity uses.
#[test]
fn diff_reports_a_distribution_only_change() {
    let plain = doc_with(&[("p", DocParam::continuous(Dimension::Length, 1.0))]);
    let annotated_doc = doc_with(&[("p", annotated(1.0, Distribution::Normal { sigma: 0.01 }))]);
    let d = plain.diff(&annotated_doc);
    assert_eq!(
        d.params,
        vec![ParamName::new("p")],
        "adding a distribution is a param diff"
    );
    assert!(!plain.bit_eq(&annotated_doc));
    let widened = doc_with(&[("p", annotated(1.0, Distribution::Normal { sigma: 0.02 }))]);
    assert_eq!(
        annotated_doc.diff(&widened).params,
        vec![ParamName::new("p")],
        "changing a distribution is a param diff"
    );
}
