//! The DocEdit vocabulary v1 and the pure `apply` (spec D2 + D6).
//!
//! `apply` returns a NEW document value; the input is untouched.
//! Undo/redo is keeping prior values — no edit destroys history at
//! this layer (spec D2).

use crate::appearance::{Attr, AttrKind};
use crate::distribution::DistributionFault;
use crate::doc::{DisplayUnitRefusal, Doc, DocParam, DocParamValue, ParamName};
use crate::expr::{Dimension, DimensionError, Expr, ExprPath};
use crate::meta::{MetaValue, MetaVersionError};
use crate::names::EntityKind;
use crate::node::{Node, PlacementRuleFault, RecipeNodeId, SlotId, StableName};
use crate::roots::RootFault;
use crate::witness::{BranchCertification, WitnessDatum};
use geom_core::Tol;

/// The v1 edit vocabulary (spec D6), extended by M4 PR 4 with the two
/// explicit-repair edits: `Rebind` (NAMING-DESIGN N5 — the ONLY name
/// repair; the automatic-rebinding policy menu is EMPTY by ratified
/// decision) and `ReWitness`/`ReWitnessBulk` (SOLVER-DESIGN W4 — the
/// recorded witness adoption; never silent write-back).
///
/// M4 PR 6 landed the reserved `SetTolerance` arm (the recorded-ε
/// edit; its flipped-predicate audit reports through the PR 4
/// verdict-diff engine) plus the D7 metadata pair
/// (`SetAppearanceMeta`/`ClearAppearanceMeta`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum DocEdit<P> {
    /// Insert a node; the new [`RecipeNodeId`] is minted from the
    /// document's monotone counter and returned in the
    /// [`EditRecord`]. Input refs must resolve to EXISTING nodes —
    /// which is also why insertion can never create a cycle.
    InsertNode {
        /// The node payload (data only, spec D3).
        node: Node<P>,
    },
    /// Delete a node. Refused while any live node holds it as an
    /// INPUT (typed, spec D3/D6); the id is never reused afterwards.
    ///
    /// A payload NAME of the node is not an input and does not refuse
    /// (DM7, the §0 carve-out): the edit is accepted and every name it
    /// stranded rides the record as a [`Maintenance::Strand`].
    DeleteNode {
        /// The node to delete.
        id: RecipeNodeId,
    },
    /// **Replace a node's whole LIST input** (DM4) — a union's members,
    /// a loft's sections ([`Node::list_input`]).
    ///
    /// The one edit that changes a live node's inputs, and it can be
    /// that because it is unambiguous by construction: the new list is
    /// stated in full, so nothing is inferred about which of the old
    /// entries survived, moved or was meant. There is no positional
    /// spelling and no per-entry edit; DM6 rules that no other rewiring
    /// edit exists.
    ///
    /// Deleting one member is this edit without it plus a plain
    /// [`DocEdit::DeleteNode`] of the orphaned node, one committed
    /// action.
    ///
    /// Every check [`DocEdit::InsertNode`] makes of a node's inputs is
    /// made here, of the REWRITTEN node, through the same functions:
    /// liveness ([`EditError::UnresolvedInput`]), acyclicity
    /// ([`EditError::WouldCycle`]), pairwise distinctness
    /// ([`EditError::DuplicateInput`]) and the list's own floor
    /// ([`EditError::TooFewMembers`]). A node with no list input
    /// refuses [`EditError::SetMembersOnNonList`].
    SetMembers {
        /// The node whose list is replaced.
        node: RecipeNodeId,
        /// The whole new list, in order (D9: the order is data).
        members: Vec<RecipeNodeId>,
    },
    /// Replace a CONTINUOUS slot's expression (Length/Angle/Scalar
    /// slots; spec D3's continuous parameters).
    SetParam {
        /// The node owning the slot.
        node: RecipeNodeId,
        /// The named slot (spec D5: never an index).
        slot: SlotId,
        /// The replacement expression (dimension re-checked).
        expr: Expr,
    },
    /// Replace a STRUCTURAL (Count-typed) slot's expression — a
    /// DISTINCT arm from [`DocEdit::SetParam`] so the structural/
    /// continuous divide is unlosable in the edit stream (spec D3,
    /// DESIGN.md "stated, not emergent").
    SetStructuralParam {
        /// The node owning the slot.
        node: RecipeNodeId,
        /// The named structural slot.
        slot: SlotId,
        /// The replacement Count expression.
        expr: Expr,
    },
    /// Replace the expression SUBTREE at an [`ExprPath`] (empty path =
    /// the whole slot), re-running dimension checks on rebuilt
    /// ancestors (spec D6).
    SetExpression {
        /// The subtree address.
        path: ExprPath,
        /// The replacement subtree.
        expr: Expr,
    },
    /// Create or replace a document-level named parameter (spec D6).
    /// A dimension change re-validates every referencing expression.
    SetDocParam {
        /// The parameter name.
        name: ParamName,
        /// The declared dimension and exact value.
        value: DocParam,
    },
    /// Write a NEW VALUE into an already-declared document parameter,
    /// keeping its declaration: its dimension and its optional
    /// distribution ride through untouched
    /// ([`DocParam::with_value`]).
    ///
    /// The door [`Self::SetDocParam`] cannot be. That one is
    /// create-or-replace, so it takes a whole `DocParam` and a caller
    /// who assembled one from `(dim, value)` — the natural spelling,
    /// and the only one a value-editing panel, gesture or binding
    /// wants — deletes any annotation the parameter carried, with no
    /// refusal and no diagnostic. This edit removes the way to make
    /// that mistake: there is nothing here to omit.
    ///
    /// Refuses typed on a name the document does not declare (there is
    /// no declaration to carry forward) and on a kind mismatch (a
    /// count for a continuous parameter or the reverse — that is a
    /// redeclaration, and belongs to the other door).
    SetDocParamValue {
        /// The parameter name — must already be declared.
        name: ParamName,
        /// The replacement value.
        value: DocParamValue,
    },
    /// Write a new NOTATION onto an already-declared document
    /// parameter, keeping its declaration: its dimension, its exact
    /// value and its optional distribution ride through untouched
    /// ([`DocParam::with_display_unit`]).
    ///
    /// [`Self::SetDocParamValue`]'s mirror over the other field of the
    /// declaration, and it exists for the same reason. A parameter's
    /// display unit sits on the DECLARATION, beside `dim` and
    /// `distribution`, so the only other way to re-spell it is
    /// [`Self::SetDocParam`] — create-or-replace — with a `DocParam`
    /// the caller assembled, and the natural spelling
    /// ([`DocParam::continuous`] plus the notation) names no
    /// distribution and therefore deletes any the parameter carried.
    /// There is nothing to omit here.
    ///
    /// A notation change is not a redeclaration — the argument, in
    /// full, is [`DocParam::with_display_unit`]'s rustdoc.
    ///
    /// Refuses typed on a name the document does not declare
    /// ([`EditError::DocParamNotDeclared`] — there is no declaration to
    /// carry forward), on a `Count`
    /// ([`EditError::DocParamCountHasNoUnit`] — a count is an integer
    /// and names no notation) and on a unit that does not measure the
    /// declared dimension ([`EditError::DocParamUnitMismatch`] — the
    /// pairing the save/load validator refuses a document for).
    SetDocParamUnit {
        /// The parameter name — must already be declared, and must not
        /// be a `Count`.
        name: ParamName,
        /// The notation to write, which must MEASURE the declared
        /// dimension.
        unit: crate::expr::UnitSym,
    },
    /// The explicit name repair (N5, spec D3): rewrite every document
    /// site that references `from` EXACTLY (Declare pairs and
    /// appearance-store keys in v1) to reference `to`.
    /// One-shot recorded intent — no alias table persists, nothing
    /// follows automatically afterwards (the ratified EMPTY policy
    /// menu). Validation mirrors Declare's edit-time carve-out: node
    /// existence NOW (`to`'s node must be live; `from`'s node must at
    /// least have once existed — a never-minted id is a typo);
    /// name-level resolution stays an evaluation-time concern.
    Rebind {
        /// The name being repaired (may be stranded — its node may be
        /// deleted; that is the `NodeGone` repair case).
        from: StableName,
        /// The selection it now denotes (selections ARE stable names,
        /// G1).
        to: StableName,
    },
    /// The explicit witness adoption (SOLVER-DESIGN W4, spec D5):
    /// records `witness` as the sketch-bearing node's branch
    /// selection. Recorded, replayable, undoable; parameter-edit
    /// rebuilds never write a witness back — this edit is the ONLY
    /// witness-changing event besides a committed sketch edit (M6).
    ReWitness {
        /// The sketch-bearing node.
        node: RecipeNodeId,
        /// The opaque witness datum (schema + bytes).
        witness: WitnessDatum,
    },
    /// The bulk certified-same-branch witness adoption (W4's ratified
    /// amendment): semantically invisible when the certificate holds,
    /// so an editor may record it in bulk (e.g. piggybacked on a
    /// commit) — the certification obligation rides AS DATA and the
    /// M6 solver adds the checker that enforces it (additive, no
    /// schema change). v1 validates shape only (live sketch-bearing
    /// nodes, no duplicates, non-empty).
    ReWitnessBulk {
        /// The per-node witness adoptions.
        entries: Vec<(RecipeNodeId, WitnessDatum)>,
        /// The certified-same-branch evidence (opaque; M6's checker
        /// consumes it).
        certification: BranchCertification,
    },
    /// Attach (or replace) one appearance attribute on a face or body
    /// stable name (M4 PR 7; [`crate::appearance`] module docs).
    /// Validation mirrors `Declare`'s ruled carve-out: the name's
    /// NODE must be live at edit time (a never-existed id is a typo,
    /// refused at the best-diagnostics door); name-LEVEL resolution
    /// happens at evaluation, where a non-resolving name surfaces as
    /// a typed [`crate::appearance::AppearanceLoss`] — never a silent
    /// drop. A later `DeleteNode` MAY strand the attachment (N5
    /// dangling semantics, same as Declare).
    SetAppearance {
        /// The face or body name attributed.
        name: StableName,
        /// The attribute (occupies its [`AttrKind`] slot; one per
        /// kind per name).
        attr: Attr,
    },
    /// Remove one attribute kind from a name. Deliberately does NOT
    /// require the node to be live: clearing is the repair path for
    /// attributes stranded on retired/deleted names.
    ClearAppearance {
        /// The attributed name.
        name: StableName,
        /// The attribute kind removed.
        kind: AttrKind,
    },
    /// The recorded-ε edit (M4 PR 6 spec D4, H4's landing). Applying
    /// records the new ε in the document — a pure value edit, and a
    /// STRUCTURAL one (ε parameterizes every content key: the full
    /// cone recomputes). The audit semantics — persist-grade replay
    /// at the new ε plus the PR 4 diff engine's flipped-predicate
    /// report — necessarily SPAN processes: one process hosts one ε
    /// (`geom_core::Tolerance` commits once), so the replay happens
    /// in a fresh process (save → load) and the two runs' serialized
    /// verdict summaries diff through
    /// [`crate::resolve::diff_summaries`]. In the editing
    /// process, [`crate::eval::evaluate`] refuses a document whose
    /// recorded ε disagrees with the committed process ε — loudly,
    /// per node — so a SetTolerance result is never silently
    /// evaluated at the old ε.
    SetTolerance {
        /// The new modeling tolerance (finite, strictly positive).
        eps: f64,
    },
    /// Attach (or replace) one BLACK-BOX metadata value on a face or
    /// body name's appearance record (M4 PR 6 spec D7). Validation
    /// mirrors `SetAppearance` (kind + node-liveness carve-out), plus
    /// the two metadata doors: the D7 producer convention (a map
    /// carrying an integer `"v"` field — structural enforcement only,
    /// the kernel never reads the version's meaning) and the D2 float
    /// policy (NaN/inf refused at the edit door).
    SetAppearanceMeta {
        /// The face or body name attributed.
        name: StableName,
        /// The metadata key (a producer-owned namespace).
        key: String,
        /// The value tree (stored, round-tripped, never interpreted).
        value: MetaValue,
    },
    /// Remove one metadata key from a name's appearance record. Like
    /// `ClearAppearance`, does NOT require the node to be live
    /// (clearing is the stranded-record repair path) and refuses
    /// loudly when the key is not set.
    ClearAppearanceMeta {
        /// The attributed name.
        name: StableName,
        /// The metadata key removed.
        key: String,
    },
    /// Set the document's ordered product roots outright (A10;
    /// ASM-ROOTS D-3). THE designate/undesignate door: one TOTAL edit
    /// rather than partial add/remove arms, so the product's solid
    /// order is always stated rather than inferred from an edit
    /// sequence. Validator-checked like any other apply, recorded like
    /// any other edit, and undone by keeping the prior value.
    SetRoots {
        /// The new root list, in product order.
        roots: Vec<RecipeNodeId>,
    },
    /// Place an instance's cluster (A11; ASM-2A D-2). The target is
    /// the instantiate node whose singleton cluster moves; the frame
    /// replaces whatever was recorded (the identity, if nothing was).
    /// Recorded and undoable like any other edit — undo restores the
    /// prior registry state, including its ABSENCE.
    SetPlacement {
        /// The instantiate node whose cluster this frame places.
        node: RecipeNodeId,
        /// The cluster's new frame.
        frame: crate::placement::Frame,
    },
    /// Move ONE instance's pin to a new version of the same document
    /// (A13's per-reference primitive; ASM-UPD D-1). The id does not
    /// move — A4 keeps identity and version distinct, and this edit
    /// touches only the version half.
    ///
    /// **The A13 clause-4 contract, verbatim**: *Update triggers
    /// ordinary re-evaluation; once R2-b lands, a pin move on an
    /// instance with crossing declarations additionally triggers mate
    /// re-verification (A4's "does it actually fit" gate — the edit's
    /// contract, stated once). Disk-moved-pin-held staleness is AQ5's
    /// capture question, out of this decision.* No R2-b machinery
    /// exists here; the clause is stated so the later unit extends a
    /// contract rather than inventing one.
    ///
    /// **The new pin is RECIPE DATA, not a resolution.** `apply` does
    /// not reach across the document seam — it has no resolver and no
    /// store — so a pin naming content that does not exist is accepted
    /// here and refused at EVALUATION, through the seam vocabulary
    /// that already names both pins ([`crate::ResolveFault::PinMismatch`]
    /// / [`crate::ResolveFault::Unresolved`]). Checking at the edit
    /// door would make the edit's meaning depend on which store was
    /// mounted when it was recorded, which is exactly what a recorded,
    /// replayable log must not carry.
    UpdateReference {
        /// The instantiate node whose pin moves.
        node: RecipeNodeId,
        /// The version this reference now names. Undo is keeping the
        /// prior document, which still carries the prior pin.
        new_pin: crate::ident::ContentPin,
    },
}

