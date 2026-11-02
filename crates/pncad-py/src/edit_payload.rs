//! The `EditError` payload, flattened once for the exception that
//! carries it.
//!
//! `crate::tags::edit_error_tag` answers WHICH edit refused and
//! `edit_inner_variant_tag` the arm of the refusal it holds. This
//! module answers the third question — what the arm CARRIES — as a
//! record whose every field is present on every arm, `None` where the
//! arm does not carry one, so `getattr` never raises on the Python
//! side and a caller reads a payload without first branching on
//! `variant`.
//!
//! # Why it lives outside `py`
//!
//! The match is a DRIFT ALARM: it is exhaustive with no wildcard, so
//! an arm added kernel-side is a compile error here rather than a
//! silently unprojected payload. `crate::py` compiles only under the
//! `python` feature, and hosted CI's default row has no interpreter —
//! so a projection sited there is an alarm that does not ring on the
//! row that runs everywhere. Sited here it rings on every build, and
//! `src/tests.rs` can construct the arms no Python door reaches and
//! read every field of each.
//!
//! # The flattening
//!
//! One attribute per CONCEPT, not per field name. Where two arms name
//! one concept differently the kernel's clearest name wins and the
//! mapping is stated at the arm — the `readback_err` `which`/`through`
//! precedent. Where two arms name two concepts the SAME, one keeps the
//! kernel's word and the other is spelled apart, because one attribute
//! cannot hold two types.
//!
//! A NESTED REFUSAL is not flattened: `ProfileProgramRefused`,
//! `Dimension`, `InvalidDistribution`,
//! `PlacementAxis` and `MetaUnversioned` each hold another error type,
//! `inner_variant` names its arm, and the fields inside it belong to
//! that type's own door.

use pncad::document::{
    ConstraintId, ContentPin, EditError, FreeValue, HeldNodes, MateFault, RecipeNodeId, VarName,
    VarRef,
};
use pncad::prelude::StableName;
use pncad::select::EntityKind;

use crate::errors::{dimension_tag, slot_kind_tag, var_kind_tag};
use crate::tags::{attr_kind_tag, slot_id_tag};

/// The position a slot names inside a list, for a loft's section or a
/// union's member.
fn operand_index(slot: &pncad::document::SlotId) -> Option<usize> {
    use pncad::document::OperandSlot as S;
    match slot {
        pncad::document::SlotId::Operand(S::Section(i) | S::Member(i)) => usize::try_from(*i).ok(),
        _ => None,
    }
}

