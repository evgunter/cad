//! **The shell's correspondence** — the hollowing verb, in the same
//! per-instance-data pattern as the blends', the boolean's, the
//! split's and the sweeps'.
//!
//! # What it declares, and what stays upstairs
//!
//! `Node::Shell` has one input and one slot: a `target` whose evaluated
//! body AND name table the lowering reads, and a `thickness`. Its
//! payload is a frozen, ORDERED list of face names — the faces opened
//! into rims — resolved through the target's table exactly as a blend's
//! selection is, with the kind check demanding a FACE. What is declared
//! here is the correspondence proper: which slot is the verb's scalar
//! parameter ([`ShellSlots`]), how resolved face keys and the evaluated
//! thickness become the kernel verb (`build`), which arm of the closed
//! record channel the result arrives in (`record`), and which emitter
//! mints names from the birth record (`emitter`). The order of the
//! designation crosses to the kernel untouched: the record says the
//! first designated face of a chart carries the rim, so the order is
//! authored data the kernel reads and this module never sorts it.
//!
//! # Every field is direct per-instance data
//!
//! As in the sibling modules: function pointers and literals per
//! instance, no match over a verb vocabulary anywhere in this file, so
//! a future verb never has to open it. There is one instance, and the
//! struct is carried anyway: it is the seam where the lowering reads
//! its four literals, and reading them off a struct is what keeps the
//! lowering free of a shell-specific spelling.
//!
//! # The lane door
//!
//! The kernel door validates what it built with a CERTIFIED claim
//! (`topo::shell_open`'s last act is the certified at-rest validator),
//! so it is formed only at a scalar with certification rights — and
//! not every evaluation scalar has them: a dual does not certify (the
//! DL3 ruling), so no `Body<Dual<_>>` can be handed to the door at all.
//! [`ShellLane`] is that fact as a trait, the `MinClearanceLane` shape:
//! the certifying scalars run the seat's door, and a scalar without
//! rights answers `None`, which the lowering refuses TYPED rather than
//! building an unvalidated hollow.
//!
//! # The refusal fold
//!
//! `ShellError<T>` is the first kernel refusal reaching the document
//! layer that carries lane scalars, and `NodeErrorKind` is scalar-free
//! by construction. [`fold_shell_error`] is the TOTAL fold to the
//! `f64` witness, arm by arm through every nested generic payload —
//! no wildcard anywhere, so a new arm in any of the three kernel enums
//! is a compile error here and never a silently dropped number.

use geom_core::{Bounds, Decide, Tol};
use std::sync::Arc;
use topo::{Body, FaceKey, ReplaceFaceError, ShellError, ShellNaming};
use verbs::{ScalarParam, Verb, VerbError, VerbOut, VerbRecord};

use crate::names::{self, NameTable, NamingError};
use crate::node::{RecipeNodeId, SlotId};

/// The shell's naming emitter: this node's id, its operand's id and
/// table, the result body and its birth record, in.
pub(crate) type ShellEmitter<T> = fn(
    RecipeNodeId,
    RecipeNodeId,
    &NameTable,
    &Body<T>,
    &ShellNaming,
) -> Result<Arc<NameTable>, NamingError>;

/// **The shell's correspondence**, as data — everything the lowering
/// needs to turn a `Node::Shell` into a [`Verb`] and its result into a
/// name table. Adding a field here is how the verb declares something
/// the lowering must know; adding an arm to a match inside the
/// lowering is not.
pub(crate) struct ShellVerb<T: geom_core::Real> {
    /// **Resolved open faces + slot value → the kernel verb.** The
    /// designation order crosses verbatim (module docs).
    pub(crate) build: fn(Vec<FaceKey>, T) -> Verb<T>,
    /// This verb's naming emitter.
    pub(crate) emitter: ShellEmitter<T>,
    /// **This family's arm of the closed record channel**, as a
    /// projection — read through [`super::read_record`], which owns
    /// the foreign-family refusal. The record is not an `Option`: the
    /// doors write it as they act, so a shell result always carries
    /// one and there is no "no records" sentence to invent.
    pub(crate) record: fn(VerbRecord<T>) -> Option<ShellNaming>,
    /// The thickness slot and the kernel parameter it is.
    pub(crate) slots: ShellSlots,
    /// What a WRONG-FAMILY record is called when this verb's result
    /// arrives carrying another family's channel — a kernel bug
    /// surfaced typed, unreachable while the doors and the
    /// correspondences agree; the sentence names the door so the
    /// refusal does too.
    pub(crate) foreign_record: &'static str,
}

