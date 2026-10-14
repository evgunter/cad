//! **The E6 drive over a loft at `Interval`, with the box elsewhere.**
//! A loft whose sections read no boxed parameter certifies when every
//! section value is a point (polygons on an unrotated frame), and
//! bisects to its leaf budget when one is a few ulps wide with no box
//! at all: an arc's sweep, or the plane of a rotated authored frame.
//! The loft refuses `SectionNotAPoint` / `PlacementNotAPoint` at every
//! leaf, which a smaller box cannot lift, and the drive has no terminal
//! class for it.
//!
//! Measured against the merge base of PR 4741 with the same documents:
//! main certified none of the three, refusing each in one leaf as a
//! `FlipCrossing` on a predicate-count divergence at the loft (no
//! verdict flipped). Both refusing rows wait on
//! `work/inlane/the-skin-at-interval-encloses-the-nominal-skin.md`,
//! and move the day it lands.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, Receipt, RefusalReason, drive};
use editor_core::{
    Dimension, Distribution, DocEdit, Formula, FreeVar, LoopProgram, Node, ProfileProgram, UnitSym,
    VarName,
};
use fixture::{Recorder, len};
use geom_core::Tol;

fn circle() -> LoopProgram<Formula> {
    LoopProgram::Circle {
        centre: [len(0.0), len(0.0)],
        radius: len(1.0),
    }
}

/// The rectangle `[0, width] × [0, 1]`.
fn rect(width: Formula) -> LoopProgram<Formula> {
    let at = |x: Formula, y: f64| {
        editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Point([x, len(y)]))
    };
    LoopProgram::Chain(vec![
        editor_core::ProgramStep::At([len(0.0), len(0.0)]),
        at(width.clone(), 0.0),
        at(width, 1.0),
        at(len(0.0), 1.0),
        editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Start),
    ])
}

/// A toleranced `w` read only by a profile of its own, and a loft of
/// `section` on two frames one unit apart whose in-plane axis is `u`.
/// The drive's box is `w`'s, and it reaches no loft section.
fn loft_doc(
    section: impl Fn() -> LoopProgram<Formula>,
    u: [f64; 3],
) -> (editor_core::ProfileDoc, editor_core::RecipeNodeId) {
    let half = Tol::witness().eps() / 16.0;
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("w"),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Length,
            value: 2.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half,
                hi: half,
            }),
        }),
    });
    let v = [-u[1], u[0], 0.0];
    let mut sections = Vec::new();
    for z in [0.0, 1.0] {
        let frame = r.insert(fixture::frame([0.0, 0.0, z], u, v));
        sections.push(r.insert(Node::Profile(ProfileProgram {
            frame: frame.into(),
            loops: vec![section()],
            ids: Vec::new(),
        })));
    }
    let loft = r.insert(Node::Loft {
        profiles: sections.into_iter().map(Into::into).collect(),
        v_degree: Formula::count(1),
    });
    let elsewhere = r.insert(fixture::frame(
        [5.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
    ));
    r.insert(Node::Profile(ProfileProgram {
        frame: elsewhere.into(),
        loops: vec![rect(Formula::named(
            VarName::from_static("w"),
            Dimension::Length,
        ))],
        ids: Vec::new(),
    }));
    (r.doc, loft)
}

/// [`loft_doc`] driven at 256 leaves.
fn drive_loft(
    section: impl Fn() -> LoopProgram<Formula>,
    u: [f64; 3],
) -> (Receipt, Vec<RefusalReason>) {
    let (doc, _) = loft_doc(section, u);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_leaves: 256,
            ..DriveConfig::default()
        },
        Tol::witness(),
    )
    .expect("the drive runs");
    let reasons = verdict.refused().iter().map(|l| l.reason.clone()).collect();
    (verdict.receipt(), reasons)
}

/// Every refused leaf is budget mass, and nothing certified.
fn assert_bisected_to_budget(label: &str, (receipt, reasons): (Receipt, Vec<RefusalReason>)) {
    assert_eq!(receipt.certified, 0, "{label}: {receipt:?}");
    assert_eq!(receipt.refused, 256, "{label}: {receipt:?}");
    assert!(
        reasons
            .iter()
            .all(|r| matches!(r, RefusalReason::Budget(_))),
        "{label}: {:?}",
        reasons.first()
    );
}

#[test]
fn a_point_polygon_loft_certifies_under_a_box_elsewhere() {
    let (receipt, reasons) = drive_loft(|| rect(len(1.0)), [1.0, 0.0, 0.0]);
    assert_eq!(
        receipt,
        Receipt {
            certified: 1,
            refused: 0,
            splits: 0
        },
        "{:?}",
        reasons.first()
    );
}

#[test]
fn an_arc_loft_bisects_to_its_budget_under_a_box_elsewhere() {
    assert_bisected_to_budget("arc", drive_loft(circle, [1.0, 0.0, 0.0]));
}

#[test]
fn a_rotated_frame_loft_bisects_to_its_budget_under_a_box_elsewhere() {
    assert_bisected_to_budget("rotated", drive_loft(|| rect(len(1.0)), [0.6, 0.8, 0.0]));
}

/// The refusal behind each budget row, one evaluation at `Interval`
/// with no box: under the guided lift (the drive's) the arc section
/// and the rotated plane are a few ulps wide and refuse by name; under
/// the pinned lift both are the `f64` form lifted, and loft.
#[test]
fn with_no_box_the_guided_lift_refuses_an_arc_section_and_a_rotated_plane() {
    use editor_core::{NodeErrorKind, ProfileLift};
    use geom_core::Interval;
    use sweep::LoftError;
    let run = |doc: &editor_core::ProfileDoc, lift| {
        editor_core::evaluate::<Interval>(
            doc,
            None,
            &editor_core::CancelToken::new(),
            &editor_core::EvalOptions {
                profile_lift: lift,
                ..editor_core::EvalOptions::default()
            },
            Tol::witness(),
        )
    };
    let (arc, arc_loft) = loft_doc(circle, [1.0, 0.0, 0.0]);
    let (rotated, rotated_loft) = loft_doc(|| rect(len(1.0)), [0.6, 0.8, 0.0]);
    for (doc, loft) in [(&arc, arc_loft), (&rotated, rotated_loft)] {
        let pinned = run(doc, ProfileLift::Pinned);
        assert!(
            pinned.node_error(loft).is_none(),
            "{:?}",
            pinned.node_error(loft)
        );
    }
    let guided = run(&arc, ProfileLift::Guided);
    match guided.node_error(arc_loft).map(|e| &e.kind) {
        Some(NodeErrorKind::Loft(LoftError::SectionNotAPoint { section: 0 })) => {}
        other => panic!("the arc section is no point at Interval, got {other:?}"),
    }
    let guided = run(&rotated, ProfileLift::Guided);
    match guided.node_error(rotated_loft).map(|e| &e.kind) {
        Some(NodeErrorKind::Loft(LoftError::PlacementNotAPoint { section: 0 })) => {}
        other => panic!("the rotated plane is no point at Interval, got {other:?}"),
    }
}
