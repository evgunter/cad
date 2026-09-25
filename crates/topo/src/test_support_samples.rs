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
//! every [`CensusContact`], [`StaleDeclaration`] and [`RingContact`];
//! and every [`Undecided`] reason, which is every `what` the
//! cross-solid backstop can raise. One level further in, the enums
//! `PcurveMintError::Certify`, `MassPropsError::Face` and
//! `CertifyError::PlaneNurbs` carry are sampled whole too; the four
//! `OffsetFitError` wrappers carry one value each.
//!
//! Where a raise site fills a field with prose — a `what`, a `detail`,
//! a steer — the sample carries the prose a real run renders, and a
//! field with several real values is sampled at each one this module
//! names. Arena keys are the default (null) keys.
#![allow(clippy::expect_used)] // one fixed band, well-formed by construction

use geom_brep::MaterialWedge;
use geom_brep::certify::{CertCheck, CertifyError};
use geom_brep::edge_nurbs::PlaneNurbsRefusal;
use geom_brep::offset_fit::{OffsetFitError, OffsetLimb};
use geom_brep::pcurve_cache::{FittedMagnitude, PcurveCertifyError, PcurveCheck};
use geom_brep::props::PropsError;
use geom_core::{Band, BandError, BandField, Indeterminate, MarginDiag};
use strum::IntoEnumIterator as _;

use crate::boolean::ContainError;
use crate::census::Undecided;
use crate::chart_region::ChartRegionError;
use crate::contact::{
    ContactClass, ContactFinding, ContactRefusal, ContactVerdict, DeclaredContact, FIT_DEFERRAL,
};
use crate::entity::{
    EdgeKey, EntityId, FaceKey, GeomRef, HalfEdgeKey, LoopKey, ShellKey, SolidKey, VertexKey,
};
use crate::geometry::{CurveKey, PointKey};
use crate::pcurves::PcurveMintError;
use crate::props::MassPropsError;
use crate::validate::{
    CensusContact, CensusSubject, CensusUnsupportedCause, RingContact, StaleDeclaration,
    ValidationError,
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
    };
    [
        with(MarginDiag::Value(5e-9)),
        with(MarginDiag::Enclosure {
            lo: -2e-9,
            hi: 4e-9,
        }),
        with(MarginDiag::Invalid),
    ]
}

fn diag() -> Indeterminate {
    diags()[0]
}

/// The margin every `ContactRefusal::Contradicted` raise site carries:
/// invalid, naming the contact predicate that decided.
fn contradiction_margin(predicate: &'static str) -> Indeterminate {
    Indeterminate {
        margin: MarginDiag::Invalid,
        band: band(),
        predicate: Some(predicate),
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
        ChartRegionError::WitnessBudgetExhausted {
            segments: crate::chart_region::WITNESS_BUDGET.segments + 1,
            cells: 0,
        },
        ChartRegionError::Corrupt,
    ];
    v.push(ChartRegionError::ChartDivergence {
        detail: "same GeomSource with non-bit-identical descriptions — the same-source \
                 theorem violated (forged or corrupted source attachment)",
    });
    v.extend(diags().into_iter().map(ChartRegionError::Escalated));
    v
}

fn contact_refusals() -> Vec<ContactRefusal> {
    let mut v = vec![
        ContactRefusal::Contradicted {
            diag: contradiction_margin("contact_rest_senses_opposed"),
            steer: None,
        },
        ContactRefusal::Contradicted {
            diag: contradiction_margin("carrier_cyl_radius"),
            steer: Some(FIT_DEFERRAL),
        },
        ContactRefusal::Escalated { diag: diag() },
        ContactRefusal::Undeclared { diag: diag() },
    ];
    // The longest `what`s the raise sites write.
    v.extend(
        [
            "a declared face's surface kind is outside the Rest ladder's inventory \
             (plane, sphere, cylinder)",
            "the (carrier kind, surface-kind pair) triple is outside the jet \
             certificate's span-bound lane (the order-k boundary)",
        ]
        .map(|what| ContactRefusal::NotCertifiable { what }),
    );
    v
}

fn contain_errors() -> Vec<ContainError> {
    vec![
        ContainError::Escalated(diag()),
        ContainError::RayExhausted,
        ContainError::Corrupt,
        ContainError::ArcLoopUnsupported {
            r#loop: LoopKey::default(),
        },
    ]
}

