//! **[`BooleanError::refusal`] states each variant's class**: one variant
//! per class, and each arm that delegates to its payload or splits on it,
//! on both sides of the split. Red where an arm is re-classed by hand, a
//! delegate stops reading its payload, or a split reads one side for both.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom_brep::ladder::{Aborts, Refusal};
use geom_core::{Band, Indeterminate, MarginDiag};

use super::solid_contain::PointInSolidError;
use super::{BooleanError, Coincide, DeclarationRead, EdgeInBand, LineUnread, Operand, SectorRead};
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopKey, ShellKey, VertexKey};

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

fn diag() -> Indeterminate {
    Indeterminate {
        margin: MarginDiag::value(5e-9),
        band: band(),
        predicate: Some("t"),
        terminal_sliver: false,
    }
}

#[test]
fn each_class_and_each_delegate_reads_as_its_doc_states() {
    let (op, face, edge, vertex) = (
        Operand::A,
        FaceKey::default(),
        EdgeKey::default(),
        VertexKey::default(),
    );
    let fixed: [(&str, BooleanError, Result<Refusal, Aborts>); 19] = [
        (
            "an escalation",
            BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag()),
            Ok(Refusal::Margin(diag())),
        ),
        (
            "an operand gate in band",
            BooleanError::CoplanarNeighbours {
                operand: op,
                faces: [face, face],
                offset: diag(),
            },
            Ok(Refusal::Margin(diag())),
        ),
        (
            "an apex in band",
            BooleanError::CrossingAtConeApex {
                operand: op,
                face,
                edge,
                band: band(),
            },
            Ok(Refusal::InBand),
        ),
        (
            "an arm not built",
            BooleanError::TangencyUnsupported,
            Ok(Refusal::Limit),
        ),
        (
            "a face kind with no arm",
            BooleanError::CurvedBooleanUnsupported {
                operand: op,
                face,
                kind: geom::SurfaceKind::Torus,
            },
            Ok(Refusal::Limit),
        ),
        (
            "a root finder's frontier",
            BooleanError::SplineCrossingUnsettled {
                operand: op,
                edge,
                face,
                face_of: Operand::B,
                why: geom_brep::roots::Unsettled::Floor,
            },
            Ok(Refusal::Unsettled),
        ),
        (
            "a kernel fault",
            BooleanError::ClassificationInvariant { what: "t" },
            Err(Aborts),
        ),
        (
            "an invalid input",
            BooleanError::InvalidInput {
                operand: op,
                what: "t",
            },
            Err(Aborts),
        ),
        (
            "no first in-band witness",
            BooleanError::ShellWitnessExhausted {
                operand: op,
                shell: ShellKey::default(),
                on_boundary: 0,
                in_band: 0,
                first_in_band: None,
            },
            Ok(Refusal::InBand),
        ),
        (
            "a join margin",
            BooleanError::JoinUndecided(super::edge_join::JoinUndecided {
                vertex,
                reading: super::edge_join::JoinReading::Regularity(diag()),
            }),
            Ok(Refusal::Margin(diag())),
        ),
        (
            "a bound along the line",
            line(LineUnread::Along),
            Ok(Refusal::Limit),
        ),
        (
            "a line's faces unplaced",
            line(LineUnread::Unplaced),
            Ok(Refusal::InBand),
        ),
        (
            "a line's faces giving no reference",
            line(LineUnread::NoReference),
            Ok(Refusal::Limit),
        ),
        ("a vertex read twice", twice(false), Ok(Refusal::Limit)),
        (
            "a vertex read twice, an edge in band",
            twice(true),
            Ok(Refusal::InBand),
        ),
        (
            "a fitted edge undescribed",
            undescribed(true),
            Ok(Refusal::Limit),
        ),
        ("an edge undescribed", undescribed(false), Err(Aborts)),
        (
            "an edge in a face, a defect",
            in_face(super::slit::EdgeInFaceGap::Crossed),
            Err(Aborts),
        ),
        (
            "an edge in a face on a torus outside the ring convention, an invalid input",
            in_face(super::slit::EdgeInFaceGap::DegenerateTorus(horn())),
            Err(Aborts),
        ),
    ];
    for (label, error, want) in fixed {
        assert_eq!(error.refusal(), want, "{label}");
    }
    let unsettled = |why| BooleanError::SplineCrossingUnsettled {
        operand: op,
        edge,
        face,
        face_of: Operand::B,
        why,
    };
    let more: [(&str, BooleanError, Result<Refusal, Aborts>); 10] = [
        (
            "a result no model represents, non-manifold",
            BooleanError::NonManifoldResult {
                a_vertex: vertex,
                b_vertices: [vertex; 2],
            },
            Err(Aborts),
        ),
        (
            "a result no model represents, unbounded",
            BooleanError::UnrepresentableResult,
            Err(Aborts),
        ),
        (
            "a torus no model represents",
            BooleanError::DegenerateTorus {
                operand: op,
                face,
                convention: geom_brep::TorusConvention::Tube,
                verdict: geom_brep::recourse::Refused::Negative {
                    margin: MarginDiag::value(-1.0),
                },
            },
            Err(Aborts),
        ),
        (
            "a chord with no finite length, past the format's scale",
            BooleanError::NonFiniteSectorChord { vertex, face },
            Ok(Refusal::Limit),
        ),
        (
            "a chart image that refused, not yet split",
            BooleanError::JoinUndecided(super::edge_join::JoinUndecided {
                vertex,
                reading: super::edge_join::JoinReading::ChartClass(
                    geom_brep::IsoFamilyRefusal::Image(
                        geom_brep::PcurveCertifyError::UnsupportedChart {
                            chart: geom::SurfaceKind::Torus,
                        },
                    ),
                ),
            }),
            Err(Aborts),
        ),
        (
            "a line's faces read against each other within the tolerance",
            line(LineUnread::Contradicted),
            Ok(Refusal::InBand),
        ),
        (
            "a meeting in band of a cone's apex",
            unsettled(geom_brep::roots::Unsettled::AtApex),
            Ok(Refusal::InBand),
        ),
        (
            "a meeting at a spline's crease, not yet split",
            unsettled(geom_brep::roots::Unsettled::Crease),
            Err(Aborts),
        ),
        (
            "a line along a cone's generator",
            unsettled(geom_brep::roots::Unsettled::Degenerate),
            Ok(Refusal::Limit),
        ),
        (
            "a walk past its budget",
            unsettled(geom_brep::roots::Unsettled::Exhausted),
            Ok(Refusal::Unsettled),
        ),
    ];
    for (label, error, want) in more {
        assert_eq!(error.refusal(), want, "{label}");
    }
    let faces = [face, face];
    assert_eq!(
        BooleanError::TouchesItself {
            operand: op,
            record: 0,
            faces,
        }
        .refusal(),
        Ok(Refusal::Limit),
        "an area self-touch inside one shell, not read yet"
    );
    assert_eq!(
        BooleanError::ToolSeatCrossed { record: 0, faces }.refusal(),
        Ok(Refusal::Limit),
        "a cut across its tool's seat, not built yet"
    );
    let crease = |reading| BooleanError::NormalAtSplineCrease {
        operand: op,
        face,
        band: band(),
        reading,
    };
    assert_eq!(
        crease(Err(diag())).refusal(),
        Ok(Refusal::Margin(diag())),
        "a crease in band is ranked by its margin"
    );
    assert_eq!(
        crease(Ok(MarginDiag::value(0.0))).refusal(),
        Ok(Refusal::Limit),
        "a crease decided zero is a limit"
    );
    assert_eq!(
        in_face(super::slit::EdgeInFaceGap::Tangent).refusal(),
        Ok(Refusal::Limit),
        "an edge in a face, a parting not built"
    );
    for abort in crate::face_normal::NORMAL_INVALID_INPUTS {
        let torus = abort.torus().unwrap();
        let geom_brep::TorusVerdict::Refused(verdict) = torus.verdict else {
            panic!("{torus:?} is decided")
        };
        let text = in_face(super::slit::EdgeInFaceGap::DegenerateTorus(torus))
            .render(geom_brep::recourse::Reading::Build);
        let lever = torus.convention.sized().recourse(
            verdict.arm(),
            geom_brep::recourse::ReadAt::from(geom_brep::recourse::Reading::Build),
        );
        assert!(
            text.contains(torus.convention.refused("a torus face's", verdict).as_str())
                && text.ends_with(lever.as_str())
                && !text.contains(geom_core::KERNEL_DEFECT_ENDING)
                && !text.contains(geom_core::NOT_YET_ENDING),
            "an edge in a face on a torus outside the ring convention ends on its half's \
             lever: {text}"
        );
    }
    // The delegates read their payload's own standing.
    let solid = PointInSolidError::ZeroVolumeBody;
    assert_eq!(
        BooleanError::Containment(solid.clone()).refusal(),
        solid.refusal(),
        "the solid door's refusal"
    );
    assert_eq!(
        BooleanError::ShellWitnessExhausted {
            operand: op,
            shell: ShellKey::default(),
            on_boundary: 0,
            in_band: 1,
            first_in_band: Some(solid.clone()),
        }
        .refusal(),
        solid.refusal(),
        "a first in-band witness"
    );
    let contain = super::ContainError::EmptyLoop(LoopKey::default());
    assert_eq!(
        BooleanError::PointInFaceRefused {
            operand: op,
            face,
            refusal: contain.clone(),
        }
        .refusal(),
        contain.refusal(),
        "the face door's refusal"
    );
}

