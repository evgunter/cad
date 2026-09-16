//! REVIEW PROBE (lane `wire-rv`, PR #2738): the persisted bytes of a
//! profile program that reaches every `ProgramStep` variant, every
//! `ProgramArcData` mode, both sides, both windings, every
//! `ProgramTarget` form and all three `LoopProgram` forms.
//!
//! It writes the serialization to `$WIRE_RV_OUT` so the same corpus can
//! be run on main's kernel and on the PR head and the bytes diffed.
//! Deliberately self-contained (public API only) so it cherry-picks
//! onto either tree.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    Dimension, Expr, LoopProgram, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget,
};

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).unwrap()
}
fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).unwrap()
}
fn sca(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).unwrap()
}
fn pt(x: f64, y: f64) -> [Expr; 2] {
    [len(x), len(y)]
}
fn point(x: f64, y: f64) -> ProgramTarget {
    ProgramTarget::Point(pt(x, y))
}

fn every_spec() -> Vec<ProgramArcData> {
    let mut out = Vec::new();
    for side in [profile::ArcSide::Left, profile::ArcSide::Right] {
        out.push(ProgramArcData::Radius { r: len(2.0), side });
        out.push(ProgramArcData::Sweep {
            r: len(1.5),
            side,
            angle: ang(0.6),
        });
        out.push(ProgramArcData::ArcLen {
            r: len(1.75),
            side,
            len: len(0.9),
        });
    }
    for target in [
        ProgramTarget::Start,
        ProgramTarget::StartArriving,
        point(2.0, 1.0),
    ] {
        out.push(ProgramArcData::Bulge {
            target: target.clone(),
            b: sca(0.3),
        });
        out.push(ProgramArcData::Via {
            q: pt(4.5, 0.5),
            target: target.clone(),
        });
        for winding in [profile::ArcSweep::Ccw, profile::ArcSweep::Cw] {
            out.push(ProgramArcData::Center {
                c: pt(6.0, 1.0),
                winding,
                target: target.clone(),
            });
        }
    }
    out
}

fn steps() -> Vec<ProgramStep> {
    let mut steps = vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::Angle(ang(0.25)),
        ProgramStep::Toward {
            dx: sca(1.0),
            dy: sca(0.5),
        },
        ProgramStep::Tangent,
        ProgramStep::Cusp,
        ProgramStep::Turn(ang(0.1)),
        ProgramStep::Line(len(1.0)),
        ProgramStep::Fillet(len(0.2)),
        ProgramStep::FarEndTo(pt(7.0, 2.0)),
        ProgramStep::CloseTo,
    ];
    for target in [
        ProgramTarget::Start,
        ProgramTarget::StartArriving,
        point(3.0, 0.0),
    ] {
        steps.push(ProgramStep::LineTo(target.clone()));
        steps.push(ProgramStep::ContinueTo(target.clone()));
        steps.push(ProgramStep::TangentArcTo(target.clone()));
    }
    for spec in every_spec() {
        steps.push(ProgramStep::ArcTo(spec.clone()));
        steps.push(ProgramStep::FilletArc {
            radius: len(0.3),
            spec: spec.clone(),
        });
        steps.push(ProgramStep::ArcFillet {
            spec: spec.clone(),
            radius: len(0.4),
        });
        steps.push(ProgramStep::ArcFilletArc {
            spec: spec.clone(),
            radius: len(0.5),
            spec2: spec.clone(),
        });
    }
    steps
}

fn program() -> ProfileProgram {
    ProfileProgram {
        plane: editor_core::RecipeNodeId(0),
        loops: vec![
            LoopProgram::Chain(steps()),
            LoopProgram::circle(1.0, 1.0, 0.5).unwrap(),
            LoopProgram::circle_split(2.0, 2.0, 0.75, 5, 0.2).unwrap(),
        ],
    }
}

#[test]
fn wire_rv_every_variant_serializes() {
    let text = serde_json::to_string_pretty(&program()).expect("serializes");
    let back: ProfileProgram = serde_json::from_str(&text).expect("deserializes");
    assert_eq!(back, program(), "round trip");
    if let Ok(path) = std::env::var("WIRE_RV_OUT") {
        std::fs::write(&path, &text).expect("write");
    }
}
