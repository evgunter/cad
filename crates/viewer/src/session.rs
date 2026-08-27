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
    Dimension, DimensionError, Doc, DocEdit, EditError, Evaluation, ParamName, ParseError,
    ProfileProgram, RecipeNodeId, SlotId, apply, parse_expr,
};
use pncad::geom_core::Tol;

use crate::docio::{self, DocIoError};
use crate::evalseam::{EvalRequest, EvalService, Generation, InlineEvaluator};
use crate::history::History;
use crate::props::{self, SlotDriver, SlotValue};
use crate::tree::{self, TreeRow};

/// What the panels have selected. A typed layer-3 value: recipe node
/// ids and parameter names, never an arena key.
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
    Save(PathBuf),
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
    gesture: Option<Gesture>,
    scratch: Option<Doc<ProfileProgram>>,
    eval: Box<dyn EvalService>,
    generation: Generation,
    landed: Option<Arc<Evaluation<f64>>>,
    landed_generation: Option<Generation>,
    path: Option<PathBuf>,
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
            history: History::new(doc),
            tol,
            selection: Selection::None,
            gesture: None,
            scratch: None,
            eval,
            generation: Generation::FIRST,
            landed: None,
            landed_generation: None,
            path: None,
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

    /// The most recent evaluation that answered the current document.
    pub fn evaluation(&self) -> Option<&Evaluation<f64>> {
        self.landed.as_deref()
    }

    /// The most recent evaluation, shared.
    pub fn evaluation_arc(&self) -> Option<&Arc<Evaluation<f64>>> {
        self.landed.as_ref()
    }

    /// The file this session is backed by, if any.
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
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
        tree::rows(self.doc(), self.evaluation())
    }

    /// The property rows for the selected node; empty for any other
    /// selection.
    pub fn slot_rows(&self) -> Vec<props::SlotRow> {
        match &self.selection {
            Selection::Node(id) => props::slot_rows(self.doc(), *id),
            Selection::None | Selection::Param(_) => Vec::new(),
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
        self.landed = Some(done.evaluation);
        Landing::Landed
    }

    /// Perform one operation.
    pub fn perform(&mut self, op: SessionOp) -> OpOutcome {
        match op {
            SessionOp::Select(selection) => {
                self.selection = selection;
                OpOutcome::default()
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
        self.request_eval();
        OpOutcome::default()
    }

    fn open(&mut self, path: &Path) -> OpOutcome {
        match docio::open(path, self.tol) {
            Ok(history) => {
                self.history = history;
                self.selection = Selection::None;
                self.gesture = None;
                self.scratch = None;
                self.path = Some(path.to_path_buf());
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
                OpOutcome::default()
            }
            Err(error) => OpOutcome::refused(Refusal::Io(Box::new(error))),
        }
    }

    /// **The one door an edit enters the document through**: apply,
    /// record, re-evaluate.
    fn commit(&mut self, edit: DocEdit<ProfileProgram>) -> OpOutcome {
        match apply(self.history.doc(), &edit, self.tol) {
            Ok(applied) => {
                self.history.commit(edit.clone(), applied.doc);
                self.request_eval();
                OpOutcome {
                    committed: vec![edit],
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
        self.eval.submit(EvalRequest {
            generation: self.generation,
            doc: self.doc().clone(),
            tol: self.tol,
        });
    }
}

impl std::fmt::Debug for DocSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DocSession")
            .field("generation", &self.generation)
            .field("landed_generation", &self.landed_generation)
            .field("selection", &self.selection)
            .field("states", &self.history.len())
            .field("gesture", &self.gesture.is_some())
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}
