//! MEASUREMENT (the BOXFINISH build-record probe; never for main): two
//! documents whose logs turn over a box while their cells may not, each
//! driven as main drives it, printing what the drive reports. Run with
//! `BUILD_RECORD_PROBE_DIR` set to have each leaf's record beside it.
//! Nothing here asserts: these rows report.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::{place_halves, revolved};
use crate::fixture::{Recorder, len, len2};
use crate::m10_3_driver_interval::uniform;

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DEFAULT_MAX_DEPTH, DriveConfig, drive};
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    Dimension, DocEdit, Formula, FreeVar, LoopProgram, Node, Operand, ProfileDoc, ProfileProgram,
    ProgramStep, ProgramTarget, UnitSym, VarName,
};
use geom_core::Tol;

fn report(what: &str, doc: &ProfileDoc, max_leaves: usize) {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let t0 = std::time::Instant::now();
    match drive(
        doc,
        &analyzed,
        &DriveConfig {
            max_depth: DEFAULT_MAX_DEPTH,
            max_leaves,
            ..DriveConfig::default()
        },
        &editor_core::Stop::default(),
        Tol::witness(),
    ) {
        Err(e) => println!("PROBE {what}: the drive refuses: {e:?}"),
        Ok(v) => {
            println!(
                "PROBE {what}: {:?} in {:?}; accounting {:?}",
                v.receipt(),
                t0.elapsed(),
                v.accounting()
            );
            for l in v.refused() {
                let reason: String = format!("{:?}", l.reason).chars().take(500).collect();
                println!("PROBE {what}:   refused {:?}: {reason}", l.box_);
            }
        }
    }
}

/// A cone cut by a plane square to its axis at the nominal, tilted by
/// `tilt = 0 ± half`.
fn tilted_cone_cut(half: f64, nominal: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("tilt"),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Angle,
            value: nominal,
            display_unit: UnitSym::canonical_for(Dimension::Angle),
            distribution: Some(uniform(half)),
        }),
    });
    let to = |x, y| ProgramTarget::Point(len2([x, y]));
    let cone = revolved(
        &mut r,
        vec![
            ProgramStep::At(len2([0.0, -1.5625])),
            ProgramStep::LineTo(to(0.75, -1.0)),
            ProgramStep::LineTo(to(0.0, -1.0)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ],
    );
    let own = editor_core::test_support::frame_on(
        &r.doc,
        &cone.body(),
        &editor_core::Frame::IDENTITY,
        Tol::witness(),
    );
    let pose = |def| Operand::Pose(Box::new(def));
    let slid = pose(PoseDef::Slide {
        pose: RoleRead::plain(own.clone()),
        along: RoleRead::through(own, PoseRole::Y),
        by: len(-1.25),
    });
    let tilted = pose(PoseDef::Spin {
        pose: RoleRead::plain(slid.clone()),
        about: RoleRead::through(slid, PoseRole::X),
        by: Formula::named(VarName::from_static("tilt"), Dimension::Angle),
    });
    let split = r.insert(Node::Split {
        target: cone.body().into(),
        tool: RoleRead::through(tilted, PoseRole::Zx),
    });
    place_halves(&mut r, split);
    r.doc
}

/// An extruded pentagon whose fourth corner sits at height
/// `h = nominal ± half`: convex above 1, straight at 1, reflex below.
fn turning_crease(nominal: f64, half: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("h"),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Length,
            value: nominal,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(uniform(half)),
        }),
    });
    let h = || Formula::named(VarName::from_static("h"), Dimension::Length);
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::polygon_expr([
            [len(0.0), len(0.0)],
            [len(2.0), len(0.0)],
            [len(2.0), len(1.0)],
            [len(1.0), h()],
            [len(0.0), len(1.0)],
        ])],
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
fn probe_a_tilted_plane_cuts_a_cone() {
    for (nominal, half) in [
        (0.0, 1e-9),
        (0.0, 1e-6),
        (0.0, 1e-3),
        (0.0, 5e-2),
        (0.05, 1e-3),
        (0.05, 0.1),
    ] {
        report(
            &format!("cone tilt {nominal} ± {half:e}"),
            &tilted_cone_cut(half, nominal),
            64,
        );
    }
}

#[test]
fn probe_a_crease_turns_from_convex_to_concave() {
    for (nominal, half) in [(1.1, 0.05), (1.1, 0.3), (0.9, 0.05), (1.0, 0.1)] {
        report(
            &format!("crease h {nominal} ± {half:e}"),
            &turning_crease(nominal, half),
            64,
        );
    }
}
