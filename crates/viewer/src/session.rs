//! The document session: everything the panels operate on, and the
//! typed operations that operate on it.
//!
//! # One shape, from the toolkit and from a test
//!
//! G1's rule is that every operation the GUI performs is itself API.
//! [`SessionOp`] is that vocabulary for the panels, [`DocSession::perform`]
//! is the only function that performs one, and [`OpOutcome`] reports
//! what it emitted. A widget's job is to name an op; a test's job is to
//! name the same op and read the outcome. Nothing the panels can do is
//! expressible only as a click.
//!
//! # Preview and commit are the same edit, entered differently
//!
//! A continuous gesture — dragging a slider over a dimension —
//! evaluates PREVIEW edits against a scratch document and commits
//! exactly one `DocEdit` on release. The scratch document is a value
//! beside the history, never in it, so a gesture leaves no trace if it
//! is abandoned and exactly one undo step if it is not. Transient
//! gesture state (which slot, the value in flight) lives here in layer
//! 3 and never enters the document.
//!
//! # Every edit re-enters at the same door
//!
//! `apply` is pure, so a committed edit produces a new document value,
//! which is pushed onto the history and submitted to the evaluation
//! seam under a fresh generation. There is one such door
//! ([`DocSession::commit`]) and one submit ([`DocSession::request_eval`]);
//! undo, redo, open and every edit route through them, which is why
//! "the picture agrees with the document" is a property of the
//! structure rather than of each call site remembering.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use pncad::document::{
    Alignment, Dimension, DimensionError, Doc, DocEdit, EditError, Evaluation, Frame, Node,
    ParamName, ParseError, PartResolver, ProductError, ProfileProgram, RecipeNodeId, SlotId, apply,
    assemble, parse_expr, product,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::select::{ContactClass, Resolution, RunCtx, resolve};

use crate::display::{DisplayFault, DisplayState, DisplayView};
use crate::docio::{self, DirResolver, DocIoError};
use crate::evalseam::{EvalRequest, EvalService, Generation, InlineEvaluator};
use crate::history::History;
use crate::props::{self, SlotDriver, SlotValue};
use crate::tree::{self, TreeRow};

/// A picked face: the stable name it is, and the node whose body
/// carried it when it was picked.
///
/// **The name is the selection**; the node rides along because it is
/// what the feature tree highlights and what the property panel shows
/// slots for, and re-deriving it would mean resolving the name again
/// for a question the pick already answered. G1's rule is satisfied
/// exactly: a `StableName` and a `RecipeNodeId`, no arena key.
///
/// The node is the one whose evaluated body was hit, which is not
/// necessarily `name.node` (the node whose operation MINTED the
/// entity): a face minted by a profile's extrude and passed through a
/// later transform is hit on the transform's body and named after the
/// extrude. Both are true and they answer different questions; this
/// field answers "whose body did the ray meet".
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaceSelection {
    /// The picked face's stable name — what survives re-evaluation.
    pub name: StableName,
    /// The node whose body was hit.
    pub node: RecipeNodeId,
    /// The output body index within that node's value.
    pub body: u32,
}

/// What the session has selected. A typed layer-3 value: stable
/// names, recipe node ids and parameter names, never an arena key.
///
/// **Single-select, by ratification** (the GUI plan's rulings): one
/// selection, and nothing here is shaped to grow a second. Multi-select
/// is GQ7 and deferred by design.
///
/// **ONE value for the viewport and the panels.** A face picked in the
/// viewport and a node clicked in the tree write the same field, which
/// is what makes "click a face, watch its feature highlight" a
/// property of the state rather than of two widgets agreeing —
/// [`Selection::node`] is the one inversion both read.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Selection {
    /// Nothing selected.
    #[default]
    None,
    /// A recipe node, selected in the feature tree.
    Node(RecipeNodeId),
    /// A document parameter, selected in the property panel — where
    /// the expression-driven refusal's affordance navigates to.
    Param(ParamName),
    /// A face, picked in the viewport.
    Face(FaceSelection),
}

impl Selection {
    /// The recipe node this selection is about, when it is about one:
    /// the node itself, or a picked face's owning node.
    ///
    /// **The one home for the viewport→tree inversion.** The feature
    /// tree's highlight and the property panel's slot rows both read
    /// it, so a face pick reaches them without either of them knowing
    /// what a face is.
    pub fn node(&self) -> Option<RecipeNodeId> {
        match self {
            Self::Node(id) => Some(*id),
            Self::Face(face) => Some(face.node),
            Self::None | Self::Param(_) => None,
        }
    }

    /// The picked face, when the selection is one.
    pub fn face(&self) -> Option<&FaceSelection> {
        match self {
            Self::Face(face) => Some(face),
            Self::None | Self::Node(_) | Self::Param(_) => None,
        }
    }
}

