//! The eframe application: docked chrome around the wgpu viewport,
//! with the document panels beside it.
//!
//! # What this module is allowed to contain
//!
//! Toolkit adaptation, and nothing else. It turns egui pointer state
//! into [`ViewportEvent`](crate::input::ViewportEvent)s and into
//! [`SessionOp`]s, hands both to the renderer-free half of this crate,
//! and paints what comes back. Every decision it takes is a call into
//! that half, which is what makes the navigation and the editing
//! testable without a window (G1) — and what makes the seam reading in
//! the PR body a statement about egui rather than about our code.
//!
//! What the panels OFFER is not toolkit adaptation and is not here:
//! the authoring vocabularies are [`crate::forms`], the in-flight form
//! state is [`crate::drafts`], the free helpers over `egui::Ui` are
//! [`crate::widgets`], and the pane bodies are [`crate::pane`]
//! (`crates/viewer/README.md`, Module boundaries). What stays is
//! [`ViewerApp`], [`ViewerBehavior`], the frame loop and the entry
//! points.
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
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use editor_core::appearance::Rgba8;
use eframe::egui;
use egui_tiles::{ContainerKind, EditAction, Tile, TileId, Tiles, Tree, UiResponse};
use pncad::geom_core::Tol;

use crate::camera::{self, Camera};
use crate::display::DisplayView;
use crate::drafts::Drafts;
use crate::evalseam::Generation;
#[cfg(not(target_family = "wasm"))]
use crate::evalseam::ThreadEvaluator;
use crate::frame::{self, IdQueryLog, StatusUpdate};
use crate::gpu::{DEPTH_BITS, ViewportRenderer};
use crate::input::InputMap;
use crate::parts::PartChooser;
use crate::pick::{self, PickCache, PickIndex};
use crate::prefs::{self, Prefs, PrefsStore};
use crate::scene::{self, DisplayTolerance, SceneMesh};
use crate::session::{DocSession, Refusal, Selection, SessionOp};
use crate::sketch::{self, PreviewError, ProfilePreview};
use crate::theme::{Polarity, Theme};
use crate::tools::Tools;

// The one re-export this module carries: `tests/panel_display.rs`
// reaches the field-writing value by the `viewer::app::FieldWriting`
// path to assert the unit/tick pair on it.
pub use crate::forms::FieldWriting;

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

/// A container's tab title, in words.
///
/// A tab title is prose a person reads, so the layout vocabulary is
/// spelled here rather than taken from `ContainerKind`'s `Debug`. The
/// match is exhaustive over a foreign enum on purpose: a kind added
/// upstream breaks this build instead of quietly reaching a user as a
/// type identifier, which is the guarantee `Debug` cannot give whether
/// or not the new kind carries a field.
fn container_kind_title(kind: ContainerKind) -> &'static str {
    match kind {
        ContainerKind::Tabs => "Tabs",
        ContainerKind::Horizontal => "Columns",
        ContainerKind::Vertical => "Rows",
        ContainerKind::Grid => "Grid",
    }
}

