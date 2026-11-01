//! **The typed operations on the session, and what one did**: the
//! crate's shared operation vocabulary.
//!
//! A VOCABULARY. [`SessionOp`] is a value the panels name and the
//! session performs, [`OpOutcome`] is the value it reports; neither
//! names the session, which is what lets a test name the same op a
//! widget does.
//!
//! [`SessionOp::permitted_during_value_gesture`] is the mid-gesture
//! policy as data — one match rather than a rule inferred from every
//! dispatch target.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::path::PathBuf;

use pncad::document::{
    Coord, DocEdit, DocumentId, Formula, Frame, FreeVar, Label, LoopProgram, Maintenance, Mate,
    MateRef, Operand, ProfileProgram, RecipeNodeId, SlotId, StepId, VarId, VarName,
};
use pncad::prelude::StableName;
use pncad::quantity::UnitDef;

use crate::display::PruneReport;
use crate::props::SlotValue;
use crate::session::author::{PartSelectSpec, PatternRuleSpec};
use crate::session::probe::BoundsTarget;
use crate::session::refuse::Refusal;
use crate::session::select::{FaceSelection, Hovered, Selection};

/// One typed operation on the session.
#[derive(Clone, Debug, PartialEq)]
pub enum SessionOp {
    /// Move the selection.
    Select(Selection),
    /// Move the transient hover, or clear it with `None`.
    ///
    /// **Hover is layer-3 state and nothing else**: it never enters
    /// the document, never enters the history, and is not persisted.
    /// It is an operation rather than a field the chrome writes for
    /// the same reason every other move here is one (G1's
    /// operations-are-API rule) — a headless test hovers by naming
    /// this op.
    Hover(Option<Hovered>),
    /// Delete a recipe node **and every node downstream of it**, as
    /// one action and one undo.
    ///
    /// The panel's own door to `DocEdit::DeleteNode`, and the edit the
    /// survival semantics are exercised through: deleting the feature
    /// a selected face belongs to is the first of the three ways a
    /// selection's referent goes away.
    ///
    /// The cost is knowable before the click:
    /// [`super::DocSession::delete_affordance`] carries the same list the
    /// operation deletes. What it removes is the dependent CONE — a delete that
    /// reconnected consumers to the target's input instead (splice) is
    /// open as issue #1324.
    DeleteNode {
        /// The node to delete, with its dependents.
        node: RecipeNodeId,
    },
    /// Remove one constraint from its placement's bundle
    /// (`DocEdit::RemoveConstraint`): the bundle written whole without
    /// it, by the rule a mate's addition writes it — a mate taken away
    /// frees what it fixed, and each freedom no value then sets is
    /// written the constant `0`, so the copy stays pinned (D10
    /// `[placement.values]`). A constraint is no node, so it has no cone
    /// to delete: the copy it pinned is recomputed from what remains. A
    /// constraint the bundle does not hold is refused as
    /// `DocEdit::RemoveConstraint` refuses it.
    RemoveConstraint {
        /// The constraint: its placement and minted id.
        mate: MateRef,
    },
    /// Give `placement`'s copy a space of its own: a `new_space()`
    /// inserted, the placement re-targeted into it and its bundle
    /// written as the identity (`editor_core::stand`), as one action.
    /// The copy leaves the space it was placed into — the world's, the
    /// product's — and is drawn as a shown space of its own (DI5), which
    /// free-move moves and a mate places again.
    IntoOwnSpace {
        /// The placement: a `Place` or an instance.
        placement: RecipeNodeId,
    },
    /// Write a value into a node's slot. Refused if the slot is
    /// driven by an expression.
    SetSlot {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
        /// The new value.
        value: SlotValue,
    },
    /// Take a locally-valid-range probe for one field: how far it can
    /// move, in each direction, before the document acquires a failure
    /// it does not have now.
    ///
    /// **Explicitly asked for, never automatic.** The probe costs tens
    /// of evaluations (`bounds`'s cost note), which is a price worth
    /// paying when a user is about to change a number and not worth
    /// paying on every selection. The answer lands in the session
    /// ([`super::DocSession::bounds`]) and is discarded on the next document
    /// change.
    ///
    /// It changes no document, commits nothing and enters no history:
    /// every candidate is applied to a scratch copy that is dropped.
    ///
    /// A slot driven by an expression is refused, exactly as
    /// [`SessionOp::SetSlot`] and [`SessionOp::BeginGesture`] refuse
    /// it: the range would be a range of numbers for a field that
    /// takes no number. The affordance names the driving variables,
    /// which are the fields to probe instead.
    ProbeBounds {
        /// The field to probe.
        target: BoundsTarget,
    },
    /// **Stage a creating operation for its verdict**, committing
    /// nothing ([`crate::session::DocSession::staged_for`]): the run its
    /// door would record, evaluated beside the picture. A draft's
    /// question, not an action: it neither edits nor counts as one
    /// ([`crate::frame::acts`]).
    Stage {
        /// The operation staged.
        creation: Creation,
    },
    /// Change how a slot's literal is WRITTEN — its display unit —
    /// leaving the canonical value bit-identical.
    ///
    /// A separate door from [`SessionOp::SetSlot`] because the value
    /// and its notation are independent facts about a literal (D7 keeps
    /// the display unit out of expression identity entirely), and an
    /// operation that moved both could not move either alone.
    ///
    /// There is no "remember no unit" spelling, because there is no
    /// such state: every literal names its notation, and the canonical
    /// one is named by naming it (`m`, `rad`, or the dimensionless
    /// row).
    ///
    /// It is a document edit and enters the history like any other:
    /// the unit is stored in the document and persists, so changing it
    /// is a change to the document, not to the picture.
    SetSlotUnit {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
        /// The unit to write it in.
        unit: UnitDef,
    },
    /// Replace a slot's expression from source text, through the
    /// shipped `parse_formula` door. This is the affordance's editing
    /// half: a driven slot refuses a NUMBER, never an expression.
    SetSlotExpression {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
        /// The expression source.
        text: String,
    },
    /// **Accept a typed value's offer** (`DocSession::offered`): the
    /// slot reads the offered variable, by one `SetParam` (or
    /// `SetStructuralParam`) whose formula is the variable alone, so
    /// two slots share one variable from then on. An unnamed variable
    /// has one reader (VR2), so accepting one names it: `name` is
    /// written first, by a `RenameVar`, and the two edits are one
    /// action and one undo step. A variable not on offer at that slot
    /// is refused (`Refusal::NotOffered`), and a name for one that
    /// already has a name (`Refusal::OfferIsNamed`): accepting renames
    /// nothing.
    SetSlotVariable {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
        /// The variable it reads from now on.
        var: VarId,
        /// The name the variable is shared under, for one that has none.
        name: Option<VarName>,
    },
    /// **Decline a typed value's offer**: the slot keeps the variable
    /// its typed value minted, distinct from every variable offered
    /// (D10: "declining the offer is what makes the two distinct").
    /// Changes no document and enters no history.
    DeclineOffer {
        /// The node.
        node: RecipeNodeId,
        /// The slot whose offer is declined.
        slot: SlotId,
    },
    /// Write a value into a document variable.
    SetVariable {
        /// The variable.
        var: VarId,
        /// The new value.
        value: SlotValue,
    },
    /// Change how a document variable's value is WRITTEN — its
    /// display unit — leaving the exact value alone.
    ///
    /// [`SessionOp::SetSlotUnit`]'s counterpart at the other kind of
    /// row, and a separate door from [`SessionOp::SetVariable`] for that
    /// one's reason: a variable's value and its notation are
    /// independent facts about the declaration, and an operation that
    /// moved both could not move either alone.
    ///
    /// There is no "remember no unit" spelling here either: a
    /// continuous declaration always names its notation, and the
    /// canonical one is named by naming it. A `Count` names none and
    /// refuses.
    SetVariableUnit {
        /// The variable.
        var: VarId,
        /// The unit to write it in.
        unit: UnitDef,
    },
    /// Write a document variable from the text a person typed into
    /// its value field — a number and, optionally, the notation to
    /// write it in.
    ///
    /// **The unit-bearing half of the panel's value field**, and the
    /// door that reads `50 mm`. A bare number needs no parse and takes
    /// [`SessionOp::SetVariable`] instead, the way a slot's bare number
    /// takes [`SessionOp::SetSlot`].
    ///
    /// The text is read by `editor_core::parse::parse_formula`, the one
    /// parser of a unit-bearing number this workspace has, and the
    /// value and its notation are committed as ONE action — one user
    /// action is one undo, and a document that took the value without
    /// the notation would be a half-applied edit nobody asked for.
    ///
    /// **Text that parses to anything but a number defines the
    /// variable** (`DocEdit::DefineVar`): `base_r * 2` makes it a
    /// defined variable, keeping its identity, and a number typed over
    /// a defined one makes it free again. Text that does not parse
    /// carries `parse_formula`'s own refusal, which names the token and
    /// its offset; a definition the door refuses (a cycle, a read the
    /// document does not answer) carries the door's.
    SetVariableText {
        /// The variable.
        var: VarId,
        /// What was typed.
        text: String,
    },
    /// Declare a NEW document variable — the panel's create
    /// affordance, committing exactly one `DocEdit::DeclareVar`.
    ///
    /// A taken name is the edit's to refuse
    /// (`EditError::VarNameTaken`, naming the holder), forwarded
    /// through [`Refusal::Edit`]: the declare never replaces, so a
    /// plus-shaped button cannot change a standing variable's kind
    /// under the expressions that read it. Writing a standing
    /// variable is [`SessionOp::SetVariable`]'s door.
    DeclareVar {
        /// The new variable's name.
        name: VarName,
        /// Its declared dimension and exact value.
        value: FreeVar,
    },
    /// Rename a document variable, or clear its name — exactly one
    /// `DocEdit::RenameVar`.
    ///
    /// Only the name moves: the variable's id, and so every reader of
    /// it and every row keyed by it, stays. The refusals (an id the
    /// document does not hold, a taken name, the name it already has,
    /// clearing the name of a variable nothing reads) are the edit's,
    /// forwarded through [`Refusal::Edit`].
    RenameVar {
        /// The variable.
        var: VarId,
        /// Its new name, or `None` to clear it.
        name: Option<VarName>,
    },
    /// Delete a document variable — exactly one `DocEdit::DeleteVar`.
    ///
    /// Its readers are left reading an id the document no longer
    /// holds, and fail typed at evaluation. The refusals (an id the
    /// document does not hold, an anonymous variable) are the edit's,
    /// forwarded through [`Refusal::Edit`].
    DeleteVar {
        /// The variable.
        var: VarId,
    },
    /// Start a continuous gesture over a slot.
    BeginGesture {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
    },
    /// Start a continuous gesture over a DOCUMENT VARIABLE.
    ///
    /// The same preview/commit machinery as [`SessionOp::BeginGesture`]
    /// and deliberately a separate door rather than a widened one: the
    /// two targets are addressed differently (a node and a slot; a
    /// name) and collapsing them would put an `Option` in every arm.
    /// A variable is where the expression-driven affordance sends a
    /// user, so it is a dragged widget on a primary path and gets the
    /// gesture rule the ratified preview-vs-commit decision (G1) demands.
    BeginVariableGesture {
        /// The variable.
        var: VarId,
    },
    /// Move the in-flight SLOT gesture. Emits a preview edit against
    /// scratch state; commits nothing.
    ///
    /// **It names the slot it is dragging**, and is refused
    /// [`Refusal::WrongGesture`] when that is not the slot the open
    /// gesture was begun on. The subject of a driving operation is the
    /// field the user has hold of, never whichever gesture happens to
    /// be open: a second drag's [`SessionOp::BeginGesture`] is refused
    /// [`Refusal::GestureInFlight`], and this payload is what refuses
    /// its preview and its commit with it, so a drag that could not
    /// open cannot steer the one that did.
    PreviewGesture {
        /// The node whose slot the gesture is dragging.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
        /// The value under the pointer.
        value: f64,
    },
    /// Release: commit exactly one edit carrying the slot gesture's
    /// last previewed value. Names its slot for
    /// [`SessionOp::PreviewGesture`]'s reason, and refuses the same
    /// way — the commit is the half of the pair that reaches the
    /// document.
    CommitGesture {
        /// The node whose slot the gesture is dragging.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
    },
    /// [`SessionOp::PreviewGesture`] for a DOCUMENT VARIABLE drag —
    /// the gesture [`SessionOp::BeginVariableGesture`] opens.
    ///
    /// A second door rather than one preview naming either target, for
    /// [`SessionOp::BeginVariableGesture`]'s own reason: the two targets
    /// are addressed differently, and an operation that names its
    /// gesture names it the way its begin did — one spelling per
    /// target, so a caller repeats the words it already wrote rather
    /// than translating them into a second vocabulary.
    PreviewVariableGesture {
        /// The variable the gesture is dragging.
        var: VarId,
        /// The value under the pointer.
        value: f64,
    },
    /// Release: commit exactly one edit carrying the variable
    /// gesture's last previewed value.
    CommitVariableGesture {
        /// The variable the gesture is dragging.
        var: VarId,
    },
    /// Abandon whichever value gesture is open, leaving the document
    /// untouched.
    ///
    /// **The one value-gesture operation that names no target, and
    /// deliberately.** It is the chrome's cancel door
    /// ([`super::DocSession::cancel_doors`]), whose subject is the
    /// session's state rather than a field: a drag whose field is no
    /// longer drawn is exactly what it exists to close, so a target it
    /// had to name would be one the caller can no longer read off the
    /// panel.
    ///
    /// The field emits it too, when `egui` aborts the drag on Escape
    /// ([`crate::widgets::drag_gesture_ops`]), and that emitter DOES
    /// know its field — it is the widget the drag was on. It names it
    /// no more than the door does, because the target would have to be
    /// an `Option` for the emitter that cannot have one, and an
    /// operation that names its gesture only sometimes cannot be
    /// refused for naming the wrong one.
    CancelGesture,
    /// Step the cursor toward the root.
    Undo,
    /// Step the cursor along the current branch.
    Redo,
    /// Cancel the evaluation in flight.
    CancelEvaluation,
    /// Ask for the current document to be evaluated again.
    ///
    /// The pair to [`SessionOp::CancelEvaluation`], and the only way
    /// back from a cancel: a canceled run leaves the picture older than
    /// the document with nothing running, and every other route to a
    /// re-submit goes through an edit or an undo, i.e. through changing
    /// the document to get it re-drawn. The memo makes the re-run of an
    /// unchanged document nearly free.
    Reevaluate,
    /// Replace the session's document with a file's.
    Open(PathBuf),
    /// Write the committed document at a path, identity kept (DI4):
    /// through the store of the folder the file lands in, with no
    /// edit log (GQ3). Save and Save As… both push this.
    ///
    /// That folder's scan decides the act before anything is written:
    /// a resave, a create at the chosen name, or a refusal. A folder
    /// that already holds this document under another name refuses
    /// `SaveWouldDuplicateId` ([`Refusal::save_choice`] reads the two
    /// ways on out of it), and a folder whose scan refuses gives that
    /// refusal; both arrive as [`Refusal::Workspace`].
    ///
    /// **A save-as MOVES the document seam**: the directory rule
    /// follows the file, so an assembly saved into a directory that
    /// does not hold its parts silently rebinds to that directory and
    /// its instances refuse at the next evaluation — typed on the tree
    /// badges, and recoverable by saving back, but nothing warns
    /// first. Recorded with the rest of the seam's newly-reachable
    /// edges in issue #1387.
    Save(PathBuf),
    /// **Save as a new document** (DI4): the committed document under
    /// a fresh random id, written at the path through that folder's
    /// store, and the session's whole history re-identified under the
    /// new id as one operation, so no undo returns to the original's
    /// identity. The original's file is untouched, and inbound
    /// `DocRef`s keep naming it. Like [`SessionOp::Save`], the session
    /// is then backed by the path and its resolver follows the folder.
    SaveAsNewDocument(PathBuf),
    /// Hide or show the body `node` builds (G3): a DISPLAY operation —
    /// the scene and the pick index drop what draws it; the document
    /// and the feature tree keep it, and nothing is persisted
    /// ([`crate::display::DisplayState::set_body_hidden`]).
    SetBodyHidden {
        /// The node building the body.
        node: RecipeNodeId,
        /// Hidden, or shown again.
        hidden: bool,
    },
    /// Hide or show a space, every body drawn in it: a DISPLAY operation
    /// as [`SessionOp::SetBodyHidden`] is
    /// ([`crate::display::DisplayState::set_space_hidden`]).
    SetSpaceHidden {
        /// The space.
        space: VarId,
        /// Hidden, or shown again.
        hidden: bool,
    },
    /// Open a free-move probe gesture on the space `node`'s body is in
    /// (G3's fit probe, DI5). Refused typed for a body in the world, and
    /// while no evaluation that answers the document has landed — a
    /// body with no value has no picture to move
    /// ([`crate::display::free_move_check`]).
    BeginFreeMove {
        /// The node whose drawn body the probe is opened on.
        node: RecipeNodeId,
    },
    /// Stream the probe's display frame. Each preview REPLACES the
    /// last; nothing enters the document.
    ///
    /// **It names the node it is probing**, and is refused
    /// [`crate::display::DisplayFault::WrongFreeMove`] when that is
    /// not the node the open probe was begun on — the value
    /// gesture's rule ([`SessionOp::PreviewGesture`]) on the other
    /// drag, where the identity is one node rather than a target.
    ///
    /// **One node is the whole identity because the panel draws one
    /// probe per node**, which is the subject rule and not a coarser one: a
    /// slot has one drag and is named by a slot. The panel draws the
    /// probe as three millimetre boxes; mapped a triple per box they
    /// are three gestures over one probe, and this payload cannot
    /// separate them — both name the same node, so the second
    /// box's preview overwrites the first's frame and its commit lands
    /// it and CLOSES the probe the pointer is still holding. So the
    /// row is mapped once ([`crate::widgets::vec3_row_ops`]).
    ///
    /// **A second DRIVER naming the open probe is accepted, and that is
    /// what a target buys over a token.** The node's one probe is
    /// driven by whoever names it: a batch whose
    /// [`SessionOp::BeginFreeMove`] is refused still previews and still
    /// commits, into the probe its own payload names. That is the
    /// recovery for a probe whose field stopped being drawn — the
    /// selection moved off the node, which
    /// [`SessionOp::permitted_during_free_move`] allows — and a token
    /// minted per begin would refuse it. The value gesture spends its
    /// identity the same way; `crates/viewer/README.md`'s *A driving
    /// operation names its own gesture* carries both rows.
    PreviewFreeMove {
        /// The node being probed.
        node: RecipeNodeId,
        /// The display frame composed over the bodies drawn in its
        /// space.
        frame: Frame,
    },
    /// Land the probe: the last previewed frame becomes its space's
    /// committed display value. NO history holds it — the plan's undo
    /// note governs document state only. Names its node for
    /// [`SessionOp::PreviewFreeMove`]'s reason.
    CommitFreeMove {
        /// The node being probed.
        node: RecipeNodeId,
    },
    /// Abandon whichever probe is open, restoring the committed
    /// picture. Names no node, for [`SessionOp::CancelGesture`]'s
    /// reason: it is the other cancel door, and the other thing an
    /// Escape on the field emits.
    CancelFreeMove,
    /// Add the mate to `placement`'s bundle (`DocEdit::AddConstraint`,
    /// which writes the bundle whole: the values whose freedoms the
    /// mate fixes go, each freedom it leaves keeps its value or is the
    /// constant `0`, and a mate beside mates that already fix its
    /// equations is refused overconstrained), then the values written
    /// over the freedoms they set (`DocEdit::SetConstraints`). A mate
    /// whose `to` reads
    /// outside the placement's target space is refused by the door,
    /// naming both spaces, unless the author asked to move the copy
    /// there (`into_target_space`): the gesture never moves a copy into
    /// another space as a side effect. The mate
    /// tool's one committed action, so one history state and one undo.
    /// A bundle the door refuses ends the action and nothing of it is
    /// recorded.
    /// Everything before it (the two picks, the choice) is tool state.
    AddMate {
        /// The placement whose copy the mate places.
        placement: RecipeNodeId,
        /// The mate, `on` read off that copy.
        mate: Box<Mate<Formula>>,
        /// The freedoms it leaves that are set, each to its number.
        values: Vec<(Coord, Formula)>,
        /// Whether the author asked, by name, to move the copy into the
        /// space the mate's `to` reads where that is not its target:
        /// the placement is first stood by the identity bundle, then
        /// re-targeted there, in the same action. `false` refuses such a
        /// mate at the door, naming both spaces.
        into_target_space: bool,
    },
    /// Replace the session's document with a fresh empty one (GAUTH-1's
    /// creation door zero) — the ONE creation op that is a
    /// session-state replacement rather than an `InsertNode`, because
    /// before it runs there is no document to insert into.
    ///
    /// The id is a fresh random one
    /// ([`pncad::workspace::random_document_id`]), never derived from
    /// the name, so two documents created under one name are two parts
    /// to every store. A blank name refuses [`Refusal::EmptyName`];
    /// an OS entropy source that refuses answers
    /// [`Refusal::Workspace`] carrying
    /// `WorkspaceError::RandomnessUnavailable`. Either refusal leaves
    /// the session as it was.
    ///
    /// Everything session-scoped is cleared: path, history, selection,
    /// hover, display state and the resolver (a fresh document has no
    /// backing file, so its instantiate nodes refuse with the shipped
    /// no-resolver semantics until it is saved). Refused mid-gesture
    /// by the table ([`SessionOp::permitted_during_value_gesture`]).
    NewDocument {
        /// The name typed at the New form. It decides no identity, and
        /// nothing past this door keeps it yet
        /// (`work/vseam/a-new-documents-typed-name-is-kept-nowhere`).
        name: String,
    },
    /// Insert one profile node from template loops, drawn on its own
    /// plane — the add-profile tool's one committed edit.
    ///
    /// `loops` is DESCRIPTION order, nothing more: which loop is the
    /// outer boundary and which are holes is decided by the profile
    /// layer's containment forest at replay
    /// (`profile::structure`), never by list position — a list
    /// written holes-first describes the same profile. Every refusal
    /// is the edit door's own: an empty list, a degenerate loop and a
    /// non-nested pair all refuse through the authoring-time check
    /// ([`Refusal::Edit`]), one rule for authored and hand-written
    /// programs alike (only a non-finite field refuses earlier, at
    /// the literal door). A profile reads no frame: where it stands is
    /// the placement of what is built from it.
    AddProfile {
        /// The loop programs, in description order.
        loops: Vec<LoopProgram<Formula>>,
    },
    /// **"Sketch on this face"** (spec §5): `loops` drawn as a profile
    /// on its own plane, extruded `distance` up its own `z`, the
    /// extrude's copy placed against the picked face — a `Plane` mate
    /// from its start cap, flipped, to the face's plane, its origin
    /// over the face body's own and turned `spin` about the face's
    /// normal — and the face's body and the copy fused, as ONE action
    /// (`Recording::sketch_on_face`), the face body's world placements
    /// re-pointed to the union (A10, Ev's residue 1).
    ///
    /// A pick on a copy is read on the body it places. A face whose
    /// node is no body refuses [`Refusal::WrongNodeKind`]; every other
    /// refusal is the edit door's ([`Refusal::Edit`]), and a face that
    /// is not planar or does not resolve refuses at evaluation, on the
    /// placement's badge.
    SketchOnFace {
        /// The picked face.
        face: FaceSelection,
        /// The loop programs, in description order.
        loops: Vec<LoopProgram<Formula>>,
        /// The extrusion depth (`Length`).
        distance: Formula,
        /// The turn about the face's normal (`Angle`).
        spin: Formula,
    },
    /// **A profile drawn on a frame, continuing tangent to a face's
    /// trace** (D10, "Constructing a coincidence"): `chains` drawn on the
    /// frame `Through` the first traced face's own axis and a vertex of
    /// `plane`, a plane face square to that axis — its origin the
    /// vertex's foot on the axis — tracing `traced`, by place
    /// ([`pncad::trace::traced_frame`], [`pncad::trace::drawn_profile`]).
    /// One `InsertNode`, in the frame's space.
    ///
    /// What is proven: a sweep up the frame's `z` of a chain that leaves
    /// a traced wall's trace along its tangent (`OnTrace`, then
    /// `AlongTrace`) is tangent to that wall by construction, and the
    /// coincidence door reports the union's row `SameConstruction`.
    ///
    /// A traced face with no axis, or a picked face that is no plane
    /// square to it, refuses [`Refusal::Frame`]; an argument of the
    /// wrong dimension [`Refusal::Edit`]. A crossing the piece does not
    /// make builds, and refuses at evaluation on the profile's badge.
    AddDrawnProfile {
        /// The plane face the frame's origin is stood in.
        plane: FaceSelection,
        /// The faces the profile traces, by place.
        traced: Vec<FaceSelection>,
        /// The loops, outer first, in the document's vocabulary.
        chains: Vec<pncad::trace::Chain>,
    },
    /// **Write the path editor's program over a committed profile's**
    /// — the door the add-profile form's editor commits through when
    /// it is opened on an existing node instead of on nothing. A
    /// `node` that is not a `Node::Profile` refuses
    /// [`Refusal::WrongNodeKind`] at the door.
    ///
    /// `loops` is the whole program as the editor holds it, lowered in
    /// the form's notation, and `ids` says which committed step each
    /// of its steps is: per loop, per step, the committed step's
    /// [`StepId`] it keeps, or `None` for a step the editor made. What
    /// reaches the history is ONE `DocEdit::SetProgram` — one action,
    /// one undo — whatever moved: numbers, steps inserted, removed or
    /// reordered, verbs, arc modes, targets, a split circle's count.
    /// Nothing at all reaches it when the program is the committed one
    /// with every step kept in place, which is what makes opening a
    /// profile and applying it untouched cost no history entry.
    ///
    /// Every argument of a kept step that did not move is written as
    /// the document holds it, so only what moved takes the editor's
    /// notation. A kept step's argument an expression drives refuses
    /// with the affordance if the program does not hold it unmoved
    /// ([`Refusal::DrivenByExpression`]), exactly as the slot field
    /// does. The program itself, and `ids`' shape, are the edit door's
    /// to judge, and refuse in its words ([`Refusal::Edit`]).
    ///
    /// A name on a step the program does not keep, or on a kept step's
    /// piece it stops drawing, is stranded, and the door's report of it
    /// rides [`OpOutcome::maintenance`];
    /// [`crate::session::DocSession::edit_profile_report`] reads the
    /// same rows before the op is performed.
    ///
    /// `base` is the program the editor was loaded from. A document
    /// whose program is no longer that one (compared by value) refuses
    /// [`Refusal::ProfileEditStale`]: the editor's program was an edit
    /// of one that is not there any more.
    EditProfile {
        /// The profile node.
        node: RecipeNodeId,
        /// The committed program the editor's program was loaded
        /// from, as written ([`crate::sketch::written_program`]).
        base: ProfileProgram<Formula>,
        /// The loop programs the editor holds, in description order.
        loops: Vec<LoopProgram<Formula>>,
        /// Per loop, per step: the committed step it keeps, or `None`
        /// for a new one (`DocEdit::SetProgram`'s `ids`).
        ids: Vec<Vec<Option<StepId>>>,
    },
    /// Insert one extrude of an existing profile node, and its identity
    /// world placement (A10) — the extrude tool's one committed action,
    /// so what was made is drawn. A `profile` that is not a
    /// `Node::Profile` in this document refuses
    /// [`Refusal::WrongNodeKind`] at the door.
    ///
    /// The extrude goes up the profile's own `+z`. The distance is a
    /// depth: the door inserts what it is given, and a negative one
    /// refuses at evaluation.
    AddExtrude {
        /// The profile node extruded.
        profile: RecipeNodeId,
        /// The extrusion depth (`Length`).
        distance: Formula,
    },
    /// Insert one revolve of an existing profile node about an axis
    /// line in the profile's own plane, and its identity world
    /// placement (A10) — the revolve tool's one committed action.
    /// A wrong-kind profile pick refuses [`Refusal::WrongNodeKind`] at
    /// the door.
    AddRevolve {
        /// The profile node revolved.
        profile: RecipeNodeId,
        /// A point on the axis, in the profile's 2-D coordinates
        /// (`Length`).
        axis_through: [Formula; 2],
        /// The axis's heading from the profile's x axis (`Angle`).
        axis_heading: Formula,
        /// The sweep angle (`Angle`); the chrome's default is a full
        /// turn.
        angle: Formula,
    },
    /// Insert one boolean of existing bodies — the boolean tool's one
    /// committed action (GAUTH-4).
    ///
    /// **The spec is the node's shape** ([`super::BooleanSpec`]): a
    /// union or an intersect of a list of members, any count, or a
    /// subtraction of exactly two operands whose order is data — `from`
    /// is kept and `tool` removed. Any operand's non-body pick refuses
    /// [`Refusal::WrongNodeKind`] at this layer. A node listed twice is
    /// not refused: a repeated read glues, so it is a document like any
    /// other.
    ///
    /// **The result takes the operands' place in the world** (A10, Ev's
    /// residue 1): the placements of the first operand the world
    /// places, in the spec's order, are re-pointed to the boolean
    /// through the slot door, and every other operand's placements are
    /// deleted, in the same action — so a combine of placed bodies
    /// leaves the world `[result]`, and one undo restores them. A
    /// repeated operand counts once; operands nothing places place
    /// nothing.
    ///
    /// **Operands in two spaces are related by a placement** (D10): each
    /// operand after the first whose value lives in another space than
    /// the first's is placed into the first's, in the same action. A
    /// construction is placed at the identity and the boolean reads the
    /// new placement; a copy placed into its own body's space
    /// ([`Self::AddTransform`]'s) is re-targeted in place, its bundle
    /// the values on the first's space's axes that stand it at the pose
    /// it held relative to its body. An operand whose pose does not
    /// solve is read as it is, and the boolean refuses `SpaceMismatch`
    /// at evaluation, on its own badge.
    AddBoolean {
        /// The operation and its operands.
        spec: super::BooleanSpec,
    },
    /// Insert one split of an existing body by a plane — the split
    /// tool's one committed edit. A split defines two bodies, its
    /// `above` and `below` outputs, and it changes nothing in the
    /// world: the target's placements keep reading the target, and a
    /// half is read by its port wherever a body seat takes one.
    ///
    /// The plane is the one through `origin` with normal `normal` in
    /// the target's own coordinates, read off the target's geometry
    /// (`Node::Split`'s tool is a plane read whose reads reach its
    /// operand's root alone) at the numbers' nominal values in the
    /// document. A target seat of the wrong kind refuses
    /// [`Refusal::WrongNodeKind`]; a number of the wrong dimension, one
    /// that does not evaluate, a normal that names no direction and a
    /// target with no geometry to read the plane off refuse
    /// [`Refusal::Frame`].
    AddSplit {
        /// The body split.
        target: Operand,
        /// A point on the plane (`Length`).
        origin: [Formula; 3],
        /// The plane's normal (`Scalar`).
        normal: [Formula; 3],
    },
    /// **Mirror an existing body across the plane of a picked face** —
    /// the mirror tool's one committed action. The plane is the face's
    /// own, read where it was picked: a pick on a copy is read on the
    /// body the copy places (`crate::world::body_of`), so the plane reads
    /// the geometry of the body's space, as a mirror's plane must.
    ///
    /// A body the world draws becomes the symmetric part: the image
    /// (`Node::Mirror`) and the union of the body and its image, in the
    /// body's space, the body's drawn copies re-pointed to the union —
    /// the feature takeover a fillet makes — so the part is drawn where
    /// each copy stands and moves with the copy's placement. A mate that
    /// another placement seats on a face of the body's copy goes stale
    /// at the takeover, as it does at a fillet's. A body nothing places
    /// gets its image alone, reached in the tree.
    ///
    /// A body seat of the wrong kind refuses [`Refusal::WrongNodeKind`];
    /// a face read in another space — another construction's, or its
    /// image's — refuses at the insert door (`EditError::SpaceMismatch`).
    /// Whether the face is planar is the mirror's own refusal where it
    /// evaluates.
    AddMirror {
        /// The body reflected.
        body: Operand,
        /// The face whose plane it is reflected across.
        plane: FaceSelection,
    },
    /// **Move an existing body rigidly** — the transform tool's one
    /// committed action: a placement of the body into the space it is
    /// in (`Node::Place`, no mate), its values on that space's axes —
    /// rotated by `rotation_angle` about `rotation_axis` through the
    /// body's own origin, then translated — and the body's world
    /// placements re-pointed to it (A10, Ev's residue 1), as one action.
    ///
    /// The translation is written as the three slides, each formula as
    /// given; a rotation about one of the space's axes is the spin about
    /// it by `rotation_angle` as given, and one about any other axis is
    /// the chain of turns its frame makes at the numbers' NOMINAL
    /// values. What the placement holds afterwards is its values,
    /// editable through their slots. A number of the wrong dimension,
    /// one that does not evaluate, and an axis that names no direction
    /// refuse [`Refusal::Frame`]; an input that is no placeable body
    /// refuses as a body seat does.
    AddTransform {
        /// The body placed.
        input: Operand,
        /// Translation components (`Length`).
        translation: [Formula; 3],
        /// Rotation-axis components (`Scalar`).
        rotation_axis: [Formula; 3],
        /// Rotation angle (`Angle`).
        rotation_angle: Formula,
    },
    /// Insert one pattern of an existing body — the pattern tool's one
    /// committed edit: an index `k = index(count)` and one placement of
    /// the body in its own space whose values read it (D10
    /// `[repetition]`), a family of `count` copies. The family is
    /// placed where the body is; each copy reaches a seat whole or
    /// through an indexed read (`SessionOp::AddPart`).
    ///
    /// The count is an `i64` and lands as the index's exact Count
    /// literal, edited afterwards by redefining the index. A count of
    /// zero or less is admitted here and refuses at evaluation on the
    /// placement's own badge — the same division of labour a
    /// degenerate profile takes, one rule for authored and
    /// hand-written documents.
    AddPattern {
        /// The body replicated.
        input: Operand,
        /// Copy count.
        count: i64,
        /// The replication rule; a circular rule's axis is read off
        /// the input's own geometry at its numbers' nominal values, and
        /// refuses [`Refusal::Frame`] as [`Self::AddSplit`]'s plane does.
        rule: PatternRuleSpec,
    },
    /// Insert one FUSED pattern of an existing body — the pattern
    /// tool's other committed edit, and the door a boolean's refusal
    /// of a pattern points at.
    ///
    /// [`SessionOp::AddPattern`]'s twin, seat for seat and slot for
    /// slot: the same index and placement, and a `Union` reading the
    /// family whole — ONE body, the union of the prototype at every
    /// placement.
    ///
    /// **The prototype, not a family.** This door's seat wants a BODY,
    /// as the unfused door's does. A user who already authored the
    /// unfused pattern fuses it with a boolean over the family instead.
    ///
    /// **The fused body takes the prototype's place in the world** (A10,
    /// Ev's residue 1): the prototype's placements are re-pointed to it
    /// in the same action.
    ///
    /// **Whether the copies meet is the union's question, not this
    /// door's**: the door authors what the user asked for and the union
    /// says what it builds.
    AddFusedPattern {
        /// The prototype each member copies.
        input: Operand,
        /// The index's count.
        count: i64,
        /// How each member is placed, with the axis a circular one was
        /// picked with.
        rule: PatternRuleSpec,
    },
    /// Insert one constant-radius fillet on a SET of an existing
    /// body's edges — the blend tool's one committed action, as a
    /// fillet, re-pointing the body's world placements to it (A10, Ev's
    /// residue 1).
    ///
    /// **A target that is a world placement is read on the body it
    /// places**: the fillet reads that body, and a copy carries its
    /// body's names as they are, so each selected name is the body's.
    ///
    /// **The selection is a frozen commitment** (`Node::Fillet`'s
    /// ratified #217 semantics): what is authored here is what the
    /// node keeps, and `Node::fillet` canonicalizes it (sorted,
    /// deduplicated) so two recipes selecting the same edges are
    /// bit-identical.
    ///
    /// **The names are NOT checked against the evaluation here**, and
    /// that is the freeze rule rather than an omission. Whether a name
    /// still resolves through the target's table is evaluation's
    /// question, answered typed on the node's own badge
    /// (`NodeErrorKind::SelectResolve`) — a door that pre-screened it
    /// would be a second authority on the same fact, and would refuse
    /// to author the node whose refusal is the honest thing to show.
    /// What the names say as written — how many, of what kind, in
    /// what order — is the edit door's (`Select::fault`: a set of edges
    /// holds at least one), and the target's KIND is a fact about the
    /// committed document alone, so both refuse here
    /// ([`Refusal::WrongNodeKind`], and the insert's own, forwarded
    /// through [`Refusal::Edit`]).
    AddFillet {
        /// The body whose edges are blended.
        target: RecipeNodeId,
        /// The blend radius (`Length`).
        radius: Formula,
        /// The edges to blend, by stable name.
        selection: Vec<StableName>,
    },
    /// Insert one equal-setback chamfer on a SET of an existing body's
    /// edges — [`SessionOp::AddFillet`]'s twin, and the blend tool's
    /// other committed edit.
    ///
    /// A separate op for the reason `Node::Chamfer` is a separate
    /// variant: the size means a SETBACK along both supports rather
    /// than a rolling ball's radius, it lands in a different slot
    /// (`SlotId::ChamferDistance`), and a recipe whose size changed
    /// meaning on a boolean's value would be a document a reader can
    /// misread. Everything [`SessionOp::AddFillet`] says about the
    /// selection holds here unaltered.
    AddChamfer {
        /// The body whose edges are chamfered.
        target: RecipeNodeId,
        /// The setback along both supports (`Length`).
        distance: Formula,
        /// The edges to chamfer, by stable name.
        selection: Vec<StableName>,
    },
    /// Insert one MEMBER of a family as a body of its own: a placement
    /// standing in the family's space whose read is the family indexed
    /// at `select` (`BodyRead::indexed`), drawn as a creation is.
    ///
    /// **`select` is an authoring spec**: an `i64`, where the read
    /// carries a `Count` formula. A member index is authored exact and
    /// edited afterwards through the variable doors. The seat must read
    /// a family, so a node that is none refuses
    /// [`Refusal::WrongNodeKind`] here rather than landing and failing at
    /// evaluation, the rule [`SessionOp::AddFillet`] states for its
    /// target's kind. What is left to evaluation is the index being IN
    /// RANGE, a fact about the family's value:
    /// `NodeErrorKind::MemberOutOfRange`, typed, on the node's own
    /// badge.
    AddPart {
        /// The split or pattern whose value is read.
        of: RecipeNodeId,
        /// Which body of it.
        select: PartSelectSpec,
    },
    /// **Duplicate one body**: a copy stepped clear of the picked body,
    /// drawn and editable on its own.
    ///
    /// **A duplicate is one stepped copy**: a placement of the body in
    /// its own space, slid clear by the measured step, placed beside the
    /// original as it is drawn (A10). One action, one undo.
    /// The original's placements are untouched. The copy is a body of its
    /// own rather than a second placement of the original, because a
    /// feature gesture re-points every placement of its target body
    /// (`work/placing/a-feature-gesture-re-points-every-copy-of-its-target.md`).
    ///
    /// **The step is measured, not fixed**
    /// ([`crate::combine::duplicate_step`]): along
    /// [`crate::combine::STEP_DIRECTION`], far enough that the copy
    /// clears the original by at least [`crate::combine::DUPLICATE_GAP`]
    /// of the body's own width. It is read off the LANDED value of the
    /// CURRENT document, so this door refuses ([`Refusal::Duplicate`])
    /// before anything has landed, while an edit has not landed yet,
    /// and for an input whose value is several bodies — which the body
    /// seat's node-kind gate admits for a transform of a pattern, and
    /// which a pattern of two would index in place, adding nothing.
    /// Both numbers land in the pattern's ordinary slots and are edited
    /// in the property panel afterwards.
    ///
    /// **Where the copy is drawn**: beside the drawn copy `beside`, or
    /// beside each drawn copy of `input` when the pick named the body
    /// itself; at the identity of the world for a body nothing draws.
    /// Each placement is written by values on every freedom at that
    /// copy's evaluated motion, into the copy's space, with no mate, so
    /// the duplicate stands where the person sees the original, stepped
    /// along the original's own `x`, and does not follow it.
    Duplicate {
        /// The body duplicated.
        input: Operand,
        /// The drawn copy of `input` the pick landed on, if any.
        beside: Option<RecipeNodeId>,
    },
    /// Commit **exactly one** `DocEdit` inserting an instance of
    /// another document — the assembly-authoring door, and the second
    /// insert door after the mate tool's.
    ///
    /// **The pin is minted HERE, not carried in.** The op names only
    /// which part (`id`); the version it pins is the store's content
    /// at the moment of the commit
    /// (`Workspace::current_pin`), so an authored reference always
    /// starts life resolving. From then on A4's Cargo.lock semantics
    /// hold: the pin moves by its own recorded edit and by nothing
    /// else.
    ///
    /// **The directory is the open file's own** — the same one every
    /// reference resolves against ([`crate::docio::DirResolver`]), so
    /// a session with no backing file refuses
    /// ([`Refusal::NoDocumentDirectory`]) rather than authoring a
    /// reference into a store it has not got.
    ///
    /// **A creation, so its copy is placed** (A10): an instance is a
    /// placement, inserted and placed at the identity by the "place
    /// part" gesture (the identity bundle), with its reference and an empty
    /// interface record (an authored instance crosses no split seam).
    /// Hiding, the free-move probe and the mate tool take it from
    /// there.
    AddInstance {
        /// Which document in the open document's own directory.
        id: DocumentId,
    },
    /// **Accept the updated version of the part `id` names**, at every
    /// instance of it: the edits `pncad::workspace::update_to_store`
    /// answers — one `DocEdit::UpdateReference` per site whose pin
    /// moves — committed as one action, so one undo.
    ///
    /// The pin is minted at the commit from the store's content, as
    /// [`SessionOp::AddInstance`]'s is, so an offer drawn before the
    /// part's file changed again accepts the version on disk now. What
    /// the store or the elaboration refuses — no file for the id, or
    /// every reference already on the version the store holds — is
    /// refused in their own words ([`Refusal::Workspace`]).
    AcceptPartVersion {
        /// Which part, by the identity every reference to it carries.
        id: DocumentId,
    },
    /// **Rename a node**: set its label, or clear it with `None`
    /// (DESIGN.md Band 1, "Node labels") — one `DocEdit::SetLabel`,
    /// one undo. A label is document data beside the node, so this
    /// recomputes nothing. Writing the label the node already has is
    /// no action and costs no undo step.
    SetLabel {
        /// The node renamed.
        node: RecipeNodeId,
        /// Its new label, `None` to clear it.
        label: Option<Label>,
    },
    /// **Say what the document makes** (D10 `[operations]`): its
    /// product set to the one body of `bodies`, or to a `⊔` of them in
    /// the order given, inserted in the same action; to nothing where
    /// `bodies` is empty. One action, one undo. The door refuses a read
    /// whose body is not in the world. The properties pane offers it on a
    /// selected world copy: add it to the product, or take it out.
    SetProduct {
        /// The bodies the document makes.
        bodies: Vec<Operand>,
    },
    /// **A creation, labelled**: the creation's own edits and then a
    /// `DocEdit::SetLabel` on the node it made (`world::made`: the last
    /// node it minted that is not a placement), as ONE action and one
    /// undo. The insert carries no label, because what an insert
    /// carries is what its node's id is minted from; the label is the
    /// last edit of the creation's own run.
    CreateLabelled {
        /// The creation.
        creation: Creation,
        /// The label its node is given.
        label: Label,
    },
}

