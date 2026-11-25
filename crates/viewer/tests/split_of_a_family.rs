//! **A split of a family is refused at the door** (D10 `[repetition]`):
//! a split's target takes one body, so a family there refuses — at the
//! viewer's body gate, as the kernel's door refuses it by kind — and a
//! split of each member is written `{k in xs: split(xs[k])}`. A split never sees a family, so the
//! evaluator never meets one there; the family read whole by a union is
//! one body, and its split evaluates.
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use common::len;
use pncad::document::{Bodies, BodyRead, Doc, Node, Operand, RecipeNodeId};
use pncad::geom_core::Tol;
use viewer::session::{DocSession, PatternRuleSpec, Refusal, SessionOp};

/// The box every family repeats: x, y in [-0.01, 0.01], z in [0, 0.01].
const BOX: [f64; 3] = [0.02, 0.02, 0.01];
/// Where the split cuts the box, up from its start cap.
const CUT: f64 = 0.0025;

/// The box and a two-member pattern of it through the pattern door.
fn boxed_pattern(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let mut session =
        common::inline_without_door(Doc::empty_derived("split-of-a-family", tol), tol);
    let body = common::xy_box_in(&mut session, BOX);
    let pattern = common::session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: len(0.04),
            },
        },
    );
    (session, body, pattern)
}

/// **A split of a pattern refuses at the body gate**; nothing is
/// inserted.
#[test]
fn a_split_of_a_pattern_is_refused_by_kind() {
    let tol = Tol::witness();
    let (mut session, body, pattern) = boxed_pattern(tol);
    session.pump();
    let plane = common::cut_across(&session, body, CUT);
    let out = session.perform(SessionOp::AddSplit {
        target: pattern.into(),
        plane,
    });
    assert!(
        matches!(
            &out.refusal,
            Some(Refusal::WrongNodeKind {
                wanted: viewer::session::NodeKindWanted::Body,
                ..
            })
        ),
        "a split of a family refuses at the body gate: {:?}",
        out.refusal
    );
    assert_eq!(out.made, None, "nothing is inserted");
}

/// **A split of the family read whole evaluates**: a union of the
/// pattern is one body, which the split cuts.
#[test]
fn a_split_of_a_family_read_whole_evaluates() {
    let tol = Tol::witness();
    let (session, body, pattern) = boxed_pattern(tol);
    let mut doc = session.committed_doc().clone();
    let whole = common::insert_into(
        &mut doc,
        Node::Union {
            members: Bodies::Family(BodyRead::plain(Operand::from(pattern))),
        },
        tol,
    );
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let plane = common::cut_across(&session, body, CUT);
    let split = common::session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: whole.into(),
            plane,
        },
    );
    session.pump();
    let eval = session.evaluation().expect("landed");
    assert!(
        eval.node_error(split).is_none(),
        "the split of the union evaluates: {:?}",
        eval.node_error(split).map(|e| &e.kind)
    );
}
