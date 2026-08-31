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

use editor_core::appearance::Rgba8;
use eframe::egui;
use egui_tiles::{EditAction, Tile, TileId, Tiles, Tree, UiResponse};
use pncad::document::{Axis3, Dimension, DocumentId, ParamName, RecipeNodeId, SlotId};
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
use crate::parts::PartChooser;
use crate::pick::{self, PickCache, PickIndex};
use crate::prefs::{self, Prefs, PrefsStore};
use crate::props::{self, SlotDriver, SlotGroup, SlotRow, SlotValue};
use crate::revolvetool::RevolveTool;
use crate::scene::{self, DisplayTolerance, SceneMesh};
use crate::session::{
    BoundsTarget, DatumSpec, DocSession, ProfileShape, Refusal, Selection, SessionOp, Standing,
};
use crate::theme::{Polarity, Theme};
use crate::tree::{RowStatus, TreeRow};

/// Where this build keeps preferences.
///
/// One `cfg` alias rather than a trait object: there is exactly one
/// store per target, chosen at compile time, and a `Box<dyn>` would
/// buy a choice nothing makes. The browser's arm is [`prefs::Absent`]
/// until a `web_sys::Storage` store is written — it reports, and the
/// Save control disables itself, exactly as the file chooser does
/// where no portal exists.
#[cfg(not(target_family = "wasm"))]
type Store = prefs::file::FileStore;
#[cfg(target_family = "wasm")]
type Store = prefs::Absent;

/// This build's store.
fn prefs_store() -> Store {
    #[cfg(not(target_family = "wasm"))]
    {
        // The path comes from `frame`, the crate's one ambient door.
        prefs::file::FileStore::new(frame::prefs_path())
    }
    #[cfg(target_family = "wasm")]
    {
        prefs::Absent
    }
}
use pncad::document::{AxisSense, Frame, MatePrimitive};

/// The OS window title: the project's displayed name, which is a
/// PLACEHOLDER until Q9 settles a real one. It is not the crate name
/// — the crate, the binary and the canvas element stay `viewer`, and
/// only what a user reads says `pncad`.
const WINDOW_TITLE: &str = "pncad";

/// What the toolbar calls a document with no path of its own.
const UNTITLED: &str = "untitled";

/// The tab name of the container holding the feature tree and the
/// properties — the document's model, as against the View pane's
/// display settings.
const MODEL_TAB_TITLE: &str = "Model";

/// The most of the Features/Properties stack the feature tree is
/// auto-given: past this, the tree scrolls in its own half rather than
/// crowding the properties out.
const FEATURES_SHARE_CAP: f32 = 0.5;

/// Points of breathing room under the feature tree when its height is
/// what sizes the tile.
const FEATURES_SLACK: f32 = 8.0;

/// The starting display tolerance: 0.1 mm, fine enough that a 24 mm
/// hole reads as a circle and coarse enough to redraw instantly.
const INITIAL_DELTA: f64 = 1.0e-4;

/// Direction the light travels, world space; a unit vector over the
/// viewer's left shoulder.
const LIGHT_DIRECTION: [f32; 3] = [0.408_248_3, 0.408_248_3, -0.816_496_6];

/// The document file extension the dialog filters on.
///
/// `cfg`-ed with the dialogs it filters for: the browser build links
/// no chooser, so the constant has no reader there and an
/// unconditional one would be dead code under CI's `-D warnings`.
#[cfg(not(target_family = "wasm"))]
const DOC_EXTENSION: &str = "pncad";

/// `color` as the toolkit's own colour type.
///
/// The one place a [`Rgba8`] becomes an `egui::Color32`, matching
/// `theme::linear`'s role on the viewport side: a palette states sRGB
/// and each renderer converts once, at its own door.
fn chrome(color: Rgba8) -> egui::Color32 {
    egui::Color32::from_rgb(color.r, color.g, color.b)
}

/// Put the toolkit's chrome on `polarity`'s ground.
///
/// **The one place a [`Polarity`] meets `egui`**, and the reason
/// `crate::theme` can stay a non-`app` module: the palette states
/// which ground it is built on, and the mapping onto a toolkit's own
/// light and dark visuals lives here, where the toolkit already does.
///
/// `set_theme` rather than `set_visuals`: the preference is what the
/// context should be asked to follow, and stating it that way leaves
/// the toolkit's own per-theme visuals intact underneath — a
/// `set_visuals` would freeze one snapshot of them into the style.
fn apply_polarity(ctx: &egui::Context, polarity: Polarity) {
    ctx.set_theme(match polarity {
        Polarity::Light => egui::ThemePreference::Light,
        Polarity::Dark => egui::ThemePreference::Dark,
    });
}

/// Points of indent per level of the feature tree.
const INDENT_STEP: f32 = 12.0;

/// The deepest level the tree indents for.
///
/// A backstop, not the working limit: `tree`'s depth counts BRANCHES
/// off a node's primary input, so a chained document sits a handful
/// of levels deep however long its chain is. A document that does
/// nest genuinely deeper than this stops moving right here; the rows
/// stay in evaluation order, so the tree is still readable as a
/// sequence, and what is lost is depth information that had already
/// stopped fitting the pane.
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
#[derive(Debug)]
struct Drafts {
    /// The View pane's δ field while it has focus, in millimetres as
    /// typed. `None` whenever it does not, so an unfocused field shows
    /// the δ actually in force — including one the triangle budget
    /// chose after this field last committed.
    delta_mm: Option<String>,
    /// The slot whose value field is holding REFUSED text.
    ///
    /// The field's text is egui's while it has focus and the
    /// document's afterwards, so there is only one thing this layer
    /// has to remember: text a parse refusal sent back
    /// ([`frame::retype_draft`]). Acting on the refusal — going off to
    /// declare the parameter it named — must not cost the text that
    /// raised it, so the field keeps showing it until an expression
    /// edit for that slot lands.
    expr_target: Option<(RecipeNodeId, SlotId)>,
    /// The refused text itself.
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
    /// The toolbar's New… form: `Some(text)` while the name field is
    /// open, `None` while it is not. The op is not emitted until
    /// Create — a name in flight is a draft, not a document.
    new_doc_name: Option<String>,
    /// The add-datum form's kind choice.
    datum_kind: DatumKind,
    /// The add-datum form's origin/position, metres.
    datum_origin: [f64; 3],
    /// Its normal/direction (unitless; ignored by the point form).
    datum_direction: [f64; 3],
    /// The add-profile form's shape choice.
    profile_shape: ShapeKind,
    /// The circle form's centre, metres.
    profile_centre: [f64; 2],
    /// The circle form's radius, metres.
    profile_radius: f64,
    /// Whether the circle carries a concentric bore (a second loop
    /// inside the first) — what lets the chrome author the ring
    /// demo's annulus while staying a template, not a sketcher. The
    /// form guards bore < radius (see `add_profile_ui`): the loop
    /// ROLES come from the profile layer's containment forest, so a
    /// bore at or beyond the outer would not fail — it would swap
    /// which circle is the hole, silently defeating the form's own
    /// wording.
    profile_bored: bool,
    /// The bore's radius, metres.
    profile_bore: f64,
    /// The rectangle form's width and height, metres.
    profile_extent: [f64; 2],
    /// The extrude form's distance, metres.
    extrude_distance: f64,
    /// The revolve tool's angle, radians.
    revolve_angle: f64,
}

