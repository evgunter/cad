//! The census's confirmation direction, run alone, and what each of
//! its answers on a recorded pair means to a gate.

use geom_core::Band;

use crate::body::Body;
use crate::boolean::ContactRecords;
use crate::validate::{CensusUnsupportedCause, StaleDeclaration, ValidationError};

/// **What the census answered on the records a gate was handed**
/// ([`confirm_only`]). A record with no finding here is confirmed.
#[derive(Debug, Default)]
pub(crate) struct Confirmation {
    /// The census read the pair and the record is not what it found, or
    /// it read it and could not decide: the gate refuses on these.
    pub(crate) refused: Vec<ValidationError>,
    /// The census has no lane that reads the pair: the record is held
    /// on its citation, and the body's bar is tier 3
    /// ([`crate::Bar::Tier3`]).
    pub(crate) no_lane: Vec<ValidationError>,
}

/// One finding of the confirmation direction, as a gate takes it.
enum Reading {
    /// No lane reads the pair.
    NoLane,
    /// Read, and not confirmed.
    NotConfirmed,
}

/// **The confirmation direction alone**: each of `contacts` read where
/// it lies ([`super::confirm_records`]), every finding one record's that
/// did not confirm, sorted by what the census did ([`Confirmation`]). A
/// deciding door's gate runs this over the records it is handed
/// (`[topology.tiers]`); it searches for no meeting, so it pays nothing
/// for the body's other cells.
///
/// **What it can still hand back as held, unconfirmed**
/// ([`Confirmation::no_lane`]): a record of a kind whose lane says no
/// reading speaks to it ([`crate::contact::ContactReach::NoLane`]) and
/// whose two cells [`decided_apart`] does not decide apart. Such a record may
/// be false, a caller's [`crate::Built::recording`] reaches this gate
/// with one, and nothing here reads what it cites: the end state is a
/// census lane for every kind a door records
/// (`work/heldrecords/a-recorded-tangency-past-the-certifiers-set-is-held-on-its-citation.md`,
/// which names the test that holds one).
pub(crate) fn confirm_only<T: crate::props::AtRestPolicy>(
    body: &Body<T>,
    contacts: &ContactRecords,
    band: Band,
    region: Option<T::RegionLane>,
) -> Confirmation {
    let mut errors = Vec::new();
    if !contacts.is_empty() {
        super::confirm_records(
            body,
            &super::snapshot(body),
            contacts,
            band,
            region,
            &mut errors,
        );
    }
    let mut out = Confirmation::default();
    for finding in errors {
        match reading(&finding) {
            Reading::NoLane => out.no_lane.push(finding),
            Reading::NotConfirmed => out.refused.push(finding),
        }
    }
    // No record is held on its citation before its separation is read:
    // a lane that answers "this pair is not mine" has read no distance,
    // and two cells a definite distance apart do not touch whatever
    // their kind. The order is deliberate: a lane that reads the pair
    // says more than a separating axis does (a verdict, a margin), so
    // its answer stands where it gives one, and separation is read only
    // where a lane declined, and then of every record, so no gate with
    // no declined lane pays for a cover. The row that goes red if a
    // record were held without this read:
    // `sweep review_7124_delta_probes::red_no_record_between_bodies_that_touch_nowhere_is_held`.
    if !out.no_lane.is_empty() {
        out.refused.extend(
            contacts
                .rows()
                .iter()
                .filter_map(|row| decided_apart(body, row.record, band))
                .map(|declaration| ValidationError::StaleContactDeclaration { declaration }),
        );
    }
    out
}

