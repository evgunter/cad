//! **The typed operations on the session, and what one did**: the
//! crate's shared operation vocabulary.
//!
//! A VOCABULARY. [`SessionOp`] is a value the panels name and the
//! session performs, [`OpOutcome`] is the value it reports; neither
//! names the session, which is what lets a test name the same op a
//! widget does.
//!
//! [`SessionOp::permitted_during_value_gesture`] is the mid-gesture
//! policy as data — one exhaustive match rather than a rule inferred
//! from every dispatch target.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::path::PathBuf;

use pncad::document::{
    Alignment, BooleanOp, DocEdit, DocParam, DocumentId, Expr, Frame, LoopProgram, ParamName,
    ProfileProgram, RecipeNodeId, SlotId,
};
use pncad::prelude::StableName;
use pncad::quantity::UnitDef;
use pncad::select::ContactClass;

use crate::props::SlotValue;
use crate::session::author::{DatumSpec, PatternRuleSpec};
use crate::session::probe::BoundsTarget;
use crate::session::refuse::Refusal;
use crate::session::select::{Hovered, Selection};

/// One typed operation on the session.
#[derive(Clone, Debug)]
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
    /// takes no number. The affordance names the driving parameters,
    /// which are the fields to probe instead.
    ProbeBounds {
        /// The field to probe.
        target: BoundsTarget,
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
    /// shipped `parse_expr` door. This is the affordance's editing
    /// half: a driven slot refuses a NUMBER, never an expression.
    SetSlotExpression {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
        /// The expression source.
        text: String,
    },
    /// Write a value into a document parameter.
    SetParam {
        /// The parameter.
        name: ParamName,
        /// The new value.
        value: SlotValue,
    },
    /// Declare a NEW document parameter — the panel's create
    /// affordance, committing exactly one `DocEdit::SetDocParam`.
    ///
    /// That edit is create-or-replace, and stays so at the API. This
    /// door refuses an already-declared name typed
    /// ([`Refusal::ParamExists`]): a "create" that replaced would
    /// change not just the value but possibly the declared DIMENSION,
    /// re-validating every referencing expression — a blast radius no
    /// plus-shaped button should carry. Replacing stays spellable
    /// through the door that says so ([`SessionOp::SetParam`], which
    /// conversely refuses a name that does NOT exist — the two doors
    /// partition the edit's semantics).
    CreateParam {
        /// The new parameter's name.
        name: ParamName,
        /// Its declared dimension and exact value.
        value: DocParam,
    },
    /// Start a continuous gesture over a slot.
    BeginGesture {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
    },
    /// Start a continuous gesture over a DOCUMENT PARAMETER.
    ///
    /// The same preview/commit machinery as [`SessionOp::BeginGesture`]
    /// and deliberately a separate door rather than a widened one: the
    /// two targets are addressed differently (a node and a slot; a
    /// name) and collapsing them would put an `Option` in every arm.
    /// A parameter is where the expression-driven affordance sends a
    /// user, so it is a dragged widget on a primary path and gets the
    /// gesture rule the ratified preview-vs-commit decision demands.
    BeginParamGesture {
        /// The parameter.
        name: ParamName,
    },
    /// Move the in-flight gesture. Emits a preview edit against
    /// scratch state; commits nothing.
    PreviewGesture {
        /// The value under the pointer.
        value: f64,
    },
    /// Release: commit exactly one edit carrying the gesture's last
    /// previewed value.
    CommitGesture,
    /// Abandon the gesture, leaving the document untouched.
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
    /// Write the current path to a file.
    ///
    /// **Saving a COPY beside the original bricks the store for both**
    /// (issue #1117): the copy claims the same document id, so the
    /// directory then holds two files with one identity and every
    /// resolution through it refuses `DuplicateId` — typed and
    /// recoverable (delete either file), but a surprising blast
    /// radius for an ordinary act. The identity is the document's, not
    /// the file's, so a cheap rename-on-save cannot fix it without
    /// forking the document; the issue carries the design question.
    /// Save over the original, or save into a different directory.
    ///
    /// **A save-as MOVES the document seam**: the directory rule
    /// follows the file, so an assembly saved into a directory that
    /// does not hold its parts silently rebinds to that directory and
    /// its instances refuse at the next evaluation — typed on the tree
    /// badges, and recoverable by saving back, but nothing warns
    /// first. Recorded with the rest of the seam's newly-reachable
    /// edges in issue #1387.
    Save(PathBuf),
    /// Hide or show one instance (G3): a DISPLAY operation — the
    /// scene and the pick index drop a hidden instance; the document
    /// and the feature tree keep it, and nothing is persisted.
    SetInstanceHidden {
        /// The instance.
        instance: RecipeNodeId,
        /// Hidden, or shown again.
        hidden: bool,
    },
    /// Open a free-move probe gesture on a completely-unconstrained
    /// instance (G3's fit probe). Refused typed for a mate-
    /// constrained instance — eligibility is derived from the
    /// document, never guessed from solver state.
    BeginFreeMove {
        /// The instance to probe.
        instance: RecipeNodeId,
    },
    /// Stream the probe's display frame. Each preview REPLACES the
    /// last; nothing enters the document.
    PreviewFreeMove {
        /// The display frame composed over the instance's drawn
        /// placement.
        frame: Frame,
    },
    /// Land the probe: the last previewed frame becomes the
    /// instance's committed display value. NO history holds it — the
    /// plan's undo note governs document state only.
    CommitFreeMove,
    /// Abandon the probe, restoring the committed picture.
    CancelFreeMove,
    /// Commit **exactly one** `DocEdit` adding a mate node — the mate
    /// tool's single committed edit. Everything before it (the two
    /// picks, the class choice, the derived frames) is tool state; the
    /// document transition is this op alone, entering at the same
    /// commit door as every other edit: one apply, one history state,
    /// one re-evaluation, and the free-move supersession prune.
    AddMate {
        /// The `a` reference — the head names a member of A11's
        /// vocabulary (`pncad::document::member_of`).
        a: StableName,
        /// The `b` reference, same vocabulary.
        b: StableName,
        /// The declared contact class.
        class: ContactClass,
        /// The alignment datum (frames in each member's own part
        /// coordinates).
        alignment: Alignment,
    },
    /// Replace the session's document with a fresh empty one (GAUTH-1's
    /// creation door zero) — the ONE creation op that is a
    /// session-state replacement rather than an `InsertNode`, because
    /// before it runs there is no document to insert into.
    ///
    /// The identity ruling (logged in `docs/GAUTH-LOG.md`): the id is
    /// authored at creation from the typed name —
    /// `DocumentId::derive(&name)`, the deterministic spelling the
    /// demo corpus uses — and never re-minted at save. Two documents
    /// derived from one name collide at WORKSPACE resolution, where
    /// the store's duplicate-id refusal is the fail-loud recourse.
    ///
    /// Everything session-scoped is cleared: path, history, selection,
    /// hover, display state and the resolver (a fresh document has no
    /// backing file, so its instantiate nodes refuse with the shipped
    /// no-resolver semantics until it is saved). Refused mid-gesture
    /// by the table ([`SessionOp::permitted_during_value_gesture`]);
    /// a blank name refuses [`Refusal::EmptyName`].
    ///
    /// The name is TRIMMED before the id is derived, so `" ring "` and
    /// `"ring"` are one document by design — surrounding whitespace is
    /// a typing accident, not an identity a user could re-derive on
    /// purpose.
    NewDocument {
        /// The document's name; the id is derived from its trim.
        name: String,
    },
    /// Insert one datum node with literal slots — the add-datum forms'
    /// one committed edit each.
    AddDatum {
        /// The datum's literal payload.
        datum: DatumSpec,
    },
    /// Insert one profile node from template loops on a plane — the
    /// add-profile tool's one committed edit.
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
    /// the literal door). The plane is a REFERENCE to a frame node,
    /// which the pick below spells out.
    AddProfile {
        /// **The frame node the profile is drawn on** — a PICK, not a
        /// field.
        ///
        /// It was a `SketchPlane<f64>` the form filled in from a
        /// world-XY constant. A profile's plane is a document node
        /// now, so the form names one that already exists rather than
        /// minting one on the side: one submit inserts one node, and
        /// the frame a person drew on is the frame they can see in the
        /// viewport and edit afterwards.
        ///
        /// A reference that does not name a `Datum::Frame` refuses
        /// [`Refusal::WrongNodeKind`] at the door, like every other
        /// pick.
        plane: RecipeNodeId,
        /// The loop programs, in description order.
        loops: Vec<LoopProgram>,
    },
    /// Insert one extrude of an existing profile node — the extrude
    /// tool's one committed edit. A `profile` that is not a
    /// `Node::Profile` in this document refuses
    /// [`Refusal::WrongNodeKind`] at the door.
    ///
    /// A NEGATIVE distance is admitted deliberately and builds: it is
    /// an extrusion along the negative sketch normal, the same value
    /// the property panel can author into the slot afterwards, and
    /// the door does not narrow what the vocabulary means.
    AddExtrude {
        /// The profile node extruded.
        profile: RecipeNodeId,
        /// The extrusion distance (`Length`).
        distance: Expr,
    },
    /// Insert one revolve of an existing profile node about an
    /// existing axis datum — the revolve tool's one committed edit.
    /// Either seat's wrong-kind pick refuses
    /// [`Refusal::WrongNodeKind`] at the door.
    AddRevolve {
        /// The profile node revolved.
        profile: RecipeNodeId,
        /// The `Datum::AxisInPlane` node revolved about — an axis
        /// written in the same sketch frame the profile is drawn on.
        axis: RecipeNodeId,
        /// The sweep angle (`Angle`); the chrome's default is a full
        /// turn.
        angle: Expr,
    },
    /// Insert one regularized boolean of two existing bodies — the
    /// boolean tool's one committed edit (GAUTH-4).
    ///
    /// **The operand order is data**: `Subtract` keeps `a` and removes
    /// `b`, so the two seats are not interchangeable and the form says
    /// which pick is which. Either seat's non-body pick refuses
    /// [`Refusal::WrongNodeKind`]; one node in both seats refuses
    /// [`Refusal::SelfBoolean`].
    ///
    /// `declare` is authored `None`: coincidence intent is a
    /// `Node::Declare` input, and authoring one needs the entity picks
    /// (a face pair) that this tool does not take. A declaration is
    /// added afterwards through the vocabulary that owns it, never
    /// guessed at here.
    AddBoolean {
        /// The operation — the KERNEL's enum, which the recipe node
        /// carries unconverted.
        op: BooleanOp,
        /// The first operand: the body a subtraction keeps.
        a: RecipeNodeId,
        /// The second operand: the body a subtraction removes.
        b: RecipeNodeId,
    },
    /// Insert one split of an existing body by an existing datum
    /// plane — the split tool's one committed edit.
    ///
    /// The tool seat is a PLANE and not a body: `Node::Split`'s tool
    /// operand is the plane the cut is taken on. Both seats refuse
    /// [`Refusal::WrongNodeKind`] for the wrong kind.
    AddSplit {
        /// The body split.
        target: RecipeNodeId,
        /// The `Datum::Plane` node it is cut by.
        tool: RecipeNodeId,
    },
    /// Insert one rigid placement of an existing body — the transform
    /// tool's one committed edit. The property panel is the editor for
    /// every slot afterwards.
    AddTransform {
        /// The body placed.
        input: RecipeNodeId,
        /// Translation components (`Length`).
        translation: [Expr; 3],
        /// Rotation-axis components (`Scalar`).
        rotation_axis: [Expr; 3],
        /// Rotation angle (`Angle`).
        rotation_angle: Expr,
    },
    /// Insert one pattern of an existing body — the pattern tool's one
    /// committed edit.
    ///
    /// The count is an `i64` and lands as an exact Count literal: it
    /// is the node's STRUCTURAL slot (spec D3), edited afterwards
    /// through `SetStructuralParam` and never through the continuous
    /// door. A count of zero or less is admitted here and refuses at
    /// evaluation on the node's own badge — the same division of
    /// labour a degenerate profile takes, one rule for authored and
    /// hand-written documents.
    AddPattern {
        /// The body replicated.
        input: RecipeNodeId,
        /// Instance count.
        count: i64,
        /// The replication rule, with the axis a circular rule was
        /// picked with.
        rule: PatternRuleSpec,
    },
    /// Insert one FUSED pattern of an existing body — the pattern
    /// tool's other committed edit, and the door a boolean's refusal
    /// of a pattern points at.
    ///
    /// [`SessionOp::AddPattern`]'s twin, seat for seat and slot for
    /// slot, because `Node::PlacedUnion` takes exactly what
    /// `Node::Pattern` takes: a PROTOTYPE body, a count, a rule. What
    /// differs is the result — ONE body, the union of the prototype at
    /// every placement, which every downstream body seat consumes
    /// where a pattern's several instances are refused.
    ///
    /// **The prototype, not a pattern node.** The fused node replicates
    /// a body itself rather than consuming a `Node::Pattern`, so this
    /// door's seat wants a BODY and refuses
    /// [`Refusal::WrongNodeKind`] for a pattern exactly as the
    /// unfused door does. A user who already authored the unfused
    /// pattern re-commits over the same prototype and deletes the
    /// pattern; there is no edit that converts one node into the
    /// other, and inventing one here would be a second spelling of a
    /// rule the document already has.
    ///
    /// **Disjointness is the node's question, not this door's.**
    /// Placements that overlap refuse typed at evaluation on the
    /// node's own badge, the same division of labour a non-positive
    /// count takes: the door authors what the user asked for and the
    /// node says whether it can be built.
    AddPlacedUnion {
        /// The prototype placed at every placement.
        input: RecipeNodeId,
        /// Placement count.
        count: i64,
        /// The placement rule, with the axis a circular rule was
        /// picked with.
        rule: PatternRuleSpec,
    },
    /// Insert one constant-radius fillet on a SET of an existing
    /// body's edges — the blend tool's one committed edit, as a
    /// fillet.
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
    /// (`NodeErrorKind::BlendSelectionResolve`, and
    /// `BlendSelectionEmpty` for an empty set) — a door that
    /// pre-screened it would be a second authority on the same fact,
    /// and would refuse to author the node whose refusal is the honest
    /// thing to show. The target's KIND is a fact about the committed
    /// document alone, so that one does refuse here
    /// ([`Refusal::WrongNodeKind`]).
    AddFillet {
        /// The body whose edges are blended.
        target: RecipeNodeId,
        /// The blend radius (`Length`).
        radius: Expr,
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
        distance: Expr,
        /// The edges to chamfer, by stable name.
        selection: Vec<StableName>,
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
    /// No placement is authored: A11 puts placement on the cluster and
    /// an instance carries no frame of its own, so the inserted node
    /// is complete with its reference and an empty interface record
    /// (an authored instance crosses no split seam). Hiding, the
    /// free-move probe and the mate tool take it from there.
    AddInstance {
        /// Which document in the open document's own directory.
        id: DocumentId,
    },
}

impl SessionOp {
    /// Whether this operation is permitted while a **value gesture**
    /// is in flight — a slot or document-parameter drag opened by
    /// [`SessionOp::BeginGesture`] or [`SessionOp::BeginParamGesture`],
    /// the only thing [`Refusal::GestureInFlight`] ever speaks about.
    ///
    /// **It is not a statement about the free-move gesture.** That is
    /// a second, independent drag living on the display state
    /// ([`crate::display::DisplayState::begin_free_move`]), with its own in-flight
    /// refusal. Nothing observed so far enforces either an exclusion
    /// or an independence between the two: they can be open at once
    /// and neither answer implies the other, and whether that is
    /// intended is open — see
    /// `work/view/two-gestures-can-be-in-flight-together.md`. An
    /// operation this returns `true` for is permitted
    /// mid-value-gesture and nothing more.
    ///
    /// The whole policy is here, exhaustively, so that the set of
    /// operations a drag refuses can be READ rather than reconstructed
    /// from the dispatch, and so that a new operation cannot join the
    /// enum without an answer: [`super::DocSession::perform`] consults this
    /// once, before dispatch, and no arm re-guards against the VALUE
    /// gesture. Four arms do guard against the OTHER one: the
    /// `*FreeMove` quartet delegates to [`crate::display::DisplayState`], which refuses
    /// [`crate::display::DisplayFault::FreeMoveInFlight`] off its own state.
    ///
    /// Three shapes of `true` sit in the table:
    ///
    /// - the ops that DRIVE the gesture ([`SessionOp::PreviewGesture`],
    ///   [`SessionOp::CommitGesture`], [`SessionOp::CancelGesture`]),
    ///   which a guard would deadlock;
    /// - layer-3 moves that touch neither the document nor the history
    ///   ([`SessionOp::Select`], [`SessionOp::Hover`], the free-move
    ///   family, [`SessionOp::SetInstanceHidden`]) and the evaluation
    ///   controls ([`SessionOp::CancelEvaluation`],
    ///   [`SessionOp::Reevaluate`]);
    /// - [`SessionOp::Save`], which writes the COMMITTED history and
    ///   so ignores a preview that is not in it. Whether a save under
    ///   an open drag should be permitted at all is a question this
    ///   table only records the current answer to.
    ///
    /// Everything else moves the document, the history or the file the
    /// drag is previewing against, and is refused.
    #[must_use]
    pub fn permitted_during_value_gesture(&self) -> bool {
        match self {
            Self::Select(_)
            | Self::Hover(_)
            | Self::PreviewGesture { .. }
            | Self::CommitGesture
            | Self::CancelGesture
            | Self::CancelEvaluation
            | Self::Reevaluate
            | Self::Save(_)
            | Self::SetInstanceHidden { .. }
            | Self::BeginFreeMove { .. }
            | Self::PreviewFreeMove { .. }
            | Self::CommitFreeMove
            | Self::CancelFreeMove => true,
            Self::DeleteNode { .. }
            | Self::SetSlot { .. }
            | Self::ProbeBounds { .. }
            | Self::SetSlotUnit { .. }
            | Self::SetSlotExpression { .. }
            | Self::SetParam { .. }
            | Self::CreateParam { .. }
            | Self::BeginGesture { .. }
            | Self::BeginParamGesture { .. }
            | Self::Undo
            | Self::Redo
            | Self::Open(_)
            | Self::NewDocument { .. }
            | Self::AddMate { .. }
            | Self::AddDatum { .. }
            | Self::AddProfile { .. }
            | Self::AddExtrude { .. }
            | Self::AddRevolve { .. }
            | Self::AddBoolean { .. }
            | Self::AddSplit { .. }
            | Self::AddTransform { .. }
            | Self::AddPattern { .. }
            | Self::AddPlacedUnion { .. }
            | Self::AddFillet { .. }
            | Self::AddChamfer { .. }
            | Self::AddInstance { .. } => false,
        }
    }
}

/// What an operation did.
#[derive(Debug, Default)]
pub struct OpOutcome {
    /// The edits that entered the history — at most one per op, and
    /// exactly one for a gesture's whole drag.
    pub committed: Vec<DocEdit<ProfileProgram>>,
    /// The edits evaluated against scratch state and NOT recorded.
    pub previewed: Vec<DocEdit<ProfileProgram>>,
    /// Why nothing (or nothing more) happened.
    pub refusal: Option<Refusal>,
    /// Instances whose free-move probe was **discarded** by this
    /// operation's document transition — the G3 supersession, reported
    /// rather than inferred: a mate landing on a probed instance
    /// removes its probe here, and the instance is drawn at its
    /// solved placement from the next landed evaluation on.
    pub superseded: Vec<RecipeNodeId>,
}

impl OpOutcome {
    pub(super) fn refused(refusal: Refusal) -> Self {
        Self {
            refusal: Some(refusal),
            ..Self::default()
        }
    }
}
