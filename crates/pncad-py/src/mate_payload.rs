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
//! seventeen attributes over thirteen arms, so a per-accessor match
//! would name the same thirteen arms seventeen times and an arm added
//! kernel-side would owe seventeen edits.
//!
//! # The flattening
//!
//! One attribute per CONCEPT. [`MateFaultPayload::error`] is the
//! PLACER's evaluation refusal, as the tag every node failure crosses
//! with; the kernel's `Frame` and `Band` arms name their own inner
//! refusals `error` too, and those are different types that do not
//! share this attribute — their prose is in `str(fault)`.
//!
//! A NESTED REFUSAL is not flattened: `Frame`'s frame-ladder refusal,
//! `Band`'s band-constructor refusal, `Indeterminate`'s predicate
//! diagnostics and `Unleverable`'s lever refusal each belong to
//! another type's own vocabulary, as do the two document ids
//! `PosesOfAnotherDocument` names and the lever a `Contradictory`
//! roll was measured over.

use pncad::document::{MateFault, MateSide, RecipeNodeId, Subgroup};

use crate::tags::node_error_tag;

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
    /// The predicate that decided against a contradictory pair.
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
}

impl MateFaultPayload {
    /// Which attributes this payload CARRIES, in the order the value
    /// publishes them.
    ///
    /// The destructuring is exhaustive with no `..`, so a field added
    /// to the record and not answered here fails to compile — the
    /// same alarm the match over [`MateFault`] is, one level in.
    pub fn presence(&self) -> [(&'static str, bool); 17] {
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
    };
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
        MateFault::PosesOfAnotherDocument {
            expected: _,
            found: _,
        }
        | MateFault::Band { error: _ } => none,
        // `error` here is the frame ladder's refusal, a different
        // type from the placer refusal this record's `error` carries.
        MateFault::Frame {
            mate,
            side,
            error: _,
        } => MateFaultPayload {
            mate: Some(*mate),
            side: Some(*side),
            ..none
        },
        // `diag` names the predicate's own diagnostics, and
        // `refusal` the lever refusal's scale numbers; both stay in
        // the prose their own types write.
        MateFault::ClassNotAdmitted { mate }
        | MateFault::Indeterminate { mate, diag: _ }
        | MateFault::Unleverable { mate, refusal: _ } => MateFaultPayload {
            mate: Some(*mate),
            ..none
        },
        MateFault::TableLacks { mate, what } => MateFaultPayload {
            mate: Some(*mate),
            what: Some(what),
            ..none
        },
        // `lever` is the solve's own scale surrogate rather than
        // anything in the model, so the pair stays in the prose that
        // says so.
        MateFault::Contradictory {
            held,
            added,
            predicate,
            clash,
            lever: _,
        } => MateFaultPayload {
            held: Some(*held),
            added: Some(*added),
            predicate: Some(predicate),
            clash: Some(*clash),
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
