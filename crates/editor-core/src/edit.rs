//! The DocEdit vocabulary v1 and the pure `apply` (spec D2 + D6).
//!
//! `apply` returns a NEW document value; the input is untouched.
//! Undo/redo is keeping prior values — no edit destroys history at
//! this layer (spec D2). `apply` is pure over the document AND the
//! reach it is handed ([`crate::mate::MateReach`]), which it asks for
//! one decision only: the clocking rider of a mate being inserted,
//! levered over the mated parts' own extent. No edit records a frame
//! (A11 (2)), so replay is the edits alone, applied with no reach
//! ([`apply_replayed`]): pure over the log (spec D7), and never a
//! solve.

use crate::appearance::{Attr, AttrKind};
use crate::distribution::{Distribution, DistributionFault};
use crate::doc::Reader;
use crate::doc::{
    DisplayUnitRefusal, DistributionRefusal, Doc, ExpansionFault, FreeValue, FreeVar,
    GaugeRefFault, NameCarrier, VarName, VarReadFault, WitnessSiteFault,
};
use crate::expr::Unlowered;
use crate::expr::{Dimension, DimensionError, Expr, ExprPath};
use crate::formula::{Formula, FreshFault, LowerFault, NameFault, SlotRoot};
use crate::mate::reach::MateReach;
use crate::meta::{MetaValue, MetaVersionError};
use crate::names::{EntityKind, ProfileEdgeRef};
use crate::node::{
    AssertionBoundFault, CountMismatch, Node, PlacementRuleFault, RecipeNodeId, SlotDimensionFault,
    SlotId, StableName, StepId,
};
use crate::placement::{FrameFault, FrameSite};
use crate::spoken::{SpokenName, SpokenNode, SpokenVar};
use crate::var::{FreshEntry, Var, VarDecl, VarDef, VarId, VarKind, VarRef, WrittenDef};
use crate::witness::{BranchCertification, WitnessDatum};
use geom_core::Tol;

/// **What [`DocEdit::SetParam`] writes into a slot**: a formula at a
/// scalar slot, a read at an operand ([`SlotId::kind`] says which a
/// slot takes). A read at a scalar slot is the formula of that one
/// variable.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SlotValue {
    /// An expression, lowered to the variable the slot reads.
    Formula(Formula),
    /// A read of a variable, as written.
    Read(crate::Operand),
}

impl From<Formula> for SlotValue {
    fn from(formula: Formula) -> Self {
        Self::Formula(formula)
    }
}

impl From<crate::Operand> for SlotValue {
    fn from(read: crate::Operand) -> Self {
        Self::Read(read)
    }
}

/// The recorded edit vocabulary (spec D6): a closed set of intents over
/// a document value, every arm plain data, applied by the pure
/// [`apply`] (spec D2), which answers a new document and leaves its
/// input untouched. The set has three shapes. Structural edits over
/// nodes, their slots and where instances sit
/// (`InsertNode`, `DeleteNode`, `SetMembers`, `SetProgram`,
/// `SetParam`, `SetStructuralParam`, `SetExpression`,
/// `SetOffset`, `SetGauge`, `Promote`, `Fold`, `UpdateReference`). The
/// variable family: `DeclareVar`, which mints a variable, `DefineVar`,
/// which replaces its definition, the carry-forward doors, each
/// moving ONE field of a free variable and keeping the rest
/// (`SetVarValue`, `SetVarUnit`, `SetVarDistribution`), `RenameVar`,
/// which gives or clears its name and moves no reader, and
/// `DeleteVar`, which removes it and leaves its readers unresolved
/// (VR7); [`CarryForwardDoor`] names each door after the declare in a
/// refusal. The explicit repairs
/// and the document's
/// presentation state: `Rebind`, the ONLY name repair — the
/// automatic-rebinding policy menu is empty by ratified decision
/// (NAMING-DESIGN N5); `ReWitness`/`ReWitnessBulk`, the recorded
/// witness adoption, never a silent write-back (SOLVER-DESIGN W4);
/// `SetTolerance`, the recorded ε; and the appearance and metadata
/// pairs (`SetAppearance`/`ClearAppearance`,
/// `SetAppearanceMeta`/`ClearAppearanceMeta`, spec D7), and
/// `SetLabel`, a node's label. Each arm's own
/// doc states what it does and what it refuses; every refusal is a
/// typed [`EditError`].
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(bound(
    serialize = "P::Authored: serde::Serialize",
    deserialize = "P::Authored: serde::Deserialize<'de>"
))]
pub enum DocEdit<P: crate::ProfilePayload> {
    /// Insert a node; the new [`RecipeNodeId`] is minted from the
    /// document's mint chain ([`crate::Mint`]) and returned in the
    /// [`EditRecord`]. Input refs must resolve to EXISTING nodes —
    /// which is also why insertion can never create a cycle.
    InsertNode {
        /// The node payload (data only, spec D3), boxed: a whole node,
        /// a mate's two frames included, is many times every other
        /// arm, and a history is a `Vec` of edits.
        node: Box<Node<P::Authored, Formula>>,
        /// The edit's fresh table (VR6): the variables it mints for its
        /// formulas to read as [`Formula::fresh`], entry by entry and
        /// before anything else it mints. An entry's definition may
        /// read the entries before it, and an entry two readers share
        /// carries a name (VR2).
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        fresh: Vec<FreshEntry>,
    },
    /// Delete a node. Refused while any live node holds it as an
    /// INPUT (typed, spec D3/D6); the id is never reused afterwards.
    ///
    /// A payload NAME of the node is not an input and does not refuse
    /// (DM7, the §0 carve-out): the edit is accepted and every name it
    /// stranded rides the record as a [`Maintenance::Strand`]. An
    /// appearance key is the same carve-out at the store instead of a
    /// payload, and rides it as a [`Maintenance::StrandedAppearance`].
    DeleteNode {
        /// The node to delete.
        id: RecipeNodeId,
    },
    /// **Replace a node's whole LIST input** (DM4) — a union's members,
    /// a loft's sections ([`Node::list_input`]): the slot door
    /// ([`DocEdit::SetParam`] with a [`SlotValue::Read`]) on a list. The new list is stated in full, so nothing
    /// is inferred about which of the old entries survived, moved or
    /// was meant.
    ///
    /// Every check the insert door makes of a node's reads is made
    /// here, of the REWRITTEN node, through the same functions: each
    /// entry's lowering at its seat's kind ([`EditError::UnresolvedInput`],
    /// [`EditError::SlotVarKind`]), acyclicity
    /// ([`EditError::WouldCycle`]), pairwise distinctness
    /// ([`EditError::DuplicateInput`]) and the list's own floor
    /// ([`EditError::TooFewMembers`]). A node with no list input
    /// refuses [`EditError::SetMembersOnNonList`].
    SetMembers {
        /// The node whose list is replaced.
        node: RecipeNodeId,
        /// The whole new list, in order (D9: the order is data), each
        /// entry lowered as [`DocEdit::SetParam`]'s read is.
        members: Vec<crate::Operand>,
    },
    /// **Replace a live Boolean's or Union's whole DECLARED PAIRS**
    /// ([`crate::DeclaredPair`]) — [`DocEdit::SetMembers`]'s shape: the
    /// new list is stated in full and nothing is inferred about the old
    /// one. An empty list clears the declaration.
    ///
    /// A declaration is a parameter, not an operand: it carries no
    /// material and mints no names, and its sites are the node's own
    /// operands. So this edit moves no DAG edge, and DM6's rule that no
    /// edit rewires a live node's inputs is untouched by it.
    ///
    /// The pairs are checked exactly as an insert checks them: names
    /// and sites live ([`EditError::DeclareNamesMissingNode`],
    /// [`EditError::NameStepNeverMinted`], [`EditError::ReadSiteMissingNode`]),
    /// each site one of the node's operands
    /// ([`EditError::DeclaredSiteNotAnOperand`]), and each name minted
    /// by a node the declaring node reads, directly or through what it
    /// reads ([`EditError::DeclaredNameNotUpstream`]). A node
    /// of any other kind refuses [`EditError::SetDeclareOnNonDeclaring`].
    SetDeclare {
        /// The Boolean or Union whose declaration is replaced.
        node: RecipeNodeId,
        /// The whole new list of declared pairs.
        #[serde(with = "crate::persist::kernel_wire::contact_class::pairs")]
        pairs: Vec<crate::DeclaredPair>,
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
    /// shape, and so is which of its steps are the old ones: `ids`
    /// gives each new step the [`StepId`] of the old step it KEEPS, or
    /// `None` for a step it adds, which the door mints from the
    /// document's mint chain (`names/README.md`, "N1, the profile
    /// pieces"). The door is told, never guesses. A kept id must be a
    /// step of the program the node holds and stand for one step
    /// ([`EditError::StepIdsRefused`]).
    ///
    /// Every check [`DocEdit::InsertNode`] makes of a profile is made
    /// here, of the REWRITTEN node, through the same functions: the
    /// slot walk (`check_node_slots` — dimensions and document
    /// parameter references over every argument of every loop), then
    /// resolve + replay + validate under the current parameter
    /// environment ([`EditError::ProfileProgramRefused`]). Before any
    /// of that the ids' shape is checked, so a program is never
    /// replayed for an edit that could not have been honoured.
    ///
    /// **The names.** A name on a profile piece spells the piece's
    /// step id and role, so a kept step's names keep denoting its
    /// pieces wherever the new program draws them: nothing is
    /// rewritten. A step the new program does not keep takes its id
    /// with it — the id is never minted again — so every name on it
    /// keeps its spelling, resolves `Vanished`, and is reported
    /// [`Maintenance::Strand`] / [`Maintenance::StrandedAppearance`]
    /// exactly as a delete reports it (DM7: the subject is the edit
    /// that removes a name's referent, of which the delete is one).
    /// So is a name on a kept step's piece that the new program does
    /// not draw under the current parameters and the old one did, or
    /// could not be replayed to say — a fillet inserted or moved before
    /// a leg takes the leg's segment (`names/README.md`, "Undrawn pieces
    /// vanish rather than alias").
    ///
    /// A program byte-identical to the current one, keeping every
    /// step, is legal and reports nothing.
    ///
    /// **The plane cannot be carried**, which is the whole of DM6's
    /// claim here, pinned where a claim about types belongs — the
    /// variant has no such field, so an edit that tried to move it
    /// does not compile:
    ///
    /// ```compile_fail,E0560
    /// let _: editor_core::DocEdit<editor_core::ProfileProgram> =
    ///     editor_core::DocEdit::SetProgram {
    ///         node: editor_core::RecipeNodeId::new(0, 1),
    ///         plane: editor_core::RecipeNodeId::new(0, 0),
    ///         loops: Vec::new(),
    ///         ids: Vec::new(),
    ///     };
    /// ```
    SetProgram {
        /// The profile node whose program is replaced.
        node: RecipeNodeId,
        /// The whole new program, outer loop first then holes in
        /// description order — every loop stated in full.
        loops: Vec<crate::program::LoopProgram<Formula>>,
        /// Per new loop, per step in program order: the id of the old
        /// step it keeps, or `None` for a new step.
        ids: Vec<Vec<Option<StepId>>>,
        /// The edit's fresh table (VR6): the variables it mints for its
        /// formulas to read as [`Formula::fresh`], entry by entry and
        /// before anything else it mints. An entry's definition may
        /// read the entries before it, and an entry two readers share
        /// carries a name (VR2).
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        fresh: Vec<FreshEntry>,
    },
    /// **The slot door**: replace a CONTINUOUS slot's value
    /// (Length/Angle/Scalar slots; spec D3's continuous parameters) or
    /// write an operand (DM6, "no edit infers a re-point").
    ///
    /// At a scalar slot the value is a formula, its dimension re-checked
    /// (a read there is the formula of one variable). At an operand
    /// ([`SlotId::Operand`]) it is a read, lowered to a variable of the
    /// slot's kind ([`SlotId::kind`]; [`EditError::SlotVarKind`]
    /// otherwise) that is live, and the rewritten node passes the checks
    /// the insert door makes of a node's reads: DM5's distinctness and
    /// acyclicity over reads. A placer's operand keeps the shape its
    /// output was minted with. An operand write reports, and never
    /// refuses, the payload names it strands ([`Maintenance::Strand`]);
    /// [`DocEdit::Rebind`] repairs them. [`DocEdit::SetMembers`] is this
    /// door on a list.
    SetParam {
        /// The node owning the slot.
        node: RecipeNodeId,
        /// The named slot (spec D5: never an index).
        slot: SlotId,
        /// What the slot is to hold.
        value: SlotValue,
        /// The edit's fresh table (VR6): the variables it mints for its
        /// formulas to read as [`Formula::fresh`], entry by entry and
        /// before anything else it mints. An entry's definition may
        /// read the entries before it, and an entry two readers share
        /// carries a name (VR2).
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        fresh: Vec<FreshEntry>,
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
        expr: Formula,
        /// The edit's fresh table (VR6): the variables it mints for its
        /// formulas to read as [`Formula::fresh`], entry by entry and
        /// before anything else it mints. An entry's definition may
        /// read the entries before it, and an entry two readers share
        /// carries a name (VR2).
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        fresh: Vec<FreshEntry>,
    },
    /// Set which side of its sketch plane an extrude goes toward — the
    /// one structural choice on a [`Node::Extrude`] that is not an
    /// expression, so it has its own arm rather than a slot. A node of
    /// another kind refuses [`EditError::SetExtrudeSideOnNonExtrude`].
    SetExtrudeSide {
        /// The extrude node.
        node: RecipeNodeId,
        /// The side it goes toward.
        #[serde(with = "crate::persist::kernel_wire::extrude_side")]
        side: crate::node::ExtrudeSide,
    },
    /// Replace the expression SUBTREE at an [`ExprPath`] (empty path =
    /// the whole slot), re-running dimension checks on rebuilt
    /// ancestors (spec D6).
    SetExpression {
        /// The subtree address.
        path: ExprPath,
        /// The replacement subtree.
        expr: Formula,
    },
    /// **Declare a variable** (VARIABLES-DESIGN VR1/VR2): mint its id
    /// from the document's mint chain and hold `name` beside it.
    /// Structural when the kind is `Count`.
    ///
    /// Refuses a name the document already holds
    /// ([`EditError::VarNameTaken`], naming the holder), and a
    /// definition no door may write (the checks every door that writes
    /// a definition runs: [`EditError::NonFiniteVar`],
    /// [`EditError::InvalidDistribution`],
    /// [`EditError::ContinuousVarCannotBeCount`],
    /// [`EditError::VarUnitMismatch`]).
    DeclareVar {
        /// The name, unique within the document.
        name: VarName,
        /// The definition; its kind is the variable's, for good.
        def: VarDecl,
    },
    /// **Replace a variable's definition**, keeping its identity, its
    /// name and its kind (VR3: a new kind is a new variable).
    ///
    /// Refuses a variable the document does not hold
    /// ([`EditError::UnknownVar`]), a definition of another kind
    /// ([`EditError::VarKindFixed`]), and the definition checks
    /// [`Self::DeclareVar`] runs.
    DefineVar {
        /// The variable.
        var: VarRef,
        /// The new definition.
        def: VarDecl,
        /// The edit's fresh table (VR6): the variables it mints for its
        /// formulas to read as [`Formula::fresh`], entry by entry and
        /// before anything else it mints. An entry's definition may
        /// read the entries before it, and an entry two readers share
        /// carries a name (VR2).
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        fresh: Vec<FreshEntry>,
    },
    /// Write a NEW VALUE into a free variable, keeping its definition:
    /// its kind, its notation and its optional distribution ride
    /// through untouched ([`FreeVar::with_value`]).
    ///
    /// The value-editing door, so a caller holding only a number — a
    /// panel, a gesture, a binding — has nothing to omit: assembling a
    /// whole [`FreeVar`] for [`Self::DefineVar`] from `(dim, value)`
    /// would delete any annotation the variable carried.
    ///
    /// Refuses a variable the document does not hold
    /// ([`EditError::UnknownVar`]) and a value of the other kind (a
    /// count for a continuous variable or the reverse,
    /// [`EditError::VarValueKindMismatch`]).
    SetVarValue {
        /// The variable.
        var: VarRef,
        /// The replacement value.
        value: FreeValue,
    },
    /// Write a new NOTATION onto a free variable, keeping its value
    /// and its optional distribution ([`FreeVar::with_display_unit`]).
    /// A notation change is not a redefinition — the argument, in
    /// full, is [`FreeVar::with_display_unit`]'s rustdoc.
    ///
    /// Refuses a variable the document does not hold
    /// ([`EditError::UnknownVar`]), a `Count`
    /// ([`EditError::VarCountHasNoUnit`] — a count is an integer
    /// and names no notation) and a unit that does not measure the
    /// variable's kind ([`EditError::VarUnitMismatch`]).
    SetVarUnit {
        /// The variable, which must not be a `Count`.
        var: VarRef,
        /// The notation to write, which must MEASURE the kind.
        unit: crate::expr::UnitSym,
    },
    /// Write an E1/E2 ANNOTATION onto a free variable, keeping its
    /// value and its notation ([`FreeVar::with_distribution`]).
    ///
    /// **`None` CLEARS the annotation**, through this same door; the
    /// argument is [`FreeVar::with_distribution`]'s rustdoc.
    ///
    /// Refuses a variable the document does not hold
    /// ([`EditError::UnknownVar`]), a `Count`
    /// ([`EditError::VarCountHasNoDistribution`]) and a
    /// distribution that breaks an E2 invariant
    /// ([`EditError::NonFiniteVar`],
    /// [`EditError::InvalidDistribution`] — the invariants the
    /// save/load validator refuses a document for).
    SetVarDistribution {
        /// The variable, which must not be a `Count`.
        var: VarRef,
        /// The annotation to write, or `None` to clear it.
        distribution: Option<Distribution>,
    },
    /// **Name, rename or unname a variable** (VR2): writes the name
    /// beside the variable and nothing else. Not structural, no
    /// `DocDiff` entry, nothing recomputes: a reader holds the id.
    ///
    /// Refuses a variable the document does not hold
    /// ([`EditError::UnknownVar`]), a name another variable holds
    /// ([`EditError::VarNameTaken`]), the name the variable already has
    /// ([`EditError::VarNameUnchanged`]), and clearing the name of a
    /// variable nothing reads ([`EditError::AnonymousVarUnread`]) or
    /// more than one reader reads ([`EditError::SharedVarNeedsName`]).
    RenameVar {
        /// The variable.
        var: VarRef,
        /// The new name, `None` to clear it.
        name: Option<VarName>,
    },
    /// **Delete a named variable** (VR7): it leaves the table and its
    /// name with it. Its readers are untouched, unresolved and typed:
    /// evaluation refuses at each, and the mint log keeps the id, so a
    /// later declare never mints it again. Structural when something
    /// read it.
    ///
    /// Refuses a variable the document does not hold
    /// ([`EditError::UnknownVar`]) and an anonymous one, which goes with
    /// its reader ([`EditError::DeleteAnonymousVar`]).
    DeleteVar {
        /// The variable.
        var: VarRef,
    },
    /// The explicit name repair (N5, spec D3), addressed by body and
    /// name (D10): rewrite every selection of `body` that names `from`
    /// EXACTLY to name `to`; with no body, every site no selection holds
    /// (declared pairs, mate heads, an instance's crossings, appearance
    /// keys). A name is scoped by the body it is read in (N1), so the
    /// body says which entity the repair is about.
    /// One-shot recorded intent — no alias table persists, nothing
    /// follows automatically afterwards (the ratified EMPTY policy
    /// menu). Validation mirrors Declare's edit-time carve-out: node
    /// existence NOW (`to`'s node must be live; `from`'s node must at
    /// least have once existed — a never-minted id is a typo);
    /// name-level resolution stays an evaluation-time concern.
    Rebind {
        /// The body whose selections are repaired, or `None` for the
        /// sites no selection holds.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        body: Option<VarId>,
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
    /// Validation mirrors declared pairs' ruled carve-out: the name's
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
    /// Set an instance's offset in its gauge (A11 (2)), or clear it
    /// with `None`. On its group's root the offset places the group;
    /// on any other member it is a statement the solve checks.
    SetOffset {
        /// The instantiate node.
        instance: RecipeNodeId,
        /// Its new offset, `None` for none.
        offset: Option<crate::placement::Placement<Formula>>,
        /// The edit's fresh table (VR6): the variables it mints for its
        /// formulas to read as [`Formula::fresh`], entry by entry and
        /// before anything else it mints. An entry's definition may
        /// read the entries before it, and an entry two readers share
        /// carries a name (VR2).
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        fresh: Vec<FreshEntry>,
    },
    /// Set the gauge a node sits on (A11 (2)): an instance's gauge or
    /// a gauge's parent, `None` for the world. The gauge must be live
    /// and must not sit on `node` through its own chain.
    ///
    /// A declaring mate whose two instances this puts on one gauge
    /// starts placing, and that is admitted: the regauge is what the
    /// caller named, and "these two now share a frame" is what a
    /// placing mate means. The edits that regauge as a side effect of
    /// something else ([`regauge_then_mate`], [`DocEdit::Fold`]) refuse
    /// it instead.
    SetGauge {
        /// The instance or gauge.
        node: RecipeNodeId,
        /// The gauge it now sits on, `None` for the world.
        gauge: Option<RecipeNodeId>,
    },
    /// **Turn an instance's offset into a gauge** (A4): a gauge under
    /// the instance's gauge, holding the instance's offset, with the
    /// instance on it at the empty chain. The other members of the
    /// instance's group move onto the gauge with it, so each mate that
    /// placed still places. The gauge's id is minted as its insert
    /// would mint it, and the record returns it; it joins the root
    /// list just ahead of the instance, or at the end when the
    /// instance is not a root. It moves no label. [`DocEdit::Fold`] is
    /// the inverse.
    ///
    /// Refuses a node that is no instance
    /// ([`EditError::PromoteOnNonInstance`]), an instance that is not
    /// the earliest member of its group carrying an offset
    /// ([`EditError::PromoteNonRoot`]), an instance whose group carries
    /// no offset at all ([`EditError::PromoteWithoutOffset`]), and a
    /// group whose other member carries an offset, stated in the
    /// instance's gauge and so not on the new one
    /// ([`EditError::PromoteMemberOffset`]).
    Promote {
        /// The instance whose offset becomes a gauge.
        instance: RecipeNodeId,
    },
    /// **Dissolve a gauge** (A4): every node on it hangs from its
    /// parent instead, and each one's own chain — a gauge's
    /// placement, an instance's offset when it has one — gets the
    /// gauge's steps in front ([`crate::placement::Placement::compose`]).
    /// No frame is computed, and every pose is unchanged bit for bit:
    /// a frame is folded one step at a time down the whole chain
    /// (`mate::solve::group_frame`), so where a step sits among the
    /// chain's placements does not change its arithmetic. An instance
    /// with no offset keeps none. The gauge goes as
    /// [`DocEdit::DeleteNode`] takes a node, with the same
    /// maintenance.
    ///
    /// **Labels.** With exactly one node on it and that node
    /// unlabelled, the node takes the gauge's label; otherwise the
    /// label goes, reported ([`Maintenance::LabelDropped`]). So
    /// `promote ∘ fold` is the identity up to node ids on a gauge with
    /// one dependent at the empty chain and no label, and
    /// `fold ∘ promote` is the identity.
    ///
    /// Refuses a node that is no gauge ([`EditError::FoldOnNonGauge`])
    /// and a fold that would put both
    /// instances of a declaring mate on one gauge, so that it would
    /// start placing ([`EditError::FoldWouldStartPlacing`]).
    Fold {
        /// The gauge that dissolves.
        gauge: RecipeNodeId,
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
    /// **Set or clear a node's label** (DESIGN.md Band 1, "Node
    /// labels"): `Some` replaces whatever label the node had, `None`
    /// clears it. The label is document data beside the node, so this
    /// edit moves no content key and recomputes nothing; it does move
    /// the content pin, as a recolour does.
    ///
    /// Refuses a node that is not live ([`EditError::UnknownNode`]),
    /// and an edit that would leave the label as it is
    /// ([`EditError::LabelUnchanged`]).
    ///
    /// A labelled creation is an [`DocEdit::InsertNode`] and then this
    /// edit, committed together: the insert carries no label, because
    /// what it carries is what the node's id is minted from.
    SetLabel {
        /// The node to label.
        node: RecipeNodeId,
        /// The new label, `None` to clear it.
        label: Option<crate::Label>,
    },
}

impl<P: crate::ProfilePayload> DocEdit<P> {
    /// **One copy of `body` in the world** (A10): the insert of a
    /// [`Node::PlaceInWorld`] reading it at `pose`, the identity when
    /// `None`. Nothing places but this; Python's `Doc.place` is the
    /// same edit.
    pub fn place(
        body: impl Into<crate::Operand>,
        pose: Option<crate::placement::Placement<Formula>>,
    ) -> Self {
        Self::InsertNode {
            node: Box::new(Node::place_in_world(
                body,
                pose.unwrap_or(crate::placement::Placement::IDENTITY),
            )),
            fresh: Vec::new(),
        }
    }

    /// **Whether this edit writes a mate's alignment datum** — the
    /// frames, the primitive, the sense and the rider the solve's
    /// per-mate admission decides on. Two edits do, and both ask the
    /// admission of the mate they wrote: the insert of a `Node::Mate`,
    /// and a slot edit (`SetParam`, `SetExpression`) addressed at one
    /// of a mate side's frame-offset steps ([`SlotId::MateFrameStep`],
    /// an address only a mate carries). A `Rebind` moves a reference's
    /// NAME (and, read at its own mint, its operand), never the datum,
    /// and what it strands is N5's — the solve's at evaluation; a
    /// document parameter an offset reads moves a STATE, which is the
    /// solve's at evaluation too.
    ///
    /// **Exhaustive, with no wildcard arm**: an arm added without an
    /// answer here stops the crate compiling rather than writing a
    /// datum past the admission.
    pub(crate) fn writes_a_mates_datum(&self) -> bool {
        match self {
            Self::InsertNode { node, .. } => matches!(&**node, Node::Mate { .. }),
            Self::SetParam { slot, .. } => slot.is_mate_frame_step(),
            Self::SetExpression { path, .. } => path.slot.is_mate_frame_step(),
            // A reshaping rewrites no name and never touches a datum;
            // a head whose piece it stops drawing is N5's, the solve's
            // at evaluation.
            Self::SetProgram { .. } => false,
            Self::DeleteNode { .. }
            | Self::SetMembers { .. }
            | Self::SetDeclare { .. }
            | Self::Rebind { .. }
            | Self::SetOffset { .. }
            | Self::SetGauge { .. }
            | Self::Promote { .. }
            | Self::Fold { .. }
            | Self::SetStructuralParam { .. }
            | Self::SetExtrudeSide { .. }
            | Self::DeclareVar { .. }
            | Self::DefineVar { .. }
            | Self::SetVarValue { .. }
            | Self::SetVarUnit { .. }
            | Self::SetVarDistribution { .. }
            | Self::RenameVar { .. }
            | Self::DeleteVar { .. }
            | Self::ReWitness { .. }
            | Self::ReWitnessBulk { .. }
            | Self::SetAppearance { .. }
            | Self::ClearAppearance { .. }
            | Self::SetTolerance { .. }
            | Self::SetAppearanceMeta { .. }
            | Self::ClearAppearanceMeta { .. }
            | Self::SetLabel { .. }
            | Self::UpdateReference { .. } => false,
        }
    }
}

/// **Where an expression an edit carries sits**, as the refusal of a
/// read in it names the address: a slot, or the node's payload (the
/// expressions no slot addresses).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExprSite {
    /// A slot expression.
    Slot(SlotId),
    /// A payload expression.
    Payload,
}

/// **A name the lowering left, as the door refuses it** at its
/// address: a name no variable holds is an unknown name; a name a
/// variable holds at another kind is that variable's kind fault, in the
/// words [`check_reads`] gives a stored reader of it.
fn name_refusal<P>(doc: &Doc<P>, node: SpokenNode, site: ExprSite, fault: NameFault) -> EditError {
    let NameFault { name, dim, why } = fault;
    match (site, why) {
        (ExprSite::Slot(slot), Unlowered::Unheld) => {
            EditError::SlotUnknownVarName { name, node, slot }
        }
        (ExprSite::Payload, Unlowered::Unheld) => EditError::PayloadUnknownVarName { name, node },
        (site, Unlowered::Kind { var, declared }) => read_refusal(
            doc,
            node,
            site,
            VarReadFault::Kind {
                var,
                declared,
                referenced: dim,
            },
        ),
    }
}

/// **A formula that does not lower, as this door refuses it**, at its
/// address: a name in [`name_refusal`]'s words, a fresh-table read in
/// its own.
fn lower_refusal<P>(
    doc: &Doc<P>,
    node: SpokenNode,
    site: ExprSite,
    fault: LowerFault,
) -> EditError {
    match fault {
        LowerFault::Name(fault) => name_refusal(doc, node, site, fault),
        LowerFault::Fresh(fault) => fresh_refusal(fault),
        LowerFault::Quantity { dim } => quantity_unminted(dim),
    }
}

/// D2 addendum row 4: the door mints a variable for every written
/// quantity before it lowers the formula holding it.
fn quantity_unminted(dim: Dimension) -> EditError {
    unreachable!("the door lowered a written {dim} it minted no variable for")
}

/// **The variables minted for `formula`'s written quantities**, one
/// anonymous free variable each, in pre-order (VR6): what
/// [`Formula::lower_with`] reads them as.
///
/// **Where they fall in the mint order is the one place two doors
/// differ, and for one reason: an id is minted from what the door
/// knows when it mints it.** An anonymous variable's id hashes what it
/// holds ([`crate::mint::Held`]), and a definition holds its
/// quantities' readers, so a slot's definition and a fresh entry's mint
/// their quantities first and the variable they define after them. A
/// declared variable's id hashes its kind alone, which the refusals of
/// a declare speak before its definition lowers
/// ([`crate::Doc::spoken_declare`]), so `DeclareVar` mints the declared
/// id first and its definition's quantities after it.
fn mint_quantities<P>(new: &mut Doc<P>, formula: &Formula) -> Result<Vec<VarId>, EditError> {
    formula
        .quantities()
        .into_iter()
        .map(|free| mint_anonymous(new, WrittenDef::Free(free)))
        .collect()
}

/// **A fresh-table read that does not resolve**, as this door refuses it.
fn fresh_refusal(FreshFault { index, dim, held }: FreshFault) -> EditError {
    match held {
        None => EditError::FreshUnheld {
            index,
            referenced: dim,
        },
        Some(held) => EditError::FreshKind {
            index,
            held,
            referenced: dim,
        },
    }
}

/// **Why one slot formula did not lower**, as the walk that lowers it
/// answers ([`Lowering::slot`]). The address is not the walk's to know:
/// [`lower_value`] finds it.
#[derive(Debug)]
enum SlotFault {
    /// A name or a fresh read that does not lower.
    Lower(LowerFault),
    /// A lowered read the document cannot answer.
    Read(VarReadFault),
    /// The mint's own refusal, which names a variable, not an address.
    Mint(EditError),
}

impl SlotFault {
    /// This fault refused at `site`.
    fn at<P>(self, doc: &Doc<P>, node: SpokenNode, site: ExprSite) -> EditError {
        match self {
            Self::Lower(fault) => lower_refusal(doc, node, site, fault),
            Self::Read(fault) => read_refusal(doc, node, site, fault),
            Self::Mint(refusal) => refusal,
        }
    }
}

/// **The door's lowering of one edit** (VARIABLES-DESIGN VR4, VR6):
/// the edit's fresh table, minted first, entry by entry, then every
/// formula it writes at a slot lowered to the one variable the slot
/// stores ([`Lowering::slot`]).
struct Lowering {
    /// The variables the fresh table minted, by entry, with each one's
    /// dimension.
    fresh: Vec<(VarId, Dimension)>,
    /// Whether the edit minted a defined variable, whose expansion the
    /// bound is asked of when the lowering closes.
    defined: core::cell::Cell<bool>,
}

impl Lowering {
    /// The lowering of an edit with no fresh table.
    fn none() -> Self {
        Self {
            fresh: Vec::new(),
            defined: core::cell::Cell::new(false),
        }
    }

    /// Mints the edit's fresh table into `new`: each entry a variable
    /// under the entry's name, or none, its definition lowered in the
    /// document's names and the entries before it.
    fn start<P>(new: &mut Doc<P>, fresh: &[FreshEntry]) -> Result<Self, EditError> {
        let mut minted = Self::none();
        for FreshEntry { name, decl } in fresh {
            let def = match decl {
                VarDecl::Free(free) => WrittenDef::Free(free.clone()),
                VarDecl::Defined(formula) => {
                    // A definition that does not lower draws no id (the
                    // anonymous draw reads what it holds), so a refusal
                    // of it speaks the entry by its kind's draw.
                    let spoken = SpokenVar::new(new.mint.would_declare(decl.kind()), None);
                    WrittenDef::Defined(minted.lower_definition(new, &spoken, formula)?)
                }
            };
            let dim = decl.dim();
            minted
                .defined
                .set(minted.defined.get() | matches!(def, WrittenDef::Defined(_)));
            if let Some(name) = name
                && let Some(holder) = new.var_named(name.as_str())
            {
                return Err(EditError::VarNameTaken {
                    name: name.clone(),
                    holder: new.spoken_var(holder),
                });
            }
            let var = mint_anonymous(new, def)?;
            if let Some(name) = name {
                new.var_names.insert(var, name.clone());
            }
            minted.fresh.push((var, dim));
        }
        Ok(minted)
    }

    /// A definition's formula lowered in `new`'s names and this edit's
    /// fresh table, each written quantity minted as an anonymous free
    /// variable first, a name that does not lower refusing in the words
    /// a faulty read of `var`'s definition takes.
    fn lower_definition<P>(
        &self,
        new: &mut Doc<P>,
        var: &SpokenVar,
        formula: &Formula,
    ) -> Result<Expr, EditError> {
        let refuse = |new: &Doc<P>, fault| match fault {
            LowerFault::Fresh(fault) => fresh_refusal(fault),
            LowerFault::Name(fault) => definition_name_refusal(new, var, fault),
            LowerFault::Quantity { dim } => quantity_unminted(dim),
        };
        if let Some(fault) = formula.unresolved(&|name| new.lowering_scope(name), &self.fresh) {
            return Err(refuse(new, fault));
        }
        let minted = mint_quantities(new, formula)?;
        formula
            .lower_with(&|name| new.lowering_scope(name), &self.fresh, &minted)
            .map_err(|fault| refuse(new, fault))
    }

    /// **Why a slot formula would not lower**, asked of nothing but the
    /// formula and the document: a name or a fresh read that does not
    /// lower, or a lowered read the document cannot answer. `None`
    /// where it lowers and every read holds. What [`lower_value`] asks
    /// of each addressed formula to find the one a walk refused.
    fn fault<P>(&self, doc: &Doc<P>, formula: &Formula) -> Option<SlotFault> {
        if let Some(fault) = formula.unresolved(&|name| doc.lowering_scope(name), &self.fresh) {
            return Some(SlotFault::Lower(fault));
        }
        doc.var_read_faults_of(formula.lowered_reads(&|name| doc.lowering_scope(name), &self.fresh))
            .into_iter()
            .next()
            .map(SlotFault::Read)
    }

    /// **The variable a slot formula lowers to** (spec §1's slot-root
    /// table): a lone variable, by id, name or fresh entry, is itself;
    /// a lone written value mints an anonymous free variable holding
    /// it; anything else mints an anonymous variable defined by the
    /// formula, lowered. A formula that does not lower, or reads what
    /// the document cannot answer, is refused before anything is
    /// minted for it.
    fn slot<P>(&self, new: &mut Doc<P>, formula: &Formula) -> Result<VarId, SlotFault> {
        let root = formula.slot_root();
        if let SlotRoot::Value(free) = root {
            return mint_anonymous(new, WrittenDef::Free(free)).map_err(SlotFault::Mint);
        }
        // Every fault is asked before anything is minted for the
        // formula's quantities: a formula that does not lower, or reads
        // what the document cannot answer, mints nothing.
        if let Some(fault) = self.fault(new, formula) {
            return Err(fault);
        }
        let minted = mint_quantities(new, formula).map_err(SlotFault::Mint)?;
        let expr = formula
            .lower_with(&|name| new.lowering_scope(name), &self.fresh, &minted)
            .map_err(SlotFault::Lower)?;
        match (root, expr.as_var()) {
            (SlotRoot::Value(_), _) => unreachable!("a value is minted above"),
            (SlotRoot::Formula, _) => {
                self.defined.set(true);
                mint_anonymous(new, WrittenDef::Defined(expr)).map_err(SlotFault::Mint)
            }
            // A lone variable leaf, by id, name or fresh entry, lowers
            // to a lone reader: the variable itself.
            (SlotRoot::Var(_) | SlotRoot::Name(..) | SlotRoot::Fresh(..), Some(var)) => Ok(var),
            (SlotRoot::Var(_) | SlotRoot::Name(..) | SlotRoot::Fresh(..), None) => {
                self.defined.set(true);
                mint_anonymous(new, WrittenDef::Defined(expr)).map_err(SlotFault::Mint)
            }
        }
    }

    /// **The edit's lowering, closed**, once what it wrote stands in
    /// `new`: every fresh entry is read ([`EditError::FreshUnread`]),
    /// and no definition it minted outgrows the expansion bound. The
    /// answer is the variables the fresh table minted, entry by entry.
    fn finish<P: crate::ProfilePayload>(self, new: &Doc<P>) -> Result<Vec<VarId>, EditError> {
        if !self.fresh.is_empty() {
            let readers = new.reader_counts();
            if let Some(index) = self
                .fresh
                .iter()
                .position(|(var, _)| readers.get(var).copied().unwrap_or(0) == 0)
            {
                return Err(EditError::FreshUnread {
                    index: u16::try_from(index).unwrap_or(u16::MAX),
                });
            }
        }
        // Nothing reads a variable minted here but this edit's own
        // formulas, so only a definition it minted can be the one that
        // outgrows the bound.
        if self.defined.get() {
            expansion_refusal(new)?;
        }
        Ok(self.fresh.into_iter().map(|(var, _)| var).collect())
    }
}

/// **An anonymous variable minted into `new`** (VR6): the id drawn
/// from the mint chain by its kind and what it holds
/// ([`crate::Mint`]'s anonymous draw), the definition checked as a
/// declare's is.
fn mint_anonymous<P>(new: &mut Doc<P>, def: WrittenDef) -> Result<VarId, EditError> {
    check_var_def(
        &SpokenVar::new(new.mint.would_declare_anonymous(&def), None),
        &def,
    )?;
    let id = new.mint.declare_anonymous(&def);
    new.vars.insert(id, Var::written(def));
    Ok(id)
}

/// **An authored value lowered in one walk**: `walk` maps every formula
/// the value holds to the variable its slot stores
/// ([`Lowering::slot`]), and its answer is the stored value. Where a
/// formula does not lower, the fault the refusal names is the first of
/// `rows` — the value's addressed formulas, in table order — that does
/// not ([`Lowering::fault`]), with its address; `None` for the address
/// where no row holds a fault, a formula the walk maps and the table
/// does not address (a count beside a listed placement rule, the one
/// such field: [`Node::exprs`]' edge), and the walk's own fault is
/// handed back for the caller to refuse typed. A refusal of the mint's
/// names a variable, not an address, and is the refusal as it stands.
fn lower_value<P, K: Copy, U>(
    new: &mut Doc<P>,
    lowering: &Lowering,
    rows: &[(K, &Formula)],
    walk: impl FnOnce(&mut dyn FnMut(&Formula) -> Result<VarId, SlotFault>) -> Result<U, SlotFault>,
) -> Result<U, WalkFault<K>> {
    match walk(&mut |formula| lowering.slot(new, formula)) {
        Ok(value) => Ok(value),
        Err(SlotFault::Mint(refusal)) => Err(WalkFault::Refused(refusal)),
        Err(fault) => Err(rows
            .iter()
            .find_map(|&(key, formula)| Some((key, lowering.fault(new, formula)?)))
            .map_or(WalkFault::Unaddressed(fault), |(key, fault)| {
                WalkFault::At(key, fault)
            })),
    }
}

/// **Why [`lower_value`] did not lower a value**, with the address
/// where it has one.
enum WalkFault<K> {
    /// The first addressed formula that does not lower, and why.
    At(K, SlotFault),
    /// A fault in a formula no row addresses.
    Unaddressed(SlotFault),
    /// The mint's refusal, as it stands.
    Refused(EditError),
}

impl<K> WalkFault<K> {
    /// This refusal, an addressed fault refused at `site(key)` and an
    /// unaddressed one by `unaddressed`.
    fn refuse<P>(
        self,
        doc: &Doc<P>,
        node: impl FnOnce() -> SpokenNode,
        site: impl FnOnce(K) -> ExprSite,
        unaddressed: impl FnOnce(SlotFault) -> EditError,
    ) -> EditError {
        match self {
            Self::At(key, fault) => fault.at(doc, node(), site(key)),
            Self::Unaddressed(fault) => unaddressed(fault),
            Self::Refused(refusal) => refusal,
        }
    }
}

/// **An authored node as the door writes it**: every slot, every payload
/// expression and every other formula field lowered to the variable it
/// stores in one walk ([`lower_value`]), a fault refusing at its
/// address, slots before payload in their table order. A formula no
/// address names is the count of a node whose placement rule takes
/// none, and the refusal is the rule's own
/// ([`EditError::PlacementRuleMismatch`]), the one the door gives that
/// node whatever its count holds. `spoken` speaks the node a refusal
/// names.
fn lower_node<P: crate::ProfilePayload>(
    new: &mut Doc<P>,
    lowering: &Lowering,
    node: &Node<P::Authored, Formula>,
    tol: Tol,
    spoken: impl Fn() -> SpokenNode,
) -> Result<Node<P>, EditError> {
    let rows: Vec<(ExprSite, &Formula)> = node
        .rows()
        .into_iter()
        .map(|(slot, formula)| (ExprSite::Slot(slot), formula))
        .chain(
            crate::node::payload_exprs(node)
                .into_iter()
                .flatten()
                .map(|formula| (ExprSite::Payload, formula)),
        )
        .collect();
    // The operands first, each against the document as it stands: the
    // slot lowering below mints only anonymous scalars, which no
    // operand reads.
    let mut reads = lower_reads(new, node, None, &spoken)?.into_iter();
    lower_value(new, lowering, &rows, |f| {
        node.try_map_slots(|p, g, r| P::lower(p, g, r), &mut |e| f(e), &mut |_, _| {
            Ok(reads
                .next()
                .unwrap_or_else(|| unreachable!("one read per operand, walked in one order")))
        })
    })
    .map_err(|unlowered| {
        unlowered.refuse(
            new,
            &spoken,
            |site| site,
            |fault| match node.placement_rule_fault(tol) {
                Some(PlacementRuleFault::CountSpelling { shape }) => {
                    EditError::PlacementRuleMismatch {
                        node: spoken(),
                        shape,
                    }
                }
                _ => fault.at(new, spoken(), ExprSite::Payload),
            },
        )
    })
}

/// **Every operand of an authored node as the door writes it**, in the
/// walk order of [`Node::try_map_slots`] ([`lower_operand`] each). A
/// placer re-pointed in place keeps its output's kind: `fixed` is that
/// kind, which its operand must have.
fn lower_reads<P: crate::ProfilePayload>(
    doc: &mut Doc<P>,
    node: &Node<P::Authored, Formula>,
    fixed: Option<VarKind>,
    spoken: &impl Fn() -> SpokenNode,
) -> Result<Vec<VarId>, EditError> {
    let half = match node {
        Node::Part {
            select: crate::PartSelect::SplitHalf(half),
            ..
        } => Some(*half),
        _ => None,
    };
    let mut reads = Vec::new();
    node.try_map_slots(
        |p, g, r| P::lower(p, g, r),
        &mut |_| Ok::<_, EditError>(VarId::new(0, 0)),
        &mut |slot, read| {
            let expected = match (slot, fixed) {
                (crate::OperandSlot::Input, Some(kind)) => crate::SlotKind::Is(kind),
                _ => slot.kind(),
            };
            let var = lower_operand(doc, spoken, SlotId::Operand(slot), read, half, expected)?;
            reads.push(var);
            Ok(var)
        },
    )?;
    Ok(reads)
}

/// **One operand as the door writes it** (DM6): the variable `read`
/// lowers to — a node named alone is its one output (spec Q5), a port
/// spelled out is that port's, a variable by id or name is itself —
/// refused unless it is live and of a kind the seat admits
/// (`expected`). A node with several outputs (a revolve's body and
/// axis, a split's two halves) refuses the sugar
/// ([`EditError::AmbiguousOutput`], naming its ports): the read spells
/// its port. A part projection's selected half (`half`) refuses a read
/// of the split's other half.
fn lower_operand<P: crate::ProfilePayload>(
    doc: &mut Doc<P>,
    spoken: &impl Fn() -> SpokenNode,
    slot: SlotId,
    read: &crate::Operand,
    half: Option<crate::SplitHalf>,
    expected: crate::SlotKind,
) -> Result<VarId, EditError> {
    lower_read(doc, spoken, slot, read, half, expected, Reader::Constructs)
}

/// [`lower_operand`] for a reader that `observes` its read
/// ([`Reader::Observes`]: a measure's selection, which may read a
/// world copy) or constructs from it.
fn lower_read<P: crate::ProfilePayload>(
    doc: &mut Doc<P>,
    spoken: &impl Fn() -> SpokenNode,
    slot: SlotId,
    read: &crate::Operand,
    half: Option<crate::SplitHalf>,
    expected: crate::SlotKind,
    reader: Reader,
) -> Result<VarId, EditError> {
    let unresolved = || EditError::OperandUnresolved {
        node: spoken(),
        slot,
        read: read.clone(),
    };
    let var = match read {
        crate::Operand::Node(id) => {
            if doc.node(*id).is_none() {
                return Err(EditError::UnresolvedInput {
                    input: SpokenNode::absent(*id),
                });
            }
            match doc.outputs(*id).as_slice() {
                [] => {
                    return Err(EditError::DefinesNothing {
                        input: doc.spoken(*id),
                        slot,
                    });
                }
                [one] => *one,
                _ => {
                    return Err(EditError::AmbiguousOutput {
                        input: doc.spoken(*id),
                        slot,
                        ports: doc
                            .signature(*id)
                            .unwrap_or_default()
                            .into_iter()
                            .map(|(name, _)| name)
                            .collect(),
                    });
                }
            }
        }
        crate::Operand::Output { node: id, port } => {
            if doc.node(*id).is_none() {
                return Err(EditError::UnresolvedInput {
                    input: SpokenNode::absent(*id),
                });
            }
            doc.output(*id, *port).ok_or_else(unresolved)?
        }
        crate::Operand::Var(var) => *var,
        crate::Operand::Name(name) => doc.var_named(name.as_str()).ok_or_else(unresolved)?,
        // Minted at the singleton of its names' kind where the seat
        // admits several, and then read like any variable: a measure's
        // primitive that does not admit that kind refuses here.
        crate::Operand::Select { body, names } => {
            mint_selection(doc, spoken, slot, body, names, expected)?
        }
    };
    check_read(doc, spoken, slot, var, half, expected, reader, unresolved)
}

/// **A selection authored at a seat, minted** (D10): the body read
/// lowered as a `Body` operand, the names checked live and in the
/// stored form of the seat's selection kind, and one anonymous
/// selection variable minted for the seat to read. A selection authored
/// at two seats is two variables.
fn mint_selection<P: crate::ProfilePayload>(
    doc: &mut Doc<P>,
    spoken: &impl Fn() -> SpokenNode,
    slot: SlotId,
    body: &crate::Operand,
    names: &[StableName],
    expected: crate::SlotKind,
) -> Result<VarId, EditError> {
    // A measure observes the copy's placed geometry, so its selection
    // may read a world copy; every construction seat may not.
    let reader = match expected {
        crate::SlotKind::Measured(_) => Reader::Observes,
        _ => Reader::Constructs,
    };
    let body = lower_read(
        doc,
        spoken,
        slot,
        body,
        None,
        crate::SlotKind::Is(VarKind::Body),
        reader,
    )?;
    let shape = |fault| EditError::SelectionShape {
        node: spoken(),
        slot,
        fault,
    };
    // A whole body named at a measure seat is a read of that body: the
    // name must be the body read's, its operation's or upstream of it.
    if let ([name], crate::SlotKind::Measured(_)) = (names, expected)
        && name.kind == EntityKind::Body
    {
        if !doc.nodes.contains_key(&name.node) {
            return Err(EditError::DeclareNamesMissingNode {
                name: doc.spoken_name(name),
            });
        }
        let on_body = doc.read_operation(body).is_some_and(|at| {
            at == name.node || crate::doc::strict_ancestors(doc, at).contains(&name.node)
        });
        if !on_body {
            return Err(shape(crate::var::SelectionFault::OtherBody));
        }
        return Ok(body);
    }
    let entity = names.first().map_or(EntityKind::Face, |n| n.kind);
    let kind = expected
        .selection_kind(entity)
        .ok_or_else(|| shape(crate::var::SelectionFault::Seat { entity }))?;
    if let Some(fault) = crate::var::Select::fault(kind, names) {
        return Err(shape(fault));
    }
    for name in names {
        if !doc.nodes.contains_key(&name.node) {
            return Err(EditError::DeclareNamesMissingNode {
                name: doc.spoken_name(name),
            });
        }
        check_name_steps(doc, doc, name)?;
    }
    mint_anonymous(
        doc,
        WrittenDef::Select(
            kind,
            crate::var::Select {
                body,
                names: names.to_vec(),
            },
        ),
    )
}

/// [`lower_operand`]'s checks of the variable an operand resolved to:
/// live, and then [`Doc::read_fault`] — the load door's rule too —
/// rendered in this module's vocabulary.
#[allow(clippy::too_many_arguments)] // the read and its seat, whole: one caller
fn check_read<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    spoken: &impl Fn() -> SpokenNode,
    slot: SlotId,
    var: VarId,
    half: Option<crate::SplitHalf>,
    expected: crate::SlotKind,
    reader: Reader,
    unresolved: impl Fn() -> EditError,
) -> Result<VarId, EditError> {
    let Some(held) = doc.var(var) else {
        return Err(unresolved());
    };
    match doc.read_fault(held, expected, half, reader) {
        None => Ok(var),
        Some(crate::doc::ReadFault::Kind { found }) => Err(EditError::SlotVarKind {
            var: Box::new(doc.spoken_var(var)),
            node: spoken(),
            slot,
            found,
            expected,
        }),
        Some(crate::doc::ReadFault::OtherHalf { half }) => Err(EditError::PartHalfPort {
            node: spoken(),
            half,
            var: Box::new(doc.spoken_var(var)),
        }),
        Some(crate::doc::ReadFault::WorldCopy { placement }) => Err(EditError::ReadsWorldCopy {
            node: spoken(),
            slot,
            placement: doc.spoken(placement),
        }),
    }
}

/// **One formula lowered into `doc` as a slot's**, outside any edit:
/// the variable the door would store for it, minted into `doc` where
/// the door would mint one. The test support's stored forms are built
/// through it, so a row that places a node by hand holds what the door
/// writes.
///
/// # Errors
///
/// A formula that does not lower, or reads what `doc` cannot answer.
#[doc(hidden)]
pub fn lower_slot_into<P>(doc: &mut Doc<P>, formula: &Formula) -> Result<VarId, EditError> {
    Lowering::none().slot(doc, formula).map_err(|fault| {
        fault.at(
            doc,
            SpokenNode::absent(RecipeNodeId::new(0, 0)),
            ExprSite::Payload,
        )
    })
}

/// **A selection minted into `doc` as given**, outside any edit: of the
/// kind a selection of `names` at `slot` is minted at, reading `body`,
/// with no door around it. The test support's shape rows build their
/// selections through it.
///
/// # Panics
///
/// If `slot` reads no selection of the names' entity kind.
#[doc(hidden)]
#[track_caller]
#[allow(clippy::expect_used)]
pub fn selection_into<P>(
    doc: &mut Doc<P>,
    slot: crate::OperandSlot,
    body: VarId,
    names: &[StableName],
) -> VarId {
    let entity = names.first().map_or(EntityKind::Face, |n| n.kind);
    let kind = slot
        .kind()
        .selection_kind(entity)
        .expect("a seat that reads a selection of these names");
    let def = WrittenDef::Select(
        kind,
        crate::var::Select {
            body,
            names: names.to_vec(),
        },
    );
    let id = doc.mint.declare_anonymous(&def);
    doc.vars.insert(id, Var::written(def));
    id
}

/// **One authored node lowered into `doc`**, outside any edit: what the
/// insert door would store for it — its operands each lowered at its
/// seat ([`lower_operand`]), its slots each through
/// [`lower_slot_into`] — minting into `doc` where the door would mint.
/// The test support's stored nodes are built through it.
///
/// # Errors
///
/// An operand or a formula that does not lower in `doc`.
#[doc(hidden)]
pub fn lower_node_into<P: crate::ProfilePayload>(
    doc: &mut Doc<P>,
    node: &Node<P::Authored, Formula>,
) -> Result<Node<P>, EditError> {
    let spoken = || SpokenNode::absent(RecipeNodeId::new(0, 0));
    let mut reads = lower_reads(doc, node, None, &spoken)?.into_iter();
    node.try_map_slots(
        |p, g, r| P::lower(p, g, r),
        &mut |f| lower_slot_into(doc, f),
        &mut |_, _| {
            Ok(reads
                .next()
                .unwrap_or_else(|| unreachable!("one read per operand, walked in one order")))
        },
    )
}

/// **A name a definition reads that does not lower**, as the doors
/// refuse it of `var`'s definition.
fn definition_name_refusal<P>(
    doc: &Doc<P>,
    var: &SpokenVar,
    NameFault { name, dim, why }: NameFault,
) -> EditError {
    match why {
        Unlowered::Unheld => EditError::DefinitionUnknownVarName {
            var: var.clone(),
            name,
        },
        Unlowered::Kind {
            var: read,
            declared,
        } => EditError::DefinitionVarKind {
            var: var.clone(),
            read: doc.spoken_var(read),
            declared,
            referenced: dim,
        },
    }
}

/// **A declared definition as the door writes it**: a free one as
/// handed, a defined one lowered in the document's names and the
/// edit's fresh table, a name that does not lower refusing in the words
/// [`check_definition`] gives a faulty read of `var`'s definition.
fn lower_decl<P>(
    doc: &mut Doc<P>,
    lowering: &Lowering,
    var: &SpokenVar,
    decl: &VarDecl,
) -> Result<WrittenDef, EditError> {
    match decl {
        VarDecl::Free(free) => Ok(WrittenDef::Free(free.clone())),
        VarDecl::Defined(formula) => lowering
            .lower_definition(doc, var, formula)
            .map(WrittenDef::Defined),
    }
}

/// **A `SetProgram`'s loops as the door writes them**, each lowered in
/// one walk ([`lower_value`]); the first that does not lower refuses
/// at the profile's address of its first argument, in program order,
/// that does not. An argument no row addresses refuses at the
/// profile's payload.
fn lower_loops<P>(
    new: &mut Doc<P>,
    lowering: &Lowering,
    node: SpokenNode,
    loops: &[crate::LoopProgram<Formula>],
) -> Result<Vec<crate::LoopProgram>, EditError> {
    let mut lowered = Vec::with_capacity(loops.len());
    for (li, lp) in loops.iter().enumerate() {
        let rows: Vec<(SlotId, &Formula)> = lp
            .rows()
            .into_iter()
            .map(|((step, arg), formula)| {
                let loop_ = crate::program::program_index(li);
                (SlotId::Profile { loop_, step, arg }, formula)
            })
            .collect();
        let lp = lower_value(new, lowering, &rows, |f| lp.try_map_slots(&mut |e| f(e))).map_err(
            |unlowered| {
                unlowered.refuse(
                    new,
                    || node.clone(),
                    ExprSite::Slot,
                    |fault| fault.at(new, node.clone(), ExprSite::Payload),
                )
            },
        )?;
        lowered.push(lp);
    }
    Ok(lowered)
}

/// **The reads of one expression an edit writes**, against the
/// document's table, in this door's vocabulary: every leaf must read a
/// live variable at its kind. The rule is `Doc::var_read_faults`, which
/// the load door reads too.
fn check_reads<P>(
    doc: &Doc<P>,
    node: &SpokenNode,
    site: ExprSite,
    expr: &Expr,
) -> Result<(), EditError> {
    let Some(fault) = doc.var_read_faults(expr).into_iter().next() else {
        return Ok(());
    };
    Err(read_refusal(doc, node.clone(), site, fault))
}

/// **A faulty read as this door refuses it**, at its address: the one
/// vocabulary [`check_reads`] and the lowering's kind faults
/// ([`name_refusal`]) share.
fn read_refusal<P>(
    doc: &Doc<P>,
    node: SpokenNode,
    site: ExprSite,
    fault: VarReadFault,
) -> EditError {
    match (site, fault) {
        (ExprSite::Slot(slot), VarReadFault::Unminted { var } | VarReadFault::Dead { var }) => {
            EditError::SlotUnresolvedVar {
                var: doc.spoken_var(var),
                node,
                slot,
            }
        }
        (ExprSite::Payload, VarReadFault::Unminted { var } | VarReadFault::Dead { var }) => {
            EditError::PayloadUnresolvedVar {
                var: doc.spoken_var(var),
                node,
            }
        }
        (
            ExprSite::Slot(slot),
            VarReadFault::Kind {
                var,
                declared,
                referenced,
            },
        ) => EditError::SlotVarKind {
            var: Box::new(doc.spoken_var(var)),
            node,
            slot,
            found: declared,
            expected: crate::SlotKind::Is(VarKind::from(referenced)),
        },
        (
            ExprSite::Payload,
            VarReadFault::Kind {
                var,
                declared,
                referenced,
            },
        ) => EditError::PayloadVarKind {
            var: doc.spoken_var(var),
            node,
            declared,
            referenced,
        },
    }
}

/// Which door an edit of a STANDING variable came through: the three
/// carry-forward edits, which write one field of a definition and carry
/// the rest untouched; [`DocEdit::DefineVar`], which replaces the
/// definition whole and carries only the variable's identity, name and
/// kind; and [`DocEdit::RenameVar`] and [`DocEdit::DeleteVar`], which
/// touch no definition. The type keeps its carry-forward name for the
/// three it was minted for.
///
/// It exists so a refusal every such door shares can name the one the
/// caller actually used ([`EditError::UnknownVar`]). A door over a
/// further field adds an arm here and the compile names every sentence
/// that has to learn the word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarryForwardDoor {
    /// [`DocEdit::DefineVar`] — the whole definition replaced, keeping
    /// the identity, the name and the kind. Not a carry-forward: nothing
    /// of the old definition survives it.
    Definition,
    /// [`DocEdit::SetVarValue`] — the number.
    Value,
    /// [`DocEdit::SetVarUnit`] — the notation.
    Notation,
    /// [`DocEdit::SetVarDistribution`] — the E1/E2 annotation.
    Annotation,
    /// [`DocEdit::RenameVar`] — the name, which is not the definition.
    Rename,
    /// [`DocEdit::DeleteVar`] — the variable itself.
    Delete,
}