fn line(why: LineUnread) -> BooleanError {
    BooleanError::LineMaterialUnread {
        a_vertex: VertexKey::default(),
        b_vertex: VertexKey::default(),
        why,
    }
}

fn twice(in_band: bool) -> BooleanError {
    BooleanError::VertexReadTwice {
        operand: Operand::A,
        vertex: VertexKey::default(),
        reads: [SectorRead::Pair(VertexKey::default()); 2],
        in_band: in_band.then_some(EdgeInBand {
            edge: HalfEdgeKey::default(),
            face: None,
            predicate: None,
        }),
    }
}

fn undescribed(fitted: bool) -> BooleanError {
    BooleanError::EdgeUndescribed {
        edge: EdgeKey::default(),
        failure: crate::merge_faces::EdgeDescribeFailure::NotWalkable,
        cause: None,
        fitted,
    }
}

fn in_face(gap: super::slit::EdgeInFaceGap) -> BooleanError {
    BooleanError::EdgeInFace {
        edge: EdgeKey::default(),
        face: FaceKey::default(),
        gap,
    }
}

/// **A face's normal that aborted a Boolean reading is answered by its
/// row** ([`BooleanError::of_aborted_normal`], which takes the abort
/// alone): a torus outside the ring convention, an invalid input, is the
/// Boolean's torus refusal on the face it names, on either half; the
/// point off the surface, a kernel fault, is the invariant the caller
/// names.
#[test]
fn an_aborted_normal_is_answered_by_its_row() {
    use crate::face_normal::{NORMAL_INVALID_INPUTS, NormalAbort};
    let face = FaceKey::default();
    for abort in NORMAL_INVALID_INPUTS {
        let NormalAbort::InvalidInput {
            convention,
            verdict,
        } = abort
        else {
            panic!("{abort:?} is an invalid input")
        };
        let input = BooleanError::of_aborted_normal(abort, (Operand::B, face), "off");
        assert!(
            matches!(
                input,
                BooleanError::DegenerateTorus {
                    operand: Operand::B,
                    convention: c,
                    verdict: v,
                    ..
                } if c == convention && v == verdict
            ),
            "an invalid input: {input:?}"
        );
        let text = input.render(geom_brep::recourse::Reading::Build);
        let lever = convention.sized().recourse(
            verdict.arm(),
            geom_brep::recourse::ReadAt::from(geom_brep::recourse::Reading::Build),
        );
        assert!(
            text.ends_with(lever.as_str()),
            "{convention:?} ends on its own lever: {text}"
        );
    }
    let fault =
        BooleanError::of_aborted_normal(NormalAbort::KernelFault, (Operand::A, face), "off");
    assert!(
        matches!(fault, BooleanError::ClassificationInvariant { what: "off" }),
        "a kernel fault: {fault:?}"
    );
}

