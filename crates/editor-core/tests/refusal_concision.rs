//! **A refusal the viewer shows fits where it is shown.**
//!
//! The viewer renders a failed node's `NodeError` `Display` verbatim on
//! the feature tree's fault line and the status line, so the sentence a
//! kernel door writes is the sentence the person holding the mouse
//! reads. These rows build the case that sentence must serve and read
//! it back: what failed, the short reason, and the recourse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::eval;
use crate::fixture::{Recorder, ang, axis_in_plane, frame, len};

use editor_core::{BooleanOp, LoopProgram, Node, NodeResult, ProfileProgram};

/// A cone frustum (a full revolve about `y`) unioned with a block that
/// straddles its slanted wall: a cone face against the block's plane
/// faces. The cone is the curved kind the operand gate still has no
/// arm for, so this is the pair refusal the sentence below is about.
fn cone_block_union_refusal() -> String {
    let mut r = Recorder::new();
    let plane = r.insert(frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let cone_p = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (0.4, 1.0), (0.0, 1.0)]).unwrap(),
        ],
        ids: Vec::new(),
    }));
    let axis = r.insert(axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    let cone = r.insert(Node::Revolve {
        profile: cone_p,
        axis,
        angle: ang(std::f64::consts::TAU),
    });
    let block_plane = r.insert(frame([0.0, 0.0, -0.25], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let block_p = r.insert(Node::Profile(ProfileProgram {
        plane: block_plane,
        loops: vec![
            LoopProgram::polygon([(0.5, 0.4), (1.5, 0.4), (1.5, 0.6), (0.5, 0.6)]).unwrap(),
        ],
        ids: Vec::new(),
    }));
    let block = r.insert(Node::Extrude {
        profile: block_p,
        distance: len(0.5),
    });
    let union = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: cone,
        b: block,
        declare: Vec::new(),
    });
    let ev = eval::<f64>(&r.doc);
    match ev.nodes.get(&union) {
        Some(NodeResult::Failed(e)) => e.to_string(),
        other => panic!("the cone × block union must refuse; got {other:?}"),
    }
}