// The door as it appears inside a refusal's sentence, in the user's
// vocabulary rather than the enum's: "a value edit", not "Value".
impl core::fmt::Display for CarryForwardDoor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Definition => "a definition edit",
            Self::Value => "a value edit",
            Self::Notation => "a notation edit",
            Self::Annotation => "an annotation edit",
            Self::Rename => "a rename",
            Self::Delete => "a delete",
        })
    }
}

/// **The one recourse for a variable the document does not hold**,
/// and the only home of its wording: a deleted one, or an address that
/// names none.
///
/// One mistake reaches two doors. Typing into the value field of a
/// variable the document no longer holds reaches a carry-forward edit,
/// which refuses [`EditError::UnknownVar`]; dragging that variable's
/// row is a lookup with no edit behind it, and the viewer refuses
/// `Refusal::NoSuchVariable` (`crates/viewer/src/session/refuse.rs`, the
/// second reader of this const and the only one outside this crate).
/// The two are converged on the RECOURSE and not on the sentence,
/// because a drag has no refused edit to report and a sentence that
/// borrowed the door's frame would report a refusal of something
/// nobody attempted. What is converged is what the user must DO, so
/// it is written once here and rendered twice. It is not "declare it":
/// a declare mints a new variable, which nothing addressed by the old
/// id reads.
pub const UNKNOWN_VAR_RECOURSE: &str = "pick a variable the document holds";

/// **The most expression nodes a variable's expansion may hold**: a
/// coincidence token writes a defined variable's definition expanded
/// through every definition it reads, so a diamond of definitions would
/// grow it exponentially without a bound. The doors that write a
/// definition refuse past it ([`EditError::DefinitionTooLarge`]).
pub const DEFINITION_NODE_BOUND: usize = 4096;

/// **The sentence of a definition cycle**, rendered once for the edit
/// door ([`EditError::DefinitionCycle`]) and the load walk
/// ([`crate::persist::SnapshotError::DefinitionCycle`]): one fact at
/// two doors.
pub(crate) struct DefinitionCycleSentence<'a> {
    pub(crate) var: &'a SpokenVar,
    pub(crate) through: &'a [SpokenVar],
}

impl core::fmt::Display for DefinitionCycleSentence<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let var = self.var;
        write!(f, "the definition of {var} reads {var} back, through ")?;
        for (i, held) in self.through.iter().enumerate() {
            if i > 0 {
                f.write_str(" → ")?;
            }
            write!(f, "{held}")?;
        }
        write!(f, " → {var}")
    }
}

/// **The sentence of an expansion past [`DEFINITION_NODE_BOUND`]**,
/// rendered once for both doors, as [`DefinitionCycleSentence`].
pub(crate) struct DefinitionTooLargeSentence<'a> {
    pub(crate) var: &'a SpokenVar,
    pub(crate) nodes: usize,
}

impl core::fmt::Display for DefinitionTooLargeSentence<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} expands, through the definitions it reads, to {} expression nodes, past the \
             bound of {DEFINITION_NODE_BOUND}",
            self.var, self.nodes
        )
    }
}

/// **The sentence of a definition reading a variable at another
/// kind**, rendered once for both doors, as [`DefinitionCycleSentence`].
pub(crate) struct DefinitionVarKindSentence<'a> {
    pub(crate) var: &'a SpokenVar,
    pub(crate) read: &'a SpokenVar,
    pub(crate) declared: VarKind,
    pub(crate) referenced: Dimension,
}

impl core::fmt::Display for DefinitionVarKindSentence<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} is declared {} but the definition of {} reads it as {}",
            self.read, self.declared, self.var, self.referenced
        )
    }
}

