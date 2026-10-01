//! The document session: everything the panels operate on, and the
//! typed operations that operate on it.
//!
//! # What this module is allowed to contain
//!
//! The driver, and nothing else. `session` owns [`DocSession`] and
//! dispatches [`SessionOp`]; what stays here is that state, its
//! `Gesture`, its [`Derived`] block with the [`LandedRun`] inside it,
//! [`Landing`], [`AtRestBadge`], [`Outstanding`],
//! [`DocSession::perform`]
//! and the operation doors — every door mutates the session, and
//! `perform`'s dispatch is the one place an operation becomes state.
//! The three values are what the session says about itself and are
//! minted nowhere else.
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
    Assembly, AssemblyError, BooleanOp, ChecksConfig, ChecksReport, Dimension, DimensionError, Doc,
    DocEdit, DocParam, DocParamValue, DocRef, DocumentId, EvalOptions, Evaluation, Expr,
    LoopProgram, Maintenance, MaintenanceNet, Node, ParamName, PartReach, PartResolver,
    ProductError, ProfileProgram, RecipeNodeId, SlotId, StepId, Subject, apply, assemble_gathered,
    cascade_delete_order, parse_expr, product_recorded, run_checks_on,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::quantity::UnitDef;
use pncad::select::{FlushFinding, Resolution, RunCtx, declare_node, resolve};
use pncad::topo::Body;

use crate::blend::BlendKindChoice;
use crate::combine::{self, DuplicateFault, PatternOutputChoice};
use crate::display::{DisplayFault, DisplayState, DisplayView};
use crate::docio::{self, DirResolver, NoFile};
use crate::evalseam::{EvalRequest, EvalService, InlineEvaluator, evaluate_beside};
use crate::g1;
use crate::generation::Generation;
use crate::history::History;
use crate::parts::{self, PartFiles};
use crate::pickcache;
use crate::props::{self, Notation, SlotDriver, SlotValue};
use crate::sketch;
use crate::tree::{self, TreeRow};

pub mod author;
pub mod delete;
pub mod op;
pub mod probe;
pub mod refuse;
pub mod select;

pub use author::{DatumSpec, PartSelectSpec, PatternRuleSpec, ProfilePlane, ProfileShape};
pub use delete::DeleteAffordance;
pub use op::{CancelDoor, FreeMoveName, GestureName, OpOutcome, SessionOp, ValueGestureName};
pub use probe::{BoundsReading, BoundsTarget};
pub use refuse::{
    DeclareOffer, FaceFrameFault, NO_FACE_PICKED, NodeKindWanted, Refusal, RefusedBoolean, Step,
    VersionOffer, admits, face_frame_seat, face_frame_seat_drawn,
};
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
    /// [`SlotValue::of`] — the one home for that rule, refusal
    /// included.
    ///
    /// # Errors
    ///
    /// [`SlotValue::of`]'s, which is `Expr::literal`'s own
    /// finiteness refusal reached for a `Count` target, where the
    /// literal door is not on the path.
    fn value_of(&self, value: f64) -> Result<SlotValue, pncad::document::DimensionError> {
        SlotValue::of(self.dimension(), value)
    }

    /// What an operation has to name to drive this gesture: the
    /// subject half of this target, without the facts the begin
    /// looked up.
    ///
    /// A target carries the display unit or the declared dimension
    /// its begin read off the base document; an operation arriving
    /// from the chrome carries neither and has no business asserting
    /// them. So the comparison that decides whether a preview belongs
    /// to the open gesture is over [`ValueGestureName`], and this is
    /// the one place a target becomes one.
    fn name(&self) -> ValueGestureName {
        match self {
            Self::Slot { node, slot, .. } => ValueGestureName::Slot {
                node: *node,
                slot: *slot,
            },
            Self::Param { name, .. } => ValueGestureName::Param(name.clone()),
        }
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

/// What a value gesture holds for its life: layer-3 state only.
///
/// The value in flight is not here — it is [`g1::Slot`]'s, along with
/// the three rules that move it.
#[derive(Debug)]
struct Gesture {
    target: GestureTarget,
    /// The document the previews are applied to — the history's
    /// current value, held so each preview replaces the last rather
    /// than stacking.
    base: Doc<ProfileProgram>,
}

/// **The value drag's words for the three G1 states**, declared once
/// and handed to [`g1::Slot`] at every door.
///
/// The machine is shared with the free-move probe and the
/// vocabularies are not: these three sentences are about a field's
/// drag, and the probe's three are about an instance's placement
/// ([`crate::display::DisplayFault`]).
fn gesture_words() -> g1::Refusals<Refusal> {
    g1::Refusals {
        none: Refusal::NoGesture,
        in_flight: Refusal::GestureInFlight,
        wrong: Refusal::WrongGesture,
    }
}

/// The slot's driver and current value, or the refusal that says the
/// slot is not there.
///
/// Over a document rather than over the session, so a begin's target
/// check can run inside [`g1::Slot::begin`]'s closure, where the
/// session's gesture field is already borrowed.
fn driver_of(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    slot: SlotId,
) -> Result<(SlotDriver, Option<SlotValue>), Refusal> {
    let row = props::slot_rows(doc, node)
        .into_iter()
        .find(|row| row.slot == slot)
        .ok_or(Refusal::NoSuchSlot { node, slot })?;
    Ok((row.driver, row.value.ok()))
}

/// Refuse a numeric edit to a driven slot, with the affordance.
fn guard_driven(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    slot: SlotId,
    notation: Notation,
) -> Result<(), Refusal> {
    let (driver, current) = driver_of(doc, node, slot)?;
    match driver {
        SlotDriver::Literal => Ok(()),
        SlotDriver::Expression { params } => Err(Refusal::DrivenByExpression {
            node,
            slot,
            params,
            current,
            notation,
        }),
    }
}

/// **A profile editor's program with every unmoved argument of every
/// kept step carried from the committed program** — the argument as
/// the document holds it, so an argument the editor re-minted in its
/// own notation but did not move keeps the notation it was written
/// in, and only what moved is written in the editor's.
///
/// A step is the committed one when `ids` keeps its id; the argument
/// is the same one when it has the same
/// [`pncad::document::StepArg`] on that step, and unmoved when its
/// value is the committed one at the bits ([`Expr::bit_eq`]).
///
/// # Errors
///
/// [`Refusal::DrivenByExpression`], with the affordance, for an
/// argument of a kept step that an expression drives and that the
/// editor's program does not hold unmoved: a number written over a
/// computation, exactly as the slot field refuses it. A step the
/// program drops takes its arguments with it, driven or not — that is
/// not a write over them.
///
/// An id that is not the committed program's, or `ids` of the wrong
/// shape, carries nothing here: the edit door refuses those typed
/// ([`pncad::document::EditError::StepIdsRefused`]).
fn carry_unmoved(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    current: &ProfileProgram,
    loops: Vec<LoopProgram>,
    ids: &[Vec<Option<StepId>>],
    notation: Notation,
) -> Result<Vec<LoopProgram>, Refusal> {
    let was: std::collections::HashMap<StepId, (u32, u32)> = current
        .ids
        .iter()
        .enumerate()
        .flat_map(|(loop_, steps)| {
            steps
                .iter()
                .enumerate()
                .map(move |(step, id)| (*id, (program_index(loop_), program_index(step))))
        })
        .collect();
    let was = &was;
    let now: std::collections::HashMap<(u32, u32), (u32, u32)> = ids
        .iter()
        .enumerate()
        .flat_map(|(loop_, steps)| {
            steps.iter().enumerate().filter_map(move |(step, kept)| {
                let at = (program_index(loop_), program_index(step));
                was.get(&(*kept)?).map(|&was| (was, at))
            })
        })
        .collect();
    let old = Node::Profile(current.clone());
    let mut new = Node::Profile(ProfileProgram {
        plane: current.plane,
        loops,
        ids: Vec::new(),
    });
    for slot in old.slots() {
        let SlotId::Profile { loop_, step, arg } = slot else {
            unreachable!(
                "a profile node lists only profile slots, and {} is not one",
                slot.label()
            )
        };
        // A step the program drops: nothing of it is written.
        let Some(&(new_loop, new_step)) = now.get(&(loop_, step)) else {
            continue;
        };
        let Some(committed) = old.expr(slot) else {
            unreachable!(
                "`Node::slots` is the domain of `Node::expr`, and {} was listed by it",
                slot.label()
            )
        };
        let moved = SlotId::Profile {
            loop_: new_loop,
            step: new_step,
            arg,
        };
        match new.expr_mut(moved) {
            Some(held) if held.bit_eq(committed) => *held = committed.clone(),
            _ if committed.literal_value().is_some() => {}
            _ => guard_driven(doc, node, slot, notation)?,
        }
    }
    let Node::Profile(program) = new else {
        unreachable!("the carried program was built as a profile node")
    };
    Ok(program.loops)
}

/// A position in a profile program as the slot vocabulary spells it.
fn program_index(i: usize) -> u32 {
    let Ok(narrowed) = u32::try_from(i) else {
        unreachable!("a collection of {i} elements does not fit in memory")
    };
    narrowed
}

/// The whole of what the document panels operate on.
pub struct DocSession {
    history: History,
    tol: Tol,
    gesture: g1::Slot<Gesture, SlotValue>,
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
    /// The resolver that request resolves through — the landed run's
    /// memo is only a memo for a run under the same one
    /// ([`DocSession::memo_under`]).
    requested_resolver: Option<Arc<dyn PartResolver>>,
    /// Everything this session knows *because of* the document under
    /// it — one value, so that replacing that document is one
    /// assignment ([`DocSession::clear_for_new_document`]).
    derived: Derived,
    path: Option<PathBuf>,
    /// Layer-3 display state — hide and free-move — with its one home
    /// here (the seam-friction inventory rule). Never persisted; reset
    /// by `Open`; pruned against every new document value.
    display: DisplayState,
    /// The document seam: the opened file's own directory, consulted
    /// lazily (the directory rule and the scan-at-resolution posture
    /// are [`DirResolver`]'s docs). A session over an in-memory
    /// document carries none, and its instantiate nodes refuse through
    /// [`NoFile`]. Replaced — never inherited — on every `Open`, so a
    /// document can never silently resolve against the previous
    /// document's directory.
    resolver: Option<Arc<DirResolver>>,
    /// The working notation ([`props::Notation`]): what a value nobody
    /// wrote reads in, here because the refusals this session answers
    /// read such values. Per person and never in
    /// the document, so no `Open` or new document resets it.
    notation: Notation,
}

/// What the session knows because of the document under it: what is
/// picked out of it, what a drag is previewing over it, what the last
/// run said about it, and what a range probe found in it.
///
/// **One value, because replacing the document invalidates all of it
/// at once.** `Open` and `NewDocument` install a different document
/// under a live session, and every field here is a statement about
/// the document that was there before — left in place they answer
/// about the previous model until the first run lands, which is the
/// stale badge these two doors exist to prevent. Held together,
/// [`Derived::none`] is the ONE spelling of "nothing is known yet":
/// the constructor's value, both doors' value, and what
/// [`DocSession::land`] overwrites a part of. A field added here is
/// therefore reset by being declared, not by being remembered at
/// three call sites.
///
/// **Three neighbours are outside it, each for a reason absorbing
/// them would destroy.**
///
/// - [`DocSession::display`] is reset by [`DisplayState::clear`], not
///   by reconstruction: its revision counter is the chrome's "does
///   the drawn scene need rebuilding" key and is monotonic across the
///   reset, so a fresh value would send it backwards and a scene
///   built under the old count would read as current. Its own
///   `clear` closes the same hazard inside it.
/// - [`DocSession::gesture`] is cleared by nothing, and must not be.
///   The refusal runs the other way round: while a value drag is in
///   flight the DOOR is refused — `Open` and `NewDocument` are two of
///   the rows [`SessionOp::permitted_during_value_gesture`] says no
///   to, checked once in [`DocSession::perform`] — and the drag is
///   left untouched, because a gesture silently dissolved under the
///   pointer is the half-acted state that refusal exists to prevent.
///   A walk that cleared it would encode the opposite policy. The
///   precondition is therefore ESTABLISHED at `perform`, before a
///   door writes anything, which is also why neither door restates it
///   and why nothing here re-checks it: a check inside this function
///   could only fire with the session already half-replaced.
///
///   **That table governs VALUE gestures only, and there is a
///   second.** The free-move drag [`DisplayState`] owns is a
///   different value with a different owner, so it gets its own row
///   list ([`SessionOp::permitted_during_free_move`]) rather than a
///   widened one — the two drags refuse different sets, and a single
///   table could only serve both by refusing the union. What the two
///   agree on is exactly these two doors: a replacement drops the
///   display state whole, so both are refused while either drag is
///   open, and neither is ever dissolved under the pointer.
/// - `path` and `resolver` are facts about the backing FILE rather
///   than about the document, and are the part of the two doors that
///   genuinely differs: `Open` sets both, `NewDocument` clears both.
struct Derived {
    selection: Selection,
    /// What the cursor is over: transient, never persisted, and its
    /// ONE home. A widget that kept its own copy would be the
    /// per-widget shadow the panels' inventory discipline forbids.
    hover: Option<Hovered>,
    /// The document the panels show in place of the history's current
    /// value while a gesture previews against it — `Some` only while
    /// [`DocSession::gesture`] is `Some`, which is what makes clearing
    /// it in `clear_for_new_document` a no-op rather than half a
    /// dissolved drag.
    scratch: Option<Doc<ProfileProgram>>,
    /// The last completed run and everything taken from it; `None`
    /// before anything has landed.
    landed: Option<LandedRun>,
    /// The last locally-valid-range probe, and the field it was taken
    /// for — layer-3 state, never persisted, and its one home.
    ///
    /// Kept rather than recomputed per frame because a probe costs tens
    /// of evaluations: it is taken when asked for
    /// ([`SessionOp::ProbeBounds`]) and DISCARDED, never repaired, the
    /// moment the document changes. A range is a statement about one
    /// document, and showing yesterday's range beside today's number is
    /// the class of stale-confident answer this crate's staleness rules
    /// exist to prevent. That discard is at every submit
    /// ([`DocSession::request_eval`]) and so is STRICTER than this
    /// value's own reset: a commit or an undo drops the probe without
    /// touching anything else here. Both doors therefore drop it
    /// twice, once with this value and again at the submit each ends
    /// with. The redundancy is deliberate — the walk names every
    /// field it invalidates rather than leaving one of them to a route
    /// a reader of the walk cannot see.
    bounds: Option<BoundsReading>,
}

impl Derived {
    /// Nothing known about the document underneath: nothing picked,
    /// nothing hovered, no preview, nothing landed, no range probed.
    fn none() -> Self {
        Self {
            selection: Selection::None,
            hover: None,
            scratch: None,
            landed: None,
            bounds: None,
        }
    }
}

/// Exhaustive by destructuring; every field is carried, so this walk
/// `finish`es. The rule the four walks share is one paragraph in
/// `crates/viewer/README.md` ("The dump is held to the same
/// declaration"), not restated here.
///
/// `scratch` is carried as its presence: it is a whole `Doc`, and that
/// one is in flight is the fact — it is `Some` exactly while
/// [`DocSession::gesture`] is. It renders as an ELISION —
/// `Some(<Doc>)`, never the `bool` the presence is — because `finish`
/// says only that every field is SHOWN, and whether the value shown is
/// the whole field is a question each summarised field answers for
/// itself.
impl core::fmt::Debug for Derived {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            selection,
            hover,
            scratch,
            landed,
            bounds,
        } = self;
        f.debug_struct("Derived")
            .field("selection", selection)
            .field("hover", hover)
            .field("scratch", &scratch.as_ref().map(|_| format_args!("<Doc>")))
            .field("landed", landed)
            .field("bounds", bounds)
            .finish()
    }
}

