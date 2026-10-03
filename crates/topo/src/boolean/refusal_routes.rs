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
//!   subject ([`Coincide::subject`]) and its own ending from its own
//!   pass set, led by the declaration where one read ahead of it would
//!   settle it; a decision on a size the user may intend its subject,
//!   its own lever and, on an in-band margin on a side it passes on, the
//!   tolerance that decides it
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
//! A coincidence is a decision each wrap site states, naming which
//! question it asks ([`Coincide`]) and carrying what the door read of
//! the pair's declaration ahead of it ([`DeclarationRead`]), which only
//! the declared-pairs lookup mints as settling: the refusal offers a
//! declaration only where one read ahead would settle the question.

use geom_brep::LeverRung;
use geom_brep::recourse::{
    Classified, Reading, RefusedArm, SizedDecision, SizedPass, StoredDefinite, Unsized,
};
use geom_core::{Indeterminate, UNREADABLE_MARGIN_NOTE};

use crate::contact::{BooleanCoincidence, ContactClass};

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

/// The proximity lever as a literal, for `concat!`: the move a
/// coincidence every verdict of which passes names.
macro_rules! proximity_lever {
    () => {
        "move the parts so they clearly meet or clearly stand apart there"
    };
}

/// The corner edge-length lever as a literal, for `concat!`
/// ([`CORNER_EDGES`]).
macro_rules! corner_edges {
    () => {
        "make the edges at the corner where the two faces meet clearly longer than the tolerance"
    };
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
    /// Whether parts of the two solids coincide, which question the site
    /// asks ([`Coincide`], whose pass set is its own), and what its door
    /// read of the pair's declaration ahead of the question: the refusal
    /// offers a declaration only where that read says one would settle
    /// it ([`DeclarationRead::Settles`]), and otherwise names the
    /// question's own lever and, where it passes on a nonzero sign, the
    /// tolerance the margin gives.
    Coincidence(Coincide, DeclarationRead),
    /// Whether two planes face the same way or opposite ways, at a
    /// cross-operand door, which both definite signs answer and no
    /// declaration changes ([`PlaneRung::Orientation`]).
    PlaneOrientation,
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
    /// Whether two vertices coincide: a vertex of one solid with one of
    /// the other, asked of an edge end on a curved face, or two pierces
    /// of one face, asked of a kept face's pinch. A coincidence both
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
    /// How many times an arc crosses a torus: the circle × torus lane's
    /// certified roots (`circle_torus`), whose rungs are a plane-height
    /// depth, contour residuals and sides, a coaxial tilt and offset, and
    /// the quartic's own rows.
    ArcTorusRoots,
    /// Where a straight edge crosses a sphere: the line × sphere
    /// quadratic's discriminant (`solid_contain::line_sphere_roots`,
    /// `bool_ray_sphere_disc`).
    SphereRoots,
    /// Where an arc crosses a sphere: the circle × sphere lane's
    /// extremes (`circle_sphere`, `bool_circle_sphere_extreme`), read
    /// for a coaxial carrier's constant residual and for either end of
    /// a tilted one's range; on an ellipse, the ellipse door's rows
    /// (`ellipse_roots`).
    ArcSphereRoots,
    /// Where an arc crosses a cylinder wall: the circle × cylinder lane's
    /// certified roots (`circle_cylinder`) — on a circle square to the
    /// wall's axis the square arm's extremes, otherwise the half-angle
    /// quartic's rows; on an ellipse, the ellipse door's rows
    /// (`ellipse_roots`), by the same two arms.
    ArcCylinderRoots,
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
    /// Whether the two faces at a seam edge of the result cross there or
    /// touch tangentially (`dihedral_wedge`, as the result's seam
    /// re-description reads it): every definite verdict passes (a crease
    /// on a positive margin, a smooth join on zero). Asked of the result,
    /// after every declaration was spent, so none is read.
    SeamWedge,
    /// Whether the two faces touching along a seam of the result curve
    /// apart there or share their curvature (`tangent_second_order`, the
    /// must-carry rule's second-order reading at a station of a smooth
    /// seam): both definite verdicts pass (the intrinsic tangency on a
    /// positive margin, the conventional description on zero). Asked of
    /// the result, after every declaration was spent, so none is read.
    SeamJet,
    /// A question the curved-extent scan asks of a sphere face, which
    /// takes no declarations.
    Sphere(SphereQuestion),
    /// A check of the kernel's own construction: a margin an earlier
    /// decision fixed, re-read, or a quantity that cannot honestly be
    /// negative. Its definite refusals are the kernel's, and so is its
    /// escalation.
    SelfCheck(SelfCheck),
}

/// **What a coincidence's door read of the face pair's declaration
/// ahead of the question**, and so whether a declaration would change
/// the verdict (D4 ¶1 (i)). Only [`DeclaredPairs::read`] mints
/// [`DeclarationRead::Settles`]: it looks the pair up, and settles only
/// a question a declaration of a class the door admits there would
/// change ([`Coincide::settled_by`]). Only it offers a declaration.
///
/// [`DeclaredPairs::read`]: super::DeclaredPairs::read
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeclarationRead {
    /// The door looked the pair up and found no declaration, and a
    /// declaration of the class [`Settling`] names, which the door
    /// admits for this pair, would settle the question.
    Settles(Settling),
    /// The door looked the pair up and found it declared under this
    /// class, and the question refused all the same: the declaration is
    /// spent there, and no second one is offered.
    Spent(BooleanCoincidence),
    /// No declaration read ahead of the question settles it: the site
    /// takes no declarations, or the door found none and admits no class
    /// that would change the verdict.
    Moot,
}

/// The class whose declaration would settle a coincidence its door
/// found undeclared, and the question the door read it for: the proof
/// that [`DeclarationRead::Settles`] came from the declared-pairs
/// lookup. Its fields are private to this module, so no other site can
/// state it, and a read carried to another question settles nothing
/// there ([`BooleanDecision::Coincidence`] renders it unsettled).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settling {
    class: BooleanCoincidence,
    question: Coincide,
}

impl Settling {
    /// The coincidence a declaration of which would settle the
    /// question: the one the pair's senses make it, where the door read
    /// them ([`BooleanCoincidence::of_senses`]).
    #[must_use]
    pub const fn class(self) -> BooleanCoincidence {
        self.class
    }

    /// The question the lookup found it settles.
    #[must_use]
    pub const fn question(self) -> Coincide {
        self.question
    }
}

impl<T: geom_core::Real> super::DeclaredPairs<T> {
    /// **What a door read of the declaration ahead of `question`**,
    /// looked up for the face pairs it asks about, in order: the first
    /// declared pair's class is [`DeclarationRead::Spent`] (the question
    /// refused a declared pair); where none is declared, the question is
    /// settled by the first class in `admitted` (the classes the door
    /// admits for this pair's geometry) whose declaration would change
    /// its verdict ([`Coincide::settled_by`]), and otherwise
    /// [`DeclarationRead::Moot`].
    pub(crate) fn read(
        &self,
        pairs: &[(
            super::Operand,
            crate::entity::FaceKey,
            super::Operand,
            crate::entity::FaceKey,
        )],
        question: Coincide,
        admitted: &[BooleanCoincidence],
    ) -> DeclarationRead {
        if let Some(class) = pairs
            .iter()
            .find_map(|&(o1, f1, o2, f2)| self.class_of(o1, f1, o2, f2))
        {
            return DeclarationRead::Spent(class);
        }
        admitted
            .iter()
            .copied()
            .find(|&class| question.settled_by(class))
            .map_or(DeclarationRead::Moot, |class| {
                DeclarationRead::Settles(Settling { class, question })
            })
    }

    /// The plane door of two corners of the two solids read on one
    /// another (`vtxfac`'s coplanar lump, `recl`'s carrier identity),
    /// with what it read of the pair's declaration: a one-carrier
    /// declaration bridges an in-band parallelism
    /// ([`Coincide::OnPlanes`]), and the door admits the one the pair's
    /// `senses` make it ([`BooleanCoincidence::of_senses`]). Senses the
    /// door could not read admit none: a guess would offer a declaration
    /// the declaration door contradicts.
    pub(crate) fn on_pair_door(
        &self,
        pair: (
            super::Operand,
            crate::entity::FaceKey,
            super::Operand,
            crate::entity::FaceKey,
        ),
        senses: Option<super::CarrierRelation>,
    ) -> PlaneDoor {
        let admitted = senses.and_then(BooleanCoincidence::of_senses);
        PlaneDoor::OnPair(self.read(&[pair], Coincide::OnPlanes, admitted.as_slice()))
    }
}

