//! **What a Boolean refusal says for the decision that raised it**
//! (D4 ¶1 (i)): each decision is a closed type set where the refusal is
//! raised or wrapped, so its sentence is an exhaustive match, composed
//! here.
//!
//! - [`Contradiction`]: which fact contradicted a declared face pair
//!   (`BooleanError::DeclarationContradicted`,
//!   `MergeCoplanarError::DeclarationContradicted`), set by the rung
//!   that decided it (`plane_eq`'s declared rung, `carrier_eq`'s kind
//!   and data rungs). The verdict is definite, so it ends in
//!   `contact::CONTRADICTION_RECOURSE`, with no tolerance arm.
//! - [`BooleanDecision`]: which decision `BooleanError::Escalated`
//!   escalated on, set at the site that wraps the escalation.
//!   [`BooleanDecision::render`] composes the whole sentence from the
//!   decision and its verdict: a coincidence between the two solids its
//!   subject ([`Coincide::subject`]) and, where a declaration read
//!   ahead of it would settle it, `COINCIDENCE_RECOURSE`, or elsewhere
//!   the coincidence's own lever and the tolerance the gap gives; a
//!   decision on a size the user may intend its subject, its own lever
//!   and, on an in-band
//!   margin, the tolerance that decides it
//!   (`geom_brep::recourse::SizedDecision`); a decision whose margin is
//!   no size the user chose, or a family the escalation does not tell
//!   apart, its subject and its lever alone; a residual or a kernel
//!   self-check its subject and the defect ending
//!   (`geom_brep::recourse::Unsized`). A decision with a decided
//!   refusal of its own ends it from the same table: the torus
//!   convention's (`BooleanError::DegenerateTorus`) reads
//!   [`TorusConvention::sized`], the pierce curvature's
//!   (`BooleanError::CurvedSectorSideUnsupported`) [`PIERCE_CURVATURE`]
//!   and the maximal-faces gate's (`BooleanError::CoplanarNeighbours`)
//!   [`NEIGHBOUR_OFFSET`], as their escalations do.
//!
//! A coincidence is a decision each wrap site states, naming which one
//! it asks about ([`Coincide`]) and what the door read of the pair's
//! declaration ahead of it ([`DeclarationRead`]): the refusal offers a
//! declaration only where one read ahead would settle the question.

use geom_brep::LeverRung;
use geom_brep::recourse::{
    Classified, Reading, RefusedArm, SizedDecision, SizedPass, StoredDefinite, Unsized,
};
use geom_core::{COINCIDENCE_RECOURSE, Indeterminate, UNREADABLE_MARGIN_NOTE};

use crate::contact::ContactClass;

use super::plane_eq::PLANE_ORIENTATION;
pub use super::plane_eq::PlaneRung;
pub use super::solid_contain::WallRung;
use crate::face_normal::NormalDecision;
pub use crate::sector_shape::SectorRung;
use crate::splitting::ConicRootFault;
pub use crate::splitting::CrossingDecision;
pub use geom_brep::{SectionRadius, TorusConvention};

/// Which fact contradicted a declared pair: the rung that found the
/// two carriers definitely distinct.
///
/// The façade's curated list carries neither this nor
/// [`BooleanDecision`]; why, and what would change that, is
/// `work/lib/boolean-decision-and-contradiction-are-rungs-under-boolean-error.md`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contradiction {
    /// The declared planes' normals are not parallel.
    PlanesNotParallel,
    /// The declared planes are parallel and offset.
    PlanesApart,
    /// The declared faces lie on different kinds of surface.
    KindsDiffer,
    /// The declared cylinders' axes are not parallel.
    CylinderAxesNotParallel,
    /// The declared cylinders' axes are parallel and offset.
    CylinderAxesApart,
    /// The declared cylinders' radii differ.
    CylinderRadiiDiffer,
    /// The declared spheres' centres differ.
    SphereCentresDiffer,
    /// The declared spheres' radii differ.
    SphereRadiiDiffer,
    /// The declared tori's axes are not parallel.
    TorusAxesNotParallel,
    /// The declared tori's centres differ.
    TorusCentresDiffer,
    /// The declared tori's major radii differ.
    TorusMajorRadiiDiffer,
    /// The declared tori's tube radii differ.
    TorusTubeRadiiDiffer,
}

impl Contradiction {
    /// The fact, as a clause with no colon or dash of its own.
    #[must_use]
    pub fn fact(self) -> &'static str {
        match self {
            Self::PlanesNotParallel => "the declared planes are not parallel",
            Self::PlanesApart => "the declared planes are parallel but apart",
            Self::KindsDiffer => "the declared faces are different kinds of surface",
            Self::CylinderAxesNotParallel => "the declared cylinders' axes are not parallel",
            Self::CylinderAxesApart => "the declared cylinders' axes are parallel but apart",
            Self::CylinderRadiiDiffer => "the declared cylinders' radii differ",
            Self::SphereCentresDiffer => "the declared spheres' centres differ",
            Self::SphereRadiiDiffer => "the declared spheres' radii differ",
            Self::TorusAxesNotParallel => "the declared tori's axes are not parallel",
            Self::TorusCentresDiffer => "the declared tori's centres differ",
            Self::TorusMajorRadiiDiffer => "the declared tori's major radii differ",
            Self::TorusTubeRadiiDiffer => "the declared tori's tube radii differ",
        }
    }

    /// Whether the fact is a separation a designed clearance could
    /// explain (a parallel offset, a centre offset, a radius
    /// difference), which `contact_verify::fit_steer` points at `Fit`.
    /// An angle or a kind no gap reconciles is not, and neither are the
    /// torus separations, which the steer's inventory does not name.
    #[must_use]
    pub(crate) fn fits_a_clearance(self) -> bool {
        match self {
            Self::PlanesApart
            | Self::CylinderAxesApart
            | Self::CylinderRadiiDiffer
            | Self::SphereCentresDiffer
            | Self::SphereRadiiDiffer => true,
            Self::PlanesNotParallel
            | Self::KindsDiffer
            | Self::CylinderAxesNotParallel
            | Self::TorusAxesNotParallel
            | Self::TorusCentresDiffer
            | Self::TorusMajorRadiiDiffer
            | Self::TorusTubeRadiiDiffer => false,
        }
    }
}

/// Which decision a Boolean escalation came from, set at the site that
/// wraps the escalation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(strum::EnumDiscriminants))]
#[cfg_attr(
    test,
    strum_discriminants(name(BooleanDecisionKind), vis(pub(crate)), derive(strum::EnumIter))
)]
pub enum BooleanDecision {
    /// Whether parts of the two solids coincide, which parts the site
    /// asks about, and what its door read of the pair's declaration
    /// ahead of the question: the refusal offers a declaration only
    /// where that read says one would settle it
    /// ([`DeclarationRead::Settles`]), and otherwise names the
    /// coincidence's own lever and the tolerance the gap gives.
    Coincidence(Coincide, DeclarationRead),
    /// Whether two planes face the same way or opposite ways, at a
    /// cross-operand door, which both definite signs answer and no
    /// declaration changes ([`PlaneRung::Orientation`]).
    PlaneOrientation,
    /// Whether a declared pair's planes are parallel. The declared rung
    /// bridges an in-band margin, so what escalates is a norm it could
    /// not read ([`PlaneRung::Parallel`]).
    DeclaredParallel,
    /// Whether two neighbouring faces of one operand lie on one plane,
    /// as the maximal-faces gate asks it of a rung of the plane ladder
    /// (F7). Declarations name pairs across the operands, so none
    /// settles it.
    Neighbours(PlaneRung),
    /// A corner's own shape (`sector_shape`'s rungs).
    Corner(SectorRung),
    /// Whether a pierce point lies on the curved face it pierces, so
    /// the face's normal can be read there. The point is a vertex of
    /// the piercing solid that the contact sweep already placed on that
    /// face, so this is a residual re-asking a decision taken upstream.
    PierceOnFace,
    /// A half of a pierced torus's ring convention.
    Torus(TorusConvention),
    /// Whether a point lies inside a face, on its boundary, or outside
    /// it (`ContainError::Escalated`), from any rung of the walk.
    Containment,
    /// Where a crossing lands along its edge.
    Crossing(CrossingDecision),
    /// Whether a vertex of one solid coincides with a vertex of the
    /// other, asked of an edge end on a curved face: a coincidence both
    /// verdicts of which pass, and which no face-pair declaration names.
    VertexOnVertex,
    /// Whether a split point lies on the circle it was placed on: a
    /// residual on a point the kernel placed.
    SplitPointOnCircle,
    /// Whether an arc stays short of a full turn.
    ArcSpan,
    /// Whether the result's volume agrees with its operands': the
    /// kernel checking its own result.
    VolumeBackstop,
    /// Whether a length to measure an angle over is positive, at a gate
    /// that meters a reading by it.
    LeverArm(LeverArm),
    /// Whether a section operand's radius is positive, the arm's guard
    /// before it classifies any pose.
    Radius(SectionRadius),
    /// Where a straight edge crosses a cylinder wall: a rung of the line
    /// × wall quadratic.
    WallRoots(WallRung),
    /// How many times a straight edge crosses a torus: the certified
    /// count of the line × torus quartic's real roots.
    TorusRoots,
    /// Whether an edge leaves a curved face steeply enough, against the
    /// face's own bend, to read which side of it the edge goes.
    PierceCurvature,
    /// Whether two directions at a corner, read parallel, point the same
    /// way or opposite ways (`bool_dir_same`): the cosine levered at the
    /// corner's arm, `|cos| ≈ 1` once parallelism read zero, so what it
    /// measures is the arm, and either definite sign passes. A decided
    /// zero refuses with its decided margin, as the in-band arm does.
    DirectionSense,
    /// Which side of a face a corner's bisector leaves on, between two
    /// edges of the corner definitely on one side of it
    /// (`bool_sector_bisector_side`): either definite side passes, and a
    /// reading in the zero band, reachable only where the band's ratio
    /// is 2 or less, refuses rather than read the wrong side.
    BisectorSide,
}

/// **What a coincidence's door read of the face pair's declaration
/// ahead of the question**, and so whether a declaration would change
/// the verdict (D4 ¶1 (i)). A door holding the pair's class states it
/// through [`DeclarationRead::of`], so the refusal follows what was
/// read; only [`DeclarationRead::Settles`] offers a declaration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeclarationRead {
    /// The door reads the pair's declaration ahead of the question and
    /// finds none, and a verified one would settle the question.
    Settles,
    /// The door read the pair's declaration, of this class, ahead of the
    /// question, which refused all the same: the declaration is spent
    /// there, and no second one is offered.
    Spent(ContactClass),
    /// No declaration read ahead of the question settles it: the door
    /// reads none, or the one it reads only moves the refusal to another
    /// question, which then refuses the declared pair.
    Moot,
}

impl DeclarationRead {
    /// What a door read of a pair whose declared class is `class`.
    #[must_use]
    pub const fn of(class: Option<ContactClass>) -> Self {
        match class {
            None => Self::Settles,
            Some(class) => Self::Spent(class),
        }
    }
}

/// **Which coincidence between the two solids a wrap site asks about**:
/// the argument a site states when its escalation is a coincidence, the
/// subject its refusal opens on ([`Coincide::subject`]), and the lever
/// it names where no declaration would settle it
/// ([`Coincide::unsettled`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum Coincide {
    /// Whether two planes of the two solids are parallel, at a door
    /// whose ladder does not bridge an in-band margin: the pair is not
    /// declared `Rest` ([`PlaneRung::Parallel`]).
    Planes,
    /// Whether two faces' carriers are one carrier, where the carrier
    /// ladder refuses past its declared rung.
    Carriers,
    /// Whether a vertex of one solid lies on a face of the other.
    VertexOnFace,
    /// Whether a curved edge of one solid lies in, grazes or misses the
    /// plane of a face of the other.
    EdgeOnPlane,
    /// Whether an edge of one solid clears a curved face of the other,
    /// or lies on it.
    EdgeOnCurvedFace,
    /// Which side of a face of the other solid a corner's edge leaves
    /// on.
    SectorSide,
    /// Which way a face of one solid curves away from a face of the
    /// other that it touches tangentially, along a corner's edge: the
    /// second-order side of a declared-`Tangent` pair.
    TangentSide,
    /// How two corners of the two solids overlap where they meet: a
    /// direction within a sector, two sectors' faces parallel, their
    /// bounds in line, the order of a strut's germs, the germ line.
    Sectors,
    /// Whether an edge of one solid runs along an edge of the other.
    EdgeOnEdge,
    /// Where two faces of the two solids touch tangentially.
    TangentLocus,
    /// Whether the two faces at a seam edge cross there or touch
    /// tangentially.
    SeamWedge,
    /// Whether a sphere face of one solid meets the other solid.
    Sphere,
    /// Where the surfaces of a face of each solid meet (a section's
    /// pose).
    Section,
    /// How the sections' ends pair up where the two solids meet.
    Join,
    /// Whether a declared contact holds along its witness or its shared
    /// rim.
    Contact,
}