/// One completed evaluation, landed: the pair it answers and every
/// verdict taken for that pair.
///
/// **These move together or not at all.** Each is a statement about
/// ONE (document, evaluation) pair, taken from that pair's single
/// gather in [`DocSession::land`]; one field of this value read beside
/// another run's would describe a run that never happened. As one
/// value, `land` writes exactly what [`Derived::none`] clears, and a
/// new thing taken at landing cannot be computed at one door and
/// forgotten at the other.
///
/// The gather's outcome is one `Result`: a refusal carries nothing
/// the gather would have produced, so no A5 verdict and no body can
/// sit beside a gather refusal. That is all the shape rules out.
/// [`Gathered`] can still spell pairs only `land` keeps out — a
/// `Certified` badge with no body, a `Refused` one with a body.
struct LandedRun {
    evaluation: Arc<Evaluation<f64>>,
    /// The resolver [`LandedRun::evaluation`] resolved through.
    resolver: Option<Arc<dyn PartResolver>>,
    /// The document [`LandedRun::evaluation`] answers.
    ///
    /// **Resolution is a question about a PAIR.** `resolve` reads the
    /// recipe and the evaluation together, so asking it about the
    /// document as it stands now and the evaluation as it stood one
    /// edit ago is asking about a run that never happened — and the
    /// diagnosis it answers with would be about that non-run. The two
    /// move together here, one generation behind the shown document
    /// while a run is outstanding, which is exactly what the picture
    /// does.
    doc: Arc<Doc<ProfileProgram>>,
    /// The generation this run answered ([`DocSession::busy`] compares
    /// it against the one the session is waiting on).
    generation: Generation,
    /// What the gather left this pair ([`Gathered`]), or its refusal
    /// ([`DocSession::product_fault`]).
    gather: Result<Gathered, ProductError>,
    /// The advisory-check report for this pair
    /// ([`DocSession::checks`]); `None` when the registry itself
    /// refused, or did not run. It sits outside [`LandedRun::gather`]
    /// because one refusal still has a report: the one
    /// `ProductErrorKind::means_no_body` reads as an absence.
    checks: Option<ChecksReport>,
    /// **The part files the run's resolver could name** — one scan of
    /// the session's directory, taken at landing ([`PartFiles`]'s doc
    /// says why then): the file names the tree names this pair's
    /// instances and their carried lines by. Unscanned for a document that
    /// instantiates nothing, which never asks.
    files: PartFiles,
}

/// What a gather that succeeded left the landing ([`LandedRun::gather`]'s
/// `Ok`): the A5 verdict taken over the product, and the product's body
/// where the gate did not consume it.
struct Gathered {
    /// The A5 at-rest verdict for this pair ([`DocSession::at_rest`]);
    /// `None` where [`AtRestBadge`] says none is taken.
    at_rest: Option<AtRestBadge>,
    /// **The aggregate the landing's own gather produced**, kept so
    /// that a consumer which needs the product does not gather it a
    /// second time ([`DocSession::landed_body`]). Written once, by
    /// `land`.
    ///
    /// `None` when the A5 gate CONSUMED it. The gate takes the product
    /// by value, and its refusal is the one exit that does not hand
    /// the body back (a certification returns it on `Assembly`).
    /// Nothing is cloned to close that gap: on this lane's measurement
    /// a body clone is 2.7% of a gather but is paid per LANDING, while
    /// the gather it would save is paid per opened document — the
    /// wrong trade for an edit session. The display fit gathers its
    /// own, on the fit worker ([`DocSession::fit_request`]'s
    /// `Ungathered`).
    ///
    /// **What those numbers are load-bearing for, and why they carry
    /// no guard.** They chose between two designs that are both
    /// CORRECT — gather again where it is wanted, or clone at every
    /// gate — so nothing here breaks if the ratio drifts; what would
    /// break is the TRADE, and a trade is re-decided by re-measuring,
    /// not by a failing assertion. A wall-clock guard in the gate
    /// would be a flake rather than a witness, and a scheduled
    /// re-measure would be a standing chore over a number no
    /// behaviour reads.
    ///
    /// Half of it is re-taken anyway, and by someone else: the gather
    /// column of `editor-core/tests/m4_pr8_latency.rs` (`gather_ms`)
    /// runs on the nightly register, so the 87 ms side has a standing
    /// witness this crate does not maintain. What has none is the
    /// clone denominator — nothing in the tree times a `Body::clone`.
    ///
    /// **What this file's rows do and do not guard.**
    /// `viewer/tests/landing_gathers.rs` counts the gathers of every
    /// path that must not pay one, so a change that made
    /// [`DocSession::landed_body`] gather, or that stopped `land`
    /// keeping the body, reds there. It does NOT see the doors:
    /// restoring [`crate::scene::fit_delta`]'s or [`crate::scene::scene_of_body`]'s old
    /// pair-taking signatures reds nothing, because those gathers
    /// would run inside `scene` where no row counts. Re-measure before
    /// changing the shape; do not trust the figures to have stayed
    /// true.
    body: Option<Arc<Body<f64>>>,
}

/// Exhaustive by destructuring, through [`Gathered`] too; the shared
/// rule is `crates/viewer/README.md`'s.
///
/// The two `_` arms are the run's DATA — `evaluation` is the result
/// DAG and `doc` is the recipe DAG it answers — and everything else
/// here is a verdict ABOUT that pair. The gather renders as the arm it
/// took: its refusal as `fault`, or what it left as `at_rest` and
/// `body`. `checks` is a `Vec` per finding and is carried as its two
/// counts; `body` is a gathered aggregate and is carried as its
/// presence, which is whether the gate left it. Both render as
/// SUMMARIES — two counts, and the elision `Some(<Body>)` — so neither
/// can be read as the field's own value; `finish_non_exhaustive` here
/// is about the `_` arms and says nothing about them.
impl core::fmt::Debug for LandedRun {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            evaluation: _,
            resolver: _,
            doc: _,
            generation,
            gather,
            checks,
            files,
        } = self;
        let mut out = f.debug_struct("LandedRun");
        out.field("generation", generation);
        match gather {
            Ok(Gathered { at_rest, body }) => out
                .field("at_rest", at_rest)
                .field("body", &body.as_ref().map(|_| format_args!("<Body>"))),
            Err(fault) => out.field("fault", fault),
        };
        out.field("files", files);
        match checks {
            Some(report) => out.field(
                "checks",
                &format_args!(
                    "{} finding(s), {} skipped",
                    report.findings.len(),
                    report.skipped.len()
                ),
            ),
            None => out.field("checks", &Option::<()>::None),
        };
        out.finish_non_exhaustive()
    }
}

/// The A5 at-rest verdict for the landed pair — a mated document's
/// declarations run through the kernel's own verification door
/// (`assemble`), once per landed evaluation, so a committed mate's
/// class verdict does not die at the commit: a `Tangent` that solves
/// green still shows the gate refusing it, and an undeclared contact
/// between instances surfaces on the draw path instead of waiting for
/// an export.
///
/// Taken only when the gate ran: an assembly-shaped document (one
/// holding at least one `InstantiatePart`) whose product gathered. A
/// part document's tiers are not this badge's subject, and the gate's
/// cost is not spent where it answers nothing the badges do not
/// already say. A gather refusal, of any class, leaves no product for
/// the gate to judge, so it takes no badge: it is
/// [`DocSession::product_fault`]'s, and `frame::badge_site` routes it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AtRestBadge {
    /// The gate certified the assembled product; how many declarations
    /// its mates minted.
    Certified {
        /// The minted declaration count.
        minted: usize,
    },
    /// The gate's own refusal of the product it judged, in its own
    /// rendering, never a sentence composed here.
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

