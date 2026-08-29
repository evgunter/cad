//! The eframe application: docked chrome around the wgpu viewport,
//! with the document panels beside it.
//!
//! # What this module is allowed to contain
//!
//! Toolkit adaptation, and nothing else. It turns egui pointer state
//! into [`ViewportEvent`]s and into [`SessionOp`]s, hands both to the
//! renderer-free half of this crate, and paints what comes back. Every
//! decision it takes is a call into that half, which is what makes the
//! navigation and the editing testable without a window (G1) — and
//! what makes the seam reading in the PR body a statement about egui
//! rather than about our code.
//!
//! # Panes name operations; the application performs them
//!
//! A pane never mutates the session. It reads the session as a value
//! and pushes [`SessionOp`]s into a queue, which the application
//! drains after the layout has been walked. That is the toolkit-side
//! shape of `handle(event, ui_state) → (ui_state′, edits, overlay)`,
//! and it is also what makes the borrows work out: the layout needs a
//! shared view of everything and a mutable hold on nothing.
//!
//! # OQ-b: the docking crate is `egui_tiles`
//!
//! The layout is a `Tree<Pane>` the application **owns**: panes are
//! our enum, the tree is our field, and rendering is a `Behavior` impl
//! that reads it — the same discipline the rest of this crate runs on,
//! one level up.
//!
//! # Evaluation is not on this thread
//!
//! The application drives [`DocSession`] over a
//! [`crate::evalseam::ThreadEvaluator`]: edits submit
//! a document and the frame loop polls for results. The busy indicator
//! and the Cancel button are the two things that makes visible. What
//! this module knows about threads is one constructor call; everything
//! else is the seam's vocabulary, which the wasm build satisfies with
//! no thread at all.

use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use egui_tiles::{TileId, Tiles, Tree, UiResponse};
use pncad::document::{Axis3, Dimension, ParamName, RecipeNodeId, SlotId};
use pncad::geom_core::Tol;
use pncad::quantity::UnitDef;

use crate::camera::{self, Camera, CameraOp};
use crate::display::{DisplayView, free_move_check};
use crate::evalseam::Generation;
#[cfg(not(target_family = "wasm"))]
use crate::evalseam::ThreadEvaluator;
use crate::frame::{self, IdQueryLog, IdStep, StatusUpdate};
use crate::gpu::{DEPTH_BITS, IdQuery, ViewportCallback, ViewportRenderer};
use crate::input::{self, InputMap, PointerButton, ViewportEvent, ViewportSize};
use crate::matetool::{MateChoice, MateTool, MateToolState, admitted_classes};
use crate::pick::{self, PickCache, PickIndex};
use crate::props::{self, SlotDriver, SlotGroup, SlotRow, SlotValue};
use crate::scene::{self, DisplayTolerance, SceneMesh};
use crate::session::{BoundsTarget, DocSession, Refusal, Selection, SessionOp, Standing};
use crate::tree::{RowStatus, TreeRow};
use pncad::document::{AxisSense, Frame, MatePrimitive};

/// The window title.
const WINDOW_TITLE: &str = "viewer";

/// The starting display tolerance: 0.1 mm, fine enough that a 24 mm
/// hole reads as a circle and coarse enough to redraw instantly.
const INITIAL_DELTA: f64 = 1.0e-4;

/// The step the chrome's coarsen/refine buttons take.
const DELTA_STEP: f64 = 2.0;

/// Direction the light travels, world space; a unit vector over the
/// viewer's left shoulder.
const LIGHT_DIRECTION: [f32; 3] = [0.408_248_3, 0.408_248_3, -0.816_496_6];

/// The body's base colour, linear RGB — a neutral machined grey, so
/// shading reads as shape rather than as colour.
const BASE_COLOR: [f32; 3] = [0.62, 0.64, 0.67];

/// The document file extension the dialog filters on.
///
/// `cfg`-ed with the dialogs it filters for: the browser build links
/// no chooser, so the constant has no reader there and an
/// unconditional one would be dead code under CI's `-D warnings`.
#[cfg(not(target_family = "wasm"))]
const DOC_EXTENSION: &str = "pncad";

/// The colour an unresolved selection and a deleted feature are drawn
/// in — the same red the failed/poisoned badges use, because both say
/// "this does not denote anything".
const UNRESOLVED_COLOR: egui::Color32 = egui::Color32::from_rgb(210, 90, 70);

/// Points of indent per level of the feature tree.
const INDENT_STEP: f32 = 12.0;

/// The deepest level the tree indents for.
///
/// Depth is the longest input chain and a real document reaches far
/// past this — the tour's `diefillet` hits 27, which at
/// [`INDENT_STEP`] would be 324 points of dead space in a pane that
/// holds a third of the window. Past the clamp the rows stop moving
/// right; they stay in evaluation order, so the tree is still readable
/// as a sequence, and what is lost is depth information that had
/// already stopped fitting.
const INDENT_MAX_DEPTH: usize = 8;

/// The indent a row at `depth` draws at.
fn indent(depth: usize) -> f32 {
    depth.min(INDENT_MAX_DEPTH) as f32 * INDENT_STEP
}

/// One docked pane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pane {
    /// The 3D viewport.
    Viewport,
    /// The feature tree over the evaluation's result DAG.
    Features,
    /// The selected node's (or parameter's) properties.
    Properties,
    /// View settings: display δ and the camera's state.
    View,
}

/// Transient text a panel is mid-edit on.
///
/// Layer-3 state that never enters the document: an expression the
/// user is typing is not an edit until they commit it, and a draft
/// abandoned by selecting elsewhere leaves nothing behind.
#[derive(Debug, Default)]
struct Drafts {
    /// The slot the expression field is currently for.
    expr_target: Option<(RecipeNodeId, SlotId)>,
    /// What is typed in it.
    expr_text: String,
    /// The add-parameter form's name field.
    new_param_name: String,
    /// Its chosen dimension — `None` until the user picks one, and
    /// the Create button waits for the pick. The offer path lands
    /// here from an expression whose context does not determine the
    /// new parameter's dimension, and a silently-defaulted one would
    /// be a guess none of this program's doors make.
    new_param_dimension: Option<Dimension>,
    /// Its value field.
    new_param_value: f64,
    /// The name an unknown-parameter refusal offered to create
    /// ([`frame::creation_offer`]); shown over the form while the
    /// name field still says it.
    new_param_offer: Option<ParamName>,
    /// The mate tool's class/alignment choice, as widget state: an
    /// index into [`admitted_classes`], an index into
    /// [`MATE_PRIMITIVES`], and the sense toggle. Draft chrome state
    /// only — the typed choice is minted at commit.
    mate_class: usize,
    mate_primitive: usize,
    mate_opposed: bool,
}

/// The primitives the chrome offers, with their labels. The op
/// vocabulary accepts any [`MatePrimitive`]; these are the three the
/// panel can spell without a numeric field (`PlanarRest`'s offset is
/// authored 0 — a flush rest; a standoff is typed through the tree's
/// ordinary property doors once the node exists).
const MATE_PRIMITIVES: [(MatePrimitive, &str); 3] = [
    (MatePrimitive::FrameCoincidence, "frame coincidence"),
    (MatePrimitive::Coaxial, "coaxial"),
    (MatePrimitive::PlanarRest { offset: 0.0 }, "planar rest"),
];

/// Everything the application knows.
pub struct ViewerApp {
    session: DocSession,
    delta: DisplayTolerance,
    scene: Arc<SceneMesh>,
    /// What the cursor picks against, and the rebuild-on-stale loop
    /// that keeps it current — one attempt per (generation, δ), so a
    /// document with a failed root does not re-tessellate every
    /// healthy root on every repainted frame.
    picks: PickCache,
    /// Where the id pass leaves its answer: `serial << 32 | id`.
    id_answer: Arc<AtomicU64>,
    /// Which id query is outstanding and what it was asked about.
    id_log: IdQueryLog,
    /// Bumped on every rebuild; the GPU uploads when it disagrees.
    revision: u64,
    /// The evaluation generation `scene` was built from. When it
    /// disagrees with the session's landed generation, the picture is
    /// out of date and exactly one rebuild is owed.
    scene_generation: Option<Generation>,
    /// The display-state revision `scene` was built under — hide and
    /// free-move are scene inputs too, so a display change owes a
    /// rebuild exactly as a new evaluation does.
    scene_display: Option<u64>,
    /// The focus set `scene` was built under — the ids of what the side
    /// panel is showing (`pick::focus`), which the scene carries as a
    /// per-corner flag and therefore has to be rebuilt for.
    ///
    /// Compared as a SET rather than counted by a revision, because
    /// unlike hide and free-move the focus is DERIVED (from the
    /// selection and the index), so there is no mutation to hang a
    /// counter off and no owner to bump one. Moving the selection
    /// between two faces of the same feature leaves the set equal and
    /// correctly rebuilds nothing.
    scene_focus: BTreeSet<u32>,
    /// The modal mate tool, when active. `None` is "not in the mate
    /// tool"; the tool's own state (the held picks) lives inside it.
    mate_tool: Option<MateTool>,
    camera: Camera,
    input: InputMap,
    tree: Tree<Pane>,
    drafts: Drafts,
    /// A fit is owed, and will be taken by the viewport pane on the
    /// next frame — the only place that knows the real aspect.
    pending_fit: bool,
    /// A fit is owed as soon as the NEXT rebuilt scene lands — set by
    /// a successful `Open`, whose document arrives asynchronously, so
    /// fitting immediately would frame the outgoing picture.
    fit_on_scene: bool,
    /// The last thing that went wrong, kept so a refused operation is
    /// visible instead of silently dropped.
    status: Option<String>,
    /// Whether the environment can show a file dialog at all — probed
    /// once at startup ([`frame::chooser_backend`]); the Open/Save As
    /// controls read it every frame.
    chooser: frame::ChooserBackend,
}

