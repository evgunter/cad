//! **The finished body** (`docs/DESIGN.md` `[topology.tiers]`, the
//! at-rest row of `[one-way.places]`): the scalar's
//! [`AtRestPolicy::finish`], the one constructor of a finished body,
//! the two types it constructs, and its refusal.
//!
//! Each scalar names its finished type ([`AtRestPolicy::Finished`]),
//! the way [`geom_core::Real::Carries`] names what an edge carries. At
//! the scalars with certification rights (`f64`, `Probe`, `Interval`,
//! `Sym`) it is [`AtRestBody`], which only a certifying scalar can
//! construct. At a dual it is [`Replay`]: a body that passed tier 2
//! alone and carries no verdict, so nothing that reads a verdict
//! accepts one (DUAL-DESIGN DL3).
//!
//! The refusal is [`NotFinished`]: every finding the gate made, none
//! folded into another, each read through its family's one text and
//! recourse ([`FindingFamily`]).

use core::fmt;
use core::ops::Deref;

use geom_core::{Real, Tol};

use crate::body::Body;
use crate::props::AtRestPolicy;
use crate::validate::{AtRestBody, ValidationError};

/// **What every scalar's finished type is**: a body read through
/// [`Deref`] and never borrowed mutably, handed back only by
/// [`FinishedBody::into_body`], which gives up the finish.
///
/// It names its scalar ([`FinishedBody::Scalar`]), whose finished type it
/// is, so a door generic over the finished body reads its scalar off
/// the operand it is handed.
pub trait FinishedBody:
    Deref<Target = Body<Self::Scalar>> + Clone + fmt::Debug + Send + Sync + 'static
{
    /// The scalar this is the finished type of.
    type Scalar: AtRestPolicy<Finished = Self>;

    /// The body, its finish given up: construction state again.
    #[must_use]
    fn into_body(self) -> Body<Self::Scalar>;
}

impl<T: AtRestPolicy<Finished = Self>> FinishedBody for AtRestBody<T> {
    type Scalar = T;

    fn into_body(self) -> Body<T> {
        AtRestBody::into_body(self)
    }
}

/// **A dual's finished body**: a replay that passed tier 2 alone
/// ([`crate::validate_closed`]). It carries no verdict, so no door
/// that reads one accepts it; the base-scalar evaluation it replays is
/// the validation of record (DUAL-DESIGN DL3).
#[derive(Clone, Debug)]
pub struct Replay<T: Real> {
    body: Body<T>,
}

impl<T: Real> Replay<T> {
    /// Tier 2 over `body`, the replay kept where it passes.
    ///
    /// # Errors
    ///
    /// Tier 2's findings.
    pub(crate) fn of(body: Body<T>) -> Result<Self, NotFinished> {
        crate::validate::validate_closed(&body).map_err(NotFinished::new)?;
        Ok(Self { body })
    }

    /// The body, the replay given up.
    #[must_use]
    pub fn into_body(self) -> Body<T> {
        self.body
    }
}

impl<T: Real> Deref for Replay<T> {
    type Target = Body<T>;

    fn deref(&self) -> &Body<T> {
        &self.body
    }
}

impl<T: AtRestPolicy<Finished = Self>> FinishedBody for Replay<T> {
    type Scalar = T;

    fn into_body(self) -> Body<T> {
        Replay::into_body(self)
    }
}

/// **`body` finished at its scalar** ([`AtRestPolicy::finish`]), the
/// one spelling a caller outside the policy names.
///
/// # Errors
///
/// [`NotFinished`]: every finding the scalar's gate made.
pub fn finish<T: AtRestPolicy>(body: Body<T>, tol: Tol) -> Result<T::Finished, NotFinished> {
    T::finish(body, tol)
}

/// **Why a body is not a finished body**: the non-empty list of what
/// the scalar's gate found, every finding kept and none folded into
/// another. A door's error holds it whole in one variant and names only
/// its subject.
#[derive(Clone, Debug, PartialEq)]
pub struct NotFinished {
    findings: Vec<ValidationError>,
}

impl NotFinished {
    /// The gate's findings.
    ///
    /// # Panics
    ///
    /// On an empty list: a gate that refused found something.
    pub(crate) fn new(findings: Vec<ValidationError>) -> Self {
        assert!(
            !findings.is_empty(),
            "a gate refused a body and found nothing: every refusal names a finding"
        );
        Self { findings }
    }

