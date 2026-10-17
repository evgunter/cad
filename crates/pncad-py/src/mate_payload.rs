//! The `MateFault` payload, flattened once for the value that carries
//! it.
//!
//! `crate::tags::mate_fault_tag` answers WHICH mate refusal fired.
//! This module answers what the arm CARRIES, as a record whose every
//! field is present on every arm, `None` where the arm does not carry
//! one — so `getattr` never raises on the Python side and a caller
//! reads a payload without first branching on `variant`.
//!
//! # Why it lives outside `py`
//!
//! The match is a DRIFT ALARM: it is exhaustive with no wildcard, so
//! an arm added kernel-side is a compile error here rather than a
//! silently all-`None` payload nobody chose. `crate::py` compiles
//! only under the `python` feature, so a projection sited there is an
//! alarm that does not ring on the row that runs everywhere. Sited
//! here it rings on every build, and `src/tests.rs` can construct the
//! arms and read every field of each. It is the `crate::edit_payload`
//! shape, for the same reasons.
//!
//! One record rather than one match per attribute: every field of
//! [`MateFaultPayload`] is read off `MateFault`'s arms, so a
//! per-accessor match would name the same arms once per field and an
//! arm added kernel-side would owe an edit per field.
//!
//! A mate is a constraint of a placement, never a node, so every field
//! that names one holds its address, a [`MateRef`]: the placement and
//! the constraint's id.
//!
//! # The flattening
//!
//! One attribute per CONCEPT. [`MateFaultPayload::error`] is the
//! evaluation's own refusal for a pose that did not evaluate, as the
//! tag every node failure crosses with.
//!
//! [`MateFaultPayload::predicate`] is one concept and therefore one
//! attribute: the predicate that DECIDED. A contradictory pair names
//! the predicate that decided against it and an escalated one names
//! the predicate that could not decide at all — the same question
//! ("which predicate was this about"), asked of two outcomes, and the
//! arm the caller already has says which outcome it is holding.
//!
//! A NESTED REFUSAL crosses as its own word rather than being
//! flattened into this record's other attributes:
//! [`MateFaultPayload::inner_variant`] is the lever refusal's word on
//! `Unleverable` and the world read's on `WorldRead`. `Indeterminate` carries a STRUCT rather than
//! an enum, so it has no inner word at all and its shape is which
//! margin attribute is set.
//!
//! # The classifier's words are the frame door's words
//!
//! `margin` / `margin_low` / `margin_high`, `zero` / `escalate` and
//! `predicate` are `py::place::frame_err`'s vocabulary, spelled here
//! rather than re-spelled: the escalation a mate reports and the one
//! a frame constructor reports are the same value, and a caller that
//! learned the words at one door reads them at the other. The fork
//! itself is [`crate::escalation`], which both doors call.

use pncad::document::{
    Clash, ConstraintId, Coord, DocumentId, Lever, LeverRefusal, MateFault, MateRef, MateSide,
    NoChartWhy, ReachRefusal, RecipeNodeId, Subgroup, VarKind,
};
use pncad::geom_core::Indeterminate;

use crate::escalation::escalation;
use crate::tags::{lever_refusal_tag, node_error_tag, world_read_fault_tag};

