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
pub(crate) fn confirm_only<T: crate::props::AtRestPolicy + geom_core::Bounds>(
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
    // their kind. A lane that reads the pair says more than a box does
    // (a verdict, a margin), so the boxes are read only where a lane
    // declined, and then of every record.
    if !out.no_lane.is_empty() {
        out.refused.extend(
            contacts
                .rows()
                .iter()
                .filter_map(|row| apart(body, row.record.kind(), band))
                .map(|declaration| ValidationError::StaleContactDeclaration { declaration }),
        );
    }
    out
}

/// **A recorded pair whose cells are decided apart**, as the stale
/// record it is. Two readings, each in the granting direction, either
/// of which decides it:
///
/// - the narrow phase the Boolean's sweep reads behind a box overlap
///   ([`crate::separating::apart`]), for a pair of faces or edges: both
///   cells' covers separate, past the band, along the axis between
///   their anchors, a world axis, or a planar cell's normal;
/// - each cell's enclosure (a face's or an edge's cover box, a vertex's
///   point) and a lower bound on the distance between the two
///   ([`bvh::Aabb::separation_lo`]) past the band's far end.
///
/// A cell with no cover, or one the body does not hold, grants
/// nothing: its record is not apart by this reading, and the lanes say
/// what is wrong with it.
///
/// It decides apart only what a separating direction among those
/// decides: two cells that touch nowhere and that none of them
/// separates (an edge in the notch of an L-shaped face on its carrier)
/// are not apart here
/// (`work/heldrecords/a-recorded-tangency-past-the-certifiers-set-is-held-on-its-citation.md`).
fn apart<T: crate::props::AtRestPolicy + geom_core::Bounds>(
    body: &Body<T>,
    kind: crate::boolean::ContactKind,
    band: Band,
) -> Option<StaleDeclaration> {
    use crate::boolean::ContactKind;
    use crate::separating::Item;
    let vertex = |v| {
        let record = body.get_vertex(v)?;
        bvh::Aabb::from_points([body.point_of(v, record)])
    };
    let edge = |e| {
        body.get_edge(e)?;
        Some(crate::cover::edge_cover(body, e).ok()?.aabb(0.0))
    };
    let face = |f| {
        body.get_face(f)?;
        Some(crate::cover::face_cover(body, f, band).ok()?.aabb(0.0))
    };
    // The narrow phase over two live cells, its axes the world's and
    // each planar cell's normal.
    let narrow = |x: Item<T>, y: Item<T>| {
        let faces = [x, y].into_iter().filter_map(|item| match item {
            Item::Face(f) => Some((body, f)),
            _ => None,
        });
        let mut axes = crate::separating::face_axes(faces, band, None);
        axes.extend(crate::separating::world_axes());
        crate::separating::apart((body, x), (body, y), &axes, 0.0, band)
    };
    let (a, b, items, declaration) = match kind {
        ContactKind::VertexVertex(a, b) => (
            vertex(a)?,
            vertex(b)?,
            None,
            StaleDeclaration::VertexVertex { a, b },
        ),
        ContactKind::VertexEdge(v, e) => (
            vertex(v)?,
            edge(e)?,
            None,
            StaleDeclaration::VertexOnEdge { vertex: v, edge: e },
        ),
        ContactKind::VertexFace(v, f) => (
            vertex(v)?,
            face(f)?,
            None,
            StaleDeclaration::VertexOnFace { vertex: v, face: f },
        ),
        ContactKind::EdgeEdge(a, b) => (
            edge(a)?,
            edge(b)?,
            Some((Item::Edge(a), Item::Edge(b))),
            StaleDeclaration::EdgeEdge { a, b },
        ),
        ContactKind::EdgeFace(e, f) => (
            edge(e)?,
            face(f)?,
            Some((Item::Edge(e), Item::Face(f))),
            StaleDeclaration::EdgeFace { edge: e, face: f },
        ),
        ContactKind::FaceFace(a, b) => (
            face(a)?,
            face(b)?,
            Some((Item::Face(a), Item::Face(b))),
            StaleDeclaration::Patch {
                face_a: a,
                face_b: b,
            },
        ),
    };
    (a.separation_lo(&b) > band.escalate() || items.is_some_and(|(x, y)| narrow(x, y)))
        .then_some(declaration)
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
        | ValidationError::ComponentEulerViolation { .. }
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::apart;
    use crate::boolean::ContactKind;
    use crate::test_support_fixtures::geometric_cube;
    use geom_core::{Band, Tol};

    /// A cell with no cover is not read as apart: the separation
    /// reading grants only what a cover proves, so a record on such a
    /// cell goes on to the lanes. A null strut hung at a cube's corner
    /// has no carrier, so no cover; against the cube's farthest edge it
    /// is not apart, where two of the cube's own opposite edges are.
    #[test]
    fn a_cell_with_no_cover_is_not_read_as_apart() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let cube = geometric_cube::<f64>(tol);
        let mut body = cube.body;
        let corner = cube.seed.vertex;
        let he = body.get_vertex(corner).unwrap().emanating.unwrap();
        let strut = body
            .mev_null(
                crate::construction::MevSite::Fan { he1: he, he2: he },
                crate::null::NewVertexSide::Above,
            )
            .unwrap();
        assert!(
            crate::cover::edge_cover(&body, strut.edge).is_err(),
            "the strut has no cover"
        );
        let at = |v| body.get_vertex(v).unwrap().point;
        let touches = |e, v| {
            let (a, b) = body.edge_vertices(e).unwrap();
            at(a) == at(v) || at(b) == at(v)
        };
        let edges: Vec<_> = body
            .edges()
            .map(|(e, _)| e)
            .filter(|&e| e != strut.edge)
            .collect();
        let far = *edges
            .iter()
            .find(|&&e| {
                let (a, b) = body.edge_vertices(e).unwrap();
                [a, b].iter().all(|&v| {
                    let (p, q) = (
                        body.point_of(v, body.get_vertex(v).unwrap()),
                        body.point_of(corner, body.get_vertex(corner).unwrap()),
                    );
                    p.x != q.x && p.y != q.y
                })
            })
            .expect("a cube edge sharing neither x nor y with the corner");
        let near = *edges
            .iter()
            .find(|&&e| touches(e, corner))
            .expect("an edge at the corner");
        assert!(
            apart(&body, ContactKind::EdgeEdge(near, far), band).is_some(),
            "two covered edges a side apart are apart, so the reading is not vacuous"
        );
        assert_eq!(
            apart(&body, ContactKind::EdgeEdge(strut.edge, far), band),
            None,
            "an edge with no cover grants nothing"
        );
    }
}
