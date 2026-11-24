//! **The strict/first-turn split, stated executably.**
//!
//! The tree carries two derived shapes over one substrate (a node's
//! verdict log): `VerdictVector`, ordered and hashed to a
//! `VerdictVectorKey`, which certification gates on; and `vdiff`'s first
//! turn, the first choice two runs take differently, which the flip
//! report names from. These rows assert that neither subsumes the other,
//! and pin what the first turn reads.
//!
//! Every row is the same shape — two runs of one document, one node's
//! log replaced on each side — because the substrate is the log and
//! nothing else.
//!
//! 1. **A real sign change** is named, and moves the key: the positive
//!    control the silent rows are read against.
//! 2. **Permutation.** The first difference is another predicate, so the
//!    two runs' paths parted at something the log does not state: no
//!    flip. The vector is ordered, so its key moves.
//! 3. **Checks trading signs.** Instances of a check trade signs, as a
//!    through slot becoming a blind pocket trades convex corners for
//!    concave ones at tier 3, while every choice is the same: no flip,
//!    and the key moves.
//! 4. **Decision order names the cause.** Two choices turn; the first
//!    in decision order is named, whatever the names' alphabetical
//!    order.
//! 5. **One log ending** before the other names no flip.
//! 6. **Sign exchange.** Two instances of one predicate trade signs:
//!    the first of them turned, and it is named.
//! 7. **Pruning parted.** Two runs decide one predicate with two signs
//!    at one place, after a box query that handed on other candidates:
//!    two different pairs, so no flip; the same candidates, a flip.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture;

use editor_core::{
    CancelToken, EvalOptions, Evaluation, NodeResult, ProfileDoc, RecipeNodeId, VerdictFlip,
    VerdictVector, diff_verdicts, evaluate,
};
use fixture::on_frame;
use geom_core::k_stats::{Role, Verdict};
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

/// A choice.
fn v(predicate: &'static str, sign: Sign) -> Verdict {
    Verdict {
        predicate,
        sign,
        role: Role::Choice,
    }
}

