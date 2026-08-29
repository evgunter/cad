//! The DocEdit vocabulary v1 and the pure `apply` (spec D2 + D6).
//!
//! `apply` returns a NEW document value; the input is untouched.
//! Undo/redo is keeping prior values — no edit destroys history at
//! this layer (spec D2).

use crate::appearance::{Attr, AttrKind};
use crate::distribution::DistributionFault;
use crate::doc::{Doc, DocParam, DocParamValue, ParamName};
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
    /// Delete a node. Refused while any live node references it
    /// (typed, spec D3/D6); the id is never reused afterwards.
    DeleteNode {
        /// The node to delete.
        id: RecipeNodeId,
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
        /// The typed refusal.
        refusal: crate::program::ProgramRefusal,
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
    /// A value-only edit ([`DocEdit::SetDocParamValue`]) named a
    /// parameter this document does not declare. The value door
    /// carries an existing declaration forward, so there has to be
    /// one; declaring a parameter is [`DocEdit::SetDocParam`]'s job.
    DocParamNotDeclared {
        /// The undeclared parameter.
        name: ParamName,
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

// LIB-DOORS F6 (reopened on review): the human-readable rendering the
// bindings' exception messages consume. The comment-style rule
// applies — each arm states the PROBLEM (and where it is), not the
// enum's guts; identifiers render via `Debug` because they ARE the
// location, and the typed variant remains the machine contract.
impl core::fmt::Display for EditError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownNode { id } => write!(f, "edit: node {} is not live", id.0),
            Self::ProfileProgramRefused { node, refusal } => {
                write!(
                    f,
                    "edit: node {}'s profile program refused: {refusal}",
                    node.0
                )
            }
            Self::UnresolvedInput { input } => {
                write!(f, "edit: input {} does not resolve to a live node", input.0)
            }
            Self::WouldCycle { at } => {
                write!(
                    f,
                    "edit: the recipe graph would cycle (through node {})",
                    at.0
                )
            }
            Self::DeleteWouldDangle { id, referenced_by } => write!(
                f,
                "edit: deleting node {} would dangle node {}'s reference to it",
                id.0, referenced_by.0
            ),
            Self::UnknownSlot { id, slot } => {
                write!(f, "edit: node {} has no slot {slot:?}", id.0)
            }
            Self::SlotDimensionMismatch {
                slot,
                expected,
                found,
            } => write!(
                f,
                "edit: slot {slot:?} needs a {expected:?} expression, got {found:?}"
            ),
            Self::StructuralSlotNeedsStructuralEdit { slot } => write!(
                f,
                "edit: slot {slot:?} is structural — use a structural edit"
            ),
            Self::NotStructuralSlot { slot } => {
                write!(f, "edit: slot {slot:?} is continuous, not structural")
            }
            Self::UnknownPayloadParam { name, node } => write!(
                f,
                "edit: document parameter {:?} does not exist (referenced by node {}'s \
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
                "edit: document parameter {:?} is declared {declared:?} but node {}'s \
                 measurement payload references it as {referenced:?}",
                name.0, node.0
            ),
            Self::MeasureMalformed { node, fault } => {
                write!(f, "edit: measure node {}: {fault}", node.0)
            }
            Self::AssertionTarget { node, measure } => write!(
                f,
                "edit: assertion node {} references node {}, which is not a measure — an \
                 assertion constrains a measurement",
                node.0, measure.0
            ),
            Self::AssertionDimension {
                node,
                measure,
                measured,
                bound,
            } => write!(
                f,
                "edit: assertion node {} bounds a {measured:?} measure (node {}) with a \
                 {bound:?} expression — an assertion compares like with like or not at all",
                node.0, measure.0
            ),
            Self::UnknownDocParam { name, node, slot } => write!(
                f,
                "edit: document parameter {:?} does not exist (referenced by node {}, slot {slot:?})",
                name.0, node.0
            ),
            Self::DocParamDimensionMismatch {
                name,
                node,
                slot,
                declared,
                referenced,
            } => write!(
                f,
                "edit: parameter {:?} is declared {declared:?} but node {} (slot {slot:?}) references it as {referenced:?}",
                name.0, node.0
            ),
            Self::ContinuousParamCannotBeCount { name } => write!(
                f,
                "edit: parameter {:?}: a continuous parameter cannot be Count — use a Count parameter",
                name.0
            ),
            Self::DocParamNotDeclared { name } => write!(
                f,
                "edit: parameter {:?} is not declared, so a value edit has no declaration to carry \
                 forward — declare it first",
                name.0
            ),
            Self::DocParamValueKindMismatch {
                name,
                declared,
                offered,
            } => write!(
                f,
                "edit: parameter {:?} is declared {declared:?} but the value edit offered a \
                 {offered} — changing a parameter's kind is a redeclaration",
                name.0
            ),
            Self::PathOffTree { path } => {
                write!(f, "edit: expression path {path:?} runs off the tree")
            }
            Self::Dimension(e) => write!(f, "edit: {e}"),
            Self::DeclareNamesMissingNode { name } => write!(
                f,
                "edit: declared name {name:?} refers to a node that is not live"
            ),
            Self::NonFiniteDocParam { name } => write!(
                f,
                "edit: parameter {:?}: the value and every distribution offset must be finite",
                name.0
            ),
            Self::InvalidDistribution { name, fault } => {
                write!(f, "edit: parameter {:?}: {fault}", name.0)
            }
            Self::RebindTargetMissingNode { name } => write!(
                f,
                "edit: rebind target {name:?} refers to a node that is not live"
            ),
            Self::RebindUnknownName { name } => write!(
                f,
                "edit: rebind source {name:?} was never minted by this document"
            ),
            Self::RebindKindMismatch { from, to } => write!(
                f,
                "edit: a rebind cannot cross entity kinds ({from:?} to {to:?})"
            ),
            Self::RebindIdentity { name } => write!(
                f,
                "edit: rebinding {name:?} to itself is a recorded no-op — refused"
            ),
            Self::RebindNoReferences { name } => write!(
                f,
                "edit: no document site references {name:?} — nothing to repair"
            ),
            Self::WitnessOnNonSketch { node } => write!(
                f,
                "edit: node {} is not sketch-bearing — nothing to re-witness",
                node.0
            ),
            Self::DuplicateWitnessEntry { node } => write!(
                f,
                "edit: node {} appears twice in the re-witness bulk",
                node.0
            ),
            Self::EmptyWitnessBulk => {
                f.write_str("edit: a re-witness bulk with no entries is a no-op — refused")
            }
            Self::NameUnresolvedInEvaluation { name } => write!(
                f,
                "edit: name {name:?} does not resolve in the supplied evaluation — recording the reference would strand it"
            ),
            Self::RebindAppearanceCollision { name, kind } => write!(
                f,
                "edit: the rebind would land two {kind:?} attributes on {name:?} — clear one first"
            ),
            Self::AppearanceWrongKind { name } => write!(
                f,
                "edit: appearance attaches to faces and bodies only (refused for {name:?})"
            ),
            Self::AppearanceNamesMissingNode { name } => write!(
                f,
                "edit: appearance name {name:?} refers to a node that is not live"
            ),
            Self::AppearanceNotSet { name, kind } => {
                write!(f, "edit: no {kind:?} attribute is set on {name:?}")
            }
            Self::InvalidTolerance { value } => write!(
                f,
                "edit: tolerance {value:e} is not finite and strictly positive"
            ),
            Self::MetaUnversioned { name, key, error } => write!(
                f,
                "edit: metadata {key:?} on {name:?} does not carry the D7 integer \"v\" \
                 version field: {error}"
            ),
            Self::MetaNonFinite { name, key, path } => write!(
                f,
                "edit: metadata {key:?} on {name:?} carries a non-finite float at {path}"
            ),
            Self::MetaNotSet { name, key } => {
                write!(f, "edit: no metadata {key:?} is set on {name:?}")
            }
            Self::RebindMetadataCollision { name, key } => write!(
                f,
                "edit: the rebind would land two values under metadata {key:?} on {name:?} — clear one first"
            ),
            Self::Roots(fault) => write!(f, "edit: {fault}"),
            Self::PlacementOnNonInstance { node } => write!(
                f,
                "edit: node {} does not instantiate a part, so it has no placement cluster to \
                 place",
                node.0
            ),
            // The two rule-shaped arms FORWARD the fault set's one
            // prose vocabulary (`PlacementRuleFault`'s `Display`); the
            // two frame-shaped arms below keep their own prose because
            // their subject is a single cluster frame, which has no
            // index in a rule's placement list.
            Self::EmptyPlacementList { node } => write!(
                f,
                "edit: node {}: {}",
                node.0,
                PlacementRuleFault::NoPlacements
            ),
            Self::PlacementRuleMismatch { node } => write!(
                f,
                "edit: node {}: {}",
                node.0,
                PlacementRuleFault::CountSpelling
            ),
            Self::ImproperPlacement { node, determinant } => write!(
                f,
                "edit: the placement frame for node {} is improper (determinant {determinant}); \
                 mirrored placements are admitted only behind the equivariance audit",
                node.0
            ),
            Self::NonFinitePlacement { node } => write!(
                f,
                "edit: the placement frame for node {} carries a non-finite coordinate",
                node.0
            ),
            Self::NonFiniteAlignment { node } => write!(
                f,
                "edit: the mate at node {} carries a non-finite alignment coordinate",
                node.0
            ),
            Self::UpdateOnNonInstance { node } => write!(
                f,
                "edit: node {} does not instantiate a part, so it has no pinned version to update",
                node.0
            ),
            Self::PinUnchanged { node, pin } => write!(
                f,
                "edit: node {} already pins {pin}, so this update would record no version move",
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

/// An accepted edit: the NEW document (the input untouched, spec D2)
/// plus the [`EditRecord`].
#[derive(Debug, Clone, PartialEq)]
pub struct Applied<P> {
    /// The new document value.
    pub doc: Doc<P>,
    /// What the edit did.
    pub record: EditRecord,
    /// **The A11 cluster-record maintenance** this edit performed
    /// (ASM-R2a D-3): the joins, splits, gauge rewrites and drops the
    /// mate graph's motion forced on the placement registry.
    ///
    /// It rides the accepted edit rather than being a second edit of
    /// its own — the A10 root-list precedent, verbatim: automatic
    /// maintenance is the invariant's own bookkeeping, deterministic
    /// from the edit, so a replay reproduces it and undo (keeping the
    /// prior document value) restores it exactly. What the record
    /// adds is VISIBILITY: an absorbed cluster's frame is consumed
    /// here, where a caller can read what was consumed.
    pub maintenance: Vec<crate::mate::ClusterMaintenance>,
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

/// Reject cycles in the recipe DAG (spec D3/D6). Defensive: insertion
/// referencing only existing nodes cannot cycle, but the invariant is
/// checked. Iterative DFS, three-color, deterministic order.
fn check_acyclic<P>(doc: &Doc<P>) -> Result<(), EditError> {
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
            // chamfer alongside it), a mate's two heads under A12).
            for name in node.payload_names() {
                if !new.nodes.contains_key(&name.node) {
                    return Err(EditError::DeclareNamesMissingNode { name: name.clone() });
                }
            }
            let id = RecipeNodeId(new.next_id);
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
                p.check(&new.param_env::<f64>(), tol)
                    .map_err(|refusal| EditError::ProfileProgramRefused { node: id, refusal })?;
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
                return Err(EditError::DocParamNotDeclared { name: name.clone() });
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
            // alike — appearance-store keys). Zero sites = nothing to
            // repair, refused.
            // Every payload site, by the one list that says which
            // payloads carry a name (`Node::payload_names`' twin): the
            // rewrite reaches a mate's heads exactly as it reaches a
            // Declare pair, and a blend selection's GROWTH PATH (M6-5,
            // ruled #217) re-canonicalizes there — for a chamfer's
            // selection exactly as for a fillet's, since both are the
            // same canonical set.
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
    let maintenance = if reconcile {
        crate::mate::solve::reconcile(doc, &mut new, tol)
    } else {
        Vec::new()
    };
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
        p.check(&new.param_env::<f64>(), tol)
            .map_err(|refusal| EditError::ProfileProgramRefused { node: id, refusal })?;
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
