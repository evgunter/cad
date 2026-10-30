//! **Every [`ValidationError`] shape the viewer can draw, on a
//! representative payload**: the samples two rows read.
//!
//! - This crate's `errors_display_without_panicking` indexes them by
//!   the enum's compiler-derived companion, so an arm added without a
//!   sample reds by name, and it asserts `nested_coverage_gaps` is
//!   empty — the same statement one level in, for every enum an arm
//!   renders whole.
//! - `editor-core`'s `refusal_concision_at_rest` renders each one the
//!   way the viewer's at-rest and product badges draw a finding and
//!   holds it to the refusal standard — which is why the list lives
//!   behind the `test_support` door rather than in `crate::fixtures`,
//!   which no other crate can name ([`crate::test_support_impl`] states
//!   the routing rule).
//!
//! **Every variant of every nested enum, not one sample per arm.** An
//! arm that renders a nested refusal (`{cause}`, `{error}`, `{source}`)
//! is as long as its longest nested sentence, so each such arm is
//! sampled once per variant of the enum it carries:
//! [`CensusUnsupportedCause`] and, under it, every [`ChartRegionError`],
//! [`ContactRefusal`] and [`ContainError`]; every `CertifyError`,
//! [`PcurveMintError`], [`MassPropsError`], `OffsetFitError` and
//! [`BandError`]; every margin shape an [`Indeterminate`] renders;
//! every [`CensusContact`], [`StaleDeclaration`], [`RingContact`] and
//! [`RingPairContact`];
//! and every [`Undecided`] reason, which is every `what` the
//! cross-solid backstop can raise. One level further in, the enums
//! `PcurveMintError::Certify`, `MassPropsError::Face` and
//! `CertifyError::PlaneNurbs` carry are sampled whole too, as are
//! `OffsetFitError::PatchBound`'s and `OffsetFitError::Band`'s; the
//! other four `OffsetFitError` wrappers carry one value each.
//!
//! Where a raise site fills a field with prose — a `what` or a `detail`
//! — the sample carries the prose a real run renders, and a
//! field with several real values is sampled at each one this module
//! names. Arena keys are the default (null) keys.
#![allow(clippy::expect_used)] // one fixed band, well-formed by construction

use geom_brep::certify::{CertCheck, CertifyError};
use geom_brep::edge_nurbs::PlaneNurbsRefusal;
use geom_brep::offset_fit::{OFFSET_FIT_SAMPLE_CAP, OFFSET_FIT_WORK, OffsetFitError, OffsetLimb};
use geom_brep::pcurve_cache::{FittedMagnitude, PcurveCertifyError, PcurveCheck};
use geom_brep::props::PropsError;
use geom_brep::recourse::{Classified, Refused};
use geom_core::k_stats::Magnitude;
use geom_core::{Band, BandError, BandField, Indeterminate, MarginDiag};
use strum::IntoEnumIterator as _;

use crate::boolean::ContainError;
use crate::census::Undecided;
use crate::chart_region::ChartRegionError;
use crate::contact::{ContactFinding, ContactRefusal, ContactVerdict, DeclaredContact, GlueClass};
use crate::entity::{
    EdgeKey, EntityId, FaceKey, GeomRef, HalfEdgeKey, LoopKey, ShellKey, SolidKey, VertexKey,
};
use crate::geometry::{CurveKey, PointKey};
use crate::pcurves::PcurveMintError;
use crate::props::MassPropsError;
use crate::validate::{
    CensusContact, CensusSubject, CensusUnsupportedCause, RingContact, RingPairContact,
    StaleDeclaration, ValidationError, WedgeCheck,
};

fn band() -> Band {
    Band::new(1e-9, 1e-8).expect("a well-formed band")
}

/// The three shapes an [`Indeterminate`] renders: a margin value, an
/// enclosure, and the invalid margin a definite-relation site carries.
fn diags() -> [Indeterminate; 3] {
    let with = |margin| Indeterminate {
        margin,
        band: band(),
        predicate: Some("side_of_plane"),
        terminal_sliver: false,
    };
    [
        with(MarginDiag::value(5e-9)),
        with(MarginDiag::enclosure(-2e-9, 4e-9)),
        with(MarginDiag::INVALID),
    ]
}

fn diag() -> Indeterminate {
    diags()[0]
}

/// A band-decided zero verdict on `margin`, as `f64` classification
/// reports it.
fn zero_verdict(margin: f64) -> Refused {
    Refused::Zero(Classified {
        margin: MarginDiag::value(margin),
        band: band(),
    })
}

/// A positive-passing gate's rejection of a reading past the band on
/// the refused side.
fn rejected_negative(predicate: &'static str) -> Indeterminate {
    geom_core::k_stats::decide_positive(predicate, geom_core::Margin::of(-1e-3), band())
        .expect_err("a definitely negative reading is the gate's rejection")
}

/// A sign-certain negative verdict on `margin`.
fn negative_verdict(margin: f64) -> Refused {
    Refused::Negative {
        margin: MarginDiag::value(margin),
    }
}

/// The margin every `ContactRefusal::Contradicted` raise site carries:
/// invalid, naming the contact predicate that decided.
fn contradiction_margin(predicate: &'static str) -> Indeterminate {
    Indeterminate {
        margin: MarginDiag::INVALID,
        band: band(),
        predicate: Some(predicate),
        terminal_sliver: false,
    }
}

fn band_errors() -> Vec<BandError> {
    vec![
        BandError::InvalidValue {
            field: BandField::Escalate,
            value: 0.0,
        },
        BandError::InvalidLeverArm { value: f64::NAN },
        BandError::Empty {
            zero: 1.0,
            escalate: 0.5,
        },
    ]
}

fn chart_region_errors() -> Vec<ChartRegionError> {
    let face = FaceKey::default();
    let mut v = vec![
        ChartRegionError::NonPlanarTrim {
            face,
            half_edge: HalfEdgeKey::default(),
            what: "no closed-form chart image for this carrier kind",
        },
        ChartRegionError::MissingCache {
            half_edge: HalfEdgeKey::default(),
        },
        ChartRegionError::ArmUnbounded { chart: "sphere" },
        ChartRegionError::SeamBranch,
        ChartRegionError::PeriodFold,
        ChartRegionError::CarrierTilt,
        ChartRegionError::TouchingBoundary,
        ChartRegionError::DegenerateLoop {
            face,
            r#loop: LoopKey::default(),
        },
        ChartRegionError::RayExhausted,
        ChartRegionError::WitnessSegmentCapExceeded {
            segments: crate::chart_region::WITNESS_SEGMENT_CAP + 1,
        },
        ChartRegionError::WitnessCellCapExceeded {
            segments: crate::chart_region::WITNESS_SEGMENT_CAP,
            cells: crate::chart_region::WITNESS_CELL_CAP,
        },
        ChartRegionError::EdgeScale,
        ChartRegionError::Corrupt,
    ];
    v.push(ChartRegionError::ChartDivergence {
        detail: "two descriptions that do not read bit-identical — no one chart holds both",
    });
    v.extend(diags().into_iter().map(ChartRegionError::Escalated));
    v
}

fn contact_refusals() -> Vec<ContactRefusal> {
    let mut v = vec![
        ContactRefusal::Contradicted {
            diag: contradiction_margin("contact_rest_senses_opposed"),
        },
        ContactRefusal::Contradicted {
            diag: contradiction_margin("carrier_cyl_radius"),
        },
        ContactRefusal::Escalated { diag: diag() },
        ContactRefusal::Undeclared { diag: diag() },
    ];
    // The longest `what`s the raise sites write.
    v.extend(
        [
            "a declared face's surface kind is outside the Rest ladder's inventory \
             (plane, sphere, cylinder, torus)",
            "the (carrier kind, surface-kind pair) triple is outside the jet \
             certificate's span-bound lane (the order-k boundary)",
        ]
        .map(|what| ContactRefusal::NotCertifiable { what }),
    );
    v
}

fn uncrossable() -> crate::splitting::Uncrossable {
    crate::splitting::Uncrossable {
        r#loop: LoopKey::default(),
        edge: EdgeKey::default(),
        carrier: crate::splitting::UncrossableCarrier::Spiric,
    }
}