/// Typed, specific edit refusal (spec D6: no stringly errors).
///
/// **The variable-read naming convention is stated here and nowhere
/// else.** `Doc::var_read_faults` answers what is wrong with an
/// expression's variable leaves, and the edit door refuses THREE facts
/// about a leaf it writes — a name no variable holds, a kind the
/// variable does not have, a variable the document does not hold — at
/// TWO addresses: a slot expression and a payload expression (a
/// measured expression's value leaf, an assertion's bound — the
/// expressions no slot addresses). Those six meanings are named as the
/// product `{Slot,Payload}` x `{UnknownVarName,VarKind,UnresolvedVar}`:
/// the ADDRESS leads, the FACT trails. The load door's kind facts
/// ([`crate::SnapshotError::SlotVarKind`],
/// [`crate::SnapshotError::PayloadVarKind`]) carry the SAME names,
/// because the address is what the walk iterates and the fact is what
/// the rule answers. The guard is
/// `display_contract::the_two_doors_spell_the_var_read_refusals_the_same_way_and_each_reports_its_address`,
/// which measures both halves: the names per door, and that each arm's
/// address word is the address its sentence reports.
///
/// **Its SCOPE is a variable READ, and there are two families.** Those
/// arms name facts about a read AT an address, which is why the address
/// can lead. This enum's other variable refusals are about the
/// variable's DEFINITION or NAME — [`EditError::VarUnitMismatch`],
/// [`EditError::VarValueKindMismatch`],
/// [`EditError::VarCountHasNoUnit`],
/// [`EditError::VarCountHasNoDistribution`],
/// [`EditError::ContinuousVarCannotBeCount`],
/// [`EditError::VarKindFixed`], [`EditError::UnknownVar`],
/// [`EditError::VarNameTaken`], [`EditError::VarNameUnchanged`],
/// [`EditError::AnonymousVarUnread`], [`EditError::SharedVarNeedsName`]
/// and [`EditError::DeleteAnonymousVar`] — and those have no address: the
/// variable IS the subject, so each is named by its FACT alone.
/// ([`EditError::UnknownVar`]'s `door` says which edit was refused —
/// which edit, not where a read sits.)
///
/// [`crate::expr::EvalError::VarKindMismatch`] and
/// [`crate::expr::EvalError::UnresolvedVar`] are the facts raised at
/// EVALUATION instead of at a door, and keep their own names because
/// the split lands elsewhere there: the arm carries the fact and the
/// WRAPPER carries the address — [`crate::eval::NodeErrorKind::Expr`]
/// names a node and a slot, [`crate::eval::NodeErrorKind::PayloadExpr`]
/// names a node and a payload, and both forward the refusal unaltered.
///
/// **Which family a new arm joins is decided by what it refuses**, a
/// reference or a declaration — never by the words already in its
/// name. A sweep by SHAPE misses half of the declaration family:
/// [`EditError::VarCountHasNoUnit`] and its siblings carry no
/// `Mismatch` in them, so sweep by SUBJECT (`FreeVar`, `Var`) too.
///
/// Every other mention of the convention in this tree cites this
/// paragraph instead of re-wording it.
#[derive(Debug, Clone, PartialEq)]
pub enum EditError {
    /// The edit targets a node id that is not live.
    UnknownNode {
        /// The missing id.
        id: SpokenNode,
    },
    /// An edit that CREATES or MODIFIES a profile program failed the
    /// authoring-time check (LIB-SWITCH §4d, VQ9): the program is
    /// resolved, replayed, and validated under the CURRENT parameter
    /// environment at the edit door, so the author sees refusals at
    /// the verb, not at first evaluation. A variable edit deliberately
    /// NEVER takes this door — a parameter edit that breaks a
    /// downstream profile surfaces as that node's typed evaluation
    /// error (V1 class 2: refusing programs may exist at rest); both
    /// directions are pinned by test.
    ProfileProgramRefused {
        /// The profile node (for `InsertNode`, the id being minted).
        node: SpokenNode,
        /// The typed refusal, behind a pointer: it is this enum's
        /// widest payload, and every edit door returns the enum BY
        /// VALUE, so held inline it sets the width of every `Result`
        /// in the edit vocabulary and of the persist and replay
        /// refusals that wrap one. `AssemblyError::Product` carries
        /// `ProductError` the same way for the same reason.
        refusal: Box<crate::program::ProgramRefusal>,
    },
    /// An operand an edit writes names a node that is not live (spec
    /// D3: `apply` rejects unresolvable refs).
    UnresolvedInput {
        /// The dangling upstream reference.
        input: SpokenNode,
    },
    /// A read an edit writes into a slot names a variable the document
    /// does not hold — a deleted one, an id it never minted, a name nothing
    /// holds — or a port the node's signature lacks (DM6: the read is
    /// live).
    OperandUnresolved {
        /// The reading node.
        node: SpokenNode,
        /// The slot.
        slot: SlotId,
        /// What it reads, as written.
        read: crate::Operand,
    },
    /// A read names a node by itself, and the node defines several
    /// outputs (a revolve's body and axis, a split's two halves): the
    /// node-alone sugar is its one output only (spec Q5), so the read
    /// names its port ([`crate::Operand::Output`]).
    AmbiguousOutput {
        /// The node named.
        input: SpokenNode,
        /// The slot.
        slot: SlotId,
        /// The node's ports, by name, in port order.
        ports: Vec<&'static str>,
    },
    /// A read names a node that defines nothing to read: an
    /// assertion, a mate or a gauge.
    DefinesNothing {
        /// The node named.
        input: SpokenNode,
        /// The slot.
        slot: SlotId,
    },
    /// A part projection over a split reads the split's other half: the
    /// read and the selection name one half.
    PartHalfPort {
        /// The part projection.
        node: SpokenNode,
        /// The half it selects.
        half: crate::SplitHalf,
        /// The output it reads, boxed so the refusal stays a small `Err`.
        var: Box<SpokenVar>,
    },
    /// A construction seat reads a world placement's copy (D10:
    /// construction never reads the world; a measure observes a copy's
    /// placed geometry), so a slot reads the body the placement reads.
    ReadsWorldCopy {
        /// The reading node.
        node: SpokenNode,
        /// The slot.
        slot: SlotId,
        /// The placement whose copy it reads.
        placement: SpokenNode,
    },
    /// The recipe graph would contain a cycle (defensive: insertion
    /// referencing only pre-existing nodes cannot cycle, but the
    /// invariant is CHECKED, not assumed — spec D3).
    WouldCycle {
        /// A node on the detected cycle.
        at: SpokenNode,
    },
    /// **A node's inputs are not pairwise distinct** (DM5): one node
    /// reached twice through one node's edges.
    ///
    /// It is one structural rule over a node's operand reads
    /// ([`Node::operand_rows`]), not a rule per node kind, so it covers
    /// a boolean whose two operands coincide and a list with a repeated
    /// entry alike — and it is stated once, at [`Node::input_fault`],
    /// with this door,
    /// [`DocEdit::SetMembers`] and the load validator as its three
    /// callers.
    DuplicateInput {
        /// The node whose input list repeats.
        node: SpokenNode,
        /// The input it reaches twice.
        input: SpokenNode,
    },
    /// A selection this edit authors at `slot` is not one the
    /// document can store ([`crate::var::SelectionFault`]): names of an
    /// entity kind the seat reads no selection of, a singleton not
    /// naming one entity, or a set out of its stored order (edges sorted and
    /// deduplicated, faces each once). The construction doors
    /// ([`Node::fillet`], [`Node::chamfer`], [`Node::shell`]) write the
    /// stored order; a selection that arrives without it is refused
    /// rather than repaired, because a repair would move the content key
    /// behind the caller's back.
    SelectionShape {
        /// The node whose seat the selection is authored at.
        node: SpokenNode,
        /// The seat.
        slot: SlotId,
        /// Why.
        fault: crate::var::SelectionFault,
    },
    /// `SetMembers` aimed at a node that has no list input
    /// ([`Node::list_input`]) — a boolean's operands are named slots,
    /// and replacing "the list" of a node that has none is not a
    /// smaller version of this edit, it is a different sentence.
    SetMembersOnNonList {
        /// The node that carries no list.
        node: SpokenNode,
    },
    /// `SetDeclare` aimed at a node that declares no contacts — only a
    /// [`Node::Boolean`] and a [`Node::Union`] carry declared pairs.
    SetDeclareOnNonDeclaring {
        /// The node that carries no declaration.
        node: SpokenNode,
    },
    /// A declared pair's side is READ AT a node that is not one of the
    /// declaring node's operands ([`crate::DeclaredPair`], DM4): the
    /// site is the side, and a site the node does not have is a table
    /// it cannot read the name in. Asked by every door that writes a
    /// pair — the insert door, `SetDeclare` and `Rebind`.
    DeclaredSiteNotAnOperand {
        /// The node whose declaration it is.
        node: SpokenNode,
        /// The side's name.
        name: SpokenName,
        /// The node the side is read at.
        site: SpokenNode,
    },
    /// A declared pair's name is minted by a node the declaring node
    /// does not read, directly or through what it reads — itself, a
    /// node downstream of it, or one beside it: a declaration names
    /// only what its operands could hold ([`crate::DeclaredPair`]),
    /// decided by the read relation alone (D10). Asked by the same
    /// doors as [`EditError::DeclaredSiteNotAnOperand`].
    DeclaredNameNotUpstream {
        /// The node whose declaration it is.
        node: SpokenNode,
        /// The name.
        name: SpokenName,
    },
    /// `SetProgram` aimed at a node that holds no profile program: a
    /// node of another kind, or a `Node::Profile` over a payload that
    /// carries no program ([`crate::ProfilePayload::loops`] answers
    /// `None` — the `Doc<P>` test payloads). Replacing "the program"
    /// of a node that has none is a different sentence, not a smaller
    /// version of this edit.
    SetProgramOnNonProfile {
        /// The node that holds no program.
        node: SpokenNode,
    },
    /// A [`DocEdit::SetExtrudeSide`] aimed at a node that is not an
    /// extrude.
    SetExtrudeSideOnNonExtrude {
        /// The node that has no side.
        node: SpokenNode,
    },
    /// A program's step ids were refused (`names/README.md`, "N1, the
    /// profile pieces"): an `InsertNode` program carrying ids of its
    /// own, or a `SetProgram` whose ids do not have the new program's
    /// shape, keep a step the node does not hold, or keep one twice.
    /// Checked before the program is replayed.
    StepIdsRefused {
        /// The profile node.
        node: SpokenNode,
        /// What is wrong with the ids.
        fault: crate::program::StepIdFault,
    },
    /// A list input left with fewer than two entries. A union of one
    /// body is that body and a loft through one section is not a skin:
    /// either is a node whose meaning is its own input, spelled as an
    /// operator.
    TooFewMembers {
        /// The node whose list is short.
        node: SpokenNode,
        /// How many entries it would have had.
        found: usize,
    },
    /// The node does not carry the named slot.
    UnknownSlot {
        /// The node.
        id: SpokenNode,
        /// The slot it lacks.
        slot: SlotId,
    },
    /// The expression's dimension does not match the slot's required
    /// dimension (checks re-run on every touched expression, spec D6),
    /// or the slot is an operand, which holds a read and no expression.
    SlotDimensionMismatch {
        /// The slot.
        slot: SlotId,
        /// What the slot takes: its dimension's kind, or an operand's
        /// kind, which no expression has.
        expected: crate::SlotKind,
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
    /// A SLOT expression the edit writes reads a name the document
    /// does not hold. First of the six variable-read arms at this door,
    /// which sit together and are named under the convention this
    /// enum's own doc states.
    SlotUnknownVarName {
        /// The name.
        name: VarName,
        /// The reading node.
        node: SpokenNode,
        /// The reading slot.
        slot: SlotId,
    },
    /// A slot the edit writes reads a variable of a kind it does not
    /// admit: an expression reads one at another dimension than its kind
    /// (by a name, or by a reader whose cached kind disagrees with the
    /// table), or an operand reads one its field does not take (DM6:
    /// the read is of the slot's kind) — a placer's operand re-pointed
    /// at the other shape among them, whose output's kind was fixed at
    /// minting.
    SlotVarKind {
        /// The variable, boxed so the refusal stays a small `Err`.
        var: Box<SpokenVar>,
        /// The reading node.
        node: SpokenNode,
        /// The reading slot.
        slot: SlotId,
        /// The kind of the variable read.
        found: VarKind,
        /// What the read there takes: at an operand, the slot's kind;
        /// at an expression, the kind of the dimension its leaf reads
        /// the variable at — the slot's own at the formula's root, and
        /// the leaf's inside a function (`sin(w)` at a length slot
        /// reads `w` as an angle).
        expected: crate::SlotKind,
    },
    /// A SLOT expression the edit writes reads a variable the document
    /// does not hold: a deleted one, or an id it never minted.
    SlotUnresolvedVar {
        /// The variable read.
        var: SpokenVar,
        /// The reading node.
        node: SpokenNode,
        /// The reading slot.
        slot: SlotId,
    },
    /// A PAYLOAD expression — a measured expression's value leaf, an
    /// assertion's bound, the expressions no slot addresses — reads a
    /// name the document does not hold. The address is the NODE rather
    /// than a slot, so the arm says so instead of borrowing a slot name
    /// from a node that has one.
    PayloadUnknownVarName {
        /// The name.
        name: VarName,
        /// The reading node.
        node: SpokenNode,
    },
    /// A PAYLOAD expression reads a variable at another dimension than
    /// its kind.
    PayloadVarKind {
        /// The variable.
        var: SpokenVar,
        /// The reading node.
        node: SpokenNode,
        /// The dimension the variable's kind reads at.
        declared: VarKind,
        /// The dimension the expression reads it at.
        referenced: Dimension,
    },
    /// A PAYLOAD expression reads a variable the document does not
    /// hold.
    PayloadUnresolvedVar {
        /// The variable read.
        var: SpokenVar,
        /// The reading node.
        node: SpokenNode,
    },
    /// A [`Node::Assertion`]'s bound is dimensioned differently from
    /// the value it bounds — refused at the edit door, so a document
    /// never carries a comparison of metres with radians.
    AssertionDimension {
        /// The assertion.
        node: SpokenNode,
        /// The value's dimension.
        measured: Dimension,
        /// The bound's.
        bound: Dimension,
    },
    /// **A construction's slot reads an observed variable** (D10): a
    /// measure's output, or a definition reading one, directly or
    /// through another definition. Only an assertion reads one; a
    /// construction reads what was written.
    ConstructionReadsObserved {
        /// The reading node.
        node: SpokenNode,
        /// The slot that reads it.
        slot: SlotId,
        /// The variable the slot reads.
        var: Box<SpokenVar>,
    },
    /// A `Continuous` free variable defined with `Dimension::Count` —
    /// a count is [`FreeVar::Count`] (an exact integer).
    ContinuousVarCannotBeCount {
        /// The variable.
        var: SpokenVar,
    },
    /// An edit of a standing variable — [`DocEdit::DefineVar`],
    /// [`DocEdit::SetVarValue`], [`DocEdit::SetVarUnit`] or
    /// [`DocEdit::SetVarDistribution`] — addressed a variable this
    /// document does not hold. Declaring one is
    /// [`DocEdit::DeclareVar`]'s job.
    ///
    /// ONE arm for all of them because the FAULT is one — the missing
    /// variable — and so is the recourse. What differs is which edit
    /// the user submitted, and that rides along in `door` so the
    /// sentence can say it.
    UnknownVar {
        /// The address the edit carried.
        var: VarRef,
        /// Which edit was refused.
        door: CarryForwardDoor,
    },
    /// A [`DocEdit::DeclareVar`] or a [`DocEdit::RenameVar`] offered a
    /// name the document already holds (VR2: a name is unique).
    VarNameTaken {
        /// The name.
        name: VarName,
        /// The variable holding it.
        holder: SpokenVar,
    },
    /// A [`DocEdit::RenameVar`] offered the name the variable already
    /// has, or cleared a name it does not have ([`Self::LabelUnchanged`]'s
    /// twin).
    VarNameUnchanged {
        /// The variable.
        var: SpokenVar,
    },
    /// A [`DocEdit::RenameVar`] cleared the name of a variable nothing
    /// reads: an anonymous variable is read by something (VR7), and
    /// this one would be neither named nor read.
    AnonymousVarUnread {
        /// The variable.
        var: SpokenVar,
    },
    /// An edit would leave a variable with no name and more than one
    /// reader: it gave an unnamed variable a second reader, or cleared
    /// the name of one two readers share (VR2, VR7). An unnamed variable
    /// has exactly one reader, a slot or a definition; sharing one is
    /// naming it.
    SharedVarNeedsName {
        /// The variable to name.
        var: SpokenVar,
    },
    /// A [`DocEdit::DeleteVar`] named an anonymous variable, whose
    /// lifecycle is its reader's: it goes with it (VR7).
    DeleteAnonymousVar {
        /// The variable.
        var: SpokenVar,
    },
    /// A formula the edit writes reads an entry of the edit's fresh
    /// table that the table does not hold.
    FreshUnheld {
        /// The index read.
        index: u16,
        /// The dimension the formula reads it at.
        referenced: Dimension,
    },
    /// A formula the edit writes reads an entry of the edit's fresh
    /// table at another dimension than the entry's kind.
    FreshKind {
        /// The index read.
        index: u16,
        /// The dimension the entry's kind reads at.
        held: Dimension,
        /// The dimension the formula reads it at.
        referenced: Dimension,
    },
    /// An entry of the edit's fresh table that nothing the edit writes
    /// reads: the variable it would mint has no reader (VR7).
    FreshUnread {
        /// The entry's index.
        index: u16,
    },
    /// A [`DocEdit::DefineVar`] offered a definition of another kind
    /// than the variable's. A kind is fixed at minting (VR3): a new
    /// kind is a new variable.
    VarKindFixed {
        /// The variable.
        var: SpokenVar,
        /// Its kind.
        kind: VarKind,
        /// The kind the definition holds.
        offered: VarKind,
    },
    /// A carry-forward door ([`DocEdit::SetVarValue`],
    /// [`DocEdit::SetVarUnit`], [`DocEdit::SetVarDistribution`]) named
    /// a defined variable. A defined variable holds no value, notation
    /// or distribution of its own: they are its inputs' (VR3).
    NotAFreeVar {
        /// The variable.
        var: SpokenVar,
        /// Which edit was refused.
        door: CarryForwardDoor,
    },
    /// A door that writes or deletes a variable's definition named an
    /// operation's output ([`VarDef::Output`]). Its operation defines
    /// it, and it lives exactly as long as that node; only a rename
    /// reaches it.
    VarIsAnOutput {
        /// The variable.
        var: SpokenVar,
        /// The operation defining it.
        node: SpokenNode,
        /// Which edit was refused.
        door: CarryForwardDoor,
    },
    /// A definition ([`DocEdit::DeclareVar`], [`DocEdit::DefineVar`])
    /// reads, directly or through other definitions, the variable it
    /// defines (VR3).
    DefinitionCycle {
        /// The variable defined.
        var: SpokenVar,
        /// The cycle, from `var` on: each variable's definition reads
        /// the next, and the last one's reads `var`.
        through: Vec<SpokenVar>,
    },
    /// A definition whose expansion through the definitions it reads
    /// would exceed [`DEFINITION_NODE_BOUND`] expression nodes: a
    /// coincidence token writes that expansion, so a diamond of
    /// definitions would grow it exponentially.
    DefinitionTooLarge {
        /// The variable whose expansion is too large.
        var: SpokenVar,
        /// Its expansion's node count, saturating at one past the
        /// bound.
        nodes: usize,
    },
    /// A definition reads a name the document does not hold at the
    /// kind it is read at.
    DefinitionUnknownVarName {
        /// The variable defined.
        var: SpokenVar,
        /// The name read.
        name: VarName,
    },
    /// A definition reads a variable this document does not hold.
    DefinitionUnresolvedVar {
        /// The variable defined.
        var: SpokenVar,
        /// The variable read.
        read: SpokenVar,
    },
    /// A definition reads a variable at a kind other than its own.
    DefinitionVarKind {
        /// The variable defined.
        var: SpokenVar,
        /// The variable read.
        read: SpokenVar,
        /// The read variable's dimension.
        declared: VarKind,
        /// The dimension the definition reads it at.
        referenced: Dimension,
    },
    /// A notation edit ([`DocEdit::SetVarUnit`]) named a `Count`
    /// variable. A count is an exact integer, not a quantity: it names
    /// no notation and carries no field to write one into.
    ///
    /// Distinct from [`Self::VarValueKindMismatch`], which is a
    /// value offered at the wrong kind. There is no notation for a
    /// count under ANY definition.
    VarCountHasNoUnit {
        /// The count variable.
        var: SpokenVar,
    },
    /// An annotation edit ([`DocEdit::SetVarDistribution`]) named a
    /// `Count` variable, which takes no distribution and carries no
    /// field to write one into — the argument is
    /// [`FreeVar::with_distribution`]'s rustdoc (E11.3).
    ///
    /// Raised for a CLEARING edit too: a caller aiming an annotation
    /// edit at a count has the wrong variable, and answering `Ok`
    /// because the field happened to be absent would hide that.
    VarCountHasNoDistribution {
        /// The count variable.
        var: SpokenVar,
    },
    /// A unit that does not MEASURE the variable's kind — millimetres
    /// for an angle, degrees for a length — offered by a notation edit
    /// ([`DocEdit::SetVarUnit`]) or carried by a definition
    /// ([`DocEdit::DeclareVar`], [`DocEdit::DefineVar`], whose payload
    /// is `pub` and can pair any unit with any dimension).
    ///
    /// The same pairing the shared save/load validator refuses a
    /// document for (`PersistError::DisplayUnit`) and the authoring
    /// doors ([`FreeVar::written_length`], [`FreeVar::written_angle`])
    /// make unreachable by construction. Its sentence is
    /// `PersistError::DisplayUnit`'s shape with one word of difference:
    /// "the display unit offered", because here it is an argument that
    /// never reached a document.
    VarUnitMismatch {
        /// The variable.
        var: SpokenVar,
        /// The dimension the offered unit measures.
        unit: Dimension,
        /// The dimension the variable holds.
        declared: Dimension,
    },
    /// A value edit offered a value of the wrong kind — a count for a
    /// continuous variable or a continuous value for a count. A kind is
    /// fixed at minting (VR3).
    VarValueKindMismatch {
        /// The variable.
        var: SpokenVar,
        /// The dimension the variable holds (`Count` for a count).
        declared: Dimension,
        /// The value the edit offered.
        offered: FreeValue,
    },
    /// A `SetExpression` path runs off the expression tree (spec D5).
    PathOffTree {
        /// The node whose slot the path addresses.
        node: SpokenNode,
        /// The slot.
        slot: SlotId,
        /// The AST-child indices that run off the tree
        /// ([`ExprPath::path`]); with `node` and `slot`, the whole
        /// address.
        path: Vec<u8>,
    },
    /// Replacing the subtree broke an ancestor's dimension check.
    Dimension(DimensionError),
    /// A name-referencing payload — a boolean's declared pairs or a
    /// `Fillet`'s selection (M6-5) — names a node that does not exist
    /// at edit time (spec D3 carve-out, ruled): a never-existed id is
    /// a TYPO, refused at the best-diagnostics door. (A later
    /// `DeleteNode` stranding a name is ALLOWED — N5 dangling
    /// semantics; see [`crate::DeclaredPair`].)
    DeclareNamesMissingNode {
        /// The name whose node is not live.
        name: SpokenName,
    },
    /// A name written into the document spells a profile step the
    /// document never minted — one its mint log does not hold. The
    /// node half's rule ([`EditError::DeclareNamesMissingNode`]) for
    /// the half of a name that is a step id: a never-minted id is a
    /// typo, or a name carried from another branch of the document. (A
    /// step a `SetProgram` dropped was minted, so a name on it is
    /// ALLOWED — it strands, DM7.)
    NameStepNeverMinted {
        /// The name.
        name: SpokenName,
        /// The step it spells.
        step: StepId,
    },
    /// A reference's READ SITE — the operand a mate is authored
    /// against ([`Node::payload_read_sites`]) — names a node that does
    /// not exist at edit time. The name half's rule, applied to the
    /// half that is a node id rather than a name: a never-existed id
    /// is a TYPO. (A later `DeleteNode` stranding an operand is
    /// ALLOWED — N5 dangling semantics — and the solve refuses typed.)
    ReadSiteMissingNode {
        /// The operand that is not live.
        at: SpokenNode,
    },
    /// A non-finite (NaN/inf) float on a continuous doc param — its
    /// value or one of its distribution's offsets — refused at the
    /// edit door (ruled door 1 of the non-finite policy; F3's
    /// persist-time refusal then has nothing to catch).
    NonFiniteVar {
        /// The variable.
        var: SpokenVar,
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
        /// The variable.
        var: SpokenVar,
        /// The invariant that failed.
        fault: DistributionFault,
    },
    /// A `Rebind` whose target name's node is not live (the selection
    /// must denote something the recipe still has — best-diagnostics
    /// door, mirrors the Declare carve-out).
    RebindTargetMissingNode {
        /// The target name whose node is gone.
        name: SpokenName,
    },
    /// A `Rebind` whose SOURCE name's node was never minted by this
    /// document (ids are never reused, so an id the mint log does not
    /// hold is a typo or a foreign name — refused;
    /// a deleted-but-once-lived node is ALLOWED, that is the
    /// `NodeGone` repair case).
    RebindUnknownName {
        /// The foreign source name.
        name: SpokenName,
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
        name: SpokenName,
    },
    /// A `Rebind` whose source name no document site references:
    /// there is nothing to repair (GUI selection state is not
    /// document state — repairing a selection is re-selecting).
    RebindNoReferences {
        /// The unreferenced source name.
        name: SpokenName,
    },
    /// A `ReWitness` aimed at a node that is not sketch-bearing (the
    /// witness datum is GQ1's per-sketch-node branch selection;
    /// Profile is the v1 sketch-bearing node kind).
    WitnessOnNonSketch {
        /// The non-sketch node.
        node: SpokenNode,
    },
    /// A `ReWitnessBulk` with the same node listed twice (which entry
    /// wins would be positional — refused).
    DuplicateWitnessEntry {
        /// The duplicated node.
        node: SpokenNode,
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
        name: SpokenName,
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
        name: SpokenName,
        /// The colliding attribute kind.
        kind: AttrKind,
    },
    /// A `SetAppearance` on an edge or vertex name — v1 appearance is
    /// per-face/per-body (M4-PLAN item 7); edge/vertex attributes are
    /// a future additive extension, refused typed until ratified.
    AppearanceWrongKind {
        /// The refused name.
        name: SpokenName,
    },
    /// A `SetAppearance` naming a node that is not live at edit time
    /// (the Declare-parallel carve-out: a never-existed id is a typo;
    /// see [`DocEdit::SetAppearance`]).
    AppearanceNamesMissingNode {
        /// The name whose node is not live.
        name: SpokenName,
    },
    /// A `ClearAppearance` for an attribute that is not set — loud
    /// no-ops per the fail-loud charter.
    AppearanceNotSet {
        /// The name.
        name: SpokenName,
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
        name: SpokenName,
        /// The metadata key.
        key: String,
        /// The typed shape refusal.
        error: MetaVersionError,
    },
    /// A `SetAppearanceMeta` value carrying a non-finite float (D2:
    /// refused at the edit door, never stored).
    MetaNonFinite {
        /// The name.
        name: SpokenName,
        /// The metadata key.
        key: String,
        /// Path of the offending float within the value tree.
        path: String,
    },
    /// A `ClearAppearanceMeta` for a key that is not set — loud
    /// no-ops per the fail-loud charter.
    MetaNotSet {
        /// The name.
        name: SpokenName,
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
        name: SpokenName,
        /// The colliding metadata key.
        key: String,
    },
    /// An offset aimed at a node that does not instantiate a part
    /// (A11 (2): an offset places an instance in its gauge, and
    /// nothing else has one).
    OffsetOnNonInstance {
        /// The offending target.
        node: SpokenNode,
    },
    /// A gauge aimed at a node that neither instantiates a part nor is
    /// a gauge (A11 (2): those are the two things that sit on one).
    GaugeOnNonPlaced {
        /// The offending target.
        node: SpokenNode,
    },
    /// A gauge reference that names no live node: a deleted gauge, or
    /// an id this document never minted. A deleted gauge's references
    /// are kept, dangling, but no edit writes a new one.
    GaugeNotLive {
        /// The instance or gauge the reference is written on.
        node: SpokenNode,
        /// The id it names.
        gauge: SpokenNode,
    },
    /// A gauge reference that names a live node that is not a gauge.
    NotAGauge {
        /// The instance or gauge the reference is written on.
        node: SpokenNode,
        /// The node it names.
        gauge: SpokenNode,
    },
    /// A gauge reference that would make a gauge sit on itself
    /// through its own chain.
    GaugeCycle {
        /// The gauge the reference is written on.
        node: SpokenNode,
        /// The gauge it names, which sits on `node`.
        gauge: SpokenNode,
    },
    /// **"Copy a gauge, then mate" would make another mate start
    /// placing** (A11 (2); [`regauge_then_mate`]): re-gauging the first
    /// operand's group would put both instances of `mate`, which
    /// declares today, on one gauge, so it would start placing and move
    /// a group the action never named.
    WouldStartPlacing {
        /// The mate that would start placing.
        mate: SpokenNode,
    },
    /// A promote aimed at a node that does not instantiate a part.
    PromoteOnNonInstance {
        /// The offending target.
        node: SpokenNode,
    },
    /// A promote of an instance whose group carries no offset at all:
    /// there is no chain to become a gauge.
    PromoteWithoutOffset {
        /// The instance.
        node: SpokenNode,
    },
    /// A promote of an instance that is not the earliest member of its
    /// group carrying an offset: that member's offset is the one that
    /// states where the group sits (A11 (2)), and any offset `node`
    /// carries is a statement the solve checks.
    PromoteNonRoot {
        /// The instance.
        node: SpokenNode,
        /// The earliest member of its group carrying an offset.
        root: SpokenNode,
    },
    /// A promote whose group holds another member carrying an offset:
    /// that offset is stated in the instance's gauge, and would not
    /// hold on the promoted one.
    PromoteMemberOffset {
        /// The instance promoted.
        node: SpokenNode,
        /// The member carrying an offset.
        member: SpokenNode,
    },
    /// A fold aimed at a node that is not a gauge.
    FoldOnNonGauge {
        /// The offending target.
        node: SpokenNode,
    },
    /// A fold that would put both instances of `mate`, which declares
    /// today, on one gauge, so it would start placing and move a group
    /// the fold never named.
    FoldWouldStartPlacing {
        /// The gauge folded.
        node: SpokenNode,
        /// The mate that would start placing.
        mate: SpokenNode,
    },
    /// A placement-rule node whose rule and count slot would give two
    /// answers to "how many placements" (GROUP-BOOLEAN-DESIGN).
    PlacementRuleMismatch {
        /// The offending node.
        node: SpokenNode,
        /// Which answer it gives twice.
        shape: CountMismatch,
    },
    /// A placement-rule node whose `Explicit` rule lists NO placements
    /// (GROUP-BOOLEAN-DESIGN): the list IS the count, so an empty one
    /// is the explicit rule's `count < 1` — refused for the reason a
    /// stepped rule's zero is, rather than quietly denoting an empty
    /// body.
    EmptyPlacementList {
        /// The offending node.
        node: SpokenNode,
    },
    /// An IMPROPER placement frame — determinant ≤ 0, i.e. a mirror
    /// (A6). Admitting one is gated on the equivariance audit R4 owns:
    /// until that lands, a mirrored placement is refused rather than
    /// silently trusted to leave every orientation-sensitive predicate
    /// and every outward normal intact.
    ImproperPlacement {
        /// The node holding the frame.
        node: SpokenNode,
        /// Which of its frames.
        at: FrameSite,
        /// The linear part's determinant.
        determinant: f64,
    },
    /// A placement frame carrying a non-finite coordinate.
    NonFinitePlacement {
        /// The node holding the frame.
        node: SpokenNode,
        /// Which of its frames.
        at: FrameSite,
    },
    /// A proper placement frame that is not definitely a rigid motion
    /// at tolerance — it may scale or shear an axis. The evaluation
    /// refuses to move a body by one ([`topo::TransformError::NotRigid`],
    /// the same predicate), so the document never admits one.
    NonRigidPlacement {
        /// The node holding the frame.
        node: SpokenNode,
        /// Which of its frames.
        at: FrameSite,
        /// The rigidity check that refused; routing, never rendered by
        /// name.
        check: &'static str,
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
    /// A mate's alignment datum carries a non-finite number outside
    /// its frames — the clocking rider, or a length the primitive
    /// authors: a datum nothing can decide about never enters the
    /// document. A frame offset's literal steps are the placement
    /// frame rule's (`NonFinitePlacement` at a `FrameSite::MateStep`).
    NonFiniteAlignment {
        /// The mate being inserted.
        node: SpokenNode,
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
    /// needs the group and stays the solve's; and a STATE a mate
    /// comes to hold after insert (a head a rebind or a shrunk pattern
    /// strands, a `Part` re-pointed, a doctored or older snapshot),
    /// which the doors do not re-decide and the solve refuses at
    /// evaluation.
    MateRefused {
        /// The mate being inserted.
        node: SpokenNode,
        /// The solve's own fault, unaltered.
        fault: Box<crate::mate::MateFault>,
        /// The nodes the fault names, as the document the mate stands
        /// in held them at the refusal ([`crate::spoken::held_by`]).
        held: crate::spoken::HeldNodes,
    },
    /// A pin update aimed at a node that does not instantiate a part
    /// (A13; ASM-UPD D-1: only a cross-document reference HAS a
    /// version).
    UpdateOnNonInstance {
        /// The offending target.
        node: SpokenNode,
    },
    /// A pin update whose new pin is the one the reference already
    /// names. The edit would record a step that changes nothing —
    /// refused rather than written, so a log's presence of an update
    /// always means a version actually moved (ASM-UPD D-1's fail-loud
    /// rule).
    PinUnchanged {
        /// The reference that already names this pin.
        node: SpokenNode,
        /// The pin both sides carry.
        pin: crate::ident::ContentPin,
    },
    /// A `SetLabel` that would leave the node's label as it is: the
    /// same text again, or a clear of a node with none. Refused rather
    /// than recorded, so a log's label edit always moved a label.
    LabelUnchanged {
        /// The node, with the label it already has
        /// ([`SpokenNode::label`], `None` for none).
        node: SpokenNode,
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
// applies — each arm states the PROBLEM (and where it is) and then the
// RECOURSE, not the enum's guts; stable names, kinds and dimensions render through their
// own prose spellings (`StableName`'s `Display`, the `noun`
// renderings, `Dimension`'s `Display`), never `Debug`. A name is
// parenthesized apposition when the sentence's subject is a role word
// ("the rebind target ({name})") and inline when the name itself is
// the subject ("{name} does not resolve"); new arms copy whichever
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
// name renders through `VarName`'s `Display` at every door that
// frames it in a sentence of its own; the one door that quotes is
// `ParseError::UnknownParam`, which echoes the bytes an author typed
// and says so at the site. The SLOT id renders through `SlotId::label`
// for the same reason a name does: a variant identifier dropped into a
// sentence is the `Debug` dump's fingerprint, and the slot vocabulary
// has one prose spelling of its own, so a message reads "slot origin x"
// rather than "slot Origin(X)".
/// The AXIS's refusal, in the authoring vocabulary — what makes
/// `Frame::rotate_then_translate(..)?` compose with
/// `apply(.., DocEdit::SetOffset { .. })?` in one function.
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

// The recourse phrases more than one arm states, each written once.
// `HELD_NODE`'s literal, for the `concat!` that composes it.
macro_rules! held_node {
    () => {
        "a node the document holds"
    };
}
/// The node a refusal's recourse points at instead of the one it names.
const HELD_NODE: &str = held_node!();
/// The recourse of a name whose node is not live.
const NAME_A_HELD_ENTITY: &str = concat!("name an entity of ", held_node!());
/// The recourse of a name no variable holds: declaring it gives the
/// name a variable, and the leaf lowers to it.
const DECLARE_THE_NAME: &str = "declare it first";
/// What an unknown name can read instead.
const OR_A_DECLARED_PARAM: &str = ", or read a declared variable";
/// The recourse of a reader of a variable the document does not hold.
const READ_A_HELD_VAR: &str = "read a variable the document holds";
/// The recourse of a rebind that would land two values on one name.
const CLEAR_ONE_FIRST: &str = "clear one of the two first";

/// Whether a rendering of an [`EditError`] ends with the recourse
/// ([`EditError`]'s `Display`) or stops at the problem
/// ([`EditError::problem`]).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tail {
    Recourse,
    ProblemOnly,
}

impl Tail {
    /// Ends the sentence with `recourse`, labelled, or with nothing.
    fn recourse(
        self,
        f: &mut core::fmt::Formatter<'_>,
        recourse: core::fmt::Arguments<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Recourse => write!(f, ". {}", crate::sentence::Recourse(recourse)),
            Self::ProblemOnly => Ok(()),
        }
    }

    /// Ends the sentence with a dead end's whole ending, or with
    /// nothing.
    fn ending(self, f: &mut core::fmt::Formatter<'_>, ending: &str) -> core::fmt::Result {
        match self {
            Self::Recourse => write!(f, ". {ending}"),
            Self::ProblemOnly => Ok(()),
        }
    }
}

/// **A placement frame's refusal**: the frame's subject at its site,
/// the frame rule's own clause, and its recourse
/// ([`FrameFault::recourse`]), which every door that raises one can
/// honour — each refuses the edit that carried the frame.
fn frame_refusal(
    f: &mut core::fmt::Formatter<'_>,
    tail: Tail,
    node: &SpokenNode,
    at: FrameSite,
    fault: FrameFault,
) -> core::fmt::Result {
    write!(f, "{} {fault}", at.subject(node))?;
    tail.recourse(f, format_args!("{}", fault.recourse()))
}

impl EditError {
    /// The arm a frame the admission rule refused is reported under,
    /// at `at` on `node`.
    fn placement_frame(node: SpokenNode, at: FrameSite, fault: FrameFault) -> Self {
        match fault {
            FrameFault::NonFinite => Self::NonFinitePlacement { node, at },
            FrameFault::Improper { determinant } => Self::ImproperPlacement {
                node,
                at,
                determinant,
            },
            FrameFault::NotRigid { check } => Self::NonRigidPlacement { node, at, check },
        }
    }
}

/// [`EditError::StepIdsRefused`]'s recourse, by the fault. `InsertNode`
/// raises `Preminted`, `SetProgram` the shape and keep arms;
/// `NotMinted` is the load door's, which an edit door
/// spells [`EditError::NameStepNeverMinted`].
fn step_ids_recourse(
    f: &mut core::fmt::Formatter<'_>,
    tail: Tail,
    fault: &crate::program::StepIdFault,
) -> core::fmt::Result {
    use crate::program::StepIdFault as F;
    match fault {
        F::Preminted => tail.recourse(f, format_args!("clear the program's step ids first")),
        F::LoopCount { .. } => tail.recourse(
            f,
            format_args!("give one list of kept ids per loop of the new program"),
        ),
        F::Shape { loop_, .. } => tail.recourse(
            f,
            format_args!(
                "give loop {loop_} one entry per authored step, with no id for a step it adds"
            ),
        ),
        F::NotThisProfiles { .. } => tail.recourse(
            f,
            format_args!("keep only ids of this node's own steps, and give a new step none"),
        ),
        F::Repeated { .. } => tail.recourse(
            f,
            format_args!("keep the id on one of the two steps, and give the other none"),
        ),
        F::NotMinted { .. } => tail.ending(f, geom_core::KERNEL_DEFECT_ENDING),
    }
}

/// [`EditError::problem`]'s rendering.
struct Problem<'a>(&'a EditError);

impl core::fmt::Display for Problem<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.render(f, Tail::ProblemOnly)
    }
}

