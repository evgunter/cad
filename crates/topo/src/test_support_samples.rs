//! **One [`ValidationError`] of every arm, on a representative
//! payload**: the samples two rows read.
//!
//! - This crate's `errors_display_without_panicking` indexes them by
//!   the enum's compiler-derived companion, so an arm added without a
//!   sample here reds by name.
//! - `editor-core`'s `refusal_concision_at_rest` renders each one the
//!   way the viewer's at-rest badge draws a finding and holds it to the
//!   refusal word budget — which is why the list lives behind the
//!   `test_support` door rather than in `crate::fixtures`, which no
//!   other crate can name ([`crate::test_support_impl`] states the
//!   routing rule).
//!
//! **Representative, not minimal**: where a raise site fills a field
//! with prose — a `what`, a `steer`, a region witness — the sample
//! carries the prose a real run renders, and an arm raised with more
//! than one such string has a sample per string, because the budget
//! is a claim about what a reader sees. Arena keys are the default
//! (null) keys: a key renders as one word whatever its index.
#![allow(clippy::expect_used)] // two fixed bands, well-formed and inverted by construction

use geom_brep::MaterialWedge;
use geom_core::{Band, Indeterminate, MarginDiag};

use crate::census::{CARRIER_COMPARISON_WITNESS, CONFORMAL_REGION_WITNESS};
use crate::chart_region::ChartRegionError;
use crate::contact::{ContactClass, ContactFinding, ContactRefusal, ContactVerdict};
use crate::contact::{DeclaredContact, FIT_DEFERRAL};
use crate::entity::{
    EdgeKey, EntityId, FaceKey, GeomRef, HalfEdgeKey, LoopKey, ShellKey, SolidKey, VertexKey,
};
use crate::geometry::{CurveKey, PointKey};
use crate::validate::{
    CensusContact, CensusSubject, CensusUnsupportedCause, RingContact, StaleDeclaration,
    ValidationError,
};

/// A census or tier-3 escalation's diagnostic, as a run renders it: a
/// margin, the band it fell in, and the predicate that asked.
fn indeterminate() -> Indeterminate {
    Indeterminate {
        margin: MarginDiag::Value(5e-9),
        band: Band::new(1e-9, 1e-8).expect("a well-formed band"),
        predicate: Some("side_of_plane"),
    }
}

