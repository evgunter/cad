//! Which two solids a census finding is about.

use geom_core::Real;

use crate::body::Body;
use crate::entity::SolidKey;
use crate::validate::ValidationError;

/// **The two solids a census finding is about**, smaller key first:
/// the pair an interference verdict decided, the two entities an
/// undecidable verdict names, the two cells of an unrecorded touch, the
/// two edges of a coincident-wedges verdict, the edge and face of a
/// face-through-wedge verdict, the one solid (twice) of a solid
/// crossing itself, the two faces of a contradicted
/// declaration or of a pair the census had no lane for.
///
/// `None` for a finding that names no pair of solids: an escalation,
/// which names no entity at all
/// (`work/contacthold/an-escalated-census-finding-names-no-pair.md`), a
/// stale record, a lane subject that is one entity, a touch with a cell
/// of an input or a tool, and every finding about one body's own
/// topology. The match is exhaustive, so a reader that sorts findings
/// by solid never keeps one it could not place by accident: a finding
/// added to [`ValidationError`] fails to compile here until it is
/// placed.
pub fn census_finding_solids<T: Real>(
    body: &Body<T>,
    error: &ValidationError,
) -> Option<(SolidKey, SolidKey)> {
    use ValidationError as E;
    let edge = |e| crate::census_cell_solid(body, crate::Cell::Edge(e));
    let face = |f| body.solid_of_face(f);
    let (x, y) = match error {
        E::InstanceInterference { outer, inner, .. } => (*outer, *inner),
        E::SameSideInterference { a, b } => (face(*a)?, face(*b)?),
        E::TouchInterference { a, b, .. } => (
            crate::census_cell_solid(body, *a)?,
            crate::census_cell_solid(body, *b)?,
        ),
        E::CensusUndecidable { a, b, .. } => (
            crate::census_entity_solid(body, *a)?,
            crate::census_entity_solid(body, *b)?,
        ),
        // A pierce, or a crossing whose side test read `SameSide`, by
        // its own cells: two solids', or one solid's crossing itself.
        E::UnrecordedContact {
            contact: crate::CensusContact::EdgeFacePierce { edge: e, face: f },
            ..
        } => (edge(*e)?, face(*f)?),
        E::UnrecordedContact {
            contact:
                crate::CensusContact::EdgeEdgeCross {
                    a,
                    b,
                    side: Some(crate::CrossingSideVerdict::SameSide),
                },
            ..
        } => (edge(*a)?, edge(*b)?),
        touch @ E::UnrecordedContact { .. } => {
            let [x, y] = crate::census_contact_row(touch)?
                .cells
                .map(|cell| match cell {
                    crate::RowCell::Result { cell } => crate::census_cell_solid(body, cell),
                    crate::RowCell::Input { .. } | crate::RowCell::Tool => None,
                });
            (x?, y?)
        }
        E::CoincidentWedgesOverlap { edges: [a, b], .. }
        | E::CoincidentWedgesOverlapUndecided { edges: [a, b], .. } => (edge(*a)?, edge(*b)?),
        E::FaceThroughEdgeWedge {
            edge: e, face: f, ..
        }
        | E::FaceThroughEdgeWedgeUndecided {
            edge: e, face: f, ..
        } => (edge(*e)?, face(*f)?),
        // A solid crossing itself is about that one solid, placed as
        // (s, s) as census `Named::of` reads it: inside an arrival a
        // door already censused, it is that door's.
        E::BoundaryCrossesItself { solid, .. }
        | E::BoundaryCrossesItselfUndecided { solid, .. } => (*solid, *solid),
        E::ContactContradicted { record, .. } => (face(record.a)?, face(record.b)?),
        E::CensusUnsupported { subject, .. } | E::CensusLaneUnsupported { subject } => {
            match subject {
                crate::CensusSubject::FacePair(a, b) => (face(*a)?, face(*b)?),
                crate::CensusSubject::Entity(_) => return None,
            }
        }
        // Census findings that name no two solids: a stale record is
        // about the record a door carried, not a meeting; an escalation
        // names no entity at all
        // (`work/contacthold/an-escalated-census-finding-names-no-pair.md`);
        // an arc end off its corner names one face.
        E::StaleContactDeclaration { .. }
        | E::CensusEscalated { .. }
        | E::ArcEndOffCorner { .. } => {
            return None;
        }
        // Every other finding is about one body's own topology or
        // geometry, never a pair of solids. Listed rather than a
        // wildcard, so a finding topo adds is placed here by hand.
        E::OutsideSessionBox { .. }
        | E::Band { .. }
        | E::DanglingDescription { .. }
        | E::UncertifiableSurface { .. }
        | E::PoisonedSurfaceDescription { .. }
        | E::ApproxCertification { .. }
        | E::ApproxLaneUnsupported { .. }
        | E::DegenerateTorus { .. }
        | E::DegenerateTorusEscalated { .. }
        | E::DegenerateSpindle { .. }
        | E::SheetApexReached { .. }
        | E::PoisonedSurfaceDatum { .. }
        | E::UnrepresentableSurfaceDatum { .. }
        | E::PoisonedCurveDatum { .. }
        | E::UnrepresentableCurveDatum { .. }
        | E::SurfaceDatumUndecided { .. }
        | E::CurveDatumUndecided { .. }
        | E::EdgeCertification { .. }
        | E::DescriptionNotAdjacent { .. }
        | E::PlanarFaceResidual { .. }
        | E::PlanarFaceEscalated { .. }
        | E::PlanarBoundaryResidual { .. }
        | E::PlanarBoundaryEscalated { .. }
        | E::SliverDihedral { .. }
        | E::TransverseNotIntrinsic { .. }
        | E::MixedDihedral { .. }
        | E::ScaffoldAtRest { .. }
        | E::JoinableVertexAtRest { .. }
        | E::JoinUndecidedAtRest { .. }
        | E::TangentNotIntrinsic { .. }
        | E::LaminaWedge { .. }
        | E::NoDihedralArm { .. }
        | E::LoopRoleInverted { .. }
        | E::CurvedSenseInverted { .. }
        | E::SeamRegionOvercovered { .. }
        | E::SeamRegionCoverUnresolved { .. }
        | E::ChartSingularityNotAVertex { .. }
        | E::ChartSingularityUndecided { .. }
        | E::WrapTipOffChartSingularity { .. }
        | E::WrapTipUndecided { .. }
        | E::WrapTipWithoutChartSingularity { .. }
        | E::NegativeVolume { .. }
        | E::SliverInsideOut { .. }
        | E::VolumeUncomputable { .. }
        | E::VolumeSignUnresolved { .. }
        | E::OrientationUndecided { .. }
        | E::Pcurve { .. }
        | E::RingMeetsOuter { .. }
        | E::RingContactEscalated { .. }
        | E::RingOutsideOuter { .. }
        | E::RingNestingUndecided { .. }
        | E::CycleOffFaceSide { .. }
        | E::CycleSideUndecided { .. }
        | E::EssentialCyclesUnpaired { .. }
        | E::RingMeetsRing { .. }
        | E::RingPairContactEscalated { .. }
        | E::PinchCornerCrossed { .. }
        | E::PinchCornerEscalated { .. }
        | E::ShellWinding { .. }
        | E::SolidOuterShells { .. }
        | E::ShellRoleUndecided { .. }
        | E::CuspEdgesMissing { .. }
        | E::MaterialOnBothSides { .. }
        | E::DanglingTopology { .. }
        | E::DanglingGeometry { .. }
        | E::NextPrevMismatch { .. }
        | E::LoopCycleOverrun { .. }
        | E::ParentLoopMismatch { .. }
        | E::UnreachableHalfEdge { .. }
        | E::EdgeHalvesIdentical { .. }
        | E::EdgeSlotBackpointerMismatch { .. }
        | E::HalfEdgeUnclaimed { .. }
        | E::HalfEdgeMultiplyClaimed { .. }
        | E::EdgeNotAntiparallel { .. }
        | E::EmanatingStartMismatch { .. }
        | E::EmptyLoopVertexWithEmanating { .. }
        | E::LoneVertexWithIncidence { .. }
        | E::VertexOrbitOverrun { .. }
        | E::OrbitForeignMember { .. }
        | E::SplitVertexOrbit { .. }
        | E::OuterListedAsRing { .. }
        | E::BackPointerMismatch { .. }
        | E::OrphanEntity { .. }
        | E::MultiplyOwned { .. }
        | E::OrphanGeometry { .. }
        | E::Unmovable { .. }
        | E::SolidWithoutShells { .. }
        | E::ShellWithoutFaces { .. }
        | E::EdgeAcrossShells { .. }
        | E::MissingProvenance { .. }
        | E::LeakedProvenance { .. }
        | E::ScaffoldingEmptyLoop { .. }
        | E::ScaffoldingStrutVertex { .. }
        | E::ShellDisconnected { .. }
        | E::NullScaffoldShared { .. }
        | E::LeakedNullFaceRecord { .. }
        | E::StaleNullFaceLoop { .. }
        | E::StaleNullFaceOwnership { .. }
        | E::NullEdgeAtRest { .. }
        | E::NullFaceAtRest { .. } => return None,
    };
    Some((x.min(y), x.max(y)))
}