fn plane_nurbs_refusals() -> Vec<PlaneNurbsRefusal> {
    vec![
        PlaneNurbsRefusal::FootPointInconclusive {
            sample: 3,
            last_distance: 1e-7,
        },
        PlaneNurbsRefusal::NotTransverse { sample: 3 },
        PlaneNurbsRefusal::PcurveFit,
        PlaneNurbsRefusal::Limb {
            limb: geom_brep::SsiLimb::Tube,
            value: 1e-7,
        },
        PlaneNurbsRefusal::TubeStraddles {
            certified_clearance: 1e-7,
            boxes: 12,
        },
        PlaneNurbsRefusal::Escalated(diag()),
        PlaneNurbsRefusal::Unsupported {
            what: "a rational NURBS surface",
        },
    ]
}

fn certify_errors() -> Vec<CertifyError> {
    let key = geom_brep::SurfaceKey::default();
    let mut v = vec![
        CertifyError::ChartImageUnavailable {
            chart: "cone",
            carrier: "ellipse",
        },
        CertifyError::UnresolvedSurface { key },
        CertifyError::Unimplemented,
        CertifyError::IntersectionSameSurface { key },
        CertifyError::SeamOnNonPeriodic,
        CertifyError::IntervalNotForward,
        CertifyError::WindingExceeded,
        CertifyError::ResidualExceeded {
            check: CertCheck::Surface1Residual,
            sample: 4,
        },
        CertifyError::NotTransverse { sample: 4 },
        CertifyError::NotSecondOrderSeparated {
            sample: 4,
            band: band(),
        },
        CertifyError::TangentCertificateUnsupported,
        CertifyError::Escalated {
            check: CertCheck::Transversality,
            sample: 4,
            cause: diag(),
        },
    ];
    v.extend(band_errors().into_iter().map(CertifyError::Band));
    v.extend(
        plane_nurbs_refusals()
            .into_iter()
            .map(CertifyError::PlaneNurbs),
    );
    v
}

