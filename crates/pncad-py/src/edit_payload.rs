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
//! `MeasureMalformed`, `Dimension`, `InvalidDistribution`,
//! `PlacementAxis` and `MetaUnversioned` each hold another error type,
//! `inner_variant` names its arm, and the fields inside it belong to
//! that type's own door. `Roots` is the exception and it is not a
//! counter-example: its payload is recipe node ids, which are leaf
//! values, so they cross under the node roles every other arm uses.

use pncad::document::{ContentPin, DocParamValue, EditError, ParamName, RecipeNodeId, RootFault};
use pncad::prelude::StableName;
use pncad::select::EntityKind;

use crate::errors::dimension_tag;
use crate::tags::{attr_kind_tag, slot_id_tag};

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
    /// A node the subject NAMES: an operand that does not resolve, an
    /// input reached twice, the measure an assertion constrains.
    pub input: Option<RecipeNodeId>,
    /// A node DOWNSTREAM of [`Self::node`] that references it — the
    /// live consumer a delete would dangle, the descendant root that
    /// makes an ancestor root redundant.
    pub referenced_by: Option<RecipeNodeId>,
    /// The named expression slot the refusal is about
    /// ([`crate::tags::slot_id_tag`]).
    pub slot: Option<&'static str>,
    /// The DOCUMENT PARAMETER the refusal is about.
    pub param: Option<&'a ParamName>,
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
    /// How many entries a short list would have had.
    pub count: Option<usize>,
    /// A repeated designation's FIRST position.
    pub first: Option<u32>,
    /// The position at which it is named AGAIN.
    pub again: Option<u32>,
    /// A refused scalar the door names in its own right — a
    /// tolerance's ε.
    pub value: Option<f64>,
    /// The doc-parameter value a kind-mismatching value edit offered.
    pub offered: Option<DocParamValue>,
    /// A placement frame's linear determinant.
    pub determinant: Option<f64>,
    /// The AST child indices of an expression address, from the
    /// slot's root.
    pub path: Option<&'a [u8]>,
    /// Where inside a metadata value the offending float sits.
    pub value_path: Option<&'a str>,
    /// The content pin a reference already names.
    pub pin: Option<ContentPin>,
}

