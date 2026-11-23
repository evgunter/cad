//! **A rectangle whose width is a parameter box certifies over the box
//! through the driver**: drawn with the pen (heading 0, two quarter
//! turns, closed onto its start) and as four formula corners, extruded,
//! and driven at `Sym<Interval>` with its leaves replayed guided.
//!
//! The arm this row exists for is the pen turned by `π/2` written in
//! radians over ±0.05, the document `profile/a-pen-drawn-rectangle-with-a-boxed-side-refuses-line-span-over-any-box`
//! measured. There the profile node refused `Escalated { site:
//! SegmentPair(0, 3), gate: LineSpan }`: the closing pair's shared vertex
//! was read off the chord it ends, with the box on every factor
//! (`profile::seg`'s `line_line` now takes it by construction). So that
//! arm also replays the root box once and asserts the profile node builds.
//! The other arms passed before that change and are guards: the pen by
//! `turn/4`, the corners, and every arm at ±1e-6.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{Recorder, ang, len};

use crate::m10_8_arc_family_interval::replay;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_MAX_DEPTH, DriveConfig, drive};
use editor_core::{
    Dimension, Distribution, DocEdit, Formula, FreeVar, LoopProgram, Node, ProfileDoc,
    ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, UnitSym, VarName,
};
use geom_core::{SymRules, Tol};

/// How the rectangle is spelled.
#[derive(Clone, Copy, Debug)]
enum Form {
    /// The pen, turning by `turn/4`.
    PenTurns,
    /// The pen, turning by `π/2` written in radians.
    PenRadians,
    /// Four corners.
    Corners,
}

/// The width `w = 1 ± half`, depth and height 1, and its profile node.
fn slab(half: f64, form: Form) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("w"),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Length,
            value: 1.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half,
                hi: half,
            }),
        }),
    });
    let w = || Formula::named(VarName::from_static("w"), Dimension::Length);
    let quarter = || match form {
        Form::PenRadians => ang(core::f64::consts::FRAC_PI_2),
        _ => Formula::turn_over(4).expect("a right angle"),
    };
    let lp = if !matches!(form, Form::Corners) {
        LoopProgram::Chain(vec![
            ProgramStep::At([len(0.0), len(0.0)]),
            ProgramStep::Angle(ang(0.0)),
            ProgramStep::Line(w()),
            ProgramStep::Turn(quarter()),
            ProgramStep::Line(len(1.0)),
            ProgramStep::Turn(quarter()),
            ProgramStep::Line(w()),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])
    } else {
        LoopProgram::polygon_expr([
            [len(0.0), len(0.0)],
            [w(), len(0.0)],
            [w(), len(1.0)],
            [len(0.0), len(1.0)],
        ])
    };
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![lp],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    (r.doc, p)
}

#[test]
fn a_pen_drawn_and_a_cornered_rectangle_certify_over_a_boxed_width() {
    for form in [Form::PenTurns, Form::PenRadians, Form::Corners] {
        for half in [1e-6, 0.05] {
            let (doc, profile) = slab(half, form);
            let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
            if matches!(form, Form::PenRadians) && half == 0.05 {
                let (_, refusal, _) = replay(
                    &doc,
                    &ParamBox::of(&analyzed),
                    SymRules::shipped(),
                    Tol::witness(),
                );
                let at_profile = format!("node {} ", profile.0);
                assert!(
                    !refusal.as_ref().is_some_and(|r| r.starts_with(&at_profile)),
                    "the item's witness: the profile node refuses over the root box: {refusal:?}"
                );
            }
            let v = drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    max_depth: DEFAULT_MAX_DEPTH,
                    max_leaves: 32,
                    ..DriveConfig::default()
                },
                &editor_core::Stop::default(),
                Tol::witness(),
            )
            .expect("the drive runs");
            let reasons: Vec<String> = v
                .refused()
                .iter()
                .map(|l| format!("{:?}", l.reason).chars().take(400).collect())
                .collect();
            assert!(
                reasons.is_empty(),
                "{form:?}, half {half:e}: refused leaves {reasons:#?}"
            );
            assert!(
                v.receipt().holds(),
                "{form:?}, half {half:e}: {:?}",
                v.receipt()
            );
            let certified = v.accounting().certified.clone().expect("a measure");
            assert!(
                (certified - 1.0).abs() < 1e-12,
                "{form:?}, half {half:e}: certified mass {certified}"
            );
        }
    }
}