/// The status line for a referent the resolution machinery cannot
/// place right now.
///
/// The cause is rendered through its OWN `Display`: the layer that
/// raised the indeterminacy names it, and this one contributes only
/// the noun it is talking about. Named rather than composed inside the
/// render pass so the wording has one home and can be asserted on.
pub fn indeterminate_wording(noun: &str, cause: &editor_core::ResolveIndeterminate) -> String {
    format!("this {noun} cannot be resolved right now: {cause}")
}

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
pub(crate) fn chrome(color: Rgba8) -> egui::Color32 {
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

/// **The glyphs the chrome draws, and the rule they all obey.**
///
/// egui bundles its own fonts, and the PROPORTIONAL family is exactly
/// three: `Ubuntu-Light`, `NotoEmoji-Regular` and `emoji-icon-font`.
/// A character in none of them is not approximated — it is drawn as
/// the missing-glyph box, on the user's screen, with nothing anywhere
/// reporting it. That is what these four used to be: `✕`, `▲`, `▼`
/// and `▸` are all absent from that stack (`▲`/`▼` exist only in
/// `Hack-Regular`, which is the MONOSPACE family and never reached by
/// a button label), so every row of the path form carried three empty
/// boxes and every product root in the feature tree carried a fourth.
///
/// They are named here rather than spelled at each use so the rule has
/// somewhere to be written down: **a glyph added to this list must
/// exist in that stack.** `×` is Latin-1 and lives in Ubuntu-Light;
/// `⬆`, `⬇` and `»` are in the emoji fonts and Ubuntu-Light
/// respectively.
pub(crate) const GLYPH_REMOVE: &str = "×";
/// Move a list row earlier — see [`GLYPH_REMOVE`] for the font rule.
pub(crate) const GLYPH_UP: &str = "⬆";
/// Move a list row later — see [`GLYPH_REMOVE`] for the font rule.
pub(crate) const GLYPH_DOWN: &str = "⬇";
/// Marks a product root in the feature tree — see [`GLYPH_REMOVE`].
pub(crate) const GLYPH_ROOT: &str = "»";

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
    /// **The modal tools, at most one open** — the mate tool, the
    /// revolve tool and the four combining tools as one value, with
    /// the exclusivity rule inside it rather than spread across the
    /// activation sites ([`crate::tools::Tools`]).
    tools: Tools,
    /// The `Add part…` chooser, when open. `None` is "no chooser"; the
    /// scanned catalogue it is showing lives inside it, taken once when
    /// it opened.
    ///
    /// NOT one of [`Tools`]: the exclusivity rule there is about the
    /// selection stream, and a chooser consumes none — it picks a
    /// document out of a list.
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
    /// Whether the add-profile form was DRAWN last frame — the
    /// latch that decides whether a profile preview is computed and
    /// drawn at all.
    ///
    /// A latch and not a question asked directly, because the form is
    /// inside a collapsing section inside a pane inside a tiled
    /// layout, and whether it is on screen is a fact only the frame
    /// that drew it knows. It costs one frame at each end: the
    /// wireframe appears the frame after the section is opened and
    /// leaves the frame after it is closed, which is the same
    /// staleness the preview itself carries and for the same reason.
    profile_form_drawn: bool,
    /// Whether the advisory-check findings window is open.
    ///
    /// Application chrome state, not a draft: nothing is being
    /// composed, and closing the window abandons nothing. It survives
    /// re-evaluation on purpose — a window opened to read a finding
    /// should still be open when the edit made to answer it lands, so
    /// the reader can see whether the finding went away.
    checks_shown: bool,
    /// A fit is owed, and will be taken by the viewport pane on the
    /// next frame — the only place that knows the real aspect.
    pending_fit: bool,
    /// **Whether the viewport draws the document's datums.**
    ///
    /// A VIEW setting and not a document one, and the line is the same
    /// one `DisplayState`'s hide is on the other side of: which datums
    /// exist is the recipe's business, and whether this window draws
    /// them is this window's. It is not persisted for that reason
    /// either — a preference file holds what a person chose about the
    /// application, and this is what they chose about a glance.
    ///
    /// On by default: a feature nobody can see is a feature nobody
    /// finds, and construction geometry is most wanted exactly when it
    /// has just been authored.
    show_datums: bool,
    /// A fit is owed as soon as the NEXT rebuilt scene lands — set by
    /// a successful `Open`, whose document arrives asynchronously, so
    /// fitting immediately would frame the outgoing picture.
    fit_on_scene: bool,
    /// The last thing that went wrong, kept so a refused operation is
    /// visible instead of silently dropped.
    status: Option<String>,
    /// **What THIS frame has to say that is not a refusal** — the open
    /// tool's declined picks and survival drops, and the free-move
    /// placements the frame's own operations superseded — collected as
    /// they happen and applied with the batch verdict rather than
    /// before it.
    ///
    /// They cannot be written straight to [`ViewerApp::status`]: the
    /// batch of the same frame is performed afterwards, and a clean
    /// acting batch clears the line, so a notice assigned early lives
    /// for zero frames (`frame::frame_status` carries the argument).
    /// Drained every frame by `perform_batch`, so nothing here
    /// survives into the next one.
    notices: Vec<String>,
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
            tools: Tools::new(),
            part_chooser: None,
            profile_form_drawn: false,
            checks_shown: false,
            camera,
            input,
            theme,
            tree: initial_layout(),
            split_dragged: false,
            drafts: Drafts::default(),
            pending_fit: true,
            show_datums: true,
            fit_on_scene: false,
            // Whatever the preferences file had to say, in the one
            // place this crate puts a thing that went wrong.
            status: (!notices.is_empty()).then(|| notices.join(frame::NOTICE_SEPARATOR)),
            notices: Vec::new(),
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
        // **The survival step, once per frame, for whichever tool is
        // open** — the obligation every tool's module docs put on its
        // consumer, discharged in the one place that holds them all.
        // Each notice already names its tool.
        let dropped = self
            .tools
            .reconcile(self.session.doc(), self.session.landed_pair());
        self.notices.extend(dropped.iter().map(ToString::to_string));
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
                // The gather's own verdict is NOT written here. A
                // naming collision across roots is not a node failure,
                // so no tree badge carries it — but it is a standing
                // fact about the landed pair, not this frame's news,
                // and the status line carries news
                // (`frame`'s header). It badges beside the at-rest and
                // checks reads, off `frame::product_badge`, which is a
                // read of held state and so cannot be stale here or
                // erased by anything the rest of the frame does.
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
        // **No early return on an empty batch.** The frame's tool
        // notices are applied here, and a frame that produced one
        // without queueing an op — a survival drop on a document the
        // seam just landed — is exactly the frame that needs its
        // notice shown.
        let mut notices = core::mem::take(&mut self.notices);
        if ops.is_empty() && notices.is_empty() {
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
            let tool_edit = self.tools.commits_open_tool(&op);
            let outcome = self.session.perform(op);
            // **Where a supersession reaches the user.** The session
            // reports which free-move placements its document
            // transition discarded; this is the one read of that field
            // outside the test suite, and it goes onto the frame's
            // notices rather than onto the line, because the accepted
            // edit that caused it is about to answer `Clear`
            // (`frame::supersession_notice` carries the argument).
            notices.extend(frame::supersession_notice(&outcome.superseded));
            match outcome.refusal {
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
                // A modal tool closes when its edit actually
                // COMMITS, not when its button is clicked — a refusal
                // leaves the tool open with its picks held, to be
                // corrected (each tool panel's commit arm carries the
                // other half of this rule). One open tool at a time is
                // what lets this close "the" tool without asking which
                // op came from which panel.
                None if tool_edit => self.tools.close(),
                None => {}
            }
        }
        let update = frame::frame_status(&notices, &performed, refusal.as_ref());
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

    /// This application's door onto [`frame::apply`]: the batch policy
    /// and the dialog policy hand their verdict here rather than
    /// assigning the field.
    ///
    /// **Not the one place a [`StatusUpdate`] becomes the field** —
    /// that is `frame::apply`, and `pane::viewport::land` reaches it
    /// directly, having a `&mut Option<String>` and no `&mut self` to
    /// come through. This is the `&mut self` shorthand, nothing more.
    fn apply_status(&mut self, update: StatusUpdate) {
        frame::apply(&mut self.status, update);
    }

    /// **The advisory-check findings, in a window a reader can keep
    /// open.**
    ///
    /// The badge in the toolbar says how many there are; this says
    /// what they are. Each finding renders through its OWN `Display`
    /// — one composed sentence carrying its subject, its story and
    /// its recourse (`editor_core::finding`) — so the window states
    /// no vocabulary of its own, and beside it sits the one thing
    /// chrome can add: a jump to the root the finding is about, which
    /// is the feature a reader would otherwise have to find by
    /// counting rows.
    ///
    /// The report's SKIPPED checks are shown too, for the reason the
    /// report carries them: "checked and fine" and "not checked" are
    /// different answers, and a window that showed only findings
    /// would let the second read as the first.
    ///
    /// The window is drawn while it is open even when the findings
    /// have gone — an edit answering a finding is exactly when
    /// somebody is looking — so it says so rather than emptying
    /// silently.
    fn checks_window(&mut self, ctx: &egui::Context, ops: &mut Vec<SessionOp>) {
        if !self.checks_shown {
            return;
        }
        let mut open = true;
        egui::Window::new("Checks")
            .open(&mut open)
            .default_width(420.0)
            .show(ctx, |ui| match self.session.checks() {
                None => {
                    ui.label("nothing has been checked yet");
                }
                Some(report) => {
                    if report.findings.is_empty() {
                        ui.label("checks: no findings");
                    }
                    for finding in &report.findings {
                        ui.horizontal_top(|ui| {
                            if ui
                                .button(format!("feature {}", finding.root.0))
                                .on_hover_text("select the root this finding is about")
                                .clicked()
                            {
                                ops.push(SessionOp::Select(Selection::Node(finding.root)));
                            }
                            ui.label(finding.to_string());
                        });
                    }
                    if !report.skipped.is_empty() {
                        ui.separator();
                        ui.weak(format!(
                            "not run (severity Off): {}",
                            report
                                .skipped
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(", ")
                        ));
                    }
                }
            });
        self.checks_shown = open;
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
                    // **A button, not a label.** The findings were
                    // reachable only by hovering the badge, which is
                    // a poor home for text a reader needs to keep
                    // open while they act on it: a tooltip is gone
                    // the moment the pointer moves toward the feature
                    // it names. The badge opens the window instead,
                    // and the window is where the sentences live.
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new(format!(
                                    "checks: {} finding(s)",
                                    report.findings.len()
                                ))
                                .color(chrome(self.theme.unresolved)),
                            )
                            .frame(false),
                        )
                        .on_hover_text("show what the checks found")
                        .clicked()
                    {
                        self.checks_shown = !self.checks_shown;
                    }
                }
                // The gather's verdict, for the landed pair. **A
                // standing fact, so a badge** — the status line beside
                // it carries one frame's news and is cleared by the
                // next acting batch, while "the product on screen does
                // not gather" is true until another pair lands.
                //
                // Drawn in the unresolved colour, like the at-rest
                // refusal and the checks findings and unlike the
                // budget's weak advisory below: the colour is
                // REDUNDANT here, in `Theme::unresolved`'s own sense —
                // the badge says "product: …" in words either way — and
                // it is the spelling this toolbar already uses for a
                // verdict a reader may need to act on.
                //
                // Which faults reach it is `frame::product_badge`'s,
                // and it declines every state another channel carries:
                // the three per-node arms are the feature tree's, and
                // an empty document is the blank viewport's.
                if let Some(fault) = frame::product_badge(self.session.product_fault()) {
                    ui.separator();
                    ui.colored_label(chrome(self.theme.unresolved), fault);
                }
                // The display budget's badge: shown while the δ on
                // screen is the one the budget CHOSE when the document
                // opened, and gone the moment the user picks their
                // own. A read of held state, like the badges above,
                // which is why it is here rather than in the status
                // line below — that line carries one frame's news, and
                // "this δ was chosen for you" has to outlive a mouse
                // drag.
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
        // **The preview is taken once, before the panes draw**, from
        // the drafts as they stand — so the panel's refusal line and
        // the viewport's wireframe are two views of one replay rather
        // than two replays that could disagree.
        //
        // It is therefore one frame behind an edit made DURING this
        // frame, which the repaint request below closes: the loops
        // are compared afterwards and a frame that changed them asks
        // for another, so the picture catches up on the next one
        // rather than waiting for the next input event.
        let authored = self.drafts.profile_loops();
        // **No frame picked, no preview.** The form draws on a frame
        // the document holds, so with none picked there is no plane to
        // place the loops on and nothing honest to show — the form
        // says what it is waiting for instead, exactly as it does for
        // a shape nobody chose.
        let preview_plane = self
            .drafts
            .profile_plane
            .zip(self.session.landed_pair())
            .and_then(|(frame, (doc, evaluation))| sketch::frame_placement(doc, evaluation, frame));
        let profile_preview = self
            .profile_form_drawn
            .then_some(preview_plane)
            .flatten()
            .map(|plane| sketch::preview(plane, &authored, self.session.tol(), self.delta.get()));
        let mut profile_form_drawn = false;
        let mut delta_request: Option<f64> = None;
        let mut features_content_height: Option<f32> = None;
        let mut split_dragged = self.split_dragged;
        // **The tiles stand on the chrome's own ground.** `no_frame`
        // alone gives the panes no background at all, which does not
        // leave them transparent onto something sensible: it leaves
        // them on eframe's window clear colour, a near-black constant
        // (`App::clear_color`'s default) that no `Visuals` ever
        // touches. Every pane in the side panel was therefore drawn on
        // black under all three palettes, so the light themes put dark
        // text on it and could not be read.
        //
        // `Frame::NONE.fill(panel_fill)` is the narrow fix: NONE keeps
        // the zero margin the tiles need to reach the window edge, and
        // `panel_fill` is the same ground the toolbar above them
        // already stands on — so the chrome follows [`Polarity`]
        // through the toolkit's visuals, exactly as
        // [`apply_polarity`] intends, instead of one panel escaping it.
        egui::CentralPanel::no_frame()
            .frame(egui::Frame::NONE.fill(ui.visuals().panel_fill))
            .show(ui, |ui| {
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
                    tools: &mut self.tools,
                    part_chooser: &mut self.part_chooser,
                    profile_preview: &profile_preview,
                    profile_form_drawn: &mut profile_form_drawn,
                    pending_fit: &mut self.pending_fit,
                    status: &mut self.status,
                    id_answer: &self.id_answer,
                    id_log: &mut self.id_log,
                    ops: &mut ops,
                    delta_request: &mut delta_request,
                    features_content_height: &mut features_content_height,
                    split_dragged: &mut split_dragged,
                    show_datums: &mut self.show_datums,
                };
                self.tree.ui(&mut behavior, ui);
            });
        self.checks_window(ui.ctx(), &mut ops);
        self.profile_form_drawn = profile_form_drawn;
        // An edit made while the panes drew leaves the preview a
        // frame behind. Asking for a repaint is what makes that one
        // frame rather than "until the next input event".
        if profile_form_drawn && self.drafts.profile_loops() != authored {
            ui.ctx().request_repaint();
        }
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

        // The open tool consumes the selection vocabulary: a pick this
        // frame produced is ALSO held as a tool pick (the
        // two-sequential-picks ruling — the same single-select value,
        // copied into tool state). Which vocabulary each tool reads is
        // `Tools::feed`'s to know, and a pick a tool DECLINED comes
        // back as a notice shown exactly as a survival drop is.
        let declined = self.tools.feed(self.session.doc(), &ops);
        self.notices
            .extend(declined.iter().map(ToString::to_string));

        self.perform_batch(ops);
    }

    /// What the window is cleared to before a single panel paints.
    ///
    /// The trait's default is a hard-coded near-black at 180/255
    /// alpha — a constant that does not read the `Visuals` it is
    /// handed, so it stays black under a light palette and
    /// translucent under every one. Both are wrong here: the chrome
    /// states its ground through [`Polarity`], and a viewer that let
    /// the desktop show through its panels would be reporting a
    /// transparency nobody asked for.
    ///
    /// The same `panel_fill` the central panel above fills with, so
    /// the clear and the panel agree and no seam can appear between
    /// them.
    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        visuals.panel_fill.to_normalized_gamma_f32()
    }
}