/// **Which VALUE drag an operation names**: the slot or the document
/// variable a [`SessionOp::BeginGesture`] or
/// [`SessionOp::BeginVariableGesture`] opened a gesture on.
///
/// Two arms and no third, because a value gesture's target has two
/// kinds: the session's gesture target mints one of these and nothing
/// else, so the comparison that decides whether a preview belongs to
/// the open gesture is total over what a drag can be on.
///
/// **It mints its own three driving operations**, which is what holds
/// them to one target: a chrome that names the slot once cannot emit
/// a preview for one field and a commit for another.
///
/// The wider question — which of this crate's gestures an operation
/// names at all — is [`GestureName`], of which this is one half.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueGestureName {
    /// A node's slot.
    Slot {
        /// The node whose slot the gesture drags.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
    },
    /// A document variable.
    Variable(VarId),
}

impl ValueGestureName {
    /// The operation that opens this gesture.
    #[must_use]
    pub fn begin(&self) -> SessionOp {
        match self {
            Self::Slot { node, slot } => SessionOp::BeginGesture {
                node: *node,
                slot: *slot,
            },
            Self::Variable(var) => SessionOp::BeginVariableGesture { var: *var },
        }
    }

    /// The operation that moves it, carrying the value under the
    /// pointer.
    #[must_use]
    pub fn preview(&self, value: f64) -> SessionOp {
        match self {
            Self::Slot { node, slot } => SessionOp::PreviewGesture {
                node: *node,
                slot: *slot,
                value,
            },
            Self::Variable(var) => SessionOp::PreviewVariableGesture { var: *var, value },
        }
    }

