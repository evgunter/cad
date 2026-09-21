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

use crate::camera::{self, Camera, CameraError};
use crate::display::DisplayView;
use crate::drafts::{Drafts, ProfileDoors};
use crate::evalseam::FitService;
#[cfg(not(target_family = "wasm"))]
use crate::evalseam::ThreadEvaluator;
use crate::frame::{self, StatusUpdate};
use crate::gpu::{DEPTH_BITS, ViewportRenderer};
use crate::idpass::IdQueryLog;
use crate::input::InputMap;
use crate::marks;
use crate::parts::PartChooser;
use crate::pickcache::{self, PickCache};
use crate::pickindex::{PickIndex, PictureKey};
use crate::platform;
use crate::prefs::{self, Prefs, PrefsStore};
use crate::scene::{self, DisplayTolerance, SceneError, SceneMesh};
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
/// until a `web_sys::Storage` store is written.
///
/// **The `cfg` decides which store answers, never whether it can keep
/// anything.** Both arms can answer [`prefs::PrefsStore::unusable`]
/// with `Some`: `Absent` always does, and the native `FileStore` does
/// in an environment that names no config directory, which is
/// [`platform::prefs_path`]'s `None`. So everything the chrome does about
/// a store that keeps nothing keys on that read and not on the target
/// — which is what makes the browser and a desktop launched from a
/// stripped environment one case, and what leaves a future
/// `web_sys::Storage` store out of the case on its own.
#[cfg(not(target_family = "wasm"))]
type Store = prefs::file::FileStore;
#[cfg(target_family = "wasm")]
type Store = prefs::Absent;

