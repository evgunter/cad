//! Rows for the unlogged lifted closed-form pass (ledger row F48 in
//! `docs/predicate-dimension-audit.md`): what an `f64` witness and an
//! `Interval` replay of one document record, and what the drive
//! certifies and refuses on bodies with curved closed-form faces.
//!
//! The parity, member and named-flip rows began as the probes of PR
//! 7716's review.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::corpus;
use crate::fixture::tol;

use corpus::plate_param::{HOLE_CENTRES, HOLE_R, HOLE_R_VALUE, PLATE_DEPTH};
use corpus::{body_of, eval};
use editor_core::VerdictVector;
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, RefusalReason, certifying_vector};
use editor_core::range::{RangeField, RangeSeed, derive};
use editor_core::{
    Dimension, DocEdit, EvalOutcome, Evaluation, FreeVar, ProfileDoc, VarName, apply,
};
use geom_core::{Interval, Tol};

/// Per predicate, how many definite verdicts a vector holds.
fn counts(v: &VerdictVector) -> BTreeMap<&'static str, usize> {
    let mut out = BTreeMap::new();
    for row in &v.rows {
        for verdict in &row.verdicts {
            *out.entry(verdict.predicate).or_insert(0) += 1;
        }
    }
    out
}

/// The rows of a vector with their node ids dropped: two documents that
/// differ in one variable's value may mint other ids.
fn shape(v: &VerdictVector) -> Vec<(String, Vec<String>)> {
    v.rows
        .iter()
        .map(|r| {
            (
                format!("{:?}", r.outcome),
                r.verdicts.iter().map(|d| format!("{d:?}")).collect(),
            )
        })
        .collect()
}

/// A document's two vectors at one point: the `f64` witness's and an
/// `Interval` replay's.
fn vectors(doc: &ProfileDoc) -> (VerdictVector, VerdictVector) {
    let witness: Evaluation<f64> = eval(doc);
    let leaf: Evaluation<Interval> = eval(doc);
    assert_eq!(witness.outcome, EvalOutcome::Completed);
    assert_eq!(leaf.outcome, EvalOutcome::Completed);
    (
        certifying_vector(doc, &witness),
        certifying_vector(doc, &leaf),
    )
}

/// The `props_*` kinds of a vector, with their counts.
fn props(v: &VerdictVector) -> BTreeMap<&'static str, usize> {
    counts(v)
        .into_iter()
        .filter(|(name, _)| name.starts_with("props_"))
        .collect()
}

/// The curved corpus documents whose two vectors are equal. Ten more
/// corpus documents part, on predicates other than `props_*`
/// (`work/boxfinish/the-two-lanes-decision-logs-differ-at-one-point-with-the-build-the-same`).
const CURVED: &[&str] = &["plate_param", "cut_cylinder", "cup", "pin_in_bore"];

/// A revolved ball or cone and a block standing clear of it, unioned:
/// the smallest taken body whose sphere or cone wall's `props_*` reach
/// a verdict vector (a body no node takes is finished off the node's
/// log at `f64` and never at a leaf).
fn round_beside_a_block(cone: bool) -> ProfileDoc {
    use crate::corpus::{Recorder, placed_in, revolved};
    use crate::fixture::{len, len2, scl};
    use editor_core::{
        LoopProgram, Node, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget,
    };
    let mut r = Recorder::new();
    let to = |x, y| ProgramTarget::Point(len2([x, y]));
    let meridian = if cone {
        vec![
            ProgramStep::At(len2([0.0, 1.5])),
            ProgramStep::LineTo(to(0.75, 0.0)),
            ProgramStep::LineTo(to(0.0, 0.0)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ]
    } else {
        vec![
            ProgramStep::At(len2([0.0, -1.25])),
            ProgramStep::ArcTo(ProgramArcData::Bulge {
                target: to(0.0, 1.25),
                b: scl(1.0),
            }),
            ProgramStep::LineTo(ProgramTarget::Start),
        ]
    };
    let round = revolved(&mut r, meridian);
    let block_p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([(4.0, 4.0), (5.0, 4.0), (5.0, 5.0), (4.0, 5.0)])
                .expect("finite block corners"),
        ],
        ids: Vec::new(),
        on: None,
    }));
    let block = r.insert(Node::Extrude {
        profile: block_p.into(),
        distance: len(0.5),
    });
    let placed = placed_in(
        &mut r,
        &round.body(),
        editor_core::Operand::from(block),
        editor_core::stand(),
    );
    r.insert(Node::Union {
        members: editor_core::Bodies::spelled(vec![
            editor_core::BodyRead::plain(round.body()),
            placed.into(),
        ]),
    });
    r.doc
}