impl core::fmt::Display for EditError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.render(f, Tail::Recourse)
    }
}

impl EditError {
    /// The refusal without its recourse: what a door renders when it
    /// forwards an edit the user did not author, and so states a
    /// recourse of its own (`SplitError::PartEdit`, `InlineError::Edit`
    /// and the other doors that replay or derive edits). An arm that
    /// forwards another layer's sentence (`ProfileProgramRefused`,
    /// `MateRefused`) forwards it whole here too:
    /// the forwarded layer owns that text.
    pub fn problem(&self) -> impl core::fmt::Display + '_ {
        Problem(self)
    }

    /// **This refusal with every node it names spoken again from
    /// `doc`, a later version of the document it was raised in**
    /// ([`SpokenNode::respoken`], whose rule and soundness argument
    /// this follows), and each variable it names
    /// ([`crate::spoken::SpokenVar::respoken`]), so a label or a name
    /// changed since the door refused is the one it says.
    ///
    /// **`doc` must be a version of the document whose door raised
    /// this refusal.** The refusal carries no [`crate::DocumentId`] to
    /// check that by, so the caller answers for it — and a refusal a
    /// split or an inline forwards (`SplitError::PartEdit`,
    /// `InlineError::Edit`) was spelled in another document than the
    /// one the caller holds.
    ///
    /// **An arm whose sentence is about the state the door saw stays
    /// as raised**: [`EditError::LabelUnchanged`] (about the label the
    /// node held, which said with a later one would claim a label the
    /// refused edit never offered), [`EditError::VarNameUnchanged`]
    /// (the same, of a variable's name), and the arms
    /// that say a node is not live — said from a version that holds it
    /// again, `X "plate" is not live` would contradict itself.
    #[must_use]
    pub fn respoken<P: crate::ProfilePayload>(&self, doc: &Doc<P>) -> Self {
        let mut again = self.clone();
        match &mut again {
            Self::UnknownSlot { id, slot: _ } => {
                *id = id.respoken(doc);
            }
            Self::ProfileProgramRefused { node, refusal: _ }
            | Self::SelectionShape {
                node,
                slot: _,
                fault: _,
            }
            | Self::SetMembersOnNonList { node }
            | Self::SetDeclareOnNonDeclaring { node }
            | Self::SetProgramOnNonProfile { node }
            | Self::SetExtrudeSideOnNonExtrude { node }
            | Self::StepIdsRefused { node, fault: _ }
            | Self::TooFewMembers { node, found: _ }
            | Self::SlotUnknownVarName {
                node,
                name: _,
                slot: _,
            }
            | Self::PayloadUnknownVarName { node, name: _ }
            | Self::PathOffTree {
                node,
                slot: _,
                path: _,
            }
            | Self::WitnessOnNonSketch { node }
            | Self::DuplicateWitnessEntry { node }
            | Self::OffsetOnNonInstance { node }
            | Self::GaugeOnNonPlaced { node }
            | Self::PromoteOnNonInstance { node }
            | Self::PromoteWithoutOffset { node }
            | Self::FoldOnNonGauge { node }
            | Self::PlacementRuleMismatch { node, shape: _ }
            | Self::EmptyPlacementList { node }
            | Self::ImproperPlacement {
                node,
                at: _,
                determinant: _,
            }
            | Self::NonFinitePlacement { node, at: _ }
            | Self::NonRigidPlacement {
                node,
                at: _,
                check: _,
            }
            | Self::NonFiniteAlignment { node }
            | Self::UpdateOnNonInstance { node }
            | Self::PinUnchanged { node, pin: _ } => {
                *node = node.respoken(doc);
            }
            Self::SlotVarKind {
                node,
                var,
                slot: _,
                found: _,
                expected: _,
            } => {
                *node = node.respoken(doc);
                **var = var.respoken(doc);
            }
            Self::VarIsAnOutput { var, node, door: _ } => {
                *node = node.respoken(doc);
                *var = var.respoken(doc);
            }
            Self::SlotUnresolvedVar { node, var, slot: _ }
            | Self::PayloadVarKind {
                node,
                var,
                declared: _,
                referenced: _,
            }
            | Self::PayloadUnresolvedVar { node, var } => {
                *node = node.respoken(doc);
                *var = var.respoken(doc);
            }
            Self::ContinuousVarCannotBeCount { var }
            | Self::VarNameTaken {
                name: _,
                holder: var,
            }
            | Self::AnonymousVarUnread { var }
            | Self::SharedVarNeedsName { var }
            | Self::DeleteAnonymousVar { var }
            | Self::VarKindFixed {
                var,
                kind: _,
                offered: _,
            }
            | Self::VarCountHasNoUnit { var }
            | Self::VarCountHasNoDistribution { var }
            | Self::VarUnitMismatch {
                var,
                unit: _,
                declared: _,
            }
            | Self::VarValueKindMismatch {
                var,
                declared: _,
                offered: _,
            }
            | Self::NonFiniteVar { var, field: _ }
            | Self::InvalidDistribution { var, fault: _ }
            | Self::NotAFreeVar { var, door: _ }
            | Self::DefinitionTooLarge { var, nodes: _ }
            | Self::DefinitionUnknownVarName { var, name: _ } => {
                *var = var.respoken(doc);
            }
            Self::DefinitionUnresolvedVar { var, read }
            | Self::DefinitionVarKind {
                var,
                read,
                declared: _,
                referenced: _,
            } => {
                *var = var.respoken(doc);
                *read = read.respoken(doc);
            }
            Self::DefinitionCycle { var, through } => {
                *var = var.respoken(doc);
                for held in through.iter_mut() {
                    *held = held.respoken(doc);
                }
            }
            Self::WouldCycle { at } => {
                *at = at.respoken(doc);
            }
            Self::DuplicateInput { node, input }
            | Self::PromoteNonRoot { node, root: input }
            | Self::PromoteMemberOffset {
                node,
                member: input,
            }
            | Self::FoldWouldStartPlacing { node, mate: input } => {
                *node = node.respoken(doc);
                *input = input.respoken(doc);
            }
            Self::OperandUnresolved {
                node,
                slot: _,
                read: _,
            } => *node = node.respoken(doc),
            Self::PartHalfPort { node, half: _, var } => {
                *node = node.respoken(doc);
                **var = var.respoken(doc);
            }
            Self::ReadsWorldCopy {
                node,
                slot: _,
                placement,
            } => {
                *node = node.respoken(doc);
                *placement = placement.respoken(doc);
            }
            Self::AmbiguousOutput {
                input,
                slot: _,
                ports: _,
            }
            | Self::DefinesNothing { input, slot: _ } => {
                *input = input.respoken(doc);
            }
            Self::AssertionDimension {
                node,
                measured: _,
                bound: _,
            } => {
                *node = node.respoken(doc);
            }
            Self::ConstructionReadsObserved { node, slot: _, var } => {
                *node = node.respoken(doc);
                **var = var.respoken(doc);
            }
            Self::NameStepNeverMinted { name, step: _ }
            | Self::RebindUnknownName { name }
            | Self::RebindIdentity { name }
            | Self::RebindNoReferences { name }
            | Self::NameUnresolvedInEvaluation { name }
            | Self::RebindAppearanceCollision { name, kind: _ }
            | Self::AppearanceWrongKind { name }
            | Self::AppearanceNotSet { name, kind: _ }
            | Self::MetaUnversioned {
                name,
                key: _,
                error: _,
            }
            | Self::MetaNonFinite {
                name,
                key: _,
                path: _,
            }
            | Self::MetaNotSet { name, key: _ }
            | Self::RebindMetadataCollision { name, key: _ } => {
                *name = name.respoken(doc);
            }
            Self::GaugeNotLive { node, gauge: _ } => {
                *node = node.respoken(doc);
            }
            Self::DeclaredSiteNotAnOperand { node, name, site } => {
                *node = node.respoken(doc);
                *name = name.respoken(doc);
                *site = site.respoken(doc);
            }
            Self::DeclaredNameNotUpstream { node, name } => {
                *node = node.respoken(doc);
                *name = name.respoken(doc);
            }
            Self::NotAGauge { node, gauge } | Self::GaugeCycle { node, gauge } => {
                *node = node.respoken(doc);
                *gauge = gauge.respoken(doc);
            }
            Self::WouldStartPlacing { mate } => {
                *mate = mate.respoken(doc);
            }
            Self::MateRefused {
                node,
                held,
                fault: _,
            } => {
                *node = node.respoken(doc);
                *held = held.respoken(doc);
            }
            Self::LabelUnchanged { node: _ }
            | Self::VarNameUnchanged { var: _ }
            | Self::UnknownNode { id: _ }
            | Self::UnresolvedInput { input: _ }
            | Self::ReadSiteMissingNode { at: _ }
            | Self::DeclareNamesMissingNode { name: _ }
            | Self::RebindTargetMissingNode { name: _ }
            | Self::AppearanceNamesMissingNode { name: _ }
            | Self::SlotDimensionMismatch {
                slot: _,
                expected: _,
                found: _,
            }
            | Self::StructuralSlotNeedsStructuralEdit { slot: _ }
            | Self::NotStructuralSlot { slot: _ }
            | Self::UnknownVar { var: _, door: _ }
            | Self::FreshUnheld {
                index: _,
                referenced: _,
            }
            | Self::FreshKind {
                index: _,
                held: _,
                referenced: _,
            }
            | Self::FreshUnread { index: _ }
            | Self::Dimension(_)
            | Self::RebindKindMismatch { from: _, to: _ }
            | Self::EmptyWitnessBulk
            | Self::EvaluationOfAnotherDocument {
                expected: _,
                found: _,
            }
            | Self::InvalidTolerance { value: _ }
            | Self::PlacementAxis { error: _ } => {}
        }
        again
    }

    #[allow(clippy::too_many_lines)] // one arm per variant, each short
    fn render(&self, f: &mut core::fmt::Formatter<'_>, tail: Tail) -> core::fmt::Result {
        match self {
            Self::UnknownNode { id } => {
                write!(f, "{id} is not live")?;
                tail.recourse(f, format_args!("aim the edit at {HELD_NODE}"))
            }
            Self::ProfileProgramRefused { node, refusal } => {
                write!(f, "{node}'s sketch refused: {refusal}")
            }
            Self::UnresolvedInput { input } => {
                write!(f, "the input {input} is not a live node")?;
                tail.recourse(f, format_args!("take the input from {HELD_NODE}"))
            }
            // Only `SetMembers` can close a loop: an insert's inputs are
            // already live, so none of them can be built from it.
            Self::WouldCycle { at } => {
                write!(f, "the recipe graph would cycle (through {at})")?;
                tail.recourse(
                    f,
                    format_args!(
                        "take the members from nodes that are not built from the node being \
                         edited"
                    ),
                )
            }
            // Forwarded, not restated: `InputFault` owns this
            // vocabulary (`node::duplicate_input`, which takes the
            // input as this door speaks it). The door adds its own
            // frame — which edit the fault is about — and joins it with
            // a colon, because the forwarded sentence carries an em-dash
            // of its own and two in a row read as a dump.
            //
            // **The frame does not name `node`.** From `InsertNode` it
            // is spoken by its kind and the tag the mint drew
            // ([`SpokenNode::entering`]), and that tag names nothing
            // once the edit is refused, so naming it would send a
            // reader looking for a node that does not exist. The node
            // the reader CAN act on is `input`, which the sentence
            // names, and it is live on both paths (`InsertNode` and
            // `SetMembers`).
            //
            // The action is the door's to add: `InputFault` states the
            // rule ("pairwise distinct"), which says what is wrong and
            // not what to do about it.
            Self::DuplicateInput { input, .. } => {
                f.write_str("the node this edit writes would be invalid: ")?;
                crate::node::duplicate_input(f, input)?;
                tail.recourse(
                    f,
                    format_args!("replace one of the two with a different node"),
                )
            }
            Self::SetMembersOnNonList { node } => {
                write!(
                    f,
                    "{node} carries no list input, so it has no members to set"
                )?;
                tail.recourse(
                    f,
                    format_args!(
                        "set the members of a node that takes a list, or insert a new node \
                         over the inputs you want"
                    ),
                )
            }
            Self::SetDeclareOnNonDeclaring { node } => {
                write!(
                    f,
                    "{node} is not a boolean or a union, so it has no contacts to declare"
                )?;
                tail.recourse(
                    f,
                    format_args!("declare on the boolean or union that joins the pair"),
                )
            }
            Self::DeclaredSiteNotAnOperand { node, name, site } => {
                write!(
                    f,
                    "the declaration names {name}, read at {site}, which is not an operand of {node}"
                )?;
                tail.recourse(
                    f,
                    format_args!("read it at the operand of {node} whose entity it is"),
                )
            }
            Self::DeclaredNameNotUpstream { node, name } => {
                write!(
                    f,
                    "the declaration names {name}, which is not minted upstream of {node}, so \
                     none of its operands can hold it"
                )?;
                tail.recourse(
                    f,
                    format_args!("declare an entity of one of {node}'s operands"),
                )
            }
            // A document's profile node always holds a program; the
            // node named is of another kind.
            Self::SetProgramOnNonProfile { node } => {
                write!(
                    f,
                    "{node} holds no profile program, so it has no program to set"
                )?;
                tail.recourse(f, format_args!("aim the edit at a profile node"))
            }
            Self::SetExtrudeSideOnNonExtrude { node } => {
                write!(f, "{node} is not an extrude, so it has no side to set")?;
                tail.recourse(f, format_args!("aim the edit at an extrude node"))
            }
            // The fault owns its sentence, shared with the load door;
            // the door adds which node's program the ids were about,
            // and the recourse, since only it knows which edit wrote
            // them.
            Self::StepIdsRefused { node, fault } => {
                write!(
                    f,
                    "{node}'s program cannot take the step ids given: {fault}"
                )?;
                step_ids_recourse(f, tail, fault)
            }
            Self::TooFewMembers { found, .. } => {
                write!(
                    f,
                    "the node this edit writes would be invalid: {}",
                    crate::node::InputFault::TooFew { found: *found }
                )?;
                tail.recourse(f, format_args!("list two or more entries"))
            }
            Self::SelectionShape { slot, fault, .. } => {
                write!(
                    f,
                    "the selection at {slot} is not one a document stores: {fault}"
                )?;
                tail.recourse(
                    f,
                    format_args!(
                        "author the names in the stored order (an edge set sorted, a face set \
                         naming each face once), as the fillet, chamfer and shell builders do, \
                         or name entities of the kind the seat reads"
                    ),
                )
            }
            Self::OperandUnresolved { node, slot, read } => {
                write!(
                    f,
                    "{node}'s {slot} reads {read}, which is not a variable of this document"
                )?;
                tail.recourse(
                    f,
                    format_args!("read an output of {HELD_NODE}, by its node or its port"),
                )
            }
            Self::AmbiguousOutput { input, slot, ports } => {
                write!(
                    f,
                    "{input} defines {} outputs ({}), so naming it alone does not say which its \
                     {slot} reads",
                    ports.len(),
                    ports.join(", ")
                )?;
                tail.recourse(f, format_args!("name the port the {slot} reads"))
            }
            Self::DefinesNothing { input, slot } => {
                write!(f, "{input} defines nothing a {slot} could read")?;
                tail.recourse(f, format_args!("read an operation that defines a value"))
            }
            Self::PartHalfPort { node, half, var } => {
                write!(f, "{node} selects the {} half but reads {var}", half.name())?;
                tail.recourse(f, format_args!("read the split's {} output", half.name()))
            }
            Self::ReadsWorldCopy {
                node,
                slot,
                placement,
            } => {
                write!(
                    f,
                    "{node}'s {slot} reads the world copy {placement} makes, which no \
                     construction reads: only the product, export and a measure do"
                )?;
                tail.recourse(f, format_args!("read the body {placement} reads"))
            }
            Self::UnknownSlot { id, slot } => {
                write!(f, "{id} has no slot {}", slot.label())?;
                tail.recourse(f, format_args!("edit a slot this node has"))
            }
            // The rule's own clause, forwarded rather than restated:
            // this door's subject IS the slot, so the problem is the
            // clause and nothing more.
            Self::SlotDimensionMismatch {
                slot,
                expected,
                found,
            } => {
                write!(
                    f,
                    "{}",
                    SlotDimensionFault {
                        slot: *slot,
                        expected: *expected,
                        found: *found
                    }
                )?;
                tail.recourse(
                    f,
                    format_args!(
                        "write it from {expected} literals and parameters declared {expected}"
                    ),
                )
            }
            Self::StructuralSlotNeedsStructuralEdit { slot } => {
                write!(
                    f,
                    "slot {} is structural, so a continuous edit cannot set it",
                    slot.label()
                )?;
                tail.recourse(f, format_args!("set it with a structural edit"))
            }
            Self::NotStructuralSlot { slot } => {
                write!(f, "slot {} is continuous, not structural", slot.label())?;
                tail.recourse(f, format_args!("set it with a continuous edit"))
            }
            Self::PayloadUnknownVarName { name, node } => {
                write!(
                    f,
                    "no variable is named {name} (read by {}'s payload expression)",
                    node
                )?;
                tail.recourse(f, format_args!("{DECLARE_THE_NAME}{OR_A_DECLARED_PARAM}"))
            }
            Self::PayloadVarKind {
                var,
                node,
                declared,
                referenced,
            } => {
                write!(
                    f,
                    "{var} is declared {declared} but {}'s payload expression reads it as \
                     {referenced}",
                    node
                )?;
                tail.recourse(f, format_args!("{}", ParamDimensionRecourse(*referenced)))
            }
            Self::PayloadUnresolvedVar { var, node } => {
                write!(
                    f,
                    "{var} is not a variable of this document (read by {}'s payload \
                     expression)",
                    node
                )?;
                tail.recourse(f, format_args!("{READ_A_HELD_VAR}"))
            }
            Self::AssertionDimension {
                node,
                measured,
                bound,
            } => {
                write!(
                    f,
                    "{} bounds {} {measured} value with {} {bound} expression — an assertion \
                     compares like with like or not at all",
                    node,
                    measured.article(),
                    bound.article(),
                )?;
                tail.recourse(
                    f,
                    format_args!("bound it with {} {measured} expression", measured.article()),
                )
            }
            Self::ConstructionReadsObserved { node, slot, var } => {
                write!(
                    f,
                    "{node}'s slot {} reads {var}, a measured value — only an assertion reads a \
                     measured value; a construction reads what was written",
                    slot.label()
                )?;
                tail.recourse(
                    f,
                    format_args!(
                        "write the value the slot reads, and check it against the measure with an \
                         assertion"
                    ),
                )
            }
            Self::SlotUnknownVarName { name, node, slot } => {
                write!(
                    f,
                    "no variable is named {name} (read by {}, slot {})",
                    node,
                    slot.label()
                )?;
                tail.recourse(f, format_args!("{DECLARE_THE_NAME}{OR_A_DECLARED_PARAM}"))
            }
            Self::SlotVarKind {
                var,
                node,
                slot,
                found,
                expected,
            } => match expected {
                crate::SlotKind::Is(kind) if let Some(referenced) = kind.dimension() => {
                    write!(
                        f,
                        "{var} is declared {found} but {} (slot {}) reads it as {referenced}",
                        node,
                        slot.label(),
                    )?;
                    tail.recourse(f, format_args!("{}", ParamDimensionRecourse(referenced)))
                }
                _ => {
                    write!(
                        f,
                        "{node}'s {} reads {var}, which is {} {found}, where it takes {expected}",
                        slot.label(),
                        crate::sentence::article(&found.to_string()),
                    )?;
                    tail.recourse(f, format_args!("read {expected} there"))
                }
            },
            Self::SlotUnresolvedVar { var, node, slot } => {
                write!(
                    f,
                    "{var} is not a variable of this document (read by {}, slot {})",
                    node,
                    slot.label()
                )?;
                tail.recourse(f, format_args!("{READ_A_HELD_VAR}"))
            }
            Self::ContinuousVarCannotBeCount { var } => {
                write!(
                    f,
                    "{var} is continuous, and a continuous variable cannot be a count"
                )?;
                tail.recourse(
                    f,
                    format_args!("define it as a count, or give it a quantity's dimension"),
                )
            }
            // The recourse is `UNKNOWN_VAR_RECOURSE`, which the
            // viewer's `Refusal::NoSuchVariable` renders too; the const's
            // own doc says why the two doors converge there.
            Self::UnknownVar { var, door } => {
                write!(
                    f,
                    "{var} is not a variable of this document, so {door} has nothing to \
                     act on"
                )?;
                tail.recourse(f, format_args!("{UNKNOWN_VAR_RECOURSE}"))
            }
            Self::VarIsAnOutput { var, node, door } => {
                write!(
                    f,
                    "{var} is an output of {node}, which defines it, so {door} cannot reach it"
                )?;
                tail.recourse(f, format_args!("edit or delete {node}"))
            }
            Self::NotAFreeVar { var, door } => {
                write!(
                    f,
                    "{var} is defined by an expression, so {door} has no value, notation or \
                     distribution of its own to write"
                )?;
                tail.recourse(
                    f,
                    format_args!(
                        "edit a free variable its definition reads, or redefine {var} as a \
                         free variable"
                    ),
                )
            }
            Self::DefinitionCycle { var, through } => {
                write!(f, "{}", DefinitionCycleSentence { var, through })?;
                tail.recourse(
                    f,
                    format_args!("define {var} without reading any variable on that cycle"),
                )
            }
            Self::DefinitionTooLarge { var, nodes } => {
                write!(f, "{}", DefinitionTooLargeSentence { var, nodes: *nodes })?;
                tail.recourse(
                    f,
                    format_args!(
                        "define {var} over fewer nested definitions, or make one of them free"
                    ),
                )
            }
            Self::DefinitionUnknownVarName { var, name } => {
                write!(
                    f,
                    "no variable is named {name} (read by the definition of {var})"
                )?;
                tail.recourse(f, format_args!("{DECLARE_THE_NAME}{OR_A_DECLARED_PARAM}"))
            }
            Self::DefinitionUnresolvedVar { var, read } => {
                write!(
                    f,
                    "{read} is not a variable of this document (read by the definition of \
                     {var})"
                )?;
                tail.recourse(f, format_args!("{READ_A_HELD_VAR}"))
            }
            Self::DefinitionVarKind {
                var,
                read,
                declared,
                referenced,
            } => {
                write!(
                    f,
                    "{}",
                    DefinitionVarKindSentence {
                        var,
                        read,
                        declared: *declared,
                        referenced: *referenced,
                    }
                )?;
                tail.recourse(f, format_args!("{}", ParamDimensionRecourse(*referenced)))
            }
            Self::VarNameTaken { name, holder } => {
                write!(
                    f,
                    "the name {name} is held by another variable ({}), and a name is unique \
                     within a document",
                    holder.id()
                )?;
                tail.recourse(
                    f,
                    format_args!("choose a name the document does not hold, or edit {name}"),
                )
            }
            Self::VarNameUnchanged { var } => {
                match var.name() {
                    Some(name) => write!(f, "{} is already named {name}", var.id())?,
                    None => write!(f, "{var} already has no name")?,
                }
                tail.recourse(f, format_args!("offer a different name"))
            }
            Self::AnonymousVarUnread { var } => {
                write!(
                    f,
                    "nothing reads {var}, and a variable with no name is one something reads"
                )?;
                tail.recourse(
                    f,
                    format_args!("read it from a slot before clearing its name, or delete it"),
                )
            }
            Self::SharedVarNeedsName { var } => {
                write!(
                    f,
                    "this edit leaves {var} with no name and more than one reader, and a \
                     variable with no name has exactly one"
                )?;
                tail.recourse(
                    f,
                    format_args!("name it, and its readers share it by that name"),
                )
            }
            Self::DeleteAnonymousVar { var } => {
                write!(
                    f,
                    "{var} has no name, and a variable with no name goes with the one \
                     expression reading it"
                )?;
                tail.recourse(
                    f,
                    format_args!("replace the expressions that read it, or name it first"),
                )
            }
            Self::FreshUnheld { index, referenced } => {
                write!(
                    f,
                    "a formula reads entry {index} of the edit's fresh table as {} {referenced}, \
                     and the table holds no entry {index}",
                    referenced.article()
                )?;
                tail.recourse(
                    f,
                    format_args!("add the entry to the table, or read an entry it holds"),
                )
            }
            Self::FreshKind {
                index,
                held,
                referenced,
            } => {
                write!(
                    f,
                    "a formula reads entry {index} of the edit's fresh table as {} {referenced}, \
                     and the entry is {} {held}",
                    referenced.article(),
                    held.article()
                )?;
                tail.recourse(
                    f,
                    format_args!("read the entry as {} {held}", held.article()),
                )
            }
            Self::FreshUnread { index } => {
                write!(
                    f,
                    "entry {index} of the edit's fresh table is read by nothing the edit \
                     writes, and a variable with no name goes when nothing reads it"
                )?;
                tail.recourse(f, format_args!("read the entry, or drop it from the table"))
            }
            Self::VarKindFixed { var, kind, offered } => {
                write!(
                    f,
                    "{var} is of kind {kind} and the definition offered is of kind {offered}, \
                     and a variable's kind is fixed when it is declared"
                )?;
                tail.recourse(
                    f,
                    format_args!(
                        "offer a definition of kind {kind}, or declare a new variable of kind \
                         {offered}"
                    ),
                )
            }
            Self::VarCountHasNoUnit { var } => {
                write!(
                    f,
                    "{var} is a count, and a count is an integer rather than a \
                     quantity, so it has no display unit to change"
                )?;
                tail.recourse(
                    f,
                    format_args!("{COUNT_HAS_NO_FIELD}, which carries a unit"),
                )
            }
            Self::VarCountHasNoDistribution { var } => {
                write!(
                    f,
                    "{var} is a count, and a count is a structural parameter that \
                     is fixed under any error analysis, so it has no distribution to change"
                )?;
                tail.recourse(
                    f,
                    format_args!("{COUNT_HAS_NO_FIELD}, which carries a distribution"),
                )
            }
            Self::VarUnitMismatch {
                var,
                unit,
                declared,
            } => {
                write!(
                    f,
                    "{var} is declared {declared} but the display unit offered \
                     measures {unit}"
                )?;
                tail.recourse(f, format_args!("offer a unit that measures {declared}"))
            }
            Self::VarValueKindMismatch {
                var,
                declared,
                offered,
            } => {
                write!(
                    f,
                    "{var} is declared {declared} but the value edit offered a \
                     {offered}, and a variable's kind is fixed when it is declared"
                )?;
                tail.recourse(
                    f,
                    format_args!(
                        "offer a value of the declared kind, or declare a new variable of the \
                         other"
                    ),
                )
            }
            Self::PathOffTree { node, slot, path } => {
                let steps: Vec<String> = path.iter().map(u8::to_string).collect();
                write!(
                    f,
                    "the expression path [{}] in {}'s {} slot runs off the tree",
                    steps.join(", "),
                    node,
                    slot.label()
                )?;
                tail.recourse(
                    f,
                    format_args!("address a subexpression the slot's expression holds"),
                )
            }
            Self::Dimension(e) => {
                write!(f, "{e}")?;
                tail.recourse(
                    f,
                    format_args!("splice in an expression whose dimension fits its place"),
                )
            }
            Self::NameStepNeverMinted { name, step } => {
                write!(
                    f,
                    "{name} spells the profile step id {}, which this document never minted \
                     (its mint log does not hold it)",
                    step
                )?;
                tail.recourse(
                    f,
                    format_args!("name a piece of a step this document minted"),
                )
            }
            Self::DeclareNamesMissingNode { name } => {
                write!(
                    f,
                    "the declaration names {name}, which refers to a node that is not live"
                )?;
                tail.recourse(f, format_args!("{NAME_A_HELD_ENTITY}"))
            }
            Self::ReadSiteMissingNode { at } => {
                write!(f, "the reference is read at {at}, which is not live")?;
                tail.recourse(f, format_args!("read it at {HELD_NODE}"))
            }
            Self::NonFiniteVar { var, field } => {
                write!(
                    f,
                    "{var}'s {field} is not finite — the value and every \
                     distribution offset must be a number"
                )?;
                tail.recourse(f, format_args!("give each of them a finite value"))
            }
            // A non-finite offset never arrives here: the edit door
            // routes it to `NonFiniteVar` (`distribution_fault_error`).
            Self::InvalidDistribution { var, fault } => {
                write!(f, "{var} has an invalid distribution: {fault}")?;
                match fault {
                    DistributionFault::NonFinite { .. } => {
                        tail.ending(f, geom_core::KERNEL_DEFECT_ENDING)
                    }
                    DistributionFault::SigmaNotPositive { .. } => {
                        tail.recourse(f, format_args!("give it a sigma above zero"))
                    }
                    DistributionFault::NominalOutsideSupport { .. } => tail.recourse(
                        f,
                        format_args!(
                            "give lo and hi as offsets from the value, lo at or below zero and \
                             hi at or above it"
                        ),
                    ),
                }
            }
            Self::RebindTargetMissingNode { name } => {
                write!(
                    f,
                    "the rebind target ({name}) refers to a node that is not live"
                )?;
                tail.recourse(f, format_args!("rebind to a name of {HELD_NODE}"))
            }
            Self::RebindUnknownName { name } => {
                write!(
                    f,
                    "the rebind source ({name}) was never minted by this document"
                )?;
                tail.recourse(f, format_args!("rebind a name this document minted"))
            }
            Self::RebindKindMismatch { from, to } => {
                write!(
                    f,
                    "a rebind cannot cross entity kinds ({} to {})",
                    from.noun(),
                    to.noun(),
                )?;
                tail.recourse(f, format_args!("rebind it to another {} name", from.noun()))
            }
            Self::RebindIdentity { name } => {
                write!(f, "rebinding {name} to itself would change nothing")?;
                tail.recourse(f, format_args!("rebind it to a different name"))
            }
            Self::RebindNoReferences { name } => {
                write!(
                    f,
                    "no document site references {name}, so there is nothing to repair"
                )?;
                tail.recourse(f, format_args!("rebind a name the document references"))
            }
            Self::WitnessOnNonSketch { node } => {
                write!(
                    f,
                    "{} is not sketch-bearing, so it has nothing to re-witness",
                    node
                )?;
                tail.recourse(f, format_args!("re-witness a sketch-bearing node"))
            }
            Self::DuplicateWitnessEntry { node } => {
                write!(f, "{node} appears twice in the re-witness bulk")?;
                tail.recourse(f, format_args!("list each node once"))
            }
            Self::EmptyWitnessBulk => {
                f.write_str("a re-witness bulk with no entries would change nothing")?;
                tail.recourse(f, format_args!("list at least one node to re-witness"))
            }
            Self::NameUnresolvedInEvaluation { name } => {
                write!(
                    f,
                    "{name} does not resolve in the supplied evaluation — recording the \
                     reference would strand it"
                )?;
                tail.recourse(f, format_args!("name an entity the evaluation holds"))
            }
            Self::EvaluationOfAnotherDocument { expected, found } => {
                write!(
                    f,
                    "the supplied evaluation is of document {found}, not of document \
                     {expected} — its names would be checked against another document's \
                     tables"
                )?;
                tail.recourse(
                    f,
                    format_args!("evaluate this document and supply that evaluation"),
                )
            }
            Self::RebindAppearanceCollision { name, kind } => {
                write!(
                    f,
                    "the rebind would land two {} attributes on {name}",
                    kind.noun()
                )?;
                tail.recourse(f, format_args!("{CLEAR_ONE_FIRST}"))
            }
            Self::AppearanceWrongKind { name } => {
                write!(
                    f,
                    "appearance attaches to faces and bodies only (refused for {name})"
                )?;
                tail.recourse(f, format_args!("set it on a face or a body"))
            }
            Self::AppearanceNamesMissingNode { name } => {
                write!(
                    f,
                    "the appearance target ({name}) refers to a node that is not live"
                )?;
                tail.recourse(f, format_args!("{NAME_A_HELD_ENTITY}"))
            }
            Self::AppearanceNotSet { name, kind } => {
                write!(f, "no {} attribute is set on {name}", kind.noun())?;
                tail.recourse(f, format_args!("clear only an attribute the name carries"))
            }
            Self::InvalidTolerance { value } => {
                write!(f, "tolerance {value:e} is not finite and strictly positive")?;
                tail.recourse(
                    f,
                    format_args!(
                        "keep the tolerance the document records, or offer a positive length \
                         in metres"
                    ),
                )
            }
            Self::MetaUnversioned { name, key, error } => {
                write!(
                    f,
                    "metadata {key:?} on {name} does not carry an integer \"v\" version \
                     field: {error}"
                )?;
                tail.recourse(f, format_args!("store a map with an integer \"v\" entry"))
            }
            Self::MetaNonFinite { name, key, path } => {
                write!(
                    f,
                    "metadata {key:?} on {name} carries a non-finite float at {path}"
                )?;
                tail.recourse(f, format_args!("store a finite number there"))
            }
            Self::MetaNotSet { name, key } => {
                write!(f, "no metadata {key:?} is set on {name}")?;
                tail.recourse(f, format_args!("clear only a key the name carries"))
            }
            Self::RebindMetadataCollision { name, key } => {
                write!(
                    f,
                    "the rebind would land two values under metadata {key:?} on {name}"
                )?;
                tail.recourse(f, format_args!("{CLEAR_ONE_FIRST}"))
            }
            Self::OffsetOnNonInstance { node } => {
                write!(
                    f,
                    "{} does not instantiate a part, so it has no offset to set",
                    node
                )?;
                tail.recourse(
                    f,
                    format_args!("aim the offset at a node that instantiates a part"),
                )
            }
            Self::GaugeOnNonPlaced { node } => {
                write!(
                    f,
                    "{} neither instantiates a part nor is a gauge, so it sits on no gauge",
                    node
                )?;
                tail.recourse(
                    f,
                    format_args!("aim the gauge edit at an instance or a gauge"),
                )
            }
            Self::GaugeNotLive { node, gauge } => {
                write!(
                    f,
                    "{}'s gauge reference names {}, which is not live",
                    node, gauge
                )?;
                tail.recourse(f, format_args!("name a live gauge, or the world"))
            }
            Self::NotAGauge { node, gauge } => {
                write!(
                    f,
                    "{}'s gauge reference names {}, which is not a gauge",
                    node, gauge
                )?;
                tail.recourse(f, format_args!("name a gauge, or the world"))
            }
            Self::GaugeCycle { node, gauge } => {
                write!(f, "{} cannot sit on {}, which sits on it", node, gauge)?;
                tail.recourse(
                    f,
                    format_args!("name a gauge that does not sit on {}", node),
                )
            }
            Self::WouldStartPlacing { mate } => {
                write!(
                    f,
                    "copying the gauge would put both instances of {}, which declares, on one \
                     gauge, so it would start placing",
                    mate
                )?;
                tail.recourse(
                    f,
                    format_args!("delete {}, then copy the gauge and mate", mate),
                )
            }
            Self::PromoteOnNonInstance { node } => {
                write!(
                    f,
                    "{} does not instantiate a part, so it has no offset to promote",
                    node
                )?;
                tail.recourse(f, format_args!("promote an instance"))
            }
            Self::PromoteWithoutOffset { node } => {
                write!(
                    f,
                    "no member of {}'s group carries an offset, so there is no chain to become \
                     a gauge",
                    node
                )?;
                tail.recourse(
                    f,
                    format_args!("set {}'s offset (SetOffset), then promote", node),
                )
            }
            Self::PromoteNonRoot { node, root } => {
                write!(
                    f,
                    "{} is not its group's root: {} is the earliest member carrying an offset, \
                     and that offset is the one that states where the group sits",
                    node, root
                )?;
                tail.recourse(f, format_args!("promote {}", root))
            }
            Self::PromoteMemberOffset { node, member } => {
                write!(
                    f,
                    "{} carries an offset stated in {}'s gauge, and it moves onto the promoted \
                     gauge, where that statement would not hold",
                    member, node
                )?;
                tail.recourse(
                    f,
                    format_args!("clear {}'s offset (SetOffset), then promote", member),
                )
            }
            Self::FoldOnNonGauge { node } => {
                write!(f, "{} is not a gauge, so there is nothing to fold", node)?;
                tail.recourse(f, format_args!("fold a gauge"))
            }
            Self::FoldWouldStartPlacing { node, mate } => {
                write!(
                    f,
                    "folding {} would put both instances of {}, which declares, on one gauge, \
                     so it would start placing",
                    node, mate
                )?;
                tail.recourse(f, format_args!("delete {}, then fold {}", mate, node))
            }
            // The two rule-shaped arms FORWARD the fault set's one
            // prose vocabulary (`PlacementRuleFault`'s `Display`). A
            // rule's shape is written only by the insert that authors
            // its node, so the recourse is that insert's.
            Self::EmptyPlacementList { node } => {
                write!(f, "{node}: {}", PlacementRuleFault::NoPlacements)?;
                tail.recourse(f, format_args!("list at least one placement"))
            }
            Self::PlacementRuleMismatch { node, shape } => {
                write!(
                    f,
                    "{node}: {}",
                    PlacementRuleFault::CountSpelling { shape: *shape }
                )?;
                tail.recourse(
                    f,
                    format_args!(
                        "{}",
                        match shape {
                            CountMismatch::ListedWithCount =>
                                "insert it without a count, since the list is the count",
                            CountMismatch::SteppedWithoutCount => "insert it with a count",
                            CountMismatch::ListedOnPattern =>
                                "keep a pattern of separate copies by giving it a stepped rule, \
                                 or insert a placed union to list the placements, fusing the \
                                 copies into one body",
                        }
                    ),
                )
            }
            Self::ImproperPlacement {
                node,
                at,
                determinant,
            } => frame_refusal(
                f,
                tail,
                node,
                *at,
                FrameFault::Improper {
                    determinant: *determinant,
                },
            ),
            Self::NonRigidPlacement { node, at, check } => {
                frame_refusal(f, tail, node, *at, FrameFault::NotRigid { check })
            }
            Self::PlacementAxis { error } => {
                write!(
                    f,
                    "the placement frame's rotation axis is unusable: {error}"
                )?;
                tail.recourse(
                    f,
                    format_args!("give the rotation axis a direction of nonzero length"),
                )
            }
            Self::NonFinitePlacement { node, at } => {
                frame_refusal(f, tail, node, *at, FrameFault::NonFinite)
            }
            Self::NonFiniteAlignment { node } => {
                write!(f, "{node} carries a non-finite alignment coordinate")?;
                tail.recourse(
                    f,
                    format_args!("give every alignment coordinate a finite value"),
                )
            }
            Self::MateRefused { node, fault, held } => write!(
                f,
                "{node} is refused by the solve on its own datum: {}",
                crate::spoken::Said(
                    &**fault,
                    crate::spoken::Speaker::held(held).about(node.id())
                )
            ),
            Self::UpdateOnNonInstance { node } => {
                write!(
                    f,
                    "{node} does not instantiate a part, so it has no pinned version to update"
                )?;
                tail.recourse(
                    f,
                    format_args!("aim the update at a node that instantiates a part"),
                )
            }
            Self::PinUnchanged { node, pin } => {
                write!(
                    f,
                    "{node} already pins {pin}, so this update would record no version move"
                )?;
                tail.recourse(
                    f,
                    format_args!("offer a version other than the one it pins"),
                )
            }
            Self::LabelUnchanged { node } => {
                match node.label() {
                    Some(_) => write!(f, "{node} already has that label")?,
                    None => write!(f, "{node} has no label to clear")?,
                }
                tail.recourse(f, format_args!("offer a label other than the one it has"))
            }
        }
    }
}