/// Every [`ValidationError`] arm at least once, on a representative
/// payload (the module docs).
#[must_use]
pub fn validation_error_samples() -> Vec<ValidationError> {
    let face = FaceKey::default();
    let other_face = FaceKey::default();
    let edge = EdgeKey::default();
    let vertex = VertexKey::default();
    let he = HalfEdgeKey::default();
    let loop_ = LoopKey::default();
    let shell = ShellKey::default();
    let solid = SolidKey::default();
    let point = PointKey::default();
    let band_error = || Band::new(1.0, 0.0).expect_err("an inverted band");
    let declared = |class| DeclaredContact {
        a: face,
        b: other_face,
        class,
    };
    let witness = "(0.4375, 0.5, 0.25)".to_owned();
    let mut samples = vec![
        ValidationError::Band {
            error: band_error(),
        },
        ValidationError::DanglingDescription {
            from: GeomRef::Point(point),
            to: GeomRef::Point(point),
        },
        ValidationError::UncertifiableSurface { face },
        ValidationError::PoisonedSurfaceDescription { face },
        ValidationError::ApproxCertification {
            face,
            error: geom_brep::OffsetFitError::InvalidRequest {
                d: 0.0,
                tolerance: 0.0,
            },
        },
        ValidationError::ApproxLaneUnsupported { face },
        ValidationError::DegenerateTorus { face },
        ValidationError::DegenerateTorusEscalated {
            face,
            cause: indeterminate(),
        },
        ValidationError::NonpositiveTorusTube { face },
        ValidationError::EdgeCertification {
            edge,
            error: geom_brep::CertifyError::Unimplemented,
        },
        ValidationError::DescriptionNotAdjacent { edge },
        ValidationError::PlanarFaceResidual { face, vertex },
        ValidationError::PlanarFaceEscalated {
            face,
            vertex,
            cause: indeterminate(),
        },
        ValidationError::PlanarBoundaryResidual { face, edge },
        ValidationError::PlanarBoundaryEscalated {
            face,
            edge,
            cause: indeterminate(),
        },
        ValidationError::SliverDihedral {
            edge,
            cause: indeterminate(),
        },
        ValidationError::TransverseNotIntrinsic { edge },
        ValidationError::ScaffoldAtRest { edge },
        ValidationError::TangentNotIntrinsic { edge },
        ValidationError::UndeclaredCusp {
            edge,
            wedge: MaterialWedge::Cusp,
        },
        ValidationError::UndeclaredCusp {
            edge,
            wedge: MaterialWedge::Slit,
        },
        ValidationError::LaminaWedge { edge },
        ValidationError::LoopRoleInverted {
            face,
            r#loop: loop_,
        },
        ValidationError::CurvedSenseInverted { face },
        ValidationError::NegativeVolume { solid },
        ValidationError::VolumeUncomputable {
            solid,
            source: crate::props::MassPropsError::Band {
                error: band_error(),
            },
        },
        ValidationError::Pcurve {
            finding: crate::pcurves::PcurveMintError::Corrupt,
        },
        ValidationError::RingMeetsOuter {
            face,
            ring: loop_,
            contact: RingContact::Vertex {
                ring_vertex: vertex,
                outer_vertex: vertex,
            },
        },
        ValidationError::RingMeetsOuter {
            face,
            ring: loop_,
            contact: RingContact::VertexOnEdge {
                ring_vertex: vertex,
                outer_edge: edge,
            },
        },
        ValidationError::RingMeetsOuter {
            face,
            ring: loop_,
            contact: RingContact::Edge {
                ring_edge: edge,
                outer_edge: edge,
            },
        },
        ValidationError::RingContactEscalated {
            face,
            ring: loop_,
            source: indeterminate(),
        },
        ValidationError::RingOutsideOuter {
            face,
            ring: loop_,
            ring_vertex: vertex,
        },
        ValidationError::RingNestingUndecided {
            face,
            ring: loop_,
            source: crate::boolean::ContainError::Escalated(indeterminate()),
        },
        ValidationError::UndeclaredContact {
            contact: CensusContact::EdgeFacePierce { edge, face },
            witness: witness.clone(),
        },
        ValidationError::UndeclaredContact {
            contact: CensusContact::ConformalPatch {
                finding: ContactFinding {
                    pair: declared(ContactClass::Rest),
                    verdict: ContactVerdict::Definite,
                },
            },
            witness: CONFORMAL_REGION_WITNESS.to_owned(),
        },
        ValidationError::StaleContactDeclaration {
            declaration: StaleDeclaration::VertexVertex {
                a: vertex,
                b: vertex,
            },
        },
        ValidationError::ContactContradicted {
            declaration: declared(ContactClass::Rest),
            witness: CARRIER_COMPARISON_WITNESS.to_owned(),
            margin: indeterminate(),
            steer: None,
        },
        ValidationError::ContactContradicted {
            declaration: declared(ContactClass::Tangent),
            witness: witness.clone(),
            margin: indeterminate(),
            steer: Some(FIT_DEFERRAL),
        },
        ValidationError::CensusEscalated {
            cause: indeterminate(),
        },
        // Three causes, not one three times: an arm whose cause is only
        // ever the chart-region one would leave the other two
        // composition paths unrendered. The segment figure is derived
        // from the cap it is one past, never restated.
        ValidationError::CensusUnsupported {
            subject: CensusSubject::FacePair(face, other_face),
            cause: CensusUnsupportedCause::ChartRegion(ChartRegionError::WitnessBudgetExhausted {
                segments: crate::chart_region::WITNESS_BUDGET.segments + 1,
                cells: 0,
            }),
        },
        ValidationError::CensusUnsupported {
            subject: CensusSubject::Entity(EntityId::Face(face)),
            cause: CensusUnsupportedCause::FaceUnboundable,
        },
        ValidationError::CensusUnsupported {
            subject: CensusSubject::Entity(EntityId::Edge(edge)),
            cause: CensusUnsupportedCause::ContactLane(ContactRefusal::NotCertifiable {
                what: "a declared face's surface kind is outside the Rest ladder's \
                       inventory (plane, sphere, cylinder)",
            }),
        },
        ValidationError::CensusLaneUnsupported {
            subject: CensusSubject::FacePair(face, other_face),
        },
        ValidationError::InstanceInterference {
            outer: solid,
            inner: solid,
            witness: vertex,
        },
        ValidationError::DanglingTopology {
            from: EntityId::Solid(solid),
            to: EntityId::Shell(shell),
        },
        ValidationError::DanglingGeometry {
            from: EntityId::Vertex(vertex),
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
            owner: EntityId::Face(other_face),
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
    ];
    // Every `what` a raise site writes, each its own sample: the arm's
    // length is the literal's plus this.
    samples.extend(crate::census::UNDECIDABLE_WHATS.iter().map(|&what| {
        ValidationError::CensusUndecidable {
            a: EntityId::Face(face),
            b: EntityId::Face(other_face),
            what,
        }
    }));
    samples
}