/// Which of the two CARRY-FORWARD doors an edit came through — the
/// edits that write one field of a standing declaration and carry the
/// rest untouched.
///
/// It exists so a refusal both doors share can name the one the caller
/// actually used ([`EditError::DocParamNotDeclared`]). A third door
/// over a third field adds an arm here and the compile names every
/// sentence that has to learn the word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarryForwardDoor {
    /// [`DocEdit::SetDocParamValue`] — the number.
    Value,
    /// [`DocEdit::SetDocParamUnit`] — the notation.
    Notation,
}

// The door as it appears inside a refusal's sentence, in the user's
// vocabulary rather than the enum's: "a value edit", not "Value".
impl core::fmt::Display for CarryForwardDoor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Value => "a value edit",
            Self::Notation => "a notation edit",
        })
    }
}

/// Typed, specific edit refusal (spec D6: no stringly errors).
#[derive(Debug, Clone, PartialEq)]
pub enum EditError {
    /// The edit targets a node id that is not live.
    UnknownNode {
        /// The missing id.
        id: RecipeNodeId,
    },
    /// An edit that CREATES or MODIFIES a profile program failed the
    /// authoring-time check (LIB-SWITCH §4d, VQ9): the program is
    /// resolved, replayed, and validated under the CURRENT parameter
    /// environment at the edit door, so the author sees refusals at
    /// the verb, not at first evaluation. `SetDocParam` deliberately
    /// NEVER takes this door — a parameter edit that breaks a
    /// downstream profile surfaces as that node's typed evaluation
    /// error (V1 class 2: refusing programs may exist at rest); both
    /// directions are pinned by test.
    ProfileProgramRefused {
        /// The profile node (for `InsertNode`, the id being minted).
        node: RecipeNodeId,
        /// The typed refusal, behind a pointer: it is this enum's
        /// widest payload, and every edit door returns the enum BY
        /// VALUE, so held inline it sets the width of every `Result`
        /// in the edit vocabulary and of the persist and replay
        /// refusals that wrap one. `AssemblyError::Product` carries
        /// `ProductError` the same way for the same reason.
        refusal: Box<crate::program::ProgramRefusal>,
    },
    /// An inserted node's input ref does not resolve to a live node
    /// (spec D3: `apply` rejects unresolvable refs).
    UnresolvedInput {
        /// The dangling upstream reference.
        input: RecipeNodeId,
    },
    /// The recipe graph would contain a cycle (defensive: insertion
    /// referencing only pre-existing nodes cannot cycle, but the
    /// invariant is CHECKED, not assumed — spec D3).
    WouldCycle {
        /// A node on the detected cycle.
        at: RecipeNodeId,
    },
    /// **A node's inputs are not pairwise distinct** (DM5): one node
    /// reached twice through one node's edges.
    ///
    /// It is one structural rule over [`Node::inputs`], not a rule per
    /// node kind, so it covers a boolean or a split whose two operands
    /// coincide and a list with a repeated entry alike — and it is
    /// stated once, at [`Node::input_fault`], with this door,
    /// [`DocEdit::SetMembers`] and the load validator as its three
    /// callers.
    DuplicateInput {
        /// The node whose input list repeats.
        node: RecipeNodeId,
        /// The input it reaches twice.
        input: RecipeNodeId,
    },
    /// The node this edit writes names one face twice in its ORDERED
    /// designation ([`crate::node::InputFault::RepeatedDesignation`]):
    /// a hand-built `Node::Shell` that bypassed the construction door,
    /// which drops a repeat keeping the first occurrence. Refused
    /// rather than repaired, at this door as at the load door.
    RepeatedDesignation {
        /// The node whose designation repeats.
        node: RecipeNodeId,
        /// The position of the entry's first occurrence.
        first: u32,
        /// The position at which it is named again.
        again: u32,
    },
    /// The node this edit writes carries a blend selection that is not
    /// canonical — sorted and deduplicated
    /// ([`crate::node::InputFault::SelectionNotCanonical`]): a
    /// hand-built `Node::Fillet` or `Node::Chamfer` that bypassed
    /// [`Node::fillet`]/[`Node::chamfer`], which sort and dedup.
    /// Refused rather than repaired, at this door as at the load door,
    /// because re-sorting would move the node's content key behind the
    /// caller's back.
    SelectionNotCanonical {
        /// The node whose selection is out of canonical form.
        node: RecipeNodeId,
        /// The position of the entry that does not sort strictly
        /// before the one after it.
        at: u32,
    },
    /// `SetMembers` aimed at a node that has no list input
    /// ([`Node::list_input`]) — a boolean's operands are named slots,
    /// and replacing "the list" of a node that has none is not a
    /// smaller version of this edit, it is a different sentence.
    SetMembersOnNonList {
        /// The node that carries no list.
        node: RecipeNodeId,
    },
    /// A list input left with fewer than two entries. A union of one
    /// body is that body and a loft through one section is not a skin:
    /// either is a node whose meaning is its own input, spelled as an
    /// operator.
    TooFewMembers {
        /// The node whose list is short.
        node: RecipeNodeId,
        /// How many entries it would have had.
        found: usize,
    },
    /// Deleting this node would dangle a live reference to it.
    DeleteWouldDangle {
        /// The deletion target.
        id: RecipeNodeId,
        /// A live node still referencing it.
        referenced_by: RecipeNodeId,
    },
    /// The node does not carry the named slot.
    UnknownSlot {
        /// The node.
        id: RecipeNodeId,
        /// The slot it lacks.
        slot: SlotId,
    },
    /// The expression's dimension does not match the slot's required
    /// dimension (checks re-run on every touched expression, spec D6).
    SlotDimensionMismatch {
        /// The slot.
        slot: SlotId,
        /// The slot's required dimension.
        expected: Dimension,
        /// The offered expression's dimension.
        found: Dimension,
    },
    /// `SetParam` aimed at a STRUCTURAL slot — structural edits go
    /// through the distinct `SetStructuralParam` arm (spec D3).
    StructuralSlotNeedsStructuralEdit {
        /// The structural slot.
        slot: SlotId,
    },
    /// `SetStructuralParam` aimed at a continuous slot.
    NotStructuralSlot {
        /// The continuous slot.
        slot: SlotId,
    },
    /// A node's PAYLOAD expression (a measured expression's value
    /// leaf, an assertion's bound — the expressions no slot addresses)
    /// references a document parameter that does not exist. The same
    /// fault as [`EditError::UnknownDocParam`] at an address that is
    /// not a slot, so it says so rather than borrowing a slot name
    /// from a node that has one.
    UnknownPayloadParam {
        /// The missing parameter.
        name: ParamName,
        /// The referencing node.
        node: RecipeNodeId,
    },
    /// A payload expression's recorded ref dimension disagrees with the
    /// document parameter's declared dimension.
    PayloadParamDimensionMismatch {
        /// The parameter.
        name: ParamName,
        /// The referencing node.
        node: RecipeNodeId,
        /// The dimension the parameter is declared with.
        declared: Dimension,
        /// The dimension the expression recorded.
        referenced: Dimension,
    },
    /// A [`Node::Measure`]'s expression reads a reference the node does
    /// not carry ([`crate::MeasureNodeFault`]).
    MeasureMalformed {
        /// The measure node.
        node: RecipeNodeId,
        /// What is wrong with it.
        fault: crate::node::MeasureNodeFault,
    },
    /// A [`Node::Assertion`] references a node that is not a measure.
    /// An assertion constrains a measurement; there is nothing else in
    /// the vocabulary for it to constrain.
    AssertionTarget {
        /// The assertion.
        node: RecipeNodeId,
        /// What it references.
        measure: RecipeNodeId,
    },
    /// A node's `declare` input names a node that is not a
    /// [`Node::Declare`]. The slot carries coincidence INTENT, which
    /// only a `Declare` holds; a body or a datum wired there is a
    /// mis-wire, refused where it is authored rather than at the
    /// evaluation that would have found nothing to resolve.
    DeclareInputNotDeclare {
        /// The consuming node (the boolean or the union).
        node: RecipeNodeId,
        /// What its `declare` input names.
        input: RecipeNodeId,
    },
    /// A [`Node::Assertion`]'s bound is dimensioned differently from
    /// the measure it constrains — refused at the edit door, so a
    /// document never carries a comparison of metres with radians.
    AssertionDimension {
        /// The assertion.
        node: RecipeNodeId,
        /// The measure it constrains.
        measure: RecipeNodeId,
        /// The measure's dimension.
        measured: Dimension,
        /// The bound's.
        bound: Dimension,
    },
    /// An expression references a document parameter that does not
    /// exist.
    UnknownDocParam {
        /// The missing parameter.
        name: ParamName,
        /// The referencing node.
        node: RecipeNodeId,
        /// The referencing slot.
        slot: SlotId,
    },
    /// An expression's recorded ref dimension disagrees with the
    /// document parameter's declared dimension.
    DocParamDimensionMismatch {
        /// The parameter.
        name: ParamName,
        /// The referencing node.
        node: RecipeNodeId,
        /// The referencing slot.
        slot: SlotId,
        /// The document table's declared dimension.
        declared: Dimension,
        /// The dimension the expression's ref recorded.
        referenced: Dimension,
    },
    /// A `Continuous` doc param declared with `Dimension::Count` —
    /// Count parameters use [`DocParam::Count`] (exact integers).
    ContinuousParamCannotBeCount {
        /// The parameter.
        name: ParamName,
    },
    /// A carry-forward edit — [`DocEdit::SetDocParamValue`] or
    /// [`DocEdit::SetDocParamUnit`] — named a parameter this document
    /// does not declare. Both doors carry an existing declaration
    /// forward, so there has to be one; declaring a parameter is
    /// [`DocEdit::SetDocParam`]'s job.
    ///
    /// ONE arm for both doors because the FAULT is one — the missing
    /// declaration, which neither door is about — and so is the
    /// recourse. What differs is which edit the user submitted, and
    /// that rides along in `door` so the sentence can say it: a
    /// refusal that read "a carry-forward edit" would make a reader
    /// work out which of their two edits it was talking about.
    DocParamNotDeclared {
        /// The undeclared parameter.
        name: ParamName,
        /// Which carry-forward edit was refused.
        door: CarryForwardDoor,
    },
    /// A notation edit ([`DocEdit::SetDocParamUnit`]) named a `Count`
    /// parameter. A count is an exact integer, not a quantity: it
    /// names no notation and carries no field to write one into.
    ///
    /// Distinct from [`Self::DocParamValueKindMismatch`], which is a
    /// value offered at the wrong kind and would be a redeclaration.
    /// Nothing is being redeclared here — there is no notation for a
    /// count under ANY declaration.
    DocParamCountHasNoUnit {
        /// The count parameter.
        name: ParamName,
    },
    /// A notation edit ([`DocEdit::SetDocParamUnit`]) offered a unit
    /// that does not MEASURE the parameter's declared dimension —
    /// millimetres for an angle, degrees for a length.
    ///
    /// The same pairing the shared save/load validator refuses a
    /// document for (`PersistError::DisplayUnit`) and the authoring
    /// doors ([`DocParam::written_length`], [`DocParam::written_angle`])
    /// make unreachable by construction; this is that fault refused at
    /// the edit door, before it can reach a document at all.
    ///
    /// Raised by BOTH doors that write a declaration — this one and
    /// [`DocEdit::SetDocParam`], the create-or-replace door, whose
    /// payload is `pub` and can pair any unit with any dimension.
    ///
    /// Its sentence is `PersistError::DisplayUnit`'s shape — *declared
    /// X but the unit measures Y* — with ONE word of difference,
    /// deliberately: the validator says "its display unit", because
    /// there the unit is a fact already stored on the document, and
    /// this says "the display unit offered", because here it is an
    /// argument that never reached one.
    DocParamUnitMismatch {
        /// The parameter.
        name: ParamName,
        /// The dimension the offered unit measures.
        unit: Dimension,
        /// The dimension the document declares.
        declared: Dimension,
    },
    /// A value-only edit offered a value of the wrong kind — a count
    /// for a continuous parameter or a continuous value for a count.
    /// Changing a parameter's kind is a REDECLARATION
    /// ([`DocEdit::SetDocParam`]), where the dimension and the
    /// distribution are stated afresh rather than carried.
    DocParamValueKindMismatch {
        /// The parameter.
        name: ParamName,
        /// The dimension the document declares (`Count` for a count
        /// parameter).
        declared: Dimension,
        /// The value the edit offered.
        offered: DocParamValue,
    },
    /// A `SetExpression` path runs off the expression tree (spec D5).
    PathOffTree {
        /// The offending address.
        path: ExprPath,
    },
    /// Replacing the subtree broke an ancestor's dimension check.
    Dimension(DimensionError),
    /// A name-referencing payload — a `Declare`'s pairs or a
    /// `Fillet`'s selection (M6-5) — names a node that does not exist
    /// at edit time (spec D3 carve-out, ruled): a never-existed id is
    /// a TYPO, refused at the best-diagnostics door. (A later
    /// `DeleteNode` stranding a name is ALLOWED — N5 dangling
    /// semantics; see [`Node::Declare`].)
    DeclareNamesMissingNode {
        /// The name whose node is not live.
        name: StableName,
    },
    /// A reference's READ SITE — the operand a mate is authored
    /// against ([`Node::payload_read_sites`]) — names a node that does
    /// not exist at edit time. The name half's rule, applied to the
    /// half that is a node id rather than a name: a never-existed id
    /// is a TYPO. (A later `DeleteNode` stranding an operand is
    /// ALLOWED — N5 dangling semantics — and the solve refuses typed.)
    ReadSiteMissingNode {
        /// The operand that is not live.
        at: RecipeNodeId,
    },
    /// A non-finite (NaN/inf) float on a continuous doc param — its
    /// value or one of its distribution's offsets — refused at the
    /// edit door (ruled door 1 of the non-finite policy; F3's
    /// persist-time refusal then has nothing to catch).
    NonFiniteDocParam {
        /// The parameter.
        name: ParamName,
    },
    /// A doc param's distribution breaks an E2 invariant other than
    /// finiteness: `sigma > 0`, or bounds containing the nominal.
    /// The SAME check the persistence doors run, so a document that
    /// would refuse to load cannot be authored.
    InvalidDistribution {
        /// The parameter.
        name: ParamName,
        /// The invariant that failed.
        fault: DistributionFault,
    },
    /// A `Rebind` whose target name's node is not live (the selection
    /// must denote something the recipe still has — best-diagnostics
    /// door, mirrors the Declare carve-out).
    RebindTargetMissingNode {
        /// The target name whose node is gone.
        name: StableName,
    },
    /// A `Rebind` whose SOURCE name's node was never minted by this
    /// document (ids are monotone and never reused, so an id at or
    /// above the mint counter is a typo or a foreign name — refused;
    /// a deleted-but-once-lived node is ALLOWED, that is the
    /// `NodeGone` repair case).
    RebindUnknownName {
        /// The foreign source name.
        name: StableName,
    },
    /// A `Rebind` across entity kinds (a face reference cannot come
    /// to denote an edge — the reference's kind is part of its type).
    RebindKindMismatch {
        /// The source name's kind.
        from: EntityKind,
        /// The target name's kind.
        to: EntityKind,
    },
    /// A `Rebind` from a name to itself — a recorded no-op is noise,
    /// refused loudly.
    RebindIdentity {
        /// The name.
        name: StableName,
    },
    /// A `Rebind` whose source name no document site references:
    /// there is nothing to repair (GUI selection state is not
    /// document state — repairing a selection is re-selecting).
    RebindNoReferences {
        /// The unreferenced source name.
        name: StableName,
    },
    /// A `ReWitness` aimed at a node that is not sketch-bearing (the
    /// witness datum is GQ1's per-sketch-node branch selection;
    /// Profile is the v1 sketch-bearing node kind).
    WitnessOnNonSketch {
        /// The non-sketch node.
        node: RecipeNodeId,
    },
    /// A `ReWitnessBulk` with the same node listed twice (which entry
    /// wins would be positional — refused).
    DuplicateWitnessEntry {
        /// The duplicated node.
        node: RecipeNodeId,
    },
    /// A `ReWitnessBulk` with no entries — a recorded no-op, refused.
    EmptyWitnessBulk,
    /// Name-level edit-time validation (M4 PR 4, the PR 3 R6 banked
    /// obligation, via [`crate::resolve::apply_with_names`]): the
    /// name's minting node HAS an Ok value in the supplied
    /// evaluation, yet no table carries the name — recording the
    /// reference would strand it immediately. The forward-reference
    /// carve-out stands: names whose nodes are unevaluated (or
    /// failed/poisoned) in the supplied evaluation are not checkable
    /// and pass through to evaluation-time resolution.
    NameUnresolvedInEvaluation {
        /// The name no table carries.
        name: StableName,
    },
    /// The supplied evaluation is of ANOTHER document (DI3, A2a).
    ///
    /// Raised by [`crate::resolve::apply_with_names`], whose docs say
    /// why that door checks; this arm is the edit vocabulary's word
    /// for the answer, as `ProductError`, `MateFault` and
    /// `ChecksError` each carry their own over the one predicate
    /// [`crate::ident::mispaired`].
    EvaluationOfAnotherDocument {
        /// The document the edit is being applied to.
        expected: crate::ident::DocumentId,
        /// The document the supplied evaluation is of.
        found: crate::ident::DocumentId,
    },
    /// A `Rebind` whose appearance-key rewrite would land two
    /// attributes of the same kind on the target name (`from`'s
    /// attribute set collides with one already attached to `to`).
    /// Which value survives would be an auto-pick — refused loudly;
    /// the repair is an explicit `ClearAppearance` on either side
    /// first (fail-loud charter, the `ClearAppearance` loud-no-op
    /// precedent).
    RebindAppearanceCollision {
        /// The target name that already carries the kind.
        name: StableName,
        /// The colliding attribute kind.
        kind: AttrKind,
    },
    /// A `SetAppearance` on an edge or vertex name — v1 appearance is
    /// per-face/per-body (M4-PLAN item 7); edge/vertex attributes are
    /// a future additive extension, refused typed until ratified.
    AppearanceWrongKind {
        /// The refused name.
        name: StableName,
    },
    /// A `SetAppearance` naming a node that is not live at edit time
    /// (the Declare-parallel carve-out: a never-existed id is a typo;
    /// see [`DocEdit::SetAppearance`]).
    AppearanceNamesMissingNode {
        /// The name whose node is not live.
        name: StableName,
    },
    /// A `ClearAppearance` for an attribute that is not set — loud
    /// no-ops per the fail-loud charter.
    AppearanceNotSet {
        /// The name.
        name: StableName,
        /// The kind that was not set on it.
        kind: AttrKind,
    },
    /// A `SetTolerance` whose ε is not finite and strictly positive.
    InvalidTolerance {
        /// The refused value.
        value: f64,
    },
    /// A `SetAppearanceMeta` value violating the D7 producer
    /// convention (a map carrying an integer `"v"` version field).
    MetaUnversioned {
        /// The name.
        name: StableName,
        /// The metadata key.
        key: String,
        /// The typed shape refusal.
        error: MetaVersionError,
    },
    /// A `SetAppearanceMeta` value carrying a non-finite float (D2:
    /// refused at the edit door, never stored).
    MetaNonFinite {
        /// The name.
        name: StableName,
        /// The metadata key.
        key: String,
        /// Path of the offending float within the value tree.
        path: String,
    },
    /// A `ClearAppearanceMeta` for a key that is not set — loud
    /// no-ops per the fail-loud charter.
    MetaNotSet {
        /// The name.
        name: StableName,
        /// The key that was not set on it.
        key: String,
    },
    /// A `Rebind` whose appearance-record move would land two
    /// metadata values under one key on the target name (the D7 twin
    /// of [`EditError::RebindAppearanceCollision`]). Which value
    /// survives would be an auto-pick — refused loudly; the repair is
    /// an explicit `ClearAppearanceMeta` on either side first.
    RebindMetadataCollision {
        /// The target name that already carries the key.
        name: StableName,
        /// The colliding metadata key.
        key: String,
    },
    /// The edit's result would violate a product-root invariant (A10;
    /// ASM-ROOTS D-2). Reached from `SetRoots` in practice — the
    /// automatic maintenance keeps every other arm's result legal —
    /// but checked after EVERY apply, so no door can produce an
    /// invariant-violating document.
    Roots(RootFault),
    /// A placement aimed at a node that does not instantiate a part
    /// (A11: a placement frame places a CLUSTER of instances, and
    /// nothing else has one).
    PlacementOnNonInstance {
        /// The offending target.
        node: RecipeNodeId,
    },
    /// A placement-rule node whose rule and count slot would give two
    /// answers to "how many placements" (GROUP-BOOLEAN-DESIGN): an
    /// `Explicit` rule paired with a count slot, a stepped rule with
    /// none — or a `Pattern` carrying an `Explicit` rule at all, since
    /// its count is a non-optional field.
    PlacementRuleMismatch {
        /// The offending node.
        node: RecipeNodeId,
    },
    /// A placement-rule node whose `Explicit` rule lists NO placements
    /// (GROUP-BOOLEAN-DESIGN): the list IS the count, so an empty one
    /// is the explicit rule's `count < 1` — refused for the reason a
    /// stepped rule's zero is, rather than quietly denoting an empty
    /// body.
    EmptyPlacementList {
        /// The offending node.
        node: RecipeNodeId,
    },
    /// An IMPROPER placement frame — determinant ≤ 0, i.e. a mirror
    /// (A6). Admitting one is gated on the equivariance audit R4 owns:
    /// until that lands, a mirrored placement is refused rather than
    /// silently trusted to leave every orientation-sensitive predicate
    /// and every outward normal intact.
    ImproperPlacement {
        /// The offending target.
        node: RecipeNodeId,
        /// The linear part's determinant.
        determinant: f64,
    },
    /// A placement frame carrying a non-finite coordinate.
    NonFinitePlacement {
        /// The offending target.
        node: RecipeNodeId,
    },
    /// **A placement frame's ROTATION AXIS has no definite
    /// direction** — the [`crate::AxisRefusal`]
    /// [`crate::Frame::rotate_then_translate`] raises where the axis
    /// is decided, carried into this vocabulary unaltered so an
    /// author building a frame and setting it speaks ONE error type
    /// from the constructor through the door.
    ///
    /// Its own arm rather than [`EditError::NonFinitePlacement`]:
    /// that one's subject is the frame's coordinates, and a reader
    /// told their frame is not finite goes looking at the frame,
    /// which is exactly the mistaken subject this arm exists to stop
    /// reporting. The cause is the axis, in the evaluation layer's
    /// own words, with the role word naming the vector.
    PlacementAxis {
        /// The direction door's refusal, unaltered.
        error: crate::eval::NodeRefusal,
    },
    /// A mate's alignment datum carries a non-finite coordinate. The
    /// placement registry's own rule, one level out: an authored frame
    /// nothing can decide about never enters the document.
    NonFiniteAlignment {
        /// The mate being inserted.
        node: RecipeNodeId,
    },
    /// A pin update aimed at a node that does not instantiate a part
    /// (A13; ASM-UPD D-1 — the [`EditError::PlacementOnNonInstance`]
    /// precedent: only a cross-document reference HAS a version).
    UpdateOnNonInstance {
        /// The offending target.
        node: RecipeNodeId,
    },
    /// A pin update whose new pin is the one the reference already
    /// names. The edit would record a step that changes nothing —
    /// refused rather than written, so a log's presence of an update
    /// always means a version actually moved (ASM-UPD D-1's fail-loud
    /// rule).
    PinUnchanged {
        /// The reference that already names this pin.
        node: RecipeNodeId,
        /// The pin both sides carry.
        pin: crate::ident::ContentPin,
    },
}