    /// Every finding, in the order the gate made them; never empty.
    #[must_use]
    pub fn as_slice(&self) -> &[ValidationError] {
        &self.findings
    }

    /// A refusal over `findings`, for the rows that render a door's
    /// error without running its gate.
    ///
    /// # Panics
    ///
    /// On an empty list.
    #[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
    #[doc(hidden)]
    #[must_use]
    pub fn for_tests(findings: Vec<ValidationError>) -> Self {
        Self::new(findings)
    }

    /// The findings, the refusal given up.
    #[must_use]
    pub fn into_findings(self) -> Vec<ValidationError> {
        self.findings
    }

    /// Each family the findings fall in, in the order its first finding
    /// was made, with its findings in theirs.
    #[must_use]
    pub fn by_family(&self) -> Vec<(FindingFamily, Vec<&ValidationError>)> {
        let mut groups: Vec<(FindingFamily, Vec<&ValidationError>)> = Vec::new();
        for finding in &self.findings {
            let family = finding.family();
            match groups.iter_mut().find(|(f, _)| *f == family) {
                Some((_, members)) => members.push(finding),
                None => groups.push((family, vec![finding])),
            }
        }
        groups
    }
}

impl Deref for NotFinished {
    type Target = [ValidationError];

    /// Every finding, in the order the gate made them; never empty.
    fn deref(&self) -> &[ValidationError] {
        &self.findings
    }
}

impl PartialEq<Vec<ValidationError>> for NotFinished {
    fn eq(&self, other: &Vec<ValidationError>) -> bool {
        self.findings == *other
    }
}

impl<const N: usize> PartialEq<[ValidationError; N]> for NotFinished {
    fn eq(&self, other: &[ValidationError; N]) -> bool {
        self.findings[..] == other[..]
    }
}

impl<'a> IntoIterator for &'a NotFinished {
    type Item = &'a ValidationError;
    type IntoIter = core::slice::Iter<'a, ValidationError>;

    fn into_iter(self) -> Self::IntoIter {
        self.findings.iter()
    }
}

impl fmt::Display for NotFinished {
    /// Each family once, its text, then each of its findings, which ends
    /// in the family's recourse.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, (family, members)) in self.by_family().into_iter().enumerate() {
            if i > 0 {
                f.write_str("; ")?;
            }
            write!(f, "{} ({}): ", family.text(), members.len())?;
            for (j, finding) in members.into_iter().enumerate() {
                if j > 0 {
                    f.write_str(" | ")?;
                }
                write!(f, "{finding}")?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for NotFinished {}

/// **The decision or check that made a finding** — the unit a
/// refusal's text and recourse are written once for
/// (`docs/DESIGN.md` `[topology.tiers]`, `[tolerance.recourse]` (i)).
///
/// A family's findings share its text ([`FindingFamily::text`]) and its
/// recourse. Where the recourse is one sentence whatever the finding
/// ([`FindingFamily::recourse`] answers it), [`ValidationError`]'s
/// display ends every member in it and no member writes it. Where the
/// family is one decision with a verdict (definite, or a margin in the
/// band), its recourse is that decision's ending, written once at the
/// decision and read off the finding's own verdict and margin, so the
/// margin rides the finding as data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FindingFamily {
    /// The run's tolerance forms no band, so nothing geometric is read.
    Tolerance,
    /// Tier 1: the topology's own structure (links, orbits, ownership,
    /// provenance, construction records) is broken.
    Structure,
    /// Tier 2 and the transience fence: what a construction leaves
    /// behind before it comes to rest (an empty loop, a strut, a shell
    /// in pieces, a null edge or face, a scaffold description).
    Scaffolding,
    /// A stored description disagrees with the geometry it describes,
    /// or holds numbers that describe nothing.
    Description,
    /// A surface's stored size breaks its kind's convention (a ring
    /// torus, a datum in range): a size the user may intend.
    Convention,
    /// A certificate re-derived at rest failed, or the lane that derives
    /// it does not run at this scalar.
    Certificate,
    /// A flat face's vertices and edges stay on its plane (check 2).
    Planar,
    /// How the faces at an edge meet (check 4: the dihedral and the
    /// material wedge).
    Dihedral,
    /// Maximal edges (check 11): no vertex the join would take.
    Join,
    /// A face's sense and its region on its surface (check 6, its
    /// seam-region cover and its chart singularities).
    FaceSense,
    /// A face's loops bound one region (check 9 and one region per
    /// face).
    FaceRegion,
    /// Which side of the boundary is material (checks 7 and 10: the
    /// volume's sign, the shells' roles and winding).
    Orientation,
    /// The census of the body's contacts against its records (tier 3′).
    Contact,
    /// The boundary is embedded off its records (tier 3′): no face
    /// through a wedge, no overlapping wedges, no self-crossing.
    Embedding,
    /// Instances interfere (the census's containment arm).
    Interference,
}

impl FindingFamily {
    /// The family's one text: what its findings say about the body.
    #[must_use]
    pub const fn text(self) -> &'static str {
        match self {
            Self::Tolerance => "the tolerance forms no band",
            Self::Structure => "the topology is broken",
            Self::Scaffolding => "construction scaffolding is left at rest",
            Self::Description => "a stored description does not describe its geometry",
            Self::Convention => "a surface breaks its kind's convention",
            Self::Certificate => "a certificate does not hold at rest",
            Self::Planar => "a flat face leaves its plane",
            Self::Dihedral => "faces meet badly at an edge",
            Self::Join => "edges are not joined",
            Self::FaceSense => "a face's sense or region on its surface is wrong",
            Self::FaceRegion => "a face's loops do not bound one region",
            Self::Orientation => "the material's side is wrong or unread",
            Self::Contact => "a contact disagrees with its records",
            Self::Embedding => "the boundary meets itself off its records",
            Self::Interference => "instances interfere",
        }
    }