impl core::error::Error for EditError {}

/// The way through a count's unit or distribution: a kind is fixed,
/// so the quantity is a new variable.
const COUNT_HAS_NO_FIELD: &str = "declare a continuous variable in its place";

/// The recourse of a variable reference whose dimension disagrees with
/// the variable's kind. The kind is fixed (VR3), so the reference is
/// the side that can move.
struct ParamDimensionRecourse(Dimension);

impl core::fmt::Display for ParamDimensionRecourse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let referenced = self.0;
        write!(f, "reference a variable declared {referenced}")
    }
}

/// What an accepted edit did (spec D6: structural edits are FLAGGED
/// in the returned record; the record also returns the minted id —
/// without it a caller could never reference an inserted node).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditRecord {
    /// The id an `InsertNode` or a `Promote` minted, `None` otherwise.
    pub minted: Option<RecipeNodeId>,
    /// The variable a `DeclareVar` minted, `None` otherwise.
    pub minted_var: Option<VarId>,
    /// The variables the edit's fresh table minted, entry by entry
    /// ([`DocEdit::InsertNode`]'s `fresh`); empty for an edit with none.
    pub fresh: Vec<VarId>,
    /// The variables an `InsertNode`'s node defines, one per port of
    /// its signature in port order ([`crate::Node::outputs`]); empty
    /// for any other edit.
    pub outputs: Vec<VarId>,
    /// Whether the edit was STRUCTURAL (spec D3/D6): it can change
    /// the result's combinatorial shape — insert/delete, a
    /// Count-slot expression edit, an edit of a Count variable, or an edit
    /// of recipe payload no slot carries (an extrude's side
    /// (`SetExtrudeSide`), a member list, a profile program, among
    /// others; the edit's own arm in `apply` says which).
    /// Continuous edits (`SetParam`, continuous-slot `SetExpression`,
    /// an edit of a continuous variable) leave recipe structure fixed.
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
    /// **The mate door's offset clear** (A11 (2)): inserting a placing
    /// mate that joins two groups places the first operand's group on
    /// the second's, so every member of `a`'s group gives up its
    /// offset — the merged group keeps `b`'s root, and a freshly
    /// inserted part never carries a stray checked offset. One row per
    /// cleared offset. Structural and deterministic from the edit, so
    /// replay reproduces it with no solve, and the offset it cleared
    /// rides here for the caller to read.
    OffsetCleared {
        /// A member of `a`'s group before the mate joined it, spoken
        /// from the document the mate entered.
        instance: SpokenNode,
        /// The offset it carried.
        offset: crate::placement::Placement,
    },
    /// **A payload name this edit stranded** (DM7): `node` survives
    /// and carries `name`, whose referent the edit removed — the node
    /// that minted it ([`DocEdit::DeleteNode`]), or the profile step
    /// it named a piece of ([`DocEdit::SetProgram`], for a name on a
    /// step the reshaping did not keep, or on a kept step's piece it
    /// stopped drawing).
    ///
    /// Either way the name still says exactly what it always said.
    /// After a delete what is gone is the node that minted it, so
    /// evaluation answers [`crate::resolve::ResolveError::NodeGone`] —
    /// rung 1 of the N5 ladder. After a reshaping what is gone is the
    /// piece: a dropped step's id is never minted again, and a kept
    /// step's piece reported here is one the new program does not
    /// draw, so evaluation answers
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
        node: SpokenNode,
        /// The name it carries: its minting node is the one a delete
        /// removed, or its locator names a step a reshaping dropped or
        /// a kept step's piece it stopped drawing.
        name: SpokenName,
        /// Which of those the edit took.
        took: Took,
    },
    /// **A selection's name this edit stranded** (DM7): the selection
    /// `var` survives and names `name`, whose referent the edit removed,
    /// as [`Self::Strand`] says of a payload name. Every reader of the
    /// selection refuses at evaluation with the selection's N5
    /// refusal, and [`DocEdit::Rebind`] is the repair.
    StrandedSelection {
        /// The surviving selection that names it.
        var: SpokenVar,
        /// The nodes that read it, in document order: an unnamed
        /// selection has one, and is spoken by it (VR2).
        readers: Vec<SpokenNode>,
        /// The name.
        name: SpokenName,
        /// What the edit took.
        took: Took,
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
    /// repairs.
    StrandedAppearance {
        /// The key the store holds the attachment under: its `node` is
        /// the id a delete removed, or its locator names a step a
        /// reshaping dropped or a kept step's piece it stopped drawing.
        name: SpokenName,
        /// Which of those the edit took.
        took: Took,
    },
    /// **An operand this edit stranded** (D10; DM7 generalised from
    /// names to reads): `node` survives and its `slot` reads `var`, an
    /// output of the operation the edit deleted. The read keeps its id
    /// and is never re-pointed: evaluation refuses `node` as
    /// [`crate::NodeErrorKind::UnresolvedRead`] until an edit names a
    /// new read for the slot ([`DocEdit::SetParam`],
    /// [`DocEdit::SetMembers`]) or deletes `node`.
    StrandedRead {
        /// The surviving reader.
        node: SpokenNode,
        /// The operand that reads the removed output.
        slot: crate::OperandSlot,
        /// The output it reads, spoken from the document the edit
        /// entered.
        var: SpokenVar,
    },
    /// **An anonymous variable this edit removed** (VR7): the edit
    /// detached the one expression reading it, and a variable with no
    /// name goes with its reader. The mint log keeps its id, so it is
    /// never minted again.
    AnonymousVarRemoved {
        /// The variable, spoken from the document the edit entered.
        var: SpokenVar,
        /// The tolerance it carried, which went with it: an analysis
        /// axis the document no longer has (VR8). `None` for one that
        /// carried none, whose retirement loses nothing a reader of
        /// the document is shown ([`Maintenance::is_silent_retirement`]).
        distribution: Option<Distribution>,
    },
    /// **A label a fold dropped** ([`DocEdit::Fold`]): the gauge went
    /// and no single unlabelled node took its place — it had several
    /// dependents, none, or one already labelled — so its label went
    /// with it.
    LabelDropped {
        /// The gauge folded, spoken from the document the fold entered.
        gauge: SpokenNode,
        /// The label it carried.
        label: crate::Label,
    },
}

impl Maintenance {
    /// **A retirement nothing a reader of the document is shown went
    /// with**: an anonymous variable that carried no tolerance. Its
    /// removal is what rewriting or deleting the slot it was written at
    /// means — a typed value retires one on almost every edit — so a
    /// surface that tells the person what an edit cost leaves it out.
    /// One that carried a tolerance is not silent: the analysis axis it
    /// was went with it.
    pub fn is_silent_retirement(&self) -> bool {
        matches!(
            self,
            Self::AnonymousVarRemoved {
                distribution: None,
                ..
            }
        )
    }
}

/// **What an edit took from a name it stranded** ([`Maintenance::Strand`],
/// [`Maintenance::StrandedAppearance`]), said in the row: the node that
/// minted it (a delete), a profile step it names (a reshaping that
/// dropped the step), or a piece a kept step stopped drawing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Took {
    /// The node that minted the name was deleted.
    Node,
    /// A profile step the name names was dropped.
    Step,
    /// A step the name names was kept and no longer draws its piece.
    Piece,
    /// A read was re-pointed, and the node that minted the name is no
    /// longer upstream of the node that carries it.
    Reach,
}

impl Took {
    /// **What the edit took from `name`, as a strand row says it**: a
    /// delete says the node it deleted, the name's minter — the name's
    /// words say the node that made its leaf, which a carry through a
    /// Boolean's or a fillet's output is not — and a reshaping says
    /// what it dropped.
    #[must_use]
    pub fn said<'a>(&'a self, name: &'a SpokenName) -> impl core::fmt::Display + 'a {
        struct Said<'a>(&'a Took, &'a SpokenName);
        impl core::fmt::Display for Said<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self.0 {
                    Took::Node => {
                        write!(f, "deleted {}, which minted the name", self.1.minter())
                    }
                    Took::Step => f.write_str("dropped a profile step it names"),
                    Took::Piece => {
                        f.write_str("kept a step it names but no longer draws that piece")
                    }
                    Took::Reach => write!(
                        f,
                        "re-pointed a read past {}, which minted the name",
                        self.1.minter()
                    ),
                }
            }
        }
        Said(self, name)
    }
}

impl core::fmt::Display for Maintenance {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OffsetCleared { instance, .. } => write!(
                f,
                "the mate placed {i}'s group, its first operand's, on its second operand's \
                 group, so {i}'s offset was cleared",
                i = instance
            ),
            // The sentence names what was taken as the name's
            // REFERENT ([`Took`]): "the end cap of node 7, which this
            // edit deleted" would read as though the name were deleted,
            // and the name is exactly what survives.
            Self::Strand { node, name, took } => write!(
                f,
                "{} carries a name for {}; this edit {}, so the name resolves to nothing \
                 until it is rebound",
                node,
                name,
                took.said(name)
            ),
            // The same sentence with the store where the carrying
            // node was: what a reader has to know is that the paint
            // is still there and what took its referent. A store holds
            // a thing UNDER a key, and the key is a name for the
            // entity `SpokenName`'s Display says.
            Self::StrandedRead { node, slot, var } => write!(
                f,
                "{node}'s {slot} reads {var}, which this edit deleted with its operation, so \
                 {node} refuses until the {slot} reads a live value"
            ),
            Self::StrandedSelection {
                var,
                readers,
                name,
                took,
            } => match (var.name(), readers.as_slice()) {
                (None, [reader]) => write!(
                    f,
                    "{reader} selects {name}; this edit {}, so {reader} refuses until the name \
                     is rebound",
                    took.said(name)
                ),
                _ => write!(
                    f,
                    "{var} selects {name}; this edit {}, so every reader of {var} refuses until \
                     the name is rebound",
                    took.said(name)
                ),
            },
            Self::StrandedAppearance { name, took } => write!(
                f,
                "the appearance store holds an attachment under a name for {}; this edit \
                 {}, so the name resolves to nothing until it is rebound or cleared",
                name,
                took.said(name)
            ),
            Self::LabelDropped { gauge, label } => write!(
                f,
                "the fold took {} out of the document, and no single unlabelled node stood \
                 in for it, so its label \"{}\" went with it",
                gauge, label
            ),
            Self::AnonymousVarRemoved { var, distribution } => {
                write!(
                    f,
                    "the edit left nothing reading {var}, which had no name, so it went with \
                     its reader"
                )?;
                if distribution.is_some() {
                    f.write_str(", and the tolerance it carried went with it")?;
                }
                Ok(())
            }
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
/// The rows speak their nodes from `before`, the document the door was
/// handed, which still holds the deleted minting node.
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
/// node it names (`dm7_delete_strands`'s
/// `a_carrier_deleted_with_the_node_it_names_reports_nothing`
/// measures it).
fn stranded_references<P: crate::ProfilePayload>(
    before: &Doc<P>,
    doc: &Doc<P>,
    deleted: RecipeNodeId,
) -> Vec<Maintenance> {
    let goes = selections_going(doc);
    doc.name_carriers()
        .filter(|carrier| carrier.name().node == deleted)
        .filter(|carrier| !matches!(carrier, NameCarrier::Select { var, .. } if goes.contains(var)))
        .map(|carrier| match carrier {
            NameCarrier::Payload { node, name } => Maintenance::Strand {
                node: before.spoken(node),
                name: before.spoken_name(name),
                took: Took::Node,
            },
            NameCarrier::Select { var, name } => Maintenance::StrandedSelection {
                var: before.spoken_var(var),
                readers: selection_readers(before, doc, var),
                name: before.spoken_name(name),
                took: Took::Node,
            },
            NameCarrier::Store { name } => Maintenance::StrandedAppearance {
                name: before.spoken_name(name),
                took: Took::Node,
            },
        })
        .collect()
}

/// **A `SetProgram`'s step ids, checked and minted** — before the
/// program is replayed (`names/README.md`, "N1, the profile pieces"):
/// one list per new loop and one entry per authored step; every kept
/// id a step of the program the node holds, and kept once. Each `None`
/// is minted from the document's mint chain, in loop then step order.
/// Returns the new program's ids and the old steps it does not keep.
fn settle_step_ids(
    node: &SpokenNode,
    old: &[Vec<StepId>],
    new: &[crate::program::LoopProgram],
    ids: &[Vec<Option<StepId>>],
    mint: &mut crate::Mint,
) -> Result<(Vec<Vec<StepId>>, std::collections::BTreeSet<StepId>), EditError> {
    use crate::program::{StepIdFault, program_index};
    let refuse = |fault| EditError::StepIdsRefused {
        node: node.clone(),
        fault,
    };
    if ids.len() != new.len() {
        return Err(refuse(StepIdFault::LoopCount {
            loops: new.len(),
            given: ids.len(),
        }));
    }
    let mut dropped: std::collections::BTreeSet<StepId> = old.iter().flatten().copied().collect();
    let held = dropped.clone();
    for (i, (lp, given)) in new.iter().zip(ids).enumerate() {
        if given.len() != lp.authored_steps() {
            return Err(refuse(StepIdFault::Shape {
                loop_: program_index(i),
                authored: lp.authored_steps(),
                given: given.len(),
            }));
        }
        for step in given.iter().flatten() {
            if !held.contains(step) {
                return Err(refuse(StepIdFault::NotThisProfiles { step: *step }));
            }
            if !dropped.remove(step) {
                return Err(refuse(StepIdFault::Repeated { step: *step }));
            }
        }
    }
    let minted = mint.set_program(node.id(), new, ids);
    Ok((minted, dropped))
}

/// **The pieces of kept steps a `SetProgram` stops drawing**, among
/// those a name in `doc` spells ([`StableName::step_pieces`]): not drawn
/// by `new` and drawn by `old` where it replays, under `doc`'s current
/// parameters, as
/// [`crate::ProfilePayload::drawn_pieces`] answers — the one authority
/// on which pieces a program draws. A kept step's piece goes undrawn
/// when another piece takes its segment (N1, "Undrawn pieces vanish
/// rather than alias"): a fillet inserted or moved before its leg.
///
/// A piece `old` draws and `new` does not is the edit's to report; one
/// `old` already left undrawn is not, since the edit removed nothing.
/// Where `old` does not replay under the current parameters (a legal
/// at-rest state) there is no drawn set to compare against, and every
/// named kept piece `new` does not draw is reported: the edit removes
/// the referent's future either way. Neither program is replayed when
/// no name spells a kept step's piece.
fn undrawn_kept_pieces<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    written: &Doc<P>,
    node: RecipeNodeId,
    old: &P,
    new: &P,
    dropped: &std::collections::BTreeSet<StepId>,
    tol: Tol,
) -> Result<std::collections::BTreeSet<ProfileEdgeRef>, EditError> {
    let Some(ids) = old.step_ids() else {
        return Ok(std::collections::BTreeSet::new());
    };
    let kept: std::collections::BTreeSet<StepId> = ids
        .iter()
        .flatten()
        .filter(|s| !dropped.contains(s))
        .copied()
        .collect();
    let named: std::collections::BTreeSet<ProfileEdgeRef> = doc
        .name_carriers()
        .flat_map(|c| c.name().step_pieces())
        .filter(|p| p.step().is_some_and(|s| kept.contains(&s)))
        .collect();
    if named.is_empty() {
        return Ok(named);
    }
    let env = doc.var_env::<f64>();
    let refused = |refusal| EditError::ProfileProgramRefused {
        node: doc.spoken(node),
        refusal: Box::new(refusal),
    };
    let before = match old.drawn_pieces(&env, tol) {
        Ok(drawn) => Some(drawn),
        Err(refusal @ crate::ProgramRefusal::Pieces(_)) => return Err(refused(refusal)),
        Err(_) => None,
    };
    // The new program reads the variables its lowering minted, which
    // the document it is written into holds.
    let after = new
        .drawn_pieces(&written.var_env::<f64>(), tol)
        .map_err(refused)?;
    Ok(named
        .into_iter()
        .filter(|p| before.as_ref().is_none_or(|b| b.contains(p)) && !after.contains(p))
        .collect())
}

/// **The names a `SetProgram` stranded**: every carried name that
/// spells a piece of a step the reshaping dropped
/// ([`StableName::piece_steps`]), or a kept step's piece it stopped
/// drawing (`undrawn`, [`undrawn_kept_pieces`]) —
/// [`Maintenance::Strand`] on its carrying node,
/// [`Maintenance::StrandedAppearance`] on a store key, in that order.
/// Nothing is rewritten: the name keeps its spelling and resolves
/// `Vanished`. A step id is unique across the document, so which node
/// minted the name does not enter. The rows speak their nodes from
/// `before`, a kept step where `doc` draws it and a dropped one where
/// it sat in `before` ([`crate::SpokenName::steps_respoken`]).
fn stranded_steps<P: crate::ProfilePayload>(
    before: &Doc<P>,
    doc: &Doc<P>,
    dropped: &std::collections::BTreeSet<StepId>,
    undrawn: &std::collections::BTreeSet<ProfileEdgeRef>,
) -> Vec<Maintenance> {
    if dropped.is_empty() && undrawn.is_empty() {
        return Vec::new();
    }
    let mut strands = Vec::new();
    let mut keys = Vec::new();
    let goes = selections_going(doc);
    for carrier in doc.name_carriers() {
        if matches!(carrier, NameCarrier::Select { var, .. } if goes.contains(&var)) {
            continue;
        }
        let pieces = carrier.name().step_pieces();
        let took = if pieces
            .iter()
            .any(|p| p.step().is_some_and(|s| dropped.contains(&s)))
        {
            Took::Step
        } else if pieces.iter().any(|p| undrawn.contains(p)) {
            Took::Piece
        } else {
            continue;
        };
        match carrier {
            NameCarrier::Payload { node, name } => strands.push(Maintenance::Strand {
                node: before.spoken(node),
                name: before.spoken_name(name).steps_respoken(doc),
                took,
            }),
            NameCarrier::Select { var, name } => strands.push(Maintenance::StrandedSelection {
                var: before.spoken_var(var),
                readers: selection_readers(before, doc, var),
                name: before.spoken_name(name).steps_respoken(doc),
                took,
            }),
            NameCarrier::Store { name } => {
                keys.push(Maintenance::StrandedAppearance {
                    name: before.spoken_name(name).steps_respoken(doc),
                    took,
                });
            }
        }
    }
    strands.extend(keys);
    strands
}

