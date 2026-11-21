//! **A rectangle whose width is a parameter box certifies over the box**,
//! drawn with the pen (heading 0, turns of `turn/4`, closed onto its start)
//! and as four formula corners, extruded and driven at `Sym<Interval>`.
//! Each adjacent pair shares a stored vertex, whose side and span the
//! profile's pair pass takes by construction (`profile::seg`'s
//! `line_line`); read, they straddle the band over any box wider than
//! about `1e-11`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{Recorder, ang, len};

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DEFAULT_MAX_DEPTH, DriveConfig, drive};
use editor_core::{
    Dimension, Distribution, DocEdit, Formula, FreeVar, LoopProgram, Node, ProfileDoc,
    ProfileProgram, ProgramStep, ProgramTarget, UnitSym, VarName,
};
use geom_core::Tol;

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

/// The width `w = 1 ± half`, depth and height 1.
fn slab(half: f64, form: Form) -> ProfileDoc {
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
    r.doc
}

#[test]
fn a_pen_drawn_and_a_cornered_rectangle_certify_over_a_boxed_width() {
    for form in [Form::PenTurns, Form::PenRadians, Form::Corners] {
        for half in [1e-6, 0.05] {
            let doc = slab(half, form);
            let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
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
            assert!(v.receipt().holds(), "{form:?}, half {half:e}: {:?}", v.receipt());
            let certified = v.accounting().certified.clone().expect("a measure");
            assert!(
                (certified - 1.0).abs() < 1e-12,
                "{form:?}, half {half:e}: certified mass {certified}"
            );
        }
    }
}