/// What one [`MateFault`] arm carries, every field present.
///
/// Every field is a plain kernel value: the Python wrappers are built
/// at the accessor, so this record is what the no-interpreter build
/// tests. Borrowed from the fault for the one field that is not
/// `Copy` — the constraints a bundle's fault names.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MateFaultPayload<'a> {
    /// The mate the fault is ABOUT, by its address.
    pub mate: Option<MateRef>,
    /// Which pose of the mate refused.
    pub side: Option<MateSide>,
    /// **The evaluation's own refusal for a pose that did not
    /// evaluate**, as the tag every node failure crosses with
    /// ([`crate::tags::node_error_tag`]).
    pub error: Option<&'static str>,
    /// The instance whose part a lever's reach was asked of.
    pub instance: Option<RecipeNodeId>,
    /// The node whose copied body a lever's reach was asked of.
    pub shape: Option<RecipeNodeId>,
    /// The placement a bundle's fault is about: the one a constraint
    /// over-constrains, or whose constraints target different spaces.
    pub placement: Option<RecipeNodeId>,
    /// The constraints of [`Self::placement`] the fault names: each
    /// whose removal leaves the bundle's codimension unchanged, or the
    /// bundle's constraints whose targets differ.
    pub constraints: Option<&'a [ConstraintId]>,
    /// What the rest of an over-constrained bundle already holds the
    /// copy to.
    pub residual: Option<Subgroup>,
    /// The mate already folded, for a contradictory pair.
    pub held: Option<MateRef>,
    /// The mate whose intersection died against it.
    pub added: Option<MateRef>,
    /// **The predicate the refusal is about**: the one that decided
    /// against a contradictory pair, or the one an escalation could
    /// not decide at all. One concept, one attribute — the arm says
    /// which outcome it names.
    pub predicate: Option<&'static str>,
    /// The measured clash, in metres: a length verbatim, a lever's
    /// product, and `None` for the structural refusal, which measures
    /// nothing.
    pub clash: Option<f64>,
    /// What a lever's unbounded face is, in its own words.
    pub what: Option<&'static str>,
    /// The document whose placement was asked for.
    pub expected_document: Option<DocumentId>,
    /// The document the solve is of.
    pub found_document: Option<DocumentId>,
    /// **The nested refusal's own word**: the lever refusal's, or the
    /// world read's. An arm whose payload is a
    /// struct rather than an enum has none.
    pub inner_variant: Option<&'static str>,
    /// The in-band margin, in metres, when the classifier saw a
    /// value.
    pub margin: Option<f64>,
    /// The classified enclosure's lower bound, when it saw an
    /// enclosure rather than a value.
    pub margin_low: Option<f64>,
    /// Its upper bound.
    pub margin_high: Option<f64>,
    /// The coincidence threshold of the band a margin was classified
    /// against.
    pub zero: Option<f64>,
    /// Its escalation threshold.
    pub escalate: Option<f64>,
    /// The lever's RESIDUAL, a pure number — a sine, a cosine, a
    /// Frobenius departure from the identity, a reachability defect,
    /// named by `predicate` — when a contradictory clash levered one
    /// (`Lever::Residual`).
    pub lever_residual: Option<f64>,
    /// Its ARM, in metres: an upper bound on the two mated bodies'
    /// extent together from the datum, and NOT a contact feature, so
    /// it names the bodies' scale and nothing else in the model. The
    /// set half times `lever_arm` is the `clash` beside it.
    pub lever_arm: Option<f64>,
    /// The kind of pose a mate's `on` reads, where the two kinds
    /// differ.
    pub on_kind: Option<VarKind>,
    /// The kind of pose its `to` reads.
    pub to_kind: Option<VarKind>,
    /// **The value a value's fault is about**, by its address.
    pub value_ref: Option<MateRef>,
    /// The constraint id a value names that is no mate of its bundle.
    pub named: Option<ConstraintId>,
    /// The freedom a value sets.
    pub coord: Option<Coord>,
    /// The kind of the mate whose kind alone fixes the freedom a value
    /// sets.
    pub mate_kind: Option<VarKind>,
    /// Why a value's freedom has no chart.
    pub why: Option<NoChartWhy>,
}