/// Why the application could not start (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum StartupError {
    /// The document could not be authored.
    Document(scene::SceneDocError),
    /// The document did not produce a drawable scene.
    Scene(scene::SceneError),
    /// The starting camera could not be framed on the scene.
    Camera(camera::CameraError),
    /// `eframe` handed the application no wgpu render state — the
    /// application was built against a renderer it does not have.
    NoWgpuRenderState,
    /// The evaluation worker could not be started. Fatal on purpose: a
    /// seam with no worker accepts every submit and answers none, so
    /// the application would open onto a permanent "evaluating…".
    ///
    /// Absent on wasm, where the seam is [`crate::evalseam::InlineEvaluator`]
    /// — nothing is spawned, so nothing can refuse to spawn. The arm
    /// is `cfg`-ed away rather than kept and never constructed,
    /// because a closed enum (D4 ¶3) whose reader must ask which arms
    /// are reachable is no longer telling the truth about its states.
    #[cfg(not(target_family = "wasm"))]
    Evaluator(crate::evalseam::SpawnError),
}

impl core::fmt::Display for StartupError {
    /// Every payload arm forwards to the refusing layer's own
    /// `Display`; the prefix is only which startup step was standing
    /// when it refused.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Document(error) => {
                write!(f, "the starting document could not be authored: {error}")
            }
            Self::Scene(error) => {
                write!(f, "the starting document draws no scene: {error}")
            }
            Self::Camera(error) => {
                write!(
                    f,
                    "no camera could be framed on the starting scene: {error}"
                )
            }
            Self::NoWgpuRenderState => f.write_str(
                "eframe handed the viewer no wgpu render state: this build was linked \
                 against a renderer it does not have",
            ),
            #[cfg(not(target_family = "wasm"))]
            Self::Evaluator(error) => write!(f, "{error}"),
        }
    }
}

impl core::error::Error for StartupError {}

/// The evaluation seam this build gets, and the ONE place the two
/// platforms differ about it.
///
/// `evalseam` already carried both arms — [`ThreadEvaluator`] behind
/// `cfg(not(target_family = "wasm"))` and
/// [`crate::evalseam::InlineEvaluator`] unconditionally, both
/// implementing one `EvalService` — because GUI-PLAN's platform
/// section made "the interaction layer never assumes threads" a
/// constraint on every GUI unit. This function is that constraint
/// finally being *spent*: the browser takes the inline arm, and
/// nothing else in the application knows which it got.
///
/// **What the inline arm costs, stated rather than discovered.**
/// It evaluates on the calling thread, so a rebuild blocks the frame
/// that submitted it and the browser tab stops painting until the
/// kernel returns. The busy indicator cannot help — it would need an
/// in-op yield point, and GUI-PLAN rules those absent for v1. This is
/// the honest price of not taking the threaded lane (GUI-5's pinned
/// nightly + `wasm-bindgen-rayon` + cross-origin isolation), and it
/// is a spike's price to pay.
///
/// # Errors
///
/// [`StartupError::Evaluator`] if the OS refuses the worker thread.
/// The wasm arm is infallible — it spawns nothing — and returns
/// `Ok` unconditionally.
fn evaluator() -> Result<Box<dyn crate::evalseam::EvalService>, StartupError> {
    #[cfg(not(target_family = "wasm"))]
    {
        Ok(Box::new(
            ThreadEvaluator::spawn().map_err(StartupError::Evaluator)?,
        ))
    }
    #[cfg(target_family = "wasm")]
    {
        Ok(Box::new(crate::evalseam::InlineEvaluator::new()))
    }
}

impl ViewerApp {
    /// Build the application: author the starting document, evaluate
    /// it, tessellate at the initial δ, frame a camera on the result,
    /// and install the viewport pipeline into the render state.
    ///
    /// # Errors
    ///
    /// Every arm of [`StartupError`]. Startup refuses loudly rather
    /// than opening an empty window — an empty viewport is the one
    /// failure mode a user cannot diagnose.
    pub fn new(cc: &eframe::CreationContext<'_>, tol: Tol) -> Result<Self, StartupError> {
        let delta = DisplayTolerance::new(INITIAL_DELTA).map_err(StartupError::Scene)?;
        let (document, _root) = scene::plate_with_hole(tol).map_err(StartupError::Document)?;
        let mesh = scene::scene_of(&document, delta, tol).map_err(StartupError::Scene)?;
        // A provisional camera at a square aspect, because no pane has
        // been laid out yet and there is no real aspect to read. It is
        // provisional for exactly one frame: `pending_fit` below makes
        // the viewport re-frame at its true aspect the first time it
        // runs, so what a user sees is never the invented framing.
        let camera = Camera::framing(&mesh.bounds(), 1.0).map_err(StartupError::Camera)?;

        let render_state = cc
            .wgpu_render_state
            .as_ref()
            .ok_or(StartupError::NoWgpuRenderState)?;
        let renderer = ViewportRenderer::new(&render_state.device, render_state.target_format);
        render_state
            .renderer
            .write()
            .callback_resources
            .insert(renderer);

        Ok(Self {
            session: DocSession::new(document, tol, evaluator()?),
            delta,
            scene: Arc::new(mesh),
            picks: PickCache::new(),
            id_answer: Arc::new(AtomicU64::new(0)),
            id_log: IdQueryLog::new(),
            revision: 1,
            scene_generation: None,
            scene_display: None,
            scene_focus: BTreeSet::new(),
            mate_tool: None,
            camera,
            input: InputMap::default(),
            tree: initial_layout(),
            drafts: Drafts::default(),
            pending_fit: true,
            fit_on_scene: false,
            status: None,
            chooser: frame::chooser_backend(),
        })
    }

    /// Take whatever the seam finished and, if the picture is behind
    /// the document, rebuild it.
    ///
    /// The one place a document becomes a scene. Gated on the
    /// generation so a frame that changed nothing re-tessellates
    /// nothing.
    fn sync_scene(&mut self) {
        self.session.pump();
        // The mate tool's survival step: re-read the held picks
        // against the landed pair, reporting each typed drop.
        if let (Some(tool), Some((doc, eval))) =
            (self.mate_tool.as_mut(), self.session.landed_pair())
        {
            for event in tool.reconcile(doc, eval) {
                self.status = Some(format!("mate tool: {event}"));
            }
        }
        // The cache owns the retry policy: one attempt per (landed
        // generation, δ). A refused build is reported and held, not
        // re-attempted every frame behind a stale picture.
        let rebuilt = match self.picks.sync(&self.session, self.delta) {
            pick::CacheStep::Held | pick::CacheStep::Nothing => return,
            pick::CacheStep::Refused => {
                self.status = self
                    .picks
                    .error()
                    .map(|error| format!("pick index: {error}"));
                return;
            }
            pick::CacheStep::Rebuilt => true,
            pick::CacheStep::Current => false,
        };
        // The scene is a function of (index, display state, focus): a
        // display or selection change over a current index still owes
        // exactly one rebuild.
        let display_revision = self.session.display().revision();
        let Some(index) = self.picks.index() else {
            return;
        };
        let focus = pick::focus(index, self.session.doc(), self.session.selection());
        if !rebuilt && self.scene_display == Some(display_revision) && self.scene_focus == focus {
            return;
        }
        match index.scene_focused(&self.session.display_view(), &focus) {
            Ok(mesh) => {
                // Marked current ONLY on success: a refused build must
                // not consume this (generation, display) pair, or the
                // stale picture stays on screen marked as the current
                // one and is never retried.
                self.scene_generation = self.session.landed_generation();
                self.scene_display = Some(display_revision);
                self.scene_focus = focus;
                self.scene = Arc::new(mesh);
                self.revision = self.revision.wrapping_add(1);
                if self.fit_on_scene {
                    self.fit_on_scene = false;
                    self.pending_fit = true;
                }
                // The gather's own verdict, computed once when the
                // evaluation landed. A naming collision across roots is
                // not a node failure, so no tree badge carries it and
                // the viewport would otherwise draw a product nothing
                // says is malformed.
                //
                // The budget's verdict is NOT here, and the reason is
                // worth writing down: this line is transient — a
                // camera fold clears it through `land`, which every
                // re-frame performs, including the one an Open books —
                // and a coarsened δ is not transient, it is a standing
                // fact about the picture on screen. It is a BADGE, up
                // with the at-rest and checks reads.
                self.status = self
                    .session
                    .product_fault()
                    .map(|fault| format!("product: {fault}"));
            }
            Err(error) => self.status = Some(format!("scene: {error}")),
        }
    }

    /// Change δ, rebuilding the picture from the evaluation already in
    /// hand — a display change is not a document change and re-runs no
    /// geometry above the tessellator.
    fn rescale_delta(&mut self, factor: f64) {
        match self.delta.scaled(factor) {
            Ok(delta) => {
                self.delta = delta;
                self.sync_scene();
            }
            Err(error) => self.status = Some(format!("{error}")),
        }
    }