/// Whether the selection still denotes something in the evaluation on
/// screen — the ratified resolution-failure semantics, as a value.
///
/// **A vanished reference is a STATE, not an event.** Nothing clears
/// the selection when the thing it names stops existing: the name
/// stays, this verdict changes, and the chrome renders the unresolved
/// state distinctly while the affordances that need a live entity
/// switch off. That is the whole of GQ7's recorded constraint (tools
/// survive the referenced entity vanishing) at v1's single-select
/// scope.
#[derive(Clone, Debug, PartialEq)]
pub enum Standing {
    /// There is nothing selected to resolve.
    Empty,
    /// A node selection, and whether the document still holds it.
    Node {
        /// The node.
        node: RecipeNodeId,
        /// Whether it is still in the recipe.
        present: bool,
    },
    /// A parameter selection, and whether the document still declares
    /// it.
    Param {
        /// The parameter.
        name: ParamName,
        /// Whether it is still declared.
        present: bool,
    },
    /// A face selection, and the resolution verdict its name got.
    Face {
        /// The selection.
        face: FaceSelection,
        /// What the shipped resolution machinery answered — `None`
        /// when there is no evaluation to answer against yet, which is
        /// neither "live" nor "vanished" and is not reported as
        /// either.
        ///
        /// Boxed for the reason [`Refusal::Edit`] is: a `Resolution`
        /// carrying a diagnosis and a tombstone is an order of
        /// magnitude wider than the other arms here, and this value is
        /// returned by value on every frame.
        resolution: Option<Box<Resolution>>,
    },
}

impl Standing {
    /// Whether the selection denotes something the chrome may edit
    /// against.
    ///
    /// The one predicate the dependent affordances read. A face whose
    /// name did not resolve is NOT live; so is one with no evaluation
    /// behind it yet, because "we cannot tell" and "yes" are not the
    /// same answer and only one of them may enable a button.
    pub fn live(&self) -> bool {
        match self {
            Self::Empty => false,
            Self::Node { present, .. } | Self::Param { present, .. } => *present,
            Self::Face { resolution, .. } => {
                matches!(resolution.as_deref(), Some(Resolution::Resolved(_)))
            }
        }
    }

    /// The typed unresolved verdict, when the selection has one.
    ///
    /// `Some` exactly when a face selection's name failed to resolve
    /// or the evaluation could not answer for it — the two arms that
    /// render distinctly.
    pub fn unresolved(&self) -> Option<&Resolution> {
        match self {
            Self::Face {
                resolution: Some(resolution),
                ..
            } if !matches!(**resolution, Resolution::Resolved(_)) => Some(resolution),
            _ => None,
        }
    }
}

/// Why an operation did not happen. Every arm is a value the chrome
/// renders; none is a message this layer composed about someone else's
/// failure.
#[derive(Debug)]
pub enum Refusal {
    /// The slot is driven by an expression, so a direct numeric edit
    /// is refused — the ratified affordance. The payload is what the
    /// affordance needs: which parameters drive it (each navigable and
    /// editable), and what the slot evaluates to today.
    DrivenByExpression {
        /// The node holding the slot.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
        /// The document parameters the driving expression reads.
        params: Vec<ParamName>,
        /// The slot's current value, when it has one.
        current: Option<SlotValue>,
    },
    /// The node does not exist, or does not carry that slot.
    NoSuchSlot {
        /// The node named.
        node: RecipeNodeId,
        /// The slot named.
        slot: SlotId,
    },
    /// No document parameter by that name.
    NoSuchParam(ParamName),
    /// `apply` refused the edit.
    ///
    /// Boxed, as `Io` is below: these two payloads are an order of
    /// magnitude larger than every other arm, and a refusal is
    /// returned by value from functions on the ordinary path — so the
    /// unboxed shape made every `Ok` in this module pay for the widest
    /// error nobody was raising.
    Edit(Box<EditError>),
    /// The value was not a usable dimensioned literal.
    Dimension(DimensionError),
    /// The expression text did not parse.
    Parse(Box<ParseError>),
    /// A gesture operation arrived with no gesture in flight.
    NoGesture,
    /// A gesture is in flight, so this operation is not available.
    GestureInFlight,
    /// A file operation failed.
    Io(Box<DocIoError>),
    /// Undo at the root, or redo at the tip of the current branch.
    NothingToDo,
    /// A display-state operation refused (hide on a non-instance, a
    /// free-move on a mate-constrained instance, a gesture out of
    /// order) — the fault's own typed vocabulary, unaltered.
    Display(DisplayFault),
}

impl Refusal {
    /// How much this refusal has to say, lower being more.
    ///
    /// **A frame performs a BATCH of operations**, and a batch can hold
    /// more than one refusal: dragging an expression-driven slot queues
    /// `BeginGesture` (refused with the ratified affordance) and
    /// `PreviewGesture` (refused `NoGesture`, purely because the first
    /// refusal stopped the gesture from opening). A chrome that keeps
    /// the last refusal shows the second one and buries the decision
    /// the affordance exists to deliver.
    ///
    /// So the ranks are: the affordance first, because it is a ratified
    /// decision about what the user just tried; then every refusal that
    /// names a real failure; then the bookkeeping ones, which are
    /// consequences of an earlier refusal at least as often as they are
    /// news. [`Refusal::preferred`] applies it.
    pub fn rank(&self) -> u8 {
        match self {
            Self::DrivenByExpression { .. } => 0,
            Self::NoSuchSlot { .. }
            | Self::NoSuchParam(_)
            | Self::Edit(_)
            | Self::Dimension(_)
            | Self::Parse(_)
            | Self::Io(_) => 1,
            // The two gesture-order arms rank with their document
            // twins; the substantive display refusals rank with the
            // real failures, because "this instance is mate-
            // constrained" is a decision about what the user tried.
            Self::Display(DisplayFault::NoFreeMove | DisplayFault::FreeMoveInFlight) => 2,
            Self::Display(_) => 1,
            Self::NoGesture | Self::GestureInFlight | Self::NothingToDo => 2,
        }
    }

    /// The refusal a frame should show, given the one it already has.
    ///
    /// Strictly better wins; ties keep the incumbent, so within one
    /// rank the FIRST refusal of a frame is the one displayed — it is
    /// the one that describes what the user's action ran into, and
    /// everything after it is downstream of that.
    pub fn preferred(shown: Option<Self>, next: Self) -> Option<Self> {
        match shown {
            Some(shown) if shown.rank() <= next.rank() => Some(shown),
            _ => Some(next),
        }
    }
}

