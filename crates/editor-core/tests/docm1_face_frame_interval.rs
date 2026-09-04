//! **DOCM-1 under the Interval lane** — DM1c's rows: a profile on a
//! derived frame is placed at the lane scalar under EVERY lift (its
//! 2-D structure record still `f64`-pinned), a loft section on a
//! derived frame refuses typed off `f64`, and a widened parameter the
//! frame's body reads recomputes the profile rather than serving the
//! nominal memo entry.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::corpus;
use crate::docm1_face_frame::lofted_on_face_frame;
use crate::fixture::{self, Recorder, ang};

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{
    CancelToken, CapEnd, Datum, Dimension, Distribution, DocEdit, DocParam, EvalOptions,
    Evaluation, Expr, Node, NodeError, NodeErrorKind, NodeResult, ParamName, ProfileDoc,
    ProfileLift, RecipeNodeId, RoleSeg, UnitSym, ValuePayload, evaluate,
};
use geom_core::{Bounds, Interval, Tol};
use topo::{DatumValue, UnitVec3, validate_closed};

fn run(
    doc: &ProfileDoc,
    prior: Option<&Evaluation<Interval>>,
    opts: &EvalOptions,
) -> Evaluation<Interval> {
    evaluate::<Interval>(doc, prior, &CancelToken::new(), opts, Tol::witness())
}

fn lifted(lift: ProfileLift) -> EvalOptions {
    EvalOptions {
        profile_lift: lift,
        ..EvalOptions::default()
    }
}

/// **The corpus document's profile is placed at the lane scalar under
/// both lifts**: the profile on the derived frame evaluates green at
/// `Interval`, and the boss's bottom encloses the cap's height —
/// which is the frame's landed origin, not a placeholder.
#[test]
fn a_profile_on_a_derived_frame_is_placed_at_the_lane_scalar_under_every_lift() {
    let cd = corpus::face_sketch::document();
    let boss = cd.result.expect("the boss");
    let frame = cd
        .doc
        .order()
        .iter()
        .copied()
        .find(|id| matches!(cd.doc.node(*id), Some(Node::Datum(Datum::FaceFrame { .. }))))
        .expect("the derived frame");
    for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
        let ev = run(&cd.doc, None, &lifted(lift));
        assert!(
            corpus::failures(&ev).is_empty(),
            "{lift:?}: {:?}",
            corpus::failures(&ev)
        );
        let ValuePayload::Datum(DatumValue::Frame { origin, u, v }) =
            &ev.value(frame).expect("the frame").payload
        else {
            panic!("a frame value");
        };
        assert!(
            origin.z.lo() <= 1.0 && 1.0 <= origin.z.hi(),
            "{lift:?}: origin {origin:?}"
        );
        let n = UnitVec3::get(*u).cross(UnitVec3::get(*v));
        assert!(
            n.z.lo() <= 1.0 && 1.0 <= n.z.hi(),
            "{lift:?}: outward normal {n:?}"
        );
        let body = corpus::body_of(&ev, boss);
        assert_eq!(validate_closed(body), Ok(()), "{lift:?}");
        let bottom = body
            .points()
            .map(|(_, p)| p.z.lo())
            .fold(f64::INFINITY, f64::min);
        assert!(
            (bottom - 1.0).abs() <= 1e-9,
            "{lift:?}: the boss stands on the cap: {bottom}"
        );
    }
}

/// **A loft section on a derived frame refuses typed off f64**, naming
/// the profile and the frame (the f64 half of the row evaluates, in
/// the f64 suite).
#[test]
fn a_section_on_a_derived_frame_refuses_derived_frame_section_at_interval() {
    let (doc, loft) = lofted_on_face_frame();
    let ev = run(&doc, None, &EvalOptions::default());
    let frame = doc
        .order()
        .iter()
        .copied()
        .find(|id| matches!(doc.node(*id), Some(Node::Datum(Datum::FaceFrame { .. }))))
        .expect("the derived frame");
    let Some(Node::Loft { profiles, .. }) = doc.node(loft) else {
        panic!("the loft");
    };
    let section = profiles[0];
    match ev.nodes.get(&loft) {
        Some(NodeResult::Failed(NodeError {
            kind:
                NodeErrorKind::DerivedFrameSection {
                    profile,
                    frame: named,
                },
            ..
        })) => {
            assert_eq!(*profile, section);
            assert_eq!(*named, frame);
        }
        other => panic!("the loft must refuse typed, got {other:?}"),
    }
    // The profile itself is fine: it is the SECTION use that has no
    // f64 placement, not the profile.
    assert!(matches!(ev.nodes.get(&section), Some(NodeResult::Ok(_))));
}

/// A box whose height is a document parameter with a declared
/// distribution, and a boss profile sketched on its top face.
fn boxed_on_param(width: f64) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("h"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 1.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -width,
                hi: width,
            }),
        },
    });
    let (plane, profile) = r.profile_keeping(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 1.0)],
    );
    let _ = plane;
    let cube = r.insert(Node::Extrude {
        profile,
        distance: Expr::param(ParamName::new("h"), Dimension::Length),
    });
    let frame = r.insert(Node::Datum(Datum::FaceFrame {
        at: cube,
        face: fixture::fname(cube, RoleSeg::Cap(CapEnd::Top)),
        spin: ang(0.0),
    }));
    // The document ends at the boss PROFILE: the row below measures the
    // profile's memo and its carried placement, and an interval
    // extrude of a profile whose placement carries width refuses its
    // own endpoint certification at the witness ε — a fact about the
    // extrude's certification, not about the frame.
    let boss_p = r.insert(Node::Profile(fixture::desc(
        frame,
        vec![fixture::square(0.0, 0.0, 0.5)],
    )));
    (r.doc, boss_p)
}

/// **PP5 for the derived placement**: widening the parameter the
/// frame's body reads recomputes the profile on the frame — the
/// widened placement cannot be served from the nominal memo entry.
///
/// The widening is hair-thin (±1e-10): the interval extrude's own
/// endpoint certification refuses a height bracket wide enough to
/// straddle its margin band, and what this row measures is the memo
/// and the carried width, not how wide a box the lane can build.
#[test]
fn widening_the_frames_body_parameter_recomputes_the_profile() {
    let (doc, boss_p) = boxed_on_param(1e-10);
    let nominal = run(&doc, None, &EvalOptions::default());
    assert!(
        corpus::failures(&nominal).is_empty(),
        "{:?}",
        corpus::failures(&nominal)
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        ..EvalOptions::default()
    };
    let widened = run(&doc, Some(&nominal), &opts);
    assert!(
        corpus::failures(&widened).is_empty(),
        "{:?}",
        corpus::failures(&widened)
    );
    // The box's own frame and profile read no parameter and are
    // served from the memo; everything from the box up — the derived
    // frame and the boss profile — recomputes.
    assert_eq!(widened.reused, 2, "the two parameter-free leaves");
    assert_eq!(widened.recomputed, doc.len() - 2);
    let ValuePayload::Profile(p) = &widened.value(boss_p).expect("the boss profile").payload else {
        panic!("a profile");
    };
    let z = p.validated.plane().placement.translation.z;
    assert!(
        z.hi() - z.lo() >= 1.9e-10,
        "the placement carries the widened height: {z:?}"
    );
}
