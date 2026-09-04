//! The document session: everything the panels operate on, and the
//! typed operations that operate on it.
//!
//! # What this module is allowed to contain
//!
//! The driver, and nothing else. `session` owns [`DocSession`] and
//! dispatches [`SessionOp`]; what stays here is that state, its
//! `Gesture`, [`Landing`], [`AtRestBadge`], [`DocSession::perform`]
//! and the operation doors — every door mutates the session, and
//! `perform`'s dispatch is the one place an operation becomes state.
//!
//! The values those doors speak in are vocabularies beside it, six of
//! them: what is selected is [`select`], the refusal ladder with its
//! recourse wording is [`refuse`], the operation vocabulary itself is
//! [`op`], the authoring specs and their lowering to nodes are
//! [`author`], the delete cascade's wording is [`delete`], and the
//! range probe is [`probe`] (`crates/viewer/README.md`, Module
//! boundaries). None of their `use` blocks names [`DocSession`].
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
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use std::path::{Path, PathBuf};
use std::sync::Arc;

use pncad::document::{
    BooleanOp, ChecksConfig, ChecksReport, Dimension, DimensionError, Doc, DocEdit, DocParam,
    DocRef, DocumentId, Evaluation, Expr, LoopProgram, Node, ParamName, PartResolver, ProductError,
    ProfileProgram, RecipeNodeId, SlotId, apply, assemble, cascade_delete_order, parse_expr,
    product, run_checks,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::quantity::UnitDef;
use pncad::select::{Resolution, RunCtx, resolve};

use crate::blend::BlendKindChoice;
use crate::combine::{self, PatternOutputChoice};
use crate::display::{DisplayState, DisplayView};
use crate::docio::{self, DirResolver};
use crate::evalseam::{EvalRequest, EvalService, Generation, InlineEvaluator};
use crate::history::History;
use crate::parts;
use crate::props::{self, SlotDriver, SlotValue};
use crate::tree::{self, TreeRow};

pub mod author;
pub mod delete;
pub mod op;
pub mod probe;
pub mod refuse;
pub mod select;

pub use author::{DatumSpec, PatternRuleSpec, ProfileShape};
pub use delete::DeleteAffordance;
pub use op::{OpOutcome, SessionOp};
pub use probe::{BoundsReading, BoundsTarget};
pub use refuse::{NodeKindWanted, Refusal, admits};
pub use select::{EdgeSelection, FaceSelection, Hovered, Selection, Standing};

use author::datum_node;

/// What a gesture is dragging.
#[derive(Clone, Debug)]
enum GestureTarget {
    /// A node's named slot, with the display unit that slot's literal
    /// remembered when the gesture opened.
    ///
    /// Captured at `begin_gesture` rather than re-read per preview
    /// because a gesture is defined against its BASE document (the one
    /// every preview is applied to), and the notation is a fact about
    /// that base. Re-reading it would read the scratch document the
    /// previews are writing — the gesture's own output.
    Slot {
        node: RecipeNodeId,
        slot: SlotId,
        unit: Option<UnitDef>,
    },
    /// A document parameter, with the dimension it is declared at.
    ///
    /// **No unit, and it is not the slot arm's omission.** A slot's
    /// edit rebuilds the literal, so the notation has to be carried
    /// into it or the drag rewrites it; a parameter's edit is the
    /// value door (`DocEdit::SetDocParamValue`), which writes a number
    /// into the standing declaration and leaves the authored unit
    /// beside it untouched. There is nothing here for a captured unit
    /// to protect. The panel still SHOWS the drag in the parameter's
    /// written unit — it converts before the value crosses into this
    /// layer, which is canonical throughout.
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
    ///
    /// A parameter's edit is the VALUE door, so the parameter's
    /// declaration — its dimension and any distribution — is read off
    /// the document by the edit itself rather than reassembled here.
    fn edit(&self, value: SlotValue) -> Result<DocEdit<ProfileProgram>, DimensionError> {
        match self {
            Self::Slot { node, slot, unit } => props::slot_edit(*node, *slot, value, *unit),
            Self::Param { name, .. } => Ok(props::param_edit(name.clone(), value)),
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
    hover: Option<Hovered>,
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
    /// The advisory-check report for the landed pair, computed once
    /// when it lands ([`DocSession::checks`]); `None` before anything
    /// lands, or when the registry itself refused.
    landed_checks: Option<ChecksReport>,
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
    /// The last locally-valid-range probe, and the field it was taken
    /// for — layer-3 state, never persisted, and its one home.
    ///
    /// Kept rather than recomputed per frame because a probe costs tens
    /// of evaluations: it is taken when asked for
    /// ([`SessionOp::ProbeBounds`]) and DISCARDED, never repaired, the
    /// moment the document changes. A range is a statement about one
    /// document, and showing yesterday's range beside today's number is
    /// the class of stale-confident answer this crate's staleness rules
    /// exist to prevent.
    bounds: Option<BoundsReading>,
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
            landed_checks: None,
            landed_generation: None,
            path: None,
            display: DisplayState::new(),
            resolver: None,
            bounds: None,
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

    /// **What a delete of `node` would cost, said before it is paid.**
    ///
    /// The `cascade` field is the deletion order itself — exactly what
    /// [`SessionOp::DeleteNode`] applies — so the number on the button
    /// and the number of features that vanish are one list read twice.
    ///
    /// Read off the COMMITTED document, which is the one the edits
    /// apply to; the delete op is one of the table's refusals
    /// ([`SessionOp::permitted_during_value_gesture`]) while a gesture
    /// holds a scratch value, so the two never disagree at a live
    /// button.
    pub fn delete_affordance(&self, node: RecipeNodeId) -> DeleteAffordance {
        DeleteAffordance::of(self.committed_doc(), node)
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
    pub fn hover(&self) -> Option<&Hovered> {
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
                resolution: self.entity_resolution(&face.name),
            },
            Selection::Edge(edge) => Standing::Edge {
                edge: edge.clone(),
                resolution: self.entity_resolution(&edge.name),
            },
        }
    }

    /// One picked name's verdict against the landed run — the shipped
    /// `resolve` door, asked once and spelled once for both entity
    /// kinds.
    fn entity_resolution(&self, name: &StableName) -> Option<Box<Resolution>> {
        self.landed_pair()
            .map(|(doc, eval)| Box::new(resolve(RunCtx { doc, eval }, name)))
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

    /// The last locally-valid-range probe, with the field it was taken
    /// for and the unit it was searched in. `None` before any probe,
    /// and after every document change (`request_eval`'s discard).
    pub fn bounds(&self) -> Option<&BoundsReading> {
        self.bounds.as_ref()
    }

    /// The advisory-check report for the landed pair — findings in
    /// deterministic order, and the residents that were configured
    /// `Off`. `None` before anything lands, or when the registry
    /// refused (which is distinct from an empty report: "not checked"
    /// and "checked and fine" are different answers).
    pub fn checks(&self) -> Option<&ChecksReport> {
        self.landed_checks.as_ref()
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

    /// The property rows as the panel LAYS THEM OUT — [`Self::slot_rows`]
    /// folded so that the three components of a 3-vector arrive as one
    /// group (`props::group_rows`).
    ///
    /// A second door rather than a replacement because the two answer
    /// different questions: a test asserting what a node's slots are
    /// wants the flat list, and a panel deciding how many lines to draw
    /// wants this. Both are the same rows — the grouping only bundles.
    pub fn slot_groups(&self) -> Vec<props::SlotGroup> {
        props::group_rows(self.slot_rows())
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
        // The advisory registry, same spot and same reason. It REPORTS
        // — a document with findings still draws, which is the whole
        // point of running it on the draw path: a product whose roots
        // interpenetrate renders a picture that looks almost right,
        // and the finding is the only thing that says otherwise.
        // A refusal of the registry itself leaves no report rather
        // than a clean one: "not checked" is not "checked and fine".
        self.landed_checks = run_checks(
            self.requested_doc.as_ref(),
            &done.evaluation,
            &ChecksConfig::default(),
            self.tol,
        )
        .ok();
        self.landed = Some(done.evaluation);
        self.landed_doc = Some(Arc::clone(&self.requested_doc));
        Landing::Landed
    }

    /// Perform one operation.
    ///
    /// The mid-gesture policy is applied ONCE, here, off
    /// [`SessionOp::permitted_during_value_gesture`] — no arm below
    /// carries a guard against the VALUE gesture of its own, so the set
    /// of operations a slot or parameter drag refuses is the table and
    /// only the table. The free-move arms do carry a guard, against
    /// their own gesture: they delegate to [`DisplayState`], which
    /// refuses [`crate::display::DisplayFault::FreeMoveInFlight`] off the free-move
    /// state this check never reads.
    pub fn perform(&mut self, op: SessionOp) -> OpOutcome {
        if self.gesture.is_some() && !op.permitted_during_value_gesture() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        match op {
            SessionOp::Select(selection) => {
                self.selection = selection;
                OpOutcome::default()
            }
            SessionOp::Hover(hover) => {
                self.hover = hover;
                OpOutcome::default()
            }
            SessionOp::DeleteNode { node } => self.delete_node(node),
            SessionOp::SetSlot { node, slot, value } => self.set_slot(node, slot, value),
            SessionOp::ProbeBounds { target } => self.probe_bounds(target),
            SessionOp::SetSlotUnit { node, slot, unit } => self.set_slot_unit(node, slot, unit),
            SessionOp::SetSlotExpression { node, slot, text } => {
                self.set_slot_expression(node, slot, &text)
            }
            SessionOp::SetParam { name, value } => self.set_param(&name, value),
            SessionOp::CreateParam { name, value } => self.create_param(name, value),
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
            } => self.commit(DocEdit::InsertNode {
                node: Node::Mate {
                    a,
                    b,
                    class,
                    alignment,
                },
            }),
            SessionOp::NewDocument { name } => self.new_document(&name),
            SessionOp::AddDatum { datum } => self.add_datum(datum),
            SessionOp::AddProfile { plane, loops } => self.add_profile(plane, loops),
            SessionOp::AddExtrude { profile, distance } => self.add_extrude(profile, distance),
            SessionOp::AddRevolve {
                profile,
                axis,
                angle,
            } => self.add_revolve(profile, axis, angle),
            SessionOp::AddBoolean { op, a, b } => self.add_boolean(op, a, b),
            SessionOp::AddSplit { target, tool } => self.add_split(target, tool),
            SessionOp::AddTransform {
                input,
                translation,
                rotation_axis,
                rotation_angle,
            } => self.add_transform(input, translation, rotation_axis, rotation_angle),
            SessionOp::AddPattern { input, count, rule } => {
                self.add_pattern(input, count, rule, PatternOutputChoice::Instances)
            }
            SessionOp::AddPlacedUnion { input, count, rule } => {
                self.add_pattern(input, count, rule, PatternOutputChoice::Fused)
            }
            SessionOp::AddFillet {
                target,
                radius,
                selection,
            } => self.add_blend(target, radius, selection, BlendKindChoice::Fillet),
            SessionOp::AddChamfer {
                target,
                distance,
                selection,
            } => self.add_blend(target, distance, selection, BlendKindChoice::Chamfer),
            SessionOp::AddInstance { id } => self.add_instance(id),
        }
    }

    /// The documents the open document's own directory offers as
    /// parts — the `Add part…` chooser's listing, as a value.
    ///
    /// # Errors
    ///
    /// [`Refusal::NoDocumentDirectory`] for a session with no backing
    /// file (there is no store to list), and [`Refusal::Workspace`]
    /// carrying the scan's own refusal — which is where a duplicate id
    /// or an unreadable sibling surfaces, at the chooser rather than
    /// at a tree badge, since no node exists yet to badge.
    pub fn part_catalogue(&self) -> Result<Vec<parts::PartEntry>, Refusal> {
        let resolver = self
            .resolver
            .as_deref()
            .ok_or(Refusal::NoDocumentDirectory)?;
        parts::catalogue(resolver, self.committed_doc().id())
            .map_err(|error| Refusal::Workspace(Box::new(error)))
    }

    /// Insert an instance of the part `id` names, minting its
    /// reference through the store: identity as asked for, version
    /// from the directory's content NOW.
    ///
    /// The store is reached through the resolver, which is the
    /// directory rule's home ([`DirResolver::workspace`]) — the same
    /// object every resolution consults, so the door a reference is
    /// authored through and the door it is later resolved through
    /// cannot come apart.
    ///
    /// The pin read is a full load of the referenced file (the store's
    /// own door), so a part that does not load refuses HERE — before a
    /// node exists — rather than as an unresolvable instance the user
    /// then has to delete.
    ///
    /// **Identity is checked before the directory**, deliberately: a
    /// document is its own document wherever its file lives, so the
    /// self-instance refusal is the one that survives saving, and
    /// naming the recoverable problem first would send a user off to
    /// save for nothing.
    fn add_instance(&mut self, id: DocumentId) -> OpOutcome {
        if let Some(refusal) = Refusal::self_instance(self.committed_doc().id(), id) {
            return OpOutcome::refused(refusal);
        }
        let Some(resolver) = self.resolver.as_deref() else {
            return OpOutcome::refused(Refusal::NoDocumentDirectory);
        };
        let pin = match resolver
            .workspace()
            .and_then(|ws| ws.current_pin(id, self.tol))
        {
            Ok(pin) => pin,
            Err(error) => return OpOutcome::refused(Refusal::Workspace(Box::new(error))),
        };
        self.commit(DocEdit::InsertNode {
            node: Node::instantiate_part(DocRef { id, pin }),
        })
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
        if let Err(refusal) = self.guard_driven(node, slot) {
            return OpOutcome::refused(refusal);
        }
        let unit = props::slot_unit(self.committed_doc(), node, slot);
        match props::slot_edit(node, slot, value, unit) {
            Ok(edit) => self.commit(edit),
            Err(error) => OpOutcome::refused(Refusal::Dimension(error)),
        }
    }

    /// Take one locally-valid-range probe for `target` and keep the
    /// reading.
    ///
    /// The search itself is [`probe::probe_bounds`]; what this door
    /// adds is the driven-slot guard, the document and evaluation the
    /// search is taken against, and the store.
    fn probe_bounds(&mut self, target: BoundsTarget) -> OpOutcome {
        // A driven slot is not a field the user can put a number into,
        // so a range of numbers for it is not an answer to any question
        // they can act on: the probe refuses it with the same
        // affordance the write and the drag do, which names the
        // parameters to probe instead. A parameter has no driver and
        // reaches this door unguarded.
        if let BoundsTarget::Slot { node, slot } = target
            && let Err(refusal) = self.guard_driven(node, slot)
        {
            return OpOutcome::refused(refusal);
        }
        // Read off `base` — the document the samples below are applied
        // to — and not off the session again. One document answers
        // where the search starts, what it steps by, and what every
        // candidate is judged against, so a probe cannot seed from one
        // document and search another.
        let base = self.doc().clone();
        let prior = self.landed.clone();
        let resolver = self
            .resolver
            .as_ref()
            .map(|ws| Arc::clone(ws) as Arc<dyn PartResolver>);
        match probe::probe_bounds(&base, target, prior.as_deref(), &resolver, self.tol) {
            Ok(reading) => {
                self.bounds = Some(reading);
                OpOutcome::default()
            }
            Err(refusal) => OpOutcome::refused(refusal),
        }
    }

    /// Rewrite a slot literal's display unit — the value stays put.
    ///
    /// No `guard_driven` here, and deliberately: the driven refusal
    /// protects a computed slot from being overwritten with a NUMBER,
    /// and this op writes no number. What a driven slot refuses is the
    /// narrower `SlotUnitFault::NotALiteral` the panel model raises —
    /// an expression has no authored notation to change.
    fn set_slot_unit(&mut self, node: RecipeNodeId, slot: SlotId, unit: UnitDef) -> OpOutcome {
        match props::slot_unit_edit(self.committed_doc(), node, slot, unit) {
            Ok(edit) => self.commit(edit),
            Err(fault) => OpOutcome::refused(Refusal::SlotUnit(fault)),
        }
    }

    fn set_slot_expression(&mut self, node: RecipeNodeId, slot: SlotId, text: &str) -> OpOutcome {
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

    /// The value door: write a declared parameter's value.
    ///
    /// A name the document does not declare takes the commit path so
    /// the typed refusal comes from the door rather than from here —
    /// `DocEdit::SetDocParamValue` carries an existing declaration
    /// forward and refuses `EditError::DocParamNotDeclared` when there
    /// is none.
    fn set_param(&mut self, name: &ParamName, value: SlotValue) -> OpOutcome {
        self.commit(props::param_edit(name.clone(), value))
    }

    /// The create door: refuse an already-declared name typed, commit
    /// the edit for a new one. See [`SessionOp::CreateParam`] for why
    /// this door narrows the edit's create-or-replace semantics.
    fn create_param(&mut self, name: ParamName, value: DocParam) -> OpOutcome {
        if let Some(existing) = self.committed_doc().params().get(&name) {
            return OpOutcome::refused(Refusal::ParamExists {
                dimension: existing.dim(),
                name,
            });
        }
        self.commit(DocEdit::SetDocParam { name, value })
    }

    fn begin_gesture(&mut self, node: RecipeNodeId, slot: SlotId) -> OpOutcome {
        if let Err(refusal) = self.guard_driven(node, slot) {
            return OpOutcome::refused(refusal);
        }
        let unit = props::slot_unit(self.committed_doc(), node, slot);
        self.start(GestureTarget::Slot { node, slot, unit })
    }

    fn begin_param_gesture(&mut self, name: &ParamName) -> OpOutcome {
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
                // **The display layer's identity, held rather than
                // argued.** Every display predicate is a function of
                // the node graph, and the free-move probe is admitted
                // against the COMMITTED document while the view and
                // the panel's own admission test resolve against this
                // scratch — so the two agree only while a gesture's
                // edits leave the graph alone. This holds the half a
                // check can hold; the other half is that
                // [`GestureTarget::edit`] can produce nothing but
                // `SetParam`, `SetStructuralParam` and
                // `SetDocParamValue`, none of which removes a node
                // either.
                assert!(
                    applied.record.minted.is_none(),
                    "a value gesture's preview minted a node, which the display                      layer's admission tests are not re-run against"
                );
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

    /// Refused mid-gesture — the table's answer
    /// ([`SessionOp::permitted_during_value_gesture`]), shared with
    /// [`SessionOp::NewDocument`]: both replace the document a drag is
    /// previewing against, and a gesture silently dissolved under the
    /// pointer is the kind of half-acted state that refusal exists to
    /// prevent.
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
                // The landed run answered the PREVIOUS document. Left
                // in place it would render the old model's tree and
                // resolve the new document's names against it until
                // the first run lands.
                self.landed = None;
                self.landed_doc = None;
                self.landed_fault = None;
                self.landed_at_rest = None;
                self.landed_checks = None;
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

    /// Replace the session with a fresh empty document — the
    /// [`SessionOp::NewDocument`] semantics (see that arm's docs for
    /// the identity ruling and what is cleared).
    ///
    /// The same clearing walk as [`DocSession::open`], with the two
    /// file-shaped fields going the other way: no path and no
    /// resolver, because nothing backs this document until it is
    /// saved.
    fn new_document(&mut self, name: &str) -> OpOutcome {
        let name = name.trim();
        if name.is_empty() {
            return OpOutcome::refused(Refusal::EmptyName);
        }
        self.history = History::new(Doc::empty_derived(name, self.tol));
        self.selection = Selection::None;
        self.hover = None;
        self.landed = None;
        self.landed_doc = None;
        self.landed_fault = None;
        self.landed_at_rest = None;
        self.landed_checks = None;
        self.landed_generation = None;
        self.scratch = None;
        self.path = None;
        self.resolver = None;
        self.display.clear();
        self.request_eval();
        OpOutcome::default()
    }

    /// Insert one datum node with literal slots
    /// ([`SessionOp::AddDatum`]).
    fn add_datum(&mut self, datum: DatumSpec) -> OpOutcome {
        self.commit(DocEdit::InsertNode {
            node: datum_node(datum),
        })
    }

    /// Insert one profile node ([`SessionOp::AddProfile`]).
    ///
    /// No loop-count or nesting judgment here: an empty list, a
    /// degenerate loop and a non-nested pair all go to `commit`, where
    /// the edit door's authoring-time check refuses them typed in the
    /// profile layer's own words — the one rule authored and
    /// hand-written programs share.
    fn add_profile(&mut self, plane: RecipeNodeId, loops: Vec<LoopProgram>) -> OpOutcome {
        // The plane is a PICK now, so it is gated where every other
        // pick is: at this door, by kind, before the edit. Without
        // this the reference would reach evaluation and refuse there
        // — a typed refusal either way, but one the person gets after
        // the node lands rather than instead of it.
        if let Err(refusal) = self.require_kind(plane, NodeKindWanted::Frame) {
            return OpOutcome::refused(refusal);
        }
        self.commit(DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram { plane, loops }),
        })
    }

    /// Insert one extrude of an existing profile
    /// ([`SessionOp::AddExtrude`]).
    fn add_extrude(&mut self, profile: RecipeNodeId, distance: Expr) -> OpOutcome {
        if let Err(refusal) = self.require_kind(profile, NodeKindWanted::Profile) {
            return OpOutcome::refused(refusal);
        }
        self.commit(DocEdit::InsertNode {
            node: Node::Extrude { profile, distance },
        })
    }

    /// Insert one revolve of an existing profile about an existing
    /// axis datum ([`SessionOp::AddRevolve`]).
    fn add_revolve(&mut self, profile: RecipeNodeId, axis: RecipeNodeId, angle: Expr) -> OpOutcome {
        if let Err(refusal) = self.require_kind(profile, NodeKindWanted::Profile) {
            return OpOutcome::refused(refusal);
        }
        if let Err(refusal) = self.require_kind(axis, NodeKindWanted::SketchAxis) {
            return OpOutcome::refused(refusal);
        }
        self.commit(DocEdit::InsertNode {
            node: Node::Revolve {
                profile,
                axis,
                angle,
            },
        })
    }

    /// Insert one regularized boolean of two existing bodies
    /// ([`SessionOp::AddBoolean`]).
    fn add_boolean(&mut self, op: BooleanOp, a: RecipeNodeId, b: RecipeNodeId) -> OpOutcome {
        for seat in [a, b] {
            if let Err(refusal) = self.require_kind(seat, NodeKindWanted::Body) {
                return OpOutcome::refused(refusal);
            }
        }
        // AFTER the kind gate, so a self-boolean of two profiles is
        // reported as "that is not a body" — the fact the user can act
        // on — rather than as the narrower complaint about the pair.
        if a == b {
            return OpOutcome::refused(Refusal::SelfBoolean { node: a });
        }
        self.commit(DocEdit::InsertNode {
            node: Node::Boolean {
                op,
                a,
                b,
                declare: None,
            },
        })
    }

    /// Insert one split of an existing body by an existing datum plane
    /// ([`SessionOp::AddSplit`]).
    fn add_split(&mut self, target: RecipeNodeId, tool: RecipeNodeId) -> OpOutcome {
        if let Err(refusal) = self.require_kind(target, NodeKindWanted::Body) {
            return OpOutcome::refused(refusal);
        }
        if let Err(refusal) = self.require_kind(tool, NodeKindWanted::Plane) {
            return OpOutcome::refused(refusal);
        }
        self.commit(DocEdit::InsertNode {
            node: Node::Split { target, tool },
        })
    }

    /// Insert one rigid placement of an existing body
    /// ([`SessionOp::AddTransform`]).
    fn add_transform(
        &mut self,
        input: RecipeNodeId,
        translation: [Expr; 3],
        rotation_axis: [Expr; 3],
        rotation_angle: Expr,
    ) -> OpOutcome {
        if let Err(refusal) = self.require_kind(input, NodeKindWanted::Body) {
            return OpOutcome::refused(refusal);
        }
        self.commit(DocEdit::InsertNode {
            node: combine::transform_node(input, translation, rotation_axis, rotation_angle),
        })
    }

    /// Insert one pattern of an existing body, fused or not
    /// ([`SessionOp::AddPattern`], [`SessionOp::AddPlacedUnion`]).
    ///
    /// **One function for the two ops**, for `add_blend`'s reason: the
    /// prototype seat, the axis seat a circular
    /// rule adds and the commit are the same move for both, and the
    /// only difference — which node is minted — is one match below
    /// where a reader can see the pair side by side.
    fn add_pattern(
        &mut self,
        input: RecipeNodeId,
        count: i64,
        rule: PatternRuleSpec,
        output: PatternOutputChoice,
    ) -> OpOutcome {
        if let Err(refusal) = self.require_kind(input, NodeKindWanted::Body) {
            return OpOutcome::refused(refusal);
        }
        if let PatternRuleSpec::Circular { axis, .. } = rule
            && let Err(refusal) = self.require_kind(axis, NodeKindWanted::Axis)
        {
            return OpOutcome::refused(refusal);
        }
        let node = match output {
            PatternOutputChoice::Instances => combine::pattern_node(input, count, rule),
            PatternOutputChoice::Fused => combine::placed_union_node(input, count, rule),
        };
        self.commit(DocEdit::InsertNode { node })
    }

    /// Insert one blend — fillet or chamfer — on a set of an existing
    /// body's edges ([`SessionOp::AddFillet`],
    /// [`SessionOp::AddChamfer`]).
    ///
    /// **One function for the two ops**, because everything a door
    /// does is the same for both: the same body seat, the same Length
    /// literal, the same commit. What
    /// differs is which node is minted and which slot the size lands
    /// in, and that difference is `kind`'s alone — spelled once in the
    /// match below, where a reader can see the two side by side
    /// instead of comparing two near-identical functions for the line
    /// that is not the same.
    fn add_blend(
        &mut self,
        target: RecipeNodeId,
        size: Expr,
        selection: Vec<StableName>,
        kind: BlendKindChoice,
    ) -> OpOutcome {
        if let Err(refusal) = self.require_kind(target, NodeKindWanted::Body) {
            return OpOutcome::refused(refusal);
        }
        // The CANONICALIZING constructors, never the struct literals:
        // canonical form is what makes two recipes over the same edges
        // bit-identical, and `persist`'s strict door treats a
        // non-canonical set on the wire as a corrupt file.
        let node = match kind {
            BlendKindChoice::Fillet => Node::fillet(target, size, selection),
            BlendKindChoice::Chamfer => Node::chamfer(target, size, selection),
        };
        self.commit(DocEdit::InsertNode { node })
    }

    /// The node-kind gate every creation seat shares: the named node
    /// must be the wanted kind in the committed document — absent and
    /// wrong-kind refuse the same arm, because both mean "there is
    /// nothing of that kind there to consume".
    fn require_kind(&self, node: RecipeNodeId, wanted: NodeKindWanted) -> Result<(), Refusal> {
        if admits(self.committed_doc().node(node), wanted) {
            Ok(())
        } else {
            Err(Refusal::WrongNodeKind { node, wanted })
        }
    }

    /// **The one door an edit enters the document through**: apply,
    /// record, re-evaluate — and reconcile the display state, which is
    /// where a free-move probe is superseded by the mate that
    /// constrains its instance (the prune DISCARDS the value; see
    /// `display`'s module docs for why discard and not zero).
    fn commit(&mut self, edit: DocEdit<ProfileProgram>) -> OpOutcome {
        self.commit_action(vec![edit])
    }

    /// **Delete a feature and everything downstream of it**, as one
    /// action and therefore one undo.
    ///
    /// A recipe node that anything consumes cannot leave the document
    /// alone — `DocEdit::DeleteNode` refuses a delete that would
    /// dangle a live reference, and in a chain-shaped model (a boolean
    /// per feature) that is every node but the last. The whole
    /// dependent cone therefore goes, consumers first, which is the
    /// order [`cascade_delete_order`] hands back and the order the
    /// door accepts.
    ///
    /// **The cone, not the chain's remainder**: nodes whose only tie
    /// to the target is that they fed it survive as roots of their own,
    /// so deleting one pip's boolean out of a die leaves that pip's
    /// body in the document, unconsumed. Reconnecting a deleted node's
    /// consumers to its input instead — splice, the CAD-conventional
    /// delete — needs an edit that rewires a live node's inputs. The
    /// vocabulary has ONE such edit now, `DocEdit::SetMembers`, and it
    /// reaches only the LIST inputs (a `Union`'s members, a `Loft`'s
    /// sections): a splice has to rewire a NAMED operand, which nothing
    /// here does. Still open as issue #1324.
    ///
    /// An id the document does not hold takes the single-edit path so
    /// the typed refusal comes from the door rather than from here.
    fn delete_node(&mut self, node: RecipeNodeId) -> OpOutcome {
        let doomed = cascade_delete_order(self.committed_doc(), node);
        if doomed.is_empty() {
            return self.commit(DocEdit::DeleteNode { id: node });
        }
        self.commit_action(
            doomed
                .into_iter()
                .map(|id| DocEdit::DeleteNode { id })
                .collect(),
        )
    }

    /// The same door for an action that takes SEVERAL edits: apply
    /// them in order, and record the whole run as one history state,
    /// so one user action is one undo.
    ///
    /// **All or nothing**: each edit is applied to the value the last
    /// one produced and nothing is recorded until every one has
    /// succeeded, so a refusal anywhere leaves the session on the
    /// document it started from. That is purity doing the work — no
    /// rollback exists to be got wrong.
    fn commit_action(&mut self, edits: Vec<DocEdit<ProfileProgram>>) -> OpOutcome {
        // Threaded rather than cloned up front: the first `apply`
        // reads the history's value in place, and each later one reads
        // its predecessor's output, so a group of one costs exactly
        // what a single commit always cost.
        assert!(!edits.is_empty(), "an action commits at least one edit");
        let mut produced: Option<Doc<ProfileProgram>> = None;
        for edit in &edits {
            let attempt = {
                let base = produced.as_ref().unwrap_or_else(|| self.history.doc());
                apply(base, edit, self.tol)
            };
            match attempt {
                Ok(applied) => produced = Some(applied.doc),
                Err(error) => return OpOutcome::refused(Refusal::Edit(Box::new(error))),
            }
        }
        let Some(doc) = produced else {
            unreachable!("the loop applied at least one edit and kept its output")
        };
        self.history.commit_group(edits.clone(), doc);
        let superseded = self.display.prune(self.history.doc());
        self.request_eval();
        OpOutcome {
            committed: edits,
            superseded,
            ..OpOutcome::default()
        }
    }

    /// **The one submit**: mint the next generation and hand the shown
    /// document to the seam.
    fn request_eval(&mut self) {
        // **Every route that changes the shown document passes here**,
        // which is why the range probe is discarded here and nowhere
        // else: a commit, a gesture preview, an undo, an open. The one
        // caller that does not change the document is `Reevaluate`, and
        // discarding for it too is the conservative direction — a range
        // recomputed on request costs a button press, a stale one costs
        // a wrong decision.
        self.bounds = None;
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