impl core::fmt::Display for Refusal {
    /// Renders each arm through its payload's OWN `Display` wherever
    /// the payload has one — `EditError`, `DimensionError` and
    /// `PersistError` (inside [`DocIoError`]) all do, and using them is
    /// the same rule the feature tree's badges follow: the layer that
    /// raised the failure names it.
    ///
    /// Two exceptions, both stated rather than hidden. The affordance
    /// arm's wording is a RATIFIED decision of this layer's, so it is
    /// composed here (and here only — [`Refusal::affordance`] is its
    /// single home). And `ParseError` has no `Display` in
    /// `editor-core`, so that one arm still shows a debug rendering;
    /// the gap is recorded in issue #1103 alongside the unparser, which
    /// is the same missing surface.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DrivenByExpression {
                params, current, ..
            } => write!(f, "{}", Self::affordance(params, *current)),
            Self::NoSuchSlot { node, slot } => {
                write!(f, "node {node:?} has no {slot:?} slot")
            }
            Self::NoSuchParam(name) => write!(f, "no document parameter named {}", name.0),
            Self::Edit(error) => write!(f, "the edit was refused: {error}"),
            Self::Dimension(error) => write!(f, "{error}"),
            Self::Parse(error) => write!(f, "the expression did not parse: {error:?}"),
            Self::NoGesture => write!(f, "no drag is in progress"),
            Self::GestureInFlight => write!(f, "finish the drag first"),
            Self::Io(error) => write!(f, "{error}"),
            Self::NothingToDo => write!(f, "nothing to undo or redo"),
            Self::Display(fault) => write!(f, "{fault}"),
        }
    }
}

impl core::error::Error for Refusal {}

impl Refusal {
    /// **The ratified affordance sentence, and its one home.**
    ///
    /// "Dragging an expression-driven dimension → refuse, with an
    /// affordance" is a ratified micro-decision whose WORDING is part
    /// of the decision, so it is composed once and every surface that
    /// shows it — the status line, the inline note under the slot row —
    /// calls this. Two independently-built copies is how the wording
    /// drifts from the decision.
    pub fn affordance(params: &[ParamName], current: Option<SlotValue>) -> String {
        let over = if params.is_empty() {
            "an expression".to_owned()
        } else {
            let names: Vec<&str> = params.iter().map(|p| p.0.as_str()).collect();
            format!("an expression over {}", names.join(", "))
        };
        match current {
            Some(value) => format!(
                "driven by {over} (currently {}) — edit the expression?",
                value.as_f64()
            ),
            None => format!("driven by {over} — edit the expression?"),
        }
    }
}

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
    Hover(Option<FaceSelection>),
    /// Delete a recipe node.
    ///
    /// The panel's own door to `DocEdit::DeleteNode`, and the edit the
    /// survival semantics are exercised through: deleting the feature
    /// a selected face belongs to is the first of the three ways a
    /// selection's referent goes away.
    DeleteNode {
        /// The node to delete.
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
        /// The `a` reference (instance-qualified).
        a: StableName,
        /// The `b` reference (instance-qualified).
        b: StableName,
        /// The declared contact class.
        class: ContactClass,
        /// The alignment datum (frames in each instance's own part
        /// coordinates).
        alignment: Alignment,
    },
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
    fn refused(refusal: Refusal) -> Self {
        Self {
            refusal: Some(refusal),
            ..Self::default()
        }
    }
}

/// What a gesture is dragging.
#[derive(Clone, Debug)]
enum GestureTarget {
    /// A node's named slot.
    Slot { node: RecipeNodeId, slot: SlotId },
    /// A document parameter, with the dimension it is declared at.
    Param {
        name: ParamName,
        dimension: Dimension,
    },
}

impl GestureTarget {
    /// This target's dimension — a slot's from its `SlotId`, a
    /// parameter's from its declaration.
    fn dimension(&self) -> Dimension {
        match self {
            Self::Slot { slot, .. } => slot.dimension(),
            Self::Param { dimension, .. } => *dimension,
        }
    }

    /// Which arm a dragged `f64` becomes, through
    /// [`SlotValue::of`] — the one home for that rule.
    fn value_of(&self, value: f64) -> SlotValue {
        SlotValue::of(self.dimension(), value)
    }

    /// The edit that writes `value` into this target.
    fn edit(&self, value: SlotValue) -> Result<DocEdit<ProfileProgram>, DimensionError> {
        match self {
            Self::Slot { node, slot } => props::slot_edit(*node, *slot, value),
            Self::Param { name, dimension } => {
                Ok(props::param_edit(name.clone(), *dimension, value))
            }
        }
    }
}

/// A gesture in flight: layer-3 state only.
#[derive(Debug)]
struct Gesture {
    target: GestureTarget,
    /// The document the previews are applied to — the history's
    /// current value, held so each preview replaces the last rather
    /// than stacking.
    base: Doc<ProfileProgram>,
    /// The last previewed value, and the one the commit records.
    value: Option<SlotValue>,
}