/// **An `f64` witness and an `Interval` replay at one point state one
/// verdict vector on the curved corpus documents and on a ball beside
/// a block**: cylinder walls (the corpus), a revolved wall (`cup`) and
/// a sphere, whose `props_sphere_*` premises are on both vectors.
///
/// Red with the lifted pass logged (`unlogged` taken out of `topo`'s
/// `lifted_closed_form`): the witness lists each curved face's
/// premises once more than the replay does.
#[test]
fn a_point_replay_states_the_witnesss_vector_on_curved_documents() {
    let extra = [corpus::cup::document()];
    let mut parted = Vec::new();
    for doc in corpus::documents().into_iter().chain(extra) {
        if !CURVED.contains(&doc.name) {
            continue;
        }
        let (witness, leaf) = vectors(&doc.doc);
        if doc.name == "plate_param" {
            assert!(
                props(&witness).contains_key("props_rim_fit"),
                "the plate's hole walls' premises are not on the vector"
            );
        }
        if witness != leaf {
            parted.push(doc.name);
        }
    }
    let (witness, leaf) = vectors(&round_beside_a_block(false));
    assert!(
        props(&witness).contains_key("props_sphere_circle_fit"),
        "the ball's premises are not on the vector: {:?}",
        props(&witness)
    );
    if witness != leaf {
        parted.push("a ball beside a block");
    }
    assert!(parted.is_empty(), "vectors part on {parted:?}");
}

/// **A cone's premises are on both vectors as often**: a cone beside a
/// block lists every `props_*` kind at the witness as many times as at
/// the replay, its `props_cone_*` among them.
///
/// The whole vectors are not held equal: they part on the count of
/// `pcurve_projected_lever` rounds, at both scalars' own revolve
/// (`work/isotail/limb-2-logs-other-decisions-at-an-enclosure-scalar-than-at-the-witness`).
///
/// Red with the lifted pass logged: the witness's counts are the
/// replay's and half again.
#[test]
fn a_cones_premises_are_on_both_vectors_as_often() {
    let (witness, leaf) = vectors(&round_beside_a_block(true));
    let (w, l) = (props(&witness), props(&leaf));
    assert!(
        w.contains_key("props_cone_nappe"),
        "the cone's premises are not on the vector: {w:?}"
    );
    assert_eq!(w, l, "the two vectors count a premise differently");
}

fn set_hole_r(doc: &ProfileDoc, value: f64) -> ProfileDoc {
    apply(
        doc,
        &DocEdit::DefineVar {
            var: VarName::from_static(HOLE_R).into(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, value)),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("DefineVar does not refuse")
    .doc
}

/// The plate's holes as its body stores them: each distinct cylinder's
/// axis foot in the profile plane and its radius, in order.
fn holes(doc: &ProfileDoc, solid: editor_core::RecipeNodeId) -> Vec<(f64, f64, f64)> {
    let ev: Evaluation<f64> = eval(doc);
    let body = body_of(&ev, solid);
    let mut out: Vec<(f64, f64, f64)> = body
        .faces()
        .filter_map(|(key, _)| match body.face_surface(key) {
            Ok(geom::Surface::Cylinder { frame, radius, .. }) => {
                let o = frame.origin();
                Some((o.x, o.y, *radius))
            }
            _ => None,
        })
        .collect();
    out.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // A hole's wall is two faces on one cylinder.
    out.dedup();
    out
}

fn volume(doc: &ProfileDoc, solid: editor_core::RecipeNodeId) -> f64 {
    let ev: Evaluation<f64> = eval(doc);
    assert_eq!(ev.outcome, EvalOutcome::Completed);
    topo::test_support::pinned_volume(
        "member",
        &topo::test_support::finished("member", topo::Body::clone(&body_of(&ev, solid)), tol()),
        tol(),
    )
    .volume
}

/// **What certifies over the plate's `[0, ε/4]` leaf is the witness's
/// build at members of the leaf**: an `f64` build at each sampled
/// member states the witness's decisions row for row, its two holes
/// stand where the witness's do at the member's radius, and its volume
/// is the witness's less the two holes' growth.
///
/// This is the row that says what the leaf's certificate is worth;
/// `docm9_range`'s narrow-leaf row says only that the drive certifies.
///
/// Red with the lifted pass logged (`unlogged` taken out of `topo`'s
/// `lifted_closed_form`): the leaf is refused.
#[test]
fn members_of_the_plates_certified_leaf_build_as_the_witness_did() {
    let corpus_doc = corpus::plate_param::document();
    let doc = corpus_doc.doc;
    let solid = corpus_doc.result.unwrap();
    let field = RangeField::Param(doc.var_named(HOLE_R).unwrap());
    let width = tol().eps() / 4.0;
    let derived = derive(&doc, &field, RangeSeed { lo: 0.0, hi: width }, tol()).expect("admitted");
    let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
    let level = DriveConfig {
        max_depth: 0,
        ..DriveConfig::default()
    };
    let stop = editor_core::Stop::default();
    let verdict = editor_core::drive::drive(&derived.doc, &analyzed, &level, &stop, tol())
        .expect("the plate builds");
    assert_eq!(verdict.certified().len(), 1, "{:?}", verdict.refused());

    let witness: Evaluation<f64> = eval(&doc);
    let wshape = shape(&certifying_vector(&doc, &witness));
    let v0 = volume(&doc, solid);
    for k in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let r = HOLE_R_VALUE + k * width;
        let member = set_hole_r(&doc, r);
        let ev: Evaluation<f64> = eval(&member);
        assert_eq!(ev.outcome, EvalOutcome::Completed, "member {k} builds");
        assert!(
            shape(&certifying_vector(&member, &ev)) == wshape,
            "the member at {k} of the leaf decides other than the witness"
        );
        let stood = holes(&member, solid);
        let want: Vec<(f64, f64, f64)> = HOLE_CENTRES.iter().map(|&(x, y)| (x, y, r)).collect();
        assert_eq!(stood, want, "member {k}: the holes stand elsewhere");
        let grown =
            2.0 * std::f64::consts::PI * (r * r - HOLE_R_VALUE * HOLE_R_VALUE) * PLATE_DEPTH;
        let v = volume(&member, solid);
        assert!(
            (v - (v0 - grown)).abs() < 1e-13,
            "member {k}: volume {v} against {}",
            v0 - grown
        );
    }
}