fn pcurve_certify_errors() -> Vec<PcurveCertifyError> {
    let mut v = vec![
        PcurveCertifyError::UnsupportedChart { chart: "torus" },
        PcurveCertifyError::UnsupportedCarrier,
        PcurveCertifyError::FittedLaneUnsupported { scalar: "Dual64" },
        PcurveCertifyError::FittedMateMissing,
        PcurveCertifyError::IsoUnsupported {
            what: "a rational NURBS surface",
        },
        PcurveCertifyError::ChartRow {
            source: geom_core::spline::SplineError::DomainInvalid { lo: 1.0, hi: 0.0 },
        },
        PcurveCertifyError::FittedCertificate {
            limb: Some(geom_brep::SsiLimb::Tube),
            what: "the uniqueness tube straddles a second branch",
            magnitude: Some(FittedMagnitude::CertifiedClearance {
                certified_clearance: 1e-7,
                boxes: 12,
            }),
        },
        PcurveCertifyError::FittedEscalated { cause: diag() },
        PcurveCertifyError::IntervalNotForward,
        PcurveCertifyError::ChartWindingUnsupported,
        PcurveCertifyError::AzimuthPeriodExceeded,
        PcurveCertifyError::ResidualExceeded {
            check: PcurveCheck::MapResidual,
            sample: 4,
        },
        PcurveCertifyError::TrimEscape,
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
        PcurveMintError::Corrupt,
        PcurveMintError::LoopDiscontinuity { half_edge },
        PcurveMintError::LoopNotClosed { face },
        PcurveMintError::SingularChartJoint {
            face,
            r#loop,
            half_edge,
        },
        PcurveMintError::OuterSpansPeriod,
        PcurveMintError::LoopWraps { face, r#loop },
        PcurveMintError::MissingCache { half_edge },
        PcurveMintError::Escalated {
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
        PropsError::NotOneChartBranch {
            edge: 2,
            what: "the edge crosses the seam",
        },
        PropsError::DegenerateFace,
        PropsError::Escalated { cause: diag() },
        PropsError::QuadratureBudget {
            width_len: 1e-6,
            target_len: 1e-9,
            rounds: 12,
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
    ]);
    v.extend(
        props_errors()
            .into_iter()
            .map(|source| MassPropsError::Face { face, source }),
    );
    v
}

fn offset_fit_errors() -> Vec<OffsetFitError> {
    use geom_brep::offset_meters::MeterError;
    vec![
        OffsetFitError::Meter(MeterError::NormalFloor {
            floor: 1e-9,
            thinness: 1e-3,
            speed_lever: 2.0,
        }),
        OffsetFitError::Meter(MeterError::CurvatureHeadroom {
            reach: 0.5,
            headroom: -0.1,
            kappa: (2.0, 0.5),
        }),
        OffsetFitError::Meter(MeterError::Escalated { source: diag() }),
        OffsetFitError::PatchBound(geom_brep::patch_bound::PatchBoundError::Crease),
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
        OffsetFitError::BudgetExhausted {
            budget: 4096,
            grid: (64, 64),
            achieved: 3e-6,
            tolerance: 1e-6,
        },
        OffsetFitError::SampleCapReached {
            cap: 4096,
            rounds: 6,
            grid: (64, 64),
            achieved: 3e-6,
            tolerance: 1e-6,
        },
        OffsetFitError::BoundNotFinite {
            rounds: 6,
            grid: (64, 64),
            d: 0.1,
            tolerance: 1e-6,
            last_finite: Some(3e-6),
        },
        OffsetFitError::RefinementStalled {
            rounds: 6,
            grid: (64, 64),
            achieved: 3e-6,
            tolerance: 1e-6,
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
    ]
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
        CensusContact::EdgeEdgeCross { a: edge, b: edge },
        CensusContact::EdgeEdgeOverlap { a: edge, b: edge },
        CensusContact::EdgeFaceOverlap { edge, face },
        CensusContact::ConformalPatch {
            finding: ContactFinding {
                pair: DeclaredContact {
                    a: face,
                    b: face,
                    class: ContactClass::Rest,
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
        StaleDeclaration::CurveLocus {
            face_a: face,
            face_b: face,
            witness: EdgeKey::default(),
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

/// `arm/Variant`, the variant read off the nested value's `Debug`.
fn label<T: core::fmt::Debug>(arm: &str, nested: &T) -> String {
    let debug = format!("{nested:?}");
    let head: String = debug
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    format!("{arm}/{head}")
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
        ValidationError::NullEdgeAtRest { edge },
        ValidationError::NullFaceAtRest { face },
        // Tier 3 arms that carry nothing nested.
        ValidationError::UncertifiableSurface { face },
        ValidationError::PoisonedSurfaceDescription { face },
        ValidationError::ApproxLaneUnsupported { face },
        ValidationError::DegenerateTorus { face },
        ValidationError::DescriptionNotAdjacent { edge },
        ValidationError::PlanarFaceResidual { face, vertex },
        ValidationError::PlanarBoundaryResidual { face, edge },
        ValidationError::TransverseNotIntrinsic { edge },
        ValidationError::ScaffoldAtRest { edge },
        ValidationError::TangentNotIntrinsic { edge },
        ValidationError::LaminaWedge { edge },
        ValidationError::LoopRoleInverted {
            face,
            r#loop: loop_,
        },
        ValidationError::CurvedSenseInverted { face },
        ValidationError::NegativeVolume { solid },
        ValidationError::RingOutsideOuter {
            face,
            ring: loop_,
            ring_vertex: vertex,
        },
        ValidationError::CensusLaneUnsupported { subject: pair },
        ValidationError::InstanceInterference {
            outer: solid,
            inner: solid,
            witness: vertex,
        },
    ] {
        s.push((label("", &e).trim_start_matches('/').to_owned(), e));
    }

    // A surface datum, poisoned or outside its range: every datum, at
    // each end of the range.
    for datum in geom::SurfaceDatum::iter() {
        let kind = geom_brep::SurfaceKind::Torus;
        s.push((
            label("PoisonedSurfaceDatum", &datum),
            ValidationError::PoisonedSurfaceDatum { face, kind, datum },
        ));
        for end in geom::ConventionEnd::iter() {
            s.push((
                label("UnrepresentableSurfaceDatum", &datum),
                ValidationError::UnrepresentableSurfaceDatum {
                    face,
                    kind,
                    datum,
                    end,
                },
            ));
        }
    }

    // A carrier datum, poisoned or outside its range: every datum, at
    // each end of the range.
    for datum in geom::CurveDatum::iter() {
        let kind = crate::query::CurveKind::Ellipse;
        s.push((
            label("PoisonedCurveDatum", &datum),
            ValidationError::PoisonedCurveDatum { edge, kind, datum },
        ));
        for end in geom::ConventionEnd::iter() {
            s.push((
                label("UnrepresentableCurveDatum", &datum),
                ValidationError::UnrepresentableCurveDatum {
                    edge,
                    kind,
                    datum,
                    end,
                },
            ));
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
                "SliverDihedral",
                ValidationError::SliverDihedral { edge, cause },
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
                "CensusEscalated",
                ValidationError::CensusEscalated { cause },
            ),
        ] {
            s.push((format!("{arm}{m}"), e));
        }
    }
    for wedge in [MaterialWedge::Cusp, MaterialWedge::Slit] {
        s.push((
            label("UndeclaredCusp", &wedge),
            ValidationError::UndeclaredCusp { edge, wedge },
        ));
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
    for source in contain_errors() {
        s.push((
            label("RingNestingUndecided", &source),
            ValidationError::RingNestingUndecided {
                face,
                ring: loop_,
                source,
            },
        ));
    }

    // Tier 3′: the census.
    for contact in census_contacts() {
        let witness = match contact {
            CensusContact::ConformalPatch { .. } => {
                crate::census::CONFORMAL_REGION_WITNESS.to_owned()
            }
            // The crossing arm's real shape: the position, then the side
            // verdict after " — ", which the message does not render.
            CensusContact::EdgeEdgeCross { .. } => {
                "(0.4375, 0.5, 0.25) — side verdict: same-side".to_owned()
            }
            _ => "(0.4375, 0.5, 0.25)".to_owned(),
        };
        s.push((
            label("UndeclaredContact", &contact),
            ValidationError::UndeclaredContact { contact, witness },
        ));
    }
    for declaration in stale_declarations() {
        s.push((
            label("StaleContactDeclaration", &declaration),
            ValidationError::StaleContactDeclaration { declaration },
        ));
    }
    // Door 1's whole-surface verdict (a Rest record) and the curve
    // record's witnessing edge (a Tangent record), each with and
    // without the designed-clearance steer.
    for (class, witness, predicate) in [
        (
            ContactClass::Rest,
            crate::census::CARRIER_COMPARISON_WITNESS,
            "carrier_sphere_radius",
        ),
        (
            ContactClass::Tangent,
            crate::census::CURVE_RECORD_WITNESS,
            "contact_tangent_opposed",
        ),
    ] {
        for steer in [None, Some(FIT_DEFERRAL)] {
            s.push((
                format!(
                    "ContactContradicted/{}{}",
                    class.name(),
                    if steer.is_some() { "/steered" } else { "" }
                ),
                ValidationError::ContactContradicted {
                    declaration: DeclaredContact {
                        a: face,
                        b: face,
                        class,
                    },
                    witness: witness.to_owned(),
                    margin: contradiction_margin(predicate),
                    steer,
                },
            ));
        }
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
            label("CensusUnsupported/Containment", &e),
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
    for why in Undecided::iter() {
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
            },
        ));
    }
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
        CensusContactKind, CensusUnsupportedCauseKind, RingContactKind, StaleDeclarationKind,
    };
    use geom_brep::certify::CertifyErrorKind;
    use geom_brep::edge_nurbs::PlaneNurbsRefusalKind;
    use geom_brep::offset_fit::OffsetFitErrorKind;
    use geom_brep::pcurve_cache::PcurveCertifyErrorKind;
    use geom_brep::props::PropsErrorKind;
    use geom_core::predicate::{BandErrorKind, MarginDiagKind};
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
    out.extend(gaps::<_, CertifyErrorKind>(
        "CertifyError",
        &certify_errors(),
    ));
    out.extend(gaps::<_, PlaneNurbsRefusalKind>(
        "PlaneNurbsRefusal",
        &plane_nurbs_refusals(),
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
    out.extend(gaps::<_, MarginDiagKind>("MarginDiag", &margins));
    out.extend(gaps::<_, CensusContactKind>(
        "CensusContact",
        &census_contacts(),
    ));
    out.extend(gaps::<_, StaleDeclarationKind>(
        "StaleDeclaration",
        &stale_declarations(),
    ));
    out.extend(gaps::<_, RingContactKind>("RingContact", &ring_contacts()));
    out
}
