//! **R2 review probes for DOCM-5** — the subject door and DI3.
//!
//! `run_checks` refuses a `(doc, evaluation)` pair that is not a pair
//! (DI3): its gather is where the pairing is checked, and with the
//! default config every call reaches it. `run_checks_on` is the door
//! under it and takes the subject as an argument, so nothing on that
//! path reads the pairing at all — while `connectedness`, which the
//! door still runs, reads `doc.roots()` against `ev.value(root)` and
//! answers happily about the wrong document.
//!
//! `work/docm/pair-doors-outside-the-three-do-not-check-document-identity.md`
//! books the `run_checks` half of this and names THIS unit as the one
//! that would settle it, on the premise that "a resident handed a
//! subject never reads the evaluation itself". Connectedness does.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::corpus;
use crate::fixture;

use editor_core::{
    ChecksConfig, ChecksError, DocumentId, Evaluation, Node, ProfileDoc, Subject, product_recorded,
    run_checks, run_checks_on,
};
use fixture::{insert, len, on_frame, square};
use geom_core::Tol;

/// Two extruded squares, the second offset by `apart` in x. Node ids
/// are assigned in insertion order, so two of these under two document
/// ids are one recipe under two identities: every lookup a foreign
/// evaluation is asked for answers, which is the state DI3 exists for.
fn twin_pair(id: &str, apart: f64) -> ProfileDoc {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive(id), tol);
    let (doc, first) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile: first,
            distance: len(1.0),
        },
    );
    let (doc, second) = on_frame(
        doc,
        [apart, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: second,
            distance: len(1.0),
        },
    )
    .0
}

/// **RED PROBE — `run_checks_on` answers about another document.**
///
/// `overlapping` and `apart` are one recipe under two identities: the
/// same node ids, different geometry. The wrapper refuses the crossed
/// pair (`ChecksError::Product`, the gather's DI3 door). The door under
/// it takes the crossed pair and reports — over `apart`'s roots and
/// `overlapping`'s evaluation — with no refusal anywhere.
///
/// This row asserts the refusal DI3 would give and is RED on
/// `4c727c88`; it is a reviewer's probe, not a proposed fix.
#[test]
fn the_subject_door_refuses_an_evaluation_of_another_document() {
    let tol = Tol::witness();
    let cfg = ChecksConfig::default();
    let overlapping = twin_pair("docm5r2-di3-overlap", 0.4);
    let apart = twin_pair("docm5r2-di3-apart", 4.0);
    let ev_overlapping: Evaluation<f64> = corpus::eval(&overlapping);

    // The premise: the wrapper DOES refuse the crossed pair today.
    match run_checks(&apart, &ev_overlapping, &cfg, tol) {
        Err(ChecksError::Product { .. }) => {}
        other => panic!("the premise: the wrapper refuses the crossed pair, got {other:?}"),
    }

    // And the two documents really are different, so an answer about
    // one is a wrong answer about the other.
    let crossed = product_recorded(&overlapping, &ev_overlapping, tol).expect("the twin gathers");
    let honest = run_checks(&overlapping, &ev_overlapping, &cfg, tol).expect("its own report");
    assert!(
        !honest.findings.is_empty(),
        "the premise: the overlapping twin has something to report"
    );

    let smuggled = run_checks_on(
        &apart,
        &ev_overlapping,
        Subject::Product(&crossed),
        &cfg,
        tol,
    );
    match smuggled {
        Err(ChecksError::Product { .. }) => {}
        Ok(report) => panic!(
            "the door answered about another document: {} finding(s), \
             over `apart`'s roots and `overlapping`'s evaluation",
            report.findings.len()
        ),
        Err(other) => panic!("expected the pairing refusal, got {other}"),
    }
}

/// **RED PROBE — and the same with no subject at all.**
///
/// `Subject::NoBodyRoots` is the arm a caller reaches for when the
/// document denotes no body. Nothing ties it to `doc`, so it is also
/// the arm that skips the only resident that would have refused, and
/// the door then runs connectedness over a foreign evaluation.
#[test]
fn the_subject_door_refuses_a_foreign_evaluation_under_no_body_roots() {
    let tol = Tol::witness();
    let cfg = ChecksConfig::default();
    let a = twin_pair("docm5r2-di3-nb-a", 0.4);
    let b = twin_pair("docm5r2-di3-nb-b", 4.0);
    let ev_a: Evaluation<f64> = corpus::eval(&a);

    match run_checks_on(&b, &ev_a, Subject::NoBodyRoots, &cfg, tol) {
        Err(ChecksError::Product { .. }) => {}
        Ok(_) => panic!(
            "the door ran connectedness over an evaluation of another \
             document and reported"
        ),
        Err(other) => panic!("expected the pairing refusal, got {other}"),
    }
}