    /// Perform one operation and record what it refused.
    /// Perform one frame's whole batch of operations, keeping the
    /// refusal worth showing.
    ///
    /// **Not one assignment per op.** A frame queues several ops and
    /// several of them can refuse; assigning `status` from each in turn
    /// keeps the LAST, which is how dragging a driven slot came to
    /// display `NoGesture` instead of the ratified affordance —
    /// `BeginGesture` refuses with the affordance and the same frame's
    /// `PreviewGesture` refuses `NoGesture` on top of it. `Refusal`
    /// ranks itself; this keeps the best-ranked, first-seen one, and
    /// clears the line only when a batch refuses nothing at all.
    fn perform_batch(&mut self, ops: Vec<SessionOp>) {
        if ops.is_empty() {
            return;
        }
        let mut refusal: Option<Refusal> = None;
        // The VERDICTS are `frame`'s, computed from the ops and the
        // refusal; this loop only performs and collects. The rules
        // used to live inline here, in app-gated code no row could
        // reach — see `frame`'s module docs.
        let mut performed: Vec<SessionOp> = Vec::with_capacity(ops.len());
        for op in ops {
            performed.push(op.clone());
            let opened = matches!(op, SessionOp::Open(_));
            match self.session.perform(op).refusal {
                Some(next) => refusal = Refusal::preferred(refusal, next),
                // A replaced document owes a re-frame — taken when
                // its scene actually lands, not on the outgoing
                // picture.
                None if opened => self.fit_on_scene = true,
                None => {}
            }
        }
        let update = frame::batch_status(&performed, refusal.as_ref());
        // The refuse-then-offer pair for a parse refusal: restore the
        // refused draft so acting on the refusal does not cost the
        // text that raised it, and — for an unknown parameter name —
        // prefill the add-parameter affordance with the name it
        // offers to create (dimension deliberately left unpicked).
        if let Some((node, slot, text)) = frame::retype_draft(&performed, refusal.as_ref()) {
            self.drafts.expr_target = Some((node, slot));
            self.drafts.expr_text = text;
        }
        if let Some(name) = frame::creation_offer(refusal.as_ref()) {
            self.drafts.new_param_name = name.0.clone();
            self.drafts.new_param_dimension = None;
            self.drafts.new_param_offer = Some(name.clone());
        }
        self.apply_status(update);
    }

    /// Apply a policy verdict to the status line — the one place a
    /// [`StatusUpdate`] becomes the field, shared by the batch policy
    /// and the dialog policy so neither hand-assigns.
    fn apply_status(&mut self, update: StatusUpdate) {
        match update {
            StatusUpdate::Keep => {}
            StatusUpdate::Clear => self.status = None,
            StatusUpdate::Show(message) => self.status = Some(message),
        }
    }
}

impl eframe::App for ViewerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.sync_scene();
        let mut ops: Vec<SessionOp> = Vec::new();

        egui::Panel::top("viewer_toolbar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(WINDOW_TITLE);
                ui.separator();
                // The chooser-backend verdict, probed once at startup:
                // with confidently NO backend (no zenity, no session
                // bus) the dialogs are disabled UP FRONT with the
                // reason as their tooltip — a dead click is exactly
                // the silent failure #1097 reported. Under a
                // plausibly-present backend, a dialog handing back
                // `None` is read as a genuine cancel and stays quiet;
                // `frame::dialog_status` is that rule as a policy
                // value, and its loud arm is the belt to this
                // disabling's braces.
                let chooser = self.chooser;
                if ui
                    .add_enabled(chooser.usable(), egui::Button::new("Open…"))
                    .on_disabled_hover_text(frame::NO_CHOOSER_BACKEND)
                    .clicked()
                {
                    // Unreachable on wasm — `chooser` is `Absent`
                    // there, so the button is disabled and never
                    // reports a click — but unreachable code still has
                    // to compile, and `pick_open` does not exist on
                    // that target. The `cfg` is on the BODY rather
                    // than the button so the browser build still shows
                    // the control and its disabled reason, which is
                    // the #1125 posture: a door that cannot open says
                    // so, it does not vanish.
                    #[cfg(not(target_family = "wasm"))]
                    {
                        let path = pick_open();
                        let update = frame::dialog_status(chooser, path.is_some());
                        if let Some(path) = path {
                            ops.push(SessionOp::Open(path));
                        }
                        self.apply_status(update);
                    }
                }
                if ui
                    .add_enabled(chooser.usable(), egui::Button::new("Save As…"))
                    .on_disabled_hover_text(frame::NO_CHOOSER_BACKEND)
                    .clicked()
                {
                    // Unreachable on wasm, for the reason the Open…
                    // arm above states.
                    #[cfg(not(target_family = "wasm"))]
                    {
                        let path = pick_save(self.session.path());
                        let update = frame::dialog_status(chooser, path.is_some());
                        if let Some(path) = path {
                            ops.push(SessionOp::Save(path));
                        }
                        self.apply_status(update);
                    }
                }
                ui.separator();
                if ui
                    .add_enabled(self.session.history().can_undo(), egui::Button::new("Undo"))
                    .clicked()
                {
                    ops.push(SessionOp::Undo);
                }
                if ui
                    .add_enabled(self.session.history().can_redo(), egui::Button::new("Redo"))
                    .clicked()
                {
                    ops.push(SessionOp::Redo);
                }
                ui.separator();
                if ui.button("Fit").clicked() {
                    // The toolbar has no pane rectangle, so it asks
                    // for a fit rather than performing one; the
                    // viewport takes it at the real aspect.
                    self.pending_fit = true;
                }
                if ui.button("Coarser δ").clicked() {
                    self.rescale_delta(DELTA_STEP);
                }
                if ui.button("Finer δ").clicked() {
                    self.rescale_delta(1.0 / DELTA_STEP);
                }
                // The indicator is a READ of session state, and the
                // buttons beside it are the shipped token and its pair.
                // Neither knows whether a thread is involved.
                //
                // THREE states, not two, because a cancel leaves a
                // fourth thing to say: the picture is older than the
                // document AND nothing is running. A spinner there
                // would be a lie about work nobody is doing.
                if self.session.busy() {
                    ui.separator();
                    if self.session.running() {
                        ui.spinner();
                        ui.label("evaluating…");
                        if ui.button("Cancel").clicked() {
                            ops.push(SessionOp::CancelEvaluation);
                        }
                        // A background result is not a user event, so
                        // nothing else would wake the frame loop to
                        // collect it.
                        ui.ctx().request_repaint();
                    } else {
                        ui.label("canceled — showing an older result");
                        if ui.button("Re-evaluate").clicked() {
                            ops.push(SessionOp::Reevaluate);
                        }
                    }
                }
                // The A5 at-rest badge, for assembly-shaped documents:
                // the verification verdict living past the commit.
                match self.session.at_rest() {
                    Some(crate::session::AtRestBadge::Certified { minted }) => {
                        ui.separator();
                        ui.weak(format!("at rest: certified ({minted} declaration(s))"));
                    }
                    Some(crate::session::AtRestBadge::Refused { message }) => {
                        ui.separator();
                        ui.colored_label(UNRESOLVED_COLOR, format!("at rest: {message}"));
                    }
                    None => {}
                }
                // The advisory-check badge. It REPORTS: the scene below
                // is drawn either way, because a product whose roots
                // interpenetrate renders a picture that looks almost
                // right and the finding is the only thing that says
                // otherwise. Hover for the findings' own sentences —
                // each carries its recourse, so the badge never
                // composes one here.
                if let Some(report) = self.session.checks()
                    && !report.findings.is_empty()
                {
                    ui.separator();
                    ui.colored_label(
                        UNRESOLVED_COLOR,
                        format!("checks: {} finding(s)", report.findings.len()),
                    )
                    .on_hover_ui(|ui| {
                        for finding in &report.findings {
                            ui.label(finding.to_string());
                        }
                    });
                }
                // The display budget's badge: shown only when the
                // picture on screen is NOT at the δ that was asked
                // for. A read of held state, like the two badges
                // above, which is why it is here rather than in the
                // status line below — that line is cleared by the next
                // camera fold, and "you are not looking at what you
                // asked for" has to outlive a mouse drag.
                if let Some(fitted) = self.picks.fitted()
                    && let Some(wording) = fitted.wording()
                {
                    ui.separator();
                    ui.weak(format!("δ {:.3} mm drawn", fitted.delta.get() * 1.0e3))
                        .on_hover_text(wording);
                }
                if let Some(status) = &self.status {
                    ui.separator();
                    ui.label(status.as_str());
                }
            });
        });

        let display = self.session.display_view();
        egui::CentralPanel::no_frame().show(ui, |ui| {
            let mut behavior = ViewerBehavior {
                session: &self.session,
                delta: self.delta,
                fitted: self.picks.fitted(),
                scene: &self.scene,
                index: self.picks.index(),
                revision: self.revision,
                camera: &mut self.camera,
                input: self.input,
                drafts: &mut self.drafts,
                display: &display,
                mate_tool: &mut self.mate_tool,
                pending_fit: &mut self.pending_fit,
                status: &mut self.status,
                id_answer: &self.id_answer,
                id_log: &mut self.id_log,
                ops: &mut ops,
            };
            self.tree.ui(&mut behavior, ui);
        });

        // The mate tool consumes the selection vocabulary: while the
        // tool is active, a face pick this frame produced is ALSO held
        // as a tool pick (the two-sequential-picks ruling — the same
        // single-select value, copied into tool state).
        if let Some(tool) = self.mate_tool.as_mut() {
            for op in &ops {
                if let SessionOp::Select(Selection::Face(face)) = op {
                    tool.pick(face.clone());
                }
            }
        }

        self.perform_batch(ops);
    }
}

/// The `Behavior` egui_tiles renders panes through: a borrow of the
/// application's state for the duration of one frame.
struct ViewerBehavior<'a> {
    session: &'a DocSession,
    /// The δ that was ASKED for. What is drawn may be coarser — see
    /// `fitted`, and `scene::TRIANGLE_BUDGET` for why.
    delta: DisplayTolerance,
    /// What the triangle budget made of `delta` on the held index.
    fitted: Option<crate::scene::FittedDelta>,
    scene: &'a Arc<SceneMesh>,
    index: Option<&'a PickIndex>,
    revision: u64,
    camera: &'a mut Camera,
    input: InputMap,
    drafts: &'a mut Drafts,
    /// The display snapshot this frame draws and picks under.
    display: &'a DisplayView,
    /// The modal mate tool, if active.
    mate_tool: &'a mut Option<MateTool>,
    pending_fit: &'a mut bool,
    status: &'a mut Option<String>,
    id_answer: &'a Arc<AtomicU64>,
    id_log: &'a mut IdQueryLog,
    ops: &'a mut Vec<SessionOp>,
}

