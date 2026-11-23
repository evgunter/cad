//! **A family at the doors** (D10 `[repetition]`): a pattern read whole
//! at the list seats (union, intersect) is every member, and at a seat
//! for one body (a subtract's tool until it is a list seat, a split's
//! target, a mirror's body) it refuses by kind, saying to write the
//! binder `{k in xs: op(xs[k])}`.
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use common::len;
use pncad::document::{
    Bodies, BodyRead, BooleanValue, Doc, Formula, Node, Operand, ProfileProgram, RecipeNodeId,
};
use pncad::geom_core::Tol;
use pncad::prelude::ValuePayload;
use viewer::session::{BooleanSpec, DocSession, FaceSelection, PatternRuleSpec, SessionOp};

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
        &pncad::topo::finish(body.body().clone(), tol).expect("tier 3"),
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

/// **A pattern read whole at a subtraction's tool seat cuts the kept
/// body by every member**: the tool is a `Bodies` seat (DESIGN
/// `[repetition]`, REFERENCES DM4), so the result is one body with
/// every member removed. A union over the same read takes every member
/// (one body), and an intersection takes every member too, so it is
/// empty for a row of disjoint copies.
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
            tool: vec![pattern.into()]
        }),
        format!("Boolean {:.3e}", C_VOL - 2.0 * OVERLAP),
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

/// The refusal of a family at a seat for one body: the body gate's,
/// before the door (which refuses it by kind, `SlotVarKind`).
fn assert_refused_by_kind(refusal: &Option<viewer::session::Refusal>) {
    assert!(
        matches!(
            refusal,
            Some(viewer::session::Refusal::WrongNodeKind {
                wanted: viewer::session::NodeKindWanted::Body,
                ..
            })
        ),
        "a family at a seat for one body refuses at the gate: {refusal:?}"
    );
}

/// The box and a two-member pattern of it through the pattern door.
fn boxed_pattern() -> (DocSession, RecipeNodeId, RecipeNodeId) {
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
    (session, body, pattern)
}

/// **A split and a mirror of a pattern refuse by kind**, whether or not
/// something reads the pattern: each takes one body, so "each member"
/// is written as a binder.
#[test]
fn a_split_and_a_mirror_of_a_pattern_refuse_by_kind() {
    let (mut session, body, pattern) = boxed_pattern();
    let plane = common::cut_across(&session, body, 0.0025);
    let split = session.perform(SessionOp::AddSplit {
        target: pattern.into(),
        plane,
    });
    assert_refused_by_kind(&split.refusal);
    let mirror = session.perform(SessionOp::AddMirror {
        body: pattern.into(),
        plane: viewer::session::ToolPlane::Face(cap_end(body)),
    });
    assert_refused_by_kind(&mirror.refusal);
    // Read whole by a union first, the pattern still refuses at a split.
    let mut doc = session.committed_doc().clone();
    common::insert_into(
        &mut doc,
        Node::Union {
            members: Bodies::Family(BodyRead::plain(Operand::from(pattern))),
        },
        Tol::witness(),
    );
    let mut session = common::inline_without_door(doc, Tol::witness());
    let plane = common::cut_across(&session, body, 0.0025);
    let split = session.perform(SessionOp::AddSplit {
        target: pattern.into(),
        plane,
    });
    assert_refused_by_kind(&split.refusal);
}