/// What the session owes at one moment: the picture against the
/// document, and the seam against the picture.
///
/// [`DocSession::busy`] and [`DocSession::running`] answer those two
/// separately and both stay, because each is useful alone. **Read
/// together they are one three-state fact**, and this is that fact as
/// a value — the only thing a consumer of it is handed
/// ([`DocSession::outstanding`] is the one site that reads both).
/// `crates/viewer/README.md`, The session's vocabularies, argues why.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outstanding {
    /// The picture answers the document the session holds: nothing is
    /// owed and nothing is running.
    Current,
    /// The picture is older than the document and the seam is working
    /// on it.
    Evaluating,
    /// The picture is older than the document and NOTHING is working
    /// on it — a cancel. [`SessionOp::Reevaluate`] is what recovers
    /// from it, and the state exists so the chrome does not spin over
    /// an idle seam forever.
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
            requested_resolver: None,
            history: History::new(doc),
            tol,
            gesture: g1::Slot::closed(),
            eval,
            generation: Generation::FIRST,
            derived: Derived::none(),
            path: None,
            display: DisplayState::new(),
            resolver: None,
            notation: Notation::DEFAULT,
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
        self.derived
            .scratch
            .as_ref()
            .unwrap_or_else(|| self.history.doc())
    }

    /// The document the history is on, ignoring any preview.
    pub fn committed_doc(&self) -> &Doc<ProfileProgram> {
        self.history.doc()
    }

    /// The edit history.
    pub fn history(&self) -> &History {
        &self.history
    }

    /// The working notation a value nobody wrote reads in
    /// ([`props::Notation`]), [`Notation::DEFAULT`] until one is set.
    pub fn notation(&self) -> Notation {
        self.notation
    }

    /// Read every value nobody wrote in `notation` from now on. Not an
    /// edit: no document changes and nothing enters the history.
    pub fn set_notation(&mut self, notation: Notation) {
        self.notation = notation;
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

    /// **The chrome's cancel doors, one per gesture** — the exits
    /// that are not the widget the gesture was opened on, and the only
    /// exits a gesture whose widget is no longer drawn has left
    /// ([`CancelDoor`] carries that argument).
    ///
    /// **Two, and the population is the operations that cancel a
    /// GESTURE** rather than everything the enum spells `Cancel`:
    /// [`SessionOp::CancelEvaluation`] cancels a run, not a gesture,
    /// and has its own control beside the spinner that reports the
    /// run. The census is held from the operation vocabulary's side by
    /// `crates/viewer/tests/gesture_table.rs`'s
    /// `every_gesture_cancel_has_a_chrome_door`, which names every
    /// `SessionOp`.
    ///
    /// Each door reads the state of its OWN gesture: this session's
    /// value drag, and [`crate::display::DisplayState::probing`] for
    /// the free-move probe. The two are independent and can be in
    /// flight together, so one control standing for both would have
    /// nothing to say about which it closed.
    pub fn cancel_doors(&self) -> [CancelDoor; 2] {
        [
            CancelDoor::of(
                "Cancel drag",
                SessionOp::CancelGesture,
                self.gesture.held().is_some(),
                Refusal::NoGesture,
            ),
            CancelDoor::of(
                "Cancel free-move",
                SessionOp::CancelFreeMove,
                self.display.probing().is_some(),
                Refusal::Display(DisplayFault::NoFreeMove),
            ),
        ]
    }

    /// The ε this session decides at.
    pub fn tol(&self) -> Tol {
        self.tol
    }

    /// The current selection.
    pub fn selection(&self) -> &Selection {
        &self.derived.selection
    }

    /// What the cursor is over, if anything.
    pub fn hover(&self) -> Option<&Hovered> {
        self.derived.hover.as_ref()
    }

    /// Whether the selection still denotes something in the evaluation
    /// on screen.
    ///
    /// **A pure function of (shown document, landed evaluation,
    /// selection)** — recomputed, never cached, so it cannot be stale
    /// with respect to the state it describes. A face's verdict comes
    /// from the shipped `resolve` door; nothing here re-implements the
    /// resolution ladder, and the one reading made of its answer is the
    /// feature tree's, of which node an indeterminate verdict waits on.
    pub fn standing(&self) -> Standing {
        match &self.derived.selection {
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
    /// kinds, with the node it waits on named as the feature tree
    /// names it ([`crate::tree::resolution_as_drawn`]).
    fn entity_resolution(&self, name: &StableName) -> Option<Box<Resolution>> {
        self.landed_pair().map(|(doc, eval)| {
            Box::new(crate::tree::resolution_as_drawn(
                resolve(RunCtx { doc, eval }, name),
                eval,
            ))
        })
    }

    /// The most recent evaluation that answered the current document.
    pub fn evaluation(&self) -> Option<&Evaluation<f64>> {
        Some(self.derived.landed.as_ref()?.evaluation.as_ref())
    }

    /// The most recent evaluation, shared.
    pub fn evaluation_arc(&self) -> Option<&Arc<Evaluation<f64>>> {
        Some(&self.derived.landed.as_ref()?.evaluation)
    }

    /// The landed evaluation together with the document it answers —
    /// the pair every name question is asked of.
    ///
    /// The two are one value ([`LandedRun`]), so a caller cannot pick
    /// up one without the other.
    pub fn landed_pair(&self) -> Option<(&Doc<ProfileProgram>, &Evaluation<f64>)> {
        let run = self.derived.landed.as_ref()?;
        Some((run.doc.as_ref(), run.evaluation.as_ref()))
    }

    /// Why the landed evaluation's product does not gather, if it does
    /// not — every class, whichever channel reports it
    /// (`frame::badge_site` decides that).
    ///
    /// `None` both when the product is well formed and when nothing
    /// has landed yet; [`DocSession::landed_pair`] distinguishes those.
    pub fn product_fault(&self) -> Option<&ProductError> {
        self.derived.landed.as_ref()?.gather.as_ref().err()
    }

    /// The A5 at-rest verdict for the landed pair ([`AtRestBadge`]),
    /// when [`AtRestBadge`] says one is taken. `None` otherwise, and
    /// before anything lands.
    pub fn at_rest(&self) -> Option<&AtRestBadge> {
        self.derived
            .landed
            .as_ref()?
            .gather
            .as_ref()
            .ok()?
            .at_rest
            .as_ref()
    }

    /// The last locally-valid-range probe, with the field it was taken
    /// for and the unit it was searched in. `None` before any probe,
    /// and after every document change (`request_eval`'s discard).
    pub fn bounds(&self) -> Option<&BoundsReading> {
        self.derived.bounds.as_ref()
    }

    /// **The gathered product of the landed run** — the aggregate a
    /// display fit sizes itself on, and the aggregate a scene is
    /// tessellated from.
    ///
    /// **Handed over rather than re-derived**, and **free to ask**:
    /// this is a borrow of what the landing already gathered, never a
    /// gather of its own. On this lane's 165-root, 990-face
    /// measurement a gather is 87 ms; handing this body on is an
    /// `Arc` clone.
    ///
    /// **A plain `&self` getter, deliberately.** A door that could
    /// gather behind a getter would put 87 ms behind a read, and —
    /// the cost that decided it — a `&mut self` accessor cannot be
    /// called from a worker or from an `&DocSession`-shaped request
    /// builder, which is exactly the move the off-thread pick-index
    /// work has filed against this probe. A read that a background
    /// thread cannot make is a read that forecloses its own future.
    ///
    /// `None` has three causes and they are not the same question:
    /// nothing has landed; the gather REFUSED, so no product exists
    /// ([`DocSession::product_fault`] says so); or the gather
    /// succeeded and the A5 gate consumed the body in refusing
    /// ([`Gathered::body`] carries that case). A caller that needs a
    /// body in the third case gathers one for itself and pays for it
    /// where the payment is visible — [`crate::scene::product_of_evaluation`]
    /// is that door.
    pub fn landed_body(&self) -> Option<&Body<f64>> {
        Some(
            self.derived
                .landed
                .as_ref()?
                .gather
                .as_ref()
                .ok()?
                .body
                .as_ref()?,
        )
    }

    /// The advisory-check report for the landed pair — findings in
    /// deterministic order, and the residents that were configured
    /// `Off`. `None` before anything lands, or when the registry
    /// refused (which is distinct from an empty report: "not checked"
    /// and "checked and fine" are different answers).
    pub fn checks(&self) -> Option<&ChecksReport> {
        self.derived.landed.as_ref()?.checks.as_ref()
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

    /// **The options this session's document evaluates under**: its
    /// resolver (the directory rule, above), the defaults otherwise —
    /// the same options the evaluation seam submits, so a caller that
    /// solves the document outside a run (the mate tool's proposal,
    /// a suite reading the solve back) resolves each mated part the
    /// way the landed evaluation resolved it.
    pub fn eval_options(&self) -> EvalOptions {
        crate::evalseam::options(&self.run_resolver())
    }

    /// **The resolver every evaluation of this session's documents runs
    /// under** — the seam's runs and the ones taken beside it alike.
    fn run_resolver(&self) -> Option<Arc<dyn PartResolver>> {
        Some(self.resolver_seam())
    }

    /// **The landed run as a memo for a run under `resolver`**, or
    /// `None` when it resolved through another seam — the seam's own
    /// priming rule ([`crate::evalseam::same_resolver`]), for the runs
    /// taken beside it.
    fn memo_under(&self, resolver: &Option<Arc<dyn PartResolver>>) -> Option<Arc<Evaluation<f64>>> {
        self.derived
            .landed
            .as_ref()
            .filter(|run| crate::evalseam::same_resolver(&run.resolver, resolver))
            .map(|run| Arc::clone(&run.evaluation))
    }

    /// **The session's resolver as the document seam** — the directory
    /// rule's resolver, widened to the trait every door that resolves
    /// a part takes; [`NoFile`] when the session has no directory, so a
    /// part's refusal states the viewer's way through.
    fn resolver_seam(&self) -> Arc<dyn PartResolver> {
        match &self.resolver {
            Some(dir) => Arc::clone(dir) as Arc<dyn PartResolver>,
            None => NoFile::seam(),
        }
    }

    /// The generation the session is waiting for a result on.
    pub fn generation(&self) -> Generation {
        self.generation
    }

    /// The generation of the result currently on screen — what a
    /// consumer derived from the evaluation (the scene) compares
    /// against to know whether its own copy is current.
    pub fn landed_generation(&self) -> Option<Generation> {
        Some(self.derived.landed.as_ref()?.generation)
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
        self.landed_generation() != Some(self.generation)
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

    /// The two reads above as [`Outstanding`] — the one value a
    /// consumer of "is there work outstanding" is given, read here by
    /// NAME rather than paired into an argument list.
    ///
    /// `!busy() && running()` reaches the first arm and reads as
    /// [`Outstanding::Current`], because the picture is what the
    /// chrome describes. **That combination is unreachable through
    /// both shipped seams, by two mechanisms and not by the shape of
    /// this function**: [`DocSession::request_eval`] bumps the
    /// generation on EVERY submit, so `!busy()` means the newest
    /// generation submitted is the one that landed; and both seams
    /// keep at most one request outstanding
    /// (`crates/viewer/src/evalseam.rs`, the module header), so a
    /// landed newest generation leaves the seam nothing to be doing.
    /// The second is a property of the two implementations rather than
    /// of [`EvalService`], which is why the arm is executed by a row
    /// holding a seam that reports work anyway
    /// (`tests/eval_seam.rs`) instead of being left to the comment.
    pub fn outstanding(&self) -> Outstanding {
        if !self.busy() {
            Outstanding::Current
        } else if self.running() {
            Outstanding::Evaluating
        } else {
            Outstanding::Canceled
        }
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
        match &self.derived.landed {
            Some(run) => {
                let mut rows = tree::rows(&run.doc, Some(&run.evaluation), &run.files);
                // An offer acts on the COMMITTED document, so one read
                // off a run of an older document is withheld until the
                // current one lands: the accept it made may already be
                // the edit that run is answering.
                if self.busy() {
                    for row in &mut rows {
                        row.version_offer = None;
                    }
                }
                rows
            }
            // Nothing has landed: the shown document with no
            // evaluation, which renders every row `Unevaluated`, and
            // no scan of the directory, which names a part as unread.
            None => tree::rows(self.doc(), None, &PartFiles::Unscanned),
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
        match self.derived.selection.node() {
            Some(id) => props::slot_rows(self.doc(), id),
            None => Vec::new(),
        }
    }

    /// The property rows as the panel LAYS THEM OUT — [`Self::slot_rows`]
    /// folded so that the three components of a 3-vector arrive as one
    /// group ([`crate::props::group_rows`]).
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
        // **ONE gather, feeding all four of the landing's consumers.**
        // Computed HERE because here is the one place a result becomes
        // the session's, so it cannot be run twice or skipped — and
        // ONCE because the ORDER below is what makes one enough: the
        // product's own verdict reads the refusal, the registry
        // BORROWS the product, and the A5 badge CONSUMES it last.
        //
        // The fourth consumer is the SESSION, which keeps the body for
        // the display fit ([`Gathered::body`]) — and it is why the
        // sentence that used to end this paragraph ("nothing after the
        // badge wants a product") no longer holds. Nothing is cloned
        // for it either: it takes what the gate did not eat.
        let doc: &Doc<ProfileProgram> = &self.requested_doc;
        let cfg = ChecksConfig::default();
        // Whether the document is assembly-shaped is a fact about its
        // nodes: the `Ok` arm reads it for the gate, and the part-file
        // scan below reads it too.
        let assembly_shaped = assembly_shaped(doc);
        let (gather, checks) = match product_recorded(doc, &done.evaluation, self.tol) {
            Ok(product) => {
                // The advisory registry. It REPORTS — a document with
                // findings still draws, which is the whole point of
                // running it on the draw path: a product whose roots
                // interpenetrate renders a picture that looks almost
                // right, and the finding is the only thing that says
                // otherwise. A refusal of the registry itself leaves
                // no report rather than a clean one: "not checked" is
                // not "checked and fine".
                let checks = run_checks_on(
                    doc,
                    &done.evaluation,
                    Subject::Product(&product),
                    &cfg,
                    self.tol,
                )
                .ok();
                // **The gate is still last, and still the only
                // consumer** (DOCM-5's order). What changed is that
                // the body it does not consume is kept rather than
                // dropped: a certification hands the aggregate back on
                // `Assembly`, and a document with no gate to run never
                // gave it away.
                let (at_rest, body) = if assembly_shaped {
                    let (verdict, kept) = badge(assemble_gathered(product, self.tol));
                    (Some(verdict), kept)
                } else {
                    (None, Some(Arc::new(product.body.into_body())))
                };
                (Ok(Gathered { at_rest, body }), checks)
            }
            Err(fault) => {
                // **The product's own verdict.** The gather is the only
                // thing that answers "is this document's product well
                // formed", so every class of refusal is kept here; which
                // channel reports which is `frame::badge_site`'s. The
                // gate never ran, so no A5 badge is taken.
                //
                // A refusal that `ProductErrorKind::means_no_body`
                // reads as an absence is the one the registry still
                // runs over, on the subject that says so. Every other
                // refusal leaves the report absent, which is "not
                // checked".
                let checks = fault
                    .kind()
                    .means_no_body()
                    .then(|| {
                        run_checks_on(doc, &done.evaluation, Subject::NoBodyRoots, &cfg, self.tol)
                            .ok()
                    })
                    .flatten();
                (Err(fault), checks)
            }
        };
        // Only a document that instantiates a part has a part to name,
        // so only it pays the scan.
        let files = if assembly_shaped {
            PartFiles::scanned(self.resolver.as_deref())
        } else {
            PartFiles::Unscanned
        };
        // The landed pair and its verdicts become the session's as ONE
        // value, which is the same value `Derived::none` clears.
        self.derived.landed = Some(LandedRun {
            evaluation: done.evaluation,
            resolver: self.requested_resolver.clone(),
            doc: Arc::clone(&self.requested_doc),
            generation: done.generation,
            gather,
            checks,
            files,
        });
        Landing::Landed
    }

    /// Perform one operation.
    ///
    /// **The mid-drag policy is applied here and nowhere else**, as
    /// two tables — one per gesture, because the session has two
    /// independent drags that refuse different sets
    /// ([`SessionOp::permitted_during_free_move`] carries the argument
    /// for why they cannot be one). No arm below carries a guard
    /// against either drag of its own, so the set of operations a drag
    /// refuses is its table and only its table.
    ///
    /// The order is the value gesture first. Both drags can be open at
    /// once, and when both refuse the same operation the value
    /// gesture's refusal is the one shown — *"finish the drag first"*
    /// names the drag whose preview the operation would have landed
    /// against, which is the nearer of the two answers.
    ///
    /// **Rule 1 is the one answer neither table spells**, for either
    /// drag. A begin that arrives under an open gesture is refused by
    /// the gesture's own door — [`g1::Slot::begin`], reached through
    /// [`DocSession::start`] for the value drag and through
    /// [`DisplayState::begin_free_move`] for the probe — with the same
    /// refusal a row here would raise, off the same state. So each
    /// begin is `true` in the table that would otherwise pre-empt it —
    /// [`SessionOp::BeginGesture`] and [`SessionOp::BeginParamGesture`]
    /// in the value table, [`SessionOp::BeginFreeMove`] in the
    /// free-move one — and the set of operations a drag refuses is its
    /// table plus that one rule, held once for both drags rather than
    /// spelled per gesture and per table.
    pub fn perform(&mut self, op: SessionOp) -> OpOutcome {
        if self.gesture.held().is_some() && !op.permitted_during_value_gesture() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        if self.display.probing().is_some() && !op.permitted_during_free_move() {
            return OpOutcome::refused(Refusal::Display(DisplayFault::FreeMoveInFlight));
        }
        match op {
            SessionOp::Select(selection) => {
                self.derived.selection = selection;
                OpOutcome::default()
            }
            SessionOp::Hover(hover) => {
                self.derived.hover = hover;
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
            SessionOp::SetParamUnit { name, unit } => self.set_param_unit(name, unit),
            SessionOp::SetParamText { name, text } => self.set_param_text(name, &text),
            SessionOp::CreateParam { name, value } => self.create_param(name, value),
            SessionOp::BeginGesture { node, slot } => self.begin_gesture(node, slot),
            SessionOp::BeginParamGesture { name } => self.begin_param_gesture(&name),
            SessionOp::PreviewGesture { node, slot, value } => {
                self.preview_gesture(&ValueGestureName::Slot { node, slot }, value)
            }
            SessionOp::CommitGesture { node, slot } => {
                self.commit_gesture(&ValueGestureName::Slot { node, slot })
            }
            SessionOp::PreviewParamGesture { name, value } => {
                self.preview_gesture(&ValueGestureName::Param(name), value)
            }
            SessionOp::CommitParamGesture { name } => {
                self.commit_gesture(&ValueGestureName::Param(name))
            }
            SessionOp::CancelGesture => match self.gesture.cancel(gesture_words()) {
                // Only a drag that actually put a scratch document on
                // screen owes a re-submit to take it away again, and
                // whether it did is what the cancel answers — the
                // scratch and the gesture's value are written by one
                // preview.
                Ok(previewed) => {
                    let scratch = self.derived.scratch.take();
                    debug_assert_eq!(
                        previewed,
                        scratch.is_some(),
                        "a cancelled drag's scratch document and its previewed value \
                         are written by one preview and must end together"
                    );
                    if previewed {
                        self.request_eval();
                    }
                    OpOutcome::default()
                }
                Err(refusal) => OpOutcome::refused(refusal),
            },
            SessionOp::Undo => self.step(Step::Undo),
            SessionOp::Redo => self.step(Step::Redo),
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
            SessionOp::PreviewFreeMove { instance, frame } => {
                match self.display.preview_free_move(instance, frame) {
                    Ok(()) => OpOutcome::default(),
                    Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
                }
            }
            SessionOp::CommitFreeMove { instance } => {
                match self.display.commit_free_move(instance) {
                    Ok(()) => OpOutcome::default(),
                    Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
                }
            }
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
            SessionOp::EditProfile {
                node,
                base,
                loops,
                ids,
            } => self.edit_profile(node, &base, loops, ids),
            SessionOp::AddExtrude { profile, distance } => self.add_extrude(profile, distance),
            SessionOp::AddRevolve {
                profile,
                axis,
                angle,
            } => self.add_revolve(profile, axis, angle),
            SessionOp::AddBoolean { op, a, b, declare } => self.add_boolean(op, a, b, declare),
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
            SessionOp::AddPart { of, select } => self.add_part(of, select),
            SessionOp::Duplicate { input } => self.add_duplicate(input),
            SessionOp::AddInstance { id } => self.add_instance(id),
            SessionOp::AcceptPartVersion { id } => self.accept_part_version(id),
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
        self.read_store(|ws| Ok(parts::catalogue(ws, self.committed_doc().id())))
    }

    /// **One scan of the document's directory, as a value**
    /// ([`parts::PartCensus`]) — the directory that was read and what
    /// reading it answered, taken together because they are about one
    /// moment.
    ///
    /// The chooser is a vocabulary and may not name this driver, so
    /// the read is hoisted to here rather than the rule widened
    /// (`crates/viewer/README.md`, *What a vocabulary reads, it is
    /// handed*); the derivation moving into the driver is what that
    /// costs.
    #[must_use]
    pub fn part_census(&self) -> parts::PartCensus {
        parts::PartCensus::taken(
            self.resolve_dir().map(Path::to_path_buf),
            self.part_catalogue(),
        )
    }

    /// **What a pick index is built from** ([`pickcache::IndexInputs`]):
    /// the landed pair, the generation it answered and the ε to
    /// tessellate at — or `None` when nothing has landed, which is the
    /// cache's own "forget everything" case.
    ///
    /// Three of the four are read together for the same reason the
    /// pair is one value: one landing writes them, so a caller cannot
    /// pick up a generation without the run it answers. The fourth,
    /// `tol`, is this session's ε — construction-time, never rewritten
    /// by a landing — and rides along because the build needs it.
    ///
    /// `pickcache` is a vocabulary and may not name this driver, so
    /// the read is hoisted rather than the rule widened
    /// (`crates/viewer/README.md`, *What a vocabulary reads, it is
    /// handed*, carries the argument).
    #[must_use]
    pub fn index_inputs(&self) -> Option<pickcache::IndexInputs<'_>> {
        let run = self.derived.landed.as_ref()?;
        Some(pickcache::IndexInputs::of(
            run.generation,
            run.doc.as_ref(),
            &run.evaluation,
            self.tol,
        ))
    }

    /// **What the display budget is handed to price `requested` on**,
    /// minted here for [`DocSession::index_inputs`]'s reason: the
    /// generation and the thing it describes leave this type paired,
    /// so nothing above can price one landing's body under another
    /// landing's number.
    ///
    /// The arm is the LANDING's shape, and it is read here because
    /// here is where that shape is known.
    /// [`crate::evalseam::FitSubject::Landed`] hands on the body the
    /// landing already gathered — an `Arc` clone, per
    /// [`DocSession::landed_body`]. `Ungathered` is the third `None`
    /// cause that accessor names: the gather succeeded and the A5 gate
    /// consumed the body in refusing, so there is a product to be had
    /// and nobody holding it. That arm names the pair rather than
    /// gathering from it, and the gather is paid on the fit worker —
    /// which is why this is a getter again and not a spelled-out
    /// choice at the call site: neither arm costs this thread
    /// anything.
    ///
    /// `None` before anything lands, and for a landing whose gather
    /// REFUSED ([`DocSession::product_fault`]) — a document with no
    /// product has no size to fit a δ to, and the index build below is
    /// about to refuse it with its own typed answer.
    pub fn fit_request(
        &self,
        requested: crate::scene::DisplayTolerance,
    ) -> Option<crate::evalseam::FitRequest> {
        let run = self.derived.landed.as_ref()?;
        let subject = match &run.gather {
            Err(_) => return None,
            Ok(Gathered {
                body: Some(body), ..
            }) => crate::evalseam::FitSubject::Landed(Arc::clone(body)),
            Ok(Gathered { body: None, .. }) => crate::evalseam::FitSubject::Ungathered {
                doc: Arc::clone(&run.doc),
                evaluation: Arc::clone(&run.evaluation),
            },
        };
        Some(crate::evalseam::FitRequest {
            generation: run.generation,
            requested,
            subject,
            tol: self.tol,
        })
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
        let pin = match self.read_store(|ws| ws.current_pin(id, self.tol)) {
            Ok(pin) => pin,
            Err(refusal) => return OpOutcome::refused(refusal),
        };
        self.commit(DocEdit::InsertNode {
            node: Node::instantiate_part(DocRef { id, pin }),
        })
    }

    /// Move every reference to the part `id` onto the version its file
    /// holds now, as one action: the store's own elaboration
    /// ([`pncad::workspace::update_to_store`]), applied whole.
    fn accept_part_version(&mut self, id: DocumentId) -> OpOutcome {
        let doc = self.committed_doc();
        match self.read_store(|ws| pncad::workspace::update_to_store(doc, id, ws, self.tol)) {
            Ok(edits) => self.commit_action(edits),
            Err(refusal) => OpOutcome::refused(refusal),
        }
    }

    /// **A read of the session's store, refused typed**: `read` over
    /// the directory every reference resolves against
    /// ([`DirResolver::workspace`]) — [`Refusal::NoDocumentDirectory`]
    /// for a session with no backing file, and the store's own
    /// refusal, scan or read, as [`Refusal::Workspace`].
    fn read_store<T>(
        &self,
        read: impl FnOnce(&pncad::workspace::Workspace) -> Result<T, pncad::workspace::WorkspaceError>,
    ) -> Result<T, Refusal> {
        let resolver = self
            .resolver
            .as_deref()
            .ok_or(Refusal::NoDocumentDirectory)?;
        resolver
            .workspace()
            .and_then(|ws| read(&ws))
            .map_err(|error| Refusal::Workspace(Box::new(error)))
    }

    fn set_slot(&mut self, node: RecipeNodeId, slot: SlotId, value: SlotValue) -> OpOutcome {
        if let Err(refusal) = guard_driven(self.committed_doc(), node, slot, self.notation) {
            return OpOutcome::refused(refusal);
        }
        let unit = props::slot_unit(self.committed_doc(), node, slot);
        match props::slot_edit(node, slot, value, unit) {
            Ok(edit) => self.commit_written(edit),
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
            && let Err(refusal) = guard_driven(self.committed_doc(), node, slot, self.notation)
        {
            return OpOutcome::refused(refusal);
        }
        // Read off `base` — the document the samples below are applied
        // to — and not off the session again. One document answers
        // where the search starts, what it steps by, and what every
        // candidate is judged against, so a probe cannot seed from one
        // document and search another.
        let base = self.doc().clone();
        let resolver = self.run_resolver();
        let prior = self.memo_under(&resolver);
        match probe::probe_bounds(&base, target, prior.as_deref(), &resolver, self.tol) {
            Ok(reading) => {
                self.derived.bounds = Some(reading);
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

    /// **What `SessionOp::SetSlotUnit` would answer for this slot
    /// whatever unit is picked, or `None` where the slot has a written
    /// notation to change** — asked ahead of the pick by the control
    /// that pushes the op.
    ///
    /// It is the unit-free half of the op's own slot admission
    /// ([`props::slot_literal`], which `props::slot_unit_edit` runs
    /// first), so the words it carries are the refusal's own. It is not
    /// everything `perform` can answer: the session-wide gate `perform`
    /// applies before any op (a held value gesture refuses
    /// `SetSlotUnit` with [`Refusal::GestureInFlight`]) is not read
    /// here, and neither is the unit-dependent arm, which the op
    /// answers at the pick.
    pub fn slot_unit_refusal(&self, node: RecipeNodeId, slot: SlotId) -> Option<Refusal> {
        props::slot_literal(self.committed_doc(), node, slot)
            .err()
            .map(Refusal::SlotUnit)
    }

    /// **The declared dimensions `parse_expr` reads text against** —
    /// every text door's first argument, and one function because
    /// both of them wanted it.
    ///
    /// The parser needs them so a parameter reference records the
    /// dimension `apply` will re-check it against; a door that built
    /// the map itself would be free to build a different one.
    fn param_dims(&self) -> std::collections::BTreeMap<ParamName, Dimension> {
        self.committed_doc()
            .params()
            .iter()
            .map(|(name, param)| (name.clone(), param.dim()))
            .collect()
    }

    fn set_slot_expression(&mut self, node: RecipeNodeId, slot: SlotId, text: &str) -> OpOutcome {
        let expr = match parse_expr(text, &self.param_dims()) {
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
        // **Through the written door**, like every other door that
        // writes a panel field's value. The field's own guard cannot
        // answer for this one: a literal slot that evaluated shows its
        // NUMBER, so the render the echo compares against is the bare
        // `8.0` while the slot's source is `8 mm`, and re-typing the
        // source is no echo of anything. `Self::writes_nothing` asks the question
        // that is actually being asked here — whether the expression
        // offered is the expression standing — and the `unparse` /
        // `parse_expr` round trip preserves both the bits and the
        // display unit, so a source re-typed as itself compares equal.
        self.commit_written(edit)
    }

    /// The value door: write a declared parameter's value.
    ///
    /// A name the document does not declare takes the commit path so
    /// the typed refusal comes from the door rather than from here —
    /// `DocEdit::SetDocParamValue` carries an existing declaration
    /// forward and refuses `EditError::DocParamNotDeclared` when there
    /// is none.
    fn set_param(&mut self, name: &ParamName, value: SlotValue) -> OpOutcome {
        self.commit_written(props::param_edit(name.clone(), value))
    }

    /// The notation door: rewrite a declared parameter's display unit,
    /// its value untouched.
    ///
    /// No pre-check, for [`Self::set_param`]'s reason: every way this
    /// can refuse — an undeclared name, a `Count`, a unit that does
    /// not measure the declared dimension — is refused by
    /// `DocEdit::SetDocParamUnit` in the door's own words, and a
    /// second opinion here could only agree or disagree.
    fn set_param_unit(&mut self, name: ParamName, unit: UnitDef) -> OpOutcome {
        self.commit_written(props::param_unit_edit(name, unit))
    }

    /// The text door: a number, and the notation to write it in, from
    /// one piece of typed text.
    ///
    /// **One parser.** `parse_expr` reads `50 mm` into a literal that
    /// already carries the canonical value and remembers the unit —
    /// the one multiply is the parser's — so what arrives here is read
    /// off the literal and never scaled again.
    ///
    /// **One action, therefore one undo.** The value and the notation
    /// go through [`Self::commit_action`], which is all-or-nothing: a
    /// unit that does not measure the parameter's dimension refuses
    /// the whole action rather than landing a value in a notation the
    /// document would then refuse to save. The notation edit is first
    /// for that reason — the pairing is judged before any value moves.
    ///
    /// **Only the edits that change something are submitted**, and
    /// that is a DIFFERENT rule from the field's own guard. The
    /// field's ([`props::echoed`]) is about the text: a render the
    /// field handed back at itself is not something a person typed.
    /// This one is about the document: an edit that writes what the
    /// declaration already holds is submitted by nobody, so `50 mm`
    /// typed over a parameter already declared `50 mm` costs no undo
    /// step even though the text is not the field's render of it. It
    /// is asked over the PAIR, because this door carries a pair —
    /// `0.05 m` over `50 mm` moves the notation and not the value.
    ///
    /// The number door has no such check and needs none: it changes
    /// one fact, and a number that reads back as the one standing
    /// cannot have got past the field's guard as anything but a
    /// deliberate re-type.
    fn set_param_text(&mut self, name: ParamName, text: &str) -> OpOutcome {
        let expr = match parse_expr(text, &self.param_dims()) {
            Ok(expr) => expr,
            Err(error) => return OpOutcome::refused(Refusal::Parse(Box::new(error))),
        };
        // A literal is the whole of what a parameter can hold. Every
        // other kind of expression — a reference, an operator, a count
        // — is refused by name rather than flattened to a number,
        // because flattening would store a value the text does not
        // say.
        let (Some(value), Some(unit)) = (expr.literal_value(), expr.display_unit()) else {
            return OpOutcome::refused(Refusal::ParamNotANumber { name });
        };
        let declared = self.committed_doc().params().get(&name).map(DocParam::dim);
        let notation = props::param_unit_edit(name.clone(), unit);
        let written = props::param_edit(name.clone(), SlotValue::Continuous(value));
        match declared {
            // An undeclared name takes the commit path so the typed
            // refusal comes from the door rather than from here, the
            // way [`Self::set_param`]'s does.
            None => return self.commit(written),
            // **A `Count` gets the VALUE edit and only that.** A count
            // names no notation at all, so the notation half of this
            // door has nothing to say about one, and submitting it
            // anyway answers what the user did — typing a value — in
            // the words of a change nobody asked for ("it has no
            // display unit to change"). The value edit is the act, and
            // refusing it names the right half: a count declared where
            // a continuous value was typed.
            Some(Dimension::Count) => return self.commit(written),
            Some(Dimension::Length | Dimension::Angle | Dimension::Scalar) => {}
        }
        let edits: Vec<DocEdit<ProfileProgram>> = [notation, written]
            .into_iter()
            .filter(|edit| !self.writes_nothing(edit))
            .collect();
        if edits.is_empty() {
            return OpOutcome::default();
        }
        self.commit_action(edits)
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

    /// The slot door: a drag over a literal slot's number.
    ///
    /// The driven-slot guard is the TARGET check and runs inside
    /// [`Self::start`]'s closure, so it answers only for a gesture
    /// that is actually being opened.
    fn begin_gesture(&mut self, node: RecipeNodeId, slot: SlotId) -> OpOutcome {
        let notation = self.notation;
        self.start(move |doc| {
            guard_driven(doc, node, slot, notation)?;
            Ok(GestureTarget::Slot {
                node,
                slot,
                unit: props::slot_unit(doc, node, slot),
            })
        })
    }

    /// The parameter door: a drag over a declared parameter's value.
    fn begin_param_gesture(&mut self, name: &ParamName) -> OpOutcome {
        let name = name.clone();
        self.start(move |doc| {
            let dimension = doc
                .params()
                .get(&name)
                .map(|param| param.dim())
                .ok_or_else(|| Refusal::NoSuchParam(name.clone()))?;
            Ok(GestureTarget::Param { name, dimension })
        })
    }

    /// Open a gesture, refusing one that is already open and
    /// validating `target` only once the slot is known free.
    ///
    /// **Rule 1 is [`g1::Slot::begin`]'s and is spelled nowhere else**
    /// — the same door the probe's begin goes through, with this
    /// gesture's words. `BeginGesture` and `BeginParamGesture` are
    /// therefore `true` in
    /// [`SessionOp::permitted_during_value_gesture`]: a row there
    /// would refuse the same state with the same
    /// [`Refusal::GestureInFlight`] one layer up, and the door's own
    /// arm would never run.
    ///
    /// `target` is the caller's check — a driven slot, a parameter the
    /// document does not declare — and runs inside the slot's closure
    /// rather than ahead of it, so a begin arriving under an open drag
    /// is answered *finish the drag first* rather than told about a
    /// field it was never going to open.
    fn start(
        &mut self,
        target: impl FnOnce(&Doc<ProfileProgram>) -> Result<GestureTarget, Refusal>,
    ) -> OpOutcome {
        // The committed document is borrowed beside the slot rather
        // than through `self`: the target's check reads it while the
        // gesture field is being written.
        let history = &self.history;
        match self.gesture.begin(gesture_words(), || {
            Ok(Gesture {
                target: target(history.doc())?,
                base: history.doc().clone(),
            })
        }) {
            Ok(()) => OpOutcome::default(),
            Err(refusal) => OpOutcome::refused(refusal),
        }
    }

    /// Move the gesture `named` names.
    ///
    /// The replacement and the two refusals are [`g1::Slot::preview`]'s
    /// — held there for both gestures — and what is this door's own is
    /// the edit, the scratch document and the eval request.
    ///
    /// **The name is checked before the value is used**, so a drag on
    /// a field that could not open its own gesture previews nothing
    /// rather than previewing its number into the open gesture's slot.
    /// The refusal is [`Refusal::WrongGesture`] and not
    /// [`Refusal::GestureInFlight`]: the operation IS permitted while
    /// a drag is open (`permitted_during_value_gesture`, which every
    /// driving operation has to be), and what it is not is about this
    /// drag.
    fn preview_gesture(&mut self, named: &ValueGestureName, value: f64) -> OpOutcome {
        let resolver = self.resolver_seam();
        let tol = self.tol;
        let previewed = self.gesture.preview(
            gesture_words(),
            |gesture| gesture.target.name() == *named,
            |gesture| {
                let slot_value = gesture.target.value_of(value).map_err(Refusal::Dimension)?;
                let edit = gesture
                    .target
                    .edit(slot_value)
                    .map_err(Refusal::Dimension)?;
                // Applied to the gesture's BASE, so previews replace
                // one another instead of composing, and the history
                // never sees any of them. The reach is the session's
                // own seam: a gesture that moved a root would mint a
                // frame from the parts' extent, and with no directory
                // to resolve against it refuses typed. Built per tick,
                // and lazy — a slot gesture moves no root, so what a
                // tick pays for it is the construction and nothing
                // more.
                let reach = PartReach::<f64>::with_resolver(Some(&resolver), tol);
                let applied = apply(&gesture.base, &edit, tol, &reach)
                    .map_err(|error| Refusal::Edit(Box::new(error)))?;
                // **The display layer's identity, held rather than
                // argued.** Every display predicate is a function of
                // the node graph, and the free-move probe is admitted
                // against the COMMITTED document while the view and
                // the panel — which run ONE admission test between
                // them, `display::instance_check` — resolve against
                // this scratch, so the two agree only while a gesture's
                // edits leave the graph alone. This holds the half a
                // check can hold; the other half is that
                // [`GestureTarget::edit`] can produce nothing but
                // `SetParam`, `SetStructuralParam` and
                // `SetDocParamValue`, none of which removes a node
                // either.
                assert!(
                    applied.record.minted.is_none(),
                    "a value gesture's preview minted a node, which the display \
                     layer's admission tests are not re-run against"
                );
                Ok((slot_value, (edit, applied.doc)))
            },
        );
        match previewed {
            Ok((edit, doc)) => {
                self.derived.scratch = Some(doc);
                self.request_eval();
                OpOutcome {
                    previewed: vec![edit],
                    ..OpOutcome::default()
                }
            }
            Err(refusal) => OpOutcome::refused(refusal),
        }
    }

    /// Land the gesture `named` names.
    ///
    /// The no-move rule and the name check are [`g1::Slot::commit`]'s,
    /// held there for both gestures: **the name is checked before the
    /// gesture is taken**, so a refused commit leaves the drag it does
    /// not name open — the release event of one field is not a release
    /// of another, and a gesture that ends here would end with nobody
    /// having let go of it. What is this door's own is what a landed
    /// value becomes: one `DocEdit` on the history, and one undo step.
    fn commit_gesture(&mut self, named: &ValueGestureName) -> OpOutcome {
        let landed = match self
            .gesture
            .commit(gesture_words(), |gesture| gesture.target.name() == *named)
        {
            Ok(landed) => landed,
            Err(refusal) => return OpOutcome::refused(refusal),
        };
        let previewed = self.derived.scratch.take().is_some();
        debug_assert_eq!(
            previewed,
            landed.is_some(),
            "the scratch document and the gesture's value are written by one preview \
             and must end together"
        );
        // A gesture that never moved commits nothing: one undo step
        // per gesture that CHANGED something, none for a click that
        // happened to land on a slider. It also asks for NOTHING —
        // dropping a scratch that was never set leaves the shown
        // document exactly as it was, and a request for a picture we
        // already have spends a generation and flickers the indicator
        // to say so.
        let Some((gesture, value)) = landed else {
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

    /// Undo or redo, by the direction the op names.
    ///
    /// **The refusal is the MOVE's own answer**, not a pre-check: the
    /// door attempts the step and refuses on the `None` the attempt
    /// returns, so a history whose `can_step` half ever disagreed with
    /// its move half would fail here rather than report a clean
    /// outcome for a step that did not happen. The chrome's
    /// [`Refusal::nothing_to_step`] composes the same refusal VALUE
    /// ahead of the click, because a button has to decide whether to
    /// offer the move; one composition of the words, two readings of
    /// the history, and this one is the one that acts.
    fn step(&mut self, direction: Step) -> OpOutcome {
        let moved = match direction {
            Step::Undo => self.history.undo(),
            Step::Redo => self.history.redo(),
        };
        if moved.is_none() {
            return OpOutcome::refused(Refusal::NothingToDo { direction });
        }
        // The document moved, so the display state's derived facts
        // (which instances exist; which are mate-constrained) may have
        // too — an undo past a mate's insertion does NOT resurrect a
        // discarded probe (the value is gone, not parked), but a redo
        // over one discards the probe it constrains.
        let pruned = self.display.prune(self.history.doc());
        self.request_eval();
        OpOutcome::from_prune(pruned)
    }

    /// Refused mid-gesture by BOTH tables
    /// ([`SessionOp::permitted_during_value_gesture`] and
    /// [`SessionOp::permitted_during_free_move`]), shared with
    /// [`SessionOp::NewDocument`]: both doors replace the document a
    /// drag is previewing against and drop the display state whole, and
    /// a gesture silently dissolved under the pointer is the kind of
    /// half-acted state that refusal exists to prevent.
    fn open(&mut self, path: &Path) -> OpOutcome {
        match docio::open(path, self.tol) {
            Ok(history) => {
                // The directory rule: the resolver is the opened
                // file's own directory, consulted lazily at each
                // resolution (DirResolver's docs carry the posture).
                self.resolver = Some(Arc::new(DirResolver::new(session_dir(path))));
                self.history = history;
                self.path = Some(path.to_path_buf());
                self.clear_for_new_document();
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
    /// The same two doors as [`DocSession::open`] — a new history and
    /// [`DocSession::clear_for_new_document`] — with the file-shaped
    /// fields going the other way: no path and no resolver, because
    /// nothing backs this document until it is saved.
    fn new_document(&mut self, name: &str) -> OpOutcome {
        let name = match Refusal::new_document_name(name) {
            Ok(name) => name,
            Err(refusal) => return OpOutcome::refused(refusal),
        };
        self.history = History::new(Doc::empty_derived(name, self.tol));
        self.path = None;
        self.resolver = None;
        self.clear_for_new_document();
        self.request_eval();
        OpOutcome::default()
    }

    /// **Drop everything the previous document left behind**, for the
    /// two doors that replace it ([`SessionOp::Open`],
    /// [`SessionOp::NewDocument`]). The history and the file-shaped
    /// fields are the caller's to set; everything else derived from
    /// the old document goes here.
    ///
    /// The landed run answered the PREVIOUS document: left in place it
    /// would render the old model's tree and resolve the new
    /// document's names against it until the first run lands. That is
    /// one instance of the rule, not the rule — hence one value
    /// rebuilt from nothing ([`Derived`]) rather than a field-by-field
    /// walk each door has to remember.
    ///
    /// G3: hide and free-move are display state of a SESSION over a
    /// document, never of the document — a fresh open starts with
    /// none, which is what makes "save, reopen, layer-3 state gone" a
    /// property of the structure. It is `clear`ed rather than rebuilt
    /// because its revision counter must not go backwards
    /// ([`Derived`]'s own docs carry that and the other two
    /// exclusions).
    ///
    /// **Nothing is checked here, and that is the point.** Three
    /// things hold on entry and none is this function's to enforce:
    /// **no value gesture is open** and **no free-move gesture is in
    /// flight**, because `perform` refused both doors while either was
    /// (the two tables' guarantee, a different guarantee from the one
    /// below), and `scratch` is already `None`, because a preview never
    /// outlives the gesture that wrote it (`Derived`'s own invariant,
    /// which is what makes clearing it below a no-op rather than half
    /// a dissolved drag). A check placed here could only fire after a
    /// caller had already written `history` and the file-shaped
    /// fields, which is the half-replaced session the refusal exists to
    /// prevent — so the precondition lives at `perform`, where refusing
    /// costs a `Refusal` and nothing has moved yet.
    ///
    /// The free-move half of that is newer than the rest of this walk
    /// and was the one asymmetry in it: the drag was dissolved here,
    /// silently, by the same `display.clear()` below, while the value
    /// drag one field over was protected by a refusal at the door. The
    /// two are the same state under a pointer and now get the same
    /// answer ([`SessionOp::permitted_during_free_move`]).
    fn clear_for_new_document(&mut self) {
        self.derived = Derived::none();
        self.display.clear();
    }

    /// Insert one datum node ([`SessionOp::AddDatum`]). Its slots are
    /// literals; the two kinds that also name a node by PICK — an axis
    /// in a sketch names its frame, a frame on a face names the body
    /// its face is read out of — are gated by kind here, so a pick of
    /// the wrong kind is refused `WrongNodeKind` before the edit.
    fn add_datum(&mut self, datum: DatumSpec) -> OpOutcome {
        // An axis in a sketch names its frame by PICK, so it is gated
        // at this door by kind, as the add-profile door gates its
        // plane.
        if let DatumSpec::AxisInPlane { plane, .. } = &datum
            && let Err(refusal) = self.require_kind(*plane, NodeKindWanted::Frame)
        {
            return OpOutcome::refused(refusal);
        }
        // A frame on a face names the node its face is read out of by
        // PICK too, and that node has to denote ONE body: the
        // evaluator reads the face through its single-body operand
        // door, so a split side or a pattern instance would mint a
        // node that refuses after the edit lands. Gated here for the
        // same reason the frame above is.
        if let DatumSpec::FaceFrame { at, .. } = &datum
            && let Err(refusal) = self.require_kind(*at, NodeKindWanted::Body)
        {
            return OpOutcome::refused(refusal);
        }
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
    fn add_profile(&mut self, plane: ProfilePlane, loops: Vec<LoopProgram>) -> OpOutcome {
        let plane = match plane {
            // The plane is a PICK, so it is gated where every other
            // pick is: at this door, by kind, before the edit. Without
            // this the reference would reach evaluation and refuse
            // there — a typed refusal either way, but one the person
            // gets after the node lands rather than instead of it.
            ProfilePlane::Existing(plane) => {
                if let Err(refusal) = self.require_kind(plane, NodeKindWanted::Frame) {
                    return OpOutcome::refused(refusal);
                }
                plane
            }
            // **No kind gate on this arm, and that is the point**: the
            // id the profile names is one this action mints a line
            // below, from a `DatumSpec::Frame`, so there is no pick
            // that could be of the wrong kind. Asking `require_kind`
            // here would be a question about a node that does not
            // exist yet.
            ProfilePlane::NewXy => return self.add_profile_on_new_xy(loops),
        };
        self.commit(DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane,
                loops,
                ids: Vec::new(),
            }),
        })
    }

    /// Insert the world XY frame and a profile drawn on it, as ONE
    /// committed action and therefore one undo
    /// ([`ProfilePlane::NewXy`]).
    ///
    /// The frame's id is not predicted: each edit is built from what
    /// its predecessor MINTED ([`Self::commit_run`]), so the profile
    /// names the node the door actually created. All-or-nothing comes
    /// free with that — a profile the insert door refuses leaves no
    /// orphan frame behind, because nothing is recorded until both
    /// edits have landed.
    fn add_profile_on_new_xy(&mut self, loops: Vec<LoopProgram>) -> OpOutcome {
        let frame = match ProfilePlane::world_xy() {
            Ok(frame) => datum_node(frame),
            Err(error) => return OpOutcome::refused(Refusal::Dimension(error)),
        };
        let mut loops = Some(loops);
        self.commit_run(|minted| match minted {
            [] => Some(DocEdit::InsertNode {
                node: frame.clone(),
            }),
            [Some(plane)] => Some(DocEdit::InsertNode {
                node: Node::Profile(ProfileProgram {
                    plane: *plane,
                    // Loud, like the arm below: ending the run here
                    // instead would commit the lone frame, which is
                    // the exact orphan all-or-nothing promises
                    // against. The generator is called once per
                    // position and this position comes round once.
                    loops: loops
                        .take()
                        .unwrap_or_else(|| unreachable!("the profile's position comes round once")),
                    ids: Vec::new(),
                }),
            }),
            [None] => unreachable!("an `InsertNode` mints an id (`EditRecord::minted`)"),
            _ => None,
        })
    }

    /// Write the path editor's program over a committed profile's
    /// ([`SessionOp::EditProfile`], whose doc carries the rules).
    fn edit_profile(
        &mut self,
        node: RecipeNodeId,
        base: &ProfileProgram,
        loops: Vec<LoopProgram>,
        ids: Vec<Vec<Option<StepId>>>,
    ) -> OpOutcome {
        match self.set_program_of(node, base, loops, ids) {
            Ok(Some(edit)) => self.commit(edit),
            Ok(None) => OpOutcome::default(),
            Err(refusal) => OpOutcome::refused(refusal),
        }
    }

    /// **What [`SessionOp::EditProfile`] would report, without
    /// committing it** — the edit door's own `Applied::maintenance` for
    /// the one `SetProgram` the op would commit, netted by
    /// [`MaintenanceNet`] as the commit nets it: every name on a step
    /// the reshaping drops, stranded. Empty when the op would write
    /// nothing.
    ///
    /// The profile editor reads it BEFORE its Apply, while the person
    /// can still keep the step; the op's outcome carries the same rows
    /// after.
    ///
    /// # Errors
    ///
    /// The refusal the op itself would give.
    pub fn edit_profile_report(
        &self,
        node: RecipeNodeId,
        base: &ProfileProgram,
        loops: Vec<LoopProgram>,
        ids: Vec<Vec<Option<StepId>>>,
    ) -> Result<Vec<Maintenance>, Refusal> {
        let Some(edit) = self.set_program_of(node, base, loops, ids)? else {
            return Ok(Vec::new());
        };
        let resolver = self.resolver_seam();
        let reach = PartReach::<f64>::with_resolver(Some(&resolver), self.tol);
        let applied = apply(self.committed_doc(), &edit, self.tol, &reach)
            .map_err(|error| Refusal::Edit(Box::new(error)))?;
        let mut net = MaintenanceNet::new();
        net.push(&applied);
        Ok(net.finish(&applied.doc))
    }

    /// **The one `SetProgram` [`SessionOp::EditProfile`] commits**, or
    /// `None` when it would write nothing.
    fn set_program_of(
        &self,
        node: RecipeNodeId,
        base: &ProfileProgram,
        loops: Vec<LoopProgram>,
        ids: Vec<Vec<Option<StepId>>>,
    ) -> Result<Option<DocEdit<ProfileProgram>>, Refusal> {
        self.require_kind(node, NodeKindWanted::Profile)?;
        let doc = self.committed_doc();
        let Some(Node::Profile(current)) = doc.node(node) else {
            unreachable!("`require_kind` admitted feature {} as a profile", node.0)
        };
        // The editor's program is an edit OF the program it loaded;
        // over any other program it would be a guess about what the
        // person meant. Compared by value, so a unit rewrite since the
        // load does not refuse.
        if current != base {
            return Err(Refusal::ProfileEditStale { node });
        }
        let loops = carry_unmoved(doc, node, current, loops, &ids, self.notation)?;
        let unchanged = sketch::is_committed(current, &loops, &ids);
        Ok((!unchanged).then_some(DocEdit::SetProgram { node, loops, ids }))
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

    /// Insert one regularized boolean of two existing bodies, and the
    /// declaration of the contacts it names
    /// ([`SessionOp::AddBoolean`]).
    fn add_boolean(
        &mut self,
        op: BooleanOp,
        a: RecipeNodeId,
        b: RecipeNodeId,
        declare: Vec<FlushFinding>,
    ) -> OpOutcome {
        for seat in [a, b] {
            if let Err(refusal) = self.require_kind(seat, NodeKindWanted::Body) {
                return OpOutcome::refused(refusal);
            }
        }
        // One node in both seats is NOT pre-checked here: the edit
        // door refuses it typed (`EditError::DuplicateInput`, off
        // `Node::input_fault`'s pairwise-distinct rule), and a flat arm
        // must not restate a refusal a door already gives
        // (`crates/viewer/README.md`). The kind gate above still speaks
        // first, which is what keeps two PROFILES in both seats
        // reported as "that is not a body" — the fact the user can act
        // on — rather than as the narrower complaint about the pair.
        let declaration = if declare.is_empty() {
            None
        } else {
            Some(declare_node(&declare).unwrap_or_else(|error| {
                unreachable!(
                    "`declare_node` refuses only an empty list, and this one is not: {error}"
                )
            }))
        };
        let boolean = |declare| DocEdit::InsertNode {
            node: Node::Boolean { op, a, b, declare },
        };
        let staged = self.stage_run(|minted| match (minted, &declaration) {
            ([], None) => Some(boolean(None)),
            ([], Some(node)) => Some(DocEdit::InsertNode { node: node.clone() }),
            ([Some(declared)], Some(_)) => Some(boolean(Some(*declared))),
            _ => None,
        });
        let staged = match staged {
            Ok(staged) => staged,
            Err(refusal) => return OpOutcome::refused(refusal),
        };
        let Some(Some(node)) = staged.minted.last().copied() else {
            unreachable!("the run ends on the boolean's `InsertNode`")
        };
        // Judged before it is recorded: the one recourse to a contact
        // refusal is a declaration in the same action, which cannot be
        // added to a committed node.
        let resolver = self.run_resolver();
        let memo = self.memo_under(&resolver);
        let judged = evaluate_beside(&staged.doc, memo.as_deref(), &resolver, self.tol);
        if let Some(refused) =
            RefusedBoolean::read(&judged, node, (op, [a, b]), declare, self.generation)
        {
            return OpOutcome::refused(Refusal::Contact(Box::new(refused)));
        }
        self.record_run(staged)
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
        // Total, as the other lowerings are: slot dimensions are the
        // edit door's question.
        self.commit(DocEdit::InsertNode {
            node: Node::transform(
                input,
                pncad::document::Step::Rigid {
                    translation,
                    axis: rotation_axis,
                    angle: rotation_angle,
                },
            ),
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

    /// Insert one projection of a multi-body value
    /// ([`SessionOp::AddPart`]).
    ///
    /// **The seat is the SELECTION's**, not one kind for both arms: a
    /// half reads a split and an index reads a pattern, and which of
    /// the two a node is, is a fact about the committed document. The
    /// refusal therefore names the kind the chosen selector wanted,
    /// which is what a user can act on — "that is a pattern, and a
    /// half comes out of a split".
    fn add_part(&mut self, of: RecipeNodeId, select: PartSelectSpec) -> OpOutcome {
        let wanted = match select {
            PartSelectSpec::SplitHalf(_) => NodeKindWanted::Split,
            PartSelectSpec::Instance(_) => NodeKindWanted::Instances,
        };
        if let Err(refusal) = self.require_kind(of, wanted) {
            return OpOutcome::refused(refusal);
        }
        self.commit(DocEdit::InsertNode {
            node: combine::part_node(of, select),
        })
    }

    /// Duplicate one body ([`SessionOp::Duplicate`]): a pattern of two
    /// over it, and one projection per instance.
    ///
    /// **Three inserts as ONE action and therefore one undo**, the
    /// shape [`Self::add_profile_on_new_xy`] takes and for its reason:
    /// the projections name the id the pattern insert MINTED, so each
    /// edit is built from what its predecessor produced rather than
    /// from a predicted id, and all-or-nothing comes free — a refusal
    /// anywhere leaves no half-built duplicate behind.
    ///
    /// **Why the projections are part of the gesture and not a
    /// follow-up.** The viewport draws `Doc::roots`, and `roots`
    /// maintenance drops a new node's inputs: the pattern alone is one
    /// root holding two bodies, which cannot be hidden, placed or
    /// blended one copy at a time, and the first `Part` authored
    /// afterwards would consume the pattern and leave the other copy
    /// undrawn. Committing both projections is what leaves two roots,
    /// so the copy is movable and the original stays on screen.
    fn add_duplicate(&mut self, input: RecipeNodeId) -> OpOutcome {
        if let Err(refusal) = self.require_kind(input, NodeKindWanted::Body) {
            return OpOutcome::refused(refusal);
        }
        // The kind gate above is the document's; this is the VALUE's —
        // one body, with a width to clear — and the value must be the
        // CURRENT document's. `busy` is the authority on that: it is
        // the session's own comparison of the landed generation against
        // the committed one, so an edit or a document replacement that
        // has not landed yet is refused here rather than measured off
        // the value it replaced.
        let step = match self.landed_pair() {
            None => Err(DuplicateFault::NotLanded),
            Some(_) if self.busy() => Err(DuplicateFault::Stale),
            Some((_, eval)) => combine::duplicate_step(eval, input, self.tol),
        };
        let step = match step {
            Ok(step) => step,
            Err(fault) => return OpOutcome::refused(Refusal::Duplicate(fault)),
        };
        let pattern = match combine::duplicate_rule(step) {
            Ok(rule) => combine::pattern_node(input, combine::DUPLICATE_COUNT, rule),
            Err(error) => return OpOutcome::refused(Refusal::Dimension(error)),
        };
        // Position 0 is the pattern; position `1 + i` projects instance
        // `i`, for every `i` below the pattern's own count. Instance 0
        // — the original, where it already stood — goes first, so it
        // takes the root slot the pattern took from the body it
        // replicates; each later projection's input has stopped being a
        // root by then, so it is APPENDED to the root list.
        self.commit_run(|minted| match minted.split_first() {
            None => Some(DocEdit::InsertNode {
                node: pattern.clone(),
            }),
            Some((Some(pattern), projections)) => i64::try_from(projections.len())
                .ok()
                .filter(|index| *index < combine::DUPLICATE_COUNT)
                .map(|index| DocEdit::InsertNode {
                    node: combine::part_node(*pattern, PartSelectSpec::Instance(index)),
                }),
            Some((None, _)) => {
                unreachable!("an `InsertNode` mints an id (`EditRecord::minted`)")
            }
        })
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

    /// **An edit that writes what the document already holds is not
    /// submitted** — one rule, one home, asked of the EDIT so every
    /// door that writes a panel field's value or its notation gets
    /// the same answer.
    ///
    /// **A different rule from the field's own guard, and they are
    /// two because they answer two questions.** `props::echoed` asks
    /// whether a TEXT came out of the field itself, which is the
    /// question about a click that typed nothing. This asks whether
    /// the DOCUMENT would move, which is the question about the undo
    /// step — and it has to be asked separately, because a widget
    /// hands one typed text over on more than one frame and because a
    /// person may re-type, in different characters, the number that
    /// already stands.
    ///
    /// **It is asked AFTER a door's refusals, never before.** A
    /// driven slot is owed its affordance even when the number
    /// offered happens to match what the computation produced, so
    /// [`Self::set_slot`] guards first and reaches this second.
    ///
    /// **Every other edit submits**, and that is the conservative
    /// direction rather than a gap: an insert, a delete or a rename
    /// has no standing value of its own to be equal to.
    fn writes_nothing(&self, edit: &DocEdit<ProfileProgram>) -> bool {
        let doc = self.committed_doc();
        match edit {
            // A slot's literal, against the expression the node
            // stands at — the same comparison for a structural slot,
            // which differs only in which edit carries it.
            DocEdit::SetParam { node, slot, expr }
            | DocEdit::SetStructuralParam { node, slot, expr } => {
                doc.node(*node).and_then(|node| node.expr(*slot)) == Some(expr)
            }
            // A declaration's two independent fields, each against
            // its own half. A kind that does not match is no match:
            // the edit is a redeclaration and the door refuses it.
            DocEdit::SetDocParamValue { name, value } => match (doc.params().get(name), value) {
                (
                    Some(DocParam::Continuous { value: stood, .. }),
                    DocParamValue::Continuous(offered),
                ) => stood == offered,
                (Some(DocParam::Count { value: stood }), DocParamValue::Count(offered)) => {
                    stood == offered
                }
                _ => false,
            },
            DocEdit::SetDocParamUnit { name, unit } => matches!(
                doc.params().get(name),
                Some(DocParam::Continuous { display_unit, .. }) if display_unit == unit
            ),
            // Every other edit submits. The structure of the recipe and
            // the shape of the product: a node inserted, deleted,
            // re-parented or re-pointed has no standing value of its
            // own for an offered one to equal.
            DocEdit::InsertNode { .. }
            | DocEdit::DeleteNode { .. }
            | DocEdit::SetMembers { .. }
            // A profile's program replaced whole: structure, not a
            // panel field's value — and the identity program keeping
            // every step is the door's own no-op.
            | DocEdit::SetProgram { .. }
            | DocEdit::SetRoots { .. }
            | DocEdit::Rebind { .. }
            | DocEdit::UpdateReference { .. }
            // An instance's offset, and the gauge an instance or a
            // gauge sits on, are structure: which frame a group stands
            // on, not a panel field's value.
            | DocEdit::SetOffset { .. }
            | DocEdit::SetGauge { .. }
            // The declaration doors that are not the value or the
            // notation half. `SetDocParam` is create-or-replace: a
            // redeclaration is an act — it is how a parameter's
            // DIMENSION changes — and the door refuses or performs it
            // on its own terms. A distribution is an annotation no
            // panel field shows.
            | DocEdit::SetDocParam { .. }
            | DocEdit::SetDocParamDistribution { .. }
            // A subtree rewrite at an `ExprPath`, which no panel door
            // emits: the comparison it would want is against the
            // REBUILT ancestor rather than against the payload, and
            // inventing one here would be a second opinion about what
            // that edit means.
            | DocEdit::SetExpression { .. }
            // Presentation, tolerance and witnesses — none of them a
            // value a panel field writes.
            | DocEdit::SetAppearance { .. }
            | DocEdit::ClearAppearance { .. }
            | DocEdit::SetAppearanceMeta { .. }
            | DocEdit::ClearAppearanceMeta { .. }
            | DocEdit::SetTolerance { .. }
            | DocEdit::ReWitness { .. }
            | DocEdit::ReWitnessBulk { .. } => false,
        }
    }

    /// [`Self::commit`] under [`Self::writes_nothing`]: the door for
    /// an edit that CHANGES a value or a notation, where writing what
    /// already stands is not an action and costs no undo step.
    fn commit_written(&mut self, edit: DocEdit<ProfileProgram>) -> OpOutcome {
        if self.writes_nothing(&edit) {
            return OpOutcome::default();
        }
        self.commit(edit)
    }

    /// **The one door an edit enters the document through**: apply,
    /// record, re-evaluate — and reconcile the display state, which is
    /// where a free-move probe is superseded by the mate that
    /// constrains its instance (the prune DISCARDS the value; see
    /// `display`'s module docs for why discard and not zero) and where
    /// a hide the picture can no longer honour is dropped. Both are
    /// reported on the outcome, each with the fault that caused it.
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

    /// The same door for an action that takes SEVERAL edits, a fixed
    /// list of them: one run of [`Self::commit_run`], so one user
    /// action is one undo.
    fn commit_action(&mut self, edits: Vec<DocEdit<ProfileProgram>>) -> OpOutcome {
        let mut edits = edits.into_iter();
        self.commit_run(|_| edits.next())
    }

    /// The same door again, for an action whose later edits name the
    /// ids its earlier ones MINTED: [`Self::stage_run`], then
    /// [`Self::record_run`].
    fn commit_run<F>(&mut self, next: F) -> OpOutcome
    where
        F: FnMut(&[Option<RecipeNodeId>]) -> Option<DocEdit<ProfileProgram>>,
    {
        match self.stage_run(next) {
            Ok(staged) => self.record_run(staged),
            Err(refusal) => OpOutcome::refused(refusal),
        }
    }

    /// **An action's edits, applied and not yet recorded.**
    ///
    /// `next` is handed what each edit so far minted, in order — an
    /// `InsertNode`'s new id, `None` for every edit that creates no
    /// node — and answers with the next edit, or `None` to end the
    /// run. The slice's LENGTH is how many edits have landed, so a
    /// caller that only inserts can match on its shape and a caller
    /// with a fixed list ([`Self::commit_action`]) ignores it.
    ///
    /// **All or nothing**: each edit is applied to the value the last
    /// one produced and nothing is recorded until [`Self::record_run`]
    /// takes the result, so a refusal anywhere — or a caller that
    /// drops the staged run — leaves the session on the document it
    /// started from. That is purity doing the work — no rollback
    /// exists to be got wrong.
    fn stage_run<F>(&self, mut next: F) -> Result<StagedRun, Refusal>
    where
        F: FnMut(&[Option<RecipeNodeId>]) -> Option<DocEdit<ProfileProgram>>,
    {
        // ONE reach for the whole action, over the session's own seam
        // (the directory rule; `None` refuses typed): an inserted
        // mate's clocking rider asks it, and nothing it decides is
        // recorded — the edits are the history.
        let resolver = self.resolver_seam();
        let reach = PartReach::<f64>::with_resolver(Some(&resolver), self.tol);
        // Threaded rather than cloned up front: the first `apply`
        // reads the history's value in place, and each later one reads
        // its predecessor's output, so a group of one costs exactly
        // what a single commit always cost.
        let mut produced: Option<Doc<ProfileProgram>> = None;
        let mut logged: Vec<DocEdit<ProfileProgram>> = Vec::new();
        let mut net = MaintenanceNet::new();
        let mut minted: Vec<Option<RecipeNodeId>> = Vec::new();
        while let Some(edit) = next(&minted) {
            let attempt = {
                let base = produced.as_ref().unwrap_or_else(|| self.history.doc());
                apply(base, &edit, self.tol, &reach)
            };
            match attempt {
                Ok(applied) => {
                    logged.push(edit);
                    minted.push(applied.record.minted);
                    net.push(&applied);
                    produced = Some(applied.doc);
                }
                Err(error) => return Err(Refusal::Edit(Box::new(error))),
            }
        }
        let Some(doc) = produced else {
            unreachable!("an action commits at least one edit")
        };
        Ok(StagedRun {
            doc,
            logged,
            net,
            minted,
        })
    }

    /// **A staged run, recorded**: the whole run is one history state,
    /// so one user action is one undo.
    fn record_run(&mut self, staged: StagedRun) -> OpOutcome {
        let StagedRun {
            doc,
            logged,
            net,
            minted,
        } = staged;
        let committed = logged.clone();
        // Net over the action: a row an earlier edit reported can be
        // made moot by a later one ([`MaintenanceNet`]).
        let maintenance = net.finish(&doc);
        self.history.commit_group(logged, doc);
        let pruned = self.display.prune(self.history.doc());
        self.request_eval();
        OpOutcome {
            committed,
            maintenance,
            // The ids the run minted, out to whoever asked for the
            // action — in the order the edits applied, which is the
            // order a caller that built one edit from another's id
            // reasoned in.
            minted: minted.into_iter().flatten().collect(),
            ..OpOutcome::from_prune(pruned)
        }
    }

    /// **The one submit**: mint the next generation and hand the shown
    /// document to the seam.
    fn request_eval(&mut self) {
        // **Every route that changes the shown document passes here**,
        // which is why the range probe is discarded here and nowhere
        // else: a commit, a gesture preview, an undo, an open. TWO
        // callers do not change the document — `Reevaluate`, and a
        // save-as into a different directory, which resubmits the same
        // document against a new resolver (`SessionOp::Save`'s row in
        // the mid-gesture table argues that half) — and discarding for
        // them too is the conservative direction: a range
        // recomputed on request costs a button press, a stale one costs
        // a wrong decision.
        self.derived.bounds = None;
        self.generation = self.generation.next();
        // ONE clone of the shown document per request: into the
        // `Arc` the session keeps, then out of it into the request the
        // seam owns. The second is the seam's vocabulary (`EvalRequest`
        // takes a value so a worker owns its copy) and is not a
        // retained copy — the session keeps exactly one.
        self.requested_doc = Arc::new(self.doc().clone());
        self.requested_resolver = self.run_resolver();
        self.eval.submit(EvalRequest {
            generation: self.generation,
            doc: self.requested_doc.as_ref().clone(),
            tol: self.tol,
            resolver: self.requested_resolver.clone(),
        });
    }
}

/// An action's edits applied in order and not yet recorded
/// ([`DocSession::stage_run`]).
struct StagedRun {
    /// The document the last edit produced.
    doc: Doc<ProfileProgram>,
    /// Each edit, for the history.
    logged: Vec<DocEdit<ProfileProgram>>,
    /// The maintenance, netted over the whole action.
    net: MaintenanceNet,
    /// What each edit minted, in the order the edits applied.
    minted: Vec<Option<RecipeNodeId>>,
}

/// Whether a document is assembly-shaped — one of the two conditions
/// [`AtRestBadge`] names for taking an A5 badge: a document that
/// instantiates no part declares no cross-instance rest and has
/// nothing for the gate to answer about.
fn assembly_shaped(doc: &Doc<ProfileProgram>) -> bool {
    doc.order()
        .iter()
        .filter_map(|&id| doc.node(id))
        .any(puts_an_instance)
}

/// Whether a node puts an instance of another document's part into
/// this one — what [`assembly_shaped`] asks of every node.
fn puts_an_instance(node: &Node<ProfileProgram>) -> bool {
    match node {
        Node::InstantiatePart { .. } => true,
        // Placements of a prototype drawn in THIS document: the rest
        // between them is the placement rule's, not a crossing.
        Node::PlacedUnion { .. } | Node::Pattern { .. } | Node::Part { .. } => false,
        // Relates instances some other node put in the document.
        Node::Mate { .. } => false,
        // A frame instances stand on; it puts nothing in.
        Node::Gauge { .. } => false,
        // Declares contacts between faces of a consumer's operands,
        // and puts no body of its own in.
        Node::Declare { .. } => false,
        Node::Datum(_)
        | Node::Profile(_)
        | Node::Extrude { .. }
        | Node::Revolve { .. }
        | Node::Tube { .. }
        | Node::HollowTube { .. }
        | Node::Loft { .. }
        | Node::Sweep { .. }
        | Node::Fillet { .. }
        | Node::Chamfer { .. }
        | Node::Shell { .. }
        | Node::Split { .. }
        | Node::Boolean { .. }
        | Node::Union { .. }
        | Node::Transform { .. }
        | Node::Measure { .. }
        | Node::Assertion { .. } => false,
    }
}

/// One A5 verdict as the badge that shows it — the gate's own
/// vocabulary either way: a certification with its minted count, or
/// the typed refusal rendered by its own `Display` — **and the
/// aggregate the gate hands back with it**.
///
/// The gate CONSUMES the product it judges. A certification returns
/// the same body on its `Assembly` and a refusal returns nothing, so
/// the body is an `Option` here for the same reason
/// [`Gathered::body`] is one, and this is the one place that fact is
/// read off the gate's own result type.
fn badge(verdict: Result<Assembly<f64>, AssemblyError>) -> (AtRestBadge, Option<Arc<Body<f64>>>) {
    match verdict {
        Ok(assembly) => (
            AtRestBadge::Certified {
                minted: assembly.minted.len(),
            },
            Some(Arc::new(assembly.body)),
        ),
        Err(refusal) => (
            AtRestBadge::Refused {
                message: refusal.to_string(),
            },
            None,
        ),
    }
}

/// The directory a document at `path` resolves against — its parent,
/// with a bare filename reading as the current directory.
fn session_dir(path: &Path) -> PathBuf {
    match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
        _ => PathBuf::from("."),
    }
}

/// Exhaustive by destructuring; the shared rule is
/// `crates/viewer/README.md`'s. [`Derived`] renders as ONE field, so
/// its members travel with their declaration rather than being listed
/// a second time here.
///
/// The four `_` arms, one reason each: `tol` is `Tol(())`, a ZST with
/// no content to print; `eval` is a `dyn` service and implements no
/// `Debug`; `requested_doc` is a whole recipe DAG; and `display` is
/// not derived from the document, is as large as the document's hidden
/// and moved sets, and has its own [`DocSession::display`] door to be
/// dumped through.
///
/// Three carried fields are summaries and each says so where it
/// renders: `states` is the history's LENGTH, and a count cannot be
/// read as the `Vec`; `gesture` and `resolver` are elisions naming
/// what is there, `Some(<Gesture>)` and `Some(<DirResolver>)`, rather
/// than the `bool` their presence is. `finish_non_exhaustive` is about
/// the `_` arms above and is true for reasons unrelated to these.
impl core::fmt::Debug for DocSession {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            history,
            tol: _,
            gesture,
            eval: _,
            generation,
            requested_doc: _,
            requested_resolver: _,
            derived,
            path,
            display: _,
            resolver,
            notation,
        } = self;
        f.debug_struct("DocSession")
            .field("generation", generation)
            .field("states", &history.len())
            .field(
                "gesture",
                &gesture.held().map(|_| format_args!("<Gesture>")),
            )
            .field("path", path)
            .field(
                "resolver",
                &resolver.as_ref().map(|_| format_args!("<DirResolver>")),
            )
            .field("derived", derived)
            .field("notation", notation)
            .finish_non_exhaustive()
    }
}
