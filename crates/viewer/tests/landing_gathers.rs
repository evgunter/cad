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
//! The counter is `cfg(debug_assertions)` and so is this suite, so
//! these rows are evidence only where assertions are on. That is every
//! build this repo produces, not only its test builds: the workspace's
//! `[profile.release]` keeps `debug-assertions` ON until publish. What
//! the gate buys is that cargo's own release defaults strip the counter
//! — for a consumer building the crate normally, and for this repo the
//! day that stanza comes out.
#![cfg(debug_assertions)]
// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use std::sync::Arc;

use pncad::document::{
    Dimension, Doc, Expr, Node, ProductError, ProfileProgram, gathers_on_this_thread,
};
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

/// **The landing's body is handed on, not gathered again.** The
/// display fit and the scene both want the aggregate the landing
/// already produced; asking for it costs nothing on the two paths
/// where the landing still owns it, which is what stops `Open` from
/// paying a second whole gather.
#[test]
fn asking_for_the_landed_body_costs_no_gather() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    assert_eq!(session.pump(), vec![Landing::Landed]);

    let before = gathers_on_this_thread();
    let body = session.landed_body().expect("the plate gathers");
    assert_eq!(
        gathers_on_this_thread() - before,
        0,
        "the landing's own gather is the only one"
    );
    assert!(body.solids().count() > 0, "and it is the drawable body");

    // A second asker is free too, which is the property that lets the
    // fit and a scene build share one gather.
    let before = gathers_on_this_thread();
    let again = session.landed_body().expect("still there");
    assert_eq!(gathers_on_this_thread() - before, 0);
    assert!(Arc::ptr_eq(&body, &again), "and it is the same body");
}

/// **A certified assembly hands its body back too.** The A5 gate
/// consumes the product it judges, and a certification returns the
/// same aggregate on its `Assembly` — so the one path that could have
/// lost the body to the gate does not.
#[test]
fn a_certified_assembly_keeps_the_body_the_gate_was_given() {
    let tol = Tol::witness();
    let bench = common::asm::bench("landed-body-assembly", tol);
    let mut session = common::asm::open_bench(&bench, tol);
    assert!(
        matches!(session.at_rest(), Some(AtRestBadge::Certified { .. })),
        "this row's premise is a certified gate: {:?}",
        session.at_rest()
    );

    let before = gathers_on_this_thread();
    assert!(session.landed_body().is_some(), "the aggregate is kept");
    assert_eq!(
        gathers_on_this_thread() - before,
        0,
        "the gate handed it back rather than eating it"
    );
}

/// **A gather refusal has no body to hand out, and asking does not
/// re-run the refusal.** `None` here means the pair has no product at
/// all, which is what `product_fault` is already saying.
#[test]
fn a_refused_gather_hands_out_no_body_and_gathers_nothing() {
    let tol = Tol::witness();
    let (doc, _extrude, _transform) = common::broken_document(tol);
    let mut session = DocSession::inline(doc, tol);
    assert_eq!(session.pump(), vec![Landing::Landed]);
    assert!(session.product_fault().is_some(), "the gather refuses");

    let before = gathers_on_this_thread();
    assert!(session.landed_body().is_none());
    assert_eq!(
        gathers_on_this_thread() - before,
        0,
        "a refusal is not re-derived by asking for its body"
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

/// **A4 — a gather refusal that is NOT a per-node failure lands with
/// no report.**
///
/// The row above uses a document whose ROOT failed, and the registry
/// refuses that on its own precondition — so it cannot see whether the
/// landing's `NoBodyRoots` filter is doing anything at all. This one
/// can: two `Transform`s of one extrude are two roots whose name rows
/// collide in the product table, every root evaluates, and the ONLY
/// thing that refuses is the gather. Widen the filter to `true` and
/// this row goes red where the other stays green.
#[test]
fn a_naming_collision_lands_with_a_fault_and_no_report() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("docm5-collision-landing", tol);
    let (doc, plane) = common::inserted(&doc, common::xy_frame(), tol);
    let (doc, profile) = common::inserted(&doc, common::square(plane, 0.02), tol);
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.02),
        },
        tol,
    );
    let moved = |doc: &Doc<ProfileProgram>, dx: f64| {
        common::inserted(
            doc,
            Node::Transform {
                input: extrude,
                translation: [common::len(dx), common::len(0.0), common::len(0.0)],
                rotation_axis: [common::scl(0.0), common::scl(0.0), common::scl(1.0)],
                rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite"),
            },
            tol,
        )
    };
    let (doc, _) = moved(&doc, 0.1);
    let (doc, _) = moved(&doc, 0.2);

    let mut session = DocSession::inline(doc, tol);
    assert_eq!(session.pump(), vec![Landing::Landed]);

    assert_eq!(gathers_of_one_landing(&mut session), 1);
    assert!(
        matches!(session.product_fault(), Some(ProductError::Naming { .. })),
        "the premise: only the gather refuses here, and it is a collision: {:?}",
        session.product_fault()
    );
    assert!(
        session
            .tree_rows()
            .iter()
            .all(|row| !matches!(row.status, viewer::tree::RowStatus::Failed { .. })),
        "and no node failed, so no other channel carries this"
    );
    assert!(
        session.checks().is_none(),
        "the registry has no subject and says so by reporting nothing"
    );
    assert!(
        viewer::frame::product_badge(session.product_fault()).is_some(),
        "this IS the fault channel's own case"
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

/// **Every site of the counter carries the gate attribute.**
///
/// WHAT THIS PINS AND WHAT IT DOES NOT, stated because the row's first
/// spelling claimed the second: it reads SOURCE, so it pins that the
/// cell, the increment and the reader each sit behind
/// `#[cfg(debug_assertions)]` and that there is no fourth site without
/// one. It says NOTHING about any built artifact — and it must not,
/// because this workspace's `[profile.release]` keeps
/// `debug-assertions` ON until publish (`Cargo.toml`, and `demos/tour`
/// the same), so the counter IS in every binary this repo produces.
/// The gate's payoff is cargo's own release defaults, and the day the
/// stanza comes out.
///
/// The behavioural half — that the counter counts what it claims to —
/// is the rows above, which read it across a real landing.
///
/// The read goes through the shared reader's CODE view
/// ([`test_utils::source::code_only`]): every anchor below is a code
/// fragment — an attribute above the item it gates — and the count
/// beside them is a count of SITES, so a mention of the cell in a
/// comment must not answer for one and must not inflate the total.
/// The view keeps every code byte at its own offset, so the three
/// anchors and the count are exactly what the raw text offered. This
/// site's ledger row is in `crates/test-utils/tests/reader_census.rs`.
#[test]
fn every_site_of_the_gather_counter_carries_the_debug_gate() {
    let source = test_utils::source::code_only(include_str!("../../editor-core/src/product.rs"));
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