impl Default for Drafts {
    /// The creation forms' sensible defaults (the GAUTH-1 spec):
    /// datum origin 0 with normal/direction +z, a 10 mm circle or
    /// rectangle, a 10 mm extrude, a full-turn revolve. Everything
    /// else starts empty.
    fn default() -> Self {
        Self {
            delta_mm: None,
            expr_target: None,
            expr_text: String::new(),
            new_param_name: String::new(),
            new_param_dimension: None,
            new_param_value: 0.0,
            new_param_offer: None,
            mate_class: 0,
            mate_primitive: 0,
            mate_opposed: false,
            new_doc_name: None,
            datum_kind: DatumKind::Plane,
            datum_origin: [0.0; 3],
            datum_direction: [0.0, 0.0, 1.0],
            profile_shape: ShapeKind::Circle,
            profile_centre: [0.0; 2],
            profile_radius: 0.01,
            profile_bored: false,
            profile_bore: 0.005,
            profile_extent: [0.01, 0.01],
            extrude_distance: 0.01,
            revolve_angle: core::f64::consts::TAU,
        }
    }
}

/// The add-datum form's kind choice — one form, the three
/// [`DatumSpec`] arms. An enum rather than an index into a label
/// list, so every consumer matches exhaustively and a fourth kind
/// cannot leave a silent wildcard arm behind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DatumKind {
    /// A plane datum.
    Plane,
    /// An axis datum.
    Axis,
    /// A point datum.
    Point,
}

impl DatumKind {
    /// Every kind with its radio label, in form order.
    const ALL: [(Self, &'static str); 3] = [
        (Self::Plane, "plane"),
        (Self::Axis, "axis"),
        (Self::Point, "point"),
    ];
}

/// The add-profile form's template choice — the GAUTH-1 template
/// vocabulary, an enum for the reason [`DatumKind`] is one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShapeKind {
    /// A circle, optionally with a concentric bore.
    Circle,
    /// A centred rectangle.
    Rectangle,
}

