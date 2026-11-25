//! **A family the body gate admits, at the doors that take it**: a
//! pattern read whole at the boolean seats, and split at the split
//! door beside the readers a split's takeover re-points, each
//! evaluated and its geometry read.
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use common::len;
use pncad::document::{
    Bodies, BodyRead, BooleanValue, Doc, Formula, Node, Operand, ProfileProgram, RecipeNodeId,
};
use pncad::geom_core::Tol;
use pncad::prelude::ValuePayload;
use viewer::session::{
    BooleanSpec, DocSession, FaceSelection, NodeKindWanted, PatternRuleSpec, SessionOp,
};

/// The box every family repeats: x, y in [-0.01, 0.01], z in [0, 0.01].
const BOX: [f64; 3] = [0.02, 0.02, 0.01];
/// The cutter: x in [-0.02, 0.06], y in [0, 0.015], z in [-0.01, 0.025]
/// once slid, so it overlaps each member of a two-member row 0.04
/// apart in a 0.02 x 0.01 x 0.01 block and touches no face of one.
const CUTTER: [f64; 3] = [0.08, 0.015, 0.035];
const CUTTER_AT: [f64; 3] = [0.02, 0.0075, -0.01];
const B_VOL: f64 = 0.02 * 0.02 * 0.01;
const C_VOL: f64 = 0.08 * 0.015 * 0.035;
const OVERLAP: f64 = 0.02 * 0.01 * 0.01;

/// The box, a cutter slid across where a two-member row of it stands,
/// and that row (a pattern of the box, 0.04 apart along x).
struct World {
    doc: Doc<ProfileProgram>,
    cutter: RecipeNodeId,
    pattern: RecipeNodeId,
}

fn world(tol: Tol) -> World {
    let mut session = common::inline_without_door(Doc::empty_derived("review-denotes", tol), tol);
    let body = common::xy_box_in(&mut session, BOX);
    let raw_cutter = common::xy_box_in(&mut session, CUTTER);
    let doc = session.committed_doc().clone();
    let (doc, cutter) = crate::fixture::slid(doc, body, raw_cutter, CUTTER_AT);
    let (doc, pattern) =
        common::row_of_copies(&doc, body, 2, (pncad::document::TargetAxis::X, 0.04), tol);
    World {
        doc,
        cutter,
        pattern,
    }
}

fn cap_end(body: RecipeNodeId) -> FaceSelection {
    FaceSelection {
        name: pncad::prelude::StableName {
            kind: pncad::prelude::EntityKind::Face,
            node: body,
            path: vec![pncad::select::RoleSeg::Cap(pncad::select::CapEnd::End)],
        },
        node: body,
        body: 0,
    }
}

fn vol(body: &pncad::document::Framed<f64>, tol: Tol) -> f64 {
    pncad::topo::mass_properties(
        &pncad::topo::finish(pncad::topo::Built::of(body.body().clone()), tol).expect("tier 3"),
        tol,
        pncad::topo::test_support::pinned_resolution(tol),
    )
    .expect("mass properties")
    .met()
    .expect("met")
    .volume
}

/// What `node` evaluated to, as a sentence: its error, or its value's
/// shape and member volumes.
fn described(session: &DocSession, node: RecipeNodeId, tol: Tol) -> String {
    let eval = session.evaluation().expect("landed");
    if let Some(err) = eval.node_error(node) {
        return format!("ERROR {:?}", err.kind);
    }
    let Some(value) = eval.value(node) else {
        return "no value".into();
    };
    match &value.payload {
        ValuePayload::Body(b) => format!("Body {:.3e}", vol(b, tol)),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            format!("Boolean {:.3e}", vol(body, tol))
        }
        ValuePayload::Boolean(BooleanValue::Empty) => "Boolean Empty".into(),
        ValuePayload::Instances(members) => format!(
            "Instances [{}]",
            members
                .iter()
                .map(|m| m
                    .as_ref()
                    .map_or("empty".into(), |b| format!("{:.3e}", vol(b, tol))))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        ValuePayload::Split { .. } => "Split".into(),
        ValuePayload::Family => "Family".into(),
        ValuePayload::Assertion(v) => format!("Assertion {v:?}").chars().take(80).collect(),
        other => format!("{:?}", std::mem::discriminant(other)),
    }
}

