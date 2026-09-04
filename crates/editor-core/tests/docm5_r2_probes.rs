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

/// **RED PROBE — `run_checks` with the separation resident OFF now
/// refuses a document it used to report on.**
///
/// Before the subject door, the gather lived inside the separation
/// resident, so a config that turned that resident off never reached
/// it: the registry answered `Ok`, with `Separation` in `skipped`.
/// The wrapper gathers before it reads the config, so the same call
/// now raises `ChecksError::Product` — an observable change at a
/// public door, on a configuration the DOCM-4 review already had a
/// probe for.
#[test]
fn the_wrapper_with_separation_off_does_not_need_a_subject() {
    let tol = Tol::witness();
    let cfg = ChecksConfig {
        separation: editor_core::Advisory::Off,
        ..ChecksConfig::default()
    };
    let a = twin_pair("docm5r2-off-a", 0.4);
    let b = twin_pair("docm5r2-off-b", 4.0);
    let ev_a: Evaluation<f64> = corpus::eval(&a);

    let report = run_checks(&b, &ev_a, &cfg, tol)
        .expect("with the resident that needs a subject off, there is nothing to gather for");
    assert!(
        report.skipped.contains(&editor_core::CheckId::Separation),
        "and the skip is visible"
    );
}

/// **MEASURED — `run_checks` with the separation resident OFF pays for
/// a gather nothing reads.**
///
/// The gather is ~30x the registry (the unit's own measurement). With
/// `Separation: Off` the only consumer of the subject is skipped, so
/// the whole of that ~30x is waste — on the same document the unit
/// states its numbers for. Reported as a ratio against the same call
/// with the resident on, so the row does not pin a millisecond.
#[test]
fn the_wrapper_with_separation_off_still_pays_for_the_gather() {
    use std::time::Instant;
    let tol = Tol::witness();
    let doc = editor_core::apply(
        &corpus::documents()
            .into_iter()
            .find(|d| d.name == "heat_sink")
            .expect("the corpus carries the heat sink")
            .doc,
        &editor_core::DocEdit::SetDocParam {
            name: editor_core::ParamName::new("fins"),
            value: editor_core::DocParam::Count { value: 40 },
        },
        tol,
    )
    .expect("the fin count is a document parameter")
    .doc;
    let ev: Evaluation<f64> = corpus::eval(&doc);
    let off = ChecksConfig {
        separation: editor_core::Advisory::Off,
        ..ChecksConfig::default()
    };

    let subject = product_recorded(&doc, &ev, tol).expect("the heat sink gathers");
    let t0 = Instant::now();
    run_checks_on(&doc, &ev, Subject::Product(&subject), &off, tol).expect("the door runs");
    let door = t0.elapsed();
    let t0 = Instant::now();
    run_checks(&doc, &ev, &off, tol).expect("and so does the wrapper");
    let wrapper = t0.elapsed();

    println!(
        "separation off: door {:?}, wrapper {:?} ({:.1}x)",
        door,
        wrapper,
        wrapper.as_secs_f64() / door.as_secs_f64().max(f64::MIN_POSITIVE)
    );
    assert!(
        wrapper < door * 4,
        "the wrapper pays a gather no resident reads: door {door:?}, wrapper {wrapper:?}"
    );
}