/// This build's store.
fn prefs_store() -> Store {
    #[cfg(not(target_family = "wasm"))]
    {
        // The path comes from `frame`, the crate's one ambient door.
        prefs::file::FileStore::new(platform::prefs_path())
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
///
/// **Native only**, `cfg`-ed to match its one reader: [`run`] hands it
/// to `eframe::run_native` as the window's name, and a browser has no
/// window to name — the page's `<title>` is what a user reads there,
/// and `run_web` — absent from this configuration, so named rather
/// than linked — is handed a canvas rather than a title.
#[cfg(not(target_family = "wasm"))]
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
/// [`crate::theme::linear`]'s role on the viewport side: a palette states sRGB
/// and each renderer converts once, at its own door.
pub(crate) fn chrome(color: Rgba8) -> egui::Color32 {
    egui::Color32::from_rgb(color.r, color.g, color.b)
}

/// `text` in the weight or colour its [`frame::Tone`] asks for.
///
/// **The one place the tone-to-chrome mapping is made.** Its two
/// readers are the toolbar's badge family ([`draw_badge`]) and the
/// feature tree's row badge, which reads the same tone off
/// [`crate::tree::RowStatus::tone`] — two families, one rule, so what
/// `Advisory` looks like is changed here or nowhere.
///
/// [`crate::theme::Theme::unresolved`]'s contract is that the colour is
/// REDUNDANT — everything wearing it says its own words — so this
/// decides salience and never meaning.
pub(crate) fn toned(text: impl Into<String>, theme: &Theme, tone: frame::Tone) -> egui::RichText {
    let text = egui::RichText::new(text);
    match tone {
        frame::Tone::Advisory => text.weak(),
        frame::Tone::Actionable => text.color(chrome(theme.unresolved)),
    }
}

/// **Draw one standing-fact badge**, and hand the response back.
///
/// The one draw the badge family has. What a badge SAYS, how loud it
/// is, whether it has more to say on hover and whether it is a control
/// are all the value's ([`crate::frame::Badge`]); what a click on a control
/// MEANS is the caller's, which is why this returns the response
/// instead of naming a window.
///
/// The separator rides here too: a badge that is drawn is a badge that
/// is separated from what precedes it, and a badge that is silent
/// leaves no gap behind.
fn draw_badge(ui: &mut egui::Ui, theme: &Theme, badge: &frame::Badge) -> egui::Response {
    ui.separator();
    let text = toned(badge.label(), theme, badge.tone());
    let response = match badge.affordance() {
        frame::Affordance::Read => ui.label(text),
        // Frameless, so a control the reader can open still reads as a
        // badge in a row of badges.
        frame::Affordance::Opens => ui.add(egui::Button::new(text).frame(false)),
    };
    match badge.detail() {
        Some(detail) => response.on_hover_text(detail),
        None => response,
    }
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
    /// **The index identity `scene` carries**: the
    /// [`crate::pickindex::PictureKey`] of the [`PickIndex`] whose id
    /// map minted the per-corner ids in the mesh on screen, as
    /// [`PickIndex::current_for`] takes it.
    ///
    /// A pick id is a word of ONE index's alphabet. Anything that
    /// resolves an id the picture produced, or mints one for the
    /// picture to compare against, is reading that alphabet, so it owes
    /// a check that the index in hand is the index this pair names —
    /// `pane::viewport::drawn_index` is where that check lives.
    ///
    /// `None` is a picture no index minted ids for: the startup mesh
    /// comes from [`scene::scene_of`], whose corners all carry
    /// [`crate::pickindex::IdMap::NOTHING`].
    scene_key: Option<PictureKey>,
    /// The display-state revision `scene` was built under — hide and
    /// free-move are scene inputs too, so a display change owes a
    /// rebuild exactly as a new evaluation does.
    scene_display: Option<u64>,
    /// The focus set `scene` was built under — the ids of what the side
    /// panel is showing ([`crate::marks::focus`]), which the scene carries as a
    /// per-corner flag and therefore has to be rebuilt for.
    ///
    /// Compared as a SET rather than counted by a revision, because
    /// unlike hide and free-move the focus is DERIVED (from the
    /// selection and the index), so there is no mutation to hang a
    /// counter off and no owner to bump one. Moving the selection
    /// between two faces of the same feature leaves the set equal and
    /// correctly rebuilds nothing.
    scene_focus: BTreeSet<u32>,
    /// **What the last scene rebuild refused**, held until one
    /// succeeds — the state [`crate::frame::scene_badge`] reads.
    ///
    /// Held rather than announced: the viewport keeps drawing the mesh
    /// it already has, so "the picture is older than the document and
    /// the rebuild will not run" is true on every frame until a
    /// rebuild lands, and a badge is a read of exactly this.
    scene_fault: Option<SceneError>,
    /// **What the last projection refused**, written by the viewport
    /// as it paints and read by [`crate::frame::projection_badge`].
    ///
    /// The toolbar draws before the panes, so the badge appears on the
    /// frame after the refusal. A view matrix that cannot be formed is
    /// not a one-frame condition — nothing is drawn until the camera
    /// moves somewhere it can be — so a badge one frame behind is a
    /// badge that appears.
    projection_fault: Option<CameraError>,
    /// **How many datums the viewport drew nothing of on the last
    /// frame it drew**, read by [`crate::frame::datums_badge`].
    ///
    /// One frame behind for the reason above. Unlike the fault above
    /// it is not held past the frame that made it: the frame entry
    /// point (`<ViewerApp as eframe::App>::ui`) zeroes the value the
    /// panes write and assigns the result back unconditionally, so
    /// this says what the LAST FRAME found and never what some
    /// earlier one did.
    datums_vanished: usize,
    /// **How many committed profiles the viewport could not draw on
    /// the last frame it drew** (`crate::sketch::CommittedProfiles::
    /// undrawn`), read by [`crate::frame::profiles_badge`]. Zeroed and
    /// assigned back every frame exactly as [`Self::datums_vanished`]
    /// is.
    profiles_undrawn: usize,
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
    /// **The display budget's seam** — where the probe tessellations
    /// that price a document run, which is not this thread.
    ///
    /// State ABOUT WORK IN FLIGHT rather than a second opinion about
    /// the document, which is the same standing `PickCache`'s record
    /// of what it has asked for has (`crates/viewer/GUI-DESIGN.md`,
    /// *What that does to the frame-state inventory*). Nothing here is
    /// derived from the document and nothing here can be WRONG about
    /// it: what the seam holds is a request, and the δ it answers with
    /// is compared against the δ in force before it is taken.
    fit: Box<dyn FitService>,
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
    /// **It is persisted where the store can keep it.**
    /// [`Self::remember_prefs`] writes it on every switch and
    /// `Prefs::resolve_theme` reads it back at startup, so a viewer
    /// reopened remembers. Where the store keeps nothing the switch
    /// still applies to the screen and only the memory is lost, which
    /// the toolbar says beside the picker rather than leaving a reader
    /// to discover next session.
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
    ///
    /// One latch per door of the profile editor: the add-profile form,
    /// and the same editor opened on a committed profile
    /// (`ViewerBehavior::edit_profile_ui`).
    profile_drawn: ProfileDoors<bool>,
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
    /// visible instead of silently dropped — with what it is ABOUT, so
    /// the next event about that subject can retire it
    /// ([`crate::frame::Subject`]).
    status: Option<frame::Message>,
    /// **Everything THIS frame has to say**, collected as it happens
    /// and ranked with the batch verdict rather than before it. It was
    /// the open tool's declined picks and survival drops plus the
    /// display state the frame's own operations withdrew; the sweep
    /// that routed every writer through the ranking made it the whole
    /// list, refusals included — the δ the display budget declined,
    /// the preferences store that would not write, the δ field's
    /// unparseable text, ten tool refusals from the create pane, the
    /// pick refusal, the unindexed-click refusal, the two picking
    /// paths' disagreement, and a camera fold's refusal through
    /// [`frame::deliver`].
    ///
    /// A refusal is not a special case here and never was: what makes
    /// it belong is that it HAPPENED on this frame, which is the whole
    /// test. What it is not is a RETIREMENT — nothing on this list
    /// takes a sentence away — which is why [`ViewerApp::status`]
    /// survives beside it.
    ///
    /// They cannot be written straight to [`ViewerApp::status`] —
    /// [`crate::frame::frame_status`] carries that argument, and it is the one
    /// place it is made. Drained every frame by `perform_batch`, so
    /// nothing here survives into the next one.
    notices: Vec<frame::Message>,
    /// Whether the environment can show a file dialog at all — probed
    /// once at startup ([`platform::chooser_backend`]); the Open/Save As
    /// controls read it every frame.
    chooser: platform::ChooserBackend,
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
    /// session ([`crate::prefs::Notice::UnknownPreset`]) but must survive in
    /// the file, or opening an older viewer once would silently
    /// delete a newer one's choice. Kept as a field rather than
    /// re-read at save time because the save happens on a UI event
    /// and reading the file there would race the very write it is
    /// about to do.
    keys_pref: Option<String>,
    /// The directory the last file dialog opened from or saved to —
    /// the second of the three places a dialog can open
    /// ([`frame::dialog_dir`]). Loaded from the preferences at startup
    /// and written back by [`Self::remember_prefs`] on every dialog
    /// that returns a path, so it outlives the session where the store
    /// keeps anything and lasts the session where it does not.
    ///
    /// Loaded and carried on every target, wasm included, so a write of
    /// the preferences never drops it; only the native dialogs read it.
    last_dir: Option<std::path::PathBuf>,
    /// The directory the viewer was launched from — the last of the
    /// three places, read once ([`platform::launch_dir`]) because a
    /// working directory is an environment reading. `None` when it
    /// could not be read, which the startup notices said.
    #[cfg(not(target_family = "wasm"))]
    launch_dir: Option<std::path::PathBuf>,
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
    /// A seam's worker thread could not be started — the evaluation
    /// worker or the index one; [`crate::evalseam::Worker`] names
    /// which, inside the payload, which is where a set of two belongs
    /// rather than as two arms here. Fatal on purpose: a seam with no
    /// worker accepts every submit and answers none, so the
    /// application would open onto a permanent "evaluating…" or a
    /// permanent "indexing…".
    ///
    /// Absent on wasm, where the seams are
    /// [`crate::evalseam::InlineEvaluator`] and
    /// [`crate::evalseam::InlineIndexer`] — nothing is spawned, so
    /// nothing can refuse to spawn. The arm
    /// is `cfg`-ed away rather than kept and never constructed,
    /// because a closed enum (D4 ¶3) whose reader must ask which arms
    /// are reachable is no longer telling the truth about its states.
    #[cfg(not(target_family = "wasm"))]
    Worker(crate::evalseam::SpawnError),
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
            Self::Worker(error) => write!(f, "{error}"),
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
/// [`StartupError::Worker`] if the OS refuses the worker thread.
/// The wasm arm is infallible — it spawns nothing — and returns
/// `Ok` unconditionally.
fn evaluator() -> Result<Box<dyn crate::evalseam::EvalService>, StartupError> {
    #[cfg(not(target_family = "wasm"))]
    {
        Ok(Box::new(
            ThreadEvaluator::spawn().map_err(StartupError::Worker)?,
        ))
    }
    #[cfg(target_family = "wasm")]
    {
        Ok(Box::new(crate::evalseam::InlineEvaluator::new()))
    }
}

/// The index seam this build runs on — the same choice
/// [`evaluator`] makes, for the same reason.
///
/// # Errors
///
/// [`StartupError::Worker`] if the OS refuses the thread; the wasm arm
/// is infallible. A viewer whose index seam never started would draw
/// its opening picture and refuse every pick on every document
/// forever — a failure to meet at startup, not to discover by clicking.
fn indexer() -> Result<Box<dyn crate::evalseam::IndexService>, StartupError> {
    #[cfg(not(target_family = "wasm"))]
    {
        Ok(Box::new(
            crate::evalseam::ThreadIndexer::spawn().map_err(StartupError::Worker)?,
        ))
    }
    #[cfg(target_family = "wasm")]
    {
        Ok(Box::new(crate::evalseam::InlineIndexer::new()))
    }
}

/// The display budget's fit seam this build runs on — [`evaluator`]'s
/// choice again, for its reason.
///
/// # Errors
///
/// [`StartupError::Worker`] if the OS refuses the thread; the wasm arm
/// is infallible. The index build waits on this seam's answer, so a
/// viewer whose fit worker never started would open every document to
/// a picture that never arrives — a failure to meet at startup, not to
/// discover by opening a file.
fn fitter() -> Result<Box<dyn FitService>, StartupError> {
    #[cfg(not(target_family = "wasm"))]
    {
        Ok(Box::new(
            crate::evalseam::ThreadFitter::spawn().map_err(StartupError::Worker)?,
        ))
    }
    #[cfg(target_family = "wasm")]
    {
        Ok(Box::new(crate::evalseam::InlineFitter::new()))
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
        let app = Self::assemble(&cc.egui_ctx, tol)?;

        // The half of startup that needs a device: the viewport's
        // pipeline, installed into the frame's render state. It is
        // the only half, which is why the rest is `assemble` and a
        // context with no device can still run the chrome.
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

        Ok(app)
    }

    /// Everything startup does that does not need a graphics device:
    /// the document, its evaluation and tessellation, the camera,
    /// the preferences and the two context-wide styles they set.
    ///
    /// # Errors
    ///
    /// Every arm of [`StartupError`] except
    /// [`StartupError::NoWgpuRenderState`], which is [`Self::new`]'s.
    fn assemble(egui_ctx: &egui::Context, tol: Tol) -> Result<Self, StartupError> {
        let delta = DisplayTolerance::new(scene::INITIAL_DELTA).map_err(StartupError::Scene)?;
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
        // The launch directory, read once for the same reason the
        // preferences path is: an environment reading belongs to the
        // process's start, not to the frame that happens to need it.
        // A directory that cannot be read is said here, once, and the
        // dialogs fall through it — nothing is invented in its place.
        #[cfg(not(target_family = "wasm"))]
        let launch_dir = match platform::launch_dir() {
            Ok(dir) => Some(dir),
            Err(error) => {
                notices.push(format!(
                    "launch directory unreadable ({error}); a file dialog opens where its \
                     backend chooses until one is used"
                ));
                None
            }
        };

        // The startup palette reaches the chrome here, not on the
        // first frame: a window that opened dark and turned light one
        // frame later would flash, and the flash would be the honest
        // report of a theme applied too late.
        apply_polarity(egui_ctx, theme.polarity);

        // And the numeric rule onto both of the context's styles, so a
        // field that never reached `widgets::number_field` still says
        // what it holds. See that function's neighbour for why this is
        // a default rather than a check.
        crate::widgets::install_number_formatter(egui_ctx);

        Ok(Self {
            session: DocSession::new(document, tol, evaluator()?),
            delta,
            scene: Arc::new(mesh),
            picks: PickCache::new(indexer()?),
            id_answer: Arc::new(AtomicU64::new(0)),
            id_log: IdQueryLog::new(),
            revision: 1,
            scene_key: None,
            scene_display: None,
            scene_focus: BTreeSet::new(),
            scene_fault: None,
            projection_fault: None,
            datums_vanished: 0,
            profiles_undrawn: 0,
            // The startup document goes through the same door an
            // opened one does: it is small enough that the budget will
            // not move its δ, and a first picture that took a
            // different path from every later one is a difference
            // waiting to be a bug.
            fit_delta_on_scene: true,
            budget_delta: None,
            fit: fitter()?,
            tools: Tools::new(),
            part_chooser: None,
            profile_drawn: ProfileDoors::default(),
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
            status: frame::startup_notices(&notices),
            notices: Vec::new(),
            chooser: platform::chooser_backend(),
            store,
            keys_pref: saved.keys,
            last_dir: saved.last_dir,
            #[cfg(not(target_family = "wasm"))]
            launch_dir,
        })
    }

    /// Take the display budget's answer, if one is ready.
    ///
    /// **Dropped rather than applied on either half of a mismatched
    /// key.** An answer for a generation the session has moved past
    /// prices a document nobody is looking at. One for a δ the View
    /// pane has moved off prices a number the user has replaced, and
    /// the budget's authority stops at the δ a document OPENS at
    /// ([`ViewerApp::fit_delta_on_scene`]) — so the typed number wins,
    /// silently, because nothing was taken away from anyone.
    ///
    /// A fit that REFUSED leaves δ alone: the document is one whose
    /// roots do not gather, or one that tessellates at NEITHER of the
    /// two δ the fit can fall back between (`scene::fit_delta`'s scale
    /// probe and the rung that prices the request — a refusal at one of
    /// them is answered by the other, so only a body that refuses at
    /// both gets here). The index build is about to say so with its own
    /// typed refusal, and two opinions about that would be one too many.
    fn take_fit(&mut self) {
        while let Some(done) = self.fit.poll() {
            if Some(done.generation) != self.session.landed_generation()
                || done.requested != self.delta
            {
                continue;
            }
            if let Ok(fitted) = done.fit {
                self.delta = fitted.delta;
                self.budget_delta = fitted.requested_cost.map(|_| fitted);
            }
        }
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
        self.notices.extend(dropped.iter().map(|dropped| {
            // A tool's survival drop is provoked by the document
            // transition it did not survive, so the act that accepts
            // the next one is what retires it.
            frame::tool_news(dropped.to_string())
        }));
        // **The budget picks the δ a document opens at**, once, before
        // anything is built at the δ in force — so the un-budgeted
        // build is never paid for, only avoided. `scene::fit_delta`
        // carries the method and the numbers; `TRIANGLE_BUDGET` says
        // why there is a budget at all.
        //
        // **It runs on its own worker** (`crate::evalseam::FitService`).
        // The ladder is a run of probe tessellations costing about an
        // eighth of a full one, which on a document dense enough to
        // want a budget is the better part of a second, and a window
        // that stops repainting for it is the defect the index seam was
        // cut to remove — one step earlier, and on the very documents
        // the seam was cut for. So the shape is the index seam's:
        // submit, keep painting, take the answer when it comes.
        self.take_fit();
        if self.fit_delta_on_scene && self.session.landed_generation().is_some() {
            // Spent on the first landing, whether or not that landing
            // has a body to price: a document whose product refuses has
            // no size to fit a δ to, and leaving the budget armed for
            // its first successful EDIT would make it something other
            // than the δ a document OPENS at.
            self.fit_delta_on_scene = false;
            if let Some(request) = self.session.fit_request(self.delta) {
                self.fit.submit(request);
            }
        }
        // **The index seam answers here.** A build that finished is
        // installed or refused now, before the currency check below
        // reads the cache — so an index that landed for the picture on
        // screen is used on the frame it arrives rather than the next
        // one. A refusal is held by the cache and badged, not
        // announced; a superseded answer is nothing to say
        // (`pickcache::IndexLanding::Stale` is what restart-without-cancel
        // produces, once per δ changed mid-build).
        let mut rebuilt = false;
        for landing in self.picks.pump() {
            match landing {
                pickcache::IndexLanding::Built => rebuilt = true,
                // Nothing to say here: the cache HOLDS the refusal
                // under its one-attempt-per (generation, δ) policy,
                // and `frame::index_badge` reads it every frame the
                // toolbar draws. Announcing it once put a read on a
                // line the next acting batch sweeps.
                pickcache::IndexLanding::Refused => {}
                pickcache::IndexLanding::Stale => {}
            }
        }
        // The cache owns the retry policy: one attempt per (landed
        // generation, δ). A refused build is reported and held, not
        // re-attempted every frame behind a stale picture.
        //
        // Every arm but `Current` leaves the viewport drawing the mesh
        // it already has — an older picture, which the indexing
        // indicator names and `pickcache::unindexed` refuses picks against.
        // **The index waits for the budget's answer.** A build
        // submitted at the δ in force while the fit is still pricing
        // the document IS the un-budgeted build the budget exists to
        // avoid, so there is no δ to offer until the fit is done and
        // `None` is how the cache is told. It forgets: the previous
        // document's index stops answering picks the moment this one
        // lands, exactly as it would have on a submit.
        //
        // A δ typed while a fit is outstanding waits for it too, then
        // discards it (`ViewerApp::take_fit`) and builds verbatim. The
        // wait is the fit's own length and buys nothing, and is
        // accepted rather than tracked: the alternative is a second
        // record of what the seam already holds, for a window under a
        // second on the frames just after a document opens.
        let settled = (!self.fit.busy()).then_some(self.delta);
        match self.picks.sync(self.session.index_inputs(), settled) {
            pickcache::CacheStep::Held
            | pickcache::CacheStep::Nothing
            | pickcache::CacheStep::Submitted
            | pickcache::CacheStep::Indexing => return,
            pickcache::CacheStep::Current => {}
        }
        // The scene is a function of (index, display state, focus): a
        // display or selection change over a current index still owes
        // exactly one rebuild.
        let display_revision = self.session.display().revision();
        let Some(index) = self.picks.index() else {
            return;
        };
        let focus = marks::focus(index, self.session.doc(), self.session.selection());
        if !rebuilt && self.scene_display == Some(display_revision) && self.scene_focus == focus {
            return;
        }
        match index.scene_focused(&self.session.display_view(), &focus) {
            Ok(mesh) => {
                // Marked current ONLY on success: a refused build must
                // not consume this (generation, display) pair, or the
                // stale picture stays on screen marked as the current
                // one and is never retried.
                // Taken from the INDEX, not from the session: the
                // pair that matters is the one whose id map is in this
                // mesh, and reading the session's generation here would
                // be a second derivation of it that nothing holds to
                // the first.
                self.scene_key = Some(index.key());
                self.scene_display = Some(display_revision);
                self.scene_focus = focus;
                self.scene = Arc::new(mesh);
                self.revision = self.revision.wrapping_add(1);
                // The badge's read, ended by the rebuild it was
                // waiting for.
                self.scene_fault = None;
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
            // Held, not announced: this pair is deliberately left
            // unmarked above so the rebuild is retried, and the fault
            // is true of the picture on screen until one succeeds.
            Err(error) => self.scene_fault = Some(error),
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
            Err(error) => self.notices.push(frame::delta_refusal(&error)),
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
        let Some(fraction) = features_fraction(wanted, stack) else {
            return;
        };
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
            let accepted_op = op.clone();
            let outcome = self.session.perform(op);
            // **Where a withdrawal reaches the user**: everything
            // this operation's document transition took out of the
            // display state, onto the frame's notices like every other
            // one (`frame::frame_status` carries the argument).
            //
            // ONE call, not one per kind. Three hand-written `extend`s
            // stood here, and the list they fanned out was held to the
            // report's by nothing — this code is `app`-gated, so no
            // row can execute it and a kind dropped here is invisible
            // until a user misses a sentence. `Withdrawal::all`
            // destructures the report, so the list is the report's and
            // a fourth kind reds there.
            notices.extend(
                frame::Withdrawal::all(&outcome.withdrawn).map(|withdrawal| withdrawal.notice()),
            );
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
                None if tool_edit => {
                    self.tools.close();
                    self.drafts.accepted(&accepted_op);
                }
                // A form whose op committed comes to rest, for the
                // tool's reason: a refusal leaves it holding its draft.
                None => self.drafts.accepted(&accepted_op),
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

    /// Write everything the viewer remembers — the theme, the key
    /// preset, the last dialog directory — to the preferences store.
    /// Called when the theme changes and when a dialog returns a path
    /// in a new directory.
    ///
    /// **Best-effort, and it reports.** A write that FAILED — a store
    /// with somewhere to write that could not — is worth one line in
    /// the status area and nothing more: the theme is already applied
    /// on screen and the path already chosen, so a failure here costs
    /// the next session's memory of them, never this session's work. A store that can never be
    /// written is not that case and is not reported here at all; it is
    /// a standing fact the toolbar badges, per the guard below.
    /// Refusing the switch or the path because it could not be
    /// recorded would be the worse trade.
    ///
    /// The whole document is rewritten rather than patched, so every
    /// setting this viewer understands has to be carried across —
    /// which is why [`Self::keys_pref`] exists. A key it does NOT
    /// understand is lost, and that is stated rather than hidden: it
    /// is the price of a hand-written renderer that keeps its
    /// comments, and such a key was already reported on load.
    fn remember_prefs(&mut self) {
        // **A store that keeps nothing is not asked**, and what it
        // would have said is already said: `frame::prefs_badge` reads
        // the same value on the toolbar beside the picker, for as long
        // as it is true. Asking anyway would hand back
        // `prefs::Unusable::refusal` — the same sentence, once per
        // switch, on the channel that carries one frame's news, which
        // is the misclassification Ev ruled on for the absent file
        // chooser. Nothing is discarded here because nothing is
        // attempted.
        if self.store.unusable().is_some() {
            return;
        }
        let prefs = Prefs {
            theme: Some(self.theme.name.to_owned()),
            keys: self.keys_pref.clone(),
            last_dir: self.last_dir.clone(),
        };
        if let Err(error) = self.store.save(&prefs.to_toml()) {
            self.notices.push(frame::store_refusal(&error));
        }
    }

    /// Remember the directory a dialog just returned a path in: the
    /// next dialog opens there when no document says otherwise
    /// ([`frame::dialog_dir`]), and so does the next run's, where the
    /// store keeps anything. An unchanged directory writes nothing: the
    /// file already says it.
    #[cfg(not(target_family = "wasm"))]
    fn remember_dir(&mut self, path: &std::path::Path) {
        let Some(dir) = frame::containing_dir(path) else {
            return;
        };
        if self.last_dir.as_deref() == Some(dir) {
            return;
        }
        self.last_dir = Some(dir.to_owned());
        self.remember_prefs();
    }

    /// **The one door both file dialogs go through**, so Open… and
    /// Save As… cannot disagree about where they start and neither can
    /// forget to remember where it ended. It opens where
    /// [`frame::dialog_dir`] says, with `Path::is_dir` as the witness
    /// that a candidate still exists; offers a saved document's own
    /// file name to Save As…; and remembers the directory of the path
    /// it returns ([`Self::remember_dir`]). A THIN veneer over
    /// `SessionOp::Open` and `SessionOp::Save`: everything it does is
    /// choose a `Path`.
    ///
    /// **It blocks, deliberately.** A modal file chooser is the
    /// platform's own idea of a modal file chooser, and the alternative
    /// (an async handle polled across frames) would buy responsiveness
    /// during an interaction that is already modal, at the cost of a
    /// second state machine.
    ///
    /// **The starting directory reaches the portal and nothing else.**
    /// `rfd` hands `set_directory` to the portal as `current_folder`;
    /// its zenity fallback drops it and forwards only the file name, so
    /// a zenity dialog opens at zenity's own default (the process's
    /// working directory, which is the launch directory). The directory
    /// is not spelled into the file name for zenity's sake: which
    /// backend answers cannot be known from here — with no session-bus
    /// address in the environment `rfd` still reaches a portal through
    /// D-Bus autolaunch where one runs — and a portal handed that name
    /// shows the whole directory path in its name field. A portal that
    /// lacks `current_folder` on its open dialog shows its own default
    /// there; its save dialog honours it.
    ///
    /// **Absent on wasm**, and so are the bodies of the two button arms
    /// that call it. That second state machine is exactly what the
    /// browser would force — `rfd`'s wasm backend offers only the async
    /// dialog, and there are no paths behind it either — so the browser
    /// build has no door here, and [`platform::chooser_backend`] says so
    /// to the chrome by disabling both buttons.
    #[cfg(not(target_family = "wasm"))]
    fn file_dialog(&mut self, kind: FileDialog) -> Option<std::path::PathBuf> {
        let mut dialog = rfd::FileDialog::new().add_filter("document", &[DOC_EXTENSION]);
        if let Some(dir) = frame::dialog_dir(
            self.session.path(),
            self.last_dir.as_deref(),
            self.launch_dir.as_deref(),
            std::path::Path::is_dir,
        ) {
            dialog = dialog.set_directory(dir);
        }
        let path = match kind {
            FileDialog::Open => dialog.pick_file(),
            FileDialog::SaveAs => {
                if let Some(name) = self
                    .session
                    .path()
                    .and_then(std::path::Path::file_name)
                    .and_then(std::ffi::OsStr::to_str)
                {
                    dialog = dialog.set_file_name(name);
                }
                dialog.save_file()
            }
        }?;
        self.remember_dir(&path);
        Some(path)
    }

    /// This application's door onto [`frame::apply`], for the verdict
    /// the ranking has ALREADY WEIGHED: `perform_batch` hands
    /// [`crate::frame::frame_status`]'s answer here rather than assigning the
    /// field. Its one live caller, and deliberately so — a `Show` that
    /// has been through the ranking must reach the field, and handing
    /// it to [`frame::deliver`] instead would loop it back onto
    /// `notices` to be ranked a second time.
    ///
    /// **Not the one place a [`StatusUpdate`] becomes the field** —
    /// that is [`crate::frame::apply`], which [`crate::pane::viewport`] reaches directly
    /// at both of its doors: `land` through [`frame::deliver`], and the
    /// cursor's retirement through [`crate::frame::apply`] itself. Neither has a
    /// `&mut self` to come through; both take the `&mut
    /// Option<frame::Message>` this is shorthand for. This is the
    /// `&mut self` shorthand, nothing more.
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

    /// The toolbar row: the controls the chrome keeps drawn in every
    /// state, the two gesture cancel doors among them.
    ///
    /// **Wrapped, not one line.** A non-wrapping row is laid out on
    /// one line and clipped, so a window narrower than the row's
    /// natural width does not shrink the right-hand end — it puts it
    /// out of reach, with nothing saying so. The two cancel doors sit
    /// there, and a cancel door is the ONE exit from a state in which
    /// every other operation refuses [`Refusal::GestureInFlight`],
    /// which is why it is in the panel that is always drawn. Wrapping
    /// costs a second line only at widths where the alternative was a
    /// control nobody could click.
    ///
    /// [`Refusal::GestureInFlight`]: crate::session::Refusal::GestureInFlight
    fn toolbar_ui(&mut self, ui: &mut egui::Ui, ops: &mut Vec<SessionOp>, chosen: &mut Theme) {
        ui.horizontal_wrapped(|ui| {
            // What is OPEN, not what the program is called: the
            // window title already carries the application's name,
            // and a toolbar that repeats it tells a user nothing
            // they cannot see in their own title bar.
            ui.label(document_name(self.session.path()));
            ui.separator();
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
            // With confidently NO backend the dialogs are disabled
            // UP FRONT with the reason as their tooltip, because a
            // dead click is exactly the silent failure #1097
            // reported. **That tooltip is the whole surface** —
            // why it is the only one is the viewer README's, under
            // *"A missing file-chooser backend is not on the line
            // at all"*. Under a plausibly-present backend a dialog
            // handing back `None` is a genuine cancel, which says
            // nothing.
            let chooser = self.chooser;
            if ui
                .add_enabled(chooser.usable(), egui::Button::new("Open…"))
                .on_disabled_hover_text(platform::NO_CHOOSER_BACKEND)
                .clicked()
            {
                // Unreachable on wasm — `chooser` is `Absent`
                // there, so the button is disabled and never
                // reports a click — but unreachable code still has
                // to compile, and `Self::file_dialog` does not exist
                // on that target. The `cfg` is on the BODY rather
                // than the button so the browser build still shows
                // the control and its disabled reason, which is
                // the #1125 posture: a door that cannot open says
                // so, it does not vanish.
                #[cfg(not(target_family = "wasm"))]
                if let Some(path) = self.file_dialog(FileDialog::Open) {
                    ops.push(SessionOp::Open(path));
                }
            }
            if ui
                .add_enabled(chooser.usable(), egui::Button::new("Save As…"))
                .on_disabled_hover_text(platform::NO_CHOOSER_BACKEND)
                .clicked()
            {
                // Unreachable on wasm, for the reason the Open…
                // arm above states.
                #[cfg(not(target_family = "wasm"))]
                if let Some(path) = self.file_dialog(FileDialog::SaveAs) {
                    ops.push(SessionOp::Save(path));
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
            // **The cancel doors**, beside the history controls
            // because that is where a reader whose every edit is
            // being refused already is. They are HERE and not on
            // the field that opened the gesture: the field is what
            // can stop being drawn mid-drag, and a cancel sited on
            // it would vanish with the exit it exists to replace
            // (`session::CancelDoor`). This panel is drawn on every
            // frame whatever the selection, the standing and the
            // layout are.
            //
            // Enabled exactly while the door's own gesture is in
            // flight, and out of flight it says the refusal the
            // operation itself would give — the value that knows
            // carries the words, so the two cannot disagree.
            for door in self.session.cancel_doors() {
                let button = ui.add_enabled(door.blocked.is_none(), egui::Button::new(door.label));
                let clicked = match &door.blocked {
                    Some(refusal) => button.on_disabled_hover_text(refusal.to_string()).clicked(),
                    None => button.clicked(),
                };
                if clicked {
                    ops.push(door.op);
                }
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
            // ONE indicator for one wait: `progress` ranks what
            // the session owes against what the index seam is
            // doing, so the toolbar never lights two spinners for
            // the same moment.
            // **The spinner follows the work, never the name**
            // (`frame::Progress::Canceled`). A fit in flight is the
            // index build's first step from a user's seat — nothing is
            // on screen for it, the build follows it with no gap, and
            // the one progress state is what says a picture is coming.
            match frame::progress(
                self.session.outstanding(),
                self.picks.indexing() || self.fit.busy(),
            ) {
                Some(frame::Progress::Evaluating) => {
                    ui.separator();
                    ui.spinner();
                    ui.label("evaluating…");
                    if ui.button("Cancel").clicked() {
                        ops.push(SessionOp::CancelEvaluation);
                    }
                    // A background result is not a user event, so
                    // nothing else would wake the frame loop to
                    // collect it.
                    ui.ctx().request_repaint();
                }
                Some(frame::Progress::Canceled { indexing }) => {
                    ui.separator();
                    // The recourse is UNCONDITIONAL: the cancel is
                    // what the reader has to act on, and an index
                    // build behind it must not take the button
                    // away for the seconds it runs. The spinner
                    // reads left of the label because that is
                    // where the other two arms put theirs.
                    if indexing {
                        ui.spinner();
                    }
                    ui.label("canceled — showing an older result");
                    if ui.button("Re-evaluate").clicked() {
                        ops.push(SessionOp::Reevaluate);
                    }
                    if indexing {
                        ui.weak("indexing…")
                            .on_hover_text(crate::pickcache::NotIndexed::Building.to_string());
                        ui.ctx().request_repaint();
                    }
                }
                // No Cancel button beside it, and that is the
                // seam's promise showing through the chrome: the
                // build cannot be stopped, only outrun by a newer
                // one (`evalseam`, the index seam).
                Some(frame::Progress::Indexing) => {
                    ui.separator();
                    ui.spinner();
                    ui.label("indexing…")
                        .on_hover_text(crate::pickcache::NotIndexed::Building.to_string());
                    ui.ctx().request_repaint();
                }
                None => {}
            }
            // **The reads, one draw each.** Each is a function of
            // the typed value it reads — including its silence,
            // which is what lets a row assert the `None`
            // — and each states its own tone and affordance, so
            // nothing about how a badge looks is decided here
            // (`frame::Badge`).
            //
            // The A5 at-rest verdict, for assembly-shaped
            // documents: the verification verdict living past the
            // commit.
            if let Some(badge) = frame::at_rest_badge(self.session.at_rest()) {
                draw_badge(ui, &self.theme, &badge);
            }
            // The advisory checks. It REPORTS: the scene below is
            // drawn either way, because a product whose roots
            // interpenetrate renders a picture that looks almost
            // right and the finding is the only thing that says
            // otherwise. The badge OPENS the window the findings'
            // own sentences live in, and what a click means is
            // this call site's — which is why the draw hands the
            // response back rather than naming a window.
            let checks = frame::checks_badge(self.session.checks());
            if let Some(badge) = checks
                && draw_badge(ui, &self.theme, &badge).clicked()
            {
                self.checks_shown = !self.checks_shown;
            }
            // The gather's verdict, for the landed pair. **A read
            // a reader consults, so a badge** — the line beside it
            // carries what just happened and is cleared by the next
            // acting batch, while "the product on screen does not
            // gather" is true until another pair lands.
            //
            // Which faults reach it is `frame::product_badge`'s,
            // and it declines every state another channel carries:
            // the three per-node arms are the feature tree's, and
            // an empty document is the blank viewport's.
            if let Some(badge) = frame::product_badge(self.session.product_fault()) {
                draw_badge(ui, &self.theme, &badge);
            }
            // The display budget's: shown while the δ on screen is
            // the one the budget CHOSE when the document opened,
            // and gone the moment the user picks their own.
            if let Some(badge) = frame::delta_badge(self.budget_delta.as_ref()) {
                draw_badge(ui, &self.theme, &badge);
            }
            // **The three display seams that hold a refusal.**
            // Each is a read of held state — the scene fault kept
            // until a rebuild lands, the pick cache's own held
            // refusal, the projection the viewport could not form
            // — so each stands for as long as the picture is stale
            // rather than until the next acting batch sweeps a
            // line. What a pick aimed at the missing index gets is
            // still the line's, because that is an outcome
            // (`frame::unindexed_refusal`).
            for badge in [
                frame::scene_badge(self.scene_fault.as_ref()),
                frame::index_badge(self.picks.error()),
                frame::projection_badge(self.projection_fault.as_ref()),
            ]
            .into_iter()
            .flatten()
            {
                draw_badge(ui, &self.theme, &badge);
            }
            // **The datums the last drawn frame drew nothing of.**
            // Not one of the three above: those hold a refusal until
            // the seam they name succeeds, and this is a count the
            // frame re-takes, so it stands for exactly as long as the
            // view that produced it. What it buys is the one thing the
            // picture cannot say — that the document HAS datums and
            // this view draws none of them, which on screen is
            // indistinguishable from a document with none.
            if let Some(badge) = frame::datums_badge(self.datums_vanished) {
                draw_badge(ui, &self.theme, &badge);
            }
            // The same kind of count for the committed profiles: a
            // profile left out of the picture is otherwise a document
            // without it.
            if let Some(badge) = frame::profiles_badge(self.profiles_undrawn) {
                draw_badge(ui, &self.theme, &badge);
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
                        ui.selectable_value(&mut *chosen, *theme, theme.name);
                    }
                });
            // **Beside the picker, not in the badge row above**:
            // it is the only badge that is about a control rather
            // than about the document or the picture, and a
            // reader deciding whether a choice will survive the
            // session needs it where the choice is made. It is
            // drawn whether or not anything has been picked yet,
            // because a standing fact is worth knowing BEFORE the
            // choice, and it is drawn once because it is a read
            // rather than an answer to the switch.
            if let Some(badge) = frame::prefs_badge(self.store.unusable().as_ref()) {
                draw_badge(ui, &self.theme, &badge);
            }
            if let Some(status) = &self.status {
                ui.separator();
                ui.label(status.text());
            }
        });
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
            self.toolbar_ui(ui, &mut ops, &mut chosen);
        });

        if chosen != self.theme {
            self.theme = chosen;
            apply_polarity(ui.ctx(), chosen.polarity);
            self.remember_prefs();
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
        //
        // The edit draft is brought up to the document FIRST: a
        // selection that left its node drops it, and an undo that
        // changed its program reloads it — so the preview taken below
        // is of the draft the pane is about to show, not last frame's.
        self.drafts.sync_profile_edit(
            self.session.committed_doc(),
            self.session.selection().node(),
        );
        let held = self.drafts.door_loops();
        // **No frame picked, no preview.** The form draws on a frame
        // the document holds, so with none picked there is no plane to
        // place the loops on and nothing honest to show — the form
        // says what it is waiting for instead, exactly as it does for
        // a shape nobody chose. The edit door's frame is the
        // committed profile's own.
        let (tol, delta) = (self.session.tol(), self.delta.get());
        let profile_previews = held.as_ref().zip(self.profile_drawn).map(|(held, drawn)| {
            held.as_ref()
                .filter(|_| drawn)
                .and_then(|held| held.frame.zip(self.session.landed_pair()))
                .and_then(|(frame, (doc, evaluation))| {
                    sketch::frame_placement(doc, evaluation, frame)
                })
                .zip(held.as_ref())
                .map(|(plane, held)| sketch::preview(plane, &held.loops, tol, delta))
        });
        // The committed node the edit door is previewing in its place,
        // which the committed-profile pass leaves out.
        let profile_edited = self.drafts.edited_in_place(profile_previews.edit.as_ref());
        let mut profile_drawn = ProfileDoors::<bool>::default();
        // **Zeroed here and assigned back below, every frame.** The
        // viewport writes it while it draws; a frame the viewport does
        // not draw at all is a frame with no datums vanishing in it,
        // and this is where that is said rather than left to whatever
        // the field last held.
        let mut datums_vanished = 0_usize;
        let mut profiles_undrawn = 0_usize;
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
                    scene_key: self.scene_key,
                    indexing: self.picks.indexing(),
                    revision: self.revision,
                    camera: &mut self.camera,
                    input: self.input,
                    theme: self.theme,
                    drafts: &mut self.drafts,
                    display: &display,
                    tools: &mut self.tools,
                    part_chooser: &mut self.part_chooser,
                    profile_previews: &profile_previews,
                    profile_drawn: &mut profile_drawn,
                    profile_edited,
                    pending_fit: &mut self.pending_fit,
                    projection_fault: &mut self.projection_fault,
                    datums_vanished: &mut datums_vanished,
                    profiles_undrawn: &mut profiles_undrawn,
                    notices: &mut self.notices,
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
        self.profile_drawn = profile_drawn;
        self.datums_vanished = datums_vanished;
        self.profiles_undrawn = profiles_undrawn;
        // An edit made while the panes drew leaves the preview a
        // frame behind. Asking for a repaint is what makes that one
        // frame rather than "until the next input event".
        let moved =
            profile_drawn
                .zip(held.zip(self.drafts.door_loops()))
                .map(|(drawn, (before, now))| {
                    drawn
                        && match (before, now) {
                            (Some(before), Some(now)) => !before.previews_as(&now),
                            (None, None) => false,
                            _ => true,
                        }
                });
        if moved.into_array().contains(&true) {
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
        self.notices.extend(declined.iter().map(|declined| {
            // A declined pick answers an act the user aimed at the
            // document, like every other rank-2 notice this frame.
            frame::tool_news(declined.to_string())
        }));

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
    /// The index identity the `scene` above carries (`ViewerApp::
    /// scene_key`), for the reads of `index` that are about the
    /// PICTURE rather than about the document.
    pub(crate) scene_key: Option<PictureKey>,
    /// Whether a build for the picture this frame WANTS is under way —
    /// the other half of what `index: None` means, and the half that
    /// decides which sentence a refused pick gets
    /// ([`crate::pickcache::NotIndexed`]). Carried as a value rather than re-derived
    /// from the session, because "someone is building one" is the pick
    /// cache's answer and nothing else's.
    pub(crate) indexing: bool,
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
    /// What each door of the profile editor's loops would draw, taken
    /// once for the frame: the panel says what it refuses and the
    /// viewport draws what it replayed, from ONE reading.
    ///
    /// `None` is "no preview was taken this frame" — the frame a door
    /// first comes on screen, before the latch below has told anyone
    /// to take one. Distinct from `Some(Ok(empty))`, which is a
    /// preview that WAS taken and drew nothing, and which the form is
    /// entitled to say so about.
    pub(crate) profile_previews: &'a ProfileDoors<Option<Result<ProfilePreview, PreviewError>>>,
    /// Set by each door's editor while it draws; read next frame.
    pub(crate) profile_drawn: &'a mut ProfileDoors<bool>,
    /// The committed profile the edit door is previewing this frame,
    /// which the committed-profile pass leaves out
    /// (`sketch::committed`'s `except`).
    pub(crate) profile_edited: Option<pncad::document::RecipeNodeId>,
    pub(crate) pending_fit: &'a mut bool,
    /// Where the viewport leaves a view matrix it could not form, for
    /// [`frame::projection_badge`] to read: a read of the camera, so
    /// the pane holds it rather than writing a sentence the toolbar
    /// had already painted past and the next accepted act would
    /// sweep.
    pub(crate) projection_fault: &'a mut Option<CameraError>,
    /// **How many datums the viewport drew nothing of this frame**,
    /// for [`frame::datums_badge`] to read.
    ///
    /// Written by the viewport pane and read by the toolbar next
    /// frame, like the fault above — and unlike it, it does not have
    /// to be cleared by anyone. The frame entry point
    /// (`<ViewerApp as eframe::App>::ui`) zeroes the local this
    /// borrows before the panes draw and assigns the result back
    /// after, whether or not the viewport was one of them, so a
    /// viewport dragged shut or tabbed away reports none rather than
    /// leaving the last count it made standing. That is
    /// `profile_drawn`'s discipline above, and it is the one
    /// `work/view/projection-fault-has-no-sweeper.md` says the fault
    /// still lacks.
    pub(crate) datums_vanished: &'a mut usize,
    /// How many committed profiles the viewport drew nothing of
    /// ([`ViewerApp::profiles_undrawn`]); zeroed by the frame entry
    /// point and written by the viewport, as `datums_vanished` is.
    pub(crate) profiles_undrawn: &'a mut usize,
    /// **What this frame's panes have to SAY**, joined and ranked by
    /// [`frame::frame_status`] with everything else the frame
    /// produced. A pane that assigned `status` instead had no way to
    /// say "I have nothing to add", and its sentence was erased by the
    /// batch this frame accepted before it was ever painted —
    /// `perform_batch` runs after the panes have drawn.
    pub(crate) notices: &'a mut Vec<frame::Message>,
    /// The line itself, for the one thing a notice cannot do: RETIRE a
    /// sentence. [`crate::frame::cursor_status`] and a clean camera fold expire
    /// what they last said and add nothing, so both reach the field
    /// directly — by different doors, because the two policies are not
    /// the same shape. `cursor_status` answers only `Keep` or `Expire`,
    /// so it can never have news and goes straight through
    /// [`crate::frame::apply`] ([`crate::pane::viewport`], the id pass). `fold_status`
    /// can answer either way, so `land` hands it to
    /// [`frame::deliver`], which routes the refusal to `notices` above
    /// and the clean fold's retirement here.
    pub(crate) status: &'a mut Option<frame::Message>,
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

/// Which of the two file dialogs [`ViewerApp::file_dialog`] puts up.
#[cfg(not(target_family = "wasm"))]
#[derive(Clone, Copy, Debug)]
enum FileDialog {
    /// Open…: pick an existing document.
    Open,
    /// Save As…: choose where the document goes, offered its current
    /// file name when it has one.
    SaveAs,
}

/// **The Features tile's share of the stack it sits in**, capped at
/// [`FEATURES_SHARE_CAP`], and `None` when the two measurements are
/// not numbers to divide.
///
/// **The caller's `stack <= 0.0` arm above is about an EMPTY stack and
/// this one is about an unmeasurable one**, and they are written apart
/// because they are different facts: the first is the very first
/// frame, before either tile has a rectangle, which is a legitimate
/// state the caller is documented to do nothing in. The second is a
/// toolkit measurement that is not a length, and there is nothing
/// legitimate about it.
///
/// **A `clamp` is not a bound against the value it cannot order.**
/// `f32::clamp` returns `self` when `self` is a `NaN`, so a share that
/// could not be computed used to leave here looking exactly like one
/// that had been — and `egui_tiles` keeps shares as STATE rather than
/// recomputing them per frame, so a single poisoned frame left the
/// pair of panes with a split no later frame and no divider drag could
/// recover. Refusing keeps the last share the arithmetic actually
/// produced.
///
/// `stack > 0.0` is stated here as well as at the caller rather than
/// relied on from it: this door's answer has to be true of its own
/// arguments, and a division whose denominator is checked somewhere
/// else is checked by nothing when a second caller arrives.
fn features_fraction(wanted: f32, stack: f32) -> Option<f32> {
    if !wanted.is_finite() || !(stack.is_finite() && stack > 0.0) {
        return None;
    }
    // Slack over the measured height so the last row is not flush
    // against the divider.
    Some(((wanted + FEATURES_SLACK) / stack).clamp(0.0, FEATURES_SHARE_CAP))
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
    if platform::running_under_wsl() {
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
/// The browser counterpart of `run`, and deliberately the whole of
/// the difference between the two platforms' entry points: everything
/// downstream — the session, the panes, the camera, the input map —
/// is the same code the native build runs.
///
/// **No `open` parameter, unlike `run`.** There is no path to hand
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

#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]

    use super::{FEATURES_SHARE_CAP, Polarity, Theme, ViewerApp, features_fraction};
    use crate::session::SessionOp;
    use eframe::egui;

    /// **A share that could not be computed is not a share.**
    ///
    /// `f32::clamp` returns `self` when `self` is a `NaN`, so both of
    /// this door's measurements used to arrive at `set_share` in a
    /// field shaped like a number the layout had produced.
    ///
    /// The shape this pins is DISTINGUISHABILITY, and for an `f32`
    /// answer that takes two rows rather than one. Asserting only that
    /// the poisoned answer differs from every legitimate one passes
    /// over a `Some(NaN)` for free — a `NaN` is unequal to everything,
    /// including itself — which is the broken door's own answer wearing
    /// the test's approval. So: the poisoned inputs answer `None`, and
    /// the row below holds that every legitimate input answers a share.
    /// Neither row alone says anything.
    ///
    /// Both arguments are poisoned in turn, because a `NaN` in either
    /// one reaches the division and a row that poisoned only the
    /// denominator would have chosen its answer.
    #[test]
    fn a_share_that_could_not_be_measured_is_no_share() {
        for (what, wanted, stack) in [
            ("a content height", f32::NAN, 500.0),
            ("a stack height", 100.0, f32::NAN),
            ("an unbounded stack", 100.0, f32::INFINITY),
            ("an unbounded content height", f32::INFINITY, 500.0),
        ] {
            assert_eq!(
                features_fraction(wanted, stack),
                None,
                "{what} that is not a number",
            );
        }
    }

    /// **Every answer this door does give is a share**: a number, at
    /// or above nothing, at or below the cap. The row above is a claim
    /// about `None` and this one is what makes it mean anything — a
    /// door that answered `None` for everything would satisfy the
    /// first and fail this.
    #[test]
    fn every_share_it_gives_is_a_share() {
        for (what, wanted, stack) in [
            ("a tree that wants nothing", 0.0, 500.0),
            ("a tree taller than the stack", 5_000.0, 500.0),
            ("an ordinary tree", 100.0, 500.0),
        ] {
            let answer = features_fraction(wanted, stack);
            assert!(
                matches!(answer, Some(share)
                    if share.is_finite() && (0.0..=FEATURES_SHARE_CAP).contains(&share)),
                "{what} answered {answer:?}",
            );
        }
    }

    /// The door's answers are not one answer, which is what makes the
    /// two rows above a test of anything: a function returning the cap
    /// for every input satisfies both.
    #[test]
    fn the_legitimate_shares_are_distinct() {
        let floor = features_fraction(0.0, 500.0);
        let cap = features_fraction(5_000.0, 500.0);
        let ordinary = features_fraction(100.0, 500.0);
        assert_ne!(floor, ordinary, "the floor and an ordinary share");
        assert_ne!(ordinary, cap, "an ordinary share and the cap");
        // All three pairs. A set of three has three of them, and
        // checking the two adjacent ones leaves this one unread.
        assert_ne!(floor, cap, "the floor and the cap");
        assert_eq!(cap, Some(FEATURES_SHARE_CAP), "the cap is the cap");
    }

    /// The very first frame, before either tile has a rectangle, is
    /// the caller's own arm and not this door's — so a zero stack is
    /// not something this function is asked about. What it IS asked
    /// about is a denominator it was handed anyway, and it refuses
    /// rather than dividing by it.
    #[test]
    fn an_empty_stack_is_refused_here_too() {
        assert_eq!(features_fraction(100.0, 0.0), None, "an empty stack");
        assert_eq!(features_fraction(100.0, -1.0), None, "a negative stack");
    }

    /// The narrowest window this chrome is held to, in points.
    ///
    /// The browser is the narrow case the viewer actually ships into —
    /// `run_web` puts this same toolbar in a canvas the page sizes —
    /// and an upright phone viewport is the narrow end of the browser:
    /// 400 points is about the widest of that class, so a window this
    /// wide is the easiest member of the hardest case. A desktop
    /// window tiled to half of a 1280-point screen gets 640 and is
    /// therefore already covered by it.
    const NARROW: f32 = 400.0;

    /// A window wider than any toolbar will ask for, which is how the
    /// row's natural width is read: nothing constrains the layout, so
    /// what it occupies is what it wants.
    const UNBOUNDED: f32 = 4000.0;

    /// What one headless frame of the toolbar occupied, and what it
    /// was given to occupy.
    struct Row {
        /// The width the row's content laid itself out across.
        occupied: f32,
        /// The width the panel offered it.
        available: f32,
    }

    /// Lay the real toolbar out in a headless context whose window is
    /// `width` points wide.
    ///
    /// Two frames: the first is the one egui sizes from defaults, the
    /// second is the one a user looks at.
    fn toolbar_row(width: f32) -> Row {
        let ctx = egui::Context::default();
        let mut app = ViewerApp::assemble(&ctx, pncad::tolerance::witness())
            .expect("startup that needs no graphics device");
        let mut row = Row {
            occupied: f32::NAN,
            available: f32::NAN,
        };
        for _ in 0..2 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(width, 600.0),
                )),
                ..Default::default()
            };
            let app = &mut app;
            let row = &mut row;
            let mut output = ctx.run_ui(input, |ui| {
                egui::Panel::top("viewer_toolbar").show(ui, |ui| {
                    let mut ops: Vec<SessionOp> = Vec::new();
                    let mut chosen = Theme::ALL[0];
                    row.available = ui.available_width();
                    // The row's OWN rect, through a scope: a panel's
                    // `Ui` is expanded to the panel's width whatever
                    // it holds, so its `min_rect` answers the window
                    // rather than the toolbar.
                    let laid_out = ui.scope(|ui| {
                        app.toolbar_ui(ui, &mut ops, &mut chosen);
                    });
                    row.occupied = laid_out.response.rect.width();
                });
            });
            // Nothing here paints, so the frame's texture delta is
            // dropped rather than uploaded, and epaint refuses a drop
            // it did not see taken.
            output.textures_delta.clear();
        }
        row
    }

    /// **The row does not fit a narrow window.** This is the
    /// measurement the wrapping answers, and the reason the row below
    /// is a hold rather than a tautology: if the toolbar ever loses
    /// enough controls to fit, this reads red, and the honest repair
    /// is to retire both rows rather than to widen the number.
    #[test]
    fn the_toolbar_asks_for_more_width_than_a_narrow_window_gives() {
        let natural = toolbar_row(UNBOUNDED).occupied;
        assert!(
            natural > NARROW,
            "the toolbar's natural width is {natural} points, which already fits a {NARROW}-point window"
        );
    }

    /// **And it wraps rather than running off the edge.** A row laid
    /// out past the window's right edge is clipped, and a clipped
    /// control is not small — it is unreachable, which for the two
    /// cancel doors means no exit from a gesture at all.
    #[test]
    fn the_toolbar_wraps_rather_than_running_past_a_narrow_window() {
        let row = toolbar_row(NARROW);
        assert!(
            row.occupied <= row.available,
            "the toolbar occupied {} points of the {} it was given, so {} points of it lie past the right edge",
            row.occupied,
            row.available,
            row.occupied - row.available
        );
    }
    /// The context startup installed onto, and the app it assembled.
    ///
    /// [`ViewerApp::assemble`] is the half of startup that needs no
    /// graphics device, and both of the context-wide installs are in
    /// it — the `egui::Context` it is handed reaches nothing else —
    /// so a bare context is the whole subject. **No frame is run**:
    /// each install exists to be in force before anything is drawn,
    /// so the read a row owes is the one taken straight afterwards.
    fn started() -> (egui::Context, ViewerApp) {
        let ctx = egui::Context::default();
        let app = ViewerApp::assemble(&ctx, pncad::tolerance::witness())
            .expect("startup that needs no graphics device");
        (ctx, app)
    }

    /// **Startup states the resolved palette's polarity on the
    /// context.**
    ///
    /// Two reads, because neither alone is a statement about the
    /// install. The visuals are what the first frame paints, and they
    /// are the reason the install is where it is — but `egui`'s
    /// `fallback_theme` is `Theme::Dark` and this chrome's default
    /// palette is a dark one, so a context startup never touched
    /// already answers `dark_mode` here and that read is green over
    /// no install at all. The preference is what `apply_polarity`
    /// states — `set_theme` rather than `set_visuals`, for the reason
    /// its own doc gives — and an untouched context holds
    /// `ThemePreference::System` whichever palette resolves, so it is
    /// the read that fails when nobody applies anything.
    #[test]
    fn startup_states_the_resolved_polarity_on_the_context() {
        let (ctx, app) = started();
        let stated = ctx.options(|options| options.theme_preference);
        let wanted = match app.theme.polarity {
            Polarity::Light => egui::ThemePreference::Light,
            Polarity::Dark => egui::ThemePreference::Dark,
        };
        assert_eq!(
            stated, wanted,
            "startup resolved {:?} and left the context stating {stated:?}",
            app.theme.polarity
        );
        let dark_mode = ctx.global_style().visuals.dark_mode;
        assert_eq!(
            dark_mode,
            matches!(app.theme.polarity, Polarity::Dark),
            "startup resolved {:?} and the visuals a first frame would paint report dark_mode = {dark_mode}",
            app.theme.polarity
        );
    }

    /// **Startup installs the chrome's numeric rule onto both of the
    /// context's styles.**
    ///
    /// Behavioural, because a `NumberFormatter` is a function value
    /// and its `PartialEq` is `Arc::ptr_eq`: a comparison against a
    /// freshly built `NumberFormatter::new(number_text)` is false
    /// however right the install is, and one that passed would be a
    /// statement about an allocation rather than about what a field
    /// will say. So the read is the text — a value spelled through
    /// the context's own formatter, against the text the door spells.
    ///
    /// **Both styles rather than whichever the polarity in force
    /// selects**, so this row reads nothing that the other install
    /// decides. That the install writes both is
    /// `widgets::field_tests::a_bare_field_survives_a_theme_switch`'s
    /// claim, and stays its claim; what is read here is that startup
    /// performs the install at all.
    ///
    /// The witness is what makes the reading a statement, and the
    /// first assertion is what says so: 40 nm in millimetres over the
    /// decimal range a length field is really handed is a value
    /// `{:.3}` cannot read back, so the door's text and the toolkit's
    /// untouched default differ there. When that assertion goes red
    /// this row can no longer see the install, whatever the other two
    /// say.
    #[test]
    fn startup_installs_the_number_rule_onto_both_of_the_contexts_styles() {
        /// 40 nm, in the millimetres a length field holds.
        const WITNESS: f64 = 4.0e-5;
        /// The decimal range a length field shown in millimetres is
        /// handed, derived at `widgets::field_tests::MM`.
        const DECIMALS: core::ops::RangeInclusive<usize> = 1..=3;

        let door = crate::widgets::number_text(WITNESS, DECIMALS);
        let toolkit = egui::emath::format_with_decimals_in_range(WITNESS, DECIMALS);
        assert_ne!(
            door, toolkit,
            "the witness spells the same either way, so nothing below distinguishes the install from the toolkit's default"
        );

        let (ctx, _app) = started();
        let (dark, light) = ctx.options(|options| {
            (
                options.dark_style.number_formatter.clone(),
                options.light_style.number_formatter.clone(),
            )
        });
        assert_eq!(
            dark.format(WITNESS, DECIMALS),
            door,
            "the context's dark style spells {WITNESS} its own way"
        );
        assert_eq!(
            light.format(WITNESS, DECIMALS),
            door,
            "the context's light style spells {WITNESS} its own way"
        );
    }
}