    /// The family's one recourse, where it is one sentence whatever the
    /// finding; `None` where the family is one decision whose ending
    /// reads the finding's verdict and margin.
    #[must_use]
    pub const fn recourse(self) -> Option<&'static str> {
        match self {
            Self::Tolerance => Some(crate::validate::TOLERANCE),
            Self::Structure | Self::Scaffolding | Self::Description => {
                Some(geom_core::KERNEL_OR_FILE_DEFECT_ENDING)
            }
            Self::Convention
            | Self::Certificate
            | Self::Planar
            | Self::Dihedral
            | Self::Join
            | Self::FaceSense
            | Self::FaceRegion
            | Self::Orientation
            | Self::Contact
            | Self::Embedding
            | Self::Interference => None,
        }
    }
}

impl ValidationError {
    /// **The family this finding belongs to** — the one place each
    /// finding's family is stated.
    #[must_use]
    pub const fn family(&self) -> FindingFamily {
        use FindingFamily as F;
        match self {
            Self::Band { .. } => F::Tolerance,
            Self::DanglingTopology { .. }
            | Self::DanglingGeometry { .. }
            | Self::NextPrevMismatch { .. }
            | Self::LoopCycleOverrun { .. }
            | Self::ParentLoopMismatch { .. }
            | Self::UnreachableHalfEdge { .. }
            | Self::EdgeHalvesIdentical { .. }
            | Self::EdgeSlotBackpointerMismatch { .. }
            | Self::HalfEdgeUnclaimed { .. }
            | Self::HalfEdgeMultiplyClaimed { .. }
            | Self::EdgeNotAntiparallel { .. }
            | Self::EmanatingStartMismatch { .. }
            | Self::EmptyLoopVertexWithEmanating { .. }
            | Self::LoneVertexWithIncidence { .. }
            | Self::VertexOrbitOverrun { .. }
            | Self::OrbitForeignMember { .. }
            | Self::SplitVertexOrbit { .. }
            | Self::OuterListedAsRing { .. }
            | Self::BackPointerMismatch { .. }
            | Self::OrphanEntity { .. }
            | Self::MultiplyOwned { .. }
            | Self::OrphanGeometry { .. }
            | Self::SolidWithoutShells { .. }
            | Self::ShellWithoutFaces { .. }
            | Self::EdgeAcrossShells { .. }
            | Self::ComponentEulerViolation { .. }
            | Self::MissingProvenance { .. }
            | Self::LeakedProvenance { .. }
            | Self::NullScaffoldShared { .. }
            | Self::LeakedNullFaceRecord { .. }
            | Self::StaleNullFaceLoop { .. }
            | Self::StaleNullFaceOwnership { .. } => F::Structure,
            Self::ScaffoldingEmptyLoop { .. }
            | Self::ScaffoldingStrutVertex { .. }
            | Self::ShellDisconnected { .. }
            | Self::NullEdgeAtRest { .. }
            | Self::NullFaceAtRest { .. }
            | Self::ScaffoldAtRest { .. } => F::Scaffolding,
            Self::DanglingDescription { .. }
            | Self::UncertifiableSurface { .. }
            | Self::PoisonedSurfaceDescription { .. }
            | Self::PoisonedSurfaceDatum { .. }
            | Self::PoisonedCurveDatum { .. }
            | Self::UnrepresentableCurveDatum { .. }
            | Self::DescriptionNotAdjacent { .. }
            | Self::TransverseNotIntrinsic { .. }
            | Self::TangentNotIntrinsic { .. } => F::Description,
            Self::UnrepresentableSurfaceDatum { .. }
            | Self::DegenerateTorus { .. }
            | Self::DegenerateTorusEscalated { .. } => F::Convention,
            Self::ApproxCertification { .. }
            | Self::ApproxLaneUnsupported { .. }
            | Self::EdgeCertification { .. }
            | Self::Pcurve { .. }
            | Self::VolumeUncomputable { .. } => F::Certificate,
            Self::PlanarFaceResidual { .. }
            | Self::PlanarFaceEscalated { .. }
            | Self::PlanarBoundaryResidual { .. }
            | Self::PlanarBoundaryEscalated { .. } => F::Planar,
            Self::SliverDihedral { .. }
            | Self::MixedDihedral { .. }
            | Self::NoDihedralArm { .. }
            | Self::LaminaWedge { .. } => F::Dihedral,
            Self::JoinableVertexAtRest { .. } | Self::JoinUndecidedAtRest { .. } => F::Join,
            Self::LoopRoleInverted { .. }
            | Self::CurvedSenseInverted { .. }
            | Self::SeamRegionOvercovered { .. }
            | Self::SeamRegionCoverUnresolved { .. }
            | Self::ChartSingularityNotAVertex { .. }
            | Self::ChartSingularityUndecided { .. }
            | Self::WrapTipOffChartSingularity { .. }
            | Self::WrapTipWithoutChartSingularity { .. }
            | Self::WrapTipUndecided { .. } => F::FaceSense,
            Self::RingMeetsOuter { .. }
            | Self::RingContactEscalated { .. }
            | Self::RingOutsideOuter { .. }
            | Self::RingNestingUndecided { .. }
            | Self::CycleOffFaceSide { .. }
            | Self::CycleSideUndecided { .. }
            | Self::EssentialCyclesUnpaired { .. }
            | Self::RingMeetsRing { .. }
            | Self::RingPairContactEscalated { .. }
            | Self::PinchCornerCrossed { .. }
            | Self::PinchCornerEscalated { .. } => F::FaceRegion,
            Self::NegativeVolume { .. }
            | Self::SliverInsideOut { .. }
            | Self::VolumeSignUnresolved { .. }
            | Self::OrientationUndecided { .. }
            | Self::ShellWinding { .. }
            | Self::SolidOuterShells { .. }
            | Self::ShellRoleUndecided { .. } => F::Orientation,
            Self::UnrecordedContact { .. }
            | Self::StaleContactDeclaration { .. }
            | Self::ContactContradicted { .. }
            | Self::CensusEscalated { .. }
            | Self::CensusUnsupported { .. }
            | Self::CensusLaneUnsupported { .. }
            | Self::CensusUndecidable { .. }
            | Self::ArcEndOffCorner { .. }
            | Self::CuspEdgesMissing { .. } => F::Contact,
            Self::FaceThroughEdgeWedge { .. }
            | Self::FaceThroughEdgeWedgeUndecided { .. }
            | Self::CoincidentWedgesOverlap { .. }
            | Self::CoincidentWedgesOverlapUndecided { .. }
            | Self::BoundaryCrossesItself { .. }
            | Self::BoundaryCrossesItselfUndecided { .. } => F::Embedding,
            Self::InstanceInterference { .. }
            | Self::SameSideInterference { .. }
            | Self::TouchInterference { .. } => F::Interference,
        }
    }
}
