//! **R2 review probes for DOCM-5** — the landing's observables against
//! the spelling they replaced.
//!
//! `land` used to compute its three results independently: `product`
//! for the fault, `run_checks` for the report, `assemble` for the
//! badge. It now computes them from ONE gather in a fixed order. The
//! claim under review is that nothing observable moved, and the way to
//! falsify it is to keep the old spelling as an ORACLE and compare it
//! against the session's fields on documents the unit did not choose.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use std::sync::Arc;

use pncad::document::{
    ChecksConfig, ChecksReport, Doc, DocEdit, Evaluation, Node, ProfileProgram,
    apply, assemble, product, run_checks,
};
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use viewer::evalseam::EvalDone;
use viewer::session::{AtRestBadge, DocSession, Landing, SessionOp};

/// The three observables, as the MERGE BASE computed them.
#[derive(Debug, PartialEq)]
struct Observables {
    fault: Option<String>,
    checks: Option<ChecksReport>,
    at_rest: Option<AtRestBadge>,
}

/// The old spelling, transcribed: `product` for the fault,
/// `run_checks` for the report, `assemble` behind the assembly-shape
/// test for the badge.
fn oracle(doc: &Doc<ProfileProgram>, ev: &Evaluation<f64>, tol: Tol) -> Observables {
    let assembly_shaped = doc
        .order()
        .iter()
        .any(|&id| matches!(doc.node(id), Some(Node::InstantiatePart { .. })));
    Observables {
        fault: product(doc, ev, tol).err().map(|e| e.to_string()),
        checks: run_checks(doc, ev, &ChecksConfig::default(), tol).ok(),
        at_rest: assembly_shaped.then(|| match assemble(doc, ev, tol) {
            Ok(assembly) => AtRestBadge::Certified {
                minted: assembly.minted.len(),
            },
            Err(refusal) => AtRestBadge::Refused {
                message: refusal.to_string(),
            },
        }),
    }
}

/// What the session actually landed.
fn landed(session: &DocSession) -> Observables {
    Observables {
        fault: session.product_fault().map(ToString::to_string),
        checks: session.checks().cloned(),
        at_rest: session.at_rest().cloned(),
    }
}

/// Assert the landing equals the old spelling, and answer the number of
/// gathers one `land` of the same result costs.
fn agrees_with_the_old_spelling(label: &str, session: &mut DocSession) -> u64 {
    let tol = session.tol();
    let evaluation = Arc::clone(
        session
            .evaluation_arc()
            .expect("a result has already landed"),
    );
    let want = oracle(session.doc(), &evaluation, tol);
    assert_eq!(landed(session), want, "{label}");

    let generation = session
        .landed_generation()
        .expect("the session knows its generation");
    let before = gathers();
    assert_eq!(
        session.land(EvalDone {
            generation,
            evaluation,
        }),
        Landing::Landed,
        "{label}: the re-land is a landing"
    );
    let cost = gathers() - before;
    assert_eq!(landed(session), want, "{label}: and again after re-landing");
    cost
}

#[cfg(debug_assertions)]
fn gathers() -> u64 {
    pncad::document::gathers_on_this_thread()
}

#[cfg(not(debug_assertions))]
fn gathers() -> u64 {
    1
}