/// What one [`EditError`] arm carries, every field present.
///
/// Borrowed from the refusal rather than cloned: the payload is read
/// once, at the raise, and a `String` key or a stable name copied on
/// the way to a Python string would be copied twice.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EditPayload<'a> {
    /// The node the refusal is ABOUT — the edit's target, the node
    /// being written, the node on the cycle.
    pub node: Option<RecipeNodeId>,
    /// A node the subject NAMES: an operand that does not resolve, the
    /// measure an assertion constrains.
    pub input: Option<RecipeNodeId>,
    /// The named expression slot the refusal is about
    /// ([`crate::tags::slot_id_tag`]).
    pub slot: Option<&'static str>,
    /// The DOCUMENT PARAMETER the refusal is about.
    pub param: Option<&'a VarName>,
    /// The STABLE NAME the refusal is about.
    pub name: Option<&'a StableName>,
    /// The appearance-metadata key.
    pub key: Option<&'a str>,
    /// The dimension the door REQUIRED, as
    /// [`crate::errors::dimension_tag`] spells it.
    pub expected: Option<&'static str>,
    /// The dimension it was OFFERED, same spelling.
    pub found: Option<&'static str>,
    /// Which display attribute ([`crate::tags::attr_kind_tag`]).
    pub kind: Option<&'static str>,
    /// A rebind's SOURCE entity kind.
    pub from_kind: Option<EntityKind>,
    /// A rebind's TARGET entity kind.
    pub to_kind: Option<EntityKind>,
    /// The entity kind a selection's name DENOTES, where the refusal is
    /// that the seat reads another (or reads no selection of it).
    pub found_kind: Option<EntityKind>,
    /// The entity kind the seat READS, beside [`Self::found_kind`].
    pub expected_kind: Option<EntityKind>,
    /// A count the refusal names: the nodes a definition expands to,
    /// or the names a selection holds.
    pub count: Option<usize>,
    /// The count the edit HANDED, beside the one the node holds
    /// ([`Self::count`]): a loft's sections against its re-pointing.
    pub given: Option<usize>,
    /// The position a selection's shape fault is reported at. The
    /// fault decides which position it is: a repeated face's first
    /// occurrence, the edge that does not sort strictly before the one
    /// after it, or the name of another entity kind than the
    /// selection's, or the step of a pick's path that names no piece.
    /// Each is one index into one list, so they share the attribute
    /// rather than minting a second word for it.
    pub first: Option<usize>,
    /// The position at which a repeat is named AGAIN — carried only by
    /// a repeated face, the one fault that names two entries.
    pub again: Option<usize>,
    /// A refused scalar the door names in its own right — a
    /// tolerance's ε.
    pub value: Option<f64>,
    /// The doc-parameter value a kind-mismatching value edit offered.
    pub offered: Option<FreeValue>,
    /// Which of a node's placement frames: a step of a transform's
    /// chain, or an explicit rule's listed placement.
    pub index: Option<usize>,
    /// The constraint of the placement [`Self::node`] the refusal is
    /// about: with the node, the constraint's address (`MateRef`).
    pub constraint: Option<ConstraintId>,
    /// The AST child indices of an expression address, from the
    /// slot's root.
    pub path: Option<&'a [u8]>,
    /// Where inside a metadata value the offending float sits.
    pub value_path: Option<&'a str>,
    /// The content pin a reference already names.
    pub pin: Option<ContentPin>,
    /// The solve's own fault about a mate the door refused on its
    /// datum — the one nested refusal that crosses as a VALUE rather
    /// than as a word alone, because it is the same value
    /// `SolvedPoses.fault` answers for a mate the solve refused, and a
    /// caller reads its lever, its clash and its recourse off the
    /// `MateFault` type it already knows. Beside it, the nodes it
    /// names as the door's document held them, which its words speak.
    pub fault: Option<(&'a MateFault, &'a HeldNodes)>,
}