/// **The pairing predicate's finding, in this door's vocabulary.**
///
/// A2a's rule is one predicate (`ident::mispaired`) and one arm per
/// error type over it. The projection lives HERE, at the type that
/// owns the arm, so a door that runs the predicate writes `?` or
/// `m.into()` and no site re-spells which field goes where.
impl From<crate::ident::Mispaired> for EditError {
    fn from(m: crate::ident::Mispaired) -> Self {
        Self::EvaluationOfAnotherDocument {
            expected: m.expected,
            found: m.found,
        }
    }
}

// LIB-DOORS F6 (reopened on review): the human-readable rendering the
// bindings' exception messages consume. The comment-style rule
// applies — each arm states the PROBLEM (and where it is), not the
// enum's guts; stable names, kinds and dimensions render through their
// own prose spellings (`StableName`'s `Display`, the `noun`
// renderings, `Dimension`'s `Display`), never `Debug`. A name is
// parenthesized apposition when the sentence's subject is a role word
// ("the rebind target ({name})") and inline when the name itself is
// the subject ("the {name} does not resolve"); new arms copy whichever
// their sentence shape calls for. The typed variant remains the
// machine contract.
//
// **No category prefix, and parameter names render bare.** These
// sentences are read verbatim by a person: the viewer's status line
// composes them under its own frame ("the edit was refused: …") and
// the bindings raise them as an exception message whose class already
// says which door refused. An `edit: ` opening was therefore a second
// spelling of the caller's own frame, and a `{:?}` name arrived
// double-quoted inside prose that quotes nothing else — a dump in the
// middle of a sentence. Both are gone: the frame belongs to whoever
// received the refusal, and a name is written unquoted.
//
// **That makes `EditError` the exception in this crate, not the rule,
// and the exception is deliberate.** Its neighbours still open with a
// category of their own — `persist:`, `split:`, `inline:`, `parse:`,
// `product:` — and `refactor.rs` quotes a parameter name exactly the
// way this impl used to. They are outside the amendment that changed
// this one, so they keep their spelling until someone decides for
// them; a reader comparing the two should not read this paragraph as
// describing the crate. The SLOT id renders through `SlotId::label`
// for the same reason a name does: a variant identifier dropped into a
// sentence is the `Debug` dump's fingerprint, and the slot vocabulary
// has one prose spelling of its own, so a message reads "slot origin x"
// rather than "slot Origin(X)".
/// The AXIS's refusal, in the authoring vocabulary — what makes
/// `Frame::rotate_then_translate(..)?` compose with
/// `apply(.., DocEdit::SetPlacement { .. })?` in one function.
///
/// It converts from [`crate::AxisRefusal`] and from nothing else. A
/// blanket `From<NodeErrorKind>` would make every node refusal in the
/// crate convert into this arm through a bare `?`, which is a
/// catch-all in the authoring vocabulary — the shape this arm was
/// added to close.
impl From<crate::AxisRefusal> for EditError {
    fn from(error: crate::AxisRefusal) -> Self {
        Self::PlacementAxis {
            error: error.carried(),
        }
    }
}

