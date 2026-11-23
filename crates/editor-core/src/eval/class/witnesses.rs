//! **One witness of every [`super::NodeErrorClass`]**: the census both
//! this crate's own row and a consumer's census of its answers per class
//! walk (`test_support::node_error_witnesses`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

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
    RecordRefused,
    RecordMispaired,
    WrongOperand,
    EmptyOperand,
    EmptyMember,
    NotFinished,
    UnfinishedLeaf,
    EmptyHalf,
    MemberOutOfRange,
    MemberRefused,
    TwoBinders,
    YieldsNothing,
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
    FoldStep,
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
    MateNotTheDatumsOwn,
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

fn band(tol: Tol) -> Band {
    Band::linear(tol).expect("the witness band")
}

fn band_error() -> BandError {
    BandError::Empty {
        zero: 1.0e-6,
        escalate: 1.0e-9,
    }
}

fn diag(tol: Tol) -> Indeterminate {
    Indeterminate {
        margin: MarginDiag::value(3.0e-10),
        band: band(tol),
        // UNDECIDED-NAME: the in-band cause diag() hands each class row to render
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
fn witness(class: C, tol: Tol) -> K {
    use crate::{EvalError, SlotId};
    let n = |i: u64| RecipeNodeId::new(0, i);
    let c = |i: u32| crate::ConstraintId(crate::MintId::new(i, i.into()));
    let m = |i: u64| crate::MateRef::new(n(i), c(1));
    match class {
        C::Expr => K::Expr {
            slot: SlotId::Distance,
            source: EvalError::NonFiniteResult,
        },
        C::Profile => K::Profile(profile::ProfileError::PlaneNotFinite),
        C::ProfileReplay => K::ProfileReplay {
            loop_: 0,
            error: profile::ReplayError::new(
                2,
                profile::ReplayErrorKind::Transition {
                    state: profile::TipState::Closed,
                    verb: None,
                },
            ),
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
        C::Split => K::Split(topo::SplitError::Finish(
            topo::SplitFinishError::EmptyOperand,
        )),
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
            edge: None,
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
        C::RecordRefused => K::RecordRefused,
        C::RecordMispaired => K::RecordMispaired(crate::RecordMispairing::Document {
            expected: crate::ident::DocumentId(1),
            found: crate::ident::DocumentId(2),
        }),
        C::WrongOperand => K::WrongOperand {
            input: n(3),
            expected: "body",
            found: "profile",
        },
        C::EmptyOperand => K::EmptyOperand { input: n(3) },
        C::EmptyMember => K::EmptyMember {
            input: n(3),
            at: vec![(crate::VarId::new(2, 0), 1)],
        },
        C::NotFinished => K::NotFinished {
            findings: topo::NotFinished::for_tests(vec![topo::ValidationError::ScaffoldAtRest {
                edge: topo::EdgeKey::default(),
            }]),
        },
        C::UnfinishedLeaf => K::UnfinishedLeaf {
            input: n(3),
            into: None,
            findings: topo::NotFinished::for_tests(vec![topo::ValidationError::ScaffoldAtRest {
                edge: topo::EdgeKey::default(),
            }]),
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
        C::TwoBinders => K::TwoBinders {
            a: crate::VarId::new(2, 0),
            b: crate::VarId::new(3, 0),
        },
        C::YieldsNothing => K::YieldsNothing {
            binder: crate::VarId::new(2, 0),
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
        C::Escalated => K::Escalated { source: diag(tol) },
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
        C::FoldStep => K::FoldStep {
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
        C::PartProductFailureUnrecorded => part(crate::PartFault::PartProductFailureUnrecorded {
            node: n(7),
            held: Default::default(),
        }),
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
        C::MateNotTheDatumsOwn => mate(crate::MateFault::NotTheDatumsOwn { mate: m(9) }),
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
            b: crate::VarId::new(0, 6),
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
        C::MeasureUnsupported => K::MeasureUnsupported(crate::eval::measure::MeasureUnsupported {
            verb: "distance",
            a: "cylinder",
            b: "torus",
        }),
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

/// Every class, each with its witness.
pub(crate) fn all(tol: Tol) -> Vec<(C, K)> {
    ALL.iter()
        .map(|&class| (class, witness(class, tol)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{ALL, witness};
    use geom_core::Tol;

    /// **Every class has an arm that projects onto it, and nothing
    /// else does.**
    ///
    /// `NodeErrorKind::class` is exhaustive over the ERROR and over each value it
    /// splits on, so an arm or a value added there reds this crate.
    /// [`witness`] is exhaustive over the CLASS, so a variant added to
    /// the class alone reds here at compile time, and [`ALL`] is every
    /// class by construction; this row asks each witness to project
    /// onto the class it is filed under, which is what reds a
    /// mis-projected arm or a split that reads the wrong value. Two
    /// classes cannot share a witness's projection, so the pairing is a
    /// bijection over [`ALL`].
    #[test]
    fn each_class_has_an_arm_and_each_arm_projects_to_its_own_class() {
        for &class in ALL {
            let err = witness(class, Tol::witness());
            assert_eq!(
                err.class(),
                class,
                "the witness filed under {class:?} projects elsewhere: {err:?}"
            );
        }
    }
}