    /// The operation that lands it.
    #[must_use]
    pub fn commit(&self) -> SessionOp {
        match self {
            Self::Slot { node, slot } => SessionOp::CommitGesture {
                node: *node,
                slot: *slot,
            },
            Self::Variable(var) => SessionOp::CommitVariableGesture { var: *var },
        }
    }
}

/// **Which free-move probe an operation names**: the node it was
/// opened on, which is the whole identity because the panel draws one
/// probe per node
/// ([`SessionOp::PreviewFreeMove`] carries why).
///
/// A type rather than a bare [`RecipeNodeId`] so that the probe's
/// three driving operations are minted from one value, the way
/// [`ValueGestureName`] mints the value drag's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FreeMoveName {
    /// The node being probed.
    pub node: RecipeNodeId,
}

impl FreeMoveName {
    /// The operation that opens this probe.
    #[must_use]
    pub const fn begin(&self) -> SessionOp {
        SessionOp::BeginFreeMove { node: self.node }
    }

    /// The operation that moves it, carrying the whole composed frame
    /// rather than a delta.
    #[must_use]
    pub const fn preview(&self, frame: Frame) -> SessionOp {
        SessionOp::PreviewFreeMove {
            node: self.node,
            frame,
        }
    }

    /// The operation that lands it.
    #[must_use]
    pub const fn commit(&self) -> SessionOp {
        SessionOp::CommitFreeMove { node: self.node }
    }
}

