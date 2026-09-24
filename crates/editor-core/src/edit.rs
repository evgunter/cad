//! The DocEdit vocabulary v1 and the pure `apply` (spec D2 + D6).
//!
//! `apply` returns a NEW document value; the input is untouched.
//! Undo/redo is keeping prior values — no edit destroys history at
//! this layer (spec D2). `apply` is pure over the document AND the
//! reach it is handed ([`crate::mate::MateReach`]): an edit that
//! moves a cluster's gauge mints that cluster's frame from a solve of
//! the prior document, whose lever is the mated parts' own extent, so
//! the result is a function of the document and of the parts' pinned
//! content — never of a store's mood. Every other edit never asks the
//! reach. What the maintenance decided rides the edit
//! ([`Applied::maintenance`], logged as [`LoggedEdit`]), and replay
//! re-applies those rows instead of solving: replay is pure over the
//! log alone (spec D7).

use crate::appearance::{Attr, AttrKind};
use crate::distribution::{Distribution, DistributionFault};
use crate::doc::{
    DisplayUnitRefusal, DistributionRefusal, Doc, DocParam, DocParamValue, NameCarrier, ParamName,
    ParamRefFault, PlacementFault, WitnessSiteFault,
};
use crate::expr::{Dimension, DimensionError, Expr, ExprPath};
use crate::mate::reach::MateReach;
use crate::mate::solve::Maintain;
use crate::meta::{MetaValue, MetaVersionError};
use crate::names::{EntityKind, ProfileEdgeRef, ProfileVertexRef, SegRewrite};
use crate::node::{
    AssertionBoundFault, Node, PlacementRuleFault, RecipeNodeId, SlotDimensionFault, SlotId,
    StableName,
};
use crate::program::{CheckedRecords, ProgramRefusal, checked_replay};
use crate::roots::RootFault;
use crate::witness::{BranchCertification, WitnessDatum};
use geom_core::Tol;

/// The recorded edit vocabulary (spec D6): a closed set of intents over
/// a document value, every arm plain data, applied by the pure
/// [`apply`] (spec D2), which answers a new document and leaves its
/// input untouched. The set has three shapes. Structural edits over
/// nodes, their slots and the document's roots and placements
/// (`InsertNode`, `DeleteNode`, `SetMembers`, `SetProgram`,
/// `SetParam`, `SetStructuralParam`, `SetExpression`, `SetRoots`,
/// `SetPlacement`, `UpdateReference`). The document-parameter family: one
/// create-or-replace door (`SetDocParam`) and the carry-forward doors,
/// each moving ONE field of a standing declaration and keeping the
/// rest (`SetDocParamValue`, `SetDocParamUnit`,
/// `SetDocParamDistribution`; [`CarryForwardDoor`] names them in a
/// refusal). The explicit repairs and the document's
/// presentation state: `Rebind`, the ONLY name repair — the
/// automatic-rebinding policy menu is empty by ratified decision
/// (NAMING-DESIGN N5); `ReWitness`/`ReWitnessBulk`, the recorded
/// witness adoption, never a silent write-back (SOLVER-DESIGN W4);
/// `SetTolerance`, the recorded ε; and the appearance and metadata
/// pairs (`SetAppearance`/`ClearAppearance`,
/// `SetAppearanceMeta`/`ClearAppearanceMeta`, spec D7). Each arm's own
/// doc states what it does and what it refuses; every refusal is a
/// typed [`EditError`].
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
    /// stranded rides the record as a [`Maintenance::Strand`]. An
    /// appearance key is the same carve-out at the store instead of a
    /// payload, and rides it as a [`Maintenance::StrandedAppearance`].
    ///
    /// A `Declare` the deleted node consumed, and nothing else
    /// consumes, is left inert rather than stranded — the edge ran
    /// the other way, so no name is dangling — and rides the record
    /// as a [`Maintenance::OrphanedDeclare`], whose doc carries the
    /// transition rule the report is built on.
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
    /// **Replace a live profile node's PROGRAM whole** — its loops,
    /// their verbs, order and count, arc modes, sides, windings,
    /// target forms, a split circle's `n` — validated once, as one
    /// edit (V2, `crates/profile/README.md`: structure changes only by
    /// this edit). The plane is NOT carried and does not move: it is
    /// the profile's one DAG input (`Node::Profile(p) =>
    /// p.plane_input()`), and DM6 rules that no edit rewires a live
    /// node's inputs — the only rewiring edit is
    /// [`DocEdit::SetMembers`] over a list, and this is not that.
    ///
    /// The new program is stated in full, [`DocEdit::SetMembers`]'s
    /// shape: nothing is inferred about which of the old loops or
    /// steps survived. What IS inferred is the one thing the editor
    /// that authored the edit knows and the door cannot: which old
    /// step each new step continues, stated per loop and per step in
    /// `provenance` ([`LoopProvenance`]). The door is told, never
    /// guesses.
    ///
    /// Every check [`DocEdit::InsertNode`] makes of a profile is made
    /// here, of the REWRITTEN node, through the same functions: the
    /// slot walk (`check_node_slots` — dimensions and document
    /// parameter references over every argument of every loop), then
    /// resolve + replay + validate under the current parameter
    /// environment ([`EditError::ProfileProgramRefused`]). Before any
    /// of that the provenance's SHAPE is checked
    /// ([`EditError::ProvenanceMalformed`]), so a program is never
    /// replayed for an edit that could not have been honoured.
    ///
    /// **The names.** A name holding a profile locator
    /// (`ProfileEdgeRef` / `ProfileVertexRef`) is spelled in the
    /// program's own coordinates (`eval::anchor`, DM8), so reshaping
    /// the program moves what every such name denotes. The door reads
    /// which segments each old step drew and each new step draws off
    /// the two replay records and, for every name carrier the
    /// document holds (`Doc::name_carriers`): a name on a segment of a
    /// KEPT step — one an old step continues into with the same number
    /// of segments — is REWRITTEN in place to its new coordinates and
    /// reported [`Maintenance::Rebound`]; a name on a segment of a
    /// DROPPED step, a CHANGED step (one whose segment count moved: a
    /// `line` re-authored as `arc_fillet`) or a loop no new loop
    /// continues is reported [`Maintenance::Strand`] /
    /// [`Maintenance::StrandedAppearance`] exactly as a delete reports
    /// it (DM7: the subject is the edit that removes a name's
    /// referent, of which the delete is one). Names spelled in another
    /// profile's coordinates are untouched by construction: the walk
    /// asks each minting node which profile anchors its locators
    /// ([`Node::anchoring_profile`]).
    ///
    /// **The retirement.** The door does three things — reports every
    /// strand, rebinds every kept name (both ruled on `[ev]` #2904),
    /// and RETIRES every stranded name, which the ruling did not spell
    /// out and which is put to Ev on
    /// `work/edit/stranded-names-are-retired-to-an-undrawable-coordinate.md`.
    /// A stranded name's locator is rewritten to a coordinate at or
    /// above [`RETIRED_FLOOR`], where no program draws, and the
    /// `Strand` / `StrandedAppearance` row carries that spelling:
    /// left in place it would denote whichever segment the new program
    /// draws at its old index as if it always had (the DI1 aliasing
    /// class), and collide with a kept name moved onto that index. A
    /// retired name resolves `Vanished` at every evaluation until it
    /// is rebound, and a name already retired is left as it is and
    /// reported by no later edit.
    ///
    /// A program byte-identical to the current one under the identity
    /// provenance is legal and reports nothing.
    ///
    /// **The plane cannot be carried**, which is the whole of DM6's
    /// claim here, pinned where a claim about types belongs — the
    /// variant has no such field, so an edit that tried to move it
    /// does not compile:
    ///
    /// ```compile_fail,E0560
    /// let _: editor_core::DocEdit<editor_core::ProfileProgram> =
    ///     editor_core::DocEdit::SetProgram {
    ///         node: editor_core::RecipeNodeId(1),
    ///         plane: editor_core::RecipeNodeId(0),
    ///         loops: Vec::new(),
    ///         provenance: Vec::new(),
    ///     };
    /// ```
    SetProgram {
        /// The profile node whose program is replaced.
        node: RecipeNodeId,
        /// The whole new program, outer loop first then holes in
        /// description order — every loop stated in full.
        loops: Vec<crate::program::LoopProgram>,
        /// One entry per new loop, in `loops`' order: which old loop
        /// it continues and which old step each of its steps
        /// continues.
        provenance: Vec<LoopProvenance>,
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
    /// Write an E1/E2 ANNOTATION onto an already-declared document
    /// parameter, keeping its declaration: its dimension, its exact
    /// value and its authored display unit ride through untouched
    /// ([`DocParam::with_distribution`]).
    ///
    /// The third of the carry-forward doors, one per field of the
    /// declaration a narrow edit can move, and it exists for its
    /// siblings' reason. The only other way to annotate a standing
    /// parameter is [`Self::SetDocParam`] — create-or-replace — with a
    /// `DocParam` the caller assembled, and the authoring spelling for
    /// an annotated parameter ([`DocParam::continuous_with`]) writes
    /// the CANONICAL notation: a parameter authored in millimetres
    /// reverts to metres the moment anyone annotates it. There is
    /// nothing to restate here.
    ///
    /// **`None` CLEARS the annotation**, through this same door; the
    /// argument is [`DocParam::with_distribution`]'s rustdoc.
    ///
    /// Refuses typed on a name the document does not declare
    /// ([`EditError::DocParamNotDeclared`] — there is no declaration to
    /// carry forward), on a `Count`
    /// ([`EditError::DocParamCountHasNoDistribution`] — a count takes
    /// no annotation, the argument again being
    /// [`DocParam::with_distribution`]'s rustdoc) and on a
    /// distribution that breaks an E2 invariant
    /// ([`EditError::NonFiniteDocParam`],
    /// [`EditError::InvalidDistribution`] — the invariants the
    /// save/load validator refuses a document for).
    SetDocParamDistribution {
        /// The parameter name — must already be declared, and must not
        /// be a `Count`.
        name: ParamName,
        /// The annotation to write, or `None` to clear it.
        distribution: Option<Distribution>,
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
    /// dangling semantics, same as Declare), and reports it where it
    /// happens as a [`Maintenance::StrandedAppearance`] (DM7).
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

/// **Where one loop of a [`DocEdit::SetProgram`] came from**: which
/// old loop it continues and which old step each of its steps
/// continues, stated by the editor that reshaped the program — the one
/// party that knows it inserted a leg rather than replaced one.
///
/// Every index is into the program the node HOLDS when the edit is
/// applied: `from` an old loop index, each `steps[i]` an old step
/// index of that old loop. `None` is "new" — a loop or step nothing
/// old continues into — and it is the only way to say so: a new loop's
/// steps are all new, and naming an old step under a loop with no
/// `from` is a shape fault ([`ProvenanceFault::StepOfNewLoop`]).
///
/// The shape is checked before the program is: one entry per new loop,
/// one per new step, every index inside the old program, no old loop
/// and no old step continued twice ([`ProvenanceFault`]). An old loop
/// or step NOTHING continues is dropped, and every name on its
/// segments strands (DM7).
///
/// Persisted beside its edit, so it is on the wire and refuses a
/// field it does not know like every other wire type.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoopProvenance {
    /// The old loop this loop continues, or `None` for a new loop.
    pub from: Option<u32>,
    /// Per step of this loop, in program order: the old step (of the
    /// loop `from` names) it continues, or `None` for a new step. A
    /// carrier form (`circle`, `circle_split`) has exactly one step,
    /// numbered 0, so its list has one entry.
    pub steps: Vec<Option<u32>>,
}

impl LoopProvenance {
    /// The provenance under which every loop and step of `loops`
    /// continues the old loop and step at the same index — the
    /// spelling of "this program is the one the node holds, and each
    /// step is still itself", which a byte-identical program carries.
    pub fn identity(loops: &[crate::program::LoopProgram]) -> Vec<Self> {
        loops
            .iter()
            .enumerate()
            .map(|(i, lp)| Self {
                from: Some(crate::program::program_index(i)),
                steps: (0..lp.authored_steps())
                    .map(|k| Some(crate::program::program_index(k)))
                    .collect(),
            })
            .collect()
    }
}

/// What is wrong with the SHAPE of a [`DocEdit::SetProgram`]'s
/// provenance — refused before the program is replayed, since a
/// provenance the door could not honour makes the replay moot.
///
/// One typed fault per way the shape can be wrong, each named for what
/// is wrong rather than for where the check tripped, and carried as
/// ONE arm of [`EditError`] ([`EditError::ProvenanceMalformed`]) the
/// way a measure's shape faults are ([`EditError::MeasureMalformed`]):
/// they are seven answers to one question about one field of one edit,
/// and a caller that branches on the edit's refusal reads the field's
/// fault beside it rather than seven refusals of the edit.
///
/// Every index here is a coordinate a caller wrote, so the faults name
/// it in the caller's own terms: a NEW loop or step index where the
/// entry sits, an OLD one where it points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvenanceFault {
    /// The provenance has an entry per loop, and the count is not the
    /// new program's loop count.
    LoopCount {
        /// How many loops the new program has.
        loops: usize,
        /// How many entries the provenance has.
        provenance: usize,
    },
    /// One loop's entry has a step per step, and the count is not
    /// that loop's authored step count.
    StepCount {
        /// The new loop.
        loop_: u32,
        /// How many steps it authors.
        steps: usize,
        /// How many entries its provenance has.
        provenance: usize,
    },
    /// A new loop claims to continue an old loop the old program does
    /// not have.
    NoSuchOldLoop {
        /// The new loop.
        loop_: u32,
        /// The old loop index it names.
        from: u32,
        /// How many loops the old program has.
        old_loops: usize,
    },
    /// A new step claims to continue an old step its old loop does
    /// not have.
    NoSuchOldStep {
        /// The new loop.
        loop_: u32,
        /// The new step.
        step: u32,
        /// The old loop the new loop continues.
        from: u32,
        /// The old step index it names.
        old_step: u32,
        /// How many steps that old loop authors.
        old_steps: usize,
    },
    /// A step of a NEW loop (`from: None`) claims to continue an old
    /// step. A new loop continues nothing, so there is no old loop for
    /// the step index to be a step of.
    StepOfNewLoop {
        /// The new loop.
        loop_: u32,
        /// The new step.
        step: u32,
        /// The old step index it names.
        old_step: u32,
    },
    /// Two new loops claim to continue one old loop. A loop continues
    /// into at most one loop; a copy is a new loop.
    OldLoopContinuedTwice {
        /// The old loop.
        from: u32,
        /// The first new loop that names it.
        first: u32,
        /// The later one.
        again: u32,
    },
    /// Two steps of one new loop claim to continue one old step. A step
    /// continues into at most one step; a copy is a new step.
    OldStepContinuedTwice {
        /// The new loop.
        loop_: u32,
        /// The old loop it continues.
        from: u32,
        /// The old step named twice.
        old_step: u32,
        /// The first new step that names it.
        first: u32,
        /// The later one.
        again: u32,
    },
}