/// **One appearance record moved onto the key `to`**, attribute by
/// attribute and metadata entry by entry, refusing where `to` already
/// carries the same kind or key: which value survives would be an
/// auto-pick. The store half of [`DocEdit::Rebind`]'s name rewrite,
/// speaking its refusal from `before`, the document the door was handed.
fn move_appearance_record<P: crate::ProfilePayload>(
    before: &Doc<P>,
    store: &mut crate::appearance::AppearanceMap,
    moved: crate::appearance::AppearanceRecord,
    to: &StableName,
) -> Result<(), EditError> {
    let dst = store.entry(to.clone()).or_default();
    for (kind, attr) in moved.attrs {
        if dst.attrs.contains_key(&kind) {
            return Err(EditError::RebindAppearanceCollision {
                name: before.spoken_name(to),
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
                name: before.spoken_name(to),
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
    /// **What the edit did that the caller did not ask for**: the
    /// references it stranded (DM7) — the payload names, then the
    /// appearance keys — and the offset the mate door cleared. See [`Maintenance`].
    ///
    /// **The order is a CONTRACT, not an accident of the
    /// implementation, and a consumer may rely on it**: every
    /// [`Maintenance::Strand`] first, in the document's node order
    /// and within one node in the payload's own order; then every
    /// [`Maintenance::StrandedAppearance`], in the appearance store's
    /// key order. The strands are read at the door, out of the
    /// document the edit had just produced. A delete and a
    /// `SetProgram` report strands; the insert of a placing
    /// mate that joins two groups reports one
    /// [`Maintenance::OffsetCleared`] per offset its first operand's
    /// group held, in document order, and nothing else; no other edit
    /// reports anything.
    ///
    /// The paragraph above is the contract — it is stated here in
    /// full because a consumer outside this crate cannot read
    /// `Carrier::ALL`, which is `pub(crate)`. In-crate the order has
    /// one home all the same: one roster, `Carrier::ALL`, drives the
    /// walk both doors read (`Doc::name_carriers`) — filtered on the
    /// deleted node for a delete, on the dropped steps and the
    /// undrawn kept pieces for a program edit — so the strands' order
    /// is that roster's, and a reader who wants to see why reads it
    /// there.
    ///
    /// Each boundary is held by the row whose fixture actually
    /// produces the pair of kinds it separates:
    /// `dm7_delete_strands::an_appearance_strand_follows_the_payload_strands_of_the_same_delete`
    /// for payload strand before appearance strand.
    /// What a consumer may NOT do is read position 0 as a kind: a
    /// delete that strands no payload name puts an appearance strand
    /// there, so an arm is found by matching, never by
    /// index.
    ///
    /// Every row is a function of the document and the edit alone, so
    /// replaying the edit reproduces it.
    pub maintenance: Vec<Maintenance>,
}

/// **An action's maintenance, net of what the action itself made
/// moot** — the rows several accepted edits reported, folded into what
/// is true of the document the action ENDS at.
///
/// [`Applied::maintenance`] is a function of one `(document, edit)`
/// pair and answers what that edit did. An action — a cascade delete
/// ([`cascade_delete_order`]'s sequence), a parameter's value and its
/// notation written together — is several edits, and a row one of them
/// reported can be about nothing the action leaves behind: a cascade's
/// early delete strands a name on a carrier a later one deletes. This
/// is the one
/// spelling of which rows survive, so every caller that holds a
/// sequence (the viewer's session, the pre-click count a chrome states
/// before a cascade) answers the same.
///
/// Fed one accepted edit at a time with [`Self::push`], in the order
/// they applied; closed with [`Self::finish`] against the document the
/// last one produced.
///
/// # What survives
///
/// - A [`Maintenance::Strand`] survives when its carrier is live at the
///   end AND still holds the stranded name: a carrier a later edit
///   deleted, or whose name a later edit rewrote (a [`DocEdit::Rebind`]
///   repairs exactly this), strands nothing.
/// - A [`Maintenance::StrandedAppearance`] survives when the store
///   still holds its key.
/// - A [`Maintenance::StrandedRead`] survives when its reader is live at
///   the end and its operand still reads the removed output: a later
///   edit that deleted the reader or re-pointed the operand took it
///   back.
/// - A [`Maintenance::OffsetCleared`] survives when the instance is
///   live at the end and still carries no offset: a later edit that
///   deleted it, or gave it an offset again, took the report back.
/// - A [`Maintenance::LabelDropped`] always survives: the gauge is
///   gone and its id is never minted again.
///
/// Surviving rows keep the order the edits reported them in, each
/// edit's rows in [`Applied::maintenance`]'s own order.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MaintenanceNet {
    rows: Vec<Maintenance>,
}

impl MaintenanceNet {
    /// No edit yet.
    pub fn new() -> Self {
        Self::default()
    }

    /// Fold in one accepted edit's rows, in the order it reported
    /// them. [`Applied`] carries the rows with the document they were
    /// read out of, so the rows are a door's by construction.
    pub fn push<P>(&mut self, edit: &Applied<P>) {
        self.rows.extend(edit.maintenance.iter().cloned());
    }

    /// The rows that survive, against `end` — the document the last
    /// pushed edit produced.
    pub fn finish<P: crate::ProfilePayload>(self, end: &Doc<P>) -> Vec<Maintenance> {
        self.rows
            .into_iter()
            .filter(|row| match row {
                Maintenance::Strand { node, name, .. } => end
                    .node(node.id())
                    .is_some_and(|carrier| carrier.payload_names().contains(&name.name())),
                Maintenance::StrandedSelection { var, name, .. } => end
                    .selection(var.id())
                    .is_some_and(|select| select.names.contains(name.name())),
                Maintenance::StrandedAppearance { name, .. } => {
                    end.appearance().contains_key(name.name())
                }
                Maintenance::StrandedRead { node, slot, var } => {
                    end.node(node.id()).is_some_and(|reader| {
                        reader.operand_rows().into_iter().any(|(at, read)| {
                            at == *slot && end.selection(read).map_or(read, |s| s.body) == var.id()
                        })
                    })
                }
                Maintenance::OffsetCleared { instance, .. } => matches!(
                    end.node(instance.id()),
                    Some(Node::InstantiatePart { offset: None, .. })
                ),
                Maintenance::LabelDropped { .. } | Maintenance::AnonymousVarRemoved { .. } => true,
            })
            .collect()
    }
}

/// **An action of several edits, under way.** Each edit is applied to
/// the document the one before it produced, starting from a document
/// the recording borrows, and its [`Applied`] is taken up whole: the
/// edit joins the list, the id it minted joins the minted list, and
/// its maintenance is folded into a [`MaintenanceNet`].
/// [`Self::finish`] answers the action as a [`Recorded`].
///
/// **A refusal ends the action.** A refused edit records nothing, every
/// edit after it is refused with that same refusal and applies nothing,
/// and [`Self::finish`] answers it in place of the action. So an action
/// is all or nothing whether or not its caller stops at the refusal: a
/// caller that goes on past one cannot finish with the edits around it.
/// A caller asking whether an edit would land asks [`apply`] against
/// [`Self::doc`], which records nothing.
///
/// Nothing is written anywhere until a caller takes the [`Recorded`]
/// up, so undo is the caller keeping the document it started from.
pub struct Recording<'a, P: crate::ProfilePayload> {
    start: &'a Doc<P>,
    produced: Option<Doc<P>>,
    refused: Option<EditError>,
    edits: Vec<DocEdit<P>>,
    minted: Vec<Option<RecipeNodeId>>,
    maintenance: MaintenanceNet,
    tol: Tol,
    reach: &'a dyn MateReach,
}

impl<'a, P: Clone + crate::ProfilePayload> Recording<'a, P> {
    /// An action starting from `doc`, each edit applied at `tol`
    /// through `reach` ([`apply`]).
    pub fn start(doc: &'a Doc<P>, tol: Tol, reach: &'a dyn MateReach) -> Self {
        Self {
            start: doc,
            produced: None,
            refused: None,
            edits: Vec::new(),
            minted: Vec::new(),
            maintenance: MaintenanceNet::new(),
            tol,
            reach,
        }
    }

    /// The document so far: the last edit's, or the start before any.
    pub fn doc(&self) -> &Doc<P> {
        self.produced.as_ref().unwrap_or(self.start)
    }

    /// What each edit so far minted, in the order the edits applied: an
    /// `InsertNode`'s new id, `None` for every other edit.
    pub fn minted(&self) -> &[Option<RecipeNodeId>] {
        &self.minted
    }

    /// Whether the action has recorded no edit yet.
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }

    /// Apply `edit` to the document so far and record it. Returns the
    /// id it minted, if any.
    ///
    /// # Errors
    ///
    /// The edit's own refusal ([`apply`]'s), which ends the action; or,
    /// once an earlier edit has ended it, that edit's refusal. Nothing
    /// is recorded on that arm.
    pub fn apply(&mut self, edit: DocEdit<P>) -> Result<Option<RecipeNodeId>, EditError> {
        self.apply_recorded(edit).map(|record| record.minted)
    }

    /// [`Self::apply`], answering the edit's whole record: what it
    /// minted, its fresh table's variables among it.
    ///
    /// # Errors
    ///
    /// As [`Self::apply`]'s.
    pub fn apply_recorded(&mut self, edit: DocEdit<P>) -> Result<EditRecord, EditError> {
        self.open()?;
        let applied = apply(self.doc(), &edit, self.tol, self.reach).map_err(|e| self.end(e))?;
        let record = applied.record.clone();
        self.take(edit, applied);
        Ok(record)
    }

    /// Insert `node` and record the insert — [`Self::apply`] of its
    /// [`DocEdit::InsertNode`] — answering the id it minted.
    ///
    /// # Errors
    ///
    /// As [`Self::apply`]'s.
    pub fn insert(&mut self, node: Node<P::Authored, Formula>) -> Result<RecipeNodeId, EditError> {
        self.open()?;
        let (applied, id) =
            apply_insert(self.doc(), &node, self.tol, self.reach).map_err(|e| self.end(e))?;
        self.take(
            DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            applied,
        );
        Ok(id)
    }

    /// **Measures, inserted** ([`fn@measure`]): one [`Node::Measure`]
    /// per primitive, in order; returns the measures and their outputs.
    /// Arithmetic over the outputs is an ordinary formula over them.
    ///
    /// # Errors
    ///
    /// The insert door's refusals.
    pub fn measure<R: Clone + Into<crate::Operand>>(
        &mut self,
        primitives: &[crate::measure::MeasurePrimitive<R>],
    ) -> Result<Measured, EditError> {
        let mut measured = Measured {
            measures: Vec::with_capacity(primitives.len()),
            outputs: Vec::with_capacity(primitives.len()),
        };
        for primitive in primitives {
            let id = self.insert(Node::measure(primitive))?;
            let Some(output) = self.doc().output(id, 0) else {
                unreachable!("an inserted measure defines its value")
            };
            measured.measures.push(id);
            measured.outputs.push(output);
        }
        Ok(measured)
    }

    /// Declare a variable and record the declare — [`Self::apply`] of
    /// its [`DocEdit::DeclareVar`] — answering the id it minted.
    ///
    /// # Errors
    ///
    /// As [`Self::apply`]'s.
    pub fn declare(&mut self, name: VarName, def: VarDecl) -> Result<VarId, EditError> {
        self.open()?;
        let edit = DocEdit::DeclareVar { name, def };
        let applied = apply(self.doc(), &edit, self.tol, self.reach).map_err(|e| self.end(e))?;
        let Some(id) = applied.record.minted_var else {
            unreachable!("an accepted declare mints a variable")
        };
        self.take(edit, applied);
        Ok(id)
    }

    /// The refusal that ended the action, if one has.
    fn open(&self) -> Result<(), EditError> {
        self.refused.clone().map_or(Ok(()), Err)
    }

    fn end(&mut self, refusal: EditError) -> EditError {
        self.refused = Some(refusal.clone());
        refusal
    }

    fn take(&mut self, edit: DocEdit<P>, applied: Applied<P>) -> Option<RecipeNodeId> {
        self.maintenance.push(&applied);
        self.edits.push(edit);
        self.minted.push(applied.record.minted);
        self.produced = Some(applied.doc);
        applied.record.minted
    }

    /// The action, its maintenance netted against the document its
    /// last edit produced.
    ///
    /// # Errors
    ///
    /// The refusal that ended the action, if an edit was refused.
    pub fn finish(self) -> Result<Recorded<P>, EditError> {
        self.open()?;
        let doc = self.produced.unwrap_or_else(|| self.start.clone());
        Ok(Recorded {
            maintenance: self.maintenance.finish(&doc),
            doc,
            edits: self.edits,
            minted: self.minted,
        })
    }
}

/// **An action of several edits, applied** ([`Recording::finish`]): the
/// document it produced and the record of how. Undo is the caller
/// keeping the document the recording started from.
#[derive(Debug, Clone, PartialEq)]
pub struct Recorded<P: crate::ProfilePayload> {
    /// The document the last edit produced; the start's, unchanged,
    /// when the action recorded no edit.
    pub doc: Doc<P>,
    /// The edits that produce `doc` from the start, in the order they
    /// applied.
    pub edits: Vec<DocEdit<P>>,
    /// The maintenance `edits` reported, net of what a later edit in
    /// the list took back ([`MaintenanceNet`]), in edit order.
    pub maintenance: Vec<Maintenance>,
    /// What each edit minted, in the order the edits applied.
    pub minted: Vec<Option<RecipeNodeId>>,
}

/// A name written into the document spells only steps the document
/// minted ([`EditError::NameStepNeverMinted`]) — the edit door's half
/// of the load door's `SnapshotError::NameStepNotMinted`, so a
/// document this door accepts is one the load door reads back. `doc`
/// is the document being written, `before` the one the door was handed.
fn check_name_steps<P: crate::ProfilePayload>(
    before: &Doc<P>,
    doc: &Doc<P>,
    name: &StableName,
) -> Result<(), EditError> {
    match name
        .piece_steps()
        .into_iter()
        .find(|s| !doc.mint.has_step(*s))
    {
        None => Ok(()),
        Some(step) => Err(EditError::NameStepNeverMinted {
            name: before.spoken_name(name),
            step,
        }),
    }
}

/// **The node an edit writes, as its refusal speaks it**: as `doc`
/// holds it, or, for the node an insert is minting and `doc` does not
/// hold yet, by its kind and tag ([`SpokenNode::entering`]).
///
/// **Which document an edit refusal speaks from.** A node the door was
/// handed is spoken from that document, as it stood before the edit
/// (`doc` in [`door`] and the writers it hands the working copy to,
/// `before` in their helpers), never from the working copy the edit is
/// writing. A node the edit is minting is spoken by
/// [`SpokenNode::entering`]. An id neither holds is
/// [`SpokenNode::absent`].
fn written<P>(doc: &Doc<P>, id: RecipeNodeId, node: &Node<P>) -> SpokenNode {
    if doc.node(id).is_some() {
        doc.spoken(id)
    } else {
        SpokenNode::entering(id, node)
    }
}

/// **A node an edit's result names, as its refusal speaks it**: from
/// `before` when it holds the node, else by its kind as `after` mints
/// it ([`written`]), else [`SpokenNode::absent`]: a refusal speaks the
/// node as the author handed it.
fn spoken_before_else_after<P>(before: &Doc<P>, after: &Doc<P>, id: RecipeNodeId) -> SpokenNode {
    match after.node(id) {
        Some(node) => written(before, id, node),
        None => before.spoken(id),
    }
}

/// A broken E2 invariant as the edit door reports it, in ONE place.
///
/// The split is by CLASS, not by door: a non-finite offset is a
/// non-finite float on a document parameter and joins the ruled
/// non-finite policy's own refusal (door 1), the rest are distribution
/// shape faults. Every door that writes a definition and the
/// annotation door reach it, so a caller comparing their refusals reads
/// one answer rather than two spellings of it.
fn distribution_fault_error(var: &SpokenVar, fault: DistributionFault) -> EditError {
    match fault {
        DistributionFault::NonFinite { field } => EditError::NonFiniteVar {
            var: var.clone(),
            field: crate::doc::DocParamField::Offset(field),
        },
        DistributionFault::SigmaNotPositive { .. }
        | DistributionFault::NominalOutsideSupport { .. } => EditError::InvalidDistribution {
            var: var.clone(),
            fault,
        },
    }
}

/// **The checks every door that writes a definition runs**, so no two
/// of them can come to disagree about what a legal variable is:
/// [`DocEdit::DeclareVar`], [`DocEdit::DefineVar`] and the three
/// carry-forward doors, through [`write_free`].
///
/// **The NAME is not checked here, because it cannot be wrong**: a
/// [`VarName`] is admissible by construction, and the load door refuses
/// one at the token (`VarName`'s `Deserialize` is the same
/// constructor).
///
/// **The check order is the LOAD door's** (`persist::check`'s
/// `validate_document`): floats first, then the distribution's shape,
/// then the notation. A variable broken in two ways at once therefore
/// gets the same VERDICT whichever door refuses it, and names the same
/// one of its two faults.
///
/// For one definition the two doors reach that verdict by different
/// rules, and say so in different words: a CONTINUOUS variable of kind
/// `Count` is refused here as
/// [`EditError::ContinuousVarCannotBeCount`], and at the load door by
/// the notation walk one step earlier (`PersistError::DisplayUnit`),
/// because no unit in the table measures a count. Both refuse the same
/// definitions; only this door can name the structural/continuous
/// divide as the reason.
fn check_var_def(var: &SpokenVar, def: &WrittenDef) -> Result<(), EditError> {
    // A definition holds no float; what it reads is
    // `check_definition`'s.
    let WrittenDef::Free(value) = def else {
        return Ok(());
    };
    // Ruled door 1 (non-finite policy): recipe data never carries
    // NaN/inf — the nominal and the distribution offsets alike, by the
    // ONE predicate the load door's float walk asks
    // (`FreeVar::first_non_finite`), which is also what decides WHICH
    // float this refusal names.
    if let Some(field) = value.first_non_finite() {
        return Err(EditError::NonFiniteVar {
            var: var.clone(),
            field,
        });
    }
    // The REST of E2's invariants, from the ONE shared check the
    // persistence doors also run.
    if let Some(d) = value.distribution()
        && let Err(fault) = d.check()
    {
        return Err(distribution_fault_error(var, fault));
    }
    // The structural/continuous divide (`FreeVar::is_continuous_count`).
    if value.is_continuous_count() {
        return Err(EditError::ContinuousVarCannotBeCount { var: var.clone() });
    }
    // The unit/dimension pairing, at EVERY door that writes a
    // definition: a `FreeVar` is a `pub` payload, so a caller can state
    // a mismatched pair, and an in-memory document must never hold a
    // variable no file could carry. `measures()` is the same predicate
    // the notation door and the validator ask.
    if let FreeVar::Continuous {
        dim, display_unit, ..
    } = *value
    {
        let measured = display_unit.measures();
        if measured != dim {
            return Err(EditError::VarUnitMismatch {
                var: var.clone(),
                unit: measured,
                declared: dim,
            });
        }
    }
    Ok(())
}

/// Write `value` as the definition of the standing variable `id`: the
/// shared tail of the carry-forward doors. The kind cannot move here —
/// each door carries the variable's own kind forward — so no reader is
/// re-checked.
fn write_free<P>(
    new: &mut Doc<P>,
    id: VarId,
    var: &SpokenVar,
    value: FreeVar,
) -> Result<EditRecord, EditError> {
    let def = WrittenDef::Free(value);
    check_var_def(var, &def)?;
    let structural = def.kind() == VarKind::Count;
    new.vars.insert(id, Var::written(def));
    Ok(EditRecord {
        minted: None,
        minted_var: None,
        structural,
        fresh: Vec::new(),
        outputs: Vec::new(),
    })
}

impl EditError {
    /// **[`EditError::VarIsAnOutput`] for `var` at `door`**, where `var`
    /// is an operation's output in `doc`, and `None` otherwise: the one
    /// spelling of the refusal, which every door that writes or deletes
    /// a variable asks, and a client opening such an edit asks first.
    #[must_use]
    pub fn output_refusal<P>(doc: &Doc<P>, var: VarId, door: CarryForwardDoor) -> Option<Self> {
        let (node, _) = doc.var(var)?.def().output()?;
        Some(Self::VarIsAnOutput {
            var: doc.spoken_var(var),
            node: doc.spoken(node),
            door,
        })
    }
}

/// [`EditError::output_refusal`], as a door's early return.
fn refuse_output<P>(doc: &Doc<P>, id: VarId, door: CarryForwardDoor) -> Result<(), EditError> {
    EditError::output_refusal(doc, id, door).map_or(Ok(()), Err)
}

/// The live FREE variable an edit of a standing variable addresses, or
/// that door's refusal: a variable the document does not hold, an
/// output, or a defined one, which holds no value, notation or
/// distribution of its own.
fn standing_var<P>(
    doc: &Doc<P>,
    var: &VarRef,
    door: CarryForwardDoor,
) -> Result<(VarId, SpokenVar, FreeVar), EditError> {
    let Some((id, def)) = doc
        .resolve_var(var)
        .and_then(|id| Some((id, doc.var(id)?.def())))
    else {
        return Err(EditError::UnknownVar {
            var: var.clone(),
            door,
        });
    };
    match def {
        VarDef::Free(free) => Ok((id, doc.spoken_var(id), free.clone())),
        VarDef::Output { .. } => Err(EditError::output_refusal(doc, id, door)
            .unwrap_or_else(|| unreachable!("an output's refusal is the output's"))),
        VarDef::Defined(_) | VarDef::Select(_) => Err(EditError::NotAFreeVar {
            var: doc.spoken_var(id),
            door,
        }),
    }
}

/// **The checks a written definition answers to**, against the
/// document it was written into: every variable it reads is one the
/// document holds, at its kind; it does not read itself back (VR3);
/// and no variable's expansion outgrows [`DEFINITION_NODE_BOUND`].
/// A free definition reads nothing and passes.
fn check_definition<P>(new: &Doc<P>, id: VarId) -> Result<(), EditError> {
    let Some(expr) = new.var(id).and_then(|v| v.def().defined()) else {
        return Ok(());
    };
    let var = new.spoken_var(id);
    if let Some(fault) = new.var_read_faults(expr).into_iter().next() {
        return Err(match fault {
            VarReadFault::Unminted { var: read } | VarReadFault::Dead { var: read } => {
                EditError::DefinitionUnresolvedVar {
                    var,
                    read: new.spoken_var(read),
                }
            }
            VarReadFault::Kind {
                var: read,
                declared,
                referenced,
            } => EditError::DefinitionVarKind {
                var,
                read: new.spoken_var(read),
                declared,
                referenced,
            },
        });
    }
    // A redefinition can close a cycle or grow every expansion that
    // reads it, so the whole document is asked, by the search the load
    // walk asks too.
    expansion_refusal(new)
}

/// **The expansion search's answer as this door refuses it**: a
/// definition cycle, or an expansion past the bound
/// ([`DEFINITION_NODE_BOUND`]).
fn expansion_refusal<P>(new: &Doc<P>) -> Result<(), EditError> {
    match new.expansion_fault() {
        None => Ok(()),
        Some(ExpansionFault::Cycle { var, through }) => Err(EditError::DefinitionCycle {
            var: new.spoken_var(var),
            through: through.into_iter().map(|v| new.spoken_var(v)).collect(),
        }),
        Some(ExpansionFault::TooLarge { var, nodes }) => Err(EditError::DefinitionTooLarge {
            var: new.spoken_var(var),
            nodes,
        }),
    }
}

/// Validate every slot of a node payload against slot dimensions and
/// the variable table, keyed as `id` for error reporting.
fn check_node_slots<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    before: &Doc<P>,
    id: RecipeNodeId,
    node: &Node<P>,
) -> Result<(), EditError> {
    // The variable each slot reads, against the table, at the
    // dimension its address fixes (VR4): a variable of another kind is
    // the read's kind fault. The formulas' own dimensions are the
    // lowering's question, asked before it.
    for (slot, &var) in node.rows() {
        let reader = Expr::var(var, slot.expr_dimension());
        check_reads(
            doc,
            &written(before, id, node),
            ExprSite::Slot(slot),
            &reader,
        )?;
    }
    // The variables the expressions no slot addresses read (E3/E10),
    // each at the dimension its leaf, or its bound's measure, reads it.
    for (var, dim) in node.payload_reads(doc).into_iter().flatten() {
        let reader = Expr::var(var, dim);
        check_reads(doc, &written(before, id, node), ExprSite::Payload, &reader)?;
    }
    // An assertion's bound against the dimension of its value (E10):
    // `Node::assertion_bound_fault`, the one home the load door reads it
    // from too.
    if let Some(AssertionBoundFault { measured, bound }) = node.assertion_bound_fault(doc) {
        return Err(EditError::AssertionDimension {
            node: written(before, id, node),
            measured,
            bound,
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
    doc: &Doc<P>,
    id: RecipeNodeId,
    node: &Node<P>,
) -> Result<(), EditError> {
    let Some(fault) = node.input_fault() else {
        return Ok(());
    };
    let subject = written(doc, id, node);
    Err(match fault {
        crate::node::InputFault::Duplicate { input } => EditError::DuplicateInput {
            node: subject,
            input: doc.defined_by(input).map_or_else(
                || unreachable!("an operand the door wrote reads a live output"),
                |(at, _)| doc.spoken(at),
            ),
        },
        crate::node::InputFault::TooFew { found } => EditError::TooFewMembers {
            node: subject,
            found,
        },
    })
}

/// Spec D3 carve-out (ruled): a payload's name refs must point at LIVE
/// nodes at edit time — a never-existed id is a typo. They are not DAG
/// edges: later deletes may strand them (N5), so the edits that WRITE a
/// payload are the only doors that check, and they check every payload
/// name — [`Node::payload_names`] is the list. The same holds for the
/// node a reference is READ AT where that node is not also an input
/// ([`Node::payload_read_sites`]): a never-existed id is a typo; a later
/// delete stranding it is the evaluation's to refuse.
///
/// `doc` is the document the door was handed, `new` the one being
/// written.
fn check_payload_refs<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    new: &Doc<P>,
    node: &Node<P>,
) -> Result<(), EditError> {
    for name in node.payload_names() {
        if !new.nodes.contains_key(&name.node) {
            return Err(EditError::DeclareNamesMissingNode {
                name: doc.spoken_name(name),
            });
        }
        check_name_steps(doc, new, name)?;
    }
    for at in node.payload_read_sites() {
        if !new.nodes.contains_key(&at) {
            return Err(EditError::ReadSiteMissingNode {
                at: SpokenNode::absent(at),
            });
        }
    }
    Ok(())
}

/// [`crate::node::declared_side_fault`] asked of the sides `sides` a
/// door writes onto `node`, refused typed. `carrier` speaks the node
/// and `at` is its id (`None` for a node being inserted): every door
/// that writes a pair asks this, so a pair no door admits is one no
/// document holds.
fn check_declared_sides<'p, P: crate::ProfilePayload>(
    doc: &Doc<P>,
    new: &Doc<P>,
    node: &Node<P>,
    sides: impl IntoIterator<Item = &'p crate::SitedRef>,
    carrier: impl Fn() -> SpokenNode,
    at: Option<RecipeNodeId>,
) -> Result<(), EditError> {
    // A declared site is a node whose output this node reads.
    let operands: Vec<RecipeNodeId> = node
        .operand_rows()
        .into_iter()
        .filter_map(|(_, read)| new.defined_by(read).map(|(site, _)| site))
        .collect();
    // A name its carrier's operands could hold is minted by a node the
    // carrier reads, directly or through what it reads (D10: reading is
    // the only dependency, so no position decides it): the carrier's
    // strict ancestors, or — for a node not yet inserted — the
    // operations it reads and theirs.
    let reach: std::collections::BTreeSet<RecipeNodeId> = match at {
        Some(at) => crate::doc::strict_ancestors(new, at),
        None => operands
            .iter()
            .flat_map(|&site| std::iter::once(site).chain(crate::doc::strict_ancestors(new, site)))
            .collect(),
    };
    let upstream = |minter: RecipeNodeId| reach.contains(&minter);
    match crate::node::declared_side_fault(sides, Some(&operands), upstream) {
        None => Ok(()),
        Some((side, crate::node::DeclaredSideFault::SiteNotAnOperand)) => {
            Err(EditError::DeclaredSiteNotAnOperand {
                node: carrier(),
                name: doc.spoken_name(&side.name),
                site: doc.spoken(side.at),
            })
        }
        Some((side, crate::node::DeclaredSideFault::NameNotUpstream)) => {
            Err(EditError::DeclaredNameNotUpstream {
                node: carrier(),
                name: doc.spoken_name(&side.name),
            })
        }
    }
}

/// What of a node a door writes ([`check_written_node`]).
#[derive(Clone, Copy)]
enum Writes {
    /// The whole node (`InsertNode`): its payload is asked too.
    Node,
    /// Its reads only (the slot door at an operand): the payload it
    /// leaves as it was is not asked again. A payload name a moved
    /// read takes out of reach is reported, never refused
    /// ([`Maintenance::Strand`] with [`Took::Reach`],
    /// [`stranded_by_repoint`]), and one an earlier delete stranded
    /// does not block the re-point that repairs its reader.
    Reads,
}

/// **The checks every door that writes a node asks of it as it will
/// stand** (DM6): `node` is `id`'s content after the edit, its reads
/// lowered, judged against `new` before it is written there. One
/// function, called by `InsertNode` of the node it mints and by the
/// slot door ([`DocEdit::SetParam`] at an operand,
/// [`DocEdit::SetMembers`]) of the node it rewrites, so no door admits
/// a node another refuses:
///
/// - its payload's names and read sites live ([`check_payload_refs`])
///   and its declared pairs ([`check_declared_sides`]), where the door
///   writes the payload (`writes`);
/// - DM5 and the list floor ([`check_node_inputs`]);
/// - a gauge reference live and of a gauge ([`check_gauge_ref`]);
/// - a decidable alignment, and a mate's frame offsets;
/// - the variable every slot and payload expression reads, a measure's
///   reference indices, and an assertion's bound against the dimension
///   of the measure it reads ([`check_node_slots`]);
/// - a profile program that replays under the current values.
///
/// Acyclicity is the document's, asked once the node is written.
fn check_written_node<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    new: &Doc<P>,
    id: RecipeNodeId,
    node: &Node<P>,
    writes: Writes,
    tol: Tol,
) -> Result<(), EditError> {
    let spoken = || written(doc, id, node);
    if let Writes::Node = writes {
        check_payload_refs(doc, new, node)?;
    }
    check_node_inputs(doc, id, node)?;
    if let Writes::Node = writes {
        check_declared_sides(
            doc,
            new,
            node,
            crate::node::declared_sides(node.declared_pairs()),
            spoken,
            None,
        )?;
    }
    // A gauge reference is a reading edge, as a mate's operand
    // is: a never-live or wrong-kind one is a typo, refused here.
    check_gauge_ref(new, id, node.gauge_ref(), spoken)?;
    // ASM-R2a D-1, through `Node::has_non_finite_alignment` —
    // the one place a node is asked whether its alignment datum
    // is decidable, which the load door's walk asks too.
    if node.has_non_finite_alignment() {
        return Err(EditError::NonFiniteAlignment { node: spoken() });
    }
    // A mate's admission composes its frame offsets, so their literal
    // steps meet the frame rule first, by the predicate the
    // whole-document pass asks of every placement.
    if let (Node::Mate { .. }, Some((at, fault))) = (node, node.placement_frame_fault(tol)) {
        return Err(EditError::placement_frame(spoken(), at, fault));
    }
    check_node_slots(new, doc, id, node)?;
    // The VQ9 authoring-time door (LIB-SWITCH §4d): a profile
    // program resolves + replays + validates under the CURRENT param
    // env, refusing typed here rather than at first evaluation.
    if let Node::Profile(p) = node {
        p.check(&new.var_env::<f64>(), tol).map_err(|refusal| {
            EditError::ProfileProgramRefused {
                node: spoken(),
                refusal: Box::new(refusal),
            }
        })?;
    }
    Ok(())
}

/// **[`DocEdit::SetParam`] at an operand** (DM6): `read` lowered at
/// the slot's kind — a placer's operand at the shape its output was
/// minted with (VR3) — and written over the node's read, under every
/// check the insert door makes of a node ([`write_reads`]).
fn set_operand<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    new: &mut Doc<P>,
    reported: &mut Vec<Maintenance>,
    node: RecipeNodeId,
    slot: crate::OperandSlot,
    read: &crate::Operand,
    tol: Tol,
) -> Result<EditRecord, EditError> {
    let Some(current) = new.nodes.get(&node).cloned() else {
        return Err(EditError::UnknownNode {
            id: SpokenNode::absent(node),
        });
    };
    if !current.operand_rows().iter().any(|(at, _)| *at == slot) {
        return Err(EditError::UnknownSlot {
            id: doc.spoken(node),
            slot: SlotId::Operand(slot),
        });
    }
    let half = current.selected_half();
    let expected = match (&current, new.output(node, 0).and_then(|v| new.var(v))) {
        (Node::Transform { .. }, Some(output)) => crate::SlotKind::Is(output.kind()),
        _ => slot.kind(),
    };
    let spoken = || doc.spoken(node);
    let var = lower_operand(new, &spoken, SlotId::Operand(slot), read, half, expected)?;
    let mut rewritten = current;
    for (at, held) in rewritten.operand_rows_mut() {
        if at == slot {
            *held = var;
        }
    }
    write_reads(doc, new, reported, node, rewritten, tol)
}

/// **The slot door's common half on an operand** (DM6: no edit infers
/// a re-point): `rewritten` is the node `id` with the reads its edit
/// named in full, each already lowered at its seat's kind
/// ([`lower_operand`]). It passes every check the insert door makes of
/// a node ([`check_written_node`]) and acyclicity over reads, and the
/// root list follows the moved reads. The write reports, and never
/// refuses, the payload names it strands: a name whose minting node is
/// no longer upstream of the node that carries it
/// ([`Maintenance::Strand`] with [`Took::Reach`]).
fn write_reads<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    new: &mut Doc<P>,
    reported: &mut Vec<Maintenance>,
    id: RecipeNodeId,
    rewritten: Node<P>,
    tol: Tol,
) -> Result<EditRecord, EditError> {
    check_written_node(doc, new, id, &rewritten, Writes::Reads, tol)?;
    new.nodes.insert(id, rewritten);
    check_acyclic(new)?;
    reported.extend(stranded_by_repoint(doc, new, id));
    Ok(EditRecord {
        minted: None,
        minted_var: None,
        structural: true,
        fresh: Vec::new(),
        outputs: Vec::new(),
    })
}

