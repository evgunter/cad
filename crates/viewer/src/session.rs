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

/// A gesture in flight: layer-3 state only.
#[derive(Debug)]
struct Gesture {
    node: RecipeNodeId,
    slot: SlotId,
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
    /// It answered an older document and was discarded.
    Stale,
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
    pub fn land(&mut self, done: crate::evalseam::EvalDone) -> Landing {
        if done.generation != self.generation {
            return Landing::Stale;
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
            SessionOp::PreviewGesture { value } => self.preview_gesture(value),
            SessionOp::CommitGesture => self.commit_gesture(),
            SessionOp::CancelGesture => {
                let had = self.gesture.take().is_some();
                self.scratch = None;
                if had {
                    self.request_eval();
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
        self.gesture = Some(Gesture {
            node,
            slot,
            base: self.history.doc().clone(),
            value: None,
        });
        OpOutcome::default()
    }

    fn preview_gesture(&mut self, value: f64) -> OpOutcome {
        let Some(gesture) = self.gesture.as_mut() else {
            return OpOutcome::refused(Refusal::NoGesture);
        };
        // A Count slot under a continuous gesture takes the value
        // truncated toward zero; the slot's own dimension decides,
        // never the widget.
        let slot_value = if gesture.slot.is_structural() {
            SlotValue::Count(value as i64)
        } else {
            SlotValue::Continuous(value)
        };
        let edit = match props::slot_edit(gesture.node, gesture.slot, slot_value) {
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
        self.scratch = None;
        // A gesture that never moved commits nothing: one undo step
        // per gesture that CHANGED something, none for a click that
        // happened to land on a slider.
        let Some(value) = gesture.value else {
            self.request_eval();
            return OpOutcome::default();
        };
        match props::slot_edit(gesture.node, gesture.slot, value) {
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