/// A check.
fn c(predicate: &'static str, sign: Sign) -> Verdict {
    Verdict {
        predicate,
        sign,
        role: Role::Check,
    }
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

/// The one flip the engine names between the two runs.
fn named(a: &Evaluation<f64>, b: &Evaluation<f64>) -> VerdictFlip {
    let flips = diff_verdicts(a, b).report();
    assert_eq!(flips.len(), 1, "one flip: {flips:?}");
    flips[0].1
}

/// The two forms disagree, and the disagreement is the split.
fn assert_silent_engine_moved_key(a: &Evaluation<f64>, b: &Evaluation<f64>) {
    assert!(
        diff_verdicts(a, b).is_empty(),
        "the engine names a flip: {:?}",
        diff_verdicts(a, b).report()
    );
    let (ka, kb) = (VerdictVector::of(a).key(), VerdictVector::of(b).key());
    assert_ne!(ka, kb, "the strict vectors' keys agree");
    assert_ne!(VerdictVector::of(a), VerdictVector::of(b));
}

/// Row 1, the positive control: a real sign change is NAMED, and moves
/// the key as well.
#[test]
fn a_real_sign_change_is_named_and_moves_the_vector_key() {
    let (a, b) = pair(
        vec![v("side_of", Sign::Negative)],
        vec![v("side_of", Sign::Positive)],
    );
    assert_eq!(
        named(&a, &b),
        VerdictFlip {
            predicate: "side_of",
            from: Sign::Negative,
            to: Sign::Positive,
        }
    );
    assert_ne!(VerdictVector::of(&a).key(), VerdictVector::of(&b).key());
}

/// Row 2: a permutation of one node's log. No sign changed, and the
/// first difference is another predicate.
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

/// Row 3: checks trade signs around identical choices.
#[test]
fn checks_trading_signs_name_no_flip_and_move_the_vector_key() {
    let (a, b) = pair(
        vec![
            v("side_of", Sign::Negative),
            c("dihedral_sign", Sign::Positive),
            c("dihedral_sign", Sign::Positive),
            v("orientation", Sign::Positive),
        ],
        vec![
            v("side_of", Sign::Negative),
            c("dihedral_sign", Sign::Negative),
            v("orientation", Sign::Positive),
            c("dihedral_sign", Sign::Negative),
        ],
    );
    assert_silent_engine_moved_key(&a, &b);
}

/// Row 4: two choices turn; the first in decision order is the flip,
/// though the other's name sorts first.
#[test]
fn the_first_choice_in_decision_order_is_named() {
    let (a, b) = pair(
        vec![
            c("a_check", Sign::Positive),
            v("z_choice", Sign::Negative),
            v("a_choice", Sign::Positive),
        ],
        vec![
            c("a_check", Sign::Negative),
            v("z_choice", Sign::Positive),
            v("a_choice", Sign::Negative),
        ],
    );
    assert_eq!(
        named(&a, &b),
        VerdictFlip {
            predicate: "z_choice",
            from: Sign::Negative,
            to: Sign::Positive,
        }
    );
}

/// Row 5: one run's choices are a prefix of the other's.
#[test]
fn a_log_ending_first_names_no_flip() {
    let (a, b) = pair(
        vec![v("side_of", Sign::Negative)],
        vec![v("side_of", Sign::Negative), v("side_of", Sign::Positive)],
    );
    assert_silent_engine_moved_key(&a, &b);
}

/// Row 6: two instances of one predicate exchange signs. The first of
/// them is the turn.
#[test]
fn sign_exchange_within_one_node_names_its_first_turn() {
    let (a, b) = pair(
        vec![v("side_of", Sign::Negative), v("side_of", Sign::Positive)],
        vec![v("side_of", Sign::Positive), v("side_of", Sign::Negative)],
    );
    assert_eq!(
        named(&a, &b),
        VerdictFlip {
            predicate: "side_of",
            from: Sign::Negative,
            to: Sign::Positive,
        }
    );
    assert_ne!(VerdictVector::of(&a).key(), VerdictVector::of(&b).key());
}

/// One candidate mark, before the verdict at `before`: `None` for a
/// query's end.
fn mark(before: usize, item: Option<u32>) -> geom_core::k_stats::Mark {
    geom_core::k_stats::Mark {
        before,
        site: geom_core::k_stats::QuerySite::new("props_query", "the test's candidates").site(),
        item: item.map_or(
            geom_core::k_stats::MarkItem::End,
            geom_core::k_stats::MarkItem::Candidate,
        ),
        role: Role::Choice,
    }
}

/// `pair`, each run's node carrying `marks` beside its log.
fn marked(
    (a_log, a_marks): (Vec<Verdict>, Vec<geom_core::k_stats::Mark>),
    (b_log, b_marks): (Vec<Verdict>, Vec<geom_core::k_stats::Mark>),
) -> (Evaluation<f64>, Evaluation<f64>) {
    let (mut a, mut b) = pair(a_log, b_log);
    for (ev, marks) in [(&mut a, a_marks), (&mut b, b_marks)] {
        for result in ev.nodes.values_mut() {
            if let NodeResult::Ok(value) = result
                && !value.verdicts.is_empty()
            {
                value.marks = Arc::new(marks.clone());
            }
        }
    }
    (a, b)
}

/// Row 7: the same predicate at the same place with two signs is a turn
/// only after the same candidates.
#[test]
fn a_turn_after_other_candidates_is_no_flip() {
    let log = |sign| vec![v("plane_eq_parallel", sign)];
    let (a, b) = marked(
        (log(Sign::Zero), vec![mark(0, Some(0)), mark(0, None)]),
        (log(Sign::Positive), vec![mark(0, None)]),
    );
    assert_silent_engine_moved_key(&a, &b);
    let (a, b) = marked(
        (log(Sign::Zero), vec![mark(0, Some(0)), mark(0, None)]),
        (log(Sign::Positive), vec![mark(0, Some(0)), mark(0, None)]),
    );
    assert_eq!(
        named(&a, &b),
        VerdictFlip {
            predicate: "plane_eq_parallel",
            from: Sign::Zero,
            to: Sign::Positive,
        },
        "after the same candidates the turn is named"
    );
}

/// Row 7, its other half: a query a check scope ran is not on the
/// choices' path. The same choices and the same turn, with marks that
/// differ only among checks: the turn is still named.
#[test]
fn a_check_scoped_query_parts_nothing() {
    let log = |sign| vec![v("plane_eq_parallel", sign)];
    let check_mark = |item| geom_core::k_stats::Mark {
        role: Role::Check,
        ..mark(0, item)
    };
    let (a, b) = marked(
        (log(Sign::Zero), vec![check_mark(Some(0)), check_mark(None)]),
        (log(Sign::Positive), vec![check_mark(None)]),
    );
    assert_eq!(
        named(&a, &b),
        VerdictFlip {
            predicate: "plane_eq_parallel",
            from: Sign::Zero,
            to: Sign::Positive,
        },
        "a check's candidates part no choice"
    );
}
