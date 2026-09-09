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
//! One record rather than one match per attribute: `MateFault` has
//! thirty-one attributes over thirteen arms, so a per-accessor match
//! would name the same thirteen arms thirty-one times and an arm
//! added kernel-side would owe thirty-one edits.
//!
//! # The flattening
//!
//! One attribute per CONCEPT. [`MateFaultPayload::error`] is the
//! PLACER's evaluation refusal, as the tag every node failure crosses
//! with; the kernel's `Frame` and `Band` arms name their own inner
//! refusals `error` too, and those are different types that do not
//! share this attribute — their `inner_variant` is the word they
//! cross under and their prose is in `str(fault)`.
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
//! [`MateFaultPayload::inner_variant`] is the frame ladder's tag on
//! `Frame`, the band constructor's on `Band` and the lever refusal's
//! on `Unleverable`. `Indeterminate` carries a STRUCT rather than an
//! enum, so it has no inner word at all and its shape is which margin
//! attribute is set.
//!
//! # The classifier's words are the frame door's words
//!
//! `margin` / `margin_low` / `margin_high`, `zero` / `escalate` and
//! `predicate` are `py::place::frame_err`'s vocabulary, spelled here
//! rather than re-spelled: the escalation a mate reports and the one
//! a frame constructor reports are the same value, and a caller that
//! learned the words at one door reads them at the other. The fork
//! itself is [`crate::escalation`], which both doors call.
//!
//! Two arms feed those attributes and neither is a second concept: a
//! mate's own escalation (`Indeterminate`), and the escalation inside
//! the frame refusal a mate's datum wrapped
//! (`Frame`'s `FrameError::Degenerate`). `field` / `value` are the
//! band arm's, at the same door, reached through `Frame`'s
//! `FrameError::Band` as well as through `Band` itself.

use pncad::document::{DocumentId, LeverRefusal, MateFault, MateSide, RecipeNodeId, Subgroup};
use pncad::geom_core::{BandError, FrameError, Indeterminate};

use crate::escalation::escalation;
use crate::tags::{
    band_error_tag, band_field_tag, frame_error_tag, lever_refusal_tag, node_error_tag,
};

/// What one [`MateFault`] arm carries, every field present.
///
/// Every field is a plain kernel value: the Python wrappers are built
/// at the accessor, so this record is what the no-interpreter build
/// tests.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MateFaultPayload {
    /// The mate the fault is ABOUT.
    pub mate: Option<RecipeNodeId>,
    /// Which side of the mate refused.
    pub side: Option<MateSide>,
    /// The instantiate node a dangling reference head claims.
    pub head: Option<RecipeNodeId>,
    /// The placer whose pose could not be derived.
    pub placer: Option<RecipeNodeId>,
    /// **The evaluation's own refusal for that placer**, as the tag
    /// every node failure crosses with
    /// ([`crate::tags::node_error_tag`]).
    pub error: Option<&'static str>,
    /// The instance a self-mate names twice.
    pub instance: Option<RecipeNodeId>,
    /// The instance an under-determined tree mate extended FROM.
    pub parent: Option<RecipeNodeId>,
    /// The instance it failed to place.
    pub child: Option<RecipeNodeId>,
    /// What survived an under-determined fold.
    pub residual: Option<Subgroup>,
    /// The mate already folded, for a contradictory pair.
    pub held: Option<RecipeNodeId>,
    /// The mate whose intersection died against it.
    pub added: Option<RecipeNodeId>,
    /// **The predicate the refusal is about**: the one that decided
    /// against a contradictory pair, or the one an escalation could
    /// not decide at all. One concept, one attribute — the arm says
    /// which outcome it names.
    pub predicate: Option<&'static str>,
    /// The measured clash, in metres.
    pub clash: Option<f64>,
    /// The `Part` node whose index expression disagrees with the copy
    /// the reference's name names.
    pub part: Option<RecipeNodeId>,
    /// The copy that reference's NAME names.
    pub named: Option<u32>,
    /// What the `Part`'s index expression evaluates to instead.
    pub selected: Option<i64>,
    /// What the coset table was asked for, in its own words.
    pub what: Option<&'static str>,
    /// The document whose placement was asked for.
    pub expected_document: Option<DocumentId>,
    /// The document the solve is of.
    pub found_document: Option<DocumentId>,
    /// **The nested refusal's own word**: the frame ladder's tag, the
    /// band constructor's, or the lever refusal's. An arm whose
    /// payload is a struct rather than an enum has none.
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
    /// against, or of the band a constructor could not form.
    pub zero: Option<f64>,
    /// Its escalation threshold.
    pub escalate: Option<f64>,
    /// WHICH band threshold a rejected value was
    /// ([`crate::tags::band_field_tag`]).
    pub field: Option<&'static str>,
    /// The rejected number: a threshold, or a lever arm handed to the
    /// band constructor.
    pub value: Option<f64>,
    /// The lever's TILT, in radians, when a contradictory clash was
    /// levered rather than measured outright.
    pub lever_tilt: Option<f64>,
    /// Its ARM, in metres. The arm is the solve's own scale surrogate
    /// — the larger of the two frame origins' distances and the
    /// authored lengths, floored at one metre — and NOT a contact
    /// feature, so it names that scale and nothing in the model.
    /// `lever_tilt * lever_arm` is the `clash` beside it.
    pub lever_arm: Option<f64>,
    /// The length scale a datum named, when it named one too small to
    /// lever a verdict over.
    pub extent: Option<f64>,
    /// The floor that scale is under.
    pub floor: Option<f64>,
}