/// Land a fold: take the camera it reached, and either show the
/// refusal that stopped it or clear the last one.
///
/// **The one place a camera move becomes application state.** Both the
/// toolbar's single operations and the viewport's event stream come
/// through here, so "record the refusal, clear it on success" has one
/// implementation.
fn land(camera: &mut Camera, status: &mut Option<String>, folded: &camera::Folded) {
    *camera = folded.camera;
    *status = folded
        .refused
        .as_ref()
        .map(|(op, error)| format!("camera: {error} (from {op})"));
}

impl egui_tiles::Behavior<Pane> for ViewerBehavior<'_> {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match pane {
            Pane::Viewport => "Viewport".into(),
            Pane::Features => "Features".into(),
            Pane::Properties => "Properties".into(),
            Pane::View => "View".into(),
        }
    }

    fn pane_ui(&mut self, ui: &mut egui::Ui, tile_id: TileId, pane: &mut Pane) -> UiResponse {
        // The viewport IS its rectangle: it allocates exactly the
        // available size and paints into it, so a scroll container
        // around it would have nothing true to say.
        if *pane == Pane::Viewport {
            self.viewport_ui(ui);
            return UiResponse::None;
        }
        // Every CHROME pane scrolls its own overflow — the class of
        // panes, not the one that happened to clip. First light
        // (#1097): the Properties pane's lower content was unreachable
        // at any window height, clipped with no scrollbar. auto_shrink
        // is off on both axes so the pane fills its tile (a scrollbar
        // at the tile's edge, no collapse under short content); the
        // salt is the tile id, so two tabs of one tile scroll
        // independently.
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .id_salt(tile_id)
            .show(ui, |ui| match pane {
                // Handled above; this arm cannot be reached.
                Pane::Viewport => {}
                Pane::Features => self.features_ui(ui),
                Pane::Properties => self.properties_ui(ui),
                Pane::View => self.view_ui(ui),
            });
        UiResponse::None
    }
}