/// **Which coincidence between the two solids a wrap site asks about**:
/// the argument a site states when its escalation is a coincidence, the
/// subject its refusal opens on ([`Coincide::subject`]), and its own
/// pass set, which decides its ending.
///
/// Each question's pass set is what its deciding code does with each
/// verdict, predicate by predicate: a sign passes where the code goes on
/// to an answer on it, and does not where it refuses at once, or where it
/// fixes the next question's verdict to a refusal. Sites whose code
/// passes on different sets ask different questions. A norm that reads
/// definitely negative is poison, not a verdict, and is the kernel's
/// ([`SelfCheck`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum Coincide {
    /// Whether two planes of the two solids are parallel, at the
    /// declared-`Tangent` pair's conformal screen: a definitely distinct
    /// pair goes on to the witness lane, which has no locus for two
    /// planes, and a parallel one to the offset rung, which contradicts a
    /// `Tangent` claim on one carrier, so no verdict passes the
    /// operation.
    Planes,
    /// Whether the planes of two corners of the two solids read on one
    /// another are parallel (`vtxfac`, `recl`, and a declared `Rest`
    /// pair's ladder): only zero passes (a definitely positive one there
    /// contradicts the reading that brought the pair, a kernel
    /// invariant), and a `Rest` declaration bridges the residue.
    OnPlanes,
    /// Whether a vertex of one solid lies on a plane face of the other,
    /// where every side passes. The sweep refuses at the first vertex it
    /// reads in band, before it has read the others, and a smaller
    /// tolerance brings the vertices it read in the zero band into the
    /// band, so no margin it carries binds the operation
    /// ([`LeverPass::Unbound`]).
    VertexOnFace,
    /// Whether an end of a straight edge of one solid lies on a curved
    /// face of the other: every side passes, a vertex definitely inside
    /// the face going on to the exact wall roots.
    VertexOnCurvedFace,
    /// Whether a vertex of one solid lies on a curved face of the other
    /// that it is declared to touch: on it passes, and clear of it
    /// passes only beside an end the face records; a vertex definitely
    /// inside it is a crossing the declared-cover arm refuses.
    VertexOnCoveredFace,
    /// Whether a curved edge of one solid lies in, grazes or misses the
    /// plane of a face of the other: every verdict passes.
    EdgeOnPlane,
    /// Whether a straight edge of one solid clears a curved face of the
    /// other or lies on it: every verdict passes (a clear one at once,
    /// the rest to the exact wall roots).
    EdgeOnCurvedFace,
    /// Whether an arc of one solid lies on a curved face of the other
    /// that it is declared to touch: on it or clear of it passes.
    ArcOnCoveredFace,
    /// Which side of a face of the other solid a corner's edge leaves
    /// on: every side passes.
    SectorSide,
    /// Which way a face of one solid curves away from a face of the
    /// other that it touches tangentially, along a corner's edge: the
    /// second-order side of a declared-`Tangent` pair, every side of
    /// which passes.
    TangentSide,
    /// How two corners of the two solids overlap where they meet: a
    /// direction within a sector, two sectors' faces parallel, their
    /// bounds in line, the order of a strut's germs. Every verdict
    /// passes.
    ///
    /// One question over sites a declaration settles and sites it does
    /// not: only `vtxfac`'s coplanar lump reads the pair ahead of it, with
    /// the classes its door admits for the pierced face; every other site
    /// (`within`, the strut order, `pair_search`, the directions' overlap,
    /// the pierce germ line) passes `Moot` or a read minted with no class
    /// admitted, which the lookup never mints `Settles` from. The read,
    /// not the variant, carries the difference.
    Sectors,
    /// Whether an edge of one solid runs along an edge of the other
    /// (`bool_ee_collinear`, a norm): along it passes, and so does a
    /// clear angle.
    EdgeOnEdge,
    /// Whether plane faces of the two solids along an edge they share
    /// overlap or only touch (`bool_dir_same`, the membership tie): the
    /// cosine of two unit directions levered at the corner's arm, so
    /// either definite sign passes and a decided zero refuses as the
    /// in-band arm does ([`CORNER_SENSE`]). No declaration settles it:
    /// the door refuses `Tangent` on two planes, and `Rest` changes
    /// nothing here.
    FlankSense,
    /// The membership tie where a flanking face is curved: either
    /// definite sense goes on to the curved flank, which the Boolean
    /// cannot yet meet (`CurvedBooleanUnsupported`), so no verdict
    /// passes the operation.
    CurvedFlankSense,
    /// Where two faces of the two solids that are declared to touch
    /// tangentially touch: its rungs pass on different sets, and the
    /// escalation does not say which refused.
    TangentLocus,
    /// Whether a face of each solid ends on one circle, asked only at a
    /// declared-`Tangent` pair's door: every verdict goes on to a
    /// refusal of that declaration (the rim's routing, or the class
    /// unsupported), so no verdict passes the operation
    /// ([`RIM_FRONTIER`]).
    Rim,
    /// Whether a declared contact holds along its witness: the contact
    /// table's rows pass on different sets.
    Contact,
    /// Whether two faces declared on one carrier (`Rest` or a
    /// continuation) lie within the band of one another at every point
    /// of both: the pair's displacement, bounded above over a ball
    /// enclosing the faces, stands past the band, and no point known to
    /// lie on them stands definitely off (`carrier_eq`'s
    /// `CarrierEqError::Unsettled`). Only an in-band bound passes, and
    /// the bound is what stands past it, so no sign of the margin
    /// passes.
    DeclaredReach,
    /// Where the surfaces of a face of each solid meet (a section's
    /// pose): the pose rows pass on different sets.
    Section,
    /// How the sections' ends pair up where the two solids meet: every
    /// verdict of the chord, facing and nearest-end rows passes.
    Join,
}

impl Coincide {
    /// The coincidence, as a clause with no colon or dash of its own:
    /// the subject of every refusal that asks it.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::Planes | Self::OnPlanes => "whether a face of each solid lies on one plane",
            Self::VertexOnFace | Self::VertexOnCurvedFace | Self::VertexOnCoveredFace => {
                "whether a vertex of one solid lies on a face of the other"
            }
            Self::EdgeOnPlane => {
                "whether a curved edge of one solid lies in, grazes or misses the plane of a face \
                 of the other"
            }
            Self::EdgeOnCurvedFace => {
                "whether an edge of one solid clears a curved face of the other or lies on it"
            }
            Self::ArcOnCoveredFace => {
                "whether an arc of one solid lies on a curved face of the other or clears it"
            }
            Self::SectorSide => "which side of a face of the other solid a corner's edge leaves on",
            Self::TangentSide => {
                "which way a face of one solid curves away from a face of the other that it \
                 touches"
            }
            Self::Sectors => "how two corners of the two solids overlap where they meet",
            Self::EdgeOnEdge => "whether an edge of one solid runs along an edge of the other",
            Self::FlankSense | Self::CurvedFlankSense => {
                "whether faces of the two solids along an edge they share overlap or only touch"
            }
            Self::TangentLocus => "where two faces of the two solids touch tangentially",
            Self::Rim => "whether a face of each solid ends on one circle",
            Self::Contact => "whether a declared contact holds along its witness",
            Self::DeclaredReach => {
                "whether two faces declared on one surface stay within the tolerance of one another at \
                 every point of both, which neither a bound over the faces nor a point on them \
                 settles"
            }
            Self::Section => "where the surfaces of a face of each solid meet",
            Self::Join => "how the sections' ends pair up where the two solids meet",
        }
    }

    /// Whether a declaration of `class`, where the door admits it, would
    /// change this question's verdict: the one statement
    /// [`DeclaredPairs::read`](super::DeclaredPairs::read) settles by.
    ///
    /// - [`Coincide::OnPlanes`]: a one-carrier pair's (`Rest` or a
    ///   continuation) ladder bridges an in-band parallelism
    ///   (`plane_eq`'s declared rung).
    /// - [`Coincide::Sectors`], at `vtxfac`'s coplanar sector: a
    ///   one-carrier pair's lump takes the residue through the carrier
    ///   ladder's declared rung, and a `Tangent` pair's descends to the
    ///   second order (`sectors::tangent_lump`).
    ///
    /// Every other question refuses a declared pair as it refuses an
    /// undeclared one, or meets no declaration its door verifies.
    #[must_use]
    pub const fn settled_by(self, class: BooleanCoincidence) -> bool {
        use BooleanCoincidence::{Contact, Continuation};
        match (self, class) {
            (Self::OnPlanes, Contact(ContactClass::Rest) | Continuation)
            | (Self::Sectors, Contact(ContactClass::Rest | ContactClass::Tangent) | Continuation) => {
                true
            }
            (Self::OnPlanes, Contact(ContactClass::Tangent))
            | (
                Self::Planes
                | Self::VertexOnFace
                | Self::VertexOnCurvedFace
                | Self::VertexOnCoveredFace
                | Self::EdgeOnPlane
                | Self::EdgeOnCurvedFace
                | Self::ArcOnCoveredFace
                | Self::SectorSide
                | Self::TangentSide
                | Self::EdgeOnEdge
                | Self::FlankSense
                | Self::CurvedFlankSense
                | Self::TangentLocus
                | Self::Rim
                | Self::Contact
                | Self::DeclaredReach
                | Self::Section
                | Self::Join,
                Contact(ContactClass::Rest | ContactClass::Tangent) | Continuation,
            ) => false,
        }
    }

    /// The question's ending where no declaration settles it: its lever,
    /// and, on a side its pass set holds a nonzero sign of, the tolerance
    /// an in-band margin gives.
    const fn ending(self) -> Ending {
        match self {
            Self::Planes => Ending::Frontier(PLANES_FRONTIER),
            Self::Rim => Ending::Frontier(RIM_FRONTIER),
            Self::OnPlanes => {
                Ending::Lever(geom_core::coincidence_move_arm!(), LeverPass::ZeroOnly)
            }
            Self::VertexOnFace => Ending::Lever(proximity_lever!(), LeverPass::Unbound),
            Self::EdgeOnPlane | Self::SectorSide | Self::Sectors | Self::Join => {
                Ending::Sized(proximity(SizedPass::AnySign))
            }
            // The two edges' directions' cross, a norm levered at the
            // arm: collinear passes, and so does a clear angle.
            Self::EdgeOnEdge => Ending::Sized(proximity(SizedPass::NonNegative)),
            Self::EdgeOnCurvedFace | Self::VertexOnCurvedFace => Ending::Sized(CURVED_CLEARANCE),
            Self::VertexOnCoveredFace => Ending::Lever(COVERED_VERTEX_LEVER, LeverPass::ByArm),
            Self::ArcOnCoveredFace => Ending::Lever(COVERED_ARC_LEVER, LeverPass::DeclaredAway),
            Self::TangentSide => Ending::Sized(TANGENT_SIDE),
            Self::FlankSense => Ending::Sized(FLANK_SENSE),
            Self::CurvedFlankSense => Ending::Lever(CURVED_FLANK_LEVER, LeverPass::Frontier),
            Self::TangentLocus => Ending::Lever(
                "move the parts so the declared faces clearly touch along one line",
                LeverPass::ByRung,
            ),
            Self::Contact | Self::Section => Ending::Lever(proximity_lever!(), LeverPass::ByRung),
            Self::DeclaredReach => Ending::Lever(DECLARED_REACH_LEVER, LeverPass::Never),
        }
    }

    /// The question's ending where a declaration read ahead of it would
    /// settle it: the declaration, then the question's own ending.
    const fn settled(self) -> Ending {
        match self {
            Self::OnPlanes => Ending::Lever(
                geom_core::DEFINITE_COINCIDENCE_RECOURSE,
                LeverPass::ZeroOnly,
            ),
            Self::Sectors => Ending::Sized(SizedDecision {
                lever: concat!(
                    geom_core::coincidence_declare_arm!(),
                    ", or ",
                    proximity_lever!()
                ),
                ..proximity(SizedPass::AnySign)
            }),
            // No declaration settles these (`settled_by`), so no read
            // settles them either: their own ending.
            Self::Planes
            | Self::VertexOnFace
            | Self::VertexOnCurvedFace
            | Self::VertexOnCoveredFace
            | Self::EdgeOnPlane
            | Self::EdgeOnCurvedFace
            | Self::ArcOnCoveredFace
            | Self::SectorSide
            | Self::TangentSide
            | Self::EdgeOnEdge
            | Self::FlankSense
            | Self::CurvedFlankSense
            | Self::TangentLocus
            | Self::Rim
            | Self::Contact
            | Self::DeclaredReach
            | Self::Section
            | Self::Join => self.ending(),
        }
    }
}