/// Two extruded squares in one document, `apart` metres apart in x —
/// one root each, so the product holds two solids and the separation
/// resident's pair walk actually runs. At `apart` smaller than the
/// side the two interpenetrate, which is the finding.
fn two_squares(label: &str, side: f64, apart: f64, tol: Tol) -> Doc<ProfileProgram> {
    let doc: Doc<ProfileProgram> = Doc::empty_derived(label, tol);
    let (doc, plane) = common::inserted(&doc, common::xy_frame(), tol);
    let (doc, first) = common::inserted(&doc, common::square(plane, side), tol);
    let (doc, _) = common::inserted(
        &doc,
        Node::Extrude {
            profile: first,
            distance: common::len(side),
        },
        tol,
    );
    let (doc, plane_b) = common::inserted(
        &doc,
        common::frame([apart, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        tol,
    );
    let (doc, second) = common::inserted(&doc, common::square(plane_b, side), tol);
    let (doc, _) = common::inserted(
        &doc,
        Node::Extrude {
            profile: second,
            distance: common::len(side),
        },
        tol,
    );
    doc
}

/// **C1/C4 — five part-shaped landings the unit did not choose.**
///
/// Two disjoint solids (the pair walk runs and finds nothing), two
/// interpenetrating ones (the pair walk finds something, so the report
/// compared is a non-empty one), a document that denotes no body, and
/// a document whose root fails. Each is asserted against the old
/// spelling and each must cost one gather.
#[test]
fn part_landings_match_the_old_spelling_on_one_gather() {
    let tol = Tol::witness();
    let mut any_finding = false;
    for (label, doc) in [
        ("disjoint", two_squares("docm5r2-disjoint", 0.02, 0.1, tol)),
        ("overlapping", two_squares("docm5r2-overlap", 0.02, 0.01, tol)),
        (
            "no body",
            Doc::<ProfileProgram>::empty_derived("docm5r2-empty", tol),
        ),
        ("broken", common::broken_document(tol).0),
        ("plate", common::parametric_plate(tol).0),
    ] {
        let mut session = DocSession::inline(doc, tol);
        assert_eq!(session.pump(), vec![Landing::Landed], "{label}");
        assert_eq!(
            agrees_with_the_old_spelling(label, &mut session),
            1,
            "{label}: one gather per landing"
        );
        any_finding |= session
            .checks()
            .is_some_and(|report| !report.findings.is_empty());
    }
    assert!(
        any_finding,
        "the premise: one of these documents must actually make the \
         registry report a finding, or the report equality is vacuous"
    );
}

/// **C1/C3/C4 — an assembly-shaped landing with a MINT REFUSAL.**
///
/// A `Tangent` mate is a class the table gives no at-rest record, so
/// the gather records it `unminted` and the gate raises it. That is
/// the arm `assemble_gathered` inherited from `assemble`, and no row
/// in the unit takes it through the landing.
#[test]
fn an_assembly_with_a_mint_refusal_matches_the_old_spelling_on_one_gather() {
    let tol = Tol::witness();
    let bench = common::asm::bench("docm5r2-mint", tol);
    let mut session = common::asm::open_bench(&bench, tol);
    assert_eq!(
        agrees_with_the_old_spelling("certified", &mut session),
        1,
        "the mate-less assembly"
    );

    session.perform(SessionOp::AddMate {
        a: common::asm::in_part(bench.post_b, &bench.post_top),
        b: common::asm::in_part(bench.shelf_i, &bench.shelf_bottom),
        class: ContactClass::Tangent,
        alignment: common::asm::seat_alignment(0.0, None),
    });
    session.pump();
    match session.at_rest() {
        Some(AtRestBadge::Refused { message }) => {
            assert!(message.contains("no at-rest kernel record"), "{message}");
        }
        other => panic!("the mint refusal must red the badge, got {other:?}"),
    }
    assert_eq!(
        agrees_with_the_old_spelling("mint refusal", &mut session),
        1,
        "the refusing assembly still lands on one gather"
    );
}

/// **C1/C4 — an assembly-shaped document whose GATHER refuses.**
///
/// An `InstantiatePart` with no resolver behind it fails to evaluate,
/// so the document is assembly-shaped AND has no product: the badge
/// arm and the fault arm are both live at once. The old spelling ran
/// `assemble` for the badge and got `AssemblyError::Product`; the new
/// one renders the same sentence from a refusal it holds itself.
#[test]
fn an_assembly_shaped_gather_refusal_matches_the_old_spelling() {
    let tol = Tol::witness();
    let bench = common::asm::bench("docm5r2-unresolved", tol);
    let doc: Doc<ProfileProgram> = Doc::empty_derived("docm5r2-unresolved-asm", tol);
    let doc = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::instantiate_part(bench.post.clone()),
        },
        tol,
    )
    .expect("the instantiate node inserts")
    .doc;

    let mut session = DocSession::inline(doc, tol);
    assert_eq!(session.pump(), vec![Landing::Landed]);
    assert!(
        session.product_fault().is_some(),
        "the premise: an unresolvable instance does not gather"
    );
    assert!(
        matches!(session.at_rest(), Some(AtRestBadge::Refused { .. })),
        "and the document is still assembly-shaped: {:?}",
        session.at_rest()
    );
    assert_eq!(
        agrees_with_the_old_spelling("unresolved assembly", &mut session),
        1,
    );
}