impl ViewerBehavior<'_> {
    /// The viewport pane: read the pointer, fold it into camera
    /// operations, then queue the paint callback.
    fn viewport_ui(&mut self, ui: &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        let pixels_per_point = f64::from(ui.ctx().pixels_per_point());
        let viewport = ViewportSize {
            width_px: f64::from(rect.width()) * pixels_per_point,
            height_px: f64::from(rect.height()) * pixels_per_point,
        };
        let Some(aspect) = viewport.aspect() else {
            return;
        };

        let (shift, alt) = ui.input(|i| (i.modifiers.shift, i.modifiers.alt));
        let mut events: Vec<ViewportEvent> = Vec::new();
        for (egui_button, button) in [
            (egui::PointerButton::Primary, PointerButton::Primary),
            (egui::PointerButton::Secondary, PointerButton::Secondary),
            (egui::PointerButton::Middle, PointerButton::Middle),
        ] {
            if response.dragged_by(egui_button) {
                let delta = response.drag_delta();
                events.push(ViewportEvent::Drag {
                    button,
                    shift,
                    alt,
                    delta_px: [
                        f64::from(delta.x) * pixels_per_point,
                        f64::from(delta.y) * pixels_per_point,
                    ],
                });
            }
        }
        if response.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                // egui reports scroll in points; a wheel notch is
                // conventionally 50 of them.
                events.push(ViewportEvent::Scroll {
                    units: f64::from(scroll) / 50.0,
                });
            }
        }
        // Cursor events, in the same stream. `hover_pos` is in screen
        // POINTS; the viewport speaks physical pixels from the pane's
        // own top-left corner, so the two conversions happen here and
        // the mapping below sees one convention.
        let cursor_px = response.hover_pos().map(|pos| {
            [
                f64::from(pos.x - rect.min.x) * pixels_per_point,
                f64::from(pos.y - rect.min.y) * pixels_per_point,
            ]
        });
        match cursor_px {
            Some(pos_px) => {
                if response.clicked_by(egui::PointerButton::Primary) {
                    events.push(ViewportEvent::Click {
                        button: PointerButton::Primary,
                        pos_px,
                    });
                }
                events.push(ViewportEvent::Hover { pos_px });
            }
            // Only when there is a hover to clear — the session's own
            // state answers that, so nothing here shadows it.
            None if self.session.hover().is_some() => events.push(ViewportEvent::Leave),
            None => {}
        }
        // An owed fit is taken here and nowhere else: this is the only
        // place with a real aspect to fit against.
        if *self.pending_fit {
            *self.pending_fit = false;
            let fit = CameraOp::Frame {
                bounds: self.scene.bounds(),
                aspect,
            };
            let folded = camera::fold_recorded(self.camera, std::slice::from_ref(&fit));
            land(self.camera, self.status, &folded);
        }

        // ONE fold, the same one `map_stream` gives the tests.
        //
        // Landed only when the fold actually MOVED something: the
        // stream now carries cursor events too, and a stream that
        // denotes no camera operation is not a camera event — landing
        // it would clear the status line on every frame the pointer is
        // inside the viewport.
        let folded = input::fold_events(&self.input, self.camera, viewport, &events);
        if frame::folded_moved(&folded) {
            land(self.camera, self.status, &folded);
        }

        // **One movement verdict for both picking paths.** The id
        // query's bookkeeping answers "has anything changed under this
        // cursor since the last question", and the CPU ray obeys the
        // same answer: a still cursor over an unchanged picture is a
        // ray cast whose result is already known. Without it an orbit
        // drag ran a full ray cast AND a blocking GPU readback on every
        // frame, because the app pushes a `Hover` whenever the pointer
        // is inside the pane — true of every frame of a drag.
        let generation = self.index.map(PickIndex::generation);
        let step = self.id_log.step(cursor_px, generation);

        // The cursor path: actions in, session operations out. Every
        // step of it — the un-projection, the ray service, the miss
        // rule — lives in `pick::PickIndex::op_for`, so this is the
        // same path a headless test drives.
        let actions = input::pick_stream(&self.input, &events);
        if let (Some(index), Some(eval)) = (self.index, self.session.evaluation()) {
            for action in actions {
                // A hover over an unchanged picture at an unmoved
                // cursor asks a question whose answer the session
                // already holds. A click never skips: it is an
                // ACTION, not an observation.
                if step == IdStep::Hold && matches!(action, input::PickAction::Hover(_)) {
                    continue;
                }
                match index.op_under(eval, self.camera, viewport, action, self.display) {
                    // A hover that changes nothing is not queued: an
                    // operation per frame that performs no transition
                    // is churn in the one log a test reads.
                    Ok(SessionOp::Hover(face)) if face.as_ref() == self.session.hover() => {}
                    Ok(op) => self.ops.push(op),
                    Err(error) => *self.status = Some(error.to_string()),
                }
            }
        }

        // What to mark, as a pure function of what is drawn and what is
        // selected. Recomputed every frame; nothing retains it.
        let highlight = self
            .index
            .map(|index| pick::highlight(index, self.session.selection(), self.session.hover()));

        let matrix = match self.camera.view_projection(aspect) {
            Ok(matrix) => matrix,
            Err(error) => {
                *self.status = Some(format!("projection: {error}"));
                return;
            }
        };

        // The two paths' agreement, compared BY NAME (`frame::
        // disagreement` says why ids are the wrong currency, and
        // records the ray-authoritative role inversion against
        // GQ6-RESURVEY §3). Reported, never resolved.
        if let Some(report) = self.index.and_then(|index| {
            frame::disagreement(
                index,
                self.id_answer.load(Ordering::Relaxed),
                self.id_log.outstanding(),
                self.session.hover().map(|face| &face.name),
            )
        }) {
            *self.status = Some(report.to_string());
        }

        let id_query = match (step, cursor_px) {
            (IdStep::Ask { serial }, Some(cursor)) => {
                viewport.ndc_of(cursor).map(|[nx, ny]| IdQuery {
                    cursor_ndc: [nx as f32, ny as f32],
                    viewport_px: [viewport.width_px as f32, viewport.height_px as f32],
                    serial,
                    answer: Arc::clone(self.id_answer),
                })
            }
            _ => None,
        };

        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            ViewportCallback {
                scene: Arc::clone(self.scene),
                revision: self.revision,
                view_projection: to_f32(&matrix),
                light_direction: LIGHT_DIRECTION,
                base_color: BASE_COLOR,
                highlight: highlight.unwrap_or_default(),
                id_query,
            },
        ));
    }

    /// The feature tree: one row per recipe node, with its status
    /// badge from the evaluation's typed result.
    fn features_ui(&mut self, ui: &mut egui::Ui) {
        let rows = self.session.tree_rows();
        ui.horizontal(|ui| {
            ui.heading("Features");
            ui.label(format!("{} nodes", rows.len()));
        });
        ui.separator();
        // A face picked in the viewport highlights its owning
        // feature here — one selection value, one inversion. Overflow
        // is `pane_ui`'s scroll container's job, the same one every
        // chrome pane sits in; a second ScrollArea here would nest.
        let selected = self.session.selection().node();
        for row in &rows {
            self.feature_row(ui, row, selected == Some(row.id));
        }
    }

    /// One feature-tree row.
    fn feature_row(&mut self, ui: &mut egui::Ui, row: &TreeRow, selected: bool) {
        ui.horizontal(|ui| {
            ui.add_space(indent(row.depth));
            let label = if row.root {
                format!("{} ▸", row.kind)
            } else {
                row.kind.to_owned()
            };
            if ui.selectable_label(selected, label).clicked() {
                self.ops.push(SessionOp::Select(Selection::Node(row.id)));
            }
            // The hide toggle, on instance rows only: a hidden
            // instance stays IN this tree (that is the point — the
            // tree is the document, the viewport is the display), and
            // the checkbox is the display op's chrome.
            if row.kind == "InstantiatePart" {
                let mut shown = !self.display.hidden.contains(&row.id);
                if ui.checkbox(&mut shown, "shown").changed() {
                    self.ops.push(SessionOp::SetInstanceHidden {
                        instance: row.id,
                        hidden: !shown,
                    });
                }
            }
            match &row.status {
                RowStatus::Ok => {}
                RowStatus::Unevaluated => {
                    ui.weak(row.status.badge());
                }
                RowStatus::Failed { .. } | RowStatus::Poisoned { .. } => {
                    ui.colored_label(UNRESOLVED_COLOR, row.status.badge());
                }
            }
        });
        // The typed payload's own message, indented under the row it
        // belongs to. Never a sentence this module wrote.
        if let Some(message) = row.status.message() {
            ui.horizontal(|ui| {
                ui.add_space(indent(row.depth) + INDENT_STEP);
                ui.weak(message);
            });
        }
        // The node's standing caveat (a mate class with no at-rest
        // record) — the admission verdict, outliving the commit.
        if let Some(note) = &row.note {
            ui.horizontal(|ui| {
                ui.add_space(indent(row.depth) + INDENT_STEP);
                ui.weak(note);
            });
        }
    }

    /// The property panel.
    fn properties_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Properties");
        ui.separator();
        self.mate_tool_ui(ui);
        let standing = self.session.standing();
        self.standing_ui(ui, &standing);
        if let Some(node) = self.session.selection().node() {
            self.instance_ui(ui, node);
        }
        match self.session.selection().clone() {
            Selection::None => {
                ui.weak("select a feature");
            }
            Selection::Node(node) => {
                let groups = self.session.slot_groups();
                if groups.is_empty() {
                    ui.weak("this feature carries no parameters");
                }
                for group in &groups {
                    self.slot_group_ui(ui, node, group);
                }
            }
            Selection::Face(face) => {
                // Slot rows for the OWNING node: a face pick is a way
                // of reaching the feature that made it, which is what
                // G3's click-to-select is for.
                let groups = self.session.slot_groups();
                if groups.is_empty() && standing.live() {
                    ui.weak("this feature carries no parameters");
                }
                for group in &groups {
                    self.slot_group_ui(ui, face.node, group);
                }
            }
            Selection::Param(name) => {
                if let Some(row) = crate::props::param_rows(self.session.doc())
                    .into_iter()
                    .find(|row| row.name == name)
                {
                    ui.label(format!("parameter {} ({:?})", row.name.0, row.dimension));
                    let mut value = row.value.as_f64();
                    let widget = ui.add(egui::DragValue::new(&mut value).speed(
                        if row.dimension == Dimension::Count {
                            1.0
                        } else {
                            0.0005
                        },
                    ));
                    drag_ops(
                        &widget,
                        value,
                        SessionOp::BeginParamGesture { name: name.clone() },
                        |value| SessionOp::PreviewGesture { value },
                        SessionOp::CommitGesture,
                        |value| {
                            vec![SessionOp::SetParam {
                                name: name.clone(),
                                value: SlotValue::of(row.dimension, value),
                            }]
                        },
                        self.ops,
                    );
                    self.param_bounds_ui(ui, &name, row.dimension);
                } else {
                    ui.weak("that parameter is gone");
                }
            }
        }
        ui.separator();
        ui.label("document parameters");
        for row in crate::props::param_rows(self.session.doc()) {
            if ui.link(row.name.0.clone()).clicked() {
                self.ops
                    .push(SessionOp::Select(Selection::Param(row.name.clone())));
            }
        }
        self.add_param_ui(ui);
    }

    /// The create half of the document-parameters section: name,
    /// dimension, value, one [`SessionOp::CreateParam`] on commit.
    ///
    /// Two deliberate frictions, both refusals-in-advance. The
    /// dimension starts UNPICKED and Create waits for it — the offer
    /// path arrives here from an expression whose context does not
    /// determine the new parameter's dimension, and a silent default
    /// would be a guess. And a name that is already declared shows the
    /// session's own already-exists sentence with the edit door
    /// offered, before the click ever reaches the typed refusal
    /// backing it ([`Refusal::ParamExists`]).
    fn add_param_ui(&mut self, ui: &mut egui::Ui) {
        // The offer from an unknown-parameter parse refusal, shown
        // while the name field still says the offered name.
        if let Some(offered) = self.drafts.new_param_offer.clone() {
            if offered.0 == self.drafts.new_param_name.trim() {
                ui.weak(Refusal::offer_wording(&offered));
            } else {
                // The user typed past the offer; it is stale.
                self.drafts.new_param_offer = None;
            }
        }
        ui.horizontal(|ui| {
            ui.label("add");
            ui.add(
                egui::TextEdit::singleline(&mut self.drafts.new_param_name)
                    .hint_text("name")
                    .desired_width(90.0),
            );
            for (dimension, label) in [
                (Dimension::Length, "Length"),
                (Dimension::Angle, "Angle"),
                (Dimension::Count, "Count"),
                (Dimension::Scalar, "Scalar"),
            ] {
                ui.radio_value(&mut self.drafts.new_param_dimension, Some(dimension), label);
            }
            ui.add(
                egui::DragValue::new(&mut self.drafts.new_param_value).speed(
                    if self.drafts.new_param_dimension == Some(Dimension::Count) {
                        1.0
                    } else {
                        0.0005
                    },
                ),
            );
        });
        let name = self.drafts.new_param_name.trim();
        let existing = if name.is_empty() {
            None
        } else {
            self.session.doc().params().get(&ParamName::new(name))
        };
        if let Some(existing) = existing {
            // The same sentence the session's refusal would show, and
            // the edit door it offers instead — refuse-then-offer,
            // ahead of the click.
            let name = ParamName::new(name);
            ui.horizontal(|ui| {
                ui.weak(Refusal::exists_wording(&name, existing.dim()));
                if ui.link(format!("edit {}", name.0)).clicked() {
                    self.ops.push(SessionOp::Select(Selection::Param(name)));
                }
            });
            return;
        }
        let ready = !name.is_empty() && self.drafts.new_param_dimension.is_some();
        let create = ui.add_enabled(ready, egui::Button::new("Create"));
        let create = if self.drafts.new_param_dimension.is_none() {
            create.on_disabled_hover_text("pick a dimension first")
        } else {
            create
        };
        if create.clicked()
            && let Some(dimension) = self.drafts.new_param_dimension
        {
            self.ops.push(SessionOp::CreateParam {
                name: ParamName::new(name),
                value: crate::props::doc_param(
                    dimension,
                    SlotValue::of(dimension, self.drafts.new_param_value),
                ),
            });
            self.drafts.new_param_name.clear();
            self.drafts.new_param_dimension = None;
            self.drafts.new_param_value = 0.0;
            self.drafts.new_param_offer = None;
        }
    }

    /// The selection's own header: what is selected, whether it still
    /// denotes anything, and the affordances that depend on it.
    ///
    /// **The unresolved state renders here and disables nothing by
    /// hand**: the enabling condition is `standing.live()` in every
    /// case, and the rows themselves are already empty because
    /// `DocSession::slot_rows` refuses to produce them. Two places
    /// would be two policies.
    fn standing_ui(&mut self, ui: &mut egui::Ui, standing: &Standing) {
        match standing {
            Standing::Empty => {}
            Standing::Node { node, present } => {
                ui.horizontal(|ui| {
                    ui.label(format!("feature {}", node.0));
                    if *present {
                        if ui.button(delete_label(self.session, *node)).clicked() {
                            self.ops.push(SessionOp::DeleteNode { node: *node });
                        }
                    } else {
                        ui.colored_label(UNRESOLVED_COLOR, "deleted");
                    }
                });
            }
            Standing::Param { name, present } => {
                if !present {
                    ui.colored_label(
                        UNRESOLVED_COLOR,
                        format!("parameter {} is no longer declared", name.0),
                    );
                }
            }
            Standing::Face { face, resolution } => {
                ui.horizontal(|ui| {
                    // Always a face: edge and vertex picking is out
                    // of scope for v1 selection, so the kind is not a
                    // variable to render.
                    ui.label(format!("face of feature {}", face.node.0));
                    if standing.live() && ui.button(delete_label(self.session, face.node)).clicked()
                    {
                        self.ops.push(SessionOp::DeleteNode { node: face.node });
                    }
                });
                // The typed verdict, rendered from the resolution
                // machinery's own payload — never a sentence composed
                // here about somebody else's refusal.
                match resolution.as_deref() {
                    None => {
                        ui.weak("no evaluation yet to resolve this against");
                    }
                    Some(pncad::select::Resolution::Resolved(_)) => {}
                    Some(pncad::select::Resolution::Failed(failure)) => {
                        ui.colored_label(
                            UNRESOLVED_COLOR,
                            format!("this face is gone: {}", failure.error),
                        );
                        if !failure.offers.is_empty() {
                            ui.weak(format!(
                                "{} rebind candidate(s) offered",
                                failure.offers.len()
                            ));
                        }
                    }
                    Some(pncad::select::Resolution::Indeterminate(cause)) => {
                        ui.colored_label(
                            UNRESOLVED_COLOR,
                            format!("this face cannot be resolved right now: {cause:?}"),
                        );
                    }
                }
            }
        }
    }

    /// The selected instance's display controls: the hide toggle and
    /// the free-move probe. Draws nothing for a non-instance node —
    /// the section is about per-instance display state, which other
    /// nodes do not have.
    fn instance_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId) {
        if !crate::display::is_instance(self.session.doc(), node) {
            return;
        }
        ui.separator();
        ui.label(format!("instance {}", node.0));
        let mut shown = !self.display.hidden.contains(&node);
        if ui.checkbox(&mut shown, "shown in viewport").changed() {
            self.ops.push(SessionOp::SetInstanceHidden {
                instance: node,
                hidden: !shown,
            });
        }
        match free_move_check(self.session.doc(), node) {
            Err(fault) => {
                // The typed ineligibility, shown where the control
                // would be — the same sentence the op would refuse
                // with.
                ui.weak(fault.to_string());
            }
            Ok(()) => {
                let current = self
                    .display
                    .moved
                    .get(&node)
                    .map_or([0.0; 3], |frame| frame.translation);
                ui.label("free-move probe (mm, display only):");
                let mut mm = current.map(|v| v * 1000.0);
                // The G1 gesture triple over DISPLAY state, through the
                // one widget→gesture mapping (`drag_ops`) so the typed-
                // input arm exists here too: typing a value performs a
                // one-shot begin/preview/commit, exactly one committed
                // display value. Each component's preview composes the
                // FULL frame from all three, so dragging x does not
                // zero y and z. The chrome offers the translation
                // components; the op vocabulary takes any rigid frame.
                ui.horizontal(|ui| {
                    for axis in 0..3 {
                        let mut value = mm[axis];
                        let widget = ui.add(egui::DragValue::new(&mut value).speed(0.5));
                        mm[axis] = value;
                        let frame_of = |mm: [f64; 3]| Frame::translation(mm.map(|v| v / 1000.0));
                        drag_ops(
                            &widget,
                            value,
                            SessionOp::BeginFreeMove { instance: node },
                            |_| SessionOp::PreviewFreeMove {
                                frame: frame_of(mm),
                            },
                            SessionOp::CommitFreeMove,
                            |_| {
                                vec![
                                    SessionOp::BeginFreeMove { instance: node },
                                    SessionOp::PreviewFreeMove {
                                        frame: frame_of(mm),
                                    },
                                    SessionOp::CommitFreeMove,
                                ]
                            },
                            self.ops,
                        );
                    }
                });
            }
        }
    }

    /// The mate tool's panel: activation, the held picks, the class
    /// choice with the kernel's admission verdicts, and the one
    /// committed edit.
    fn mate_tool_ui(&mut self, ui: &mut egui::Ui) {
        // A CLONE of the small tool value, so the panel can read it
        // while pushing ops and closing the tool — the authoritative
        // copy stays in the application and is only ever REPLACED
        // whole (activation, deactivation), never edited here.
        let Some(tool) = self.mate_tool.clone() else {
            if ui.button("Mate tool…").clicked() {
                *self.mate_tool = Some(MateTool::new());
            }
            return;
        };
        ui.label("mate tool: pick two faces in the viewport");
        match tool.state() {
            MateToolState::Idle => {
                ui.weak("no picks yet");
            }
            MateToolState::One(a) => {
                ui.weak(format!("pick a: face of instance {}", a.node.0));
            }
            MateToolState::Two { a, b } => {
                ui.weak(format!(
                    "pick a: instance {}; pick b: instance {}",
                    a.node.0, b.node.0
                ));
            }
        }
        // The class choice, offered THROUGH the kernel's admission
        // table: each class is shown with its verdict, and the
        // deferral (Fit and every future class) is a sentence here
        // rather than a button — the tool never offers what the doors
        // will not execute.
        let classes = admitted_classes();
        ui.horizontal(|ui| {
            for (ix, entry) in classes.iter().enumerate() {
                ui.radio_value(&mut self.drafts.mate_class, ix, entry.class.name());
            }
        });
        if let Some(entry) = classes.get(self.drafts.mate_class) {
            // The verdict in the table's own words: a minting class
            // says so; a class with no at-rest record shows the
            // table's reason, never a Debug dump of it.
            ui.weak(format!(
                "admission: {}",
                match entry.admission {
                    pncad::document::ClassAdmission::Mints => "mints an at-rest record",
                    other => other.no_record_reason(),
                }
            ));
        }
        ui.horizontal(|ui| {
            for (ix, (_, label)) in MATE_PRIMITIVES.iter().enumerate() {
                ui.radio_value(&mut self.drafts.mate_primitive, ix, *label);
            }
        });
        ui.checkbox(&mut self.drafts.mate_opposed, "axes opposed");
        let mut close = false;
        ui.horizontal(|ui| {
            if ui.button("Commit mate").clicked() {
                match (
                    classes.get(self.drafts.mate_class),
                    self.session.landed_pair(),
                ) {
                    (Some(entry), Some((doc, eval))) => {
                        let choice = MateChoice {
                            class: entry.class,
                            primitive: MATE_PRIMITIVES
                                .get(self.drafts.mate_primitive)
                                .map_or(MatePrimitive::FrameCoincidence, |(p, _)| *p),
                            sense: if self.drafts.mate_opposed {
                                AxisSense::Opposed
                            } else {
                                AxisSense::Aligned
                            },
                            clocking: None,
                        };
                        match tool.proposal(doc, eval, self.session.tol(), choice) {
                            Ok(proposal) => {
                                // Exactly one committed DocEdit; the
                                // tool closes with it.
                                self.ops.push(proposal.op());
                                close = true;
                            }
                            Err(error) => *self.status = Some(format!("mate tool: {error}")),
                        }
                    }
                    _ => {
                        *self.status = Some(
                            "mate tool: no landed evaluation to derive frames from".to_owned(),
                        );
                    }
                }
            }
            if ui.button("Cancel").clicked() {
                close = true;
            }
        });
        if close {
            *self.mate_tool = None;
        }
        ui.separator();
    }

    /// One PANEL ROW: a scalar slot on its own, or a 3-vector's three
    /// components on one line.
    ///
    /// The grouping is `props::SlotGroup`'s and the vocabulary's (see
    /// its docs); this function only lays it out. What the two arms
    /// share — the number widget, the gesture mapping, the driven
    /// affordance, the expression door, the range probe — is called
    /// once per COMPONENT, so a component of a vector is edited by
    /// exactly the operations a stand-alone slot is.
    ///
    /// **Three lines per group at most, whatever its arity.** Folding
    /// three slots onto one line buys nothing if their doors then take
    /// three lines each, so the doors are a single line of small
    /// buttons tagged by axis rather than a stacked block per
    /// component.
    fn slot_group_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, group: &SlotGroup) {
        match group {
            SlotGroup::Scalar(row) => {
                ui.horizontal(|ui| {
                    ui.label(row.slot.label());
                    self.slot_value_ui(ui, node, row);
                    self.slot_unit_ui(ui, node, core::slice::from_ref(row));
                    ui.weak(format!("{:?}", row.dimension));
                    if row.structural {
                        ui.weak("structural");
                    }
                });
                self.slot_notes_ui(ui, node, row);
                ui.horizontal(|ui| {
                    self.expression_button(ui, node, row, "expression…");
                    self.range_button(ui, node, row, "range?");
                });
            }
            SlotGroup::Vector { family, rows } => {
                ui.horizontal(|ui| {
                    ui.label(family.label());
                    for (axis, row) in Axis3::ALL.iter().zip(rows.iter()) {
                        ui.weak(axis.label());
                        self.slot_value_ui(ui, node, row);
                    }
                    // ONE picker for the vector: three components of a
                    // point are written in one unit or the user is
                    // being told something they did not mean to say.
                    // The picker reports a disagreement rather than
                    // hiding it (`slot_unit_ui`'s mixed arm).
                    self.slot_unit_ui(ui, node, rows.as_slice());
                    ui.weak(format!("{:?}", family.dimension()));
                });
                // The notes stay PER COMPONENT: an affordance names the
                // parameters driving one component, and a range is one
                // field's. Each names its axis.
                for (axis, row) in Axis3::ALL.iter().zip(rows.iter()) {
                    self.slot_notes_ui(ui, node, row);
                    let _ = axis;
                }
                ui.horizontal(|ui| {
                    ui.weak("expression");
                    for (axis, row) in Axis3::ALL.iter().zip(rows.iter()) {
                        self.expression_button(ui, node, row, axis.label());
                    }
                    ui.weak("range");
                    for (axis, row) in Axis3::ALL.iter().zip(rows.iter()) {
                        self.range_button(ui, node, row, axis.label());
                    }
                });
            }
        }
    }

    /// The number itself: shown in the unit the slot is WRITTEN in,
    /// authored back through the same factor.
    ///
    /// The conversion is `props::in_written` / `props::from_written`,
    /// which is the text door's one-multiply semantics — so a number
    /// typed here and the same number typed into the expression field
    /// land on identical bits. Everything crossing into the session
    /// below is canonical, exactly as it was.
    fn slot_value_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, row: &SlotRow) {
        let Ok(value) = row.value else {
            if let Err(ref error) = row.value {
                ui.weak(format!("{error}"));
            }
            return;
        };
        let unit = props::written_unit(row.dimension, row.unit);
        let mut number = props::in_written(value.as_f64(), unit);
        // The drag speed is in WRITTEN units now, so it has to be
        // scaled with them: 0.0005 was a half-micron step when the
        // field held metres, and would be a half-micron step in
        // millimetres too — i.e. a thousand times finer than the same
        // gesture used to be — if the divide were not applied.
        let speed = if row.structural {
            1.0
        } else {
            props::in_written(0.0005, unit)
        };
        let mut widget = egui::DragValue::new(&mut number).speed(speed);
        if let Some(unit) = unit {
            widget = widget.suffix(format!(" {}", unit.symbol()));
        }
        let widget = ui.add(widget);
        drag_ops(
            &widget,
            props::from_written(number, unit),
            SessionOp::BeginGesture {
                node,
                slot: row.slot,
            },
            |value| SessionOp::PreviewGesture { value },
            SessionOp::CommitGesture,
            |value| {
                vec![SessionOp::SetSlot {
                    node,
                    slot: row.slot,
                    value: SlotValue::of(row.dimension, value),
                }]
            },
            self.ops,
        );
    }

    /// The written-unit picker for one slot or for a whole vector.
    ///
    /// **"How do I want this number written" is an edit, not a view
    /// setting** — the unit is stored per literal and persists — so the
    /// picker emits `SessionOp::SetSlotUnit`, one per component.
    ///
    /// A vector whose components disagree shows `mixed` and is not
    /// quietly normalized: the document says what it says until someone
    /// picks. Choosing a unit then writes it to every component, which
    /// is the only reading of a single picker over three slots.
    ///
    /// Nothing is drawn at all for a dimension with no units (`Scalar`,
    /// `Count`) — there is no notation to offer for a number that is
    /// not a quantity.
    fn slot_unit_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, rows: &[SlotRow]) {
        let Some(first) = rows.first() else {
            return;
        };
        let options = props::unit_options(first.dimension);
        if options.is_empty() {
            return;
        }
        let written: Vec<Option<UnitDef>> = rows
            .iter()
            .map(|row| props::written_unit(row.dimension, row.unit))
            .collect();
        let common = written
            .iter()
            .all(|unit| *unit == written[0])
            .then_some(written[0])
            .flatten();
        let label = common.as_ref().map_or("mixed", UnitDef::symbol);
        // `id_salt` off the first component's slot: two vectors on one
        // node (a plane's origin and its normal) draw two pickers, and
        // egui identifies a popup by its id.
        egui::ComboBox::from_id_salt((node.0, format!("{:?}", first.slot), "unit"))
            .selected_text(label)
            .width(52.0)
            .show_ui(ui, |ui| {
                for option in options {
                    let picked = common == Some(option);
                    if ui.selectable_label(picked, option.symbol()).clicked() && !picked {
                        for row in rows {
                            self.ops.push(SessionOp::SetSlotUnit {
                                node,
                                slot: row.slot,
                                unit: Some(option),
                            });
                        }
                    }
                }
            });
    }

    /// What a slot has to SAY, under its number: the expression-driven
    /// refusal's affordance, the range reading when one has been taken
    /// for this field, and the expression editor while it is open.
    ///
    /// The affordance is attached to the row rather than raised on
    /// refusal alone so the user can see WHY the number will not move
    /// before they fight it — the refusal itself still surfaces in the
    /// status line when they try.
    fn slot_notes_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, row: &SlotRow) {
        if let SlotDriver::Expression { params } = &row.driver {
            ui.horizontal(|ui| {
                // The ratified wording, from its one home — the same
                // string the status line shows when the edit is
                // actually attempted.
                ui.weak(Refusal::affordance(
                    params,
                    row.value.as_ref().ok().copied(),
                ));
                for name in params {
                    if ui.link(format!("edit {}", name.0)).clicked() {
                        self.ops
                            .push(SessionOp::Select(Selection::Param(name.clone())));
                    }
                }
            });
        }
        let target = BoundsTarget::Slot {
            node,
            slot: row.slot,
        };
        if let Some((probed, result)) = self.session.bounds()
            && *probed == target
        {
            ui.weak(format!(
                "{}: {}",
                row.slot.label(),
                result.wording(props::written_unit(row.dimension, row.unit))
            ));
        }
        if self.drafts.expr_target == Some((node, row.slot)) {
            ui.horizontal(|ui| {
                ui.label(format!("{} =", row.slot.label()));
                ui.text_edit_singleline(&mut self.drafts.expr_text);
                if ui.button("Set").clicked() {
                    self.ops.push(SessionOp::SetSlotExpression {
                        node,
                        slot: row.slot,
                        text: self.drafts.expr_text.clone(),
                    });
                    self.drafts.expr_target = None;
                    self.drafts.expr_text.clear();
                }
                if ui.button("Cancel").clicked() {
                    self.drafts.expr_target = None;
                    self.drafts.expr_text.clear();
                }
            });
        }
    }

    /// The button that opens the expression text door for one slot.
    ///
    /// The field is deliberately EMPTY rather than pre-filled: the
    /// expression API has no text rendering, so a pre-filled field
    /// would be this crate's guess at what the slot says. See the
    /// module docs of `props`.
    fn expression_button(
        &mut self,
        ui: &mut egui::Ui,
        node: RecipeNodeId,
        row: &SlotRow,
        label: &str,
    ) {
        if ui.small_button(label).clicked() {
            self.drafts.expr_target = Some((node, row.slot));
            self.drafts.expr_text.clear();
        }
    }

    /// The button that asks for one slot's locally-valid range.
    ///
    /// **Asked for, not automatic** — see `SessionOp::ProbeBounds` for
    /// why. Offered only where a number can actually be written: a
    /// driven slot's value is not the user's to move, so a range for it
    /// would answer a question they cannot act on. The reading itself
    /// lands in [`Self::slot_notes_ui`], in the slot's own written unit.
    fn range_button(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, row: &SlotRow, label: &str) {
        let offered = !row.driver.is_driven() && row.value.is_ok();
        let button = ui.add_enabled(offered, egui::Button::new(label).small());
        let button = if offered {
            button.on_hover_text(
                "probe how far this can move before something new fails (tens of evaluations)",
            )
        } else {
            button.on_disabled_hover_text("a computed slot has no range of its own to probe")
        };
        if button.clicked() {
            self.ops.push(SessionOp::ProbeBounds {
                target: BoundsTarget::Slot {
                    node,
                    slot: row.slot,
                },
            });
        }
    }

    /// The range probe's button and reading for a DOCUMENT PARAMETER —
    /// the one field that is not a slot.
    fn param_bounds_ui(&mut self, ui: &mut egui::Ui, name: &ParamName, dimension: Dimension) {
        let target = BoundsTarget::Param { name: name.clone() };
        ui.horizontal(|ui| {
            if let Some((probed, result)) = self.session.bounds()
                && *probed == target
            {
                // A document parameter stores no display unit
                // (`props`' module docs name the asymmetry), so its
                // range reads in the canonical one.
                ui.weak(result.wording(props::written_unit(dimension, None)));
            }
            if ui
                .small_button("range?")
                .on_hover_text(
                    "probe how far this can move before something new fails \
                     (tens of evaluations)",
                )
                .clicked()
            {
                self.ops.push(SessionOp::ProbeBounds { target });
            }
        });
    }

    /// The view pane: the numbers the camera and the tessellation are
    /// actually running at.
    fn view_ui(&mut self, ui: &mut egui::Ui) {
        let stats = self.scene.stats();
        ui.heading("View");
        // The requested δ and the drawn one, and only one line when
        // they are the same number. The budget's own sentence is in
        // the status line; this pane's job is the reading.
        let drawn = self.fitted.map_or(self.delta, |fitted| fitted.delta);
        if drawn == self.delta {
            ui.label(format!("display δ: {:.3} mm", self.delta.get() * 1000.0));
        } else {
            ui.label(format!(
                "display δ: {:.3} mm asked, {:.3} mm drawn",
                self.delta.get() * 1000.0,
                drawn.get() * 1000.0
            ));
        }
        ui.label(format!("faces: {}", stats.faces));
        ui.label(format!("triangles: {}", stats.triangles));
        ui.separator();
        ui.label(format!(
            "camera yaw {:.1}°, pitch {:.1}°",
            self.camera.yaw().to_degrees(),
            self.camera.pitch().to_degrees()
        ));
        ui.label(format!(
            "distance {:.1} mm (band {:.1}–{:.1})",
            self.camera.distance() * 1000.0,
            self.camera.min_distance() * 1000.0,
            self.camera.max_distance() * 1000.0
        ));
        ui.separator();
        ui.label(format!("history: {} states", self.session.history().len()));
        match self.session.path() {
            Some(path) => ui.label(format!("file: {}", path.display())),
            None => ui.weak("unsaved document"),
        };
        if let Some(status) = self.status.as_ref() {
            ui.separator();
            ui.label(status.as_str());
        }
    }
}