impl<'a> MateFaultPayload<'a> {
    /// Which attributes this payload CARRIES, in the order the value
    /// publishes them.
    ///
    /// The destructuring is exhaustive with no `..`, so a field added
    /// to the record and not answered here fails to compile — the
    /// same alarm the match over [`MateFault`] is, one level in.
    pub fn presence(&self) -> [(&'static str, bool); 30] {
        let Self {
            mate,
            side,
            error,
            instance,
            shape,
            placement,
            constraints,
            residual,
            held,
            added,
            predicate,
            clash,
            what,
            expected_document,
            found_document,
            inner_variant,
            margin,
            margin_low,
            margin_high,
            zero,
            escalate,
            lever_residual,
            lever_arm,
            on_kind,
            to_kind,
            value_ref,
            named,
            coord,
            mate_kind,
            why,
        } = self;
        [
            ("mate", mate.is_some()),
            ("side", side.is_some()),
            ("error", error.is_some()),
            ("instance", instance.is_some()),
            ("shape", shape.is_some()),
            ("placement", placement.is_some()),
            ("constraints", constraints.is_some()),
            ("residual", residual.is_some()),
            ("held", held.is_some()),
            ("added", added.is_some()),
            ("predicate", predicate.is_some()),
            ("clash", clash.is_some()),
            ("what", what.is_some()),
            ("expected_document", expected_document.is_some()),
            ("found_document", found_document.is_some()),
            ("inner_variant", inner_variant.is_some()),
            ("margin", margin.is_some()),
            ("margin_low", margin_low.is_some()),
            ("margin_high", margin_high.is_some()),
            ("zero", zero.is_some()),
            ("escalate", escalate.is_some()),
            ("lever_residual", lever_residual.is_some()),
            ("lever_arm", lever_arm.is_some()),
            ("on_kind", on_kind.is_some()),
            ("to_kind", to_kind.is_some()),
            ("value_ref", value_ref.is_some()),
            ("named", named.is_some()),
            ("coord", coord.is_some()),
            ("mate_kind", mate_kind.is_some()),
            ("why", why.is_some()),
        ]
    }

    /// The attributes this payload carries, publication order — what
    /// a caller reading this arm finds set rather than `None`.
    pub fn present(&self) -> Vec<&'static str> {
        self.presence()
            .into_iter()
            .filter_map(|(name, set)| set.then_some(name))
            .collect()
    }

    /// The all-`None` record every arm starts from.
    pub const NONE: Self = Self {
        mate: None,
        side: None,
        error: None,
        instance: None,
        shape: None,
        placement: None,
        constraints: None,
        residual: None,
        held: None,
        added: None,
        predicate: None,
        clash: None,
        what: None,
        expected_document: None,
        found_document: None,
        inner_variant: None,
        margin: None,
        margin_low: None,
        margin_high: None,
        zero: None,
        escalate: None,
        lever_residual: None,
        lever_arm: None,
        on_kind: None,
        to_kind: None,
        value_ref: None,
        named: None,
        coord: None,
        mate_kind: None,
        why: None,
    };
}

/// The classifier's escalation, on the fields the frame door
/// publishes it under — one fork, in [`crate::escalation`], called by
/// both doors.
fn with_escalation<'a>(base: MateFaultPayload<'a>, diag: &Indeterminate) -> MateFaultPayload<'a> {
    let seen = escalation(diag);
    MateFaultPayload {
        margin: seen.margin,
        margin_low: seen.margin_low,
        margin_high: seen.margin_high,
        zero: Some(seen.zero),
        escalate: Some(seen.escalate),
        predicate: seen.predicate,
        ..base
    }
}

/// The word a reach refusal's unbounded face names its kind by.
fn unbounded(refusal: &ReachRefusal) -> Option<&'static str> {
    match refusal {
        ReachRefusal::FaceUnbounded { kind, .. } => Some(kind.name()),
        ReachRefusal::PartUnresolved { .. }
        | ReachRefusal::MalformedBody { .. }
        | ReachRefusal::NoExtent
        | ReachRefusal::NoFiniteBound => None,
    }
}