/// **A flip on a body with a curved face is still refused, named**: a
/// unit square whose bottom side carries a vertex at a parameter height
/// (convex below zero, a notch above), with a round hole, extruded. The
/// leaves short of the crossing certify (the hole's wall is a curved
/// closed-form face), and every leaf wholly past it is a flip crossing
/// whose evidence names a predicate.
///
/// Red with the lifted pass logged (`unlogged` taken out of `topo`'s
/// `lifted_closed_form`): no leaf certifies.
#[test]
fn a_flip_on_a_body_with_a_round_hole_is_refused_named() {
    use crate::fixture::{Recorder, len};
    use editor_core::{Formula, LoopProgram, Node, ProfileProgram};
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("height"),
        def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, -0.125)),
    });
    let height = Formula::named(VarName::from_static("height"), Dimension::Length);
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon_expr([
                [len(0.0), len(0.0)],
                [len(0.5), height],
                [len(1.0), len(0.0)],
                [len(1.0), len(1.0)],
                [len(0.0), len(1.0)],
            ]),
            LoopProgram::Circle {
                centre: [len(0.5), len(0.625)],
                radius: len(0.125),
            },
        ],
        ids: Vec::new(),
        on: None,
    }));
    let block = r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    // A taker, as the plate has: a body no node takes is finished off
    // the node's log at `f64` and never at a leaf, so its mass
    // properties' decisions reach neither vector (measured: without the
    // union this row is green with the lifted pass logged too).
    let tab_p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([(0.75, 0.75), (1.25, 0.75), (1.25, 1.25), (0.75, 1.25)])
                .expect("finite tab corners"),
        ],
        ids: Vec::new(),
        on: None,
    }));
    let tab = r.insert(Node::Extrude {
        profile: tab_p.into(),
        distance: len(0.5),
    });
    let placed = r.against(
        block,
        tab,
        &editor_core::Frame::translation([0.0, 0.0, 0.25]),
    );
    r.insert(Node::Union {
        members: editor_core::Bodies::spelled(vec![block.into(), placed.into()]),
    });
    let doc = r.doc;
    let field = RangeField::Param(doc.var_named("height").unwrap());
    // The crossing is at offset 0.125; the seed reaches 0.0625 past it.
    let seed = RangeSeed {
        lo: 0.0,
        hi: 0.1875,
    };
    let derived = derive(&doc, &field, seed, tol()).expect("admitted");
    let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
    let level = DriveConfig {
        max_depth: 5,
        ..DriveConfig::default()
    };
    let stop = editor_core::Stop::default();
    let verdict = editor_core::drive::drive(&derived.doc, &analyzed, &level, &stop, tol())
        .expect("the notched square builds");
    let mut named = 0;
    let mut unnamed = 0;
    for leaf in verdict.refused() {
        match &leaf.reason {
            RefusalReason::FlipCrossing { flipped } if flipped.is_empty() => unnamed += 1,
            RefusalReason::FlipCrossing { .. } => {
                named += 1;
            }
            _ => {}
        }
    }
    assert!(
        !verdict.certified().is_empty(),
        "no leaf short of the crossing certifies"
    );
    assert!(named > 0, "no leaf past the crossing is a named flip");
    assert_eq!(unnamed, 0, "a flip crossing names nothing");
}