impl core::fmt::Display for EditError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownNode { id } => write!(f, "node {} is not live", id.0),
            Self::ProfileProgramRefused { node, refusal } => {
                write!(f, "node {}'s profile program refused: {refusal}", node.0)
            }
            Self::UnresolvedInput { input } => {
                write!(f, "input {} does not resolve to a live node", input.0)
            }
            Self::WouldCycle { at } => {
                write!(f, "the recipe graph would cycle (through node {})", at.0)
            }
            // Forwarded, not restated: `InputFault` owns this
            // vocabulary and `node.rs` promises every door that renders
            // it forwards. The door adds its own frame — which edit the
            // fault is about — and joins it with a colon, because the
            // forwarded sentence carries an em-dash of its own and two
            // in a row read as a dump.
            //
            // **The frame does not name `node`, and must not.**
            // `check_node_inputs` is reached from `InsertNode` with
            // `RecipeNodeId(new.next_id)` — an id that does not exist
            // and never will if the edit is refused — and from
            // `SetMembers` with a live one. This rendering cannot tell
            // which, so a sentence naming that id tells a person to go
            // and look at a node that may be a phantom. The id the
            // reader CAN act on is `input`, which the forwarded fault
            // names, and it is live on both paths.
            //
            // The action is the door's to add: `InputFault` states the
            // rule ("pairwise distinct"), which says what is wrong and
            // not what to do about it. `TooFew` needs no such clause —
            // its own sentence carries the count that is required.
            Self::DuplicateInput { input, .. } => write!(
                f,
                "the node this edit writes would be invalid: {}. Replace one of the two with a \
                 different node.",
                crate::node::InputFault::Duplicate { input: *input }
            ),
            Self::SetMembersOnNonList { node } => write!(
                f,
                "node {} carries no list input, so it has no members to set",
                node.0
            ),
            Self::TooFewMembers { found, .. } => write!(
                f,
                "the node this edit writes would be invalid: {}",
                crate::node::InputFault::TooFew { found: *found }
            ),
            Self::RepeatedDesignation { first, again, .. } => write!(
                f,
                "the node this edit writes would be invalid: {}. Build it through `Node::shell`, \
                 which keeps the first occurrence.",
                crate::node::InputFault::RepeatedDesignation {
                    first: *first,
                    again: *again,
                }
            ),
            Self::SelectionNotCanonical { at, .. } => write!(
                f,
                "the node this edit writes would be invalid: {}. Build it through \
                 `Node::fillet` or `Node::chamfer`, which sort and deduplicate.",
                crate::node::InputFault::SelectionNotCanonical { at: *at }
            ),
            Self::DeleteWouldDangle { id, referenced_by } => write!(
                f,
                "node {} is still an input to node {} — delete node {} first, \
                 or delete node {} together with everything downstream of it",
                id.0, referenced_by.0, referenced_by.0, id.0
            ),
            Self::UnknownSlot { id, slot } => {
                write!(f, "node {} has no slot {}", id.0, slot.label())
            }
            Self::SlotDimensionMismatch {
                slot,
                expected,
                found,
            } => write!(
                f,
                "slot {} needs {} {expected} expression, got {} {found}",
                slot.label(),
                expected.article(),
                found.article()
            ),
            Self::StructuralSlotNeedsStructuralEdit { slot } => {
                write!(
                    f,
                    "slot {} is structural — use a structural edit",
                    slot.label()
                )
            }
            Self::NotStructuralSlot { slot } => {
                write!(f, "slot {} is continuous, not structural", slot.label())
            }
            Self::UnknownPayloadParam { name, node } => write!(
                f,
                "document parameter {} does not exist (referenced by node {}'s \
                 measurement payload)",
                name.0, node.0
            ),
            Self::PayloadParamDimensionMismatch {
                name,
                node,
                declared,
                referenced,
            } => write!(
                f,
                "document parameter {} is declared {declared} but node {}'s \
                 measurement payload references it as {referenced}",
                name.0, node.0
            ),
            Self::MeasureMalformed { node, fault } => {
                write!(f, "measure node {}: {fault}", node.0)
            }
            Self::AssertionTarget { node, measure } => write!(
                f,
                "assertion node {} references node {}, which is not a measure — an \
                 assertion constrains a measurement",
                node.0, measure.0
            ),
            Self::DeclareInputNotDeclare { node, input } => write!(
                f,
                "node {}'s declare input names node {}, which is not a declaration — \
                 wire a Declare node there, or leave the input empty",
                node.0, input.0
            ),
            Self::AssertionDimension {
                node,
                measure,
                measured,
                bound,
            } => write!(
                f,
                "assertion node {} bounds {} {measured} measure (node {}) with {} \
                 {bound} expression — an assertion compares like with like or not at all",
                node.0,
                measured.article(),
                measure.0,
                bound.article()
            ),
            Self::UnknownDocParam { name, node, slot } => write!(
                f,
                "document parameter {} does not exist (referenced by node {}, slot {})",
                name.0,
                node.0,
                slot.label()
            ),
            Self::DocParamDimensionMismatch {
                name,
                node,
                slot,
                declared,
                referenced,
            } => write!(
                f,
                "parameter {} is declared {declared} but node {} (slot {}) references it as {referenced}",
                name.0,
                node.0,
                slot.label()
            ),
            Self::ContinuousParamCannotBeCount { name } => write!(
                f,
                "parameter {}: a continuous parameter cannot be a count — use a count parameter",
                name.0
            ),
            // The closing clause is also `Refusal::NoSuchParam`'s, in
            // the viewer: one mistake reaches this door by typing and
            // that lookup by dragging, and the two are converged on the
            // RECOURSE rather than on the sentence. A viewer test holds
            // them in step (`panel_edits::refusals_render_as_sentences`).
            Self::DocParamNotDeclared { name, door } => write!(
                f,
                "parameter {} is not declared, so {door} has no declaration to carry \
                 forward — declare it first",
                name.0
            ),
            Self::DocParamCountHasNoUnit { name } => write!(
                f,
                "parameter {} is a count, and a count is an integer rather than a quantity — \
                 it has no display unit to change",
                name.0
            ),
            Self::DocParamUnitMismatch {
                name,
                unit,
                declared,
            } => write!(
                f,
                "parameter {} is declared {declared} but the display unit offered measures \
                 {unit}",
                name.0
            ),
            Self::DocParamValueKindMismatch {
                name,
                declared,
                offered,
            } => write!(
                f,
                "parameter {} is declared {declared} but the value edit offered a \
                 {offered} — changing a parameter's kind is a redeclaration",
                name.0
            ),
            Self::PathOffTree { path } => {
                write!(f, "expression path {path:?} runs off the tree")
            }
            Self::Dimension(e) => write!(f, "{e}"),
            Self::DeclareNamesMissingNode { name } => {
                write!(f, "the declared {name} refers to a node that is not live")
            }
            Self::ReadSiteMissingNode { at } => write!(
                f,
                "the reference is read at node {}, which is not live",
                at.0
            ),
            Self::NonFiniteDocParam { name } => write!(
                f,
                "parameter {}: the value and every distribution offset must be finite",
                name.0
            ),
            Self::InvalidDistribution { name, fault } => {
                write!(f, "parameter {}: {fault}", name.0)
            }
            Self::RebindTargetMissingNode { name } => write!(
                f,
                "the rebind target ({name}) refers to a node that is not live"
            ),
            Self::RebindUnknownName { name } => write!(
                f,
                "the rebind source ({name}) was never minted by this document"
            ),
            Self::RebindKindMismatch { from, to } => write!(
                f,
                "a rebind cannot cross entity kinds ({} to {})",
                from.noun(),
                to.noun()
            ),
            Self::RebindIdentity { name } => write!(
                f,
                "rebinding the {name} to itself is a recorded no-op — refused"
            ),
            Self::RebindNoReferences { name } => write!(
                f,
                "no document site references the {name} — nothing to repair"
            ),
            Self::WitnessOnNonSketch { node } => write!(
                f,
                "node {} is not sketch-bearing — nothing to re-witness",
                node.0
            ),
            Self::DuplicateWitnessEntry { node } => {
                write!(f, "node {} appears twice in the re-witness bulk", node.0)
            }
            Self::EmptyWitnessBulk => {
                f.write_str("a re-witness bulk with no entries is a no-op — refused")
            }
            Self::NameUnresolvedInEvaluation { name } => write!(
                f,
                "the {name} does not resolve in the supplied evaluation — recording the \
                 reference would strand it"
            ),
            Self::EvaluationOfAnotherDocument { expected, found } => write!(
                f,
                "the supplied evaluation is of document {found}, not of document \
                 {expected} — its names would be checked against another document's \
                 tables"
            ),
            Self::RebindAppearanceCollision { name, kind } => write!(
                f,
                "the rebind would land two {} attributes on the {name} — clear one first",
                kind.noun()
            ),
            Self::AppearanceWrongKind { name } => write!(
                f,
                "appearance attaches to faces and bodies only (refused for the {name})"
            ),
            Self::AppearanceNamesMissingNode { name } => write!(
                f,
                "the appearance target ({name}) refers to a node that is not live"
            ),
            Self::AppearanceNotSet { name, kind } => {
                write!(f, "no {} attribute is set on the {name}", kind.noun())
            }
            Self::InvalidTolerance { value } => {
                write!(f, "tolerance {value:e} is not finite and strictly positive")
            }
            Self::MetaUnversioned { name, key, error } => write!(
                f,
                "metadata {key:?} on the {name} does not carry the D7 integer \"v\" \
                 version field: {error}"
            ),
            Self::MetaNonFinite { name, key, path } => write!(
                f,
                "metadata {key:?} on the {name} carries a non-finite float at {path}"
            ),
            Self::MetaNotSet { name, key } => {
                write!(f, "no metadata {key:?} is set on the {name}")
            }
            Self::RebindMetadataCollision { name, key } => write!(
                f,
                "the rebind would land two values under metadata {key:?} on the {name} — \
                 clear one first"
            ),
            Self::Roots(fault) => write!(f, "{fault}"),
            Self::PlacementOnNonInstance { node } => write!(
                f,
                "node {} does not instantiate a part, so it has no placement cluster to \
                 place",
                node.0
            ),
            // The two rule-shaped arms FORWARD the fault set's one
            // prose vocabulary (`PlacementRuleFault`'s `Display`); the
            // two frame-shaped arms below keep their own prose because
            // their subject is a single cluster frame, which has no
            // index in a rule's placement list.
            Self::EmptyPlacementList { node } => {
                write!(f, "node {}: {}", node.0, PlacementRuleFault::NoPlacements)
            }
            Self::PlacementRuleMismatch { node } => {
                write!(f, "node {}: {}", node.0, PlacementRuleFault::CountSpelling)
            }
            Self::ImproperPlacement { node, determinant } => write!(
                f,
                "the placement frame for node {} is improper (determinant {determinant}); \
                 mirrored placements are admitted only behind the equivariance audit",
                node.0
            ),
            Self::PlacementAxis { error } => {
                write!(
                    f,
                    "the placement frame's rotation axis is unusable: {error}"
                )
            }
            Self::NonFinitePlacement { node } => write!(
                f,
                "the placement frame for node {} carries a non-finite coordinate",
                node.0
            ),
            Self::NonFiniteAlignment { node } => write!(
                f,
                "the mate at node {} carries a non-finite alignment coordinate",
                node.0
            ),
            Self::UpdateOnNonInstance { node } => write!(
                f,
                "node {} does not instantiate a part, so it has no pinned version to update",
                node.0
            ),
            Self::PinUnchanged { node, pin } => write!(
                f,
                "node {} already pins {pin}, so this update would record no version move",
                node.0
            ),
        }
    }
}