/// **A recorded pair whose cells are decided apart**, as the stale
/// record it is: the narrow phase the Boolean's sweep reads behind a
/// box overlap ([`crate::separating::apart`]) over the record's two
/// cells: a face, an edge, and a vertex as the ball of no radius about
/// its point. Their covers separate, past the band, along the axis between
/// their anchors, a world axis, or a planar cell's normal. It reads in
/// the granting direction: a cell with no cover, or one the body does
/// not hold, grants nothing, so its record is not apart by this
/// reading, and the lanes say what is wrong with it.
///
/// It decides apart only what a separating direction among those
/// decides: two cells that touch nowhere and that none of them
/// separates (an edge in the notch of an L-shaped face on its carrier)
/// are not apart here
/// (`work/heldrecords/a-recorded-tangency-past-the-certifiers-set-is-held-on-its-citation.md`).
pub(crate) fn decided_apart<T: crate::props::AtRestPolicy>(
    body: &Body<T>,
    record: crate::Contact,
    band: Band,
) -> Option<StaleDeclaration> {
    use crate::separating::Item;
    let item = |cell| match cell {
        crate::Cell::Vertex(v) => body.get_vertex(v).map(|vertex| Item::Ball {
            center: body.point_of(v, vertex),
            radius: T::zero(),
        }),
        crate::Cell::Edge(e) => body.get_edge(e).map(|_| Item::Edge(e)),
        crate::Cell::Face(f) => body.get_face(f).map(|_| Item::Face(f)),
    };
    let [x, y] = record.cells();
    let (x, y) = (item(x)?, item(y)?);
    let faces = [x, y].into_iter().filter_map(|item| match item {
        Item::Face(f) => Some((body, f)),
        _ => None,
    });
    let mut axes = crate::separating::face_axes(faces, band, None);
    axes.extend(crate::separating::world_axes());
    crate::separating::apart((body, x), (body, y), &axes, 0.0, band).then(|| record.kind().stale())
}