impl Coincide {
    /// The coincidence, as a clause with no colon or dash of its own:
    /// the subject of every refusal that asks it.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::Planes => "whether a face of each solid lies on one plane",
            Self::Carriers => "whether a face of each solid lies on one surface",
            Self::VertexOnFace => "whether a vertex of one solid lies on a face of the other",
            Self::EdgeOnPlane => {
                "whether a curved edge of one solid lies in, grazes or misses the plane of a face \
                 of the other"
            }
            Self::EdgeOnCurvedFace => {
                "whether an edge of one solid clears a curved face of the other or lies on it"
            }
            Self::SectorSide => "which side of a face of the other solid a corner's edge leaves on",
            Self::TangentSide => {
                "which way a face of one solid curves away from a face of the other that it \
                 touches"
            }
            Self::Sectors => "how two corners of the two solids overlap where they meet",
            Self::EdgeOnEdge => "whether an edge of one solid runs along an edge of the other",
            Self::TangentLocus => "where two faces of the two solids touch tangentially",
            Self::SeamWedge => {
                "whether the two faces at a seam edge cross there or touch tangentially"
            }
            Self::Sphere => "whether a sphere face of one solid meets the other solid",
            Self::Section => "where the surfaces of a face of each solid meet",
            Self::Join => "how the sections' ends pair up where the two solids meet",
            Self::Contact => "whether a declared contact holds along its witness or its shared rim",
        }
    }
}

impl Coincide {
    /// The coincidence's ending where no declaration read ahead of it
    /// settles it: its lever, and the tolerance an in-band margin gives
    /// on a side it passes on. Every definite verdict of each passes at
    /// the sites that raise it; a site whose code refuses a verdict
    /// raises a decision of its own ([`BooleanDecision::DirectionSense`],
    /// [`BooleanDecision::BisectorSide`]).
    #[must_use]
    pub const fn unsettled(self) -> SizedDecision {
        match self {
            Self::Planes => PLANES,
            Self::TangentSide => TANGENT_SIDE,
            Self::SeamWedge => SEAM_WEDGE,
            Self::Carriers
            | Self::VertexOnFace
            | Self::EdgeOnPlane
            | Self::EdgeOnCurvedFace
            | Self::SectorSide
            | Self::Sectors
            | Self::EdgeOnEdge
            | Self::TangentLocus
            | Self::Sphere
            | Self::Section
            | Self::Join
            | Self::Contact => PROXIMITY,
        }
    }
}

/// A coincidence no declaration settles: every definite verdict passes
/// (the parts meet, or lie apart on a side), so the geometry is the
/// lever and an in-band gap is a size a smaller tolerance decides.
const PROXIMITY: SizedDecision = SizedDecision {
    lever: "move the parts so they clearly meet or clearly stand apart there",
    size: "gap",
    passes: SizedPass::AnySign,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// Whether two planes of the two solids are parallel
/// ([`Coincide::Planes`]): the sine of their angle over the door's arm,
/// which passes on a clear angle (the planes are distinct) and on zero
/// (the ladder reads their offset next).
const PLANES: SizedDecision = SizedDecision {
    lever: "tilt one face so the two are clearly parallel or clearly not",
    size: "tilt between the two faces' planes",
    passes: SizedPass::NonNegative,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// The second-order side of a declared-`Tangent` pair
/// ([`Coincide::TangentSide`]): the faces' relative bend over the
/// corner's arm, which passes on either side and on zero (the
/// direction rides the tangency locus).
const TANGENT_SIDE: SizedDecision = SizedDecision {
    lever: "make one face clearly curve away from the other where they touch, or make both \
            curve alike there",
    size: "difference in bend",
    passes: SizedPass::AnySign,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// Whether the faces at a seam edge cross or touch tangentially
/// ([`Coincide::SeamWedge`]): the sine of their angle over the folded
/// lever arm, a crease on a positive margin and a smooth join on zero.
const SEAM_WEDGE: SizedDecision = SizedDecision {
    lever: "move the geometry so the faces at that seam meet either clearly creased or clearly \
            smooth",
    size: "fold across the seam",
    passes: SizedPass::NonNegative,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// Whether two directions at a corner, read parallel, point the same
/// way or opposite ways ([`BooleanDecision::DirectionSense`]): the
/// cosine at the corner's arm, whose size is the arm, and which either
/// definite sign passes.
const DIRECTION_SENSE: SizedDecision = SizedDecision {
    lever: CORNER_EDGES,
    size: "length of the corner's shorter edge",
    passes: SizedPass::NonZero,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// The one move that lengthens the arm a corner's reading is metered
/// over: shared by every decision whose margin, once its direction has
/// read, measures that arm ([`LeverArm`]'s corner gates,
/// [`BooleanDecision::DirectionSense`], [`BooleanDecision::BisectorSide`],
/// [`PLANE_ORIENTATION`]).
pub(crate) const CORNER_EDGES: &str =
    "make the edges at the corner where the two faces meet clearly longer than the tolerance";

/// **Which gate meters a reading over a lever arm** whose arm rung
/// escalated ([`BooleanDecision::LeverArm`]): the arm is a length the
/// gate measures the reading's angle over, and it passes only on a
/// definitely positive one.
///
/// Not [`SectorRung::Arm`], though both ask whether a length at a corner
/// is positive: that rung is the corner's own shape, its angle measured
/// over its shorter edge before any other solid is read, and it shares
/// its lever with the corner's straightness rung. These gates meter a
/// reading against a face of the other solid, over the reach of the
/// bound they read, which for a curved edge is its extent rather than
/// the corner's arm, so a corner whose shape passed can still refuse
/// here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum LeverArm {
    /// `enters_material_arm` at a pierce sector's side code: the
    /// reach of the bound it reads (a curved edge's extent, or the
    /// sector's arm).
    SectorSide,
    /// `tangent_sector_order2_arm` at a declared-tangent sector pair:
    /// the sector's arm.
    SectorCurving,
    /// `dihedral_arm` at a seam edge the result re-describes: the fold
    /// of the edge's extent and its faces' radii of curvature.
    Seam,
}

impl LeverArm {
    /// What the gate decides, as a clause with no colon or dash of its
    /// own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::SectorSide => {
                "whether an edge at a corner is long enough to read which side of a face it \
                 leaves on"
            }
            Self::SectorCurving => {
                "whether a corner's edges are long enough to read which way a face curves there"
            }
            Self::Seam => {
                "whether a seam edge is long enough, for how its faces curve, to measure the \
                 angle between them"
            }
        }
    }

    /// The gate's lever and the size its arm measures.
    const fn ending(self) -> Ending {
        match self {
            Self::SectorSide | Self::SectorCurving => {
                sized(CORNER_EDGES, "edge length", SizedPass::Positive)
            }
            // The dihedral's own arm, as every door that reads it ends it.
            Self::Seam => Ending::Sized(geom_brep::DIHEDRAL_ARM),
        }
    }
}

/// Whether an edge leaves a curved face steeply enough to read its side
/// ([`BooleanDecision::PierceCurvature`]): the margin is the edge's
/// first-order departure from the face's tangent plane less the
/// sagitta the face bends through over the same length, and a
/// definitely positive one passes. Its decided refusal
/// (`BooleanError::CurvedSectorSideUnsupported`) ends here too.
pub(crate) const PIERCE_CURVATURE: SizedDecision = SizedDecision {
    lever: "make the edge leave the curved face at a steeper angle, or make the face curve less \
            sharply there",
    size: "departure beyond the bend",
    passes: SizedPass::Positive,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// Whether two neighbouring faces of one operand lie on one plane, as
/// the maximal-faces gate reads the offset between their planes (F7):
/// either definite sign passes, and a zero offset with no shared source
/// refuses (`BooleanError::CoplanarNeighbours`, whose
/// [`NeighbourOffset`] is the refused arm). Declarations name pairs
/// across the operands, so none settles it.
pub(crate) const NEIGHBOUR_OFFSET: SizedDecision = SizedDecision {
    lever: NEIGHBOUR_LEVER,
    size: "offset between the two faces' planes",
    passes: SizedPass::NonZero,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// The offset the maximal-faces gate refused two neighbouring faces of
/// one operand on ([`NEIGHBOUR_OFFSET`]), as the plane ladder's offset
/// rung decided it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NeighbourOffset {
    /// The offset decided zero, with the margin its band decided: a
    /// nonzero one in the zero band is a size a smaller tolerance
    /// decides apart.
    Zero(Classified),
    /// The offset landed in the ambiguity band, or was poisoned.
    Undecided(Indeterminate),
}

impl NeighbourOffset {
    /// The refused arm of [`NEIGHBOUR_OFFSET`] this offset is.
    pub(crate) fn arm(&self) -> RefusedArm<'_> {
        match self {
            Self::Zero(classified) => RefusedArm::Zero(*classified),
            Self::Undecided(diag) => RefusedArm::Undecided(diag),
        }
    }

    /// The margin the rung classified or could not, with its band, as
    /// the payload a refusal quotes.
    pub(crate) const fn reported(self) -> Indeterminate {
        match self {
            Self::Zero(Classified { margin, band }) => Indeterminate {
                margin,
                band,
                predicate: Some("bool_plane_offset"),
                terminal_sliver: false,
            },
            Self::Undecided(diag) => diag,
        }
    }

    /// The maximal-faces gate's ending for this offset: the lever, and
    /// the tolerance a nonzero margin gives.
    pub(crate) fn ending(&self) -> String {
        NEIGHBOUR_OFFSET.recourse(self.arm(), Reading::Build)
    }
}

/// **Which sub-frontier the declared rest contact's zip met**
/// (`BooleanError::RestZipUnsupported`). The pair's declaration is
/// verified before the zip starts, so none is offered; each ends in the
/// lever that reaches past it, or, where no change the user makes is
/// known to, the frontier's own ending.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum RestZipFrontier {
    /// Two edges of one operand span one segment of the seam.
    ParallelSeamEdges,
    /// The Euler operator minting a seam chord across its host face
    /// refused.
    ChordMefRefused,
    /// The Euler operator minting a seam chord from a hole's boundary
    /// refused.
    ChordMekrRefused,
    /// The Euler operator minting a seam chord from an isolated pierce
    /// point refused.
    PierceRingMekrRefused,
    /// A seam chord's endpoint lies on no boundary of its host face.
    ChordEndpointAbsent,
    /// A seam chord joins two isolated pierce points.
    ChordBetweenIsolatedPierces,
    /// A seam chord's endpoint recurs on its host face's boundary.
    ChordEndpointRevisited,
    /// A contact patch's boundary vertex has no partner across the seam.
    PatchVertexUnmatched,
    /// The two contact patches' face cycles do not match across the
    /// mate.
    PatchCyclesIncongruent,
    /// The two contact patches hold different numbers of holes.
    HoleCountsDiffer,
    /// A hole's boundary vertex has no partner across the seam.
    HoleVertexUnmatched,
    /// The two contact patches' holes do not match across the mate.
    HoleCyclesIncongruent,
    /// A face zipped along part of its boundary holds holes.
    SlitFaceHoles,
    /// The two contact faces share their whole boundary.
    WholeBoundaryShared,
    /// A vertex inside a run of the seam holds edges off the run.
    RunVertexBranches,
    /// A run edge closing a band lies outside the folded face's loops.
    BandRunOffLoops,
    /// A vertex pair inside a zipped fold is fused already.
    FoldVertexFused,
}

