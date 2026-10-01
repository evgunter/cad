//! [`NodeErrorClass`]: which arm of [`NodeErrorKind`] refused, without
//! the payload.

use sweep::blend::BlendKind;

use super::{NodeErrorKind, PartFault};
use crate::mate::MateFault;
use crate::node::PlacementRuleFault;
use crate::part::ResolveFault;

/// Which arm of [`NodeErrorKind`] refused, without the payload.
///
/// A [`NodeErrorKind`] is neither `Clone` nor `PartialEq` — it carries
/// kernel refusals unaltered, and those have neither. A consumer that
/// must keep the whole refusal holds a [`crate::NodeRefusal`], which is
/// `Clone + Eq` by sharing it; one that must branch on WHICH refusal it
/// is, hash it or name it in a test reads this class, which drops the
/// payload and is `Copy + Eq + Hash`. It is what the Python binding's
/// tag map reads.
///
/// One variant per [`NodeErrorKind`] arm, except where the arm carries a
/// value that decides which refusal it is to a caller: there, one
/// variant per value, named for the arm and then the value. The two
/// blends share one kernel error type and one arm, so the VERB decides
/// ([`Self::Fillet`], [`Self::Chamfer`], and their selection refusals);
/// a placement rule's fault, a carried frame direction's fact, an
/// instantiation's fault and a mate's fault each decide theirs. Nothing
/// else in a payload is read.
///
/// [`NodeErrorKind::class`] matches exhaustively, and so does every
/// split inside it but the carried frame direction's — an arm added to
/// the error, or a value added to a split, reds `class` itself, here in
/// this crate. The frame direction's split is read off the raise it
/// carries ([`Self::raised`]), so a new direction fact reds at that
/// raise's own map in `wire`.
///
/// A variant HERE with no arm behind it is a phantom: nothing constructs
/// it, so no test can reach it. This module's tests therefore carry the
/// visit that reds one — an exhaustive match over this enum, which names
/// the phantom at compile time. The fix at that red is to delete the
/// phantom, never to give it a label: a name minted for a phantom
/// publishes a class no refusal can ever carry.
///
/// Deliberately NOT `Ord`. The declaration order mirrors
/// [`NodeErrorKind`]'s for reading, and nothing depends on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeErrorClass {
    /// [`NodeErrorKind::Expr`].
    Expr,
    /// [`NodeErrorKind::Profile`].
    Profile,
    /// [`NodeErrorKind::ProfileReplay`].
    ProfileReplay,
    /// [`NodeErrorKind::ProfileLaneReplay`].
    ProfileLaneReplay,
    /// [`NodeErrorKind::ProfileAnchor`].
    ProfileAnchor,
    /// [`NodeErrorKind::ProfilePieces`].
    ProfilePieces,
    /// [`NodeErrorKind::Extrude`].
    Extrude,
    /// [`NodeErrorKind::Revolve`].
    Revolve,
    /// [`NodeErrorKind::Tube`].
    Tube,
    /// [`NodeErrorKind::Split`].
    Split,
    /// [`NodeErrorKind::Blend`] refused by a fillet.
    Fillet,
    /// [`NodeErrorKind::Blend`] refused by a chamfer.
    Chamfer,
    /// [`NodeErrorKind::Boolean`].
    Boolean,
    /// [`NodeErrorKind::Transform`].
    Transform,
    /// [`NodeErrorKind::Skin`].
    Skin,
    /// [`NodeErrorKind::Loft`].
    Loft,
    /// [`NodeErrorKind::CurvedSolidFrontier`].
    CurvedSolidFrontier,
    /// [`NodeErrorKind::MissingInput`].
    MissingInput,
    /// [`NodeErrorKind::ToleranceConflict`].
    ToleranceConflict,
    /// [`NodeErrorKind::ParamBox`].
    ParamBox,
    /// [`NodeErrorKind::Seed`].
    Seed,
    /// [`NodeErrorKind::SeedPinnedSection`].
    SeedPinnedSection,
    /// [`NodeErrorKind::WrongOperand`].
    WrongOperand,
    /// [`NodeErrorKind::EmptyOperand`].
    EmptyOperand,
    /// [`NodeErrorKind::EmptyHalf`].
    EmptyHalf,
    /// [`NodeErrorKind::InstanceOutOfRange`].
    InstanceOutOfRange,
    /// [`NodeErrorKind::DegenerateDirection`].
    DegenerateDirection,
    /// [`NodeErrorKind::NonFiniteDirection`].
    NonFiniteDirection,
    /// [`NodeErrorKind::UnderflowedDirection`].
    UnderflowedDirection,
    /// [`NodeErrorKind::Band`].
    Band,
    /// [`NodeErrorKind::MissingSlot`].
    MissingSlot,
    /// [`NodeErrorKind::VerbArity`].
    VerbArity,
    /// [`NodeErrorKind::Escalated`].
    Escalated,
    /// [`NodeErrorKind::AxisInDifferentPlane`].
    AxisInDifferentPlane,
    /// [`NodeErrorKind::NonPositiveCount`].
    NonPositiveCount,
    /// [`NodeErrorKind::PlacementsUncertified`].
    PlacementsUncertified,
    /// [`NodeErrorKind::PlacementRule`] carrying
    /// [`PlacementRuleFault::CountSpelling`].
    PlacementRuleCountSpelling,
    /// [`NodeErrorKind::PlacementRule`] carrying
    /// [`PlacementRuleFault::NoPlacements`].
    PlacementRuleNoPlacements,
    /// [`NodeErrorKind::PlacementRule`] carrying
    /// [`PlacementRuleFault::NonFiniteFrame`].
    PlacementRuleNonFiniteFrame,
    /// [`NodeErrorKind::PlacementRule`] carrying
    /// [`PlacementRuleFault::ImproperFrame`].
    PlacementRuleImproperFrame,
    /// [`NodeErrorKind::PlacementRule`] carrying
    /// [`PlacementRuleFault::NonRigidFrame`].
    PlacementRuleNonRigidFrame,
    /// [`NodeErrorKind::UnschedulableCycle`].
    UnschedulableCycle,
    /// [`NodeErrorKind::Naming`].
    Naming,
    /// [`NodeErrorKind::ParamSourceAttach`].
    ParamSourceAttach,
    /// [`NodeErrorKind::DeclareResolve`].
    DeclareResolve,
    /// [`NodeErrorKind::DeclareSiteNotAnOperand`].
    DeclareSiteNotAnOperand,
    /// [`NodeErrorKind::DeclareUnsupportedPair`].
    DeclareUnsupportedPair,
    /// [`NodeErrorKind::UndeclaredContact`].
    UndeclaredContact,
    /// [`NodeErrorKind::UndeclarableContact`].
    UndeclarableContact,
    /// [`NodeErrorKind::BlendSelectionResolve`] refused by a fillet.
    FilletSelectionResolve,
    /// [`NodeErrorKind::BlendSelectionResolve`] refused by a chamfer.
    ChamferSelectionResolve,
    /// [`NodeErrorKind::BlendSelectionKind`] refused by a fillet.
    FilletSelectionKind,
    /// [`NodeErrorKind::BlendSelectionKind`] refused by a chamfer.
    ChamferSelectionKind,
    /// [`NodeErrorKind::BlendSelectionEmpty`] refused by a fillet.
    FilletSelectionEmpty,
    /// [`NodeErrorKind::BlendSelectionEmpty`] refused by a chamfer.
    ChamferSelectionEmpty,
    /// [`NodeErrorKind::Shell`].
    Shell,
    /// [`NodeErrorKind::ShellOpenResolve`].
    ShellOpenResolve,
    /// [`NodeErrorKind::ShellOpenKind`].
    ShellOpenKind,
    /// [`NodeErrorKind::ShellLaneUnsupported`].
    ShellLaneUnsupported,
    /// [`NodeErrorKind::FaceFrameResolve`].
    FaceFrameResolve,
    /// [`NodeErrorKind::FaceFrameKind`].
    FaceFrameKind,
    /// [`NodeErrorKind::FaceFrameNotPlanar`].
    FaceFrameNotPlanar,
    /// [`NodeErrorKind::FaceFrameReadback`].
    FaceFrameReadback,
    /// [`NodeErrorKind::DerivedFrameSection`].
    DerivedFrameSection,
    /// [`NodeErrorKind::FrameDirection`] carrying
    /// [`geom_core::UnitVec3Error::Degenerate`].
    FrameDirectionDegenerate,
    /// [`NodeErrorKind::FrameDirection`] carrying
    /// [`geom_core::UnitVec3Error::NonFiniteLength`].
    FrameDirectionNonFiniteLength,
    /// [`NodeErrorKind::FrameDirection`] carrying
    /// [`geom_core::UnitVec3Error::UnderflowedLength`].
    FrameDirectionUnderflowedLength,
    /// [`NodeErrorKind::FrameDirection`] carrying
    /// [`geom_core::UnitVec3Error::Escalated`].
    FrameDirectionEscalated,
    /// [`NodeErrorKind::WitnessBifurcation`].
    WitnessBifurcation,
    /// [`NodeErrorKind::Part`] carrying [`PartFault::NoResolver`].
    PartNoResolver,
    /// [`NodeErrorKind::Part`] carrying [`PartFault::Unresolved`] with
    /// [`ResolveFault::PinMismatch`].
    PartPinMismatch,
    /// [`NodeErrorKind::Part`] carrying [`PartFault::Unresolved`] with
    /// [`ResolveFault::EpsilonSeam`].
    PartEpsilonSeam,
    /// [`NodeErrorKind::Part`] carrying [`PartFault::Unresolved`] with
    /// [`ResolveFault::Unresolved`].
    PartUnresolved,
    /// [`NodeErrorKind::Part`] carrying [`PartFault::PartRootFailed`].
    PartRootFailed,
    /// [`NodeErrorKind::Part`] carrying [`PartFault::PartRootPoisoned`].
    PartRootPoisoned,
    /// [`NodeErrorKind::Part`] carrying
    /// [`PartFault::RootFailureUnrecorded`].
    PartRootFailureUnrecorded,
    /// [`NodeErrorKind::Part`] carrying [`PartFault::PartProduct`].
    PartProduct,
    /// [`NodeErrorKind::Part`] carrying [`PartFault::ReferenceCycle`].
    PartReferenceCycle,
    /// [`NodeErrorKind::Part`] carrying [`PartFault::DepthExceeded`].
    PartDepthExceeded,
    /// [`NodeErrorKind::Part`] carrying [`PartFault::NotEntered`].
    PartNotEntered,
    /// [`NodeErrorKind::Mate`] carrying
    /// [`MateFault::PosesOfAnotherDocument`].
    MatePosesOfAnotherDocument,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::Frame`].
    MateFrame,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::ClassNotAdmitted`].
    MateClassNotAdmitted,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::TableLacks`].
    MateTableLacks,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::Indeterminate`].
    MateIndeterminate,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::Band`].
    MateBand,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::Contradictory`].
    MateContradictory,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::Under`].
    MateUnder,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::DanglingHead`].
    MateDanglingHead,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::PlacerRefused`].
    MatePlacerRefused,
    /// [`NodeErrorKind::Mate`] carrying
    /// [`MateFault::PartSelectsAnotherCopy`].
    MatePartSelectsAnotherCopy,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::SelfMate`].
    MateSelf,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::Unleverable`].
    MateUnleverable,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::FaceUnresolved`].
    MateFaceUnresolved,
    /// [`NodeErrorKind::CrossingUnverified`].
    CrossingUnverified,
    /// [`NodeErrorKind::MeasureRefResolve`].
    MeasureRefResolve,
    /// [`NodeErrorKind::MeasureRefUnreadable`].
    MeasureRefUnreadable,
    /// [`NodeErrorKind::MeasureNonFinite`].
    MeasureNonFinite,
    /// [`NodeErrorKind::MeasureNotParallel`].
    MeasureNotParallel,
    /// [`NodeErrorKind::MeasureUnsupported`].
    MeasureUnsupported,
    /// [`NodeErrorKind::MeasureMalformed`].
    MeasureMalformed,
    /// [`NodeErrorKind::PayloadExpr`].
    PayloadExpr,
    /// [`NodeErrorKind::MeasureSelectionKind`].
    MeasureSelectionKind,
    /// [`NodeErrorKind::MeasureClearanceRefused`].
    MeasureClearanceRefused,
    /// [`NodeErrorKind::AssertionDimension`].
    AssertionDimension,
}