impl EditPayload<'_> {
    /// Which attributes this payload CARRIES, in the order the
    /// exception publishes them.
    ///
    /// The destructuring is exhaustive with no `..`, so a field added
    /// to the record and not answered here fails to compile — the
    /// same alarm the match over `EditError` is, one level in.
    pub fn presence(&self) -> [(&'static str, bool); 21] {
        let Self {
            node,
            input,
            referenced_by,
            slot,
            param,
            name,
            key,
            expected,
            found,
            kind,
            from_kind,
            to_kind,
            count,
            first,
            again,
            value,
            offered,
            determinant,
            path,
            value_path,
            pin,
        } = self;
        [
            ("node", node.is_some()),
            ("input", input.is_some()),
            ("referenced_by", referenced_by.is_some()),
            ("slot", slot.is_some()),
            ("param", param.is_some()),
            ("name", name.is_some()),
            ("key", key.is_some()),
            ("expected", expected.is_some()),
            ("found", found.is_some()),
            ("kind", kind.is_some()),
            ("from_kind", from_kind.is_some()),
            ("to_kind", to_kind.is_some()),
            ("count", count.is_some()),
            ("first", first.is_some()),
            ("again", again.is_some()),
            ("value", value.is_some()),
            ("offered", offered.is_some()),
            ("determinant", determinant.is_some()),
            ("path", path.is_some()),
            ("value_path", value_path.is_some()),
            ("pin", pin.is_some()),
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
        referenced_by: None,
        slot: None,
        param: None,
        name: None,
        key: None,
        expected: None,
        found: None,
        kind: None,
        from_kind: None,
        to_kind: None,
        count: None,
        first: None,
        again: None,
        value: None,
        offered: None,
        determinant: None,
        path: None,
        value_path: None,
        pin: None,
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
            node: Some(*id),
            ..none
        },
        EditError::WouldCycle { at } | EditError::ReadSiteMissingNode { at } => EditPayload {
            node: Some(*at),
            ..none
        },
        EditError::SetMembersOnNonList { node }
        | EditError::WitnessOnNonSketch { node }
        | EditError::DuplicateWitnessEntry { node }
        | EditError::PlacementOnNonInstance { node }
        | EditError::PlacementRuleMismatch { node }
        | EditError::EmptyPlacementList { node }
        | EditError::NonFinitePlacement { node }
        | EditError::NonFiniteAlignment { node }
        | EditError::UpdateOnNonInstance { node } => EditPayload {
            node: Some(*node),
            ..none
        },
        // The nested refusals: `inner_variant` names the arm and the
        // fields inside it stay on that type's own door.
        EditError::ProfileProgramRefused { node, refusal: _ }
        | EditError::MeasureMalformed { node, fault: _ } => EditPayload {
            node: Some(*node),
            ..none
        },
        EditError::UnresolvedInput { input } => EditPayload {
            input: Some(*input),
            ..none
        },
        EditError::DuplicateInput { node, input }
        | EditError::DeclareInputNotDeclare { node, input } => EditPayload {
            node: Some(*node),
            input: Some(*input),
            ..none
        },
        // An assertion's `measure` IS the node it reads, so it takes
        // the `input` role rather than a fourth node attribute.
        EditError::AssertionTarget { node, measure } => EditPayload {
            node: Some(*node),
            input: Some(*measure),
            ..none
        },
        EditError::AssertionDimension {
            node,
            measure,
            measured,
            bound,
        } => EditPayload {
            node: Some(*node),
            input: Some(*measure),
            expected: Some(dim(*measured)),
            found: Some(dim(*bound)),
            ..none
        },
        EditError::RepeatedDesignation { node, first, again } => EditPayload {
            node: Some(*node),
            first: Some(*first),
            again: Some(*again),
            ..none
        },
        // `found` here is a COUNT, not a dimension, so it takes the
        // `count` attribute: one attribute never carries two types.
        EditError::TooFewMembers { node, found } => EditPayload {
            node: Some(*node),
            count: Some(*found),
            ..none
        },
        EditError::DeleteWouldDangle { id, referenced_by } => EditPayload {
            node: Some(*id),
            referenced_by: Some(*referenced_by),
            ..none
        },
        EditError::UnknownSlot { id, slot } => EditPayload {
            node: Some(*id),
            slot: Some(slot_id_tag(slot)),
            ..none
        },
        EditError::SlotDimensionMismatch {
            slot,
            expected,
            found,
        } => EditPayload {
            slot: Some(slot_id_tag(slot)),
            expected: Some(dim(*expected)),
            found: Some(dim(*found)),
            ..none
        },
        EditError::StructuralSlotNeedsStructuralEdit { slot }
        | EditError::NotStructuralSlot { slot } => EditPayload {
            slot: Some(slot_id_tag(slot)),
            ..none
        },
        EditError::UnknownPayloadParam { name, node } => EditPayload {
            node: Some(*node),
            param: Some(name),
            ..none
        },
        // `declared`/`referenced` are the same two concepts
        // `SlotDimensionMismatch` calls `expected`/`found` — what the
        // door required, and what it was offered.
        EditError::PayloadParamDimensionMismatch {
            name,
            node,
            declared,
            referenced,
        } => EditPayload {
            node: Some(*node),
            param: Some(name),
            expected: Some(dim(*declared)),
            found: Some(dim(*referenced)),
            ..none
        },
        EditError::UnknownDocParam { name, node, slot } => EditPayload {
            node: Some(*node),
            param: Some(name),
            slot: Some(slot_id_tag(slot)),
            ..none
        },
        EditError::DocParamDimensionMismatch {
            name,
            node,
            slot,
            declared,
            referenced,
        } => EditPayload {
            node: Some(*node),
            param: Some(name),
            slot: Some(slot_id_tag(slot)),
            expected: Some(dim(*declared)),
            found: Some(dim(*referenced)),
            ..none
        },
        EditError::ContinuousParamCannotBeCount { name }
        | EditError::DocParamNotDeclared { name }
        | EditError::NonFiniteDocParam { name }
        | EditError::InvalidDistribution { name, fault: _ } => EditPayload {
            param: Some(name),
            ..none
        },
        EditError::DocParamValueKindMismatch {
            name,
            declared,
            offered,
        } => EditPayload {
            param: Some(name),
            expected: Some(dim(*declared)),
            offered: Some(*offered),
            ..none
        },
        // The expression address decomposes into the two attributes
        // that already name its halves, plus the child indices below
        // the slot.
        EditError::PathOffTree { path } => EditPayload {
            node: Some(path.node),
            slot: Some(slot_id_tag(&path.slot)),
            path: Some(&path.path),
            ..none
        },
        EditError::Dimension(_) => none,
        EditError::DeclareNamesMissingNode { name }
        | EditError::RebindTargetMissingNode { name }
        | EditError::RebindUnknownName { name }
        | EditError::RebindIdentity { name }
        | EditError::RebindNoReferences { name }
        | EditError::NameUnresolvedInEvaluation { name }
        | EditError::AppearanceWrongKind { name }
        | EditError::AppearanceNamesMissingNode { name } => EditPayload {
            name: Some(name),
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
            name: Some(name),
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
            name: Some(name),
            key: Some(key),
            ..none
        },
        // `path` here addresses a float inside a METADATA value, not
        // an expression tree: a different address in a different tree,
        // and a `str` where the other is a tuple of child indices, so
        // it is spelled apart rather than folded.
        EditError::MetaNonFinite { name, key, path } => EditPayload {
            name: Some(name),
            key: Some(key),
            value_path: Some(path),
            ..none
        },
        EditError::EmptyWitnessBulk => none,
        EditError::InvalidTolerance { value } => EditPayload {
            value: Some(*value),
            ..none
        },
        EditError::ImproperPlacement { node, determinant } => EditPayload {
            node: Some(*node),
            determinant: Some(*determinant),
            ..none
        },
        EditError::PlacementAxis { error: _ } => none,
        EditError::PinUnchanged { node, pin } => EditPayload {
            node: Some(*node),
            pin: Some(*pin),
            ..none
        },
        // The product-root invariants read their WORD off the fault
        // (`variant` is `root_duplicate`, not `roots`), and their
        // payload is recipe node ids — leaf values, so they cross
        // under the same node roles every other arm uses.
        EditError::Roots(fault) => match fault {
            RootFault::NotLive { root } | RootFault::Duplicate { root } => EditPayload {
                node: Some(*root),
                ..none
            },
            RootFault::Uncovered { node } => EditPayload {
                node: Some(*node),
                ..none
            },
            RootFault::Ancestor {
                ancestor,
                descendant,
            } => EditPayload {
                node: Some(*ancestor),
                referenced_by: Some(*descendant),
                ..none
            },
        },
    }
}
