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

use pncad::document::pose::{PoseRole, RoleRead};
use pncad::document::{
    CheckId, ChecksConfig, ChecksReport, Dimension, DimensionError, Doc, DocEdit, DocRef,
    DocumentId, EditError, EvalOptions, Evaluation, Formula, Frame, FreeValue, FreeVar, HeldNodes,
    Label, LoopProgram, Maintenance, Node, Operand, OperandSlot, PartResolver, ProductError,
    ProfileProgram, RecipeNodeId, Recorded, Recording, SlotId, StepId, Subject, VarId, VarName,
    apply, cascade_delete_order, parse_formula, product_recorded, run_checks_at_rest_on,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::quantity::UnitDef;
use pncad::select::Resolution;
use pncad::topo::Body;

use crate::blend::BlendKindChoice;
use crate::combine::{self, DuplicateFault, PatternOutputChoice};
use crate::display::{DisplayFault, DisplayState, DisplayView};
use crate::docio::{self, DirResolver, NoFile};
use crate::evalseam::{
    DoorDone, DoorRequest, DoorService, EvalRequest, EvalService, InlineEvaluator, InlineProver,
    InlineStager, StagedKey, StagedRequest, StagedService,
};
use crate::g1;
use crate::generation::Generation;
use crate::history::{History, HistoryId};
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
mod snap;

pub use author::{BooleanSpec, PartSelectSpec, PatternRuleSpec, ProfileShape};
pub use delete::DeleteAffordance;
pub use op::{
    CancelDoor, Creation, FreeMoveName, GestureName, OpOutcome, SessionOp, ValueGestureName,
};
pub use probe::{BoundsReading, BoundsTarget};
pub use refuse::{
    GestureFrameFault, NodeKindWanted, Refusal, SaveChoice, Step, VersionOffer, admits, admits_read,
};
pub use select::{EdgeSelection, FaceSelection, Hovered, Selection, Standing, VertexSelection};

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
    /// A document variable, with the dimension it is declared at.
    ///
    /// **No unit, and it is not the slot arm's omission.** A slot's
    /// edit rebuilds the literal, so the notation has to be carried
    /// into it or the drag rewrites it; a variable's edit is the
    /// value door (`DocEdit::SetVarValue`), which writes a number
    /// into the standing declaration and leaves the authored unit
    /// beside it untouched. There is nothing here for a captured unit
    /// to protect. The panel still SHOWS the drag in the variable's
    /// written unit — it converts before the value crosses into this
    /// layer, which is canonical throughout.
    Variable { var: VarId, dimension: Dimension },
}

impl GestureTarget {
    /// This target's dimension — a slot's from its `SlotId`, a
    /// variable's from its declaration.
    fn dimension(&self) -> Dimension {
        match self {
            Self::Slot { slot, .. } => slot.expr_dimension(),
            Self::Variable { dimension, .. } => *dimension,
        }
    }

    /// Which arm a dragged `f64` becomes, through
    /// [`SlotValue::of`] — the one home for that rule, refusal
    /// included.
    ///
    /// # Errors
    ///
    /// [`SlotValue::of`]'s, which is `Formula::literal`'s own
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
            Self::Variable { var, .. } => ValueGestureName::Variable(*var),
        }
    }

    /// The edit that writes `value` into this target.
    ///
    /// A variable's edit is the VALUE door, so the variable's
    /// declaration — its dimension and any distribution — is read off
    /// the document by the edit itself rather than reassembled here.
    fn edit(
        &self,
        doc: &Doc<ProfileProgram>,
        value: SlotValue,
    ) -> Result<DocEdit<ProfileProgram>, DimensionError> {
        match self {
            Self::Slot { node, slot, unit } => props::slot_edit(doc, *node, *slot, value, *unit),
            Self::Variable { var, .. } => Ok(props::variable_edit(*var, value)),
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
/// drag, and the probe's three are about a space's display frame
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
        .ok_or_else(|| Refusal::NoSuchSlot {
            node: doc.spoken(node),
            slot,
        })?;
    Ok((row.driver, row.value.ok().and_then(|r| r.one())))
}

/// Whether `expr` reads any variable, by id or by name.
fn reads_a_variable(expr: &Formula) -> bool {
    let (mut ids, mut names) = (Vec::new(), Vec::new());
    expr.var_reads(&mut ids);
    expr.named_reads(&mut names);
    !ids.is_empty() || !names.is_empty()
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
        SlotDriver::Expression { variables } => Err(Refusal::DrivenByExpression {
            node,
            slot,
            variables,
            current,
            notation,
        }),
    }
}