/// The whole of what the document panels operate on.
pub struct DocSession {
    history: History,
    tol: Tol,
    selection: Selection,
    /// What the cursor is over: transient, never persisted, and its
    /// ONE home. A widget that kept its own copy would be the
    /// per-widget shadow the panels' inventory discipline forbids.
    hover: Option<FaceSelection>,
    gesture: Option<Gesture>,
    scratch: Option<Doc<ProfileProgram>>,
    eval: Box<dyn EvalService>,
    generation: Generation,
    /// The document handed to the seam under [`DocSession::generation`]
    /// — kept so a landed result can be paired with the document it
    /// actually answers.
    ///
    /// `Arc` so that landing it costs a handle rather than a copy: the
    /// landed pair and the outstanding request name the SAME document
    /// value for as long as they agree, which is most of the time.
    requested_doc: Arc<Doc<ProfileProgram>>,
    landed: Option<Arc<Evaluation<f64>>>,
    /// The document [`DocSession::landed`] answers.
    ///
    /// **Resolution is a question about a PAIR.** `resolve` reads the
    /// recipe and the evaluation together, so asking it about the
    /// document as it stands now and the evaluation as it stood one
    /// edit ago is asking about a run that never happened — and the
    /// diagnosis it answers with would be about that non-run. The two
    /// move together here, one generation behind the shown document
    /// while a run is outstanding, which is exactly what the picture
    /// does.
    landed_doc: Option<Arc<Doc<ProfileProgram>>>,
    landed_generation: Option<Generation>,
    /// The gather's refusal for the landed pair, computed once when it
    /// lands ([`DocSession::product_fault`]).
    landed_fault: Option<ProductError>,
    /// The A5 at-rest verdict for the landed pair, computed once when
    /// it lands ([`DocSession::at_rest`]); `None` for a document that
    /// is not assembly-shaped, or before anything lands.
    landed_at_rest: Option<AtRestBadge>,
    path: Option<PathBuf>,
    /// Layer-3 display state — hide and free-move — with its one home
    /// here (the seam-friction inventory rule). Never persisted; reset
    /// by `Open`; pruned against every new document value.
    display: DisplayState,
    /// The document seam: the opened file's own directory, consulted
    /// lazily (the directory rule and the scan-at-resolution posture
    /// are [`DirResolver`]'s docs). A session over an in-memory
    /// document carries no resolver, and its instantiate nodes refuse
    /// typed. Replaced — never inherited — on every `Open`, so a
    /// document can never silently resolve against the previous
    /// document's directory.
    resolver: Option<Arc<DirResolver>>,
}

/// The A5 at-rest verdict for the landed pair — a mated document's
/// declarations run through the kernel's own verification door
/// (`assemble`), once per landed evaluation, so a committed mate's
/// class verdict does not die at the commit: a `Tangent` that solves
/// green still shows the gate refusing it, and an undeclared contact
/// between instances surfaces on the draw path instead of waiting for
/// an export.
///
/// Taken only for assembly-shaped documents (one holding at least one
/// `InstantiatePart`) — a part document's tiers are not this badge's
/// subject, and the gate's cost is not spent where it answers nothing
/// the badges do not already say.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AtRestBadge {
    /// The gate certified the assembled product; how many declarations
    /// its mates minted.
    Certified {
        /// The minted declaration count.
        minted: usize,
    },
    /// The gate refused — its own rendering, never a sentence composed
    /// here.
    Refused {
        /// The typed refusal's `Display`.
        message: String,
    },
}

/// What happened to a result the seam handed back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Landing {
    /// It answered the current document and is now the session's.
    Landed,
    /// It answered an older request and was discarded.
    Stale,
    /// It answered the current request but was CANCELED, so it carries
    /// the completed prefix of a run nobody asked to see half of. The
    /// last good evaluation stays on screen and the session goes on
    /// owing an answer.
    Canceled,
}

impl DocSession {
    /// A session over `doc`, evaluated through `eval`.
    ///
    /// The first evaluation is requested here, so a session is never
    /// in a state where the document and the picture disagree because
    /// nobody asked.
    pub fn new(doc: Doc<ProfileProgram>, tol: Tol, eval: Box<dyn EvalService>) -> Self {
        let mut session = Self {
            // A placeholder the `request_eval` below overwrites before
            // anything can read it. It is the empty document rather
            // than a clone of `doc` because a clone here would be a
            // second copy nobody ever looks at.
            requested_doc: Arc::new(Doc::empty_derived("unsubmitted", tol)),
            history: History::new(doc),
            tol,
            selection: Selection::None,
            hover: None,
            gesture: None,
            scratch: None,
            eval,
            generation: Generation::FIRST,
            landed: None,
            landed_doc: None,
            landed_fault: None,
            landed_at_rest: None,
            landed_generation: None,
            path: None,
            display: DisplayState::new(),
            resolver: None,
        };
        session.request_eval();
        session
    }

    /// A session running the seam inline — the shape a test and the
    /// wasm build use.
    pub fn inline(doc: Doc<ProfileProgram>, tol: Tol) -> Self {
        Self::new(doc, tol, Box::new(InlineEvaluator::new()))
    }

    /// The document the panels show: the gesture's scratch value while
    /// one is in flight, the history's current value otherwise.
    pub fn doc(&self) -> &Doc<ProfileProgram> {
        self.scratch.as_ref().unwrap_or_else(|| self.history.doc())
    }

    /// The document the history is on, ignoring any preview.
    pub fn committed_doc(&self) -> &Doc<ProfileProgram> {
        self.history.doc()
    }

    /// The edit history.
    pub fn history(&self) -> &History {
        &self.history
    }

    /// The ε this session decides at.
    pub fn tol(&self) -> Tol {
        self.tol
    }

    /// The current selection.
    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    /// What the cursor is over, if anything.
    pub fn hover(&self) -> Option<&FaceSelection> {
        self.hover.as_ref()
    }