/// **How a gate takes one finding of the confirmation direction.** The
/// findings and the lane causes are matched whole, with no arm for
/// "any other": a finding or a cause added to one of them is placed
/// here by whoever adds it.
fn reading(finding: &ValidationError) -> Reading {
    use crate::chart_region::ChartRegionError as Region;
    use crate::contact::{ContactReach, ContactRefusal};
    match finding {
        ValidationError::CensusUnsupported { cause, .. } => match cause {
            // Nothing the kernel has speaks to the pair: its kinds are
            // outside every lane's set and nothing was measured, as the
            // site that raised it states ([`ContactReach::NoLane`]: two carriers
            // with no closed-form tangent locus, a kind outside the rest
            // inventory, a pair outside the jet certificate's lane, an
            // edge lying on a face's carrier). Each lane is owed by
            // `work/heldrecords/a-recorded-tangency-past-the-certifiers-set-is-held-on-its-citation.md`.
            CensusUnsupportedCause::ContactLane(ContactRefusal::NotCertifiable {
                reach: ContactReach::NoLane,
                ..
            }) => Reading::NoLane,
            // A reading ran, or was handed what it could not read, and
            // did not decide ([`ContactReach::Unread`]: a trim the containment
            // door does not read, a face with no extent or no surface,
            // a span bound the schedule did not reach, a meter that
            // refused).
            CensusUnsupportedCause::ContactLane(ContactRefusal::NotCertifiable {
                reach: ContactReach::Unread,
                ..
            }) => Reading::NotConfirmed,
            // In the band: the certifier looked and could not decide.
            CensusUnsupportedCause::ContactLane(ContactRefusal::Escalated { .. }) => {
                Reading::NotConfirmed
            }
            CensusUnsupportedCause::ChartRegion(region) => match region {
                // The boundaries touch, so the overlap is undecidable
                // at this ε either way: looked, and could not decide.
                Region::TouchingBoundary
                // In the band, or every ray grazed: looked, undecided.
                | Region::Escalated(_)
                | Region::RayExhausted
                // The carriers disagree by more than the pair absorbs:
                // read, and not one contact.
                | Region::CarrierTilt
                // The body's own state is wrong: a kernel defect.
                | Region::Corrupt
                // The region door's limits on a shape or a size. No
                // recorded pair reaches one in any suite, so none is
                // classed as a missing lane: each refuses until a body
                // a door builds shows it holding a true record.
                | Region::ChartDivergence { .. }
                | Region::NonPlanarTrim { .. }
                | Region::TrimOffFace { .. }
                | Region::MissingCache { .. }
                | Region::ArmUnbounded { .. }
                | Region::SeamBranch
                | Region::PeriodFold
                | Region::DegenerateLoop { .. }
                // A face with no outline is a skeleton, not a body at
                // rest: nothing a record can stand on.
                | Region::NoOutline(_)
                | Region::ShortLoop { .. }
                | Region::WitnessSegmentCapExceeded { .. }
                | Region::WitnessCellCapExceeded { .. }
                | Region::EdgeScale
                | Region::FaceUncovered(_) => Reading::NotConfirmed,
            },
            // A curved edge, a meeting, a face's bound or a containment
            // the census could not read. As the region door's limits
            // above: unreached by a recorded pair, refused until shown.
            CensusUnsupportedCause::CurvedEdge(_)
            | CensusUnsupportedCause::Meeting(_)
            | CensusUnsupportedCause::FaceUnboundable
            | CensusUnsupportedCause::Containment(_) => Reading::NotConfirmed,
        },
        // What the confirmation says of a record it read: no touch
        // behind it, a different contact than recorded, in the band, or
        // undecided.
        ValidationError::StaleContactDeclaration { .. }
        | ValidationError::ContactContradicted { .. }
        | ValidationError::CensusEscalated { .. }
        | ValidationError::CensusUndecidable { .. }
        | ValidationError::CensusLaneUnsupported { .. }
        | ValidationError::MaterialOnBothSides { .. }
        | ValidationError::CuspEdgesMissing { .. } => Reading::NotConfirmed,
        // No finding of the confirmation direction today: each is a
        // defect of the body, which tier 3 refuses before a record is
        // read. Named, so that a variant added to the enum is placed
        // here by whoever adds it; none is a lane's "not mine", so
        // each refuses.
        ValidationError::OutsideSessionBox { .. }
        | ValidationError::Band { .. }
        | ValidationError::DanglingDescription { .. }
        | ValidationError::UncertifiableSurface { .. }
        | ValidationError::PoisonedSurfaceDescription { .. }
        | ValidationError::ApproxCertification { .. }
        | ValidationError::ApproxLaneUnsupported { .. }
        | ValidationError::DegenerateTorus { .. }
        | ValidationError::DegenerateTorusEscalated { .. }
        | ValidationError::DegenerateSpindle { .. }
        | ValidationError::SheetApexReached { .. }
        | ValidationError::PoisonedSurfaceDatum { .. }
        | ValidationError::UnrepresentableSurfaceDatum { .. }
        | ValidationError::SurfaceDatumUndecided { .. }
        | ValidationError::PoisonedCurveDatum { .. }
        | ValidationError::UnrepresentableCurveDatum { .. }
        | ValidationError::CurveDatumUndecided { .. }
        | ValidationError::EdgeCertification { .. }
        | ValidationError::DescriptionNotAdjacent { .. }
        | ValidationError::PlanarFaceResidual { .. }
        | ValidationError::PlanarFaceEscalated { .. }
        | ValidationError::PlanarBoundaryResidual { .. }
        | ValidationError::PlanarBoundaryEscalated { .. }
        | ValidationError::SliverDihedral { .. }
        | ValidationError::TransverseNotIntrinsic { .. }
        | ValidationError::MixedDihedral { .. }
        | ValidationError::ScaffoldAtRest { .. }
        | ValidationError::JoinableVertexAtRest { .. }
        | ValidationError::JoinUndecidedAtRest { .. }
        | ValidationError::TangentNotIntrinsic { .. }
        | ValidationError::LaminaWedge { .. }
        | ValidationError::NoDihedralArm { .. }
        | ValidationError::LoopRoleInverted { .. }
        | ValidationError::CurvedSenseInverted { .. }
        | ValidationError::SeamRegionOvercovered { .. }
        | ValidationError::SeamRegionCoverUnresolved { .. }
        | ValidationError::ChartSingularityNotAVertex { .. }
        | ValidationError::ChartSingularityUndecided { .. }
        | ValidationError::WrapTipOffChartSingularity { .. }
        | ValidationError::WrapTipWithoutChartSingularity { .. }
        | ValidationError::WrapTipUndecided { .. }
        | ValidationError::NegativeVolume { .. }
        | ValidationError::SliverInsideOut { .. }
        | ValidationError::VolumeUncomputable { .. }
        | ValidationError::VolumeSignUnresolved { .. }
        | ValidationError::OrientationUndecided { .. }
        | ValidationError::Pcurve { .. }
        | ValidationError::RingMeetsOuter { .. }
        | ValidationError::RingContactEscalated { .. }
        | ValidationError::RingOutsideOuter { .. }
        | ValidationError::RingNestingUndecided { .. }
        | ValidationError::CycleOffFaceSide { .. }
        | ValidationError::CycleSideUndecided { .. }
        | ValidationError::EssentialCyclesUnpaired { .. }
        | ValidationError::RingMeetsRing { .. }
        | ValidationError::RingPairContactEscalated { .. }
        | ValidationError::PinchCornerCrossed { .. }
        | ValidationError::PinchCornerEscalated { .. }
        | ValidationError::ShellWinding { .. }
        | ValidationError::SolidOuterShells { .. }
        | ValidationError::ShellRoleUndecided { .. }
        | ValidationError::UnrecordedContact { .. }
        | ValidationError::FaceThroughEdgeWedge { .. }
        | ValidationError::FaceThroughEdgeWedgeUndecided { .. }
        | ValidationError::CoincidentWedgesOverlap { .. }
        | ValidationError::CoincidentWedgesOverlapUndecided { .. }
        | ValidationError::BoundaryCrossesItself { .. }
        | ValidationError::BoundaryCrossesItselfUndecided { .. }
        | ValidationError::ArcEndOffCorner { .. }
        | ValidationError::InstanceInterference { .. }
        | ValidationError::SameSideInterference { .. }
        | ValidationError::TouchInterference { .. }
        | ValidationError::DanglingTopology { .. }
        | ValidationError::DanglingGeometry { .. }
        | ValidationError::NextPrevMismatch { .. }
        | ValidationError::LoopCycleOverrun { .. }
        | ValidationError::ParentLoopMismatch { .. }
        | ValidationError::UnreachableHalfEdge { .. }
        | ValidationError::EdgeHalvesIdentical { .. }
        | ValidationError::EdgeSlotBackpointerMismatch { .. }
        | ValidationError::HalfEdgeUnclaimed { .. }
        | ValidationError::HalfEdgeMultiplyClaimed { .. }
        | ValidationError::EdgeNotAntiparallel { .. }
        | ValidationError::EmanatingStartMismatch { .. }
        | ValidationError::EmptyLoopVertexWithEmanating { .. }
        | ValidationError::LoneVertexWithIncidence { .. }
        | ValidationError::VertexOrbitOverrun { .. }
        | ValidationError::OrbitForeignMember { .. }
        | ValidationError::SplitVertexOrbit { .. }
        | ValidationError::OuterListedAsRing { .. }
        | ValidationError::BackPointerMismatch { .. }
        | ValidationError::OrphanEntity { .. }
        | ValidationError::MultiplyOwned { .. }
        | ValidationError::OrphanGeometry { .. }
        | ValidationError::Unmovable { .. }
        | ValidationError::SolidWithoutShells { .. }
        | ValidationError::ShellWithoutFaces { .. }
        | ValidationError::EdgeAcrossShells { .. }
        | ValidationError::MissingProvenance { .. }
        | ValidationError::LeakedProvenance { .. }
        | ValidationError::ScaffoldingEmptyLoop { .. }
        | ValidationError::ScaffoldingStrutVertex { .. }
        | ValidationError::ShellDisconnected { .. }
        | ValidationError::NullScaffoldShared { .. }
        | ValidationError::LeakedNullFaceRecord { .. }
        | ValidationError::StaleNullFaceLoop { .. }
        | ValidationError::StaleNullFaceOwnership { .. }
        | ValidationError::NullEdgeAtRest { .. }
        | ValidationError::NullFaceAtRest { .. } => Reading::NotConfirmed,
    }
}
