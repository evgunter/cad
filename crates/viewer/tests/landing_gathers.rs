//! **One gather per landing**, and the landing's observables around it.
//!
//! `DocSession::land` has three consumers of the document's product —
//! the product's own verdict, the advisory registry, and the A5 badge —
//! and each used to derive it for itself. What replaced that is an
//! ORDER (fault, registry, badge) in which one gather is enough,
//! because only the last of the three consumes the product. An order is
//! not a thing a reader can see holding, so it is counted: the gather
//! carries a debug-only counter and these rows read the DIFFERENCE
//! across one `land`.
//!
//! The counter is `cfg(debug_assertions)` and so is this suite. A
//! release build carries neither, which is the point of the shape — but
//! it also means these rows are evidence only where assertions are on,
//! and the suite says so rather than vanishing quietly.
#![cfg(debug_assertions)]
// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use std::sync::Arc;

use pncad::document::{Doc, ProductError, ProfileProgram, gathers_on_this_thread};
use pncad::geom_core::Tol;
use viewer::evalseam::EvalDone;
use viewer::session::{AtRestBadge, DocSession, Landing};

/// Re-land the result a session already holds, and answer how many
/// times the gather ran while it did.
///
/// Landing a result the session has already landed is the same call on
/// the same inputs — `land` is a pure fold of a finished evaluation
/// into the session — so the count is `land`'s alone. Counting across
/// `pump` instead would fold in the EVALUATION's gathers, and an
/// assembly's evaluation gathers each instantiated part's own product
/// at the seam, which is a different document's and not this claim.
fn gathers_of_one_landing(session: &mut DocSession) -> u64 {
    let evaluation = Arc::clone(
        session
            .evaluation_arc()
            .expect("a result has already landed"),
    );
    let generation = session
        .landed_generation()
        .expect("and the session knows its generation");
    let before = gathers_on_this_thread();
    assert_eq!(
        session.land(EvalDone {
            generation,
            evaluation,
        }),
        Landing::Landed,
        "the re-land is a landing"
    );
    gathers_on_this_thread() - before
}

/// **A1 — a part document lands on one gather**, and carries all three
/// of the landing's results from it: no fault, a report, and no badge
/// (a part document is not assembly-shaped).
#[test]
fn a_part_document_lands_on_one_gather() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    assert_eq!(session.pump(), vec![Landing::Landed]);

    assert_eq!(gathers_of_one_landing(&mut session), 1);
    assert!(session.product_fault().is_none(), "the product gathers");
    assert!(session.checks().is_some(), "and the registry reports");
    assert!(session.at_rest().is_none(), "a part has no A5 badge");
}

/// **A1 — an assembly-shaped document lands on one gather too**, and
/// that is the case the order is FOR: the badge consumes the product,
/// so it goes last and the registry reads it first.
#[test]
fn an_assembly_shaped_document_lands_on_one_gather() {
    let tol = Tol::witness();
    let bench = common::asm::bench("docm5-one-gather", tol);
    let mut session = common::asm::open_bench(&bench, tol);

    assert_eq!(gathers_of_one_landing(&mut session), 1);
    assert!(session.product_fault().is_none(), "the product gathers");
    assert!(session.checks().is_some(), "the registry reports");
    assert!(
        matches!(session.at_rest(), Some(AtRestBadge::Certified { .. })),
        "and the A5 badge is taken: {:?}",
        session.at_rest()
    );
}

/// **A4 — a gather refusal lands as it always did**: the fault is the
/// gather's own refusal, the report is ABSENT rather than clean ("not
/// checked" is not "checked and fine"), and the landing still costs one
/// gather.
#[test]
fn a_gather_refusal_lands_with_a_fault_and_no_report() {
    let tol = Tol::witness();
    let (doc, _extrude, _moved) = common::broken_document(tol);
    let mut session = DocSession::inline(doc, tol);
    assert_eq!(session.pump(), vec![Landing::Landed]);

    assert_eq!(gathers_of_one_landing(&mut session), 1);
    assert!(
        session.product_fault().is_some(),
        "a root that did not evaluate is a gather refusal"
    );
    assert!(
        session.checks().is_none(),
        "and the registry has no subject, so there is no report"
    );
    assert!(session.at_rest().is_none(), "a part has no A5 badge");
}

/// **A4 — a document that denotes no body is not a refusal.** The
/// gather says `NoBodyRoots`, which the landing reads as a SUBJECT: the
/// registry runs over it and reports clean. The fault field still
/// carries the gather's answer, and the badge channel is the one that
/// keeps quiet about it (`frame::product_badge`).
#[test]
fn a_document_with_no_body_lands_a_clean_report() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("docm5-empty-landing", tol);
    let mut session = DocSession::inline(doc, tol);
    assert_eq!(session.pump(), vec![Landing::Landed]);

    assert_eq!(gathers_of_one_landing(&mut session), 1);
    assert!(
        matches!(session.product_fault(), Some(ProductError::NoBodyRoots)),
        "the gather's own answer: {:?}",
        session.product_fault()
    );
    assert!(
        viewer::frame::product_badge(session.product_fault()).is_none(),
        "which the badge channel deliberately stays quiet about"
    );
    let report = session.checks().expect("the registry still reports");
    assert!(report.findings.is_empty(), "cleanly: {report}");
    assert!(session.at_rest().is_none(), "and there is no badge");
}

/// **The counter is debug-only.** `product.rs` gates the cell, the
/// increment and the reader, so a release build carries none of the
/// three — the witness costs the shipped kernel nothing.
#[test]
fn the_gather_counter_is_gated_out_of_release_builds() {
    let source = include_str!("../../editor-core/src/product.rs");
    let gated = [
        "#[cfg(debug_assertions)]\nthread_local! {\n    static GATHERS",
        "#[cfg(debug_assertions)]\n    GATHERS.with(",
        "#[cfg(debug_assertions)]\n#[must_use]\npub fn gathers_on_this_thread",
    ];
    for anchor in gated {
        assert!(
            source.contains(anchor),
            "the counter's three sites are each behind the gate; missing: {anchor}"
        );
    }
    assert_eq!(
        source.matches("GATHERS").count(),
        gated.len(),
        "and there is no fourth site the gate does not cover"
    );
}