    /// Whether the selection still denotes something in the evaluation
    /// on screen.
    ///
    /// **A pure function of (shown document, landed evaluation,
    /// selection)** — recomputed, never cached, so it cannot be stale
    /// with respect to the state it describes. A face's verdict comes
    /// from the shipped `resolve` door; nothing here re-implements the
    /// resolution ladder or interprets its answer beyond arranging it
    /// beside the other two selection kinds.
    pub fn standing(&self) -> Standing {
        match &self.selection {
            Selection::None => Standing::Empty,
            Selection::Node(node) => Standing::Node {
                node: *node,
                present: self.doc().node(*node).is_some(),
            },
            Selection::Param(name) => Standing::Param {
                name: name.clone(),
                present: self.doc().params().contains_key(name),
            },
            Selection::Face(face) => Standing::Face {
                face: face.clone(),
                resolution: self
                    .landed_pair()
                    .map(|(doc, eval)| Box::new(resolve(RunCtx { doc, eval }, &face.name))),
            },
        }
    }

    /// The most recent evaluation that answered the current document.
    pub fn evaluation(&self) -> Option<&Evaluation<f64>> {
        self.landed.as_deref()
    }

    /// The most recent evaluation, shared.
    pub fn evaluation_arc(&self) -> Option<&Arc<Evaluation<f64>>> {
        self.landed.as_ref()
    }

    /// The landed evaluation together with the document it answers —
    /// the pair every name question is asked of.
    ///
    /// The two fields are set in one place and read together, so a
    /// caller cannot pick up one without the other.
    pub fn landed_pair(&self) -> Option<(&Doc<ProfileProgram>, &Evaluation<f64>)> {
        Some((self.landed_doc.as_deref()?, self.landed.as_deref()?))
    }

    /// Why the landed evaluation's product does not gather, if it does
    /// not — the gather-level refusal no per-node badge can carry.
    ///
    /// `None` both when the product is well formed and when nothing
    /// has landed yet; [`DocSession::landed_pair`] distinguishes those.
    pub fn product_fault(&self) -> Option<&ProductError> {
        self.landed_fault.as_ref()
    }

    /// The A5 at-rest verdict for the landed pair ([`AtRestBadge`]),
    /// when the landed document is assembly-shaped. `None` for a part
    /// document, and before anything lands.
    pub fn at_rest(&self) -> Option<&AtRestBadge> {
        self.landed_at_rest.as_ref()
    }

    /// The file this session is backed by, if any.
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// The layer-3 display state (hide, free-move).
    pub fn display(&self) -> &DisplayState {
        &self.display
    }

    /// The display snapshot the scene and pick paths consume.
    pub fn display_view(&self) -> DisplayView {
        self.display.view(self.doc())
    }

    /// The directory this session resolves part references against —
    /// the opened file's own, or `None` for an in-memory document
    /// (the [`DirResolver`] directory rule).
    pub fn resolve_dir(&self) -> Option<&Path> {
        self.resolver.as_deref().map(DirResolver::dir)
    }

    /// The generation the session is waiting for a result on.
    pub fn generation(&self) -> Generation {
        self.generation
    }

    /// The generation of the result currently on screen — what a
    /// consumer derived from the evaluation (the scene) compares
    /// against to know whether its own copy is current.
    pub fn landed_generation(&self) -> Option<Generation> {
        self.landed_generation
    }

    /// Whether the result on screen answers a document the session has
    /// already moved past — what the busy indicator reads. A VALUE the
    /// chrome consumes, not a spinner the seam draws.
    ///
    /// Defined against the SESSION's own two generations rather than
    /// by asking the seam whether it has work: those agree whenever
    /// the seam is the only source of results, and where they differ
    /// this is the honest one. "Am I showing the current document" is
    /// the question a busy indicator answers.
    pub fn busy(&self) -> bool {
        self.landed_generation != Some(self.generation)
    }

    /// Whether the seam actually has work outstanding.
    ///
    /// [`DocSession::busy`] asks "is the picture older than the
    /// document"; this asks "is anyone doing something about it". They
    /// agree except in one state, and that state is the whole reason
    /// this exists: after a Cancel the picture is older and **nothing
    /// is running**, so a chrome that only read `busy` would show a
    /// spinner over an idle seam forever. `busy() && !running()` is
    /// "canceled, showing an older result" — what
    /// [`SessionOp::Reevaluate`] recovers from.
    pub fn running(&self) -> bool {
        self.eval.busy()
    }

    /// The feature tree's rows for the shown document.
    pub fn tree_rows(&self) -> Vec<TreeRow> {
        // **The landed PAIR, not the shown document against the landed
        // evaluation.** A row's badge is a statement about what a run
        // said about a node, so reading it off a document that run
        // never saw describes a run that never happened — the same
        // defect `landed_pair` exists to make unreachable, and it lived
        // one function away from the fix that introduced it. While a
        // run is outstanding the tree therefore shows the picture's
        // document, which is what the viewport shows too.
        match self.landed_pair() {
            Some((doc, eval)) => tree::rows(doc, Some(eval)),
            // Nothing has landed: the shown document with no
            // evaluation, which renders every row `Unevaluated`.
            None => tree::rows(self.doc(), None),
        }
    }

    /// The property rows for the selected node — the node itself, or a
    /// picked face's owning node ([`Selection::node`]); empty for any
    /// other selection.
    ///
    /// **A selection whose referent has vanished offers no rows.**
    /// That is the "disables dependent affordances" half of the
    /// resolution-failure semantics, taken at the source rather than
    /// at each widget: a panel that cannot get rows cannot offer an
    /// edit against a face that is no longer there.
    pub fn slot_rows(&self) -> Vec<props::SlotRow> {
        if !self.standing().live() {
            return Vec::new();
        }
        match self.selection.node() {
            Some(id) => props::slot_rows(self.doc(), id),
            None => Vec::new(),
        }
    }