/// [`Coincide::DeclaredReach`]'s lever: a declared pair is settled by
/// geometry that reads one way or the other over both faces.
const DECLARED_REACH_LEVER: &str =
    "move the parts so the declared faces clearly coincide, or clearly do not";

/// A coincidence whose geometry lever is to make the parts clearly meet
/// or clearly stand apart, passing on `passes`: an in-band gap on a side
/// it passes on is a size a smaller tolerance decides.
const fn proximity(passes: SizedPass) -> SizedDecision {
    SizedDecision {
        lever: proximity_lever!(),
        size: "gap",
        passes,
        stored: StoredDefinite::Lever,
        at_zero: None,
    }
}

/// Whether two planes of the two solids are parallel at the
/// declared-`Tangent` pair's conformal screen ([`Coincide::Planes`]):
/// parallel goes on to the offset rung, which contradicts a `Tangent`
/// claim (`ContactContradicted`), and a clear tilt to the witness lane,
/// which has no locus for two planes (`UnsupportedDeclarationClass`).
/// No move of the parts passes under that declaration, so no lever is
/// named.
const PLANES_FRONTIER: &str = "Whichever way it reads, the Boolean cannot yet act on a Tangent \
                               contact declared between two plane faces";

/// Whether the faces of a declared-`Tangent` pair end on one circle
/// ([`Coincide::Rim`]): every verdict goes on to a refusal, whatever
/// the rims are (one rim: `RimSeamNotDeclarable`, `RimCuspArmUnbuilt` or
/// `ContactContradicted`; none: `UnsupportedDeclarationClass`), so no
/// move of the parts passes under that declaration.
const RIM_FRONTIER: &str = "Whichever way it reads, the Boolean cannot yet act on a Tangent \
                            contact declared between faces that end on one circle";

/// A curved flank's membership tie ([`Coincide::CurvedFlankSense`]):
/// either sense goes on to the curved flank, which the Boolean cannot
/// yet meet (`CurvedBooleanUnsupported`), so the move that passes is
/// the one that takes the curved face off that edge.
const CURVED_FLANK_LEVER: &str =
    "reshape the parts so the faces along that edge are planes that only touch there";

/// A vertex against a curved face it is declared to touch
/// ([`Coincide::VertexOnCoveredFace`]): the face's implicit residual at
/// the vertex. On the face passes; clear of it passes only where the
/// arc's other end is recorded on the face, and a vertex moved clearly
/// clear of the face it is declared to touch contradicts the
/// declaration at the door, so the lever is the side the declaration
/// holds on.
const COVERED_VERTEX_LEVER: &str = "move the parts so the vertex lies clearly on that face";

/// A straight edge, or an end of one, against a curved face
/// ([`Coincide::EdgeOnCurvedFace`], [`Coincide::VertexOnCurvedFace`]).
/// Every side passes the question; a positive margin is a gap a smaller
/// tolerance decides clear, and a negative one goes on to the exact
/// wall roots, whose discriminant is the depth the edge's line reaches
/// inside the wall (`WallRoots`), the length this margin reads on the
/// other side, so a smaller tolerance is offered on both.
const CURVED_CLEARANCE: SizedDecision = proximity(SizedPass::NonZero);

/// A covered arc against the curved face it is declared to touch
/// ([`Coincide::ArcOnCoveredFace`]): passes on zero (the declared-cover
/// arm reads its ends) and positive (clear), the gap its own face's
/// declared contact says is not there ([`LeverPass::DeclaredAway`]): an
/// arc moved clearly clear of it contradicts the declaration, so the
/// lever is the side the declaration holds on.
const COVERED_ARC_LEVER: &str = "move the parts so the arc lies clearly on that face";

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
/// ([`BooleanDecision::SeamWedge`]): the sine of their angle over the
/// folded lever arm, a crease on a positive margin and a smooth join on
/// zero. The margin is a norm over a length, so no negative one is read.
const SEAM_WEDGE: SizedDecision = SizedDecision {
    lever: "move the geometry so the faces at that seam meet either clearly creased or clearly \
            smooth",
    size: "fold across the seam",
    passes: SizedPass::NonNegative,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// Whether the faces touching along a seam curve apart or share their
/// curvature ([`BooleanDecision::SeamJet`]): the sagitta their relative
/// bend subtends over the folded lever arm, a determinate tangency on a
/// positive margin and an under-determined one on zero. The margin is a
/// magnitude, so no negative one is read.
const SEAM_JET: SizedDecision = SizedDecision {
    lever: "move the geometry so the faces touching along that seam either clearly curve apart \
            there or clearly share their curvature",
    size: "curvature difference across the seam",
    passes: SizedPass::NonNegative,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// **The sense of two directions read parallel at a corner**: a
/// cosine of unit directions levered at the corner's arm, `≈ ±arm`, so
/// what it measures is the arm, and either definite sign passes. One
/// decision wherever it is read: two planes' orientation at the
/// cross-operand doors ([`BooleanDecision::PlaneOrientation`]), two
/// bound directions at a corner ([`BooleanDecision::DirectionSense`]),
/// and two flanks along a shared edge ([`Coincide::FlankSense`]).
pub(crate) const CORNER_SENSE: SizedDecision = SizedDecision {
    lever: CORNER_EDGES,
    size: "length of the corner's shorter edge",
    passes: SizedPass::NonZero,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// The membership tie on plane flanks ([`Coincide::FlankSense`]): either
/// sense passes the tie, but flanks that overlap lie on one plane, which
/// the door then asks to be the same face (an undeclared pair refuses
/// there, `UndeclaredCoincidence`, whatever tolerance decided the tie),
/// so a smaller tolerance is offered on the side where they only touch.
const FLANK_SENSE: SizedDecision = SizedDecision {
    passes: SizedPass::Negative,
    ..CORNER_SENSE
};

/// The one move that lengthens the arm a corner's reading is metered
/// over: shared by every decision whose margin, once its direction has
/// read, measures that arm ([`LeverArm`]'s corner gates,
/// [`CORNER_SENSE`], [`BooleanDecision::BisectorSide`]).
pub(crate) const CORNER_EDGES: &str = corner_edges!();

/// **Which question the curved-extent scan asks of a sphere face**
/// ([`BooleanDecision::Sphere`]). The scan takes no declarations; each
/// question's pass set is its own, and each decided refusal ends as its
/// escalation does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum SphereQuestion {
    /// Whether the sphere crosses a plane face's carrier or clears it
    /// (`bool_sphere_extent_gap`, `r − |s|`): either definite side
    /// passes, and a decided zero, a tangency, refuses with its decided
    /// margin.
    AgainstPlane,
    /// Whether two spheres stand apart (`bool_sphere_sphere_gap`): a
    /// positive gap passes. A gap at or below zero goes on to
    /// [`SphereQuestion::Nested`], whose margin is then about minus the
    /// smaller sphere's diameter, so a gap within the band of zero
    /// refuses there.
    Apart,
    /// Whether the smaller of two overlapping spheres lies strictly
    /// inside the larger (`bool_sphere_sphere_nested`): a positive
    /// clearance passes, and so does a negative one whose two spheres'
    /// faces the section certificate certifies apart. A decided zero, and
    /// a crossing whose circle lies inside both faces, refuse
    /// (`BooleanError::SpheresMeet` is its decided refusal).
    Nested,
    /// Whether the plane faces one sphere pokes through are parallel
    /// (`bool_sphere_escape_parallel`): only a zero passes (a single
    /// re-chart serves parallel planes), so no smaller tolerance
    /// decides a margin passing.
    EscapeParallel,
    /// Whether the sphere's stored polar axis leans away from the escape
    /// normal it is re-charted onto (`bool_sphere_recut_align`, the
    /// axes' cross levered at the radius): a definite lean passes, and a
    /// decided zero refuses with its decided margin.
    RecutAlign,
}

impl SphereQuestion {
    /// What the scan decides, as a clause with no colon or dash of its
    /// own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::AgainstPlane => {
                "whether a sphere of one solid crosses the plane of a face of the other"
            }
            Self::Apart => "whether a sphere of each solid stands clear of the other",
            Self::Nested => "whether the smaller of two spheres lies strictly inside the larger",
            Self::EscapeParallel => "whether the plane faces a sphere pokes through are parallel",
            Self::RecutAlign => {
                "whether a sphere's polar axis leans away from the face it pokes through"
            }
        }
    }

    /// The question's ending on every arm, its decided refusal's too.
    const fn ending(self) -> Ending {
        match self {
            Self::AgainstPlane => Ending::Sized(SPHERE_AGAINST_PLANE),
            Self::Apart | Self::Nested => Ending::Sized(SPHERES),
            Self::EscapeParallel => Ending::Lever(EXTENT_LEVER, LeverPass::ZeroOnly),
            // The lean is the axes' cross, a norm levered at the radius.
            Self::RecutAlign => Ending::Sized(SizedDecision {
                size: "lean of the sphere's polar axis",
                passes: SizedPass::Positive,
                ..SPHERE_AGAINST_PLANE
            }),
        }
    }
}

