//! Review rows for PR 7379 (`denotes-body-enumerates-its-gaps-against-the-operand-door-and-misses-one`):
//! every real body seat driven over every family form the body gate
//! admits, evaluated, its geometry measured.
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
    AssertedClaim, AssertedSeat, BooleanSpec, DocSession, FaceSelection, NodeKindWanted,
    PatternRuleSpec, SessionOp,
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

struct World {
    doc: Doc<ProfileProgram>,
    body: RecipeNodeId,
    cutter: RecipeNodeId,
    far: RecipeNodeId,
    variants: Vec<(&'static str, RecipeNodeId)>,
}

fn world(tol: Tol) -> World {
    let mut session = common::inline_without_door(Doc::empty_derived("review-denotes", tol), tol);
    let body = common::xy_box_in(&mut session, BOX);
    let raw_cutter = common::xy_box_in(&mut session, CUTTER);
    let raw_far = common::xy_box_in(&mut session, [0.01, 0.01, 0.01]);
    let profile = common::rectangle_in(&mut session, 0.01, 0.01);
    let path = common::rectangle_in(&mut session, 0.02, 0.02);
    let doc = session.committed_doc().clone();
    let (doc, cutter) = crate::fixture::slid(doc, body, raw_cutter, CUTTER_AT);
    let (doc, far) = crate::fixture::slid(doc, body, raw_far, [0.3, 0.0, 0.0]);
    let (doc, pattern) =
        common::row_of_copies(&doc, body, 2, (pncad::document::TargetAxis::X, 0.04), tol);
    let (doc, moved) = crate::fixture::slid(doc, pattern, pattern, [0.0, 0.0, 0.005]);
    let mut doc = doc;
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
    // A mirror of the pattern, through the mirror door.
    let mut s = common::inline_without_door(doc.clone(), tol);
    let out = s.perform(SessionOp::AddMirror {
        body: pattern.into(),
        plane: cap_end(body),
    });
    let mirror = out.made;
    let doc = if out.refusal.is_none() {
        s.committed_doc().clone()
    } else {
        eprintln!("mirror of a pattern refused: {:?}", out.refusal);
        doc
    };
    let mut variants = vec![
        ("pattern", pattern),
        ("transform of a pattern", moved),
        ("family union", family_union),
        ("empty boolean", empty),
        ("sweep", sweep),
    ];
    if let Some(m) = mirror {
        variants.push(("mirror of a pattern", m));
    }
    World {
        doc,
        body,
        cutter,
        far,
        variants,
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

fn seat_ops(
    w: &World,
    v: RecipeNodeId,
    session: &DocSession,
) -> Vec<(&'static str, Option<SessionOp>)> {
    let edges = {
        let eval = session.evaluation().expect("landed");
        pncad::select::all_edges(eval, w.body)
    };
    let c: BodyRead<Formula> = w.cutter.into();
    vec![
        (
            "union [v, c]",
            Some(SessionOp::AddBoolean {
                spec: BooleanSpec::Union(vec![v.into(), c.clone()]),
            }),
        ),
        (
            "intersect [v, c]",
            Some(SessionOp::AddBoolean {
                spec: BooleanSpec::Intersect(vec![v.into(), c.clone()]),
            }),
        ),
        (
            "subtract v - c",
            Some(SessionOp::AddBoolean {
                spec: BooleanSpec::Subtract {
                    from: v.into(),
                    tool: c.clone(),
                },
            }),
        ),
        (
            "subtract c - v",
            Some(SessionOp::AddBoolean {
                spec: BooleanSpec::Subtract {
                    from: c.clone(),
                    tool: v.into(),
                },
            }),
        ),
        (
            "split v",
            Some(SessionOp::AddSplit {
                target: v.into(),
                plane: common::cut_across(session, w.body, 0.0025),
            }),
        ),
        (
            "mirror v",
            Some(SessionOp::AddMirror {
                body: v.into(),
                plane: cap_end(w.body),
            }),
        ),
        (
            "pattern v",
            Some(SessionOp::AddPattern {
                input: v.into(),
                count: 2,
                rule: PatternRuleSpec::Linear {
                    direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::Y),
                    spacing: len(0.05),
                },
            }),
        ),
        (
            "fused pattern v",
            Some(SessionOp::AddFusedPattern {
                input: v.into(),
                count: 2,
                rule: PatternRuleSpec::Linear {
                    direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::Y),
                    spacing: len(0.05),
                },
            }),
        ),
        (
            "clearance v, far",
            Some(SessionOp::AddAssertion {
                a: AssertedSeat::Body(v.into()),
                b: AssertedSeat::Body(w.far.into()),
                claim: AssertedClaim::Clearance(pncad::document::ClearanceBounds::AtLeast(
                    Formula::zero(pncad::document::Dimension::Length),
                )),
            }),
        ),
        (
            "fillet v (box's names)",
            Some(SessionOp::AddFillet {
                target: v,
                radius: len(0.001),
                selection: edges.clone(),
            }),
        ),
        (
            "chamfer v (box's names)",
            Some(SessionOp::AddChamfer {
                target: v,
                distance: len(0.001),
                selection: edges,
            }),
        ),
        ("shell v (doc door)", None),
        ("placement of v (doc door)", None),
    ]
}

/// **Every body seat, driven through its own door over every family
/// form the body gate admits, evaluates without panicking**, and the
/// table of what each made is printed (`--nocapture`). Red today on
/// one cell: the split door over a pattern a placement reads (see
/// `a_split_of_a_transformed_pattern_evaluates`).
#[test]
fn every_body_seat_over_every_family_form_evaluates_without_panicking() {
    let tol = Tol::witness();
    let w = world(tol);
    let mut base = common::inline_without_door(w.doc.clone(), tol);
    base.pump();
    let mut panicked = Vec::new();
    for &(name, v) in &w.variants {
        let has_body = {
            let eval = base.evaluation().expect("landed");
            eval.value(v)
                .and_then(|value| match &value.payload {
                    ValuePayload::Boolean(BooleanValue::Empty) => None,
                    other => Some(other),
                })
                .is_some()
        };
        let admitted = viewer::session::admits(&w.doc, v, NodeKindWanted::Body);
        eprintln!(
            "\n=== {name}: admits(Body) = {admitted}; value: {}",
            described(&base, v, tol)
        );
        for (seat, op) in seat_ops(&w, v, &base) {
            let doc = w.doc.clone();
            let line = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match op {
                Some(op) => {
                    let mut s = common::inline_without_door(w.doc.clone(), tol);
                    s.pump();
                    let out = s.perform(op);
                    match (out.refusal, out.made) {
                        (Some(r), _) => format!("REFUSED {r:?}").chars().take(160).collect(),
                        (None, Some(made)) => {
                            s.pump();
                            described(&s, made, tol)
                        }
                        (None, None) => "no node made".into(),
                    }
                }
                None if seat.starts_with("shell") => {
                    match pncad::document::apply(
                        &w.doc,
                        &pncad::document::DocEdit::InsertNode {
                            node: Box::new(Node::shell(v, len(0.001), Vec::new())),
                            fresh: Vec::new(),
                        },
                        tol,
                    ) {
                        Err(e) => format!("EDIT REFUSED {e:?}"),
                        Ok(applied) => {
                            let made = applied.record.minted.expect("minted");
                            let mut s = common::inline_without_door(applied.doc, tol);
                            s.pump();
                            described(&s, made, tol)
                        }
                    }
                }
                None if !has_body => {
                    "skipped: no body to place (the fixture's placer reads a value)".into()
                }
                None => {
                    let (doc, placed) =
                        crate::fixture::slid(w.doc.clone(), w.body, v, [0.0, 0.1, 0.0]);
                    let mut s = common::inline_without_door(doc, tol);
                    s.pump();
                    described(&s, placed, tol)
                }
            }))
            .unwrap_or_else(|p| {
                format!(
                    "PANIC {}",
                    p.downcast_ref::<&str>()
                        .map(|s| s.to_string())
                        .or_else(|| p.downcast_ref::<String>().cloned())
                        .unwrap_or_default()
                )
            });
            let _ = doc;
            if line.starts_with("PANIC") {
                panicked.push(format!("{name} at {seat}: {line}"));
            }
            eprintln!("  {seat:28} {line}");
        }
    }
    eprintln!(
        "\nexpected per member: union {:.3e}, intersect {:.3e}, v-c {:.3e}, c-v {:.3e}",
        B_VOL + C_VOL - OVERLAP,
        OVERLAP,
        B_VOL - OVERLAP,
        C_VOL - OVERLAP
    );
    assert!(panicked.is_empty(), "the evaluator panicked: {panicked:#?}");
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
    let pattern = w.variants[0].1;
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
/// pattern onto the split's `above ⊔ below`. Red today: the evaluator
/// panics in `split_port` (`eval/wire.rs`), "a split evaluates to its
/// two sides", because a split of a family evaluates to a family.
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
            plane: cap_end(body),
        });
        assert!(out.refusal.is_none(), "{:?}", out.refusal);
    });
    assert!(got.is_ok(), "the evaluation panicked: {got:?}");
}

/// The same, a transform of the pattern standing beside it (a
/// placement whose source is the pattern).
#[test]
fn a_split_of_a_transformed_pattern_evaluates() {
    let got = split_of_a_read_pattern(|session, pattern| {
        let (doc, _) = crate::fixture::slid(
            session.committed_doc().clone(),
            pattern,
            pattern,
            [0.0, 0.0, 0.005],
        );
        *session = common::inline_without_door(doc, Tol::witness());
    });
    assert!(got.is_ok(), "the evaluation panicked: {got:?}");
}
