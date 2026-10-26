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
//! A NESTED REFUSAL crosses as its own word rather than being
//! flattened into this record's other attributes:
//! [`MateFaultPayload::inner_variant`] is the lever refusal's word on
//! `Unleverable`.

use pncad::document::{
    ConstraintId, DocumentId, LeverRefusal, MateFault, MateRef, MateSide, NoChartWhy, Orientation,
    ReachRefusal, RecipeNodeId, VarKind,
};

use crate::tags::{lever_refusal_tag, node_error_tag};

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
    /// over-constrains, or whose bundle leaves a freedom unwritten.
    pub placement: Option<RecipeNodeId>,
    /// The constraints of [`Self::placement`] the fault names: each an
    /// equation of which the rest of the bundle already fixes.
    pub constraints: Option<&'a [ConstraintId]>,
    /// How many of the copy's six freedoms an over-constrained bundle
    /// leaves.
    pub free: Option<u8>,
    /// What a lever's unbounded face is, in its own words.
    pub what: Option<&'static str>,
    /// The document whose placement was asked for.
    pub expected_document: Option<DocumentId>,
    /// The document the solve is of.
    pub found_document: Option<DocumentId>,
    /// **The nested refusal's own word**: the lever refusal's. An arm
    /// with no nested refusal has none.
    pub inner_variant: Option<&'static str>,
    /// The kind of pose a mate's `on` reads, where the two kinds
    /// differ.
    pub on_kind: Option<VarKind>,
    /// The kind of pose its `to` reads.
    pub to_kind: Option<VarKind>,
    /// **The value a value's fault is about**, by its address.
    pub value_ref: Option<MateRef>,
    /// Why a value's freedom has no chart.
    pub why: Option<NoChartWhy>,
    /// The orientation a mate was written with, where it is backwards.
    pub orient: Option<Orientation>,
    /// The orientation to write in its place.
    pub write: Option<Orientation>,
}

impl<'a> MateFaultPayload<'a> {
    /// Which attributes this payload CARRIES, in the order the value
    /// publishes them.
    ///
    /// The destructuring is exhaustive with no `..`, so a field added
    /// to the record and not answered here fails to compile — the
    /// same alarm the match over [`MateFault`] is, one level in.
    pub fn presence(&self) -> [(&'static str, bool); 18] {
        let Self {
            mate,
            side,
            error,
            instance,
            shape,
            placement,
            constraints,
            free,
            what,
            expected_document,
            found_document,
            inner_variant,
            on_kind,
            to_kind,
            value_ref,
            why,
            orient,
            write,
        } = self;
        [
            ("mate", mate.is_some()),
            ("side", side.is_some()),
            ("error", error.is_some()),
            ("instance", instance.is_some()),
            ("shape", shape.is_some()),
            ("placement", placement.is_some()),
            ("constraints", constraints.is_some()),
            ("free", free.is_some()),
            ("what", what.is_some()),
            ("expected_document", expected_document.is_some()),
            ("found_document", found_document.is_some()),
            ("inner_variant", inner_variant.is_some()),
            ("on_kind", on_kind.is_some()),
            ("to_kind", to_kind.is_some()),
            ("value_ref", value_ref.is_some()),
            ("why", why.is_some()),
            ("orient", orient.is_some()),
            ("write", write.is_some()),
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
        free: None,
        what: None,
        expected_document: None,
        found_document: None,
        inner_variant: None,
        on_kind: None,
        to_kind: None,
        value_ref: None,
        why: None,
        orient: None,
        write: None,
    };
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
        MateFault::NotOnTheCopy { mate } | MateFault::Unoriented { mate } => MateFaultPayload {
            mate: Some(*mate),
            ..none
        },
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
        // A bundle's faults: the placement is the subject, and the
        // constraints it names ride beside it, each by its id.
        MateFault::Overconstrained {
            placement,
            constraints,
            free,
        } => MateFaultPayload {
            placement: Some(*placement),
            constraints: Some(constraints),
            free: Some(*free),
            ..none
        },
        MateFault::Unpinned { placement, free } => MateFaultPayload {
            placement: Some(*placement),
            free: Some(*free),
            ..none
        },
        MateFault::KindsDiffer { mate, on, to } => MateFaultPayload {
            mate: Some(*mate),
            on_kind: Some(*on),
            to_kind: Some(*to),
            ..none
        },
        // The orientation written backwards, and the one to write.
        MateFault::Backwards {
            mate,
            orient,
            write,
        } => MateFaultPayload {
            mate: Some(*mate),
            orient: Some(*orient),
            write: Some(*write),
            ..none
        },
        // A value's fault: the value is the subject, by its address.
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