/// **Which gesture an operation names** — one type for every drag in
/// this crate.
///
/// The six driving operations spell their subject three ways, each
/// right for its own door: a node and a slot, a variable id, a
/// probed node. What they are all spellings OF is this. A gesture's name
/// is what a preview and a commit are checked against before they
/// touch anything ([`crate::g1::Slot`]'s two name checks), and
/// [`SessionOp::names_gesture`] is the one place an operation becomes
/// one.
///
/// **The layers below still speak their own subjects.** The session
/// compares a [`ValueGestureName`] and the display state compares a
/// probed node, because each holds only the gesture it can have; this is
/// the name an OPERATION carries, which is the level at which all
/// three are the same kind of fact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GestureName {
    /// A slot or document-variable drag.
    Value(ValueGestureName),
    /// A space's free-move probe, by the node it was opened on.
    FreeMove(FreeMoveName),
}

impl GestureName {
    /// The operation that opens this gesture.
    #[must_use]
    pub fn begin(&self) -> SessionOp {
        match self {
            Self::Value(value) => value.begin(),
            Self::FreeMove(probe) => probe.begin(),
        }
    }

    /// The operation that lands it.
    #[must_use]
    pub fn commit(&self) -> SessionOp {
        match self {
            Self::Value(value) => value.commit(),
            Self::FreeMove(probe) => probe.commit(),
        }
    }