/// A ring torus refused outside its convention, decided.
fn horn() -> geom_brep::TorusRefusal {
    geom_brep::TorusRefusal {
        convention: geom_brep::TorusConvention::Ring,
        verdict: geom_brep::TorusVerdict::Refused(geom_brep::recourse::Refused::Negative {
            margin: MarginDiag::INVALID,
        }),
    }
}

/// **A line's wedge unread on a torus outside the ring convention renders
/// through the convention's home** ([`BooleanError::LineWedges`]): the
/// Boolean reads the line on a result whose operands a dual never put
/// through tier 3, so its sentence is the half's own clause and lever,
/// on either half, and never the kernel's.
#[test]
fn a_line_wedge_on_an_invalid_torus_ends_on_its_half() {
    use crate::face_normal::{NORMAL_INVALID_INPUTS, NormalAbort};
    let at = geom_brep::recourse::ReadAt::from(geom_brep::recourse::Reading::Build);
    for abort in NORMAL_INVALID_INPUTS {
        let NormalAbort::InvalidInput {
            convention,
            verdict,
        } = abort
        else {
            panic!("{abort:?} is an invalid input")
        };
        let text = BooleanError::LineWedges {
            fault: super::LineWedgeFault::Unread {
                edge: EdgeKey::default(),
                why: crate::link::TouchUnread::NormalFault(abort),
            },
            from: "(0, 0, 0)".to_owned(),
            to: "(1, 0, 0)".to_owned(),
        }
        .render(geom_brep::recourse::Reading::Build);
        assert!(
            text.contains(convention.refused("its", verdict).as_str())
                && text.ends_with(convention.sized().recourse(verdict.arm(), at).as_str())
                && !text.contains(geom_core::KERNEL_DEFECT_ENDING),
            "{convention:?}: {text}"
        );
    }
}