/// **The slot ↔ parameter join of the shell**, free of the lane
/// scalar: the document side names a slot, the kernel side names a
/// parameter, and the parameter → field flow
/// (`verbs::VerbKind::param_flow`) is keyed on the latter. The join is
/// what lets the lowering attach the slot's lowered expression identity
/// through the verb's declared flow, and the content key read the same
/// slot, without either knowing which verb it holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ShellSlots {
    /// The slot whose evaluated scalar is the wall thickness.
    pub(crate) size_slot: SlotId,
    /// Which kernel scalar parameter that slot IS.
    pub(crate) size_param: ScalarParam,
}

/// The shell's join.
pub(crate) const SHELL_SLOTS: ShellSlots = ShellSlots {
    size_slot: SlotId::ShellThickness,
    size_param: ScalarParam::ShellThickness,
};

/// The shell's kernel payload. A named function rather than a closure
/// so it can be a plain `fn` pointer in the struct above.
fn build_shell<T: geom_core::Real>(open: Vec<FaceKey>, thickness: T) -> Verb<T> {
    Verb::Shell { thickness, open }
}

/// The shell family's arm of the record channel. Exhaustive with no
/// wildcard (D3): a family added to the channel breaks this at compile
/// time and is routed here deliberately.
fn shell_record<T: geom_core::Real>(record: VerbRecord<T>) -> Option<ShellNaming> {
    match record {
        VerbRecord::Shell(naming) => Some(naming),
        VerbRecord::Blend(_)
        | VerbRecord::Boolean { .. }
        | VerbRecord::Extrude(_)
        | VerbRecord::Revolve(_)
        | VerbRecord::Split(_) => None,
    }
}

/// The shell's correspondence.
///
/// A function rather than a `const` because the struct is generic in
/// the lane scalar and a module-level const cannot be: the call is
/// monomorphized and inlined, so this costs nothing at run time.
pub(crate) fn shell<T: geom_core::Real>() -> ShellVerb<T> {
    ShellVerb {
        build: build_shell,
        emitter: names::name_shell,
        record: shell_record,
        slots: SHELL_SLOTS,
        foreign_record: "the shell returned a record that is not a shell's",
    }
}

/// **Which evaluation scalars can run the shell door**, as a trait
/// (module docs): the seat's `run_shell` is formed only at a scalar
/// with certification rights, and this is where each scalar says
/// whether it has them.
///
/// `None` is an answer and never a fallback: it says this scalar
/// cannot form the call, so the lowering refuses typed. Running the
/// door is the whole of what a certifying scalar does here — every
/// check and every refusal stays the kernel's.
pub trait ShellLane: geom_core::Real {
    /// The lane's name, for the typed refusal a scalar without a door
    /// carries.
    const LANE: &'static str;

    /// Run the hollowing verb against its operand, or `None` at a
    /// scalar that cannot certify.
    fn run_shell(
        verb: &Verb<Self>,
        operand: &Body<Self>,
        tol: Tol,
    ) -> Option<Result<VerbOut<Self>, VerbError<Self>>>;
}

impl ShellLane for f64 {
    const LANE: &'static str = "f64";

    fn run_shell(
        verb: &Verb<Self>,
        operand: &Body<Self>,
        tol: Tol,
    ) -> Option<Result<VerbOut<Self>, VerbError<Self>>> {
        Some(verb.run_shell(operand, tol))
    }
}

/// The recording scalar is `f64` with a sink attached, so it carries
/// exactly what `f64` carries — here, the door.
#[cfg(feature = "probe")]
impl ShellLane for geom_core::Probe {
    const LANE: &'static str = "Probe";

    fn run_shell(
        verb: &Verb<Self>,
        operand: &Body<Self>,
        tol: Tol,
    ) -> Option<Result<VerbOut<Self>, VerbError<Self>>> {
        Some(verb.run_shell(operand, tol))
    }
}

/// The certified interval scalar runs the door: its brackets are what
/// the validator's certified claim is made of.
#[cfg(feature = "interval")]
impl ShellLane for geom_core::Interval {
    const LANE: &'static str = "Interval";

    fn run_shell(
        verb: &Verb<Self>,
        operand: &Body<Self>,
        tol: Tol,
    ) -> Option<Result<VerbOut<Self>, VerbError<Self>>> {
        Some(verb.run_shell(operand, tol))
    }
}

/// **The symbolic tier over a certifying scalar** runs the door at
/// `Sym<T>` itself, for the reason `PropsQuadLane` gives at its `Sym`
/// impl: the tier changes how an identically-zero margin decides and
/// nothing else, so wrapping a certifying base must not demote a
/// certifying lane to a refusing one — the driver's leaf replay would
/// otherwise stop hollowing the bodies it certifies.
impl<T> ShellLane for geom_core::Sym<T>
where
    T: geom_core::CertifiedBounds,
    geom_core::Sym<T>: Decide + topo::PropsQuadLane,
{
    const LANE: &'static str = "Sym";

    fn run_shell(
        verb: &Verb<Self>,
        operand: &Body<Self>,
        tol: Tol,
    ) -> Option<Result<VerbOut<Self>, VerbError<Self>>> {
        Some(verb.run_shell(operand, tol))
    }
}