/// **The one mapping from a `DragValue` to session operations**, and
/// the only place in this crate that turns a widget into a gesture.
///
/// G1 ratifies the shape: a continuous gesture emits previews against
/// scratch state and exactly ONE committed edit on release. egui's
/// `DragValue` does not hand that over — it conflates dragging with
/// typing and fires `changed()` every frame of a drag — so the
/// translation is the `drag_started` / `dragged` / `drag_stopped`
/// triple below.
///
/// **It is a function because the same file once had two copies of it
/// and one of them was wrong**: the slot rows mapped the triple and the
/// document-parameter row mapped a bare `changed()`, so dragging a
/// parameter committed one edit, one undo step and one re-evaluation
/// per frame. Two spellings of a ratified rule is one spelling too
/// many. Any future dragged number in this file calls this; nothing but
/// this comment enforces that, which is the honest state of it.
/// Generalized over the GESTURE VOCABULARY (`preview`/`commit` are
/// parameters) because the free-move probe runs the same triple over
/// display ops rather than document ops — one mapping, two
/// vocabularies, and the typed-input arm (`changed() && !dragged()`)
/// covered for BOTH, which is the arm a hand-mapped copy of this
/// function silently dropped once already.
fn drag_ops(
    widget: &egui::Response,
    value: f64,
    begin: SessionOp,
    preview: impl Fn(f64) -> SessionOp,
    commit: SessionOp,
    typed: impl Fn(f64) -> Vec<SessionOp>,
    ops: &mut Vec<SessionOp>,
) {
    if widget.drag_started() {
        ops.push(begin);
    }
    if widget.dragged() && widget.changed() {
        ops.push(preview(value));
    }
    if widget.drag_stopped() {
        ops.push(commit);
    } else if widget.changed() && !widget.dragged() {
        // Typed, not dragged: whatever the vocabulary spells a direct
        // value entry as — one edit for a document slot, a one-shot
        // begin/preview/commit for the display probe.
        ops.extend(typed(value));
    }
}