    /// Take whatever the seam has finished, discarding results for
    /// documents the session has moved past.
    ///
    /// Returns one entry per result handled, so a caller can assert on
    /// what was discarded rather than infer it.
    pub fn pump(&mut self) -> Vec<Landing> {
        let mut landings = Vec::new();
        while let Some(done) = self.eval.poll() {
            landings.push(self.land(done));
        }
        landings
    }

    /// Decide one result's fate. Public so the staleness rule is
    /// testable without a scheduler.
    ///
    /// **Two filters, and both are refusals to show a wrong picture.**
    /// A result for a superseded request is [`Landing::Stale`]. A
    /// result for the CURRENT request that did not complete is
    /// [`Landing::Canceled`]: a canceled run answers with the prefix it
    /// finished before the token was seen, and that prefix is not the
    /// document — rendered, it is a feature tree of `Unevaluated` rows
    /// and a product that gathers to nothing. The last good evaluation
    /// therefore stays, and [`DocSession::busy`] goes on reporting that
    /// the picture is older than the document, which it is.
    pub fn land(&mut self, done: crate::evalseam::EvalDone) -> Landing {
        if done.generation != self.generation {
            return Landing::Stale;
        }
        if !done.completed() {
            return Landing::Canceled;
        }
        self.landed_generation = Some(done.generation);
        // **The product's own verdict, once per landed evaluation.**
        // The gather is the only thing that answers "is this
        // document's product well formed" — a naming collision across
        // roots is not a node failure, so the feature tree's badges
        // cannot see it, and a viewport that draws the parts without
        // ever asking would render a body nothing says is wrong.
        // Computed HERE because here is the one place a result becomes
        // the session's, so it cannot be run twice or skipped.
        self.landed_fault = product(self.requested_doc.as_ref(), &done.evaluation, self.tol).err();
        // The A5 verdict, in the same once-per-landing spot and for
        // the same reason: nowhere else can it run exactly once per
        // result that becomes the session's.
        self.landed_at_rest = at_rest_of(self.requested_doc.as_ref(), &done.evaluation, self.tol);
        self.landed = Some(done.evaluation);
        self.landed_doc = Some(Arc::clone(&self.requested_doc));
        Landing::Landed
    }

    /// Perform one operation.
    pub fn perform(&mut self, op: SessionOp) -> OpOutcome {
        match op {
            SessionOp::Select(selection) => {
                self.selection = selection;
                OpOutcome::default()
            }
            SessionOp::Hover(hover) => {
                self.hover = hover;
                OpOutcome::default()
            }
            SessionOp::DeleteNode { node } => {
                if self.gesture.is_some() {
                    return OpOutcome::refused(Refusal::GestureInFlight);
                }
                self.commit(DocEdit::DeleteNode { id: node })
            }
            SessionOp::SetSlot { node, slot, value } => self.set_slot(node, slot, value),
            SessionOp::SetSlotExpression { node, slot, text } => {
                self.set_slot_expression(node, slot, &text)
            }
            SessionOp::SetParam { name, value } => self.set_param(&name, value),
            SessionOp::BeginGesture { node, slot } => self.begin_gesture(node, slot),
            SessionOp::BeginParamGesture { name } => self.begin_param_gesture(&name),
            SessionOp::PreviewGesture { value } => self.preview_gesture(value),
            SessionOp::CommitGesture => self.commit_gesture(),
            SessionOp::CancelGesture => {
                let had = self.gesture.take().is_some();
                // Same rule as a no-move commit: only a gesture that
                // actually put a scratch document on screen owes a
                // re-submit to take it away again.
                let previewed = self.scratch.take().is_some();
                if had {
                    if previewed {
                        self.request_eval();
                    }
                    OpOutcome::default()
                } else {
                    OpOutcome::refused(Refusal::NoGesture)
                }
            }
            SessionOp::Undo => self.step(true),
            SessionOp::Redo => self.step(false),
            SessionOp::CancelEvaluation => {
                self.eval.cancel();
                OpOutcome::default()
            }
            SessionOp::Reevaluate => {
                self.request_eval();
                OpOutcome::default()
            }
            SessionOp::Open(path) => self.open(&path),
            SessionOp::Save(path) => self.save(&path),
            SessionOp::SetInstanceHidden { instance, hidden } => {
                match self
                    .display
                    .set_hidden(self.history.doc(), instance, hidden)
                {
                    Ok(()) => OpOutcome::default(),
                    Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
                }
            }
            SessionOp::BeginFreeMove { instance } => {
                match self.display.begin_free_move(self.history.doc(), instance) {
                    Ok(()) => OpOutcome::default(),
                    Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
                }
            }
            SessionOp::PreviewFreeMove { frame } => match self.display.preview_free_move(frame) {
                Ok(()) => OpOutcome::default(),
                Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
            },
            SessionOp::CommitFreeMove => match self.display.commit_free_move() {
                Ok(()) => OpOutcome::default(),
                Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
            },
            SessionOp::CancelFreeMove => match self.display.cancel_free_move() {
                Ok(()) => OpOutcome::default(),
                Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
            },
            SessionOp::AddMate {
                a,
                b,
                class,
                alignment,
            } => {
                if self.gesture.is_some() {
                    return OpOutcome::refused(Refusal::GestureInFlight);
                }
                self.commit(DocEdit::InsertNode {
                    node: Node::Mate {
                        a,
                        b,
                        class,
                        alignment,
                    },
                })
            }
        }
    }