    /// The operation that abandons it.
    ///
    /// **It names no gesture and is still the name's to mint**: the
    /// two cancels are one per DRAG rather than one per gesture
    /// ([`SessionOp::CancelGesture`] carries why a cancel names no
    /// target), so which of the two a control emits is decided by
    /// which drag it is driving and by nothing else.
    #[must_use]
    pub const fn cancel(&self) -> SessionOp {
        match self {
            Self::Value(_) => SessionOp::CancelGesture,
            Self::FreeMove(_) => SessionOp::CancelFreeMove,
        }
    }
}

impl SessionOp {
    /// **Whether this operation inserts a node** — what
    /// [`Creation::of`] admits. Exhaustive with no wildcard arm, so an
    /// operation added tomorrow says whether it creates.
    #[must_use]
    pub fn creates_a_node(&self) -> bool {
        match self {
            Self::AddProfile { .. }
            | Self::SketchOnFace { .. }
            | Self::AddDrawnProfile { .. }
            | Self::AddExtrude { .. }
            | Self::AddRevolve { .. }
            | Self::AddBoolean { .. }
            | Self::AddSplit { .. }
            | Self::AddMirror { .. }
            | Self::AddTransform { .. }
            | Self::AddPattern { .. }
            | Self::AddFusedPattern { .. }
            | Self::AddFillet { .. }
            | Self::AddChamfer { .. }
            | Self::AddPart { .. }
            | Self::Duplicate { .. }
            | Self::AddInstance { .. } => true,
            // A mate is a constraint in a placement's bundle, no node.
            Self::AddMate { .. }
            | Self::Select(_)
            | Self::Hover(_)
            | Self::DeleteNode { .. }
            | Self::RemoveConstraint { .. }
            | Self::IntoOwnSpace { .. }
            | Self::SetSlot { .. }
            | Self::SetSlotVariable { .. }
            | Self::DeclineOffer { .. }
            | Self::ProbeBounds { .. }
            | Self::Stage { .. }
            | Self::SetSlotUnit { .. }
            | Self::SetSlotExpression { .. }
            | Self::SetVariable { .. }
            | Self::SetVariableUnit { .. }
            | Self::SetVariableText { .. }
            | Self::DeclareVar { .. }
            | Self::RenameVar { .. }
            | Self::DeleteVar { .. }
            | Self::BeginGesture { .. }
            | Self::BeginVariableGesture { .. }
            | Self::PreviewGesture { .. }
            | Self::CommitGesture { .. }
            | Self::PreviewVariableGesture { .. }
            | Self::CommitVariableGesture { .. }
            | Self::CancelGesture
            | Self::Undo
            | Self::Redo
            | Self::CancelEvaluation
            | Self::Reevaluate
            | Self::Open(_)
            | Self::Save(_)
            | Self::SaveAsNewDocument(_)
            | Self::SetBodyHidden { .. }
            | Self::SetSpaceHidden { .. }
            | Self::BeginFreeMove { .. }
            | Self::PreviewFreeMove { .. }
            | Self::CommitFreeMove { .. }
            | Self::CancelFreeMove
            | Self::NewDocument { .. }
            | Self::EditProfile { .. }
            | Self::AcceptPartVersion { .. }
            | Self::SetLabel { .. }
            | Self::SetProduct { .. }
            | Self::CreateLabelled { .. } => false,
        }
    }