/// **A pattern read whole at a subtraction's tool seat cuts ONE hole
/// per copy of the kept body** — today's answer, pinned: the
/// subtraction reaches the pattern's index, so it is evaluated once
/// per member and the result is a family of the kept body, each with
/// one member removed, not the kept body with every member removed.
/// A union over the same read takes every member (one body), and an
/// intersection takes every member too, so it is empty for a row of
/// disjoint copies.
#[test]
fn a_pattern_read_whole_at_the_boolean_seats() {
    let tol = Tol::witness();
    let w = world(tol);
    let pattern = w.pattern;
    let run = |spec: BooleanSpec| {
        let mut s = common::inline_without_door(w.doc.clone(), tol);
        let out = s.perform(SessionOp::AddBoolean { spec });
        assert!(out.refusal.is_none(), "{:?}", out.refusal);
        let made = out.made.expect("a boolean");
        s.pump();
        described(&s, made, tol)
    };
    let c: BodyRead<Formula> = w.cutter.into();
    assert_eq!(
        run(BooleanSpec::Subtract {
            from: c.clone(),
            tool: pattern.into()
        }),
        format!("Instances [{0:.3e}, {0:.3e}]", C_VOL - OVERLAP),
    );
    assert_eq!(
        run(BooleanSpec::Union(vec![pattern.into(), c.clone()])),
        format!("Boolean {:.3e}", 2.0 * B_VOL + C_VOL - 2.0 * OVERLAP),
    );
    assert_eq!(
        run(BooleanSpec::Intersect(vec![pattern.into(), c])),
        "Boolean Empty"
    );
}

/// The box, a two-member pattern of it through the pattern door, and
/// `reader` (given the pattern) inserted beside them.
fn split_of_a_read_pattern(
    reader: impl FnOnce(&mut DocSession, RecipeNodeId),
) -> Result<String, String> {
    let tol = Tol::witness();
    let mut session =
        common::inline_without_door(Doc::empty_derived("review-split-pattern", tol), tol);
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
    reader(&mut session, pattern);
    assert!(viewer::session::admits(
        session.committed_doc(),
        pattern,
        NodeKindWanted::Body
    ));
    let plane = common::cut_across(&session, body, 0.0025);
    let out = session.perform(SessionOp::AddSplit {
        target: pattern.into(),
        plane,
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    for edit in &out.committed {
        if let pncad::document::DocEdit::InsertNode { node, .. } = edit {
            eprintln!(
                "split door inserted: {}",
                format!("{node:?}").chars().take(200).collect::<String>()
            );
        } else {
            eprintln!(
                "split door edit: {}",
                format!("{edit:?}").chars().take(200).collect::<String>()
            );
        }
    }
    let split = out.made.expect("a split");
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        session.pump();
        let eval = session.evaluation().expect("landed");
        format!("{:?}", eval.node_error(split).map(|e| &e.kind))
    }))
    .map_err(|p| {
        p.downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| p.downcast_ref::<String>().cloned())
            .unwrap_or_default()
    })
}

/// **A split of a pattern something reads evaluates**: the split door
/// admits the pattern at its target seat (the body gate says it is a
/// body), the commit lands, and the takeover re-points what read the
/// pattern onto the split's `above ⊔ below`.
#[test]
fn a_split_of_a_pattern_a_union_reads_evaluates() {
    let got = split_of_a_read_pattern(|session, pattern| {
        let doc = session.committed_doc().clone();
        let mut doc = doc;
        common::insert_into(
            &mut doc,
            Node::Union {
                members: Bodies::Family(BodyRead::plain(Operand::from(pattern))),
            },
            Tol::witness(),
        );
        *session = common::inline_without_door(doc, Tol::witness());
    });
    assert!(got.is_ok(), "the evaluation panicked: {got:?}");
}

/// The same, the pattern read by nothing: lands and evaluates.
#[test]
fn a_split_of_a_pattern_nothing_reads_evaluates() {
    let got = split_of_a_read_pattern(|_, _| {});
    assert_eq!(got, Ok("None".to_string()));
}

/// The same, the pattern made a symmetric part by the mirror door
/// first (a union of the pattern and its image reads it).
#[test]
fn a_split_of_a_mirrored_pattern_evaluates() {
    let got = split_of_a_read_pattern(|session, pattern| {
        let body = {
            let doc = session.committed_doc();
            match doc.node(pattern) {
                Some(Node::Place { .. }) => {}
                other => panic!("{other:?}"),
            }
            common::authored_ids(doc)
                .into_iter()
                .find(|&id| matches!(doc.node(id), Some(Node::Extrude { .. })))
                .expect("the box")
        };
        let out = session.perform(SessionOp::AddMirror {
            body: pattern.into(),
            plane: viewer::session::ToolPlane::Face(cap_end(body)),
        });
        assert!(out.refusal.is_none(), "{:?}", out.refusal);
    });
    assert!(got.is_ok(), "the evaluation panicked: {got:?}");
}