impl NodeErrorKind {
    /// Which arm refused — and, where the arm splits, which value it
    /// carries — without the payload.
    ///
    /// Exhaustive over [`NodeErrorKind`] and over every value it splits
    /// on: adding an arm or a value is a compile error here.
    #[must_use]
    pub fn class(&self) -> NodeErrorClass {
        use NodeErrorClass as C;
        match self {
            Self::Expr { .. } => C::Expr,
            Self::Profile(_) => C::Profile,
            Self::ProfileReplay { .. } => C::ProfileReplay,
            Self::ProfileLaneReplay { .. } => C::ProfileLaneReplay,
            Self::ProfileAnchor { .. } => C::ProfileAnchor,
            Self::ProfilePieces { .. } => C::ProfilePieces,
            Self::Extrude(_) => C::Extrude,
            Self::Revolve(_) => C::Revolve,
            Self::Tube(_) => C::Tube,
            Self::Split(_) => C::Split,
            Self::Blend { verb, .. } => by_verb(*verb, C::Fillet, C::Chamfer),
            Self::Boolean(_) => C::Boolean,
            Self::Transform(_) => C::Transform,
            Self::Skin(_) => C::Skin,
            Self::Loft(_) => C::Loft,
            Self::CurvedSolidFrontier { .. } => C::CurvedSolidFrontier,
            Self::MissingInput { .. } => C::MissingInput,
            Self::ToleranceConflict { .. } => C::ToleranceConflict,
            Self::ParamBox { .. } => C::ParamBox,
            Self::Seed { .. } => C::Seed,
            Self::SeedPinnedSection { .. } => C::SeedPinnedSection,
            Self::WrongOperand { .. } => C::WrongOperand,
            Self::EmptyOperand { .. } => C::EmptyOperand,
            Self::EmptyHalf { .. } => C::EmptyHalf,
            Self::InstanceOutOfRange { .. } => C::InstanceOutOfRange,
            Self::DegenerateDirection { .. } => C::DegenerateDirection,
            Self::NonFiniteDirection { .. } => C::NonFiniteDirection,
            Self::UnderflowedDirection { .. } => C::UnderflowedDirection,
            Self::Band(_) => C::Band,
            Self::MissingSlot { .. } => C::MissingSlot,
            Self::VerbArity { .. } => C::VerbArity,
            Self::Escalated { .. } => C::Escalated,
            Self::AxisInDifferentPlane { .. } => C::AxisInDifferentPlane,
            Self::NonPositiveCount { .. } => C::NonPositiveCount,
            Self::PlacementsUncertified { .. } => C::PlacementsUncertified,
            Self::PlacementRule(fault) => C::of_placement_rule(fault),
            Self::UnschedulableCycle => C::UnschedulableCycle,
            Self::Naming(_) => C::Naming,
            Self::ParamSourceAttach(_) => C::ParamSourceAttach,
            Self::DeclareResolve { .. } => C::DeclareResolve,
            Self::DeclareSiteNotAnOperand { .. } => C::DeclareSiteNotAnOperand,
            Self::DeclareUnsupportedPair { .. } => C::DeclareUnsupportedPair,
            Self::UndeclaredContact { .. } => C::UndeclaredContact,
            Self::UndeclarableContact { .. } => C::UndeclarableContact,
            Self::BlendSelectionResolve { verb, .. } => {
                by_verb(*verb, C::FilletSelectionResolve, C::ChamferSelectionResolve)
            }
            Self::BlendSelectionKind { verb, .. } => {
                by_verb(*verb, C::FilletSelectionKind, C::ChamferSelectionKind)
            }
            Self::BlendSelectionEmpty { verb } => {
                by_verb(*verb, C::FilletSelectionEmpty, C::ChamferSelectionEmpty)
            }
            Self::Shell(_) => C::Shell,
            Self::ShellOpenResolve { .. } => C::ShellOpenResolve,
            Self::ShellOpenKind { .. } => C::ShellOpenKind,
            Self::ShellLaneUnsupported { .. } => C::ShellLaneUnsupported,
            Self::FaceFrameResolve { .. } => C::FaceFrameResolve,
            Self::FaceFrameKind { .. } => C::FaceFrameKind,
            Self::FaceFrameNotPlanar { .. } => C::FaceFrameNotPlanar,
            Self::FaceFrameReadback { .. } => C::FaceFrameReadback,
            Self::DerivedFrameSection { .. } => C::DerivedFrameSection,
            Self::FrameDirection { refusal, .. } => {
                let raised = refusal.node_error().class();
                FRAME_DIRECTION
                    .into_iter()
                    .find(|carried| carried.raised() == raised)
                    .unwrap_or_else(|| {
                        unreachable!(
                            "a direction refusal raises one of four classes, not {raised:?}"
                        )
                    })
            }
            Self::WitnessBifurcation(_) => C::WitnessBifurcation,
            Self::Part { fault, .. } => match fault {
                PartFault::NoResolver => C::PartNoResolver,
                PartFault::Unresolved { fault, .. } => match fault {
                    ResolveFault::PinMismatch => C::PartPinMismatch,
                    ResolveFault::EpsilonSeam => C::PartEpsilonSeam,
                    ResolveFault::Unresolved => C::PartUnresolved,
                },
                PartFault::PartRootFailed { .. } => C::PartRootFailed,
                PartFault::PartRootPoisoned { .. } => C::PartRootPoisoned,
                PartFault::RootFailureUnrecorded { .. } => C::PartRootFailureUnrecorded,
                PartFault::PartProduct { .. } => C::PartProduct,
                PartFault::ReferenceCycle { .. } => C::PartReferenceCycle,
                PartFault::DepthExceeded => C::PartDepthExceeded,
                PartFault::NotEntered => C::PartNotEntered,
            },
            Self::Mate(fault) => C::of_mate(fault),
            Self::CrossingUnverified { .. } => C::CrossingUnverified,
            Self::MeasureRefResolve { .. } => C::MeasureRefResolve,
            Self::MeasureRefUnreadable { .. } => C::MeasureRefUnreadable,
            Self::MeasureNonFinite { .. } => C::MeasureNonFinite,
            Self::MeasureNotParallel { .. } => C::MeasureNotParallel,
            Self::MeasureUnsupported(_) => C::MeasureUnsupported,
            Self::MeasureMalformed(_) => C::MeasureMalformed,
            Self::PayloadExpr { .. } => C::PayloadExpr,
            Self::MeasureSelectionKind { .. } => C::MeasureSelectionKind,
            Self::MeasureClearanceRefused(_) => C::MeasureClearanceRefused,
            Self::AssertionDimension { .. } => C::AssertionDimension,
        }
    }
}