    /// **Which gesture this operation drives**, or `None` for an
    /// operation that drives none.
    ///
    /// The one place a [`SessionOp`] becomes a [`GestureName`], and
    /// the answer to *are these three spellings the same kind of
    /// fact*: they are, and this is where the tree says so.
    ///
    /// **The two cancels answer `None`, and that is the rule rather
    /// than an omission**: a cancel names no target
    /// ([`SessionOp::CancelGesture`]), so it drives whichever gesture
    /// is open. [`GestureName::cancel`] is the other direction, which
    /// a name can answer because the DRAG it belongs to is decided.
    #[must_use]
    pub fn names_gesture(&self) -> Option<GestureName> {
        let value = |name| Some(GestureName::Value(name));
        let probe = |node| Some(GestureName::FreeMove(FreeMoveName { node }));
        match self {
            Self::BeginGesture { node, slot }
            | Self::PreviewGesture { node, slot, .. }
            | Self::CommitGesture { node, slot } => value(ValueGestureName::Slot {
                node: *node,
                slot: *slot,
            }),
            Self::BeginVariableGesture { var }
            | Self::PreviewVariableGesture { var, .. }
            | Self::CommitVariableGesture { var } => value(ValueGestureName::Variable(*var)),
            Self::BeginFreeMove { node }
            | Self::PreviewFreeMove { node, .. }
            | Self::CommitFreeMove { node } => probe(*node),
            Self::CancelGesture
            | Self::CancelFreeMove
            | Self::Select(_)
            | Self::Hover(_)
            | Self::DeleteNode { .. }
            | Self::RemoveConstraint { .. }
            | Self::IntoOwnSpace { .. }
            | Self::SetSlot { .. }
            | Self::SetSlotVariable { .. }
            | Self::DeclineOffer { .. }
            | Self::ProbeBounds { .. }
            | Self::Stage { .. }
            | Self::SetSlotUnit { .. }
            | Self::SetSlotExpression { .. }
            | Self::SetVariable { .. }
            | Self::SetVariableUnit { .. }
            | Self::SetVariableText { .. }
            | Self::DeclareVar { .. }
            | Self::RenameVar { .. }
            | Self::DeleteVar { .. }
            | Self::Undo
            | Self::Redo
            | Self::CancelEvaluation
            | Self::Reevaluate
            | Self::Open(_)
            | Self::Save(_)
            | Self::SaveAsNewDocument(_)
            | Self::SetBodyHidden { .. }
            | Self::SetSpaceHidden { .. }
            | Self::AddMate { .. }
            | Self::NewDocument { .. }
            | Self::AddProfile { .. }
            | Self::SketchOnFace { .. }
            | Self::AddDrawnProfile { .. }
            | Self::EditProfile { .. }
            | Self::AddExtrude { .. }
            | Self::AddRevolve { .. }
            | Self::AddBoolean { .. }
            | Self::AddSplit { .. }
            | Self::AddMirror { .. }
            | Self::AddTransform { .. }
            | Self::AddPattern { .. }
            | Self::AddFusedPattern { .. }
            | Self::AddFillet { .. }
            | Self::AddChamfer { .. }
            | Self::AddPart { .. }
            | Self::Duplicate { .. }
            | Self::AddInstance { .. }
            | Self::AcceptPartVersion { .. }
            | Self::SetLabel { .. }
            | Self::SetProduct { .. }
            | Self::CreateLabelled { .. } => None,
        }
    }

    /// Whether this operation is permitted while a **value gesture**
    /// is in flight — a slot or document-variable drag opened by
    /// [`SessionOp::BeginGesture`] or [`SessionOp::BeginVariableGesture`],
    /// the only thing [`Refusal::GestureInFlight`] ever speaks about.
    ///
    /// **It is not a statement about the free-move gesture.** That is
    /// a second, independent drag living on the display state
    /// ([`crate::display::DisplayState::begin_free_move`]), with its own in-flight
    /// refusal and its own table
    /// ([`SessionOp::permitted_during_free_move`], which carries why
    /// the two cannot be one). The two can be open at once, and the
    /// four `*FreeMove` rows below are what permits it. An operation
    /// this returns `true` for is permitted mid-value-gesture and
    /// nothing more.
    ///
    /// # Why the two drags may overlap
    ///
    /// DI5 (`crates/editor-core/IDENTITY.md`) keeps all four
    /// `*FreeMove` rows display-only: releasing a free-move writes
    /// nothing to the document, and the frame stays display state keyed
    /// by the space it was drawn for. So
    /// [`SessionOp::CommitFreeMove`] commits nothing to the history, and
    /// the argument below covers it as it covers the other three.
    ///
    /// The two drags own disjoint state. A value gesture owns a
    /// `Doc` snapshot and writes a scratch `Doc`; the free-move
    /// gesture's previews and its committed frames enter no `Doc`, so
    /// a probe committed or discarded mid-drag cannot reach what a
    /// preview is applied to or what a commit records.
    ///
    /// They meet in three places, and that is where the safety
    /// actually lives — all three ask a display predicate about a
    /// document, and they do not all ask about the SAME document:
    ///
    /// - the operation admits against the COMMITTED document and the
    ///   landing that answers it or the scratch
    ///   ([`crate::display::free_move_check`], via
    ///   [`crate::display::DisplayState::begin_free_move`] and
    ///   [`super::DocSession::space_evaluation`]);
    /// - the view the scene draws
    ///   ([`super::DocSession::display_view`]) resolves against the
    ///   PREVIEWED one; the Properties pane reads the COMMITTED one and
    ///   runs the SAME admission test the operation does
    ///   ([`crate::display::free_move_check`], over the same
    ///   [`super::DocSession::space_evaluation`]) to decide whether to
    ///   DRAW the control the operation then decides whether to
    ///   ACCEPT;
    /// - every committed edit prunes the display state against the new
    ///   document, and again when its evaluation lands — a prune that
    ///   DISCARDS committed frames and kills an in-flight free-move
    ///   whose space stopped being shown,
    ///   reporting the first in [`PruneReport::superseded`] and the
    ///   second in [`PruneReport::killed_gesture`], each with the
    ///   fault that decided it, on [`OpOutcome::withdrawn`].
    ///
    /// **The identity that makes all three the same answer**: a value
    /// gesture's edits are `SetParam` and `SetStructuralParam`, which
    /// replace an expression on a node that already exists, and
    /// `SetVarValue`, which writes a definition in `doc.vars`
    /// and touches no node at all. None of the three mints or removes
    /// one. Every display predicate is a function of the node graph —
    /// which nodes exist, of what kind, with which inputs and which
    /// mate references — and never of a slot's expression or a
    /// variable's value. The free-move admission reads its spaces off
    /// an evaluation (DI5), and those are the graph's too: pinning is
    /// structural, which mates name which copies and which name the
    /// world, so a landing of either document answers both. The one
    /// space answer a value can move is a placement the evaluation
    /// refuses ([`crate::display::AdmissionFault::NoSpace`]): admission
    /// reads it off whichever of the two landed last, and a scratch
    /// landing's prune acts on the graph's answers alone
    /// ([`crate::display::DisplayState::prune_structural`]), so a
    /// committed pin a preview overtook still lands mid-drag and a
    /// previewed refusal discards nothing committed. So the previewed
    /// document and the committed one agree on every display question
    /// the graph decides, at every point of a drag; a refused placement
    /// is said when the committed document lands, whatever refused it.
    /// Break that — let a gesture's edit change the graph — and
    /// committing a slider would take an in-flight probe away under
    /// the pointer holding it, saying nothing. The minting half is
    /// held rather than argued, by an assertion on the applied
    /// record in `DocSession::preview_gesture`; the rest is what
    /// `a_value_gesture_and_a_free_move_probe_do_not_disturb_each_other`
    /// pins, dragging both node-writing doors.
    ///
    /// Whether the overlap was ever DECIDED is a separate question and
    /// the record does not answer it: these four rows were carried
    /// forward from a tree where the `*FreeMove` operations were
    /// merely unguarded. What is established is that it is sound
    /// today, on the mechanism above.
    ///
    /// The whole policy is here, so that the set of operations a drag
    /// refuses can be READ rather than reconstructed from the
    /// dispatch: [`super::DocSession::perform`] consults this
    /// once, before dispatch, and no arm re-guards against the VALUE
    /// gesture with a table of its own. Six arms guard from a
    /// GESTURE's state: the `*FreeMove` quartet delegates to
    /// [`crate::display::DisplayState`], which refuses
    /// [`crate::display::DisplayFault::FreeMoveInFlight`] off its own
    /// state, and the two value-gesture begins delegate to
    /// [`crate::g1::Slot::begin`], which refuses
    /// [`Refusal::GestureInFlight`] off this one. That rule is rule 1
    /// and is held once for both drags, which is why no row here
    /// spells it.
    ///
    /// Four shapes of `true` sit in the table:
    ///
    /// - the ops that DRIVE the gesture ([`SessionOp::PreviewGesture`],
    ///   [`SessionOp::CommitGesture`],
    ///   [`SessionOp::PreviewVariableGesture`],
    ///   [`SessionOp::CommitVariableGesture`],
    ///   [`SessionOp::CancelGesture`]), which a guard would deadlock.
    ///   **Permitted here is not unconditional**: the four that name a
    ///   target are refused [`Refusal::WrongGesture`] from inside their
    ///   own arms when the target is not the open gesture's. That is a
    ///   question about a payload and this table is a function of the
    ///   operation alone, so it is answered where the gesture's state
    ///   is and not by a row here;
    /// - layer-3 moves that touch neither the document nor the history
    ///   ([`SessionOp::Select`], [`SessionOp::Hover`], the free-move
    ///   family, [`SessionOp::SetBodyHidden`]) and the evaluation
    ///   controls ([`SessionOp::CancelEvaluation`],
    ///   [`SessionOp::Reevaluate`]);
    /// - [`SessionOp::Save`], which has TWO effects and this row is
    ///   about both. It writes the COMMITTED document, so it ignores a
    ///   preview that is not in it; and on a save-as whose parent
    ///   directory differs it rebinds the resolver and re-evaluates,
    ///   which submits the SHOWN document — mid-gesture, the scratch.
    ///   That second half acts on the preview the first half ignores.
    ///   What makes it safe is the door it goes through:
    ///   `DocSession::request_eval` submits a document and writes no
    ///   history state, so the drag's base and the committed history
    ///   are both untouched and the picture being dragged is the one
    ///   the new directory's references resolve for — the directory
    ///   rule following the file. That is a property of `request_eval`
    ///   and is asserted where it lives, not here. Whether a save
    ///   under an open drag should be permitted at all is a question
    ///   this table only records the current answer to.
    ///
    /// - the two doors that OPEN a value gesture
    ///   ([`SessionOp::BeginGesture`],
    ///   [`SessionOp::BeginVariableGesture`]). They are permitted here
    ///   and refused anyway, one layer down: `DocSession::start`
    ///   hands them to [`crate::g1::Slot::begin`], whose first rule refuses
    ///   [`Refusal::GestureInFlight`] off the very state this check
    ///   reads. A `false` row would be a second spelling of one
    ///   answer and would make the door's own arm unreachable — the
    ///   argument `permitted_during_free_move` makes for
    ///   [`SessionOp::BeginFreeMove`], which is the same rule about
    ///   the other drag.
    ///
    /// Everything else is refused, and all but one of those rows move
    /// the document, the history or the file the drag is previewing
    /// against — stated as "all but one" rather than as a count,
    /// because the count is a property of the enum and goes stale the
    /// day an operation joins it. [`SessionOp::ProbeBounds`] is the
    /// exception and moves none of them: it READS the shown document,
    /// which
    /// mid-drag is the scratch, so a range taken there would be a
    /// statement about a picture the drag is about to replace and
    /// would outlive it by one keystroke.
    #[must_use]
    pub fn permitted_during_value_gesture(&self) -> bool {
        match self {
            Self::Select(_)
            | Self::Hover(_)
            | Self::Stage { .. }
            | Self::PreviewGesture { .. }
            | Self::CommitGesture { .. }
            | Self::PreviewVariableGesture { .. }
            | Self::CommitVariableGesture { .. }
            | Self::CancelGesture
            | Self::CancelEvaluation
            | Self::Reevaluate
            | Self::Save(_)
            | Self::SetBodyHidden { .. }
            | Self::SetSpaceHidden { .. }
            | Self::BeginGesture { .. }
            | Self::BeginVariableGesture { .. }
            | Self::BeginFreeMove { .. }
            | Self::PreviewFreeMove { .. }
            | Self::CommitFreeMove { .. }
            | Self::CancelFreeMove
            | Self::DeclineOffer { .. } => true,
            Self::DeleteNode { .. }
            | Self::RemoveConstraint { .. }
            | Self::IntoOwnSpace { .. }
            | Self::SetSlot { .. }
            | Self::SetSlotVariable { .. }
            | Self::ProbeBounds { .. }
            | Self::SetSlotUnit { .. }
            | Self::SetSlotExpression { .. }
            | Self::SetVariable { .. }
            | Self::SetVariableUnit { .. }
            | Self::SetVariableText { .. }
            | Self::DeclareVar { .. }
            | Self::RenameVar { .. }
            | Self::DeleteVar { .. }
            | Self::Undo
            | Self::Redo
            | Self::Open(_)
            | Self::SaveAsNewDocument(_)
            | Self::NewDocument { .. }
            | Self::AddMate { .. }
            | Self::AddProfile { .. }
            | Self::SketchOnFace { .. }
            | Self::AddDrawnProfile { .. }
            | Self::EditProfile { .. }
            | Self::AddExtrude { .. }
            | Self::AddRevolve { .. }
            | Self::AddBoolean { .. }
            | Self::AddSplit { .. }
            | Self::AddMirror { .. }
            | Self::AddTransform { .. }
            | Self::AddPattern { .. }
            | Self::AddFusedPattern { .. }
            | Self::AddFillet { .. }
            | Self::AddChamfer { .. }
            | Self::AddPart { .. }
            | Self::Duplicate { .. }
            | Self::AddInstance { .. }
            | Self::AcceptPartVersion { .. }
            | Self::SetLabel { .. }
            | Self::SetProduct { .. } => false,
            // A labelled creation is its creation, labelled.
            Self::CreateLabelled { creation, .. } => creation.op().permitted_during_value_gesture(),
        }
    }

