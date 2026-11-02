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
    /// [`NodeErrorKind::WrongOperand`].
    WrongOperand,
    /// [`NodeErrorKind::EmptyOperand`].
    EmptyOperand,
    /// [`NodeErrorKind::UnfinishedOperand`].
    UnfinishedOperand,
    /// [`NodeErrorKind::EmptyHalf`].
    EmptyHalf,
    /// [`NodeErrorKind::MemberOutOfRange`].
    MemberOutOfRange,
    /// [`NodeErrorKind::MemberRefused`].
    MemberRefused,
    /// [`NodeErrorKind::IndicesUnrelated`].
    IndicesUnrelated,
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
    /// [`NodeErrorKind::UnionFoldStep`].
    UnionFoldStep,
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
            Self::WrongOperand { .. } => C::WrongOperand,
            Self::EmptyOperand { .. } => C::EmptyOperand,
            Self::UnfinishedOperand { .. } => C::UnfinishedOperand,
            Self::EmptyHalf { .. } => C::EmptyHalf,
            Self::MemberOutOfRange { .. } => C::MemberOutOfRange,
            Self::MemberRefused { .. } => C::MemberRefused,
            Self::IndicesUnrelated { .. } => C::IndicesUnrelated,
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
            Self::UnionFoldStep { .. } => C::UnionFoldStep,
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
            MateFault::Unleverable { .. } => Self::MateUnleverable,
            MateFault::KindsDiffer { .. } => Self::MateKindsDiffer,
            MateFault::SideUnevaluated { .. } => Self::MateSideUnevaluated,
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{NodeErrorClass as C, NodeErrorKind as K};
    use crate::names::{EntityKind, StableName};
    use crate::node::RecipeNodeId;
    use geom_core::{Band, BandError, Indeterminate, MarginDiag, Tol};
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
        Trace,
        Extrude,
        Revolve,
        Tube,
        Split,
        ContactCarry,
        Fillet,
        Chamfer,
        Boolean,
        Transform,
        Mirror,
        Motion,
        Skin,
        Loft,
        CurvedSolidFrontier,
        MissingInput,
        UnresolvedRead,
        SelectResolve,
        ToleranceConflict,
        ParamBox,
        Seed,
        WrongOperand,
        EmptyOperand,
        UnfinishedOperand,
        EmptyHalf,
        MemberOutOfRange,
        MemberRefused,
        IndicesUnrelated,
        IndexCount,
        IndexCountOutOfRange,
        NoSuchPiece,
        PieceShape,
        DegenerateDirection,
        NonFiniteDirection,
        UnderflowedDirection,
        Band,
        MissingSlot,
        Escalated,
        PoseRead,
        PoseDegenerate,
        PoseScalar,
        CountOutOfRange,
        UnschedulableCycle,
        Naming,
        UnionFoldStep,
        Shell,
        ShellLaneUnsupported,
        WitnessBifurcation,
        PartNoResolver,
        PartPinMismatch,
        PartEpsilonSeam,
        PartUnresolved,
        PartProductFailed,
        PartProductPoisoned,
        PartProductFailureUnrecorded,
        PartProduct,
        PartReferenceCycle,
        PartDepthExceeded,
        PartNotEntered,
        MatePosesOfAnotherDocument,
        MateOverconstrained,
        MateUnpinned,
        MateNotOnTheCopy,
        MateUnleverable,
        MateKindsDiffer,
        MateSideUnevaluated,
        MateNoChart,
        MateBackwards,
        MateUnoriented,
        SpaceMismatch,
        CrossingUnverified,
        MeasureRefUnreadable,
        MeasureNonFinite,
        MeasureNotParallel,
        MeasureUnsupported,
        PayloadExpr,
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
            node: RecipeNodeId::new(0, 3),
            path: Vec::new(),
        }
    }

    fn resolve_error() -> Box<crate::ResolveError> {
        Box::new(crate::ResolveError::NodeGone {
            name: name(),
            edit: crate::RecipeEditRef::NodeDeleted {
                node: RecipeNodeId::new(0, 3),
            },
        })
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
        use crate::{EvalError, SlotId};
        let n = |i: u64| RecipeNodeId::new(0, i);
        let c = |i: u32| crate::ConstraintId(crate::MintId::new(i, i.into()));
        let m = |i: u64| crate::MateRef::new(n(i), c(1));
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
            C::Trace => K::Trace(crate::trace::TraceRefusal {
                loop_: 0,
                step: 2,
                face: Some(0),
                fault: crate::trace::TraceStepFault::NoTrace,
            }),
            C::Extrude => K::Extrude(sweep::ExtrudeError::ObliqueExtrusion),
            C::Revolve => K::Revolve(sweep::RevolveError::DegenerateAxis),
            C::Tube => K::Tube(Box::new(sweep::TubeError::DegenerateWindow)),
            C::Split => K::Split(topo::SplitError::Finish(topo::SplitFinishError::Corrupt)),
            C::ContactCarry => K::ContactCarry {
                refusal: topo::CarryRefusal::Stale { record: 0 },
            },
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
            C::Mirror => K::Mirror(topo::TransformError::NurbsPlaceholder),
            C::Motion => K::Motion(geom_core::RigidMotionError::NonFinite(
                geom_core::MotionPart::Angle,
            )),
            C::Skin => K::Skin(sweep::SkinError::TooFewSections { have: 1, need: 2 }),
            C::Loft => K::Loft(sweep::LoftError::Band(band_error())),
            C::CurvedSolidFrontier => K::CurvedSolidFrontier {
                what: "a sweep along a curved path",
            },
            C::MissingInput => K::MissingInput { input: n(3) },
            C::UnresolvedRead => K::UnresolvedRead {
                slot: crate::OperandSlot::Target,
                var: crate::VarId::new(3, 3),
            },
            C::SelectResolve => K::SelectResolve {
                slot: crate::OperandSlot::Selection,
                var: crate::VarId::new(3, 3),
                reference: 0,
                error: resolve_error(),
            },
            C::ToleranceConflict => K::ToleranceConflict {
                document_eps: 1.0e-7,
                process_eps: 1.0e-9,
            },
            C::ParamBox => K::ParamBox {
                source: crate::ParamBoxError::UnknownParam {
                    param: crate::SpokenVar::new(crate::VarId::new(0, 1), None),
                },
            },
            C::Seed => K::Seed {
                source: crate::SeedError::UnknownVar {
                    var: crate::SpokenVar::new(crate::VarId::new(0, 1), None),
                },
            },
            C::WrongOperand => K::WrongOperand {
                input: n(3),
                expected: "body",
                found: "profile",
            },
            C::EmptyOperand => K::EmptyOperand { input: n(3) },
            C::UnfinishedOperand => K::UnfinishedOperand {
                input: n(3),
                errors: vec![topo::ValidationError::ScaffoldAtRest {
                    edge: topo::EdgeKey::default(),
                }],
            },
            C::EmptyHalf => K::EmptyHalf {
                input: n(3),
                half: crate::SplitHalf::Above,
            },
            C::MemberOutOfRange => K::MemberOutOfRange {
                input: n(3),
                at: vec![7],
            },
            C::MemberRefused => K::MemberRefused {
                at: vec![(crate::VarId::new(2, 0), 3)],
                source: Box::new(K::UnschedulableCycle),
            },
            C::IndicesUnrelated => K::IndicesUnrelated {
                a: crate::VarId::new(2, 0),
                b: crate::VarId::new(3, 0),
            },
            C::IndexCount => K::IndexCount {
                index: crate::VarId::new(2, 0),
                source: EvalError::NonFiniteResult,
            },
            C::IndexCountOutOfRange => K::IndexCountOutOfRange {
                index: crate::VarId::new(2, 0),
                count: -1,
            },
            C::NoSuchPiece => K::NoSuchPiece {
                pick: crate::VarId::new(2, 0),
                at: 0,
            },
            C::PieceShape => K::PieceShape {
                pick: crate::VarId::new(2, 0),
                family: true,
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
            C::Escalated => K::Escalated {
                predicate: "side_of_plane",
                source: diag(),
            },
            C::PoseRead => K::PoseRead {
                pose: crate::VarKind::Axis,
                fault: crate::pose::PoseReadFault::NoAxis {
                    carrier: crate::pose::Carrier::Surface(geom::SurfaceKind::Plane),
                },
            },
            C::PoseDegenerate => K::PoseDegenerate {
                construction: crate::pose::PoseConstruction::Through,
            },
            C::PoseScalar => K::PoseScalar {
                var: crate::VarId::new(2, 0),
                source: EvalError::NonFiniteResult,
            },
            C::CountOutOfRange => K::CountOutOfRange {
                slot: crate::node::SlotId::Stations,
                count: 0,
                admits: 1..=i64::MAX,
            },
            C::UnschedulableCycle => K::UnschedulableCycle,
            C::Naming => K::Naming(crate::NamingError::Emission {
                what: "a cap face with no profile loop behind it",
            }),
            C::UnionFoldStep => K::UnionFoldStep {
                member: 2,
                read: crate::VarId::new(2, 2),
                refusal: Box::new(K::UnschedulableCycle),
            },
            C::Shell => K::Shell(Box::new(topo::ShellError::Thickness { thickness: -0.5 })),
            C::ShellLaneUnsupported => K::ShellLaneUnsupported { scalar: "interval" },
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
            C::PartProductFailed => part(crate::PartFault::PartProductFailed {
                held: Default::default(),
                node: n(7),
                refusal: K::Extrude(sweep::ExtrudeError::DegenerateExtrusion).into(),
            }),
            C::PartProductPoisoned => part(crate::PartFault::PartProductPoisoned {
                held: Default::default(),
                node: n(8),
                through: n(7),
                refusal: K::Extrude(sweep::ExtrudeError::DegenerateExtrusion).into(),
            }),
            C::PartProductFailureUnrecorded => {
                part(crate::PartFault::PartProductFailureUnrecorded {
                    node: n(7),
                    held: Default::default(),
                })
            }
            C::PartProduct => part(crate::PartFault::PartProduct {
                held: Default::default(),
                refusal: crate::ProductError::EmptyProduct.into(),
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
            C::MateOverconstrained => mate(crate::MateFault::Overconstrained {
                placement: n(6),
                constraints: vec![c(1), c(2)],
                free: 1,
            }),
            C::MateUnpinned => mate(crate::MateFault::Unpinned {
                placement: n(6),
                free: 2,
            }),
            C::MateNotOnTheCopy => mate(crate::MateFault::NotOnTheCopy { mate: m(9) }),
            C::MateUnleverable => mate(crate::MateFault::Unleverable {
                mate: m(9),
                refusal: Box::new(crate::LeverRefusal::Reach {
                    instance: n(6),
                    part: doc_ref(),
                    refusal: crate::ReachRefusal::NoExtent,
                }),
            }),
            C::MateKindsDiffer => mate(crate::MateFault::KindsDiffer {
                mate: m(9),
                on: crate::VarKind::Plane,
                to: crate::VarKind::Axis,
            }),
            C::MateSideUnevaluated => mate(crate::MateFault::SideUnevaluated {
                mate: m(9),
                side: crate::MateSide::On,
                refusal: Box::new(K::EmptyOperand { input: n(3) }.into()),
            }),
            C::MateNoChart => mate(crate::MateFault::NoChart {
                value: m(9),
                why: crate::mate::NoChartWhy::SlideTurnsABall,
            }),
            C::MateBackwards => mate(crate::MateFault::Backwards {
                mate: m(9),
                orient: crate::Orientation::IDENTITY,
                write: crate::Orientation::new(crate::SignedAxis::PlusX, crate::SignedAxis::MinusZ)
                    .expect("x and z lie along two axes"),
            }),
            C::MateUnoriented => mate(crate::MateFault::Unoriented { mate: m(9) }),
            C::SpaceMismatch => K::SpaceMismatch {
                a: crate::VarId::WORLD,
                b: crate::VarId::new(0, test_utils::refusal::tagged(6)),
            },
            C::CrossingUnverified => K::CrossingUnverified {
                instance: n(6),
                outer: Box::new(crate::FaceName::new(name()).expect("a face name")),
                name: Box::new(name()),
            },
            C::MeasureRefUnreadable => K::MeasureRefUnreadable {
                slot: crate::OperandSlot::Measured(crate::MeasureVerb::Distance, 0),
                var: crate::VarId::new(3, 3),
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
            C::PayloadExpr => K::PayloadExpr {
                what: "placement",
                index: 2,
                source: EvalError::NonFiniteResult,
            },
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