/// **A dual does not certify** (the DL3 ruling, unmoved), and the shell
/// door's last act is a certified validation of what it built, so the
/// call cannot be formed at any `Dual<T>` — the signature refuses it,
/// not a run-time arm. The whole family answers `None`, and a document
/// evaluated for sensitivities meets a typed refusal at its shell node
/// rather than an unvalidated hollow.
impl<T: geom_core::Real> ShellLane for geom_core::Dual<T>
where
    geom_core::Dual<T>: geom_core::Real,
{
    const LANE: &'static str = "Dual";

    fn run_shell(
        _verb: &Verb<Self>,
        _operand: &Body<Self>,
        _tol: Tol,
    ) -> Option<Result<VerbOut<Self>, VerbError<Self>>> {
        None
    }
}

/// **The total fold of a shell refusal to its `f64` witness** (module
/// docs). Every lane scalar in a payload becomes the infimum of the
/// bracket it carries — the value itself at `f64`, and at a bracket
/// scalar the number the kernel's own gates meter (the blends'
/// `NonpositiveSize` reports `size.lo()` the same way). Every arm of
/// every nested enum is written out, so the fold can neither drop an
/// arm nor a number without failing to compile.
pub(crate) fn fold_shell_error<T: Bounds>(error: ShellError<T>) -> ShellError<f64> {
    use ShellError as E;
    match error {
        E::Band { error } => E::Band { error },
        E::Thickness { thickness } => E::Thickness {
            thickness: thickness.lo(),
        },
        E::NotOneSolid { solids } => E::NotOneSolid { solids },
        E::OperandAlreadyHollow { shells } => E::OperandAlreadyHollow { shells },
        E::WallClearance {
            face,
            other,
            gap,
            needed,
        } => E::WallClearance {
            face,
            other,
            gap: gap.lo(),
            needed: needed.lo(),
        },
        E::ChartSenseMixed { face, other } => E::ChartSenseMixed { face, other },
        E::Face { face, error } => E::Face {
            face,
            error: Box::new(fold_replace_face_error(*error)),
        },
        E::OpenFaceStale { face } => E::OpenFaceStale { face },
        E::OpenFaceRepeated { face } => E::OpenFaceRepeated { face },
        E::OpenFacesExhaustShell { shell } => E::OpenFacesExhaustShell { shell },
        E::OpenFacesDisconnect { shell, components } => {
            E::OpenFacesDisconnect { shell, components }
        }
        E::OpenFaceRingUnsupported { face, kind } => E::OpenFaceRingUnsupported { face, kind },
        E::OpenFaceChartPartial { face, other } => E::OpenFaceChartPartial { face, other },
        E::Lift { face, error } => E::Lift {
            face,
            error: Box::new(fold_replace_face_error(*error)),
        },
        E::Insert { error } => E::Insert { error },
        E::OpenFaceRimNotExpressible { face, what } => E::OpenFaceRimNotExpressible { face, what },
        E::Rim { face, error } => E::Rim { face, error },
        E::Escalated { source } => E::Escalated { source },
        E::Corrupt { key } => E::Corrupt { key },
        E::NotValid { errors } => E::NotValid { errors },
    }
}

