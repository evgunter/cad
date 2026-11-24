//! [`NodeErrorClass`]: which arm of [`NodeErrorKind`] refused, without
//! the payload.

use sweep::blend::BlendKind;

use super::{NodeErrorKind, PartFault};
use crate::mate::MateFault;
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
/// an instantiation's fault and a mate's fault each decide theirs. Nothing else in a payload is read.
///
/// [`NodeErrorKind::class`] matches exhaustively, and so does every
/// split inside it — an arm added to the error, or a value added to a
/// split, reds `class` itself, here in this crate.
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
    /// [`NodeErrorKind::Trace`].
    Trace,
    /// [`NodeErrorKind::Extrude`].
    Extrude,
    /// [`NodeErrorKind::Revolve`].
    Revolve,
    /// [`NodeErrorKind::Tube`].
    Tube,
    /// [`NodeErrorKind::Split`].
    Split,
    /// [`NodeErrorKind::ContactCarry`].
    ContactCarry,
    /// [`NodeErrorKind::Blend`] refused by a fillet.
    Fillet,
    /// [`NodeErrorKind::Blend`] refused by a chamfer.
    Chamfer,
    /// [`NodeErrorKind::Boolean`].
    Boolean,
    /// [`NodeErrorKind::Transform`].
    Transform,
    /// [`NodeErrorKind::Mirror`].
    Mirror,
    /// [`NodeErrorKind::Motion`].
    Motion,
    /// [`NodeErrorKind::Skin`].
    Skin,
    /// [`NodeErrorKind::Loft`].
    Loft,
    /// [`NodeErrorKind::CurvedSolidFrontier`].
    CurvedSolidFrontier,
    /// [`NodeErrorKind::MissingInput`].
    MissingInput,
    /// [`NodeErrorKind::UnresolvedRead`].
    UnresolvedRead,
    /// [`NodeErrorKind::SelectResolve`].
    SelectResolve,
    /// [`NodeErrorKind::ToleranceConflict`].
    ToleranceConflict,
    /// [`NodeErrorKind::ParamBox`].
    ParamBox,
    /// [`NodeErrorKind::Seed`].
    Seed,
    /// [`NodeErrorKind::RecordRefused`].
    RecordRefused,
    /// [`NodeErrorKind::RecordMispaired`].
    RecordMispaired,
    /// [`NodeErrorKind::WrongOperand`].
    WrongOperand,
    /// [`NodeErrorKind::EmptyOperand`].
    EmptyOperand,
    /// [`NodeErrorKind::EmptyMember`].
    EmptyMember,
    /// [`NodeErrorKind::NotFinished`].
    NotFinished,
    /// [`NodeErrorKind::UnfinishedLeaf`].
    UnfinishedLeaf,
    /// [`NodeErrorKind::EmptyHalf`].
    EmptyHalf,
    /// [`NodeErrorKind::MemberOutOfRange`].
    MemberOutOfRange,
    /// [`NodeErrorKind::MemberRefused`].
    MemberRefused,
    /// [`NodeErrorKind::TwoBinders`].
    TwoBinders,
    /// [`NodeErrorKind::YieldsNothing`].
    YieldsNothing,
    /// [`NodeErrorKind::IndexCount`].
    IndexCount,
    /// [`NodeErrorKind::IndexCountOutOfRange`].
    IndexCountOutOfRange,
    /// [`NodeErrorKind::NoSuchPiece`].
    NoSuchPiece,
    /// [`NodeErrorKind::PieceShape`].
    PieceShape,
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
    /// [`NodeErrorKind::Escalated`].
    Escalated,
    /// [`NodeErrorKind::PoseRead`].
    PoseRead,
    /// [`NodeErrorKind::PoseDegenerate`].
    PoseDegenerate,
    /// [`NodeErrorKind::PoseScalar`].
    PoseScalar,
    /// [`NodeErrorKind::CountOutOfRange`].
    CountOutOfRange,
    /// [`NodeErrorKind::UnschedulableCycle`].
    UnschedulableCycle,
    /// [`NodeErrorKind::Naming`].
    Naming,
    /// [`NodeErrorKind::FoldStep`].
    FoldStep,
    /// [`NodeErrorKind::Shell`].
    Shell,
    /// [`NodeErrorKind::ShellLaneUnsupported`].
    ShellLaneUnsupported,
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
    /// [`NodeErrorKind::Part`] carrying [`PartFault::PartProductFailed`].
    PartProductFailed,
    /// [`NodeErrorKind::Part`] carrying [`PartFault::PartProductPoisoned`].
    PartProductPoisoned,
    /// [`NodeErrorKind::Part`] carrying
    /// [`PartFault::PartProductFailureUnrecorded`].
    PartProductFailureUnrecorded,
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
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::Overconstrained`].
    MateOverconstrained,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::Unpinned`].
    MateUnpinned,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::NotOnTheCopy`].
    MateNotOnTheCopy,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::NotTheDatumsOwn`].
    MateNotTheDatumsOwn,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::Unleverable`].
    MateUnleverable,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::KindsDiffer`].
    MateKindsDiffer,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::SideUnevaluated`].
    MateSideUnevaluated,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::NoChart`].
    MateNoChart,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::Backwards`].
    MateBackwards,
    /// [`NodeErrorKind::Mate`] carrying [`MateFault::Unoriented`].
    MateUnoriented,
    /// [`NodeErrorKind::SpaceMismatch`].
    SpaceMismatch,
    /// [`NodeErrorKind::CrossingUnverified`].
    CrossingUnverified,
    /// [`NodeErrorKind::MeasureRefUnreadable`].
    MeasureRefUnreadable,
    /// [`NodeErrorKind::MeasureNonFinite`].
    MeasureNonFinite,
    /// [`NodeErrorKind::MeasureNotParallel`].
    MeasureNotParallel,
    /// [`NodeErrorKind::MeasureUnsupported`].
    MeasureUnsupported,
    /// [`NodeErrorKind::PayloadExpr`].
    PayloadExpr,
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
            Self::Trace(_) => C::Trace,
            Self::Extrude(_) => C::Extrude,
            Self::Revolve(_) => C::Revolve,
            Self::Tube(_) => C::Tube,
            Self::Split(_) => C::Split,
            Self::ContactCarry { .. } => C::ContactCarry,
            Self::Blend { verb, .. } => by_verb(*verb, C::Fillet, C::Chamfer),
            Self::Boolean(_) => C::Boolean,
            Self::Transform(_) => C::Transform,
            Self::Mirror(_) => C::Mirror,
            Self::Motion(_) => C::Motion,
            Self::Skin(_) => C::Skin,
            Self::Loft(_) => C::Loft,
            Self::CurvedSolidFrontier { .. } => C::CurvedSolidFrontier,
            Self::MissingInput { .. } => C::MissingInput,
            Self::UnresolvedRead { .. } => C::UnresolvedRead,
            Self::SelectResolve { .. } => C::SelectResolve,
            Self::ToleranceConflict { .. } => C::ToleranceConflict,
            Self::ParamBox { .. } => C::ParamBox,
            Self::Seed { .. } => C::Seed,
            Self::RecordRefused => C::RecordRefused,
            Self::RecordMispaired(_) => C::RecordMispaired,
            Self::WrongOperand { .. } => C::WrongOperand,
            Self::EmptyOperand { .. } => C::EmptyOperand,
            Self::EmptyMember { .. } => C::EmptyMember,
            Self::NotFinished { .. } => C::NotFinished,
            Self::UnfinishedLeaf { .. } => C::UnfinishedLeaf,
            Self::EmptyHalf { .. } => C::EmptyHalf,
            Self::MemberOutOfRange { .. } => C::MemberOutOfRange,
            Self::MemberRefused { .. } => C::MemberRefused,
            Self::TwoBinders { .. } => C::TwoBinders,
            Self::YieldsNothing { .. } => C::YieldsNothing,
            Self::IndexCount { .. } => C::IndexCount,
            Self::IndexCountOutOfRange { .. } => C::IndexCountOutOfRange,
            Self::NoSuchPiece { .. } => C::NoSuchPiece,
            Self::PieceShape { .. } => C::PieceShape,
            Self::DegenerateDirection { .. } => C::DegenerateDirection,
            Self::NonFiniteDirection { .. } => C::NonFiniteDirection,
            Self::UnderflowedDirection { .. } => C::UnderflowedDirection,
            Self::Band(_) => C::Band,
            Self::MissingSlot { .. } => C::MissingSlot,
            Self::Escalated { .. }
            | Self::MeasureEscalated { .. }
            | Self::PlacementAxisEscalated { .. }
            | Self::PoseDirectionEscalated { .. } => C::Escalated,
            Self::PoseRead { .. } => C::PoseRead,
            Self::PoseDegenerate { .. } => C::PoseDegenerate,
            Self::PoseScalar { .. } => C::PoseScalar,
            Self::CountOutOfRange { .. } => C::CountOutOfRange,
            Self::UnschedulableCycle => C::UnschedulableCycle,
            Self::Naming(_) => C::Naming,
            Self::FoldStep { .. } => C::FoldStep,
            Self::Shell(_) => C::Shell,
            Self::ShellLaneUnsupported { .. } => C::ShellLaneUnsupported,
            Self::WitnessBifurcation(_) => C::WitnessBifurcation,
            Self::Part { fault, .. } => match fault {
                PartFault::NoResolver => C::PartNoResolver,
                PartFault::Unresolved { fault, .. } => match fault {
                    ResolveFault::PinMismatch => C::PartPinMismatch,
                    ResolveFault::EpsilonSeam => C::PartEpsilonSeam,
                    ResolveFault::Unresolved => C::PartUnresolved,
                },
                PartFault::PartProductFailed { .. } => C::PartProductFailed,
                PartFault::PartProductPoisoned { .. } => C::PartProductPoisoned,
                PartFault::PartProductFailureUnrecorded { .. } => C::PartProductFailureUnrecorded,
                PartFault::PartProduct { .. } => C::PartProduct,
                PartFault::ReferenceCycle { .. } => C::PartReferenceCycle,
                PartFault::DepthExceeded => C::PartDepthExceeded,
                PartFault::NotEntered => C::PartNotEntered,
            },
            Self::Mate(fault) => C::of_mate(fault),
            Self::SpaceMismatch { .. } => C::SpaceMismatch,
            Self::CrossingUnverified { .. } => C::CrossingUnverified,
            Self::MeasureRefUnreadable { .. } => C::MeasureRefUnreadable,
            Self::MeasureNonFinite { .. } => C::MeasureNonFinite,
            Self::MeasureNotParallel { .. } => C::MeasureNotParallel,
            Self::MeasureUnsupported(_) => C::MeasureUnsupported,
            Self::PayloadExpr { .. } => C::PayloadExpr,
            Self::AssertionDimension { .. } => C::AssertionDimension,
        }
    }
}

impl NodeErrorClass {
    /// The class of the node refusal that carries a mate fault. Every
    /// door that publishes the fault reads it through this class, so
    /// the fault has one word wherever it is refused.
    #[must_use]
    pub fn of_mate(fault: &MateFault) -> Self {
        match fault {
            MateFault::PosesOfAnotherDocument { .. } => Self::MatePosesOfAnotherDocument,
            MateFault::Overconstrained { .. } => Self::MateOverconstrained,
            MateFault::Unpinned { .. } => Self::MateUnpinned,
            MateFault::NotOnTheCopy { .. } => Self::MateNotOnTheCopy,
            MateFault::NotTheDatumsOwn { .. } => Self::MateNotTheDatumsOwn,
            MateFault::Unleverable { .. } => Self::MateUnleverable,
            MateFault::KindsDiffer { .. } => Self::MateKindsDiffer,
            MateFault::SideUnevaluated { .. } | MateFault::ReadUnevaluated { .. } => {
                Self::MateSideUnevaluated
            }
            MateFault::NoChart { .. } => Self::MateNoChart,
            MateFault::Backwards { .. } => Self::MateBackwards,
            MateFault::Unoriented { .. } => Self::MateUnoriented,
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

/// **One witness of every class** — the census both this crate's own
/// row and a consumer's census of its answers per class walk
/// (`test_support::node_error_witnesses`).
#[cfg(any(test, feature = "test-support"))]
pub(crate) mod witnesses;