/// A sphere against a plane face's carrier
/// ([`SphereQuestion::AgainstPlane`]): either definite side passes.
const SPHERE_AGAINST_PLANE: SizedDecision = SizedDecision {
    lever: "move the parts so the sphere clearly crosses that face's plane or clearly clears it",
    size: "gap",
    passes: SizedPass::NonZero,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// Two spheres of the two solids ([`SphereQuestion::Apart`],
/// [`SphereQuestion::Nested`]): a pair clearly apart or clearly nested
/// passes on the carriers, and a crossing pair passes when its faces
/// are certified apart. The scan runs only where no edge crosses a
/// face, so two sphere faces that meet there share nothing the join's
/// sphere-pair arm can run a chord along; the lever names the moves
/// that settle the carriers.
pub(crate) const SPHERES: SizedDecision = SizedDecision {
    lever: "move the spheres so they clearly stand apart, or so one lies clearly inside the other",
    size: "clearance",
    passes: SizedPass::Positive,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// The curved-extent scan's lever where its enclosures cannot certify
/// the operands (`BooleanError::FallbackExtentUnsupported`, and the
/// escape-parallel question that leads to one of its arms).
pub(crate) const EXTENT_LEVER: &str =
    "move them so their boundaries cross, or so their curved faces stand further apart";

/// **Which of the kernel's own checks escalated**
/// ([`BooleanDecision::SelfCheck`]): each re-reads a margin an earlier
/// decision fixed, or reads a quantity that cannot honestly be negative,
/// so its definite refusal is a broken invariant and its escalation the
/// same story.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum SelfCheck {
    /// Whether a crossing record's two faces cross along a line
    /// (`bool_germ_line` at `insert::germ_dir`): the pair was sent down
    /// the crossing path because this same margin read definitely
    /// positive upstream (`sectors::pair_search`'s
    /// `bool_faces_parallel`), and a zero one is a
    /// `ClassificationInvariant`. (`vtxfac::pierce_germ_dir` reads its
    /// margin at another arm than the transition reading that sent it,
    /// so its in-band arm is the corners' overlap undecided.)
    GermLine,
    /// Whether two faces' normals at a corner can be read
    /// (`bool_faces_parallel`, `plane_eq`'s parallelism rung): a norm
    /// read definitely negative is poisoned input, and no declaration
    /// or move of the parts reads it.
    Normals,
    /// Which way a germ turns about the conic section it lies on
    /// (`bool_join_arc_facing`): a zero sense is a radial germ,
    /// malformed germ data (`JoinDesync`).
    ArcFacing,
    /// Which way a ring run of the section winds
    /// (`bool_ring_run_winding`): a zero area is a degenerate run
    /// (`JoinDesync`).
    RingWinding,
    /// The carrier ladder's contradiction arm, which its detector
    /// posture (nothing declared) cannot reach.
    CarrierLadder,
}

impl SelfCheck {
    /// What the check decides, as a clause with no colon or dash of its
    /// own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::GermLine => "whether two faces meeting at a corner cross along a line",
            Self::Normals => "whether the normals of two faces can be read",
            Self::ArcFacing => "which way a germ turns about the section it lies on",
            Self::RingWinding => "which way a ring run of the section winds",
            Self::CarrierLadder => "whether a face of each solid lies on one surface",
        }
    }
}

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
    /// sector's arm), quoted at the departure from the face it meters
    /// over that reach, which is what a smaller tolerance has to decide.
    SectorSide,
    /// `tangent_sector_order2_arm` at a declared-tangent sector pair:
    /// the sector's arm.
    SectorCurving,
    /// `dihedral_arm` at a seam edge the result re-describes: the fold
    /// of the edge's extent and its faces' radii of curvature, quoted at
    /// the wedge it meters over that fold (`geom_brep`'s `at_wedge`).
    Seam,
}

impl LeverArm {
    /// Every gate, for the readers that pair each with its reading (the
    /// executed-offer harness's one-decision pairs, `test_support`).
    pub const ALL: [Self; 3] = [Self::SectorSide, Self::SectorCurving, Self::Seam];

    /// The decision the gate meters, with what its door read of the
    /// declaration: the reading whose question a smaller tolerance that
    /// decides the arm leaves, so the gate and it are one decision to a
    /// user re-running at the tolerance offered.
    #[must_use]
    pub const fn reading(self, read: DeclarationRead) -> BooleanDecision {
        match self {
            Self::SectorSide => BooleanDecision::Coincidence(Coincide::SectorSide, read),
            Self::SectorCurving => BooleanDecision::Coincidence(Coincide::TangentSide, read),
            Self::Seam => BooleanDecision::SeamWedge,
        }
    }

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
            Self::SectorSide => sized(CORNER_EDGES, "edge's length or rise", SizedPass::Positive),
            Self::SectorCurving => sized(CORNER_EDGES, "edge length", SizedPass::Positive),
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
    lever: "make the edge leave the face more steeply, or make the face curve less sharply there",
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
    /// The other part's edge a chord stands for has no certified line
    /// or circle carrier to mint the chord on.
    TwinCarrierUnsupported,
    /// A contact patch's boundary vertex has no partner across the seam.
    PatchVertexUnmatched,
    /// The two contact patches' face cycles do not match across the
    /// mate.
    PatchCyclesIncongruent,
    /// A hole's boundary vertex has no partner across the seam.
    HoleVertexUnmatched,
    /// The two contact patches' holes do not match across the mate, one
    /// for one.
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
            Self::TwinCarrierUnsupported => {
                "seam chord's counterpart edge has no certified line or circle carrier"
            }
            Self::PatchVertexUnmatched => "patch boundary vertex without a seam correspondent",
            Self::PatchCyclesIncongruent => "patch face cycles not congruent across the mate",
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
            Self::HoleVertexUnmatched | Self::HoleCyclesIncongruent => {
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
            | Self::TwinCarrierUnsupported
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
#[derive(Clone, Copy)]
enum Ending {
    /// A decision on a size the user may intend.
    Sized(SizedDecision),
    /// The lever alone: no margin of the decision gives a tolerance to
    /// tighten below, for the reason its pass set states.
    Lever(&'static str, LeverPass),
    /// A decision every verdict of which goes on to a refusal no move of
    /// the parts passes under the declaration its door holds: what the
    /// Boolean cannot yet do, and no lever.
    Frontier(&'static str),
    /// A decision with no size the user chose.
    Unsized(Unsized),
}

impl Ending {
    /// The ending's sentence on the escalation `diag`, at the Boolean
    /// that built the geometry.
    fn recourse(self, diag: &Indeterminate) -> String {
        let arm = RefusedArm::Undecided(diag);
        match self {
            Self::Sized(decision) => decision.recourse(arm, Reading::Build),
            Self::Lever(lever, passes) => passes.recourse(lever, diag),
            Self::Frontier(what) => format!("{what}. {}", geom_core::NOT_YET_ENDING),
            Self::Unsized(decision) => decision.recourse(arm, Reading::Build),
        }
    }
}

/// What a lever-alone decision passes on, as its deciding code has it,
/// and so why no refusal of it offers the tolerance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LeverPass {
    /// It passes on either definite sign, and the margin is no length
    /// the user chose, so a tolerance below it names no size: the offer
    /// waits on a length door for the margin.
    NonZeroNotALength,
    /// The arms that read its verdict pass on different sets, so no one
    /// sign set is the decision's.
    ByArm,
    /// Its rungs pass on different sets, and the escalation does not
    /// say which rung refused.
    ByRung,
    /// It passes only on a definite zero, which no smaller tolerance
    /// decides a nonzero margin onto.
    ZeroOnly,
    /// No sign of its margin passes where it is asked.
    Never,
    /// Every verdict it passes on goes on to a refusal no smaller
    /// tolerance passes (a frontier, or a declaration its door refuses),
    /// so no tolerance decides the operation passing.
    Frontier,
    /// Its escalation carries the band's own enclosure rather than a
    /// measured margin, so there is no value to tighten below.
    Unmeasured,
    /// It passes on a gap from a face the pair is declared to touch: a
    /// smaller tolerance that decides the gap decides the declaration
    /// contradicted at the door, so it offers none.
    DeclaredAway,
    /// Its refusal is raised at the first reading in band, before the
    /// other readings of it are taken, and a smaller tolerance brings the
    /// readings this one decided in the zero band into the band, so no
    /// margin it carries binds the operation.
    Unbound,
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
    /// A sector pair of the two solids read on one another (`vtxfac`,
    /// `recl`), or a declared `Rest` pair's verification, with what the
    /// door read of the pair's declaration before it ran the ladder
    /// ([`Coincide::OnPlanes`]).
    OnPair(DeclarationRead),
    /// A declared-`Tangent` pair's conformal screen, where a definitely
    /// distinct pair passes ([`Coincide::Planes`]), with the declaration
    /// the door holds.
    Screen(DeclarationRead),
    /// Two neighbouring faces of one operand (F7).
    Neighbours,
}

/// The maximal-faces gate's lever: the one move that takes two
/// neighbouring faces off the question, whichever rung asked it.
pub(crate) const NEIGHBOUR_LEVER: &str = "merge the two faces into one first \
                               (merge_coplanar_faces), or \
                               tilt one to meet at a clear angle along an edge clearly longer \
                               than the tolerance";

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
    /// across the operands, in-band parallelism is the question the door
    /// asks of the pair, with what it read (a declared `Rest` pair's
    /// ladder bridges an in-band margin), and orientation a decision of
    /// its own; between neighbours of one operand, both rungs ask the
    /// maximal-faces question. A norm read definitely negative is
    /// poisoned input at every door, which no declaration and no move
    /// of the parts reads.
    pub(crate) const fn of_plane_rung(rung: PlaneRung, door: PlaneDoor) -> Self {
        match (door, rung) {
            (_, PlaneRung::Norm) => Self::SelfCheck(SelfCheck::Normals),
            (PlaneDoor::OnPair(read), PlaneRung::Parallel) => {
                Self::Coincidence(Coincide::OnPlanes, read)
            }
            (PlaneDoor::Screen(read), PlaneRung::Parallel) => {
                Self::Coincidence(Coincide::Planes, read)
            }
            (PlaneDoor::OnPair(_) | PlaneDoor::Screen(_), PlaneRung::Orientation) => {
                Self::PlaneOrientation
            }
            (PlaneDoor::Neighbours, rung) => Self::Neighbours(rung),
        }
    }

    /// The decision a conic root lane's escalation came from: a
    /// crossing decision as the fault routes it, and otherwise a
    /// coincidence between the plane and the conic, with what the door
    /// read of the declaration ahead of it.
    pub(crate) fn of_conic_root(fault: ConicRootFault, read: DeclarationRead) -> Self {
        fault.decision().map_or(
            Self::Coincidence(Coincide::EdgeOnPlane, read),
            Self::Crossing,
        )
    }

    /// The decision a lever-armed reading escalated on at `gate`: its arm
    /// rung is the gate's own length, and its reading is the decision the
    /// gate meters ([`LeverArm::reading`]), with what its door read of the
    /// declaration.
    pub(crate) const fn of_lever(gate: LeverArm, read: DeclarationRead, rung: LeverRung) -> Self {
        match rung {
            LeverRung::Arm => Self::LeverArm(gate),
            LeverRung::Reading => gate.reading(read),
        }
    }

    /// What the decision decides, as a clause with no colon or dash of
    /// its own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::Coincidence(which, _) => which.subject(),
            Self::PlaneOrientation => PlaneRung::Orientation.subject(),
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
            Self::VertexOnVertex => "whether two vertices coincide",
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
            Self::ArcTorusRoots => "how many times an arc crosses a torus",
            Self::SphereRoots => "whether an edge crosses a sphere, grazes it or misses it",
            Self::ArcSphereRoots => "whether an arc crosses a sphere, grazes it or misses it",
            Self::ArcCylinderRoots => {
                "whether an arc crosses a cylinder wall, grazes it or misses it"
            }
            Self::PierceCurvature => {
                "whether an edge leaves a curved face steeply enough against its bend to read \
                 which side it goes"
            }
            Self::DirectionSense => {
                "whether two parallel directions at a corner point the same way or opposite ways"
            }
            Self::BisectorSide => "which side of a face a corner's bisector leaves on",
            Self::SeamWedge => {
                "whether the two faces at a seam edge cross there or touch tangentially"
            }
            Self::SeamJet => {
                "whether the two faces touching along a seam edge curve apart there or share \
                 their curvature"
            }
            Self::Sphere(question) => question.subject(),
            Self::SelfCheck(check) => check.subject(),
        }
    }