impl RestZipFrontier {
    /// The sub-frontier, as a clause with no colon or dash of its own.
    #[must_use]
    pub const fn what(self) -> &'static str {
        match self {
            Self::ParallelSeamEdges => "two parallel operand edges span one seam segment",
            Self::ChordMefRefused => "seam chord mef refused on its host face",
            Self::ChordMekrRefused => "seam chord mekr refused on its host face",
            Self::PierceRingMekrRefused => "seam chord mekr (pierce ring) refused",
            Self::ChordEndpointAbsent => "seam chord endpoint has no boundary presence",
            Self::ChordBetweenIsolatedPierces => "seam chord between two isolated pierce points",
            Self::ChordEndpointRevisited => {
                "seam chord endpoint revisited by its host face boundary"
            }
            Self::PatchVertexUnmatched => "patch boundary vertex without a seam correspondent",
            Self::PatchCyclesIncongruent => "patch face cycles not congruent across the mate",
            Self::HoleCountsDiffer => "patch pair carries differing interior-boundary counts",
            Self::HoleVertexUnmatched => "ring boundary vertex without a seam correspondent",
            Self::HoleCyclesIncongruent => "ring cycles not congruent across the mate",
            Self::SlitFaceHoles => "slit-zip face carries rings",
            Self::WholeBoundaryShared => "patch pair shares its whole boundary",
            Self::RunVertexBranches => "seam-run interior vertex holds edges beyond the run",
            Self::BandRunOffLoops => "band-closure run edge outside the folded face's loops",
            Self::FoldVertexFused => "pre-fused vertex pair inside a slit-zip fold",
        }
    }

    /// The ending: the lever where one reaches past the sub-frontier,
    /// and the frontier's own ending elsewhere.
    pub(crate) const fn ending(self) -> &'static str {
        match self {
            // The zip glues the holes of the two contact faces pairwise,
            // by congruent cycles: holes that match across the mate are
            // what it takes.
            Self::HoleCountsDiffer | Self::HoleVertexUnmatched | Self::HoleCyclesIncongruent => {
                "Recourse: make the holes inside the declared contact match, one for one and \
                 corner for corner, across the two parts"
            }
            // The Euler operators' own refusals, and configurations of
            // the seam no move of the parts is known to avoid while
            // keeping the contact: a contact already planar can meet
            // them (two isolated pierce points).
            Self::ParallelSeamEdges
            | Self::ChordMefRefused
            | Self::ChordMekrRefused
            | Self::PierceRingMekrRefused
            | Self::ChordEndpointAbsent
            | Self::ChordBetweenIsolatedPierces
            | Self::ChordEndpointRevisited
            | Self::PatchVertexUnmatched
            | Self::PatchCyclesIncongruent
            | Self::SlitFaceHoles
            | Self::WholeBoundaryShared
            | Self::RunVertexBranches
            | Self::BandRunOffLoops
            | Self::FoldVertexFused => geom_core::NOT_YET_ENDING,
        }
    }
}

/// How one decision's escalation ends.
enum Ending {
    /// A coincidence between the two solids: the declaration, the
    /// geometry and the tolerance (`COINCIDENCE_RECOURSE`), in the
    /// coincidence's own sentence.
    Coincidence,
    /// A decision on a size the user may intend.
    Sized(SizedDecision),
    /// The lever alone: no margin of the decision gives a tolerance to
    /// tighten below, for the reason its pass set states.
    Lever(&'static str, LeverPass),
    /// A decision with no size the user chose.
    Unsized(Unsized),
}

/// What a lever-alone decision passes on, as its deciding code has it,
/// and so why no refusal of it offers the tolerance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LeverPass {
    /// It passes on a definitely positive margin, which is no length the
    /// user chose, so a tolerance below it names no size: the offer
    /// waits on a length door for the margin.
    PositiveNotALength,
    /// It passes on either definite sign, the margin no length the user
    /// chose, as [`LeverPass::PositiveNotALength`].
    NonZeroNotALength,
    /// The arms that read its verdict pass on different sets, so no one
    /// sign set is the decision's.
    ByArm,
    /// Its rungs pass on different sets, and the escalation does not
    /// say which rung refused.
    ByRung,
    /// No sign of its margin passes where it is asked.
    Never,
}

impl LeverPass {
    /// The ending on `diag`: the lever, with the unreadable-margin note
    /// on a poisoned margin, and never the tolerance, for the reason the
    /// pass set gives (no length to tighten below, or no one sign set a
    /// smaller tolerance would decide the margin into).
    fn recourse(self, lever: &str, diag: &Indeterminate) -> String {
        if diag.margin.is_invalid() {
            format!("Recourse: {lever}; {UNREADABLE_MARGIN_NOTE}")
        } else {
            format!("Recourse: {lever}")
        }
    }
}

/// Which door compared two planes: what a plane rung's escalation asks
/// there, and which move reaches a pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PlaneDoor {
    /// A face of each operand, with what the door read of the pair's
    /// declaration before it ran the ladder.
    Pair(DeclarationRead),
    /// Two neighbouring faces of one operand (F7).
    Neighbours,
}

impl PlaneDoor {
    /// The cross-operand door a pair whose declared class is `class`
    /// meets.
    pub(crate) const fn of(class: Option<ContactClass>) -> Self {
        Self::Pair(DeclarationRead::of(class))
    }
}

/// The maximal-faces gate's lever: the one move that takes two
/// neighbouring faces off the question, whichever rung asked it.
pub(crate) const NEIGHBOUR_LEVER: &str = "merge the two faces into one first \
                               (merge_coplanar_faces), or \
                               tilt one so they meet at a clear angle along an edge clearly \
                               longer than the tolerance";

/// A corner's own shape: its arm and its straightness, the two rungs
/// of [`SectorRung`], which one move answers.
const CORNER_LEVER: &str = "reshape that corner so its edges are clearly longer than the tolerance and clearly not in line";

/// A sized decision's table row at a build, where the stored arm is
/// never read.
const fn sized(lever: &'static str, size: &'static str, passes: SizedPass) -> Ending {
    Ending::Sized(SizedDecision {
        lever,
        size,
        passes,
        stored: StoredDefinite::Lever,
        at_zero: None,
    })
}

impl BooleanDecision {
    /// The decision a pierce point's face normal escalated on.
    pub(crate) const fn of_normal(decision: NormalDecision) -> Self {
        match decision {
            NormalDecision::Torus(half) => Self::Torus(half),
            NormalDecision::OnSurface => Self::PierceOnFace,
        }
    }

    /// The decision a plane-identity rung escalated on at `door`:
    /// across the operands, in-band parallelism is the planes'
    /// coincidence with what the door read (a declared `Rest` pair's
    /// ladder bridges an in-band margin, so only an unreadable norm
    /// escalates there), and orientation a decision of its own; between
    /// neighbours of one operand, both rungs ask the maximal-faces
    /// question.
    pub(crate) const fn of_plane_rung(rung: PlaneRung, door: PlaneDoor) -> Self {
        match (door, rung) {
            (PlaneDoor::Pair(DeclarationRead::Spent(ContactClass::Rest)), PlaneRung::Parallel) => {
                Self::DeclaredParallel
            }
            (PlaneDoor::Pair(read), PlaneRung::Parallel) => {
                Self::Coincidence(Coincide::Planes, read)
            }
            (PlaneDoor::Pair(_), PlaneRung::Orientation) => Self::PlaneOrientation,
            (PlaneDoor::Neighbours, rung) => Self::Neighbours(rung),
        }
    }

    /// The decision a conic root lane's escalation came from: a
    /// crossing decision as the fault routes it, and otherwise a
    /// coincidence between the plane and the conic, which the sweep asks
    /// ahead of any declaration.
    pub(crate) fn of_conic_root(fault: ConicRootFault) -> Self {
        fault.decision().map_or(
            Self::Coincidence(Coincide::EdgeOnPlane, DeclarationRead::Moot),
            Self::Crossing,
        )
    }

    /// The decision a lever-armed reading escalated on at `gate`: its arm
    /// rung is the gate's own length, and its reading is the coincidence
    /// `reading` names, with what the gate's door read of the pair's
    /// declaration ahead of it.
    pub(crate) const fn of_lever(
        gate: LeverArm,
        reading: Coincide,
        read: DeclarationRead,
        rung: LeverRung,
    ) -> Self {
        match rung {
            LeverRung::Arm => Self::LeverArm(gate),
            LeverRung::Reading => Self::Coincidence(reading, read),
        }
    }