/// The `Behavior` egui_tiles renders panes through: a borrow of the
/// application's state for the duration of one frame.
pub(crate) struct ViewerBehavior<'a> {
    pub(crate) session: &'a DocSession,
    /// The δ the picture is drawn at.
    pub(crate) delta: DisplayTolerance,
    /// Set while `delta` is the one the triangle budget chose when the
    /// document opened, rather than one the user picked.
    pub(crate) budget_delta: Option<crate::scene::FittedDelta>,
    pub(crate) scene: &'a Arc<SceneMesh>,
    pub(crate) index: Option<&'a PickIndex>,
    pub(crate) revision: u64,
    pub(crate) camera: &'a mut Camera,
    pub(crate) input: InputMap,
    /// The palette this frame draws with; `Copy`, because a theme is
    /// a small value and the frame must not be able to change it.
    pub(crate) theme: Theme,
    pub(crate) drafts: &'a mut Drafts,
    /// The display snapshot this frame draws and picks under.
    pub(crate) display: &'a DisplayView,
    /// The modal tools, at most one open.
    pub(crate) tools: &'a mut Tools,
    /// The `Add part…` chooser, if open.
    pub(crate) part_chooser: &'a mut Option<PartChooser>,
    /// What the add-profile form's loops would draw, taken once for
    /// the frame: the panel says what it refuses and the viewport
    /// draws what it replayed, from ONE reading.
    ///
    /// `None` is "no preview was taken this frame" — the frame the
    /// form first comes on screen, before the latch below has told
    /// anyone to take one. Distinct from `Some(Ok(empty))`, which is
    /// a preview that WAS taken and drew nothing, and which the form
    /// is entitled to say so about.
    pub(crate) profile_preview: &'a Option<Result<ProfilePreview, PreviewError>>,
    /// Set by the add-profile form while it draws; read next frame.
    pub(crate) profile_form_drawn: &'a mut bool,
    pub(crate) pending_fit: &'a mut bool,
    pub(crate) status: &'a mut Option<String>,
    pub(crate) id_answer: &'a Arc<AtomicU64>,
    pub(crate) id_log: &'a mut IdQueryLog,
    pub(crate) ops: &'a mut Vec<SessionOp>,
    /// A δ the View pane's field committed this frame, in world units.
    /// The pane holds a borrow of the app, not the app, so it hands
    /// the number back for [`ViewerApp::set_delta`] to judge.
    pub(crate) delta_request: &'a mut Option<f64>,
    /// What the Features pane's content laid out to this frame, once
    /// it has drawn.
    pub(crate) features_content_height: &'a mut Option<f32>,
    /// Set when the user resized a tile themselves.
    pub(crate) split_dragged: &'a mut bool,
    /// Whether the viewport draws datums ([`ViewerApp::show_datums`]);
    /// the View pane's checkbox writes through it.
    pub(crate) show_datums: &'a mut bool,
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
            Some(Tile::Container(container)) => container_kind_title(container.kind()).into(),
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
        // **Both axes**, not just the vertical. A row of this
        // chrome is as wide as the controls on it — a path step's verb
        // decides how many fields follow it — so a pane that scrolled
        // only downward clipped the right-hand end of its widest rows
        // with nothing to reach them by. That is the same failure
        // first light found downward (#1097), in the other direction.
        let scrolled = egui::ScrollArea::both()
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
pub(crate) fn to_f32(matrix: &[[f64; 4]; 4]) -> [[f32; 4]; 4] {
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
    /// said. The one arm on this crate's surface that cannot forward
    /// to its payload's own words: the platform hands back a
    /// `JsValue`, which implements no `Display`, and the orphan rule
    /// puts writing one out of this crate's reach. The captured text
    /// is therefore a `Debug` rendering, taken at the seam.
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
        // The one payload on this crate's surface that CANNOT forward:
        // `JsValue` is `wasm-bindgen`'s, it implements no `Display`,
        // and the orphan rule forecloses writing one here. `Debug` is
        // the honest rendering — `as_string()` is not the alternative,
        // because it answers `None` for every non-string `JsValue` and
        // would drop the browser's message entirely.
        .map_err(|error| WebStartupError::Runner(format!("{error:?}")))
}