    /// How the decision's escalation ends: from what it passes on. A
    /// coincidence a declaration read ahead would settle ends in the
    /// declaration first ([`Coincide::settled`]), on a margin it could
    /// read; a poisoned one no declaration reads, and a read the lookup
    /// minted for another question settles nothing here, so each ends as
    /// the question does unsettled.
    fn ending(self, diag: &Indeterminate) -> Ending {
        match self {
            Self::Coincidence(which, DeclarationRead::Settles(settling))
                if settling.question == which && !diag.margin.is_invalid() =>
            {
                which.settled()
            }
            // Overlapping plane flanks lie on one plane, which the door
            // then asks to be one face: a declared one-carrier pair's
            // (`Rest` or a continuation) is the one face it verified, so
            // both senses pass there.
            Self::Coincidence(
                Coincide::FlankSense,
                DeclarationRead::Spent(BooleanCoincidence::REST | BooleanCoincidence::Continuation),
            ) => Ending::Sized(CORNER_SENSE),
            Self::Coincidence(which, _) => which.ending(),
            // Its margin is the normals' cosine at the door's arm, `≈ ±arm`,
            // and the offset rung asks next: a declared `Rest` pair's
            // bridges, an undeclared pair's refuses as an undeclared
            // coincidence or a carrier contradiction at every tolerance.
            Self::PlaneOrientation => Ending::Lever(CORNER_EDGES, LeverPass::ByArm),
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
            // A poisoned norm, which `of_plane_rung` routes to the
            // kernel's own check at every door.
            Self::Neighbours(PlaneRung::Norm) => Ending::Unsized(Unsized::Defect),
            // The arm passes on a positive length.
            Self::Corner(SectorRung::Arm) => {
                sized(CORNER_LEVER, "edge length", SizedPass::Positive)
            }
            // The margin is cos θ levered by the shorter edge: that
            // edge's projection onto the other, a length. A straight
            // corner passes on a negative one (`sector_shape` gates it with
            // `decide_negative`); a sector bounded twice by
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
            Self::Radius(SectionRadius::Cylinder) => Ending::Sized(SectionRadius::Cylinder.sized()),
            // The cylinder's guard asks first, and a cylinder meeting a
            // coaxial sphere is the narrower, so an in-band sphere radius
            // here belongs to a pair that does not meet, which the frame
            // refuses at every smaller tolerance.
            Self::Radius(SectionRadius::Sphere) => {
                Ending::Lever(SectionRadius::Sphere.sized().lever, LeverPass::Frontier)
            }
            // The margin is the edge's drift off its distance from the
            // axis over its own length. A positive one has roots to find;
            // a zero one is a constant residual, which every edge-sweep
            // arm refuses at the frontier.
            Self::WallRoots(WallRung::AxisParallel) => sized(
                "turn the edge clearly away from the direction of the cylinder's axis",
                "edge's drift off the cylinder's axis",
                SizedPass::Positive,
            ),
            // Two roots pass at every arm; a miss passes where both ends
            // are off the wall and refuses where one is on it (a miss
            // cannot hold a zero endpoint); a zero is a tangency, refused
            // at the frontier. The margin is the depth the edge's line
            // reaches inside the wall, a length.
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
            // The lane's rungs pass on different sets (the plane-height
            // depth passes positive, a contour side either sign, the
            // quartic's rows a definite sign each) and in different units
            // (some are lengths, the quartic's rows are not), and the
            // escalation names its predicate only, so no one margin gives
            // a tolerance to tighten below.
            Self::ArcTorusRoots => Ending::Lever(
                "move the parts so the arc clearly crosses the torus or clearly misses it",
                LeverPass::ByRung,
            ),
            // As the wall's discriminant: two roots pass, a miss passes
            // where both ends are off the sphere, and a zero is a
            // tangency, refused at the frontier.
            Self::SphereRoots => Ending::Lever(
                "move the parts so the edge clearly crosses the sphere or clearly misses it",
                LeverPass::ByArm,
            ),
            // One predicate reads three extremes that pass on different
            // sets (a coaxial residual's zero is the constant case, the
            // near end passes positive, the far end negative), and the
            // escalation does not say which refused.
            Self::ArcSphereRoots => Ending::Lever(
                "move the parts so the arc clearly crosses the sphere or clearly misses it",
                LeverPass::ByRung,
            ),
            // The two lanes' rungs together: the extremes' sets, and the
            // quartic's rows, which are not all lengths.
            Self::ArcCylinderRoots => Ending::Lever(
                "move the parts so the arc clearly crosses the cylinder or clearly misses it",
                LeverPass::ByRung,
            ),
            Self::PierceCurvature => Ending::Sized(PIERCE_CURVATURE),
            Self::DirectionSense => Ending::Sized(CORNER_SENSE),
            // Its escalation carries the zero band's own enclosure, not a
            // measured margin; a longer arm lengthens every reading of the
            // corner.
            Self::BisectorSide => Ending::Lever(CORNER_EDGES, LeverPass::Unmeasured),
            Self::SeamWedge => Ending::Sized(SEAM_WEDGE),
            Self::SeamJet => Ending::Sized(SEAM_JET),
            Self::Sphere(question) => question.ending(),
            // A broken invariant, as the check's definite refusal says.
            Self::SelfCheck(_) => Ending::Unsized(Unsized::Defect),
        }
    }

    /// The whole sentence an escalation of this decision renders, at the
    /// Boolean that built the geometry: the subject, the payload and the
    /// one ending the verdict gives.
    #[must_use]
    pub(crate) fn render(self, diag: &Indeterminate) -> String {
        let (subject, payload) = (self.subject(), diag.payload());
        let ending = self.ending(diag).recourse(diag);
        format!("{subject} is undecided: {payload}. {ending}")
    }
}