impl EditPayload<'_> {
    /// Which attributes this payload CARRIES, in the order the
    /// exception publishes them.
    ///
    /// The destructuring is exhaustive with no `..`, so a field added
    /// to the record and not answered here fails to compile — the
    /// same alarm the match over `EditError` is, one level in.
    pub fn presence(&self) -> [(&'static str, bool); 25] {
        let Self {
            node,
            input,
            slot,
            param,
            name,
            key,
            expected,
            found,
            kind,
            from_kind,
            to_kind,
            found_kind,
            expected_kind,
            count,
            given,
            first,
            again,
            value,
            offered,
            index,
            constraint,
            path,
            value_path,
            pin,
            fault,
        } = self;
        [
            ("node", node.is_some()),
            ("input", input.is_some()),
            ("slot", slot.is_some()),
            ("param", param.is_some()),
            ("name", name.is_some()),
            ("key", key.is_some()),
            ("expected", expected.is_some()),
            ("found", found.is_some()),
            ("kind", kind.is_some()),
            ("from_kind", from_kind.is_some()),
            ("to_kind", to_kind.is_some()),
            ("found_kind", found_kind.is_some()),
            ("expected_kind", expected_kind.is_some()),
            ("count", count.is_some()),
            ("given", given.is_some()),
            ("first", first.is_some()),
            ("again", again.is_some()),
            ("value", value.is_some()),
            ("offered", offered.is_some()),
            ("index", index.is_some()),
            ("constraint", constraint.is_some()),
            ("path", path.is_some()),
            ("value_path", value_path.is_some()),
            ("pin", pin.is_some()),
            ("fault", fault.is_some()),
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
        node: None,
        input: None,
        slot: None,
        param: None,
        name: None,
        key: None,
        expected: None,
        found: None,
        kind: None,
        from_kind: None,
        to_kind: None,
        found_kind: None,
        expected_kind: None,
        count: None,
        given: None,
        first: None,
        again: None,
        value: None,
        offered: None,
        index: None,
        constraint: None,
        path: None,
        value_path: None,
        pin: None,
        fault: None,
    };
}

/// Flatten one edit refusal into its payload record.
///
/// The match is EXHAUSTIVE with no wildcard arm: an arm added to
/// `EditError` fails this build rather than reaching Python with an
/// all-`None` payload nobody chose for it.
pub fn edit_payload(err: &EditError) -> EditPayload<'_> {
    let none = EditPayload::NONE;
    let dim = dimension_tag;
    match err {
        // The subject node under three kernel spellings — `id` at the
        // doors that take a target, `node` at the doors that write
        // one, `at` where the fault is a position in the graph.
        EditError::UnknownNode { id } => EditPayload {
            node: Some(id.id()),
            ..none
        },
        // The mate is the subject, and the solve's fault about it
        // crosses whole: `inner_variant` says which arm, `fault` is
        // the arm's own payload.
        EditError::MateRefused { node, fault, held } => EditPayload {
            node: Some(node.id()),
            fault: Some((fault, held)),
            ..none
        },
        EditError::WouldCycle { at } | EditError::ReadSiteMissingNode { at } => EditPayload {
            node: Some(at.id()),
            ..none
        },
        EditError::SetMembersOnNonList { node }
        | EditError::SetLoftSectionsOnNonLoft { node }
        | EditError::LoftSectionsSpelled { node }
        | EditError::SetProgramOnNonProfile { node }
        | EditError::WitnessOnNonSketch { node }
        | EditError::DuplicateWitnessEntry { node }
        | EditError::TakeOverRepeatsCopy { node }
        | EditError::WorldIsUndeletable { node }
        | EditError::ConstraintOnNonPlacement { node }
        | EditError::SpaceMismatch { node, a: _, b: _ }
        | EditError::IndicesUnrelated { node, a: _, b: _ }
        | EditError::UpdateOnNonInstance { node }
        | EditError::LabelUnchanged { node } => EditPayload {
            node: Some(node.id()),
            ..none
        },
        // The nested refusals: `inner_variant` names the arm and the
        // fields inside it stay on that type's own door.
        EditError::ProfileProgramRefused { node, refusal: _ }
        | EditError::StepIdsRefused { node, fault: _ } => EditPayload {
            node: Some(node.id()),
            ..none
        },
        EditError::UnresolvedInput { input } => EditPayload {
            input: Some(input.id()),
            ..none
        },
        // A constraint's address: the placement is the subject, and the
        // id beside it names the constraint.
        EditError::UnknownConstraint { node, constraint } => EditPayload {
            node: Some(node.id()),
            constraint: Some(*constraint),
            ..none
        },
        EditError::AssertionDimension {
            node,
            measured,
            bound,
        } => EditPayload {
            node: Some(node.id()),
            expected: Some(dim(*measured)),
            found: Some(dim(*bound)),
            ..none
        },
        // One position for an edge set, not two: its canonical form
        // breaks between an entry and its successor, so the successor's
        // index is the entry's plus one and publishing it would be
        // arithmetic dressed as data.
        EditError::SelectionShape { node, slot, fault } => {
            use pncad::document::SelectionFault as F;
            let (first, again, count) = match fault {
                F::Repeated { first, again } => (Some(*first), Some(*again), None),
                F::NotCanonical { at } | F::Kind { at, .. } => (Some(*at), None, None),
                F::Count { count, .. } => (None, None, Some(*count)),
                F::NotASelection { .. } | F::Seat { .. } | F::OtherBody => (None, None, None),
            };
            let (found_kind, expected_kind) = match fault {
                F::Kind {
                    found, expected, ..
                } => (Some(*found), Some(*expected)),
                F::Seat { entity } => (Some(*entity), None),
                F::Repeated { .. }
                | F::NotCanonical { .. }
                | F::Count { .. }
                | F::NotASelection { .. }
                | F::OtherBody => (None, None),
            };
            EditPayload {
                node: Some(node.id()),
                slot: Some(slot_id_tag(slot)),
                index: operand_index(slot),
                first,
                again,
                count,
                found_kind,
                expected_kind,
                ..none
            }
        }
        EditError::SelectionNameKind {
            node,
            slot,
            at,
            name,
            found,
            expected,
        } => EditPayload {
            node: Some(node.id()),
            slot: Some(slot_id_tag(slot)),
            index: operand_index(slot),
            name: Some(name.name()),
            first: Some(*at),
            found_kind: Some(*found),
            expected_kind: Some(*expected),
            ..none
        },
        EditError::PoseShape { node, slot, fault: _ } => EditPayload {
            node: Some(node.id()),
            slot: Some(slot_id_tag(slot)),
            index: operand_index(slot),
            ..none
        },
        EditError::IndexedRead { node, .. } => EditPayload {
            node: Some(node.id()),
            ..none
        },
        // An operand's slot rides `slot` in the slot vocabulary, its
        // field's word, and a section's or a member's position rides
        // `index`.
        EditError::OperandUnresolved { node, slot, read: _ } => EditPayload {
            node: Some(node.id()),
            slot: Some(slot_id_tag(slot)),
            index: operand_index(slot),
            ..none
        },
        EditError::AmbiguousOutput {
            input,
            slot,
            ports: _,
        }
        | EditError::DefinesNothing { input, slot } => {
            EditPayload {
                input: Some(input.id()),
                slot: Some(slot_id_tag(slot)),
                index: operand_index(slot),
                ..none
            }
        }
        // The step of the pick's path that names no piece is one
        // index into one list, so it rides `first`.
        EditError::PickPath { node, slot, at } => EditPayload {
            node: Some(node.id()),
            slot: Some(slot_id_tag(slot)),
            index: operand_index(slot),
            first: *at,
            ..none
        },
        // Why the read is no product rides `inner_variant`.
        EditError::ProductRead { .. } => none,
        EditError::UnknownSlot { id, slot } => EditPayload {
            node: Some(id.id()),
            slot: Some(slot_id_tag(slot)),
            index: operand_index(slot),
            ..none
        },
        EditError::SlotDimensionMismatch {
            slot,
            expected,
            found,
        } => EditPayload {
            slot: Some(slot_id_tag(slot)),
            expected: Some(slot_kind_tag(*expected)),
            found: Some(dim(*found)),
            ..none
        },
        EditError::StructuralSlotNeedsStructuralEdit { slot }
        | EditError::NotStructuralSlot { slot } => EditPayload {
            slot: Some(slot_id_tag(slot)),
            ..none
        },
        EditError::PayloadUnknownVarName { name, node } => EditPayload {
            node: Some(node.id()),
            param: Some(name),
            ..none
        },
        // `declared`/`referenced` are the same two concepts
        // `SlotDimensionMismatch` calls `expected`/`found` — what the
        // door required, and what it was offered.
        EditError::PayloadVarKind {
            var,
            node,
            declared,
            referenced,
        } => EditPayload {
            node: Some(node.id()),
            param: var.name(),
            expected: Some(var_kind_tag(*declared)),
            found: Some(dim(*referenced)),
            ..none
        },
        EditError::SlotUnknownVarName { name, node, slot } => EditPayload {
            node: Some(node.id()),
            param: Some(name),
            slot: Some(slot_id_tag(slot)),
            ..none
        },
        // What the slot takes and the kind it was offered, at an
        // expression's slot as at an operand's.
        EditError::SlotVarKind {
            var,
            node,
            slot,
            found,
            expected,
        } => EditPayload {
            node: Some(node.id()),
            param: var.name(),
            slot: Some(slot_id_tag(slot)),
            index: operand_index(slot),
            expected: Some(slot_kind_tag(*expected)),
            found: Some(var_kind_tag(*found)),
            ..none
        },
        EditError::ConstructionReadsObserved { node, slot, var } => EditPayload {
            node: Some(node.id()),
            param: var.name(),
            slot: Some(slot_id_tag(slot)),
            index: operand_index(slot),
            ..none
        },
        EditError::ContinuousVarCannotBeCount { var }
        | EditError::VarCountHasNoUnit { var }
        | EditError::VarCountHasNoDistribution { var }
        // The field the refusal names does not cross: a caller holds
        // the parameter it just submitted, and `non_finite_var`
        // plus the Rust sentence say which float it was.
        | EditError::NonFiniteVar { var, field: _ }
        | EditError::InvalidDistribution { var, fault: _ } => EditPayload {
            param: var.name(),
            ..none
        },
        // An address by id carries no name to cross; the tag and the
        // sentence say which edit was refused.
        EditError::UnknownVar { var, door: _ } => EditPayload {
            param: match var {
                VarRef::Name(name) => Some(name),
                VarRef::Id(_) => None,
            },
            ..none
        },
        // A definition's faults name the variable defined; the
        // variable it reads, or the cycle, rides in the sentence.
        EditError::VarIsAnOutput { var, node, door: _ } => EditPayload {
            node: Some(node.id()),
            param: var.name(),
            ..none
        },
        EditError::NotAFreeVar { var, door: _ }
        | EditError::DefinitionCycle { var, through: _ }
        | EditError::DefinitionUnknownVarName { var, name: _ }
        | EditError::DefinitionUnresolvedVar { var, read: _ } => EditPayload {
            param: var.name(),
            ..none
        },
        EditError::Index { var, fault } => EditPayload {
            param: var.name(),
            found: match fault {
                pncad::document::IndexFault::CountNotCount { found } => Some(dim(*found)),
                pncad::document::IndexFault::WithinNoIndex => None,
            },
            ..none
        },
        EditError::LoftShape { node, fault } => match fault {
            pncad::document::LoftFault::TooFewSections { have } => EditPayload {
                node: Some(node.id()),
                count: Some(*have),
                ..none
            },
            pncad::document::LoftFault::EmptyStation { section }
            | pncad::document::LoftFault::StationInFirstPlane { section } => EditPayload {
                node: Some(node.id()),
                index: Some(*section),
                ..none
            },
            pncad::document::LoftFault::DegreeOutOfRange { degree, sections } => EditPayload {
                node: Some(node.id()),
                count: Some(*sections),
                given: usize::try_from(*degree).ok(),
                ..none
            },
        },
        EditError::LoftSectionCount {
            node,
            sections,
            given,
        } => EditPayload {
            node: Some(node.id()),
            count: Some(*sections),
            given: Some(*given),
            ..none
        },
        EditError::DefinitionTooLarge { var, nodes } => EditPayload {
            param: var.name(),
            count: Some(*nodes),
            ..none
        },
        EditError::DefinitionVarKind {
            var,
            read: _,
            declared,
            referenced,
        } => EditPayload {
            param: var.name(),
            expected: Some(var_kind_tag(*declared)),
            found: Some(dim(*referenced)),
            ..none
        },
        EditError::VarNameTaken { name, holder: _ } => EditPayload {
            param: Some(name),
            ..none
        },
        EditError::FreshUnheld { .. } | EditError::FreshKind { .. } | EditError::FreshUnread { .. } => {
            none
        }
        EditError::SlotUnresolvedVar { var, node, slot } => EditPayload {
            node: Some(node.id()),
            param: var.name(),
            slot: Some(slot_id_tag(slot)),
            ..none
        },
        EditError::PayloadUnresolvedVar { var, node } => EditPayload {
            node: Some(node.id()),
            param: var.name(),
            ..none
        },
        EditError::VarNameUnchanged { var }
        | EditError::AnonymousVarUnread { var }
        | EditError::SharedVarNeedsName { var }
        | EditError::DeleteAnonymousVar { var } => EditPayload {
            param: var.name(),
            ..none
        },
        EditError::VarKindFixed {
            var,
            kind,
            offered,
        } => EditPayload {
            param: var.name(),
            expected: Some(var_kind_tag(*kind)),
            found: Some(var_kind_tag(*offered)),
            ..none
        },
        EditError::VarValueKindMismatch {
            var,
            declared,
            offered,
        } => EditPayload {
            param: var.name(),
            expected: Some(dim(*declared)),
            offered: Some(*offered),
            ..none
        },
        // The notation door's dimension fault: `expected` is the
        // declaration's dimension and `found` what the offered unit
        // measures — the pair `slot_var_kind` already
        // spells, over a unit rather than over a reference.
        EditError::VarUnitMismatch {
            var,
            unit,
            declared,
        } => EditPayload {
            param: var.name(),
            expected: Some(dim(*declared)),
            found: Some(dim(*unit)),
            ..none
        },
        // The expression address decomposes into the two attributes
        // that already name its halves, plus the child indices below
        // the slot.
        EditError::PathOffTree { node, slot, path } => EditPayload {
            node: Some(node.id()),
            slot: Some(slot_id_tag(slot)),
            path: Some(path),
            ..none
        },
        EditError::Dimension(_) => none,
        // The document-mismatch arm names two DOCUMENTS, which no
        // attribute of this record carries; the message states both
        // (the `ProductError` arm's precedent, same refusal one door
        // over).
        EditError::EvaluationOfAnotherDocument { .. } => none,
        EditError::DeclareNamesMissingNode { name }
        | EditError::RebindTargetMissingNode { name }
        | EditError::RebindUnknownName { name }
        | EditError::RebindIdentity { name }
        | EditError::RebindNoReferences { name }
        | EditError::NameUnresolvedInEvaluation { name }
        | EditError::AppearanceWrongKind { name }
        | EditError::AppearanceNamesMissingNode { name }
        // The body read crosses in the message; the name is what the
        // caller wrote and repairs.
        | EditError::AppearanceBodyNotARead { name, .. }
        // The step and the counter cross in the message; the name is
        // what the caller wrote and repairs.
        | EditError::NameStepNeverMinted { name, .. } => EditPayload {
            name: Some(name.name()),
            ..none
        },
        // A rebind's two ENTITY kinds. `from` is a Python keyword, so
        // neither half can keep the kernel's bare word and both take
        // the role suffix rather than one of the pair reading oddly.
        EditError::RebindKindMismatch { from, to } => EditPayload {
            from_kind: Some(*from),
            to_kind: Some(*to),
            ..none
        },
        EditError::RebindAppearanceCollision { name, kind }
        | EditError::AppearanceNotSet { name, kind } => EditPayload {
            name: Some(name.name()),
            kind: Some(attr_kind_tag(kind)),
            ..none
        },
        EditError::MetaNotSet { name, key }
        | EditError::RebindMetadataCollision { name, key }
        | EditError::MetaUnversioned {
            name,
            key,
            error: _,
        } => EditPayload {
            name: Some(name.name()),
            key: Some(key),
            ..none
        },
        // `path` here addresses a float inside a METADATA value, not
        // an expression tree: a different address in a different tree,
        // and a `str` where the other is a tuple of child indices, so
        // it is spelled apart rather than folded.
        EditError::MetaNonFinite { name, key, path } => EditPayload {
            name: Some(name.name()),
            key: Some(key),
            value_path: Some(path),
            ..none
        },
        EditError::EmptyWitnessBulk => none,
        EditError::InvalidTolerance { value } => EditPayload {
            value: Some(*value),
            ..none
        },
        EditError::PlacementAxis { error: _ } => none,
        EditError::PinUnchanged { node, pin } => EditPayload {
            node: Some(node.id()),
            pin: Some(*pin),
            ..none
        },
    }
}