/// The face-replacement door's refusal, folded arm by arm.
fn fold_replace_face_error<T: Bounds>(error: ReplaceFaceError<T>) -> ReplaceFaceError<f64> {
    use ReplaceFaceError as R;
    match error {
        R::StaleFace { face } => R::StaleFace { face },
        R::Corrupt => R::Corrupt,
        R::Offset { face, error } => R::Offset {
            face,
            error: fold_offset_error(error),
        },
        R::Fit { face, error } => R::Fit { face, error },
        R::ApproxLaneUnsupported { face } => R::ApproxLaneUnsupported { face },
        R::SharedSurfaceKey { face, other } => R::SharedSurfaceKey { face, other },
        R::EmptyGroup => R::EmptyGroup,
        R::GroupChartsDiffer { face, other } => R::GroupChartsDiffer { face, other },
        R::PlaceholderSurface { face } => R::PlaceholderSurface { face },
        R::ApexWindow {
            face,
            v_min,
            v_max,
            shift,
        } => R::ApexWindow {
            face,
            v_min: v_min.lo(),
            v_max: v_max.lo(),
            shift: shift.lo(),
        },
        R::ApexWindowUnknown { face } => R::ApexWindowUnknown { face },
        R::NeighborPairUnroutable {
            edge,
            kind,
            other_kind,
        } => R::NeighborPairUnroutable {
            edge,
            kind,
            other_kind,
        },
        R::FittedBoundaryUnsupported { edge, what } => R::FittedBoundaryUnsupported { edge, what },
        R::CarrierLaneUnsupported { edge, what } => R::CarrierLaneUnsupported { edge, what },
        R::IsoRow { edge, error } => R::IsoRow {
            edge,
            error: fold_iso_row_error(error),
        },
        R::Structure { edge, error } => R::Structure { edge, error },
        R::VertexDisagreement { vertex, gap } => R::VertexDisagreement {
            vertex,
            gap: gap.lo(),
        },
        R::ReanchorOffCarrier { edge, gap } => R::ReanchorOffCarrier {
            edge,
            gap: gap.lo(),
        },
        R::TogetherNonPlanar { face, kind } => R::TogetherNonPlanar { face, kind },
        R::TogetherPartialSet { face } => R::TogetherPartialSet { face },
        R::TogetherCorner {
            vertex,
            planes,
            what,
        } => R::TogetherCorner {
            vertex,
            planes,
            what,
        },
        R::TogetherChartMixed { face, other } => R::TogetherChartMixed { face, other },
        R::TogetherFaceRepeated { face } => R::TogetherFaceRepeated { face },
        R::TogetherEdgeDisagreement { edge, gap } => R::TogetherEdgeDisagreement {
            edge,
            gap: gap.lo(),
        },
        R::TogetherAxialUnsupported { face, kind } => R::TogetherAxialUnsupported { face, kind },
        R::TogetherNotAxial { face, what } => R::TogetherNotAxial { face, what },
        R::TogetherAxialCorner {
            vertex,
            surfaces,
            what,
        } => R::TogetherAxialCorner {
            vertex,
            surfaces,
            what,
        },
        R::TogetherAxialEdge { edge, what } => R::TogetherAxialEdge { edge, what },
        R::Escalated { source } => R::Escalated { source },
        R::Op { edge, error } => R::Op { edge, error },
        R::Pcurve { source } => R::Pcurve { source },
        R::ResultNotClosed { errors } => R::ResultNotClosed { errors },
    }
}

/// The analytic offset mint's refusal, folded arm by arm.
fn fold_offset_error<T: Bounds>(error: geom_brep::OffsetError<T>) -> geom_brep::OffsetError<f64> {
    use geom_brep::OffsetError as O;
    match error {
        O::RadiusFloor { kind, realized } => O::RadiusFloor {
            kind,
            realized: realized.lo(),
        },
        O::TorusRing { realized_minor } => O::TorusRing {
            realized_minor: realized_minor.lo(),
        },
        O::NotClosedUnderOffset => O::NotClosedUnderOffset,
        O::ApproxNesting => O::ApproxNesting,
        O::Escalated { source } => O::Escalated { source },
    }
}

/// The boundary-row extraction's refusal, folded arm by arm.
fn fold_iso_row_error<T: Bounds>(error: geom_brep::IsoRowError<T>) -> geom_brep::IsoRowError<f64> {
    use geom_brep::IsoRowError as I;
    match error {
        I::Interior { u, domain } => I::Interior { u: u.lo(), domain },
        I::Structure { source } => I::Structure { source },
        I::Escalated { source } => I::Escalated { source },
        I::WeightsNotSeparable { control_counts } => I::WeightsNotSeparable { control_counts },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use verbs::VerbKind;

    /// The correspondence builds the verb its name claims, in the
    /// order it was handed, and its slot join names the shell's own
    /// scalar parameter.
    #[test]
    fn the_correspondence_builds_the_shell_in_designation_order() {
        let v: Verb<f64> = (shell::<f64>().build)(Vec::new(), 1.0);
        assert_eq!(v.kind(), VerbKind::Shell, "the shell built a {v:?}");
        assert_eq!(
            shell::<f64>().slots.size_param.verb(),
            VerbKind::Shell,
            "the shell's correspondence names another verb's scalar parameter"
        );
        assert_eq!(shell::<f64>().slots.size_slot, SlotId::ShellThickness);
    }

    /// The fold keeps every number: at `f64` the witness IS the value,
    /// so the folded refusal displays exactly as the kernel's.
    #[test]
    fn the_fold_at_f64_is_the_identity_on_display() {
        let e: ShellError<f64> = ShellError::Thickness { thickness: -0.25 };
        let text = e.to_string();
        assert_eq!(fold_shell_error(e).to_string(), text);
    }
}
