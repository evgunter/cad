//! **The strict/permutation-invariant split, stated executably.**
//!
//! The tree carries two derived shapes over one substrate (a node's
//! verdict log): `VerdictVector`, ordered and hashed to a
//! `VerdictVectorKey`, which certification gates on; and `vdiff`'s
//! per-predicate sign POPULATIONS, which the flip report names from.
//! The claim that neither subsumes the other was argued in prose at
//! both types and asserted nowhere: these two rows assert it.
//!
//! Both rows are the same shape — two runs of one document, one node's
//! log replaced on each side — because the substrate is the log and
//! nothing else. The population engine reports NO flip on either pair
//! while the two vectors' keys DIFFER, which is exactly the asymmetry
//! the split exists for: the strict form gates and cannot explain, the
//! population form explains and must not gate.
//!
//! 1. **Permutation.** No verdict changes sign; the decision ORDER
//!    inside one node changes. Populations are permutation-invariant
//!    by construction, so the engine is silent; the vector is ordered,
//!    so its key moves.
//! 2. **Sign exchange.** Two instances of ONE predicate trade opposite
//!    signs at one node. Every site's sign changed, and the populations
//!    still net to nothing — the blind spot `vdiff`'s module docs
//!    document, here as a row rather than a sentence.
//!
//! A third row is the positive control the other two are read against:
//! one predicate really changes sign, the engine names exactly one
//! flip, and the key moves too. Silence in rows 1 and 2 is evidence
//! about the population form only because this row shows the engine is
//! not silent on the same fixture.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture;

use editor_core::{
    CancelToken, EvalOptions, Evaluation, NodeResult, ProfileDoc, RecipeNodeId, VerdictVector,
    diff_verdicts, evaluate,
};
use fixture::on_frame;
use geom_core::k_stats::Verdict;
use geom_core::{Sign, Tol};

/// A frame and a profile drawn on it (two nodes), evaluated at f64.
/// The geometry is irrelevant — what the rows need is one node with a
/// value whose log they can set.
fn run() -> (Evaluation<f64>, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("props_verdict_shapes", Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert!(ev.value(profile).is_some(), "the profile node built");
    (ev, profile)
}

fn v(predicate: &'static str, sign: Sign) -> Verdict {
    Verdict { predicate, sign }
}

/// The two runs of one document, each carrying `log` at `node`.
fn pair(a_log: Vec<Verdict>, b_log: Vec<Verdict>) -> (Evaluation<f64>, Evaluation<f64>) {
    let (mut a, node) = run();
    let (mut b, other) = run();
    assert_eq!(node, other, "one document, one node id");
    for (ev, log) in [(&mut a, a_log), (&mut b, b_log)] {
        match ev.nodes.get_mut(&node) {
            Some(NodeResult::Ok(value)) => value.verdicts = Arc::new(log),
            _ => panic!("the profile node built"),
        }
    }
    (a, b)
}

/// The two forms disagree, and the disagreement is the split.
fn assert_silent_engine_moved_key(a: &Evaluation<f64>, b: &Evaluation<f64>) {
    assert!(
        diff_verdicts(a, b).is_empty(),
        "the population engine names a flip: {:?}",
        diff_verdicts(a, b).report()
    );
    let (ka, kb) = (VerdictVector::of(a).key(), VerdictVector::of(b).key());
    assert_ne!(ka, kb, "the strict vectors' keys agree");
    assert_ne!(VerdictVector::of(a), VerdictVector::of(b));
}

/// Row 1: a permutation of one node's log. No sign changed.
#[test]
fn permuted_log_names_no_flip_and_moves_the_vector_key() {
    let (a, b) = pair(
        vec![
            v("side_of", Sign::Negative),
            v("orientation", Sign::Positive),
        ],
        vec![
            v("orientation", Sign::Positive),
            v("side_of", Sign::Negative),
        ],
    );
    assert_silent_engine_moved_key(&a, &b);
}

/// The positive control: a real sign change is NAMED, and moves the key
/// as well. Both forms answer; they differ only where rows 1 and 2 put
/// them.
#[test]
fn a_real_sign_change_is_named_and_moves_the_vector_key() {
    let (a, b) = pair(
        vec![v("side_of", Sign::Negative)],
        vec![v("side_of", Sign::Positive)],
    );
    let flips = diff_verdicts(&a, &b).report();
    assert_eq!(flips.len(), 1, "one flip: {flips:?}");
    let (_, flip) = flips[0];
    assert_eq!(flip.predicate, "side_of");
    assert_eq!(
        (flip.from, flip.to, flip.count),
        (Sign::Negative, Sign::Positive, 1)
    );
    assert_ne!(VerdictVector::of(&a).key(), VerdictVector::of(&b).key());
}

/// Row 2: two instances of one predicate exchange signs. Every site
/// changed; the populations did not.
#[test]
fn sign_exchange_within_one_node_names_no_flip_and_moves_the_vector_key() {
    let (a, b) = pair(
        vec![v("side_of", Sign::Negative), v("side_of", Sign::Positive)],
        vec![v("side_of", Sign::Positive), v("side_of", Sign::Negative)],
    );
    assert_silent_engine_moved_key(&a, &b);
}