impl core::fmt::Display for ProvenanceFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::LoopCount { loops, provenance } => write!(
                f,
                "the new program has {loops} loops and the provenance {provenance} entries; \
                 the provenance states one entry per loop"
            ),
            Self::StepCount {
                loop_,
                steps,
                provenance,
            } => write!(
                f,
                "loop {loop_} authors {steps} steps and its provenance has {provenance} \
                 entries; a loop's provenance states one entry per step"
            ),
            Self::NoSuchOldLoop {
                loop_,
                from,
                old_loops,
            } => write!(
                f,
                "loop {loop_} continues old loop {from}, and the program being replaced has \
                 {old_loops} loops"
            ),
            Self::NoSuchOldStep {
                loop_,
                step,
                from,
                old_step,
                old_steps,
            } => write!(
                f,
                "loop {loop_} step {step} continues old step {old_step} of old loop {from}, \
                 which authors {old_steps} steps"
            ),
            Self::StepOfNewLoop {
                loop_,
                step,
                old_step,
            } => write!(
                f,
                "loop {loop_} is a new loop and its step {step} continues old step {old_step}; \
                 a new loop continues nothing, so its steps are all new"
            ),
            Self::OldLoopContinuedTwice { from, first, again } => write!(
                f,
                "old loop {from} is continued by loop {first} and again by loop {again}; a \
                 loop continues into at most one loop"
            ),
            Self::OldStepContinuedTwice {
                loop_,
                from,
                old_step,
                first,
                again,
            } => write!(
                f,
                "old step {old_step} of old loop {from} is continued by loop {loop_}'s step \
                 {first} and again by its step {again}; a step continues into at most one step"
            ),
        }
    }
}

impl core::error::Error for ProvenanceFault {}

impl<P> DocEdit<P> {
    /// **Whether this edit can move the MATE GRAPH** — the reading
    /// edges A11's clusters are made of: the instance set, the mate
    /// set, or a mate's heads.
    ///
    /// [`apply`] re-keys the placement registry
    /// ([`crate::mate::solve::reconcile`]) after exactly the edits that
    /// answer `true`, and that is what makes a non-gauge placement row
    /// unrepresentable through the edit doors — the asymmetry
    /// [`crate::doc::placement_fault`] records, and the one the load
    /// door's `PlacementNotGauge` exists for.
    ///
    /// **Exhaustive, with no wildcard arm**, because that invariant is
    /// what a new edit arm can silently break: an arm added without an
    /// answer here stops the crate compiling, rather than defaulting to
    /// "moves nothing" and making a refusal the load door owns
    /// reachable from an edit door.
    /// **Whether this edit writes a mate's alignment datum** — the
    /// numbers, the primitive, the sense and the rider the solve's
    /// per-mate admission decides on. Exactly one edit does: the
    /// insert of a `Node::Mate`, which is where the admission is asked.
    /// A `Rebind` moves a reference's NAME (and, read at its own mint,
    /// its operand), never the datum, and what it strands is N5's —
    /// the solve's at evaluation.
    ///
    /// Exhaustive with no wildcard arm, for the reason
    /// [`Self::moves_the_mate_graph`] gives: an arm added without an
    /// answer here stops the crate compiling rather than writing a
    /// datum past the admission.
    pub(crate) fn writes_a_mates_datum(&self) -> bool {
        match self {
            Self::InsertNode { node } => matches!(node, Node::Mate { .. }),
            // A reshaping rebinds or retires the NAMES a mate's heads
            // hold — `Rebind`'s motion over every name at once — and
            // never touches a datum; a head it strands is N5's, the
            // solve's at evaluation.
            Self::SetProgram { .. } => false,
            Self::DeleteNode { .. }
            | Self::SetMembers { .. }
            | Self::Rebind { .. }
            | Self::SetPlacement { .. }
            | Self::SetParam { .. }
            | Self::SetStructuralParam { .. }
            | Self::SetExpression { .. }
            | Self::SetDocParam { .. }
            | Self::SetDocParamValue { .. }
            | Self::SetDocParamUnit { .. }
            | Self::SetDocParamDistribution { .. }
            | Self::ReWitness { .. }
            | Self::ReWitnessBulk { .. }
            | Self::SetAppearance { .. }
            | Self::ClearAppearance { .. }
            | Self::SetTolerance { .. }
            | Self::SetAppearanceMeta { .. }
            | Self::ClearAppearanceMeta { .. }
            | Self::SetRoots { .. }
            | Self::UpdateReference { .. } => false,
        }
    }

    pub(crate) fn moves_the_mate_graph(&self) -> bool {
        match self {
            // The instance set and the mate set are both node sets, so
            // the two edits over nodes move the graph.
            Self::InsertNode { .. } | Self::DeleteNode { .. } => true,
            // A list input is a reading edge, and a cluster is made of
            // reading edges.
            Self::SetMembers { .. } => true,
            // A rebound mate head moves a reading edge onto another
            // node, which is the graph's shape changing without its
            // node set changing.
            Self::Rebind { .. } => true,
            // A reshaped program rebinds every name on its kept steps
            // in place, a mate's head among them — the same motion as
            // `Rebind`, over every name at once.
            Self::SetProgram { .. } => true,
            // Everything else writes a value, a slot, a payload or a
            // presentation record, and leaves the reading edges where
            // they are. `SetPlacement` is the pointed one: it WRITES
            // the registry the reconciliation re-keys, and the edit
            // door keys it on the gauge itself, so it has no graph
            // motion to reconcile.
            Self::SetPlacement { .. }
            | Self::SetParam { .. }
            | Self::SetStructuralParam { .. }
            | Self::SetExpression { .. }
            | Self::SetDocParam { .. }
            | Self::SetDocParamValue { .. }
            | Self::SetDocParamUnit { .. }
            | Self::SetDocParamDistribution { .. }
            | Self::ReWitness { .. }
            | Self::ReWitnessBulk { .. }
            | Self::SetAppearance { .. }
            | Self::ClearAppearance { .. }
            | Self::SetTolerance { .. }
            | Self::SetAppearanceMeta { .. }
            | Self::ClearAppearanceMeta { .. }
            | Self::SetRoots { .. }
            | Self::UpdateReference { .. } => false,
        }
    }
}

/// Which CARRY-FORWARD door an edit came through — the edits that
/// write one field of a standing declaration and carry the rest
/// untouched.
///
/// It exists so a refusal every such door shares can name the one the
/// caller actually used ([`EditError::DocParamNotDeclared`]). A door
/// over a further field adds an arm here and the compile names every
/// sentence that has to learn the word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarryForwardDoor {
    /// [`DocEdit::SetDocParamValue`] — the number.
    Value,
    /// [`DocEdit::SetDocParamUnit`] — the notation.
    Notation,
    /// [`DocEdit::SetDocParamDistribution`] — the E1/E2 annotation.
    Annotation,
}

// The door as it appears inside a refusal's sentence, in the user's
// vocabulary rather than the enum's: "a value edit", not "Value".
impl core::fmt::Display for CarryForwardDoor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Value => "a value edit",
            Self::Notation => "a notation edit",
            Self::Annotation => "an annotation edit",
        })
    }
}

/// **The one recourse for a parameter name that does not exist**, and
/// the only home of its wording.
///
/// One mistake reaches two doors. Typing an undeclared name into the
/// value field reaches a carry-forward edit, which refuses
/// [`EditError::DocParamNotDeclared`]; dragging that parameter's row
/// is a lookup with no edit behind it, and the viewer refuses
/// `Refusal::NoSuchParam` (`crates/viewer/src/session/refuse.rs`, the
/// second reader of this const and the only one outside this crate).
/// The two are converged on the RECOURSE and not on the sentence,
/// because a drag has no refused edit to report and a sentence that
/// borrowed the door's frame would report a refusal of something
/// nobody attempted. What is converged is what the user must DO, so
/// it is written once here and rendered twice.
pub const UNDECLARED_PARAM_RECOURSE: &str = "declare it first";