/// Every refusal the solid door can hand the face door's curved reads,
/// carried as [`ContainError::Curved`]; `Loop` once per loop refusal, and
/// `OffPlane` once per cause.
fn point_in_solid_errors() -> Vec<crate::boolean::PointInSolidError> {
    use crate::boolean::PointInSolidError as S;
    use crate::splitting::{OffPlane, OffPlaneCause, PointInLoopError as L};
    let face = FaceKey::default();
    let r#loop = LoopKey::default();
    vec![
        S::Escalated { face, diag: diag() },
        S::RayExhausted,
        S::ZeroVolumeBody,
        S::Loop(L::Escalated {
            r#loop,
            decision: crate::splitting::LoopDecision::Boundary,
            escalation: crate::splitting::Escalation::Margin,
            diag: diag(),
        }),
        S::Loop(L::RayExhausted { r#loop }),
        S::Loop(L::CorruptLoop { r#loop }),
        S::Loop(L::Uncrossable(uncrossable())),
        S::Loop(L::OffPlane(OffPlane {
            r#loop,
            cause: OffPlaneCause::Loop {
                edge: crate::entity::EdgeKey::default(),
            },
        })),
        S::Loop(L::OffPlane(OffPlane {
            r#loop,
            cause: OffPlaneCause::Query,
        })),
        S::Loop(L::OffPlane(OffPlane {
            r#loop,
            cause: OffPlaneCause::NormalNotUnit,
        })),
        S::CorruptFace { face },
        S::KindUnsupported {
            face,
            kind: geom::SurfaceKind::Nurbs,
        },
        S::VolumeUncertified,
        S::InsideOut,
        S::PartialSphereFace { face },
        S::PartialConeFace { face },
        S::PartialTorusFace { face },
        S::EdgeCarrierUnsupported {
            face,
            cause: uncrossable(),
        },
        S::WallOutlineUnsupported { face },
        S::RootCountDisagrees {
            face,
            certified: 2,
            constructed: 4,
        },
        S::NoSuchSolid {
            solid: SolidKey::default(),
        },
    ]
}

/// [`ContainError::ArcEndClearOfCorners`] at the sample band: an end
/// decided off both corners (`off_corner`, read only at a band finer
/// than the run's), or every end at a corner (the band's own parting, the
/// one the census reaches: the point within the zero band of the end's
/// point and past the escalation from a corner within the zero band of
/// it, so `K < 2`).
#[must_use]
pub fn arc_end_clear_of_corners(off_corner: bool) -> ContainError {
    let (zero, off) = (
        (Magnitude::Zero, MarginDiag::value(5e-10)),
        (Magnitude::Positive, MarginDiag::value(2e-8)),
    );
    let gaps = if off_corner {
        [[zero, off], [off, off]]
    } else {
        [[zero, off], [off, zero]]
    };
    ContainError::ArcEndClearOfCorners(Box::new(crate::boolean::ArcEndGaps { gaps, band: band() }))
}

fn contain_errors() -> Vec<ContainError> {
    use crate::boolean::ContainDecision;
    use crate::splitting::{Escalation, LoopDecision};
    let [value, _, poisoned] = diags();
    let over_wound = Indeterminate {
        margin: MarginDiag::value(-5e-9),
        ..value
    };
    // Each ending form a site can raise: the valued tighten (or the lever
    // alone where the margin gives none), the lever with the
    // unreadable-margin note, and a straddle of two bounds. The span rule
    // escalates only on an over-wound margin or a straddle.
    let arc_span = Some(ContainDecision::Loop(LoopDecision::ArcSpan));
    let escalated = |decision, escalation, diag| ContainError::Escalated {
        decision,
        escalation,
        diag,
    };
    // Every pair a site raises (`boolean::CONTAINMENT_RAISED`), in each form
    // its reading can take there.
    let mut v: Vec<ContainError> = crate::boolean::CONTAINMENT_RAISED
        .into_iter()
        .flat_map(|(decision, escalation)| match escalation {
            Escalation::Margin if decision == arc_span => {
                vec![escalated(decision, escalation, over_wound)]
            }
            Escalation::Margin => vec![
                escalated(decision, escalation, value),
                escalated(decision, escalation, poisoned),
            ],
            Escalation::Straddle => {
                vec![escalated(decision, escalation, poisoned)]
            }
        })
        .collect();
    v.extend([
        arc_end_clear_of_corners(true),
        arc_end_clear_of_corners(false),
        ContainError::RayExhausted,
        ContainError::StaleFace(crate::entity::FaceKey::default()),
        ContainError::EmptyLoop(LoopKey::default()),
        ContainError::LoopUnreadable(LoopKey::default()),
        ContainError::Uncrossable(uncrossable()),
    ]);
    v.extend(
        point_in_solid_errors()
            .into_iter()
            .map(ContainError::Curved),
    );
    v
}

/// The carrier-domain refusal on a width that overflows.
fn carrier_domain_invalid() -> geom_brep::CarrierDomainRefusal {
    geom_brep::CarrierDomainRefusal {
        lo: -f64::MAX,
        hi: f64::MAX,
        fault: geom_brep::CarrierDomainFault::Interval,
    }
}

/// A spiric with no certified net over the span holding its root's
/// zero.
fn carrier_enclosure() -> geom_brep::CarrierEnclosureRefusal {
    geom_brep::CarrierEnclosureRefusal {
        kind: geom::CurveKind::Spiric,
        span: (3.140625, 3.15),
    }
}

/// The carrier-domain refusal on a domain too narrow for its ends.
fn carrier_domain_collapsed() -> geom_brep::CarrierDomainRefusal {
    geom_brep::CarrierDomainRefusal {
        lo: 1.0e6,
        hi: 1.0e6 + 1.0e-9,
        fault: geom_brep::CarrierDomainFault::Merge,
    }
}

fn plane_nurbs_refusals() -> Vec<PlaneNurbsRefusal> {
    vec![
        PlaneNurbsRefusal::FootPointInconclusive {
            sample: 3,
            last_distance: 1e-7,
        },
        PlaneNurbsRefusal::NotTransverse {
            sample: 3,
            lever: geom_brep::ssi::PointLever::CurvatureRadius,
            verdict: zero_verdict(0.0),
        },
        PlaneNurbsRefusal::PcurveFit,
        PlaneNurbsRefusal::CarrierDomain(carrier_domain_invalid()),
        PlaneNurbsRefusal::CarrierDomain(carrier_domain_collapsed()),
        PlaneNurbsRefusal::CarrierEnclosure(carrier_enclosure()),
        PlaneNurbsRefusal::Limb {
            limb: geom_brep::SsiLimb::Tube,
            margin: MarginDiag::value(1e-7),
        },
        PlaneNurbsRefusal::TubeStraddles {
            verdict: Refused::Zero(Classified {
                margin: MarginDiag::value(5e-10),
                band: band(),
            }),
            boxes: 12,
        },
        PlaneNurbsRefusal::TubeStraddles {
            verdict: Refused::Negative {
                margin: MarginDiag::value(-1e-7),
            },
            boxes: 12,
        },
        PlaneNurbsRefusal::TransversalityEscalated {
            sample: 4,
            cause: diag(),
        },
        PlaneNurbsRefusal::Escalated {
            limb: geom_brep::SsiLimb::OnLocus,
            cause: diag(),
        },
        PlaneNurbsRefusal::Escalated {
            limb: geom_brep::SsiLimb::HullSup,
            cause: diag(),
        },
        PlaneNurbsRefusal::Escalated {
            limb: geom_brep::SsiLimb::Tube,
            cause: diag(),
        },
        PlaneNurbsRefusal::ArcRim(geom_brep::ArcRimRefusal::Escalated {
            check: geom_brep::PcurveCheck::MapResidual,
            cause: diag(),
        }),
        PlaneNurbsRefusal::ArcRim(geom_brep::ArcRimRefusal::Refused {
            iso: Some(geom_brep::PcurveCheck::MapResidual),
            limb: geom_brep::SsiLimb::HullValueChart,
            bound: MarginDiag::value(4.6e-4),
        }),
        PlaneNurbsRefusal::TubeNotOneArc {
            rungs: 3,
            cause: geom_brep::ssi::OneArcRefusal::Short,
        },
        PlaneNurbsRefusal::TubeNotOneArc {
            rungs: 3,
            cause: geom_brep::ssi::OneArcRefusal::Undecided(diag()),
        },
        PlaneNurbsRefusal::TubeNotOneArc {
            rungs: 3,
            cause: geom_brep::ssi::OneArcRefusal::Count { solutions: 0 },
        },
        PlaneNurbsRefusal::TubeNotOneArc {
            rungs: 20,
            cause: geom_brep::ssi::OneArcRefusal::Count { solutions: 4 },
        },
        PlaneNurbsRefusal::TubeNotOneArc {
            rungs: 20,
            cause: geom_brep::ssi::OneArcRefusal::Unlinked,
        },
        PlaneNurbsRefusal::TubeNotOneArc {
            rungs: 16,
            cause: geom_brep::ssi::OneArcRefusal::AlongSide { chart: 0 },
        },
        PlaneNurbsRefusal::TubeNotOneArc {
            rungs: 16,
            cause: geom_brep::ssi::OneArcRefusal::EndOnSides { sides: 2 },
        },
        PlaneNurbsRefusal::ReportedTransversalityPoisoned(diag()),
        PlaneNurbsRefusal::ChartSpeed(geom_brep::ChartSpeedRefusal::Zero {
            axis: geom_brep::ChartAxis::U,
        }),
        PlaneNurbsRefusal::Unsupported {
            what: "a rational NURBS surface",
        },
    ]
}

fn certify_errors() -> Vec<CertifyError> {
    let key = geom_brep::SurfaceKey::default();
    let mut v = vec![
        CertifyError::ChartImageUnavailable {
            chart: geom::SurfaceKind::Cone,
            carrier: geom::CurveKind::Ellipse,
        },
        CertifyError::NoGeneralLane {
            chart: geom::SurfaceKind::Torus,
            carrier: geom::CurveKind::Circle,
            gap: geom_brep::LaneGap::Unwritten,
        },
        CertifyError::NoGeneralLane {
            chart: geom::SurfaceKind::Sphere,
            carrier: geom::CurveKind::Circle,
            gap: geom_brep::LaneGap::Pose {
                piece: 1,
                channel: geom_brep::SectorChannel::Azimuth,
            },
        },
        CertifyError::CarrierOffChart {
            chart: geom::SurfaceKind::Plane,
            carrier: geom::CurveKind::Line,
            why: "the line's points are not on the chart",
            margin: None,
        },
        CertifyError::CarrierOffChart {
            chart: geom::SurfaceKind::Cone,
            carrier: geom::CurveKind::Line,
            why: "the line is not shown off the plane through the apex",
            margin: Some(geom_brep::OffMargin::InBand(MarginDiag::value(4e-10))),
        },
        CertifyError::UnresolvedSurface { key },
        CertifyError::Unimplemented,
        CertifyError::PlaneNurbs(PlaneNurbsRefusal::ArcRim(
            geom_brep::ArcRimRefusal::Refused {
                iso: None,
                limb: geom_brep::SsiLimb::HullValueChart,
                bound: MarginDiag::value(4.6e-4),
            },
        )),
        CertifyError::IntersectionSameSurface { key },
        CertifyError::Row {
            surface: key,
            refusal: geom_brep::RowRefusal::Image(Some(geom_brep::PcurveCheck::MapResidual)),
        },
        CertifyError::Row {
            surface: key,
            refusal: geom_brep::RowRefusal::Image(None),
        },
        CertifyError::Row {
            surface: key,
            refusal: geom_brep::RowRefusal::ImageEscalated {
                check: geom_brep::PcurveCheck::ParamSpan,
                cause: diag(),
            },
        },
        CertifyError::Row {
            surface: key,
            refusal: geom_brep::RowRefusal::ChartSpeed(geom_brep::ChartSpeedRefusal::Zero {
                axis: geom_brep::ChartAxis::U,
            }),
        },
        CertifyError::Row {
            surface: key,
            refusal: geom_brep::RowRefusal::OffSide,
        },
        CertifyError::WrapOnNonPeriodic,
        // Both zero-span stories: a length a smaller tolerance decides,
        // and a span of no length, which none does.
        CertifyError::IntervalNotForward {
            verdict: zero_verdict(5e-10),
        },
        CertifyError::IntervalNotForward {
            verdict: zero_verdict(0.0),
        },
        CertifyError::IntervalNotForward {
            verdict: negative_verdict(-1e-3),
        },
        CertifyError::WindingExceeded,
        // A door's input interval, of no length and past a full turn.
        CertifyError::StatedInterval {
            refused: geom_brep::StatedSpan::Forward(geom_brep::ForwardReading::Decided(
                zero_verdict(0.0),
            )),
        },
        CertifyError::StatedInterval {
            refused: geom_brep::StatedSpan::Winding(geom_brep::WindingReading::Exceeded),
        },
        CertifyError::ResidualExceeded {
            check: CertCheck::SurfaceResidual { surface: key },
            sample: 4,
            margin: MarginDiag::value(3e-8),
        },
        CertifyError::NotTransverse {
            lever: None,
            sample: 4,
            verdict: zero_verdict(5e-10),
        },
        // A lever arm or a spline's metered length that is not there:
        // one a smaller tolerance decides, and one of no length at all.
        CertifyError::ArmCollapsed {
            sample: 4,
            verdict: zero_verdict(0.0),
        },
        CertifyError::ArmCollapsed {
            sample: 4,
            verdict: zero_verdict(5e-10),
        },
        CertifyError::SpanMeterCollapsed {
            verdict: zero_verdict(0.0),
        },
        CertifyError::SpanMeterCollapsed {
            verdict: negative_verdict(-1e-3),
        },
        CertifyError::NotSecondOrderSeparated {
            sample: 4,
            verdict: zero_verdict(0.0),
        },
        CertifyError::TubeNotSeparated {
            verdict: zero_verdict(5e-10),
        },
        CertifyError::TubeNotSeparated {
            verdict: negative_verdict(-1e-7),
        },
        CertifyError::TangentCertificateUnsupported,
        CertifyError::Escalated {
            check: CertCheck::Transversality,
            sample: 4,
            cause: diag(),
        },
        CertifyError::Escalated {
            check: CertCheck::TransversalityArm,
            sample: 4,
            cause: diag(),
        },
        CertifyError::Escalated {
            check: CertCheck::ParamSpanMeter,
            sample: geom_brep::certify::NOT_A_SAMPLE,
            cause: diag(),
        },
        // A carrier's incidence in band at the edge certificate.
        CertifyError::Escalated {
            check: CertCheck::ChartIncidence,
            sample: geom_brep::certify::NOT_A_SAMPLE,
            cause: diag(),
        },
        // A gate's rejected negative, routed as undecided: a stored
        // contradiction's definite reading, and one the lever reaches.
        CertifyError::Escalated {
            check: CertCheck::TransversalityArm,
            sample: 4,
            cause: rejected_negative("dihedral_arm"),
        },
        CertifyError::Escalated {
            check: CertCheck::ParamSpanMeter,
            sample: geom_brep::certify::NOT_A_SAMPLE,
            cause: rejected_negative("param_span_meter"),
        },
    ];
    v.extend(band_errors().into_iter().map(CertifyError::Band));
    v.extend(
        plane_nurbs_refusals()
            .into_iter()
            .map(CertifyError::PlaneNurbs),
    );
    v.extend(
        analytic_rung3_refusals()
            .into_iter()
            .map(CertifyError::AnalyticRung3),
    );
    v
}

fn analytic_rung3_refusals() -> Vec<geom_brep::AnalyticRung3Refusal> {
    use geom_brep::AnalyticRung3Refusal as A;
    vec![
        A::KernelLimit {
            operand: geom::SurfaceKind::Plane,
            bound: 7.5e-5,
            margin: MarginDiag::value(7.5e-5),
            width: 1e-6,
            extent: 2.0,
        },
        A::KernelLimit {
            operand: geom::SurfaceKind::Torus,
            bound: 7.5e-5,
            margin: MarginDiag::value(7.5e-5),
            width: 1e-15,
            extent: 2.0,
        },
        A::Limb {
            operand: geom::SurfaceKind::Plane,
            margin: MarginDiag::value(7.5e-5),
        },
        A::Escalated {
            operand: Some(geom::SurfaceKind::Cylinder),
            limb: geom_brep::Rung3Limb::HullSup,
            cause: diag(),
        },
        A::Escalated {
            operand: None,
            limb: geom_brep::Rung3Limb::Tube,
            cause: diag(),
        },
        A::Stalled {
            operand: geom::SurfaceKind::Cylinder,
            cause: diag(),
        },
        A::NoOffsetBound {
            operand: geom::SurfaceKind::Cone,
            why: geom_brep::SectorChannel::Lever.describe(),
        },
        A::CarrierEnclosure(carrier_enclosure()),
        A::NoTubeEnclosure {
            operand: geom::SurfaceKind::Torus,
        },
        A::TubeStraddles {
            verdict: Refused::Zero(Classified {
                margin: MarginDiag::value(0.0),
                band: band(),
            }),
            boxes: 12,
        },
        A::TubeNotOneArc {
            rungs: 3,
            cause: geom_brep::ssi::OneArcRefusal::Short,
        },
        A::Unsupported {
            what: "the analytic rung-3 certificate reads two analytic operands",
        },
    ]
}

fn pcurve_certify_errors() -> Vec<PcurveCertifyError> {
    let mut v = vec![
        PcurveCertifyError::UnsupportedChart {
            chart: geom::SurfaceKind::Torus,
        },
        PcurveCertifyError::NoGeneralLane {
            chart: geom::SurfaceKind::Torus,
            carrier: geom::CurveKind::Circle,
            gap: geom_brep::LaneGap::Unwritten,
        },
        PcurveCertifyError::NoGeneralLane {
            chart: geom::SurfaceKind::Sphere,
            carrier: geom::CurveKind::Circle,
            gap: geom_brep::LaneGap::Pose {
                piece: 1,
                channel: geom_brep::SectorChannel::Azimuth,
            },
        },
        PcurveCertifyError::CarrierOffChart {
            chart: geom::SurfaceKind::Sphere,
            carrier: geom::CurveKind::Line,
            why: "a sphere holds no line",
            margin: None,
        },
        PcurveCertifyError::CarrierOffChart {
            chart: geom::SurfaceKind::Cone,
            carrier: geom::CurveKind::Circle,
            why: "the rim's height is not shown away from the apex's, where the apex plane \
                  meets the cone only at the apex",
            margin: Some(geom_brep::OffMargin::InBand(geom_core::MarginDiag::value(
                4e-10,
            ))),
        },
        PcurveCertifyError::CarrierOffChart {
            chart: geom::SurfaceKind::Torus,
            carrier: geom::CurveKind::Circle,
            why: "the circle is no section of the torus",
            margin: Some(geom_brep::OffMargin::Past(geom_core::MarginDiag::value(
                3e-7,
            ))),
        },
        PcurveCertifyError::Escalated {
            check: geom_brep::PcurveCheck::ChartIncidence,
            sample: 0,
            cause: diag(),
        },
        PcurveCertifyError::ImageMismatch {
            image: geom_brep::PcurveKind::General,
            why: "a fitted-grade image at the closed-form door",
        },
        PcurveCertifyError::FittedLaneUnsupported { scalar: "dual" },
        PcurveCertifyError::SectorRefused {
            piece: 0,
            channel: geom_brep::SectorChannel::Azimuth,
        },
        PcurveCertifyError::IsoUnsupported {
            what: "a rational NURBS surface",
        },
        PcurveCertifyError::ChartRow {
            source: geom_core::spline::SplineError::DomainInvalid { lo: 1.0, hi: 0.0 },
        },
        PcurveCertifyError::FittedCertificate {
            limb: Some(geom_brep::SsiLimb::OnLocus),
            what: "a foot point of the chart-image schedule would not converge",
            magnitude: Some(FittedMagnitude::LastFootDistance {
                t: 0.5,
                last_distance: 1e-7,
            }),
        },
        PcurveCertifyError::FittedLimb {
            limb: geom_brep::SsiLimb::HullSupChart,
            margin: MarginDiag::value(1e-7),
        },
        PcurveCertifyError::CarrierDomain(carrier_domain_collapsed()),
        PcurveCertifyError::CarrierEnclosure(carrier_enclosure()),
        PcurveCertifyError::FittedEscalated {
            decision: geom_brep::SsiDecision::Limb(geom_brep::SsiLimb::HullSupChart),
            cause: diag(),
        },
        PcurveCertifyError::IntervalNotForward,
        PcurveCertifyError::ChartWindingUnsupported,
        PcurveCertifyError::ChannelKindMismatch {
            chart: geom::SurfaceKind::Cylinder,
        },
        PcurveCertifyError::WindingMismatch {
            stored: [Some(1), None],
            derived: [Some(-1), None],
        },
        PcurveCertifyError::PlaceholderChart,
        PcurveCertifyError::AzimuthPeriodExceeded,
        PcurveCertifyError::PlacementTurnsExceeded,
        PcurveCertifyError::TubePeriodExceeded,
        PcurveCertifyError::ResidualExceeded {
            check: PcurveCheck::MapResidual,
            sample: 4,
        },
        PcurveCertifyError::Escalated {
            check: PcurveCheck::Envelope,
            sample: 4,
            cause: diag(),
        },
    ];
    v.extend(band_errors().into_iter().map(PcurveCertifyError::Band));
    v
}

fn pcurve_mint_errors() -> Vec<PcurveMintError> {
    let face = FaceKey::default();
    let half_edge = HalfEdgeKey::default();
    let r#loop = LoopKey::default();
    let mut v = vec![
        PcurveMintError::Stale {
            role: "half_edge",
            key: EntityId::HalfEdge(half_edge),
        },
        PcurveMintError::NoCarrier { half_edge },
        PcurveMintError::LoopDiscontinuity { half_edge },
        PcurveMintError::LoopNotClosed { face },
        PcurveMintError::JointWithoutRoom {
            face,
            r#loop,
            half_edge,
        },
        PcurveMintError::MissingCache { half_edge },
        PcurveMintError::Unminted { face },
        PcurveMintError::RowInterval { half_edge },
        PcurveMintError::PlaceholderChart { face },
        PcurveMintError::Escalated {
            half_edge,
            cause: diag(),
        },
        PcurveMintError::NetStretchUndecided {
            half_edge,
            cause: diag(),
        },
    ];
    v.extend(band_errors().into_iter().map(PcurveMintError::Band));
    v.extend(
        pcurve_certify_errors()
            .into_iter()
            .map(|error| PcurveMintError::Certify { half_edge, error }),
    );
    v
}

fn props_errors() -> Vec<PropsError> {
    vec![
        PropsError::Unimplemented,
        PropsError::NotIsoRectangle {
            what: "a trim edge that is not an iso-parameter line",
        },
        PropsError::NappeSpanning,
        PropsError::SphereLoop {
            what: "props_loop_closed",
        },
        PropsError::ConeLoop {
            what: "props_cone_apex_loop_sweep",
        },
        PropsError::HolesExceedFace,
        PropsError::SenseContradicted,
        PropsError::SenseInverted,
        PropsError::SeamRegionOvercovered { times: 2 },
        PropsError::SeamRegionCoverUnresolved,
        PropsError::SeamRegionSensesDisagree,
        PropsError::RingWoundWithFace,
        PropsError::ChartLoopOpen,
        PropsError::LoopsWindChart,
        PropsError::NotOneChartBranch {
            edge: 2,
            what: "the edge crosses the seam",
        },
        PropsError::DegenerateFace,
        PropsError::OffSurface {
            what: "a rim circle that is not on the cylinder",
        },
        PropsError::Escalated {
            cause: diag(),
            check: geom_brep::props::PropsCheck::Inventory,
        },
        PropsError::QuadratureUnsupported {
            what: "a NURBS face",
        },
    ]
}

fn mass_props_errors() -> Vec<MassPropsError> {
    let face = FaceKey::default();
    let mut v: Vec<MassPropsError> = band_errors()
        .into_iter()
        .map(|error| MassPropsError::Band { error })
        .collect();
    v.extend([
        MassPropsError::RingOnCurvedFace { face },
        MassPropsError::Corrupt {
            what: "a face's loop does not close",
        },
        MassPropsError::NullScaffoldEdge {
            edge: EdgeKey::default(),
        },
        MassPropsError::ResolutionFinerThanEps {
            resolution: 1e-10,
            eps: 1e-9,
        },
        MassPropsError::EmptyContains,
        MassPropsError::Poisoned { cause: diag() },
        MassPropsError::Undecided {
            volume: (-1e-9, 1e-9),
            stop: crate::Stop::Wall,
        },
    ]);
    v.extend(
        props_errors()
            .into_iter()
            .map(|source| MassPropsError::Face { face, source }),
    );
    v
}

fn offset_fit_errors() -> Vec<OffsetFitError> {
    use geom_brep::offset_meters::{Meter, MeterError};
    use geom_brep::patch_bound::PatchBoundError;
    let zero = zero_verdict;
    let mut v = vec![
        // Both zero-floor stories: a thinness a smaller tolerance
        // decides, and a floor of exactly zero, which none does.
        OffsetFitError::Meter(MeterError::NormalFloor {
            floor: 1e-9,
            speed_lever: 2.0,
            verdict: zero(5e-10),
        }),
        OffsetFitError::Meter(MeterError::NormalFloor {
            floor: 0.0,
            speed_lever: 2.0,
            verdict: zero(0.0),
        }),
        // Both verdicts: a sign-certain fold, and one inside the zero
        // band, whose ending differs.
        OffsetFitError::Meter(MeterError::CurvatureHeadroom {
            reach: 0.5,
            kappa: (2.0, 0.5),
            verdict: Refused::Negative {
                margin: MarginDiag::value(-0.1),
            },
        }),
        OffsetFitError::Meter(MeterError::CurvatureHeadroom {
            reach: 0.5,
            kappa: (2.0, 0.5),
            verdict: zero(5e-10),
        }),
        OffsetFitError::Meter(MeterError::Escalated {
            meter: Meter::NormalFloor,
            source: diag().with_predicate(Meter::NormalFloor.predicate()),
        }),
        OffsetFitError::Fit(geom::curves::fit::FitError::TooFewPoints { have: 2, need: 4 }),
        OffsetFitError::Structure(geom_core::spline::SplineError::DomainInvalid {
            lo: 1.0,
            hi: 0.0,
        }),
        OffsetFitError::InvalidRequest {
            d: 0.0,
            tolerance: 0.0,
        },
        OffsetFitError::NonFiniteSample { uv: (0.5, 0.25) },
        OffsetFitError::SeedOverCap {
            cap: OFFSET_FIT_SAMPLE_CAP,
            grid: (49, 48),
        },
        OffsetFitError::WallReached {
            wall: geom_brep::OffsetFitWall::SampleCap {
                cap: OFFSET_FIT_SAMPLE_CAP,
            },
            rounds: 6,
            grid: (48, 48),
            achieved: 3e-6,
            tolerance: 1e-6,
            best: geom_brep::BestBound {
                bound: 2e-6,
                grid: (36, 48),
            },
        },
        OffsetFitError::WallReached {
            wall: geom_brep::OffsetFitWall::Work {
                budget: OFFSET_FIT_WORK,
                spent: OFFSET_FIT_WORK - 1000,
            },
            rounds: 9,
            grid: (40, 32),
            achieved: 3e-6,
            tolerance: 1e-6,
            best: geom_brep::BestBound {
                bound: 2e-6,
                grid: (36, 32),
            },
        },
        OffsetFitError::BoundNotFinite {
            rounds: 6,
            grid: (48, 48),
            d: 0.1,
            tolerance: 1e-6,
            best: Some(geom_brep::BestBound {
                bound: 3e-6,
                grid: (36, 48),
            }),
        },
        OffsetFitError::BoundNotFinite {
            rounds: 6,
            grid: (48, 48),
            d: 1e-8,
            tolerance: 1e-6,
            best: None,
        },
        OffsetFitError::RefinementStalled {
            rounds: 6,
            grid: (48, 48),
            achieved: 3e-6,
            tolerance: 1e-6,
            best: geom_brep::BestBound {
                bound: 2e-6,
                grid: (36, 48),
            },
        },
        OffsetFitError::WindowUnsupported {
            window: geom::ApproxWindow {
                u: (0.0, 1.0),
                v: (0.0, 1.0),
            },
        },
        OffsetFitError::Limb {
            limb: OffsetLimb::HullSup,
            bound: 3e-6,
            tolerance: 1e-6,
        },
        OffsetFitError::Limb {
            limb: OffsetLimb::OnLocus,
            bound: 3e-6,
            tolerance: 1e-6,
        },
        // The mint's: the loop's NaN residual, and its certification
        // sampling the accepted fit above the tolerance.
        OffsetFitError::MintLimb {
            limb: OffsetLimb::OnLocus,
            bound: f64::NAN,
            tolerance: 1e-6,
        },
        OffsetFitError::MintLimb {
            limb: OffsetLimb::OnLocus,
            bound: 3e-6,
            tolerance: 1e-6,
        },
    ];
    // Every patch-bound note is its own sentence, and the enum is
    // fieldless, so its compiler-derived roster is the sample list.
    v.extend(PatchBoundError::iter().map(OffsetFitError::PatchBound));
    v.extend(band_errors().into_iter().map(OffsetFitError::Band));
    v
}

fn census_contacts() -> Vec<CensusContact> {
    let (vertex, edge, face) = (VertexKey::default(), EdgeKey::default(), FaceKey::default());
    vec![
        CensusContact::VertexVertex {
            a: vertex,
            b: vertex,
        },
        CensusContact::VertexOnFace { vertex, face },
        CensusContact::VertexOnEdge { vertex, edge },
        CensusContact::EdgeFacePierce { edge, face },
        CensusContact::EdgeEdgeCross {
            a: edge,
            b: edge,
            side: None,
        },
        CensusContact::EdgeEdgeOverlap { a: edge, b: edge },
        CensusContact::EdgeFaceOverlap { edge, face },
        CensusContact::FaceFaceTangent {
            finding: ContactFinding {
                pair: DeclaredContact {
                    a: face,
                    b: face,
                    class: GlueClass::Tangent,
                },
                verdict: ContactVerdict::Definite,
            },
        },
        CensusContact::ConformalPatch {
            finding: ContactFinding {
                pair: DeclaredContact {
                    a: face,
                    b: face,
                    class: GlueClass::Rest,
                },
                verdict: ContactVerdict::Definite,
            },
        },
    ]
}

fn stale_declarations() -> Vec<StaleDeclaration> {
    let (vertex, face) = (VertexKey::default(), FaceKey::default());
    vec![
        StaleDeclaration::VertexVertex {
            a: vertex,
            b: vertex,
        },
        StaleDeclaration::VertexOnFace { vertex, face },
        StaleDeclaration::VertexOnEdge {
            vertex,
            edge: EdgeKey::default(),
        },
        StaleDeclaration::EdgeEdge {
            a: EdgeKey::default(),
            b: EdgeKey::default(),
        },
        StaleDeclaration::EdgeFace {
            edge: EdgeKey::default(),
            face,
        },
        StaleDeclaration::Patch {
            face_a: face,
            face_b: face,
        },
    ]
}

fn ring_contacts() -> Vec<RingContact> {
    let (vertex, edge) = (VertexKey::default(), EdgeKey::default());
    vec![
        RingContact::Vertex {
            ring_vertex: vertex,
            outer_vertex: vertex,
        },
        RingContact::VertexOnEdge {
            ring_vertex: vertex,
            outer_edge: edge,
        },
        RingContact::Edge {
            ring_edge: edge,
            outer_edge: edge,
        },
        RingContact::OuterVertexOnEdge {
            outer_vertex: vertex,
            ring_edge: edge,
        },
        RingContact::Circles {
            ring_loop: LoopKey::default(),
            outer_loop: LoopKey::default(),
        },
        RingContact::EdgesMeet {
            ring_edge: edge,
            outer_edge: edge,
        },
    ]
}

fn ring_pair_contacts() -> Vec<RingPairContact> {
    let (vertex, edge, r#loop) = (VertexKey::default(), EdgeKey::default(), LoopKey::default());
    vec![
        RingPairContact::Vertex {
            ring_vertex: vertex,
            other_vertex: vertex,
        },
        RingPairContact::VertexOnEdge {
            ring_vertex: vertex,
            other_edge: edge,
        },
        RingPairContact::Edge {
            ring_edge: edge,
            other_edge: edge,
        },
        RingPairContact::OtherVertexOnEdge {
            other_vertex: vertex,
            ring_edge: edge,
        },
        RingPairContact::Circles {
            ring_loop: r#loop,
            other_loop: r#loop,
        },
        RingPairContact::EdgesMeet {
            ring_edge: edge,
            other_edge: edge,
        },
    ]
}

/// `arm/Variant`, the variant read off the nested value's `Debug`.
fn label<T: core::fmt::Debug>(arm: &str, nested: &T) -> String {
    let debug = format!("{nested:?}");
    let head: String = debug
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    format!("{arm}/{head}")
}

/// `arm/Curved/Loop/Variant`: [`label`] read down the arms that carry a
/// solid or loop refusal whole, and down to an off-plane loop's cause,
/// so each carried refusal gets its own label.
fn path_label<T: core::fmt::Debug>(arm: &str, nested: &T) -> String {
    let debug = format!("{nested:?}");
    let mut path = arm.to_owned();
    let mut rest = debug.as_str();
    loop {
        let head: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        path = format!("{path}/{head}");
        match rest[head.len()..].strip_prefix('(') {
            Some(inner) if matches!(head.as_str(), "Curved" | "Loop") => rest = inner,
            // An escalation's ending differs by its decision, how its
            // reading stood, and its margin's kind.
            _ if head == "Escalated" && rest.contains("escalation: ") => {
                let field = |name: &str| {
                    rest.split_once(name)
                        .map_or("", |(_, tail)| tail.split([',', ' ']).next().unwrap_or(""))
                };
                let decision = rest
                    .split_once("decision: ")
                    .and_then(|(_, tail)| tail.split_once(", escalation"))
                    .map_or("", |(d, _)| d);
                let kind = rest.split_once("margin: ").map_or("", |(_, tail)| {
                    tail.split(|c: char| !c.is_alphanumeric())
                        .next()
                        .unwrap_or("")
                });
                return format!("{path}/{decision}/{}/{kind}", field("escalation: "));
            }
            // An off-plane loop's refusals differ by cause alone.
            _ if head == "OffPlane" => match rest.split_once("cause: ") {
                Some((_, cause)) => rest = cause,
                None => return path,
            },
            _ => return path,
        }
    }
}

/// Every [`ValidationError`] shape the viewer can draw, each with a
/// short label naming the arm and the nested variant it carries (the
/// module docs).
#[must_use]
pub fn validation_error_samples() -> Vec<(String, ValidationError)> {
    let face = FaceKey::default();
    let edge = EdgeKey::default();
    let vertex = VertexKey::default();
    let he = HalfEdgeKey::default();
    let loop_ = LoopKey::default();
    let shell = ShellKey::default();
    let solid = SolidKey::default();
    let point = PointKey::default();
    let pair = CensusSubject::FacePair(face, face);
    let mut s: Vec<(String, ValidationError)> = Vec::new();

    // Tiers 1 and 2: the body's own structure.
    for e in [
        ValidationError::DanglingTopology {
            from: EntityId::Solid(solid),
            to: EntityId::Shell(shell),
        },
        ValidationError::DanglingGeometry {
            from: EntityId::Vertex(vertex),
            to: GeomRef::Point(point),
        },
        ValidationError::DanglingDescription {
            from: GeomRef::Point(point),
            to: GeomRef::Point(point),
        },
        ValidationError::NextPrevMismatch { half_edge: he },
        ValidationError::LoopCycleOverrun { loop_ },
        ValidationError::ParentLoopMismatch {
            half_edge: he,
            owner: loop_,
        },
        ValidationError::UnreachableHalfEdge { half_edge: he },
        ValidationError::EdgeHalvesIdentical { edge },
        ValidationError::EdgeSlotBackpointerMismatch {
            edge,
            half_edge: he,
        },
        ValidationError::HalfEdgeUnclaimed { half_edge: he },
        ValidationError::HalfEdgeMultiplyClaimed {
            half_edge: he,
            claims: 2,
        },
        ValidationError::EdgeNotAntiparallel { edge },
        ValidationError::EmanatingStartMismatch {
            vertex,
            emanating: he,
        },
        ValidationError::EmptyLoopVertexWithEmanating {
            vertex,
            empty_loops: 1,
        },
        ValidationError::LoneVertexWithIncidence {
            vertex,
            incident: 2,
        },
        ValidationError::VertexOrbitOverrun { vertex },
        ValidationError::OrbitForeignMember {
            vertex,
            half_edge: he,
        },
        ValidationError::SplitVertexOrbit {
            vertex,
            orbit: 2,
            incident: 4,
        },
        ValidationError::OuterListedAsRing { face },
        ValidationError::BackPointerMismatch {
            child: EntityId::Loop(loop_),
            stored: EntityId::Face(face),
            owner: EntityId::Face(face),
        },
        ValidationError::OrphanEntity {
            entity: EntityId::Vertex(vertex),
        },
        ValidationError::MultiplyOwned {
            child: EntityId::Shell(shell),
            owners: 2,
        },
        ValidationError::OrphanGeometry {
            geometry: GeomRef::Point(point),
        },
        ValidationError::SolidWithoutShells { solid },
        ValidationError::ShellWithoutFaces { shell },
        ValidationError::EdgeAcrossShells {
            edge,
            shell_plus: shell,
            shell_minus: shell,
        },
        ValidationError::ComponentEulerViolation {
            shell,
            seed: face,
            vertices: 2,
            edges: 2,
            faces: 1,
            rings: 0,
        },
        ValidationError::MissingProvenance {
            entity: EntityId::Face(face),
        },
        ValidationError::LeakedProvenance {
            entity: EntityId::Face(face),
        },
        ValidationError::ScaffoldingEmptyLoop { loop_ },
        ValidationError::ScaffoldingStrutVertex { vertex },
        ValidationError::ShellDisconnected {
            shell,
            components: 2,
        },
        ValidationError::NullScaffoldShared {
            curve: CurveKey::default(),
            edges: 2,
        },
        ValidationError::LeakedNullFaceRecord { face },
        ValidationError::StaleNullFaceLoop {
            face,
            named_loop: loop_,
        },
        ValidationError::StaleNullFaceOwnership {
            face,
            named_loop: loop_,
        },
        ValidationError::NullEdgeAtRest { edge },
        ValidationError::NullFaceAtRest { face },
        // Tier 3 arms that carry nothing nested.
        ValidationError::UncertifiableSurface { face },
        ValidationError::PoisonedSurfaceDescription { face },
        ValidationError::ApproxLaneUnsupported {
            face,
            scalar: "interval",
        },
        ValidationError::DegenerateTorus {
            face,
            verdict: zero_verdict(5e-10),
        },
        ValidationError::DegenerateTorus {
            face,
            verdict: negative_verdict(-1e-3),
        },
        ValidationError::DescriptionNotAdjacent { edge },
        ValidationError::PlanarFaceResidual { face, vertex },
        ValidationError::PlanarBoundaryResidual { face, edge },
        ValidationError::TransverseNotIntrinsic { edge },
        ValidationError::MixedDihedral {
            edge,
            stations: geom_brep::MixedStations {
                transverse: 3,
                smooth: 4,
            },
        },
        ValidationError::ScaffoldAtRest { edge },
        ValidationError::JoinableVertexAtRest { vertex },
        ValidationError::JoinUndecidedAtRest {
            undecided: crate::boolean::JoinUndecided {
                vertex,
                reading: crate::boolean::JoinReading::Regularity(diag()),
            },
        },
        ValidationError::JoinUndecidedAtRest {
            undecided: crate::boolean::JoinUndecided {
                vertex,
                reading: crate::boolean::JoinReading::ChartClass(
                    geom_brep::IsoFamilyRefusal::Undecided(diag()),
                ),
            },
        },
        ValidationError::TangentNotIntrinsic { edge },
        ValidationError::LaminaWedge { edge },
        ValidationError::NoDihedralArm {
            edge,
            verdict: zero_verdict(0.0),
        },
        ValidationError::LoopRoleInverted {
            face,
            r#loop: loop_,
        },
        ValidationError::CurvedSenseInverted { face },
        ValidationError::SeamRegionOvercovered { face, times: 2 },
        ValidationError::SeamRegionCoverUnresolved { face },
        ValidationError::ChartSingularityNotAVertex {
            face,
            point: geom_brep::chart_singularity::Singularity::North,
            lies: crate::SingularityLies::Interior,
        },
        ValidationError::ChartSingularityNotAVertex {
            face,
            point: geom_brep::chart_singularity::Singularity::Apex,
            lies: crate::SingularityLies::InsideEdge { edge },
        },
        ValidationError::ChartSingularityUndecided {
            face,
            point: geom_brep::chart_singularity::Singularity::South,
            reading: geom_brep::chart_singularity::Undecided::Boundary(diag()),
        },
        ValidationError::ChartSingularityUndecided {
            face,
            point: geom_brep::chart_singularity::Singularity::North,
            reading: geom_brep::chart_singularity::Undecided::Side(
                geom_brep::chart_singularity::SideUnread::Straddle,
            ),
        },
        ValidationError::NegativeVolume { solid },
        ValidationError::SliverInsideOut { solid },
        ValidationError::RingOutsideOuter {
            face,
            ring: loop_,
            ring_vertex: vertex,
        },
        ValidationError::PinchCornerCrossed { face, vertex, edge },
        ValidationError::CensusLaneUnsupported { subject: pair },
        ValidationError::InstanceInterference {
            outer: solid,
            inner: solid,
            witness: vertex,
        },
        ValidationError::SameSideInterference { a: face, b: face },
        ValidationError::TouchInterference {
            a: crate::Cell::Vertex(vertex),
            b: crate::Cell::Face(face),
            witness: "(0.0, 2.0, 0.5)".to_owned(),
        },
        ValidationError::CuspEdgesMissing {
            a: face,
            b: face,
            witness: "(0.0, 2.0, 0.5)".to_owned(),
        },
    ] {
        s.push((label("", &e).trim_start_matches('/').to_owned(), e));
    }

    // A surface datum, poisoned or outside its range: every datum, at
    // each end of the range.
    for datum in geom::SurfaceDatum::iter() {
        let kind = geom::SurfaceKind::Torus;
        s.push((
            label("PoisonedSurfaceDatum", &datum),
            ValidationError::PoisonedSurfaceDatum { face, kind, datum },
        ));
        // Every measure at every end, the frame's included: the
        // renderings differ by measure, so each is budget-checked.
        for measure in geom::ConventionMeasure::iter() {
            for end in geom::ConventionEnd::iter() {
                s.push((
                    format!(
                        "{}/{measure:?}",
                        label("UnrepresentableSurfaceDatum", &datum)
                    ),
                    ValidationError::UnrepresentableSurfaceDatum {
                        face,
                        kind,
                        datum,
                        measure,
                        end,
                    },
                ));
            }
        }
    }

    // A carrier datum, poisoned or outside its range: every datum, at
    // each end of the range.
    for datum in geom::CurveDatum::iter() {
        let kind = geom::CurveKind::Ellipse;
        s.push((
            label("PoisonedCurveDatum", &datum),
            ValidationError::PoisonedCurveDatum { edge, kind, datum },
        ));
        // Every measure at every end, the frame's included: the
        // renderings differ by measure, so each is budget-checked.
        for measure in geom::ConventionMeasure::iter() {
            for end in geom::ConventionEnd::iter() {
                s.push((
                    format!("{}/{measure:?}", label("UnrepresentableCurveDatum", &datum)),
                    ValidationError::UnrepresentableCurveDatum {
                        edge,
                        kind,
                        datum,
                        measure,
                        end,
                    },
                ));
            }
        }
    }

    // Tier 3 arms that render something nested: one sample per variant.
    for error in band_errors() {
        s.push((label("Band", &error), ValidationError::Band { error }));
    }
    for error in offset_fit_errors() {
        s.push((
            label("ApproxCertification", &error),
            ValidationError::ApproxCertification { face, error },
        ));
    }
    for error in certify_errors() {
        let l = match &error {
            CertifyError::PlaneNurbs(r) => label("EdgeCertification/PlaneNurbs", r),
            CertifyError::AnalyticRung3(r) => label("EdgeCertification/AnalyticRung3", r),
            other => label("EdgeCertification", other),
        };
        s.push((l, ValidationError::EdgeCertification { edge, error }));
    }
    for cause in diags() {
        let m = label("", &cause.margin);
        for (arm, e) in [
            (
                "DegenerateTorusEscalated",
                ValidationError::DegenerateTorusEscalated { face, cause },
            ),
            (
                "PlanarFaceEscalated",
                ValidationError::PlanarFaceEscalated {
                    face,
                    vertex,
                    cause,
                },
            ),
            (
                "PlanarBoundaryEscalated",
                ValidationError::PlanarBoundaryEscalated { face, edge, cause },
            ),
            (
                "RingContactEscalated",
                ValidationError::RingContactEscalated {
                    face,
                    ring: loop_,
                    source: cause,
                },
            ),
            (
                "RingPairContactEscalated",
                ValidationError::RingPairContactEscalated {
                    face,
                    ring: loop_,
                    other: loop_,
                    source: cause,
                },
            ),
            (
                "PinchCornerEscalated",
                ValidationError::PinchCornerEscalated {
                    face,
                    vertex,
                    source: cause,
                },
            ),
            (
                "CensusEscalated",
                ValidationError::CensusEscalated { cause },
            ),
        ] {
            s.push((format!("{arm}{m}"), e));
        }
        for check in [
            WedgeCheck::Arm,
            WedgeCheck::Dihedral,
            WedgeCheck::SecondOrder,
            WedgeCheck::MaterialPairing,
            WedgeCheck::MaterialSide,
        ] {
            s.push((
                format!("SliverDihedral/{check:?}{m}"),
                ValidationError::SliverDihedral { edge, check, cause },
            ));
        }
    }
    for source in mass_props_errors() {
        let l = match &source {
            MassPropsError::Face { source, .. } => label("VolumeUncomputable/Face", source),
            other => label("VolumeUncomputable", other),
        };
        s.push((l, ValidationError::VolumeUncomputable { solid, source }));
    }
    for finding in pcurve_mint_errors() {
        let l = match &finding {
            PcurveMintError::Certify { error, .. } => label("Pcurve/Certify", error),
            other => label("Pcurve", other),
        };
        s.push((l, ValidationError::Pcurve { finding }));
    }
    for contact in ring_contacts() {
        s.push((
            label("RingMeetsOuter", &contact),
            ValidationError::RingMeetsOuter {
                face,
                ring: loop_,
                contact,
            },
        ));
    }
    for contact in ring_pair_contacts() {
        s.push((
            label("RingMeetsRing", &contact),
            ValidationError::RingMeetsRing {
                face,
                ring: loop_,
                other: loop_,
                contact,
            },
        ));
    }
    for source in contain_errors() {
        s.push((
            path_label("RingNestingUndecided", &source),
            ValidationError::RingNestingUndecided {
                face,
                ring: loop_,
                source,
            },
        ));
    }
    // Check 10: both rendered magnitudes, the doubled count and the
    // negative one.
    for (winding, bounded) in [(1, 2), (0, -1)] {
        s.push((
            format!("ShellWinding/{bounded}"),
            ValidationError::ShellWinding {
                solid,
                shell: ShellKey::default(),
                winding,
                bounded,
            },
        ));
    }
    s.push((
        "SolidOuterShells".to_owned(),
        ValidationError::SolidOuterShells { solid, outer: 2 },
    ));
    s.push((
        "VolumeSignUnresolved".to_owned(),
        ValidationError::VolumeSignUnresolved {
            solid,
            limit: crate::props::SignLimit::Scalar,
            volume: Some((-1e-9, 1e-9)),
        },
    ));
    s.push((
        "ShellRoleUndecided".to_owned(),
        ValidationError::ShellRoleUndecided {
            solid,
            error: crate::ShellClassifyError::Straddles {
                shell: ShellKey::default(),
                low: geom_core::MarginDiag::value(-1e-12),
                high: geom_core::MarginDiag::value(1e-12),
            },
        },
    ));
    s.push((
        "OrientationUndecided".to_owned(),
        ValidationError::OrientationUndecided {
            solid,
            reading: crate::OrientationUnread::Volume {
                margin: None,
                aimed: 0,
                set_aside: Vec::new(),
            },
        },
    ));

    // Tier 3′: the census.
    for contact in census_contacts() {
        let witness = match contact {
            CensusContact::ConformalPatch { .. } => {
                crate::census::CONFORMAL_REGION_WITNESS.to_owned()
            }
            _ => "(0.4375, 0.5, 0.25)".to_owned(),
        };
        s.push((
            label("UnrecordedContact", &contact),
            ValidationError::UnrecordedContact {
                contact,
                witness,
                margin: Some(geom_core::MarginDiag::value(0.0)),
            },
        ));
    }
    for declaration in stale_declarations() {
        s.push((
            label("StaleContactDeclaration", &declaration),
            ValidationError::StaleContactDeclaration { declaration },
        ));
    }
    // Door 1's whole-surface verdict (a Rest record) and the tangent
    // line of a face-face record on two carriers (a Tangent record),
    // each.
    for (class, witness, predicate) in [
        (
            GlueClass::Rest,
            crate::census::CARRIER_COMPARISON_WITNESS,
            "carrier_sphere_radius",
        ),
        (
            GlueClass::Tangent,
            crate::census::TANGENT_LOCUS_WITNESS,
            "contact_tangent_opposed",
        ),
    ] {
        s.push((
            format!("ContactContradicted/{}", class.name()),
            ValidationError::ContactContradicted {
                declaration: DeclaredContact {
                    a: face,
                    b: face,
                    class,
                },
                witness: witness.to_owned(),
                margin: contradiction_margin(predicate),
            },
        ));
    }
    let mut causes: Vec<(String, CensusUnsupportedCause)> = Vec::new();
    for e in chart_region_errors() {
        causes.push((
            label("CensusUnsupported/ChartRegion", &e),
            CensusUnsupportedCause::ChartRegion(e),
        ));
    }
    for e in contact_refusals() {
        causes.push((
            label("CensusUnsupported/ContactLane", &e),
            CensusUnsupportedCause::ContactLane(e),
        ));
    }
    for e in contain_errors() {
        causes.push((
            path_label("CensusUnsupported/Containment", &e),
            CensusUnsupportedCause::Containment(e),
        ));
    }
    causes.push((
        "CensusUnsupported/FaceUnboundable".to_owned(),
        CensusUnsupportedCause::FaceUnboundable,
    ));
    for (l, cause) in causes {
        // The subject each cause is raised on: a face pair for the
        // chart-region and contact lanes, one face for the rest.
        let subject = match cause {
            CensusUnsupportedCause::ChartRegion(_) | CensusUnsupportedCause::ContactLane(_) => pair,
            CensusUnsupportedCause::Containment(_) | CensusUnsupportedCause::FaceUnboundable => {
                CensusSubject::Entity(EntityId::Face(face))
            }
        };
        s.push((l, ValidationError::CensusUnsupported { subject, cause }));
    }
    // Every `what` the backstop raises, on the pair kind its arm raises
    // it on (`Undecided` is their one source).
    let carried = crate::splitting::LoopDecision::ALL
        .map(|decision| Undecided::WitnessTooClose(Some(decision)));
    for why in Undecided::iter().chain(carried) {
        let (a, b) = if why.on_faces() {
            (EntityId::Face(face), EntityId::Face(face))
        } else {
            (EntityId::Solid(solid), EntityId::Solid(solid))
        };
        s.push((
            format!("CensusUndecidable/{why:?}"),
            ValidationError::CensusUndecidable {
                a,
                b,
                what: why.what(),
                reason: crate::validate::Undecidable::MayOverlap,
            },
        ));
    }
    for reason in [
        crate::validate::Undecidable::ThinInBand,
        crate::validate::Undecidable::MayOverlap,
    ] {
        s.push((
            format!("CensusUndecidable/unread/{reason:?}"),
            crate::census::unread_sample(face, face, reason),
        ));
    }
    s.push((
        "FaceThroughEdgeWedge".to_owned(),
        ValidationError::FaceThroughEdgeWedge {
            edge: EdgeKey::default(),
            face,
            witness: "(0.0, 0.0, 0.0)".to_owned(),
        },
    ));
    for why in crate::link::TouchUnread::iter() {
        s.push((
            format!("FaceThroughEdgeWedgeUndecided/{why:?}"),
            ValidationError::FaceThroughEdgeWedgeUndecided {
                edge: EdgeKey::default(),
                face,
                witness: "(0.0, 0.0, 0.0)".to_owned(),
                why,
            },
        ));
    }
    s.push((
        "CoincidentWedgesOverlap".to_owned(),
        ValidationError::CoincidentWedgesOverlap {
            edges: [EdgeKey::default(); 2],
            witness: "(0.0, 0.0, 0.0)".to_owned(),
        },
    ));
    // The reasons tier 3′'s line reading names: a sign in band, or an
    // edge's wedge with no reading.
    let line_unread = core::iter::once((crate::link::TouchUnread::InBand, None))
        .chain(crate::census::WEDGE_UNREAD.map(|why| (why, Some(EdgeKey::default()))));
    for (why, unread) in line_unread {
        s.push((
            format!("CoincidentWedgesOverlapUndecided/{why:?}"),
            ValidationError::CoincidentWedgesOverlapUndecided {
                edges: [EdgeKey::default(); 2],
                unread,
                witness: "(0.0, 0.0, 0.0)".to_owned(),
                why,
            },
        ));
    }
    s.push((
        "BoundaryCrossesItself".to_owned(),
        ValidationError::BoundaryCrossesItself {
            solid,
            witness: "(0.0, 0.0, 0.0)".to_owned(),
        },
    ));
    for why in crate::link::TouchUnread::iter() {
        s.push((
            format!("BoundaryCrossesItselfUndecided/{why:?}"),
            ValidationError::BoundaryCrossesItselfUndecided {
                solid,
                witness: "(0.0, 0.0, 0.0)".to_owned(),
                why,
            },
        ));
    }
    let ContainError::ArcEndClearOfCorners(gaps) = arc_end_clear_of_corners(true) else {
        unreachable!("the sample builds the arc-end finding")
    };
    s.push((
        "ArcEndOffCorner".to_owned(),
        ValidationError::ArcEndOffCorner { face, gaps },
    ));
    s
}

/// The variants of `E` no sample in `seen` carries, as `name::Kind`.
/// The roster is `E`'s compiler-derived discriminant enum, so a variant
/// added upstream is a gap here until a sample carries it — no list in
/// this file names the variants.
#[cfg(test)]
fn gaps<E, K>(name: &str, seen: &[E]) -> Vec<String>
where
    K: strum::IntoEnumIterator + PartialEq + core::fmt::Debug + for<'a> From<&'a E>,
{
    K::iter()
        .filter(|kind| !seen.iter().any(|e| K::from(e) == *kind))
        .map(|kind| format!("{name}::{kind:?}"))
        .collect()
}

/// The nested variants no sample carries — empty when every variant of
/// every enum [`validation_error_samples`] renders is sampled.
#[cfg(test)]
pub(crate) fn nested_coverage_gaps() -> Vec<String> {
    use crate::boolean::ContainErrorKind;
    use crate::chart_region::ChartRegionErrorKind;
    use crate::contact::ContactRefusalKind;
    use crate::pcurves::PcurveMintErrorKind;
    use crate::props::MassPropsErrorKind;
    use crate::validate::{
        CensusContactKind, CensusUnsupportedCauseKind, RingContactKind, RingPairContactKind,
        StaleDeclarationKind,
    };
    use geom_brep::certify::CertifyErrorKind;
    use geom_brep::edge_nurbs::PlaneNurbsRefusalKind;
    use geom_brep::offset_fit::OffsetFitErrorKind;
    use geom_brep::pcurve_cache::PcurveCertifyErrorKind;
    use geom_brep::props::PropsErrorKind;
    use geom_core::MarginKind;
    use geom_core::predicate::BandErrorKind;
    let causes: Vec<CensusUnsupportedCause> = validation_error_samples()
        .into_iter()
        .filter_map(|(_, e)| match e {
            ValidationError::CensusUnsupported { cause, .. } => Some(cause),
            _ => None,
        })
        .collect();
    let margins: Vec<MarginDiag> = diags().iter().map(|d| d.margin).collect();
    let mut out = Vec::new();
    out.extend(gaps::<_, CensusUnsupportedCauseKind>(
        "CensusUnsupportedCause",
        &causes,
    ));
    out.extend(gaps::<_, ChartRegionErrorKind>(
        "ChartRegionError",
        &chart_region_errors(),
    ));
    out.extend(gaps::<_, ContactRefusalKind>(
        "ContactRefusal",
        &contact_refusals(),
    ));
    out.extend(gaps::<_, ContainErrorKind>(
        "ContainError",
        &contain_errors(),
    ));
    let carried: Vec<crate::boolean::PointInSolidError> = contain_errors()
        .into_iter()
        .filter_map(|e| match e {
            ContainError::Curved(e) => Some(e),
            _ => None,
        })
        .collect();
    out.extend(gaps::<
        _,
        crate::boolean::solid_contain::PointInSolidErrorKind,
    >("PointInSolidError", &carried));
    let loops: Vec<crate::splitting::PointInLoopError> = carried
        .into_iter()
        .filter_map(|e| match e {
            crate::boolean::PointInSolidError::Loop(e) => Some(e),
            _ => None,
        })
        .collect();
    out.extend(
        gaps::<_, crate::splitting::containment::PointInLoopErrorKind>("PointInLoopError", &loops),
    );
    out.extend(gaps::<_, CertifyErrorKind>(
        "CertifyError",
        &certify_errors(),
    ));
    out.extend(gaps::<_, PlaneNurbsRefusalKind>(
        "PlaneNurbsRefusal",
        &plane_nurbs_refusals(),
    ));
    let arc_rims: Vec<geom_brep::ArcRimRefusal> = plane_nurbs_refusals()
        .into_iter()
        .filter_map(|e| match e {
            PlaneNurbsRefusal::ArcRim(r) => Some(r),
            _ => None,
        })
        .collect();
    out.extend(gaps::<_, geom_brep::edge_nurbs::ArcRimRefusalKind>(
        "ArcRimRefusal",
        &arc_rims,
    ));
    out.extend(gaps::<_, geom_brep::edge_nurbs::AnalyticRung3RefusalKind>(
        "AnalyticRung3Refusal",
        &analytic_rung3_refusals(),
    ));
    out.extend(gaps::<_, PcurveMintErrorKind>(
        "PcurveMintError",
        &pcurve_mint_errors(),
    ));
    out.extend(gaps::<_, PcurveCertifyErrorKind>(
        "PcurveCertifyError",
        &pcurve_certify_errors(),
    ));
    out.extend(gaps::<_, MassPropsErrorKind>(
        "MassPropsError",
        &mass_props_errors(),
    ));
    out.extend(gaps::<_, PropsErrorKind>("PropsError", &props_errors()));
    out.extend(gaps::<_, OffsetFitErrorKind>(
        "OffsetFitError",
        &offset_fit_errors(),
    ));
    out.extend(gaps::<_, BandErrorKind>("BandError", &band_errors()));
    out.extend(gaps::<_, MarginKind>("MarginDiag", &margins));
    out.extend(gaps::<_, CensusContactKind>(
        "CensusContact",
        &census_contacts(),
    ));
    out.extend(gaps::<_, StaleDeclarationKind>(
        "StaleDeclaration",
        &stale_declarations(),
    ));
    out.extend(gaps::<_, RingContactKind>("RingContact", &ring_contacts()));
    out.extend(gaps::<_, RingPairContactKind>(
        "RingPairContact",
        &ring_pair_contacts(),
    ));
    out
}

#[cfg(test)]
mod tests {
    use geom_brep::recourse::Reading;
    use geom_core::FileCoincidence;

    /// **No sample renders a run of spaces or a line break**: the marks
    /// of a string literal continued onto its next line without its `\`,
    /// which keeps the line break and the next line's indentation, one
    /// space or many, in the text. Read over every tier-3 finding and
    /// every measurement refusal, so over every `PropsError`
    /// (`MassPropsError::Face`'s own words), whose words tier 3 does not
    /// render.
    #[test]
    fn no_sample_renders_a_run_of_spaces() {
        let rendered = super::validation_error_samples()
            .into_iter()
            .map(|(label, e)| (label, e.to_string()))
            .chain(
                super::mass_props_errors()
                    .into_iter()
                    .map(|e| (format!("{e:?}"), e.to_string())),
            );
        let problems: Vec<_> = rendered
            .filter(|(_, text)| text.contains("  ") || text.contains('\n'))
            .map(|(label, text)| format!("{label}: {text:?}"))
            .collect();
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }

    /// **Every certification refusal ends alike at rest and at the import
    /// door but for the file's ε_in and its culprit** (D4 ¶1: one
    /// recourse per decision, wherever it is read; (iv)): the door's
    /// ending departs from the at-rest one only through its ε_in
    /// sentences, which name ε_in, and where a defect ending at rest names
    /// the kernel or the file, which at the door names the file
    /// ([`geom_core::DAMAGED_FILE_ENDING`]). The roster is
    /// [`super::certify_errors`], which `nested_coverage_gaps` holds to
    /// every `CertifyError`, `PlaneNurbsRefusal` and `AnalyticRung3Refusal`
    /// variant. Red where the door's ending departs from the at-rest one
    /// any other way, the door naming the kernel included, or where no
    /// refusal reaches the file's ending.
    #[test]
    fn every_certify_refusal_ends_alike_at_rest_and_at_the_import_door() {
        use geom_core::{DAMAGED_FILE_ENDING, KERNEL_OR_FILE_DEFECT_ENDING};

        let file = FileCoincidence::new(1e-6);
        // Each refusal's ending at rest, and at the door where it is
        // read past ε_in (`None` where the door reads it within).
        let errors = super::certify_errors();
        let endings: Vec<_> = errors
            .iter()
            .filter_map(|e| {
                let door = e.ending(file);
                let within = door.as_deref().is_some_and(|d| d.contains("ε_in"));
                (!within).then(|| (e, e.ending(Reading::AtRest), door))
            })
            .collect();
        let split: Vec<String> = endings
            .iter()
            .filter(|(_, rest, door)| {
                let want = match rest.as_deref() {
                    Some(KERNEL_OR_FILE_DEFECT_ENDING) => Some(DAMAGED_FILE_ENDING.to_owned()),
                    _ => rest.clone(),
                };
                *door != want
            })
            .map(|(e, ..)| format!("{e:?}"))
            .collect();
        assert!(
            split.is_empty(),
            "the doors end apart, beyond the file's ε_in and its culprit: {split:#?}"
        );
        assert!(
            endings
                .iter()
                .any(|(_, rest, _)| rest.as_deref() == Some(KERNEL_OR_FILE_DEFECT_ENDING)),
            "no refusal in the roster reads a defect at rest"
        );
    }

    /// **A shell-classify defect read at the import door names the file**
    /// ([`crate::ShellClassifyError::recourse`]): the door it is given is
    /// the door its defect ending reads, at rest the kernel or the file
    /// and at the import door the file. Red if the classifier reads the
    /// import door as at rest.
    #[test]
    fn a_shell_classify_defect_names_the_file_at_the_import_door() {
        use geom_core::{DAMAGED_FILE_ENDING, KERNEL_OR_FILE_DEFECT_ENDING, MarginDiag};

        let refusal = crate::ShellClassifyError::Straddles {
            shell: crate::ShellKey::default(),
            low: MarginDiag::value(-1.0),
            high: MarginDiag::value(1.0),
        };
        assert_eq!(
            refusal.recourse(Reading::AtRest).as_deref(),
            Some(KERNEL_OR_FILE_DEFECT_ENDING),
            "at rest"
        );
        assert_eq!(
            refusal.recourse(FileCoincidence::new(1e-6)).as_deref(),
            Some(DAMAGED_FILE_ENDING),
            "at the import door"
        );
    }
}
