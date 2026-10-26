//! Review probes for PR 5772 (error precedence), not for merge. Run with
//! and without `REVIEW_FAULT_JUDGEMENT=0` (a naming fault injected at the
//! first judgement's naming, on base and head alike).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use crate::blocks::{block, block_on, failure, run};
use editor_core::{Bodies, Node, ProfileDoc};
use geom_core::Tol;

fn outcome(doc: &ProfileDoc, node: editor_core::RecipeNodeId) -> String {
    let ev = run(doc);
    match failure(&ev, node) {
        Some(e) => format!("REFUSED {e:?}").chars().take(400).collect(),
        None => match ev.value(node).map(|v| &v.payload) {
            Some(editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Empty)) => {
                "EMPTY".into()
            }
            Some(_) => "BUILT".into(),
            None => "NO VALUE".into(),
        },
    }
}

/// Three blocks in a row: 0 and 1 flush, 1 and 2 a sliver gap of 3ε.
/// The judgement of (1, 2) refuses; with the fault, judgement 0's
/// naming fails first.
#[test]
fn precedence_union_naming_fault_then_a_later_judgement_refuses() {
    let eps = Tol::witness().get().eps;
    let doc = ProfileDoc::empty_derived("review_prec_u", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (1.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block_on(doc, a, (2.0 + 3.0 * eps, 3.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, u) = crate::fixture::union_over(doc, &[a, b, c]);
    eprintln!(
        "PREC union fault={:?}: {}",
        std::env::var("REVIEW_FAULT_JUDGEMENT").ok(),
        outcome(&doc, u)
    );
}

/// An intersect of three blocks: 0 and 1 overlap, 0 and 2 touch flush
/// (judged, empty).
#[test]
fn precedence_intersect_naming_fault_then_a_later_judgement_is_empty() {
    let doc = ProfileDoc::empty_derived("review_prec_i", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block_on(doc, a, (1.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, i) = crate::fixture::insert(
        doc,
        Node::Intersect {
            members: Bodies::Spelled(vec![a.into(), b.into(), c.into()]),
        },
    );
    eprintln!(
        "PREC intersect fault={:?}: {}",
        std::env::var("REVIEW_FAULT_JUDGEMENT").ok(),
        outcome(&doc, i)
    );
}