/// The delete button's label: it names the FEATURE it deletes, by the
/// node vocabulary's own kind name.
///
/// First-light finding (#1097's run): reached from a face selection, a
/// bare "Delete feature" read as deleting the *face* — an entity this
/// vocabulary can never delete; a face is a way of reaching the node
/// that made it. The label carries the target's kind so the affordance
/// states the operation it queues. The fallback arm is for a node the
/// document no longer holds — no button renders for one today, and if
/// that changes the label stays honest rather than panicking.
fn delete_label(session: &DocSession, node: RecipeNodeId) -> String {
    match session.doc().node(node) {
        Some(target) => format!("Delete feature '{}'", crate::tree::node_kind(target)),
        None => format!("Delete feature {}", node.0),
    }
}

/// The open dialog: a THIN veneer over `SessionOp::Open`.
///
/// Everything it does is choose a `Path`. The blocking call is
/// deliberate — a modal file chooser is the platform's own idea of a
/// modal file chooser, and the alternative (an async handle polled
/// across frames) would buy responsiveness during an interaction that
/// is already modal, at the cost of a second state machine.
///
/// **Absent on wasm**, with the two callers `cfg`-ed to match. That
/// second state machine is exactly what the browser would force —
/// `rfd`'s wasm backend offers only the async dialog, and there are
/// no paths behind it either — so the browser build does not have a
/// half-open door here; it has no door, and
/// [`frame::chooser_backend`] is what says so to the chrome.
#[cfg(not(target_family = "wasm"))]
fn pick_open() -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .add_filter("document", &[DOC_EXTENSION])
        .pick_file()
}