    /// The slot's driver and current value, or the refusal that says
    /// the slot is not there.
    fn driver_of(
        &self,
        node: RecipeNodeId,
        slot: SlotId,
    ) -> Result<(SlotDriver, Option<SlotValue>), Refusal> {
        let row = props::slot_rows(self.committed_doc(), node)
            .into_iter()
            .find(|row| row.slot == slot)
            .ok_or(Refusal::NoSuchSlot { node, slot })?;
        Ok((row.driver, row.value.ok()))
    }

    /// Refuse a numeric edit to a driven slot, with the affordance.
    fn guard_driven(&self, node: RecipeNodeId, slot: SlotId) -> Result<(), Refusal> {
        let (driver, current) = self.driver_of(node, slot)?;
        match driver {
            SlotDriver::Literal => Ok(()),
            SlotDriver::Expression { params } => Err(Refusal::DrivenByExpression {
                node,
                slot,
                params,
                current,
            }),
        }
    }

    fn set_slot(&mut self, node: RecipeNodeId, slot: SlotId, value: SlotValue) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        if let Err(refusal) = self.guard_driven(node, slot) {
            return OpOutcome::refused(refusal);
        }
        match props::slot_edit(node, slot, value) {
            Ok(edit) => self.commit(edit),
            Err(error) => OpOutcome::refused(Refusal::Dimension(error)),
        }
    }

    fn set_slot_expression(&mut self, node: RecipeNodeId, slot: SlotId, text: &str) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        // `parse_expr` needs the document's declared dimensions so a
        // parameter reference records the dimension `apply` will
        // re-check it against.
        let dims: std::collections::BTreeMap<ParamName, Dimension> = self
            .committed_doc()
            .params()
            .iter()
            .map(|(name, param)| (name.clone(), param.dim()))
            .collect();
        let expr = match parse_expr(text, &dims) {
            Ok(expr) => expr,
            Err(error) => return OpOutcome::refused(Refusal::Parse(Box::new(error))),
        };
        // An expression edit is available on a driven slot AND on a
        // literal one — the refusal is about writing a number over a
        // computation, not about the slot being off limits.
        let edit = if slot.is_structural() {
            DocEdit::SetStructuralParam { node, slot, expr }
        } else {
            DocEdit::SetParam { node, slot, expr }
        };
        self.commit(edit)
    }

    fn set_param(&mut self, name: &ParamName, value: SlotValue) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        let Some(dimension) = self.committed_doc().params().get(name).map(|p| p.dim()) else {
            return OpOutcome::refused(Refusal::NoSuchParam(name.clone()));
        };
        self.commit(props::param_edit(name.clone(), dimension, value))
    }

    fn begin_gesture(&mut self, node: RecipeNodeId, slot: SlotId) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        if let Err(refusal) = self.guard_driven(node, slot) {
            return OpOutcome::refused(refusal);
        }
        self.start(GestureTarget::Slot { node, slot })
    }

    fn begin_param_gesture(&mut self, name: &ParamName) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        let Some(dimension) = self.committed_doc().params().get(name).map(|p| p.dim()) else {
            return OpOutcome::refused(Refusal::NoSuchParam(name.clone()));
        };
        self.start(GestureTarget::Param {
            name: name.clone(),
            dimension,
        })
    }

    /// Open a gesture on an already-validated target.
    fn start(&mut self, target: GestureTarget) -> OpOutcome {
        self.gesture = Some(Gesture {
            target,
            base: self.history.doc().clone(),
            value: None,
        });
        OpOutcome::default()
    }

    fn preview_gesture(&mut self, value: f64) -> OpOutcome {
        let Some(gesture) = self.gesture.as_mut() else {
            return OpOutcome::refused(Refusal::NoGesture);
        };
        let slot_value = gesture.target.value_of(value);
        let edit = match gesture.target.edit(slot_value) {
            Ok(edit) => edit,
            Err(error) => return OpOutcome::refused(Refusal::Dimension(error)),
        };
        // Applied to the gesture's BASE, so previews replace one
        // another instead of composing, and the history never sees any
        // of them.
        match apply(&gesture.base, &edit, self.tol) {
            Ok(applied) => {
                gesture.value = Some(slot_value);
                self.scratch = Some(applied.doc);
                self.request_eval();
                OpOutcome {
                    previewed: vec![edit],
                    ..OpOutcome::default()
                }
            }
            Err(error) => OpOutcome::refused(Refusal::Edit(Box::new(error))),
        }
    }

    fn commit_gesture(&mut self) -> OpOutcome {
        let Some(gesture) = self.gesture.take() else {
            return OpOutcome::refused(Refusal::NoGesture);
        };
        let previewed = self.scratch.take().is_some();
        // A gesture that never moved commits nothing: one undo step
        // per gesture that CHANGED something, none for a click that
        // happened to land on a slider. It also asks for NOTHING —
        // dropping a scratch that was never set leaves the shown
        // document exactly as it was, and a request for a picture we
        // already have spends a generation and flickers the indicator
        // to say so.
        let Some(value) = gesture.value else {
            if previewed {
                self.request_eval();
            }
            return OpOutcome::default();
        };
        match gesture.target.edit(value) {
            Ok(edit) => self.commit(edit),
            Err(error) => OpOutcome::refused(Refusal::Dimension(error)),
        }
    }

    /// Undo (`toward_root`) or redo.
    fn step(&mut self, toward_root: bool) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        let moved = if toward_root {
            self.history.undo()
        } else {
            self.history.redo()
        };
        if moved.is_none() {
            return OpOutcome::refused(Refusal::NothingToDo);
        }
        // The document moved, so the display state's derived facts
        // (which instances exist; which are mate-constrained) may have
        // too — an undo past a mate's insertion does NOT resurrect a
        // discarded probe (the value is gone, not parked), but a redo
        // over one discards the probe it constrains.
        let superseded = self.display.prune(self.history.doc());
        self.request_eval();
        OpOutcome {
            superseded,
            ..OpOutcome::default()
        }
    }

    fn open(&mut self, path: &Path) -> OpOutcome {
        match docio::open(path, self.tol) {
            Ok(history) => {
                // The directory rule: the resolver is the opened
                // file's own directory, consulted lazily at each
                // resolution (DirResolver's docs carry the posture).
                self.resolver = Some(Arc::new(DirResolver::new(session_dir(path))));
                self.history = history;
                self.selection = Selection::None;
                self.hover = None;
                self.gesture = None;
                // The landed run answered the PREVIOUS document. Left
                // in place it would render the old model's tree and
                // resolve the new document's names against it until
                // the first run lands.
                self.landed = None;
                self.landed_doc = None;
                self.landed_fault = None;
                self.landed_at_rest = None;
                self.landed_generation = None;
                self.scratch = None;
                self.path = Some(path.to_path_buf());
                // G3: hide and free-move are display state of a
                // SESSION over a document, never of the document — a
                // fresh open starts with none, which is also what
                // makes "save, reopen, layer-3 state gone" a property
                // of the structure.
                self.display.clear();
                self.request_eval();
                OpOutcome::default()
            }
            Err(error) => OpOutcome::refused(Refusal::Io(Box::new(error))),
        }
    }

    fn save(&mut self, path: &Path) -> OpOutcome {
        match docio::save_path(path, &self.history, self.tol) {
            Ok(()) => {
                self.path = Some(path.to_path_buf());
                // Save-as moves the backing file, and the directory
                // rule follows the file: rebind the resolver to the
                // new parent and re-evaluate, since references may
                // resolve differently there.
                let dir = session_dir(path);
                if self.resolver.as_deref().map(DirResolver::dir) != Some(dir.as_path()) {
                    self.resolver = Some(Arc::new(DirResolver::new(dir)));
                    self.request_eval();
                }
                OpOutcome::default()
            }
            Err(error) => OpOutcome::refused(Refusal::Io(Box::new(error))),
        }
    }

    /// **The one door an edit enters the document through**: apply,
    /// record, re-evaluate — and reconcile the display state, which is
    /// where a free-move probe is superseded by the mate that
    /// constrains its instance (the prune DISCARDS the value; see
    /// `display`'s module docs for why discard and not zero).
    fn commit(&mut self, edit: DocEdit<ProfileProgram>) -> OpOutcome {
        match apply(self.history.doc(), &edit, self.tol) {
            Ok(applied) => {
                self.history.commit(edit.clone(), applied.doc);
                let superseded = self.display.prune(self.history.doc());
                self.request_eval();
                OpOutcome {
                    committed: vec![edit],
                    superseded,
                    ..OpOutcome::default()
                }
            }
            Err(error) => OpOutcome::refused(Refusal::Edit(Box::new(error))),
        }
    }

    /// **The one submit**: mint the next generation and hand the shown
    /// document to the seam.
    fn request_eval(&mut self) {
        self.generation = self.generation.next();
        // ONE clone of the shown document per request: into the
        // `Arc` the session keeps, then out of it into the request the
        // seam owns. The second is the seam's vocabulary (`EvalRequest`
        // takes a value so a worker owns its copy) and is not a
        // retained copy — the session keeps exactly one.
        self.requested_doc = Arc::new(self.doc().clone());
        self.eval.submit(EvalRequest {
            generation: self.generation,
            doc: self.requested_doc.as_ref().clone(),
            tol: self.tol,
            resolver: self
                .resolver
                .as_ref()
                .map(|ws| Arc::clone(ws) as Arc<dyn PartResolver>),
        });
    }
}