impl core::error::Error for EditError {}

/// What an accepted edit did (spec D6: structural edits are FLAGGED
/// in the returned record; the record also returns the minted id —
/// without it a caller could never reference an inserted node).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditRecord {
    /// The id minted by an `InsertNode`, `None` otherwise.
    pub minted: Option<RecipeNodeId>,
    /// Whether the edit was STRUCTURAL (spec D3/D6): it can change
    /// the result's combinatorial shape — insert/delete, a
    /// Count-slot expression edit, or a Count doc-param set.
    /// Continuous edits (`SetParam`, continuous-slot `SetExpression`,
    /// continuous `SetDocParam`) leave recipe structure fixed.
    pub structural: bool,
}

/// One act of **automatic maintenance** an accepted edit performed:
/// bookkeeping the edit forced, or a consequence it left behind, that
/// the caller never asked for by name.
///
/// Every act rides the accepted edit rather than being a second edit
/// of its own — the A10 root-list precedent, verbatim: maintenance is
/// deterministic from the edit, so a replay reproduces it and undo
/// (keeping the prior document value) restores it exactly. What the
/// record adds is VISIBILITY, at the door where the consequence
/// happened rather than at the next evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum Maintenance {
    /// **The A11 cluster-record maintenance** (ASM-R2a D-3): the
    /// joins, splits, gauge rewrites and drops the mate graph's
    /// motion forced on the placement registry. An absorbed cluster's
    /// frame is CONSUMED into the record, where a caller can read
    /// what was consumed.
    Cluster(crate::mate::ClusterMaintenance),
    /// **A payload name this edit stranded** (DM7): `node` survives
    /// and carries `name`, whose minting node the edit deleted.
    ///
    /// The name still says exactly what it always said; what is gone
    /// is the node that minted it, so evaluation answers
    /// [`crate::resolve::ResolveError::NodeGone`] — rung 1 of the N5
    /// ladder — and [`DocEdit::Rebind`] is the repair. A name is not
    /// a DAG edge (the D3 carve-out), so the delete is legal: this
    /// row is what the door owes instead of a refusal.
    ///
    /// The deleted minting node is `name.node` and is not repeated as
    /// a field of its own: a second copy is a disagreement waiting to
    /// happen.
    Strand {
        /// The surviving node whose payload carries the name.
        node: RecipeNodeId,
        /// The name it carries. Its `node` is the id this edit
        /// deleted.
        name: StableName,
    },
}

impl core::fmt::Display for Maintenance {
    /// The cluster arm DELEGATES: a registry act's sentence belongs to
    /// the type that knows what the act is, so each enum renders its
    /// own arms and the F6 census guards each list where it lives.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Cluster(act) => write!(f, "{act}"),
            // The relative clause binds to the NODE, not to the name:
            // "a face name minted by node 7, which this edit deleted"
            // reads as though the name were deleted, and the name is
            // exactly what survives.
            Self::Strand { node, name } => write!(
                f,
                "node {} carries a {}; this edit deleted node {}, so the name resolves to \
                 nothing until it is rebound",
                node.0, name, name.node.0
            ),
        }
    }
}

/// **DM7's report**: every payload name a `DeleteNode` just stranded —
/// one row per `(carrier, name)` whose minting node is `deleted`.
///
/// `doc` is the document AFTER the removal, so the nodes walked are
/// exactly the survivors and a name that left with its own carrier is
/// not reported: nothing is stranded when nothing is left to carry it.
/// Rows come in document order, and within one node in
/// [`Node::payload_names`]' order, which is meaning for the ordered
/// payloads (a shell's rim, a measure's arguments).
///
/// The walk is [`Node::payload_names`] — the same single answer to
/// "which payloads carry a name" that the insert door checks with, so
/// a payload kind cannot be live at one door and invisible at the
/// other.
///
/// [`Node::payload_read_sites`] — a mate's two operands — are NOT
/// here. A read site is a node id rather than a name: no N5 ladder
/// resolves it and `Rebind` cannot repair it, so a delete that strands
/// one is the solve's to refuse (A12), not this door's to report.
///
/// **Cost.** One pass over the document's payload names per accepted
/// delete, so a cascade of `n` nodes pays `n` passes. That is the
/// price of reporting at the door rather than once at the end, and it
/// is what makes the rows TRUE of the document each step produced;
/// `cascade_delete_order` is a walk of the same shape already, and a
/// caller who wants one number for the whole cascade computes it from
/// the doomed set instead of from these rows (the transients cancel —
/// `rv_a_cascade_reports_strands_on_carriers_it_then_deletes`).
fn stranded_names<P>(doc: &Doc<P>, deleted: RecipeNodeId) -> Vec<Maintenance> {
    let mut out = Vec::new();
    for &id in doc.order() {
        let Some(node) = doc.node(id) else { continue };
        for name in node.payload_names() {
            if name.node == deleted {
                out.push(Maintenance::Strand {
                    node: id,
                    name: name.clone(),
                });
            }
        }
    }
    out
}

/// An accepted edit: the NEW document (the input untouched, spec D2)
/// plus the [`EditRecord`].
#[derive(Debug, Clone, PartialEq)]
pub struct Applied<P> {
    /// The new document value.
    pub doc: Doc<P>,
    /// What the edit did.
    pub record: EditRecord,
    /// **What the edit did that the caller did not ask for**: the A11
    /// cluster-record maintenance it forced, and the payload names it
    /// stranded (DM7). See [`Maintenance`].
    ///
    /// **The order is a CONTRACT, not an accident of the
    /// implementation**: the strands come first — read at the door,
    /// out of the document the edit had just produced — and the
    /// cluster acts follow, reconciling the registry against it
    /// afterwards. A consumer may rely on that; it is pinned by
    /// `dm7_delete_strands::a_mates_head_strands_and_its_read_site_does_not`,
    /// the one edit that produces both kinds at once. What a consumer
    /// may NOT do is read position 0 as a kind: a delete that strands
    /// nothing puts a cluster act there, so an arm is found by
    /// matching, never by index.
    pub maintenance: Vec<Maintenance>,
}