/// The classes of [`NodeErrorKind::FrameDirection`], one per fact the
/// carried refusal can report.
const FRAME_DIRECTION: [NodeErrorClass; 4] = [
    NodeErrorClass::FrameDirectionDegenerate,
    NodeErrorClass::FrameDirectionNonFiniteLength,
    NodeErrorClass::FrameDirectionUnderflowedLength,
    NodeErrorClass::FrameDirectionEscalated,
];

impl NodeErrorClass {
    /// The class of the node refusal that carries a placement-rule
    /// fault. Every door that publishes the fault reads it through this
    /// class, so the fault has one word wherever it is refused.
    #[must_use]
    pub fn of_placement_rule(fault: &PlacementRuleFault) -> Self {
        match fault {
            PlacementRuleFault::CountSpelling => Self::PlacementRuleCountSpelling,
            PlacementRuleFault::NoPlacements => Self::PlacementRuleNoPlacements,
            PlacementRuleFault::NonFiniteFrame { .. } => Self::PlacementRuleNonFiniteFrame,
            PlacementRuleFault::ImproperFrame { .. } => Self::PlacementRuleImproperFrame,
            PlacementRuleFault::NonRigidFrame { .. } => Self::PlacementRuleNonRigidFrame,
        }
    }

    /// The class of the node refusal that carries a mate fault. Every
    /// door that publishes the fault reads it through this class, so
    /// the fault has one word wherever it is refused.
    #[must_use]
    pub fn of_mate(fault: &MateFault) -> Self {
        match fault {
            MateFault::PosesOfAnotherDocument { .. } => Self::MatePosesOfAnotherDocument,
            MateFault::Frame { .. } => Self::MateFrame,
            MateFault::ClassNotAdmitted { .. } => Self::MateClassNotAdmitted,
            MateFault::TableLacks { .. } => Self::MateTableLacks,
            MateFault::Indeterminate { .. } => Self::MateIndeterminate,
            MateFault::Band { .. } => Self::MateBand,
            MateFault::Contradictory { .. } => Self::MateContradictory,
            MateFault::Under { .. } => Self::MateUnder,
            MateFault::DanglingHead { .. } => Self::MateDanglingHead,
            MateFault::PlacerRefused { .. } => Self::MatePlacerRefused,
            MateFault::PartSelectsAnotherCopy { .. } => Self::MatePartSelectsAnotherCopy,
            MateFault::SelfMate { .. } => Self::MateSelf,
            MateFault::Unleverable { .. } => Self::MateUnleverable,
            MateFault::FaceUnresolved { .. } => Self::MateFaceUnresolved,
        }
    }