impl MateFaultPayload {
    /// Which attributes this payload CARRIES, in the order the value
    /// publishes them.
    ///
    /// The destructuring is exhaustive with no `..`, so a field added
    /// to the record and not answered here fails to compile — the
    /// same alarm the match over [`MateFault`] is, one level in.
    pub fn presence(&self) -> [(&'static str, bool); 31] {
        let Self {
            mate,
            side,
            head,
            placer,
            error,
            instance,
            parent,
            child,
            residual,
            held,
            added,
            predicate,
            clash,
            part,
            named,
            selected,
            what,
            expected_document,
            found_document,
            inner_variant,
            margin,
            margin_low,
            margin_high,
            zero,
            escalate,
            field,
            value,
            lever_tilt,
            lever_arm,
            extent,
            floor,
        } = self;
        [
            ("mate", mate.is_some()),
            ("side", side.is_some()),
            ("head", head.is_some()),
            ("placer", placer.is_some()),
            ("error", error.is_some()),
            ("instance", instance.is_some()),
            ("parent", parent.is_some()),
            ("child", child.is_some()),
            ("residual", residual.is_some()),
            ("held", held.is_some()),
            ("added", added.is_some()),
            ("predicate", predicate.is_some()),
            ("clash", clash.is_some()),
            ("part", part.is_some()),
            ("named", named.is_some()),
            ("selected", selected.is_some()),
            ("what", what.is_some()),
            ("expected_document", expected_document.is_some()),
            ("found_document", found_document.is_some()),
            ("inner_variant", inner_variant.is_some()),
            ("margin", margin.is_some()),
            ("margin_low", margin_low.is_some()),
            ("margin_high", margin_high.is_some()),
            ("zero", zero.is_some()),
            ("escalate", escalate.is_some()),
            ("field", field.is_some()),
            ("value", value.is_some()),
            ("lever_tilt", lever_tilt.is_some()),
            ("lever_arm", lever_arm.is_some()),
            ("extent", extent.is_some()),
            ("floor", floor.is_some()),
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
        head: None,
        placer: None,
        error: None,
        instance: None,
        parent: None,
        child: None,
        residual: None,
        held: None,
        added: None,
        predicate: None,
        clash: None,
        part: None,
        named: None,
        selected: None,
        what: None,
        expected_document: None,
        found_document: None,
        inner_variant: None,
        margin: None,
        margin_low: None,
        margin_high: None,
        zero: None,
        escalate: None,
        field: None,
        value: None,
        lever_tilt: None,
        lever_arm: None,
        extent: None,
        floor: None,
    };
}

/// The classifier's escalation, on the fields the frame door
/// publishes it under — one fork, in [`crate::escalation`], called by
/// both doors.
fn with_escalation(base: MateFaultPayload, diag: &Indeterminate) -> MateFaultPayload {
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

/// A band constructor's refusal, on the frame door's own words: which
/// threshold was rejected and what it was, or the pair a band could
/// not be formed from.
fn with_band(base: MateFaultPayload, error: &BandError) -> MateFaultPayload {
    match error {
        BandError::InvalidValue { field, value } => MateFaultPayload {
            field: Some(band_field_tag(field)),
            value: Some(*value),
            ..base
        },
        BandError::InvalidLeverArm { value } => MateFaultPayload {
            value: Some(*value),
            ..base
        },
        BandError::Empty { zero, escalate } => MateFaultPayload {
            zero: Some(*zero),
            escalate: Some(*escalate),
            ..base
        },
    }
}

/// The frame ladder's refusal: its own word, and whatever the arm
/// under it carries.
///
/// `inner_variant` is ONE level in — the word `FrameError` crosses
/// under at the frame door itself, `band` included. A band refusal
/// two levels down is what `field`, `value`, `zero` and `escalate`
/// then say, which is the whole of its payload.
fn with_frame(base: MateFaultPayload, error: &FrameError) -> MateFaultPayload {
    let base = MateFaultPayload {
        inner_variant: Some(frame_error_tag(error)),
        ..base
    };
    match error {
        // A definite zero carries no classification: the arm refused
        // outright rather than landing in the band.
        FrameError::Degenerate {
            indeterminate: None,
            ..
        } => base,
        FrameError::Degenerate {
            indeterminate: Some(diag),
            ..
        } => with_escalation(base, diag),
        FrameError::Band(inner) => with_band(base, inner),
    }
}

/// Flatten one mate refusal into its payload record.
///
/// The match is EXHAUSTIVE with no wildcard arm: an arm added to
/// [`MateFault`] fails this build rather than reaching Python with an
/// all-`None` payload nobody chose for it. An arm that names a mate
/// answers `mate`; an arm that does not answers `None` BY NAME.
pub fn mate_payload(fault: &MateFault) -> MateFaultPayload {
    let none = MateFaultPayload::NONE;
    match fault {
        // The two arms whose subject is not a mate at all. The
        // document ids one names, and the band refusal the other
        // holds, belong to those types' own vocabularies and stay in
        // the prose.
        MateFault::PosesOfAnotherDocument { expected, found } => MateFaultPayload {
            expected_document: Some(*expected),
            found_document: Some(*found),
            ..none
        },
        MateFault::Band { error } => with_band(
            MateFaultPayload {
                inner_variant: Some(band_error_tag(error)),
                ..none
            },
            error,
        ),
        // `error` here is the frame ladder's refusal, a different
        // type from the placer refusal this record's `error` carries:
        // it crosses as `inner_variant` and the classifier's own
        // words, never on that attribute.
        MateFault::Frame { mate, side, error } => with_frame(
            MateFaultPayload {
                mate: Some(*mate),
                side: Some(*side),
                ..none
            },
            error,
        ),
        MateFault::ClassNotAdmitted { mate } => MateFaultPayload {
            mate: Some(*mate),
            ..none
        },
        // A struct, not an enum: the escalation has no inner WORD,
        // and its shape is which margin attribute is set.
        MateFault::Indeterminate { mate, diag } => with_escalation(
            MateFaultPayload {
                mate: Some(*mate),
                ..none
            },
            diag,
        ),
        MateFault::Unleverable { mate, refusal } => {
            let LeverRefusal::DatumTooSmall { extent, floor } = refusal;
            MateFaultPayload {
                mate: Some(*mate),
                inner_variant: Some(lever_refusal_tag(refusal)),
                extent: Some(*extent),
                floor: Some(*floor),
                ..none
            }
        }
        MateFault::TableLacks { mate, what } => MateFaultPayload {
            mate: Some(*mate),
            what: Some(what),
            ..none
        },
        // The lever is the solve's own scale surrogate rather than
        // anything in the model, and `clash` is the PRODUCT of its
        // two halves: an arm that measured its margin without a lever
        // carries neither half.
        MateFault::Contradictory {
            held,
            added,
            predicate,
            clash,
            lever,
        } => MateFaultPayload {
            held: Some(*held),
            added: Some(*added),
            predicate: Some(predicate),
            clash: Some(*clash),
            lever_tilt: lever.map(|(radians, _)| radians),
            lever_arm: lever.map(|(_, arm)| arm),
            ..none
        },
        MateFault::Under {
            mate,
            parent,
            child,
            residual,
        } => MateFaultPayload {
            mate: Some(*mate),
            parent: Some(*parent),
            child: Some(*child),
            residual: Some(*residual),
            ..none
        },
        MateFault::DanglingHead { mate, side, head } => MateFaultPayload {
            mate: Some(*mate),
            side: Some(*side),
            head: Some(*head),
            ..none
        },
        MateFault::PlacerRefused {
            mate,
            side,
            placer,
            error,
        } => MateFaultPayload {
            mate: Some(*mate),
            side: Some(*side),
            placer: Some(*placer),
            error: Some(node_error_tag(error.kind())),
            ..none
        },
        MateFault::PartSelectsAnotherCopy {
            mate,
            side,
            part,
            named,
            selected,
        } => MateFaultPayload {
            mate: Some(*mate),
            side: Some(*side),
            part: Some(*part),
            named: Some(*named),
            selected: Some(*selected),
            ..none
        },
        MateFault::SelfMate { mate, instance } => MateFaultPayload {
            mate: Some(*mate),
            instance: Some(*instance),
            ..none
        },
    }
}
