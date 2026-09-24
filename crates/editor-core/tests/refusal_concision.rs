//! **A refusal the viewer shows fits where it is shown.**
//!
//! The viewer renders a failed node's `NodeError` `Display` verbatim on
//! the feature tree's fault line and the status line, so the sentence a
//! kernel door writes is the sentence the person holding the mouse
//! reads. These rows build the case that sentence must serve and read
//! it back: what failed, the short reason, and the recourse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::eval;
use crate::fixture::{Recorder, frame, len, scl};

use editor_core::{BooleanOp, Datum, LoopProgram, Node, NodeResult, ProfileProgram, TubeWindow};

/// A ring torus (R = 2, r = 0.5, about z) unioned with a block that
/// straddles its tube at +x: a torus face against the block's plane
/// faces, the shape of two dumbbell halves joined at a torus bell.
fn torus_block_union_refusal() -> String {
    let mut r = Recorder::new();
    let spine = r.insert(Node::Datum(Datum::Axis {
        origin: [len(0.0), len(0.0), len(0.0)],
        direction: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    let ring = r.insert(Node::Tube {
        spine,
        u_ref: [scl(1.0), scl(0.0), scl(0.0)],
        major_radius: len(2.0),
        window: TubeWindow::Full,
        minor_radius: len(0.5),
    });
    let plane = r.insert(frame([0.0, 0.0, -0.25], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let block_p = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([(1.75, -0.25), (3.0, -0.25), (3.0, 0.25), (1.75, 0.25)]).unwrap(),
        ],
    }));
    let block = r.insert(Node::Extrude {
        profile: block_p,
        distance: len(0.5),
    });
    let union = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: ring,
        b: block,
        declare: None,
    });
    let ev = eval::<f64>(&r.doc);
    match ev.nodes.get(&union) {
        Some(NodeResult::Failed(e)) => e.to_string(),
        other => panic!("the torus × block union must refuse; got {other:?}"),
    }
}

/// **The worked example**: two solids joined where a torus face meets a
/// plane face, built through the public document doors. The sentence
/// names the pair in the user's terms, keeps the box test's MAY ("may
/// meet"), and ends on the recourse. The length claim is not pinned
/// here but by [`every_rewritten_boolean_refusal_renders_within_the_budget`],
/// over every arm.
#[test]
fn the_torus_plane_union_refusal_names_the_pair_and_ends_on_its_recourse() {
    let msg = torus_block_union_refusal();
    assert!(
        msg.contains(
            "the Boolean op refused: the first operand's torus face may meet the second \
             operand's plane face"
        ),
        "the refusal names the pair by operand, as a may: {msg}"
    );
    assert!(
        msg.ends_with(
            "Recourse: reshape the parts so they meet only where a plane face meets a \
             plane, cylinder or sphere face, or move them so the torus face stays clear \
             of the other solid"
        ),
        "the refusal ends on its recourse: {msg}"
    );
    assert!(
        !msg.contains("coincidence"),
        "a torus × plane pair is not a coincidence refusal, and the wrapper must not \
         point the reader at that recourse: {msg}"
    );
    assert!(!msg.contains("FaceKey("), "no arena key dump: {msg}");
}