    /// Whether this operation is permitted while a **free-move
    /// gesture** is in flight — the probe drag
    /// [`SessionOp::BeginFreeMove`] opens on
    /// [`crate::display::DisplayState`], the other of the session's
    /// two independent drags.
    ///
    /// # Why there are two tables and not one
    ///
    /// The two drags refuse DIFFERENT sets, so one predicate could
    /// only answer for both by refusing the union — and the union is
    /// wrong in both directions. A value gesture refuses every
    /// operation that moves the document, because it previews against
    /// a snapshot of it. A free move does not: a commit that lands
    /// under a probe is pruned against the new document and REPORTED
    /// ([`crate::display::PruneReport`]), which is a better answer
    /// than a refusal — the user learns the mate took their placement
    /// instead of being told to finish a drag they can finish for
    /// nothing.
    ///
    /// So this table refuses exactly the two doors a prune cannot
    /// answer for: [`SessionOp::Open`] and [`SessionOp::NewDocument`]
    /// **replace** the document rather than moving it, and
    /// `DocSession::clear_for_new_document` then drops the whole
    /// display state — the in-flight drag with it, under the pointer
    /// still holding it. That is the half-acted state the value
    /// gesture's table refuses for, applied to the other drag for the
    /// same reason.
    ///
    /// **And a report is not available here as the alternative**, which
    /// is what settles the fork rather than taste. A withdrawal names
    /// a node or a space and carries a [`crate::display::DisplayFault`] about
    /// a document; at a replacement the only document left to ask is
    /// the incoming one, where a [`pncad::document::RecipeNodeId`]
    /// minted by the outgoing document's counter means something else
    /// or nothing. [`crate::display::DisplayState::clear`] states that
    /// argument in full. A door that can neither report truthfully nor
    /// act without destroying a gesture refuses.
    ///
    /// # What is NOT in this table
    ///
    /// The `*FreeMove` quartet is `true` here, and three of the four
    /// have to be: a begin, a preview, a commit and a cancel are how a
    /// drag is driven and ended, and refusing one would strand the
    /// gesture this refusal exists to protect.
    /// [`SessionOp::BeginFreeMove`] is the exception, and it is `true`
    /// because it is already answered one layer down —
    /// [`crate::display::DisplayState::begin_free_move`] refuses
    /// [`crate::display::DisplayFault::FreeMoveInFlight`] off its own
    /// state, with the same refusal this check raises. A second `false`
    /// row would be a second spelling of one answer. The value table
    /// says the same of its own two begins, so the argument is one
    /// rule about rule 1 rather than one table's exception.
    ///
    /// [`SessionOp::PreviewFreeMove`] and [`SessionOp::CommitFreeMove`]
    /// are `true` here and still refused
    /// [`crate::display::DisplayFault::WrongFreeMove`] when they name
    /// a node that is not the one being probed — the value
    /// gesture's rule, answered the same way and for the same reason:
    /// the question is about a payload against a state, and this table
    /// is a function of the operation alone.
    ///
    /// [`SessionOp::Save`] is `true`, as it is in the value table and
    /// for a narrower reason: a save writes the committed document and
    /// a free-move probe enters no history at all, so there is nothing
    /// of the drag for a save to write or to lose.
    #[must_use]
    pub fn permitted_during_free_move(&self) -> bool {
        match self {
            Self::Open(_) | Self::NewDocument { .. } => false,
            Self::Select(_)
            | Self::Hover(_)
            | Self::Stage { .. }
            | Self::PreviewGesture { .. }
            | Self::CommitGesture { .. }
            | Self::PreviewVariableGesture { .. }
            | Self::CommitVariableGesture { .. }
            | Self::CancelGesture
            | Self::CancelEvaluation
            | Self::Reevaluate
            | Self::Save(_)
            | Self::SaveAsNewDocument(_)
            | Self::SetBodyHidden { .. }
            | Self::SetSpaceHidden { .. }
            | Self::BeginFreeMove { .. }
            | Self::PreviewFreeMove { .. }
            | Self::CommitFreeMove { .. }
            | Self::CancelFreeMove
            | Self::DeleteNode { .. }
            | Self::RemoveConstraint { .. }
            | Self::IntoOwnSpace { .. }
            | Self::SetSlot { .. }
            | Self::SetSlotVariable { .. }
            | Self::DeclineOffer { .. }
            | Self::ProbeBounds { .. }
            | Self::SetSlotUnit { .. }
            | Self::SetSlotExpression { .. }
            | Self::SetVariable { .. }
            | Self::SetVariableUnit { .. }
            | Self::SetVariableText { .. }
            | Self::DeclareVar { .. }
            | Self::RenameVar { .. }
            | Self::DeleteVar { .. }
            | Self::BeginGesture { .. }
            | Self::BeginVariableGesture { .. }
            | Self::Undo
            | Self::Redo
            | Self::AddMate { .. }
            | Self::AddProfile { .. }
            | Self::SketchOnFace { .. }
            | Self::AddDrawnProfile { .. }
            | Self::EditProfile { .. }
            | Self::AddExtrude { .. }
            | Self::AddRevolve { .. }
            | Self::AddBoolean { .. }
            | Self::AddSplit { .. }
            | Self::AddMirror { .. }
            | Self::AddTransform { .. }
            | Self::AddPattern { .. }
            | Self::AddFusedPattern { .. }
            | Self::AddFillet { .. }
            | Self::AddChamfer { .. }
            | Self::AddPart { .. }
            | Self::Duplicate { .. }
            | Self::AddInstance { .. }
            | Self::AcceptPartVersion { .. }
            | Self::SetLabel { .. }
            | Self::SetProduct { .. } => true,
            Self::CreateLabelled { creation, .. } => creation.op().permitted_during_free_move(),
        }
    }