/// Validate one expression's document-parameter refs against the
/// param table (spec D6: dimension checks re-run on touched
/// expressions; `node`/`slot` locate the expression for the error).
fn check_param_refs<P>(
    doc: &Doc<P>,
    node: RecipeNodeId,
    slot: SlotId,
    expr: &Expr,
) -> Result<(), EditError> {
    let mut refs = Vec::new();
    expr.param_refs(&mut refs);
    for (name, referenced) in refs {
        match doc.params().get(&name) {
            None => return Err(EditError::UnknownDocParam { name, node, slot }),
            Some(p) if p.dim() != referenced => {
                return Err(EditError::DocParamDimensionMismatch {
                    name,
                    node,
                    slot,
                    declared: p.dim(),
                    referenced,
                });
            }
            Some(_) => {}
        }
    }
    Ok(())
}

/// Validate every slot of a node payload against slot dimensions and
/// the param table, keyed as `id` for error reporting.
/// Write a fully-formed [`DocParam`] into the document: the shared
/// tail of both parameter doors, so the create-or-replace door and the
/// value door cannot come to disagree about what a legal parameter is.
///
/// **The check order is the LOAD door's** (`persist::check`'s
/// `validate_document`): floats first, then the distribution's shape,
/// then the structural `dim: Count` fault that
/// [`validate_snapshot`](crate::persist) reports last. A document
/// broken in two ways at once therefore names the same fault whichever
/// door refuses it, which is the property a caller comparing an edit
/// refusal against a load refusal actually relies on.
fn write_doc_param<P: Clone + crate::ProfilePayload>(
    new: &mut Doc<P>,
    name: &ParamName,
    value: DocParam,
) -> Result<EditRecord, EditError> {
    // Ruled door 1 (non-finite policy): recipe data never carries
    // NaN/inf — the nominal and the distribution offsets alike.
    if let DocParam::Continuous { value: v, .. } = value
        && !v.is_finite()
    {
        return Err(EditError::NonFiniteDocParam { name: name.clone() });
    }
    // Every E2 invariant, from the ONE shared check the persistence
    // doors also run: a non-finite offset joins the non-finite class
    // above, the rest refuse as a distribution fault.
    if let Some(d) = value.distribution()
        && let Err(fault) = d.check()
    {
        return Err(match fault {
            DistributionFault::NonFinite { .. } => {
                EditError::NonFiniteDocParam { name: name.clone() }
            }
            DistributionFault::SigmaNotPositive { .. }
            | DistributionFault::NominalOutsideSupport { .. } => EditError::InvalidDistribution {
                name: name.clone(),
                fault,
            },
        });
    }
    if let DocParam::Continuous {
        dim: Dimension::Count,
        ..
    } = value
    {
        return Err(EditError::ContinuousParamCannotBeCount { name: name.clone() });
    }
    // The unit/dimension pairing, at EVERY door that writes a
    // declaration rather than only at the one that writes a notation.
    // This is the create-or-replace door, whose `DocParam` a caller
    // assembles out of a `pub` payload, so it is the one door that can
    // state a mismatched pair — and before this check the only thing
    // that refused it was save/load, which meant an in-memory document
    // could hold a parameter no file could ever carry. `measures()` is
    // the same predicate the notation door and the validator ask.
    if let DocParam::Continuous {
        dim, display_unit, ..
    } = value
    {
        let measured = display_unit.measures();
        if measured != dim {
            return Err(EditError::DocParamUnitMismatch {
                name: name.clone(),
                unit: measured,
                declared: dim,
            });
        }
    }
    let structural = matches!(value, DocParam::Count { .. });
    new.params.insert(name.clone(), value);
    // A (re)declaration can change the dimension out from under
    // referencing expressions: re-validate every slot (documents are
    // small; spec D6's re-run requirement).
    for &id in &new.order {
        if let Some(node) = new.nodes.get(&id) {
            check_node_slots(new, id, node)?;
        }
    }
    Ok(EditRecord {
        minted: None,
        structural,
    })
}

fn check_node_slots<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    id: RecipeNodeId,
    node: &Node<P>,
) -> Result<(), EditError> {
    for slot in node.slots() {
        // slots() and expr() agree by construction; a miss here is a
        // vocabulary bug, surfaced as UnknownSlot rather than hidden.
        let Some(expr) = node.expr(slot) else {
            return Err(EditError::UnknownSlot { id, slot });
        };
        if expr.dim() != slot.dimension() {
            return Err(EditError::SlotDimensionMismatch {
                slot,
                expected: slot.dimension(),
                found: expr.dim(),
            });
        }
        check_param_refs(doc, id, slot, expr)?;
    }
    // The expressions no slot addresses (E3/E10). Their DIMENSIONS are
    // already fixed by construction — a `MeasureExpr` runs the F1
    // checker at every constructor, and an assertion's bound is checked
    // against its measure below — so what is left here is the same
    // parameter-table re-check every slot expression gets.
    for expr in crate::node::payload_exprs(node).into_iter().flatten() {
        let mut refs = Vec::new();
        expr.param_refs(&mut refs);
        for (name, referenced) in refs {
            match doc.params().get(&name) {
                None => return Err(EditError::UnknownPayloadParam { name, node: id }),
                Some(p) if p.dim() != referenced => {
                    return Err(EditError::PayloadParamDimensionMismatch {
                        name,
                        node: id,
                        declared: p.dim(),
                        referenced,
                    });
                }
                Some(_) => {}
            }
        }
    }
    // A measured expression's reference indices, at the edit door as
    // well as the construction and load doors: `Node::Measure` is a
    // public variant, so a hand-built value can reach `apply` without
    // passing `Node::measure`.
    if let Some(fault) = node.measure_fault() {
        return Err(EditError::MeasureMalformed { node: id, fault });
    }
    // An assertion's bound against the dimension of the measure it
    // constrains — the one check that needs the DOCUMENT, which is why
    // it lands here and not on the node.
    if let Node::Assertion { measure, bound, .. } = node {
        let measured = match doc.node(*measure) {
            Some(Node::Measure { expr, .. }) => expr.dim(),
            _ => {
                return Err(EditError::AssertionTarget {
                    node: id,
                    measure: *measure,
                });
            }
        };
        if measured != bound.dim() {
            return Err(EditError::AssertionDimension {
                node: id,
                measure: *measure,
                measured,
                bound: bound.dim(),
            });
        }
    }
    Ok(())
}

/// DM5 at an EDIT door: [`Node::input_fault`] rendered in this
/// module's vocabulary. The rule itself lives on the node, where the
/// load door reads it too; this is the door's name for its answer, and
/// there is no second copy of the question.
///
/// Liveness is the caller's, and is checked before this, so a list of
/// dangling ids reports the dangling id rather than a count.
fn check_node_inputs<P: crate::ProfilePayload>(
    id: RecipeNodeId,
    node: &Node<P>,
) -> Result<(), EditError> {
    match node.input_fault() {
        None => Ok(()),
        Some(crate::node::InputFault::Duplicate { input }) => {
            Err(EditError::DuplicateInput { node: id, input })
        }
        Some(crate::node::InputFault::TooFew { found }) => {
            Err(EditError::TooFewMembers { node: id, found })
        }
        Some(crate::node::InputFault::RepeatedDesignation { first, again }) => {
            Err(EditError::RepeatedDesignation {
                node: id,
                first,
                again,
            })
        }
        Some(crate::node::InputFault::SelectionNotCanonical { at }) => {
            Err(EditError::SelectionNotCanonical { node: id, at })
        }
    }
}

/// The `declare` edge's kind rule, in this door's vocabulary.
///
/// The rule itself is [`Node::bad_declare_input`], asked by this door
/// and by the load door (`persist::check`) of one answer; what is here
/// is only this door's word for the refusal.
fn check_declare_input<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    id: RecipeNodeId,
    node: &Node<P>,
) -> Result<(), EditError> {
    match node.bad_declare_input(doc) {
        Some(input) => Err(EditError::DeclareInputNotDeclare { node: id, input }),
        None => Ok(()),
    }
}

/// Reject cycles in the recipe DAG (spec D3/D6). Defensive: insertion
/// referencing only existing nodes cannot cycle, but the invariant is
/// checked. Iterative DFS, three-color, deterministic order.
fn check_acyclic<P: crate::ProfilePayload>(doc: &Doc<P>) -> Result<(), EditError> {
    use std::collections::BTreeMap;
    #[derive(Clone, Copy, PartialEq)]
    enum Color {
        White,
        Grey,
        Black,
    }
    let mut color: BTreeMap<RecipeNodeId, Color> =
        doc.order().iter().map(|&id| (id, Color::White)).collect();
    for &root in doc.order() {
        if color.get(&root) != Some(&Color::White) {
            continue;
        }
        // Stack of (node, next-input-index) frames.
        let mut stack = vec![(root, 0usize)];
        color.insert(root, Color::Grey);
        while let Some(&mut (id, ref mut next)) = stack.last_mut() {
            let inputs = doc.node(id).map(|n| n.inputs()).unwrap_or_default();
            if *next >= inputs.len() {
                color.insert(id, Color::Black);
                stack.pop();
                continue;
            }
            let input = inputs[*next];
            *next += 1;
            match color.get(&input) {
                Some(Color::Grey) => return Err(EditError::WouldCycle { at: input }),
                Some(Color::White) => {
                    color.insert(input, Color::Grey);
                    stack.push((input, 0));
                }
                // Black (done) or a ref outside the map (dangling —
                // reported by ref checks, not the cycle walk).
                _ => {}
            }
        }
    }
    Ok(())
}

/// The nodes a cascading delete of `id` must remove, ordered so that
/// [`DocEdit::DeleteNode`] accepts every one of them in turn:
/// consumers first, `id` last.
///
/// The set is `id` plus everything reachable from it along the
/// CONSUMER direction of the recipe DAG — the transitive closure of
/// the same [`Node::inputs`] relation [`EditError::DeleteWouldDangle`]
/// is stated over, which is why applying this sequence in order never
/// dangles a reference: every node still live at each step has all of
/// its inputs still live.
///
/// The answer is empty for an id the document does not hold; a caller
/// that wants the refusal asks [`apply`] for it, so the typed verdict
/// has one home.
///
/// One forward pass suffices because [`Doc::order`] is insertion
/// order and an insertion's inputs must already be live, making the
/// list topological: a consumer is always seen after every input it
/// could inherit doom from.
pub fn cascade_delete_order<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    id: RecipeNodeId,
) -> Vec<RecipeNodeId> {
    use std::collections::BTreeSet;
    if doc.node(id).is_none() {
        return Vec::new();
    }
    let mut doomed: BTreeSet<RecipeNodeId> = BTreeSet::from([id]);
    for &n in doc.order() {
        let doomed_by_input = doc
            .node(n)
            .is_some_and(|node| node.inputs().iter().any(|input| doomed.contains(input)));
        if doomed_by_input {
            doomed.insert(n);
        }
    }
    doc.order()
        .iter()
        .rev()
        .copied()
        .filter(|n| doomed.contains(n))
        .collect()
}

