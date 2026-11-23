//! **A split of a family evaluates without panicking** (D10
//! `[repetition]`): the split door over every family form the body
//! gate admits, and over a pattern a placement reads, which the door
//! re-points onto the split's `above ⊔ below`. A split of a family is a
//! family of splits; a read of one of its halves by a reader of no
//! member refuses typed (`HalfOfAFamily`). What such a read should be
//! waits on `work/formwords/a-family-read-at-a-body-seat-maps-implicitly.md`.
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use common::len;
use pncad::document::{
    Bodies, BodyRead, BooleanValue, Doc, Formula, Node, Operand, ProfileProgram, RecipeNodeId,
};
use pncad::geom_core::Tol;
use pncad::prelude::ValuePayload;
use viewer::session::{DocSession, FaceSelection, NodeKindWanted, PatternRuleSpec, SessionOp};

/// The box every family repeats: x, y in [-0.01, 0.01], z in [0, 0.01].
const BOX: [f64; 3] = [0.02, 0.02, 0.01];
/// Where the split cuts the box, up from its start cap.
const CUT: f64 = 0.0025;

/// The box, and each family form the body gate admits beside it, by
/// name: a pattern of two, a placement of that pattern, a union of the
/// pattern read whole, an empty boolean, a sweep, and a mirror of the
/// pattern.
fn world(
    tol: Tol,
) -> (
    Doc<ProfileProgram>,
    RecipeNodeId,
    Vec<(&'static str, RecipeNodeId)>,
) {
    let mut session =
        common::inline_without_door(Doc::empty_derived("split-of-a-family", tol), tol);
    let body = common::xy_box_in(&mut session, BOX);
    let raw_far = common::xy_box_in(&mut session, [0.01, 0.01, 0.01]);
    let profile = common::rectangle_in(&mut session, 0.01, 0.01);
    let path = common::rectangle_in(&mut session, 0.02, 0.02);
    let doc = session.committed_doc().clone();
    let (doc, far) = crate::fixture::slid(doc, body, raw_far, [0.3, 0.0, 0.0]);
    let (doc, pattern) =
        common::row_of_copies(&doc, body, 2, (pncad::document::TargetAxis::X, 0.04), tol);
    let (mut doc, moved) = crate::fixture::slid(doc, pattern, pattern, [0.0, 0.0, 0.005]);
    let family_union = common::insert_into(
        &mut doc,
        Node::Union {
            members: Bodies::Family(BodyRead::plain(Operand::from(pattern))),
        },
        tol,
    );
    let empty = common::insert_into(
        &mut doc,
        Node::Intersect {
            members: Bodies::Spelled(vec![body.into(), far.into()]),
        },
        tol,
    );
    let sweep = common::insert_into(
        &mut doc,
        Node::Sweep {
            profile: profile.into(),
            path: path.into(),
            stations: Formula::count(8),
            v_degree: Formula::count(3),
        },
        tol,
    );
    let mut s = common::inline_without_door(doc, tol);
    let mirror = common::session_insert(
        &mut s,
        SessionOp::AddMirror {
            body: pattern.into(),
            plane: viewer::session::ToolPlane::Face(cap_end(body)),
        },
    );
    let variants = vec![
        ("pattern", pattern),
        ("transform of a pattern", moved),
        ("family union", family_union),
        ("empty boolean", empty),
        ("sweep", sweep),
        ("mirror of a pattern", mirror),
    ];
    (s.committed_doc().clone(), body, variants)
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
/// shape and its bodies' volumes.
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
        other => format!("{:?}", std::mem::discriminant(other)),
    }
}

/// The panic `run` raised, as text, or what it answered.
fn caught<R>(run: impl FnOnce() -> R) -> Result<R, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(run)).map_err(|p| {
        p.downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| p.downcast_ref::<String>().cloned())
            .unwrap_or_default()
    })
}

/// **The split door over every family form the body gate admits
/// evaluates without panicking**, each split reading its target's
/// every member: a split of a pattern of two is a family of two splits.
#[test]
fn a_split_of_every_family_form_evaluates_without_panicking() {
    let tol = Tol::witness();
    let (doc, body, variants) = world(tol);
    let mut panicked = Vec::new();
    for &(name, v) in &variants {
        let line = caught(|| {
            let mut s = common::inline_without_door(doc.clone(), tol);
            s.pump();
            let plane = common::cut_across(&s, body, CUT);
            let out = s.perform(SessionOp::AddSplit {
                target: v.into(),
                plane,
            });
            match (out.refusal, out.made) {
                (Some(r), _) => format!("REFUSED {r:?}"),
                (None, Some(made)) => {
                    s.pump();
                    described(&s, made, tol)
                }
                (None, None) => "no node made".into(),
            }
        })
        .unwrap_or_else(|p| {
            panicked.push(format!("{name}: {p}"));
            format!("PANIC {p}")
        });
        eprintln!("split of {name:24} {line}");
        if name.contains("pattern") {
            assert_eq!(
                line, "Family",
                "a split of the {name} is a family of splits"
            );
        }
    }
    assert!(panicked.is_empty(), "the evaluator panicked: {panicked:#?}");
}

/// The box, a two-member pattern of it through the pattern door, and
/// `reader` (given the pattern) inserted beside them, then the split
/// door over the pattern: the split, and the session.
fn split_of_a_read_pattern(
    reader: impl FnOnce(&mut DocSession, RecipeNodeId),
) -> (DocSession, RecipeNodeId) {
    let tol = Tol::witness();
    let mut session =
        common::inline_without_door(Doc::empty_derived("split-of-a-read-pattern", tol), tol);
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
    let plane = common::cut_across(&session, body, CUT);
    let split = common::session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: pattern.into(),
            plane,
        },
    );
    (session, split)
}

/// **A split of a pattern a placement reads evaluates, the read of its
/// halves refusing typed**: the door re-points the placement onto the
/// split's `above ⊔ below`. A split of a family is a family of splits,
/// so that assemble, which reads no member, has no one half to read: it
/// refuses `HalfOfAFamily` naming the split. No evaluation panics.
#[test]
fn a_split_of_a_transformed_pattern_evaluates() {
    let tol = Tol::witness();
    let (mut session, split) = split_of_a_read_pattern(|session, pattern| {
        let (doc, _) = crate::fixture::slid(
            session.committed_doc().clone(),
            pattern,
            pattern,
            [0.0, 0.0, 0.005],
        );
        *session = common::inline_without_door(doc, tol);
    });
    let got = caught(|| {
        session.pump();
        described(&session, split, tol)
    });
    assert_eq!(
        got.as_deref(),
        Ok("Family"),
        "a split of a pattern is a family of splits"
    );
    let doc = session.committed_doc();
    let eval = session.evaluation().expect("landed");
    let assemble = common::authored_ids(doc)
        .into_iter()
        .find(|&id| matches!(doc.node(id), Some(Node::Assemble { .. })))
        .expect("the door wrote above ⊔ below");
    assert!(
        matches!(
            eval.node_error(assemble).map(|e| &e.kind),
            Some(pncad::document::NodeErrorKind::HalfOfAFamily { input, .. }) if *input == split
        ),
        "the assemble refuses its half of a family: {:?}",
        eval.node_error(assemble).map(|e| &e.kind)
    );
}