/// Typed, specific edit refusal (spec D6: no stringly errors).
///
/// **The param-ref naming convention is stated here and nowhere else.**
/// `Doc::param_ref_fault` answers TWO facts about a reference to a
/// document parameter — undeclared, wrong dimension — and each door
/// asks it at TWO addresses: a slot expression and a payload
/// expression (a measured expression's value leaf, an assertion's
/// bound — the expressions no slot addresses). Those four meanings are
/// named as the product `{Slot,Payload}` x
/// `{UnknownDocParam,DocParamDimension}`: the ADDRESS leads, the FACT
/// trails, and the parameter is ONE noun (`DocParam`) in every arm.
/// The load door's four ([`crate::SnapshotError::SlotUnknownDocParam`]
/// and its three siblings) are the SAME four names, because the
/// address is what the walk iterates and the fact is what the rule
/// answers — so a reader who knows one of the eight arms can spell the
/// other seven. The guard is
/// `display_contract::the_two_doors_spell_the_four_param_ref_refusals_the_same_way_and_each_reports_its_address`,
/// which measures both halves: the four names per door, and that each
/// arm's address word is the address its sentence reports.
///
/// **Its SCOPE is a param REFERENCE, and there are two families.**
/// Those eight arms name two facts about a reference AT an address,
/// which is why the address can lead. This enum's other
/// document-parameter refusals are about the parameter's
/// DECLARATION — [`EditError::DocParamUnitMismatch`],
/// [`EditError::DocParamValueKindMismatch`],
/// [`EditError::DocParamCountHasNoUnit`],
/// [`EditError::DocParamCountHasNoDistribution`],
/// [`EditError::ContinuousParamCannotBeCount`] and
/// [`EditError::DocParamNotDeclared`] — and a declaration has no
/// address: the parameter IS the subject, so each is named by its
/// FACT alone. Forcing them into `{address}{fact}` would mint an
/// address word denoting nothing, so the shape is deliberately not
/// theirs. ([`EditError::DocParamNotDeclared`]'s `door` says which
/// carry-forward edit was refused — which edit, not where a
/// reference sits.)
///
/// [`crate::expr::EvalError::ParamDimensionMismatch`] is the
/// dimension fact raised at EVALUATION instead of at a door, and it
/// keeps its own name because the split lands elsewhere there: the
/// arm carries the fact and the WRAPPER carries the address —
/// [`crate::eval::NodeErrorKind::Expr`] names a node and a slot,
/// [`crate::eval::NodeErrorKind::PayloadExpr`] names a node and a
/// payload, and both forward the refusal unaltered.
///
/// **Which family a new arm joins is decided by what it refuses**, a
/// reference or a declaration — never by the words already in its
/// name. A sweep by SHAPE misses half of the declaration family:
/// [`EditError::DocParamCountHasNoUnit`] and its siblings carry no
/// `Mismatch` in them, so sweep by SUBJECT (`DocParam`, `Param`) too.
///
/// Every other mention of the convention in this tree cites this
/// paragraph instead of re-wording it.
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
    /// `SetProgram` aimed at a node that holds no profile program: a
    /// node of another kind, or a `Node::Profile` over a payload that
    /// carries no program ([`crate::ProfilePayload::loops`] answers
    /// `None` — the `Doc<P>` test payloads). Replacing "the program"
    /// of a node that has none is a different sentence, not a smaller
    /// version of this edit.
    SetProgramOnNonProfile {
        /// The node that holds no program.
        node: RecipeNodeId,
    },
    /// A `SetProgram`'s provenance does not have the shape its program
    /// needs ([`ProvenanceFault`] says which way), refused before the
    /// program is replayed: a provenance the door could not honour
    /// makes the replay moot, and a caller mends the field named
    /// rather than the program.
    ProvenanceMalformed {
        /// The profile node.
        node: RecipeNodeId,
        /// What is wrong with the provenance.
        fault: ProvenanceFault,
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
    /// The A11 cluster-record maintenance needed a solved frame — a
    /// cluster whose gauge moved — and the prior document's solve
    /// reached NO VERDICT at that gauge: its parts could not be
    /// levered or do not resolve, the band could not be formed, a
    /// case split escalated, or a placer's pose could not be derived.
    /// A pose may exist and nothing knows it, so the edit is refused
    /// rather than recording a frame nothing decided. (A cluster the
    /// solve DECIDED has no pose — contradictory, under-determined, a
    /// dangling head — is not this: deleting the offending mate is
    /// the recourse those refusals name, and the orphan keeps the
    /// cluster's frame.)
    MaintenanceRefused {
        /// The gauge the maintenance was solving for.
        gauge: RecipeNodeId,
        /// The solve's fault at the gauge — a cluster's refusal
        /// reaches every member the solve could not pose. `None` when
        /// the solve placed nothing at the gauge and recorded no fault
        /// on it: a state the solve's own invariants exclude, which
        /// this arm REPORTS (fail-loud, without a panic — the door has
        /// a typed refusal and a caller to hand it to, so it is not an
        /// `unreachable!`) rather than reading as the identity or
        /// borrowing another cluster's fault.
        fault: Option<Box<crate::mate::MateFault>>,
    },
    /// Replay of a logged edit that recorded no maintenance rows,
    /// where the edit performs cluster maintenance. Replay performs
    /// exactly an entry's rows ([`LoggedEdit`]) and never solves: a row
    /// that needs a frame has nothing to mint it from, and a row it
    /// could derive from the documents alone (a `Join`, a `Drop`) is
    /// refused rather than re-derived, so the entry and its replay
    /// never disagree. `persist::save` verifies its log
    /// through this same replay, so no file it wrote refuses here.
    MaintenanceUnrecorded {
        /// The gauge the unrecorded act moves: the absorbed cluster's
        /// for a join, the new gauge for a split or rewrite, the dropped
        /// one for a drop.
        gauge: RecipeNodeId,
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
    /// A SLOT expression references a document parameter the document
    /// does not declare. First of the four arms of
    /// `Doc::param_ref_fault` at this door, which sit together and are
    /// named under the convention this enum's own doc states.
    SlotUnknownDocParam {
        /// The missing parameter.
        name: ParamName,
        /// The referencing node.
        node: RecipeNodeId,
        /// The referencing slot.
        slot: SlotId,
    },
    /// A SLOT expression's recorded ref dimension disagrees with the
    /// document parameter's declared dimension.
    SlotDocParamDimension {
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
    /// A PAYLOAD expression — a measured expression's value leaf, an
    /// assertion's bound, the expressions no slot addresses —
    /// references a document parameter the document does not declare.
    /// The address is the NODE rather than a slot, so the arm says so
    /// instead of borrowing a slot name from a node that has one.
    PayloadUnknownDocParam {
        /// The missing parameter.
        name: ParamName,
        /// The referencing node.
        node: RecipeNodeId,
    },
    /// A PAYLOAD expression's recorded ref dimension disagrees with the
    /// document parameter's declared dimension.
    PayloadDocParamDimension {
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
    /// A `Continuous` doc param declared with `Dimension::Count` —
    /// Count parameters use [`DocParam::Count`] (exact integers).
    ContinuousParamCannotBeCount {
        /// The parameter.
        name: ParamName,
    },
    /// A carry-forward edit — [`DocEdit::SetDocParamValue`],
    /// [`DocEdit::SetDocParamUnit`] or
    /// [`DocEdit::SetDocParamDistribution`] — named a parameter this
    /// document does not declare. All three carry an existing
    /// declaration forward, so there has to be one; declaring a
    /// parameter is [`DocEdit::SetDocParam`]'s job.
    ///
    /// ONE arm for all of them because the FAULT is one — the missing
    /// declaration, which neither door is about — and so is the
    /// recourse. What differs is which edit the user submitted, and
    /// that rides along in `door` so the sentence can say it: a
    /// refusal that read "a carry-forward edit" would make a reader
    /// work out which of their edits it was talking about.
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
    /// An annotation edit ([`DocEdit::SetDocParamDistribution`]) named
    /// a `Count` parameter, which takes no distribution and carries no
    /// field to write one into — the argument is
    /// [`DocParam::with_distribution`]'s rustdoc (E11.3).
    ///
    /// [`Self::DocParamCountHasNoUnit`]'s sibling at the third field,
    /// and separate from it for the same reason the two doors are
    /// separate — the fault is what the count has no room for, and a
    /// caller branching on it is told which of their edits to
    /// withdraw. Raised for a CLEARING edit too: a caller aiming an
    /// annotation edit at a count has the wrong parameter, and
    /// answering `Ok` because the field happened to be absent would
    /// hide that.
    DocParamCountHasNoDistribution {
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
        /// WHICH of its floats it is. The predicate identifies the
        /// field to answer at all, and this door carries it for the
        /// reason the load door's site does: a sentence naming `sigma`
        /// beats one naming only the parameter.
        field: crate::doc::DocParamField,
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
    /// The mate solve refuses this mate on its OWN datum — a fact
    /// about the mate alone, which the solve records against it
    /// whenever it reads the datum: its head does not resolve to a
    /// member, it names one member twice, its class is outside the
    /// vocabulary, a frame has no definite direction, the coset table
    /// has no row for it, or its clocking rider contradicts the
    /// coincidence it rides (decided over the mate's own lever,
    /// through the reach this door holds). The same admission the
    /// solve makes (`ASSEMBLY.md` A11 rule 1), asked at the insert
    /// door, so the insert is refused; the recourse is the fault's
    /// own. A mate on a pair the fold never reads — two members over
    /// one instance — is refused on the datum alone all the same.
    /// What is NOT this: a verdict about a PAIR — under-determined,
    /// contradicting ANOTHER mate, an escalation on a fold — which
    /// needs the cluster and stays the solve's; and a STATE a mate
    /// comes to hold after insert (a head a rebind or a shrunk pattern
    /// strands, a `Part` re-pointed, a doctored or older snapshot),
    /// which the doors do not re-decide and the solve refuses at
    /// evaluation.
    MateRefused {
        /// The mate being inserted.
        node: RecipeNodeId,
        /// The solve's own fault, unaltered.
        fault: Box<crate::mate::MateFault>,
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
// **The category prefix makes `EditError` the exception in this crate,
// not the rule, and that exception is deliberate.** Its neighbours
// still open with a category of their own — `persist:`, `split:`,
// `inline:`, `parse:`, `product:` — and are outside the amendment that
// changed this one, so a reader comparing the two should not read that
// paragraph as describing the crate.
//
// **The bare name, by contrast, IS the crate's rule.** A parameter
// name renders through `ParamName`'s `Display` at every door that
// frames it in a sentence of its own; the one door that quotes is
// `ParseError::UnknownParam`, which echoes the bytes an author typed
// and says so at the site. The SLOT id renders through `SlotId::label`
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
                write!(f, "node {}'s sketch refused: {refusal}", node.0)
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
            Self::SetProgramOnNonProfile { node } => write!(
                f,
                "node {} holds no profile program, so it has no program to set",
                node.0
            ),
            // The fault owns its sentence; the door adds which node's
            // program the provenance was about.
            Self::ProvenanceMalformed { node, fault } => {
                write!(f, "node {}'s program provenance: {fault}", node.0)
            }
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
            // The rule's own clause, forwarded rather than restated:
            // this door's subject IS the slot, so the sentence is the
            // clause and nothing more.
            Self::SlotDimensionMismatch {
                slot,
                expected,
                found,
            } => write!(
                f,
                "{}",
                SlotDimensionFault {
                    slot: *slot,
                    expected: *expected,
                    found: *found
                }
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
            Self::PayloadUnknownDocParam { name, node } => write!(
                f,
                "document parameter {name} does not exist (referenced by node {}'s \
                 payload expression)",
                node.0
            ),
            Self::PayloadDocParamDimension {
                name,
                node,
                declared,
                referenced,
            } => write!(
                f,
                "document parameter {name} is declared {declared} but node {}'s \
                 payload expression references it as {referenced}",
                node.0
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
            Self::SlotUnknownDocParam { name, node, slot } => write!(
                f,
                "document parameter {name} does not exist (referenced by node {}, slot {})",
                node.0,
                slot.label()
            ),
            Self::SlotDocParamDimension {
                name,
                node,
                slot,
                declared,
                referenced,
            } => write!(
                f,
                "parameter {name} is declared {declared} but node {} (slot {}) references it as \
                 {referenced}",
                node.0,
                slot.label()
            ),
            Self::ContinuousParamCannotBeCount { name } => write!(
                f,
                "parameter {name} is continuous, and a continuous parameter cannot be a count — \
                 use a count parameter"
            ),
            // The closing clause is `UNDECLARED_PARAM_RECOURSE`, which
            // the viewer's `Refusal::NoSuchParam` renders too; the
            // const's own doc says why the two doors converge there.
            Self::DocParamNotDeclared { name, door } => write!(
                f,
                "parameter {name} is not declared, so {door} has no declaration to carry \
                 forward — {UNDECLARED_PARAM_RECOURSE}"
            ),
            Self::DocParamCountHasNoUnit { name } => write!(
                f,
                "parameter {name} is a count, and a count is an integer rather than a quantity — \
                 it has no display unit to change"
            ),
            Self::DocParamCountHasNoDistribution { name } => write!(
                f,
                "parameter {name} is a count, and a count is a structural parameter that is fixed \
                 under any error analysis — it has no distribution to change"
            ),
            Self::DocParamUnitMismatch {
                name,
                unit,
                declared,
            } => write!(
                f,
                "parameter {name} is declared {declared} but the display unit offered measures \
                 {unit}"
            ),
            Self::DocParamValueKindMismatch {
                name,
                declared,
                offered,
            } => write!(
                f,
                "parameter {name} is declared {declared} but the value edit offered a \
                 {offered} — changing a parameter's kind is a redeclaration"
            ),
            Self::PathOffTree { path } => {
                let steps: Vec<String> = path.path.iter().map(u8::to_string).collect();
                write!(
                    f,
                    "the expression path [{}] in node {}'s {} slot runs off the tree",
                    steps.join(", "),
                    path.node.0,
                    path.slot.label()
                )
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
            Self::NonFiniteDocParam { name, field } => write!(
                f,
                "parameter {name}'s {field} is not finite — the value and every distribution \
                 offset must be a number"
            ),
            Self::InvalidDistribution { name, fault } => {
                write!(f, "parameter {name} has an invalid distribution: {fault}")
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
            Self::MateRefused { node, fault } => write!(
                f,
                "the mate at node {} is refused by the solve on its own datum: {fault}",
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
            Self::MaintenanceRefused { gauge, fault } => {
                write!(
                    f,
                    "the cluster-record maintenance could not place gauge {}: the prior \
                     document's solve ",
                    gauge.0
                )?;
                match fault {
                    Some(fault) => write!(f, "refused: {fault}"),
                    None => write!(f, "recorded no pose for it and no fault"),
                }
            }
            Self::MaintenanceUnrecorded { gauge } => write!(
                f,
                "the logged edit carries no maintenance rows but moves gauge {}; a log entry \
                 records every cluster row `apply` returned for its edit, and replay neither \
                 solves nor re-derives them",
                gauge.0
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
    /// and carries `name`, whose referent the edit removed — the node
    /// that minted it ([`DocEdit::DeleteNode`]), or the profile
    /// segment it named ([`DocEdit::SetProgram`], for a name on a step
    /// the reshaping dropped or changed).
    ///
    /// After a delete the name still says exactly what it always said;
    /// what is gone is the node that minted it, so evaluation answers
    /// [`crate::resolve::ResolveError::NodeGone`] — rung 1 of the N5
    /// ladder. After a reshaping the name is spelled at a RETIRED
    /// coordinate — at or above [`RETIRED_FLOOR`], which no program
    /// draws (its doc says why it is not left where it was) — so
    /// evaluation answers
    /// [`crate::resolve::ResolveError::Vanished`], rung 3. Either way
    /// the name resolves to nothing and [`DocEdit::Rebind`] is the
    /// repair, from the spelling this row carries. A name is not a
    /// DAG edge (the D3 carve-out), so both edits are legal: this row
    /// is what the door owes instead of a refusal.
    ///
    /// The deleted minting node is `name.node` and is not repeated as
    /// a field of its own: a second copy is a disagreement waiting to
    /// happen.
    Strand {
        /// The surviving node whose payload carries the name.
        node: RecipeNodeId,
        /// The name it carries, as it carries it now: its `node` is
        /// the id a delete removed, or its locator is the coordinate
        /// a reshaping retired it to.
        name: StableName,
    },
    /// **An appearance attachment this edit stranded** (DM7): the
    /// document's appearance store holds an attribute under `name`,
    /// whose referent the edit removed — the minting node, or the
    /// profile segment.
    ///
    /// The second carrier, and it carries no node: an attachment is
    /// keyed by a [`StableName`] in the store rather than held in a
    /// node's payload, so there is no surviving carrier to name and
    /// this is an arm of its own rather than a [`Self::Strand`] with
    /// a sentinel. Everything else is the payload strand's:
    /// [`DocEdit::SetAppearance`] gives the key Declare's N5
    /// semantics, evaluation answers
    /// [`crate::appearance::AppearanceLoss`], and [`DocEdit::Rebind`]
    /// is the repair — [`DocEdit::ClearAppearance`] is the other one,
    /// and deliberately does not require a live node.
    ///
    /// The attachment itself is untouched: DM7 reports, it never
    /// repairs. A reshaping re-keys it under the retired spelling, as
    /// it does every carrier's copy of the name, and touches the
    /// attributes under it not at all.
    StrandedAppearance {
        /// The key the store holds the attachment under, as it holds
        /// it now: its `node` is the id a delete removed, or its
        /// locator is the coordinate a reshaping retired it to.
        name: StableName,
    },
    /// **A [`Node::Declare`] this edit left with no consumer** — the
    /// delete door's orphan report, beside DM7's strands (ruled at
    /// EDIT's wave 11; for Ev's objection): `declare` survives, and
    /// the node the edit removed held the last edge that consumed
    /// it.
    ///
    /// Not a strand and not its mirror: a declaration's names point
    /// at the MEMBERS and its consumer points at IT (the `declare`
    /// edge is a DAG input, DM4), so a delete through the consumer
    /// dangles no name and the document stays legal. What is gone is
    /// the node that would ever have consumed the declaration, and
    /// this row is what says so at the door instead of leaving the
    /// author a node nothing will mention again.
    ///
    /// **The rule is a TRANSITION, not a state.** A `Declare` is
    /// legally consumerless in the one-pass authoring window DM4
    /// sites it for — inserted FIRST, its boolean or union second —
    /// so "consumerless" would report every fresh declaration; what
    /// this row says is that a delete MADE it so. The consumers are
    /// the nodes whose [`Node::inputs`] hold it, read out of the
    /// document AFTER the removal, which is the document the strands
    /// of the same edit are read out of.
    ///
    /// The strands' posture, verbatim: report, never refuse, never
    /// repair. A consumerless `Declare` evaluates to its own payload
    /// and refuses nothing, and the repair is the author's — a
    /// [`DocEdit::DeleteNode`] of the `Declare`, or a new consumer.
    ///
    /// **It has a transient the strands do not** (the strand walk's
    /// own cost paragraph says there are none to cancel there, and
    /// stays true of strands). Deleting the `Declare`
    /// itself means cascading its consumers first
    /// ([`cascade_delete_order`]), and the consumer's step is the
    /// same `(document, edit)` pair as the delete of that consumer
    /// for any other reason, so it reports this row and the next
    /// step removes its subject. Maintenance is a function of the
    /// document and the edit, so the cancellation is not this door's:
    /// the subject of a transient row is always in the doomed set
    /// (`dm7_delete_strands::the_orphan_transient_is_cancellable_at_the_cascade_door`),
    /// so the CASCADE door — the caller that holds
    /// [`cascade_delete_order`]'s answer — is where the net over an
    /// action is computed, and nothing computes it today.
    OrphanedDeclare {
        /// The `Declare` left with no consumer. It is LIVE in the
        /// document this edit produced — the surviving node is the
        /// subject here, where a strand's surviving node is the
        /// carrier and the deleted one is in the name. It is also a
        /// product ROOT of that document, since the same delete
        /// re-rooted it: whether a `Declare` may be one is
        /// `work/edit/an-orphaned-declare-joins-the-product-root-set.md`,
        /// and it is why this arm's `Display` sentence says no node
        /// CONSUMES the declaration rather than that nothing reads
        /// it.
        declare: RecipeNodeId,
    },
    /// **A name this edit rewrote in place** ([`DocEdit::SetProgram`]):
    /// `from` was held by a carrier — a payload, or the appearance
    /// store's keys — and denoted a segment or vertex of a profile
    /// step the edit KEPT; the step's segments now sit at other
    /// coordinates, so every carrier holding `from` now holds `to`,
    /// which denotes the same segment under the new program.
    ///
    /// The one arm that reports a REPAIR rather than a consequence
    /// left for the author: the door knows exactly where the segment
    /// went (it read both replay records), so leaving the name for a
    /// [`DocEdit::Rebind`] would be leaving the author to re-derive an
    /// answer the door had. What the row keeps is the VISIBILITY a
    /// rewrite would otherwise lose: a moved name is in the accepted
    /// edit's report, so a name never re-denotes silently.
    ///
    /// Both names are `StableName`s, and no carrier is named: a
    /// payload name's carrier is the node that holds it and a store
    /// key has none, and a name held by several carriers at once (a
    /// fillet's selection and a paint on the same wall) moved in every
    /// one of them under this ONE row — the row is about the name, not
    /// about where it was found.
    Rebound {
        /// The name every carrier held before the edit.
        from: StableName,
        /// The name every carrier holds now.
        to: StableName,
    },
}

impl core::fmt::Display for Maintenance {
    /// The cluster arm DELEGATES: a registry act's sentence belongs to
    /// the type that knows what the act is, so each enum renders its
    /// own arms and the F6 census guards each list where it lives.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Cluster(act) => write!(f, "{act}"),
            // The sentence names what was removed as the name's
            // REFERENT — the minting node under a delete, the profile
            // segment under a reshaping — because the row does not say
            // which edit made it and must not claim the node is gone
            // when the segment is. "a face name minted by node 7,
            // which this edit deleted" would read as though the name
            // were deleted, and the name is exactly what survives.
            Self::Strand { node, name } => write!(
                f,
                "node {} carries a {}; this edit removed what it denoted (its minting node, or \
                 the profile segment it named), so the name resolves to nothing until it is \
                 rebound",
                node.0, name
            ),
            // The same sentence with the store where the carrying
            // node was: what a reader has to know is that the paint
            // is still there and what took its referent. A store holds
            // a thing UNDER a key, and `StableName`'s own Display
            // supplies the noun ("face name minted by node 7"), so
            // the article is this sentence's to provide.
            Self::StrandedAppearance { name } => write!(
                f,
                "the appearance store holds an attachment under a {}; this edit removed what it \
                 denoted (its minting node, or the profile segment it named), so the name \
                 resolves to nothing until it is rebound or cleared",
                name
            ),
            // The subject is the SURVIVOR here, where both strand
            // sentences above open on a carrier and close on the
            // casualty. What the declaration lost is a reader, not a
            // name, so the sentence says which node went and what
            // that leaves: the node is inert until it is deleted or
            // consumed again.
            // "nothing reads it" would be false: the same delete
            // re-roots the declaration into the document's product
            // root set (`work/edit/an-orphaned-declare-joins-the-product-root-set.md`).
            // What it lost is a CONSUMER, which is what the sentence
            // says.
            Self::OrphanedDeclare { declare } => write!(
                f,
                "node {} declares contacts and this edit deleted the last node that consumed \
                 it, so no node consumes the declaration until a boolean or union names it \
                 again",
                declare.0
            ),
            // The two names render through `StableName`'s own Display,
            // which spells the kind and the minting node and not the
            // path, so the two spellings read alike; what the sentence
            // adds is what happened between them — the profile the
            // name's segment was drawn from was reshaped, and the
            // rewrite is a repair the door made, not a loss it left.
            // "The same step's segment": combinatorially the same
            // segment, drawn by the step the old one continues into;
            // its plane may differ when that step's start moved.
            Self::Rebound { from, to } => write!(
                f,
                "a {} was rewritten in place to the {} that draws the same step's segment under \
                 the reshaped profile program, so every carrier of the name still denotes what \
                 it did",
                from, to
            ),
        }
    }
}

/// **DM7's report**: every reference the document still holds whose
/// minting node the `DeleteNode` just removed — one row per stranded
/// name, by carrier.
///
/// The walk is [`Doc::name_carriers`], the document's one enumeration
/// of which of its fields hold a `StableName`, so the clause's two
/// arms are two arms of ONE pass rather than two functions a third
/// carrier would have to be remembered into: a payload name becomes a
/// [`Maintenance::Strand`] naming the node that carries it, a store
/// key becomes a [`Maintenance::StrandedAppearance`], which names no
/// node because the store holds the attachment itself.
///
/// `doc` is the document AFTER the removal, so the payloads walked
/// are exactly the survivors and a name that left with its own
/// carrier is not reported: nothing is stranded when nothing is left
/// to carry it. The appearance store is not pruned by the delete —
/// that is what makes a key STRANDED rather than gone — so the store
/// half reads the same keys either way; it reads `doc` so that the
/// one pass cannot disagree with itself about which nodes are gone.
///
/// Row order is the enumeration's, which is the order
/// [`Applied::maintenance`] contracts for: payload strands in
/// document order and within one node in [`Node::payload_names`]'
/// order (meaning, for the ordered payloads — a shell's rim, a
/// measure's arguments), then store keys in the store's own
/// `BTreeMap` order, which is `StableName`'s. Nothing is sorted here.
///
/// [`Node::payload_read_sites`] — a mate's two operands — are NOT
/// here. A read site is a node id rather than a name: no N5 ladder
/// resolves it and `Rebind` cannot repair it, so a delete that strands
/// one is the solve's to refuse (A12), not this door's to report.
///
/// **Cost.** One pass over the document's name carriers per accepted
/// delete, so a cascade of `n` nodes pays `n` passes. That is the
/// price of reporting at the door rather than once at the end, and it
/// is what makes the rows TRUE of the document each step produced;
/// `cascade_delete_order` is a walk of the same shape already, and a
/// caller who wants one number for the whole cascade computes it from
/// the doomed set instead of from these rows. Under the vocabulary as
/// it stands there are no transients to cancel: a payload name points
/// at a producer UPSTREAM of its carrier and a cascade deletes
/// dependents first, so a doomed carrier is always gone before the
/// node it names (`rv_dm7_probes`'s
/// `rv_a_sited_declaration_strands_nothing_inside_a_cascade` states
/// the argument and measures the declaration case).
fn stranded_references<P>(doc: &Doc<P>, deleted: RecipeNodeId) -> Vec<Maintenance> {
    doc.name_carriers()
        .filter(|carrier| carrier.name().node == deleted)
        .map(|carrier| match carrier {
            NameCarrier::Payload { node, name } => Maintenance::Strand {
                node,
                name: name.clone(),
            },
            NameCarrier::Store { name } => Maintenance::StrandedAppearance { name: name.clone() },
        })
        .collect()
}

/// **The delete door's orphan report**, beside DM7's strands: the
/// [`Node::Declare`] nodes the accepted `DeleteNode` left with no
/// consumer — one row per `Declare` whose last consuming edge the
/// removed node held.
///
/// `doc` is the document AFTER the removal and `deleted_inputs` is
/// the removed node's [`Node::inputs`], so the two halves of the
/// question are asked of the same two facts the strand pass uses: who
/// is gone, and what the document now holds. A `Declare` is reported
/// exactly when the removed node named it, it is still live, and no
/// live node's `inputs()` hold it.
///
/// [`Maintenance::OrphanedDeclare`] carries the rule — a transition,
/// not a state — and the implementation of it is that the candidates
/// are the deleted node's own inputs rather than the document's
/// declarations.
///
/// Consumption is read through `inputs()` rather than
/// [`Node::declare_input`]: the question is which nodes would ever
/// read this one, and that is the DAG edge. A future node kind that
/// consumes declarations therefore counts here the day it compiles,
/// without a second list to remember it into.
///
/// Sink-hood is [`crate::roots::is_sink`], the one home for "does
/// anything still read this node": the root maintainers ask it of
/// the same deleted node's inputs at the same step, so a report that
/// computed it its own way could name a `Declare` the root set did
/// not, or miss one it did.
///
/// The set is **at most one** under the node vocabulary as it
/// stands, since [`Node::declare_input`] is an `Option` and no kind
/// holds two; the walk is the deleted node's input list, so should a
/// kind ever hold two the rows come in input order, which is what
/// [`Applied::maintenance`] contracts for. Nothing is sorted here.
///
/// **Cost.** One pass over the document's nodes per `Declare` input
/// of the deleted node — nothing for the overwhelming majority of
/// deletes, whose node consumes no declaration at all.
fn orphaned_declares<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    deleted_inputs: &[RecipeNodeId],
) -> Vec<Maintenance> {
    deleted_inputs
        .iter()
        .copied()
        .filter(|id| matches!(doc.node(*id), Some(Node::Declare { .. })))
        .filter(|id| crate::roots::is_sink(doc, *id))
        .map(|declare| Maintenance::OrphanedDeclare { declare })
        .collect()
}

/// **The provenance's shape** ([`LoopProvenance`]), checked before the
/// program is replayed: one entry per new loop, one per new step,
/// every index inside the old program, no old loop or old step
/// continued twice. The first fault met is the one reported, in the
/// order the entries are read — loop count, then per loop its step
/// count, its `from`, and its steps.
fn check_provenance(
    node: RecipeNodeId,
    old: &[crate::program::LoopProgram],
    new: &[crate::program::LoopProgram],
    provenance: &[LoopProvenance],
) -> Result<(), EditError> {
    use crate::program::program_index as ix;
    let fault = |fault: ProvenanceFault| EditError::ProvenanceMalformed { node, fault };
    if provenance.len() != new.len() {
        return Err(fault(ProvenanceFault::LoopCount {
            loops: new.len(),
            provenance: provenance.len(),
        }));
    }
    // Which new loop first continued each old loop, so a second is
    // named against the first.
    let mut continued_by: Vec<Option<u32>> = vec![None; old.len()];
    for (i, (lp, prov)) in new.iter().zip(provenance).enumerate() {
        let loop_ = ix(i);
        if prov.steps.len() != lp.authored_steps() {
            return Err(fault(ProvenanceFault::StepCount {
                loop_,
                steps: lp.authored_steps(),
                provenance: prov.steps.len(),
            }));
        }
        let old_steps = match prov.from {
            None => None,
            Some(from) => {
                let Some(old_loop) = old.get(from as usize) else {
                    return Err(fault(ProvenanceFault::NoSuchOldLoop {
                        loop_,
                        from,
                        old_loops: old.len(),
                    }));
                };
                if let Some(first) = continued_by[from as usize] {
                    return Err(fault(ProvenanceFault::OldLoopContinuedTwice {
                        from,
                        first,
                        again: loop_,
                    }));
                }
                continued_by[from as usize] = Some(loop_);
                Some((from, old_loop.authored_steps()))
            }
        };
        let mut step_continued_by: Vec<Option<u32>> = vec![None; old_steps.map_or(0, |(_, n)| n)];
        for (j, old_step) in prov.steps.iter().enumerate() {
            let step = ix(j);
            let Some(old_step) = *old_step else {
                continue;
            };
            let Some((from, count)) = old_steps else {
                return Err(fault(ProvenanceFault::StepOfNewLoop {
                    loop_,
                    step,
                    old_step,
                }));
            };
            if old_step as usize >= count {
                return Err(fault(ProvenanceFault::NoSuchOldStep {
                    loop_,
                    step,
                    from,
                    old_step,
                    old_steps: count,
                }));
            }
            if let Some(first) = step_continued_by[old_step as usize] {
                return Err(fault(ProvenanceFault::OldStepContinuedTwice {
                    loop_,
                    from,
                    old_step,
                    first,
                    again: step,
                }));
            }
            step_continued_by[old_step as usize] = Some(step);
        }
    }
    Ok(())
}

/// **The floor of the retired index space.** A name whose step a
/// reshaping dropped or changed is RETIRED by [`DocEdit::SetProgram`]
/// to a coordinate at or above this floor — segment `RETIRED_FLOOR +
/// s` of the loop that continues its loop, or loop `RETIRED_FLOOR + l`
/// of the program where none does, `s` and `l` its old coordinates —
/// and no program draws there: a coordinate is a count of the loop's
/// segments in memory (`program_index` narrows exactly such a count,
/// and a loop of `RETIRED_FLOOR` segments would hold 2^31 vertices,
/// tens of gibibytes before the validator's pairwise self-intersection
/// walk ever ran over it), so a drawn coordinate is bounded far below
/// the floor by what fits, not by a check. That is what lets a retired
/// name stay dead under EVERY later edit, a slot edit included — a
/// corner fillet whose runs reach a `Zero` fit emits nothing, so a
/// `SetParam` on its radius grows the loop by two segments and reports
/// nothing, and a coordinate one past the OLD end would have gone live
/// under it. A coordinate already at or above the floor is left
/// exactly where it is by every later reshaping, and reported by none
/// (DM7's clause is the referent THE EDIT removed; a name already
/// retired lost its referent at an earlier one).
///
/// The sum cannot overflow: an old coordinate below the floor plus
/// the floor is at most `u32::MAX - 1`.
pub const RETIRED_FLOOR: u32 = u32::MAX / 2;

/// Whether a coordinate is already in the retired space, where the
/// map leaves it alone.
fn already_retired(loop_: u32, index: u32) -> bool {
    loop_ >= RETIRED_FLOOR || index >= RETIRED_FLOOR
}

/// **Where every segment of the program a node held went**, read off
/// the two checked replay records and the provenance — the map a
/// `SetProgram` rewrites every profile locator through.
///
/// A KEPT step is an old step some new step continues whose recorded
/// span has the SAME length as the new step's: its segments map
/// segment-for-segment in order. A step nothing continues is dropped;
/// a continued step whose span length moved is changed and treated as
/// dropped for every name on it — the record says which arm of the
/// transition table ran, and a different arm did not draw "the same
/// segments, elsewhere". A loop no new loop continues maps none of its
/// segments. Segments are PROGRAM-order indices on both sides (the
/// names hold them so, DM8, and the spans are recorded so), and no
/// canonical permutation enters.
///
/// A vertex maps as the segment ARRIVING at it: vertex `v` is the end
/// of segment `v - 1` (mod the loop's length), so it lands at the end
/// of that segment's image. The segment LEAVING it would be wrong
/// exactly where this edit is most used — a leg inserted before step
/// `s` leaves old vertex `s - 1` where it stands while segment
/// `s - 1`'s image now starts at the inserted point.
///
/// A segment nothing maps has no image, and the name on it is RETIRED
/// ([`RETIRED_FLOOR`]) rather than left in place: left in place it
/// would denote whichever segment the new program draws at its old
/// index, silently, and collide in a selection with a kept name moved
/// onto that index. Retired it resolves to nothing — `Vanished`, the
/// N5 ladder's rung whose diagnosis offers the repair — and the
/// strand row carries the spelling the document now holds. A name
/// already retired is not a strand of this edit and is untouched.
struct SegmentMap {
    /// Per OLD loop, per old segment: the new `(loop, segment)` it
    /// maps to, or `None` where the step that drew it was dropped or
    /// changed. Empty when the old program's record cannot be read, in
    /// which case no segment maps and every name strands: a
    /// provenance cannot be honoured against spans nobody can read.
    old: Vec<Vec<Option<(u32, u32)>>>,
    /// Per OLD loop, the new loop that continues it (`None` where
    /// none does) — where a retired name of that loop is filed.
    continued: Vec<Option<u32>>,
    /// Per NEW loop, its segment count — a vertex wraps modulo it.
    new_len: Vec<u32>,
}

/// One locator's image under a [`SegmentMap`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Image<L> {
    /// The step that drew it was kept: this is where it went.
    Kept(L),
    /// Nothing drew it any more: this is the retired coordinate the
    /// name now carries, and the name is reported stranded.
    Retired(L),
}

impl SegmentMap {
    /// `old_loops` is the old program's loop count — known from the
    /// program itself, so a loop's continuation is read off the
    /// provenance whether or not the old program's record can be
    /// read; `old` is its checked records where it can.
    ///
    /// Both records are read through DM8's checked door
    /// ([`CheckedRecords::span_of`]): the shape check that admitted
    /// them says the step counts agree with the programs, and a span
    /// past its loop's end is refused per step. For the NEW program
    /// that refusal is the door's ([`ProgramRefusal::Record`]); for
    /// the OLD program, which is replayed without validation and may
    /// be a program the tree no longer admits, a span the walk refuses
    /// is a step whose segments cannot be found, and every name on it
    /// strands — reported, never guessed.
    ///
    /// # Errors
    ///
    /// [`ProgramRefusal::Record`] where the new program's checked
    /// record refuses a span.
    fn build(
        old: Option<&[CheckedRecords<'_, '_>]>,
        old_loops: usize,
        new: &[CheckedRecords<'_, '_>],
        provenance: &[LoopProvenance],
    ) -> Result<Self, ProgramRefusal> {
        use crate::program::program_index as ix;
        let new_len: Vec<u32> = new.iter().map(|r| ix(r.segments())).collect();
        let mut continued: Vec<Option<u32>> = vec![None; old_loops];
        for (i, prov) in provenance.iter().enumerate() {
            if let Some(from) = prov.from
                && let Some(slot) = continued.get_mut(from as usize)
            {
                *slot = Some(ix(i));
            }
        }
        let Some(old) = old else {
            return Ok(Self {
                old: Vec::new(),
                continued,
                new_len,
            });
        };
        let mut map: Vec<Vec<Option<(u32, u32)>>> =
            old.iter().map(|r| vec![None; r.segments()]).collect();
        for (i, prov) in provenance.iter().enumerate() {
            let Some(from) = prov.from else {
                continue;
            };
            // The shape check ran first, so `from` is an old loop and
            // every `Some` step is one of its steps; the records were
            // admitted by the checked door, so each has one span per
            // authored step.
            let (Some(old_record), Some(new_record)) = (old.get(from as usize), new.get(i)) else {
                continue;
            };
            for (j, old_step) in prov.steps.iter().enumerate() {
                let Some(k) = *old_step else {
                    continue;
                };
                let ns = new_record
                    .span_of(ix(j))
                    .map_err(|error| ProgramRefusal::Record {
                        loop_: ix(i),
                        error,
                    })?;
                let Ok(os) = old_record.span_of(k) else {
                    continue;
                };
                if os.len() != ns.len() {
                    continue;
                }
                for (t, s) in os.iter().enumerate() {
                    map[from as usize][s] = Some((ix(i), ix(ns.start() + t)));
                }
            }
        }
        Ok(Self {
            old: map,
            continued,
            new_len,
        })
    }

    /// Where old segment `(loop, segment)` went, or `None` where the
    /// step that drew it was dropped or changed, the loop was dropped,
    /// or the old program never drew that coordinate.
    fn kept(&self, loop_: u32, segment: u32) -> Option<(u32, u32)> {
        *self.old.get(loop_ as usize)?.get(segment as usize)?
    }

    /// The retired coordinate for old `(loop, index)`, `index` below
    /// the floor: on the loop that continues its loop, at
    /// `RETIRED_FLOOR + index`; or on loop `RETIRED_FLOOR + loop`
    /// where nothing does, at `index`. Distinct per old coordinate.
    fn retired(&self, loop_: u32, index: u32) -> (u32, u32) {
        match self.continued.get(loop_ as usize).copied().flatten() {
            Some(i) => (i, RETIRED_FLOOR + index),
            None => (RETIRED_FLOOR + loop_, index),
        }
    }

    /// The image of an edge locator; `None` where it is already
    /// retired and this edit has nothing to say about it.
    fn edge(&self, e: ProfileEdgeRef) -> Option<Image<ProfileEdgeRef>> {
        if already_retired(e.loop_index, e.segment) {
            return None;
        }
        let at = |(loop_index, segment)| ProfileEdgeRef {
            loop_index,
            segment,
        };
        Some(match self.kept(e.loop_index, e.segment) {
            Some(to) => Image::Kept(at(to)),
            None => Image::Retired(at(self.retired(e.loop_index, e.segment))),
        })
    }

    /// The image of a vertex locator, carried by the segment arriving
    /// at it; `None` where it is already retired.
    fn vertex(&self, v: ProfileVertexRef) -> Option<Image<ProfileVertexRef>> {
        use crate::program::program_index as ix;
        if already_retired(v.loop_index, v.vertex) {
            return None;
        }
        let at = |(loop_index, vertex)| ProfileVertexRef { loop_index, vertex };
        let retired = || Some(Image::Retired(at(self.retired(v.loop_index, v.vertex))));
        let Some(n_old) = self.old.get(v.loop_index as usize).map(Vec::len) else {
            return retired();
        };
        if n_old == 0 || v.vertex as usize >= n_old {
            return retired();
        }
        let arriving = (v.vertex as usize + n_old - 1) % n_old;
        let Some((loop_index, segment)) = self.kept(v.loop_index, ix(arriving)) else {
            return retired();
        };
        let n_new = self.new_len.get(loop_index as usize).copied().unwrap_or(0);
        if n_new == 0 {
            return retired();
        }
        Some(Image::Kept(at((loop_index, (segment + 1) % n_new))))
    }
}

/// **A name mapped through a reshaped program**: the [`SegRewrite`]
/// that decides, for every `StableName` a carrier holds, whether the
/// edit left it alone, moved it, or stranded it — and what it is
/// spelled as now. The walk over a name's shape is
/// `RoleSeg::rewrite`'s, shared with the anchor rewrite and the
/// split re-map; what this rewriter adds is the locator's image and
/// the descent into every carried name.
struct ProgramRemap<'a> {
    map: &'a SegmentMap,
    /// The nodes whose OWN locators are spelled in the reshaped
    /// profile's coordinates ([`Node::anchoring_profile`]). A name
    /// minted by any other node holds this profile's coordinates only
    /// inside the `NameRef`s it carries, which the descent reaches.
    anchored: std::collections::BTreeSet<RecipeNodeId>,
    /// Whether the name being walked right now is minted by an
    /// anchored node — set per name on the way down, restored on the
    /// way up.
    here: bool,
    /// Whether any locator of the name being imaged had no image:
    /// reset per top-level name, read after the walk.
    stranded: bool,
}

impl ProgramRemap<'_> {
    fn new(
        map: &SegmentMap,
        anchored: std::collections::BTreeSet<RecipeNodeId>,
    ) -> ProgramRemap<'_> {
        ProgramRemap {
            map,
            anchored,
            here: false,
            stranded: false,
        }
    }

    /// `None`: untouched. `Some(Kept(now))`: moved onto the kept
    /// segments' new coordinates. `Some(Retired(now))`: a locator
    /// anywhere in the name — at its own level or inside a carried
    /// name — had no image, and the whole name is reported stranded
    /// under the spelling `now`.
    fn image(&mut self, name: &StableName) -> Option<Image<StableName>> {
        self.stranded = false;
        let Ok(now) = self.name(name);
        let now = now?;
        Some(if self.stranded {
            Image::Retired(now)
        } else {
            Image::Kept(now)
        })
    }
}

impl SegRewrite for ProgramRemap<'_> {
    type Error = core::convert::Infallible;

    fn edge(&mut self, e: ProfileEdgeRef) -> Result<ProfileEdgeRef, Self::Error> {
        if !self.here {
            return Ok(e);
        }
        Ok(match self.map.edge(e) {
            Some(Image::Kept(to)) => to,
            Some(Image::Retired(to)) => {
                self.stranded = true;
                to
            }
            None => e,
        })
    }

    fn vertex(&mut self, v: ProfileVertexRef) -> Result<ProfileVertexRef, Self::Error> {
        if !self.here {
            return Ok(v);
        }
        Ok(match self.map.vertex(v) {
            Some(Image::Kept(to)) => to,
            Some(Image::Retired(to)) => {
                self.stranded = true;
                to
            }
            None => v,
        })
    }

    /// Every carried name descends: its own locators map iff ITS
    /// minting node anchors to this profile, and the answer is the
    /// rewritten name where anything in it moved.
    fn name(&mut self, n: &StableName) -> Result<Option<StableName>, Self::Error> {
        let outer = core::mem::replace(&mut self.here, self.anchored.contains(&n.node));
        let Ok(now) = n.clone().rewrite_path(self);
        self.here = outer;
        Ok((now != *n).then_some(now))
    }
}

/// **The reshaping's report and rewrite, in one walk over the
/// document's name carriers** ([`Doc::rewrite_names`], the `&mut` twin
/// of the walk the delete's report reads, driven by the same
/// `Carrier::ALL` roster): every name the map strands is rewritten to
/// its retired spelling and reported ([`Maintenance::Strand`] on its
/// carrying node, [`Maintenance::StrandedAppearance`] on a store key),
/// every name the map moves is rewritten in place and reported
/// [`Maintenance::Rebound`] once, at the first carrier the walk met it
/// in. A name already retired is neither.
///
/// The rows come back in [`Applied::maintenance`]'s contracted order:
/// the strands, then the stranded keys, then the rebounds.
///
/// The rewrite is a function of the map alone, so it is deterministic
/// and a replay reproduces it: nothing here reads a hash order.
///
/// # Errors
///
/// A re-keyed appearance record landing on a key already carrying the
/// same attribute kind or metadata key refuses as `Rebind`'s own
/// collision does ([`EditError::RebindAppearanceCollision`],
/// [`EditError::RebindMetadataCollision`]) — which value survives
/// would be an auto-pick. The map cannot produce one on its own: kept
/// segments map injectively and retired coordinates are distinct from
/// every drawn one and from each other, so two keys of this profile
/// never land on one key. What can collide is a key this edit moves
/// with a key of ANOTHER profile that happens to spell the same name,
/// which no coordinate can distinguish and this door does not decide.
fn reshape_report<P>(
    doc: &mut Doc<P>,
    remap: &mut ProgramRemap<'_>,
) -> Result<Vec<Maintenance>, EditError> {
    let mut strands = Vec::new();
    let mut stranded_keys = Vec::new();
    let mut rebounds = Vec::new();
    // The names already reported moved, so a name held by two
    // carriers rides one row. A list rather than a set: the order it
    // is searched in is not an output, and it stays free of any hash.
    let mut reported: Vec<StableName> = Vec::new();
    doc.rewrite_names(
        |carrier| {
            let (name, node) = match carrier {
                NameCarrier::Payload { node, name } => (name, Some(node)),
                NameCarrier::Store { name } => (name, None),
            };
            match remap.image(name)? {
                Image::Retired(now) => {
                    match node {
                        Some(node) => strands.push(Maintenance::Strand {
                            node,
                            name: now.clone(),
                        }),
                        None => stranded_keys
                            .push(Maintenance::StrandedAppearance { name: now.clone() }),
                    }
                    Some(now)
                }
                Image::Kept(now) => {
                    if !reported.contains(name) {
                        reported.push(name.clone());
                        rebounds.push(Maintenance::Rebound {
                            from: name.clone(),
                            to: now.clone(),
                        });
                    }
                    Some(now)
                }
            }
        },
        move_appearance_record,
    )?;
    let mut out = strands;
    out.extend(stranded_keys);
    out.extend(rebounds);
    Ok(out)
}

/// **One appearance record moved onto the key `to`**, attribute by
/// attribute and metadata entry by entry, refusing where `to` already
/// carries the same kind or key: which value survives would be an
/// auto-pick. The store half of every name rewrite —
/// [`DocEdit::Rebind`]'s one pair and [`DocEdit::SetProgram`]'s map —
/// so the two doors cannot disagree about what a collision is.
fn move_appearance_record(
    store: &mut crate::appearance::AppearanceMap,
    moved: crate::appearance::AppearanceRecord,
    to: &StableName,
) -> Result<(), EditError> {
    let dst = store.entry(to.clone()).or_default();
    for (kind, attr) in moved.attrs {
        if dst.attrs.contains_key(&kind) {
            return Err(EditError::RebindAppearanceCollision {
                name: to.clone(),
                kind,
            });
        }
        dst.attrs.insert(kind, attr);
    }
    // The D7 metadata rides the record through the same move, under
    // the same no-auto-pick collision rule.
    for (key, value) in moved.metadata {
        if dst.metadata.contains_key(&key) {
            return Err(EditError::RebindMetadataCollision {
                name: to.clone(),
                key,
            });
        }
        dst.metadata.insert(key, value);
    }
    Ok(())
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
    /// cluster-record maintenance it forced, the references it
    /// stranded (DM7) — the payload names, then the appearance keys —
    /// the names it rewrote in place, and the declarations it left
    /// with no consumer. See [`Maintenance`].
    ///
    /// **The order is a CONTRACT, not an accident of the
    /// implementation, and a consumer may rely on it**: every
    /// [`Maintenance::Strand`] first, in the document's node order
    /// and within one node in the payload's own order; then every
    /// [`Maintenance::StrandedAppearance`], in the appearance store's
    /// key order; then every [`Maintenance::Rebound`], one row per
    /// name in the order the same walk first met it — the node
    /// carriers' occurrences in document order and payload order, then
    /// the store's keys in key order — so a name held by a payload and
    /// by the store rides one row, at the payload's position; then
    /// every [`Maintenance::OrphanedDeclare`] (at most one today —
    /// [`Node::declare_input`] is an `Option`, so no node kind holds
    /// two; in the deleted node's input order should a kind ever hold
    /// two, and
    /// `dm7_delete_strands::no_delete_can_report_two_orphans_today`
    /// reds the day that changes); then the A11 cluster acts, which
    /// reconcile the registry against the document the strands were
    /// read out of. The strands, the rebounds and the orphans are read
    /// at the door, out of the document the edit had just produced. A
    /// delete reports strands and orphans and never a rebound; a
    /// `SetProgram` reports strands and rebounds and never an orphan;
    /// no other edit reports any of the three.
    ///
    /// The paragraph above is the contract — it is stated here in
    /// full because a consumer outside this crate cannot read
    /// `Carrier::ALL`, which is `pub(crate)`. In-crate the order has
    /// one home all the same: one roster, `Carrier::ALL`, drives both
    /// walks — the delete's report reads `Doc::name_carriers`, filtered
    /// on the deleted node, and the program edit's rewrites through
    /// `Doc::rewrite_names`, its `&mut` twin, mapped through the
    /// segment map — so the strands' and the rebounds' orders are that
    /// roster's, and a reader who wants to see why reads it there.
    ///
    /// Each boundary is held by the row whose fixture actually
    /// produces the pair of kinds it separates:
    /// `dm7_delete_strands::an_appearance_strand_follows_the_payload_strands_of_the_same_delete`
    /// for payload strand before appearance strand,
    /// `dm7_delete_strands::a_mates_head_strands_and_its_read_site_does_not`
    /// for payload strand before cluster act, and
    /// `dm7_delete_strands::an_appearance_strand_precedes_the_cluster_acts_of_the_same_delete`
    /// for appearance strand before cluster act — the last one paints,
    /// which the mate row does not, so it is the only row a walk that
    /// appended the store's rows after `reconcile` goes red on.
    /// The orphan boundary is
    /// `dm7_delete_strands::an_orphaned_declare_follows_the_strands_of_the_same_delete`,
    /// whose one delete both strands a name a surviving node carries
    /// and takes a declaration's last consumer. The rebound boundary
    /// is
    /// `edit_set_program::a_reshaping_reports_its_strands_then_its_stranded_keys_then_its_rebounds`,
    /// whose one edit strands a payload name, strands a store key and
    /// rebinds a name held by both carriers.
    /// What a consumer may NOT do is read position 0 as a kind: a
    /// delete that strands no payload name puts an appearance strand
    /// or a cluster act there, so an arm is found by matching, never
    /// by index.
    ///
    /// The cluster acts are a function of the edit AND the reach it
    /// was applied through — a row that needs a solved frame mints it
    /// from the mated parts' extent — which is why the log carries
    /// them ([`LoggedEdit`], through [`Applied::cluster_rows`]) and
    /// replay re-applies them rather than deriving them again.
    pub maintenance: Vec<Maintenance>,
}

impl<P> Applied<P> {
    /// **The cluster acts this edit performed** — the
    /// [`Maintenance::Cluster`] rows of [`Self::maintenance`], in
    /// order: what the log records for the edit ([`LoggedEdit`]) and
    /// replay re-applies without a solve. A strand is not here: it is
    /// a fact the next evaluation reports from the document itself.
    pub fn cluster_rows(&self) -> Vec<crate::mate::ClusterMaintenance> {
        self.maintenance
            .iter()
            .filter_map(|m| match m {
                Maintenance::Cluster(act) => Some(act.clone()),
                // A strand and an orphaned Declare are facts the next
                // evaluation reports from the document, and a rebound
                // name is one the document now holds rewritten; only a
                // cluster act is state replay has to re-apply.
                Maintenance::Strand { .. }
                | Maintenance::StrandedAppearance { .. }
                | Maintenance::OrphanedDeclare { .. }
                | Maintenance::Rebound { .. } => None,
            })
            .collect()
    }
}

/// One expression's document-parameter refs against the param table,
/// in THIS door's vocabulary (spec D6: dimension checks re-run on
/// touched expressions; `node`/`slot` locate the expression for the
/// error). The rule itself is `Doc::param_ref_fault`, the one home the
/// load door reads it from too.
fn check_param_refs<P>(
    doc: &Doc<P>,
    node: RecipeNodeId,
    slot: SlotId,
    expr: &Expr,
) -> Result<(), EditError> {
    match doc.param_ref_fault(expr) {
        None => Ok(()),
        Some(ParamRefFault::Unknown { name }) => {
            Err(EditError::SlotUnknownDocParam { name, node, slot })
        }
        Some(ParamRefFault::Dimension {
            name,
            declared,
            referenced,
        }) => Err(EditError::SlotDocParamDimension {
            name,
            node,
            slot,
            declared,
            referenced,
        }),
    }
}

/// A broken E2 invariant as the edit door reports it, in ONE place.
///
/// The split is by CLASS, not by door: a non-finite offset is a
/// non-finite float on a document parameter and joins the ruled
/// non-finite policy's own refusal (door 1), the rest are distribution
/// shape faults. Both the create-or-replace door and the annotation
/// door reach it, so a caller comparing their refusals reads one
/// answer rather than two spellings of it.
fn distribution_fault_error(name: &ParamName, fault: DistributionFault) -> EditError {
    match fault {
        DistributionFault::NonFinite { field } => EditError::NonFiniteDocParam {
            name: name.clone(),
            field: crate::doc::DocParamField::Offset(field),
        },
        DistributionFault::SigmaNotPositive { .. }
        | DistributionFault::NominalOutsideSupport { .. } => EditError::InvalidDistribution {
            name: name.clone(),
            fault,
        },
    }
}

/// Write a fully-formed [`DocParam`] into the document: the shared
/// tail of every parameter door, so no two of them can come to
/// disagree about what a legal parameter is. Four doors reach it —
/// the create-or-replace door ([`DocEdit::SetDocParam`]) and the three
/// carry-forward doors, one per movable field of the declaration:
/// [`DocEdit::SetDocParamValue`], [`DocEdit::SetDocParamUnit`] and
/// [`DocEdit::SetDocParamDistribution`]. A fifth door writing a
/// declaration routes through here too, and adds itself to that list.
///
/// **The check order is the LOAD door's** (`persist::check`'s
/// `validate_document`): floats first, then the distribution's shape,
/// then the notation walk. A parameter broken in two ways at once
/// therefore gets the same VERDICT whichever door refuses it, and
/// names the same one of its two faults — which is the property a
/// caller comparing an edit refusal against a load refusal relies on.
///
/// For one declaration the two doors reach that verdict by different
/// rules, and say so in different words: a CONTINUOUS parameter
/// declared `Count` is refused here as
/// [`EditError::ContinuousParamCannotBeCount`], and at the load door
/// by the notation walk one step earlier
/// (`PersistError::DisplayUnit`), because no unit in the table
/// measures a count. Both refuse the same declarations; only this door
/// can name the structural/continuous divide as the reason.
fn write_doc_param<P: Clone + crate::ProfilePayload>(
    new: &mut Doc<P>,
    name: &ParamName,
    value: DocParam,
) -> Result<EditRecord, EditError> {
    // Ruled door 1 (non-finite policy): recipe data never carries
    // NaN/inf — the nominal and the distribution offsets alike, by the
    // ONE predicate the load door's float walk asks
    // (`DocParam::first_non_finite`), which is also what decides WHICH
    // float this refusal names.
    if let Some(field) = value.first_non_finite() {
        return Err(EditError::NonFiniteDocParam {
            name: name.clone(),
            field,
        });
    }
    // The REST of E2's invariants, from the ONE shared check the
    // persistence doors also run. Its non-finite arm is unreachable
    // from here — the walk above has already refused every non-finite
    // offset — and stays reachable from the annotation door, which
    // routes a distribution through `Distribution::check` without a
    // declaration around it.
    if let Some(d) = value.distribution()
        && let Err(fault) = d.check()
    {
        return Err(distribution_fault_error(name, fault));
    }
    // The structural/continuous divide (`DocParam::is_continuous_count`).
    // This door is where it is REACHABLE: at the load door the same
    // declaration refuses one walk earlier, because no unit in the
    // table measures a count and the notation walk below asks that of
    // every continuous parameter.
    if value.is_continuous_count() {
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

/// Validate every slot of a node payload against slot dimensions and
/// the param table, keyed as `id` for error reporting.
fn check_node_slots<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    id: RecipeNodeId,
    node: &Node<P>,
) -> Result<(), EditError> {
    // D6's slot rule, from the ONE home the load door reads it from
    // too (`Node::slot_dimension_fault`); this is the door's name for
    // its answer.
    //
    // It is a walk over ALL the node's slots, and it runs before the
    // param-table walk below rather than interleaved with it slot by
    // slot: a node broken in both ways at once names the dimension its
    // address fixes, which is the answer the load door gives for the
    // same node (`persist::check`'s walk order).
    if let Some(SlotDimensionFault {
        slot,
        expected,
        found,
    }) = node.slot_dimension_fault()
    {
        return Err(EditError::SlotDimensionMismatch {
            slot,
            expected,
            found,
        });
    }
    // The param table, against the slot expressions the rule above has
    // just established are all readable.
    for (slot, expr) in node
        .slots()
        .into_iter()
        .filter_map(|slot| Some((slot, node.expr(slot)?)))
    {
        check_param_refs(doc, id, slot, expr)?;
    }
    // The expressions no slot addresses (E3/E10). Their DIMENSIONS are
    // already fixed by construction — a `MeasureExpr` runs the F1
    // checker at every constructor, and an assertion's bound is checked
    // against its measure below — so what is left here is the same
    // parameter-table re-check every slot expression gets.
    for expr in crate::node::payload_exprs(node).into_iter().flatten() {
        match doc.param_ref_fault(expr) {
            None => {}
            Some(ParamRefFault::Unknown { name }) => {
                return Err(EditError::PayloadUnknownDocParam { name, node: id });
            }
            Some(ParamRefFault::Dimension {
                name,
                declared,
                referenced,
            }) => {
                return Err(EditError::PayloadDocParamDimension {
                    name,
                    node: id,
                    declared,
                    referenced,
                });
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
    // constrains (E10): `Node::assertion_bound_fault`, the one home the
    // load door reads it from too. The predicate takes the document
    // because the measured dimension is another node's property; this
    // is the door's name for its answer.
    if let Some(fault) = node.assertion_bound_fault(doc) {
        return Err(match fault {
            AssertionBoundFault::TargetNotMeasure { measure, .. } => {
                EditError::AssertionTarget { node: id, measure }
            }
            AssertionBoundFault::DimensionMismatch {
                measure,
                measured,
                bound,
            } => EditError::AssertionDimension {
                node: id,
                measure,
                measured,
                bound,
            },
        });
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

/// Apply one edit to a document — pure over the document and the
/// reach (spec D2): the input is untouched, and the output is a
/// function of `doc`, `edit` and what `reach` answers; on acceptance
/// a new value comes back with the [`EditRecord`] and the maintenance
/// rows ([`Applied::maintenance`]). All validation is here — refs
/// resolve, no cycles, dimension checks re-run on touched expressions
/// (spec D6), and a mate being inserted passes the solve's own
/// per-mate admission, its clocking rider decided over the mated
/// parts' reach ([`EditError::MateRefused`]).
pub fn apply<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    edit: &DocEdit<P>,
    tol: Tol,
    reach: &dyn MateReach,
) -> Result<Applied<P>, EditError> {
    apply_maintaining(doc, edit, tol, Maintain::Solve(reach))
}

/// **A logged edit and the maintenance it performed** — one entry of
/// a document's edit log, on the wire and in a history.
///
/// `maintenance` is the cluster half of what [`apply`] returned for
/// the edit ([`Applied::cluster_rows`]): the A11 cluster-record rows
/// the mate graph's motion forced on the placement registry, frames
/// included. Replay ([`apply_logged`], [`Doc::replay`],
/// `persist::load`) re-applies these rows and never solves, so the log
/// carries what was decided (D9) and replay stays a function of the log
/// alone. Replay performs EXACTLY an entry's rows: a non-empty list is
/// re-applied verbatim, and an empty one whose edit would perform any
/// refuses ([`EditError::MaintenanceUnrecorded`]) rather than having
/// them re-derived — so the log has one answer to "what did this edit
/// do", and the entry is it.
///
/// On the wire an entry is `{ "edit": …, "maintenance": […] }`, both
/// fields present, `maintenance` empty for an edit that performed
/// none, and nothing is tried and then abandoned while reading one: an
/// empty list and an absent field are not two spellings of one fact.
/// (serde's derived visitor also reads the positional array
/// `[<edit>, [<row>…]]`, the same hatch `persist`'s module docs
/// disclose for the file body; it is a second spelling of the one
/// shape, branched on by its first token, not a fallback.)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoggedEdit<P> {
    /// The edit.
    pub edit: DocEdit<P>,
    /// The maintenance rows `apply` returned for it.
    pub maintenance: Vec<crate::mate::ClusterMaintenance>,
}

impl<P> LoggedEdit<P> {
    /// An entry with no rows: a claim that the edit performs no
    /// cluster maintenance, which replay holds it to
    /// ([`EditError::MaintenanceUnrecorded`]). An edit that inserts or
    /// removes a mate, or deletes a mated instance, needs its rows
    /// (`LoggedEdit { edit, maintenance: applied.cluster_rows() }`).
    pub fn bare(edit: DocEdit<P>) -> Self {
        Self {
            edit,
            maintenance: Vec::new(),
        }
    }

    /// Every edit as a [`Self::bare`] entry.
    pub fn bare_all(edits: &[DocEdit<P>]) -> Vec<Self>
    where
        P: Clone,
    {
        edits.iter().cloned().map(Self::bare).collect()
    }
}

impl<P> From<DocEdit<P>> for LoggedEdit<P> {
    fn from(edit: DocEdit<P>) -> Self {
        Self::bare(edit)
    }
}

/// **Replay one logged edit**: the edit through every door [`apply`]
/// has, then the RECORDED maintenance rows applied to the registry —
/// no solve, no reach, no store. An entry with no rows claims the edit
/// performs no cluster maintenance, and refuses
/// [`EditError::MaintenanceUnrecorded`] if it performs any.
///
/// # Errors
///
/// [`apply`]'s, plus `MaintenanceUnrecorded`.
pub fn apply_logged<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    entry: &LoggedEdit<P>,
    tol: Tol,
) -> Result<Applied<P>, EditError> {
    // An entry with no rows claims the edit performs no cluster
    // maintenance. `Never` holds it to that — any row the replay would
    // perform refuses — where an empty `Recorded` would replay the edit
    // as if it had none, silently. So the branch is here, once.
    let how = if entry.maintenance.is_empty() {
        Maintain::Never
    } else {
        Maintain::Recorded(&entry.maintenance)
    };
    apply_maintaining(doc, &entry.edit, tol, how)
}

/// [`apply`] with the maintenance's source chosen by the door
/// ([`Maintain`]): solved through a reach, refused where a frame
/// would be needed, or the recorded rows re-applied.
#[allow(clippy::too_many_lines)] // one arm per DocEdit variant, each short
fn apply_maintaining<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    edit: &DocEdit<P>,
    tol: Tol,
    how: Maintain<'_>,
) -> Result<Applied<P>, EditError> {
    let mut new = doc.clone();
    // A11's cluster records follow the mate graph automatically, after
    // exactly the edits that can move it — read off the edit itself
    // ([`DocEdit::moves_the_mate_graph`]), so a new arm answers the
    // question or does not compile.
    let reconcile = edit.moves_the_mate_graph();
    // The report read at the door that made it: DM7's strands and the
    // declarations a delete orphaned, or the strands and the rebounds
    // a reshaped program made. Two edits fill this — `DeleteNode`,
    // the only edit that removes a node, and `SetProgram`, the only
    // edit that moves what a profile's segments are; `Rebind` moves
    // references onto a live name at the author's word and reports
    // nothing.
    let mut reported: Vec<Maintenance> = Vec::new();
    let record = match edit {
        DocEdit::InsertNode { node } => {
            // Liveness, and it stays spelled here rather than moving to
            // a shared home: the rule IS the node map's own lookup, so
            // the load door's `DanglingInput` walk and this loop share
            // `contains_key` already and have no predicate between them
            // to extract. What differs is the subject — one incoming
            // reference here, every edge a file claims there.
            for input in node.inputs() {
                if !new.nodes.contains_key(&input) {
                    return Err(EditError::UnresolvedInput { input });
                }
            }
            // Spec D3 carve-out (ruled): a payload's name refs must
            // point at LIVE nodes at edit time — a never-existed id is
            // a typo. They are not DAG edges: later deletes may strand
            // them (N5), so this is the ONLY door that checks, and it
            // checks every payload name — `Node::payload_names` is the
            // list.
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
            // ASM-R2a D-1, through `Node::has_non_finite_alignment` —
            // the one place a node is asked whether its alignment datum
            // is decidable, which the load door's walk asks too.
            if node.has_non_finite_alignment() {
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
            // The solve's own per-mate admission (A11 rule 1), asked
            // of the document the mate now stands in — its walks read
            // the operands there — through the reach this door holds:
            // a mate the coset table refuses on its own datum is
            // refused at this door. The verdicts about a PAIR stay the
            // solve's (`admit_mate`), and so does every STATE a mate
            // comes to hold after insert — this is the one door that
            // writes a mate's datum (`DocEdit::writes_a_mates_datum`),
            // and a reference a later edit strands is N5's. The
            // environment is the document's own nominal, built here
            // once for this door's reading, the way the evaluation
            // builds its own and hands it to the solve.
            debug_assert!(edit.writes_a_mates_datum() == matches!(node, Node::Mate { .. }));
            if matches!(node, Node::Mate { .. }) {
                let env = new.param_env::<f64>();
                crate::mate::solve::admit_mate(&new, id, node, &env, how.reach(), tol)
                    .map_err(|fault| EditError::MateRefused { node: id, fault })?;
            }
            EditRecord {
                minted: Some(id),
                structural: true,
            }
        }
        DocEdit::DeleteNode { id } => {
            if !new.nodes.contains_key(id) {
                return Err(EditError::UnknownNode { id: *id });
            }
            // Who reads this node is `roots`' question — the same
            // predicate the root set is maintained by, so the
            // refusal and the re-rooting below cannot disagree about
            // what a live consumer is.
            if let Some(referenced_by) = crate::roots::consumer(&new, *id) {
                return Err(EditError::DeleteWouldDangle {
                    id: *id,
                    referenced_by,
                });
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
            // DM7: a name of this node is not a DAG edge, so the
            // check above never saw one and the edit stands. What the
            // door owes is the report — every surviving reference
            // whose minting node just left, in both of the document's
            // carriers, read out of the document as it now stands.
            reported = stranded_references(&new, *id);
            // The declaration half of the same question, out of the
            // same post-removal document so the two reports cannot
            // disagree about which nodes are gone. Appended after the
            // strands, which is the order the field contracts. The
            // input list feeds both readers — this door and
            // `roots::on_delete` below — so the declaration reported
            // inert and the inputs re-rooted are read off one value.
            reported.extend(orphaned_declares(&new, &inputs));
            crate::roots::on_delete(&mut new, *id, &inputs);
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
            EditRecord {
                minted: None,
                structural: true,
            }
        }
        DocEdit::SetProgram {
            node,
            loops,
            provenance,
        } => {
            let payload = match new.nodes.get(node) {
                None => return Err(EditError::UnknownNode { id: *node }),
                Some(Node::Profile(p)) => p,
                Some(_) => return Err(EditError::SetProgramOnNonProfile { node: *node }),
            };
            let (Some(old_loops), Some(rewritten)) =
                (payload.loops(), payload.with_loops(loops.clone()))
            else {
                return Err(EditError::SetProgramOnNonProfile { node: *node });
            };
            // The provenance's shape FIRST: a provenance the door
            // could not honour makes the replay below moot, and the
            // caller mends the field named rather than the program.
            check_provenance(*node, old_loops, loops, provenance)?;
            // The rewritten node walks the insert door's own slot
            // checks — the same function, not a mirror: dimensions and
            // parameter references over every argument of every loop.
            let probe = Node::Profile(rewritten);
            check_node_slots(&new, *node, &probe)?;
            let Node::Profile(rewritten) = &probe else {
                unreachable!("the probe was built as a profile node two lines above")
            };
            // Then the VQ9 door the insert door runs, keeping what the
            // replay decided: the new program's per-step spans.
            let env = new.param_env::<f64>();
            let refused = |refusal| EditError::ProfileProgramRefused {
                node: *node,
                refusal: Box::new(refusal),
            };
            let (new_loops, new_records) = rewritten.check_returning(&env, tol).map_err(refused)?;
            // Read through DM8's checked door, so which segments each
            // authored step draws is answered by the one walk that
            // answers it at evaluation; a record the walk refuses is a
            // typed refusal of the edit, never a guess.
            let new_programs = rewritten
                .loops()
                .ok_or(EditError::SetProgramOnNonProfile { node: *node })?;
            let new_checked =
                checked_replay(new_programs, &new_loops, &new_records).map_err(refused)?;
            // The program being replaced, replayed under the same env
            // for ITS spans — the coordinates every name on it holds.
            // Validation is not asked of it: a program that replays
            // and does not validate (V1 class 2: refusing programs may
            // exist at rest) still has the spans its names were
            // published against. One that does not even replay, or
            // whose record the checked door refuses, has no readable
            // spans, so nothing can be mapped and every name on it
            // strands — reported, never guessed.
            let old_replayed = payload.replay_records(&env, tol).ok();
            let old_checked = old_replayed
                .as_ref()
                .and_then(|(loops, records)| checked_replay(old_loops, loops, records).ok());
            let map = SegmentMap::build(
                old_checked.as_deref(),
                old_loops.len(),
                &new_checked,
                provenance,
            )
            .map_err(refused)?;
            new.nodes.insert(*node, probe);
            // Whose locators are spelled in this profile's coordinates:
            // the sweeps over it. Every other node's names hold them
            // only inside carried names, which the walk descends.
            let anchored = new
                .order
                .iter()
                .copied()
                .filter(|id| {
                    new.nodes
                        .get(id)
                        .is_some_and(|n| n.anchoring_profile() == Some(*node))
                })
                .collect();
            reported = reshape_report(&mut new, &mut ProgramRemap::new(&map, anchored))?;
            // Structural whatever moved: the edit's class is a
            // rewrite of program structure — verbs, order, count —
            // and the record classifies the edit, as
            // `SetStructuralParam` is structural whether or not the
            // count it writes differs. An identity program is this
            // edit with nothing to do, not a different edit.
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
        DocEdit::SetDocParamDistribution { name, distribution } => {
            let Some(declared) = new.params.get(name) else {
                return Err(EditError::DocParamNotDeclared {
                    name: name.clone(),
                    door: CarryForwardDoor::Annotation,
                });
            };
            // THE carry-forward, over the third field: the declaration
            // is read off the document and reused whole, so the value
            // and the NOTATION cannot be dropped by an omission here.
            // Both reasons it can refuse are the DOOR's — this routes
            // them, and decides neither.
            let written = declared
                .with_distribution(*distribution)
                .map_err(|why| match why {
                    DistributionRefusal::CountHasNoAnnotation => {
                        EditError::DocParamCountHasNoDistribution { name: name.clone() }
                    }
                    DistributionRefusal::Invalid { fault } => distribution_fault_error(name, fault),
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
            // One-shot rewrite of every EXACT reference, at every
            // payload site — `Node::payload_names` is the list and
            // `Node::rebind_payload_names` is its rewriting twin, so
            // no carrier can be repaired here and missed there. The
            // appearance store is the document's OTHER carrier and is
            // rewritten below, not by this loop. Zero sites across
            // both = nothing to repair, refused.
            let mut declare_sites = 0usize;
            for node in new.nodes.values_mut() {
                declare_sites += node.rebind_payload_names(from, to);
            }
            // Appearance keys are rebind sites (the attribute rides
            // the name — PR 7's store; also the spec D9 banked
            // operand→final repair path). A per-kind collision with
            // an attribute already on `to` is refused loudly: which
            // value survives would be an auto-pick —
            // `move_appearance_record` is the one home for that rule.
            let mut appearance_sites = 0usize;
            if let Some(moved) = new.appearance.remove(from) {
                appearance_sites += 1;
                move_appearance_record(&mut new.appearance, moved, to)?;
            }
            if declare_sites + appearance_sites == 0 {
                return Err(EditError::RebindNoReferences { name: from.clone() });
            }
            EditRecord {
                minted: None,
                // A payload name changed: content keys move and the
                // threading consumes them — structural, whichever
                // carrier held it. An appearance-only rebind is
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
            // The recorded ε's admission rule, by the one predicate the
            // save/load validator also asks of a snapshot
            // (`crate::doc::epsilon_admissible`).
            if !crate::doc::epsilon_admissible(*eps) {
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
            // D7's producer convention, by the one predicate
            // `MetaValue::require_versioned`, which the save/load
            // validator also calls. Only the WALK differs between the
            // two doors, and irreducibly: this door holds the one value
            // it is about to write, and the validator holds a map that
            // arrived whole.
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
            // A11's admission rule for a registry row, asked of the one
            // predicate the load door's walk asks
            // (`crate::doc::placement_fault`); this is the edit door's
            // name for its answer.
            if let Some(fault) = crate::doc::placement_fault(&new, *node, frame) {
                return Err(match fault {
                    PlacementFault::NotAnInstance => {
                        EditError::PlacementOnNonInstance { node: *node }
                    }
                    PlacementFault::NonFiniteFrame => EditError::NonFinitePlacement { node: *node },
                    PlacementFault::ImproperFrame { determinant } => EditError::ImproperPlacement {
                        node: *node,
                        determinant,
                    },
                });
            }
            // A11: the record keys on the cluster, never the
            // instance. A singleton cluster's gauge IS the instance,
            // so a mate-less document's registry is unchanged. This is
            // also why no edit door asks the load door's GAUGE rule:
            // the key is normalised here rather than refused, and the
            // cluster maintenance re-keys the registry whenever the
            // mate graph moves.
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
    let mut maintenance = reported;
    if reconcile {
        maintenance.extend(
            crate::mate::solve::maintain(doc, &mut new, tol, how)?
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

/// A witness edit's site check: the store's key rule
/// ([`crate::doc::witness_site_fault`], the same question the load
/// door's walk asks of a file's store), rendered in this door's
/// vocabulary.
fn check_witness_site<P>(doc: &Doc<P>, id: RecipeNodeId) -> Result<(), EditError> {
    match crate::doc::witness_site_fault(doc, id) {
        None => Ok(()),
        Some(WitnessSiteFault::NoSuchNode) => Err(EditError::UnknownNode { id }),
        Some(WitnessSiteFault::NotSketchBearing) => Err(EditError::WitnessOnNonSketch { node: id }),
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
    // Whether the node HAS the slot is this door's own question: the
    // subject is an address a caller named, and `SetParam` aimed at a
    // radius on an extrude is a reachable mistake rather than the
    // node-layer invariant `Node::slot_dimension_fault` asserts.
    if node.expr(slot).is_none() {
        return Err(EditError::UnknownSlot { id, slot });
    }
    // D6's comparison, from its one home (`SlotId::dimension_fault`) —
    // the same rule the node-wide walk asks, of an expression the node
    // does not hold yet.
    if let Some(SlotDimensionFault {
        slot,
        expected,
        found,
    }) = slot.dimension_fault(expr)
    {
        return Err(EditError::SlotDimensionMismatch {
            slot,
            expected,
            found,
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
    /// Method form of [`apply`] (spec D2's pure edit entry point —
    /// pure over the document and the reach).
    pub fn apply(
        &self,
        edit: &DocEdit<P>,
        tol: Tol,
        reach: &dyn MateReach,
    ) -> Result<Applied<P>, EditError> {
        apply(self, edit, tol, reach)
    }

    /// Replay a logged edit list from the EMPTY document under the
    /// given identity (spec D7): the result reproduces the edits'
    /// document BIT-IDENTICALLY (floats are stored exactly; ids
    /// re-mint deterministically; the recorded cluster rows are
    /// re-applied, never re-solved — [`apply_logged`] — so no store is
    /// in hand and none is needed). The document id is supplied, not
    /// replayed: identity is authored data the log never carries
    /// (ASM-1 D-1).
    ///
    /// The answer is the document, not the maintenance its edits
    /// performed: [`Applied::maintenance`] is a fact about ONE
    /// application, reported to the caller who made it, and what it
    /// did is already in the document it produced — a registry act
    /// rewrote the registry (from the rows the log carries, which is
    /// what lets it be re-applied without a solve), and a stranded
    /// name (DM7) resolves to nothing until rebound, which the next
    /// evaluation reports typed (N5). The replayed document is the
    /// state, the same boundary the load door draws
    /// ([`crate::persist::Loaded`]); the round trip is lossless
    /// because the same delete against the replayed document reports
    /// the same strands, which DM7's round-trip row pins.
    ///
    /// # Errors
    ///
    /// [`apply_logged`]'s: an entry with no rows whose edit performs
    /// cluster maintenance refuses [`EditError::MaintenanceUnrecorded`]
    /// — the log claims no maintenance the edit in fact performed.
    pub fn replay(
        id: crate::DocumentId,
        log: &[LoggedEdit<P>],
        tol: Tol,
    ) -> Result<Doc<P>, EditError> {
        let mut doc = Doc::empty(id, tol);
        for entry in log {
            doc = apply_logged(&doc, entry, tol)?.doc;
        }
        Ok(doc)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::expect_used)]

    use super::DocEdit;
    use crate::program::ProfileProgram;

    /// **The mate-graph question is answered by the edit, not by the
    /// arm that happens to remember.**
    ///
    /// `apply` re-keys the A11 registry after exactly the edits
    /// [`DocEdit::moves_the_mate_graph`] answers `true` for, and that
    /// re-keying is what makes `SnapshotError::PlacementNotGauge`
    /// unreachable through the edit doors. This row names the four
    /// that move it — the instance set, the mate set, a list input and
    /// a rebound head — and names `SetPlacement` on the other side,
    /// because that is the edit whose row the reconciliation re-keys
    /// and the one a reader is most likely to expect here.
    ///
    /// A new arm cannot silently join the `false` side: the match has
    /// no wildcard, so it stops compiling until it is classified. What
    /// this row adds is that the four are classified CORRECTLY, which
    /// the compiler cannot say.
    #[test]
    fn exactly_the_graph_moving_edits_ask_for_reconciliation() {
        let id = crate::node::RecipeNodeId(1);
        let moves: [DocEdit<ProfileProgram>; 4] = [
            DocEdit::InsertNode {
                node: crate::node::Node::Datum(crate::node::Datum::Point {
                    position: [
                        crate::expr::Expr::literal(0.0, crate::expr::Dimension::Length)
                            .expect("finite"),
                        crate::expr::Expr::literal(0.0, crate::expr::Dimension::Length)
                            .expect("finite"),
                        crate::expr::Expr::literal(0.0, crate::expr::Dimension::Length)
                            .expect("finite"),
                    ],
                }),
            },
            DocEdit::DeleteNode { id },
            DocEdit::SetMembers {
                node: id,
                members: vec![],
            },
            DocEdit::Rebind {
                from: crate::names::StableName {
                    kind: crate::names::EntityKind::Face,
                    node: id,
                    path: vec![],
                },
                to: crate::names::StableName {
                    kind: crate::names::EntityKind::Face,
                    node: id,
                    path: vec![],
                },
            },
        ];
        for edit in &moves {
            assert!(
                edit.moves_the_mate_graph(),
                "{edit:?} changes the instance set, the mate set or a mate's head"
            );
        }
        let keyed_on_the_gauge: DocEdit<ProfileProgram> = DocEdit::SetPlacement {
            node: id,
            frame: crate::placement::Frame::IDENTITY,
        };
        assert!(
            !keyed_on_the_gauge.moves_the_mate_graph(),
            "SetPlacement writes the registry the reconciliation re-keys; it moves no reading edge"
        );
    }

    /// **The datum question is answered by the edit too**: the insert
    /// of a mate writes a mate's alignment datum and nothing else does
    /// — not the insert of another node, not the rebind that moves a
    /// head (a reference, not the datum), not the structural edit that
    /// shrinks a pattern under one. The per-mate admission is asked
    /// at exactly the edit this answers `true` for.
    #[test]
    fn exactly_the_mate_insert_writes_a_mates_datum() {
        let id = crate::node::RecipeNodeId(1);
        let name = |node| crate::names::StableName {
            kind: crate::names::EntityKind::Face,
            node,
            path: vec![],
        };
        let frame = crate::mate::MateFrame::authored([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
        let mate: DocEdit<ProfileProgram> = DocEdit::InsertNode {
            node: crate::node::Node::Mate {
                a: crate::node::SitedFace::at_mint(
                    crate::names::FaceName::new(name(id)).expect("a face"),
                ),
                b: crate::node::SitedFace::at_mint(
                    crate::names::FaceName::new(name(crate::node::RecipeNodeId(2)))
                        .expect("a face"),
                ),
                class: crate::mate::ContactClass::Rest,
                alignment: crate::mate::Alignment {
                    a: frame.clone(),
                    b: frame,
                    primitive: crate::mate::MatePrimitive::FrameCoincidence,
                    sense: crate::mate::AxisSense::Aligned,
                    clocking: None,
                },
            },
        };
        assert!(mate.writes_a_mates_datum());
        let others: [DocEdit<ProfileProgram>; 3] = [
            DocEdit::InsertNode {
                node: crate::node::Node::Datum(crate::node::Datum::Point {
                    position: [
                        crate::expr::Expr::literal(0.0, crate::expr::Dimension::Length)
                            .expect("finite"),
                        crate::expr::Expr::literal(0.0, crate::expr::Dimension::Length)
                            .expect("finite"),
                        crate::expr::Expr::literal(0.0, crate::expr::Dimension::Length)
                            .expect("finite"),
                    ],
                }),
            },
            DocEdit::Rebind {
                from: name(id),
                to: name(crate::node::RecipeNodeId(2)),
            },
            DocEdit::SetStructuralParam {
                node: id,
                slot: crate::node::SlotId::Count,
                expr: crate::expr::Expr::count(1),
            },
        ];
        for edit in &others {
            assert!(
                !edit.writes_a_mates_datum(),
                "{edit:?} writes no mate's datum; what it strands is the solve's"
            );
        }
    }
}