/// Apply one edit to a document, PURELY (spec D2): the input is
/// untouched; on acceptance a new value comes back with the
/// [`EditRecord`]. All validation is here — refs resolve, no cycles,
/// dimension checks re-run on touched expressions (spec D6).
#[allow(clippy::too_many_lines)] // one arm per DocEdit variant, each short
pub fn apply<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    edit: &DocEdit<P>,
    tol: Tol,
) -> Result<Applied<P>, EditError> {
    let mut new = doc.clone();
    // A11's cluster records follow the mate graph automatically. The
    // edits that can move it are exactly those that change the
    // instance set, the mate set, or a mate's heads.
    let mut reconcile = false;
    // DM7's strands, read at the door that made them. Only
    // `DeleteNode` can strand a name: no other edit removes a node,
    // and `Rebind` moves references onto a live one.
    let mut strands: Vec<Maintenance> = Vec::new();
    let record = match edit {
        DocEdit::InsertNode { node } => {
            for input in node.inputs() {
                if !new.nodes.contains_key(&input) {
                    return Err(EditError::UnresolvedInput { input });
                }
            }
            // Spec D3 carve-out (ruled): a payload's name refs must
            // point at LIVE nodes at edit time — a never-existed id is
            // a typo. They are not DAG edges: later deletes may strand
            // them (N5), so this is the ONLY door that checks, for
            // every payload that carries a name (`Node::payload_names`
            // — Declare pairs, a BLEND's selection (fillet under M6-5,
            // chamfer alongside it), a SHELL's ordered open list, a
            // derived frame's face, a measure's references, a mate's
            // two heads under A12).
            for name in node.payload_names() {
                if !new.nodes.contains_key(&name.node) {
                    return Err(EditError::DeclareNamesMissingNode { name: name.clone() });
                }
            }
            // The same check for the node a reference is READ AT
            // where that node is not also an input
            // (`Node::payload_read_sites` — a mate's two operands).
            // Same rule, same door, same N5 aftermath: a
            // never-existed id is a typo; a later delete stranding it
            // is the solve's to refuse.
            for at in node.payload_read_sites() {
                if !new.nodes.contains_key(&at) {
                    return Err(EditError::ReadSiteMissingNode { at });
                }
            }
            let id = RecipeNodeId(new.next_id);
            check_node_inputs(id, node)?;
            check_declare_input(&new, id, node)?;
            if let Node::Mate { alignment, .. } = node
                && !alignment.is_finite()
            {
                return Err(EditError::NonFiniteAlignment { node: id });
            }
            check_node_slots(&new, id, node)?;
            // The VQ9 authoring-time door (LIB-SWITCH §4d): a profile
            // program entering the document resolves + replays +
            // validates under the CURRENT param env, refusing typed
            // here rather than at first evaluation.
            if let Node::Profile(p) = node {
                p.check(&new.param_env::<f64>(), tol).map_err(|refusal| {
                    EditError::ProfileProgramRefused {
                        node: id,
                        refusal: Box::new(refusal),
                    }
                })?;
            }
            new.next_id += 1;
            new.nodes.insert(id, node.clone());
            new.order.push(id);
            check_acyclic(&new)?;
            crate::roots::on_insert(&mut new, id, &node.inputs());
            reconcile = true;
            EditRecord {
                minted: Some(id),
                structural: true,
            }
        }
        DocEdit::DeleteNode { id } => {
            if !new.nodes.contains_key(id) {
                return Err(EditError::UnknownNode { id: *id });
            }
            for (&other, node) in &new.nodes {
                if other != *id && node.inputs().contains(id) {
                    return Err(EditError::DeleteWouldDangle {
                        id: *id,
                        referenced_by: other,
                    });
                }
            }
            // The liveness check above proved the entry present and
            // nothing since removes it, so the removal that takes the
            // node out of the document is also what yields the input
            // list `roots::on_delete` needs: no absent case is left to
            // default, and an empty list would be a different edit.
            let Some(node) = new.nodes.remove(id) else {
                unreachable!("DeleteNode: node {} was live at the check above", id.0)
            };
            let inputs = node.inputs();
            new.order.retain(|&n| n != *id);
            // DM7: a payload name of this node is not a DAG edge, so
            // the check above never saw one and the edit stands. What
            // the door owes is the report — every surviving name whose
            // minting node just left, read out of the document as it
            // now stands.
            strands = stranded_names(&new, *id);
            crate::roots::on_delete(&mut new, *id, &inputs);
            reconcile = true;
            // The node's witness (if any) dies with it — ids are
            // never reused, so the entry could never be read again.
            new.witnesses.remove(id);
            // Same for its cluster's placement: the registry's keys
            // name live instantiate nodes, an invariant the save
            // validator re-checks.
            new.placements.remove(id);
            // next_id is NOT decremented: ids are never reused (D3).
            EditRecord {
                minted: None,
                structural: true,
            }
        }
        DocEdit::SetMembers { node, members } => {
            let Some(current) = new.nodes.get(node) else {
                return Err(EditError::UnknownNode { id: *node });
            };
            if current.list_input().is_none() {
                return Err(EditError::SetMembersOnNonList { node: *node });
            }
            // Liveness FIRST, and of the offered list rather than of
            // the rewritten node, so a dangling entry is named as the
            // dangling entry it is.
            for member in members {
                if !new.nodes.contains_key(member) {
                    return Err(EditError::UnresolvedInput { input: *member });
                }
            }
            // The rewrite happens, then the rewritten node walks the
            // insert door's own checks — the same functions, not
            // mirrors of them, so this edit cannot reach a state
            // `InsertNode` would have refused.
            let mut rewritten = current.clone();
            if !rewritten.set_list_input(members.clone()) {
                return Err(EditError::SetMembersOnNonList { node: *node });
            }
            check_node_inputs(*node, &rewritten)?;
            new.nodes.insert(*node, rewritten);
            // The DAG's edges moved, so both invariants that ride on
            // them are re-established rather than assumed: acyclicity
            // (a member downstream of this node would close a loop —
            // the one refusal `InsertNode` gets for free and this edit
            // does not), and the product-root set, which is a function
            // of the edges.
            check_acyclic(&new)?;
            crate::roots::on_set_members(&mut new);
            reconcile = true;
            EditRecord {
                minted: None,
                structural: true,
            }
        }
        DocEdit::SetParam { node, slot, expr } => {
            if slot.is_structural() {
                return Err(EditError::StructuralSlotNeedsStructuralEdit { slot: *slot });
            }
            set_slot(&mut new, *node, *slot, expr)?;
            check_profile_after_slot_edit(&new, *node, *slot, tol)?;
            EditRecord {
                minted: None,
                structural: false,
            }
        }
        DocEdit::SetStructuralParam { node, slot, expr } => {
            if !slot.is_structural() {
                return Err(EditError::NotStructuralSlot { slot: *slot });
            }
            set_slot(&mut new, *node, *slot, expr)?;
            EditRecord {
                minted: None,
                structural: true,
            }
        }
        DocEdit::SetExpression { path, expr } => {
            let Some(node) = new.nodes.get(&path.node) else {
                return Err(EditError::UnknownNode { id: path.node });
            };
            let Some(root) = node.expr(path.slot) else {
                return Err(EditError::UnknownSlot {
                    id: path.node,
                    slot: path.slot,
                });
            };
            let rebuilt = root
                .with_replaced(&path.path, expr.clone())
                .ok_or_else(|| EditError::PathOffTree { path: path.clone() })?
                .map_err(EditError::Dimension)?;
            let structural = path.slot.is_structural();
            set_slot(&mut new, path.node, path.slot, &rebuilt)?;
            check_profile_after_slot_edit(&new, path.node, path.slot, tol)?;
            EditRecord {
                minted: None,
                structural,
            }
        }
        DocEdit::SetDocParam { name, value } => write_doc_param(&mut new, name, value.clone())?,
        DocEdit::SetDocParamValue { name, value } => {
            let Some(declared) = new.params.get(name) else {
                return Err(EditError::DocParamNotDeclared {
                    name: name.clone(),
                    door: CarryForwardDoor::Value,
                });
            };
            // THE carry-forward: the declaration is read off the
            // document and reused whole, so the dimension and the
            // distribution cannot be dropped by an omission here.
            let Some(written) = declared.with_value(*value) else {
                return Err(EditError::DocParamValueKindMismatch {
                    name: name.clone(),
                    declared: declared.dim(),
                    offered: *value,
                });
            };
            write_doc_param(&mut new, name, written)?
        }
        DocEdit::SetDocParamUnit { name, unit } => {
            let Some(declared) = new.params.get(name) else {
                return Err(EditError::DocParamNotDeclared {
                    name: name.clone(),
                    door: CarryForwardDoor::Notation,
                });
            };
            // THE carry-forward, over the other field: the declaration
            // is read off the document and reused whole, so the value
            // and the distribution cannot be dropped by an omission
            // here. Both reasons it can refuse are the DOOR's — this
            // routes them, and decides neither.
            let written = declared.with_display_unit(*unit).map_err(|why| match why {
                DisplayUnitRefusal::CountHasNoNotation => {
                    EditError::DocParamCountHasNoUnit { name: name.clone() }
                }
                DisplayUnitRefusal::Mismatch { unit, declared } => {
                    EditError::DocParamUnitMismatch {
                        name: name.clone(),
                        unit,
                        declared,
                    }
                }
            })?;
            write_doc_param(&mut new, name, written)?
        }
        DocEdit::Rebind { from, to } => {
            if from == to {
                return Err(EditError::RebindIdentity { name: from.clone() });
            }
            if from.kind != to.kind {
                return Err(EditError::RebindKindMismatch {
                    from: from.kind,
                    to: to.kind,
                });
            }
            // The target must denote a LIVE node (node existence now;
            // name-level resolution at evaluation — the Declare
            // carve-out's split, spec D3).
            if !new.nodes.contains_key(&to.node) {
                return Err(EditError::RebindTargetMissingNode { name: to.clone() });
            }
            // The source must have ONCE existed (ids are monotone and
            // never reused): dead-but-once-lived is exactly the
            // NodeGone repair; never-minted is a typo.
            if from.node.0 >= new.next_id {
                return Err(EditError::RebindUnknownName { name: from.clone() });
            }
            // One-shot rewrite of every EXACT reference (sites:
            // Declare pairs, blend selections — fillet and chamfer
            // alike — a shell's open list, appearance-store keys).
            // Zero sites = nothing to repair, refused.
            // Every payload site, by the one list that says which
            // payloads carry a name (`Node::payload_names`' twin): the
            // rewrite reaches a mate's heads exactly as it reaches a
            // Declare pair, and a blend selection's GROWTH PATH (M6-5,
            // ruled #217) re-canonicalizes there — for a chamfer's
            // selection exactly as for a fillet's, since both are the
            // same canonical set; a shell's ORDERED list re-canonicalizes
            // to its own form, dropping a repeat and keeping the
            // earlier position. A mate reference read AT ITS OWN
            // MINT stays read at its own mint; one read elsewhere
            // keeps its operand, which is an authored fact this edit
            // knows nothing about.
            let mut declare_sites = 0usize;
            for node in new.nodes.values_mut() {
                declare_sites += node.rebind_payload_names(from, to);
            }
            // Appearance keys are rebind sites (the attribute rides
            // the name — PR 7's store; also the spec D9 banked
            // operand→final repair path). A per-kind collision with
            // an attribute already on `to` is refused loudly: which
            // value survives would be an auto-pick.
            let mut appearance_sites = 0usize;
            if let Some(moved) = new.appearance.remove(from) {
                appearance_sites += 1;
                let dst = new.appearance.entry(to.clone()).or_default();
                for (kind, attr) in moved.attrs {
                    if dst.attrs.contains_key(&kind) {
                        return Err(EditError::RebindAppearanceCollision {
                            name: to.clone(),
                            kind,
                        });
                    }
                    dst.attrs.insert(kind, attr);
                }
                // The D7 metadata rides the record through the same
                // move, under the same no-auto-pick collision rule.
                for (key, value) in moved.metadata {
                    if dst.metadata.contains_key(&key) {
                        return Err(EditError::RebindMetadataCollision {
                            name: to.clone(),
                            key,
                        });
                    }
                    dst.metadata.insert(key, value);
                }
            }
            if declare_sites + appearance_sites == 0 {
                return Err(EditError::RebindNoReferences { name: from.clone() });
            }
            // A rebound mate head moves a reading edge, and a reading
            // edge is what a cluster is made of.
            reconcile = true;
            EditRecord {
                minted: None,
                // Declare payloads or blend selections changed:
                // content keys move and the threading consumes them
                // — structural. An appearance-only rebind is
                // presentation motion: no content key moves, nothing
                // recomputes.
                structural: declare_sites > 0,
            }
        }
        DocEdit::ReWitness { node, witness } => {
            check_witness_site(&new, *node)?;
            new.witnesses.insert(*node, witness.clone());
            EditRecord {
                minted: None,
                structural: false,
            }
        }
        DocEdit::SetAppearance { name, attr } => {
            // v1 scope: faces and bodies (M4-PLAN item 7); edges/
            // vertices stay a typed refusal until ratified.
            if !matches!(name.kind, EntityKind::Face | EntityKind::Body) {
                return Err(EditError::AppearanceWrongKind { name: name.clone() });
            }
            // Node existence NOW, name-level resolution at evaluation
            // (the ruled Declare carve-out, applied to the second
            // name-referencing edit).
            if !new.nodes.contains_key(&name.node) {
                return Err(EditError::AppearanceNamesMissingNode { name: name.clone() });
            }
            new.appearance
                .entry(name.clone())
                .or_default()
                .attrs
                .insert(attr.kind(), attr.clone());
            // Presentation only: never structural, never a recompute.
            EditRecord {
                minted: None,
                structural: false,
            }
        }
        DocEdit::ReWitnessBulk {
            entries,
            certification: _,
        } => {
            // v1 validates SHAPE; the certification payload rides as
            // data and the M6 solver adds its checker (W4 — additive,
            // no schema change).
            if entries.is_empty() {
                return Err(EditError::EmptyWitnessBulk);
            }
            let mut seen = std::collections::BTreeSet::new();
            for (node, _) in entries {
                check_witness_site(&new, *node)?;
                if !seen.insert(*node) {
                    return Err(EditError::DuplicateWitnessEntry { node: *node });
                }
            }
            for (node, witness) in entries {
                new.witnesses.insert(*node, witness.clone());
            }
            EditRecord {
                minted: None,
                structural: false,
            }
        }
        DocEdit::ClearAppearance { name, kind } => {
            let not_set = || EditError::AppearanceNotSet {
                name: name.clone(),
                kind: *kind,
            };
            let Some(rec) = new.appearance.get_mut(name) else {
                return Err(not_set());
            };
            if rec.attrs.remove(kind).is_none() {
                return Err(not_set());
            }
            if rec.is_empty() {
                new.appearance.remove(name);
            }
            EditRecord {
                minted: None,
                structural: false,
            }
        }
        DocEdit::SetTolerance { eps } => {
            if !(eps.is_finite() && *eps > 0.0) {
                return Err(EditError::InvalidTolerance { value: *eps });
            }
            new.epsilon = *eps;
            EditRecord {
                minted: None,
                // ε parameterizes every content key (and every
                // predicate band): the whole cone recomputes.
                structural: true,
            }
        }
        DocEdit::SetAppearanceMeta { name, key, value } => {
            // Same v1 scope and node-liveness carve-out as
            // SetAppearance: the metadata rides the SAME record.
            if !matches!(name.kind, EntityKind::Face | EntityKind::Body) {
                return Err(EditError::AppearanceWrongKind { name: name.clone() });
            }
            if !new.nodes.contains_key(&name.node) {
                return Err(EditError::AppearanceNamesMissingNode { name: name.clone() });
            }
            if let Err(error) = value.require_versioned() {
                return Err(EditError::MetaUnversioned {
                    name: name.clone(),
                    key: key.clone(),
                    error,
                });
            }
            if let Some(path) = value.first_non_finite() {
                return Err(EditError::MetaNonFinite {
                    name: name.clone(),
                    key: key.clone(),
                    path,
                });
            }
            new.appearance
                .entry(name.clone())
                .or_default()
                .metadata
                .insert(key.clone(), value.clone());
            EditRecord {
                minted: None,
                structural: false,
            }
        }
        DocEdit::ClearAppearanceMeta { name, key } => {
            let not_set = || EditError::MetaNotSet {
                name: name.clone(),
                key: key.clone(),
            };
            let Some(rec) = new.appearance.get_mut(name) else {
                return Err(not_set());
            };
            if rec.metadata.remove(key).is_none() {
                return Err(not_set());
            }
            if rec.is_empty() {
                new.appearance.remove(name);
            }
            EditRecord {
                minted: None,
                structural: false,
            }
        }
        DocEdit::SetRoots { roots } => {
            new.roots.clone_from(roots);
            // Structural: the root list decides which nodes the
            // document's product gathers, and in what order — the
            // product's combinatorial shape, not a continuous value.
            EditRecord {
                minted: None,
                structural: true,
            }
        }
        DocEdit::SetPlacement { node, frame } => {
            if !matches!(new.nodes.get(node), Some(Node::InstantiatePart { .. })) {
                return Err(EditError::PlacementOnNonInstance { node: *node });
            }
            if !frame.is_finite() {
                return Err(EditError::NonFinitePlacement { node: *node });
            }
            let determinant = frame.determinant();
            if determinant <= 0.0 {
                return Err(EditError::ImproperPlacement {
                    node: *node,
                    determinant,
                });
            }
            // A11: the record keys on the cluster, never the
            // instance. A singleton cluster's gauge IS the instance,
            // so a mate-less document's registry is unchanged.
            let gauge = crate::mate::gauge_of(&new, *node);
            new.placements.insert(gauge, *frame);
            // Structural: a placement decides where the instance's
            // material lands, so it is recipe shape, not a continuous
            // slot value — and it moves the document's content pin.
            EditRecord {
                minted: None,
                structural: true,
            }
        }
        DocEdit::UpdateReference { node, new_pin } => {
            // Three refusals, each naming its own subject — an unknown
            // id and a live-but-wrong-kind node are different mistakes
            // and must not collapse into one message.
            let Some(target) = new.nodes.get_mut(node) else {
                return Err(EditError::UnknownNode { id: *node });
            };
            let Node::InstantiatePart { doc_ref, .. } = target else {
                return Err(EditError::UpdateOnNonInstance { node: *node });
            };
            if doc_ref.pin == *new_pin {
                return Err(EditError::PinUnchanged {
                    node: *node,
                    pin: *new_pin,
                });
            }
            // Only the version half moves: A4's document id answers
            // "which part", and no update ever changes that answer.
            doc_ref.pin = *new_pin;
            // Structural: the pin decides WHICH content this instance
            // materializes, so it is recipe shape, not a continuous
            // slot value — and it moves this document's own pin.
            EditRecord {
                minted: None,
                structural: true,
            }
        }
    };
    // The D-2 backstop, on EVERY arm: the maintenance rules make the
    // invariant-violating states unreachable, and this is what says so
    // rather than assuming it.
    crate::roots::check(&new).map_err(EditError::Roots)?;
    // The placement-rule backstop, on EVERY arm (GROUP-BOOLEAN-DESIGN):
    // "how many placements" has exactly ONE spelling, an explicit rule
    // lists at least one placement, and its frames meet the SAME A6/A11
    // bar `SetPlacement` holds a cluster frame to — finite and proper.
    // Checked over the whole document rather than per arm because a
    // structural slot edit can reach a bad state from a node that was
    // consistent before.
    for (&node, n) in &new.nodes {
        match n.placement_rule_fault() {
            None => {}
            Some(PlacementRuleFault::CountSpelling) => {
                return Err(EditError::PlacementRuleMismatch { node });
            }
            Some(PlacementRuleFault::NoPlacements) => {
                return Err(EditError::EmptyPlacementList { node });
            }
            Some(PlacementRuleFault::NonFiniteFrame { .. }) => {
                return Err(EditError::NonFinitePlacement { node });
            }
            Some(PlacementRuleFault::ImproperFrame { determinant, .. }) => {
                return Err(EditError::ImproperPlacement { node, determinant });
            }
        }
    }
    let mut maintenance = strands;
    if reconcile {
        maintenance.extend(
            crate::mate::solve::reconcile(doc, &mut new, tol)
                .into_iter()
                .map(Maintenance::Cluster),
        );
    }
    Ok(Applied {
        doc: new,
        record,
        maintenance,
    })
}

