//! **The E6 drive over a loft at `Interval`, with the box elsewhere.**
//! A loft whose sections read no boxed parameter certifies in one leaf
//! when its sections are polygons, standing at its own frame (every
//! value a point) or placed on a rotated one (the copy's frame a few
//! ulps wide, which the placement moves at the lane scalar). An arc
//! section is a few ulps wide with no box at all, and an arc's weight
//! is no point at `Interval`: its sweep is certified (a full turn, four
//! sub-arcs) and the carrier's door refuses the weight
//! (`SplineError::CarrierWeightNotAPoint`, C6's interim until S4/S5) at
//! every leaf, which a smaller box cannot lift, and the drive bisects
//! to its leaf budget
//! (`work/inlane/an-enclosure-carrier-holds-point-weights-and-knots.md`).
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
/// `section` over two sections one unit apart up its own z — placed in
/// the world on the frame whose in-plane axis is `u` where `u` is not
/// the world's x. The drive's box is `w`'s, and it reaches no loft
/// section. Returns the document and the loft, or its placement.
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
    for _ in 0..2 {
        sections.push(r.insert(Node::Profile(ProfileProgram {
            loops: vec![section()],
            ids: Vec::new(),
            on: None,
        })));
    }
    let loft = r.insert(Node::Loft {
        first: sections[0].into(),
        then: vec![editor_core::LoftSection {
            profile: sections[1].into(),
            station: vec![editor_core::StationMove::Slide {
                along: editor_core::Axis3::Z,
                by: len(1.0),
            }],
        }],
        v_degree: Formula::count(1),
    });
    let loft = if u == [1.0, 0.0, 0.0] {
        loft
    } else {
        r.in_world(loft, &fixture::axes_at([0.0; 3], u, v))
    };
    r.insert(Node::Profile(ProfileProgram {
        loops: vec![rect(Formula::named(
            VarName::from_static("w"),
            Dimension::Length,
        ))],
        ids: Vec::new(),
        on: None,
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

/// One leaf certifies, nothing splits.
fn assert_certified_in_one_leaf(label: &str, (receipt, reasons): (Receipt, Vec<RefusalReason>)) {
    assert_eq!(
        receipt,
        Receipt {
            certified: 1,
            refused: 0,
            splits: 0
        },
        "{label}: {:?}",
        reasons.first()
    );
}

#[test]
#[ignore = "disabled red: work/isotail/a-loft-drive-at-interval-refuses-every-leaf-since-the-one-polynomial-weight-test"]
fn a_point_polygon_loft_certifies_under_a_box_elsewhere() {
    assert_certified_in_one_leaf("point", drive_loft(|| rect(len(1.0)), [1.0, 0.0, 0.0]));
}

#[test]
fn an_arc_loft_bisects_to_its_budget_under_a_box_elsewhere() {
    assert_bisected_to_budget("arc", drive_loft(circle, [1.0, 0.0, 0.0]));
}

#[test]
#[ignore = "disabled red: work/isotail/a-loft-drive-at-interval-refuses-every-leaf-since-the-one-polynomial-weight-test"]
fn a_loft_placed_on_a_rotated_frame_certifies_under_a_box_elsewhere() {
    assert_certified_in_one_leaf("rotated", drive_loft(|| rect(len(1.0)), [0.6, 0.8, 0.0]));
}

/// What each row's loft does, one evaluation at `Interval` with no
/// box: the profile is elaborated at the lane, so the arc section and
/// the rotated placement are a few ulps wide. The arc's weight is no
/// point and refuses by name; the rotated copy places in `T`.
#[test]
fn with_no_box_an_arc_section_refuses_and_a_rotated_copy_places() {
    use editor_core::NodeErrorKind;
    use geom_core::Interval;
    use sweep::SkinError;
    let run = |doc: &editor_core::ProfileDoc| {
        editor_core::evaluate::<Interval>(
            doc,
            None,
            &editor_core::CancelToken::new(),
            &editor_core::EvalOptions::default(),
            Tol::witness(),
        )
    };
    let (arc, arc_loft) = loft_doc(circle, [1.0, 0.0, 0.0]);
    let (rotated, rotated_loft) = loft_doc(|| rect(len(1.0)), [0.6, 0.8, 0.0]);
    match run(&arc).node_error(arc_loft).map(|e| &e.kind) {
        Some(NodeErrorKind::Skin(SkinError::Structure(
            geom_core::spline::SplineError::CarrierWeightNotAPoint { .. },
        ))) => {}
        other => panic!("the arc's weight is no point at Interval, got {other:?}"),
    }
    let rotated_run = run(&rotated);
    assert!(
        rotated_run.node_error(rotated_loft).is_none(),
        "the rotated copy places in T: {:?}",
        rotated_run.node_error(rotated_loft)
    );
}