/// **The word budget for a refusal the viewer shows: 75 words**, counted
/// on the RENDERED sentence exactly as the feature tree's fault line and
/// the status line draw it (`NodeError`'s `Display`, the "node N failed:
/// the Boolean op refused:" wrapper included), with a representative
/// payload in every placeholder. 75 is what a person reads in one pass
/// at the status line's wrapped width; the worked example is 65.
///
/// Every `topo::BooleanError` arm whose prose the concision pass wrote,
/// and every `topo::PointInSolidError` arm as it arrives through
/// `BooleanError::Containment`, is constructed directly (the fixtures
/// that reach them are expensive or do not exist), rendered, and held
/// to the budget. The old text of the worked example alone rendered 277.
#[test]
fn every_rewritten_boolean_refusal_renders_within_the_budget() {
    const BUDGET: usize = 75;
    let mut problems = Vec::new();
    for (name, text) in rendered_boolean_refusals() {
        let words = text.split_whitespace().count();
        eprintln!("MEASURE {words} {name}: {text}");
        if words > BUDGET {
            problems.push(format!(
                "{name} renders {words} words, over {BUDGET}: {text}"
            ));
        }
        if text.contains("FaceKey(") {
            problems.push(format!("{name} dumps an arena key: {text}"));
        }
        if text.contains("boolean_reduce:") || text.contains("point_in_solid:") {
            problems.push(format!("{name} carries a kernel stage prefix: {text}"));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// Every rewritten arm, rendered the way the viewer renders a failed node.
fn rendered_boolean_refusals() -> Vec<(&'static str, String)> {
    use geom_brep::{MaterialWedge, RadiusEvidence, SurfaceKind};
    use geom_core::{Band, Indeterminate, MarginDiag, Tol};
    use topo::{
        BooleanError, BooleanOp, ContactClass, DeclaredContact, EdgeKey, FaceKey, LoopKey, Operand,
        PlaneRelation, PointInSolidError, SolidKey, VertexKey,
    };

    let band = Band::linear(Tol::witness()).expect("the witness band");
    let diag = Indeterminate {
        margin: MarginDiag::Value(3.0e-10),
        band,
        predicate: Some("side_of_plane"),
    };
    let face = FaceKey::default();
    let edge = EdgeKey::default();
    let declaration = DeclaredContact {
        a: face,
        b: face,
        class: ContactClass::Tangent,
    };
    let boolean: Vec<(&'static str, BooleanError)> = vec![
        (
            "CurvedPairUnsupported",
            BooleanError::CurvedPairUnsupported {
                op: Some(BooleanOp::Subtract),
                operand: Operand::A,
                face,
                kind: SurfaceKind::Nurbs,
                other_face: face,
                other_kind: SurfaceKind::Cylinder,
            },
        ),
        (
            "CurvedBooleanUnsupported",
            BooleanError::CurvedBooleanUnsupported {
                operand: Operand::B,
                face,
                kind: SurfaceKind::Approx,
            },
        ),
        (
            "CurvedPierceUnsupported",
            BooleanError::CurvedPierceUnsupported {
                operand: Operand::A,
                face,
                edge,
                band,
            },
        ),
        (
            "CurvedSectorSideUnsupported",
            BooleanError::CurvedSectorSideUnsupported { band },
        ),
        (
            "CurvedEdgeUnsupported",
            BooleanError::CurvedEdgeUnsupported {
                operand: Operand::B,
                edge,
            },
        ),
        (
            "PointSplitCarrierUnsupported",
            BooleanError::PointSplitCarrierUnsupported {
                operand: Operand::A,
                edge,
            },
        ),
        (
            "ArcLoopContainmentUnsupported",
            BooleanError::ArcLoopContainmentUnsupported {
                operand: Operand::A,
                r#loop: LoopKey::default(),
            },
        ),
        (
            "ScaffoldingOperand",
            BooleanError::ScaffoldingOperand {
                operand: Operand::A,
                edge,
            },
        ),
        (
            "NonMaximalFaces",
            BooleanError::NonMaximalFaces {
                operand: Operand::B,
                edge,
            },
        ),
        (
            "NurbsExtentUnsupported",
            BooleanError::NurbsExtentUnsupported {
                operand: Operand::B,
                face,
            },
        ),
        (
            "FallbackExtentUnsupported",
            BooleanError::FallbackExtentUnsupported {
                operand: Operand::A,
                face,
                what: "the sphere's section circle runs near the plane face's boundary — \
                       whole-circle membership cannot be certified from the enclosures, \
                       and no crossing layer saw an event",
            },
        ),
        (
            "GermFrameUnsupported",
            BooleanError::GermFrameUnsupported {
                a_face: face,
                a_kind: SurfaceKind::Cylinder,
                b_face: face,
                b_kind: SurfaceKind::Sphere,
            },
        ),
        (
            "GermFrameCylinderPinch",
            BooleanError::GermFrameCylinderPinch {
                a_face: face,
                b_face: face,
                evidence: RadiusEvidence::None,
            },
        ),
        (
            "NonFiniteSectorChord",
            BooleanError::NonFiniteSectorChord {
                vertex: VertexKey::default(),
                face,
            },
        ),
        (
            "UnderflowedSectorChord",
            BooleanError::UnderflowedSectorChord {
                vertex: VertexKey::default(),
                face,
            },
        ),
        ("Escalated", BooleanError::Escalated { diag }),
        (
            "UndeclaredCoincidence",
            BooleanError::UndeclaredCoincidence {
                diag,
                pair: [(Operand::A, face), (Operand::B, face)],
                relation: PlaneRelation::SameOpposite,
            },
        ),
        (
            "RimSeamNotDeclarable",
            BooleanError::RimSeamNotDeclarable { declaration },
        ),
        (
            "RimCuspArmUnbuilt",
            BooleanError::RimCuspArmUnbuilt {
                declaration,
                wedge: MaterialWedge::Slit,
            },
        ),
    ];
    let contained: Vec<(&'static str, PointInSolidError)> = vec![
        (
            "Containment(Escalated)",
            PointInSolidError::Escalated { face, diag },
        ),
        ("Containment(RayExhausted)", PointInSolidError::RayExhausted),
        (
            "Containment(ZeroVolumeBody)",
            PointInSolidError::ZeroVolumeBody,
        ),
        (
            "Containment(CorruptFace)",
            PointInSolidError::CorruptFace { face },
        ),
        (
            "Containment(KindUnsupported)",
            PointInSolidError::KindUnsupported {
                face,
                kind: SurfaceKind::Nurbs,
            },
        ),
        (
            "Containment(VolumeUncertified)",
            PointInSolidError::VolumeUncertified,
        ),
        (
            "Containment(PartialSphereFace)",
            PointInSolidError::PartialSphereFace { face },
        ),
        (
            "Containment(PartialConeFace)",
            PointInSolidError::PartialConeFace { face },
        ),
        (
            "Containment(PartialTorusFace)",
            PointInSolidError::PartialTorusFace { face },
        ),
        (
            "Containment(NoSuchSolid)",
            PointInSolidError::NoSuchSolid {
                solid: SolidKey::default(),
            },
        ),
        (
            "Containment(SurfaceSharedOutsideSolid)",
            PointInSolidError::SurfaceSharedOutsideSolid { face, other: face },
        ),
    ];
    boolean
        .into_iter()
        .chain(
            contained
                .into_iter()
                .map(|(name, e)| (name, BooleanError::Containment(e))),
        )
        .map(|(name, e)| (name, as_the_viewer_shows_it(e)))
        .collect()
}

/// A `BooleanError` as the feature tree's fault line draws it: the
/// failed node's `NodeError` `Display`, wrapper included.
fn as_the_viewer_shows_it(e: topo::BooleanError) -> String {
    editor_core::NodeError {
        node: editor_core::RecipeNodeId(5),
        kind: editor_core::NodeErrorKind::Boolean(e),
        escalations: std::sync::Arc::new(Vec::new()),
    }
    .to_string()
}
