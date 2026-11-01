//! **A union announces the first defect in evaluation order**
//! (`[failure.taxonomy]`: a silent discard is never an answer). The
//! kernel's n-ary door runs every judgement and step before the node
//! names any, so the node names them in the order they ran and stops at
//! the first refusal, its own naming's included: a naming refusal of an
//! earlier judgement comes before a later judgement's refusal, and
//! before an intersect's later empty result, which would otherwise
//! discard it. No geometry makes a judgement's naming refuse on demand,
//! so the refusal is the test seam's
//! ([`editor_core::test_support::with_judgement_naming_fault`]).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, failure, run};
use editor_core::test_support::{JUDGEMENT_NAMING_FAULT, with_judgement_naming_fault};
use editor_core::{
    Bodies, BooleanValue, NamingError, Node, NodeErrorKind, ProfileDoc, RecipeNodeId, ValuePayload,
};
use geom_core::Tol;

/// How a union or an intersect evaluated.
#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    /// It built a body.
    Built,
    /// The typed empty result.
    Empty,
    /// The seam's naming refusal.
    Seam,
    /// The pair boolean's refusal.
    Boolean,
    /// Any other refusal, spelled.
    Other(String),
}

/// How `node` evaluated.
fn outcome(doc: &ProfileDoc, node: RecipeNodeId) -> Outcome {
    let ev = run(doc);
    match failure(&ev, node) {
        Some(NodeErrorKind::Naming(NamingError::Emission { what }))
            if *what == JUDGEMENT_NAMING_FAULT =>
        {
            Outcome::Seam
        }
        Some(NodeErrorKind::Boolean(_)) => Outcome::Boolean,
        Some(other) => Outcome::Other(format!("{other:?}")),
        None => match ev.value(node).map(|v| &v.payload) {
            Some(ValuePayload::Boolean(BooleanValue::Empty)) => Outcome::Empty,
            Some(_) => Outcome::Built,
            None => panic!("{node:?} has no value and no refusal"),
        },
    }
}

/// Three blocks in a row: the first two flush, the third 3ε past the
/// second, a sliver the judgement of (1, 2) refuses. Judgement 0 is
/// (0, 1); its naming refused comes first.
#[test]
fn a_judgements_naming_refusal_comes_before_a_later_judgements_refusal() {
    let eps = Tol::witness().get().eps;
    let doc = ProfileDoc::empty_derived("first_defect_union", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (1.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block_on(doc, a, (2.0 + 3.0 * eps, 3.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, u) = crate::fixture::union_over(doc, &[a, b, c]);
    assert_eq!(
        outcome(&doc, u),
        Outcome::Boolean,
        "the sliver pair refuses the union"
    );
    assert_eq!(
        with_judgement_naming_fault(0, || outcome(&doc, u)),
        Outcome::Seam,
        "judgement 0's naming refusal is announced, not the later pair's"
    );
}

/// An intersect of three blocks: 0 and 1 overlap, 0 and 2 touch flush,
/// so the judgement of (0, 2) is empty and the intersect is the typed
/// empty result. Judgement 0 is (0, 1); its naming refused is announced,
/// not discarded by the later empty result.
#[test]
fn a_judgements_naming_refusal_is_not_discarded_by_a_later_empty_judgement() {
    let doc = ProfileDoc::empty_derived("first_defect_intersect", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block_on(doc, a, (1.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, i) = crate::fixture::insert(
        doc,
        Node::Intersect {
            members: Bodies::Spelled(vec![a.into(), b.into(), c.into()]),
        },
    );
    assert_eq!(
        outcome(&doc, i),
        Outcome::Empty,
        "the intersect is the typed empty result"
    );
    assert_eq!(
        with_judgement_naming_fault(0, || outcome(&doc, i)),
        Outcome::Seam,
        "judgement 0's naming refusal is announced, not discarded"
    );
}