    /// The class the same refusal has where the direction door raises
    /// it. A frame direction refusal carried to a profile is the fact
    /// the frame raised, so each of its classes answers the raise's
    /// class; every other class answers itself.
    ///
    /// This is the one place the carried classes are paired with the
    /// raised ones, and the question to ask of a direction: "any
    /// zero-length direction, raised or carried" is
    /// `class.raised() == NodeErrorClass::DegenerateDirection`, and
    /// likewise [`Self::NonFiniteDirection`] and
    /// [`Self::UnderflowedDirection`].
    #[must_use]
    pub fn raised(self) -> Self {
        match self {
            Self::FrameDirectionDegenerate => Self::DegenerateDirection,
            Self::FrameDirectionNonFiniteLength => Self::NonFiniteDirection,
            Self::FrameDirectionUnderflowedLength => Self::UnderflowedDirection,
            Self::FrameDirectionEscalated => Self::Escalated,
            other => other,
        }
    }
}

/// The class a blend verb's refusal is, of the two its arm splits into.
fn by_verb(verb: BlendKind, fillet: NodeErrorClass, chamfer: NodeErrorClass) -> NodeErrorClass {
    match verb {
        BlendKind::Fillet => fillet,
        BlendKind::Chamfer => chamfer,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{NodeErrorClass as C, NodeErrorKind as K};
    use crate::names::{EntityKey, EntityKind, StableName};
    use crate::node::RecipeNodeId;
    use geom_core::{Band, BandError, Indeterminate, MarginDiag, Tol, UnitVec3Error};
    use sweep::blend::BlendKind;

    /// Every class, declared ONCE for two uses: [`ALL`], which the
    /// census walks, and an exhaustive match over the class. A class
    /// left out of the list is a non-exhaustive match (E0004) and one
    /// listed twice an unreachable pattern, so `ALL` is every class
    /// exactly once by construction.
    macro_rules! all_classes {
        ($($v:ident),* $(,)?) => {
            const ALL: &[C] = &[$(C::$v),*];
            #[deny(unreachable_patterns)]
            #[allow(dead_code)]
            fn all_is_exhaustive(c: C) {
                match c {
                    $(C::$v)|* => {}
                }
            }
        };
    }

    all_classes! {
        Expr,
        Profile,
        ProfileReplay,
        ProfileLaneReplay,
        ProfileAnchor,
        ProfilePieces,
        Extrude,
        Revolve,
        Tube,
        Split,
        Fillet,
        Chamfer,
        Boolean,
        Transform,
        Skin,
        Loft,
        CurvedSolidFrontier,
        MissingInput,
        ToleranceConflict,
        ParamBox,
        Seed,
        SeedPinnedSection,
        WrongOperand,
        EmptyOperand,
        EmptyHalf,
        InstanceOutOfRange,
        DegenerateDirection,
        NonFiniteDirection,
        UnderflowedDirection,
        Band,
        MissingSlot,
        VerbArity,
        Escalated,
        AxisInDifferentPlane,
        NonPositiveCount,
        PlacementsUncertified,
        PlacementRuleCountSpelling,
        PlacementRuleNoPlacements,
        PlacementRuleNonFiniteFrame,
        PlacementRuleImproperFrame,
        PlacementRuleNonRigidFrame,
        UnschedulableCycle,
        Naming,
        ParamSourceAttach,
        DeclareResolve,
        DeclareSiteNotAnOperand,
        DeclareUnsupportedPair,
        UndeclaredContact,
        UndeclarableContact,
        FilletSelectionResolve,
        ChamferSelectionResolve,
        FilletSelectionKind,
        ChamferSelectionKind,
        FilletSelectionEmpty,
        ChamferSelectionEmpty,
        Shell,
        ShellOpenResolve,
        ShellOpenKind,
        ShellLaneUnsupported,
        FaceFrameResolve,
        FaceFrameKind,
        FaceFrameNotPlanar,
        FaceFrameReadback,
        DerivedFrameSection,
        FrameDirectionDegenerate,
        FrameDirectionNonFiniteLength,
        FrameDirectionUnderflowedLength,
        FrameDirectionEscalated,
        WitnessBifurcation,
        PartNoResolver,
        PartPinMismatch,
        PartEpsilonSeam,
        PartUnresolved,
        PartRootFailed,
        PartRootPoisoned,
        PartRootFailureUnrecorded,
        PartProduct,
        PartReferenceCycle,
        PartDepthExceeded,
        PartNotEntered,
        MatePosesOfAnotherDocument,
        MateFrame,
        MateClassNotAdmitted,
        MateTableLacks,
        MateIndeterminate,
        MateBand,
        MateContradictory,
        MateUnder,
        MateDanglingHead,
        MatePlacerRefused,
        MatePartSelectsAnotherCopy,
        MateSelf,
        MateUnleverable,
        MateFaceUnresolved,
        CrossingUnverified,
        MeasureRefResolve,
        MeasureRefUnreadable,
        MeasureNonFinite,
        MeasureNotParallel,
        MeasureUnsupported,
        MeasureMalformed,
        PayloadExpr,
        MeasureSelectionKind,
        MeasureClearanceRefused,
        AssertionDimension,
    }

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness band")
    }

    fn band_error() -> BandError {
        BandError::Empty {
            zero: 1.0e-6,
            escalate: 1.0e-9,
        }
    }

    fn diag() -> Indeterminate {
        Indeterminate {
            margin: MarginDiag::value(3.0e-10),
            band: band(),
            predicate: Some("side_of_plane"),
            terminal_sliver: false,
        }
    }

    fn name() -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(3),
            path: Vec::new(),
        }
    }

    fn resolve_error() -> Box<crate::ResolveError> {
        Box::new(crate::ResolveError::NodeGone {
            name: name(),
            edit: crate::RecipeEditRef::NodeDeleted {
                node: RecipeNodeId(3),
            },
        })
    }

    /// The entity door's own answer for a whole body, which is the only
    /// way a [`crate::Found`] is minted.
    fn found_body(refuse: fn(crate::Found) -> K) -> K {
        crate::eval::entity_door::entity(EntityKey::Body, |_| None::<()>, refuse)
            .expect_err("a read that finds nothing refuses")
    }

    fn doc_ref() -> crate::DocRef {
        crate::DocRef {
            id: crate::DocumentId::derive("bracket"),
            pin: crate::ContentPin::of_bytes(b"bracket v3"),
        }
    }

    fn part(fault: crate::PartFault) -> K {
        K::Part {
            doc_ref: doc_ref(),
            fault,
        }
    }

    fn mate(fault: crate::MateFault) -> K {
        K::Mate(Box::new(fault))
    }

    fn frame_direction(error: UnitVec3Error) -> K {
        K::FrameDirection {
            profile: RecipeNodeId(4),
            frame: RecipeNodeId(2),
            refusal: crate::DirectionRefusal {
                role: "frame normal",
                error,
            },
        }
    }

    /// **The census and the phantom visit, in one match.** One witness
    /// per class, keyed by the class it must project to.
    ///
    /// Exhaustive over [`C`], so a variant added to the class alone reds
    /// HERE, by name, at compile time. The witness written for it is
    /// then checked to project onto it, which no arm of the error can do
    /// for a phantom — so a witness borrowed from another class reds at
    /// run time rather than passing as a census row.
    #[allow(clippy::too_many_lines)]
    fn witness(class: C) -> K {
        use crate::{EvalError, ParamName, SlotId};
        let n = RecipeNodeId;
        match class {
            C::Expr => K::Expr {
                slot: SlotId::Distance,
                source: EvalError::NonFiniteResult,
            },
            C::Profile => K::Profile(profile::ProfileError::EmptyProfile),
            C::ProfileReplay => K::ProfileReplay {
                loop_: 0,
                error: profile::ReplayError {
                    step: 2,
                    kind: profile::ReplayErrorKind::Transition {
                        state: profile::TipState::Closed,
                        verb: None,
                    },
                },
            },
            C::ProfileLaneReplay => K::ProfileLaneReplay {
                loop_: 0,
                step: 2,
                structure: None,
            },
            C::ProfileAnchor => K::ProfileAnchor { loop_: 0 },
            C::ProfilePieces => K::ProfilePieces {
                fault: crate::PiecesFault::Length {
                    loop_: 0,
                    recorded: 4,
                    anchored: 5,
                },
            },
            C::Extrude => K::Extrude(sweep::ExtrudeError::ObliqueExtrusion),
            C::Revolve => K::Revolve(sweep::RevolveError::DegenerateAxis),
            C::Tube => K::Tube(Box::new(sweep::TubeError::DegenerateWindow)),
            C::Split => K::Split(topo::SplitError::Finish(topo::SplitFinishError::Corrupt)),
            C::Fillet | C::Chamfer => K::Blend {
                verb: if class == C::Fillet {
                    BlendKind::Fillet
                } else {
                    BlendKind::Chamfer
                },
                error: sweep::blend::BlendError::NonpositiveSize { size: 0.0 },
            },
            C::Boolean => K::Boolean(topo::BooleanError::UnrepresentableResult),
            C::Transform => K::Transform(topo::TransformError::NurbsPlaceholder),
            C::Skin => K::Skin(sweep::SkinError::TooFewSections { have: 1, need: 2 }),
            C::Loft => K::Loft(sweep::LoftError::Band(band_error())),
            C::CurvedSolidFrontier => K::CurvedSolidFrontier {
                what: "a sweep along a curved path",
            },
            C::MissingInput => K::MissingInput { input: n(3) },
            C::ToleranceConflict => K::ToleranceConflict {
                document_eps: 1.0e-7,
                process_eps: 1.0e-9,
            },
            C::ParamBox => K::ParamBox {
                source: crate::ParamBoxError::UnknownParam {
                    param: ParamName::from_static("width"),
                },
            },
            C::Seed => K::Seed {
                source: crate::SeedError::UnknownParam {
                    param: ParamName::from_static("width"),
                },
            },
            C::SeedPinnedSection => K::SeedPinnedSection {
                section: n(3),
                param: ParamName::from_static("width"),
            },
            C::WrongOperand => K::WrongOperand {
                input: n(3),
                expected: "body",
                found: "profile",
            },
            C::EmptyOperand => K::EmptyOperand { input: n(3) },
            C::EmptyHalf => K::EmptyHalf {
                input: n(3),
                half: crate::SplitHalf::Above,
            },
            C::InstanceOutOfRange => K::InstanceOutOfRange {
                input: n(3),
                index: 7,
                count: 4,
            },
            C::DegenerateDirection => K::DegenerateDirection {
                role: "extrude direction",
            },
            C::NonFiniteDirection => K::NonFiniteDirection {
                role: "extrude direction",
            },
            C::UnderflowedDirection => K::UnderflowedDirection {
                role: "extrude direction",
            },
            C::Band => K::Band(band_error()),
            C::MissingSlot => K::MissingSlot {
                slot: SlotId::Distance,
            },
            C::VerbArity => K::VerbArity {
                verb: verbs::VerbKind::Fillet,
                given: verbs::Arity::Two,
            },
            C::Escalated => K::Escalated {
                predicate: "side_of_plane",
                source: diag(),
            },
            C::AxisInDifferentPlane => K::AxisInDifferentPlane {
                axis: n(3),
                axis_plane: Some(n(1)),
                profile_plane: Some(n(2)),
            },
            C::NonPositiveCount => K::NonPositiveCount { count: 0 },
            C::PlacementsUncertified => K::PlacementsUncertified { i: 0, j: 1 },
            C::PlacementRuleCountSpelling => {
                K::PlacementRule(crate::PlacementRuleFault::CountSpelling)
            }
            C::PlacementRuleNoPlacements => {
                K::PlacementRule(crate::PlacementRuleFault::NoPlacements)
            }
            C::PlacementRuleNonFiniteFrame => {
                K::PlacementRule(crate::PlacementRuleFault::NonFiniteFrame { index: 2 })
            }
            C::PlacementRuleImproperFrame => {
                K::PlacementRule(crate::PlacementRuleFault::ImproperFrame {
                    index: 2,
                    determinant: -1.0,
                })
            }
            C::PlacementRuleNonRigidFrame => {
                K::PlacementRule(crate::PlacementRuleFault::NonRigidFrame {
                    index: 2,
                    check: "transform_rigid_col0_unit",
                })
            }
            C::UnschedulableCycle => K::UnschedulableCycle,
            C::Naming => K::Naming(crate::NamingError::Emission {
                what: "a cap face with no profile loop behind it",
            }),
            C::ParamSourceAttach => K::ParamSourceAttach(topo::ParamAttachError::StaleKey),
            C::DeclareResolve => K::DeclareResolve {
                error: resolve_error(),
            },
            C::DeclareSiteNotAnOperand => K::DeclareSiteNotAnOperand { at: n(3) },
            C::DeclareUnsupportedPair => K::DeclareUnsupportedPair {
                kinds: (EntityKind::Edge, EntityKind::Vertex),
                cross_operand: true,
            },
            C::UndeclaredContact => K::UndeclaredContact {
                finding: Box::new(crate::FlushFinding {
                    pair: (
                        crate::SitedRef {
                            at: n(2),
                            name: name(),
                        },
                        crate::SitedRef {
                            at: n(3),
                            name: name(),
                        },
                    ),
                    class: topo::ContactClass::Rest,
                    evidence: crate::FlushEvidence {
                        relation: topo::PlaneRelation::SameOpposite,
                        rung: crate::FlushRung::DecidedCoincident,
                    },
                }),
                merged: Box::new((Vec::new(), Vec::new())),
                diag: diag(),
            },
            C::UndeclarableContact => K::UndeclarableContact {
                row: Box::new(name()),
                diag: diag(),
            },
            C::FilletSelectionResolve => K::BlendSelectionResolve {
                verb: BlendKind::Fillet,
                error: resolve_error(),
            },
            C::ChamferSelectionResolve => K::BlendSelectionResolve {
                verb: BlendKind::Chamfer,
                error: resolve_error(),
            },
            C::FilletSelectionKind => found_body(|found| K::BlendSelectionKind {
                verb: BlendKind::Fillet,
                name: Box::new(name()),
                found,
            }),
            C::ChamferSelectionKind => found_body(|found| K::BlendSelectionKind {
                verb: BlendKind::Chamfer,
                name: Box::new(name()),
                found,
            }),
            C::FilletSelectionEmpty => K::BlendSelectionEmpty {
                verb: BlendKind::Fillet,
            },
            C::ChamferSelectionEmpty => K::BlendSelectionEmpty {
                verb: BlendKind::Chamfer,
            },
            C::Shell => K::Shell(Box::new(topo::ShellError::Thickness { thickness: -0.5 })),
            C::ShellOpenResolve => K::ShellOpenResolve {
                error: resolve_error(),
            },
            C::ShellOpenKind => found_body(|found| K::ShellOpenKind {
                name: Box::new(name()),
                found,
            }),
            C::ShellLaneUnsupported => K::ShellLaneUnsupported { scalar: "interval" },
            C::FaceFrameResolve => K::FaceFrameResolve {
                error: resolve_error(),
            },
            C::FaceFrameKind => found_body(|found| K::FaceFrameKind {
                name: Box::new(name()),
                found,
            }),
            C::FaceFrameNotPlanar => K::FaceFrameNotPlanar {
                carrier: geom_brep::SurfaceKind::Cylinder,
            },
            C::FaceFrameReadback => K::FaceFrameReadback {
                error: topo::readback::ReadbackError::NoCarrier,
            },
            C::DerivedFrameSection => K::DerivedFrameSection {
                profile: n(3),
                frame: n(2),
            },
            C::FrameDirectionDegenerate => frame_direction(UnitVec3Error::Degenerate),
            C::FrameDirectionNonFiniteLength => frame_direction(UnitVec3Error::NonFiniteLength),
            C::FrameDirectionUnderflowedLength => frame_direction(UnitVec3Error::UnderflowedLength),
            C::FrameDirectionEscalated => frame_direction(UnitVec3Error::Escalated(diag())),
            C::WitnessBifurcation => K::WitnessBifurcation(crate::WitnessBifurcation {
                kind: crate::BifurcationKind::FoldProximity,
                margin: crate::BranchMarginEvidence {
                    margin: 3.0e-10,
                    band_zero: 1.0e-9,
                    band_escalate: 1.0e-8,
                },
                implicated: vec![crate::Implicated::Constraint(2)],
                witness_age: crate::WitnessAge {
                    solved_under: Vec::new(),
                    at_solve: Vec::new(),
                },
            }),
            C::PartNoResolver => part(crate::PartFault::NoResolver),
            C::PartPinMismatch | C::PartEpsilonSeam | C::PartUnresolved => {
                part(crate::PartFault::Unresolved {
                    fault: match class {
                        C::PartPinMismatch => crate::ResolveFault::PinMismatch,
                        C::PartEpsilonSeam => crate::ResolveFault::EpsilonSeam,
                        _ => crate::ResolveFault::Unresolved,
                    },
                    message: "the resolver's own words".to_owned(),
                })
            }
            C::PartRootFailed => part(crate::PartFault::PartRootFailed {
                node: n(7),
                refusal: K::Extrude(sweep::ExtrudeError::DegenerateExtrusion).into(),
            }),
            C::PartRootPoisoned => part(crate::PartFault::PartRootPoisoned {
                root: n(8),
                through: n(7),
                refusal: K::Extrude(sweep::ExtrudeError::DegenerateExtrusion).into(),
            }),
            C::PartRootFailureUnrecorded => {
                part(crate::PartFault::RootFailureUnrecorded { node: n(7) })
            }
            C::PartProduct => part(crate::PartFault::PartProduct {
                kind: crate::ProductErrorKind::NoBodyRoots,
                message: "the document declares no body root".to_owned(),
            }),
            C::PartReferenceCycle => part(crate::PartFault::ReferenceCycle {
                cycle: vec![doc_ref(), doc_ref()],
            }),
            C::PartDepthExceeded => part(crate::PartFault::DepthExceeded),
            C::PartNotEntered => part(crate::PartFault::NotEntered),
            C::MatePosesOfAnotherDocument => mate(crate::MateFault::PosesOfAnotherDocument {
                expected: crate::DocumentId::derive("a"),
                found: crate::DocumentId::derive("b"),
            }),
            C::MateFrame => mate(crate::MateFault::Frame {
                mate: n(9),
                side: crate::MateSide::A,
                error: geom_core::FrameError::Degenerate {
                    input: geom_core::FrameInput::Aim,
                    indeterminate: None,
                },
            }),
            C::MateClassNotAdmitted => mate(crate::MateFault::ClassNotAdmitted { mate: n(9) }),
            C::MateTableLacks => mate(crate::MateFault::TableLacks {
                mate: n(9),
                what: "a clocking rider on a planar rest",
            }),
            C::MateIndeterminate => mate(crate::MateFault::Indeterminate {
                mate: n(9),
                diag: Box::new(diag()),
            }),
            C::MateBand => mate(crate::MateFault::Band {
                error: band_error(),
            }),
            C::MateContradictory => mate(crate::MateFault::Contradictory {
                held: n(8),
                added: n(9),
                predicate: "mate_coaxial",
                clash: crate::Clash::Length { metres: 0.002 },
            }),
            C::MateUnder => mate(crate::MateFault::Under {
                mate: n(9),
                parent: n(6),
                child: n(7),
                residual: crate::Subgroup::Se3,
            }),
            C::MateDanglingHead => mate(crate::MateFault::DanglingHead {
                mate: n(9),
                side: crate::MateSide::B,
                head: n(4),
            }),
            C::MatePlacerRefused => mate(crate::MateFault::PlacerRefused {
                mate: n(9),
                side: crate::MateSide::B,
                placer: n(4),
                error: K::EmptyOperand { input: n(3) }.into(),
                placer_row: crate::PlacerRow::Silent,
            }),
            C::MatePartSelectsAnotherCopy => mate(crate::MateFault::PartSelectsAnotherCopy {
                mate: n(9),
                side: crate::MateSide::A,
                part: n(6),
                named: 2,
                selected: 5,
            }),
            C::MateSelf => mate(crate::MateFault::SelfMate {
                mate: n(9),
                instance: n(6),
            }),
            C::MateUnleverable => mate(crate::MateFault::Unleverable {
                mate: n(9),
                refusal: Box::new(crate::LeverRefusal::Reach {
                    instance: n(6),
                    part: doc_ref(),
                    refusal: crate::ReachRefusal::NoExtent,
                }),
            }),
            C::MateFaceUnresolved => mate(crate::MateFault::FaceUnresolved {
                mate: n(9),
                side: crate::MateSide::A,
                refusal: Box::new(crate::FaceRefusal::Reach {
                    instance: n(6),
                    part: doc_ref(),
                    face: crate::FaceName::new(name()).expect("a face name"),
                    refusal: crate::FacePoseRefusal::NoSuchName,
                }),
            }),
            C::CrossingUnverified => K::CrossingUnverified {
                instance: n(6),
                outer: Box::new(crate::FaceName::new(name()).expect("a face name")),
                name: Box::new(name()),
            },
            C::MeasureRefResolve => K::MeasureRefResolve {
                error: resolve_error(),
            },
            C::MeasureRefUnreadable => K::MeasureRefUnreadable {
                name: Box::new(name()),
                error: crate::InterrogateError::NoSuchName,
            },
            C::MeasureNonFinite => K::MeasureNonFinite {
                source: EvalError::NonFiniteResult,
            },
            C::MeasureNotParallel => K::MeasureNotParallel {
                verb: "distance",
                a: "plane",
                b: "plane",
                predicate: "measure_parallel",
            },
            C::MeasureUnsupported => {
                K::MeasureUnsupported(crate::eval::measure::MeasureUnsupported {
                    verb: "distance",
                    a: "cylinder",
                    b: "torus",
                })
            }
            C::MeasureMalformed => {
                K::MeasureMalformed(crate::MeasureNodeFault::RefIndexOutOfRange {
                    verb: "min_clearance",
                    index: 2,
                    refs: 2,
                })
            }
            C::PayloadExpr => K::PayloadExpr {
                what: "placement",
                index: 2,
                source: EvalError::NonFiniteResult,
            },
            C::MeasureSelectionKind => found_body(|found| K::MeasureSelectionKind {
                verb: "min_clearance",
                found,
            }),
            C::MeasureClearanceRefused => {
                K::MeasureClearanceRefused(crate::clearance::ClearanceRefusal::Unsupported {
                    carrier: "a free-form face",
                    face: topo::FaceKey::default(),
                })
            }
            C::AssertionDimension => K::AssertionDimension {
                measured: crate::Dimension::Length,
                bound: crate::Dimension::Angle,
            },
        }
    }

    /// **Every class has an arm that projects onto it, and nothing
    /// else does.**
    ///
    /// [`K::class`] is exhaustive over the ERROR and over each value it
    /// splits on, so an arm or a value added there reds this crate.
    /// [`witness`] is exhaustive over the CLASS, so a variant added to
    /// [`C`] alone reds here at compile time, and [`ALL`] is every
    /// class by construction; this row asks each witness to project
    /// onto the class it is filed under, which is what reds a
    /// mis-projected arm or a split that reads the wrong value. Two
    /// classes cannot share a witness's projection, so the pairing is a
    /// bijection over [`ALL`].
    #[test]
    fn each_class_has_an_arm_and_each_arm_projects_to_its_own_class() {
        for &class in ALL {
            let err = witness(class);
            assert_eq!(
                err.class(),
                class,
                "the witness filed under {class:?} projects elsewhere: {err:?}"
            );
        }
    }
}