/// **The names a re-point of `id`'s reads strands**: for `id` and every
/// node downstream of it in `new`, each payload name it carries whose
/// minting node was upstream of it in `before` and is not in `new`; then
/// each name of a selection whose body is `id` or downstream of it,
/// read or not, or which `id` or a node downstream of it reads, that
/// its body cannot reach in `new` — the body's
/// operation or upstream of it — where it could in `before`, or, for a
/// selection the edit authored at `id`, where the read it replaced
/// there could. Payload rows in document order, within a node in
/// [`Node::payload_names`]' order; then selection rows.
fn stranded_by_repoint<P: crate::ProfilePayload>(
    before: &Doc<P>,
    new: &Doc<P>,
    id: RecipeNodeId,
) -> Vec<Maintenance> {
    let reach = |doc: &Doc<P>, var: VarId| -> Option<std::collections::BTreeSet<RecipeNodeId>> {
        let at = doc.read_operation(var)?;
        let mut reach = crate::doc::strict_ancestors(doc, at);
        reach.insert(at);
        Some(reach)
    };
    let mut selections: Vec<VarId> = Vec::new();
    let mut rows = Vec::new();
    for (&carrier, node) in &new.nodes {
        let names = node.payload_names();
        if names.is_empty() {
            continue;
        }
        let after = crate::doc::strict_ancestors(new, carrier);
        if carrier != id && !after.contains(&id) {
            continue;
        }
        let was = crate::doc::strict_ancestors(before, carrier);
        for name in names {
            if was.contains(&name.node) && !after.contains(&name.node) {
                rows.push(Maintenance::Strand {
                    node: before.spoken(carrier),
                    name: before.spoken_name(name),
                    took: Took::Reach,
                });
            }
        }
    }
    for (&var, held) in new.vars() {
        if held.def().select().is_none() {
            continue;
        }
        let Some(at) = new.read_operation(var) else {
            continue;
        };
        if at == id || crate::doc::strict_ancestors(new, at).contains(&id) {
            selections.push(var);
        }
    }
    for (&reader, node) in &new.nodes {
        if reader != id && !crate::doc::strict_ancestors(new, reader).contains(&id) {
            continue;
        }
        for (_, var) in node.operand_rows() {
            if new.selection(var).is_some() && !selections.contains(&var) {
                selections.push(var);
            }
        }
    }
    for var in selections {
        let (Some(select), Some(after)) = (new.selection(var), reach(new, var)) else {
            continue;
        };
        // A selection this edit authored at `id` is held to the read
        // it replaced there: a name that read could reach and this one
        // cannot is one the edit took. A name no read there ever
        // reached is evaluation's to refuse, as at the insert door.
        let replaced = || {
            let slot = new
                .nodes
                .get(&id)?
                .operand_rows()
                .into_iter()
                .find(|(_, v)| *v == var)?
                .0;
            let old = before
                .nodes
                .get(&id)?
                .operand_rows()
                .into_iter()
                .find(|(at, _)| *at == slot)?
                .1;
            reach(before, old)
        };
        let was = match before.selection(var) {
            Some(_) => reach(before, var),
            None => replaced(),
        };
        let Some(was) = was else {
            continue;
        };
        for name in &select.names {
            if was.contains(&name.node) && !after.contains(&name.node) {
                rows.push(Maintenance::StrandedSelection {
                    var: new.spoken_var(var),
                    readers: selection_readers(before, new, var),
                    name: before.spoken_name(name),
                    took: Took::Reach,
                });
            }
        }
    }
    rows
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
        doc.ids().iter().map(|&id| (id, Color::White)).collect();
    for root in doc.ids() {
        if color.get(&root) != Some(&Color::White) {
            continue;
        }
        // Stack of (node, next-input-index) frames.
        let mut stack = vec![(root, 0usize)];
        color.insert(root, Color::Grey);
        while let Some(&mut (id, ref mut next)) = stack.last_mut() {
            let inputs = doc.upstream(id);
            if *next >= inputs.len() {
                color.insert(id, Color::Black);
                stack.pop();
                continue;
            }
            let input = inputs[*next];
            *next += 1;
            match color.get(&input) {
                Some(Color::Grey) => {
                    return Err(EditError::WouldCycle {
                        at: doc.spoken(input),
                    });
                }
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

/// What [`regauge_then_mate`] did: the document the whole action
/// produced, the edits that produce it, the maintenance they
/// performed, and the mate's id. The edits are a record beside a
/// document the door has already applied — the shape of
/// [`crate::InlineOutcome`] — so a caller that keeps edits (a history,
/// the log [`crate::persist::save`] writes) records the action whole,
/// and one that keeps only documents takes up `doc` and `maintenance`
/// together. Undo is the caller keeping the input value: the input is
/// untouched.
#[derive(Debug, Clone, PartialEq)]
pub struct RegaugeThenMateOutcome<P: crate::ProfilePayload> {
    /// The input document with the action applied.
    pub doc: Doc<P>,
    /// The edits that produce `doc` from the input, in the order the
    /// door applied them: a [`DocEdit::SetGauge`] for each member of
    /// `a`'s group not already on `b`'s gauge, then the mate's
    /// [`DocEdit::InsertNode`].
    pub edits: Vec<DocEdit<P>>,
    /// The maintenance `edits` reported, net of what a later edit in
    /// the list took back ([`MaintenanceNet`]), in edit order.
    pub maintenance: Vec<Maintenance>,
    /// The mate's id, minted by its insert.
    pub mate: RecipeNodeId,
}

/// **"Copy `b`'s gauge to `a`, then mate `a` to `b`"** (A11 (2)), as
/// one action: every member of the group the mate's `a` side reads is
/// put on the gauge its `b` side's instance sits on, then the mate is
/// inserted — which then places, and joins the two groups: the first
/// operand's group is placed on the second's ([`DocEdit::InsertNode`]'s
/// mate door clears the offsets `a`'s group held).
///
/// The door applies the whole action and answers the result with its
/// record ([`RegaugeThenMateOutcome`]). The re-gauges are computed
/// against the input document, and each edit is applied to the
/// document the one before it produced; a refusal at any step refuses
/// the whole action, and no outcome comes back.
///
/// The action is the bare insert when the two sides already share a
/// gauge, when a side resolves to no member — the insert door then
/// refuses the mate in its own words — and for a node that is not a
/// mate, which has no gauge to copy.
///
/// # Errors
///
/// [`EditError::WouldStartPlacing`] when the re-gauge would put the
/// two instances of a mate already in the document on one gauge: that
/// mate declares today, and would start placing — moving a group the
/// action never named. Otherwise [`apply`]'s, from the step that
/// refuses.
pub fn regauge_then_mate<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    mate: Node<P::Authored, Formula>,
    tol: Tol,
    reach: &dyn MateReach,
) -> Result<RegaugeThenMateOutcome<P>, EditError> {
    let mut action = Recording::start(doc, tol, reach);
    for regauge in regauges_for(doc, &mate)? {
        action.apply(regauge)?;
    }
    let mate = action.insert(mate)?;
    let Recorded {
        doc,
        edits,
        maintenance,
        ..
    } = action.finish()?;
    Ok(RegaugeThenMateOutcome {
        doc,
        edits,
        maintenance,
        mate,
    })
}

/// **What [`Recording::measure`] inserted**: the measures, one per
/// primitive in order, and each one's output in the same order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Measured {
    /// The `Measure` nodes, one per primitive.
    pub measures: Vec<RecipeNodeId>,
    /// Each measure's output, the observed scalar it defines.
    pub outputs: Vec<VarId>,
}

/// [`fn@measure`]'s outcome: the action applied, and what it inserted.
#[derive(Debug)]
pub struct MeasureOutcome<P: crate::ProfilePayload> {
    /// The document the action produced.
    pub doc: Doc<P>,
    /// The edits that produce `doc` from the start, one insert per
    /// measure.
    pub edits: Vec<DocEdit<P>>,
    /// The maintenance they reported.
    pub maintenance: Vec<Maintenance>,
    /// The measures and their outputs.
    pub measured: Measured,
}

/// **Measures, as one action** (E3, D10): one [`Node::Measure`] per
/// primitive, each defining one observed scalar ([`Recording::measure`]).
/// The edit list is the split's shape: the edits, and the document they
/// produce.
///
/// # Errors
///
/// The insert door's refusals; the document is untouched.
pub fn measure<P: Clone + crate::ProfilePayload, R: Clone + Into<crate::Operand>>(
    doc: &Doc<P>,
    primitives: &[crate::measure::MeasurePrimitive<R>],
    tol: Tol,
    reach: &dyn MateReach,
) -> Result<MeasureOutcome<P>, EditError> {
    let mut action = Recording::start(doc, tol, reach);
    let measured = action.measure(primitives)?;
    let Recorded {
        doc,
        edits,
        maintenance,
        ..
    } = action.finish()?;
    Ok(MeasureOutcome {
        doc,
        edits,
        maintenance,
        measured,
    })
}

/// The `SetGauge` edits [`regauge_then_mate`] applies before its
/// insert, in order, computed against `doc`.
fn regauges_for<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    mate: &Node<P::Authored, Formula>,
) -> Result<Vec<DocEdit<P>>, EditError> {
    let mut edits = Vec::new();
    if let Node::Mate { a, b, .. } = mate
        && let (Some(ma), Some(mb)) = (
            crate::mate::member_of(doc, a),
            crate::mate::member_of(doc, b),
        )
    {
        let gauge = doc.node(mb.instance).and_then(Node::gauge_ref);
        let groups = crate::mate::groups(doc);
        let moved: &[RecipeNodeId] = groups
            .iter()
            .find(|g| g.contains(&ma.instance))
            .map_or(&[], Vec::as_slice);
        let gauge_after = |instance: RecipeNodeId| {
            if moved.contains(&instance) {
                gauge
            } else {
                doc.node(instance).and_then(Node::gauge_ref)
            }
        };
        if let Some(id) = mate_that_would_start_placing(doc, gauge_after) {
            return Err(EditError::WouldStartPlacing {
                mate: doc.spoken(id),
            });
        }
        for &member in moved {
            if doc.node(member).and_then(Node::gauge_ref) != gauge {
                edits.push(DocEdit::SetGauge {
                    node: member,
                    gauge,
                });
            }
        }
    }
    Ok(edits)
}

/// The nodes a cascading delete of `id` must remove — the GUI's
/// "delete with everything downstream" — ordered consumers first, `id`
/// last.
///
/// The set is `id` plus everything that reads it, directly or through
/// other readers: the closure of [`Doc::upstream`] in the consumer
/// direction, so no node it leaves standing reads one it removes. A
/// [`DocEdit::DeleteNode`] of a read node is accepted on its own and
/// leaves its readers unresolved (D10); this is the convenience that
/// removes them with it. The order is the reverse of the evaluation
/// schedule's, so each delete in turn removes a node nothing left
/// standing reads.
///
/// The answer is empty for an id the document does not hold.
pub fn cascade_delete_order<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    id: RecipeNodeId,
) -> Vec<RecipeNodeId> {
    use std::collections::BTreeSet;
    if doc.node(id).is_none() {
        return Vec::new();
    }
    let order = crate::eval::schedule::schedule(doc).order;
    let mut doomed: BTreeSet<RecipeNodeId> = BTreeSet::from([id]);
    for &n in &order {
        if doc.upstream(n).iter().any(|input| doomed.contains(input)) {
            doomed.insert(n);
        }
    }
    order
        .into_iter()
        .rev()
        .filter(|n| doomed.contains(n))
        .collect()
}

/// Apply one edit to a document — pure over the document and the
/// reach (spec D2): the input is untouched, and the output is a
/// function of `doc`, `edit` and what `reach` answers; on acceptance
/// a new value comes back with the [`EditRecord`] and the maintenance
/// reports ([`Applied::maintenance`]). All validation is here — refs
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
    apply_with(doc, edit, tol, Some(reach))
}

/// **Replay one edit**: the edit through every door [`apply`] has,
/// with no reach — no solve, no store. No edit records a frame and no
/// maintenance needs one (A11 (2)), so the edits ARE the log, and the
/// one decision `apply` levers through a reach — the clocking rider of
/// an inserted mate — is declined here, as
/// [`crate::mate::solve::admit_mate`] states.
///
/// # Errors
///
/// [`apply`]'s.
pub fn apply_replayed<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    edit: &DocEdit<P>,
    tol: Tol,
) -> Result<Applied<P>, EditError> {
    apply_with(doc, edit, tol, None)
}

/// [`apply`] with the reach the door holds: the live door's, or none
/// on replay ([`apply_replayed`]).
fn apply_with<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    edit: &DocEdit<P>,
    tol: Tol,
    reach: Option<&dyn MateReach>,
) -> Result<Applied<P>, EditError> {
    let (doc, record, maintenance) = door(doc, tol, |new, reported| {
        write_edit(doc, new, reported, edit, tol, reach)
    })?;
    Ok(Applied {
        doc,
        record,
        maintenance,
    })
}

/// [`apply`] of the [`DocEdit::InsertNode`] of `node`, answering the
/// id it minted beside the [`Applied`]: the same door, with the id
/// in the type rather than in [`EditRecord::minted`]'s `Option`.
pub(crate) fn apply_insert<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    node: &Node<P::Authored, Formula>,
    tol: Tol,
    reach: &dyn MateReach,
) -> Result<(Applied<P>, RecipeNodeId), EditError> {
    let (doc, minted, maintenance) = door(doc, tol, |new, reported| {
        insert_into(doc, new, reported, node, &[], tol, Some(reach))
    })?;
    let id = minted.0;
    let applied = Applied {
        doc,
        record: inserted(minted),
        maintenance,
    };
    Ok((applied, id))
}

/// **Every edit door's frame**: `write` the edit into a copy of `doc`,
/// then hold the result to the backstops every edit answers to. The
/// input is untouched (spec D2).
fn door<P: Clone + crate::ProfilePayload, T>(
    doc: &Doc<P>,
    tol: Tol,
    write: impl FnOnce(&mut Doc<P>, &mut Vec<Maintenance>) -> Result<T, EditError>,
) -> Result<(Doc<P>, T, Vec<Maintenance>), EditError> {
    let mut new = doc.clone();
    // The report read at the door that made it: DM7's strands a delete
    // made, or the strands a reshaped program made. `DeleteNode` fills it, the only edit that removes
    // a node, and `SetProgram`, the only edit that drops a profile
    // step. `Rebind` moves references onto a live name at the author's
    // word and reports nothing.
    let mut reported: Vec<Maintenance> = Vec::new();
    let wrote = write(&mut new, &mut reported)?;
    // VR7, on EVERY arm: an anonymous variable goes with its reader, so
    // the edit that detached it removes it. The mint log keeps its id.
    for var in new.unread_anonymous_vars() {
        new.vars.remove(&var);
        reported.push(Maintenance::AnonymousVarRemoved {
            var: doc.spoken_var(var),
            distribution: doc.free(var).and_then(FreeVar::distribution).copied(),
        });
    }
    // VR2, on EVERY arm: an anonymous variable has one reader, so the
    // edit that gave it a second, or cleared the name two share, refuses
    // until it is named.
    if let Some(&var) = new.shared_unnamed_vars().first() {
        // Spoken as the document before the edit held it: a cleared
        // name is the one to give back.
        let held = if doc.var(var).is_some() { doc } else { &new };
        return Err(EditError::SharedVarNeedsName {
            var: held.spoken_var(var),
        });
    }
    // D10, on EVERY arm: a construction reads what was written. A slot
    // write, a read's repoint and a redefinition can each make a
    // construction's slot read an observed variable, and this is the
    // one place that refuses it.
    if let Some((node, slot, var)) = new.observed_read_fault() {
        return Err(EditError::ConstructionReadsObserved {
            node: spoken_before_else_after(doc, &new, node),
            slot,
            var: Box::new(new.spoken_var(var)),
        });
    }
    // The placement-rule backstop, on EVERY arm (GROUP-BOOLEAN-DESIGN):
    // "how many placements" has exactly ONE spelling, an explicit rule
    // lists at least one placement, and its frames meet the SAME bar
    // every placement's literal steps are held to
    // (`Frame::admission_fault`).
    // A rule's shape and its listed frames are written only by the
    // insert that authors its node, but a listed frame is admitted at
    // the `tol` this edit is applied at, which need not be the one its
    // insert was; so the whole document is checked, in id order, and
    // where one edit breaks two nodes the refusal names the one placed
    // first.
    for (&node, n) in &new.nodes {
        let listed = |index| FrameSite::Listed { index };
        match n.placement_rule_fault(tol) {
            None => {}
            Some(PlacementRuleFault::CountSpelling { shape }) => {
                return Err(EditError::PlacementRuleMismatch {
                    node: written(doc, node, n),
                    shape,
                });
            }
            Some(PlacementRuleFault::NoPlacements) => {
                return Err(EditError::EmptyPlacementList {
                    node: written(doc, node, n),
                });
            }
            Some(PlacementRuleFault::NonFiniteFrame { index }) => {
                return Err(EditError::NonFinitePlacement {
                    node: written(doc, node, n),
                    at: listed(index),
                });
            }
            Some(PlacementRuleFault::ImproperFrame { index, determinant }) => {
                return Err(EditError::ImproperPlacement {
                    node: written(doc, node, n),
                    at: listed(index),
                    determinant,
                });
            }
            Some(PlacementRuleFault::NonRigidFrame { index, check }) => {
                return Err(EditError::NonRigidPlacement {
                    node: written(doc, node, n),
                    at: listed(index),
                    check,
                });
            }
        }
        // A placement's literal steps — a transform's, a gauge's, an
        // instance's offset, a mate's frame offsets — meet the same
        // bar, by the same predicate.
        if let Some((at, fault)) = n.placement_frame_fault(tol) {
            return Err(EditError::placement_frame(written(doc, node, n), at, fault));
        }
    }
    Ok((new, wrote, reported))
}

/// The record of an accepted insert that minted `id`.
fn inserted((id, fresh, outputs): (RecipeNodeId, Vec<VarId>, Vec<VarId>)) -> EditRecord {
    EditRecord {
        minted: Some(id),
        minted_var: None,
        structural: true,
        fresh,
        outputs,
    }
}

/// [`DocEdit::InsertNode`] of `node`, written into `new` (a copy of
/// `doc`): the id it mints, or the door's refusal.
fn insert_into<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    new: &mut Doc<P>,
    reported: &mut Vec<Maintenance>,
    authored: &Node<P::Authored, Formula>,
    fresh: &[FreshEntry],
    tol: Tol,
    reach: Option<&dyn MateReach>,
) -> Result<(RecipeNodeId, Vec<VarId>, Vec<VarId>), EditError> {
    // A refusal speaks the node by the id drawn from it with every name
    // the document holds lowered, so a node authored by name and the
    // same node authored by id speak one id even where some other name
    // does not lower.
    let before_lowering = new.mint.clone();
    let would = || {
        let mut held = authored.clone();
        for formula in held.exprs_mut() {
            *formula = formula.lower_held(&|name| doc.lowering_scope(name));
        }
        let would = before_lowering.clone().insert(&held);
        SpokenNode::entering(would, &held)
    };
    // The node as the door writes it, lowered before anything else is
    // asked of it (VR4): the fresh table minted, then every slot the
    // variable it stores. Its id is minted from the lowered node, so a
    // node authored by name and the same node authored by id mint one
    // id.
    let lowering = Lowering::start(new, fresh)?;
    let node = &lower_node(new, &lowering, authored, tol, would)?;
    // D6's slot rule, of the formulas as written, asked after the names
    // as the door has always asked it: each carries the
    // dimension its address fixes (`Node::formula_dimension_fault`).
    if let Some(SlotDimensionFault {
        slot,
        expected,
        found,
    }) = authored.formula_dimension_fault()
    {
        return Err(EditError::SlotDimensionMismatch {
            slot,
            expected,
            found,
        });
    }
    // N1: the node's id is minted from the document's mint
    // chain, extended by the node as the edit states it, and
    // then every authored step's, from the same chain. Minting
    // extends a mint held aside, so a refusal further down
    // leaves the document's untouched.
    let mut mint = new.mint.clone();
    let id = mint.insert(node);
    check_written_node(doc, new, id, node, Writes::Node, tol)?;
    let entering = SpokenNode::entering(id, node);
    let mut node = node.clone();
    if let Node::Profile(p) = &mut node {
        p.mint_step_ids(&mut mint)
            .map_err(|fault| EditError::StepIdsRefused {
                node: entering,
                fault,
            })?;
    }
    // D10: the node defines one variable per port of its signature,
    // minted after its steps. Its inputs are live (above), so every
    // port's kind resolves.
    let Some(signature) = new.signature_of(&node) else {
        unreachable!("a node whose inputs are live has a signature")
    };
    let ports = u8::try_from(signature.len())
        .unwrap_or_else(|_| unreachable!("a signature is a handful of ports"));
    let outputs = mint.outputs_of_insert(ports);
    new.mint = mint;
    for ((port, (_, kind)), &var) in (0u8..).zip(&signature).zip(&outputs) {
        new.vars.insert(var, Var::output(*kind, id, port));
    }
    new.nodes.insert(id, node.clone());
    let fresh = lowering.finish(new)?;
    check_acyclic(new)?;
    // The solve's own per-mate admission (A11 rule 1), asked
    // of the document the mate now stands in — its walks read
    // the operands there — through the reach this door holds:
    // a mate the coset table refuses on its own datum is
    // refused at this door. The verdicts about a PAIR stay the
    // solve's (`admit_mate`), and so does every STATE a mate
    // comes to hold after insert: a reference a later edit
    // strands is N5's. The other edit that writes a datum — a
    // slot edit at a frame-offset step — asks the same
    // admission (`DocEdit::writes_a_mates_datum`).
    if matches!(node, Node::Mate { .. }) {
        admit_written_mate(new, id, tol, reach)?;
        reported.extend(clear_joined_offsets(doc, new, &node));
    }
    Ok((id, fresh, outputs))
}

/// **The solve's per-mate admission of a mate an edit just wrote**
/// (A11 rule 1), asked of the document the mate now stands in, at
/// that document's own nominal environment, through the reach the
/// door holds — the one call every edit that writes a mate's datum
/// ([`DocEdit::writes_a_mates_datum`]) makes.
fn admit_written_mate<P: Clone + crate::ProfilePayload>(
    new: &Doc<P>,
    id: RecipeNodeId,
    tol: Tol,
    reach: Option<&dyn MateReach>,
) -> Result<(), EditError> {
    let Some(node) = new.nodes.get(&id) else {
        unreachable!("the edit wrote {} into the document", new.spoken(id))
    };
    let env = new.var_env::<f64>();
    crate::mate::solve::admit_mate(new, id, node, &env, reach, tol).map_err(|fault| {
        EditError::MateRefused {
            node: new.spoken(id),
            held: crate::spoken::held_by(&*fault, new),
            fault,
        }
    })
}