    /// What the decision decides, as a clause with no colon or dash of
    /// its own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::Coincidence(which, _) => which.subject(),
            Self::PlaneOrientation => PlaneRung::Orientation.subject(),
            Self::DeclaredParallel => PlaneRung::Parallel.subject(),
            Self::Neighbours(_) => "whether two neighbouring faces of one operand lie on one plane",
            Self::Corner(rung) => rung.subject(),
            Self::PierceOnFace => {
                "whether a point lies on a curved face, so the face's normal can be read there"
            }
            Self::Torus(half) => half.subject(),
            Self::Containment => {
                "whether a point lies inside a face, on its boundary, or outside it"
            }
            Self::Crossing(decision) => decision.subject(),
            Self::VertexOnVertex => "whether a vertex of one solid coincides with one of the other",
            Self::SplitPointOnCircle => "whether a split point lies on the circle it was placed on",
            Self::ArcSpan => "whether an arc stays short of a full turn",
            Self::VolumeBackstop => "whether the result's volume agrees with its operands'",
            Self::LeverArm(gate) => gate.subject(),
            Self::Radius(radius) => radius.subject(),
            Self::WallRoots(WallRung::AxisParallel) => {
                "whether an edge runs parallel to a cylinder's axis"
            }
            Self::WallRoots(WallRung::Discriminant) => {
                "whether an edge crosses a cylinder wall, grazes it or misses it"
            }
            Self::TorusRoots => "how many times an edge crosses a torus",
            Self::PierceCurvature => {
                "whether an edge leaves a curved face steeply enough, for how sharply the face \
                 bends, to read which side it goes"
            }
            Self::DirectionSense => {
                "whether two parallel directions at a corner point the same way or opposite ways"
            }
            Self::BisectorSide => "which side of a face a corner's bisector leaves on",
        }
    }

    /// How the decision's escalation ends: from what it passes on.
    fn ending(self) -> Ending {
        match self {
            Self::Coincidence(_, DeclarationRead::Settles) => Ending::Coincidence,
            Self::Coincidence(which, DeclarationRead::Spent(_) | DeclarationRead::Moot) => {
                Ending::Sized(which.unsettled())
            }
            Self::PlaneOrientation => Ending::Sized(PLANE_ORIENTATION),
            // A poisoned description, which the merge's declared rung
            // ends the same way (`MergeDecision::DeclaredPlanes`).
            Self::DeclaredParallel => Ending::Unsized(Unsized::Defect),
            // The margin is the normals' sine over the shared edge's
            // chord, and a definitely positive one (a clear angle)
            // passes.
            Self::Neighbours(PlaneRung::Parallel) => sized(
                NEIGHBOUR_LEVER,
                "bend across their shared edge",
                SizedPass::Positive,
            ),
            // Asked only once the angle read flat over the chord; either
            // definite orientation then leaves the faces coplanar, which
            // the gate refuses, so no sign of this margin passes and no
            // tolerance decides it passing.
            Self::Neighbours(PlaneRung::Orientation) => {
                Ending::Lever(NEIGHBOUR_LEVER, LeverPass::Never)
            }
            // The arm passes on a positive length.
            Self::Corner(SectorRung::Arm) => {
                sized(CORNER_LEVER, "edge length", SizedPass::Positive)
            }
            // The margin is cos θ levered by the shorter edge: that
            // edge's projection onto the other, a length. A straight
            // corner passes on a negative one; a sector bounded twice by
            // one edge passes on any definite one.
            Self::Corner(SectorRung::Straight { full_circle }) => sized(
                CORNER_LEVER,
                "projection of one of the corner's edges onto the other",
                if full_circle {
                    SizedPass::AnySign
                } else {
                    SizedPass::Negative
                },
            ),
            // Each passes only at zero, and its definite sibling (a
            // point definitely off) is a broken classification
            // invariant.
            Self::PierceOnFace | Self::SplitPointOnCircle => Ending::Unsized(Unsized::Defect),
            Self::Torus(half) => Ending::Sized(half.sized()),
            // The escalation does not carry which rung of the walk
            // refused, and the rungs pass on different sets (the carrier
            // rung is a residual where the caller placed the point on
            // the surface; the period rung refuses a negative margin),
            // so no one margin gives a tolerance to tighten below.
            Self::Containment => Ending::Lever(
                "move the parts so they meet clearly inside or clearly outside that face's \
                 boundary",
                LeverPass::ByRung,
            ),
            Self::Crossing(decision) => Ending::Sized(decision.sized()),
            // Both definite verdicts pass (the vertices meet, or lie
            // apart); a negative distance is not a verdict.
            Self::VertexOnVertex => sized(
                "move the parts so their vertices clearly meet or lie clearly apart",
                "distance between the vertices",
                SizedPass::NonNegative,
            ),
            // The edge's certification decided this same margin (its
            // headroom to a full turn, metred at the radius) at this
            // band, so neither arm is one the user reaches: an arc
            // certified short of a full turn that now reads longer, or
            // undecided, is the kernel's. It ends as its definite
            // sibling, a broken classification invariant, does.
            Self::ArcSpan => Ending::Unsized(Unsized::Defect),
            Self::VolumeBackstop => Ending::Unsized(Unsized::Defect),
            // The arm passes on a positive length, and a zero-band one
            // is a size a smaller tolerance decides positive
            // (`geom_brep::enters`'s arm gate carries its margin).
            Self::LeverArm(gate) => gate.ending(),
            Self::Radius(radius) => Ending::Sized(radius.sized()),
            // A positive `|d⊥|²/2r` has roots to find; a zero one is a
            // constant residual, which every edge-sweep arm refuses at
            // the frontier. The margin is 1/m, ledger row F2 (debt
            // #214, `docs/predicate-dimension-audit.md`).
            Self::WallRoots(WallRung::AxisParallel) => Ending::Lever(
                "turn the edge clearly away from the direction of the cylinder's axis",
                LeverPass::PositiveNotALength,
            ),
            // Two roots pass at every arm; a miss passes where both ends
            // are off the wall and refuses where one is on it (a miss
            // cannot hold a zero endpoint); a zero is a tangency, refused
            // at the frontier. The margin `disc/(2r)²` is dimensionless,
            // ledger row F2 (debt #214).
            Self::WallRoots(WallRung::Discriminant) => Ending::Lever(
                "move the parts so the edge clearly crosses the wall or clearly misses it",
                LeverPass::ByArm,
            ),
            // Every rung of the quartic ladder asks whether the count is
            // certain, and a definite sign of each reads it; a zero-band
            // sign leaves it uncertain, refused at the frontier. The
            // margins are the resolvent's signs metered over the lever,
            // no size the user chose: clause-(i) debt, #214.
            Self::TorusRoots => Ending::Lever(
                "move the parts so the edge clearly crosses the torus or clearly misses it",
                LeverPass::NonZeroNotALength,
            ),
            Self::PierceCurvature => Ending::Sized(PIERCE_CURVATURE),
            Self::DirectionSense => Ending::Sized(DIRECTION_SENSE),
            // Its margin is an enclosure of the zero band, which no
            // smaller tolerance decides onto one side; a longer arm
            // lengthens every reading of the corner.
            Self::BisectorSide => sized(
                CORNER_EDGES,
                "reading of the corner's bisector",
                SizedPass::NonZero,
            ),
        }
    }

    /// The whole sentence an escalation of this decision renders, at the
    /// Boolean that built the geometry: the subject, the payload and the
    /// one ending the verdict gives.
    #[must_use]
    pub(crate) fn render(self, diag: &Indeterminate) -> String {
        let arm = RefusedArm::Undecided(diag);
        let (subject, payload) = (self.subject(), diag.payload());
        let ending = match self.ending() {
            Ending::Coincidence => {
                return format!(
                    "{subject} is undecided: {payload}, and the Boolean never snaps parts of \
                     the two solids together. Recourse: {COINCIDENCE_RECOURSE}"
                );
            }
            Ending::Sized(decision) => decision.recourse(arm, Reading::Build),
            Ending::Lever(lever, passes) => passes.recourse(lever, diag),
            Ending::Unsized(decision) => decision.recourse(arm, Reading::Build),
        };
        format!("{subject} is undecided: {payload}. {ending}")
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::boolean::{BooleanError, CarrierDesc, CarrierEqError, Operand, PlaneIdentity};
    use crate::entity::{EdgeKey, VertexKey};
    use crate::euler::EulerOpError;
    use crate::merge_faces::MergeCoplanarError;
    use crate::sector_shape::SectorRungKind;
    use crate::splitting::SplitReduceError;
    use geom_core::{Band, KERNEL_DEFECT_ENDING, MarginDiag, Point3, Tol, Vec3};
    use strum::IntoEnumIterator as _;
    use test_utils::refusal::{recourse_markers, stage_prefixes, subjectless_escalations};

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness band forms")
    }

    fn diag_of(margin: MarginDiag) -> Indeterminate {
        Indeterminate {
            margin,
            band: band(),
            predicate: Some("routing_probe"),
            terminal_sliver: false,
        }
    }

    /// The band's multiplier `K`, which turns a margin into the
    /// tolerance a smaller one than which decides it.
    fn k() -> f64 {
        band().escalate() / band().zero()
    }

    /// The point margin `diag` carries, for this row's arithmetic.
    fn point_margin(diag: &Indeterminate) -> f64 {
        diag.margin
            .diagnostic_f64_for_error_text()
            .value()
            .expect("a point margin")
    }

    /// The tolerance an ending offers to tighten below, where it offers
    /// one; `Some(None)` for an offer that names no value.
    fn offered_below(text: &str) -> Option<Option<f64>> {
        let (_, tail) = text.split_once("tighten the tolerance")?;
        Some(
            tail.strip_prefix(" below ")
                .and_then(|v| v.strip_suffix(" m"))
                .and_then(|v| v.parse::<f64>().ok()),
        )
    }

    /// The refusal-shape guard's three checks on one rendered text:
    /// exactly one recourse, a subject for every escalation payload,
    /// and no stage prefix other than the `filed` ones.
    fn short_of_the_guard(text: &str, filed: &[&str]) -> Vec<String> {
        let mut out = Vec::new();
        match recourse_markers(text) {
            1 => {}
            n => out.push(format!("{n} recourses, not one")),
        }
        for clause in subjectless_escalations(text) {
            out.push(format!("an escalation with no subject ({clause:?})"));
        }
        for prefix in stage_prefixes(text, &[]) {
            if !filed.iter().any(|f| prefix.starts_with(f)) {
                out.push(format!("the stage prefix {prefix:?}"));
            }
        }
        out
    }

    const NEIGHBOURS: &str = "whether two neighbouring faces of one operand lie on one plane";
    const NEIGHBOUR_ENDING: &str = "Recourse: merge the two faces into one first \
                                    (merge_coplanar_faces), or tilt one so they meet at a clear \
                                    angle along an edge clearly longer than the tolerance";

    const TUBE_LEVER: &str =
        "Recourse: reshape the torus so its tube is clearly thicker than the tolerance";
    const RING_LEVER: &str = "Recourse: make the tube radius clearly smaller than the ring radius";

    const CROSSING_LEVER_ENDING: &str =
        "Recourse: move the geometry so the crossing lands clearly away from the edge's ends";

    const WALL_LEVER: &str =
        "Recourse: move the parts so the edge clearly crosses the wall or clearly misses it";
    const PIERCE_LEVER: &str = "Recourse: make the edge leave the curved face at a steeper \
                                angle, or make the face curve less sharply there";

    /// The split door's own clause, a stage for a subject, filed with
    /// its owner: `work/reach/reach-refusals-short-of-the-shape-guard.md`.
    const SPLIT_DOOR_FILED: &str = "inserting the plane crossing on edge";

    /// **`split_edge`'s in-band interiority reads whole at every door
    /// that forwards it**, on a real raise: the split and the Boolean
    /// here (the blend's door is `sweep`'s, and its row is there). The
    /// subject is the decision; the one recourse is the lever every
    /// splitting door has and, the decision passing on a positive
    /// margin, the tolerance below which that margin is decided
    /// passing. No door is offered a declaration.
    #[test]
    fn the_split_param_escalation_reads_whole_at_every_splitting_door() {
        let raise = || {
            let cube = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness());
            let mut body = cube.body;
            let edge = cube.mevs[0].edge;
            let b = band();
            let err = body
                .split_edge(edge, (b.zero() + b.escalate()) * 0.5, Tol::witness())
                .unwrap_err();
            let EulerOpError::SplitParamEscalated { diag, .. } = err else {
                panic!("the band-midpoint split escalates: {err:?}");
            };
            (err, point_margin(&diag))
        };
        let edge = EdgeKey::default();
        let (operator, margin) = raise();
        assert!(margin > 0.0, "the midpoint margin is positive: {margin:e}");
        let rendered = [
            ("the operator", operator.to_string()),
            (
                "the split",
                SplitReduceError::CrossingInsertion {
                    edge,
                    endpoints: (VertexKey::default(), VertexKey::default()),
                    source: raise().0,
                }
                .to_string(),
            ),
            (
                "the Boolean",
                BooleanError::CrossingInsertion {
                    operand: Operand::A,
                    edge,
                    source: raise().0,
                }
                .to_string(),
            ),
        ];
        for (door, text) in rendered {
            let problems = short_of_the_guard(&text, &[SPLIT_DOOR_FILED]);
            assert!(problems.is_empty(), "{door}: {problems:?}: {text}");
            assert!(
                text.contains(
                    "whether a crossing lands strictly inside its edge is undecided: margin "
                ) && text.contains(CROSSING_LEVER_ENDING)
                    && !text.contains("declare"),
                "{door} states the decision and the lever it has, and no declaration: {text}"
            );
            assert_eq!(
                offered_below(&text),
                Some(Some(margin / k())),
                "{door} offers the tolerance the margin gives: {text}"
            );
        }
    }

    /// **Every decision, by construction.** The top-level variants come
    /// from the compiler (`BooleanDecisionKind::iter`), and each kind's
    /// concrete decisions from a match that must name every kind, over
    /// the nested rungs' own compiler-derived lists: a new decision, or
    /// a new rung under one, is a compile error or a new row here, never
    /// a silent pass.
    fn every_decision() -> Vec<BooleanDecision> {
        BooleanDecisionKind::iter()
            .flat_map(|kind| match kind {
                BooleanDecisionKind::Coincidence => Coincide::iter()
                    .flat_map(|which| {
                        every_read().map(move |read| BooleanDecision::Coincidence(which, read))
                    })
                    .collect(),
                BooleanDecisionKind::PlaneOrientation => vec![BooleanDecision::PlaneOrientation],
                BooleanDecisionKind::DeclaredParallel => vec![BooleanDecision::DeclaredParallel],
                BooleanDecisionKind::Neighbours => {
                    PlaneRung::iter().map(BooleanDecision::Neighbours).collect()
                }
                BooleanDecisionKind::Corner => SectorRungKind::iter()
                    .flat_map(|rung| match rung {
                        SectorRungKind::Arm => vec![SectorRung::Arm],
                        SectorRungKind::Straight => [false, true]
                            .map(|full_circle| SectorRung::Straight { full_circle })
                            .to_vec(),
                    })
                    .map(BooleanDecision::Corner)
                    .collect(),
                BooleanDecisionKind::PierceOnFace => vec![BooleanDecision::PierceOnFace],
                BooleanDecisionKind::Torus => TorusConvention::iter()
                    .map(BooleanDecision::Torus)
                    .collect(),
                BooleanDecisionKind::Containment => vec![BooleanDecision::Containment],
                BooleanDecisionKind::Crossing => CrossingDecision::iter()
                    .map(BooleanDecision::Crossing)
                    .collect(),
                BooleanDecisionKind::VertexOnVertex => vec![BooleanDecision::VertexOnVertex],
                BooleanDecisionKind::SplitPointOnCircle => {
                    vec![BooleanDecision::SplitPointOnCircle]
                }
                BooleanDecisionKind::ArcSpan => vec![BooleanDecision::ArcSpan],
                BooleanDecisionKind::VolumeBackstop => vec![BooleanDecision::VolumeBackstop],
                BooleanDecisionKind::LeverArm => {
                    LeverArm::iter().map(BooleanDecision::LeverArm).collect()
                }
                BooleanDecisionKind::Radius => {
                    SectionRadius::iter().map(BooleanDecision::Radius).collect()
                }
                BooleanDecisionKind::WallRoots => {
                    WallRung::iter().map(BooleanDecision::WallRoots).collect()
                }
                BooleanDecisionKind::TorusRoots => vec![BooleanDecision::TorusRoots],
                BooleanDecisionKind::PierceCurvature => vec![BooleanDecision::PierceCurvature],
                BooleanDecisionKind::DirectionSense => vec![BooleanDecision::DirectionSense],
                BooleanDecisionKind::BisectorSide => vec![BooleanDecision::BisectorSide],
            })
            .collect()
    }

    /// Every read a door can state: each class a spent declaration can
    /// carry comes from [`ContactClass::ALL`], and the match names every
    /// variant, so a new read is a compile error here.
    fn every_read() -> impl Iterator<Item = DeclarationRead> + Clone {
        let witness = |read: DeclarationRead| match read {
            DeclarationRead::Settles | DeclarationRead::Spent(_) | DeclarationRead::Moot => read,
        };
        [DeclarationRead::Settles, DeclarationRead::Moot]
            .into_iter()
            .chain(
                ContactClass::ALL
                    .iter()
                    .copied()
                    .map(DeclarationRead::Spent),
            )
            .map(witness)
    }

    /// How a decision's escalation must end, written independently of
    /// the table.
    enum Ending {
        /// The coincidence's own sentence.
        Coincidence,
        /// The lever, and the pass set the table must state, from which
        /// the margins a smaller tolerance decides passing follow.
        Sized(&'static str, SizedPass),
        /// The lever alone, on every margin, with the pass set the table
        /// states as the reason.
        Lever(&'static str, LeverPass),
        /// The defect ending.
        Defect,
    }

    /// Each coincidence's subject, as a literal.
    fn coincide_subject(which: Coincide) -> &'static str {
        match which {
            Coincide::Planes => "whether a face of each solid lies on one plane",
            Coincide::Carriers => "whether a face of each solid lies on one surface",
            Coincide::VertexOnFace => "whether a vertex of one solid lies on a face of the other",
            Coincide::EdgeOnPlane => {
                "whether a curved edge of one solid lies in, grazes or misses the plane of a face \
                 of the other"
            }
            Coincide::EdgeOnCurvedFace => {
                "whether an edge of one solid clears a curved face of the other or lies on it"
            }
            Coincide::SectorSide => {
                "which side of a face of the other solid a corner's edge leaves on"
            }
            Coincide::TangentSide => {
                "which way a face of one solid curves away from a face of the other that it \
                 touches"
            }
            Coincide::Sectors => "how two corners of the two solids overlap where they meet",
            Coincide::EdgeOnEdge => "whether an edge of one solid runs along an edge of the other",
            Coincide::TangentLocus => "where two faces of the two solids touch tangentially",
            Coincide::SeamWedge => {
                "whether the two faces at a seam edge cross there or touch tangentially"
            }
            Coincide::Sphere => "whether a sphere face of one solid meets the other solid",
            Coincide::Section => "where the surfaces of a face of each solid meet",
            Coincide::Join => "how the sections' ends pair up where the two solids meet",
            Coincide::Contact => {
                "whether a declared contact holds along its witness or its shared rim"
            }
        }
    }

    /// Each decision's subject and ending, as literals: an independent
    /// statement of the words `subject` and `ending` must produce.
    fn want(decision: BooleanDecision) -> (&'static str, Ending) {
        const CORNER: &str = "Recourse: reshape that corner so its edges are clearly longer than \
                              the tolerance and clearly not in line";
        const STRAIGHT: &str = "whether a corner is straight or folds back on itself";
        const LONGER: &str = "Recourse: make the edges at the corner where the two faces meet \
                              clearly longer than the tolerance";
        match decision {
            BooleanDecision::Coincidence(which, DeclarationRead::Settles) => {
                (coincide_subject(which), Ending::Coincidence)
            }
            BooleanDecision::Coincidence(
                which,
                DeclarationRead::Spent(_) | DeclarationRead::Moot,
            ) => (
                coincide_subject(which),
                match which {
                    Coincide::Planes => Ending::Sized(
                        "Recourse: tilt one face so the two are clearly parallel or clearly not",
                        SizedPass::NonNegative,
                    ),
                    Coincide::TangentSide => Ending::Sized(
                        "Recourse: make one face clearly curve away from the other where they \
                         touch, or make both curve alike there",
                        SizedPass::AnySign,
                    ),
                    Coincide::SeamWedge => Ending::Sized(
                        "Recourse: move the geometry so the faces at that seam meet either \
                         clearly creased or clearly smooth",
                        SizedPass::NonNegative,
                    ),
                    _ => Ending::Sized(
                        "Recourse: move the parts so they clearly meet or clearly stand apart \
                         there",
                        SizedPass::AnySign,
                    ),
                },
            ),
            BooleanDecision::DirectionSense => (
                "whether two parallel directions at a corner point the same way or opposite ways",
                Ending::Sized(LONGER, SizedPass::NonZero),
            ),
            BooleanDecision::BisectorSide => (
                "which side of a face a corner's bisector leaves on",
                Ending::Sized(LONGER, SizedPass::NonZero),
            ),
            BooleanDecision::LeverArm(LeverArm::SectorSide) => (
                "whether an edge at a corner is long enough to read which side of a face it \
                 leaves on",
                Ending::Sized(LONGER, SizedPass::Positive),
            ),
            BooleanDecision::LeverArm(LeverArm::SectorCurving) => (
                "whether a corner's edges are long enough to read which way a face curves there",
                Ending::Sized(LONGER, SizedPass::Positive),
            ),
            BooleanDecision::LeverArm(LeverArm::Seam) => (
                "whether a seam edge is long enough, for how its faces curve, to measure the \
                 angle between them",
                Ending::Sized(
                    "Recourse: move the geometry so that edge is clearly longer, and its faces \
                     curve less tightly there",
                    SizedPass::Positive,
                ),
            ),
            BooleanDecision::Radius(SectionRadius::Cylinder) => (
                "whether a cylinder's radius is positive",
                Ending::Sized(
                    "Recourse: make the cylinder's radius clearly larger than the tolerance",
                    SizedPass::Positive,
                ),
            ),
            BooleanDecision::Radius(SectionRadius::Sphere) => (
                "whether a sphere's radius is positive",
                Ending::Sized(
                    "Recourse: make the sphere's radius clearly larger than the tolerance",
                    SizedPass::Positive,
                ),
            ),
            BooleanDecision::WallRoots(WallRung::AxisParallel) => (
                "whether an edge runs parallel to a cylinder's axis",
                Ending::Lever(
                    "Recourse: turn the edge clearly away from the direction of the cylinder's \
                     axis",
                    LeverPass::PositiveNotALength,
                ),
            ),
            BooleanDecision::WallRoots(WallRung::Discriminant) => (
                "whether an edge crosses a cylinder wall, grazes it or misses it",
                Ending::Lever(WALL_LEVER, LeverPass::ByArm),
            ),
            BooleanDecision::TorusRoots => (
                "how many times an edge crosses a torus",
                Ending::Lever(
                    "Recourse: move the parts so the edge clearly crosses the torus or clearly \
                     misses it",
                    LeverPass::NonZeroNotALength,
                ),
            ),
            BooleanDecision::PierceCurvature => (
                "whether an edge leaves a curved face steeply enough, for how sharply the face \
                 bends, to read which side it goes",
                Ending::Sized(PIERCE_LEVER, SizedPass::Positive),
            ),
            BooleanDecision::Corner(SectorRung::Arm) => (
                "whether a corner's edges are long enough to measure its angle over",
                Ending::Sized(CORNER, SizedPass::Positive),
            ),
            BooleanDecision::Corner(SectorRung::Straight { full_circle: false }) => {
                (STRAIGHT, Ending::Sized(CORNER, SizedPass::Negative))
            }
            BooleanDecision::Corner(SectorRung::Straight { full_circle: true }) => {
                (STRAIGHT, Ending::Sized(CORNER, SizedPass::AnySign))
            }
            BooleanDecision::PierceOnFace => (
                "whether a point lies on a curved face, so the face's normal can be read there",
                Ending::Defect,
            ),
            BooleanDecision::PlaneOrientation => (
                "whether the two planes face the same way or opposite ways",
                Ending::Sized(LONGER, SizedPass::NonZero),
            ),
            BooleanDecision::DeclaredParallel => {
                ("whether the two planes are parallel", Ending::Defect)
            }
            BooleanDecision::Neighbours(PlaneRung::Parallel) => (
                NEIGHBOURS,
                Ending::Sized(NEIGHBOUR_ENDING, SizedPass::Positive),
            ),
            BooleanDecision::Neighbours(PlaneRung::Orientation) => (
                NEIGHBOURS,
                Ending::Lever(NEIGHBOUR_ENDING, LeverPass::Never),
            ),
            BooleanDecision::Torus(TorusConvention::Tube) => (
                "whether a torus's tube radius is positive",
                Ending::Sized(TUBE_LEVER, SizedPass::Positive),
            ),
            BooleanDecision::Torus(TorusConvention::Ring) => (
                "whether a torus's tube radius is smaller than its ring radius",
                Ending::Sized(RING_LEVER, SizedPass::Positive),
            ),
            BooleanDecision::Containment => (
                "whether a point lies inside a face, on its boundary, or outside it",
                Ending::Lever(
                    "Recourse: move the parts so they meet clearly inside or clearly outside \
                     that face's boundary",
                    LeverPass::ByRung,
                ),
            ),
            BooleanDecision::Crossing(CrossingDecision::OnEdge) => (
                "whether a crossing lands strictly inside its edge",
                Ending::Sized(CROSSING_LEVER_ENDING, SizedPass::AnySign),
            ),
            BooleanDecision::Crossing(CrossingDecision::Order) => (
                "which of two crossings on an edge comes first",
                Ending::Sized(
                    "Recourse: move the geometry so the two crossings on that edge lie clearly \
                     apart",
                    SizedPass::AnySign,
                ),
            ),
            BooleanDecision::VertexOnVertex => (
                "whether a vertex of one solid coincides with one of the other",
                Ending::Sized(
                    "Recourse: move the parts so their vertices clearly meet or lie clearly \
                     apart",
                    SizedPass::NonNegative,
                ),
            ),
            BooleanDecision::SplitPointOnCircle => (
                "whether a split point lies on the circle it was placed on",
                Ending::Defect,
            ),
            BooleanDecision::ArcSpan => {
                ("whether an arc stays short of a full turn", Ending::Defect)
            }
            BooleanDecision::VolumeBackstop => (
                "whether the result's volume agrees with its operands'",
                Ending::Defect,
            ),
        }
    }

    /// The tolerance a smaller one than which decides `margin` on a side
    /// `passes` accepts: a point margin at `|m|/K`, an enclosure with
    /// both ends on one such side at its nearer end's.
    fn expected_offer(margin: MarginDiag, passes: SizedPass) -> Option<f64> {
        let on = |v: f64| {
            v != 0.0
                && match passes {
                    SizedPass::Positive | SizedPass::NonNegative => v > 0.0,
                    SizedPass::Negative => v < 0.0,
                    SizedPass::NonZero | SizedPass::AnySign => true,
                }
        };
        match margin.diagnostic_f64_for_error_text() {
            geom_core::ErrorTextReading::Value(m) => on(m).then(|| m.abs() / k()),
            geom_core::ErrorTextReading::Enclosure { lo, hi } => {
                (on(lo) && on(hi) && (lo > 0.0) == (hi > 0.0)).then(|| lo.abs().min(hi.abs()) / k())
            }
            geom_core::ErrorTextReading::Invalid => None,
        }
    }

    /// **`BooleanError::Escalated` ends as its decision and verdict
    /// give**, for every decision, on in-band margins of each sign (a
    /// point and an enclosure), an enclosure across zero, a signed zero,
    /// and an `INVALID` margin (the enclosure and zero rows are the
    /// review's `probe_c7_shape_guard_enclosures`):
    ///
    /// - every sentence opens on its decision's own subject, written
    ///   here as a literal, and passes the refusal-shape guard;
    /// - a coincidence composes the coincidence sentence, and only it
    ///   offers a declaration;
    /// - a decision on a size names its lever and, on an in-band margin
    ///   on a side it passes on, the tolerance below which that margin is
    ///   decided passing; elsewhere no tolerance, and an `INVALID` margin
    ///   adds the unreadable-margin note;
    /// - a family the escalation does not tell apart names its lever
    ///   alone;
    /// - a residual or a kernel self-check ends as a defect and never
    ///   names the tolerance.
    #[test]
    fn every_escalation_ends_as_its_decision_and_verdict_give() {
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        let mid = (z + e) / 2.0;
        for decision in every_decision() {
            for margin in [
                MarginDiag::value(mid),
                MarginDiag::value(-mid),
                MarginDiag::enclosure(2.0 * z, 0.5 * e),
                MarginDiag::enclosure(-0.5 * e, -2.0 * z),
                MarginDiag::enclosure(-2.0 * z, 3.0 * z),
                MarginDiag::value(0.0),
                MarginDiag::value(-0.0),
                MarginDiag::INVALID,
            ] {
                let diag = diag_of(margin);
                let text = BooleanError::Escalated { decision, diag }.to_string();
                let label = format!("{decision:?} at {margin}");
                let problems = short_of_the_guard(&text, &[]);
                assert!(problems.is_empty(), "{label}: {problems:?}: {text}");
                assert!(
                    !text.contains("routing_probe"),
                    "{label}: the routing name stays out: {text}"
                );
                assert_eq!(
                    text.contains("declare the coincidence"),
                    matches!(
                        decision,
                        BooleanDecision::Coincidence(_, DeclarationRead::Settles)
                    ),
                    "{label}: only a coincidence a declaration read ahead would settle offers \
                     one: {text}"
                );
                let (subject, ending) = want(decision);
                let head = format!("{subject} is undecided: {}. ", diag.payload());
                let tail = text.strip_prefix(&head);
                match ending {
                    Ending::Coincidence => assert_eq!(
                        text,
                        format!(
                            "{subject} is undecided: {}, and the Boolean never snaps parts of \
                             the two solids together. Recourse: declare the coincidence, move \
                             the geometry, or lower the tolerance",
                            diag.payload()
                        ),
                        "{label}"
                    ),
                    Ending::Sized(lever, passes) => {
                        assert!(
                            matches!(decision.ending(), super::Ending::Sized(d) if d.passes == passes),
                            "{label}: the pass set the table states"
                        );
                        assert!(
                            tail.is_some_and(|t| t.starts_with(lever)),
                            "{label}: its subject, then its lever: {text}"
                        );
                        assert_eq!(
                            text.ends_with(UNREADABLE_MARGIN_NOTE),
                            margin.is_invalid(),
                            "{label}: {text}"
                        );
                        assert_eq!(
                            offered_below(&text),
                            expected_offer(margin, passes).map(Some),
                            "{label}: {text}"
                        );
                    }
                    Ending::Lever(lever, passes) => {
                        assert!(
                            matches!(decision.ending(), super::Ending::Lever(_, p) if p == passes),
                            "{label}: the pass set the table states"
                        );
                        let want = if margin.is_invalid() {
                            format!("{lever}; {UNREADABLE_MARGIN_NOTE}")
                        } else {
                            lever.to_owned()
                        };
                        assert_eq!(tail, Some(want.as_str()), "{label}: {text}");
                    }
                    Ending::Defect => assert_eq!(
                        tail,
                        Some(KERNEL_DEFECT_ENDING),
                        "{label}: its subject, then the defect ending: {text}"
                    ),
                }
            }
        }
    }

    /// **A containment escalation on a residual rung names no
    /// tolerance**, on a real raise: a point off a cylinder wall by an
    /// in-band distance escalates the carrier rung, which passes only
    /// on the surface where the crossing layer placed the point there,
    /// so its margin is a miss, not a size. The containment family ends
    /// on its lever alone. (The review's
    /// `probe_c1_containment_residual_rung_constructed` rendered "tighten
    /// the tolerance below …" here.)
    #[test]
    fn a_containment_escalation_on_a_residual_rung_names_its_lever_alone() {
        use crate::test_support_fixtures::{CylFrame, brick, cyl_wall_sheet};
        let tol = Tol::witness();
        let b = band();
        let mut body: crate::body::Body<f64> = brick((10.0, 11.0), (10.0, 11.0), (0.0, 1.0), tol);
        let wall = cyl_wall_sheet(
            &mut body,
            CylFrame::canonical(1.0),
            None,
            (0.5, 2.0),
            (0.0, 1.0),
            tol,
        );
        let r = 1.0 + (b.zero() + b.escalate()) / 2.0;
        let p = Point3::new(r * 1.2_f64.cos(), r * 1.2_f64.sin(), 0.5);
        let diag = match crate::boolean::contain::curved_face_placement(&body, wall, p, b) {
            Err(crate::boolean::ContainError::Escalated(diag)) => diag,
            other => panic!("an in-band point off the wall escalates: {other:?}"),
        };
        assert_eq!(diag.predicate, Some("bool_curved_contain_carrier"));
        let text = BooleanError::Escalated {
            decision: BooleanDecision::Containment,
            diag,
        }
        .to_string();
        assert!(
            text.ends_with(
                "Recourse: move the parts so they meet clearly inside or clearly outside that \
                 face's boundary"
            ) && !text.contains("tolerance below"),
            "{text}"
        );
    }

    /// **The conic root lane ends alike at the split and the Boolean**,
    /// rung by rung, through the one routing ([`ConicRootFault::decision`]):
    /// a crossing rung renders the same sentence at both doors, its
    /// subject and its decision's ending; a rung that asks whether the
    /// plane coincides with the conic states its own subject at the split,
    /// with the split's levers and no declaration, and is the
    /// coincidence at the Boolean, where the sweep asks it ahead of any
    /// declaration and so offers none either.
    #[test]
    fn the_conic_root_lane_ends_alike_at_the_split_and_the_boolean() {
        let b = band();
        let diag = diag_of(MarginDiag::value((b.zero() + b.escalate()) / 2.0));
        let faults = [
            (
                ConicRootFault::PlaneParallel(diag),
                "whether a curved edge's plane is parallel to the plane that cuts it",
            ),
            (
                ConicRootFault::BellyGraze(diag),
                "whether a plane cuts a curved edge, grazes it or misses it",
            ),
            (
                ConicRootFault::CrossingInterior(diag),
                "whether a crossing lands strictly inside its edge",
            ),
            (
                ConicRootFault::RootOrder(diag),
                "which of two crossings on an edge comes first",
            ),
        ];
        for (fault, subject) in faults {
            let split = SplitReduceError::CrossingEscalated {
                edge: EdgeKey::default(),
                fault,
            }
            .to_string();
            let boolean = BooleanError::Escalated {
                decision: BooleanDecision::of_conic_root(fault),
                diag,
            }
            .to_string();
            for text in [&split, &boolean] {
                let problems = short_of_the_guard(text, &[]);
                assert!(problems.is_empty(), "{fault:?}: {problems:?}: {text}");
            }
            assert!(
                split.starts_with(&format!("{subject} is undecided: {}. ", diag.payload())),
                "{fault:?}: {split}"
            );
            match fault.decision() {
                Some(_) => assert_eq!(split, boolean, "{fault:?}"),
                None => {
                    assert!(
                        split.ends_with(
                            "Recourse: move the split plane or the geometry, or lower the \
                             tolerance"
                        ) && !split.contains("declare"),
                        "{fault:?}: {split}"
                    );
                    assert!(
                        boolean.starts_with(
                            "whether a curved edge of one solid lies in, grazes or misses the \
                             plane of a face of the other is undecided: "
                        ) && !boolean.contains("declare"),
                        "{fault:?}: the sweep reads no declaration ahead of it: {boolean}"
                    );
                }
            }
        }
    }

    /// The recourse of a contradicted declaration, as `contact` states
    /// it for every contradiction.
    const CONTRADICTED: &str =
        "Recourse: correct or remove the declaration, or move the geometry so it holds";

    /// Each contradiction's clause, and the predicate whose definite
    /// verdict raises it, written independently of the enum.
    const FACTS: &[(Contradiction, &str, &str)] = &[
        (
            Contradiction::PlanesNotParallel,
            "bool_plane_parallel",
            "the declared planes are not parallel",
        ),
        (
            Contradiction::PlanesApart,
            "bool_plane_offset",
            "the declared planes are parallel but apart",
        ),
        (
            Contradiction::KindsDiffer,
            "carrier_kind",
            "the declared faces are different kinds of surface",
        ),
        (
            Contradiction::SphereCentresDiffer,
            "carrier_sphere_center",
            "the declared spheres' centres differ",
        ),
        (
            Contradiction::SphereRadiiDiffer,
            "carrier_sphere_radius",
            "the declared spheres' radii differ",
        ),
        (
            Contradiction::CylinderAxesNotParallel,
            "carrier_cyl_axis_parallel",
            "the declared cylinders' axes are not parallel",
        ),
        (
            Contradiction::CylinderAxesApart,
            "carrier_cyl_axis_offset",
            "the declared cylinders' axes are parallel but apart",
        ),
        (
            Contradiction::CylinderRadiiDiffer,
            "carrier_cyl_radius",
            "the declared cylinders' radii differ",
        ),
        (
            Contradiction::TorusAxesNotParallel,
            "carrier_torus_axis_parallel",
            "the declared tori's axes are not parallel",
        ),
        (
            Contradiction::TorusCentresDiffer,
            "carrier_torus_center",
            "the declared tori's centres differ",
        ),
        (
            Contradiction::TorusMajorRadiiDiffer,
            "carrier_torus_major_radius",
            "the declared tori's major radii differ",
        ),
        (
            Contradiction::TorusTubeRadiiDiffer,
            "carrier_torus_minor_radius",
            "the declared tori's tube radii differ",
        ),
    ];

    /// A declared pair of `c1` and `c2`, verified by the real rung
    /// (`carrier_eq`, which `recl` and `vtxfac` call and `plane_eq`
    /// serves for planes): the contradiction it raises.
    fn contradicted(c1: CarrierDesc<f64>, c2: CarrierDesc<f64>) -> (Contradiction, Indeterminate) {
        let id = PlaneIdentity {
            s1: None,
            s2: None,
            declared: true,
        };
        match crate::boolean::carrier_eq(&c1, &c2, id, 1.0, band()) {
            Err(CarrierEqError::Contradicted { fact, diag }) => (fact, diag),
            other => panic!("a declared pair this far apart contradicts: {other:?}"),
        }
    }

    /// **A contradicted declaration names the fact that contradicted
    /// it**, one clause per rung, on the verdict the real rung raises
    /// for a pair built to trip that rung alone — two cylinders of
    /// different radii among them, the non-planar pair `recl` raises.
    /// The verdict is definite: no payload, no declaration offered, no
    /// tolerance, one recourse.
    #[test]
    fn every_contradiction_names_the_fact_that_contradicted() {
        let p = Point3::new;
        let (x, z) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let o = p(0.0, 0.0, 0.0);
        let plane = |origin, normal| CarrierDesc::Plane { origin, normal };
        let sphere = |center, radius| CarrierDesc::Sphere {
            center,
            radius,
            outward: true,
        };
        let cylinder = |origin, axis, radius| CarrierDesc::Cylinder {
            origin,
            axis,
            radius,
            outward: true,
        };
        let torus = |center, axis, major_radius, minor_radius| CarrierDesc::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            outward: true,
        };
        let pairs = [
            (plane(o, z), plane(o, x)),
            (plane(o, z), plane(p(0.0, 0.0, 1.0), z)),
            (plane(o, z), sphere(o, 1.0)),
            (sphere(o, 1.0), sphere(p(1.0, 0.0, 0.0), 1.0)),
            (sphere(o, 1.0), sphere(o, 2.0)),
            (cylinder(o, z, 1.0), cylinder(o, x, 1.0)),
            (cylinder(o, z, 1.0), cylinder(p(1.0, 0.0, 0.0), z, 1.0)),
            (cylinder(o, z, 1.0), cylinder(o, z, 2.0)),
            (torus(o, z, 2.0, 0.5), torus(o, x, 2.0, 0.5)),
            (torus(o, z, 2.0, 0.5), torus(p(0.0, 0.0, 1.0), z, 2.0, 0.5)),
            (torus(o, z, 2.0, 0.5), torus(o, z, 3.0, 0.5)),
            (torus(o, z, 2.0, 0.5), torus(o, z, 2.0, 0.25)),
        ];
        assert_eq!(pairs.len(), FACTS.len());
        for ((c1, c2), &(want, name, clause)) in pairs.into_iter().zip(FACTS) {
            let (fact, diag) = contradicted(c1, c2);
            assert_eq!(
                (fact, diag.predicate),
                (want, Some(name)),
                "the pair trips {name}"
            );
            let planar = matches!(
                fact,
                Contradiction::PlanesNotParallel | Contradiction::PlanesApart
            );
            let mut texts = vec![(
                BooleanError::DeclarationContradicted { fact }.to_string(),
                "the Boolean",
            )];
            if planar {
                texts.push((
                    MergeCoplanarError::DeclarationContradicted { fact }.to_string(),
                    "the merge",
                ));
            }
            for (text, who) in texts {
                assert_eq!(
                    text,
                    format!(
                        "a declared coincidence contradicts the geometry: {clause}, and {who} \
                         never glues a lie. {CONTRADICTED}"
                    ),
                    "{who}, {name}"
                );
                let problems = short_of_the_guard(&text, &[]);
                assert!(problems.is_empty(), "{who}, {name}: {problems:?}: {text}");
                assert!(
                    !text.contains("tolerance"),
                    "{who}, {name}: a definite verdict names no tolerance: {text}"
                );
            }
        }
    }

    /// **The merge meets a contradicted declaration on a real raise**:
    /// two faces of a brick that meet at an edge, declared one surface.
    #[test]
    fn a_declared_pair_of_meeting_faces_is_contradicted_at_the_merge() {
        let tol = Tol::witness();
        let mut body =
            crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let normal = |b: &crate::body::Body<f64>, s| match b.get_surface(s) {
            Some(&crate::Surface::Plane { normal, .. }) => normal,
            other => panic!("a brick face is a plane: {other:?}"),
        };
        let surfaces: Vec<_> = body.faces().map(|(_, f)| f.surface).collect();
        let first = surfaces[0];
        let meeting = *surfaces
            .iter()
            .find(|&&s| normal(&body, s).dot(normal(&body, first)).abs() < 0.5)
            .expect("a brick face meets four others");
        let err = body
            .merge_coplanar_faces_declared(&[(first, meeting)], tol)
            .expect_err("two meeting faces are not one plane");
        assert!(
            matches!(
                err,
                MergeCoplanarError::DeclarationContradicted {
                    fact: Contradiction::PlanesNotParallel
                }
            ),
            "the declared rung contradicts on parallelism: {err:?}"
        );
        let text = err.to_string();
        assert_eq!(
            text,
            format!(
                "a declared coincidence contradicts the geometry: the declared planes are not \
                 parallel, and the merge never glues a lie. {CONTRADICTED}"
            )
        );
        let wrapped = BooleanError::Merge(err).to_string();
        let problems = short_of_the_guard(&wrapped, &[]);
        assert!(problems.is_empty(), "{problems:?}: {wrapped}");
    }

    /// A one-face skeletal body whose face carries `surface`.
    fn face_on(surface: crate::Surface<f64>) -> (crate::body::Body<f64>, crate::entity::FaceKey) {
        let st = crate::fixtures::mvfs_state();
        let mut body = st.body;
        body.set_face_surface(
            st.face,
            crate::euler::FaceSurface::New {
                surface,
                sense: true,
            },
        )
        .expect("a skeletal face takes any surface");
        (body, st.face)
    }

    /// **A pierced torus tells one story per half of its ring
    /// convention, on every arm** (D4 ¶1 (iv)), each a real raise: the
    /// pierce point's normal door refuses a torus face and the Boolean
    /// routes that refusal as `vtxfac` does. In band, decided at zero
    /// (with a margin and exactly on), and definitely negative, each
    /// half names its own lever and passes the refusal-shape guard;
    /// an arm with a positive margin offers the tolerance the margin
    /// gives; no arm offers a declaration or calls the torus a pairing
    /// not supported yet. (No offer on a nonpositive margin is not
    /// asserted: `Refused::Negative` is sign-certain and a zero margin
    /// leaves no size, so no single edit makes one offer.)
    #[test]
    fn a_pierced_torus_tells_one_story_per_convention_half() {
        use crate::face_normal::face_outward_normal_at;
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        let mid = (z + e) / 2.0;
        // (half, R, r, the half's margin, whether the arm is in band).
        let rows = [
            (TorusConvention::Tube, 0.75, mid, mid, true),
            (TorusConvention::Tube, 0.75, 0.5 * z, 0.5 * z, false),
            (TorusConvention::Tube, 0.75, -0.3, -0.3, false),
            (TorusConvention::Ring, 0.5 + mid, 0.5, 0.5 + mid - 0.5, true),
            (
                TorusConvention::Ring,
                0.5 + 0.5 * z,
                0.5,
                0.5 + 0.5 * z - 0.5,
                false,
            ),
            (TorusConvention::Ring, 0.5, 0.5, 0.0, false),
            (TorusConvention::Ring, 0.4, 0.5, 0.4 - 0.5, false),
        ];
        for (half, big_r, r, margin, in_band) in rows {
            let label = format!("{half:?} at R = {big_r}, r = {r}");
            let (body, face) = face_on(crate::Surface::Torus {
                center: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                major_radius: big_r,
                minor_radius: r,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            });
            let refusal = face_outward_normal_at(&body, face, Point3::new(big_r + r, 0.0, 0.0), b)
                .expect_err("a torus outside the ring convention has no normal");
            let err = BooleanError::of_pierced_normal(refusal, Operand::B, face);
            match (&err, in_band) {
                (BooleanError::Escalated { decision, .. }, true) => {
                    assert_eq!(*decision, BooleanDecision::Torus(half), "{label}");
                }
                (BooleanError::DegenerateTorus { convention, .. }, false) => {
                    assert_eq!(*convention, half, "{label}");
                }
                _ => panic!("{label}: the arm's own refusal: {err:?}"),
            }
            let text = err.to_string();
            let problems = short_of_the_guard(&text, &[]);
            assert!(problems.is_empty(), "{label}: {problems:?}: {text}");
            let lever = match half {
                TorusConvention::Tube => TUBE_LEVER,
                TorusConvention::Ring => RING_LEVER,
            };
            assert!(
                text.contains(lever) && !text.contains("declare") && !text.contains("supported"),
                "{label}: the half's one lever, no declaration, no gap: {text}"
            );
            if margin > 0.0 {
                assert_eq!(
                    offered_below(&text),
                    Some(Some(margin / k())),
                    "{label}: the tolerance a positive margin gives: {text}"
                );
            }
        }
    }

    /// Two planes through `z = 1`, the second facing `sign` times the
    /// first's normal.
    fn planes(
        sign: f64,
    ) -> (
        crate::boolean::PlaneDesc<f64>,
        crate::boolean::PlaneDesc<f64>,
    ) {
        let plane = |n: f64| crate::boolean::PlaneDesc {
            origin: Point3::new(0.0, 0.0, 1.0),
            normal: Vec3::new(0.0, 0.0, n),
        };
        (plane(1.0), plane(sign))
    }

    const DECLARED: PlaneIdentity<'static> = PlaneIdentity {
        s1: None,
        s2: None,
        declared: true,
    };

    /// **At the Boolean's cross-operand doors a plane pair's orientation
    /// refusal names its own decision, the move that reaches a pass, and
    /// no declaration** (D4 ¶1 (i)), each a real raise: two coincident
    /// planes, facing the same way and opposite ways, compared at an arm
    /// that puts the orientation margin (the normals' cosine at the arm)
    /// in the zero band and in the ambiguity band, by the declared rung
    /// and by the undeclared ladder. The zero verdict carries the margin
    /// the rung decided. Both definite signs pass there, so either
    /// sign's margin offers the tolerance it gives; the lever lengthens
    /// the arm, the one thing an undecided margin measures.
    #[test]
    fn the_boolean_orientation_refusal_names_the_arm_and_offers_no_declaration() {
        use crate::boolean::{PlaneDoor, PlaneEqError, PlaneRung, oriented_plane_eq};
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        for (arm, zero) in [(0.5 * z, true), ((z + e) / 2.0, false)] {
            for sign in [1.0, -1.0] {
                let (p1, p2) = planes(sign);
                for id in [DECLARED, PlaneIdentity::NONE] {
                    let label = format!("arm {arm:e}, facing {sign}, declared {}", id.declared);
                    let err = oriented_plane_eq(&p1, &p2, id, arm, b)
                        .expect_err("an orientation margin this small refuses");
                    let PlaneEqError::Escalated {
                        rung: PlaneRung::Orientation,
                        diag,
                    } = err
                    else {
                        panic!("{label}: the orientation rung refuses: {err:?}");
                    };
                    assert_eq!(
                        point_margin(&diag),
                        sign * arm,
                        "{label}: the margin the rung decided rides the payload"
                    );
                    let text = BooleanError::plane_identity(
                        PlaneRung::Orientation,
                        PlaneDoor::of(id.declared.then_some(ContactClass::Rest)),
                        diag,
                    )
                    .to_string();
                    let problems = short_of_the_guard(&text, &[]);
                    assert!(problems.is_empty(), "{label}: {problems:?}: {text}");
                    assert!(
                        text.starts_with(
                            "whether the two planes face the same way or opposite ways is \
                             undecided: "
                        ) && text.contains(
                            "Recourse: make the edges at the corner where the two faces meet \
                             clearly longer than the tolerance"
                        ) && text.contains(if zero {
                            "lies within the zero band"
                        } else {
                            "lies inside the ambiguity band"
                        }) && !text.contains("declare"),
                        "{label}: {text}"
                    );
                    assert_eq!(
                        offered_below(&text),
                        Some(Some(arm / k())),
                        "{label}: {text}"
                    );
                }
            }
        }
    }

    /// **The merge's declared pair tells one story across its in-band
    /// and definite arms** (D4 ¶1 (iv)), and it is the merge's own: only
    /// a pair facing the same way glues, so the decision passes on a
    /// positive margin alone, where the Boolean's passes on either sign.
    /// Real raises of the declared rung, routed as the merge routes them
    /// (`declared_pair_verdict`): coincident planes facing the same way
    /// (positive margins) and opposite ways (negative), at a shared-edge
    /// chord in the zero band, in the ambiguity band, and definite. The
    /// same-facing definite pair glues; every other arm names the one
    /// lever toward that pass, no declaration, no stage label and no
    /// face key; a positive margin offers the tolerance it gives, and a
    /// negative margin or the definite opposite arm offers none, since
    /// no smaller tolerance turns opposite faces into same-facing ones.
    /// The Boolean's merge stage wraps the sentence behind its own
    /// label, compared as text rather than through the shape guard,
    /// which does not see that label
    /// (`work/tint/the-shape-guard-misses-the-boolean-merge-stage-label.md`).
    #[test]
    fn the_merge_orientation_tells_one_story_across_its_arms() {
        use crate::boolean::oriented_plane_eq;
        use crate::entity::FaceKey;
        use crate::merge_faces::declared_pair_verdict;
        const LEVER: &str = "Recourse: turn one of the two faces so both clearly face the same \
                             way, across a shared edge whose ends lie clearly apart";
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        let (f1, f2) = (FaceKey::default(), FaceKey::default());
        for (chord, definite) in [(0.5 * z, false), ((z + e) / 2.0, false), (1.0, true)] {
            for sign in [1.0, -1.0] {
                let label = format!("chord {chord:e}, facing {sign}");
                let (p1, p2) = planes(sign);
                let verdict =
                    declared_pair_verdict(oriented_plane_eq(&p1, &p2, DECLARED, chord, b), f1, f2);
                let err = match verdict {
                    Ok(glued) => {
                        assert!(
                            definite && sign > 0.0 && glued,
                            "{label}: only a definite same-facing pair glues"
                        );
                        continue;
                    }
                    Err(err) => err,
                };
                assert_eq!(
                    matches!(err, MergeCoplanarError::DeclaredOppositeOrientation { .. }),
                    definite,
                    "{label}: the definite opposite pair is the decision's sign-certain arm: \
                     {err:?}"
                );
                let text = err.to_string();
                assert_eq!(recourse_markers(&text), 1, "{label}: {text}");
                assert!(
                    subjectless_escalations(&text).is_empty()
                        && stage_prefixes(&text, &[]).is_empty(),
                    "{label}: {text}"
                );
                let head = if definite {
                    "the two declared faces face opposite ways across the edge they share. "
                } else {
                    "whether the two declared faces face the same way across the edge they \
                     share is undecided: "
                };
                assert!(
                    text.starts_with(head)
                        && text.contains(LEVER)
                        && !text.contains("declare the")
                        && !text.contains("merge_coplanar_faces")
                        && !text.contains("FaceKey"),
                    "{label}: {text}"
                );
                let offer = (!definite && sign > 0.0).then(|| chord / k());
                assert_eq!(offered_below(&text), offer.map(Some), "{label}: {text}");
                assert_eq!(
                    BooleanError::Merge(err).to_string(),
                    format!("coplanar-merge output stage refused: {text}"),
                    "{label}: the Boolean's merge stage forwards the sentence"
                );
            }
        }
    }

    /// **The maximal-faces gate ends a near-flat pair of neighbours in
    /// its own lever, on a real raise** (F7): a brick's top face split
    /// along its diagonal, one half re-described on a plane bent about
    /// that diagonal by an angle whose sine over the diagonal lands in
    /// the ambiguity band. The gate's parallelism rung escalates, and
    /// the refusal names the gate's decision and lever with the
    /// tolerance its margin gives, not the declare menu, which no
    /// declaration between two faces of one operand could settle.
    #[test]
    fn the_maximal_faces_gate_ends_near_flat_neighbours_in_its_own_lever() {
        let b = band();
        let body = top_split_redescribed(|p0, along, diagonal| {
            let theta = (b.zero() + b.escalate()) / 2.0 / diagonal;
            let up = Vec3::new(0.0, 0.0, 1.0);
            crate::Surface::Plane {
                origin: p0,
                normal: up * theta.cos() + along.cross(up) * theta.sin(),
                u_ref: along,
            }
        });
        let err = super::super::reduce::gate_maximal_faces(&body, Operand::A, b)
            .expect_err("the bent neighbours are too flat to call");
        let BooleanError::Escalated { decision, diag } = err else {
            panic!("the gate escalates: {err:?}");
        };
        assert_eq!(decision, BooleanDecision::Neighbours(PlaneRung::Parallel));
        let text = BooleanError::Escalated { decision, diag }.to_string();
        let problems = short_of_the_guard(&text, &[]);
        assert!(problems.is_empty(), "{problems:?}: {text}");
        assert!(
            text.starts_with(NEIGHBOURS)
                && text.contains(NEIGHBOUR_ENDING)
                && !text.contains("declare"),
            "{text}"
        );
        assert_eq!(
            offered_below(&text),
            Some(Some(point_margin(&diag) / k())),
            "{text}"
        );
    }

    /// A unit prism whose top face is split along its diagonal, one half
    /// re-described on the plane `plane` gives from the diagonal's first
    /// end, its unit direction and its length. The new surface has no
    /// shared source with its neighbour.
    fn top_split_redescribed(
        plane: impl FnOnce(Point3<f64>, Vec3<f64>, f64) -> crate::Surface<f64>,
    ) -> crate::body::Body<f64> {
        let tol = Tol::witness();
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let prism = crate::test_support_fixtures::prism_z::<f64>(&square, 0.0, 1.0, tol);
        let mut body = prism.body;
        let outer = body.get_face(prism.top_face).expect("the top face").outer;
        let crate::LoopBoundary::Cycle { first } =
            body.get_loop(outer).expect("its outer loop").boundary
        else {
            panic!("the top face's outer loop is a cycle");
        };
        let cycle = body.loop_cycle(first).expect("the top loop walks");
        let at = |body: &crate::body::Body<f64>, he| {
            let v = body.get_half_edge(he).expect("a live half-edge").start;
            *body
                .get_point(body.get_vertex(v).expect("a live vertex").point)
                .expect("a live point")
        };
        let (p0, p1) = (at(&body, cycle[0]), at(&body, cycle[2]));
        let half = body
            .mef_chord(
                crate::euler::MefSite::Chords {
                    he1: cycle[0],
                    he2: cycle[2],
                },
                tol,
            )
            .expect("the diagonal splits the top face");
        let diagonal = (p1 - p0).norm();
        let along = (p1 - p0) * (1.0 / diagonal);
        body.set_face_surface(
            half.face,
            crate::euler::FaceSurface::New {
                surface: plane(p0, along, diagonal),
                sense: true,
            },
        )
        .expect("a face takes a plane through its diagonal");
        body
    }

    /// **Two neighbouring faces of one operand on one plane end in the
    /// gate's lever and offer no declaration** (F7), on real raises:
    /// the split top's half re-described on the same plane (a decided
    /// zero offset, no shared source), on a parallel plane a nonzero
    /// offset inside the zero band above, and on one an in-band offset
    /// above. Declarations name pairs across the operands, so the
    /// refusal names the maximal-faces gate's lever. The offset passes
    /// on either definite sign, so every nonzero margin, the zero band's
    /// included, offers the tolerance it gives, and an exactly zero one
    /// none; the text quotes the margin the rung decided, never a claim
    /// of exactness the margin does not make. (The review's
    /// `probe_c2_each_new_arm_rendered` read "their planes' offset is
    /// exactly zero" at the zero band's `5e-10`.)
    #[test]
    fn coplanar_neighbours_end_in_the_gate_lever_and_offer_no_declaration() {
        let b = band();
        let up = Vec3::new(0.0, 0.0, 1.0);
        let mid = (b.zero() + b.escalate()) / 2.0;
        for (offset, decided) in [(0.0, true), (0.5 * b.zero(), true), (mid, false)] {
            let body = top_split_redescribed(|p0, along, _| crate::Surface::Plane {
                origin: p0 + up * offset,
                normal: up,
                u_ref: along,
            });
            let err = super::super::reduce::gate_maximal_faces(&body, Operand::A, b)
                .expect_err("coplanar neighbours with no shared source refuse");
            let BooleanError::CoplanarNeighbours {
                operand,
                offset: refused,
                ..
            } = err
            else {
                panic!("offset {offset:e}: the gate's own refusal: {err:?}");
            };
            assert_eq!(operand, Operand::A);
            assert_eq!(
                matches!(refused, NeighbourOffset::Zero(_)),
                decided,
                "offset {offset:e}: {refused:?}"
            );
            let reported = refused.reported();
            let text = err.to_string();
            let problems = short_of_the_guard(&text, &[]);
            assert!(problems.is_empty(), "{problems:?}: {text}");
            assert!(
                text.starts_with(&format!(
                    "two neighbouring faces of the first operand lie on one plane, or nearly ({}). ",
                    reported.payload()
                )) && text.contains(NEIGHBOUR_ENDING)
                    && !text.contains("declare")
                    && !text.contains("exactly"),
                "offset {offset:e}: {text}"
            );
            let m = point_margin(&reported);
            let offer = (m != 0.0).then(|| m.abs() / k());
            assert_eq!(
                offered_below(&text),
                offer.map(Some),
                "offset {offset:e}: {text}"
            );
            assert_eq!(offer.is_none(), offset == 0.0, "offset {offset:e}: {text}");
        }
    }

    /// **The three definite arms, and the maximal-faces gate's, pass the
    /// refusal-shape guard and offer a declaration only where their
    /// door takes one**: the curved pierce frontier (the declared-cover
    /// rung reads a declaration of the edge's face against the curved
    /// one) ends in the coincidence's levers without the tolerance; the
    /// pierce curvature, on both refused verdicts, names its lever, with
    /// the tolerance a zero-band margin gives; the declared rest zip
    /// names the geometry alone; the undeclared coincidence keeps its
    /// menu.
    #[test]
    fn the_definite_arms_offer_a_declaration_only_where_their_door_takes_one() {
        use crate::boolean::PlaneRelation;
        use crate::entity::FaceKey;
        use geom_brep::recourse::Refused;
        let b = band();
        let zero_margin = 0.5 * b.zero();
        let diag = diag_of(MarginDiag::value((b.zero() + b.escalate()) / 2.0));
        let face = FaceKey::default();
        let rows: [(&str, BooleanError, bool, Option<f64>); 6] = [
            (
                "curved pierce",
                BooleanError::CurvedPierceUnsupported {
                    operand: Operand::A,
                    face,
                    edge: EdgeKey::default(),
                    band: b,
                },
                true,
                None,
            ),
            (
                "pierce curvature at zero",
                BooleanError::CurvedSectorSideUnsupported {
                    verdict: Refused::Zero(Classified {
                        margin: MarginDiag::value(zero_margin),
                        band: b,
                    }),
                },
                false,
                Some(zero_margin / k()),
            ),
            (
                "pierce curvature, negative",
                BooleanError::CurvedSectorSideUnsupported {
                    verdict: Refused::Negative {
                        margin: MarginDiag::value(-1e-3),
                    },
                },
                false,
                None,
            ),
            (
                "rest zip",
                BooleanError::RestZipUnsupported {
                    what: RestZipFrontier::SlitFaceHoles,
                },
                false,
                None,
            ),
            (
                "coplanar neighbours",
                BooleanError::CoplanarNeighbours {
                    operand: Operand::B,
                    faces: [face, face],
                    offset: NeighbourOffset::Undecided(diag),
                },
                false,
                Some(point_margin(&diag) / k()),
            ),
            (
                "undeclared coincidence",
                BooleanError::UndeclaredCoincidence {
                    diag,
                    pair: [(Operand::A, face), (Operand::B, face)],
                    relation: PlaneRelation::SameOriented,
                },
                true,
                None,
            ),
        ];
        for (name, err, declares, offer) in rows {
            let text = err.to_string();
            let problems = short_of_the_guard(&text, &[]);
            assert!(problems.is_empty(), "{name}: {problems:?}: {text}");
            assert_eq!(text.contains("declare the"), declares, "{name}: {text}");
            if !matches!(err, BooleanError::UndeclaredCoincidence { .. }) {
                assert_eq!(offered_below(&text), offer.map(Some), "{name}: {text}");
            }
        }
        let pierce = BooleanError::CurvedPierceUnsupported {
            operand: Operand::A,
            face,
            edge: EdgeKey::default(),
            band: b,
        }
        .to_string();
        assert!(
            pierce.ends_with("Recourse: declare the coincidence, or move the geometry"),
            "the frontier's definite half keeps its in-band sibling's levers, less the \
             tolerance: {pierce}"
        );
        for verdict in [
            Refused::Zero(Classified {
                margin: MarginDiag::value(zero_margin),
                band: b,
            }),
            Refused::Negative {
                margin: MarginDiag::value(-1e-3),
            },
        ] {
            let text = BooleanError::CurvedSectorSideUnsupported { verdict }.to_string();
            assert!(
                text.contains(PIERCE_LEVER),
                "the definite pierce curvature tells its in-band sibling's story: {text}"
            );
        }
    }

    /// **A declared pair's parallelism, and the maximal-faces gate's
    /// rungs, each end where their own door can reach** (D4 ¶1 (i)).
    /// At a declared door the parallelism rung bridges an in-band
    /// margin, so its one escalation is a norm it could not read
    /// (`plane_eq::unreadable_norm`, the rung's own raise): the merge and
    /// the Boolean both end it as a defect, with no declaration. The
    /// maximal-faces gate compares two faces of one operand, which no
    /// declaration names: a real in-band parallelism raise there (two
    /// neighbours bent by an in-band angle over a unit chord) and a
    /// real orientation raise (coincident neighbours over a chord in
    /// the band) end in the gate's lever, the former with the tolerance
    /// its margin gives and the latter with none, since either
    /// orientation leaves the faces coplanar.
    #[test]
    fn declared_parallelism_and_the_neighbour_gate_end_where_their_door_reaches() {
        use crate::boolean::plane_eq::unreadable_norm;
        use crate::boolean::{PlaneDesc, PlaneDoor, PlaneEqError, PlaneRung, oriented_plane_eq};
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        let PlaneEqError::Escalated {
            rung: PlaneRung::Parallel,
            diag,
        } = unreadable_norm(b)
        else {
            panic!("the unreadable norm is the parallelism rung's");
        };
        let boolean = BooleanError::plane_identity(
            PlaneRung::Parallel,
            PlaneDoor::of(Some(ContactClass::Rest)),
            diag,
        )
        .to_string();
        let merge = MergeCoplanarError::of_declared_refusal(unreadable_norm(b)).to_string();
        for text in [&boolean, &merge] {
            let problems = short_of_the_guard(text, &[]);
            assert!(problems.is_empty(), "{problems:?}: {text}");
            assert_eq!(
                text.strip_prefix("whether the two planes are parallel is undecided: ")
                    .and_then(|t| t.split_once(". "))
                    .map(|(_, ending)| ending),
                Some(KERNEL_DEFECT_ENDING),
                "a declared door's unreadable norm is a defect, at both: {text}"
            );
        }
        const GATE: &str = "Recourse: merge the two faces into one first (merge_coplanar_faces), \
                            or tilt one so they meet at a clear angle along an edge clearly \
                            longer than the tolerance";
        let theta = (z + e) / 2.0;
        let (flat, _) = planes(1.0);
        let bent = PlaneDesc {
            origin: Point3::new(0.0, 0.0, 1.0),
            normal: Vec3::new(theta.sin(), 0.0, theta.cos()),
        };
        for (p2, chord, rung) in [
            (bent, 1.0, PlaneRung::Parallel),
            (flat, (z + e) / 2.0, PlaneRung::Orientation),
        ] {
            let err = oriented_plane_eq(&flat, &p2, PlaneIdentity::NONE, chord, b)
                .expect_err("the gate's rung refuses");
            let PlaneEqError::Escalated { rung: got, diag } = err else {
                panic!("{rung:?}: an escalation: {err:?}");
            };
            assert_eq!(got, rung);
            let text = BooleanError::plane_identity(rung, PlaneDoor::Neighbours, diag).to_string();
            let problems = short_of_the_guard(&text, &[]);
            assert!(problems.is_empty(), "{rung:?}: {problems:?}: {text}");
            assert!(
                text.starts_with(
                    "whether two neighbouring faces of one operand lie on one plane is \
                     undecided: "
                ) && text.contains(GATE)
                    && !text.contains("declare"),
                "{rung:?}: {text}"
            );
            let offer = (rung == PlaneRung::Parallel).then(|| point_margin(&diag) / k());
            assert_eq!(offered_below(&text), offer.map(Some), "{rung:?}: {text}");
        }
    }
}