#[cfg(test)]
#[path = "offer_rows.rs"]
mod offer_rows;

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
                                    (merge_coplanar_faces), or tilt one to meet at a clear angle \
                                    along an edge clearly longer than the tolerance";

    const TUBE_LEVER: &str =
        "Recourse: reshape the torus so its tube is clearly thicker than the tolerance";
    const RING_LEVER: &str = "Recourse: make the tube radius clearly smaller than the ring radius";

    const CROSSING_LEVER_ENDING: &str =
        "Recourse: move the geometry so the crossing lands clearly away from the edge's ends";

    const WALL_LEVER: &str =
        "Recourse: move the parts so the edge clearly crosses the wall or clearly misses it";
    const PIERCE_LEVER: &str = "Recourse: make the edge leave the face more steeply, or make the face curve less sharply \
         there";

    /// The split door's own clause, a stage for a subject, filed with
    /// its owner: `work/hone/reach-refusals-short-of-the-shape-guard.md`.
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
    pub(super) fn every_decision() -> Vec<BooleanDecision> {
        BooleanDecisionKind::iter()
            .flat_map(|kind| match kind {
                BooleanDecisionKind::Coincidence => Coincide::iter()
                    .flat_map(|which| {
                        every_read(which).map(move |read| BooleanDecision::Coincidence(which, read))
                    })
                    .collect(),
                BooleanDecisionKind::PlaneOrientation => vec![BooleanDecision::PlaneOrientation],
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
                BooleanDecisionKind::ArcTorusRoots => vec![BooleanDecision::ArcTorusRoots],
                BooleanDecisionKind::SphereRoots => vec![BooleanDecision::SphereRoots],
                BooleanDecisionKind::ArcSphereRoots => vec![BooleanDecision::ArcSphereRoots],
                BooleanDecisionKind::ArcCylinderRoots => vec![BooleanDecision::ArcCylinderRoots],
                BooleanDecisionKind::PierceCurvature => vec![BooleanDecision::PierceCurvature],
                BooleanDecisionKind::DirectionSense => vec![BooleanDecision::DirectionSense],
                BooleanDecisionKind::BisectorSide => vec![BooleanDecision::BisectorSide],
                BooleanDecisionKind::SeamWedge => vec![BooleanDecision::SeamWedge],
                BooleanDecisionKind::SeamJet => vec![BooleanDecision::SeamJet],
                BooleanDecisionKind::Sphere => SphereQuestion::iter()
                    .map(BooleanDecision::Sphere)
                    .collect(),
                BooleanDecisionKind::SelfCheck => {
                    SelfCheck::iter().map(BooleanDecision::SelfCheck).collect()
                }
            })
            .collect()
    }

    /// Every read a door can hand `which`, each minted by the one
    /// constructor ([`crate::boolean::DeclaredPairs::read`]): the pair
    /// declared under each class ([`BooleanCoincidence::ALL`]), and undeclared
    /// with each class admitted, and with none.
    fn every_read(which: Coincide) -> impl Iterator<Item = DeclarationRead> + Clone {
        use crate::boolean::{BooleanDeclarations, DeclaredPairs, FacePairDeclaration};
        let face = crate::entity::FaceKey::default();
        let pair = [(Operand::A, face, Operand::B, face)];
        let declared = |class: Option<BooleanCoincidence>| {
            let decls = BooleanDeclarations {
                coincident_faces: class
                    .map(|class| vec![FacePairDeclaration::new(face, face, class)])
                    .unwrap_or_default(),
                ..BooleanDeclarations::none()
            };
            DeclaredPairs::<f64>::without_struts(&decls, Default::default())
        };
        let mut reads = vec![declared(None).read(&pair, which, &[])];
        for &class in BooleanCoincidence::ALL {
            reads.push(declared(Some(class)).read(&pair, which, BooleanCoincidence::ALL));
            reads.push(declared(None).read(&pair, which, &[class]));
        }
        reads.dedup();
        reads.into_iter()
    }

    /// How a decision's escalation must end, written independently of
    /// the table.
    enum Ending {
        /// The lever, and the pass set the table must state, from which
        /// the margins a smaller tolerance decides passing follow.
        Sized(&'static str, SizedPass),
        /// The lever alone, on every margin, with the pass set the table
        /// states as the reason.
        Lever(&'static str, LeverPass),
        /// What the Boolean cannot yet do, and no lever, on every margin.
        Frontier(&'static str),
        /// The defect ending.
        Defect,
    }

    /// Each coincidence's subject, as a literal.
    fn coincide_subject(which: Coincide) -> &'static str {
        match which {
            Coincide::Planes | Coincide::OnPlanes => {
                "whether a face of each solid lies on one plane"
            }
            Coincide::VertexOnFace
            | Coincide::VertexOnCurvedFace
            | Coincide::VertexOnCoveredFace => {
                "whether a vertex of one solid lies on a face of the other"
            }
            Coincide::EdgeOnPlane => {
                "whether a curved edge of one solid lies in, grazes or misses the plane of a face \
                 of the other"
            }
            Coincide::EdgeOnCurvedFace => {
                "whether an edge of one solid clears a curved face of the other or lies on it"
            }
            Coincide::ArcOnCoveredFace => {
                "whether an arc of one solid lies on a curved face of the other or clears it"
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
            Coincide::FlankSense | Coincide::CurvedFlankSense => {
                "whether faces of the two solids along an edge they share overlap or only touch"
            }
            Coincide::TangentLocus => "where two faces of the two solids touch tangentially",
            Coincide::Rim => "whether a face of each solid ends on one circle",
            Coincide::Contact => "whether a declared contact holds along its witness",
            Coincide::DeclaredReach => {
                "whether two faces declared on one surface stay within the tolerance of one another at \
                 every point of both, which neither a bound over the faces nor a point on them \
                 settles"
            }
            Coincide::Section => "where the surfaces of a face of each solid meet",
            Coincide::Join => "how the sections' ends pair up where the two solids meet",
        }
    }

    /// Each coincidence's own ending where no declaration settles it,
    /// as literals: every question stated on its own, its pass set the
    /// one its deciding code has.
    fn coincide_ending(which: Coincide) -> Ending {
        const MEET: &str =
            "Recourse: move the parts so they clearly meet or clearly stand apart there";
        match which {
            // Every verdict at the declared-`Tangent` screen of two planes
            // goes on to a refusal no tolerance passes.
            Coincide::Planes => Ending::Frontier(
                "Whichever way it reads, the Boolean cannot yet act on a Tangent contact \
                 declared between two plane faces. There is no way through yet",
            ),
            Coincide::OnPlanes => Ending::Lever("Recourse: move the geometry", LeverPass::ZeroOnly),
            // Refused at the first vertex read in band, the rest unread.
            Coincide::VertexOnFace => Ending::Lever(MEET, LeverPass::Unbound),
            // Clear of the face passes only beside an end the face
            // records, and contradicts the declaration elsewhere.
            Coincide::VertexOnCoveredFace => Ending::Lever(
                "Recourse: move the parts so the vertex lies clearly on that face",
                LeverPass::ByArm,
            ),
            Coincide::EdgeOnPlane => Ending::Sized(MEET, SizedPass::AnySign),
            // A negative margin goes on to the wall roots, whose depth a
            // smaller tolerance decides too: both sides are offered.
            Coincide::EdgeOnCurvedFace | Coincide::VertexOnCurvedFace => {
                Ending::Sized(MEET, SizedPass::NonZero)
            }
            // A clear arc is a gap its face's declared contact says is
            // not there.
            Coincide::ArcOnCoveredFace => Ending::Lever(
                "Recourse: move the parts so the arc lies clearly on that face",
                LeverPass::DeclaredAway,
            ),
            Coincide::SectorSide => Ending::Sized(MEET, SizedPass::AnySign),
            Coincide::TangentSide => Ending::Sized(
                "Recourse: make one face clearly curve away from the other where they touch, or \
                 make both curve alike there",
                SizedPass::AnySign,
            ),
            Coincide::Sectors => Ending::Sized(MEET, SizedPass::AnySign),
            Coincide::EdgeOnEdge => Ending::Sized(MEET, SizedPass::NonNegative),
            // Overlapping plane flanks go on to the undeclared coplanar
            // pair's refusal: the offer is the touching side's alone.
            Coincide::FlankSense => Ending::Sized(LONGER, SizedPass::Negative),
            // Either sense goes on to a curved flank the Boolean cannot
            // yet meet.
            Coincide::CurvedFlankSense => Ending::Lever(
                "Recourse: reshape the parts so the faces along that edge are planes that only \
                 touch there",
                LeverPass::Frontier,
            ),
            Coincide::TangentLocus => Ending::Lever(
                "Recourse: move the parts so the declared faces clearly touch along one line",
                LeverPass::ByRung,
            ),
            // Asked only at a declared-`Tangent` door, every verdict of
            // which goes on to a refusal of that declaration.
            Coincide::Rim => Ending::Frontier(
                "Whichever way it reads, the Boolean cannot yet act on a Tangent contact \
                 declared between faces that end on one circle. There is no way through yet",
            ),
            Coincide::Contact => Ending::Lever(MEET, LeverPass::ByRung),
            Coincide::DeclaredReach => Ending::Lever(
                "Recourse: move the parts so the declared faces clearly coincide, or clearly do not",
                LeverPass::Never,
            ),
            Coincide::Section => Ending::Lever(MEET, LeverPass::ByRung),
            Coincide::Join => Ending::Sized(MEET, SizedPass::AnySign),
        }
    }

    /// Each settleable coincidence's ending where a declaration read
    /// ahead of it would settle it: the declaration, then its own lever,
    /// and the tolerance only where its own pass set holds a nonzero
    /// sign. `None` for a question no declaration settles.
    fn coincide_settled(which: Coincide) -> Option<Ending> {
        match which {
            Coincide::OnPlanes => Some(Ending::Lever(
                "Recourse: declare the coincidence, or move the geometry",
                LeverPass::ZeroOnly,
            )),
            Coincide::Sectors => Some(Ending::Sized(
                "Recourse: declare the coincidence, or move the parts so they clearly meet or \
                 clearly stand apart there",
                SizedPass::AnySign,
            )),
            Coincide::Planes
            | Coincide::VertexOnFace
            | Coincide::VertexOnCurvedFace
            | Coincide::VertexOnCoveredFace
            | Coincide::EdgeOnPlane
            | Coincide::EdgeOnCurvedFace
            | Coincide::ArcOnCoveredFace
            | Coincide::SectorSide
            | Coincide::TangentSide
            | Coincide::EdgeOnEdge
            | Coincide::FlankSense
            | Coincide::CurvedFlankSense
            | Coincide::TangentLocus
            | Coincide::Rim
            | Coincide::Contact
            | Coincide::DeclaredReach
            | Coincide::Section
            | Coincide::Join => None,
        }
    }

    const LONGER: &str = "Recourse: make the edges at the corner where the two faces meet \
                          clearly longer than the tolerance";

    /// Each decision's subject and ending, as literals: an independent
    /// statement of the words `subject` and `ending` must produce, on a
    /// margin it can read (`readable`) or a poisoned one.
    fn want(decision: BooleanDecision, readable: bool) -> (&'static str, Ending) {
        const CORNER: &str = "Recourse: reshape that corner so its edges are clearly longer than \
                              the tolerance and clearly not in line";
        const STRAIGHT: &str = "whether a corner is straight or folds back on itself";
        const SPHERES: &str = "Recourse: move the spheres so they clearly stand apart, or so one \
                               lies clearly inside the other";
        const AGAINST: &str = "Recourse: move the parts so the sphere clearly crosses that face's \
                               plane or clearly clears it";
        match decision {
            BooleanDecision::Coincidence(which, DeclarationRead::Settles(_)) if readable => (
                coincide_subject(which),
                coincide_settled(which).expect("only a settleable question is minted settled"),
            ),
            // A declared one-carrier pair's overlapping flanks are the
            // face the door verified: both senses pass.
            BooleanDecision::Coincidence(
                Coincide::FlankSense,
                DeclarationRead::Spent(BooleanCoincidence::REST | BooleanCoincidence::Continuation),
            ) => (
                coincide_subject(Coincide::FlankSense),
                Ending::Sized(LONGER, SizedPass::NonZero),
            ),
            BooleanDecision::Coincidence(
                which,
                DeclarationRead::Settles(_) | DeclarationRead::Spent(_) | DeclarationRead::Moot,
            ) => (coincide_subject(which), coincide_ending(which)),
            BooleanDecision::DirectionSense => (
                "whether two parallel directions at a corner point the same way or opposite ways",
                Ending::Sized(LONGER, SizedPass::NonZero),
            ),
            BooleanDecision::BisectorSide => (
                "which side of a face a corner's bisector leaves on",
                Ending::Lever(LONGER, LeverPass::Unmeasured),
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
                Ending::Lever(
                    "Recourse: make the sphere's radius clearly larger than the tolerance",
                    LeverPass::Frontier,
                ),
            ),
            BooleanDecision::WallRoots(WallRung::AxisParallel) => (
                "whether an edge runs parallel to a cylinder's axis",
                Ending::Sized(
                    "Recourse: turn the edge clearly away from the direction of the cylinder's \
                     axis",
                    SizedPass::Positive,
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
            BooleanDecision::ArcTorusRoots => (
                "how many times an arc crosses a torus",
                Ending::Lever(
                    "Recourse: move the parts so the arc clearly crosses the torus or clearly \
                     misses it",
                    LeverPass::ByRung,
                ),
            ),
            BooleanDecision::SphereRoots => (
                "whether an edge crosses a sphere, grazes it or misses it",
                Ending::Lever(
                    "Recourse: move the parts so the edge clearly crosses the sphere or clearly \
                     misses it",
                    LeverPass::ByArm,
                ),
            ),
            BooleanDecision::ArcSphereRoots => (
                "whether an arc crosses a sphere, grazes it or misses it",
                Ending::Lever(
                    "Recourse: move the parts so the arc clearly crosses the sphere or clearly \
                     misses it",
                    LeverPass::ByRung,
                ),
            ),
            BooleanDecision::ArcCylinderRoots => (
                "whether an arc crosses a cylinder wall, grazes it or misses it",
                Ending::Lever(
                    "Recourse: move the parts so the arc clearly crosses the cylinder or clearly \
                     misses it",
                    LeverPass::ByRung,
                ),
            ),
            BooleanDecision::SeamWedge => (
                "whether the two faces at a seam edge cross there or touch tangentially",
                Ending::Sized(
                    "Recourse: move the geometry so the faces at that seam meet either clearly \
                     creased or clearly smooth",
                    SizedPass::NonNegative,
                ),
            ),
            BooleanDecision::SeamJet => (
                "whether the two faces touching along a seam edge curve apart there or share \
                 their curvature",
                Ending::Sized(
                    "Recourse: move the geometry so the faces touching along that seam either \
                     clearly curve apart there or clearly share their curvature",
                    SizedPass::NonNegative,
                ),
            ),
            BooleanDecision::Sphere(SphereQuestion::AgainstPlane) => (
                "whether a sphere of one solid crosses the plane of a face of the other",
                Ending::Sized(AGAINST, SizedPass::NonZero),
            ),
            BooleanDecision::Sphere(SphereQuestion::Apart) => (
                "whether a sphere of each solid stands clear of the other",
                Ending::Sized(SPHERES, SizedPass::Positive),
            ),
            BooleanDecision::Sphere(SphereQuestion::Nested) => (
                "whether the smaller of two spheres lies strictly inside the larger",
                Ending::Sized(SPHERES, SizedPass::Positive),
            ),
            BooleanDecision::Sphere(SphereQuestion::EscapeParallel) => (
                "whether the plane faces a sphere pokes through are parallel",
                Ending::Lever(
                    "Recourse: move them so their boundaries cross, or so their curved faces \
                     stand further apart",
                    LeverPass::ZeroOnly,
                ),
            ),
            BooleanDecision::Sphere(SphereQuestion::RecutAlign) => (
                "whether a sphere's polar axis leans away from the face it pokes through",
                Ending::Sized(AGAINST, SizedPass::Positive),
            ),
            BooleanDecision::SelfCheck(SelfCheck::GermLine) => (
                "whether two faces meeting at a corner cross along a line",
                Ending::Defect,
            ),
            BooleanDecision::SelfCheck(SelfCheck::Normals) => (
                "whether the normals of two faces can be read",
                Ending::Defect,
            ),
            BooleanDecision::SelfCheck(SelfCheck::ArcFacing) => (
                "which way a germ turns about the section it lies on",
                Ending::Defect,
            ),
            BooleanDecision::SelfCheck(SelfCheck::RingWinding) => {
                ("which way a ring run of the section winds", Ending::Defect)
            }
            BooleanDecision::SelfCheck(SelfCheck::CarrierLadder) => (
                "whether a face of each solid lies on one surface",
                Ending::Defect,
            ),
            BooleanDecision::PierceCurvature => (
                "whether an edge leaves a curved face steeply enough against its bend to read \
                 which side it goes",
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
                Ending::Lever(LONGER, LeverPass::ByArm),
            ),
            BooleanDecision::Neighbours(PlaneRung::Parallel) => (
                NEIGHBOURS,
                Ending::Sized(NEIGHBOUR_ENDING, SizedPass::Positive),
            ),
            BooleanDecision::Neighbours(PlaneRung::Orientation) => (
                NEIGHBOURS,
                Ending::Lever(NEIGHBOUR_ENDING, LeverPass::Never),
            ),
            BooleanDecision::Neighbours(PlaneRung::Norm) => (NEIGHBOURS, Ending::Defect),
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
                "whether two vertices coincide",
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

    /// **A lever gate's reading is derived, not listed**: `LeverArm::ALL`
    /// is every gate, so the executed-offer harness's one-decision pairs
    /// (`test_support::offer_same_decision`, from `LeverArm::reading`) are
    /// every gate's, and each gate's arm rung and reading rung route to
    /// the gate and to its reading.
    #[test]
    fn every_lever_gate_pairs_with_the_reading_it_meters() {
        assert_eq!(LeverArm::ALL.to_vec(), LeverArm::iter().collect::<Vec<_>>());
        for gate in LeverArm::ALL {
            let read = DeclarationRead::Moot;
            assert_eq!(
                BooleanDecision::of_lever(gate, read, geom_brep::LeverRung::Arm),
                BooleanDecision::LeverArm(gate)
            );
            assert_eq!(
                BooleanDecision::of_lever(gate, read, geom_brep::LeverRung::Reading),
                gate.reading(read)
            );
        }
        let same = crate::test_support::offer_same_decision;
        assert!(same("LeverArm(SectorSide)", "Coincidence(SectorSide)"));
        assert!(same("Coincidence(TangentSide)", "LeverArm(SectorCurving)"));
        assert!(same("LeverArm(Seam)", "SeamWedge"));
        assert!(!same("LeverArm(Seam)", "Coincidence(SectorSide)"));
    }

    /// **`BooleanError::Escalated` ends as its decision and verdict
    /// give**, for every decision, on in-band margins of each sign (a
    /// point and an enclosure), an enclosure across zero, a signed zero,
    /// and an `INVALID` margin (the enclosure and zero rows are the
    /// review's `probe_c7_shape_guard_enclosures`):
    ///
    /// - every sentence opens on its decision's own subject, written
    ///   here as a literal, and passes the refusal-shape guard;
    /// - only a coincidence whose read settles it offers a declaration,
    ///   and only on a margin it can read;
    /// - a decision on a size names its lever and, on an in-band margin
    ///   on a side it passes on, the tolerance below which that margin is
    ///   decided passing; elsewhere no tolerance, and an `INVALID` margin
    ///   adds the unreadable-margin note;
    /// - a family the escalation does not tell apart, and a question
    ///   that passes only at zero, name their lever alone;
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
                let readable = !margin.is_invalid();
                assert_eq!(
                    text.contains("declare the coincidence"),
                    readable
                        && matches!(
                            decision,
                            BooleanDecision::Coincidence(_, DeclarationRead::Settles(_))
                        ),
                    "{label}: only a coincidence a declaration read ahead would settle offers \
                     one, and never on a margin no declaration reads: {text}"
                );
                let (subject, ending) = want(decision, readable);
                let head = format!("{subject} is undecided: {}. ", diag.payload());
                let tail = text.strip_prefix(&head);
                match ending {
                    Ending::Sized(lever, passes) => {
                        assert!(
                            matches!(decision.ending(&diag), super::Ending::Sized(d) if d.passes == passes),
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
                            matches!(decision.ending(&diag), super::Ending::Lever(_, p) if p == passes),
                            "{label}: the pass set the table states"
                        );
                        let want = if margin.is_invalid() {
                            format!("{lever}; {UNREADABLE_MARGIN_NOTE}")
                        } else {
                            lever.to_owned()
                        };
                        assert_eq!(tail, Some(want.as_str()), "{label}: {text}");
                    }
                    Ending::Frontier(what) => {
                        assert!(
                            matches!(decision.ending(&diag), super::Ending::Frontier(_)),
                            "{label}: a frontier"
                        );
                        assert_eq!(tail, Some(what), "{label}: {text}");
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

    /// **Every escalation renders within the viewer's word budget**, in
    /// band, with a point payload and with an enclosure payload on
    /// either side of zero and across it, wrapped as the feature tree's
    /// fault line draws it (the second review's words probe, adopted):
    /// under the 75 words editor-core's `refusal_concision` holds each
    /// refusal to, for every decision and every read a door can hand
    /// it.
    #[test]
    fn every_escalation_renders_within_the_viewers_word_budget() {
        const WRAPPER: &str = "node 5 failed: the Boolean op refused:";
        let (z, e) = (band().zero(), band().escalate());
        let margins = [
            MarginDiag::value(5.500000010982831e-9),
            MarginDiag::value(-5.500000010982831e-9),
            MarginDiag::enclosure(2.000000000000001 * z, 0.5000000000000001 * e),
            MarginDiag::enclosure(-0.5000000000000001 * e, -2.000000000000001 * z),
            MarginDiag::enclosure(-2.000000000000001 * z, 3.000000000000001 * z),
        ];
        let mut over = Vec::new();
        for decision in every_decision() {
            for margin in margins {
                let diag = Indeterminate {
                    margin,
                    band: band(),
                    predicate: Some("side_of_plane"),
                    terminal_sliver: false,
                };
                let text = format!("{WRAPPER} {}", BooleanError::Escalated { decision, diag });
                let words = text.split_whitespace().count();
                if words >= 75 {
                    over.push(format!("{decision:?} at {margin}: {words} words: {text}"));
                }
            }
        }
        assert!(over.is_empty(), "{}", over.join("\n"));
    }

    /// **A declaration is offered only where the lookup mints it**: the
    /// one constructor ([`crate::boolean::DeclaredPairs::read`]) settles
    /// a question only for a class the door admits and the question
    /// states it is settled by, spends a declared pair's class, and says
    /// `Moot` otherwise, over every question and class. Each settleable
    /// question's settling classes, stated here on their own.
    #[test]
    fn a_declaration_read_is_minted_by_the_lookup_alone() {
        use crate::boolean::{BooleanDeclarations, DeclaredPairs, FacePairDeclaration};
        let face = crate::entity::FaceKey::default();
        let pair = [(Operand::A, face, Operand::B, face)];
        let none =
            DeclaredPairs::<f64>::without_struts(&BooleanDeclarations::none(), Default::default());
        let settled = |which: Coincide| -> &'static [BooleanCoincidence] {
            match which {
                Coincide::OnPlanes => &[BooleanCoincidence::REST, BooleanCoincidence::Continuation],
                Coincide::Sectors => &[
                    BooleanCoincidence::REST,
                    BooleanCoincidence::TANGENT,
                    BooleanCoincidence::Continuation,
                ],
                Coincide::Planes
                | Coincide::VertexOnFace
                | Coincide::VertexOnCurvedFace
                | Coincide::VertexOnCoveredFace
                | Coincide::EdgeOnPlane
                | Coincide::EdgeOnCurvedFace
                | Coincide::ArcOnCoveredFace
                | Coincide::SectorSide
                | Coincide::TangentSide
                | Coincide::EdgeOnEdge
                | Coincide::FlankSense
                | Coincide::CurvedFlankSense
                | Coincide::TangentLocus
                | Coincide::Rim
                | Coincide::Contact
                | Coincide::DeclaredReach
                | Coincide::Section
                | Coincide::Join => &[],
            }
        };
        for which in Coincide::iter() {
            for &class in BooleanCoincidence::ALL {
                let got = none.read(&pair, which, &[class]);
                let want = if settled(which).contains(&class) {
                    DeclarationRead::Settles(Settling {
                        class,
                        question: which,
                    })
                } else {
                    DeclarationRead::Moot
                };
                assert_eq!(got, want, "{which:?} undeclared, {class:?} admitted");
                assert_eq!(
                    none.read(&pair, which, &[]),
                    DeclarationRead::Moot,
                    "{which:?}: a door that admits no class settles nothing"
                );
                let decls = BooleanDeclarations {
                    coincident_faces: vec![FacePairDeclaration::new(face, face, class)],
                    ..BooleanDeclarations::none()
                };
                let declared = DeclaredPairs::<f64>::without_struts(&decls, Default::default());
                assert_eq!(
                    declared.read(&pair, which, BooleanCoincidence::ALL),
                    DeclarationRead::Spent(class),
                    "{which:?} declared {class:?}: the declaration is spent"
                );
            }
        }
        // The plane door offers what the pair's senses make it, and
        // nothing on senses it could not read.
        use crate::boolean::CarrierRelation;
        for (senses, offer) in [
            (
                Some(CarrierRelation::SameOpposite),
                Some(BooleanCoincidence::REST),
            ),
            (
                Some(CarrierRelation::SameOriented),
                Some(BooleanCoincidence::Continuation),
            ),
            (Some(CarrierRelation::Distinct), None),
            (None, None),
        ] {
            let want = offer.map_or(DeclarationRead::Moot, |class| {
                DeclarationRead::Settles(Settling {
                    class,
                    question: Coincide::OnPlanes,
                })
            });
            assert_eq!(
                none.on_pair_door(pair[0], senses),
                PlaneDoor::OnPair(want),
                "senses {senses:?}"
            );
        }
    }

    /// **A read settles only the question the lookup minted it for** (the
    /// coincfr3 review's MINOR-2 probe, which carried an `OnPlanes` read
    /// to `FlankSense` and rendered "declare"): every settling read,
    /// carried to every other question, renders that question unsettled.
    ///
    /// Mutant: `BooleanDecision::ending` settling on any `Settles` read.
    #[test]
    fn a_settling_read_carried_to_another_question_offers_no_declaration() {
        use crate::boolean::{BooleanDeclarations, DeclaredPairs};
        let face = crate::entity::FaceKey::default();
        let pair = [(Operand::A, face, Operand::B, face)];
        let none =
            DeclaredPairs::<f64>::without_struts(&BooleanDeclarations::none(), Default::default());
        let diag = diag_of(MarginDiag::value((band().zero() + band().escalate()) / 2.0));
        let mut carried_any = 0;
        for minted in Coincide::iter() {
            for &class in BooleanCoincidence::ALL {
                let read = none.read(&pair, minted, &[class]);
                if !matches!(read, DeclarationRead::Settles(_)) {
                    continue;
                }
                for carried in Coincide::iter() {
                    let decision = BooleanDecision::Coincidence(carried, read);
                    let text = BooleanError::Escalated { decision, diag }.to_string();
                    assert_eq!(
                        text.contains("declare the coincidence"),
                        carried == minted,
                        "{minted:?}'s {class:?} read carried to {carried:?}: {text}"
                    );
                    carried_any += usize::from(carried != minted);
                }
            }
        }
        assert!(carried_any > 0, "the lookup mints a settling read to carry");
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

    /// **An ellipse carrier's escalation reads by door**: the section
    /// through a curved face whose kind the carrier constructor could not
    /// decide says which question it hinged on, offers the split the
    /// geometry and the tolerance alone and the Boolean its declaration
    /// too, and offers neither a carrier to construct.
    #[test]
    fn an_ellipse_carrier_escalation_is_routed_by_door() {
        let b = band();
        let rows = [
            (
                "ellipse_axes_distinct",
                "whether the curve is a circle or an ellipse",
            ),
            (
                "ellipse_minor_positive",
                "whether the curve's minor semi-axis is positive",
            ),
        ];
        for (predicate, subject) in rows {
            let diag = Indeterminate {
                predicate: Some(predicate),
                ..diag_of(MarginDiag::value((b.zero() + b.escalate()) / 2.0))
            };
            let join = || crate::SplitJoinError::Section {
                face: crate::entity::FaceKey::default(),
                source: geom_brep::SectionError::Carrier(geom::EllipseInvalid::Escalated(diag)),
            };
            let split = crate::SplitError::Join(join()).to_string();
            let boolean = BooleanError::Join(join()).to_string();
            for text in [&split, &boolean] {
                let problems = short_of_the_guard(text, &[]);
                assert!(problems.is_empty(), "{predicate}: {problems:?}: {text}");
                assert!(
                    text.contains(&format!(
                        "{subject} is undecided for the section through a curved face: {}. ",
                        diag.payload()
                    )) && !text.contains("Circle carrier"),
                    "{predicate}: {text}"
                );
            }
            assert!(
                split.ends_with("Recourse: move the geometry, or lower the tolerance")
                    && !split.contains("declare"),
                "{predicate}: the split takes no declaration: {split}"
            );
            assert!(
                boolean.ends_with(&format!("Recourse: {}", geom_core::COINCIDENCE_RECOURSE)),
                "{predicate}: the Boolean takes one: {boolean}"
            );
        }
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
                decision: BooleanDecision::of_conic_root(fault, DeclarationRead::Moot),
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
    /// serves for planes) over a metre about the origin, with points on
    /// `c1` known: the contradiction it raises.
    fn contradicted(c1: CarrierDesc<f64>, c2: CarrierDesc<f64>) -> (Contradiction, Indeterminate) {
        let id = PlaneIdentity {
            s1: None,
            s2: None,
            declared: true,
        };
        let on = crate::boolean::carrier_eq::points_on(&c1);
        let extent = crate::boolean::ConsumedExtent {
            on: [&on, &[]],
            ..crate::boolean::ConsumedExtent::arm(1.0)
        };
        match crate::boolean::carrier_eq(&c1, &c2, id, &extent, band()) {
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
    /// the rung decided. Both definite signs pass the rung, but the
    /// offset rung asks next, where an undeclared coincident pair refuses
    /// at every tolerance (executed: `offer_rows`'
    /// `planes_facing_at_a_short_arm`), so no tolerance is offered; the
    /// lever lengthens the arm, the one thing an undecided margin
    /// measures.
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
                    let err = oriented_plane_eq(
                        &p1,
                        &p2,
                        id,
                        &crate::boolean::ConsumedExtent::arm(arm),
                        b,
                    )
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
                        PlaneDoor::OnPair(if id.declared {
                            DeclarationRead::Spent(BooleanCoincidence::REST)
                        } else {
                            DeclarationRead::Moot
                        }),
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
                    assert_eq!(offered_below(&text), None, "{label}: {text}");
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
    /// (positive margins) and opposite ways (negative), over a reach in
    /// the zero band, in the ambiguity band, and definite. The
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
                             way, on faces that clearly span a length";
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        let (f1, f2) = (FaceKey::default(), FaceKey::default());
        for (chord, definite) in [(0.5 * z, false), ((z + e) / 2.0, false), (1.0, true)] {
            for sign in [1.0, -1.0] {
                let label = format!("chord {chord:e}, facing {sign}");
                let (p1, p2) = planes(sign);
                let verdict = declared_pair_verdict(
                    oriented_plane_eq(
                        &p1,
                        &p2,
                        DECLARED,
                        &crate::boolean::ConsumedExtent::arm(chord),
                        b,
                    ),
                    f1,
                    f2,
                );
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
    pub(super) fn top_split_redescribed(
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
        // Lifts both refusals: the re-described half is the gate's input; its edges are not the row.
        body.set_face_surface_stranding_for_tests(
            half.face,
            crate::euler::FaceSurface::New {
                surface: plane(p0, along, diagonal),
                sense: true,
            },
        )
        .expect("a face takes a plane through its diagonal");
        // The diagonal rests in the untouched half's chart, which holds
        // it exactly; left the chord's scaffold, no result keeping it
        // passes the result gate.
        let chart = body.get_face(prism.top_face).expect("the top face").surface;
        body.set_edge_curve(
            half.edge,
            geom_brep::EdgeCurveSpec::line_between(p0, p1).at_rest_in_chart(chart, false),
            tol,
        )
        .expect("the diagonal rests in the top's chart");
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
    fn an_unreadable_norm_and_the_neighbour_gate_end_where_their_door_reaches() {
        use crate::boolean::plane_eq::unreadable_norm;
        use crate::boolean::{PlaneDesc, PlaneDoor, PlaneEqError, PlaneRung, oriented_plane_eq};
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        let PlaneEqError::Escalated {
            rung: PlaneRung::Norm,
            diag,
        } = unreadable_norm(b)
        else {
            panic!("the unreadable norm is its own rung");
        };
        // Every door a plane rung reaches, the undeclared ones included:
        // no declaration reads poison, so none is offered, and the
        // refusal is the kernel's.
        let undeclared = crate::boolean::DeclaredPairs::<f64>::without_struts(
            &crate::boolean::BooleanDeclarations::none(),
            Default::default(),
        );
        let face = crate::entity::FaceKey::default();
        let doors = [
            undeclared.on_pair_door(
                (Operand::A, face, Operand::B, face),
                Some(crate::boolean::CarrierRelation::SameOpposite),
            ),
            PlaneDoor::OnPair(DeclarationRead::Spent(BooleanCoincidence::REST)),
            PlaneDoor::Screen(DeclarationRead::Spent(BooleanCoincidence::TANGENT)),
            PlaneDoor::Neighbours,
        ];
        let merge = MergeCoplanarError::of_declared_refusal(unreadable_norm(b)).to_string();
        let texts =
            doors.map(|door| BooleanError::plane_identity(PlaneRung::Norm, door, diag).to_string());
        for text in texts.iter().chain([&merge]) {
            let problems = short_of_the_guard(text, &[]);
            assert!(problems.is_empty(), "{problems:?}: {text}");
            assert_eq!(
                text.strip_prefix("whether the normals of two faces can be read is undecided: ")
                    .and_then(|t| t.split_once(". "))
                    .map(|(_, ending)| ending),
                Some(KERNEL_DEFECT_ENDING),
                "an unreadable norm is a defect at every door: {text}"
            );
        }
        const GATE: &str = "Recourse: merge the two faces into one first (merge_coplanar_faces), \
                            or tilt one to meet at a clear angle along an edge clearly longer \
                            than the tolerance";
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
            let err = oriented_plane_eq(
                &flat,
                &p2,
                PlaneIdentity::NONE,
                &crate::boolean::ConsumedExtent::arm(chord),
                b,
            )
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