/// One edit written into `new` (a copy of `doc`), its maintenance
/// into `reported`: the record, or the door's refusal.
#[allow(clippy::too_many_lines)] // one arm per DocEdit variant, each short
fn write_edit<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    new: &mut Doc<P>,
    reported: &mut Vec<Maintenance>,
    edit: &DocEdit<P>,
    tol: Tol,
    reach: Option<&dyn MateReach>,
) -> Result<EditRecord, EditError> {
    Ok(match edit {
        DocEdit::InsertNode { node, fresh } => {
            debug_assert!(edit.writes_a_mates_datum() == matches!(**node, Node::Mate { .. }));
            inserted(insert_into(doc, new, reported, node, fresh, tol, reach)?)
        }
        DocEdit::DeleteNode { id } => {
            if !new.nodes.contains_key(id) {
                return Err(EditError::UnknownNode {
                    id: SpokenNode::absent(*id),
                });
            }
            *reported = remove_node(doc, new, *id);
            // A gauge's references are kept, dangling (A11 (2)): the
            // group it unplaced names it as the cause, and a
            // `SetGauge` re-places it.
            EditRecord {
                minted: None,
                minted_var: None,
                structural: true,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::SetMembers { node, members } => {
            let Some(current) = new.nodes.get(node).cloned() else {
                return Err(EditError::UnknownNode {
                    id: SpokenNode::absent(*node),
                });
            };
            let at = match current {
                Node::Union { .. } => crate::OperandSlot::Member,
                Node::Loft { .. } => crate::OperandSlot::Section,
                _ => {
                    return Err(EditError::SetMembersOnNonList {
                        node: doc.spoken(*node),
                    });
                }
            };
            let spoken = || doc.spoken(*node);
            let mut list = Vec::with_capacity(members.len());
            for (i, member) in members.iter().enumerate() {
                let slot = at(u32::try_from(i).unwrap_or(u32::MAX));
                list.push(lower_operand(
                    new,
                    &spoken,
                    SlotId::Operand(slot),
                    member,
                    None,
                    slot.kind(),
                )?);
            }
            let mut rewritten = current.clone();
            if !rewritten.set_list_input(list) {
                unreachable!("a union and a loft hold a list")
            }
            write_reads(doc, new, reported, *node, rewritten, tol)?
        }
        DocEdit::SetDeclare { node, pairs } => {
            let mut rewritten = match new.nodes.get(node) {
                None => {
                    return Err(EditError::UnknownNode {
                        id: SpokenNode::absent(*node),
                    });
                }
                Some(current) => current.clone(),
            };
            let (Node::Boolean { declare, .. } | Node::Union { declare, .. }) = &mut rewritten
            else {
                return Err(EditError::SetDeclareOnNonDeclaring {
                    node: doc.spoken(*node),
                });
            };
            declare.clone_from(pairs);
            // The pairs are payload names and read sites, checked by the
            // insert door's own functions; no edge moved, so neither
            // acyclicity nor the root set is asked again.
            check_payload_refs(doc, new, &rewritten)?;
            check_declared_sides(
                doc,
                new,
                &rewritten,
                crate::node::declared_sides(pairs),
                || doc.spoken(*node),
                Some(*node),
            )?;
            new.nodes.insert(*node, rewritten);
            EditRecord {
                minted: None,
                minted_var: None,
                structural: true,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::SetProgram {
            node,
            loops,
            ids,
            fresh,
        } => {
            let payload = match new.nodes.get(node) {
                None => {
                    return Err(EditError::UnknownNode {
                        id: SpokenNode::absent(*node),
                    });
                }
                Some(Node::Profile(p)) => p.clone(),
                Some(_) => {
                    return Err(EditError::SetProgramOnNonProfile {
                        node: doc.spoken(*node),
                    });
                }
            };
            let payload = &payload;
            let Some(old_ids) = payload.step_ids() else {
                return Err(EditError::SetProgramOnNonProfile {
                    node: doc.spoken(*node),
                });
            };
            // Every argument as written, in program order, at its
            // address: its lowering and its reads, then its dimension
            // (D6) — the first argument that does not hold refuses.
            let rows: Vec<(SlotId, &Formula)> = loops
                .iter()
                .enumerate()
                .flat_map(|(li, lp)| {
                    lp.rows().into_iter().map(move |((step, arg), formula)| {
                        let loop_ = crate::program::program_index(li);
                        (SlotId::Profile { loop_, step, arg }, formula)
                    })
                })
                .collect();
            let lowering = Lowering::start(new, fresh)?;
            let loops = lower_loops(new, &lowering, doc.spoken(*node), loops)?;
            if let Some(SlotDimensionFault {
                slot,
                expected,
                found,
            }) = rows
                .iter()
                .find_map(|&(slot, formula)| slot.dimension_fault(formula))
            {
                return Err(EditError::SlotDimensionMismatch {
                    slot,
                    expected,
                    found,
                });
            }
            // The ids FIRST: ids the door could not honour make the
            // replay below moot, and the caller mends the field named
            // rather than the program. Minting extends a mint held
            // aside, so a refusal further down leaves the document's
            // untouched.
            let mut mint = new.mint.clone();
            let (minted, dropped) =
                settle_step_ids(&doc.spoken(*node), old_ids, &loops, ids, &mut mint)?;
            let Some(rewritten) = payload.with_program(loops, minted) else {
                return Err(EditError::SetProgramOnNonProfile {
                    node: doc.spoken(*node),
                });
            };
            // The rewritten node walks the insert door's own slot
            // checks — the same function, not a mirror: dimensions and
            // parameter references over every argument of every loop.
            let probe = Node::Profile(rewritten);
            check_node_slots(new, doc, *node, &probe)?;
            let Node::Profile(rewritten) = &probe else {
                unreachable!("the probe was built as a profile node two lines above")
            };
            // Then the VQ9 door the insert door runs.
            rewritten
                .check(&new.var_env::<f64>(), tol)
                .map_err(|refusal| EditError::ProfileProgramRefused {
                    node: doc.spoken(*node),
                    refusal: Box::new(refusal),
                })?;
            let undrawn = undrawn_kept_pieces(doc, new, *node, payload, rewritten, &dropped, tol)?;
            new.nodes.insert(*node, probe);
            new.mint = mint;
            let fresh = lowering.finish(new)?;
            *reported = stranded_steps(doc, new, &dropped, &undrawn);
            // Structural whatever moved: the edit's class is a
            // rewrite of program structure — verbs, order, count —
            // and the record classifies the edit, as
            // `SetStructuralParam` is structural whether or not the
            // count it writes differs. An identity program is this
            // edit with nothing to do, not a different edit.
            EditRecord {
                minted: None,
                minted_var: None,
                structural: true,
                fresh,
                outputs: Vec::new(),
            }
        }
        DocEdit::SetParam {
            node,
            slot,
            value,
            fresh,
        } => {
            if slot.is_structural() {
                return Err(EditError::StructuralSlotNeedsStructuralEdit { slot: *slot });
            }
            let formula = match (slot, value) {
                (SlotId::Operand(operand), SlotValue::Read(read)) => {
                    if !fresh.is_empty() {
                        return Err(EditError::FreshUnread { index: 0 });
                    }
                    return set_operand(doc, new, reported, *node, *operand, read, tol);
                }
                (_, SlotValue::Formula(formula)) => std::borrow::Cow::Borrowed(formula),
                // A read at a scalar slot is the formula of the one
                // variable it resolves to, held to every rule a formula
                // is.
                (_, SlotValue::Read(read)) => {
                    let spoken = || doc.spoken(*node);
                    let var = lower_operand(new, &spoken, *slot, read, None, slot.kind())?;
                    std::borrow::Cow::Owned(Formula::var(
                        var,
                        slot.dimension()
                            .unwrap_or_else(|| unreachable!("an operand slot is answered above")),
                    ))
                }
            };
            let fresh = set_slot(new, doc, *node, *slot, &formula, fresh)?;
            check_profile_after_slot_edit(new, doc, *node, *slot, tol)?;
            if edit.writes_a_mates_datum() {
                admit_written_mate(new, *node, tol, reach)?;
            }
            EditRecord {
                minted: None,
                minted_var: None,
                structural: false,
                fresh,
                outputs: Vec::new(),
            }
        }
        DocEdit::SetStructuralParam {
            node,
            slot,
            expr,
            fresh,
        } => {
            if !slot.is_structural() {
                return Err(EditError::NotStructuralSlot { slot: *slot });
            }
            let fresh = set_slot(new, doc, *node, *slot, expr, fresh)?;
            EditRecord {
                minted: None,
                minted_var: None,
                structural: true,
                fresh,
                outputs: Vec::new(),
            }
        }
        DocEdit::SetExtrudeSide { node, side } => {
            let Some(target) = new.nodes.get_mut(node) else {
                return Err(EditError::UnknownNode {
                    id: SpokenNode::absent(*node),
                });
            };
            let Node::Extrude { side: held, .. } = target else {
                return Err(EditError::SetExtrudeSideOnNonExtrude {
                    node: doc.spoken(*node),
                });
            };
            *held = *side;
            // Structural whether or not the side moved, as
            // `SetStructuralParam` is whatever count it writes.
            EditRecord {
                minted: None,
                minted_var: None,
                structural: true,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::SetExpression { path, expr } => {
            if !new.nodes.contains_key(&path.node) {
                return Err(EditError::UnknownNode {
                    id: SpokenNode::absent(path.node),
                });
            }
            // The path addresses the slot as written
            // (`Doc::slot_expansion`). The slot is expanded only along
            // it (`Doc::expansion_along`), the subtree it names replaced,
            // and the slot re-lowered from the whole, so every variable
            // the rest of it reads — an anonymous one included — is read
            // again by id.
            let Some(var) = new.slot(path.node, path.slot) else {
                return Err(EditError::UnknownSlot {
                    id: doc.spoken(path.node),
                    slot: path.slot,
                });
            };
            let dim = new
                .vars
                .get(&var)
                .and_then(|v| v.kind().dimension())
                .unwrap_or_else(|| path.slot.expr_dimension());
            let off_tree = || EditError::PathOffTree {
                node: doc.spoken(path.node),
                slot: path.slot,
                path: path.path.clone(),
            };
            let root = new
                .expansion_along(&Expr::var(var, dim), &path.path)
                .ok_or_else(off_tree)?;
            let rebuilt = Formula::from(&root)
                .with_replaced(&path.path, expr.clone())
                .ok_or_else(off_tree)?
                .map_err(EditError::Dimension)?;
            let structural = path.slot.is_structural();
            set_slot(new, doc, path.node, path.slot, &rebuilt, &[])?;
            check_profile_after_slot_edit(new, doc, path.node, path.slot, tol)?;
            if edit.writes_a_mates_datum() {
                admit_written_mate(new, path.node, tol, reach)?;
            }
            EditRecord {
                minted: None,
                minted_var: None,
                structural,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::DeclareVar { name, def } => {
            if let Some(holder) = doc.var_named(name.as_str()) {
                return Err(EditError::VarNameTaken {
                    name: name.clone(),
                    holder: doc.spoken_var(holder),
                });
            }
            // A free definition is checked BEFORE anything is minted, and
            // a refused declare speaks the id it would have minted. What
            // a defined one reads is asked after, with the rest of its
            // checks.
            let spoken = SpokenVar::new(new.mint.would_declare(def.kind()), Some(name.clone()));
            if let VarDecl::Free(free) = def {
                check_var_def(&spoken, &WrittenDef::Free(free.clone()))?;
            }
            let id = new.mint.declare(def.kind());
            // The declared variable mints first, its definition's
            // written quantities after it (`mint_quantities` says why
            // the slot door's order is the other way round).
            let def = lower_decl(new, &Lowering::none(), &spoken, def)?;
            new.vars.insert(id, Var::written(def.clone()));
            new.var_names.insert(id, name.clone());
            check_definition(new, id)?;
            EditRecord {
                minted: None,
                minted_var: Some(id),
                structural: def.kind() == VarKind::Count,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::DefineVar { var, def, fresh } => {
            let Some(id) = doc.resolve_var(var) else {
                return Err(EditError::UnknownVar {
                    var: var.clone(),
                    door: CarryForwardDoor::Definition,
                });
            };
            let spoken = doc.spoken_var(id);
            refuse_output(doc, id, CarryForwardDoor::Definition)?;
            // The kind first: a definition of another kind is refused
            // whatever it reads, and what it reads is asked after.
            let kind = doc.var(id).map_or(def.kind(), Var::kind);
            if def.kind() != kind {
                return Err(EditError::VarKindFixed {
                    var: spoken,
                    kind,
                    offered: def.kind(),
                });
            }
            let lowering = Lowering::start(new, fresh)?;
            let def = lower_decl(new, &lowering, &spoken, def)?;
            let record = match def {
                WrittenDef::Free(value) => write_free(new, id, &spoken, value)?,
                WrittenDef::Defined(_) => {
                    check_var_def(&spoken, &def)?;
                    new.vars.insert(id, Var::written(def));
                    check_definition(new, id)?;
                    EditRecord {
                        minted: None,
                        minted_var: None,
                        structural: kind == VarKind::Count,
                        fresh: Vec::new(),
                        outputs: Vec::new(),
                    }
                }
                WrittenDef::Select(..) => unreachable!("a declaration lowers to no selection"),
            };
            EditRecord {
                fresh: lowering.finish(new)?,
                ..record
            }
        }
        DocEdit::SetVarValue { var, value } => {
            let (id, spoken, declared) = standing_var(doc, var, CarryForwardDoor::Value)?;
            // THE carry-forward: the definition is read off the
            // document and reused whole, so the kind and the
            // distribution cannot be dropped by an omission here.
            let Some(written) = declared.with_value(*value) else {
                return Err(EditError::VarValueKindMismatch {
                    var: spoken,
                    declared: declared.dim(),
                    offered: *value,
                });
            };
            write_free(new, id, &spoken, written)?
        }
        DocEdit::SetVarUnit { var, unit } => {
            let (id, spoken, declared) = standing_var(doc, var, CarryForwardDoor::Notation)?;
            // THE carry-forward, over the notation. Both reasons it
            // can refuse are the DOOR's — this routes them, and decides
            // neither.
            let written = declared.with_display_unit(*unit).map_err(|why| match why {
                DisplayUnitRefusal::CountHasNoNotation => EditError::VarCountHasNoUnit {
                    var: spoken.clone(),
                },
                DisplayUnitRefusal::Mismatch { unit, declared } => EditError::VarUnitMismatch {
                    var: spoken.clone(),
                    unit,
                    declared,
                },
            })?;
            write_free(new, id, &spoken, written)?
        }
        DocEdit::SetVarDistribution { var, distribution } => {
            let (id, spoken, declared) = standing_var(doc, var, CarryForwardDoor::Annotation)?;
            // THE carry-forward, over the annotation.
            let written = declared
                .with_distribution(*distribution)
                .map_err(|why| match why {
                    DistributionRefusal::CountHasNoAnnotation => {
                        EditError::VarCountHasNoDistribution {
                            var: spoken.clone(),
                        }
                    }
                    DistributionRefusal::Invalid { fault } => {
                        distribution_fault_error(&spoken, fault)
                    }
                    DistributionRefusal::NotAWrittenValue => {
                        unreachable!("a free variable's own distribution is a written value's")
                    }
                })?;
            write_free(new, id, &spoken, written)?
        }
        DocEdit::RenameVar { var, name } => {
            let Some(id) = doc.resolve_var(var) else {
                return Err(EditError::UnknownVar {
                    var: var.clone(),
                    door: CarryForwardDoor::Rename,
                });
            };
            if doc.var_name(id) == name.as_ref() {
                return Err(EditError::VarNameUnchanged {
                    var: doc.spoken_var(id),
                });
            }
            match name {
                Some(name) => {
                    if let Some(holder) = doc.var_named(name.as_str()) {
                        return Err(EditError::VarNameTaken {
                            name: name.clone(),
                            holder: doc.spoken_var(holder),
                        });
                    }
                    new.var_names.insert(id, name.clone());
                }
                None => {
                    new.var_names.remove(&id);
                    if new.unread_anonymous_vars().contains(&id) {
                        return Err(EditError::AnonymousVarUnread {
                            var: doc.spoken_var(id),
                        });
                    }
                }
            }
            // A reader holds the id: nothing evaluated moves.
            EditRecord {
                minted: None,
                minted_var: None,
                structural: false,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::DeleteVar { var } => {
            let Some(id) = doc.resolve_var(var) else {
                return Err(EditError::UnknownVar {
                    var: var.clone(),
                    door: CarryForwardDoor::Delete,
                });
            };
            refuse_output(doc, id, CarryForwardDoor::Delete)?;
            if doc.var_name(id).is_none() {
                return Err(EditError::DeleteAnonymousVar {
                    var: doc.spoken_var(id),
                });
            }
            new.vars.remove(&id);
            new.var_names.remove(&id);
            // The readers stay, unresolved: each now refuses at
            // evaluation, which is structural; a variable nothing read
            // moves nothing evaluated.
            EditRecord {
                minted: None,
                minted_var: None,
                structural: !doc.var_readers(id).is_empty(),
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::Rebind { body, from, to } => {
            if from == to {
                return Err(EditError::RebindIdentity {
                    name: doc.spoken_name(from),
                });
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
                return Err(EditError::RebindTargetMissingNode {
                    name: doc.spoken_name(to),
                });
            }
            check_name_steps(doc, new, to)?;
            // The source must have ONCE existed (ids are monotone and
            // never reused): dead-but-once-lived is exactly the
            // NodeGone repair; never-minted is a typo.
            if !new.has_minted(from.node) {
                return Err(EditError::RebindUnknownName {
                    name: doc.spoken_name(from),
                });
            }
            // A body's selections (`Doc::rewrite_selection_names`), or
            // else every payload site and the appearance store
            // (`Node::payload_names` lists the payload sites and
            // `Node::rebind_payload_names` is its rewriting twin, so no
            // payload carrier can be repaired here and missed there).
            // Zero sites = nothing to repair, refused.
            let mut declare_sites = 0usize;
            let mut appearance_sites = 0usize;
            if let Some(body) = body {
                declare_sites += new
                    .rewrite_selection_names(*body, &mut |name| (name == from).then(|| to.clone()));
            } else {
                let mut redeclared = Vec::new();
                for (&id, node) in &mut new.nodes {
                    let before = node.declared_pairs().to_vec();
                    declare_sites += node.rebind_payload_names(from, to);
                    if node.declared_pairs() != before.as_slice() {
                        redeclared.push((id, before));
                    }
                }
                // A rewritten declared side is one this door writes, so
                // it answers the rule every such door asks — of the
                // sides the rebind moved only, so a strand a union
                // already held, even the other side of a pair it
                // rewrites, does not block an unrelated repair.
                for (id, before) in redeclared {
                    let Some(node) = new.nodes.get(&id) else {
                        continue;
                    };
                    let moved = crate::node::declared_sides(node.declared_pairs())
                        .zip(crate::node::declared_sides(&before))
                        .filter(|(after, was)| after != was)
                        .map(|(after, _)| after);
                    check_declared_sides(doc, new, node, moved, || doc.spoken(id), Some(id))?;
                }
                // Appearance keys are rebind sites (the attribute rides
                // the name). A per-kind collision with an attribute
                // already on `to` is refused loudly: which value
                // survives would be an auto-pick —
                // `move_appearance_record` is the one home for that rule.
                if let Some(moved) = new.appearance.remove(from) {
                    appearance_sites += 1;
                    move_appearance_record(doc, &mut new.appearance, moved, to)?;
                }
            }
            if declare_sites + appearance_sites == 0 {
                return Err(EditError::RebindNoReferences {
                    name: doc.spoken_name(from),
                });
            }
            EditRecord {
                minted: None,
                minted_var: None,
                // A payload name changed: content keys move and the
                // threading consumes them — structural, whichever
                // carrier held it. An appearance-only rebind is
                // presentation motion: no content key moves, nothing
                // recomputes.
                structural: declare_sites > 0,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::ReWitness { node, witness } => {
            check_witness_site(doc, *node)?;
            new.witnesses.insert(*node, witness.clone());
            EditRecord {
                minted: None,
                minted_var: None,
                structural: false,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::SetAppearance { name, attr } => {
            // v1 scope: faces and bodies (M4-PLAN item 7); edges/
            // vertices stay a typed refusal until ratified.
            if !matches!(name.kind, EntityKind::Face | EntityKind::Body) {
                return Err(EditError::AppearanceWrongKind {
                    name: doc.spoken_name(name),
                });
            }
            // Node existence NOW, name-level resolution at evaluation
            // (the ruled Declare carve-out, applied to the second
            // name-referencing edit).
            if !new.nodes.contains_key(&name.node) {
                return Err(EditError::AppearanceNamesMissingNode {
                    name: doc.spoken_name(name),
                });
            }
            check_name_steps(doc, new, name)?;
            new.appearance
                .entry(name.clone())
                .or_default()
                .attrs
                .insert(attr.kind(), attr.clone());
            // Presentation only: never structural, never a recompute.
            EditRecord {
                minted: None,
                minted_var: None,
                structural: false,
                fresh: Vec::new(),
                outputs: Vec::new(),
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
                check_witness_site(doc, *node)?;
                if !seen.insert(*node) {
                    return Err(EditError::DuplicateWitnessEntry {
                        node: doc.spoken(*node),
                    });
                }
            }
            for (node, witness) in entries {
                new.witnesses.insert(*node, witness.clone());
            }
            EditRecord {
                minted: None,
                minted_var: None,
                structural: false,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::ClearAppearance { name, kind } => {
            let not_set = || EditError::AppearanceNotSet {
                name: doc.spoken_name(name),
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
                minted_var: None,
                structural: false,
                fresh: Vec::new(),
                outputs: Vec::new(),
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
                minted_var: None,
                // ε parameterizes every content key (and every
                // predicate band): the whole cone recomputes.
                structural: true,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::SetAppearanceMeta { name, key, value } => {
            // Same v1 scope and node-liveness carve-out as
            // SetAppearance: the metadata rides the SAME record.
            if !matches!(name.kind, EntityKind::Face | EntityKind::Body) {
                return Err(EditError::AppearanceWrongKind {
                    name: doc.spoken_name(name),
                });
            }
            if !new.nodes.contains_key(&name.node) {
                return Err(EditError::AppearanceNamesMissingNode {
                    name: doc.spoken_name(name),
                });
            }
            check_name_steps(doc, new, name)?;
            // D7's producer convention, by the one predicate
            // `MetaValue::require_versioned`, which the save/load
            // validator also calls. Only the WALK differs between the
            // two doors, and irreducibly: this door holds the one value
            // it is about to write, and the validator holds a map that
            // arrived whole.
            if let Err(error) = value.require_versioned() {
                return Err(EditError::MetaUnversioned {
                    name: doc.spoken_name(name),
                    key: key.clone(),
                    error,
                });
            }
            if let Some(path) = value.first_non_finite() {
                return Err(EditError::MetaNonFinite {
                    name: doc.spoken_name(name),
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
                minted_var: None,
                structural: false,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::ClearAppearanceMeta { name, key } => {
            let not_set = || EditError::MetaNotSet {
                name: doc.spoken_name(name),
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
                minted_var: None,
                structural: false,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::SetOffset {
            instance,
            offset,
            fresh,
        } => {
            match new.nodes.get(instance) {
                None => {
                    return Err(EditError::UnknownNode {
                        id: SpokenNode::absent(*instance),
                    });
                }
                Some(Node::InstantiatePart { .. }) => {}
                Some(_) => {
                    return Err(EditError::OffsetOnNonInstance {
                        node: doc.spoken(*instance),
                    });
                }
            }
            let lowering = Lowering::start(new, fresh)?;
            let lowered = match offset {
                None => None,
                Some(offset) => {
                    let rows = offset.rows();
                    let spoken = || doc.spoken(*instance);
                    let lowered = lower_value(new, &lowering, &rows, |f| {
                        offset.try_map_slots(&mut |e| f(e))
                    })
                    .map_err(|unlowered| {
                        unlowered.refuse(new, spoken, ExprSite::Slot, |fault| {
                            fault.at(new, spoken(), ExprSite::Payload)
                        })
                    })?;
                    if let Some(SlotDimensionFault {
                        slot,
                        expected,
                        found,
                    }) = rows
                        .iter()
                        .find_map(|&(slot, formula)| slot.dimension_fault(formula))
                    {
                        return Err(EditError::SlotDimensionMismatch {
                            slot,
                            expected,
                            found,
                        });
                    }
                    Some(lowered)
                }
            };
            if let Some(Node::InstantiatePart { offset: held, .. }) = new.nodes.get_mut(instance) {
                *held = lowered;
            }
            // The offset's rigid steps are the instance's slots, held
            // to the insert door's own checks: dimensions and parameter
            // references. Its literal steps meet the frame rule in the
            // backstop `door` holds every edit to, with every other
            // placement's.
            let probe = new.nodes[instance].clone();
            check_node_slots(new, doc, *instance, &probe)?;
            let fresh = lowering.finish(new)?;
            // Structural: where an instance sits is recipe shape, and it
            // moves the document's content pin.
            EditRecord {
                minted: None,
                minted_var: None,
                structural: true,
                fresh,
                outputs: Vec::new(),
            }
        }
        DocEdit::SetGauge { node, gauge } => {
            match new.nodes.get(node) {
                None => {
                    return Err(EditError::UnknownNode {
                        id: SpokenNode::absent(*node),
                    });
                }
                Some(Node::InstantiatePart { .. } | Node::Gauge { .. }) => {}
                Some(_) => {
                    return Err(EditError::GaugeOnNonPlaced {
                        node: doc.spoken(*node),
                    });
                }
            }
            check_gauge_ref(new, *node, *gauge, || doc.spoken(*node))?;
            match new.nodes.get_mut(node) {
                Some(
                    Node::InstantiatePart { gauge: held, .. } | Node::Gauge { parent: held, .. },
                ) => {
                    *held = *gauge;
                }
                _ => unreachable!("node {} was checked an instance or a gauge above", node),
            }
            EditRecord {
                minted: None,
                minted_var: None,
                structural: true,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::Promote { instance } => {
            let PromotePlan {
                parent,
                offset,
                group,
            } = promote_plan(doc, *instance)?;
            let promoted: Node<P> = Node::gauge(parent, offset);
            let mut mint = new.mint.clone();
            let id = mint.insert(&promoted);
            // The gauge's parent is the instance's gauge, which a live
            // group's root may name dangling: the insert door's check.
            check_gauge_ref(new, id, parent, || SpokenNode::entering(id, &promoted))?;
            new.mint = mint;
            new.nodes.insert(id, promoted);
            for member in &group {
                let Some(Node::InstantiatePart { gauge, offset, .. }) = new.nodes.get_mut(member)
                else {
                    unreachable!("a group's members are live instances")
                };
                *gauge = Some(id);
                if member == instance {
                    *offset = Some(crate::placement::Placement::IDENTITY);
                }
            }
            EditRecord {
                minted: Some(id),
                minted_var: None,
                structural: true,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::Fold { gauge } => {
            let Some(target) = doc.node(*gauge) else {
                return Err(EditError::UnknownNode {
                    id: SpokenNode::absent(*gauge),
                });
            };
            let Node::Gauge { parent, placement } = target else {
                return Err(EditError::FoldOnNonGauge {
                    node: doc.spoken(*gauge),
                });
            };
            let after = |id: RecipeNodeId| match doc.node(id).and_then(Node::gauge_ref) {
                Some(g) if g == *gauge => *parent,
                other => other,
            };
            if let Some(mate) = mate_that_would_start_placing(doc, after) {
                return Err(EditError::FoldWouldStartPlacing {
                    node: doc.spoken(*gauge),
                    mate: doc.spoken(mate),
                });
            }
            let dependents: Vec<RecipeNodeId> = doc
                .ids()
                .iter()
                .copied()
                .filter(|&id| doc.node(id).and_then(Node::gauge_ref) == Some(*gauge))
                .collect();
            for dependent in &dependents {
                match new.nodes.get_mut(dependent) {
                    Some(Node::InstantiatePart {
                        gauge: held,
                        offset,
                        ..
                    }) => {
                        *held = *parent;
                        if let Some(offset) = offset {
                            *offset = placement.compose(offset);
                        }
                    }
                    Some(Node::Gauge {
                        parent: held,
                        placement: own,
                    }) => {
                        *held = *parent;
                        *own = placement.compose(own);
                    }
                    _ => unreachable!("a node with a gauge reference is an instance or a gauge"),
                }
            }
            if let Some(label) = new.labels.get(gauge).cloned() {
                match dependents[..] {
                    [only] if !new.labels.contains_key(&only) => {
                        new.labels.insert(only, label);
                    }
                    _ => reported.push(Maintenance::LabelDropped {
                        gauge: doc.spoken(*gauge),
                        label,
                    }),
                }
            }
            reported.extend(remove_node(doc, new, *gauge));
            EditRecord {
                minted: None,
                minted_var: None,
                structural: true,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::SetLabel { node, label } => {
            if !new.nodes.contains_key(node) {
                return Err(EditError::UnknownNode {
                    id: SpokenNode::absent(*node),
                });
            }
            if new.labels.get(node) == label.as_ref() {
                return Err(EditError::LabelUnchanged {
                    node: doc.spoken(*node),
                });
            }
            match label {
                Some(label) => new.labels.insert(*node, label.clone()),
                None => new.labels.remove(node),
            };
            EditRecord {
                minted: None,
                minted_var: None,
                structural: false,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
        DocEdit::UpdateReference { node, new_pin } => {
            // Three refusals, each naming its own subject — an unknown
            // id and a live-but-wrong-kind node are different mistakes
            // and must not collapse into one message.
            let Some(target) = new.nodes.get_mut(node) else {
                return Err(EditError::UnknownNode {
                    id: SpokenNode::absent(*node),
                });
            };
            let Node::InstantiatePart { doc_ref, .. } = target else {
                return Err(EditError::UpdateOnNonInstance {
                    node: doc.spoken(*node),
                });
            };
            if doc_ref.pin == *new_pin {
                return Err(EditError::PinUnchanged {
                    node: doc.spoken(*node),
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
                minted_var: None,
                structural: true,
                fresh: Vec::new(),
                outputs: Vec::new(),
            }
        }
    })
}

/// **The mate a regauge would make start placing** (A11 (2)): the
/// first mate, in document order, that declares in `doc` — its two
/// instances on different gauge references — and would place once
/// every instance sits where `gauge_after` puts it, which would move a
/// group nobody named.
///
/// One predicate for every edit that regauges as a side effect of
/// what its caller named ([`regauge_then_mate`], [`DocEdit::Fold`]).
/// [`DocEdit::SetGauge`] does not ask it: there the regauge is what
/// the caller named.
fn mate_that_would_start_placing<P>(
    doc: &Doc<P>,
    gauge_after: impl Fn(RecipeNodeId) -> Option<RecipeNodeId>,
) -> Option<RecipeNodeId> {
    doc.ids().iter().copied().find(|&id| {
        let Some(Node::Mate { a, b, .. }) = doc.node(id) else {
            return false;
        };
        let (Some(x), Some(y)) = (
            crate::mate::member_of(doc, a),
            crate::mate::member_of(doc, b),
        ) else {
            return false;
        };
        !crate::mate::places(doc, x.instance, y.instance)
            && gauge_after(x.instance) == gauge_after(y.instance)
    })
}

/// **A node removed** (D10: deleting a variable leaves its readers
/// unresolved, typed, never re-pointed): it leaves the node table, the
/// order and the witness and label stores, and its outputs leave
/// the variable table. Nothing is refused: the answer is the report of
/// what it stranded — each operand still reading one of its outputs
/// ([`Maintenance::StrandedRead`], [`stranded_reads`]), then DM7's
/// surviving names whose minting node just left
/// ([`stranded_references`]). The mint log keeps the ids: they are
/// never reused (D3). `before` is the document the door was handed.
fn remove_node<P: crate::ProfilePayload>(
    before: &Doc<P>,
    new: &mut Doc<P>,
    id: RecipeNodeId,
) -> Vec<Maintenance> {
    let outputs = new.outputs(id);
    if new.nodes.remove(&id).is_none() {
        unreachable!("node {} is removed only while live", id)
    }
    let mut reported = stranded_reads(before, new, &outputs);
    reported.extend(stranded_references(before, new, id));
    new.witnesses.remove(&id);
    new.labels.remove(&id);
    for var in outputs {
        new.vars.remove(&var);
        new.var_names.remove(&var);
    }
    reported
}

/// The nodes of `doc` that read the selection `var`, in document order,
/// spoken from `before`.
fn selection_readers<P: crate::ProfilePayload>(
    before: &Doc<P>,
    doc: &Doc<P>,
    var: VarId,
) -> Vec<SpokenNode> {
    doc.ids()
        .into_iter()
        .filter(|&id| {
            doc.node(id)
                .is_some_and(|node| node.operand_rows().iter().any(|&(_, read)| read == var))
        })
        .map(|id| before.spoken(id))
        .collect()
}

/// **The selections of `doc` that go with the edit** (VR7): unnamed,
/// with no reader left. A name one holds is stranded by nothing.
fn selections_going<P: crate::ProfilePayload>(doc: &Doc<P>) -> std::collections::BTreeSet<VarId> {
    let readers = doc.reader_counts();
    doc.vars()
        .iter()
        .filter(|(id, var)| {
            var.def().select().is_some()
                && doc.var_name(**id).is_none()
                && readers.get(id).copied().unwrap_or(0) == 0
        })
        .map(|(&id, _)| id)
        .collect()
}

/// **Every operand of `doc` reading one of `removed`**, directly or as
/// a selection's body, in document order and within a node in field
/// order, as the strand rows a removal reports, spoken from `before`,
/// which still holds them.
fn stranded_reads<P: crate::ProfilePayload>(
    before: &Doc<P>,
    doc: &Doc<P>,
    removed: &[VarId],
) -> Vec<Maintenance> {
    doc.nodes
        .iter()
        .flat_map(|(&node, n)| {
            n.operand_rows()
                .into_iter()
                .map(|(slot, read)| (slot, doc.selection(read).map_or(read, |s| s.body)))
                .filter(|(_, read)| removed.contains(read))
                .map(move |(slot, var)| Maintenance::StrandedRead {
                    node: before.spoken(node),
                    slot,
                    var: before.spoken_var(var),
                })
        })
        .collect()
}

/// What a [`DocEdit::Promote`] builds: the gauge's parent and
/// placement, and the group that moves onto it.
pub(crate) struct PromotePlan {
    /// The instance's gauge, which the minted gauge sits on.
    pub(crate) parent: Option<RecipeNodeId>,
    /// The instance's offset, which the minted gauge holds.
    pub(crate) offset: crate::placement::Placement,
    /// The instance's group, in document order.
    pub(crate) group: Vec<RecipeNodeId>,
}

/// **Promote's refusals, and what it would build** — the one reading
/// of "can this instance's offset become a gauge", which
/// [`DocEdit::Promote`] and `refactor::inline`'s minted gauge share.
///
/// The root is the earliest member carrying an offset (A11 (2)),
/// asked before anything else so each recourse names an edit that
/// moves forward: a member that is not that one refuses
/// [`EditError::PromoteNonRoot`] naming it; a group with no offset at
/// all refuses [`EditError::PromoteWithoutOffset`] (setting the
/// instance's then makes it the root); another member's offset refuses
/// [`EditError::PromoteMemberOffset`].
///
/// # Errors
///
/// [`EditError::UnknownNode`], [`EditError::PromoteOnNonInstance`] and
/// the three above.
pub(crate) fn promote_plan<P>(
    doc: &Doc<P>,
    instance: RecipeNodeId,
) -> Result<PromotePlan, EditError> {
    let Some(target) = doc.node(instance) else {
        return Err(EditError::UnknownNode {
            id: SpokenNode::absent(instance),
        });
    };
    let Node::InstantiatePart { gauge: parent, .. } = target else {
        return Err(EditError::PromoteOnNonInstance {
            node: doc.spoken(instance),
        });
    };
    let groups = crate::mate::groups(doc);
    let Some(group) = groups.into_iter().find(|g| g.contains(&instance)) else {
        unreachable!("a live instance is in one of its document's groups")
    };
    let offset_of = |id: RecipeNodeId| match doc.node(id) {
        Some(Node::InstantiatePart { offset, .. }) => offset.clone(),
        _ => None,
    };
    let Some(root) = group.iter().copied().find(|&m| offset_of(m).is_some()) else {
        return Err(EditError::PromoteWithoutOffset {
            node: doc.spoken(instance),
        });
    };
    if root != instance {
        return Err(EditError::PromoteNonRoot {
            node: doc.spoken(instance),
            root: doc.spoken(root),
        });
    }
    if let Some(&member) = group
        .iter()
        .find(|&&m| m != instance && offset_of(m).is_some())
    {
        return Err(EditError::PromoteMemberOffset {
            node: doc.spoken(instance),
            member: doc.spoken(member),
        });
    }
    let Some(offset) = offset_of(instance) else {
        unreachable!("the root carries an offset")
    };
    Ok(PromotePlan {
        parent: *parent,
        offset,
        group,
    })
}

/// **A gauge reference, at the edit doors**: `gauge`, written on
/// `node`, names a live gauge whose chain does not reach `node`
/// ([`crate::doc::gauge_ref_fault`]).
///
/// `spoken` says `node` as the refusal names it: the entering node at
/// an insert, the held one at a re-gauge.
fn check_gauge_ref<P>(
    doc: &Doc<P>,
    node: RecipeNodeId,
    gauge: Option<RecipeNodeId>,
    spoken: impl FnOnce() -> SpokenNode,
) -> Result<(), EditError> {
    let Some(target) = gauge else {
        return Ok(());
    };
    let Some(fault) = crate::doc::gauge_ref_fault(doc, node, gauge) else {
        return Ok(());
    };
    let (node, gauge) = (spoken(), doc.spoken(target));
    Err(match fault {
        GaugeRefFault::Deleted | GaugeRefFault::NeverMinted => {
            EditError::GaugeNotLive { node, gauge }
        }
        GaugeRefFault::NotAGauge => EditError::NotAGauge { node, gauge },
        GaugeRefFault::Cycle => EditError::GaugeCycle { node, gauge },
    })
}

/// **The mate door** (A11 (2)): a PLACING mate that joins two groups
/// places the first operand's group on the second's — "mate `a` to
/// `b`" moves `a` — so when `b`'s group is placed, every offset in
/// `a`'s group is cleared, the root's and the checked members' alike:
/// they stated where `a`'s group sat, and the mate moves it. The
/// merged group's root is then `b`'s root (the root rule,
/// `mate::solve::root_and_cause`, finds no candidate in `a`'s), so `b`
/// does not move. A `b` group nothing places has no world pose to
/// keep, and nothing is cleared. Which side moves is independent of
/// which side's frame states the datum ([`crate::mate::Alignment`]).
/// `before` is the document without the mate, which is where the two
/// groups are read; `after` holds it. Each cleared offset is reported,
/// in document order. Structural: walks and offsets only, so replay
/// reproduces it with no solve.
fn clear_joined_offsets<P: crate::ProfilePayload>(
    before: &Doc<P>,
    after: &mut Doc<P>,
    mate: &Node<P>,
) -> Vec<Maintenance> {
    let Node::Mate { a, b, .. } = mate else {
        return Vec::new();
    };
    let (Some(ia), Some(ib)) = (
        crate::mate::member_of(after, a).map(|m| m.instance),
        crate::mate::member_of(after, b).map(|m| m.instance),
    ) else {
        return Vec::new();
    };
    if !crate::mate::places(after, ia, ib) {
        return Vec::new();
    }
    let groups = crate::mate::groups(before);
    let group_of = |i| groups.iter().find(|g| g.contains(&i));
    let (Some(ga), Some(gb)) = (group_of(ia), group_of(ib)) else {
        return Vec::new();
    };
    if ga == gb || crate::mate::solve::root_and_cause(before, gb).1.is_some() {
        return Vec::new();
    }
    let mut cleared = Vec::new();
    for &instance in ga {
        if let Some(Node::InstantiatePart { offset, .. }) = after.nodes.get_mut(&instance)
            && let Some(offset) = offset.take()
        {
            cleared.push(Maintenance::OffsetCleared {
                instance: before.spoken(instance),
                offset,
            });
        }
    }
    cleared
}

/// A witness edit's site check: the store's key rule
/// ([`crate::doc::witness_site_fault`], the same question the load
/// door's walk asks of a file's store), rendered in this door's
/// vocabulary.
fn check_witness_site<P>(doc: &Doc<P>, id: RecipeNodeId) -> Result<(), EditError> {
    match crate::doc::witness_site_fault(doc, id) {
        None => Ok(()),
        Some(WitnessSiteFault::NoSuchNode) => Err(EditError::UnknownNode {
            id: SpokenNode::absent(id),
        }),
        Some(WitnessSiteFault::NotSketchBearing) => Err(EditError::WitnessOnNonSketch {
            node: doc.spoken(id),
        }),
    }
}

/// The VQ9 door after a slot edit landed in a profile program
/// (LIB-SWITCH §4d): re-run resolve + replay + validate under the
/// CURRENT param env; refuse typed. Non-profile slots pass through.
fn check_profile_after_slot_edit<P: crate::ProfilePayload>(
    new: &Doc<P>,
    before: &Doc<P>,
    id: RecipeNodeId,
    slot: SlotId,
    tol: Tol,
) -> Result<(), EditError> {
    if matches!(slot, SlotId::Profile { .. })
        && let Some(Node::Profile(p)) = new.nodes.get(&id)
    {
        p.check(&new.var_env::<f64>(), tol).map_err(|refusal| {
            EditError::ProfileProgramRefused {
                node: before.spoken(id),
                refusal: Box::new(refusal),
            }
        })?;
    }
    Ok(())
}

/// Shared slot-write path: node exists, slot exists, dimension
/// matches, the formula lowers and its reads hold — then the slot
/// stores the variable it lowers to (VR4), its fresh table minted
/// first.
fn set_slot<P: Clone + crate::ProfilePayload>(
    new: &mut Doc<P>,
    before: &Doc<P>,
    id: RecipeNodeId,
    slot: SlotId,
    formula: &Formula,
    fresh: &[FreshEntry],
) -> Result<Vec<VarId>, EditError> {
    let Some(node) = new.nodes.get(&id) else {
        return Err(EditError::UnknownNode {
            id: SpokenNode::absent(id),
        });
    };
    let spoken = before.spoken(id);
    // Whether the node HAS the slot is this door's own question: the
    // subject is an address a caller named, and `SetParam` aimed at a
    // radius on an extrude is a reachable mistake rather than the
    // node-layer invariant `Node::formula_dimension_fault` asserts.
    let holds = match slot {
        SlotId::Operand(operand) => node.operand_rows().iter().any(|(at, _)| *at == operand),
        _ => node.expr(slot).is_some(),
    };
    if !holds {
        return Err(EditError::UnknownSlot { id: spoken, slot });
    }
    // D6's comparison, from its one home (`SlotId::dimension_fault`) —
    // the same rule the node-wide walk asks, of a formula the node
    // does not hold yet.
    if let Some(SlotDimensionFault {
        slot,
        expected,
        found,
    }) = slot.dimension_fault(formula)
    {
        return Err(EditError::SlotDimensionMismatch {
            slot,
            expected,
            found,
        });
    }
    let lowering = Lowering::start(new, fresh)?;
    let var = lowering
        .slot(new, formula)
        .map_err(|fault| fault.at(new, spoken.clone(), ExprSite::Slot(slot)))?;
    let reader = Expr::var(var, slot.expr_dimension());
    check_reads(new, &spoken, ExprSite::Slot(slot), &reader)?;
    let Some(target) = new.nodes.get_mut(&id).and_then(|n| n.expr_mut(slot)) else {
        return Err(EditError::UnknownSlot { id: spoken, slot });
    };
    *target = var;
    lowering.finish(new)
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
    /// re-mint deterministically; nothing is solved —
    /// [`apply_replayed`] — so no store is in hand and none is needed). The document id is supplied, not
    /// replayed: identity is authored data the log never carries
    /// (ASM-1 D-1).
    ///
    /// The answer is the document, not the maintenance its edits
    /// performed: [`Applied::maintenance`] is a fact about ONE
    /// application, reported to the caller who made it, and what it
    /// did is already in the document it produced — a cleared offset
    /// is cleared, and a stranded
    /// name (DM7) resolves to nothing until rebound, which the next
    /// evaluation reports typed (N5). The replayed document is the
    /// state, the same boundary the load door draws
    /// ([`crate::persist::Loaded`]); the round trip is lossless
    /// because the same delete against the replayed document reports
    /// the same strands, which DM7's round-trip row pins.
    ///
    /// # Errors
    ///
    /// [`apply_replayed`]'s.
    pub fn replay(
        id: crate::DocumentId,
        log: &[DocEdit<P>],
        tol: Tol,
    ) -> Result<Doc<P>, EditError> {
        let mut doc = Doc::empty(id, tol);
        for edit in log {
            doc = apply_replayed(&doc, edit, tol)?.doc;
        }
        Ok(doc)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::expect_used)]

    use super::DocEdit;
    use crate::program::ProfileProgram;
    use crate::test_support::len;

    /// **The datum question is answered by the edit too**: the insert
    /// of a mate and a slot edit at a mate's frame-offset step write a
    /// mate's alignment datum, and nothing else does — not the insert
    /// of another node, not a slot edit at any other address, not the
    /// rebind that moves a head (a reference, not the datum), not the
    /// structural edit that shrinks a pattern under one. The per-mate
    /// admission is asked at exactly the edits this answers `true` for.
    #[test]
    fn exactly_the_mate_insert_writes_a_mates_datum() {
        let id = crate::node::RecipeNodeId::new(0, 1);
        let name = |node| crate::names::StableName {
            kind: crate::names::EntityKind::Face,
            node,
            path: vec![],
        };
        let frame = crate::mate::MateFrame::from_face();
        let mate: DocEdit<ProfileProgram> = DocEdit::InsertNode {
            node: Box::new(crate::node::Node::Mate {
                a: crate::node::SitedFace::at_mint(
                    crate::names::FaceName::new(name(id)).expect("a face"),
                ),
                b: crate::node::SitedFace::at_mint(
                    crate::names::FaceName::new(name(crate::node::RecipeNodeId::new(0, 2)))
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
            }),
            fresh: Vec::new(),
        };
        assert!(mate.writes_a_mates_datum());
        let step = crate::node::SlotId::MateFrameStep {
            side: crate::mate::MateSide::B,
            step: 0,
            arg: crate::node::RigidArg::RotationAngle,
        };
        let at_a_step: [DocEdit<ProfileProgram>; 2] = [
            DocEdit::SetParam {
                node: id,
                slot: step,
                value: crate::test_support::ang(0.5).into(),
                fresh: Vec::new(),
            },
            DocEdit::SetExpression {
                path: crate::expr::ExprPath {
                    node: id,
                    slot: step,
                    path: vec![],
                },
                expr: crate::test_support::ang(0.5),
            },
        ];
        for edit in &at_a_step {
            assert!(edit.writes_a_mates_datum(), "{edit:?} writes a frame step");
        }
        let others: [DocEdit<ProfileProgram>; 4] = [
            DocEdit::InsertNode {
                node: Box::new(crate::node::Node::Datum(crate::node::Datum::Point {
                    position: [len(0.0), len(0.0), len(0.0)],
                })),
                fresh: Vec::new(),
            },
            DocEdit::Rebind {
                body: None,
                from: name(id),
                to: name(crate::node::RecipeNodeId::new(0, 2)),
            },
            DocEdit::SetStructuralParam {
                node: id,
                slot: crate::node::SlotId::Count,
                expr: crate::Formula::count(1),
                fresh: Vec::new(),
            },
            DocEdit::SetParam {
                node: id,
                slot: crate::node::SlotId::RotationAngle,
                value: crate::test_support::ang(0.5).into(),
                fresh: Vec::new(),
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