/// Flatten one mate refusal into its payload record.
///
/// The match is EXHAUSTIVE with no wildcard arm: an arm added to
/// [`MateFault`] fails this build rather than reaching Python with an
/// all-`None` payload nobody chose for it. An arm that names a mate
/// answers `mate`; an arm that does not answers `None` BY NAME.
pub fn mate_payload(fault: &MateFault) -> MateFaultPayload<'_> {
    let none = MateFaultPayload::NONE;
    match fault {
        // The arm whose subject is not a mate at all: two documents.
        MateFault::PosesOfAnotherDocument { expected, found } => MateFaultPayload {
            expected_document: Some(*expected),
            found_document: Some(*found),
            ..none
        },
        MateFault::ClassNotAdmitted { mate } | MateFault::NotOnTheCopy { mate } => {
            MateFaultPayload {
                mate: Some(*mate),
                ..none
            }
        }
        // A struct, not an enum: the escalation has no inner WORD,
        // and its shape is which margin attribute is set.
        MateFault::Indeterminate { mate, diag } => with_escalation(
            MateFaultPayload {
                mate: Some(*mate),
                ..none
            },
            diag,
        ),
        // No lever could be formed. A reach not in hand names the
        // instance or the copied shape it is about beside the refusal's
        // word, and a face that cannot be bounded names its kind in
        // `what`; an out-of-range lever is the pair's, and names
        // neither.
        MateFault::Unleverable { mate, refusal } => {
            let (instance, shape, what) = match refusal.as_ref() {
                LeverRefusal::Reach {
                    instance, refusal, ..
                } => (Some(*instance), None, unbounded(refusal)),
                LeverRefusal::Shape { shape, refusal } => (None, Some(*shape), unbounded(refusal)),
                LeverRefusal::OutOfRange { .. } | LeverRefusal::BelowZeroBand { .. } => {
                    (None, None, None)
                }
            };
            MateFaultPayload {
                mate: Some(*mate),
                inner_variant: Some(lever_refusal_tag(refusal)),
                instance,
                shape,
                what,
                ..none
            }
        }
        // The lever is the mated bodies' own extent rather than
        // anything in the model, and `clash` is the PRODUCT of its
        // two halves. A length measured outright, or the structural
        // refusal, carries neither.
        MateFault::PoseOutOfRange { held, added } => MateFaultPayload {
            held: Some(*held),
            added: Some(*added),
            ..none
        },
        MateFault::Contradictory {
            held,
            added,
            predicate,
            clash,
        } => {
            let (lever_residual, lever_arm) = match clash {
                Clash::Levered(Lever::Residual { value, arm }) => (Some(*value), Some(*arm)),
                Clash::Structural | Clash::Length { .. } => (None, None),
            };
            MateFaultPayload {
                held: Some(*held),
                added: Some(*added),
                predicate: Some(predicate),
                clash: clash.deviation(),
                lever_residual,
                lever_arm,
                ..none
            }
        }
        // A bundle's faults: the placement is the subject, and the
        // constraints it names ride beside it, each by its id.
        MateFault::Overconstrained {
            placement,
            constraints,
            held,
        } => MateFaultPayload {
            placement: Some(*placement),
            constraints: Some(constraints),
            residual: Some(*held),
            ..none
        },
        MateFault::TargetsDiffer {
            placement,
            constraints,
        } => MateFaultPayload {
            placement: Some(*placement),
            constraints: Some(constraints),
            ..none
        },
        MateFault::KindsDiffer { mate, on, to } => MateFaultPayload {
            mate: Some(*mate),
            on_kind: Some(*on),
            to_kind: Some(*to),
            ..none
        },
        // The world read's own refusal crosses as its word.
        MateFault::WorldRead { mate, fault } => MateFaultPayload {
            mate: Some(*mate),
            inner_variant: Some(world_read_fault_tag(fault)),
            ..none
        },
        // A value's faults: the value is the subject, by its address.
        MateFault::ValueNamesNoMate { value, mate } => MateFaultPayload {
            value_ref: Some(*value),
            named: Some(*mate),
            ..none
        },
        MateFault::NotAFreedom { value, coord, kind } => MateFaultPayload {
            value_ref: Some(*value),
            coord: Some(*coord),
            mate_kind: *kind,
            ..none
        },
        MateFault::NoChart { value, why } => MateFaultPayload {
            value_ref: Some(*value),
            why: Some(*why),
            ..none
        },
        // The pose's own refusal crosses as its class's word, as every
        // node failure does.
        MateFault::SideUnevaluated {
            mate,
            side,
            refusal,
        } => MateFaultPayload {
            mate: Some(*mate),
            side: Some(*side),
            error: Some(node_error_tag(refusal.kind().class())),
            ..none
        },
    }
}