    /// **Whether this operation puts another document under the
    /// session**, when it lands: the doors that run
    /// `DocSession::clear_for_new_document`. Everything the viewer
    /// holds about the outgoing document is owed a reset by exactly
    /// these, and a refusal raised before one says nothing about the
    /// ids of the document after it.
    ///
    /// Every operation is listed, so a new one is sorted here by
    /// whoever adds it.
    #[must_use]
    pub fn replaces_the_document(&self) -> bool {
        match self {
            Self::Open(_) | Self::NewDocument { .. } => true,
            Self::Select(_)
            | Self::Hover(_)
            | Self::PreviewGesture { .. }
            | Self::CommitGesture { .. }
            | Self::PreviewVariableGesture { .. }
            | Self::CommitVariableGesture { .. }
            | Self::CancelGesture
            | Self::CancelEvaluation
            | Self::Reevaluate
            | Self::Save(_)
            | Self::SaveAsNewDocument(_)
            | Self::SetBodyHidden { .. }
            | Self::SetSpaceHidden { .. }
            | Self::BeginFreeMove { .. }
            | Self::PreviewFreeMove { .. }
            | Self::CommitFreeMove { .. }
            | Self::CancelFreeMove
            | Self::DeleteNode { .. }
            | Self::RemoveConstraint { .. }
            | Self::IntoOwnSpace { .. }
            | Self::SetSlot { .. }
            | Self::SetSlotVariable { .. }
            | Self::DeclineOffer { .. }
            | Self::ProbeBounds { .. }
            | Self::Stage { .. }
            | Self::SetSlotUnit { .. }
            | Self::SetSlotExpression { .. }
            | Self::SetVariable { .. }
            | Self::SetVariableUnit { .. }
            | Self::SetVariableText { .. }
            | Self::DeclareVar { .. }
            | Self::RenameVar { .. }
            | Self::DeleteVar { .. }
            | Self::BeginGesture { .. }
            | Self::BeginVariableGesture { .. }
            | Self::Undo
            | Self::Redo
            | Self::AddMate { .. }
            | Self::AddProfile { .. }
            | Self::SketchOnFace { .. }
            | Self::AddDrawnProfile { .. }
            | Self::EditProfile { .. }
            | Self::AddExtrude { .. }
            | Self::AddRevolve { .. }
            | Self::AddBoolean { .. }
            | Self::AddSplit { .. }
            | Self::AddMirror { .. }
            | Self::AddTransform { .. }
            | Self::AddPattern { .. }
            | Self::AddFusedPattern { .. }
            | Self::AddFillet { .. }
            | Self::AddChamfer { .. }
            | Self::AddPart { .. }
            | Self::Duplicate { .. }
            | Self::AddInstance { .. }
            | Self::AcceptPartVersion { .. }
            | Self::SetLabel { .. }
            | Self::SetProduct { .. } => false,
            Self::CreateLabelled { creation, .. } => creation.op().replaces_the_document(),
        }
    }
}

/// **An operation that creates a node** — the only operation
/// [`SessionOp::CreateLabelled`] labels, so a label can never be handed
/// to an operation that has no node to give it.
#[derive(Clone, Debug, PartialEq)]
pub struct Creation(Box<SessionOp>);

impl Creation {
    /// `op` as a creation, or `op` back when it creates no node.
    ///
    /// # Errors
    ///
    /// The operation itself, when it is not one of the creating
    /// operations.
    pub fn of(op: SessionOp) -> Result<Self, Box<SessionOp>> {
        let op = Box::new(op);
        if op.creates_a_node() {
            Ok(Self(op))
        } else {
            Err(op)
        }
    }

    /// The creating operation.
    #[must_use]
    pub fn op(&self) -> &SessionOp {
        &self.0
    }

    /// The creating operation, unwrapped.
    #[must_use]
    pub fn into_op(self) -> SessionOp {
        *self.0
    }
}

/// What an operation did.
#[derive(Debug, Default)]
pub struct OpOutcome {
    /// The edits that entered the history, in the order they applied
    /// — exactly one for a gesture's whole drag.
    pub committed: Vec<DocEdit<ProfileProgram>>,
    /// The edits evaluated against scratch state and NOT recorded.
    pub previewed: Vec<DocEdit<ProfileProgram>>,
    /// **The ids this operation's inserts MINTED**, in the order the
    /// action applied them — empty for every operation that inserted
    /// nothing.
    ///
    /// The `EditRecord::minted` the edit door already answers with,
    /// carried out to the chrome instead of being dropped at the
    /// session's edge. A form that has just committed a node it will
    /// go on referring to has no other way to learn its id: a
    /// `DocEdit::InsertNode` carries the payload and not the id, and
    /// reading "the last node in document order" would be a guess
    /// about an action nobody promised inserts only one thing.
    pub minted: Vec<RecipeNodeId>,
    /// Why nothing (or nothing more) happened.
    pub refusal: Option<Refusal>,
    /// What this operation's document transition WITHDREW from the
    /// display state — the prune's own report, carried rather than
    /// copied out field by field.
    ///
    /// **The three kinds and the argument for wording them apart are
    /// [`PruneReport`]'s**, stated once where the prune fills them in.
    /// They were re-declared here for a while, with the same element
    /// types and the same reasons written twice; what that bought was
    /// a second place a fourth kind had to be added, and what it cost
    /// is that neither copy was the place the kind is RENDERED. The
    /// rendering door is [`crate::frame::Withdrawal::all`], which
    /// destructures this report, so a fourth kind now reds at the
    /// sentence that has to word it.
    ///
    /// Empty on every operation that moved no document — a refusal, a
    /// selection, a hover — because there was no transition to prune
    /// against.
    pub withdrawn: PruneReport,
    /// **What the committed edits did that the user did not ask for by
    /// name** — the edit door's `Applied::maintenance`, every row of
    /// every edit the action applied, in the order they applied and
    /// each edit's rows in the door's own order.
    ///
    /// The log keeps the edits alone (replay re-applies them, and each
    /// re-derives its rows), so this is the one place the rows —
    /// a name stranded or rewritten in place, an appearance key
    /// stranded — leave the session. The chrome words them through
    /// [`crate::frame::outcome_notices`].
    ///
    /// **Net over the action, not per edit.** One action can apply
    /// several edits (a cascade delete, a profile on a new frame), and
    /// a row an earlier edit reported can be made moot by a later one — a strand the action went on to repair or whose
    /// carrier it deleted, a name it moved
    /// twice. The rows are netted by the action's
    /// `pncad::document::Recording`, which states which survive,
    /// so this holds what is true of the document the action ended at.
    ///
    /// Empty on every operation that committed nothing, and on a
    /// gesture's previews: a preview enters no history, so nothing it
    /// did has happened yet.
    pub maintenance: Vec<Maintenance>,
}

impl OpOutcome {
    pub(super) fn refused(refusal: Refusal) -> Self {
        Self {
            refusal: Some(refusal),
            ..Self::default()
        }
    }

    /// Every withdrawal a prune reported, on the outcome that carries
    /// it to the chrome.
    ///
    /// **Nothing is copied out.** This used to destructure the report
    /// into three fields of the same names, so that a fourth kind was
    /// E0027 at the COPY; the copy is gone and with it the second
    /// declaration, and the exhaustiveness sits at
    /// [`crate::frame::Withdrawal::all`] instead — the call that has
    /// to word the kind, which is where a missed one was actually
    /// reaching the user unworded.
    pub(super) fn from_prune(withdrawn: PruneReport) -> Self {
        Self {
            withdrawn,
            ..Self::default()
        }
    }
}

/// **One cancel door**: a gesture's exit that is NOT the widget the
/// gesture was opened on.
///
/// A gesture's ordinary exit is the release event on the field that
/// opened it ([`crate::widgets::drag_gesture_ops`]'s `drag_stopped`
/// arm), and that exit exists only on a frame the field is drawn.
/// `DocSession::slot_rows` answers nothing for a selection whose
/// standing is not live, so a slot drag whose own preview lands an
/// evaluation its picked face does not survive loses its only door
/// under the pointer still holding it: no release operation is
/// emitted, the gesture stays open with no pointer behind it, and
/// [`Refusal::GestureInFlight`] — *"finish the drag first"* — then
/// answers every operation that moves the document, naming a remedy
/// with nothing behind it. A cancel door is that remedy, and its
/// absence is what made the refusal dishonest.
///
/// **A door is drawn while its gesture is in flight, and only then**:
/// the toolbar is where a cancel goes, and a cancel with nothing to
/// cancel is chrome a reader at rest has to read past. Whether it is in
/// flight is [`CancelDoor::blocked`], the refusal its own operation
/// gives with no gesture open — the operation's answer, not a second
/// test beside it.
#[derive(Debug)]
pub struct CancelDoor {
    /// What the control is called.
    pub label: &'static str,
    /// The operation a click emits.
    pub op: SessionOp,
    /// `None` while the gesture this door closes is in flight — the
    /// one state the door can act in, and the one state it is drawn
    /// in — and otherwise the refusal [`CancelDoor::op`] answers with.
    pub blocked: Option<Refusal>,
}

impl CancelDoor {
    /// One door, from the gesture's own in-flight state and the
    /// refusal its operation gives when there is no gesture.
    ///
    /// **Both doors are composed here rather than spelled twice.** The
    /// two gestures are held by different values, opened by different
    /// operations and refuse in different vocabularies; that a cancel
    /// is available exactly while its gesture is in flight, and says
    /// the operation's own refusal otherwise, is the part that is not
    /// a difference.
    pub(super) fn of(
        label: &'static str,
        op: SessionOp,
        in_flight: bool,
        refused: Refusal,
    ) -> Self {
        Self {
            label,
            op,
            blocked: (!in_flight).then_some(refused),
        }
    }
}