/// The save dialog, starting where the current document lives.
///
/// Absent on wasm for the reason [`pick_open`] states.
#[cfg(not(target_family = "wasm"))]
fn pick_save(current: Option<&std::path::Path>) -> Option<std::path::PathBuf> {
    let mut dialog = rfd::FileDialog::new().add_filter("document", &[DOC_EXTENSION]);
    if let Some(path) = current
        && let Some(dir) = path.parent()
    {
        dialog = dialog.set_directory(dir);
    }
    dialog.save_file()
}

/// The starting layout: the viewport with a tabbed side panel, in a
/// horizontal split with the viewport taking the larger share.
pub fn initial_layout() -> Tree<Pane> {
    let mut tiles = Tiles::default();
    let viewport = tiles.insert_pane(Pane::Viewport);
    let features = tiles.insert_pane(Pane::Features);
    let properties = tiles.insert_pane(Pane::Properties);
    let view = tiles.insert_pane(Pane::View);
    // The tree above the properties: selecting in one drives the
    // other, so they are visible together rather than tabbed apart.
    let stack = egui_tiles::Linear::new_binary(
        egui_tiles::LinearDir::Vertical,
        [features, properties],
        0.5,
    );
    let stacked = tiles.insert_container(egui_tiles::Container::Linear(stack));
    let side = tiles.insert_tab_tile(vec![stacked, view]);
    // Two thirds of the width to the viewport: the panels are this
    // unit's subject and need room to read.
    let linear =
        egui_tiles::Linear::new_binary(egui_tiles::LinearDir::Horizontal, [viewport, side], 0.66);
    let root = tiles.insert_container(egui_tiles::Container::Linear(linear));
    Tree::new("viewer_tree", root, tiles)
}

/// Column-major `f64` matrix to the `f32` the GPU consumes.
///
/// Written as a `map` rather than an indexed loop on purpose: the
/// earlier shape wrote through `get_mut` at statically-in-range
/// indices, so a wrong index would have produced a *partly converted*
/// matrix and no error at all. `map` cannot miss a slot.
fn to_f32(matrix: &[[f64; 4]; 4]) -> [[f32; 4]; 4] {
    matrix.map(|column| column.map(|value| value as f32))
}

/// Run the application, optionally opening `open` at startup.
///
/// The path goes through [`SessionOp::Open`] — the same typed door
/// the dialog feeds; a CLI argument is a way of choosing the `Path`,
/// never a different code path. An open that refuses shows its typed
/// refusal in the status line over the built-in startup document,
/// exactly as a refused dialog open would.
///
/// The depth buffer request is load-bearing — see `gpu`'s module docs.
///
/// # Errors
///
/// `eframe`'s own startup error, or a [`StartupError`] boxed into it:
/// a viewer that cannot build its scene reports why and exits rather
/// than opening a window onto nothing.
#[cfg(not(target_family = "wasm"))]
pub fn run(tol: Tol, open: Option<std::path::PathBuf>) -> eframe::Result<()> {
    #[allow(unused_mut)] // mutated only on the cfg(linux) arm below
    let mut options = eframe::NativeOptions {
        // EXPLICIT, not defaulted (first light, #1097): a bare
        // `NativeOptions::default()` leaves resizability and the
        // window's size to whatever the winit backend negotiates with
        // the window manager, and on at least one real WM that
        // negotiation produced a window resizable vertically but not
        // horizontally, with content stuck off the right edge. Stating
        // the intent — resizable, a size the chrome fits in, a floor it
        // stays readable at — is the portable posture whatever the
        // backend's own defaults do.
        viewport: egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 500.0]),
        depth_buffer: DEPTH_BITS,
        ..Default::default()
    };
    // WSLg: PREFER the X11 (XWayland) backend. Confirmed on the
    // first-light box (#1097): the horizontally-unresizable window is
    // WSLg's Wayland RAIL-shell CSD path, and `WAYLAND_DISPLAY=` —
    // the same X11 preference by hand — fixes resizing entirely. The
    // hook fires ONLY when WSL is detected (the env markers WSL
    // itself sets), so every other environment keeps winit's own
    // backend choice and needs nothing unset.
    #[cfg(target_os = "linux")]
    if frame::running_under_wsl() {
        options.event_loop_builder = Some(Box::new(|builder| {
            use winit::platform::x11::EventLoopBuilderExtX11 as _;
            builder.with_x11();
        }));
    }
    eframe::run_native(
        WINDOW_TITLE,
        options,
        Box::new(move |cc| {
            let mut app = ViewerApp::new(cc, tol).map_err(|error| error.to_string())?;
            if let Some(path) = open {
                // A successful open books the re-frame itself (the
                // success-only arm in `perform_batch`); a refused one
                // must not — the picture is still the startup scene.
                app.perform_batch(vec![SessionOp::Open(path)]);
            }
            Ok(Box::new(app))
        }),
    )
}

/// Why the browser build could not start.
///
/// A closed enum (D4 ¶3) rather than a `JsValue` or a string, and
/// deliberately so: on a phone there is no console to read and no
/// terminal behind the page, so every arm here is something the shell
/// prints INTO the page. The one failure mode a phone user cannot
/// diagnose is a blank canvas.
#[cfg(target_family = "wasm")]
#[derive(Debug)]
pub enum WebStartupError {
    /// No `window`, or no `document` on it — the module is running
    /// somewhere that is not a browser page (a bare Worker, say).
    NoDocument,
    /// No element carries the requested id.
    NoCanvasElement(String),
    /// An element carries the id, but it is not a `<canvas>`.
    NotACanvas(String),
    /// The application itself refused to start — the same typed
    /// refusals the native build reports to a terminal.
    Startup(StartupError),
    /// `eframe`'s own web runner refused, with whatever the browser
    /// said. The one arm that cannot be typed further: it is a
    /// `JsValue` from the platform, rendered through its `Debug`.
    Runner(String),
}

#[cfg(target_family = "wasm")]
impl core::fmt::Display for WebStartupError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoDocument => f.write_str(
                "no browser document: this build must run on a page, not in a bare worker",
            ),
            Self::NoCanvasElement(id) => {
                write!(f, "the page has no element with id `{id}`")
            }
            Self::NotACanvas(id) => {
                write!(f, "the element with id `{id}` is not a <canvas>")
            }
            Self::Startup(error) => write!(f, "{error}"),
            Self::Runner(message) => {
                write!(f, "the web runner refused to start: {message}")
            }
        }
    }
}

#[cfg(target_family = "wasm")]
impl core::error::Error for WebStartupError {}

/// Run the application on the `<canvas>` carrying `canvas_id`.
///
/// The browser counterpart of [`run`], and deliberately the whole of
/// the difference between the two platforms' entry points: everything
/// downstream — the session, the panes, the camera, the input map —
/// is the same code the native build runs.
///
/// **No `open` parameter, unlike [`run`].** There is no path to hand
/// it: the browser build links no file dialog and has no filesystem
/// to name, so it opens on the built-in startup document and stays
/// there. That is the spike's stated scope, not an oversight —
/// document I/O in the browser needs the download/upload or OPFS
/// story GUI-5 owns.
///
/// # Errors
///
/// Every arm of [`WebStartupError`]. Nothing here is allowed to fail
/// silently: a blank canvas on a phone is undiagnosable, so each
/// refusal carries a sentence the page can print.
#[cfg(target_family = "wasm")]
pub async fn run_web(tol: Tol, canvas_id: &str) -> Result<(), WebStartupError> {
    use eframe::wasm_bindgen::JsCast as _;

    let canvas = eframe::web_sys::window()
        .and_then(|window| window.document())
        .ok_or(WebStartupError::NoDocument)?
        .get_element_by_id(canvas_id)
        .ok_or_else(|| WebStartupError::NoCanvasElement(canvas_id.to_owned()))?
        .dyn_into::<eframe::web_sys::HtmlCanvasElement>()
        .map_err(|_| WebStartupError::NotACanvas(canvas_id.to_owned()))?;

    let options = eframe::WebOptions {
        // Load-bearing exactly as it is natively — see `gpu`'s module
        // docs. The viewport is a depth-tested pass, and a browser
        // that hands back a depth-less surface draws the scene with
        // its far faces in front.
        depth_buffer: DEPTH_BITS,
        ..Default::default()
    };

    eframe::WebRunner::new()
        .start(
            canvas,
            options,
            Box::new(move |cc| {
                let app = ViewerApp::new(cc, tol).map_err(|error| error.to_string())?;
                Ok(Box::new(app))
            }),
        )
        .await
        .map_err(|error| WebStartupError::Runner(format!("{error:?}")))
}