/// A witness edit's site check: the node is live and sketch-bearing
/// (Profile — the v1 sketch node kind; mates extend this at their
/// milestone).
fn check_witness_site<P>(doc: &Doc<P>, id: RecipeNodeId) -> Result<(), EditError> {
    match doc.nodes.get(&id) {
        None => Err(EditError::UnknownNode { id }),
        Some(Node::Profile(_)) => Ok(()),
        Some(_) => Err(EditError::WitnessOnNonSketch { node: id }),
    }
}

/// The VQ9 door after a slot edit landed in a profile program
/// (LIB-SWITCH §4d): re-run resolve + replay + validate under the
/// CURRENT param env; refuse typed. Non-profile slots pass through.
fn check_profile_after_slot_edit<P: crate::ProfilePayload>(
    new: &Doc<P>,
    id: RecipeNodeId,
    slot: SlotId,
    tol: Tol,
) -> Result<(), EditError> {
    if matches!(slot, SlotId::Profile { .. })
        && let Some(Node::Profile(p)) = new.nodes.get(&id)
    {
        p.check(&new.param_env::<f64>(), tol).map_err(|refusal| {
            EditError::ProfileProgramRefused {
                node: id,
                refusal: Box::new(refusal),
            }
        })?;
    }
    Ok(())
}

/// Shared slot-write path: node exists, slot exists, dimension
/// matches, param refs valid — then write.
fn set_slot<P: Clone + crate::ProfilePayload>(
    new: &mut Doc<P>,
    id: RecipeNodeId,
    slot: SlotId,
    expr: &Expr,
) -> Result<(), EditError> {
    let Some(node) = new.nodes.get(&id) else {
        return Err(EditError::UnknownNode { id });
    };
    if node.expr(slot).is_none() {
        return Err(EditError::UnknownSlot { id, slot });
    }
    if expr.dim() != slot.dimension() {
        return Err(EditError::SlotDimensionMismatch {
            slot,
            expected: slot.dimension(),
            found: expr.dim(),
        });
    }
    check_param_refs(new, id, slot, expr)?;
    if let Some(target) = new.nodes.get_mut(&id).and_then(|n| n.expr_mut(slot)) {
        *target = expr.clone();
        Ok(())
    } else {
        Err(EditError::UnknownSlot { id, slot })
    }
}

impl<P: Clone + crate::ProfilePayload> Doc<P> {
    /// Method form of [`apply`] (spec D2's pure edit entry point).
    pub fn apply(&self, edit: &DocEdit<P>, tol: Tol) -> Result<Applied<P>, EditError> {
        apply(self, edit, tol)
    }

    /// Replay an edit list from the EMPTY document under the given
    /// identity (spec D7): the result reproduces the edits' document
    /// BIT-IDENTICALLY (floats are stored exactly; ids re-mint
    /// deterministically). The document id is supplied, not replayed:
    /// identity is authored data the log never carries (ASM-1 D-1).
    pub fn replay(
        id: crate::DocumentId,
        edits: &[DocEdit<P>],
        tol: Tol,
    ) -> Result<Doc<P>, EditError> {
        let mut doc = Doc::empty(id, tol);
        for edit in edits {
            doc = apply(&doc, edit, tol)?.doc;
        }
        Ok(doc)
    }
}