impl ShapeKind {
    /// Every shape with its radio label, in form order.
    const ALL: [(Self, &'static str); 2] =
        [(Self::Circle, "circle"), (Self::Rectangle, "rectangle")];
}

/// One drag tick of a creation-form numeric field, in the field's own
/// unit — half a millimetre on the metre fields, matching the drag
/// feel of the shipped panel value fields. One home so the forms
/// cannot drift apart a digit at a time.
const FIELD_DRAG_SPEED: f64 = 0.0005;

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
    /// Whether the next scene to land should have its δ CHOSEN by the
    /// triangle budget, rather than drawn at the δ already in force.
    ///
    /// Set at startup and on every successful `Open`, and cleared the
    /// moment it is spent. It is what makes the budget a DEFAULT: a
    /// document arrives, the budget picks a δ that draws it in a
    /// reasonable time, and from then on δ is whatever the user asked
    /// for. Anything stronger — clamping every rebuild — would make
    /// the View pane's δ field a control that does nothing on exactly
    /// the documents someone would want it for.
    fit_delta_on_scene: bool,
    /// The budget's verdict, while the δ it chose is still the δ in
    /// force. `None` once the user has moved δ themselves
    /// ([`ViewerApp::set_delta`] clears it), because from there
    /// the number on screen is theirs and the badge would be claiming
    /// a choice it did not make.
    budget_delta: Option<crate::scene::FittedDelta>,
    /// The modal mate tool, when active. `None` is "not in the mate
    /// tool"; the tool's own state (the held picks) lives inside it.
    mate_tool: Option<MateTool>,
    /// The modal revolve tool, when active — the same shape as the
    /// mate tool, holding node picks instead of face picks.
    revolve_tool: Option<RevolveTool>,
    /// The `Add part…` chooser, when open. `None` is "no chooser"; the
    /// scanned catalogue it is showing lives inside it, taken once when
    /// it opened.
    part_chooser: Option<PartChooser>,
    camera: Camera,
    input: InputMap,
    /// The palette in force — a USER preference, held in the
    /// application rather than in the document (`crate::theme`), so
    /// switching it can never touch what a file says.
    ///
    /// Nothing persists it yet: it is chosen at startup and may be
    /// changed in-session, and a viewer reopened forgets. The
    /// preferences file that will remember it is its own piece of
    /// work; this field is what it will write into.
    theme: Theme,
    tree: Tree<Pane>,
    /// Whether the user has resized a tile themselves. From the first
    /// drag the layout is theirs and nothing here sizes it again.
    split_dragged: bool,
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
    /// Where the theme choice is remembered. Held rather than
    /// rediscovered per save: the path is an environment read, and a
    /// viewer whose config directory moved mid-session would be
    /// stranger than one that kept writing where it started.
    store: Store,
    /// The input preset the loaded file named, carried so that saving
    /// a theme change writes it back rather than dropping it.
    ///
    /// **The name as WRITTEN, not the resolved [`InputMap`]** — a
    /// preset this viewer does not recognise falls back for the
    /// session (`prefs::Notice::UnknownPreset`) but must survive in
    /// the file, or opening an older viewer once would silently
    /// delete a newer one's choice. Kept as a field rather than
    /// re-read at save time because the save happens on a UI event
    /// and reading the file there would race the very write it is
    /// about to do.
    keys_pref: Option<String>,
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

        // Preferences, before anything is drawn. A refusal here is
        // never fatal: a viewer that would not open because its
        // colour scheme was unreadable would be trading the whole
        // product for a preference, so every arm ends in a message
        // and the defaults.
        let store = prefs_store();
        let (saved, mut notices) = match store.load() {
            Ok(Some(text)) => match Prefs::from_toml(&text) {
                Ok((prefs, notices)) => (prefs, notices.iter().map(ToString::to_string).collect()),
                Err(error) => (Prefs::default(), vec![error.to_string()]),
            },
            Ok(None) => (Prefs::default(), Vec::new()),
            Err(error) => (Prefs::default(), vec![error.to_string()]),
        };
        let (theme, theme_notice) = saved.resolve_theme();
        let (input, keys_notice) = saved.resolve_keys();
        notices.extend(
            [theme_notice, keys_notice]
                .into_iter()
                .flatten()
                .map(|n| n.to_string()),
        );

        // The startup palette reaches the chrome here, not on the
        // first frame: a window that opened dark and turned light one
        // frame later would flash, and the flash would be the honest
        // report of a theme applied too late.
        apply_polarity(&cc.egui_ctx, theme.polarity);

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
            // The startup document goes through the same door an
            // opened one does: it is small enough that the budget will
            // not move its δ, and a first picture that took a
            // different path from every later one is a difference
            // waiting to be a bug.
            fit_delta_on_scene: true,
            budget_delta: None,
            mate_tool: None,
            revolve_tool: None,
            part_chooser: None,
            camera,
            input,
            theme,
            tree: initial_layout(),
            split_dragged: false,
            drafts: Drafts::default(),
            pending_fit: true,
            fit_on_scene: false,
            // Whatever the preferences file had to say, in the one
            // place this crate puts a thing that went wrong.
            status: (!notices.is_empty()).then(|| notices.join("; ")),
            chooser: frame::chooser_backend(),
            store,
            keys_pref: saved.keys,
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
        // The revolve tool's survival step: its picks are nodes, so
        // the document alone answers whether each is still there.
        if let Some(tool) = self.revolve_tool.as_mut() {
            for event in tool.reconcile(self.session.doc()) {
                self.status = Some(format!("revolve tool: {event}"));
            }
        }
        // **The budget picks the δ a document opens at**, once, before
        // anything is built at the δ in force — so the un-budgeted
        // build is never paid for, only avoided. `scene::fit_delta`
        // carries the method and the numbers; `TRIANGLE_BUDGET` says
        // why there is a budget at all.
        //
        // A fit that refuses leaves δ alone: the document is one whose
        // roots do not gather or whose probe will not tessellate, and
        // the index build below is about to say so with its own typed
        // refusal. Two opinions about that would be one too many.
        if self.fit_delta_on_scene
            && let Some((doc, evaluation)) = self.session.landed_pair()
        {
            self.fit_delta_on_scene = false;
            if let Ok(fitted) = scene::fit_delta(doc, evaluation, self.delta, self.session.tol()) {
                self.delta = fitted.delta;
                self.budget_delta = fitted.requested_cost.map(|_| fitted);
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

    /// Change δ to `delta` world units, rebuilding the picture from
    /// the evaluation already in hand — a display change is not a
    /// document change and re-runs no geometry above the tessellator.
    ///
    /// The value goes through [`DisplayTolerance::new`], the one door
    /// that decides what a δ may be, so a zero or a negative number is
    /// refused with the tessellator's own condition and the picture
    /// keeps the δ it had.
    fn set_delta(&mut self, delta: f64) {
        match DisplayTolerance::new(delta) {
            Ok(delta) => {
                self.delta = delta;
                // From here the number is the user's. The budget chose
                // an opening δ and has no further say — including no
                // say over a δ finer than it would have picked, which
                // is the whole difference between a default and a cap.
                self.budget_delta = None;
                self.sync_scene();
            }
            Err(error) => self.status = Some(format!("{error}")),
        }
    }

    /// Give the Features tile the height its content wants, capped at
    /// [`FEATURES_SHARE_CAP`] of the stack it shares with Properties.
    ///
    /// The two panes are read TOGETHER — selecting in the tree is what
    /// fills the properties — so a three-row tree that pushes the
    /// Properties heading half a page down is half a pane of nothing.
    /// A long tree reaches the cap and the stack is split as it always
    /// was, the tree scrolling inside its own half.
    ///
    /// `wanted` is the tree's laid-out height in points. The stack's
    /// own height comes from the two tiles' last layout, so this is a
    /// no-op on the very first frame, before there are rectangles to
    /// read; the frame after has both. The caller owns the check that
    /// the user has not taken the divider over.
    fn fit_features_share(&mut self, wanted: f32) {
        let tiles = &mut self.tree.tiles;
        let (Some(features), Some(properties)) = (
            tiles.find_pane(&Pane::Features),
            tiles.find_pane(&Pane::Properties),
        ) else {
            return;
        };
        let (Some(above), Some(below)) = (tiles.rect(features), tiles.rect(properties)) else {
            return;
        };
        let stack = above.height() + below.height();
        if stack <= 0.0 {
            return;
        }
        let Some(stacked) = model_stack(tiles) else {
            return;
        };
        // Slack over the measured height so the last row is not flush
        // against the divider.
        let fraction = ((wanted + FEATURES_SLACK) / stack).clamp(0.0, FEATURES_SHARE_CAP);
        if let Some(Tile::Container(egui_tiles::Container::Linear(linear))) = tiles.get_mut(stacked)
        {
            // Shares are relative, so a pair summing to 2 states the
            // fraction directly — the spelling `Linear::new_binary`
            // itself uses.
            linear.shares.set_share(features, 2.0 * fraction);
            linear.shares.set_share(properties, 2.0 * (1.0 - fraction));
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
            let revolved = matches!(op, SessionOp::AddRevolve { .. });
            match self.session.perform(op).refusal {
                Some(next) => refusal = Refusal::preferred(refusal, next),
                // A replaced document owes a re-frame AND a fresh δ
                // — both taken when its scene actually lands, not on
                // the outgoing picture. The δ the last document was
                // being read at says nothing about this one: it is a
                // length in metres, and the new document may be a
                // different size and a different shape.
                None if opened => {
                    self.fit_on_scene = true;
                    self.fit_delta_on_scene = true;
                    self.budget_delta = None;
                }
                // The revolve tool closes when its edit actually
                // COMMITS, not when its button is clicked — a refusal
                // leaves the tool open with its picks held
                // (`revolve_tool_ui`'s commit arm carries the other
                // half of this rule).
                None if revolved => self.revolve_tool = None,
                None => {}
            }
        }
        let update = frame::batch_status(&performed, refusal.as_ref());
        // The refuse-then-offer pair for a parse refusal: hold the
        // refused text in the field it was typed into so acting on the
        // refusal does not cost it, and — for an unknown parameter
        // name — prefill the add-parameter affordance with the name it
        // offers to create (dimension deliberately left unpicked). An
        // expression edit that was NOT refused this way releases the
        // field back to the document, which is now what the user
        // asked for.
        match frame::retype_draft(&performed, refusal.as_ref()) {
            Some((node, slot, text)) => {
                self.drafts.expr_target = Some((node, slot));
                self.drafts.expr_text = text;
            }
            None if performed
                .iter()
                .any(|op| matches!(op, SessionOp::SetSlotExpression { .. })) =>
            {
                self.drafts.expr_target = None;
                self.drafts.expr_text.clear();
            }
            None => {}
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
    /// Write the current theme choice to the preferences store.
    ///
    /// **Best-effort, and it reports.** A store that cannot be
    /// written is worth one line in the status area and nothing more:
    /// the theme is already applied on screen, so a failure here
    /// costs the next session's memory of it, never this session's
    /// work. Refusing the switch because it could not be recorded
    /// would be the worse trade.
    ///
    /// The whole document is rewritten rather than patched, so every
    /// setting this viewer understands has to be carried across —
    /// which is why [`Self::keys_pref`] exists. A key it does NOT
    /// understand is lost, and that is stated rather than hidden: it
    /// is the price of a hand-written renderer that keeps its
    /// comments, and such a key was already reported on load.
    fn remember_theme(&mut self) {
        if !self.store.usable() {
            return;
        }
        let prefs = Prefs {
            theme: Some(self.theme.name.to_owned()),
            keys: self.keys_pref.clone(),
        };
        if let Err(error) = self.store.save(&prefs.to_toml()) {
            self.status = Some(error.to_string());
        }
    }

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
        // The palette the chrome may switch to this frame. Collected
        // like `ops` and applied after the closures rather than
        // written through `self` inside one — a theme change is
        // application state, never a `SessionOp`, because no palette
        // has ever changed what a document says.
        let mut chosen = self.theme;

        egui::Panel::top("viewer_toolbar").show(ui, |ui| {
            ui.horizontal(|ui| {
                // What is OPEN, not what the program is called: the
                // window title already carries the application's name,
                // and a toolbar that repeats it tells a user nothing
                // they cannot see in their own title bar.
                ui.label(document_name(self.session.path()));
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
                // The New… control (GAUTH-1): one name field, because
                // the document id is derived from the name — see
                // `SessionOp::NewDocument`. The field is a draft; the
                // op is emitted only by Create, and only for a
                // non-blank name (the typed refusal backing the
                // disabled button is `Refusal::EmptyName`).
                match self.drafts.new_doc_name.as_mut() {
                    None => {
                        if ui.button("New…").clicked() {
                            self.drafts.new_doc_name = Some(String::new());
                        }
                    }
                    Some(name) => {
                        ui.add(
                            egui::TextEdit::singleline(name)
                                .hint_text("document name")
                                .desired_width(120.0),
                        );
                        let typed = name.trim().to_owned();
                        if ui
                            .add_enabled(!typed.is_empty(), egui::Button::new("Create"))
                            .on_disabled_hover_text("the document id is derived from the name")
                            .clicked()
                        {
                            ops.push(SessionOp::NewDocument { name: typed });
                            self.drafts.new_doc_name = None;
                        } else if ui.button("Cancel").clicked() {
                            self.drafts.new_doc_name = None;
                        }
                    }
                }
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
                if ui
                    .button("Zoom to fit")
                    .on_hover_text("frame the whole model in the viewport")
                    .clicked()
                {
                    // The toolbar has no pane rectangle, so it asks
                    // for a fit rather than performing one; the
                    // viewport takes it at the real aspect.
                    self.pending_fit = true;
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
                        ui.colored_label(
                            chrome(self.theme.unresolved),
                            format!("at rest: {message}"),
                        );
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
                        chrome(self.theme.unresolved),
                        format!("checks: {} finding(s)", report.findings.len()),
                    )
                    .on_hover_ui(|ui| {
                        for finding in &report.findings {
                            ui.label(finding.to_string());
                        }
                    });
                }
                // The display budget's badge: shown while the δ on
                // screen is the one the budget CHOSE when the document
                // opened, and gone the moment the user picks their
                // own. A read of held state, like the two badges
                // above, which is why it is here rather than in the
                // status line below — that line is cleared by the next
                // camera fold (issue filed), and "this δ was chosen
                // for you" has to outlive a mouse drag.
                if let Some(fitted) = self.budget_delta
                    && let Some(wording) = fitted.wording()
                {
                    ui.separator();
                    ui.weak(format!("δ {:.3} mm chosen", fitted.delta.get() * 1.0e3))
                        .on_hover_text(wording);
                }
                ui.separator();
                // The palette picker. Every registered theme, by the
                // name `crate::theme` gives it — the registry IS the
                // menu, so a theme cannot be shipped and left
                // unreachable.
                egui::ComboBox::from_id_salt("viewer_theme")
                    .selected_text(chosen.name)
                    .show_ui(ui, |ui| {
                        for theme in Theme::ALL {
                            ui.selectable_value(&mut chosen, *theme, theme.name);
                        }
                    });
                if let Some(status) = &self.status {
                    ui.separator();
                    ui.label(status.as_str());
                }
            });
        });

        if chosen != self.theme {
            self.theme = chosen;
            apply_polarity(ui.ctx(), chosen.polarity);
            self.remember_theme();
        }

        let display = self.session.display_view();
        let mut delta_request: Option<f64> = None;
        let mut features_content_height: Option<f32> = None;
        let mut split_dragged = self.split_dragged;
        egui::CentralPanel::no_frame().show(ui, |ui| {
            let mut behavior = ViewerBehavior {
                session: &self.session,
                delta: self.delta,
                budget_delta: self.budget_delta,
                scene: &self.scene,
                index: self.picks.index(),
                revision: self.revision,
                camera: &mut self.camera,
                input: self.input,
                theme: self.theme,
                drafts: &mut self.drafts,
                display: &display,
                mate_tool: &mut self.mate_tool,
                revolve_tool: &mut self.revolve_tool,
                part_chooser: &mut self.part_chooser,
                pending_fit: &mut self.pending_fit,
                status: &mut self.status,
                id_answer: &self.id_answer,
                id_log: &mut self.id_log,
                ops: &mut ops,
                delta_request: &mut delta_request,
                features_content_height: &mut features_content_height,
                split_dragged: &mut split_dragged,
            };
            self.tree.ui(&mut behavior, ui);
        });
        // Read AFTER the frame drew, and before anything writes a
        // share back: a divider dragged this frame has already set the
        // flag, so the auto-size below stands down on the same frame
        // the user's mouse moved rather than one frame later, having
        // overwritten it once.
        self.split_dragged = split_dragged;
        if !self.split_dragged
            && let Some(height) = features_content_height
        {
            self.fit_features_share(height);
        }
        if let Some(delta) = delta_request {
            self.set_delta(delta);
        }

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
        // The revolve tool consumes the same stream at node
        // resolution: a tree click is a node pick directly, a face
        // pick reaches the feature it belongs to (`Selection::node`,
        // the one viewport→tree inversion).
        if let Some(tool) = self.revolve_tool.as_mut() {
            for op in &ops {
                if let SessionOp::Select(selection) = op
                    && let Some(node) = selection.node()
                {
                    tool.pick(node);
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
    /// The δ the picture is drawn at.
    delta: DisplayTolerance,
    /// Set while `delta` is the one the triangle budget chose when the
    /// document opened, rather than one the user picked.
    budget_delta: Option<crate::scene::FittedDelta>,
    scene: &'a Arc<SceneMesh>,
    index: Option<&'a PickIndex>,
    revision: u64,
    camera: &'a mut Camera,
    input: InputMap,
    /// The palette this frame draws with; `Copy`, because a theme is
    /// a small value and the frame must not be able to change it.
    theme: Theme,
    drafts: &'a mut Drafts,
    /// The display snapshot this frame draws and picks under.
    display: &'a DisplayView,
    /// The modal mate tool, if active.
    mate_tool: &'a mut Option<MateTool>,
    /// The modal revolve tool, if active.
    revolve_tool: &'a mut Option<RevolveTool>,
    /// The `Add part…` chooser, if open.
    part_chooser: &'a mut Option<PartChooser>,
    pending_fit: &'a mut bool,
    status: &'a mut Option<String>,
    id_answer: &'a Arc<AtomicU64>,
    id_log: &'a mut IdQueryLog,
    ops: &'a mut Vec<SessionOp>,
    /// A δ the View pane's field committed this frame, in world units.
    /// The pane holds a borrow of the app, not the app, so it hands
    /// the number back for [`ViewerApp::set_delta`] to judge.
    delta_request: &'a mut Option<f64>,
    /// What the Features pane's content laid out to this frame, once
    /// it has drawn.
    features_content_height: &'a mut Option<f32>,
    /// Set when the user resized a tile themselves.
    split_dragged: &'a mut bool,
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

    /// The title of any TILE, container as well as pane.
    ///
    /// egui_tiles titles an unnamed container by its layout direction,
    /// so the Features/Properties stack came up as `Vertical` — the
    /// name of a split rather than of anything in it. That stack
    /// ([`model_stack`]) is the document's model and says so. Every
    /// other tile falls through to the same defaults the trait would
    /// have used.
    fn tab_title_for_tile(&mut self, tiles: &Tiles<Pane>, tile_id: TileId) -> egui::WidgetText {
        if model_stack(tiles) == Some(tile_id) {
            return MODEL_TAB_TITLE.into();
        }
        match tiles.get(tile_id) {
            Some(Tile::Pane(pane)) => self.tab_title_for_pane(pane),
            Some(Tile::Container(container)) => format!("{:?}", container.kind()).into(),
            None => "MISSING TILE".into(),
        }
    }

    /// A layout edit the USER made.
    ///
    /// The one that matters here is a resize: from the first time
    /// someone drags the Features/Properties divider, the split is
    /// theirs and [`ViewerApp::fit_features_share`] stops touching it.
    /// Auto-sizing is a default, and a default that argues with a
    /// mouse is a bug.
    fn on_edit(&mut self, edit_action: EditAction) {
        if edit_action == EditAction::TileResized {
            *self.split_dragged = true;
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
        let scrolled = egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .id_salt(tile_id)
            .show(ui, |ui| match pane {
                // Handled above; this arm cannot be reached.
                Pane::Viewport => {}
                Pane::Features => self.features_ui(ui),
                Pane::Properties => self.properties_ui(ui),
                Pane::View => self.view_ui(ui),
            });
        // The tree's own height, measured rather than predicted: the
        // scroll area knows what its content laid out to, and that is
        // the number the Features/Properties split is sized from.
        if *pane == Pane::Features {
            *self.features_content_height = Some(scrolled.content_size.y);
        }
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
        // **An open tool narrows the priority rule, it does not
        // re-decide it.** The mate tool takes faces, and on a real
        // part whole faces sit within the edge radius of their own
        // boundary — a narrow shelf, a small hole's wall — so with
        // edges always winning those faces were unpickable for as long
        // as the tool was open. The narrowing travels through
        // `hovered_for`, the one door that answers what a cursor
        // means, so the tool cannot end up on a different rule.
        let kinds = if self.mate_tool.is_some() {
            pick::PickKinds::FacesOnly
        } else {
            pick::PickKinds::Any
        };
        if let (Some(index), Some(eval)) = (self.index, self.session.evaluation()) {
            for action in actions {
                // A hover over an unchanged picture at an unmoved
                // cursor asks a question whose answer the session
                // already holds. A click never skips: it is an
                // ACTION, not an observation.
                if step == IdStep::Hold && matches!(action, input::PickAction::Hover(_)) {
                    continue;
                }
                match index.op_under(eval, self.camera, viewport, action, self.display, kinds) {
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
        // The edge half of the same question, and the same discipline:
        // recomputed every frame from state that lives in one place.
        let edges = self
            .index
            .map(|index| {
                pick::edge_overlay(
                    index,
                    self.display,
                    self.session.selection(),
                    self.session.hover(),
                )
            })
            .unwrap_or_default();

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
        //
        // **The ray side of this comparison is the FACE under the
        // cursor, not the hover.** An id buffer can answer with a
        // patch and nothing else, so the question both sides must
        // answer is "which patch is here"; the hover answers a
        // different one as soon as the priority rule picks an edge,
        // and feeding it would report a disagreement between two
        // questions on every frame the cursor came within
        // `EDGE_PICK_RADIUS_PX` of an edge. So the face is re-derived
        // through `face_under_cursor`, and only where there is a fresh
        // answer waiting for it — `disagreement` still owns the
        // freshness rule, this only declines to do the work when no
        // question is outstanding at all.
        let outstanding = self.id_log.outstanding();
        let from_ray = outstanding.and_then(|_| {
            let index = self.index?;
            let eval = self.session.evaluation()?;
            index
                .face_under_cursor(eval, self.camera, viewport, cursor_px?, self.display)
                .ok()
                .flatten()
        });
        if let Some(report) = self.index.and_then(|index| {
            frame::disagreement(
                index,
                self.id_answer.load(Ordering::Relaxed),
                outstanding,
                from_ray.as_ref().map(|face| &face.name),
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
                theme: self.theme,
                highlight: highlight.unwrap_or_default(),
                edges,
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
                    ui.colored_label(chrome(self.theme.unresolved), row.status.badge());
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
        self.create_ui(ui);
        self.add_part_ui(ui);
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
            // Slot rows for the feature that MADE the picked entity —
            // the node `slot_groups` itself answered for, so the rows
            // shown and the node an edit lands on are one answer. A
            // pick is a way of reaching that feature, which is what
            // G3's click-to-select is for, and an edge reaches it the
            // same way a face does.
            Selection::Face(_) | Selection::Edge(_) => {
                let groups = self.session.slot_groups();
                if groups.is_empty() && standing.live() {
                    ui.weak("this feature carries no parameters");
                }
                // `Selection::node()` is the one inversion — always
                // `Some` on these two arms, and read rather than
                // re-derived so the rows and the edits land on the
                // node `slot_groups` answered for.
                if let Some(feature) = self.session.selection().node() {
                    for group in &groups {
                        self.slot_group_ui(ui, feature, group);
                    }
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
                        if delete_button(ui, self.session, *node) {
                            self.ops.push(SessionOp::DeleteNode { node: *node });
                        }
                    } else {
                        ui.colored_label(chrome(self.theme.unresolved), "deleted");
                    }
                });
            }
            Standing::Param { name, present } => {
                if !present {
                    ui.colored_label(
                        chrome(self.theme.unresolved),
                        format!("parameter {} is no longer declared", name.0),
                    );
                }
            }
            Standing::Face { face, resolution } => {
                self.entity_standing_ui(
                    ui,
                    "face",
                    face.feature(),
                    resolution.as_deref(),
                    standing.live(),
                );
            }
            Standing::Edge { edge, resolution } => {
                self.entity_standing_ui(
                    ui,
                    "edge",
                    edge.feature(),
                    resolution.as_deref(),
                    standing.live(),
                );
            }
        }
    }

    /// A picked entity's header: which feature it belongs to, the
    /// delete that feature offers, and the typed resolution verdict.
    ///
    /// **One rendering for every kind of picked entity**, taking the
    /// noun as an argument: a face and an edge differ in what they are
    /// called and in nothing else this panel does, and two copies of
    /// the verdict ladder is how the two come to report a vanished
    /// referent differently.
    fn entity_standing_ui(
        &mut self,
        ui: &mut egui::Ui,
        noun: &str,
        feature: RecipeNodeId,
        resolution: Option<&pncad::select::Resolution>,
        live: bool,
    ) {
        ui.horizontal(|ui| {
            // The feature that MADE the entity, so the button deletes
            // what the label names.
            ui.label(format!("{noun} of feature {}", feature.0));
            if live && delete_button(ui, self.session, feature) {
                self.ops.push(SessionOp::DeleteNode { node: feature });
            }
        });
        // The typed verdict, rendered from the resolution machinery's
        // own payload — never a sentence composed here about somebody
        // else's refusal.
        match resolution {
            None => {
                ui.weak("no evaluation yet to resolve this against");
            }
            Some(pncad::select::Resolution::Resolved(_)) => {}
            Some(pncad::select::Resolution::Failed(failure)) => {
                ui.colored_label(
                    chrome(self.theme.unresolved),
                    format!("this {noun} is gone: {}", failure.error),
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
                    chrome(self.theme.unresolved),
                    format!("this {noun} cannot be resolved right now: {cause:?}"),
                );
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
                // ONE modal tool at a time: both tools consume the
                // same selection stream, and two open at once would
                // fill a mate seat and a revolve seat with one click.
                *self.revolve_tool = None;
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

    /// The creation section (GAUTH-1): the add-datum, add-profile and
    /// extrude forms plus the modal revolve tool. Each form is
    /// minimal — its few required fields with sensible defaults — and
    /// emits exactly one creation op; the property panel is the
    /// editor for everything after the insert.
    fn create_ui(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Add feature", |ui| {
            self.add_datum_ui(ui);
            ui.separator();
            self.add_profile_ui(ui);
            ui.separator();
            self.extrude_ui(ui);
            ui.separator();
            self.revolve_tool_ui(ui);
        });
        ui.separator();
    }

    /// The add-datum form: one kind choice, two vector rows, one
    /// [`SessionOp::AddDatum`] on commit.
    fn add_datum_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("datum");
            for (kind, label) in DatumKind::ALL {
                ui.radio_value(&mut self.drafts.datum_kind, kind, label);
            }
        });
        let kind = self.drafts.datum_kind;
        vec3_row(
            ui,
            match kind {
                DatumKind::Point => "position (m)",
                DatumKind::Plane | DatumKind::Axis => "origin (m)",
            },
            &mut self.drafts.datum_origin,
        );
        match kind {
            DatumKind::Plane => vec3_row(ui, "normal", &mut self.drafts.datum_direction),
            DatumKind::Axis => vec3_row(ui, "direction", &mut self.drafts.datum_direction),
            DatumKind::Point => {}
        }
        if ui.button("Add datum").clicked() {
            let datum = match kind {
                DatumKind::Plane => DatumSpec::Plane {
                    origin: self.drafts.datum_origin,
                    normal: self.drafts.datum_direction,
                },
                DatumKind::Axis => DatumSpec::Axis {
                    origin: self.drafts.datum_origin,
                    direction: self.drafts.datum_direction,
                },
                DatumKind::Point => DatumSpec::Point {
                    position: self.drafts.datum_origin,
                },
            };
            self.ops.push(SessionOp::AddDatum { datum });
        }
    }

    /// The add-profile form: a template shape with Length fields, one
    /// [`SessionOp::AddProfile`] on commit — on the world XY plane,
    /// which the form says.
    ///
    /// The circle's optional bore is what lets this template author
    /// the hollow ring's annulus (one profile node, two loops); the
    /// face-frame placement arm is deferred as a filed issue — the
    /// interrogation vocabulary deliberately answers no "is this face
    /// planar" verdict for it to gate on.
    ///
    /// The bore field is guarded IN THE FORM: loop roles come from
    /// the profile layer's containment forest, not from list order,
    /// so a bore at or beyond the outer radius would not refuse — it
    /// would silently swap which circle is the hole. The form says
    /// "bore", so it disables Create until the bore is smaller, with
    /// the reason shown. This is a chrome affordance guarding the
    /// template's stated intent; the op stays unjudged and the
    /// kernel's containment rule stays the one home.
    fn add_profile_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("profile");
            for (shape, label) in ShapeKind::ALL {
                ui.radio_value(&mut self.drafts.profile_shape, shape, label);
            }
            ui.weak("on the world XY plane");
        });
        let mut loops = Vec::new();
        let mut blocked: Option<&'static str> = None;
        match self.drafts.profile_shape {
            ShapeKind::Circle => {
                ui.horizontal(|ui| {
                    ui.label("centre (m)");
                    ui.add(
                        egui::DragValue::new(&mut self.drafts.profile_centre[0])
                            .speed(FIELD_DRAG_SPEED),
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.drafts.profile_centre[1])
                            .speed(FIELD_DRAG_SPEED),
                    );
                    ui.label("radius (m)");
                    ui.add(
                        egui::DragValue::new(&mut self.drafts.profile_radius)
                            .speed(FIELD_DRAG_SPEED),
                    );
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.drafts.profile_bored, "with bore");
                    if self.drafts.profile_bored {
                        ui.label("bore radius (m)");
                        ui.add(
                            egui::DragValue::new(&mut self.drafts.profile_bore)
                                .speed(FIELD_DRAG_SPEED),
                        );
                    }
                });
                loops.push(ProfileShape::Circle {
                    centre: self.drafts.profile_centre,
                    radius: self.drafts.profile_radius,
                });
                if self.drafts.profile_bored {
                    if self.drafts.profile_bore >= self.drafts.profile_radius {
                        blocked = Some(
                            "the bore must be smaller than the radius — which loop is the \
                             hole is decided by containment, so a larger bore would swap \
                             the roles rather than refuse",
                        );
                    }
                    loops.push(ProfileShape::Circle {
                        centre: self.drafts.profile_centre,
                        radius: self.drafts.profile_bore,
                    });
                }
            }
            ShapeKind::Rectangle => {
                ui.horizontal(|ui| {
                    ui.label("width (m)");
                    ui.add(
                        egui::DragValue::new(&mut self.drafts.profile_extent[0])
                            .speed(FIELD_DRAG_SPEED),
                    );
                    ui.label("height (m)");
                    ui.add(
                        egui::DragValue::new(&mut self.drafts.profile_extent[1])
                            .speed(FIELD_DRAG_SPEED),
                    );
                });
                loops.push(ProfileShape::Rectangle {
                    width: self.drafts.profile_extent[0],
                    height: self.drafts.profile_extent[1],
                });
            }
        }
        if let Some(reason) = blocked {
            ui.weak(reason);
        }
        if ui
            .add_enabled(blocked.is_none(), egui::Button::new("Add profile"))
            .clicked()
        {
            self.ops.push(SessionOp::AddProfile {
                plane: pncad::profile::SketchPlane::xy(),
                loops,
            });
        }
    }

    /// The extrude form: the current selection is the profile (a tree
    /// pick, or a face pick whose feature is one — `Selection::node`),
    /// one distance field, one [`SessionOp::AddExtrude`] on commit.
    /// A selection that is not a profile refuses typed at the door
    /// and lands on the status line.
    fn extrude_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("extrude");
            ui.label("distance (m)");
            ui.add(egui::DragValue::new(&mut self.drafts.extrude_distance).speed(FIELD_DRAG_SPEED));
        });
        match self.session.selection().node() {
            Some(node) => {
                if ui.button(format!("Extrude feature {}", node.0)).clicked() {
                    self.ops.push(SessionOp::AddExtrude {
                        profile: node,
                        distance: self.drafts.extrude_distance,
                    });
                }
            }
            None => {
                ui.add_enabled(false, egui::Button::new("Extrude"))
                    .on_disabled_hover_text("select the profile to extrude first");
            }
        }
    }

    /// The revolve tool's panel: activation, the two held picks
    /// (profile, then axis), the angle field, and the one committed
    /// edit — the mate tool's chrome shape over node picks.
    fn revolve_tool_ui(&mut self, ui: &mut egui::Ui) {
        // A copy of the small tool value, for the reason the mate
        // panel takes one: the panel reads it while pushing ops and
        // closing the tool; the authoritative copy is only ever
        // replaced whole.
        let Some(tool) = *self.revolve_tool else {
            if ui.button("Revolve tool…").clicked() {
                *self.revolve_tool = Some(RevolveTool::new());
                // ONE modal tool at a time — the mate activation
                // carries the argument.
                *self.mate_tool = None;
            }
            return;
        };
        ui.label("revolve tool: pick the profile, then the axis");
        match (tool.profile(), tool.axis()) {
            (None, _) => {
                ui.weak("no picks yet");
            }
            (Some(profile), None) => {
                ui.weak(format!("profile: feature {}", profile.0));
            }
            (Some(profile), Some(axis)) => {
                ui.weak(format!(
                    "profile: feature {}; axis: feature {}",
                    profile.0, axis.0
                ));
            }
        }
        ui.horizontal(|ui| {
            ui.label("angle (rad)");
            ui.add(egui::DragValue::new(&mut self.drafts.revolve_angle).speed(0.005));
        });
        let mut close = false;
        ui.horizontal(|ui| {
            if ui.button("Commit revolve").clicked() {
                match tool.op(self.drafts.revolve_angle) {
                    // The op is queued; the tool is NOT closed here.
                    // The application closes it when the op actually
                    // COMMITS (`perform_batch`), so a wrong-kind pick
                    // refusing typed at the session door leaves the
                    // held picks in place to correct, instead of
                    // costing both to a refusal.
                    Ok(op) => self.ops.push(op),
                    Err(error) => *self.status = Some(format!("revolve tool: {error}")),
                }
            }
            if ui.button("Cancel").clicked() {
                close = true;
            }
        });
        if close {
            *self.revolve_tool = None;
        }
    }

    /// The `Add part…` door: the open document's own directory,
    /// listed as parts, one click inserting an instance of one.
    ///
    /// **The listing is a snapshot the chooser holds**, not a scan per
    /// frame: opening a workspace reads every `.pncad` header, which is
    /// a click's worth of work and not a frame's. Rescan re-takes it.
    ///
    /// **A door that cannot open says so.** With no backing file there
    /// is no directory to list, and with a directory that will not scan
    /// (duplicate id, unreadable sibling) there is no honest list — so
    /// the chooser opens either way and shows the typed refusal where
    /// the list would be. That is also where a scan refusal belongs
    /// rather than on a tree badge: no node exists yet to badge.
    fn add_part_ui(&mut self, ui: &mut egui::Ui) {
        if self.part_chooser.is_none() {
            if ui
                .button("Add part…")
                .on_hover_text("insert an instance of another document in this one's directory")
                .clicked()
            {
                // NOT part of the one-modal-tool-at-a-time rule the
                // mate and revolve activations keep, deliberately: that
                // rule exists because those two consume the same
                // SELECTION stream, so a pick would fill two seats. A
                // chooser consumes no picks — it reads a directory and
                // emits its op from a button — so it neither closes a
                // pick tool nor is closed by one, and a pick made while
                // it is open lands exactly where it would have.
                *self.part_chooser = Some(PartChooser::opened(self.session));
            }
            return;
        }
        let mut chosen: Option<DocumentId> = None;
        let mut rescan = false;
        let mut close = false;
        if let Some(chooser) = self.part_chooser.as_ref() {
            egui::Window::new("Add part")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    match chooser.dir() {
                        Some(dir) => ui.weak(format!("parts in {}", dir.display())),
                        None => ui.weak("no directory"),
                    };
                    match chooser.offered() {
                        // An EMPTY listing is not "no parts here": a
                        // saved session's own file is in its own
                        // directory, so the only way to scan clean and
                        // find nothing is for that file to have gone
                        // away underneath the session. Say that, since
                        // it is also why the instances already placed
                        // will stop resolving.
                        Ok([]) => {
                            ui.weak(
                                "this directory holds no documents at all — not even the open \
                                 document's own file, which has gone from it",
                            );
                        }
                        Ok(entries) => {
                            for entry in entries {
                                ui.horizontal(|ui| {
                                    // An entry that cannot be picked
                                    // stays VISIBLE and disabled,
                                    // carrying the op's own refusal —
                                    // read off the entry, not minted
                                    // here.
                                    let refusal = entry.refusal();
                                    let mut pick = ui.add_enabled(
                                        refusal.is_none(),
                                        egui::Button::new(entry.file_name()),
                                    );
                                    if let Some(refusal) = refusal {
                                        pick = pick.on_disabled_hover_text(refusal.to_string());
                                    }
                                    if pick.clicked() {
                                        chosen = Some(entry.id);
                                    }
                                    ui.weak(entry.id.to_string());
                                });
                            }
                        }
                        // The refusing layer's own sentence — the
                        // store's or the directory rule's — never one
                        // composed here.
                        Err(refusal) => {
                            ui.label(refusal.to_string());
                        }
                    }
                    ui.horizontal(|ui| {
                        if ui
                            .button("Rescan")
                            .on_hover_text("re-read this directory")
                            .clicked()
                        {
                            rescan = true;
                        }
                        if ui.button("Cancel").clicked() {
                            close = true;
                        }
                    });
                });
        }
        if let Some(id) = chosen {
            // Exactly one committed edit, and the chooser closes with
            // it — the mate tool's shape.
            self.ops.push(SessionOp::AddInstance { id });
            close = true;
        }
        if rescan && let Some(chooser) = self.part_chooser.as_mut() {
            chooser.rescan(self.session);
        }
        if close {
            *self.part_chooser = None;
        }
        ui.separator();
    }

    /// One PANEL ROW: a scalar slot on its own, or a 3-vector's three
    /// components on one line.
    ///
    /// The grouping is `props::SlotGroup`'s and the vocabulary's (see
    /// its docs); this function only lays it out. What the two arms
    /// share — the value field (numbers AND expressions, one widget),
    /// the gesture mapping, the driven affordance, the range probe —
    /// is called once per COMPONENT, so a component of a vector is
    /// edited by exactly the operations a stand-alone slot is.
    ///
    /// **Three lines per group at most, whatever its arity.** Folding
    /// three slots onto one line buys nothing if their doors then take
    /// three lines each, so the range probe is a single line of small
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
                    ui.weak("range");
                    for (axis, row) in Axis3::ALL.iter().zip(rows.iter()) {
                        self.range_button(ui, node, row, axis.label());
                    }
                });
            }
        }
    }

    /// **The one value field: a number AND an expression.**
    ///
    /// It is a `DragValue` — the scrub gesture is the widget's whole
    /// point — wearing a `custom_parser`, which is egui's seam for a
    /// field whose text is not necessarily a number: a parser that
    /// answers `None` rejects the text and leaves the value alone,
    /// which is exactly what an expression needs. So what a user typed
    /// is read once, by `props::field_edit`, and takes one of two
    /// doors: a bare number through `SessionOp::SetSlot`, anything
    /// else — an operator, a parameter, a unit — through
    /// `SessionOp::SetSlotExpression`. The panel parses nothing.
    ///
    /// The field commits on Enter or on leaving it
    /// (`update_while_editing(false)`), never per keystroke: half of
    /// `thickness * 2` is a parse refusal at best and a DIFFERENT
    /// parameter at worst.
    ///
    /// The number is shown in the unit the slot is WRITTEN in and
    /// authored back through the same factor (`props::in_written` /
    /// `props::from_written`, the text door's one-multiply semantics),
    /// with NO unit suffix on the text: the picker beside the field
    /// names the unit, and saying it twice adjacently says it once.
    fn slot_value_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, row: &SlotRow) {
        let unit = props::written_unit(row.dimension, row.unit);
        // A slot that did not evaluate still has SOURCE to edit — it
        // is the slot most likely to need it — so the field is drawn
        // for it too, over the one number it does not have. The fault
        // itself is said beside the field.
        if let Err(ref error) = row.value {
            ui.weak(format!("{error}"));
        }
        let mut number = props::in_written(
            match row.value {
                Ok(value) => value.as_f64(),
                Err(_) => 0.0,
            },
            unit,
        );
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
        // What the field says, when that is not the dragged number:
        // the text a parse refusal handed back, else the slot's own
        // source. A LITERAL slot with a value shows no fixed text at
        // all — egui formats the number it is dragging, and a text
        // pinned from the row would freeze the field mid-gesture.
        let fixed = if self.drafts.expr_target == Some((node, row.slot)) {
            Some(self.drafts.expr_text.clone())
        } else if row.driver.is_driven() || row.value.is_err() {
            Some(props::field_text(row))
        } else {
            None
        };
        // The parser runs inside `ui.add`, so what it read comes back
        // out through a cell rather than a return value.
        let typed: core::cell::RefCell<Option<props::FieldEdit>> = core::cell::RefCell::new(None);
        let mut widget = egui::DragValue::new(&mut number)
            .speed(speed)
            .update_while_editing(false)
            .custom_parser(|text| match props::field_edit(text) {
                props::FieldEdit::Number(value) => {
                    *typed.borrow_mut() = Some(props::FieldEdit::Number(value));
                    Some(value)
                }
                // Rejected as a number, which is what routes it to the
                // expression door and leaves the field's value where
                // it was until the document answers.
                edit => {
                    *typed.borrow_mut() = Some(edit);
                    None
                }
            });
        if let Some(text) = fixed {
            widget = widget.custom_formatter(move |_, _| text.clone());
        }
        let widget = ui.add(widget);
        drag_gesture_ops(
            &widget,
            props::from_written(number, unit),
            SessionOp::BeginGesture {
                node,
                slot: row.slot,
            },
            |value| SessionOp::PreviewGesture { value },
            SessionOp::CommitGesture,
            self.ops,
        );
        // **Text that says what the slot already says is not an
        // edit.** The field commits on leaving it, so clicking into
        // one and clicking away again must not cost an undo step. A
        // DRIVEN slot is exempt for the number arm: writing a number
        // over a computation is the refusal's own case, and it is owed
        // its affordance even when the number happens to match.
        match typed.into_inner() {
            Some(props::FieldEdit::Number(written)) => {
                let value = SlotValue::of(row.dimension, props::from_written(written, unit));
                if row.driver.is_driven() || row.value != Ok(value) {
                    self.ops.push(SessionOp::SetSlot {
                        node,
                        slot: row.slot,
                        value,
                    });
                }
            }
            Some(props::FieldEdit::Expression(text)) => {
                if row.source.as_deref() != Some(text.as_str()) {
                    self.ops.push(SessionOp::SetSlotExpression {
                        node,
                        slot: row.slot,
                        text,
                    });
                }
            }
            // An emptied field is not an edit: there is no expression
            // it could mean, and blanking a dimension is not a way to
            // delete anything in this vocabulary.
            Some(props::FieldEdit::Empty) | None => {}
        }
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
            // Wide enough for the longest symbol the table carries
            // (`pi rad`) plus the combo's arrow.
            .selected_text(label)
            .width(72.0)
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

    /// The δ control: the display tolerance as a number the user types,
    /// in millimetres.
    ///
    /// **A text field rather than a pair of step buttons.** δ is a
    /// LENGTH, and the question a user has is "how fine, in mm" — a
    /// halve/double pair answers it only by repeated clicking and
    /// cannot reach a number in between. It is also not a `DragValue`:
    /// a drag would commit a tessellation per frame, which is the one
    /// mistake [`drag_ops`] exists to keep out of this file.
    ///
    /// Committing on lost focus covers Enter too — egui's singleline
    /// field surrenders focus on Enter — so there is one commit path,
    /// not two. What is typed is a DRAFT until then: nothing
    /// re-tessellates while a number is half-entered, and `0.` on the
    /// way to `0.05` never reaches the tessellator.
    fn delta_ui(&mut self, ui: &mut egui::Ui) {
        let in_force = self.delta.get() * 1.0e3;
        let field = ui
            .horizontal(|ui| {
                let text = self
                    .drafts
                    .delta_mm
                    .get_or_insert_with(|| format!("{in_force:.3}"));
                let field = ui.add(egui::TextEdit::singleline(text).desired_width(56.0));
                ui.label("mm display δ");
                field
            })
            .inner;
        if field.lost_focus()
            && let Some(typed) = self.drafts.delta_mm.take()
        {
            match typed.trim().parse::<f64>() {
                // Judged by `DisplayTolerance`, not here: a δ that is
                // not a finite positive length is refused at that one
                // door, wherever it came from.
                Ok(mm) => *self.delta_request = Some(mm * 1.0e-3),
                Err(error) => {
                    *self.status = Some(format!(
                        "display δ: {:?} is not a number ({error})",
                        typed.trim()
                    ));
                }
            }
        }
        // The draft lives exactly as long as the focus does, so a δ
        // that moved under the field (the budget's choice on open)
        // shows up in it.
        if !field.has_focus() {
            self.drafts.delta_mm = None;
        }
    }

    /// The view pane: the numbers the camera and the tessellation are
    /// actually running at.
    fn view_ui(&mut self, ui: &mut egui::Ui) {
        let stats = self.scene.stats();
        ui.heading("View");
        // One δ, because there is only ever one: the budget chose it
        // when the document opened or the user did, and the note says
        // which. The triangle count below is the picture's own, so
        // nothing here is a prediction.
        self.delta_ui(ui);
        if self.budget_delta.is_some() {
            ui.weak("chosen for the triangle budget; δ is yours from here");
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
///
/// The DRAG half is [`drag_gesture_ops`], which the slot field calls
/// directly: a field that reads its own text decides for itself what
/// was typed, so `changed()` is not what tells it.
fn drag_ops(
    widget: &egui::Response,
    value: f64,
    begin: SessionOp,
    preview: impl Fn(f64) -> SessionOp,
    commit: SessionOp,
    typed: impl Fn(f64) -> Vec<SessionOp>,
    ops: &mut Vec<SessionOp>,
) {
    if drag_gesture_ops(widget, value, begin, preview, commit, ops) {
        return;
    }
    if widget.changed() && !widget.dragged() {
        // Typed, not dragged: whatever the vocabulary spells a direct
        // value entry as — one edit for a document slot, a one-shot
        // begin/preview/commit for the display probe.
        ops.extend(typed(value));
    }
}

/// The drag half of [`drag_ops`]'s triple: begin on press, preview on
/// every frame the value moves, commit on release. Answers whether the
/// release happened, i.e. whether this frame's change was a gesture's
/// and belongs to nothing else.
fn drag_gesture_ops(
    widget: &egui::Response,
    value: f64,
    begin: SessionOp,
    preview: impl Fn(f64) -> SessionOp,
    commit: SessionOp,
    ops: &mut Vec<SessionOp>,
) -> bool {
    if widget.drag_started() {
        ops.push(begin);
    }
    if widget.dragged() && widget.changed() {
        ops.push(preview(value));
    }
    if widget.drag_stopped() {
        ops.push(commit);
        return true;
    }
    false
}

/// One labeled row of three draggable components — the add-datum
/// form's vector fields.
fn vec3_row(ui: &mut egui::Ui, label: &str, value: &mut [f64; 3]) {
    ui.horizontal(|ui| {
        ui.label(label);
        for component in value {
            ui.add(egui::DragValue::new(component).speed(FIELD_DRAG_SPEED));
        }
    });
}

/// The delete button: a renderer for [`DocSession::delete_affordance`]
/// and nothing else, so the two places a delete is reachable from (a
/// node selection and a face selection) cannot state different costs
/// for the same operation, and the sentence itself is testable without
/// a window.
fn delete_button(ui: &mut egui::Ui, session: &DocSession, node: RecipeNodeId) -> bool {
    let affordance = session.delete_affordance(node);
    let button = ui.button(affordance.label);
    match affordance.hover {
        Some(text) => button.on_hover_text(text).clicked(),
        None => button.clicked(),
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

/// The container holding the feature tree and the properties — the
/// document's MODEL, as against the View pane's display settings.
///
/// Found by content rather than remembered by id: the layout is a
/// value the user rearranges, so a stored id would sooner or later
/// name a tile a drag had dissolved. `None` says the two panes are
/// not currently stacked together, which is a layout the user is
/// entitled to and which nothing here needs to name.
pub fn model_stack(tiles: &Tiles<Pane>) -> Option<TileId> {
    let features = tiles.find_pane(&Pane::Features)?;
    let properties = tiles.find_pane(&Pane::Properties)?;
    tiles.tile_ids().find(|&id| {
        matches!(tiles.get(id), Some(Tile::Container(container))
            if container.has_child(features) && container.has_child(properties))
    })
}

/// The toolbar's name for the open document: the file stem of the
/// path it is saved at, or [`UNTITLED`] while it has none.
///
/// The STEM, not the full path — the path is already shown in full in
/// the View pane, and a toolbar is where a user glances for which of
/// several documents they are in. A path whose bytes are not UTF-8 is
/// shown lossily rather than dropped: a name with a replacement
/// character in it still identifies the document.
pub fn document_name(path: Option<&std::path::Path>) -> String {
    path.and_then(std::path::Path::file_stem).map_or_else(
        || UNTITLED.to_owned(),
        |stem| stem.to_string_lossy().into_owned(),
    )
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