/// **The worked example**: two solids joined where a cone face meets a
/// plane face, built through the public document doors. The sentence
/// names the pair in the user's terms, keeps the box test's MAY ("may
/// meet"), and ends on the recourse. The length claim is not pinned
/// here but by [`every_rewritten_boolean_refusal_renders_within_the_budget`],
/// over every arm.
#[test]
fn the_cone_plane_union_refusal_names_the_pair_and_ends_on_its_recourse() {
    let msg = cone_block_union_refusal();
    assert!(
        msg.contains(
            "the Boolean op refused: the first operand's cone face may meet the second \
             operand's plane face"
        ),
        "the refusal names the pair by operand, as a may: {msg}"
    );
    assert!(
        msg.ends_with(
            "Recourse: reshape the parts so they meet only where a plane face meets a \
             plane, cylinder or sphere face, or move them so the cone face stays clear \
             of the other solid"
        ),
        "the refusal ends on its recourse: {msg}"
    );
    assert!(
        !msg.contains("coincidence"),
        "a cone × plane pair is not a coincidence refusal, and the wrapper must not \
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
        if NO_TOLERANCE_PASSES.contains(&name) && text.contains("tolerance below") {
            problems.push(format!(
                "{name} offers a tolerance no smaller one honours: {text}"
            ));
        }
        if A_TOLERANCE_PASSES.contains(&name) && !text.contains("tolerance below") {
            problems.push(format!(
                "{name} drops the tolerance a smaller one honours: {text}"
            ));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// The rows whose refusal no smaller tolerance passes, so a sentence
/// that offers one spends words on a false offer: two spheres that
/// definitely cross (`SpheresMeet`'s negative verdict) cross at every
/// tolerance, and the Boolean cannot yet join them.
const NO_TOLERANCE_PASSES: &[&str] = &["SpheresMeet"];

/// The rows whose refusal a smaller tolerance passes, so the sentence
/// owes the value: a nesting clearance within the zero band is decided
/// nested below it (executed: `sweep`'s `offer_rows`,
/// `nested_in_the_zero_band`).
const A_TOLERANCE_PASSES: &[&str] = &["SpheresMeet (touching, nested within the zero band)"];

/// Every rewritten arm, rendered the way the viewer renders a failed node.
fn rendered_boolean_refusals() -> Vec<(&'static str, String)> {
    use geom::SurfaceKind;
    use geom_brep::{MaterialWedge, RadiusEvidence};
    use geom_core::{Band, Indeterminate, MarginDiag, Tol};
    use topo::{
        BooleanError, BooleanOp, ContactClass, DeclaredContact, EdgeKey, FaceKey, LoopKey, Operand,
        PlaneRelation, PointInSolidError, SolidKey, VertexKey,
    };

    let band = Band::linear(Tol::witness()).expect("the witness band");
    let diag = Indeterminate {
        margin: MarginDiag::value(3.0e-10),
        band,
        predicate: Some("side_of_plane"),
        terminal_sliver: false,
    };
    // An in-band margin with every digit a real one carries: the
    // longest payload an escalation quotes.
    let in_band = Indeterminate {
        margin: MarginDiag::value(5.500000010982831e-9),
        ..diag
    };
    // An in-band enclosure, which quotes two numbers where a point
    // quotes one: the longest payload the interval lane reports.
    let enclosed = Indeterminate {
        margin: MarginDiag::enclosure(2.000000000000001e-9, 5.000000000000001e-9),
        ..diag
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
                site: topo::PairRefusalSite::RevertRoster,
                operand: Operand::A,
                face,
                kind: SurfaceKind::Nurbs,
                other_face: face,
                other_kind: SurfaceKind::Cylinder,
            },
        ),
        (
            "CurvedPairUnsupported (interior-loop guard)",
            BooleanError::CurvedPairUnsupported {
                op: Some(BooleanOp::Intersect),
                site: topo::PairRefusalSite::InteriorLoopGuard,
                operand: Operand::A,
                face,
                kind: SurfaceKind::Torus,
                other_face: face,
                other_kind: SurfaceKind::Plane,
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
            "DegenerateTorus",
            BooleanError::DegenerateTorus {
                operand: Operand::B,
                face,
                convention: topo::TorusConvention::Ring,
                verdict: geom_brep::recourse::Refused::Zero(geom_brep::recourse::Classified {
                    margin: MarginDiag::value(5.0e-10),
                    band,
                }),
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
            BooleanError::CurvedSectorSideUnsupported {
                verdict: geom_brep::recourse::Refused::Negative {
                    margin: MarginDiag::value(-3.0e-4),
                },
            },
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
                cause: topo::Uncrossable {
                    r#loop: LoopKey::default(),
                    edge,
                    carrier: topo::UncrossableCarrier::Spiric,
                },
            },
        ),
        (
            "ScaffoldingOperand",
            BooleanError::ScaffoldingOperand {
                operand: Operand::A,
                errors: vec![topo::ValidationError::ScaffoldingStrutVertex {
                    vertex: topo::VertexKey::default(),
                }],
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
            "CoplanarNeighbours",
            BooleanError::CoplanarNeighbours {
                operand: Operand::B,
                faces: [face, face],
                offset: topo::NeighbourOffset::Undecided(diag),
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
        (
            "Escalated",
            BooleanError::Escalated {
                decision: topo::BooleanDecision::Coincidence(
                    topo::Coincide::VertexOnFace,
                    topo::DeclarationRead::Moot,
                ),
                diag,
            },
        ),
        // The longest escalations in band, with a point payload and with
        // an enclosure (every decision is held under this budget by
        // `topo`'s `every_escalation_renders_within_the_viewers_word_budget`).
        (
            "Escalated (PierceCurvature, in band)",
            BooleanError::Escalated {
                decision: topo::BooleanDecision::PierceCurvature,
                diag: in_band,
            },
        ),
        (
            "Escalated (PierceCurvature, in-band enclosure)",
            BooleanError::Escalated {
                decision: topo::BooleanDecision::PierceCurvature,
                diag: enclosed,
            },
        ),
        (
            "Escalated (Neighbours(Parallel), in-band enclosure)",
            BooleanError::Escalated {
                decision: topo::BooleanDecision::Neighbours(topo::PlaneRung::Parallel),
                diag: enclosed,
            },
        ),
        (
            "Escalated (LeverArm(Seam), in-band enclosure)",
            BooleanError::Escalated {
                decision: topo::BooleanDecision::LeverArm(topo::LeverArm::Seam),
                diag: enclosed,
            },
        ),
        (
            "Escalated (TangentSide, declared, in-band enclosure)",
            BooleanError::Escalated {
                decision: topo::BooleanDecision::Coincidence(
                    topo::Coincide::TangentSide,
                    topo::DeclarationRead::Spent(topo::BooleanCoincidence::TANGENT),
                ),
                diag: enclosed,
            },
        ),
        (
            "Escalated (Sphere(Nested), in-band enclosure)",
            BooleanError::Escalated {
                decision: topo::BooleanDecision::Sphere(topo::SphereQuestion::Nested),
                diag: enclosed,
            },
        ),
        (
            "SpheresMeet",
            BooleanError::SpheresMeet {
                operand: Operand::A,
                face,
                verdict: geom_brep::recourse::Refused::Negative {
                    margin: MarginDiag::value(-0.5),
                },
            },
        ),
        (
            "SpheresMeet (touching, nested within the zero band)",
            BooleanError::SpheresMeet {
                operand: Operand::A,
                face,
                verdict: geom_brep::recourse::Refused::Zero(geom_brep::recourse::Classified {
                    margin: MarginDiag::value(0.5 * band.zero()),
                    band,
                }),
            },
        ),
        (
            "Escalated (LeverArm(Seam), in band)",
            BooleanError::Escalated {
                decision: topo::BooleanDecision::LeverArm(topo::LeverArm::Seam),
                diag: in_band,
            },
        ),
        (
            "Escalated (EdgeOnPlane, no declaration read, in band)",
            BooleanError::Escalated {
                decision: topo::BooleanDecision::Coincidence(
                    topo::Coincide::EdgeOnPlane,
                    topo::DeclarationRead::Moot,
                ),
                diag: in_band,
            },
        ),
        // The longest escalation of every decision in band, and the
        // longest coincidence (rendered over `every_decision` in PR
        // 3513's second fix pass: F7's in-band bend rendered 77).
        (
            "Escalated (Neighbours(Parallel), in band)",
            BooleanError::Escalated {
                decision: topo::BooleanDecision::Neighbours(topo::PlaneRung::Parallel),
                diag: in_band,
            },
        ),
        (
            "Escalated (TangentSide, declared, in band)",
            BooleanError::Escalated {
                decision: topo::BooleanDecision::Coincidence(
                    topo::Coincide::TangentSide,
                    topo::DeclarationRead::Spent(topo::BooleanCoincidence::TANGENT),
                ),
                diag: in_band,
            },
        ),
        (
            "UndeclaredCoincidence",
            BooleanError::UndeclaredCoincidence {
                diag,
                pair: [(Operand::A, face), (Operand::B, face)],
                relation: PlaneRelation::SameOpposite,
            },
        ),
        (
            "ShellWitnessExhausted",
            BooleanError::ShellWitnessExhausted {
                operand: Operand::B,
                shell: topo::ShellKey::default(),
                on_boundary: 26,
                in_band: 0,
                first_in_band: None,
            },
        ),
        (
            "ShellWitnessExhausted (in band)",
            BooleanError::ShellWitnessExhausted {
                operand: Operand::B,
                shell: topo::ShellKey::default(),
                on_boundary: 20,
                in_band: 6,
                first_in_band: Some(topo::PointInSolidError::RayExhausted),
            },
        ),
        (
            "CoincidentShell (unpaired)",
            BooleanError::CoincidentShell {
                operand: Operand::A,
                shell: topo::ShellKey::default(),
                orientation: topo::ShellOrientation::Unpaired { face },
            },
        ),
        (
            "CoincidentShell (mixed)",
            BooleanError::CoincidentShell {
                operand: Operand::B,
                shell: topo::ShellKey::default(),
                orientation: topo::ShellOrientation::Mixed,
            },
        ),
        (
            "CoincidentShell (not covered back)",
            BooleanError::CoincidentShell {
                operand: Operand::A,
                shell: topo::ShellKey::default(),
                orientation: topo::ShellOrientation::Same,
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
            "Containment(EdgeCarrierUnsupported)",
            PointInSolidError::EdgeCarrierUnsupported {
                face,
                cause: topo::Uncrossable {
                    r#loop: Default::default(),
                    edge: Default::default(),
                    carrier: topo::UncrossableCarrier::Spiric,
                },
            },
        ),
        (
            "Containment(WallOutlineUnsupported)",
            PointInSolidError::WallOutlineUnsupported { face },
        ),
        (
            "Containment(NoSuchSolid)",
            PointInSolidError::NoSuchSolid {
                solid: SolidKey::default(),
            },
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