/// The at-rest verdict for one landed pair, taken only for
/// assembly-shaped documents (see [`AtRestBadge`]). The gate's own
/// vocabulary either way: a certification with its minted count, or
/// the typed refusal rendered by its own `Display`.
fn at_rest_of(doc: &Doc<ProfileProgram>, eval: &Evaluation<f64>, tol: Tol) -> Option<AtRestBadge> {
    let assembly_shaped = doc
        .order()
        .iter()
        .any(|&id| matches!(doc.node(id), Some(Node::InstantiatePart { .. })));
    if !assembly_shaped {
        return None;
    }
    Some(match assemble(doc, eval, tol) {
        Ok(assembly) => AtRestBadge::Certified {
            minted: assembly.minted.len(),
        },
        Err(refusal) => AtRestBadge::Refused {
            message: refusal.to_string(),
        },
    })
}

/// The directory a document at `path` resolves against — its parent,
/// with a bare filename reading as the current directory.
fn session_dir(path: &Path) -> PathBuf {
    match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
        _ => PathBuf::from("."),
    }
}

impl std::fmt::Debug for DocSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DocSession")
            .field("generation", &self.generation)
            .field("landed_generation", &self.landed_generation)
            .field("selection", &self.selection)
            .field("hover", &self.hover)
            .field("states", &self.history.len())
            .field("gesture", &self.gesture.is_some())
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}