/// **A profile editor's program with every unmoved argument of every
/// kept step carried from the committed program** — the variable the
/// document holds, so its identity and its distribution survive the
/// reshaping; what moved is written as the program holds it.
///
/// A step is the committed one when `ids` keeps its id; the argument
/// is the same one when it has the same
/// [`pncad::document::StepArg`] on that step, and unmoved when it reads
/// the committed variable or is written as the committed argument is
/// written ([`sketch::written_as`]: the bits and the unit).
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
    loops: Vec<LoopProgram<Formula>>,
    ids: &[Vec<Option<StepId>>],
    notation: Notation,
) -> Result<Vec<LoopProgram<Formula>>, Refusal> {
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
        loops,
        ids: Vec::new(),
        on: None,
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
        let Some(&committed) = old.expr(slot) else {
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
        // An argument written as it was keeps the variable it read, so
        // its identity and its distribution survive the reshaping
        // (`Node::authored`'s rule).
        let reader = Formula::var(committed, arg.dimension());
        let written =
            Formula::from(doc.written(&pncad::document::Expr::var(committed, arg.dimension())));
        match new.expr_mut(moved) {
            Some(held) if held.bit_eq(&reader) || sketch::written_as(held, &written) => {
                *held = reader;
            }
            _ if doc.is_typed_value(committed) => {}
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
    /// **The coincidence door's seam**: the landed report's door runs
    /// here, at rest, and the next request cancels it
    /// ([`DocSession::request_eval`]).
    door: Box<dyn DoorService>,
    /// **The staged seam**: an operation staged and not committed runs
    /// here for its verdict ([`DocSession::stage`]).
    stager: Box<dyn StagedService>,
    /// How many stages have been asked for: the next one's key.
    stages: u64,
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
    /// What the prunes on landed evaluations withdrew and nobody has
    /// taken yet ([`DocSession::take_landed_withdrawals`]). A landing is
    /// not an operation, so its withdrawals ride no [`OpOutcome`].
    landed_withdrawals: Vec<crate::display::PruneReport>,
    /// **The frames "Add part…" gestures write**, by the history state
    /// each gesture's action made: measured once the evaluation answering
    /// the insert lands, taken back by that state's undo and written
    /// again by its redo. Keyed by the state, never by the space, so an
    /// action that mints the same space id elsewhere inherits nothing.
    inserts: std::collections::BTreeMap<HistoryId, Insert>,
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
    /// The nodes [`Derived::selection`] names, as the last document that
    /// held them spoke them ([`DocSession::selection_said`]).
    said: HeldNodes,
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
    /// The slot whose typed value is offered the variables of equal
    /// value ([`DocSession::offered`]), and the variable that typing
    /// minted. It stands until it is accepted or declined, or until the
    /// slot no longer reads that variable at the value typed — a retype,
    /// a drag of that very variable, a move of it by its own door — or
    /// the history steps (an undo, a redo), so an offer is only ever
    /// made about a value as it was typed. An edit elsewhere leaves it.
    offer: Option<SlotOffer>,
    /// **The latest stage** ([`SessionOp::Stage`]) and what it came to:
    /// a statement about the document it was staged on, so dropped with
    /// the range probe at every submit ([`DocSession::request_eval`]),
    /// and with everything else here when the document is replaced.
    stage: Option<Stage>,
}

/// A typed value's offer: the slot it was typed at, the variable it
/// minted there, and the value typed.
#[derive(Clone, Copy, Debug, PartialEq)]
struct SlotOffer {
    node: RecipeNodeId,
    slot: SlotId,
    var: VarId,
    typed: props::SlotValue,
}

impl SlotOffer {
    /// Whether the slot still reads the variable typing minted, at the
    /// bits typed.
    fn stands(&self, doc: &Doc<ProfileProgram>) -> bool {
        doc.slot(self.node, self.slot) == Some(self.var)
            && props::variable_value(doc, self.var)
                .is_some_and(|now| props::bit_equal(now, self.typed))
    }
}

impl Derived {
    /// Nothing known about the document underneath: nothing picked,
    /// nothing hovered, no preview, nothing landed, no range probed.
    fn none() -> Self {
        Self {
            selection: Selection::None,
            said: HeldNodes::default(),
            hover: None,
            scratch: None,
            landed: None,
            bounds: None,
            offer: None,
            stage: None,
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
            said,
            hover,
            scratch,
            landed,
            bounds,
            offer,
            stage,
        } = self;
        f.debug_struct("Derived")
            .field("selection", selection)
            .field("said", said)
            .field("hover", hover)
            .field("scratch", &scratch.as_ref().map(|_| format_args!("<Doc>")))
            .field("landed", landed)
            .field("bounds", bounds)
            .field("offer", offer)
            .field("stage", &stage.as_ref().map(|stage| stage.key))
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
/// the gather would have produced, so no at-rest badge and no body can
/// sit beside a gather refusal. That is all the shape rules out.
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
/// `Ok`): the at-rest badge read off the checks report, and the
/// product's body.
struct Gathered {
    /// The at-rest badge for this pair ([`DocSession::at_rest`]);
    /// `None` where [`AtRestBadge`] says none is taken.
    at_rest: Option<AtRestBadge>,
    /// **The aggregate the landing's own gather produced**, in the
    /// world's coordinates ([`crate::scene::product_in_world`]), kept so
    /// that a consumer which needs the product does not gather it a
    /// second time ([`DocSession::landed_body`]). Written once, by
    /// `land`.
    ///
    /// `viewer/tests/landing_gathers.rs` counts the gathers of every
    /// path that must not pay one, so a change that made
    /// [`DocSession::landed_body`] gather, or that stopped `land`
    /// keeping the body, reds there.
    body: Arc<Body<f64>>,
}

/// Exhaustive by destructuring, through [`Gathered`] too; the shared
/// rule is `crates/viewer/README.md`'s.
///
/// The two `_` arms are the run's DATA — `evaluation` is the result
/// DAG and `doc` is the recipe DAG it answers — and everything else
/// here is a verdict ABOUT that pair. The gather renders as the arm it
/// took: its refusal as `fault`, or what it left as `at_rest` and
/// `body`. `checks` is a `Vec` per finding and is carried as its two
/// counts; `body` is a gathered aggregate and is carried as the elision
/// `<Body>`. Both render as SUMMARIES, so neither can be read as the
/// field's own value; `finish_non_exhaustive` here is about the `_`
/// arms and says nothing about them.
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
            Ok(Gathered { at_rest, body: _ }) => out
                .field("at_rest", at_rest)
                .field("body", &format_args!("<Body>")),
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

/// **The at-rest badge for the landed pair**: the at-rest check's
/// findings over the landed product and every `⊔` beside it, counted
/// (`CheckId::AtRest` in the landing's checks report). Nothing at rest
/// refuses, so the badge says what the check found: overlaps between
/// copies, loud and quiet, and the findings the census could not look at.
///
/// Taken only for an assembly-shaped document (one holding at least
/// one `InstantiatePart`) whose product gathered and whose checks ran.
/// A part document's tiers are not this badge's subject. A gather
/// refusal, of any class, leaves no product to read, so it takes no
/// badge: it is [`DocSession::product_fault`]'s, and
/// `frame::badge_site` routes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtRestBadge {
    /// The interference findings no assertion quiets.
    pub loud: usize,
    /// The interference findings an assertion quiets.
    pub quiet: usize,
    /// The findings the census could not look at, which nothing quiets.
    pub could_not_look: usize,
}

impl AtRestBadge {
    /// The badge `report`'s at-rest findings make.
    #[must_use]
    pub fn of(report: &ChecksReport) -> Self {
        let mut badge = Self {
            loud: 0,
            quiet: 0,
            could_not_look: 0,
        };
        for finding in report
            .findings
            .iter()
            .filter(|f| f.check == CheckId::AtRest)
        {
            if finding.evidence.could_not_look() {
                badge.could_not_look += 1;
            } else if finding.quiet.is_some() {
                badge.quiet += 1;
            } else {
                badge.loud += 1;
            }
        }
        badge
    }
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
/// `crates/viewer/README.md`, The session's vocabularies, argues why,
/// and why [`crate::frame::Progress`] shares two of these names.
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
            door: Box::new(InlineProver::new()),
            stager: Box::new(InlineStager::new()),
            stages: 0,
            generation: Generation::FIRST,
            derived: Derived::none(),
            path: None,
            display: DisplayState::new(),
            landed_withdrawals: Vec::new(),
            inserts: std::collections::BTreeMap::new(),
            resolver: None,
            notation: Notation::DEFAULT,
        };
        session.request_eval();
        session
    }

    /// This session with its coincidence door run through `door`, in
    /// place of the inline seam [`DocSession::new`] gives it, which runs
    /// a landing's door inside [`DocSession::pump`]. Taken before the
    /// first pump: a job the replaced seam holds is dropped with it.
    #[must_use]
    pub fn with_door(mut self, door: Box<dyn DoorService>) -> Self {
        self.door = door;
        self
    }

    /// This session with its staged runs taken through `stager`, in
    /// place of the inline seam [`DocSession::new`] gives it, which runs
    /// a stage inside [`DocSession::pump`].
    #[must_use]
    pub fn with_stager(mut self, stager: Box<dyn StagedService>) -> Self {
        self.stager = stager;
        self
    }

    /// **Stage a creating operation without committing it**
    /// ([`SessionOp::Stage`]): the run its door would record, staged on
    /// the committed document and handed to the staged seam to evaluate
    /// beside the picture. A refusal is the stage's answer, not the
    /// status line's: a draft asks, and nothing was tried. A later stage
    /// supersedes this one, and its run's answer is dropped.
    fn stage(&mut self, creation: Creation) {
        self.stages += 1;
        let key = StagedKey(self.stages);
        let staged = self
            .creation_door(creation.clone().into_op())
            .and_then(|door| self.stage_run(|run| door(run)));
        let answer = match staged {
            Err(refusal) => StageAnswer::Refused(refusal),
            Ok((recorded, node)) => {
                let resolver = self.run_resolver();
                self.stager.submit(StagedRequest {
                    key,
                    doc: recorded.doc.clone(),
                    prime: self.memo_under(&resolver),
                    resolver,
                    tol: self.tol,
                });
                StageAnswer::Made {
                    node,
                    doc: recorded.doc,
                    evaluation: None,
                }
            }
        };
        self.derived.stage = Some(Stage {
            key,
            creation,
            answer,
        });
    }

    /// **What staging `creation` on the committed document came to**:
    /// `None` until it is staged there ([`SessionOp::Stage`]), so a
    /// draft that moved asks again, and so does one whose document
    /// moved: the stage lives in [`Derived`] and goes with it.
    pub fn staged_for(&self, creation: &Creation) -> Option<Staged<'_>> {
        let stage = self
            .derived
            .stage
            .as_ref()
            .filter(|stage| stage.creation == *creation)?;
        Some(match &stage.answer {
            StageAnswer::Refused(refusal) => Staged::Refused(refusal),
            StageAnswer::Made {
                evaluation: None, ..
            } => Staged::Checking,
            StageAnswer::Made {
                node,
                doc,
                evaluation: Some(evaluation),
            } => Staged::Evaluated {
                doc,
                node: *node,
                evaluation,
            },
        })
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

    /// **The selection's nodes as the last document that held them
    /// spoke them** ([`Selection::nodes`]): what a sentence about the
    /// selection says a node by once the document it is spoken from no
    /// longer holds it (`Speaker::or_held`). Spoken when the selection
    /// is made, and again from the shown document after every
    /// operation (`SpokenNode::respoken`'s rule), so a deleted node
    /// is said by the last label it had.
    pub fn selection_said(&self) -> &HeldNodes {
        &self.derived.said
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
            Selection::Variable(var) => Standing::Variable {
                var: self.doc().spoken_var(*var),
                present: self.doc().var(*var).is_some(),
            },
            Selection::Face(face) => Standing::Face {
                face: face.clone(),
                resolution: self.entity_resolution(face.node, face.body, &face.name),
            },
            Selection::Edge(edge) => Standing::Edge {
                edge: edge.clone(),
                resolution: self.entity_resolution(edge.node, edge.body, &edge.name),
            },
            Selection::Vertex(vertex) => Standing::Vertex {
                vertex: vertex.clone(),
                resolution: self.entity_resolution(vertex.node, vertex.body, &vertex.name),
            },
        }
    }

    /// **The nodes `selection` names, as the session speaks them now**:
    /// each as the shown document holds it, or as the landed run's
    /// document does where the shown one no longer holds it. A selection
    /// is picked in one of the two: the feature tree draws the shown
    /// document, and the viewport and the Checks window the landed run.
    /// Both are versions of this session's one document, so an id
    /// names one node in each (`SpokenNode::respoken`'s soundness
    /// paragraph); a replaced document drops the landed run with the
    /// selection.
    fn spoken_now(&self, selection: &Selection) -> HeldNodes {
        let landed = self.landed_pair().map(|(doc, _)| doc);
        selection
            .nodes()
            .into_iter()
            .map(|id| {
                let shown = self.doc().spoken(id);
                match landed {
                    Some(landed) if shown.kind().is_none() => landed.spoken(id),
                    Some(_) | None => shown,
                }
            })
            .collect()
    }

    /// One picked name's verdict against the landed run — the shipped
    /// `resolve` door, asked once and spelled once for both entity
    /// kinds, with the node it waits on named as the feature tree
    /// names it ([`crate::tree::resolution_as_drawn`]).
    fn entity_resolution(
        &self,
        node: RecipeNodeId,
        body: u32,
        name: &StableName,
    ) -> Option<Box<Resolution>> {
        self.landed_pair()
            .map(|(doc, eval)| Box::new(crate::tree::pick_resolution(doc, eval, node, body, name)))
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

    /// **The evaluation that answers the shown document's spaces**
    /// (DI5): the landed one, when the document it answers is the
    /// committed one or the shown one; `None` while it answers an older
    /// document, until the next landing — what free-move admission
    /// reads ([`crate::display::free_move_check`]).
    ///
    /// Mid-drag the two differ only by a value gesture's edits, which
    /// leave the node graph alone
    /// ([`SessionOp::permitted_during_value_gesture`]), and which space
    /// a copy is placed into is a function of that graph — each
    /// placement's `target` — so a landing of either answers both.
    pub fn space_evaluation(&self) -> Option<&Evaluation<f64>> {
        space_evaluation(&self.derived, &self.history)
    }

    /// The part files the landed run's resolver could name
    /// ([`PartFiles`]): what the tree names an instance's part by.
    /// Unscanned while nothing has landed.
    pub fn part_files(&self) -> &PartFiles {
        static UNSCANNED: PartFiles = PartFiles::Unscanned;
        self.derived
            .landed
            .as_ref()
            .map_or(&UNSCANNED, |run| &run.files)
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

    /// The at-rest badge for the landed pair ([`AtRestBadge`]),
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

    /// **The variables a value typed at `slot` on `node` is offered**
    /// (D10: typing a value offers an existing variable of equal
    /// value): empty unless a value was typed there, still stands as
    /// typed, and has not been accepted or declined — a drag, a retype,
    /// an undo or a redo closes it — and empty where no variable
    /// equals it ([`props::equal_variables`]). Read off the committed
    /// document, which is the one the accepting edit applies to.
    pub fn offered(&self, node: RecipeNodeId, slot: SlotId) -> Vec<props::Offered> {
        let doc = self.committed_doc();
        match self.derived.offer {
            Some(offer) if offer.node == node && offer.slot == slot && offer.stands(doc) => {
                props::equal_variables(doc, node, slot)
            }
            Some(_) | None => Vec::new(),
        }
    }

    /// **The gathered product of the landed run** — the aggregate a
    /// display fit sizes itself on, and the aggregate a scene is
    /// tessellated from.
    ///
    /// **Handed over rather than re-derived**, and **free to ask**:
    /// this is a borrow of what the landing already gathered, never a
    /// gather of its own. On this lane's 165-body, 990-face
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
    /// `None` has two causes and they are not the same question:
    /// nothing has landed, or the gather REFUSED, so no product exists
    /// ([`DocSession::product_fault`] says so).
    pub fn landed_body(&self) -> Option<&Body<f64>> {
        Some(&self.derived.landed.as_ref()?.gather.as_ref().ok()?.body)
    }

    /// The advisory-check report for the landed pair — findings in
    /// deterministic order, the residents that were configured `Off`,
    /// and the coincidence door while it runs at rest
    /// ([`ChecksReport::pending`]). `None` before anything lands, or
    /// when the registry refused (which is distinct from an empty
    /// report: "not checked" and "checked and fine" are different
    /// answers).
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

    /// **Every show/hide toggle** ([`crate::display::Toggles`]), read
    /// over the committed document — the one the toggle's own door
    /// decides against ([`SessionOp::SetBodyOn`]), so what a checkbox
    /// shows and what its click does read one document.
    pub fn toggles(&self) -> crate::display::Toggles {
        self.display.toggles(self.committed_doc())
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
        while let Some(done) = self.door.poll() {
            self.settle(done);
        }
        while let Some(done) = self.stager.poll() {
            if let Some(Stage {
                answer: StageAnswer::Made { evaluation, .. },
                ..
            }) = self
                .derived
                .stage
                .as_mut()
                .filter(|stage| stage.key == done.key)
            {
                *evaluation = Some(done.evaluation);
            }
        }
        landings
    }

    /// **Settles a door job into the landed report it was split from**
    /// — only that one: an answer for a landing since replaced, or one
    /// the next request canceled, is dropped, and a report whose door
    /// was canceled goes on reading it pending until a landing answers
    /// again. A second answer for a report already settled is dropped
    /// too.
    pub fn settle(&mut self, done: DoorDone) {
        let Some(run) = self.derived.landed.as_mut() else {
            return;
        };
        if run.generation != done.generation {
            return;
        }
        if let (Some(report), Ok(findings)) = (run.checks.as_mut(), done.findings)
            && report.pending.contains(&CheckId::UnprovenCoincidence)
        {
            report.settle(findings);
        }
    }

    /// Whether a staged run is still running ([`SessionOp::Stage`]).
    pub fn staging(&self) -> bool {
        self.stager.busy()
    }

    /// Whether the landed report's coincidence door is still running
    /// at rest.
    pub fn proving(&self) -> bool {
        self.door.busy()
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
        // ONCE: the product's own verdict reads the refusal, the
        // registry BORROWS the product, the at-rest badge reads the
        // registry's report, and the SESSION keeps the body for the
        // display fit ([`Gathered::body`]).
        let doc: &Doc<ProfileProgram> = &self.requested_doc;
        let cfg = ChecksConfig::default();
        // Whether the document is assembly-shaped is a fact about its
        // nodes: the `Ok` arm reads it for the badge, and the part-file
        // scan below reads it too.
        let assembly_shaped = assembly_shaped(doc);
        let (gather, checks) = match product_recorded(doc, &done.evaluation, self.tol) {
            Ok(product) => {
                // The advisory registry. It REPORTS — a document with
                // findings still draws, which is the whole point of
                // running it on the draw path: a product whose copies
                // interpenetrate renders a picture that looks almost
                // right, and the finding is the only thing that says
                // otherwise. A refusal of the registry itself leaves
                // no report rather than a clean one: "not checked" is
                // not "checked and fine".
                let checks = run_checks_at_rest_on(
                    doc,
                    &done.evaluation,
                    Subject::Product(&product),
                    &cfg,
                    self.tol,
                )
                .ok();
                let at_rest = checks
                    .as_ref()
                    .filter(|_| assembly_shaped)
                    .map(|(report, _)| AtRestBadge::of(report));
                // Kept as the picture draws it: in the world, composed
                // by the scene's door, never in the gather's computing
                // frame.
                let gather =
                    crate::scene::product_in_world(product, doc, self.tol).map(|body| Gathered {
                        at_rest,
                        body: Arc::new(body),
                    });
                (gather, checks)
            }
            Err(fault) => {
                // **The product's own verdict.** The gather is the only
                // thing that answers "is this document's product well
                // formed", so every class of refusal is kept here; which
                // channel reports which is `frame::badge_site`'s. There
                // is no product to read, so no at-rest badge is taken.
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
                        run_checks_at_rest_on(
                            doc,
                            &done.evaluation,
                            Subject::EmptyProduct,
                            &cfg,
                            self.tol,
                        )
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
        // **The door runs at rest**: the report reads it pending until
        // the job lands ([`DocSession::settle`]).
        let (checks, door) = checks.map_or((None, None), |(report, door)| (Some(report), door));
        if let Some(job) = door {
            self.door.submit(DoorRequest {
                generation: done.generation,
                doc: Arc::clone(&self.requested_doc),
                job,
            });
        }
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
        // **The second prune** (DI5): whether a free-move gesture's copy
        // still evaluates is asked here, of the evaluation and the
        // document it answers, rather than when the edit committed. A
        // landing of a value gesture's scratch shares the committed
        // graph but not its values, so it prunes on what the graph
        // decides alone (`DisplayState::prune_structural`), and a
        // placement a previewed value refuses — which the user may
        // still cancel — discards nothing committed.
        if let Some(run) = self.derived.landed.as_ref() {
            let eval = run.evaluation.as_ref();
            let report = if *self.requested_doc == *self.history.doc() {
                self.display.prune(&self.requested_doc, Some(eval))
            } else {
                self.display.prune_structural(&self.requested_doc, eval)
            };
            if !report.is_empty() {
                self.landed_withdrawals.push(report);
            }
            if *self.requested_doc == *self.history.doc() {
                // Each insert on the path whose frame is not measured yet,
                // in history order, clear of what is drawn and of the
                // frames measured before it — never of a space still
                // waiting, which stands where it will not stay.
                let path = self.history.path();
                let mut waiting: Vec<HistoryId> = path
                    .iter()
                    .copied()
                    .filter(|state| {
                        self.inserts.get(state).is_some_and(|insert| {
                            insert.frame.is_none() && self.requested_doc.var(insert.space).is_some()
                        })
                    })
                    .collect();
                while !waiting.is_empty() {
                    let state = waiting.remove(0);
                    let ignored: std::collections::BTreeSet<VarId> = waiting
                        .iter()
                        .map(|later| self.inserts[later].space)
                        .collect();
                    let Some(insert) = self.inserts.get_mut(&state) else {
                        continue;
                    };
                    let view = self.display.view(&self.requested_doc);
                    let frame = crate::pickindex::clear_of_drawn(
                        &self.requested_doc,
                        eval,
                        &view,
                        insert.space,
                        &ignored,
                        self.tol,
                    )
                    .unwrap_or(Frame::IDENTITY);
                    insert.frame = Some(frame);
                    if !frame.is_identity_bits() {
                        self.display.place_space(insert.space, insert.node, frame);
                    }
                }
            }
        }
        Landing::Landed
    }

    /// **What the landings' prunes withdrew** since the last call, in
    /// landing order: a gesture whose copy no longer evaluates. The
    /// chrome reports them as it
    /// reports an operation's [`OpOutcome::withdrawn`].
    pub fn take_landed_withdrawals(&mut self) -> Vec<crate::display::PruneReport> {
        core::mem::take(&mut self.landed_withdrawals)
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
    /// [`SessionOp::BeginGesture`] and [`SessionOp::BeginVariableGesture`]
    /// in the value table, [`SessionOp::BeginFreeMove`] in the
    /// free-move one — and the set of operations a drag refuses is its
    /// table plus that one rule, held once for both drags rather than
    /// spelled per gesture and per table.
    pub fn perform(&mut self, op: SessionOp) -> OpOutcome {
        let outcome = self.perform_op(op, None);
        // Every document change is an operation, so this is the one
        // place the kept nodes follow the newest document that holds
        // them. A landing changes only the landed run, an earlier
        // version of the shown document.
        self.derived.said = self.derived.said.respoken(self.doc());
        // And the one place an offer closes once its value no longer
        // stands as typed: closed for good, so a value moved away and
        // back is never offered again — an offer is made straight after
        // a typing op and never by any other.
        let doc = self.history.doc();
        self.derived.offer = self.derived.offer.filter(|offer| offer.stands(doc));
        outcome
    }

    /// [`DocSession::perform`]'s operation, before the selection's kept
    /// nodes are spoken again — labelled `labelled` where it is a
    /// [`SessionOp::CreateLabelled`]'s creation.
    ///
    /// **The one dispatch.** A creating op's arm hands its door's
    /// [`Creating`] run, with the label, to [`Self::create`]; the label
    /// reaches no door. The match is exhaustive over [`SessionOp`], and a
    /// creating door answers a [`Creating`] rather than an outcome, so a
    /// creation that bypasses [`Self::create`] does not compile.
    fn perform_op(&mut self, op: SessionOp, mut labelled: Option<Label>) -> OpOutcome {
        if self.gesture.held().is_some() && !op.permitted_during_value_gesture() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        if self.display.probing().is_some() && !op.permitted_during_free_move() {
            return OpOutcome::refused(Refusal::Display(DisplayFault::FreeMoveInFlight));
        }
        let outcome = match op {
            SessionOp::Select(selection) => {
                self.derived.said = self.spoken_now(&selection);
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
            SessionOp::Stage { creation } => {
                self.stage(creation);
                OpOutcome::default()
            }
            SessionOp::SetSlotUnit { node, slot, unit } => self.set_slot_unit(node, slot, unit),
            SessionOp::SetSlotExpression { node, slot, text } => {
                self.set_slot_expression(node, slot, &text)
            }
            SessionOp::SetSlotVariable {
                node,
                slot,
                var,
                name,
            } => self.set_slot_variable(node, slot, var, name),
            SessionOp::DeclineOffer { node, slot } => {
                self.close_offer(node, slot);
                OpOutcome::default()
            }
            SessionOp::SetVariable { var, value } => self.set_variable(var, value),
            SessionOp::SetVariableUnit { var, unit } => self.set_variable_unit(var, unit),
            SessionOp::SetVariableText { var, text } => self.set_variable_text(var, &text),
            SessionOp::DeclareVar { name, value } => self.declare_var(name, value),
            SessionOp::RenameVar { var, name } => self.commit(DocEdit::RenameVar {
                var: var.into(),
                name,
            }),
            SessionOp::DeleteVar { var } => self.commit(DocEdit::DeleteVar { var: var.into() }),
            SessionOp::BeginGesture { node, slot } => self.begin_gesture(node, slot),
            SessionOp::BeginVariableGesture { var } => self.begin_variable_gesture(var),
            SessionOp::PreviewGesture { node, slot, value } => {
                self.preview_gesture(&ValueGestureName::Slot { node, slot }, value)
            }
            SessionOp::CommitGesture { node, slot } => {
                self.commit_gesture(&ValueGestureName::Slot { node, slot })
            }
            SessionOp::PreviewVariableGesture { var, value } => {
                self.preview_gesture(&ValueGestureName::Variable(var), value)
            }
            SessionOp::CommitVariableGesture { var } => {
                self.commit_gesture(&ValueGestureName::Variable(var))
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
            SessionOp::SaveAsNewDocument(path) => self.save_as_new_document(&path),
            SessionOp::SetBodyOn { node, on } => {
                match self.display.set_body_on(self.history.doc(), node, on) {
                    Ok(()) => OpOutcome::default(),
                    Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
                }
            }
            SessionOp::SetSpaceHidden { space, hidden } => {
                match self
                    .display
                    .set_space_hidden(self.history.doc(), space, hidden)
                {
                    Ok(()) => OpOutcome::default(),
                    Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
                }
            }
            SessionOp::BeginFreeMove { node } => {
                // `space_evaluation` over the two fields it reads: the
                // display state beside them is borrowed mutably.
                let eval = space_evaluation(&self.derived, &self.history);
                match self.display.begin_free_move(self.history.doc(), eval, node) {
                    Ok(()) => OpOutcome::default(),
                    Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
                }
            }
            SessionOp::PreviewFreeMove { node, frame } => {
                match self
                    .display
                    .preview_free_move(self.history.doc(), node, frame, self.tol())
                {
                    Ok(()) => OpOutcome::default(),
                    Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
                }
            }
            SessionOp::CommitFreeMove { node } => {
                match self.display.commit_free_move(self.history.doc(), node) {
                    Ok(()) => OpOutcome::default(),
                    Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
                }
            }
            SessionOp::CancelFreeMove => match self.display.cancel_free_move() {
                Ok(()) => OpOutcome::default(),
                Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
            },
            SessionOp::AddMate {
                placement,
                mate,
                values,
                into_target_space,
            } => {
                let (opts, tol) = (self.eval_options(), self.tol);
                self.commit_run(|run| {
                    let space = run.doc().operand_space(&mate.to);
                    if let Some(space) = space.filter(|&space| {
                        into_target_space && Some(space) != run.doc().node_space(placement)
                    }) {
                        into_space(run, placement, space)?;
                    }
                    // Where the copy stands before its mate, and the
                    // evaluation the snap reads its mate's poses off.
                    let ev: Evaluation<f64> = pncad::document::evaluate(
                        run.doc(),
                        None,
                        &pncad::document::CancelToken::new(),
                        &opts,
                        tol,
                    );
                    let before =
                        pncad::document::SolvedPoses::of(run.doc(), &ev).relative(placement);
                    // The door writes the bundle whole with the mate in it;
                    // the gesture then writes the orientation and the values
                    // that stand the copy where the snap puts it, so it moves
                    // only by the snap, and the values the person named over
                    // them.
                    run.apply(DocEdit::AddConstraint {
                        placement,
                        mate,
                        fresh: Vec::new(),
                    })?;
                    let snapped = snap::snapped(run.doc(), placement, before, &ev, tol);
                    if snapped.is_none() && values.is_empty() {
                        return Ok(());
                    }
                    let held = snapped.unwrap_or_else(|| held_bundle(run.doc(), placement));
                    write_bundle(run, placement, valued(held, values))
                })
            }
            SessionOp::RemoveConstraint { mate } => self.remove_constraint(mate),
            SessionOp::IntoOwnSpace { placement } => self.commit_run(|run| {
                let node = run.insert(Node::Space)?;
                let Some(&space) = run.doc().outputs(node).first() else {
                    unreachable!("a `new_space()` defines its one output")
                };
                // The copy stands at the identity of its new space,
                // whatever values placed it in the one it leaves.
                write_bundle(run, placement, editor_core::stand().bundle())?;
                into_space(run, placement, space)
            }),
            SessionOp::NewDocument { name } => self.new_document(&name),
            SessionOp::EditProfile {
                node,
                base,
                loops,
                ids,
            } => self.edit_profile(node, &base, loops, ids),
            op @ (SessionOp::AddProfile { .. }
            | SessionOp::SketchOnFace { .. }
            | SessionOp::AddDrawnProfile { .. }
            | SessionOp::AddExtrude { .. }
            | SessionOp::AddRevolve { .. }
            | SessionOp::AddBoolean { .. }
            | SessionOp::AddSplit { .. }
            | SessionOp::AddMirror { .. }
            | SessionOp::AddTransform { .. }
            | SessionOp::AddPattern { .. }
            | SessionOp::AddFusedPattern { .. }
            | SessionOp::AddFillet { .. }
            | SessionOp::AddChamfer { .. }
            | SessionOp::AddPart { .. }
            | SessionOp::Duplicate { .. }
            | SessionOp::AddInstance { .. }) => {
                let inserts = matches!(op, SessionOp::AddInstance { .. });
                let door = self.creation_door(op);
                let outcome = self.create(labelled.take(), door);
                // An inserted part's space is drawn clear of what is
                // drawn: the frame belongs to this history step, and is
                // measured once its picture lands.
                if let Some(node) = outcome.made.filter(|_| inserts)
                    && let Some(space) = self.committed_doc().node_space(node)
                {
                    self.inserts.insert(
                        self.history.current(),
                        Insert {
                            space,
                            node,
                            frame: None,
                        },
                    );
                }
                outcome
            }
            SessionOp::AcceptPartVersion { id } => self.accept_part_version(id),
            SessionOp::SetLabel { node, label } => {
                self.commit_written(DocEdit::SetLabel { node, label })
            }
            SessionOp::SetProduct { bodies } => self.set_product(bodies),
            SessionOp::AddToProduct { node, body } => self.add_to_product(node, body),
            SessionOp::CreateLabelled { creation, label } => {
                self.perform_op(creation.into_op(), Some(label))
            }
        };
        assert!(
            labelled.is_none(),
            "a labelled op's arm handed its label to no creation's run"
        );
        outcome
    }

    /// **A creating operation's door**: the run it would stage, or its
    /// refusal, decided off the session and writing nothing. The one
    /// dispatch both [`Self::perform_op`] and [`Self::stage`] reach a
    /// creation's door through, so what a stage evaluates is what the
    /// commit would write.
    fn creation_door(&self, op: SessionOp) -> Result<Creating, Refusal> {
        match op {
            SessionOp::AddProfile { loops } => self.add_profile(loops),
            SessionOp::SketchOnFace {
                face,
                loops,
                distance,
                spin,
            } => self.sketch_on_face(&face, loops, distance, spin),
            SessionOp::AddDrawnProfile {
                plane,
                traced,
                chains,
            } => self.add_drawn_profile(&plane, &traced, chains),
            SessionOp::AddExtrude { profile, distance } => self.add_extrude(profile, distance),
            SessionOp::AddRevolve {
                profile,
                axis_through,
                axis_heading,
                angle,
            } => self.add_revolve(profile, axis_through, axis_heading, angle),
            SessionOp::AddBoolean { spec } => self.add_boolean(spec),
            SessionOp::AddSplit {
                target,
                origin,
                normal,
            } => self.add_split(target, &origin, &normal),
            SessionOp::AddMirror { body, plane } => self.add_mirror(body, &plane),
            SessionOp::AddTransform {
                input,
                translation,
                rotation_axis,
                rotation_angle,
            } => self.add_transform(input, &translation, &rotation_axis, &rotation_angle),
            SessionOp::AddPattern { input, count, rule } => {
                self.add_pattern(input, count, rule, PatternOutputChoice::Instances)
            }
            SessionOp::AddFusedPattern { input, count, rule } => {
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
            SessionOp::Duplicate { input, beside } => self.add_duplicate(input, beside),
            SessionOp::AddInstance { id } => self.add_instance(id),
            other => unreachable!("only a creating operation reaches its door: {other:?}"),
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
    /// The author's shows ride too ([`DisplayState::shown`]): a show
    /// draws a body the default leaves out, so the picture the index
    /// describes is the landing's drawing them.
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
            self.display.shown(),
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
    /// [`DocSession::landed_body`] — so this costs this thread nothing.
    ///
    /// `None` before anything lands, and for a landing whose gather
    /// REFUSED ([`DocSession::product_fault`]): the budget prices the
    /// gathered product, and there is none. The index still draws the
    /// copies that have a value, unbudgeted
    /// (`work/vgeom/a-document-with-one-failed-copy-opens-with-no-display-budget.md`).
    pub fn fit_request(
        &self,
        requested: crate::scene::DisplayTolerance,
    ) -> Option<crate::evalseam::FitRequest> {
        let run = self.derived.landed.as_ref()?;
        let subject = match &run.gather {
            Err(_) => return None,
            Ok(Gathered { body, .. }) => crate::evalseam::FitSubject::Landed(Arc::clone(body)),
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
    fn add_instance(&self, id: DocumentId) -> Result<Creating, Refusal> {
        if let Some(refusal) = Refusal::self_instance(self.committed_doc().id(), id) {
            return Err(refusal);
        }
        let pin = self.read_store(|ws| ws.current_pin(id, self.tol))?;
        // A creation: a space of its own, and the instance into it by
        // the identity bundle.
        Ok(creating(move |run| {
            let space = run.insert(Node::Space)?;
            let Some(&space) = run.doc().outputs(space).first() else {
                unreachable!("a `new_space()` defines its one output")
            };
            run.insert(Node::instantiate_part(
                DocRef { id, pin },
                Operand::Var(space),
                pncad::document::stand().bundle(),
            ))
        }))
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
            .map_err(Refusal::from)
    }

    fn set_slot(&mut self, node: RecipeNodeId, slot: SlotId, value: SlotValue) -> OpOutcome {
        if let Err(refusal) = guard_driven(self.committed_doc(), node, slot, self.notation) {
            return OpOutcome::refused(refusal);
        }
        let unit = props::slot_unit(self.committed_doc(), node, slot);
        match props::slot_typed_edit(node, slot, value, unit) {
            Ok(edit) => {
                let outcome = self.commit_written(edit);
                self.offer_after(node, slot, &outcome);
                outcome
            }
            Err(error) => OpOutcome::refused(Refusal::Dimension(error)),
        }
    }

    /// **Open the offer a typed value earns** (D10), once the edit that
    /// typed it has landed: the slot now reads a variable that typing
    /// minted, and [`Self::offered`] answers the variables of equal
    /// value. Text that reads a variable or computes one mints no typed
    /// value and opens nothing; a refused or empty edit leaves the
    /// standing offer as it was.
    fn offer_after(&mut self, node: RecipeNodeId, slot: SlotId, outcome: &OpOutcome) {
        if outcome.refusal.is_some() || outcome.committed.is_empty() {
            return;
        }
        let doc = self.committed_doc();
        self.derived.offer = doc
            .slot(node, slot)
            .filter(|&var| doc.is_typed_value(var))
            .and_then(|var| {
                let typed = props::variable_value(doc, var)?;
                Some(SlotOffer {
                    node,
                    slot,
                    var,
                    typed,
                })
            });
    }

    /// **Accept an offer**: the slot reads `var` — the slot-write
    /// gesture, one undo step. An unnamed `var` is named `name` in the
    /// same step, since a variable two slots share has a name (VR2);
    /// accepted without one, the door refuses it
    /// (`EditError::SharedVarNeedsName`). Only a variable on offer
    /// there ([`Self::offered`]) is accepted, so a stale button can
    /// never join a slot to a variable chosen against a value it no
    /// longer holds; the door checks the kind.
    fn set_slot_variable(
        &mut self,
        node: RecipeNodeId,
        slot: SlotId,
        var: VarId,
        name: Option<VarName>,
    ) -> OpOutcome {
        if !self
            .offered(node, slot)
            .iter()
            .any(|offered| offered.var == var)
        {
            let spoken = self.committed_doc().spoken_var(var);
            return OpOutcome::refused(Refusal::NotOffered(spoken));
        }
        if name.is_some() && self.committed_doc().var_name(var).is_some() {
            let spoken = self.committed_doc().spoken_var(var);
            return OpOutcome::refused(Refusal::OfferIsNamed(spoken));
        }
        let read = props::slot_read_edit(node, slot, var);
        let outcome = match name {
            None => self.commit_written(read),
            Some(name) => self.commit_action(vec![
                DocEdit::RenameVar {
                    var: var.into(),
                    name: Some(name),
                },
                read,
            ]),
        };
        if outcome.refusal.is_none() {
            self.close_offer(node, slot);
        }
        outcome
    }

    /// **Decline an offer**: the slot keeps the variable its typed value
    /// minted, which is what makes it distinct from the ones offered
    /// (D10). It changes no document; declining where nothing is
    /// offered is the same nothing.
    fn close_offer(&mut self, node: RecipeNodeId, slot: SlotId) {
        if self
            .derived
            .offer
            .is_some_and(|offer| offer.node == node && offer.slot == slot)
        {
            self.derived.offer = None;
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
        // variables to probe instead. A variable has no driver and
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

    /// **What `SessionOp::SetBodyOn` would answer for `node`, or
    /// `None` where it would accept** — asked ahead of the click by the
    /// control that pushes the op.
    ///
    /// It is the whole of that op's admission: its door runs
    /// [`crate::display::body_check`] against the committed document,
    /// and neither drag's table in `perform` refuses it. So a row drawn
    /// from the LANDED document (the feature tree's) reads the refusal
    /// for a node deleted since the last landing.
    pub fn body_toggle_refusal(&self, node: RecipeNodeId) -> Option<Refusal> {
        crate::display::body_check(self.committed_doc(), node)
            .err()
            .map(|fault| Refusal::Display(fault.into()))
    }

    fn set_slot_expression(&mut self, node: RecipeNodeId, slot: SlotId, text: &str) -> OpOutcome {
        let expr = match parse_formula(text, &self.committed_doc().var_scope()) {
            Ok(expr) => expr,
            Err(error) => return OpOutcome::refused(Refusal::Parse(Box::new(error))),
        };
        // An expression edit is available on a driven slot AND on a
        // literal one — the refusal is about writing a number over a
        // computation, not about the slot being off limits.
        let edit = if slot.is_structural() {
            DocEdit::SetStructuralParam {
                node,
                slot,
                expr,
                fresh: Vec::new(),
            }
        } else {
            DocEdit::SetParam {
                node,
                slot,
                value: expr.into(),
                fresh: Vec::new(),
            }
        };
        // **Through the written door**, like every other door that
        // writes a panel field's value. The field's own guard cannot
        // answer for this one: a literal slot that evaluated shows its
        // NUMBER, so the render the echo compares against is the bare
        // `8.0` while the slot's source is `8 mm`, and re-typing the
        // source is no echo of anything. `Self::writes_nothing` asks the question
        // that is actually being asked here — whether the expression
        // offered is the expression standing — and the `unparse` /
        // `parse_formula` round trip preserves both the bits and the
        // display unit, so a source re-typed as itself compares equal.
        let outcome = self.commit_written(edit);
        self.offer_after(node, slot, &outcome);
        outcome
    }

    /// The value door: write a declared variable's value.
    ///
    /// A variable the document does not hold takes the commit path so
    /// the typed refusal comes from the door rather than from here —
    /// `DocEdit::SetVarValue` carries an existing definition forward
    /// and refuses `EditError::UnknownVar` when there is none.
    ///
    /// A count written to an index redefines its count
    /// ([`props::index_count_edit`]).
    fn set_variable(&mut self, var: VarId, value: SlotValue) -> OpOutcome {
        if let SlotValue::Count(count) = value
            && let Some(edit) =
                props::index_count_edit(self.committed_doc(), var, Formula::count(count))
        {
            return self.commit_written(edit);
        }
        self.commit_written(props::variable_edit(var, value))
    }

    /// The notation door: rewrite a declared variable's display unit,
    /// its value untouched.
    ///
    /// No pre-check, for [`Self::set_variable`]'s reason: every way this
    /// can refuse — a variable the document does not hold, a `Count`,
    /// a unit that does not measure the declared dimension — is refused by
    /// `DocEdit::SetVarUnit` in the door's own words, and a
    /// second opinion here could only agree or disagree.
    fn set_variable_unit(&mut self, var: VarId, unit: UnitDef) -> OpOutcome {
        self.commit_written(props::variable_unit_edit(var, unit))
    }

    /// The text door: a number, and the notation to write it in, from
    /// one piece of typed text — or an expression, which DEFINES the
    /// variable (`DocEdit::DefineVar`), keeping its identity.
    ///
    /// **One parser.** `parse_formula` reads `50 mm` into a literal that
    /// already carries the canonical value and remembers the unit —
    /// the one multiply is the parser's — so what arrives here is read
    /// off the literal and never scaled again.
    ///
    /// **One action, therefore one undo.** The value and the notation
    /// go through [`Self::commit_action`], which is all-or-nothing: a
    /// unit that does not measure the variable's dimension refuses
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
    /// typed over a variable already declared `50 mm` costs no undo
    /// step even though the text is not the field's render of it. It
    /// is asked over the PAIR, because this door carries a pair —
    /// `0.05 m` over `50 mm` moves the notation and not the value.
    ///
    /// The number door has no such check and needs none: it changes
    /// one fact, and a number that reads back as the one standing
    /// cannot have got past the field's guard as anything but a
    /// deliberate re-type.
    fn set_variable_text(&mut self, var: VarId, text: &str) -> OpOutcome {
        let expr = match parse_formula(text, &self.committed_doc().var_scope()) {
            Ok(expr) => expr,
            Err(error) => return OpOutcome::refused(Refusal::Parse(Box::new(error))),
        };
        // Text typed over an index is its count.
        if let Some(edit) = props::index_count_edit(self.committed_doc(), var, expr.clone()) {
            return self.commit(edit);
        }
        // Text that reads a variable defines this one by it.
        if reads_a_variable(&expr) {
            return self.commit(DocEdit::DefineVar {
                var: var.into(),
                def: pncad::document::VarDecl::defined(expr),
                fresh: Vec::new(),
            });
        }
        // Constant text is a value, folded as a written quantity is, so
        // `50 mm + 1 mm` writes 51 mm and keeps the variable's identity
        // and distribution. Only a literal carries a notation to write.
        let env = pncad::document::VarEnv::<f64>::default();
        let folded = if expr.dim() == Dimension::Count {
            pncad::document::eval_count(&expr, &env).map(SlotValue::Count)
        } else {
            pncad::document::eval(&expr, &env).map(SlotValue::Continuous)
        };
        let value = match folded {
            Ok(value) => value,
            Err(source) => {
                return OpOutcome::refused(Refusal::ConstantRefused {
                    var: self.committed_doc().spoken_var(var),
                    source,
                });
            }
        };
        let doc = self.committed_doc();
        let Some(held) = doc.var(var) else {
            // An undeclared variable takes the commit path so the typed
            // refusal comes from the door rather than from here, the
            // way [`Self::set_variable`]'s does.
            return self.commit(props::variable_edit(var, value));
        };
        let kind = held.kind();
        let defined = held.def().defined().map(pncad::document::Expr::dim);
        let notation = match (value, expr.display_unit()) {
            (SlotValue::Continuous(_), Some(unit)) => Some(props::variable_unit_edit(var, unit)),
            _ => None,
        };
        let written = props::variable_edit(var, value);
        // A value typed over a definition frees the variable first, at
        // its own kind and in its kind's canonical notation, and then
        // writes the value through the same doors a free variable's
        // field does — one action, so a value of another kind refuses
        // in the value door's words and leaves the definition standing.
        if let Some(dim) = defined {
            let freed = match dim {
                Dimension::Count => FreeVar::Count { value: 0 },
                dim => FreeVar::continuous(dim, 0.0),
            };
            let free_again = DocEdit::DefineVar {
                var: var.into(),
                def: pncad::document::VarDecl::Free(freed),
                fresh: Vec::new(),
            };
            return self.commit_action(
                std::iter::once(free_again)
                    .chain(notation)
                    .chain([written])
                    .collect(),
            );
        }
        // **A `Count` gets the VALUE edit and only that.** A count names
        // no notation at all, so the notation half of this door has
        // nothing to say about one, and submitting it anyway answers
        // what the user did — typing a value — in the words of a change
        // nobody asked for ("it has no display unit to change"). The
        // value edit is the act, and refusing it names the right half:
        // a count declared where a continuous value was typed.
        if kind == pncad::document::VarKind::Count {
            return self.commit(written);
        }
        let edits: Vec<DocEdit<ProfileProgram>> = notation
            .into_iter()
            .chain([written])
            .filter(|edit| !self.writes_nothing(edit))
            .collect();
        if edits.is_empty() {
            return OpOutcome::default();
        }
        self.commit_action(edits)
    }

    /// The create door: one `DeclareVar`. A taken name is the
    /// declare's to refuse (`EditError::VarNameTaken`), and reaches the
    /// caller through [`Refusal::Edit`] in the door's own words.
    fn declare_var(&mut self, name: VarName, value: FreeVar) -> OpOutcome {
        self.commit(DocEdit::DeclareVar {
            name,
            def: pncad::document::VarDecl::Free(value),
        })
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

    /// The variable door: a drag over a declared variable's value. A
    /// defined one opens too, at its kind, and its first preview is
    /// refused by the value door in its own words
    /// ([`pncad::document::EditError::NotAFreeVar`]).
    fn begin_variable_gesture(&mut self, var: VarId) -> OpOutcome {
        self.start(move |doc| {
            let held = doc.var(var).ok_or(Refusal::NoSuchVariable(var))?;
            // An operation's output is no parameter: the value door
            // refuses it in its words.
            if let Some(refusal) =
                EditError::output_refusal(doc, var, pncad::document::CarryForwardDoor::Value)
            {
                return Err(Refusal::Edit(Box::new(refusal)));
            }
            let Some(dimension) = held.kind().dimension() else {
                unreachable!("a free or defined variable is a scalar")
            };
            Ok(GestureTarget::Variable { var, dimension })
        })
    }

    /// Open a gesture, refusing one that is already open and
    /// validating `target` only once the slot is known free.
    ///
    /// **Rule 1 is [`g1::Slot::begin`]'s and is spelled nowhere else**
    /// — the same door the probe's begin goes through, with this
    /// gesture's words. `BeginGesture` and `BeginVariableGesture` are
    /// therefore `true` in
    /// [`SessionOp::permitted_during_value_gesture`]: a row there
    /// would refuse the same state with the same
    /// [`Refusal::GestureInFlight`] one layer up, and the door's own
    /// arm would never run.
    ///
    /// `target` is the caller's check — a driven slot, a variable the
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
        let tol = self.tol;
        let previewed = self.gesture.preview(
            gesture_words(),
            |gesture| gesture.target.name() == *named,
            |gesture| {
                let slot_value = gesture.target.value_of(value).map_err(Refusal::Dimension)?;
                let edit = gesture
                    .target
                    .edit(&gesture.base, slot_value)
                    .map_err(Refusal::Dimension)?;
                // Applied to the gesture's BASE, so previews replace
                // one another instead of composing, and the history
                // never sees any of them.
                let applied = apply(&gesture.base, &edit, tol)
                    .map_err(|error| Refusal::Edit(Box::new(error)))?;
                // **The display layer's identity, held rather than
                // argued.** Every display predicate is a function of
                // the node graph, and the free-move probe is admitted
                // against the COMMITTED document while the view and
                // the panel — which run ONE admission test between
                // them, `display::free_move_check` — resolve against
                // this scratch, so the two agree only while a gesture's
                // edits leave the graph alone. This holds the half a
                // check can hold; the other half is that
                // [`GestureTarget::edit`] can produce nothing but
                // `SetParam`, `SetStructuralParam` and
                // `SetVarValue`, none of which removes a node
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
        match gesture.target.edit(self.committed_doc(), value) {
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
        let left = self.history.current();
        let moved = match direction {
            Step::Undo => self.history.undo(),
            Step::Redo => self.history.redo(),
        };
        if moved.is_none() {
            return OpOutcome::refused(Refusal::NothingToDo { direction });
        }
        // An insert's frame is its history step's: the undo of the step
        // takes it back, and its redo writes it again.
        let frame_of = |inserts: &std::collections::BTreeMap<HistoryId, Insert>, state| {
            inserts
                .get(&state)
                .and_then(|insert: &Insert| {
                    insert.frame.map(|frame| (insert.space, insert.node, frame))
                })
                .filter(|(_, _, frame)| !frame.is_identity_bits())
        };
        match direction {
            Step::Undo => {
                if let Some((space, _, frame)) = frame_of(&self.inserts, left) {
                    self.display.take_space_frame(space, frame);
                }
            }
            Step::Redo => {
                if let Some((space, node, frame)) = frame_of(&self.inserts, self.history.current())
                {
                    self.display.place_space(space, node, frame);
                }
            }
        }
        // The document moved, so the display state's derived facts
        // (which instances exist; which spaces are shown) may have
        // too — an undo past a re-target does NOT resurrect a
        // discarded frame (the value is gone, not parked), and a redo
        // over one discards the frame of the space it empties.
        let pruned = self.display.prune(self.history.doc(), None);
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
                self.resolver = Some(Arc::new(DirResolver::new(docio::file_dir(path))));
                self.history = history;
                self.path = Some(path.to_path_buf());
                self.clear_for_new_document();
                self.request_eval();
                OpOutcome::default()
            }
            Err(error) => OpOutcome::refused(Refusal::Io(Box::new(error))),
        }
    }

    /// Write the committed document at `path` through that folder's
    /// store ([`docio::save`]), identity kept.
    fn save(&mut self, path: &Path) -> OpOutcome {
        match docio::save(path, self.history.doc(), self.tol) {
            Ok(_) => {
                self.bind_file(path);
                OpOutcome::default()
            }
            Err(error) => OpOutcome::refused(error.into()),
        }
    }

    /// The fork (DI4): the committed document under a fresh random
    /// id, written at `path` through that folder's store
    /// ([`docio::save_as_new_document`]), and then the whole history
    /// re-identified under the new id. The write comes first, so a
    /// refused one leaves the session as it was.
    fn save_as_new_document(&mut self, path: &Path) -> OpOutcome {
        match docio::save_as_new_document(path, self.history.doc(), self.tol) {
            Ok(id) => {
                self.history.under_identity(id);
                // The landed run is of the old identity, so a run is
                // owed whether or not the folder moved.
                if !self.bind_file(path) {
                    self.request_eval();
                }
                OpOutcome::default()
            }
            Err(error) => OpOutcome::refused(error.into()),
        }
    }

    /// **The session is now backed by `path`.** The directory rule
    /// follows the file: a save into another folder rebinds the
    /// resolver to it and re-evaluates, since references may resolve
    /// differently there. Answers whether it did.
    fn bind_file(&mut self, path: &Path) -> bool {
        self.path = Some(path.to_path_buf());
        let dir = docio::file_dir(path);
        let rebinds = self.resolver.as_deref().map(DirResolver::dir) != Some(dir.as_path());
        if rebinds {
            self.resolver = Some(Arc::new(DirResolver::new(dir)));
            self.request_eval();
        }
        rebinds
    }

    /// Replace the session with a fresh empty document under a fresh
    /// random id — the [`SessionOp::NewDocument`] semantics (that arm's
    /// docs say what is cleared). Both refusals, a blank name and
    /// unavailable randomness, come before anything is replaced.
    ///
    /// The same two doors as [`DocSession::open`] — a new history and
    /// [`DocSession::clear_for_new_document`] — with the file-shaped
    /// fields going the other way: no path and no resolver, because
    /// nothing backs this document until it is saved.
    fn new_document(&mut self, name: &str) -> OpOutcome {
        if let Err(refusal) = Refusal::new_document_name(name) {
            return OpOutcome::refused(refusal);
        }
        // The `Err` arm is UNTESTED, and labelled as such: `getrandom`
        // has no injection seam.
        let id = match pncad::workspace::random_document_id() {
            Ok(id) => id,
            Err(error) => return OpOutcome::refused(error.into()),
        };
        self.history = History::new(Doc::empty(id, self.tol));
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
        self.stager.cancel();
        self.display.clear();
        // A landing's report names the outgoing document's nodes, which
        // the incoming one cannot speak (`DisplayState::clear`).
        self.landed_withdrawals.clear();
        // History ids are arena positions of the outgoing history.
        self.inserts.clear();
    }

    /// Insert one profile node ([`SessionOp::AddProfile`]), drawn on its
    /// own plane.
    ///
    /// No loop-count or nesting judgment here: an empty list, a
    /// degenerate loop and a non-nested pair all go to `commit`, where
    /// the edit door's authoring-time check refuses them typed in the
    /// profile layer's own words — the one rule authored and
    /// hand-written programs share.
    fn add_profile(&self, loops: Vec<LoopProgram<Formula>>) -> Result<Creating, Refusal> {
        Ok(creating(move |run| {
            run.insert(Node::Profile(ProfileProgram {
                loops,
                ids: Vec::new(),
                on: None,
            }))
        }))
    }

    /// **"Sketch on this face"** ([`SessionOp::SketchOnFace`]): the
    /// kernel's one gesture (`Recording::sketch_on_face`) over the body
    /// the face is on, and the union taking over what read that body
    /// ([`Takeover`]), in one action.
    fn sketch_on_face(
        &self,
        face: &FaceSelection,
        loops: Vec<LoopProgram<Formula>>,
        distance: Formula,
        spin: Formula,
    ) -> Result<Creating, Refusal> {
        let doc = self.committed_doc();
        let (body, name) = picked_body(doc, face);
        self.require_read(&body, NodeKindWanted::Body)?;
        let opts = self.eval_options();
        let takeover = Takeover::of(doc, &body);
        takeover.stranded_pick(doc)?;
        Ok(creating(move |run| {
            let sketched = run.sketch_on_face(
                body,
                name,
                ProfileProgram {
                    loops,
                    ids: Vec::new(),
                    on: None,
                },
                distance,
                spin,
                None,
                &opts,
            )?;
            takeover.repoint(run, sketched.union, None)?;
            Ok(sketched.union)
        }))
    }

    /// **A profile drawn on the frame of `plane` and the first traced
    /// face, tracing `traced`** ([`SessionOp::AddDrawnProfile`]): the
    /// frame read against an evaluation of the committed document
    /// ([`drawn_frame`]), the program built by the façade's door
    /// ([`pncad::trace::drawn_profile`], which checks each argument's
    /// dimension), and one `InsertNode`.
    fn add_drawn_profile(
        &self,
        plane: &FaceSelection,
        traced: &[FaceSelection],
        chains: Vec<pncad::trace::Chain>,
    ) -> Result<Creating, Refusal> {
        let doc = self.committed_doc();
        let Some(first) = traced.first() else {
            return Err(Refusal::Frame(Box::new(GestureFrameFault::Traced(
                Arc::new(pncad::trace::TracedFrameFault::NotAFace),
            ))));
        };
        let ev: Evaluation<f64> = pncad::document::evaluate(
            doc,
            None,
            &pncad::document::CancelToken::new(),
            &self.eval_options(),
            self.tol,
        );
        let frame = match drawn_frame(doc, &ev, plane, first, self.tol) {
            Ok(frame) => frame,
            Err(fault) => {
                return Err(Refusal::Frame(Box::new(GestureFrameFault::Traced(
                    Arc::new(fault),
                ))));
            }
        };
        let faces = traced.iter().map(|face| pick_read(doc, face)).collect();
        match pncad::trace::drawn_profile(frame, faces, chains) {
            Ok(node) => Ok(creating(move |run| run.insert(node))),
            Err(error) => Err(Refusal::Edit(Box::new(error))),
        }
    }

    /// **`face`'s trace in the plane of `plane`, for display**
    /// ([`pncad::trace::sketch_trace`]), against the landed evaluation:
    /// on the frame a profile drawn there tracing `face` stands on
    /// ([`drawn_frame`]), read as that profile would read it, and
    /// sampled across a square the traced body spans. Nothing is
    /// written.
    ///
    /// # Errors
    ///
    /// [`SketchTraceFault`].
    pub fn sketch_trace(
        &self,
        plane: &FaceSelection,
        face: &FaceSelection,
    ) -> Result<pncad::trace::TraceMarks, SketchTraceFault> {
        let (doc, ev) = self.landed_pair().ok_or(SketchTraceFault::Unevaluated)?;
        let frame = drawn_frame(doc, ev, plane, face, self.tol)
            .map_err(|fault| SketchTraceFault::Frame(Arc::new(fault)))?;
        let half = TRACE_WINDOW_MARGIN * body_reach(ev, face.node).unwrap_or(1.0);
        pncad::trace::sketch_trace(
            doc,
            ev,
            frame,
            pick_read(doc, face),
            (pncad::geom_core::Point2::origin(), half),
            SKETCH_TRACE_RAYS,
            self.tol,
        )
        .map_err(|fault| SketchTraceFault::Trace(Arc::new(fault)))
    }

    /// Write the path editor's program over a committed profile's
    /// ([`SessionOp::EditProfile`], whose doc carries the rules).
    fn edit_profile(
        &mut self,
        node: RecipeNodeId,
        base: &ProfileProgram<Formula>,
        loops: Vec<LoopProgram<Formula>>,
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
    /// [`Recording`] as the commit nets it: every name on a step
    /// the reshaping drops, or on a kept step's piece it stops drawing,
    /// stranded. Empty when the op would write nothing.
    ///
    /// The profile editor reads it BEFORE its Apply, while the person
    /// can still keep the step; the op's outcome carries the same rows
    /// after, beside the anonymous variables the rewrite retires, which
    /// this count leaves out unless one carried a tolerance
    /// (`Maintenance::is_silent_retirement`).
    ///
    /// # Errors
    ///
    /// The refusal the op itself would give.
    pub fn edit_profile_report(
        &self,
        node: RecipeNodeId,
        base: &ProfileProgram<Formula>,
        loops: Vec<LoopProgram<Formula>>,
        ids: Vec<Vec<Option<StepId>>>,
    ) -> Result<Vec<Maintenance>, Refusal> {
        let Some(edit) = self.set_program_of(node, base, loops, ids)? else {
            return Ok(Vec::new());
        };
        let mut run = Recording::start(self.committed_doc(), self.tol);
        let refused = |error| Refusal::Edit(Box::new(error));
        run.apply(edit).map_err(refused)?;
        // What the count says is what the person would lose: a
        // reshaping retires the variables the arguments it rewrote were
        // written in, which is no name and no paint of theirs — unless
        // one carried a tolerance (`Maintenance::is_silent_retirement`).
        Ok(run
            .finish()
            .map_err(refused)?
            .maintenance
            .into_iter()
            .filter(|row| !row.is_silent_retirement())
            .collect())
    }

    /// **The one `SetProgram` [`SessionOp::EditProfile`] commits**, or
    /// `None` when it would write nothing.
    fn set_program_of(
        &self,
        node: RecipeNodeId,
        base: &ProfileProgram<Formula>,
        loops: Vec<LoopProgram<Formula>>,
        ids: Vec<Vec<Option<StepId>>>,
    ) -> Result<Option<DocEdit<ProfileProgram>>, Refusal> {
        self.require_kind(node, NodeKindWanted::Profile)?;
        let doc = self.committed_doc();
        let Some(Node::Profile(current)) = doc.node(node) else {
            unreachable!("`require_kind` admitted node {} as a profile", node)
        };
        // The editor's program is an edit OF the program it loaded;
        // over any other program it would be a guess about what the
        // person meant. Compared as written and by value, so a value
        // moved since the load refuses and a unit rewrite does not.
        if sketch::written_program(doc, current) != *base {
            return Err(Refusal::ProfileEditStale {
                node: doc.spoken(node),
            });
        }
        let loops = carry_unmoved(doc, node, current, loops, &ids, self.notation)?;
        // The carried loops read each unmoved argument's variable, so
        // they are compared with the program re-authored.
        let authored = ProfileProgram {
            loops: current.loops.iter().map(LoopProgram::authored).collect(),
            ids: current.ids.clone(),
            on: None,
        };
        let unchanged = sketch::is_committed(&authored, &loops, &ids);
        Ok((!unchanged).then_some(DocEdit::SetProgram {
            node,
            loops,
            ids,
            fresh: Vec::new(),
        }))
    }

    /// Insert one extrude of an existing profile
    /// ([`SessionOp::AddExtrude`]).
    fn add_extrude(&self, profile: RecipeNodeId, distance: Formula) -> Result<Creating, Refusal> {
        self.require_kind(profile, NodeKindWanted::Profile)?;
        let node = Node::Extrude {
            profile: profile.into(),
            distance,
        };
        Ok(creating(move |run| run.insert(node)))
    }

    /// Insert one revolve of an existing profile about an axis line in
    /// its own plane ([`SessionOp::AddRevolve`]).
    fn add_revolve(
        &self,
        profile: RecipeNodeId,
        axis_through: [Formula; 2],
        axis_heading: Formula,
        angle: Formula,
    ) -> Result<Creating, Refusal> {
        self.require_kind(profile, NodeKindWanted::Profile)?;
        let node = Node::Revolve {
            profile: profile.into(),
            axis_through,
            axis_heading,
            angle,
        };
        Ok(creating(move |run| run.insert(node)))
    }

    /// Insert one boolean of existing bodies ([`SessionOp::AddBoolean`]).
    fn add_boolean(&self, spec: BooleanSpec) -> Result<Creating, Refusal> {
        let doc = self.committed_doc();
        let spec = spec.with_operands(
            spec.operands()
                .into_iter()
                .map(|read| as_body(doc, read))
                .collect(),
        );
        for seat in spec.operands() {
            self.require_read(&seat, NodeKindWanted::Body)?;
        }
        // The first operand something reads is taken over by the
        // combine ([`combine_in_world`]'s choice, asked of the same
        // document before the operands are placed).
        if let Some(takeover) = spec
            .operands()
            .iter()
            .map(|operand| Takeover::of(doc, operand))
            .find(|takeover| !takeover.is_empty())
        {
            takeover.stranded_pick(doc)?;
        }
        // A node spelled twice is not refused: a repeated read glues
        // (A∪A = A∩A = A, A−A the typed empty body), so it is a
        // document like any other.
        Ok(creating(move |run| {
            let operands = in_one_space(run, &spec.operands())?;
            let node = run.insert(spec.with_operands(operands).node())?;
            combine_in_world(run, &spec.operands(), node)?;
            Ok(node)
        }))
    }

    /// Insert one split of an existing body ([`SessionOp::AddSplit`]):
    /// its tool the plane through `origin` with normal `normal` in the
    /// target's own coordinates, read off the target's geometry
    /// ([`frame_off`] through its `xy`) at the numbers' nominal values.
    fn add_split(
        &self,
        target: Operand,
        origin: &[Formula; 3],
        normal: &[Formula; 3],
    ) -> Result<Creating, Refusal> {
        let target = as_body(self.committed_doc(), target);
        self.require_read(&target, NodeKindWanted::Body)?;
        let tool = self.read_off(&target, origin, normal, "split plane", PoseRole::Xy)?;
        Ok(creating(move |run| {
            run.insert(Node::Split {
                target: target.into(),
                tool,
            })
        }))
    }

    /// **Mirror a body across the plane of a picked face**
    /// ([`SessionOp::AddMirror`]): the image, and the symmetric part,
    /// the body and its image unioned, taking over what reads the body
    /// along a seen path ([`Takeover`]).
    fn add_mirror(&self, body: Operand, plane: &FaceSelection) -> Result<Creating, Refusal> {
        let body = as_body(self.committed_doc(), body);
        self.require_read(&body, NodeKindWanted::Body)?;
        let doc = self.committed_doc();
        let (on, face) = picked_body(doc, plane);
        let takeover = Takeover::of(doc, &body);
        takeover.stranded_pick(doc)?;
        let node = combine::mirror_node(body.clone(), on, face);
        Ok(creating(move |run| {
            let image = run.insert(node)?;
            let part = run.insert(BooleanSpec::Union(vec![body, image.into()]).node())?;
            takeover.repoint(run, part, None)?;
            Ok(part)
        }))
    }

    /// **A pose read off the geometry of `body`'s space**: the frame
    /// standing at `origin` with `z` along `direction` (in the
    /// coordinates of the space `body` lives in, at the numbers' nominal
    /// values in the committed document), read through `role` — a
    /// split's plane through `xy`, a circular pattern's axis through
    /// `z`. A placement into a construction's space is read through
    /// that construction ([`crate::world::construction_of_space`]); a
    /// copy that stands in its space is read where it stands.
    fn read_off(
        &self,
        body: &Operand,
        origin: &[Formula; 3],
        direction: &[Formula; 3],
        what: &'static str,
        role: PoseRole,
    ) -> Result<RoleRead<Operand>, Refusal> {
        let doc = self.committed_doc();
        let origin = nominals(doc, what, origin, Dimension::Length)?;
        let direction = nominals(doc, what, direction, Dimension::Scalar)?;
        let at = frame_along(origin, direction, what, self.tol)?;
        let construction = crate::world::read_output(doc, body)
            .filter(|&(node, _)| matches!(doc.node(node), Some(Node::Place { .. })))
            .and_then(|(node, _)| {
                crate::world::construction_of_space(doc, node).filter(|&r| r != node)
            })
            .map(|construction| body_of(doc, construction));
        let body = construction.as_ref().unwrap_or(body);
        // `frame_off` takes a placed copy's frame in the copy's own
        // coordinates and carries it by the copy's motion; the numbers
        // are in its space's, so they are carried back first.
        let opts = self.eval_options();
        let at = match crate::world::read_output(doc, body).and_then(|(node, _)| {
            pncad::document::solve_document(doc, &opts, self.tol).relative(node)
        }) {
            Some(pose) => Frame::from_affine(pose.affine().inverse() * at.affine()),
            None => at,
        };
        let ev: Evaluation<f64> = pncad::document::evaluate(
            doc,
            None,
            &pncad::document::CancelToken::new(),
            &opts,
            self.tol,
        );
        let frame = frame_off(doc, &ev, body, &at, self.tol)
            .map_err(|refusal| Refusal::Frame(Box::new(GestureFrameFault::Geometry(refusal))))?;
        Ok(RoleRead::through(frame, role))
    }

    /// Insert one rigid move of an existing body
    /// ([`SessionOp::AddTransform`]): a placement of `input` into the
    /// space it is in, by values on that space's axes ([`gesture_moves`])
    /// — rotated by `rotation_angle` about `rotation_axis` through
    /// `input`'s own origin, then translated — and the body's world
    /// placements re-pointed to it (A10, Ev's residue 1), in one action.
    fn add_transform(
        &self,
        input: Operand,
        translation: &[Formula; 3],
        rotation_axis: &[Formula; 3],
        rotation_angle: &Formula,
    ) -> Result<Creating, Refusal> {
        let input = as_body(self.committed_doc(), input);
        self.require_placeable(&input)?;
        let doc = self.committed_doc();
        let at = gesture_frame(doc, translation, rotation_axis, rotation_angle, self.tol)?;
        let moves = gesture_moves(doc, translation, rotation_axis, rotation_angle, &at)?;
        let takeover = Takeover::of(doc, &input);
        // Into the space the body is in, on that space's axes.
        let target = self.home_space(&input)?;
        Ok(creating(move |run| {
            let placement = run.insert(Node::place(
                pncad::document::BodyRead::plain(input.clone()),
                Operand::Var(target),
                pncad::document::Placed::unmated(moves).bundle(),
            ))?;
            takeover.repoint(run, placement, None)?;
            Ok(placement)
        }))
    }

    /// Insert one pattern of an existing body, fused or not
    /// ([`SessionOp::AddPattern`], [`SessionOp::AddFusedPattern`]): the
    /// index `k = index(count)` and one placement of the body in its own
    /// space whose values read it (D10 `[repetition]`,
    /// [`pncad::pattern`]) — a row slid `scalar(k)·spacing` along the
    /// rule's direction, or a ring spun `scalar(k)·step` about its axis
    /// — and, fused, a union reading the family whole. The family, or
    /// the union, takes over what read the body along a seen path
    /// ([`Takeover`]), so member 0 stands where the body stood.
    ///
    /// **One function for the two ops**, for `add_blend`'s reason: the
    /// prototype seat, the axis seat a circular rule adds and the
    /// commit are the same move for both, and the only difference —
    /// whether a union fuses the family — is one match below.
    fn add_pattern(
        &self,
        input: Operand,
        count: i64,
        rule: PatternRuleSpec,
        output: PatternOutputChoice,
    ) -> Result<Creating, Refusal> {
        let input = as_body(self.committed_doc(), input);
        self.require_placeable(&input)?;
        let doc = self.committed_doc();
        let index = combine::fresh_index(doc);
        let target = self.home_space(&input)?;
        let source = pncad::document::BodyRead::plain(input.clone());
        let placement = match rule {
            PatternRuleSpec::Linear { direction, spacing } => {
                combine::row_steps(&direction, &spacing).and_then(|steps| {
                    pncad::pattern::linear_pattern(&index, source, Operand::Var(target), &steps)
                })
            }
            PatternRuleSpec::Circular {
                through,
                direction,
                step,
            } => {
                let axis = axis_operand(self.read_off(
                    &input,
                    &through,
                    &direction,
                    "pattern axis",
                    PoseRole::Z,
                )?);
                let along = combine::ring_slide_axis(&direction);
                pncad::pattern::circular_pattern(
                    doc,
                    &index,
                    source,
                    Operand::Var(target),
                    (axis.clone(), axis, along),
                    pncad::pattern::RingStep::By(step),
                )
            }
        };
        let placement = placement.map_err(Refusal::Dimension)?;
        let declare = move |run: &mut Recording<'_, ProfileProgram>| {
            run.declare(
                index,
                pncad::document::VarDecl::Index {
                    count: Formula::count(count),
                    within: None,
                },
            )
            .map(drop)
        };
        let doc = self.committed_doc();
        let takeover = Takeover::of(doc, &input);
        takeover.stranded_pick(doc)?;
        Ok(creating(move |run| {
            declare(run)?;
            let row = run.insert(placement)?;
            match output {
                // Several bodies, the family itself: a reader of one
                // body reads them through a `⊔`.
                PatternOutputChoice::Instances => {
                    let whole = if takeover.reads_one_body() {
                        Some(run.insert(Node::Assemble {
                            members: pncad::document::Bodies::Family(row.into()),
                        })?)
                    } else {
                        None
                    };
                    takeover.repoint(run, row, whole)?;
                    Ok(row)
                }
                PatternOutputChoice::Fused => {
                    let fused = run.insert(Node::Union {
                        members: pncad::document::Bodies::Family(row.into()),
                    })?;
                    takeover.repoint(run, fused, None)?;
                    Ok(fused)
                }
            }
        }))
    }

    /// Insert one read of a family's member ([`SessionOp::AddPart`]).
    fn add_part(&self, of: RecipeNodeId, select: PartSelectSpec) -> Result<Creating, Refusal> {
        self.require_kind(of, NodeKindWanted::Instances)?;
        let node = combine::part_node(self.committed_doc(), of, select).ok_or_else(|| {
            Refusal::NotABodyRead {
                read: Box::new(of.into()),
            }
        })?;
        Ok(creating(move |run| run.insert(node)))
    }

    /// Duplicate one body ([`SessionOp::Duplicate`]): of a copy, one
    /// more copy of its body in that copy's space, by values on every
    /// freedom at the copy's evaluated motion stepped clear along the
    /// original's own `x`, and no mate; of a body nothing copies, two
    /// copies in its own space, a stand-in by the identity and the
    /// stepped one.
    ///
    /// **One action and therefore one undo.** The original's placements
    /// are untouched.
    fn add_duplicate(
        &self,
        input: Operand,
        beside: Option<RecipeNodeId>,
    ) -> Result<Creating, Refusal> {
        let input = as_body(self.committed_doc(), input);
        self.require_read(&input, NodeKindWanted::Body)?;
        // The kind gate above is the document's; this is the VALUE's —
        // one body, with a width to clear — and the value must be the
        // CURRENT document's. `busy` is the authority on that: it is
        // the session's own comparison of the landed generation against
        // the committed one, so an edit or a document replacement that
        // has not landed yet is refused here rather than measured off
        // the value it replaced.
        let landed = match self.landed_pair() {
            None => Err(DuplicateFault::NotLanded),
            Some(_) if self.busy() => Err(DuplicateFault::Stale),
            Some((doc, eval)) => combine::duplicate_step(doc, eval, &input, self.tol)
                .and_then(|step| Ok((step, duplicate_seats(doc, eval, &input, beside)?))),
        };
        let (step, seats) = match landed {
            Ok(landed) => landed,
            Err(fault) => return Err(Refusal::Duplicate(fault)),
        };
        let stepped = Frame::translation(combine::STEP_DIRECTION.map(|d| d * step));
        let own = self.home_space(&input)?;
        let copy = move |target: VarId, bundle| {
            Node::place(
                pncad::document::BodyRead::plain(input.clone()),
                Operand::Var(target),
                bundle,
            )
        };
        let unmated = |at: &Frame| pncad::document::Placed::unmated(moves_of(at)).bundle();
        // The copy made is the stepped one: an unplaced body's second.
        let made = usize::from(seats.is_empty());
        let mut copies = Vec::new();
        if seats.is_empty() {
            copies.push(copy(own, editor_core::stand().bundle()));
            copies.push(copy(own, unmated(&stepped)));
        }
        for (target, at) in seats {
            let at = Frame::from_affine(at.affine::<f64>() * stepped.affine::<f64>());
            copies.push(copy(target, unmated(&at)));
        }
        Ok(creating(move |run| {
            let mut minted = Vec::new();
            for copy in copies {
                minted.push(run.insert(copy)?);
            }
            Ok(minted[made])
        }))
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
        &self,
        target: RecipeNodeId,
        size: Formula,
        selection: Vec<StableName>,
        kind: BlendKindChoice,
    ) -> Result<Creating, Refusal> {
        // The CANONICALIZING constructors, never the struct literals:
        // canonical form is what makes two recipes over the same edges
        // bit-identical, and `persist`'s strict door treats a
        // non-canonical set on the wire as a corrupt file.
        // A pick on a copy is read on its body (A10): the body the
        // placement places, and the body's own name for each edge.
        let doc = self.committed_doc();
        // A copy carries its body's names, so the edges' names are the
        // body's already. Any other target is read as the body it
        // defines: a revolve's body, not its axis.
        let read: Operand = crate::world::placed_var(doc, target)
            .map_or_else(|| as_body(doc, target.into()), Operand::from);
        match read {
            // A node with no body output: the kind gate's own refusal.
            Operand::Node(node) => self.require_kind(node, NodeKindWanted::Body)?,
            _ => self.require_read(&read, NodeKindWanted::Body)?,
        }
        let takeover = Takeover::of(doc, &read);
        takeover.stranded_pick(doc)?;
        let node = match kind {
            BlendKindChoice::Fillet => Node::fillet(read, size, selection),
            BlendKindChoice::Chamfer => Node::chamfer(read, size, selection),
        };
        // A feature gesture's one action (A10, Ev's residue 1): the
        // result, taking over what read the target, so the picture
        // shows the result where it showed the target and one undo
        // restores both.
        Ok(creating(move |run| {
            let result = run.insert(node)?;
            takeover.repoint(run, result, None)?;
            Ok(result)
        }))
    }

    /// **The space the body `read` is in**, where a gesture writes what
    /// it makes of the body. A read in no space refuses
    /// [`Refusal::NotABodyRead`]: nothing lands in the world unasked.
    fn home_space(&self, read: &Operand) -> Result<VarId, Refusal> {
        self.committed_doc()
            .operand_space(read)
            .ok_or_else(|| Refusal::NotABodyRead {
                read: Box::new(read.clone()),
            })
    }

    /// [`Self::require_kind`] of a seat's read: a node named alone, or
    /// one of its ports ([`admits_read`]).
    fn require_read(&self, read: &Operand, wanted: NodeKindWanted) -> Result<(), Refusal> {
        let doc = self.committed_doc();
        if admits_read(doc, read, wanted) {
            return Ok(());
        }
        match crate::world::read_output(doc, read) {
            Some((node, _)) => Err(Refusal::WrongNodeKind {
                node: doc.spoken(node),
                wanted,
            }),
            None => match read {
                Operand::Var(var) => Err(Refusal::NoSuchVariable(*var)),
                _ => Err(Refusal::NotABodyRead {
                    read: Box::new(read.clone()),
                }),
            },
        }
    }

    /// **The gate a placer's input asks**: a body, or a placement's
    /// copies ([`moves_copies`]) — a pattern of an instance places its
    /// copies, which the kernel's placer seat admits
    /// (`SlotKind::Placeable`). Anything else is refused as a body seat
    /// refuses, naming the body it wanted.
    fn require_placeable(&self, read: &Operand) -> Result<(), Refusal> {
        let doc = self.committed_doc();
        let copies =
            crate::world::read_output(doc, read).is_some_and(|(node, _)| moves_copies(doc, node));
        if copies && admits_read(doc, read, NodeKindWanted::Instances) {
            return Ok(());
        }
        self.require_read(read, NodeKindWanted::Body)
    }

    /// The node-kind gate every creation seat shares: the named node
    /// must be the wanted kind in the committed document — absent and
    /// wrong-kind refuse the same arm, because both mean "there is
    /// nothing of that kind there to consume".
    fn require_kind(&self, node: RecipeNodeId, wanted: NodeKindWanted) -> Result<(), Refusal> {
        let doc = self.committed_doc();
        if admits(doc, node, wanted) {
            Ok(())
        } else {
            Err(Refusal::WrongNodeKind {
                node: doc.spoken(node),
                wanted,
            })
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
            // A slot's expression, against the one the node stands at
            // — the same comparison for a structural slot, which
            // differs only in which edit carries it. The offered one is
            // compared as written, its name leaves read as the variables
            // they name, since the slot's written form reads ids.
            DocEdit::SetParam {
                node,
                slot,
                value: pncad::document::SlotValue::Formula(expr),
                ..
            }
            | DocEdit::SetStructuralParam {
                node, slot, expr, ..
            } => doc
                .resolve(expr)
                .is_ok_and(|offered| doc.slot_expansion(*node, *slot) == Some(offered)),
            // A read is lowered by the door, against the document it
            // enters; no panel offers one, so it is never a field's
            // standing value.
            DocEdit::SetParam {
                value: pncad::document::SlotValue::Read(_),
                ..
            } => false,
            // A declaration's two independent fields, each against
            // its own half. A kind that does not match is no match:
            // the edit is a redeclaration and the door refuses it.
            DocEdit::SetVarValue { var, value } => {
                match (doc.resolve_var(var).and_then(|id| doc.free(id)), value) {
                    (
                        Some(FreeVar::Continuous { value: stood, .. }),
                        FreeValue::Continuous(offered),
                    ) => stood == offered,
                    (Some(FreeVar::Count { value: stood }), FreeValue::Count(offered)) => {
                        stood == offered
                    }
                    _ => false,
                }
            }
            DocEdit::SetVarUnit { var, unit } => matches!(
                doc.resolve_var(var).and_then(|id| doc.free(id)),
                Some(FreeVar::Continuous { display_unit, .. }) if display_unit == unit
            ),
            // The rename field's text, against the label the node has.
            DocEdit::SetLabel { node, label } => doc.label(*node) == label.as_ref(),
            // Every other edit submits. The structure of the recipe and
            // the shape of the product: a node inserted, deleted,
            // re-parented or re-pointed has no standing value of its
            // own for an offered one to equal.
            DocEdit::InsertNode { .. }
            | DocEdit::DeleteNode { .. }
            | DocEdit::SetMembers { .. }
            | DocEdit::SetLoftSections { .. }
            // What the document makes: structure, not a panel field.
            | DocEdit::SetProduct { .. }
            // A profile's program replaced whole: structure, not a
            // panel field's value — and the identity program keeping
            // every step is the door's own no-op.
            | DocEdit::SetProgram { .. }
            | DocEdit::Rebind { .. }
            | DocEdit::UpdateReference { .. }
            // A placement's bundle gaining or losing a constraint is
            // structure: where a copy stands, not a panel field's value.
            | DocEdit::AddConstraint { .. }
            | DocEdit::RemoveConstraint { .. }
            | DocEdit::RemoveConstraints { .. }
            | DocEdit::SetConstraints { .. }
            // The variable doors that are not the value or the
            // notation half. A declare mints a variable and a
            // definition replaces one whole: each is an act the door
            // refuses or performs on its own terms. A distribution is
            // an annotation no panel field shows. A rename's unchanged
            // name is the door's own refusal (`VarNameUnchanged`), and
            // a delete has no standing value to equal.
            | DocEdit::DeclareVar { .. }
            | DocEdit::DefineVar { .. }
            | DocEdit::SetVarDistribution { .. }
            | DocEdit::RenameVar { .. }
            | DocEdit::DeleteVar { .. }
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
    /// record, re-evaluate — and reconcile the display state against
    /// what the document alone says: a free-move frame whose space is
    /// gone is discarded (see `display`'s module docs for why discard
    /// and not zero), and a hide the picture can no longer honour is
    /// dropped, each reported on the outcome with the fault that caused
    /// it. A frame a placement now pins goes when the edit's evaluation
    /// lands (`DocSession::land`'s second prune).
    fn commit(&mut self, edit: DocEdit<ProfileProgram>) -> OpOutcome {
        self.commit_action(vec![edit])
    }

    /// **The product set to `bodies`** ([`SessionOp::SetProduct`]), as
    /// one action ([`write_product`]).
    fn set_product(&mut self, bodies: Vec<Operand>) -> OpOutcome {
        self.commit_run(|run| write_product(run, &bodies))
    }

    /// **Add to the product** ([`SessionOp::AddToProduct`]): the body
    /// `node` draws — the port `body` names where it has several — joins
    /// the product by the branch [`ProductDoor::of`] reads off the
    /// committed document, as one action.
    fn add_to_product(&mut self, node: RecipeNodeId, body: Option<u32>) -> OpOutcome {
        let doc = self.committed_doc();
        let door = match ProductDoor::of(doc, node, body) {
            Ok(door) => door,
            Err(refusal) => return OpOutcome::refused(refusal),
        };
        if let ProductDoor::Place(read) = &door
            && let Err(refusal) = self.require_placeable(read)
        {
            return OpOutcome::refused(refusal);
        }
        // The space the moved placement leaves, where the same history
        // step minted it with the placement: an "Add part…" insert's.
        let minted = match &door {
            ProductDoor::IntoWorld { placement, .. } => doc
                .node_space(*placement)
                .and_then(|space| doc.defined_by(space))
                .map(|(space, _)| space)
                .filter(|&space| matches!(doc.node(space), Some(Node::Space)))
                .filter(|&space| self.minted_together(space, *placement)),
            ProductDoor::Add(_) | ProductDoor::Place(_) => None,
        };
        self.commit_run(|run| {
            let copy = match door {
                ProductDoor::Add(copy) => copy,
                ProductDoor::IntoWorld { placement, copy } => {
                    into_space(run, placement, VarId::WORLD)?;
                    if let Some(id) = minted.filter(|&id| {
                        run.doc()
                            .output(id, 0)
                            .is_some_and(|space| !run.doc().reader_counts().contains_key(&space))
                    }) {
                        run.apply(DocEdit::DeleteNode { id })?;
                    }
                    copy
                }
                ProductDoor::Place(read) => {
                    let placement = run.insert(Node::place(
                        pncad::document::BodyRead::plain(read),
                        Operand::Var(VarId::WORLD),
                        editor_core::stand().bundle(),
                    ))?;
                    let Some(copy) = run.doc().output(placement, 0) else {
                        unreachable!("a placement defines its copy")
                    };
                    copy
                }
            };
            // A stray the product read already keeps its place in the slot.
            if crate::world::product_reads(run.doc()).contains(&copy) {
                return Ok(());
            }
            let bodies = crate::world::product_with(run.doc(), copy);
            write_product(run, &bodies)
        })
    }

    /// **Whether one history step on the path to the cursor inserted
    /// both `a` and `b`** — a gesture's own provenance, so the product
    /// door deletes only a space minted with the copy it moves, never
    /// one a person or a script made. A document opened with both
    /// already in it has no such step.
    fn minted_together(&self, a: RecipeNodeId, b: RecipeNodeId) -> bool {
        let path = self.history.path();
        path.windows(2).any(|step| {
            let before = self.history.entry(step[0]).doc();
            let after = self.history.entry(step[1]).doc();
            [a, b]
                .iter()
                .all(|&id| before.node(id).is_none() && after.node(id).is_some())
        })
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
    /// to the target is that they fed it survive, so deleting one pip's
    /// boolean out of a die leaves that pip's body in the document,
    /// unconsumed and unplaced — the cone takes the boolean's world
    /// placements with it, since they read it. Reconnecting a deleted node's
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

    /// **What `SessionOp::DeleteNode` would answer for `node` because
    /// the committed document does not hold it, or `None` where it
    /// does** — asked ahead of the click by the control that pushes
    /// the op.
    ///
    /// It walks [`Self::delete_node`]'s own branch over the document
    /// that op reads: an empty cascade takes the single edit, and that
    /// edit is staged through the door ([`Self::stage_run`]) and never
    /// recorded, so the refusal is `EditError`'s and has one home. A
    /// selection can outlive its feature here — a face resolves
    /// against the last LANDED run, which still holds a feature the
    /// committed document has dropped until the next run lands, and
    /// indefinitely after a cancel.
    ///
    /// Not everything `perform` can answer: a held value gesture
    /// refuses the op before this admission runs, and a held node's
    /// cascade is applied at the click rather than rehearsed every
    /// frame.
    pub fn delete_refusal(&self, node: RecipeNodeId) -> Option<Refusal> {
        if !cascade_delete_order(self.committed_doc(), node).is_empty() {
            return None;
        }
        self.stage_run(|run| run.apply(DocEdit::DeleteNode { id: node }).map(drop))
            .err()
    }

    /// The same door for an action that takes SEVERAL edits, a fixed
    /// list of them: one run of [`Self::commit_run`], so one user
    /// action is one undo.
    fn commit_action(&mut self, edits: Vec<DocEdit<ProfileProgram>>) -> OpOutcome {
        self.commit_run(|run| {
            for edit in edits {
                run.apply(edit)?;
            }
            Ok(())
        })
    }

    /// The same door again, for an action whose later edits name the
    /// ids its earlier ones MINTED: [`Self::stage_run`], then
    /// [`Self::record_run`].
    fn commit_run(
        &mut self,
        action: impl FnOnce(&mut Recording<'_, ProfileProgram>) -> Result<(), EditError>,
    ) -> OpOutcome {
        match self.stage_run(action) {
            Ok((staged, ())) => self.record_run(staged),
            Err(refusal) => OpOutcome::refused(refusal),
        }
    }

    /// **A creating door's run, recorded**, the node it made on the
    /// outcome ([`OpOutcome::made`]). Where `labelled` is given
    /// ([`SessionOp::CreateLabelled`]) the run ends with a `SetLabel`
    /// on that node, so the edits, the minted ids and the netted
    /// maintenance the outcome carries are the one recording's. A
    /// refused creation records nothing, its label included.
    fn create(&mut self, labelled: Option<Label>, door: Result<Creating, Refusal>) -> OpOutcome {
        let action = match door {
            Ok(action) => action,
            Err(refusal) => return OpOutcome::refused(refusal),
        };
        let staged = self.stage_run(|run| {
            let node = action(run)?;
            if let Some(label) = labelled {
                run.apply(DocEdit::SetLabel {
                    node,
                    label: Some(label),
                })?;
            }
            Ok(node)
        });
        match staged {
            Ok((staged, made)) => OpOutcome {
                made: Some(made),
                ..self.record_run(staged)
            },
            Err(refusal) => OpOutcome::refused(refusal),
        }
    }

    /// **One constraint removed** ([`SessionOp::RemoveConstraint`]):
    /// the bundle written whole again without it, by the door's rule
    /// ([`DocEdit::RemoveConstraint`]) — a mate taken away frees what it
    /// fixed, and each freedom left unnamed is written `0`, so the copy
    /// stays pinned.
    fn remove_constraint(&mut self, mate: pncad::document::MateRef) -> OpOutcome {
        self.commit_run(|run| {
            run.apply(DocEdit::RemoveConstraint {
                placement: mate.placement,
                constraint: mate.id,
            })
            .map(drop)
        })
    }

    /// **An action's edits, applied and not yet recorded**, beside what
    /// `action` answered.
    ///
    /// `action` records its edits on the [`Recording`] it is handed — an
    /// insert through [`Recording::insert`], which answers the id it
    /// minted, so a later edit names the node the door actually
    /// created.
    ///
    /// **All or nothing**: each edit is applied to the value the last
    /// one produced and nothing is recorded until [`Self::record_run`]
    /// takes the result. A refused edit ends the run — the recording
    /// answers its refusal from then on, `finish` included — so a
    /// refusal leaves the session on the document it started from
    /// whether or not `action` stops at it, and so does a caller that
    /// drops the staged run. That is purity doing the work — no
    /// rollback exists to be got wrong.
    fn stage_run<T>(
        &self,
        action: impl FnOnce(&mut Recording<'_, ProfileProgram>) -> Result<T, EditError>,
    ) -> Result<(Recorded<ProfileProgram>, T), Refusal> {
        // Nothing the door decides is recorded: the edits are the
        // history.
        let mut run = Recording::start(self.history.doc(), self.tol);
        let answer = action(&mut run);
        let refused = |error| Refusal::Edit(Box::new(error));
        let staged = run.finish().map_err(refused)?;
        let answer = answer.map_err(refused)?;
        if staged.edits.is_empty() {
            unreachable!("an action commits at least one edit")
        }
        Ok((staged, answer))
    }

    /// **A staged run, recorded**: the whole run is one history state,
    /// so one user action is one undo.
    fn record_run(&mut self, staged: Recorded<ProfileProgram>) -> OpOutcome {
        let Recorded {
            doc,
            edits,
            maintenance,
            minted,
        } = staged;
        let committed = edits.clone();
        self.history.commit_group(edits, doc);
        let pruned = self.display.prune(self.history.doc(), None);
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
        // The stage answers for the document this request replaces.
        self.derived.stage = None;
        self.stager.cancel();
        self.generation = self.generation.next();
        // ONE clone of the shown document per request: into the
        // `Arc` the session keeps, then out of it into the request the
        // seam owns. The second is the seam's vocabulary (`EvalRequest`
        // takes a value so a worker owns its copy) and is not a
        // retained copy — the session keeps exactly one.
        self.requested_doc = Arc::new(self.doc().clone());
        self.requested_resolver = self.run_resolver();
        // The landing this request will replace no longer wants its door.
        self.door.cancel();
        self.eval.submit(EvalRequest {
            generation: self.generation,
            doc: self.requested_doc.as_ref().clone(),
            tol: self.tol,
            resolver: self.requested_resolver.clone(),
        });
    }
}

/// **One staged operation** ([`DocSession::stage`]): what was staged and
/// what it came to, on the document [`Derived`] is about.
struct Stage {
    key: StagedKey,
    creation: Creation,
    answer: StageAnswer,
}

/// What a stage came to: its door's refusal, or the node it made in the
/// staged document and that document's evaluation once it lands.
enum StageAnswer {
    Refused(Refusal),
    Made {
        node: RecipeNodeId,
        doc: Doc<ProfileProgram>,
        evaluation: Option<Arc<Evaluation<f64>>>,
    },
}

/// **What a staged operation came to**, so far ([`DocSession::staged_for`]).
#[derive(Debug)]
pub enum Staged<'a> {
    /// Its door refused it, as committing it would.
    Refused(&'a Refusal),
    /// Its run has not landed.
    Checking,
    /// Its run landed: the staged document, the node the operation made
    /// in it, and the document's evaluation.
    Evaluated {
        /// The staged document.
        doc: &'a Doc<ProfileProgram>,
        /// The node the operation made.
        node: RecipeNodeId,
        /// The staged document's evaluation.
        evaluation: &'a Evaluation<f64>,
    },
}

/// **An "Add part…" gesture's frame**: the space it minted, the instance
/// in it, and the display frame written for the space — `None` until the
/// evaluation answering the insert lands, the identity where nothing else
/// was drawn.
#[derive(Debug, Clone, Copy)]
struct Insert {
    space: VarId,
    node: RecipeNodeId,
    frame: Option<Frame>,
}

/// **A creation's run, decided and not yet staged**: what a creating
/// door answers once its checks pass, the run answering the node it
/// made — the one a label or a form is about. The door reads the
/// session and writes nothing; [`DocSession::create`] stages the run,
/// labels it where asked, and records it.
type Creating =
    Box<dyn FnOnce(&mut Recording<'_, ProfileProgram>) -> Result<RecipeNodeId, EditError>>;

/// `action` as a [`Creating`] run.
fn creating(
    action: impl FnOnce(&mut Recording<'_, ProfileProgram>) -> Result<RecipeNodeId, EditError> + 'static,
) -> Creating {
    Box::new(action)
}

/// **The product slot written as `bodies`**: the one body, a `⊔` of
/// several inserted first, or nothing; the `⊔` the slot read before,
/// where nothing else reads it, deleted in the same run — the action
/// says what the document makes, not one more thing beside it.
fn write_product(
    run: &mut Recording<'_, ProfileProgram>,
    bodies: &[Operand],
) -> Result<(), EditError> {
    let doc = run.doc();
    let held = doc
        .product()
        .and_then(|var| doc.operation_of(var))
        .filter(|&at| matches!(doc.node(at), Some(Node::Assemble { .. })))
        .filter(|&at| {
            doc.output(at, 0)
                .is_some_and(|read| !doc.reader_counts().contains_key(&read))
        });
    if let Some(id) = held {
        run.apply(DocEdit::SetProduct { product: None })?;
        run.apply(DocEdit::DeleteNode { id })?;
    }
    let product = match bodies {
        [] => None,
        [one] => Some(one.clone()),
        several => {
            let members = pncad::document::Bodies::Spelled(
                several
                    .iter()
                    .cloned()
                    .map(pncad::document::BodyRead::plain)
                    .collect(),
            );
            Some(Operand::Node(run.insert(Node::Assemble { members })?))
        }
    };
    run.apply(DocEdit::SetProduct { product }).map(drop)
}

/// **Which branch of the product door a body takes**
/// ([`SessionOp::AddToProduct`]; `crates/viewer/README.md`, *A body
/// reaches the product through one door*).
#[derive(Debug)]
enum ProductDoor {
    /// A body in the world — a world copy, an instance there, a `⊔`
    /// there — or an existing world copy of the body: added to the slot
    /// as it is.
    Add(VarId),
    /// A placement's copy in another space that nothing reads
    /// ([`crate::world::movable_into_world`]): its placement re-targeted
    /// into the world, and the copy added. A placement defines one copy,
    /// so a pick's `body` names nothing more here.
    IntoWorld {
        /// The placement.
        placement: RecipeNodeId,
        /// Its copy.
        copy: VarId,
    },
    /// Any other body: placed into the world by the identity bundle,
    /// and that copy added.
    Place(Operand),
}

impl ProductDoor {
    /// The branch the body `node` draws takes in `doc` — the port
    /// `body` names, for a node with several outputs.
    ///
    /// **A body joins the product once.** Where the slot reads it, or a
    /// world copy of it, the door refuses [`Refusal::InTheProduct`]
    /// naming that copy — except a copy the slot reads from outside the
    /// world (`ProductOutsideWorld`) that the door can move back, which
    /// it moves. Where the world holds a copy of it the slot does not
    /// read, that copy is added rather than another placed beside it,
    /// which would stack coincident material. A node that names no one
    /// body refuses [`Refusal::NotABodyRead`].
    fn of(
        doc: &Doc<ProfileProgram>,
        node: RecipeNodeId,
        body: Option<u32>,
    ) -> Result<Self, Refusal> {
        let not_a_body = || Refusal::NotABodyRead {
            read: Box::new(Operand::Node(node)),
        };
        let read = crate::world::body_read(doc, node, body).ok_or_else(not_a_body)?;
        let var = crate::world::read_var(doc, &read).ok_or_else(not_a_body)?;
        let product = crate::world::product_reads(doc);
        let movable = crate::world::movable_into_world(doc, node);
        let in_world = doc.space_of(var) == Some(VarId::WORLD);
        if product.contains(&var) {
            return match movable {
                Some(copy) if !in_world => Ok(Self::IntoWorld {
                    placement: node,
                    copy,
                }),
                _ => Err(Refusal::InTheProduct {
                    copy: doc.spoken(node),
                }),
            };
        }
        let world_copies: Vec<(RecipeNodeId, VarId)> = crate::world::placements_of_var(doc, var)
            .into_iter()
            .filter(|&copy| doc.node_space(copy) == Some(VarId::WORLD))
            .filter_map(|copy| Some((copy, doc.output(copy, 0)?)))
            .collect();
        if let Some(&(copy, _)) = world_copies.iter().find(|(_, out)| product.contains(out)) {
            return Err(Refusal::InTheProduct {
                copy: doc.spoken(copy),
            });
        }
        if in_world {
            return Ok(Self::Add(var));
        }
        if let Some(&(_, copy)) = world_copies.first() {
            return Ok(Self::Add(copy));
        }
        if let Some(copy) = movable {
            return Ok(Self::IntoWorld {
                placement: node,
                copy,
            });
        }
        Ok(Self::Place(read))
    }
}

/// **What a takeover re-points** (spec §5′.3 (a)): every read of the
/// body `taken` along a path the picture is seen through — a gesture's
/// copy of it (a placement's source, [`crate::world::placements_of_var`]),
/// a `⊔` operand, and the product slot — read off the committed
/// document before the gesture's own nodes exist.
struct Takeover {
    /// The body taken over.
    taken: Option<VarId>,
    /// Each copy of `taken`, its source re-pointed.
    copies: Vec<RecipeNodeId>,
    /// Each `⊔` operand reading `taken` whole, by node and seat.
    operands: Vec<(RecipeNodeId, OperandSlot)>,
    /// Whether the product slot reads `taken` itself.
    product: bool,
}

impl Takeover {
    /// The takeover of the body `read` names in `doc`.
    fn of(doc: &Doc<ProfileProgram>, read: &Operand) -> Self {
        Self::of_var(doc, crate::world::read_var(doc, read))
    }

    /// The takeover of `taken` in `doc`.
    fn of_var(doc: &Doc<ProfileProgram>, taken: Option<VarId>) -> Self {
        let Some(var) = taken else {
            return Self {
                taken,
                copies: Vec::new(),
                operands: Vec::new(),
                product: false,
            };
        };
        let operands = doc
            .ids()
            .into_iter()
            .filter_map(|id| Some((id, doc.node(id)?)))
            .filter(|(_, node)| matches!(node, Node::Assemble { .. }))
            .flat_map(|(id, node)| {
                node.body_reads()
                    .into_iter()
                    .filter(|(_, read)| read.read == var && !read.is_indexed())
                    .map(move |(seat, _)| (id, seat))
            })
            .collect();
        Self {
            taken,
            copies: crate::world::placements_of_var(doc, var),
            operands,
            product: doc.product() == Some(var),
        }
    }

    /// Whether nothing reads the body along a seen path: it is drawn
    /// itself, or not at all.
    fn is_empty(&self) -> bool {
        self.copies.is_empty() && self.operands.is_empty() && !self.product
    }

    /// Whether a reader takes only one body: a `⊔` operand or the
    /// product slot, where a family is read through a `⊔` over it.
    fn reads_one_body(&self) -> bool {
        !self.operands.is_empty() || self.product
    }

    /// **The takeover refuses where it would strand a pick**
    /// ([`Refusal::StrandedPick`]): the first reader it re-points that a
    /// pick reads through — a copy of the body that is the pick's own
    /// placement, or a `⊔` whose operand is the piece the pick names,
    /// read by the pick's placement or along its path. A feature's
    /// result, a mirror's or a sketch's union, a combine and a pattern
    /// hold no pieces, and a pick is never re-pointed, so re-pointing
    /// that reader would leave the pick naming a piece its copy no
    /// longer holds. The transform's takeover places the body it took,
    /// whose pieces its copy carries, and asks nothing here.
    fn stranded_pick(&self, doc: &Doc<ProfileProgram>) -> Result<(), Refusal> {
        let picks: Vec<(VarId, &pncad::document::Pick)> = doc
            .vars()
            .iter()
            .filter_map(|(&var, held)| match held.def() {
                pncad::document::VarDef::Pick(pick) => Some((var, pick)),
                _ => None,
            })
            .collect();
        let stranded = |reader: RecipeNodeId, pick: VarId| {
            Err(Refusal::StrandedPick {
                copy: doc.spoken(reader),
                pick: doc.spoken_var(pick),
            })
        };
        for &copy in &self.copies {
            let outputs = doc.outputs(copy);
            if let Some(&(pick, _)) = picks
                .iter()
                .find(|(_, held)| outputs.contains(&held.placement))
            {
                return stranded(copy, pick);
            }
        }
        // What a pick's placement copies — down the whole chain of
        // copies, a copy of a copy (a transform's) included — and the
        // path to its piece.
        let reads = |held: &pncad::document::Pick| -> Vec<VarId> {
            let mut copied = Vec::new();
            let mut at = held.placement;
            while let Some(Node::Place { source, .. }) =
                doc.operation_of(at).and_then(|node| doc.node(node))
            {
                if copied.contains(&source.read) {
                    break;
                }
                copied.push(source.read);
                at = source.read;
            }
            copied.into_iter().chain(held.of.iter().copied()).collect()
        };
        for &(union, _) in &self.operands {
            let outputs = doc.outputs(union);
            let through = picks.iter().find(|(_, held)| {
                self.taken.is_some_and(|taken| held.of.contains(&taken))
                    && reads(held).iter().any(|var| outputs.contains(var))
            });
            if let Some(&(pick, _)) = through {
                return stranded(union, pick);
            }
        }
        Ok(())
    }

    /// **Re-point every reader to `result`** (A10, Ev's residue 1),
    /// through the slot door, in the action that authored `result`: a
    /// copy of the body copies `result` in its place, a `⊔` reads it
    /// as its operand, and the product slot reads it. `whole` is what
    /// a reader of one body reads where `result` is several (a
    /// pattern's family): a `⊔` over them.
    fn repoint(
        self,
        run: &mut Recording<'_, ProfileProgram>,
        result: RecipeNodeId,
        whole: Option<RecipeNodeId>,
    ) -> Result<(), EditError> {
        let one = whole.unwrap_or(result);
        let slots = self
            .copies
            .into_iter()
            .map(|copy| (copy, OperandSlot::Source, result))
            .chain(
                self.operands
                    .into_iter()
                    .map(|(node, seat)| (node, seat, one)),
            );
        for (node, seat, to) in slots {
            run.apply(DocEdit::SetParam {
                node,
                slot: SlotId::Operand(seat),
                value: pncad::document::SlotValue::Read(to.into()),
                fresh: Vec::new(),
            })?;
        }
        if self.product {
            run.apply(DocEdit::SetProduct {
                product: Some(one.into()),
            })?;
        }
        Ok(())
    }
}

/// **The body `node` defines, as a read**: its first `Body` output (a
/// revolve defines its body and its axis, and naming it alone reads
/// neither), else the node alone.
fn body_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Operand {
    body_outputs(doc, node)
        .first()
        .map_or_else(|| node.into(), |&var| Operand::from(var))
}

/// **The `Body` outputs `node` defines**, in port order.
fn body_outputs(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Vec<VarId> {
    doc.outputs(node)
        .into_iter()
        .filter(|&var| {
            doc.var(var)
                .is_some_and(|v| v.kind() == pncad::document::VarKind::Body)
        })
        .collect()
}

/// **A body seat's read of `read`**: a node named alone that is no body
/// itself but defines exactly one, a revolve beside its axis say, read
/// as that body ([`body_of`]); any other read as it is, so a split's two
/// halves still refuse at the kind gate. A node's copy was read this way
/// when a pick reached it through its placement; drawn unplaced, it is
/// picked on itself.
fn as_body(doc: &Doc<ProfileProgram>, read: Operand) -> Operand {
    match read {
        Operand::Node(node)
            if doc.node(node).is_some()
                && !admits(doc, node, NodeKindWanted::Body)
                && body_outputs(doc, node).len() == 1 =>
        {
            body_of(doc, node)
        }
        read => read,
    }
}

/// **A combine's operands, in the first one's space** (D10
/// `[placement.spaces]`): each operand after the first that lives in
/// another space is placed into the first's, at the identity of its
/// axes.
///
/// A construction is placed into the first's space and read through
/// that placement. A copy placed into its own body's space by values
/// alone (the transform gesture's) is re-targeted into the first's
/// space with its values as written, so it stands on the first's axes
/// as it stood on its body's and every variable keeps driving it. An
/// operand in no space, or a copy whose bundle holds a mate (which
/// reads its body's space), is read as it is: the combine then refuses
/// `SpaceMismatch` at the edit door, in the kernel's words.
fn in_one_space(
    run: &mut Recording<'_, ProfileProgram>,
    operands: &[Operand],
) -> Result<Vec<Operand>, EditError> {
    let Some((first, rest)) = operands.split_first() else {
        return Ok(Vec::new());
    };
    let Some(home) = run.doc().operand_space(first) else {
        return Ok(operands.to_vec());
    };
    let node_of = |doc: &Doc<ProfileProgram>, read: &Operand| {
        crate::world::read_output(doc, read).map(|(node, _)| node)
    };
    let mut read = vec![first.clone()];
    for operand in rest {
        let Some(node) = node_of(run.doc(), operand) else {
            read.push(operand.clone());
            continue;
        };
        if run.doc().operand_space(operand) == Some(home) {
            read.push(operand.clone());
            continue;
        }
        // A copy placed into its own body's space.
        let self_placed = match run.doc().node(node) {
            Some(Node::Place { source, .. }) => {
                run.doc().space_of(source.read) == run.doc().node_space(node)
            }
            _ => false,
        };
        if self_placed {
            if held_bundle(run.doc(), node)
                .iter()
                .any(|c| c.mate().is_some())
            {
                read.push(operand.clone());
                continue;
            }
            run.apply(DocEdit::SetParam {
                node,
                slot: SlotId::Operand(pncad::document::OperandSlot::Space),
                value: pncad::document::SlotValue::Read(Operand::Var(home)),
                fresh: Vec::new(),
            })?;
            read.push(operand.clone());
        } else {
            // A body never placed stands where it stands.
            let placement = run.insert(Node::place(
                pncad::document::BodyRead::plain(operand.clone()),
                Operand::Var(home),
                pncad::document::stand().bundle(),
            ))?;
            read.push(placement.into());
        }
    }
    Ok(read)
}

/// **A combine's takeover**: the first operand, in the spec's order,
/// that something reads along a seen path ([`Takeover`]) has those
/// readers re-pointed to `result`, and every other operand's copies are
/// deleted — its material is in the result — so a combine of placed
/// bodies leaves `[result]`. A repeated operand counts once; operands
/// nothing reads re-point nothing.
fn combine_in_world(
    run: &mut Recording<'_, ProfileProgram>,
    operands: &[Operand],
    result: RecipeNodeId,
) -> Result<(), EditError> {
    let mut distinct: Vec<&Operand> = Vec::new();
    for operand in operands {
        if !distinct.contains(&operand) {
            distinct.push(operand);
        }
    }
    let mut read = distinct
        .into_iter()
        .map(|operand| Takeover::of(run.doc(), operand))
        .filter(|takeover| !takeover.is_empty())
        .collect::<Vec<_>>()
        .into_iter();
    let Some(first) = read.next() else {
        return Ok(());
    };
    first.repoint(run, result, None)?;
    for placement in read.flat_map(|takeover| takeover.copies) {
        run.apply(DocEdit::DeleteNode { id: placement })?;
    }
    Ok(())
}

/// **`held` with the mate panel's `values` written over it**: each
/// value of `held` on a freedom `values` sets takes that number, keeping
/// its id; a freedom `values` sets that `held` holds no value on is
/// appended, for the door to refuse as the bundle's own
/// ([`pncad::document::MateFault::Overconstrained`]).
pub(crate) fn valued(
    held: Vec<pncad::document::Constraint<Formula>>,
    values: Vec<(pncad::document::Coord, Formula)>,
) -> Vec<pncad::document::Constraint<Formula>> {
    use pncad::document::{Constraint, ConstraintId, Value};
    let mut values = values;
    let mut bundle: Vec<Constraint<Formula>> = held
        .into_iter()
        .map(|c| {
            let set = c
                .value()
                .and_then(|v| values.iter().position(|(coord, _)| *coord == v.coord));
            match set {
                Some(at) => {
                    let (coord, by) = values.remove(at);
                    Constraint::of_value(c.id, Value { coord, by })
                }
                None => c,
            }
        })
        .collect();
    bundle.extend((1..).zip(values).map(|(i, (coord, by))| {
        Constraint::of_value(ConstraintId::placeholder(i), Value { coord, by })
    }));
    bundle
}

/// **`placement`'s bundle as authored**: each constraint re-authored,
/// so a value keeps reading its variable by id. Empty for a node that
/// is no placement.
pub(crate) fn held_bundle(
    doc: &Doc<ProfileProgram>,
    placement: RecipeNodeId,
) -> Vec<pncad::document::Constraint<Formula>> {
    match doc.node(placement).map(|node| node.authored(doc)) {
        Some(Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. }) => {
            constraints
        }
        _ => Vec::new(),
    }
}

/// **`constraints` written onto `placement` whole**
/// ([`DocEdit::SetConstraints`]). A mate reads a pose in the space its
/// copy is placed into (D10 `[placement.spaces]`), and the gesture never
/// moves the copy into another space as a side effect: a mate whose `to`
/// is outside the placement's target is refused by the door,
/// [`EditError::SpaceMismatch`] naming both spaces. Moving the copy is a
/// gesture of its own ([`SessionOp::IntoOwnSpace`]).
fn write_bundle(
    run: &mut Recording<'_, ProfileProgram>,
    placement: RecipeNodeId,
    constraints: Vec<pncad::document::Constraint<Formula>>,
) -> Result<(), EditError> {
    run.apply(DocEdit::SetConstraints {
        placement,
        constraints,
        fresh: Vec::new(),
    })?;
    Ok(())
}

/// **`placement` moved into `space` by name**: a bundle holding a mate
/// first stood by the identity bundle, which reads nothing, so the
/// re-target strands no mate of the bundle's own, then its `target`
/// re-pointed there.
fn into_space(
    run: &mut Recording<'_, ProfileProgram>,
    placement: RecipeNodeId,
    space: VarId,
) -> Result<(), EditError> {
    if held_bundle(run.doc(), placement)
        .iter()
        .any(|c| c.mate().is_some())
    {
        write_bundle(run, placement, editor_core::stand().bundle())?;
    }
    run.apply(DocEdit::SetParam {
        node: placement,
        slot: SlotId::Operand(OperandSlot::Space),
        value: pncad::document::SlotValue::Read(Operand::Var(space)),
        fresh: Vec::new(),
    })?;
    Ok(())
}

/// **A gesture's numbers at their nominal values** in `doc`: each
/// formula's names read as the variables the document holds them as,
/// evaluated at their values — `what` naming the field in a refusal.
fn nominals(
    doc: &Doc<ProfileProgram>,
    what: &'static str,
    formulas: &[Formula; 3],
    wanted: Dimension,
) -> Result<[f64; 3], Refusal> {
    let [a, b, c] = formulas;
    Ok([
        nominal(doc, what, a, wanted)?,
        nominal(doc, what, b, wanted)?,
        nominal(doc, what, c, wanted)?,
    ])
}

/// One of [`nominals`].
fn nominal(
    doc: &Doc<ProfileProgram>,
    field: &'static str,
    formula: &Formula,
    wanted: Dimension,
) -> Result<f64, Refusal> {
    let refused = |fault| Refusal::Frame(Box::new(fault));
    let found = formula.dim();
    if found != wanted {
        return Err(refused(GestureFrameFault::Dimension {
            field,
            wanted,
            found,
        }));
    }
    let held = doc
        .resolve(formula)
        .map_err(|fault| refused(GestureFrameFault::Unresolved { field, fault }))?;
    pncad::document::eval::<f64>(&held, &doc.var_env())
        .map_err(|error| refused(GestureFrameFault::Unevaluated { field, error }))
}

/// **The frame the transform gesture's numbers make**: rotated by
/// `angle` about `axis` through the origin, then translated, at their
/// nominal values ([`Frame::rotate_then_translate`]).
fn gesture_frame(
    doc: &Doc<ProfileProgram>,
    translation: &[Formula; 3],
    axis: &[Formula; 3],
    angle: &Formula,
    tol: Tol,
) -> Result<Frame, Refusal> {
    let translation = nominals(doc, "translation", translation, Dimension::Length)?;
    let axis = nominals(doc, "rotation axis", axis, Dimension::Scalar)?;
    let angle = nominal(doc, "rotation angle", angle, Dimension::Angle)?;
    let band = pncad::geom_core::Band::linear(tol)
        .unwrap_or_else(|_| unreachable!("the session's tolerance is a band"));
    Frame::rotate_then_translate(axis, angle, translation, band).map_err(|refusal| {
        Refusal::Frame(Box::new(GestureFrameFault::Direction {
            field: "rotation axis",
            refusal,
        }))
    })
}

/// **The frame at `origin` whose `z` is `direction`**, its `x` the
/// first world axis off the direction made orthogonal to it — the
/// frame a plane or an axis given by a point and a direction is read
/// through. A direction that names none refuses as the direction door
/// refuses it.
fn frame_along(
    origin: [f64; 3],
    direction: [f64; 3],
    what: &'static str,
    tol: Tol,
) -> Result<Frame, Refusal> {
    use pncad::geom_core::Vec3;
    let band = pncad::geom_core::Band::linear(tol)
        .unwrap_or_else(|_| unreachable!("the session's tolerance is a band"));
    // The direction door's own refusal for a direction that names none.
    Frame::rotate_then_translate(direction, 0.0, origin, band).map_err(|refusal| {
        Refusal::Frame(Box::new(GestureFrameFault::Direction {
            field: what,
            refusal,
        }))
    })?;
    let n = Vec3::new(direction[0], direction[1], direction[2]);
    let n = n * (1.0 / n.norm());
    let seed = if n.x.abs() <= n.y.abs().max(n.z.abs()) {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let x = seed - n * seed.dot(n);
    let x = x * (1.0 / x.norm());
    let y = n.cross(x);
    Ok(Frame {
        columns: [[x.x, x.y, x.z], [y.x, y.y, y.z], [n.x, n.y, n.z]],
        translation: origin,
    })
}

/// **The transform gesture's values** on the axes of the space its body
/// is in: the slides its translation writes, as written, and the spin
/// its rotation writes — about the space's own axis, by the angle as
/// written, where the rotation axis is one of the three, else the chain
/// of turns about `x`, `y` and `z` that `at` (the frame the gesture's
/// numbers make, [`gesture_frame`]) turns by ([`moves_of`]).
fn gesture_moves(
    doc: &Doc<ProfileProgram>,
    translation: &[Formula; 3],
    axis: &[Formula; 3],
    angle: &Formula,
    at: &Frame,
) -> Result<Vec<(pncad::document::Coord, Formula)>, Refusal> {
    use pncad::document::{Coord, TargetAxis};
    let about = nominals(doc, "rotation axis", axis, Dimension::Scalar)?;
    let lone = TargetAxis::ALL
        .into_iter()
        .zip(about)
        .filter(|&(_, c)| c != 0.0);
    let spins = match lone.collect::<Vec<_>>().as_slice() {
        [(axis, sense)] => {
            let by = if *sense > 0.0 {
                angle.clone()
            } else {
                Formula::neg(angle.clone())
                    .unwrap_or_else(|_| unreachable!("an angle negates to an angle"))
            };
            vec![(Coord::Spin(Some(*axis)), by)]
        }
        _ => moves_of(&Frame {
            columns: at.columns,
            translation: [0.0; 3],
        }),
    };
    let slides = TargetAxis::ALL
        .into_iter()
        .zip(translation.iter().cloned())
        .map(|(axis, by)| (Coord::Slide(axis), by));
    Ok(spins.into_iter().chain(slides).collect())
}

/// **The values standing a copy at `at`** on its target space's axes:
/// the chain of turns about `x`, `y` and `z` its axes make — each a
/// quarter turn written exactly (`turn/4`) where they are a signed
/// permutation of the space's, else the angles `atan2` takes off them
/// (`linear = Rz(γ)·Ry(β)·Rx(α)`) — and the slides its origin makes,
/// each turn and slide that is not zero.
fn moves_of(at: &Frame) -> Vec<(pncad::document::Coord, Formula)> {
    use pncad::document::{Coord, TargetAxis};
    // Entry `r(i, j)`: row `i` of column `j`.
    let r = |i: usize, j: usize| at.columns[j][i];
    let exact = (0..3).all(|j| {
        let column = at.columns[j];
        column.iter().filter(|&&c| c != 0.0).count() == 1
            && column.iter().all(|&c| c == 0.0 || c.abs() == 1.0)
    });
    let quarter = |angle: f64| -> Formula {
        let quarters = (angle / std::f64::consts::FRAC_PI_2).round();
        let turn =
            |den| Formula::turn_over(den).unwrap_or_else(|_| unreachable!("a turn is an angle"));
        match quarters as i64 {
            1 => turn(4),
            2 | -2 => turn(2),
            _ => Formula::neg(turn(4)).unwrap_or_else(|_| unreachable!("an angle negates")),
        }
    };
    let cos_beta = r(0, 0).hypot(r(1, 0));
    let beta = (-r(2, 0)).atan2(cos_beta);
    let (alpha, gamma) = if cos_beta > f64::EPSILON.sqrt() {
        (r(2, 1).atan2(r(2, 2)), r(1, 0).atan2(r(0, 0)))
    } else {
        ((-r(1, 2)).atan2(r(1, 1)), 0.0)
    };
    let spins = [
        (TargetAxis::X, alpha),
        (TargetAxis::Y, beta),
        (TargetAxis::Z, gamma),
    ]
    .into_iter()
    .filter(|&(_, angle)| angle != 0.0)
    .map(|(axis, angle)| {
        let by = if exact {
            quarter(angle)
        } else {
            Formula::literal(angle, Dimension::Angle)
                .unwrap_or_else(|_| unreachable!("a finite angle is a literal"))
        };
        (Coord::Spin(Some(axis)), by)
    });
    let slides = TargetAxis::ALL
        .into_iter()
        .zip(at.translation)
        .filter(|&(_, by)| by != 0.0)
        .map(|(axis, by)| {
            let by = Formula::literal(by, Dimension::Length)
                .unwrap_or_else(|_| unreachable!("a finite length is a literal"));
            (Coord::Slide(axis), by)
        });
    spins.chain(slides).collect()
}

/// How many rays each way [`DocSession::sketch_trace`] samples a trace
/// with: enough points to read a circle's shape by, few enough to
/// redraw each time the picks or the landed evaluation move.
const SKETCH_TRACE_RAYS: u32 = 24;

/// How far past the traced body's reach the trace is shown: a quarter
/// more, so the body's own outline is inside the window.
const TRACE_WINDOW_MARGIN: f64 = 1.25;

/// **Why a face's trace could not be shown** ([`DocSession::sketch_trace`]).
#[derive(Debug, Clone)]
pub enum SketchTraceFault {
    /// Nothing has landed to read the trace against.
    Unevaluated,
    /// No frame reads off the picks ([`drawn_frame`]): the traced face
    /// has no axis, or the picked face is no plane square to it.
    Frame(Arc<pncad::trace::TracedFrameFault>),
    /// The trace's own refusal: the plane misses the face, the carrier
    /// has no closed form, or a read refused.
    Trace(Arc<pncad::trace::SketchTraceError>),
}

impl core::fmt::Display for SketchTraceFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unevaluated => f.write_str("the document has not been evaluated yet"),
            Self::Frame(e) => write!(f, "{e}"),
            Self::Trace(e) => write!(f, "{e}"),
        }
    }
}

impl core::error::Error for SketchTraceFault {}

/// **The body a pick is read on, and the entity's name there**
/// (`crate::world::body_of`; a copy carries its body's names, so the
/// name is the pick's): the node's output by its port where it holds
/// several, its body output otherwise.
fn picked_body(
    doc: &Doc<ProfileProgram>,
    pick: &impl crate::tree::PickedEntity,
) -> (Operand, StableName) {
    let on = pick.on();
    let (at, port) = crate::world::body_of(doc, on.node(), on.body());
    let name = pick.name().clone();
    let body = match crate::world::seat_read(doc, at, Some(port)) {
        (at, Some(port)) => Operand::output(at, port),
        (at, None) => body_of(doc, at),
    };
    (body, name)
}

/// **The entity a pick selects, as an operand** — a face, or for the
/// mate tool an edge or a vertex: its name on the body the pick is read
/// on ([`picked_body`]).
pub(crate) fn pick_read(
    doc: &Doc<ProfileProgram>,
    pick: &impl crate::tree::PickedEntity,
) -> Operand {
    let (body, name) = picked_body(doc, pick);
    Operand::select(body, vec![name])
}

/// **The frame a sketch tracing `traced` stands on, in the plane of
/// `plane`** ([`pncad::trace::traced_frame`]): `Through` the traced
/// face's own axis and a vertex of the picked plane face. A boss swept
/// up that axis from a piece leaving the trace along its tangent is
/// tangent to the traced wall by construction: the coincidence door
/// proves the union's tangency row `SameConstruction`.
fn drawn_frame(
    doc: &Doc<ProfileProgram>,
    ev: &Evaluation<f64>,
    plane: &FaceSelection,
    traced: &FaceSelection,
    tol: Tol,
) -> Result<Operand, pncad::trace::TracedFrameFault> {
    pncad::trace::traced_frame(
        doc,
        ev,
        &pick_read(doc, plane),
        &pick_read(doc, traced),
        tol,
    )
}

/// How far `node`'s body reaches from its own origin across its `xy`:
/// the window a trace is shown across.
fn body_reach(ev: &Evaluation<f64>, node: pncad::document::RecipeNodeId) -> Option<f64> {
    use pncad::document::ValuePayload;
    let value = ev.usable(node).ok()?;
    let body = match &value.payload {
        ValuePayload::Body(b)
        | ValuePayload::Boolean(pncad::document::BooleanValue::Body { body: b, .. }) => b.shared(),
        _ => return None,
    };
    body.vertices()
        .filter_map(|(v, _)| pncad::topo::readback::vertex_point(body, v).ok())
        .map(|p| p.x.abs().max(p.y.abs()))
        .reduce(f64::max)
        .filter(|&r| r > 0.0)
}

/// **A frame read off `body`'s own geometry, standing at `at` in its
/// own coordinates**: `Through` the first edge by id with an axis and
/// the first vertex by id off its line, moved to `at` by the moves its
/// axes and origin make there ([`moves_of`],
/// [`pncad::document::moved_frame`]). A copy placed into its body's
/// space is read through its body, `at` carried by the copy's motion.
///
/// # Errors
///
/// [`pncad::document::PinRefusal`]: `body` reads no node, or one that
/// did not evaluate to a body, or a body with no such edge and vertex.
fn frame_off(
    doc: &Doc<ProfileProgram>,
    ev: &Evaluation<f64>,
    body: &Operand,
    at: &Frame,
    tol: Tol,
) -> Result<Operand, pncad::document::PinRefusal> {
    use editor_core::names::{EntityKey, EntityRef};
    use pncad::document::{FrameMove, NodeResult, PinRefusal, ValuePayload};
    use pncad::geom_core::UnitVec3;
    use pncad::topo::readback;
    let node = match body {
        Operand::Output { node, .. } | Operand::Node(node) => *node,
        Operand::Var(var) => doc.operation_of(*var).ok_or(PinRefusal::NoNode)?,
        _ => return Err(PinRefusal::NoNode),
    };
    let Some(NodeResult::Ok(value)) = ev.result(node) else {
        return Err(PinRefusal::Unevaluated { node });
    };
    // A copy's body is its source's, in the source's coordinates: the
    // frame is read there, as `at` states it.
    let solid = match &value.payload {
        ValuePayload::Body(b)
        | ValuePayload::Boolean(pncad::document::BooleanValue::Body { body: b, .. }) => {
            std::sync::Arc::clone(b.shared())
        }
        _ => return Err(PinRefusal::NotABody { node }),
    };
    let var = match body {
        Operand::Var(var) => Some(*var),
        Operand::Output { node, port } => doc.output(*node, *port),
        _ => doc.output(node, 0),
    };
    let table = var
        .and_then(|var| ev.table(var))
        .ok_or(PinRefusal::NotABody { node })?;
    let band = pncad::geom_core::Band::linear(tol)
        .unwrap_or_else(|_| unreachable!("the session's tolerance is a band"));
    let name = |key| table.name_of(&EntityRef { body: 0, key }).cloned();
    let picked = solid.edges().find_map(|(edge, _)| {
        let kind = readback::edge_carrier_kind(&solid, edge).ok()?;
        matches!(
            kind,
            pncad::geom::CurveKind::Line
                | pncad::geom::CurveKind::Circle
                | pncad::geom::CurveKind::Ellipse
        )
        .then_some(())?;
        let line = readback::edge_pose(&solid, edge).ok()?;
        let dir = UnitVec3::new(line.axis, pncad::topo::query::POSE_UNIT_NORM, band).ok()?;
        let edge_name = name(EntityKey::Edge(edge))?;
        solid.vertices().find_map(|(vertex, _)| {
            let point = readback::vertex_point(&solid, vertex).ok()?;
            let off = point - line.origin;
            let across = off - dir.get() * off.dot(dir.get());
            UnitVec3::new(across, "frame_vertex_off_the_line", band).ok()?;
            let vertex_name = name(EntityKey::Vertex(vertex))?;
            let foot = line.origin + dir.get() * off.dot(dir.get());
            let frame = pncad::geom_core::RigidMotion::from_aim_and_reference(
                foot,
                dir,
                off,
                pncad::topo::query::POSE_UNIT_NORM,
                band,
            )
            .ok()?;
            Some((edge_name.clone(), vertex_name, frame))
        })
    });
    let (edge_name, vertex_name, frame) = picked.ok_or(PinRefusal::NoFrame { node })?;
    let at = at.affine();
    use pncad::document::pose::PoseDef;
    let read = |def: PoseDef<Operand, Formula>| RoleRead::plain(Operand::Pose(Box::new(def)));
    let base = Operand::Pose(Box::new(PoseDef::Through {
        axis: read(PoseDef::Axis {
            of: Operand::select(body.clone(), vec![edge_name]),
        }),
        point: read(PoseDef::Point {
            of: Operand::select(body.clone(), vec![vertex_name]),
        }),
    }));
    let moves = moves_of(&Frame::from_affine(frame.to_affine().inverse() * at))
        .into_iter()
        .map(|(coord, by)| match coord {
            pncad::document::Coord::Spin(Some(axis)) => FrameMove::Spin(axis, by),
            pncad::document::Coord::Spin(None) => {
                unreachable!("a frame's moves are a chain about its own axes")
            }
            pncad::document::Coord::Slide(axis) => FrameMove::Slide(axis, by),
        })
        .collect::<Vec<_>>();
    Ok(pncad::document::moved_frame(base, &moves))
}

/// [`DocSession::space_evaluation`], over the two session fields it
/// reads: the shown document is the scratch while a value gesture
/// previews, the committed one otherwise ([`DocSession::doc`]).
fn space_evaluation<'a>(derived: &'a Derived, history: &History) -> Option<&'a Evaluation<f64>> {
    let run = derived.landed.as_ref()?;
    let committed = history.doc();
    let shown = derived.scratch.as_ref().unwrap_or(committed);
    (*run.doc == *committed || *run.doc == *shown).then(|| run.evaluation.as_ref())
}

/// One placement a duplicate steps from: the space its copy is in, and
/// where it stands there.
type DuplicateSeat = (VarId, Frame);

/// **Where a duplicate of `input` is placed** ([`SessionOp::Duplicate`]):
/// for `beside`, or else for each copy of `input`, that copy's target
/// space and its evaluated motion ([`combine::copy_motion`]). Empty for
/// a body nothing copies.
///
/// # Errors
///
/// [`DuplicateFault::NotACopy`] for a `beside` that is not a drawn
/// copy of `input`; [`DuplicateFault::NoValue`] for a copy `eval`
/// holds no value for.
fn duplicate_seats(
    doc: &Doc<ProfileProgram>,
    eval: &Evaluation<f64>,
    input: &Operand,
    beside: Option<RecipeNodeId>,
) -> Result<Vec<DuplicateSeat>, DuplicateFault> {
    let drawn = crate::world::placements_of_read(doc, input);
    let copies = match beside {
        Some(copy) if drawn.contains(&copy) => vec![copy],
        Some(copy) => {
            let input = crate::world::read_output(doc, input)
                .map_or_else(|| doc.spoken(copy), |(node, _)| doc.spoken(node));
            return Err(DuplicateFault::NotACopy {
                copy: doc.spoken(copy),
                input,
            });
        }
        None => drawn,
    };
    copies
        .into_iter()
        .map(|copy| {
            let target = match doc.node(copy) {
                Some(Node::Place { target, .. }) => *target,
                _ => unreachable!("a drawn copy of a body read is a `Place`"),
            };
            Ok((target, combine::copy_motion(doc, eval, copy)?))
        })
        .collect()
}

/// **Whether `node`'s value is a placement's copy**: a placement or
/// an instance.
fn moves_copies(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> bool {
    matches!(
        doc.node(node),
        Some(Node::InstantiatePart { .. } | Node::Place { .. })
    )
}

/// Whether a document is assembly-shaped — one of the two conditions
/// [`AtRestBadge`] names for taking an at-rest badge: a document that
/// instantiates no part has no cross-instance rest for the badge to
/// answer about.
fn assembly_shaped(doc: &Doc<ProfileProgram>) -> bool {
    doc.ids()
        .iter()
        .filter_map(|&id| doc.node(id))
        .any(puts_an_instance)
}

/// Whether a node puts an instance of another document's part into
/// this one — what [`assembly_shaped`] asks of every node.
fn puts_an_instance(node: &Node<ProfileProgram>) -> bool {
    match node {
        Node::InstantiatePart { .. } => true,
        // Copies bodies this document already holds; its constraints
        // relate copies some other node put in the document.
        Node::Place { .. } => false,
        // Where copies stand; it puts nothing in.
        Node::World | Node::Space => false,
        Node::Profile(_)
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
        | Node::Mirror { .. }
        | Node::Subtract { .. }
        | Node::Union { .. }
        | Node::Intersect { .. }
        | Node::Assemble { .. }
        | Node::Measure { .. }
        | Node::Assertion { .. } => false,
    }
}

/// Exhaustive by destructuring; the shared rule is
/// `crates/viewer/README.md`'s. [`Derived`] renders as ONE field, so
/// its members travel with their declaration rather than being listed
/// a second time here.
///
/// The `_` arms, one reason each: `tol` is `Tol(())`, a ZST with
/// no content to print; `eval`, `door` and `stager` are `dyn` services
/// and implement no `Debug`; `requested_doc` is a whole recipe DAG;
/// `stages` is a count only `stage` reads; and `display` is
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
            door: _,
            stager: _,
            stages: _,
            generation,
            requested_doc: _,
            requested_resolver: _,
            derived,
            path,
            display: _,
            landed_withdrawals: _,
            inserts: _,
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

/// **An axis read as a mate's operand**: a plain read as it is, and a
/// read through a role (a frame's `z`) as the axis slid zero along
/// itself, which is that axis.
fn axis_operand(axis: RoleRead<Operand>) -> Operand {
    match axis.role {
        None => axis.read,
        Some(_) => Operand::Pose(Box::new(pncad::document::pose::PoseDef::Slide {
            pose: axis.clone(),
            along: axis,
            by: Formula::zero(Dimension::Length),
        })),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use super::{DocSession, Refusal};
    use pncad::document::{Doc, Node, ProfileProgram};
    use pncad::geom_core::Tol;

    /// **An action that swallows a refusal commits nothing.** The
    /// action lands a square profile, has an empty one refused (an
    /// empty loop list), ignores the refusal, lands a second square and
    /// answers success; the door still refuses the whole action, so
    /// neither square is committed. This reds if the run goes on past a
    /// refusal, which would commit the two squares around it.
    #[test]
    fn an_action_that_swallows_a_refusal_commits_nothing() {
        let tol = Tol::witness();
        let mut session = DocSession::inline(Doc::empty_derived("swallowed-refusal", tol), tol);
        let square = || crate::test_support::square(0.01);
        let before = session.history().len();
        let outcome = session.commit_run(|run| {
            run.insert(square())?;
            let _ = run.insert(Node::Profile(ProfileProgram {
                loops: Vec::new(),
                ids: Vec::new(),
                on: None,
            }));
            let _ = run.insert(square());
            Ok(())
        });
        assert!(
            matches!(outcome.refusal, Some(Refusal::Edit(_))),
            "the profile's refusal is the action's: {:?}",
            outcome.refusal
        );
        assert!(outcome.committed.is_empty(), "nothing is committed");
        assert_eq!(session.history().len(), before, "nothing is recorded");
        assert!(session.committed_doc().is_empty(), "neither square landed");
    }

    /// **The hide admission is asked of the document the op reads, not
    /// of the one the tree is drawn from**: an instance deleted since
    /// the last landing still has a tree row, and the answer for it is
    /// the refusal `SetBodyOn` gives.
    #[test]
    fn a_deleted_instance_not_yet_landed_is_refused_its_hide() {
        use pncad::document::{ContentPin, DocRef, DocumentId, ProfileDoc};

        use super::{DocSession, SessionOp};
        let tol = Tol::witness();
        let mut doc = ProfileDoc::empty(DocumentId::derive("hide-asm"), tol);
        let instance = crate::test_support::insert_into(
            &mut doc,
            Node::instantiate_part(
                DocRef {
                    id: DocumentId::derive("hide-part"),
                    pin: ContentPin::of_bytes(b"hide-part"),
                },
                pncad::document::Operand::Var(pncad::document::VarId::WORLD),
                pncad::document::stand().bundle(),
            ),
            tol,
        );
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        assert!(session.body_toggle_refusal(instance).is_none());
        let deleted = session.perform(SessionOp::DeleteNode { node: instance });
        assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
        assert!(
            session.tree_rows().iter().any(|row| row.id == instance),
            "the tree still draws the landed document's row"
        );
        let said = session
            .body_toggle_refusal(instance)
            .expect("the committed document holds no such node")
            .to_string();
        let got = session
            .perform(SessionOp::SetBodyOn {
                node: instance,
                on: false,
            })
            .refusal
            .expect("the op refuses a deleted instance")
            .to_string();
        assert_eq!(said, got);
        assert_eq!(
            said,
            format!(
                "node {} is not in the document",
                test_utils::refusal::tag(instance.0.digest())
            )
        );
    }
}
